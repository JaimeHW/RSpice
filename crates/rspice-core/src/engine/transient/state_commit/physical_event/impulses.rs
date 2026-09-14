//! Device charge actions at one physical event, not across an integration step.
use super::*;

pub(in crate::engine::transient) enum PhysicalDeviceImpulses {
    /// The existing physical continuity certificate proves every charge equal.
    Continuous,
    Jumps {
        capacitors: Vec<Value>,
        bjt_terminals: Vec<[Value; 4]>,
    },
}

impl PhysicalDeviceImpulses {
    pub(super) fn prepare(
        continuous: bool,
        capacitors: usize,
        bjts: usize,
    ) -> Result<Self, SimulationError> {
        if continuous {
            return Ok(Self::Continuous);
        }
        let mut capacitor_charges = Vec::new();
        let mut terminal_charges = Vec::new();
        capacitor_charges
            .try_reserve_exact(capacitors)
            .map_err(|_| failure("cannot allocate capacitor impulse preparation"))?;
        terminal_charges
            .try_reserve_exact(bjts)
            .map_err(|_| failure("cannot allocate BJT impulse preparation"))?;
        Ok(Self::Jumps {
            capacitors: capacitor_charges,
            bjt_terminals: terminal_charges,
        })
    }
}

pub(super) fn bjt_terminal_charges(
    model: &crate::device::Bjt,
    incoming_solution: &[Value],
    startup_charges: Option<&[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    outgoing: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ports: &[(Option<usize>, Option<usize>); BJT_DYNAMIC_CHARGE_COUNT],
) -> Result<[Value; 4], SimulationError> {
    let incoming = if let Some(charges) = startup_charges {
        *charges
    } else {
        let (branches, _, _) = model.mna_charge_state_at_solution(incoming_solution);
        let mut charges = branches.map(|branch| branch.charge);
        if let Some(charge) = model.legacy_external_bc_charge(incoming_solution) {
            charges[BJT_QBCX_BRANCH_INDEX] = charge.charge;
        }
        charges
    };
    if ports
        .iter()
        .flat_map(|(p, n)| [p, n])
        .flatten()
        .any(|&port| port >= 4)
    {
        return Err(failure("invalid external BJT charge incidence"));
    }
    let mut differences = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
    for ((delta, &outgoing), incoming) in differences.iter_mut().zip(outgoing).zip(incoming) {
        *delta = sum([(outgoing, 1.0), (incoming, -1.0)].into_iter())?;
    }
    let mut terminals = [0.0; 4];
    for (terminal, charge) in terminals.iter_mut().enumerate() {
        *charge = sum(differences.iter().zip(ports).flat_map(|(&delta, &(p, n))| {
            [
                (delta, if p == Some(terminal) { 1.0 } else { 0.0 }),
                (delta, if n == Some(terminal) { -1.0 } else { 0.0 }),
            ]
        }))?;
    }
    // The branch array deliberately excludes externalized QBCX incidence.
    // It also names the builder's prime nodes when RC/RB/RE are externalized;
    // those intrinsic currents are not the authored device lead currents.
    model
        .authored_transient_lead_impulses(terminals, differences[BJT_QBCX_BRANCH_INDEX])
        .map_err(failure)
}
