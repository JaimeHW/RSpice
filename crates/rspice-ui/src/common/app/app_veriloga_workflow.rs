use std::path::PathBuf;

use egui::Context;

use super::{ConsoleMessage, RSpiceApp, VERILOGA_LIBRARY_NAME, save_global_veriloga_library};

struct CompiledVerilogaPayload {
    module: crate::panels::CompiledModuleInfo,
    artifact: Option<rspice_veriloga::CompiledModel>,
    dependencies: Vec<PathBuf>,
}

fn take_compiled_veriloga_payload(
    dialog: &mut crate::panels::VerilogALoadDialogState,
) -> Option<CompiledVerilogaPayload> {
    let module = dialog.compiled_module.take()?;
    let artifact = dialog.compiled_artifact.take();
    let dependencies = dialog.compiled_dependencies.take().unwrap_or_default();
    Some(CompiledVerilogaPayload {
        module,
        artifact,
        dependencies,
    })
}

fn serialize_veriloga_ports(ports: &[String]) -> String {
    serde_json::to_string(ports).unwrap_or_else(|_| ports.join(","))
}

fn ensure_veriloga_user_library_exists(library_manager: &mut crate::state::LibraryManager) {
    if library_manager.get_library(VERILOGA_LIBRARY_NAME).is_none() {
        let mut library = crate::state::library_browser::Library::new(VERILOGA_LIBRARY_NAME);
        library.read_only = false;
        library_manager.add_library(library);
    }
}

fn update_veriloga_parameter_metadata(
    cell: &mut crate::state::library_browser::Cell,
    parameters: &[crate::panels::ParameterInfo],
) {
    cell.metadata.retain(|key, _| !key.starts_with("param_"));
    for parameter in parameters {
        cell.metadata.insert(
            format!("param_{}", parameter.name),
            parameter.default_value.clone(),
        );
    }
}

fn upsert_veriloga_cell_and_view(
    library: &mut crate::state::library_browser::Library,
    module: &crate::panels::CompiledModuleInfo,
    source_path_text: &str,
    serialized_ports: &str,
) {
    let cell = library.get_or_create_cell(&module.name);
    cell.description = format!(
        "Verilog-A model with {} terminals: {}",
        module.ports.len(),
        module.ports.join(", ")
    );
    cell.category = "Verilog-A".to_string();
    cell.metadata
        .insert("veriloga.module".to_string(), module.name.clone());
    cell.metadata.insert(
        "veriloga.source_path".to_string(),
        source_path_text.to_string(),
    );
    cell.metadata
        .insert("veriloga.ports".to_string(), serialized_ports.to_string());

    update_veriloga_parameter_metadata(cell, &module.parameters);

    let view = cell.views.entry("veriloga".to_string()).or_insert_with(|| {
        crate::state::library_browser::View::new(
            "veriloga",
            crate::state::library_browser::ViewType::VerilogA,
        )
    });
    view.view_type = crate::state::library_browser::ViewType::VerilogA;
    view.file_path = Some(module.source_path.clone());
    view.metadata
        .insert("veriloga.module".to_string(), module.name.clone());
    view.metadata.insert(
        "veriloga.source_path".to_string(),
        source_path_text.to_string(),
    );
    view.metadata
        .insert("veriloga.ports".to_string(), serialized_ports.to_string());
}

