//! Circuit representation with Struct-of-Arrays design and pre-indexed matrix stamping
//!
//! This module provides high-performance circuit storage using data-oriented design:
//! - Struct-of-Arrays (SoA) for cache-friendly iteration
//! - Pre-computed matrix indices for zero-cost stamping in the hot loop
//! - Separation of topology (static) from values (mutable)

#![allow(clippy::too_many_arguments)]
use crate::device::behavioral::BehavioralSources;
use crate::device::{Cccs, Ccvs, MatrixStamper, NonlinearConvergenceCriteria, Vccs, Vcvs};
use crate::numerics::integration::CompanionCoefficients;
use crate::solver::{CscIndex, StaticMatrix, TripletMatrix};
use crate::xspice::{CodeModelRegistry, DigitalValue, XspiceEventScheduler, XspiceInstance};
use crate::{NodeId, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;

mod storage;
pub use storage::{
    Capacitors, CurrentSources, Diodes, Inductors, ResistorBranches, Resistors,
    ThermalResistorState, VoltageSources,
};
// The nonlinear device arrays are stamping machinery that no frontend names,
// so they stay reachable inside the crate only. `Diodes` is re-exported above
// because the public `CircuitData::diode_storage` returns one.
pub(crate) use storage::{
    B3SoiDds, B3SoiFds, B3SoiPds, Bjts, Bsim3v3s, Bsim4v8s, Ekv3Mosfets, EkvMosfets, Mosfets,
    Vdmoses,
};
mod construction;
pub(crate) mod dae;
mod external_models;
pub(crate) use external_models::VerilogADcAcceptedStateCarrier;
mod force_accept;
mod introspection;
pub use introspection::{DeviceOpEntry, DeviceOpReport};
mod linear_stamping;
mod magnetic;
mod net_kind;
pub(crate) use net_kind::{NetKind, NetKinds};
mod nonlinear;
pub use crate::op_label::{OP_LABELS, OpLabel};
pub(crate) use nonlinear::{AcceptedNativeNonlinearCheckpointStates, NonlinearDeviceStateSnapshot};

/// Resolve persisted text back to the interned label this build emits.
///
/// This is the reader's half of the vocabulary, and the reason the vocabulary
/// outlives its emitters: a label stays resolvable after the family that used
/// to report it changes, so a project written by an older build still opens.
///
/// It lives here rather than beside the vocabulary because the fixed half is a
/// leaf the device families read down into, while this half has to ask the
/// compiled Verilog-A catalog — which sits above them.
pub fn resolve_op_label(text: &str) -> Option<&'static str> {
    crate::op_label::declared_op_label(text).or_else(|| generated_catalog_op_label(text))
}

/// Labels contributed by the Verilog-A models compiled into this build.
///
/// A model's canonical name is its device family, and each external terminal
/// names the current entering it. Both are generated, so the catalog answers
/// for them; a build without the catalog has no such labels to resolve.
fn generated_catalog_op_label(text: &str) -> Option<&'static str> {
    for descriptor in crate::device::veriloga_builtins::generated_veriloga_model_descriptors() {
        if descriptor.model_name == text {
            return Some(descriptor.model_name);
        }
        for terminal in descriptor.terminals {
            if terminal.current_parameter == text {
                return Some(terminal.current_parameter);
            }
        }
    }
    None
}
pub(crate) mod xyce_dae;
pub(crate) mod xyce_load;

type XspiceDriverId = (String, String, usize);
type XspiceDigitalDrivers = HashMap<NodeId, HashMap<XspiceDriverId, DigitalValue>>;
type XspiceRealDrivers = HashMap<NodeId, HashMap<XspiceDriverId, Value>>;

#[inline]
fn solution_node_voltage(solution: &[Value], node: NodeId) -> Option<Value> {
    if node == 0 {
        Some(0.0)
    } else {
        solution
            .get(node - 1)
            .copied()
            .filter(|value| value.is_finite())
    }
}

