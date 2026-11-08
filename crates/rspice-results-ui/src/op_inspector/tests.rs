use super::*;

fn node(name: &str, value: f64) -> OperatingPointValue {
    OperatingPointValue {
        name: name.to_owned(),
        value,
        unit: "V".to_owned(),
    }
}

fn device(
    name: &str,
    family: &'static str,
    region: Option<&'static str>,
    params: Vec<(&'static str, f64)>,
) -> DeviceOpEntry {
    DeviceOpEntry {
        name: name.to_owned(),
        device_kind: family,
        region,
        params,
    }
}

fn plan_with<'a>(
    filter: &'a str,
    sort: &'a OpSort,
    collapsed: &'a BTreeSet<String>,
    dc: Option<&DcOpResult>,
    devices: Option<&DeviceOpReport>,
) -> OpPlan {
    OpPlan::new(
        OpControls {
            filter,
            sort,
            collapsed,
        },
        dc,
        devices,
        None,
    )
}

fn plan_of(dc: Option<&DcOpResult>, devices: Option<&DeviceOpReport>) -> OpPlan {
    plan_with("", &OpSort::new(), &BTreeSet::new(), dc, devices)
}

fn titles(plan: &OpPlan) -> Vec<&str> {
    plan.sections
        .iter()
        .map(|section| section.title.as_str())
        .collect()
}

fn section_names(plan: &OpPlan, title: &str, names: impl Fn(usize) -> String) -> Vec<String> {
    let wanted = plan
        .sections
        .iter()
        .position(|section| section.title == title)
        .unwrap_or_else(|| panic!("no section {title}"));
    plan.rows
        .iter()
        .filter_map(|row| match *row {
            PlanRow::Value { section, index } if section == wanted => Some(names(index)),
            _ => None,
        })
        .collect()
}

