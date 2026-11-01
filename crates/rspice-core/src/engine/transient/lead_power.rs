//! Xyce's lead power, `P(dev)` and its alias `W(dev)`, over an accepted
//! transient: each lead current times the voltage across that lead, summed
//! as `N_IO_Op.C` (`BranchData*PowerOp::get`, 1296-1450) sums them.
//!
//! - a two-terminal device, and a controlled source or switch through its
//!   output leads: `I * (V+ - V-)`;
//! - a BJT: `|IB * (VB - VE)| + |IC * (VC - VE)|`;
//! - a MOSFET, JFET or MESFET: `ID * (VD - VS) + IG * (VG - VS)`.
//!
//! The lead currents are the run's own records (a derived or solved branch
//! for a two-terminal device, the device trace for a transistor), and the
//! voltages are the device's external nodes, which a power request retains.

use crate::Value;
use crate::analysis::transient::{TransientDeviceOpTrace, TransientResult};
use crate::engine::SimulationError;
use crate::netlist::{
    ElementKind, Netlist, OutputAnalysisKind, OutputSymbolKind, canonical_symbol,
    measure_output_dependencies,
};

/// How one device's power is summed from its leads.
#[derive(Debug, Clone, PartialEq)]
enum Leads {
    TwoTerminal {
        positive: String,
        negative: String,
    },
    Bjt {
        collector: String,
        base: String,
        emitter: String,
    },
    Fet {
        drain: String,
        gate: String,
        source: String,
    },
}

/// One `P()`/`W()` request, resolved against the flattened deck.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct LeadPowerRequest {
    device: String,
    leads: Leads,
}

/// Whether any transient output or measure names `P(dev)` or `W(dev)`.
pub(super) fn requests_lead_power(netlist: &Netlist) -> bool {
    !requested_devices(netlist).is_empty()
}

fn is_power_operator(operator: &str) -> bool {
    operator.eq_ignore_ascii_case("P") || operator.eq_ignore_ascii_case("W")
}

fn requested_devices(netlist: &Netlist) -> Vec<String> {
    let mut devices: Vec<String> = Vec::new();
    let mut push = |symbol: &str| {
        if !devices
            .iter()
            .any(|known| canonical_symbol(known) == canonical_symbol(symbol))
        {
            devices.push(symbol.trim().to_owned());
        }
    };
    for request in &netlist.output_requests {
        if request
            .analysis
            .is_some_and(|analysis| analysis != OutputAnalysisKind::Tran)
        {
            continue;
        }
        for dependency in &request.dependencies {
            if dependency.kind == OutputSymbolKind::Device
                && is_power_operator(&dependency.operator)
            {
                push(&dependency.symbol);
            }
        }
    }
    for statement in &netlist.measurements {
        if OutputAnalysisKind::from_keyword(&statement.analysis) != Some(OutputAnalysisKind::Tran) {
            continue;
        }
        for dependency in measure_output_dependencies(statement) {
            if dependency.kind == OutputSymbolKind::Device
                && is_power_operator(&dependency.operator)
            {
                push(&dependency.symbol);
            }
        }
    }
    devices
}

