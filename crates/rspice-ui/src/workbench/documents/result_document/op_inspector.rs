//! Structured DC operating-point result document.
//!
//! The viewer deliberately renders only the explicitly selected analysis. Node
//! voltages, branch currents, device quantities, run-point facts, and retained
//! detail policy all come from immutable result evidence; the UI never fills a
//! missing solver fact with a design-time or fixture value.

use std::collections::BTreeMap;

use egui::Ui;

use crate::state::{
    AnalysisResultPayload, AnalysisType, DcOpResult, OperatingPointAnnotationEvidence,
    OperatingPointDeviceDetailEvidence, OperatingPointProcessEvidence,
    SchematicAnnotationVisibility,
};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;

use super::virtual_rows::RowOffsets;
use super::well_hint;

const NAME_W: f32 = 146.0;
const KIND_W: f32 = 82.0;
const REGION_W: f32 = 92.0;
const VALUE_MIN_W: f32 = 78.0;
const ACTION_W: f32 = 82.0;
const ROW_H: f32 = 25.0;
const HEADER_H: f32 = 23.0;
const GROUP_H: f32 = 24.0;
const DEVICE_GROUP_GAP: f32 = 8.0;
const CELL_INSET: f32 = 9.0;
const TWO_CARD_BREAKPOINT: f32 = 840.0;
const CARD_HEADER_H: f32 = 31.0;

#[derive(Clone)]
struct RetainedDetail {
    policy: OperatingPointDeviceDetailEvidence,
    selected: Vec<String>,
    violations: Vec<String>,
}

#[derive(Clone)]
struct SolveFacts {
    temperature_celsius: f64,
    process: OperatingPointProcessEvidence,
    point_index: u64,
    point_count: u64,
    mna_nodes: usize,
    mna_branches: usize,
    annotation: OperatingPointAnnotationEvidence,
}

#[derive(Clone)]
struct OpEvidence {
    run_id: u64,
    label: String,
    success: bool,
    error: Option<String>,
    dc: Option<DcOpResult>,
    devices: Option<rspice_core::circuit::DeviceOpReport>,
    detail: Option<RetainedDetail>,
    facts: Option<SolveFacts>,
}

#[derive(Clone)]
enum OpAction {
    LocateNode(String),
    LocateDevice(u64),
}

fn selected_op_evidence(state: &AppState) -> Option<OpEvidence> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    if analysis.analysis_type != AnalysisType::DcOp {
        return None;
    }

    let (detail, facts) = match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::OperatingPoint {
            temperature_celsius,
            annotation,
            device_detail,
            selected_devices,
            violation_devices,
            mna_node_names,
            mna_branch_names,
            run_point_index,
            run_point_count,
            run_point_process,
            ..
        }) => (
            Some(RetainedDetail {
                policy: *device_detail,
                selected: selected_devices.clone(),
                violations: violation_devices.clone(),
            }),
            Some(SolveFacts {
                temperature_celsius: *temperature_celsius,
                process: *run_point_process,
                point_index: *run_point_index,
                point_count: *run_point_count,
                mna_nodes: mna_node_names.len(),
                mna_branches: mna_branch_names.len(),
                annotation: *annotation,
            }),
        ),
        _ => (None, None),
    };

    Some(OpEvidence {
        run_id: run.id,
        label: analysis.label.clone(),
        success: analysis.success,
        error: analysis.error_message.clone(),
        dc: analysis.dc_op.clone(),
        devices: analysis.device_op.clone(),
        detail,
        facts,
    })
}

fn retained_detail_allows(entry_name: &str, retained_detail: Option<&RetainedDetail>) -> bool {
    let Some(detail) = retained_detail else {
        // Legacy reports predate explicit detail metadata. Their existing rows
        // remain exact retained evidence, so hiding them would discard data.
        return true;
    };
    match detail.policy {
        OperatingPointDeviceDetailEvidence::AllDevices => true,
        OperatingPointDeviceDetailEvidence::SelectedAndViolations => {
            (detail.selected.is_empty() && detail.violations.is_empty())
                || contains_identity(&detail.selected, entry_name)
                || contains_identity(&detail.violations, entry_name)
        }
        OperatingPointDeviceDetailEvidence::ViolationsOnly => {
            contains_identity(&detail.violations, entry_name)
        }
        OperatingPointDeviceDetailEvidence::None => false,
    }
}

fn contains_identity(identities: &[String], candidate: &str) -> bool {
    identities
        .iter()
        .any(|identity| identity.eq_ignore_ascii_case(candidate))
}

fn process_label(process: OperatingPointProcessEvidence) -> &'static str {
    match process {
        OperatingPointProcessEvidence::TT => "TT",
        OperatingPointProcessEvidence::SS => "SS",
        OperatingPointProcessEvidence::FF => "FF",
        OperatingPointProcessEvidence::SF => "SF",
        OperatingPointProcessEvidence::FS => "FS",
    }
}

fn detail_label(detail: Option<&RetainedDetail>) -> &'static str {
    match detail.map(|detail| detail.policy) {
        None => "legacy retained report",
        Some(OperatingPointDeviceDetailEvidence::AllDevices) => "all devices retained",
        Some(OperatingPointDeviceDetailEvidence::SelectedAndViolations) => {
            "selected devices and violations retained"
        }
        Some(OperatingPointDeviceDetailEvidence::ViolationsOnly) => "violations retained",
        Some(OperatingPointDeviceDetailEvidence::None) => "device detail not retained",
    }
}

fn annotation_label(annotation: OperatingPointAnnotationEvidence) -> &'static str {
    match annotation {
        OperatingPointAnnotationEvidence::VoltagesAndCurrents => "voltages and currents",
        OperatingPointAnnotationEvidence::VoltagesOnly => "voltages only",
        OperatingPointAnnotationEvidence::VoltagesAndDeviceOp => "voltages and device OP",
        OperatingPointAnnotationEvidence::None => "not retained",
    }
}

