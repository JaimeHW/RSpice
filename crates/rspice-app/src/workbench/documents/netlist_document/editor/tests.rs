//! What the netlist editor shows for a deck: the value hover, completion
//! through `show`, live lint across includes, and the diagnostics a parse
//! maps back onto the authored lines.

use super::*;

/// Two parameters, a derived one, and an element card that reads both, so
/// a hover has something to answer on every kind of span.
const HOVER_DECK: &str = "value hover\n\
.param w=2u\n\
.param l={w*3}\n\
M1 d g s b nch W={w} L={l*2}\n\
.model nch nmos\n\
.end\n";

/// Stage widths the deck surface is reviewed at.
const HOVER_STAGE_WIDTHS: [f32; 2] = [1600.0, 1000.0];

/// Render the editor twice at `width`: once plain, once with the pointer
/// parked on `column` of `row`.
///
/// The pointer is aimed with the arithmetic the virtual editor hit-tests
/// with — its own origin, gutter, code padding, and monospace advance —
/// which the first pass reads back rather than restating.
#[cfg(not(target_arch = "wasm32"))]
fn hover_rasters(
    width: f32,
    row: usize,
    column: usize,
) -> (
    crate::ui::raster::Canvas,
    crate::ui::raster::Canvas,
    egui::Pos2,
    Option<super::super::language::NetlistValueHover>,
) {
    let mut state = AppState::default();
    state.model.workspace.content.netlist_source = Some(HOVER_DECK.to_owned());
    state.model.simulation.source.netlist_content = HOVER_DECK.to_owned();
    state.code.netlist.active_document = super::super::ActiveNetlistDocument::OwnedSource;
    state.code.netlist.active_document_initialized = true;

    let geometry = std::cell::Cell::new((egui::Pos2::ZERO, 0.0_f32));
    let size = egui::vec2(width, 220.0);
    let plain = crate::ui::raster::render(size, |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                let advance = ui.fonts_mut(|fonts| {
                    fonts
                        .layout_no_wrap(
                            "M".to_owned(),
                            theme::mono(FONT_SIZE, FontWeight::Regular),
                            egui::Color32::WHITE,
                        )
                        .size()
                        .x
                });
                geometry.set((ui.cursor().min, advance));
                show(ui, &mut state);
            });
    });

    let (origin, advance) = geometry.get();
    let pointer = origin
        + egui::vec2(
            GUTTER_W + CODE_LEFT_PADDING + column as f32 * advance,
            f32::from(CODE_TOP_PADDING) + (row as f32 + 0.5) * LINE_HEIGHT,
        );
    let hovered = crate::ui::raster::render_with_pointer(size, pointer, |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| show(ui, &mut state));
    });
    let resolved = state
        .code
        .netlist
        .value_index
        .as_ref()
        .and_then(super::super::language::CachedValueIndex::last_hover)
        .cloned();
    (hovered, plain, pointer, resolved)
}

/// The region a tooltip opens into, below and right of the pointer.
#[cfg(not(target_arch = "wasm32"))]
fn hover_popover_region(pointer: egui::Pos2) -> egui::Rect {
    egui::Rect::from_min_size(pointer + egui::vec2(0.0, 12.0), egui::vec2(260.0, 60.0))
}

/// The defect this exists for: a hover that resolves in state but never
/// reaches a surface. Read it off the render, not off the resolver.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn hovering_a_parameter_opens_a_popover_the_plain_editor_does_not() {
    for width in HOVER_STAGE_WIDTHS {
        // Row 2 is `.param l={w*3}`; column 7 is the `l` it binds.
        let (hovered, plain, pointer, resolved) = hover_rasters(width, 2, 7);
        assert_eq!(
            resolved
                .as_ref()
                .map(|hover| (hover.token.as_str(), hover.value.as_deref())),
            Some(("l", Some("6u"))),
            "the pointer must land on the parameter the popover is claimed for"
        );
        let region = hover_popover_region(pointer);
        let read = |canvas: &crate::ui::raster::Canvas| {
            canvas
                .pixels_in(region)
                .flat_map(|pixel| pixel.to_array())
                .collect::<Vec<_>>()
        };
        let hovered_pixels = read(&hovered);
        assert!(
            !hovered_pixels.is_empty(),
            "the popover region is off canvas"
        );
        assert_ne!(
            hovered_pixels,
            read(&plain),
            "at {width} px the hovered editor renders the same as the plain one"
        );
    }
}

#[test]
fn editor_diagnostic_pipeline_has_no_panic_shortcuts() {
    let production = crate::source_guard::production_source(include_str!("../editor.rs"));
    for forbidden in [".expect(", ".unwrap(", "panic!(", "unreachable!("] {
        assert!(
            !production.contains(forbidden),
            "netlist editor production code contains panic shortcut {forbidden}"
        );
    }
}

