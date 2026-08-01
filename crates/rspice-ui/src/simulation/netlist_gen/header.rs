//! The deck header.
//!
//! The title line and the analysis, option, and include directives that
//! precede the instance cards. The first line of a SPICE deck is the title,
//! never a comment, and this is where that is guaranteed.

use super::*;

impl<'a> NetlistGenerator<'a> {
    //-------------------------------------------------------------------------

    /// Generate netlist header
    pub(super) fn generate_header(&mut self) {
        // No generation timestamp: the executable source is content-addressed
        // and a prepared run is rebuilt and digest-compared before dispatch, so
        // a wall-clock line would expire an authorized run whenever the two
        // builds straddled a second boundary. Run timing is retained on the run
        // receipt, which is where wall-clock telemetry belongs.
        self.lines.push("* RSpice Netlist".to_string());
        self.lines
            .push(format!("* Components: {}", self.schematic.components.len()));
        self.lines.push(format!("* Nets: {}", self.nets.len()));
        if self.hierarchy_path == "/top"
            && let Some(hierarchy) = self.hierarchy
        {
            let globals = hierarchy.global_net_names();
            if !globals.is_empty() {
                self.lines.push(format!(".GLOBAL {}", globals.join(" ")));
            }
        }
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
                .map(str::to_owned)
                .or_else(|| binding.model_section.clone());
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

    /// `.VERILOGA` registers a source/runtime globally for the executable
    /// deck. Nested subcircuit generators discover their own dependencies,
    /// so collect, validate, deduplicate, and hoist those directives once
    /// after the full hierarchy has been assembled.
    pub(super) fn hoist_veriloga_directives(&mut self) {
        let mut directives = std::collections::BTreeMap::<String, (Option<String>, String)>::new();
        let mut retained = Vec::with_capacity(self.lines.len());
        for line in self.lines.drain(..) {
            let Some((path, model)) = parse_veriloga_directive(&line) else {
                retained.push(line);
                continue;
            };
            let model_key = model.as_deref().map(str::to_ascii_lowercase);
            match directives.get(&path) {
                Some((existing, _)) if *existing == model_key => {}
                Some((existing, _)) => self.errors.push(format!(
                    "Verilog-A source '{path}' has conflicting module selections '{}' and '{}'",
                    existing.as_deref().unwrap_or("<compiler selected>"),
                    model_key.as_deref().unwrap_or("<compiler selected>")
                )),
                None => {
                    directives.insert(path, (model_key, line.trim().to_owned()));
                }
            }
        }
        remove_empty_library_include_sections(&mut retained);
        if directives.is_empty() {
            self.lines = retained;
            return;
        }
        let insertion = retained
            .iter()
            .position(String::is_empty)
            .map_or(retained.len(), |index| index + 1);
        let mut section = Vec::with_capacity(directives.len() + 2);
        section.push("* Verilog-A sources".to_owned());
        section.extend(directives.into_values().map(|(_, line)| line));
        section.push(String::new());
        retained.splice(insertion..insertion, section);
        self.lines = retained;
    }
}

fn parse_veriloga_directive(line: &str) -> Option<(String, Option<String>)> {
    let (command, remainder) = take_token(line)?;
    if !command.eq_ignore_ascii_case(".veriloga") {
        return None;
    }
    let (path, remainder) = take_token(remainder)?;
    let remainder = remainder.trim();
    if remainder.is_empty() {
        return Some((path.to_owned(), None));
    }
    let (model, trailing) = take_token(remainder)?;
    trailing
        .trim()
        .is_empty()
        .then(|| (path.to_owned(), Some(model.to_owned())))
}

fn take_token(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();
    let first = input.chars().next()?;
    if matches!(first, '\'' | '"') {
        let quoted = &input[first.len_utf8()..];
        let mut escaped = false;
        for (index, character) in quoted.char_indices() {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == first {
                return Some((&quoted[..index], &quoted[index + character.len_utf8()..]));
            }
        }
        return None;
    }
    let end = input.find(char::is_whitespace).unwrap_or(input.len());
    (end > 0).then_some((&input[..end], &input[end..]))
}

fn remove_empty_library_include_sections(lines: &mut Vec<String>) {
    let mut index = 0;
    while index < lines.len() {
        if lines[index].trim() != "* Library includes" {
            index += 1;
            continue;
        }
        let end = (index + 1..lines.len())
            .find(|candidate| lines[*candidate].is_empty())
            .unwrap_or(lines.len());
        let has_include = lines[index + 1..end].iter().any(|line| {
            let command = line.split_whitespace().next().unwrap_or_default();
            matches!(
                command.to_ascii_lowercase().as_str(),
                ".include" | ".inc" | ".lib"
            )
        });
        if has_include {
            index = end.saturating_add(1);
        } else {
            let drain_end = usize::min(end.saturating_add(1), lines.len());
            lines.drain(index..drain_end);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_bound_library_section_uses_lib_directive() {
        let mut schematic = crate::state::SchematicState::default();
        let mut binding = crate::state::LibraryCellInstance::new("models", "nmos_18", "spice");
        binding.source_path = Some(std::path::PathBuf::from("C:/models/foundry.lib"));
        binding.model_section = Some("tt".to_owned());
        binding.terminal_order = vec!["d".to_owned(), "g".to_owned()];
        binding.interface_bound = true;
        schematic.components.push(
            crate::state::Component::new(
                1,
                crate::state::ComponentType::CellInstance,
                crate::state::Point::origin(),
            )
            .with_library_cell(binding),
        );
        let mut generator = NetlistGenerator::new(&schematic);

        generator.generate_library_view_includes();

        assert!(generator.errors.is_empty(), "{:?}", generator.errors);
        assert!(
            generator
                .lines
                .iter()
                .any(|line| line == ".lib \"C:/models/foundry.lib\" tt")
        );
    }

    #[test]
    fn hierarchical_veriloga_directives_are_hoisted_and_deduplicated_once() {
        let schematic = crate::state::SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        generator.lines = vec![
            "* RSpice Netlist".to_owned(),
            String::new(),
            "* Library includes".to_owned(),
            ".VERILOGA \"__rspice_project__/exact/model.va\" owned".to_owned(),
            String::new(),
            ".subckt child p n".to_owned(),
            "* Library includes".to_owned(),
            ".veriloga \"__rspice_project__/exact/model.va\" OWNED".to_owned(),
            String::new(),
            ".ends child".to_owned(),
        ];

        generator.hoist_veriloga_directives();

        assert!(generator.errors.is_empty(), "{:?}", generator.errors);
        assert_eq!(
            generator
                .lines
                .iter()
                .filter(|line| line
                    .trim_start()
                    .to_ascii_lowercase()
                    .starts_with(".veriloga"))
                .count(),
            1
        );
        let directive = generator
            .lines
            .iter()
            .position(|line| {
                line.trim_start()
                    .to_ascii_lowercase()
                    .starts_with(".veriloga")
            })
            .unwrap();
        let subcircuit = generator
            .lines
            .iter()
            .position(|line| line.starts_with(".subckt"))
            .unwrap();
        assert!(directive < subcircuit);
        assert!(
            !generator
                .lines
                .iter()
                .any(|line| line == "* Library includes")
        );
    }

    #[test]
    fn conflicting_module_selections_fail_closed() {
        let schematic = crate::state::SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        generator.lines = vec![
            "* RSpice Netlist".to_owned(),
            String::new(),
            ".veriloga \"models/device core.va\" first".to_owned(),
            ".veriloga \"models/device core.va\" second".to_owned(),
        ];

        generator.hoist_veriloga_directives();

        assert_eq!(generator.errors.len(), 1);
        assert!(generator.errors[0].contains("conflicting module selections"));
    }
}