#[inline]
fn projection_changed(old: Value, new: Value) -> bool {
    let scale = old.abs().max(new.abs()).max(1.0);
    (new - old).abs() > 64.0 * Value::EPSILON * scale
}

#[inline]
fn project_two_terminal_voltage(
    solution: &mut [Value],
    node_pos: NodeId,
    node_neg: NodeId,
    target_voltage: Value,
) -> bool {
    if !target_voltage.is_finite() {
        return false;
    }

    if node_neg == 0 && node_pos > 0 {
        if let Some(v) = solution.get_mut(node_pos - 1) {
            let changed = projection_changed(*v, target_voltage);
            *v = target_voltage;
            return changed;
        }
        return false;
    }

    if node_pos == 0 && node_neg > 0 {
        if let Some(v) = solution.get_mut(node_neg - 1) {
            let projected = -target_voltage;
            let changed = projection_changed(*v, projected);
            *v = projected;
            return changed;
        }
        return false;
    }

    if node_pos == 0 || node_neg == 0 {
        return false;
    }

    let vp_idx = node_pos - 1;
    let vn_idx = node_neg - 1;
    if vp_idx >= solution.len() || vn_idx >= solution.len() {
        return false;
    }

    let vp = solution[vp_idx];
    let vn = solution[vn_idx];
    if !(vp.is_finite() && vn.is_finite()) {
        return false;
    }

    let midpoint = 0.5 * (vp + vn);
    let half_diff = 0.5 * target_voltage;
    let projected_vp = midpoint + half_diff;
    let projected_vn = midpoint - half_diff;
    let changed = projection_changed(vp, projected_vp) || projection_changed(vn, projected_vn);
    solution[vp_idx] = projected_vp;
    solution[vn_idx] = projected_vn;
    changed
}

/// Errors in circuit construction
#[derive(Debug, Error)]
pub enum CircuitError {
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Duplicate node name: {0}")]
    DuplicateNode(String),

    #[error("Invalid component: {0}")]
    InvalidComponent(String),

    #[error(transparent)]
    BehavioralReference(Box<crate::device::BehavioralReferenceError>),

    #[error("No ground node in circuit")]
    NoGround,

    #[error("Floating node detected: {0}")]
    FloatingNode(String),

    #[error("Singular matrix - circuit may have issues")]
    SingularMatrix,
}

impl From<crate::device::BehavioralReferenceError> for CircuitError {
    fn from(error: crate::device::BehavioralReferenceError) -> Self {
        Self::BehavioralReference(Box::new(error))
    }
}

/// Pre-computed index into the matrix values array for O(1) stamping
#[derive(Debug, Clone, Copy)]
pub struct MatrixIndex(pub usize);

/// Pre-computed stamp location (row, col) - used during setup
#[derive(Debug, Clone, Copy)]
pub struct StampLocation {
    pub row: NodeId,
    pub col: NodeId,
}

/// Pre-indexed stamp for a two-terminal device
/// During simulation, we write directly to these matrix positions
#[derive(Debug, Clone, Copy)]
pub struct TwoTerminalStamp {
    /// (node+, node+)
    pub pp: StampLocation,
    /// (node+, node-)
    pub pn: StampLocation,
    /// (node-, node+)
    pub np: StampLocation,
    /// (node-, node-)
    pub nn: StampLocation,
    /// Pre-baked CSC indices for O(1) stamping (populated after matrix build)
    pub csc_pp: Option<CscIndex>,
    pub csc_pn: Option<CscIndex>,
    pub csc_np: Option<CscIndex>,
    pub csc_nn: Option<CscIndex>,
}

impl TwoTerminalStamp {
    pub fn new(node_pos: NodeId, node_neg: NodeId) -> Self {
        Self {
            pp: StampLocation {
                row: node_pos,
                col: node_pos,
            },
            pn: StampLocation {
                row: node_pos,
                col: node_neg,
            },
            np: StampLocation {
                row: node_neg,
                col: node_pos,
            },
            nn: StampLocation {
                row: node_neg,
                col: node_neg,
            },
            csc_pp: None,
            csc_pn: None,
            csc_np: None,
            csc_nn: None,
        }
    }

