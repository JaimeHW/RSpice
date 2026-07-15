//! Responsive workbench composition policy.

use super::state::{WidthClass, WorkbenchState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutSpec {
    pub width_class: WidthClass,
    pub compact_shell: bool,
    pub coarse_pointer: bool,
    pub show_status_bar: bool,
    pub show_activity_rail: bool,
    pub show_navigator_dock: bool,
    pub show_inspector_dock: bool,
    pub navigator_uses_drawer: bool,
    pub inspector_uses_drawer: bool,
    pub workspaces_uses_drawer: bool,
    pub has_overlay_drawer: bool,
    pub show_console_strip: bool,
    pub show_console_body: bool,
    pub console_resizable: bool,
    pub show_phone_navigation: bool,
    pub toolbar_labels: bool,
    pub toolbar_tool_limit: Option<usize>,
    pub show_pvt_selector: bool,
    pub show_run_config_selector: bool,
    pub navigator_resizable: bool,
    pub inspector_resizable: bool,
    pub title_bar_height: f32,
    pub toolbar_height: f32,
    pub toolbar_control_height: f32,
    pub run_control_height: f32,
    pub document_bar_height: f32,
    pub status_bar_height: f32,
    pub phone_navigation_height: f32,
    pub navigator_width: f32,
    pub inspector_width: f32,
    pub console_min_height: f32,
    pub console_max_height: f32,
    pub console_height: f32,
}

impl LayoutSpec {
    #[cfg(test)]
    pub fn resolve(viewport_width: f32, viewport_height: f32, state: &WorkbenchState) -> Self {
        Self::resolve_with_pointer(viewport_width, viewport_height, false, state)
    }

    pub fn resolve_with_pointer(
        viewport_width: f32,
        viewport_height: f32,
        coarse_pointer: bool,
        state: &WorkbenchState,
    ) -> Self {
        let width_class = WidthClass::for_width(viewport_width);
        let short_landscape = viewport_width <= 900.0 && viewport_height <= 500.0;
        let compact_shell = width_class.uses_bottom_navigation() || short_landscape;
        let navigator_uses_drawer = compact_shell;
        let inspector_uses_drawer = compact_shell || width_class.inspector_uses_drawer();
        let workspaces_uses_drawer = viewport_width <= 560.0 || short_landscape;
        let show_console_body =
            (state.console_visible || state.console_maximized) && !state.focus_mode;
        // Explicit mobile/coarse-pointer landscape gives the constrained
        // canvas the entire remaining height. A fine-pointer desktop/browser
        // window at the same dimensions retains the mockup's 31 px strip.
        let show_console_strip = show_console_body || !short_landscape || !coarse_pointer;
        let touch_console = compact_shell || coarse_pointer;
        let maximized = state.console_maximized && !touch_console;
        let metrics = ChromeMetrics::resolve(viewport_width, short_landscape, coarse_pointer);
        let (console_min_height, console_max_height, console_height) = if show_console_body {
            let (minimum, maximum) =
                console_height_bounds(touch_console, viewport_height, maximized);
            let requested = if maximized {
                maximum
            } else {
                state.console_height
            };
            (minimum, maximum, requested.clamp(minimum, maximum))
        } else {
            (
                metrics.console_tab,
                metrics.console_tab,
                metrics.console_tab,
            )
        };

        let show_navigator_dock =
            !navigator_uses_drawer && !state.focus_mode && state.navigator_visible;
        let show_inspector_dock =
            !inspector_uses_drawer && !state.focus_mode && state.inspector_visible;

        Self {
            width_class,
            compact_shell,
            coarse_pointer,
            show_status_bar: metrics.status > 0.0,
            show_activity_rail: !compact_shell,
            show_navigator_dock,
            show_inspector_dock,
            navigator_uses_drawer,
            inspector_uses_drawer,
            workspaces_uses_drawer,
            has_overlay_drawer: navigator_uses_drawer
                || inspector_uses_drawer
                || workspaces_uses_drawer,
            show_console_strip,
            show_console_body,
            console_resizable: show_console_body && !touch_console && !maximized,
            show_phone_navigation: compact_shell,
            // The mockup retains labeled tools above its 820 px compact-shell
            // cutoff; only the compact toolbar projects icons alone.
            toolbar_labels: viewport_width > 820.0 && !short_landscape,
            toolbar_tool_limit: if viewport_width <= 560.0 {
                Some(2)
            } else if short_landscape {
                Some(3)
            } else {
                None
            },
            show_pvt_selector: viewport_width > 820.0 && !short_landscape,
            // The canonical run-configuration summary is present only when
            // both workstation docks fit (the mockup removes it at 1260 px).
            show_run_config_selector: matches!(width_class, WidthClass::Wide) && !short_landscape,
            navigator_resizable: matches!(width_class, WidthClass::Wide),
            inspector_resizable: matches!(width_class, WidthClass::Wide),
            title_bar_height: metrics.title,
            toolbar_height: metrics.toolbar,
            toolbar_control_height: metrics.toolbar_control,
            run_control_height: metrics.run_control,
            document_bar_height: metrics.document,
            status_bar_height: metrics.status,
            phone_navigation_height: 54.0,
            // At 821-1260 px the mockup fixes the left column at 228 px and
            // removes its splitter. Wide layouts restore the persisted dock.
            navigator_width: if matches!(width_class, WidthClass::Desktop) {
                228.0
            } else {
                state.navigator_width.clamp(220.0, 440.0)
            },
            inspector_width: state.inspector_width.clamp(278.0, 440.0),
            console_min_height,
            console_max_height,
            console_height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ChromeMetrics {
    title: f32,
    toolbar: f32,
    toolbar_control: f32,
    run_control: f32,
    document: f32,
    status: f32,
    console_tab: f32,
}

impl ChromeMetrics {
    fn resolve(width: f32, short_landscape: bool, coarse_pointer: bool) -> Self {
        let mut metrics = Self {
            title: 35.0,
            toolbar: 45.0,
            toolbar_control: 29.0,
            run_control: 31.0,
            document: 34.0,
            status: 25.0,
            console_tab: 31.0,
        };

        if width <= 820.0 {
            metrics.title = 42.0;
            metrics.toolbar = 47.0;
            // The late commercial/a11y cascade overrides the earlier compact
            // 34 px declarations for every interactive toolbar target.
            metrics.toolbar_control = 44.0;
            metrics.run_control = 44.0;
            metrics.document = 36.0;
            metrics.status = 27.0;
        }
        if width <= 560.0 {
            metrics.title = 40.0;
            metrics.toolbar = 46.0;
            metrics.status = 0.0;
        }
        if short_landscape {
            metrics.title = 44.0;
            metrics.toolbar = 48.0;
            // The short-landscape rule changes the chrome rows, but does not
            // redefine tool targets. Fine-pointer 821-900 px windows retain
            // the workstation 29/31 px controls; <=820 px and coarse-pointer
            // rules continue to impose their later 44 px minimums.
            metrics.document = 34.0;
            metrics.status = 0.0;
        }
        if coarse_pointer {
            // The final coarse-pointer contract is independent of viewport
            // width: title, toolbar, and their primary controls all retain a
            // 44 px minimum target on wide touch workstations too.
            metrics.title = 44.0;
            metrics.toolbar = 48.0;
            metrics.toolbar_control = 44.0;
            metrics.run_control = 44.0;
            metrics.document = 44.0;
            metrics.console_tab = 44.0;
            // The mockup's explicit tablet platform uses a 44 px status row;
            // phone and short-landscape compositions intentionally hide it.
            if width > 560.0 && width <= 1200.0 && !short_landscape {
                metrics.status = 44.0;
            }
        }

        metrics
    }
}

fn console_height_bounds(touch_layout: bool, viewport_height: f32, maximized: bool) -> (f32, f32) {
    if touch_layout {
        // Match the touch composition: 42% of viewport height capped at
        // 260 px, with the short-landscape override of 45% capped at 180 px.
        // Ninety pixels is the mockup's splitter accessibility floor.
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
    fn phone_uses_drawers_five_item_navigation_and_exact_chrome() {
        let spec = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());

        assert!(!spec.show_navigator_dock);
        assert!(!spec.show_inspector_dock);
        assert!(spec.navigator_uses_drawer);
        assert!(spec.inspector_uses_drawer);
        assert!(spec.workspaces_uses_drawer);
        assert!(spec.show_phone_navigation);
        assert!(!spec.show_console_body);
        assert!(!spec.show_pvt_selector);
        assert_eq!(spec.toolbar_tool_limit, Some(2));
        assert!(!spec.show_status_bar);
        assert_eq!(spec.title_bar_height, 40.0);
        assert_eq!(spec.toolbar_height, 46.0);
        assert_eq!(spec.document_bar_height, 36.0);
        assert_eq!(spec.phone_navigation_height, 54.0);
    }

    #[test]
    fn tablet_width_uses_two_drawers_and_all_workspace_items() {
        let spec = LayoutSpec::resolve(768.0, 1024.0, &WorkbenchState::default());

        assert!(!spec.show_activity_rail);
        assert!(spec.navigator_uses_drawer);
        assert!(spec.inspector_uses_drawer);
        assert!(!spec.workspaces_uses_drawer);
        assert_eq!(spec.title_bar_height, 42.0);
        assert_eq!(spec.toolbar_control_height, 44.0);
        assert_eq!(spec.run_control_height, 44.0);
        assert_eq!(spec.status_bar_height, 27.0);
    }

    #[test]
    fn intermediate_desktop_docks_left_and_draws_right() {
        let spec = LayoutSpec::resolve(834.0, 1112.0, &WorkbenchState::default());

        assert!(spec.show_activity_rail);
        assert!(spec.show_navigator_dock);
        assert!(!spec.navigator_uses_drawer);
        assert!(!spec.show_inspector_dock);
        assert!(spec.inspector_uses_drawer);
        assert_eq!(spec.navigator_width, 228.0);
        assert!(!spec.navigator_resizable);
    }

    #[test]
    fn wide_desktop_keeps_both_context_docks() {
        let spec = LayoutSpec::resolve(1280.0, 900.0, &WorkbenchState::default());

        assert!(spec.show_navigator_dock);
        assert!(spec.show_inspector_dock);
        assert!(spec.show_activity_rail);
        assert!(!spec.show_phone_navigation);
        assert!(spec.navigator_resizable);
        assert!(spec.inspector_resizable);
        assert!(spec.show_run_config_selector);
    }

    #[test]
    fn focus_mode_collapses_console_and_removes_context_docks() {
        let mut state = WorkbenchState::default();
        state.focus_mode = true;
        state.console_visible = true;
        let spec = LayoutSpec::resolve(1728.0, 1117.0, &state);

        assert!(!spec.show_navigator_dock);
        assert!(!spec.show_inspector_dock);
        assert!(!spec.show_console_body);
        assert_eq!(spec.console_height, 31.0);
    }

    #[test]
    fn coarse_tablet_uses_touch_metrics_and_bounded_console() {
        let mut state = WorkbenchState::default();
        state.console_height = 640.0;
        state.console_visible = true;

        let spec = LayoutSpec::resolve_with_pointer(834.0, 1112.0, true, &state);

        assert_eq!(spec.title_bar_height, 44.0);
        assert_eq!(spec.toolbar_height, 48.0);
        assert_eq!(spec.document_bar_height, 44.0);
        assert_eq!(spec.status_bar_height, 44.0);
        assert_eq!(spec.console_height, 260.0);
        assert_eq!(spec.console_max_height, 260.0);
        assert!(!spec.console_resizable);
    }

    #[test]
    fn wide_coarse_pointer_keeps_the_mockup_touch_target_contract() {
        let spec =
            LayoutSpec::resolve_with_pointer(1_440.0, 900.0, true, &WorkbenchState::default());

        assert_eq!(spec.title_bar_height, 44.0);
        assert_eq!(spec.toolbar_height, 48.0);
        assert_eq!(spec.toolbar_control_height, 44.0);
        assert_eq!(spec.run_control_height, 44.0);
        assert_eq!(spec.document_bar_height, 44.0);
        assert_eq!(spec.status_bar_height, 25.0);
        assert_eq!(spec.console_height, 44.0);
    }

    #[test]
    fn short_landscape_uses_the_mockup_compact_override() {
        let mut state = WorkbenchState::default();
        state.console_visible = true;
        state.console_height = 640.0;

        let spec = LayoutSpec::resolve(844.0, 390.0, &state);

        assert!(spec.compact_shell);
        assert!(spec.workspaces_uses_drawer);
        assert!(spec.show_phone_navigation);
        assert!(spec.show_console_strip);
        assert!(spec.show_console_body);
        assert!(!spec.show_status_bar);
        assert_eq!(spec.title_bar_height, 44.0);
        assert_eq!(spec.toolbar_height, 48.0);
        assert_eq!(spec.toolbar_control_height, 29.0);
        assert_eq!(spec.run_control_height, 31.0);
        assert_eq!(spec.document_bar_height, 34.0);
        assert!(!spec.toolbar_labels);
        assert_eq!(spec.console_height, 175.5);
        assert_eq!(spec.console_max_height, 175.5);
        assert!(!spec.show_pvt_selector);
        assert_eq!(spec.toolbar_tool_limit, Some(3));
    }

    #[test]
    fn collapsed_phone_landscape_removes_the_console_row() {
        let spec = LayoutSpec::resolve_with_pointer(844.0, 390.0, true, &WorkbenchState::default());

        assert!(spec.compact_shell);
        assert!(!spec.show_console_body);
        assert!(!spec.show_console_strip);
        assert_eq!(spec.toolbar_control_height, 44.0);
        assert_eq!(spec.run_control_height, 44.0);
    }

    #[test]
    fn narrow_fine_pointer_landscape_keeps_the_late_44_px_target_rule() {
        let spec = LayoutSpec::resolve(800.0, 390.0, &WorkbenchState::default());

        assert!(spec.compact_shell);
        assert_eq!(spec.toolbar_control_height, 44.0);
        assert_eq!(spec.run_control_height, 44.0);
    }

    #[test]
    fn fine_pointer_short_landscape_retains_the_collapsed_console_strip() {
        let spec = LayoutSpec::resolve(844.0, 390.0, &WorkbenchState::default());

        assert!(spec.compact_shell);
        assert!(!spec.show_console_body);
        assert!(spec.show_console_strip);
        assert_eq!(spec.console_height, 31.0);
    }

    #[test]
    fn pvt_selector_follows_the_mockup_width_cutoff() {
        assert!(!LayoutSpec::resolve(820.0, 900.0, &WorkbenchState::default()).show_pvt_selector);
        assert!(LayoutSpec::resolve(821.0, 900.0, &WorkbenchState::default()).show_pvt_selector);
        assert!(!LayoutSpec::resolve(844.0, 390.0, &WorkbenchState::default()).show_pvt_selector);
        assert!(
            !LayoutSpec::resolve(1260.0, 900.0, &WorkbenchState::default())
                .show_run_config_selector
        );
        assert!(
            LayoutSpec::resolve(1261.0, 900.0, &WorkbenchState::default()).show_run_config_selector
        );
    }
}
