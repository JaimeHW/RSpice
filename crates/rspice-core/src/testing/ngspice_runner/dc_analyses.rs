//! DC operating-point and sweep regression runners.

use super::*;

impl TestRunner {
    pub(super) fn run_dc_op_test(
        &self,
        name: &str,
        cir_path: &Path,
        source_path: &Path,
        source: &str,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Self::parse_regression_source(source, source_path) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC OP".to_string()),
                };
            }
        };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let primary_result = primary_engine.run_dc_op(&netlist);
        let op_result = match primary_result {
            Ok(result) if !Self::simulation_result_contains_non_finite(&result) => Ok(result),
            Ok(_) => robust_engine.run_dc_op(&netlist),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => {
                robust_engine.run_dc_op(&netlist)
            }
            Err(err) => Err(err),
        };

        match op_result {
            Ok(result) => {
                let mut mismatches = self.compare_test_gold_nodes(&result);
                if mismatches.is_empty() {
                    mismatches = match self.compare_dc_op_reference(cir_path, &result) {
                        Ok(m) => m,
                        Err(e) => {
                            return TestResult {
                                name: name.to_string(),
                                passed: false,
                                error: Some(format!("Reference comparison error: {}", e)),
                                mismatches: Vec::new(),
                                duration_ms: start.elapsed().as_millis(),
                                analysis_type: Some("DC OP".to_string()),
                            };
                        }
                    };
                }
                let passed = mismatches.is_empty();

                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} assertion(s) failed", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC OP".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("DC OP".to_string()),
            },
        }
    }

    /// Compare test nodes (`*_t`) against gold nodes (`*_g`).
    ///
    /// Many ngspice regression circuits encode assertions in-circuit by driving
    /// parallel `_t` (test) and `_g` (gold) nodes. This validates those pairs
    /// directly from the operating-point solution.
    pub(super) fn compare_test_gold_nodes(
        &self,
        result: &crate::SimulationResult,
    ) -> Vec<ValueMismatch> {
        let mut mismatches = Vec::new();

        // SPICE node names are case-insensitive.
        let mut node_to_idx: HashMap<String, usize> =
            HashMap::with_capacity(result.node_names.len());
        for (idx, name) in result.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        for (test_idx, test_name) in result.node_names.iter().enumerate() {
            let test_lower = test_name.to_ascii_lowercase();
            let Some(base) = test_lower.strip_suffix("_t") else {
                continue;
            };

            let gold_name = format!("{base}_g");
            let Some(&gold_idx) = node_to_idx.get(&gold_name) else {
                mismatches.push(ValueMismatch {
                    x_value: 0.0,
                    node: test_name.clone(),
                    expected: f64::NAN,
                    actual: result.node_voltages.get(test_idx).copied().unwrap_or(0.0),
                    relative_error: f64::INFINITY,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    break;
                }
                continue;
            };

            let actual = result.node_voltages.get(test_idx).copied().unwrap_or(0.0);
            let expected = result.node_voltages.get(gold_idx).copied().unwrap_or(0.0);

            if let Some(relative_error) = self.compare_values(expected, actual) {
                mismatches.push(ValueMismatch {
                    x_value: 0.0,
                    node: test_name.clone(),
                    expected,
                    actual,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    break;
                }
            }
        }

        mismatches
    }

    pub(super) fn run_dc_sweep_test(
        &self,
        name: &str,
        cir_path: &Path,
        source_path: &Path,
        source: &str,
        sweep_source: &str,
        start_val: Value,
        stop_val: Value,
        step_val: Value,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Self::parse_regression_source(source, source_path) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                };
            }
        };

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms);
        let primary_engine = self.create_dynamic_engine();
        let primary_result = primary_engine.run_dc_sweep_with_abort(
            &netlist,
            sweep_source,
            start_val,
            stop_val,
            step_val,
            &abort,
        );

        let sweep_result = match primary_result {
            Ok(results) if !Self::dc_sweep_results_contain_non_finite(&results) => Ok(results),
            Ok(_) => {
                let robust_engine = self.create_dc_engine();
                robust_engine.run_dc_sweep_with_abort(
                    &netlist,
                    sweep_source,
                    start_val,
                    stop_val,
                    step_val,
                    &abort,
                )
            }
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => {
                let robust_engine = self.create_dc_engine();
                robust_engine.run_dc_sweep_with_abort(
                    &netlist,
                    sweep_source,
                    start_val,
                    stop_val,
                    step_val,
                    &abort,
                )
            }
            Err(err) => Err(err),
        };

        match sweep_result {
            Ok(results) => {
                let mismatches = match self.compare_dc_sweep_reference(cir_path, &netlist, &results)
                {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("DC Sweep".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("DC Sweep".to_string()),
            },
        }
    }

    #[inline]
    pub(super) fn is_recoverable_dc_convergence_error(
        err: &crate::engine::SimulationError,
    ) -> bool {
        let message = err.to_string().to_ascii_lowercase();
        !message.contains("aborted")
            && (message.contains("convergence")
                || message.contains("singular")
                || message.contains("failed to converge"))
    }

    #[inline]
    pub(super) fn generate_sweep_points(
        start: Value,
        stop: Value,
        step: Value,
    ) -> Result<Vec<Value>, String> {
        if !start.is_finite() || !stop.is_finite() || !step.is_finite() || step == 0.0 {
            return Err("Invalid DC sweep range".to_string());
        }
        if (stop > start && step < 0.0) || (stop < start && step > 0.0) {
            return Err("DC sweep step sign does not reach stop".to_string());
        }

        let mut points = Vec::new();
        let mut point_index = 0usize;
        let max_points = 2_000_000usize;
        let eps = (step.abs() * 1e-9).max(1e-18);
        let done = |value: Value| -> bool {
            if step > 0.0 {
                value > stop + eps
            } else {
                value < stop - eps
            }
        };

        loop {
            let value = start + step * point_index as Value;
            if done(value) {
                break;
            }

            let snapped_to_stop = (value - stop).abs() <= eps;
            points.push(if snapped_to_stop { stop } else { value });
            point_index += 1;
            if point_index >= max_points {
                return Err("DC sweep exceeded point limit".to_string());
            }
            if snapped_to_stop {
                break;
            }
        }
        if points.is_empty() {
            points.push(start);
        }
        Ok(points)
    }

    pub(super) fn set_source_dc_value(
        &self,
        netlist: &mut Netlist,
        source_name: &str,
        dc_value: Value,
    ) -> Result<(), String> {
        for element in &mut netlist.elements {
            if !element.name.eq_ignore_ascii_case(source_name) {
                continue;
            }
            match &mut element.kind {
                crate::netlist::ElementKind::VoltageSource(spec)
                | crate::netlist::ElementKind::CurrentSource(spec) => {
                    match spec {
                        crate::netlist::SourceSpec::Dc(v) => *v = dc_value,
                        crate::netlist::SourceSpec::DcAc { dc_value: v, .. } => *v = dc_value,
                        crate::netlist::SourceSpec::DcTransient { dc_value: v, .. } => {
                            *v = dc_value
                        }
                        crate::netlist::SourceSpec::DcAcTransient { dc_value: v, .. } => {
                            *v = dc_value
                        }
                        crate::netlist::SourceSpec::Ac { .. }
                        | crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::Pat { .. }
                        | crate::netlist::SourceSpec::Exp { .. }
                        | crate::netlist::SourceSpec::Sffm { .. }
                        | crate::netlist::SourceSpec::Am { .. }
                        | crate::netlist::SourceSpec::TrNoise { .. } => {
                            *spec = crate::netlist::SourceSpec::Dc(dc_value);
                        }
                    }
                    return Ok(());
                }
                _ => {
                    return Err(format!(
                        "Sweep source '{}' is not an independent source",
                        source_name
                    ));
                }
            }
        }
        Err(format!("Sweep source '{}' not found", source_name))
    }

    pub(super) fn assign_ac_to_source_spec(
        spec: &mut crate::netlist::SourceSpec,
        magnitude: Value,
        phase: Value,
    ) {
        *spec = spec.clone().with_ac(magnitude, phase);
    }

    pub(super) fn set_source_ac_value(
        &self,
        netlist: &mut Netlist,
        source_name: &str,
        magnitude: Value,
        phase: Value,
    ) -> Result<(), String> {
        for element in &mut netlist.elements {
            if !element.name.eq_ignore_ascii_case(source_name) {
                continue;
            }
            match &mut element.kind {
                crate::netlist::ElementKind::VoltageSource(spec)
                | crate::netlist::ElementKind::CurrentSource(spec) => {
                    Self::assign_ac_to_source_spec(spec, magnitude, phase);
                    return Ok(());
                }
                _ => {
                    return Err(format!(
                        "Transfer source '{}' is not an independent source",
                        source_name
                    ));
                }
            }
        }
        Err(format!("Transfer source '{}' not found", source_name))
    }

    pub(super) fn clear_all_source_ac_values(&self, netlist: &mut Netlist) {
        for element in &mut netlist.elements {
            match &mut element.kind {
                crate::netlist::ElementKind::VoltageSource(spec)
                | crate::netlist::ElementKind::CurrentSource(spec) => {
                    Self::assign_ac_to_source_spec(spec, 0.0, 0.0);
                }
                _ => {}
            }
        }
    }

    pub(super) fn run_dc_sweep_2d_test(
        &self,
        name: &str,
        cir_path: &Path,
        source_path: &Path,
        source: &str,
        inner_source: &str,
        inner_start: Value,
        inner_stop: Value,
        inner_step: Value,
        outer_source: &str,
        outer_start: Value,
        outer_stop: Value,
        outer_step: Value,
        start: std::time::Instant,
    ) -> TestResult {
        let base_netlist = match Self::parse_regression_source(source, source_path) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                };
            }
        };

        let outer_points = match Self::generate_sweep_points(outer_start, outer_stop, outer_step) {
            Ok(points) => points,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Sweep setup error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                };
            }
        };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms);
        let mut merged_results: Vec<(Value, crate::SimulationResult)> = Vec::new();

        for outer_value in outer_points {
            let mut netlist = base_netlist.clone();
            if let Err(e) = self.set_source_dc_value(&mut netlist, outer_source, outer_value) {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Sweep setup error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                };
            }

            let primary_result = primary_engine.run_dc_sweep_with_abort(
                &netlist,
                inner_source,
                inner_start,
                inner_stop,
                inner_step,
                &abort,
            );
            let sweep_result = match primary_result {
                Ok(results) if !Self::dc_sweep_results_contain_non_finite(&results) => Ok(results),
                Ok(_) => robust_engine.run_dc_sweep_with_abort(
                    &netlist,
                    inner_source,
                    inner_start,
                    inner_stop,
                    inner_step,
                    &abort,
                ),
                Err(err) if Self::is_recoverable_dc_convergence_error(&err) => robust_engine
                    .run_dc_sweep_with_abort(
                        &netlist,
                        inner_source,
                        inner_start,
                        inner_stop,
                        inner_step,
                        &abort,
                    ),
                Err(err) => Err(err),
            };

            match sweep_result {
                Ok(mut results) => merged_results.append(&mut results),
                Err(e) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(format!("Simulation error: {}", e)),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("DC Sweep".to_string()),
                    };
                }
            }
        }

        let mismatches =
            match self.compare_dc_sweep_reference(cir_path, &base_netlist, &merged_results) {
                Ok(m) => m,
                Err(e) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(format!("Reference comparison error: {}", e)),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("DC Sweep".to_string()),
                    };
                }
            };
        let passed = mismatches.is_empty();
        TestResult {
            name: name.to_string(),
            passed,
            error: if passed {
                None
            } else {
                Some(format!("{} reference mismatch(es)", mismatches.len()))
            },
            mismatches,
            duration_ms: start.elapsed().as_millis(),
            analysis_type: Some("DC Sweep".to_string()),
        }
    }
}
