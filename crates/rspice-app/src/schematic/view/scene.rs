//! App scene composition and project/run provenance for editor-owned painting.

use super::{
    super::symbols::SymbolLibrary,
    SchematicSymbolContext,
    drawing::ProbeVisualStatus,
    drawing_sheet::ActiveDrawingSheet,
    sheet_visibility::{
        active_junction_at, active_sheet_has_objects, active_wire_at, object_is_on_active_sheet,
        objects_on_active_sheet,
    },
    viewport::Viewport,
};
use crate::schematic::bus_notations;
use crate::state::{
    CellViewRef, CrossProbeIndex, Point, SchematicAnnotationVisibility,
    SchematicBackAnnotationContent, SchematicHierarchyVisibility, SchematicNetHighlighting, Tool,
};
use crate::workbench::app_state::AppState;
use egui::{Painter, Rect};
use rspice_design::connectivity::{extract_with_hierarchy, summary::projection_nets};
use rspice_design::hierarchy::HierarchySource;
use rspice_schematic_editor::view::{
    cross_probe::wrapped_signal_name,
    design_view::DesignView,
    pointer_target::{self, PointerTarget},
    scene::{
        self, BadgeSubject, OperatingPointCanvasAnnotation, named_net_class_color,
        normalized_probe_expression,
    },
    tool_input::{ToolPointerAction, ToolPointerView},
};

/// What a click at the hovered point would take, for the hover
/// pre-highlight: in the Select tool the pointer target a click resolves (or
/// the next one a repeat click cycles to), in the Probe tool the wire or
/// instance a probe click lands on, and the chooser's focused row while the
/// overlap chooser is open. Nothing while
/// a button is down (a drag or a band), a popup is open, or the canvas is
/// driven by touch, which has no hover.
fn canvas_hover(
    ctx: &egui::Context,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
) -> Option<PointerTarget> {
    // The chooser's focused row, while it is open; then the object a repeat
    // click would cycle to.
    if let Some(chooser) = super::pick_chooser::open_chooser(ctx) {
        return chooser.focused_target();
    }
    let cycled = super::pick_chooser::cycle_hover(ctx);
    let editor = &state.schematic.session.editor;
    if !matches!(editor.tool, Tool::Select | Tool::Probe)
        || editor.selection_rect.is_active()
        || egui::Popup::is_any_open(ctx)
        || ctx.input(|input| input.pointer.any_down())
        || pointer_target::canvas_touch_mode(ctx)
    {
        return None;
    }
    let position = ctx
        .pointer_hover_pos()
        .filter(|position| available.contains(*position))?;
    let view = ToolPointerView {
        design: super::schematic_design_view(state),
        snap_engine: &editor.snap_engine,
        can_edit: !state.schematic_edit_read_only(),
    };
    match view.action(editor.tool, viewport, symbol_context, position)? {
        ToolPointerAction::Select(_) if cycled.is_some() => cycled,
        ToolPointerAction::Select(query) => pointer_target::pointer_target(
            &view.design,
            query,
            state.ui.schematic_selection_filter,
            symbol_context,
            ctx,
            viewport,
            || state.workspace.content.active_view.display_path(),
        ),
        ToolPointerAction::Probe(grid) => active_wire_at(state, grid)
            .map(PointerTarget::Wire)
            .or_else(|| {
                let components =
                    objects_on_active_sheet(state, &state.schematic.document().components, |item| {
                        item.id
                    });
                symbol_context
                    .component_at_resolved_symbol(components.as_ref(), grid)
                    .map(PointerTarget::Component)
            }),
        _ => None,
    }
}

pub(super) fn draw_scene(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    symbol_library: Option<&SymbolLibrary>,
    symbol_context: &SchematicSymbolContext,
    drawing_sheet: &ActiveDrawingSheet,
) {
    super::drawing_sheet::draw_base(painter, available, viewport, state, drawing_sheet);

    // Context before content: whatever the open sheet is instantiated by is
    // painted under everything the open sheet owns, never over it.
    draw_parent_context(painter, viewport, state);

    // First-run guidance: an empty sheet says what to do next instead of
    // presenting a silent dot field.
    if !active_sheet_has_objects(state) {
        let drawing_area = drawing_sheet
            .geometry
            .drawing_area
            .screen_rect(viewport)
            .intersect(available);
        scene::draw_empty_hint(
            painter,
            if drawing_area.is_positive() {
                drawing_area
            } else {
                available
            },
        );
    }

    let probe_statuses = probe_visual_statuses(state);
    let probe_status =
        |probe: &crate::state::SchematicProbe| probe_visual_status(probe, &probe_statuses);
    let selected_trace_expression = || {
        state
            .ui
            .results
            .valid_selected_trace(&state.simulation)
            .map(|trace| trace.source_name())
    };
    scene::draw_content(
        painter,
        available,
        viewport,
        &scene::SceneView {
            design: super::schematic_design_view(state),
            editor: &state.schematic.session.editor,
            net_highlighting: state.ui.schematic_visibility.net_highlighting,
            parameter_labels: state.ui.schematic_visibility.parameter_labels,
            hover_wire_vertex: state.dialogs.interaction.hover_wire_vertex,
            selection_filter: state.ui.schematic_selection_filter,
            hover: canvas_hover(painter.ctx(), available, viewport, state, symbol_context),
        },
        scene::SceneSymbols {
            library: symbol_library,
            context: symbol_context,
        },
        scene::SceneOverlays {
            net_class_colors: if state.ui.schematic_visibility.net_highlighting
                == SchematicNetHighlighting::NetClassColors
            {
                net_class_colors(state)
            } else {
                std::collections::HashMap::new()
            },
            operating_point: operating_point_annotations(state),
            probe_status: &probe_status,
            selected_trace_expression: &selected_trace_expression,
            open_pins: &open_pins(painter.ctx(), state),
        },
        || state.workspace.content.active_view.display_path(),
    );

    super::drawing_sheet::draw_overflow_advisories(
        painter,
        available,
        viewport,
        state,
        symbol_context,
        drawing_sheet,
    );

    // Check results last — violation badges annotate everything below.
    if state.ui.schematic_visibility.annotations == SchematicAnnotationVisibility::ViolationsOnly {
        super::violations::draw_violation_markers(painter, viewport, state);
    }

    scene::draw_keyboard_focus(
        painter,
        viewport,
        &super::schematic_design_view(state),
        &state.schematic.session.editor.selection,
        state.dialogs.interaction.schematic_keyboard_focus,
        symbol_context,
    );
}

