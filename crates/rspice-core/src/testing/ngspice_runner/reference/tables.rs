use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn load_reference_table_for_axis(
        &self,
        cir_path: &Path,
        axis_candidates: &[&str],
    ) -> Result<Option<ReferenceTable>, String> {
        let Some(reference_output) = self.load_reference_output(cir_path)? else {
            return Ok(None);
        };
        let Ok(tables) = self.parse_ngspice_output_tables(&reference_output.content) else {
            return Ok(None);
        };
        if tables.is_empty() {
            return Ok(None);
        }

        for candidate in axis_candidates {
            let target = Self::normalize_variable_name(candidate);
            let matching: Vec<ReferenceTable> = tables
                .iter()
                .filter(|table| Self::normalize_variable_name(&table.x_name) == target)
                .cloned()
                .collect();
            if !matching.is_empty() {
                let combined = Self::combine_reference_tables(target, matching);
                let unknown_nodes =
                    self.reference_table_unknown_voltage_nodes(cir_path, &combined)?;
                if !unknown_nodes.is_empty() {
                    log::warn!(
                        "Ignoring reference output {} because it mentions node(s) absent from '{}': {}",
                        reference_output.description,
                        cir_path.display(),
                        unknown_nodes.join(", ")
                    );
                    return Ok(None);
                }
                return Ok(Some(combined));
            }
        }

        Ok(None)
    }

    pub(in crate::testing::ngspice_runner) fn parse_ngspice_output_tables(
        &self,
        content: &str,
    ) -> Result<Vec<ReferenceTable>, String> {
        let mut tables: Vec<ReferenceTable> = Vec::new();
        let mut current_table = ReferenceTable::default();
        let mut in_data_section = false;
        let mut x_col_idx = 0usize;
        let mut value_col_start = 1usize;
        let mut current_vars: Vec<String> = Vec::new();

        let finalize_current = |tables: &mut Vec<ReferenceTable>,
                                table: &mut ReferenceTable,
                                vars: &mut Vec<String>| {
            if !table.variables.is_empty() {
                tables.push(std::mem::take(table));
            }
            vars.clear();
        };

        for raw_line in content.lines() {
            let trimmed = raw_line
                .trim_matches(|c: char| c.is_whitespace() || c == '\u{000c}')
                .trim();
            if trimmed.is_empty() {
                continue;
            }

            let lower = trimmed.to_ascii_lowercase();
            if lower.starts_with("index ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 {
                    finalize_current(&mut tables, &mut current_table, &mut current_vars);
                    current_table.x_name = parts[1].to_string();
                    current_vars = parts[2..].iter().map(|s| s.to_string()).collect();
                    x_col_idx = 1;
                    value_col_start = 2;
                    in_data_section = true;
                }
                continue;
            }
            if lower.starts_with("time ")
                || lower.starts_with("frequency ")
                || lower.starts_with("v-sweep ")
            {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2
                    && !Self::looks_like_ascii_plot_axis_header(parts[0], &parts[1..])
                {
                    finalize_current(&mut tables, &mut current_table, &mut current_vars);
                    current_table.x_name = parts[0].to_string();
                    current_vars = parts[1..].iter().map(|s| s.to_string()).collect();
                    x_col_idx = 0;
                    value_col_start = 1;
                    in_data_section = true;
                }
                continue;
            }
            if !in_data_section || trimmed.starts_with('-') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < value_col_start + current_vars.len() {
                continue;
            }

            let indexed_row = x_col_idx == 1 && value_col_start == 2;
            let complex_axis_row = indexed_row && parts.len() >= 3 + current_vars.len() * 2;
            let complex_row = indexed_row && parts.len() >= 2 + current_vars.len() * 2;

            let Some(x_value) = (if complex_row {
                parts
                    .get(1)
                    .and_then(|value| value.trim_end_matches(',').parse::<f64>().ok())
            } else {
                parts
                    .get(x_col_idx)
                    .and_then(|value| value.parse::<f64>().ok())
            }) else {
                continue;
            };

            if complex_row {
                let value_pair_start = if complex_axis_row { 3 } else { 2 };
                for (var_idx, var_name) in current_vars.iter().enumerate() {
                    let real_idx = value_pair_start + var_idx * 2;
                    let imag_idx = real_idx + 1;
                    let Some(re_str) = parts.get(real_idx) else {
                        continue;
                    };
                    let Some(im_str) = parts.get(imag_idx) else {
                        continue;
                    };
                    let Ok(re) = re_str.trim_end_matches(',').parse::<f64>() else {
                        continue;
                    };
                    let Ok(im) = im_str.trim_end_matches(',').parse::<f64>() else {
                        continue;
                    };
                    let complex = num_complex::Complex64::new(re, im);
                    let y_value = Self::evaluate_reference_complex_output(
                        var_name,
                        complex,
                        self.config.absolute_tolerance,
                    );

                    let entry = current_table.variables.entry(var_name.clone()).or_default();
                    entry.x.push(x_value);
                    entry.y.push(y_value);
                }
                continue;
            }

            for (var_idx, var_name) in current_vars.iter().enumerate() {
                let Some(val_str) = parts.get(value_col_start + var_idx) else {
                    continue;
                };
                let Ok(y_value) = (*val_str).parse::<f64>() else {
                    continue;
                };
                let entry = current_table.variables.entry(var_name.clone()).or_default();
                entry.x.push(x_value);
                entry.y.push(y_value);
            }
        }

        if !current_table.variables.is_empty() {
            tables.push(current_table);
        }

        if tables.is_empty() {
            Err("No tabular data found in ngspice output".to_string())
        } else {
            Ok(Self::merge_contiguous_reference_tables(tables))
        }
    }

    pub(in crate::testing::ngspice_runner) fn looks_like_ascii_plot_axis_header(
        axis: &str,
        vars: &[&str],
    ) -> bool {
        if vars.is_empty() {
            return false;
        }

        let normalized_axis = Self::normalize_variable_name(axis);
        if !matches!(normalized_axis.as_str(), "time" | "frequency" | "v-sweep") {
            return false;
        }

        vars.iter()
            .any(|token| Self::header_token_is_ascii_plot_tick(token))
    }

    pub(in crate::testing::ngspice_runner) fn header_token_is_ascii_plot_tick(token: &str) -> bool {
        let normalized = Self::normalize_variable_name(token);
        if normalized.is_empty() {
            return true;
        }

        normalized.parse::<f64>().is_ok()
            || crate::netlist::lexer::parse_spice_value(&normalized).is_ok()
    }

    pub(in crate::testing::ngspice_runner) fn merge_contiguous_reference_tables(
        tables: Vec<ReferenceTable>,
    ) -> Vec<ReferenceTable> {
        let mut merged: Vec<ReferenceTable> = Vec::new();

        for mut table in tables {
            if let Some(last) = merged.last_mut()
                && Self::can_merge_reference_tables(last, &table)
            {
                for (name, mut series) in std::mem::take(&mut table.variables) {
                    if let Some(dst) = last.variables.get_mut(&name) {
                        dst.x.append(&mut series.x);
                        dst.y.append(&mut series.y);
                    }
                }
                continue;
            }
            merged.push(table);
        }

        merged
    }

    pub(in crate::testing::ngspice_runner) fn can_merge_reference_tables(
        a: &ReferenceTable,
        b: &ReferenceTable,
    ) -> bool {
        if Self::normalize_variable_name(&a.x_name) != Self::normalize_variable_name(&b.x_name) {
            return false;
        }
        if a.variables.len() != b.variables.len() || a.variables.is_empty() {
            return false;
        }

        for (name, a_series) in &a.variables {
            let Some(b_series) = b.variables.get(name) else {
                return false;
            };
            if a_series.x.is_empty() || b_series.x.is_empty() {
                return false;
            }
            let a_last = a_series.x[a_series.x.len() - 1];
            let b_first = b_series.x[0];
            // Merge only continuation segments (page breaks), not independent analyses.
            if b_first < a_last {
                return false;
            }
        }
        true
    }
}