fn signal_leaf(name: &str) -> &str {
    let name = name.trim();
    if name.len() >= 3
        && name.as_bytes().get(1) == Some(&b'(')
        && name.ends_with(')')
        && matches!(name.as_bytes()[0].to_ascii_uppercase(), b'V' | b'I' | b'P')
    {
        name[2..name.len() - 1].trim()
    } else {
        name
    }
}

fn hierarchy_parts(name: &str) -> (&str, &str) {
    let name = signal_leaf(name);
    name.rmatch_indices(['/', '.', ':'])
        .next()
        .map_or(("Top", name), |(index, separator)| {
            let leaf = &name[index + separator.len()..];
            let scope = &name[..index];
            if scope.is_empty() || leaf.is_empty() {
                ("Top", name)
            } else {
                (scope, leaf)
            }
        })
}

fn node_matches(row: &crate::state::OperatingPointValue, filter: &str) -> bool {
    if filter.trim().is_empty() {
        return true;
    }
    let filter = filter.trim().to_ascii_lowercase();
    let (scope, leaf) = hierarchy_parts(&row.name);
    row.name.to_ascii_lowercase().contains(&filter)
        || scope.to_ascii_lowercase().contains(&filter)
        || leaf.to_ascii_lowercase().contains(&filter)
        || row.unit.to_ascii_lowercase().contains(&filter)
}

fn device_matches(entry: &rspice_core::circuit::DeviceOpEntry, filter: &str) -> bool {
    if filter.trim().is_empty() {
        return true;
    }
    let filter = filter.trim().to_ascii_lowercase();
    let (scope, leaf) = hierarchy_parts(&entry.name);
    entry.name.to_ascii_lowercase().contains(&filter)
        || scope.to_ascii_lowercase().contains(&filter)
        || leaf.to_ascii_lowercase().contains(&filter)
        || entry.device_kind.to_ascii_lowercase().contains(&filter)
        || entry
            .region
            .is_some_and(|region| region.to_ascii_lowercase().contains(&filter))
        || entry
            .params
            .iter()
            .any(|(name, _)| name.to_ascii_lowercase().contains(&filter))
}

fn grouped_nodes<'a>(
    dc: &'a DcOpResult,
    filter: &str,
) -> BTreeMap<String, Vec<&'a crate::state::OperatingPointValue>> {
    let mut groups = BTreeMap::<String, Vec<_>>::new();
    for row in dc
        .node_voltages
        .iter()
        .filter(|row| node_matches(row, filter))
    {
        groups
            .entry(hierarchy_parts(&row.name).0.to_owned())
            .or_default()
            .push(row);
    }
    for rows in groups.values_mut() {
        rows.sort_by(|left, right| {
            signal_leaf(&left.name)
                .to_ascii_lowercase()
                .cmp(&signal_leaf(&right.name).to_ascii_lowercase())
        });
    }
    groups
}

fn grouped_devices<'a>(
    report: &'a rspice_core::circuit::DeviceOpReport,
    detail: Option<&RetainedDetail>,
    filter: &str,
    sort: Option<&(String, bool)>,
) -> BTreeMap<String, Vec<&'a rspice_core::circuit::DeviceOpEntry>> {
    let mut groups = BTreeMap::<String, Vec<_>>::new();
    for entry in report
        .entries
        .iter()
        .filter(|entry| retained_detail_allows(&entry.name, detail))
        .filter(|entry| device_matches(entry, filter))
    {
        groups
            .entry(hierarchy_parts(&entry.name).0.to_owned())
            .or_default()
            .push(entry);
    }
    for rows in groups.values_mut() {
        rows.sort_by(|left, right| match sort {
            Some((key, ascending)) => {
                let left_value = device_sort_value(left, key);
                let right_value = device_sort_value(right, key);
                let value_order = match (left_value, right_value) {
                    (Some(left), Some(right)) => {
                        let order = left
                            .abs()
                            .total_cmp(&right.abs())
                            .then_with(|| left.total_cmp(&right));
                        if *ascending { order } else { order.reverse() }
                    }
                    // Missing and non-finite quantities stay below retained numeric
                    // evidence in both directions.
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                };
                value_order.then_with(|| compare_device_names(left, right))
            }
            None => compare_device_names(left, right),
        });
    }
    groups
}

fn compare_device_names(
    left: &rspice_core::circuit::DeviceOpEntry,
    right: &rspice_core::circuit::DeviceOpEntry,
) -> std::cmp::Ordering {
    signal_leaf(&left.name)
        .to_ascii_lowercase()
        .cmp(&signal_leaf(&right.name).to_ascii_lowercase())
}

fn device_sort_value(entry: &rspice_core::circuit::DeviceOpEntry, key: &str) -> Option<f64> {
    entry
        .params
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
        .map(|(_, value)| *value)
        .filter(|value| value.is_finite())
}

fn device_columns(
    rows: &[&rspice_core::circuit::DeviceOpEntry],
) -> Vec<(&'static str, &'static str)> {
    let mut columns = Vec::new();
    for entry in rows {
        for (name, _) in &entry.params {
            if !columns.iter().any(|(candidate, _)| candidate == name) {
                columns.push((*name, device_param_unit(entry.device_kind, name)));
            }
        }
    }
    columns
}

fn device_param_unit(family: &str, name: &str) -> &'static str {
    match (family, name) {
        ("MOSFET", "id") | ("BJT", "ic" | "ib") | ("DIODE", "id") => "A",
        ("MOSFET", "vgs" | "vds" | "vbs" | "vth") | ("BJT", "vbe" | "vce") | ("DIODE", "vd") => "V",
        ("MOSFET", "gm" | "gds" | "gmb") | ("BJT", "gm") | ("DIODE", "gd") => "S",
        _ => "",
    }
}

fn op_column_rect(row: egui::Rect, offset: f32, width: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(row.left() + offset, row.top()),
        egui::vec2(width, row.height()),
    )
}

