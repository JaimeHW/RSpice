//! The operating-point sheet: node voltages, branch currents and each device
//! family's own quantities on one scrolling grid, and the exact readout of the
//! row the reader picked.

use crate::{
    operational::{FailureSiteControl, failure_site_hint, failure_site_label},
    presentation::well_hint,
    virtual_rows::RowOffsets,
};
use egui::{
    Ui,
    text::{LayoutJob, TextFormat},
};
use rspice_results::engine_evidence::{DeviceOpEntry, DeviceOpReport};
use rspice_results::operating_point::{
    DcOpResult, OperatingPointDeviceDetailEvidence, OperatingPointValue,
    report::{
        OperatingPointReportFacts, RetainedDeviceDetail as RetainedDetail, device_param_unit,
        process_label, retained_detail_allows, signal_leaf,
    },
};
use rspice_ui_kit::{
    panels::{WorkbenchIcon, section_block_gap, section_header},
    plot::{fmt_si_readout, fmt_si_significant},
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, PROBLEM_MARK_SIDE, Problem, measurement_table, paint_problem_mark},
};
use rspice_vocabulary::hierarchy_path::ProbeTarget;
use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

const ROW_H: f32 = 20.0;
const COLUMN_HEADER_H: f32 = 20.0;
const TITLE_H: f32 = 24.0;
const SECTION_GAP: f32 = 8.0;
/// Room below the last row, so a floating horizontal scrollbar never sits on it.
const SHEET_END_PAD: f32 = 12.0;
const NAME_MIN_W: f32 = 160.0;
const NAME_MAX_W: f32 = 320.0;
/// Where a name, a section title and a name column's header all start.
const NAME_INSET: f32 = 22.0;
const NAME_END_PAD: f32 = 8.0;
const VALUE_W: f32 = 104.0;
const CELL_INSET: f32 = 8.0;
/// The slot the prefix and unit start in; every mantissa ends just before it.
const UNIT_SLOT_W: f32 = 34.0;
const MANTISSA_GAP: f32 = 4.0;
const CHEVRON_CENTER: f32 = 11.0;
const CHEVRON_SIDE: f32 = 11.0;
const TITLE_COUNT_GAP: f32 = 7.0;
const PROBLEM_ROW_H: f32 = 24.0;
/// A retained value on the sheet reads to this many significant digits; the
/// inspector and the hover carry the rest.
const TABLE_SIGNIFICANT_DIGITS: usize = 6;
const READOUT_SIGNIFICANT_DIGITS: usize = 15;
const CELSIUS: &str = "\u{b0}C";
const REGION_HEADER: &str = "Region";
const DEVICES_TITLE: &str = "Device operating points";
const DEVICE_FAMILY_ORDER: [&str; 3] = ["MOSFET", "BJT", "DIODE"];
const NON_FINITE: &str = "non-finite";
const NODE_NOT_ON_SCHEMATIC: &str =
    "This retained node has no current, unambiguous schematic mapping.";
const DEVICE_NOT_ON_SCHEMATIC: &str =
    "This retained device has no current, exact schematic identity.";
const SCHEMATIC_CHANGED: &str = "Schematic changed since this run; cross-probing is off.";

#[derive(Clone)]
pub struct OpEvidence {
    pub success: bool,
    pub error: Option<String>,
    pub detail_policy: Option<OperatingPointDeviceDetailEvidence>,
    pub facts: Option<OperatingPointReportFacts>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpAction {
    LocateNode(String),
    LocateDevice(u64),
    /// A cross-probe the schematic cannot answer, and why.
    Refused(&'static str),
}

/// Read-only schematic queries for visible rows; the host applies returned actions.
pub trait SchematicMapping {
    fn display_node<'a>(&self, name: &'a str) -> Cow<'a, str>;
    fn node_available(&self, name: &str) -> bool;
    fn device_target(&self, name: &str) -> Option<u64>;
}

/// Each section's sort: section title to (column, ascending).
pub type OpSort = BTreeMap<String, (String, bool)>;

#[derive(Clone, Copy)]
pub struct OpControls<'a> {
    pub filter: &'a str,
    pub sort: &'a OpSort,
    pub collapsed: &'a BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpRowKind {
    Node,
    Branch,
    Device,
}

/// The row the reader picked: where it sat, and what it was called so a later
/// solve can find it again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpSelection {
    pub kind: OpRowKind,
    pub index: usize,
    pub name: String,
}

impl OpSelection {
    /// The selection against `dc` and `devices`: itself, moved if the row now
    /// sits elsewhere, or `None` if the row is gone. Constant time unless the
    /// data changed under it.
    pub fn resolved(
        &self,
        dc: Option<&DcOpResult>,
        devices: Option<&DeviceOpReport>,
    ) -> Option<Self> {
        if RowSource::of(self.kind, self.index, dc, devices)
            .is_some_and(|row| row.name() == self.name)
        {
            return Some(self.clone());
        }
        let found = match self.kind {
            OpRowKind::Node => position(dc?.node_voltages.iter().map(|row| &row.name), &self.name),
            OpRowKind::Branch => {
                position(dc?.branch_currents.iter().map(|row| &row.name), &self.name)
            }
            OpRowKind::Device => {
                position(devices?.entries.iter().map(|entry| &entry.name), &self.name)
            }
        }?;
        Some(Self {
            index: found,
            ..self.clone()
        })
    }
}

fn position<'a>(mut names: impl Iterator<Item = &'a String>, wanted: &str) -> Option<usize> {
    names.position(|name| name == wanted)
}

pub struct OpView<'a, M> {
    pub evidence: &'a OpEvidence,
    pub dc: Option<&'a DcOpResult>,
    pub devices: Option<&'a DeviceOpReport>,
    pub selection: Option<&'a OpSelection>,
    pub mapping: &'a M,
    /// The failed solve's offer to mark what it named on the drawing.
    pub failure_site: Option<FailureSiteControl<'a>>,
}

#[derive(Default)]
pub struct OpResponse {
    pub action: Option<OpAction>,
    /// The section and column whose header was clicked.
    pub clicked_sort: Option<(String, String)>,
    /// A row was picked, or the sheet's empty space was clicked (`Some(None)`).
    pub selected: Option<Option<OpSelection>>,
    pub toggled_section: Option<String>,
    pub highlight_failure_sites: bool,
}

pub enum OpAbsence {
    Missing,
    OtherAnalysis,
    NoValues,
}

pub fn show_absent(ui: &mut Ui, absence: OpAbsence) {
    well_hint(
        ui,
        match absence {
            OpAbsence::Missing => "No operating-point analysis is selected.",
            OpAbsence::OtherAnalysis => "The selected analysis is not a DC operating-point result.",
            OpAbsence::NoValues => {
                "The selected operating-point analysis retained no node, branch, or device values."
            }
        },
    );
}

/// Why the device table is short, when it is.
///
/// A full save is the normal state and says nothing. A reduced one explains
/// rows the reader expected and cannot find, which is the only case worth a
/// line of the panel.
fn reduced_scope_warning(
    policy: Option<OperatingPointDeviceDetailEvidence>,
) -> Option<&'static str> {
    match policy {
        None | Some(OperatingPointDeviceDetailEvidence::AllDevices) => None,
        Some(OperatingPointDeviceDetailEvidence::SelectedAndViolations) => {
            Some("Only selected and violating devices were saved.")
        }
        Some(OperatingPointDeviceDetailEvidence::ViolationsOnly) => {
            Some("Only violating devices were saved.")
        }
        Some(OperatingPointDeviceDetailEvidence::None) => {
            Some("No per-device detail was saved for this run.")
        }
    }
}