/// The defect this exists for: completion used to be wired to a
/// `TextEdit` branch that only ran for an *empty* buffer, so the popover
/// was unreachable for every real deck while its own unit tests passed.
/// Assert it against a document with content in it, through `show`, or
/// the same class of regression comes back invisible.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn completion_reaches_a_document_that_has_content_in_it() {
    let mut state = AppState::default();
    let source = "test deck\n.tr";
    state.model.workspace.content.netlist_source = Some(source.to_owned());
    state.model.simulation.source.netlist_content = source.to_owned();
    state.code.netlist.active_document = super::super::ActiveNetlistDocument::OwnedSource;
    state.code.netlist.active_document_initialized = true;
    assert!(
        super::super::active_netlist_source_is_editable(&state),
        "fixture must be an editable owned document"
    );

    let context = egui::Context::default();
    crate::ui::Theme::default().apply(&context);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(900.0, 600.0),
        )),
        ..Default::default()
    };
    // Frame one builds the model and takes focus; the caret request is
    // applied on the frame after, which is when an anchor exists.
    for frame in 0..3 {
        let _ = context.clone().run_ui(input.clone(), |ui| {
            if frame == 0 {
                let editor_id = editor_id(&state);
                ui.ctx()
                    .memory_mut(|memory| memory.request_focus(editor_id));
                crate::workbench::documents::text_editor_commands::queue_caret_char_index(
                    ui.ctx(),
                    editor_id,
                    source.chars().count(),
                );
            }
            show(ui, &mut state);
        });
    }

    assert!(
        state.code.netlist.completion_open,
        "the dot-command popover must open for a caret trailing `.tr` in a non-empty deck"
    );
}

#[cfg(not(target_arch = "wasm32"))]
struct IncludeFixtureDir(PathBuf);

