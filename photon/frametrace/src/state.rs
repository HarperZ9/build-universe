use std::collections::{BTreeMap, HashMap};

use crate::{Event, Hazard, ReadSlot, ResourceId, Stage, ViewId, ViewKind, WriteSlot};

/// Shader-resource slots per stage (D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT).
pub const SRV_SLOTS: u32 = 128;
/// Simultaneous render targets (D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT).
pub const RTV_SLOTS: u32 = 8;
/// UAV slots (D3D11_1_UAV_SLOT_COUNT).
pub const UAV_SLOTS: u32 = 64;

const STAGES: [Stage; 6] = [
    Stage::Vs,
    Stage::Ps,
    Stage::Cs,
    Stage::Gs,
    Stage::Hs,
    Stage::Ds,
];

/// The live binding state of a D3D11 immediate context: a symbol table for the
/// GPU frame. Apply events in order, then query the binding tables or detect
/// hazards at any point. Slot maps are sparse (only bound slots are stored).
#[derive(Default)]
pub struct FrameState {
    views: HashMap<ViewId, (ResourceId, ViewKind)>,
    srv: HashMap<Stage, BTreeMap<u32, ViewId>>,
    uav: BTreeMap<u32, ViewId>,
    rtv: BTreeMap<u32, ViewId>,
    dsv: Option<ViewId>,
    timeline: Vec<Event>,
    checkpoint: u64,
}

impl FrameState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply one event, updating the binding tables and recording it.
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
                        Some(view) => {
                            table.insert(slot, *view);
                        }
                        None => {
                            table.remove(&slot);
                        }
                    }
                }
            }
            Event::SetUnorderedAccessViews { start_slot, views } => {
                for (i, v) in views.iter().enumerate() {
                    let slot = start_slot + i as u32;
                    match v {
                        Some(view) => {
                            self.uav.insert(slot, *view);
                        }
                        None => {
                            self.uav.remove(&slot);
                        }
                    }
                }
            }
            Event::SetRenderTargets { rtvs, dsv } => {
                // OMSetRenderTargets replaces the entire RTV array and the DSV.
                self.rtv.clear();
                for (i, v) in rtvs.iter().enumerate() {
                    if let Some(view) = v {
                        self.rtv.insert(i as u32, *view);
                    }
                }
                self.dsv = *dsv;
            }
            Event::ClearRenderTargetView { .. } | Event::ClearDepthStencilView { .. } => {
                // Clears do not change bindings; recorded for the timeline.
            }
            Event::Draw | Event::Dispatch => {
                self.checkpoint += 1;
            }
        }
        self.timeline.push(event);
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

    /// Detect read/write hazards in the CURRENT binding state. A resource is
    /// reported when it is bound as a shader-resource view (read) and also as a
    /// render-target, depth-stencil, or unordered-access view (write). A lone
    /// UAV is not a hazard (it has no separate reader). Write-write conflicts
    /// (e.g. RTV plus UAV on one resource) are a known v0 gap.
    pub fn hazards(&self) -> Vec<Hazard> {
        let mut reads: BTreeMap<u64, Vec<ReadSlot>> = BTreeMap::new();
        let mut writes: BTreeMap<u64, Vec<WriteSlot>> = BTreeMap::new();

        // Reads: shader-resource views across every stage, in stage then slot order.
        for stage in STAGES {
            if let Some(table) = self.srv.get(&stage) {
                for (slot, view) in table {
                    if let Some((res, kind)) = self.resolve(*view) {
                        if kind.reads() {
                            reads
                                .entry(res.0)
                                .or_default()
                                .push(ReadSlot { stage, slot: *slot });
                        }
                    }
                }
            }
        }

        // Writes: render targets, depth-stencil, and unordered-access views.
        for (slot, view) in &self.rtv {
            if let Some((res, _)) = self.resolve(*view) {
                writes.entry(res.0).or_default().push(WriteSlot::Rtv(*slot));
            }
        }
        if let Some(view) = self.dsv {
            if let Some((res, _)) = self.resolve(view) {
                writes.entry(res.0).or_default().push(WriteSlot::Dsv);
            }
        }
        for (slot, view) in &self.uav {
            if let Some((res, _)) = self.resolve(*view) {
                writes.entry(res.0).or_default().push(WriteSlot::Uav(*slot));
            }
        }

        let mut hazards = Vec::new();
        for (res, w) in &writes {
            if let Some(r) = reads.get(res) {
                hazards.push(Hazard {
                    resource: ResourceId(*res),
                    reads: r.clone(),
                    writes: w.clone(),
                    at_checkpoint: self.checkpoint,
                });
            }
        }
        hazards
    }
}
