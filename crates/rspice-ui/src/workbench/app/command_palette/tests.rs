//! Command palette search, navigation, and input ownership regressions.

use super::*;

#[test]
fn task_focused_menu_can_open_the_palette_in_an_exact_scope() {
    let mut app = RSpiceApp::test_instance();
    app.state.dialogs.command_palette.open_in_scope("Design");

    assert!(app.state.dialogs.command_palette.open);
    assert_eq!(
        app.state.dialogs.command_palette.initial_scope.as_deref(),
        Some("Design")
    );
    assert_eq!(
        PaletteScope::from_label("design"),
        Some(PaletteScope::Design)
    );
    assert_eq!(PaletteScope::from_label("unknown"), None);
}
use crate::workbench::commands::vocabulary::COMMAND_REGISTRY;

#[test]
fn desktop_and_phone_layouts_match_the_mockup_contract() {
    let desktop = PaletteLayout::resolve(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0)),
        false,
    );
    assert_eq!(desktop.surface.width(), 650.0);
    assert_eq!(desktop.surface.height(), 560.0);
    assert_eq!(desktop.surface.left(), 395.0);
    assert_eq!(desktop.surface.top(), 72.0);
    assert_eq!(desktop.regions().search.height(), 48.0);
    assert_eq!(desktop.regions().scope.height(), 37.0);
    assert_eq!(desktop.regions().footer.height(), 31.0);
    assert_eq!(RESULT_ROW_HEIGHT, 40.0);
    assert_eq!(RESULT_ICON_COLUMN, 28.0);
    assert_eq!(RESULT_ICON_SIDE, 26.0);

    let phone = PaletteLayout::resolve(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(390.0, 844.0)),
        true,
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
    );
    assert!(!touch_desktop.narrow);
    assert_eq!(touch_desktop.scope_height, 59.0);
}

#[test]
fn palette_is_a_typed_registry_projection() {
    let commands = palette_commands().collect::<Vec<_>>();
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
    let commands = palette_commands().collect::<Vec<_>>();
    assert!(
        commands
            .iter()
            .copied()
            .all(|command| { PaletteScope::for_command(command) != PaletteScope::All })
    );
    let assigned = PaletteScope::ALL
        .into_iter()
        .skip(1)
        .map(|scope| {
            commands
                .iter()
                .copied()
                .filter(|command| scope.contains(*command))
                .count()
        })
        .sum::<usize>();
    assert_eq!(assigned, commands.len());
    assert!(
        commands
            .iter()
            .copied()
            .all(|command| PaletteScope::for_command(command) != PaletteScope::Signals),
        "signal rows must represent real result waveforms, not generic commands"
    );
}

#[test]
fn command_scopes_and_metadata_search_only_return_registered_commands() {
    let design = palette_commands()
        .filter(|command| PaletteScope::Design.contains(*command))
        .filter(|command| match_entry(&PaletteEntry::Command(*command), "wire").is_some())
        .collect::<Vec<_>>();
    assert_eq!(design, vec![Command::PlaceWire]);

    assert_eq!(
        PaletteScope::for_command(Command::PlaceProbe),
        PaletteScope::Design
    );
    assert_eq!(
        PaletteScope::for_command(Command::PlaceBus),
        PaletteScope::Design
    );
    assert_eq!(
        PaletteScope::for_command(Command::PlaceBusTap),
        PaletteScope::Design
    );
    assert_eq!(
        PaletteScope::for_command(Command::PlaceJunction),
        PaletteScope::Design
    );
    assert_eq!(
        PaletteScope::for_command(Command::ExportWaveformsCsv),
        PaletteScope::Results
    );

    let help = palette_commands()
        .filter(|command| PaletteScope::Help.contains(*command))
        .collect::<Vec<_>>();
    assert!(help.contains(&Command::KeyboardShortcuts));
    assert!(help.contains(&Command::About));
}

#[test]
fn bus_commands_use_the_mockup_palette_glyphs() {
    assert_eq!(
        row_icon(&PaletteEntry::Command(Command::PlaceBus)),
        Icon::Bus
    );
    assert_eq!(
        row_icon(&PaletteEntry::Command(Command::PlaceBusTap)),
        Icon::Bus
    );
}

#[test]
fn stretch_selection_uses_the_exact_pointer_glyph() {
    assert_eq!(
        row_icon(&PaletteEntry::Command(Command::StretchSelection)),
        Icon::Select
    );
}

#[test]
fn create_array_uses_the_exact_grid_glyph() {
    assert_eq!(
        row_icon(&PaletteEntry::Command(Command::ArraySelection)),
        Icon::Grid
    );
}

