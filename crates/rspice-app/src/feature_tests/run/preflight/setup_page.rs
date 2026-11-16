//! A Simulate setup page on the running application, driven a frame at a
//! time with real input events and read back through the accessibility tree
//! the page announces.

use super::*;
use egui::{Pos2, Rect, vec2};
use rspice_app_state::state::SimulationPage;

/// One control as the page's accessibility tree announces it.
#[derive(Debug, Clone)]
pub(super) struct Control {
    pub(super) label: String,
    pub(super) role: egui::accesskit::Role,
    pub(super) value: Option<String>,
    pub(super) description: Option<String>,
    pub(super) rect: Rect,
}

/// A Simulate setup page drawn on the application.
pub(super) struct Page {
    ctx: egui::Context,
    pub(super) app: RSpiceApp,
    pub(super) controls: Vec<Control>,
    /// The label of the control holding keyboard focus.
    pub(super) focused: Option<String>,
    /// The route the page asked for in the last pass.
    pub(super) routed:
        Option<rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticRoute>,
}

impl Page {
    /// The Simulate setup page `shown`, drawn on `app`.
    pub(super) fn open_on(mut app: RSpiceApp, shown: SimulationPage) -> Self {
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        app.state.workbench.activate(Workspace::Simulate);
        app.state.simulate.simulation_page = shown;
        let mut page = Self {
            ctx,
            app,
            controls: Vec::new(),
            focused: None,
            routed: None,
        };
        // Twice: the first pass builds the fonts the second lays out with.
        page.pass(Vec::new());
        page.pass(Vec::new());
        page
    }

    /// One frame of the page, and the route it asked for applied, as the
    /// frame loop applies it.
    pub(super) fn pass(&mut self, events: Vec<egui::Event>) {
        use rspice_app_workbench::workbench::framework::host::{Host, Point};
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 1600.0))),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        app.contribute(Point::Surface(Workspace::Simulate), ui);
                    });
            },
        );
        if let Some(route) = self.app.state.model.diagnostics.take_pending_route() {
            self.routed = Some(route.clone());
            self.app.state.model.diagnostics.request_route(route);
        }
        crate::workbench::diagnostic_routes::apply_pending(&self.ctx, &mut self.app);
        let Some(update) = output.platform_output.accesskit_update else {
            return;
        };
        self.focused = update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .and_then(|(_, node)| node.label().map(str::to_owned));
        self.controls = update
            .nodes
            .iter()
            .filter_map(|(_, node)| {
                let bounds = node.bounds()?;
                Some(Control {
                    label: node.label()?.to_owned(),
                    role: node.role(),
                    value: node.value().map(str::to_owned),
                    description: node.description().map(str::to_owned),
                    rect: Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                })
            })
            .collect();
    }

    pub(super) fn control(&self, label: &str) -> &Control {
        let hits: Vec<&Control> = self
            .controls
            .iter()
            .filter(|control| control.label == label)
            .collect();
        assert_eq!(
            hits.len(),
            1,
            "one control called {label:?}; the page announces {:?}",
            self.controls
                .iter()
                .map(|control| control.label.as_str())
                .collect::<Vec<_>>()
        );
        hits[0]
    }

    pub(super) fn click(&mut self, label: &str) {
        let at = self.control(label).rect.center();
        self.click_at(at);
    }

    pub(super) fn click_at(&mut self, at: Pos2) {
        self.pass(vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ]);
        self.pass(Vec::new());
    }

    pub(super) fn key(&mut self, key: egui::Key) {
        self.pass(vec![egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }]);
        self.pass(Vec::new());
    }

    /// Replace what the focused field holds with `text`.
    pub(super) fn type_over(&mut self, text: &str) {
        let mut events = vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        }];
        if text.is_empty() {
            events.push(egui::Event::Key {
                key: egui::Key::Backspace,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::default(),
            });
        } else {
            events.push(egui::Event::Text(text.to_owned()));
        }
        self.pass(events);
    }

    /// Click the field called `label`, type `text` over it and Tab off it.
    pub(super) fn retype(&mut self, label: &str, text: &str) {
        self.click(label);
        self.type_over(text);
        self.key(egui::Key::Tab);
    }

    /// Click the field called `label`, clear it and type `text` one
    /// keystroke a frame, as a hand types it, then Tab off it.
    pub(super) fn type_slowly(&mut self, label: &str, text: &str) {
        self.click(label);
        self.type_over("");
        for typed in text.chars() {
            self.pass(vec![egui::Event::Text(typed.to_string())]);
        }
        self.key(egui::Key::Tab);
    }
}

/// The whole options record the plan holds, in its stored form: what an Undo
/// has to put back exactly.
pub(super) fn stored_options(app: &RSpiceApp) -> String {
    serde_json::to_string(&app.state.model.sim_setup.options).expect("the options serialize")
}
