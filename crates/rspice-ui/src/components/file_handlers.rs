//! Toolbar File Operation Handlers
//!
//! Platform-specific implementations for file open/save operations.
//! Desktop uses native file dialogs (rfd); web shows not-supported message.

use dioxus::prelude::*;
use std::path::PathBuf;

use crate::state::{ConsoleMessage, SchematicState, SimulationState, WaveformData};

/// Result from a file operation
pub enum FileOpResult {
    /// Operation succeeded with a path
    Success(PathBuf),
    /// Operation cancelled by user
    Cancelled,
    /// Operation not supported on this platform
    NotSupported,
}

// ============================================================================
// Desktop implementations using rfd
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
pub mod handlers {
    use super::*;

    /// Open netlist file dialog and load content
    pub async fn open_netlist(mut sim_state: Signal<SimulationState>) {
        if let Some(path) = rfd::AsyncFileDialog::new()
            .add_filter("SPICE Netlist", &["cir", "sp", "spice", "net"])
            .add_filter("All Files", &["*"])
            .pick_file()
            .await
        {
            match std::fs::read_to_string(path.path()) {
                Ok(content) => {
                    let mut state = sim_state.write();
                    state.netlist_content = content;
                    state.current_file = Some(path.path().to_path_buf());
                    state.is_dirty = false;
                    state.waveforms.clear();
                    state.console_messages.push(ConsoleMessage::success(format!(
                        "Opened: {}",
                        path.path().display()
                    )));
                }
                Err(e) => {
                    sim_state
                        .write()
                        .console_messages
                        .push(ConsoleMessage::error(format!("Failed to open file: {}", e)));
                }
            }
        }
    }

    /// Save netlist to file
    pub async fn save_netlist(mut sim_state: Signal<SimulationState>) {
        let content = sim_state.read().netlist_content.clone();
        let current_path = sim_state.read().current_file.clone();

        let save_path = if let Some(path) = current_path {
            Some(path)
        } else {
            rfd::AsyncFileDialog::new()
                .add_filter("SPICE Netlist", &["cir"])
                .set_file_name("circuit.cir")
                .save_file()
                .await
                .map(|f| f.path().to_path_buf())
        };

        if let Some(path) = save_path {
            match std::fs::write(&path, &content) {
                Ok(_) => {
                    let mut state = sim_state.write();
                    state.current_file = Some(path.clone());
                    state.is_dirty = false;
                    state.console_messages.push(ConsoleMessage::success(format!(
                        "Saved: {}",
                        path.display()
                    )));
                }
                Err(e) => {
                    sim_state
                        .write()
                        .console_messages
                        .push(ConsoleMessage::error(format!("Failed to save: {}", e)));
                }
            }
        }
    }

    /// Import LTspice .raw file
    pub async fn import_raw(mut sim_state: Signal<SimulationState>) {
        if let Some(path) = rfd::AsyncFileDialog::new()
            .add_filter("LTspice Raw", &["raw"])
            .add_filter("All Files", &["*"])
            .pick_file()
            .await
        {
            match rspice_core::compat::parse_raw_file(path.path()) {
                Ok(waveform_data) => {
                    let mut state = sim_state.write();
                    state.waveforms.clear();

                    for (idx, wf) in waveform_data.waveforms.iter().enumerate().skip(1) {
                        if let Some(values) = if wf.y_imag.is_some() {
                            Some(
                                wf.y.iter()
                                    .zip(wf.y_imag.as_ref().unwrap())
                                    .map(|(r, i)| (r * r + i * i).sqrt())
                                    .collect::<Vec<_>>(),
                            )
                        } else {
                            Some(wf.y.clone())
                        } {
                            if !wf.x.is_empty() && wf.x.len() == values.len() {
                                state.waveforms.push(WaveformData {
                                    name: wf.name.clone(),
                                    x: wf.x.clone(),
                                    y: values,
                                    color: crate::theme::Theme::trace_color_static(idx - 1)
                                        .to_string(),
                                    visible: true,
                                });
                            }
                        }
                    }

                    let trace_count = state.waveforms.len();
                    state.console_messages.push(ConsoleMessage::success(format!(
                        "Imported {} traces from: {}",
                        trace_count,
                        path.path().display()
                    )));
                }
                Err(e) => {
                    sim_state
                        .write()
                        .console_messages
                        .push(ConsoleMessage::error(format!(
                            "Failed to import .raw file: {}",
                            e
                        )));
                }
            }
        }
    }

