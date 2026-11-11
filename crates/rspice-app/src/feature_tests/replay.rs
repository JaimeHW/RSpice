//! The browser jobs' input, replayed natively through whole frames of the
//! application.
//!
//! `tools/ci/browser_workbench.py` drives the real browser workbench by the
//! labels and roles in the AccessKit tree it publishes, as its `controls`
//! reads them, and presses a control at the centre of the bounds that tree
//! gives it. That job needs a WebDriver and a WebGPU browser, so a case here
//! replays the same clicks and keys natively and reads the same tree: a
//! renamed control, a field that lost its name, a row whose announced bounds
//! do not take the press, or a button that no longer opens what the script
//! expects fails here first.

use std::collections::HashMap;

use egui::accesskit::{Affine, NodeId, Point, Role, TreeUpdate};
use egui::{Context, Key, Modifiers, Pos2, Rect, pos2, vec2};

use crate::workbench::RSpiceApp;

/// The primary modifier as the browser's Control key arrives off a Mac.
pub(super) const PRIMARY: Modifiers = Modifiers {
    ctrl: true,
    command: true,
    ..Modifiers::NONE
};

/// One rendered control, as the browser observer reads it.
#[derive(Debug, Clone)]
pub(super) struct Control {
    pub(super) role: Role,
    pub(super) label: String,
    pub(super) description: String,
    pub(super) value: String,
    pub(super) center: Pos2,
}

/// Every control with bounds, its centre carried through the transforms of
/// the nodes above it, in the order `controls` walks them.
fn controls(tree: &TreeUpdate) -> Vec<Control> {
    let nodes: HashMap<NodeId, &egui::accesskit::Node> =
        tree.nodes.iter().map(|(id, node)| (*id, node)).collect();
    let root = tree
        .tree
        .as_ref()
        .expect("every frame publishes the whole tree")
        .root;
    let mut found = Vec::new();
    let mut pending = vec![(root, Affine::IDENTITY)];
    while let Some((id, parent)) = pending.pop() {
        let node = nodes[&id];
        let transform = node.transform().map_or(parent, |own| parent * *own);
        if let Some(bounds) = node.bounds() {
            let center = transform
                * Point::new((bounds.x0 + bounds.x1) / 2.0, (bounds.y0 + bounds.y1) / 2.0);
            found.push(Control {
                role: node.role(),
                label: node.label().unwrap_or_default().to_owned(),
                description: node.description().unwrap_or_default().to_owned(),
                value: node.value().unwrap_or_default().to_owned(),
                center: pos2(center.x as f32, center.y as f32),
            });
        }
        pending.extend(
            node.children()
                .iter()
                .rev()
                .map(|child| (*child, transform)),
        );
    }
    found
}

/// The one control named `label` with `role`, as the script's `click`
/// demands; otherwise what the tree holds under that name or role instead.
pub(super) fn only<'a>(controls: &'a [Control], label: &str, role: Role) -> &'a Control {
    let candidates: Vec<&Control> = controls
        .iter()
        .filter(|control| control.label == label && control.role == role)
        .collect();
    if let [control] = candidates.as_slice() {
        return control;
    }
    let same_label: Vec<(Role, &str)> = controls
        .iter()
        .filter(|control| control.label == label)
        .map(|control| (control.role, control.value.as_str()))
        .collect();
    let same_role: Vec<&str> = controls
        .iter()
        .filter(|control| control.role == role)
        .map(|control| control.label.as_str())
        .collect();
    panic!(
        "expected one {role:?} named {label:?}, found {}; that name is on {same_label:?}; \
         the {role:?} controls are {same_role:?}",
        candidates.len()
    );
}

fn key_event(key: Key, modifiers: Modifiers, pressed: bool) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed,
        repeat: false,
        modifiers,
    }
}

/// The application at the size of the browser job's window, a frame at a time.
pub(super) struct Replay {
    ctx: Context,
    pub(super) app: RSpiceApp,
    frame: eframe::Frame,
    time: f64,
}

impl Replay {
    /// A new session with a project open, as the browser workbench starts.
    pub(super) fn new() -> Self {
        Self::of(RSpiceApp::test_instance())
    }

    /// `app` with a project open, as the browser workbench starts.
    pub(super) fn of(mut app: RSpiceApp) -> Self {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        app.state.open_project_session();
        Self {
            ctx,
            app,
            frame: eframe::Frame::_new_kittest(),
            time: 0.0,
        }
    }

    /// The context the frames run in, for a test that raises what the shell
    /// raises between frames.
    pub(super) fn ctx(&self) -> &Context {
        &self.ctx
    }

    /// Three frames, the first carrying `events`; what the last one published.
    pub(super) fn frames(&mut self, mut events: Vec<egui::Event>) -> Vec<Control> {
        let mut tree = None;
        for _ in 0..3 {
            self.time += 0.1;
            let output = self.ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1440.0, 1000.0))),
                    time: Some(self.time),
                    focused: true,
                    events: std::mem::take(&mut events),
                    ..Default::default()
                },
                |ui| eframe::App::ui(&mut self.app, ui, &mut self.frame),
            );
            tree = output.platform_output.accesskit_update;
        }
        controls(&tree.expect("a frame with AccessKit on publishes its tree"))
    }

    /// The script's `click`: the one control named `label` with `role` in a
    /// fresh frame, pressed and released at its centre.
    pub(super) fn click(&mut self, label: &str, role: Role) -> Vec<Control> {
        let at = only(&self.frames(Vec::new()), label, role).center;
        let press = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: Modifiers::NONE,
        };
        self.frames(vec![egui::Event::PointerMoved(at), press(true)]);
        self.frames(vec![press(false)])
    }

    pub(super) fn key(&mut self, key: Key, modifiers: Modifiers) -> Vec<Control> {
        self.frames(vec![
            key_event(key, modifiers, true),
            key_event(key, modifiers, false),
        ])
    }

    /// Typed text, each line break the Enter key.
    pub(super) fn type_text(&mut self, text: &str) -> Vec<Control> {
        let mut events = Vec::new();
        for (index, line) in text.split('\n').enumerate() {
            if index > 0 {
                events.push(key_event(Key::Enter, Modifiers::NONE, true));
                events.push(key_event(Key::Enter, Modifiers::NONE, false));
            }
            if !line.is_empty() {
                events.push(egui::Event::Text(line.to_owned()));
            }
        }
        self.frames(events)
    }

    /// The script's `choose_command`: the palette by its key, its search
    /// cleared and given `label`, and the row of that name pressed.
    pub(super) fn choose_command(&mut self, label: &str) -> Vec<Control> {
        let opened = self.key(Key::K, PRIMARY);
        only(&opened, "Command search", Role::ComboBox);
        self.click("Command search", Role::ComboBox);
        self.key(Key::A, PRIMARY);
        let found = self.type_text(label);
        let row = only(&found, label, Role::ListBoxOption);
        assert!(
            !row.description.starts_with("Unavailable:"),
            "{label}: {}",
            row.description
        );
        self.click(label, Role::ListBoxOption)
    }
}
