//! The design's nets an initial condition or nodeset can name.

use rspice_design::connectivity::summary::{DesignNet, NetDomain};
use rspice_simulation_contract::options::{SimulationOptions, StartupKind, net_name_problem};

use crate::preparation::{PreparationError, PreparationStage};

/// The net of the run's root `name` names, compared as SPICE compares node
/// names, or why a startup card cannot hold it: no such net, ground, or a
/// net that carries no voltage.
pub fn startup_net<'a>(name: &str, nets: &'a [DesignNet]) -> Result<&'a DesignNet, String> {
    let name = name.trim();
    if let Some(problem) = net_name_problem(name) {
        return Err(problem);
    }
    let net = nets
        .iter()
        .find(|net| net.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| format!("the design has no net {name}"))?;
    match net.domain {
        NetDomain::Electrical | NetDomain::Mixed(_) => Ok(net),
        domain => Err(format!(
            "{} is a {} net, which holds no voltage",
            net.name,
            domain.label()
        )),
    }
}

/// Refuse a run whose initial conditions or nodesets name a net the root no
/// longer has: the engine would ignore the card, and the run would start
/// somewhere the plan does not say.
pub fn check_startup_nets(
    options: &SimulationOptions,
    nets: &[DesignNet],
) -> Result<(), PreparationError> {
    for (kind, rows) in [
        (StartupKind::InitialCondition, &options.initial_conditions),
        (StartupKind::NodeSet, &options.nodesets),
    ] {
        for row in rows {
            startup_net(&row.net, nets).map_err(|problem| {
                PreparationError::new(
                    PreparationStage::DesignChecks,
                    format!("{} {}: {problem}", kind.label(), row.net),
                )
            })?;
        }
    }
    Ok(())
}