#[cfg(not(target_arch = "wasm32"))]
impl IncludeFixtureDir {
    fn new(label: &str) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};

        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        // A deck linted against an authenticated bundle is matched to the
        // sealed sources by path identity, and those were recorded through
        // `std::fs::canonicalize`. Rooting the fixture at an aliased
        // temporary directory would put the deck in a second identity
        // space that no sealed source can be found in.
        let path = crate::fixture_root::canonical_temp_dir().join(format!(
            "rspice-editor-include-{label}-{}-{id}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("create include fixture directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for IncludeFixtureDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_native_fixture(
    source: &str,
    root: &Path,
) -> (Vec<Diagnostic>, Option<Vec<completion::SymbolEntry>>) {
    let (diagnostics, symbols, _) = parse_buffer_with_context(
        source,
        Some(root),
        &rspice_simulation::netlist_preparation::IncludeSearchChain::default(),
        IncludeAccess::NativeFilesystem,
        None,
        &NoAbort,
    );
    (diagnostics, symbols)
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn live_lint_resolves_nested_includes_from_each_owning_source() {
    let fixture = IncludeFixtureDir::new("nested");
    let root = fixture.path().join("root.cir");
    std::fs::write(
        fixture.path().join("child.inc"),
        ".include \"nested/grand.inc\"\n",
    )
    .expect("write child include");
    std::fs::create_dir_all(fixture.path().join("nested")).expect("create nested directory");
    std::fs::write(
        fixture.path().join("nested/grand.inc"),
        ".subckt CELL a b\nRIN a b 1k\n.ends CELL\n",
    )
    .expect("write grandchild include");
    let source = "nested include lint\n.include \"child.inc\"\nX1 out 0 CELL\n.end\n";

    let (diagnostics, symbols) = parse_native_fixture(source, &root);

    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let symbols = symbols.expect("clean dependency closure harvests symbols");
    assert!(symbols.iter().any(|symbol| symbol.name == "CELL"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn live_lint_honors_external_lib_section_selection() {
    let fixture = IncludeFixtureDir::new("lib-section");
    let root = fixture.path().join("root.cir");
    std::fs::write(
        fixture.path().join("models.lib"),
        ".lib TT\n.model DTT D(Is=1e-14)\n.endl TT\n\
         .lib FF\n.enddata\n.endl FF\n",
    )
    .expect("write sectioned library");
    let source = "selected library lint\n.lib \"models.lib\" TT\nD1 out 0 DTT\n.end\n";

    let (diagnostics, symbols) = parse_native_fixture(source, &root);

    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    assert!(
        symbols
            .expect("selected section parses")
            .iter()
            .any(|symbol| symbol.name.eq_ignore_ascii_case("DTT"))
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn live_lint_reports_missing_include_at_root_directive() {
    let fixture = IncludeFixtureDir::new("missing");
    let root = fixture.path().join("root.cir");
    let source = "missing include lint\n.include \"missing.inc\"\n.end\n";

    let (diagnostics, symbols) = parse_native_fixture(source, &root);

    assert!(symbols.is_none());
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, Some(1));
    assert_eq!(diagnostics[0].source_line, Some(1));
    assert_eq!(diagnostics[0].source_path.as_deref(), Some(root.as_path()));
    assert!(diagnostics[0].message.contains("Include file not found"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn live_lint_reports_include_cycle_at_root_dependency_edge() {
    let fixture = IncludeFixtureDir::new("cycle");
    let root = fixture.path().join("root.cir");
    std::fs::write(fixture.path().join("a.inc"), ".include \"b.inc\"\n")
        .expect("write first cycle member");
    std::fs::write(fixture.path().join("b.inc"), ".include \"a.inc\"\n")
        .expect("write second cycle member");
    let source = "cyclic include lint\n.include \"a.inc\"\n.end\n";

    let (diagnostics, symbols) = parse_native_fixture(source, &root);

    assert!(symbols.is_none());
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, Some(1));
    assert_eq!(diagnostics[0].source_path.as_deref(), Some(root.as_path()));
    assert!(
        diagnostics[0]
            .message
            .to_ascii_lowercase()
            .contains("circular include")
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn live_lint_keeps_included_syntax_error_on_included_source() {
    let fixture = IncludeFixtureDir::new("included-syntax");
    let root = fixture.path().join("root.cir");
    let child = fixture.path().join("child.inc");
    std::fs::write(&child, "R1 out 0 1k FIRST SECOND\n").expect("write invalid included source");
    let source = "included syntax lint\n.include \"child.inc\"\n.end\n";

    let (diagnostics, symbols) = parse_native_fixture(source, &root);
    let canonical_child = child.canonicalize().expect("canonicalize included source");

    assert!(symbols.is_none());
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0].source_path.as_deref(),
        Some(canonical_child.as_path())
    );
    assert_eq!(diagnostics[0].source_line, Some(0));
    assert_eq!(
        diagnostics[0].line, None,
        "an included-source line must not paint the root editor gutter"
    );
    assert!(diagnostics[0].message.contains("Unexpected trailing token"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn live_lint_keeps_included_warning_off_the_root_gutter() {
    let fixture = IncludeFixtureDir::new("included-warning");
    let root = fixture.path().join("root.cir");
    let child = fixture.path().join("child.inc");
    std::fs::write(&child, "Rchild out 0\n").expect("write warning-producing include");
    let source = "included warning lint\n.include \"child.inc\"\n.end\n";

    let (diagnostics, symbols) = parse_native_fixture(source, &root);
    let canonical_child = child.canonicalize().expect("canonicalize included source");

    assert!(symbols.is_some());
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].severity, DiagnosticSeverity::Warning);
    assert_eq!(
        diagnostics[0].source_path.as_deref(),
        Some(canonical_child.as_path())
    );
    assert_eq!(diagnostics[0].source_line, Some(0));
    assert_eq!(
        diagnostics[0].line, None,
        "an included warning must not paint the root editor gutter"
    );
}

#[test]
fn live_lint_fails_closed_when_browser_cannot_resolve_include() {
    let source = "browser include lint\n.include \"models/device.lib\"\n.end\n";
    let (diagnostics, symbols, _) = parse_buffer_with_context(
        source,
        Some(Path::new("project/root.cir")),
        &rspice_simulation::netlist_preparation::IncludeSearchChain::default(),
        IncludeAccess::AuthenticatedBundleOnly,
        None,
        &NoAbort,
    );

    assert!(symbols.is_none());
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, Some(1));
    assert!(
        diagnostics[0]
            .message
            .contains("must come from an imported source bundle")
    );
}

#[test]
fn live_lint_parses_dependency_free_browser_deck_without_a_bundle() {
    let source = "browser in-memory lint\nV1 input 0 5\nR1 input 0 1k\n.op\n.end\n";
    let (diagnostics, symbols, _) = parse_buffer_with_context(
        source,
        Some(Path::new("project/root.cir")),
        &rspice_simulation::netlist_preparation::IncludeSearchChain::default(),
        IncludeAccess::AuthenticatedBundleOnly,
        None,
        &NoAbort,
    );

    assert!(symbols.is_some());
    assert!(diagnostics.iter().all(|diagnostic| {
        !diagnostic
            .message
            .contains("must come from an imported source bundle")
    }));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn live_lint_parses_authenticated_browser_model_bundle_without_filesystem_fallback() {
    let fixture = IncludeFixtureDir::new("authenticated-browser");
    let root = fixture.path().join("root.cir");
    let model = fixture.path().join("device.lib");
    std::fs::write(&model, ".lib TT\n.model DSEALED D(Is=1e-14)\n.endl TT\n")
        .expect("write authenticated model library");
    let mut manager = crate::state::ModelLibraryManager::new();
    manager
        .load_library_file(&model, Some("TT"))
        .expect("import authenticated model library");
    let sealed = manager
        .seal_execution_sources()
        .expect("seal authenticated model library");
    let source = "authenticated browser lint\n.lib \"device.lib\" TT\nD1 out 0 DSEALED\n.end\n";

    let (diagnostics, symbols, _) = parse_buffer_with_context(
        source,
        Some(&root),
        &rspice_simulation::netlist_preparation::IncludeSearchChain::default(),
        IncludeAccess::AuthenticatedBundleOnly,
        Some(&sealed),
        &NoAbort,
    );

    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    assert!(
        symbols
            .expect("authenticated source parses")
            .iter()
            .any(|symbol| symbol.name.eq_ignore_ascii_case("DSEALED"))
    );
}

#[test]
fn live_lint_uses_abort_aware_parser_contract() {
    let source = "cancelled lint\nR1 out 0 1k\n.end\n";
    let (diagnostics, symbols, _) = parse_buffer_with_context(
        source,
        None,
        &rspice_simulation::netlist_preparation::IncludeSearchChain::default(),
        IncludeAccess::NativeFilesystem,
        None,
        &rspice_core::abort_signal::ImmediateAbort,
    );

    assert!(symbols.is_none());
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].severity, DiagnosticSeverity::Info);
    assert!(diagnostics[0].message.contains("cancelled"));
}

/// A parameter override, shown in the editor as its own lines, over a
/// retained generated base.
fn parameter_override_state(base: &str, authored: &str) -> AppState {
    let provenance = crate::state::GeneratedProvenance::try_new(
        "netlist-editor-override-test",
        crate::state::GenerationInput::new(
            crate::product::ObjectRevision::INITIAL,
            crate::state::content_digest("netlist-editor-override-input"),
        ),
    )
    .unwrap();
    let artifact = crate::state::GeneratedArtifact::try_from_utf8(
        provenance,
        base.as_bytes().to_vec(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let generated = crate::state::NetlistDocument::from_generated(
        crate::state::NetlistDocumentId::new(),
        artifact,
    )
    .unwrap();
    let mut owned = generated
        .create_editable_copy(
            crate::state::NetlistDocumentId::new(),
            generated.content_digest(),
        )
        .unwrap();
    owned
        .replace_editable_source(owned.content_digest(), authored.as_bytes().to_vec())
        .unwrap();
    let mut state = AppState::default();
    state.model.workspace.content.netlist_source = Some(authored.to_owned());
    state.model.workspace.content.netlist_document = Some(owned.clone());
    state.model.workspace.content.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        deck_id: uuid::Uuid::new_v4(),
        artifact_name: "corner.cir".to_owned(),
        strategy: crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::Lf,
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    });
    state.code.netlist.owned_document = Some(owned);
    state.code.netlist.active_document = super::super::ActiveNetlistDocument::OwnedSource;
    state.model.simulation.source.netlist_content = authored.to_owned();
    state
}

/// A parameter override is validated as the deck it executes as — the
/// base with the override spliced in before `.end` — and its error is
/// reported on the override's own line, not on the line it lands on in
/// that deck.
#[test]
fn a_parameter_override_error_is_reported_on_its_own_line() {
    let base = "* generated\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let authored = "* corner\n.temp -273.15\n";
    let mut state = parameter_override_state(base, authored);

    let revision = state.code.netlist.revision;
    validate_visible_source(&mut state, revision);
    let error = state
        .code
        .netlist
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        .expect("a temperature below absolute zero is refused");
    assert_eq!(
        error.line,
        Some(1),
        "the override's second line, not the composed deck's sixth: {error:?}",
    );
    assert!(error.is_current());
}

/// A deck in the editor, validated the way the editor validates it.
fn validated(deck: &str) -> AppState {
    let mut state = AppState::default();
    state.model.simulation.source.netlist_content = deck.to_owned();
    let revision = state.code.netlist.revision;
    validate_visible_source(&mut state, revision);
    state
}

/// The value hover at the `offset`th byte of the first `needle` in `deck`.
fn hover_on(
    state: &mut AppState,
    deck: &str,
    needle: &str,
    offset: usize,
) -> Option<super::super::language::NetlistValueHover> {
    let byte = deck.find(needle).expect("deck contains the token") + offset;
    let index = deck[..byte].chars().count();
    super::super::language::value_hover_at(state, deck, index)
}

/// The parser reads only the branch it takes, so the hover binds that
/// branch's value everywhere and says the other definition is not read,
/// and why, without a value.
#[test]
fn value_hover_reads_only_the_if_branch_the_parser_takes() {
    for (mode, taken, skipped, value, holds) in [
        (1, "vth=0.5", "vth=0.4", "500m", "true"),
        (0, "vth=0.4", "vth=0.5", "400m", "false"),
    ] {
        let deck = format!(
            "hover\n.param mode={mode}\n.if (mode==1)\n.param vth=0.5\n.else\n\
             .param vth=0.4\n.endif\nV1 a 0 {{vth}}\n.end\n"
        );
        let mut state = validated(&deck);

        let reader = hover_on(&mut state, &deck, "{vth}", 1).expect("reader hover");
        assert_eq!(reader.value.as_deref(), Some(value), "mode={mode}");

        let binding = hover_on(&mut state, &deck, taken, 0).expect("binding hover");
        assert_eq!(binding.value.as_deref(), Some(value), "mode={mode}");
        assert!(!binding.not_read);

        let unread = hover_on(&mut state, &deck, skipped, 0).expect("unread hover");
        assert_eq!(unread.token, "vth");
        assert_eq!(
            unread.value, None,
            "mode={mode}: an unread line binds nothing"
        );
        assert_eq!(
            unread.detail.as_deref(),
            Some(format!("not read: in an .if branch not taken (mode==1 is {holds})").as_str()),
            "mode={mode}"
        );
        assert!(
            unread.not_read,
            "the parser's answer outranks the project symbol"
        );
    }
}

#[test]
fn value_hover_reads_only_the_branch_a_nested_if_takes() {
    let deck = "hover\n.param mode=1 corner=2\n.if (mode==1)\n.if (corner==1)\n\
                .param vth=0.1\n.elseif (corner==2)\n.param vth=0.2\n.else\n\
                .param vth=0.3\n.endif\n.else\n.param vth=0.9\n.endif\n\
                V1 a 0 {vth}\n.end\n";
    let mut state = validated(deck);

    assert_eq!(
        hover_on(&mut state, deck, "{vth}", 1).and_then(|hover| hover.value),
        Some("200m".to_owned())
    );
    for (needle, reason) in [
        ("vth=0.1", "corner==1 is false"),
        ("vth=0.3", "corner==2 is true"),
        ("vth=0.9", "mode==1 is true"),
    ] {
        let hover = hover_on(&mut state, deck, needle, 0).expect("hover");
        assert_eq!(hover.value, None, "{needle}");
        assert_eq!(
            hover.detail,
            Some(format!("not read: in an .if branch not taken ({reason})")),
            "{needle}"
        );
    }
}

/// A definition after the block is read wherever it is, so the last read
/// one still binds the reader above the block.
#[test]
fn value_hover_binds_a_redefinition_after_the_endif_everywhere() {
    let deck = "hover\n.param mode=1\nV2 b 0 {vth}\n.if (mode==1)\n.param vth=0.5\n\
                .else\n.param vth=0.4\n.endif\n.param vth=0.7\n.end\n";
    let mut state = validated(deck);

    let reader = hover_on(&mut state, deck, "{vth}", 1).expect("hover");
    assert_eq!(reader.value.as_deref(), Some("700m"));
    assert_eq!(reader.detail.as_deref(), Some("defined line 9"));
    assert_eq!(
        hover_on(&mut state, deck, "vth=0.4", 0).and_then(|hover| hover.detail),
        Some("not read: in an .if branch not taken (mode==1 is true)".to_owned())
    );
}

/// A subcircuit body binds its parameters when it is instantiated, so a
/// deck that never instantiates it parses with the name unbound.
#[test]
fn value_hover_names_where_a_parameter_only_unread_lines_define_is_written() {
    let deck = "hover\n.param mode=0\n.if (mode==1)\n.param vth=0.5\n.endif\n\
                .subckt cell a b\nR1 a b {vth}\n.ends\n.end\n";
    let mut state = validated(deck);

    let reader = hover_on(&mut state, deck, "{vth}", 1).expect("hover");
    assert_eq!(reader.value, None);
    assert_eq!(
        reader.detail.as_deref(),
        Some("vth is defined only on line 4, in an .if branch not taken (mode==1 is false)")
    );
    assert!(reader.not_read);
}

/// A deck's own inline sections are definitions no card selects.
#[test]
fn value_hover_reads_none_of_a_decks_own_inline_library_sections() {
    let deck = "hover\n.param vth=0.9\n.lib tt\n.param vth=0.5\n.endl tt\n\
                .lib ss\n.param vth=0.4\n.endl ss\nV1 a 0 {vth}\n.end\n";
    let mut state = validated(deck);

    assert_eq!(
        hover_on(&mut state, deck, "{vth}", 1).and_then(|hover| hover.value),
        Some("900m".to_owned())
    );
    for (needle, section) in [("vth=0.5", "tt"), ("vth=0.4", "ss")] {
        assert_eq!(
            hover_on(&mut state, deck, needle, 0).and_then(|hover| hover.detail),
            Some(format!("not read: in .lib section {section}, not selected")),
        );
    }
}

/// Until a parse of this revision decides the branches, the hover vouches
/// for no value in a deck that has them; once it has, it answers.
#[test]
fn value_hover_waits_for_the_parse_that_decides_the_branches() {
    let deck = "hover\n.param mode=1\n.if (mode==1)\n.param vth=0.5\n.else\n\
                .param vth=0.4\n.endif\nV1 a 0 {vth}\n.end\n";
    let mut state = AppState::default();
    state.model.simulation.source.netlist_content = deck.to_owned();

    assert_eq!(hover_on(&mut state, deck, "{vth}", 1), None);

    let revision = state.code.netlist.revision;
    validate_visible_source(&mut state, revision);
    assert_eq!(
        hover_on(&mut state, deck, "{vth}", 1).and_then(|hover| hover.value),
        Some("500m".to_owned()),
        "the arriving parse rebuilds the index the earlier hover built"
    );
}

/// The reference lint counts what the parser reads: a model written only
/// in a skipped branch defines nothing, and a card there references
/// nothing.
#[test]
fn the_reference_lint_counts_only_the_branch_the_parser_takes() {
    let deck = "lint\n.param mode=0\n.if (mode==1)\n.model dsel d\nDOFF a 0 dmissing\n\
                .endif\nD1 a 0 dsel\n.end\n";
    let state = validated(deck);

    let unknown = state
        .code
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.canonical.code.as_ref() == "SPICE-UNKNOWN-MODEL")
        .map(|diagnostic| diagnostic.message.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        unknown,
        vec!["Unknown model `dsel` in this deck.".to_owned()]
    );
}

#[test]
fn parse_buffer_maps_syntax_error_to_structured_diagnostic() {
    let (diagnostics, symbols) = parse_buffer("title\n.enddata\n");

    assert!(symbols.is_none());
    assert_eq!(diagnostics.len(), 1);
    let diagnostic = &diagnostics[0];
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.line, Some(1));
    assert_eq!(diagnostic.column, None);
    assert!(diagnostic.span.is_none());
    assert!(diagnostic.fix.is_none());
    assert!(!diagnostic.message.trim().is_empty());
}

/// The imported ngspice deck this wave exists for: the block's commands
/// each reach the document with their own verdict, and the deck surface's
/// band counts exactly those records instead of re-reading the buffer.
#[test]
fn control_commands_reach_the_document_line_by_line_and_feed_the_band() {
    let source = "control deck\n\
                  V1 in 0 1\n\
                  R1 in out 1k\n\
                  .control\n\
                  tran 1n 100n\n\
                  print v(out)\n\
                  .endc\n\
                  .end\n";

    let (diagnostics, _) = parse_buffer(source);

    let promoted = diagnostics
        .iter()
        .find(|diagnostic| {
            diagnostic.canonical.code.as_ref() == super::super::diagnostics::CONTROL_PROMOTED_CODE
        })
        .expect("the promoted command reaches the document");
    assert_eq!(promoted.severity, DiagnosticSeverity::Info);
    assert_eq!(promoted.line, Some(4));
    assert!(
        promoted.message.contains(".tran 1n 100n"),
        "{}",
        promoted.message
    );

    let dropped = diagnostics
        .iter()
        .find(|diagnostic| {
            diagnostic.canonical.code.as_ref() == super::super::diagnostics::CONTROL_DROPPED_CODE
        })
        .expect("the ignored command reaches the document");
    assert_eq!(dropped.severity, DiagnosticSeverity::Warning);
    assert_eq!(dropped.line, Some(5));
    assert!(dropped.message.contains("'print'"), "{}", dropped.message);

    let collection = NetlistDiagnosticCollection::try_new(diagnostics, source)
        .expect("the collection publishes");
    let summary = super::super::control_disposition_summary(&collection)
        .expect("the band is painted for a deck with a control region");
    assert_eq!((summary.promoted, summary.dropped), (1, 1));
    assert_eq!(
        summary
            .examples
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec![
            "line 5 · Control command `tran` was promoted into `.tran 1n 100n`.",
            "line 6 · control command 'print' ignored; .control scripting is not executed, so whatever it requests (output, plotting, control flow) will not run",
        ],
        "the hover walks down the block rather than listing promotions and drops apart"
    );
}

#[test]
fn a_deck_without_a_control_region_paints_no_band() {
    let (diagnostics, _) = parse_buffer("plain deck\nR1 in 0 1k\n.op\n.end\n");
    let collection =
        NetlistDiagnosticCollection::try_new(diagnostics, "plain deck\nR1 in 0 1k\n.op\n.end\n")
            .expect("the collection publishes");

    assert!(super::super::control_disposition_summary(&collection).is_none());
}

#[test]
fn mapped_syntax_origin_accepts_windows_verbatim_drive_paths() {
    let source = r"\\?\C:\projects\rspice\models\child.inc";
    let (detail, origin) = mapped_syntax_origin(
        format!("{source}:17: Unexpected trailing token"),
        17,
        Some(Path::new(r"C:\projects\rspice\root.cir")),
    );

    assert_eq!(detail, "Unexpected trailing token");
    assert_eq!(origin.path.as_deref(), Some(Path::new(source)));
    assert_eq!(origin.line, 17);
}

#[test]
fn parse_buffer_maps_missing_ends_to_opening_line() {
    let source = "* missing ends\n.subckt Cell a b\nR1 a b 1\n.end\n";

    let (diagnostics, symbols) = parse_buffer(source);

    assert!(symbols.is_none());
    assert_eq!(diagnostics.len(), 1);
    let diagnostic = &diagnostics[0];
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.line, Some(1));
    assert!(diagnostic.message.contains("Subcircuit CELL missing .ENDS"));
    assert!(diagnostic.message.contains("reached .END"));
}

#[test]
fn parse_buffer_maps_initcond_duplicate_to_duplicate_card_line() {
    let source = "duplicate initcond\n\
                  .INITCOND C1 IC=1\n\
                  .INITCOND malformed second card\n\
                  C1 1 0 1u\n\
                  .END\n";

    let (diagnostics, symbols) = parse_buffer(source);

    assert!(symbols.is_none());
    assert_eq!(diagnostics.len(), 1);
    let diagnostic = &diagnostics[0];
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.line, Some(2));
    assert!(diagnostic.message.contains("may appear only once"));
}

#[test]
fn duplicate_subcircuit_binding_diagnostic_preserves_hierarchy_context() {
    let diagnostic = parse_error_diagnostic(
        rspice_core::netlist::ParseError::DuplicateSubcircuitPortBinding(Box::new(
            rspice_core::netlist::DuplicateSubcircuitPortBindingError {
                subcircuit_name: "inv1".into(),
                canonical_subcircuit_name: "INV1".into(),
                instance_name: "Xinv1".into(),
                canonical_instance_name: "XINV1".into(),
                qualified_instance_name: "TOP.Xinv1".into(),
                formal_port: "GND".into(),
                first_position: 4,
                conflicting_position: 8,
                first_actual_node: "0".into(),
                conflicting_actual_node: "VDD".into(),
            },
        )),
    );

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.line, None);
    assert!(
        diagnostic
            .message
            .contains("Subcircuit: inv1 (read as INV1)")
    );
    assert!(diagnostic.message.contains("TOP.Xinv1"));
    assert!(diagnostic.message.contains("positions 4 and 8"));
    assert!(diagnostic.message.contains("effective nodes 0 and VDD"));
}

#[test]
fn global_subcircuit_binding_diagnostic_preserves_effective_node() {
    let diagnostic = parse_error_diagnostic(
        rspice_core::netlist::ParseError::GlobalSubcircuitPortBinding(Box::new(
            rspice_core::netlist::GlobalSubcircuitPortBindingError {
                subcircuit_name: "cell".into(),
                canonical_subcircuit_name: "CELL".into(),
                instance_name: "X1".into(),
                canonical_instance_name: "X1".into(),
                qualified_instance_name: "TOP.X1".into(),
                formal_port: "$G_SHARED".into(),
                position: 1,
                actual_node: "LOCAL".into(),
            },
        )),
    );

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert!(
        diagnostic
            .message
            .contains("Subcircuit: cell (read as CELL)")
    );
    assert!(diagnostic.message.contains("TOP.X1"));
    assert!(diagnostic.message.contains("effective node LOCAL"));
}

#[test]
fn undefined_subcircuit_diagnostic_preserves_hierarchy_identity() {
    let diagnostic = parse_error_diagnostic(rspice_core::netlist::ParseError::UndefinedSubcircuit(
        Box::new(rspice_core::netlist::UndefinedSubcircuitError {
            subcircuit_name: "missing".into(),
            canonical_subcircuit_name: "MISSING".into(),
            instance_name: "x1".into(),
            canonical_instance_name: "X1".into(),
            qualified_instance_name: "TOP.X1".into(),
        }),
    ));

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert!(
        diagnostic
            .message
            .contains("Subcircuit: missing (read as MISSING)")
    );
    assert!(diagnostic.message.contains("TOP.X1 (read as X1)"));
}

#[test]
fn missing_device_model_diagnostic_preserves_device_identity() {
    let diagnostic = parse_error_diagnostic(rspice_core::netlist::ParseError::MissingDeviceModel(
        Box::new(rspice_core::netlist::MissingDeviceModelError {
            line: 4,
            device_name: "d1".into(),
            canonical_device_name: "D1".into(),
            device_type: "DIODE".into(),
        }),
    ));

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert!(diagnostic.message.contains("Device: d1 (read as D1)"));
    assert!(diagnostic.message.contains("type: DIODE"));
}

#[test]
fn undefined_mutual_inductor_diagnostic_preserves_origin_and_identity() {
    let diagnostic = parse_error_diagnostic(
        rspice_core::netlist::ParseError::UndefinedMutualInductorReference(Box::new(
            rspice_core::netlist::UndefinedMutualInductorReferenceError {
                origin: rspice_core::netlist::NetlistSourceLocation::in_file("bug75.cir", 12),
                authored_coupling_name: "K3".into(),
                canonical_coupling_name: "K3".into(),
                qualified_coupling_name: "K3".into(),
                authored_inductor_name: "L2".into(),
                canonical_inductor_name: "L2".into(),
                qualified_inductor_name: "L2".into(),
                scope_name: None,
                reference_position: 2,
            },
        )),
    );

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.line, None);
    assert_eq!(diagnostic.source_line, Some(11));
    assert_eq!(
        diagnostic.source_path.as_deref(),
        Some(Path::new("bug75.cir"))
    );
    assert!(diagnostic.message.contains("Undefined inductor L2"));
    assert!(diagnostic.message.contains("Coupling: K3 (read as K3)"));
    assert!(diagnostic.message.contains("scope: top level"));
    assert!(diagnostic.message.contains("reference position 2"));
}

#[test]
fn output_symbol_diagnostics_preserve_origins_order_and_repetitions() {
    let error = rspice_core::netlist::ParseError::OutputSymbolValidation(Box::new(
        rspice_core::netlist::OutputSymbolValidationError {
            unresolved: vec![
                rspice_core::netlist::UnresolvedOutputSymbol {
                    directive: rspice_core::netlist::OutputDirectiveKind::Print,
                    origin: rspice_core::netlist::NetlistSourceLocation::in_memory(7),
                    operator: "V".into(),
                    symbol: "missing".into(),
                    kind: rspice_core::netlist::OutputSymbolKind::Node,
                },
                rspice_core::netlist::UnresolvedOutputSymbol {
                    directive: rspice_core::netlist::OutputDirectiveKind::Measure,
                    origin: rspice_core::netlist::NetlistSourceLocation::in_memory(11),
                    operator: "I".into(),
                    symbol: "Rbogus".into(),
                    kind: rspice_core::netlist::OutputSymbolKind::Device,
                },
                rspice_core::netlist::UnresolvedOutputSymbol {
                    directive: rspice_core::netlist::OutputDirectiveKind::Measure,
                    origin: rspice_core::netlist::NetlistSourceLocation::in_memory(11),
                    operator: "I".into(),
                    symbol: "Rbogus".into(),
                    kind: rspice_core::netlist::OutputSymbolKind::Device,
                },
            ],
        },
    ));

    let diagnostics = parse_error_diagnostics(error);

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].line, Some(6));
    assert_eq!(diagnostics[1].line, Some(10));
    assert!(diagnostics[0].message.contains("node `missing`"));
    assert!(diagnostics[1].message.contains("device `Rbogus`"));
    assert_eq!(diagnostics[1].message.matches("device `Rbogus`").count(), 2);
}

