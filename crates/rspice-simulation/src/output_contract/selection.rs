//! Which quantities a plan effectively saves, occurrence by occurrence.
//!
//! A run's outputs are decided before its deck exists, from the plan's own
//! rows, the markers on the drawing, and — for what a run that saves
//! everything plots — the design itself. That last case is the reason this
//! module knows about hierarchy: the design is not one sheet. Two instances of
//! one master carry two sets of nodes, the engine names them apart, and a
//! selection that enumerated the master once would ask for one of them and
//! silently answer for whichever the flattened deck resolved first.
//!
//! Each occurrence therefore contributes its own rows, named by its own engine
//! scope, and no occurrence contributes twice. The design root keeps today's
//! spelling exactly: it owns no scope, so `V(OUT)` there is still `V(OUT)`.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::preparation::{PreparationError, PreparationStage};
use rspice_design::connectivity::summary::{DesignNet, projection_nets};
use rspice_design::library::{LibraryCatalog, ViewType};
use rspice_design::projection::DesignProjection;
use rspice_design::schematic::probe::SchematicProbe;
use rspice_design_model::port::PortDirection;
use rspice_results::saved_output::{
    SavedOutputDisplayIntent, SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming,
};
use rspice_simulation_contract::capture_group::{CaptureGroup, CaptureGroupMembership};
use rspice_simulation_contract::output_policy::OutputSelectionMode;
use rspice_simulation_contract::saved_output::{
    SavedOutput, SavedOutputCompatibility, SavedOutputOrigin,
};
use rspice_vocabulary::hierarchy_path::InstancePath;
use rspice_vocabulary::product::{SavedOutputId, SimulationPlanId};

/// One occurrence of the executed design and the nets its master owns.
pub struct OccurrenceNets {
    occurrence: InstancePath,
    /// The engine's name for the occurrence, `x1.x2`, empty at the root: the
    /// key [`occurrence_at`] finds it by.
    scope: String,
    nets: Arc<Vec<DesignNet>>,
    /// The parts its master draws that carry one current each, as the
    /// drawing names them and as the deck writes them.
    devices: Arc<Vec<OccurrenceDevice>>,
}

/// A part that carries one current, `I(R1)`: the name the drawing gives it
/// and the element the deck writes it as, which can differ (a resistor drawn
/// `C4` is the card `RC4`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccurrenceDevice {
    pub drawn: String,
    pub written: String,
}

