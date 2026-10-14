//! Build settings: a Verilog-A view's build profile as a form, driven the way
//! a reader drives it — opened from its command, filled in, saved, and read
//! back from the profile document it wrote.

use std::collections::BTreeMap;

use super::dialogs::veriloga_build_settings::save_veriloga_build_settings;
use super::veriloga_design_tests::{
    DESKTOP, add_veriloga_cell, open_in_design, open_project_app, themed_context, wait_for_compile,
};
use super::*;
use crate::state::ProjectSourceRole;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::documents::code_workspace;
use crate::workbench::{MessageId, UiTextLocale};
use rspice_simulation::project_veriloga::build_profile::{
    VerilogAFallbackPolicy, resolve_veriloga_build_profile,
};

/// Two top modules in one file, so the entry module is a real choice.
const TWO_MODULES: &str = "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real g = 1m;\n  analog I(p, n) <+ g * V(p, n);\nendmodule\n\nmodule gain_fast(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real g = 2m;\n  analog I(p, n) <+ g * V(p, n);\nendmodule\n";

/// A module whose conductance only the build profile can supply.
const NEEDS_A_DEFINE: &str = "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real g = `GAIN_VALUE;\n  analog I(p, n) <+ g * V(p, n);\nendmodule\n";

fn messages() -> crate::workbench::MessageCatalog {
    crate::workbench::MessageCatalog::new(UiTextLocale::EnglishUnitedStates)
}

/// A project with the `gain` Verilog-A view open in Design, its source
/// replaced by `source`.
fn veriloga_view_holding(source: &str) -> RSpiceApp {
    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let selected = code_workspace::selected_veriloga_source(&app).unwrap();
    code_workspace::replace_selected_veriloga_file(
        &mut app,
        &selected,
        "model.va",
        source.to_owned(),
    )
    .unwrap();
    app
}

fn draft(app: &mut RSpiceApp) -> &mut VerilogABuildSettingsDraft {
    app.state
        .dialogs
        .veriloga_build_settings
        .as_mut()
        .expect("Build settings is open")
}

/// One whole application frame with `events` delivered to it.
fn frame_with(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    size: egui::Vec2,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            events,
            ..Default::default()
        },
        |ui| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        },
    )
}

/// The accessibility node the frame drew and named `name`, if any.
fn node<'a>(output: &'a egui::FullOutput, name: &str) -> Option<&'a egui::accesskit::Node> {
    named_nodes(output, name).next()
}

/// The text field the frame drew and named `name`, if any.
fn field_node<'a>(output: &'a egui::FullOutput, name: &str) -> Option<&'a egui::accesskit::Node> {
    named_nodes(output, name).find(|node| node.role() == egui::accesskit::Role::TextInput)
}

fn named_nodes<'a>(
    output: &'a egui::FullOutput,
    name: &str,
) -> impl Iterator<Item = &'a egui::accesskit::Node> {
    output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .map(|(_, node)| node)
        .filter(move |node| node.label() == Some(name) && node.bounds().is_some())
}

fn node_rect(node: &egui::accesskit::Node) -> egui::Rect {
    let bounds = node.bounds().expect("a named control has bounds");
    egui::Rect::from_min_max(
        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
    )
}

