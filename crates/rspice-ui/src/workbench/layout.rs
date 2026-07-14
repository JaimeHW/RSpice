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
    pub show_pvt_selector: bool,
    pub navigator_width: f32,
    pub inspector_width: f32,
    pub console_min_height: f32,
    pub console_max_height: f32,
    pub console_height: f32,
}

impl LayoutSpec {
    pub fn resolve(viewport_width: f32, viewport_height: f32, state: &WorkbenchState) -> Self {
        let width_class = WidthClass::for_width(viewport_width);
        let docked = !width_class.uses_drawers() && !state.focus_mode;
        let show_console_body = state.console_visible
            && !state.focus_mode
            && (!width_class.is_phone() || state.console_maximized);
        let (console_min_height, console_max_height) =
            console_height_bounds(width_class, viewport_height, state.console_maximized);
        let requested_console_height = if state.console_maximized {
            console_max_height
        } else {
            state.console_height
        };
        Self {
            width_class,
            show_title_menus: matches!(width_class, WidthClass::Desktop | WidthClass::Wide),
            show_activity_rail: !width_class.is_phone(),
            show_navigator_dock: docked && state.navigator_visible,
            show_inspector_dock: docked && state.inspector_visible,
            show_console_body,
            show_phone_navigation: width_class.is_phone(),
            toolbar_labels: matches!(width_class, WidthClass::Wide),
            // The mockup removes the compact PVT selector at 820 px while
            // keeping Run and the panel toggles fixed at the trailing edge.
            show_pvt_selector: viewport_width > 820.0
                && !(viewport_width <= 900.0 && viewport_height <= 500.0),
            navigator_width: state.navigator_width.clamp(236.0, 460.0),
            inspector_width: state.inspector_width.clamp(260.0, 520.0),
            console_min_height,
            console_max_height,
            console_height: requested_console_height.clamp(console_min_height, console_max_height),
        }
    }
}

fn console_height_bounds(
    width_class: WidthClass,
    viewport_height: f32,
    maximized: bool,
) -> (f32, f32) {
    if width_class.uses_drawers() {
        // Match the touch composition: 42% of viewport height capped at
        // 260 px, with the short-landscape override of 45% capped at 180 px. Ninety
        // pixels is the mockup's splitter accessibility floor.
        let cap = if viewport_height <= 500.0 {
            (viewport_height * 0.45).min(180.0)
        } else {
            (viewport_height * 0.42).min(260.0)
        }
        .max(90.0);
        (90.0, cap)
    } else if maximized {
        (112.0, 520.0)
    } else {
        (112.0, 420.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_uses_drawers_and_bottom_navigation() {
        let spec = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());

        assert!(!spec.show_navigator_dock);
        assert!(!spec.show_inspector_dock);
        assert!(spec.show_phone_navigation);
        assert!(!spec.show_console_body);
        assert!(!spec.show_pvt_selector);
    }

    #[test]
    fn desktop_keeps_both_context_docks() {
        let spec = LayoutSpec::resolve(1280.0, 900.0, &WorkbenchState::default());

        assert!(spec.show_navigator_dock);
        assert!(spec.show_inspector_dock);
        assert!(spec.show_console_body);
        assert!(!spec.show_phone_navigation);
    }

    #[test]
    fn focus_mode_removes_every_nonessential_dock() {
        let mut state = WorkbenchState::default();
        state.focus_mode = true;
        let spec = LayoutSpec::resolve(1728.0, 1117.0, &state);

        assert!(!spec.show_navigator_dock);
        assert!(!spec.show_inspector_dock);
        assert!(!spec.show_console_body);
    }

    #[test]
    fn tablet_console_cannot_dominate_portrait_workspace() {
        let mut state = WorkbenchState::default();
        state.console_height = 640.0;
        state.console_maximized = true;

        let spec = LayoutSpec::resolve(834.0, 1112.0, &state);

        assert_eq!(spec.console_height, 260.0);
        assert_eq!(spec.console_max_height, 260.0);
    }

    #[test]
    fn short_touch_landscape_uses_the_mockup_console_cap() {
        let mut state = WorkbenchState::default();
        state.console_maximized = true;

        let spec = LayoutSpec::resolve(844.0, 390.0, &state);

        assert_eq!(spec.console_height, 175.5);
        assert_eq!(spec.console_max_height, 175.5);
    }

    #[test]
    fn pvt_selector_follows_the_mockup_width_cutoff() {
        assert!(!LayoutSpec::resolve(820.0, 900.0, &WorkbenchState::default()).show_pvt_selector);
        assert!(LayoutSpec::resolve(821.0, 900.0, &WorkbenchState::default()).show_pvt_selector);
        assert!(!LayoutSpec::resolve(844.0, 390.0, &WorkbenchState::default()).show_pvt_selector);
    }
}