    /// Link this stamp to a StaticMatrix, caching CSC indices for O(1) access
    pub fn link(&mut self, matrix: &StaticMatrix) {
        if self.pp.row > 0 && self.pp.col > 0 {
            self.csc_pp = matrix.get_index(self.pp.row - 1, self.pp.col - 1);
        }
        if self.pn.row > 0 && self.pn.col > 0 {
            self.csc_pn = matrix.get_index(self.pn.row - 1, self.pn.col - 1);
        }
        if self.np.row > 0 && self.np.col > 0 {
            self.csc_np = matrix.get_index(self.np.row - 1, self.np.col - 1);
        }
        if self.nn.row > 0 && self.nn.col > 0 {
            self.csc_nn = matrix.get_index(self.nn.row - 1, self.nn.col - 1);
        }
    }

    /// Stamp a conductance value using pre-baked CSC indices (O(1) per stamp)
    #[inline]
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, g: Value) {
        if let Some(idx) = self.csc_pp {
            matrix.stamp_direct(idx, g);
        }
        if let Some(idx) = self.csc_pn {
            matrix.stamp_direct(idx, -g);
        }
        if let Some(idx) = self.csc_np {
            matrix.stamp_direct(idx, -g);
        }
        if let Some(idx) = self.csc_nn {
            matrix.stamp_direct(idx, g);
        }
    }

    /// Stamp a conductance value into the matrix
    #[inline]
    pub fn stamp_conductance(&self, matrix: &mut TripletMatrix, g: Value) {
        if self.pp.row != 0 && self.pp.col != 0 {
            matrix.push(self.pp.row - 1, self.pp.col - 1, g);
        }
        if self.pn.row != 0 && self.pn.col != 0 {
            matrix.push(self.pn.row - 1, self.pn.col - 1, -g);
        }
        if self.np.row != 0 && self.np.col != 0 {
            matrix.push(self.np.row - 1, self.np.col - 1, -g);
        }
        if self.nn.row != 0 && self.nn.col != 0 {
            matrix.push(self.nn.row - 1, self.nn.col - 1, g);
        }
    }
}

//=============================================================================
// Main Circuit Data Structure
//=============================================================================

/// High-performance circuit representation using Struct-of-Arrays
#[derive(Debug, Clone)]
pub struct JilesAthertonBinding {
    /// Index into `inductors` SoA arrays that owns this runtime branch/state.
    pub inductor_index: usize,
    /// Branch ordinal (1-indexed) allocated for this inductor.
    pub branch_ordinal: NodeId,
    /// Optional hidden state slot carrying Xyce LEVEL=1 magnetization.  The
    /// slot is appended after the public MNA block so branch ordinals and all
    /// external current/probe mappings remain stable.
    pub hidden_m_slot: Option<usize>,
    /// Optional hidden LEVEL=1 rate slot carrying Xyce's R state.
    pub hidden_r_slot: Option<usize>,
    /// Stateful Jiles-Atherton model used to update effective inductance.
    pub device: crate::device::passive::JilesAthertonInductor,
    /// Xyce nonlinear-core output namespace (`YMIN!<K-name>`), when this
    /// binding originated from a `K ... Core(...)` card.
    pub core_output_name: Option<String>,
    /// Whether the Core model requested SI rather than Xyce's default CGS
    /// B/H output units.
    pub core_bh_si_units: bool,
}

/// One winding in a shared Xyce nonlinear magnetic-core DAE.  The numeric
/// value on an Xyce Core winding is its turn count; the shared device uses
/// these values both for the aggregate ampere-turn field and for the
/// constant mutual charge matrix.
#[derive(Debug, Clone)]
pub struct XyceCoreWindingBinding {
    /// Index into the circuit's inductor SoA storage.
    pub inductor_index: usize,
    /// Physical winding turns as authored on the L-card.
    pub turns: Value,
}

