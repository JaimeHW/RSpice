//! The netlist navigator projects live counts, never a cached outline.
//!
//! These fix the arithmetic: every declaration a group counts is reachable
//! from the tree or the index, a collapsed group still stands for what it
//! hides, and a section is found by the name the active locale actually drew
//! rather than by its English key.

use super::*;

fn netlist_index(source: &str) -> crate::state::NetlistSourceIndex {
    crate::state::NetlistSourceIndex::parse(source)
}

fn english() -> MessageCatalog {
    MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
}

fn netlist_projection(source: &str, query: &str) -> NetlistNavigatorProjection {
    NetlistNavigatorProjection::from_index(
        &netlist_index(source),
        query,
        "top.sp",
        true,
        &std::collections::BTreeSet::new(),
        &[],
        english(),
    )
}

/// Every declaration a group discloses, named the way a drawn row is.
fn disclosed(
    group: &NetlistOutlineGroup,
    index: &crate::state::NetlistSourceIndex,
) -> Vec<NetlistOutlineChild> {
    (0..group.declarations())
        .filter_map(|position| group.child(position, index))
        .collect()
}

const OUTLINE_DECK: &str = "Precision amplifier\n.include models/base.lib\n.lib corners/process.lib TT\n.param gain=10 offset=1m\nR1 in out 1k\nXAMP in out opamp\n.model nch nmos\n.ac dec 10 1 1g\n.meas ac peak max v(out)\n.end\n";

#[test]
fn netlist_navigator_projects_exact_live_counts_and_include_lines() {
    let index = netlist_index(OUTLINE_DECK);
    let projection = netlist_projection(OUTLINE_DECK, "");

    assert_eq!(projection.line_count, 10);
    let count = |kind| {
        projection
            .groups
            .iter()
            .find(|group| group.row.kind == kind)
            .and_then(|group| group.row.meta.as_deref())
    };
    assert_eq!(
        projection
            .root_row
            .as_ref()
            .and_then(|row| row.meta.as_deref()),
        Some("root")
    );
    assert_eq!(count(NetlistNavigatorRowKind::Parameters), Some("1"));
    assert_eq!(count(NetlistNavigatorRowKind::Instances), Some("2"));
    let instances = projection
        .groups
        .iter()
        .find(|group| group.row.kind == NetlistNavigatorRowKind::Instances)
        .expect("instances group exists");
    assert!(instances.contains_line(&index, 5));
    assert!(instances.contains_line(&index, 6));
    // The caret between two declarations belongs to neither.
    assert!(!instances.contains_line(&index, 7));
    assert_eq!(count(NetlistNavigatorRowKind::Models), Some("1"));
    assert_eq!(count(NetlistNavigatorRowKind::Analyses), Some("1"));
    assert_eq!(count(NetlistNavigatorRowKind::Measurements), Some("1"));
    assert_eq!(projection.include_rows.len(), 2);
    assert_eq!(projection.include_rows[0].label, "models/base.lib");
    assert_eq!(projection.include_rows[0].target_line, Some(2));
    assert_eq!(projection.include_rows[1].label, "corners/process.lib");
    assert_eq!(projection.include_rows[1].target_line, Some(3));
    assert!(projection.show_source_mapping);
}

#[test]
fn an_include_row_names_the_fate_of_its_own_dependency() {
    let states = [
        IncludeRowFacts {
            locator: "models/base.lib".to_owned(),
            state: MessageId::NetlistNavigatorDependencyMissing,
            via: None,
            chain: None,
            shadowed_by: None,
            sections: Vec::new(),
        },
        IncludeRowFacts {
            locator: "corners/process.lib".to_owned(),
            state: MessageId::NetlistNavigatorAuthorityVendor,
            via: None,
            chain: None,
            shadowed_by: None,
            sections: Vec::new(),
        },
    ];
    let projection = NetlistNavigatorProjection::from_index(
        &netlist_index(OUTLINE_DECK),
        "",
        "top.sp",
        true,
        &std::collections::BTreeSet::new(),
        &states,
        english(),
    );

    // The header states one verdict for the whole closure; the row that
    // earned it has to be identifiable.
    assert_eq!(projection.include_rows[0].meta.as_deref(), Some("missing"));
    assert_eq!(
        projection.include_rows[1].meta.as_deref(),
        Some("vendor source")
    );

    // A locator the closure never retained claims no fate at all.
    let unknown = netlist_projection(OUTLINE_DECK, "");
    assert_eq!(unknown.include_rows[0].meta.as_deref(), Some("line 2"));
}