fn paint_cell(
    ui: &Ui,
    cell: egui::Rect,
    text: impl ToString,
    align: egui::Align2,
    font: egui::FontId,
    color: egui::Color32,
) {
    let x = if align == egui::Align2::RIGHT_CENTER {
        cell.right() - CELL_INSET
    } else {
        cell.left() + CELL_INSET
    };
    ui.painter()
        .with_clip_rect(cell.shrink2(egui::vec2(2.0, 0.0)))
        .text(egui::pos2(x, cell.center().y), align, text, font, color);
}

fn group_header(ui: &mut Ui, width: f32, scope: &str, count: usize) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, GROUP_H), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            true,
            format!("Hierarchy {scope}, {count} rows"),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::RowHeader);
        node.set_label(format!("Hierarchy {scope}, {count} rows"));
    });
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    paint_cell(
        ui,
        op_column_rect(rect, 0.0, width),
        format!("{scope}  ·  {count}"),
        egui::Align2::LEFT_CENTER,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
}

fn column_header(ui: &mut Ui, width: f32, columns: &[(f32, &str, bool)]) -> egui::Rect {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, HEADER_H), egui::Sense::hover());
    let header_label = columns
        .iter()
        .filter_map(|(_, label, _)| (!label.is_empty()).then_some(*label))
        .collect::<Vec<_>>()
        .join(", ");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            true,
            format!("Table columns: {header_label}"),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(format!("Column headers: {header_label}"));
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    let mut offset = 0.0;
    for (column_width, label, numeric) in columns {
        paint_cell(
            ui,
            op_column_rect(rect, offset, *column_width),
            *label,
            if *numeric {
                egui::Align2::RIGHT_CENTER
            } else {
                egui::Align2::LEFT_CENTER
            },
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
        offset += *column_width;
    }
    rect
}

fn device_column_header(
    ui: &mut Ui,
    width: f32,
    columns: &[(&'static str, &'static str)],
    scope: &str,
    sort: Option<&(String, bool)>,
) -> Option<String> {
    let mut headers = vec![
        (NAME_W, "INSTANCE", false),
        (KIND_W, "FAMILY", false),
        (REGION_W, "REGION", false),
    ];
    headers.extend(columns.iter().map(|(name, _)| (VALUE_MIN_W, *name, true)));
    headers.push((ACTION_W, "", false));
    let rect = column_header(ui, width, &headers);
    let t = Tokens::get(ui.ctx());
    let mut clicked = None;
    for (index, (key, _)) in columns.iter().enumerate() {
        let cell = op_column_rect(
            rect,
            NAME_W + KIND_W + REGION_W + index as f32 * VALUE_MIN_W,
            VALUE_MIN_W,
        );
        let selected = sort.is_some_and(|(current, _)| current.eq_ignore_ascii_case(key));
        let ascending = selected && sort.is_some_and(|(_, ascending)| *ascending);
        let response = ui.interact(
            cell,
            ui.id().with(("op-sort", scope, *key)),
            egui::Sense::click(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                format!(
                    "Sort {scope} devices by {key} {}",
                    if selected && !ascending {
                        "ascending"
                    } else {
                        "descending"
                    }
                ),
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::ColumnHeader);
            node.set_label(format!(
                "{key}; {}; activate to sort {direction}",
                if selected {
                    if ascending {
                        "sorted ascending"
                    } else {
                        "sorted descending"
                    }
                } else {
                    "not sorted"
                },
                direction = if selected && !ascending {
                    "ascending"
                } else {
                    "descending"
                },
            ));
        });
        if response.hovered() {
            ui.painter().rect_filled(
                cell.shrink2(egui::vec2(2.0, 1.0)),
                0.0,
                t.color.bg_hover.gamma_multiply(0.45),
            );
        }
        if selected {
            let center = egui::pos2(cell.left() + 8.0, cell.center().y);
            let direction = if ascending { -1.0 } else { 1.0 };
            ui.painter().add(egui::Shape::convex_polygon(
                vec![
                    center + egui::vec2(0.0, direction * 3.0),
                    center + egui::vec2(-3.5, -direction * 2.5),
                    center + egui::vec2(3.5, -direction * 2.5),
                ],
                t.color.accent,
                egui::Stroke::NONE,
            ));
        }
        theme::paint_focus_ring(ui, &response, cell.shrink(1.0));
        if response.clicked() {
            clicked = Some((*key).to_owned());
        }
    }
    clicked
}

fn card_header(ui: &mut Ui, title: &str, count: usize, suffix: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), CARD_HEADER_H),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    paint_cell(
        ui,
        op_column_rect(rect, 0.0, rect.width() * 0.64),
        title,
        egui::Align2::LEFT_CENTER,
        theme::sans(tokens::FS_1, FontWeight::Medium),
        t.color.text,
    );
    paint_cell(
        ui,
        op_column_rect(rect, rect.width() * 0.64, rect.width() * 0.36),
        format!("{count} {suffix}"),
        egui::Align2::RIGHT_CENTER,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

fn stacked_body_height(available_height: f32) -> f32 {
    ((available_height - CARD_HEADER_H * 2.0 - 1.0) * 0.5).max(1.0)
}

fn result_mapping_is_current(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    run.prepared_receipt()
        .is_some_and(|receipt| receipt.project_revision() == state.workspace.project.revision())
        && state.simulation.cross_probe.is_current_for(
            &state.workspace.active_view,
            state.schematic.topology_version(),
        )
}

fn node_target_available(state: &AppState, name: &str) -> bool {
    if !result_mapping_is_current(state) {
        return false;
    }
    let name = signal_leaf(name);
    name != "0"
        && state
            .simulation
            .cross_probe
            .net_to_points
            .iter()
            .any(|(candidate, points)| candidate.eq_ignore_ascii_case(name) && !points.is_empty())
}

fn device_target(state: &AppState, name: &str) -> Option<u64> {
    if !result_mapping_is_current(state) {
        return None;
    }
    state
        .schematic
        .components
        .iter()
        .find(|component| component.spice_instance_name().eq_ignore_ascii_case(name))
        .map(|component| component.id)
}

fn apply_action(ui: &Ui, state: &mut AppState, action: OpAction) {
    match action {
        OpAction::LocateNode(name) => {
            let signal = if name
                .trim_start()
                .chars()
                .next()
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case(&'V'))
                && name.contains('(')
            {
                name
            } else {
                format!("V({})", signal_leaf(&name))
            };
            match crate::schematic::view::select_signal_conductor(state, &signal) {
                Ok(_) => {
                    state.ui.schematic_visibility.annotations =
                        SchematicAnnotationVisibility::OperatingPoint;
                    state
                        .workbench
                        .activate(crate::workbench::state::Workspace::Design);
                }
                Err(error) => state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Cannot locate node",
                    error.message(&signal),
                ),
            }
        }
        OpAction::LocateDevice(component_id) => {
            let Some(component) = state
                .schematic
                .components
                .iter()
                .find(|component| component.id == component_id)
            else {
                state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Cannot locate device",
                    "The retained device no longer resolves to the active schematic.",
                );
                return;
            };
            let position = component.pos;
            state
                .schematic
                .selection
                .select_only_component(component_id);
            state.schematic.center_request = Some(position);
            state.ui.schematic_visibility.annotations =
                SchematicAnnotationVisibility::OperatingPoint;
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Design);
        }
    }
}

