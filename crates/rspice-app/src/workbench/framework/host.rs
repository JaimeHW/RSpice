//! Contribution points: the places a container leaves to whoever owns what is
//! drawn there, and the host that answers for them.
//!
//! A container draws its own frame — a heading, a filter field, a scroll area
//! — and hands the `Ui` it is drawing in to the host at the point where an
//! owner's content goes. The host resolves the point to its owner, so the
//! container names no owner, and the owner draws in that same `Ui`: no scope,
//! id or scroll area comes between them, so egui ids and the memory keyed by
//! them stay where they were.
//!
//! What a container draws around a point can depend on the owner's own state,
//! such as the heading of the page that is showing. The container asks the
//! host for it and keeps drawing it. Where an owner supplies data rather than
//! drawing — a tab list, a title, a badge, a status cell — the container asks
//! the host for that data and keeps measuring and painting it.
//!
//! A point is keyed by the host's own types, which this module does not name:
//! it names nothing of the workbench it frames.
//!
//! A container that offers commands — a menu, the palette, a toolbar button,
//! a navigation sheet — holds each by its key and asks the host what the
//! command is called, whether it can run, and to run it. The menus it draws
//! are the host's tables, handed over the same way.

use std::borrow::Cow;

use egui::Ui;
use egui::os::OperatingSystem;
use rspice_ui_kit::panels::WorkbenchIcon;

use super::command_registry::Registry;
use crate::state::command_catalog::{CommandKey, CommandPlatform};
use crate::ui::icons::Icon;
use crate::ui::widgets::Problem;

/// A place in the workbench a container leaves to an owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Point<W, S, L> {
    /// The stage of a workspace: the surface the current route opens, or else
    /// the workspace's own page.
    Surface(W),
    /// The results pane beside the stage while the results split is showing.
    SplitResults,
    /// A workspace's navigator, below the heading and filter field the
    /// container draws for it.
    Navigator(W),
    /// The inspector's body, inside the scroll area the container keeps.
    Inspector(InspectorScope<W>),
    /// A slot of the context toolbar, drawn to the chrome metrics the shell
    /// resolved for the frame.
    Toolbar(ToolbarSlot<W, S>, L),
}

/// Which inspector section answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InspectorScope<W> {
    /// A section that answers in whatever workspace is showing. While it
    /// applies it comes before every workspace's own section.
    AnyWorkspace,
    /// The workspace's own section.
    Workspace(W),
}

/// A slot of the context toolbar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ToolbarSlot<W, S> {
    /// The lane between the document tools and the trailing group, which
    /// scrolls when its tools overflow it.
    Tools(ToolbarScope<W, S>),
    /// The trailing group's controls before the inspector toggle, while the
    /// container shows the simulation context.
    Trailing,
}

/// Whose tools the toolbar's lane shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ToolbarScope<W, S> {
    /// No project is open.
    NoProject,
    /// The current route opens a surface that brings tools of its own, in
    /// place of its workspace's.
    Surface(S),
    /// The workspace's own tools.
    Workspace(W),
}

/// What the navigator container draws around a workspace's navigator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NavigatorSection {
    /// The panel heading.
    pub(crate) title: &'static str,
    /// What the container's filter field shows while it is empty.
    pub(crate) placeholder: &'static str,
    /// Whether the container draws the filter field above the navigator. A
    /// navigator that filters by facets of its own draws its own.
    pub(crate) container_draws_search: bool,
}

/// What a workspace's owner furnishes its activity-rail button with.
pub(crate) struct RailEntry<H: ?Sized> {
    /// The count the button's badge shows, given whether the workspace is the
    /// one showing.
    pub(crate) badge: Option<fn(&H, bool) -> usize>,
    /// Run on every frame the workspace shows, and when its button is pressed.
    pub(crate) on_visible: Option<fn(&mut H)>,
}

/// Which end of the status bar a cell sits at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StatusZone {
    /// Laid out from the left edge.
    Leading,
    /// Laid out from the right edge inward.
    Trailing,
}

