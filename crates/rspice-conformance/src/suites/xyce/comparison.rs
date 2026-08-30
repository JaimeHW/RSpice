//! Comparing engine output against a reference, within tolerance.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceFileCompareTolerance {
    /// Validate tolerances before reproducing Release 7.10's `file_compare.pl`.
    ///
    /// Zero-valued tolerances are meaningful because the historical script
    /// uses strict comparisons; negative and non-finite tolerances are not.
    pub(super) fn validate(self) -> Result<Self, String> {
        if !self.absolute.is_finite()
            || self.absolute < 0.0
            || !self.relative.is_finite()
            || self.relative < 0.0
            || !self.zero.is_finite()
            || self.zero < 0.0
        {
            return Err(format!(
                "Release 7.10 file_compare tolerances must be finite and nonnegative, got {self:?}"
            ));
        }
        Ok(self)
    }
}

impl XyceTestRunner {
    pub(super) fn compare_measure_cont_step_waveforms(
        index: usize,
        owner: &XyceStepTranEvaluation,
        control: &XyceStepTranEvaluation,
    ) -> Result<(), String> {
        let exact_values = |left: &[Value], right: &[Value]| {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| left.to_bits() == right.to_bits())
        };
        if owner.step_values != control.step_values
            || owner.transient.node_names != control.transient.node_names
            || owner.transient.branch_names != control.transient.branch_names
            || !exact_values(&owner.transient.time, &control.transient.time)
            || owner.transient.voltages.len() != control.transient.voltages.len()
            || owner
                .transient
                .voltages
                .iter()
                .zip(&control.transient.voltages)
                .any(|(left, right)| !exact_values(left, right))
            || owner.transient.branch_currents.len() != control.transient.branch_currents.len()
            || owner
                .transient
                .branch_currents
                .iter()
                .zip(&control.transient.branch_currents)
                .any(|(left, right)| !exact_values(left, right))
        {
            return Err(format!(
                "MEASURE_CONT STEP owner waveform does not exactly equal control {index}"
            ));
        }
        Ok(())
    }

    pub(super) fn compare_measure_cont_step_measurements(
        index: usize,
        owner: &XyceStepTranEvaluation,
        control: &XyceStepTranEvaluation,
    ) -> Result<(), String> {
        let owner_rows = Self::mixed_measurement_rows(
            &owner.scalar,
            &owner.continuous,
            &owner.netlist.measurements,
            "TRAN",
            "TRAN_CONT",
        )?;
        let control_rows = Self::mixed_measurement_rows(
            &control.scalar,
            &control.continuous,
            &control.netlist.measurements,
            "TRAN",
            "TRAN_CONT",
        )?;
        let owner_stream = Self::measure_cont_step_mt_stream(&owner_rows)?;
        let control_stream = Self::measure_cont_step_mt_stream(&control_rows)?;
        if owner_stream != control_stream {
            return Err(format!(
                "MEASURE_CONT STEP owner mt{} stream differs from control mt0",
                index
            ));
        }
        Ok(())
    }

    pub(super) fn compare_noise_step_gs_semantics(
        gs: &[XyceMeasureContGsRow],
        offset: usize,
        netlist: &Netlist,
        scalar: &[rspice_core::analysis::MeasureResult],
        continuous: &[rspice_core::analysis::ContinuousMeasureResult],
        tolerance: XyceFileCompareTolerance,
    ) -> Result<usize, String> {
        for result in continuous {
            result.validate_invariants().map_err(|error| {
                format!("continuous result '{}' is invalid: {error}", result.name)
            })?;
        }
        let actual = Self::mixed_measurement_rows(
            scalar,
            continuous,
            &netlist.measurements,
            "NOISE",
            "NOISE_CONT",
        )?;
        let expected = gs.get(offset..offset + actual.len()).ok_or_else(|| {
            format!(
                "GS ended before step projection {offset}..{}",
                offset + actual.len()
            )
        })?;
        let mut mismatches = Vec::new();
        for (row, (expected, actual)) in expected.iter().zip(&actual).enumerate() {
            if !expected.mixed.name.eq_ignore_ascii_case(&actual.name) {
                return Err(format!(
                    "GS row {} is '{}' but declaration projection is '{}'",
                    offset + row,
                    expected.mixed.name,
                    actual.name
                ));
            }
            Self::compare_mixed_measurement_value(
                &mut mismatches,
                row,
                &actual.name,
                expected.mixed.value,
                actual.value,
                tolerance,
            )?;
            Self::compare_mixed_measurement_metadata(
                &mut mismatches,
                row,
                &format!("{}:trig", actual.name),
                expected.mixed.trigger_axis,
                actual.trigger_axis,
                tolerance,
            )?;
            Self::compare_mixed_measurement_metadata(
                &mut mismatches,
                row,
                &format!("{}:targ", actual.name),
                expected.mixed.target_axis,
                actual.target_axis,
                tolerance,
            )?;
        }

        let numeric = |value: Option<Value>| {
            value.map(|value| XyceMeasurementReferenceValue::Numeric {
                value,
                quantization: None,
            })
        };
        let mut projected_row = 0usize;
        let mut scalar_index = 0usize;
        let mut continuous_index = 0usize;
        for declaration in &netlist.measurements {
            if declaration.analysis.eq_ignore_ascii_case("NOISE") {
                let result = scalar
                    .get(scalar_index)
                    .ok_or_else(|| format!("NOISE evaluator omitted '{}'", declaration.name))?;
                scalar_index += 1;
                let expected_row = expected.get(projected_row).ok_or_else(|| {
                    format!("GS ended at scalar declaration '{}'", declaration.name)
                })?;
                Self::compare_mixed_measurement_metadata(
                    &mut mismatches,
                    projected_row,
                    &format!("{}:event", declaration.name),
                    expected_row.event_axis,
                    numeric(result.event_axis),
                    tolerance,
                )?;
                projected_row += 1;
                continue;
            }
            if !declaration.analysis.eq_ignore_ascii_case("NOISE_CONT") {
                return Err(format!(
                    "unexpected measurement analysis '{}' in stepped NOISE GS projection",
                    declaration.analysis
                ));
            }
            let result = continuous
                .get(continuous_index)
                .ok_or_else(|| format!("NOISE_CONT evaluator omitted '{}'", declaration.name))?;
            continuous_index += 1;
            let row_count = if result.failure.is_some() {
                1
            } else {
                result.records.len()
            };
            for record_index in 0..row_count {
                let expected_row = expected
                    .get(projected_row)
                    .ok_or_else(|| format!("GS ended inside declaration '{}'", declaration.name))?;
                let actual_axis = result
                    .records
                    .get(record_index)
                    .and_then(|record| record.event_axis)
                    .or_else(|| {
                        (result.failure.is_some() && record_index == 0)
                            .then_some(match &declaration.measure_type {
                                rspice_core::analysis::MeasureType::Derivative { at, .. }
                                | rspice_core::analysis::MeasureType::Find { at, .. } => *at,
                                _ => None,
                            })
                            .flatten()
                    });
                Self::compare_mixed_measurement_metadata(
                    &mut mismatches,
                    projected_row,
                    &format!("{}:event", declaration.name),
                    expected_row.event_axis,
                    numeric(actual_axis),
                    tolerance,
                )?;
                projected_row += 1;
            }
        }
        if scalar_index != scalar.len()
            || continuous_index != continuous.len()
            || projected_row != actual.len()
        {
            return Err(format!(
                "stepped NOISE GS projection left rows/results unclaimed: rows={projected_row}/{}, scalar={scalar_index}/{}, continuous={continuous_index}/{}",
                actual.len(),
                scalar.len(),
                continuous.len()
            ));
        }
        if mismatches.is_empty() {
            Ok(actual.len())
        } else {
            Err(format!(
                "stepped NOISE GS semantic comparison produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ))
        }
    }

    pub(super) fn compare_xdm_replaceground_tables(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
        good_results: &[DcSweepPointResult],
        test_results: &[DcSweepPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        // See the constants above: these declared wrapper values were not
        // forwarded by Release 7.10's HSPICE branch.
        if XYCE_XDM_REPLACEGROUND_DECLARED_ABSOLUTE_TOLERANCE.to_bits() != 1.0e-5f64.to_bits()
            || XYCE_XDM_REPLACEGROUND_DECLARED_RELATIVE_TOLERANCE.to_bits() != 1.0e-3f64.to_bits()
            || XYCE_XDM_REPLACEGROUND_DECLARED_ZERO_TOLERANCE.to_bits() != 1.0e-10f64.to_bits()
        {
            return Err("XDM wrapper declared-tolerance provenance changed".to_string());
        }
        self.compare_release_7_10_xyce_verify_dc_tables(
            "XDM REPLACEGROUND",
            good,
            test,
            good_results,
            test_results,
        )
    }

    pub(super) fn compare_release_7_10_xyce_verify_dc_tables(
        &self,
        label: &str,
        good: &XycePrnTable,
        test: &XycePrnTable,
        good_results: &[DcSweepPointResult],
        _test_results: &[DcSweepPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if good.columns.len() != test.columns.len()
            || !good
                .columns
                .iter()
                .zip(&test.columns)
                .all(|(good, test)| good.eq_ignore_ascii_case(test))
            || good.rows.len() != test.rows.len()
            || good_results.len() != good.rows.len()
        {
            return Err(format!(
                "{label} comparison layout differs: good {:?}/{} rows, test {:?}/{} rows",
                good.columns,
                good.rows.len(),
                test.columns,
                test.rows.len()
            ));
        }
        if good.rows.is_empty() {
            return Err(format!("{label} xyce_verify comparison has no DC points"));
        }
        for (row_index, good_result) in good_results.iter().enumerate().take(good.rows.len()) {
            if good.rows[row_index].len() != good.columns.len()
                || test.rows[row_index].len() != test.columns.len()
            {
                return Err(format!(
                    "{label} comparison row {row_index} does not match its column layout"
                ));
            }
            let requested_sweep = Self::xyce_prn_scientific_roundtrip(
                good_result.sweep_value,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            let raw_good_axis = Self::xyce_prn_scientific_roundtrip(
                good.rows[row_index][1],
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            let good_axis = if raw_good_axis.abs() <= XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE {
                0.0
            } else {
                raw_good_axis
            };
            if (requested_sweep - good_axis).abs()
                > XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE
            {
                return Err(format!(
                    "{label} good V(1) does not match the formatted requested sweep at row {row_index}: requested={requested_sweep}, good={good_axis}"
                ));
            }
            let expected_index = row_index as Value;
            let good_index = Self::xyce_prn_scientific_roundtrip(
                good.rows[row_index][0],
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            let test_index = Self::xyce_prn_scientific_roundtrip(
                test.rows[row_index][0],
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if good_index.to_bits() != expected_index.to_bits()
                || test_index.to_bits() != expected_index.to_bits()
            {
                return Err(format!(
                    "{label} serialized Index differs at row {row_index}: good={good_index}, test={test_index}"
                ));
            }
        }

        if good.rows.len() == 1 {
            let requested_sweep = Self::xyce_prn_scientific_roundtrip(
                good_results[0].sweep_value,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            let test_axis = Self::xyce_prn_scientific_roundtrip(
                test.rows[0][1],
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if !test_axis.is_finite()
                || (requested_sweep - test_axis).abs()
                    > XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE
            {
                return Err(format!(
                    "{label} one-point test axis does not match the requested sweep: requested={requested_sweep}, test={test_axis}"
                ));
            }

            let mut mismatches = Vec::new();
            for column_index in 2..good.columns.len() {
                let raw_good_value = Self::xyce_prn_scientific_roundtrip(
                    good.rows[0][column_index],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )?;
                let raw_test_value = Self::xyce_prn_scientific_roundtrip(
                    test.rows[0][column_index],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )?;
                if !raw_good_value.is_finite() || !raw_test_value.is_finite() {
                    return Err(format!(
                        "{label} one-point comparison contains non-finite {}: good={raw_good_value}, test={raw_test_value}",
                        good.columns[column_index]
                    ));
                }
                let good_value = if raw_good_value.abs() <= XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE {
                    0.0
                } else {
                    raw_good_value
                };
                let test_value = if raw_test_value.abs() <= XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE {
                    0.0
                } else {
                    raw_test_value
                };
                let difference = good_value - test_value;
                let normalized =
                    if difference.abs() < XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE {
                        0.0
                    } else {
                        difference
                            / (XYCE_VERIFY_DEFAULT_RELATIVE_TOLERANCE * good_value.abs()
                                + XYCE_VERIFY_DEFAULT_ABSOLUTE_TOLERANCE)
                    };
                if !normalized.is_finite() || normalized.abs() > 1.0 {
                    mismatches.push(XyceValueMismatch {
                        row: 0,
                        probe: good.columns[column_index].clone(),
                        expected: good_value,
                        actual: test_value,
                        relative_error: normalized.abs(),
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        break;
                    }
                }
            }
            return Ok(mismatches);
        }

        // Release 7.10 xyce_verify treats its first numeric file as `good`,
        // its second as `test`, integrates squared normalized error with the
        // trapezoidal rule over the independent axis, and fails a signal only
        // when normalized RMS exceeds one.
        let mut mismatches = Vec::new();
        let serialized_test_axis = test
            .rows
            .iter()
            .map(|row| {
                let raw_axis = Self::xyce_prn_scientific_roundtrip(
                    row[1],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )?;
                Ok::<Value, String>(if raw_axis.abs() <= XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE {
                    0.0
                } else {
                    raw_axis
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let axis_start = serialized_test_axis[0];
        let axis_stop = serialized_test_axis[serialized_test_axis.len() - 1];
        let axis_span = (axis_stop - axis_start).abs();
        if !axis_span.is_finite() || axis_span <= 0.0 {
            return Err(format!(
                "{label} xyce_verify axis has invalid span {axis_start}..{axis_stop}"
            ));
        }
        // The intended one-variable DC verifier contract treats V(1) as the
        // independent variable after removing Index. This adapter validates
        // the good axis against the requested sweep and starts dependent
        // comparisons at V(2); the test V(1) values supply errNorm's
        // integration grid. Release 7.10's Perl check of the good axis was
        // accidentally dormant due to a misspelled loop variable. Dedicated
        // family contracts such as BUG402 may strengthen this by validating
        // both axes before entering the historical value comparison.
        for column_index in 2..good.columns.len() {
            let mut errors = Vec::with_capacity(good.rows.len());
            for row_index in 0..good.rows.len() {
                let raw_good_value = Self::xyce_prn_scientific_roundtrip(
                    good.rows[row_index][column_index],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )?;
                let raw_test_value = Self::xyce_prn_scientific_roundtrip(
                    test.rows[row_index][column_index],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )?;
                if !raw_good_value.is_finite() || !raw_test_value.is_finite() {
                    return Err(format!(
                        "{label} comparison contains non-finite {} at row {row_index}: good={raw_good_value}, test={raw_test_value}",
                        good.columns[column_index]
                    ));
                }
                // xyce_verify applies each column's ZEROTOL while loading
                // both files, before computing differences and error norms.
                let good_value = if raw_good_value.abs() <= XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE {
                    0.0
                } else {
                    raw_good_value
                };
                let test_value = if raw_test_value.abs() <= XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE {
                    0.0
                } else {
                    raw_test_value
                };
                let difference = good_value - test_value;
                let normalized =
                    if difference.abs() < XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE {
                        0.0
                    } else {
                        difference
                            / (XYCE_VERIFY_DEFAULT_RELATIVE_TOLERANCE * good_value.abs()
                                + XYCE_VERIFY_DEFAULT_ABSOLUTE_TOLERANCE)
                    };
                errors.push(normalized);
            }
            let mut integral = 0.0;
            for row_index in 1..errors.len() {
                let width =
                    (serialized_test_axis[row_index] - serialized_test_axis[row_index - 1]).abs();
                integral +=
                    0.5 * width * (errors[row_index - 1].powi(2) + errors[row_index].powi(2));
            }
            let normalized_rms = (integral / axis_span).sqrt();
            if !normalized_rms.is_finite() || normalized_rms > 1.0 {
                mismatches.push(XyceValueMismatch {
                    row: good.rows.len() - 1,
                    probe: good.columns[column_index].clone(),
                    expected: good.rows.last().expect("nonempty good table")[column_index],
                    actual: test.rows.last().expect("nonempty test table")[column_index],
                    relative_error: normalized_rms,
                });
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_release_7_10_xyce_verify_dc_batches(
        &self,
        label: &str,
        good: &XycePrnTable,
        test: &XycePrnTable,
        good_batches: &[XyceDcResultBatch],
        test_batches: &[XyceDcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if good_batches.len() != test_batches.len() || good_batches.is_empty() {
            return Err(format!(
                "{label} STEP batch count differs: good={}, test={}",
                good_batches.len(),
                test_batches.len()
            ));
        }
        let good_row_count = good_batches
            .iter()
            .map(|batch| batch.results.len())
            .sum::<usize>();
        let test_row_count = test_batches
            .iter()
            .map(|batch| batch.results.len())
            .sum::<usize>();
        if good.rows.len() != good_row_count || test.rows.len() != test_row_count {
            return Err(format!(
                "{label} table/batch row census differs: good={}/{good_row_count}, test={}/{test_row_count}",
                good.rows.len(),
                test.rows.len()
            ));
        }

        let mut good_offset = 0usize;
        let mut test_offset = 0usize;
        let mut all_mismatches = Vec::new();
        for (batch_index, (good_batch, test_batch)) in
            good_batches.iter().zip(test_batches).enumerate()
        {
            let good_end = good_offset + good_batch.results.len();
            let test_end = test_offset + test_batch.results.len();
            let good_table = XycePrnTable {
                columns: good.columns.clone(),
                rows: good.rows[good_offset..good_end].to_vec(),
            };
            let test_table = XycePrnTable {
                columns: test.columns.clone(),
                rows: test.rows[test_offset..test_end].to_vec(),
            };
            let batch_label = format!("{label} STEP batch {batch_index}");
            let mut mismatches = self.compare_release_7_10_xyce_verify_dc_tables(
                &batch_label,
                &good_table,
                &test_table,
                &good_batch.results,
                &test_batch.results,
            )?;
            for mismatch in &mut mismatches {
                mismatch.row += good_offset;
                mismatch.probe = format!("STEP[{batch_index}] {}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
            good_offset = good_end;
            test_offset = test_end;
        }
        Ok(all_mismatches)
    }

    pub(super) fn native_default_prn_tran_wrapper_tolerance(
        relative_path: &str,
    ) -> Option<XyceComparisonTolerance> {
        match Self::normalize_manifest_key(relative_path).as_str() {
            "netlists/output/tran/tran-prn-noindex.cir"
            | "netlists/output/tran/tran-step-gnuplot.cir"
            | "netlists/output/tran/tran-stepnum-col.cir" => Some(XyceComparisonTolerance {
                relative: 1.0e-3,
                absolute: 1.0e-5,
                zero: Some(1.0e-8),
            }),
            _ => None,
        }
    }

    pub(super) fn compare_ac_initial_condition_outputs(
        &self,
        plan: &XyceStaticAcPlan,
        netlist: &Netlist,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if !Self::source_has_op_analysis(&plan.source) {
            return Ok(Vec::new());
        }
        let requests = if plan.output_override {
            Self::output_override_print_output_request(&plan.source, "AC_IC")?
                .into_iter()
                .collect::<Vec<_>>()
        } else {
            Self::aggregate_print_output_requests(
                Self::print_output_requests(&plan.source, "AC_IC")?,
                "AC_IC",
            )?
        };
        if requests.is_empty() {
            return Ok(Vec::new());
        }

        let dc = Self::synthetic_op_dc_sweep(netlist)?;
        let (result, device_op_report) = self
            .create_dc_engine()
            .run_dc_op_with_report(netlist)
            .map_err(|err| format!(".OP solve for AC_IC output failed: {err}"))?;
        let op_result = DcSweepPointResult {
            sweep_value: dc.start,
            result,
            device_op_report,
        };

        let mut all_mismatches = Vec::new();
        for request in requests {
            let reference_path = self.ac_initial_condition_reference_path(plan, &request)?;
            let reference =
                Self::parse_ac_initial_condition_reference_file(&request, &reference_path)
                    .map_err(|err| {
                        format!(
                            "failed to parse AC_IC oracle {}: {err}",
                            self.display_path(&reference_path)
                        )
                    })?;
            let print = XycePrintRequest {
                probes: request.probes.clone(),
            };
            let mut mismatches = self.compare_ac_initial_condition_reference(
                &reference,
                &print,
                netlist,
                &plan.source,
                &dc,
                &op_result,
            )?;
            if let Some(file) = request.file.as_deref()
                && Some(file) != plan.primary_ac_ic_file.as_deref()
            {
                for mismatch in &mut mismatches {
                    mismatch.probe = format!("{file}:{}", mismatch.probe);
                }
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn compare_ac_initial_condition_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        dc: &XyceDcSweep,
        result: &DcSweepPointResult,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let layout = Self::transient_reference_layout(reference)?;
        if reference.rows.len() != 1 {
            return Err(format!(
                "AC_IC reference row count ({}) does not match .OP point count (1)",
                reference.rows.len()
            ));
        }

        let row = &reference.rows[0];
        if row.len() != reference.columns.len() {
            return Err(format!(
                "row 0 has {} values, expected {}",
                row.len(),
                reference.columns.len()
            ));
        }

        let mut mismatches = Vec::new();
        if let Some(stepnum_column) = layout.stepnum_column {
            let expected_stepnum = row[stepnum_column];
            if let Some(relative_error) = self.value_mismatch(
                expected_stepnum,
                0.0,
                self.default_comparison_tolerance("stepnum"),
            ) {
                mismatches.push(XyceValueMismatch {
                    row: 0,
                    probe: "STEPNUM".to_string(),
                    expected: expected_stepnum,
                    actual: 0.0,
                    relative_error,
                });
            }
        }
        if let Some(index_column) = layout.index_column {
            let expected_index = row[index_column];
            if let Some(relative_error) = self.value_mismatch(
                expected_index,
                0.0,
                self.default_comparison_tolerance("index"),
            ) {
                mismatches.push(XyceValueMismatch {
                    row: 0,
                    probe: "Index".to_string(),
                    expected: expected_index,
                    actual: 0.0,
                    relative_error,
                });
            }
        }

        let expected_time = row[layout.time_column];
        if let Some(relative_error) = self.value_mismatch(
            expected_time,
            0.0,
            self.default_comparison_tolerance("time"),
        ) {
            mismatches.push(XyceValueMismatch {
                row: 0,
                probe: "TIME".to_string(),
                expected: expected_time,
                actual: 0.0,
                relative_error,
            });
        }
        if mismatches.len() >= self.config.max_mismatches {
            mismatches.truncate(self.config.max_mismatches);
            return Ok(mismatches);
        }

        let data_columns = self.reference_data_columns(
            reference,
            print,
            netlist,
            layout.data_column_offset,
            true,
        )?;
        let comp_tolerances = self.comp_tolerances(source, &data_columns)?;
        let sweep_point = XyceDcSweepPoint {
            primary: dc.start,
            secondary: None,
        };
        for (column_index, column) in data_columns.iter().enumerate() {
            let expected = row[column_index + layout.data_column_offset];
            let (probe, actual) = match column {
                XyceReferenceColumn::PrimarySweep { name } => (name.as_str(), dc.start),
                XyceReferenceColumn::Probe { name } => (
                    name.as_str(),
                    Self::evaluate_dc_probe(
                        name,
                        netlist,
                        dc,
                        sweep_point,
                        &result.result,
                        &result.device_op_report,
                    )?,
                ),
            };
            let normalized_probe = Self::normalize_probe(probe);
            let tolerance = comp_tolerances
                .get(&normalized_probe)
                .copied()
                .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
            if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                mismatches.push(XyceValueMismatch {
                    row: 0,
                    probe: probe.to_string(),
                    expected,
                    actual,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    mismatches.truncate(self.config.max_mismatches);
                    break;
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_step_ac_reference_batches(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        source: &str,
        points_per_step: usize,
        batches: &[XyceAcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let step_references =
            Self::split_ac_step_reference(reference, batches.len(), points_per_step)?;
        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (step_index, (batch, step_reference)) in
            batches.iter().zip(step_references.iter()).enumerate()
        {
            let mut step_mismatches = self.compare_ac_prn_reference_with_step(
                step_reference,
                print,
                &batch.netlist,
                source,
                &batch.results,
                Some(step_index),
            )?;
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
            }
            row_offset += step_reference.rows.len();
            mismatches.extend(step_mismatches);
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_ac_side_outputs(
        &self,
        plan: &XyceStaticAcPlan,
        netlist: &Netlist,
        results: &[AcResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let Some(primary_reference_path) = plan.reference_path.as_ref() else {
            return Err(
                "AC side-output comparison requires a primary .PRINT AC oracle".to_string(),
            );
        };
        let side_outputs = Self::prn_compatible_ac_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            if Some(file) == plan.primary_ac_file.as_deref() {
                continue;
            }
            let reference_path =
                Self::ac_side_output_reference_path(primary_reference_path, &request, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse AC side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches =
                self.compare_ac_prn_reference(&reference, &print, netlist, &plan.source, results)?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn compare_ac_data_side_outputs(
        &self,
        plan: &XyceStaticAcPlan,
        points: &[XyceAcDataPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let Some(primary_reference_path) = plan.reference_path.as_ref() else {
            return Err(
                ".AC DATA side-output comparison requires a primary .PRINT AC oracle".to_string(),
            );
        };
        let side_outputs = Self::prn_compatible_ac_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            if Some(file) == plan.primary_ac_file.as_deref() {
                continue;
            }
            let reference_path =
                Self::ac_side_output_reference_path(primary_reference_path, &request, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse AC side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches =
                self.compare_ac_data_prn_reference(&reference, &print, &plan.source, points)?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn compare_step_ac_side_outputs(
        &self,
        plan: &XyceStaticAcPlan,
        points_per_step: usize,
        batches: &[XyceAcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let Some(primary_reference_path) = plan.reference_path.as_ref() else {
            return Err(
                "stepped AC side-output comparison requires a primary .PRINT AC oracle".to_string(),
            );
        };
        let side_outputs = Self::prn_compatible_ac_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            if Some(file) == plan.primary_ac_file.as_deref() {
                continue;
            }
            let reference_path =
                Self::ac_side_output_reference_path(primary_reference_path, &request, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse AC side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches = self.compare_step_ac_reference_batches(
                &reference,
                &print,
                &plan.source,
                points_per_step,
                batches,
            )?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn candidate_mismatches_are_better(
        current_best: Option<&[XyceValueMismatch]>,
        candidate: &[XyceValueMismatch],
    ) -> bool {
        let Some(best) = current_best else {
            return true;
        };
        if candidate.len() != best.len() {
            return candidate.len() < best.len();
        }

        let candidate_max = Self::mismatch_max_relative_error(candidate);
        let best_max = Self::mismatch_max_relative_error(best);
        if candidate_max != best_max {
            return candidate_max < best_max;
        }

        Self::mismatch_relative_error_sum(candidate) < Self::mismatch_relative_error_sum(best)
    }

    pub(super) fn mismatch_max_relative_error(mismatches: &[XyceValueMismatch]) -> f64 {
        mismatches
            .iter()
            .map(|mismatch| mismatch.relative_error)
            .filter(|value| value.is_finite())
            .fold(0.0, f64::max)
    }

    pub(super) fn mismatch_relative_error_sum(mismatches: &[XyceValueMismatch]) -> f64 {
        mismatches
            .iter()
            .map(|mismatch| mismatch.relative_error)
            .filter(|value| value.is_finite())
            .sum()
    }

    pub(super) fn compare_step_tran_runs(
        &self,
        plan: &XyceStaticTranPlan,
        step_runs: &[XyceStepRun],
        step_references: &[XycePrnTable],
        abort: &dyn AbortSignal,
        locked_time_grid: bool,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_step_tran_runs_with_print(
            plan,
            plan.require_print("stepped transient comparison")?,
            step_runs,
            step_references,
            abort,
            locked_time_grid,
            true,
        )
    }

    pub(super) fn compare_step_tran_runs_with_print(
        &self,
        plan: &XyceStaticTranPlan,
        print: &XycePrintRequest,
        step_runs: &[XyceStepRun],
        step_references: &[XycePrnTable],
        abort: &dyn AbortSignal,
        locked_time_grid: bool,
        use_plan_comparison_mode: bool,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let uses_integrated_rms =
            use_plan_comparison_mode && plan.comparison_mode.uses_integrated_rms_verifier();
        if uses_integrated_rms && locked_time_grid {
            return Err(
                "integrated-RMS stepped transient verification does not admit a locked-grid retry"
                    .to_string(),
            );
        }
        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (step_index, (run, reference)) in
            step_runs.iter().zip(step_references.iter()).enumerate()
        {
            let max_step = if uses_integrated_rms {
                Self::transient_max_step_for_static_plan(plan, &run.netlist, &plan.tran, reference)
            } else {
                Self::transient_max_step_for_reference(&run.netlist, &plan.tran, reference)
            }
            .map_err(|err| {
                if err.contains("transient harness execution envelope") {
                    format!("UNSUPPORTED: {err}")
                } else {
                    format!("reference time-grid error: step {}: {err}", step_index + 1)
                }
            })?;
            let engine = if uses_integrated_rms {
                self.create_xyce_static_tran_engine(
                    None,
                    Self::xyce_initial_timestep_for_tran(&plan.tran),
                )
            } else if locked_time_grid {
                self.create_xyce_engine_with_locked_time_grid(Some(Self::reference_time_grid(
                    reference,
                )?))
            } else {
                self.create_xyce_engine()
            };
            let result = match engine.run_tran_with_startup_mode_and_abort(
                &run.netlist,
                plan.tran.stop,
                max_step,
                rspice_core::engine::TransientStartupMode::from_uic(plan.tran.uic),
                abort,
            ) {
                Ok(result) => result,
                Err(SimulationError::Aborted) => {
                    return Err(format!(
                        "simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ));
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return Err(format!(
                        "UNSUPPORTED: RSpice runtime does not yet support this .STEP TRAN deck: {err}"
                    ));
                }
                Err(err) => {
                    return Err(format!("simulation error: step {}: {err}", step_index + 1));
                }
            };

            let mut step_mismatches = if uses_integrated_rms {
                self.compare_static_tran_primary_reference(reference, plan, &run.netlist, &result)
            } else {
                self.compare_tran_prn_reference(
                    reference,
                    print,
                    &run.netlist,
                    &plan.source,
                    &result,
                    plan.wrapper_tolerance,
                )
            }
            .map_err(|err| format!("reference comparison error: step {}: {err}", step_index + 1))?;
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
            }
            mismatches.extend(step_mismatches);
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                return Ok(mismatches);
            }
            row_offset += reference.rows.len();
        }
        Ok(mismatches)
    }

    pub(super) fn compare_tran_side_outputs(
        &self,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let side_outputs = Self::prn_compatible_tran_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            let reference_path = Self::side_output_reference_path(
                plan.require_waveform_reference_path("transient side-output comparison")?,
                file,
            )?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse transient side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches = self.compare_tran_prn_reference(
                &reference,
                &print,
                netlist,
                &plan.source,
                result,
                plan.wrapper_tolerance,
            )?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn compare_step_tran_side_outputs(
        &self,
        plan: &XyceStaticTranPlan,
        step_runs: &[XyceStepRun],
        abort: &dyn AbortSignal,
        locked_time_grid: bool,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let side_outputs = Self::prn_compatible_tran_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            let reference_path = Self::side_output_reference_path(
                plan.require_waveform_reference_path("stepped transient side-output comparison")?,
                file,
            )?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse transient side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let step_references = Self::split_transient_step_reference(&reference, step_runs.len())
                .map_err(|err| format!("{file}: {err}"))?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches = self.compare_step_tran_runs_with_print(
                plan,
                &print,
                step_runs,
                &step_references,
                abort,
                locked_time_grid,
                false,
            )?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn compare_gnuplot_splot_side_output_batches(
        &self,
        plan: &XyceExecutionPlan,
        batches: &[XyceDcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let (_, side) = Self::gnuplot_splot_print_pair(&plan.source)?;
        if side.probes != plan.print.probes {
            return Err("SPLOT side-output probes differ from primary GNUPLOT probes".to_string());
        }
        let file = side
            .file
            .as_deref()
            .ok_or_else(|| "SPLOT side-output request has no FILE= target".to_string())?;
        let side_reference_path =
            Self::side_output_reference_candidate(&plan.reference_path, file)?;
        let reference_path = if side_reference_path.is_file() {
            side_reference_path
        } else {
            plan.reference_path.clone()
        };
        let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
            format!(
                "failed to parse SPLOT side-output oracle {}: {err}",
                self.display_path(&reference_path)
            )
        })?;
        let side_print = XycePrintRequest {
            probes: side.probes,
        };
        let mut mismatches = self.compare_dc_prn_reference_batches(
            &reference,
            &side_print,
            &plan.source,
            &plan.dc,
            batches,
        )?;
        for mismatch in &mut mismatches {
            mismatch.probe = format!("{file}:{}", mismatch.probe);
        }
        Ok(mismatches)
    }

    pub(super) fn compare_prn_compatible_side_output_batches(
        &self,
        plan: &XyceExecutionPlan,
        batches: &[XyceDcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let side_outputs = Self::prn_compatible_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            let reference_path = Self::side_output_reference_path(&plan.reference_path, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches = self.compare_dc_prn_reference_batches(
                &reference,
                &print,
                &plan.source,
                &plan.dc,
                batches,
            )?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn compare_resistor_dtemp_tables(
        &self,
        reference: &XycePrnTable,
        owner: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns != owner.columns
            || reference.columns.len() != 3
            || reference.columns[0] != "Index"
        {
            return Err(format!(
                "upstream-compatible stepped PRN columns differ or are noncanonical: reference={:?}, owner={:?}",
                reference.columns, owner.columns
            ));
        }
        if reference.rows.len() != 18 || owner.rows.len() != reference.rows.len() {
            return Err(format!(
                "upstream-compatible stepped PRN row census must be 18, got reference={} owner={}",
                reference.rows.len(),
                owner.rows.len()
            ));
        }

        // The removed wrapper's first and effective oracle is `diff` over the
        // two default PRNs. It supplies no good-result artifact for the
        // attempted xyce_verify fallback, so this adapter intentionally keeps
        // segment order, per-step Index resets, columns, and serialized numeric
        // payload exact instead of admitting a tolerant substitute.
        let mut mismatches = Vec::new();
        for (row_index, (expected_row, actual_row)) in
            reference.rows.iter().zip(&owner.rows).enumerate()
        {
            if expected_row.len() != reference.columns.len()
                || actual_row.len() != owner.columns.len()
            {
                return Err(format!(
                    "stepped PRN row {row_index} has a noncanonical width"
                ));
            }
            let expected_index = (row_index % 6) as Value;
            if expected_row[0].to_bits() != expected_index.to_bits()
                || actual_row[0].to_bits() != expected_index.to_bits()
            {
                return Err(format!(
                    "stepped PRN row {row_index} does not preserve the canonical per-segment Index reset"
                ));
            }
            for (column_index, (&expected, &actual)) in
                expected_row.iter().zip(actual_row).enumerate()
            {
                let expected_text = Self::xyce_default_prn_text(expected)?;
                let actual_text = Self::xyce_default_prn_text(actual)?;
                if expected_text != actual_text {
                    let scale = expected.abs().max(actual.abs()).max(Value::MIN_POSITIVE);
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: reference.columns[column_index].clone(),
                        expected,
                        actual,
                        relative_error: (expected - actual).abs() / scale,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_bug647_resistor_tables(
        &self,
        reference: &XycePrnTable,
        owner: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        const ABS_TOL: Value = 1.0e-6;
        const REL_TOL: Value = 0.01;
        const ZERO_TOL: Value = 1.0e-12;
        let reference_tokens = Self::bug647_default_prn_token_lines(reference)?;
        let owner_tokens = Self::bug647_default_prn_token_lines(owner)?;
        if reference_tokens.len() != 1_622
            || owner_tokens.len() != 1_622
            || reference_tokens[0] != owner_tokens[0]
            || reference_tokens[1_621] != owner_tokens[1_621]
        {
            return Err("paired default PRNs do not have identical 1622-line header/data/footer token streams".into());
        }
        if reference.columns != owner.columns
            || reference.columns.len() != 7
            || reference.columns[0] != "Index"
            || reference.rows.len() != 1_620
            || owner.rows.len() != 1_620
        {
            return Err(format!(
                "paired default PRNs require identical seven-column headers and 1620 rows, got reference={:?}/{} owner={:?}/{}",
                reference.columns,
                reference.rows.len(),
                owner.columns,
                owner.rows.len()
            ));
        }
        let mut mismatches = Vec::new();
        for (row_index, (expected_row, actual_row)) in
            reference.rows.iter().zip(&owner.rows).enumerate()
        {
            if expected_row.len() != 7 || actual_row.len() != 7 {
                return Err(format!("paired PRN row {row_index} is not seven columns"));
            }
            let expected_index = (row_index % 6) as Value;
            if expected_row[0].to_bits() != expected_index.to_bits()
                || actual_row[0].to_bits() != expected_index.to_bits()
            {
                return Err(format!(
                    "paired PRN row {row_index} does not reset Index for each STEP coordinate"
                ));
            }
            for (column_index, (&expected, &actual)) in
                expected_row.iter().zip(actual_row).enumerate()
            {
                let expected = Self::xyce_default_prn_roundtrip(expected)?;
                let actual = Self::xyce_default_prn_roundtrip(actual)?;
                let exact = actual == expected;
                let both_zero = actual.abs() <= ZERO_TOL && expected.abs() <= ZERO_TOL;
                let absolute_error = (actual - expected).abs();
                let relative_error = absolute_error / expected.abs();
                let within_both = absolute_error < ABS_TOL && relative_error < REL_TOL;
                // Reproduce Release 7.10 file_compare.pl literally. Its FFT
                // phase clause omits an outer abs around `abs(value)-180`.
                let phase_clause =
                    (expected.abs() - 180.0) < ABS_TOL && (actual.abs() - 180.0) < ABS_TOL;
                if exact || both_zero || within_both || phase_clause {
                    continue;
                }
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: reference.columns[column_index].clone(),
                    expected,
                    actual,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }
        }
        Ok(mismatches)
    }

    /// Reproduce the numeric-table comparison performed by Xyce Release
    /// 7.10's `file_compare.pl`.
    ///
    /// `gold` is intentionally the denominator of the asymmetric relative
    /// error calculation. Values are compared only after the default Xyce
    /// PRN serialization round trip, matching the Perl script's view of the
    /// generated files. The final clause preserves the script's historical
    /// FFT-phase bug literally: it omitted an outer absolute value around
    /// `abs(value) - 180`.
    pub(super) fn compare_release_7_10_file_compare_tables(
        &self,
        gold: &XycePrnTable,
        test: &XycePrnTable,
        tolerance: XyceFileCompareTolerance,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let tolerance = tolerance.validate()?;
        if gold.columns != test.columns {
            return Err(format!(
                "Release 7.10 file_compare headers differ: gold {:?}, test {:?}",
                gold.columns, test.columns
            ));
        }
        if gold.rows.len() != test.rows.len() {
            return Err(format!(
                "Release 7.10 file_compare row counts differ: gold {}, test {}",
                gold.rows.len(),
                test.rows.len()
            ));
        }

        let mut mismatches = Vec::new();
        for (row_index, (gold_row, test_row)) in gold.rows.iter().zip(&test.rows).enumerate() {
            if gold_row.len() != gold.columns.len() || test_row.len() != test.columns.len() {
                return Err(format!(
                    "Release 7.10 file_compare row {row_index} width differs from its header: gold {}/{}, test {}/{}",
                    gold_row.len(),
                    gold.columns.len(),
                    test_row.len(),
                    test.columns.len()
                ));
            }

            for (column_index, (&gold_value, &test_value)) in
                gold_row.iter().zip(test_row).enumerate()
            {
                let probe = &gold.columns[column_index];
                let expected = Self::xyce_default_prn_roundtrip(gold_value).map_err(|error| {
                    format!(
                        "Release 7.10 file_compare cannot serialize gold {probe} at row {row_index}: {error}"
                    )
                })?;
                let actual = Self::xyce_default_prn_roundtrip(test_value).map_err(|error| {
                    format!(
                        "Release 7.10 file_compare cannot serialize test {probe} at row {row_index}: {error}"
                    )
                })?;
                let absolute_error = (actual - expected).abs();
                let relative_error = absolute_error / expected.abs();
                let exact = actual == expected;
                let both_zero = actual.abs() <= tolerance.zero && expected.abs() <= tolerance.zero;
                let within_both =
                    absolute_error < tolerance.absolute && relative_error < tolerance.relative;
                let phase_clause = (expected.abs() - 180.0) < tolerance.absolute
                    && (actual.abs() - 180.0) < tolerance.absolute;
                if exact || both_zero || within_both || phase_clause {
                    continue;
                }

                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: probe.clone(),
                    expected,
                    actual,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_bug655_continuation_tables(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let good_tokens = Self::bug655_default_prn_token_lines(good)?;
        let test_tokens = Self::bug655_default_prn_token_lines(test)?;
        if good_tokens.len() != 23
            || test_tokens.len() != 23
            || !good_tokens[0]
                .iter()
                .zip(&test_tokens[0])
                .all(|(left, right)| left.eq_ignore_ascii_case(right))
            || good_tokens[22] != test_tokens[22]
        {
            return Err(
                "BUG 655 paired default PRNs do not have matching complete header/data/footer layouts"
                    .into(),
            );
        }
        if good.columns.len() != 3
            || test.columns.len() != 3
            || good.rows.len() != 21
            || test.rows.len() != 21
            || !good
                .columns
                .iter()
                .zip(&test.columns)
                .all(|(left, right)| left.eq_ignore_ascii_case(right))
        {
            return Err(format!(
                "BUG 655 xyce_verify requires matching three-column, 21-row PRNs, got good={:?}/{} test={:?}/{}",
                good.columns,
                good.rows.len(),
                test.columns,
                test.rows.len()
            ));
        }

        let tolerance = XyceVerifyTransientTolerance::release_7_10_default().validate()?;
        let zero_small = |value: Value| {
            if value.abs() <= tolerance.zero {
                0.0
            } else {
                value
            }
        };
        let mut good_rows = Vec::with_capacity(21);
        let mut test_rows = Vec::with_capacity(21);
        for (row_index, (good_row, test_row)) in good.rows.iter().zip(&test.rows).enumerate() {
            let expected_index = row_index as Value;
            if good_row.len() != 3
                || test_row.len() != 3
                || good_row[0].to_bits() != expected_index.to_bits()
                || test_row[0].to_bits() != expected_index.to_bits()
            {
                return Err(format!(
                    "BUG 655 PRN row {row_index} does not preserve the canonical Index and three-field layout"
                ));
            }
            let serialize = |value: Value, role: &str, column: &str| {
                Self::xyce_prn_scientific_roundtrip(value, XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION)
                    .map(zero_small)
                    .map_err(|err| {
                        format!(
                            "could not serialize BUG 655 {role} {column} at row {row_index}: {err}"
                        )
                    })
            };
            good_rows.push([
                serialize(good_row[1], "good", &good.columns[1])?,
                serialize(good_row[2], "good", &good.columns[2])?,
            ]);
            test_rows.push([
                serialize(test_row[1], "test", &test.columns[1])?,
                serialize(test_row[2], "test", &test.columns[2])?,
            ]);
        }

        for (row_index, (good_row, test_row)) in good_rows.iter().zip(&test_rows).enumerate() {
            let expected_sweep = Self::xyce_prn_scientific_roundtrip(
                -100.0e-6 + row_index as Value * 10.0e-6,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            for (role, actual) in [("good", good_row[0]), ("test", test_row[0])] {
                if (actual - expected_sweep).abs() > tolerance.absolute_difference {
                    return Err(format!(
                        "BUG 655 {role} sweep coordinate {row_index} is {actual}, expected precision-8 value {expected_sweep}"
                    ));
                }
            }
        }

        let duration = (test_rows[20][0] - test_rows[0][0]).abs();
        if !duration.is_finite() || duration <= 0.0 {
            return Err(format!(
                "BUG 655 test sweep integration interval is invalid: [{}, {}]",
                test_rows[0][0], test_rows[20][0]
            ));
        }
        let mut squared_errors = [0.0; 21];
        let mut worst_row = 0usize;
        let mut worst_error = 0.0;
        for row_index in 0..21 {
            let normalized_error = Self::xyce_verify_normalized_error_with_tolerance(
                good_rows[row_index][1],
                test_rows[row_index][1],
                tolerance,
            );
            if !normalized_error.is_finite() {
                return Err(format!(
                    "BUG 655 normalized V(3) error is non-finite at row {row_index}"
                ));
            }
            if normalized_error.abs() > worst_error {
                worst_error = normalized_error.abs();
                worst_row = row_index;
            }
            squared_errors[row_index] = normalized_error * normalized_error;
        }
        let mut integrated_error = 0.0;
        for row_index in 1..21 {
            let width = (test_rows[row_index][0] - test_rows[row_index - 1][0]).abs();
            integrated_error +=
                0.5 * (squared_errors[row_index] + squared_errors[row_index - 1]) * width;
        }
        let rms_error = (integrated_error / duration).sqrt();
        if !rms_error.is_finite() {
            return Err("BUG 655 integrated V(3) RMS error is non-finite".into());
        }
        if rms_error <= 1.0 {
            return Ok(Vec::new());
        }
        Ok(vec![XyceValueMismatch {
            row: worst_row,
            probe: good.columns[2].clone(),
            expected: good_rows[worst_row][1],
            actual: test_rows[worst_row][1],
            relative_error: rms_error,
        }])
    }

    pub(super) fn compare_bug662_header_tables(
        &self,
        reference: &XycePrnTable,
        owner: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let reference_lines = Self::bug662_default_prn_token_lines(reference)?;
        let owner_lines = Self::bug662_default_prn_token_lines(owner)?;
        Self::validate_bug662_default_prn_token_lines(&reference_lines, reference.rows.len())?;
        Self::validate_bug662_default_prn_token_lines(&owner_lines, owner.rows.len())?;
        if reference_lines.first() != owner_lines.first()
            || reference_lines.last() != owner_lines.last()
        {
            return Err(
                "BUG 662 default PRNs do not preserve the same canonical header and completion footer"
                    .into(),
            );
        }
        self.compare_xyce_verify_transient_tables(reference, owner)
    }

    pub(super) fn compare_bug667_nodeset_tables(
        &self,
        owner: &XycePrnTable,
        reference: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        const COLUMNS: [&str; 8] = [
            "Index",
            "TIME",
            "V(N15206)",
            "V(N15971)",
            "V(N15554)",
            "V(N15997)",
            "V(N16554)",
            "V(N16997)",
        ];
        if owner.columns != COLUMNS
            || reference.columns != COLUMNS
            || owner.rows.len() < 2
            || reference.rows.len() < 2
        {
            return Err(format!(
                "BUG 667 default PRNs require the exact eight-column header and complete nonempty transient domains: owner={:?}/{} reference={:?}/{}",
                owner.columns,
                owner.rows.len(),
                reference.columns,
                reference.rows.len()
            ));
        }

        // The upstream oracle is plain `diff`, not xyce_verify. The shared
        // exact serializer comparison requires equal row counts, canonical
        // Index sequences, case-sensitive columns, and byte-identical
        // precision-8 scientific tokens for every TIME and probe value.
        self.compare_serialized_default_prn_tables(owner, reference)
    }

    pub(super) fn compare_bug754_global_parameter_tables(
        &self,
        reference: &XycePrnTable,
        owner: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let reference_fingerprint = Self::bug754_default_prn_serialization_fingerprint(reference)?;
        let owner_fingerprint = Self::bug754_default_prn_serialization_fingerprint(owner)?;
        if reference_fingerprint == owner_fingerprint {
            return Ok(Vec::new());
        }
        let mismatches = self.compare_serialized_default_prn_tables(reference, owner)?;
        if mismatches.is_empty() {
            Err(
                "serialized default PRNs differ outside their numeric data tokens despite equal tables"
                    .into(),
            )
        } else {
            Ok(mismatches)
        }
    }

    pub(super) fn compare_analytic_integer_dc_table(
        &self,
        actual: &XycePrnTable,
        kind: XyceAnalyticIntegerKind,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let expected_width = match kind {
            XyceAnalyticIntegerKind::Fmod => 3,
            XyceAnalyticIntegerKind::IntFloorCeil => 5,
        };
        if actual.columns.len() != expected_width
            || actual
                .columns
                .first()
                .is_none_or(|column| column != "Index")
            || actual.rows.is_empty()
        {
            return Err(format!(
                "analytic integer DC output requires a nonempty indexed table with {expected_width} columns, got {:?} and {} row(s)",
                actual.columns,
                actual.rows.len()
            ));
        }

        self.compare_analytic_integer_rows(actual, kind, 1, "analytic integer DC")
    }

    pub(super) fn compare_analytic_int_floor_ceil_tran_table(
        &self,
        actual: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if actual.columns.len() != 6
            || actual
                .columns
                .first()
                .is_none_or(|column| column != "Index")
            || actual.columns.get(1).is_none_or(|column| column != "TIME")
            || actual.rows.is_empty()
        {
            return Err(format!(
                "analytic INT/FLOOR/CEIL transient output requires a nonempty Index/TIME/input/int/floor/ceil table, got {:?} and {} row(s)",
                actual.columns,
                actual.rows.len()
            ));
        }
        self.compare_analytic_integer_rows(
            actual,
            XyceAnalyticIntegerKind::IntFloorCeil,
            2,
            "analytic INT/FLOOR/CEIL transient",
        )
    }

    fn compare_analytic_integer_rows(
        &self,
        actual: &XycePrnTable,
        kind: XyceAnalyticIntegerKind,
        input_column: usize,
        label: &str,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let output_count = match kind {
            XyceAnalyticIntegerKind::Fmod => 1,
            XyceAnalyticIntegerKind::IntFloorCeil => 3,
        };
        let expected_width = input_column + 1 + output_count;
        if actual.columns.len() != expected_width {
            return Err(format!(
                "{label} output requires {expected_width} columns, got {}",
                actual.columns.len()
            ));
        }
        let mut mismatches = Vec::new();
        for (row_index, row) in actual.rows.iter().enumerate() {
            if row.len() != expected_width
                || row[0].to_bits() != (row_index as Value).to_bits()
                || row.iter().any(|value| !value.is_finite())
            {
                return Err(format!(
                    "{label} row {row_index} is malformed, nonfinite, or has a noncanonical index"
                ));
            }
            // The Release 7.10 Perl wrappers consume whitespace-split tokens
            // from Xyce's already-written default PRN. Round-trip every input
            // and DUT output through that exact serialization boundary before
            // applying their numeric-equality checks.
            let printed_input = Self::xyce_default_prn_roundtrip(row[input_column])?;
            let expected = match kind {
                XyceAnalyticIntegerKind::Fmod => vec![99.5 % printed_input],
                XyceAnalyticIntegerKind::IntFloorCeil => vec![
                    printed_input.trunc(),
                    printed_input.floor(),
                    printed_input.ceil(),
                ],
            };
            for (offset, expected_value) in expected.into_iter().enumerate() {
                let column = input_column + offset + 1;
                let actual_value = Self::xyce_default_prn_roundtrip(row[column])?;
                if expected_value == actual_value {
                    continue;
                }
                let scale = expected_value
                    .abs()
                    .max(actual_value.abs())
                    .max(Value::MIN_POSITIVE);
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: actual.columns[column].clone(),
                    expected: expected_value,
                    actual: actual_value,
                    relative_error: (expected_value - actual_value).abs() / scale,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_strict_ac_family_snapshots(
        baseline: &XyceStrictAcFamilySnapshot,
        target: &XyceStrictAcFamilySnapshot,
    ) -> Result<(), String> {
        match (baseline, target) {
            (
                XyceStrictAcFamilySnapshot::AbmFrequency(baseline),
                XyceStrictAcFamilySnapshot::AbmFrequency(target),
            ) => Self::compare_abm_frequency_snapshots(baseline, target),
            (
                XyceStrictAcFamilySnapshot::Bug1043AcDataParameters(baseline),
                XyceStrictAcFamilySnapshot::Bug1043AcDataParameters(target),
            ) => Self::compare_bug1043_ac_data_parameter_snapshots(baseline, target),
            (
                XyceStrictAcFamilySnapshot::AcAnalysisExpression(baseline),
                XyceStrictAcFamilySnapshot::AcAnalysisExpression(target),
            ) => Self::compare_ac_analysis_expression_snapshots(baseline, target),
            _ => Err("baseline and target use different exact-AC snapshot kinds".to_string()),
        }
    }

    pub(super) fn compare_abm_frequency_snapshots(
        baseline: &XyceAbmFrequencySnapshot,
        target: &XyceAbmFrequencySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceAbmFrequencyRepresentation::DataTableControl
            || target.representation != XyceAbmFrequencyRepresentation::RuntimeDecadeExpression
        {
            return Err(
                "ABM_FREQ must compare one DATA control against one runtime DEC owner".to_string(),
            );
        }
        if baseline.kind != target.kind
            || baseline.variable != target.variable
            || baseline.frequency_bits != target.frequency_bits
            || baseline.effective_resistance_bits != target.effective_resistance_bits
            || baseline.source_nodes != target.source_nodes
            || baseline.source_ac_bits != target.source_ac_bits
            || baseline.source_transient_bits != target.source_transient_bits
            || baseline.load_nodes != target.load_nodes
            || baseline.capacitance_bits != target.capacitance_bits
            || baseline.ordered_probes != target.ordered_probes
        {
            return Err(
                "ABM_FREQ owner/control changes its axis, topology, effective load, source, or ordered probes"
                    .to_string(),
            );
        }
        if baseline.runtime_expression.is_some()
            || baseline.data_overrides.len() != XYCE_ABM_FREQUENCY_GRID.len()
            || target.runtime_expression.is_none()
            || !target.data_overrides.is_empty()
        {
            return Err(
                "ABM_FREQ runtime-expression and DATA-row provenance is not canonical".to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_bug1043_ac_data_parameter_snapshots(
        baseline: &XyceBug1043AcDataParameterSnapshot,
        target: &XyceBug1043AcDataParameterSnapshot,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_1043_SON AC DATA parameter family";
        if baseline.representation
            != XyceBug1043AcDataParameterRepresentation::RuntimeExpressionBaseline
            || target.representation != XyceBug1043AcDataParameterRepresentation::DataTableOwner
        {
            return Err(format!(
                "{LABEL} must compare the runtime-expression baseline as GOODFILE against the DATA owner as TESTFILE"
            ));
        }
        if baseline.frequency_bits != target.frequency_bits
            || baseline.effective_rows != target.effective_rows
            || baseline.source_nodes != target.source_nodes
            || baseline.resistor_nodes != target.resistor_nodes
            || baseline.capacitor_nodes != target.capacitor_nodes
            || baseline.ordered_probes != target.ordered_probes
        {
            return Err(format!(
                "{LABEL} changes its frequency grid, effective row values, topology, or ordered probes"
            ));
        }
        if baseline.runtime_expressions.len() != 4
            || !baseline.data_overrides.is_empty()
            || !target.runtime_expressions.is_empty()
            || target.data_overrides.len() != baseline.effective_rows.len()
            || target.data_overrides.iter().any(|row| row.len() != 5)
        {
            return Err(format!(
                "{LABEL} runtime-expression and five-column DATA-row provenance is not canonical"
            ));
        }
        Ok(())
    }

    pub(super) fn compare_strict_dc_family_snapshots(
        baseline: &XyceStrictDcFamilySnapshot,
        target: &XyceStrictDcFamilySnapshot,
    ) -> Result<(), String> {
        match (baseline, target) {
            (
                XyceStrictDcFamilySnapshot::AbmLookupOrder(baseline),
                XyceStrictDcFamilySnapshot::AbmLookupOrder(target),
            ) => Self::compare_abm_lookup_order_snapshots(baseline, target),
            (
                XyceStrictDcFamilySnapshot::BjtExternalNode(baseline),
                XyceStrictDcFamilySnapshot::BjtExternalNode(target),
            ) => Self::compare_bjt_external_node_family_snapshots(baseline, target),
            (
                XyceStrictDcFamilySnapshot::DcAnalysisExpression(baseline),
                XyceStrictDcFamilySnapshot::DcAnalysisExpression(target),
            ) => Self::compare_dc_analysis_expression_snapshots(baseline, target),
            (
                XyceStrictDcFamilySnapshot::DelimitedExpression(baseline),
                XyceStrictDcFamilySnapshot::DelimitedExpression(target),
            ) => Self::compare_delimited_expression_snapshots(baseline, target),
            (
                XyceStrictDcFamilySnapshot::PassivePrimaryValue(baseline),
                XyceStrictDcFamilySnapshot::PassivePrimaryValue(target),
            ) => Self::compare_passive_primary_snapshots(baseline, target),
            (
                XyceStrictDcFamilySnapshot::SubcktParameterPrecedence(baseline),
                XyceStrictDcFamilySnapshot::SubcktParameterPrecedence(target),
            ) => {
                if baseline == target {
                    Ok(())
                } else {
                    Err("flattened resistor-divider semantics differ".to_string())
                }
            }
            (
                XyceStrictDcFamilySnapshot::SubcktParameterResolution(baseline),
                XyceStrictDcFamilySnapshot::SubcktParameterResolution(target),
            ) => {
                if baseline.representation
                    != XyceSubcktParameterResolutionRepresentation::FormalDefaultAndInstanceOverride
                    || target.representation
                        == XyceSubcktParameterResolutionRepresentation::UndefinedBinding
                    || !baseline
                        .parameter_name
                        .eq_ignore_ascii_case(&target.parameter_name)
                {
                    Err(
                        "subcircuit parameter-resolution roles or parameter identity differ"
                            .to_string(),
                    )
                } else if baseline.flattened_elements == target.flattened_elements {
                    Ok(())
                } else {
                    Err("flattened subcircuit parameter-resolution semantics differ".to_string())
                }
            }
            (
                XyceStrictDcFamilySnapshot::NestedIncludeIdentity(baseline),
                XyceStrictDcFamilySnapshot::NestedIncludeIdentity(target),
            ) => {
                if baseline == target {
                    Ok(())
                } else {
                    Err("scope-qualified nested include semantics differ".to_string())
                }
            }
            (
                XyceStrictDcFamilySnapshot::SourceMultiplicity(baseline),
                XyceStrictDcFamilySnapshot::SourceMultiplicity(target),
            ) => Self::compare_source_multiplicity_snapshots(baseline, target),
            _ => Err("baseline and target use different exact-DC snapshot kinds".to_string()),
        }
    }

    pub(super) fn compare_abm_lookup_order_snapshots(
        baseline: &XyceAbmLookupOrderSnapshot,
        target: &XyceAbmLookupOrderSnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceAbmLookupRepresentation::SortedControl
            || target.representation != XyceAbmLookupRepresentation::OutOfOrderOwner
        {
            return Err(
                "ABM_SPLINES lookup ordering must compare a sorted control against its reverse-order wrapper owner"
                    .to_string(),
            );
        }
        if baseline.kind != target.kind
            || baseline.canonical_points_bits != target.canonical_points_bits
            || baseline.elements != target.elements
        {
            return Err(
                "ABM_SPLINES lookup owner/control changes interpolation kind, canonical points, topology, input, or numeric state"
                    .to_string(),
            );
        }
        if baseline.authored_points_bits == target.authored_points_bits {
            return Err(
                "ABM_SPLINES lookup owner/control no longer exercise distinct point orderings"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_source_multiplicity_snapshots(
        baseline: &XyceSourceMultiplicitySnapshot,
        target: &XyceSourceMultiplicitySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceSourceMultiplicityRepresentation::LinearBaseline
            || target.representation == XyceSourceMultiplicityRepresentation::LinearBaseline
        {
            return Err(
                "source multiplicity must compare an explicit 0.2-S baseline against one M=10 owner representation"
                    .to_string(),
            );
        }
        if baseline.analysis != target.analysis
            || baseline.flattened_elements != target.flattened_elements
            || baseline.effective_gain_bits != target.effective_gain_bits
            || baseline.source_nodes != target.source_nodes
            || baseline.control_nodes != target.control_nodes
            || baseline.ordered_probes != target.ordered_probes
        {
            return Err(
                "source multiplicity changes analysis, flattened topology/current law, or ordered probes"
                    .to_string(),
            );
        }
        if baseline.authored_multiplicity_bits != 1.0f64.to_bits()
            || baseline.authored_multiplicity_given
            || baseline.flattened_multiplicity_bits != 1.0f64.to_bits()
            || baseline.flattened_multiplicity_given
            || !baseline.hierarchy_multiplicity_bits.is_empty()
            || target.flattened_multiplicity_bits != 10.0f64.to_bits()
        {
            return Err(
                "source multiplicity baseline or effective owner multiplier state is not canonical"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_passive_primary_snapshots(
        baseline: &XycePassivePrimaryValueSnapshot,
        target: &XycePassivePrimaryValueSnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XycePassivePrimaryRepresentation::Named
            || target.representation != XycePassivePrimaryRepresentation::Positional
        {
            return Err("passive primary-value parity requires named baseline -> positional target representation order".to_string());
        }
        if baseline.device_kind != target.device_kind {
            return Err("passive primary-value device kinds differ".to_string());
        }
        if baseline.title != target.title {
            return Err("circuit titles differ".to_string());
        }
        if baseline.active_source_fingerprint != target.active_source_fingerprint {
            return Err(
                "active source differs outside the admitted primary-value token".to_string(),
            );
        }
        if baseline.model_name != target.model_name
            || baseline.model_type != target.model_type
            || baseline.model_numeric_bits != target.model_numeric_bits
        {
            return Err("passive model identity, type, or numeric state differs".to_string());
        }
        if baseline.elements != target.elements {
            return Err(
                "element topology, waveform, or non-representation values differ".to_string(),
            );
        }
        if baseline.effective_primary_bits != target.effective_primary_bits {
            return Err("effective passive primary values differ".to_string());
        }
        Ok(())
    }

    pub(super) fn compare_passive_temperature_override_snapshots(
        baseline: &XycePassiveTemperatureOverrideSnapshot,
        target: &XycePassiveTemperatureOverrideSnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XycePassiveTemperatureRepresentation::Model
            || !matches!(
                target.representation,
                XycePassiveTemperatureRepresentation::ScalarInstance
                    | XycePassiveTemperatureRepresentation::VectorInstance
            )
        {
            return Err(
                "family must compare model TC1/TC2 with scalar or vector instance precedence"
                    .to_string(),
            );
        }
        if baseline.title != target.title
            || baseline.device_kind != target.device_kind
            || baseline.elements != target.elements
            || baseline.model_name != target.model_name
            || baseline.model_type != target.model_type
            || baseline.model_tnom_bits != target.model_tnom_bits
            || baseline.option_directives != target.option_directives
        {
            return Err(
                "circuit identity, topology, non-overridden values, model binding, or options differ"
                    .to_string(),
            );
        }
        if baseline.winning_tc_bits != target.winning_tc_bits {
            return Err("winning TC1/TC2 coefficients differ".to_string());
        }
        if baseline.effective_primary_bits != target.effective_primary_bits {
            return Err("effective temperature-scaled passive values differ".to_string());
        }
        Ok(())
    }

    pub(super) fn compare_ac_analysis_expression_snapshots(
        baseline: &XyceAcAnalysisExpressionSnapshot,
        target: &XyceAcAnalysisExpressionSnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceAcAnalysisRepresentation::DirectNumeric
            || target.representation != XyceAcAnalysisRepresentation::ParameterExpression
            || !baseline.parameter_bits.is_empty()
            || target.parameter_bits.is_empty()
            || !baseline.footer_suppressed
            || target.footer_suppressed
        {
            return Err(
                "family must compare a direct numeric .AC baseline with a parameter-expression .AC representation"
                    .to_string(),
            );
        }
        if baseline.nonrepresentation_source != target.nonrepresentation_source {
            return Err(
                "circuit, source, output, or non-analysis directive semantics differ".to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_delimited_expression_snapshots(
        baseline: &XyceDelimitedExpressionFamilySnapshot,
        target: &XyceDelimitedExpressionFamilySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceDelimitedExpressionRepresentation::Braced
            || target.representation != XyceDelimitedExpressionRepresentation::SingleQuoted
        {
            return Err(
                "family must compare one pure braced baseline with one pure single-quoted wrapper"
                    .to_string(),
            );
        }
        if baseline.expression_sites != target.expression_sites {
            return Err("expression-site AST bijection differs".to_string());
        }
        if baseline.parameter_bits != target.parameter_bits
            || baseline.elements != target.elements
            || baseline.print_probes != target.print_probes
        {
            return Err("resolved circuit, parameter, or print semantics differ".to_string());
        }
        Ok(())
    }

    pub(super) fn compare_dc_analysis_expression_snapshots(
        baseline: &XyceDcAnalysisExpressionSnapshot,
        target: &XyceDcAnalysisExpressionSnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceDcAnalysisRepresentation::DirectNumeric
            || target.representation != XyceDcAnalysisRepresentation::ParameterExpression
            || !baseline.parameter_bits.is_empty()
            || target.parameter_bits.is_empty()
        {
            return Err(
                "family must compare a direct numeric .DC baseline with a parameter-expression .DC representation"
                    .to_string(),
            );
        }
        if baseline.nonrepresentation_source != target.nonrepresentation_source {
            return Err(
                "circuit, model, output, or non-analysis directive semantics differ".to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_transient_analysis_expression_snapshots(
        baseline: &XyceTransientAnalysisExpressionSnapshot,
        target: &XyceTransientAnalysisExpressionSnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceTransientAnalysisRepresentation::DirectNumeric
            || target.representation != XyceTransientAnalysisRepresentation::ParameterExpression
            || !baseline.parameter_bits.is_empty()
            || target.parameter_bits.is_empty()
        {
            return Err(
                "family must compare a parameter-free numeric .TRAN baseline with a scalar parameter-expression .TRAN target"
                    .to_string(),
            );
        }
        if baseline.title != target.title
            || baseline.elements != target.elements
            || baseline.option_directives != target.option_directives
            || baseline.nonrepresentation_source != target.nonrepresentation_source
        {
            return Err(
                "circuit title, element identity/topology/state, source waveforms, or options differ"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_strict_transient_family_snapshots(
        baseline: &XyceStrictTransientFamilySnapshot,
        target: &XyceStrictTransientFamilySnapshot,
    ) -> Result<(), String> {
        match (baseline, target) {
            (
                XyceStrictTransientFamilySnapshot::AgeCap(baseline),
                XyceStrictTransientFamilySnapshot::AgeCap(target),
            ) => Self::compare_age_cap_family_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::DiodeModelAlias(baseline),
                XyceStrictTransientFamilySnapshot::DiodeModelAlias(target),
            ) => Self::compare_diode_model_alias_family_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::SwitchStateCase(baseline),
                XyceStrictTransientFamilySnapshot::SwitchStateCase(target),
            ) => Self::compare_switch_state_case_family_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::ScopedModel(baseline),
                XyceStrictTransientFamilySnapshot::ScopedModel(target),
            ) if baseline == target => Ok(()),
            (
                XyceStrictTransientFamilySnapshot::ScopedModel(_),
                XyceStrictTransientFamilySnapshot::ScopedModel(_),
            ) => {
                Err("flattened elements or effective nonlinear model parameters differ".to_string())
            }
            (
                XyceStrictTransientFamilySnapshot::SinExpression(baseline),
                XyceStrictTransientFamilySnapshot::SinExpression(target),
            ) => Self::compare_sin_expression_family_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::ParamExpression(baseline),
                XyceStrictTransientFamilySnapshot::ParamExpression(target),
            ) => Self::compare_param_expression_family_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::Params1(baseline),
                XyceStrictTransientFamilySnapshot::Params1(target),
            ) => Self::compare_params1_family_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::NakedAlgebra(baseline),
                XyceStrictTransientFamilySnapshot::NakedAlgebra(target),
            ) => Self::compare_naked_algebra_family_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::Bug1826ThermalParameter(baseline),
                XyceStrictTransientFamilySnapshot::Bug1826ThermalParameter(target),
            ) => Self::compare_bug1826_thermal_parameter_family_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::SourceMultiplicity(baseline),
                XyceStrictTransientFamilySnapshot::SourceMultiplicity(target),
            ) => Self::compare_source_multiplicity_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::PassivePrimaryValue(baseline),
                XyceStrictTransientFamilySnapshot::PassivePrimaryValue(target),
            ) => Self::compare_passive_primary_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::PassiveTemperatureOverride(baseline),
                XyceStrictTransientFamilySnapshot::PassiveTemperatureOverride(target),
            ) => Self::compare_passive_temperature_override_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::TransientAnalysisExpression(baseline),
                XyceStrictTransientFamilySnapshot::TransientAnalysisExpression(target),
            ) => Self::compare_transient_analysis_expression_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::Bug1085(baseline),
                XyceStrictTransientFamilySnapshot::Bug1085(target),
            ) => Self::compare_bug1085_user_function_snapshots(baseline, target),
            (
                XyceStrictTransientFamilySnapshot::Bug38(baseline),
                XyceStrictTransientFamilySnapshot::Bug38(target),
            ) => Self::compare_bug38_family_snapshots(baseline, target),
            _ => Err("baseline and target use different strict family snapshot kinds".to_string()),
        }
    }

    pub(super) fn compare_diode_model_alias_family_snapshots(
        baseline: &XyceDiodeModelAliasFamilySnapshot,
        target: &XyceDiodeModelAliasFamilySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceDiodeModelAliasRepresentation::Canonical
            || target.representation != XyceDiodeModelAliasRepresentation::Alias
        {
            return Err(
                "family must compare the canonical IS/BV/CJO model card to JS/VB/CJ aliases"
                    .to_string(),
            );
        }
        let mut baseline = baseline.clone();
        let mut target = target.clone();
        baseline.representation = XyceDiodeModelAliasRepresentation::Canonical;
        target.representation = XyceDiodeModelAliasRepresentation::Canonical;
        if baseline != target {
            return Err(
                "source bytes, typed topology, model state, probes, or resolved native diode differ outside the admitted alias tokens"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_switch_state_case_family_snapshots(
        baseline: &XyceSwitchStateCaseFamilySnapshot,
        target: &XyceSwitchStateCaseFamilySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceSwitchStateCaseRepresentation::Uppercase
            || target.representation != XyceSwitchStateCaseRepresentation::Lowercase
        {
            return Err(
                "family must compare the canonical uppercase baseline to lowercase initial-state spelling"
                    .to_string(),
            );
        }
        let mut baseline = baseline.clone();
        let mut target = target.clone();
        baseline.representation = XyceSwitchStateCaseRepresentation::Uppercase;
        target.representation = XyceSwitchStateCaseRepresentation::Uppercase;
        if baseline != target {
            return Err(
                "source bytes, typed AST, topology, model state, probes, or resolved switch differ outside the initial-state token case"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_age_cap_family_snapshots(
        baseline: &XyceAgeCapFamilySnapshot,
        target: &XyceAgeCapFamilySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceAgeCapRepresentation::NativeAge
            || target.representation != XyceAgeCapRepresentation::ParameterExpression
        {
            return Err(
                "family must compare native AGE/D capacitance to parameter-expression capacitance"
                    .to_string(),
            );
        }
        if baseline.elements != target.elements
            || baseline.ordered_probes != target.ordered_probes
            || baseline.option_directives != target.option_directives
            || baseline.age_semantics != target.age_semantics
        {
            return Err("resolved topology, sources, effective capacitance, ordered probes, or options differ".to_string());
        }
        Ok(())
    }

    pub(super) fn compare_sin_expression_family_snapshots(
        baseline: &XyceSinExpressionFamilySnapshot,
        target: &XyceSinExpressionFamilySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceSinExpressionRepresentation::IndependentSin
            || target.representation != XyceSinExpressionRepresentation::BehavioralSpiceSin
        {
            return Err(
                "family must compare an independent SIN baseline with a behavioral SPICE_SIN target"
                    .to_string(),
            );
        }
        if baseline.resistor_name != target.resistor_name || baseline.resistor != target.resistor {
            return Err("resistor identity, topology, or value differs".to_string());
        }
        if baseline.source_nodes != target.source_nodes {
            return Err("excitation topology differs".to_string());
        }
        if baseline.waveform_bits != target.waveform_bits {
            return Err("canonical sinusoidal waveform values differ".to_string());
        }
        Ok(())
    }

    pub(super) fn compare_param_expression_family_snapshots(
        baseline: &XyceParamExpressionFamilySnapshot,
        target: &XyceParamExpressionFamilySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceParamExpressionRepresentation::ParameterCoefficient
            || target.representation != XyceParamExpressionRepresentation::LiteralCoefficient
        {
            return Err(
                "family must compare a direct parameter-coefficient baseline with a direct literal-coefficient target"
                    .to_string(),
            );
        }
        if baseline.title != target.title {
            return Err("circuit titles differ".to_string());
        }
        if baseline.parameter_name != target.parameter_name
            || baseline.parameter_bits != target.parameter_bits
        {
            return Err("global parameter identity or value differs".to_string());
        }
        if baseline.subcircuit_name != target.subcircuit_name
            || baseline.subcircuit_ports != target.subcircuit_ports
        {
            return Err("subcircuit identity or ordered ports differ".to_string());
        }
        if baseline.flattened_elements != target.flattened_elements {
            return Err(
                "flattened element identity, topology, or numeric state differs".to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_params1_family_snapshots(
        baseline: &XyceParams1Snapshot,
        target: &XyceParams1Snapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceParams1Representation::LiteralValues
            || target.representation != XyceParams1Representation::GlobalParameters
        {
            return Err(
                "family must compare the direct-literal PARAMS1 baseline with the direct-global-parameter member"
                    .to_string(),
            );
        }
        let mut baseline = baseline.clone();
        let mut target = target.clone();
        baseline.representation = XyceParams1Representation::LiteralValues;
        target.representation = XyceParams1Representation::LiteralValues;
        if baseline != target {
            return Err(
                "circuit title, exact topology, resolved values, transient analysis, or ordered probes differ outside the admitted literal/global-parameter spelling"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_naked_algebra_family_snapshots(
        baseline: &XyceNakedAlgebraSnapshot,
        target: &XyceNakedAlgebraSnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceNakedAlgebraRepresentation::BracedLocalBaseline
            || !matches!(
                target.representation,
                XyceNakedAlgebraRepresentation::MixedLocalParameters
                    | XyceNakedAlgebraRepresentation::MixedGlobalParameters
            )
        {
            return Err(
                "family must compare the braced-literal local baseline with a canonical mixed-expression local/global member"
                    .to_string(),
            );
        }
        let mut baseline = baseline.clone();
        let mut target = target.clone();
        baseline.representation = XyceNakedAlgebraRepresentation::BracedLocalBaseline;
        target.representation = XyceNakedAlgebraRepresentation::BracedLocalBaseline;
        target.title.clone_from(&baseline.title);
        if baseline != target {
            return Err(
                "resolved parameter values, behavioral SPICE_PULSE semantics, topology, transient analysis, TIMEINT options, or ordered probes differ outside the admitted parameter representation and non-semantic title/comments"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_bug1826_thermal_parameter_family_snapshots(
        baseline: &XyceBug1826ThermalParameterSnapshot,
        target: &XyceBug1826ThermalParameterSnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceBug1826ThermalParameterRepresentation::GlobalParameter
            || target.representation != XyceBug1826ThermalParameterRepresentation::LocalParameter
        {
            return Err(
                "family must compare the GLOBAL_PARAM baseline with the ordinary PARAM member"
                    .to_string(),
            );
        }
        let mut baseline = baseline.clone();
        let mut target = target.clone();
        baseline.representation = XyceBug1826ThermalParameterRepresentation::GlobalParameter;
        target.representation = XyceBug1826ThermalParameterRepresentation::GlobalParameter;
        if baseline != target {
            return Err(
                "parameter value, topology, copper material expressions, native thermal state, transient analysis, or ordered probes differ outside the admitted parameter namespace"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn compare_bjt_external_node_family_snapshots(
        baseline: &XyceBjtExternalNodeFamilySnapshot,
        target: &XyceBjtExternalNodeFamilySnapshot,
    ) -> Result<(), String> {
        if baseline.representation == target.representation {
            return Err(
                "both members use the same BJT substrate representation instead of an omitted/explicit pair"
                    .to_string(),
            );
        }
        if baseline.title != target.title {
            return Err("circuit titles differ".to_string());
        }
        if baseline.elements != target.elements {
            return Err("element topology or values differ".to_string());
        }
        if baseline.bjt_model_bits != target.bjt_model_bits {
            return Err("explicit BJT model fields differ".to_string());
        }
        Ok(())
    }

    pub(super) fn xyce_verify_normalized_error_with_tolerance(
        expected: Value,
        actual: Value,
        tolerance: XyceVerifyTransientTolerance,
    ) -> Value {
        let difference = expected - actual;
        if difference.abs() < tolerance.absolute_difference {
            0.0
        } else {
            difference / (tolerance.relative * expected.abs() + tolerance.absolute)
        }
    }

    pub(super) fn compare_xyce_verify_transient_tables(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_xyce_verify_transient_tables_with_abort(
            good,
            test,
            &rspice_core::abort_signal::NoAbort,
        )
    }

    pub(super) fn compare_xyce_verify_transient_tables_with_abort(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_xyce_verify_transient_tables_with_uniform_tolerance_and_abort(
            good,
            test,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            abort,
        )
    }

    pub(super) fn compare_baseline_family_xyce_verify_tables(
        &self,
        kind: XyceBaselineFamilyKind,
        baseline: &XycePrnTable,
        member: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if kind.xyce_verify_member_is_good_waveform() {
            self.compare_xyce_verify_transient_tables(member, baseline)
        } else {
            self.compare_xyce_verify_transient_tables(baseline, member)
        }
    }

    pub(super) fn compare_xyce_verify_transient_tables_with_tolerance(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
        tolerance: XyceVerifyTransientTolerance,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if good.columns.len() != 3 || test.columns.len() != 3 {
            return Err(format!(
                "custom xyce_verify transient tolerance requires exactly one output column, got good={:?}, test={:?}",
                good.columns, test.columns
            ));
        }
        self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            good,
            test,
            tolerance,
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )
    }

    pub(super) fn compare_xyce_verify_transient_tables_with_uniform_tolerance(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
        tolerance: XyceVerifyTransientTolerance,
        scientific_precision: usize,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_xyce_verify_transient_tables_with_uniform_tolerance_and_abort(
            good,
            test,
            tolerance,
            scientific_precision,
            &rspice_core::abort_signal::NoAbort,
        )
    }

    fn compare_xyce_verify_transient_tables_with_uniform_tolerance_and_abort(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
        tolerance: XyceVerifyTransientTolerance,
        scientific_precision: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let tolerance = tolerance.validate()?;
        let probe_count = good.columns.len().saturating_sub(2);
        self.compare_xyce_verify_transient_tables_with_probe_tolerances_and_abort(
            good,
            test,
            &vec![tolerance; probe_count],
            scientific_precision,
            abort,
        )
    }

    pub(super) fn compare_xyce_verify_transient_tables_with_probe_tolerances(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
        tolerances: &[XyceVerifyTransientTolerance],
        scientific_precision: usize,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_xyce_verify_transient_tables_with_probe_tolerances_and_abort(
            good,
            test,
            tolerances,
            scientific_precision,
            &rspice_core::abort_signal::NoAbort,
        )
    }

    fn compare_xyce_verify_transient_tables_with_probe_tolerances_and_abort(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
        tolerances: &[XyceVerifyTransientTolerance],
        scientific_precision: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let ensure_not_aborted = || {
            if abort.is_aborted() {
                Err("xyce_verify transient comparison aborted".to_string())
            } else {
                Ok(())
            }
        };
        ensure_not_aborted()?;
        Self::xyce_prn_scientific_text(0.0, scientific_precision)?;
        if good.columns.len() != test.columns.len()
            || !good
                .columns
                .iter()
                .zip(&test.columns)
                .all(|(left, right)| left.eq_ignore_ascii_case(right))
        {
            return Err(format!(
                "xyce_verify transient columns differ: good={:?}, test={:?}",
                good.columns, test.columns
            ));
        }
        if tolerances.len() != good.columns.len().saturating_sub(2) {
            return Err(format!(
                "xyce_verify transient comparison has {} output column(s), but {} tolerance(s) were supplied",
                good.columns.len().saturating_sub(2),
                tolerances.len()
            ));
        }
        let tolerances = tolerances
            .iter()
            .copied()
            .map(XyceVerifyTransientTolerance::validate)
            .collect::<Result<Vec<_>, _>>()?;
        let quotient_operand_indices =
            Self::xyce_verify_quotient_operand_indices(&good.columns[2..]);
        let serialization_relative_tolerance = 4.0 * 10.0f64.powi(-(scientific_precision as i32));
        for (row_index, row) in test.rows.iter().enumerate() {
            ensure_not_aborted()?;
            for (probe_index, operands) in quotient_operand_indices.iter().enumerate() {
                let Some((numerator_index, divisor_index)) = *operands else {
                    continue;
                };
                let Some((&actual, &numerator, &divisor)) = row
                    .get(probe_index + 2)
                    .zip(row.get(numerator_index + 2))
                    .zip(row.get(divisor_index + 2))
                    .map(|((actual, numerator), divisor)| (actual, numerator, divisor))
                else {
                    return Err(format!(
                        "xyce_verify test row {row_index} is missing quotient or operand columns"
                    ));
                };
                let recomputed = numerator / divisor;
                if !recomputed.is_finite() {
                    continue;
                }
                let consistency_scale = actual
                    .abs()
                    .max(recomputed.abs())
                    .max(tolerances[probe_index].absolute);
                let consistency_error = (actual - recomputed).abs() / consistency_scale;
                if !consistency_error.is_finite()
                    || consistency_error > serialization_relative_tolerance
                {
                    return Err(format!(
                        "printed quotient {} is not algebraically consistent at test row {row_index}: printed={actual} numerator={numerator} divisor={divisor}",
                        good.columns[probe_index + 2]
                    ));
                }
            }
        }

        ensure_not_aborted()?;
        let good_rows = Self::normalized_xyce_verify_transient_rows(
            good,
            "good",
            &tolerances,
            scientific_precision,
        )?;
        ensure_not_aborted()?;
        let test_rows = Self::normalized_xyce_verify_transient_rows(
            test,
            "test",
            &tolerances,
            scientific_precision,
        )?;
        ensure_not_aborted()?;
        if good_rows.len() < 2 || test_rows.len() < 2 {
            return Err(format!(
                "xyce_verify transient RMS requires at least two distinct printed times in both series, found good={} and test={}",
                good_rows.len(),
                test_rows.len()
            ));
        }
        let good_first = good_rows[0].0;
        let good_last = good_rows[good_rows.len() - 1].0;
        let test_first = test_rows[0].0;
        let test_last = test_rows[test_rows.len() - 1].0;
        if good_first > test_first {
            return Err(format!(
                "good transient series starts at {good_first}, after test series start {test_first}"
            ));
        }
        if good_last < test_last {
            return Err(format!(
                "good transient series ends at {good_last}, before test series end {test_last}"
            ));
        }

        let probe_count = good.columns.len() - 2;
        let mut interpolated = Vec::with_capacity(test_rows.len());
        let mut good_high = 0usize;
        for (test_time, _) in &test_rows {
            ensure_not_aborted()?;
            while good_high + 1 < good_rows.len() && good_rows[good_high].0 < *test_time {
                good_high += 1;
            }
            if good_rows[good_high].0 < *test_time {
                return Err(format!(
                    "could not bracket test time {test_time} in the good transient series"
                ));
            }
            if good_rows[good_high].0 == *test_time {
                interpolated.push(good_rows[good_high].1.clone());
                continue;
            }
            let Some(good_low) = good_high.checked_sub(1) else {
                return Err(format!(
                    "could not find a good transient sample below test time {test_time}"
                ));
            };
            let low_time = good_rows[good_low].0;
            let high_time = good_rows[good_high].0;
            let width = high_time - low_time;
            if !width.is_finite() || width <= 0.0 {
                return Err(format!(
                    "good transient interpolation interval [{low_time}, {high_time}] is invalid"
                ));
            }
            let values = (0..probe_count)
                .map(|probe_index| {
                    Self::xyce_verify_linear_interpolate(
                        low_time,
                        high_time,
                        *test_time,
                        good_rows[good_low].1[probe_index],
                        good_rows[good_high].1[probe_index],
                    )
                })
                .collect::<Vec<_>>();
            if values.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "good transient interpolation produced a non-finite value at time {test_time}"
                ));
            }
            interpolated.push(values);
        }

        let mut squared_errors = vec![vec![0.0; probe_count]; test_rows.len()];
        let mut worst_rows = vec![0usize; probe_count];
        let mut worst_errors = vec![0.0; probe_count];
        for (row_index, ((_, test_values), good_values)) in
            test_rows.iter().zip(&interpolated).enumerate()
        {
            ensure_not_aborted()?;
            for probe_index in 0..probe_count {
                let expected = good_values[probe_index];
                let actual = test_values[probe_index];
                let probe_tolerance = tolerances[probe_index];
                let normalized_error = if let Some((numerator_index, divisor_index)) =
                    quotient_operand_indices[probe_index]
                {
                    let numerator_tolerance = tolerances[numerator_index];
                    let divisor_tolerance = tolerances[divisor_index];
                    if probe_tolerance.offset != 0.0
                        || numerator_tolerance.offset != 0.0
                        || divisor_tolerance.offset != 0.0
                    {
                        // OFFSET destroys the algebraic identity between the
                        // independently shifted quotient and operand traces.
                        // In that case use the official trace-local metric.
                        Self::xyce_verify_normalized_error_with_tolerance(
                            expected,
                            actual,
                            probe_tolerance,
                        )
                    } else {
                        let numerator_error = Self::xyce_verify_normalized_error_with_tolerance(
                            good_values[numerator_index],
                            test_values[numerator_index],
                            numerator_tolerance,
                        );
                        let divisor_error = Self::xyce_verify_normalized_error_with_tolerance(
                            good_values[divisor_index],
                            test_values[divisor_index],
                            divisor_tolerance,
                        );
                        let operand_error = numerator_error.abs().max(divisor_error.abs());
                        let one_sided_conditioning_floor = (divisor_tolerance.absolute
                            / divisor_tolerance.relative)
                            .min(Value::MAX / 2.0);
                        let divisor_conditioning_floor = divisor_tolerance
                            .zero
                            .max(2.0 * one_sided_conditioning_floor);
                        let divisor_is_conditioned = test_values[divisor_index]
                            .abs()
                            .min(good_values[divisor_index].abs())
                            > divisor_conditioning_floor;
                        let interpolated_reference_quotient =
                            good_values[numerator_index] / good_values[divisor_index];
                        let reference_consistency_scale = expected
                            .abs()
                            .max(interpolated_reference_quotient.abs())
                            .max(probe_tolerance.absolute);
                        let reference_consistency_error =
                            (expected - interpolated_reference_quotient).abs()
                                / reference_consistency_scale;
                        let reference_quotient_is_consistent = interpolated_reference_quotient
                            .is_finite()
                            && reference_consistency_error.is_finite()
                            && reference_consistency_error <= serialization_relative_tolerance;
                        if divisor_is_conditioned && reference_quotient_is_consistent {
                            Self::xyce_verify_normalized_error_with_tolerance(
                                expected,
                                actual,
                                probe_tolerance,
                            )
                        } else {
                            operand_error
                        }
                    }
                } else {
                    Self::xyce_verify_normalized_error_with_tolerance(
                        expected,
                        actual,
                        probe_tolerance,
                    )
                };
                if !normalized_error.is_finite() {
                    return Err(format!(
                        "normalized xyce_verify error is non-finite at test row {row_index}, probe {}",
                        good.columns[probe_index + 2]
                    ));
                }
                if normalized_error.abs() > worst_errors[probe_index] {
                    worst_errors[probe_index] = normalized_error.abs();
                    worst_rows[probe_index] = row_index;
                }
                squared_errors[row_index][probe_index] = normalized_error * normalized_error;
            }
        }

        let mut rms_errors = vec![0.0; probe_count];
        let duration = (test_last - test_first).abs();
        if !duration.is_finite() || duration <= 0.0 {
            return Err(format!(
                "test transient integration interval [{test_first}, {test_last}] is invalid"
            ));
        }
        for row_index in 1..test_rows.len() {
            ensure_not_aborted()?;
            let width = (test_rows[row_index].0 - test_rows[row_index - 1].0).abs();
            for probe_index in 0..probe_count {
                rms_errors[probe_index] += 0.5
                    * (squared_errors[row_index][probe_index]
                        + squared_errors[row_index - 1][probe_index])
                    * width;
            }
        }
        for error in &mut rms_errors {
            ensure_not_aborted()?;
            *error = (*error / duration).sqrt();
        }

        let mut mismatches = Vec::new();
        for (probe_index, rms_error) in rms_errors.into_iter().enumerate() {
            ensure_not_aborted()?;
            if !rms_error.is_finite() {
                return Err(format!(
                    "xyce_verify RMS error is non-finite for probe {}",
                    good.columns[probe_index + 2]
                ));
            }
            if rms_error > 1.0 {
                let row = worst_rows[probe_index];
                mismatches.push(XyceValueMismatch {
                    row,
                    probe: good.columns[probe_index + 2].clone(),
                    expected: interpolated[row][probe_index],
                    actual: test_rows[row].1[probe_index],
                    relative_error: rms_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    break;
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_exact_prn_tables(
        &self,
        expected: &XycePrnTable,
        actual: &XycePrnTable,
        expected_raw_time: &[Value],
        actual_raw_time: &[Value],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_exact_prn_tables_with_axis(
            expected,
            actual,
            expected_raw_time,
            actual_raw_time,
            "TIME",
        )
    }

    pub(super) fn compare_serialized_default_prn_tables(
        &self,
        expected: &XycePrnTable,
        actual: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_serialized_default_prn_tables_with_column_case(expected, actual, false)
    }

    pub(super) fn compare_serialized_default_prn_tables_case_insensitive(
        &self,
        expected: &XycePrnTable,
        actual: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_serialized_default_prn_tables_with_column_case(expected, actual, true)
    }

    pub(super) fn compare_serialized_default_prn_tables_with_column_case(
        &self,
        expected: &XycePrnTable,
        actual: &XycePrnTable,
        case_insensitive_columns: bool,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let columns_match = expected.columns.len() == actual.columns.len()
            && expected
                .columns
                .iter()
                .zip(&actual.columns)
                .all(|(expected, actual)| {
                    if case_insensitive_columns {
                        expected.eq_ignore_ascii_case(actual)
                    } else {
                        expected == actual
                    }
                });
        if !columns_match {
            return Err(format!(
                "default .prn columns differ: expected {:?}, actual {:?}",
                expected.columns, actual.columns
            ));
        }
        if expected.columns.len() < 2 || !expected.columns[0].eq_ignore_ascii_case("Index") {
            return Err(format!(
                "default .prn relational output requires an indexed table, got columns {:?}",
                expected.columns
            ));
        }
        if expected.rows.is_empty() || actual.rows.is_empty() {
            return Err(format!(
                "default .prn relational output must be nonempty, got {} baseline row(s) and {} target row(s)",
                expected.rows.len(),
                actual.rows.len()
            ));
        }
        if expected.rows.len() != actual.rows.len() {
            return Err(format!(
                "default .prn row count differs: expected {}, actual {}",
                expected.rows.len(),
                actual.rows.len()
            ));
        }

        let mut mismatches = Vec::new();
        for (row_index, (expected_row, actual_row)) in
            expected.rows.iter().zip(&actual.rows).enumerate()
        {
            if expected_row.len() != expected.columns.len()
                || actual_row.len() != actual.columns.len()
            {
                return Err(format!(
                    "default .prn row {row_index} width differs from its column layout"
                ));
            }
            let canonical_index = row_index as Value;
            if expected_row[0].to_bits() != canonical_index.to_bits()
                || actual_row[0].to_bits() != canonical_index.to_bits()
            {
                return Err(format!(
                    "default .prn row {row_index} does not preserve the canonical Index sequence: expected table {}, actual table {}",
                    expected_row[0], actual_row[0]
                ));
            }
            for column_index in 1..expected.columns.len() {
                let expected_value = expected_row[column_index];
                let actual_value = actual_row[column_index];
                let expected_text = Self::xyce_default_prn_text(expected_value).map_err(|err| {
                    format!(
                        "could not serialize baseline row {row_index} column '{}': {err}",
                        expected.columns[column_index]
                    )
                })?;
                let actual_text = Self::xyce_default_prn_text(actual_value).map_err(|err| {
                    format!(
                        "could not serialize target row {row_index} column '{}': {err}",
                        actual.columns[column_index]
                    )
                })?;
                if expected_text == actual_text {
                    continue;
                }
                let scale = expected_value
                    .abs()
                    .max(actual_value.abs())
                    .max(Value::MIN_POSITIVE);
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: expected.columns[column_index].clone(),
                    expected: expected_value,
                    actual: actual_value,
                    relative_error: (expected_value - actual_value).abs() / scale,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_exact_dc_prn_tables(
        &self,
        expected: &XycePrnTable,
        actual: &XycePrnTable,
        expected_raw_sweep: &[Value],
        actual_raw_sweep: &[Value],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_exact_prn_tables_with_axis(
            expected,
            actual,
            expected_raw_sweep,
            actual_raw_sweep,
            "DC_SWEEP",
        )
    }

    pub(super) fn compare_exact_prn_tables_with_axis(
        &self,
        expected: &XycePrnTable,
        actual: &XycePrnTable,
        expected_raw_axis: &[Value],
        actual_raw_axis: &[Value],
        axis_name: &str,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if expected_raw_axis.len() != expected.rows.len()
            || actual_raw_axis.len() != actual.rows.len()
        {
            return Err(format!(
                "exact relational raw {axis_name} lengths ({}, {}) do not match table row counts ({}, {})",
                expected_raw_axis.len(),
                actual_raw_axis.len(),
                expected.rows.len(),
                actual.rows.len()
            ));
        }
        if expected_raw_axis.len() != actual_raw_axis.len() {
            return Err(format!(
                "exact relational raw {axis_name} count differs: expected {}, actual {}",
                expected_raw_axis.len(),
                actual_raw_axis.len()
            ));
        }
        let mut axis_mismatches = Vec::new();
        for (row, (&expected, &actual)) in expected_raw_axis.iter().zip(actual_raw_axis).enumerate()
        {
            if !expected.is_finite() || !actual.is_finite() {
                return Err(format!(
                    "exact relational raw {axis_name} row {row} contains non-finite value(s): expected {expected}, actual {actual}"
                ));
            }
            if expected.to_bits() != actual.to_bits() {
                axis_mismatches.push(XyceValueMismatch {
                    row,
                    probe: axis_name.to_string(),
                    expected,
                    actual,
                    relative_error: (expected - actual).abs()
                        / expected.abs().max(actual.abs()).max(Value::MIN_POSITIVE),
                });
                if axis_mismatches.len() >= self.config.max_mismatches {
                    return Ok(axis_mismatches);
                }
            }
        }
        if !axis_mismatches.is_empty() {
            return Ok(axis_mismatches);
        }

        if expected.columns != actual.columns {
            return Err(format!(
                "exact relational columns differ: expected {:?}, actual {:?}",
                expected.columns, actual.columns
            ));
        }
        if expected.rows.len() != actual.rows.len() {
            return Err(format!(
                "exact relational row count differs: expected {}, actual {}",
                expected.rows.len(),
                actual.rows.len()
            ));
        }

        let mut mismatches = Vec::new();
        for (row_index, (expected_row, actual_row)) in
            expected.rows.iter().zip(&actual.rows).enumerate()
        {
            if expected_row.len() != expected.columns.len()
                || actual_row.len() != actual.columns.len()
            {
                return Err(format!(
                    "exact relational row {row_index} width differs from its column layout"
                ));
            }
            for (column_index, (&expected_value, &actual_value)) in
                expected_row.iter().zip(actual_row).enumerate()
            {
                if !expected_value.is_finite() || !actual_value.is_finite() {
                    return Err(format!(
                        "exact relational row {row_index} column '{}' contains non-finite value(s): expected {expected_value}, actual {actual_value}",
                        expected.columns[column_index]
                    ));
                }
                if expected_value.to_bits() == actual_value.to_bits() {
                    continue;
                }
                let scale = expected_value
                    .abs()
                    .max(actual_value.abs())
                    .max(f64::MIN_POSITIVE);
                let relative_error = (expected_value - actual_value).abs() / scale;
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: expected.columns[column_index].clone(),
                    expected: expected_value,
                    actual: actual_value,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_dc_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        dc: &XyceDcSweep,
        results: &[DcSweepPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let batches = [XyceDcResultBatch {
            netlist: netlist.clone(),
            results: results.to_vec(),
        }];
        self.compare_dc_prn_reference_batches(reference, print, source, dc, &batches)
    }

    pub(super) fn compare_dc_data_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        source: &str,
        dc: &XyceDcSweep,
        points: &[XyceDcDataPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }
        let has_stepnum_column = reference.columns[0].eq_ignore_ascii_case("STEPNUM");
        let index_column = usize::from(has_stepnum_column);
        let has_index_column = reference
            .columns
            .get(index_column)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"));
        let data_column_offset = usize::from(has_stepnum_column) + usize::from(has_index_column);
        if data_column_offset == 0 && !Self::reference_columns_are_compact_probe_table(reference) {
            return Err(format!(
                "expected first Xyce .prn column to be Index, STEPNUM, or a compact probe label, got '{}'",
                reference.columns[0]
            ));
        }
        if reference.rows.len() != points.len() {
            return Err(format!(
                "reference row count ({}) does not match .DC DATA row count ({})",
                reference.rows.len(),
                points.len()
            ));
        }

        let mapping_netlist = points
            .first()
            .map(|point| &point.netlist)
            .ok_or_else(|| ".DC DATA comparison has no result rows".to_string())?;
        let data_columns = self.reference_data_columns(
            reference,
            print,
            mapping_netlist,
            data_column_offset,
            has_index_column,
        )?;
        let comp_tolerances = self.comp_tolerances(source, &data_columns)?;
        let print_precisions = Self::dc_print_precisions(source)?;

        let mut mismatches = Vec::new();
        for (row_index, point) in points.iter().enumerate() {
            let row = reference
                .rows
                .get(row_index)
                .ok_or_else(|| format!("missing reference row for .DC DATA row {row_index}"))?;
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if has_stepnum_column {
                let expected_stepnum = row[0];
                if (expected_stepnum - 0.0).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "STEPNUM".to_string(),
                        expected: expected_stepnum,
                        actual: 0.0,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
            if has_index_column {
                let expected_index = row[index_column];
                let actual_index = row_index as Value;
                if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: actual_index,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }

            let sweep_point = XyceDcSweepPoint {
                primary: point.point.sweep_value,
                secondary: None,
            };
            let node_voltages = Self::dc_node_voltage_index(&point.point.result);
            for (column_index, column) in data_columns.iter().enumerate() {
                let expected = row[column_index + data_column_offset];
                let (probe, actual) = match column {
                    XyceReferenceColumn::PrimarySweep { name } => {
                        (name.as_str(), sweep_point.primary)
                    }
                    XyceReferenceColumn::Probe { name } => (
                        name.as_str(),
                        Self::evaluate_dc_probe_with_node_voltage_index(
                            name,
                            &point.netlist,
                            dc,
                            sweep_point,
                            &point.point.result,
                            &point.point.device_op_report,
                            &node_voltages,
                        )?,
                    ),
                };
                let actual = Self::quantize_dc_print_value_with_precisions(
                    &print_precisions,
                    probe,
                    actual,
                )?;
                let normalized_probe = Self::normalize_probe(probe);
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: probe.to_string(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    pub(super) fn compare_dc_prn_reference_batches(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        source: &str,
        dc: &XyceDcSweep,
        batches: &[XyceDcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }
        let has_stepnum_column = reference.columns[0].eq_ignore_ascii_case("STEPNUM");
        let index_column = usize::from(has_stepnum_column);
        let has_index_column = reference
            .columns
            .get(index_column)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"));
        let data_column_offset = usize::from(has_stepnum_column) + usize::from(has_index_column);
        if data_column_offset == 0 && !Self::reference_columns_are_compact_probe_table(reference) {
            return Err(format!(
                "expected first Xyce .prn column to be Index, STEPNUM, or a compact probe label, got '{}'",
                reference.columns[0]
            ));
        }
        let result_count = batches
            .iter()
            .map(|batch| batch.results.len())
            .sum::<usize>();
        if reference.rows.len() != result_count {
            return Err(format!(
                "reference row count ({}) does not match simulation point count ({})",
                reference.rows.len(),
                result_count
            ));
        }

        let mapping_netlist = batches
            .first()
            .map(|batch| &batch.netlist)
            .ok_or_else(|| "DC comparison has no result batches".to_string())?;
        let data_columns = self.reference_data_columns(
            reference,
            print,
            mapping_netlist,
            data_column_offset,
            has_index_column,
        )?;
        let comp_tolerances = self.comp_tolerances(source, &data_columns)?;
        let print_precisions = Self::dc_print_precisions(source)?;
        let primary_points = dc.primary_spec().points();
        if primary_points.is_empty() {
            return Err("primary DC sweep has no points".to_string());
        }
        let secondary_points = dc.sweep2.as_ref().map(|sweep| sweep.spec().points());
        if secondary_points.as_ref().is_some_and(Vec::is_empty) {
            return Err("secondary DC sweep has no points".to_string());
        }
        let mut mismatches = Vec::new();
        let mut global_row_index = 0usize;
        for (batch_index, batch) in batches.iter().enumerate() {
            let equation_sweep = if batch.netlist.measurements.is_empty() {
                Vec::new()
            } else {
                batch
                    .results
                    .iter()
                    .map(|point| (point.sweep_value, point.result.clone()))
                    .collect::<Vec<_>>()
            };
            let equation_traces = if equation_sweep.is_empty() {
                Vec::new()
            } else {
                rspice_core::analysis::evaluate_dc_equation_measurements(
                    &batch.netlist,
                    &equation_sweep,
                )?
            };
            let accepted_axis = batch
                .results
                .iter()
                .map(|point| point.sweep_value)
                .collect::<Vec<_>>();
            let segment_starts = secondary_points
                .as_ref()
                .map(|_| {
                    (primary_points.len()..accepted_axis.len())
                        .step_by(primary_points.len())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let measurement_output_traces = Self::measurement_output_traces(
                &batch.netlist,
                &accepted_axis,
                data_columns.iter().filter_map(|column| match column {
                    XyceReferenceColumn::Probe { name } => Some(name.as_str()),
                    XyceReferenceColumn::PrimarySweep { .. } => None,
                }),
                "DC",
                "DC_CONT",
                &segment_starts,
                |trace_netlist| {
                    rspice_core::analysis::evaluate_dc_continuous_measurements(
                        trace_netlist,
                        &equation_sweep,
                    )
                },
            )?;
            for (local_row_index, point) in batch.results.iter().enumerate() {
                let row = reference.rows.get(global_row_index).ok_or_else(|| {
                    format!("missing reference row for simulation row {global_row_index}")
                })?;
                if row.len() != reference.columns.len() {
                    return Err(format!(
                        "row {} has {} values, expected {}",
                        global_row_index,
                        row.len(),
                        reference.columns.len()
                    ));
                }
                if has_stepnum_column {
                    let expected_stepnum = row[0];
                    let actual_stepnum = batch_index as f64;
                    if (expected_stepnum - actual_stepnum).abs() > self.config.absolute_tolerance {
                        mismatches.push(XyceValueMismatch {
                            row: global_row_index,
                            probe: "STEPNUM".to_string(),
                            expected: expected_stepnum,
                            actual: actual_stepnum,
                            relative_error: 1.0,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return Ok(mismatches);
                        }
                    }
                }
                if has_index_column {
                    let expected_index = row[index_column];
                    let actual_index = local_row_index as f64;
                    if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                        mismatches.push(XyceValueMismatch {
                            row: global_row_index,
                            probe: "Index".to_string(),
                            expected: expected_index,
                            actual: actual_index,
                            relative_error: 1.0,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return Ok(mismatches);
                        }
                    }
                }
                let value_offset = data_column_offset;

                let sweep_point = XyceDcSweepPoint {
                    primary: point.sweep_value,
                    secondary: if let Some(points) = secondary_points.as_ref() {
                        let outer_index = local_row_index / primary_points.len();
                        Some(*points.get(outer_index).ok_or_else(|| {
                            format!(
                                "row {global_row_index} maps outside secondary DC sweep point count ({})",
                                points.len()
                            )
                        })?)
                    } else {
                        None
                    },
                };
                let point_netlist = Self::dc_sweep_point_netlist(&batch.netlist, dc, sweep_point)?;
                let probe_netlist = point_netlist.as_ref().unwrap_or(&batch.netlist);
                let node_voltages = Self::dc_node_voltage_index(&point.result);
                for (column_index, column) in data_columns.iter().enumerate() {
                    let expected = row[column_index + value_offset];
                    let (probe, actual) = match column {
                        XyceReferenceColumn::PrimarySweep { name } => {
                            (name.as_str(), sweep_point.primary)
                        }
                        XyceReferenceColumn::Probe { name } => (
                            name.as_str(),
                            if let Some(trace) = equation_traces
                                .iter()
                                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                            {
                                *trace.values.get(local_row_index).ok_or_else(|| {
                                    format!(
                                        "DC equation measure '{}' has no value for row {}",
                                        trace.name, local_row_index
                                    )
                                })?
                            } else if let Some(trace) =
                                measurement_output_traces.get(&name.to_ascii_uppercase())
                            {
                                trace
                                    .iter()
                                    .filter(|(activation_index, _)| {
                                        local_row_index >= *activation_index
                                    })
                                    .map(|(_, value)| *value)
                                    .next_back()
                                    .unwrap_or(0.0)
                            } else {
                                Self::evaluate_dc_probe_with_node_voltage_index(
                                    name,
                                    probe_netlist,
                                    dc,
                                    sweep_point,
                                    &point.result,
                                    &point.device_op_report,
                                    &node_voltages,
                                )?
                            },
                        ),
                    };
                    let actual = Self::quantize_dc_print_value_with_precisions(
                        &print_precisions,
                        probe,
                        actual,
                    )?;
                    let normalized_probe = Self::normalize_probe(probe);
                    let tolerance = comp_tolerances
                        .get(&normalized_probe)
                        .copied()
                        .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                    if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                        mismatches.push(XyceValueMismatch {
                            row: global_row_index,
                            probe: probe.to_string(),
                            expected,
                            actual,
                            relative_error,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return Ok(mismatches);
                        }
                    }
                }
                global_row_index += 1;
            }
        }

        Ok(mismatches)
    }

    pub(super) fn compare_dc_sensitivity_outputs(
        &self,
        plan: &XyceStaticDcSensitivityPlan,
        batches: &[XyceDcResultBatch],
        start: Instant,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let evaluations = self.evaluate_dc_sensitivity_batches(plan, batches, start)?;
        let mut mismatches = self.compare_dc_sensitivity_table(
            plan,
            &plan.reference_path,
            plan.reference_format,
            &plan.print,
            plan.no_index,
            &evaluations,
            None,
        )?;
        for side in &plan.side_outputs {
            let mut side_mismatches = self.compare_dc_sensitivity_table(
                plan,
                &side.reference_path,
                side.reference_format,
                &side.print,
                side.no_index,
                &evaluations,
                Some(&side.file),
            )?;
            mismatches.append(&mut side_mismatches);
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_dc_sensitivity_table(
        &self,
        plan: &XyceStaticDcSensitivityPlan,
        reference_path: &Path,
        reference_format: XyceDcSensitivityReferenceFormat,
        print: &XycePrintRequest,
        no_index: bool,
        evaluations: &[XyceDcSensitivityEvaluation],
        side_file: Option<&str>,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let reference = match reference_format {
            XyceDcSensitivityReferenceFormat::Prn => Self::parse_prn_file(reference_path)?,
            XyceDcSensitivityReferenceFormat::Csv => Self::parse_csv_file(reference_path)?,
        };
        if reference.columns.is_empty() {
            return Err(format!(
                "DC sensitivity oracle {} has no columns",
                reference_path.display()
            ));
        }
        if reference.rows.len() != evaluations.len() {
            return Err(format!(
                "DC sensitivity oracle {} has {} rows, but the simulation produced {} points",
                reference_path.display(),
                reference.rows.len(),
                evaluations.len()
            ));
        }
        let has_stepnum = reference
            .columns
            .first()
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"));
        if has_stepnum && !plan.add_stepnum_col {
            return Err(format!(
                "DC sensitivity oracle {} contains STEPNUM without ADD_STEPNUM_COL",
                reference_path.display()
            ));
        }
        if plan.add_stepnum_col && !plan.dc.steps.is_empty() && !has_stepnum {
            return Err(format!(
                "DC sensitivity oracle {} is missing the requested STEPNUM column",
                reference_path.display()
            ));
        }
        let index_column = usize::from(has_stepnum);
        let data_offset = index_column + usize::from(!no_index);
        if !no_index
            && reference
                .columns
                .get(index_column)
                .is_none_or(|column| !column.eq_ignore_ascii_case("Index"))
        {
            return Err(format!(
                "DC sensitivity oracle {} must contain Index before data columns",
                reference_path.display()
            ));
        }
        if no_index && has_stepnum && reference.columns.len() <= data_offset {
            return Err(format!(
                "DC sensitivity NOINDEX oracle {} has no data columns",
                reference_path.display()
            ));
        }
        let mut expected_generated = BTreeSet::new();
        for objective in &plan.objectives {
            let objective_probe = Self::xyce_sensitivity_objective_probe(&objective.spec);
            expected_generated.insert(Self::dc_sensitivity_nominal_column_name(&objective_probe));
            for parameter in &plan.parameters {
                if plan.direct {
                    expected_generated.insert(Self::dc_sensitivity_derivative_column_name(
                        &objective_probe,
                        parameter,
                        "dir",
                    ));
                }
                if plan.adjoint {
                    expected_generated.insert(Self::dc_sensitivity_derivative_column_name(
                        &objective_probe,
                        parameter,
                        "adj",
                    ));
                }
            }
        }
        let generated_offset = reference
            .columns
            .iter()
            .enumerate()
            .skip(data_offset)
            .find_map(|(index, column)| {
                expected_generated
                    .contains(&Self::normalize_probe(column))
                    .then_some(index)
            })
            .ok_or_else(|| {
                format!(
                    "DC sensitivity oracle {} has no generated objective/sensitivity columns",
                    reference_path.display()
                )
            })?;
        let base_columns = &reference.columns[data_offset..generated_offset];
        if base_columns.len() != print.probes.len() {
            return Err(format!(
                "DC sensitivity oracle {} has {} base columns, but .PRINT SENS declares {} probes",
                reference_path.display(),
                base_columns.len(),
                print.probes.len()
            ));
        }
        for (column, probe) in base_columns.iter().zip(&print.probes) {
            if Self::normalize_probe(column) != Self::normalize_probe(probe) {
                return Err(format!(
                    "DC sensitivity oracle {} base column '{}' does not match .PRINT SENS probe '{}'",
                    reference_path.display(),
                    column,
                    probe
                ));
            }
        }
        let mut generated_indices = BTreeMap::new();
        for (index, column) in reference.columns.iter().enumerate().skip(generated_offset) {
            let normalized = Self::normalize_probe(column);
            if !expected_generated.contains(&normalized) {
                return Err(format!(
                    "DC sensitivity oracle {} contains unsupported generated column '{}'",
                    reference_path.display(),
                    column
                ));
            }
            if generated_indices.insert(normalized, index).is_some() {
                return Err(format!(
                    "DC sensitivity oracle {} contains duplicate generated column '{}'",
                    reference_path.display(),
                    column
                ));
            }
        }
        if generated_indices.len() != expected_generated.len() {
            return Err(format!(
                "DC sensitivity oracle {} has {} generated columns, expected {}",
                reference_path.display(),
                generated_indices.len(),
                expected_generated.len()
            ));
        }
        let mut mismatches = Vec::new();
        for (row_index, (row, evaluation)) in reference.rows.iter().zip(evaluations).enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "DC sensitivity oracle {} row {} has {} values, expected {}",
                    reference_path.display(),
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if has_stepnum {
                let expected = row[0];
                let actual = evaluation.step_index as Value;
                Self::record_sensitivity_mismatch(
                    self,
                    &mut mismatches,
                    row_index,
                    &reference.columns[0],
                    expected,
                    actual,
                );
            }
            if !no_index {
                let expected = row[index_column];
                let actual = evaluation.local_index as Value;
                Self::record_sensitivity_mismatch(
                    self,
                    &mut mismatches,
                    row_index,
                    &reference.columns[index_column],
                    expected,
                    actual,
                );
            }
            let sweep_point = XyceDcSweepPoint {
                primary: evaluation.point.sweep_value,
                secondary: None,
            };
            for (column_index, probe) in print.probes.iter().enumerate() {
                let expected = row[data_offset + column_index];
                let actual = Self::evaluate_dc_probe(
                    probe,
                    &evaluation.netlist,
                    &plan.dc.dc,
                    sweep_point,
                    &evaluation.point.result,
                    &evaluation.point.device_op_report,
                )?;
                let actual = Self::quantize_dc_print_value(&plan.dc.source, probe, actual)?;
                let normalized = Self::normalize_probe(probe);
                let tolerance = self.default_comparison_tolerance(&normalized);
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: side_file
                            .map(|file| format!("{file}:{probe}"))
                            .unwrap_or_else(|| probe.to_string()),
                        expected,
                        actual,
                        relative_error,
                    });
                }
            }
            for (objective_index, objective) in plan.objectives.iter().enumerate() {
                let objective_probe = Self::xyce_sensitivity_objective_probe(&objective.spec);
                let sensitivity = evaluation
                    .objectives
                    .get(objective_index)
                    .ok_or_else(|| "DC sensitivity objective result count mismatch".to_string())?;
                let nominal_key = Self::dc_sensitivity_nominal_column_name(&objective_probe);
                let nominal_column = *generated_indices
                    .get(&nominal_key)
                    .ok_or_else(|| format!("missing generated column '{nominal_key}'"))?;
                let nominal_expected = row[nominal_column];
                let nominal_actual = sensitivity.output_value;
                let nominal_probe = side_file
                    .map(|file| format!("{file}:{}", reference.columns[nominal_column]))
                    .unwrap_or_else(|| reference.columns[nominal_column].clone());
                Self::record_sensitivity_mismatch(
                    self,
                    &mut mismatches,
                    row_index,
                    &nominal_probe,
                    nominal_expected,
                    nominal_actual,
                );
                for parameter in &plan.parameters {
                    let trace = sensitivity
                        .sensitivities
                        .iter()
                        .find(|trace| Self::xyce_dc_sensitivity_trace_matches(trace, parameter))
                        .ok_or_else(|| {
                            format!(
                                "DC sensitivity objective '{}' produced no trace for parameter '{parameter}'",
                                objective.authored_name
                            )
                        })?;
                    for mode in [plan.direct.then_some("dir"), plan.adjoint.then_some("adj")]
                        .into_iter()
                        .flatten()
                    {
                        let key = Self::dc_sensitivity_derivative_column_name(
                            &objective_probe,
                            parameter,
                            mode,
                        );
                        let column = *generated_indices
                            .get(&key)
                            .ok_or_else(|| format!("missing generated column '{key}'"))?;
                        let expected = row[column];
                        let probe = side_file
                            .map(|file| format!("{file}:{}", reference.columns[column]))
                            .unwrap_or_else(|| reference.columns[column].clone());
                        Self::record_sensitivity_mismatch(
                            self,
                            &mut mismatches,
                            row_index,
                            &probe,
                            expected,
                            trace.absolute,
                        );
                    }
                }
            }
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                return Ok(mismatches);
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_ac_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        results: &[AcResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_ac_prn_reference_with_step(reference, print, netlist, source, results, None)
    }

    pub(super) fn compare_ac_sensitivity_outputs(
        &self,
        plan: &XyceStaticAcSensitivityPlan,
        netlist: &Netlist,
        source: &str,
        results: &[AcResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let mut all_mismatches =
            self.compare_ac_sensitivity_prn_reference(plan, netlist, source, results)?;
        for side in &plan.side_outputs {
            let side_plan = XyceStaticAcSensitivityPlan {
                reference_path: side.reference_path.clone(),
                reference_format: side.reference_format,
                print: side.print.clone(),
                objectives: plan.objectives.clone(),
                parameters: plan.parameters.clone(),
                direct: plan.direct,
                adjoint: plan.adjoint,
                no_index: side.no_index,
                side_outputs: Vec::new(),
            };
            let mut mismatches =
                self.compare_ac_sensitivity_prn_reference(&side_plan, netlist, source, results)?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{}:{}", side.file, mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn compare_step_ac_sensitivity_outputs(
        &self,
        plan: &XyceStaticAcSensitivityPlan,
        source: &str,
        points_per_step: usize,
        batches: &[XyceAcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if points_per_step == 0 {
            return Err("AC sensitivity step comparison has no frequency points".to_string());
        }
        let total_rows = batches
            .len()
            .checked_mul(points_per_step)
            .ok_or_else(|| "AC sensitivity step row count overflow".to_string())?;

        let mut outputs = Vec::with_capacity(1 + plan.side_outputs.len());
        outputs.push((None, plan.clone()));
        for side in &plan.side_outputs {
            outputs.push((
                Some(side.file.clone()),
                XyceStaticAcSensitivityPlan {
                    reference_path: side.reference_path.clone(),
                    reference_format: side.reference_format,
                    print: side.print.clone(),
                    objectives: plan.objectives.clone(),
                    parameters: plan.parameters.clone(),
                    direct: plan.direct,
                    adjoint: plan.adjoint,
                    no_index: side.no_index,
                    side_outputs: Vec::new(),
                },
            ));
        }

        let mut all_mismatches = Vec::new();
        for (file, output) in outputs {
            let reference = match output.reference_format {
                XyceAcSensitivityReferenceFormat::Prn => {
                    if Self::source_requests_ac_print_headerless(source) {
                        Self::parse_headerless_ac_sensitivity_prn_file(&output)?
                    } else {
                        Self::parse_prn_file(&output.reference_path)?
                    }
                }
                XyceAcSensitivityReferenceFormat::Csv => {
                    Self::parse_csv_file(&output.reference_path)?
                }
            };
            if reference.rows.len() != total_rows {
                return Err(format!(
                    "AC sensitivity oracle {} has {} rows, but stepped simulation produced {} rows",
                    output.reference_path.display(),
                    reference.rows.len(),
                    total_rows
                ));
            }
            let mut row_offset = 0usize;
            for (step_index, batch) in batches.iter().enumerate() {
                if batch.results.len() != points_per_step {
                    return Err(format!(
                        "AC sensitivity step {} produced {} frequency points, expected {}",
                        step_index,
                        batch.results.len(),
                        points_per_step
                    ));
                }
                let rows = reference.rows[row_offset..row_offset + points_per_step].to_vec();
                let step_reference = XycePrnTable {
                    columns: reference.columns.clone(),
                    rows,
                };
                let mut mismatches = self.compare_ac_sensitivity_table_reference(
                    &output,
                    &batch.netlist,
                    source,
                    &batch.results,
                    &step_reference,
                    Some(step_index),
                )?;
                for mismatch in &mut mismatches {
                    mismatch.row += row_offset;
                    if let Some(file) = file.as_ref() {
                        mismatch.probe = format!("{file}:{}", mismatch.probe);
                    }
                }
                all_mismatches.extend(mismatches);
                row_offset += points_per_step;
                if all_mismatches.len() >= self.config.max_mismatches {
                    all_mismatches.truncate(self.config.max_mismatches);
                    return Ok(all_mismatches);
                }
            }
        }
        Ok(all_mismatches)
    }

    pub(super) fn compare_ac_sensitivity_prn_reference(
        &self,
        plan: &XyceStaticAcSensitivityPlan,
        netlist: &Netlist,
        source: &str,
        results: &[AcResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let reference = match plan.reference_format {
            XyceAcSensitivityReferenceFormat::Prn => {
                if Self::source_requests_ac_print_headerless(source) {
                    Self::parse_headerless_ac_sensitivity_prn_file(plan)?
                } else {
                    Self::parse_prn_file(&plan.reference_path)?
                }
            }
            XyceAcSensitivityReferenceFormat::Csv => Self::parse_csv_file(&plan.reference_path)?,
        };
        self.compare_ac_sensitivity_table_reference(
            plan, netlist, source, results, &reference, None,
        )
    }

    pub(super) fn compare_ac_sensitivity_table_reference(
        &self,
        plan: &XyceStaticAcSensitivityPlan,
        netlist: &Netlist,
        source: &str,
        results: &[AcResult],
        reference: &XycePrnTable,
        expected_stepnum: Option<usize>,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let has_stepnum_column = reference
            .columns
            .first()
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"));
        let index_column = usize::from(has_stepnum_column);
        let frequency_column = index_column + usize::from(!plan.no_index);
        let leading_columns = frequency_column + 1;
        if reference.columns.len() < leading_columns + 2 {
            return Err(if has_stepnum_column && plan.no_index {
                "AC sensitivity oracle must contain STEPNUM, FREQ, and data columns".to_string()
            } else if has_stepnum_column {
                "AC sensitivity oracle must contain STEPNUM, Index, FREQ, and data columns"
                    .to_string()
            } else if plan.no_index {
                "AC sensitivity oracle must contain FREQ and data columns".to_string()
            } else {
                "AC sensitivity oracle must contain Index, FREQ, and data columns".to_string()
            });
        }
        let valid_leading_columns = if plan.no_index {
            reference
                .columns
                .get(index_column)
                .is_some_and(|column| Self::is_ac_frequency_reference_column(column))
        } else {
            reference
                .columns
                .get(index_column)
                .is_some_and(|column| column.eq_ignore_ascii_case("Index"))
                && reference
                    .columns
                    .get(frequency_column)
                    .is_some_and(|column| Self::is_ac_frequency_reference_column(column))
        };
        if !valid_leading_columns {
            return Err(if has_stepnum_column && plan.no_index {
                "AC sensitivity NOINDEX oracle must begin with STEPNUM FREQ".to_string()
            } else if has_stepnum_column {
                "AC sensitivity oracle must begin with STEPNUM Index FREQ columns".to_string()
            } else if plan.no_index {
                "AC sensitivity NOINDEX oracle must begin with FREQ".to_string()
            } else {
                "AC sensitivity oracle must begin with Index FREQ columns".to_string()
            });
        }
        if reference.rows.len() != results.len() {
            return Err(format!(
                "AC sensitivity oracle has {} rows, but the simulation produced {} frequency points",
                reference.rows.len(),
                results.len()
            ));
        }

        let generated_offset = reference
            .columns
            .iter()
            .enumerate()
            .skip(leading_columns)
            .find_map(|(index, column)| {
                Self::is_ac_sensitivity_generated_column(column).then_some(index)
            })
            .ok_or_else(|| {
                "AC sensitivity oracle has no generated objective/sensitivity columns".to_string()
            })?;
        if generated_offset == leading_columns {
            return Err(
                "AC sensitivity oracle has no ordinary .PRINT SENS data columns".to_string(),
            );
        }
        let base_reference = XycePrnTable {
            columns: reference.columns[..generated_offset].to_vec(),
            rows: reference.rows.clone(),
        };
        let base_columns =
            Self::reference_ac_data_columns(&base_reference, &plan.print, leading_columns)?;
        let phase_output_radians = Self::source_requests_ac_phase_output_radians(source);
        let base_comp_columns = base_columns
            .iter()
            .map(|column| XyceReferenceColumn::Probe {
                name: column.probe_name().to_string(),
            })
            .collect::<Vec<_>>();
        let base_tolerances = self.comp_tolerances(source, &base_comp_columns)?;

        let mut generated_indices = BTreeMap::new();
        for (index, column) in reference.columns.iter().enumerate().skip(generated_offset) {
            let normalized = Self::normalize_probe(column);
            if generated_indices
                .insert(normalized.clone(), index)
                .is_some()
            {
                return Err(format!(
                    "AC sensitivity oracle contains duplicate generated column '{column}'"
                ));
            }
            if !Self::is_ac_sensitivity_generated_column(column) {
                return Err(format!(
                    "AC sensitivity oracle contains unsupported generated column '{column}'"
                ));
            }
        }

        let frequencies = results
            .iter()
            .map(|result| result.frequency)
            .collect::<Vec<_>>();
        let engine = self.create_xyce_engine();
        let mut sensitivity_results = Vec::with_capacity(plan.objectives.len());
        let mut expected_generated_columns = BTreeSet::new();
        for objective in &plan.objectives {
            let output = Self::xyce_sensitivity_output_from_spec(&objective.spec, &results[0])?;
            let objective_probe = Self::xyce_sensitivity_objective_probe(&objective.spec);
            for component in ["re", "im", "mag", "ph"] {
                expected_generated_columns.insert(Self::xyce_sensitivity_column_name(
                    component,
                    &objective_probe,
                    None,
                    None,
                ));
            }
            for parameter in &plan.parameters {
                if plan.direct {
                    for component in ["re", "im", "mag", "ph"] {
                        expected_generated_columns.insert(Self::xyce_sensitivity_column_name(
                            component,
                            &objective_probe,
                            Some(parameter),
                            Some("dir"),
                        ));
                    }
                }
                if plan.adjoint {
                    for component in ["re", "im", "mag", "ph"] {
                        expected_generated_columns.insert(Self::xyce_sensitivity_column_name(
                            component,
                            &objective_probe,
                            Some(parameter),
                            Some("adj"),
                        ));
                    }
                }
            }
            let sensitivity = engine
                .run_sensitivity_ac_complete(netlist, output, &frequencies, &plan.parameters)
                .map_err(|error| error.to_string())?;
            if sensitivity.frequencies.len() != frequencies.len()
                || sensitivity.output_values.len() != frequencies.len()
            {
                return Err(format!(
                    "AC sensitivity objective '{}' returned an invalid frequency trace shape",
                    objective.authored_name
                ));
            }
            sensitivity_results.push(sensitivity);
        }
        if generated_indices.len() != expected_generated_columns.len() {
            return Err(format!(
                "AC sensitivity oracle has {} generated columns, expected {} for the declared objectives/parameters",
                generated_indices.len(),
                expected_generated_columns.len()
            ));
        }
        for expected in &expected_generated_columns {
            if !generated_indices.contains_key(expected) {
                return Err(format!(
                    "AC sensitivity oracle is missing generated column '{expected}'"
                ));
            }
        }

        let mut mismatches = Vec::new();
        for (row_index, (row, result)) in reference.rows.iter().zip(results).enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "AC sensitivity oracle row {row_index} has {} values, expected {}",
                    row.len(),
                    reference.columns.len()
                ));
            }
            if has_stepnum_column {
                let expected = row[0];
                let actual = expected_stepnum.map_or(0.0, |step| step as Value);
                if (expected - actual).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "STEPNUM".to_string(),
                        expected,
                        actual,
                        relative_error: 1.0,
                    });
                }
            }
            if !plan.no_index {
                let expected_index = row[index_column];
                if (expected_index - row_index as Value).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: row_index as Value,
                        relative_error: 1.0,
                    });
                }
            }
            if let Some(relative_error) = self.value_mismatch(
                row[frequency_column],
                result.frequency,
                XyceComparisonTolerance::from_config(&self.config),
            ) {
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: reference.columns[frequency_column].clone(),
                    expected: row[frequency_column],
                    actual: result.frequency,
                    relative_error,
                });
            }
            for (column_index, column) in base_columns.iter().enumerate() {
                let expected = row[column_index + leading_columns];
                let actual = Self::evaluate_ac_reference_column(
                    column,
                    netlist,
                    result,
                    phase_output_radians,
                )?;
                let normalized = Self::normalize_probe(column.probe_name());
                let tolerance = base_tolerances
                    .get(&normalized)
                    .copied()
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: reference.columns[column_index + leading_columns].clone(),
                        expected,
                        actual,
                        relative_error,
                    });
                }
            }

            for (objective, sensitivity) in plan.objectives.iter().zip(&sensitivity_results) {
                let objective_probe = Self::xyce_sensitivity_objective_probe(&objective.spec);
                let output = sensitivity.output_values[row_index];
                for component in ["re", "im", "mag", "ph"] {
                    let key =
                        Self::xyce_sensitivity_column_name(component, &objective_probe, None, None);
                    let expected = row[*generated_indices
                        .get(&key)
                        .expect("validated nominal sensitivity column")];
                    let actual = Self::xyce_sensitivity_value(
                        component,
                        output,
                        row_index,
                        None,
                        phase_output_radians,
                    );
                    Self::record_sensitivity_mismatch(
                        self,
                        &mut mismatches,
                        row_index,
                        &reference.columns[*generated_indices
                            .get(&key)
                            .expect("validated nominal sensitivity column")],
                        expected,
                        actual,
                    );
                }
                for parameter in &plan.parameters {
                    let trace = sensitivity
                        .sensitivities
                        .iter()
                        .find(|trace| Self::xyce_sensitivity_trace_matches(trace, parameter))
                        .ok_or_else(|| {
                            format!(
                                "AC sensitivity objective '{}' produced no trace for parameter '{parameter}'",
                                objective.authored_name
                            )
                        })?;
                    for mode in [plan.direct.then_some("dir"), plan.adjoint.then_some("adj")]
                        .into_iter()
                        .flatten()
                    {
                        for component in ["re", "im", "mag", "ph"] {
                            let key = Self::xyce_sensitivity_column_name(
                                component,
                                &objective_probe,
                                Some(parameter),
                                Some(mode),
                            );
                            let column_index = *generated_indices
                                .get(&key)
                                .expect("validated derivative sensitivity column");
                            let expected = row[column_index];
                            let actual = Self::xyce_sensitivity_value(
                                component,
                                output,
                                row_index,
                                Some(trace),
                                phase_output_radians,
                            );
                            Self::record_sensitivity_mismatch(
                                self,
                                &mut mismatches,
                                row_index,
                                &reference.columns[column_index],
                                expected,
                                actual,
                            );
                        }
                    }
                }
            }
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                return Ok(mismatches);
            }
        }
        Ok(mismatches)
    }

    pub(super) fn record_sensitivity_mismatch(
        &self,
        mismatches: &mut Vec<XyceValueMismatch>,
        row: usize,
        probe: &str,
        expected: Value,
        actual: Value,
    ) {
        let normalized = Self::normalize_probe(probe);
        let tolerance = self.default_comparison_tolerance(&normalized);
        if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
            mismatches.push(XyceValueMismatch {
                row,
                probe: probe.to_string(),
                expected,
                actual,
                relative_error,
            });
        }
    }

    pub(super) fn compare_noise_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        results: &[rspice_core::analysis::NoiseResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_noise_prn_reference_with_contexts(
            reference, print, netlist, source, results, None, None,
        )
    }

    pub(super) fn compare_noise_prn_reference_with_step(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        results: &[rspice_core::analysis::NoiseResult],
        expected_step_index: Option<usize>,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_noise_prn_reference_with_contexts(
            reference,
            print,
            netlist,
            source,
            results,
            None,
            expected_step_index,
        )
    }

    pub(super) fn compare_noise_data_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        results: &[rspice_core::analysis::NoiseResult],
        row_netlists: &[Netlist],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if row_netlists.len() != results.len() {
            return Err(format!(
                ".NOISE DATA context count ({}) does not match result count ({})",
                row_netlists.len(),
                results.len()
            ));
        }
        self.compare_noise_prn_reference_with_contexts(
            reference,
            print,
            netlist,
            source,
            results,
            Some(row_netlists),
            None,
        )
    }

    pub(super) fn compare_noise_prn_reference_with_contexts(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        results: &[rspice_core::analysis::NoiseResult],
        row_netlists: Option<&[Netlist]>,
        expected_step_index: Option<usize>,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }
        let mut data_column_offset = 0usize;
        let stepnum_column = reference
            .columns
            .first()
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"))
            .then_some(0usize);
        if stepnum_column.is_some() {
            data_column_offset += 1;
        }
        let index_column = reference
            .columns
            .get(data_column_offset)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"))
            .then_some(data_column_offset);
        if index_column.is_some() {
            data_column_offset += 1;
        }
        let frequency_column = reference
            .columns
            .get(data_column_offset)
            .filter(|column| Self::is_ac_frequency_reference_column(column))
            .map(|_| data_column_offset)
            .ok_or_else(|| {
                format!(
                    "expected Xyce NOISE frequency column at position {}, got '{}'",
                    data_column_offset,
                    reference
                        .columns
                        .get(data_column_offset)
                        .map(String::as_str)
                        .unwrap_or("<missing>")
                )
            })?;
        data_column_offset += 1;
        if reference.rows.len() != results.len() {
            return Err(format!(
                "reference row count ({}) does not match NOISE simulation point count ({})",
                reference.rows.len(),
                results.len()
            ));
        }

        let data_columns = Self::reference_ac_data_columns(reference, print, data_column_offset)?;
        let series = rspice_core::analysis::NoiseSweepSeries::from_sweep(results)?
            .ok_or_else(|| "NOISE simulation produced no points".to_string())?;
        let signals = series.signal_map();
        let equation_traces =
            rspice_core::analysis::evaluate_noise_equation_measurements(netlist, results)?;
        let measurement_output_traces =
            Self::noise_measurement_output_traces(netlist, results, &data_columns)?;
        let comp_columns = data_columns
            .iter()
            .map(|column| XyceReferenceColumn::Probe {
                name: column.probe_name().to_string(),
            })
            .collect::<Vec<_>>();
        let comp_tolerances = self.comp_tolerances(source, &comp_columns)?;
        let frequency_tolerance = XyceComparisonTolerance::from_config(&self.config);

        let mut mismatches = Vec::new();
        for (row_index, (row, result)) in reference.rows.iter().zip(results).enumerate() {
            let row_netlist = row_netlists
                .and_then(|contexts| contexts.get(row_index))
                .unwrap_or(netlist);
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if let Some(stepnum_column) = stepnum_column {
                let expected = row[stepnum_column];
                let actual = expected_step_index.unwrap_or(0) as Value;
                if (expected - actual).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "STEPNUM".to_string(),
                        expected,
                        actual,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
            if let Some(index_column) = index_column {
                let expected = row[index_column];
                let actual = row_index as Value;
                if (expected - actual).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected,
                        actual,
                        relative_error: 1.0,
                    });
                }
            }
            let expected_frequency = row[frequency_column];
            if let Some(relative_error) =
                self.value_mismatch(expected_frequency, result.frequency, frequency_tolerance)
            {
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: reference.columns[frequency_column].clone(),
                    expected: expected_frequency,
                    actual: result.frequency,
                    relative_error,
                });
            }
            for (column_index, column) in data_columns.iter().enumerate() {
                let probe = column.probe_name();
                let signal_probe = Self::noise_reference_signal_probe(column)?;
                let expected = row[column_index + data_column_offset];
                let actual = if column.component() == XyceAcProbeComponent::Scalar
                    && let Some(trace) = equation_traces
                        .iter()
                        .find(|trace| trace.name.eq_ignore_ascii_case(probe))
                {
                    *trace.values.get(row_index).ok_or_else(|| {
                        format!(
                            "NOISE equation measure '{}' has no value for row {}",
                            trace.name, row_index
                        )
                    })?
                } else if column.component() == XyceAcProbeComponent::Scalar
                    && let Some(trace) = measurement_output_traces.get(&probe.to_ascii_uppercase())
                {
                    trace
                        .iter()
                        .filter(|(activation_axis, _)| result.frequency >= *activation_axis)
                        .map(|(_, value)| *value)
                        .next_back()
                        .unwrap_or(0.0)
                } else if column.component() == XyceAcProbeComponent::Scalar
                    && let Some(value) = Self::evaluate_static_frequency_device_parameter_probe(
                        "NOISE",
                        row_netlist,
                        &Self::normalize_probe(probe),
                    )
                {
                    value?
                } else if let Some(waveform) = signals.iter().find_map(|(name, values)| {
                    name.eq_ignore_ascii_case(&signal_probe).then_some(*values)
                }) {
                    *waveform.get(row_index).ok_or_else(|| {
                        format!("NOISE probe '{probe}' has no value for row {row_index}")
                    })?
                } else if column.component() == XyceAcProbeComponent::Scalar
                    && let Some(expression) = Self::print_expression_inner(probe)
                {
                    let mut context = row_netlist.params.clone();
                    context.set("FREQ", result.frequency);
                    context.set("FREQUENCY", result.frequency);
                    context.set("HERTZ", result.frequency);
                    for (name, values) in &signals {
                        if let Some(value) = values.get(row_index) {
                            context.set(name, *value);
                        }
                    }
                    let mut call_value = |call: &str| {
                        if call.get(..3).is_some_and(|prefix| {
                            prefix.eq_ignore_ascii_case("dno") || prefix.eq_ignore_ascii_case("dni")
                        }) {
                            let probe = rspice_core::analysis::NoiseContributionProbe::parse(call)
                                .map_err(|err| err.to_string())?;
                            return result.contribution(&probe).map_err(|err| err.to_string());
                        }
                        let normalized = Self::normalize_probe(call);
                        signals
                            .iter()
                            .find_map(|(name, values)| {
                                Self::normalize_probe(name)
                                    .eq_ignore_ascii_case(&normalized)
                                    .then_some(*values)
                            })
                            .and_then(|values| values.get(row_index).copied())
                            .ok_or_else(|| {
                                format!(
                                    "NOISE expression probe '{call}' is unavailable at row {row_index}"
                                )
                            })
                    };
                    Self::evaluate_print_expression_with_probe_calls(
                        expression,
                        context,
                        &mut call_value,
                    )
                    .map_err(|err| {
                        format!("NOISE print expression '{probe}' failed at row {row_index}: {err}")
                    })?
                } else {
                    return Err(format!("NOISE probe '{probe}' is not available"));
                };
                let normalized_probe = Self::normalize_probe(probe);
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: reference.columns[column_index + data_column_offset].clone(),
                        expected,
                        actual,
                        relative_error,
                    });
                }
                if mismatches.len() >= self.config.max_mismatches {
                    mismatches.truncate(self.config.max_mismatches);
                    return Ok(mismatches);
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_ac_data_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        source: &str,
        points: &[XyceAcDataPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }

        let mut data_column_offset = 0usize;
        let stepnum_column = reference
            .columns
            .first()
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"))
            .then_some(0usize);
        if stepnum_column.is_some() {
            data_column_offset += 1;
        }

        let index_column = reference
            .columns
            .get(data_column_offset)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"))
            .then_some(data_column_offset);
        if index_column.is_some() {
            data_column_offset += 1;
        }

        let frequency_column = reference
            .columns
            .get(data_column_offset)
            .filter(|column| Self::is_ac_frequency_reference_column(column))
            .map(|_| data_column_offset)
            .ok_or_else(|| {
                format!(
                    "expected Xyce .FD.prn frequency column at position {}, got '{}'",
                    data_column_offset,
                    reference
                        .columns
                        .get(data_column_offset)
                        .map(String::as_str)
                        .unwrap_or("<missing>")
                )
            })?;
        data_column_offset += 1;

        if reference.rows.len() != points.len() {
            return Err(format!(
                "reference row count ({}) does not match AC point count ({})",
                reference.rows.len(),
                points.len()
            ));
        }

        let data_columns = Self::reference_ac_data_columns(reference, print, data_column_offset)?;
        let phase_output_radians = Self::source_requests_ac_phase_output_radians(source);
        let comp_columns = data_columns
            .iter()
            .map(|column| XyceReferenceColumn::Probe {
                name: column.probe_name().to_string(),
            })
            .collect::<Vec<_>>();
        let comp_tolerances = self.comp_tolerances(source, &comp_columns)?;
        let frequency_tolerance = XyceComparisonTolerance::from_config(&self.config);

        let mut mismatches = Vec::new();
        for (row_index, (row, point)) in reference.rows.iter().zip(points).enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if let Some(stepnum_column) = stepnum_column {
                let expected_stepnum = row[stepnum_column];
                if (expected_stepnum - 0.0).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "STEPNUM".to_string(),
                        expected: expected_stepnum,
                        actual: 0.0,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
            if let Some(index_column) = index_column {
                let expected_index = row[index_column];
                let actual_index = row_index as Value;
                if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: actual_index,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }

            let expected_frequency = row[frequency_column];
            if let Some(relative_error) = self.value_mismatch(
                expected_frequency,
                point.result.frequency,
                frequency_tolerance,
            ) {
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: reference.columns[frequency_column].clone(),
                    expected: expected_frequency,
                    actual: point.result.frequency,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }

            for (column_index, column) in data_columns.iter().enumerate() {
                let expected = row[column_index + data_column_offset];
                let actual = Self::evaluate_ac_reference_column(
                    column,
                    &point.netlist,
                    &point.result,
                    phase_output_radians,
                )?;
                let normalized_probe = Self::normalize_probe(column.probe_name());
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: reference.columns[column_index + data_column_offset].clone(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    pub(super) fn compare_ac_prn_reference_with_step(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        results: &[AcResult],
        expected_step_index: Option<usize>,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }

        let mut data_column_offset = 0usize;
        let stepnum_column = reference
            .columns
            .first()
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"))
            .then_some(0usize);
        if stepnum_column.is_some() {
            data_column_offset += 1;
        }

        let index_column = reference
            .columns
            .get(data_column_offset)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"))
            .then_some(data_column_offset);
        if index_column.is_some() {
            data_column_offset += 1;
        }

        let frequency_column = reference
            .columns
            .get(data_column_offset)
            .filter(|column| Self::is_ac_frequency_reference_column(column))
            .map(|_| data_column_offset)
            .ok_or_else(|| {
                format!(
                    "expected Xyce .FD.prn frequency column at position {}, got '{}'",
                    data_column_offset,
                    reference
                        .columns
                        .get(data_column_offset)
                        .map(String::as_str)
                        .unwrap_or("<missing>")
                )
            })?;
        data_column_offset += 1;

        if reference.rows.len() != results.len() {
            return Err(format!(
                "reference row count ({}) does not match AC simulation point count ({})",
                reference.rows.len(),
                results.len()
            ));
        }

        let data_columns = Self::reference_ac_data_columns(reference, print, data_column_offset)?;
        let equation_traces =
            rspice_core::analysis::evaluate_ac_equation_measurements(netlist, results)?;
        let measurement_output_traces = Self::measurement_output_traces(
            netlist,
            &results
                .iter()
                .map(|result| result.frequency)
                .collect::<Vec<_>>(),
            data_columns.iter().filter_map(|column| {
                (column.component() == XyceAcProbeComponent::Scalar).then_some(column.probe_name())
            }),
            "AC",
            "AC_CONT",
            &[],
            |trace_netlist| {
                rspice_core::analysis::evaluate_ac_continuous_measurements(trace_netlist, results)
            },
        )?;
        let phase_output_radians = Self::source_requests_ac_phase_output_radians(source);
        let comp_columns = data_columns
            .iter()
            .map(|column| XyceReferenceColumn::Probe {
                name: column.probe_name().to_string(),
            })
            .collect::<Vec<_>>();
        let comp_tolerances = self.comp_tolerances(source, &comp_columns)?;
        let frequency_tolerance = XyceComparisonTolerance::from_config(&self.config);

        let mut mismatches = Vec::new();
        for (row_index, (row, result)) in reference.rows.iter().zip(results).enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if let Some(stepnum_column) = stepnum_column {
                let expected_stepnum = row[stepnum_column];
                let actual_stepnum = expected_step_index.unwrap_or(0) as f64;
                if (expected_stepnum - actual_stepnum).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "STEPNUM".to_string(),
                        expected: expected_stepnum,
                        actual: actual_stepnum,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
            if let Some(index_column) = index_column {
                let expected_index = row[index_column];
                let actual_index = row_index as f64;
                if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: actual_index,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }

            let expected_frequency = row[frequency_column];
            if let Some(relative_error) =
                self.value_mismatch(expected_frequency, result.frequency, frequency_tolerance)
            {
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: reference.columns[frequency_column].clone(),
                    expected: expected_frequency,
                    actual: result.frequency,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }

            for (column_index, column) in data_columns.iter().enumerate() {
                let expected = row[column_index + data_column_offset];
                let actual = if let Some(trace) = equation_traces
                    .iter()
                    .find(|trace| trace.name.eq_ignore_ascii_case(column.probe_name()))
                {
                    *trace.values.get(row_index).ok_or_else(|| {
                        format!(
                            "AC equation measure '{}' has no value for row {}",
                            trace.name, row_index
                        )
                    })?
                } else if column.component() == XyceAcProbeComponent::Scalar
                    && let Some(trace) =
                        measurement_output_traces.get(&column.probe_name().to_ascii_uppercase())
                {
                    trace
                        .iter()
                        .filter(|(activation_index, _)| row_index >= *activation_index)
                        .map(|(_, value)| *value)
                        .next_back()
                        .unwrap_or(0.0)
                } else {
                    Self::evaluate_ac_reference_column(
                        column,
                        netlist,
                        result,
                        phase_output_radians,
                    )?
                };
                let normalized_probe = Self::normalize_probe(column.probe_name());
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: reference.columns[column_index + data_column_offset].clone(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    pub(super) fn compare_static_tran_primary_reference(
        &self,
        reference: &XycePrnTable,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        match plan.comparison_mode {
            XyceStaticTranComparisonMode::Pointwise => self.compare_tran_prn_reference(
                reference,
                plan.require_print("pointwise transient primary comparison")?,
                netlist,
                &plan.source,
                result,
                plan.wrapper_tolerance,
            ),
            XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision,
            } => {
                let actual = Self::transient_family_result_to_prn_table(plan, netlist, result)?;
                self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
                    reference,
                    &actual,
                    XyceVerifyTransientTolerance::release_7_10_default(),
                    scientific_precision,
                )
            }
            XyceStaticTranComparisonMode::Release710IntegratedRmsComp {
                scientific_precision,
                ..
            } => {
                let actual = Self::transient_family_result_to_prn_table(plan, netlist, result)?;
                let tolerances =
                    Self::xyce_verify_comp_tolerances(&plan.source, &reference.columns[2..])?;
                self.compare_xyce_verify_transient_tables_with_probe_tolerances(
                    reference,
                    &actual,
                    &tolerances,
                    scientific_precision,
                )
            }
        }
    }

    #[cfg(test)]
    pub(super) fn compare_static_tran_reference_grid_diagnostic(
        &self,
        reference: &XycePrnTable,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        match plan.comparison_mode {
            XyceStaticTranComparisonMode::Pointwise => Err(
                "reference-grid integrated-RMS fallback requires an integrated verifier mode"
                    .to_string(),
            ),
            XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision,
            } => {
                let actual = Self::transient_family_result_to_prn_table_on_reference_grid(
                    plan,
                    netlist,
                    result,
                    reference,
                    scientific_precision,
                )?;
                self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
                    reference,
                    &actual,
                    XyceVerifyTransientTolerance::release_7_10_default(),
                    scientific_precision,
                )
            }
            XyceStaticTranComparisonMode::Release710IntegratedRmsComp {
                scientific_precision,
                ..
            } => {
                let actual = Self::transient_family_result_to_prn_table_on_reference_grid(
                    plan,
                    netlist,
                    result,
                    reference,
                    scientific_precision,
                )?;
                let tolerances =
                    Self::xyce_verify_comp_tolerances(&plan.source, &reference.columns[2..])?;
                self.compare_xyce_verify_transient_tables_with_probe_tolerances(
                    reference,
                    &actual,
                    &tolerances,
                    scientific_precision,
                )
            }
        }
    }

    pub(super) fn compare_tran_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        result: &TransientResult,
        wrapper_tolerance: Option<XyceComparisonTolerance>,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let layout = Self::transient_reference_layout(reference)?;
        let data_columns = Self::reference_tran_data_columns(
            reference,
            print,
            netlist,
            layout.data_column_offset,
        )?;
        let comp_columns = data_columns
            .iter()
            .map(|probe| XyceReferenceColumn::Probe {
                name: probe.clone(),
            })
            .collect::<Vec<_>>();
        let comp_tolerances = self.comp_tolerances(source, &comp_columns)?;
        let output_initial_interval = netlist
            .options
            .output_interval_schedule
            .as_ref()
            .map(|schedule| schedule.initial_interval);
        let tran_time_scale_factor = Self::tran_print_time_scale_factor(source)?;
        Self::validate_transient_result_time_grid(result)?;
        let reference_times = reference
            .rows
            .iter()
            .map(|row| {
                row.get(layout.time_column).copied().unwrap_or(f64::NAN) / tran_time_scale_factor
            })
            .collect::<Vec<_>>();
        let index_aligned_grid = result.time.len() == reference_times.len()
            && reference_times.windows(2).zip(result.time.windows(2)).all(
                |(reference_window, actual_window)| {
                    let reference_dt = reference_window[1] - reference_window[0];
                    let actual_dt = actual_window[1] - actual_window[0];
                    if !reference_dt.is_finite()
                        || !actual_dt.is_finite()
                        || reference_dt <= 0.0
                        || actual_dt <= 0.0
                    {
                        return false;
                    }
                    let ratio = actual_dt / reference_dt;
                    ratio.is_finite() && (0.75..=1.333_333_333_333_333_3).contains(&ratio)
                },
            );
        let stateful_waveforms = data_columns
            .iter()
            .map(|probe| Self::derived_tran_probe_waveform(probe, netlist, result))
            .collect::<Result<Vec<_>, _>>()?;
        let measurement_output_traces = Self::measurement_output_traces(
            netlist,
            &result.time,
            data_columns.iter().map(String::as_str),
            "TRAN",
            "TRAN_CONT",
            &[],
            |trace_netlist| {
                rspice_core::analysis::evaluate_tran_continuous_measurements(trace_netlist, result)
            },
        )?;

        let mut mismatches = Vec::new();
        for (row_index, row) in reference.rows.iter().enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if let Some(index_column) = layout.index_column {
                let expected_index = row[index_column];
                let actual_index = row_index as f64;
                if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: actual_index,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }

            let reference_time = row[layout.time_column];
            if !reference_time.is_finite() {
                return Err(format!(
                    "row {row_index} has non-finite TIME value {reference_time}"
                ));
            }
            let time = reference_time / tran_time_scale_factor;

            for (column_index, probe) in data_columns.iter().enumerate() {
                let expected = row[column_index + layout.data_column_offset];
                let actual = if let Some(trace) =
                    measurement_output_traces.get(&probe.to_ascii_uppercase())
                {
                    let time_tolerance = Self::default_prn_time_quantization_tolerance(time);
                    trace
                        .iter()
                        .filter(|(activation_index, _)| {
                            result
                                .time
                                .get(*activation_index)
                                .is_some_and(|activation_time| {
                                    *activation_time <= time + time_tolerance
                                })
                        })
                        .map(|(_, value)| *value)
                        .next_back()
                        .unwrap_or(0.0)
                } else {
                    match &stateful_waveforms[column_index] {
                        Some(values) => {
                            if index_aligned_grid {
                                values
                                    .get(row_index)
                                    .copied()
                                    .ok_or_else(|| {
                                        format!(
                                            "stateful transient waveform is missing aligned row {row_index}"
                                        )
                                    })?
                            } else {
                                Self::interpolate_transient_waveform_at(&result.time, values, time)?
                            }
                        }
                        None if index_aligned_grid => Self::evaluate_tran_probe(
                            probe,
                            netlist,
                            result,
                            result.time.get(row_index).copied().ok_or_else(|| {
                                format!("transient result is missing aligned row {row_index}")
                            })?,
                        )?,
                        None => Self::evaluate_tran_probe(probe, netlist, result, time)?,
                    }
                };
                let normalized_probe = Self::normalize_probe(probe);
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .or(wrapper_tolerance)
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    let time_tolerance = Self::default_prn_time_quantization_tolerance(time);
                    if stateful_waveforms[column_index].is_none()
                        && time_tolerance > 0.0
                        && self.transient_probe_matches_within_time_quantization(
                            probe,
                            netlist,
                            result,
                            time,
                            expected,
                            actual,
                            tolerance,
                            time_tolerance,
                        )?
                    {
                        continue;
                    }
                    if stateful_waveforms[column_index].is_none()
                        && self.transient_probe_matches_reference_time_neighborhood(
                            reference,
                            layout.time_column,
                            row_index,
                            column_index + layout.data_column_offset,
                            actual,
                            tolerance,
                            time_tolerance,
                            tran_time_scale_factor,
                        )
                    {
                        continue;
                    }
                    if stateful_waveforms[column_index].is_none()
                        && let Some(output_interval) = output_initial_interval
                        && self.transient_probe_matches_output_interval_corridor(
                            probe,
                            netlist,
                            result,
                            reference,
                            layout.time_column,
                            row_index,
                            expected,
                            tolerance,
                            output_interval,
                            tran_time_scale_factor,
                        )?
                    {
                        continue;
                    }
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: probe.clone(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    pub(super) fn default_prn_time_quantization_tolerance(time: Value) -> Value {
        if !time.is_finite() || time == 0.0 {
            return 0.0;
        }
        0.5 * 10.0_f64.powf(time.abs().log10().floor() - XYCE_DEFAULT_PRN_FRACTION_DIGITS)
    }

    pub(super) fn compare_step_res_reference(
        &self,
        path: &Path,
        netlist: &Netlist,
        steps: &[StepCommand],
        step_runs: &[XyceStepRun],
    ) -> Result<(), String> {
        const PARAMETER_SWEEP_FOOTER: &str = "End of Xyce(TM) Parameter Sweep";

        let content = fs::read_to_string(path)
            .map_err(|err| format!("{}: {err}", self.display_path(path)))?;
        let mut nonempty_lines = content
            .lines()
            .enumerate()
            .map(|(index, line)| (index + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty());

        let Some((header_line, header)) = nonempty_lines.next() else {
            return Err(format!("{} is empty", self.display_path(path)));
        };
        let header_fields = header.split_whitespace().collect::<Vec<_>>();
        if !header_fields
            .first()
            .is_some_and(|field| field.eq_ignore_ascii_case("STEP"))
        {
            return Err(format!(
                "{} line {header_line} must start with STEP",
                self.display_path(path)
            ));
        }
        let expected_columns = Self::step_res_expected_columns(netlist, steps, step_runs)?;
        if header_fields.len() != expected_columns.len() + 1 {
            return Err(format!(
                "{} line {header_line} has {} columns; expected STEP plus {} .STEP variable column(s)",
                self.display_path(path),
                header_fields.len(),
                expected_columns.len()
            ));
        }
        for (column_index, (expected_name, _)) in expected_columns.iter().enumerate() {
            let actual_name = header_fields[column_index + 1];
            if !actual_name.eq_ignore_ascii_case(expected_name) {
                return Err(format!(
                    "{} line {header_line} .STEP column {} is '{}', expected '{}'",
                    self.display_path(path),
                    column_index + 1,
                    actual_name,
                    expected_name
                ));
            }
        }

        let expected_row_count = expected_columns
            .first()
            .map(|(_, values)| values.len())
            .unwrap_or(0);
        let remaining_lines = nonempty_lines.collect::<Vec<_>>();
        if remaining_lines.len() < expected_row_count {
            return Err(format!(
                "{} has {} step row(s), expected {}",
                self.display_path(path),
                remaining_lines.len(),
                expected_row_count
            ));
        }
        let (rows, footer_and_trailing) = remaining_lines.split_at(expected_row_count);

        for (row_index, (line_number, line)) in rows.iter().copied().enumerate() {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() != expected_columns.len() + 1 {
                return Err(format!(
                    "{} line {line_number} has {} columns, expected STEP index plus {} value column(s)",
                    self.display_path(path),
                    fields.len(),
                    expected_columns.len()
                ));
            }
            let actual_index = fields[0].parse::<usize>().map_err(|err| {
                format!(
                    "{} line {line_number} has invalid STEP index '{}': {err}",
                    self.display_path(path),
                    fields[0]
                )
            })?;
            if actual_index != row_index {
                return Err(format!(
                    "{} line {line_number} has STEP index {actual_index}, expected {row_index}",
                    self.display_path(path)
                ));
            }
            for (column_index, (expected_name, expected_values)) in
                expected_columns.iter().enumerate()
            {
                let actual =
                    Self::parse_xyce_numeric_token(fields[column_index + 1]).map_err(|err| {
                        format!(
                            "{} line {line_number} has invalid STEP value '{}': {err}",
                            self.display_path(path),
                            fields[column_index + 1]
                        )
                    })?;
                let expected = expected_values[row_index];
                let tolerance = XyceComparisonTolerance::from_config(&self.config);
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    return Err(format!(
                        "{} line {line_number} STEP {} expected {:.8e}, actual {:.8e}, rel {:.3e}",
                        self.display_path(path),
                        expected_name,
                        expected,
                        actual,
                        relative_error
                    ));
                }
            }
        }

        let Some((footer_line_number, footer)) = footer_and_trailing.first().copied() else {
            return Err(format!(
                "{} is missing required footer '{PARAMETER_SWEEP_FOOTER}'",
                self.display_path(path)
            ));
        };
        if footer != PARAMETER_SWEEP_FOOTER {
            return Err(format!(
                "{} line {footer_line_number} must be exactly '{PARAMETER_SWEEP_FOOTER}', found '{footer}'",
                self.display_path(path)
            ));
        }
        if let Some((line_number, line)) = footer_and_trailing.get(1).copied() {
            if line == PARAMETER_SWEEP_FOOTER {
                return Err(format!(
                    "{} line {line_number} duplicates footer '{PARAMETER_SWEEP_FOOTER}'",
                    self.display_path(path)
                ));
            }
            return Err(format!(
                "{} line {line_number} has nonblank content after footer: '{line}'",
                self.display_path(path)
            ));
        }

        Ok(())
    }

    pub(super) fn comp_tolerances(
        &self,
        source: &str,
        columns: &[XyceReferenceColumn],
    ) -> Result<BTreeMap<String, XyceComparisonTolerance>, String> {
        let directives = Self::logical_comp_directives(source);
        if directives.is_empty() {
            return Ok(BTreeMap::new());
        }
        let mut compared_probes = BTreeSet::new();
        for column in columns {
            let XyceReferenceColumn::Probe { name } = column else {
                continue;
            };
            compared_probes.insert(Self::normalize_probe(name));
        }

        let mut tolerances = BTreeMap::new();
        for line in directives {
            let rest = Self::comp_directive_body(&line)
                .expect("logical COMP collector only returns COMP directives");
            let Some((probe, options)) = Self::split_comp_directive(rest) else {
                continue;
            };
            let normalized_probe = Self::normalize_probe(&probe);
            if !compared_probes.contains(&normalized_probe) {
                continue;
            }
            let default_tolerance = self.default_comparison_tolerance(&normalized_probe);
            let tolerance = Self::parse_comp_tolerance(&options, default_tolerance)?;
            tolerances.insert(normalized_probe, tolerance);
        }
        Ok(tolerances)
    }

    pub(super) fn xyce_verify_comp_tolerances(
        source: &str,
        columns: &[String],
    ) -> Result<Vec<XyceVerifyTransientTolerance>, String> {
        let normalized_columns = columns
            .iter()
            .map(|column| Self::normalize_probe(column))
            .collect::<Vec<_>>();
        let mut tolerances =
            vec![XyceVerifyTransientTolerance::release_7_10_default(); columns.len()];
        let mut assigned = vec![false; columns.len()];
        let mut saw_comp = false;
        for line in Self::logical_comp_directives(source) {
            let rest = Self::comp_directive_body(&line)
                .expect("logical COMP collector only returns COMP directives");
            saw_comp = true;
            let (probe, options) = Self::split_comp_directive(rest).ok_or_else(|| {
                "Xyce *COMP directive is missing its probe expression".to_string()
            })?;
            let tolerance = Self::parse_xyce_verify_comp_tolerance(&options)?;
            let normalized_probe = Self::normalize_probe(&probe);
            let Some(index) = normalized_columns
                .iter()
                .position(|column| *column == normalized_probe)
            else {
                // Release 7.10 stores unreferenced *COMP entries harmlessly;
                // only printed columns consult the resulting tolerance map.
                continue;
            };
            if assigned[index] {
                return Err(format!(
                    "Xyce *COMP defines duplicate tolerances for printed probe '{}'",
                    columns[index]
                ));
            }
            assigned[index] = true;
            tolerances[index] = tolerance;
        }
        if !saw_comp {
            return Err("Xyce integrated-RMS *COMP contract has no *COMP directive".to_string());
        }
        if !assigned.iter().any(|assigned| *assigned) {
            return Err(XYCE_VERIFY_COMP_NO_PRINTED_PROBE.to_string());
        }
        Ok(tolerances)
    }

    pub(super) fn parse_xyce_verify_comp_tolerance(
        options: &str,
    ) -> Result<XyceVerifyTransientTolerance, String> {
        let mut tolerance = XyceVerifyTransientTolerance::release_7_10_default();
        let tokens = options.split_whitespace().collect::<Vec<_>>();
        let mut seen = BTreeSet::new();
        let mut index = 0usize;
        while index < tokens.len() {
            let token = tokens[index];
            let (raw_key, raw_value, consumed) = if token.ends_with('=') {
                let key = token.trim_end_matches('=');
                let value = tokens.get(index + 1).copied().unwrap_or_default();
                (key, value, 2usize)
            } else if let Some((key, value)) = token.split_once('=') {
                (key, value, 1usize)
            } else if tokens
                .get(index + 1)
                .is_some_and(|candidate| *candidate == "=")
            {
                let value = tokens.get(index + 2).copied().unwrap_or_default();
                (token, value, 3usize)
            } else {
                return Err(format!(
                    "Xyce *COMP option '{token}' must use KEY=VALUE syntax"
                ));
            };
            let key = raw_key.trim().to_ascii_lowercase();
            let value = raw_value.trim();
            if key.is_empty() || value.is_empty() {
                return Err(format!("Xyce *COMP option '{key}' is missing a value"));
            }
            if !seen.insert(key.clone()) {
                return Err(format!("Xyce *COMP option '{key}' is duplicated"));
            }
            let parsed = Self::parse_comp_float(value)?;
            match key.as_str() {
                "reltol" => tolerance.relative = parsed,
                "abstol" => tolerance.absolute = parsed,
                "zerotol" => tolerance.zero = parsed,
                "absdifftol" => tolerance.absolute_difference = parsed,
                "offset" => tolerance.offset = parsed,
                "numfail" if parsed == 0.0 => {}
                "numfail" => {
                    return Err(format!(
                        "Xyce integrated-RMS *COMP option {key}={value} requires a broader comparison contract"
                    ));
                }
                _ => return Err(format!("unrecognized Xyce *COMP option '{key}'")),
            }
            index += consumed;
        }
        tolerance.validate()
    }

    pub(super) fn default_comparison_tolerance(
        &self,
        normalized_probe: &str,
    ) -> XyceComparisonTolerance {
        let mut tolerance = XyceComparisonTolerance::from_config(&self.config);
        if Self::probe_uses_voltage_tolerance(normalized_probe) {
            tolerance.absolute = self.config.voltage_absolute_tolerance;
        }
        if Self::probe_uses_current_tolerance(normalized_probe) {
            tolerance.zero = Some(
                tolerance
                    .zero
                    .unwrap_or(2.0 * self.config.absolute_tolerance),
            );
        }
        if Self::probe_uses_power_tolerance(normalized_probe) {
            tolerance.absolute = tolerance.absolute.max(self.config.power_absolute_tolerance);
        }
        tolerance
    }

    pub(super) fn probe_uses_power_tolerance(normalized_probe: &str) -> bool {
        if Self::parse_power_probe(normalized_probe).is_some() {
            return true;
        }
        Self::print_expression_inner(normalized_probe).is_some_and(|expression| {
            Self::parse_power_probe(&Self::normalize_probe(expression)).is_some()
        })
    }

    pub(super) fn probe_uses_voltage_tolerance(normalized_probe: &str) -> bool {
        normalized_probe == "v-sweep"
            || Self::parse_ac_voltage_probe(normalized_probe)
                .is_some_and(|probe| probe.accessor.uses_voltage_tolerance())
    }

    pub(super) fn probe_uses_current_tolerance(normalized_probe: &str) -> bool {
        Self::parse_current_probe(normalized_probe).is_some()
            || Self::parse_ac_current_probe(normalized_probe).is_some()
            || Self::parse_lead_current_probe(normalized_probe).is_some()
    }

    pub(super) fn parse_comp_tolerance(
        options: &str,
        default_tolerance: XyceComparisonTolerance,
    ) -> Result<XyceComparisonTolerance, String> {
        let mut tolerance = default_tolerance;
        let tokens = options.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index < tokens.len() {
            let token = tokens[index];
            let (raw_key, raw_value, consumed) = if let Some((key, value)) = token.split_once('=') {
                (key, value, 1usize)
            } else if token.ends_with('=') {
                let key = token.trim_end_matches('=');
                let value = tokens.get(index + 1).copied().unwrap_or_default();
                (key, value, 2usize)
            } else if tokens
                .get(index + 1)
                .is_some_and(|candidate| *candidate == "=")
            {
                let value = tokens.get(index + 2).copied().unwrap_or_default();
                (token, value, 3usize)
            } else {
                index += 1;
                continue;
            };

            let key = raw_key.trim().to_ascii_lowercase();
            let value = raw_value.trim();
            if value.is_empty() {
                return Err(format!("Xyce *COMP option '{key}' is missing a value"));
            }
            match key.as_str() {
                "reltol" => tolerance = tolerance.with_relative(Self::parse_comp_float(value)?),
                "abstol" | "absdifftol" => {
                    tolerance = tolerance.with_absolute(Self::parse_comp_float(value)?)
                }
                "zerotol" => tolerance = tolerance.with_zero(Self::parse_comp_float(value)?),
                _ => {}
            }
            index += consumed;
        }
        Ok(tolerance)
    }

    pub(super) fn value_mismatch(
        &self,
        expected: f64,
        actual: f64,
        tolerance: XyceComparisonTolerance,
    ) -> Option<f64> {
        if !expected.is_finite() || !actual.is_finite() {
            return Some(f64::INFINITY);
        }
        let abs_error = (expected - actual).abs();
        if let Some(zero_tolerance) = tolerance.zero
            && expected.abs() <= zero_tolerance
            && actual.abs() <= zero_tolerance
        {
            return None;
        }
        if abs_error <= tolerance.absolute {
            return None;
        }
        let scale = expected.abs().max(actual.abs()).max(tolerance.absolute);
        let relative_error = abs_error / scale;
        (relative_error > tolerance.relative).then_some(relative_error)
    }

    pub(super) fn resistor_branch_form_tolerance(netlist: &Netlist) -> Value {
        netlist
            .options
            .device_zero_resistance_tol
            .unwrap_or(XYCE_DEFAULT_ZERO_RESISTANCE_TOL)
            .max(0.0)
    }

    pub(super) fn compare_ac_comparator_tables(
        &self,
        gold: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_ac_comparator_tables_with_tolerance(
            gold,
            test,
            XyceAcComparatorTolerance::new(1.0e-6, 1.0e-3, 1.0e-10, 1.0e-6)
                .expect("built-in ACComparator tolerance is valid"),
        )
    }

    pub(super) fn compare_ac_comparator_tables_with_tolerance(
        &self,
        gold: &XycePrnTable,
        test: &XycePrnTable,
        tolerance: XyceAcComparatorTolerance,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let (absolute_tolerance, relative_tolerance, zero_tolerance, frequency_relative_tolerance) =
            tolerance.values();

        if gold.columns.len() < 2 || test.columns.len() < 2 {
            return Err(format!(
                "ACComparator tables require at least Index and FREQ columns, got gold={:?}, test={:?}",
                gold.columns, test.columns
            ));
        }
        if gold.columns != test.columns {
            return Err(format!(
                "ACComparator headers differ: gold {:?}, test {:?}",
                gold.columns, test.columns
            ));
        }
        if gold.rows.len() != test.rows.len() {
            return Err(format!(
                "ACComparator row counts differ: gold {}, test {}",
                gold.rows.len(),
                test.rows.len()
            ));
        }
        let mut mismatches = Vec::new();
        for (row_index, (gold_row, test_row)) in gold.rows.iter().zip(&test.rows).enumerate() {
            if gold_row.len() != gold.columns.len() || test_row.len() != test.columns.len() {
                return Err(format!(
                    "ACComparator row {row_index} width differs from its header"
                ));
            }
            if gold_row[0] != test_row[0] {
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: gold.columns[0].clone(),
                    expected: gold_row[0],
                    actual: test_row[0],
                    relative_error: 1.0,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }
            if gold_row[1] != test_row[1] {
                let difference = (test_row[1] - gold_row[1]).abs();
                let failed = if gold_row[1] == 0.0 {
                    test_row[1] > absolute_tolerance
                } else {
                    difference / gold_row[1] > frequency_relative_tolerance
                };
                if failed {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: gold.columns[1].clone(),
                        expected: gold_row[1],
                        actual: test_row[1],
                        relative_error: difference / gold_row[1].abs().max(absolute_tolerance),
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
            for column in 2..gold.columns.len() {
                let expected = gold_row[column];
                let actual = test_row[column];
                if expected == actual
                    || (expected.abs() < zero_tolerance && actual.abs() < zero_tolerance)
                {
                    continue;
                }
                let absolute_difference = (actual - expected).abs();
                let relative_difference = absolute_difference / expected.abs();
                if !(absolute_difference < absolute_tolerance
                    && relative_difference < relative_tolerance)
                {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: gold.columns[column].clone(),
                        expected,
                        actual,
                        relative_error: relative_difference,
                    });
                }
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }
            if mismatches.len() >= self.config.max_mismatches {
                return Ok(mismatches);
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_measurement_references(
        &self,
        paths: &[PathBuf],
        actual: &[rspice_core::analysis::MeasureResult],
        tolerance: XyceFileCompareTolerance,
        measure_fail_output: Option<bool>,
        measure_default_value: Option<Value>,
        analysis: &str,
        declarations: &[rspice_core::analysis::MeasureStatement],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if paths.len() != 1 {
            return Err(format!(
                "unstepped measurement comparison requires exactly one artifact, found {}",
                paths.len()
            ));
        }
        let expected = Self::parse_measurement_reference_file(&paths[0])?;
        if expected.len() != actual.len() {
            return Err(format!(
                "measurement artifact contains {} results but the netlist evaluated {} statements",
                expected.len(),
                actual.len()
            ));
        }
        let mut mismatches = Vec::new();
        for (row, (reference, result)) in expected.iter().zip(actual).enumerate() {
            if reference.name != result.name {
                return Err(format!(
                    "measurement {} is '{}' in the artifact but '{}' in declaration order",
                    row, reference.name, result.name
                ));
            }
            match reference.value {
                XyceMeasurementReferenceValue::Failed if result.value.is_none() => {}
                XyceMeasurementReferenceValue::Failed => {
                    return Err(format!(
                        "measurement '{}' was expected to be FAILED but evaluated to {}",
                        reference.name,
                        result.value.expect("matched nonempty measurement result")
                    ));
                }
                XyceMeasurementReferenceValue::Numeric {
                    value: expected_value,
                    quantization,
                } => {
                    let actual_value = match result.value {
                        Some(value) => value,
                        None if measure_fail_output == Some(false) => {
                            let local_default = declarations
                                .iter()
                                .find(|statement| {
                                    statement.name.eq_ignore_ascii_case(&reference.name)
                                        && statement.analysis.eq_ignore_ascii_case(analysis)
                                })
                                .and_then(|statement| statement.default_value);
                            measure_default_value.or(local_default).unwrap_or(0.0)
                        }
                        None => {
                            return Err(format!(
                                "measurement '{}' was expected to evaluate to {} but FAILED: {}",
                                reference.name,
                                expected_value,
                                result.error.as_deref().unwrap_or("no failure reason")
                            ));
                        }
                    };
                    let absolute_error = (expected_value - actual_value).abs();
                    let relative_error = if expected_value == 0.0 {
                        f64::INFINITY
                    } else {
                        absolute_error / expected_value.abs()
                    };
                    let matches = Self::measurement_value_matches(
                        expected_value,
                        actual_value,
                        quantization,
                        tolerance,
                    );
                    if !matches {
                        mismatches.push(XyceValueMismatch {
                            row,
                            probe: reference.name.clone(),
                            expected: expected_value,
                            actual: actual_value,
                            relative_error,
                        });
                    }
                }
            }
        }
        Ok(mismatches)
    }

    pub(super) fn compare_continuous_measurement_references(
        &self,
        paths: &[PathBuf],
        actual: &[rspice_core::analysis::ContinuousMeasureResult],
        tolerance: XyceFileCompareTolerance,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        for result in actual {
            result.validate_invariants().map_err(|error| {
                format!(
                    "continuous measurement '{}' violates its result invariant: {error}",
                    result.name
                )
            })?;
        }
        if paths.len() != actual.len() {
            return Err(format!(
                "continuous measurement oracle contains {} artifact(s) but the netlist evaluated {} statement(s)",
                paths.len(),
                actual.len()
            ));
        }
        let mut mismatches = Vec::new();
        let mut global_row = 0usize;
        for (path, result) in paths.iter().zip(actual) {
            let reference = Self::parse_continuous_measurement_reference_file(path)?;
            if !reference.name.eq_ignore_ascii_case(&result.name) {
                return Err(format!(
                    "continuous artifact '{}' contains '{}' but declaration order requires '{}'",
                    path.display(),
                    reference.name,
                    result.name
                ));
            }
            if reference.records.len() == 1
                && matches!(
                    reference.records[0].value,
                    XyceMeasurementReferenceValue::Failed
                )
            {
                if result.failure.is_none() {
                    return Err(format!(
                        "continuous measurement '{}' was expected to be FAILED but produced {} record(s)",
                        result.name,
                        result.records.len()
                    ));
                }
                global_row += 1;
                continue;
            }
            if let Some(failure) = result.failure.as_deref() {
                return Err(format!(
                    "continuous measurement '{}' was expected to produce {} record(s) but FAILED: {failure}",
                    result.name,
                    reference.records.len()
                ));
            }
            if reference.records.len() != result.records.len() {
                return Err(format!(
                    "continuous measurement '{}' artifact contains {} record(s) but evaluation produced {}",
                    result.name,
                    reference.records.len(),
                    result.records.len()
                ));
            }
            for (reference_record, actual_record) in reference.records.iter().zip(&result.records) {
                Self::compare_continuous_measurement_value(
                    &mut mismatches,
                    global_row,
                    &result.name,
                    reference_record.value,
                    Some(actual_record.value),
                    tolerance,
                )?;
                Self::compare_continuous_measurement_value(
                    &mut mismatches,
                    global_row,
                    &format!("{}:trig", result.name),
                    reference_record
                        .trigger_axis
                        .unwrap_or(XyceMeasurementReferenceValue::Failed),
                    actual_record.trigger_axis,
                    tolerance,
                )?;
                Self::compare_continuous_measurement_value(
                    &mut mismatches,
                    global_row,
                    &format!("{}:targ", result.name),
                    reference_record
                        .target_axis
                        .unwrap_or(XyceMeasurementReferenceValue::Failed),
                    actual_record.target_axis,
                    tolerance,
                )?;
                global_row += 1;
            }
        }
        Ok(mismatches)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn compare_analysis_measurement_outputs(
        &self,
        scalar_paths: &[PathBuf],
        continuous_paths: &[PathBuf],
        scalar: &[rspice_core::analysis::MeasureResult],
        continuous: &[rspice_core::analysis::ContinuousMeasureResult],
        tolerance: XyceFileCompareTolerance,
        measure_fail_output: Option<bool>,
        measure_default_value: Option<Value>,
        use_continuous_files: bool,
        declarations: &[rspice_core::analysis::MeasureStatement],
        base_analysis: &str,
        continuous_analysis: &str,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let has_continuous_declarations = declarations.iter().any(|declaration| {
            declaration
                .analysis
                .eq_ignore_ascii_case(continuous_analysis)
        });
        if continuous_paths.is_empty() && continuous.is_empty() && !has_continuous_declarations {
            // USE_CONT_FILES controls how continuous measures are serialized;
            // it must not turn an otherwise scalar-only aggregate into the
            // mixed-row model. The scalar comparator owns MEASFAIL and
            // DEFAULT_VAL projection for failed ordinary measurements.
            return self.compare_measurement_references(
                scalar_paths,
                scalar,
                tolerance,
                measure_fail_output,
                measure_default_value,
                base_analysis,
                declarations,
            );
        }
        if !use_continuous_files {
            return self.compare_mixed_measurement_references(
                scalar_paths,
                scalar,
                continuous,
                tolerance,
                declarations,
                base_analysis,
                continuous_analysis,
            );
        }

        let mut mismatches = if scalar_paths.is_empty() {
            Vec::new()
        } else {
            self.compare_measurement_references(
                scalar_paths,
                scalar,
                tolerance,
                measure_fail_output,
                measure_default_value,
                base_analysis,
                declarations,
            )?
        };
        let continuous_declarations = declarations
            .iter()
            .filter(|declaration| {
                declaration
                    .analysis
                    .eq_ignore_ascii_case(continuous_analysis)
            })
            .collect::<Vec<_>>();
        if continuous_declarations.len() != continuous.len() {
            return Err(format!(
                "{continuous_analysis} evaluator returned {} result(s) for {} declaration(s)",
                continuous.len(),
                continuous_declarations.len()
            ));
        }
        let mut visible_continuous = Vec::new();
        for (declaration, result) in continuous_declarations.into_iter().zip(continuous) {
            if !declaration.name.eq_ignore_ascii_case(&result.name) {
                return Err(format!(
                    "{continuous_analysis} evaluator returned '{}' for declaration '{}'",
                    result.name, declaration.name
                ));
            }
            if declaration.print_policy == rspice_core::analysis::MeasurePrintPolicy::All {
                visible_continuous.push(result.clone());
            }
        }
        mismatches.extend(self.compare_continuous_measurement_references(
            continuous_paths,
            &visible_continuous,
            tolerance,
        )?);
        Ok(mismatches)
    }

    pub(super) fn compare_mixed_measurement_references(
        &self,
        paths: &[PathBuf],
        scalar: &[rspice_core::analysis::MeasureResult],
        continuous: &[rspice_core::analysis::ContinuousMeasureResult],
        tolerance: XyceFileCompareTolerance,
        declarations: &[rspice_core::analysis::MeasureStatement],
        base_analysis: &str,
        continuous_analysis: &str,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        for result in continuous {
            result.validate_invariants().map_err(|error| {
                format!(
                    "continuous measurement '{}' violates its result invariant: {error}",
                    result.name
                )
            })?;
        }
        if paths.len() != 1 {
            return Err(format!(
                "one mixed measurement run requires exactly one aggregate artifact, found {}",
                paths.len()
            ));
        }
        let expected = Self::parse_mixed_measurement_reference_file(&paths[0])?;
        let actual = Self::mixed_measurement_rows(
            scalar,
            continuous,
            declarations,
            base_analysis,
            continuous_analysis,
        )?;
        if expected.len() != actual.len() {
            return Err(format!(
                "mixed measurement artifact contains {} row(s) but evaluation emitted {} row(s)",
                expected.len(),
                actual.len()
            ));
        }

        let mut mismatches = Vec::new();
        for (row, (reference, result)) in expected.iter().zip(&actual).enumerate() {
            if !reference.name.eq_ignore_ascii_case(&result.name) {
                return Err(format!(
                    "mixed measurement row {row} is '{}' in the artifact but '{}' in declaration-ordered evaluation",
                    reference.name, result.name
                ));
            }
            Self::compare_mixed_measurement_value(
                &mut mismatches,
                row,
                &reference.name,
                reference.value,
                result.value,
                tolerance,
            )?;
            Self::compare_mixed_measurement_metadata(
                &mut mismatches,
                row,
                &format!("{}:trig", reference.name),
                reference.trigger_axis,
                result.trigger_axis,
                tolerance,
            )?;
            Self::compare_mixed_measurement_metadata(
                &mut mismatches,
                row,
                &format!("{}:targ", reference.name),
                reference.target_axis,
                result.target_axis,
                tolerance,
            )?;
        }
        Ok(mismatches)
    }

    pub(super) fn compare_mixed_measurement_value(
        mismatches: &mut Vec<XyceValueMismatch>,
        row: usize,
        probe: &str,
        expected: XyceMeasurementReferenceValue,
        actual: XyceMeasurementReferenceValue,
        tolerance: XyceFileCompareTolerance,
    ) -> Result<(), String> {
        match (expected, actual) {
            (XyceMeasurementReferenceValue::Failed, XyceMeasurementReferenceValue::Failed) => {
                Ok(())
            }
            (
                XyceMeasurementReferenceValue::Failed,
                XyceMeasurementReferenceValue::Numeric { value, .. },
            ) => Err(format!(
                "mixed measurement '{probe}' row {row} was expected to be FAILED but evaluated to {value}"
            )),
            (
                XyceMeasurementReferenceValue::Numeric { value, .. },
                XyceMeasurementReferenceValue::Failed,
            ) => Err(format!(
                "mixed measurement '{probe}' row {row} was expected to evaluate to {value} but FAILED"
            )),
            (
                XyceMeasurementReferenceValue::Numeric {
                    value: expected,
                    quantization,
                },
                XyceMeasurementReferenceValue::Numeric { value: actual, .. },
            ) => {
                if !Self::measurement_value_matches(expected, actual, quantization, tolerance) {
                    let absolute_error = (expected - actual).abs();
                    mismatches.push(XyceValueMismatch {
                        row,
                        probe: probe.to_string(),
                        expected,
                        actual,
                        relative_error: if expected == 0.0 {
                            f64::INFINITY
                        } else {
                            absolute_error / expected.abs()
                        },
                    });
                }
                Ok(())
            }
        }
    }

    pub(super) fn compare_mixed_measurement_metadata(
        mismatches: &mut Vec<XyceValueMismatch>,
        row: usize,
        probe: &str,
        expected: Option<XyceMeasurementReferenceValue>,
        actual: Option<XyceMeasurementReferenceValue>,
        tolerance: XyceFileCompareTolerance,
    ) -> Result<(), String> {
        match (expected, actual) {
            (None, None) | (Some(XyceMeasurementReferenceValue::Failed), None) => Ok(()),
            (None, Some(XyceMeasurementReferenceValue::Numeric { value, .. })) => Err(format!(
                "mixed measurement artifact has no {probe} metadata at row {row}, but evaluation produced {value}"
            )),
            (
                Some(XyceMeasurementReferenceValue::Failed),
                Some(XyceMeasurementReferenceValue::Numeric { value, .. }),
            ) => Err(format!(
                "mixed measurement artifact marks {probe} not found at row {row}, but evaluation produced {value}"
            )),
            (
                Some(XyceMeasurementReferenceValue::Numeric { value, .. }),
                None | Some(XyceMeasurementReferenceValue::Failed),
            ) => Err(format!(
                "mixed measurement artifact expects {probe}={value} at row {row}, but evaluation omitted it"
            )),
            (Some(expected), Some(actual)) => Self::compare_mixed_measurement_value(
                mismatches, row, probe, expected, actual, tolerance,
            ),
            (None, Some(XyceMeasurementReferenceValue::Failed)) => Ok(()),
        }
    }

    pub(super) fn compare_continuous_measurement_value(
        mismatches: &mut Vec<XyceValueMismatch>,
        row: usize,
        probe: &str,
        expected: XyceMeasurementReferenceValue,
        actual: Option<Value>,
        tolerance: XyceFileCompareTolerance,
    ) -> Result<(), String> {
        match (expected, actual) {
            (XyceMeasurementReferenceValue::Failed, None) => Ok(()),
            (XyceMeasurementReferenceValue::Failed, Some(actual)) => Err(format!(
                "continuous artifact has no {probe} metadata at row {row}, but evaluation produced {actual}"
            )),
            (
                XyceMeasurementReferenceValue::Numeric {
                    value,
                    quantization,
                },
                Some(actual),
            ) => {
                if !Self::measurement_value_matches(value, actual, quantization, tolerance) {
                    let absolute_error = (value - actual).abs();
                    mismatches.push(XyceValueMismatch {
                        row,
                        probe: probe.to_string(),
                        expected: value,
                        actual,
                        relative_error: if value == 0.0 {
                            f64::INFINITY
                        } else {
                            absolute_error / value.abs()
                        },
                    });
                }
                Ok(())
            }
            (XyceMeasurementReferenceValue::Numeric { value, .. }, None) => Err(format!(
                "continuous artifact expects {probe}={value} at row {row}, but evaluation omitted it"
            )),
        }
    }
}

#[cfg(test)]
mod xyce_verify_abort_tests {
    use super::*;

    fn runner() -> XyceTestRunner {
        let config = XyceRunnerConfig {
            max_mismatches: 16,
            ..XyceRunnerConfig::default()
        };
        XyceTestRunner::new(env!("CARGO_MANIFEST_DIR"), config)
    }

    fn constant_table(value: Value) -> XycePrnTable {
        XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "TIME".to_string(),
                "V(out)".to_string(),
            ],
            rows: vec![vec![0.0, 0.0, value], vec![1.0, 1.0, value]],
        }
    }

    #[test]
    fn xyce_verify_integrated_rms_abort_overload_fails_immediately() {
        let table = constant_table(1.0);
        let error = runner()
            .compare_xyce_verify_transient_tables_with_abort(
                &table,
                &table,
                &rspice_core::abort_signal::ImmediateAbort,
            )
            .expect_err("an already-aborted comparison must fail closed");

        assert_eq!(error, "xyce_verify transient comparison aborted");
    }

    #[test]
    fn xyce_verify_integrated_rms_abort_overload_preserves_good_test_direction() {
        let runner = runner();
        let smaller = constant_table(1.0);
        let larger = constant_table(1.010_05);
        let no_abort = rspice_core::abort_signal::NoAbort;

        let legacy_forward = runner
            .compare_xyce_verify_transient_tables(&smaller, &larger)
            .expect("legacy forward comparison remains structurally valid");
        let abort_aware_forward = runner
            .compare_xyce_verify_transient_tables_with_abort(&smaller, &larger, &no_abort)
            .expect("abort-aware forward comparison remains structurally valid");
        assert_eq!(legacy_forward.len(), 1);
        assert_eq!(abort_aware_forward.len(), legacy_forward.len());
        assert_eq!(abort_aware_forward[0].probe, legacy_forward[0].probe);
        assert_eq!(
            abort_aware_forward[0].relative_error.to_bits(),
            legacy_forward[0].relative_error.to_bits()
        );

        assert!(
            runner
                .compare_xyce_verify_transient_tables(&larger, &smaller)
                .expect("legacy reverse comparison remains structurally valid")
                .is_empty(),
            "xyce_verify normalizes by the GOOD waveform, so reversing GOOD and TEST is intentionally asymmetric"
        );
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_abort(&larger, &smaller, &no_abort)
                .expect("abort-aware reverse comparison remains structurally valid")
                .is_empty(),
            "the abort overload must preserve GOOD/TEST direction semantics"
        );
    }
}

#[cfg(test)]
mod release_7_10_file_compare_tests {
    use super::*;

    fn runner() -> XyceTestRunner {
        let config = XyceRunnerConfig {
            max_mismatches: 16,
            ..XyceRunnerConfig::default()
        };
        XyceTestRunner::new(env!("CARGO_MANIFEST_DIR"), config)
    }

    fn table(columns: &[&str], rows: &[&[Value]]) -> XycePrnTable {
        XycePrnTable {
            columns: columns.iter().map(|column| (*column).to_string()).collect(),
            rows: rows.iter().map(|row| row.to_vec()).collect(),
        }
    }

    fn tolerance(absolute: Value, relative: Value, zero: Value) -> XyceFileCompareTolerance {
        XyceFileCompareTolerance {
            absolute,
            relative,
            zero,
        }
    }

    #[test]
    fn release_7_10_file_compare_requires_exact_layout() {
        let runner = runner();
        let gold = table(&["Index", "V(1)"], &[&[0.0, 200.0]]);
        let different_case = table(&["Index", "v(1)"], &[&[0.0, 200.0]]);
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &gold,
                    &different_case,
                    tolerance(1.0e-6, 1.0e-2, 1.0e-12),
                )
                .unwrap_err()
                .contains("headers differ")
        );

        let extra_row = table(&["Index", "V(1)"], &[&[0.0, 200.0], &[1.0, 201.0]]);
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &gold,
                    &extra_row,
                    tolerance(1.0e-6, 1.0e-2, 1.0e-12),
                )
                .unwrap_err()
                .contains("row counts differ")
        );

        let short_row = table(&["Index", "V(1)"], &[&[0.0]]);
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &gold,
                    &short_row,
                    tolerance(1.0e-6, 1.0e-2, 1.0e-12),
                )
                .unwrap_err()
                .contains("width differs")
        );
    }

    #[test]
    fn release_7_10_file_compare_rounds_through_default_prn_and_uses_strict_bounds() {
        let runner = runner();
        let serialized_equal_gold = table(&["V(1)"], &[&[200.000_000_001]]);
        let serialized_equal_test = table(&["V(1)"], &[&[200.0]]);
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &serialized_equal_gold,
                    &serialized_equal_test,
                    tolerance(0.0, 0.0, 0.0),
                )
                .unwrap()
                .is_empty()
        );

        let gold = table(&["V(1)"], &[&[200.0]]);
        let absolute_boundary = table(&["V(1)"], &[&[201.0]]);
        assert_eq!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &gold,
                    &absolute_boundary,
                    tolerance(1.0, 1.0, 0.0),
                )
                .unwrap()
                .len(),
            1,
            "absolute equality must fail the script's strict '<' check"
        );

        let relative_boundary = table(&["V(1)"], &[&[202.0]]);
        let expected = XyceTestRunner::xyce_default_prn_roundtrip(200.0).unwrap();
        let actual = XyceTestRunner::xyce_default_prn_roundtrip(202.0).unwrap();
        let exact_relative_boundary = (actual - expected).abs() / expected.abs();
        assert_eq!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &gold,
                    &relative_boundary,
                    tolerance(3.0, exact_relative_boundary, 0.0),
                )
                .unwrap()
                .len(),
            1,
            "relative equality must fail the script's strict '<' check"
        );

        let inside_both = table(&["V(1)"], &[&[200.5]]);
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &gold,
                    &inside_both,
                    tolerance(0.500_001, 0.003, 0.0),
                )
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn release_7_10_file_compare_relative_error_is_gold_asymmetric() {
        let runner = runner();
        let smaller = table(&["V(1)"], &[&[200.0]]);
        let larger = table(&["V(1)"], &[&[300.0]]);
        let tolerance = tolerance(101.0, 0.4, 0.0);

        assert_eq!(
            runner
                .compare_release_7_10_file_compare_tables(&smaller, &larger, tolerance)
                .unwrap()
                .len(),
            1
        );
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(&larger, &smaller, tolerance)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn release_7_10_file_compare_preserves_zero_and_phase_clauses() {
        let runner = runner();
        let zero_gold = table(&["V(1)"], &[&[1.0e-12]]);
        let zero_test = table(&["V(1)"], &[&[-1.0e-12]]);
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &zero_gold,
                    &zero_test,
                    tolerance(0.0, 0.0, 1.0e-12),
                )
                .unwrap()
                .is_empty()
        );

        // This pair is neither exact, near zero, nor within either numeric
        // tolerance. It passes only because the Perl phase clause compares
        // `abs(value) - 180` directly instead of taking its outer absolute.
        let phase_gold = table(&["P(V(1))"], &[&[100.0]]);
        let phase_test = table(&["P(V(1))"], &[&[-170.0]]);
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &phase_gold,
                    &phase_test,
                    tolerance(1.0e-6, 0.0, 0.0),
                )
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn release_7_10_file_compare_rejects_nonfinite_values_and_tolerances() {
        let runner = runner();
        let finite = table(&["V(1)"], &[&[200.0]]);
        let nonfinite = table(&["V(1)"], &[&[Value::INFINITY]]);
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &finite,
                    &nonfinite,
                    tolerance(1.0e-6, 1.0e-2, 1.0e-12),
                )
                .unwrap_err()
                .contains("cannot serialize")
        );

        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &finite,
                    &finite,
                    tolerance(-1.0, 1.0e-2, 1.0e-12),
                )
                .unwrap_err()
                .contains("finite and nonnegative")
        );
    }
}