fn show_solve_strip(ui: &mut Ui, evidence: &OpEvidence) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(11, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("OP")
                        .font(theme::mono(tokens::FS_1, FontWeight::SemiBold))
                        .color(t.color.accent),
                );
                ui.label(
                    egui::RichText::new("Operating point · retained DC solution")
                        .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                        .color(t.color.text),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!("Run {} · {}", evidence.run_id, evidence.label))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                if let Some(facts) = &evidence.facts {
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!(
                            "{} · {:.3} °C · point {}/{}",
                            process_label(facts.process),
                            facts.temperature_celsius,
                            facts.point_index.saturating_add(1),
                            facts.point_count.max(1),
                        ))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    );
                }
                let nodes = evidence.dc.as_ref().map_or(0, |dc| dc.node_voltages.len());
                let branches = evidence
                    .dc
                    .as_ref()
                    .map_or(0, |dc| dc.branch_currents.len());
                ui.separator();
                ui.label(
                    egui::RichText::new(format!("{nodes} nodes · {branches} branches"))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                if !evidence.success {
                    ui.separator();
                    ui.label(
                        egui::RichText::new("retained partial evidence")
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.err),
                    );
                }
            });
            if !evidence.success
                && let Some(error) = evidence.error.as_deref()
            {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            }
        });
}

fn unavailable_card(ui: &mut Ui, title: &str, message: &str) {
    card_header(ui, title, 0, "retained");
    let t = Tokens::get(ui.ctx());
    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(message)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn empty_table_message(ui: &mut Ui, message: impl Into<String>) {
    let t = Tokens::get(ui.ctx());
    let message = message.into();
    let response = ui.label(
        egui::RichText::new(&message)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(message);
    });
}

fn node_value_text(row: &crate::state::OperatingPointValue) -> (String, bool) {
    if row.value.is_finite() {
        (fmt_si(row.value, &row.unit, 6), true)
    } else {
        ("invalid · non-finite".to_owned(), false)
    }
}

