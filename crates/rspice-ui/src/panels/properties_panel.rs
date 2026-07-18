//! Property dialog host.
//!
//! The floating tabbed property dialog (schema-driven editing via the
//! `PropertyRegistry`). Inline inspection lives in the workbench inspector
//! (`crate::workbench::panels::schematic`).

use crate::common::app::AppState;
use crate::properties::{TabbedDialogResult, render_tabbed_property_dialog};

/// Render the floating tabbed property dialog window
/// Call this from the main app update loop
pub fn render_property_dialog(ctx: &egui::Context, state: &mut AppState) -> TabbedDialogResult {
    if state.tabbed_property_dialog.open {
        state.tabbed_property_dialog.session_error = component_property_session_error(state);
    }
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let number_locale = state.ui.number_locale;
    let commit_policy = state.schematic.document_policy.property_commit;
    let result = render_tabbed_property_dialog(
        ctx,
        &mut state.tabbed_property_dialog,
        &state.property_registry,
        &state.model_library_manager,
        quantity_policy,
        number_locale,
        commit_policy,
    );

    // Handle dialog result - apply changes back to component
    if matches!(result, TabbedDialogResult::Applied)
        && let Some(comp_id) = state.tabbed_property_dialog.component_id
    {
        if let Some(error) = component_property_session_error(state) {
            state.tabbed_property_dialog.open = true;
            state.tabbed_property_dialog.session_error = Some(error);
            return TabbedDialogResult::None;
        }
        let committed = state.tabbed_property_dialog.take_prepared_commit();
        let committed_names: Vec<String> = committed.keys().cloned().collect();
        // The bridge serializes the full component property map. Merge the
        // validated delta onto the last committed baseline so a rejected
        // partial field cannot erase an unrelated existing parameter.
        let mut values = state.tabbed_property_dialog.original_values.clone();
        values.extend(committed);
        let Some(component) = state.schematic.components.iter().find(|c| c.id == comp_id) else {
            state.tabbed_property_dialog.open = true;
            state.tabbed_property_dialog.session_error = Some(
                "The selected component no longer exists. Close and reopen Object properties."
                    .to_owned(),
            );
            return TabbedDialogResult::None;
        };
        let mut candidate = component.clone();
        crate::properties::property_bridge::apply_properties_to_component(
            &mut candidate,
            &values,
            &state.property_registry,
        );
        if &candidate != component {
            let before = crate::state::SchematicSnapshot::capture(&state.schematic);
            if let Some(component) = state
                .schematic
                .components
                .iter_mut()
                .find(|component| component.id == comp_id)
            {
                *component = candidate;
            }
            state.schematic.is_dirty = true;
            state.schematic.bump_topology_version();
            state.schematic.commit_undo_from(before, "edit properties");
        }

        // The mockup primary closes after the exact transaction completes.
        if state.tabbed_property_dialog.open {
            state.tabbed_property_dialog.component_baseline = state
                .schematic
                .components
                .iter()
                .find(|component| component.id == comp_id)
                .cloned();
            state
                .tabbed_property_dialog
                .mark_fields_applied(committed_names);
        } else {
            state.tabbed_property_dialog.clear_after_apply();
        }
    }

    result
}