#[test]
fn startup_conflict_diagnostic_points_at_conflicting_card_and_preserves_first_origin() {
    let diagnostic =
        parse_error_diagnostic(rspice_core::netlist::ParseError::StartupDirectiveConflict(
            Box::new(rspice_core::netlist::StartupDirectiveConflictError {
                first_kind: rspice_core::netlist::StartupDirectiveKind::Ic,
                first: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 7),
                conflicting_kind: rspice_core::netlist::StartupDirectiveKind::NodeSet,
                conflicting: rspice_core::netlist::NetlistSourceLocation::in_file(
                    "included.cir",
                    11,
                ),
            }),
        ));

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.line, None);
    assert_eq!(diagnostic.source_line, Some(10));
    assert_eq!(
        diagnostic.source_path.as_deref(),
        Some(Path::new("included.cir"))
    );
    assert!(diagnostic.message.contains(".IC at deck.cir:7"));
    assert!(diagnostic.message.contains(".NODESET at included.cir:11"));
}

#[test]
fn editor_keeps_completion_symbols_when_only_output_validation_fails() {
    let source = "output validation\n\
                  V1 1 0 1\n\
                  .PRINT OP V(missing)\n\
                  .END\n";

    let (diagnostics, symbols) = parse_buffer(source);

    assert!(
        symbols.is_some(),
        "semantic lint must retain completion state"
    );
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, Some(2));
    assert!(diagnostics[0].message.contains("node `missing` via V"));
    assert!(diagnostics[0].message.contains(".PRINT"));
}