#[test]
fn every_outline_group_discloses_the_declarations_it_counts() {
    let index = netlist_index(OUTLINE_DECK);
    let projection = netlist_projection(OUTLINE_DECK, "");

    let children = |kind| {
        projection
            .groups
            .iter()
            .find(|group| group.row.kind == kind)
            .map(|group| {
                disclosed(group, &index)
                    .into_iter()
                    .map(|child| (child.label, child.meta, child.line))
                    .collect::<Vec<_>>()
            })
            .expect("group exists")
    };
    let expected = |rows: &[(&str, Option<&str>, usize)]| {
        rows.iter()
            .map(|(label, meta, line)| ((*label).to_owned(), meta.map(str::to_owned), *line))
            .collect::<Vec<_>>()
    };

    // A `.param` card that declares two names attributes no value to
    // either, because the row would otherwise give both the first one's.
    assert_eq!(
        children(NetlistNavigatorRowKind::Parameters),
        expected(&[("gain, offset", None, 4)])
    );
    // A resistor's value is positional; a subcircuit call's master is the
    // last positional field. Both are exact from the element letter.
    assert_eq!(
        children(NetlistNavigatorRowKind::Instances),
        expected(&[("R1", Some("1k"), 5), ("XAMP", Some("opamp"), 6)])
    );
    assert_eq!(
        children(NetlistNavigatorRowKind::Models),
        expected(&[("nch", Some("nmos"), 7)])
    );
    assert_eq!(
        children(NetlistNavigatorRowKind::Analyses),
        expected(&[(".ac", Some("dec 10 1 1g"), 8)])
    );
    assert_eq!(
        children(NetlistNavigatorRowKind::Measurements),
        expected(&[("peak", Some("ac"), 9)])
    );
}

#[test]
fn a_collapsed_group_keeps_its_count_and_stands_in_for_its_declarations() {
    let index = netlist_index(OUTLINE_DECK);
    let collapsed = std::collections::BTreeSet::from([crate::state::OutlineSectionKind::Devices]);
    let projection = NetlistNavigatorProjection::from_index(
        &index,
        "",
        "top.sp",
        true,
        &collapsed,
        &[],
        english(),
    );

    let instances = projection
        .groups
        .iter()
        .find(|group| group.row.kind == NetlistNavigatorRowKind::Instances)
        .expect("instances group exists");
    assert!(!instances.expanded);
    assert_eq!(instances.row.meta.as_deref(), Some("2"));
    // A collapsed group draws no declarations but still answers for them,
    // which is how it stands in for the one holding the caret.
    assert_eq!(instances.declarations(), 2);
    assert!(instances.contains_line(&index, 6));
}

#[test]
fn every_parsed_category_is_reachable_from_the_structure_tree_or_the_index() {
    let source = "deck\n.global vdd\n.func square(x) {x*x}\n.options reltol=1e-5\n.subckt amp in out\nM1 out in 0 0 nch\n.ends amp\n.if corner\n.save v(out)\n.endif\n.end\n";
    let projection = netlist_projection(source, "");

    let index = projection
        .semantic_rows
        .iter()
        .map(|row| (row.label.as_str(), row.meta.as_str(), row.line))
        .collect::<Vec<_>>();
    assert_eq!(
        index,
        vec![
            // `.ends` shares the section but is not a definition.
            ("Hierarchy", "1 definition", 5),
            ("Globals", "1 declaration", 2),
            ("Functions", "1 definition", 3),
            ("Solver options", "1 card", 4),
            ("Save and probe", "1 directive", 9),
            ("Conditionals", "2 cards", 8),
            ("Control", "1 directive", 11),
        ]
    );
    // The header is the sum of the rows beneath it, so a reader who adds
    // them up gets the number the section claims.
    assert_eq!(
        projection.semantic_cards,
        index.len() + 1,
        "seven categories, one of which counts two conditionals"
    );
    assert_eq!(projection.semantic_cards, 8);
}

#[test]
fn a_category_the_deck_does_not_declare_is_left_out_of_the_index() {
    let projection = netlist_projection("deck\nR1 in out 1k\n.end\n", "");

    assert!(
        projection
            .semantic_rows
            .iter()
            .all(|row| row.label != "Conditionals"),
        "an absent category must not be listed as present with a zero"
    );
    // A structure group stays, because it says in its own place what the
    // deck does not declare.
    assert!(projection.groups.iter().any(|group| group.row.kind
        == NetlistNavigatorRowKind::Measurements
        && group.declarations() == 0
        && !english().text(group.empty_note).is_empty()));
}

