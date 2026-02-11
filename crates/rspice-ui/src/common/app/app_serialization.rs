use super::{AppState, BottomPanelTab, PanelSizes, PanelVisibility};

impl serde::Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize minimal state needed for session recovery.
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AppState", 2)?;
        state.serialize_field("panels", &PanelVisibilitySer::from(&self.panels))?;
        state.serialize_field("panel_sizes", &PanelSizesSer::from(&self.panel_sizes))?;
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
        }

        // Deserialize minimal persisted data and use defaults for the rest.
        let de = AppStateDe::deserialize(deserializer)?;
        Ok(Self {
            panels: de.panels.into(),
            panel_sizes: de.panel_sizes.into(),
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
    pub(super) smith_chart: bool,
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
            smith_chart: panels.smith_chart,
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
            smith_chart: serialized.smith_chart,
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
