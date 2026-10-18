//! App scene composition and project/run provenance for editor-owned painting.

use super::{
    super::symbols::SymbolLibrary,
    SchematicSymbolContext,
    drawing::ProbeVisualStatus,
    drawing_sheet::ActiveDrawingSheet,
    sheet_visibility::{active_sheet_has_objects, active_wire_at, object_is_on_active_sheet},
    viewport::Viewport,
};
use crate::schematic::bus_notations;
use crate::state::{
    CrossProbeIndex, Point, SchematicAnnotationVisibility, SchematicBackAnnotationContent,
    SchematicNetHighlighting, SheetCatalog, Tool,
};
use crate::workbench::app_state::AppState;
use egui::{Painter, Rect};
use rspice_design::connectivity::{extract, summary::projection_nets};
use rspice_schematic_editor::view::{
    cross_probe::wrapped_signal_name,
    pointer_target::{self, PointerTarget},
    scene::{
        self, BadgeSubject, OperatingPointCanvasAnnotation, named_net_class_color,
        normalized_probe_expression,
    },
    tool_input::{ToolPointerAction, ToolPointerView},
};
use std::collections::HashMap;

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
                let components = view
                    .design
                    .visible_objects(&state.schematic.document().components, |item| item.id);
                symbol_context
                    .component_at_resolved_symbol(&components, grid)
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
            operating_point_layout: Some(state.schematic.session.operating_point_badges.layout()),
            probe_status: &probe_status,
            selected_trace_expression: &selected_trace_expression,
            open_pins: &open_pins(painter.ctx(), state, symbol_context),
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
/// topology, symbol revision and sheet.
type OpenPins = std::sync::Arc<std::collections::HashMap<u64, Vec<Point>>>;

/// What an open-pin set was extracted from.
type OpenPinsKey = (u64, u64, Option<crate::state::SheetId>);

/// Each instance's open terminals on the open sheet. Extracted once per
/// topology, symbol revision and sheet, not per frame.
fn open_pins(
    ctx: &egui::Context,
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
) -> OpenPins {
    let key: OpenPinsKey = (
        state.schematic.topology_version(),
        super::symbol_context_revision(state),
        super::sheet_visibility::active_sheet_id(state),
    );
    let id = egui::Id::new("rspice.schematic.open_pins");
    if let Some((cached, pins)) = ctx.data(|data| data.get_temp::<(OpenPinsKey, OpenPins)>(id))
        && cached == key
    {
        return pins;
    }
    let pins = OpenPins::new(extract_open_pins(state, symbol_context));
    ctx.data_mut(|data| data.insert_temp(id, (key, OpenPins::clone(&pins))));
    pins
}

/// The open sheet's terminals that nothing meets, where the canvas draws them.
///
/// Open means `ExtractedTerminal::attached == false`, the judgement the design
/// check reports as an unconnected pin, made by the one [`extract`]. It runs
/// on the open sheet's own objects, in the canvas's coordinates, with each
/// instance's pins resolved through the symbols the frame draws, so a mark
/// sits on the terminal it is about. Seeing one sheet, it does not join what
/// only the whole cell joins: same-named interface ports, and ground symbols,
/// on different sheets.
fn extract_open_pins(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
) -> std::collections::HashMap<u64, Vec<Point>> {
    let sheet = super::sheet_visibility::active_sheet_connectivity(state);
    let connectivity = extract(
        &sheet,
        |binding| symbol_context.resolved_binding_symbol(binding).cloned(),
        // A name decides which net a terminal is on, never whether anything
        // meets it.
        |_| None,
    );
    let mut open = std::collections::HashMap::<u64, Vec<Point>>::new();
    for terminal in connectivity.terminals {
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

/// Every solved node's points on the active sheet, keyed by the engine's
/// folded name for it, read at the active tab's occurrence and at no other.
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
///
/// Built once and looked up per node: asking every mapping's every net for its
/// engine name once per solved node made the join quadratic.
fn occurrence_engine_points<'a>(
    state: &'a AppState,
    index: Option<&'a CrossProbeIndex>,
) -> HashMap<String, &'a Vec<Point>> {
    let occurrence = state.workspace.content.occurrence_path();
    let selected = match index {
        Some(index) => index.for_occurrence(&occurrence),
        None if occurrence.is_root() => std::slice::from_ref(&state.simulation.source.cross_probe),
        None => &[],
    };
    // Only the active sheet's share of the geometry, in its own coordinates:
    // another sheet's points mean nothing on this one.
    let sheet = super::sheet_visibility::active_sheet_id(state);
    let mut points_by_engine = HashMap::new();
    for mapping in selected {
        for (leaf, points) in &mapping.on_sheet(sheet).net_to_points {
            if let Some(engine) = mapping.engine_name(leaf) {
                points_by_engine
                    .entry(engine.to_ascii_lowercase())
                    .or_insert(points);
            }
        }
    }
    points_by_engine
}