/// The filter used to be compared against the English label behind a
/// section rather than the label the section draws, so a translated
/// navigator could not be searched by the names on its own rows.
#[test]
fn a_section_is_found_by_the_name_it_draws_in_the_locale_that_drew_it() {
    for locale in crate::workbench::UiTextLocale::ALL {
        let messages = MessageCatalog::new(locale);
        let drawn = messages.text(MessageId::NetlistNavigatorParameters);
        let projection = NetlistNavigatorProjection::from_index(
            &netlist_index(OUTLINE_DECK),
            &drawn,
            "top.sp",
            true,
            &std::collections::BTreeSet::new(),
            &[],
            messages,
        );
        assert!(
            projection
                .groups
                .iter()
                .any(|group| group.row.label == drawn),
            "filtering by {drawn:?} found no group in {locale:?}"
        );

        // The provenance panel is disclosed by typing its own heading, and
        // that heading was two English keywords compared against the query.
        let heading = messages.text(MessageId::NetlistNavigatorSourceMapping);
        let projection = NetlistNavigatorProjection::from_index(
            &netlist_index(OUTLINE_DECK),
            &heading,
            "top.sp",
            true,
            &std::collections::BTreeSet::new(),
            &[],
            messages,
        );
        assert!(
            projection.show_source_mapping,
            "filtering by {heading:?} hid the section it names in {locale:?}"
        );
    }
}

#[test]
fn netlist_navigator_filter_matches_symbols_and_exact_source_lines() {
    let source = "deck\n.param gain=10\nR1 in out 1k\nR2 out 0 2k\n.end\n";
    let index = netlist_index(source);

    let symbol = netlist_projection(source, "r2");
    assert!(symbol.root_row.is_none());
    assert_eq!(symbol.groups.len(), 1);
    assert_eq!(
        symbol.groups[0].row.kind,
        NetlistNavigatorRowKind::Instances
    );
    // A filtered count that showed the total would read as "this is
    // everything the deck declares".
    assert_eq!(symbol.groups[0].row.meta.as_deref(), Some("1 of 2"));
    assert_eq!(symbol.groups[0].row.target_line, Some(4));
    assert!(!symbol.groups[0].contains_line(&index, 3));
    assert!(symbol.groups[0].contains_line(&index, 4));
    assert!(!symbol.show_source_mapping);

    let line = netlist_projection(source, "line 2");
    assert_eq!(line.groups.len(), 1);
    assert_eq!(line.groups[0].row.kind, NetlistNavigatorRowKind::Parameters);
    assert_eq!(line.groups[0].row.target_line, Some(2));
}

#[test]
fn a_filter_discloses_what_it_kept_even_where_the_group_was_collapsed() {
    let index = netlist_index("deck\nR1 in out 1k\nR2 out 0 2k\n.end\n");
    let collapsed = std::collections::BTreeSet::from([crate::state::OutlineSectionKind::Devices]);
    let projection = NetlistNavigatorProjection::from_index(
        &index,
        "r2",
        "top.sp",
        true,
        &collapsed,
        &[],
        english(),
    );

    assert_eq!(projection.groups.len(), 1);
    assert!(projection.groups[0].expanded);
    let kept = disclosed(&projection.groups[0], &index);
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].label, "R2");
}

#[test]
fn an_instance_binding_is_named_only_where_the_element_letter_fixes_it() {
    let source =
        "deck\nM1 d g s b nch W=1u\nD1 a k dmod\nQ1 c b e qmod\nV1 in 0 DC 1.8\nD2 a k\n.end\n";
    let index = netlist_index(source);
    let projection = netlist_projection(source, "");

    let instances = disclosed(
        projection
            .groups
            .iter()
            .find(|group| group.row.kind == NetlistNavigatorRowKind::Instances)
            .expect("instances group exists"),
        &index,
    );
    let binding = |name: &str| {
        instances
            .iter()
            .find(|child| child.label == name)
            .and_then(|child| child.meta.clone())
    };
    assert_eq!(binding("M1").as_deref(), Some("nch"));
    assert_eq!(binding("D1").as_deref(), Some("dmod"));
    assert_eq!(binding("Q1").as_deref(), Some("qmod"));
    // A source's argument list is not a model name.
    assert_eq!(binding("V1"), None);
    // A diode short of its model card must not report its cathode as one.
    assert_eq!(binding("D2"), None);
}

