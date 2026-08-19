//! Which quantities a plan effectively saves, occurrence by occurrence.
//!
//! A run's outputs are decided before its deck exists, from the plan's own
//! rows, the markers on the drawing, and — when neither names anything — the
//! design itself. That last case is the reason this module knows about
//! hierarchy: the design is not one sheet. Two instances of one master carry
//! two sets of nodes, the engine names them apart, and an automatic selection
//! that enumerated the master once would ask for one of them and silently
//! answer for whichever the flattened deck resolved first.
//!
//! Each occurrence therefore contributes its own rows, named by its own engine
//! scope, and no occurrence contributes twice. The design root keeps today's
//! spelling exactly: it owns no scope, so `V(OUT)` there is still `V(OUT)`.

use std::collections::HashSet;
use std::sync::Arc;

use crate::simulation::execution::{PreparationError, PreparationStage};
use crate::simulation::netlist_gen::{DesignNet, projection_nets};
use crate::state::{InstancePath, OccurrenceProbeSpelling};

const AUTOMATIC_OUTPUT_SMALL_DESIGN_LIMIT: usize = 16;
const AUTOMATIC_OUTPUT_HARD_LIMIT: usize = 32;

/// One occurrence of the executed design and the nets its master owns.
pub(super) struct OccurrenceNets {
    pub occurrence: InstancePath,
    pub nets: Arc<Vec<DesignNet>>,
}

/// Every occurrence the configured design emits, paired with its master's nets.
///
/// The design root comes first and carries the caller's own summary, because
/// the root is netlisted from the live editor buffer rather than from the
/// projection's copy of it. Below it, each instantiation is its own row: two
/// instances of one master share a drawing and not one node. An occurrence the
/// engine grammar cannot spell is left out, for the same reason a probe on it
/// cannot be requested — there is no name to ask for.
pub(super) fn projection_occurrence_nets(
    libraries: &crate::state::LibraryManager,
    projection: &crate::state::workspace::DesignProjection,
    root_nets: Arc<Vec<DesignNet>>,
) -> Vec<OccurrenceNets> {
    let mut occurrences = vec![OccurrenceNets {
        occurrence: InstancePath::root(),
        nets: root_nets,
    }];
    let Some(plan) = projection.plan() else {
        return occurrences;
    };
    let mut below = plan
        .bindings()
        .filter(|binding| !binding.instance_path().is_root())
        .filter(|binding| {
            matches!(
                binding.resolved_view_type(),
                crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
            ) && binding.instance_path().to_engine_name().is_ok()
        })
        .map(|binding| OccurrenceNets {
            occurrence: binding.instance_path().clone(),
            nets: projection_nets(libraries, projection, &binding.resolved_reference().key()),
        })
        .collect::<Vec<_>>();
    below.sort_by_key(|entry| (entry.occurrence.depth(), entry.occurrence.fold_key()));
    occurrences.append(&mut below);
    occurrences
}

fn prepared_output_expression_key(expression: &str) -> String {
    expression
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

/// One net automatic selection may retain, ranked before the bound applies.
struct AutomaticCandidate {
    depth: usize,
    priority: u8,
    scope: String,
    /// Engine name of the node, which is both the tie-break and the stable
    /// identity: it is the one spelling that separates `/X1`'s `n1` from
    /// `/X2`'s without depending on how either sheet spells the net.
    canonical: String,
    spelling: OccurrenceProbeSpelling,
}

/// Resolve the plan's effective saved-output set without mutating project
/// data. Automatic outputs belong to the prepared snapshot, use stable IDs,
/// and therefore remain deterministic for an unchanged plan and topology.
pub(super) fn effective_plan_saved_outputs(
    selection_mode: crate::state::OutputSelectionMode,
    explicit: &[crate::state::SavedOutput],
    probes: &[crate::state::SchematicProbe],
    occurrences: &[OccurrenceNets],
    plan_id: crate::product::SimulationPlanId,
) -> Result<(Vec<crate::state::SavedOutput>, bool), PreparationError> {
    let mut enabled_probe_outputs =
        std::collections::HashMap::<crate::product::SavedOutputId, HashSet<String>>::new();
    let mut enabled_probe_expressions = std::collections::BTreeMap::new();
    for probe in probes.iter().filter(|probe| probe.enabled) {
        let Some(expression) = probe.source_expression.as_deref().map(str::trim) else {
            continue;
        };
        let expression_key = prepared_output_expression_key(expression);
        if expression_key == "v(0)" {
            continue;
        }
        enabled_probe_expressions
            .entry(expression_key.clone())
            .and_modify(|(_, plot): &mut (String, bool)| *plot |= probe.plot_on_materialization)
            .or_insert_with(|| (expression.to_owned(), probe.plot_on_materialization));
        if probe.plan_id == Some(plan_id)
            && let Some(output_id) = probe.saved_output_id
        {
            enabled_probe_outputs
                .entry(output_id)
                .or_default()
                .insert(expression_key);
        }
    }
    let mut explicit = explicit
        .iter()
        .filter(|output| {
            output.origin != crate::state::SavedOutputOrigin::SchematicProbe
                || enabled_probe_outputs
                    .get(&output.id)
                    .is_some_and(|expressions| {
                        expressions
                            .contains(&prepared_output_expression_key(&output.source_expression))
                    })
        })
        .cloned()
        .map(|mut output| {
            let expression_plot = enabled_probe_expressions
                .get(&prepared_output_expression_key(&output.source_expression))
                .map(|(_, plot)| *plot);
            if let Some(plot) = expression_plot {
                output.display_intent = if plot {
                    crate::state::SavedOutputDisplayIntent::Plot
                } else {
                    crate::state::SavedOutputDisplayIntent::DataBrowserOnly
                };
            }
            output
        })
        .collect::<Vec<_>>();
    let mut present_expressions = explicit
        .iter()
        .map(|output| prepared_output_expression_key(&output.source_expression))
        .collect::<HashSet<_>>();
    for (expression_key, (expression, plot)) in enabled_probe_expressions {
        if !present_expressions.insert(expression_key.clone()) {
            continue;
        }
        let mut output_name = expression.clone();
        if explicit
            .iter()
            .any(|output| output.name.eq_ignore_ascii_case(&output_name))
        {
            let mut ordinal = explicit.len().saturating_add(1);
            loop {
                let candidate = format!("Schematic probe {ordinal}");
                if !explicit
                    .iter()
                    .any(|output| output.name.eq_ignore_ascii_case(&candidate))
                {
                    output_name = candidate;
                    break;
                }
                ordinal = ordinal.saturating_add(1);
            }
        }
        let mut output = crate::state::SavedOutput::new(
            crate::state::SavedOutputKind::RawVoltageOrCurrent,
            output_name,
            expression,
            crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
            crate::state::SavedOutputPolicy::SelectedAndFinalPoints,
            crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
        )
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Schematic probe output is invalid: {error}"),
            )
        })?
        .with_origin(crate::state::SavedOutputOrigin::SchematicProbe)
        .with_display_intent(if plot {
            crate::state::SavedOutputDisplayIntent::Plot
        } else {
            crate::state::SavedOutputDisplayIntent::DataBrowserOnly
        });
        let identity = format!("rspice.schematic-probe/v1/{expression_key}");
        output.id =
            crate::product::SavedOutputId::from_namespace(plan_id.as_uuid(), identity.as_bytes());
        explicit.push(output);
    }
    if selection_mode == crate::state::OutputSelectionMode::SaveAll {
        return Ok((explicit, false));
    }
    if !explicit.is_empty() {
        return Ok((explicit, false));
    }
    if selection_mode == crate::state::OutputSelectionMode::ExplicitOnly {
        return Ok((Vec::new(), false));
    }

    Ok((automatic_outputs(occurrences, plan_id)?, true))
}