#[test]
fn parse_buffer_appends_unknown_reference_lints_after_clean_parse() {
    let source = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.op\n.end\n";

    let (diagnostics, symbols) = parse_buffer(source);

    assert!(symbols.is_some());
    assert_eq!(diagnostics.len(), 1);
    let diagnostic = &diagnostics[0];
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(&source[diagnostic.span.clone().unwrap()], "nchh");
    assert_eq!(diagnostic.fix.as_ref().unwrap().replacement, "nch");
}

#[test]
fn output_expression_diagnostic_is_localized_to_the_authored_request() {
    let error = rspice_core::netlist::ParseError::OutputExpressionValidation(Box::new(
        rspice_core::netlist::OutputExpressionValidationError {
            directive: rspice_core::netlist::OutputDirectiveKind::Print,
            origin: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 9),
            expression: "FABS(V(1))".into(),
            issue: rspice_core::netlist::OutputExpressionIssue::UnknownFunction {
                function: "FABS".into(),
            },
        },
    ));
    let diagnostic = parse_error_diagnostic_at(error, Some(Path::new("deck.cir")));
    assert_eq!(diagnostic.line, Some(8));
    assert_eq!(diagnostic.source_line, Some(8));
    assert!(diagnostic.message.contains(".PRINT"));
    assert!(diagnostic.message.contains("FABS"));
}