/// What a status cell reports. The bar lays a cell out by what it reports:
/// the dense metadata gives way on a narrow bar, and the prompt takes only the
/// room the other cells leave.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StatusKey {
    /// The answer to the reader's last gesture.
    Prompt,
    /// What the checks found.
    Checks,
    /// Where the reader is in what the workspace shows.
    Context,
    /// What is selected.
    Selection,
    /// The page a multi-sheet drawing is open at.
    Page,
    /// The magnification of the canvas or plot showing.
    Zoom,
    /// What the simulation engine is doing.
    Engine,
    /// Where the application runs.
    Platform,
}

/// The mark in front of a status cell's text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StatusMark {
    None,
    /// A warning or a failure the reader has to notice.
    Problem(Problem),
    /// A run in progress: the bar's one live indicator.
    Live,
    Processor,
}

/// How a status cell answers the pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StatusControl {
    /// It only states.
    Label,
    /// It is a button.
    Button,
    /// It stays a button while it cannot act, drawn disabled.
    Guarded { enabled: bool },
}

/// What a status cell shows on a frame.
pub(crate) struct StatusText<'a> {
    pub(crate) text: Cow<'a, str>,
    pub(crate) mark: StatusMark,
    pub(crate) control: StatusControl,
    /// The tooltip. Over a guarded cell that cannot act it says why.
    pub(crate) hover: Option<Cow<'a, str>>,
}

impl<'a> StatusText<'a> {
    /// A cell that only states `text`.
    pub(crate) fn label(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            mark: StatusMark::None,
            control: StatusControl::Label,
            hover: None,
        }
    }
}

/// A status cell, as the host lists it.
pub(crate) struct StatusCell<H: Host + ?Sized> {
    /// What the cell reports.
    pub(crate) key: StatusKey,
    /// The end of the bar it sits at.
    pub(crate) zone: StatusZone,
    /// Its place in the zone, counted from the zone's own edge.
    pub(crate) order: usize,
    /// The one workspace it reports in, or `None` for every workspace.
    pub(crate) workspace: Option<H::WorkspaceKey>,
    /// What it shows this frame, or `None` to leave its place to the next
    /// cell listed for it. A cell only reads; what changes on a frame is
    /// worked out once, into the frame value.
    pub(crate) content: for<'a> fn(&H, &'a H::StatusFrame) -> Option<StatusText<'a>>,
    /// What pressing it does.
    pub(crate) activate: Option<fn(&mut H, &H::StatusFrame)>,
}

