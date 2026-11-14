//! The project symbol index, value hovers, semantic rename and quick fixes,
//! over the authenticated source closure.

use super::*;
use crate::workbench::code::tests::{CodeHarness, harness};

#[test]
fn project_index_resolves_cross_file_definitions_and_references_case_insensitively() {
    let root = "root\n.include \"cell.inc\"\n.param gain=2\n.func shape(x)={x*gain}\nRgain in 0 gain\nX1 in out cell gain={shape(gain)}\n.end\n";
    let include = ".subckt CELL in out params: gain=1\n.param local=2\nXBUF in out leaf\n.ends CELL\n.subckt LEAF in out\nR1 in out 1k\n.ends LEAF\n";
    let locator = crate::state::SourceLocator::try_new("cell.inc", "cell.inc").unwrap();
    let dependency = crate::state::DependencyMetadata::unresolved_direct_to(0, "cell.inc", locator)
        .unwrap()
        .resolve_utf8(include.as_bytes().to_vec())
        .unwrap();
    let provenance = crate::state::GeneratedProvenance::try_new(
        "language-index-test",
        crate::state::GenerationInput::new(
            crate::product::ObjectRevision::INITIAL,
            crate::state::content_digest("language-index-input"),
        ),
    )
    .unwrap();
    let artifact = crate::state::GeneratedArtifact::try_from_utf8(
        provenance,
        root.as_bytes().to_vec(),
        vec![dependency],
        Vec::new(),
    )
    .unwrap();
    let document = crate::state::NetlistDocument::from_generated(
        crate::state::NetlistDocumentId::new(),
        artifact,
    )
    .unwrap();

    let index = NetlistProjectIndex::build(ActiveNetlistDocument::Generated, &document);

    let definitions = index.definitions_named("cell");
    assert_eq!(definitions.len(), 1);
    assert_eq!(
        definitions[0].definition.dependency_identity.as_deref(),
        Some("cell.inc")
    );
    assert_eq!(
        index.references_named("CELL").len(),
        2,
        "the X instance and matching .ends card are both semantic references"
    );
    assert_eq!(index.hierarchy().len(), 2);
    assert_eq!(index.hierarchy()[0].path, "X1");
    assert_eq!(index.hierarchy()[0].target, "cell");
    assert_eq!(index.hierarchy()[1].path, "X1/XBUF");
    assert_eq!(index.hierarchy()[1].target, "leaf");
    assert_eq!(index.source_count(), 2);
}

