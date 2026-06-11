use super::*;

impl Engine {
    pub(in crate::engine::advanced) fn collect_sensitivity_elements(
        circuit: &CircuitData,
    ) -> Vec<ElementDesc> {
        let mut elements = Vec::new();

        for (idx, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let name = circuit
                .resistors
                .names
                .get(idx)
                .cloned()
                .unwrap_or_else(|| format!("R{}", idx + 1));
            let g = circuit
                .resistors
                .small_signal_conductances
                .get(idx)
                .copied()
                .unwrap_or_else(|| {
                    circuit
                        .resistors
                        .conductances
                        .get(idx)
                        .copied()
                        .unwrap_or(0.0)
                });
            if !g.is_finite() || g.abs() <= 1e-30 {
                continue;
            }

            elements.push(ElementDesc::resistor(
                &name,
                Self::optional_system_index(stamp.pp.row),
                Self::optional_system_index(stamp.nn.row),
                1.0 / g,
            ));
        }

        for idx in 0..circuit.current_sources.names.len() {
            let name = circuit.current_sources.names[idx].clone();
            let value = circuit.current_sources.dc_values[idx];
            if !value.is_finite() {
                continue;
            }

            elements.push(ElementDesc::current_source(
                &name,
                Self::optional_system_index(circuit.current_sources.node_pos[idx]),
                Self::optional_system_index(circuit.current_sources.node_neg[idx]),
                value,
            ));
        }

        for idx in 0..circuit.voltage_sources.names.len() {
            let name = circuit.voltage_sources.names[idx].clone();
            let value = circuit.voltage_sources.dc_values[idx];
            let branch_ordinal = circuit.voltage_sources.branch_indices[idx];
            if !value.is_finite() || branch_ordinal == 0 {
                continue;
            }

            elements.push(ElementDesc::voltage_source(
                &name,
                Self::optional_system_index(circuit.voltage_sources.node_pos[idx]),
                Self::optional_system_index(circuit.voltage_sources.node_neg[idx]),
                circuit.get_branch_matrix_index(branch_ordinal) - 1,
                value,
            ));
        }

        elements
    }

    /// Run DC operating-point sensitivity using the linearized MNA system.
    pub fn run_sensitivity_linearized(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
    ) -> Result<SensitivityResult, SimulationError> {
        if output_pos == 0 {
            return Err(SimulationError::Circuit(
                "Sensitivity output node must not be ground".to_string(),
            ));
        }

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit(netlist)?;
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let dc_solution = engine.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit.prepare_behavioral_small_signal(&dc_solution);

        let dense_g = Self::build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, 0.0)
            .to_dense_real();
        let elements = Self::collect_sensitivity_elements(&circuit);
        if elements.is_empty() {
            return Err(SimulationError::Circuit(
                "Sensitivity analysis found no eligible linear elements or independent sources"
                    .to_string(),
            ));
        }

