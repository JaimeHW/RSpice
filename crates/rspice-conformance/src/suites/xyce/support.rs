//! Shared helpers that do not belong to a single stage.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    /// Create a runner rooted at `tests/xyce`.
    pub fn new<P: AsRef<Path>>(root: P, config: XyceRunnerConfig) -> Self {
        let root = root
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| root.as_ref().to_path_buf());
        let upstream_wrapper_decks = Self::load_upstream_wrapper_decks(&root);
        Self {
            root,
            config,
            upstream_wrapper_decks,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn config(&self) -> &XyceRunnerConfig {
        &self.config
    }

    pub(super) fn blank_wrapper_directive_matches(line: &str, directives: &[&str]) -> bool {
        let stripped = Self::strip_netlist_comment(line);
        let mut fields = stripped.split_whitespace();
        let Some(command) = fields.next() else {
            return false;
        };
        directives
            .iter()
            .any(|directive| command.eq_ignore_ascii_case(directive))
            && fields.next().is_none()
    }

    pub(super) fn quote_spice_path(path: &Path) -> Result<String, String> {
        let path = path.to_string_lossy();
        if path.contains('"') {
            return Err(format!(
                "wrapper-origin include/library path contains an unsupported quote character: {path}"
            ));
        }
        Ok(format!("\"{path}\""))
    }

    pub(super) fn xyce_paramfile_parameter_name_is_supported(name: &str) -> bool {
        let mut chars = name.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        (first.is_ascii_alphabetic() || first == '_')
            && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    }

    /// RSpice never executes upstream `.sh`/`.pl` harness files in this runner.
    pub fn executes_upstream_scripts(&self) -> bool {
        false
    }

    /// Whether a retained deck had an upstream `.cir.sh` wrapper sidecar in the
    /// source corpus. Those scripts are intentionally not vendored; the
    /// cross-platform manifest records the execution contract instead.
    pub fn requires_upstream_wrapper(&self, relative_path: &str) -> bool {
        self.upstream_wrapper_decks
            .contains(&Self::normalize_manifest_key(relative_path))
    }

    pub fn statistics(results: &[XyceTestResult]) -> XyceStatistics {
        let mut stats = XyceStatistics::default();
        stats.total = results.len();
        for result in results {
            stats.total_time_ms += result.duration_ms;
            if result.expected_unsupported {
                stats.expected_unsupported += 1;
            } else {
                stats.executed += 1;
                if result.passed {
                    stats.passed += 1;
                }
            }
            if !result.passed {
                stats.failed += 1;
            }
        }
        stats
    }

    pub(super) fn failure_result(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        error: String,
        mismatches: Vec<XyceValueMismatch>,
    ) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: false,
            expected_unsupported: false,
            error: Some(error),
            mismatches,
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    pub(super) fn passed_result(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
    ) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: true,
            expected_unsupported: false,
            error: None,
            mismatches: Vec::new(),
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    pub(super) fn measure_cont_step_mt_stream(
        rows: &[XyceMixedMeasurementReferenceRow],
    ) -> Result<Vec<String>, String> {
        fn field(value: Option<XyceMeasurementReferenceValue>) -> Result<String, String> {
            match value {
                None => Ok("NONE".to_string()),
                Some(XyceMeasurementReferenceValue::Failed) => Ok("FAILED".to_string()),
                Some(XyceMeasurementReferenceValue::Numeric { value, .. }) => {
                    XyceTestRunner::xyce_prn_scientific_text(value, 6)
                }
            }
        }
        rows.iter()
            .map(|row| {
                Ok(format!(
                    "{}\t{}\t{}\t{}",
                    row.name.to_ascii_uppercase(),
                    field(Some(row.value))?,
                    field(row.trigger_axis)?,
                    field(row.target_axis)?
                ))
            })
            .collect()
    }

    pub(super) fn check_measure_cont_tran_deadline(
        &self,
        start: Instant,
        phase: &str,
    ) -> Result<(), String> {
        if DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1)).is_aborted() {
            Err(format!(
                "MEASURE_CONT shared deadline expired during {phase} ({}ms)",
                self.config.max_time_per_test_ms
            ))
        } else {
            Ok(())
        }
    }

    pub(super) fn measure_cont_serialized_remeasure_result(
        result: &TransientResult,
        serialized_table: Option<&XycePrnTable>,
        scientific_precision: usize,
    ) -> Result<TransientResult, String> {
        Self::validate_transient_result_time_grid(result)?;
        let (source_time, source_voltages) = if let Some(table) = serialized_table {
            if table.columns.len() < 2
                || !table.columns[0].eq_ignore_ascii_case("INDEX")
                || !table.columns[1].eq_ignore_ascii_case("TIME")
                || table
                    .rows
                    .iter()
                    .any(|row| row.len() != table.columns.len())
            {
                return Err("MEASURE_CONT serialized PRN table has an invalid shape".into());
            }
            let time = table.rows.iter().map(|row| row[1]).collect::<Vec<_>>();
            let mut voltages = Vec::with_capacity(result.node_names.len());
            for name in &result.node_names {
                let probe = format!("V({name})");
                let column = table
                    .columns
                    .iter()
                    .position(|candidate| candidate.eq_ignore_ascii_case(&probe))
                    .ok_or_else(|| {
                        format!("MEASURE_CONT serialized PRN table omitted physical probe {probe}")
                    })?;
                voltages.push(table.rows.iter().map(|row| row[column]).collect::<Vec<_>>());
            }
            (time, voltages)
        } else {
            (result.time.clone(), result.voltages.clone())
        };
        let mut indices = Vec::with_capacity(source_time.len());
        let mut time = Vec::with_capacity(source_time.len());
        for (index, value) in source_time.iter().copied().enumerate() {
            let printed = Self::xyce_prn_scientific_roundtrip(value, scientific_precision)?;
            if time.last().is_some_and(|previous| *previous > printed) {
                return Err(format!(
                    "MEASURE_CONT serialized PRN time regressed at accepted point {index}"
                ));
            }
            if time.last() == Some(&printed) {
                continue;
            }
            indices.push(index);
            time.push(printed);
        }
        let serialize_waveforms = |role: &str,
                                   waveforms: &[Vec<Value>],
                                   source_len: usize|
         -> Result<Vec<Vec<Value>>, String> {
            waveforms
                .iter()
                .enumerate()
                .map(|(waveform_index, waveform)| {
                    if waveform.len() != source_len {
                        return Err(format!(
                            "MEASURE_CONT {role} waveform {waveform_index} has {} samples for {source_len} serialized times",
                            waveform.len(),
                        ));
                    }
                    indices
                        .iter()
                        .map(|index| {
                            Self::xyce_prn_scientific_roundtrip(
                                waveform[*index],
                                scientific_precision,
                            )
                        })
                        .collect()
                })
                .collect()
        };
        let (branch_currents, branch_names) = if serialized_table.is_some() {
            (Vec::new(), Vec::new())
        } else {
            (
                serialize_waveforms("branch-current", &result.branch_currents, result.time.len())?,
                result.branch_names.clone(),
            )
        };
        Ok(TransientResult {
            time,
            step_sizes: vec![0.0; result.time.len()],
            voltages: serialize_waveforms("voltage", &source_voltages, source_time.len())?,
            branch_currents,
            num_nodes: result.num_nodes,
            node_names: result.node_names.clone(),
            branch_names,
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
        })
    }

    pub(super) fn check_abm_transient_deadline(
        &self,
        start: Instant,
        phase: &str,
    ) -> Result<(), String> {
        if DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1)).is_aborted() {
            Err(format!(
                "ABM transient shared deadline expired during {phase} ({}ms)",
                self.config.max_time_per_test_ms
            ))
        } else {
            Ok(())
        }
    }

    /// Return the canonical byte representation used for textual provenance.
    ///
    /// Git and the Release 7.10 artifacts use LF. A Windows checkout may
    /// materialize the same text with CRLF, so provenance hashes normalize
    /// CRLF to LF before applying byte counts and digests. Bare CR is rejected
    /// instead of being silently reinterpreted, and every non-line-ending byte
    /// remains covered by the canonical identity.
    pub(super) fn canonical_lf_text_identity(label: &str, bytes: &[u8]) -> Result<Vec<u8>, String> {
        std::str::from_utf8(bytes).map_err(|error| format!("{label} is not UTF-8: {error}"))?;
        let mut canonical = Vec::with_capacity(bytes.len());
        let mut index = 0usize;
        while index < bytes.len() {
            match bytes[index] {
                b'\r' => {
                    if bytes.get(index + 1) != Some(&b'\n') {
                        return Err(format!(
                            "{label} contains a bare carriage return at byte {index}"
                        ));
                    }
                    canonical.push(b'\n');
                    index += 2;
                }
                byte => {
                    canonical.push(byte);
                    index += 1;
                }
            }
        }
        Ok(canonical)
    }

    pub(super) fn abm_transient_pwl_value(
        kind: XyceAbmTransientKind,
        time: Value,
    ) -> Result<Value, String> {
        let points = Self::abm_transient_pwl_points(kind);
        if !time.is_finite() || time < points[0].0 || time > points[points.len() - 1].0 {
            return Err(format!(
                "ABM transient PWL time {time} is outside the exact domain"
            ));
        }
        for pair in points.windows(2) {
            let (left_time, left_value) = pair[0];
            let (right_time, right_value) = pair[1];
            if time <= right_time {
                let fraction = (time - left_time) / (right_time - left_time);
                return Ok(left_value + fraction * (right_value - left_value));
            }
        }
        Ok(points[points.len() - 1].1)
    }

    pub(super) fn abm_transient_pwl_max_slope_near(
        kind: XyceAbmTransientKind,
        time: Value,
        radius: Value,
    ) -> Result<Value, String> {
        if !time.is_finite() || !radius.is_finite() || radius < 0.0 {
            return Err("ABM transient PWL slope window is nonfinite or negative".into());
        }
        let points = Self::abm_transient_pwl_points(kind);
        let window_start = time - radius;
        let window_end = time + radius;
        let mut maximum: Value = 0.0;
        let mut intersects = false;
        for pair in points.windows(2) {
            let (left_time, left_value) = pair[0];
            let (right_time, right_value) = pair[1];
            if right_time < window_start || left_time > window_end {
                continue;
            }
            intersects = true;
            maximum = maximum.max(((right_value - left_value) / (right_time - left_time)).abs());
        }
        if intersects {
            Ok(maximum)
        } else {
            Err(format!(
                "ABM transient PWL slope window around {time} is outside the exact domain"
            ))
        }
    }

    pub(super) fn abm_transient_pwl_points(
        kind: XyceAbmTransientKind,
    ) -> &'static [(Value, Value)] {
        match kind {
            XyceAbmTransientKind::DirectTime | XyceAbmTransientKind::ParameterTime => &[
                (0.0, 0.0),
                (1.0, 5.0),
                (2.0, 10.0),
                (3.0, 10.0),
                (4.0, 5.0),
                (5.0, 0.0),
                (6.0, 0.0),
            ],
            XyceAbmTransientKind::SquareRoot => &[
                (0.0, 0.0),
                (1.0, 1.0),
                (2.0, 4.0),
                (3.0, 9.0),
                (4.0, 16.0),
                (5.0, 25.0),
                (6.0, 36.0),
                (7.0, 49.0),
                (8.0, 1.0e6),
                (9.0, 998001.0),
                (10.0, 1.0e8),
                (11.0, 1.0e10),
                (12.0, 390625.0),
            ],
        }
    }

    pub(super) fn abm_transient_dynamic_gold_table(
        actual: &XycePrnTable,
        kind: XyceAbmTransientKind,
    ) -> Result<XycePrnTable, String> {
        Self::abm_transient_derived_table(actual, kind, false)
    }

    pub(super) fn abm_transient_counterfactual_table(
        actual: &XycePrnTable,
        kind: XyceAbmTransientKind,
    ) -> Result<XycePrnTable, String> {
        Self::abm_transient_derived_table(actual, kind, true)
    }

    pub(super) fn abm_transient_derived_table(
        actual: &XycePrnTable,
        kind: XyceAbmTransientKind,
        counterfactual: bool,
    ) -> Result<XycePrnTable, String> {
        if actual.columns != kind.expected_columns() || actual.rows.len() < 3 {
            return Err(
                "ABM transient derived gold received a table outside its exact layout".into(),
            );
        }
        let mut rows = Vec::with_capacity(actual.rows.len());
        for (row_index, row) in actual.rows.iter().enumerate() {
            if row.len() != actual.columns.len() {
                return Err(format!(
                    "ABM transient derived-gold row {row_index} has wrong width"
                ));
            }
            // The historical Perl programs split the DUT's already serialized
            // default PRN and printf all fields with %.8e before xyce_verify.
            let time = Self::xyce_default_prn_roundtrip(row[1])?;
            let input = Self::xyce_default_prn_roundtrip(row[2])?;
            let expected = match (kind, counterfactual) {
                (XyceAbmTransientKind::DirectTime, false)
                | (XyceAbmTransientKind::ParameterTime, false) => vec![time * input],
                (XyceAbmTransientKind::DirectTime, true)
                | (XyceAbmTransientKind::ParameterTime, true) => vec![time + input],
                (XyceAbmTransientKind::SquareRoot, false) => vec![input.sqrt(), input],
                (XyceAbmTransientKind::SquareRoot, true) => vec![input, input.sqrt()],
            };
            let mut derived = vec![row[0], time, input];
            for value in expected {
                derived.push(Self::xyce_default_prn_roundtrip(value)?);
            }
            if derived.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "ABM transient derived gold produced nonfinite row {row_index}"
                ));
            }
            rows.push(derived);
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    pub(super) fn check_abm_pow_deadline(&self, start: Instant, phase: &str) -> Result<(), String> {
        if DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1)).is_aborted() {
            Err(format!(
                "ABM_POW shared deadline expired during {phase} ({}ms)",
                self.config.max_time_per_test_ms
            ))
        } else {
            Ok(())
        }
    }

    pub(super) fn abm_pow_dynamic_gold_table(
        actual: &XycePrnTable,
        kind: XyceAbmPowKind,
    ) -> Result<XycePrnTable, String> {
        Self::abm_pow_derived_table(actual, kind, false)
    }

    pub(super) fn abm_pow_counterfactual_table(
        actual: &XycePrnTable,
        kind: XyceAbmPowKind,
    ) -> Result<XycePrnTable, String> {
        Self::abm_pow_derived_table(actual, kind, true)
    }

    pub(super) fn abm_pow_derived_table(
        actual: &XycePrnTable,
        kind: XyceAbmPowKind,
        counterfactual: bool,
    ) -> Result<XycePrnTable, String> {
        if actual.columns != kind.expected_columns() || actual.rows.len() != kind.expected_rows() {
            return Err("ABM_POW derived gold received a table outside its exact layout".into());
        }
        let mut rows = Vec::with_capacity(actual.rows.len());
        for (row_index, row) in actual.rows.iter().enumerate() {
            if row.len() != actual.columns.len() {
                return Err(format!(
                    "ABM_POW derived-gold row {row_index} has wrong width"
                ));
            }
            // The Perl programs split the already serialized default PRN and
            // calculate from its V(1) token, then printf every result as %.8e.
            let input = Self::xyce_default_prn_roundtrip(row[1])?;
            let expected = match (kind, counterfactual) {
                (XyceAbmPowKind::UnaryMinusPrecedence, false) => vec![-(input * input)],
                (XyceAbmPowKind::UnaryMinusPrecedence, true) => vec![input * input],
                (XyceAbmPowKind::NegativeIntegerExponent, false) => {
                    let inverse = 1.0 / input;
                    vec![inverse * inverse, inverse * inverse * inverse]
                }
                (XyceAbmPowKind::NegativeIntegerExponent, true) => {
                    vec![input * input, input * input * input]
                }
                (XyceAbmPowKind::FractionalPrincipalComplex, false) => vec![
                    Complex64::new(input, 0.0).powf(2.1).re,
                    Complex64::new(-input, 0.0).powf(3.1).re,
                ],
                (XyceAbmPowKind::FractionalPrincipalComplex, true) => vec![
                    if input < 0.0 { 0.0 } else { input.powf(2.1) },
                    if -input < 0.0 {
                        0.0
                    } else {
                        (-input).powf(3.1)
                    },
                ],
            };
            let mut derived = vec![row[0], input];
            for value in expected {
                derived.push(Self::xyce_default_prn_roundtrip(value)?);
            }
            if derived.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "ABM_POW derived gold produced nonfinite row {row_index}"
                ));
            }
            rows.push(derived);
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    pub(super) fn check_xdm_replaceground_deadline(
        &self,
        start: Instant,
        phase: &str,
    ) -> Result<(), String> {
        let deadline = DeadlineAbort::new(start, self.config.max_time_per_test_ms);
        if deadline.is_aborted() {
            return Err(format!(
                "XDM REPLACEGROUND shared deadline expired during {phase} ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        Ok(())
    }

    pub(super) fn project_xdm_replaceground_hspice(source: &str) -> Result<String, String> {
        let mut physical_lines = source.lines();
        let title = physical_lines
            .next()
            .ok_or_else(|| "paired HSPICE source is empty".to_string())?;
        if !title.trim_start().starts_with('$') {
            return Err(
                "paired HSPICE source must retain its authored '$' title record".to_string(),
            );
        }
        let mut projected = String::new();
        projected.push_str(title);
        projected.push('\n');
        projected.push_str(".PREPROCESS REPLACEGROUND TRUE\n");
        projected.push_str(".OPTIONS DEVICE TNOM=25\n");
        let mut print_count = 0usize;
        for line in physical_lines {
            let trimmed = line.trim();
            let fields = trimmed.split_whitespace().collect::<Vec<_>>();
            if fields
                .first()
                .is_some_and(|field| field.eq_ignore_ascii_case(".PREPROCESS"))
                || fields
                    .first()
                    .is_some_and(|field| field.eq_ignore_ascii_case(".OPTIONS"))
                || fields
                    .first()
                    .is_some_and(|field| field.eq_ignore_ascii_case(".CONTROL"))
            {
                return Err(format!(
                    "paired HSPICE source contains pre-existing translation control '{trimmed}'"
                ));
            }
            if fields
                .first()
                .is_some_and(|field| field.eq_ignore_ascii_case(".PRINT"))
                && fields
                    .get(1)
                    .is_some_and(|analysis| analysis.eq_ignore_ascii_case("DC"))
            {
                if fields.len() < 3
                    || fields
                        .iter()
                        .skip(2)
                        .any(|field| field.to_ascii_uppercase().starts_with("FORMAT"))
                {
                    return Err(
                        "paired HSPICE projection requires one plain nonempty .PRINT DC card"
                            .to_string(),
                    );
                }
                print_count += 1;
            }
            projected.push_str(line);
            projected.push('\n');
        }
        if print_count != 1 {
            return Err(format!(
                "paired HSPICE projection requires exactly one .PRINT DC card, found {print_count}"
            ));
        }
        Ok(projected)
    }

    pub(super) fn check_addresistors_deadline(
        &self,
        start: Instant,
        phase: &str,
    ) -> Result<(), String> {
        if DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1)).is_aborted() {
            return Err(format!(
                "ADDRESISTORS shared deadline expired during {phase} ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        Ok(())
    }

    pub(super) fn addresistors_transient_plan(
        deck: &XyceDeck,
        source: &str,
        netlist: &Netlist,
    ) -> Result<XyceStaticTranPlan, String> {
        let output = Self::single_tran_print_output_request(source)?;
        if output.format.is_some()
            || output.file.is_some()
            || output.probes != ["V(2)".to_string(), "V(X1:2)".to_string()]
        {
            return Err(format!(
                "ADDRESISTORS transient requires default .PRINT TRAN V(2) V(X1:2), got {output:?}"
            ));
        }
        let tran = Self::single_tran_analysis(netlist)?;
        let steps = Self::step_commands(netlist)?;
        Ok(XyceStaticTranPlan {
            deck_path: deck.path.clone(),
            reference_path: deck.path.clone(),
            source: source.to_string(),
            print: XycePrintRequest {
                probes: output.probes,
            },
            output_override: false,
            timeint_conststep: false,
            tran,
            steps,
            contract: XyceStaticTranContract::WrapperStatic,
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision: XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            },
        })
    }

    pub(super) fn addresistors_dynamic_gold_table(
        actual: &XycePrnTable,
    ) -> Result<XycePrnTable, String> {
        Self::validate_addresistors_transient_schedule(actual)?;
        let mut rows = Vec::with_capacity(actual.rows.len());
        for (row_index, row) in actual.rows.iter().enumerate() {
            let serialized_time = Self::xyce_default_prn_roundtrip(row[1])?;
            let expected = Self::xyce_default_prn_roundtrip((-serialized_time).exp())?;
            rows.push(vec![row[0], serialized_time, expected, expected]);
            if rows[row_index].iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "ADDRESISTORS dynamic gold produced a non-finite row {row_index}"
                ));
            }
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    pub(super) fn check_removeunused_deadline(
        &self,
        start: Instant,
        phase: &str,
    ) -> Result<(), String> {
        if DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1)).is_aborted() {
            return Err(format!(
                "REMOVEUNUSED shared deadline expired during {phase} ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        Ok(())
    }

    pub(super) fn removeunused_dynamic_gold_table(
        actual: &XycePrnTable,
        kind: XyceRemoveUnusedKind,
    ) -> Result<XycePrnTable, String> {
        let ratio = kind.expected_divider_ratio();
        let mut rows = Vec::with_capacity(actual.rows.len());
        for (row_index, row) in actual.rows.iter().enumerate() {
            if row.len() != 3 {
                return Err(format!(
                    "REMOVEUNUSED dynamic gold row {row_index} has width {}",
                    row.len()
                ));
            }
            // The Perl gold script reads the already serialized V(1), applies
            // the analytic ratio, then writes the result with %14.8e.
            let serialized_v1 = Self::xyce_default_prn_roundtrip(row[1])?;
            let expected_v2 = Self::xyce_default_prn_roundtrip(ratio * serialized_v1)?;
            rows.push(vec![row[0], serialized_v1, expected_v2]);
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    pub(super) fn replace_bug702_block_once(
        label: &str,
        source: String,
        old: &str,
        new: &str,
    ) -> Result<String, String> {
        let count = source.matches(old).count();
        if count != 1 {
            return Err(format!(
                "{label} canonical representation block count changed: expected 1, got {count}"
            ));
        }
        Ok(source.replacen(old, new, 1))
    }

    pub(super) fn require_exact_syntax_failure(
        label: &str,
        source: &str,
        deck_path: &Path,
        expected_line: usize,
        expected_message: &str,
    ) -> Result<(), String> {
        match Self::parse_xyce_netlist(source, deck_path) {
            Err(ParseError::Syntax { line, message })
                if line == expected_line && message == expected_message =>
            {
                Ok(())
            }
            Err(error) => Err(format!(
                "{label} produced the wrong typed parse failure: expected line {expected_line} / {expected_message:?}, got {error:?}"
            )),
            Ok(_) => Err(format!(
                "{label} unexpectedly parsed; the canonical malformed construct is absent"
            )),
        }
    }

    pub(super) fn require_missing_library_dependency_absent(
        label: &str,
        deck_path: &Path,
        execution_dir: &Path,
    ) -> Result<(), String> {
        let owner_dir = deck_path
            .parent()
            .ok_or_else(|| format!("{label} deck has no parent directory"))?;
        let mut search_directories = BTreeSet::new();
        for directory in [
            owner_dir.to_path_buf(),
            execution_dir.to_path_buf(),
            owner_dir.join("lib"),
            owner_dir.join("models"),
            owner_dir.join("..").join("lib"),
            owner_dir.join("..").join("models"),
        ] {
            search_directories.insert(directory);
        }
        let mut present = Vec::new();
        for directory in search_directories {
            if !directory.is_dir() {
                continue;
            }
            for entry in fs::read_dir(&directory).map_err(|error| {
                format!(
                    "failed to inspect {label} missing-library search directory {}: {error}",
                    directory.display()
                )
            })? {
                let entry = entry.map_err(|error| {
                    format!(
                        "failed to inspect {label} missing-library search entry in {}: {error}",
                        directory.display()
                    )
                })?;
                if entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.eq_ignore_ascii_case("plugh.lib"))
                {
                    present.push(entry.path());
                }
            }
        }
        present.sort();
        if !present.is_empty() {
            return Err(format!(
                "{label} missing-library oracle acquired a resolvable plugh.lib dependency: {present:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn require_bug702_missing_ic_dat(family_dir: &Path) -> Result<(), String> {
        for entry in fs::read_dir(family_dir).map_err(|error| {
            format!(
                "failed to inspect BUG702 execution directory {}: {error}",
                family_dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to inspect BUG702 execution-directory entry in {}: {error}",
                    family_dir.display()
                )
            })?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.eq_ignore_ascii_case("ic.dat"))
            {
                return Err(format!(
                    "BUG702 missing-initcond oracle acquired execution-directory dependency {}",
                    entry.path().display()
                ));
            }
        }
        Ok(())
    }

    pub(super) fn require_exact_duplicate_device_failure(
        label: &str,
        source: &str,
        deck_path: &Path,
        expected_name: &str,
        expected_scope: &str,
        expected_first_line: usize,
        expected_duplicate_line: usize,
    ) -> Result<(), String> {
        match Self::parse_xyce_netlist(source, deck_path) {
            Err(ParseError::DuplicateName {
                canonical_name,
                first_name,
                duplicate_name,
                scope,
                first_line,
                duplicate_line,
            }) if canonical_name == expected_name
                && first_name == expected_name
                && duplicate_name == expected_name
                && scope == expected_scope
                && first_line == expected_first_line
                && duplicate_line == expected_duplicate_line =>
            {
                Ok(())
            }
            Err(error) => Err(format!(
                "{label} produced the wrong typed duplicate-device failure: {error:?}"
            )),
            Ok(_) => Err(format!(
                "{label} unexpectedly parsed; the duplicate-device condition is absent"
            )),
        }
    }

    pub(super) fn require_bug769_one_line_delta_family(deck_path: &Path) -> Result<(), String> {
        let family = deck_path
            .parent()
            .ok_or_else(|| "BUG 769 record has no family directory".to_string())?;
        let members = [
            ("bug_769a.cir", ".param RVAL={76K+v(3)}"),
            ("bug_769b.cir", ".param RVAL={76K+i(v2)}"),
            ("bug_769c.cir", ".param RVAL={76K+i(c2)}"),
        ];
        let mut sources = Vec::with_capacity(members.len());
        for (file_name, expected_probe_line) in members {
            let path = family.join(file_name);
            let source = fs::read_to_string(&path).map_err(|error| {
                format!("failed to read BUG 769 family member {path:?}: {error}")
            })?;
            let lines = source.lines().map(str::to_string).collect::<Vec<_>>();
            if lines.len() != 82 {
                return Err(format!(
                    "BUG 769 family member {file_name} physical line count changed: expected 82, got {}",
                    lines.len()
                ));
            }
            if lines[68] != expected_probe_line {
                return Err(format!(
                    "BUG 769 family member {file_name} physical line 69 changed: expected {expected_probe_line:?}, got {:?}",
                    lines[68]
                ));
            }
            sources.push((file_name, lines));
        }
        for line_index in 0..82 {
            if line_index == 68 {
                continue;
            }
            let reference = &sources[0].1[line_index];
            for (file_name, lines) in &sources[1..] {
                if &lines[line_index] != reference {
                    return Err(format!(
                        "BUG 769 family must differ only at physical line 69; {file_name} differs at line {}",
                        line_index + 1
                    ));
                }
            }
        }
        Ok(())
    }

    pub(super) fn execution_plan(&self, deck: &XyceDeck) -> Result<XyceExecutionPlan, String> {
        let requires_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        let mut source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;
        source =
            Self::source_with_static_dc_wrapper_bindings(&source, &deck.path, requires_wrapper)?;

        let measurement_reference_paths = self.measurement_reference_paths(&deck.path, "ms")?;

        let wrapper_contract = if requires_wrapper {
            if measurement_reference_paths.is_empty() {
                Some(Self::native_static_prn_wrapper_contract(
                    &deck.relative_path,
                    &deck.path,
                    &source,
                )?)
            } else {
                Self::validate_scalar_dc_measurement_wrapper_source(&source)?;
                Some(XyceStaticDcContract::WrapperDefault)
            }
        } else {
            None
        };
        let (execution_deck_path, execution_dir) = if matches!(
            wrapper_contract,
            Some(XyceStaticDcContract::WrapperTopLevelExecutionDir)
        ) {
            (
                Self::top_level_execution_deck_path(&deck.path)?,
                Some(
                    deck.path
                        .parent()
                        .ok_or_else(|| "wrapper deck has no parent directory".to_string())?
                        .to_path_buf(),
                ),
            )
        } else {
            (deck.path.clone(), None)
        };

        let expression_dialect = if matches!(
            wrapper_contract,
            Some(XyceStaticDcContract::WrapperHspiceMath)
        ) {
            ExpressionDialect::Ngspice
        } else {
            ExpressionDialect::Xyce
        };
        let static_plan = if matches!(
            wrapper_contract,
            Some(XyceStaticDcContract::WrapperTopLevelExecutionDir)
        ) {
            self.static_dc_plan_for_path_with_execution_dir(
                &execution_deck_path,
                expression_dialect,
                execution_dir.as_deref(),
            )?
        } else {
            self.static_dc_plan_for_source_with_execution_dir(
                &execution_deck_path,
                source,
                expression_dialect,
                execution_dir.as_deref(),
            )?
        };
        let parsed_netlist = Self::parse_netlist_with_expression_dialect_and_execution_dir(
            &static_plan.source,
            &static_plan.deck_path,
            expression_dialect,
            static_plan.execution_dir.as_deref(),
        )
        .map_err(|err| format!("netlist parser failed after static DC validation: {err}"))?;
        let continuous_measurement_reference_paths =
            if parsed_netlist.options.measure_use_cont_files() {
                self.continuous_measurement_reference_paths(
                    &deck.path,
                    &parsed_netlist,
                    "DC_CONT",
                    "ms",
                )?
            } else {
                Vec::new()
            };
        let contract = if let Some(contract) = wrapper_contract {
            self.validate_native_static_prn_wrapper_contract(contract, &static_plan)?;
            contract
        } else {
            Self::static_dc_contract_for_print_format(false, static_plan.print_format.as_deref())?
        };
        if matches!(contract, XyceStaticDcContract::WrapperFilePrn) && !static_plan.steps.is_empty()
        {
            return Err(
                "wrapper-origin file-output contract does not cover .STEP DC decks yet".to_string(),
            );
        }
        let reference_path = self
            .static_output_reference_path(&deck.path, contract.reference_extension())
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        if !reference_path.is_file()
            && measurement_reference_paths.is_empty()
            && continuous_measurement_reference_paths.is_empty()
            && !matches!(
                contract,
                XyceStaticDcContract::WrapperFilePrn | XyceStaticDcContract::WrapperNoOutput
            )
        {
            return Err(format!(
                "no checked-in static .{} oracle at {}",
                contract.reference_extension(),
                self.display_path(&reference_path)
            ));
        }

        Ok(XyceExecutionPlan {
            deck_path: static_plan.deck_path,
            execution_dir: static_plan.execution_dir,
            reference_path,
            measurement_reference_paths,
            continuous_measurement_reference_paths,
            measurement_tolerance: XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
            source: static_plan.source,
            expression_dialect,
            print: static_plan.print,
            dc: static_plan.dc,
            dc_data: static_plan.dc_data,
            steps: static_plan.steps,
            contract,
        })
    }

    pub(super) fn static_tran_family_plan_for_path(
        &self,
        deck_path: &Path,
        purpose: XyceStaticTranPlanPurpose,
    ) -> Result<XyceStaticTranPlan, String> {
        self.static_tran_plan_for_path_with_purpose(deck_path, purpose)
    }

    pub(super) fn static_tran_plan_for_path_with_purpose(
        &self,
        deck_path: &Path,
        purpose: XyceStaticTranPlanPurpose,
    ) -> Result<XyceStaticTranPlan, String> {
        let relative_path = self.relative_key(deck_path);
        let deck = XyceDeck {
            path: deck_path.to_path_buf(),
            section: Self::section_for_relative_path(&relative_path),
            relative_path,
        };
        self.static_tran_plan_for_deck_with_purpose(&deck, purpose)
    }

    pub(super) fn select_static_tran_comparison_mode(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        purpose: XyceStaticTranPlanPurpose,
        requires_wrapper: bool,
    ) -> Result<XyceStaticTranComparisonMode, String> {
        let pointwise = XyceStaticTranComparisonMode::Pointwise;
        if purpose == XyceStaticTranPlanPurpose::DefaultLevel9XyceVerifyOracle {
            return Err(
                "the default LEVEL=9 xyce_verify purpose is derived internally only after the strict integrated-RMS selector succeeds"
                    .to_string(),
            );
        }
        if purpose == XyceStaticTranPlanPurpose::AbsoluteOracle
            && !requires_wrapper
            && plan.contract == XyceStaticTranContract::PlainStatic
            && !plan.output_override
            && !plan.timeint_conststep
            && !plan.steps.is_empty()
            && plan.wrapper_tolerance.is_none()
            && plan.reference_path.is_file()
            && Self::native_transient_uses_standard_startup(netlist)
            && netlist.diagnostics.is_empty()
        {
            let scientific_precision =
                Self::xyce_verify_step_tran_scientific_precision(&plan.source)?;
            if Self::source_has_comp_directive(&plan.source) {
                match Self::xyce_verify_comp_tolerances(&plan.source, &plan.print.probes) {
                    Ok(tolerances) => {
                        return Ok(XyceStaticTranComparisonMode::Release710IntegratedRmsComp {
                            scientific_precision,
                            error_bounds: if tolerances
                                .into_iter()
                                .any(XyceVerifyTransientTolerance::has_nondefault_error_bounds)
                            {
                                XyceVerifyCompErrorBounds::DeckOverrides
                            } else {
                                XyceVerifyCompErrorBounds::Release710Default
                            },
                        });
                    }
                    Err(error) if error == XYCE_VERIFY_COMP_NO_PRINTED_PROBE => {
                        // Release 7.10 stores valid *COMP entries for TIME,
                        // Index, and other unprinted probes without applying
                        // them to a dependent column. The transient waveform
                        // therefore retains the default integrated-RMS bounds.
                        return Ok(XyceStaticTranComparisonMode::Release710IntegratedRms {
                            scientific_precision,
                        });
                    }
                    Err(_) => {}
                }
            } else {
                return Ok(XyceStaticTranComparisonMode::Release710IntegratedRms {
                    scientific_precision,
                });
            }
        }
        if purpose == XyceStaticTranPlanPurpose::AbsoluteOracle
            && !requires_wrapper
            && plan.contract == XyceStaticTranContract::PlainStatic
            && !plan.output_override
            && !plan.timeint_conststep
            && plan.steps.is_empty()
            && plan.wrapper_tolerance.is_none()
            && plan.reference_path.is_file()
            && Self::source_has_comp_directive(&plan.source)
            && Self::native_transient_uses_standard_startup(netlist)
            && netlist.diagnostics.is_empty()
            && Self::validate_native_transient_contract_for_purpose(
                netlist,
                XyceStaticTranPlanPurpose::AbsoluteOracle,
            )
            .is_ok()
        {
            if let Ok(tolerances) =
                Self::xyce_verify_comp_tolerances(&plan.source, &plan.print.probes)
            {
                return Ok(XyceStaticTranComparisonMode::Release710IntegratedRmsComp {
                    scientific_precision: XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                    error_bounds: if tolerances
                        .into_iter()
                        .any(XyceVerifyTransientTolerance::has_nondefault_error_bounds)
                    {
                        XyceVerifyCompErrorBounds::DeckOverrides
                    } else {
                        XyceVerifyCompErrorBounds::Release710Default
                    },
                });
            }
        }
        // EKV26's canonical transient implementation is adaptive while the
        // Xyce PRN oracle records accepted breakpoints.  A reference-grid
        // pointwise comparison would therefore compare different row counts;
        // use Xyce's integrated-RMS verifier for the strictly validated
        // complementary pair envelope instead.
        if purpose == XyceStaticTranPlanPurpose::AbsoluteOracle
            && !requires_wrapper
            && plan.contract == XyceStaticTranContract::PlainStatic
            && !plan.output_override
            && !plan.timeint_conststep
            && plan.steps.is_empty()
            && plan.wrapper_tolerance.is_none()
            && plan.reference_path.is_file()
            && !Self::source_has_comp_directive(&plan.source)
            && Self::native_transient_uses_standard_startup(netlist)
            && netlist.diagnostics.is_empty()
            && Self::netlist_is_native_transient_ekv26_pair(netlist)
            && Self::validate_native_transient_contract_for_purpose(
                netlist,
                XyceStaticTranPlanPurpose::AbsoluteOracle,
            )
            .is_ok()
        {
            return Ok(XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision: XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            });
        }
        // Xyce's LEVEL=18 VDMOS implementation records accepted breakpoints
        // on an adaptive grid, while the native UCCM implementation may
        // choose different breakpoints for the same circuit.  Admit only the
        // strict, fully numeric IRF130-shaped Level=18 envelope to the
        // Release 7.10 integrated-RMS verifier; all other VDMOS decks remain
        // fail-closed under the ordinary pointwise/native contract.
        if purpose == XyceStaticTranPlanPurpose::AbsoluteOracle
            && !requires_wrapper
            && plan.contract == XyceStaticTranContract::PlainStatic
            && !plan.output_override
            && !plan.timeint_conststep
            && plan.steps.is_empty()
            && plan.wrapper_tolerance.is_none()
            && plan.reference_path.is_file()
            && !Self::source_has_comp_directive(&plan.source)
            && Self::native_transient_uses_standard_startup(netlist)
            && netlist.diagnostics.is_empty()
            && Self::netlist_is_native_absolute_transient_vdmos_level18(netlist)
            && Self::validate_native_transient_contract_for_purpose(
                netlist,
                XyceStaticTranPlanPurpose::AbsoluteOracle,
            )
            .is_ok()
        {
            let scientific_precision =
                Self::xyce_verify_step_tran_scientific_precision(&plan.source)?;
            return Ok(XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision,
            });
        }
        if purpose != XyceStaticTranPlanPurpose::AbsoluteOracle
            || requires_wrapper
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || !plan.reference_path.is_file()
            || (Self::source_has_comp_directive(&plan.source)
                && !Self::netlist_is_native_level9_xyce_verify_envelope(netlist))
            || !Self::native_transient_uses_standard_startup(netlist)
            || !netlist.diagnostics.is_empty()
            || !netlist.subcircuits.is_empty()
            || netlist
                .elements
                .iter()
                .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }))
        {
            return Ok(pointwise);
        }

        if Self::netlist_is_native_exact_is_diode_xyce_verify_envelope(netlist) {
            return Ok(XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision: XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            });
        }

        if !Self::netlist_is_native_level9_xyce_verify_envelope(netlist) {
            return Ok(pointwise);
        }

        let scientific_precision = match Self::strict_level9_xyce_verify_source_precision(
            &plan.source,
            netlist.models.len(),
        ) {
            Ok(Some(precision)) => precision,
            Ok(None) => return Ok(pointwise),
            Err(_error)
                if Self::level9_xyce_verify_default_output(&plan.source, netlist.models.len()) =>
            {
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION
            }
            Err(error) => return Err(error),
        };
        if Self::source_has_comp_directive(&plan.source) {
            if !Self::logical_comp_directives(&plan.source)
                .iter()
                .filter_map(|line| Self::comp_directive_body(line))
                .filter_map(Self::split_comp_directive)
                .any(|(_, options)| !options.trim().is_empty())
            {
                return Ok(pointwise);
            }
            match Self::xyce_verify_comp_tolerances(&plan.source, &plan.print.probes) {
                Ok(tolerances) => {
                    return Ok(XyceStaticTranComparisonMode::Release710IntegratedRmsComp {
                        scientific_precision,
                        error_bounds: if tolerances
                            .into_iter()
                            .any(XyceVerifyTransientTolerance::has_nondefault_error_bounds)
                        {
                            XyceVerifyCompErrorBounds::DeckOverrides
                        } else {
                            XyceVerifyCompErrorBounds::Release710Default
                        },
                    });
                }
                Err(error) if error == XYCE_VERIFY_COMP_NO_PRINTED_PROBE => {}
                Err(_) => return Ok(pointwise),
            }
        }
        Ok(XyceStaticTranComparisonMode::Release710IntegratedRms {
            scientific_precision,
        })
    }

    /// Resolve the precision used by Xyce's primary indexed STD transient
    /// output for a stepped run. `xyce_verify.pl` compares each sweep using
    /// the values serialized at this precision, so the verifier boundary must
    /// not be evaluated with the in-memory `f64` values.
    pub(super) fn xyce_verify_step_tran_scientific_precision(
        source: &str,
    ) -> Result<usize, String> {
        let mut primary_precision = None;
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            if stripped.is_empty() {
                continue;
            }
            let fields = Self::split_print_fields(stripped)?;
            if fields.len() < 2
                || !fields[0].eq_ignore_ascii_case(".PRINT")
                || !fields[1].eq_ignore_ascii_case("TRAN")
            {
                continue;
            }

            let field_refs = fields.iter().map(String::as_str).collect::<Vec<_>>();
            let mut precision = XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION;
            let mut format = "STD";
            let mut has_file = false;
            let mut has_probe = false;
            let mut index = 2usize;
            while index < fields.len() {
                if let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&field_refs, index)
                {
                    let key = raw_key.trim().to_ascii_lowercase();
                    let value = raw_value.trim().trim_matches(['"', '\'']);
                    match key.as_str() {
                        "file" => has_file = true,
                        "format" => format = value,
                        "precision" => {
                            let parsed = rspice_core::netlist::lexer::parse_spice_value(value).map_err(
                                |err| {
                                    format!(
                                        "stepped xyce_verify .PRINT TRAN PRECISION must be numeric: '{value}': {err}"
                                    )
                                },
                            )?;
                            if !parsed.is_finite()
                                || parsed < f64::from(i32::MIN)
                                || parsed > f64::from(i32::MAX)
                            {
                                return Err(format!(
                                    "stepped xyce_verify .PRINT TRAN PRECISION must be a finite Xyce integer value, got {parsed}"
                                ));
                            }
                            let effective = parsed as i32;
                            if !(1..=XYCE_MAX_IEEE754_PRN_SCIENTIFIC_PRECISION as i32)
                                .contains(&effective)
                            {
                                return Err(format!(
                                    "stepped xyce_verify .PRINT TRAN PRECISION must resolve from 1 through {XYCE_MAX_IEEE754_PRN_SCIENTIFIC_PRECISION}, got {effective}"
                                ));
                            }
                            precision = effective as usize;
                        }
                        _ => {}
                    }
                    index += consumed;
                    continue;
                }

                let normalized = field_refs[index].to_ascii_lowercase();
                if normalized == "noindex" {
                    return Err(
                        "stepped xyce_verify primary .PRINT TRAN must retain its Index column"
                            .to_string(),
                    );
                }
                if !Self::is_print_option_token(&normalized) {
                    has_probe = true;
                }
                index += 1;
            }
            if has_file || !has_probe {
                continue;
            }
            if !format.eq_ignore_ascii_case("STD") {
                return Err(format!(
                    "stepped xyce_verify primary .PRINT TRAN requires indexed FORMAT=STD, got FORMAT={format}"
                ));
            }
            if primary_precision.replace(precision).is_some() {
                return Err(
                    "stepped xyce_verify requires exactly one primary .PRINT TRAN output"
                        .to_string(),
                );
            }
        }

        primary_precision.ok_or_else(|| {
            "stepped xyce_verify requires one primary indexed .PRINT TRAN output".to_string()
        })
    }

    pub(super) fn pointwise_switch_transition_needs_rms_fallback(netlist: &Netlist) -> bool {
        netlist.models.iter().any(|model| {
            if !matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "VSWITCH" | "VSW"
            ) {
                return false;
            }

            let parameter = |names: &[&str]| {
                model.params.iter().find_map(|(name, value)| {
                    names
                        .iter()
                        .any(|candidate| name.eq_ignore_ascii_case(candidate))
                        .then_some(*value)
                })
            };
            let Some((on, off)) = parameter(&["ON", "VON"]).zip(parameter(&["OFF", "VOFF"])) else {
                return false;
            };
            let span = (on - off).abs();
            let scale = on.abs().max(off.abs()).max(1.0);

            // A narrow Xyce switch curve is intentionally very steep. Tiny
            // differences in accepted transition times can move pointwise
            // samples substantially even when the waveforms are equivalent.
            // Keep pointwise comparison as the primary check and allow the
            // canonical Release 7.10 integrated-RMS verifier as a fallback.
            span.is_finite() && span > 0.0 && span <= scale * 0.01
        })
    }

    pub(super) fn logical_comp_directives(source: &str) -> Vec<String> {
        let mut directives = Vec::new();
        let mut current: Option<String> = None;
        for raw in source.lines() {
            let line = raw.split_once(';').map_or(raw, |(head, _)| head).trim_end();
            if line.trim_start().starts_with('+') {
                if let Some(directive) = current.as_mut() {
                    directive.push(' ');
                    directive.push_str(line.trim_start().trim_start_matches('+').trim_start());
                }
                continue;
            }
            if let Some(directive) = current.take() {
                directives.push(directive);
            }
            if Self::comp_directive_body(line).is_some() {
                current = Some(line.trim_start().to_string());
            }
        }
        if let Some(directive) = current {
            directives.push(directive);
        }
        directives
    }

    pub(super) fn comp_directive_body(line: &str) -> Option<&str> {
        let trimmed = line
            .split_once(';')
            .map_or(line, |(head, _)| head)
            .trim_start();
        let prefix = trimmed.get(..5)?;
        let body = trimmed.get(5..)?;
        (prefix.eq_ignore_ascii_case("*COMP")
            && body.chars().next().is_none_or(char::is_whitespace))
        .then(|| body.trim_start())
    }

    pub(super) fn split_xyce_sensitivity_list(value: &str) -> Result<Vec<&str>, String> {
        let mut fields = Vec::new();
        let mut start = 0usize;
        let mut parentheses = 0usize;
        let mut braces = 0usize;
        for (index, character) in value.char_indices() {
            match character {
                '(' => parentheses = parentheses.saturating_add(1),
                ')' => {
                    parentheses = parentheses.checked_sub(1).ok_or_else(|| {
                        format!("Xyce .SENS list has an unmatched ')' in '{value}'")
                    })?;
                }
                '{' => braces = braces.saturating_add(1),
                '}' => {
                    braces = braces.checked_sub(1).ok_or_else(|| {
                        format!("Xyce .SENS list has an unmatched '}}' in '{value}'")
                    })?;
                }
                ',' if parentheses == 0 && braces == 0 => {
                    fields.push(&value[start..index]);
                    start = index + character.len_utf8();
                }
                _ => {}
            }
        }
        if parentheses != 0 || braces != 0 {
            return Err(format!(
                "Xyce .SENS list has unbalanced grouping in '{value}'"
            ));
        }
        fields.push(&value[start..]);
        Ok(fields)
    }

    pub(super) fn relational_ac_plan_for_path(
        &self,
        path: &Path,
    ) -> Result<XyceRelationalAcPlan, String> {
        let source = fs::read_to_string(path)
            .map_err(|err| format!("failed to read relational AC deck: {err}"))?;
        if Self::contains_control_block(&source) {
            return Err(
                "relational AC comparison does not interpret simulator control blocks".to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;
        let output = Self::canonical_print_output_request(&source, "AC", false)?
            .ok_or_else(|| "deck has no .PRINT AC statement with static columns".to_string())?;
        if output.format.is_some() || output.file.is_some() || output.probes.is_empty() {
            return Err(
                "relational AC comparison requires one nonempty primary .PRINT AC using default PRN output"
                    .to_string(),
            );
        }
        let print = XycePrintRequest {
            probes: output.probes,
        };
        let netlist = Self::parse_xyce_netlist(&source, path)
            .map_err(|err| format!("netlist parser rejected relational AC deck: {err}"))?;
        let ac = Self::single_ac_analysis(&netlist)?;
        if ac.data_points().is_some() || !Self::step_commands(&netlist)?.is_empty() {
            return Err("relational AC comparison does not admit .AC DATA or .STEP".to_string());
        }
        Self::validate_static_ac_contract(&netlist, &ac, &print)?;
        Ok(XyceRelationalAcPlan {
            deck_path: path.to_path_buf(),
            source,
            print,
            ac,
        })
    }

    pub(super) fn static_dc_plan_for_path(
        &self,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
    ) -> Result<XyceStaticDcPlan, String> {
        self.static_dc_plan_for_path_with_execution_dir(deck_path, expression_dialect, None)
    }

    pub(super) fn static_dc_plan_for_path_with_redefinition_policy(
        &self,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
        parameter_redefinition_policy: ParameterRedefinitionPolicy,
    ) -> Result<XyceStaticDcPlan, String> {
        let source =
            fs::read_to_string(deck_path).map_err(|err| format!("failed to read deck: {err}"))?;
        self.static_dc_plan_for_source_with_execution_dir_and_redefinition_policy(
            deck_path,
            source,
            expression_dialect,
            parameter_redefinition_policy,
            None,
        )
    }

    pub(super) fn static_dc_plan_for_path_with_execution_dir(
        &self,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
        execution_dir: Option<&Path>,
    ) -> Result<XyceStaticDcPlan, String> {
        let source =
            fs::read_to_string(deck_path).map_err(|err| format!("failed to read deck: {err}"))?;
        self.static_dc_plan_for_source_with_execution_dir(
            deck_path,
            source,
            expression_dialect,
            execution_dir,
        )
    }

    pub(super) fn voltage_probe_targets_subcircuit_node(normalized_probe: &str) -> bool {
        Self::parse_voltage_probe(normalized_probe).is_some_and(|probe| {
            Self::node_name_targets_subcircuit_node(&probe.node_pos)
                || probe
                    .node_neg
                    .as_deref()
                    .is_some_and(Self::node_name_targets_subcircuit_node)
        })
    }

    pub(super) fn node_name_targets_subcircuit_node(node: &str) -> bool {
        node.contains(':') || Self::node_name_uses_period_hierarchy(node)
    }

    pub(super) fn node_name_uses_period_hierarchy(node: &str) -> bool {
        let Some((first_segment, _)) = node.split_once('.') else {
            return false;
        };
        first_segment.len() > 1 && first_segment.starts_with('x')
    }

    pub(super) fn upstream_wrapper_required_reason() -> &'static str {
        "upstream wrapper semantics are required; RSPICE-HARNESS-MANIFEST.tsv records the removed .cir.sh sidecar contract"
    }

    pub(super) fn lin_directive_is_ac_only(line: &str) -> Result<bool, String> {
        let fields = Self::split_grouped_whitespace_fields(line, ".LIN directive")?;
        if fields
            .first()
            .is_none_or(|command| !command.eq_ignore_ascii_case(".lin"))
        {
            return Ok(false);
        }
        let token_refs = fields.iter().map(String::as_str).collect::<Vec<_>>();
        let mut index = 1usize;
        let mut sparcalc = None;
        while index < token_refs.len() {
            let Some((key, value, consumed)) = Self::print_option_assignment(&token_refs, index)
            else {
                return Ok(false);
            };
            if !key.eq_ignore_ascii_case("SPARCALC") || sparcalc.is_some() {
                return Ok(false);
            }
            let Ok(parsed) = rspice_core::netlist::lexer::parse_spice_value(value) else {
                return Ok(false);
            };
            if !parsed.is_finite() || parsed.fract() != 0.0 || !(0.0..=1.0).contains(&parsed) {
                return Ok(false);
            }
            sparcalc = Some(parsed as i32);
            index += consumed;
        }
        Ok(sparcalc == Some(0))
    }

    #[cfg(test)]
    pub(super) fn nested_step_runs_for_commands(
        engine: &Engine,
        netlist: &Netlist,
        steps: &[StepCommand],
    ) -> Result<Vec<XyceStepRun>, SimulationError> {
        Self::nested_step_runs_for_commands_with_limits_and_abort(
            engine,
            netlist,
            steps,
            xyce_step_plan_limits(),
            &rspice_core::abort_signal::NoAbort,
        )
    }

    pub(super) fn nested_step_runs_for_commands_with_limits_and_abort(
        engine: &Engine,
        netlist: &Netlist,
        steps: &[StepCommand],
        limits: StepPlanLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<XyceStepRun>, SimulationError> {
        let plan = engine.plan_step_commands_with_abort(netlist, steps, limits, abort)?;
        let mut runs = Vec::with_capacity(plan.total_runs());
        for run_index in 0..plan.total_runs() {
            let materialized = engine.materialize_step_run_with_abort(&plan, run_index, abort)?;
            debug_assert_eq!(materialized.run_index(), run_index);
            let (step_values, netlist) = materialized.into_parts();
            runs.push(XyceStepRun {
                step_values,
                netlist,
            });
        }
        Ok(runs)
    }

    pub(super) fn baseline_family_analysis_for_path(
        &self,
        baseline_path: &Path,
    ) -> Result<XyceBaselineFamilyAnalysis, String> {
        let source = fs::read_to_string(baseline_path)
            .map_err(|err| format!("failed to read baseline deck: {err}"))?;
        let ac_outputs = Self::print_output_requests(&source, "AC")?
            .into_iter()
            .filter(|request| request.file.is_none())
            .count();
        let dc_outputs = Self::print_output_requests(&source, "DC")?
            .into_iter()
            .filter(|request| request.file.is_none())
            .count();
        let tran_outputs = Self::print_output_requests(&source, "TRAN")?
            .into_iter()
            .filter(|request| request.file.is_none())
            .count();

        match (ac_outputs, dc_outputs, tran_outputs) {
            (1, 0, 0) => Ok(XyceBaselineFamilyAnalysis::Ac),
            (0, 1, 0) => Ok(XyceBaselineFamilyAnalysis::Dc),
            (0, 0, 1) => Ok(XyceBaselineFamilyAnalysis::Tran),
            (0, 0, 0) => Err(
                "deck has neither one primary .PRINT AC, one primary .PRINT DC, nor one primary .PRINT TRAN output"
                    .to_string(),
            ),
            (ac, dc, tran) => Err(format!(
                "deck has {ac} primary .PRINT AC output(s), {dc} primary .PRINT DC output(s), and {tran} primary .PRINT TRAN output(s); family analysis selection requires exactly one unambiguous primary output"
            )),
        }
    }

    pub(super) fn baseline_family_targets(
        contract: &XyceBaselineFamilyContract,
    ) -> (Vec<PathBuf>, bool) {
        let baseline_record = contract
            .target_path
            .as_ref()
            .is_some_and(|target| Self::same_path(target, &contract.baseline_path));
        let targets = if let Some(target_path) = contract.target_path.as_ref()
            && !baseline_record
        {
            vec![target_path.clone()]
        } else {
            contract
                .member_paths
                .iter()
                .filter(|path| !Self::same_path(path, &contract.baseline_path))
                .cloned()
                .collect()
        };
        (targets, baseline_record)
    }

    pub(super) fn tran_analyses_match_exactly(
        baseline: &XyceTranAnalysis,
        target: &XyceTranAnalysis,
    ) -> bool {
        baseline.step.to_bits() == target.step.to_bits()
            && baseline.stop.to_bits() == target.stop.to_bits()
            && baseline.start.map(Value::to_bits) == target.start.map(Value::to_bits)
            && baseline.max_step.map(Value::to_bits) == target.max_step.map(Value::to_bits)
            && baseline.uic == target.uic
    }

    pub(super) fn baseline_family_qualification_result(
        &self,
        deck: &XyceDeck,
        start: Instant,
        result_contract: &str,
        comparison: XyceBaselineFamilyComparison,
        reason: String,
    ) -> XyceTestResult {
        if comparison.strict_qualification() {
            self.failure_result(deck, start, result_contract, reason, Vec::new())
        } else {
            self.expected_unsupported_result(deck, start, result_contract, &reason)
        }
    }

    pub(super) fn simulate_resistor_dtemp_step_plan(
        &self,
        plan: &XyceStaticDcPlan,
        start: Instant,
        role: &str,
    ) -> Result<XycePrnTable, String> {
        let netlist = Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
            &plan.source,
            &plan.deck_path,
            plan.expression_dialect,
            plan.parameter_redefinition_policy,
            plan.execution_dir.as_deref(),
        )
        .map_err(|err| format!("{role} parse failed: {err}"))?;
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let engine = self.create_dc_engine();
        let step_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
            &engine,
            &netlist,
            &plan.steps,
            xyce_step_plan_limits(),
            &abort,
        )
        .map_err(|err| format!("{role} STEP expansion failed: {err}"))?;
        if step_runs.len() != 3 {
            return Err(format!(
                "{role} STEP expansion produced {} batches instead of three",
                step_runs.len()
            ));
        }

        let mut columns = vec!["Index".to_string()];
        columns.extend(plan.print.probes.iter().cloned());
        let mut rows = Vec::new();
        for (step_index, run) in step_runs.into_iter().enumerate() {
            let results = engine
                .run_dc_sweep2_spec_with_report_and_abort(
                    &run.netlist,
                    &plan.dc.source,
                    &plan.dc.primary_spec(),
                    plan.dc.sweep2.as_ref(),
                    &abort,
                )
                .map_err(|err| {
                    format!("{role} step {} simulation failed: {err}", step_index + 1)
                })?;
            if results.len() != 6 {
                return Err(format!(
                    "{role} step {} produced {} DC points instead of six",
                    step_index + 1,
                    results.len()
                ));
            }
            for (row_index, point) in results.iter().enumerate() {
                let sweep_point = XyceDcSweepPoint {
                    primary: point.sweep_value,
                    secondary: None,
                };
                let mut row = vec![row_index as Value];
                for probe in &plan.print.probes {
                    row.push(Self::evaluate_dc_probe(
                        probe,
                        &run.netlist,
                        &plan.dc,
                        sweep_point,
                        &point.result,
                        &point.device_op_report,
                    )?);
                }
                rows.push(row);
            }
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn simulate_bug647_resistor_pair(
        &self,
        owner_plan: &XyceStaticDcPlan,
        reference_plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<(XycePrnTable, XycePrnTable), String> {
        let parse = |plan: &XyceStaticDcPlan, role: &str| {
            Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
                &plan.source,
                &plan.deck_path,
                plan.expression_dialect,
                plan.parameter_redefinition_policy,
                plan.execution_dir.as_deref(),
            )
            .map_err(|err| format!("{role} parse failed: {err}"))
        };
        let owner_netlist = parse(owner_plan, "instance-parameter owner")?;
        let reference_netlist = parse(reference_plan, "model-parameter reference")?;
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let engine = self.create_dc_engine();
        let expand = |netlist: &Netlist, plan: &XyceStaticDcPlan, role: &str| {
            Self::nested_step_runs_for_commands_with_limits_and_abort(
                &engine,
                netlist,
                &plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|err| format!("{role} STEP expansion failed: {err}"))
        };
        let owner_runs = expand(&owner_netlist, owner_plan, "instance-parameter owner")?;
        let reference_runs = expand(
            &reference_netlist,
            reference_plan,
            "model-parameter reference",
        )?;
        if owner_runs.len() != 270 || reference_runs.len() != 270 {
            return Err(format!(
                "paired STEP census must be 270 runs per deck, got owner={} reference={}",
                owner_runs.len(),
                reference_runs.len()
            ));
        }

        let owner_grids = owner_plan
            .steps
            .iter()
            .map(|step| step.sweep.values())
            .collect::<Vec<_>>();
        let reference_grids = reference_plan
            .steps
            .iter()
            .map(|step| step.sweep.values())
            .collect::<Vec<_>>();
        if owner_grids.iter().map(Vec::len).collect::<Vec<_>>() != [5, 6, 3, 3]
            || owner_grids.len() != reference_grids.len()
            || owner_grids
                .iter()
                .zip(&reference_grids)
                .any(|(owner, reference)| {
                    owner.len() != reference.len()
                        || owner
                            .iter()
                            .zip(reference)
                            .any(|(owner, reference)| owner.to_bits() != reference.to_bits())
                })
        {
            return Err(
                "paired STEP grids are not the exact 5x6x3x3 W/TEMP/TC1/TC2 Cartesian product"
                    .into(),
            );
        }

        for (run_index, (owner, reference)) in owner_runs.iter().zip(&reference_runs).enumerate() {
            let expected = [
                owner_grids[0][run_index % 5],
                owner_grids[1][(run_index / 5) % 6],
                owner_grids[2][(run_index / 30) % 3],
                owner_grids[3][(run_index / 90) % 3],
            ];
            if owner.step_values.len() != 4
                || reference.step_values.len() != 4
                || owner
                    .step_values
                    .iter()
                    .zip(expected)
                    .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
                || reference
                    .step_values
                    .iter()
                    .zip(expected)
                    .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
            {
                return Err(format!(
                    "paired STEP coordinate {run_index} does not preserve first-declared-fastest W/TEMP/TC1/TC2 order"
                ));
            }

            for (parameter, coordinate_index) in
                [("W", 0usize), ("TEMP", 1), ("TC1", 2), ("TC2", 3)]
            {
                let resolve = |run: &XyceStepRun, role: &str| {
                    engine
                        .resolved_resistor_parameter(&run.netlist, "R1", parameter)
                        .map_err(|err| {
                            format!(
                                "{role} coordinate {run_index} effective {parameter} resolution failed: {err}"
                            )
                        })?
                        .ok_or_else(|| {
                            format!(
                                "{role} coordinate {run_index} has no effective R1:{parameter}"
                            )
                        })
                };
                let owner_value = resolve(owner, "owner")?;
                let reference_value = resolve(reference, "reference")?;
                if owner_value.to_bits() != reference_value.to_bits()
                    || owner_value.to_bits() != expected[coordinate_index].to_bits()
                {
                    return Err(format!(
                        "coordinate {run_index} effective R1:{parameter} differs: owner={owner_value} reference={reference_value} expected={}",
                        expected[coordinate_index]
                    ));
                }
            }
            let owner_resistance = engine
                .resolved_resistor_parameter(&owner.netlist, "R1", "R")
                .map_err(|err| {
                    format!("owner coordinate {run_index} resistance resolution failed: {err}")
                })?
                .ok_or_else(|| format!("owner coordinate {run_index} has no resistance"))?;
            let reference_resistance = engine
                .resolved_resistor_parameter(&reference.netlist, "R1", "R")
                .map_err(|err| {
                    format!("reference coordinate {run_index} resistance resolution failed: {err}")
                })?
                .ok_or_else(|| format!("reference coordinate {run_index} has no resistance"))?;
            if owner_resistance.to_bits() != reference_resistance.to_bits() {
                return Err(format!(
                    "coordinate {run_index} effective resistance differs: owner={owner_resistance} reference={reference_resistance}"
                ));
            }
        }

        let owner_table = self.simulate_bug647_resistor_runs(
            owner_plan,
            owner_runs,
            &engine,
            &abort,
            "instance-parameter owner",
        )?;
        let reference_table = self.simulate_bug647_resistor_runs(
            reference_plan,
            reference_runs,
            &engine,
            &abort,
            "model-parameter reference",
        )?;
        Ok((owner_table, reference_table))
    }

    pub(super) fn simulate_bug647_resistor_runs(
        &self,
        plan: &XyceStaticDcPlan,
        runs: Vec<XyceStepRun>,
        engine: &Engine,
        abort: &DeadlineAbort,
        role: &str,
    ) -> Result<XycePrnTable, String> {
        let mut columns = vec!["Index".to_string()];
        columns.extend(plan.print.probes.iter().cloned());
        let mut rows = Vec::with_capacity(1_620);
        for (step_index, run) in runs.into_iter().enumerate() {
            let netlist = run.netlist;
            let results = engine
                .run_dc_sweep2_spec_with_report_and_abort(
                    &netlist,
                    &plan.dc.source,
                    &plan.dc.primary_spec(),
                    plan.dc.sweep2.as_ref(),
                    abort,
                )
                .map_err(|err| {
                    format!("{role} step {} simulation failed: {err}", step_index + 1)
                })?;
            if results.len() != 6 {
                return Err(format!(
                    "{role} step {} produced {} DC points instead of six",
                    step_index + 1,
                    results.len()
                ));
            }
            for (row_index, point) in results.iter().enumerate() {
                let sweep_point = XyceDcSweepPoint {
                    primary: point.sweep_value,
                    secondary: None,
                };
                let mut row = vec![row_index as Value];
                for probe in &plan.print.probes {
                    row.push(Self::evaluate_dc_probe(
                        probe,
                        &netlist,
                        &plan.dc,
                        sweep_point,
                        &point.result,
                        &point.device_op_report,
                    )?);
                }
                rows.push(row);
            }
        }
        if rows.len() != 1_620 {
            return Err(format!(
                "{role} output has {} rows instead of 1620",
                rows.len()
            ));
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn simulate_bug655_continuation_member(
        &self,
        plan: &XyceStaticDcPlan,
        start: Instant,
        role: &str,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|err| format!("{role} simulation failed: {err}"))?;
        if results.len() != 21 {
            return Err(format!(
                "{role} produced {} DC points instead of 21",
                results.len()
            ));
        }
        self.dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|err| format!("{role} default PRN generation failed: {err}"))
    }

    pub(super) fn simulate_bug662_header_member(
        &self,
        plan: &XyceStaticTranPlan,
        start: Instant,
        role: &str,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|err| format!("{role} parse/simulation failed: {err}"))?;
        Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|err| format!("{role} default PRN generation failed: {err}"))
    }

    pub(super) fn simulate_bug667_nodeset_member(
        &self,
        plan: &XyceStaticTranPlan,
        start: Instant,
        role: &str,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|err| format!("{role} parse/simulation failed: {err}"))?;
        Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|err| format!("{role} default PRN generation failed: {err}"))
    }

    pub(super) fn simulate_bug754_global_parameter_member(
        &self,
        plan: &XyceStaticDcPlan,
        start: Instant,
        role: &str,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|err| format!("{role} simulation failed: {err}"))?;
        if results.len() != 1_001 {
            return Err(format!(
                "{role} produced {} DC points instead of 1001",
                results.len()
            ));
        }
        self.dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|err| format!("{role} default PRN generation failed: {err}"))
    }

    /// Xyce 7.10's StepErrorControl default maximum timestep is ten percent
    /// of the requested transient analysis window (`TSTOP-TSTART`). An
    /// explicit DTMAX replaces this default; it is not minimized against it.
    /// The `.TRAN` print step is the initial timestep/output cadence, not a
    /// persistent maximum timestep.
    pub(super) fn xyce_default_transient_max_step(tran: &XyceTranAnalysis) -> Value {
        (0.1 * (tran.stop - tran.start.unwrap_or(0.0))).max(f64::MIN_POSITIVE)
    }

    /// Resolve the actual Xyce transient solver ceiling. The print cadence is
    /// deliberately absent: Xyce's adaptive integrator is not constrained by
    /// either `.TRAN TSTEP` or the density of a verification table.
    pub(super) fn xyce_transient_solver_max_step(tran: &XyceTranAnalysis) -> Value {
        tran.max_step
            .unwrap_or_else(|| Self::xyce_default_transient_max_step(tran))
    }

    pub(super) fn analytic_timeint_only_options_match(
        options: &rspice_core::netlist::SimulationOptions,
        reltol_bits: u64,
        abstol_bits: u64,
        method_selector: Option<&str>,
        lte_reference: Option<TransientLteReference>,
    ) -> bool {
        let rspice_core::netlist::SimulationOptions {
            replace_ground: _,
            remove_unused: _,
            add_resistors: _,
            measure_fail_output: _,
            measure_default_value: _,
            measure_use_cont_files: _,
            reltol,
            abstol,
            vntol,
            iabstol,
            residual_reltol,
            gmin,
            method,
            trtol,
            timeint_reltol,
            timeint_abstol,
            timeint_delmax,
            timeint_use_device_max_timestep,
            nonlin_transient_reltol,
            nonlin_transient_abstol,
            nonlin_transient_deltaxtol,
            nonlin_transient_rhstol,
            nonlin_transient_maxstep,
            nonlin_transient_enforce_device_convergence,
            transient_lte_reference,
            transient_new_bp_stepping,
            ramptime,
            digital_delay_type,
            xspice_event_trace_save,
            itl1,
            itl2,
            itl4,
            itl6,
            chgtol,
            pivtol,
            temp,
            tnom,
            seed,
            allow_simplified_mos,
            auto_bridge,
            auto_bridge_show_generated,
            auto_bridge_family,
            auto_bridge_templates,
            auto_bridge_param_names,
            topology_supernode,
            device_zero_resistance_tol,
            device_min_resistance: _,
            device_min_capacitance: _,
            b3soi_gmin_scaling,
            device_try_to_compact,
            hb_num_frequencies,
            nonlinear_continuation,
            scale: _,
        } = options;
        reltol.is_none()
            && abstol.is_none()
            && timeint_reltol.is_some_and(|value| value.to_bits() == reltol_bits)
            && timeint_abstol.is_some_and(|value| value.to_bits() == abstol_bits)
            && timeint_delmax.is_none()
            && timeint_use_device_max_timestep.is_none()
            && nonlin_transient_reltol.is_none()
            && nonlin_transient_abstol.is_none()
            && nonlin_transient_deltaxtol.is_none()
            && nonlin_transient_rhstol.is_none()
            && nonlin_transient_maxstep.is_none()
            && nonlin_transient_enforce_device_convergence.is_none()
            && *transient_lte_reference == lte_reference
            && transient_new_bp_stepping.is_none()
            && vntol.is_none()
            && iabstol.is_none()
            && residual_reltol.is_none()
            && gmin.is_none()
            && method.as_deref() == method_selector
            && trtol.is_none()
            && ramptime.is_none()
            && digital_delay_type.is_none()
            && xspice_event_trace_save.is_none()
            && itl1.is_none()
            && itl2.is_none()
            && itl4.is_none()
            && itl6.is_none()
            && chgtol.is_none()
            && pivtol.is_none()
            && temp.is_none()
            && tnom.is_none()
            && seed.is_none()
            && allow_simplified_mos.is_none()
            && auto_bridge.is_none()
            && auto_bridge_show_generated.is_none()
            && auto_bridge_family.is_none()
            && auto_bridge_templates.is_empty()
            && auto_bridge_param_names.is_empty()
            && topology_supernode.is_none()
            && device_zero_resistance_tol.is_none()
            && b3soi_gmin_scaling.is_none()
            && device_try_to_compact.is_none()
            && nonlinear_continuation.is_none()
            && hb_num_frequencies.is_empty()
    }

    pub(super) fn analytic_rc_options_match(
        options: &rspice_core::netlist::SimulationOptions,
        source: &XyceAnalyticRcSourceContract,
    ) -> bool {
        Self::analytic_timeint_only_options_match(
            options,
            source.reltol_bits,
            source.abstol_bits,
            None,
            source.transient_lte_reference,
        )
    }

    pub(super) fn analytic_sinusoidal_rc_options_match(
        options: &rspice_core::netlist::SimulationOptions,
        source: &XyceAnalyticSinusoidalRcSourceContract,
    ) -> bool {
        Self::analytic_timeint_only_options_match(
            options,
            source.timeint_reltol_bits,
            source.timeint_abstol_bits,
            Some(source.method_selector.as_str()),
            None,
        )
    }

    pub(super) fn direct_numeric_function_arguments(
        field: &str,
        function_name: &str,
        arity: usize,
    ) -> Result<Vec<Value>, String> {
        let field = field.trim();
        let Some(open) = field.find('(') else {
            return Err(format!("expected {function_name}(...) source form"));
        };
        if !field[..open].eq_ignore_ascii_case(function_name)
            || !field.ends_with(')')
            || field[open + 1..field.len() - 1]
                .chars()
                .any(|ch| matches!(ch, ',' | '=' | '{' | '}' | '\'' | '"' | '(' | ')'))
        {
            return Err(format!(
                "expected one direct whitespace-separated {function_name}(...) source field"
            ));
        }
        let arguments = Self::split_grouped_whitespace_fields(
            &field[open + 1..field.len() - 1],
            &format!("{function_name} argument list"),
        )?;
        if arguments.len() != arity {
            return Err(format!(
                "{function_name} requires exactly {arity} direct numeric arguments, found {}",
                arguments.len()
            ));
        }
        arguments
            .iter()
            .map(|argument| Self::single_spice_numeric_literal_value(argument))
            .collect()
    }

    pub(super) fn bare_subckt_parameter_expression_name(expression: &str) -> Option<String> {
        let trimmed = expression.trim();
        let inner = trimmed
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(trimmed)
            .trim();
        match rspice_core::netlist::expr::parse_expression(inner).ok()? {
            rspice_core::netlist::expr::Expr::Param(name)
                if Self::is_single_spice_identifier(&name) =>
            {
                Some(name.to_ascii_lowercase())
            }
            _ => None,
        }
    }

    pub(super) fn passive_primary_name_is_literal_ground(node: &str) -> bool {
        node.trim() == "0"
    }

    pub(super) fn canonical_passive_primary_node_name(node: &str) -> String {
        if Self::passive_primary_name_is_literal_ground(node) {
            "0".to_string()
        } else {
            node.trim().to_ascii_lowercase()
        }
    }

    pub(super) fn passive_temperature_coefficient_pair(
        params: &[(String, Value)],
        owner: &str,
    ) -> Result<[Value; 2], String> {
        if params.len() != 2 {
            return Err(format!(
                "passive temperature-coefficient override {owner} requires exactly TC1 and TC2"
            ));
        }
        let tc1 = Self::instance_param(params, &["TC1"]);
        let tc2 = Self::instance_param(params, &["TC2"]);
        if tc1.is_none()
            || tc2.is_none()
            || params
                .iter()
                .filter(|(name, _)| name.eq_ignore_ascii_case("TC1"))
                .count()
                != 1
            || params
                .iter()
                .filter(|(name, _)| name.eq_ignore_ascii_case("TC2"))
                .count()
                != 1
        {
            return Err(format!(
                "passive temperature-coefficient override {owner} requires unique scalar TC1 and TC2"
            ));
        }
        let pair = [tc1.unwrap(), tc2.unwrap()];
        if pair.iter().any(|value| !value.is_finite()) {
            return Err(format!(
                "passive temperature-coefficient override {owner} TC1/TC2 must be finite"
            ));
        }
        Ok(pair)
    }

    pub(super) fn passive_temperature_instance_state(
        params: &[(String, Value)],
        model_tc: [Value; 2],
    ) -> Result<(XycePassiveTemperatureRepresentation, [Value; 2], Value), String> {
        let temperature_values = params
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case("TEMP"))
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        let [temperature] = temperature_values.as_slice() else {
            return Err(
                "passive temperature-coefficient override requires exactly one instance TEMP"
                    .to_string(),
            );
        };
        if !temperature.is_finite() {
            return Err(
                "passive temperature-coefficient override requires finite instance TEMP"
                    .to_string(),
            );
        }
        let tc_params = params
            .iter()
            .filter(|(name, _)| {
                name.eq_ignore_ascii_case("TC1") || name.eq_ignore_ascii_case("TC2")
            })
            .cloned()
            .collect::<Vec<_>>();
        if params.len() == 1 && tc_params.is_empty() {
            return Ok((
                XycePassiveTemperatureRepresentation::ModelCoefficients,
                model_tc,
                *temperature,
            ));
        }
        if params.len() == 3 && tc_params.len() == 2 {
            let instance_tc = Self::passive_temperature_coefficient_pair(&tc_params, "instance")?;
            return Ok((
                XycePassiveTemperatureRepresentation::InstanceCoefficients,
                instance_tc,
                *temperature,
            ));
        }
        Err(
            "passive temperature-coefficient override instance admits only TEMP or TEMP plus scalar TC1 and TC2"
                .to_string(),
        )
    }

    pub(super) fn delimited_expression_token(
        token: &str,
    ) -> Result<(XyceDelimitedExpressionRepresentation, &str), String> {
        const LABEL: &str = "delimited-expression parity";
        let trimmed = token.trim();
        let (representation, inner) = if let Some(inner) = trimmed
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
        {
            (XyceDelimitedExpressionRepresentation::Braced, inner)
        } else if let Some(inner) = trimmed
            .strip_prefix('\'')
            .and_then(|value| value.strip_suffix('\''))
        {
            (XyceDelimitedExpressionRepresentation::SingleQuoted, inner)
        } else {
            return Err(format!(
                "{LABEL} expression must occupy one whole braced or single-quoted token: '{token}'"
            ));
        };
        let inner = inner.trim();
        if inner.is_empty() || inner.contains('{') || inner.contains('}') || inner.contains('\'') {
            return Err(format!(
                "{LABEL} expression must be nonempty and use one pure outer delimiter dialect: '{token}'"
            ));
        }
        Ok((representation, inner))
    }

    pub(super) fn age_cap_direct_pulse_field(field: &str) -> bool {
        let field = field.trim();
        let Some(prefix) = field.get(..6) else {
            return false;
        };
        if !prefix.eq_ignore_ascii_case("PULSE(") || !field.ends_with(')') {
            return false;
        }
        let arguments = &field[6..field.len() - 1];
        let arguments = arguments
            .split(|ch: char| ch == ',' || ch.is_whitespace())
            .filter(|argument| !argument.is_empty())
            .collect::<Vec<_>>();
        matches!(arguments.len(), 6 | 7)
            && arguments
                .iter()
                .all(|argument| Self::is_single_spice_numeric_literal(argument))
    }

    pub(super) fn age_cap_pulse_spec_is_bounded(spec: &rspice_core::netlist::SourceSpec) -> bool {
        let rspice_core::netlist::SourceSpec::Pulse {
            v1,
            v2,
            delay,
            rise,
            fall,
            width,
            period,
            phase,
            ..
        } = spec
        else {
            return false;
        };
        v1.is_finite()
            && v2.is_finite()
            && delay.is_finite()
            && *delay >= 0.0
            && rise.is_finite()
            && *rise > 0.0
            && fall.is_finite()
            && *fall > 0.0
            && width.is_finite()
            && *width > 0.0
            && (period.is_nan() || (period.is_finite() && *period > 0.0))
            && phase.is_finite()
    }

    pub(super) fn age_cap_parameter_graph(
        definitions: &BTreeMap<String, rspice_core::netlist::expr::Expr>,
        cap_expression: &rspice_core::netlist::expr::Expr,
    ) -> Result<[String; 5], String> {
        use rspice_core::netlist::expr::{BinOpKind, Expr};
        const LABEL: &str = "native capacitor AGE/D equivalence";
        let Expr::Param(effective_name) = cap_expression else {
            return Err(format!(
                "{LABEL} capacitor must reference one effective-capacitance parameter"
            ));
        };
        let effective_name = effective_name.to_ascii_lowercase();
        let Some(Expr::BinOp {
            op: BinOpKind::Mul,
            left,
            right,
        }) = definitions.get(&effective_name)
        else {
            return Err(format!(
                "{LABEL} effective parameter must be base*(1-D*log_age)"
            ));
        };
        let Expr::Param(base_name) = left.as_ref() else {
            return Err(format!(
                "{LABEL} effective parameter must begin with a base parameter"
            ));
        };
        let Expr::BinOp {
            op: BinOpKind::Sub,
            left: one,
            right: degradation_product,
        } = right.as_ref()
        else {
            return Err(format!("{LABEL} effective parameter must use 1-D*log_age"));
        };
        if !matches!(one.as_ref(), Expr::Number(value) if value.to_bits() == 1.0f64.to_bits()) {
            return Err(format!("{LABEL} effective parameter requires literal one"));
        }
        let Expr::BinOp {
            op: BinOpKind::Mul,
            left: degradation,
            right: log_age,
        } = degradation_product.as_ref()
        else {
            return Err(format!("{LABEL} effective parameter requires D*log_age"));
        };
        let (Expr::Param(degradation_name), Expr::Param(log_name)) =
            (degradation.as_ref(), log_age.as_ref())
        else {
            return Err(format!("{LABEL} D and log-age terms must be parameters"));
        };
        let base_name = base_name.to_ascii_lowercase();
        let degradation_name = degradation_name.to_ascii_lowercase();
        let log_name = log_name.to_ascii_lowercase();
        let Some(Expr::FnCall { name, args }) = definitions.get(&log_name) else {
            return Err(format!("{LABEL} log-age parameter must be log10(age)"));
        };
        let [Expr::Param(age_name)] = args.as_slice() else {
            return Err(format!(
                "{LABEL} log10 must consume exactly one age parameter"
            ));
        };
        if !name.eq_ignore_ascii_case("log10") {
            return Err(format!("{LABEL} age degradation requires log10"));
        }
        let age_name = age_name.to_ascii_lowercase();
        let names = [
            base_name,
            degradation_name,
            age_name,
            log_name,
            effective_name,
        ];
        if definitions.len() != 5 || names.iter().collect::<BTreeSet<_>>().len() != 5 {
            return Err(format!(
                "{LABEL} requires five distinct, fully used parameters"
            ));
        }
        let literal = |name: &str| match definitions.get(name) {
            Some(Expr::Number(value)) if value.is_finite() => Some(*value),
            _ => None,
        };
        let base =
            literal(&names[0]).ok_or_else(|| format!("{LABEL} base must be a finite literal"))?;
        let degradation =
            literal(&names[1]).ok_or_else(|| format!("{LABEL} D must be a finite literal"))?;
        let age =
            literal(&names[2]).ok_or_else(|| format!("{LABEL} age must be a finite literal"))?;
        if base <= 0.0 || degradation < 0.0 || age <= 1.0 {
            return Err(format!("{LABEL} requires base>0, D>=0, and age>1"));
        }
        Ok(names)
    }

    pub(super) fn age_cap_semantic_values(
        netlist: &Netlist,
        source: &str,
        representation: XyceAgeCapRepresentation,
        effective: Value,
    ) -> Result<[u64; 4], String> {
        const LABEL: &str = "native capacitor AGE/D equivalence";
        if representation == XyceAgeCapRepresentation::NativeAge {
            let capacitor = netlist
                .elements
                .iter()
                .find(|element| matches!(element.kind, ElementKind::Capacitor { .. }))
                .ok_or_else(|| format!("{LABEL} has no capacitor"))?;
            let ElementKind::Capacitor {
                value,
                instance_params,
                ..
            } = &capacitor.kind
            else {
                unreachable!()
            };
            let age = instance_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("AGE"))
                .map(|(_, value)| *value)
                .ok_or_else(|| format!("{LABEL} aged capacitor has no AGE"))?;
            let degradation = instance_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("D"))
                .map(|(_, value)| *value)
                .unwrap_or(rspice_core::engine::XYCE_DEFAULT_CAPACITOR_AGE_DEGRADATION);
            return Ok([
                value.to_bits(),
                age.to_bits(),
                degradation.to_bits(),
                effective.to_bits(),
            ]);
        }
        let mut definitions = BTreeMap::new();
        let mut cap_ast = None;
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".param") {
                let (name, expression) = stripped[command.len()..]
                    .trim()
                    .split_once('=')
                    .ok_or_else(|| format!("{LABEL} malformed parameter"))?;
                let expression = expression.trim();
                let inner = Self::print_expression_inner(expression).unwrap_or(expression);
                definitions.insert(
                    name.trim().to_ascii_lowercase(),
                    rspice_core::netlist::expr::parse_expression(inner)
                        .map_err(|err| format!("{LABEL} malformed parameter: {err}"))?,
                );
            } else if command
                .chars()
                .next()
                .is_some_and(|ch| ch.eq_ignore_ascii_case(&'C'))
            {
                let fields = Self::split_grouped_whitespace_fields(stripped, LABEL)?;
                let expression = fields
                    .get(3)
                    .and_then(|field| Self::print_expression_inner(field))
                    .ok_or_else(|| format!("{LABEL} parameter capacitor is not an expression"))?;
                cap_ast = Some(
                    rspice_core::netlist::expr::parse_expression(expression)
                        .map_err(|err| format!("{LABEL} malformed capacitor expression: {err}"))?,
                );
            }
        }
        let names = Self::age_cap_parameter_graph(
            &definitions,
            &cap_ast.ok_or_else(|| format!("{LABEL} missing capacitor expression"))?,
        )?;
        let value = |name: &str| {
            netlist
                .params
                .get(name)
                .filter(|value| value.is_finite())
                .ok_or_else(|| format!("{LABEL} parameter '{name}' is not finite"))
        };
        let base = value(&names[0])?;
        let degradation = value(&names[1])?;
        let age = value(&names[2])?;
        let declared_effective = value(&names[4])?;
        if declared_effective.to_bits() != effective.to_bits() {
            return Err(format!("{LABEL} declared and stamped capacitances differ"));
        }
        Ok([
            base.to_bits(),
            age.to_bits(),
            degradation.to_bits(),
            effective.to_bits(),
        ])
    }

    pub(super) fn param_expression_name_is_literal_ground(node: &str) -> bool {
        node.trim() == "0"
    }

    pub(super) fn canonical_param_expression_node_name(node: &str) -> String {
        if Self::param_expression_name_is_literal_ground(node) {
            "0".to_string()
        } else {
            node.trim().to_ascii_lowercase()
        }
    }

    pub(super) fn raw_param_expression_voltage_node(
        expression: &rspice_core::netlist::expr::Expr,
    ) -> Option<&str> {
        let rspice_core::netlist::expr::Expr::FnCall { name, args } = expression else {
            return None;
        };
        let [rspice_core::netlist::expr::Expr::Param(node)] = args.as_slice() else {
            return None;
        };
        name.eq_ignore_ascii_case("V").then_some(node.as_str())
    }

    pub(super) fn raw_param_expression_squared_voltage_difference_matches(
        expression: &rspice_core::netlist::expr::Expr,
        positive_node: &str,
        negative_node: &str,
    ) -> bool {
        use rspice_core::netlist::expr::{BinOpKind, Expr as NetExpr};
        let NetExpr::BinOp {
            op: BinOpKind::Pow,
            left,
            right,
        } = expression
        else {
            return false;
        };
        let NetExpr::Number(exponent) = right.as_ref() else {
            return false;
        };
        let NetExpr::BinOp {
            op: BinOpKind::Sub,
            left,
            right,
        } = left.as_ref()
        else {
            return false;
        };
        exponent.to_bits() == 2.0f64.to_bits()
            && Self::raw_param_expression_voltage_node(left)
                .is_some_and(|node| node.eq_ignore_ascii_case(positive_node))
            && Self::raw_param_expression_voltage_node(right)
                .is_some_and(|node| node.eq_ignore_ascii_case(negative_node))
    }

    pub(super) fn qualify_raw_param_expression(
        expression: &str,
        parameter_name: &str,
        parameter_value: Value,
        ports: &[String],
    ) -> Result<XyceParamExpressionRepresentation, String> {
        use rspice_core::netlist::expr::{BinOpKind, Expr as NetExpr};
        let ast = rspice_core::netlist::expr::parse_expression(expression)
            .map_err(|err| format!("could not parse the behavioral expression: {err}"))?;
        let NetExpr::BinOp {
            op: BinOpKind::Mul,
            left: coefficient,
            right: magnitude,
        } = ast
        else {
            return Err(
                "behavioral expression must directly multiply its coefficient by sqrt(...)"
                    .to_string(),
            );
        };
        let representation = match coefficient.as_ref() {
            NetExpr::Param(name) if name.eq_ignore_ascii_case(parameter_name) => {
                XyceParamExpressionRepresentation::ParameterCoefficient
            }
            NetExpr::Number(value) if value.to_bits() == parameter_value.to_bits() => {
                XyceParamExpressionRepresentation::LiteralCoefficient
            }
            NetExpr::Param(_) => {
                return Err(
                    "behavioral coefficient must reference the unique global parameter directly"
                        .to_string(),
                );
            }
            NetExpr::Number(value) => {
                return Err(format!(
                    "literal behavioral coefficient {value} does not exactly equal the global parameter value {parameter_value}"
                ));
            }
            _ => {
                return Err(
                    "behavioral coefficient must be one direct parameter reference or numeric literal"
                        .to_string(),
                );
            }
        };
        let NetExpr::FnCall { name, args } = magnitude.as_ref() else {
            return Err("behavioral magnitude must be one direct sqrt(...) call".to_string());
        };
        if !name.eq_ignore_ascii_case("sqrt") {
            return Err("behavioral magnitude function must be sqrt".to_string());
        }
        let [
            NetExpr::BinOp {
                op: BinOpKind::Add,
                left: first,
                right: second,
            },
        ] = args.as_slice()
        else {
            return Err(
                "sqrt must contain one direct sum of two squared voltage differences".to_string(),
            );
        };
        if ports.len() != 6
            || !Self::raw_param_expression_squared_voltage_difference_matches(
                first, &ports[2], &ports[3],
            )
            || !Self::raw_param_expression_squared_voltage_difference_matches(
                second, &ports[4], &ports[5],
            )
        {
            return Err(
                "behavioral expression must preserve the ordered squared voltage differences for subcircuit ports 2/3 and 4/5"
                    .to_string(),
            );
        }
        Ok(representation)
    }

    pub(super) fn qualify_prepared_param_expression(
        expression: &str,
        params: &rspice_core::netlist::expr::ParamContext,
        coefficient: Value,
        first_positive: &str,
        first_negative: &str,
        second_positive: &str,
        second_negative: &str,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params)
            .map_err(|err| format!("could not prepare the flattened expression: {err}"))?;
        let ast = parse_expression_strict(&prepared)
            .map_err(|err| format!("could not parse the prepared flattened expression: {err}"))?;
        let Expr::Binary {
            op: rspice_core::expr::BinaryOp::Mul,
            left,
            right,
        } = ast
        else {
            return Err(
                "prepared expression must directly multiply a constant by sqrt(...)".to_string(),
            );
        };
        let Expr::Const(prepared_coefficient) = left.as_ref() else {
            return Err("prepared behavioral coefficient must be constant".to_string());
        };
        if prepared_coefficient.to_bits() != coefficient.to_bits() {
            return Err(format!(
                "prepared behavioral coefficient {prepared_coefficient} does not exactly equal {coefficient}"
            ));
        }
        let Expr::Function {
            func: rspice_core::expr::Function::Sqrt,
            args,
        } = right.as_ref()
        else {
            return Err("prepared behavioral magnitude must be sqrt".to_string());
        };
        let [
            Expr::Binary {
                op: rspice_core::expr::BinaryOp::Add,
                left: first,
                right: second,
            },
        ] = args.as_slice()
        else {
            return Err(
                "prepared sqrt must contain one direct sum of squared voltage differences"
                    .to_string(),
            );
        };
        if !Self::strict_param_expression_squared_voltage_difference_matches(
            first,
            first_positive,
            first_negative,
        ) || !Self::strict_param_expression_squared_voltage_difference_matches(
            second,
            second_positive,
            second_negative,
        ) {
            return Err(
                "prepared expression changed the ordered squared voltage-difference topology"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn sin_expression_name_is_literal_ground(node: &str) -> bool {
        node.trim() == "0"
    }

    pub(super) fn canonical_sin_expression_node_name(node: &str) -> String {
        if Self::sin_expression_name_is_literal_ground(node) {
            "0".to_string()
        } else {
            node.trim().to_ascii_lowercase()
        }
    }

    pub(super) fn qualified_sin_expression_waveform_bits(
        offset: Value,
        amplitude: Value,
        frequency: Value,
    ) -> Result<[u64; 6], String> {
        if !offset.is_finite()
            || !amplitude.is_finite()
            || amplitude == 0.0
            || !frequency.is_finite()
            || frequency <= 0.0
        {
            return Err(format!(
                "qualified SIN/SPICE_SIN values require finite offset, finite nonzero amplitude, and finite positive frequency; got offset={offset}, amplitude={amplitude}, frequency={frequency}"
            ));
        }
        Ok([
            offset.to_bits(),
            amplitude.to_bits(),
            frequency.to_bits(),
            0.0f64.to_bits(),
            0.0f64.to_bits(),
            0.0f64.to_bits(),
        ])
    }

    pub(super) fn bjt_external_node_name_is_literal_ground(node: &str) -> bool {
        node.trim() == "0"
    }

    /// Xyce's lexical ground spellings used by exact-oracle admission guards.
    /// These guards inspect authored decks before REPLACEGROUND policy is
    /// applied, so they must not use the execution layer's canonical-only
    /// ground predicate.
    pub(super) fn xyce_ground_alias_name(node: &str) -> bool {
        matches!(
            node.trim().to_ascii_uppercase().as_str(),
            "0" | "GND" | "GND!" | "GROUND"
        )
    }

    pub(super) fn canonical_bjt_external_node_name(node: &str) -> String {
        if Self::bjt_external_node_name_is_literal_ground(node) {
            "0".to_string()
        } else {
            node.trim().to_ascii_lowercase()
        }
    }

    pub(super) fn normalized_xyce_verify_transient_rows(
        table: &XycePrnTable,
        role: &str,
        tolerances: &[XyceVerifyTransientTolerance],
        scientific_precision: usize,
    ) -> Result<Vec<(Value, Vec<Value>)>, String> {
        if table.columns.len() < 3
            || !table.columns[0].eq_ignore_ascii_case("Index")
            || !table.columns[1].eq_ignore_ascii_case("TIME")
        {
            return Err(format!(
                "{role} table must contain Index, TIME, and at least one output column"
            ));
        }
        if table.rows.is_empty() {
            return Err(format!("{role} table contains no transient rows"));
        }
        if tolerances.len() != table.columns.len() - 2 {
            return Err(format!(
                "{role} table has {} output column(s), but {} xyce_verify tolerance(s) were supplied",
                table.columns.len() - 2,
                tolerances.len()
            ));
        }

        let mut normalized = Vec::with_capacity(table.rows.len());
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != table.columns.len() {
                return Err(format!(
                    "{role} row {row_index} has {} values, expected {}",
                    row.len(),
                    table.columns.len()
                ));
            }
            if row.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "{role} row {row_index} contains a non-finite value"
                ));
            }

            let zero_small = |value: Value, zero_tolerance: Value| {
                if value.abs() <= zero_tolerance {
                    0.0
                } else {
                    value
                }
            };
            // rc_osc overrides only its expression's *COMP tolerance. TIME
            // retains Release 7.10's independent default zero threshold.
            let time = zero_small(
                Self::xyce_prn_scientific_roundtrip(row[1], scientific_precision).map_err(
                    |err| format!("could not serialize {role} TIME at row {row_index}: {err}"),
                )?,
                XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE,
            );
            let mut values = Vec::with_capacity(row.len() - 2);
            for (column_index, &value) in row[2..].iter().enumerate() {
                let serialized = Self::xyce_prn_scientific_roundtrip(value, scientific_precision)
                    .map_err(|err| {
                    format!(
                        "could not serialize {role} {} at row {row_index}: {err}",
                        table.columns[column_index + 2]
                    )
                })?;
                // Release 7.10 ReadDataFile applies *COMP OFFSET to both the
                // good and test series before ZEROTOL normalization.
                let shifted = serialized + tolerances[column_index].offset;
                if !shifted.is_finite() {
                    return Err(format!(
                        "{role} {} at row {row_index} became non-finite after applying OFFSET={}",
                        table.columns[column_index + 2],
                        tolerances[column_index].offset
                    ));
                }
                values.push(zero_small(shifted, tolerances[column_index].zero));
            }
            if normalized
                .last()
                .is_some_and(|(previous_time, _)| *previous_time == time)
            {
                // Release 7.10's ReadDataFile keeps the first printed row at
                // a time and discards immediately following duplicates before
                // interpolation.
                continue;
            }
            if normalized
                .last()
                .is_some_and(|(previous_time, _)| *previous_time > time)
            {
                return Err(format!(
                    "{role} transient times are not monotonically increasing at row {row_index}"
                ));
            }
            normalized.push((time, values));
        }
        if normalized.is_empty() {
            return Err(format!(
                "{role} table has no rows after duplicate-time normalization"
            ));
        }
        Ok(normalized)
    }

    pub(super) fn xyce_verify_linear_interpolate(
        low_time: Value,
        high_time: Value,
        test_time: Value,
        low_value: Value,
        high_value: Value,
    ) -> Value {
        (high_value - low_value) / (high_time - low_time) * (test_time - low_time) + low_value
    }

    pub(super) fn xyce_verify_normalized_error(expected: Value, actual: Value) -> Value {
        Self::xyce_verify_normalized_error_with_tolerance(
            expected,
            actual,
            XyceVerifyTransientTolerance::release_7_10_default(),
        )
    }

    pub(super) fn xyce_verify_quotient_operand_indices(
        columns: &[String],
    ) -> Vec<Option<(usize, usize)>> {
        let normalized_columns = columns
            .iter()
            .map(|column| Self::normalize_probe(column))
            .collect::<Vec<_>>();
        columns
            .iter()
            .map(|column| {
                let expression = Self::print_expression_inner(column)?;
                let ast = parse_expression_strict(expression).ok()?;
                let (numerator_probe, divisor_probe) =
                    Self::xyce_verify_direct_quotient_probes(&ast)?;
                let normalized_numerator = Self::normalize_probe(&numerator_probe);
                let normalized_divisor = Self::normalize_probe(&divisor_probe);
                let numerator_index = normalized_columns
                    .iter()
                    .position(|candidate| *candidate == normalized_numerator)?;
                let divisor_index = normalized_columns
                    .iter()
                    .position(|candidate| *candidate == normalized_divisor)?;
                Some((numerator_index, divisor_index))
            })
            .collect()
    }

    pub(super) fn xyce_verify_direct_quotient_probes(
        expression: &Expr,
    ) -> Option<(String, String)> {
        let Expr::Binary {
            op: BinaryOp::Div,
            left,
            right,
            ..
        } = expression
        else {
            return None;
        };
        Some((
            Self::xyce_verify_direct_probe(left)?,
            Self::xyce_verify_direct_probe(right)?,
        ))
    }

    pub(super) fn xyce_verify_direct_probe(expression: &Expr) -> Option<String> {
        match expression {
            Expr::NodeVoltage(node) => Some(format!("V({node})")),
            Expr::BranchCurrent(element) => Some(format!("I({element})")),
            _ => None,
        }
    }

    pub(super) fn create_dc_engine(&self) -> Engine {
        self.create_xyce_engine()
    }

    pub(super) fn create_xyce_engine(&self) -> Engine {
        self.create_xyce_engine_with_locked_time_grid(None)
    }

    pub(super) fn create_xyce_engine_with_locked_time_grid(
        &self,
        locked_time_grid: Option<Vec<Value>>,
    ) -> Engine {
        Engine::new(self.xyce_engine_config(locked_time_grid))
    }

    pub(super) fn create_xyce_static_tran_engine(
        &self,
        locked_time_grid: Option<Vec<Value>>,
        initial_timestep: Option<Value>,
    ) -> Engine {
        let mut config = self.xyce_engine_config(locked_time_grid);
        config.transient_initial_timestep = initial_timestep;
        config.integration_method =
            rspice_core::numerics::integration::IntegrationMethod::Trapezoidal;
        Engine::new(config)
    }

    pub(super) fn create_xyce_static_tran_engine_with_step_sizes(
        &self,
        locked_time_grid: Option<Vec<Value>>,
        locked_time_step_sizes: Option<Vec<Value>>,
        initial_timestep: Option<Value>,
    ) -> Engine {
        self.create_xyce_static_tran_engine_with_step_sizes_and_integration_method(
            locked_time_grid,
            locked_time_step_sizes,
            initial_timestep,
            rspice_core::numerics::integration::IntegrationMethod::Trapezoidal,
        )
    }

    pub(super) fn create_xyce_static_tran_engine_with_step_sizes_and_integration_method(
        &self,
        locked_time_grid: Option<Vec<Value>>,
        locked_time_step_sizes: Option<Vec<Value>>,
        initial_timestep: Option<Value>,
        integration_method: rspice_core::numerics::integration::IntegrationMethod,
    ) -> Engine {
        let mut config = self.xyce_engine_config(locked_time_grid);
        config.locked_time_step_sizes = locked_time_step_sizes.map(Arc::new);
        config.transient_initial_timestep = initial_timestep;
        config.integration_method = integration_method;
        Engine::new(config)
    }

    pub(super) fn create_xyce_static_tran_engine_with_integration_method(
        &self,
        locked_time_grid: Option<Vec<Value>>,
        integration_method: rspice_core::numerics::integration::IntegrationMethod,
        initial_timestep: Option<Value>,
    ) -> Engine {
        let mut config = self.xyce_engine_config(locked_time_grid);
        config.transient_initial_timestep = initial_timestep;
        config.integration_method = integration_method;
        Engine::new(config)
    }

    pub(super) fn xyce_engine_config(
        &self,
        locked_time_grid: Option<Vec<Value>>,
    ) -> SimulationConfig {
        let defaults = SimulationConfig::default();
        let mut convergence_config = ConvergenceConfig::robust();
        // Xyce 7.10 DeviceOptions uses an independent RELTOL=1e-4 for
        // device-current/voltage convergence.  Its nonlinear-solver residual
        // status test remains at 1e-3, so do not replace the whole convergence
        // tolerance set with the device default.
        convergence_config.voltage_reltol = 1.0e-4;
        SimulationConfig {
            max_iterations: defaults.max_iterations.max(1200),
            convergence_config,
            spice_dialect: SpiceDialect::Xyce,
            xyce_tra_interpolation: self.config.xyce_tra_interpolation,
            // Xyce and ngspice regression decks use 27 C unless overridden.
            temperature: 300.15,
            locked_time_grid: locked_time_grid.map(Arc::new),
            ..defaults
        }
    }

    pub(super) fn normalized_probe_requests_linear_capacitor_branch_quantity(
        netlist: &Netlist,
        normalized: &str,
    ) -> bool {
        if let Some(element_name) = Self::parse_current_probe(normalized)
            && Self::find_capacitor_element(netlist, &element_name).is_some()
        {
            return true;
        }
        if let Some(element_name) = Self::parse_power_probe(normalized)
            && Self::find_capacitor_element(netlist, &element_name).is_some()
        {
            return true;
        }
        false
    }

    pub(super) fn xyce_dc_sensitivity_trace_matches(
        trace: &rspice_core::analysis::Sensitivity,
        parameter: &str,
    ) -> bool {
        trace.vector_name.eq_ignore_ascii_case(parameter)
            || trace.element.eq_ignore_ascii_case(parameter)
            || format!("{}:{}", trace.element, trace.parameter).eq_ignore_ascii_case(parameter)
    }

    pub(super) fn xyce_sensitivity_objective_probe(
        spec: &XyceAcSensitivityObjectiveSpec,
    ) -> String {
        match spec {
            XyceAcSensitivityObjectiveSpec::Voltage { positive, negative } => negative
                .as_deref()
                .map(|negative| format!("v({positive},{negative})"))
                .unwrap_or_else(|| format!("v({positive})")),
            XyceAcSensitivityObjectiveSpec::BranchCurrent(element) => format!("i({element})"),
        }
    }

    pub(super) fn xyce_sensitivity_column_name(
        component: &str,
        objective_probe: &str,
        parameter: Option<&str>,
        mode: Option<&str>,
    ) -> String {
        let objective = format!("{{{objective_probe}}}");
        match (parameter, mode) {
            (Some(parameter), Some(mode)) => {
                Self::normalize_probe(&format!("d_{component}({objective})/d_{parameter}_{mode}"))
            }
            (None, None) => Self::normalize_probe(&format!("{component}({objective})")),
            _ => unreachable!("sensitivity column parameter/mode must be paired"),
        }
    }

    pub(super) fn xyce_sensitivity_trace_matches(
        trace: &rspice_core::analysis::AcSensitivity,
        parameter: &str,
    ) -> bool {
        trace.vector_name.eq_ignore_ascii_case(parameter)
            || format!("{}:{}", trace.element, trace.parameter).eq_ignore_ascii_case(parameter)
    }

    pub(super) fn xyce_sensitivity_value(
        component: &str,
        output: Complex64,
        row_index: usize,
        trace: Option<&rspice_core::analysis::AcSensitivity>,
        phase_output_radians: bool,
    ) -> Value {
        let value = trace.map_or(output, |trace| {
            // The sensitivity trace stores the complex derivative in
            // `absolute`; the other fields are derivatives of magnitude and
            // phase and are selected below.
            trace.absolute[row_index]
        });
        if let Some(trace) = trace {
            return match component {
                "re" => trace.absolute[row_index].re,
                "im" => trace.absolute[row_index].im,
                "mag" => trace.magnitude[row_index],
                "ph" if phase_output_radians => trace.phase[row_index],
                "ph" => trace.phase[row_index].to_degrees(),
                _ => 0.0,
            };
        }
        match component {
            "re" => value.re,
            "im" => value.im,
            "mag" => value.norm(),
            "ph" if phase_output_radians => value.arg(),
            "ph" => value.arg().to_degrees(),
            _ => 0.0,
        }
    }

    pub(super) fn continuous_measurement_failure_is_uninitialized(
        measure_type: &rspice_core::analysis::MeasureType,
        failure: &str,
    ) -> bool {
        match measure_type {
            rspice_core::analysis::MeasureType::When { .. } => {
                failure == "WHEN condition not found in measurement window"
            }
            rspice_core::analysis::MeasureType::Find { .. } => matches!(
                failure,
                "Time point not in simulation range"
                    | "WHEN condition not found in the measurement window"
            ),
            rspice_core::analysis::MeasureType::Derivative { .. } => matches!(
                failure,
                "Time point not in simulation range"
                    | "AT point is outside the measurement window"
                    | "WHEN condition never met in the measurement window"
            ),
            rspice_core::analysis::MeasureType::Delay { .. } => {
                failure == "trigger/target event pair not found"
            }
            _ => false,
        }
    }

    pub(super) fn continuous_record_activation_index(
        accepted_axis: &[Value],
        segment_starts: &[usize],
        record: &rspice_core::analysis::ContinuousMeasureRecord,
        minimum_index: usize,
    ) -> Option<usize> {
        let locate = |event_axis| {
            Self::accepted_point_activation_index(
                accepted_axis,
                segment_starts,
                event_axis,
                minimum_index,
            )
        };
        if let Some(event_axis) = record.event_axis {
            return locate(event_axis);
        }
        match (record.trigger_axis, record.target_axis) {
            (Some(trigger), Some(target)) => Some(locate(trigger)?.max(locate(target)?)),
            (Some(trigger), None) => locate(trigger),
            (None, Some(target)) => locate(target),
            (None, None) => None,
        }
    }

    pub(super) fn accepted_point_activation_index(
        accepted_axis: &[Value],
        segment_starts: &[usize],
        event_axis: Value,
        minimum_index: usize,
    ) -> Option<usize> {
        if !event_axis.is_finite() || minimum_index >= accepted_axis.len() {
            return None;
        }
        let segment_starts = segment_starts.iter().copied().collect::<BTreeSet<_>>();
        for index in minimum_index..accepted_axis.len() {
            let axis = accepted_axis[index];
            let scale = axis.abs().max(event_axis.abs()).max(1.0);
            if (axis - event_axis).abs() <= 32.0 * f64::EPSILON * scale {
                return Some(index);
            }
            if index == 0 || segment_starts.contains(&index) {
                continue;
            }
            let previous = accepted_axis[index - 1];
            if event_axis >= previous.min(axis) && event_axis <= previous.max(axis) {
                return Some(index);
            }
        }
        None
    }

    pub(super) fn pointwise_switch_transition_rms_fallback_passes(
        &self,
        reference: &XycePrnTable,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<bool, String> {
        if plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !Self::pointwise_switch_transition_needs_rms_fallback(netlist)
        {
            return Ok(false);
        }

        let mut rms_plan = plan.clone();
        rms_plan.comparison_mode = XyceStaticTranComparisonMode::Release710IntegratedRms {
            scientific_precision: XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        };
        Ok(self
            .compare_static_tran_primary_reference(reference, &rms_plan, netlist, result)?
            .is_empty())
    }

    pub(super) fn compact_device_work_limited_step(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
    ) -> Result<Option<Value>, String> {
        if tran.max_step.is_some() {
            return Ok(None);
        }
        let compact_device_count =
            Self::transient_flattened_problem_size(netlist)?.compact_device_count;
        if compact_device_count == 0 {
            return Ok(None);
        }

        // The harness compares by interpolation. Do not turn one tiny oracle
        // or source-resolution target into hundreds of thousands of globally
        // capped nonlinear solves. This relaxes only the harness-imposed cap;
        // engine LTE control and source breakpoints may still accept smaller
        // steps, and an explicit deck TMAX always wins.
        Ok(Some(
            tran.stop * compact_device_count as Value / MAX_NATIVE_TRAN_TARGET_COMPACT_DEVICE_STEPS,
        ))
    }

    pub(super) fn preflight_transient_estimated_steps(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
    ) -> Value {
        let solver_max_step = Self::transient_oracle_solver_max_step(tran);
        let source_step = Self::source_transient_max_step(netlist, tran)
            .and_then(|step| Self::feasible_oracle_limited_step(tran, step));
        let max_step = [Some(solver_max_step), source_step]
            .into_iter()
            .flatten()
            .filter(|value| value.is_finite() && *value > 0.0)
            .reduce(Value::min)
            .unwrap_or(solver_max_step);
        (tran.stop / max_step).ceil()
    }

    pub(super) fn subcircuit_problem_size_estimate(
        subcircuit: &SubcircuitDef,
        defs: &BTreeMap<String, &SubcircuitDef>,
        stack: &mut BTreeSet<String>,
    ) -> Result<XyceTransientProblemSize, String> {
        let key = subcircuit.name.to_ascii_lowercase();
        if !stack.insert(key.clone()) {
            return Err(format!(
                "transient harness execution envelope cannot estimate recursive subcircuit '{}'",
                subcircuit.name
            ));
        }

        let ports = subcircuit
            .ports
            .iter()
            .map(|port| port.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let mut local_nodes = BTreeSet::new();
        let mut element_count = 0usize;
        let mut compact_device_count = 0usize;
        let mut internal_node_count = 0usize;

        for element in &subcircuit.elements {
            for node in &element.nodes {
                let normalized = node.to_ascii_lowercase();
                if !Self::node_name_is_ground(node) && !ports.contains(&normalized) {
                    local_nodes.insert(normalized);
                }
            }
            if let ElementKind::Subcircuit { subckt_name, .. } = &element.kind {
                let child = defs.get(&subckt_name.to_ascii_lowercase()).ok_or_else(|| {
                    format!(
                        "transient harness execution envelope cannot estimate unresolved subcircuit '{}'",
                        subckt_name
                    )
                })?;
                let size = Self::subcircuit_problem_size_estimate(child, defs, stack)?;
                element_count += size.element_count;
                compact_device_count += size.compact_device_count;
                internal_node_count += size.node_count;
            } else {
                element_count += 1;
                compact_device_count += Self::transient_element_compact_device_count(element);
            }
        }

        stack.remove(&key);
        Ok(XyceTransientProblemSize {
            element_count,
            compact_device_count,
            node_count: local_nodes.len() + internal_node_count,
        })
    }

    pub(super) fn node_name_is_ground(node: &str) -> bool {
        let normalized = node.trim();
        normalized == "0"
            || normalized.eq_ignore_ascii_case("gnd")
            || normalized.eq_ignore_ascii_case("ground")
    }

    pub(super) fn xyce_initial_timestep_for_tran(tran: &XyceTranAnalysis) -> Option<Value> {
        let step = if tran.step.is_finite() && tran.step > 0.0 {
            tran.step
        } else {
            1.0e-10
        };
        Some(step.max(1.0e-30))
    }

    pub(super) fn feasible_oracle_limited_step(
        tran: &XyceTranAnalysis,
        step: Value,
    ) -> Option<Value> {
        let step = (step.is_finite() && step > f64::MIN_POSITIVE).then_some(step)?;
        let estimated_steps = (tran.stop / step).ceil();
        (estimated_steps <= MAX_NATIVE_TRAN_ORACLE_STEPS).then_some(step)
    }

    pub(super) fn positive_duration_step(
        duration: Value,
        points_per_duration: Value,
    ) -> Option<Value> {
        (duration.is_finite() && duration > 0.0 && points_per_duration > 0.0)
            .then_some(duration / points_per_duration)
    }

    pub(super) fn positive_frequency_step(frequency: Value) -> Option<Value> {
        (frequency.is_finite() && frequency > 0.0)
            .then_some(1.0 / (frequency * TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD))
    }

    pub(super) fn resolved_sin_frequency(frequency: Value, tstop: Value) -> Value {
        if frequency.is_finite() && frequency > 0.0 {
            frequency
        } else if tstop.is_finite() && tstop > 0.0 {
            1.0 / tstop
        } else {
            1.0e3
        }
    }

    pub(super) fn resolved_modulated_frequency(
        frequency: Value,
        default_cycles: Value,
        tstop: Value,
    ) -> Value {
        if frequency.is_finite() && frequency > 0.0 {
            frequency
        } else if tstop.is_finite() && tstop > 0.0 {
            default_cycles / tstop
        } else {
            default_cycles * 1.0e3
        }
    }

    pub(super) fn normalize_ac_expression_probe_key(probe: &str) -> String {
        let expression = Self::print_expression_inner(probe).unwrap_or(probe);
        Self::normalize_probe(expression)
    }

    pub(super) fn split_comp_directive(rest: &str) -> Option<(String, String)> {
        let rest = rest
            .split_once(';')
            .map(|(head, _)| head)
            .unwrap_or(rest)
            .trim();
        if rest.is_empty() {
            return None;
        }

        if rest.starts_with('{') {
            let mut depth = 0usize;
            for (index, ch) in rest.char_indices() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            let end = index + ch.len_utf8();
                            return Some((
                                rest[..end].trim().to_string(),
                                rest[end..].trim().to_string(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut split = rest.splitn(2, char::is_whitespace);
        let probe = split.next()?.trim();
        if probe.is_empty() {
            return None;
        }
        Some((
            probe.to_string(),
            split.next().unwrap_or_default().trim().to_string(),
        ))
    }

    pub(super) fn assignment_value(line: &str, key: &str) -> Result<Option<String>, String> {
        let normalized = Self::normalize_assignment_spacing(line);
        for field in normalized.split_whitespace().skip(1) {
            let Some((name, value)) = field.split_once('=') else {
                continue;
            };
            if !name.eq_ignore_ascii_case(key) {
                continue;
            }
            if value.is_empty() {
                return Err(format!("assignment '{key}=' has no value"));
            }
            return Ok(Some(value.to_string()));
        }
        Ok(None)
    }

    pub(super) fn normalize_assignment_spacing(line: &str) -> String {
        let mut out = String::with_capacity(line.len());
        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch.is_whitespace() {
                while chars.peek().is_some_and(|next| next.is_whitespace()) {
                    chars.next();
                }
                if chars.peek() == Some(&'=') || out.ends_with('=') {
                    continue;
                }
                out.push(' ');
                continue;
            }
            out.push(ch);
        }
        out
    }

    pub(super) fn native_xyce_level2_diode_instance_param(name: &str, value: Value) -> bool {
        value.is_finite()
            && match name.to_ascii_uppercase().as_str() {
                "AREA" | "M" | "MULT" => value > 0.0,
                "PJ" => value >= 0.0,
                "TEMP" | "DTEMP" => value > -273.15,
                _ => false,
            }
    }

    pub(super) fn capacitor_uses_solution_dependent_value(
        netlist: &Netlist,
        element_name: &str,
    ) -> bool {
        Self::find_capacitor_element(netlist, element_name).is_some_and(|element| {
            matches!(
                &element.kind,
                ElementKind::Capacitor {
                    value,
                    value_expr: Some(_),
                    ..
                } if !value.is_finite()
            )
        })
    }

    pub(super) fn direct_branch_current_control(expression: &str) -> Option<String> {
        let normalized = Self::normalize_probe(expression);
        Self::parse_current_probe(&normalized)
    }

    #[cfg(test)]
    pub(super) fn expression_depends_on_solution_quantity(expression: &Expr) -> bool {
        match expression {
            Expr::NodeVoltage(_) | Expr::BranchCurrent(_) => true,
            Expr::Unary { operand, .. } => Self::expression_depends_on_solution_quantity(operand),
            Expr::Binary { left, right, .. } => {
                Self::expression_depends_on_solution_quantity(left)
                    || Self::expression_depends_on_solution_quantity(right)
            }
            Expr::Function { args, .. } => args
                .iter()
                .any(Self::expression_depends_on_solution_quantity),
            Expr::Const(_)
            | Expr::StringLiteral(_)
            | Expr::LookupTable(_)
            | Expr::Time
            | Expr::Frequency
            | Expr::Temperature
            | Expr::ThermalVoltage
            | Expr::Gmin => false,
        }
    }

    pub(super) fn expression_depends_on_ac_runtime_quantity(expression: &Expr) -> bool {
        match expression {
            Expr::Time | Expr::Frequency => true,
            Expr::Unary { operand, .. } => Self::expression_depends_on_ac_runtime_quantity(operand),
            Expr::Binary { left, right, .. } => {
                Self::expression_depends_on_ac_runtime_quantity(left)
                    || Self::expression_depends_on_ac_runtime_quantity(right)
            }
            Expr::Function { args, .. } => args
                .iter()
                .any(Self::expression_depends_on_ac_runtime_quantity),
            Expr::Const(_)
            | Expr::StringLiteral(_)
            | Expr::LookupTable(_)
            | Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::Temperature
            | Expr::ThermalVoltage
            | Expr::Gmin => false,
        }
    }

    pub(super) fn expression_depends_on_frequency(expression: &Expr) -> bool {
        match expression {
            Expr::Frequency => true,
            Expr::Unary { operand, .. } => Self::expression_depends_on_frequency(operand),
            Expr::Binary { left, right, .. } => {
                Self::expression_depends_on_frequency(left)
                    || Self::expression_depends_on_frequency(right)
            }
            Expr::Function { args, .. } => args.iter().any(Self::expression_depends_on_frequency),
            Expr::Const(_)
            | Expr::StringLiteral(_)
            | Expr::LookupTable(_)
            | Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::Time
            | Expr::Temperature
            | Expr::ThermalVoltage
            | Expr::Gmin => false,
        }
    }

    pub(super) fn passive_value_expression_depends_on_runtime_quantity(expression: &Expr) -> bool {
        match expression {
            Expr::Const(_) | Expr::StringLiteral(_) | Expr::Temperature => false,
            Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::LookupTable(_)
            | Expr::Time
            | Expr::Frequency
            | Expr::ThermalVoltage
            | Expr::Gmin => true,
            Expr::Unary { operand, .. } => {
                Self::passive_value_expression_depends_on_runtime_quantity(operand)
            }
            Expr::Binary { left, right, .. } => {
                Self::passive_value_expression_depends_on_runtime_quantity(left)
                    || Self::passive_value_expression_depends_on_runtime_quantity(right)
            }
            Expr::Function { args, .. } => args
                .iter()
                .any(Self::passive_value_expression_depends_on_runtime_quantity),
        }
    }

    pub(super) fn elements_use_ekv3_level301_mosfet(
        elements: &[rspice_core::netlist::Element],
        models: &[rspice_core::netlist::ModelDef],
        scoped_models: &[rspice_core::netlist::ModelDef],
    ) -> bool {
        elements.iter().any(|element| {
            let ElementKind::Mosfet {
                model,
                instance_params,
                ..
            } = &element.kind
            else {
                return false;
            };
            Self::find_model_or_binned(scoped_models, model, instance_params)
                .or_else(|| Self::find_model_or_binned(models, model, instance_params))
                .is_some_and(Self::model_is_ekv3_level301)
        })
    }

    pub(super) fn elements_use_native_vbic_bjt(
        elements: &[rspice_core::netlist::Element],
        models: &[rspice_core::netlist::ModelDef],
        scoped_models: &[rspice_core::netlist::ModelDef],
    ) -> bool {
        elements.iter().any(|element| {
            let ElementKind::Bjt { model, .. } = &element.kind else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_native_vbic_bjt)
        })
    }

    pub(super) fn bin_range_contains(
        value: Option<Value>,
        min: Option<Value>,
        max: Option<Value>,
    ) -> bool {
        if min.is_none() && max.is_none() {
            return true;
        }
        let Some(value) = value else {
            return false;
        };
        min.is_none_or(|min| value >= min) && max.is_none_or(|max| value <= max)
    }

    pub(super) fn bin_range_size(min: Option<Value>, max: Option<Value>) -> Value {
        match (min, max) {
            (Some(min), Some(max)) => max - min,
            (Some(_), None) | (None, Some(_)) => Value::MAX / 4.0,
            (None, None) => 0.0,
        }
    }

    pub(super) fn probe_names_live_measurement(
        probe: &str,
        netlist: &Netlist,
        scalar_analysis: &str,
        continuous_analysis: &str,
    ) -> bool {
        let candidate = Self::print_expression_inner(probe).unwrap_or(probe).trim();
        netlist.measurements.iter().any(|measurement| {
            measurement.name.eq_ignore_ascii_case(candidate)
                && (measurement.analysis.eq_ignore_ascii_case(scalar_analysis)
                    || measurement
                        .analysis
                        .eq_ignore_ascii_case(continuous_analysis))
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Equation { .. }
                        | rspice_core::analysis::MeasureType::Find { .. }
                        | rspice_core::analysis::MeasureType::Derivative { .. }
                        | rspice_core::analysis::MeasureType::When { .. }
                        | rspice_core::analysis::MeasureType::Delay { .. }
                )
        })
    }

    pub(super) fn resistor_uses_branch_form(netlist: &Netlist, resistance: Value) -> bool {
        resistance.is_finite() && resistance.abs() <= Self::resistor_branch_form_tolerance(netlist)
    }

    pub(super) fn indexed_dc_voltage_named(
        node_voltages: &HashMap<String, Value>,
        netlist: &Netlist,
        node_name: &str,
    ) -> Option<Value> {
        if netlist.ground_policy().is_ground(node_name) {
            return Some(0.0);
        }
        Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| node_voltages.get(&candidate.to_ascii_lowercase()).copied())
    }

    pub(super) fn hb_result_to_transient_result(
        result: &rspice_core::analysis::HbResult,
    ) -> TransientResult {
        let sample_count = 2 * result.num_harmonics + 1;
        let period = result.fundamental_freq.recip();
        let time = (0..sample_count)
            .map(|sample| sample as Value * period / sample_count as Value)
            .collect::<Vec<_>>();
        let voltages = result
            .spectral_voltages
            .iter()
            .map(|node| {
                time.iter()
                    .map(|time| {
                        let mut value = node
                            .coefficients
                            .first()
                            .map(|value| value.re)
                            .unwrap_or(0.0);
                        for (harmonic, coefficient) in
                            node.coefficients.iter().copied().enumerate().skip(1)
                        {
                            let angle = std::f64::consts::TAU
                                * harmonic as Value
                                * result.fundamental_freq
                                * time;
                            value += (coefficient * Complex64::from_polar(1.0, angle)).re;
                        }
                        value
                    })
                    .collect()
            })
            .collect::<Vec<_>>();
        TransientResult {
            time,
            step_sizes: vec![0.0; voltages.first().map_or(0, Vec::len)],
            voltages,
            branch_currents: Vec::new(),
            num_nodes: result.spectral_voltages.len(),
            node_names: result
                .spectral_voltages
                .iter()
                .map(|node| node.node_name.clone())
                .collect(),
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
        }
    }

    pub(super) fn static_transient_device_parameter_value(
        netlist: &Netlist,
        token: &str,
    ) -> Result<Value, String> {
        let normalized = Self::normalize_probe(token);
        let (element_name, parameter) = Self::parse_device_parameter_probe(&normalized)
            .ok_or_else(|| format!("invalid transient device parameter token '{token}'"))?;
        match parameter.as_str() {
            "r" => Self::effective_resistor_value(netlist, &element_name)?.ok_or_else(|| {
                format!("resistor parameter probe '{token}' has no finite resistance")
            }),
            "c" => Self::effective_capacitor_value(netlist, &element_name).ok_or_else(|| {
                format!("capacitor parameter probe '{token}' has no finite capacitance")
            }),
            "l" => Self::effective_inductor_value(netlist, &element_name).ok_or_else(|| {
                format!("inductor parameter probe '{token}' has no finite inductance")
            }),
            "temp" => Self::resistor_temperature_value(netlist, &element_name)?.ok_or_else(|| {
                format!("resistor parameter probe '{token}' has no finite temperature")
            }),
            _ => Err(format!(
                "device parameter probe '{token}' is not supported in stateful transient output"
            )),
        }
    }

    pub(super) fn stateful_tran_probe_waveform(
        probe: &str,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<Option<Vec<Value>>, String> {
        let Some(mut runtime) = Self::stateful_tran_print_expression(probe, netlist)? else {
            return Ok(None);
        };
        let mut values = Vec::with_capacity(result.time.len());
        for &time in &result.time {
            values.push(Self::evaluate_stateful_tran_print_expression(
                &mut runtime,
                netlist,
                result,
                time,
            )?);
        }
        Ok(Some(values))
    }

    pub(super) fn derived_tran_probe_waveform(
        probe: &str,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<Option<Vec<Value>>, String> {
        let traces = rspice_core::analysis::evaluate_tran_equation_measurements(netlist, result)?;
        if let Some(trace) = traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case(probe))
        {
            return Ok(Some(trace.values.clone()));
        }

        if let Some(expression) = Self::print_expression_inner(probe) {
            let upper = expression.to_ascii_uppercase();
            let references_measure = traces.iter().any(|trace| {
                let name = trace.name.to_ascii_uppercase();
                upper.match_indices(&name).any(|(start, _)| {
                    let end = start + name.len();
                    let identifier = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_';
                    (start == 0 || !identifier(upper.as_bytes()[start - 1]))
                        && (end == upper.len() || !identifier(upper.as_bytes()[end]))
                })
            });
            if references_measure {
                let mut values = Vec::with_capacity(result.time.len());
                for (row, &time) in result.time.iter().enumerate() {
                    let mut context = Self::print_tran_eval_context(netlist, time);
                    for trace in &traces {
                        context.set(&trace.name, trace.values[row]);
                    }
                    let mut call_value = |call: &str| {
                        Self::evaluate_atomic_tran_probe(
                            &Self::normalize_probe(call),
                            netlist,
                            result,
                            time,
                        )
                    };
                    values.push(Self::evaluate_print_expression_with_probe_calls(
                        expression,
                        context,
                        &mut call_value,
                    )?);
                }
                return Ok(Some(values));
            }
        }

        Self::stateful_tran_probe_waveform(probe, netlist, result)
    }

    pub(super) fn expression_contains_sdt(expression: &Expr) -> bool {
        match expression {
            Expr::Function { func, args } => {
                matches!(func, rspice_core::expr::Function::Sdt)
                    || args.iter().any(Self::expression_contains_sdt)
            }
            Expr::Unary { operand, .. } => Self::expression_contains_sdt(operand),
            Expr::Binary { left, right, .. } => {
                Self::expression_contains_sdt(left) || Self::expression_contains_sdt(right)
            }
            Expr::Const(_)
            | Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::StringLiteral(_)
            | Expr::LookupTable(_)
            | Expr::Time
            | Expr::Frequency
            | Expr::Temperature
            | Expr::ThermalVoltage
            | Expr::Gmin => false,
        }
    }

    pub(super) fn interpolate_transient_waveform_at(
        times: &[Value],
        values: &[Value],
        time: Value,
    ) -> Result<Value, String> {
        if times.len() != values.len() {
            return Err(format!(
                "transient waveform has {} sample(s) for {} time point(s)",
                values.len(),
                times.len()
            ));
        }
        let Some((&first_time, &last_time)) = times.first().zip(times.last()) else {
            return Err("transient waveform has no samples".to_string());
        };
        let scale = first_time.abs().max(last_time.abs()).max(time.abs());
        let edge_tol = (1.0e-12 * scale)
            .max(64.0 * f64::EPSILON * scale)
            .max(1.0e-30);
        if time < first_time - edge_tol || time > last_time + edge_tol {
            return Err(format!(
                "requested transient sample time {time:e} is outside simulated range [{first_time:e}, {last_time:e}]"
            ));
        }
        if time <= first_time + edge_tol {
            return Ok(values[0]);
        }
        if time >= last_time - edge_tol {
            return Ok(*values.last().expect("non-empty waveform"));
        }

        let upper = times.partition_point(|sample| *sample < time);
        if upper == 0 || upper >= times.len() {
            return Err(format!(
                "requested transient sample time {time:e} is outside interpolation brackets"
            ));
        }
        let lower = upper - 1;
        let t0 = times[lower];
        let t1 = times[upper];
        if (time - t0).abs() <= edge_tol {
            return Ok(values[lower]);
        }
        if (time - t1).abs() <= edge_tol {
            return Ok(values[upper]);
        }
        let dt = t1 - t0;
        if !dt.is_finite() || dt <= 0.0 {
            return Err(format!(
                "invalid transient interpolation interval [{t0:e}, {t1:e}]"
            ));
        }
        let alpha = (time - t0) / dt;
        Ok(values[lower] + alpha * (values[upper] - values[lower]))
    }

    pub(super) fn central_difference_derivative<F>(
        center: Value,
        mut eval_at: F,
    ) -> Result<Value, String>
    where
        F: FnMut(Value) -> Result<Value, String>,
    {
        let scale = center.abs().max(1.0);
        let mut last_finite = None;
        for relative_step in [1.0e-4, 3.0e-5, 1.0e-5, 3.0e-6, 1.0e-6] {
            let step = scale * relative_step;
            let hi = eval_at(center + step)?;
            let lo = eval_at(center - step)?;
            let derivative = (hi - lo) / (2.0 * step);
            if derivative.is_finite() {
                last_finite = Some(derivative);
            }
        }
        last_finite.ok_or_else(|| "DDX derivative evaluated to a non-finite value".to_string())
    }

    pub(super) fn split_top_level_args(input: &str) -> Result<Vec<String>, String> {
        let mut args = Vec::new();
        let mut start = 0usize;
        let mut paren_depth = 0usize;
        let mut brace_depth = 0usize;

        for (index, ch) in input.char_indices() {
            match ch {
                '(' => paren_depth += 1,
                ')' => {
                    paren_depth = paren_depth.checked_sub(1).ok_or_else(|| {
                        format!("unbalanced ')' while parsing function arguments '{input}'")
                    })?;
                }
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth = brace_depth.checked_sub(1).ok_or_else(|| {
                        format!("unbalanced '}}' while parsing function arguments '{input}'")
                    })?;
                }
                ',' if paren_depth == 0 && brace_depth == 0 => {
                    args.push(input[start..index].trim().to_string());
                    start = index + ch.len_utf8();
                }
                _ => {}
            }
        }

        if paren_depth != 0 || brace_depth != 0 {
            return Err(format!(
                "unbalanced delimiters while parsing function arguments '{input}'"
            ));
        }
        args.push(input[start..].trim().to_string());
        if args.iter().any(|arg| arg.is_empty()) {
            return Err(format!("empty function argument in '{input}'"));
        }
        Ok(args)
    }

    pub(super) fn braced_expression_is_atomic_real_probe(
        normalized_expression: &str,
        netlist: &Netlist,
    ) -> bool {
        Self::parse_device_parameter_probe(normalized_expression).is_some()
            || Self::bare_device_parameter_probe_is_atomic_real_probe(
                netlist,
                normalized_expression,
            )
            || Self::parse_device_operating_point_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_lead_current_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_tran_voltage_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_current_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_power_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
    }

    pub(super) fn bare_device_parameter_probe_is_atomic_real_probe(
        netlist: &Netlist,
        probe: &str,
    ) -> bool {
        let Some(probe_name) = Self::parse_bare_device_parameter_probe(probe) else {
            return false;
        };
        netlist.params.get_complex(&probe_name).is_none()
            && Self::bare_device_parameter_probe_is_supported(netlist, &probe_name)
    }

    pub(super) fn braced_expression_is_atomic_ac_probe(
        normalized_expression: &str,
        netlist: &Netlist,
    ) -> bool {
        Self::parse_device_parameter_probe(normalized_expression).is_some()
            || Self::bare_device_parameter_probe_is_atomic_ac_probe(netlist, normalized_expression)
            || Self::parse_device_operating_point_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_lead_current_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_ac_voltage_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_ac_current_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_power_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
    }

    pub(super) fn bare_device_parameter_probe_is_atomic_ac_probe(
        netlist: &Netlist,
        probe: &str,
    ) -> bool {
        let Some(probe_name) = Self::parse_bare_device_parameter_probe(probe) else {
            return false;
        };
        netlist.params.get_complex(&probe_name).is_none()
            && Self::bare_device_parameter_probe_is_supported(netlist, &probe_name)
    }

    pub(super) fn probe_call_covers_entire_expression(expression: &str) -> bool {
        let Some(open_index) = expression.find('(') else {
            return false;
        };
        if open_index == 0 || !expression.is_char_boundary(open_index) {
            return false;
        }
        Self::matching_parenthesis_index(expression, open_index)
            .is_ok_and(|close_index| close_index + 1 == expression.len())
    }

    pub(super) fn matching_parenthesis_index(
        expression: &str,
        open_index: usize,
    ) -> Result<usize, String> {
        if !expression[open_index..].starts_with('(') {
            return Err(format!(
                "internal error: expected '(' in .PRINT expression '{expression}'"
            ));
        }

        let mut depth = 0usize;
        for (relative_index, ch) in expression[open_index..].char_indices() {
            let absolute_index = open_index + relative_index;
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return Ok(absolute_index);
                    }
                }
                _ => {}
            }
        }

        Err(format!(
            "unterminated probe call in .PRINT expression '{{{expression}}}'"
        ))
    }

    pub(super) fn bare_device_parameter_probe_is_supported(netlist: &Netlist, probe: &str) -> bool {
        Self::find_bare_device_parameter_element(netlist, probe).is_some()
    }

    pub(super) fn xyce_device_operating_point_value(
        entry: &rspice_core::circuit::DeviceOpEntry,
        parameter: &str,
    ) -> Option<Value> {
        if matches!(entry.device_kind, "BSIM3" | "BSIM4") {
            return Self::xyce_bsim_device_store_value(entry, parameter);
        }

        Self::device_op_entry_param(entry, parameter)
    }

    pub(super) fn xyce_bsim_device_store_value(
        entry: &rspice_core::circuit::DeviceOpEntry,
        parameter: &str,
    ) -> Option<Value> {
        let raw = if parameter == "vdsat" && entry.device_kind == "BSIM4" {
            Self::device_op_entry_param(entry, "output_vdsat")
                .or_else(|| Self::device_op_entry_param(entry, parameter))?
        } else {
            Self::device_op_entry_param(entry, parameter)?
        };
        let vds = Self::device_op_entry_param(entry, "vds").unwrap_or(0.0);
        if vds >= 0.0 {
            return Some(raw);
        }

        match parameter {
            // Xyce stores BSIM3/BSIM4 gm after the same inverse-mode sign
            // swap it applies to the MNA stamp.
            "gm" => Some(-raw),
            // Xyce's Vds/Vgs/Vbs store nodes are the mode-frame branch
            // voltages: Vds, Vgs, Vbs in normal mode; -Vds, Vgd, Vbd in
            // inverse mode.
            "vds" => Some(-vds),
            "vgs" => Some(raw - vds),
            "vbs" => Some(raw - vds),
            _ => Some(raw),
        }
    }

    pub(super) fn device_op_entry_param(
        entry: &rspice_core::circuit::DeviceOpEntry,
        parameter: &str,
    ) -> Option<Value> {
        entry
            .params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
            .map(|(_, value)| *value)
    }

    pub(super) fn canonical_device_op_parameter(parameter: &str) -> Option<&'static str> {
        match parameter.trim().to_ascii_lowercase().as_str() {
            "id" | "ids" => Some("id"),
            "vgs" => Some("vgs"),
            "vds" => Some("vds"),
            "vbs" => Some("vbs"),
            "vth" | "vto" => Some("vth"),
            "vdsat" | "vdssat" => Some("vdsat"),
            "gm" => Some("gm"),
            "gds" => Some("gds"),
            "gmb" | "gmbs" => Some("gmb"),
            "cd" => Some("cd"),
            "m" => Some("m"),
            "h" => Some("h"),
            "b" => Some("b"),
            _ => None,
        }
    }

    pub(super) fn lead_current_probe_is_omitted_empty_wildcard(
        netlist: &Netlist,
        probe: &XyceLeadCurrentProbe,
    ) -> bool {
        probe.element_name == "*"
            && !Self::netlist_has_lead_current_wildcard_match(netlist, probe.terminal)
    }

    pub(super) fn element_matches_lead_current_wildcard(
        element: &rspice_core::netlist::Element,
        terminal: XyceLeadCurrentTerminal,
    ) -> bool {
        match terminal {
            XyceLeadCurrentTerminal::Drain | XyceLeadCurrentTerminal::Gate => matches!(
                element.kind,
                ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. }
            ),
            XyceLeadCurrentTerminal::Source => matches!(
                element.kind,
                ElementKind::Mosfet { .. }
                    | ElementKind::Jfet { .. }
                    | ElementKind::Mesfet { .. }
                    | ElementKind::Bjt { .. }
            ),
            XyceLeadCurrentTerminal::Bulk => matches!(
                element.kind,
                ElementKind::Mosfet { .. } | ElementKind::Bjt { .. }
            ),
            XyceLeadCurrentTerminal::Collector | XyceLeadCurrentTerminal::Emitter => {
                matches!(element.kind, ElementKind::Bjt { .. })
            }
        }
    }

    pub(super) fn native_relational_diode_instance_param(name: &str, value: Value) -> bool {
        value.is_finite()
            && match name.to_ascii_uppercase().as_str() {
                "AREA" | "M" => value > 0.0,
                "TEMP" => value > -273.15,
                _ => false,
            }
    }

    pub(super) fn native_relational_mos3_instance_param(name: &str, value: Value) -> bool {
        value.is_finite()
            && value > 0.0
            && matches!(name.to_ascii_uppercase().as_str(), "L" | "W" | "M" | "NF")
    }

    pub(super) fn native_relational_mos3_effective_geometry_is_valid(
        model: &rspice_core::netlist::ModelDef,
        instance_params: &[(String, Value)],
    ) -> bool {
        let Some(length) = Self::numeric_param_value(instance_params, "L")
            .or_else(|| Self::numeric_param_value(&model.params, "L"))
        else {
            return false;
        };
        let Some(width) = Self::numeric_param_value(instance_params, "W")
            .or_else(|| Self::numeric_param_value(&model.params, "W"))
        else {
            return false;
        };
        let lateral_diffusion = Self::numeric_param_value(&model.params, "LD").unwrap_or(0.0);
        let length_adjust = Self::numeric_param_value(&model.params, "XL").unwrap_or(0.0);
        let width_narrow = Self::numeric_param_value(&model.params, "WD").unwrap_or(0.0);
        let width_adjust = Self::numeric_param_value(&model.params, "XW").unwrap_or(0.0);
        let effective_length = length - 2.0 * lateral_diffusion + length_adjust;
        let effective_width = width - 2.0 * width_narrow + width_adjust;
        effective_length.is_finite()
            && effective_length > 0.0
            && effective_width.is_finite()
            && effective_width > 0.0
    }

    pub(super) fn native_transient_uses_standard_startup(netlist: &Netlist) -> bool {
        !Self::tran_uses_uic(netlist)
            && !netlist
                .analyses
                .iter()
                .any(|analysis| matches!(analysis, AnalysisCommand::Temp { .. }))
            && netlist.initial_conditions.is_empty()
            && netlist.node_sets.is_empty()
            && netlist
                .options
                .temp
                .is_none_or(|temp| temp.is_finite() && (temp - 27.0).abs() <= 1.0e-9)
            && netlist
                .options
                .tnom
                .is_none_or(|tnom| tnom.is_finite() && (tnom - 27.0).abs() <= 1.0e-9)
            && netlist.elements.iter().all(|element| match &element.kind {
                ElementKind::Capacitor {
                    initial_voltage, ..
                } => initial_voltage.is_none(),
                ElementKind::Inductor {
                    initial_current, ..
                } => initial_current.is_none(),
                ElementKind::VSwitch { initial_state, .. }
                | ElementKind::ISwitch { initial_state, .. }
                | ElementKind::GenericSwitch { initial_state, .. } => initial_state.is_none(),
                _ => true,
            })
    }

    pub(super) fn native_transient_vbic_pwl_is_valid(
        points: &[(Value, Value)],
        delay: Value,
        repeat_from: Option<Value>,
    ) -> bool {
        delay.is_finite()
            && delay >= 0.0
            && repeat_from.is_none()
            && points.len() >= 2
            && points
                .iter()
                .all(|(time, value)| time.is_finite() && value.is_finite())
            && points.windows(2).all(|window| window[1].0 > window[0].0)
    }

    pub(super) fn native_transient_bsim4_capacitor_instance_params_are_valid(
        params: &[(String, Value)],
    ) -> bool {
        let mut names = BTreeSet::new();
        let mut has_length = false;
        let mut has_width = false;
        params.iter().all(|(name, value)| {
            let key = name.to_ascii_uppercase();
            if !value.is_finite() || !names.insert(key.clone()) {
                return false;
            }
            match key.as_str() {
                "L" => {
                    has_length = *value > 0.0;
                    has_length
                }
                "W" => {
                    has_width = *value > 0.0;
                    has_width
                }
                "M" | "AD" | "AS" | "PD" | "PS" => *value > 0.0,
                _ => false,
            }
        }) && has_length
            && has_width
    }

    pub(super) fn native_transient_bsim3_capacitor_instance_params_are_valid(
        params: &[(String, Value)],
    ) -> bool {
        let mut names = BTreeSet::new();
        let mut has_length = false;
        let mut has_width = false;
        params.iter().all(|(name, value)| {
            let key = name.to_ascii_uppercase();
            if !value.is_finite() || !names.insert(key.clone()) {
                return false;
            }
            match key.as_str() {
                "L" => {
                    has_length = *value > 0.0;
                    has_length
                }
                "W" => {
                    has_width = *value > 0.0;
                    has_width
                }
                "M" | "AD" | "AS" | "PD" | "PS" => *value > 0.0,
                _ => false,
            }
        }) && has_length
            && has_width
    }

    pub(super) fn native_absolute_transient_w_l_instance_params(
        params: &[(String, Value)],
    ) -> bool {
        if params.len() != 2
            || !params.iter().all(|(name, value)| {
                value.is_finite()
                    && *value > 0.0
                    && matches!(name.to_ascii_uppercase().as_str(), "W" | "L")
            })
        {
            return false;
        }
        params
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case("W"))
            .count()
            == 1
            && params
                .iter()
                .filter(|(name, _)| name.eq_ignore_ascii_case("L"))
                .count()
                == 1
    }

    pub(super) fn native_absolute_transient_legacy_diode_instance_param(
        name: &str,
        value: Value,
    ) -> bool {
        value.is_finite()
            && match name.to_ascii_uppercase().as_str() {
                "AREA" | "M" | "MULT" => value > 0.0,
                "PJ" => value >= 0.0,
                "TEMP" | "DTEMP" => value > -273.15,
                _ => false,
            }
    }

    pub(super) fn native_transient_level1_mos_instance_params_are_valid(
        params: &[(String, Value)],
    ) -> bool {
        let mut names = BTreeSet::new();
        let mut has_length = false;
        let mut has_width = false;
        for (name, value) in params {
            if !value.is_finite() || !names.insert(name.to_ascii_uppercase()) {
                return false;
            }
            match name.to_ascii_uppercase().as_str() {
                "L" => {
                    has_length = *value > 0.0;
                }
                "W" => {
                    has_width = *value > 0.0;
                }
                "M" => {
                    if *value <= 0.0 {
                        return false;
                    }
                }
                "AD" | "AS" | "PD" | "PS" | "NRD" | "NRS" => {
                    if *value < 0.0 {
                        return false;
                    }
                }
                "TEMP" | "DTEMP" => {
                    if *value <= -273.15 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        has_length && has_width
    }

    pub(super) fn native_transient_ekv26_instance_params_are_valid(
        params: &[(String, Value)],
    ) -> bool {
        let mut names = BTreeSet::new();
        let mut has_length = false;
        let mut has_width = false;
        for (name, value) in params {
            if !value.is_finite() {
                return false;
            }
            let normalized = match name.to_ascii_uppercase().as_str() {
                "L" | "LENGTH" => {
                    has_length = *value > 0.0;
                    "L"
                }
                "W" | "WIDTH" => {
                    has_width = *value > 0.0;
                    "W"
                }
                "M" | "MULT" => {
                    if *value <= 0.0 {
                        return false;
                    }
                    "M"
                }
                "NS" => {
                    if *value <= 0.0 {
                        return false;
                    }
                    "NS"
                }
                "AS" => {
                    if *value < 0.0 {
                        return false;
                    }
                    "AS"
                }
                "AD" => {
                    if *value < 0.0 {
                        return false;
                    }
                    "AD"
                }
                "PS" => {
                    if *value < 0.0 {
                        return false;
                    }
                    "PS"
                }
                "PD" => {
                    if *value < 0.0 {
                        return false;
                    }
                    "PD"
                }
                "TEMP" => {
                    if *value <= -273.15 {
                        return false;
                    }
                    "TEMP"
                }
                "DTEMP" => "DTEMP",
                _ => return false,
            };
            if !names.insert(normalized) {
                return false;
            }
        }
        has_length && has_width
    }

    pub(super) fn elements_device_is_native_legacy_bjt(
        elements: &[rspice_core::netlist::Element],
        models: &[rspice_core::netlist::ModelDef],
        scoped_models: &[rspice_core::netlist::ModelDef],
        instance_name: &str,
    ) -> bool {
        elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Bjt { model, .. } = &element.kind else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_native_legacy_bjt)
        })
    }

    pub(super) fn elements_device_is_native_static_ac_legacy_npn_bjt(
        elements: &[rspice_core::netlist::Element],
        models: &[rspice_core::netlist::ModelDef],
        scoped_models: &[rspice_core::netlist::ModelDef],
        instance_name: &str,
    ) -> bool {
        elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Bjt {
                model,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                return false;
            };
            let topology_is_qualified = match element.nodes.as_slice() {
                [_, _, _] => true,
                [_, _, _, substrate] => Self::node_name_is_ground(substrate),
                _ => false,
            };
            if !topology_is_qualified || !instance_params.is_empty() || !deferred_params.is_empty()
            {
                return false;
            }
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_native_static_ac_legacy_npn_bjt)
        })
    }

    pub(super) fn xyce_level2_native_diode_param(name: &str) -> bool {
        matches!(
            name.to_ascii_uppercase().as_str(),
            "LEVEL"
                | "IS"
                | "JS"
                | "N"
                | "RS"
                | "KF"
                | "AF"
                | "BV"
                | "VB"
                | "IBV"
                | "IKF"
                | "IK"
                | "IKR"
                | "ISR"
                | "NR"
                | "CJO"
                | "CJ0"
                | "CJ"
                | "VJ"
                | "M"
                | "TT"
                | "FC"
                | "JSW"
                | "NS"
                | "CJSW"
                | "CJP"
                | "PHP"
                | "VJSW"
                | "MJSW"
                | "FCS"
                | "NBV"
                | "XTI"
                | "EG"
                | "TNOM"
                | "TBV1"
                | "TBV2"
        )
    }

    pub(super) fn native_ac_bsim3_instance_params_are_valid(params: &[(String, Value)]) -> bool {
        let mut names = BTreeSet::new();
        params.iter().all(|(name, value)| {
            value.is_finite()
                && *value > 0.0
                && matches!(name.to_ascii_uppercase().as_str(), "L" | "W")
                && names.insert(name.to_ascii_uppercase())
        }) && names.contains("L")
            && names.contains("W")
    }

    pub(super) fn native_ac_b3soi_instance_params_are_valid(params: &[(String, Value)]) -> bool {
        let mut names = BTreeSet::new();
        params.iter().all(|(name, value)| {
            value.is_finite()
                && *value > 0.0
                && matches!(name.to_ascii_uppercase().as_str(), "L" | "W")
                && names.insert(name.to_ascii_uppercase())
        }) && names.contains("L")
            && names.contains("W")
    }

    pub(super) fn native_ac_bsim4_instance_params_are_valid(params: &[(String, Value)]) -> bool {
        let mut names = BTreeSet::new();
        params.iter().all(|(name, value)| {
            value.is_finite()
                && *value > 0.0
                && matches!(name.to_ascii_uppercase().as_str(), "L" | "W")
                && names.insert(name.to_ascii_uppercase())
        }) && names.contains("L")
            && names.contains("W")
    }

    pub(super) fn native_ac_classic_mos_instance_params_are_valid(
        params: &[(String, Value)],
    ) -> bool {
        let mut names = BTreeSet::new();
        params.iter().all(|(name, value)| {
            value.is_finite()
                && *value > 0.0
                && matches!(name.to_ascii_uppercase().as_str(), "L" | "W")
                && names.insert(name.to_ascii_uppercase())
        }) && names.contains("L")
            && names.contains("W")
    }

    pub(super) fn elements_device_is_native_classic_jfet(
        elements: &[rspice_core::netlist::Element],
        models: &[rspice_core::netlist::ModelDef],
        scoped_models: &[rspice_core::netlist::ModelDef],
        instance_name: &str,
    ) -> bool {
        elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Jfet { model, .. } = &element.kind else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_native_classic_jfet)
        })
    }

    pub(super) fn elements_device_is_native_b3soi_mosfet(
        elements: &[rspice_core::netlist::Element],
        models: &[rspice_core::netlist::ModelDef],
        scoped_models: &[rspice_core::netlist::ModelDef],
        instance_name: &str,
    ) -> bool {
        elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Mosfet {
                model,
                instance_params,
                ..
            } = &element.kind
            else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(|model| Self::model_is_native_b3soi_mosfet(model, instance_params))
        })
    }

    pub(super) fn numeric_param_value(params: &[(String, Value)], name: &str) -> Option<Value> {
        params
            .iter()
            .rev()
            .find(|(param_name, _)| param_name.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
    }

    pub(super) fn device_instance_names_match(lhs: &str, rhs: &str) -> bool {
        Self::normalize_device_instance_name(lhs) == Self::normalize_device_instance_name(rhs)
    }

    pub(super) fn normalize_device_instance_name(name: &str) -> String {
        Self::normalize_probe(name).replace(':', ".")
    }

    pub(super) fn node_lookup_candidates(netlist: &Netlist, node_name: &str) -> Vec<String> {
        Engine::node_lookup_candidates(netlist, node_name)
    }

    pub(super) fn add_runtime_scalar_parameter_bindings(
        context: &mut rspice_core::netlist::ParamContext,
    ) {
        rspice_core::netlist::expr::materialize_available_parameter_expressions(context);
    }

    pub(super) fn add_runtime_file_table_parameter_bindings(
        netlist: &Netlist,
        context: &mut rspice_core::netlist::ParamContext,
        time: Value,
    ) {
        let mut expressions = context.all_parameter_expressions();
        expressions.extend(context.all_global_expressions());
        expressions.sort_by(|left, right| left.0.cmp(&right.0));
        expressions.dedup_by(|left, right| left.0.eq_ignore_ascii_case(&right.0));

        for (name, expression) in expressions {
            let Ok(prepared) = prepare_behavioral_expression(&expression, context) else {
                continue;
            };
            let Ok(ast) = parse_expression_strict(&prepared) else {
                continue;
            };
            if !Self::strict_expression_contains_file_table(&ast) {
                continue;
            }
            let Ok(ast) = rspice_core::expr::resolve_file_lookup_functions_with_limits(
                ast,
                netlist.source_path.as_deref(),
                rspice_core::resource::ResourceLimits::default(),
            ) else {
                continue;
            };
            let program = compile(&ast);
            let temperature = context.get("TEMP").unwrap_or(27.0);
            let gmin = context.get("GMIN").unwrap_or(rspice_core::constants::GMIN);
            let eval_context = Context::transient(&[], &[], time)
                .with_temperature(temperature)
                .with_gmin(gmin)
                .with_expression_dialect(ExpressionDialect::Xyce);
            let value = Vm::new().execute(&program, &eval_context);
            if value.is_finite() {
                context.set(&name, value);
            }
        }
    }

    pub(super) fn add_resistor_parameter_bindings(
        netlist: &Netlist,
        context: &mut rspice_core::netlist::ParamContext,
    ) {
        for element in &netlist.elements {
            let ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                ..
            } = &element.kind
            else {
                continue;
            };
            if netlist.params.get(&element.name).is_some() {
                continue;
            }

            let Some(resistance) = Self::resistor_parameter_r_value_from_parts(
                *value,
                value_expr.as_deref(),
                model.as_deref(),
                instance_params,
                context,
            ) else {
                continue;
            };
            context.set(&element.name, resistance);
        }
    }

    pub(super) fn active_temperature_c(
        netlist: &Netlist,
        dc: Option<&XyceDcSweep>,
        sweep_point: Option<XyceDcSweepPoint>,
    ) -> Value {
        if let (Some(dc), Some(sweep_point)) = (dc, sweep_point) {
            if Self::is_temperature_name(&dc.source) {
                return sweep_point.primary;
            }
            if let Some(sweep2) = &dc.sweep2
                && Self::is_temperature_name(&sweep2.source)
                && let Some(secondary) = sweep_point.secondary
            {
                return secondary;
            }
        }

        Self::netlist_temperature_c(netlist)
    }

    pub(super) fn thermal_voltage_celsius(temp_c: Value) -> Value {
        rspice_core::constants::thermal_voltage(rspice_core::constants::celsius_to_kelvin(temp_c))
    }

    pub(super) fn resistor_voltage_drop(
        netlist: &Netlist,
        result: &rspice_core::SimulationResult,
        resistor_name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_resistor_element(netlist, resistor_name)
            .ok_or_else(|| format!("resistor '{}' not found", resistor_name))?;
        let node_pos = element
            .nodes
            .first()
            .ok_or_else(|| format!("resistor '{}' has no positive node", resistor_name))?;
        let node_neg = element
            .nodes
            .get(1)
            .ok_or_else(|| format!("resistor '{}' has no negative node", resistor_name))?;
        let v_pos = Self::result_voltage_named(result, netlist, node_pos)
            .ok_or_else(|| format!("node '{}' not found in DC result", node_pos))?;
        let v_neg = Self::result_voltage_named(result, netlist, node_neg)
            .ok_or_else(|| format!("node '{}' not found in DC result", node_neg))?;
        Ok(v_pos - v_neg)
    }

    pub(super) fn result_voltage_named(
        result: &rspice_core::SimulationResult,
        netlist: &Netlist,
        node_name: &str,
    ) -> Option<Value> {
        Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| result.try_voltage_named(&candidate))
    }

    pub(super) fn result_branch_current_named(
        result: &rspice_core::SimulationResult,
        branch_name: &str,
    ) -> Option<Value> {
        result.branch_current_named(branch_name).or_else(|| {
            let normalized = Self::normalize_device_instance_name(branch_name);
            (normalized != branch_name).then(|| result.branch_current_named(&normalized))?
        })
    }

    pub(super) fn semiconductor_instance_parameter_probe_is_supported(
        netlist: &Netlist,
        element_name: &str,
        parameter: &str,
    ) -> bool {
        let Some(element) = Self::find_semiconductor_device_element(netlist, element_name) else {
            return false;
        };
        if parameter.eq_ignore_ascii_case("TEMP") {
            return true;
        }
        let Some((model_name, instance_params)) =
            Self::semiconductor_model_and_instance_params(&element)
        else {
            return false;
        };
        Self::instance_param(instance_params, &[parameter]).is_some()
            || Self::semiconductor_model_parameter_is_supported(netlist, model_name, parameter)
    }

    pub(super) fn populate_flattened_dc_probe_index(
        netlist: &Netlist,
        index: &mut XyceDcProbeIndex,
    ) {
        if index.flattened_lookup_attempted {
            return;
        }
        index.flattened_lookup_attempted = true;
        if netlist.subcircuits.is_empty() {
            return;
        }
        let Ok(flattened) = flatten_netlist_with_models(netlist) else {
            return;
        };

        let mut diode_names = HashSet::new();
        let mut recorded_branch_names = HashSet::new();
        for element in flattened.elements {
            let name = Self::normalize_device_instance_name(&element.name);
            if matches!(element.kind, ElementKind::Diode { .. }) {
                diode_names.insert(name.clone());
            }
            if Self::element_has_recorded_branch_current(&element.kind) {
                recorded_branch_names.insert(name);
            }
        }
        index.flattened_diode_names = Some(diode_names);
        index.flattened_recorded_branch_names = Some(recorded_branch_names);
    }

    pub(super) fn elements_have_recorded_branch_current(
        elements: &[rspice_core::netlist::Element],
        source: &str,
    ) -> bool {
        elements.iter().any(|element| {
            Self::device_instance_names_match(&element.name, source)
                && Self::element_has_recorded_branch_current(&element.kind)
        })
    }

    pub(super) fn element_has_recorded_branch_current(kind: &ElementKind) -> bool {
        matches!(
            kind,
            ElementKind::VoltageSource(_)
                | ElementKind::Resistor { .. }
                | ElementKind::Capacitor { .. }
                | ElementKind::Inductor { .. }
                | ElementKind::JilesAthertonInductor { .. }
                | ElementKind::XyceMemristor { .. }
                | ElementKind::Vcvs { .. }
                | ElementKind::Ccvs { .. }
                | ElementKind::VSwitch { .. }
                | ElementKind::ISwitch { .. }
                | ElementKind::GenericSwitch { .. }
                | ElementKind::BehavioralVoltage { .. }
                | ElementKind::BehavioralCurrent { .. }
        )
    }

    pub(super) fn tran_uses_uic(netlist: &Netlist) -> bool {
        netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                rspice_core::netlist::AnalysisCommand::Tran { uic: true, .. }
            )
        })
    }

    pub(super) fn time_is_transient_initial_sample(result: &TransientResult, time: Value) -> bool {
        let Some(first) = result.time.first().copied() else {
            return false;
        };
        (time - first).abs() <= 1.0e-30
    }

    pub(super) fn scalar_parameter_probe_is_supported(
        netlist: &Netlist,
        parameter_name: &str,
    ) -> bool {
        parameter_name.eq_ignore_ascii_case("TEMP")
            || parameter_name.eq_ignore_ascii_case("TEMPER")
            || parameter_name.eq_ignore_ascii_case("VT")
            || netlist.params.get(parameter_name).is_some()
            || Self::scalar_parameter_expression(&netlist.params, parameter_name).is_some_and(
                |expression| {
                    let context = Self::print_eval_context(netlist, None, None);
                    rspice_core::netlist::expr::prepare_behavioral_expression(expression, &context)
                        .and_then(|prepared| {
                            rspice_core::netlist::expr::eval_expression(&prepared, &context)
                                .map_err(|err| err.to_string())
                        })
                        .is_ok()
                },
            )
    }

    pub(super) fn scalar_parameter_expression<'a>(
        params: &'a rspice_core::netlist::ParamContext,
        parameter_name: &str,
    ) -> Option<&'a str> {
        if params.has_parameter_binding(parameter_name) {
            params.get_parameter_expression(parameter_name)
        } else {
            params.get_global_expression(parameter_name)
        }
    }

    pub(super) fn effective_resistor_value(
        netlist: &Netlist,
        name: &str,
    ) -> Result<Option<Value>, String> {
        Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_resistor_parameter(netlist, name, "R")
        .map_err(|err| format!("resistor '{name}' parameter resolution failed: {err}"))
    }

    pub(super) fn effective_capacitor_value(netlist: &Netlist, name: &str) -> Option<Value> {
        Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_capacitor_value(netlist, name)
        .ok()
        .flatten()
    }

    pub(super) fn effective_inductor_value(netlist: &Netlist, name: &str) -> Option<Value> {
        Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_inductor_value(netlist, name)
        .ok()
        .flatten()
    }

    pub(super) fn inductor_instance_params_affect_effective_value(
        instance_params: &[(String, Value)],
    ) -> bool {
        const EFFECTIVE_VALUE_PARAMS: &[&str] = &[
            "L",
            "IND",
            "VALUE",
            "INDUCTANCE",
            "M",
            "MULT",
            "SCALE",
            "TEMP",
            "DTEMP",
            "TC1",
            "TC2",
        ];
        instance_params.iter().any(|(name, _)| {
            EFFECTIVE_VALUE_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
        })
    }

    pub(super) fn capacitor_instance_params_affect_effective_value(
        instance_params: &[(String, Value)],
    ) -> bool {
        const EFFECTIVE_VALUE_PARAMS: &[&str] = &["L", "W", "M", "TEMP", "DTEMP", "TC1", "TC2"];
        instance_params.iter().any(|(name, _)| {
            EFFECTIVE_VALUE_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
        })
    }

    pub(super) fn resistor_parameter_r_value(
        netlist: &Netlist,
        name: &str,
    ) -> Result<Option<Value>, String> {
        let Some(element) = Self::find_resistor_element(netlist, name) else {
            return Ok(None);
        };
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Ok(None);
        };

        if Self::resistor_uses_xyce_default_marker(instance_params) {
            if let Some(resistance) = Self::effective_resistor_value(netlist, name)? {
                return Ok(Some(resistance));
            }
        }
        if let Some(resistance) = Self::instance_param(instance_params, &["R", "VALUE"]) {
            return Ok(Some(resistance));
        }
        Ok(Self::resistor_parameter_r_value_from_parts(
            *value,
            value_expr.as_deref(),
            model.as_deref(),
            instance_params,
            &Self::print_eval_context(netlist, None, None),
        ))
    }

    pub(super) fn resistor_uses_xyce_default_marker(instance_params: &[(String, Value)]) -> bool {
        Self::instance_param(
            instance_params,
            &[rspice_core::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER],
        )
        .is_some()
    }

    pub(super) fn resistor_parameter_r_value_from_parts(
        value: Value,
        value_expr: Option<&str>,
        model: Option<&str>,
        instance_params: &[(String, Value)],
        context: &rspice_core::netlist::ParamContext,
    ) -> Option<Value> {
        if let Some(resistance) = Self::instance_param(instance_params, &["R", "VALUE"]) {
            return Some(resistance);
        }
        if value.is_finite() {
            return Some(value);
        }
        if let Some(expression) = value_expr {
            return rspice_core::netlist::expr::eval_expression(expression, context).ok();
        }
        if model.is_some() {
            return Some(1000.0);
        }
        None
    }

    pub(super) fn resistor_temperature_value(
        netlist: &Netlist,
        name: &str,
    ) -> Result<Option<Value>, String> {
        Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_resistor_parameter(netlist, name, "TEMP")
        .map_err(|err| format!("resistor '{name}' parameter resolution failed: {err}"))
    }

    pub(super) fn resistor_instance_parameter_probe_is_supported(
        netlist: &Netlist,
        name: &str,
        parameter: &str,
    ) -> bool {
        let Some(element) = Self::find_resistor_element(netlist, name) else {
            return false;
        };
        let ElementKind::Resistor {
            instance_params, ..
        } = &element.kind
        else {
            return false;
        };
        ["W", "WIDTH", "TC", "TC1", "TC2", "M"]
            .iter()
            .any(|candidate| parameter.eq_ignore_ascii_case(candidate))
            || Self::instance_param(instance_params, &[parameter]).is_some()
    }

    pub(super) fn resistor_instance_parameter_value(
        netlist: &Netlist,
        name: &str,
        parameter: &str,
    ) -> Result<Option<Value>, String> {
        let Some(element) = Self::find_resistor_element(netlist, name) else {
            return Ok(None);
        };
        let ElementKind::Resistor {
            instance_params, ..
        } = &element.kind
        else {
            return Ok(None);
        };

        if ["W", "WIDTH", "TC", "TC1", "TC2"]
            .iter()
            .any(|candidate| parameter.eq_ignore_ascii_case(candidate))
        {
            return Engine::new(SimulationConfig {
                spice_dialect: SpiceDialect::Xyce,
                ..SimulationConfig::default()
            })
            .resolved_resistor_parameter(netlist, name, parameter)
            .map_err(|err| format!("resistor '{name}' parameter resolution failed: {err}"));
        }
        if parameter.eq_ignore_ascii_case("M") {
            return Ok(Some(
                Self::instance_param(instance_params, &[parameter]).unwrap_or(1.0),
            ));
        }
        Ok(Self::instance_param(instance_params, &[parameter]))
    }

    pub(super) fn xyce_memristor_resistance_store_name(
        netlist: &Netlist,
        name: &str,
    ) -> Option<String> {
        Self::find_native_xyce_memristor_element(netlist, name)
            .map(|element| format!("{}:R", element.name))
    }

    pub(super) fn element_has_bare_device_parameter(kind: &ElementKind) -> bool {
        matches!(
            kind,
            ElementKind::VoltageSource(_)
                | ElementKind::CurrentSource(_)
                | ElementKind::Resistor { .. }
                | ElementKind::Capacitor { .. }
                | ElementKind::Inductor { .. }
        )
    }

    pub(super) fn instance_param(params: &[(String, Value)], names: &[&str]) -> Option<Value> {
        names.iter().find_map(|candidate| {
            params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(candidate))
                .map(|(_, value)| *value)
        })
    }

    pub(super) fn frequency_data_table_points(
        netlist: &Netlist,
        table_name: &str,
        analysis: &str,
    ) -> Result<Vec<XyceFrequencyDataPoint>, String> {
        let points = netlist
            .frequency_data_table_points(table_name)
            .map_err(|error| format!("{analysis} {error}"))?;
        Ok(points
            .into_iter()
            .map(|point| XyceFrequencyDataPoint {
                frequency: point.frequency,
                overrides: point.overrides,
            })
            .collect())
    }

    pub(super) fn apply_static_dc_dimension(
        netlist: &mut Netlist,
        dimension: &XyceDcSweepDimension,
        value: Value,
    ) -> Result<(), SimulationError> {
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "DC sweep dimension '{}' produced a non-finite value {value}",
                dimension.source
            )));
        }

        if Self::is_temperature_name(&dimension.source) {
            netlist.options.temp = Some(value);
            netlist.params.set("TEMP", value);
            netlist.params.set("TEMPER", value);
            netlist
                .params
                .set("VT", Self::thermal_voltage_celsius(value));
            return Ok(());
        }

        if Self::scalar_parameter_sweep_source_is_supported(netlist, &dimension.source) {
            let had_source_text = netlist.source_text.is_some();
            let (updated, bindings) = Engine::create_perturbed_netlist_multi(
                netlist,
                &[(dimension.source.clone(), value)],
            )?;
            if had_source_text && bindings == 0 {
                return Err(SimulationError::Circuit(format!(
                    "DC sweep parameter '{}' is not bound to any netlist expression",
                    dimension.source
                )));
            }
            *netlist = updated;
            return Ok(());
        }

        if let Some(device_parameter) =
            Engine::canonical_device_parameter_sweep_source(netlist, &dimension.source)
        {
            let had_source_text = netlist.source_text.is_some();
            let (updated, bindings) =
                Engine::create_perturbed_netlist_multi(netlist, &[(device_parameter, value)])?;
            if had_source_text && bindings == 0 {
                return Err(SimulationError::Circuit(format!(
                    "DC sweep device parameter '{}' is not bound to any netlist element",
                    dimension.source
                )));
            }
            *netlist = updated;
            return Ok(());
        }

        let element = netlist
            .elements
            .iter_mut()
            .find(|element| Self::device_instance_names_match(&element.name, &dimension.source))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "DC sweep source '{}' was not found in the netlist",
                    dimension.source
                ))
            })?;
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                *spec = spec.clone().with_dc_value(value);
                // A direct source override is represented in the AST rather than
                // reparsed from the authored source, so later parameter sweeps
                // must retain the override instead of reconstructing the source.
                netlist.source_text = None;
                netlist.source_path = None;
                Ok(())
            }
            _ => Err(SimulationError::Circuit(format!(
                "DC sweep source '{}' is not an independent source",
                dimension.source
            ))),
        }
    }

    pub(super) fn split_grouped_whitespace_fields(
        line: &str,
        source_label: &str,
    ) -> Result<Vec<String>, String> {
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut single_quote = false;
        let mut double_quote = false;
        let mut escaped = false;
        let mut brace_depth = 0usize;
        let mut paren_depth = 0usize;

        for ch in line.chars() {
            if escaped {
                current.push(ch);
                escaped = false;
                continue;
            }

            match ch {
                '\\' if single_quote || double_quote => {
                    current.push(ch);
                    escaped = true;
                }
                '\'' if !double_quote => {
                    single_quote = !single_quote;
                    current.push(ch);
                }
                '"' if !single_quote => {
                    double_quote = !double_quote;
                    current.push(ch);
                }
                '{' if !single_quote && !double_quote => {
                    brace_depth += 1;
                    current.push(ch);
                }
                '}' if !single_quote && !double_quote => {
                    brace_depth = brace_depth.checked_sub(1).ok_or_else(|| {
                        format!("unmatched closing brace in {source_label}: {line}")
                    })?;
                    current.push(ch);
                }
                '(' if !single_quote && !double_quote => {
                    paren_depth += 1;
                    current.push(ch);
                }
                ')' if !single_quote && !double_quote => {
                    paren_depth = paren_depth.checked_sub(1).ok_or_else(|| {
                        format!("unmatched closing parenthesis in {source_label}: {line}")
                    })?;
                    current.push(ch);
                }
                ch if ch.is_whitespace()
                    && !single_quote
                    && !double_quote
                    && brace_depth == 0
                    && paren_depth == 0 =>
                {
                    if !current.is_empty() {
                        fields.push(std::mem::take(&mut current));
                    }
                }
                _ => current.push(ch),
            }
        }

        if single_quote || double_quote {
            return Err(format!("unterminated quote in {source_label}: {line}"));
        }
        if brace_depth != 0 {
            return Err(format!(
                "unterminated brace expression in {source_label}: {line}"
            ));
        }
        if paren_depth != 0 {
            return Err(format!(
                "unterminated parenthesized probe in {source_label}: {line}"
            ));
        }
        if !current.is_empty() {
            fields.push(current);
        }

        Ok(fields)
    }

    pub(super) fn mixed_measurement_rows(
        scalar: &[rspice_core::analysis::MeasureResult],
        continuous: &[rspice_core::analysis::ContinuousMeasureResult],
        declarations: &[rspice_core::analysis::MeasureStatement],
        base_analysis: &str,
        continuous_analysis: &str,
    ) -> Result<Vec<XyceMixedMeasurementReferenceRow>, String> {
        for result in continuous {
            result.validate_invariants().map_err(|error| {
                format!(
                    "continuous measurement '{}' violates its result invariant: {error}",
                    result.name
                )
            })?;
        }
        let mut actual = Vec::new();
        let mut scalar_index = 0usize;
        let mut continuous_index = 0usize;
        for declaration in declarations.iter().filter(|declaration| {
            declaration.analysis.eq_ignore_ascii_case(base_analysis)
                || declaration
                    .analysis
                    .eq_ignore_ascii_case(continuous_analysis)
        }) {
            if declaration.analysis.eq_ignore_ascii_case(base_analysis) {
                let result = scalar.get(scalar_index).ok_or_else(|| {
                    format!(
                        "scalar {base_analysis} evaluator omitted declaration {} ('{}')",
                        scalar_index, declaration.name
                    )
                })?;
                scalar_index += 1;
                if !result.name.eq_ignore_ascii_case(&declaration.name) {
                    return Err(format!(
                        "scalar {base_analysis} evaluator returned '{}' for declaration '{}'",
                        result.name, declaration.name
                    ));
                }
                if declaration.print_policy == rspice_core::analysis::MeasurePrintPolicy::All {
                    actual.push(XyceMixedMeasurementReferenceRow {
                        name: result.name.clone(),
                        value: result.value.map_or(
                            XyceMeasurementReferenceValue::Failed,
                            |value| XyceMeasurementReferenceValue::Numeric {
                                value,
                                quantization: None,
                            },
                        ),
                        trigger_axis: None,
                        target_axis: None,
                    });
                }
            } else {
                let result = continuous.get(continuous_index).ok_or_else(|| {
                    format!(
                        "continuous {continuous_analysis} evaluator omitted declaration {} ('{}')",
                        continuous_index, declaration.name
                    )
                })?;
                continuous_index += 1;
                if !result.name.eq_ignore_ascii_case(&declaration.name) {
                    return Err(format!(
                        "continuous {continuous_analysis} evaluator returned '{}' for declaration '{}'",
                        result.name, declaration.name
                    ));
                }
                if declaration.print_policy != rspice_core::analysis::MeasurePrintPolicy::All {
                    continue;
                }
                if result.failure.is_some() {
                    actual.push(XyceMixedMeasurementReferenceRow {
                        name: result.name.clone(),
                        value: XyceMeasurementReferenceValue::Failed,
                        trigger_axis: result.failure_metadata.and_then(|metadata| {
                            metadata.trigger_axis.map(|value| {
                                XyceMeasurementReferenceValue::Numeric {
                                    value,
                                    quantization: None,
                                }
                            })
                        }),
                        target_axis: result.failure_metadata.and_then(|metadata| {
                            metadata.target_axis.map(|value| {
                                XyceMeasurementReferenceValue::Numeric {
                                    value,
                                    quantization: None,
                                }
                            })
                        }),
                    });
                } else {
                    actual.extend(result.records.iter().map(|record| {
                        XyceMixedMeasurementReferenceRow {
                            name: result.name.clone(),
                            value: XyceMeasurementReferenceValue::Numeric {
                                value: record.value,
                                quantization: None,
                            },
                            trigger_axis: record.trigger_axis.map(|value| {
                                XyceMeasurementReferenceValue::Numeric {
                                    value,
                                    quantization: None,
                                }
                            }),
                            target_axis: record.target_axis.map(|value| {
                                XyceMeasurementReferenceValue::Numeric {
                                    value,
                                    quantization: None,
                                }
                            }),
                        }
                    }));
                }
            }
        }
        if scalar_index != scalar.len() || continuous_index != continuous.len() {
            return Err(format!(
                "measurement evaluators returned unclaimed results (scalar {scalar_index}/{}, continuous {continuous_index}/{})",
                scalar.len(),
                continuous.len()
            ));
        }
        Ok(actual)
    }

    pub(super) fn measurement_value_matches(
        expected: Value,
        actual: Value,
        quantization: Option<Value>,
        tolerance: XyceFileCompareTolerance,
    ) -> bool {
        if !expected.is_finite() || !actual.is_finite() {
            return false;
        }
        let absolute_error = (expected - actual).abs();
        let both_zero = expected.abs() <= tolerance.zero && actual.abs() <= tolerance.zero;
        let relative_error = if expected == 0.0 {
            f64::INFINITY
        } else {
            absolute_error / expected.abs()
        };
        let within_printed_quantization =
            quantization.is_some_and(|unit| absolute_error <= unit * 0.5);
        expected == actual
            || both_zero
            || within_printed_quantization
            || (absolute_error < tolerance.absolute && relative_error < tolerance.relative)
    }

    pub(super) fn measurement_literal_quantization(raw: &str) -> Option<Value> {
        let raw = raw.trim();
        let exponent_index = raw.find(['e', 'E']);
        let (mantissa, exponent) = if let Some(index) = exponent_index {
            let exponent = raw.get(index + 1..)?.parse::<i32>().ok()?;
            (raw.get(..index)?, exponent)
        } else {
            (raw, 0)
        };
        let unsigned = mantissa.strip_prefix(['+', '-']).unwrap_or(mantissa);
        let (integer, fraction) = unsigned.split_once('.')?;
        if integer.is_empty()
            || !integer.bytes().all(|byte| byte.is_ascii_digit())
            || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        {
            return None;
        }
        let fractional_digits = i32::try_from(fraction.len()).ok()?;
        let decimal_exponent = exponent.checked_sub(fractional_digits)?;
        let quantization = 10.0_f64.powi(decimal_exponent);
        (quantization.is_finite() && quantization > 0.0).then_some(quantization)
    }

    pub(super) fn tecplot_zone_title_binding(
        title: &str,
        name: &str,
    ) -> Option<XyceTecplotBinding> {
        let tokens = title.split_whitespace().collect::<Vec<_>>();
        tokens.windows(3).find_map(|window| {
            (window[0].eq_ignore_ascii_case(name) && window[1] == "=")
                .then(|| {
                    Self::parse_xyce_numeric_token(window[2])
                        .ok()
                        .map(|value| XyceTecplotBinding {
                            value,
                            quantization: Self::measurement_literal_quantization(window[2]),
                        })
                })
                .flatten()
        })
    }

    pub(super) fn tecplot_binding_matches(expected: XyceTecplotBinding, actual: Value) -> bool {
        expected.value == actual
            || expected
                .quantization
                .is_some_and(|unit| (expected.value - actual).abs() <= unit * 0.5)
    }

    pub(super) fn expanded_ac_csd_columns(
        sweep_column: &str,
        input_columns: &[String],
        expansion: &[XyceAcCsdColumnExpansion],
    ) -> Vec<String> {
        let mut columns = Vec::with_capacity(
            1 + input_columns.len()
                + expansion
                    .iter()
                    .filter(|kind| matches!(kind, XyceAcCsdColumnExpansion::Complex))
                    .count(),
        );
        columns.push(sweep_column.to_string());
        for (column, expansion) in input_columns.iter().zip(expansion.iter()) {
            match expansion {
                XyceAcCsdColumnExpansion::Scalar => columns.push(column.clone()),
                XyceAcCsdColumnExpansion::Complex => {
                    columns.push(format!("Re({column})"));
                    columns.push(format!("Im({column})"));
                }
            }
        }
        columns
    }

    pub(super) fn skip_raw_blank_lines(bytes: &[u8], offset: &mut usize) {
        while *offset < bytes.len() {
            let mut cursor = *offset;
            let Some(line) = Self::read_raw_line(bytes, &mut cursor) else {
                return;
            };
            if !line.trim().is_empty() {
                return;
            }
            *offset = cursor;
        }
    }

    pub(super) fn circuit_file_count(dir: &Path) -> Option<usize> {
        Some(
            fs::read_dir(dir)
                .ok()?
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().ok().is_some_and(|kind| kind.is_file())
                        && entry
                            .path()
                            .extension()
                            .and_then(|extension| extension.to_str())
                            .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                })
                .count(),
        )
    }

    pub(super) fn section_for_relative_path(relative_path: &str) -> XyceDeckSection {
        if relative_path.starts_with("Netlists/") {
            XyceDeckSection::Netlists
        } else {
            XyceDeckSection::Other
        }
    }

    pub(super) fn nested_include_identity_operand_span(
        line: &str,
    ) -> Result<(String, usize, usize), String> {
        let statement_end = line.find(';').unwrap_or(line.len());
        let statement = &line[..statement_end];
        let bytes = statement.as_bytes();
        let mut cursor = 0usize;
        while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            cursor += 1;
        }
        while bytes
            .get(cursor)
            .is_some_and(|byte| !byte.is_ascii_whitespace())
        {
            cursor += 1;
        }
        if !bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            return Err("include directive has no separated operand".to_string());
        }
        while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            cursor += 1;
        }
        let operand_start = cursor;
        if bytes
            .get(cursor)
            .is_some_and(|byte| matches!(*byte, b'\'' | b'\"'))
        {
            return Err("nested-include identity requires an unquoted include operand".to_string());
        }
        while bytes
            .get(cursor)
            .is_some_and(|byte| !byte.is_ascii_whitespace())
        {
            cursor += 1;
        }
        let operand_end = cursor;
        if operand_start == operand_end || !statement[cursor..].trim().is_empty() {
            return Err("include directive must contain exactly one bare operand".to_string());
        }
        let requested = rspice_core::netlist::parse_include_directive(statement)
            .ok_or_else(|| format!("malformed include directive '{}'", statement.trim()))?;
        if requested != statement[operand_start..operand_end] {
            return Err("include operand span does not match the parsed path".to_string());
        }
        Ok((requested, operand_start, operand_end))
    }

    pub(super) fn top_level_literal_parameter_definitions(
        source: &str,
    ) -> Result<BTreeMap<String, Vec<Value>>, String> {
        let mut definitions = BTreeMap::<String, Vec<Value>>::new();
        let mut subckt_depth = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            if stripped.is_empty() {
                continue;
            }
            let fields = Self::split_grouped_whitespace_fields(
                stripped,
                "numbered redefinition parameter statement",
            )?;
            let Some(command) = fields.first() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".subckt") {
                subckt_depth = subckt_depth
                    .checked_add(1)
                    .ok_or_else(|| "subcircuit nesting depth overflow".to_string())?;
                continue;
            }
            if command.eq_ignore_ascii_case(".ends") {
                subckt_depth = subckt_depth
                    .checked_sub(1)
                    .ok_or_else(|| ".ENDS without .SUBCKT in family source".to_string())?;
                continue;
            }
            if subckt_depth != 0 {
                continue;
            }
            if command.eq_ignore_ascii_case(".global_param")
                || command.eq_ignore_ascii_case(".csparam")
            {
                return Err(
                    "numbered redefinition family admits top-level .PARAM literals only"
                        .to_string(),
                );
            }
            if !command.eq_ignore_ascii_case(".param") {
                continue;
            }
            if fields.len() != 2 {
                return Err(
                    "each top-level .PARAM line must contain exactly one assignment".to_string(),
                );
            }
            let (name, raw_value) = fields[1]
                .split_once('=')
                .ok_or_else(|| "top-level .PARAM must use NAME=VALUE syntax".to_string())?;
            if raw_value.contains('=') || !Self::is_single_spice_identifier(name) {
                return Err("top-level .PARAM assignment is not canonical".to_string());
            }
            let value = Self::single_spice_numeric_literal_value(raw_value)?;
            if !value.is_finite() {
                return Err("top-level .PARAM literal must be finite".to_string());
            }
            definitions
                .entry(name.to_ascii_uppercase())
                .or_default()
                .push(value);
        }
        if subckt_depth != 0 {
            return Err("unterminated .SUBCKT in family source".to_string());
        }
        Ok(definitions)
    }

    pub(super) fn shared_stepped_dc_topology(netlist: &Netlist) -> Result<Vec<String>, String> {
        let source = netlist
            .elements
            .iter()
            .find(|element| {
                matches!(
                    &element.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(_))
                )
            })
            .ok_or_else(|| "semantic member has no independent DC source".to_string())?;
        let source_value = match &source.kind {
            ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) => *value,
            _ => unreachable!("source was selected by its element kind"),
        };
        let source_nodes = source
            .nodes
            .iter()
            .map(|node| node.trim().to_ascii_lowercase())
            .collect::<Vec<_>>();
        if !source_value.is_finite()
            || source_nodes.len() != 2
            || source_nodes.iter().any(String::is_empty)
            || source_nodes[0] == "0"
            || source_nodes[1] != "0"
            || source_nodes[0] == source_nodes[1]
        {
            return Err(
                "semantic member requires one finite, non-degenerate two-terminal DC source oriented from a non-ground node to ground".to_string(),
            );
        }

        let load = netlist
            .elements
            .iter()
            .find(|element| {
                matches!(
                    &element.kind,
                    ElementKind::Resistor { .. } | ElementKind::Subcircuit { .. }
                )
            })
            .ok_or_else(|| "semantic member has no resistor load or subcircuit load".to_string())?;
        let load_nodes = load
            .nodes
            .iter()
            .map(|node| node.trim().to_ascii_lowercase())
            .collect::<Vec<_>>();
        if load_nodes != source_nodes {
            return Err(
                "semantic member load terminals must match the ordered DC-source terminals"
                    .to_string(),
            );
        }

        if let ElementKind::Subcircuit { subckt_name, .. } = &load.kind {
            let [subcircuit] = netlist.subcircuits.as_slice() else {
                return Err("semantic hierarchical member requires one subcircuit".to_string());
            };
            let [body_element] = subcircuit.elements.as_slice() else {
                return Err(
                    "semantic hierarchical member requires one subcircuit body element".to_string(),
                );
            };
            let ports = subcircuit
                .ports
                .iter()
                .map(|node| node.trim().to_ascii_lowercase())
                .collect::<Vec<_>>();
            let body_nodes = body_element
                .nodes
                .iter()
                .map(|node| node.trim().to_ascii_lowercase())
                .collect::<Vec<_>>();
            if !subcircuit.name.eq_ignore_ascii_case(subckt_name)
                || ports.len() != 2
                || ports.iter().any(|port| port.is_empty() || port == "0")
                || ports[0] == ports[1]
                || !matches!(&body_element.kind, ElementKind::Resistor { .. })
                || ports != body_nodes
            {
                return Err(
                    "semantic subcircuit resistor terminals must match its ordered two-port interface"
                        .to_string(),
                );
            }
        }

        Ok(source_nodes)
    }

    pub(super) fn alpha_normalize_expression(text: &str, substitutions: &[(&str, &str)]) -> String {
        let mut result = String::new();
        let mut token = String::new();
        let flush = |token: &mut String, result: &mut String| {
            if token.is_empty() {
                return;
            }
            if let Some((_, replacement)) = substitutions
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(token))
            {
                result.push_str(replacement);
            } else {
                result.push_str(&token.to_ascii_lowercase());
            }
            token.clear();
        };
        for character in text.chars() {
            if character.is_ascii_alphanumeric() || character == '_' {
                token.push(character);
            } else {
                flush(&mut token, &mut result);
                if !character.is_ascii_whitespace() && character != '{' && character != '}' {
                    result.push(character.to_ascii_lowercase());
                }
            }
        }
        flush(&mut token, &mut result);
        result
    }

    pub(super) fn insert_bug754_family_member_name(
        family_members: &mut BTreeSet<String>,
        name: &str,
    ) -> Result<(), String> {
        let normalized = name.to_ascii_lowercase();
        if !family_members.insert(normalized.clone()) {
            return Err(format!(
                "BUG 754 dcsweep family contains case-colliding physical record '{name}' normalized as '{normalized}'"
            ));
        }
        Ok(())
    }

    pub(super) fn bug667_effective_nodeset_map(
        netlist: &Netlist,
    ) -> Result<BTreeMap<String, u64>, String> {
        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist)
            .map_err(|err| format!("BUG 667 hierarchy flattening failed: {err}"))?;
        let mut nodesets = BTreeMap::new();
        for nodeset in netlist
            .node_sets
            .iter()
            .chain(flattened.scoped_node_sets.iter())
        {
            if !nodeset.voltage.is_finite() || nodeset.voltage_expr.is_some() {
                return Err(format!(
                    "BUG 667 effective NODESET '{}' is nonfinite or remains deferred",
                    nodeset.node
                ));
            }
            let node = if nodeset.node.contains([':', '.']) {
                Engine::resolve_hierarchical_node_name(netlist, &nodeset.node).ok_or_else(|| {
                    format!(
                        "BUG 667 hierarchical NODESET target '{}' does not resolve through the parsed instance tree",
                        nodeset.node
                    )
                })?
            } else {
                nodeset.node.clone()
            }
            .replace(':', ".")
            .to_ascii_lowercase();
            if nodesets
                .insert(node.clone(), nodeset.voltage.to_bits())
                .is_some()
            {
                return Err(format!(
                    "BUG 667 effective NODESET map contains duplicate node '{node}'"
                ));
            }
        }
        Ok(nodesets)
    }

    pub(super) fn bug662_header_and_body(
        source: &str,
        role: XyceBug662HeaderRole,
    ) -> Result<(Vec<String>, String), String> {
        if !source.is_ascii() {
            return Err("BUG 662 title/header contract is ASCII-only".into());
        }
        let normalized = source.replace("\r\n", "\n");
        if normalized.contains('\r') {
            return Err("BUG 662 source contains a noncanonical bare carriage return".into());
        }
        let lines = normalized.lines().collect::<Vec<_>>();
        let separator = lines
            .iter()
            .position(|line| line.is_empty())
            .ok_or_else(|| "BUG 662 source has no blank title/body separator".to_string())?;
        let header = lines[..separator]
            .iter()
            .map(|line| (*line).to_string())
            .collect::<Vec<_>>();
        let expected_header_lines = match role {
            XyceBug662HeaderRole::LongHeaderOwner => 1,
            XyceBug662HeaderRole::ShortHeaderReference => 2,
        };
        if header.len() != expected_header_lines || header.iter().any(|line| !line.starts_with('*'))
        {
            return Err(format!(
                "BUG 662 {role:?} requires exactly {expected_header_lines} leading comment header line(s)"
            ));
        }
        match role {
            XyceBug662HeaderRole::LongHeaderOwner => {
                if header[0].len() != 312 || header[0].len() <= 256 {
                    return Err(format!(
                        "BUG 662 owner title must be the canonical 312-byte over-256 line, got {} bytes",
                        header[0].len()
                    ));
                }
            }
            XyceBug662HeaderRole::ShortHeaderReference => {
                if header[0].len() != 256 || header[1].len() != 57 {
                    return Err(format!(
                        "BUG 662 reference title must split at the canonical 256/57-byte boundary, got {}/{}",
                        header[0].len(),
                        header[1].len()
                    ));
                }
            }
        }
        let body = lines[separator + 1..].join("\n");
        const CANONICAL_BODY: &str = "*Analysis directives:\n\
.TRAN 0 100ns 0\n\
.PRINT TRAN V(N14950) V(N15037)\n\
\n\
* source TRANSMISSIONLINE\n\
T_T1 N14950 0 N15037 0 TD=10e-9 Z0=50\n\
R_R1 N14553 N14950 TC=0,0 R=50\n\
R_R2 N15037 0 TC=0,0 R=50\n\
V_V1 N14553 0 PULSE(0 5 0 0.1e-9 0.1e-9 5e-9 25e-9)\n\
\n\
\n\
.END";
        if body != CANONICAL_BODY {
            return Err(
                "BUG 662 executable body is not the canonical transmission-line deck".into(),
            );
        }
        Ok((header, body))
    }

    pub(super) fn bug647_resistor_diagnostics_are_exact(
        diagnostics: &[rspice_core::netlist::ParseDiagnostic],
    ) -> bool {
        diagnostics.len() == 1
            && diagnostics[0].code == "xyce_resistor_model_missing_value"
            && diagnostics[0]
                .message
                .contains("no explicit value; model resolution may use Xyce's default 1000 ohm")
    }

    pub(super) fn plain_unit_resistor_on_nodes(
        element: &rspice_core::netlist::Element,
        nodes: &[String; 2],
    ) -> bool {
        let actual_nodes = element
            .nodes
            .iter()
            .map(|node| Self::canonical_param_expression_node_name(node))
            .collect::<Vec<_>>();
        matches!(
            &element.kind,
            ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty()
                && actual_nodes == nodes
        )
    }

    pub(super) fn same_path(left: &Path, right: &Path) -> bool {
        let left = left.canonicalize().unwrap_or_else(|_| left.to_path_buf());
        let right = right.canonicalize().unwrap_or_else(|_| right.to_path_buf());
        left == right
    }

    pub(super) fn relative_key(&self, path: &Path) -> String {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        canonical
            .strip_prefix(&self.root)
            .or_else(|_| path.strip_prefix(&self.root))
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    pub(super) fn display_path(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    pub(super) fn contains_control_block(source: &str) -> bool {
        source.lines().any(|line| {
            let normalized = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            normalized == ".control" || normalized == ".endc"
        })
    }

    pub(super) fn normalize_probe(probe: &str) -> String {
        probe
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    }
}