#[test]
fn verilog_a_sources_stay_out_of_the_spice_project_index() {
    let root =
        "root\n.veriloga \"device.va\"\n.include \"cell.inc\"\nX1 a 0 device\nX2 a 0 cell\n.end\n";
    let device = "`include \"macros.va\"\nmodule device(a, b);\nendmodule\n";
    let macros = "XCLIP = 1;\nXLIMIT = 2;\n.param hidden=1\n";
    let cell = ".include \"leaf.inc\"\n.subckt cell a b\nR1 a b 1k\n.ends cell\n";
    let leaf = ".subckt leaf a b\nR1 a b 1k\n.ends leaf\n";
    let locator = |name: &str| crate::state::SourceLocator::try_new(name, name).expect("a locator");
    let dependencies = vec![
        crate::state::DependencyMetadata::unresolved_direct_to(
            0,
            "device.va",
            locator("device.va"),
        )
        .unwrap()
        .resolve_utf8(device.as_bytes().to_vec())
        .unwrap(),
        crate::state::DependencyMetadata::unresolved_transitive_to(
            locator("device.va"),
            0,
            "macros.va",
            locator("macros.va"),
        )
        .unwrap()
        .resolve_utf8(macros.as_bytes().to_vec())
        .unwrap(),
        crate::state::DependencyMetadata::unresolved_direct_to(1, "cell.inc", locator("cell.inc"))
            .unwrap()
            .resolve_utf8(cell.as_bytes().to_vec())
            .unwrap(),
        crate::state::DependencyMetadata::unresolved_transitive_to(
            locator("cell.inc"),
            0,
            "leaf.inc",
            locator("leaf.inc"),
        )
        .unwrap()
        .resolve_utf8(leaf.as_bytes().to_vec())
        .unwrap(),
    ];
    let provenance = crate::state::GeneratedProvenance::try_new(
        "language-veriloga-index-test",
        crate::state::GenerationInput::new(
            crate::product::ObjectRevision::INITIAL,
            crate::state::content_digest("language-veriloga-index-input"),
        ),
    )
    .unwrap();
    let artifact = crate::state::GeneratedArtifact::try_from_utf8(
        provenance,
        root.as_bytes().to_vec(),
        dependencies,
        Vec::new(),
    )
    .unwrap();
    let document = crate::state::NetlistDocument::from_generated(
        crate::state::NetlistDocumentId::new(),
        artifact,
    )
    .unwrap();

    let index = NetlistProjectIndex::build(ActiveNetlistDocument::Generated, &document);

    let instances = index
        .hierarchy()
        .iter()
        .map(|instance| instance.path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        instances,
        ["X1", "X2"],
        "only the deck's own instances elaborate"
    );
    assert!(index.definitions_named("hidden").is_empty());
    assert!(
        index
            .symbols()
            .iter()
            .all(|symbol| !symbol.definition.display_name.ends_with(".va")),
        "find-symbol must not reach Verilog-A text"
    );
    assert_eq!(index.definitions_named("cell").len(), 1);
    assert_eq!(
        index.definitions_named("leaf").len(),
        1,
        "a SPICE include of a SPICE include is still SPICE"
    );
    assert_eq!(index.source_count(), 3);
    assert_eq!(
        document.dependencies().len(),
        4,
        "the Includes rows read the dependencies, which stay"
    );
}