/// The bounded node-voltage set a plan with no outputs of its own falls back
/// to, drawn from every occurrence rather than from the root sheet alone.
///
/// Shallower occurrences win the bound, because a design's own interface is
/// what a reader looks at first and a deep instance's internals are what they
/// ask for by name.
fn automatic_outputs(
    occurrences: &[OccurrenceNets],
    plan_id: crate::product::SimulationPlanId,
) -> Result<Vec<crate::state::SavedOutput>, PreparationError> {
    fn carries_signal(net: &&DesignNet) -> bool {
        net.class != crate::simulation::netlist_gen::NetClass::Ground
    }

    let non_ground_count = occurrences
        .iter()
        .map(|entry| entry.nets.iter().filter(carries_signal).count())
        .sum::<usize>();
    let include_unnamed = non_ground_count <= AUTOMATIC_OUTPUT_SMALL_DESIGN_LIMIT;
    let mut candidates = occurrences
        .iter()
        .flat_map(|entry| {
            entry
                .nets
                .iter()
                .filter(carries_signal)
                .filter_map(move |net| {
                    let priority = match net.port {
                        Some(crate::state::PortDirection::Out) => 0,
                        Some(crate::state::PortDirection::InOut) => 1,
                        _ if net.authored_name => 2,
                        _ if include_unnamed => 3,
                        _ => return None,
                    };
                    Some(AutomaticCandidate {
                        depth: entry.occurrence.depth(),
                        priority,
                        scope: entry.occurrence.fold_key(),
                        // The root names a node by its leaf, so a leaf the
                        // engine grammar cannot spell is still the name the
                        // deck emitted there.
                        canonical: entry
                            .occurrence
                            .engine_name_for(&net.name)
                            .unwrap_or_else(|_| net.name.to_ascii_lowercase()),
                        spelling: OccurrenceProbeSpelling::for_leaf(
                            &entry.occurrence,
                            'V',
                            &net.name,
                        )?,
                    })
                })
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        (
            left.depth,
            left.priority,
            left.scope.as_str(),
            left.canonical.as_str(),
        )
            .cmp(&(
                right.depth,
                right.priority,
                right.scope.as_str(),
                right.canonical.as_str(),
            ))
    });
    candidates
        .dedup_by(|left, right| left.scope == right.scope && left.canonical == right.canonical);

    let mut outputs = Vec::with_capacity(candidates.len().min(AUTOMATIC_OUTPUT_HARD_LIMIT));
    for candidate in candidates.into_iter().take(AUTOMATIC_OUTPUT_HARD_LIMIT) {
        let mut output = crate::state::SavedOutput::new(
            crate::state::SavedOutputKind::RawVoltageOrCurrent,
            candidate.spelling.display().to_owned(),
            candidate.spelling.engine().to_owned(),
            crate::state::SavedOutputCompatibility::OpTranAc,
            crate::state::SavedOutputPolicy::SelectedAndFinalPoints,
            crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            crate::state::SavedOutputStreaming::StoreOnly,
        )
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Automatic output {} is invalid: {error}",
                    candidate.spelling.display()
                ),
            )
        })?
        .with_display_intent(if candidate.priority <= 1 {
            crate::state::SavedOutputDisplayIntent::Plot
        } else {
            crate::state::SavedOutputDisplayIntent::DataBrowserOnly
        });
        let identity = format!("rspice.automatic-node-voltage/v1/{}", candidate.canonical);
        output.id =
            crate::product::SavedOutputId::from_namespace(plan_id.as_uuid(), identity.as_bytes());
        outputs.push(output);
    }
    Ok(outputs)
}
