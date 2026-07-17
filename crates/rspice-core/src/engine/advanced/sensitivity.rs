use super::*;

#[derive(Debug, Clone, Copy)]
enum AcSensitivityElementField {
    ResistorValue,
    CapacitorValue,
    InductorValue,
    JilesAthertonValue,
    VcvsGain,
    CccsGain,
    VccsTransconductance,
    CcvsTransresistance,
    BehavioralTc1,
    BehavioralTc2,
    TransmissionZ0,
    TransmissionDelay,
    TransmissionFrequency,
    TransmissionLength,
    Coupling,
}

#[derive(Debug, Clone)]
enum AcSensitivityLocation {
    ElementField {
        element_index: usize,
        field: AcSensitivityElementField,
    },
    ElementParameter {
        element_index: usize,
        parameter_index: usize,
    },
    ElementNamedParameter {
        element_index: usize,
        parameter: String,
    },
    ElementVectorParameter {
        element_index: usize,
        parameter_index: usize,
        entry_index: usize,
    },
    ElementNamedVectorParameter {
        element_index: usize,
        parameter: String,
        entry_index: usize,
        resolved_values: Vec<Value>,
    },
    SourceDc {
        element_index: usize,
    },
    SourceAcMagnitude {
        element_index: usize,
    },
    SourceAcPhaseDegrees {
        element_index: usize,
    },
    ModelParameter {
        model_index: usize,
        parameter: String,
    },
    ModelVectorParameter {
        model_index: usize,
        parameter_index: usize,
        entry_index: usize,
    },
    ModelNamedVectorParameter {
        model_index: usize,
        parameter: String,
        entry_index: usize,
        resolved_values: Vec<Value>,
    },
}