/// The parts of the cell view `cell_view_key` that carry one current, read
/// off the projected drawing the deck is written from.
fn current_carrying_devices(
    projection: &DesignProjection,
    cell_view_key: &str,
) -> Arc<Vec<OccurrenceDevice>> {
    Arc::new(
        projection
            .schematic_buffers()
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(cell_view_key))
            .map(|(_, schematic)| {
                schematic
                    .document()
                    .components
                    .iter()
                    .filter(|component| component.carries_one_current())
                    .map(|component| OccurrenceDevice {
                        drawn: component.spice_instance_name(),
                        written: component.engine_element_name(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
    )
}

impl OccurrenceNets {
    /// The occurrence, as drawn: the root, or `/X1/X2`.
    #[must_use]
    pub const fn occurrence(&self) -> &InstancePath {
        &self.occurrence
    }

    /// The nets its master owns, as the master names them.
    #[must_use]
    pub fn nets(&self) -> &[DesignNet] {
        &self.nets
    }

    /// The parts its master draws that carry one current each.
    #[must_use]
    pub fn devices(&self) -> &[OccurrenceDevice] {
        &self.devices
    }
}

/// Every occurrence the configured design emits, paired with its master's nets.
///
/// The design root comes first and carries the caller's own summary, because
/// the root is netlisted from the live editor buffer rather than from the
/// projection's copy of it. Below it, each instantiation is its own row: two
/// instances of one master share a drawing and not one node. An occurrence the
/// engine grammar cannot spell is left out, for the same reason a probe on it
/// cannot be requested — there is no name to ask for.
///
/// The rows below the root are ordered shallowest first and then by engine
/// name, which is the order [`occurrence_at`] and [`occurrences_inside`]
/// search. Each context's nets are looked up once, however many occurrences
/// share it.
pub fn projection_occurrence_nets(
    libraries: &LibraryCatalog,
    projection: &DesignProjection,
    root_nets: Arc<Vec<DesignNet>>,
) -> Vec<OccurrenceNets> {
    let mut occurrences = vec![OccurrenceNets {
        occurrence: InstancePath::root(),
        scope: String::new(),
        nets: root_nets,
        devices: current_carrying_devices(projection, &projection.root().key()),
    }];
    // Every occurrence of one context shares its binding, so the binding's
    // address names the context.
    #[allow(clippy::type_complexity)]
    let mut context_nets =
        HashMap::<*const _, (Arc<Vec<DesignNet>>, Arc<Vec<OccurrenceDevice>>)>::new();
    let mut below = projection
        .plan()
        .bindings_where(|binding| {
            matches!(
                binding.resolved_view_type(),
                ViewType::Schematic | ViewType::Testbench
            )
        })
        .filter(|binding| !binding.instance_path().is_root())
        .filter_map(|binding| {
            let scope = binding.instance_path().to_engine_name().ok()?;
            let (nets, devices) = context_nets
                .entry(std::ptr::from_ref(binding.binding()))
                .or_insert_with(|| {
                    let key = binding.resolved_reference().key();
                    (
                        projection_nets(libraries, projection, &key),
                        current_carrying_devices(projection, &key),
                    )
                });
            Some(OccurrenceNets {
                occurrence: binding.instance_path().clone(),
                scope,
                nets: Arc::clone(nets),
                devices: Arc::clone(devices),
            })
        })
        .collect::<Vec<_>>();
    below.sort_by(|left, right| left.order().cmp(&right.order()));
    occurrences.append(&mut below);
    occurrences
}

impl OccurrenceNets {
    /// Where [`projection_occurrence_nets`] orders it below the root.
    fn order(&self) -> (usize, &str) {
        (self.occurrence.depth(), &self.scope)
    }
}

/// Where among `occurrences`, ordered as [`projection_occurrence_nets`]
/// orders them, the occurrence the engine knows by `scope` is, `/X1/amp` or
/// `x1.xamp` alike. Found by halving, so a design of a million instances
/// answers in twenty comparisons.
pub fn occurrence_at(occurrences: &[OccurrenceNets], scope: &InstancePath) -> Option<usize> {
    let root = occurrences.first()?;
    if scope.is_root() {
        return root.occurrence.is_root().then_some(0);
    }
    let key = scope.to_engine_name().ok()?;
    let wanted = (scope.depth(), key.as_str());
    let below = &occurrences[1..];
    let at = below.partition_point(|entry| entry.order() < wanted);
    below
        .get(at)
        .filter(|entry| entry.order() == wanted)
        .map(|_| at + 1)
}

/// The occurrences directly inside the one at `index` of `occurrences`,
/// ordered as [`projection_occurrence_nets`] orders them: one run of them,
/// found by halving.
pub fn occurrences_inside(occurrences: &[OccurrenceNets], index: usize) -> &[OccurrenceNets] {
    let (Some(parent), Some(below)) = (occurrences.get(index), occurrences.get(1..)) else {
        return &[];
    };
    let depth = parent.occurrence.depth() + 1;
    let prefix = if parent.occurrence.is_root() {
        String::new()
    } else {
        format!("{}.", parent.scope)
    };
    let start = below.partition_point(|entry| entry.order() < (depth, prefix.as_str()));
    let length = below[start..].partition_point(|entry| {
        entry.occurrence.depth() == depth && entry.scope.starts_with(&prefix)
    });
    &below[start..start + length]
}

fn prepared_output_expression_key(expression: &str) -> String {
    expression
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

/// One root net a run that saves everything may plot, ranked.
struct PlottedCandidate {
    priority: u8,
    /// The node's name in one case: the tie-break and the stable identity.
    canonical: String,
    /// The node's name as the sheet spells it.
    name: String,
}

/// The plan's effective output set, projected onto its capture groups.
///
/// Every route that forecasts or executes this plan's outputs comes through
/// here, which is what makes the group overrides one decision rather than two:
/// the page's storage forecast and the contracts a prepared run dispatches are
/// the same projected records. [`effective_plan_saved_outputs`] resolves *which*
/// outputs there are and is deliberately left un-projected so the selection
/// rules can be tested without a group set; production callers want this.
pub fn effective_plan_capture(
    selection_mode: OutputSelectionMode,
    explicit: &[SavedOutput],
    groups: &[CaptureGroup],
    probes: &[SchematicProbe],
    occurrences: &[OccurrenceNets],
    plan_id: SimulationPlanId,
) -> Result<(Vec<SavedOutput>, CaptureGroupMembership), PreparationError> {
    let mut outputs =
        effective_plan_saved_outputs(selection_mode, explicit, probes, occurrences, plan_id)?;
    let membership = crate::capture_ledger::project_onto_groups(groups, &mut outputs);
    Ok((outputs, membership))
}

/// The distinct expressions the enabled schematic probes would have a run save.
///
/// Keyed by [`prepared_output_expression_key`], so two probes on one net are
/// one output and the ground probe is none. The value is the authored spelling
/// and whether any probe on that expression asked to be plotted.
///
/// Read by [`effective_plan_saved_outputs`], which mints the outputs, and by
/// [`enabled_probe_output_count`], which is how the studio's Outputs surfaces
/// learn that a plan with an empty registry still saves something.
fn enabled_probe_expressions(
    probes: &[SchematicProbe],
) -> std::collections::BTreeMap<String, (String, bool)> {
    let mut expressions = std::collections::BTreeMap::new();
    for probe in probes.iter().filter(|probe| probe.enabled) {
        let Some(expression) = probe.source_expression.as_deref().map(str::trim) else {
            continue;
        };
        let expression_key = prepared_output_expression_key(expression);
        if expression_key == "v(0)" {
            continue;
        }
        expressions
            .entry(expression_key)
            .and_modify(|(_, plot): &mut (String, bool)| *plot |= probe.plot_on_materialization)
            .or_insert_with(|| (expression.to_owned(), probe.plot_on_materialization));
    }
    expressions
}

/// How many outputs the enabled schematic probes alone would have a run save.
///
/// A plan whose own registry is empty is not therefore a plan that saves
/// nothing: an enabled probe mints an output here, and once the set is
/// non-empty [`effective_plan_saved_outputs`] returns it in every selection
/// mode — under Save all beside everything the engine computes, under
/// Selected only in place of the whole dataset.
#[must_use]
pub fn enabled_probe_output_count(probes: &[SchematicProbe]) -> usize {
    enabled_probe_expressions(probes).len()
}

/// Resolve the plan's effective saved-output set without mutating project
/// data. The nets Save all plots belong to the prepared snapshot, use stable
/// IDs, and therefore remain deterministic for an unchanged plan and topology.
fn effective_plan_saved_outputs(
    selection_mode: OutputSelectionMode,
    explicit: &[SavedOutput],
    probes: &[SchematicProbe],
    occurrences: &[OccurrenceNets],
    plan_id: SimulationPlanId,
) -> Result<Vec<SavedOutput>, PreparationError> {
    let mut enabled_probe_outputs =
        std::collections::HashMap::<SavedOutputId, HashSet<String>>::new();
    let enabled_probe_expressions = enabled_probe_expressions(probes);
    for probe in probes.iter().filter(|probe| probe.enabled) {
        let Some(expression) = probe.source_expression.as_deref().map(str::trim) else {
            continue;
        };
        let expression_key = prepared_output_expression_key(expression);
        if expression_key == "v(0)" {
            continue;
        }
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
            output.origin != SavedOutputOrigin::SchematicProbe
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
                    SavedOutputDisplayIntent::Plot
                } else {
                    SavedOutputDisplayIntent::DataBrowserOnly
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
        let mut output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            output_name,
            expression,
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
        )
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Schematic probe output is invalid: {error}"),
            )
        })?
        .with_origin(SavedOutputOrigin::SchematicProbe)
        .with_display_intent(if plot {
            SavedOutputDisplayIntent::Plot
        } else {
            SavedOutputDisplayIntent::DataBrowserOnly
        });
        let identity = format!("rspice.schematic-probe/v1/{expression_key}");
        output.id = SavedOutputId::from_namespace(plan_id.as_uuid(), identity.as_bytes());
        explicit.push(output);
    }
    if selection_mode != OutputSelectionMode::SaveAll {
        return Ok(explicit);
    }
    // Save all keeps every quantity the engine computes; what joins the plan's
    // outputs and probes here is only what a run plots, so a first run ends on
    // a curve. A net asked for both ways is the plan's.
    for output in plotted_design_nets(occurrences, plan_id)? {
        let name_taken = explicit
            .iter()
            .any(|asked| asked.name.eq_ignore_ascii_case(&output.name));
        if !name_taken
            && present_expressions.insert(prepared_output_expression_key(&output.source_expression))
        {
            explicit.push(output);
        }
    }
    Ok(explicit)
}

/// The nets a run that saves everything plots: the design root's output
/// ports, with no port the nets its sheet names, with nothing named every
/// node, as ADE plots its outputs on completion. The root's interface is what
/// a reader looks at first; every other node voltage and current the engine
/// computes, at every level, is kept beside them in the data browser.
///
/// Each keeps every point the engine accepted, as Spectre and LTspice keep a
/// saved signal: a transient resampled to its print step reads an edge up to
/// a step late. Each streams while the run goes, so the plot grows as a
/// transient runs unless the plan turns live delivery off.
fn plotted_design_nets(
    occurrences: &[OccurrenceNets],
    plan_id: SimulationPlanId,
) -> Result<Vec<SavedOutput>, PreparationError> {
    let Some(root) = occurrences
        .first()
        .filter(|entry| entry.occurrence.is_root())
    else {
        return Ok(Vec::new());
    };
    let mut candidates = root
        .nets
        .iter()
        .filter(|net| net.class != rspice_design::connectivity::summary::NetClass::Ground)
        .map(|net| PlottedCandidate {
            priority: match net.port {
                Some(PortDirection::Out) => 0,
                Some(PortDirection::InOut) => 1,
                _ if net.authored_name => 2,
                _ => 3,
            },
            canonical: net.name.to_ascii_lowercase(),
            name: net.name.clone(),
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        (left.priority, left.canonical.as_str()).cmp(&(right.priority, right.canonical.as_str()))
    });
    candidates.dedup_by(|left, right| left.canonical == right.canonical);
    // Output and bidirectional ports plot together; below them each rank
    // plots only when nothing outranks it.
    let plotted_rank = candidates
        .iter()
        .map(|candidate| candidate.priority)
        .min()
        .map_or(1, |rank| rank.max(1));
    let mut outputs = Vec::new();
    for candidate in candidates
        .into_iter()
        .filter(|candidate| candidate.priority <= plotted_rank)
    {
        let probe = format!("V({})", candidate.name);
        let mut output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            probe.clone(),
            probe.clone(),
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
        )
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Plotted output {probe} is invalid: {error}"),
            )
        })?
        .with_origin(SavedOutputOrigin::Automatic)
        .with_display_intent(SavedOutputDisplayIntent::Plot);
        let identity = format!("rspice.automatic-node-voltage/v1/{}", candidate.canonical);
        output.id = SavedOutputId::from_namespace(plan_id.as_uuid(), identity.as_bytes());
        outputs.push(output);
    }
    Ok(outputs)
}

#[cfg(test)]
mod tests;
