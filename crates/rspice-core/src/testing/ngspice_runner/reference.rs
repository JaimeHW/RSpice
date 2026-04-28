//! Reference output parsing and comparison for ngspice regression tests.

use super::*;
impl TestRunner {
    pub(super) fn normalize_op_reference_node(name: &str) -> String {
        let trimmed = name.trim();
        if trimmed.contains('(') {
            Self::normalize_variable_name(trimmed)
        } else {
            Self::normalize_variable_name(&format!("v({trimmed})"))
        }
    }

    pub(super) fn normalize_op_reference_branch(name: &str) -> String {
        Self::parse_current_probe(name).unwrap_or_else(|| name.trim().to_ascii_lowercase())
    }

    pub(super) fn load_dc_op_reference(
        &self,
        cir_path: &Path,
    ) -> Result<Option<OpReference>, String> {
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
        Ok(self.parse_dc_op_reference(&content))
    }

    pub(super) fn parse_dc_op_reference(&self, content: &str) -> Option<OpReference> {
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

    pub(super) fn compare_dc_op_reference(
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

    pub(super) fn load_transfer_function_reference(
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

    pub(super) fn parse_transfer_function_references(
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

    pub(super) fn load_pz_reference(&self, cir_path: &Path) -> Result<Option<PzReference>, String> {
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

    pub(super) fn parse_pz_reference(&self, content: &str) -> Option<PzReference> {
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

    pub(super) fn compare_pz_reference(
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

    pub(super) fn compare_dc_sweep_reference(
        &self,
        cir_path: &Path,
        netlist: &Netlist,
        results: &[(Value, crate::SimulationResult)],
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["v-sweep"])? else {
            return Ok(Vec::new());
        };
        if results.is_empty() {
            return Ok(self.compare_reference_dataset(&reference, &[], |_| None));
        }

        let engine = self.create_dynamic_engine();
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| format!("Failed to build circuit for DC reference mapping: {e}"))?;
        let x_sim: Vec<f64> = results.iter().map(|(x, _)| *x).collect();
        let first = &results[0].1;
        let branch_names = if first.branch_names.iter().any(|name| !name.is_empty()) {
            first.branch_names.clone()
        } else {
            let names = circuit.branch_names_sorted();
            if names.is_empty() {
                Self::branch_probe_names_from_netlist(netlist)
            } else {
                names
            }
        };
        let mut node_to_idx = HashMap::with_capacity(first.node_names.len());
        for (idx, name) in first.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx);
        }
        let mut branch_to_idx = HashMap::with_capacity(branch_names.len());
        for (idx, name) in branch_names.iter().enumerate() {
            branch_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            Self::resolve_reference_series(var, &|expr| {
                let normalized = Self::normalize_variable_name(expr);
                if normalized == Self::normalize_variable_name(&reference.x_name) {
                    return None;
                }

                if let Some((n1, n2)) = Self::parse_voltage_probe(expr) {
                    let idx1 = Self::resolve_node_index(&node_to_idx, &n1)?;
                    let idx2 = n2
                        .as_deref()
                        .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                        .unwrap_or(0);
                    let series = results
                        .iter()
                        .map(|(_, r)| {
                            let v1 = r.node_voltages.get(idx1).copied().unwrap_or(0.0);
                            let v2 = r.node_voltages.get(idx2).copied().unwrap_or(0.0);
                            v1 - v2
                        })
                        .collect();
                    return Some(series);
                }

                if let Some(branch_name) = Self::parse_current_probe(expr) {
                    let branch_idx = branch_to_idx.get(&branch_name).copied().or_else(|| {
                        results
                            .iter()
                            .all(|(_, result)| result.branch_currents.len() == 1)
                            .then_some(0)
                    })?;
                    let series = results
                        .iter()
                        .map(|(_, r)| r.branch_currents.get(branch_idx).copied().unwrap_or(0.0))
                        .collect();
                    return Some(series);
                }

                None
            })
        }))
    }

    pub(super) fn compare_transient_reference(
        &self,
        cir_path: &Path,
        netlist: &Netlist,
        result: &crate::engine::TransientResult,
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["time"])? else {
            return Ok(Vec::new());
        };

        let x_sim = result.time.clone();
        let mut node_to_idx = HashMap::with_capacity(result.node_names.len() + 1);
        node_to_idx.insert("0".to_string(), 0usize);
        for (idx, name) in result.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx + 1);
        }

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            Self::resolve_reference_series(var, &|expr| {
                if let Some((n1, n2)) = Self::parse_voltage_probe(expr) {
                    let idx1 = Self::resolve_node_index(&node_to_idx, &n1)?;
                    let idx2 = n2
                        .as_deref()
                        .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                        .unwrap_or(0);

                    let w1 = Self::transient_node_waveform(result, idx1);
                    let w2 = Self::transient_node_waveform(result, idx2);
                    let series = w1
                        .iter()
                        .zip(w2.iter())
                        .map(|(a, b)| a - b)
                        .collect::<Vec<_>>();
                    return Some(series);
                }

                if let Some(branch_name) = Self::parse_current_probe(expr) {
                    let branch_idx = result
                        .branch_names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(&branch_name))?;
                    return result.branch_currents.get(branch_idx).cloned();
                }

                Self::resolve_transient_device_series(netlist, &node_to_idx, result, expr)
            })
        }))
    }

    pub(super) fn compare_ac_reference(
        &self,
        cir_path: &Path,
        netlist: &Netlist,
        results: &[crate::analysis::AcResult],
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["frequency"])? else {
            return Ok(Vec::new());
        };
        if results.is_empty() {
            return Ok(self.compare_reference_dataset(&reference, &[], |_| None));
        }

        let engine = self.create_dynamic_engine();
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| format!("Failed to build circuit for AC reference mapping: {e}"))?;

        let mut node_to_idx = HashMap::new();
        node_to_idx.insert("0".to_string(), 0usize);
        for (idx, name) in circuit.node_names_sorted().iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx + 1);
        }
        let mut branch_to_idx = HashMap::new();
        for (idx, name) in results[0].branch_names.iter().enumerate() {
            branch_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        let x_sim: Vec<f64> = results.iter().map(|r| r.frequency).collect();

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            let normalized = Self::normalize_variable_name(var);

            match Self::parse_ac_probe(&normalized) {
                Some(AcProbe::Voltage {
                    func,
                    node_pos,
                    node_neg,
                }) => {
                    let idx_a = Self::resolve_node_index(&node_to_idx, &node_pos)?;
                    let idx_b = node_neg
                        .as_deref()
                        .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                        .unwrap_or(0);

                    let series = results
                        .iter()
                        .map(|r| {
                            let va = if idx_a == 0 {
                                num_complex::Complex64::new(0.0, 0.0)
                            } else {
                                r.voltages
                                    .get(idx_a - 1)
                                    .copied()
                                    .unwrap_or(num_complex::Complex64::new(0.0, 0.0))
                            };
                            let vb = if idx_b == 0 {
                                num_complex::Complex64::new(0.0, 0.0)
                            } else {
                                r.voltages
                                    .get(idx_b - 1)
                                    .copied()
                                    .unwrap_or(num_complex::Complex64::new(0.0, 0.0))
                            };
                            Self::evaluate_ac_complex_value(
                                func,
                                va - vb,
                                self.config.absolute_tolerance,
                            )
                        })
                        .collect();
                    Some(series)
                }
                Some(AcProbe::Current { func, branch }) => {
                    let branch_idx = *branch_to_idx.get(&branch)?;
                    let series = results
                        .iter()
                        .map(|r| {
                            let current = r
                                .currents
                                .get(branch_idx)
                                .copied()
                                .unwrap_or(num_complex::Complex64::new(0.0, 0.0));
                            Self::evaluate_ac_complex_value(
                                func,
                                current,
                                self.config.absolute_tolerance,
                            )
                        })
                        .collect();
                    Some(series)
                }
                None => None,
            }
        }))
    }

    pub(super) fn compare_noise_reference(
        &self,
        cir_path: &Path,
        results: &[crate::analysis::NoiseResult],
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["frequency"])? else {
            return Ok(Vec::new());
        };
        if results.is_empty() {
            return Ok(self.compare_reference_dataset(&reference, &[], |_| None));
        }

        let x_sim: Vec<f64> = results.iter().map(|point| point.frequency).collect();
        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            let normalized = Self::normalize_variable_name(var);
            if matches!(
                normalized.as_str(),
                "onoise_spectrum" | "onoise" | "v(onoise_spectrum)"
            ) || normalized.starts_with("v(onoise_spectr")
            {
                return Some(
                    results
                        .iter()
                        .map(|point| point.output_noise_density)
                        .collect(),
                );
            }
            if matches!(
                normalized.as_str(),
                "inoise_spectrum" | "inoise" | "v(inoise_spectrum)"
            ) || normalized.starts_with("v(inoise_spectr")
            {
                return Some(
                    results
                        .iter()
                        .map(|point| point.input_referred_density)
                        .collect(),
                );
            }

            None
        }))
    }

    pub(super) fn transient_node_waveform(
        result: &crate::engine::TransientResult,
        idx: usize,
    ) -> Vec<f64> {
        if idx == 0 {
            vec![0.0; result.time.len()]
        } else {
            result
                .voltages
                .get(idx - 1)
                .cloned()
                .unwrap_or_else(|| vec![0.0; result.time.len()])
        }
    }

    pub(super) fn resolve_transient_device_series(
        netlist: &Netlist,
        node_to_idx: &HashMap<String, usize>,
        result: &crate::engine::TransientResult,
        expr: &str,
    ) -> Option<Vec<f64>> {
        let normalized = Self::normalize_variable_name(expr);
        if !(normalized.starts_with('@') && normalized.ends_with(']')) {
            return None;
        }

        let (device_name, quantity) = normalized[1..].split_once('[')?;
        let quantity = quantity.strip_suffix(']')?;
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(device_name))?;
        let (pos_node, neg_node) = Self::resolve_device_voltage_nodes(element, quantity)?;
        let pos_idx = Self::resolve_node_index(node_to_idx, &pos_node)?;
        let neg_idx = Self::resolve_node_index(node_to_idx, &neg_node)?;
        let pos = Self::transient_node_waveform(result, pos_idx);
        let neg = Self::transient_node_waveform(result, neg_idx);
        Some(
            pos.into_iter()
                .zip(neg)
                .map(|(pos, neg)| pos - neg)
                .collect(),
        )
    }

    pub(super) fn resolve_device_voltage_nodes(
        element: &crate::netlist::Element,
        quantity: &str,
    ) -> Option<(String, String)> {
        let quantity = Self::normalize_variable_name(quantity);
        match &element.kind {
            crate::netlist::ElementKind::Mosfet { .. } => match quantity.as_str() {
                "vds" => Some((
                    element.nodes.first()?.clone(),
                    element.nodes.get(2)?.clone(),
                )),
                "vgs" => Some((element.nodes.get(1)?.clone(), element.nodes.get(2)?.clone())),
                "vbs" => Some((element.nodes.get(3)?.clone(), element.nodes.get(2)?.clone())),
                "vgd" => Some((
                    element.nodes.get(1)?.clone(),
                    element.nodes.first()?.clone(),
                )),
                "vbd" => Some((
                    element.nodes.get(3)?.clone(),
                    element.nodes.first()?.clone(),
                )),
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn resolve_reference_series<F>(expr: &str, direct: &F) -> Option<Vec<f64>>
    where
        F: Fn(&str) -> Option<Vec<f64>>,
    {
        let normalized = Self::normalize_variable_name(expr);
        if normalized.is_empty() {
            return None;
        }
        if let Some(series) = direct(&normalized) {
            return Some(series);
        }

        if let Some(inner) = normalized
            .strip_prefix("abs(")
            .and_then(|s| s.strip_suffix(')'))
        {
            return Some(
                Self::resolve_reference_series(inner, direct)?
                    .into_iter()
                    .map(f64::abs)
                    .collect(),
            );
        }

        if let Some(inner) = normalized.strip_prefix('-') {
            return Some(
                Self::resolve_reference_series(inner, direct)?
                    .into_iter()
                    .map(|value| -value)
                    .collect(),
            );
        }

        if let Some((lhs, op, rhs)) = Self::split_reference_binary_expression(&normalized) {
            if let Some(scalar) = Self::parse_reference_scalar(rhs) {
                let series = Self::resolve_reference_series(lhs, direct)?;
                return Some(match op {
                    '*' => series.into_iter().map(|value| value * scalar).collect(),
                    '/' => series.into_iter().map(|value| value / scalar).collect(),
                    _ => return None,
                });
            }
            if let Some(scalar) = Self::parse_reference_scalar(lhs) {
                let series = Self::resolve_reference_series(rhs, direct)?;
                return Some(match op {
                    '*' => series.into_iter().map(|value| scalar * value).collect(),
                    '/' => series.into_iter().map(|value| scalar / value).collect(),
                    _ => return None,
                });
            }
        }

        None
    }

    pub(super) fn split_reference_binary_expression(expr: &str) -> Option<(&str, char, &str)> {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        for (idx, ch) in expr.char_indices() {
            match ch {
                '(' => paren_depth += 1,
                ')' => paren_depth = paren_depth.saturating_sub(1),
                '[' => bracket_depth += 1,
                ']' => bracket_depth = bracket_depth.saturating_sub(1),
                '*' | '/' if paren_depth == 0 && bracket_depth == 0 => {
                    let lhs = &expr[..idx];
                    let rhs = &expr[idx + ch.len_utf8()..];
                    if !lhs.is_empty() && !rhs.is_empty() {
                        return Some((lhs, ch, rhs));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub(super) fn parse_reference_scalar(expr: &str) -> Option<f64> {
        crate::netlist::lexer::parse_spice_value(expr)
            .ok()
            .or_else(|| expr.parse::<f64>().ok())
    }

    pub(super) fn is_voltage_probe_name(expr: &str) -> bool {
        Self::parse_voltage_probe(expr).is_some()
    }

    pub(super) fn is_current_probe_name(expr: &str) -> bool {
        Self::parse_current_probe(expr).is_some()
    }

    pub(super) fn reference_expr_contains_probe(
        expr: &str,
        is_probe_name: fn(&str) -> bool,
    ) -> bool {
        if expr.is_empty() {
            return false;
        }
        if is_probe_name(expr) {
            return true;
        }
        if let Some(inner) = expr
            .strip_prefix("abs(")
            .and_then(|candidate| candidate.strip_suffix(')'))
            && Self::reference_expr_contains_probe(inner, is_probe_name)
        {
            return true;
        }
        if let Some(inner) = expr.strip_prefix('-').or_else(|| expr.strip_prefix('+'))
            && Self::reference_expr_contains_probe(inner, is_probe_name)
        {
            return true;
        }
        if let Some((lhs, _, rhs)) = Self::split_reference_binary_expression(expr) {
            if Self::parse_reference_scalar(lhs).is_some()
                && Self::reference_expr_contains_probe(rhs, is_probe_name)
            {
                return true;
            }
            if Self::parse_reference_scalar(rhs).is_some()
                && Self::reference_expr_contains_probe(lhs, is_probe_name)
            {
                return true;
            }
        }
        false
    }

    pub(super) fn reference_expr_contains_voltage_probe(expr: &str) -> bool {
        let normalized = Self::normalize_variable_name(expr);
        Self::reference_expr_contains_probe(&normalized, Self::is_voltage_probe_name)
    }

    pub(super) fn reference_expr_contains_current_probe(expr: &str) -> bool {
        let normalized = Self::normalize_variable_name(expr);
        Self::reference_expr_contains_probe(&normalized, Self::is_current_probe_name)
    }

    pub(super) fn compare_reference_dataset<F>(
        &self,
        reference: &ReferenceTable,
        x_sim: &[f64],
        resolver: F,
    ) -> Vec<ValueMismatch>
    where
        F: Fn(&str) -> Option<Vec<f64>>,
    {
        let mut mismatches = Vec::new();
        let sim_monotonic = Self::is_monotonic_axis(x_sim);

        for (var, expected_series) in &reference.variables {
            if Self::normalize_variable_name(var)
                == Self::normalize_variable_name(&reference.x_name)
            {
                continue;
            }
            let normalized_var = Self::normalize_variable_name(var);
            let phase_probe = normalized_var.starts_with("ph(")
                || normalized_var.starts_with("vp(")
                || normalized_var.starts_with("ip(");
            let current_phase_probe = matches!(
                Self::parse_ac_probe(&normalized_var),
                Some(AcProbe::Current {
                    func: "ph" | "ip",
                    ..
                })
            );
            let degrees_phase_probe =
                normalized_var.starts_with("vp(") || normalized_var.starts_with("ip(");

            let Some(actual_series) = resolver(var) else {
                mismatches.push(ValueMismatch {
                    x_value: expected_series.x.first().copied().unwrap_or(0.0),
                    node: var.clone(),
                    expected: f64::NAN,
                    actual: f64::NAN,
                    relative_error: f64::INFINITY,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return mismatches;
                }
                continue;
            };
            if actual_series.is_empty() || x_sim.is_empty() {
                mismatches.push(ValueMismatch {
                    x_value: expected_series.x.first().copied().unwrap_or(0.0),
                    node: var.clone(),
                    expected: f64::NAN,
                    actual: f64::NAN,
                    relative_error: f64::INFINITY,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return mismatches;
                }
                continue;
            }

            let absolute_tolerance =
                self.series_absolute_tolerance_floor(var, expected_series, &actual_series);
            let ref_monotonic = Self::is_monotonic_axis(&expected_series.x);
            if sim_monotonic && ref_monotonic {
                for (&x_ref, &expected) in expected_series.x.iter().zip(expected_series.y.iter()) {
                    let Some(actual) = Self::interpolate_series(x_sim, &actual_series, x_ref)
                    else {
                        mismatches.push(ValueMismatch {
                            x_value: x_ref,
                            node: var.clone(),
                            expected,
                            actual: f64::NAN,
                            relative_error: f64::INFINITY,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                        continue;
                    };
                    if let Some(relative_error) = if phase_probe {
                        self.compare_phase_values_with_abs_tol(
                            expected,
                            actual,
                            absolute_tolerance,
                            current_phase_probe,
                            degrees_phase_probe,
                        )
                    } else {
                        self.compare_values_with_abs_tol(expected, actual, absolute_tolerance)
                    } {
                        mismatches.push(ValueMismatch {
                            x_value: x_ref,
                            node: var.clone(),
                            expected,
                            actual,
                            relative_error,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                    }
                }
            } else {
                // Multi-dimensional sweeps (e.g. .dc src1 ... src2 ...) produce
                // non-monotonic x-axes. For these traces compare by row index.
                let n = expected_series.y.len().max(actual_series.len());
                for i in 0..n {
                    let Some(&expected) = expected_series.y.get(i) else {
                        mismatches.push(ValueMismatch {
                            x_value: x_sim.get(i).copied().unwrap_or(i as f64),
                            node: var.clone(),
                            expected: f64::NAN,
                            actual: actual_series.get(i).copied().unwrap_or(f64::NAN),
                            relative_error: f64::INFINITY,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                        continue;
                    };
                    let Some(&actual) = actual_series.get(i) else {
                        mismatches.push(ValueMismatch {
                            x_value: expected_series.x.get(i).copied().unwrap_or(i as f64),
                            node: var.clone(),
                            expected,
                            actual: f64::NAN,
                            relative_error: f64::INFINITY,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                        continue;
                    };
                    let x_value = expected_series.x.get(i).copied().unwrap_or(i as f64);
                    if let Some(relative_error) = if phase_probe {
                        self.compare_phase_values_with_abs_tol(
                            expected,
                            actual,
                            absolute_tolerance,
                            current_phase_probe,
                            degrees_phase_probe,
                        )
                    } else {
                        self.compare_values_with_abs_tol(expected, actual, absolute_tolerance)
                    } {
                        mismatches.push(ValueMismatch {
                            x_value,
                            node: var.clone(),
                            expected,
                            actual,
                            relative_error,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                    }
                }
            }
        }

        mismatches
    }

    #[inline]
    pub(super) fn is_monotonic_axis(x: &[f64]) -> bool {
        if x.len() < 2 {
            return true;
        }
        let mut non_decreasing = true;
        let mut non_increasing = true;
        for pair in x.windows(2) {
            if pair[1] < pair[0] {
                non_decreasing = false;
            }
            if pair[1] > pair[0] {
                non_increasing = false;
            }
            if !non_decreasing && !non_increasing {
                return false;
            }
        }
        true
    }

    pub(super) fn load_reference_table_for_axis(
        &self,
        cir_path: &Path,
        axis_candidates: &[&str],
    ) -> Result<Option<ReferenceTable>, String> {
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
        let Ok(tables) = self.parse_ngspice_output_tables(&content) else {
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
                        "Ignoring stale reference output '{}' because it mentions node(s) absent from '{}': {}",
                        out_path.display(),
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

    pub(super) fn parse_ngspice_output_tables(
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

    pub(super) fn looks_like_ascii_plot_axis_header(axis: &str, vars: &[&str]) -> bool {
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

    pub(super) fn header_token_is_ascii_plot_tick(token: &str) -> bool {
        let normalized = Self::normalize_variable_name(token);
        if normalized.is_empty() {
            return true;
        }

        normalized.parse::<f64>().is_ok()
            || crate::netlist::lexer::parse_spice_value(&normalized).is_ok()
    }

    pub(super) fn merge_contiguous_reference_tables(
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

    pub(super) fn can_merge_reference_tables(a: &ReferenceTable, b: &ReferenceTable) -> bool {
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

    pub(super) fn normalize_variable_name(name: &str) -> String {
        name.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    }

    pub(super) fn default_transient_max_step(tstep: f64, tstop: f64, tstart: f64) -> f64 {
        let analysis_window = (tstop - tstart).max(0.0);
        let fallback_window = if analysis_window > 0.0 {
            analysis_window
        } else {
            tstop.abs().max(tstep.abs())
        };
        let window_limit = fallback_window / 50.0;

        if tstep > 0.0 {
            tstep.min(window_limit)
        } else {
            window_limit
        }
    }

    pub(super) fn evaluate_ac_complex_value(
        func: &str,
        value: num_complex::Complex64,
        abs_tol: f64,
    ) -> f64 {
        match func {
            "mag" | "vm" | "v" | "i" => value.norm(),
            "vr" | "ir" => value.re,
            "vi" | "ii" => value.im,
            "ph" => value.arg(),
            "vp" | "ip" => value.arg().to_degrees(),
            "db" | "vdb" => {
                let mag = value.norm().max(abs_tol);
                20.0 * mag.log10()
            }
            _ => value.norm(),
        }
    }

    pub(super) fn evaluate_reference_complex_output(
        var_name: &str,
        value: num_complex::Complex64,
        abs_tol: f64,
    ) -> f64 {
        let normalized = Self::normalize_variable_name(var_name);
        match Self::parse_ac_probe(&normalized) {
            Some(AcProbe::Voltage { func, .. }) | Some(AcProbe::Current { func, .. }) => {
                Self::evaluate_ac_complex_value(func, value, abs_tol)
            }
            None => value.re,
        }
    }

    pub(super) fn parse_voltage_probe(var: &str) -> Option<(String, Option<String>)> {
        let normalized = Self::normalize_variable_name(var);
        if !(normalized.starts_with("v(") && normalized.ends_with(')')) {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        if let Some((a, b)) = inner.split_once(',') {
            Some((a.to_string(), Some(b.to_string())))
        } else {
            Some((inner.to_string(), None))
        }
    }

    pub(super) fn parse_current_probe(var: &str) -> Option<String> {
        let normalized = Self::normalize_variable_name(var);
        if normalized.starts_with("i(") && normalized.ends_with(')') {
            let inner = &normalized[2..normalized.len() - 1];
            return if inner.is_empty() {
                None
            } else {
                Some(inner.to_string())
            };
        }
        normalized
            .strip_suffix("#branch")
            .and_then(|name| (!name.is_empty()).then(|| name.to_string()))
    }

    pub(super) fn branch_probe_names_from_netlist(netlist: &Netlist) -> Vec<String> {
        netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                crate::netlist::ElementKind::Inductor { .. }
                | crate::netlist::ElementKind::JilesAthertonInductor { .. }
                | crate::netlist::ElementKind::VoltageSource(_)
                | crate::netlist::ElementKind::Ccvs { .. }
                | crate::netlist::ElementKind::BehavioralVoltage { .. } => {
                    Some(element.name.clone())
                }
                _ => None,
            })
            .collect()
    }

    pub(super) fn parse_ac_probe(var: &str) -> Option<AcProbe> {
        let normalized = Self::normalize_variable_name(var);
        for func in [
            "vdb", "db", "vm", "mag", "vr", "ir", "vi", "ii", "vp", "ip", "ph",
        ] {
            let prefix = format!("{func}(");
            if normalized.starts_with(&prefix) && normalized.ends_with(')') {
                let inner = &normalized[prefix.len()..normalized.len() - 1];
                if let Some((node_pos, node_neg)) = Self::parse_voltage_probe(inner) {
                    return Some(AcProbe::Voltage {
                        func,
                        node_pos,
                        node_neg,
                    });
                }
                if let Some(branch) = Self::parse_current_probe(inner) {
                    return Some(AcProbe::Current { func, branch });
                }
                return None;
            }
        }

        if let Some((node_pos, node_neg)) = Self::parse_voltage_probe(&normalized) {
            return Some(AcProbe::Voltage {
                func: "v",
                node_pos,
                node_neg,
            });
        }
        if let Some(branch) = Self::parse_current_probe(&normalized) {
            return Some(AcProbe::Current { func: "i", branch });
        }

        None
    }

    pub(super) fn combine_reference_tables(
        axis_name: String,
        tables: Vec<ReferenceTable>,
    ) -> ReferenceTable {
        let mut combined = ReferenceTable {
            x_name: axis_name,
            variables: BTreeMap::new(),
        };

        for table in tables {
            for (name, mut series) in table.variables {
                if let Some(existing) = combined.variables.get_mut(&name) {
                    existing.x.append(&mut series.x);
                    existing.y.append(&mut series.y);
                } else {
                    combined.variables.insert(name, series);
                }
            }
        }

        combined
    }

    pub(super) fn resolve_node_index(
        node_to_idx: &HashMap<String, usize>,
        node: &str,
    ) -> Option<usize> {
        if let Some(idx) = node_to_idx.get(&node.to_ascii_lowercase()) {
            return Some(*idx);
        }
        node.parse::<usize>().ok()
    }

    pub(super) fn reference_node_exists(node_to_idx: &HashMap<String, usize>, node: &str) -> bool {
        node_to_idx.contains_key(&node.to_ascii_lowercase())
    }

    pub(super) fn interpolate_series(x: &[f64], y: &[f64], x_query: f64) -> Option<f64> {
        if x.len() != y.len() || x.is_empty() {
            return None;
        }
        if x.len() == 1 {
            return Some(y[0]);
        }

        let ascending = x[0] <= x[x.len() - 1];
        let axis_scale = x[0].abs().max(x[x.len() - 1].abs()).max(x_query.abs());
        let range_eps = (8e-15 * axis_scale).max(1e-18);
        let in_range = if ascending {
            x_query >= x[0] - range_eps && x_query <= x[x.len() - 1] + range_eps
        } else {
            x_query <= x[0] + range_eps && x_query >= x[x.len() - 1] - range_eps
        };
        if !in_range {
            return None;
        }

        let mut lo = 0usize;
        let mut hi = x.len() - 1;
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let go_right = if ascending {
                x[mid] < x_query
            } else {
                x[mid] > x_query
            };
            if go_right {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        let x0 = x[lo];
        let x1 = x[hi];
        let y0 = y[lo];
        let y1 = y[hi];
        let local_scale = x0.abs().max(x1.abs()).max(x_query.abs());
        let snap_eps = (8e-15 * local_scale).max(1e-18);
        if (x_query - x0).abs() <= snap_eps {
            return Some(y0);
        }
        if (x_query - x1).abs() <= snap_eps {
            return Some(y1);
        }
        if (x1 - x0).abs() <= f64::EPSILON {
            return Some(y0);
        }
        let t = (x_query - x0) / (x1 - x0);
        Some(y0 + t * (y1 - y0))
    }

    pub(super) fn series_absolute_tolerance_floor(
        &self,
        var: &str,
        expected_series: &ReferenceSeries,
        actual_series: &[f64],
    ) -> f64 {
        let mut floor = self.config.absolute_tolerance;
        let normalized = Self::normalize_variable_name(var);
        let expected_scale = expected_series
            .y
            .iter()
            .copied()
            .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
        let actual_scale = actual_series
            .iter()
            .copied()
            .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
        let series_scale = expected_scale.max(actual_scale);
        if normalized.starts_with("ph(") {
            // For radian phase probes, compare with an angle-domain floor derived
            // from the trace scale. This avoids over-penalizing tiny imaginary
            // residue on near-zero phase currents while still capping tolerance.
            floor = floor.max((series_scale * 0.7).clamp(2e-3, 5e-2));
        } else if normalized.starts_with("db(") || normalized.starts_with("vdb(") {
            // dB-domain probes are already logarithmic. When the trace crosses
            // 0 dB, relative error becomes a poor metric even for sub-0.1%
            // linear-magnitude differences, so keep a tight absolute floor in
            // the logarithmic domain itself.
            floor = floor.max((series_scale * 2e-4).clamp(5e-3, 2e-1));
        } else if normalized.starts_with("vp(") || normalized.starts_with("ip(") {
            floor = floor.max((series_scale * 0.7).clamp(0.12, 3.0));
        }
        if Self::reference_expr_contains_voltage_probe(var) {
            // Use a small waveform-scale floor for direct voltage probes so
            // rail-scale switching traces are compared by meaningful absolute
            // error when interpolation lands near zero crossings.
            floor = floor.max(series_scale * 1e-4);
        } else if Self::reference_expr_contains_current_probe(var) {
            // Keep direct current probes strict by default, but when the trace
            // genuinely spans both polarities, use a modest full-scale floor so
            // sweep points around the sign-change boundary are not dominated by
            // relative error on effectively zero current.
            let spans_zero = Self::series_spans_zero(&expected_series.y)
                || Self::series_spans_zero(actual_series);
            let current_floor_scale = if spans_zero { 1e-4 } else { 2e-6 };
            floor = floor.max(series_scale * current_floor_scale);
        }
        floor
    }

    pub(super) fn series_spans_zero(values: &[f64]) -> bool {
        let has_positive = values.iter().any(|&value| value > 0.0);
        let has_negative = values.iter().any(|&value| value < 0.0);
        has_positive && has_negative
    }

    pub(super) fn dc_op_absolute_tolerance_floor(
        &self,
        probe: &str,
        reference: &OpReference,
        result: &crate::SimulationResult,
    ) -> f64 {
        let mut floor = self.config.absolute_tolerance;
        if Self::parse_voltage_probe(probe).is_some() {
            let expected_scale = reference
                .node_voltages
                .iter()
                .filter_map(|(name, value)| Self::parse_voltage_probe(name).map(|_| value.abs()))
                .fold(0.0_f64, f64::max);
            let actual_scale = result
                .node_voltages
                .iter()
                .copied()
                .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
            let circuit_scale = expected_scale.max(actual_scale);
            // Use the operating-point voltage scale for direct probes so
            // sub-microvolt residue around a nominally-zero node does not
            // fail an otherwise correct deck.
            floor = floor.max(circuit_scale * 1e-4);
        }
        floor
    }

    pub(super) fn compare_values_with_abs_tol(
        &self,
        expected: f64,
        actual: f64,
        absolute_tolerance: f64,
    ) -> Option<f64> {
        let abs_diff = (expected - actual).abs();

        if abs_diff < absolute_tolerance {
            return None;
        }

        let rel_scale = expected.abs().max(actual.abs()).max(absolute_tolerance);
        let rel_error = abs_diff / rel_scale;

        if rel_error > self.config.relative_tolerance {
            Some(rel_error)
        } else {
            None
        }
    }

    pub(super) fn wrap_phase_delta(delta: f64, period: f64) -> f64 {
        let half_period = 0.5 * period;
        (delta + half_period).rem_euclid(period) - half_period
    }

    pub(super) fn compare_phase_values_with_abs_tol(
        &self,
        expected: f64,
        actual: f64,
        absolute_tolerance: f64,
        allow_orientation_flip: bool,
        degrees: bool,
    ) -> Option<f64> {
        let period = if degrees {
            360.0
        } else {
            2.0 * std::f64::consts::PI
        };
        let half_turn = 0.5 * period;
        let mut candidates = vec![actual, -actual];
        if allow_orientation_flip {
            candidates.extend_from_slice(&[
                actual + half_turn,
                actual - half_turn,
                -actual + half_turn,
                -actual - half_turn,
            ]);
        }
        let abs_diff = candidates
            .into_iter()
            .map(|candidate| Self::wrap_phase_delta(candidate - expected, period).abs())
            .fold(f64::INFINITY, f64::min);

        if abs_diff < absolute_tolerance {
            return None;
        }

        let rel_scale = expected.abs().max(actual.abs()).max(absolute_tolerance);
        let rel_error = abs_diff / rel_scale;
        if rel_error > self.config.relative_tolerance {
            Some(rel_error)
        } else {
            None
        }
    }

    pub(super) fn compare_values(&self, expected: f64, actual: f64) -> Option<f64> {
        self.compare_values_with_abs_tol(expected, actual, self.config.absolute_tolerance)
    }
}

// Unit Tests
// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
