//! The browser's stand-in for a platform accessibility bridge.
//!
//! eframe's web backend throws away the AccessKit tree egui builds, so a screen
//! reader finds nothing inside the canvas. This reads that same tree and keeps
//! a small off-screen part of the page in step with it: the focused control,
//! the caret's line in a text document, the fields of the dialog in front, and
//! the live regions a screen reader speaks from. Only what changed is written,
//! and nothing here takes the keyboard: DOM focus stays on the canvas.

use std::collections::HashMap;

use egui::accesskit::{Live, Node, NodeId, Role, Toggled, TreeUpdate};

use crate::workbench::localization::{MessageCatalog, MessageId};

/// One control as the mirror states it.
#[derive(Debug, Clone, PartialEq)]
pub struct MirroredControl {
    pub role: Role,
    pub name: String,
    pub value: String,
    pub description: String,
    pub toggled: Option<Toggled>,
    pub disabled: bool,
    pub read_only: bool,
}

/// The line the caret is on, in the document the keyboard is in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaretLine {
    pub editor: String,
    pub text: String,
}

/// The dialog in front and everything it asks the reader.
#[derive(Debug, Clone, PartialEq)]
pub struct MirroredDialog {
    pub title: String,
    pub description: String,
    pub fields: Vec<MirroredControl>,
}

/// What one frame's AccessKit tree says, reduced to what the page mirrors.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AccessibleProjection {
    pub focused: Option<(NodeId, MirroredControl)>,
    pub caret_line: Option<CaretLine>,
    pub dialog: Option<MirroredDialog>,
    pub live: Vec<(NodeId, String)>,
}

/// Reduce one frame's tree. Costs one pass over the nodes; the parts that need
/// the tree's shape are only walked when a dialog or a text document is in
/// play.
#[must_use]
pub fn project(update: &TreeUpdate) -> AccessibleProjection {
    let mut focused = None;
    let mut live = Vec::new();
    let mut has_dialog = false;
    for (id, node) in &update.nodes {
        if *id == update.focus {
            focused = Some(node);
        }
        has_dialog |= node.role() == Role::Dialog;
        if node.live().is_some_and(|live| live != Live::Off)
            && let Some(text) = live_text(node)
        {
            live.push((*id, text.to_owned()));
        }
    }
    live.sort_unstable_by_key(|(id, _)| *id);

    let is_root = update
        .tree
        .as_ref()
        .is_some_and(|tree| tree.root == update.focus);
    let focused = focused.filter(|_| !is_root);
    let is_text_document = focused.is_some_and(|node| node.role() == Role::MultilineTextInput);
    let index = (has_dialog || is_text_document || focused.is_some_and(has_borrowed_name))
        .then(|| index_nodes(update));

    AccessibleProjection {
        focused: focused.map(|node| (update.focus, mirrored(node, index.as_ref()))),
        caret_line: focused
            .filter(|_| is_text_document)
            .zip(index.as_ref())
            .and_then(|(node, index)| caret_line(node, index)),
        dialog: index
            .as_ref()
            .filter(|_| has_dialog)
            .and_then(|index| frontmost_dialog(update, index)),
        live,
    }
}

type NodeIndex<'a> = HashMap<NodeId, &'a Node>;

fn index_nodes(update: &TreeUpdate) -> NodeIndex<'_> {
    update.nodes.iter().map(|(id, node)| (*id, node)).collect()
}

fn has_borrowed_name(node: &Node) -> bool {
    node.label().is_none() && !node.labelled_by().is_empty()
}

/// The words a live or plain-text node holds.
fn spoken_text(node: &Node) -> Option<&str> {
    node.label()
        .or_else(|| node.value())
        .map(str::trim)
        .filter(|text| !text.is_empty())
}

/// A live node's words, untrimmed: the same answer given twice differs from
/// the last only by the no-break space the status line adds to it.
fn live_text(node: &Node) -> Option<&str> {
    node.label()
        .or_else(|| node.value())
        .filter(|text| !text.trim().is_empty())
}