/// The ancestor sheets drawn dimmed beneath the open one, outermost first,
/// each with the buffer key its multi-sheet membership is recorded under.
///
/// `schematic_visibility.hierarchy` is the descend transaction's own record of
/// how much context the author asked for, and this is what honours it: one
/// level is the immediate parent, the full hierarchy is every ancestor on the
/// occurrence, and active-only — which is also what an isolated or read-only
/// open leaves behind — is none. A document opened at its own root has no
/// ancestor to draw under it either way.
fn parent_context_sheets(
    state: &AppState,
) -> Vec<(String, &rspice_design::schematic::owned::Schematic)> {
    let levels = match state.ui.schematic_visibility.hierarchy {
        SchematicHierarchyVisibility::ActiveOnly => return Vec::new(),
        SchematicHierarchyVisibility::ActiveAndParent => 1,
        SchematicHierarchyVisibility::FullVisibleHierarchy => usize::MAX,
    };
    let Some(occurrence) = state.workspace.content.active_occurrence() else {
        return Vec::new();
    };
    let masters: Vec<&CellViewRef> = occurrence.masters().collect();
    // The deepest master is the open document itself; every master before it is
    // an ancestor the occurrence was reached through.
    let ancestors = &masters[..masters.len().saturating_sub(1)];
    ancestors[ancestors.len() - levels.min(ancestors.len())..]
        .iter()
        .filter_map(|master| {
            let key = parent_context_buffer_key(state, master)?;
            let schematic = state.workspace.content.schematic_buffers.get(&key)?;
            Some((key, schematic))
        })
        .collect()
}

/// The spelling `schematic_buffers` holds one master under, which need not be
/// the spelling the occurrence carries.
fn parent_context_buffer_key(state: &AppState, reference: &CellViewRef) -> Option<String> {
    let key = reference.key();
    state
        .workspace
        .content
        .schematic_buffers
        .keys()
        .find(|candidate| candidate.eq_ignore_ascii_case(&key))
        .cloned()
}

fn probe_visual_status(
    probe: &crate::state::SchematicProbe,
    statuses: &ProbeMaterializationStatuses,
) -> ProbeVisualStatus {
    if !probe.enabled {
        return ProbeVisualStatus::Disabled;
    }
    if let Some(status) = probe
        .saved_output_id
        .and_then(|output_id| statuses.by_output_id.get(&output_id).copied())
    {
        return status;
    }
    if let Some(status) = probe.source_expression.as_deref().and_then(|expression| {
        statuses
            .by_expression
            .get(&normalized_probe_expression(expression))
            .copied()
    }) {
        return status;
    }
    if probe.source_expression.is_some() {
        ProbeVisualStatus::Pending
    } else {
        ProbeVisualStatus::Unavailable
    }
}

#[derive(Default)]
struct ProbeMaterializationStatuses {
    by_output_id: std::collections::HashMap<crate::product::SavedOutputId, ProbeVisualStatus>,
    by_expression: std::collections::HashMap<String, ProbeVisualStatus>,
}

fn probe_visual_statuses(state: &AppState) -> ProbeMaterializationStatuses {
    let hidden_waveforms = state
        .simulation
        .view
        .waveforms
        .iter()
        .filter(|waveform| !waveform.visible)
        .map(|waveform| normalized_probe_expression(&waveform.name))
        .collect::<std::collections::HashSet<_>>();
    let mut statuses = ProbeMaterializationStatuses::default();
    let Some(run) = state.simulation.active_run() else {
        return statuses;
    };
    for receipt in run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.saved_output_receipts)
    {
        let status = match &receipt.status {
            crate::state::SavedOutputMaterializationStatus::Materialized {
                waveform_name, ..
            } if hidden_waveforms.contains(&normalized_probe_expression(waveform_name)) => {
                ProbeVisualStatus::Hidden
            }
            crate::state::SavedOutputMaterializationStatus::Materialized { .. } => {
                ProbeVisualStatus::Materialized
            }
            crate::state::SavedOutputMaterializationStatus::MaterializedDcFamily { members } => {
                if members.iter().all(|member| {
                    hidden_waveforms.contains(&normalized_probe_expression(&member.waveform_name))
                }) {
                    ProbeVisualStatus::Hidden
                } else {
                    ProbeVisualStatus::Materialized
                }
            }
            crate::state::SavedOutputMaterializationStatus::Unavailable { .. } => {
                ProbeVisualStatus::Unavailable
            }
            crate::state::SavedOutputMaterializationStatus::Deferred
            | crate::state::SavedOutputMaterializationStatus::SuppressedOnSuccess => {
                ProbeVisualStatus::Pending
            }
        };
        statuses.by_output_id.insert(receipt.output_id, status);
        statuses.by_expression.insert(
            normalized_probe_expression(&receipt.source_expression),
            status,
        );
    }
    statuses
}

