//! The shortcut resolver over the shell's command registry: multi-stroke
//! sequences and their timeouts on an injected clock, which candidate an
//! event dispatches by platform, enablement, focus and context precedence,
//! and the conventional chords of each workspace.

use std::time::Duration;

use egui::{Event, Key, Modifiers};

use crate::state::ViewType;
use crate::state::command_catalog::{CommandKey, CommandPlatform};
use crate::state::shortcuts::{
    ChordTimeoutPolicy, ContextPrecedencePolicy, ShortcutBindingSlot, ShortcutPreferences,
    ShortcutSequence, ShortcutStroke, SingleKeyCanvasPolicy,
};
use crate::workbench::app_state::session::shortcuts::ShortcutInputSnapshot;
use crate::workbench::app_state::session::shortcuts::resolver::*;
use crate::workbench::commands::vocabulary::{Command, CommandRegistry};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::state::Workspace;

fn primary() -> Modifiers {
    Modifiers {
        command: true,
        ctrl: true,
        ..Modifiers::NONE
    }
}

fn event(key: Key, modifiers: Modifiers, repeat: bool) -> Event {
    Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat,
        modifiers,
    }
}

fn snapshot(events: &[Event], non_canvas_focus: bool) -> ShortcutInputSnapshot {
    ShortcutInputSnapshot::from_events_for_test(events, non_canvas_focus)
}

fn design(canvas_focus: bool) -> ShortcutEnvironment {
    ShortcutEnvironment {
        workspace: Workspace::Design,
        active_view: ViewType::Schematic,
        canvas_focus,
        source_editor_focus: false,
    }
}

fn results() -> ShortcutEnvironment {
    ShortcutEnvironment {
        workspace: Workspace::Results,
        active_view: ViewType::Schematic,
        canvas_focus: false,
        source_editor_focus: false,
    }
}

fn models_symbol(canvas_focus: bool) -> ShortcutEnvironment {
    ShortcutEnvironment {
        workspace: Workspace::Models,
        active_view: ViewType::Symbol,
        canvas_focus,
        source_editor_focus: false,
    }
}

fn verification() -> ShortcutEnvironment {
    ShortcutEnvironment {
        workspace: Workspace::Verify,
        active_view: ViewType::Schematic,
        canvas_focus: false,
        source_editor_focus: false,
    }
}

/// A Verilog-A cell view in Design with its source editor focused.
fn focused_source_editor() -> ShortcutEnvironment {
    ShortcutEnvironment {
        workspace: Workspace::Design,
        active_view: ViewType::VerilogA,
        canvas_focus: false,
        source_editor_focus: true,
    }
}

fn bind(profile: &mut ShortcutPreferences, command: Command, strokes: Vec<ShortcutStroke>) {
    bind_key(profile, command.key(), strokes);
}

fn bind_key(profile: &mut ShortcutPreferences, command: CommandKey, strokes: Vec<ShortcutStroke>) {
    profile
        .set_binding(
            &CommandRegistry,
            command,
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(ShortcutSequence::new(strokes).unwrap()),
        )
        .unwrap();
}

