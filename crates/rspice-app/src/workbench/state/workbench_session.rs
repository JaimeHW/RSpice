//! The workbench's own session: the route and its history, the docks and
//! their layout per workspace, the documents each workspace shows, and the
//! transient state of the workbench's parts — the palette, its dialogs, the
//! drawers, the notification centre and the requests they raise.
//!
//! None of it is engineering data. The session wire in
//! `app_state::session::wire` names every field it keeps.

use std::collections::{HashMap, HashSet};

use egui::Id;
use rspice_app_state::workbench::preferences::WorkspacePreset;
use rspice_app_state::workbench::workspace::Workspace;
use serde::{Deserialize, Serialize};

use super::console_views::{
    ConsoleView, InlineEdit, NotificationFilter, SpecialistToolBrowserState,
};
use super::navigator_tree::NavigatorTrees;
use super::retired_documents;
use crate::ui::panels::WorkbenchIcon;
use crate::workbench::localization::{MessageCatalog, UiTextLocale};
use crate::workbench::routing::availability::SurfaceRouteUnavailable;
#[cfg(any(test, target_arch = "wasm32"))]
use crate::workbench::routing::navigation::BrowserHistoryEffect;
use crate::workbench::routing::navigation::{
    RouteTransition, RouteTransitionSource, SurfaceNavigation,
};
use crate::workbench::routing::surface_catalog::SurfaceId;
use crate::workbench::routing::surface_route::SurfaceRoute;

const NAVIGATION_SCHEMA_VERSION: u8 = 1;
/// Device-local shell geometry schema.
///
/// Version 1 adopts the redesigned workbench's responsive dock proportions.
/// Older sessions may contain explicit widths captured from the retired shell;
/// those values must not distort the redesigned schematic workspace forever.
pub(super) const LAYOUT_SCHEMA_VERSION: u8 = 1;

/// Device-local dock composition retained independently for each workspace.
/// Engineering documents and project state never enter this presentation
/// snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WorkspaceLayoutState {
    pub navigator_visible: bool,
    pub inspector_visible: bool,
    pub console_visible: bool,
    pub focus_mode: bool,
    pub navigator_width: f32,
    pub navigator_width_custom: bool,
    pub inspector_width: f32,
    pub inspector_width_custom: bool,
    pub console_height: f32,
}

impl Default for WorkspaceLayoutState {
    fn default() -> Self {
        Self {
            navigator_visible: true,
            inspector_visible: true,
            console_visible: false,
            focus_mode: false,
            navigator_width: default_navigator_width(),
            navigator_width_custom: false,
            inspector_width: default_inspector_width(),
            inspector_width_custom: false,
            console_height: default_console_height(),
        }
    }
}

impl WorkspaceLayoutState {
    #[must_use]
    pub fn for_preset(preset: WorkspacePreset) -> Self {
        let mut layout = Self::default();
        match preset {
            WorkspacePreset::Engineering => {}
            WorkspacePreset::Canvas => layout.focus_mode = true,
            WorkspacePreset::Diagnostics => {
                layout.console_visible = true;
                layout.console_height = 260.0;
            }
        }
        layout
    }
}

/// Stable identity of one document presentation in the canonical workspaces.
///
/// These IDs point only at authoritative project objects. Closing a tab
/// removes its presentation from the local session; it never deletes the
/// cell view, configured analysis, or immutable result dataset behind it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkspaceDocumentId {
    Project,
    CellView(crate::state::CellViewRef),
    SimulationPlan,
    ResultDataset(crate::product::DatasetId),
    VisualizationDocument(crate::product::ResultDocumentId),
    Verification,
    Models,
    NetlistGenerated(crate::state::NetlistDocumentId),
    NetlistOwned(crate::state::NetlistDocumentId),
    NetlistDependency {
        root: crate::state::NetlistDocumentId,
        logical_identity: String,
    },
    NetlistComparison,
    /// The read-only deck a completed manual run consumed. There is one at a
    /// time because one run at a time owns the deck baseline.
    NetlistRunSnapshot,
    /// Legacy single-document session identity retained so older device-local
    /// workbench snapshots continue to deserialize. New netlist sessions use
    /// the stable document-specific variants above.
    NetlistSource,
}