fn name_of(node: &Node, index: Option<&NodeIndex<'_>>) -> String {
    if let Some(label) = node.label() {
        return label.to_owned();
    }
    index.map_or_else(String::new, |index| {
        node.labelled_by()
            .iter()
            .filter_map(|id| index.get(id).and_then(|node| spoken_text(node)))
            .collect::<Vec<_>>()
            .join(" ")
    })
}

fn value_of(node: &Node) -> String {
    if node.role() == Role::Label {
        // A label's words are its name in the page.
        return String::new();
    }
    node.value().map(str::to_owned).unwrap_or_else(|| {
        node.numeric_value()
            .map_or_else(String::new, |value| value.to_string())
    })
}

fn mirrored(node: &Node, index: Option<&NodeIndex<'_>>) -> MirroredControl {
    MirroredControl {
        role: node.role(),
        name: if node.role() == Role::Label {
            spoken_text(node).unwrap_or_default().to_owned()
        } else {
            name_of(node, index)
        },
        value: value_of(node),
        description: node.description().unwrap_or_default().to_owned(),
        toggled: node.toggled(),
        disabled: node.is_disabled(),
        read_only: node.is_read_only(),
    }
}

/// The line under the caret, from the text runs the editor publishes.
fn caret_line(editor: &Node, index: &NodeIndex<'_>) -> Option<CaretLine> {
    let caret = editor.text_selection()?.focus.node;
    let runs: Vec<&Node> = editor
        .children()
        .iter()
        .filter_map(|id| index.get(id).copied())
        .filter(|node| node.role() == Role::TextRun)
        .collect();
    let at = editor
        .children()
        .iter()
        .filter(|id| {
            index
                .get(id)
                .is_some_and(|node| node.role() == Role::TextRun)
        })
        .position(|id| *id == caret)?;
    // A long line arrives as several runs; only the last of them ends it.
    let ends_line = |node: &&Node| node.value().is_some_and(|value| value.ends_with('\n'));
    let first = runs[..at]
        .iter()
        .rposition(ends_line)
        .map_or(0, |ended| ended + 1);
    let last = runs[at..]
        .iter()
        .position(ends_line)
        .map_or(runs.len() - 1, |ended| at + ended);
    let text: String = runs[first..=last]
        .iter()
        .filter_map(|node| node.value())
        .collect();
    Some(CaretLine {
        editor: editor.label().unwrap_or_default().to_owned(),
        text: text.trim_end_matches('\n').to_owned(),
    })
}

fn is_field(role: Role) -> bool {
    matches!(
        role,
        Role::Button
            | Role::CheckBox
            | Role::ComboBox
            | Role::EditableComboBox
            | Role::Heading
            | Role::Label
            | Role::Link
            | Role::ListBoxOption
            | Role::MenuItem
            | Role::MultilineTextInput
            | Role::NumberInput
            | Role::RadioButton
            | Role::SearchInput
            | Role::Slider
            | Role::SpinButton
            | Role::Switch
            | Role::Tab
            | Role::TextInput
    )
}

/// The dialog on top: dialogs are drawn in stacking order, so the last modal
/// one in the tree is the one in front.
fn frontmost_dialog(update: &TreeUpdate, index: &NodeIndex<'_>) -> Option<MirroredDialog> {
    fn walk<'a>(id: NodeId, index: &NodeIndex<'a>, order: &mut Vec<NodeId>) {
        order.push(id);
        if let Some(node) = index.get(&id) {
            for child in node.children() {
                walk(*child, index, order);
            }
        }
    }
    let mut order = Vec::new();
    walk(update.tree.as_ref()?.root, index, &mut order);

    let dialog = order
        .iter()
        .rev()
        .find(|id| {
            index
                .get(id)
                .is_some_and(|node| node.role() == Role::Dialog && node.is_modal())
        })
        .or_else(|| {
            order.iter().rev().find(|id| {
                index
                    .get(id)
                    .is_some_and(|node| node.role() == Role::Dialog)
            })
        })
        .copied()?;

    let mut fields_order = Vec::new();
    walk(dialog, index, &mut fields_order);
    let name_sources: std::collections::HashSet<NodeId> = fields_order
        .iter()
        .filter_map(|id| index.get(id))
        .flat_map(|node| node.labelled_by().iter().copied())
        .collect();
    let node = index.get(&dialog)?;
    let title = node.label().unwrap_or_default();
    let fields = fields_order[1..]
        .iter()
        .filter(|id| !name_sources.contains(id))
        .filter_map(|id| index.get(id))
        .filter(|node| is_field(node.role()))
        .map(|node| mirrored(node, Some(index)))
        .filter(|control| !(control.name.is_empty() && control.value.is_empty()))
        // The heading drawn inside the dialog is its title already.
        .filter(|control| control.role != Role::Label || control.name != title)
        .collect();
    Some(MirroredDialog {
        title: title.to_owned(),
        description: node.description().unwrap_or_default().to_owned(),
        fields,
    })
}