fn labels(plan: &OpPlan, title: &str) -> Vec<String> {
    plan.sections
        .iter()
        .find(|section| section.title == title)
        .map(|section| {
            section
                .columns
                .iter()
                .map(|column| column.label.clone())
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn a_name_reads_as_its_path_and_leaf_relative_to_the_root() {
    for (name, parent, leaf) in [
        ("V(out)", "", "out"),
        ("0", "", "0"),
        ("V(top.n2)", "", "n2"),
        ("V(X1.mid)", "X1.", "mid"),
        ("V(/X1/n)", "X1.", "n"),
        ("XAFE.M1", "XAFE.", "M1"),
        ("XA.XB.M1", "XA.XB.", "M1"),
        ("I(V1)", "", "V1"),
        ("I(x1.v2)", "x1.", "v2"),
        ("v1#branch", "", "v1"),
    ] {
        assert_eq!(
            name_parts(name),
            (parent.to_owned(), leaf.to_owned()),
            "{name}"
        );
    }
}

#[test]
fn names_count_like_a_designer_and_the_top_level_comes_first() {
    let dc = DcOpResult {
        node_voltages: [
            "V(x1.a)",
            "V(M10)",
            "V(M2)",
            "V(x1.b2)",
            "V(x1.b10)",
            "V(a)",
        ]
        .into_iter()
        .map(|name| node(name, 1.0))
        .collect(),
        ..DcOpResult::default()
    };
    let plan = plan_of(Some(&dc), None);
    assert_eq!(
        section_names(&plan, "Node voltages", |index| dc.node_voltages[index]
            .name
            .clone()),
        [
            "V(a)",
            "V(M2)",
            "V(M10)",
            "V(x1.a)",
            "V(x1.b2)",
            "V(x1.b10)"
        ]
    );
}

#[test]
fn the_sheet_lists_nodes_then_branches_then_families_in_their_order() {
    let dc = DcOpResult {
        node_voltages: vec![node("V(a)", 1.0)],
        branch_currents: vec![node("I(V1)", 1.0e-3)],
        ..DcOpResult::default()
    };
    let report = DeviceOpReport {
        entries: vec![
            device("X1", "zeta_amp", None, vec![("iout", 1.0)]),
            device("D1", "DIODE", None, vec![("vd", 0.7)]),
            device("J1", "JFET", None, vec![("id", 1.0)]),
            device("Q1", "BJT", None, vec![("ic", 1.0)]),
            device("M1", "MOSFET", Some("saturation"), vec![("id", 1.0)]),
            device("X2", "alpha_amp", None, vec![("iout", 1.0)]),
        ],
    };
    assert_eq!(
        titles(&plan_of(Some(&dc), Some(&report))),
        [
            "Node voltages",
            "Branch currents",
            "MOSFET",
            "BJT",
            "Diode",
            "alpha_amp",
            "JFET",
            "zeta_amp",
        ]
    );
}

#[test]
fn a_family_shows_only_its_own_quantities() {
    let report = DeviceOpReport {
        entries: vec![
            device(
                "D1",
                "DIODE",
                None,
                vec![("vd", 0.7), ("id", 1.0e-3), ("temp", 27.0)],
            ),
            device(
                "X1",
                "vamp",
                None,
                vec![("iinp", 1.0), ("iinn", 1.0), ("ignd", 1.0), ("iout", 1.0)],
            ),
            device(
                "M1",
                "MOSFET",
                Some("saturation"),
                vec![("gm", 1.0), ("vgs", 1.1)],
            ),
        ],
    };
    let plan = plan_of(None, Some(&report));
    assert_eq!(labels(&plan, "Diode"), ["vd", "id", "temp"]);
    assert_eq!(labels(&plan, "vamp"), ["iinp", "iinn", "ignd", "iout"]);
    assert_eq!(labels(&plan, "MOSFET"), ["vgs", "gm"]);
}

#[test]
fn a_region_column_appears_only_where_a_row_has_one() {
    let report = DeviceOpReport {
        entries: vec![
            device("M1", "MOSFET", Some("saturation"), vec![("gm", 1.0)]),
            device("Q1", "BJT", None, vec![("ic", 1.0)]),
        ],
    };
    let plan = plan_of(None, Some(&report));
    let family = |title: &str| {
        plan.sections
            .iter()
            .find(|section| section.title == title)
            .expect("the family has a section")
    };
    assert!(family("MOSFET").region);
    assert_eq!(family("MOSFET").slots(), 2);
    assert!(!family("BJT").region);
    assert_eq!(family("BJT").slots(), 1);
}

#[test]
fn sorting_one_family_leaves_the_others_and_the_columns_alone() {
    let report = DeviceOpReport {
        entries: vec![
            device(
                "M1",
                "MOSFET",
                Some("cutoff"),
                vec![("gm", 1.0e-3), ("vgs", 0.1)],
            ),
            device("M2", "MOSFET", Some("saturation"), vec![("gm", 3.0e-3)]),
            device("Q2", "BJT", None, vec![("ic", 1.0e-3)]),
            device("Q1", "BJT", None, vec![("ic", 2.0e-3)]),
        ],
    };
    let name = |index: usize| report.entries[index].name.clone();
    let mut sort = OpSort::new();
    sort.insert("MOSFET".to_owned(), ("gm".to_owned(), false));
    let collapsed = BTreeSet::new();
    let plan = plan_with("", &sort, &collapsed, None, Some(&report));
    assert_eq!(section_names(&plan, "MOSFET", name), ["M2", "M1"]);
    assert_eq!(
        section_names(&plan, "BJT", name),
        ["Q1", "Q2"],
        "a sort in one section reordered another"
    );
    assert_eq!(
        labels(&plan, "MOSFET"),
        ["vgs", "gm"],
        "the sort rewrote the columns"
    );
}

#[test]
fn a_quantity_sorts_by_magnitude_and_missing_values_stay_last() {
    let report = DeviceOpReport {
        entries: vec![
            device("M_missing", "MOSFET", Some("cutoff"), vec![]),
            device("M_low", "MOSFET", Some("saturation"), vec![("gm", 1.0e-3)]),
            device("M_high", "MOSFET", Some("saturation"), vec![("gm", 3.0e-3)]),
            device(
                "M_negative",
                "MOSFET",
                Some("saturation"),
                vec![("gm", -4.0e-3)],
            ),
            device(
                "M_nan",
                "MOSFET",
                Some("saturation"),
                vec![("gm", f64::NAN)],
            ),
        ],
    };
    let name = |index: usize| report.entries[index].name.clone();
    let collapsed = BTreeSet::new();
    let mut sort = OpSort::new();
    sort.insert("MOSFET".to_owned(), ("gm".to_owned(), false));
    let plan = plan_with("", &sort, &collapsed, None, Some(&report));
    assert_eq!(
        section_names(&plan, "MOSFET", name),
        ["M_negative", "M_high", "M_low", "M_missing", "M_nan"]
    );
    sort.insert("MOSFET".to_owned(), ("gm".to_owned(), true));
    let plan = plan_with("", &sort, &collapsed, None, Some(&report));
    assert_eq!(
        section_names(&plan, "MOSFET", name),
        ["M_low", "M_high", "M_negative", "M_missing", "M_nan"]
    );
}

#[test]
fn the_name_column_and_the_region_sort_too() {
    let report = DeviceOpReport {
        entries: vec![
            device("M2", "MOSFET", Some("triode"), vec![]),
            device("M10", "MOSFET", Some("cutoff"), vec![]),
            device("M1", "MOSFET", None, vec![]),
        ],
    };
    let name = |index: usize| report.entries[index].name.clone();
    let collapsed = BTreeSet::new();
    let mut sort = OpSort::new();
    sort.insert("MOSFET".to_owned(), ("Instance".to_owned(), false));
    let plan = plan_with("", &sort, &collapsed, None, Some(&report));
    assert_eq!(section_names(&plan, "MOSFET", name), ["M10", "M2", "M1"]);
    sort.insert("MOSFET".to_owned(), (REGION_HEADER.to_owned(), true));
    let plan = plan_with("", &sort, &collapsed, None, Some(&report));
    assert_eq!(section_names(&plan, "MOSFET", name), ["M10", "M2", "M1"]);
}

#[test]
fn the_filter_reads_names_in_every_section_and_drops_empty_ones() {
    let dc = DcOpResult {
        node_voltages: vec![
            node("V(mid)", 1.0),
            node("V(X1.mid)", 1.0),
            node("V(out)", 1.0),
        ],
        branch_currents: vec![node("I(V1)", 1.0e-3)],
        ..DcOpResult::default()
    };
    let report = DeviceOpReport {
        entries: vec![
            device("M1", "MOSFET", Some("saturation"), vec![("gm", 1.0)]),
            device("X1.Mid", "MOSFET", Some("saturation"), vec![("gm", 1.0)]),
            device("D1", "DIODE", None, vec![("gd", 1.0)]),
        ],
    };
    let sort = OpSort::new();
    let collapsed = BTreeSet::new();
    let plan = plan_with("MID", &sort, &collapsed, Some(&dc), Some(&report));
    assert_eq!(titles(&plan), ["Node voltages", "MOSFET"]);
    assert_eq!(plan.node_shown(), 2);
    assert_eq!(plan.device_shown(), 1);
    let plan = plan_with("zz", &sort, &collapsed, Some(&dc), Some(&report));
    assert!(plan.sections.is_empty());
    assert_eq!(
        plan.no_match.as_deref(),
        Some("Nothing matches \u{201c}zz\u{201d}.")
    );
    assert_eq!(plan.rows.len(), 0);
}

#[test]
fn the_filter_finds_a_device_by_region_and_family_but_not_by_quantity() {
    let dc = DcOpResult {
        node_voltages: vec![node("V(out)", 1.0)],
        branch_currents: vec![node("I(V1)", 1.0e-3)],
        ..DcOpResult::default()
    };
    let report = DeviceOpReport {
        entries: vec![
            device("M1", "MOSFET", Some("linear"), vec![("gm", 1.0)]),
            device("M2", "MOSFET", Some("saturation"), vec![("gm", 1.0)]),
            device("Q1", "BJT", Some("active"), vec![("gm", 1.0)]),
            device("D1", "DIODE", None, vec![("gd", 1.0)]),
            device("B1", "NONLINEAR_CORE", None, vec![("h", 1.0)]),
        ],
    };
    let name = |index: usize| report.entries[index].name.clone();
    let sort = OpSort::new();
    let collapsed = BTreeSet::new();
    let plan_for = |filter| plan_with(filter, &sort, &collapsed, Some(&dc), Some(&report));

    let plan = plan_for("LINEAR");
    assert_eq!(titles(&plan), ["MOSFET"], "and not the nonlinear core");
    assert_eq!(section_names(&plan, "MOSFET", name), ["M1"]);

    assert_eq!(titles(&plan_for("sat")), ["MOSFET"]);
    assert_eq!(titles(&plan_for("bjt")), ["BJT"]);
    assert_eq!(titles(&plan_for("diode")), ["Diode"]);
    assert_eq!(
        titles(&plan_for("nonlinear core")),
        ["Nonlinear core"],
        "the family as the section title spells it"
    );
    assert_eq!(
        titles(&plan_for("nonlinear_core")),
        ["Nonlinear core"],
        "the family as the report spells it"
    );

    let plan = plan_for("gm");
    assert!(plan.sections.is_empty(), "every device reports a gm");
    assert_eq!(
        plan.no_match.as_deref(),
        Some("Nothing matches \u{201c}gm\u{201d}.")
    );
}

#[test]
fn a_section_the_save_did_not_keep_is_a_title_and_one_line() {
    let plan = plan_of(None, None);
    assert_eq!(
        titles(&plan),
        [
            "Node voltages",
            "Branch currents",
            "Device operating points"
        ]
    );
    assert_eq!(
        plan.sections[0].note.as_deref(),
        Some("Node-voltage evidence was not retained for this analysis.")
    );
    assert_eq!(
        plan.sections[2].note.as_deref(),
        Some("Per-device operating-point quantities were not retained for this analysis.")
    );
    let notes = plan
        .rows
        .iter()
        .filter(|row| matches!(row, PlanRow::Note(_)))
        .count();
    assert_eq!(notes, 3);

    let (sort, collapsed) = (OpSort::new(), BTreeSet::new());
    let filtered = plan_with("x", &sort, &collapsed, None, None);
    assert!(filtered.sections.is_empty(), "a filter leaves only matches");
}

#[test]
fn folding_a_section_keeps_its_title_and_the_sheet_geometry() {
    let (dc, report) = evidence_fixture(12, 12);
    let (sort, mut collapsed) = (OpSort::new(), BTreeSet::new());
    let open = plan_with("", &sort, &collapsed, Some(&dc), Some(&report));
    collapsed.insert("MOSFET".to_owned());
    let folded = plan_with("", &sort, &collapsed, Some(&dc), Some(&report));
    assert_eq!(titles(&open), titles(&folded));
    assert_eq!(folded.device_indices().count(), 0);
    assert_eq!(
        folded.sections[2].count, 12,
        "the count still says what's inside"
    );
    assert_eq!(
        open.longest_name, folded.longest_name,
        "folding a section moved the name column"
    );
    assert!(folded.pinned_at(folded.sections[2].top + 1.0).is_none());
}

#[test]
fn every_row_of_a_kind_has_the_same_pitch() {
    let (dc, report) = evidence_fixture(6, 6);
    let plan = plan_of(Some(&dc), Some(&report));
    let (rows, offsets) = plan.row_counts();
    assert_eq!(rows, offsets);
    for row in &plan.rows {
        let expected = match row {
            PlanRow::Gap => 8.0,
            PlanRow::Title(_) => 24.0,
            PlanRow::Columns(_) | PlanRow::Value { .. } | PlanRow::Note(_) => 20.0,
        };
        assert_eq!(row.height(), expected, "{row:?}");
    }
}

#[test]
fn the_header_pins_at_the_top_and_the_next_section_pushes_it_off() {
    let (dc, report) = evidence_fixture(12, 12);
    let plan = plan_of(Some(&dc), Some(&report));
    let first = &plan.sections[0];
    let second = &plan.sections[1];
    assert_eq!(
        plan.pinned_at(0.0),
        None,
        "in place at the top of the sheet"
    );
    assert_eq!(plan.pinned_at(30.0), Some((0, 30.0)));
    let height = TITLE_H + COLUMN_HEADER_H;
    let late = first.bottom - height + 5.0;
    assert_eq!(
        plan.pinned_at(late),
        Some((0, first.bottom - height)),
        "the section's last row has pushed its header up"
    );
    let later = second.top + 3.0;
    assert_eq!(plan.pinned_at(later).map(|(index, _)| index), Some(1));
}

#[test]
fn values_share_one_mantissa_edge_and_one_unit_edge() {
    for (value, unit, mantissa, suffix) in [
        (5.0, "V", "5.00000", "V"),
        (0.751_626_9, "V", "751.627", "mV"),
        (0.0, "A", "0.00000", "A"),
        (-41.7321e-3, "A", "-41.7321", "mA"),
        (999.9996e-3, "V", "1.00000", "V"),
        (2.0e6, "Hz", "2.00000", "MHz"),
        (123.456, "", "123.456", ""),
        (1234.56, "", "1.23456", "k"),
        (27.0, CELSIUS, "27.0000", CELSIUS),
        (0.5, CELSIUS, "0.500000", CELSIUS),
        (-273.15, CELSIUS, "-273.150", CELSIUS),
    ] {
        assert_eq!(
            value_parts(value, unit),
            (mantissa.to_owned(), suffix.to_owned()),
            "{value} {unit}"
        );
    }
}

#[test]
fn the_readout_keeps_the_precision_the_table_drops() {
    assert_eq!(quantity_readout(0.751_626_903, "V"), "751.626903 mV");
    assert_eq!(quantity_readout(5.0, "V"), "5 V");
    assert_eq!(quantity_readout(26.85, CELSIUS), "26.85 \u{b0}C");
    assert_eq!(quantity_readout(27.000000000001, CELSIUS), "27 \u{b0}C");
    assert_eq!(quantity_readout(100.0, CELSIUS), "100 \u{b0}C");
    assert_eq!(quantity_readout(0.0, CELSIUS), "0 \u{b0}C");
    assert_eq!(quantity_readout(f64::NAN, "V"), NON_FINITE);
}

#[test]
fn a_non_finite_value_stays_a_row_and_is_refused_in_its_cell() {
    let failed = node("V(failed)", f64::NAN);
    let dc = DcOpResult {
        node_voltages: vec![failed.clone()],
        ..DcOpResult::default()
    };
    let plan = plan_of(Some(&dc), None);
    assert_eq!(plan.node_shown(), 1);
    assert!(matches!(
        RowSource::Scalar(&failed).cell(&plan.sections[0].columns[0]),
        CellValue::Refused
    ));
}

#[test]
fn a_family_leads_with_the_quantities_a_designer_reads() {
    let reported = |family: &'static str, names: &[&'static str]| {
        let entry = device(
            "U1",
            family,
            None,
            names.iter().map(|name| (*name, 1.0)).collect(),
        );
        let report = DeviceOpReport {
            entries: vec![entry],
        };
        let plan = plan_of(None, Some(&report));
        let columns = labels(&plan, &family_title(family));
        let selection = OpSelection {
            kind: OpRowKind::Device,
            index: 0,
            name: "U1".to_owned(),
        };
        let mapped = Mapped {
            node: false,
            device: None,
        };
        let detail =
            selection_detail(&selection, None, Some(&report), &mapped).expect("the row exists");
        let inspector: Vec<String> = detail
            .rows
            .iter()
            .map(|(label, _)| label.clone())
            .filter(|label| label != "Family" && label != "Identity")
            .collect();
        assert_eq!(columns, inspector, "the inspector reads like the sheet");
        columns
    };

    assert_eq!(
        reported(
            "MOSFET",
            &[
                "id", "ig", "is", "ib", "vgs", "vds", "vbs", "vth", "vdsat", "gm", "gds", "gmb",
                "temp", "power",
            ],
        ),
        [
            "id", "vgs", "vds", "vbs", "vth", "vdsat", "gm", "gds", "gmb", "ig", "is", "ib",
            "temp", "power",
        ],
    );
    assert_eq!(
        reported("BSIM4", &["id", "ig", "vgs", "output_vdsat", "gm"]),
        ["id", "vgs", "gm", "ig", "output_vdsat"],
        "every MOSFET model reads alike"
    );
    assert_eq!(
        reported("BJT", &["ic", "ib", "ie", "is", "vbe", "vce", "gm", "beta"]),
        ["ic", "ib", "vbe", "vce", "gm", "ie", "is", "beta"],
    );
    let as_reported = ["vd", "id", "gd", "cd"];
    assert_eq!(reported("DIODE", &as_reported), as_reported);
    let module = ["iout", "id", "ig"];
    assert_eq!(
        reported("vamp", &module),
        module,
        "a module keeps its stated order"
    );
}

#[test]
fn a_name_too_long_for_the_column_gives_up_its_middle() {
    assert_eq!(elide_middle("abcdefghij", 5), "ab\u{2026}ij");
    assert_eq!(
        elide_name("Xouter.Xinner.", "M1", 10),
        ("Xout\u{2026}er.".to_owned(), "M1".to_owned()),
    );
    assert_eq!(
        elide_name("X1.", "averyveryverylongleaf", 9),
        ("X1.".to_owned(), "a\u{2026}leaf".to_owned()),
    );
    assert_eq!(
        elide_name("Xouter.Xinner.", "averyveryverylongleaf", 9),
        ("Xout\u{2026}".to_owned(), "leaf".to_owned()),
    );
    assert_eq!(
        elide_name("X1.", "mid", 10),
        ("X1.".to_owned(), "mid".to_owned())
    );
}

#[test]
fn the_name_column_fits_the_longest_name_within_its_limits() {
    let advance = 7.0;
    assert_eq!(name_column_width(advance, 3), NAME_MIN_W);
    let fits = 20;
    assert_eq!(
        name_column_width(advance, fits),
        fits as f32 * advance + NAME_INSET + NAME_END_PAD
    );
    assert_eq!(name_column_width(advance, 500), NAME_MAX_W);
}

#[test]
fn a_selection_follows_its_row_and_goes_when_the_row_does() {
    let dc = DcOpResult {
        node_voltages: vec![node("V(a)", 1.0), node("V(b)", 2.0)],
        ..DcOpResult::default()
    };
    let selection = OpSelection {
        kind: OpRowKind::Node,
        index: 1,
        name: "V(b)".to_owned(),
    };
    assert_eq!(selection.resolved(Some(&dc), None), Some(selection.clone()));
    let moved = DcOpResult {
        node_voltages: vec![node("V(b)", 2.0), node("V(c)", 3.0)],
        ..DcOpResult::default()
    };
    assert_eq!(
        selection
            .resolved(Some(&moved), None)
            .map(|found| found.index),
        Some(0)
    );
    let gone = DcOpResult {
        node_voltages: vec![node("V(c)", 3.0)],
        ..DcOpResult::default()
    };
    assert_eq!(selection.resolved(Some(&gone), None), None);
    assert_eq!(selection.resolved(None, None), None);
}

struct Mapped {
    node: bool,
    device: Option<u64>,
}

impl SchematicMapping for Mapped {
    fn display_node<'a>(&self, name: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(name)
    }
    fn node_available(&self, _: &str) -> bool {
        self.node
    }
    fn device_target(&self, name: &str) -> Option<u64> {
        (name != "V9").then_some(self.device).flatten()
    }
}

#[test]
fn a_cross_probe_names_its_target_or_says_why_not() {
    let mapped = Mapped {
        node: true,
        device: Some(7),
    };
    assert_eq!(
        locate_action(OpRowKind::Node, "V(a)", &mapped),
        OpAction::LocateNode("V(a)".to_owned())
    );
    assert_eq!(
        locate_action(OpRowKind::Branch, "I(V1)", &mapped),
        OpAction::LocateDevice(7),
    );
    assert_eq!(
        locate_action(OpRowKind::Branch, "I(V9)", &mapped),
        OpAction::Refused(DEVICE_NOT_ON_SCHEMATIC),
        "the branch's source has to be the device asked about"
    );
    let stale = Mapped {
        node: false,
        device: None,
    };
    assert_eq!(
        locate_action(OpRowKind::Node, "V(a)", &stale),
        OpAction::Refused(NODE_NOT_ON_SCHEMATIC)
    );
    assert_eq!(
        locate_action(OpRowKind::Device, "M1", &stale),
        OpAction::Refused(DEVICE_NOT_ON_SCHEMATIC)
    );
}

#[test]
fn the_inspector_states_every_quantity_to_nine_digits_with_the_deck_name() {
    let report = DeviceOpReport {
        entries: vec![device(
            "X1.M1",
            "MOSFET",
            Some("saturation"),
            vec![("id", 1.234_567_891e-3), ("temp", 27.0)],
        )],
    };
    let selection = OpSelection {
        kind: OpRowKind::Device,
        index: 0,
        name: "X1.M1".to_owned(),
    };
    let mapped = Mapped {
        node: false,
        device: None,
    };
    let detail =
        selection_detail(&selection, None, Some(&report), &mapped).expect("the row exists");
    assert_eq!(detail.title, "X1.M1");
    assert_eq!(
        detail.rows,
        [
            ("Family", "MOSFET"),
            ("Region", "saturation"),
            ("id", "1.23456789 mA"),
            ("temp", "27 \u{b0}C"),
            ("Identity", "X1.M1"),
        ]
        .map(|(label, value)| (label.to_owned(), value.to_owned()))
    );
    assert_eq!(detail.action, OpAction::Refused(DEVICE_NOT_ON_SCHEMATIC));
}

#[test]
fn a_section_scrolls_in_whole_columns_only_as_far_as_it_overflows() {
    let name_w = 200.0;
    let grid = Grid {
        origin: egui::pos2(-300.0, 0.0),
        visible: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(600.0, 400.0)),
        name_w,
        advance: 7.0,
    };
    assert_eq!(grid.values_left(3), name_w, "it fits: nothing moves");
    assert_eq!(
        grid.values_left(5),
        name_w - 2.0 * VALUE_W,
        "it stops once its last column is whole"
    );
    assert_eq!(
        grid.values_left(20),
        name_w - 3.0 * VALUE_W,
        "it follows the bar a column at a time"
    );
}