#[test]
fn two_stroke_sequence_waits_and_dispatches_at_most_one_command() {
    let mut profile = ShortcutPreferences::default();
    bind(
        &mut profile,
        Command::CommandPalette,
        vec![
            ShortcutStroke::new(Key::K, true, false, false),
            ShortcutStroke::new(Key::P, true, false, false),
        ],
    );
    let mut state = ShortcutResolverState::default();
    let first = state.resolve(
        &snapshot(&[event(Key::K, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(first.command, None);
    assert_eq!(first.consume.len(), 1);
    let second = state.resolve(
        &snapshot(
            &[
                event(Key::P, primary(), false),
                event(Key::S, primary(), false),
            ],
            false,
        ),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_millis(100),
        |_| true,
    );
    assert_eq!(second.command, Some(Command::CommandPalette.key()));
    assert_eq!(second.consume.len(), 2);
    assert_eq!(second.deferred_start, 1);
    let deferred = state.resolve(
        &ShortcutInputSnapshot::empty_for_test(false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_millis(101),
        |_| true,
    );
    assert_eq!(deferred.command, Some(Command::Save.key()));
}

#[test]
fn exact_prefix_dispatches_on_injected_timeout_and_clock_never_rewinds() {
    let mut profile = ShortcutPreferences::default();
    bind_key(
        &mut profile,
        ids::PREFERENCES,
        vec![ShortcutStroke::new(Key::Q, true, false, false)],
    );
    bind(
        &mut profile,
        Command::ZoomFit,
        vec![
            ShortcutStroke::new(Key::Q, true, false, false),
            ShortcutStroke::new(Key::P, true, false, false),
        ],
    );
    let mut state = ShortcutResolverState::default();
    let first = state.resolve(
        &snapshot(&[event(Key::Q, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_secs(5),
        |_| true,
    );
    assert_eq!(first.command, None);
    let not_rewound = state.resolve(
        &ShortcutInputSnapshot::empty_for_test(false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_secs(1),
        |_| true,
    );
    assert_eq!(not_rewound.command, None);
    let expired = state.resolve(
        &ShortcutInputSnapshot::empty_for_test(false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_millis(6_500),
        |_| true,
    );
    assert_eq!(expired.command, Some(ids::PREFERENCES));
}

#[test]
fn no_timeout_waits_until_a_mismatch_then_dispatches_exact_prefix() {
    let mut profile = ShortcutPreferences::default();
    profile
        .policies_mut()
        .set_chord_timeout(ChordTimeoutPolicy::NoTimeout);
    bind_key(
        &mut profile,
        ids::PREFERENCES,
        vec![ShortcutStroke::new(Key::Q, true, false, false)],
    );
    bind(
        &mut profile,
        Command::ZoomFit,
        vec![
            ShortcutStroke::new(Key::Q, true, false, false),
            ShortcutStroke::new(Key::P, true, false, false),
        ],
    );
    let mut state = ShortcutResolverState::default();
    state.resolve(
        &snapshot(&[event(Key::Q, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::ZERO,
        |_| true,
    );
    let waiting = state.resolve(
        &ShortcutInputSnapshot::empty_for_test(false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_secs(60),
        |_| true,
    );
    assert_eq!(waiting.command, None);
    assert_eq!(waiting.repaint_after, None);
    let mismatch = state.resolve(
        &snapshot(&[event(Key::S, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_secs(61),
        |_| true,
    );
    assert_eq!(mismatch.command, Some(ids::PREFERENCES));
    assert_eq!(mismatch.consume.len(), 1);
    assert_eq!(mismatch.deferred_start, 0);
    let deferred = state.resolve(
        &ShortcutInputSnapshot::empty_for_test(false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_secs(62),
        |_| true,
    );
    assert_eq!(deferred.command, Some(Command::Save.key()));
    assert!(deferred.consume.is_empty());
}

#[test]
fn repeat_release_and_wrong_platform_do_not_resolve() {
    let release = Event::Key {
        key: Key::K,
        physical_key: Some(Key::K),
        pressed: false,
        repeat: false,
        modifiers: primary(),
    };
    let ignored_events = snapshot(&[event(Key::K, primary(), true), release], false);
    let mut state = ShortcutResolverState::default();
    assert_eq!(
        state
            .resolve(
                &ignored_events,
                &ShortcutPreferences::default(),
                &CommandRegistry,
                CommandPlatform::Browser,
                design(false),
                Duration::ZERO,
                |_| true,
            )
            .command,
        None
    );

    let mut desktop_only = ShortcutPreferences::default();
    bind(
        &mut desktop_only,
        Command::CommandPalette,
        vec![ShortcutStroke::new(Key::F12, false, false, false)],
    );
    let browser = state.resolve(
        &snapshot(&[event(Key::F12, Modifiers::NONE, false)], false),
        &desktop_only,
        &CommandRegistry,
        CommandPlatform::Browser,
        design(false),
        Duration::from_millis(1),
        |_| true,
    );
    assert_eq!(browser.command, None);
    assert!(browser.consume.is_empty());
    let desktop = state.resolve(
        &snapshot(&[event(Key::F12, Modifiers::NONE, false)], false),
        &desktop_only,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_millis(2),
        |_| true,
    );
    assert_eq!(desktop.command, Some(Command::CommandPalette.key()));
}

#[test]
fn disabled_higher_candidate_does_not_consume_lower_eligible_command() {
    let mut profile = ShortcutPreferences::default();
    bind(
        &mut profile,
        Command::ZoomFit,
        vec![ShortcutStroke::new(Key::K, true, false, false)],
    );
    bind(
        &mut profile,
        Command::CommandPalette,
        vec![ShortcutStroke::new(Key::K, true, false, false)],
    );
    profile
        .policies_mut()
        .set_context_precedence(ContextPrecedencePolicy::EditorModalWorkspaceGlobal);
    let mut state = ShortcutResolverState::default();
    let resolved = state.resolve(
        &snapshot(&[event(Key::K, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(true),
        Duration::ZERO,
        |command| command != Command::ZoomFit.key(),
    );
    assert_eq!(resolved.command, Some(Command::CommandPalette.key()));
    assert_eq!(resolved.consume.len(), 1);
}

#[test]
fn disabled_only_candidate_does_not_consume_the_key() {
    let mut profile = ShortcutPreferences::default();
    bind(
        &mut profile,
        Command::CommandPalette,
        vec![ShortcutStroke::new(Key::K, true, false, false)],
    );
    let mut state = ShortcutResolverState::default();
    let resolved = state.resolve(
        &snapshot(&[event(Key::K, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::ZERO,
        |_| false,
    );
    assert_eq!(resolved.command, None);
    assert!(resolved.consume.is_empty());
}

#[test]
fn invalid_whole_profile_executes_immutable_defaults_only() {
    let source = serde_json::json!({
        "policies": {
            "single-key-canvas": "disabled",
            "chord-timeout": "no-timeout",
            "context-precedence": "editor-modal-workspace-global"
        },
        "commands": {
            Command::Save.stable_id(): {"bindings": [{
                "slot": "primary",
                "platforms": ["desktop"],
                "sequence": [{"key": "DefinitelyNotAKey"}]
            }]},
            Command::CommandPalette.stable_id(): {"bindings": [{
                "slot": "primary",
                "platforms": ["desktop"],
                "sequence": [{"key": "F12"}]
            }]}
        }
    });
    let profile: ShortcutPreferences = serde_json::from_value(source).unwrap();
    assert!(!profile.is_effective_profile_valid(&CommandRegistry));
    let mut state = ShortcutResolverState::default();
    let fallback = state.resolve(
        &snapshot(&[event(Key::K, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(fallback.command, Some(Command::CommandPalette.key()));
    let rejected_override = state.resolve(
        &snapshot(&[event(Key::F12, Modifiers::NONE, false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_millis(1),
        |_| true,
    );
    assert_eq!(rejected_override.command, None);
    let rejected_policy = state.resolve(
        &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(true),
        Duration::from_millis(2),
        |_| true,
    );
    assert_eq!(rejected_policy.command, Some(Command::PlaceWire.key()));
}

#[test]
fn canvas_focus_policy_requires_exact_canvas_focus() {
    let profile = ShortcutPreferences::default();
    let mut state = ShortcutResolverState::default();
    let unfocused = state.resolve(
        &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(unfocused.command, None);
    let focused = state.resolve(
        &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(true),
        Duration::from_millis(1),
        |_| true,
    );
    assert_eq!(focused.command, Some(Command::PlaceWire.key()));

    let model_symbol = state.resolve(
        &snapshot(&[event(Key::L, Modifiers::NONE, false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        models_symbol(true),
        Duration::from_millis(2),
        |_| true,
    );
    assert_eq!(
        model_symbol.command,
        Some(Command::SymbolPolylineTool.key())
    );
}

#[test]
fn complete_symbol_tool_surface_resolves_through_the_central_registry() {
    let profile = ShortcutPreferences::default();
    let mut state = ShortcutResolverState::default();
    for (time, key, expected) in [
        (1, Key::P, Command::SymbolPinTool),
        (2, Key::L, Command::SymbolPolylineTool),
        (3, Key::C, Command::SymbolCircleTool),
        (4, Key::A, Command::SymbolArcTool),
        (5, Key::F, Command::ZoomFit),
    ] {
        let resolved = state.resolve(
            &snapshot(&[event(key, Modifiers::NONE, false)], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            models_symbol(true),
            Duration::from_millis(time),
            |_| true,
        );
        assert_eq!(resolved.command, Some(expected.key()), "key {key:?}");
    }

    let unbound_select = state.resolve(
        &snapshot(&[event(Key::S, Modifiers::NONE, false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        models_symbol(true),
        Duration::from_millis(6),
        |_| true,
    );
    assert_eq!(unbound_select.command, None);
}

#[test]
fn plain_s_resolves_stretch_only_in_the_engineering_canvas() {
    let profile = ShortcutPreferences::default();
    assert!(
        profile
            .effective_bindings(&CommandRegistry, Command::ArraySelection.key())
            .is_empty(),
        "Create array has no factory shortcut to route through the resolver"
    );
    let resolve = |environment| {
        ShortcutResolverState::default().resolve(
            &snapshot(&[event(Key::S, Modifiers::NONE, false)], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            environment,
            Duration::ZERO,
            |_| true,
        )
    };

    assert_eq!(
        resolve(design(true)).command,
        Some(Command::StretchSelection.key())
    );
    assert_eq!(resolve(models_symbol(true)).command, None);
}

#[test]
fn shift_p_resolves_schematic_place_pin_without_breaking_plain_p_contexts() {
    let profile = ShortcutPreferences::default();
    let shift = Modifiers {
        shift: true,
        ..Modifiers::NONE
    };

    let resolve = |key, modifiers, environment| {
        ShortcutResolverState::default().resolve(
            &snapshot(&[event(key, modifiers, false)], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            environment,
            Duration::ZERO,
            |_| true,
        )
    };

    assert_eq!(
        resolve(Key::P, shift, design(true)).command,
        Some(Command::PlacePin.key())
    );
    assert_eq!(
        resolve(Key::P, Modifiers::NONE, design(true)).command,
        Some(Command::PlaceProbe.key())
    );
    assert_eq!(
        resolve(Key::P, Modifiers::NONE, models_symbol(true)).command,
        Some(Command::SymbolPinTool.key())
    );
    assert_eq!(resolve(Key::P, shift, models_symbol(true)).command, None);
}

#[test]
fn shift_t_resolves_schematic_text_without_breaking_plain_bus_tap() {
    let profile = ShortcutPreferences::default();
    let shift = Modifiers {
        shift: true,
        ..Modifiers::NONE
    };
    let resolve = |modifiers, environment| {
        ShortcutResolverState::default().resolve(
            &snapshot(&[event(Key::T, modifiers, false)], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            environment,
            Duration::ZERO,
            |_| true,
        )
    };

    assert_eq!(
        resolve(shift, design(true)).command,
        Some(Command::PlaceText.key())
    );
    assert_eq!(
        resolve(Modifiers::NONE, design(true)).command,
        Some(Command::PlaceBusTap.key())
    );
    assert_eq!(resolve(shift, models_symbol(true)).command, None);
}

#[test]
fn result_context_beats_global_only_when_enabled() {
    let mut profile = ShortcutPreferences::default();
    bind_key(
        &mut profile,
        ResultsCommand::WaveformCalculator.key(),
        vec![ShortcutStroke::new(Key::K, true, false, false)],
    );
    bind(
        &mut profile,
        Command::CommandPalette,
        vec![ShortcutStroke::new(Key::K, true, false, false)],
    );
    let mut state = ShortcutResolverState::default();
    let result = state.resolve(
        &snapshot(&[event(Key::K, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        results(),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(
        result.command,
        Some(ResultsCommand::WaveformCalculator.key())
    );
}

/// The magnification and copy chords reach the Results workspace.
///
/// Their handlers have carried a Results branch all along —
/// `Command::ZoomFit` queues a view gesture and `Command::Copy` writes the
/// cursor readout — but the resolver filtered them out before dispatch
/// because their context was active on the design canvases only. Pressing
/// the chord the View menu prints beside the row did nothing.
#[test]
fn magnification_and_copy_chords_dispatch_on_a_result_sheet() {
    let profile = ShortcutPreferences::default();
    for command in [
        Command::ZoomIn,
        Command::ZoomOut,
        Command::ZoomFit,
        Command::Copy,
    ] {
        let sequence = profile
            .effective_bindings(&CommandRegistry, command.key())
            .into_iter()
            .find(|binding| binding.supports(CommandPlatform::Desktop))
            .map(|binding| binding.sequence().clone())
            .unwrap_or_else(|| panic!("{command:?} has no desktop binding to press"));
        let strokes = sequence.strokes();
        assert_eq!(
            strokes.len(),
            1,
            "{command:?} is expected to be a single-stroke chord"
        );
        let stroke = strokes[0];
        let modifiers = Modifiers {
            command: stroke.primary(),
            ctrl: stroke.primary(),
            alt: stroke.alt(),
            shift: stroke.shift(),
            ..Modifiers::NONE
        };
        let mut state = ShortcutResolverState::default();
        let resolution = state.resolve(
            &snapshot(&[event(stroke.key(), modifiers, false)], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            results(),
            Duration::ZERO,
            |candidate| candidate == command.key(),
        );
        assert_eq!(
            resolution.command,
            Some(command.key()),
            "{command:?} must dispatch on the Results workspace"
        );
    }
}

/// Widening the edit context does not hand Results the schematic-editing
/// family: those commands are `is_enabled`-false on a result sheet, and
/// the resolver consults availability before precedence.
#[test]
fn a_result_sheet_does_not_dispatch_schematic_editing_chords() {
    let profile = ShortcutPreferences::default();
    for command in [Command::Cut, Command::Paste, Command::Delete] {
        let sequence = profile
            .effective_bindings(&CommandRegistry, command.key())
            .into_iter()
            .find(|binding| binding.supports(CommandPlatform::Desktop))
            .map(|binding| binding.sequence().clone())
            .unwrap_or_else(|| panic!("{command:?} has no desktop binding to press"));
        let stroke = sequence.strokes()[0];
        let modifiers = Modifiers {
            command: stroke.primary(),
            ctrl: stroke.primary(),
            alt: stroke.alt(),
            shift: stroke.shift(),
            ..Modifiers::NONE
        };
        let mut state = ShortcutResolverState::default();
        let resolution = state.resolve(
            &snapshot(&[event(stroke.key(), modifiers, false)], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            results(),
            Duration::ZERO,
            |_| false,
        );
        assert_eq!(
            resolution.command, None,
            "{command:?} must not dispatch on the Results workspace"
        );
    }
}

#[test]
fn three_second_policy_uses_the_injected_monotonic_deadline() {
    let mut profile = ShortcutPreferences::default();
    profile
        .policies_mut()
        .set_chord_timeout(ChordTimeoutPolicy::ThreeSeconds);
    bind_key(
        &mut profile,
        ids::PREFERENCES,
        vec![ShortcutStroke::new(Key::Q, true, false, false)],
    );
    bind(
        &mut profile,
        Command::ZoomFit,
        vec![
            ShortcutStroke::new(Key::Q, true, false, false),
            ShortcutStroke::new(Key::P, true, false, false),
        ],
    );
    let mut state = ShortcutResolverState::default();
    state.resolve(
        &snapshot(&[event(Key::Q, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(
        state
            .resolve(
                &ShortcutInputSnapshot::empty_for_test(false),
                &profile,
                &CommandRegistry,
                CommandPlatform::Desktop,
                design(false),
                Duration::from_millis(2_999),
                |_| true,
            )
            .command,
        None
    );
    assert_eq!(
        state
            .resolve(
                &ShortcutInputSnapshot::empty_for_test(false),
                &profile,
                &CommandRegistry,
                CommandPlatform::Desktop,
                design(false),
                Duration::from_secs(3),
                |_| true,
            )
            .command,
        Some(ids::PREFERENCES)
    );
}

#[test]
fn four_stroke_sequence_is_supported_without_dispatching_twice() {
    let mut profile = ShortcutPreferences::default();
    bind_key(
        &mut profile,
        ids::PREFERENCES,
        vec![
            ShortcutStroke::new(Key::Q, true, false, false),
            ShortcutStroke::new(Key::W, true, false, false),
            ShortcutStroke::new(Key::E, true, false, false),
            ShortcutStroke::new(Key::R, true, false, false),
        ],
    );
    let events = [
        event(Key::Q, primary(), false),
        event(Key::W, primary(), false),
        event(Key::E, primary(), false),
        event(Key::R, primary(), false),
    ];
    let mut state = ShortcutResolverState::default();
    let resolution = state.resolve(
        &snapshot(&events, false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(resolution.command, Some(ids::PREFERENCES));
    assert_eq!(resolution.consume.len(), 4);
}

#[test]
fn single_key_canvas_require_alt_and_disabled_use_shared_projection() {
    let mut require_alt = ShortcutPreferences::default();
    require_alt
        .policies_mut()
        .set_single_key_canvas(SingleKeyCanvasPolicy::RequireAlt);
    let mut state = ShortcutResolverState::default();
    let plain = state.resolve(
        &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
        &require_alt,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(plain.command, None);
    let alt = state.resolve(
        &snapshot(
            &[event(
                Key::W,
                Modifiers {
                    alt: true,
                    ..Modifiers::NONE
                },
                false,
            )],
            false,
        ),
        &require_alt,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_millis(1),
        |_| true,
    );
    assert_eq!(alt.command, Some(Command::PlaceWire.key()));

    let mut disabled = ShortcutPreferences::default();
    disabled
        .policies_mut()
        .set_single_key_canvas(SingleKeyCanvasPolicy::Disabled);
    let disabled_result = state.resolve(
        &snapshot(
            &[event(
                Key::W,
                Modifiers {
                    alt: true,
                    ..Modifiers::NONE
                },
                false,
            )],
            false,
        ),
        &disabled,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(true),
        Duration::from_millis(2),
        |_| true,
    );
    assert_eq!(disabled_result.command, None);
}

#[test]
fn application_chrome_and_editor_precedence_policy_changes_real_selection() {
    let mut chrome_first = ShortcutPreferences::default();
    bind(
        &mut chrome_first,
        Command::Cancel,
        vec![ShortcutStroke::new(Key::Q, true, false, false)],
    );
    bind(
        &mut chrome_first,
        Command::ZoomFit,
        vec![ShortcutStroke::new(Key::Q, true, false, false)],
    );
    let mut state = ShortcutResolverState::default();
    let chrome = state.resolve(
        &snapshot(&[event(Key::Q, primary(), false)], false),
        &chrome_first,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(true),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(chrome.command, Some(Command::Cancel.key()));

    let mut editor_first = chrome_first;
    editor_first
        .policies_mut()
        .set_context_precedence(ContextPrecedencePolicy::EditorModalWorkspaceGlobal);
    let editor = state.resolve(
        &snapshot(&[event(Key::Q, primary(), false)], false),
        &editor_first,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(true),
        Duration::from_millis(1),
        |_| true,
    );
    assert_eq!(editor.command, Some(Command::ZoomFit.key()));
}

#[test]
fn non_canvas_focus_suppresses_editor_but_not_global_context() {
    let profile = ShortcutPreferences::default();
    let mut state = ShortcutResolverState::default();
    let editor = state.resolve(
        &snapshot(&[event(Key::W, Modifiers::NONE, false)], true),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(true),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(editor.command, None);
    let global = state.resolve(
        &snapshot(&[event(Key::K, primary(), false)], true),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(false),
        Duration::from_millis(1),
        |_| true,
    );
    assert_eq!(global.command, Some(Command::CommandPalette.key()));
}

#[test]
fn next_marker_primary_and_alternate_resolve_in_verify_then_design() {
    let profile = ShortcutPreferences::default();
    let mut state = ShortcutResolverState::default();
    for (time, key, environment) in [
        (0, Key::CloseBracket, verification()),
        (1, Key::F8, design(false)),
    ] {
        let resolved = state.resolve(
            &snapshot(&[event(key, Modifiers::NONE, false)], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            environment,
            Duration::from_millis(time),
            |command| command == Command::NextViolation.key(),
        );
        assert_eq!(resolved.command, Some(Command::NextViolation.key()));
    }
}

/// Next violation had `]` and F8 while Previous had no key at all, and
/// Redo answered only Ctrl+Shift+Z (audit E-16). Previous takes the
/// mirror keys, `[` and Shift+F8, and Redo also answers Ctrl+Y.
#[test]
fn previous_marker_and_redo_answer_their_conventional_keys() {
    let profile = ShortcutPreferences::default();
    let mut state = ShortcutResolverState::default();
    let shift = Modifiers {
        shift: true,
        ..Modifiers::NONE
    };
    for (time, key, modifiers, environment, command) in [
        (
            0,
            Key::OpenBracket,
            Modifiers::NONE,
            verification(),
            Command::PreviousViolation,
        ),
        (1, Key::F8, shift, design(false), Command::PreviousViolation),
        (2, Key::Y, primary(), design(true), Command::Redo),
    ] {
        let resolved = state.resolve(
            &snapshot(&[event(key, modifiers, false)], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            environment,
            Duration::from_millis(time),
            |candidate| candidate == command.key(),
        );
        assert_eq!(
            resolved.command,
            Some(command.key()),
            "{key:?} {modifiers:?}"
        );
    }
}

/// Ctrl+D belongs to whichever surface holds focus: the focused source
/// editor duplicates its line, the focused canvas its selection.
#[test]
fn ctrl_d_duplicates_the_line_in_a_focused_source_editor_and_the_selection_on_a_canvas() {
    let profile = ShortcutPreferences::default();
    let mut state = ShortcutResolverState::default();
    let in_editor = state.resolve(
        &snapshot(&[event(Key::D, primary(), false)], true),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        focused_source_editor(),
        Duration::ZERO,
        |_| true,
    );
    assert_eq!(in_editor.command, Some(Command::DuplicateLine.key()));
    let on_canvas = state.resolve(
        &snapshot(&[event(Key::D, primary(), false)], false),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        design(true),
        Duration::from_millis(1),
        |_| true,
    );
    assert_eq!(on_canvas.command, Some(Command::Duplicate.key()));
    let unfocused = state.resolve(
        &snapshot(&[event(Key::Slash, primary(), false)], true),
        &profile,
        &CommandRegistry,
        CommandPlatform::Desktop,
        ShortcutEnvironment {
            source_editor_focus: false,
            ..focused_source_editor()
        },
        Duration::from_millis(2),
        |_| true,
    );
    assert_eq!(
        unfocused.command, None,
        "a line key does nothing while another text field holds focus"
    );
}