impl WorkspaceDocumentId {
    pub const fn workspace(&self) -> Workspace {
        match self {
            Self::Project => Workspace::Project,
            Self::CellView(_) => Workspace::Design,
            Self::SimulationPlan => Workspace::Simulate,
            Self::ResultDataset(_) | Self::VisualizationDocument(_) => Workspace::Results,
            Self::Verification => Workspace::Verify,
            Self::Models => Workspace::Models,
            Self::NetlistGenerated(_)
            | Self::NetlistOwned(_)
            | Self::NetlistDependency { .. }
            | Self::NetlistComparison
            | Self::NetlistRunSnapshot
            | Self::NetlistSource => Workspace::Netlist,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WorkspaceDocument {
    pub(in crate::workbench) id: WorkspaceDocumentId,
    pub(in crate::workbench) label: String,
    pub(in crate::workbench) icon: WorkbenchIcon,
    pub(in crate::workbench) dirty: bool,
}

impl WorkspaceDocument {
    /// What the tab shows when it is too narrow for its label: a cell view's
    /// cell alone, since the tab's icon already names the view.
    pub(in crate::workbench) fn short_label(&self) -> Option<&str> {
        match &self.id {
            WorkspaceDocumentId::CellView(reference) => Some(reference.cell.as_str()),
            _ => None,
        }
    }
}

/// Device-local open-document session. Engineering content continues to live
/// in its owning project models; this registry retains only active/closed tab
/// presentation state for each workspace.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceDocumentRegistry {
    // Read through `retired_documents`: this registry is where a retired
    // document identity is most likely to be sitting, because activating one
    // wrote it here.
    #[serde(
        default,
        deserialize_with = "retired_documents::deserialize_active_map"
    )]
    active: HashMap<Workspace, WorkspaceDocumentId>,
    #[serde(default, deserialize_with = "retired_documents::deserialize_set")]
    closed: HashSet<WorkspaceDocumentId>,
}

impl WorkspaceDocumentRegistry {
    pub fn active(&self, workspace: Workspace) -> Option<&WorkspaceDocumentId> {
        self.active.get(&workspace)
    }

    pub fn activate(&mut self, document: WorkspaceDocumentId) {
        let workspace = document.workspace();
        self.closed.remove(&document);
        self.active.insert(workspace, document);
    }

    pub fn close(&mut self, document: &WorkspaceDocumentId) {
        self.closed.insert(document.clone());
        if self.active.get(&document.workspace()) == Some(document) {
            self.active.remove(&document.workspace());
        }
    }

    pub fn is_closed(&self, document: &WorkspaceDocumentId) -> bool {
        self.closed.contains(document)
    }

    pub fn retain_available(
        &mut self,
        workspace: Workspace,
        available: impl IntoIterator<Item = WorkspaceDocumentId>,
    ) {
        let available = available.into_iter().collect::<HashSet<_>>();
        self.closed
            .retain(|document| document.workspace() != workspace || available.contains(document));
        if self
            .active
            .get(&workspace)
            .is_some_and(|document| !available.contains(document))
        {
            self.active.remove(&workspace);
        }
    }
}

/// Width class used by layout composition.  Breakpoints are based on task
/// density rather than an assumed device or operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthClass {
    Phone,
    Tablet,
    Desktop,
    Wide,
}

impl WidthClass {
    pub const fn for_width(width: f32) -> Self {
        if width <= 560.0 {
            Self::Phone
        } else if width <= 820.0 {
            Self::Tablet
        } else if width <= 1260.0 {
            Self::Desktop
        } else {
            Self::Wide
        }
    }

    pub const fn uses_bottom_navigation(self) -> bool {
        matches!(self, Self::Phone | Self::Tablet)
    }