/// Runtime state for one canonical Xyce `K ... 1 CORE` device.  All windings
/// share one magnetization state and one nonlinear `mid(P)` factor; their
/// branch rows are coupled through the vacuum mutual-inductance matrix.
#[derive(Debug, Clone)]
pub struct XyceCoreGroupBinding {
    /// Canonical internal output namespace (`YMIN!KNAME`).
    pub core_output_name: String,
    /// Shared nonlinear constitutive state.
    pub device: crate::device::passive::JilesAthertonInductor,
    /// Hidden state slot carrying the shared LEVEL=1 magnetization state.
    /// Hidden slots are appended after the public MNA block.
    pub hidden_m_slot: Option<usize>,
    /// Hidden LEVEL=1 rate slot carrying Xyce's shared R state.
    pub hidden_r_slot: Option<usize>,
    /// Winding branches participating in the shared DAE.
    pub windings: Vec<XyceCoreWindingBinding>,
    /// Accepted dense `LOI` entries used as OneStep's charge history.  The
    /// vector is parallel to `windings` and is updated only at accepted
    /// transient endpoints.
    pub xyce_q_history: Vec<Value>,
    /// Scalar coupling value authored on the K-card.  Xyce's level-2
    /// MutIndNonLin2 path treats this as metadata and uses the canonical
    /// unity mutual matrix; retaining it here keeps the parsed topology
    /// available for future model levels without losing source information.
    pub coupling: Value,
    /// Whether Xyce should report B/H in SI rather than its default CGS
    /// output units.
    pub core_bh_si_units: bool,
}

/// Runtime binding for a coupled inductor pair.
#[derive(Debug, Clone)]
pub struct CoupledInductorPairBinding {
    /// Branch ordinal (1-indexed) for winding 1.
    pub branch1_ordinal: NodeId,
    /// Branch ordinal (1-indexed) for winding 2.
    pub branch2_ordinal: NodeId,
    /// Coupled inductor runtime device.
    pub device: crate::device::CoupledInductorPair,
}

/// Runtime binding for a multi-winding transformer.
#[derive(Debug, Clone)]
pub struct MultiWindingTransformerBinding {
    /// Branch ordinals (1-indexed) for each winding current.
    pub branch_ordinals: Vec<NodeId>,
    /// Transformer runtime device.
    pub device: crate::device::MultiWindingTransformer,
}

/// Circuit binding for one native Xyce memristor family.
///
/// `node_x` is integrated by a private unity capacitor in the ordinary
/// capacitor pipeline (`Qx = x`). Xyce's `N(...:R)` value is emitted as a
/// derived store waveform and deliberately does not occupy an MNA unknown.
#[derive(Debug, Clone)]
pub(crate) struct XyceMemristorBinding {
    pub name: String,
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    pub node_x: NodeId,
    pub device: crate::device::XyceMemristor,
    /// Last defined Xyce `N(...:R)` store value. The upstream device updates
    /// this only when incremental conductance is nonzero, so it is persistent
    /// analysis state rather than a stateless derived quantity.
    pub resistance_store: Value,
}

/// Sparse authored resistor-noise temperatures that must remain absolute.
///
/// Most resistors inherit the analysis temperature, optionally shifted by
/// `DTEMP`, so allocating a parallel array for every circuit would make the
/// exceptional `TEMP` provenance look like device storage. This object owns
/// that exceptional state and keeps its representation canonical: omitted
/// slots are `None`, and trailing omissions are never retained.
#[derive(Debug, Clone, Default)]
struct ResistorNoiseTemperatureProvenance {
    absolute_by_resistor: Vec<Option<Value>>,
}

impl ResistorNoiseTemperatureProvenance {
    fn set_absolute(&mut self, resistor_index: usize, temperature: Value) {
        let materialized_len = resistor_index
            .checked_add(1)
            .expect("a resistor index derived from Vec::len() cannot overflow");
        self.absolute_by_resistor.resize(materialized_len, None);
        self.absolute_by_resistor[resistor_index] = Some(temperature);
    }