/// One device's family and quantities, with each quantity's unit. The only
/// place this page reads them from a report entry.
fn device_reading(entry: &DeviceOpEntry) -> (&str, impl Iterator<Item = (&str, f64, &str)> + '_) {
    let family = entry.device_kind;
    (
        family,
        entry
            .params
            .iter()
            .map(move |(name, value)| (*name, *value, device_param_unit(family, name))),
    )
}

/// A retained name as the reader reads it: the enclosing path, ending in its
/// separator, and the leaf. The deck's wrapper and the design root are gone.
fn name_parts(name: &str) -> (String, String) {
    let signal = signal_leaf(name);
    let signal = signal.strip_suffix("#branch").unwrap_or(signal);
    match ProbeTarget::parse_legacy(signal) {
        Ok(target) if !target.scope.is_root() => {
            let mut parent = target.scope.segments().join(".");
            parent.push('.');
            (parent, target.leaf)
        }
        Ok(target) => (String::new(), target.leaf),
        Err(_) => (String::new(), signal.to_owned()),
    }
}

/// Name order that reads part numbers the way a designer counts: `M2` before
/// `M10`.
fn natural_cmp(left: &str, right: &str) -> Ordering {
    let mut a = left.chars().peekable();
    let mut b = right.chars().peekable();
    loop {
        match (a.peek().copied(), b.peek().copied()) {
            (None, None) => return left.cmp(right),
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) if x.is_ascii_digit() && y.is_ascii_digit() => {
                let run = |chars: &mut std::iter::Peekable<std::str::Chars<'_>>| {
                    let mut digits = String::new();
                    while let Some(digit) = chars.next_if(char::is_ascii_digit) {
                        digits.push(digit);
                    }
                    digits
                };
                let (x, y) = (run(&mut a), run(&mut b));
                let (x, y) = (x.trim_start_matches('0'), y.trim_start_matches('0'));
                let order = x.len().cmp(&y.len()).then_with(|| x.cmp(y));
                if order != Ordering::Equal {
                    return order;
                }
            }
            (Some(x), Some(y)) => {
                let order = x.to_ascii_lowercase().cmp(&y.to_ascii_lowercase());
                if order != Ordering::Equal {
                    return order;
                }
                a.next();
                b.next();
            }
        }
    }
}

/// Present values in the reader's direction, missing and non-finite ones
/// below them in both.
fn present_order<T>(
    left: Option<T>,
    right: Option<T>,
    ascending: bool,
    compare: impl Fn(&T, &T) -> Ordering,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => {
            let order = compare(&left, &right);
            if ascending { order } else { order.reverse() }
        }
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn magnitude_order(left: &f64, right: &f64) -> Ordering {
    left.abs()
        .total_cmp(&right.abs())
        .then_with(|| left.total_cmp(right))
}

/// A row the filter let through, with what ordering it needs.
struct ShownRow {
    index: usize,
    display: String,
    hierarchical: bool,
}

impl ShownRow {
    fn matching(index: usize, name: &str, filter: &str) -> Option<Self> {
        let (parent, leaf) = name_parts(name);
        let hierarchical = !parent.is_empty();
        let display = parent + &leaf;
        (filter.is_empty()
            || display.to_lowercase().contains(filter)
            || name.to_lowercase().contains(filter))
        .then_some(Self {
            index,
            display,
            hierarchical,
        })
    }
}

/// Top-level names first, then natural order, then the reader's sort if the
/// section has one; a sort's ties keep the natural order.
fn order_rows(
    rows: &mut [ShownRow],
    sort: Option<&(String, bool)>,
    name_header: &str,
    region: impl Fn(usize) -> Option<&'static str>,
    quantity: impl Fn(usize, &str) -> Option<f64>,
) {
    rows.sort_by(|left, right| {
        left.hierarchical
            .cmp(&right.hierarchical)
            .then_with(|| natural_cmp(&left.display, &right.display))
    });
    let Some((column, ascending)) = sort else {
        return;
    };
    if column == name_header {
        if !ascending {
            rows.reverse();
        }
    } else if column == REGION_HEADER {
        rows.sort_by(|left, right| {
            present_order(
                region(left.index),
                region(right.index),
                *ascending,
                Ord::cmp,
            )
        });
    } else {
        rows.sort_by(|left, right| {
            present_order(
                quantity(left.index, column).filter(|value| value.is_finite()),
                quantity(right.index, column).filter(|value| value.is_finite()),
                *ascending,
                magnitude_order,
            )
        });
    }
}

/// A family's own quantities, in the order the report introduced them.
/// The quantities a designer opens this page for, in the order they are
/// read. A family not listed, a Verilog-A module among them, stays as it
/// reports.
fn leading_quantities(family: &str) -> &'static [&'static str] {
    const MOSFET: &[&str] = &[
        "id", "vgs", "vds", "vbs", "vth", "vdsat", "gm", "gds", "gmb", "gmbs", "ig", "is", "ib",
    ];
    const BJT: &[&str] = &[
        "ic", "ib", "vbe", "vce", "gm", "gpi", "gmu", "go", "ie", "is",
    ];
    match family {
        "MOSFET" | "MOS9" | "BSIM3" | "BSIM4" | "B3SOIFD" | "B3SOIDD" | "B3SOIPD" | "EKV26"
        | "EKV3" | "VDMOS" => MOSFET,
        "BJT" => BJT,
        _ => &[],
    }
}

/// A quantity's place in its family: the listed ones first and in that order,
/// the rest level with each other so a stable sort leaves them in report order.
fn quantity_rank(family: &str, name: &str) -> usize {
    let leading = leading_quantities(family);
    leading
        .iter()
        .position(|known| *known == name)
        .unwrap_or(leading.len())
}

fn family_columns(report: &DeviceOpReport, rows: &[ShownRow]) -> Vec<PlanColumn> {
    let mut in_report_order: Vec<usize> = rows.iter().map(|row| row.index).collect();
    in_report_order.sort_unstable();
    let mut columns: Vec<PlanColumn> = Vec::new();
    for index in in_report_order {
        let (_, quantities) = device_reading(&report.entries[index]);
        for (name, _, unit) in quantities {
            if !columns.iter().any(|column| column.label == name) {
                columns.push(PlanColumn {
                    label: name.to_owned(),
                    unit: unit.to_owned(),
                });
            }
        }
    }
    if let Some(row) = rows.first() {
        let (family, _) = device_reading(&report.entries[row.index]);
        columns.sort_by_key(|column| quantity_rank(family, &column.label));
    }
    columns
}

/// The section title of a family: the engine's capitals where they are only
/// shouting, a Verilog-A module's name exactly as its author wrote it.
fn family_title(family: &str) -> String {
    match family {
        "DIODE" => "Diode",
        "RESISTOR" => "Resistor",
        "CAPACITOR" => "Capacitor",
        "NONLINEAR_CORE" => "Nonlinear core",
        other => other,
    }
    .to_owned()
}

