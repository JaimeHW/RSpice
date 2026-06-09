use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn normalize_op_reference_node(name: &str) -> String {
        let trimmed = name.trim();
        if trimmed.contains('(') {
            Self::normalize_variable_name(trimmed)
        } else {
            Self::normalize_variable_name(&format!("v({trimmed})"))
        }
    }

    pub(in crate::testing::ngspice_runner) fn normalize_op_reference_branch(name: &str) -> String {
        Self::parse_current_probe(name).unwrap_or_else(|| name.trim().to_ascii_lowercase())
    }

    pub(in crate::testing::ngspice_runner) fn load_dc_op_reference(
        &self,
        cir_path: &Path,
    ) -> Result<Option<OpReference>, String> {
        let Some(reference_output) = self.load_reference_output(cir_path)? else {
            return Ok(None);
        };
        Ok(self.parse_dc_op_reference(&reference_output.content))
    }

    pub(in crate::testing::ngspice_runner) fn parse_dc_op_reference(
        &self,
        content: &str,
    ) -> Option<OpReference> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Section {
            None,
            Node,
            Source,
        }

        let mut reference = OpReference::default();
        let mut section = Section::None;

        for raw_line in content.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let normalized = trimmed.to_ascii_lowercase();
            if normalized == "node voltage" || normalized.starts_with("node ") {
                section = Section::Node;
                continue;
            }
            if normalized == "source current" || normalized.starts_with("source ") {
                section = Section::Source;
                continue;
            }
            if trimmed.starts_with('-')
                || normalized.starts_with("index ")
                || normalized.starts_with("initial transient solution")
            {
                continue;
            }
            if Self::is_non_data_op_section_header(&normalized) {
                section = Section::None;
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() != 2 {
                continue;
            }
            let Ok(value) = parts[parts.len() - 1].parse::<f64>() else {
                continue;
            };

            match section {
                Section::Node => {
                    if parts[0].contains("#branch") {
                        reference
                            .branch_currents
                            .insert(Self::normalize_op_reference_branch(parts[0]), value);
                    } else if !Self::is_internal_device_op_probe(parts[0]) {
                        reference
                            .node_voltages
                            .insert(Self::normalize_op_reference_node(parts[0]), value);
                    }
                }
                Section::Source => {
                    reference
                        .branch_currents
                        .insert(Self::normalize_op_reference_branch(parts[0]), value);
                }
                Section::None => {}
            }
        }

        if reference.node_voltages.is_empty() && reference.branch_currents.is_empty() {
            None
        } else {
            Some(reference)
        }
    }

    pub(in crate::testing::ngspice_runner) fn compare_dc_op_reference(
        &self,
        cir_path: &Path,
        result: &crate::SimulationResult,
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_dc_op_reference(cir_path)? else {
            return Ok(Vec::new());
        };

        let mut mismatches = Vec::new();
        let mut node_to_idx = HashMap::with_capacity(result.node_names.len() + 1);
        node_to_idx.insert("0".to_string(), 0usize);
        for (idx, name) in result.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        for (node_name, expected) in &reference.node_voltages {
            let actual = if let Some((pos, neg)) = Self::parse_voltage_probe(node_name) {
                let pos_idx = Self::resolve_node_index(&node_to_idx, &pos);
                let neg_idx = neg
                    .as_deref()
                    .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                    .unwrap_or(0);
                pos_idx.map(|p| {
                    result.node_voltages.get(p).copied().unwrap_or(0.0)
                        - if neg_idx == 0 {
                            0.0
                        } else {
                            result.node_voltages.get(neg_idx).copied().unwrap_or(0.0)
                        }
                })
            } else {
                None
            };

            match actual {
                Some(actual) => {
                    let absolute_tolerance =
                        self.dc_op_absolute_tolerance_floor(node_name, &reference, result);
                    if let Some(relative_error) =
                        self.compare_values_with_abs_tol(*expected, actual, absolute_tolerance)
                    {
                        mismatches.push(ValueMismatch {
                            x_value: 0.0,
                            node: node_name.clone(),
                            expected: *expected,
                            actual,
                            relative_error,
                        });
                    }
                }
                None => mismatches.push(ValueMismatch {
                    x_value: 0.0,
                    node: node_name.clone(),
                    expected: *expected,
                    actual: f64::NAN,
                    relative_error: f64::INFINITY,
                }),
            }
            if mismatches.len() >= self.config.max_mismatches {
                return Ok(mismatches);
            }
        }

        for (branch_name, expected) in &reference.branch_currents {
            match result.branch_current_named(branch_name) {
                Some(actual) => {
                    if let Some(relative_error) = self.compare_values(*expected, actual) {
                        mismatches.push(ValueMismatch {
                            x_value: 0.0,
                            node: format!("i({branch_name})"),
                            expected: *expected,
                            actual,
                            relative_error,
                        });
                    }
                }
                None => mismatches.push(ValueMismatch {
                    x_value: 0.0,
                    node: format!("i({branch_name})"),
                    expected: *expected,
                    actual: f64::NAN,
                    relative_error: f64::INFINITY,
                }),
            }
            if mismatches.len() >= self.config.max_mismatches {
                return Ok(mismatches);
            }
        }

        Ok(mismatches)
    }
}