/// Conductor colours for the net-class mode, one colour per electrical net.
///
/// The partition is the netlister's, never the canvas's own: two conductor
/// groups one name joins are one node in the deck and must read as one net
/// here, and a group named by an interface port or a ground symbol carries its
/// class without a drawn label. Only authored names are coloured — an
/// autonamed conductor declares no class and stays in the default stroke.
///
/// Nothing is extracted in this paint path. The design projection is memoized
/// on the content it derives from and its per-cell-view net summary is retained
/// on the projection itself, so a frame costs a content digest and a lookup.
/// A version-keyed cache of our own would be wrong as well as redundant: net
/// identity moves with instance values and port parameters, which deliberately
/// do not advance the schematic topology version.
/// The open-pin set: every terminal nothing meets, by instance. One per
/// topology and symbol revision.
type OpenPins = std::sync::Arc<std::collections::HashMap<u64, Vec<Point>>>;

/// Each instance's open terminals: `ExtractedTerminal::attached == false`,
/// the same judgement the design check reports as an unconnected pin, so
/// the mark and the finding cannot disagree. Extracted once per topology
/// and symbol revision, not per frame.
fn open_pins(ctx: &egui::Context, state: &AppState) -> OpenPins {
    let key = (
        state.schematic.topology_version(),
        super::symbol_context_revision(state),
    );
    let id = egui::Id::new("rspice.schematic.open_pins");
    if let Some((cached, pins)) = ctx.data(|data| data.get_temp::<((u64, u64), OpenPins)>(id))
        && cached == key
    {
        return pins;
    }
    let pins = OpenPins::new(extract_open_pins(state));
    ctx.data_mut(|data| data.insert_temp(id, (key, OpenPins::clone(&pins))));
    pins
}

fn extract_open_pins(state: &AppState) -> std::collections::HashMap<u64, Vec<Point>> {
    let mut open = std::collections::HashMap::<u64, Vec<Point>>::new();
    let Ok(projection) = state.workspace.design_projection(
        &state.library_manager,
        &state.workspace.content.active_view,
        &state.schematic,
    ) else {
        return open;
    };
    let key = state.workspace.content.active_view.key();
    let Some((_, schematic)) = projection
        .schematic_buffers()
        .iter()
        .find(|(buffer, _)| buffer.eq_ignore_ascii_case(&key))
    else {
        return open;
    };
    let hierarchy =
        HierarchySource::from_design_projection(state.library_manager.catalog(), &projection);
    for terminal in extract_with_hierarchy(schematic, Some(&hierarchy)).terminals {
        if !terminal.attached {
            open.entry(terminal.component_id)
                .or_default()
                .push(terminal.point);
        }
    }
    open
}

fn net_class_colors(state: &AppState) -> std::collections::HashMap<u64, egui::Color32> {
    let mut colors = std::collections::HashMap::new();
    let Ok(projection) = state.workspace.design_projection(
        &state.library_manager,
        &state.workspace.content.active_view,
        &state.schematic,
    ) else {
        return colors;
    };
    let nets = projection_nets(
        state.library_manager.catalog(),
        &projection,
        &state.workspace.content.active_view.key(),
    );
    for net in nets.iter().filter(|net| net.authored_name) {
        // A name that declares no class is painted as a wire.
        let Some(color) = named_net_class_color(&net.name) else {
            continue;
        };
        for wire_id in &net.wire_ids {
            colors.entry(*wire_id).or_insert(color);
        }
    }
    colors
}

fn device_op_param_unit(name: &str) -> &'static str {
    if rspice_core::op_label::OpLabel::is_intrinsic_voltage_name(name) {
        return "V";
    }
    match name {
        "id" | "ic" | "ib" => "A",
        "vgs" | "vds" | "vbs" | "vth" | "vbe" | "vce" | "vd" => "V",
        "gm" | "gds" | "gmb" | "gd" => "S",
        _ => "",
    }
}

