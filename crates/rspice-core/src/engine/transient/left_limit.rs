//! The row an event step publishes.
//!
//! At a physical event the step that ends there solved the incoming interval;
//! the event then carries the state through the instant, and the integration
//! leaves from that outgoing state, a right limit. SPICE publishes the left
//! limit, the solution of the step that ends there, as at any breakpoint. The
//! row is therefore composed from an ordinary acceptance of the step's own
//! solution on copies of the circuit and of the device histories, so its node
//! voltages, branch currents, derived currents and device values all come
//! from that one solution and its own companion state. Nothing the copies
//! accept is integrated from.
use super::*;

/// The accepted copies an event row is recorded from.
pub(super) struct LeftLimitRow {
    pub circuit: crate::circuit::CircuitData,
    pub solution: Vec<Value>,
    pub bjt: BjtTransientHistory,
    pub jfet: JfetTransientHistory,
    pub diode: DiodeTransientHistory,
    pub mosfet: MosfetTransientHistory,
}

/// The event step as the ordinary acceptance would see it.
pub(super) struct LeftLimitStep<'a> {
    pub circuit: &'a crate::circuit::CircuitData,
    pub matrix: &'a mut crate::solver::StaticMatrix,
    pub incoming: &'a [Value],
    pub time: Value,
    pub dt: Value,
    pub coefficients: &'a CompanionCoefficients,
    pub xyce_one_step_order2: bool,
    pub baseline_diag_gmin: Value,
    pub initial_step: bool,
    pub final_step: bool,
    pub retained_transport_bytes: usize,
    pub histories: TransientDeviceHistories<'a>,
    pub bsim4_trnqs_coeff: &'a CompanionCoefficients,
    /// The snapshots of the step's own trial; the event's are not used.
    pub snapshots: AcceptedReactiveSnapshots<'a>,
    pub breakpoints: &'a BreakpointManager,
    pub tstop: Value,
    pub dynamic_breakpoints_added: usize,
    pub pending_dynamic_breakpoints: &'a [Value],
}

impl Engine {
    pub(super) fn accept_left_limit_copy(
        &self,
        step: LeftLimitStep<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<LeftLimitRow, SimulationError> {
        let mut circuit = step.circuit.clone();
        let mut solution = step.incoming.to_vec();
        let mut bjt = step.histories.bjt.clone();
        let mut jfet = step.histories.jfet.clone();
        let mut diode = step.histories.diode.clone();
        let mut mosfet = step.histories.mosfet.clone();
        let mut vdmos = step.histories.vdmos.clone();
        let mut b3soi = step.histories.b3soi.clone();
        let mut bsim3 = step.histories.bsim3.clone();
        let mut bsim4 = step.histories.bsim4.clone();
        let mut ekv26 = step.histories.ekv26.clone();
        let mut breakpoints = step.breakpoints.empty_like();
        let mut dynamic_breakpoints_added = step.dynamic_breakpoints_added;
        let mut warned_dynamic_breakpoint_cap = false;
        let mut pending_dynamic_breakpoints = step.pending_dynamic_breakpoints.to_vec();
        self.accept_transient_models(
            abort,
            &mut circuit,
            step.matrix,
            &mut solution,
            step.time,
            step.dt,
            step.coefficients,
            step.xyce_one_step_order2,
            false,
            step.baseline_diag_gmin,
            step.initial_step,
            step.final_step,
            Some(acceptance::NativeHistoryAcceptance {
                retained_transport_bytes: step.retained_transport_bytes,
                histories: TransientDeviceHistories {
                    bjt: &mut bjt,
                    jfet: &mut jfet,
                    diode: &mut diode,
                    mosfet: &mut mosfet,
                    vdmos: &mut vdmos,
                    b3soi: &mut b3soi,
                    bsim3: &mut bsim3,
                    bsim4: &mut bsim4,
                    ekv26: &mut ekv26,
                },
                bsim4_trnqs_coeff: step.bsim4_trnqs_coeff,
                snapshots: AcceptedReactiveSnapshots {
                    physical_event: None,
                    left_limit_observation: true,
                    ..step.snapshots
                },
                scheduling: ReactiveBreakpointScheduling {
                    breakpoints: &mut breakpoints,
                    tstop: step.tstop,
                    voltage_reltol: self.voltage_reltol(),
                    voltage_abstol: self.voltage_abstol(),
                    current_abstol: self.current_abstol(),
                },
                sink: DynamicBreakpointSink {
                    dynamic_breakpoints_added: &mut dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap: &mut warned_dynamic_breakpoint_cap,
                    pending_dynamic_breakpoints: &mut pending_dynamic_breakpoints,
                },
            }),
        )?;
        Ok(LeftLimitRow {
            circuit,
            solution,
            bjt,
            jfet,
            diode,
            mosfet,
        })
    }
}