fn family_rank(family: &str) -> usize {
    DEVICE_FAMILY_ORDER
        .iter()
        .position(|known| *known == family)
        .unwrap_or(DEVICE_FAMILY_ORDER.len())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanColumn {
    label: String,
    unit: String,
}

#[derive(Debug, Clone)]
struct PlanSection {
    kind: OpRowKind,
    title: String,
    name_header: String,
    columns: Vec<PlanColumn>,
    region: bool,
    count: usize,
    sort: Option<(String, bool)>,
    collapsed: bool,
    /// Why the section holds no rows; it stands in for them.
    note: Option<String>,
    /// Sheet offset of the title row.
    top: f32,
    /// Sheet offset just past the section's last row.
    bottom: f32,
}

impl PlanSection {
    /// Value columns on the sheet's grid, counting the region's.
    fn slots(&self) -> usize {
        self.columns.len() + usize::from(self.region)
    }
    fn has_rows(&self) -> bool {
        !self.collapsed && self.note.is_none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlanRow {
    Gap,
    Title(usize),
    Columns(usize),
    Note(usize),
    Value { section: usize, index: usize },
}

impl PlanRow {
    const fn height(self) -> f32 {
        match self {
            Self::Gap => SECTION_GAP,
            Self::Title(_) => TITLE_H,
            Self::Columns(_) => COLUMN_HEADER_H,
            Self::Note(_) | Self::Value { .. } => ROW_H,
        }
    }
}

/// What a node or branch section says when it has nothing to list.
struct ScalarSection {
    kind: OpRowKind,
    title: &'static str,
    name_header: &'static str,
    value_header: &'static str,
    not_retained: &'static str,
    empty: &'static str,
}

const NODES: ScalarSection = ScalarSection {
    kind: OpRowKind::Node,
    title: "Node voltages",
    name_header: "Node",
    value_header: "Voltage",
    not_retained: "Node-voltage evidence was not retained for this analysis.",
    empty: "The retained DC solution contains no node-voltage rows.",
};

const BRANCHES: ScalarSection = ScalarSection {
    kind: OpRowKind::Branch,
    title: "Branch currents",
    name_header: "Branch",
    value_header: "Current",
    not_retained: "Branch-current evidence was not retained for this analysis.",
    empty: "The retained DC solution contains no branch-current rows.",
};

struct PlanBuilder<'a> {
    controls: OpControls<'a>,
    filter: String,
    sections: Vec<PlanSection>,
    rows: Vec<PlanRow>,
    y: f32,
    longest_name: usize,
}

impl PlanBuilder<'_> {
    fn push(&mut self, row: PlanRow) {
        self.y += row.height();
        self.rows.push(row);
    }

    fn section(
        &self,
        kind: OpRowKind,
        title: &str,
        name_header: &str,
        columns: Vec<PlanColumn>,
    ) -> PlanSection {
        PlanSection {
            kind,
            title: title.to_owned(),
            name_header: name_header.to_owned(),
            columns,
            region: false,
            count: 0,
            sort: self.controls.sort.get(title).cloned(),
            collapsed: self.controls.collapsed.contains(title),
            note: None,
            top: 0.0,
            bottom: 0.0,
        }
    }

    fn add(&mut self, mut section: PlanSection, shown: &[ShownRow]) {
        if !self.sections.is_empty() {
            self.push(PlanRow::Gap);
        }
        let index = self.sections.len();
        section.top = self.y;
        section.count = shown.len();
        self.longest_name = shown
            .iter()
            .map(|row| row.display.chars().count())
            .fold(self.longest_name, usize::max);
        self.push(PlanRow::Title(index));
        if !section.collapsed {
            if section.note.is_some() {
                self.push(PlanRow::Note(index));
            } else {
                self.push(PlanRow::Columns(index));
                for row in shown {
                    self.push(PlanRow::Value {
                        section: index,
                        index: row.index,
                    });
                }
            }
        }
        section.bottom = self.y;
        self.sections.push(section);
    }

    /// A section with a message in place of rows. A filtered sheet lists only
    /// what matched, so it leaves such sections out.
    fn add_note(&mut self, kind: OpRowKind, title: &str, note: &str) {
        if self.filter.is_empty() {
            let mut section = self.section(kind, title, "", Vec::new());
            section.note = Some(note.to_owned());
            self.add(section, &[]);
        }
    }

    fn scalars(&mut self, spec: &ScalarSection, values: Option<&[OperatingPointValue]>) -> usize {
        let Some(values) = values else {
            self.add_note(spec.kind, spec.title, spec.not_retained);
            return 0;
        };
        if values.is_empty() {
            self.add_note(spec.kind, spec.title, spec.empty);
            return 0;
        }
        let mut shown: Vec<ShownRow> = values
            .iter()
            .enumerate()
            .filter_map(|(index, row)| ShownRow::matching(index, &row.name, &self.filter))
            .collect();
        if shown.is_empty() {
            return 0;
        }
        let column = PlanColumn {
            label: spec.value_header.to_owned(),
            unit: String::new(),
        };
        let section = self.section(spec.kind, spec.title, spec.name_header, vec![column]);
        order_rows(
            &mut shown,
            section.sort.as_ref(),
            spec.name_header,
            |_| None,
            |index, _| Some(values[index].value),
        );
        self.add(section, &shown);
        shown.len()
    }

    fn devices(
        &mut self,
        devices: Option<&DeviceOpReport>,
        detail: Option<&RetainedDetail>,
    ) -> usize {
        let mut families = BTreeMap::<(usize, String, String), Vec<ShownRow>>::new();
        if let Some(report) = devices {
            for (index, entry) in report
                .entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| retained_detail_allows(&entry.name, detail))
            {
                if let Some(row) = ShownRow::matching(index, &entry.name, &self.filter) {
                    let (family, _) = device_reading(entry);
                    let title = family_title(family);
                    let key = (family_rank(family), title.to_lowercase(), title);
                    families.entry(key).or_default().push(row);
                }
            }
        }
        let Some(report) = devices.filter(|_| !families.is_empty()) else {
            let note = if detail
                .is_some_and(|detail| detail.policy == OperatingPointDeviceDetailEvidence::None)
            {
                "The executed save policy explicitly retained no per-device operating-point detail."
            } else if devices.is_none() {
                "Per-device operating-point quantities were not retained for this analysis."
            } else if devices.is_some_and(|report| report.entries.is_empty()) {
                "The retained per-device report contains no device rows."
            } else {
                "No device row satisfies the retained device-detail scope."
            };
            self.add_note(OpRowKind::Device, DEVICES_TITLE, note);
            return 0;
        };
        let mut device_shown = 0;
        for ((_, _, title), mut shown) in families {
            let columns = family_columns(report, &shown);
            let mut section = self.section(OpRowKind::Device, &title, "Instance", columns);
            section.region = shown
                .iter()
                .any(|row| report.entries[row.index].region.is_some());
            order_rows(
                &mut shown,
                section.sort.as_ref(),
                "Instance",
                |index| report.entries[index].region,
                |index, column| {
                    let (_, mut quantities) = device_reading(&report.entries[index]);
                    quantities
                        .find(|(name, _, _)| *name == column)
                        .map(|(_, value, _)| value)
                },
            );
            device_shown += shown.len();
            self.add(section, &shown);
        }
        device_shown
    }
}

/// The sheet's rows and layout, computed once per dataset, filter, sort and
/// collapse so a frame only ever reads the rows it can show.
#[derive(Debug, Clone)]
pub struct OpPlan {
    sections: Vec<PlanSection>,
    rows: Vec<PlanRow>,
    offsets: RowOffsets,
    longest_name: usize,
    widest_slots: usize,
    node_shown: usize,
    device_shown: usize,
    /// The one line a filter that matched nothing leaves on the sheet.
    no_match: Option<String>,
}

