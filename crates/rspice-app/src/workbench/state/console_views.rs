//! Transient views the workbench keeps over its own records: which log the
//! Console page shows and what the reader selected in it, the specialist
//! tool browser, the notification centre's filter, and the inspector's
//! inline edit. None of it is durable engineering data.

use serde::{Deserialize, Serialize};

use crate::workbench::routing::surface_catalog::{SurfaceArchetype, SurfaceId};

/// Which log the Console page shows.
///
/// The session's log holds studio events and one start and one end line per
/// run; a run's own log holds everything that run wrote. The reader chooses
/// between them and nothing chooses for them: a run starting never takes the
/// page away from the log being read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConsoleChannel {
    #[default]
    Session,
    /// One run's own log, by the run's display sequence.
    Run(u64),
}

/// The Console page's three severity toggles.
///
/// Debug and trace lines count as Info: they are notes, and a fourth toggle
/// would be a control for lines most runs never write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleSeverity {
    Errors,
    Warnings,
    Info,
}

impl ConsoleSeverity {
    pub const ALL: [Self; 3] = [Self::Errors, Self::Warnings, Self::Info];

    pub const fn of(severity: crate::diagnostics::LogSeverity) -> Self {
        use crate::diagnostics::LogSeverity;
        match severity {
            LogSeverity::Error => Self::Errors,
            LogSeverity::Warning => Self::Warnings,
            LogSeverity::Info | LogSeverity::Debug | LogSeverity::Trace => Self::Info,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Errors => "Errors",
            Self::Warnings => "Warnings",
            Self::Info => "Info",
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Errors => 0,
            Self::Warnings => 1,
            Self::Info => 2,
        }
    }
}

/// The selected lines of a channel: every shown line from the one the
/// selection started on to the one it was extended to, both included.
///
/// Lines are addressed by the channel's own keys, which only grow down the
/// log, so the range survives lines arriving under it and a filter hiding
/// part of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsoleSelection {
    pub anchor: u64,
    pub cursor: u64,
}

impl ConsoleSelection {
    pub const fn single(key: u64) -> Self {
        Self {
            anchor: key,
            cursor: key,
        }
    }

    pub fn contains(self, key: u64) -> bool {
        (self.anchor.min(self.cursor)..=self.anchor.max(self.cursor)).contains(&key)
    }
}

/// Where the Console body has been asked to scroll, once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleReveal {
    Line(u64),
    End,
}

/// The reader's place on the Console page: the channel, what it is narrowed
/// to, and what is selected in it.
///
/// Transient. It is a reading position over logs, and a reopened session
/// starts at the end of its own.
#[derive(Debug, Clone, Default)]
pub struct ConsoleView {
    pub channel: ConsoleChannel,
    /// The severities switched off, by [`ConsoleSeverity::index`].
    hidden: [bool; 3],
    /// Case-insensitive text a shown line contains.
    pub find: String,
    pub selection: Option<ConsoleSelection>,
    pub reveal: Option<ConsoleReveal>,
}

impl ConsoleView {
    /// Show `channel` from its latest line, with nothing selected.
    pub fn open(&mut self, channel: ConsoleChannel) {
        self.channel = channel;
        self.selection = None;
        self.reveal = Some(ConsoleReveal::End);
    }

    /// Select the session line `key` and bring it into view, clearing the
    /// find text and showing its severity, so nothing the reader set earlier
    /// hides the line they were sent to.
    pub fn show_session_line(&mut self, key: u64, severity: crate::diagnostics::LogSeverity) {
        self.show_line(ConsoleChannel::Session, key, severity);
    }

    /// Select line `key` of `channel` and bring it into view, as
    /// [`Self::show_session_line`] does on the session's log.
    pub fn show_line(
        &mut self,
        channel: ConsoleChannel,
        key: u64,
        severity: crate::diagnostics::LogSeverity,
    ) {
        self.channel = channel;
        self.find.clear();
        self.hidden[ConsoleSeverity::of(severity).index()] = false;
        self.selection = Some(ConsoleSelection::single(key));
        self.reveal = Some(ConsoleReveal::Line(key));
    }

    pub fn shows(&self, severity: ConsoleSeverity) -> bool {
        !self.hidden[severity.index()]
    }

    pub fn toggle(&mut self, severity: ConsoleSeverity) {
        let hidden = &mut self.hidden[severity.index()];
        *hidden = !*hidden;
    }