#[test]
fn a_spaced_assignment_is_not_read_as_one_more_positional_field() {
    let source = "deck\nM1 d g s b nch W = 1u\n.param gain = 10\n.end\n";
    let index = netlist_index(source);
    let projection = netlist_projection(source, "");

    let child = |kind| {
        projection
            .groups
            .iter()
            .find(|group| group.row.kind == kind)
            .and_then(|group| group.child(0, &index))
            .expect("child exists")
    };
    assert_eq!(
        child(NetlistNavigatorRowKind::Instances).meta.as_deref(),
        Some("nch")
    );
    let parameter = child(NetlistNavigatorRowKind::Parameters);
    assert_eq!(parameter.label, "gain");
    assert_eq!(parameter.meta.as_deref(), Some("10"));
}

#[test]
fn an_expression_split_across_lexemes_is_named_rather_than_misquoted() {
    let source = "deck\n.param gain = {2 * k}\n.param trim={2*k}\nR9 a b {1 * k}\n.end\n";
    let index = netlist_index(source);
    let projection = netlist_projection(source, "");

    let parameters = disclosed(
        projection
            .groups
            .iter()
            .find(|group| group.row.kind == NetlistNavigatorRowKind::Parameters)
            .expect("parameters group exists"),
        &index,
    );
    // `{2` is not the value of anything.
    assert_eq!(parameters[0].label, "gain");
    assert_eq!(parameters[0].meta, None);
    // The same expression written without spaces survives whole.
    assert_eq!(parameters[1].label, "trim");
    assert_eq!(parameters[1].meta.as_deref(), Some("{2*k}"));

    let instance = projection
        .groups
        .iter()
        .find(|group| group.row.kind == NetlistNavigatorRowKind::Instances)
        .and_then(|group| group.child(0, &index))
        .expect("instances group exists");
    assert_eq!(instance.label, "R9");
    assert_eq!(instance.meta, None);
}

#[test]
fn a_comparison_navigator_lists_the_regions_that_changed() {
    let diff = "--- generated-aaaa\n+++ generated-bbbb\n@@ -1,4 +1,5 @@\n deck\n-R1 in out 1k\n+R1 in out 2k\n+R2 out 0 1k\n .end\n@@ -9,3 +10,3 @@\n .ac dec 10 1 1g\n-.meas ac peak max v(out)\n+.meas ac peak min v(out)\n";
    let hunks = diff_hunks(diff, english());

    assert_eq!(hunks.len(), 2);
    assert_eq!(hunks[0].label, "Lines 1\u{2013}5");
    assert_eq!(hunks[0].meta, "+2 -1");
    // The row navigates to the header inside the comparison document,
    // because that is the buffer the editor is showing.
    assert_eq!(hunks[0].line, 3);
    assert_eq!(hunks[1].label, "Lines 10\u{2013}12");
    assert_eq!(hunks[1].meta, "+1 -1");
    assert_eq!(hunks[1].line, 9);
    // A hunk owns the comparison lines up to the next header, so the row
    // can say which region the caret is standing in.
    assert_eq!(hunks[0].end_line, 8);
    assert_eq!(hunks[1].end_line, 12);
    // The `---`/`+++` header is not a change.
    assert_eq!(diff_totals(diff), (3, 2));
}

#[test]
fn identical_revisions_produce_no_changed_regions() {
    assert!(
        diff_hunks(
            "--- owned-r1-aaaa\n+++ owned-r2-aaaa\n No source changes\n",
            english()
        )
        .is_empty()
    );
}

#[test]
fn a_viewport_draws_only_the_declarations_it_can_show() {
    // 27 px rows in a 540 px panel: twenty rows and the partly scrolled
    // one at each edge, not the fifty thousand a flat deck can declare.
    let span = visible_row_span(0.0, 540.0, 27.0, 50_000);
    assert_eq!(span.start, 0);
    assert!(span.end <= 21, "drew {} rows for 540 px", span.end);

    // Scrolled: the first row is above the viewport by 1000 px.
    let scrolled = visible_row_span(-1000.0, 540.0, 27.0, 50_000);
    assert_eq!(scrolled.start, 37);
    assert!(scrolled.end >= 58 && scrolled.end <= 59, "{scrolled:?}");

    // Fewer rows than the viewport holds, and none at all.
    assert_eq!(visible_row_span(0.0, 540.0, 27.0, 4), 0..4);
    assert_eq!(visible_row_span(0.0, 540.0, 27.0, 0), 0..0);
    // Scrolled past the end: an empty span, never a reversed range.
    let past = visible_row_span(-10_000.0, 540.0, 27.0, 10);
    assert!(past.start <= past.end);
}

