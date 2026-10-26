//! Command palette search, navigation, and input ownership regressions.

use egui::{Context, Id, Key, Modifiers, Order, Rect, pos2, vec2};

use crate::state::command_catalog::CommandPlatform;
use crate::ui::icons::Icon;
use crate::workbench::RSpiceApp;
use crate::workbench::app::command_palette::*;
use crate::workbench::framework::command_registry::CommandSpec;
use crate::workbench::results::commands::ResultsCommand;

#[test]
fn task_focused_menu_can_open_the_palette_in_an_exact_scope() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.command_palette.open_in_scope("Design");

    assert!(app.state.workbench.command_palette.open);
    assert_eq!(
        app.state.workbench.command_palette.initial_scope.as_deref(),
        Some("Design")
    );
    assert_eq!(
        PaletteScope::from_label("design"),
        Some(PaletteScope::Design)
    );
    assert_eq!(PaletteScope::from_label("unknown"), None);
}
use crate::workbench::commands::vocabulary::{
    COMMAND_REGISTRY, Command, command_registry, legacy_command,
};

/// What the registry declares about `command`.
fn spec(command: Command) -> &'static CommandSpec {
    command_registry()
        .spec(command.key())
        .expect("the registry declares the command")
}

/// The commands the palette lists, as values, in its order.
fn palette_commands_listed() -> Vec<Command> {
    palette_commands(&RSpiceApp::test_instance())
        .filter_map(|command| legacy_command(command.key()))
        .collect()
}

/// `command`'s entry as the palette lists it.
fn palette_entry(command: Command) -> PaletteEntry {
    palette_commands(&RSpiceApp::test_instance())
        .find(|listed| listed.key() == command.key())
        .map(PaletteEntry::Command)
        .expect("the palette lists the command")
}

#[test]
fn desktop_and_phone_layouts_match_the_mockup_contract() {
    let desktop = PaletteLayout::resolve(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0)),
        false,
        44.0,
    );
    assert_eq!(desktop.surface.width(), 650.0);
    assert_eq!(desktop.surface.height(), 560.0);
    assert_eq!(desktop.surface.left(), 395.0);
    assert_eq!(desktop.surface.top(), 72.0);
    assert_eq!(desktop.regions().search.height(), 48.0);
    assert_eq!(desktop.regions().scope.height(), 37.0);
    assert_eq!(desktop.regions().footer.height(), 31.0);

    let phone = PaletteLayout::resolve(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(390.0, 844.0)),
        true,
        44.0,
    );
    assert!(phone.narrow);
    assert_eq!(
        phone.surface,
        Rect::from_min_max(pos2(6.0, 6.0), pos2(384.0, 838.0))
    );
    assert_eq!(phone.scope_height, 59.0);

    let touch_desktop = PaletteLayout::resolve(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(1024.0, 768.0)),
        true,
        44.0,
    );
    assert!(!touch_desktop.narrow);
    assert_eq!(touch_desktop.scope_height, 59.0);

    // A 48 px target chosen in Accessibility grows the scope band with it.
    let larger = PaletteLayout::resolve(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(1024.0, 768.0)),
        true,
        48.0,
    );
    assert_eq!(larger.scope_height, 63.0);
}

#[test]
fn palette_is_a_typed_registry_projection() {
    let commands = palette_commands_listed();
    assert!(!commands.contains(&Command::CommandPalette));
    assert!(!commands.contains(&Command::Cancel));
    assert!(commands.contains(&Command::OpenProject));
    assert!(commands.contains(&Command::GenerateNetlist));
    assert!(
        commands
            .iter()
            .all(|command| COMMAND_REGISTRY.contains(command))
    );
    assert!(commands.iter().all(|command| command.palette_visible()));
}