    pub const fn inspector_uses_drawer(self) -> bool {
        !matches!(self, Self::Wide)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Drawer {
    Navigator,
    Inspector,
    Workspaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ConsolePage {
    /// The alias restores a session last left on the retired Interactive page
    /// onto the engine log, rather than failing the whole workbench restore
    /// on a page name this build no longer has.
    #[default]
    #[serde(alias = "Interactive")]
    Console,
    Problems,
    Measurements,
    TaskLog,
}

impl ConsolePage {
    pub const ALL: [Self; 4] = [
        Self::Console,
        Self::Problems,
        Self::Measurements,
        Self::TaskLog,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Console => "Console",
            Self::Problems => "Problems",
            Self::Measurements => "Measurements",
            Self::TaskLog => "Task log",
        }
    }
}

/// The status bar's answer to the gesture just made.
///
/// Feedback, not a record: it is never logged and never notified, the next
/// prompt replaces it, and it is gone [`StatusPrompt::LIFETIME`] after it was
/// given.
#[derive(Debug, Clone)]
pub struct StatusPrompt {
    pub text: String,
    pub issued: crate::time_compat::Instant,
}

impl StatusPrompt {
    pub const LIFETIME: std::time::Duration = std::time::Duration::from_secs(6);

    /// How long the prompt has left on the bar, or `None` once it has expired.
    pub fn remaining(&self) -> Option<std::time::Duration> {
        Self::LIFETIME
            .checked_sub(self.issued.elapsed())
            .filter(|remaining| !remaining.is_zero())
    }
}

/// The source editor a page shows as its document, as the application's
/// menus, palette and shortcuts address it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ActiveSourceEditor {
    pub id: Id,
    pub editable: bool,
    /// Its language service answers go to definition, find references and
    /// rename for this document.
    pub navigable: bool,
}

/// New workbench session state. Durable layout preferences are kept with the
/// session, through the wire in `app_state::session::wire`, which names every
/// field it keeps; one-frame requests and open drawers are intentionally
/// transient.
#[derive(Debug, Clone)]
pub struct WorkbenchSession {
    pub workspace: Workspace,
    /// Device-local application-window presentations and their exclusive
    /// document ownership. Engineering documents remain owned by their
    /// project stores; this registry only controls where each is presented.
    pub window_session: crate::workbench::lifecycle::window_session::WindowSessionRegistry,
    // An `engineering_profile` key from older sessions is ignored: the
    // profile only ever narrowed one specialist tool list and is retired.
    /// Version marker used to distinguish sessions that predate canonical
    /// routes from sessions whose current route is intentionally Design.
    pub(in crate::workbench) navigation_schema_version: u8,
    /// Version marker for persisted device-local dock geometry. This is kept
    /// independent from navigation because a visual-shell migration must not
    /// rewrite the user's current route or engineering state.
    pub(in crate::workbench) layout_schema_version: u8,
    /// Canonical current route plus bounded back/forward/recent task history.
    pub(in crate::workbench) navigation: SurfaceNavigation,
    /// Recovery or deep-link diagnostic awaiting presentation in the activity
    /// stream. It is runtime state, never engineering or preference data.
    pub(in crate::workbench) route_diagnostic: Option<String>,
    pub navigator_visible: bool,
    pub inspector_visible: bool,
    pub console_visible: bool,
    pub console_maximized: bool,
    pub focus_mode: bool,
    /// Device-local presentation toggle for the mockup's equal schematic /
    /// result stage. The immutable datasets and result-document state remain
    /// owned by `SimulationState` and `UiSessionState::results`; this flag
    /// only controls whether that existing document is projected beside an
    /// eligible primary workspace.
    pub split_with_results: bool,
    /// Current viewport full-screen intent. Runtime-owned because the host
    /// window, browser, or mobile shell decides whether the request can be
    /// honored and must never restore a stale platform window state.
    pub full_screen: bool,
    /// RSpice-owned presentation state. Unlike `full_screen`, which mirrors
    /// what the host window is doing, this stays true for as long as the
    /// application chrome is intentionally suppressed — including while a
    /// browser is still deciding whether to honour the request.
    pub full_screen_presentation: bool,
    /// Capability-derived touch composition. Once a native touch event is
    /// observed it remains enabled for this process; browsers also refresh it
    /// from the exact `(pointer: coarse)` media query every frame.
    pub coarse_pointer: bool,
    pub navigator_width: f32,
    /// True after the user explicitly resizes the navigator. Until then the
    /// mockup's responsive 18vw clamp remains authoritative.
    pub navigator_width_custom: bool,
    pub inspector_width: f32,
    /// True after the user explicitly resizes the inspector. Until then the
    /// mockup's responsive 22vw clamp remains authoritative.
    pub inspector_width_custom: bool,
    pub console_height: f32,
    /// Independent device-local dock snapshots for every visited workspace.
    pub(in crate::workbench) workspace_layouts: HashMap<Workspace, WorkspaceLayoutState>,
    /// The composition a workspace with no snapshot above opens with.
    ///
    /// Mirrored from the Workspace preference, because layout reconciliation
    /// runs deep inside navigation and cannot reach the preference store from
    /// there. Without it an unvisited workspace resolved to the shipped
    /// default and the preference governed only whichever workspace happened
    /// to be active when it was chosen.
    pub(in crate::workbench) workspace_preset: WorkspacePreset,
    pub console_page: ConsolePage,
    /// The reader's place on the Console page. Transient: a reading position
    /// over logs, not a layout preference.
    pub console_view: ConsoleView,
    /// Active and locally closed document presentations in each workspace.
    /// Stable object IDs ensure a reordered run history or analysis plan does
    /// not silently retarget a restored tab.
    pub documents: WorkspaceDocumentRegistry,
    /// The status bar's feedback on the gesture just made.
    pub prompt: Option<StatusPrompt>,
    /// Where each workspace's navigator tree is unfolded, and what its filter
    /// box narrows the listing to. Runtime-only.
    pub navigator_trees: NavigatorTrees,
    pub command_query: String,
    /// The inspector's open inline-edit session. Runtime-only: a partially
    /// typed instance name must never be restored as authoritative design
    /// data, and the retained undo snapshot belongs to this process.
    pub inline_edit: InlineEdit,
    /// The last placement or editing command run, which Repeat last
    /// command runs again. The command dispatcher writes and reads it as
    /// the command's stable id, so the state layer never names the command
    /// vocabulary above it.
    pub last_repeatable_command: Option<String>,
    pub drawer: Option<Drawer>,
    /// Personal specialist discovery metadata plus task-local browser state.
    /// The owned engineering documents remain in their canonical surfaces.
    pub specialist_tool_browser: SpecialistToolBrowserState,
    /// Session activity center. Its records live in `UiSessionState::toasts`;
    /// only this transient presentation state belongs to the workbench.
    pub notification_center_open: bool,
    pub notification_filter: NotificationFilter,
    /// Hide what has been read. A second axis rather than a fourth filter,
    /// because "unread" crosses the Jobs/System split instead of joining it.
    pub notification_unread_only: bool,