fn show_node_card(
    ui: &mut Ui,
    evidence: &OpEvidence,
    filter: &str,
    body_max_height: Option<f32>,
    action: &mut Option<OpAction>,
    state: &AppState,
) -> usize {
    let Some(dc) = evidence.dc.as_ref() else {
        unavailable_card(
            ui,
            "Node voltages",
            "Node-voltage evidence was not retained for this analysis.",
        );
        return 0;
    };
    if dc.node_voltages.is_empty() {
        unavailable_card(
            ui,
            "Node voltages",
            "The retained DC solution contains no node-voltage rows.",
        );
        return 0;
    }
    let groups = grouped_nodes(dc, filter);
    let count = groups.values().map(Vec::len).sum::<usize>();
    card_header(ui, "Node voltages", count, "shown");
    if groups.is_empty() {
        empty_table_message(ui, format!("No retained node matches “{filter}”."));
        return 0;
    }

    let table_width = ui.available_width().max(500.0);
    // One flat row list, so the viewport plan below can address group headers
    // and node rows alike. A retained DC solution is one row per node, and a
    // real block has tens of thousands.
    let flat = flatten_groups(&groups);
    let offsets = RowOffsets::from_heights(flat.iter().map(NodeTableRow::height));
    let mut scroll = egui::ScrollArea::both()
        .id_salt("rspice.results.op.nodes")
        .auto_shrink([false, false]);
    if let Some(max_height) = body_max_height {
        scroll = scroll.max_height(max_height);
    }
    let table = scroll
        .show_viewport(ui, |ui, viewport| {
            ui.scope(|ui| {
            ui.set_min_width(table_width);
            column_header(
                ui,
                table_width,
                &[
                    (NAME_W, "NODE", false),
                    (VALUE_MIN_W + 36.0, "VOLTAGE", true),
                    (64.0, "UNIT", false),
                    (
                        table_width - NAME_W - VALUE_MIN_W - 100.0 - ACTION_W,
                        "IDENTITY",
                        false,
                    ),
                    (ACTION_W, "", false),
                ],
            );
            // The header is content, not chrome, so the body's own offsets
            // start below it.
            let plan = offsets.plan(egui::Rangef::new(
                viewport.min.y - HEADER_H,
                viewport.max.y - HEADER_H,
            ));
            ui.allocate_space(egui::vec2(table_width, plan.leading));
            for entry in &flat[plan.range()] {
                let row = match entry {
                    NodeTableRow::Group { scope, count } => {
                        group_header(ui, table_width, scope, *count);
                        continue;
                    }
                    NodeTableRow::Value(row) => *row,
                };
                {
                    let (rect, response) = ui
                        .allocate_exact_size(egui::vec2(table_width, ROW_H), egui::Sense::hover());
                    response.widget_info(|| {
                        let (value, valid) = node_value_text(row);
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Label,
                            true,
                            if valid {
                                format!("Node {}, value {value}", row.name)
                            } else {
                                format!("Node {}, invalid retained non-finite value", row.name)
                            },
                        )
                    });
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        let (value, valid) = node_value_text(row);
                        node.set_role(egui::accesskit::Role::Row);
                        node.set_label(if valid {
                            format!("Node {}; voltage {value}; identity {}", row.name, row.name)
                        } else {
                            format!(
                                "Node {}; voltage unavailable; retained value is non-finite; identity {}",
                                row.name, row.name
                            )
                        });
                    });
                    if response.hovered() {
                        ui.painter()
                            .rect_filled(rect, 0.0, Tokens::get(ui.ctx()).color.bg_hover);
                    }
                    ui.painter().hline(
                        rect.x_range(),
                        rect.bottom() - 0.5,
                        egui::Stroke::new(
                            1.0,
                            Tokens::get(ui.ctx()).color.border.gamma_multiply(0.6),
                        ),
                    );
                    let (_, leaf) = hierarchy_parts(&row.name);
                    paint_cell(
                        ui,
                        op_column_rect(rect, 0.0, NAME_W),
                        leaf,
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        Tokens::get(ui.ctx()).color.text,
                    );
                    let (value_text, value_valid) = node_value_text(row);
                    paint_cell(
                        ui,
                        op_column_rect(rect, NAME_W, VALUE_MIN_W + 36.0),
                        value_text,
                        egui::Align2::RIGHT_CENTER,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        if value_valid {
                            Tokens::get(ui.ctx()).color.text
                        } else {
                            Tokens::get(ui.ctx()).color.err
                        },
                    );
                    paint_cell(
                        ui,
                        op_column_rect(rect, NAME_W + VALUE_MIN_W + 36.0, 64.0),
                        &row.unit,
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        Tokens::get(ui.ctx()).color.text_faint,
                    );
                    let identity_w = table_width - NAME_W - VALUE_MIN_W - 100.0 - ACTION_W;
                    paint_cell(
                        ui,
                        op_column_rect(rect, NAME_W + VALUE_MIN_W + 100.0, identity_w),
                        &row.name,
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        Tokens::get(ui.ctx()).color.text_dim,
                    );
                    let available = node_target_available(state, &row.name);
                    let button_rect = op_column_rect(rect, table_width - ACTION_W, ACTION_W)
                        .shrink2(egui::vec2(7.0, 3.0));
                    let response = ui
                        .add_enabled_ui(available, |ui| {
                            ui.put(button_rect, egui::Button::new("Schematic"))
                        })
                        .inner
                        .on_disabled_hover_text(
                            "This retained node has no current, unambiguous schematic mapping.",
                        );
                    if response.clicked() && available {
                        *action = Some(OpAction::LocateNode(row.name.clone()));
                    }
                }
            }
            ui.allocate_space(egui::vec2(table_width, plan.trailing));
            })
            .response
        });
    ui.ctx().accesskit_node_builder(table.inner.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label("Operating-point node voltages");
    });
    count
}

/// One drawable line of the node table: a scope heading, or a retained node.
enum NodeTableRow<'a> {
    Group { scope: &'a str, count: usize },
    Value(&'a crate::state::OperatingPointValue),
}

impl NodeTableRow<'_> {
    const fn height(&self) -> f32 {
        match self {
            Self::Group { .. } => GROUP_H,
            Self::Value(_) => ROW_H,
        }
    }
}

fn flatten_groups<'a>(
    groups: &'a BTreeMap<String, Vec<&'a crate::state::OperatingPointValue>>,
) -> Vec<NodeTableRow<'a>> {
    let mut flat = Vec::with_capacity(groups.len() + groups.values().map(Vec::len).sum::<usize>());
    for (scope, rows) in groups {
        flat.push(NodeTableRow::Group {
            scope,
            count: rows.len(),
        });
        flat.extend(rows.iter().copied().map(NodeTableRow::Value));
    }
    flat
}

