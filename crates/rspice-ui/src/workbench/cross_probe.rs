//! Device-local synchronization for the schematic cross-probe preference.
//!
//! This module observes the authoritative schematic selection once after a
//! complete application frame.  It projects an exact, compatible selection
//! into the generated source map and retained result document without
//! changing project data, adding probes, or navigating away from the user's
//! current workspace.

use std::collections::HashSet;

use crate::product::ContentDigest;
use crate::simulation::netlist_gen::{DesignNet, projection_nets};
use crate::state::{GeneratedArtifact, GeneratedSourceMapEntry};
use crate::workbench::AppState;
use crate::workbench::TogglePreference;
use crate::workbench::documents::result_document::SelectedResultTrace;

/// Stable logical subject selected in the active schematic document.
#[derive(Debug, Clone, PartialEq, Eq)]
enum SchematicCrossProbeTarget {
    Component {
        component_id: u64,
        emitted_instance: String,
    },
    Net {
        name: String,
    },
}

/// Everything that can make a prior projection stale while the visual
/// schematic selection itself remains unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SynchronizationKey {
    active_view_key: String,
    /// The occurrence the active tab is editing. Two tabs on one master reached
    /// through different parents project different traces, so the occurrence is
    /// part of what makes a projection stale.
    occurrence: crate::state::InstancePath,
    topology_version: u64,
    target: Option<SchematicCrossProbeTarget>,
    cross_probe_version: u64,
    result_data_version: u64,
    active_dataset: Option<crate::product::DatasetId>,
    active_analysis_index: Option<usize>,
    active_netlist_document: crate::workbench::documents::netlist_document::ActiveNetlistDocument,
    generated_input_digest: Option<ContentDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectionObservation {
    active_view_key: String,
    topology_version: u64,
    selection: crate::state::Selection,
}

/// Transient authority retained by [`crate::workbench::UiSessionState`].
///
/// The synchronized trace is remembered so disabling the preference or
/// moving to another subject clears only a trace selected by this mechanism;
/// a later manual result selection is never overwritten during cleanup.
#[derive(Debug, Clone, Default)]
pub(crate) struct SchematicCrossProbeRuntime {
    key: Option<SynchronizationKey>,
    selection: Option<SelectionObservation>,
    target: Option<SchematicCrossProbeTarget>,
    selected_trace: Option<SelectedResultTrace>,
    source_line: Option<usize>,
}

/// Reconcile one complete frame of schematic selection changes.
///
/// Callers deliberately invoke this after every visible application window
/// has rendered. That makes canvas, navigator, and inspector gestures share
/// one device-local synchronization boundary and avoids order-dependent
/// behavior among the three surfaces.
pub(crate) fn synchronize_schematic_cross_probe(state: &mut AppState) {
    if !state
        .ui
        .preferences
        .toggle(TogglePreference::CrossProbeBehavior)
    {
        clear_synchronized_projection(state);
        return;
    }

    let selection = SelectionObservation {
        active_view_key: state.workspace.active_view.key(),
        topology_version: state.schematic.topology_version(),
        selection: state.schematic.selection.clone(),
    };
    let target = if state.ui.schematic_cross_probe.selection.as_ref() == Some(&selection) {
        state.ui.schematic_cross_probe.target.clone()
    } else {
        selected_target(state)
    };
    state.ui.schematic_cross_probe.selection = Some(selection);
    state.ui.schematic_cross_probe.target = target.clone();
    let generated_input_digest =
        current_generated_artifact(state).map(|artifact| artifact.provenance().input().digest());
    let key = SynchronizationKey {
        active_view_key: state.workspace.active_view.key(),
        occurrence: state.workspace.occurrence_path(),
        topology_version: state.schematic.topology_version(),
        target: target.clone(),
        cross_probe_version: state.simulation.cross_probe.version,
        result_data_version: state.simulation.data_version,
        active_dataset: state.simulation.active_run().map(|run| run.dataset_id),
        active_analysis_index: state.simulation.active_analysis_idx,
        active_netlist_document: state.ui.netlist.active_document,
        generated_input_digest,
    };
    if state.ui.schematic_cross_probe.key.as_ref() == Some(&key) {
        return;
    }

    clear_synchronized_result_trace(state);
    clear_synchronized_source_line(state);

    let selected_trace = target
        .as_ref()
        .and_then(|target| compatible_result_trace(state, target));
    if let Some(selected) = selected_trace.clone() {
        state.ui.results.selected_trace = Some(selected);
    }

    let source_line = target
        .as_ref()
        .and_then(|target| compatible_generated_source_line(state, target));
    state.ui.netlist.cross_probe_line = source_line;
    if source_line.is_some()
        && state.ui.netlist.active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
    {
        state.ui.netlist.requested_line = source_line;
    }

    state.ui.schematic_cross_probe.key = Some(key);
    state.ui.schematic_cross_probe.selected_trace = selected_trace;
    state.ui.schematic_cross_probe.source_line = source_line;
}

