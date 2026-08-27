//! Structured DC operating-point result document.
//!
//! The viewer deliberately renders only the explicitly selected analysis. Node
//! voltages, branch currents, device quantities, run-point facts, and retained
//! detail policy all come from immutable result evidence; the UI never fills a
//! missing solver fact with a design-time or fixture value.

use std::collections::BTreeMap;
use std::sync::Arc;

use egui::Ui;

use crate::simulation::netlist_gen::bus_notations;
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

use super::AnalysisPresentationKey;
use super::frame_work::{self, DatasetWalk};
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

/// The sheet's identity and solve facts, without the solution itself.
///
/// The retained node and device tables stay where they are; a viewer that
/// copied a million operating-point rows to read six scalars off the header
/// was paying for the whole solution twice a frame, once here and once in
/// the right panel.
#[derive(Clone)]
struct OpEvidence {
    run_id: u64,
    label: String,
    success: bool,
    error: Option<String>,
    detail_policy: Option<OperatingPointDeviceDetailEvidence>,
    node_count: usize,
    branch_count: usize,
    facts: Option<SolveFacts>,
}

#[derive(Clone)]
enum OpAction {
    LocateNode(String),
    LocateDevice(u64),
}

/// The retained device-detail scope, which is what decides whether a device
/// row is allowed to appear at all. Read only when a plan is being built.
fn retained_detail(analysis: &crate::state::AnalysisResult) -> Option<RetainedDetail> {
    match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::OperatingPoint {
            device_detail,
            selected_devices,
            violation_devices,
            ..
        }) => Some(RetainedDetail {
            policy: *device_detail,
            selected: selected_devices.clone(),
            violations: violation_devices.clone(),
        }),
        _ => None,
    }
}

fn selected_op_evidence(state: &AppState) -> Option<OpEvidence> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    if analysis.analysis_type != AnalysisType::DcOp {
        return None;
    }

    let (detail_policy, facts) = match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::OperatingPoint {
            temperature_celsius,
            annotation,
            device_detail,
            mna_node_names,
            mna_branch_names,
            run_point_index,
            run_point_count,
            run_point_process,
            ..
        }) => (
            Some(*device_detail),
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
        detail_policy,
        node_count: analysis
            .dc_op
            .as_ref()
            .map_or(0, |dc| dc.node_voltages.len()),
        branch_count: analysis
            .dc_op
            .as_ref()
            .map_or(0, |dc| dc.branch_currents.len()),
        facts,
    })
}