fn show_device_card(
    ui: &mut Ui,
    evidence: &OpEvidence,
    filter: &str,
    sort: Option<&(String, bool)>,
    body_max_height: Option<f32>,
    clicked_sort: &mut Option<String>,
    action: &mut Option<OpAction>,
    state: &AppState,
) -> usize {
    if evidence
        .detail
        .as_ref()
        .is_some_and(|detail| detail.policy == OperatingPointDeviceDetailEvidence::None)
    {
        unavailable_card(
            ui,
            "Device operating points",
            "The executed save policy explicitly retained no per-device operating-point detail.",
        );
        return 0;
    }
    let Some(report) = evidence.devices.as_ref() else {
        unavailable_card(
            ui,
            "Device operating points",
            "Per-device operating-point quantities were not retained for this analysis.",
        );
        return 0;
    };
    if report.entries.is_empty() {
        unavailable_card(
            ui,
            "Device operating points",
            "The retained per-device report contains no device rows.",
        );
        return 0;
    }
    let groups = grouped_devices(report, evidence.detail.as_ref(), filter, sort);
    let count = groups.values().map(Vec::len).sum::<usize>();
    card_header(ui, "Device operating points", count, "shown");
    if groups.is_empty() {
        if filter.trim().is_empty() {
            empty_table_message(
                ui,
                "No device row satisfies the retained device-detail scope.",
            );
        } else {
            empty_table_message(ui, format!("No retained device matches “{filter}”."));
        }
        return 0;
    }

    // Each scope carries its own column set, so the flat list references a
    // layout rather than repeating it per row. `save_device_op` on a real
    // block is one row per device; drawing them all every frame is what this
    // plan exists to stop.
    let layouts: Vec<DeviceGroupLayout<'_>> = groups
        .iter()
        .map(|(scope, rows)| DeviceGroupLayout {
            scope,
            columns: device_columns(rows),
            count: rows.len(),
        })
        .collect();
    let flat = flatten_device_groups(&groups);
    let offsets = RowOffsets::from_heights(flat.iter().map(DeviceTableRow::height));
    let mut scroll = egui::ScrollArea::both()
        .id_salt("rspice.results.op.devices")
        .auto_shrink([false, false]);
    if let Some(max_height) = body_max_height {
        scroll = scroll.max_height(max_height);
    }
    let table = scroll.show_viewport(ui, |ui, viewport| {
        ui.scope(|ui| {
            let available = ui.available_width();
            let width_of = |layout: &DeviceGroupLayout<'_>| {
                (NAME_W + KIND_W + REGION_W + layout.columns.len() as f32 * VALUE_MIN_W + ACTION_W)
                    .max(available)
            };
            let widest = layouts.iter().map(width_of).fold(available, f32::max);
            ui.set_min_width(widest);
            let plan = offsets.plan(viewport.y_range());
            ui.allocate_space(egui::vec2(widest, plan.leading));
            for row in &flat[plan.range()] {
                let layout = &layouts[row.layout()];
                let table_width = width_of(layout);
                let entry = match row {
                    DeviceTableRow::Group(_) => {
                        group_header(ui, table_width, layout.scope, layout.count);
                        continue;
                    }
                    DeviceTableRow::ColumnHeader(_) => {
                        if let Some(key) = device_column_header(
                            ui,
                            table_width,
                            &layout.columns,
                            layout.scope,
                            sort,
                        ) {
                            *clicked_sort = Some(key);
                        }
                        continue;
                    }
                    DeviceTableRow::Gap(_) => {
                        ui.add_space(DEVICE_GROUP_GAP);
                        continue;
                    }
                    DeviceTableRow::Device { entry, .. } => *entry,
                };
                let columns = &layout.columns;
                {
                    let (rect, response) = ui
                        .allocate_exact_size(egui::vec2(table_width, ROW_H), egui::Sense::hover());
                    response.widget_info(|| {
                        let values = entry
                            .params
                            .iter()
                            .map(|(name, value)| format!("{name} {value}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Label,
                            true,
                            format!(
                                "{} device {}, region {}, {}",
                                entry.device_kind,
                                entry.name,
                                entry.region.unwrap_or("not reported"),
                                values,
                            ),
                        )
                    });
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        let values = entry
                            .params
                            .iter()
                            .map(|(name, value)| format!("{name} {value}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        node.set_role(egui::accesskit::Role::Row);
                        node.set_label(format!(
                            "Instance {}; family {}; region {}; {}",
                            entry.name,
                            entry.device_kind,
                            entry.region.unwrap_or("not reported"),
                            values,
                        ));
                    });
                    let colors = &Tokens::get(ui.ctx()).color;
                    if response.hovered() {
                        ui.painter().rect_filled(rect, 0.0, colors.bg_hover);
                    }
                    ui.painter().hline(
                        rect.x_range(),
                        rect.bottom() - 0.5,
                        egui::Stroke::new(1.0, colors.border.gamma_multiply(0.6)),
                    );
                    let (_, leaf) = hierarchy_parts(&entry.name);
                    paint_cell(
                        ui,
                        op_column_rect(rect, 0.0, NAME_W),
                        leaf,
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        colors.text,
                    );
                    paint_cell(
                        ui,
                        op_column_rect(rect, NAME_W, KIND_W),
                        entry.device_kind,
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        colors.text_dim,
                    );
                    if let Some(region) = entry.region {
                        paint_cell(
                            ui,
                            op_column_rect(rect, NAME_W + KIND_W, REGION_W),
                            region,
                            egui::Align2::LEFT_CENTER,
                            theme::mono(tokens::FS_0, FontWeight::Regular),
                            region_color(region, colors),
                        );
                    }
                    for (index, (name, unit)) in columns.iter().enumerate() {
                        let text = entry
                            .params
                            .iter()
                            .find(|(candidate, _)| candidate == name)
                            .map(|(_, value)| {
                                if unit.is_empty() {
                                    crate::state::format_engineering(*value)
                                } else {
                                    fmt_si(*value, unit, 4)
                                }
                            })
                            .unwrap_or_else(|| "—".to_owned());
                        paint_cell(
                            ui,
                            op_column_rect(
                                rect,
                                NAME_W + KIND_W + REGION_W + index as f32 * VALUE_MIN_W,
                                VALUE_MIN_W,
                            ),
                            text,
                            egui::Align2::RIGHT_CENTER,
                            theme::mono(tokens::FS_1, FontWeight::Regular),
                            colors.text_dim,
                        );
                    }
                    let target = device_target(state, &entry.name);
                    let button_rect = op_column_rect(rect, table_width - ACTION_W, ACTION_W)
                        .shrink2(egui::vec2(7.0, 3.0));
                    let response = ui
                        .add_enabled_ui(target.is_some(), |ui| {
                            ui.put(button_rect, egui::Button::new("Schematic"))
                        })
                        .inner
                        .on_disabled_hover_text(
                            "This retained device has no current, exact schematic identity.",
                        );
                    if response.clicked()
                        && let Some(component_id) = target
                    {
                        *action = Some(OpAction::LocateDevice(component_id));
                    }
                }
            }
            ui.allocate_space(egui::vec2(widest, plan.trailing));
        })
        .response
    });
    ui.ctx().accesskit_node_builder(table.inner.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label("Device operating points");
    });
    count
}

/// The column set one device scope draws, resolved once per frame.
struct DeviceGroupLayout<'a> {
    scope: &'a str,
    columns: Vec<(&'static str, &'static str)>,
    count: usize,
}

/// One drawable line of the device table.
enum DeviceTableRow<'a> {
    Group(usize),
    ColumnHeader(usize),
    Device {
        layout: usize,
        entry: &'a rspice_core::circuit::DeviceOpEntry,
    },
    /// The breathing room between scopes. It is a row like any other here so
    /// the plan's arithmetic stays exact — a gap left out of the offsets
    /// drifts the viewport by 8 px per group above it.
    Gap(usize),
}