/// A flat deck of a hundred thousand cards, projected the way a frame
/// projects it.
///
/// The whole outline used to be reparsed and every declaration named on
/// every frame, which cost a tenth of a second per frame here. The budget
/// is deliberately loose — it is sized to catch work that scales with the
/// deck coming back, not to police a few hundred microseconds.
#[test]
fn a_hundred_thousand_card_deck_projects_within_a_frame() {
    let mut source = String::from("* flat deck\n");
    for card in 0..100_000 {
        match card % 4 {
            0 => source.push_str(&format!(".param k{card}={{{card} * 2}}\n")),
            1 => source.push_str(&format!("R{card} n{card} n{} 1k\n", card + 1)),
            2 => source.push_str(&format!("M{card} d{card} g{card} 0 0 nch W=1u\n")),
            _ => source.push_str(&format!(".model m{card} nmos level=54\n")),
        }
    }
    source.push_str(".op\n.end\n");
    let index = netlist_index(&source);

    let started = crate::time_compat::Instant::now();
    let projection = NetlistNavigatorProjection::from_index(
        &index,
        "",
        "flat.sp",
        false,
        &std::collections::BTreeSet::new(),
        &[],
        english(),
    );
    let devices = projection
        .groups
        .iter()
        .find(|group| group.section == OutlineSectionKind::Devices)
        .expect("device group exists");
    // What a scrolled viewport asks for, and nothing else.
    let drawn = (37..58)
        .filter_map(|position| devices.child(position, &index))
        .collect::<Vec<_>>();
    // The group standing in for the caret searches its declarations.
    let caret = devices.contains_line(&index, 100_000);
    let elapsed = started.elapsed();

    assert_eq!(devices.declarations(), 50_000);
    assert_eq!(drawn.len(), 21);
    assert_eq!(drawn[0].label, "M74");
    assert!(caret);
    assert!(
        elapsed < std::time::Duration::from_millis(50),
        "a frame spent {elapsed:?} projecting a deck it did not reparse"
    );
}

#[test]
fn netlist_navigator_geometry_matches_mockup_and_touch_contract() {
    assert_eq!(NETLIST_OUTLINE_ROW_HEIGHT, 27.0);
    assert_eq!(NETLIST_OUTLINE_TOUCH_ROW_HEIGHT, 44.0);
    assert_eq!(NETLIST_OUTLINE_PADDING_X, 9.0);
    assert_eq!(NETLIST_OUTLINE_ICON_GAP, 7.0);
    // A declaration hangs off its group's guide line, and every top-level
    // row reserves the caret column so their icons stay in one line.
    assert_eq!(
        NetlistOutlineRowShape::Leaf.label_left(),
        NetlistOutlineRowShape::Group { expanded: true }.label_left()
    );
    assert!(NetlistOutlineRowShape::Child.label_left() > NetlistOutlineRowShape::Leaf.label_left());
    assert_eq!(
        NetlistOutlineRowShape::Index.label_left(),
        NETLIST_OUTLINE_PADDING_X
    );
}

/// An include row states where the file came from, and the stage it names is
/// the resolver's own, not a word this surface invented.
#[test]
fn an_include_row_states_the_chain_stage_it_resolved_through() {
    let source = "trace deck\n.include models.lib\n.end\n";
    let messages = english();
    for (stage, expected) in [
        (
            rspice_core::netlist::IncludeSearchStage::IncludingFile,
            "via including-file dir",
        ),
        (
            rspice_core::netlist::IncludeSearchStage::LibraryPath(1),
            "via search path 2",
        ),
        (
            rspice_core::netlist::IncludeSearchStage::Conventional("lib"),
            "via lib/",
        ),
    ] {
        let facts = IncludeRowFacts {
            locator: "models.lib".to_owned(),
            state: MessageId::NetlistNavigatorAuthorityExternal,
            via: Some(include_stage_text(messages, stage)),
            chain: Some("chain".to_owned()),
            shadowed_by: None,
            sections: Vec::new(),
        };
        let projection = NetlistNavigatorProjection::from_index(
            &netlist_index(source),
            "",
            "top.sp",
            true,
            &std::collections::BTreeSet::new(),
            std::slice::from_ref(&facts),
            messages,
        );
        let row = projection
            .include_rows
            .first()
            .expect("the deck writes one include");
        let meta = row
            .meta
            .as_deref()
            .expect("an include row states its state");
        assert_eq!(
            meta, expected,
            "a 312 px row states one short phrase, not a sentence"
        );
        assert!(
            row.tooltip
                .as_deref()
                .is_some_and(|tooltip| tooltip.starts_with("external reference")),
            "the authority the meta gave up belongs in the tooltip"
        );
        assert!(!row.shadowed);
    }
}