    /// Device-local window, document, and session presentation manager.
    pub(crate) window_session_dialog:
        crate::workbench::app::dialogs::window_session::WindowSessionDialogState,

    /// Command palette state
    pub command_palette: crate::workbench::app::command_palette::CommandPaletteState,

    /// Runtime interaction state for drag, hover, etc.
    pub interaction: crate::workbench::app_state::interaction_state::InteractionState,

    /// State for save confirmation modal (unsaved changes warning)
    pub confirmation_dialog:
        crate::workbench::app::dialogs::confirmation_state::ConfirmationDialogState,

    /// Dedicated engineering review for Revert and Close Project. These
    /// destructive ellipsis actions intentionally do not use the generic
    /// Save/Don't save confirmation grammar.
    pub(crate) project_review_dialog:
        crate::workbench::app::dialogs::confirmation_state::ProjectReviewDialogState,

    /// Theme selection (direction / mode / density).
    pub theme: crate::ui::Theme,
    /// Decimal-mark information supplied by the native/web platform edge.
    /// `None` is deliberate: engineering code must not guess the host locale.
    pub number_locale: crate::quantity::UiNumberLocale,
    /// Device-local UI message locale. Engineering source, identifiers, and
    /// portable numeric syntax never consult this presentation setting.
    pub text_locale: UiTextLocale,
    /// Toast queue.
    pub toasts: crate::ui::widgets::Toasts,
    /// The source editor the page on screen shows as its document, published
    /// by that page every frame it draws one. The Edit menu, the palette and
    /// the shortcuts act on it rather than on a schematic hidden behind it.
    pub(crate) active_source_editor: Option<ActiveSourceEditor>,
    /// One-shot requests the parts of the workbench raise for one another,
    /// waiting for the frame point that takes each. Runtime-only: a request
    /// is an intent of this process, never session or project data.
    pub(crate) outbox: crate::workbench::framework::requests::Outbox,
}

pub(in crate::workbench) const fn default_navigator_width() -> f32 {
    256.0
}

pub(in crate::workbench) const fn default_inspector_width() -> f32 {
    312.0
}

/// The console dock's height on a desktop until the reader drags it: the
/// least that shows the Problems page's toolbar, its header, three whole
/// rows and the selected finding's message with the line under it. At the
/// 145 points it was, the page had room for two rows and no detail at all.
pub(crate) const fn default_console_height() -> f32 {
    208.0
}

impl Default for WorkbenchSession {
    fn default() -> Self {
        Self {
            workspace: Workspace::Design,
            window_session:
                crate::workbench::lifecycle::window_session::WindowSessionRegistry::default(),
            navigation_schema_version: NAVIGATION_SCHEMA_VERSION,
            layout_schema_version: LAYOUT_SCHEMA_VERSION,
            navigation: SurfaceNavigation::default(),
            route_diagnostic: None,
            navigator_visible: true,
            inspector_visible: true,
            console_visible: false,
            console_maximized: false,
            focus_mode: false,
            split_with_results: false,
            full_screen: false,
            full_screen_presentation: false,
            coarse_pointer: false,
            navigator_width: default_navigator_width(),
            navigator_width_custom: false,
            inspector_width: default_inspector_width(),
            inspector_width_custom: false,
            console_height: default_console_height(),
            workspace_layouts: HashMap::new(),
            workspace_preset: WorkspacePreset::default(),
            console_page: ConsolePage::Console,
            console_view: ConsoleView::default(),
            documents: WorkspaceDocumentRegistry::default(),
            prompt: None,
            navigator_trees: NavigatorTrees::default(),
            command_query: String::new(),
            inline_edit: InlineEdit::default(),
            last_repeatable_command: None,
            drawer: None,
            specialist_tool_browser: SpecialistToolBrowserState::default(),
            notification_center_open: false,
            notification_filter: NotificationFilter::default(),
            notification_unread_only: false,
            window_session_dialog: Default::default(),
            command_palette: Default::default(),
            interaction: Default::default(),
            confirmation_dialog: Default::default(),
            project_review_dialog: Default::default(),
            theme: Default::default(),
            number_locale: Default::default(),
            text_locale: Default::default(),
            toasts: Default::default(),
            active_source_editor: None,
            outbox: Default::default(),
        }
    }
}

impl WorkbenchSession {
    /// Replace the platform-provided locale used by interactive UI quantity
    /// fields. Portable project and SPICE parsers never read this value.
    pub fn set_number_locale(&mut self, locale: crate::quantity::UiNumberLocale) {
        self.number_locale = locale;
    }

