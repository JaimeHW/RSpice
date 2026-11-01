//! Xyce device-current and power wildcards on `.PRINT` cards.
//!
//! Xyce replaces an `I()`, lead-current, `P()` or `W()` operand whose device
//! name holds `*` or `?` with one operand per matching device before the run
//! (N_IO_OutputMgr.C:2947-3011, `removeWildcardVariables`). A device matches
//! when it carries the requested lead and its name matches the pattern, `*`
//! standing for any run of characters and `?` for exactly one
//! (N_IO_WildcardSupport.C:41-68). RSpice rewrites the card the same way at
//! parse time, so the rest of the pipeline sees ordinary device operands.
//! Xyce's own column order follows a hash map; RSpice lists the devices in
//! flattened circuit order. `V()` wildcards stay on the card: the output
//! projection expands them against the result's node names.

use super::flattener::{Flattener, FlattenerConfig};
use super::output_symbols::{
    OutputAnalysisKind, OutputDirectiveKind, OutputOperand, OutputOperandKind, OutputRequest,
    hierarchy_pattern_matches,
};
use super::{Element, ElementKind, Netlist, ParseWithAbortError, SaveSignal};
use crate::abort_signal::AbortSignal;

/// The lead quantities one Xyce wildcard operator selects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WildcardLeads {
    /// `I(*)` and its AC parts: the lead current of a two-terminal device,
    /// whose Xyce lead entry ends in `BRANCH_D` (N_IO_OutputMgr.C:3053-3065).
    TwoTerminal,
    /// `IB(*)`, `IC(*)`, `ID(*)` and so on: one named lead of a
    /// multi-terminal device (entries ending in `BRANCH_DB`, `BRANCH_DC`, ...).
    Lead(char),
    /// `I1(*)` and `I2(*)`: a port current of a lossless transmission line.
    Port,
    /// `P(*)` and `W(*)`: every device with lead currents, once each
    /// (N_IO_OutputMgr.C:3110-3146, `getPWildcardList`).
    Power,
}

/// One wildcard operand found on a card.
struct DeviceWildcard {
    operator: String,
    leads: WildcardLeads,
    pattern: String,
}

/// Device letters whose Xyce instances carry a single `BRANCH_D` lead entry.
const TWO_TERMINAL_DEVICES: &str = "BCDEFGHILRSVW";

/// The device wildcard an operand names, if it is one.
fn device_wildcard(kind: &OutputOperandKind) -> Option<DeviceWildcard> {
    let (operator, argument) = match kind {
        OutputOperandKind::Probe(SaveSignal::Current(device)) => ("I", device.as_str()),
        OutputOperandKind::Expression { body } => {
            let (operator, rest) = body.split_once('(')?;
            (operator.trim(), rest.strip_suffix(')')?.trim())
        }
        OutputOperandKind::Probe(_) => return None,
    };
    if !argument.contains(['*', '?']) || argument.contains(['(', ')', ',', '{', '}', '\'']) {
        return None;
    }
    let operator = operator.to_ascii_uppercase();
    let leads = match operator.as_str() {
        "I" | "IR" | "II" | "IM" | "IP" | "IDB" => WildcardLeads::TwoTerminal,
        "IB" | "IC" | "IE" | "IS" | "ID" | "IG" => WildcardLeads::Lead(operator.chars().nth(1)?),
        "I1" | "I2" => WildcardLeads::Port,
        "P" | "W" => WildcardLeads::Power,
        _ => return None,
    };
    Some(DeviceWildcard {
        operator,
        leads,
        pattern: argument.to_ascii_uppercase(),
    })
}