        let mut analyzer = SensitivityAnalyzer::new(dense_g, dc_solution, elements);
        analyzer
            .analyze(
                output_pos - 1,
                output_neg.and_then(Self::optional_system_index),
            )
            .ok_or(SimulationError::Solver(
                crate::solver::SolverError::SingularMatrix,
            ))
    }

    pub(in crate::engine::advanced) fn create_perturbed_netlist(
        netlist: &Netlist,
        param_name: &str,
        param_value: Value,
    ) -> Result<(Netlist, usize), SimulationError> {
        Self::create_perturbed_netlist_multi(
            netlist,
            &[(param_name.to_ascii_uppercase(), param_value)],
        )
    }

    pub(in crate::engine::advanced) fn create_perturbed_netlist_multi(
        netlist: &Netlist,
        overrides: &[(String, Value)],
    ) -> Result<(Netlist, usize), SimulationError> {
        let mut override_map: HashMap<String, Value> = HashMap::new();
        for (name, value) in overrides {
            override_map.insert(name.to_ascii_uppercase(), *value);
        }

        let mut ordered_overrides: Vec<(String, Value)> = override_map.into_iter().collect();
        ordered_overrides.sort_by(|a, b| a.0.cmp(&b.0));

        let mut perturbed = netlist.clone();
        for (name, value) in &ordered_overrides {
            perturbed.params.set(name, *value);
        }

        let Some(source) = &netlist.source_text else {
            return Ok((perturbed, 0));
        };

        let referenced = ordered_overrides
            .iter()
            .filter(|(name, _)| Self::source_references_param(source, name))
            .count();
        let overridden_source = Self::build_overridden_source_multi(source, &ordered_overrides);

        let mut reparsed = if let Some(source_path) = netlist.source_path.as_deref() {
            Netlist::parse_with_path(&overridden_source, source_path)
        } else {
            crate::netlist::parse_netlist(&overridden_source)
        }
        .map_err(|e| {
            SimulationError::Netlist(format!(
                "Failed to reparse netlist for parameter override set {:?}: {}",
                ordered_overrides, e
            ))
        })?;
        for (name, value) in &ordered_overrides {
            reparsed.params.set(name, *value);
        }

        Ok((reparsed, referenced))
    }

    pub(in crate::engine::advanced) fn logical_lines_after_title(source: &str) -> Vec<String> {
        let mut lines = Vec::new();
        let mut continuation = String::new();

        for raw in source.lines().skip(1) {
            let line = raw.split(';').next().unwrap_or("").trim();
            if line.is_empty() || line.starts_with('*') || line.starts_with('$') {
                continue;
            }

            if line.starts_with('+') {
                if !continuation.is_empty() {
                    continuation.push(' ');
                    continuation.push_str(line.trim_start_matches('+').trim());
                }
                continue;
            }

            if !continuation.is_empty() {
                lines.push(std::mem::take(&mut continuation));
                continuation.clear();
            }
            continuation.push_str(line);
        }

        if !continuation.is_empty() {
            lines.push(continuation);
        }

        lines
    }

    pub(in crate::engine::advanced) fn contains_identifier(
        haystack_upper: &str,
        needle_upper: &str,
    ) -> bool {
        if needle_upper.is_empty() {
            return false;
        }
        let haystack_bytes = haystack_upper.as_bytes();
        let needle_len = needle_upper.len();

        for (idx, _) in haystack_upper.match_indices(needle_upper) {
            let before_ok = idx == 0 || !Self::is_identifier_byte(haystack_bytes[idx - 1]);
            let after_idx = idx + needle_len;
            let after_ok = after_idx >= haystack_bytes.len()
                || !Self::is_identifier_byte(haystack_bytes[after_idx]);

            if before_ok && after_ok {
                return true;
            }
        }
        false
    }

    pub(in crate::engine::advanced) fn is_identifier_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }

    pub(in crate::engine::advanced) fn param_assignment_present(
        line: &str,
        param_upper: &str,
    ) -> bool {
        let trimmed = line.trim();
        let upper = trimmed.to_ascii_uppercase();
        if !upper.starts_with(".PARAM") {
            return false;
        }

        let mut idx = ".PARAM".len();
        let bytes = trimmed.as_bytes();
        while idx < bytes.len() {
            while idx < bytes.len() && (bytes[idx].is_ascii_whitespace() || bytes[idx] == b',') {
                idx += 1;
            }
            if idx >= bytes.len() {
                break;
            }

            let start = idx;
            while idx < bytes.len() && Self::is_identifier_byte(bytes[idx]) {
                idx += 1;
            }
            if idx == start {
                idx += 1;
                continue;
            }

            let name = &trimmed[start..idx];
            while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
                idx += 1;
            }

            if idx < bytes.len() && bytes[idx] == b'=' && name.eq_ignore_ascii_case(param_upper) {
                return true;
            }
        }

        false
    }

    pub(in crate::engine::advanced) fn source_references_param(
        source: &str,
        param_name: &str,
    ) -> bool {
        let param_upper = param_name.to_ascii_uppercase();

        Self::logical_lines_after_title(source).iter().any(|line| {
            let upper = line.to_ascii_uppercase();
            if upper.starts_with(".PARAM")
                || upper.starts_with(".IC")
                || upper.starts_with(".NODESET")
            {
                return false;
            }
            Self::contains_identifier(Self::binding_search_span(&upper), &param_upper)
        })
    }

    /// The slice of a logical line in which an identifier occurrence can bind
    /// a parameter.
    ///
    /// On an element line the leading token is the device's own name
    /// (`R1 1 2 {rval}`), so a match there is a device-name collision rather
    /// than a parameter reference — searching it made `run_sensitivity` and
    /// `.STEP` silently no-op when handed an element name. Dot commands have
    /// no such token and are searched whole.
    pub(in crate::engine::advanced) fn binding_search_span(line_upper: &str) -> &str {
        let trimmed = line_upper.trim_start();
        if trimmed.starts_with('.') {
            return trimmed;
        }
        trimmed
            .find(|c: char| c.is_ascii_whitespace() || c == ',')
            .map_or("", |idx| &trimmed[idx..])
    }

    pub(in crate::engine::advanced) fn build_overridden_source_multi(
        source: &str,
        overrides: &[(String, Value)],
    ) -> String {
        use std::fmt::Write;

        let title = source.lines().next().unwrap_or("Untitled");
        let mut out = String::new();

        let _ = writeln!(out, "{}", title);
        for (name, value) in overrides {
            let _ = writeln!(out, ".PARAM {}={:.17e}", name, value);
        }

        for line in Self::logical_lines_after_title(source) {
            let mut override_suffix = String::new();
            for (name, value) in overrides {
                if Self::param_assignment_present(&line, name) {
                    let _ = write!(override_suffix, " {}={:.17e}", name, value);
                }
            }

            if override_suffix.is_empty() {
                let _ = writeln!(out, "{}", line);
            } else {
                let _ = writeln!(out, "{}{}", line, override_suffix);
            }
        }

        out
    }

    /// Run sensitivity analysis
    ///
    /// Computes dVout/dparam using finite differences.
    /// Useful for design optimization and tolerance analysis.
    pub fn run_sensitivity(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        delta: Option<Value>,
    ) -> Result<Value, SimulationError> {
        let h = delta.unwrap_or(param_value.abs() * 0.01).max(1e-12);

        let (netlist_plus, rebuilt_plus) =
            Self::create_perturbed_netlist(netlist, param_name, param_value + h)?;
        let (netlist_minus, rebuilt_minus) =
            Self::create_perturbed_netlist(netlist, param_name, param_value - h)?;

        if netlist.source_text.is_some() && rebuilt_plus == 0 && rebuilt_minus == 0 {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        let result_plus = self.run_dc_op(&netlist_plus)?;
        let result_minus = self.run_dc_op(&netlist_minus)?;

        let v_plus = result_plus.voltage(output_node);
        let v_minus = result_minus.voltage(output_node);

        Ok((v_plus - v_minus) / (2.0 * h))
    }

    /// Run AC sensitivity analysis for a parameter across frequencies.
    ///
    /// Computes central differences of output voltage magnitude:
    /// d|Vout|/dp ~= (|Vout(p+h)| - |Vout(p-h)|) / (2h)
    pub fn run_sensitivity_ac(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        frequencies: &[Value],
        delta: Option<Value>,
    ) -> Result<Vec<Value>, SimulationError> {
        let h = delta.unwrap_or(param_value.abs() * 0.01).max(1e-12);

        let (netlist_plus, rebuilt_plus) =
            Self::create_perturbed_netlist(netlist, param_name, param_value + h)?;
        let (netlist_minus, rebuilt_minus) =
            Self::create_perturbed_netlist(netlist, param_name, param_value - h)?;

        if netlist.source_text.is_some() && rebuilt_plus == 0 && rebuilt_minus == 0 {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        let plus = self.run_ac(&netlist_plus, frequencies)?;
        let minus = self.run_ac(&netlist_minus, frequencies)?;
        if plus.len() != minus.len() {
            return Err(SimulationError::Circuit(
                "AC sensitivity produced inconsistent sweep lengths".to_string(),
            ));
        }

        Ok(plus
            .iter()
            .zip(minus.iter())
            .map(|(p, m)| {
                (p.voltage_magnitude(output_node) - m.voltage_magnitude(output_node)) / (2.0 * h)
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Engine;
    use crate::Netlist;

    const PARAMETRIC_DIVIDER: &str = "\
Parametric divider
.param rval=1k
V1 1 0 10
R1 1 2 {rval}
R2 2 0 1k
.end
";

    /// Same divider but with `rval` referenced bare in the value position
    /// instead of inside a brace expression.
    const BARE_REFERENCE_DIVIDER: &str = "\
Parametric divider, bare reference
.param rval=1k
V1 1 0 10
R1 1 2 rval
R2 2 0 1k
.end
";

    const ORPHAN_PARAM_DIVIDER: &str = "\
Divider with an unreferenced parameter
.param rval=1k
.param orphan=42
V1 1 0 10
R1 1 2 {rval}
R2 2 0 1k
.end
";

    /// Analytic dV(2)/drval for the divider at rval=1k: -10*1k/(1k+1k)^2.
    const EXPECTED_DIVIDER_SENSITIVITY: f64 = -2.5e-3;

    /// An element name is not a parameter binding: before the fix, the
    /// `R1 1 2 {rval}` line itself made "R1" look referenced, and the run
    /// silently returned a sensitivity of 0.0 instead of erroring.
    #[test]
    fn sensitivity_rejects_element_name_lookalike() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity(&netlist, 2, "R1", 1000.0, None)
            .expect_err("element name must not count as a parameter binding");
        let msg = err.to_string();
        assert!(
            msg.contains("'R1'") && msg.contains("is not bound to any netlist expression"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn sensitivity_perturbs_brace_bound_parameter() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let dv = Engine::default()
            .run_sensitivity(&netlist, 2, "rval", 1000.0, None)
            .expect("rval is bound through a brace expression");
        assert!(
            (dv - EXPECTED_DIVIDER_SENSITIVITY).abs() < 1e-6,
            "dV(2)/drval = {dv}, expected ~{EXPECTED_DIVIDER_SENSITIVITY}"
        );
    }

    /// Bare identifiers in value positions are parameter references too;
    /// excluding the leading element-name token must not break them.
    #[test]
    fn sensitivity_perturbs_bare_value_position_parameter() {
        let netlist = Netlist::parse(BARE_REFERENCE_DIVIDER).expect("deck parses");
        let dv = Engine::default()
            .run_sensitivity(&netlist, 2, "rval", 1000.0, None)
            .expect("rval is bound through a bare value reference");
        assert!(
            (dv - EXPECTED_DIVIDER_SENSITIVITY).abs() < 1e-6,
            "dV(2)/drval = {dv}, expected ~{EXPECTED_DIVIDER_SENSITIVITY}"
        );
    }

    #[test]
    fn sensitivity_rejects_param_defined_but_never_referenced() {
        let netlist = Netlist::parse(ORPHAN_PARAM_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity(&netlist, 2, "orphan", 42.0, None)
            .expect_err("a .param defined but never referenced must raise");
        assert!(
            err.to_string()
                .contains("is not bound to any netlist expression"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn ac_sensitivity_rejects_element_name_lookalike() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity_ac(&netlist, 2, "R1", 1000.0, &[1e3], None)
            .expect_err("element name must not count as a parameter binding");
        assert!(
            err.to_string()
                .contains("is not bound to any netlist expression"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn step_rejects_element_name_lookalike() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_step(&netlist, "R1", &[500.0, 2000.0])
            .expect_err("element name must not count as a parameter binding");
        assert!(
            err.to_string()
                .contains("is not bound to any netlist expression"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn step_sweeps_a_genuinely_bound_parameter() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let results = Engine::default()
            .run_step(&netlist, "rval", &[1000.0, 4000.0])
            .expect("rval is bound");
        assert_eq!(results.len(), 2);
        // V(2) = 10 * 1k / (rval + 1k): 5 V at 1k, 2 V at 4k.
        assert!((results[0].1.voltage(2) - 5.0).abs() < 1e-9);
        assert!((results[1].1.voltage(2) - 2.0).abs() < 1e-9);
    }
}