#[derive(Debug, Clone)]
struct AcSensitivityTarget {
    vector_name: String,
    element: String,
    element_type: ElementType,
    parameter: String,
    nominal_value: Value,
    location: AcSensitivityLocation,
}

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
        self.run_sensitivity_linearized_with_abort(netlist, output_pos, output_neg, &NoAbort)
    }

    /// Run adjoint DC sensitivity with cooperative cancellation of its
    /// operating-point solve.
    pub fn run_sensitivity_linearized_with_abort(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        abort: &dyn AbortSignal,
    ) -> Result<SensitivityResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if output_pos == 0 {
            return Err(SimulationError::Circuit(
                "Sensitivity output node must not be ground".to_string(),
            ));
        }

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::warn_xspice_mif_analysis_boundary(
            &circuit,
            "Sensitivity",
            "linearized sensitivity reports native element/source derivatives and does not use ngspice MIF DEVsen* hooks",
        );
        Self::validate_sensitivity_node("output", output_pos, circuit.num_nodes())?;
        if let Some(output_neg) = output_neg {
            Self::validate_sensitivity_node("reference", output_neg, circuit.num_nodes())?;
        }

        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let dc_solution = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            abort,
        )?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit.prepare_behavioral_small_signal(&dc_solution);

        let dense_g = Self::try_build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, 0.0)?
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

    fn validate_sensitivity_node(
        role: &str,
        node: usize,
        num_nodes: usize,
    ) -> Result<(), SimulationError> {
        if node > num_nodes {
            return Err(SimulationError::Circuit(format!(
                "Sensitivity {role} node {node} is outside circuit node range 0..={num_nodes}"
            )));
        }
        Ok(())
    }

    pub(crate) fn create_perturbed_netlist(
        netlist: &Netlist,
        param_name: &str,
        param_value: Value,
    ) -> Result<(Netlist, usize), SimulationError> {
        Self::create_perturbed_netlist_multi(
            netlist,
            &[(param_name.to_ascii_uppercase(), param_value)],
        )
    }

    pub(crate) fn create_perturbed_netlist_multi(
        netlist: &Netlist,
        overrides: &[(String, Value)],
    ) -> Result<(Netlist, usize), SimulationError> {
        Self::create_perturbed_netlist_multi_with_abort(netlist, overrides, &NoAbort)
    }

    pub(crate) fn create_perturbed_netlist_multi_with_abort(
        netlist: &Netlist,
        overrides: &[(String, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, usize), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let mut override_map: HashMap<String, Value> = HashMap::new();
        for (index, (name, value)) in overrides.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            override_map.insert(name.to_ascii_uppercase(), *value);
        }

        let mut ordered_overrides: Vec<(String, Value)> = override_map.into_iter().collect();
        ordered_overrides.sort_by(|a, b| a.0.cmp(&b.0));

        let mut param_overrides = Vec::new();
        let mut device_overrides = Vec::new();
        for (index, (name, value)) in ordered_overrides.into_iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if let Some((device_name, param_name)) = Self::split_device_parameter_override(&name) {
                device_overrides.push((device_name, param_name, value));
            } else {
                param_overrides.push((name, value));
            }
        }

        let mut perturbed = netlist.clone();
        for (name, value) in &param_overrides {
            perturbed.params.set(name, *value);
        }
        let applied_device_overrides = Self::apply_device_parameter_overrides_with_abort(
            &mut perturbed,
            &device_overrides,
            abort,
        )?;

        let Some(source) = &netlist.source_text else {
            return Ok((perturbed, applied_device_overrides));
        };

        let referenced = param_overrides
            .iter()
            .filter(|(name, _)| Self::source_references_param(source, name))
            .count();
        let overridden_source =
            Self::build_overridden_source_multi_with_abort(source, &param_overrides, abort)?;

        let parse_options = crate::netlist::NetlistParseOptions {
            statistical_mode: netlist.params.statistical_mode(),
            expression_dialect: netlist.params.expression_dialect(),
            parameter_redefinition_policy: netlist.params.parameter_redefinition_policy(),
        };
        let mut reparsed = if let Some(source_path) = netlist.source_path.as_deref() {
            Netlist::parse_with_path_and_options_and_abort(
                &overridden_source,
                source_path,
                parse_options,
                abort,
            )
        } else {
            Netlist::parse_with_options_and_abort(&overridden_source, parse_options, abort)
        }
        .map_err(|error| match error {
            crate::netlist::ParseWithAbortError::Aborted => SimulationError::Aborted,
            crate::netlist::ParseWithAbortError::Parse(error) => SimulationError::Netlist(format!(
                "Failed to reparse netlist for parameter override set {:?}: {}",
                param_overrides, error
            )),
        })?;
        for (name, value) in &param_overrides {
            reparsed.params.set(name, *value);
        }
        let applied_device_overrides = Self::apply_device_parameter_overrides_with_abort(
            &mut reparsed,
            &device_overrides,
            abort,
        )?;

        Ok((reparsed, referenced + applied_device_overrides))
    }

    fn split_device_parameter_override(name: &str) -> Option<(String, String)> {
        let (device_name, param_name) = name.split_once(':')?;
        let device_name = device_name.trim();
        let param_name = param_name.trim();
        (!device_name.is_empty() && !param_name.is_empty())
            .then(|| (device_name.to_string(), param_name.to_string()))
    }

    fn apply_device_parameter_overrides_with_abort(
        netlist: &mut Netlist,
        overrides: &[(String, String, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<usize, SimulationError> {
        let mut applied = 0;
        for (index, (device_name, param_name, value)) in overrides.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let element = netlist
                .elements
                .iter_mut()
                .find(|element| element.name.eq_ignore_ascii_case(device_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        ".STEP DATA target '{}:{}' not found in netlist",
                        device_name, param_name
                    ))
                })?;
            Self::apply_device_step_value(&mut element.kind, Some(param_name), *value)?;
            applied += 1;
        }
        if applied > 0 {
            netlist.source_text = None;
            netlist.source_path = None;
        }
        Ok(applied)
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
        if !Self::is_parameter_assignment_command(&upper) {
            return false;
        }

        let mut idx = trimmed
            .split_whitespace()
            .next()
            .map(str::len)
            .unwrap_or_default();
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

    fn is_parameter_assignment_command(upper_trimmed_line: &str) -> bool {
        upper_trimmed_line.starts_with(".PARAM")
            || upper_trimmed_line.starts_with(".CSPARAM")
            || upper_trimmed_line.starts_with(".GLOBAL_PARAM")
    }

    pub(in crate::engine::advanced) fn source_references_param(
        source: &str,
        param_name: &str,
    ) -> bool {
        let param_upper = param_name.to_ascii_uppercase();

        Self::logical_lines_after_title(source).iter().any(|line| {
            let upper = line.to_ascii_uppercase();
            if Self::is_parameter_assignment_command(upper.trim_start())
                || upper.starts_with(".DATA")
                || upper.starts_with(".ENDDATA")
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

    pub(in crate::engine::advanced) fn build_overridden_source_multi_with_abort(
        source: &str,
        overrides: &[(String, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<String, SimulationError> {
        use std::fmt::Write;

        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let title = source.lines().next().unwrap_or("Untitled");
        let mut out = String::new();

        let _ = writeln!(out, "{}", title);
        for (index, (name, value)) in overrides.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let _ = writeln!(out, ".PARAM {}={:.17e}", name, value);
        }

        let lines =
            Self::logical_lines_after_title_preserving_data_blocks_with_abort(source, abort)?;
        for (line_index, line) in lines.into_iter().enumerate() {
            if line_index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut override_suffix = String::new();
            for (override_index, (name, value)) in overrides.iter().enumerate() {
                if override_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
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

        Ok(out)
    }

    fn logical_lines_after_title_preserving_data_blocks_with_abort(
        source: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<String>, SimulationError> {
        let mut lines = Vec::new();
        let mut continuation = String::new();
        let mut in_data_block = false;

        for (line_index, raw) in source.lines().skip(1).enumerate() {
            if line_index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let line = raw.split(';').next().unwrap_or("").trim();
            if line.is_empty() || line.starts_with('*') || line.starts_with('$') {
                continue;
            }

            let head = line.split_whitespace().next().unwrap_or("");
            if in_data_block {
                if !continuation.is_empty() {
                    lines.push(std::mem::take(&mut continuation));
                }
                lines.push(line.to_string());
                if head.eq_ignore_ascii_case(".enddata") {
                    in_data_block = false;
                }
                continue;
            }

            if head.eq_ignore_ascii_case(".data") {
                if !continuation.is_empty() {
                    lines.push(std::mem::take(&mut continuation));
                }
                lines.push(line.to_string());
                in_data_block = true;
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
            }
            continuation.push_str(line);
        }

        if !continuation.is_empty() {
            lines.push(continuation);
        }

        Ok(lines)
    }

    fn sensitivity_step(
        param_value: Value,
        delta: Option<Value>,
    ) -> Result<Value, SimulationError> {
        if !param_value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "Sensitivity param_value must be finite, got {param_value}"
            )));
        }
        if let Some(delta) = delta {
            if !delta.is_finite() || delta <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Sensitivity delta must be a positive finite number, got {delta}"
                )));
            }
            return Ok(delta);
        }
        Ok((param_value.abs() * 0.01).max(1e-12))
    }

    fn sensitivity_ac_voltage_magnitude(
        result: &crate::analysis::AcResult,
        output_node: usize,
    ) -> Result<Value, SimulationError> {
        if output_node == 0 {
            return Ok(0.0);
        }

        result
            .voltages
            .get(output_node - 1)
            .map(|voltage| voltage.norm())
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "Sensitivity output node {output_node} is outside circuit node range 0..={}",
                    result.voltages.len()
                ))
            })
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
        self.run_sensitivity_with_abort(
            netlist,
            output_node,
            param_name,
            param_value,
            delta,
            &NoAbort,
        )
    }

    /// Run finite-difference DC sensitivity with cooperative cancellation.
    #[allow(clippy::too_many_arguments)]
    pub fn run_sensitivity_with_abort(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        delta: Option<Value>,
        abort: &dyn AbortSignal,
    ) -> Result<Value, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let h = Self::sensitivity_step(param_value, delta)?;

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

        let result_plus = self.run_dc_op_with_abort(&netlist_plus, abort)?;
        let result_minus = self.run_dc_op_with_abort(&netlist_minus, abort)?;

        let v_plus = result_plus.try_voltage(output_node).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "Sensitivity output node {output_node} is outside circuit node range 0..={}",
                result_plus.node_voltages.len().saturating_sub(1)
            ))
        })?;
        let v_minus = result_minus.try_voltage(output_node).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "Sensitivity output node {output_node} is outside circuit node range 0..={}",
                result_minus.node_voltages.len().saturating_sub(1)
            ))
        })?;

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
        self.run_sensitivity_ac_with_abort(
            netlist,
            output_node,
            param_name,
            param_value,
            frequencies,
            delta,
            &NoAbort,
        )
    }

    /// Run finite-difference AC sensitivity with cooperative cancellation.
    #[allow(clippy::too_many_arguments)]
    pub fn run_sensitivity_ac_with_abort(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        frequencies: &[Value],
        delta: Option<Value>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let h = Self::sensitivity_step(param_value, delta)?;

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

        let plus = self.run_ac_with_abort(&netlist_plus, frequencies, abort)?;
        let minus = self.run_ac_with_abort(&netlist_minus, frequencies, abort)?;
        if plus.len() != minus.len() {
            return Err(SimulationError::Circuit(
                "AC sensitivity produced inconsistent sweep lengths".to_string(),
            ));
        }

        plus.iter()
            .zip(minus.iter())
            .map(|(p, m)| {
                let p_mag = Self::sensitivity_ac_voltage_magnitude(p, output_node)?;
                let m_mag = Self::sensitivity_ac_voltage_magnitude(m, output_node)?;
                Ok((p_mag - m_mag) / (2.0 * h))
            })
            .collect()
    }

    fn flattened_sensitivity_netlist(netlist: &Netlist) -> Result<Netlist, SimulationError> {
        let flattened = crate::netlist::flatten_netlist_with_models(netlist)
            .map_err(|error| SimulationError::Netlist(error.to_string()))?;
        let mut flat = netlist.clone();
        flat.elements = flattened.elements;
        flat.subcircuits.clear();
        flat.models.extend(flattened.scoped_models);
        flat.initial_conditions
            .extend(flattened.scoped_initial_conditions);
        flat.node_sets.extend(flattened.scoped_node_sets);
        // Parameter perturbations below edit the resolved AST directly. A
        // retained source would cause generic override helpers to reparse the
        // original hierarchy and discard those edits.
        flat.source_text = None;
        flat.source_path = None;
        Ok(flat)
    }

    fn source_has_explicit_ac(spec: &SourceSpec) -> bool {
        match spec {
            SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
                Self::source_has_explicit_ac(inner)
            }
            SourceSpec::Ac { .. } | SourceSpec::DcAc { .. } | SourceSpec::DcAcTransient { .. } => {
                true
            }
            SourceSpec::DcTransient { transient, .. } => Self::source_has_explicit_ac(transient),
            _ => false,
        }
    }

    fn sensitivity_element_type(kind: &ElementKind) -> ElementType {
        match kind {
            ElementKind::Resistor { .. } => ElementType::Resistor,
            ElementKind::Capacitor { .. } => ElementType::Capacitor,
            ElementKind::Inductor { .. } | ElementKind::JilesAthertonInductor { .. } => {
                ElementType::Inductor
            }
            ElementKind::VoltageSource(_) | ElementKind::VoltageSourceDeferred(_) => {
                ElementType::VoltageSource
            }
            ElementKind::CurrentSource(_) | ElementKind::CurrentSourceDeferred(_) => {
                ElementType::CurrentSource
            }
            ElementKind::Diode { .. } => ElementType::Diode,
            ElementKind::Bjt { .. } => ElementType::Bjt,
            ElementKind::Mosfet { .. } => ElementType::Mosfet,
            ElementKind::Jfet { .. } => ElementType::Jfet,
            ElementKind::Mesfet { .. } => ElementType::Mesfet,
            ElementKind::Vccs { .. } => ElementType::Transconductance,
            ElementKind::Ccvs { .. } => ElementType::Transresistance,
            ElementKind::Vcvs { .. } | ElementKind::Cccs { .. } => ElementType::Other,
            ElementKind::BehavioralVoltage { .. } | ElementKind::BehavioralCurrent { .. } => {
                ElementType::BehavioralSource
            }
            ElementKind::VSwitch { .. }
            | ElementKind::ISwitch { .. }
            | ElementKind::GenericSwitch { .. } => ElementType::Switch,
            ElementKind::TransmissionLine { .. } => ElementType::TransmissionLine,
            ElementKind::Coupling { .. } => ElementType::Coupling,
            ElementKind::Xspice { .. } => ElementType::Xspice,
            ElementKind::Subcircuit { .. } => ElementType::Other,
        }
    }

    fn sensitivity_instance_params(kind: &ElementKind) -> Option<&[(String, Value)]> {
        match kind {
            ElementKind::Resistor {
                instance_params, ..
            }
            | ElementKind::Capacitor {
                instance_params, ..
            }
            | ElementKind::Inductor {
                instance_params, ..
            }
            | ElementKind::Diode {
                instance_params, ..
            }
            | ElementKind::Bjt {
                instance_params, ..
            }
            | ElementKind::Mosfet {
                instance_params, ..
            }
            | ElementKind::Jfet {
                instance_params, ..
            }
            | ElementKind::Mesfet {
                instance_params, ..
            } => Some(instance_params),
            ElementKind::Xspice { params, .. } => Some(params),
            _ => None,
        }
    }

    fn sensitivity_model_name(kind: &ElementKind) -> Option<&str> {
        match kind {
            ElementKind::Resistor { model, .. }
            | ElementKind::Capacitor { model, .. }
            | ElementKind::Inductor { model, .. }
            | ElementKind::TransmissionLine { model, .. } => model.as_deref(),
            ElementKind::JilesAthertonInductor { model, .. }
            | ElementKind::Diode { model, .. }
            | ElementKind::Bjt { model, .. }
            | ElementKind::Mosfet { model, .. }
            | ElementKind::Jfet { model, .. }
            | ElementKind::Mesfet { model, .. }
            | ElementKind::VSwitch { model, .. }
            | ElementKind::ISwitch { model, .. }
            | ElementKind::GenericSwitch { model, .. }
            | ElementKind::Xspice { model, .. } => Some(model),
            _ => None,
        }
    }

    fn is_discrete_sensitivity_parameter(parameter: &str) -> bool {
        let upper = parameter.trim().to_ascii_uppercase();
        matches!(
            upper.as_str(),
            "LEVEL"
                | "VERSION"
                | "TYPE"
                | "POLARITY"
                | "PARAMCHK"
                | "BINUNIT"
                | "OFF"
                | "ON"
                | "SELECT"
                | "METHOD"
        ) || upper.ends_with("MOD")
            || upper.ends_with("MODE")
            || upper.ends_with("FLAG")
    }

    fn add_ac_sensitivity_target(
        targets: &mut Vec<AcSensitivityTarget>,
        seen: &mut HashSet<String>,
        target: AcSensitivityTarget,
    ) {
        if target.nominal_value.is_finite() && seen.insert(target.vector_name.to_ascii_uppercase())
        {
            targets.push(target);
        }
    }

    fn resolved_model_scalar_params(
        netlist: &Netlist,
        model: &crate::netlist::ModelDef,
    ) -> Result<Vec<(String, Value)>, SimulationError> {
        Self::resolved_expression_params(
            &netlist.params,
            &model.params,
            &model.expr_params,
            &format!("model '{}'", model.name),
        )
    }

    fn resolved_expression_params(
        base_context: &crate::netlist::ParamContext,
        numeric: &[(String, Value)],
        expressions: &[(String, String)],
        owner: &str,
    ) -> Result<Vec<(String, Value)>, SimulationError> {
        let mut resolved = Vec::new();
        let mut seen = HashSet::new();
        let mut context = base_context.clone();
        for (name, value) in numeric {
            context.set(name, *value);
            if seen.insert(name.to_ascii_uppercase()) {
                resolved.push((name.clone(), *value));
            }
        }

        let mut pending = expressions.to_vec();
        while !pending.is_empty() {
            let mut next = Vec::new();
            let mut progressed = false;
            for (name, expression) in pending {
                match crate::netlist::expr::eval_expression(&expression, &context) {
                    Ok(value) if value.is_finite() => {
                        context.set(&name, value);
                        if seen.insert(name.to_ascii_uppercase()) {
                            resolved.push((name, value));
                        }
                        progressed = true;
                    }
                    Ok(value) => {
                        return Err(SimulationError::Circuit(format!(
                            "AC sensitivity {owner} parameter '{name}' resolved to non-finite value {value}"
                        )));
                    }
                    Err(_) => next.push((name, expression)),
                }
            }
            if !progressed {
                let unresolved = next
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(SimulationError::Circuit(format!(
                    "AC sensitivity could not resolve {owner} parameter(s): {unresolved}"
                )));
            }
            pending = next;
        }
        Ok(resolved)
    }

    fn resolved_vector_expression_params(
        base_context: &crate::netlist::ParamContext,
        scalar_params: &[(String, Value)],
        vectors: &[(String, Vec<String>)],
        owner: &str,
    ) -> Result<Vec<(String, Vec<Value>)>, SimulationError> {
        let mut context = base_context.clone();
        for (name, value) in scalar_params {
            context.set(name, *value);
        }
        vectors
            .iter()
            .map(|(name, expressions)| {
                let values = expressions
                    .iter()
                    .map(|expression| {
                        crate::netlist::expr::eval_expression(expression, &context).map_err(
                            |error| {
                                SimulationError::Circuit(format!(
                                    "AC sensitivity could not resolve {owner} vector parameter '{name}' entry '{expression}': {error}"
                                ))
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if let Some(value) = values.iter().find(|value| !value.is_finite()) {
                    return Err(SimulationError::Circuit(format!(
                        "AC sensitivity {owner} vector parameter '{name}' resolved to non-finite value {value}"
                    )));
                }
                Ok((name.clone(), values))
            })
            .collect()
    }

    fn collect_ac_sensitivity_targets(
        netlist: &Netlist,
    ) -> Result<Vec<AcSensitivityTarget>, SimulationError> {
        let mut targets = Vec::new();
        let mut seen = HashSet::new();
        let mut referenced_models = HashSet::new();

        for (element_index, element) in netlist.elements.iter().enumerate() {
            let name = element.name.clone();
            let element_type = Self::sensitivity_element_type(&element.kind);
            if let Some(model) = Self::sensitivity_model_name(&element.kind) {
                referenced_models.insert(model.to_ascii_uppercase());
            }

            let mut primary_aliases: &[&str] = &[];
            let mut add_field =
                |field: AcSensitivityElementField, parameter: &str, nominal_value: Value| {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: name.clone(),
                            element: name.clone(),
                            element_type,
                            parameter: parameter.to_string(),
                            nominal_value,
                            location: AcSensitivityLocation::ElementField {
                                element_index,
                                field,
                            },
                        },
                    );
                };

            match &element.kind {
                ElementKind::Resistor { value, .. } if value.is_finite() => {
                    primary_aliases = &["R", "RES", "RESISTANCE", "VALUE"];
                    add_field(AcSensitivityElementField::ResistorValue, "R", *value);
                }
                ElementKind::Capacitor { value, .. } if value.is_finite() => {
                    primary_aliases = &["C", "CAP", "CAPACITANCE", "VALUE"];
                    add_field(AcSensitivityElementField::CapacitorValue, "C", *value);
                }
                ElementKind::Inductor { value, .. } if value.is_finite() => {
                    primary_aliases = &["L", "IND", "INDUCTANCE", "VALUE"];
                    add_field(AcSensitivityElementField::InductorValue, "L", *value);
                }
                ElementKind::JilesAthertonInductor { value, .. } => {
                    add_field(AcSensitivityElementField::JilesAthertonValue, "L", *value);
                }
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: name.clone(),
                            element: name.clone(),
                            element_type,
                            parameter: "DC".to_string(),
                            nominal_value: crate::engine::extract_dc_value(spec),
                            location: AcSensitivityLocation::SourceDc { element_index },
                        },
                    );
                    if Self::source_has_explicit_ac(spec) {
                        let (magnitude, phase_radians) = crate::engine::extract_ac_value(spec);
                        for (suffix, parameter, nominal_value, location) in [
                            (
                                "AC_MAG",
                                "AC_MAG",
                                magnitude,
                                AcSensitivityLocation::SourceAcMagnitude { element_index },
                            ),
                            (
                                "AC_PHASE",
                                "AC_PHASE",
                                phase_radians.to_degrees(),
                                AcSensitivityLocation::SourceAcPhaseDegrees { element_index },
                            ),
                        ] {
                            Self::add_ac_sensitivity_target(
                                &mut targets,
                                &mut seen,
                                AcSensitivityTarget {
                                    vector_name: format!("{name}_{suffix}"),
                                    element: name.clone(),
                                    element_type,
                                    parameter: parameter.to_string(),
                                    nominal_value,
                                    location,
                                },
                            );
                        }
                    }
                }
                ElementKind::Vcvs { gain, .. } => {
                    add_field(AcSensitivityElementField::VcvsGain, "GAIN", *gain);
                }
                ElementKind::Cccs { gain, .. } => {
                    add_field(AcSensitivityElementField::CccsGain, "GAIN", *gain);
                }
                ElementKind::Vccs {
                    transconductance, ..
                } => add_field(
                    AcSensitivityElementField::VccsTransconductance,
                    "GM",
                    *transconductance,
                ),
                ElementKind::Ccvs {
                    transresistance, ..
                } => add_field(
                    AcSensitivityElementField::CcvsTransresistance,
                    "RM",
                    *transresistance,
                ),
                ElementKind::BehavioralVoltage { tc1, tc2, .. }
                | ElementKind::BehavioralCurrent { tc1, tc2, .. } => {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_TC1"),
                            element: name.clone(),
                            element_type,
                            parameter: "TC1".to_string(),
                            nominal_value: *tc1,
                            location: AcSensitivityLocation::ElementField {
                                element_index,
                                field: AcSensitivityElementField::BehavioralTc1,
                            },
                        },
                    );
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_TC2"),
                            element: name.clone(),
                            element_type,
                            parameter: "TC2".to_string(),
                            nominal_value: *tc2,
                            location: AcSensitivityLocation::ElementField {
                                element_index,
                                field: AcSensitivityElementField::BehavioralTc2,
                            },
                        },
                    );
                }
                ElementKind::TransmissionLine {
                    z0, td, freq, nl, ..
                } => {
                    for (field, parameter, nominal) in [
                        (AcSensitivityElementField::TransmissionZ0, "Z0", *z0),
                        (AcSensitivityElementField::TransmissionDelay, "TD", *td),
                        (
                            AcSensitivityElementField::TransmissionFrequency,
                            "FREQ",
                            *freq,
                        ),
                        (AcSensitivityElementField::TransmissionLength, "NL", *nl),
                    ] {
                        if let Some(nominal) = nominal {
                            Self::add_ac_sensitivity_target(
                                &mut targets,
                                &mut seen,
                                AcSensitivityTarget {
                                    vector_name: format!("{name}_{parameter}"),
                                    element: name.clone(),
                                    element_type,
                                    parameter: parameter.to_string(),
                                    nominal_value: nominal,
                                    location: AcSensitivityLocation::ElementField {
                                        element_index,
                                        field,
                                    },
                                },
                            );
                        }
                    }
                }
                ElementKind::Coupling { coefficient, .. } => {
                    add_field(AcSensitivityElementField::Coupling, "K", *coefficient);
                }
                _ => {}
            }

            if let Some(parameters) = Self::sensitivity_instance_params(&element.kind) {
                for (parameter_index, (parameter, nominal_value)) in parameters.iter().enumerate() {
                    if Self::is_discrete_sensitivity_parameter(parameter)
                        || (primary_aliases
                            .iter()
                            .any(|alias| parameter.eq_ignore_ascii_case(alias))
                            && matches!(
                                element.kind,
                                ElementKind::Resistor { value, .. }
                                    | ElementKind::Capacitor { value, .. }
                                    | ElementKind::Inductor { value, .. }
                                    if value.is_finite()
                            ))
                    {
                        continue;
                    }
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_{}", parameter.to_ascii_uppercase()),
                            element: name.clone(),
                            element_type,
                            parameter: parameter.to_ascii_uppercase(),
                            nominal_value: *nominal_value,
                            location: AcSensitivityLocation::ElementParameter {
                                element_index,
                                parameter_index,
                            },
                        },
                    );
                }
            }

            if let ElementKind::Xspice {
                params,
                expr_params,
                real_vector_params,
                real_vector_expr_params,
                ..
            } = &element.kind
            {
                let resolved_scalars = Self::resolved_expression_params(
                    &netlist.params,
                    params,
                    expr_params,
                    &format!("XSPICE instance '{name}'"),
                )?;
                for (parameter, nominal_value) in &resolved_scalars {
                    if Self::is_discrete_sensitivity_parameter(parameter) {
                        continue;
                    }
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!("{name}_{}", parameter.to_ascii_uppercase()),
                            element: name.clone(),
                            element_type,
                            parameter: parameter.to_ascii_uppercase(),
                            nominal_value: *nominal_value,
                            location: AcSensitivityLocation::ElementNamedParameter {
                                element_index,
                                parameter: parameter.clone(),
                            },
                        },
                    );
                }
                for (parameter_index, (parameter, values)) in real_vector_params.iter().enumerate()
                {
                    if Self::is_discrete_sensitivity_parameter(parameter) {
                        continue;
                    }
                    for (entry_index, nominal_value) in values.iter().copied().enumerate() {
                        Self::add_ac_sensitivity_target(
                            &mut targets,
                            &mut seen,
                            AcSensitivityTarget {
                                vector_name: format!(
                                    "{name}_{}[{entry_index}]",
                                    parameter.to_ascii_uppercase()
                                ),
                                element: name.clone(),
                                element_type,
                                parameter: format!(
                                    "{}[{entry_index}]",
                                    parameter.to_ascii_uppercase()
                                ),
                                nominal_value,
                                location: AcSensitivityLocation::ElementVectorParameter {
                                    element_index,
                                    parameter_index,
                                    entry_index,
                                },
                            },
                        );
                    }
                }
                for (parameter, values) in Self::resolved_vector_expression_params(
                    &netlist.params,
                    &resolved_scalars,
                    real_vector_expr_params,
                    &format!("XSPICE instance '{name}'"),
                )? {
                    if Self::is_discrete_sensitivity_parameter(&parameter) {
                        continue;
                    }
                    for (entry_index, nominal_value) in values.iter().copied().enumerate() {
                        Self::add_ac_sensitivity_target(
                            &mut targets,
                            &mut seen,
                            AcSensitivityTarget {
                                vector_name: format!(
                                    "{name}_{}[{entry_index}]",
                                    parameter.to_ascii_uppercase()
                                ),
                                element: name.clone(),
                                element_type,
                                parameter: format!(
                                    "{}[{entry_index}]",
                                    parameter.to_ascii_uppercase()
                                ),
                                nominal_value,
                                location: AcSensitivityLocation::ElementNamedVectorParameter {
                                    element_index,
                                    parameter: parameter.clone(),
                                    entry_index,
                                    resolved_values: values.clone(),
                                },
                            },
                        );
                    }
                }
            }
        }

        for (model_index, model) in netlist.models.iter().enumerate() {
            if !referenced_models.contains(&model.name.to_ascii_uppercase()) {
                continue;
            }
            for (parameter, nominal_value) in Self::resolved_model_scalar_params(netlist, model)? {
                if Self::is_discrete_sensitivity_parameter(&parameter) {
                    continue;
                }
                Self::add_ac_sensitivity_target(
                    &mut targets,
                    &mut seen,
                    AcSensitivityTarget {
                        vector_name: format!("{}:{}", model.name, parameter.to_ascii_uppercase()),
                        element: model.name.clone(),
                        element_type: ElementType::Model,
                        parameter: parameter.to_ascii_uppercase(),
                        nominal_value,
                        location: AcSensitivityLocation::ModelParameter {
                            model_index,
                            parameter,
                        },
                    },
                );
            }
            for (parameter_index, (parameter, values)) in
                model.real_vector_params.iter().enumerate()
            {
                if Self::is_discrete_sensitivity_parameter(parameter) {
                    continue;
                }
                for (entry_index, nominal_value) in values.iter().copied().enumerate() {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!(
                                "{}:{}[{entry_index}]",
                                model.name,
                                parameter.to_ascii_uppercase()
                            ),
                            element: model.name.clone(),
                            element_type: ElementType::Model,
                            parameter: format!("{}[{entry_index}]", parameter.to_ascii_uppercase()),
                            nominal_value,
                            location: AcSensitivityLocation::ModelVectorParameter {
                                model_index,
                                parameter_index,
                                entry_index,
                            },
                        },
                    );
                }
            }
            let resolved_scalars = Self::resolved_model_scalar_params(netlist, model)?;
            for (parameter, values) in Self::resolved_vector_expression_params(
                &netlist.params,
                &resolved_scalars,
                &model.real_vector_expr_params,
                &format!("model '{}'", model.name),
            )? {
                if Self::is_discrete_sensitivity_parameter(&parameter) {
                    continue;
                }
                for (entry_index, nominal_value) in values.iter().copied().enumerate() {
                    Self::add_ac_sensitivity_target(
                        &mut targets,
                        &mut seen,
                        AcSensitivityTarget {
                            vector_name: format!(
                                "{}:{}[{entry_index}]",
                                model.name,
                                parameter.to_ascii_uppercase()
                            ),
                            element: model.name.clone(),
                            element_type: ElementType::Model,
                            parameter: format!("{}[{entry_index}]", parameter.to_ascii_uppercase()),
                            nominal_value,
                            location: AcSensitivityLocation::ModelNamedVectorParameter {
                                model_index,
                                parameter: parameter.clone(),
                                entry_index,
                                resolved_values: values.clone(),
                            },
                        },
                    );
                }
            }
        }

        targets.sort_by(|left, right| {
            left.vector_name
                .to_ascii_uppercase()
                .cmp(&right.vector_name.to_ascii_uppercase())
        });
        Ok(targets)
    }

    fn update_primary_instance_aliases(
        parameters: &mut [(String, Value)],
        aliases: &[&str],
        value: Value,
    ) {
        for (name, existing) in parameters {
            if aliases.iter().any(|alias| name.eq_ignore_ascii_case(alias)) {
                *existing = value;
            }
        }
    }

    fn set_source_dc_for_sensitivity(spec: &mut SourceSpec, value: Value) {
        let owned = std::mem::replace(spec, SourceSpec::Dc(0.0));
        *spec = owned.with_dc_value(value);
    }

    fn set_source_ac_for_sensitivity(
        spec: &mut SourceSpec,
        magnitude: Value,
        phase_radians: Value,
    ) {
        let owned = std::mem::replace(spec, SourceSpec::Dc(0.0));
        *spec = owned.with_ac(magnitude, phase_radians);
    }

    fn apply_ac_sensitivity_target(
        netlist: &mut Netlist,
        target: &AcSensitivityTarget,
        value: Value,
    ) -> Result<(), SimulationError> {
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "Sensitivity perturbation for '{}' is non-finite: {value}",
                target.vector_name
            )));
        }
        match &target.location {
            AcSensitivityLocation::ElementField {
                element_index,
                field,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                match (field, &mut element.kind) {
                    (
                        AcSensitivityElementField::ResistorValue,
                        ElementKind::Resistor {
                            value: nominal,
                            value_expr,
                            instance_params,
                            ..
                        },
                    ) => {
                        *nominal = value;
                        *value_expr = None;
                        Self::update_primary_instance_aliases(
                            instance_params,
                            &["R", "RES", "RESISTANCE", "VALUE"],
                            value,
                        );
                    }
                    (
                        AcSensitivityElementField::CapacitorValue,
                        ElementKind::Capacitor {
                            value: nominal,
                            value_expr,
                            instance_params,
                            ..
                        },
                    ) => {
                        *nominal = value;
                        *value_expr = None;
                        Self::update_primary_instance_aliases(
                            instance_params,
                            &["C", "CAP", "CAPACITANCE", "VALUE"],
                            value,
                        );
                    }
                    (
                        AcSensitivityElementField::InductorValue,
                        ElementKind::Inductor {
                            value: nominal,
                            value_expr,
                            instance_params,
                            ..
                        },
                    ) => {
                        *nominal = value;
                        *value_expr = None;
                        Self::update_primary_instance_aliases(
                            instance_params,
                            &["L", "IND", "INDUCTANCE", "VALUE"],
                            value,
                        );
                    }
                    (
                        AcSensitivityElementField::JilesAthertonValue,
                        ElementKind::JilesAthertonInductor { value: nominal, .. },
                    ) => *nominal = value,
                    (
                        AcSensitivityElementField::VcvsGain,
                        ElementKind::Vcvs {
                            gain, gain_expr, ..
                        },
                    )
                    | (
                        AcSensitivityElementField::CccsGain,
                        ElementKind::Cccs {
                            gain, gain_expr, ..
                        },
                    ) => {
                        *gain = value;
                        *gain_expr = None;
                    }
                    (
                        AcSensitivityElementField::VccsTransconductance,
                        ElementKind::Vccs {
                            transconductance,
                            transconductance_expr,
                            ..
                        },
                    ) => {
                        *transconductance = value;
                        *transconductance_expr = None;
                    }
                    (
                        AcSensitivityElementField::CcvsTransresistance,
                        ElementKind::Ccvs {
                            transresistance,
                            transresistance_expr,
                            ..
                        },
                    ) => {
                        *transresistance = value;
                        *transresistance_expr = None;
                    }
                    (
                        AcSensitivityElementField::BehavioralTc1,
                        ElementKind::BehavioralVoltage { tc1, .. }
                        | ElementKind::BehavioralCurrent { tc1, .. },
                    ) => *tc1 = value,
                    (
                        AcSensitivityElementField::BehavioralTc2,
                        ElementKind::BehavioralVoltage { tc2, .. }
                        | ElementKind::BehavioralCurrent { tc2, .. },
                    ) => *tc2 = value,
                    (
                        AcSensitivityElementField::TransmissionZ0,
                        ElementKind::TransmissionLine { z0, .. },
                    ) => *z0 = Some(value),
                    (
                        AcSensitivityElementField::TransmissionDelay,
                        ElementKind::TransmissionLine { td, .. },
                    ) => *td = Some(value),
                    (
                        AcSensitivityElementField::TransmissionFrequency,
                        ElementKind::TransmissionLine { freq, .. },
                    ) => *freq = Some(value),
                    (
                        AcSensitivityElementField::TransmissionLength,
                        ElementKind::TransmissionLine { nl, .. },
                    ) => *nl = Some(value),
                    (
                        AcSensitivityElementField::Coupling,
                        ElementKind::Coupling { coefficient, .. },
                    ) => *coefficient = value,
                    _ => {
                        return Err(SimulationError::Circuit(format!(
                            "Sensitivity target '{}' no longer matches its element kind",
                            target.vector_name
                        )));
                    }
                }
            }
            AcSensitivityLocation::ElementParameter {
                element_index,
                parameter_index,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let parameters = match &mut element.kind {
                    ElementKind::Resistor {
                        instance_params, ..
                    }
                    | ElementKind::Capacitor {
                        instance_params, ..
                    }
                    | ElementKind::Inductor {
                        instance_params, ..
                    }
                    | ElementKind::Diode {
                        instance_params, ..
                    }
                    | ElementKind::Bjt {
                        instance_params, ..
                    }
                    | ElementKind::Mosfet {
                        instance_params, ..
                    }
                    | ElementKind::Jfet {
                        instance_params, ..
                    }
                    | ElementKind::Mesfet {
                        instance_params, ..
                    } => instance_params,
                    ElementKind::Xspice { params, .. } => params,
                    _ => {
                        return Err(SimulationError::Circuit(format!(
                            "Sensitivity target '{}' has no scalar instance parameters",
                            target.vector_name
                        )));
                    }
                };
                let (_, nominal) = parameters.get_mut(*parameter_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing parameter index {}",
                        target.vector_name, parameter_index
                    ))
                })?;
                *nominal = value;
            }
            AcSensitivityLocation::ElementNamedParameter {
                element_index,
                parameter,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let ElementKind::Xspice {
                    params,
                    expr_params,
                    ..
                } = &mut element.kind
                else {
                    return Err(SimulationError::Circuit(format!(
                        "Sensitivity target '{}' is not a named XSPICE parameter",
                        target.vector_name
                    )));
                };
                expr_params.retain(|(name, _)| !name.eq_ignore_ascii_case(parameter));
                if let Some((_, nominal)) = params
                    .iter_mut()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    *nominal = value;
                } else {
                    params.push((parameter.to_ascii_uppercase(), value));
                }
            }
            AcSensitivityLocation::ElementVectorParameter {
                element_index,
                parameter_index,
                entry_index,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let ElementKind::Xspice {
                    real_vector_params, ..
                } = &mut element.kind
                else {
                    return Err(SimulationError::Circuit(format!(
                        "Sensitivity target '{}' is not an XSPICE vector parameter",
                        target.vector_name
                    )));
                };
                let (_, values) = real_vector_params.get_mut(*parameter_index).ok_or_else(
                    || {
                        SimulationError::Circuit(format!(
                            "Sensitivity target '{}' references missing vector parameter index {}",
                            target.vector_name, parameter_index
                        ))
                    },
                )?;
                let nominal = values.get_mut(*entry_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing vector entry {}",
                        target.vector_name, entry_index
                    ))
                })?;
                *nominal = value;
            }
            AcSensitivityLocation::ElementNamedVectorParameter {
                element_index,
                parameter,
                entry_index,
                resolved_values,
            } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing element index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let ElementKind::Xspice {
                    real_vector_params,
                    real_vector_expr_params,
                    ..
                } = &mut element.kind
                else {
                    return Err(SimulationError::Circuit(format!(
                        "Sensitivity target '{}' is not a named XSPICE vector parameter",
                        target.vector_name
                    )));
                };
                real_vector_expr_params.retain(|(name, _)| !name.eq_ignore_ascii_case(parameter));
                let values = if let Some((_, values)) = real_vector_params
                    .iter_mut()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    values
                } else {
                    real_vector_params
                        .push((parameter.to_ascii_uppercase(), resolved_values.clone()));
                    &mut real_vector_params
                        .last_mut()
                        .expect("just inserted vector parameter")
                        .1
                };
                let nominal = values.get_mut(*entry_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing vector entry {}",
                        target.vector_name, entry_index
                    ))
                })?;
                *nominal = value;
            }
            AcSensitivityLocation::SourceDc { element_index }
            | AcSensitivityLocation::SourceAcMagnitude { element_index }
            | AcSensitivityLocation::SourceAcPhaseDegrees { element_index } => {
                let element = netlist.elements.get_mut(*element_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing source index {}",
                        target.vector_name, element_index
                    ))
                })?;
                let spec = match &mut element.kind {
                    ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => spec,
                    _ => {
                        return Err(SimulationError::Circuit(format!(
                            "Sensitivity target '{}' is not an independent source",
                            target.vector_name
                        )));
                    }
                };
                match target.location {
                    AcSensitivityLocation::SourceDc { .. } => {
                        Self::set_source_dc_for_sensitivity(spec, value);
                    }
                    AcSensitivityLocation::SourceAcMagnitude { .. } => {
                        let (_, phase) = crate::engine::extract_ac_value(spec);
                        Self::set_source_ac_for_sensitivity(spec, value, phase);
                    }
                    AcSensitivityLocation::SourceAcPhaseDegrees { .. } => {
                        let (magnitude, _) = crate::engine::extract_ac_value(spec);
                        Self::set_source_ac_for_sensitivity(spec, magnitude, value.to_radians());
                    }
                    _ => unreachable!("matched source sensitivity location"),
                }
            }
            AcSensitivityLocation::ModelParameter {
                model_index,
                parameter,
            } => {
                let model = netlist.models.get_mut(*model_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model index {}",
                        target.vector_name, model_index
                    ))
                })?;
                model
                    .expr_params
                    .retain(|(name, _)| !name.eq_ignore_ascii_case(parameter));
                if let Some((_, nominal)) = model
                    .params
                    .iter_mut()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    *nominal = value;
                } else {
                    model.params.push((parameter.to_ascii_uppercase(), value));
                }
            }
            AcSensitivityLocation::ModelVectorParameter {
                model_index,
                parameter_index,
                entry_index,
            } => {
                let model = netlist.models.get_mut(*model_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model index {}",
                        target.vector_name, model_index
                    ))
                })?;
                let (_, values) = model
                    .real_vector_params
                    .get_mut(*parameter_index)
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Sensitivity target '{}' references missing model vector index {}",
                            target.vector_name, parameter_index
                        ))
                    })?;
                let nominal = values.get_mut(*entry_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model vector entry {}",
                        target.vector_name, entry_index
                    ))
                })?;
                *nominal = value;
            }
            AcSensitivityLocation::ModelNamedVectorParameter {
                model_index,
                parameter,
                entry_index,
                resolved_values,
            } => {
                let model = netlist.models.get_mut(*model_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model index {}",
                        target.vector_name, model_index
                    ))
                })?;
                model
                    .real_vector_expr_params
                    .retain(|(name, _)| !name.eq_ignore_ascii_case(parameter));
                let values = if let Some((_, values)) = model
                    .real_vector_params
                    .iter_mut()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    values
                } else {
                    model
                        .real_vector_params
                        .push((parameter.to_ascii_uppercase(), resolved_values.clone()));
                    &mut model
                        .real_vector_params
                        .last_mut()
                        .expect("just inserted model vector parameter")
                        .1
                };
                let nominal = values.get_mut(*entry_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity target '{}' references missing model vector entry {}",
                        target.vector_name, entry_index
                    ))
                })?;
                *nominal = value;
            }
        }
        Ok(())
    }

    fn sensitivity_glob_matches(pattern: &str, candidate: &str) -> bool {
        let pattern = pattern.to_ascii_uppercase();
        let candidate = candidate.to_ascii_uppercase();
        let pattern = pattern.as_bytes();
        let candidate = candidate.as_bytes();
        let (mut p, mut c) = (0usize, 0usize);
        let mut star = None;
        while c < candidate.len() {
            if p < pattern.len() && (pattern[p] == b'?' || pattern[p] == candidate[c]) {
                p += 1;
                c += 1;
            } else if p < pattern.len() && pattern[p] == b'*' {
                star = Some((p, c));
                p += 1;
            } else if let Some((star_p, star_c)) = star {
                star = Some((star_p, star_c + 1));
                p = star_p + 1;
                c = star_c + 1;
            } else {
                return false;
            }
        }
        while p < pattern.len() && pattern[p] == b'*' {
            p += 1;
        }
        p == pattern.len()
    }

    fn sensitivity_target_selected(target: &AcSensitivityTarget, filters: &[String]) -> bool {
        filters.is_empty()
            || filters.iter().any(|filter| {
                Self::sensitivity_glob_matches(filter, &target.vector_name)
                    || Self::sensitivity_glob_matches(filter, &target.element)
                    || Self::sensitivity_glob_matches(
                        filter,
                        &format!("{}:{}", target.element, target.parameter),
                    )
            })
    }

    fn complete_sensitivity_step(target: &AcSensitivityTarget) -> Value {
        let parameter = target.parameter.to_ascii_uppercase();
        let absolute_floor = if parameter.contains("PHASE") {
            1.0e-3
        } else if matches!(
            parameter.as_str(),
            "C" | "CAP" | "CAPACITANCE" | "CJ" | "CJO" | "CGSO" | "CGDO" | "CGBO"
        ) || parameter.starts_with('C')
            && (parameter.contains('J') || parameter.contains("CAP"))
        {
            1.0e-18
        } else if matches!(parameter.as_str(), "L" | "IND" | "INDUCTANCE") {
            1.0e-15
        } else if matches!(parameter.as_str(), "DC" | "AC_MAG") {
            1.0e-9
        } else {
            1.0e-12
        };
        (target.nominal_value.abs() * 1.0e-3).max(absolute_floor)
    }

    fn ac_sensitivity_output_value(
        result: &crate::analysis::AcResult,
        output: &AcSensitivityOutput,
    ) -> Result<Complex64, SimulationError> {
        let voltage = |node: usize| -> Result<Complex64, SimulationError> {
            if node == 0 {
                return Ok(Complex64::new(0.0, 0.0));
            }
            result.voltages.get(node - 1).copied().ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "Sensitivity output node {node} is outside circuit node range 0..={}",
                    result.voltages.len()
                ))
            })
        };
        match output {
            AcSensitivityOutput::Voltage { positive, negative } => {
                Ok(voltage(*positive)? - voltage(negative.unwrap_or(0))?)
            }
            AcSensitivityOutput::BranchCurrent(element) => result
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(element))
                .and_then(|index| result.currents.get(index).copied())
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Sensitivity branch-current output I({element}) is unavailable; the element must own an AC MNA branch"
                    ))
                }),
        }
    }

    fn ac_sensitivity_outputs(
        results: &[crate::analysis::AcResult],
        output: &AcSensitivityOutput,
        expected_frequencies: &[Value],
    ) -> Result<Vec<Complex64>, SimulationError> {
        if results.len() != expected_frequencies.len() {
            return Err(SimulationError::Circuit(format!(
                "AC sensitivity produced {} samples for a {}-point frequency grid",
                results.len(),
                expected_frequencies.len()
            )));
        }
        results
            .iter()
            .zip(expected_frequencies)
            .map(|(result, expected)| {
                let tolerance = expected.abs().max(1.0) * 1.0e-12;
                if (result.frequency - expected).abs() > tolerance {
                    return Err(SimulationError::Circuit(format!(
                        "AC sensitivity frequency mismatch: expected {expected}, got {}",
                        result.frequency
                    )));
                }
                Self::ac_sensitivity_output_value(result, output)
            })
            .collect()
    }

    fn complete_ac_sensitivity_trace(
        target: &AcSensitivityTarget,
        nominal_output: &[Complex64],
        derivative: Vec<Complex64>,
    ) -> AcSensitivity {
        let mut normalized = Vec::with_capacity(derivative.len());
        let mut magnitude = Vec::with_capacity(derivative.len());
        let mut phase = Vec::with_capacity(derivative.len());
        for (&output, &sensitivity) in nominal_output.iter().zip(&derivative) {
            let norm_sqr = output.norm_sqr();
            if norm_sqr > 1.0e-60 {
                normalized.push(sensitivity * target.nominal_value / output);
                let product = output.conj() * sensitivity;
                magnitude.push(product.re / norm_sqr.sqrt());
                phase.push(product.im / norm_sqr);
            } else {
                normalized.push(Complex64::new(0.0, 0.0));
                magnitude.push(0.0);
                phase.push(0.0);
            }
        }
        AcSensitivity {
            vector_name: target.vector_name.clone(),
            element: target.element.clone(),
            element_type: target.element_type,
            parameter: target.parameter.clone(),
            nominal_value: target.nominal_value,
            absolute: derivative,
            normalized,
            magnitude,
            phase,
        }
    }

    fn dc_sensitivity_output_value(
        result: &SimulationResult,
        output: &AcSensitivityOutput,
    ) -> Result<Value, SimulationError> {
        match output {
            AcSensitivityOutput::Voltage { positive, negative } => {
                let positive_value = result.try_voltage(*positive).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "DC sensitivity output node {positive} is outside the solved node range"
                    ))
                })?;
                let negative_value = negative
                    .map(|node| {
                        result.try_voltage(node).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "DC sensitivity reference node {node} is outside the solved node range"
                            ))
                        })
                    })
                    .transpose()?
                    .unwrap_or(0.0);
                Ok(positive_value - negative_value)
            }
            AcSensitivityOutput::BranchCurrent(element) => {
                let matches = result
                    .branch_names
                    .iter()
                    .enumerate()
                    .filter(|(_, name)| name.eq_ignore_ascii_case(element))
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();
                match matches.as_slice() {
                    [] => Err(SimulationError::Circuit(format!(
                        "DC sensitivity branch-current output '{element}' was not found"
                    ))),
                    [index] => result.branch_currents.get(*index).copied().ok_or_else(|| {
                        SimulationError::Circuit(
                            "DC sensitivity branch-current result is malformed".to_string(),
                        )
                    }),
                    _ => Err(SimulationError::Circuit(format!(
                        "DC sensitivity branch-current output '{element}' is ambiguous"
                    ))),
                }
            }
        }
    }

    fn dc_sensitivity_target_active(target: &AcSensitivityTarget) -> bool {
        !matches!(
            target.location,
            AcSensitivityLocation::SourceAcMagnitude { .. }
                | AcSensitivityLocation::SourceAcPhaseDegrees { .. }
        )
    }

    /// Run complete netlist-wide DC sensitivity for every eligible real
    /// parameter in the flattened circuit. Unlike the legacy adjoint helper,
    /// this covers nonlinear devices, models, hierarchy, branch-current
    /// outputs, and SPICE device filters.
    pub fn run_sensitivity_dc_complete(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        filters: &[String],
    ) -> Result<SensitivityResult, SimulationError> {
        self.run_sensitivity_dc_complete_with_abort(netlist, output, filters, &NoAbort)
    }

    /// Complete DC sensitivity with cooperative cancellation.
    pub fn run_sensitivity_dc_complete_with_abort(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        filters: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<SensitivityResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let AcSensitivityOutput::Voltage { positive: 0, .. } = output {
            return Err(SimulationError::Circuit(
                "Sensitivity output node must not be ground".to_string(),
            ));
        }
        if let AcSensitivityOutput::Voltage {
            positive,
            negative: Some(negative),
        } = output
            && positive == negative
        {
            return Err(SimulationError::Circuit(
                "Sensitivity output and reference nodes must differ".to_string(),
            ));
        }

        let flat = Self::flattened_sensitivity_netlist(netlist)?;
        let nominal_result = self.run_dc_op_with_abort(&flat, abort)?;
        let nominal_output = Self::dc_sensitivity_output_value(&nominal_result, &output)?;
        let targets = Self::collect_ac_sensitivity_targets(&flat)?
            .into_iter()
            .filter(Self::dc_sensitivity_target_active)
            .filter(|target| Self::sensitivity_target_selected(target, filters))
            .collect::<Vec<_>>();
        if targets.is_empty() {
            let detail = if filters.is_empty() {
                "the flattened circuit has no eligible real-valued DC parameters".to_string()
            } else {
                format!("no DC parameter matched filter(s) {}", filters.join(", "))
            };
            return Err(SimulationError::Circuit(format!(
                "DC sensitivity cannot run: {detail}"
            )));
        }

        let output_name = match &output {
            AcSensitivityOutput::Voltage { positive, negative } => negative.map_or_else(
                || format!("V({positive})"),
                |negative| format!("V({positive},{negative})"),
            ),
            AcSensitivityOutput::BranchCurrent(element) => format!("I({element})"),
        };
        let mut result = SensitivityResult::new(&output_name, nominal_output);
        result.sensitivities.reserve(targets.len());

        for target in targets {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let h = Self::complete_sensitivity_step(&target);
            let mut plus_netlist = flat.clone();
            let mut minus_netlist = flat.clone();
            Self::apply_ac_sensitivity_target(
                &mut plus_netlist,
                &target,
                target.nominal_value + h,
            )?;
            Self::apply_ac_sensitivity_target(
                &mut minus_netlist,
                &target,
                target.nominal_value - h,
            )?;

            let plus = self.run_dc_op_with_abort(&plus_netlist, abort);
            let minus = self.run_dc_op_with_abort(&minus_netlist, abort);
            let derivative = match (plus, minus) {
                (Ok(plus), Ok(minus)) => {
                    (Self::dc_sensitivity_output_value(&plus, &output)?
                        - Self::dc_sensitivity_output_value(&minus, &output)?)
                        / (2.0 * h)
                }
                (Ok(plus), Err(minus_error)) => {
                    let mut plus_two_netlist = flat.clone();
                    Self::apply_ac_sensitivity_target(
                        &mut plus_two_netlist,
                        &target,
                        target.nominal_value + 2.0 * h,
                    )?;
                    let plus_two = self
                        .run_dc_op_with_abort(&plus_two_netlist, abort)
                        .map_err(|plus_two_error| {
                            SimulationError::Circuit(format!(
                                "DC sensitivity '{}' failed for the negative and second positive perturbations: {}; {}",
                                target.vector_name, minus_error, plus_two_error
                            ))
                        })?;
                    (-3.0 * nominal_output
                        + 4.0 * Self::dc_sensitivity_output_value(&plus, &output)?
                        - Self::dc_sensitivity_output_value(&plus_two, &output)?)
                        / (2.0 * h)
                }
                (Err(plus_error), Ok(minus)) => {
                    let mut minus_two_netlist = flat.clone();
                    Self::apply_ac_sensitivity_target(
                        &mut minus_two_netlist,
                        &target,
                        target.nominal_value - 2.0 * h,
                    )?;
                    let minus_two = self
                        .run_dc_op_with_abort(&minus_two_netlist, abort)
                        .map_err(|minus_two_error| {
                            SimulationError::Circuit(format!(
                                "DC sensitivity '{}' failed for the positive and second negative perturbations: {}; {}",
                                target.vector_name, plus_error, minus_two_error
                            ))
                        })?;
                    (3.0 * nominal_output
                        - 4.0 * Self::dc_sensitivity_output_value(&minus, &output)?
                        + Self::dc_sensitivity_output_value(&minus_two, &output)?)
                        / (2.0 * h)
                }
                (Err(plus_error), Err(minus_error)) => {
                    return Err(SimulationError::Circuit(format!(
                        "DC sensitivity '{}' failed at both perturbations: positive: {}; negative: {}",
                        target.vector_name, plus_error, minus_error
                    )));
                }
            };
            if !derivative.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "DC sensitivity '{}' produced a non-finite derivative",
                    target.vector_name
                )));
            }
            result.add(Sensitivity::new_named(
                &target.vector_name,
                &target.element,
                target.element_type,
                &target.parameter,
                target.nominal_value,
                derivative,
                nominal_output,
            ));
        }

        Ok(result)
    }

    /// Run complete AC sensitivity for every eligible real-valued parameter
    /// in the flattened netlist. The returned derivatives are complex and
    /// unnormalized, matching SPICE `.SENS AC` semantics; normalized,
    /// magnitude, and phase derivatives are retained alongside them.
    pub fn run_sensitivity_ac_complete(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        frequencies: &[Value],
        filters: &[String],
    ) -> Result<AcSensitivityResult, SimulationError> {
        self.run_sensitivity_ac_complete_with_abort(netlist, output, frequencies, filters, &NoAbort)
    }

    /// Complete AC sensitivity with cooperative cancellation.
    pub fn run_sensitivity_ac_complete_with_abort(
        &self,
        netlist: &Netlist,
        output: AcSensitivityOutput,
        frequencies: &[Value],
        filters: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<AcSensitivityResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if frequencies.is_empty()
            || frequencies
                .iter()
                .any(|frequency| !frequency.is_finite() || *frequency < 0.0)
        {
            return Err(SimulationError::Circuit(
                "AC sensitivity frequencies must be a non-empty list of finite, non-negative values"
                    .to_string(),
            ));
        }
        if let AcSensitivityOutput::Voltage { positive: 0, .. } = output {
            return Err(SimulationError::Circuit(
                "Sensitivity output node must not be ground".to_string(),
            ));
        }

        let flat = Self::flattened_sensitivity_netlist(netlist)?;
        let nominal_results = self.run_ac_with_abort(&flat, frequencies, abort)?;
        let nominal_output = Self::ac_sensitivity_outputs(&nominal_results, &output, frequencies)?;
        let targets = Self::collect_ac_sensitivity_targets(&flat)?
            .into_iter()
            .filter(|target| Self::sensitivity_target_selected(target, filters))
            .collect::<Vec<_>>();
        if targets.is_empty() {
            let detail = if filters.is_empty() {
                "the flattened circuit has no eligible real-valued parameters".to_string()
            } else {
                format!("no parameter matched filter(s) {}", filters.join(", "))
            };
            return Err(SimulationError::Circuit(format!(
                "AC sensitivity cannot run: {detail}"
            )));
        }

        let output_name = match &output {
            AcSensitivityOutput::Voltage { positive, negative } => negative.map_or_else(
                || format!("V({positive})"),
                |negative| format!("V({positive},{negative})"),
            ),
            AcSensitivityOutput::BranchCurrent(element) => format!("I({element})"),
        };
        let mut sensitivities = Vec::with_capacity(targets.len());
        for target in targets {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let h = Self::complete_sensitivity_step(&target);
            let mut plus_netlist = flat.clone();
            let mut minus_netlist = flat.clone();
            Self::apply_ac_sensitivity_target(
                &mut plus_netlist,
                &target,
                target.nominal_value + h,
            )?;
            Self::apply_ac_sensitivity_target(
                &mut minus_netlist,
                &target,
                target.nominal_value - h,
            )?;

            let plus = self.run_ac_with_abort(&plus_netlist, frequencies, abort);
            let minus = self.run_ac_with_abort(&minus_netlist, frequencies, abort);
            let derivative = match (plus, minus) {
                (Ok(plus), Ok(minus)) => {
                    let plus = Self::ac_sensitivity_outputs(&plus, &output, frequencies)?;
                    let minus = Self::ac_sensitivity_outputs(&minus, &output, frequencies)?;
                    plus.iter()
                        .zip(&minus)
                        .map(|(plus, minus)| (*plus - *minus) / (2.0 * h))
                        .collect()
                }
                (Ok(plus), Err(minus_error)) => {
                    let mut plus_two_netlist = flat.clone();
                    Self::apply_ac_sensitivity_target(
                        &mut plus_two_netlist,
                        &target,
                        target.nominal_value + 2.0 * h,
                    )?;
                    let plus_two = self
                        .run_ac_with_abort(&plus_two_netlist, frequencies, abort)
                        .map_err(|plus_two_error| {
                            SimulationError::Circuit(format!(
                                "AC sensitivity '{}' failed for the negative and second positive perturbations: {}; {}",
                                target.vector_name, minus_error, plus_two_error
                            ))
                        })?;
                    let plus = Self::ac_sensitivity_outputs(&plus, &output, frequencies)?;
                    let plus_two = Self::ac_sensitivity_outputs(&plus_two, &output, frequencies)?;
                    nominal_output
                        .iter()
                        .zip(&plus)
                        .zip(&plus_two)
                        .map(|((nominal, plus), plus_two)| {
                            (-3.0 * *nominal + 4.0 * *plus - *plus_two) / (2.0 * h)
                        })
                        .collect()
                }
                (Err(plus_error), Ok(minus)) => {
                    let mut minus_two_netlist = flat.clone();
                    Self::apply_ac_sensitivity_target(
                        &mut minus_two_netlist,
                        &target,
                        target.nominal_value - 2.0 * h,
                    )?;
                    let minus_two = self
                        .run_ac_with_abort(&minus_two_netlist, frequencies, abort)
                        .map_err(|minus_two_error| {
                            SimulationError::Circuit(format!(
                                "AC sensitivity '{}' failed for the positive and second negative perturbations: {}; {}",
                                target.vector_name, plus_error, minus_two_error
                            ))
                        })?;
                    let minus = Self::ac_sensitivity_outputs(&minus, &output, frequencies)?;
                    let minus_two = Self::ac_sensitivity_outputs(&minus_two, &output, frequencies)?;
                    nominal_output
                        .iter()
                        .zip(&minus)
                        .zip(&minus_two)
                        .map(|((nominal, minus), minus_two)| {
                            (3.0 * *nominal - 4.0 * *minus + *minus_two) / (2.0 * h)
                        })
                        .collect()
                }
                (Err(plus_error), Err(minus_error)) => {
                    return Err(SimulationError::Circuit(format!(
                        "AC sensitivity '{}' failed at both perturbations: positive: {}; negative: {}",
                        target.vector_name, plus_error, minus_error
                    )));
                }
            };
            sensitivities.push(Self::complete_ac_sensitivity_trace(
                &target,
                &nominal_output,
                derivative,
            ));
        }

        Ok(AcSensitivityResult {
            output: output_name,
            frequencies: frequencies.to_vec(),
            output_values: nominal_output,
            sensitivities,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Engine;
    use crate::Netlist;
    use crate::analysis::AcSensitivityOutput;
    use crate::netlist::AnalysisCommand;
    use crate::netlist::{StepCommand, StepSweep, StepTarget};

    #[test]
    fn source_override_construction_honors_mid_build_cancellation() {
        let abort = crate::abort_signal::CountingAbort::new(1);
        let error = Engine::build_overridden_source_multi_with_abort(
            "override cancellation\n.param p=1\nR1 1 0 {p}\n.end\n",
            &[("P".to_string(), 2.0)],
            &abort,
        )
        .expect_err("source rewrite must poll while applying overrides");
        assert!(matches!(error, crate::SimulationError::Aborted));
        assert!(abort.count() >= 2);
    }

    const PARAMETRIC_DIVIDER: &str = "\
Parametric divider
.param rval=1k
V1 1 0 10
R1 1 2 {rval}
R2 2 0 1k
.end
";

    const MODEL_STEP_DECK: &str = "\
Model step deck
V1 1 0 10
R1 1 2 RMOD L=10u W=1u
R2 2 0 1k
.model RMOD R RSH=100
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

    const AC_DIVIDER: &str = "\
AC sensitivity divider
V1 in 0 DC 0 AC 1 0
R1 in out 1k
R2 out 0 1k
.end
";

    const DC_DIVIDER: &str = "\
DC sensitivity divider
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.end
";

    #[test]
    fn complete_dc_sensitivity_reports_device_and_source_derivatives() {
        let netlist = Netlist::parse(DC_DIVIDER).expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_dc_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &[],
            )
            .expect("complete DC sensitivity runs");
        assert!(
            (result.output_value - 5.0).abs() < 1e-9,
            "output={}",
            result.output_value
        );
        let r1 = result.get("R1").expect("R1 sensitivity");
        let r2 = result.get("R2").expect("R2 sensitivity");
        let v1 = result.get("V1").expect("source sensitivity");
        assert!((r1.absolute + 2.5e-3).abs() < 1e-8);
        assert!((r2.absolute - 2.5e-3).abs() < 1e-8);
        assert!((v1.absolute - 0.5).abs() < 1e-9);
        assert!((r1.normalized + 0.5).abs() < 2e-7);
    }

    #[test]
    fn complete_dc_sensitivity_supports_branch_current_and_filters() {
        let netlist = Netlist::parse(DC_DIVIDER).expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_dc_complete(
                &netlist,
                AcSensitivityOutput::BranchCurrent("v1".to_string()),
                &["R1".to_string()],
            )
            .expect("branch-current DC sensitivity runs");
        assert_eq!(result.output, "I(v1)");
        assert_eq!(result.len(), 1);
        assert!((result.output_value + 5.0e-3).abs() < 1e-12);
        assert!((result.sensitivities[0].absolute - 2.5e-6).abs() < 1e-11);
    }

    #[test]
    fn complete_dc_sensitivity_flattens_hierarchy_and_filters_parameters() {
        let netlist = Netlist::parse(
            "Hierarchical DC sensitivity\n\
V1 in 0 10\n\
XDIV in out DIVIDER\n\
.subckt DIVIDER input output\n\
RTOP input output 1k\n\
RBOT output 0 1k\n\
.ends\n\
.end\n",
        )
        .expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_dc_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &["*RTOP".to_string()],
            )
            .expect("hierarchical filtered DC sensitivity runs");
        assert_eq!(result.len(), 1);
        assert!(
            result.sensitivities[0]
                .vector_name
                .to_ascii_uppercase()
                .ends_with("RTOP")
        );
        assert!((result.sensitivities[0].absolute + 2.5e-3).abs() < 1e-8);
    }

    #[test]
    fn complete_ac_sensitivity_reports_complex_device_and_source_derivatives() {
        let netlist = Netlist::parse(AC_DIVIDER).expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &[1.0, 1.0e3],
                &[],
            )
            .expect("complete AC sensitivity runs");

        assert_eq!(result.frequencies, vec![1.0, 1.0e3]);
        assert!((result.output_values[0].re - 0.5).abs() < 1e-12);
        let r1 = result.get("R1").expect("R1 primary sensitivity");
        let r2 = result.get("R2").expect("R2 primary sensitivity");
        assert!((r1.absolute[0].re + 2.5e-4).abs() < 1e-9);
        assert!((r2.absolute[0].re - 2.5e-4).abs() < 1e-9);
        assert!(r1.absolute[0].im.abs() < 1e-12);

        let ac_magnitude = result
            .get("V1_AC_MAG")
            .expect("source AC magnitude sensitivity");
        assert!((ac_magnitude.absolute[0].re - 0.5).abs() < 1e-9);
        let ac_phase = result
            .get("V1_AC_PHASE")
            .expect("source AC phase sensitivity");
        assert!((ac_phase.absolute[0].im - 0.5_f64.to_radians()).abs() < 1e-9);
    }

    #[test]
    fn complete_ac_sensitivity_filters_and_flattens_hierarchy() {
        let netlist = Netlist::parse(
            "Hierarchical sensitivity\n\
V1 in 0 AC 1\n\
XDIV in out DIVIDER\n\
.subckt DIVIDER input output\n\
RTOP input output 1k\n\
RBOT output 0 1k\n\
.ends\n\
.end\n",
        )
        .expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &[1.0e3],
                &["*RTOP".to_string()],
            )
            .expect("hierarchical filtered sensitivity runs");
        assert_eq!(result.len(), 1);
        assert!(
            result.sensitivities[0]
                .vector_name
                .to_ascii_uppercase()
                .ends_with("RTOP")
        );
        assert!((result.sensitivities[0].absolute[0].re + 2.5e-4).abs() < 1e-9);
    }

    #[test]
    fn complete_ac_sensitivity_supports_branch_current_outputs() {
        let netlist = Netlist::parse(AC_DIVIDER).expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::BranchCurrent("v1".to_string()),
                &[1.0e3],
                &["R1".to_string()],
            )
            .expect("branch current sensitivity runs");
        assert_eq!(result.output, "I(v1)");
        assert_eq!(result.len(), 1);
        assert!((result.sensitivities[0].absolute[0].re - 2.5e-7).abs() < 1e-12);
    }

    #[test]
    fn complete_ac_sensitivity_varies_expression_valued_model_parameters() {
        let netlist = Netlist::parse(
            "Model sensitivity\n\
.param sheet=100\n\
V1 in 0 AC 1\n\
R1 in out RMOD L=10u W=1u\n\
R2 out 0 1k\n\
.model RMOD R RSH={sheet}\n\
.end\n",
        )
        .expect("deck parses");
        let result = Engine::default()
            .run_sensitivity_ac_complete(
                &netlist,
                AcSensitivityOutput::Voltage {
                    positive: 2,
                    negative: None,
                },
                &[1.0e3],
                &["RMOD:*".to_string()],
            )
            .expect("model sensitivity runs");
        let rsh = result.get("RMOD:RSH").expect("RSH model sensitivity");
        assert_eq!(rsh.nominal_value, 100.0);
        assert!((rsh.absolute[0].re + 2.5e-3).abs() < 1e-8);
    }

    #[test]
    fn sens_parser_retains_current_output_filters_and_ac_sweep() {
        let netlist = Netlist::parse(
            "Sensitivity syntax\nV1 1 0 AC 1\nR1 1 0 1k\n.sens I(V1) R* RMOD:* AC DEC 10 1 1k\n.end\n",
        )
        .expect("deck parses");
        let AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            filters,
            ac_sweep,
        } = &netlist.analyses[0]
        else {
            panic!("expected sensitivity command");
        };
        assert_eq!(output_node, "V1");
        assert!(reference_node.is_none());
        assert!(*output_is_current);
        assert_eq!(filters, &["R*", "RMOD:*"]);
        let sweep = ac_sweep.expect("AC sweep");
        assert_eq!(sweep.points, 10);
        assert_eq!(sweep.start_freq, 1.0);
        assert_eq!(sweep.stop_freq, 1.0e3);
    }

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
    fn sensitivity_rejects_invalid_output_node_without_panicking() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity(&netlist, 999, "rval", 1000.0, None)
            .expect_err("out-of-range sensitivity output node must raise");
        let msg = err.to_string();
        assert!(
            msg.contains("Sensitivity output node") && msg.contains("999"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn linearized_sensitivity_rejects_invalid_output_node_without_panicking() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity_linearized(&netlist, 999, None)
            .expect_err("out-of-range linearized sensitivity output node must raise");
        let msg = err.to_string();
        assert!(
            msg.contains("Sensitivity output node") && msg.contains("999"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn ac_sensitivity_rejects_invalid_output_node_without_silent_zero() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_sensitivity_ac(&netlist, 999, "rval", 1000.0, &[1e3], None)
            .expect_err("out-of-range AC sensitivity output node must raise");
        let msg = err.to_string();
        assert!(
            msg.contains("Sensitivity output node") && msg.contains("999"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn sensitivity_rejects_invalid_numeric_inputs() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let engine = Engine::default();

        let err = engine
            .run_sensitivity(&netlist, 2, "rval", f64::NAN, None)
            .expect_err("non-finite sensitivity parameter values must be rejected");
        assert!(
            err.to_string().contains("param_value must be finite"),
            "unexpected error: {err}"
        );

        let err = engine
            .run_sensitivity(&netlist, 2, "rval", 1000.0, Some(0.0))
            .expect_err("zero sensitivity delta must be rejected");
        assert!(
            err.to_string()
                .contains("delta must be a positive finite number"),
            "unexpected error: {err}"
        );

        let err = engine
            .run_sensitivity_ac(&netlist, 2, "rval", f64::INFINITY, &[1e3], None)
            .expect_err("non-finite AC sensitivity parameter values must be rejected");
        assert!(
            err.to_string().contains("param_value must be finite"),
            "unexpected error: {err}"
        );

        let err = engine
            .run_sensitivity_ac(&netlist, 2, "rval", 1000.0, &[1e3], Some(-1.0))
            .expect_err("negative AC sensitivity delta must be rejected");
        assert!(
            err.to_string()
                .contains("delta must be a positive finite number"),
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
    fn step_rejects_empty_value_list() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_step(&netlist, "rval", &[])
            .expect_err("empty step sweep must not report success");
        assert!(
            err.to_string().contains("no sweep values"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn step_rejects_non_finite_value_list() {
        let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("deck parses");
        let err = Engine::default()
            .run_step(&netlist, "rval", &[1000.0, f64::NAN])
            .expect_err("non-finite step sweep values must not enter the solver");
        assert!(
            err.to_string().contains("finite"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn model_step_rejects_empty_value_list() {
        let netlist = Netlist::parse(MODEL_STEP_DECK).expect("deck parses");
        let command = StepCommand {
            target: StepTarget::Model,
            name: "RMOD".to_string(),
            param_name: Some("RSH".to_string()),
            sweep: StepSweep::List(Vec::new()),
        };
        let err = Engine::default()
            .run_step_command(&netlist, &command, &[])
            .expect_err("empty model step sweep must not report success");
        assert!(
            err.to_string().contains("no sweep values"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn colon_qualified_device_step_target_resolves_to_model_when_no_device_matches() {
        let netlist = Netlist::parse(MODEL_STEP_DECK).expect("deck parses");
        let command = StepCommand {
            target: StepTarget::Device,
            name: "RMOD".to_string(),
            param_name: Some("RSH".to_string()),
            sweep: StepSweep::List(vec![100.0, 200.0]),
        };

        let results = Engine::default()
            .run_step_command(&netlist, &command, &[100.0, 200.0])
            .expect("device-style model:param step target should resolve to the model");

        assert_eq!(results.len(), 2);
        assert!((results[0].1.voltage(2) - 5.0).abs() < 1e-9);
        assert!((results[1].1.voltage(2) - (10.0 / 3.0)).abs() < 1e-9);
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
