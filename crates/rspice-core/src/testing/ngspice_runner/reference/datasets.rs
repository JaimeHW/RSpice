use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn compare_dc_sweep_reference(
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

    pub(in crate::testing::ngspice_runner) fn compare_transient_reference(
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

    pub(in crate::testing::ngspice_runner) fn compare_ac_reference(
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

    pub(in crate::testing::ngspice_runner) fn compare_noise_reference(
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
        // ngspice-46 emits noise vectors in root-spectral-density units by
        // default: every output whose name starts with inoise/onoise gets
        // sqrt() applied at emission (cktnoise.c N_CALC, squared_value[]),
        // unless the legacy `sqrnoise` control variable is set — which batch
        // decks cannot do. References regenerated from the official binary
        // therefore carry V/sqrt(Hz), not the pre-2008 V^2/Hz convention.
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
                        .map(|point| point.output_noise_rms())
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
                        .map(|point| point.input_referred_rms())
                        .collect(),
                );
            }

            None
        }))
    }

    pub(in crate::testing::ngspice_runner) fn transient_node_waveform(
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

    pub(in crate::testing::ngspice_runner) fn resolve_transient_device_series(
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
        let (pos_node, neg_node) = Self::resolve_device_voltage_nodes(netlist, element, quantity)?;
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

    pub(in crate::testing::ngspice_runner) fn resolve_device_voltage_nodes(
        netlist: &Netlist,
        element: &crate::netlist::Element,
        quantity: &str,
    ) -> Option<(String, String)> {
        let quantity = Self::normalize_variable_name(quantity);
        match &element.kind {
            crate::netlist::ElementKind::Mosfet { model, .. } => {
                // BSIMSOI floating-body level-56 devices own an internal body
                // node; their `vbs`/`vbd` are measured against that node, not the
                // external back-gate (`nodes[3]`). A 5-terminal instance ties the
                // body to the external contact `nodes[4]` and keeps node-based
                // resolution. Non-SOI MOSFETs keep the classic terminal mapping.
                let soi_body = Self::bsimsoi_body_node_name(netlist, element, model);
                match quantity.as_str() {
                    "vds" => Some((
                        element.nodes.first()?.clone(),
                        element.nodes.get(2)?.clone(),
                    )),
                    "vgs" => {
                        Some((element.nodes.get(1)?.clone(), element.nodes.get(2)?.clone()))
                    }
                    "vbs" => {
                        let body = soi_body.unwrap_or(element.nodes.get(3)?.clone());
                        Some((body, element.nodes.get(2)?.clone()))
                    }
                    "vgd" => Some((
                        element.nodes.get(1)?.clone(),
                        element.nodes.first()?.clone(),
                    )),
                    "vbd" => {
                        let body = soi_body.unwrap_or(element.nodes.get(3)?.clone());
                        Some((body, element.nodes.first()?.clone()))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Body-node name for a BSIMSOI level-56 instance, or `None` for non-SOI
    /// MOSFETs and tied-body (5-terminal) SOI instances that resolve to an
    /// external node. Matches the builder's internal-node naming.
    fn bsimsoi_body_node_name(
        netlist: &Netlist,
        element: &crate::netlist::Element,
        model: &str,
    ) -> Option<String> {
        let level = netlist
            .models
            .iter()
            .find(|m| m.name.eq_ignore_ascii_case(model))?
            .params
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("LEVEL"))
            .map(|(_, v)| *v as i32)?;
        if level != 56 {
            return None;
        }
        if element.nodes.len() > 4 {
            // Tied body: the external contact `nodes[4]` is the body node.
            element.nodes.get(4).cloned()
        } else {
            // Floating body: the builder allocates `<name>.__body.internal`.
            Some(format!("{}.__body.internal", element.name))
        }
    }

    pub(in crate::testing::ngspice_runner) fn resolve_reference_series<F>(
        expr: &str,
        direct: &F,
    ) -> Option<Vec<f64>>
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

    pub(in crate::testing::ngspice_runner) fn split_reference_binary_expression(
        expr: &str,
    ) -> Option<(&str, char, &str)> {
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

    pub(in crate::testing::ngspice_runner) fn parse_reference_scalar(expr: &str) -> Option<f64> {
        crate::netlist::lexer::parse_spice_value(expr)
            .ok()
            .or_else(|| expr.parse::<f64>().ok())
    }

    pub(in crate::testing::ngspice_runner) fn is_voltage_probe_name(expr: &str) -> bool {
        Self::parse_voltage_probe(expr).is_some()
    }

    pub(in crate::testing::ngspice_runner) fn is_current_probe_name(expr: &str) -> bool {
        Self::parse_current_probe(expr).is_some()
    }

    pub(in crate::testing::ngspice_runner) fn reference_expr_contains_probe(
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

    pub(in crate::testing::ngspice_runner) fn reference_expr_contains_voltage_probe(
        expr: &str,
    ) -> bool {
        let normalized = Self::normalize_variable_name(expr);
        Self::reference_expr_contains_probe(&normalized, Self::is_voltage_probe_name)
    }

    pub(in crate::testing::ngspice_runner) fn reference_expr_contains_current_probe(
        expr: &str,
    ) -> bool {
        let normalized = Self::normalize_variable_name(expr);
        Self::reference_expr_contains_probe(&normalized, Self::is_current_probe_name)
    }

    pub(in crate::testing::ngspice_runner) fn compare_reference_dataset<F>(
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
            // ngspice batch tables print every phase output in radians (the
            // degree switch is an interactive-only `units` variable), so all
            // phase probes wrap in the radian domain.
            let degrees_phase_probe = false;

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
            let time_axis = Self::normalize_variable_name(&reference.x_name) == "time";
            if sim_monotonic && ref_monotonic {
                for (idx, (&x_ref, &expected)) in expected_series
                    .x
                    .iter()
                    .zip(expected_series.y.iter())
                    .enumerate()
                {
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
                        // Steep transient rows sample one specific run's
                        // internally chosen timesteps; allow a slope-gated
                        // one-local-step timing window before flagging.
                        if time_axis
                            && !phase_probe
                            && self.matches_within_time_jitter_window(
                                x_sim,
                                &actual_series,
                                x_ref,
                                expected,
                                Self::local_axis_spacing(&expected_series.x, idx),
                                Self::local_reference_slope(
                                    &expected_series.x,
                                    &expected_series.y,
                                    idx,
                                ),
                                absolute_tolerance,
                            )
                        {
                            continue;
                        }
                        // Sample-scale reference oscillations (ringing or
                        // settling wiggles a coarse reference grid overshoots)
                        // are compared against the local reference envelope.
                        if time_axis
                            && !phase_probe
                            && self.matches_within_reference_envelope(
                                expected_series,
                                idx,
                                actual,
                                absolute_tolerance,
                            )
                        {
                            continue;
                        }
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
    pub(in crate::testing::ngspice_runner) fn is_monotonic_axis(x: &[f64]) -> bool {
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
}
