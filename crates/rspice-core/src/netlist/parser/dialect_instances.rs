//! Subcircuit-instance readings particular to the ngspice dialect. Each needs
//! every `.subckt` definition in hand, so they run once the deck is parsed.

use super::*;

/// Read the deck's X lines as ngspice does, and say where that differs from
/// what was written.
///
/// - ngspice matches X-line parameters against one list, the subcircuit's
///   formals and its body `.param`s (inpcom.c), and drops a name that matches
///   none of them without a word. The dialect drops it too, with a warning
///   naming the card and the parameter. When the list is empty, ngspice
///   stops on "Mismatch: 0 formal but N actual params"; the flattener's
///   refusal stands for that case, so it is left alone here.
/// - ngspice never translates a `.global` node, so a port spelled as one is
///   the global net whatever the X line connects there. The flattener accepts
///   that binding in this dialect; this warns that the connection is not made.
pub(super) fn apply_ngspice_instance_readings(netlist: &mut Netlist) {
    struct Interface {
        name: String,
        ports: Vec<String>,
        declared: Vec<String>,
    }
    let interfaces = netlist
        .subcircuits
        .iter()
        .map(|subckt| {
            (
                subckt.name.to_ascii_uppercase(),
                Interface {
                    name: subckt.name.clone(),
                    ports: subckt.ports.clone(),
                    declared: crate::netlist::flattener::subcircuit_declared_parameters(subckt)
                        .map(str::to_owned)
                        .collect(),
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let globals = netlist
        .global_nodes
        .iter()
        .map(|node| node.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let is_global = |node: &str| {
        globals.contains(&node.to_ascii_uppercase())
            || node
                .get(..2)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("$G"))
    };

    let mut warnings = Vec::new();
    let mut read = |scope: Option<&str>, elements: &mut [Element]| {
        for element in elements {
            let ElementKind::Subcircuit {
                subckt_name,
                params,
            } = &mut element.kind
            else {
                continue;
            };
            let Some(interface) = interfaces.get(&subckt_name.to_ascii_uppercase()) else {
                continue;
            };
            let origin = netlist
                .element_origins
                .get(&crate::netlist::element_origin_key(scope, &element.name))
                .cloned();
            let mut warn = |code: &str, message: String| {
                warnings.push(match origin.clone() {
                    Some(origin) => ParseDiagnostic::warning_at(origin, code, message),
                    None => ParseDiagnostic::warning(0, code, message),
                });
            };
            if !interface.declared.is_empty() {
                params.retain(|(name, _)| {
                    let kept = name.eq_ignore_ascii_case("M")
                        || interface
                            .declared
                            .iter()
                            .any(|declared| declared.eq_ignore_ascii_case(name));
                    if !kept {
                        warn(
                            "undeclared-instance-parameter",
                            format!(
                                "Subcircuit instance '{}' sets parameter '{name}', which subcircuit '{}' does not declare; ngspice ignores it, so it is dropped",
                                element.name, interface.name
                            ),
                        );
                    }
                    kept
                });
            }
            for (formal, actual) in interface.ports.iter().zip(&element.nodes) {
                if is_global(formal) && !formal.eq_ignore_ascii_case(actual) {
                    warn(
                        "global-subcircuit-port",
                        format!(
                            "Subcircuit instance '{}' connects '{actual}' to port '{formal}' of '{}', but '{formal}' is a global node; ngspice keeps the port on the global '{formal}' and makes no connection to '{actual}' there",
                            element.name, interface.name
                        ),
                    );
                }
            }
        }
    };
    read(None, &mut netlist.elements);
    for subckt in &mut netlist.subcircuits {
        read(Some(&subckt.name), &mut subckt.elements);
    }
    netlist.diagnostics.extend(warnings);
}