#[test]
fn a_shadowed_include_row_carries_a_warning_marker_and_names_the_other_file() {
    let source = "shadow deck\n.include models.lib\n.end\n";
    let messages = english();
    let facts = IncludeRowFacts {
        locator: "models.lib".to_owned(),
        state: MessageId::NetlistNavigatorAuthorityExternal,
        via: Some(include_stage_text(
            messages,
            rspice_core::netlist::IncludeSearchStage::LibraryPath(0),
        )),
        chain: Some("Search chain for models.lib".to_owned()),
        shadowed_by: Some("/opt/pdk/second/models.lib".to_owned()),
        sections: Vec::new(),
    };
    let projection = NetlistNavigatorProjection::from_index(
        &netlist_index(source),
        "",
        "top.sp",
        true,
        &std::collections::BTreeSet::new(),
        std::slice::from_ref(&facts),
        messages,
    );
    let row = projection
        .include_rows
        .first()
        .expect("the deck writes one include");
    assert!(row.shadowed, "the row must earn the warning tone");
    assert_eq!(
        row.meta.as_deref(),
        Some(
            messages
                .text(MessageId::NetlistIncludeShadowMarker)
                .as_str()
        ),
        "the warning replaces the state word rather than crowding the name out"
    );
    assert!(
        row.tooltip
            .as_deref()
            .is_some_and(|tooltip| tooltip.contains("/opt/pdk/second/models.lib")),
        "the shadowing path belongs in the tooltip, not on the row"
    );
}

/// A locator the closure never retained claims nothing about where it came
/// from; it says so instead of showing an empty chain.
#[test]
fn an_untraced_include_row_says_no_chain_was_walked() {
    let projection = netlist_projection("untraced deck\n.include models.lib\n.end\n", "");
    let row = projection
        .include_rows
        .first()
        .expect("the deck writes one include");
    assert!(!row.shadowed);
    assert_eq!(
        row.tooltip.as_deref(),
        Some(
            english()
                .text(MessageId::NetlistIncludeChainUntraced)
                .as_str()
        )
    );
}

/// A three-corner library, the way a foundry corner file is shaped: the deck
/// binds one of these by name and the row has to say which.
fn corner_library() -> &'static str {
    "* corners\n\
     .lib tt\n\
     .model nch NMOS (LEVEL=1)\n\
     .model pch PMOS (LEVEL=1)\n\
     .subckt esd a b\n\
     .ends esd\n\
     .endl tt\n\
     .lib ff\n\
     .model nch NMOS (LEVEL=1)\n\
     .endl ff\n\
     .lib ss\n\
     .model nch NMOS (LEVEL=1)\n\
     .endl ss\n"
}

/// What the retained bytes above declare, read through the engine rather than
/// spelled out here, so the fixture and the row can never drift apart.
fn corner_sections() -> Vec<rspice_core::library::LibSectionSummary> {
    rspice_core::library::enumerate_lib_sections(corner_library().as_bytes())
}

fn library_facts(locator: &str) -> IncludeRowFacts {
    IncludeRowFacts {
        locator: locator.to_owned(),
        state: MessageId::NetlistNavigatorAuthorityVendor,
        via: Some(include_stage_text(
            english(),
            rspice_core::netlist::IncludeSearchStage::IncludingFile,
        )),
        chain: Some("Search chain".to_owned()),
        shadowed_by: None,
        sections: corner_sections(),
    }
}

fn library_projection(source: &str, locator: &str) -> NetlistNavigatorProjection {
    NetlistNavigatorProjection::from_index(
        &netlist_index(source),
        "",
        "top.sp",
        true,
        &std::collections::BTreeSet::new(),
        std::slice::from_ref(&library_facts(locator)),
        english(),
    )
}

