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