#[test]
fn registered_command_projection_is_total_without_signal_substitutes() {
    let commands = palette_commands_listed();
    assert!(
        commands
            .iter()
            .copied()
            .all(|command| { PaletteScope::for_command(spec(command)) != PaletteScope::All })
    );
    let assigned = PaletteScope::ALL
        .into_iter()
        .skip(1)
        .map(|scope| {
            commands
                .iter()
                .copied()
                .filter(|command| scope.contains(spec(*command)))
                .count()
        })
        .sum::<usize>();
    assert_eq!(assigned, commands.len());
    assert!(
        commands
            .iter()
            .copied()
            .all(|command| PaletteScope::for_command(spec(command)) != PaletteScope::Signals),
        "signal rows must represent real result waveforms, not generic commands"
    );
}

/// Opening a workspace or a project page is navigating. The palette files a
/// command by its group alone, so each of these must carry a group the
/// Navigate scope takes.
#[test]
fn opening_a_workspace_or_a_project_page_is_navigating() {
    let (mut workspaces, mut pages) = (0, 0);
    for &command in COMMAND_REGISTRY {
        match command {
            Command::OpenWorkspace(_) => workspaces += 1,
            Command::ProjectPage(_) => pages += 1,
            _ => continue,
        }
        assert_eq!(
            PaletteScope::for_command(spec(command)),
            PaletteScope::Navigate,
            "{command:?} is filed under {}",
            spec(command).group
        );
    }
    assert_eq!(workspaces, crate::workbench::state::Workspace::ALL.len());
    assert!(pages > 0, "the registry lists the project pages");
}

#[test]
fn command_scopes_and_metadata_search_only_return_registered_commands() {
    let design = palette_commands_listed()
        .into_iter()
        .filter(|command| PaletteScope::Design.contains(spec(*command)))
        .filter(|command| match_entry(&palette_entry(*command), "wire").is_some())
        .collect::<Vec<_>>();
    assert_eq!(
        design,
        vec![Command::PlaceWire, Command::JoinCollinearWires]
    );

    assert_eq!(
        PaletteScope::for_command(spec(Command::PlaceProbe)),
        PaletteScope::Design
    );
    assert_eq!(
        PaletteScope::for_command(spec(Command::PlaceBus)),
        PaletteScope::Design
    );
    assert_eq!(
        PaletteScope::for_command(spec(Command::PlaceBusTap)),
        PaletteScope::Design
    );
    assert_eq!(
        PaletteScope::for_command(spec(Command::PlaceJunction)),
        PaletteScope::Design
    );
    assert_eq!(
        PaletteScope::for_command(
            command_registry()
                .spec(ResultsCommand::ExportWaveformsCsv.key())
                .expect("the registry declares the command")
        ),
        PaletteScope::Results
    );

    let help = palette_commands_listed()
        .into_iter()
        .filter(|command| PaletteScope::Help.contains(spec(*command)))
        .collect::<Vec<_>>();
    assert!(help.contains(&Command::KeyboardShortcuts));
    assert!(help.contains(&Command::About));
}

#[test]
fn bus_commands_use_the_mockup_palette_glyphs() {
    assert_eq!(row_icon(&palette_entry(Command::PlaceBus)), Icon::Bus);
    assert_eq!(row_icon(&palette_entry(Command::PlaceBusTap)), Icon::Bus);
}

#[test]
fn stretch_selection_uses_the_exact_pointer_glyph() {
    assert_eq!(
        row_icon(&palette_entry(Command::StretchSelection)),
        Icon::Select
    );
}

#[test]
fn create_array_uses_the_exact_grid_glyph() {
    assert_eq!(
        row_icon(&palette_entry(Command::ArraySelection)),
        Icon::Grid
    );
}

#[test]
fn replace_instance_uses_the_exact_refresh_glyph() {
    assert_eq!(
        row_icon(&palette_entry(Command::ReplaceInstance)),
        Icon::Refresh
    );
}

#[test]
fn fuzzy_matching_marks_display_characters_and_searches_registry_metadata() {
    assert_eq!(
        match_spans("Open generated netlist", "ogn"),
        Some((4, vec![0, 5, 7]))
    );
    let command = palette_entry(Command::GenerateNetlist);
    let (_, marks) = match_entry(&command, "generated netlist").expect("label terms match");
    assert!(!marks.is_empty());
    assert!(match_entry(&command, "simulate").is_some());
    assert!(match_entry(&command, "nonexistent-token").is_none());
    assert_eq!(match_spans("Draw symbol circle", "wire"), None);
}