/// Whether a device of this Xyce type letter carries the requested leads.
///
/// AC and noise analyses keep only branch-current unknowns, so there `I(*)`
/// selects the devices that own one (N_IO_OutputMgr.C:3047-3052).
fn device_has_leads(
    element: &Element,
    letter: char,
    leads: WildcardLeads,
    branch_only: bool,
) -> bool {
    match leads {
        WildcardLeads::TwoTerminal if branch_only => matches!(
            element.kind,
            ElementKind::VoltageSource(_)
                | ElementKind::Inductor { .. }
                | ElementKind::Vcvs { .. }
                | ElementKind::Ccvs { .. }
                | ElementKind::BehavioralVoltage { .. }
        ),
        WildcardLeads::TwoTerminal => TWO_TERMINAL_DEVICES.contains(letter),
        WildcardLeads::Lead(lead) => match letter {
            'Q' => "BCES".contains(lead),
            'M' => "DGSB".contains(lead),
            'J' | 'Z' => "DGS".contains(lead),
            _ => false,
        },
        WildcardLeads::Port => letter == 'T',
        WildcardLeads::Power => TWO_TERMINAL_DEVICES.contains(letter) || "JMQTZ".contains(letter),
    }
}

/// The Xyce spelling of a flattened device name: upper case, `:` between levels.
fn xyce_device_name(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch == '.' {
                ':'
            } else {
                ch.to_ascii_uppercase()
            }
        })
        .collect()
}

/// The operand kind the parser gives `probe`, with the save signal it records.
fn device_operand(probe: String, saves: &mut Vec<SaveSignal>) -> OutputOperand {
    let direct = probe
        .split_once('(')
        .is_some_and(|(operator, _)| operator.eq_ignore_ascii_case("I"));
    let kind = match super::parser::parse_save_probe(&probe) {
        Some(signal) if direct => {
            saves.push(signal.clone());
            OutputOperandKind::Probe(signal)
        }
        _ => {
            saves.push(SaveSignal::Raw(probe.clone()));
            OutputOperandKind::Expression {
                body: probe.clone(),
            }
        }
    };
    OutputOperand {
        authored: probe,
        kind,
    }
}