/// Lossless projection of all evidence shown by the operating-point sheet.
pub(crate) fn export_csv(analysis: &crate::state::AnalysisResult) -> Option<super::ResultSheetCsv> {
    if !analysis.success || analysis.analysis_type != AnalysisType::DcOp {
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
    let dc = analysis.dc_op.as_ref();
    let devices = analysis.device_op.as_ref();
    if dc.is_none() && devices.is_none() && facts.is_none() {
        return None;
    }

    let mut contents = String::from("section,owner,kind,region,quantity,value,unit,detail\n");
    let mut rows = 0usize;
    {
        let mut push_value = |section: &str,
                              owner: &str,
                              kind: &str,
                              region: &str,
                              quantity: &str,
                              value: String,
                              unit: &str,
                              row_detail: &str| {
            contents.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                super::csv_field(section),
                super::csv_field(owner),
                super::csv_field(kind),
                super::csv_field(region),
                super::csv_field(quantity),
                super::csv_field(&value),
                super::csv_field(unit),
                super::csv_field(row_detail),
            ));
            rows += 1;
        };
        if let Some(facts) = facts.as_ref() {
            for (quantity, value, unit) in [
                (
                    "temperature",
                    format!("{:.17e}", facts.temperature_celsius),
                    "degC",
                ),
                ("process", process_label(facts.process).to_owned(), ""),
                ("point_index", facts.point_index.to_string(), ""),
                ("point_count", facts.point_count.to_string(), ""),
                ("mna_nodes", facts.mna_nodes.to_string(), "count"),
                ("mna_branches", facts.mna_branches.to_string(), "count"),
                (
                    "annotation",
                    annotation_label(facts.annotation).to_owned(),
                    "",
                ),
            ] {
                push_value("solve_fact", "", "", "", quantity, value, unit, "");
            }
        }
        if let Some(dc) = dc {
            for (section, values) in [
                ("node_voltage", dc.node_voltages.as_slice()),
                ("branch_current", dc.branch_currents.as_slice()),
                ("device_power", dc.power_dissipation.as_slice()),
            ] {
                for value in values {
                    let row_detail = if value.value.is_finite() {
                        ""
                    } else {
                        "non-finite retained value"
                    };
                    push_value(
                        section,
                        &value.name,
                        "",
                        "",
                        "value",
                        format!("{:.17e}", value.value),
                        &value.unit,
                        row_detail,
                    );
                }
            }
        }
        if let Some(devices) = devices {
            for entry in devices
                .entries
                .iter()
                .filter(|entry| retained_detail_allows(&entry.name, detail.as_ref()))
            {
                for (name, value) in &entry.params {
                    let row_detail = if value.is_finite() {
                        ""
                    } else {
                        "non-finite retained value"
                    };
                    push_value(
                        "device_parameter",
                        &entry.name,
                        entry.device_kind,
                        entry.region.unwrap_or_default(),
                        name,
                        format!("{value:.17e}"),
                        device_param_unit(entry.device_kind, name),
                        row_detail,
                    );
                }
            }
        }
    }

    Some(super::ResultSheetCsv {
        default_name: "rspice-operating-point.csv",
        detail: format!("{rows} operating-point evidence rows"),
        contents,
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

fn detail_label(policy: Option<OperatingPointDeviceDetailEvidence>) -> &'static str {
    match policy {
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

/// The occurrence a retained name belongs to, and the signal inside it.
///
/// The design root is implicit in a path, but a group header has to name
/// something, so a root-scoped row is filed under `root` — the cell the run was
/// actually taken from — rather than under a literal this module invents. A
/// name the path grammar cannot resolve keeps its whole spelling and stays at
/// the root, because guessing a scope for it would file retained evidence
/// under hierarchy the design does not have.
///
/// A scope is named by its fold key, which is the only identity a path map may
/// use: retained names arrive in the engine's flattened lower-case spelling,
/// and folding is what keeps a canonical `/X1/n` and an engine `x1.n` in one
/// group rather than in two that differ only in case.
fn hierarchy_parts(name: &str, root: &str) -> (String, String) {
    let signal = signal_leaf(name);
    match crate::state::ProbeTarget::parse_legacy(signal) {
        Ok(target) if !target.scope.is_root() => (target.scope.fold_key(), target.leaf),
        Ok(target) => (root.to_owned(), target.leaf),
        Err(_) => (root.to_owned(), signal.to_owned()),
    }
}

fn node_matches(row: &crate::state::OperatingPointValue, filter: &str, root: &str) -> bool {
    if filter.trim().is_empty() {
        return true;
    }
    let filter = filter.trim().to_ascii_lowercase();
    let (scope, leaf) = hierarchy_parts(&row.name, root);
    row.name.to_ascii_lowercase().contains(&filter)
        || scope.to_ascii_lowercase().contains(&filter)
        || leaf.to_ascii_lowercase().contains(&filter)
        || row.unit.to_ascii_lowercase().contains(&filter)
}

fn device_matches(entry: &rspice_core::circuit::DeviceOpEntry, filter: &str, root: &str) -> bool {
    if filter.trim().is_empty() {
        return true;
    }
    let filter = filter.trim().to_ascii_lowercase();
    let (scope, leaf) = hierarchy_parts(&entry.name, root);
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

/// Sort each group by its rows' leaf names, case-folded.
///
/// The fold happens once per row rather than twice per comparison: an
/// `N log N` sort that allocates two strings per comparison is the shape
/// that made a large operating point cost more to sort than to solve.
fn sort_groups_by_leaf(groups: &mut BTreeMap<String, Vec<(String, usize)>>) {
    for rows in groups.values_mut() {
        rows.sort_by(|left, right| left.0.cmp(&right.0));
    }
}

fn grouped_nodes(
    dc: &DcOpResult,
    filter: &str,
    root: &str,
) -> BTreeMap<String, Vec<(String, usize)>> {
    let mut groups = BTreeMap::<String, Vec<(String, usize)>>::new();
    for (index, row) in dc
        .node_voltages
        .iter()
        .enumerate()
        .filter(|(_, row)| node_matches(row, filter, root))
    {
        groups
            .entry(hierarchy_parts(&row.name, root).0)
            .or_default()
            .push((signal_leaf(&row.name).to_ascii_lowercase(), index));
    }
    sort_groups_by_leaf(&mut groups);
    groups
}

fn grouped_devices(
    report: &rspice_core::circuit::DeviceOpReport,
    detail: Option<&RetainedDetail>,
    filter: &str,
    sort: Option<&(String, bool)>,
    root: &str,
) -> BTreeMap<String, Vec<(String, usize)>> {
    let mut groups = BTreeMap::<String, Vec<(String, usize)>>::new();
    for (index, entry) in report
        .entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| retained_detail_allows(&entry.name, detail))
        .filter(|(_, entry)| device_matches(entry, filter, root))
    {
        groups
            .entry(hierarchy_parts(&entry.name, root).0)
            .or_default()
            .push((signal_leaf(&entry.name).to_ascii_lowercase(), index));
    }
    let Some((key, ascending)) = sort else {
        sort_groups_by_leaf(&mut groups);
        return groups;
    };
    for rows in groups.values_mut() {
        rows.sort_by(|left, right| {
            let left_value = device_sort_value(&report.entries[left.1], key);
            let right_value = device_sort_value(&report.entries[right.1], key);
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
            value_order.then_with(|| left.0.cmp(&right.0))
        });
    }
    groups
}

/// Everything the row plan below was built from.
///
/// The filter, sort and occurrence root are the reader's controls; the data
/// version and analysis are the evidence. A plan whose key still matches is
/// the same plan, so the frame reads it instead of rebuilding it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct OpPlanKey {
    version: u64,
    analysis: AnalysisPresentationKey,
    filter: String,
    sort: Option<(String, bool)>,
    root: String,
}

/// One occurrence heading, with the column set its rows carry.
#[derive(Debug, Clone, PartialEq, Eq)]
struct OpScope {
    scope: String,
    columns: Vec<(&'static str, &'static str)>,
    count: usize,
}

/// One drawable line of the node table, addressed by index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodePlanRow {
    Group(usize),
    Value(usize),
}

/// One drawable line of the device table, addressed by index.
///
/// The gap between scopes is a row like any other so the plan's arithmetic
/// stays exact — a gap left out of the offsets drifts the viewport by 8 px
/// per group above it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DevicePlanRow {
    Group(usize),
    ColumnHeader(usize),
    Gap(usize),
    Device { scope: usize, entry: usize },
}