#[test]
fn palette_names_are_canonical_labels_without_ellipsis() {
    assert_eq!(command_name(spec(Command::OpenProject)), "Open project");
    assert_eq!(command_name(spec(Command::SaveAll)), "Save all");
    assert_eq!(
        command_name(spec(Command::GenerateNetlist)),
        "Open generated netlist"
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn test_app() -> RSpiceApp {
    let state = crate::workbench::app_state::AppState::default();
    let automation_runtime_project_id = state.model.workspace.content.project.id();
    RSpiceApp {
        state,
        first_frame: false,
        autosave_last: None,
        applied_theme: None,
        last_window_title: String::new(),
        symbol_library: None,
        simulation_controller: crate::simulation::SimulationController::new(),
        automation_runtime: crate::automation_runtime::NativeAutomationRuntime::discover(),
        automation_runtime_project_id,
        cloud: crate::workbench::live_session::CloudHandles {
            account: crate::services::cloud_account::CloudAccountService::unconfigured(),
            live_session: crate::workbench::live_session::LiveSessionEngine::default(),
        },
        model_hub: crate::services::model_hub::ModelHubService::unavailable(
            "This test instance runs without a model-pack store.",
        ),
        file_workflow_io: Box::new(crate::io::NativeFileWorkflowIo),
        export_workflow_io: Box::new(
            crate::workbench::workflows::export_workflow::NativeExportWorkflowIo,
        ),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn raw_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1280.0, 800.0))),
        events,
        ..Default::default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn key_event(key: Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn opening_shortcut_and_repeated_letters_in_one_frame_preserve_the_search() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let mut app = test_app();
    let query = "Review comments";
    let mut events = Vec::new();
    let mut press = |key, modifiers, text: Option<char>| {
        events.push(egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        });
        if let Some(character) = text {
            events.push(egui::Event::Text(character.to_string()));
        }
        events.push(egui::Event::Key {
            key,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers,
        });
    };
    press(Key::K, Modifiers::CTRL | Modifiers::COMMAND, None);
    for character in query.chars() {
        let key = if character == ' ' {
            Key::Space
        } else {
            Key::from_name(&character.to_ascii_uppercase().to_string()).unwrap()
        };
        let modifiers = if character.is_uppercase() {
            Modifiers::SHIFT
        } else {
            Modifiers::NONE
        };
        press(key, modifiers, Some(character));
    }
    let _ = ctx.run_ui(raw_input(events), |ctx| {
        app.handle_shortcuts(ctx);
        render_command_palette(&mut app, ctx);
    });
    assert!(app.state.workbench.command_palette.open);
    assert_eq!(app.state.workbench.command_palette.query, query);

    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        app.handle_shortcuts(ctx);
        render_command_palette(&mut app, ctx);
    });
    assert_eq!(app.state.workbench.command_palette.query, query);
    assert!(!app.state.design.bus_tap.open);
}