/// Where one solved node sits on the drawing; see [`occurrence_engine_points`].
#[cfg(test)]
fn occurrence_net_points<'a>(
    state: &'a AppState,
    index: Option<&'a CrossProbeIndex>,
    engine_name: &str,
) -> Option<&'a Vec<Point>> {
    occurrence_engine_points(state, index)
        .get(&engine_name.to_ascii_lowercase())
        .copied()
}

/// The analysis whose operating point the canvas annotates, with what it
/// retained about annotation, or `None` when the canvas shows no badges.
///
/// Annotations come only from the explicitly selected analysis, and only for
/// the occurrence the active tab is editing. The retained cross-probe point map
/// is replaced for each dispatch, so pairing it with any other result would
/// falsely attach values to a different solve.
fn annotated_operating_point(
    state: &AppState,
) -> Option<(
    &crate::state::DcOpResult,
    Option<crate::state::OperatingPointAnnotationEvidence>,
    Option<&rspice_core::circuit::DeviceOpReport>,
)> {
    if state.ui.schematic_visibility.annotations != SchematicAnnotationVisibility::OperatingPoint
        || !state.simulation.source.cross_probe.is_populated()
        || !state.simulation.source.cross_probe.is_current_for(
            &state.workspace.content.active_view,
            state.schematic.topology_version(),
        )
    {
        return None;
    }
    let analysis = state.simulation.active_analysis()?;
    let dc_op = analysis.dc_op.as_ref()?;
    let retained = match analysis.result_payload.as_ref() {
        Some(crate::state::AnalysisResultPayload::OperatingPoint { annotation, .. }) => {
            Some(*annotation)
        }
        _ => None,
    };
    (retained != Some(crate::state::OperatingPointAnnotationEvidence::None)).then_some((
        dc_op,
        retained,
        analysis.device_op.as_ref(),
    ))
}

/// A fingerprint of everything the voltage badges are read from, hashed by
/// content: the run's geometry and the receipt that partitions it by
/// occurrence, the solved values, the occurrence, the active sheet and its
/// assignments, the highlighted net whose badge belongs to the selection, and
/// every schematic buffer the drawing, its net labels and its bus notations
/// come from. Cheap enough to take every frame; the badges themselves are not.
fn voltage_badge_fingerprint(state: &AppState, dc_op: &crate::state::DcOpResult) -> u64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let map = &state.simulation.source.cross_probe;
    map.version.hash(&mut hasher);
    map.source_topology_version.hash(&mut hasher);
    if let Some(receipt) = state
        .simulation
        .active_run()
        .and_then(crate::state::SimulationRun::prepared_receipt)
    {
        for row in receipt.hierarchy_map() {
            row.occurrence().hash(&mut hasher);
            row.engine_prefix().hash(&mut hasher);
            row.master_reference().key().hash(&mut hasher);
        }
    }
    for voltage in &dc_op.node_voltages {
        voltage.name.hash(&mut hasher);
        voltage.value.to_bits().hash(&mut hasher);
        voltage.unit.hash(&mut hasher);
    }
    state
        .workspace
        .content
        .occurrence_path()
        .fold_key()
        .hash(&mut hasher);
    let catalog = super::sheet_visibility::active_sheet_catalog(state);
    catalog.map(SheetCatalog::revision).hash(&mut hasher);
    catalog
        .and_then(SheetCatalog::active_sheet_id)
        .hash(&mut hasher);
    let highlight = &state.schematic.session.editor.net_highlight;
    highlight.active.hash(&mut hasher);
    highlight
        .selected_net_name
        .as_deref()
        .map(str::to_ascii_lowercase)
        .hash(&mut hasher);
    state.schematic.topology_version().hash(&mut hasher);
    state.schematic.content_version().hash(&mut hasher);
    let mut buffers = 0_u64;
    for (key, buffer) in &state.workspace.content.schematic_buffers {
        let mut entry = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut entry);
        buffer.topology_version().hash(&mut entry);
        buffer.content_version().hash(&mut entry);
        buffers ^= entry.finish();
    }
    buffers.hash(&mut hasher);
    hasher.finish()
}