/// Expand every Xyce device wildcard on the deck's `.PRINT` cards.
///
/// A pattern other than `*` that names no device at all stays on the card,
/// so output validation refuses it as Xyce does ("undefined symbols in .PRINT
/// command", N_IO_NetlistImportTool.C:962-976).
pub(super) fn expand_xyce_device_wildcards_with_abort(
    netlist: &mut Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let expands = |request: &OutputRequest| {
        request.directive == OutputDirectiveKind::Print
            && request
                .operand_kinds
                .iter()
                .any(|kind| device_wildcard(kind).is_some())
    };
    if !netlist.output_requests.iter().any(expands) {
        return Ok(());
    }
    let mut flattener = Flattener::with_models_config(
        &netlist.subcircuits,
        &netlist.models,
        FlattenerConfig::default(),
    );
    let elements = match flattener.flatten_with_abort(netlist, abort) {
        Ok(elements) => elements,
        Err(ParseWithAbortError::Aborted) => return Err(ParseWithAbortError::Aborted),
        // Elaboration reports its own failure later; keep the cards as authored.
        Err(ParseWithAbortError::Parse(_)) => return Ok(()),
    };
    let devices = elements
        .iter()
        .filter_map(|element| {
            let name = xyce_device_name(&element.name);
            let letter = name.rsplit(':').next()?.chars().next()?;
            Some((element, name, letter))
        })
        .collect::<Vec<_>>();

    let mut saves = Vec::new();
    let mut expanded_saves = Vec::new();
    for request in &mut netlist.output_requests {
        if !expands(request) {
            continue;
        }
        let branch_only = matches!(
            request.analysis,
            Some(OutputAnalysisKind::Ac | OutputAnalysisKind::Noise)
        );
        let mut operands = Vec::with_capacity(request.operands.len());
        for (authored, kind) in request.operands.iter().zip(&request.operand_kinds) {
            let wildcard = device_wildcard(kind).filter(|wildcard| {
                wildcard.pattern == "*"
                    || devices
                        .iter()
                        .any(|(_, name, _)| hierarchy_pattern_matches(&wildcard.pattern, name))
            });
            let Some(wildcard) = wildcard else {
                operands.push(OutputOperand {
                    authored: authored.clone(),
                    kind: kind.clone(),
                });
                continue;
            };
            // The parser recorded the pattern itself as a save signal; the
            // projection would refuse it as a signal no result carries.
            expanded_saves.push(match kind {
                OutputOperandKind::Probe(signal) => signal.clone(),
                OutputOperandKind::Expression { body } => SaveSignal::Raw(body.clone()),
            });
            for (index, (element, name, letter)) in devices.iter().enumerate() {
                if index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(ParseWithAbortError::Aborted);
                }
                if device_has_leads(element, *letter, wildcard.leads, branch_only)
                    && (wildcard.pattern == "*"
                        || hierarchy_pattern_matches(&wildcard.pattern, name))
                {
                    operands.push(device_operand(
                        format!("{}({name})", wildcard.operator),
                        &mut saves,
                    ));
                }
            }
        }
        let mut expanded = OutputRequest::from_ordered_operands(
            request.directive,
            request.origin.clone(),
            request.analysis,
            operands,
        );
        expanded.name = request.name.take();
        expanded.print_delimiter = request.print_delimiter.take();
        expanded.print_precision = request.print_precision;
        expanded.print_width = request.print_width;
        *request = expanded;
    }
    for signal in expanded_saves {
        if let Some(index) = netlist
            .saves
            .signals
            .iter()
            .position(|saved| *saved == signal)
        {
            netlist.saves.signals.remove(index);
        }
    }
    netlist.saves.signals.extend(saves);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::netlist::{Netlist, NetlistParseOptions, SaveSignal};

    fn xyce_print_operands(deck: &str) -> Vec<Vec<String>> {
        let netlist = Netlist::parse_with_options(
            deck,
            NetlistParseOptions {
                spice_dialect: Some(crate::config::SpiceDialect::Xyce),
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce wildcard deck parses");
        assert!(
            !netlist.saves.signals.iter().any(|signal| matches!(
                signal,
                SaveSignal::Current(name) | SaveSignal::Raw(name) if name.contains('*') && name != "X5*"
            )),
            "an expanded pattern leaves no save signal behind: {:?}",
            netlist.saves.signals
        );
        netlist
            .output_requests
            .iter()
            .map(|request| request.operands.clone())
            .collect()
    }

    #[test]
    fn current_and_power_wildcards_expand_to_each_matching_device_in_circuit_order() {
        let operands = xyce_print_operands(
            "wildcards\n\
             V1 1 0 1\n\
             R1 1 2 1\n\
             X1 2 0 sub\n\
             Q1 3 2 0 qmod\n\
             M1 3 2 0 0 mmod\n\
             .model qmod npn\n\
             .model mmod nmos\n\
             .subckt sub a b\n\
             R1 a c 1\n\
             L1 c b 1u\n\
             .ends\n\
             .tran 1n 10n\n\
             .print tran V(1) I(X1*) IB(*) P(*) W(R?)\n\
             .end\n",
        );
        assert_eq!(
            operands,
            [[
                "V(1)", "I(X1:R1)", "I(X1:L1)", "IB(Q1)", "IB(M1)", "P(V1)", "P(R1)", "P(X1:R1)",
                "P(X1:L1)", "P(Q1)", "P(M1)", "W(R1)",
            ]
            .map(String::from)
            .to_vec()]
        );
    }

    #[test]
    fn ac_current_wildcards_keep_branch_devices_and_an_unmatched_pattern_stays_for_validation() {
        let operands = xyce_print_operands(
            "ac wildcards\n\
             V1 a 0 DC 0 AC 1\n\
             L1 a b 1u\n\
             R1 b 0 2\n\
             .ac dec 1 1k 10k\n\
             .print ac I(*) IM(*) I(X5*) I1(*)\n\
             .end\n",
        );
        assert_eq!(
            operands,
            [["I(V1)", "I(L1)", "IM(V1)", "IM(L1)", "I(X5*)"]
                .map(String::from)
                .to_vec()]
        );
    }
}