#[cfg(not(target_arch = "wasm32"))]
fn primary_key_event(key: Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: Modifiers::CTRL | Modifiers::COMMAND,
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn opening_input_keeps_palette_editing_keys() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let mut app = test_app();
    let _ = ctx.run_ui(
        raw_input(vec![
            primary_key_event(Key::K),
            egui::Event::Text("wrong".to_owned()),
            primary_key_event(Key::A),
            egui::Event::Paste("Review comments".to_owned()),
        ]),
        |ctx| {
            app.handle_shortcuts(ctx);
            render_command_palette(&mut app, ctx);
        },
    );
    assert_eq!(app.state.workbench.command_palette.query, "Review comments");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn opening_input_respects_custom_palette_bindings() {
    use crate::state::shortcuts::{ShortcutBindingSlot, ShortcutSequence, ShortcutStroke};

    for chord in [false, true] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let mut app = test_app();
        let strokes = if chord {
            vec![
                ShortcutStroke::new(Key::K, true, false, false),
                ShortcutStroke::new(Key::P, true, false, false),
            ]
        } else {
            vec![ShortcutStroke::new(Key::K, false, false, false)]
        };
        app.state
            .model
            .preferences
            .shortcuts_mut()
            .unwrap()
            .set_binding(
                &crate::workbench::commands::vocabulary::CommandRegistry,
                Command::CommandPalette.key(),
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                Some(ShortcutSequence::new(strokes).unwrap()),
            )
            .unwrap();
        if chord {
            let _ = ctx.run_ui(raw_input(vec![primary_key_event(Key::K)]), |ctx| {
                app.handle_shortcuts(ctx)
            });
            assert!(!app.state.workbench.command_palette.open);
        }
        let mut events = if chord {
            vec![primary_key_event(Key::P)]
        } else {
            vec![key_event(Key::K), egui::Event::Text("k".to_owned())]
        };
        if chord {
            events.extend([
                egui::Event::Text("wrong".to_owned()),
                primary_key_event(Key::A),
            ]);
        }
        events.push(egui::Event::Paste("Review comments".to_owned()));
        let _ = ctx.run_ui(raw_input(events), |ctx| {
            app.handle_shortcuts(ctx);
            render_command_palette(&mut app, ctx);
        });
        assert_eq!(app.state.workbench.command_palette.query, "Review comments");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn opening_input_is_owned_before_background_text_edits() {
    for before in ["", " before"] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let underlying_id = Id::new("palette-opening-background-editor");
        let mut underlying = "unchanged".to_owned();
        let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id))
                    .request_focus();
            });
        });
        let mut app = test_app();
        let _ = ctx.run_ui(
            raw_input(vec![
                egui::Event::Text(before.to_owned()),
                primary_key_event(Key::K),
                egui::Event::Text("Review comments".to_owned()),
            ]),
            |ctx| {
                app.handle_shortcuts(ctx);
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
                });
                render_command_palette(&mut app, ctx);
            },
        );
        assert_eq!(underlying, format!("unchanged{before}"));
        assert_eq!(app.state.workbench.command_palette.query, "Review comments");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn empty_all_scope_uses_mockup_suggestions_and_explicit_scopes_remain_complete() {
    let mut app = test_app();
    app.state.workbench.command_palette.recent =
        vec![Command::PlaceProbe.key(), Command::Save.key()];
    let all = palette_rows(
        &app,
        "",
        PaletteScope::All,
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    assert_eq!(all[0].entry.command(), Some(Command::PlaceProbe.key()));
    assert_eq!(all[1].entry.command(), Some(Command::Save.key()));
    assert_eq!(all.len(), SUGGESTION_LIMIT);
    assert_eq!(
        all.iter()
            .filter(|row| row.entry.command() == Some(Command::Save.key()))
            .count(),
        1
    );
    assert!(all.iter().all(|row| row.section_label() != "Recent"));

    let signals = palette_rows(
        &app,
        "",
        PaletteScope::Signals,
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    assert!(
        signals.is_empty(),
        "no result data means no invented signal rows"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn real_cellview_records_are_searchable_and_open_the_workspace_document() {
    let mut app = test_app();
    let mut library = crate::state::Library::new("precision_analog");
    let mut cell = crate::state::Cell::new("gain_stage");
    cell.description = "Precision front-end amplifier".to_owned();
    cell.add_view(crate::state::View::new(
        "schematic",
        crate::state::ViewType::Schematic,
    ));
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);

    let mut rows = palette_rows(
        &app,
        "precision gain_stage",
        PaletteScope::Design,
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    assert_eq!(rows.len(), 1);
    assert!(rows[0].detail().contains("Precision front-end amplifier"));
    let entry = rows.remove(0).entry;
    assert!(matches!(entry, PaletteEntry::CellView(_)));
    entry.execute(&mut app).expect("open real cellview");

    assert_eq!(
        app.state.model.workspace.content.active_view,
        crate::state::CellViewRef::new("precision_analog", "gain_stage", "schematic")
    );
    assert_eq!(
        app.state
            .model
            .library_manager
            .selected_lcv_path()
            .as_deref(),
        Some("precision_analog/gain_stage/schematic")
    );
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Design
    );
}

/// New view… was reachable only from the project library's row menu, and a
/// Verilog-A device only by way of New Cell and New View. Both are palette
/// rows now, enabled where their dialogs could create something, and each
/// opens its dialog.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn new_view_and_new_veriloga_device_are_palette_commands_that_open_their_dialogs() {
    let mut app = test_app();
    app.state.open_project_session();
    let mut library = crate::state::Library::new("behavioral");
    let mut cell = crate::state::Cell::new("gain");
    cell.add_view(crate::state::View::new(
        "schematic",
        crate::state::ViewType::Schematic,
    ));
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .library_manager
        .select_cell("behavioral", "gain");

    for (query, command) in [
        ("Verilog-A device", Command::NewVerilogADevice),
        ("New view", Command::NewView),
    ] {
        let rows = palette_rows(
            &app,
            query,
            PaletteScope::All,
            CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        );
        let row = rows
            .into_iter()
            .find(|row| row.entry.command() == Some(command.key()))
            .unwrap_or_else(|| panic!("{query:?} finds {command:?}"));
        assert!(row.blocked.is_none(), "{command:?}: {:?}", row.blocked);
        row.entry.execute(&mut app).expect("the command runs");
    }

    assert!(app.state.code.new_veriloga_device.open);
    assert!(app.state.project.new_view_dialog);
    assert_eq!(app.state.project.new_view_library, "behavioral");
    assert_eq!(app.state.project.new_view_cell, "gain");

    // With nothing it could write, neither is offered as runnable.
    let mut read_only = test_app();
    read_only.state.open_project_session();
    let names = read_only
        .state
        .model
        .library_manager
        .libraries_sorted()
        .iter()
        .map(|library| library.name.clone())
        .collect::<Vec<_>>();
    for name in names {
        read_only
            .state
            .model
            .library_manager
            .get_library_mut(&name)
            .expect("the library")
            .read_only = true;
    }
    for command in [Command::NewVerilogADevice, Command::NewView] {
        assert!(!command.is_enabled(&read_only), "{command:?}");
    }
}

/// Build settings is a palette row while a Verilog-A view is in front, which
/// is how a phone reaches it: its toolbar keeps Compile and Generated netlist.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn veriloga_build_settings_is_a_palette_command_that_opens_its_form() {
    use crate::workbench::app::veriloga_design_tests::{
        add_veriloga_cell, open_in_design, open_project_app,
    };

    let mut app = open_project_app();
    let find = |app: &RSpiceApp| {
        palette_rows(
            app,
            "Build settings",
            PaletteScope::All,
            CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        )
        .into_iter()
        .find(|row| row.entry.command() == Some(Command::VerilogABuildSettings.key()))
    };
    assert!(
        find(&app).is_none_or(|row| row.blocked.is_some()),
        "with no Verilog-A view there are no settings to open"
    );
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let row = find(&app).expect("the palette finds Build settings");
    assert!(row.blocked.is_none(), "{:?}", row.blocked);
    row.entry.execute(&mut app).expect("the command runs");
    assert!(app.state.code.veriloga_build_settings.is_some());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn loaded_model_records_are_searchable_and_select_the_catalog_record() {
    let mut app = test_app();
    let mut library = crate::state::model_library::ModelLibrary::new("foundry_65lp");
    let mut model = crate::state::model_library::DeviceModel::new(
        "nmos_ulvt_precision",
        crate::state::model_library::ModelType::Nmos,
    );
    model.description = "Low-noise qualified input device".to_owned();
    library.add_model(model);
    app.state.model.model_library_manager.add_library(library);
    app.state.model.model_library_manager.filter_type =
        Some(crate::state::model_library::ModelType::Pmos);

    let mut rows = palette_rows(
        &app,
        "qualified foundry_65lp",
        PaletteScope::Models,
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    assert_eq!(rows.len(), 1);
    assert!(
        rows[0]
            .detail()
            .contains("Low-noise qualified input device")
    );
    let entry = rows.remove(0).entry;
    assert!(matches!(entry, PaletteEntry::Model(_)));
    entry.execute(&mut app).expect("open real model record");

    assert_eq!(
        app.state
            .model
            .model_library_manager
            .selected_library
            .as_deref(),
        Some("foundry_65lp")
    );
    assert_eq!(
        app.state.models.selected_model.as_deref(),
        Some("nmos_ulvt_precision")
    );
    assert_eq!(
        app.state.model.model_library_manager.filter_text,
        "nmos_ulvt_precision"
    );
    assert_eq!(app.state.model.model_library_manager.filter_type, None);
    assert_eq!(
        app.state.models.models_page,
        crate::state::ModelsPage::Models
    );
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Models
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn enter_executes_a_dynamic_row_without_polluting_command_recents() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = test_app();
    let mut library = crate::state::model_library::ModelLibrary::new("vendor_models");
    library.add_model(crate::state::model_library::DeviceModel::new(
        "opa189_a",
        crate::state::model_library::ModelType::Other,
    ));
    app.state.model.model_library_manager.add_library(library);
    app.state.workbench.command_palette.open();
    app.state.workbench.command_palette.query = "opa189_a".to_owned();

    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        render_command_palette(&mut app, ctx)
    });
    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Enter)]), |ctx| {
        render_command_palette(&mut app, ctx);
    });

    assert!(!app.state.workbench.command_palette.open);
    assert_eq!(
        app.state
            .model
            .model_library_manager
            .selected_library
            .as_deref(),
        Some("vendor_models")
    );
    assert_eq!(app.state.models.selected_model.as_deref(), Some("opa189_a"));
    assert!(app.state.workbench.command_palette.recent.is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn active_run_signals_are_searchable_and_open_the_visible_waveform() {
    let mut app = test_app();
    let stale_waveform =
        crate::state::RetainedTrace::new("V(stale)", vec![0.0, 1.0], vec![0.0, 0.1], "#777777");
    let mut stale_run = crate::state::SimulationRun::new(40);
    stale_run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "tran")
            .with_waveforms(vec![stale_waveform]),
    );

    let op_waveform = crate::state::RetainedTrace::new("I(VDD)", vec![0.0], vec![0.012], "#55aa99");
    let mut target_waveform = crate::state::RetainedTrace::new(
        "V(precision_out)",
        vec![0.0, 1.0],
        vec![0.0, 2.5],
        "#cc8844",
    );
    target_waveform.visible = false;
    let mut active_run = crate::state::SimulationRun::new(41);
    active_run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::DcOp, "op")
            .with_waveforms(vec![op_waveform]),
    );
    active_run.add_analysis(
        crate::state::AnalysisResult::new(2, crate::state::AnalysisType::Transient, "tran")
            .with_waveforms(vec![target_waveform]),
    );
    let active_dataset_id = active_run.dataset_id;

    app.state.model.simulation.retained.runs = vec![stale_run, active_run].into();
    assert!(app.state.model.simulation.select_run(1));
    assert!(
        palette_rows(
            &app,
            "V(stale)",
            PaletteScope::Signals,
            CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        )
        .is_empty(),
        "inactive run history must not leak into the active-signal index"
    );
    let mut rows = palette_rows(
        &app,
        "precision_out tran",
        PaletteScope::Signals,
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    assert_eq!(rows.len(), 1);
    assert!(rows[0].detail().contains("Run 41"));
    assert!(rows[0].detail().contains("hidden"));
    let entry = rows.remove(0).entry;
    assert!(matches!(entry, PaletteEntry::Signal(_)));
    assert_eq!(
        entry.command(),
        None,
        "resource rows do not enter command recents"
    );

    app.state.results.results.session.viewer = crate::workbench::ResultViewer::Bode;
    let hidden_strip = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
        active_dataset_id,
        &app.state.model.simulation.retained.runs[1].analyses[1],
    );
    let maximized_strip =
        crate::workbench::documents::result_document::AnalysisPresentationKey::new(
            active_dataset_id,
            &app.state.model.simulation.retained.runs[1].analyses[0],
        );
    app.state
        .results
        .results
        .session
        .hidden_strips
        .insert(hidden_strip);
    app.state.results.results.session.maximized_strip = Some(maximized_strip);
    let version_before = app.state.model.simulation.view.data_version;
    entry.execute(&mut app).expect("open real result signal");

    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(1));
    assert_eq!(app.state.model.simulation.view.active_analysis_idx, Some(1));
    assert_eq!(
        app.state.model.simulation.retained.runs[1].dataset_id,
        active_dataset_id
    );
    assert!(app.state.model.simulation.retained.runs[1].analyses[1].waveforms[0].visible);
    assert!(app.state.model.simulation.view.waveforms[0].visible);
    assert!(app.state.model.simulation.view.data_version > version_before);
    assert_eq!(
        app.state.results.results.session.viewer,
        crate::workbench::ResultViewer::Waves
    );
    assert!(
        !app.state
            .results
            .results
            .session
            .hidden_strips
            .contains(&hidden_strip)
    );
    assert_eq!(app.state.results.results.session.maximized_strip, None);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Results
    );
    assert!(app.state.workbench.command_palette.recent.is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn accesskit_exposes_modal_combobox_scopes_listbox_and_options() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = test_app();
    app.state.workbench.command_palette.open();
    let output = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        render_command_palette(&mut app, ctx)
    });
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog
            && node.label() == Some(DIALOG_LABEL)
            && node.description() == Some(DIALOG_DESCRIPTION)
            && node.is_modal()
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::ComboBox
            && node.label() == Some("Command search")
            && node.is_expanded() == Some(true)
            && !node.controls().is_empty()
            && node.active_descendant().is_some()
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::ListBox && node.label() == Some("Matching commands")
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::RadioGroup && node.label() == Some("Search scopes")
    }));
    assert_eq!(
        nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::RadioButton)
            .count(),
        PaletteScope::ALL.len()
    );
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::ListBoxOption && node.label().is_some()
    }));
    let expected_scope = PaletteLayout::resolve(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(1280.0, 800.0)),
        false,
        44.0,
    )
    .regions()
    .scope;
    assert_eq!(
        ctx.read_response(scope_group_id())
            .expect("scope semantic container")
            .rect,
        expected_scope
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn keyboard_navigation_wraps_and_enter_runs_only_available_commands() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = test_app();
    app.state.workbench.command_palette.open();
    let row_count = palette_rows(
        &app,
        "",
        PaletteScope::All,
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    )
    .len();
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        render_command_palette(&mut app, ctx)
    });

    let _ = ctx.run_ui(raw_input(vec![key_event(Key::ArrowDown)]), |ctx| {
        render_command_palette(&mut app, ctx);
    });
    assert_eq!(app.state.workbench.command_palette.selected, 1);

    app.state.workbench.command_palette.selected = 0;
    let _ = ctx.run_ui(raw_input(vec![key_event(Key::ArrowUp)]), |ctx| {
        render_command_palette(&mut app, ctx);
    });
    assert_eq!(app.state.workbench.command_palette.selected, row_count - 1);

    app.state.workbench.command_palette.query = "Stop active run".to_owned();
    app.state.workbench.command_palette.selected = 0;
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        render_command_palette(&mut app, ctx)
    });
    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Enter)]), |ctx| {
        render_command_palette(&mut app, ctx);
    });
    assert!(app.state.workbench.command_palette.open);
    assert!(app.state.workbench.command_palette.recent.is_empty());

    app.state.workbench.command_palette.query = "About RSpice".to_owned();
    app.state.workbench.command_palette.selected = 0;
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        render_command_palette(&mut app, ctx)
    });
    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Enter)]), |ctx| {
        render_command_palette(&mut app, ctx);
    });
    assert!(!app.state.workbench.command_palette.open);
    assert!(app.state.shell.about);
    assert_eq!(
        app.state.workbench.command_palette.recent.first(),
        Some(&Command::About.key())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn tab_moves_to_an_action_without_leaving_the_modal_layer() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = test_app();
    app.state.workbench.command_palette.open();
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        render_command_palette(&mut app, ctx)
    });
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Tab)]), |ctx| {
        render_command_palette(&mut app, ctx);
    });
    let focused = ctx.memory(|memory| memory.focused()).expect("modal focus");
    assert_ne!(focused, search_id());
    let response = ctx.read_response(focused).expect("focused action response");
    assert_eq!(
        response.layer_id,
        egui::LayerId::new(Order::Foreground, palette_id())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn modal_claims_and_restores_prior_focus_on_escape() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let underlying_id = Id::new("command-palette-test-underlying");
    let mut underlying = String::from("baseline");
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id))
                .request_focus();
        });
    });
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id));

    let mut app = test_app();
    app.state.workbench.command_palette.open();
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
        });
        render_command_palette(&mut app, ctx);
    });
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

    ctx.memory_mut(|memory| memory.request_focus(underlying_id));
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
        });
        render_command_palette(&mut app, ctx);
    });
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Escape)]), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
        });
        render_command_palette(&mut app, ctx);
    });
    assert!(!app.state.workbench.command_palette.open);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id));
    assert_eq!(underlying, "baseline");
}