/// The badge for every solved node the active sheet draws a point of.
///
/// A badge sits on the topmost-leftmost of its net's points that lies on a
/// wire, a junction or a terminal the active sheet draws.
fn voltage_badges(
    state: &AppState,
    dc_op: &crate::state::DcOpResult,
) -> Vec<OperatingPointCanvasAnnotation> {
    let index = occurrence_cross_probe_index(state);
    let points_by_engine = occurrence_engine_points(state, index.as_ref());
    // Junctions and terminals are collected once, not searched per point.
    let catalog = super::sheet_visibility::active_sheet_catalog(state);
    let visible = |id: u64| {
        rspice_schematic_editor::view::design_view::object_is_on_active_sheet(catalog, id)
    };
    let document = state.schematic.document();
    let anchors = document
        .junctions
        .iter()
        .filter(|junction| visible(junction.id))
        .map(|junction| junction.pos)
        .chain(
            document
                .components
                .iter()
                .filter(|component| visible(component.id))
                .flat_map(|component| {
                    component
                        .terminal_positions()
                        .into_iter()
                        .map(|(_, terminal)| terminal)
                }),
        )
        .collect::<std::collections::HashSet<_>>();
    // The first net label on the active sheet that spells each name.
    let mut labels = HashMap::new();
    for label in document.net_labels.iter().filter(|label| visible(label.id)) {
        labels
            .entry(label.name.to_ascii_lowercase())
            .or_insert(label.id);
    }
    // The engine answers for a bus bit under the deck's `DATA#3`; the canvas
    // annotates the conductor the drawing shows, so it quotes the drawing's
    // own spelling of it.
    let notations = bus_notations(&state.workspace, &state.schematic);
    let mut badges = Vec::new();
    for voltage in &dc_op.node_voltages {
        if !voltage.value.is_finite() {
            continue;
        }
        let Some(net_name) = wrapped_signal_name(&voltage.name, 'V') else {
            continue;
        };
        let key = net_name.to_ascii_lowercase();
        let Some(anchor) = points_by_engine.get(&key).and_then(|points| {
            points
                .iter()
                .copied()
                .filter(|point| {
                    anchors.contains(point)
                        || super::sheet_visibility::active_wire_at(state, *point).is_some()
                })
                .min_by_key(|point| (point.y, point.x))
        }) else {
            continue;
        };
        // A named net's value sits right after its own label, so it reads
        // `OUT 912.4 mV`; an unnamed net's badge carries its name.
        let label = labels.get(&key).copied();
        let display = notations.display(&voltage.name);
        let shown = wrapped_signal_name(&display, 'V').unwrap_or(net_name);
        let value = badge_quantity(voltage.value, &voltage.unit);
        let highlighted = editor_net_is_highlighted(state, net_name);
        badges.push(OperatingPointCanvasAnnotation {
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
            key,
            selected_current: false,
        });
    }
    badges
}

