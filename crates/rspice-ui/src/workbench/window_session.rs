//! Logical application-window and document-ownership session.
//!
//! Engineering objects remain in their project owners. This module stores
//! only presentation identities, viewport geometry, dock composition, and the
//! one-to-one mapping from an open document presentation to an application
//! window.

use std::collections::{BTreeMap, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::{
    SurfaceId, SurfaceRoute,
    state::{Workspace, WorkspaceDocumentId, WorkspaceLayoutState},
};

const PRIMARY_WINDOW_VALUE: u64 = 1;
const FIRST_SECONDARY_WINDOW_VALUE: u64 = 2;
const DEFAULT_WINDOW_SIZE: [f32; 2] = [1180.0, 760.0];
const MIN_WINDOW_SIZE: [f32; 2] = [520.0, 360.0];
const RECOVERY_INSET: f32 = 32.0;
const RECOVERY_CASCADE: f32 = 28.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApplicationWindowId(u64);

impl ApplicationWindowId {
    pub const fn primary() -> Self {
        Self(PRIMARY_WINDOW_VALUE)
    }

    pub const fn is_primary(self) -> bool {
        self.0 == PRIMARY_WINDOW_VALUE
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub fn viewport_id(self) -> egui::ViewportId {
        egui::ViewportId::from_hash_of(("rspice.application-window", self.0))
    }
}

impl Default for ApplicationWindowId {
    fn default() -> Self {
        Self::primary()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApplicationWindowBounds {
    pub position: Option<[f32; 2]>,
    pub inner_size: [f32; 2],
    pub monitor_size: Option<[f32; 2]>,
    pub recovery_pending: bool,
}

impl Default for ApplicationWindowBounds {
    fn default() -> Self {
        Self {
            position: None,
            inner_size: DEFAULT_WINDOW_SIZE,
            monitor_size: None,
            recovery_pending: false,
        }
    }
}

impl ApplicationWindowBounds {
    fn normalize(&mut self) {
        self.inner_size[0] = self.inner_size[0].max(MIN_WINDOW_SIZE[0]).min(16_384.0);
        self.inner_size[1] = self.inner_size[1].max(MIN_WINDOW_SIZE[1]).min(16_384.0);
        if self
            .position
            .is_some_and(|position| !position[0].is_finite() || !position[1].is_finite())
        {
            self.position = None;
        }
        if self.monitor_size.is_some_and(|size| {
            !size[0].is_finite() || !size[1].is_finite() || size[0] <= 0.0 || size[1] <= 0.0
        }) {
            self.monitor_size = None;
        }
    }

    pub fn observe(&mut self, info: &egui::ViewportInfo) {
        if let Some(rect) = info.outer_rect.or(info.inner_rect) {
            self.position = Some([rect.min.x, rect.min.y]);
            self.inner_size = [rect.width(), rect.height()];
        }
        self.monitor_size = info.monitor_size.map(|size| [size.x, size.y]);
        self.normalize();
        if self.is_off_screen() {
            self.recovery_pending = true;
        }
    }

    pub fn is_off_screen(self) -> bool {
        let (Some(position), Some(monitor)) = (self.position, self.monitor_size) else {
            return false;
        };
        let right = position[0] + self.inner_size[0];
        let bottom = position[1] + self.inner_size[1];
        right < RECOVERY_INSET
            || bottom < RECOVERY_INSET
            || position[0] > monitor[0] - RECOVERY_INSET
            || position[1] > monitor[1] - RECOVERY_INSET
    }

    pub fn recovered_position(self, cascade_index: usize) -> egui::Pos2 {
        let offset = RECOVERY_INSET + RECOVERY_CASCADE * cascade_index as f32;
        let monitor = self.monitor_size.unwrap_or([1920.0, 1080.0]);
        egui::pos2(
            offset.min((monitor[0] - MIN_WINDOW_SIZE[0]).max(0.0)),
            offset.min((monitor[1] - MIN_WINDOW_SIZE[1]).max(0.0)),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApplicationWindowState {
    pub title: String,
    pub workspace: Workspace,
    pub route: SurfaceRoute,
    pub active_document: Option<WorkspaceDocumentId>,
    pub documents: Vec<WorkspaceDocumentId>,
    pub layout: WorkspaceLayoutState,
    pub bounds: ApplicationWindowBounds,
    pub restore_on_launch: bool,
    /// Opt-in detached-window policy from the mockup. When enabled, the
    /// primary window's current dock composition is projected into this
    /// window before rendering; disabling it restores independent layout.
    pub synchronize_chrome_with_primary: bool,
    /// Runtime host fullscreen intent. A restored session always starts in
    /// ordinary windowed mode and lets the active host confirm transitions.
    #[serde(skip)]
    pub full_screen: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ApplicationWindowRenderPlan {
    pub(crate) title: String,
    pub(crate) inner_size: egui::Vec2,
    pub(crate) position: Option<egui::Pos2>,
    pub(crate) recovering: bool,
}

impl ApplicationWindowState {
    pub(crate) fn render_plan(&self, cascade_index: usize) -> ApplicationWindowRenderPlan {
        ApplicationWindowRenderPlan {
            title: self.title.clone(),
            inner_size: egui::vec2(self.bounds.inner_size[0], self.bounds.inner_size[1]),
            position: if self.bounds.recovery_pending {
                Some(self.bounds.recovered_position(cascade_index))
            } else {
                self.bounds
                    .position
                    .map(|position| egui::pos2(position[0], position[1]))
            },
            recovering: self.bounds.recovery_pending,
        }
    }
}

impl Default for ApplicationWindowState {
    fn default() -> Self {
        Self {
            title: "RSpice window".to_owned(),
            workspace: Workspace::Design,
            route: SurfaceRoute::surface(SurfaceId::Design),
            active_document: None,
            documents: Vec::new(),
            layout: WorkspaceLayoutState::default(),
            bounds: ApplicationWindowBounds::default(),
            restore_on_launch: true,
            synchronize_chrome_with_primary: false,
            full_screen: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowSessionError {
    UnknownWindow(ApplicationWindowId),
    PrimaryWindowCannotBeRemoved,
    SourceWindowMismatch {
        expected: ApplicationWindowId,
        actual: ApplicationWindowId,
    },
}

impl std::fmt::Display for WindowSessionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownWindow(window) => {
                write!(
                    formatter,
                    "application window {} does not exist",
                    window.value()
                )
            }
            Self::PrimaryWindowCannotBeRemoved => {
                formatter.write_str("the primary application window cannot be removed")
            }
            Self::SourceWindowMismatch { expected, actual } => write!(
                formatter,
                "document ownership changed from window {} to window {}",
                expected.value(),
                actual.value()
            ),
        }
    }
}

impl std::error::Error for WindowSessionError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WindowSessionRegistry {
    next_id: u64,
    windows: BTreeMap<ApplicationWindowId, ApplicationWindowState>,
    ownership: HashMap<WorkspaceDocumentId, ApplicationWindowId>,
    clamp_restored_windows: bool,
    #[serde(skip)]
    current: ApplicationWindowId,
}

impl Default for WindowSessionRegistry {
    fn default() -> Self {
        let mut windows = BTreeMap::new();
        windows.insert(
            ApplicationWindowId::primary(),
            ApplicationWindowState {
                title: "RSpice primary window".to_owned(),
                ..ApplicationWindowState::default()
            },
        );
        Self {
            next_id: FIRST_SECONDARY_WINDOW_VALUE,
            windows,
            ownership: HashMap::new(),
            clamp_restored_windows: true,
            current: ApplicationWindowId::primary(),
        }
    }
}

impl WindowSessionRegistry {
    pub fn normalize_after_restore(&mut self) {
        self.windows
            .entry(ApplicationWindowId::primary())
            .or_insert_with(|| ApplicationWindowState {
                title: "RSpice primary window".to_owned(),
                ..ApplicationWindowState::default()
            });
        self.current = ApplicationWindowId::primary();
        let clamp_restored_windows = self.clamp_restored_windows;
        self.windows.retain(|id, state| {
            state.bounds.normalize();
            if clamp_restored_windows && state.bounds.is_off_screen() {
                state.bounds.recovery_pending = true;
            }
            id.is_primary() || state.restore_on_launch
        });
        self.ownership
            .retain(|_, owner| !owner.is_primary() && self.windows.contains_key(owner));
        for state in self.windows.values_mut() {
            state.documents.clear();
            state.active_document = None;
        }
        for (document, owner) in &self.ownership {
            if let Some(state) = self.windows.get_mut(owner) {
                state.documents.push(document.clone());
            }
        }
        for state in self.windows.values_mut() {
            state.documents.sort_by_key(document_sort_key);
            state.documents.dedup();
            state.active_document = state.documents.first().cloned();
        }
        self.next_id = self
            .windows
            .keys()
            .map(|id| id.value())
            .max()
            .unwrap_or(PRIMARY_WINDOW_VALUE)
            .saturating_add(1)
            .max(FIRST_SECONDARY_WINDOW_VALUE);
    }

    pub const fn primary(&self) -> ApplicationWindowId {
        ApplicationWindowId::primary()
    }

    pub const fn current(&self) -> ApplicationWindowId {
        self.current
    }

    pub fn set_current(&mut self, window: ApplicationWindowId) -> Result<(), WindowSessionError> {
        if !self.windows.contains_key(&window) {
            return Err(WindowSessionError::UnknownWindow(window));
        }
        self.current = window;
        Ok(())
    }

    pub fn state(&self, window: ApplicationWindowId) -> Option<&ApplicationWindowState> {
        self.windows.get(&window)
    }

    pub fn state_mut(
        &mut self,
        window: ApplicationWindowId,
    ) -> Option<&mut ApplicationWindowState> {
        self.windows.get_mut(&window)
    }

    pub fn windows(&self) -> impl Iterator<Item = (ApplicationWindowId, &ApplicationWindowState)> {
        self.windows.iter().map(|(id, state)| (*id, state))
    }

    pub fn secondary_window_ids(&self) -> Vec<ApplicationWindowId> {
        self.windows
            .keys()
            .copied()
            .filter(|id| !id.is_primary())
            .collect()
    }

    pub const fn clamp_restored_windows(&self) -> bool {
        self.clamp_restored_windows
    }

    pub fn set_clamp_restored_windows(&mut self, enabled: bool) {
        self.clamp_restored_windows = enabled;
    }

    pub fn create_window(
        &mut self,
        title: impl Into<String>,
        workspace: Workspace,
        layout: WorkspaceLayoutState,
        restore_on_launch: bool,
    ) -> ApplicationWindowId {
        let id = ApplicationWindowId(self.next_id.max(FIRST_SECONDARY_WINDOW_VALUE));
        self.next_id = id.value().saturating_add(1);
        self.windows.insert(
            id,
            ApplicationWindowState {
                title: title.into(),
                workspace,
                route: SurfaceRoute::surface(SurfaceId::from_workspace(workspace)),
                layout,
                restore_on_launch,
                ..ApplicationWindowState::default()
            },
        );
        id
    }

    pub fn owner(&self, document: &WorkspaceDocumentId) -> ApplicationWindowId {
        self.ownership
            .get(document)
            .copied()
            .unwrap_or_else(ApplicationWindowId::primary)
    }

    pub fn belongs_to_current(&self, document: &WorkspaceDocumentId) -> bool {
        self.owner(document) == self.current
    }

    pub fn move_document(
        &mut self,
        document: WorkspaceDocumentId,
        expected_source: ApplicationWindowId,
        destination: ApplicationWindowId,
    ) -> Result<(), WindowSessionError> {
        if !self.windows.contains_key(&destination) {
            return Err(WindowSessionError::UnknownWindow(destination));
        }
        let actual = self.owner(&document);
        if actual != expected_source {
            return Err(WindowSessionError::SourceWindowMismatch {
                expected: expected_source,
                actual,
            });
        }
        for state in self.windows.values_mut() {
            state.documents.retain(|candidate| candidate != &document);
            if state.active_document.as_ref() == Some(&document) {
                state.active_document = state.documents.first().cloned();
            }
        }
        if destination.is_primary() {
            self.ownership.remove(&document);
            let state = self
                .windows
                .get_mut(&destination)
                .expect("primary destination always exists");
            state.workspace = document.workspace();
            state.documents.push(document.clone());
            state.documents.dedup();
            state.active_document = Some(document);
        } else {
            self.ownership.insert(document.clone(), destination);
            let state = self
                .windows
                .get_mut(&destination)
                .expect("destination was validated");
            state.workspace = document.workspace();
            state.documents.push(document.clone());
            state.documents.dedup();
            state.active_document = Some(document);
        }
        Ok(())
    }

    pub fn reattach_document(
        &mut self,
        document: WorkspaceDocumentId,
        expected_source: ApplicationWindowId,
        insert_at_end: bool,
    ) -> Result<(), WindowSessionError> {
        let actual = self.owner(&document);
        if actual != expected_source {
            return Err(WindowSessionError::SourceWindowMismatch {
                expected: expected_source,
                actual,
            });
        }
        let primary = ApplicationWindowId::primary();
        let owning_workspace = document.workspace();
        let insertion = self.windows.get(&primary).and_then(|state| {
            if !insert_at_end {
                state
                    .active_document
                    .as_ref()
                    .filter(|active| active.workspace() == owning_workspace)
                    .and_then(|active| {
                        state
                            .documents
                            .iter()
                            .position(|candidate| candidate == active)
                    })
                    .map(|position| position + 1)
                    .or_else(|| {
                        state
                            .documents
                            .iter()
                            .rposition(|candidate| candidate.workspace() == owning_workspace)
                            .map(|position| position + 1)
                    })
            } else {
                state
                    .documents
                    .iter()
                    .rposition(|candidate| candidate.workspace() == owning_workspace)
                    .map(|position| position + 1)
            }
        });
        for state in self.windows.values_mut() {
            state.documents.retain(|candidate| candidate != &document);
            if state.active_document.as_ref() == Some(&document) {
                state.active_document = state.documents.first().cloned();
            }
        }
        self.ownership.remove(&document);
        let state = self
            .windows
            .get_mut(&primary)
            .expect("primary application window always exists");
        let insertion = insertion
            .unwrap_or(state.documents.len())
            .min(state.documents.len());
        state.documents.insert(insertion, document.clone());
        state.workspace = document.workspace();
        state.active_document = Some(document);
        Ok(())
    }

    pub fn detach_document(
        &mut self,
        document: WorkspaceDocumentId,
        title: impl Into<String>,
        layout: WorkspaceLayoutState,
        restore_on_launch: bool,
    ) -> Result<ApplicationWindowId, WindowSessionError> {
        let source = self.owner(&document);
        let destination =
            self.create_window(title, document.workspace(), layout, restore_on_launch);
        if let Err(error) = self.move_document(document, source, destination) {
            self.windows.remove(&destination);
            return Err(error);
        }
        Ok(destination)
    }

    pub fn set_active_document(
        &mut self,
        window: ApplicationWindowId,
        document: WorkspaceDocumentId,
    ) -> Result<(), WindowSessionError> {
        if self.owner(&document) != window {
            return Err(WindowSessionError::SourceWindowMismatch {
                expected: window,
                actual: self.owner(&document),
            });
        }
        let state = self
            .windows
            .get_mut(&window)
            .ok_or(WindowSessionError::UnknownWindow(window))?;
        state.workspace = document.workspace();
        state.active_document = Some(document);
        Ok(())
    }

    pub fn active_document(
        &self,
        window: ApplicationWindowId,
        workspace: Workspace,
    ) -> Option<&WorkspaceDocumentId> {
        self.windows
            .get(&window)
            .and_then(|state| (state.workspace == workspace).then_some(state))
            .and_then(|state| state.active_document.as_ref())
    }

    pub fn close_window(
        &mut self,
        window: ApplicationWindowId,
    ) -> Result<Vec<WorkspaceDocumentId>, WindowSessionError> {
        if window.is_primary() {
            return Err(WindowSessionError::PrimaryWindowCannotBeRemoved);
        }
        let state = self
            .windows
            .remove(&window)
            .ok_or(WindowSessionError::UnknownWindow(window))?;
        for document in &state.documents {
            self.ownership.remove(document);
        }
        if self.current == window {
            self.current = ApplicationWindowId::primary();
        }
        Ok(state.documents)
    }

    pub fn remove_empty_window(
        &mut self,
        window: ApplicationWindowId,
    ) -> Result<bool, WindowSessionError> {
        if window.is_primary() {
            return Ok(false);
        }
        let empty = self
            .windows
            .get(&window)
            .ok_or(WindowSessionError::UnknownWindow(window))?
            .documents
            .is_empty();
        if empty {
            self.close_window(window)?;
        }
        Ok(empty)
    }

    pub fn consolidate(&mut self) -> usize {
        let count = self.windows.len().saturating_sub(1);
        self.windows.retain(|id, _| id.is_primary());
        self.ownership.clear();
        self.current = ApplicationWindowId::primary();
        count
    }

    pub fn recover_off_screen(&mut self) -> usize {
        let windows = self.secondary_window_ids();
        self.recover_windows(windows)
    }

    pub fn recover_windows(
        &mut self,
        windows: impl IntoIterator<Item = ApplicationWindowId>,
    ) -> usize {
        let mut count = 0;
        for id in windows {
            if id.is_primary() {
                continue;
            }
            let Some(state) = self.windows.get_mut(&id) else {
                continue;
            };
            state.bounds.position = None;
            state.bounds.recovery_pending = true;
            count += 1;
        }
        count
    }

    pub fn acknowledge_recovery(&mut self, window: ApplicationWindowId) {
        if let Some(state) = self.windows.get_mut(&window) {
            state.bounds.recovery_pending = false;
        }
    }

    pub fn retain_documents(&mut self, available: impl IntoIterator<Item = WorkspaceDocumentId>) {
        let ordered = available.into_iter().collect::<Vec<_>>();
        let available = ordered.iter().cloned().collect::<HashSet<_>>();
        self.ownership
            .retain(|document, _| available.contains(document));
        for state in self.windows.values_mut() {
            state
                .documents
                .retain(|document| available.contains(document));
            if state
                .active_document
                .as_ref()
                .is_some_and(|document| !available.contains(document))
            {
                state.active_document = state.documents.first().cloned();
            }
        }
        if let Some(primary) = self.windows.get_mut(&ApplicationWindowId::primary()) {
            for document in ordered {
                if !self.ownership.contains_key(&document) && !primary.documents.contains(&document)
                {
                    primary.documents.push(document);
                }
            }
        }
    }
}

fn document_sort_key(document: &WorkspaceDocumentId) -> String {
    format!("{document:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result_document() -> WorkspaceDocumentId {
        WorkspaceDocumentId::ResultDataset(crate::product::DatasetId::new())
    }

    #[test]
    fn detach_move_close_and_consolidate_preserve_unique_ownership() {
        let mut registry = WindowSessionRegistry::default();
        let first = result_document();
        let second = result_document();
        let detached = registry
            .detach_document(
                first.clone(),
                "Detached result",
                WorkspaceLayoutState::default(),
                true,
            )
            .unwrap();
        let other = registry.create_window(
            "Other",
            Workspace::Results,
            WorkspaceLayoutState::default(),
            true,
        );

        assert_eq!(registry.owner(&first), detached);
        registry
            .move_document(first.clone(), detached, other)
            .unwrap();
        assert_eq!(registry.owner(&first), other);
        registry
            .move_document(second.clone(), registry.primary(), other)
            .unwrap();
        assert_eq!(registry.owner(&second), other);

        let returned = registry.close_window(other).unwrap();
        assert_eq!(returned.len(), 2);
        assert_eq!(registry.owner(&first), registry.primary());
        assert_eq!(registry.owner(&second), registry.primary());
        assert_eq!(registry.consolidate(), 1);
        assert_eq!(registry.secondary_window_ids(), Vec::new());
    }

    #[test]
    fn restored_registry_repairs_dangling_owners_and_next_identity() {
        let mut registry = WindowSessionRegistry::default();
        let document = result_document();
        registry
            .ownership
            .insert(document.clone(), ApplicationWindowId(99));
        registry.current = ApplicationWindowId(99);
        registry.next_id = 1;

        registry.normalize_after_restore();

        assert_eq!(registry.current(), registry.primary());
        assert_eq!(registry.owner(&document), registry.primary());
        let created = registry.create_window(
            "Recovered",
            Workspace::Project,
            WorkspaceLayoutState::default(),
            true,
        );
        assert!(created.value() >= FIRST_SECONDARY_WINDOW_VALUE);
    }

    #[test]
    fn geometry_recovery_clamps_to_visible_monitor_origin() {
        let bounds = ApplicationWindowBounds {
            position: Some([4_000.0, 2_000.0]),
            inner_size: [900.0, 600.0],
            monitor_size: Some([1_920.0, 1_080.0]),
            recovery_pending: true,
        };
        assert!(bounds.is_off_screen());
        let recovered = bounds.recovered_position(2);
        assert!(recovered.x >= 0.0 && recovered.x < 1_920.0);
        assert!(recovered.y >= 0.0 && recovered.y < 1_080.0);
    }

    #[test]
    fn viewport_render_plan_uses_recovery_position_without_losing_size_or_title() {
        let state = ApplicationWindowState {
            title: "Results review".to_owned(),
            bounds: ApplicationWindowBounds {
                position: Some([4_000.0, 2_000.0]),
                inner_size: [1_240.0, 820.0],
                monitor_size: Some([1_920.0, 1_080.0]),
                recovery_pending: true,
            },
            ..ApplicationWindowState::default()
        };

        let plan = state.render_plan(1);

        assert_eq!(plan.title, "Results review");
        assert_eq!(plan.inner_size, egui::vec2(1_240.0, 820.0));
        assert_eq!(plan.position, Some(egui::pos2(60.0, 60.0)));
        assert!(plan.recovering);
    }

    #[test]
    fn stale_source_rejects_move_without_mutating_either_window() {
        let mut registry = WindowSessionRegistry::default();
        let document = result_document();
        let first = registry
            .detach_document(
                document.clone(),
                "First",
                WorkspaceLayoutState::default(),
                true,
            )
            .unwrap();
        let second = registry.create_window(
            "Second",
            Workspace::Results,
            WorkspaceLayoutState::default(),
            true,
        );

        let error = registry
            .move_document(document.clone(), registry.primary(), second)
            .unwrap_err();

        assert!(matches!(
            error,
            WindowSessionError::SourceWindowMismatch { .. }
        ));
        assert_eq!(registry.owner(&document), first);
        assert!(registry.state(first).unwrap().documents.contains(&document));
        assert!(
            !registry
                .state(second)
                .unwrap()
                .documents
                .contains(&document)
        );
    }
}