    fn clear(&mut self, resistor_index: usize) {
        if let Some(slot) = self.absolute_by_resistor.get_mut(resistor_index) {
            *slot = None;
        }
        while self
            .absolute_by_resistor
            .last()
            .is_some_and(Option::is_none)
        {
            self.absolute_by_resistor.pop();
        }
    }

    #[inline]
    fn get(&self, resistor_index: usize) -> Option<Value> {
        self.absolute_by_resistor
            .get(resistor_index)
            .copied()
            .flatten()
    }

    #[inline]
    fn materialized_len(&self) -> usize {
        self.absolute_by_resistor.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.absolute_by_resistor.is_empty()
    }
}

/// High-performance circuit representation using Struct-of-Arrays
///
/// `Clone` exists so embarrassingly-parallel sweeps (AC frequency points)
/// can give each worker thread an independent copy: device evaluation
/// caches use `Cell`/`RefCell` and are not shareable across threads.
#[derive(Debug, Clone)]
pub struct CircuitData {
    /// Node name to ID mapping
    node_map: HashMap<String, NodeId>,
    /// Tracks whether any element explicitly referenced the SPICE ground node.
    has_explicit_ground_reference: bool,
    /// Branch element name to branch ordinal mapping (for CCCS/CCVS control lookup)
    /// Keys are element names (e.g., "V1", "L1"), values are branch ordinals (1-indexed)
    branch_names: HashMap<String, NodeId>,
    /// Canonical branch names indexed by branch ordinal - 1.
    branch_name_by_ordinal: Vec<Option<String>>,
    /// Number of nodes (excluding ground)
    num_nodes: usize,
    /// What kind of quantity each net carries. This is the one record of
    /// which nets are event-driven rather than analog; the XSPICE event-node
    /// accessors read it rather than keeping a node list of their own.
    pub(crate) net_kinds: NetKinds,
    /// Number of branch current variables (voltage sources, inductors)
    num_branches: usize,
    /// Number of private scalar DAE state variables appended after the public
    /// node/branch MNA block.  These are intentionally not branch ordinals.
    hidden_state_count: usize,

    // Linear device storage (SoA for cache efficiency)
    pub(crate) resistors: Resistors,
    /// Exceptional authored absolute `TEMP` provenance, allocated lazily and
    /// kept outside the exhaustively constructible public [`Resistors`] API.
    resistor_noise_temperature_provenance: Option<ResistorNoiseTemperatureProvenance>,
    pub(crate) resistor_branches: ResistorBranches,
    pub(crate) capacitors: Capacitors,
    pub(crate) inductors: Inductors,
    pub(crate) voltage_sources: VoltageSources,
    pub(crate) current_sources: CurrentSources,
    /// Parser/synthesis insertion sequence used to build Xyce's logical
    /// reversed-BFT device schedule after final node remapping.
    pub(crate) xyce_topology_order: Vec<xyce_load::XyceDeviceRef>,
    /// Frozen master/instance and independent-source load order.
    pub(crate) xyce_load_plan: xyce_load::XyceLoadPlan,
    /// Cached resistor `F*x` operator using Xyce's row arithmetic order.
    pub(crate) xyce_linear_f_operator: xyce_load::XyceLinearDaeOperator,