#[test]
fn project_index_resolves_model_references_across_spice_device_families() {
    let source = "models\n\
.model DMOD D\n\
.model QMOD NPN\n\
.model MMOD NMOS\n\
.model RMOD R\n\
.model SMOD SW\n\
D1 a 0 dmod\n\
Q1 c b e substrate qmod area=2\n\
M1 d g s b mmod l=1u\n\
R1 a b rmod l=1u\n\
R2 b 0 resistance_value\n\
S1 a b c d smod\n\
.end\n";
    let provenance = crate::state::GeneratedProvenance::try_new(
        "language-model-reference-test",
        crate::state::GenerationInput::new(
            crate::product::ObjectRevision::INITIAL,
            crate::state::content_digest("language-model-reference-input"),
        ),
    )
    .unwrap();
    let artifact = crate::state::GeneratedArtifact::try_from_utf8(
        provenance,
        source.as_bytes().to_vec(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let document = crate::state::NetlistDocument::from_generated(
        crate::state::NetlistDocumentId::new(),
        artifact,
    )
    .unwrap();

    let index = NetlistProjectIndex::build(ActiveNetlistDocument::Generated, &document);

    for name in ["dmod", "qmod", "mmod", "rmod", "smod"] {
        assert_eq!(
            index.references_named(name).len(),
            1,
            "expected exactly one semantic reference to {name}"
        );
    }
    assert!(index.references_named("resistance_value").is_empty());
}

#[test]
fn expanded_hierarchy_stops_at_recursive_subcircuit_edges() {
    let location = |line| ProjectSourceLocation {
        root: ActiveNetlistDocument::Generated,
        dependency_identity: None,
        display_name: "recursive.sp".to_owned(),
        line,
        column: 1,
    };
    let mut index = NetlistProjectIndex {
        raw_hierarchy: vec![
            RawHierarchyInstance {
                parent: None,
                instance: "XTOP".to_owned(),
                target: "A".to_owned(),
                location: location(1),
            },
            RawHierarchyInstance {
                parent: Some("A".to_owned()),
                instance: "XB".to_owned(),
                target: "B".to_owned(),
                location: location(3),
            },
            RawHierarchyInstance {
                parent: Some("B".to_owned()),
                instance: "XA".to_owned(),
                target: "A".to_owned(),
                location: location(6),
            },
        ],
        ..NetlistProjectIndex::default()
    };

    index.expand_hierarchy();

    assert_eq!(index.hierarchy().len(), 3);
    assert_eq!(index.hierarchy()[2].path, "XTOP/XB/XA");
    assert!(index.hierarchy()[2].recursive);
    assert!(!index.hierarchy_truncated());
}

#[test]
fn project_index_tracks_parameter_and_function_expression_references() {
    let candidates = expression_identifier_candidates(".param base=1k gain = {base*2}");
    assert!(
        candidates.iter().any(|(name, _, kind)| {
            name.eq_ignore_ascii_case("base") && *kind == ProjectSymbolKind::Parameter
        }),
        "expression candidates were {candidates:?}"
    );
    let source = "expressions\n\
.param base=1k gain = {base*2}\n\
.func shape(x)={x*gain}\n\
R1 out 0 gain\n\
B1 y 0 V={shape(V(out))*gain}\n\
.end\n";
    let provenance = crate::state::GeneratedProvenance::try_new(
        "language-expression-reference-test",
        crate::state::GenerationInput::new(
            crate::product::ObjectRevision::INITIAL,
            crate::state::content_digest("language-expression-reference-input"),
        ),
    )
    .unwrap();
    let artifact = crate::state::GeneratedArtifact::try_from_utf8(
        provenance,
        source.as_bytes().to_vec(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let document = crate::state::NetlistDocument::from_generated(
        crate::state::NetlistDocumentId::new(),
        artifact,
    )
    .unwrap();

    let index = NetlistProjectIndex::build(ActiveNetlistDocument::Generated, &document);

    assert_eq!(index.definitions_named("base").len(), 1);
    assert_eq!(index.definitions_named("gain").len(), 1);
    assert_eq!(index.definitions_named("shape").len(), 1);
    assert_eq!(
        index.references_named("base").len(),
        1,
        "indexed references were {:?}",
        index.references()
    );
    assert_eq!(index.references_named("gain").len(), 3);
    assert_eq!(index.references_named("shape").len(), 1);
    assert!(index.references_named("x").is_empty());
    assert!(index.references_named("out").is_empty());
}

/// An ASCII deck, so a byte offset found with `find` is also the character
/// index the editor reports for a pointer over that byte.
const HOVER_DECK: &str = "hover\n\
.param w=2u\n\
.param l={w*3}\n\
M1 d g s b nch W={w} L={l*2}\n\
.param third={1/3}\n\
.param bad={nope*2}\n\
.end\n";

fn hover_state() -> CodeHarness {
    let mut harness = harness();
    harness.model.simulation.source.netlist_content = HOVER_DECK.to_owned();
    harness
}

/// [`value_hover_at_in`] over the parts of `state` it reads.
fn value_hover_at(harness: &mut CodeHarness, deck: &str, at: usize) -> Option<NetlistValueHover> {
    let netlist = &mut harness.state.netlist;
    value_hover_at_in(netlist, harness.wb.messages(), deck, at)
}

fn hover_at(harness: &mut CodeHarness, needle: &str, offset: usize) -> Option<NetlistValueHover> {
    let index = HOVER_DECK.find(needle).expect("deck contains the token") + offset;
    value_hover_at(harness, HOVER_DECK, index)
}

#[test]
fn value_hover_evaluates_a_parameter_definition_and_an_element_expression() {
    let mut harness = hover_state();

    let definition = hover_at(&mut harness, ".param l=", 7).expect("definition hover");
    assert_eq!(definition.token, "l");
    assert_eq!(definition.value.as_deref(), Some("6u"));
    assert_eq!(definition.detail, None, "a definition is one line");

    let expression = hover_at(&mut harness, "L={l*2}", 2).expect("expression hover");
    assert_eq!(expression.token, "{l*2}");
    assert_eq!(expression.value.as_deref(), Some("12u"));
}

#[test]
fn value_hover_names_the_line_a_reference_is_defined_on() {
    let mut harness = hover_state();

    let reference = hover_at(&mut harness, "{w*3}", 1).expect("reference hover");

    assert_eq!(reference.token, "w");
    assert_eq!(reference.value.as_deref(), Some("2u"));
    assert_eq!(reference.detail.as_deref(), Some("defined line 2"));
}

/// A reader above a redefinition hovers the value the engine binds there,
/// the last definition's, as every ngspice-46 reader does.
#[test]
fn value_hover_over_a_reader_above_a_redefinition_shows_the_final_value() {
    let deck = "hover\n.param w=1u\nM1 d g s b nch W={w}\n.param w=2u\n.end\n";
    let mut harness = harness();
    harness.model.simulation.source.netlist_content = deck.to_owned();
    let mut hover = |needle: &str| {
        let index = deck.find(needle).expect("deck contains the token");
        value_hover_at(&mut harness, deck, index).expect("hover")
    };

    let reader = hover("w}");
    assert_eq!(reader.token, "w");
    assert_eq!(reader.value.as_deref(), Some("2u"));
    assert_eq!(reader.detail.as_deref(), Some("defined line 4"));

    let superseded = hover("w=1u");
    assert_eq!(superseded.token, "w");
    assert_eq!(
        superseded.value.as_deref(),
        Some("2u"),
        "a superseded definition binds nothing"
    );
    assert_eq!(superseded.detail.as_deref(), Some("defined line 4"));

    let binding = hover("w=2u");
    assert_eq!(binding.value.as_deref(), Some("2u"));
    assert_eq!(
        binding.detail, None,
        "the binding definition is the one shown"
    );
}

#[test]
fn value_hover_shows_the_raw_number_when_engineering_notation_would_round() {
    let mut harness = hover_state();

    let rounded = hover_at(&mut harness, ".param third=", 7).expect("hover");

    assert_eq!(rounded.token, "third");
    assert_eq!(
        rounded.value.as_deref(),
        Some("0.3333333333333333"),
        "333.333m does not re-parse to the evaluated value"
    );
}

/// A sub-femto value used to lose its engineering notation here. The
/// formatter spelled the decade below femto `a`, the parser read that `a`
/// as a unit rather than as atto, the round trip failed, and the hover
/// fell back to `0.000000000000000001` — eighteen zeros a reader has to
/// count. The formatter now writes the exponent, the parser reads it back
/// exactly, and the hover shows what was written.
#[test]
fn value_hover_shows_a_sub_femto_value_as_the_exponent_that_round_trips() {
    assert_eq!(format_hover_value(1e-18), "1e-18");
    assert_eq!(format_hover_value(-2.5e-20), "-2.5e-20");
    assert_eq!(format_hover_value(1e-15), "1f", "femto still has a suffix");
}

#[test]
fn value_hover_states_why_an_undefined_reference_has_no_value() {
    let mut harness = hover_state();

    let undefined = hover_at(&mut harness, "{nope*2}", 1).expect("hover");

    assert_eq!(undefined.token, "nope");
    assert_eq!(undefined.value, None);
    let detail = undefined.detail.expect("a reason");
    assert!(
        detail.to_ascii_lowercase().contains("nope")
            && detail.contains("no definition visible here"),
        "unhelpful reason: {detail}"
    );
}

#[test]
fn value_hover_answers_a_repeated_pointer_position_from_retained_state() {
    let mut harness = hover_state();
    let definition = HOVER_DECK.find(".param l=").unwrap() + 7;
    let expression = HOVER_DECK.find("L={l*2}").unwrap() + 2;

    let first = value_hover_at(&mut harness, HOVER_DECK, definition).expect("hover");

    // Blank the retained span index. A pointer that has not moved must
    // still answer, which it can only do without consulting it.
    harness.state.netlist.value_index.as_mut().unwrap().map =
        Arc::new(rspice_core::netlist::NetlistSourceMap::default());
    assert_eq!(
        value_hover_at(&mut harness, HOVER_DECK, definition).as_ref(),
        Some(&first)
    );
    assert_eq!(
        value_hover_at(&mut harness, HOVER_DECK, expression),
        None,
        "a pointer that moved does read the index, so a blank one answers nothing"
    );

    // An edit bumps the revision, which is the only thing that rebuilds it.
    harness.state.netlist.revision += 1;
    assert_eq!(
        value_hover_at(&mut harness, HOVER_DECK, expression)
            .and_then(|hover| hover.value)
            .as_deref(),
        Some("12u")
    );
}

#[test]
fn value_hover_declines_a_unified_comparison_document() {
    let mut harness = hover_state();
    harness.state.netlist.active_document = ActiveNetlistDocument::GeneratedDiff;

    assert_eq!(hover_at(&mut harness, ".param l=", 7), None);
}

#[test]
fn cursor_symbol_is_unicode_indexed_and_accepts_spice_identifier_punctuation() {
    let source = "R1 n1 0 model_name$1\n";
    let cursor = source.find("name").unwrap() + 2;
    assert_eq!(
        symbol_at_cursor(source, cursor),
        Some("model_name$1".to_owned())
    );
}

#[test]
fn semantic_rename_updates_owned_definition_and_cross_file_reference_atomically() {
    let root = "root\n.include \"cell.inc\"\n.param gain=2\n.func shape(x)={x*gain}\nRgain in 0 gain\nX1 in out cell gain={shape(gain)}\n.end\n";
    let include = ".subckt CELL in out params: gain=1\n.ends CELL\n";
    let locator = crate::state::SourceLocator::try_new("cell.inc", "cell.inc").unwrap();
    let dependency = crate::state::DependencyMetadata::unresolved_direct_to(0, "cell.inc", locator)
        .unwrap()
        .resolve_utf8(include.as_bytes().to_vec())
        .unwrap();
    let provenance = crate::state::GeneratedProvenance::try_new(
        "language-rename-test",
        crate::state::GenerationInput::new(
            crate::product::ObjectRevision::INITIAL,
            crate::state::content_digest("language-rename-input"),
        ),
    )
    .unwrap();
    let artifact = crate::state::GeneratedArtifact::try_from_utf8(
        provenance,
        root.as_bytes().to_vec(),
        vec![dependency],
        Vec::new(),
    )
    .unwrap();
    let generated = crate::state::NetlistDocument::from_generated(
        crate::state::NetlistDocumentId::new(),
        artifact,
    )
    .unwrap();
    let owned = generated
        .create_editable_copy(
            crate::state::NetlistDocumentId::new(),
            generated.content_digest(),
        )
        .unwrap();
    let owned_include =
        crate::state::OwnedNetlistIncludeDescriptor::try_new(&owned.dependencies()[0]).unwrap();
    let mut harness = harness();
    harness.model.workspace.content.netlist_source = Some(root.to_owned());
    harness.model.workspace.content.netlist_document = Some(owned.clone());
    harness.model.workspace.content.netlist_descriptor =
        Some(crate::state::OwnedNetlistDescriptor {
            deck_id: uuid::Uuid::new_v4(),
            artifact_name: "root.sp".to_owned(),
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::Lf,
            imported_dialect: Some(crate::state::NetlistSourceDialect::RSpice),
            compatibility_reviewed: true,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: vec![owned_include],
        });
    harness.model.netlist_artifacts.generated_document = Some(generated);
    harness.state.netlist.owned_document = Some(owned);
    harness.state.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    harness.model.simulation.source.netlist_content = root.to_owned();
    harness.state.netlist.cursor_char_index = root.find("cell gain").unwrap() + 2;

    begin_rename_at_cursor_in(&harness.model, &mut harness.state.netlist).unwrap();
    harness.state.netlist.rename_dialog.replacement = "amplifier".to_owned();
    assert_eq!(
        commit_rename(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents
        )
        .unwrap(),
        3
    );

    let document = harness
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert!(
        document
            .source()
            .contains("X1 in out amplifier gain={shape(gain)}")
    );
    assert!(
        document.dependencies()[0]
            .source()
            .unwrap()
            .contains(".subckt amplifier in out")
    );
    assert!(
        document.dependencies()[0]
            .source()
            .unwrap()
            .contains(".ends amplifier")
    );
    assert!(super::super::can_undo_netlist_edit(&harness.state.netlist));
    super::super::undo_netlist_edit(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some(root)
    );
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .unwrap()
            .dependencies()[0]
            .source(),
        Some(include)
    );

    harness.state.netlist.cursor_char_index = root.find(".param gain").unwrap() + 9;
    begin_rename_at_cursor_in(&harness.model, &mut harness.state.netlist).unwrap();
    harness.state.netlist.rename_dialog.replacement = "drive".to_owned();
    assert_eq!(
        commit_rename(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents
        )
        .unwrap(),
        4
    );
    let renamed = harness
        .model
        .workspace
        .content
        .netlist_source
        .as_deref()
        .unwrap();
    assert!(renamed.contains(".param drive=2"));
    assert!(renamed.contains("{x*drive}"));
    assert!(renamed.contains("Rgain in 0 drive"));
    assert!(renamed.contains("{shape(drive)}"));
    super::super::undo_netlist_edit(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();

    harness.state.netlist.cursor_char_index = root.find("shape(x)").unwrap() + 2;
    begin_rename_at_cursor_in(&harness.model, &mut harness.state.netlist).unwrap();
    harness.state.netlist.rename_dialog.replacement = "curve".to_owned();
    assert_eq!(
        commit_rename(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents
        )
        .unwrap(),
        2
    );
    let renamed = harness
        .model
        .workspace
        .content
        .netlist_source
        .as_deref()
        .unwrap();
    assert!(renamed.contains(".func curve(x)"));
    assert!(renamed.contains("{curve(gain)}"));
}

const FIX_DECK: &str = "quick fix\nV1 out 0 1\nR1 out 0 1kk\n.op\n.end\n";

/// An editable project-owned deck, validated as the editor validates it:
/// one finding on `1kk` whose fix is bound to this exact revision and
/// buffer. Returns the finding's identity.
fn validated_fix_state() -> (CodeHarness, uuid::Uuid) {
    let provenance = crate::state::GeneratedProvenance::try_new(
        "language-quick-fix-test",
        crate::state::GenerationInput::new(
            crate::product::ObjectRevision::INITIAL,
            crate::state::content_digest("language-quick-fix-input"),
        ),
    )
    .unwrap();
    let artifact = crate::state::GeneratedArtifact::try_from_utf8(
        provenance,
        FIX_DECK.as_bytes().to_vec(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let generated = crate::state::NetlistDocument::from_generated(
        crate::state::NetlistDocumentId::new(),
        artifact,
    )
    .unwrap();
    let owned = generated
        .create_editable_copy(
            crate::state::NetlistDocumentId::new(),
            generated.content_digest(),
        )
        .unwrap();
    let mut harness = harness();
    harness.model.workspace.content.netlist_source = Some(FIX_DECK.to_owned());
    harness.model.workspace.content.netlist_document = Some(owned.clone());
    harness.model.workspace.content.netlist_descriptor =
        Some(crate::state::OwnedNetlistDescriptor {
            deck_id: uuid::Uuid::new_v4(),
            artifact_name: "fix.sp".to_owned(),
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::Lf,
            imported_dialect: Some(crate::state::NetlistSourceDialect::RSpice),
            compatibility_reviewed: true,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: Vec::new(),
        });
    harness.model.netlist_artifacts.generated_document = Some(generated);
    harness.state.netlist.owned_document = Some(owned);
    harness.state.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    harness.model.simulation.source.netlist_content = FIX_DECK.to_owned();
    harness.state.netlist.revision = 7;

    let start = FIX_DECK.find("1kk").unwrap();
    let mut diagnostic = crate::state::netlist_diagnostics::Diagnostic::error("unknown suffix kk");
    diagnostic.fix = Some(crate::state::netlist_diagnostics::DiagnosticFix {
        label: "Replace with 1k".to_owned(),
        span: start..start + 3,
        replacement: "1k".to_owned(),
    });
    let mut diagnostic = diagnostic.with_line(Some(2));
    diagnostic.bind_validation(7, &crate::state::content_digest(FIX_DECK).to_string());
    let finding = diagnostic.canonical.diagnostic_id;
    harness.state.netlist.diagnostics = Arc::new(
        crate::state::netlist_diagnostics::NetlistDiagnosticCollection::try_new(
            vec![diagnostic],
            FIX_DECK,
        )
        .unwrap(),
    );
    (harness, finding)
}

/// Pressing a fix twice from the Problems page, with the editor off
/// screen and so nothing marking the finding stale, applies it once. The
/// second press is refused rather than rewriting the bytes that moved
/// into the first press's range.
#[test]
fn a_quick_fix_pressed_twice_applies_once() {
    let (mut harness, finding) = validated_fix_state();

    apply_quick_fix_for_finding(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        finding,
    )
    .expect("the fix applies to its own bytes");
    let fixed = FIX_DECK.replace("1kk", "1k");
    assert_eq!(harness.model.simulation.source.netlist_content, fixed);
    assert!(
        harness
            .state
            .netlist
            .diagnostics
            .iter()
            .all(crate::state::netlist_diagnostics::Diagnostic::is_current),
        "precondition: nothing marked the finding stale between the presses",
    );

    let refused = apply_quick_fix_for_finding(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        finding,
    )
    .expect_err("a second press has no bytes of its own left to fix");
    assert!(refused.contains("has changed"), "{refused}");
    assert_eq!(
        harness.model.simulation.source.netlist_content, fixed,
        "the refused press leaves the source exactly as the first one left it",
    );
}

/// Any edit between validation and the press refuses the fix: one that
/// moves the revision, and one that replaces the bytes without moving it.
#[test]
fn a_quick_fix_is_refused_after_an_intervening_edit() {
    let (mut harness, finding) = validated_fix_state();
    let edited = format!("* note\n{FIX_DECK}");
    harness.model.simulation.source.netlist_content = edited.clone();
    harness.state.netlist.revision += 1;
    assert!(
        apply_quick_fix_for_finding(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents,
            finding
        )
        .is_err()
    );
    assert_eq!(harness.model.simulation.source.netlist_content, edited);

    let (mut harness, finding) = validated_fix_state();
    harness.model.simulation.source.netlist_content = edited.clone();
    assert!(
        apply_quick_fix_for_finding(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents,
            finding
        )
        .is_err(),
        "the digest refuses bytes a writer replaced without moving the revision",
    );
    assert_eq!(harness.model.simulation.source.netlist_content, edited);
}

/// An edit that moves the revision and leaves the bytes as they were —
/// typed and undone — still refuses the fix: the digest alone would let
/// it through, so the revision is what refuses it.
#[test]
fn a_quick_fix_is_refused_when_only_the_revision_moved() {
    let (mut harness, finding) = validated_fix_state();
    harness.state.netlist.revision += 1;
    let refused = apply_quick_fix_for_finding(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        finding,
    )
    .expect_err("the finding was validated at an earlier revision");
    assert!(refused.contains("has changed"), "{refused}");
    assert_eq!(harness.model.simulation.source.netlist_content, FIX_DECK);
}

/// The caret's preferred fix is offered only for the revision it was
/// validated at, so the command that applies it is not offered for a fix
/// it would refuse.
#[test]
fn the_preferred_fix_is_withdrawn_when_only_the_revision_moved() {
    let (mut harness, _) = validated_fix_state();
    harness.state.netlist.cursor_line = 2;
    assert!(
        preferred_quick_fix_available(&harness.state.netlist),
        "precondition: the caret is on the finding's line",
    );
    harness.state.netlist.revision += 1;
    assert!(!preferred_quick_fix_available(&harness.state.netlist));
}