impl DeviceTableRow<'_> {
    const fn height(&self) -> f32 {
        match self {
            Self::Group(_) => GROUP_H,
            Self::ColumnHeader(_) => HEADER_H,
            Self::Device { .. } => ROW_H,
            Self::Gap(_) => DEVICE_GROUP_GAP,
        }
    }

    const fn layout(&self) -> usize {
        match self {
            Self::Group(layout)
            | Self::ColumnHeader(layout)
            | Self::Gap(layout)
            | Self::Device { layout, .. } => *layout,
        }
    }
}

fn flatten_device_groups<'a>(
    groups: &'a BTreeMap<String, Vec<&'a rspice_core::circuit::DeviceOpEntry>>,
) -> Vec<DeviceTableRow<'a>> {
    let mut flat = Vec::new();
    for (layout, rows) in groups.values().enumerate() {
        flat.push(DeviceTableRow::Group(layout));
        flat.push(DeviceTableRow::ColumnHeader(layout));
        flat.extend(
            rows.iter()
                .copied()
                .map(|entry| DeviceTableRow::Device { layout, entry }),
        );
        flat.push(DeviceTableRow::Gap(layout));
    }
    flat
}

fn region_color(region: &str, colors: &crate::ui::palette::Palette) -> egui::Color32 {
    match region.to_ascii_lowercase().as_str() {
        "saturation" | "forward active" => colors.ok,
        "cutoff" | "breakdown" => colors.err,
        _ => colors.traces[1],
    }
}

/// Render the structured operating-point document.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(evidence) = selected_op_evidence(state) else {
        let message = match state.simulation.active_analysis() {
            Some(_) => "The selected analysis is not a DC operating-point result.",
            None => "No operating-point analysis is selected.",
        };
        well_hint(ui, message);
        return;
    };
    if evidence.dc.is_none() && evidence.devices.is_none() {
        well_hint(
            ui,
            "The selected operating-point analysis retained no node, branch, or device values.",
        );
        return;
    }

    show_solve_strip(ui, &evidence);
    ui.add_space(1.0);

    let filter = state.ui.results.op_filter.clone();
    let sort = state.ui.results.op_sort.clone();
    let mut clicked_sort = None;
    let mut action = None;
    let available = ui.available_size();
    if available.x >= TWO_CARD_BREAKPOINT {
        ui.columns(2, |columns| {
            columns[0].set_min_height(available.y);
            columns[1].set_min_height(available.y);
            show_node_card(
                &mut columns[0],
                &evidence,
                &filter,
                None,
                &mut action,
                state,
            );
            show_device_card(
                &mut columns[1],
                &evidence,
                &filter,
                sort.as_ref(),
                None,
                &mut clicked_sort,
                &mut action,
                state,
            );
        });
    } else {
        // Both structured cards remain in the viewport. Each body owns a
        // bounded two-axis scroll region; neither can consume the space needed
        // to reach the other card.
        let stacked_body_height = stacked_body_height(available.y);
        show_node_card(
            ui,
            &evidence,
            &filter,
            Some(stacked_body_height),
            &mut action,
            state,
        );
        ui.add_space(1.0);
        show_device_card(
            ui,
            &evidence,
            &filter,
            sort.as_ref(),
            Some(stacked_body_height),
            &mut clicked_sort,
            &mut action,
            state,
        );
    }

    if let Some(action) = action {
        apply_action(ui, state, action);
    }
    if let Some(key) = clicked_sort {
        state.ui.results.op_sort = match state.ui.results.op_sort.as_ref() {
            Some((current, ascending)) if current.eq_ignore_ascii_case(&key) => {
                Some((key, !*ascending))
            }
            // Analog operating-point work generally starts with the largest
            // retained magnitude, so a new quantity sorts descending by
            // absolute value while preserving the retained sign in the table.
            _ => Some((key, false)),
        };
    }
}

/// Right panel: identity and retention facts for the selected OP analysis.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(evidence) = selected_op_evidence(state) else {
        return;
    };

    section_header(ui, "OP result", None);
    let run = format!("Run {}", evidence.run_id);
    let node_count = evidence
        .dc
        .as_ref()
        .map_or(0, |dc| dc.node_voltages.len())
        .to_string();
    let branch_count = evidence
        .dc
        .as_ref()
        .map_or(0, |dc| dc.branch_currents.len())
        .to_string();
    let device_count = evidence
        .devices
        .as_ref()
        .map_or(0, |report| {
            report
                .entries
                .iter()
                .filter(|entry| retained_detail_allows(&entry.name, evidence.detail.as_ref()))
                .count()
        })
        .to_string();
    measurement_table(
        ui,
        &[
            ("Run", run.as_str()),
            ("Analysis", evidence.label.as_str()),
            ("Node values", node_count.as_str()),
            ("Branch values", branch_count.as_str()),
            ("Device rows", device_count.as_str()),
            ("Device scope", detail_label(evidence.detail.as_ref())),
        ],
    );

    if let Some(facts) = &evidence.facts {
        ui.add_space(8.0);
        section_header(ui, "Retained solve facts", None);
        let temperature = format!("{:.3} °C", facts.temperature_celsius);
        let point = format!(
            "{} / {}",
            facts.point_index.saturating_add(1),
            facts.point_count.max(1)
        );
        let process = process_label(facts.process);
        let mna_nodes = facts.mna_nodes.to_string();
        let mna_branches = facts.mna_branches.to_string();
        measurement_table(
            ui,
            &[
                ("Process", process),
                ("Temperature", temperature.as_str()),
                ("Run-set point", point.as_str()),
                ("MNA node rows", mna_nodes.as_str()),
                ("MNA branch rows", mna_branches.as_str()),
                ("Back annotation", annotation_label(facts.annotation)),
            ],
        );
    }

    ui.add_space(8.0);
    section_header(ui, "Schematic association", None);
    let current = result_mapping_is_current(state);
    measurement_table(
        ui,
        &[(
            "Mapping",
            if current {
                "current · cross-probe enabled"
            } else {
                "not current · actions disabled"
            },
        )],
    );
}

#[cfg(test)]
mod layout_tests {
    use super::*;