/// Press and release the primary button over the control named `name`, where
/// it stands once a window opened this frame has settled.
fn click(ctx: &egui::Context, app: &mut RSpiceApp, name: &str) {
    frame_with(ctx, app, DESKTOP, Vec::new());
    let output = frame_with(ctx, app, DESKTOP, Vec::new());
    let at =
        node_rect(node(&output, name).unwrap_or_else(|| panic!("{name} is on screen"))).center();
    frame_with(
        ctx,
        app,
        DESKTOP,
        vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame_with(
        ctx,
        app,
        DESKTOP,
        vec![egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
}

/// Every field the form shows is written to the profile document and read
/// back from it: what the form saves is what the next form opens on.
#[test]
fn build_settings_round_trips_every_field_through_the_profile_document() {
    let mut app = veriloga_view_holding(TWO_MODULES);
    assert!(Command::VerilogABuildSettings.is_enabled(&app));
    Command::VerilogABuildSettings.execute(&mut app);

    let form = draft(&mut app);
    assert_eq!(form.modules, ["gain", "gain_fast"]);
    assert_eq!(form.entry_module, "gain", "the module the view binds");
    form.entry_module = "gain_fast".to_owned();
    form.generated_rust = true;
    form.native_x64_jit = true;
    form.reject_fallback = true;
    form.include_paths = vec!["include".to_owned(), "models/common".to_owned()];
    form.defines = vec![("WIDTH".to_owned(), "8".to_owned())];
    // Still in its field: Save takes it rather than dropping it.
    form.define_draft = "FAST".to_owned();
    form.checks.convergence = false;
    form.checks.portability = false;

    assert!(save_veriloga_build_settings(&mut app).expect("the save is made"));
    assert!(app.state.dialogs.veriloga_build_settings.is_none());

    let selected = code_workspace::selected_veriloga_source(&app).unwrap();
    let resolved = resolve_veriloga_build_profile(selected.bundle()).unwrap();
    assert_eq!(
        resolved.logical_path.as_deref(),
        Some(".rspice/veriloga-build.toml"),
        "a source on the built-in defaults gains the profile document"
    );
    assert_eq!(
        selected
            .bundle()
            .role_for_path(".rspice/veriloga-build.toml"),
        Some(ProjectSourceRole::VerilogABuildProfile)
    );
    let profile = resolved.profile;
    assert_eq!(profile.entry_modules, ["gain_fast"]);
    assert!(profile.targets.portable_interpreter);
    assert!(profile.targets.generated_rust);
    assert!(profile.targets.native_x64_jit);
    assert_eq!(profile.targets.fallback, VerilogAFallbackPolicy::Reject);
    assert_eq!(profile.include_paths, ["include", "models/common"]);
    assert_eq!(
        profile.preprocessor.defines,
        BTreeMap::from([
            ("FAST".to_owned(), String::new()),
            ("WIDTH".to_owned(), "8".to_owned()),
        ])
    );
    assert!(profile.preprocessor.undefines.is_empty());
    assert!(profile.checks.units_and_ranges);
    assert!(!profile.checks.convergence);
    assert!(profile.checks.discontinuities);
    assert!(profile.checks.hidden_state);
    assert!(!profile.checks.portability);
    assert_eq!(
        selected.selected_module(),
        Some("gain_fast"),
        "the view compiles the entry module chosen"
    );

    Command::VerilogABuildSettings.execute(&mut app);
    let reread = draft(&mut app);
    assert_eq!(reread.entry_module, "gain_fast");
    assert!(reread.generated_rust && reread.native_x64_jit && reread.reject_fallback);
    assert_eq!(reread.include_paths, ["include", "models/common"]);
    assert_eq!(
        reread.defines,
        [
            ("FAST".to_owned(), String::new()),
            ("WIDTH".to_owned(), "8".to_owned()),
        ]
    );
    assert_eq!(reread.checks, profile.checks);
}

/// Another entry module moves the profile and the view's binding together,
/// as one step: one Undo puts both back, and one Redo makes both again.
#[test]
fn one_undo_takes_back_the_entry_module_in_the_profile_and_the_binding() {
    let mut app = veriloga_view_holding(TWO_MODULES);
    let before = code_workspace::selected_veriloga_source(&app).unwrap();
    assert_eq!(before.selected_module(), Some("gain"));
    Command::VerilogABuildSettings.execute(&mut app);
    draft(&mut app).entry_module = "gain_fast".to_owned();
    assert!(save_veriloga_build_settings(&mut app).unwrap());

    let saved = code_workspace::selected_veriloga_source(&app).unwrap();
    assert_eq!(saved.selected_module(), Some("gain_fast"));
    let profile = resolve_veriloga_build_profile(saved.bundle())
        .unwrap()
        .profile;
    assert_eq!(profile.entry_modules, ["gain_fast"]);

    assert!(Command::Undo.is_enabled(&app));
    Command::Undo.execute(&mut app);
    let undone = code_workspace::selected_veriloga_source(&app).unwrap();
    assert_eq!(
        undone.selected_module(),
        Some("gain"),
        "the binding is back"
    );
    assert_eq!(
        undone.bundle().closure_digest(),
        before.bundle().closure_digest(),
        "the profile is back"
    );
    assert!(
        resolve_veriloga_build_profile(undone.bundle())
            .unwrap()
            .logical_path
            .is_none(),
        "the profile document the save added is gone"
    );

    assert!(Command::Redo.is_enabled(&app));
    Command::Redo.execute(&mut app);
    let redone = code_workspace::selected_veriloga_source(&app).unwrap();
    assert_eq!(redone.selected_module(), Some("gain_fast"));
    assert_eq!(
        redone.bundle().closure_digest(),
        saved.bundle().closure_digest()
    );
}

/// The step never overwrites a later edit: once the source is edited after
/// the save, Undo leaves both the source and the binding alone.
#[test]
fn build_settings_undo_refuses_after_a_later_source_edit() {
    let mut app = veriloga_view_holding(TWO_MODULES);
    Command::VerilogABuildSettings.execute(&mut app);
    draft(&mut app).entry_module = "gain_fast".to_owned();
    assert!(save_veriloga_build_settings(&mut app).unwrap());
    let saved = code_workspace::selected_veriloga_source(&app).unwrap();
    let edited = format!("{TWO_MODULES}// edited after the save\n");
    code_workspace::replace_selected_veriloga_file(&mut app, &saved, "model.va", edited).unwrap();
    let later = code_workspace::selected_veriloga_source(&app).unwrap();

    assert!(!app.state.can_undo_project_design());
    Command::Undo.execute(&mut app);
    let after = code_workspace::selected_veriloga_source(&app).unwrap();
    assert_eq!(
        after.bundle().closure_digest(),
        later.bundle().closure_digest()
    );
    assert_eq!(after.selected_module(), Some("gain_fast"));
}

/// Save on a read-only project is refused, and nothing is written; the form
/// still opens there, to be read.
#[test]
fn build_settings_save_is_refused_on_a_read_only_project() {
    let mut app = veriloga_view_holding(TWO_MODULES);
    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );
    assert!(Command::VerilogABuildSettings.is_enabled(&app));
    Command::VerilogABuildSettings.execute(&mut app);
    let before = code_workspace::selected_veriloga_source(&app).unwrap();
    draft(&mut app).generated_rust = true;

    let refusal = save_veriloga_build_settings(&mut app).expect_err("read-only refuses");
    assert_eq!(
        refusal,
        messages().text(MessageId::VerilogABuildSettingsReadOnly)
    );
    let after = code_workspace::selected_veriloga_source(&app).unwrap();
    assert_eq!(
        after.bundle().closure_digest(),
        before.bundle().closure_digest()
    );
    assert!(
        app.state.dialogs.veriloga_build_settings.is_some(),
        "the form stays open"
    );

    // The form says why, and offers neither save.
    let ctx = themed_context();
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let output = frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    for verb in [
        MessageId::VerilogABuildSettingsSave,
        MessageId::VerilogABuildSettingsPrimary,
    ] {
        let button = node(&output, &messages().text(verb)).expect("the footer is drawn");
        assert!(
            button.is_disabled(),
            "{verb:?} is offered on a read-only project"
        );
    }
}

/// A refused entry is stated under its own field and holds Save, which would
/// otherwise drop it; fixing it lets the save through.
#[test]
fn a_refused_entry_is_stated_under_its_field_and_holds_save() {
    let mut app = veriloga_view_holding(TWO_MODULES);
    Command::VerilogABuildSettings.execute(&mut app);
    draft(&mut app).include_draft = "../outside".to_owned();
    let ctx = themed_context();
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let output = frame_with(&ctx, &mut app, DESKTOP, Vec::new());

    let refusal = messages().text(MessageId::VerilogABuildSettingsIncludeInvalid);
    let line = output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .map(|(_, node)| node)
                .find(|node| node.value() == Some(refusal.as_str()) && node.bounds().is_some())
        })
        .expect("the refusal is stated");
    let field = field_node(
        &output,
        &messages().text(MessageId::VerilogABuildSettingsIncludePaths),
    )
    .expect("the include path field");
    let (line, field) = (node_rect(line), node_rect(field));
    assert!(
        line.top() >= field.bottom() && line.top() - field.bottom() < 8.0,
        "under its field: line {line:?}, field {field:?}"
    );
    assert!(
        node(
            &output,
            &messages().text(MessageId::VerilogABuildSettingsSave)
        )
        .is_some_and(egui::accesskit::Node::is_disabled)
    );
    assert_eq!(
        save_veriloga_build_settings(&mut app).expect_err("held"),
        refusal
    );

    draft(&mut app).include_draft = "include".to_owned();
    assert!(save_veriloga_build_settings(&mut app).unwrap());
    let selected = code_workspace::selected_veriloga_source(&app).unwrap();
    let profile = resolve_veriloga_build_profile(selected.bundle())
        .unwrap()
        .profile;
    assert_eq!(profile.include_paths, ["include"]);
}

