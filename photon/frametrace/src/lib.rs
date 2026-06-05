//! photon-frametrace: a symbol table for the D3D11 immediate-context frame.
//!
//! Records every Set/Clear/Unbind/Draw/Dispatch event, maintains the live
//! binding table per shader stage and the output-merger, and detects
//! read/write and write/write hazards. Fed by a capture layer (a D3D11 vtable
//! hook, or a RenderDoc/PIX/ETW trace), it answers ground-truth questions like
//!   "Is the SSR output still bound to t27 when the compositor samples it?"
//!   "Was the DSV unbound before this draw to avoid a read/write hazard?"
//! instead of forcing a human (or a model) to reason about frame state in
//! their head. There is no simulator underneath an LLM; this is one.

mod state;
pub mod ffi;
#[cfg(feature = "trace")]
pub mod trace;

pub use state::{DrawHazards, FrameState, Snapshot, RTV_SLOTS, SRV_SLOTS, UAV_SLOTS};

use std::fmt;

/// An underlying GPU resource (texture/buffer). In a live hook this is the
/// ID3D11Resource pointer; in the model it is an opaque handle.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ResourceId(pub u64);

/// A view onto a resource (SRV/RTV/DSV/UAV). In a live hook this is the
/// ID3D11View pointer; in the model it is an opaque handle.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ViewId(pub u64);

/// The kind of view, which determines whether a binding reads or writes the
/// underlying resource.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ViewKind {
    /// Shader resource view (read).
    Srv,
    /// Render target view (write).
    Rtv,
    /// Depth-stencil view (write).
    Dsv,
    /// Read-only depth-stencil view: bound for the depth test but neither read
    /// as a shader resource nor written, so it does not hazard against an SRV.
    DsvReadOnly,
    /// Unordered access view (read/write).
    Uav,
}

impl ViewKind {
    /// Whether a binding of this kind reads the underlying resource.
    pub fn reads(self) -> bool {
        matches!(self, ViewKind::Srv | ViewKind::Uav)
    }
    /// Whether a binding of this kind writes the underlying resource.
    pub fn writes(self) -> bool {
        matches!(self, ViewKind::Rtv | ViewKind::Dsv | ViewKind::Uav)
    }
}

/// A programmable shader stage with its own shader-resource slots.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Stage {
    Vs,
    Ps,
    Cs,
    Gs,
    Hs,
    Ds,
}

impl Stage {
    /// Deterministic index Vs=0..Ds=5, for ordered/serializable snapshots.
    pub fn index(self) -> u8 {
        match self {
            Stage::Vs => 0,
            Stage::Ps => 1,
            Stage::Cs => 2,
            Stage::Gs => 3,
            Stage::Hs => 4,
            Stage::Ds => 5,
        }
    }
    /// Inverse of index (out-of-range maps to Vs).
    pub fn from_index(i: u8) -> Stage {
        match i {
            1 => Stage::Ps,
            2 => Stage::Cs,
            3 => Stage::Gs,
            4 => Stage::Hs,
            5 => Stage::Ds,
            _ => Stage::Vs,
        }
    }
}

/// A specific bindable location, for restore-verify reporting.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BindPoint {
    Srv { stage: Stage, slot: u32 },
    Uav(u32),
    Rtv(u32),
    Dsv,
}

impl fmt::Display for BindPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BindPoint::Srv { stage, slot } => write!(f, "{:?} t{}", stage, slot),
            BindPoint::Uav(s) => write!(f, "u{}", s),
            BindPoint::Rtv(s) => write!(f, "rtv{}", s),
            BindPoint::Dsv => write!(f, "dsv"),
        }
    }
}

/// One slot whose binding changed across a save/restore boundary: evidence the
/// restore was NOT transparent to the host, so the game runs with corrupted
/// state. The most expensive failure class, because the game did not cause it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RestoreLeak {
    pub at: BindPoint,
    pub saved: Option<ViewId>,
    pub restored: Option<ViewId>,
    pub saved_resource: Option<ResourceId>,
    pub restored_resource: Option<ResourceId>,
}

impl fmt::Display for RestoreLeak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn side(v: Option<ViewId>, r: Option<ResourceId>) -> String {
            match (v, r) {
                (Some(v), Some(r)) => format!("res#{} (view#{})", r.0, v.0),
                (Some(v), None) => format!("view#{}", v.0),
                _ => "NULL".to_string(),
            }
        }
        write!(
            f,
            "{} saved={} restored={}",
            self.at,
            side(self.saved, self.saved_resource),
            side(self.restored, self.restored_resource)
        )
    }
}

/// Where a write binding lives, for hazard reporting.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WriteSlot {
    /// Output-merger render target at the given index.
    Rtv(u32),
    /// Output-merger depth-stencil.
    Dsv,
    /// Unordered access view at the given index.
    Uav(u32),
}

/// A read binding location, for hazard reporting.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReadSlot {
    pub stage: Stage,
    pub slot: u32,
}

/// The category of a detected hazard.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HazardKind {
    /// One resource bound as a read (SRV) and a write (RTV/DSV/UAV) at once.
    ReadWrite,
    /// One resource bound through two or more distinct write views at once
    /// (e.g. RTV and UAV), with no reader.
    WriteWrite,
}

/// A detected hazard: one resource reachable through conflicting bindings at
/// the same draw/dispatch.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hazard {
    /// The category of conflict.
    pub kind: HazardKind,
    /// The resource bound through conflicting views.
    pub resource: ResourceId,
    /// Every read binding that reaches the resource.
    pub reads: Vec<ReadSlot>,
    /// Every write binding that reaches the resource.
    pub writes: Vec<WriteSlot>,
    /// The draw/dispatch checkpoint at which the hazard was observed.
    pub at_checkpoint: u64,
}

impl fmt::Display for Hazard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} hazard on resource {:?}: read({:?}) write({:?}) at checkpoint {}",
            self.kind, self.resource, self.reads, self.writes, self.at_checkpoint
        )
    }
}

/// A single recorded D3D11 immediate-context event. Unbinding is modelled as
/// setting a slot to None: D3D11 unbinds by binding a null view.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
    /// Register a view to (resource, kind) mapping (CreateShaderResourceView, etc.).
    RegisterView {
        view: ViewId,
        resource: ResourceId,
        kind: ViewKind,
    },
    /// VS/PS/CS/GS/HS/DS SetShaderResources(start_slot, views).
    SetShaderResources {
        stage: Stage,
        start_slot: u32,
        views: Vec<Option<ViewId>>,
    },
    /// CSSetUnorderedAccessViews(start_slot, views).
    SetUnorderedAccessViews {
        start_slot: u32,
        views: Vec<Option<ViewId>>,
    },
    /// OMSetRenderTargets(rtvs, dsv). Replaces the entire RTV array and the DSV.
    SetRenderTargets {
        rtvs: Vec<Option<ViewId>>,
        dsv: Option<ViewId>,
    },
    /// ClearRenderTargetView(rtv). Recorded; does not change bindings.
    ClearRenderTargetView { rtv: ViewId },
    /// ClearDepthStencilView(dsv). Recorded; does not change bindings.
    ClearDepthStencilView { dsv: ViewId },
    /// Any draw call (Draw/DrawIndexed/DrawInstanced/...). A hazard checkpoint.
    Draw,
    /// Dispatch/DispatchIndirect. A hazard checkpoint.
    Dispatch,
}