/// What the page must change, and nothing else. Empty when nothing did.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MirrorPatch {
    pub focused: Option<Option<MirroredControl>>,
    pub caret_line: Option<Option<CaretLine>>,
    pub dialog: Option<Option<MirroredDialog>>,
    pub focus_announcement: Option<String>,
    pub announcement: Option<String>,
}

impl MirrorPatch {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

/// The page as last written.
#[derive(Debug, Default)]
pub struct MirrorState {
    started: bool,
    focused: Option<(NodeId, MirroredControl)>,
    caret_line: Option<CaretLine>,
    dialog: Option<MirroredDialog>,
    live: HashMap<NodeId, String>,
}

impl MirrorState {
    /// Compare this frame with what the page shows.
    pub fn patch(
        &mut self,
        projection: AccessibleProjection,
        messages: MessageCatalog,
    ) -> MirrorPatch {
        let mut patch = MirrorPatch::default();

        let moved = self.focused.as_ref().map(|(id, _)| *id)
            != projection.focused.as_ref().map(|(id, _)| *id);
        let restated = !moved
            && match (&self.focused, &projection.focused) {
                (Some((_, before)), Some((_, now))) => {
                    !is_text_entry(now.role)
                        && (before.value != now.value || before.toggled != now.toggled)
                }
                _ => false,
            };
        if moved || restated {
            patch.focus_announcement = projection
                .focused
                .as_ref()
                .map(|(_, control)| spoken_control(control, messages));
        }
        if self.focused != projection.focused {
            patch.focused = Some(projection.focused.as_ref().map(|(_, c)| c.clone()));
            self.focused = projection.focused;
        }

        if self.caret_line != projection.caret_line {
            patch.caret_line = Some(projection.caret_line.clone());
            self.caret_line = projection.caret_line;
        }

        let mut announcements = Vec::new();
        if self.dialog.as_ref().map(|dialog| &dialog.title)
            != projection.dialog.as_ref().map(|dialog| &dialog.title)
            && let Some(dialog) = &projection.dialog
        {
            announcements.push(format!(
                "{}, {}",
                dialog.title,
                messages.text(MessageId::AccessibleRoleDialog)
            ));
        }
        if self.dialog != projection.dialog {
            patch.dialog = Some(projection.dialog.clone());
            self.dialog = projection.dialog;
        }

        let mut live = HashMap::with_capacity(projection.live.len());
        for (id, text) in projection.live {
            if self.started && self.live.get(&id) != Some(&text) {
                announcements.push(text.clone());
            }
            live.insert(id, text);
        }
        self.live = live;
        self.started = true;

        if !announcements.is_empty() {
            patch.announcement = Some(announcements.join(". "));
        }
        patch
    }
}

fn is_text_entry(role: Role) -> bool {
    matches!(
        role,
        Role::TextInput | Role::MultilineTextInput | Role::SearchInput | Role::NumberInput
    )
}

/// How a screen reader would say a control on reaching it.
#[must_use]
pub fn spoken_control(control: &MirroredControl, messages: MessageCatalog) -> String {
    let role = role_word(control.role).map(|id| messages.text(id));
    let state = state_word(control).map(|id| messages.text(id));
    [
        Some(control.name.as_str()),
        role.as_deref(),
        state.as_deref(),
        control
            .disabled
            .then(|| messages.text(MessageId::AccessibleStateUnavailable))
            .as_deref(),
        Some(control.value.as_str()),
        Some(control.description.as_str()),
    ]
    .into_iter()
    .flatten()
    .filter(|part| !part.is_empty())
    .collect::<Vec<_>>()
    .join(", ")
}

fn role_word(role: Role) -> Option<MessageId> {
    Some(match role {
        Role::Button => MessageId::AccessibleRoleButton,
        Role::Link => MessageId::AccessibleRoleLink,
        Role::CheckBox => MessageId::AccessibleRoleCheckBox,
        Role::Switch => MessageId::AccessibleRoleSwitch,
        Role::RadioButton => MessageId::AccessibleRoleRadioButton,
        Role::ComboBox | Role::EditableComboBox => MessageId::AccessibleRoleComboBox,
        Role::TextInput | Role::MultilineTextInput | Role::SearchInput | Role::NumberInput => {
            MessageId::AccessibleRoleEdit
        }
        Role::Slider => MessageId::AccessibleRoleSlider,
        Role::SpinButton => MessageId::AccessibleRoleSpinButton,
        Role::ListBoxOption => MessageId::AccessibleRoleListItem,
        Role::Tab => MessageId::AccessibleRoleTab,
        Role::MenuItem => MessageId::AccessibleRoleMenuItem,
        Role::TreeItem => MessageId::AccessibleRoleTreeItem,
        _ => return None,
    })
}

fn state_word(control: &MirroredControl) -> Option<MessageId> {
    let checkable = matches!(
        control.role,
        Role::CheckBox | Role::RadioButton | Role::Switch
    );
    match control.toggled? {
        Toggled::True if checkable => Some(MessageId::AccessibleStateChecked),
        Toggled::True => Some(MessageId::AccessibleStateSelected),
        Toggled::False if checkable => Some(MessageId::AccessibleStateNotChecked),
        Toggled::False => None,
        Toggled::Mixed => Some(MessageId::AccessibleStatePartlyChecked),
    }
}

/// The ARIA role a mirrored control carries in the page. A control with none
/// reads as plain text.
#[must_use]
pub fn aria_role(role: Role) -> Option<&'static str> {
    Some(match role {
        Role::Button => "button",
        Role::Link => "link",
        Role::CheckBox => "checkbox",
        Role::Switch => "switch",
        Role::RadioButton => "radio",
        Role::ComboBox | Role::EditableComboBox => "combobox",
        Role::TextInput | Role::MultilineTextInput | Role::SearchInput | Role::NumberInput => {
            "textbox"
        }
        Role::Slider => "slider",
        Role::SpinButton => "spinbutton",
        Role::ListBoxOption => "option",
        Role::Tab => "tab",
        Role::MenuItem => "menuitem",
        Role::TreeItem => "treeitem",
        Role::Heading => "heading",
        _ => return None,
    })
}