#[test]
fn replace_instance_uses_the_exact_refresh_glyph() {
    assert_eq!(
        row_icon(&PaletteEntry::Command(Command::ReplaceInstance)),
        Icon::Refresh
    );
}

#[test]
fn fuzzy_matching_marks_display_characters_and_searches_registry_metadata() {
    assert_eq!(
        match_spans("Open generated netlist", "ogn"),
        Some((4, vec![0, 5, 7]))
    );
    let command = PaletteEntry::Command(Command::GenerateNetlist);
    let (_, marks) = match_entry(&command, "generated netlist").expect("label terms match");
    assert!(!marks.is_empty());
    assert!(match_entry(&command, "simulate").is_some());
    assert!(match_entry(&command, "nonexistent-token").is_none());
    assert_eq!(match_spans("Draw symbol circle", "wire"), None);
}

#[test]
fn palette_names_are_canonical_labels_without_ellipsis() {
    assert_eq!(command_name(Command::OpenProject), "Open project");
    assert_eq!(command_name(Command::SaveAll), "Save all");
    assert_eq!(
        command_name(Command::GenerateNetlist),
        "Open generated netlist"
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn test_app() -> RSpiceApp {
    let state = crate::workbench::app_state::AppState::default();
    let automation_runtime_project_id = state.workspace.project.id();
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
        cloud_account: crate::services::cloud_account::CloudAccountService::unconfigured(),
        model_hub: crate::services::model_hub::ModelHubService::unavailable(
            "This test instance runs without a model-pack store.",
        ),
        live_session: crate::workbench::live_session::LiveSessionEngine::default(),
        file_workflow_io: Box::new(
            crate::workbench::workflows::file_workflow::NativeFileWorkflowIo,
        ),
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
        app.render_command_palette(ctx);
    });
    assert!(app.state.dialogs.command_palette.open);
    assert_eq!(app.state.dialogs.command_palette.query, query);

    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        app.handle_shortcuts(ctx);
        app.render_command_palette(ctx);
    });
    assert_eq!(app.state.dialogs.command_palette.query, query);
    assert!(!app.state.dialogs.bus_tap.open);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn empty_all_scope_uses_mockup_suggestions_and_explicit_scopes_remain_complete() {
    let mut app = test_app();
    app.state.dialogs.command_palette.recent = vec![Command::PlaceProbe, Command::Save];
    let all = app.palette_rows(
        "",
        PaletteScope::All,
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    assert_eq!(all[0].entry.command(), Some(Command::PlaceProbe));
    assert_eq!(all[1].entry.command(), Some(Command::Save));
    assert_eq!(all.len(), SUGGESTION_LIMIT);
    assert_eq!(
        all.iter()
            .filter(|row| row.entry.command() == Some(Command::Save))
            .count(),
        1
    );
    assert!(all.iter().all(|row| row.section_label() != "Recent"));

    let signals = app.palette_rows(
        "",
        PaletteScope::Signals,
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    assert!(
        signals.is_empty(),
        "no result data means no invented signal rows"
    );
    assert_eq!(MAX_RECENT, 8);
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
    app.state.library_manager.add_library(library);

    let mut rows = app.palette_rows(
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
        app.state.workspace.active_view,
        crate::state::CellViewRef::new("precision_analog", "gain_stage", "schematic")
    );
    assert_eq!(
        app.state.library_manager.selected_lcv_path().as_deref(),
        Some("precision_analog/gain_stage/schematic")
    );
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Design
    );
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
    app.state.model_library_manager.add_library(library);
    app.state.model_library_manager.filter_type =
        Some(crate::state::model_library::ModelType::Pmos);

    let mut rows = app.palette_rows(
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
        app.state.model_library_manager.selected_library.as_deref(),
        Some("foundry_65lp")
    );
    assert_eq!(
        app.state.workbench.selected_model.as_deref(),
        Some("nmos_ulvt_precision")
    );
    assert_eq!(
        app.state.model_library_manager.filter_text,
        "nmos_ulvt_precision"
    );
    assert_eq!(app.state.model_library_manager.filter_type, None);
    assert_eq!(
        app.state.workbench.models_page,
        crate::workbench::state::ModelsPage::Models
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
    app.state.model_library_manager.add_library(library);
    app.state.dialogs.command_palette.open();
    app.state.dialogs.command_palette.query = "opa189_a".to_owned();

    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Enter)]), |ctx| {
        app.render_command_palette(ctx);
    });

    assert!(!app.state.dialogs.command_palette.open);
    assert_eq!(
        app.state.model_library_manager.selected_library.as_deref(),
        Some("vendor_models")
    );
    assert_eq!(
        app.state.workbench.selected_model.as_deref(),
        Some("opa189_a")
    );
    assert!(app.state.dialogs.command_palette.recent.is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn active_run_signals_are_searchable_and_open_the_visible_waveform() {
    let mut app = test_app();
    let stale_waveform =
        crate::state::WaveformData::new("V(stale)", vec![0.0, 1.0], vec![0.0, 0.1], "#777777");
    let mut stale_run = crate::state::SimulationRun::new(40);
    stale_run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "tran")
            .with_waveforms(vec![stale_waveform]),
    );

    let op_waveform = crate::state::WaveformData::new("I(VDD)", vec![0.0], vec![0.012], "#55aa99");
    let mut target_waveform = crate::state::WaveformData::new(
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

    app.state.simulation.runs = vec![stale_run, active_run].into();
    assert!(app.state.simulation.select_run(1));
    assert!(
        app.palette_rows(
            "V(stale)",
            PaletteScope::Signals,
            CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        )
        .is_empty(),
        "inactive run history must not leak into the active-signal index"
    );
    let mut rows = app.palette_rows(
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

    app.state.ui.results.viewer = crate::workbench::ResultViewer::Bode;
    let hidden_strip = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
        active_dataset_id,
        &app.state.simulation.runs[1].analyses[1],
    );
    let maximized_strip =
        crate::workbench::documents::result_document::AnalysisPresentationKey::new(
            active_dataset_id,
            &app.state.simulation.runs[1].analyses[0],
        );
    app.state.ui.results.hidden_strips.insert(hidden_strip);
    app.state.ui.results.maximized_strip = Some(maximized_strip);
    let version_before = app.state.simulation.data_version;
    entry.execute(&mut app).expect("open real result signal");

    assert_eq!(app.state.simulation.active_run_idx, Some(1));
    assert_eq!(app.state.simulation.active_analysis_idx, Some(1));
    assert_eq!(app.state.simulation.runs[1].dataset_id, active_dataset_id);
    assert!(app.state.simulation.runs[1].analyses[1].waveforms[0].visible);
    assert!(app.state.simulation.waveforms[0].visible);
    assert!(app.state.simulation.data_version > version_before);
    assert_eq!(
        app.state.ui.results.viewer,
        crate::workbench::ResultViewer::Waves
    );
    assert!(!app.state.ui.results.hidden_strips.contains(&hidden_strip));
    assert_eq!(app.state.ui.results.maximized_strip, None);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Results
    );
    assert!(app.state.dialogs.command_palette.recent.is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn accesskit_exposes_modal_combobox_scopes_listbox_and_options() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = test_app();
    app.state.dialogs.command_palette.open();
    let output = ctx.run_ui(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
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
    app.state.dialogs.command_palette.open();
    let row_count = app
        .palette_rows(
            "",
            PaletteScope::All,
            CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        )
        .len();
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));

    let _ = ctx.run_ui(raw_input(vec![key_event(Key::ArrowDown)]), |ctx| {
        app.render_command_palette(ctx);
    });
    assert_eq!(app.state.dialogs.command_palette.selected, 1);

    app.state.dialogs.command_palette.selected = 0;
    let _ = ctx.run_ui(raw_input(vec![key_event(Key::ArrowUp)]), |ctx| {
        app.render_command_palette(ctx);
    });
    assert_eq!(app.state.dialogs.command_palette.selected, row_count - 1);

    app.state.dialogs.command_palette.query = "Stop active run".to_owned();
    app.state.dialogs.command_palette.selected = 0;
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Enter)]), |ctx| {
        app.render_command_palette(ctx);
    });
    assert!(app.state.dialogs.command_palette.open);
    assert!(app.state.dialogs.command_palette.recent.is_empty());

    app.state.dialogs.command_palette.query = "About RSpice".to_owned();
    app.state.dialogs.command_palette.selected = 0;
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Enter)]), |ctx| {
        app.render_command_palette(ctx);
    });
    assert!(!app.state.dialogs.command_palette.open);
    assert!(app.state.dialogs.about);
    assert_eq!(
        app.state.dialogs.command_palette.recent.first(),
        Some(&Command::About)
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn tab_moves_to_an_action_without_leaving_the_modal_layer() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = test_app();
    app.state.dialogs.command_palette.open();
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Tab)]), |ctx| {
        app.render_command_palette(ctx);
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
    app.state.dialogs.command_palette.open();
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
        });
        app.render_command_palette(ctx);
    });
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

    ctx.memory_mut(|memory| memory.request_focus(underlying_id));
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
        });
        app.render_command_palette(ctx);
    });
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

    let _ = ctx.run_ui(raw_input(vec![key_event(Key::Escape)]), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
        });
        app.render_command_palette(ctx);
    });
    assert!(!app.state.dialogs.command_palette.open);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id));
    assert_eq!(underlying, "baseline");
}