/// Save & compile saves the settings and compiles with them: a module whose
/// value only a define supplies fails without it and compiles with it.
#[test]
fn save_and_compile_compiles_with_the_new_settings() {
    let mut app = veriloga_view_holding(NEEDS_A_DEFINE);
    let ctx = themed_context();
    Command::CompileVerilogA.execute(&mut app);
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    wait_for_compile(&ctx, &mut app);
    assert!(
        app.state.ui.code_workspace.veriloga.receipt.is_none(),
        "without the define the module does not compile"
    );
    assert!(!app.state.ui.code_workspace.veriloga.last_failure.is_empty());

    Command::VerilogABuildSettings.execute(&mut app);
    draft(&mut app).define_draft = "GAIN_VALUE=2m".to_owned();
    click(
        &ctx,
        &mut app,
        &messages().text(MessageId::VerilogABuildSettingsPrimary),
    );
    assert!(
        app.state.dialogs.veriloga_build_settings.is_none(),
        "the form closed: {:?}",
        app.state
            .dialogs
            .veriloga_build_settings
            .as_ref()
            .map(|form| &form.error)
    );
    wait_for_compile(&ctx, &mut app);
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    wait_for_compile(&ctx, &mut app);

    let selected = code_workspace::selected_veriloga_source(&app).unwrap();
    let profile = resolve_veriloga_build_profile(selected.bundle())
        .unwrap()
        .profile;
    assert_eq!(
        profile
            .preprocessor
            .defines
            .get("GAIN_VALUE")
            .map(String::as_str),
        Some("2m")
    );
    let veriloga = &app.state.ui.code_workspace.veriloga;
    assert!(
        veriloga.last_failure.is_empty(),
        "{:?}",
        veriloga.last_failure
    );
    let receipt = veriloga.receipt.as_ref().expect("the module compiled");
    assert!(
        selected.matches_token(
            app.state.workspace.content.project.id(),
            receipt.compilation.token()
        ),
        "with the settings just saved"
    );
}