/// One application frame in the native loop's order, and every
/// accessibility label it drew.
#[cfg(not(target_arch = "wasm32"))]
fn application_frame(ctx: &Context, app: &mut RSpiceApp, events: Vec<egui::Event>) -> Vec<String> {
    let output = ctx.run_ui(raw_input(events), |ui| {
        let ctx = ui.ctx().clone();
        app.prepare_frame(&ctx);
        app.render_frame_chrome(ui);
        app.render_frame_dialogs(&ctx);
    });
    output
        .platform_output
        .accesskit_update
        .map(|update| {
            update
                .nodes
                .iter()
                .filter_map(|(_, node)| node.label().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

/// The palette renders after the toasts, so a notice a command it runs
/// reports is drawn by the next frame, as it was when the notice was toasted
/// on the spot. The frame that ran the command leaves it queued, which is
/// what makes the shell ask for that next frame.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_notice_reported_from_the_palette_is_toasted_by_the_next_frame() {
    let missing = std::env::temp_dir().join(format!(
        "rspice-palette-rescan-missing-{}-{}",
        std::process::id(),
        crate::time_compat::unix_epoch().as_nanos()
    ));
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.model.pdk_config = crate::state::pdk_config::PdkConfig::default();
    app.state
        .model
        .pdk_config
        .add_library_path(missing.to_string_lossy().into_owned());
    app.state.workbench.command_palette.open();
    app.state.workbench.command_palette.query = "Rescan model libraries".to_owned();
    let title = "Model library rescan completed with warnings";
    // The toast's own control: the Console's copy of the line is drawn too.
    let dismiss = format!("Dismiss {title}");
    let drawn = |labels: &[String]| labels.contains(&dismiss);

    let _ = application_frame(&ctx, &mut app, Vec::new());
    let ran = application_frame(&ctx, &mut app, vec![key_event(Key::Enter)]);
    assert!(
        !app.state.workbench.command_palette.open,
        "Enter ran the command"
    );
    assert!(
        !drawn(&ran),
        "the toasts were drawn before the palette ran it"
    );
    assert!(
        !app.state.model.notices.is_empty(),
        "the frame ends with it queued"
    );

    let next = application_frame(&ctx, &mut app, Vec::new());
    assert!(drawn(&next), "the next frame draws it: {next:?}");
    assert!(app.state.model.notices.is_empty());
    let activity = app.state.workbench.toasts.activity();
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].title(), title);
}
