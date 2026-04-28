use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn load_transfer_function_reference(
        &self,
        cir_path: &Path,
        output: &str,
        input_source: &str,
    ) -> Result<Option<TransferFunctionReference>, String> {
        let out_path = cir_path.with_extension("out");
        if !out_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&out_path).map_err(|e| {
            format!(
                "Failed to read reference output '{}': {e}",
                out_path.display()
            )
        })?;
        let target_output = Self::normalize_variable_name(output);
        let target_input = input_source.trim().to_ascii_lowercase();
        Ok(self
            .parse_transfer_function_references(&content)
            .into_iter()
            .find(|reference| {
                let input_matches = reference
                    .input_source
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(&target_input));
                let output_matches = reference
                    .output_probe
                    .as_deref()
                    .is_none_or(|probe| Self::normalize_variable_name(probe) == target_output);
                input_matches && output_matches && reference.transfer_function.is_some()
            }))
    }

    pub(in crate::testing::ngspice_runner) fn parse_transfer_function_references(
        &self,
        content: &str,
    ) -> Vec<TransferFunctionReference> {
        let mut references = Vec::new();
        let mut current: Option<TransferFunctionReference> = None;

        let finalize = |references: &mut Vec<TransferFunctionReference>,
                        current: &mut Option<TransferFunctionReference>| {
            if let Some(reference) = current.take()
                && reference.transfer_function.is_some()
            {
                references.push(reference);
            }
        };

        for raw_line in content.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let normalized = trimmed.to_ascii_lowercase();
            if normalized.starts_with("transfer function information") {
                finalize(&mut references, &mut current);
                current = Some(TransferFunctionReference::default());
                continue;
            }

            let Some(active) = current.as_mut() else {
                continue;
            };

            let Some((lhs_raw, rhs_raw)) = trimmed.split_once('=') else {
                if !normalized.starts_with("warning") {
                    finalize(&mut references, &mut current);
                }
                continue;
            };

            let lhs = lhs_raw.trim().to_ascii_lowercase();
            let Ok(value) = rhs_raw.trim().parse::<f64>() else {
                continue;
            };

            if lhs == "transfer_function" {
                active.transfer_function = Some(value);
            } else if let Some(probe) = lhs.strip_prefix("output_impedance_at_") {
                active.output_probe = Some(probe.trim().to_string());
            } else if let Some(source) = lhs.strip_suffix("#input_impedance") {
                active.input_source = Some(source.trim().to_string());
            }
        }

        finalize(&mut references, &mut current);
        references
    }

    pub(in crate::testing::ngspice_runner) fn load_pz_reference(
        &self,
        cir_path: &Path,
    ) -> Result<Option<PzReference>, String> {
        let out_path = cir_path.with_extension("out");
        if !out_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&out_path).map_err(|e| {
            format!(
                "Failed to read reference output '{}': {e}",
                out_path.display()
            )
        })?;
        Ok(self.parse_pz_reference(&content))
    }

    pub(in crate::testing::ngspice_runner) fn parse_pz_reference(
        &self,
        content: &str,
    ) -> Option<PzReference> {
        let mut reference = PzReference::default();
        let mut current_cols: Vec<String> = Vec::new();

        for raw_line in content.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() || trimmed.starts_with('-') {
                continue;
            }

            if trimmed.to_ascii_lowercase().starts_with("index ") {
                current_cols = trimmed
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.to_ascii_lowercase())
                    .collect();
                continue;
            }

            if current_cols.is_empty() {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 1 + current_cols.len() * 2 {
                continue;
            }

            for (idx, col) in current_cols.iter().enumerate() {
                let Some(re_token) = parts.get(1 + idx * 2) else {
                    continue;
                };
                let Some(im_token) = parts.get(2 + idx * 2) else {
                    continue;
                };
                let re_token = re_token.trim_end_matches(',');
                let im_token = im_token.trim_end_matches(',');
                let Ok(re) = re_token.parse::<f64>() else {
                    continue;
                };
                let Ok(im) = im_token.parse::<f64>() else {
                    continue;
                };
                let value = crate::analysis::pole_zero::Complex::new(re, im);
                if col.starts_with("pole(") {
                    reference.poles.push(value);
                } else if col.starts_with("zero(") {
                    reference.zeros.push(value);
                } else if col == "all" {
                    reference.all.push(value);
                }
            }
        }

        if reference.poles.is_empty() && reference.zeros.is_empty() && reference.all.is_empty() {
            None
        } else {
            Some(reference)
        }
    }

    pub(in crate::testing::ngspice_runner) fn compare_pz_reference(
        &self,
        cir_path: &Path,
        result: &crate::analysis::PoleZeroResult,
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_pz_reference(cir_path)? else {
            return Ok(Vec::new());
        };

        let mut mismatches = Vec::new();

        let compare_complex_lists =
            |runner: &Self,
             label: &str,
             expected: &[crate::analysis::pole_zero::Complex],
             actual: &[crate::analysis::pole_zero::Complex],
             mismatches: &mut Vec<ValueMismatch>| {
                let n = expected.len().max(actual.len());
                for idx in 0..n {
                    let expected_value = expected.get(idx).copied();
                    let actual_value = actual.get(idx).copied();
                    match (expected_value, actual_value) {
                        (Some(expected), Some(actual)) => {
                            if let Some(relative_error) =
                                runner.compare_values(expected.re, actual.re)
                            {
                                mismatches.push(ValueMismatch {
                                    x_value: idx as f64,
                                    node: format!("{label}({}).re", idx + 1),
                                    expected: expected.re,
                                    actual: actual.re,
                                    relative_error,
                                });
                            }
                            if mismatches.len() >= runner.config.max_mismatches {
                                return;
                            }
                            if let Some(relative_error) =
                                runner.compare_values(expected.im, actual.im)
                            {
                                mismatches.push(ValueMismatch {
                                    x_value: idx as f64,
                                    node: format!("{label}({}).im", idx + 1),
                                    expected: expected.im,
                                    actual: actual.im,
                                    relative_error,
                                });
                            }
                        }
                        (Some(expected), None) => mismatches.push(ValueMismatch {
                            x_value: idx as f64,
                            node: format!("{label}({})", idx + 1),
                            expected: expected.re,
                            actual: f64::NAN,
                            relative_error: f64::INFINITY,
                        }),
                        (None, Some(actual)) => mismatches.push(ValueMismatch {
                            x_value: idx as f64,
                            node: format!("{label}({})", idx + 1),
                            expected: f64::NAN,
                            actual: actual.re,
                            relative_error: f64::INFINITY,
                        }),
                        (None, None) => {}
                    }
                    if mismatches.len() >= runner.config.max_mismatches {
                        return;
                    }
                }
            };

        let mut actual_poles = result.poles.clone();
        let mut actual_zeros = result.zeros.clone();
        actual_poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
        actual_zeros.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));

        if !reference.all.is_empty() {
            let mut actual_all = actual_poles.clone();
            actual_all.extend(actual_zeros.iter().copied());
            actual_all.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            let mut expected_all = reference.all.clone();
            expected_all.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            compare_complex_lists(self, "pz", &expected_all, &actual_all, &mut mismatches);
        } else {
            let mut expected_poles = reference.poles.clone();
            expected_poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            let mut expected_zeros = reference.zeros.clone();
            expected_zeros.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            compare_complex_lists(
                self,
                "pole",
                &expected_poles,
                &actual_poles,
                &mut mismatches,
            );
            if mismatches.len() < self.config.max_mismatches {
                compare_complex_lists(
                    self,
                    "zero",
                    &expected_zeros,
                    &actual_zeros,
                    &mut mismatches,
                );
            }
        }

        Ok(mismatches)
    }
}