impl OpPlan {
    pub fn new(
        controls: OpControls<'_>,
        dc: Option<&DcOpResult>,
        devices: Option<&DeviceOpReport>,
        detail: Option<&RetainedDetail>,
    ) -> OpPlan {
        let mut builder = PlanBuilder {
            controls,
            filter: controls.filter.trim().to_lowercase(),
            sections: Vec::new(),
            rows: Vec::new(),
            y: 0.0,
            longest_name: 0,
        };
        let node_shown = builder.scalars(&NODES, dc.map(|dc| dc.node_voltages.as_slice()));
        builder.scalars(&BRANCHES, dc.map(|dc| dc.branch_currents.as_slice()));
        let device_shown = builder.devices(devices, detail);
        let no_match = (builder.sections.is_empty() && !builder.filter.is_empty()).then(|| {
            format!(
                "No name matches \u{201c}{}\u{201d}.",
                controls.filter.trim()
            )
        });
        OpPlan {
            offsets: RowOffsets::from_heights(builder.rows.iter().map(|row| row.height())),
            widest_slots: builder
                .sections
                .iter()
                .map(PlanSection::slots)
                .max()
                .unwrap_or(0),
            longest_name: builder.longest_name,
            sections: builder.sections,
            rows: builder.rows,
            node_shown,
            device_shown,
            no_match,
        }
    }

    pub fn node_shown(&self) -> usize {
        self.node_shown
    }

    /// The section whose title and column header stay on the sheet's top edge
    /// at scroll offset `top`, and the offset they sit at: `top` until the
    /// section's end pushes them off.
    fn pinned_at(&self, top: f32) -> Option<(usize, f32)> {
        let index = self
            .sections
            .partition_point(|section| section.top <= top)
            .checked_sub(1)?;
        let section = &self.sections[index];
        (section.has_rows() && top > section.top)
            .then(|| (index, top.min(section.bottom - TITLE_H - COLUMN_HEADER_H)))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn device_shown(&self) -> usize {
        self.device_shown
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn node_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.indices(OpRowKind::Node)
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn device_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.indices(OpRowKind::Device)
    }
    #[cfg(any(test, feature = "test-support"))]
    fn indices(&self, kind: OpRowKind) -> impl Iterator<Item = usize> + '_ {
        self.rows.iter().filter_map(move |row| match *row {
            PlanRow::Value { section, index } if self.sections[section].kind == kind => Some(index),
            _ => None,
        })
    }
    /// Rows in the plan and rows its offsets describe; a mismatch is a
    /// scrollbar lying about the sheet's length.
    #[cfg(any(test, feature = "test-support"))]
    pub fn row_counts(&self) -> (usize, usize) {
        (self.rows.len(), self.offsets.rows())
    }
}

/// One retained row, node or branch or device.
#[derive(Clone, Copy)]
enum RowSource<'a> {
    Scalar(&'a OperatingPointValue),
    Device(&'a DeviceOpEntry),
}

/// What one cell of a row holds.
enum CellValue {
    Number { mantissa: String, unit: String },
    Refused,
    Missing,
}

impl<'a> RowSource<'a> {
    fn of(
        kind: OpRowKind,
        index: usize,
        dc: Option<&'a DcOpResult>,
        devices: Option<&'a DeviceOpReport>,
    ) -> Option<Self> {
        match kind {
            OpRowKind::Node => dc?.node_voltages.get(index).map(Self::Scalar),
            OpRowKind::Branch => dc?.branch_currents.get(index).map(Self::Scalar),
            OpRowKind::Device => devices?.entries.get(index).map(Self::Device),
        }
    }

    fn name(self) -> &'a str {
        match self {
            Self::Scalar(row) => &row.name,
            Self::Device(entry) => &entry.name,
        }
    }

    fn region(self) -> Option<&'static str> {
        match self {
            Self::Scalar(_) => None,
            Self::Device(entry) => entry.region,
        }
    }

    /// The value and unit under `column`, if the row reports one.
    fn quantity(self, column: &PlanColumn) -> Option<(f64, String)> {
        match self {
            Self::Scalar(row) => Some((row.value, row.unit.clone())),
            Self::Device(entry) => {
                let (_, mut quantities) = device_reading(entry);
                quantities
                    .find(|(name, _, _)| *name == column.label)
                    .map(|(_, value, _)| (value, column.unit.clone()))
            }
        }
    }

    fn cell(self, column: &PlanColumn) -> CellValue {
        match self.quantity(column) {
            Some((value, unit)) if value.is_finite() => {
                let (mantissa, unit) = value_parts(value, &unit);
                CellValue::Number { mantissa, unit }
            }
            Some(_) => CellValue::Refused,
            None => CellValue::Missing,
        }
    }

    /// The name a screen reader announces with every value on the row.
    fn spoken(self, section: &PlanSection) -> String {
        let values = section
            .columns
            .iter()
            .map(|column| match self.quantity(column) {
                Some((value, unit)) => {
                    format!("{} {}", column.label, quantity_readout(value, &unit))
                }
                None => format!("{} not reported", column.label),
            })
            .collect::<Vec<_>>()
            .join(", ");
        match self.region() {
            Some(region) => format!("{}; region {region}; {values}", self.name()),
            None => format!("{}; {values}", self.name()),
        }
    }
}

/// A finite value as a mantissa to `TABLE_SIGNIFICANT_DIGITS` and the prefix
/// and unit that belong after it.
fn value_parts(value: f64, unit: &str) -> (String, String) {
    if unit == CELSIUS {
        let digits = TABLE_SIGNIFICANT_DIGITS as i32;
        let decimals = if value == 0.0 {
            digits - 1
        } else {
            (digits - 1 - value.abs().log10().floor() as i32).clamp(0, digits)
        };
        return (
            format!("{:.*}", decimals as usize, value),
            CELSIUS.to_owned(),
        );
    }
    // Rounded first, so a value on the edge of a decade carries into the next
    // prefix at the same digit count instead of one digit longer.
    let rounded = format!("{value:.*e}", TABLE_SIGNIFICANT_DIGITS - 1)
        .parse()
        .unwrap_or(value);
    let text = fmt_si_significant(rounded, unit, TABLE_SIGNIFICANT_DIGITS);
    match text.split_once(' ') {
        Some((mantissa, suffix)) => (mantissa.to_owned(), suffix.to_owned()),
        None => (text, String::new()),
    }
}

/// A retained value at full precision: the inspector's and the hover's form.
fn quantity_readout(value: f64, unit: &str) -> String {
    if !value.is_finite() {
        return NON_FINITE.to_owned();
    }
    if unit == CELSIUS {
        let text = format!("{value:.12}");
        let text = text.trim_end_matches('0').trim_end_matches('.');
        return format!("{text} {CELSIUS}");
    }
    fmt_si_readout(value, unit, READOUT_SIGNIFICANT_DIGITS)
}

fn elide_middle(text: &str, max_chars: usize) -> String {
    let count = text.chars().count();
    if count <= max_chars || max_chars == 0 {
        return text.to_owned();
    }
    let tail = (max_chars - 1) / 2;
    let head = max_chars - 1 - tail;
    let mut out: String = text.chars().take(head).collect();
    out.push('\u{2026}');
    out.extend(text.chars().skip(count - tail));
    out
}