#[test]
fn diagnostic_strip_text_omits_fix_and_truncates_for_phone_width() {
    let source = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.end\n";
    let mut diagnostic = Diagnostic::current(
        "rspice.netlist.semantic",
        "SPICE-UNKNOWN-MODEL",
        DiagnosticSeverity::Error,
        "Unknown model `nchh` in this deck.",
    );
    diagnostic.source_line = Some(1);
    diagnostic.span = Some(18..22);
    diagnostic.line = Some(1);
    diagnostic.column = Some(11);
    diagnostic.fix = Some(crate::state::netlist_diagnostics::DiagnosticFix {
        label: "Replace with nch".to_string(),
        span: 18..22,
        replacement: "nch".to_string(),
    });
    diagnostic.refresh_canonical_location();

    let text = diagnostic_strip_text(&diagnostic, source, 160.0);

    assert!(!text.contains("fix:"));
    assert!(text.ends_with("..."), "{text}");
    assert!(diagnostic_strip_text_fits(&text, 160.0));
}

#[test]
fn editor_commit_rejects_generated_artifact_mutation() {
    let mut state = AppState::default();
    state.model.simulation.source.netlist_content = "generated\n.op\n.end\n".to_owned();
    state.code.netlist.revision = 4;

    assert!(!commit_owned_source_edit(
        &mut state,
        "silently edited\n.end\n",
        1,
        2.5,
    ));
    assert!(state.model.workspace.content.netlist_source.is_none());
    assert_eq!(
        state.model.simulation.source.netlist_content,
        "generated\n.op\n.end\n"
    );
    assert!(!state.model.workspace.content.netlist_source_dirty);
    assert_eq!(state.code.netlist.revision, 4);
    assert!(state.runs.manual_run.edited_lines.is_empty());
}