impl NodePlanRow {
    const fn height(self) -> f32 {
        match self {
            Self::Group(_) => GROUP_H,
            Self::Value(_) => ROW_H,
        }
    }
}

impl DevicePlanRow {
    const fn height(self) -> f32 {
        match self {
            Self::Group(_) => GROUP_H,
            Self::ColumnHeader(_) => HEADER_H,
            Self::Device { .. } => ROW_H,
            Self::Gap(_) => DEVICE_GROUP_GAP,
        }
    }

    const fn scope(self) -> usize {
        match self {
            Self::Group(scope)
            | Self::ColumnHeader(scope)
            | Self::Gap(scope)
            | Self::Device { scope, .. } => scope,
        }
    }
}

/// The operating-point sheet's laid-out rows, built once per key.
///
/// `save_device_op` on a real block emits one row per device and one node
/// row per net, and both tables grouped, sorted, flattened and measured all
/// of them on every frame. Rows are held as indices into the retained
/// evidence, so the plan is the order — never a second copy of the solution.
#[derive(Debug, Clone)]
pub(super) struct OpPlan {
    key: OpPlanKey,
    node_scopes: Vec<OpScope>,
    node_rows: Vec<NodePlanRow>,
    node_offsets: RowOffsets,
    node_shown: usize,
    device_scopes: Vec<OpScope>,
    device_rows: Vec<DevicePlanRow>,
    device_offsets: RowOffsets,
    device_shown: usize,
    /// Device rows the retained detail policy admits, before the reader's
    /// text filter. The panel states the retained scope, not the search.
    device_in_scope: usize,
}