    /// The severities switched off, by [`ConsoleSeverity::index`].
    pub fn hidden(&self) -> [bool; 3] {
        self.hidden
    }
}

/// Canonical discovery projection used by the mockup's specialist-tool
/// browser. Pins and favorites are personal application preferences, while
/// recent tools are bounded device-local history. None of these collections
/// creates, copies, or changes the owner of an engineering document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialistToolBrowserState {
    /// Search and filter are task-local review state. They deliberately reset
    /// when a new application session starts instead of becoming project data.
    #[serde(skip)]
    pub query: String,
    #[serde(skip)]
    pub filter: SpecialistToolFilter,
    #[serde(skip)]
    pub focus_search: bool,
    /// Personal discovery preferences retained by the application session.
    #[serde(default, deserialize_with = "deserialize_specialist_surfaces")]
    pub favorites: Vec<SurfaceId>,
    #[serde(default, deserialize_with = "deserialize_specialist_surfaces")]
    pub pinned: Vec<SurfaceId>,
    /// Newest-first, bounded device-local navigation history.
    #[serde(default, deserialize_with = "deserialize_specialist_surfaces")]
    pub recents: Vec<SurfaceId>,
}

impl Default for SpecialistToolBrowserState {
    fn default() -> Self {
        Self {
            query: String::new(),
            filter: SpecialistToolFilter::All,
            focus_search: false,
            favorites: Vec::new(),
            pinned: Vec::new(),
            recents: Vec::new(),
        }
    }
}

impl SpecialistToolBrowserState {
    pub const RECENT_LIMIT: usize = 12;

    #[must_use]
    pub fn is_favorite(&self, surface: SurfaceId) -> bool {
        self.favorites.contains(&surface)
    }

    #[must_use]
    pub fn is_pinned(&self, surface: SurfaceId) -> bool {
        self.pinned.contains(&surface)
    }

    #[must_use]
    pub fn is_recent(&self, surface: SurfaceId) -> bool {
        self.recents.contains(&surface)
    }

    pub fn toggle_favorite(&mut self, surface: SurfaceId) {
        toggle_surface_membership(&mut self.favorites, surface);
    }

    pub fn toggle_pin(&mut self, surface: SurfaceId) {
        toggle_surface_membership(&mut self.pinned, surface);
    }

    pub fn record_recent(&mut self, surface: SurfaceId) {
        if surface.archetype() != SurfaceArchetype::SpecialistWorkspace {
            return;
        }
        self.recents.retain(|candidate| *candidate != surface);
        self.recents.insert(0, surface);
        self.recents.truncate(Self::RECENT_LIMIT);
    }

    /// Remove non-specialist and duplicate identities from restored personal
    /// metadata. This is intentionally lossless for every still-canonical
    /// specialist identity and never guesses replacements for removed IDs.
    pub fn normalize(&mut self) {
        normalize_specialist_list(&mut self.favorites, None);
        normalize_specialist_list(&mut self.pinned, None);
        normalize_specialist_list(&mut self.recents, Some(Self::RECENT_LIMIT));
    }
}

pub(super) fn toggle_surface_membership(surfaces: &mut Vec<SurfaceId>, surface: SurfaceId) {
    if let Some(index) = surfaces.iter().position(|candidate| *candidate == surface) {
        surfaces.remove(index);
    } else if surface.archetype() == SurfaceArchetype::SpecialistWorkspace {
        surfaces.push(surface);
    }
}

pub(super) fn normalize_specialist_list(surfaces: &mut Vec<SurfaceId>, limit: Option<usize>) {
    let mut normalized = Vec::with_capacity(surfaces.len());
    for surface in surfaces.drain(..) {
        if surface.archetype() == SurfaceArchetype::SpecialistWorkspace
            && !normalized.contains(&surface)
        {
            normalized.push(surface);
        }
    }
    if let Some(limit) = limit {
        normalized.truncate(limit);
    }
    *surfaces = normalized;
}

pub(super) fn deserialize_specialist_surfaces<'de, D>(
    deserializer: D,
) -> Result<Vec<SurfaceId>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    let Some(stable_ids) = value.as_array() else {
        return Ok(Vec::new());
    };
    Ok(stable_ids
        .iter()
        .filter_map(serde_json::Value::as_str)
        .filter_map(|stable_id| stable_id.parse().ok())
        .collect())
}