#[test]
fn a_library_include_row_states_the_section_its_own_card_binds() {
    let projection = library_projection(
        "corner deck\n.lib \"corners.lib\" ff\n.end\n",
        "corners.lib",
    );
    let row = projection
        .include_rows
        .first()
        .expect("the deck writes one library include");

    assert_eq!(
        row.meta.as_deref(),
        Some("section ff"),
        "which corner is bound outranks where the file was found"
    );
    let choice = row
        .sections
        .as_ref()
        .expect("a sectioned library offers a choice");
    assert_eq!(choice.selected.as_deref(), Some("ff"));
    assert_eq!(choice.line, 2, "the rewrite edits the card's own line");
    assert_eq!(
        choice
            .available
            .iter()
            .map(|section| section.name.as_str())
            .collect::<Vec<_>>(),
        ["tt", "ff", "ss"],
        "every section the retained bytes declare is offered, in file order"
    );

    // The row has space for one phrase; the catalog and the counts are the
    // tooltip's job, with the bound section marked.
    let tooltip = row
        .tooltip
        .as_deref()
        .expect("an include row explains itself");
    assert!(tooltip.contains("Sections in corners.lib"), "{tooltip}");
    assert!(tooltip.contains("tt · 2 models · 1 subckt"), "{tooltip}");
    assert!(
        tooltip.contains("ff · 1 model  — bound by this deck"),
        "{tooltip}"
    );
    assert!(
        !tooltip.contains("ss · 1 model  —"),
        "only the bound section carries the marker: {tooltip}"
    );
}

#[test]
fn a_library_included_without_a_section_says_so_and_counts_the_alternatives() {
    let projection = library_projection("corner deck\n.include corners.lib\n.end\n", "corners.lib");
    let row = projection
        .include_rows
        .first()
        .expect("the deck writes one include");

    assert_eq!(
        row.meta.as_deref(),
        Some("no section · 3 available"),
        "a .lib file pulled in by .include binds nothing, and the row must \
         not imply that it does"
    );
    let choice = row
        .sections
        .as_ref()
        .expect("the file still declares sections");
    assert_eq!(choice.selected, None);
    assert_eq!(choice.available.len(), 3);
}

#[test]
fn an_include_of_a_file_without_sections_gains_nothing() {
    let facts = IncludeRowFacts {
        sections: Vec::new(),
        ..library_facts("models.lib")
    };
    let projection = NetlistNavigatorProjection::from_index(
        &netlist_index("plain deck\n.include models.lib\n.end\n"),
        "",
        "top.sp",
        true,
        &std::collections::BTreeSet::new(),
        std::slice::from_ref(&facts),
        english(),
    );
    let row = projection
        .include_rows
        .first()
        .expect("the deck writes one include");

    assert!(row.sections.is_none(), "no sections, no section action");
    assert_eq!(
        row.meta.as_deref(),
        Some("via including-file dir"),
        "a file with no sections keeps stating where it came from"
    );
    assert!(
        row.tooltip
            .as_deref()
            .is_some_and(|tooltip| !tooltip.contains("Sections in")),
        "nothing to catalog, nothing added"
    );
}

/// Offscreen renders of the include rows at the dock's own width, so the trace
/// and the shadow marker can be looked at rather than only asserted about.
///
/// Run the PNG writer with `--ignored`; renders go to `RSPICE_RASTER_DIR`.
mod include_row_raster {
    use super::*;

    /// The navigator dock's authored width.
    const DOCK_WIDTH: f32 = 312.0;

    fn facts(shadowed: bool) -> Vec<IncludeRowFacts> {
        let messages = english();
        vec![
            IncludeRowFacts {
                locator: "models.lib".to_owned(),
                state: MessageId::NetlistNavigatorAuthorityVendor,
                via: Some(include_stage_text(
                    messages,
                    rspice_core::netlist::IncludeSearchStage::LibraryPath(1),
                )),
                chain: Some("Search chain for models.lib".to_owned()),
                shadowed_by: shadowed.then(|| "/opt/pdk/second/models.lib".to_owned()),
                sections: Vec::new(),
            },
            IncludeRowFacts {
                locator: "corners.lib".to_owned(),
                state: MessageId::NetlistNavigatorAuthorityExternal,
                via: Some(include_stage_text(
                    messages,
                    rspice_core::netlist::IncludeSearchStage::IncludingFile,
                )),
                chain: Some("Search chain for corners.lib".to_owned()),
                shadowed_by: None,
                sections: corner_sections(),
            },
            IncludeRowFacts {
                locator: "passives.lib".to_owned(),
                state: MessageId::NetlistNavigatorAuthorityTechnology,
                via: Some(include_stage_text(
                    messages,
                    rspice_core::netlist::IncludeSearchStage::TopLevel,
                )),
                chain: Some("Search chain for passives.lib".to_owned()),
                shadowed_by: None,
                sections: corner_sections(),
            },
        ]
    }