fn clear_synchronized_projection(state: &mut AppState) {
    clear_synchronized_result_trace(state);
    clear_synchronized_source_line(state);
    state.ui.schematic_cross_probe.key = None;
    state.ui.schematic_cross_probe.selection = None;
    state.ui.schematic_cross_probe.target = None;
}

fn clear_synchronized_result_trace(state: &mut AppState) {
    let synchronized = state.ui.schematic_cross_probe.selected_trace.take();
    if synchronized.is_some() && state.ui.results.selected_trace.as_ref() == synchronized.as_ref() {
        state.ui.results.selected_trace = None;
    }
}

fn clear_synchronized_source_line(state: &mut AppState) {
    let synchronized = state.ui.schematic_cross_probe.source_line.take();
    if synchronized.is_some() && state.ui.netlist.requested_line == synchronized {
        state.ui.netlist.requested_line = None;
    }
    state.ui.netlist.cross_probe_line = None;
}

fn selected_target(state: &AppState) -> Option<SchematicCrossProbeTarget> {
    let selection = &state.schematic.selection;
    if state.schematic.net_highlight.selected_net_name.is_some() {
        let nets = active_design_nets(state);
        if let Some(name) = semantic_selected_net_name(state, &nets) {
            return Some(SchematicCrossProbeTarget::Net { name });
        }
    }
    if let Some(component_id) = selection.single_component() {
        let component = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == component_id)?;
        if let Some(port) = component.port_spec() {
            return Some(SchematicCrossProbeTarget::Net { name: port.name });
        }
        return Some(SchematicCrossProbeTarget::Component {
            component_id,
            emitted_instance: emitted_instance_name(component),
        });
    }

    if !conductor_selection(selection) {
        return None;
    }
    let nets = active_design_nets(state);
    selected_net_name(state, &nets).map(|name| SchematicCrossProbeTarget::Net { name })
}

fn semantic_selected_net_name(state: &AppState, nets: &[DesignNet]) -> Option<String> {
    let name = state.schematic.net_highlight.selected_net_name.as_deref()?;
    let net = nets
        .iter()
        .find(|net| net.name.eq_ignore_ascii_case(name))?;
    if state.schematic.net_highlight.highlighted_wires != net.wire_ids.iter().copied().collect() {
        return None;
    }
    let selection = &state.schematic.selection;
    let no_other_classes = selection.wire_segments.is_empty()
        && selection.wire_vertices.is_empty()
        && selection.junctions.is_empty()
        && selection.buses.is_empty()
        && selection.bus_taps.is_empty()
        && selection.net_labels.is_empty()
        && selection.design_notes.is_empty()
        && selection.documentation_shapes.is_empty()
        && selection.probes.is_empty();
    let exact_wires = selection.wires == net.wire_ids.iter().copied().collect();
    let exact_components = if net.wire_ids.is_empty() {
        selection.components
            == net
                .terminals
                .iter()
                .map(|terminal| terminal.component_id)
                .collect()
    } else {
        selection.components.is_empty()
    };
    (no_other_classes && exact_wires && exact_components).then(|| net.name.clone())
}

fn conductor_selection(selection: &crate::state::Selection) -> bool {
    let has_conductor = !selection.wires.is_empty()
        || !selection.wire_segments.is_empty()
        || !selection.wire_vertices.is_empty()
        || !selection.junctions.is_empty()
        || !selection.net_labels.is_empty();
    has_conductor
        && selection.components.is_empty()
        && selection.buses.is_empty()
        && selection.bus_taps.is_empty()
        && selection.design_notes.is_empty()
        && selection.documentation_shapes.is_empty()
        && selection.probes.is_empty()
}