/// A name cut to `max_chars`: the leaf whole while it fits, the path giving
/// up its middle first, then the middle of the whole name.
fn elide_name(parent: &str, leaf: &str, max_chars: usize) -> (String, String) {
    let parent_chars = parent.chars().count();
    let leaf_chars = leaf.chars().count();
    if parent_chars + leaf_chars <= max_chars {
        return (parent.to_owned(), leaf.to_owned());
    }
    if leaf_chars + 2 <= max_chars {
        return (
            elide_middle(parent, max_chars - leaf_chars),
            leaf.to_owned(),
        );
    }
    let elided = elide_middle(&format!("{parent}{leaf}"), max_chars);
    // The head `elide_middle` kept, with its ellipsis.
    let head = max_chars - max_chars.saturating_sub(1) / 2;
    let split = if head > parent_chars {
        parent_chars
    } else {
        head
    };
    (
        elided.chars().take(split).collect(),
        elided.chars().skip(split).collect(),
    )
}

fn name_column_width(advance: f32, longest_name: usize) -> f32 {
    (longest_name as f32 * advance + NAME_INSET + NAME_END_PAD).clamp(NAME_MIN_W, NAME_MAX_W)
}

/// The sheet's coordinates: where its content sits, what the scroll area
/// shows of it, and the name column that stays put inside that.
#[derive(Clone, Copy)]
struct Grid {
    origin: egui::Pos2,
    visible: egui::Rect,
    name_w: f32,
    /// Advance of the mono font names are set in.
    advance: f32,
}

impl Grid {
    fn screen_y(&self, y: f32) -> f32 {
        self.origin.y + y
    }

    /// A row at sheet offset `y`, across the whole visible width.
    fn row(&self, y: f32, height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.visible.left(), self.screen_y(y)),
            egui::vec2(self.visible.width(), height),
        )
    }

    /// The name column: fixed however far the values scroll.
    fn name_cell(&self, y: f32, height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.visible.left(), self.screen_y(y)),
            egui::vec2(self.name_w, height),
        )
    }

    /// Where a section of `slots` value columns starts on screen. It scrolls
    /// no further than it overflows, so a section that fits stays whole and
    /// one narrower than the widest stops at its own last column; and a whole
    /// column at a time, so the frozen name column never cuts a value in two.
    fn values_left(&self, slots: usize) -> f32 {
        let scrolled = self.visible.left() - self.origin.x;
        let overflow = self.name_w + slots as f32 * VALUE_W - self.visible.width();
        let columns = (scrolled.min(overflow.max(0.0)) / VALUE_W).ceil();
        self.visible.left() + self.name_w - columns * VALUE_W
    }

    /// One slot of the shared value grid, in a section starting at `left`.
    fn value_cell(&self, left: f32, y: f32, height: f32, slot: usize) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(left + slot as f32 * VALUE_W, self.screen_y(y)),
            egui::vec2(VALUE_W, height),
        )
    }

    /// Everything right of the name column.
    fn values_clip(&self) -> egui::Rect {
        egui::Rect::from_min_max(
            egui::pos2(self.visible.left() + self.name_w, self.visible.top()),
            self.visible.max,
        )
    }

    fn max_name_chars(&self) -> usize {
        ((self.name_w - NAME_INSET - NAME_END_PAD) / self.advance).floor() as usize
    }

    /// The slot under a horizontal screen position, if it is a value's.
    fn slot_at(&self, left: f32, x: f32) -> Option<usize> {
        (x >= self.visible.left() + self.name_w).then(|| ((x - left) / VALUE_W).floor() as usize)
    }
}

fn mantissa_edge(cell: egui::Rect) -> f32 {
    cell.right() - UNIT_SLOT_W - MANTISSA_GAP
}

fn describe(ui: &Ui, response: &egui::Response, role: egui::accesskit::Role, label: &str) {
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label.to_owned())
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(role);
        node.set_label(label.to_owned());
    });
}

/// A sheet-relative header alignment.
#[derive(Clone, Copy)]
enum HeaderAlign {
    Name,
    Text,
    Number,
}

struct SheetPainter<'a, M> {
    ui: &'a Ui,
    plan: &'a OpPlan,
    source: &'a OpView<'a, M>,
    grid: Grid,
    /// Where value rows may paint: below any pinned header.
    body: egui::Rect,
    colors: rspice_ui_kit::palette::Palette,
}