/// Read the voltage badges afresh only when something they are read from
/// changed. Runs once a frame, before painting, where the session can hold
/// them.
pub(super) fn refresh_voltage_badges(state: &mut AppState) {
    let Some((dc_op, _, _)) = annotated_operating_point(state) else {
        return;
    };
    let fingerprint = voltage_badge_fingerprint(state, dc_op);
    if state
        .schematic
        .session
        .operating_point_badges
        .current(fingerprint)
        .is_some()
    {
        return;
    }
    let badges = std::sync::Arc::new(voltage_badges(state, dc_op));
    state
        .schematic
        .session
        .operating_point_badges
        .store(fingerprint, badges);
}

/// The operating-point annotations the canvas paints: the voltage badges,
/// reused while their inputs stand still, then the selected devices'
/// currents, device operating points and power.
/// A solved quantity as an operating-point badge reads it: four significant
/// digits, then the SI prefix joined to its unit, with micro spelled `µ`, so
/// a badge reads `912.4 mV` or `21.4 µA`, never `912.430m V`.
pub(super) fn badge_quantity(value: f64, unit: &str) -> String {
    use crate::quantity::engineering::{EngineeringPrecision, format_engineering_display_with};

    // The mantissa decides how many decimals make four digits.
    let fixed = format_engineering_display_with(value, EngineeringPrecision::Fixed(3));
    let mantissa = fixed
        .trim_end_matches(char::is_alphabetic)
        .parse::<f64>()
        .unwrap_or(0.0)
        .abs();
    let decimals = if mantissa > 0.0 {
        (3.0 - mantissa.log10().floor()).clamp(0.0, 3.0) as u8
    } else {
        0
    };
    let text = format_engineering_display_with(value, EngineeringPrecision::UpTo(decimals));
    if unit.is_empty() {
        return text;
    }
    let digits = text.trim_end_matches(char::is_alphabetic);
    let prefix = &text[digits.len()..];
    if prefix.is_empty() {
        format!("{digits} {unit}")
    } else {
        format!("{digits} {prefix}{unit}")
    }
}