#[cfg(target_arch = "wasm32")]
pub use dom::AccessibleMirror;

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{CaretLine, MirrorPatch, MirrorState, MirroredControl, MirroredDialog};
    use crate::workbench::localization::{MessageCatalog, UiTextLocale};
    use wasm_bindgen::JsValue;
    use web_sys::{Document, Element};

    const FOCUSED_CONTROL: &str = "rspice_mirror_focused_control";
    const CARET_LINE: &str = "rspice_mirror_caret_line";
    const DIALOG: &str = "rspice_mirror_dialog";
    const FOCUS_ANNOUNCEMENT: &str = "rspice_mirror_focus_announcement";
    const ANNOUNCEMENT: &str = "rspice_mirror_announcement";

    /// Keeps the page's off-screen mirror in step with egui's tree. Registered
    /// once; the tree it reads is built whether or not anything else asks.
    #[derive(Default)]
    pub struct AccessibleMirror {
        state: MirrorState,
        messages: Option<MessageCatalog>,
    }

    impl AccessibleMirror {
        /// The language the mirror speaks in.
        pub fn set_messages(&mut self, messages: MessageCatalog) {
            self.messages = Some(messages);
        }
    }

    impl egui::Plugin for AccessibleMirror {
        fn debug_name(&self) -> &'static str {
            "browser accessibility mirror"
        }

        fn setup(&mut self, ctx: &egui::Context) {
            ctx.enable_accesskit();
        }

        fn output_hook(&mut self, ctx: &egui::Context, output: &mut egui::FullOutput) {
            // A pass egui is about to throw away is not what the reader sees.
            if !output.platform_output.request_discard_reasons.is_empty()
                && output.platform_output.num_completed_passes
                    < ctx.options(|options| options.max_passes.get())
            {
                return;
            }
            let Some(update) = &output.platform_output.accesskit_update else {
                return;
            };
            let messages = self
                .messages
                .unwrap_or_else(|| MessageCatalog::new(UiTextLocale::default()));
            let mut patch = self.state.patch(super::project(update), messages);
            if ctx.options(|options| options.screen_reader) {
                // Spoken feedback is on and already says where focus went.
                patch.focus_announcement = None;
            }
            if patch.is_empty() {
                return;
            }
            // Writing attributes and text to elements the page owns does not
            // fail, and a frame could do nothing about it if it did.
            let _ = write(&patch);
        }
    }

    fn write(patch: &MirrorPatch) -> Result<(), JsValue> {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return Ok(());
        };
        if let Some(focused) = &patch.focused {
            if let Some(element) = document.get_element_by_id(FOCUSED_CONTROL) {
                write_control(&element, focused.as_ref())?;
            }
        }
        if let Some(line) = &patch.caret_line {
            if let Some(element) = document.get_element_by_id(CARET_LINE) {
                write_caret_line(&element, line.as_ref())?;
            }
        }
        if let Some(dialog) = &patch.dialog {
            if let Some(element) = document.get_element_by_id(DIALOG) {
                write_dialog(&document, &element, dialog.as_ref())?;
            }
        }
        if let Some(text) = &patch.focus_announcement {
            announce(&document, FOCUS_ANNOUNCEMENT, text);
        }
        if let Some(text) = &patch.announcement {
            announce(&document, ANNOUNCEMENT, text);
        }
        Ok(())
    }

    /// A live region speaks when its text changes, so the same words again
    /// carry a trailing no-break space.
    fn announce(document: &Document, region: &str, text: &str) {
        let Some(element) = document.get_element_by_id(region) else {
            return;
        };
        let repeated = element.text_content().as_deref() == Some(text);
        let text = if repeated {
            format!("{text}\u{a0}")
        } else {
            text.to_owned()
        };
        element.set_text_content(Some(&text));
    }

    fn set_attribute(element: &Element, name: &str, value: Option<&str>) -> Result<(), JsValue> {
        match value {
            Some(value) if element.get_attribute(name).as_deref() != Some(value) => {
                element.set_attribute(name, value)
            }
            Some(_) => Ok(()),
            None if element.has_attribute(name) => element.remove_attribute(name),
            None => Ok(()),
        }
    }

    fn set_text(element: &Element, text: &str) {
        if element.text_content().as_deref() != Some(text) {
            element.set_text_content(Some(text));
        }
    }

    fn write_control(element: &Element, control: Option<&MirroredControl>) -> Result<(), JsValue> {
        let role = control.and_then(|control| super::aria_role(control.role));
        set_attribute(element, "role", role)?;
        let plain = role.is_none();
        set_attribute(
            element,
            "aria-label",
            control
                .filter(|_| !plain)
                .map(|control| control.name.as_str()),
        )?;
        set_attribute(
            element,
            "aria-description",
            control
                .map(|control| control.description.as_str())
                .filter(|text| !text.is_empty()),
        )?;
        set_attribute(
            element,
            "aria-disabled",
            control.filter(|control| control.disabled).map(|_| "true"),
        )?;
        set_attribute(
            element,
            "aria-readonly",
            control.filter(|control| control.read_only).map(|_| "true"),
        )?;
        set_attribute(element, "aria-checked", control.and_then(aria_checked))?;
        set_attribute(
            element,
            "aria-multiline",
            control
                .filter(|control| control.role == egui::accesskit::Role::MultilineTextInput)
                .map(|_| "true"),
        )?;
        set_text(
            element,
            control.map_or("", |control| {
                if plain {
                    control.name.as_str()
                } else {
                    control.value.as_str()
                }
            }),
        );
        Ok(())
    }

    fn aria_checked(control: &MirroredControl) -> Option<&'static str> {
        use egui::accesskit::{Role, Toggled};
        if !matches!(
            control.role,
            Role::CheckBox | Role::RadioButton | Role::Switch
        ) {
            return None;
        }
        Some(match control.toggled? {
            Toggled::True => "true",
            Toggled::False => "false",
            Toggled::Mixed => "mixed",
        })
    }

    fn write_caret_line(element: &Element, line: Option<&CaretLine>) -> Result<(), JsValue> {
        set_attribute(element, "role", line.map(|_| "textbox"))?;
        set_attribute(element, "aria-multiline", line.map(|_| "true"))?;
        set_attribute(element, "aria-readonly", line.map(|_| "true"))?;
        set_attribute(element, "aria-label", line.map(|line| line.editor.as_str()))?;
        set_text(element, line.map_or("", |line| line.text.as_str()));
        Ok(())
    }

    /// Reuse the children already there, so a value changing in one field
    /// touches only that field.
    fn write_dialog(
        document: &Document,
        element: &Element,
        dialog: Option<&MirroredDialog>,
    ) -> Result<(), JsValue> {
        set_attribute(element, "role", dialog.map(|_| "dialog"))?;
        set_attribute(
            element,
            "aria-label",
            dialog.map(|dialog| dialog.title.as_str()),
        )?;
        set_attribute(
            element,
            "aria-description",
            dialog
                .map(|dialog| dialog.description.as_str())
                .filter(|text| !text.is_empty()),
        )?;
        let fields = dialog.map_or(&[][..], |dialog| dialog.fields.as_slice());
        let children = element.children();
        for (position, field) in fields.iter().enumerate() {
            let existing = children.item(position as u32);
            let child = match existing {
                Some(child) => child,
                None => {
                    let child = document.create_element("div")?;
                    element.append_child(&child)?;
                    child
                }
            };
            write_control(&child, Some(field))?;
        }
        while children.length() as usize > fields.len() {
            if let Some(extra) = element.last_element_child() {
                extra.remove();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::localization::UiTextLocale;
    use egui::accesskit::{TextPosition, TextSelection, Tree, TreeId};

    fn node(role: Role) -> Node {
        Node::new(role)
    }

    fn update(root_children: Vec<(NodeId, Node)>, focus: NodeId) -> TreeUpdate {
        let root_id = NodeId(1);
        let mut root = Node::new(Role::Window);
        root.set_children(root_children.iter().map(|(id, _)| *id).collect::<Vec<_>>());
        let mut nodes = vec![(root_id, root)];
        nodes.extend(root_children);
        TreeUpdate {
            nodes,
            tree: Some(Tree::new(root_id)),
            tree_id: TreeId::ROOT,
            focus,
        }
    }

    fn catalog() -> MessageCatalog {
        MessageCatalog::new(UiTextLocale::default())
    }

    fn button(name: &str) -> Node {
        let mut button = node(Role::Button);
        button.set_label(name);
        button
    }

    fn text_input(name: &str, value: &str) -> Node {
        let mut input = node(Role::TextInput);
        input.set_label(name);
        input.set_value(value);
        input
    }

    #[test]
    fn the_root_holding_focus_is_nothing_focused() {
        let seen = project(&update(vec![(NodeId(2), button("Run"))], NodeId(1)));
        assert_eq!(seen.focused, None);
    }

    #[test]
    fn a_focused_control_keeps_its_role_name_value_and_state() {
        let mut check = node(Role::CheckBox);
        check.set_label("Autosave");
        check.set_toggled(Toggled::True);
        let seen = project(&update(vec![(NodeId(2), check)], NodeId(2)));
        let (id, control) = seen.focused.expect("focused");
        assert_eq!(id, NodeId(2));
        assert_eq!(control.role, Role::CheckBox);
        assert_eq!(control.name, "Autosave");
        assert_eq!(control.toggled, Some(Toggled::True));
        assert_eq!(aria_role(control.role), Some("checkbox"));
        assert_eq!(
            spoken_control(&control, catalog()),
            "Autosave, check box, checked"
        );
    }

    #[test]
    fn a_name_can_be_borrowed_from_the_label_beside_it() {
        let mut label = node(Role::Label);
        label.set_value("Temperature");
        let mut input = node(Role::TextInput);
        input.set_value("27");
        input.set_labelled_by(vec![NodeId(2)]);
        let seen = project(&update(
            vec![(NodeId(2), label), (NodeId(3), input)],
            NodeId(3),
        ));
        assert_eq!(seen.focused.expect("focused").1.name, "Temperature");
    }

    fn text_run(value: &str) -> Node {
        let mut run = node(Role::TextRun);
        run.set_value(value);
        run
    }

    #[test]
    fn the_caret_line_joins_the_runs_of_one_line() {
        let mut editor = node(Role::MultilineTextInput);
        editor.set_label("Source");
        editor.set_children(vec![NodeId(11), NodeId(12), NodeId(13), NodeId(14)]);
        editor.set_text_selection(TextSelection {
            anchor: TextPosition {
                node: NodeId(13),
                character_index: 0,
            },
            focus: TextPosition {
                node: NodeId(13),
                character_index: 0,
            },
        });
        let mut update = update(vec![(NodeId(10), editor)], NodeId(10));
        update.nodes.extend([
            (NodeId(11), text_run("first\n")),
            (NodeId(12), text_run("second half ")),
            (NodeId(13), text_run("of the line\n")),
            (NodeId(14), text_run("third")),
        ]);
        let line = project(&update).caret_line.expect("a text document");
        assert_eq!(line.editor, "Source");
        assert_eq!(line.text, "second half of the line");
    }

    fn dialog(title: &str, modal: bool, children: Vec<NodeId>) -> Node {
        let mut dialog = node(Role::Dialog);
        dialog.set_label(title);
        dialog.set_children(children);
        if modal {
            dialog.set_modal();
        }
        dialog
    }

    #[test]
    fn the_dialog_in_front_lists_its_fields() {
        let behind = dialog("Behind", true, vec![NodeId(21)]);
        let front = dialog("Front", true, vec![NodeId(31), NodeId(32), NodeId(33)]);
        let mut label = node(Role::Label);
        label.set_value("Unsaved work is kept.");
        let mut update = update(vec![(NodeId(20), behind), (NodeId(30), front)], NodeId(32));
        update.nodes.extend([
            (NodeId(21), button("Hidden")),
            (NodeId(31), label),
            (NodeId(32), text_input("Name", "gain")),
            (NodeId(33), button("OK")),
        ]);
        let dialog = project(&update).dialog.expect("a dialog is open");
        assert_eq!(dialog.title, "Front");
        let names: Vec<(&str, &str)> = dialog
            .fields
            .iter()
            .map(|field| (field.name.as_str(), field.value.as_str()))
            .collect();
        assert_eq!(
            names,
            vec![("Unsaved work is kept.", ""), ("Name", "gain"), ("OK", "")]
        );
    }

    #[test]
    fn a_field_s_name_label_is_not_listed_twice() {
        let mut label = node(Role::Label);
        label.set_value("Name");
        let mut input = node(Role::TextInput);
        input.set_value("gain");
        input.set_labelled_by(vec![NodeId(31)]);
        let mut update = update(
            vec![(NodeId(30), dialog("D", true, vec![NodeId(31), NodeId(32)]))],
            NodeId(1),
        );
        update
            .nodes
            .extend([(NodeId(31), label), (NodeId(32), input)]);
        let dialog = project(&update).dialog.expect("a dialog is open");
        assert_eq!(dialog.fields.len(), 1);
        assert_eq!(dialog.fields[0].name, "Name");
        assert_eq!(dialog.fields[0].value, "gain");
    }

    fn with_live(id: u64, text: &str) -> AccessibleProjection {
        AccessibleProjection {
            live: vec![(NodeId(id), text.to_owned())],
            ..Default::default()
        }
    }

    #[test]
    fn what_is_already_on_screen_at_startup_is_not_announced() {
        let mut mirror = MirrorState::default();
        assert_eq!(
            mirror.patch(with_live(9, "Ready"), catalog()).announcement,
            None
        );
        assert_eq!(
            mirror
                .patch(with_live(9, "Compiled"), catalog())
                .announcement
                .as_deref(),
            Some("Compiled")
        );
    }

    #[test]
    fn a_new_live_node_is_announced_even_with_the_same_words() {
        let mut mirror = MirrorState::default();
        mirror.patch(with_live(9, "Ready"), catalog());
        assert_eq!(
            mirror
                .patch(with_live(10, "Ready"), catalog())
                .announcement
                .as_deref(),
            Some("Ready")
        );
    }

    #[test]
    fn a_frame_that_changes_nothing_writes_nothing() {
        let mut mirror = MirrorState::default();
        let projection = || {
            let mut seen = with_live(9, "Ready");
            seen.focused = Some((
                NodeId(2),
                MirroredControl {
                    role: Role::Button,
                    name: "Run".to_owned(),
                    value: String::new(),
                    description: String::new(),
                    toggled: None,
                    disabled: false,
                    read_only: false,
                },
            ));
            seen
        };
        assert!(!mirror.patch(projection(), catalog()).is_empty());
        assert!(mirror.patch(projection(), catalog()).is_empty());
    }

    fn focus_on(
        id: u64,
        role: Role,
        value: &str,
        toggled: Option<Toggled>,
    ) -> AccessibleProjection {
        AccessibleProjection {
            focused: Some((
                NodeId(id),
                MirroredControl {
                    role,
                    name: "Field".to_owned(),
                    value: value.to_owned(),
                    description: String::new(),
                    toggled,
                    disabled: false,
                    read_only: false,
                },
            )),
            ..Default::default()
        }
    }

    #[test]
    fn moving_focus_speaks_the_new_control() {
        let mut mirror = MirrorState::default();
        let patch = mirror.patch(focus_on(2, Role::Button, "", None), catalog());
        assert_eq!(patch.focus_announcement.as_deref(), Some("Field, button"));
    }

    #[test]
    fn typing_in_a_field_is_not_spoken_but_changing_a_checkbox_is() {
        let mut mirror = MirrorState::default();
        mirror.patch(focus_on(2, Role::TextInput, "a", None), catalog());
        let typed = mirror.patch(focus_on(2, Role::TextInput, "ab", None), catalog());
        assert_eq!(typed.focus_announcement, None);
        assert!(
            typed.focused.is_some(),
            "the mirror still follows the value"
        );

        let mut mirror = MirrorState::default();
        mirror.patch(
            focus_on(2, Role::CheckBox, "", Some(Toggled::False)),
            catalog(),
        );
        let toggled = mirror.patch(
            focus_on(2, Role::CheckBox, "", Some(Toggled::True)),
            catalog(),
        );
        assert_eq!(
            toggled.focus_announcement.as_deref(),
            Some("Field, check box, checked")
        );
    }

    #[test]
    fn a_dialog_opening_says_so() {
        let mut mirror = MirrorState::default();
        let open = AccessibleProjection {
            dialog: Some(MirroredDialog {
                title: "New Project".to_owned(),
                description: String::new(),
                fields: Vec::new(),
            }),
            ..Default::default()
        };
        assert_eq!(
            mirror.patch(open, catalog()).announcement.as_deref(),
            Some("New Project, dialog")
        );
    }
}
