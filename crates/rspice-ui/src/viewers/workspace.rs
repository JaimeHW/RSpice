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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_workspace_contains_waveform_tab() {
        let workspace = ViewerWorkspace::default();
        assert_eq!(workspace.tabs(), &[ActiveViewer::Waveform]);
        assert_eq!(workspace.active_viewer(), ActiveViewer::Waveform);
        assert_eq!(workspace.active_index(), 0);
    }

    #[test]
    fn open_or_focus_adds_new_tab_and_activates_it() {
        let mut workspace = ViewerWorkspace::default();

        assert!(workspace.open_or_focus(ActiveViewer::BodePlot));
        assert_eq!(workspace.tabs().len(), 2);
        assert_eq!(workspace.active_viewer(), ActiveViewer::BodePlot);
    }

    #[test]
    fn open_or_focus_existing_tab_focuses_without_duplicates() {
        let mut workspace =
            ViewerWorkspace::from_tabs(vec![ActiveViewer::Waveform, ActiveViewer::EyeDiagram], 0);

        assert!(workspace.open_or_focus(ActiveViewer::EyeDiagram));
        assert_eq!(workspace.tabs().len(), 2);
        assert_eq!(workspace.active_viewer(), ActiveViewer::EyeDiagram);
        assert!(!workspace.open_or_focus(ActiveViewer::EyeDiagram));
    }

    #[test]
    fn focus_returns_false_for_missing_viewer() {
        let mut workspace = ViewerWorkspace::default();
        assert!(!workspace.focus(ActiveViewer::Histogram));
    }

    #[test]
    fn close_viewer_returns_false_when_not_open() {
        let mut workspace = ViewerWorkspace::default();
        assert!(!workspace.close_viewer(ActiveViewer::Nyquist));
    }

    #[test]
    fn close_active_moves_focus_to_next_tab() {
        let mut workspace = ViewerWorkspace::from_tabs(
            vec![
                ActiveViewer::Waveform,
                ActiveViewer::BodePlot,
                ActiveViewer::Nyquist,
            ],
            1,
        );

        let new_active = workspace.close_active();

        assert_eq!(new_active, ActiveViewer::Nyquist);
        assert_eq!(
            workspace.tabs(),
            &[ActiveViewer::Waveform, ActiveViewer::Nyquist]
        );
        assert_eq!(workspace.active_index(), 1);
    }

    #[test]
    fn close_viewer_before_active_shifts_active_index_left() {
        let mut workspace = ViewerWorkspace::from_tabs(
            vec![
                ActiveViewer::Waveform,
                ActiveViewer::Histogram,
                ActiveViewer::BodePlot,
            ],
            2,
        );

        assert!(workspace.close_viewer(ActiveViewer::Histogram));
        assert_eq!(workspace.active_viewer(), ActiveViewer::BodePlot);
        assert_eq!(workspace.active_index(), 1);
    }

    #[test]
    fn closing_last_tab_recreates_waveform_tab() {
        let mut workspace = ViewerWorkspace::from_tabs(vec![ActiveViewer::Nyquist], 0);

        assert!(workspace.close_viewer(ActiveViewer::Nyquist));
        assert_eq!(workspace.tabs(), &[ActiveViewer::Waveform]);
        assert_eq!(workspace.active_viewer(), ActiveViewer::Waveform);
    }

    #[test]
    fn from_tabs_deduplicates_and_preserves_active_viewer() {
        let workspace = ViewerWorkspace::from_tabs(
            vec![
                ActiveViewer::Waveform,
                ActiveViewer::BodePlot,
                ActiveViewer::Waveform,
                ActiveViewer::Nyquist,
            ],
            3,
        );

        assert_eq!(
            workspace.tabs(),
            &[
                ActiveViewer::Waveform,
                ActiveViewer::BodePlot,
                ActiveViewer::Nyquist
            ]
        );
        assert_eq!(workspace.active_viewer(), ActiveViewer::Nyquist);
    }

    #[test]
    fn from_tabs_empty_falls_back_to_waveform() {
        let workspace = ViewerWorkspace::from_tabs(Vec::new(), 4);

        assert_eq!(workspace.tabs(), &[ActiveViewer::Waveform]);
        assert_eq!(workspace.active_viewer(), ActiveViewer::Waveform);
        assert_eq!(workspace.active_index(), 0);
    }
}
