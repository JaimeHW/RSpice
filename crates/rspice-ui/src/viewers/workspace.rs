//! Viewer Workspace
//!
//! Manages open specialized viewer tabs in the waveform workspace.

use super::ActiveViewer;

/// Tabbed workspace model for specialized viewers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerWorkspace {
    tabs: Vec<ActiveViewer>,
    active_index: usize,
}

impl Default for ViewerWorkspace {
    fn default() -> Self {
        Self {
            tabs: vec![ActiveViewer::Waveform],
            active_index: 0,
        }
    }
}

impl ViewerWorkspace {
    /// Create a new workspace with a default waveform tab.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restore a workspace from serialized tabs and active index.
    pub fn from_tabs(tabs: Vec<ActiveViewer>, active_index: usize) -> Self {
        let active_viewer = tabs
            .get(active_index)
            .copied()
            .unwrap_or(ActiveViewer::Waveform);

        let mut deduped = Vec::with_capacity(tabs.len().max(1));
        for viewer in tabs {
            if !deduped.contains(&viewer) {
                deduped.push(viewer);
            }
        }

        if deduped.is_empty() {
            deduped.push(ActiveViewer::Waveform);
        }

        let active_index = deduped
            .iter()
            .position(|viewer| *viewer == active_viewer)
            .unwrap_or(0);

        Self {
            tabs: deduped,
            active_index,
        }
    }

    /// Open a viewer tab or focus it if already open.
    ///
    /// Returns `true` if the workspace changed.
    pub fn open_or_focus(&mut self, viewer: ActiveViewer) -> bool {
        if let Some(index) = self.tabs.iter().position(|tab| *tab == viewer) {
            return self.focus_index(index);
        }

        self.tabs.push(viewer);
        self.active_index = self.tabs.len() - 1;
        true
    }

    /// Focus an existing viewer tab.
    ///
    /// Returns `true` if the active tab changed.
    pub fn focus(&mut self, viewer: ActiveViewer) -> bool {
        let Some(index) = self.tabs.iter().position(|tab| *tab == viewer) else {
            return false;
        };
        self.focus_index(index)
    }

    /// Focus tab by index.
    pub fn focus_index(&mut self, index: usize) -> bool {
        if index >= self.tabs.len() || self.active_index == index {
            return false;
        }
        self.active_index = index;
        true
    }

    /// Close a specific viewer tab.
    pub fn close_viewer(&mut self, viewer: ActiveViewer) -> bool {
        let Some(index) = self.tabs.iter().position(|tab| *tab == viewer) else {
            return false;
        };
        self.close_at(index).is_some()
    }

    /// Close tab by index and return the removed viewer.
    pub fn close_at(&mut self, index: usize) -> Option<ActiveViewer> {
        if index >= self.tabs.len() {
            return None;
        }

        let removed = self.tabs.remove(index);

        if self.tabs.is_empty() {
            self.tabs.push(ActiveViewer::Waveform);
            self.active_index = 0;
            return Some(removed);
        }

        if self.active_index > index {
            self.active_index -= 1;
        } else if self.active_index == index {
            self.active_index = self.active_index.min(self.tabs.len() - 1);
        }

        Some(removed)
    }

    /// Close the active tab and return the new active viewer.
    pub fn close_active(&mut self) -> ActiveViewer {
        let active_index = self.active_index;
        let _ = self.close_at(active_index);
        self.active_viewer()
    }

    /// Open tabs in display order.
    pub fn tabs(&self) -> &[ActiveViewer] {
        &self.tabs
    }

    /// Access tab by index.
    pub fn tab_at(&self, index: usize) -> Option<ActiveViewer> {
        self.tabs.get(index).copied()
    }

    /// Number of open tabs.
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Active viewer kind.
    pub fn active_viewer(&self) -> ActiveViewer {
        self.tabs[self.active_index]
    }

    /// Active tab index.
    pub fn active_index(&self) -> usize {
        self.active_index
    }

    /// Whether the workspace currently contains this viewer.
    pub fn contains(&self, viewer: ActiveViewer) -> bool {
        self.tabs.contains(&viewer)
    }

    /// Whether this viewer is currently active.
    pub fn is_active(&self, viewer: ActiveViewer) -> bool {
        self.active_viewer() == viewer
    }
}