fn device_op_annotation_label(entry: &rspice_core::circuit::DeviceOpEntry) -> String {
    let identity = entry.region.map_or_else(
        || format!("{} [{}]", entry.name, entry.device_kind),
        |region| format!("{} [{} · {region}]", entry.name, entry.device_kind),
    );
    let values = entry
        .params
        .iter()
        .take(3)
        .map(|(name, value)| {
            let unit = device_op_param_unit(name);
            format!(
                "{name}={}{}",
                crate::state::format_engineering(*value),
                unit
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");
    if values.is_empty() {
        identity
    } else {
        format!("{identity} · {values}")
    }
}

/// The run's own statement of which occurrences it emitted, joined onto the
/// retained drawing.
///
/// Only a prepared run carries that statement. Without one there is nothing to
/// join, and [`occurrence_net_points`] falls back to the one reading the
/// retained map already is.
fn occurrence_cross_probe_index(state: &AppState) -> Option<CrossProbeIndex> {
    state
        .simulation
        .active_run()
        .and_then(crate::state::SimulationRun::prepared_receipt)
        .map(|receipt| CrossProbeIndex::from_receipt(receipt, &state.simulation.source.cross_probe))
}

/// Where one solved node sits on the drawing, read at the active tab's
/// occurrence and at no other.
///
/// A net name is only half an address: `n1` inside `/X1` and `n1` inside `/X2`
/// are two different nodes and the engine solved neither of them by that name.
/// Selecting the mappings by the tab's own occurrence is what keeps one
/// occurrence's solved values off another occurrence's drawing — including at
/// the design root, where a mapping read at `/X1` is simply not among them.
///
/// The engine solved a flattened name and the drawing knows a local leaf; the
/// mapping is the only place that flattening is written down, so the match is
/// made through it rather than by stripping a prefix off the solved name.
///
/// Without a prepared receipt the retained map is the reading it was captured
/// as, which generation makes the design root's. A descended tab then has no
/// evidence of what the engine called its nodes, and borrowing the root's names
/// would annotate the wrong occurrence, so it reads nothing.
fn occurrence_net_points<'a>(
    state: &'a AppState,
    index: Option<&'a CrossProbeIndex>,
    engine_name: &str,
) -> Option<&'a Vec<Point>> {
    let occurrence = state.workspace.content.occurrence_path();
    let selected = match index {
        Some(index) => index.for_occurrence(&occurrence),
        None if occurrence.is_root() => std::slice::from_ref(&state.simulation.source.cross_probe),
        None => &[],
    };
    selected.iter().find_map(|mapping| {
        mapping.net_to_points.iter().find_map(|(leaf, points)| {
            mapping
                .engine_name(leaf)
                .is_some_and(|engine: &str| engine.eq_ignore_ascii_case(engine_name))
                .then_some(points)
        })
    })
}

/// Produce annotations only from the explicitly selected analysis, and only for
/// the occurrence the active tab is editing. The retained cross-probe point map
/// is replaced for each dispatch, so pairing it with any other result would
/// falsely attach values to a different solve.
fn operating_point_annotations(state: &AppState) -> Vec<OperatingPointCanvasAnnotation> {
    if state.ui.schematic_visibility.annotations != SchematicAnnotationVisibility::OperatingPoint
        || !state.simulation.source.cross_probe.is_populated()
        || !state.simulation.source.cross_probe.is_current_for(
            &state.workspace.content.active_view,
            state.schematic.topology_version(),
        )
    {
        return Vec::new();
    }
    let Some((dc_op, retained_annotation, device_op)) =
        state.simulation.active_analysis().and_then(|analysis| {
            analysis.dc_op.as_ref().map(|dc_op| {
                let annotation = match analysis.result_payload.as_ref() {
                    Some(crate::state::AnalysisResultPayload::OperatingPoint {
                        annotation,
                        ..
                    }) => Some(*annotation),
                    _ => None,
                };
                (dc_op, annotation, analysis.device_op.as_ref())
            })
        })
    else {
        return Vec::new();
    };
    if retained_annotation == Some(crate::state::OperatingPointAnnotationEvidence::None) {
        return Vec::new();
    }

    let mut annotations = Vec::new();
    let back_annotation = state.ui.schematic_visibility.back_annotation;
    let index = occurrence_cross_probe_index(state);
    // The engine answers for a bus bit under the deck's `DATA#3`; the canvas
    // annotates the conductor the drawing shows, so it quotes the drawing's
    // own spelling of it.
    let notations = bus_notations(&state.workspace, &state.schematic);
    for voltage in &dc_op.node_voltages {
        if !voltage.value.is_finite() {
            continue;
        }
        let Some(net_name) = wrapped_signal_name(&voltage.name, 'V') else {
            continue;
        };
        let points = occurrence_net_points(state, index.as_ref(), net_name);
        let Some(anchor) = points.and_then(|points| {
            points
                .iter()
                .copied()
                .filter(|point| {
                    super::sheet_visibility::active_wire_at(state, *point).is_some()
                        || active_junction_at(state, *point).is_some()
                        || state
                            .schematic
                            .document()
                            .components
                            .iter()
                            .any(|component| {
                                object_is_on_active_sheet(state, component.id)
                                    && component
                                        .terminal_positions()
                                        .iter()
                                        .any(|(_, terminal)| terminal == point)
                            })
                })
                .min_by_key(|point| (point.y, point.x))
        }) else {
            continue;
        };
        // A named net's value sits right after its own label, so it reads
        // `OUT 912.4 mV`; an unnamed net's badge carries its name.
        let label = state
            .schematic
            .document()
            .net_labels
            .iter()
            .find(|label| {
                object_is_on_active_sheet(state, label.id)
                    && label.name.eq_ignore_ascii_case(net_name)
            })
            .map(|label| label.id);
        let display = notations.display(&voltage.name);
        let shown = wrapped_signal_name(&display, 'V').unwrap_or(net_name);
        let value = format!(
            "{} {}",
            crate::state::format_engineering(voltage.value),
            voltage.unit
        );
        let highlighted = editor_net_is_highlighted(state, net_name);
        annotations.push(OperatingPointCanvasAnnotation {
            subject: BadgeSubject::Net { anchor, label },
            lines: vec![if label.is_some() {
                value
            } else {
                format!("{shown} {value}")
            }],
            class: if highlighted {
                0
            } else if label.is_some() {
                1
            } else {
                3
            },
            key: net_name.to_ascii_lowercase(),
            selected_current: false,
        });
    }

    let retained_currents = retained_annotation.is_none_or(|annotation| {
        annotation == crate::state::OperatingPointAnnotationEvidence::VoltagesAndCurrents
    });
    let retained_device_op = retained_annotation
        == Some(crate::state::OperatingPointAnnotationEvidence::VoltagesAndDeviceOp);
    if back_annotation == SchematicBackAnnotationContent::VoltagesOnly {
        return annotations;
    }
    // One block per selected device: its current, its operating point and
    // its power, one quantity a line, placed and kept as one.
    for component in state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| {
            state
                .schematic
                .session
                .editor
                .selection
                .has_component(component.id)
                && object_is_on_active_sheet(state, component.id)
        })
    {
        let mut lines = Vec::new();
        if retained_currents
            && let Some(current) = dc_op.branch_currents.iter().find(|current| {
                current.value.is_finite()
                    && wrapped_signal_name(&current.name, 'I')
                        .is_some_and(|name| name.eq_ignore_ascii_case(&component.name))
            })
        {
            lines.push(format!(
                "{} = {} {}",
                current.name,
                crate::state::format_engineering(current.value),
                current.unit
            ));
        }
        if retained_device_op
            && let Some(entry) = device_op.and_then(|report| {
                report
                    .entries
                    .iter()
                    .find(|entry| entry.name.eq_ignore_ascii_case(&component.name))
            })
        {
            lines.extend(
                device_op_annotation_label(entry)
                    .split(" \u{00b7} ")
                    .map(str::to_owned),
            );
        }
        if back_annotation == SchematicBackAnnotationContent::VoltagesCurrentsAndPower
            && let Some(power) = device_power(dc_op, &component.name)
        {
            lines.push(format!(
                "{} = {} {}",
                power.name,
                crate::state::format_engineering(power.value),
                power.unit
            ));
        }
        if lines.is_empty() {
            continue;
        }
        annotations.push(OperatingPointCanvasAnnotation {
            subject: BadgeSubject::Device {
                component: component.id,
            },
            lines,
            class: 0,
            key: component.name.to_ascii_lowercase(),
            selected_current: true,
        });
    }
    annotations
}

