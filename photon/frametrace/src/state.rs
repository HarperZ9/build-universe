use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::{
    BindPoint, Event, Hazard, HazardKind, ReadSlot, ResourceId, RestoreLeak, Stage, ViewId,
    ViewKind, WriteSlot,
};

/// Shader-resource slots per stage (D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT).
pub const SRV_SLOTS: u32 = 128;
/// Simultaneous render targets (D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT).
pub const RTV_SLOTS: u32 = 8;
/// UAV slots (D3D11_1_UAV_SLOT_COUNT).
pub const UAV_SLOTS: u32 = 64;

const STAGES: [Stage; 6] = [Stage::Vs, Stage::Ps, Stage::Cs, Stage::Gs, Stage::Hs, Stage::Ds];

/// The hazards observed at one draw/dispatch checkpoint.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DrawHazards {
    /// The checkpoint index (1-based, monotonic).
    pub checkpoint: u64,
    /// Hazards present in the binding state at that checkpoint.
    pub hazards: Vec<Hazard>,
}

/// The live binding state of a D3D11 immediate context: a symbol table for the
/// GPU frame. Apply events in order, then query bindings or detect hazards.
#[derive(Default)]
pub struct FrameState {
    views: HashMap<ViewId, (ResourceId, ViewKind)>,
    srv: HashMap<Stage, BTreeMap<u32, ViewId>>,
    uav: BTreeMap<u32, ViewId>,
    rtv: BTreeMap<u32, ViewId>,
    dsv: Option<ViewId>,
    timeline: Vec<Event>,
    checkpoint: u64,
    log: Vec<DrawHazards>,
}

impl FrameState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply one event, updating the binding tables and recording it. At each
    /// draw/dispatch the current hazards are snapshotted into the hazard log.
    pub fn apply(&mut self, event: Event) {
        match &event {
            Event::RegisterView { view, resource, kind } => {
                self.views.insert(*view, (*resource, *kind));
            }
            Event::SetShaderResources { stage, start_slot, views } => {
                let table = self.srv.entry(*stage).or_default();
                for (i, v) in views.iter().enumerate() {
                    let slot = start_slot + i as u32;
                    match v {
                        Some(view) => { table.insert(slot, *view); }
                        None => { table.remove(&slot); }
                    }
                }
            }
            Event::SetUnorderedAccessViews { start_slot, views } => {
                for (i, v) in views.iter().enumerate() {
                    let slot = start_slot + i as u32;
                    match v {
                        Some(view) => { self.uav.insert(slot, *view); }
                        None => { self.uav.remove(&slot); }
                    }
                }
            }
            Event::SetRenderTargets { rtvs, dsv } => {
                self.rtv.clear();
                for (i, v) in rtvs.iter().enumerate() {
                    if let Some(view) = v {
                        self.rtv.insert(i as u32, *view);
                    }
                }
                self.dsv = *dsv;
            }
            Event::ClearRenderTargetView { .. } | Event::ClearDepthStencilView { .. } => {}
            Event::Draw | Event::Dispatch => {
                self.checkpoint += 1;
            }
        }
        let is_checkpoint = matches!(event, Event::Draw | Event::Dispatch);
        self.timeline.push(event);
        if is_checkpoint {
            let hazards = self.hazards();
            let checkpoint = self.checkpoint;
            self.log.push(DrawHazards { checkpoint, hazards });
        }
    }

    /// Apply a sequence of events in order.
    pub fn apply_all<I: IntoIterator<Item = Event>>(&mut self, events: I) {
        for e in events {
            self.apply(e);
        }
    }

    /// The view bound to a shader-resource slot of a stage, if any.
    pub fn srv_at(&self, stage: Stage, slot: u32) -> Option<ViewId> {
        self.srv.get(&stage).and_then(|t| t.get(&slot)).copied()
    }

    /// The currently bound render targets as (slot, view) pairs.
    pub fn render_targets(&self) -> Vec<(u32, ViewId)> {
        self.rtv.iter().map(|(s, v)| (*s, *v)).collect()
    }

    /// The currently bound depth-stencil view, if any.
    pub fn depth_stencil(&self) -> Option<ViewId> {
        self.dsv
    }

    /// The currently bound unordered-access views as (slot, view) pairs.
    pub fn unordered_access(&self) -> Vec<(u32, ViewId)> {
        self.uav.iter().map(|(s, v)| (*s, *v)).collect()
    }

    /// Resolve a view to its (resource, kind), if registered.
    pub fn resolve(&self, view: ViewId) -> Option<(ResourceId, ViewKind)> {
        self.views.get(&view).copied()
    }

    /// The recorded event timeline.
    pub fn timeline(&self) -> &[Event] {
        &self.timeline
    }

    /// The number of draw/dispatch checkpoints applied so far.
    pub fn checkpoint(&self) -> u64 {
        self.checkpoint
    }

    /// The per-draw hazard log across the whole frame so far.
    pub fn hazard_log(&self) -> &[DrawHazards] {
        &self.log
    }

    /// Hazards recorded at a specific checkpoint, if that checkpoint exists.
    pub fn hazards_at(&self, checkpoint: u64) -> Option<&[Hazard]> {
        self.log.iter().find(|d| d.checkpoint == checkpoint).map(|d| d.hazards.as_slice())
    }

    /// Detect hazards in the CURRENT binding state. A resource read via an SRV
    /// and also written (RTV/DSV/UAV) is ReadWrite; a resource written through
    /// two or more distinct write views with no reader is WriteWrite. A
    /// read-only DSV is neither read nor write; a lone UAV is not a hazard.
    pub fn hazards(&self) -> Vec<Hazard> {
        let mut reads: BTreeMap<u64, Vec<ReadSlot>> = BTreeMap::new();
        let mut writes: BTreeMap<u64, Vec<WriteSlot>> = BTreeMap::new();

        for stage in STAGES {
            if let Some(table) = self.srv.get(&stage) {
                for (slot, view) in table {
                    if let Some((res, kind)) = self.resolve(*view) {
                        if kind.reads() {
                            reads.entry(res.0).or_default().push(ReadSlot { stage, slot: *slot });
                        }
                    }
                }
            }
        }
        for (slot, view) in &self.rtv {
            if let Some((res, _)) = self.resolve(*view) {
                writes.entry(res.0).or_default().push(WriteSlot::Rtv(*slot));
            }
        }
        if let Some(view) = self.dsv {
            if let Some((res, kind)) = self.resolve(view) {
                if kind == ViewKind::Dsv {
                    writes.entry(res.0).or_default().push(WriteSlot::Dsv);
                }
            }
        }
        for (slot, view) in &self.uav {
            if let Some((res, _)) = self.resolve(*view) {
                writes.entry(res.0).or_default().push(WriteSlot::Uav(*slot));
            }
        }

        let mut hazards = Vec::new();
        let resources: BTreeSet<u64> = writes.keys().copied().collect();
        for res in resources {
            let w = writes.get(&res).cloned().unwrap_or_default();
            if let Some(r) = reads.get(&res) {
                hazards.push(Hazard {
                    kind: HazardKind::ReadWrite,
                    resource: ResourceId(res),
                    reads: r.clone(),
                    writes: w,
                    at_checkpoint: self.checkpoint,
                });
            } else if w.len() >= 2 {
                hazards.push(Hazard {
                    kind: HazardKind::WriteWrite,
                    resource: ResourceId(res),
                    reads: Vec::new(),
                    writes: w,
                    at_checkpoint: self.checkpoint,
                });
            }
        }
        hazards
    }
}