    /// The deck binds one library by section and includes another without
    /// naming one, so both `.lib` states appear in the same render.
    const RASTER_DECK: &str = "raster deck\n\
         .include models.lib\n\
         .lib \"corners.lib\" tt\n\
         .include passives.lib\n\
         .end\n";

    fn render(shadowed: bool) -> crate::ui::raster::Canvas {
        render_at(shadowed, DOCK_WIDTH)
    }

    fn render_at(shadowed: bool, width: f32) -> crate::ui::raster::Canvas {
        let projection = NetlistNavigatorProjection::from_index(
            &netlist_index(RASTER_DECK),
            "",
            "top.sp",
            true,
            &std::collections::BTreeSet::new(),
            &facts(shadowed),
            english(),
        );
        crate::ui::raster::render(egui::vec2(width, 160.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    for row in &projection.include_rows {
                        netlist_outline_row(
                            ui,
                            OutlineRowVisual {
                                label: &row.label,
                                meta: row.meta.as_deref(),
                                meta_tone: row.shadowed.then(|| Tokens::get(ui.ctx()).color.warn),
                                icon: Some(netlist_outline_icon(row.kind)),
                                shape: NetlistOutlineRowShape::Leaf,
                                selected: false,
                                enabled: true,
                                height: NETLIST_OUTLINE_ROW_HEIGHT,
                            },
                        );
                    }
                });
        })
    }

    /// A shadowed row is not the same picture as an ordinary one: it carries a
    /// marker in the warning tone rather than only a state word.
    #[test]
    fn a_shadowed_row_is_painted_differently_from_a_resolved_one() {
        let plain = render(false);
        let shadowed = render(true);
        let region = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(DOCK_WIDTH, 54.0));
        assert_ne!(
            plain.pixels_in(region).collect::<Vec<_>>(),
            shadowed.pixels_in(region).collect::<Vec<_>>(),
            "the shadow marker must change what the row paints"
        );
    }

    /// The chooser paints the deck's own alternatives, so its rows are the
    /// menu's rows rather than a second drawing of the same list.
    fn render_chooser(editable: bool, width: f32) -> crate::ui::raster::Canvas {
        let choice = IncludeSectionChoice {
            line: 3,
            selected: Some("tt".to_owned()),
            available: corner_sections(),
        };
        crate::ui::raster::render(egui::vec2(width, 120.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    section_choice_entries(ui, &choice, editable, english(), &mut None);
                });
        })
    }

    /// A read-only document must not look like an editable one: every entry is
    /// disabled, which is the only honest way to keep showing what the library
    /// declares.
    #[test]
    fn a_read_only_chooser_is_painted_differently_from_an_editable_one() {
        let editable = render_chooser(true, DOCK_WIDTH);
        let read_only = render_chooser(false, DOCK_WIDTH);
        let region = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(DOCK_WIDTH, 100.0));
        assert_ne!(
            editable.pixels_in(region).collect::<Vec<_>>(),
            read_only.pixels_in(region).collect::<Vec<_>>(),
            "a chooser that cannot be used must say so in what it paints"
        );
    }

    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_include_rows() {
        use std::io::Write as _;

        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        let stderr = std::io::stderr();
        let mut report = stderr.lock();
        let mut write = |name: String, canvas: &crate::ui::raster::Canvas| {
            let content = canvas.content_height().max(1);
            let bytes = canvas.png(content);
            let path = directory.join(name);
            std::fs::write(&path, &bytes).expect("write include row render");
            writeln!(
                report,
                "{} {}x{} {} bytes",
                path.display(),
                canvas.width(),
                content,
                bytes.len()
            )
            .expect("write raster report");
        };
        for width in [DOCK_WIDTH, 1600.0] {
            let stage = width as u32;
            for (label, shadowed) in [("resolved", false), ("shadowed", true)] {
                write(
                    format!("netlist-navigator-include-{label}-{stage}.png"),
                    &render_at(shadowed, width),
                );
            }
            for (label, editable) in [("editable", true), ("read-only", false)] {
                write(
                    format!("netlist-navigator-section-chooser-{label}-{stage}.png"),
                    &render_chooser(editable, width),
                );
            }
        }
    }
}