fn component_property_session_error(state: &AppState) -> Option<String> {
    let dialog = &state.tabbed_property_dialog;
    if state.schematic.read_only || state.active_view_read_only() {
        return Some("The active schematic is read-only; no properties can be applied.".to_owned());
    }
    if dialog.design_execution_epoch != state.design_execution_epoch {
        return Some(
            "The design document changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.active_schematic_epoch != state.active_schematic_epoch {
        return Some(
            "The active schematic buffer changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.view_path != state.workspace.active_view.display_path() {
        return Some(
            "The active cell/view changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    let Some(baseline) = dialog.component_baseline.as_ref() else {
        return Some(
            "The selected component baseline is unavailable. Close and reopen Object properties."
                .to_owned(),
        );
    };
    let Some(current) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == baseline.id)
    else {
        return Some(
            "The selected component no longer exists. Close and reopen Object properties."
                .to_owned(),
        );
    };
    (current != baseline).then(|| {
        "The selected component changed while properties were open. Close and reopen the current object."
            .to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::app::open_property_editor;
    use crate::state::{
        CellViewRef, Component, ComponentType, Point, PropertyCommitPolicy, PropertyValue,
    };

    fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 850.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn key_event(key: egui::Key) -> egui::Event {
        key_event_with_modifiers(key, egui::Modifiers::NONE)
    }

    fn key_event_with_modifiers(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    fn state_with_resistor() -> AppState {
        let mut state = AppState::default();
        state.schematic.components.clear();
        let mut component = Component::new(44, ComponentType::Resistor, Point::new(4, 8));
        component.name = "R1".to_owned();
        component.value = "1k".to_owned();
        state.schematic.components.push(component);
        state.schematic.clear_undo_history();
        state
    }

    #[test]
    fn session_guard_rejects_read_only_stale_view_epoch_and_object() {
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        assert!(component_property_session_error(&state).is_none());

        state.schematic.read_only = true;
        assert!(component_property_session_error(&state).is_some());
        state.schematic.read_only = false;

        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        assert!(component_property_session_error(&state).is_some());
        state.design_execution_epoch = state.tabbed_property_dialog.design_execution_epoch;

        state.schematic.components[0].name = "R2".to_owned();
        assert!(component_property_session_error(&state).is_some());
    }

    #[test]
    fn rendered_component_primary_commits_name_with_one_real_undo_record() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        state
            .tabbed_property_dialog
            .set_value("name", PropertyValue::String("R99".to_owned()));

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });

        assert!(!state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R99");
        assert!(state.schematic.can_undo());
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components[0].name, "R1");
    }

    #[test]
    fn component_dirty_escape_requires_a_second_explicit_discard() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        state
            .tabbed_property_dialog
            .set_value("name", PropertyValue::String("R99".to_owned()));

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        assert!(state.tabbed_property_dialog.open);
        assert!(state.tabbed_property_dialog.discard_confirm);

        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        assert!(!state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R1");
    }

    #[test]
    fn returning_to_the_same_view_cannot_reauthorize_an_old_dialog() {
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        let captured_epoch = state.tabbed_property_dialog.active_schematic_epoch;
        let original_view = state.workspace.active_view.clone();

        state.open_workspace_view(CellViewRef::new("work", "detour", "schematic"));
        state.open_workspace_view(original_view.clone());

        assert_eq!(state.workspace.active_view, original_view);
        assert_ne!(state.active_schematic_epoch, captured_epoch);
        assert!(component_property_session_error(&state).is_some());
    }

    #[test]
    fn escape_closes_nested_model_browser_before_dirty_parent_dialog() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        state
            .tabbed_property_dialog
            .set_value("name", PropertyValue::String("R99".to_owned()));
        state.tabbed_property_dialog.model_browser.open = true;

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });

        assert!(!state.tabbed_property_dialog.model_browser.open);
        assert!(state.tabbed_property_dialog.open);
        assert!(!state.tabbed_property_dialog.discard_confirm);
        assert_eq!(state.schematic.components[0].name, "R1");
    }

    #[test]
    fn rendered_numeric_editor_retains_invalid_source_across_repaints() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        state.tabbed_property_dialog.active_tab = "Temperature".to_owned();

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run(
            dialog_input(vec![
                key_event_with_modifiers(egui::Key::A, egui::Modifiers::CTRL),
                egui::Event::Text("1e".to_owned()),
            ]),
            |ctx| {
                render_property_dialog(ctx, &mut state);
            },
        );
        assert_eq!(
            state.tabbed_property_dialog.numeric_text_draft("tc1"),
            Some("01e")
        );
        assert!(
            state
                .tabbed_property_dialog
                .validation_errors
                .contains_key("tc1")
        );

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        assert_eq!(
            state.tabbed_property_dialog.numeric_text_draft("tc1"),
            Some("01e")
        );
    }

    #[test]
    fn rendered_partial_policy_commits_only_valid_fields_and_retains_invalid_draft() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        state.schematic.document_policy.property_commit = PropertyCommitPolicy::ApplyValidFields;
        open_property_editor(&mut state, 44);
        state
            .tabbed_property_dialog
            .set_value("name", PropertyValue::String("R99".to_owned()));
        state.tabbed_property_dialog.update_numeric_text_draft(
            "m",
            "0".to_owned(),
            Some("Multiplier must be at least 1".to_owned()),
        );

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });

        assert!(state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R99");
        assert_eq!(state.schematic.components[0].params, "");
        assert_eq!(state.schematic.undo_history.undo_count(), 1);
        assert_eq!(
            state.tabbed_property_dialog.numeric_text_draft("m"),
            Some("0")
        );
        assert!(state.tabbed_property_dialog.is_modified("m"));
        assert!(!state.tabbed_property_dialog.is_modified("name"));

        state
            .tabbed_property_dialog
            .update_numeric_text_draft("m", "2".to_owned(), None);
        state
            .tabbed_property_dialog
            .set_value("m", PropertyValue::number(2.0));
        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });

        assert!(!state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R99");
        assert_eq!(state.schematic.components[0].params, "m=2");
        assert_eq!(state.schematic.undo_history.undo_count(), 2);

        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components[0].name, "R99");
        assert_eq!(state.schematic.components[0].params, "");
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components[0].name, "R1");
    }
}