impl RSpiceApp {
    pub(super) fn process_veriloga_load_dialog(&mut self, ctx: &Context) {
        let veriloga_result = crate::panels::render_veriloga_load_dialog(
            ctx,
            &mut self.state.dialogs.veriloga_dialog,
        );
        if veriloga_result != crate::panels::VerilogADialogResult::AddToLibrary {
            return;
        }

        let Some(payload) = take_compiled_veriloga_payload(&mut self.state.dialogs.veriloga_dialog)
        else {
            self.state.push_user_message(ConsoleMessage::warning(
                "No compiled Verilog-A module available to add".to_string(),
            ));
            return;
        };
        let module = payload.module;
        let compiled_artifact = payload.artifact;
        let compiled_dependencies = payload.dependencies;

        let source_path_text = module.source_path.to_string_lossy().to_string();
        let serialized_ports = serialize_veriloga_ports(&module.ports);

        ensure_veriloga_user_library_exists(&mut self.state.library_manager);

        let add_ok = if let Some(library) = self
            .state
            .library_manager
            .get_library_mut(VERILOGA_LIBRARY_NAME)
        {
            upsert_veriloga_cell_and_view(library, &module, &source_path_text, &serialized_ports);
            true
        } else {
            false
        };

        if !add_ok {
            self.state.push_user_message(ConsoleMessage::error(format!(
                "Failed to add Verilog-A model '{}' to library '{}'",
                module.name, VERILOGA_LIBRARY_NAME
            )));
        } else {
            log::info!(
                "Registered Verilog-A model '{}' with {} terminals and {} parameters",
                module.name,
                module.ports.len(),
                module.parameters.len()
            );
            self.state.push_user_message(ConsoleMessage::info(format!(
                "Verilog-A model '{}' added to library with terminals: {}",
                module.name,
                module.ports.join(", ")
            )));
        }

        if let Some(compiled_model) = compiled_artifact {
            match rspice_core::register_precompiled_veriloga_model_with_dependencies(
                &module.source_path,
                &compiled_dependencies,
                compiled_model,
            ) {
                Ok(()) => {
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Registered Verilog-A compile cache for '{}' ({} dependency file(s))",
                        module.name,
                        compiled_dependencies.len()
                    )));
                }
                Err(err) => {
                    self.state
                        .push_user_message(ConsoleMessage::warning(format!(
                            "Verilog-A compile cache registration failed for '{}': {}",
                            module.name, err
                        )));
                }
            }
        } else {
            self.state.push_user_message(ConsoleMessage::warning(format!(
                "No compiled Verilog-A artifact available for '{}'; simulation will recompile if needed",
                module.name
            )));
        }

        if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
            log::warn!("Failed to persist global Verilog-A library: {}", err);
            self.state
                .push_user_message(ConsoleMessage::warning(format!(
                    "Failed to persist Verilog-A library: {}",
                    err
                )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_module(name: &str) -> crate::panels::CompiledModuleInfo {
        crate::panels::CompiledModuleInfo {
            name: name.to_string(),
            ports: vec!["p".to_string(), "n".to_string()],
            parameters: vec![
                crate::panels::ParameterInfo {
                    name: "gain".to_string(),
                    default_value: "10".to_string(),
                    min: None,
                    max: None,
                    description: None,
                },
                crate::panels::ParameterInfo {
                    name: "offset".to_string(),
                    default_value: "1m".to_string(),
                    min: None,
                    max: None,
                    description: None,
                },
            ],
            source_path: PathBuf::from("models/opamp.va"),
            internal_nodes: 0,
            num_variables: 0,
        }
    }

    #[test]
    fn test_serialize_veriloga_ports_as_json() {
        let ports = vec!["in".to_string(), "out".to_string()];
        assert_eq!(serialize_veriloga_ports(&ports), "[\"in\",\"out\"]");
    }

    #[test]
    fn test_take_compiled_veriloga_payload_clears_dialog_state() {
        let mut dialog = crate::panels::VerilogALoadDialogState::default();
        dialog.compiled_module = Some(test_module("clear_me"));
        dialog.compiled_dependencies = Some(vec![PathBuf::from("models/opamp.va")]);

        let payload = take_compiled_veriloga_payload(&mut dialog).unwrap();
        assert_eq!(payload.module.name, "clear_me".to_string());
        assert_eq!(payload.dependencies, vec![PathBuf::from("models/opamp.va")]);

        assert!(dialog.compiled_module.is_none());
        assert!(dialog.compiled_artifact.is_none());
        assert!(dialog.compiled_dependencies.is_none());
    }

    #[test]
    fn test_take_compiled_veriloga_payload_without_module_returns_none() {
        let mut dialog = crate::panels::VerilogALoadDialogState::default();
        dialog.compiled_dependencies = Some(vec![PathBuf::from("unused.va")]);

        assert!(take_compiled_veriloga_payload(&mut dialog).is_none());
        assert!(dialog.compiled_dependencies.is_some());
    }

    #[test]
    fn test_ensure_veriloga_user_library_exists_creates_writable_library() {
        let mut manager = crate::state::LibraryManager::new();
        assert!(manager.get_library(VERILOGA_LIBRARY_NAME).is_none());

        ensure_veriloga_user_library_exists(&mut manager);

        let library = manager.get_library(VERILOGA_LIBRARY_NAME).unwrap();
        assert!(!library.read_only);
    }

    #[test]
    fn test_upsert_veriloga_cell_and_view_populates_metadata() {
        let mut library = crate::state::library_browser::Library::new(VERILOGA_LIBRARY_NAME);
        let module = test_module("my_model");
        let ports = serialize_veriloga_ports(&module.ports);

        upsert_veriloga_cell_and_view(&mut library, &module, "models/opamp.va", ports.as_str());

        let cell = library.get_cell("my_model").unwrap();
        assert_eq!(
            cell.description,
            "Verilog-A model with 2 terminals: p, n".to_string()
        );
        assert_eq!(cell.category, "Verilog-A");
        assert_eq!(
            cell.metadata.get("veriloga.module"),
            Some(&"my_model".to_string())
        );
        assert_eq!(
            cell.metadata.get("veriloga.source_path"),
            Some(&"models/opamp.va".to_string())
        );
        assert_eq!(cell.metadata.get("veriloga.ports"), Some(&ports));
        assert_eq!(cell.metadata.get("param_gain"), Some(&"10".to_string()));
        assert_eq!(cell.metadata.get("param_offset"), Some(&"1m".to_string()));

        let view = cell.get_view("veriloga").unwrap();
        assert_eq!(
            view.view_type,
            crate::state::library_browser::ViewType::VerilogA
        );
        assert_eq!(
            view.file_path,
            Some(PathBuf::from("models/opamp.va".to_string()))
        );
        assert_eq!(
            view.metadata.get("veriloga.module"),
            Some(&"my_model".to_string())
        );
    }

    #[test]
    fn test_upsert_veriloga_cell_and_view_replaces_stale_param_metadata() {
        let mut library = crate::state::library_browser::Library::new(VERILOGA_LIBRARY_NAME);
        let mut existing = crate::state::library_browser::Cell::new("my_model");
        existing
            .metadata
            .insert("param_old".to_string(), "legacy".to_string());
        existing
            .metadata
            .insert("owner".to_string(), "analog-team".to_string());
        library.add_cell(existing);

        let module = test_module("my_model");
        let ports = serialize_veriloga_ports(&module.ports);
        upsert_veriloga_cell_and_view(&mut library, &module, "models/opamp.va", ports.as_str());

        let cell = library.get_cell("my_model").unwrap();
        assert!(cell.metadata.get("param_old").is_none());
        assert_eq!(cell.metadata.get("param_gain"), Some(&"10".to_string()));
        assert_eq!(cell.metadata.get("param_offset"), Some(&"1m".to_string()));
        assert_eq!(cell.metadata.get("owner"), Some(&"analog-team".to_string()));
    }
}