fn build_op_plan(
    key: OpPlanKey,
    dc: Option<&DcOpResult>,
    devices: Option<&rspice_core::circuit::DeviceOpReport>,
    detail: Option<&RetainedDetail>,
) -> OpPlan {
    frame_work::note(DatasetWalk::OpPlan);
    let mut node_scopes = Vec::new();
    let mut node_rows = Vec::new();
    let mut node_shown = 0;
    if let Some(dc) = dc {
        for (scope, rows) in grouped_nodes(dc, &key.filter, &key.root) {
            let index = node_scopes.len();
            node_shown += rows.len();
            node_scopes.push(OpScope {
                scope,
                columns: Vec::new(),
                count: rows.len(),
            });
            node_rows.push(NodePlanRow::Group(index));
            node_rows.extend(rows.into_iter().map(|(_, row)| NodePlanRow::Value(row)));
        }
    }

    let mut device_scopes = Vec::new();
    let mut device_rows = Vec::new();
    let mut device_shown = 0;
    if let Some(report) = devices {
        for (scope, rows) in
            grouped_devices(report, detail, &key.filter, key.sort.as_ref(), &key.root)
        {
            let index = device_scopes.len();
            device_shown += rows.len();
            device_scopes.push(OpScope {
                columns: device_columns(report, &rows),
                scope,
                count: rows.len(),
            });
            device_rows.push(DevicePlanRow::Group(index));
            device_rows.push(DevicePlanRow::ColumnHeader(index));
            device_rows.extend(rows.into_iter().map(|(_, entry)| DevicePlanRow::Device {
                scope: index,
                entry,
            }));
            device_rows.push(DevicePlanRow::Gap(index));
        }
    }

    let device_in_scope = devices.map_or(0, |report| {
        report
            .entries
            .iter()
            .filter(|entry| retained_detail_allows(&entry.name, detail))
            .count()
    });
    OpPlan {
        key,
        device_in_scope,
        node_offsets: RowOffsets::from_heights(node_rows.iter().map(|row| row.height())),
        node_scopes,
        node_rows,
        node_shown,
        device_offsets: RowOffsets::from_heights(device_rows.iter().map(|row| row.height())),
        device_scopes,
        device_rows,
        device_shown,
    }
}