#[test]
fn editor_commit_updates_only_an_existing_owned_source() {
    let mut state = AppState::default();
    state.model.workspace.content.netlist_source = Some("owned\n.op\n.end\n".to_owned());
    state.code.netlist.active_document = super::super::ActiveNetlistDocument::OwnedSource;
    state.model.workspace.content.netlist_source_path =
        Some(std::path::PathBuf::from("imported/owned.cir"));
    state.model.simulation.source.netlist_content = "owned\n.op\n.end\n".to_owned();
    state.code.netlist.revision = 9;

    assert!(commit_owned_source_edit(
        &mut state,
        "owned\n.tran 1n 1u\n.end\n",
        1,
        7.25,
    ));
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some("owned\n.tran 1n 1u\n.end\n")
    );
    assert_eq!(
        state.model.simulation.source.netlist_content,
        "owned\n.tran 1n 1u\n.end\n"
    );
    assert_eq!(
        state.model.workspace.content.netlist_source_path.as_deref(),
        Some(std::path::Path::new("imported/owned.cir"))
    );
    assert!(state.model.workspace.content.netlist_source_dirty);
    assert_eq!(state.code.netlist.revision, 10);
    assert_eq!(state.code.netlist.last_edit_time, 7.25);
    assert!(state.runs.manual_run.edited_lines.contains(&1));
}