/// Resolve every requested device's leads. A device the deck does not hold,
/// or one whose power this does not define, is left to the output resolver,
/// which reports it as unavailable.
pub(super) fn requested_lead_power(
    netlist: &Netlist,
) -> Result<Vec<LeadPowerRequest>, SimulationError> {
    let devices = requested_devices(netlist);
    if devices.is_empty() {
        return Ok(Vec::new());
    }
    let elements = crate::netlist::flatten_netlist(netlist)
        .map_err(|error| SimulationError::Netlist(error.to_string()))?;
    let mut requests = Vec::new();
    for device in devices {
        let key = canonical_symbol(&device);
        let Some(element) = elements
            .iter()
            .find(|element| canonical_symbol(&element.name) == key)
        else {
            continue;
        };
        let node = |index: usize| element.nodes.get(index).cloned();
        let leads = match &element.kind {
            ElementKind::Bjt { .. } => match (node(0), node(1), node(2)) {
                (Some(collector), Some(base), Some(emitter)) => Leads::Bjt {
                    collector,
                    base,
                    emitter,
                },
                _ => continue,
            },
            ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } => {
                match (node(0), node(1), node(2)) {
                    (Some(drain), Some(gate), Some(source)) => Leads::Fet {
                        drain,
                        gate,
                        source,
                    },
                    _ => continue,
                }
            }
            ElementKind::Resistor { .. }
            | ElementKind::Capacitor { .. }
            | ElementKind::Inductor { .. }
            | ElementKind::VoltageSource(_)
            | ElementKind::CurrentSource(_)
            | ElementKind::Diode { .. }
            | ElementKind::XyceMemristor { .. }
            | ElementKind::Vcvs { .. }
            | ElementKind::Cccs { .. }
            | ElementKind::Vccs { .. }
            | ElementKind::Ccvs { .. }
            | ElementKind::BehavioralVoltage { .. }
            | ElementKind::BehavioralCurrent { .. }
            | ElementKind::VSwitch { .. }
            | ElementKind::ISwitch { .. }
            | ElementKind::GenericSwitch { .. } => match (node(0), node(1)) {
                (Some(positive), Some(negative)) => Leads::TwoTerminal { positive, negative },
                _ => continue,
            },
            _ => continue,
        };
        requests.push(LeadPowerRequest {
            device: element.name.clone(),
            leads,
        });
    }
    Ok(requests)
}

/// Publish each request's power as the device trace `P`, which the signal
/// table also spells `W(dev)`. A request whose lead currents or voltages the
/// run did not record publishes nothing.
pub(super) fn record_lead_power(requests: &[LeadPowerRequest], result: &mut TransientResult) {
    let points = result.time.len();
    let traces: Vec<TransientDeviceOpTrace> = requests
        .iter()
        .filter(|request| {
            result
                .try_device_op_waveform_named(&request.device, "P")
                .is_none()
        })
        .filter_map(|request| {
            let values = lead_power(request, result)?;
            (values.len() == points).then(|| TransientDeviceOpTrace {
                device_name: request.device.clone(),
                parameter: "P".to_owned(),
                values,
            })
        })
        .collect();
    result.device_op_traces.extend(traces);
}