#[test]
fn only_a_whole_column_is_painted_and_the_last_one_can_always_be_reached() {
    let visible = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(600.0, 400.0));
    let at = |scrolled: f32| Grid {
        origin: egui::pos2(-scrolled, 0.0),
        visible,
        name_w: 200.0,
        advance: 7.0,
    };
    let cell = |grid: &Grid, slot: usize| grid.value_cell(grid.values_left(8), 0.0, ROW_H, slot);

    let start = at(0.0);
    assert!(start.shows_whole(cell(&start, 2)), "400 pt holds three");
    assert!(
        !start.shows_whole(cell(&start, 3)),
        "the fourth would be cut"
    );

    let end = at(200.0 + 8.0 * VALUE_W - 600.0);
    assert!(end.shows_whole(cell(&end, 7)));
    assert!(!end.shows_whole(egui::Rect::from_min_size(
        egui::pos2(visible.right() - 10.0, 0.0),
        egui::vec2(VALUE_W, ROW_H),
    )));
}

#[test]
fn a_header_is_on_the_sheet_only_for_a_column_that_fits() {
    let params = [
        "id", "vgs", "vds", "vbs", "vth", "vdsat", "gm", "gds", "gmb",
    ]
    .map(|name| (name, 1.0))
    .to_vec();
    let report = DeviceOpReport {
        entries: vec![device("M1", "MOSFET", None, params)],
    };
    let plan = plan_of(None, Some(&report));
    let mapping = Mapped {
        node: false,
        device: None,
    };
    let view = OpView {
        evidence: &evidence(),
        dc: None,
        devices: Some(&report),
        selection: None,
        mapping: &mapping,
        failure_site: None,
    };
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..3 {
        let input = rspice_ui_kit::raster::display_input(SIZE, 1.0);
        let output = ctx.run_ui(input, |ui| {
            let _ = show(ui, Some(&plan), &view);
        });
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    let painted = nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .filter(|label| label.starts_with("MOSFET: ") && !label.starts_with("MOSFET: Instance"))
        .count();
    // 900 pt less the 160 pt name column holds seven 104 pt columns, whatever
    // the scroll bar's gutter takes.
    assert_eq!(painted, 7);
}

#[test]
fn a_failed_solve_offers_to_mark_what_it_named_at_the_end_of_its_row() {
    let (dc, report) = evidence_fixture(4, 4);
    let plan = plan_of(Some(&dc), Some(&report));
    let mapping = Mapped {
        node: true,
        device: None,
    };
    let failed = OpEvidence {
        success: false,
        error: Some("did not converge".to_owned()),
        facts: None,
    };
    let view = |failure_site| OpView {
        evidence: &failed,
        dc: Some(&dc),
        devices: Some(&report),
        selection: None,
        mapping: &mapping,
        failure_site,
    };
    let at = egui::pos2(SIZE.x - NAME_END_PAD - 20.0, PROBLEM_ROW_H * 0.5);
    let offer = FailureSiteControl {
        named: 2,
        elided: 0,
        headline: "Did not converge",
        marked: false,
    };
    let responses = play(&plan, &view(Some(offer)), press_release(at));
    assert!(responses[1].highlight_failure_sites);
    let responses = play(&plan, &view(None), press_release(at));
    assert!(!responses[1].highlight_failure_sites);
}

#[test]
fn the_save_policy_is_mentioned_only_when_it_kept_a_device_off_the_sheet() {
    use OperatingPointDeviceDetailEvidence as Policy;

    let report = DeviceOpReport {
        entries: vec![
            device("M1", "MOSFET", None, vec![]),
            device("M2", "MOSFET", None, vec![]),
        ],
    };
    let note = |policy, selected: &[&str], violations: &[&str]| {
        let detail = RetainedDetail {
            policy,
            selected: selected.iter().map(|name| (*name).to_owned()).collect(),
            violations: violations.iter().map(|name| (*name).to_owned()).collect(),
        };
        let (sort, collapsed) = (OpSort::new(), BTreeSet::new());
        OpPlan::new(
            OpControls {
                filter: "",
                sort: &sort,
                collapsed: &collapsed,
            },
            None,
            Some(&report),
            Some(&detail),
        )
        .scope_note()
    };

    assert_eq!(note(Policy::AllDevices, &[], &[]), None);
    assert_eq!(
        note(Policy::SelectedAndViolations, &[], &[]),
        None,
        "with nothing picked and nothing violating, every device is kept"
    );
    assert_eq!(
        note(Policy::SelectedAndViolations, &["M1"], &["M2"]),
        None,
        "the two lists together name every device"
    );
    assert_eq!(
        note(Policy::SelectedAndViolations, &["M1"], &[]),
        Some("Only selected and violating devices were saved.")
    );
    assert_eq!(
        note(Policy::ViolationsOnly, &[], &[]),
        Some("Only violating devices were saved.")
    );
    assert_eq!(
        note(Policy::None, &[], &[]),
        Some("No per-device detail was saved for this run.")
    );
}

#[test]
fn the_readout_stops_short_of_floating_point_noise() {
    assert_eq!(quantity_readout(768.000000000001e-9, "S"), "768 nS");
}

#[test]
fn a_stale_schematic_greys_the_one_cross_probe_button_and_says_why_once() {
    let report = DeviceOpReport {
        entries: vec![device("M1", "MOSFET", None, vec![("id", 1e-3)])],
    };
    let selection = OpSelection {
        kind: OpRowKind::Device,
        index: 0,
        name: "M1".to_owned(),
    };
    let stale = Mapped {
        node: false,
        device: None,
    };
    let detail = selection_detail(&selection, None, Some(&report), &stale).expect("the row exists");

    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let input = rspice_ui_kit::raster::display_input(SIZE, 1.0);
        let output = ctx.run_ui(input, |ui| {
            assert_eq!(
                right_panel(ui, &evidence(), false, None, Some(&detail)),
                None
            );
        });
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    let named = |text: &str| {
        nodes
            .iter()
            .map(|(_, node)| node)
            .filter(|node| node.label().or(node.value()) == Some(text))
            .collect::<Vec<_>>()
    };
    let buttons = named("Show in schematic");
    assert_eq!(buttons.len(), 1);
    assert!(buttons[0].is_disabled());
    assert_eq!(named(SCHEMATIC_CHANGED).len(), 1);
}

const SIZE: egui::Vec2 = egui::vec2(900.0, 400.0);

fn frame(
    ctx: &egui::Context,
    plan: &OpPlan,
    view: &OpView<'_, Mapped>,
    events: Vec<egui::Event>,
) -> OpResponse {
    let mut input = rspice_ui_kit::raster::display_input(SIZE, 1.0);
    input.events = events;
    let mut response = None;
    let _ = ctx.run_ui(input, |ui| response = Some(show(ui, Some(plan), view)));
    response.expect("the frame ran")
}

fn press_release(at: egui::Pos2) -> Vec<Vec<egui::Event>> {
    let pointer = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    vec![
        vec![egui::Event::PointerMoved(at), pointer(true)],
        vec![pointer(false)],
    ]
}

/// Plays `gestures` a frame each over the rendered sheet, returning what
/// each frame reported.
fn play(
    plan: &OpPlan,
    view: &OpView<'_, Mapped>,
    gestures: Vec<Vec<egui::Event>>,
) -> Vec<OpResponse> {
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    frame(&ctx, plan, view, Vec::new());
    gestures
        .into_iter()
        .map(|events| frame(&ctx, plan, view, events))
        .collect()
}

fn evidence() -> OpEvidence {
    OpEvidence {
        success: true,
        error: None,
        facts: None,
    }
}

/// Mouse position over the first body row of the first section: title,
/// column header, then the row.
fn first_row() -> egui::Pos2 {
    egui::pos2(60.0, TITLE_H + COLUMN_HEADER_H + ROW_H * 0.5)
}

#[test]
fn a_click_picks_the_row_and_a_double_click_cross_probes_it() {
    let (dc, report) = evidence_fixture(4, 4);
    let plan = plan_of(Some(&dc), Some(&report));
    let mapping = Mapped {
        node: true,
        device: None,
    };
    let view = OpView {
        evidence: &evidence(),
        dc: Some(&dc),
        devices: Some(&report),
        selection: None,
        mapping: &mapping,
        failure_site: None,
    };
    let responses = play(&plan, &view, press_release(first_row()));
    let picked = responses[1]
        .selected
        .clone()
        .flatten()
        .expect("the click picked the row");
    assert_eq!(picked.kind, OpRowKind::Node);
    assert_eq!(picked.name, dc.node_voltages[picked.index].name);
    assert!(responses[1].action.is_none(), "one click only selects");

    let double = press_release(first_row())
        .into_iter()
        .chain(press_release(first_row()))
        .collect();
    let responses = play(&plan, &view, double);
    assert_eq!(
        responses[3].action,
        Some(OpAction::LocateNode(picked.name.clone())),
        "the second click of a pair goes to the schematic"
    );
}

#[test]
fn a_header_click_sorts_and_a_title_click_folds() {
    let (dc, report) = evidence_fixture(4, 4);
    let plan = plan_of(Some(&dc), Some(&report));
    let mapping = Mapped {
        node: true,
        device: None,
    };
    let view = OpView {
        evidence: &evidence(),
        dc: Some(&dc),
        devices: Some(&report),
        selection: None,
        mapping: &mapping,
        failure_site: None,
    };
    let header = egui::pos2(60.0, TITLE_H + COLUMN_HEADER_H * 0.5);
    let responses = play(&plan, &view, press_release(header));
    assert_eq!(
        responses[1].clicked_sort,
        Some(("Node voltages".to_owned(), "Node".to_owned()))
    );
    assert!(
        responses[1].selected.is_none(),
        "a header click reached the sheet behind it"
    );
    let title = egui::pos2(60.0, TITLE_H * 0.5);
    let responses = play(&plan, &view, press_release(title));
    assert_eq!(
        responses[1].toggled_section.as_deref(),
        Some("Node voltages")
    );
}

#[test]
fn clicking_past_the_last_row_drops_the_selection() {
    let (dc, report) = evidence_fixture(1, 0);
    let plan = plan_of(Some(&dc), None);
    let mapping = Mapped {
        node: true,
        device: None,
    };
    let view = OpView {
        evidence: &evidence(),
        dc: Some(&dc),
        devices: Some(&report),
        selection: None,
        mapping: &mapping,
        failure_site: None,
    };
    let responses = play(&plan, &view, press_release(egui::pos2(60.0, 380.0)));
    assert_eq!(responses[1].selected, Some(None));
}

#[test]
fn a_failed_solve_says_so_above_the_rows() {
    let (dc, report) = evidence_fixture(2, 2);
    let plan = plan_of(Some(&dc), Some(&report));
    let mapping = Mapped {
        node: true,
        device: None,
    };
    let failed = OpEvidence {
        success: false,
        error: Some("Newton did not converge at node out".to_owned()),
        ..evidence()
    };
    let view = OpView {
        evidence: &failed,
        dc: Some(&dc),
        devices: Some(&report),
        selection: None,
        mapping: &mapping,
        failure_site: None,
    };
    let row = egui::pos2(
        60.0,
        PROBLEM_ROW_H + TITLE_H + COLUMN_HEADER_H + ROW_H * 0.5,
    );
    let responses = play(&plan, &view, press_release(row));
    assert!(
        responses[1].selected.clone().flatten().is_some(),
        "the problem row took more than its one line"
    );
}
