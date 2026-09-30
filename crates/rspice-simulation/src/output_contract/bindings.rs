//! Bind authored references once against the engine basis of their owning task.
//! Deferred evaluation uses exact retained columns, never authored output labels
//! or whichever deck happens to be open later.

use std::collections::BTreeMap;

use super::*;
use rspice_core::netlist::GroundPolicy;
use rspice_results::saved_output::saved_output_references;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceCandidate {
    Probe(String),
    Trace(String),
    Ground,
}

pub(super) type Candidates = BTreeMap<String, Vec<SourceCandidate>>;

impl PreparedSavedOutput {
    pub fn bind_deck(contracts: &mut [Self], netlist: &rspice_core::Netlist) -> Result<(), String> {
        let mut references = std::collections::BTreeSet::new();
        for contract in contracts.iter() {
            references.extend(
                saved_output_references(contract.kind, &contract.source_expression)?
                    .into_iter()
                    .flatten(),
            );
        }
        let requested = references
            .iter()
            .filter_map(|signal| {
                let (current, node) = probe_identity(signal);
                if current {
                    None
                } else {
                    Some(
                        rspice_app_types::hierarchy_path::ProbeTarget::engine_alias(node)
                            .unwrap_or_else(|| node.to_owned()),
                    )
                }
            })
            .collect();
        let aliases = rspice_core::netlist::collect_requested_interface_node_aliases_with_abort(
            netlist,
            &requested,
            &rspice_core::abort_signal::NoAbort,
        )
        .map_err(|error| format!("saved-output interface binding failed: {error}"))?;
        let candidates = Arc::new(
            references
                .into_iter()
                .map(|signal| {
                    let list = candidates(&signal, netlist.ground_policy(), Some(&aliases));
                    (signal, list)
                })
                .collect(),
        );
        for contract in contracts {
            contract.candidates = Some(Arc::clone(&candidates));
        }
        Ok(())
    }
}

fn candidates(
    signal: &str,
    ground: GroundPolicy,
    aliases: Option<&rspice_core::netlist::InterfaceNodeAliases>,
) -> Vec<SourceCandidate> {
    if let Some((device, quantity)) = rspice_results::saved_output::device_current_probe(signal) {
        let mut result = vec![SourceCandidate::Trace(signal.to_owned())];
        if let Some(engine) = rspice_app_types::hierarchy_path::ProbeTarget::engine_alias(device) {
            let candidate = SourceCandidate::Trace(format!("@{engine}[{quantity}]"));
            if !result.contains(&candidate) {
                result.push(candidate);
            }
        }
        return result;
    }
    let (current, node) = probe_identity(signal);
    let mut result = vec![SourceCandidate::Trace(signal.to_owned())];
    let mut add = |node: &str| {
        let candidate = if !current && ground.is_ground(node) {
            SourceCandidate::Ground
        } else {
            SourceCandidate::Probe(format!("{}({node})", if current { "I" } else { "V" }))
        };
        if !result.contains(&candidate) {
            result.push(candidate);
        }
    };
    // Literal slash/dotted nodes win before hierarchy and formal-port aliases.
    add(node);
    let engine = rspice_app_types::hierarchy_path::ProbeTarget::engine_alias(node);
    if let Some(engine) = &engine {
        add(engine);
    }
    if !current
        && let Some(target) =
            aliases.and_then(|aliases| aliases.resolve(engine.as_deref().unwrap_or(node)))
    {
        add(target);
    }
    result
}

impl PreparedSavedOutput {
    pub fn source_candidates(&self, signal: &str) -> std::borrow::Cow<'_, [SourceCandidate]> {
        if let Some(prepared) = self.candidates.as_ref().and_then(|map| map.get(signal)) {
            std::borrow::Cow::Borrowed(prepared)
        } else {
            // Old OP/DC evidence and low-level callers know only canonical zero.
            std::borrow::Cow::Owned(candidates(signal, GroundPolicy::OnlyZero, None))
        }
    }
}
