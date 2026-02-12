use super::{AppState, BottomPanelTab, PanelSizes, PanelVisibility};

impl serde::Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize minimal state needed for session recovery.
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AppState", 3)?;
        state.serialize_field("panels", &PanelVisibilitySer::from(&self.panels))?;
        state.serialize_field("panel_sizes", &PanelSizesSer::from(&self.panel_sizes))?;
        state.serialize_field(
            "viewer_workspace",
            &ViewerWorkspaceSer::from(&self.viewer_workspace),
        )?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for AppState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct AppStateDe {
            panels: PanelVisibilitySer,
            panel_sizes: PanelSizesSer,
            #[serde(default)]
            viewer_workspace: ViewerWorkspaceSer,
        }

        // Deserialize minimal persisted data and use defaults for the rest.
        let de = AppStateDe::deserialize(deserializer)?;
        Ok(Self {
            panels: de.panels.into(),
            panel_sizes: de.panel_sizes.into(),
            viewer_workspace: de.viewer_workspace.into(),
            ..Default::default()
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct PanelVisibilitySer {
    pub(super) project_browser: bool,
    #[serde(default)]
    pub(super) results_browser: bool,
    pub(super) properties: bool,
    #[serde(default = "default_bottom_panel")]
    pub(super) bottom_panel: bool,
    #[serde(default)]
    pub(super) active_bottom_tab: usize,
    #[serde(default)]
    pub(super) signal_browser: bool,
    #[serde(default)]
    pub(super) script_console: bool,
}

fn default_bottom_panel() -> bool {
    true
}

impl From<&PanelVisibility> for PanelVisibilitySer {
    fn from(panels: &PanelVisibility) -> Self {
        Self {
            project_browser: panels.project_browser,
            results_browser: panels.results_browser,
            properties: panels.properties,
            bottom_panel: panels.bottom_panel,
            active_bottom_tab: match panels.active_bottom_tab {
                BottomPanelTab::Waveform => 1,
                BottomPanelTab::Log => 2,
                BottomPanelTab::Automation => 3,
            },
            signal_browser: panels.signal_browser,
            script_console: panels.script_console,
        }
    }
}

impl From<PanelVisibilitySer> for PanelVisibility {
    fn from(serialized: PanelVisibilitySer) -> Self {
        Self {
            project_browser: serialized.project_browser,
            results_browser: serialized.results_browser,
            properties: serialized.properties,
            bottom_panel: serialized.bottom_panel,
            active_bottom_tab: match serialized.active_bottom_tab {
                0 => BottomPanelTab::Log,
                1 => BottomPanelTab::Waveform,
                2 => BottomPanelTab::Log,
                3 => BottomPanelTab::Automation,
                _ => BottomPanelTab::Log,
            },
            signal_browser: serialized.signal_browser,
            script_console: serialized.script_console,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct PanelSizesSer {
    pub(super) waveform_height: f32,
    pub(super) console_height: f32,
    pub(super) browser_width: f32,
    pub(super) properties_width: f32,
}

impl From<&PanelSizes> for PanelSizesSer {
    fn from(sizes: &PanelSizes) -> Self {
        Self {
            waveform_height: sizes.waveform_height,
            console_height: sizes.console_height,
            browser_width: sizes.browser_width,
            properties_width: sizes.properties_width,
        }
    }
}

impl From<PanelSizesSer> for PanelSizes {
    fn from(serialized: PanelSizesSer) -> Self {
        Self {
            waveform_height: serialized.waveform_height,
            console_height: serialized.console_height,
            browser_width: serialized.browser_width,
            properties_width: serialized.properties_width,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct ViewerWorkspaceSer {
    #[serde(default = "default_viewer_workspace_tabs")]
    pub(super) tabs: Vec<u8>,
    #[serde(default)]
    pub(super) active_index: usize,
}

impl Default for ViewerWorkspaceSer {
    fn default() -> Self {
        Self {
            tabs: default_viewer_workspace_tabs(),
            active_index: 0,
        }
    }
}

fn default_viewer_workspace_tabs() -> Vec<u8> {
    vec![crate::viewers::ActiveViewer::Waveform.id()]
}

impl From<&crate::viewers::ViewerWorkspace> for ViewerWorkspaceSer {
    fn from(workspace: &crate::viewers::ViewerWorkspace) -> Self {
        Self {
            tabs: workspace.tabs().iter().map(|viewer| viewer.id()).collect(),
            active_index: workspace.active_index(),
        }
    }
}

impl From<ViewerWorkspaceSer> for crate::viewers::ViewerWorkspace {
    fn from(serialized: ViewerWorkspaceSer) -> Self {
        let tabs = serialized
            .tabs
            .into_iter()
            .filter_map(crate::viewers::ActiveViewer::from_id)
            .collect();
        crate::viewers::ViewerWorkspace::from_tabs(tabs, serialized.active_index)
    }
}