/// Renders for review: Build settings over a typical profile — one module,
/// two include paths, two defines — at desktop and phone sizes, dark and
/// light.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_build_settings_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    for (name, size, coarse) in [
        ("1440x900", DESKTOP, false),
        ("1400x900", egui::vec2(1400.0, 900.0), false),
        ("390x844", egui::vec2(390.0, 844.0), true),
    ] {
        for (mode, mode_name) in [
            (crate::ui::Mode::Dark, "dark"),
            (crate::ui::Mode::Light, "light"),
        ] {
            let mut app = open_project_app();
            let (reference, _) = add_veriloga_cell(
                &mut app.state,
                "gain_cell",
                "models/gain_cell.va",
                "gain_cell",
            );
            open_in_design(&mut app.state, &reference);
            app.state.workbench.coarse_pointer = coarse;
            Command::VerilogABuildSettings.execute(&mut app);
            let form = draft(&mut app);
            form.include_paths = vec!["include".to_owned(), "models/common".to_owned()];
            form.defines = vec![
                ("WIDTH".to_owned(), "8".to_owned()),
                ("FAST".to_owned(), String::new()),
            ];
            form.generated_rust = true;
            let theme = crate::ui::Theme {
                mode,
                ..crate::ui::Theme::default()
            };
            // The frame re-resolves the reader's theme every pass, so the
            // mode is theirs, not only the renderer's.
            app.state.ui.theme = theme;
            let canvas = crate::ui::raster::render_themed(theme, size, |ui, _| {
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(ui.ctx());
            });
            let path = directory.join(format!("veriloga-build-settings-{name}-{mode_name}.png"));
            std::fs::write(&path, canvas.png(size.y as usize)).expect("write the render");
            writeln!(report, "{}", path.display()).ok();
        }
    }
}

/// Build settings sits right after Compile and Generated netlist on the
/// desktop toolbar. A phone's toolbar keeps only those two; the command
/// palette reaches it there.
#[test]
fn build_settings_follows_compile_on_the_desktop_toolbar_only() {
    let mut app = veriloga_view_holding(TWO_MODULES);
    let ctx = themed_context();
    let label = Command::VerilogABuildSettings.spec().label;
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let output = frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let settings = node_rect(node(&output, label).expect("on the desktop toolbar"));
    let netlist = node_rect(
        node(&output, &messages().text(MessageId::CodeGeneratedNetlist))
            .expect("Generated netlist is on the toolbar"),
    );
    assert!(
        settings.left() > netlist.left() && (settings.center().y - netlist.center().y).abs() < 1.0,
        "after Generated netlist, on its row: {settings:?} {netlist:?}"
    );

    let phone = egui::vec2(390.0, 844.0);
    app.state.workbench.coarse_pointer = true;
    frame_with(&ctx, &mut app, phone, Vec::new());
    let output = frame_with(&ctx, &mut app, phone, Vec::new());
    assert!(node(&output, label).is_none(), "not on a phone's toolbar");
    assert!(
        crate::workbench::commands::vocabulary::COMMAND_REGISTRY
            .contains(&Command::VerilogABuildSettings)
    );
}