impl<M: SchematicMapping> SheetPainter<'_, M> {
    fn title_row(&self, index: usize, y: f32, out: &mut OpResponse) {
        let section = &self.plan.sections[index];
        let rect = self.grid.row(y, TITLE_H);
        let response = self.ui.interact(
            rect,
            self.ui.id().with(("op-section", &section.title)),
            egui::Sense::click(),
        );
        let painter = self.ui.painter_at(rect);
        if response.hovered() {
            painter.rect_filled(rect, 0.0, self.colors.bg_hover);
        }
        let icon = if section.collapsed {
            WorkbenchIcon::ChevronRight
        } else {
            WorkbenchIcon::ChevronDown
        };
        icon.paint(
            &painter,
            egui::Rect::from_center_size(
                egui::pos2(rect.left() + CHEVRON_CENTER, rect.center().y),
                egui::Vec2::splat(CHEVRON_SIDE),
            ),
            self.colors.text_dim,
        );
        let title = painter.layout_no_wrap(
            section.title.clone(),
            theme::sans(tokens::FS_1, FontWeight::Medium),
            self.colors.text,
        );
        let title_at = egui::pos2(
            rect.left() + NAME_INSET,
            rect.center().y - title.size().y * 0.5,
        );
        let count_x = title_at.x + title.size().x + TITLE_COUNT_GAP;
        painter.galley(title_at, title, self.colors.text);
        if section.note.is_none() {
            painter.text(
                egui::pos2(count_x, rect.center().y),
                egui::Align2::LEFT_CENTER,
                section.count.to_string(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                self.colors.text_faint,
            );
        }
        describe(
            self.ui,
            &response,
            egui::accesskit::Role::Button,
            &format!(
                "{}, {} rows, {}",
                section.title,
                section.count,
                if section.collapsed {
                    "collapsed"
                } else {
                    "expanded"
                }
            ),
        );
        theme::paint_focus_ring(self.ui, &response, rect);
        if response.clicked() {
            out.toggled_section = Some(section.title.clone());
        }
    }

    fn columns_row(&self, index: usize, y: f32, out: &mut OpResponse) {
        let section = &self.plan.sections[index];
        let rect = self.grid.row(y, COLUMN_HEADER_H);
        self.ui.painter_at(rect).hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, self.colors.border),
        );
        let name = self.grid.name_cell(y, COLUMN_HEADER_H);
        self.header_cell(
            section,
            name,
            rect,
            &section.name_header,
            HeaderAlign::Name,
            out,
        );
        let values = self.grid.values_clip().intersect(rect);
        let left = self.grid.values_left(section.slots());
        let mut slot = 0;
        if section.region {
            let cell = self.grid.value_cell(left, y, COLUMN_HEADER_H, slot);
            self.header_cell(section, cell, values, REGION_HEADER, HeaderAlign::Text, out);
            slot += 1;
        }
        for column in &section.columns {
            let cell = self.grid.value_cell(left, y, COLUMN_HEADER_H, slot);
            self.header_cell(
                section,
                cell,
                values,
                &column.label,
                HeaderAlign::Number,
                out,
            );
            slot += 1;
        }
    }

    fn header_cell(
        &self,
        section: &PlanSection,
        cell: egui::Rect,
        clip: egui::Rect,
        label: &str,
        align: HeaderAlign,
        out: &mut OpResponse,
    ) {
        let shown = cell.intersect(clip);
        if !shown.is_positive() {
            return;
        }
        let response = self.ui.interact(
            shown,
            self.ui.id().with(("op-sort", &section.title, label)),
            egui::Sense::click(),
        );
        let sorted = section
            .sort
            .as_ref()
            .filter(|(column, _)| column.eq_ignore_ascii_case(label));
        let ascending = sorted.is_some_and(|(_, ascending)| *ascending);
        let painter = self.ui.painter_at(shown);
        if response.hovered() {
            painter.rect_filled(
                shown.shrink2(egui::vec2(1.0, 1.0)),
                0.0,
                self.colors.bg_hover,
            );
        }
        let font = theme::mono(tokens::FS_0, FontWeight::Regular);
        let measured =
            painter.layout_no_wrap(label.to_owned(), font.clone(), self.colors.text_faint);
        let room = match align {
            HeaderAlign::Number => cell.width() - MANTISSA_GAP,
            HeaderAlign::Name => cell.width() - NAME_INSET - NAME_END_PAD,
            HeaderAlign::Text => cell.width() - 2.0 * CELL_INSET,
        };
        let elided = measured.size().x > room;
        let galley = if elided {
            let advance = measured.size().x / label.chars().count().max(1) as f32;
            painter.layout_no_wrap(
                elide_middle(label, (room / advance).floor() as usize),
                font,
                self.colors.text_faint,
            )
        } else {
            measured
        };
        let width = galley.size().x;
        let left = match align {
            HeaderAlign::Name => cell.left() + NAME_INSET,
            HeaderAlign::Text => cell.left() + CELL_INSET,
            // A short label ends over its mantissas; a long one uses the
            // unit slot as well rather than clip.
            HeaderAlign::Number if width <= mantissa_edge(cell) - cell.left() - MANTISSA_GAP => {
                mantissa_edge(cell) - width
            }
            HeaderAlign::Number => cell.right() - MANTISSA_GAP - width,
        };
        let top = cell.center().y - galley.size().y * 0.5;
        painter.galley(egui::pos2(left, top), galley, self.colors.text_faint);
        if sorted.is_some() {
            let x = match align {
                HeaderAlign::Number => left - 7.0,
                HeaderAlign::Name | HeaderAlign::Text => left + width + 7.0,
            };
            let center = egui::pos2(x, cell.center().y);
            let direction = if ascending { -1.0 } else { 1.0 };
            painter.add(egui::Shape::convex_polygon(
                vec![
                    center + egui::vec2(0.0, direction * 3.0),
                    center + egui::vec2(-3.5, -direction * 2.5),
                    center + egui::vec2(3.5, -direction * 2.5),
                ],
                self.colors.accent,
                egui::Stroke::NONE,
            ));
        }
        let state = match sorted {
            Some(_) if ascending => "sorted ascending",
            Some(_) => "sorted descending",
            None => "not sorted",
        };
        let next = if sorted.is_some() && !ascending {
            "ascending"
        } else {
            "descending"
        };
        describe(
            self.ui,
            &response,
            egui::accesskit::Role::ColumnHeader,
            &format!(
                "{}: {label}; {state}; activate to sort {next}",
                section.title
            ),
        );
        theme::paint_focus_ring(self.ui, &response, shown.shrink(1.0));
        if response.clicked() {
            out.clicked_sort = Some((section.title.clone(), label.to_owned()));
        }
        if elided {
            response.on_hover_text(label);
        }
    }

    fn note_row(&self, index: usize, y: f32) {
        let section = &self.plan.sections[index];
        let Some(note) = section.note.as_deref() else {
            return;
        };
        let rect = self.grid.row(y, ROW_H);
        self.ui.painter_at(rect).text(
            egui::pos2(rect.left() + NAME_INSET, rect.center().y),
            egui::Align2::LEFT_CENTER,
            note,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            self.colors.text_faint,
        );
    }

    fn paint_name(
        &self,
        painter: &egui::Painter,
        cell: egui::Rect,
        row: RowSource<'_>,
        kind: OpRowKind,
    ) {
        let (parent, leaf) = name_parts(row.name());
        let leaf = if kind == OpRowKind::Node {
            self.source.mapping.display_node(&leaf)
        } else {
            Cow::Borrowed(leaf.as_str())
        };
        let (parent, leaf) = elide_name(&parent, &leaf, self.grid.max_name_chars());
        let font = theme::mono(tokens::FS_1, FontWeight::Regular);
        let mut job = LayoutJob::default();
        if !parent.is_empty() {
            job.append(
                &parent,
                0.0,
                TextFormat::simple(font.clone(), self.colors.text_faint),
            );
        }
        job.append(&leaf, 0.0, TextFormat::simple(font, self.colors.text));
        let galley = painter.layout_job(job);
        painter.galley(
            egui::pos2(
                cell.left() + NAME_INSET,
                cell.center().y - galley.size().y * 0.5,
            ),
            galley,
            self.colors.text,
        );
    }

    fn paint_value(&self, painter: &egui::Painter, cell: egui::Rect, value: CellValue) {
        let painter = painter.with_clip_rect(cell.intersect(painter.clip_rect()));
        let font = theme::mono(tokens::FS_1, FontWeight::Regular);
        match value {
            CellValue::Number { mantissa, unit } => {
                let edge = mantissa_edge(cell);
                painter.text(
                    egui::pos2(edge, cell.center().y),
                    egui::Align2::RIGHT_CENTER,
                    mantissa,
                    font,
                    self.colors.text,
                );
                if !unit.is_empty() {
                    painter.text(
                        egui::pos2(edge + MANTISSA_GAP, cell.center().y),
                        egui::Align2::LEFT_CENTER,
                        unit,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        self.colors.text_dim,
                    );
                }
            }
            CellValue::Refused => {
                paint_problem_mark(
                    &painter,
                    egui::Rect::from_center_size(
                        egui::pos2(
                            cell.left() + CELL_INSET + PROBLEM_MARK_SIDE * 0.5,
                            cell.center().y,
                        ),
                        egui::Vec2::splat(PROBLEM_MARK_SIDE),
                    ),
                    Problem::Failure,
                );
                painter.text(
                    egui::pos2(cell.right() - CELL_INSET, cell.center().y),
                    egui::Align2::RIGHT_CENTER,
                    NON_FINITE,
                    font,
                    self.colors.text_dim,
                );
            }
            CellValue::Missing => {
                painter.text(
                    egui::pos2(mantissa_edge(cell), cell.center().y),
                    egui::Align2::RIGHT_CENTER,
                    "\u{2014}",
                    font,
                    self.colors.text_faint,
                );
            }
        }
    }

    fn value_row(&self, section_index: usize, index: usize, y: f32, out: &mut OpResponse) {
        let section = &self.plan.sections[section_index];
        let Some(row) = RowSource::of(section.kind, index, self.source.dc, self.source.devices)
        else {
            return;
        };
        let rect = self.grid.row(y, ROW_H).intersect(self.body);
        if !rect.is_positive() {
            return;
        }
        let response = self.ui.interact(
            rect,
            self.ui.id().with(("op-row", section.kind as u8, index)),
            egui::Sense::click(),
        );
        let selected = self
            .source
            .selection
            .is_some_and(|picked| picked.kind == section.kind && picked.index == index);
        let painter = self.ui.painter_at(rect);
        if selected {
            painter.rect_filled(rect, 0.0, self.ui.visuals().selection.bg_fill);
        } else if response.hovered() {
            painter.rect_filled(rect, 0.0, self.colors.bg_hover);
        }
        let name_cell = self.grid.name_cell(y, ROW_H);
        self.paint_name(
            &painter.with_clip_rect(name_cell.intersect(rect)),
            name_cell,
            row,
            section.kind,
        );
        let values_clip = self.grid.values_clip().intersect(rect);
        let values = self.ui.painter_at(values_clip);
        let left = self.grid.values_left(section.slots());
        let mut slot = 0;
        if section.region {
            if let Some(region) = row.region() {
                let cell = self.grid.value_cell(left, y, ROW_H, slot);
                values.with_clip_rect(cell.intersect(values_clip)).text(
                    egui::pos2(cell.left() + CELL_INSET, cell.center().y),
                    egui::Align2::LEFT_CENTER,
                    region,
                    theme::sans(tokens::FS_1, FontWeight::Regular),
                    self.colors.text_dim,
                );
            }
            slot += 1;
        }
        for column in &section.columns {
            let cell = self.grid.value_cell(left, y, ROW_H, slot);
            slot += 1;
            if cell.intersects(values_clip) {
                self.paint_value(&values, cell, row.cell(column));
            }
        }
        describe(
            self.ui,
            &response,
            egui::accesskit::Role::Row,
            &row.spoken(section),
        );
        theme::paint_focus_ring(self.ui, &response, rect);
        if response.clicked() {
            out.selected = Some(Some(OpSelection {
                kind: section.kind,
                index,
                name: row.name().to_owned(),
            }));
        }
        if response.double_clicked() {
            out.action = Some(locate_action(section.kind, row.name(), self.source.mapping));
        }
        let pointer_x = response.hover_pos().map(|pos| pos.x);
        response.on_hover_ui(|ui| self.row_hover(ui, section, row, pointer_x));
    }

    /// The whole name, the deck's identity, and the value under the pointer
    /// at full precision.
    fn row_hover(&self, ui: &mut Ui, section: &PlanSection, row: RowSource<'_>, x: Option<f32>) {
        let (parent, leaf) = name_parts(row.name());
        let mono = |text: String, color| {
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                .color(color)
        };
        ui.label(mono(format!("{parent}{leaf}"), self.colors.text));
        ui.label(mono(row.name().to_owned(), self.colors.text_faint));
        let column = x
            .and_then(|x| self.grid.slot_at(self.grid.values_left(section.slots()), x))
            .and_then(|slot| slot.checked_sub(usize::from(section.region)))
            .and_then(|slot| section.columns.get(slot));
        if let Some((column, (value, unit))) =
            column.and_then(|column| Some((column, row.quantity(column)?)))
        {
            ui.label(mono(
                format!("{}  {}", column.label, quantity_readout(value, &unit)),
                self.colors.text,
            ));
        }
    }
}