/// The row plan for the selected operating point under the current controls.
fn op_plan(state: &mut AppState, analysis: AnalysisPresentationKey) -> Option<Arc<OpPlan>> {
    let key = OpPlanKey {
        version: state.simulation.data_version,
        analysis,
        filter: state.ui.results.op_filter.clone(),
        sort: state.ui.results.op_sort.clone(),
        root: state.workspace.simulation_root_reference().cell,
    };
    if let Some(plan) = state.ui.results.plans.op.as_ref()
        && plan.key == key
    {
        return Some(Arc::clone(plan));
    }
    let analysis_result = state.simulation.active_analysis()?;
    let detail = retained_detail(analysis_result);
    let built = Arc::new(build_op_plan(
        key,
        analysis_result.dc_op.as_ref(),
        analysis_result.device_op.as_ref(),
        detail.as_ref(),
    ));
    state.ui.results.plans.op = Some(Arc::clone(&built));
    Some(built)
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
    report: &rspice_core::circuit::DeviceOpReport,
    rows: &[(String, usize)],
) -> Vec<(&'static str, &'static str)> {
    let mut columns = Vec::new();
    for (_, index) in rows {
        let entry = &report.entries[*index];
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
                let nodes = evidence.node_count;
                let branches = evidence.branch_count;
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
    plan: &OpPlan,
    filter: &str,
    root: &str,
    body_max_height: Option<f32>,
    action: &mut Option<OpAction>,
    state: &AppState,
) -> usize {
    let Some(dc) = state
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.dc_op.as_ref())
    else {
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
    let count = plan.node_shown;
    card_header(ui, "Node voltages", count, "shown");
    if plan.node_scopes.is_empty() {
        empty_table_message(ui, format!("No retained node matches “{filter}”."));
        return 0;
    }

    // The NODE column names the conductor the design drew; the IDENTITY column
    // beside it keeps the deck name the engine solved under.
    let notations = bus_notations(&state.workspace, &state.schematic);
    let table_width = ui.available_width().max(500.0);
    // The flat row list and its offsets come from the plan: a retained DC
    // solution is one row per node, and a real block has tens of thousands.
    let flat = plan.node_rows.as_slice();
    let offsets = &plan.node_offsets;
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
            let view = offsets.plan(egui::Rangef::new(
                viewport.min.y - HEADER_H,
                viewport.max.y - HEADER_H,
            ));
            ui.allocate_space(egui::vec2(table_width, view.leading));
            for entry in &flat[view.range()] {
                let row = match *entry {
                    NodePlanRow::Group(scope) => {
                        let scope = &plan.node_scopes[scope];
                        group_header(ui, table_width, &scope.scope, scope.count);
                        continue;
                    }
                    NodePlanRow::Value(row) => &dc.node_voltages[row],
                };
                {
                    let (rect, response) = ui
                        .allocate_exact_size(egui::vec2(table_width, ROW_H), egui::Sense::hover());
                    let shown = notations.display(&row.name);
                    response.widget_info(|| {
                        let (value, valid) = node_value_text(row);
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Label,
                            true,
                            if valid {
                                format!("Node {shown}, value {value}")
                            } else {
                                format!("Node {shown}, invalid retained non-finite value")
                            },
                        )
                    });
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        let (value, valid) = node_value_text(row);
                        node.set_role(egui::accesskit::Role::Row);
                        node.set_label(if valid {
                            format!("Node {shown}; voltage {value}; identity {}", row.name)
                        } else {
                            format!(
                                "Node {shown}; voltage unavailable; retained value is non-finite; identity {}",
                                row.name
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
                    let (_, leaf) = hierarchy_parts(&row.name, root);
                    paint_cell(
                        ui,
                        op_column_rect(rect, 0.0, NAME_W),
                        notations.display(&leaf),
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
            ui.allocate_space(egui::vec2(table_width, view.trailing));
            })
            .response
        });
    ui.ctx().accesskit_node_builder(table.inner.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label("Operating-point node voltages");
    });
    count
}

fn show_device_card(
    ui: &mut Ui,
    plan: &OpPlan,
    evidence: &OpEvidence,
    filter: &str,
    sort: Option<&(String, bool)>,
    body_max_height: Option<f32>,
    clicked_sort: &mut Option<String>,
    action: &mut Option<OpAction>,
    state: &AppState,
) -> usize {
    if evidence.detail_policy == Some(OperatingPointDeviceDetailEvidence::None) {
        unavailable_card(
            ui,
            "Device operating points",
            "The executed save policy explicitly retained no per-device operating-point detail.",
        );
        return 0;
    }
    let Some(report) = state
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.device_op.as_ref())
    else {
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
    let count = plan.device_shown;
    card_header(ui, "Device operating points", count, "shown");
    if plan.device_scopes.is_empty() {
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
    // block is one row per device; grouping, sorting and measuring them all
    // every frame is what the plan exists to stop.
    let layouts = plan.device_scopes.as_slice();
    let flat = plan.device_rows.as_slice();
    let offsets = &plan.device_offsets;
    let mut scroll = egui::ScrollArea::both()
        .id_salt("rspice.results.op.devices")
        .auto_shrink([false, false]);
    if let Some(max_height) = body_max_height {
        scroll = scroll.max_height(max_height);
    }
    let table = scroll.show_viewport(ui, |ui, viewport| {
        ui.scope(|ui| {
            let available = ui.available_width();
            let width_of = |layout: &OpScope| {
                (NAME_W + KIND_W + REGION_W + layout.columns.len() as f32 * VALUE_MIN_W + ACTION_W)
                    .max(available)
            };
            let widest = layouts.iter().map(width_of).fold(available, f32::max);
            ui.set_min_width(widest);
            let view = offsets.plan(viewport.y_range());
            ui.allocate_space(egui::vec2(widest, view.leading));
            for row in &flat[view.range()] {
                let layout = &layouts[row.scope()];
                let table_width = width_of(layout);
                let entry = match *row {
                    DevicePlanRow::Group(_) => {
                        group_header(ui, table_width, &layout.scope, layout.count);
                        continue;
                    }
                    DevicePlanRow::ColumnHeader(_) => {
                        if let Some(key) = device_column_header(
                            ui,
                            table_width,
                            &layout.columns,
                            &layout.scope,
                            sort,
                        ) {
                            *clicked_sort = Some(key);
                        }
                        continue;
                    }
                    DevicePlanRow::Gap(_) => {
                        ui.add_space(DEVICE_GROUP_GAP);
                        continue;
                    }
                    DevicePlanRow::Device { entry, .. } => &report.entries[entry],
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
                    let (_, leaf) = hierarchy_parts(&entry.name, &plan.key.root);
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
            ui.allocate_space(egui::vec2(widest, view.trailing));
        })
        .response
    });
    ui.ctx().accesskit_node_builder(table.inner.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label("Device operating points");
    });
    count
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
    let retains_values = state
        .simulation
        .active_analysis()
        .is_some_and(|analysis| analysis.dc_op.is_some() || analysis.device_op.is_some());
    if !retains_values {
        well_hint(
            ui,
            "The selected operating-point analysis retained no node, branch, or device values.",
        );
        return;
    }

    show_solve_strip(ui, &evidence);
    ui.add_space(1.0);

    let Some(analysis_key) = state.simulation.active_run().and_then(|run| {
        state
            .simulation
            .active_analysis()
            .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
    }) else {
        return;
    };
    // Both cards read one plan, so the retained rows are grouped, sorted and
    // measured once per (dataset, filter, sort) rather than twice per frame.
    let Some(plan) = op_plan(state, analysis_key) else {
        return;
    };
    let filter = plan.key.filter.clone();
    let sort = plan.key.sort.clone();
    // The occurrence a root-scoped row belongs to is the cell the active
    // configuration runs, or the project's top cell when none is active.
    let root = plan.key.root.clone();
    let mut clicked_sort = None;
    let mut action = None;
    let available = ui.available_size();
    if available.x >= TWO_CARD_BREAKPOINT {
        ui.columns(2, |columns| {
            columns[0].set_min_height(available.y);
            columns[1].set_min_height(available.y);
            show_node_card(
                &mut columns[0],
                &plan,
                &filter,
                &root,
                None,
                &mut action,
                state,
            );
            show_device_card(
                &mut columns[1],
                &plan,
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
            &plan,
            &filter,
            &root,
            Some(stacked_body_height),
            &mut action,
            state,
        );
        ui.add_space(1.0);
        show_device_card(
            ui,
            &plan,
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
    let analysis_key = state.simulation.active_run().and_then(|run| {
        state
            .simulation
            .active_analysis()
            .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
    });

    section_header(ui, "OP result", None);
    let run = format!("Run {}", evidence.run_id);
    let node_count = evidence.node_count.to_string();
    let branch_count = evidence.branch_count.to_string();
    // The in-scope device count is a property of the retained detail policy,
    // so it comes off the plan rather than re-filtering every device row.
    let device_count = analysis_key
        .and_then(|key| op_plan(state, key))
        .map_or(0, |plan| plan.device_in_scope)
        .to_string();
    measurement_table(
        ui,
        &[
            ("Run", run.as_str()),
            ("Analysis", evidence.label.as_str()),
            ("Node values", node_count.as_str()),
            ("Branch values", branch_count.as_str()),
            ("Device rows", device_count.as_str()),
            ("Device scope", detail_label(evidence.detail_policy)),
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
mod tests {
    use super::*;

    #[test]
    fn op_inspector_groups_by_occurrence_not_by_the_word_top() {
        for (name, scope, leaf) in [
            ("V(out)", "amplifier", "out"),
            ("0", "amplifier", "0"),
            ("V(top.n2)", "amplifier", "n2"),
            ("V(x1.n)", "/x1", "n"),
            ("V(/X1/n)", "/x1", "n"),
            ("XAFE.M1", "/xafe", "M1"),
            ("v1#branch", "amplifier", "v1#branch"),
            ("XAFE/M1", "amplifier", "XAFE/M1"),
        ] {
            assert_eq!(
                hierarchy_parts(name, "amplifier"),
                (scope.to_owned(), leaf.to_owned()),
                "{name}"
            );
        }

        let dc = DcOpResult {
            node_voltages: ["V(out)", "V(top.n2)", "V(x1.n)", "V(/X1/n)"]
                .into_iter()
                .map(|name| crate::state::OperatingPointValue {
                    name: name.to_owned(),
                    value: 1.0,
                    unit: "V".to_owned(),
                })
                .collect(),
            ..DcOpResult::default()
        };
        let groups = grouped_nodes(&dc, "", "amplifier");
        assert_eq!(
            groups.keys().map(String::as_str).collect::<Vec<_>>(),
            ["/x1", "amplifier"],
            "the root group is the cell the run came from, never an invented literal"
        );
        assert_eq!(
            groups["amplifier"].len(),
            2,
            "the legacy root segment resolves to the same occurrence as no segment at all"
        );
        assert_eq!(
            groups["/x1"].len(),
            2,
            "an engine name and a canonical path name one occurrence"
        );
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
            name: "xafe.m1".to_owned(),
            device_kind: "MOSFET",
            region: Some("saturation"),
            params: vec![("gm", 1.2e-3)],
        };
        assert!(device_matches(&entry, "xafe", "amplifier"));
        assert!(device_matches(&entry, "mosfet", "amplifier"));
        assert!(device_matches(&entry, "satur", "amplifier"));
        assert!(device_matches(&entry, "gm", "amplifier"));
        assert!(!device_matches(&entry, "diode", "amplifier"));
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
        assert_eq!(evidence.node_count, 1);
    }

    #[test]
    fn node_evidence_does_not_depend_on_a_device_report() {
        use crate::state::{AnalysisResult, SimulationRun};

        let mut state = AppState::default();
        let mut run = SimulationRun::new(4);
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
        analysis.dc_op = Some(DcOpResult {
            node_voltages: vec![crate::state::OperatingPointValue {
                name: "V(top.out)".to_owned(),
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
        assert_eq!(evidence.node_count, 1);
        let dc = state.simulation.runs[0].analyses[0]
            .dc_op
            .as_ref()
            .expect("retained node voltages");
        assert!(state.simulation.runs[0].analyses[0].device_op.is_none());
        assert_eq!(grouped_nodes(dc, "out", "amplifier").len(), 1);
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

        let groups = grouped_nodes(&dc, "", "amplifier");
        assert_eq!(groups["amplifier"].len(), 1);
        let (display, valid) = node_value_text(&dc.node_voltages[groups["amplifier"][0].1]);
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

        let names_of = |groups: &BTreeMap<String, Vec<(String, usize)>>| {
            groups["amplifier"]
                .iter()
                .map(|(_, index)| report.entries[*index].name.clone())
                .collect::<Vec<_>>()
        };

        let descending = ("gm".to_owned(), false);
        let groups = grouped_devices(&report, None, "", Some(&descending), "amplifier");
        assert_eq!(
            names_of(&groups),
            vec!["M_negative", "M_high", "M_low", "M_missing"]
        );

        let ascending = ("gm".to_owned(), true);
        let groups = grouped_devices(&report, None, "", Some(&ascending), "amplifier");
        assert_eq!(
            names_of(&groups),
            vec!["M_low", "M_high", "M_negative", "M_missing"]
        );
    }

    /// A retained operating point with `nodes` nets and `devices` devices,
    /// spread over four occurrences.
    fn op_state(nodes: usize, devices: usize) -> AppState {
        use crate::state::{AnalysisResult, SimulationRun};

        let dc = DcOpResult {
            node_voltages: (0..nodes)
                .map(|index| crate::state::OperatingPointValue {
                    name: format!("V(x{}.n{index})", index % 4),
                    value: index as f64,
                    unit: "V".to_owned(),
                })
                .collect(),
            ..DcOpResult::default()
        };
        let report = rspice_core::circuit::DeviceOpReport {
            entries: (0..devices)
                .map(|index| rspice_core::circuit::DeviceOpEntry {
                    name: format!("x{}.m{index}", index % 4),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: vec![("gm", index as f64 * 1.0e-3), ("id", 1.0e-6)],
                })
                .collect(),
        };
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
        analysis.dc_op = Some(dc);
        analysis.device_op = Some(report);
        let mut run = SimulationRun::new(3);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        state.simulation.active_analysis_idx = Some(0);
        state
    }

    fn active_key(state: &AppState) -> AnalysisPresentationKey {
        let run = state.simulation.active_run().expect("retained run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    }

    /// The plan replaced a per-frame grouping, so it has to lay the rows out
    /// in exactly the order that grouping produced.
    #[test]
    fn the_plan_orders_rows_the_way_the_grouping_it_replaced_did() {
        let mut state = op_state(40, 40);
        let key = active_key(&state);
        let plan = op_plan(&mut state, key).expect("a row plan for the active analysis");

        let analysis = &state.simulation.runs[0].analyses[0];
        let dc = analysis.dc_op.as_ref().expect("retained node voltages");
        let report = analysis.device_op.as_ref().expect("retained device rows");
        let root = state.workspace.simulation_root_reference().cell;

        let mut expected = Vec::new();
        for (scope, rows) in grouped_nodes(dc, "", &root) {
            expected.push(format!("group {scope}"));
            expected.extend(
                rows.into_iter()
                    .map(|(_, index)| dc.node_voltages[index].name.clone()),
            );
        }
        let actual: Vec<String> = plan
            .node_rows
            .iter()
            .map(|row| match *row {
                NodePlanRow::Group(scope) => format!("group {}", plan.node_scopes[scope].scope),
                NodePlanRow::Value(index) => dc.node_voltages[index].name.clone(),
            })
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(plan.node_shown, dc.node_voltages.len());

        let mut expected = Vec::new();
        for (scope, rows) in grouped_devices(report, None, "", None, &root) {
            expected.push(format!("group {scope}"));
            expected.push("columns".to_owned());
            expected.extend(
                rows.into_iter()
                    .map(|(_, index)| report.entries[index].name.clone()),
            );
            expected.push("gap".to_owned());
        }
        let actual: Vec<String> = plan
            .device_rows
            .iter()
            .map(|row| match *row {
                DevicePlanRow::Group(scope) => {
                    format!("group {}", plan.device_scopes[scope].scope)
                }
                DevicePlanRow::ColumnHeader(_) => "columns".to_owned(),
                DevicePlanRow::Gap(_) => "gap".to_owned(),
                DevicePlanRow::Device { entry, .. } => report.entries[entry].name.clone(),
            })
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(plan.device_shown, report.entries.len());
        assert_eq!(plan.device_in_scope, report.entries.len());

        // The offsets must describe the rows the plan actually holds, or the
        // scrollbar lies about how long the table is.
        assert_eq!(plan.node_offsets.rows(), plan.node_rows.len());
        assert_eq!(plan.device_offsets.rows(), plan.device_rows.len());
    }

    /// The controls are part of the key, so moving one has to rebuild.
    #[test]
    fn the_readers_controls_are_part_of_the_row_plan_key() {
        let mut state = op_state(40, 40);
        let key = active_key(&state);
        let unfiltered = op_plan(&mut state, key).expect("a row plan");
        assert!(Arc::ptr_eq(
            &unfiltered,
            &op_plan(&mut state, key).expect("a row plan")
        ));

        state.ui.results.op_filter = "m1".to_owned();
        let filtered = op_plan(&mut state, key).expect("a row plan");
        assert!(filtered.device_shown < unfiltered.device_shown);
        assert!(
            filtered.device_shown > 0,
            "the filter matched nothing, so it proves nothing"
        );

        state.ui.results.op_filter.clear();
        state.ui.results.op_sort = Some(("gm".to_owned(), true));
        let sorted = op_plan(&mut state, key).expect("a row plan");
        assert_ne!(
            sorted.device_rows, unfiltered.device_rows,
            "a new sort key served the previous ordering"
        );
    }

    /// The rows are indices into retained evidence, so a new generation of
    /// that evidence must not be read through the previous plan.
    #[test]
    fn a_new_dataset_generation_rebuilds_the_row_plan() {
        let mut state = op_state(8, 8);
        let key = active_key(&state);
        let before = op_plan(&mut state, key).expect("a row plan");
        assert_eq!(before.node_shown, 8);

        state.simulation.runs[0].analyses[0]
            .dc_op
            .as_mut()
            .expect("retained node voltages")
            .node_voltages
            .truncate(3);
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

        let after = op_plan(&mut state, key).expect("a row plan");
        assert_eq!(
            after.node_shown, 3,
            "the sheet would have indexed rows the new dataset no longer retains"
        );
        assert_eq!(after.node_offsets.rows(), after.node_rows.len());
    }
}