impl FrameState {
    fn bound(&self, view: ViewId) -> Bound {
        Bound { view, resource: self.resolve(view).map(|(r, _)| r) }
    }

    /// Capture the current resource-binding state for restore-verification.
    pub fn snapshot(&self) -> Snapshot {
        let mut srv = BTreeMap::new();
        for (stage, table) in &self.srv {
            for (slot, view) in table {
                srv.insert((stage.index(), *slot), self.bound(*view));
            }
        }
        Snapshot {
            srv,
            uav: self.uav.iter().map(|(s, v)| (*s, self.bound(*v))).collect(),
            rtv: self.rtv.iter().map(|(s, v)| (*s, self.bound(*v))).collect(),
            dsv: self.dsv.map(|v| self.bound(v)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Bound {
    view: ViewId,
    resource: Option<ResourceId>,
}

/// An immutable capture of the resource-binding state at one instant. Diff a
/// pre-effect SAVE against the post-effect RESTORE: any differing slot is a
/// restore leak (the effect was not transparent to the host).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Snapshot {
    srv: BTreeMap<(u8, u32), Bound>,
    uav: BTreeMap<u32, Bound>,
    rtv: BTreeMap<u32, Bound>,
    dsv: Option<Bound>,
}

fn view_of(b: Option<&Bound>) -> Option<ViewId> {
    b.map(|x| x.view)
}

fn make_leak(at: BindPoint, s: Option<&Bound>, r: Option<&Bound>) -> RestoreLeak {
    RestoreLeak {
        at,
        saved: s.map(|b| b.view),
        restored: r.map(|b| b.view),
        saved_resource: s.and_then(|b| b.resource),
        restored_resource: r.and_then(|b| b.resource),
    }
}

impl Snapshot {
    /// Slots whose binding differs between this (SAVED) snapshot and the
    /// RESTORED one. Empty = the restore was transparent. Deterministic order.
    pub fn diff_restore(&self, restored: &Snapshot) -> Vec<RestoreLeak> {
        let mut leaks = Vec::new();

        let mut srv_keys: BTreeSet<(u8, u32)> = self.srv.keys().copied().collect();
        srv_keys.extend(restored.srv.keys().copied());
        for (si, slot) in srv_keys {
            let s = self.srv.get(&(si, slot));
            let r = restored.srv.get(&(si, slot));
            if view_of(s) != view_of(r) {
                leaks.push(make_leak(BindPoint::Srv { stage: Stage::from_index(si), slot }, s, r));
            }
        }
        let mut uav_keys: BTreeSet<u32> = self.uav.keys().copied().collect();
        uav_keys.extend(restored.uav.keys().copied());
        for slot in uav_keys {
            let (s, r) = (self.uav.get(&slot), restored.uav.get(&slot));
            if view_of(s) != view_of(r) {
                leaks.push(make_leak(BindPoint::Uav(slot), s, r));
            }
        }
        let mut rtv_keys: BTreeSet<u32> = self.rtv.keys().copied().collect();
        rtv_keys.extend(restored.rtv.keys().copied());
        for slot in rtv_keys {
            let (s, r) = (self.rtv.get(&slot), restored.rtv.get(&slot));
            if view_of(s) != view_of(r) {
                leaks.push(make_leak(BindPoint::Rtv(slot), s, r));
            }
        }
        if view_of(self.dsv.as_ref()) != view_of(restored.dsv.as_ref()) {
            leaks.push(make_leak(BindPoint::Dsv, self.dsv.as_ref(), restored.dsv.as_ref()));
        }
        leaks
    }
}
