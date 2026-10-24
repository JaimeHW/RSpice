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
//! host for it and keeps drawing it.
//!
//! A point is keyed by the host's workspace type, which this module does not
//! name: it names nothing of the workbench it frames.

use egui::Ui;

/// A place in the workbench a container leaves to an owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Point<W> {
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

/// The shell's side of the contribution points.
pub(crate) trait Host {
    /// The workspaces a point is keyed by.
    type WorkspaceKey;

    /// Draw, in `ui`, what the owner of `point` contributes there.
    fn contribute(&mut self, point: Point<Self::WorkspaceKey>, ui: &mut Ui);

    /// What the navigator container draws around `workspace`'s navigator.
    fn navigator_section(&self, workspace: Self::WorkspaceKey) -> NavigatorSection;

    /// The inspector section that answers while `workspace` is showing.
    fn inspector_scope(&self, workspace: Self::WorkspaceKey) -> InspectorScope<Self::WorkspaceKey>;

    /// The heading the inspector container draws above the section `scope`.
    fn inspector_title(&self, scope: InspectorScope<Self::WorkspaceKey>) -> String;
}