/// Whether the canvas's net highlight is on the net the engine calls
/// `net_name`: its badge then belongs to the selection.
fn editor_net_is_highlighted(state: &AppState, net_name: &str) -> bool {
    let highlight = &state.schematic.session.editor.net_highlight;
    highlight.active
        && highlight
            .selected_net_name
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case(net_name))
}

fn device_power<'a>(
    dc_op: &'a crate::state::DcOpResult,
    component_name: &str,
) -> Option<&'a crate::state::OperatingPointValue> {
    dc_op.power_dissipation.iter().find(|power| {
        power.value.is_finite()
            && wrapped_signal_name(&power.name, 'P')
                .or_else(|| wrapped_signal_name(&power.name, 'W'))
                .is_some_and(|name| name.eq_ignore_ascii_case(component_name))
    })
}

fn draw_parent_context(painter: &Painter, viewport: &Viewport, state: &AppState) {
    scene::draw_parent_context(
        painter,
        viewport,
        parent_context_sheets(state)
            .into_iter()
            .map(|(key, sheet)| DesignView {
                document: sheet.document(),
                canvas_cache: None,
                sheet_catalog: state
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog(&key),
                review_markers: Default::default(),
            }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::SchematicState;
    use crate::state::{
        AnalysisResult, AnalysisType, Component, ComponentType, DcOpResult, Junction,
        OperatingPointValue, SimulationRun, ViewType, Wire,
    };
    use crate::workbench::app_state::AppState;
    use std::collections::HashMap;

    /// The canvas every parent-context render in this module uses.
    const PARENT_CONTEXT_VIEWPORT: egui::Vec2 = egui::vec2(220.0, 140.0);

    /// One conductor on a sheet, so an ancestor has something to contribute.
    fn sheet_with_one_wire() -> SchematicState {
        let mut sheet = SchematicState::default();
        sheet.document_mut_for_test().wires.push(Wire::segment(
            1,
            Point::new(20, 40),
            Point::new(180, 40),
        ));
        sheet
    }

    /// A session descended one level, with the design root carrying a wire.
    fn descended_one_level() -> AppState {
        let mut state = AppState::default();
        let root = state.workspace.content.active_view.clone();
        state
            .workspace
            .insert_schematic_editor(root.key(), sheet_with_one_wire());
        state.workspace.descend_into(
            "X1".to_owned(),
            CellViewRef::new("user", "amp", "schematic"),
            ViewType::Schematic,
        );
        state
    }

    /// Two conductor groups with no wire between them, each carrying the same
    /// authored name. The netlister joins them into one node; a canvas that
    /// traced the drawing itself would see two.
    fn separated_same_name_groups() -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(11, Point::new(0, 0), Point::new(40, 0)));
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(12, Point::new(0, 100), Point::new(40, 100)));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(crate::state::NetLabel::new(21, Point::new(20, 0), "VDD"));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(crate::state::NetLabel::new(22, Point::new(20, 100), "VDD"));
        state.sync_active_schematic_to_workspace();
        state
    }

    /// Net-class colouring paints a conductor by the node the deck emits for
    /// it, so one name across two drawn groups is one colour.
    #[test]
    fn separated_groups_one_name_joins_take_one_net_colour() {
        let state = separated_same_name_groups();
        let colors = net_class_colors(&state);
        assert!(
            colors.contains_key(&11),
            "an authored conductor carries a class colour"
        );
        assert_eq!(
            colors.get(&11),
            colors.get(&12),
            "two groups the deck solves as one node must read as one net"
        );
    }

    /// The differential guard: no electrical net may be painted in more than
    /// one colour, which is what a second connectivity owner on the canvas
    /// would immediately produce.
    #[test]
    fn no_net_the_deck_solves_is_split_across_colours() {
        let state = separated_same_name_groups();
        let colors = net_class_colors(&state);
        let connectivity =
            rspice_design::connectivity::extract_with_hierarchy(&state.schematic, None);
        for net in &connectivity.nets {
            let mut painted = net.wires.iter().map(|id| colors.get(id).copied());
            let first = painted.next();
            assert!(
                painted.all(|color| Some(color) == first),
                "net {} is painted in more than one colour",
                net.spice_name()
            );
        }
    }

    /// A named net whose name declares no class (not ground, supply or
    /// clock) is painted as a wire, not in a colour hashed from its name.
    #[test]
    fn an_unclassified_named_net_takes_no_class_colour() {
        let mut state = separated_same_name_groups();
        for label in &mut state.schematic.document_mut_for_test().net_labels {
            label.name = "VOUT".to_owned();
        }
        state.sync_active_schematic_to_workspace();
        assert!(net_class_colors(&state).is_empty());
    }

    /// An unnamed conductor declares no class, so it keeps the default stroke
    /// rather than being handed an arbitrary palette entry.
    #[test]
    fn an_autonamed_conductor_takes_no_class_colour() {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(31, Point::new(0, 0), Point::new(40, 0)));
        state.sync_active_schematic_to_workspace();
        assert!(net_class_colors(&state).is_empty());
    }

    fn parent_context_canvas(state: &AppState) -> crate::ui::raster::Canvas {
        crate::ui::raster::render(PARENT_CONTEXT_VIEWPORT, |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    let viewport = Viewport {
                        offset: egui::Pos2::ZERO,
                        zoom: 1.0,
                        bounds: Rect::from_min_size(egui::Pos2::ZERO, PARENT_CONTEXT_VIEWPORT),
                    };
                    draw_parent_context(ui.painter(), &viewport, state);
                });
        })
    }

    /// The descend transaction records how much context was asked for, and the
    /// painter honours exactly that: nothing at the design root, nothing when
    /// the descent hid the parent, the immediate parent for one level, and every
    /// ancestor for the full hierarchy.
    #[test]
    fn the_parent_context_painter_draws_only_what_the_descent_asked_for() {
        let root = AppState::default();
        assert!(
            parent_context_sheets(&root).is_empty(),
            "a document opened at its own root has no ancestor"
        );

        let mut state = descended_one_level();
        state.ui.schematic_visibility.hierarchy = SchematicHierarchyVisibility::ActiveOnly;
        assert!(parent_context_sheets(&state).is_empty());

        state.ui.schematic_visibility.hierarchy = SchematicHierarchyVisibility::ActiveAndParent;
        let one_level = parent_context_sheets(&state);
        assert_eq!(one_level.len(), 1);
        assert_eq!(one_level[0].1.document().wires.len(), 1);

        state.workspace.insert_schematic_editor(
            CellViewRef::new("user", "amp", "schematic").key(),
            sheet_with_one_wire(),
        );
        state.workspace.descend_into(
            "X2".to_owned(),
            CellViewRef::new("user", "bias", "schematic"),
            ViewType::Schematic,
        );
        assert_eq!(
            parent_context_sheets(&state).len(),
            1,
            "one level is the immediate parent however deep the occurrence is"
        );

        state.ui.schematic_visibility.hierarchy =
            SchematicHierarchyVisibility::FullVisibleHierarchy;
        assert_eq!(
            parent_context_sheets(&state).len(),
            2,
            "the full hierarchy is every ancestor on the occurrence"
        );
    }

    /// The selection above decides what is drawn, and this is the drawing: a
    /// canvas with an ancestor is not the canvas without one.
    #[test]
    fn the_parent_context_painter_marks_the_canvas_only_beneath_a_descent() {
        let mut state = descended_one_level();
        state.ui.schematic_visibility.hierarchy = SchematicHierarchyVisibility::ActiveAndParent;
        let with_parent = parent_context_canvas(&state);
        let band = Rect::from_min_size(egui::Pos2::ZERO, PARENT_CONTEXT_VIEWPORT);
        assert!(
            with_parent
                .pixels_in(band)
                .any(|pixel| pixel != with_parent.background()),
            "the descended canvas painted no parent context at all"
        );

        state.ui.schematic_visibility.hierarchy = SchematicHierarchyVisibility::ActiveOnly;
        let hidden = parent_context_canvas(&state);
        assert!(
            hidden
                .pixels_in(band)
                .all(|pixel| pixel == hidden.background()),
            "hiding parent context still painted an ancestor"
        );
    }

    #[test]
    fn probe_visual_state_distinguishes_unavailable_pending_and_disabled() {
        let mut probe =
            crate::state::SchematicProbe::new(1, Point::origin(), "P1", None).expect("probe");
        let mut statuses = ProbeMaterializationStatuses::default();
        assert_eq!(
            probe_visual_status(&probe, &statuses),
            ProbeVisualStatus::Unavailable
        );

        probe.reference = "V(out)".to_owned();
        probe.source_expression = Some("V(out)".to_owned());
        probe.bind_saved_output(
            crate::product::SimulationPlanId::new(),
            crate::product::SavedOutputId::new(),
        );
        assert_eq!(
            probe_visual_status(&probe, &statuses),
            ProbeVisualStatus::Pending
        );

        statuses.by_expression.insert(
            normalized_probe_expression("V(out)"),
            ProbeVisualStatus::Materialized,
        );
        assert_eq!(
            probe_visual_status(&probe, &statuses),
            ProbeVisualStatus::Materialized,
            "cross-plan synthesized output receipts resolve by stable electrical identity"
        );

        probe.enabled = false;
        assert_eq!(
            probe_visual_status(&probe, &statuses),
            ProbeVisualStatus::Disabled
        );
    }

    fn state_with_operating_point() -> AppState {
        let mut state = AppState::default();
        let mut component = Component::new(1, ComponentType::VoltageSource, Point::new(40, 30));
        component.name = "VBIAS".to_owned();
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(component);
        state.schematic.session.editor.selection.select_component(1);

        let point = Point::new(20, 10);
        state
            .schematic
            .document_mut_for_test()
            .junctions
            .push(Junction::new(2, point));
        state.simulation.source.cross_probe.update(
            state.workspace.content.active_view.clone(),
            HashMap::from([(point, "OUT".to_owned())]),
            HashMap::from([("OUT".to_owned(), vec![Point::new(30, 10), point])]),
            HashMap::new(),
            state.schematic.topology_version(),
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(DcOpResult {
                node_voltages: vec![OperatingPointValue {
                    name: "V(out)".to_owned(),
                    value: 1.25,
                    unit: "V".to_owned(),
                }],
                branch_currents: vec![OperatingPointValue {
                    name: "I(vbias)".to_owned(),
                    value: 2.0e-3,
                    unit: "A".to_owned(),
                }],
                power_dissipation: Vec::new(),
            }),
        );
        state.simulation.retained.runs.insert(0, run);
        state.simulation.view.active_run_idx = Some(0);
        state.simulation.view.active_analysis_idx = Some(0);
        state
    }

    /// A badge's text, its lines joined.
    fn text(annotation: &OperatingPointCanvasAnnotation) -> String {
        annotation.lines.join(" · ")
    }

    /// An unnamed net's badge carries its name and value at the net's
    /// representative point, in the lowest class; the selected device's
    /// current is a block of its own on that instance, in class 0.
    #[test]
    fn operating_point_policy_maps_latest_values_to_exact_schematic_points() {
        let state = state_with_operating_point();
        let annotations = operating_point_annotations(&state);

        assert_eq!(annotations.len(), 2);
        assert_eq!(
            annotations[0].subject,
            BadgeSubject::Net {
                anchor: Point::new(20, 10),
                label: None
            }
        );
        assert!(text(&annotations[0]).starts_with("out 1.25"), "{annotations:?}");
        assert_eq!(annotations[0].class, 3);
        assert_eq!(
            annotations[1].subject,
            BadgeSubject::Device { component: 1 }
        );
        assert!(annotations[1].selected_current);
        assert_eq!(annotations[1].class, 0);
    }

    /// A net named by a label on the sheet shows its value only, set right
    /// after the label, in the named-net class.
    #[test]
    fn a_named_net_badge_shows_the_value_after_its_label() {
        let mut state = state_with_operating_point();
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(crate::state::NetLabel::new(9, Point::new(20, 10), "OUT"));
        let annotations = operating_point_annotations(&state);
        assert_eq!(
            annotations[0].subject,
            BadgeSubject::Net {
                anchor: Point::new(20, 10),
                label: Some(9)
            }
        );
        assert_eq!(annotations[0].lines, vec!["1.250 V".to_owned()]);
        assert_eq!(annotations[0].class, 1);
    }

    /// The canvas renders a solved name through the same boundary every other
    /// results surface does, and a bus bit reads `V(DATA[3])` there because
    /// the cross-probe map can be asked for the deck's own `#` spelling: the
    /// leaf grammar admits it, so the bit carries an engine name like any
    /// scalar.
    #[test]
    fn a_vector_bit_leaf_carries_an_engine_name_like_any_scalar() {
        let mut state = state_with_operating_point();
        let scalar = Point::new(20, 10);
        let bit = Point::new(20, 30);
        state.simulation.source.cross_probe.update(
            state.workspace.content.active_view.clone(),
            HashMap::from([(scalar, "OUT".to_owned()), (bit, "DATA#3".to_owned())]),
            HashMap::from([
                ("OUT".to_owned(), vec![scalar]),
                ("DATA#3".to_owned(), vec![bit]),
            ]),
            HashMap::new(),
            state.schematic.topology_version(),
        );

        assert_eq!(
            state.simulation.source.cross_probe.engine_name("OUT"),
            Some("out")
        );
        assert_eq!(
            state.simulation.source.cross_probe.engine_name("DATA#3"),
            Some("data#3"),
            "the deck's bus-bit spelling resolves to an engine name"
        );
    }

    #[test]
    fn hidden_or_voltage_only_policy_enforces_annotation_detail() {
        let mut state = state_with_operating_point();
        state.ui.schematic_visibility.back_annotation =
            SchematicBackAnnotationContent::VoltagesOnly;
        assert_eq!(operating_point_annotations(&state).len(), 1);

        state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::Hidden;
        assert!(operating_point_annotations(&state).is_empty());
    }

    #[test]
    fn power_detail_is_rendered_only_when_the_session_requests_it() {
        let mut state = state_with_operating_point();
        state.simulation.retained.runs[0].analyses[0]
            .dc_op
            .as_mut()
            .expect("fixture dc op")
            .power_dissipation
            .push(OperatingPointValue {
                name: "P(vbias)".to_owned(),
                value: 2.5e-3,
                unit: "W".to_owned(),
            });
        state.ui.schematic_visibility.back_annotation =
            SchematicBackAnnotationContent::VoltagesCurrentsAndPower;

        let annotations = operating_point_annotations(&state);

        assert!(text(&annotations[1]).contains("P(vbias)"));
        assert!(text(&annotations[1]).ends_with('W'));
        assert_eq!(annotations[1].lines.len(), 2, "current and power, one a line");
    }

    #[test]
    fn retained_device_op_mode_annotates_only_selected_authoritative_devices() {
        use crate::state::*;

        let mut state = state_with_operating_point();
        let analysis = &mut state.simulation.retained.runs[0].analyses[0];
        analysis.result_payload = Some(AnalysisResultPayload::OperatingPoint {
            temperature_mode: OperatingPointTemperatureEvidence::PvtRunSet,
            temperature_celsius: 27.0,
            initial_guess: OperatingPointInitialGuessEvidence::Automatic,
            node_initialization: OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
            homotopy: OperatingPointHomotopyEvidence::Adaptive,
            annotation: OperatingPointAnnotationEvidence::VoltagesAndDeviceOp,
            device_detail: OperatingPointDeviceDetailEvidence::SelectedAndViolations,
            save_device_op: OperatingPointSaveDeviceEvidence::Enabled,
            accuracy: OperatingPointAccuracyEvidence::Balanced,
            selected_devices: vec!["VBIAS".to_owned()],
            violation_devices: Vec::new(),
            violation_source_content_digest: None,
            validated_startup_directives: 0,
            mna_node_names: vec!["OUT".to_owned()],
            mna_branch_names: vec!["VBIAS".to_owned()],
            mna_solution: vec![1.25, 2.0e-3],
            effective_source_content_digest: None,
            previous_state: None,
            run_point_index: 0,
            run_point_count: 1,
            run_point_process: OperatingPointProcessEvidence::TT,
            run_point_supply_voltage: None,
            run_point_nominal_supply_voltage: None,
        });
        analysis.device_op = Some(rspice_core::circuit::DeviceOpReport {
            entries: vec![rspice_core::circuit::DeviceOpEntry {
                name: "VBIAS".to_owned(),
                device_kind: "SOURCE",
                region: None,
                params: vec![("id", 2.0e-3)],
            }],
        });

        let annotations = operating_point_annotations(&state);
        assert_eq!(annotations.len(), 2, "voltage plus selected device OP");
        assert!(text(&annotations[1]).contains("VBIAS [SOURCE]"));
        assert!(text(&annotations[1]).contains("id=2mA"));
    }

    /// One prepared run that emitted `amp` at `/X1` and `/X2`.
    fn receipt_for_two_occurrences(master: &CellViewRef) -> crate::state::PreparedRunReceipt {
        use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
        use crate::state::{
            AnalysisResultSourceDomain, HierarchyMapRow, PreparedRunReceipt,
            PreparedRunTaskReceipt, PreparedSourceCheckReceipt,
        };

        let digest = |byte: u8| ContentDigest::from_bytes([byte; 32]);
        PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: digest(0x31),
            source_content_digest: digest(0x32),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x33)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(
                    AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    Vec::new(),
                    0,
                    digest(0x34),
                )
                .expect("valid task receipt"),
            ],
        })
        .expect("valid plan receipt")
        .with_hierarchy_map(vec![
            HierarchyMapRow::new("/X1", "amp_1", "X1", master.clone()).expect("first instance"),
            HierarchyMapRow::new("/X2", "amp_1", "X2", master.clone()).expect("second instance"),
        ])
        .expect("distinct occurrences seal")
    }

    /// The paint path is handed the active tab's occurrence and no other's.
    ///
    /// Two instances of one master share every point on the drawing and not one
    /// signal name, so the node the engine solved as `x1.n1` may reach the
    /// drawing only on the tab opened at `/X1` — never at the design root and
    /// never at `/X2`.
    #[test]
    fn the_annotation_path_reads_only_the_active_occurrences_mappings() {
        use crate::state::InstancePath;

        let master = CellViewRef::new("user", "amp", "schematic");
        let point = Point::new(10, 20);
        let mut state = AppState::default();
        state.simulation.source.cross_probe.update(
            master.clone(),
            HashMap::from([(point, "n1".to_owned())]),
            HashMap::from([("n1".to_owned(), vec![point])]),
            HashMap::new(),
            state.schematic.topology_version(),
        );

        let unprepared = occurrence_cross_probe_index(&state);
        assert!(
            unprepared.is_none(),
            "a run with no prepared receipt names no occurrence"
        );
        assert_eq!(
            occurrence_net_points(&state, unprepared.as_ref(), "n1"),
            Some(&vec![point]),
            "the retained map is the design root's own reading"
        );
        assert_eq!(
            occurrence_net_points(&state, unprepared.as_ref(), "x1.n1"),
            None,
            "another occurrence's node must not reach the root drawing"
        );

        let run = SimulationRun::new_prepared(1, receipt_for_two_occurrences(&master));
        state.simulation.retained.runs.insert(0, run);
        state.simulation.view.active_run_idx = Some(0);
        let index = occurrence_cross_probe_index(&state).expect("a prepared run names its map");
        assert_eq!(index.for_occurrence(&InstancePath::root()).len(), 1);
        assert_eq!(
            index
                .for_occurrence(&InstancePath::parse("/X1").expect("first path"))
                .len(),
            1
        );

        assert_eq!(
            occurrence_net_points(&state, Some(&index), "n1"),
            Some(&vec![point]),
            "the root tab reads the root's own spelling"
        );
        assert_eq!(
            occurrence_net_points(&state, Some(&index), "x1.n1"),
            None,
            "an occurrence's node must not reach the tab opened at the root"
        );

        state
            .workspace
            .descend_into("X1".to_owned(), master.clone(), ViewType::Schematic);
        assert_eq!(
            occurrence_net_points(&state, Some(&index), "x1.n1"),
            Some(&vec![point]),
            "the descended tab reads its own occurrence"
        );
        assert_eq!(
            occurrence_net_points(&state, Some(&index), "x2.n1"),
            None,
            "a sibling occurrence of the same master is a different node"
        );
        assert_eq!(
            occurrence_net_points(&state, Some(&index), "n1"),
            None,
            "the root's spelling names no node inside a descended occurrence"
        );

        assert_eq!(
            occurrence_net_points(&state, None, "n1"),
            None,
            "a descended tab with no receipt borrows nothing from the root"
        );
    }

    #[test]
    fn topology_edit_invalidates_retained_operating_point_positions() {
        let mut state = state_with_operating_point();
        assert!(!operating_point_annotations(&state).is_empty());

        state.schematic.bump_topology_version();

        assert!(operating_point_annotations(&state).is_empty());
    }

    #[test]
    fn annotations_follow_only_the_explicitly_selected_operating_point() {
        let mut state = state_with_operating_point();
        state.simulation.retained.runs[0].add_analysis(
            AnalysisResult::new(2, AnalysisType::DcOp, "OP hot").with_dc_op(DcOpResult {
                node_voltages: vec![OperatingPointValue {
                    name: "V(out)".to_owned(),
                    value: 2.5,
                    unit: "V".to_owned(),
                }],
                branch_currents: vec![OperatingPointValue {
                    name: "I(vbias)".to_owned(),
                    value: 4.0e-3,
                    unit: "A".to_owned(),
                }],
                power_dissipation: Vec::new(),
            }),
        );

        state.simulation.view.active_analysis_idx = Some(1);
        let hot = operating_point_annotations(&state);
        assert!(text(&hot[0]).contains("2.5"), "{hot:?}");
        assert!(text(&hot[1]).contains("4m"), "{hot:?}");

        state.simulation.view.active_analysis_idx = Some(0);
        let nominal = operating_point_annotations(&state);
        assert!(text(&nominal[0]).contains("1.25"));
        assert!(text(&nominal[1]).contains("2m"));
    }
}