/// What double-clicking or pressing the inspector's button on this row does.
fn locate_action(kind: OpRowKind, name: &str, mapping: &impl SchematicMapping) -> OpAction {
    match kind {
        OpRowKind::Node if mapping.node_available(name) => OpAction::LocateNode(name.to_owned()),
        OpRowKind::Node => OpAction::Refused(NODE_NOT_ON_SCHEMATIC),
        // A branch current is the current through the source that owns it.
        OpRowKind::Branch | OpRowKind::Device => {
            let device = signal_leaf(name);
            mapping
                .device_target(device.strip_suffix("#branch").unwrap_or(device))
                .map_or(
                    OpAction::Refused(DEVICE_NOT_ON_SCHEMATIC),
                    OpAction::LocateDevice,
                )
        }
    }
}

fn show_sheet<M: SchematicMapping>(
    ui: &mut Ui,
    plan: &OpPlan,
    source: &OpView<'_, M>,
    out: &mut OpResponse,
) {
    let t = Tokens::get(ui.ctx());
    let colors = t.color;
    let advance = ui
        .painter()
        .layout_no_wrap(
            "0".to_owned(),
            theme::mono(tokens::FS_1, FontWeight::Regular),
            colors.text,
        )
        .size()
        .x;
    let name_w = name_column_width(advance, plan.longest_name);
    let content_w = name_w + plan.widest_slots as f32 * VALUE_W;
    let content_h = plan.offsets.total_height() + SHEET_END_PAD;
    let table = egui::ScrollArea::both()
        .id_salt("rspice.results.op.sheet")
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, viewport| {
            let (rect, background) = ui.allocate_exact_size(
                egui::vec2(
                    content_w.max(viewport.width()),
                    content_h.max(viewport.height()),
                ),
                egui::Sense::click(),
            );
            let grid = Grid {
                origin: rect.min,
                visible: viewport.translate(rect.min.to_vec2()),
                name_w,
                advance,
            };
            let pinned = plan.pinned_at(viewport.min.y);
            let mut body = grid.visible;
            if let Some((_, y)) = pinned {
                body.min.y = grid.screen_y(y + TITLE_H + COLUMN_HEADER_H).max(body.min.y);
            }
            let painter = SheetPainter {
                ui: &*ui,
                plan,
                source,
                grid,
                body,
                colors,
            };
            let view = plan.offsets.plan(viewport.y_range());
            let mut y = view.leading;
            for row in &plan.rows[view.range()] {
                let pinned_here =
                    |section: usize| pinned.is_some_and(|(index, _)| index == section);
                match *row {
                    PlanRow::Gap => {}
                    PlanRow::Title(section) | PlanRow::Columns(section) if pinned_here(section) => {
                    }
                    PlanRow::Title(section) => painter.title_row(section, y, out),
                    PlanRow::Columns(section) => painter.columns_row(section, y, out),
                    PlanRow::Note(section) => painter.note_row(section, y),
                    PlanRow::Value { section, index } => painter.value_row(section, index, y, out),
                }
                y += row.height();
            }
            if let Some((section, y)) = pinned {
                painter.title_row(section, y, out);
                painter.columns_row(section, y + TITLE_H, out);
            }
            if background.clicked() {
                out.selected = Some(None);
            }
            background
        });
    describe(
        ui,
        &table.inner,
        egui::accesskit::Role::Table,
        "Operating point",
    );
}

fn faint_line(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ROW_H),
        egui::Sense::hover(),
    );
    ui.painter().text(
        egui::pos2(rect.left() + NAME_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        message,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text_faint,
    );
    describe(ui, &response, egui::accesskit::Role::Status, message);
}