    // Nonlinear device storage (require Newton-Raphson iteration)
    pub(crate) diodes: Diodes,
    pub(crate) bjts: Bjts,
    pub(crate) mosfets: Mosfets,
    pub(crate) b3soi: B3SoiDds,
    pub(crate) b3soi_fd: B3SoiFds,
    pub(crate) b3soi_pd: B3SoiPds,
    /// Multiplier applied to the circuit-level device GMIN before it reaches
    /// BSIMSOI3 terminal-GMIN branches. Xyce defaults this to `1e-6`.
    pub(crate) b3soi_gmin_scale: Value,
    /// Nodes that no DC-conducting element ties to ground, as found during
    /// construction from the flattened elements.
    ///
    pub(crate) no_dc_path_nodes: Vec<String>,
    /// Subset of [`Self::no_dc_path_nodes`] whose ungrounded conductive
    /// components are driven by current-source equations and must therefore
    /// be rejected before numerical conditioning invents an operating point.
    pub(crate) fatal_no_dc_path_nodes: Vec<String>,
    /// Conductance of the authored `.OPTIONS RSHUNT` resistor from every
    /// electrical node to ground. This is part of the physical circuit and is
    /// kept separate from solver-controlled GMIN/conditioning diagonals.
    pub(crate) global_shunt_conductance: Value,
    /// Floating DC-conductive component index for each electrical node ID.
    /// Ground and nodes outside a floating component contain `None`.
    pub(crate) dc_floating_component_by_node: Vec<Option<usize>>,
    /// Original node spellings for each floating component, retained for
    /// deterministic diagnostics after circuit assembly.
    pub(crate) dc_floating_component_nodes: Vec<Vec<String>>,
    /// Whether each floating component is untouched by a model-specific DC
    /// terminal grouping and can therefore support a fatal topology proof.
    pub(crate) dc_floating_component_is_certain: Vec<bool>,
    pub(crate) bsim3v3: Bsim3v3s,
    pub(crate) bsim4v8: Bsim4v8s,
    pub(crate) ekv26s: EkvMosfets,
    pub(crate) ekv3s: Ekv3Mosfets,
    pub(crate) vdmoses: Vdmoses,
    pub(crate) jfets: Vec<crate::device::Jfet>,
    pub(crate) xyce_memristors: Vec<XyceMemristorBinding>,
    /// Node IDs occupied by private non-electrical DAE state variables.
    /// Maintained alongside node remapping so hot solver classification is O(1).
    pub(crate) non_electrical_state_nodes: HashSet<NodeId>,
    /// Selects each family-specific state equation during a DC/startup
    /// operating-point solve instead of its transient dynamic equation.
    pub(crate) xyce_memristor_operating_point_mode: bool,

    // Controlled sources
    pub(crate) vcvs: Vcvs,
    pub(crate) vccs: Vccs,
    pub(crate) cccs: Cccs,
    pub(crate) ccvs: Ccvs,

    // Pending control element resolutions (element name -> CCCS/CCVS indices)
    /// CCCS elements pending control branch resolution: (cccs_index, control_element_name)
    pending_cccs: Vec<(usize, String)>,
    /// CCVS elements pending control branch resolution: (ccvs_index, control_element_name)
    pending_ccvs: Vec<(usize, String)>,
    /// ISWITCH elements pending control branch resolution: (iswitch_index, control_element_name)
    pending_iswitch: Vec<(usize, String)>,

    // Advanced device storage
    pub(crate) vswitches: Vec<crate::device::VoltageSwitch>,
    pub(crate) iswitches: Vec<crate::device::CurrentSwitch>,
    pub(crate) generic_switches: Vec<crate::device::GenericSwitch>,
    pub(crate) tlines: Vec<crate::device::TransmissionLine>,
    pub(crate) coupled_tlines: Vec<crate::device::CoupledTransmissionLine>,
    pub(crate) couplings: Vec<crate::device::InductorCoupling>,
    pub(crate) coupled_inductor_pairs: Vec<CoupledInductorPairBinding>,
    pub(crate) multi_winding_transformers: Vec<MultiWindingTransformerBinding>,
    pub(crate) jiles_atherton_inductors: Vec<JilesAthertonBinding>,
    /// Shared nonlinear Xyce Core devices with two or more windings.
    pub(crate) xyce_core_groups: Vec<XyceCoreGroupBinding>,
    /// Whether the most recent transient Newton assembly encountered an
    /// unsolved Xyce Core constitutive endpoint.  A Core trial that cannot
    /// solve its hidden magnetic equation must reject the candidate step;
    /// falling back to the ordinary linear companion would silently solve a
    /// different circuit equation.
    pub(crate) xyce_core_trial_invalid: bool,
    /// Explicit branch residuals from the current Xyce Core transient stamp.
    ///
    /// The correction-form Newton solve uses `J * delta = -F`.  Reconstructing
    /// `-F` as `b - A*x` can erase the small constitutive residual when the
    /// companion/history terms are much larger than the Core equation, so the
    /// stamp retains the exact residual for the branch rows it owns.
    pub(crate) xyce_core_transient_residuals: Vec<(usize, Value)>,
    /// Circuit-level transient step-size hint for synthesized distributed
    /// structures that need finer temporal resolution than the user-level
    /// `.tran` print/max-step request to preserve propagation fidelity.
    pub(crate) transient_max_step_hint: Option<Value>,