    /// Save schematic file
    pub async fn save_schematic(
        mut schematic: Signal<SchematicState>,
        mut sim_state: Signal<SimulationState>,
    ) {
        let current_path = schematic.read().current_file.clone();
        let sch_state = schematic.read().clone();

        let save_path = if let Some(path) = current_path {
            Some(path)
        } else {
            rfd::AsyncFileDialog::new()
                .add_filter("RSpice Schematic", &["rsch"])
                .set_file_name("circuit.rsch")
                .save_file()
                .await
                .map(|f| f.path().to_path_buf())
        };

        if let Some(path) = save_path {
            match crate::state::schematic_file::save_schematic(&sch_state, &path) {
                Ok(_) => {
                    schematic.write().current_file = Some(path.clone());
                    sim_state
                        .write()
                        .console_messages
                        .push(ConsoleMessage::success(format!(
                            "Schematic saved: {}",
                            path.display()
                        )));
                }
                Err(e) => {
                    sim_state
                        .write()
                        .console_messages
                        .push(ConsoleMessage::error(format!(
                            "Failed to save schematic: {}",
                            e
                        )));
                }
            }
        }
    }

    /// Open schematic file
    pub async fn open_schematic(
        mut schematic: Signal<SchematicState>,
        mut sim_state: Signal<SimulationState>,
    ) {
        if let Some(file) = rfd::AsyncFileDialog::new()
            .add_filter("RSpice Schematic", &["rsch"])
            .pick_file()
            .await
        {
            let file_path = file.path().to_path_buf();
            match crate::state::schematic_file::load_schematic(&file_path) {
                Ok(mut loaded_state) => {
                    loaded_state.current_file = Some(file_path.clone());
                    schematic.set(loaded_state);
                    sim_state
                        .write()
                        .console_messages
                        .push(ConsoleMessage::success(format!(
                            "Schematic loaded: {}",
                            file_path.display()
                        )));
                }
                Err(e) => {
                    sim_state
                        .write()
                        .console_messages
                        .push(ConsoleMessage::error(format!(
                            "Failed to load schematic: {}",
                            e
                        )));
                }
            }
        }
    }
}

// ============================================================================
// Web stub implementations
// ============================================================================

#[cfg(target_arch = "wasm32")]
pub mod handlers {
    use super::*;

    /// Open netlist - not yet implemented for web
    pub async fn open_netlist(mut sim_state: Signal<SimulationState>) {
        sim_state
            .write()
            .console_messages
            .push(ConsoleMessage::warning(
                "File open not yet available in web version".to_string(),
            ));
    }

    /// Save netlist - not yet implemented for web
    pub async fn save_netlist(mut sim_state: Signal<SimulationState>) {
        sim_state
            .write()
            .console_messages
            .push(ConsoleMessage::warning(
                "File save not yet available in web version".to_string(),
            ));
    }

    /// Import raw - not yet implemented for web
    pub async fn import_raw(mut sim_state: Signal<SimulationState>) {
        sim_state
            .write()
            .console_messages
            .push(ConsoleMessage::warning(
                "Import not yet available in web version".to_string(),
            ));
    }

    /// Save schematic - not yet implemented for web
    pub async fn save_schematic(
        _schematic: Signal<SchematicState>,
        mut sim_state: Signal<SimulationState>,
    ) {
        sim_state
            .write()
            .console_messages
            .push(ConsoleMessage::warning(
                "Schematic save not yet available in web version".to_string(),
            ));
    }

    /// Open schematic - not yet implemented for web
    pub async fn open_schematic(
        _schematic: Signal<SchematicState>,
        mut sim_state: Signal<SimulationState>,
    ) {
        sim_state
            .write()
            .console_messages
            .push(ConsoleMessage::warning(
                "Schematic open not yet available in web version".to_string(),
            ));
    }
}

// Re-export handlers
pub use handlers::*;