/// A solve that did not finish: its mark, its words and its error on one
/// line, ending in the run's offer to mark what it named. Whether the offer
/// was taken.
fn problem_row(
    ui: &mut Ui,
    error: Option<&str>,
    failure_site: Option<&FailureSiteControl<'_>>,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PROBLEM_ROW_H),
        egui::Sense::hover(),
    );
    let mut text_right = rect.right() - NAME_END_PAD;
    let mut highlight = false;
    if let Some(offer) = failure_site {
        let mut end = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect.shrink2(egui::vec2(NAME_END_PAD, 0.0)))
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        let button = Button::new(&failure_site_label(offer))
            .compact()
            .show(&mut end);
        text_right = button.rect.left() - NAME_END_PAD;
        highlight = button.on_hover_text(failure_site_hint(offer)).clicked();
    }
    let painter = ui.painter_at(rect);
    paint_problem_mark(
        &painter,
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + CHEVRON_CENTER, rect.center().y),
            egui::Vec2::splat(PROBLEM_MARK_SIDE),
        ),
        Problem::Failure,
    );
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let title = painter.layout_no_wrap("Partial solution".to_owned(), font.clone(), t.color.text);
    let text_y = rect.center().y - title.size().y * 0.5;
    let error_x = rect.left() + NAME_INSET + title.size().x + TITLE_COUNT_GAP;
    painter.galley(
        egui::pos2(rect.left() + NAME_INSET, text_y),
        title,
        t.color.text,
    );
    if let Some(error) = error {
        let mut job = LayoutJob::simple_singleline(error.to_owned(), font, t.color.text_dim);
        job.wrap = egui::text::TextWrapping::truncate_at_width(text_right - error_x);
        painter.galley(
            egui::pos2(error_x, text_y),
            painter.layout_job(job),
            t.color.text_dim,
        );
        response.clone().on_hover_text(error);
    }
    describe(
        ui,
        &response,
        egui::accesskit::Role::Status,
        &format!("Partial solution. {}", error.unwrap_or_default()),
    );
    highlight
}

pub fn show(
    ui: &mut Ui,
    plan: Option<&OpPlan>,
    source: &OpView<'_, impl SchematicMapping>,
) -> OpResponse {
    let mut response = OpResponse::default();
    if !source.evidence.success {
        response.highlight_failure_sites = problem_row(
            ui,
            source.evidence.error.as_deref(),
            source.failure_site.as_ref(),
        );
    }
    let Some(plan) = plan else {
        return response;
    };
    if let Some(message) = plan.no_match.as_deref() {
        faint_line(ui, message);
        return response;
    }
    show_sheet(ui, plan, source, &mut response);
    response
}

/// Everything the inspector states about the picked row.
pub struct OpDetail {
    title: String,
    rows: Vec<(String, String)>,
    action: OpAction,
}

/// The inspector's account of `selection`, or `None` if its row is gone.
pub fn selection_detail(
    selection: &OpSelection,
    dc: Option<&DcOpResult>,
    devices: Option<&DeviceOpReport>,
    mapping: &impl SchematicMapping,
) -> Option<OpDetail> {
    let row = RowSource::of(selection.kind, selection.index, dc, devices)?;
    let (parent, leaf) = name_parts(row.name());
    let leaf = if selection.kind == OpRowKind::Node {
        mapping.display_node(&leaf).into_owned()
    } else {
        leaf
    };
    let mut rows = Vec::new();
    match row {
        RowSource::Scalar(value) => {
            let label = match selection.kind {
                OpRowKind::Node => NODES.value_header,
                _ => BRANCHES.value_header,
            };
            rows.push((label.to_owned(), quantity_readout(value.value, &value.unit)));
        }
        RowSource::Device(entry) => {
            let (family, quantities) = device_reading(entry);
            rows.push(("Family".to_owned(), family_title(family)));
            if let Some(region) = entry.region {
                rows.push((REGION_HEADER.to_owned(), region.to_owned()));
            }
            let mut quantities: Vec<_> = quantities.collect();
            quantities.sort_by_key(|(name, _, _)| quantity_rank(family, name));
            rows.extend(
                quantities
                    .into_iter()
                    .map(|(name, value, unit)| (name.to_owned(), quantity_readout(value, unit))),
            );
        }
    }
    rows.push(("Identity".to_owned(), row.name().to_owned()));
    Some(OpDetail {
        title: format!("{parent}{leaf}"),
        rows,
        action: locate_action(selection.kind, row.name(), mapping),
    })
}

pub fn right_panel(
    ui: &mut Ui,
    evidence: &OpEvidence,
    current: bool,
    detail: Option<&OpDetail>,
) -> Option<OpAction> {
    let mut action = None;
    if let Some(detail) = detail {
        section_header(ui, &detail.title, None);
        measurement_table(
            ui,
            &detail
                .rows
                .iter()
                .map(|(label, value)| (label.as_str(), value.as_str()))
                .collect::<Vec<_>>(),
        );
        section_block_gap(ui);
        let refused = match detail.action {
            OpAction::Refused(reason) => Some(reason),
            _ => None,
        };
        let button = Button::new("Show in schematic")
            .enabled(refused.is_none())
            .show(ui);
        if let Some(reason) = refused {
            button.on_disabled_hover_text(reason);
        } else if button.clicked() {
            action = Some(detail.action.clone());
        }
        if !current {
            crate::presentation::panel_note(ui, SCHEMATIC_CHANGED);
        }
    }

    // The dataset block above names the run and the analysis, the OP table on
    // the sheet is the node, branch and device rows counted, and the MNA
    // dimensions and the annotation setting are not the reader's.
    if let Some(facts) = &evidence.facts {
        section_header(ui, "Conditions", None);
        let temperature = fmt_si_readout(facts.temperature_celsius, CELSIUS, 4);
        let process = process_label(facts.process);
        let mut rows = vec![
            ("Process", process.to_owned()),
            ("Temperature", temperature),
        ];
        // A single-point solve has no sweep position to state.
        if facts.point_count > 1 {
            rows.push((
                "Sweep point",
                format!(
                    "{} of {}",
                    facts.point_index.saturating_add(1),
                    facts.point_count
                ),
            ));
        }
        measurement_table(
            ui,
            &rows
                .iter()
                .map(|(label, value)| (*label, value.as_str()))
                .collect::<Vec<_>>(),
        );
    }

    // A full save and a current mapping are the normal states; only the
    // reduced save and the stale mapping change what the sheet can show.
    if let Some(reduced) = reduced_scope_warning(evidence.detail_policy) {
        crate::presentation::panel_note(ui, reduced);
    }
    if !current && detail.is_none() {
        crate::presentation::panel_note(ui, SCHEMATIC_CHANGED);
    }
    action
}

pub(crate) fn op_column_rect(row: egui::Rect, offset: f32, width: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(row.left() + offset, row.top()),
        egui::vec2(width, row.height()),
    )
}

#[cfg(any(test, feature = "test-support"))]
pub fn evidence_fixture(nodes: usize, devices: usize) -> (DcOpResult, DeviceOpReport) {
    let dc = DcOpResult {
        node_voltages: (0..nodes)
            .map(|index| OperatingPointValue {
                name: format!("V(x{}.n{index})", index % 4),
                value: index as f64,
                unit: "V".to_owned(),
            })
            .collect(),
        branch_currents: (0..nodes.min(4))
            .map(|index| OperatingPointValue {
                name: format!("I(V{index})"),
                value: -1.0e-3 * index as f64,
                unit: "A".to_owned(),
            })
            .collect(),
        ..DcOpResult::default()
    };
    let report = DeviceOpReport {
        entries: (0..devices)
            .map(|index| DeviceOpEntry {
                name: format!("x{}.m{index}", index % 4),
                device_kind: "MOSFET",
                region: Some("saturation"),
                params: vec![("gm", index as f64 * 1.0e-3), ("id", 1.0e-6)],
                module: None,
            })
            .collect(),
    };
    (dc, report)
}

#[cfg(test)]
mod tests;