/// The instance name generation would emit for this placement.
///
/// The prefix test is taken over bytes rather than over a string slice.
/// `base` is authored text: an instance named `Ω1` makes `base[..1]` a slice
/// through the middle of a two-byte character, and slicing there panics. The
/// SPICE prefixes are all ASCII, so comparing the leading bytes answers the
/// same question and answers it for every name a reader can type.
fn emitted_instance_name(component: &crate::state::Component) -> String {
    let base = component.spice_instance_name();
    let prefix = component.kind.spice_prefix();
    if prefix.is_empty()
        || base.is_empty()
        || base
            .as_bytes()
            .get(..prefix.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix.as_bytes()))
    {
        base
    } else {
        format!("{prefix}{base}")
    }
}

/// Nets of the open view as the configured design resolves it.
///
/// The synchronizer projects a selection into the generated source map, so it
/// has to agree with what generation emitted. An unresolved configuration
/// therefore synchronizes nothing rather than matching against editor-buffer
/// names the generator would never produce.
fn active_design_nets(state: &AppState) -> std::sync::Arc<Vec<DesignNet>> {
    match state.workspace.design_projection(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    ) {
        Ok(projection) => projection_nets(
            &state.library_manager,
            &projection,
            &state.workspace.active_view.key(),
        ),
        Err(error) => {
            log::warn!("Schematic cross-probe has no design projection: {error}");
            std::sync::Arc::new(Vec::new())
        }
    }
}

fn selected_net_name(state: &AppState, nets: &[DesignNet]) -> Option<String> {
    let selection = &state.schematic.selection;
    if !conductor_selection(selection) {
        return None;
    }

    let mut resolved: Option<String> = None;
    let mut accept = |candidate: &str| -> bool {
        match resolved.as_deref() {
            Some(existing) => existing.eq_ignore_ascii_case(candidate),
            None => {
                resolved = Some(candidate.to_owned());
                true
            }
        }
    };
    for label in &state.schematic.net_labels {
        if selection.net_labels.contains(&label.id) && !accept(&label.name) {
            return None;
        }
    }

    let mut wire_ids = selection.wires.iter().copied().collect::<HashSet<_>>();
    wire_ids.extend(
        selection
            .wire_segments
            .iter()
            .map(|segment| segment.wire_id),
    );
    wire_ids.extend(selection.wire_vertices.iter().map(|vertex| vertex.wire_id));
    for wire_id in wire_ids {
        let Some(net) = nets.iter().find(|net| net.wire_ids.contains(&wire_id)) else {
            continue;
        };
        if !accept(&net.name) {
            return None;
        }
    }
    for junction in &selection.junctions {
        let mut names = state
            .schematic
            .wires
            .iter()
            .filter(|wire| wire.contains_point(junction.pos))
            .filter_map(|wire| {
                nets.iter()
                    .find(|net| net.wire_ids.contains(&wire.id))
                    .map(|net| net.name.as_str())
            });
        if let Some(name) = names.next()
            && (!accept(name) || names.any(|candidate| !accept(candidate)))
        {
            return None;
        }
    }
    resolved
}

fn compatible_result_trace(
    state: &AppState,
    target: &SchematicCrossProbeTarget,
) -> Option<SelectedResultTrace> {
    if !state.simulation.cross_probe.is_current_for(
        &state.workspace.active_view,
        state.schematic.topology_version(),
    ) {
        return None;
    }
    let run = state.simulation.active_run()?;
    let receipt = run.prepared_receipt()?;
    if receipt.project_revision() != state.workspace.project.revision() {
        return None;
    }

    // A trace is named by the node the engine solved, which below the design
    // root is the occurrence's flattened name rather than the local one the
    // drawing shows.
    let occurrence = state.workspace.occurrence_path();
    let signal = match target {
        SchematicCrossProbeTarget::Component {
            emitted_instance, ..
        } => crate::state::OccurrenceProbeSpelling::for_leaf(&occurrence, 'I', emitted_instance)?
            .engine()
            .to_owned(),
        SchematicCrossProbeTarget::Net { name } => {
            let index =
                crate::state::CrossProbeIndex::from_receipt(receipt, &state.simulation.cross_probe);
            let engine = index
                .for_occurrence(&occurrence)
                .first()?
                .engine_name(name)?;
            format!("V({engine})")
        }
    };

    let preferred = state.simulation.active_analysis_idx;
    let find_in_analysis = |analysis_index: usize| {
        let analysis = run.analyses.get(analysis_index)?;
        let (waveform_index, _) = analysis
            .waveforms
            .iter()
            .enumerate()
            .find(|(_, waveform)| waveform.name.eq_ignore_ascii_case(&signal))?;
        SelectedResultTrace::from_run_indices(run, analysis_index, waveform_index)
    };
    preferred
        .and_then(find_in_analysis)
        .or_else(|| (0..run.analyses.len()).find_map(find_in_analysis))
}