    // Behavioral sources (expression-based B-elements)
    pub(crate) behavioral_sources: BehavioralSources,

    // XSPICE code model instances
    /// XSPICE instance storage for code model evaluation
    pub(crate) xspice_instances: Vec<XspiceInstance>,
    /// Cached presence of event-driven XSPICE ports.
    pub(crate) xspice_has_event_driven_devices: bool,
    /// Circuit-level digital node values driven by XSPICE events.
    pub(crate) xspice_digital_values: HashMap<NodeId, DigitalValue>,
    /// Per-output digital driver values, resolved onto digital nodes.
    pub(crate) xspice_digital_drivers: XspiceDigitalDrivers,
    /// Last event time per XSPICE digital node.
    pub(crate) xspice_digital_event_times: HashMap<NodeId, Value>,
    /// ngspice-style total LOAD() contribution per event node.
    pub(crate) xspice_event_loads: HashMap<NodeId, Value>,
    /// Circuit-level real-valued event node values driven by XSPICE events.
    pub(crate) xspice_real_values: HashMap<NodeId, Value>,
    /// Per-output real-valued event drivers, summed onto real nodes.
    pub(crate) xspice_real_drivers: XspiceRealDrivers,
    /// Last event time per XSPICE real-valued event node.
    pub(crate) xspice_real_event_times: HashMap<NodeId, Value>,
    /// Circuit-level XSPICE digital event queue.
    pub(crate) xspice_event_queue: XspiceEventScheduler,
    /// Scratch nodes touched while applying a batch of XSPICE digital events.
    pub(crate) xspice_touched_digital_nodes: Vec<NodeId>,
    /// Scratch nodes touched while applying a batch of XSPICE real-valued events.
    pub(crate) xspice_touched_real_nodes: Vec<NodeId>,
    /// XSPICE code model registry (shared across instances)
    pub(crate) xspice_registry: Arc<CodeModelRegistry>,
    /// First XSPICE evaluation failure seen during the current analysis.
    pub(crate) xspice_evaluation_error: Option<String>,

    // Verilog-A devices (feature-gated)
    #[cfg(feature = "veriloga")]
    pub(crate) veriloga_devices: crate::device::veriloga::VerilogADevices,
    #[cfg(feature = "veriloga-builtins-base")]
    pub(crate) generated_veriloga_devices: crate::device::veriloga_builtins::BuiltinVerilogADevices,
    /// Solver-controlled `$simparam` environment. This is deliberately not
    /// part of nonlinear device snapshots: rollback must not change the
    /// continuation stage selected by the engine.
    #[cfg(feature = "veriloga-builtins-base")]
    pub(crate) generated_simulation_parameters:
        crate::device::veriloga_builtins::GeneratedSimulationParameters,
}

/// Stable probe metadata for inductor-backed branch measurements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InductorProbeInfo {
    pub canonical_name: String,
    pub branch_ordinal: NodeId,
    pub state_index: usize,
}

/// Adapter to use StaticMatrix with MatrixStamper trait
struct StaticMatrixStamper<'a> {
    matrix: &'a mut StaticMatrix,
    rhs: &'a mut [Value],
}

impl<'a> MatrixStamper for StaticMatrixStamper<'a> {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if row > 0 && col > 0 {
            self.matrix.add(row - 1, col - 1, value);
        }
    }

    fn stamp_rhs(&mut self, index: NodeId, value: Value) {
        if index > 0 && index <= self.rhs.len() {
            self.rhs[index - 1] += value;
        }
    }
}

impl Default for CircuitData {
    fn default() -> Self {
        Self::new()
    }
}
