//! Device-local synchronization for the schematic cross-probe preference.
//!
//! This module observes the authoritative schematic selection once after a
//! complete application frame.  It projects an exact, compatible selection
//! into the generated source map and retained result document without
//! changing project data, adding probes, or navigating away from the user's
//! current workspace.

use crate::product::ContentDigest;
use crate::state::{GeneratedArtifact, GeneratedSourceMapEntry};
use crate::workbench::TogglePreference;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::NetlistDocumentKey;
use rspice_results_ui::selection::SelectedResultTrace;

/// Stable logical subject selected in the active schematic document.
#[derive(Debug, Clone, PartialEq, Eq)]
enum SchematicCrossProbeTarget {
    Component {
        component_id: u64,
        /// The element the deck writes for it, which is the name its source
        /// line carries.
        written_element: String,
        /// The name the engine knows that element by, which is the name its
        /// trace carries.
        engine_element: String,
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
    shown_netlist_document: NetlistDocumentKey,
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
pub struct SchematicCrossProbeRuntime {
    key: Option<SynchronizationKey>,
    selection: Option<SelectionObservation>,
    target: Option<SchematicCrossProbeTarget>,
    selected_trace: Option<SelectedResultTrace>,
    source_line: Option<usize>,
}

/// What the cross-probe asks of the results and of the generated netlist
/// after a frame: let go of what it selected before, and select what the
/// schematic selection names now. The shell applies it through the results'
/// and the netlist's own fns.
#[derive(Debug)]
pub struct CrossProbeProjection {
    /// The trace the cross-probe selected before, which the results drop
    /// while it is still the one selected.
    pub released_trace: Option<SelectedResultTrace>,
    /// The source line the cross-probe asked for before, which the netlist
    /// withdraws while it is still the one requested.
    pub released_line: Option<usize>,
    /// The trace to select now.
    pub trace: Option<SelectedResultTrace>,
    /// The source line to mark now.
    pub line: Option<usize>,
}

/// Reconcile one complete frame of schematic selection changes.
///
/// Callers deliberately invoke this after every visible application window
/// has rendered. That makes canvas, navigator, and inspector gestures share
/// one device-local synchronization boundary and avoids order-dependent
/// behavior among the three surfaces.
///
/// `shown_netlist_document` is the code feature's key for the netlist
/// document its editor shows; a switch between documents makes the
/// projection stale.
///
/// `None` when nothing that decides the projection changed, so nothing is
/// written.
pub fn synchronize_schematic_cross_probe(
    model: &Model,
    runtime: &mut SchematicCrossProbeRuntime,
    shown_netlist_document: NetlistDocumentKey,
) -> Option<CrossProbeProjection> {
    if !model
        .preferences
        .toggle(TogglePreference::CrossProbeBehavior)
    {
        return Some(release_synchronized_projection(runtime));
    }

    let selection = SelectionObservation {
        active_view_key: model.workspace.content.active_view.key(),
        topology_version: model.schematic.topology_version(),
        selection: model.schematic.session.editor.selection.clone(),
    };
    let target = if runtime.selection.as_ref() == Some(&selection) {
        runtime.target.clone()
    } else {
        selected_target(model)
    };
    runtime.selection = Some(selection);
    runtime.target = target.clone();
    let generated_input_digest =
        current_generated_artifact(model).map(|artifact| artifact.provenance().input().digest());
    let key = SynchronizationKey {
        active_view_key: model.workspace.content.active_view.key(),
        occurrence: model.workspace.content.occurrence_path(),
        topology_version: model.schematic.topology_version(),
        target: target.clone(),
        cross_probe_version: model.simulation.source.cross_probe.version,
        result_data_version: model.simulation.view.data_version,
        active_dataset: model.simulation.active_run().map(|run| run.dataset_id),
        active_analysis_index: model.simulation.view.active_analysis_idx,
        shown_netlist_document,
        generated_input_digest,
    };
    if runtime.key.as_ref() == Some(&key) {
        return None;
    }

    let released_trace = runtime.selected_trace.take();
    let released_line = runtime.source_line.take();
    let selected_trace = target
        .as_ref()
        .and_then(|target| compatible_result_trace(model, target));
    let source_line = target
        .as_ref()
        .and_then(|target| compatible_generated_source_line(model, target));

    runtime.key = Some(key);
    runtime.selected_trace = selected_trace.clone();
    runtime.source_line = source_line;
    Some(CrossProbeProjection {
        released_trace,
        released_line,
        trace: selected_trace,
        line: source_line,
    })
}

/// Let go of everything the cross-probe projected, with the preference off.
fn release_synchronized_projection(
    runtime: &mut SchematicCrossProbeRuntime,
) -> CrossProbeProjection {
    runtime.key = None;
    runtime.selection = None;
    runtime.target = None;
    CrossProbeProjection {
        released_trace: runtime.selected_trace.take(),
        released_line: runtime.source_line.take(),
        trace: None,
        line: None,
    }
}

fn selected_target(model: &Model) -> Option<SchematicCrossProbeTarget> {
    if let Some(name) = model.selected_design_net() {
        return Some(SchematicCrossProbeTarget::Net { name });
    }
    let component_id = model
        .schematic
        .session
        .editor
        .selection
        .single_component()?;
    let component = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)?;
    Some(SchematicCrossProbeTarget::Component {
        component_id,
        written_element: component.written_element_name(),
        engine_element: component.engine_element_name(),
    })
}

fn compatible_result_trace(
    model: &Model,
    target: &SchematicCrossProbeTarget,
) -> Option<SelectedResultTrace> {
    if !model.result_mapping_is_current() {
        return None;
    }
    let run = model.simulation.active_run()?;
    let receipt = run.prepared_receipt()?;
    if receipt.project_revision() != model.workspace.content.project.revision() {
        return None;
    }

    // A trace is named by the node the engine solved, which below the design
    // root is the occurrence's flattened name rather than the local one the
    // drawing shows.
    let occurrence = model.workspace.content.occurrence_path();
    let signal = match target {
        SchematicCrossProbeTarget::Component { engine_element, .. } => {
            crate::state::OccurrenceProbeSpelling::for_leaf(&occurrence, 'I', engine_element)?
                .engine()
                .to_owned()
        }
        SchematicCrossProbeTarget::Net { name } => {
            let index = crate::state::CrossProbeIndex::from_receipt(
                receipt,
                &model.simulation.source.cross_probe,
            );
            let engine = index
                .for_occurrence(&occurrence)
                .first()?
                .engine_name(name)?;
            format!("V({engine})")
        }
    };

    let preferred = model.simulation.view.active_analysis_idx;
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

fn current_generated_artifact(model: &Model) -> Option<&GeneratedArtifact> {
    let artifact = model
        .netlist_artifacts
        .generated_document
        .as_ref()?
        .generated_artifact()?;
    let input = artifact.provenance().input();
    let retained = model.netlist_artifacts.generated_input_digest?;
    if model.netlist_artifacts.current_generation_input_digest != Some(retained)
        || input.digest() != retained
        || input.revision() != model.workspace.content.project.revision()
    {
        return None;
    }
    Some(artifact)
}

fn compatible_generated_source_line(
    model: &Model,
    target: &SchematicCrossProbeTarget,
) -> Option<usize> {
    let artifact = current_generated_artifact(model)?;
    source_line_for_target(artifact, &model.workspace.content.active_view, target)
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
            written_element,
            ..
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
                                candidate.eq_ignore_ascii_case(written_element)
                            })
                    })
                })
        }
        SchematicCrossProbeTarget::Net { name } => artifact
            .source_map()
            .iter()
            .filter(|entry| line_matches_view(entry))
            .find(|entry| {
                entry
                    .nets()
                    .iter()
                    .any(|net| net.eq_ignore_ascii_case(name))
            })
            .map(GeneratedSourceMapEntry::generated_line),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ObjectRevision;
    use crate::state::{GeneratedProvenance, GenerationInput};
    use crate::workbench::design::test_host::DesignTestHost;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    #[test]
    fn wireless_semantic_net_beats_its_incident_component_projection() {
        let mut state = DesignTestHost::default();
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(17);
        state
            .model
            .schematic
            .session
            .editor
            .net_highlight
            .highlight_named_wires("PORT_OUT", std::collections::HashSet::new());
        let nets = vec![rspice_design::connectivity::summary::DesignNet {
            name: "PORT_OUT".to_owned(),
            authored_name: true,
            class: rspice_design::connectivity::summary::NetClass::Signal,
            terminals: vec![rspice_design::connectivity::summary::NetTerminal {
                component_id: 17,
                reference: "X1".to_owned(),
                pin: "OUT".to_owned(),
            }],
            port: Some(crate::state::PortDirection::Out),
            wire_ids: Vec::new(),
            domain: Default::default(),
        }];

        assert_eq!(
            state.model.selected_whole_net(&nets).as_deref(),
            Some("PORT_OUT")
        );
        state.model.schematic.session.editor.net_highlight.clear();
        assert!(state.model.selected_whole_net(&nets).is_none());
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
                .expect("valid map")
                .with_nets(vec!["OUT".to_owned(), "0".to_owned()]),
                GeneratedSourceMapEntry::try_new(
                    2,
                    "user/top",
                    view.key(),
                    Some("R2".to_owned()),
                    Some(format!("{}/component/42", view.key())),
                )
                .expect("valid map")
                .with_nets(vec!["OTHER".to_owned(), "0".to_owned()]),
            ],
        )
        .expect("valid artifact");

        assert_eq!(
            source_line_for_target(
                &artifact,
                &view,
                &SchematicCrossProbeTarget::Component {
                    component_id: 42,
                    written_element: "R2".to_owned(),
                    engine_element: "R2".to_owned(),
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
    fn a_non_ascii_instance_name_resolves_without_slicing_a_character() {
        let component = crate::state::Component::new(
            1,
            crate::state::ComponentType::Resistor,
            crate::state::Point::origin(),
        )
        .with_name_value("Ω1", "1k");
        assert_eq!(component.written_element_name(), "RΩ1");
    }
}