    #[test]
    fn hierarchy_preserves_scope_and_leaf_without_inventing_a_parent() {
        assert_eq!(hierarchy_parts("V(top/amp.out)"), ("top/amp", "out"));
        assert_eq!(hierarchy_parts("XAFE/M1"), ("XAFE", "M1"));
        assert_eq!(hierarchy_parts("V(out)"), ("Top", "out"));
        assert_eq!(hierarchy_parts("0"), ("Top", "0"));
    }

    #[test]
    fn stacked_cards_reserve_room_for_both_headers_and_bodies() {
        let available = 480.0;
        let body = stacked_body_height(available);
        assert!(body > 1.0);
        assert!(CARD_HEADER_H * 2.0 + body * 2.0 + 1.0 <= available);
        assert_eq!(stacked_body_height(40.0), 1.0);
    }

    #[test]
    fn shared_filter_covers_hierarchy_family_region_and_quantity() {
        let entry = rspice_core::circuit::DeviceOpEntry {
            name: "XAFE/M1".to_owned(),
            device_kind: "MOSFET",
            region: Some("saturation"),
            params: vec![("gm", 1.2e-3)],
        };
        assert!(device_matches(&entry, "xafe"));
        assert!(device_matches(&entry, "mosfet"));
        assert!(device_matches(&entry, "satur"));
        assert!(device_matches(&entry, "gm"));
        assert!(!device_matches(&entry, "diode"));
    }

    #[test]
    fn detail_policy_never_expands_beyond_retained_rows() {
        let selected = RetainedDetail {
            policy: OperatingPointDeviceDetailEvidence::SelectedAndViolations,
            selected: vec!["M1".to_owned()],
            violations: vec!["M3".to_owned()],
        };
        assert!(retained_detail_allows("m1", Some(&selected)));
        assert!(retained_detail_allows("M3", Some(&selected)));
        assert!(!retained_detail_allows("M2", Some(&selected)));

        let none = RetainedDetail {
            policy: OperatingPointDeviceDetailEvidence::None,
            selected: Vec::new(),
            violations: Vec::new(),
        };
        assert!(!retained_detail_allows("M1", Some(&none)));
    }

    #[test]
    fn selection_never_falls_back_to_another_analysis() {
        use crate::state::{AnalysisResult, SimulationRun};

        let mut state = AppState::default();
        let mut run = SimulationRun::new(9);
        let mut first = AnalysisResult::new(1, AnalysisType::DcOp, "OP 1");
        first.dc_op = Some(DcOpResult {
            node_voltages: vec![crate::state::OperatingPointValue {
                name: "V(out)".to_owned(),
                value: 1.0,
                unit: "V".to_owned(),
            }],
            ..DcOpResult::default()
        });
        run.add_analysis(first);
        run.add_analysis(AnalysisResult::new(2, AnalysisType::Transient, "TRAN"));
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);

        state.simulation.active_analysis_idx = Some(1);
        assert!(selected_op_evidence(&state).is_none());
        state.simulation.active_analysis_idx = Some(0);
        let evidence = selected_op_evidence(&state).expect("selected OP evidence");
        assert_eq!(evidence.run_id, 9);
        assert_eq!(evidence.label, "OP 1");
        assert_eq!(evidence.dc.unwrap().node_voltages.len(), 1);
    }

    #[test]
    fn node_evidence_does_not_depend_on_a_device_report() {
        use crate::state::{AnalysisResult, SimulationRun};

        let mut state = AppState::default();
        let mut run = SimulationRun::new(4);
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
        analysis.dc_op = Some(DcOpResult {
            node_voltages: vec![crate::state::OperatingPointValue {
                name: "V(top/out)".to_owned(),
                value: 2.5,
                unit: "V".to_owned(),
            }],
            ..DcOpResult::default()
        });
        run.add_analysis(analysis);
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        let evidence = selected_op_evidence(&state).expect("retained OP evidence");
        assert!(evidence.devices.is_none());
        assert_eq!(grouped_nodes(evidence.dc.as_ref().unwrap(), "out").len(), 1);
    }

    #[test]
    fn non_finite_node_evidence_is_preserved_and_explicitly_invalid() {
        let dc = DcOpResult {
            node_voltages: vec![crate::state::OperatingPointValue {
                name: "V(failed)".to_owned(),
                value: f64::NAN,
                unit: "V".to_owned(),
            }],
            ..DcOpResult::default()
        };

        let groups = grouped_nodes(&dc, "");
        assert_eq!(groups["Top"].len(), 1);
        let (display, valid) = node_value_text(groups["Top"][0]);
        assert_eq!(display, "invalid · non-finite");
        assert!(!valid);
    }

    #[test]
    fn device_quantity_sort_is_numeric_stable_and_keeps_missing_values_last() {
        let report = rspice_core::circuit::DeviceOpReport {
            entries: vec![
                rspice_core::circuit::DeviceOpEntry {
                    name: "M_missing".to_owned(),
                    device_kind: "MOSFET",
                    region: Some("cutoff"),
                    params: vec![],
                },
                rspice_core::circuit::DeviceOpEntry {
                    name: "M_low".to_owned(),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: vec![("gm", 1.0e-3)],
                },
                rspice_core::circuit::DeviceOpEntry {
                    name: "M_high".to_owned(),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: vec![("gm", 3.0e-3)],
                },
                rspice_core::circuit::DeviceOpEntry {
                    name: "M_negative".to_owned(),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: vec![("gm", -4.0e-3)],
                },
            ],
        };

        let descending = ("gm".to_owned(), false);
        let groups = grouped_devices(&report, None, "", Some(&descending));
        let names = groups["Top"]
            .iter()
            .map(|entry| entry.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["M_negative", "M_high", "M_low", "M_missing"]);

        let ascending = ("gm".to_owned(), true);
        let groups = grouped_devices(&report, None, "", Some(&ascending));
        let names = groups["Top"]
            .iter()
            .map(|entry| entry.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["M_low", "M_high", "M_negative", "M_missing"]);
    }
}
