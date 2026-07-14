//! Responsive workbench composition policy.

use super::state::{WidthClass, WorkbenchState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutSpec {
    pub width_class: WidthClass,
    pub show_title_menus: bool,
    pub show_activity_rail: bool,
    pub show_navigator_dock: bool,
    pub show_inspector_dock: bool,
    pub show_console_body: bool,
    pub show_phone_navigation: bool,
    pub toolbar_labels: bool,
    pub navigator_width: f32,
    pub inspector_width: f32,
    pub console_height: f32,
}

impl LayoutSpec {
    pub fn resolve(viewport_width: f32, state: &WorkbenchState) -> Self {
        let width_class = WidthClass::for_width(viewport_width);
        let docked = !width_class.uses_drawers() && !state.focus_mode;
        let show_console_body = state.console_visible
            && !state.focus_mode
            && (!width_class.is_phone() || state.console_maximized);
        Self {
            width_class,
            show_title_menus: matches!(width_class, WidthClass::Desktop | WidthClass::Wide),
            show_activity_rail: !width_class.is_phone(),
            show_navigator_dock: docked && state.navigator_visible,
            show_inspector_dock: docked && state.inspector_visible,
            show_console_body,
            show_phone_navigation: width_class.is_phone(),
            toolbar_labels: matches!(width_class, WidthClass::Wide),
            navigator_width: state.navigator_width.clamp(236.0, 460.0),
            inspector_width: state.inspector_width.clamp(260.0, 520.0),
            console_height: if state.console_maximized {
                520.0
            } else {
                state.console_height.clamp(112.0, 420.0)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_uses_drawers_and_bottom_navigation() {
        let spec = LayoutSpec::resolve(390.0, &WorkbenchState::default());

        assert!(!spec.show_navigator_dock);
        assert!(!spec.show_inspector_dock);
        assert!(spec.show_phone_navigation);
        assert!(!spec.show_console_body);
    }

    #[test]
    fn desktop_keeps_both_context_docks() {
        let spec = LayoutSpec::resolve(1280.0, &WorkbenchState::default());

        assert!(spec.show_navigator_dock);
        assert!(spec.show_inspector_dock);
        assert!(spec.show_console_body);
        assert!(!spec.show_phone_navigation);
    }

    #[test]
    fn focus_mode_removes_every_nonessential_dock() {
        let mut state = WorkbenchState::default();
        state.focus_mode = true;
        let spec = LayoutSpec::resolve(1728.0, &state);

        assert!(!spec.show_navigator_dock);
        assert!(!spec.show_inspector_dock);
        assert!(!spec.show_console_body);
    }
}
