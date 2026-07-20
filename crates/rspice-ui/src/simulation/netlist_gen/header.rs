use super::*;

impl<'a> NetlistGenerator<'a> {
    //-------------------------------------------------------------------------

    /// Generate netlist header
    pub(super) fn generate_header(&mut self) {
        self.lines.push("* RSpice Netlist".to_string());
        self.lines
            .push(format!("* Generated: {}", chrono_lite_timestamp()));
        self.lines
            .push(format!("* Components: {}", self.schematic.components.len()));
        self.lines.push(format!("* Nets: {}", self.nets.len()));
        self.lines.push(String::new());
    }

    /// Emit include directives for placed library cell instances.
    ///
    /// - Verilog-A bindings emit `.VERILOGA "path" [module]`
    /// - Other source-backed bindings emit `.include "path"`
    pub(super) fn generate_library_view_includes(&mut self) {
        let mut includes = std::collections::BTreeMap::<String, Option<String>>::new();
        let mut generic_includes = std::collections::BTreeMap::<String, Option<String>>::new();

        for component in &self.schematic.components {
            let binding = match self.effective_library_binding(component) {
                Ok(Some(binding)) => binding.clone(),
                Ok(None) => continue,
                Err(error) => {
                    self.errors.push(error);
                    continue;
                }
            };

            let Some(source_path) = binding.source_path.as_ref() else {
                continue;
            };

            let key = source_path.to_string_lossy().to_string();
            let configured_model_section = self
                .hierarchy
                .and_then(|hierarchy| {
                    hierarchy.execution_binding(&self.child_hierarchy_path(component))
                })
                .and_then(|binding| binding.model_section())
                .map(str::to_owned);
            if binding.view.eq_ignore_ascii_case("veriloga") {
                let model = binding
                    .module_name
                    .as_ref()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty());
                if model.is_none() {
                    self.warnings.push(format!(
                        "Cell instance '{}' ({}/{}/{}) has no explicit Verilog-A module name; falling back to cell name during netlisting",
                        component.name, binding.library, binding.cell, binding.view
                    ));
                }
                if let Some(existing_model) = includes.get(&key) {
                    if existing_model.as_deref() != model.as_deref() {
                        self.warnings.push(format!(
                            "Conflicting Verilog-A module bindings for include '{}': keeping '{}' and ignoring '{}'",
                            key,
                            existing_model.as_deref().unwrap_or("<none>"),
                            model.as_deref().unwrap_or("<none>")
                        ));
                    }
                } else {
                    includes.insert(key, model);
                }
            } else {
                if let Some(existing_section) = generic_includes.get(&key) {
                    if existing_section.as_deref() != configured_model_section.as_deref() {
                        self.errors.push(format!(
                            "Source '{}' has conflicting model-section bindings '{}' and '{}'; use one section per source in a configuration",
                            key,
                            existing_section.as_deref().unwrap_or("<entire source>"),
                            configured_model_section.as_deref().unwrap_or("<entire source>")
                        ));
                    }
                } else {
                    generic_includes.insert(key, configured_model_section);
                }
            }
        }

        if includes.is_empty() && generic_includes.is_empty() {
            return;
        }

        self.lines.push("* Library includes".to_string());
        for (path, model_section) in generic_includes {
            let quoted_path = Self::quote_path_for_netlist(&path);
            if let Some(model_section) = model_section {
                self.lines
                    .push(format!(".lib {} {}", quoted_path, model_section));
            } else {
                self.lines.push(format!(".include {}", quoted_path));
            }
        }
        for (path, model) in includes {
            let quoted_path = Self::quote_path_for_netlist(&path);
            if let Some(model_name) = model {
                self.lines
                    .push(format!(".VERILOGA {} {}", quoted_path, model_name));
            } else {
                self.lines.push(format!(".VERILOGA {}", quoted_path));
            }
        }
        self.lines.push(String::new());
    }
}