    #[must_use]
    pub(crate) const fn messages(&self) -> MessageCatalog {
        MessageCatalog::new(self.text_locale)
    }
}

impl WorkbenchSession {
    /// Answer the gesture just made, on the status bar, as
    /// [`AppState::prompt`](crate::workbench::app_state::AppState::prompt)
    /// states it.
    pub fn prompt(&mut self, text: impl Into<String>) {
        self.prompt = Some(StatusPrompt {
            text: text.into(),
            issued: crate::time_compat::Instant::now(),
        });
    }

    /// [`Self::prompt`] what an operation that answers either way said,
    /// whether it was done or refused, and hand the outcome on.
    pub(crate) fn report(&mut self, outcome: Result<String, String>) -> Result<String, String> {
        match &outcome {
            Ok(message) | Err(message) => self.prompt(message.clone()),
        }
        outcome
    }

    /// Whether the design surface is showing in the design workspace: the
    /// only place the open sheet is edited. Print and Export own output
    /// media and never stand in for the sheet.
    pub(crate) fn shows_design_surface(&self) -> bool {
        self.workspace == Workspace::Design
            && self.current_route().surface_id() == SurfaceId::Design
    }

    /// Show the bottom dock, at its own height and out of focus mode, on
    /// whichever page it last showed.
    pub(crate) fn open_console(&mut self) {
        self.focus_mode = false;
        self.console_visible = true;
        self.console_maximized = false;
    }

    /// Whether the notification panel is on screen, for either of its owners:
    /// the bell's transient flag, or a route that names the surface (a deep
    /// link, the palette, restored history).
    pub fn notification_center_showing(&self) -> bool {
        self.notification_center_open
            || self.current_route().surface_id() == SurfaceId::NotificationCenter
    }

    /// The bell is a toggle: pressing it again puts the panel away.
    pub fn toggle_notification_center(&mut self) {
        if self.notification_center_showing() {
            self.close_notification_center();
        } else {
            self.notification_center_open = true;
        }
    }

    /// Put the panel away, returning a route-owned panel to where it was
    /// opened from so Back does not reopen it.
    pub fn close_notification_center(&mut self) {
        self.notification_center_open = false;
        if self.current_route().surface_id() != SurfaceId::NotificationCenter
            || self.navigate_back(RouteTransitionSource::User).is_some()
        {
            return;
        }
        let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(self.workspace));
        if let Err(error) = self.replace_route(fallback, RouteTransitionSource::User) {
            self.record_route_diagnostic(format!("Notifications could not be closed: {error}"));
        }
    }