fn current_generated_artifact(state: &AppState) -> Option<&GeneratedArtifact> {
    let artifact = state
        .ui
        .netlist
        .generated_document
        .as_ref()?
        .generated_artifact()?;
    let input = artifact.provenance().input();
    let retained = state.ui.netlist.generated_input_digest?;
    if state.ui.netlist.current_generation_input_digest != Some(retained)
        || input.digest() != retained
        || input.revision() != state.workspace.project.revision()
    {
        return None;
    }
    Some(artifact)
}

fn compatible_generated_source_line(
    state: &AppState,
    target: &SchematicCrossProbeTarget,
) -> Option<usize> {
    let artifact = current_generated_artifact(state)?;
    source_line_for_target(artifact, &state.workspace.active_view, target)
}

fn source_line_for_target(
    artifact: &GeneratedArtifact,
    active_view: &crate::state::CellViewRef,
    target: &SchematicCrossProbeTarget,
) -> Option<usize> {
    let cell_identity = format!("{}/{}", active_view.library, active_view.cell);
    let view_identity = active_view.key();
    let line_matches_view = |entry: &GeneratedSourceMapEntry| {
        entry.cell_identity().eq_ignore_ascii_case(&cell_identity)
            && entry.view_identity().eq_ignore_ascii_case(&view_identity)
    };

    match target {
        SchematicCrossProbeTarget::Component {
            component_id,
            emitted_instance,
        } => {
            let identity =
                GeneratedSourceMapEntry::component_identity_for(&view_identity, *component_id);
            artifact
                .generated_lines_for_component(&identity)
                .iter()
                .copied()
                .find(|line| {
                    artifact.source_map_entry(*line).is_some_and(|entry| {
                        line_matches_view(entry)
                            && entry.instance_identity().is_some_and(|candidate| {
                                candidate.eq_ignore_ascii_case(emitted_instance)
                            })
                    })
                })
        }
        SchematicCrossProbeTarget::Net { name } => {
            let source_lines = artifact.source().lines().collect::<Vec<_>>();
            artifact
                .source_map()
                .iter()
                .filter(|entry| line_matches_view(entry))
                .find(|entry| {
                    source_lines
                        .get(entry.generated_line().saturating_sub(1))
                        .is_some_and(|line| source_line_has_net_token(line, name))
                })
                .map(GeneratedSourceMapEntry::generated_line)
        }
    }
}