/// The shell's side of the contribution points.
pub(crate) trait Host: 'static {
    /// The workspaces a point is keyed by.
    type WorkspaceKey: Copy + PartialEq + 'static;
    /// The routes whose surface brings tools of its own.
    type SurfaceKey;
    /// The chrome metrics the shell resolves for a frame.
    type Layout;
    /// What the status cells read in common on a frame.
    type StatusFrame: 'static;
    /// Whether a command can run here, and why not.
    type Availability: Copy;
    /// A menu the title bar offers by name.
    type Menu: Copy;
    /// One entry of a menu, as the host writes its menus down.
    type MenuEntry: 'static;

    /// Draw, in `ui`, what the owner of `point` contributes there.
    fn contribute(
        &mut self,
        point: Point<Self::WorkspaceKey, Self::SurfaceKey, Self::Layout>,
        ui: &mut Ui,
    );

    /// What the navigator container draws around `workspace`'s navigator.
    fn navigator_section(&self, workspace: Self::WorkspaceKey) -> NavigatorSection;

    /// The inspector section that answers while `workspace` is showing.
    fn inspector_scope(&self, workspace: Self::WorkspaceKey) -> InspectorScope<Self::WorkspaceKey>;

    /// The heading the inspector container draws above the section `scope`.
    fn inspector_title(&self, scope: InspectorScope<Self::WorkspaceKey>) -> String;

    /// Whose tools the toolbar's lane shows while `workspace` is showing.
    fn toolbar_scope(
        &self,
        workspace: Self::WorkspaceKey,
    ) -> ToolbarScope<Self::WorkspaceKey, Self::SurfaceKey>;

    /// What the title bar names while `workspace` shows an open project.
    fn title_cell(&self, workspace: Self::WorkspaceKey) -> String;

    /// What `workspace`'s owner furnishes its activity-rail button with.
    fn rail_entry(&self, workspace: Self::WorkspaceKey) -> RailEntry<Self>;

    /// What the status cells read in common this frame, worked out before
    /// any of them is asked: what a frame changes, such as a prompt that
    /// expires, changes here.
    fn status_frame(&mut self, ctx: &egui::Context) -> Self::StatusFrame;

    /// Every status cell, in the order each zone's places are offered.
    fn status_cells(&self) -> &'static [StatusCell<Self>];

    /// Every command the host registers, in the order it lists them.
    fn command_registry(&self) -> &'static Registry;

    /// Whether the command `key` names can run here, and why not.
    fn availability(&self, key: CommandKey) -> Self::Availability;

    /// Whether the command `key` names would run if raised now.
    fn is_enabled(&self, key: CommandKey) -> bool;

    /// Whether the state the command `key` names switches is on; `None` for
    /// a command that switches none.
    fn checked(&self, key: CommandKey) -> Option<bool>;

    /// What the command `key` names is called.
    fn label(&self, key: CommandKey) -> &'static str;

    /// The keys the reader's profile binds the command `key` names to on
    /// `platform`, written as `os` writes them; empty for none.
    fn shortcut_label(
        &self,
        key: CommandKey,
        platform: CommandPlatform,
        os: OperatingSystem,
    ) -> String;

    /// Carry out the command `key` names, behind the prologue every command
    /// goes through. A key no command answers to does nothing.
    fn execute(&mut self, key: CommandKey);

    /// The icon a menu row of the command `key` names wears: the one its
    /// toolbar button wears, for a command that has a button.
    fn command_icon(&self, key: CommandKey) -> Option<WorkbenchIcon>;

    /// The glyph the palette gives the command `key` names, where it has
    /// one of its own rather than its scope's.
    fn palette_icon(&self, key: CommandKey) -> Option<Icon>;

    /// The command that opens `workspace`.
    fn open_workspace(&self, workspace: Self::WorkspaceKey) -> CommandKey;

    /// What the application menu `menu` holds.
    fn menu_entries(&self, menu: Self::Menu) -> &'static [Self::MenuEntry];

    /// What the compact layout's one Menu button holds.
    fn compact_menu(&self) -> &'static [Self::MenuEntry];

    /// What the source editor's right-click menu holds.
    fn source_editor_menu(&self) -> &'static [Self::MenuEntry];

    /// Whether the commands the host offers hold together: the registry's
    /// own check, with every key its menus and the framework name a command
    /// by.
    fn validate_commands(&self) -> Result<(), String>;
}

/// The shell's side of the document bar, answered over the session the bar
/// and its commands are handed.
pub(crate) trait DocumentHost {
    /// The workspaces documents belong to.
    type WorkspaceKey;
    /// What names a document.
    type DocumentId;
    /// A document as its tab shows it.
    type Document;

    /// The documents `workspace` offers as tabs, its root first.
    fn workspace_documents(&self, workspace: Self::WorkspaceKey) -> Vec<Self::Document>;

    /// The document `workspace`'s own page shows, which outranks the window's
    /// record of the active document.
    fn page_document(&self, workspace: Self::WorkspaceKey) -> Option<Self::DocumentId>;

    /// Bring `document` forward in its owner; whether it now shows.
    fn activate_owned_document(&mut self, document: &Self::DocumentId) -> bool;

    /// What `document`'s owner does as its tab closes: `Some` with whether it
    /// closed, when the owner closes the document itself; `None` when the bar
    /// is to close the tab.
    fn close_owned_document(&mut self, document: &Self::DocumentId) -> Option<bool>;
}