pub(super) fn operating_point_annotations(state: &AppState) -> Vec<OperatingPointCanvasAnnotation> {
    let Some((dc_op, retained_annotation, device_op)) = annotated_operating_point(state) else {
        return Vec::new();
    };
    let mut annotations = state
        .schematic
        .session
        .operating_point_badges
        .current(voltage_badge_fingerprint(state, dc_op))
        .map_or_else(|| voltage_badges(state, dc_op), |badges| badges.to_vec());
    let back_annotation = state.ui.schematic_visibility.back_annotation;

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
                "{} = {}",
                current.name,
                badge_quantity(current.value, &current.unit)
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
                "{} = {}",
                power.name,
                badge_quantity(power.value, &power.unit)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisType, CellViewRef, Component, ComponentType, DcOpResult, Junction,
        OperatingPointValue, SimulationRun, ViewType, Wire,
    };
    use crate::workbench::app_state::AppState;
    use std::collections::HashMap;

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
        assert!(
            text(&annotations[0]).starts_with("out 1.25"),
            "{annotations:?}"
        );
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
        assert_eq!(annotations[0].lines, vec!["1.25 V".to_owned()]);
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
        assert_eq!(
            annotations[1].lines.len(),
            2,
            "current and power, one a line"
        );
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

    /// The centres of the open-pin squares one canvas frame paints, in
    /// schematic units: the view is at 100 % with the origin at the canvas's
    /// top-left corner.
    fn open_pin_marks(ctx: &egui::Context, state: &mut AppState) -> Vec<Point> {
        let editor = &mut state.schematic.session.editor;
        (editor.needs_fit, editor.needs_drawing_sheet_fit) = (false, false);
        (editor.pan, editor.zoom) = ((0.0, 0.0), 1.0);
        let mut canvas = Rect::NOTHING;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..Default::default()
            },
            |ui| {
                canvas = ui.available_rect_before_wrap();
                super::super::render_schematic_view(ui, state, None);
            },
        );
        let mut marks = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(square)
                    if square.fill == egui::Color32::TRANSPARENT
                        && square.rect.width() < 10.0
                        && (square.rect.width() - square.rect.height()).abs() < 0.01 =>
                {
                    let centre = square.rect.center() - canvas.min;
                    Some(Point::new(centre.x.round() as i32, centre.y.round() as i32))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        marks.sort_by_key(|point| (point.x, point.y));
        marks
    }

    /// An open pin is marked on the terminal it belongs to, on every sheet.
    /// The marks used to be read off the whole cell's projection, which lays
    /// the sheet at catalog index k a stride along x, so Sheet 2's marks were
    /// drawn a stride away from its pins, off the canvas. Switching sheets
    /// changes no topology, and the marks still follow the sheet.
    #[test]
    fn an_open_pin_on_sheet_two_is_marked_at_its_own_point() {
        let mut state = super::super::sheet_visibility::two_sheet_state(&[6], &[7]);
        state.ui.grid = crate::state::GridStyle::Off;
        let first = Component::new(6, ComponentType::Resistor, Point::new(100, 100));
        let second = Component::new(7, ComponentType::Capacitor, Point::new(300, 200));
        let terminals = |component: &Component| {
            let mut points = component
                .terminal_positions()
                .into_iter()
                .map(|(_, point)| point)
                .collect::<Vec<_>>();
            points.sort_by_key(|point| (point.x, point.y));
            points
        };
        let (first_pins, second_pins) = (terminals(&first), terminals(&second));
        let document = state.schematic.document_mut_for_test();
        document.components.extend([first, second]);
        state.schematic.bump_topology_version();
        state.sync_active_schematic_to_workspace();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);

        assert_eq!(open_pin_marks(&ctx, &mut state), first_pins, "Sheet 1");

        let key = state.workspace.content.active_schematic_reference().key();
        let catalog = state
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("two sheets");
        let sheet_two = catalog.sheets()[1].id();
        catalog.set_active(sheet_two).expect("Sheet 2 opens");
        assert_eq!(
            open_pin_marks(&ctx, &mut state),
            second_pins,
            "Sheet 2's open pins are marked where the canvas draws them"
        );
    }

    /// Each sheet shows its own nets' voltages and no other sheet's, even
    /// where two sheets draw a conductor at the same coordinates.
    #[test]
    fn each_sheet_annotates_only_the_nets_it_draws() {
        use crate::schematic::view::frame_cost::two_sheet_operating_point;

        let badges = |state: &AppState| {
            operating_point_annotations(state)
                .into_iter()
                .filter_map(|annotation| match annotation.subject {
                    BadgeSubject::Net { anchor, .. } => Some((anchor, annotation.key)),
                    BadgeSubject::Device { .. } => None,
                })
                .collect::<Vec<_>>()
        };
        let anchors = |badges: &[(Point, String)], net: &str| {
            badges
                .iter()
                .filter(|(_, key)| key == net)
                .map(|(anchor, _)| *anchor)
                .collect::<Vec<_>>()
        };

        let first = badges(&two_sheet_operating_point(false));
        assert_eq!(anchors(&first, "vout"), vec![Point::new(100, 0)]);
        assert!(anchors(&first, "bias").is_empty(), "{first:?}");

        let second = badges(&two_sheet_operating_point(true));
        assert_eq!(
            anchors(&second, "bias"),
            vec![Point::new(100, 0)],
            "Sheet 2's own net is annotated where Sheet 2 draws it: {second:?}"
        );
        assert!(
            anchors(&second, "vout").is_empty(),
            "Sheet 1's voltage is not painted on Sheet 2's wire: {second:?}"
        );
    }

    /// A steady frame reuses the held badges; a value changed in place is
    /// never painted stale.
    #[test]
    fn voltage_badges_are_held_until_their_values_change() {
        use crate::schematic::view::frame_cost::two_sheet_operating_point;

        let mut state = two_sheet_operating_point(false);
        let held = |state: &AppState| {
            let (dc_op, _, _) = annotated_operating_point(state).expect("badges are shown");
            state
                .schematic
                .session
                .operating_point_badges
                .current(voltage_badge_fingerprint(state, dc_op))
        };
        assert!(held(&state).is_none());
        refresh_voltage_badges(&mut state);
        let first = held(&state).expect("a refresh holds the badges");
        refresh_voltage_badges(&mut state);
        assert!(
            std::sync::Arc::ptr_eq(&first, &held(&state).expect("still held")),
            "a steady frame reads nothing again"
        );

        state.simulation.retained.runs[0].analyses[0]
            .dc_op
            .as_mut()
            .expect("fixture dc op")
            .node_voltages[0]
            .value = 1.75;
        assert!(held(&state).is_none(), "a value replaced in place misses");
        let painted = operating_point_annotations(&state);
        assert!(
            painted
                .iter()
                .any(|badge| badge.key == "vout" && text(badge).contains("1.75 V")),
            "{painted:?}"
        );
        refresh_voltage_badges(&mut state);
        assert!(!std::sync::Arc::ptr_eq(
            &first,
            &held(&state).expect("held again")
        ));
    }

    /// Every input the voltage badges are read from moves their fingerprint
    /// when it changes on its own.
    #[test]
    fn every_badge_input_moves_the_fingerprint() {
        use crate::schematic::view::frame_cost::{
            install_projected_run_geometry, two_sheet_operating_point,
        };

        let fingerprint = |state: &AppState| {
            let dc_op = state
                .simulation
                .active_analysis()
                .and_then(|analysis| analysis.dc_op.as_ref())
                .expect("fixture dc op");
            voltage_badge_fingerprint(state, dc_op)
        };
        type Change = (&'static str, fn(&mut AppState));
        let changes: [Change; 9] = [
            ("solved value", |state| {
                state.simulation.retained.runs[0].analyses[0]
                    .dc_op
                    .as_mut()
                    .expect("dc op")
                    .node_voltages[1]
                    .value = 0.9;
            }),
            ("run geometry", |state| {
                install_projected_run_geometry(
                    state,
                    HashMap::from([("VOUT".to_owned(), vec![Point::new(100, 0)])]),
                );
            }),
            ("topology", |state| state.schematic.bump_topology_version()),
            ("committed content", |state| {
                state.schematic.init_undo_history();
                state.schematic.begin_operation("label");
                state
                    .schematic
                    .document_mut_for_test()
                    .net_labels
                    .push(crate::state::NetLabel::new(9, Point::new(150, 0), "OUT"));
                assert!(state.schematic.end_operation());
            }),
            ("another buffer", |state| {
                state.workspace.insert_schematic_editor(
                    "work/amp/schematic".to_owned(),
                    crate::state::SchematicState::default(),
                );
            }),
            ("active sheet", |state| {
                let key = state.workspace.content.active_schematic_reference().key();
                let catalog = state
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog_mut(&key)
                    .expect("two sheets");
                let second = catalog.sheets()[1].id();
                catalog.set_active(second).expect("sheet 2 opens");
            }),
            ("sheet assignment", |state| {
                let key = state.workspace.content.active_schematic_reference().key();
                let catalog = state
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog_mut(&key)
                    .expect("two sheets");
                let second = catalog.sheets()[1].id();
                catalog
                    .assign_objects(catalog.revision(), second, [1])
                    .expect("wire 1 moves to Sheet 2");
            }),
            ("occurrence", |state| {
                state.workspace.descend_into(
                    "X1".to_owned(),
                    CellViewRef::new("user", "amp", "schematic"),
                    ViewType::Schematic,
                );
            }),
            ("highlighted net", |state| {
                let highlight = &mut state.schematic.session.editor.net_highlight;
                highlight.active = true;
                highlight.selected_net_name = Some("VOUT".to_owned());
            }),
        ];
        for (input, change) in changes {
            let mut state = two_sheet_operating_point(false);
            let before = fingerprint(&state);
            change(&mut state);
            assert_ne!(fingerprint(&state), before, "{input}");
        }
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
        assert!(text(&hot[1]).contains("4 mA"), "{hot:?}");

        state.simulation.view.active_analysis_idx = Some(0);
        let nominal = operating_point_annotations(&state);
        assert!(text(&nominal[0]).contains("1.25"));
        assert!(text(&nominal[1]).contains("2 mA"));
    }
}