/// Mockup-authored specialist discovery projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpecialistToolFilter {
    #[default]
    All,
    Pinned,
    Favorites,
    Recent,
}

impl SpecialistToolFilter {
    pub const ALL: [Self; 4] = [Self::All, Self::Pinned, Self::Favorites, Self::Recent];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Pinned => "Pinned",
            Self::Favorites => "Favorites",
            Self::Recent => "Recent",
        }
    }
}

/// Domain projection selected in the mockup-specified notification center.
/// The underlying activity stream is never discarded when this changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationFilter {
    #[default]
    All,
    Jobs,
    System,
}

impl NotificationFilter {
    pub const ALL: [Self; 3] = [Self::All, Self::Jobs, Self::System];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Jobs => "Jobs",
            Self::System => "System",
        }
    }
}

/// Which instance field an inspector inline edit is editing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InlineEditField {
    /// The reference designator. Validated against the SPICE designator
    /// rules and case-insensitive uniqueness before it is ever applied.
    Instance,
    /// The instance value.
    Value,
    /// The complete free-form instance-parameter override string used by
    /// component families that do not publish a typed parameter contract.
    Parameters,
    /// One `key=value` instance parameter.
    Parameter(String),
}

/// The document and occurrence that authorized an isolated inspector draft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineEditAuthority {
    pub project: crate::product::ProjectId,
    pub document: crate::state::CellViewRef,
    pub occurrence: Option<crate::state::workspace::DocumentOccurrence>,
    pub design_epoch: u64,
    pub document_epoch: u64,
}

/// One field's expected component and validated candidate. No design snapshot
/// is needed until the application publishes the completed edit.
#[derive(Debug, Clone)]
pub struct InlineEditSession {
    pub expected: crate::state::Component,
    pub field: InlineEditField,
    pub authority: InlineEditAuthority,
    pub description: String,
    pub buffer: String,
    pub candidate: Option<crate::state::Component>,
    pub error: Option<String>,
    pub widget: Option<egui::Id>,
}

/// An inspector field remains a draft until Enter or a successful focus change.
/// Escape discards it without mutating the design or its history.
#[derive(Debug, Clone, Default)]
pub struct InlineEdit {
    session: Option<InlineEditSession>,
}

impl InlineEdit {
    /// The buffer for `target`, or `None` when a different field (or no
    /// field) owns the session.
    pub fn buffer_for(&self, component: u64, field: &InlineEditField) -> Option<&str> {
        self.session
            .as_ref()
            .filter(|session| session.expected.id == component && session.field == *field)
            .map(|session| session.buffer.as_str())
    }

    /// Which of `component`'s fields holds the open session, if any.
    ///
    /// The inspector asks so a group of editable rows can reserve its
    /// validation strip for exactly as long as one of its own fields is being
    /// typed into, rather than permanently.
    pub fn editing_field(&self, component: u64) -> Option<&InlineEditField> {
        self.session
            .as_ref()
            .filter(|session| session.expected.id == component)
            .map(|session| &session.field)
    }

    /// Why the open session's text was rejected, if it was.
    pub fn error_for(&self, component: u64, field: &InlineEditField) -> Option<&str> {
        self.session
            .as_ref()
            .filter(|session| session.expected.id == component && session.field == *field)
            .and_then(|session| session.error.as_deref())
    }

    pub fn session(&self) -> Option<&InlineEditSession> {
        self.session.as_ref()
    }

    pub fn begin(&mut self, session: InlineEditSession) {
        self.session = Some(session);
    }

    /// The host supplies either a validated complete candidate or its rejection.
    pub fn set_draft(&mut self, text: String, candidate: Result<crate::state::Component, String>) {
        if let Some(session) = &mut self.session {
            session.buffer = text;
            match candidate {
                Ok(candidate) => {
                    session.candidate = Some(candidate);
                    session.error = None;
                }
                Err(error) => {
                    session.candidate = None;
                    session.error = Some(error);
                }
            }
        }
    }

    /// Record why the typed text was not applied, or clear the rejection.
    pub fn set_error(&mut self, error: Option<String>) {
        if let Some(session) = &mut self.session {
            session.error = error;
        }
    }

    pub fn set_widget(&mut self, widget: egui::Id) {
        if let Some(session) = &mut self.session {
            session.widget = Some(widget);
        }
    }

    /// Release a committed or explicitly cancelled draft.
    pub fn end(&mut self) {
        self.session = None;
    }
}
