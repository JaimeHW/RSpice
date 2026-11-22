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
    plot::{fmt_si_signed, fmt_si_significant},
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, PROBLEM_MARK_SIDE, Problem, measurement_table, paint_problem_mark},
};
use rspice_vocabulary::{hierarchy_path::ProbeTarget, natural_order::natural_cmp};
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
/// Rounding room when a column's edge meets the sheet's.
const WHOLE_COLUMN_TOLERANCE: f32 = 0.5;
const CELL_INSET: f32 = 8.0;
/// The slot the prefix and unit start in; every mantissa ends just before it.
const UNIT_SLOT_W: f32 = 34.0;
const MANTISSA_GAP: f32 = 4.0;
const CHEVRON_CENTER: f32 = 11.0;
const CHEVRON_SIDE: f32 = 11.0;
const TITLE_COUNT_GAP: f32 = 7.0;
const PROBLEM_ROW_H: f32 = 24.0;
/// The inspector reads a picked value to at least this many significant
/// digits, more than the sheet's, the last one a double's noise cannot reach.
const READOUT_SIGNIFICANT_DIGITS: usize = 9;
const CELSIUS: &str = "\u{b0}C";
const REGION_HEADER: &str = "Region";
const DEVICES_TITLE: &str = "Device operating points";
const DEVICE_FAMILY_ORDER: [&str; 3] = ["MOSFET", "BJT", "DIODE"];
/// After the known families and every other built-in one.
const MODULE_SECTION_RANK: usize = DEVICE_FAMILY_ORDER.len() + 1;
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
    /// A device or instance name, or an instance path, as the drawing
    /// places it; the result's spelling where the host has no drawing.
    fn display_element<'a>(&self, name: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(name)
    }
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
    /// The reader's significant digits, which every value on the sheet reads at.
    pub significant_digits: usize,
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
/// line of the panel: a save policy that happened to keep every device is not
/// that case, whatever it is called.
fn reduced_scope_note(
    devices: Option<&DeviceOpReport>,
    detail: Option<&RetainedDetail>,
) -> Option<&'static str> {
    let detail = detail?;
    let left_out = devices.is_some_and(|report| {
        report
            .entries
            .iter()
            .any(|entry| !retained_detail_allows(&entry.name, Some(detail)))
    });
    match detail.policy {
        OperatingPointDeviceDetailEvidence::AllDevices => None,
        OperatingPointDeviceDetailEvidence::SelectedAndViolations => {
            left_out.then_some("Only selected and violating devices were saved.")
        }
        OperatingPointDeviceDetailEvidence::ViolationsOnly => {
            left_out.then_some("Only violating devices were saved.")
        }
        OperatingPointDeviceDetailEvidence::None => {
            Some("No per-device detail was saved for this run.")
        }
    }
}

/// One device's family and quantities, with each quantity's unit: the one a
/// module states, which may be none, or the family table's. The only place this
/// page reads them from a report entry.
fn device_reading(entry: &DeviceOpEntry) -> (&str, impl Iterator<Item = (&str, f64, &str)> + '_) {
    let kind = entry.device_kind;
    (
        entry.family(),
        entry.quantities().map(move |(name, value, units)| {
            let unit = units.unwrap_or_else(|| device_param_unit(kind, name));
            (name, value, unit)
        }),
    )
}