fn source_line_has_net_token(line: &str, net: &str) -> bool {
    let tokens = line.split_whitespace().collect::<Vec<_>>();
    let head = tokens.first().copied().unwrap_or_default();
    let normalize = |token: &&str| {
        token
            .trim_matches(|ch: char| matches!(ch, '(' | ')' | ',' | ';'))
            .eq_ignore_ascii_case(net)
    };
    if head.eq_ignore_ascii_case(".subckt") {
        return tokens
            .iter()
            .skip(2)
            .take_while(|token| !token.contains('=') && !token.eq_ignore_ascii_case("params:"))
            .any(normalize);
    }
    if head.starts_with('.') || head.starts_with('*') {
        return false;
    }

    // Cards whose terminal count is not fixed by the device letter. A
    // Gummel-Poon `Q` carries three nodes, a substrate `Q` four and a thermal
    // `Q` five; a bulk `M` carries four and an SOI `M` five. RSpice emits all
    // of those. What is fixed is the shape: the last positional token names
    // the model, and everything between the instance name and it is a node.
    let positional_end = || {
        tokens
            .iter()
            .position(|token| token.contains('=') || token.eq_ignore_ascii_case("params:"))
            .unwrap_or(tokens.len())
    };
    let node_count = match head.as_bytes().first().map(u8::to_ascii_uppercase) {
        Some(b'R' | b'C' | b'L' | b'V' | b'I' | b'D' | b'B') => 2,
        Some(b'J') => 3,
        // `Wxxx n+ n- vnam model`: the third positional token names the
        // controlling voltage source, which is an instance and never a net.
        // Counting it as a node made every source name look like a node here.
        Some(b'W') => 2,
        Some(b'E' | b'G' | b'S' | b'T' | b'O') => 4,
        Some(b'F' | b'H') => 2,
        Some(b'Q' | b'M' | b'X') => {
            // The last positional token is the subcircuit/model identity,
            // never an electrical node.
            return tokens
                .get(1..positional_end().saturating_sub(1))
                .is_some_and(|nodes| nodes.iter().any(normalize));
        }
        _ => return false,
    };
    tokens.iter().skip(1).take(node_count).any(normalize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResult, AnalysisResultSourceDomain, AnalysisType, PreparedRunReceipt,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRun, WaveformData,
    };
    use crate::state::{GeneratedProvenance, GenerationInput};

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn prepared_run(revision: ObjectRevision, waveform_name: &str) -> SimulationRun {
        prepared_run_in(revision, waveform_name, Vec::new())
    }

    fn prepared_run_in(
        revision: ObjectRevision,
        waveform_name: &str,
        hierarchy: Vec<crate::state::HierarchyMapRow>,
    ) -> SimulationRun {
        let instance = AnalysisInstanceId::new();
        let receipt = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            revision,
            digest(1),
            digest(2),
            PreparedSourceCheckReceipt::SchematicDrc(digest(3)),
            vec![
                PreparedRunTaskReceipt::new(instance, revision, Vec::new(), 5, digest(4))
                    .expect("valid task"),
            ],
        )
        .expect("valid receipt")
        .with_hierarchy_map(hierarchy)
        .expect("distinct occurrences seal");
        let mut run = SimulationRun::new_prepared(1, receipt);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(waveform_name, vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
            ]),
        );
        run
    }

    fn install_current_map(state: &mut AppState, net: &str) {
        let point = crate::state::Point::new(0, 0);
        state.simulation.cross_probe.update(
            state.workspace.active_view.clone(),
            std::collections::HashMap::from([(point, net.to_owned())]),
            std::collections::HashMap::from([(net.to_owned(), vec![point])]),
            std::collections::HashMap::new(),
            state.schematic.topology_version(),
        );
    }

    #[test]
    fn wireless_semantic_net_beats_its_incident_component_projection() {
        let mut state = AppState::default();
        state.schematic.selection.select_only_component(17);
        state
            .schematic
            .net_highlight
            .highlight_named_wires("PORT_OUT", HashSet::new());
        let nets = vec![DesignNet {
            name: "PORT_OUT".to_owned(),
            authored_name: true,
            class: crate::simulation::netlist_gen::NetClass::Signal,
            terminals: vec![crate::simulation::netlist_gen::NetTerminal {
                component_id: 17,
                reference: "X1".to_owned(),
                pin: "OUT".to_owned(),
            }],
            port: Some(crate::state::PortDirection::Out),
            wire_ids: Vec::new(),
        }];

        assert_eq!(
            semantic_selected_net_name(&state, &nets).as_deref(),
            Some("PORT_OUT")
        );
        state.schematic.net_highlight.clear();
        assert!(semantic_selected_net_name(&state, &nets).is_none());
    }

    #[test]
    fn selected_net_highlights_only_a_current_exact_retained_trace() {
        let mut state = AppState::default();
        state.schematic.net_labels.push(crate::state::NetLabel::new(
            7,
            crate::state::Point::new(0, 0),
            "OUT",
        ));
        state.schematic.selection.select_net_label(7);
        install_current_map(&mut state, "OUT");
        let revision = state.workspace.project.revision();
        state.simulation.runs.push(prepared_run(revision, "V(OUT)"));
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        synchronize_schematic_cross_probe(&mut state);

        assert_eq!(
            state
                .ui
                .results
                .selected_trace
                .as_ref()
                .map(|trace| trace.source_name()),
            Some("V(OUT)")
        );

        state.schematic.bump_topology_version();
        synchronize_schematic_cross_probe(&mut state);
        assert!(state.ui.results.selected_trace.is_none());
    }

    #[test]
    fn a_descended_tab_selects_the_trace_named_for_its_occurrence() {
        let mut state = AppState::default();
        let child = crate::state::CellViewRef::new("user", "amp", "schematic");
        state.workspace.descend_into(
            "X1".to_owned(),
            child.clone(),
            crate::state::ViewType::Schematic,
        );
        state.schematic.net_labels.push(crate::state::NetLabel::new(
            7,
            crate::state::Point::new(0, 0),
            "n1",
        ));
        state.schematic.selection.select_net_label(7);
        install_current_map(&mut state, "n1");
        let revision = state.workspace.project.revision();
        state.simulation.runs.push(prepared_run_in(
            revision,
            "V(x1.n1)",
            vec![
                crate::state::HierarchyMapRow::new("/X1", "amp_1", "X1", child)
                    .expect("one occurrence row"),
            ],
        ));
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        synchronize_schematic_cross_probe(&mut state);

        assert_eq!(
            state
                .ui
                .results
                .selected_trace
                .as_ref()
                .map(|trace| trace.source_name()),
            Some("V(x1.n1)"),
            "a net read inside /X1 must select the node the engine solved there"
        );
    }

    #[test]
    fn disabled_preference_never_projects_the_schematic_selection() {
        let mut state = AppState::default();
        state
            .ui
            .preferences
            .set_toggle(TogglePreference::CrossProbeBehavior, false);
        state.schematic.net_labels.push(crate::state::NetLabel::new(
            7,
            crate::state::Point::new(0, 0),
            "OUT",
        ));
        state.schematic.selection.select_net_label(7);
        install_current_map(&mut state, "OUT");
        let revision = state.workspace.project.revision();
        state.simulation.runs.push(prepared_run(revision, "V(OUT)"));
        state.simulation.active_run_idx = Some(0);

        synchronize_schematic_cross_probe(&mut state);

        assert!(state.ui.results.selected_trace.is_none());
        assert!(state.ui.netlist.cross_probe_line.is_none());
    }

    #[test]
    fn selected_component_uses_its_emitted_device_identity() {
        let mut state = AppState::default();
        let component_id = state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(20, 20),
        );
        let emitted = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == component_id)
            .map(emitted_instance_name)
            .expect("component exists");
        state
            .schematic
            .selection
            .select_only_component(component_id);
        install_current_map(&mut state, "OUT");
        let revision = state.workspace.project.revision();
        let signal = format!("I({emitted})");
        state.simulation.runs.push(prepared_run(revision, &signal));
        state.simulation.active_run_idx = Some(0);

        synchronize_schematic_cross_probe(&mut state);

        assert_eq!(
            state
                .ui
                .results
                .selected_trace
                .as_ref()
                .map(|trace| trace.source_name()),
            Some(signal.as_str())
        );
    }

    #[test]
    fn generated_source_mapping_uses_exact_cell_view_and_component_identity() {
        let view = crate::state::CellViewRef::new("user", "top", "schematic");
        let source = "R1 OUT 0 1k\nR2 OTHER 0 2k\n";
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "test",
                GenerationInput::new(ObjectRevision::INITIAL, digest(9)),
            )
            .expect("valid provenance"),
            source.as_bytes().to_vec(),
            Vec::new(),
            vec![
                GeneratedSourceMapEntry::try_new(
                    1,
                    "user/top",
                    view.key(),
                    Some("R1".to_owned()),
                    Some(format!("{}/component/41", view.key())),
                )
                .expect("valid map"),
                GeneratedSourceMapEntry::try_new(
                    2,
                    "user/top",
                    view.key(),
                    Some("R2".to_owned()),
                    Some(format!("{}/component/42", view.key())),
                )
                .expect("valid map"),
            ],
        )
        .expect("valid artifact");

        assert_eq!(
            source_line_for_target(
                &artifact,
                &view,
                &SchematicCrossProbeTarget::Component {
                    component_id: 42,
                    emitted_instance: "R2".to_owned(),
                },
            ),
            Some(2)
        );
        assert_eq!(
            source_line_for_target(
                &artifact,
                &view,
                &SchematicCrossProbeTarget::Net {
                    name: "OUT".to_owned(),
                },
            ),
            Some(1)
        );
        assert_eq!(
            source_line_for_target(
                &artifact,
                &crate::state::CellViewRef::new("user", "child", "schematic"),
                &SchematicCrossProbeTarget::Net {
                    name: "OUT".to_owned(),
                },
            ),
            None
        );
    }

    #[test]
    fn net_source_mapping_does_not_alias_device_values_or_model_names() {
        assert!(source_line_has_net_token("R1 OUT 0 1k", "OUT"));
        assert!(!source_line_has_net_token("R1 IN 0 OUT", "OUT"));
        assert!(source_line_has_net_token(".subckt amp IN OUT", "OUT"));
        assert!(!source_line_has_net_token("X1 IN 0 OUT", "OUT"));
    }

    /// Cross-probing a substrate or thermal BJT.
    ///
    /// `ComponentType::NpnBjt4` and `PnpBjt5` emit four- and five-node `Q`
    /// cards — `netlist_gen` asserts exactly that — so a net that reaches a
    /// BJT only through its substrate or thermal terminal has to be found on
    /// the card. A fixed three-node count stopped at the emitter, and the
    /// model name that follows the nodes must still not be mistaken for one.
    #[test]
    fn substrate_and_thermal_bjt_terminals_are_cross_probe_nodes() {
        assert!(source_line_has_net_token("Q1 C B E RSPICE_NPN", "E"));
        assert!(source_line_has_net_token("Q1 C B E SUB RSPICE_NPN", "SUB"));
        assert!(source_line_has_net_token(
            "Q2 C B E SUB TJ RSPICE_PNP_THERMAL",
            "TJ"
        ));
        assert!(!source_line_has_net_token(
            "Q1 C B E RSPICE_NPN",
            "RSPICE_NPN"
        ));
        assert!(!source_line_has_net_token(
            "Q2 C B E SUB TJ RSPICE_PNP_THERMAL",
            "RSPICE_PNP_THERMAL"
        ));
    }

    /// The five-node SOI MOSFET reaches its body terminal for the same
    /// reason, and a bulk MOSFET's model name is still not a node.
    #[test]
    fn soi_mosfet_body_terminal_is_a_cross_probe_node() {
        assert!(source_line_has_net_token(
            "M1 D G S B E RSPICE_NMOS_SOI w=1u l=180n",
            "E"
        ));
        assert!(source_line_has_net_token(
            "M1 D G S B RSPICE_NMOS w=1u",
            "B"
        ));
        assert!(!source_line_has_net_token(
            "M1 D G S B RSPICE_NMOS w=1u",
            "RSPICE_NMOS"
        ));
    }

    /// `Wxxx n+ n- vnam model` names a controlling *source*, not a third
    /// node. Counting it as one made a net that shares a name with a source
    /// resolve to the switch card it never touches.
    #[test]
    fn a_current_controlled_switch_has_two_nodes_and_a_controlling_source() {
        assert!(source_line_has_net_token("W1 OUT 0 VSENSE SWMOD", "OUT"));
        assert!(!source_line_has_net_token(
            "W1 OUT 0 VSENSE SWMOD",
            "VSENSE"
        ));
        assert!(!source_line_has_net_token("W1 OUT 0 VSENSE SWMOD", "SWMOD"));
    }

    /// A non-ASCII instance name must not panic the prefix test.
    ///
    /// `spice_prefix` is ASCII, so `base[..prefix.len()]` used to slice one
    /// byte into a two-byte character and abort the frame that resolved the
    /// selection.
    #[test]
    fn a_non_ascii_instance_name_resolves_without_slicing_a_character() {
        let component = crate::state::Component::new(
            1,
            crate::state::ComponentType::Resistor,
            crate::state::Point::origin(),
        )
        .with_name_value("Ω1", "1k");
        assert_eq!(emitted_instance_name(&component), "RΩ1");
    }
}
