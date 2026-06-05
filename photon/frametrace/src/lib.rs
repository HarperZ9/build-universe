//! photon-frametrace: a symbol table for the D3D11 immediate-context frame.
//!
//! Records every Set/Clear/Unbind/Draw/Dispatch event, maintains the live
//! binding table per shader stage and the output-merger, and detects
//! read/write hazards: a resource bound as a shader-resource view (read) while
//! it is also bound as a render-target, depth-stencil, or unordered-access
//! view (write).
//!
//! The model is fed by a capture layer (a D3D11 vtable hook, or a RenderDoc/
//! PIX/ETW trace) and answers ground-truth questions like
//!   "Is the SSR output still bound to t27 when the compositor samples it?"
//!   "Was the DSV unbound before this draw to avoid a read/write hazard?"
//! instead of forcing a human (or a model) to reason about frame state in
//! their head. There is no simulator underneath an LLM; this is one.

mod state;

pub use state::{FrameState, RTV_SLOTS, SRV_SLOTS, UAV_SLOTS};

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
    /// Depth-stencil view (write, unless read-only; read-only DSV is a TODO).
    Dsv,
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

/// A detected read/write hazard: one resource reachable as both a read view
/// and a write view at the same draw/dispatch.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hazard {
    /// The resource bound for both read and write.
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
            "resource {:?} bound read({:?}) + write({:?}) at checkpoint {}",
            self.resource, self.reads, self.writes, self.at_checkpoint
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