    /// Close a route-owned transient surface without stranding its canonical
    /// URL. Ordinary opens return to their exact source; a direct deep link
    /// replaces itself with the currently projected primary workspace.
    pub(crate) fn close_route_owned_transient(&mut self, surface: SurfaceId, label: &str) {
        if self.current_route().surface_id() != surface {
            return;
        }
        if self.navigate_back(RouteTransitionSource::User).is_some() {
            return;
        }
        let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(self.workspace));
        if let Err(error) = self.replace_route(fallback, RouteTransitionSource::User) {
            self.prompt(format!("Could not close {label}: {error}"));
        }
    }

    pub fn activate(&mut self, workspace: Workspace) {
        if let Err(error) = self.navigate(
            SurfaceRoute::surface(SurfaceId::from_workspace(workspace)),
            RouteTransitionSource::User,
        ) {
            self.record_route_diagnostic(format!(
                "The requested primary workspace could not be opened: {error}"
            ));
        }
    }

    /// Commit one canonical route transition and update the primary-workspace
    /// rendering projection when the destination is a primary surface.
    pub fn navigate(
        &mut self,
        route: SurfaceRoute,
        source: RouteTransitionSource,
    ) -> Result<RouteTransition, SurfaceRouteUnavailable> {
        crate::workbench::routing::availability::require_available(route)?;
        let previous = self.navigation.current();
        self.reconcile_workspace_layout(previous, route);
        let transition = self.navigation.navigate(route, source);
        self.reconcile_specialist_tool_browser_route(transition.previous, transition.current);
        if let Some(workspace) = route.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.navigation_schema_version = NAVIGATION_SCHEMA_VERSION;
        self.close_drawer();
        Ok(transition)
    }

    /// Replace the current route during startup canonicalization without
    /// creating a back entry.
    pub fn replace_route(
        &mut self,
        route: SurfaceRoute,
        source: RouteTransitionSource,
    ) -> Result<(), SurfaceRouteUnavailable> {
        crate::workbench::routing::availability::require_available(route)?;
        let previous = self.navigation.current();
        self.reconcile_workspace_layout(previous, route);
        self.navigation.replace(route, source);
        self.reconcile_specialist_tool_browser_route(previous, route);
        if let Some(workspace) = route.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.navigation_schema_version = NAVIGATION_SCHEMA_VERSION;
        self.close_drawer();
        Ok(())
    }

    /// Reconcile a restored session. Legacy sessions are migrated from the
    /// persisted workspace; current sessions treat the canonical route as the
    /// source of truth and keep the workspace only as a render projection.
    pub fn reconcile_restored_navigation(&mut self) {
        if self.layout_schema_version != LAYOUT_SCHEMA_VERSION {
            self.navigator_width = default_navigator_width();
            self.navigator_width_custom = false;
            self.inspector_width = default_inspector_width();
            self.inspector_width_custom = false;
            for layout in self.workspace_layouts.values_mut() {
                layout.navigator_width = default_navigator_width();
                layout.navigator_width_custom = false;
                layout.inspector_width = default_inspector_width();
                layout.inspector_width_custom = false;
            }
            self.layout_schema_version = LAYOUT_SCHEMA_VERSION;
        }
        if self.navigation.recovered_invalid_routes() {
            self.route_diagnostic = Some(
                "Malformed routes were removed from restored task history; project and document state were preserved."
                    .to_owned(),
            );
            self.navigation.acknowledge_recovery();
        }
        if self.navigation_schema_version != NAVIGATION_SCHEMA_VERSION {
            self.navigation.replace(
                SurfaceRoute::surface(SurfaceId::from_workspace(self.workspace)),
                RouteTransitionSource::Restore,
            );
            self.navigation_schema_version = NAVIGATION_SCHEMA_VERSION;
        } else {
            let current = self.navigation.current();
            if let Err(error) = crate::workbench::routing::availability::require_available(current)
            {
                self.route_diagnostic = Some(format!(
                    "The restored route was not opened because its executor is unavailable: {error}"
                ));
                self.navigation.replace(
                    SurfaceRoute::surface(SurfaceId::from_workspace(self.workspace)),
                    RouteTransitionSource::Restore,
                );
            } else if let Some(workspace) = current.surface_id().owner_workspace() {
                self.workspace = workspace;
            }
        }
        let removed = self.navigation.retain_history(|route| {
            crate::workbench::routing::availability::route_availability(route).can_open()
        });
        if removed && self.route_diagnostic.is_none() {
            self.route_diagnostic = Some(
                "Unavailable routes were removed from restored task history; project and document state were preserved."
                    .to_owned(),
            );
        }
        self.specialist_tool_browser.normalize();
    }

    pub fn record_route_diagnostic(&mut self, diagnostic: impl Into<String>) {
        self.route_diagnostic = Some(diagnostic.into());
    }

    pub fn take_route_diagnostic(&mut self) -> Option<String> {
        self.route_diagnostic.take()
    }

    #[must_use]
    pub const fn current_route(&self) -> SurfaceRoute {
        self.navigation.current()
    }

    pub(crate) fn project_application_window(
        &mut self,
        workspace: Workspace,
        route: SurfaceRoute,
        layout: WorkspaceLayoutState,
    ) {
        self.workspace = workspace;
        self.navigation.project_transient(route);
        self.apply_workspace_layout(layout);
    }

    /// Whether the foreground route is one of the three authoring/setup
    /// stages that the upgraded mockup permits beside a retained result
    /// document. Checking both route and workspace fails closed while a
    /// manager or specialist surface temporarily retains an owner workspace.
    #[must_use]
    pub fn supports_results_split(&self) -> bool {
        matches!(
            (self.workspace, self.current_route().surface_id()),
            (Workspace::Design, SurfaceId::Design)
                | (Workspace::Netlist, SurfaceId::Netlist)
                | (Workspace::Simulate, SurfaceId::Simulate)
        )
    }

    /// Effective split-stage visibility. A remembered presentation choice
    /// never creates a blank or misleading secondary pane after the project
    /// or its retained evidence goes away.
    #[must_use]
    pub fn results_split_visible(&self, project_open: bool, has_retained_result: bool) -> bool {
        self.split_with_results
            && project_open
            && has_retained_result
            && self.supports_results_split()
    }

    /// Immediate in-application return destination, if one exists. The
    /// mutable navigation stacks remain session-owned so surfaces cannot
    /// bypass route availability and reconciliation.
    #[must_use]
    pub fn previous_route(&self) -> Option<SurfaceRoute> {
        self.navigation.back_entries().last().copied()
    }

    /// Number of known in-application predecessors. Only the browser build
    /// reads this: a Back that would leave the in-app task stack is refused.
    #[must_use]
    #[cfg(target_arch = "wasm32")]
    pub fn back_route_count(&self) -> usize {
        self.navigation.back_entries().len()
    }

    /// Only the browser build reads this: `app::apply_browser_history_delta`
    /// refuses a forward jump that would leave the in-app task stack.
    #[must_use]
    #[cfg(target_arch = "wasm32")]
    pub fn forward_route_count(&self) -> usize {
        self.navigation.forward_entries().len()
    }

    pub fn navigate_back(&mut self, source: RouteTransitionSource) -> Option<RouteTransition> {
        let destination = self.navigation.back_entries().last().copied()?;
        self.reconcile_workspace_layout(self.navigation.current(), destination);
        let transition = self.navigation.go_back(source)?;
        self.reconcile_specialist_tool_browser_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.close_drawer();
        Some(transition)
    }

    /// Atomically return across up to `count` known routes, reconciling the
    /// foreground surface only once from the original route to the final
    /// destination. Only the browser build's history pop jumps several
    /// routes at once.
    #[cfg(any(target_arch = "wasm32", test))]
    pub fn navigate_back_steps(
        &mut self,
        count: usize,
        source: RouteTransitionSource,
    ) -> Option<RouteTransition> {
        let available = count.min(self.navigation.back_entries().len());
        let destination = *self.navigation.back_entries().get(
            self.navigation
                .back_entries()
                .len()
                .saturating_sub(available),
        )?;
        self.reconcile_workspace_layout(self.navigation.current(), destination);
        let transition = self.navigation.go_back_steps(count, source)?;
        self.reconcile_specialist_tool_browser_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.close_drawer();
        Some(transition)
    }

    /// Atomically advance across up to `count` known routes, reconciling the
    /// foreground surface only once from the original route to the final
    /// destination.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn navigate_forward_steps(
        &mut self,
        count: usize,
        source: RouteTransitionSource,
    ) -> Option<RouteTransition> {
        let available = count.min(self.navigation.forward_entries().len());
        let destination = *self.navigation.forward_entries().get(
            self.navigation
                .forward_entries()
                .len()
                .saturating_sub(available),
        )?;
        self.reconcile_workspace_layout(self.navigation.current(), destination);
        let transition = self.navigation.go_forward_steps(count, source)?;
        self.reconcile_specialist_tool_browser_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.close_drawer();
        Some(transition)
    }

    fn reconcile_specialist_tool_browser_route(
        &mut self,
        previous: SurfaceRoute,
        current: SurfaceRoute,
    ) {
        let browser = SurfaceId::SpecialistToolBrowser;
        if previous.surface_id() != browser && current.surface_id() == browser {
            self.specialist_tool_browser.normalize();
            self.specialist_tool_browser.focus_search = true;
        }
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn take_browser_history_effect(&mut self) -> Option<BrowserHistoryEffect> {
        self.navigation.take_browser_effect()
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn has_pending_browser_history_effects(&self) -> bool {
        self.navigation.has_pending_browser_effects()
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn clear_browser_history_effects(&mut self) {
        self.navigation.clear_browser_effects();
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn take_browser_history_effect_queue_overflowed(&mut self) -> bool {
        self.navigation.take_browser_effect_queue_overflowed()
    }

    /// Drop restored traversal stacks when a new browser process takes
    /// ownership of the current URL. The active route and workspace remain
    /// unchanged; only host-history authority is reset.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn reset_navigation_history_for_fresh_browser_session(&mut self) {
        self.navigation.reset_history();
    }

    /// Apply a reviewed workspace preset immediately and retain that exact
    /// composition for the active workspace.
    pub fn apply_workspace_preset(&mut self, preset: WorkspacePreset) {
        self.apply_workspace_layout(WorkspaceLayoutState::for_preset(preset));
        self.capture_workspace_layout(self.workspace);
    }

    /// Apply the console launch choice immediately. Errors and explicit
    /// Problems navigation may still open it later through their own owners.
    pub fn apply_console_launch_behavior(&mut self, open: bool) {
        self.focus_mode = false;
        self.console_visible = open;
        self.console_maximized = false;
        self.capture_workspace_layout(self.workspace);
    }

    #[must_use]
    pub fn workspace_layout(&self, workspace: Workspace) -> WorkspaceLayoutState {
        if workspace == self.workspace {
            return self.current_workspace_layout();
        }
        self.retained_or_preset_layout(workspace)
    }

    /// What `workspace` opens with: its own retained snapshot if it has been
    /// visited, otherwise the composition the preference asks new ones for.
    fn retained_or_preset_layout(&self, workspace: Workspace) -> WorkspaceLayoutState {
        self.workspace_layouts
            .get(&workspace)
            .copied()
            .unwrap_or_else(|| WorkspaceLayoutState::for_preset(self.workspace_preset))
    }

    /// Record the composition unvisited workspaces open with.
    ///
    /// This only sets the fallback. Workspaces the reader has already arranged
    /// keep their own snapshots, so choosing a preset never discards work;
    /// `apply_preset_to_all_workspaces` is the surface that does that, and it
    /// says so.
    pub fn set_workspace_preset(&mut self, preset: WorkspacePreset) {
        self.workspace_preset = preset;
    }

    pub fn apply_preset_to_all_workspaces(&mut self, preset: WorkspacePreset) {
        let layout = WorkspaceLayoutState::for_preset(preset);
        self.workspace_preset = preset;
        for workspace in Workspace::ALL {
            self.workspace_layouts.insert(workspace, layout);
        }
        self.apply_workspace_layout(layout);
    }

    /// Apply the mockup's results-review composition: wide plot/inspector,
    /// hidden navigator, and collapsed console.
    pub fn apply_results_review_layout(&mut self) {
        let layout = WorkspaceLayoutState {
            navigator_visible: false,
            inspector_visible: true,
            console_visible: false,
            focus_mode: false,
            inspector_width: 332.0,
            inspector_width_custom: true,
            ..WorkspaceLayoutState::default()
        };
        self.apply_workspace_layout(layout);
        self.capture_workspace_layout(self.workspace);
    }

    pub fn apply_results_review_to_all_workspaces(&mut self) {
        let layout = WorkspaceLayoutState {
            navigator_visible: false,
            inspector_visible: true,
            console_visible: false,
            focus_mode: false,
            inspector_width: 332.0,
            inspector_width_custom: true,
            ..WorkspaceLayoutState::default()
        };
        for workspace in Workspace::ALL {
            self.workspace_layouts.insert(workspace, layout);
        }
        self.apply_workspace_layout(layout);
    }

    pub(crate) fn current_workspace_layout(&self) -> WorkspaceLayoutState {
        WorkspaceLayoutState {
            navigator_visible: self.navigator_visible,
            inspector_visible: self.inspector_visible,
            console_visible: self.console_visible,
            focus_mode: self.focus_mode,
            navigator_width: self.navigator_width,
            navigator_width_custom: self.navigator_width_custom,
            inspector_width: self.inspector_width,
            inspector_width_custom: self.inspector_width_custom,
            console_height: self.console_height,
        }
    }

    fn capture_workspace_layout(&mut self, workspace: Workspace) {
        let layout = self.current_workspace_layout();
        self.workspace_layouts.insert(workspace, layout);
    }

    pub(crate) fn apply_workspace_layout(&mut self, layout: WorkspaceLayoutState) {
        self.navigator_visible = layout.navigator_visible;
        self.inspector_visible = layout.inspector_visible;
        self.console_visible = layout.console_visible;
        self.console_maximized = false;
        self.focus_mode = layout.focus_mode;
        self.navigator_width = layout.navigator_width;
        self.navigator_width_custom = layout.navigator_width_custom;
        self.inspector_width = layout.inspector_width;
        self.inspector_width_custom = layout.inspector_width_custom;
        self.console_height = layout.console_height;
        self.close_drawer();
    }

    fn reconcile_workspace_layout(&mut self, previous: SurfaceRoute, current: SurfaceRoute) {
        let previous_workspace = previous.surface_id().owner_workspace();
        let current_workspace = current.surface_id().owner_workspace();
        if previous_workspace == current_workspace {
            return;
        }
        if let Some(previous_workspace) = previous_workspace {
            self.capture_workspace_layout(previous_workspace);
        }
        if let Some(current_workspace) = current_workspace {
            let layout = self.retained_or_preset_layout(current_workspace);
            self.apply_workspace_layout(layout);
        }
    }

    pub fn toggle_drawer(&mut self, drawer: Drawer) {
        if self.drawer == Some(drawer) {
            self.close_drawer();
        } else {
            self.drawer = Some(drawer);
        }
    }

    pub fn close_drawer(&mut self) {
        self.drawer = None;
    }

    /// Dismiss the navigator's current presentation. Closing a compact
    /// drawer must not overwrite the saved desktop dock preference.
    pub fn dismiss_navigator(&mut self) {
        if self.drawer == Some(Drawer::Navigator) {
            self.close_drawer();
        } else {
            self.navigator_visible = false;
        }
    }

    /// Dismiss the inspector's current presentation. Closing a compact
    /// drawer must not overwrite the saved desktop dock preference.
    pub fn dismiss_inspector(&mut self) {
        if self.drawer == Some(Drawer::Inspector) {
            self.close_drawer();
        } else {
            self.inspector_visible = false;
        }
    }

    /// Clear a transient drawer as soon as the current responsive composition
    /// cannot present it. Navigator and inspector intentionally have distinct
    /// 821-1260 px behavior in the mockup.
    pub fn reconcile_drawer_mode(
        &mut self,
        navigator_uses_drawer: bool,
        inspector_uses_drawer: bool,
        workspaces_uses_drawer: bool,
    ) {
        let supported = match self.drawer {
            Some(Drawer::Navigator) => navigator_uses_drawer,
            Some(Drawer::Inspector) => inspector_uses_drawer,
            Some(Drawer::Workspaces) => workspaces_uses_drawer,
            None => true,
        };
        if !supported {
            self.close_drawer();
        }
    }

    pub fn reset_layout(&mut self) {
        self.navigator_visible = true;
        self.inspector_visible = true;
        self.console_visible = false;
        self.console_maximized = false;
        self.focus_mode = false;
        self.navigator_width = default_navigator_width();
        self.navigator_width_custom = false;
        self.inspector_width = default_inspector_width();
        self.inspector_width_custom = false;
        self.console_height = default_console_height();
        self.close_drawer();
        self.capture_workspace_layout(self.workspace);
    }
}