/// A retained name as the reader reads it: the enclosing path, ending in its
/// separator, and the leaf. The deck's wrapper and the design root are gone.
fn name_parts(name: &str) -> (String, String) {
    let signal = signal_leaf(name);
    let signal = signal.strip_suffix("#branch").unwrap_or(signal);
    match ProbeTarget::parse(signal) {
        Ok(target) if !target.scope.is_root() => {
            let mut parent = target.scope.segments().join(".");
            parent.push('.');
            (parent, target.leaf)
        }
        Ok(target) => (String::new(), target.leaf),
        Err(_) => (String::new(), signal.to_owned()),
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

/// Whether `filter` starts a word of `label`, so `linear` finds a device in the
/// linear region and leaves a nonlinear core alone.
fn starts_a_word(label: &str, filter: &str) -> bool {
    let label = label.to_lowercase();
    label
        .match_indices(filter)
        .any(|(at, _)| !label[..at].ends_with(char::is_alphanumeric))
}

/// A row the filter let through, with what ordering it needs.
struct ShownRow {
    index: usize,
    display: String,
    hierarchical: bool,
}

impl ShownRow {
    /// The row, if the filter finds it by its path, its deck name, or one of
    /// the `also` words a device is known by. A quantity's name is on every
    /// row of its family, so it finds nothing worth finding and is not offered.
    fn matching(index: usize, name: &str, also: &[&str], filter: &str) -> Option<Self> {
        let (parent, leaf) = name_parts(name);
        let hierarchical = !parent.is_empty();
        let display = parent + &leaf;
        let found = |text: &str| text.to_lowercase().contains(filter);
        (filter.is_empty()
            || found(&display)
            || found(name)
            || also.iter().any(|word| starts_a_word(word, filter)))
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

/// The quantities a designer opens this page for, in the order they are
/// read. A family not listed stays as it reports.
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

/// The family whose standing order a device's quantities follow. A Verilog-A
/// module has none, whatever its author called it: it keeps the order it
/// reports.
fn ranking_family(entry: &DeviceOpEntry) -> Option<&str> {
    entry.module.is_none().then_some(entry.device_kind)
}

/// A quantity's place in its family: the listed ones first and in that order,
/// the rest level with each other so a stable sort leaves them in report order.
fn quantity_rank(family: Option<&str>, name: &str) -> usize {
    let leading = family.map_or(&[] as &[&str], leading_quantities);
    leading
        .iter()
        .position(|known| *known == name)
        .unwrap_or(leading.len())
}

/// A family's own quantities, in the order the report introduced them, the
/// family's leading ones first.
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
        let family = ranking_family(&report.entries[row.index]);
        columns.sort_by_key(|column| quantity_rank(family, &column.label));
    }
    columns
}

/// The section title of a built-in family: the engine's capitals where they
/// are only shouting.
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

/// Where a device's section sits among the others, and its title. A Verilog-A
/// module follows every built-in family, under exactly the name its author
/// wrote, even one that spells a family's.
fn device_section(entry: &DeviceOpEntry) -> (usize, String) {
    match &entry.module {
        Some(module) => (MODULE_SECTION_RANK, module.module.clone()),
        None => (
            family_rank(entry.device_kind),
            family_title(entry.device_kind),
        ),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanColumn {
    label: String,
    unit: String,
}

#[derive(Debug, Clone)]
struct PlanSection {
    kind: OpRowKind,
    /// What the reader's sort and fold are held under: the title, but a
    /// module's title can repeat a family's.
    key: String,
    title: String,
    name_header: String,
    columns: Vec<PlanColumn>,
    region: bool,
    count: usize,
    sort: Option<(String, bool)>,
    collapsed: bool,
    /// The most characters a value's prefix and unit can take in this section.
    longest_unit: usize,
    /// The most characters a value's number can take: its digits, a sign and
    /// a decimal point.
    longest_mantissa: usize,
    /// Why the section holds no rows; it stands in for them.
    note: Option<String>,
    /// Sheet offset of the title row.
    top: f32,
    /// Sheet offset just past the section's last row.
    bottom: f32,
}

impl PlanSection {
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
        key: &str,
        title: &str,
        name_header: &str,
        columns: Vec<PlanColumn>,
    ) -> PlanSection {
        // A value's prefix goes in front of its unit.
        let longest_unit = columns
            .iter()
            .map(|column| column.unit.chars().count() + 1)
            .max()
            .unwrap_or(0);
        PlanSection {
            kind,
            key: key.to_owned(),
            title: title.to_owned(),
            name_header: name_header.to_owned(),
            columns,
            region: false,
            count: 0,
            sort: self.controls.sort.get(key).cloned(),
            collapsed: self.controls.collapsed.contains(key),
            longest_unit,
            longest_mantissa: self.controls.significant_digits + 2,
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
            let mut section = self.section(kind, title, title, "", Vec::new());
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
            .filter_map(|(index, row)| ShownRow::matching(index, &row.name, &[], &self.filter))
            .collect();
        if shown.is_empty() {
            return 0;
        }
        let column = PlanColumn {
            label: spec.value_header.to_owned(),
            unit: String::new(),
        };
        let section = self.section(
            spec.kind,
            spec.title,
            spec.title,
            spec.name_header,
            vec![column],
        );
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

    fn devices(&mut self, devices: Option<&DeviceOpReport>, detail: Option<&RetainedDetail>) {
        let mut families = BTreeMap::<(usize, String), Vec<ShownRow>>::new();
        if let Some(report) = devices {
            for (index, entry) in report
                .entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| retained_detail_allows(&entry.name, detail))
            {
                let (family, _) = device_reading(entry);
                let (rank, title) = device_section(entry);
                let also = [family, title.as_str(), entry.region.unwrap_or_default()];
                if let Some(row) = ShownRow::matching(index, &entry.name, &also, &self.filter) {
                    families.entry((rank, title)).or_default().push(row);
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
            return;
        };
        let mut families: Vec<_> = families.into_iter().collect();
        families.sort_by(|((left_rank, left), _), ((right_rank, right), _)| {
            left_rank
                .cmp(right_rank)
                .then_with(|| natural_cmp(left, right))
        });
        for ((rank, title), mut shown) in families {
            let columns = family_columns(report, &shown);
            let key = if rank == MODULE_SECTION_RANK {
                format!("module {title}")
            } else {
                title.clone()
            };
            let mut section = self.section(OpRowKind::Device, &key, &title, "Instance", columns);
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
            self.add(section, &shown);
        }
    }
}

/// The sheet's rows and layout, computed once per dataset, filter, sort and
/// collapse so a frame only ever reads the rows it can show.
#[derive(Debug, Clone)]
pub struct OpPlan {
    sections: Vec<PlanSection>,
    /// The reader's significant digits, which every value reads at.
    significant_digits: usize,
    rows: Vec<PlanRow>,
    offsets: RowOffsets,
    longest_name: usize,
    node_shown: usize,
    /// The one line a filter that matched nothing leaves on the sheet.
    no_match: Option<String>,
    scope_note: Option<&'static str>,
    /// The power and bias sums over every device.
    summary: OpSummary,
    /// The operating point the values read against, and whether as changes.
    compare: Option<(OpBaseline, bool)>,
    /// How many rows differ from the baseline, when there is one.
    changed: Option<usize>,
    /// Every retained row, shown or filtered out.
    row_total: usize,
    /// Whether the sheet's grid has a region column: whether any section
    /// lists a device with one. A section without regions leaves it blank,
    /// so its first value stands under every other section's first value.
    region_slot: bool,
}

/// The sheet's one value grid: every section's value columns are one width
/// and keep one room for prefixes and units, so a column lines up from
/// section to section instead of each table setting its own.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ValueGrid {
    width: f32,
    unit_slot: f32,
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
        builder.devices(devices, detail);
        let no_match = (builder.sections.is_empty() && !builder.filter.is_empty()).then(|| {
            format!(
                "Nothing matches \u{201c}{}\u{201d}.",
                controls.filter.trim()
            )
        });
        OpPlan {
            region_slot: builder.sections.iter().any(|section| section.region),
            offsets: RowOffsets::from_heights(builder.rows.iter().map(|row| row.height())),
            longest_name: builder.longest_name,
            significant_digits: controls.significant_digits,
            sections: builder.sections,
            rows: builder.rows,
            node_shown,
            no_match,
            scope_note: reduced_scope_note(devices, detail),
            summary: OpSummary::of(devices),
            compare: None,
            changed: None,
            row_total: dc.map_or(0, |dc| dc.node_voltages.len() + dc.branch_currents.len())
                + devices.map_or(0, |report| report.entries.len()),
        }
    }

    /// This plan read against `baseline`: each value as itself or, with
    /// `delta`, as its change, and the rows that moved counted.
    #[must_use]
    pub fn with_baseline(
        mut self,
        baseline: OpBaseline,
        delta: bool,
        dc: Option<&DcOpResult>,
        devices: Option<&DeviceOpReport>,
    ) -> Self {
        self.changed = Some(changed_rows(dc, devices, &baseline).len());
        self.compare = Some((baseline, delta));
        self
    }

    /// One cell of `row` under `column`, read against the baseline when the
    /// sheet compares: a moved value is marked, and as a change it is signed.
    fn cell(&self, row: RowSource<'_>, kind: OpRowKind, column: &PlanColumn) -> CellValue {
        let digits = self.significant_digits;
        let Some((baseline, delta)) = self.compare.as_ref() else {
            return row.cell(column, digits);
        };
        let held = baseline.value(kind, row.name(), &column.label);
        match (row.quantity(column), held) {
            (Some((value, unit)), Some(base)) if value.is_finite() && base.is_finite() => {
                let changed = differs(value, base);
                if !*delta {
                    let (mantissa, unit) = value_parts(value, &unit, digits);
                    CellValue::Number {
                        mantissa,
                        unit,
                        changed,
                    }
                } else if changed {
                    let (mantissa, unit) = split_value(fmt_si_signed(value - base, &unit, digits));
                    CellValue::Number {
                        mantissa,
                        unit,
                        changed,
                    }
                } else {
                    CellValue::Same
                }
            }
            // A reading the baseline lacks is a change, and as a change it
            // has nothing to be measured from.
            (Some(_), None) if *delta => CellValue::Number {
                mantissa: "new".to_owned(),
                unit: String::new(),
                changed: true,
            },
            _ => match row.cell(column, digits) {
                CellValue::Number { mantissa, unit, .. } => CellValue::Number {
                    mantissa,
                    unit,
                    changed: held.is_none(),
                },
                other => other,
            },
        }
    }

    /// The value grid every section shares, at the mono `advance`: the
    /// sheet's own column width, grown by whatever the widest number and the
    /// longest prefix and unit on the sheet need.
    fn value_grid(&self, advance: f32) -> ValueGrid {
        let longest = |of: fn(&PlanSection) -> usize| {
            self.sections.iter().map(of).max().unwrap_or(0) as f32 * advance
        };
        let unit_slot = UNIT_SLOT_W.max(longest(|section| section.longest_unit).ceil());
        let mantissa = longest(|section| section.longest_mantissa).ceil();
        let narrowest = CELL_INSET + mantissa + MANTISSA_GAP + UNIT_SLOT_W;
        ValueGrid {
            width: VALUE_W.max(narrowest) + unit_slot - UNIT_SLOT_W,
            unit_slot,
        }
    }

    /// The slots a section's values take on the grid, counting the region
    /// column, blank or not.
    fn slots(&self, section: &PlanSection) -> usize {
        section.columns.len() + usize::from(self.region_slot)
    }

    /// The width of the widest section's whole value grid.
    fn widest_values(&self, grid: ValueGrid) -> f32 {
        self.sections
            .iter()
            .map(|section| self.slots(section) as f32 * grid.width)
            .fold(0.0, f32::max)
    }

    pub fn node_shown(&self) -> usize {
        self.node_shown
    }

    /// What the inspector says about devices the save left out, if it left
    /// any out.
    pub fn scope_note(&self) -> Option<&'static str> {
        self.scope_note
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
        self.indices(OpRowKind::Device).count()
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

/// The operating point the sheet reads against, and how it reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpCompare {
    /// The dataset of the other run whose operating point is the baseline.
    pub baseline: rspice_vocabulary::product::DatasetId,
    /// Each value as its change from the baseline's rather than itself.
    pub delta: bool,
}

/// Another operating point's readings by row, which the sheet compares this
/// one against.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OpBaseline {
    nodes: BTreeMap<String, f64>,
    branches: BTreeMap<String, f64>,
    /// Each device's quantities, by device and then by quantity.
    devices: BTreeMap<String, BTreeMap<String, f64>>,
}

impl OpBaseline {
    pub fn new(dc: Option<&DcOpResult>, devices: Option<&DeviceOpReport>) -> Self {
        let scalars = |rows: &[OperatingPointValue]| {
            rows.iter()
                .map(|row| (row.name.clone(), row.value))
                .collect()
        };
        Self {
            nodes: dc.map(|dc| scalars(&dc.node_voltages)).unwrap_or_default(),
            branches: dc
                .map(|dc| scalars(&dc.branch_currents))
                .unwrap_or_default(),
            devices: devices
                .map(|report| {
                    report
                        .entries
                        .iter()
                        .map(|entry| (entry.name.clone(), device_quantities(entry)))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    /// The baseline's reading of `quantity` on the row `name`. A node or a
    /// branch has one reading, whatever its column is called.
    fn value(&self, kind: OpRowKind, name: &str, quantity: &str) -> Option<f64> {
        match kind {
            OpRowKind::Node => self.nodes.get(name).copied(),
            OpRowKind::Branch => self.branches.get(name).copied(),
            OpRowKind::Device => self.devices.get(name)?.get(quantity).copied(),
        }
    }
}

/// A device's quantities by name, as its columns read them.
fn device_quantities(entry: &DeviceOpEntry) -> BTreeMap<String, f64> {
    let (_, quantities) = device_reading(entry);
    quantities
        .map(|(name, value, _)| (name.to_owned(), value))
        .collect()
}

/// Whether two readings of one quantity differ by more than the rounding in
/// a solver's last digits.
fn differs(current: f64, baseline: f64) -> bool {
    (current - baseline).abs() > 1e-9 * current.abs().max(baseline.abs()) + 1e-18
}

/// The rows whose readings differ from `baseline`'s, in sheet order: a value
/// that moved, or a row or a quantity only one of the two reports.
pub fn changed_rows(
    dc: Option<&DcOpResult>,
    devices: Option<&DeviceOpReport>,
    baseline: &OpBaseline,
) -> Vec<(OpRowKind, String)> {
    let mut changed = Vec::new();
    let mut scalars =
        |kind: OpRowKind, rows: &[OperatingPointValue], held: &BTreeMap<String, f64>| {
            for row in rows {
                if held
                    .get(&row.name)
                    .is_none_or(|base| differs(row.value, *base))
                {
                    changed.push((kind, row.name.clone()));
                }
            }
            for name in held.keys() {
                if !rows.iter().any(|row| row.name == *name) {
                    changed.push((kind, name.clone()));
                }
            }
        };
    scalars(
        OpRowKind::Node,
        dc.map_or(&[][..], |dc| dc.node_voltages.as_slice()),
        &baseline.nodes,
    );
    scalars(
        OpRowKind::Branch,
        dc.map_or(&[][..], |dc| dc.branch_currents.as_slice()),
        &baseline.branches,
    );
    let entries = devices.map_or(&[][..], |report| report.entries.as_slice());
    for entry in entries {
        let current = device_quantities(entry);
        let same = baseline.devices.get(&entry.name).is_some_and(|held| {
            held.len() == current.len()
                && current
                    .iter()
                    .all(|(name, value)| held.get(name).is_some_and(|base| !differs(*value, *base)))
        });
        if !same {
            changed.push((OpRowKind::Device, entry.name.clone()));
        }
    }
    for name in baseline.devices.keys() {
        if !entries.iter().any(|entry| entry.name == *name) {
            changed.push((OpRowKind::Device, name.clone()));
        }
    }
    changed
}

/// Whether a device sits outside the region its family is biased in for
/// gain: a MOSFET out of saturation, a bipolar transistor out of forward
/// active. A switch sits there by design; the sheet marks it and judges
/// nothing.
fn off_bias(family: &str, region: &str) -> bool {
    match family {
        "BJT" => !matches!(region, "active" | "forward"),
        family if !leading_quantities(family).is_empty() => region != "saturation",
        _ => false,
    }
}

/// The quantity a device reports its conductive power under.
const POWER_QUANTITY: &str = "power";

/// What the details pane sums over every device: the power dissipated and
/// who dissipates most, and the transistors outside their gain region.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OpSummary {
    /// The sum of every device's reported power.
    pub dissipated: Option<f64>,
    /// The device that dissipates most, and how much.
    pub largest: Option<(String, f64)>,
    /// Devices outside their family's gain region, with the region.
    pub off_bias: Vec<(String, &'static str)>,
    /// How many transistors report a region the summary judged.
    pub biased: usize,
}

impl OpSummary {
    pub fn of(devices: Option<&DeviceOpReport>) -> Self {
        let entries = devices.map_or(&[][..], |report| report.entries.as_slice());
        let powers: Vec<(&str, f64)> = entries
            .iter()
            .filter_map(|entry| {
                entry
                    .quantities()
                    .find(|(name, ..)| *name == POWER_QUANTITY)
                    .map(|(_, value, _)| (entry.name.as_str(), value))
            })
            .filter(|(_, value)| value.is_finite())
            .collect();
        Self {
            dissipated: (!powers.is_empty()).then(|| powers.iter().map(|(_, value)| value).sum()),
            largest: powers
                .iter()
                .max_by(|left, right| left.1.total_cmp(&right.1))
                .map(|(name, value)| ((*name).to_owned(), *value)),
            off_bias: entries
                .iter()
                .filter_map(|entry| {
                    let region = entry.region?;
                    off_bias(entry.device_kind, region).then(|| (entry.name.clone(), region))
                })
                .collect(),
            biased: entries
                .iter()
                .filter(|entry| {
                    entry.region.is_some()
                        && (entry.device_kind == "BJT"
                            || !leading_quantities(entry.device_kind).is_empty())
                })
                .count(),
        }
    }
}

/// The details pane's sums: the power the devices dissipate and the largest
/// share of it, the transistors outside their gain region, and, when the
/// sheet compares, the baseline and how many rows moved.
pub fn summary_panel(
    ui: &mut Ui,
    plan: &OpPlan,
    baseline: Option<&str>,
    significant_digits: usize,
) {
    let summary = &plan.summary;
    if let Some(dissipated) = summary.dissipated {
        section_header(ui, "Power", None);
        let mut rows = vec![(
            "Dissipated",
            fmt_si_significant(dissipated, "W", significant_digits),
            true,
        )];
        if let Some((name, power)) = &summary.largest {
            rows.push((
                "Largest",
                format!(
                    "{name} \u{b7} {}",
                    fmt_si_significant(*power, "W", significant_digits)
                ),
                false,
            ));
        }
        crate::presentation::stat_table(ui, &rows);
    }
    if summary.biased > 0 {
        section_header(ui, "Bias", None);
        if summary.off_bias.is_empty() {
            crate::presentation::stat_table(
                ui,
                &[(
                    "Gain region",
                    format!("all {} transistors", summary.biased),
                    false,
                )],
            );
        } else {
            let rows: Vec<(&str, String, bool)> = summary
                .off_bias
                .iter()
                .map(|(name, region)| (name.as_str(), (*region).to_owned(), false))
                .collect();
            crate::presentation::stat_table(ui, &rows);
        }
    }
    if let (Some(baseline), Some(changed)) = (baseline, plan.changed) {
        section_header(ui, "Compare", None);
        crate::presentation::stat_table(
            ui,
            &[
                ("Baseline", baseline.to_owned(), false),
                (
                    "Changed rows",
                    format!("{changed} of {}", plan.row_total),
                    true,
                ),
            ],
        );
    }
}

/// The baseline select: the width of a run's number and a short label, at
/// which the bar still holds the filter, the select and the reading switch
/// clear of Export on the narrowest sheet.
const BASELINE_WIDTH: f32 = 140.0;

/// The sheet's controls in its bar: the other run's operating point to
/// compare against, and whether each value reads as itself or as its change.
/// `false` when no other run holds an operating point.
pub fn domain_bar(
    ui: &mut Ui,
    candidates: &[(rspice_vocabulary::product::DatasetId, String)],
    compare: &mut Option<OpCompare>,
) -> bool {
    if candidates.is_empty() {
        *compare = None;
        return false;
    }
    let current = compare.filter(|held| candidates.iter().any(|(id, _)| *id == held.baseline));
    let mut names = vec!["No baseline".to_owned()];
    names.extend(candidates.iter().map(|(_, label)| label.clone()));
    let index = current
        .and_then(|held| candidates.iter().position(|(id, _)| *id == held.baseline))
        .map_or(0, |at| at + 1);
    crate::chrome::bars::bar_label(ui, "Compare");
    let mut next = current;
    if let Some(picked) = rspice_ui_kit::widgets::select(
        ui,
        "rspice.results.op.baseline",
        "Baseline operating point",
        &names[index],
        &names,
        BASELINE_WIDTH,
    ) {
        next = picked.checked_sub(1).map(|at| OpCompare {
            baseline: candidates[at].0,
            delta: current.is_none_or(|held| held.delta),
        });
    }
    if let Some(held) = next.as_mut() {
        let mut reading = usize::from(held.delta);
        if rspice_ui_kit::widgets::segmented(
            ui,
            "rspice.results.op.reading",
            &["Value", "\u{394}"],
            &mut reading,
            rspice_ui_kit::widgets::SegmentedWidth::Natural,
        ) {
            held.delta = reading == 1;
        }
    }
    *compare = next;
    true
}

/// One retained row, node or branch or device.
#[derive(Clone, Copy)]
enum RowSource<'a> {
    Scalar(&'a OperatingPointValue),
    Device(&'a DeviceOpEntry),
}

/// What one cell of a row holds.
enum CellValue {
    /// A value; `changed` when it moved from the baseline's.
    Number {
        mantissa: String,
        unit: String,
        changed: bool,
    },
    /// A change of nothing from the baseline.
    Same,
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

    fn cell(self, column: &PlanColumn, significant_digits: usize) -> CellValue {
        match self.quantity(column) {
            Some((value, unit)) if value.is_finite() => {
                let (mantissa, unit) = value_parts(value, &unit, significant_digits);
                CellValue::Number {
                    mantissa,
                    unit,
                    changed: false,
                }
            }
            Some(_) => CellValue::Refused,
            None => CellValue::Missing,
        }
    }

    /// The name a screen reader announces with every value on the row.
    fn spoken(self, section: &PlanSection, significant_digits: usize) -> String {
        let values = section
            .columns
            .iter()
            .map(|column| match self.quantity(column) {
                Some((value, unit)) => {
                    format!(
                        "{} {}",
                        column.label,
                        quantity_readout(value, &unit, significant_digits)
                    )
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

/// A finite value as a mantissa to `significant_digits` and the prefix and
/// unit that belong after it.
fn value_parts(value: f64, unit: &str, significant_digits: usize) -> (String, String) {
    split_value(fmt_si_significant(value, unit, significant_digits))
}

/// A printed value as its mantissa and the prefix and unit after it.
fn split_value(text: String) -> (String, String) {
    match text.split_once(' ') {
        Some((mantissa, suffix)) => (mantissa.to_owned(), suffix.to_owned()),
        None => (text, String::new()),
    }
}

/// A retained value as the inspector reads it: the reader's digits, and never
/// fewer than nine.
fn quantity_readout(value: f64, unit: &str, significant_digits: usize) -> String {
    if !value.is_finite() {
        return NON_FINITE.to_owned();
    }
    fmt_si_significant(
        value,
        unit,
        significant_digits.max(READOUT_SIGNIFICANT_DIGITS),
    )
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
    values: ValueGrid,
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

    /// Where a section of `slots` value columns, each `width` wide, starts on
    /// screen. It scrolls no further than it overflows, so a section that fits
    /// stays whole and one narrower than the widest stops at its own last
    /// column; and a whole column at a time, so the frozen name column never
    /// cuts a value in two.
    fn values_left(&self, slots: usize, width: f32) -> f32 {
        let scrolled = self.visible.left() - self.origin.x;
        let overflow = self.name_w + slots as f32 * width - self.visible.width();
        let columns = (scrolled.min(overflow.max(0.0)) / width).ceil();
        self.visible.left() + self.name_w - columns * width
    }

    /// One slot of a section's value grid, the section starting at `left`.
    fn value_cell(&self, left: f32, y: f32, height: f32, slot: usize, width: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(left + slot as f32 * width, self.screen_y(y)),
            egui::vec2(width, height),
        )
    }

    /// Whether a value column is on screen whole. One that is not stays
    /// blank: half a number reads as a different number.
    fn shows_whole(&self, cell: egui::Rect) -> bool {
        cell.right() <= self.visible.right() + WHOLE_COLUMN_TOLERANCE
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
}

fn mantissa_edge(cell: egui::Rect, unit_slot: f32) -> f32 {
    cell.right() - unit_slot - MANTISSA_GAP
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
            self.ui.id().with(("op-section", &section.key)),
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
            out.toggled_section = Some(section.key.clone());
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
        let width = self.grid.values.width;
        let left = self.grid.values_left(self.plan.slots(section), width);
        let mut slot = 0;
        if self.plan.region_slot {
            let cell = self.grid.value_cell(left, y, COLUMN_HEADER_H, slot, width);
            if section.region && self.grid.shows_whole(cell) {
                self.header_cell(section, cell, values, REGION_HEADER, HeaderAlign::Text, out);
            }
            slot += 1;
        }
        for column in &section.columns {
            let cell = self.grid.value_cell(left, y, COLUMN_HEADER_H, slot, width);
            slot += 1;
            if self.grid.shows_whole(cell) {
                self.header_cell(
                    section,
                    cell,
                    values,
                    &column.label,
                    HeaderAlign::Number,
                    out,
                );
            }
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
            self.ui.id().with(("op-sort", &section.key, label)),
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
        let edge = mantissa_edge(cell, self.grid.values.unit_slot);
        let left = match align {
            HeaderAlign::Name => cell.left() + NAME_INSET,
            HeaderAlign::Text => cell.left() + CELL_INSET,
            // A short label ends over its mantissas; a long one uses the
            // unit slot as well rather than clip.
            HeaderAlign::Number if width <= edge - cell.left() - MANTISSA_GAP => edge - width,
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
            out.clicked_sort = Some((section.key.clone(), label.to_owned()));
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

    /// Paints the row's name, and returns it whole if the cell had to cut it.
    fn paint_name(
        &self,
        painter: &egui::Painter,
        cell: egui::Rect,
        row: RowSource<'_>,
        kind: OpRowKind,
    ) -> Option<String> {
        let (parent, leaf) = name_parts(row.name());
        let parent = self.source.mapping.display_element(&parent);
        let leaf = if kind == OpRowKind::Node {
            self.source.mapping.display_node(&leaf)
        } else {
            self.source.mapping.display_element(&leaf)
        };
        let (shown_parent, shown_leaf) = elide_name(&parent, &leaf, self.grid.max_name_chars());
        let font = theme::mono(tokens::FS_1, FontWeight::Regular);
        let mut job = LayoutJob::default();
        if !shown_parent.is_empty() {
            job.append(
                &shown_parent,
                0.0,
                TextFormat::simple(font.clone(), self.colors.text_dim),
            );
        }
        job.append(&shown_leaf, 0.0, TextFormat::simple(font, self.colors.text));
        let galley = painter.layout_job(job);
        painter.galley(
            egui::pos2(
                cell.left() + NAME_INSET,
                cell.center().y - galley.size().y * 0.5,
            ),
            galley,
            self.colors.text,
        );
        (shown_parent != parent || shown_leaf != *leaf).then(|| format!("{parent}{leaf}"))
    }

    fn paint_value(
        &self,
        painter: &egui::Painter,
        cell: egui::Rect,
        unit_slot: f32,
        value: CellValue,
    ) {
        let painter = painter.with_clip_rect(cell.intersect(painter.clip_rect()));
        let font = theme::mono(tokens::FS_1, FontWeight::Regular);
        match value {
            CellValue::Number {
                mantissa,
                unit,
                changed,
            } => {
                let edge = mantissa_edge(cell, unit_slot);
                painter.text(
                    egui::pos2(edge, cell.center().y),
                    egui::Align2::RIGHT_CENTER,
                    mantissa,
                    font.clone(),
                    if changed {
                        self.colors.accent
                    } else {
                        self.colors.text
                    },
                );
                if !unit.is_empty() {
                    painter.text(
                        egui::pos2(edge + MANTISSA_GAP, cell.center().y),
                        egui::Align2::LEFT_CENTER,
                        unit,
                        font,
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
                    egui::pos2(mantissa_edge(cell, unit_slot), cell.center().y),
                    egui::Align2::RIGHT_CENTER,
                    "\u{2014}",
                    font,
                    self.colors.text_faint,
                );
            }
            CellValue::Same => {
                painter.text(
                    egui::pos2(mantissa_edge(cell, unit_slot), cell.center().y),
                    egui::Align2::RIGHT_CENTER,
                    "0",
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
        let cut_name = self.paint_name(
            &painter.with_clip_rect(name_cell.intersect(rect)),
            name_cell,
            row,
            section.kind,
        );
        let values_clip = self.grid.values_clip().intersect(rect);
        let values = self.ui.painter_at(values_clip);
        let width = self.grid.values.width;
        let unit_slot = self.grid.values.unit_slot;
        let left = self.grid.values_left(self.plan.slots(section), width);
        let mut slot = 0;
        if self.plan.region_slot {
            let cell = self.grid.value_cell(left, y, ROW_H, slot, width);
            if let Some(region) = row.region().filter(|_| self.grid.shows_whole(cell)) {
                // A transistor outside its gain region reads in the warning
                // ink, so a triode or saturated device stands out of the list.
                let flagged =
                    matches!(row, RowSource::Device(entry) if off_bias(entry.device_kind, region));
                values.with_clip_rect(cell.intersect(values_clip)).text(
                    egui::pos2(cell.left() + CELL_INSET, cell.center().y),
                    egui::Align2::LEFT_CENTER,
                    region,
                    theme::sans(tokens::FS_1, FontWeight::Regular),
                    if flagged {
                        self.colors.warn
                    } else {
                        self.colors.text_dim
                    },
                );
            }
            slot += 1;
        }
        for column in &section.columns {
            let cell = self.grid.value_cell(left, y, ROW_H, slot, width);
            slot += 1;
            if self.grid.shows_whole(cell) {
                self.paint_value(
                    &values,
                    cell,
                    unit_slot,
                    self.plan.cell(row, section.kind, column),
                );
            }
        }
        describe(
            self.ui,
            &response,
            egui::accesskit::Role::Row,
            &row.spoken(section, self.plan.significant_digits),
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
        // Over the cut name only: anything else on the row is already legible,
        // and a tooltip would restate it on top of the row below.
        if let Some(whole) = cut_name.filter(|_| {
            response
                .hover_pos()
                .is_some_and(|pos| name_cell.contains(pos))
        }) {
            response.on_hover_text_at_pointer(
                egui::RichText::new(whole).font(theme::mono(tokens::FS_1, FontWeight::Regular)),
            );
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
    let values = plan.value_grid(advance);
    let content_w = name_w + plan.widest_values(values);
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
                values,
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
    significant_digits: usize,
) -> Option<OpDetail> {
    let row = RowSource::of(selection.kind, selection.index, dc, devices)?;
    let (parent, leaf) = name_parts(row.name());
    let parent = mapping.display_element(&parent).into_owned();
    let leaf = if selection.kind == OpRowKind::Node {
        mapping.display_node(&leaf).into_owned()
    } else {
        mapping.display_element(&leaf).into_owned()
    };
    let mut rows = Vec::new();
    match row {
        RowSource::Scalar(value) => {
            let label = match selection.kind {
                OpRowKind::Node => NODES.value_header,
                _ => BRANCHES.value_header,
            };
            rows.push((
                label.to_owned(),
                quantity_readout(value.value, &value.unit, significant_digits),
            ));
        }
        RowSource::Device(entry) => {
            let (_, quantities) = device_reading(entry);
            let family = ranking_family(entry);
            rows.push(("Family".to_owned(), device_section(entry).1));
            if let Some(region) = entry.region {
                rows.push((REGION_HEADER.to_owned(), region.to_owned()));
            }
            let mut quantities: Vec<_> = quantities.collect();
            quantities.sort_by_key(|(name, _, _)| quantity_rank(family, name));
            rows.extend(quantities.into_iter().map(|(name, value, unit)| {
                (
                    name.to_owned(),
                    quantity_readout(value, unit, significant_digits),
                )
            }));
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
    scope_note: Option<&str>,
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
        let temperature = fmt_si_significant(facts.temperature_celsius, CELSIUS, 4);
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
    if let Some(note) = scope_note {
        crate::presentation::panel_note(ui, note);
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