fn lead_power(request: &LeadPowerRequest, result: &TransientResult) -> Option<Vec<Value>> {
    let points = result.time.len();
    let voltage = |node: &str| -> Option<Vec<Value>> {
        let key = canonical_symbol(node);
        if matches!(key.as_str(), "0" | "GND" | "GND!") {
            return Some(vec![0.0; points]);
        }
        let index = result
            .node_names
            .iter()
            .position(|name| canonical_symbol(name) == key)?;
        result
            .voltages
            .get(index)
            .filter(|values| values.len() == points)
            .cloned()
    };
    let across = |positive: &str, negative: &str| -> Option<Vec<Value>> {
        let (positive, negative) = (voltage(positive)?, voltage(negative)?);
        Some(positive.iter().zip(&negative).map(|(p, n)| p - n).collect())
    };
    let device_trace = |parameter: &str| {
        result
            .try_device_op_waveform_named(&request.device, parameter)
            .filter(|values| values.len() == points)
    };
    match &request.leads {
        Leads::TwoTerminal { positive, negative } => {
            let key = canonical_symbol(&request.device);
            let current = result
                .branch_names
                .iter()
                .position(|name| canonical_symbol(name) == key)
                .and_then(|index| result.branch_currents.get(index))
                .filter(|values| values.len() == points)?;
            let across = across(positive, negative)?;
            Some(current.iter().zip(&across).map(|(i, v)| i * v).collect())
        }
        Leads::Bjt {
            collector,
            base,
            emitter,
        } => {
            let (ib, ic) = (device_trace("ib")?, device_trace("ic")?);
            let (vbe, vce) = (across(base, emitter)?, across(collector, emitter)?);
            Some(
                (0..points)
                    .map(|k| (ib[k] * vbe[k]).abs() + (ic[k] * vce[k]).abs())
                    .collect(),
            )
        }
        Leads::Fet {
            drain,
            gate,
            source,
        } => {
            let (id, ig) = (device_trace("id")?, device_trace("ig")?);
            let (vds, vgs) = (across(drain, source)?, across(gate, source)?);
            Some(
                (0..points)
                    .map(|k| id[k] * vds[k] + ig[k] * vgs[k])
                    .collect(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::abort_signal::NoAbort;
    use crate::analysis::measure_signals::evaluate_tran_output_requests_with_abort;
    use crate::netlist::Netlist;

    #[test]
    fn a_transient_prints_each_device_lead_power_as_xyce_sums_it() {
        let netlist = Netlist::parse_with_options(
            "lead power\n\
             V1 1 0 PWL 0 0 1u 2\n\
             R1 1 2 100\n\
             C1 2 0 1n\n\
             D1 2 3 DM\n\
             R2 3 0 50\n\
             Q1 4 2 0 QM\n\
             RC 1 4 1k\n\
             M1 5 2 0 0 NM L=1u W=10u\n\
             RD 1 5 1k\n\
             .MODEL DM D IS=1e-14\n\
             .MODEL QM NPN BF=100 IS=1e-16\n\
             .MODEL NM NMOS LEVEL=1 VTO=0.5 KP=1e-4\n\
             .TRAN 0 1u\n\
             .PRINT TRAN P(R1) W(C1) P(D1) P(V1) P(Q1) P(M1) I(R1) I(C1) I(D1) I(V1)\n\
             + IB(Q1) IC(Q1) ID(M1) IG(M1) V(1) V(2) V(3) V(4) V(5)\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce deck parses");
        let engine = crate::Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::config::SpiceDialect::Xyce),
        );
        let result = engine
            .run_tran(&netlist, 1e-6, 1e-8)
            .expect("transient runs");
        let columns = evaluate_tran_output_requests_with_abort(
            &netlist,
            &result,
            Default::default(),
            &NoAbort,
        )
        .expect("every lead power resolves");
        let column = |name: &str| {
            &columns
                .iter()
                .find(|(column, _, _)| column == name)
                .unwrap_or_else(|| panic!("no column {name}"))
                .2
        };
        let rows = result.time.len();
        let at = |name: &str, row: usize| column(name)[row];
        for row in 0..rows {
            let v = |node: &str| at(&format!("V({node})"), row);
            let expected = [
                ("P(R1)", at("I(R1)", row) * (v("1") - v("2"))),
                ("W(C1)", at("I(C1)", row) * v("2")),
                ("P(D1)", at("I(D1)", row) * (v("2") - v("3"))),
                ("P(V1)", at("I(V1)", row) * v("1")),
                (
                    "P(Q1)",
                    (at("IB(Q1)", row) * v("2")).abs() + (at("IC(Q1)", row) * v("4")).abs(),
                ),
                (
                    "P(M1)",
                    at("ID(M1)", row) * v("5") + at("IG(M1)", row) * v("2"),
                ),
            ];
            for (name, expected) in expected {
                let actual = at(name, row);
                assert!(
                    (actual - expected).abs() <= 1e-12 * expected.abs() + 1e-30,
                    "{name} row {row}: {actual} != {expected}"
                );
            }
        }
        // Xyce 7.10 at 1us: P(R1) 1.32486976e-2, P(D1) 2.23902902e-3,
        // P(V1) -2.71004978e-2, P(Q1) 6.99625551e-3, P(M1) 1.18072558e-4.
        // The capacitor's current at the PWL corner depends on the step that
        // lands there, so W(C1) is held to the identity above only.
        for (name, xyce) in [
            ("P(R1)", 1.32486976e-2),
            ("P(D1)", 2.23902902e-3),
            ("P(V1)", -2.71004978e-2),
            ("P(Q1)", 6.99625551e-3),
            ("P(M1)", 1.18072558e-4),
        ] {
            let actual = at(name, rows - 1);
            assert!(
                (actual - xyce).abs() <= 1e-2 * xyce.abs(),
                "{name} at 1us: {actual} against Xyce {xyce}"
            );
        }
    }
}
