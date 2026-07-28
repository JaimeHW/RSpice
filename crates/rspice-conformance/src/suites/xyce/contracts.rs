//! Per-deck contract selection: which oracle applies to a deck.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn expected_unsupported_result(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        reason: &str,
    ) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: true,
            expected_unsupported: true,
            error: Some(format!("{EXPECTED_UNSUPPORTED_MARKER} {reason}")),
            mismatches: Vec::new(),
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    pub(super) fn validate_measure_cont_manifest_family(&self) -> Result<(), String> {
        let path = self.root.join(HARNESS_MANIFEST_FILE);
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            format!(
                "failed to inspect MEASURE_CONT harness manifest {}: {error}",
                path.display()
            )
        })?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "MEASURE_CONT harness manifest {} must be a regular non-symlink file",
                path.display()
            ));
        }
        let bytes = fs::read(&path).map_err(|error| {
            format!(
                "failed to read MEASURE_CONT harness manifest {}: {error}",
                path.display()
            )
        })?;
        let canonical = Self::canonical_lf_text_identity("MEASURE_CONT harness manifest", &bytes)?;
        let source = std::str::from_utf8(&canonical)
            .map_err(|error| format!("MEASURE_CONT harness manifest is not UTF-8: {error}"))?;
        let mut paths = Vec::new();
        let mut lines = Vec::new();
        for (line_index, line) in source.lines().enumerate() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((raw_path, raw_contract)) = line.split_once('\t') else {
                if line.to_ascii_lowercase().contains("measure_cont") {
                    return Err(format!(
                        "MEASURE_CONT harness manifest line {} has no tab-delimited contract",
                        line_index + 1
                    ));
                }
                continue;
            };
            let normalized_path = raw_path.trim().replace('\\', "/");
            if !normalized_path
                .to_ascii_lowercase()
                .starts_with("netlists/measure_cont/")
            {
                continue;
            }
            if raw_path != normalized_path.as_str()
                || raw_contract != REQUIRES_UPSTREAM_WRAPPER_CONTRACT
                || line != format!("{normalized_path}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}")
            {
                return Err(format!(
                    "MEASURE_CONT harness manifest line {} is not in canonical path-tab-contract form",
                    line_index + 1
                ));
            }
            paths.push(normalized_path);
            lines.push(line.to_string());
        }
        paths.sort();
        lines.sort();
        let path_hash = blake3::hash(paths.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let line_hash = blake3::hash(lines.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if paths.len() != XYCE_MEASURE_CONT_MANIFEST_FAMILY_COUNT
            || path_hash != XYCE_MEASURE_CONT_MANIFEST_FAMILY_PATHS_BLAKE3
            || lines.len() != XYCE_MEASURE_CONT_MANIFEST_FAMILY_COUNT
            || line_hash != XYCE_MEASURE_CONT_MANIFEST_FAMILY_LINES_BLAKE3
        {
            return Err(format!(
                "MEASURE_CONT harness manifest family changed: paths={}/{path_hash}, lines={}/{line_hash}",
                paths.len(),
                lines.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_regular_text_identity(
        path: &Path,
        expected: (usize, &str),
        label: &str,
    ) -> Result<Vec<u8>, String> {
        let metadata = fs::symlink_metadata(path)
            .map_err(|error| format!("failed to inspect {label} {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "{label} {} must be a regular non-symlink file",
                path.display()
            ));
        }
        let bytes = fs::read(path)
            .map_err(|error| format!("failed to read {label} {}: {error}", path.display()))?;
        let canonical = Self::canonical_lf_text_identity(label, &bytes)?;
        Self::validate_xdm_replaceground_identity(
            label,
            &path.display().to_string(),
            &canonical,
            expected,
        )?;
        Ok(canonical)
    }

    pub(super) fn validate_measure_cont_family_census(
        &self,
        relative: &str,
        expected_count: usize,
        expected_names_hash: &str,
        expected_content_hash: &str,
    ) -> Result<(), String> {
        fn visit(
            base: &Path,
            directory: &Path,
            names: &mut Vec<String>,
            content: &mut Vec<String>,
        ) -> Result<(), String> {
            let metadata = fs::symlink_metadata(directory).map_err(|error| {
                format!(
                    "failed to inspect MEASURE_CONT family {}: {error}",
                    directory.display()
                )
            })?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
                return Err(format!(
                    "MEASURE_CONT family {} must contain only regular directories",
                    directory.display()
                ));
            }
            for entry in fs::read_dir(directory)
                .map_err(|error| format!("failed to read {}: {error}", directory.display()))?
            {
                let entry = entry.map_err(|error| {
                    format!(
                        "failed to inspect member of {}: {error}",
                        directory.display()
                    )
                })?;
                let path = entry.path();
                let metadata = fs::symlink_metadata(&path)
                    .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
                if metadata.file_type().is_symlink() {
                    return Err(format!(
                        "MEASURE_CONT family member {} is a symlink",
                        path.display()
                    ));
                }
                if metadata.file_type().is_dir() {
                    visit(base, &path, names, content)?;
                    continue;
                }
                if !metadata.file_type().is_file() {
                    return Err(format!(
                        "MEASURE_CONT family member {} is not a regular file",
                        path.display()
                    ));
                }
                let name = path
                    .strip_prefix(base)
                    .map_err(|_| "MEASURE_CONT family member escaped its base".to_string())?
                    .to_string_lossy()
                    .replace('\\', "/")
                    .to_ascii_lowercase();
                let bytes = fs::read(&path)
                    .map_err(|error| format!("failed to hash {}: {error}", path.display()))?;
                let canonical = XyceTestRunner::canonical_lf_text_identity(
                    &format!("MEASURE_CONT family member {}", path.display()),
                    &bytes,
                )?;
                names.push(name.clone());
                content.push(format!("{name}\0{}", blake3::hash(&canonical).to_hex()));
            }
            Ok(())
        }

        let base = self.root.join(relative);
        let mut names = Vec::new();
        let mut content = Vec::new();
        visit(&base, &base, &mut names, &mut content)?;
        names.sort();
        content.sort();
        let names_hash = blake3::hash(names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if names.len() != expected_count
            || names_hash != expected_names_hash
            || content.len() != expected_count
            || content_hash != expected_content_hash
        {
            return Err(format!(
                "MEASURE_CONT family census changed for {relative}: names={}/{names_hash}, content={}/{content_hash}",
                names.len(),
                content.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_historical_identities() -> Result<(), String> {
        const TOOLS: [(usize, &str); 3] = [
            (
                44922,
                "a8f47987c43ac63e7954b8a89cfaddb7edc8fbff50d5bbab43a57f417dde7c0d",
            ),
            (
                7465,
                "a700143baddab265ca2e74d69541432fb27ae66600c3fee71968797fc78efcb0",
            ),
            (
                59566,
                "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
            ),
        ];
        let wrappers = XyceMeasureContTranKind::ALL
            .into_iter()
            .map(XyceMeasureContTranKind::historical_wrapper_identity)
            .collect::<BTreeSet<_>>();
        if wrappers.len() != 3
            || wrappers.iter().chain(TOOLS.iter()).any(|(bytes, sha256)| {
                *bytes == 0
                    || sha256.len() != 64
                    || !sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
        {
            return Err("MEASURE_CONT Release-7.10 wrapper/tool provenance is malformed".into());
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_pwl(
        netlist: &Netlist,
        name: &str,
        nodes: [&str; 2],
        expected: &[(Value, Value)],
    ) -> Result<(), String> {
        let source = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| format!("MEASURE_CONT is missing {name}"))?;
        let ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pwl {
            points,
            delay,
            repeat_from,
        }) = &source.kind
        else {
            return Err(format!("MEASURE_CONT {name} is not a PWL voltage source"));
        };
        if source.nodes != nodes
            || delay.to_bits() != 0.0f64.to_bits()
            || repeat_from.is_some()
            || points.len() != expected.len()
            || points.iter().zip(expected).any(|(actual, expected)| {
                actual.0.to_bits() != expected.0.to_bits()
                    || actual.1.to_bits() != expected.1.to_bits()
            })
        {
            return Err(format!("MEASURE_CONT {name} PWL topology changed"));
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_remeasure(
        netlist: &Netlist,
        result: &TransientResult,
        serialized_table: Option<&XycePrnTable>,
        tolerance: XyceFileCompareTolerance,
        scientific_precision: usize,
    ) -> Result<(), String> {
        // Release 7.10's wrapper serializes the native waveform to default
        // scientific PRN text, reads that data back, and compares the newly
        // produced mt0 stream to the original measure run. Reconstruct the
        // same data boundary (including printed precision and duplicate-time
        // suppression) before the independent second measurement pass.
        let serialized = Self::measure_cont_serialized_remeasure_result(
            result,
            serialized_table,
            scientific_precision,
        )?;
        let original_scalar =
            rspice_core::analysis::evaluate_tran_measurements(netlist, result);
        let original_continuous =
            rspice_core::analysis::evaluate_tran_continuous_measurements(netlist, result);
        let remeasured_scalar =
            rspice_core::analysis::evaluate_tran_measurements(netlist, &serialized);
        let remeasured_continuous =
            rspice_core::analysis::evaluate_tran_continuous_measurements(
                netlist,
                &serialized,
            );
        let original = Self::mixed_measurement_rows(
            &original_scalar,
            &original_continuous,
            &netlist.measurements,
            "TRAN",
            "TRAN_CONT",
        )?;
        let remeasured = Self::mixed_measurement_rows(
            &remeasured_scalar,
            &remeasured_continuous,
            &netlist.measurements,
            "TRAN",
            "TRAN_CONT",
        )?;
        if original.len() != remeasured.len() {
            return Err(format!(
                "MEASURE_CONT serialized remeasure emitted {} row(s), original run emitted {}",
                remeasured.len(),
                original.len()
            ));
        }
        let mut mismatches = Vec::new();
        for (row, (expected, actual)) in original.iter().zip(&remeasured).enumerate() {
            if !expected.name.eq_ignore_ascii_case(&actual.name) {
                return Err(format!(
                    "MEASURE_CONT serialized remeasure row {row} is '{}' but original row is '{}'",
                    actual.name, expected.name
                ));
            }
            Self::compare_mixed_measurement_value(
                &mut mismatches,
                row,
                &expected.name,
                expected.value,
                actual.value,
                tolerance,
            )?;
            Self::compare_mixed_measurement_metadata(
                &mut mismatches,
                row,
                &format!("{}:trig", expected.name),
                expected.trigger_axis,
                actual.trigger_axis,
                tolerance,
            )?;
            Self::compare_mixed_measurement_metadata(
                &mut mismatches,
                row,
                &format!("{}:targ", expected.name),
                expected.target_axis,
                actual.target_axis,
                tolerance,
            )?;
        }
        if !mismatches.is_empty() {
            return Err(format!(
                "MEASURE_CONT serialized remeasure comparison produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_counterfactual(
        &self,
        kind: XyceMeasureContTranKind,
        netlist: &Netlist,
        scalar: &[rspice_core::analysis::MeasureResult],
        continuous: &[rspice_core::analysis::ContinuousMeasureResult],
        mt0_path: &Path,
    ) -> Result<(), String> {
        let compare = |label: &str,
                       counterfactual: &[rspice_core::analysis::ContinuousMeasureResult]|
         -> Result<(), String> {
            let mismatches = self.compare_analysis_measurement_outputs(
                &[mt0_path.to_path_buf()],
                &[],
                scalar,
                counterfactual,
                XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
                netlist.options.measure_fail_output,
                netlist.options.measure_default_value,
                false,
                &netlist.measurements,
                "TRAN",
                "TRAN_CONT",
            )?;
            if mismatches.is_empty() {
                Err(format!(
                    "MEASURE_CONT {label} counterfactual unexpectedly reproduced the aggregate oracle"
                ))
            } else {
                Ok(())
            }
        };

        let mut numeric = continuous.to_vec();
        let record = numeric
            .iter_mut()
            .find_map(|result| result.records.first_mut())
            .ok_or_else(|| {
                "MEASURE_CONT produced no successful event to test causality".to_string()
            })?;
        record.value += record.value.abs().max(1.0);
        compare("numeric-value", &numeric)?;

        let mut reordered = continuous.to_vec();
        let pair = reordered.iter_mut().find_map(|result| {
            result
                .records
                .windows(2)
                .position(|window| window[0] != window[1])
                .map(|index| (&mut result.records, index))
        });
        let Some((records, index)) = pair else {
            return Err(
                "MEASURE_CONT produced no distinct adjacent events to test ordering".into(),
            );
        };
        records.swap(index, index + 1);
        compare("event-order", &reordered)?;

        if kind == XyceMeasureContTranKind::TriggerTarget {
            let mut metadata = continuous.to_vec();
            let partial = metadata.iter_mut().find(|result| {
                result.failure_metadata.is_some_and(|metadata| {
                    metadata.trigger_axis.is_some() || metadata.target_axis.is_some()
                })
            });
            let Some(partial) = partial else {
                return Err(
                    "MEASURE_CONT TRIG/TARG produced no partial failed endpoint metadata".into(),
                );
            };
            partial.failure_metadata = None;
            let comparison = self.compare_analysis_measurement_outputs(
                &[mt0_path.to_path_buf()],
                &[],
                scalar,
                &metadata,
                XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
                netlist.options.measure_fail_output,
                netlist.options.measure_default_value,
                false,
                &netlist.measurements,
                "TRAN",
                "TRAN_CONT",
            );
            match comparison {
                Err(error) if error.contains("expects") && error.contains("omitted") => {}
                Ok(mismatches) if !mismatches.is_empty() => {}
                Err(error) => {
                    return Err(format!(
                        "MEASURE_CONT partial-metadata counterfactual failed outside the expected comparator: {error}"
                    ));
                }
                Ok(_) => {
                    return Err(
                        "MEASURE_CONT partial-metadata counterfactual reproduced the aggregate oracle"
                            .into(),
                    );
                }
            }
        } else {
            let mut detected = false;
            'candidate: for (result_index, result) in continuous.iter().enumerate() {
                for (record_index, record) in result.records.iter().enumerate() {
                    let Some(axis) = record.event_axis else {
                        continue;
                    };
                    let mut event_axis = continuous.to_vec();
                    event_axis[result_index].records[record_index].event_axis =
                        Some(axis + axis.abs().max(1.0));
                    match self.validate_measure_cont_gs_semantics(kind, netlist, &event_axis) {
                        Err(error) if error.contains("GS event metadata produced") => {
                            detected = true;
                            break 'candidate;
                        }
                        Err(error) => {
                            return Err(format!(
                                "MEASURE_CONT event-axis counterfactual failed outside the expected comparator: {error}"
                            ));
                        }
                        Ok(()) => {}
                    }
                }
            }
            if !detected {
                return Err(
                    "MEASURE_CONT event-axis counterfactual reproduced every GS row".into(),
                );
            }
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_prn_counterfactual(
        &self,
        reference: &XycePrnTable,
        actual: &XycePrnTable,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let measurement_names = netlist
            .measurements
            .iter()
            .map(|statement| statement.name.to_ascii_uppercase())
            .collect::<BTreeSet<_>>();
        let column = plan
            .print
            .probes
            .iter()
            .position(|probe| measurement_names.contains(&probe.to_ascii_uppercase()))
            .map(|index| index + 2)
            .ok_or_else(|| "MEASURE_CONT PRN contains no live measurement column".to_string())?;
        let mut counterfactual = actual.clone();
        let row = counterfactual
            .rows
            .iter_mut()
            .find(|row| row.get(column).is_some_and(|value| *value != 0.0))
            .ok_or_else(|| {
                "MEASURE_CONT PRN contains no initialized live measurement value".to_string()
            })?;
        row[column] = 0.0;
        let mismatches = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            reference,
            &counterfactual,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if mismatches.is_empty() {
            Err(
                "MEASURE_CONT live-trace activation counterfactual reproduced the PRN oracle"
                    .into(),
            )
        } else {
            Ok(())
        }
    }

    pub(super) fn validate_measure_cont_gs_semantics(
        &self,
        kind: XyceMeasureContTranKind,
        netlist: &Netlist,
        continuous: &[rspice_core::analysis::ContinuousMeasureResult],
    ) -> Result<(), String> {
        let gs_path = self.root.join(kind.gs_relative_path());
        let mt0_path = self.root.join(kind.mt0_relative_path());
        let gs = Self::parse_measure_cont_gs_file(&gs_path)?;
        let mt0 = Self::parse_mixed_measurement_reference_file(&mt0_path)?;
        if gs.len() != mt0.len() {
            return Err(format!(
                "MEASURE_CONT GS contains {} result row(s), but mt0 contains {}",
                gs.len(),
                mt0.len()
            ));
        }
        let mut mismatches = Vec::new();
        for (row, (gs_row, mt0_row)) in gs.iter().zip(&mt0).enumerate() {
            if !gs_row.mixed.name.eq_ignore_ascii_case(&mt0_row.name) {
                return Err(format!(
                    "MEASURE_CONT GS row {row} is '{}' but mt0 row is '{}'",
                    gs_row.mixed.name, mt0_row.name
                ));
            }
            Self::compare_mixed_measurement_value(
                &mut mismatches,
                row,
                &gs_row.mixed.name,
                mt0_row.value,
                gs_row.mixed.value,
                XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
            )?;
            Self::compare_mixed_measurement_metadata(
                &mut mismatches,
                row,
                &format!("{}:trig", gs_row.mixed.name),
                mt0_row.trigger_axis,
                gs_row.mixed.trigger_axis,
                XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
            )?;
            Self::compare_mixed_measurement_metadata(
                &mut mismatches,
                row,
                &format!("{}:targ", gs_row.mixed.name),
                mt0_row.target_axis,
                gs_row.mixed.target_axis,
                XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
            )?;
        }
        if !mismatches.is_empty() {
            return Err(format!(
                "MEASURE_CONT GS/mt0 semantic relation produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ));
        }

        let mut gs_index = 0usize;
        let mut continuous_index = 0usize;
        for declaration in &netlist.measurements {
            if declaration.analysis.eq_ignore_ascii_case("TRAN") {
                gs_index += 1;
                continue;
            }
            let actual = continuous.get(continuous_index).ok_or_else(|| {
                format!(
                    "TRAN_CONT evaluator omitted declaration '{}'",
                    declaration.name
                )
            })?;
            continuous_index += 1;
            let row_count = if actual.failure.is_some() {
                1
            } else {
                actual.records.len()
            };
            for record_index in 0..row_count {
                let expected = gs
                    .get(gs_index)
                    .ok_or_else(|| format!("GS ended before declaration '{}'", declaration.name))?;
                if !expected.mixed.name.eq_ignore_ascii_case(&declaration.name) {
                    return Err(format!(
                        "GS row {gs_index} is '{}' but declaration order requires '{}'",
                        expected.mixed.name, declaration.name
                    ));
                }
                if let Some(record) = actual.records.get(record_index) {
                    if let Some(expected_event_axis) = expected.event_axis {
                        Self::compare_continuous_measurement_value(
                            &mut mismatches,
                            gs_index,
                            &format!("{}:event", declaration.name),
                            expected_event_axis,
                            record.event_axis,
                            XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
                        )?;
                    }
                }
                gs_index += 1;
            }
        }
        if continuous_index != continuous.len() || gs_index != gs.len() {
            return Err(format!(
                "MEASURE_CONT GS declaration projection left rows/results unclaimed: gs={gs_index}/{}, continuous={continuous_index}/{}",
                gs.len(),
                continuous.len()
            ));
        }
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "MEASURE_CONT GS event metadata produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ))
        }
    }

    pub(super) fn validate_abm_pow_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceAbmPowKind,
        start: Instant,
    ) -> Result<(), String> {
        let source_bytes = self.validate_abm_pow_provenance(deck, kind)?;
        self.check_abm_pow_deadline(start, "provenance")?;
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("ABM_POW source is not UTF-8: {error}"))?;
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            &deck.path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let parsed = Self::parse_xyce_netlist(source, &deck.path)
            .map_err(|error| format!("ABM_POW parse failed: {error}"))?;
        Self::validate_abm_pow_plan(&plan, &parsed, kind)?;
        Self::validate_abm_pow_topology(&parsed, kind)?;
        self.check_abm_pow_deadline(start, "parse and exact topology")?;

        let (netlist, results) =
            self.run_static_dc_results(&plan, start)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!(
                        "ABM_POW native DC execution exceeded shared timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    other => format!("ABM_POW native DC execution failed: {other}"),
                })?;
        Self::validate_abm_pow_topology(&netlist, kind)?;
        let actual = self.dc_results_to_prn_table(&plan, &netlist, &results)?;
        Self::validate_abm_pow_output_domain(&actual, &results, kind)?;
        let gold = Self::abm_pow_dynamic_gold_table(&actual, kind)?;
        let mismatches = self.compare_release_7_10_xyce_verify_dc_tables(
            "ABM_POW", &gold, &actual, &results, &results,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "ABM_POW Release 7.10 generated-gold comparison produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ));
        }

        // Each Perl sidecar derives its gold columns from the DUT's serialized
        // V(1), so defend against common-mode execution errors with a semantic
        // counterfactual that implements the exact bug the deck is meant to
        // catch.  A passing counterfactual would prove that the selected power
        // grammar/complex projection is not causally observed by this oracle.
        let counterfactual = Self::abm_pow_counterfactual_table(&actual, kind)?;
        let counterfactual_mismatches = self.compare_release_7_10_xyce_verify_dc_tables(
            "ABM_POW counterfactual",
            &counterfactual,
            &actual,
            &results,
            &results,
        )?;
        if counterfactual_mismatches.is_empty() {
            return Err("ABM_POW counterfactual unexpectedly reproduced the native output".into());
        }
        self.check_abm_pow_deadline(start, "native execution, generated gold, and causality")
    }

    pub(super) fn validate_abm_pow_historical_identities() -> Result<(), String> {
        let mut wrappers = BTreeSet::new();
        let mut perl = BTreeSet::new();
        for kind in XyceAbmPowKind::ALL {
            let (wrapper_bytes, wrapper_sha256) = kind.historical_wrapper_identity();
            let (perl_bytes, perl_sha256) = kind.historical_perl_identity();
            if wrapper_bytes == 0
                || perl_bytes == 0
                || wrapper_sha256.len() != 64
                || perl_sha256.len() != 64
                || !wrapper_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
                || !perl_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
            {
                return Err("ABM_POW historical wrapper/Perl identity is malformed".into());
            }
            wrappers.insert((kind.record(), wrapper_bytes, wrapper_sha256));
            perl.insert((kind.record(), perl_bytes, perl_sha256));
        }
        if wrappers.len() != 3 || perl.len() != 3 {
            return Err("ABM_POW historical wrapper/Perl provenance is incomplete".into());
        }
        Ok(())
    }

    pub(super) fn validate_abm_pow_plan(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        kind: XyceAbmPowKind,
    ) -> Result<(), String> {
        let expected_step: Value = if kind == XyceAbmPowKind::UnaryMinusPrecedence {
            0.1
        } else {
            0.2
        };
        let expected_probes = kind
            .expected_columns()
            .iter()
            .skip(1)
            .map(|probe| probe.to_ascii_lowercase())
            .collect::<Vec<_>>();
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if !plan.dc.source.eq_ignore_ascii_case("VS")
            || plan.dc.start.to_bits() != (-2.5f64).to_bits()
            || plan.dc.stop.to_bits() != 2.5f64.to_bits()
            || plan.dc.step.to_bits() != expected_step.to_bits()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || plan.dc.sweep2.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || probes != expected_probes
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
        {
            return Err(format!(
                "ABM_POW exact DC/PRINT contract changed: sweep={:?}, probes={probes:?}, analyses={}, outputs={}",
                plan.dc,
                netlist.analyses.len(),
                netlist.output_requests.len()
            ));
        }
        if netlist.options.replace_ground.is_some()
            || netlist.options.remove_unused.is_some()
            || netlist.options.add_resistors.is_some()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
        {
            return Err("ABM_POW deck acquired unrelated model, hierarchy, parameter, data, or preprocessing state".into());
        }
        Ok(())
    }

    pub(super) fn validate_abm_pow_topology(
        netlist: &Netlist,
        kind: XyceAbmPowKind,
    ) -> Result<(), String> {
        let expected_behavioral: &[(&str, &str, &str)] = match kind {
            XyceAbmPowKind::UnaryMinusPrecedence => &[("b4", "4", "-v(1)**2")],
            XyceAbmPowKind::NegativeIntegerExponent => {
                &[("b5", "5", "(v(1))**-2"), ("b6", "6", "(v(1))**-3")]
            }
            XyceAbmPowKind::FractionalPrincipalComplex => {
                &[("b5", "5", "(v(1))**2.1"), ("b6", "6", "(-v(1))**3.1")]
            }
        };
        let expected_count = 2 + 2 * expected_behavioral.len();
        if netlist.elements.len() != expected_count {
            return Err(format!(
                "ABM_POW topology has {} elements instead of {expected_count}",
                netlist.elements.len()
            ));
        }
        let source = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("VS"))
            .ok_or_else(|| "ABM_POW has no VS source".to_string())?;
        if source.nodes != ["1", "0"]
            || !matches!(&source.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) if value.to_bits() == (-2.5f64).to_bits())
        {
            return Err("ABM_POW swept VS source topology/value changed".into());
        }
        for (name, nodes) in std::iter::once(("r1", ["1", "0"])).chain(
            expected_behavioral.iter().map(|(name, node, _)| {
                let resistor = if *name == "b4" {
                    "r4"
                } else if *name == "b5" {
                    "r5"
                } else {
                    "r6"
                };
                (resistor, [*node, "0"])
            }),
        ) {
            let resistor = netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("ABM_POW is missing resistor {name}"))?;
            if resistor.nodes != nodes
                || !matches!(&resistor.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params } if value.to_bits() == 1.0f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            {
                return Err(format!(
                    "ABM_POW resistor {name} changed exact unit-load topology"
                ));
            }
        }
        for (name, output, expected_expression) in expected_behavioral {
            let behavioral = netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("ABM_POW is missing behavioral source {name}"))?;
            let ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
            } = &behavioral.kind
            else {
                return Err(format!("ABM_POW {name} is not a behavioral voltage source"));
            };
            if behavioral.nodes != [*output, "0"]
                || tc1.to_bits() != 0.0f64.to_bits()
                || tc2.to_bits() != 0.0f64.to_bits()
                || Self::normalize_probe(expression) != *expected_expression
            {
                return Err(format!(
                    "ABM_POW {name} topology/expression changed: nodes={:?}, expression={expression:?}",
                    behavioral.nodes
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_abm_pow_output_domain(
        table: &XycePrnTable,
        results: &[DcSweepPointResult],
        kind: XyceAbmPowKind,
    ) -> Result<(), String> {
        if table.columns != kind.expected_columns()
            || table.rows.len() != kind.expected_rows()
            || results.len() != kind.expected_rows()
        {
            return Err(format!(
                "ABM_POW output layout changed: columns={:?}, rows={}, sweep_points={}",
                table.columns,
                table.rows.len(),
                results.len()
            ));
        }
        for (index, (row, point)) in table.rows.iter().zip(results).enumerate() {
            if row.len() != table.columns.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
            {
                return Err(format!(
                    "ABM_POW row {index} is malformed or nonfinite: {row:?}"
                ));
            }
            let printed_sweep = Self::xyce_default_prn_roundtrip(point.sweep_value)?;
            let printed_input = Self::xyce_default_prn_roundtrip(row[1])?;
            if printed_sweep.to_bits() != printed_input.to_bits() {
                return Err(format!(
                    "ABM_POW V(1) lost swept-source causality at row {index}: sweep={printed_sweep}, V(1)={printed_input}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_xdm_replaceground_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceXdmReplaceGroundKind,
        start: Instant,
    ) -> Result<(), String> {
        let hspice_path = self.validate_xdm_replaceground_provenance(deck, kind)?;
        self.check_xdm_replaceground_deadline(start, "provenance")?;
        let canonical_bytes = fs::read(&deck.path).map_err(|error| {
            format!(
                "failed to read canonical XDM REPLACEGROUND deck {}: {error}",
                deck.path.display()
            )
        })?;
        let hspice_bytes = fs::read(&hspice_path).map_err(|error| {
            format!(
                "failed to read paired HSPICE source {}: {error}",
                hspice_path.display()
            )
        })?;
        Self::validate_xdm_replaceground_identity(
            "canonical Xyce deck",
            kind.record(),
            &canonical_bytes,
            kind.source_identity(),
        )?;
        Self::validate_xdm_replaceground_identity(
            "paired HSPICE source",
            kind.record(),
            &hspice_bytes,
            kind.hspice_identity(),
        )?;
        let canonical_source = std::str::from_utf8(&canonical_bytes).map_err(|error| {
            format!(
                "canonical XDM REPLACEGROUND deck '{}' is not UTF-8: {error}",
                kind.record()
            )
        })?;
        let hspice_source = std::str::from_utf8(&hspice_bytes).map_err(|error| {
            format!(
                "paired HSPICE source for '{}' is not UTF-8: {error}",
                kind.record()
            )
        })?;

        Self::validate_xdm_replaceground_directives(canonical_source, 1, true)?;
        Self::validate_xdm_replaceground_directives(hspice_source, 0, false)?;
        let projected_source = Self::project_xdm_replaceground_hspice(hspice_source)?;
        Self::validate_xdm_replaceground_directives(&projected_source, 1, true)?;
        self.check_xdm_replaceground_deadline(start, "source projection")?;

        let canonical_plan = self.static_dc_plan_for_source_with_execution_dir(
            &deck.path,
            canonical_source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let projected_plan = self.static_dc_plan_for_source_with_execution_dir(
            &hspice_path,
            projected_source.clone(),
            ExpressionDialect::Xyce,
            None,
        )?;
        Self::validate_xdm_replaceground_plan(&canonical_plan, kind, "canonical")?;
        Self::validate_xdm_replaceground_plan(&projected_plan, kind, "projected HSPICE")?;
        self.check_xdm_replaceground_deadline(start, "plan validation")?;
        if !Self::dc_sweeps_match_exactly(&canonical_plan.dc, &projected_plan.dc) {
            return Err("canonical and projected HSPICE DC sweeps differ".to_string());
        }
        let canonical_probes = canonical_plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        let projected_probes = projected_plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if canonical_probes != projected_probes {
            return Err(format!(
                "canonical and projected HSPICE probe sets differ: {canonical_probes:?} versus {projected_probes:?}"
            ));
        }

        let canonical_netlist = Self::parse_netlist_with_expression_dialect(
            canonical_source,
            &deck.path,
            ExpressionDialect::Xyce,
        )
        .map_err(|error| format!("canonical XDM REPLACEGROUND parse failed: {error}"))?;
        let projected_netlist = Self::parse_netlist_with_expression_dialect(
            &projected_source,
            &hspice_path,
            ExpressionDialect::Xyce,
        )
        .map_err(|error| format!("projected HSPICE parse failed: {error}"))?;
        Self::validate_xdm_replaceground_effective_options(
            &canonical_netlist.options,
            true,
            "canonical",
        )?;
        Self::validate_xdm_replaceground_effective_options(
            &projected_netlist.options,
            true,
            "projected HSPICE",
        )?;
        Self::validate_xdm_replaceground_subcircuit_topology(&canonical_netlist, kind)?;
        Self::validate_xdm_replaceground_subcircuit_topology(&projected_netlist, kind)?;
        let canonical_structure =
            Self::xdm_replaceground_element_snapshot(&canonical_netlist, kind)?;
        let projected_structure =
            Self::xdm_replaceground_element_snapshot(&projected_netlist, kind)?;
        let expected_structure = kind.expected_flattened_snapshot();
        if canonical_structure != expected_structure {
            return Err(format!(
                "canonical XDM REPLACEGROUND flattened snapshot differs from its exact expected topology: expected={expected_structure:?}, actual={canonical_structure:?}"
            ));
        }
        if projected_structure != expected_structure {
            return Err(format!(
                "projected HSPICE flattened snapshot differs from its exact expected topology: expected={expected_structure:?}, actual={projected_structure:?}"
            ));
        }
        self.check_xdm_replaceground_deadline(start, "hierarchy and flattened topology")?;

        let (canonical_run_netlist, canonical_results) = self
            .run_static_dc_results(&canonical_plan, start)
            .map_err(|error| format!("canonical REPLACEGROUND DC execution failed: {error}"))?;
        let (projected_run_netlist, projected_results) = self
            .run_static_dc_results(&projected_plan, start)
            .map_err(|error| {
                format!("projected HSPICE REPLACEGROUND DC execution failed: {error}")
            })?;
        self.check_xdm_replaceground_deadline(start, "active DC executions")?;
        let canonical_table = self.dc_results_to_prn_table(
            &canonical_plan,
            &canonical_run_netlist,
            &canonical_results,
        )?;
        let projected_table = self.dc_results_to_prn_table(
            &projected_plan,
            &projected_run_netlist,
            &projected_results,
        )?;
        Self::validate_xdm_replaceground_analytic_table(&canonical_table, &canonical_results)?;
        Self::validate_xdm_replaceground_analytic_table(&projected_table, &projected_results)?;
        let comparison_mismatches = self.compare_xdm_replaceground_tables(
            &projected_table,
            &canonical_table,
            &projected_results,
            &canonical_results,
        )?;
        if !comparison_mismatches.is_empty() {
            return Err(format!(
                "canonical and projected HSPICE outputs differ under the effective Xyce 7.10 default verifier tolerance: {comparison_mismatches:?}"
            ));
        }
        self.check_xdm_replaceground_deadline(start, "effective verifier comparison")?;

        // Demonstrate causality rather than merely observing that two decks
        // which both happen to contain the same string agree.  Turning the
        // generated policy off must leave the authored HSPICE alias literal,
        // alter topology, and fail or numerically distinguish the circuit.
        let policy_off_source = projected_source.replacen(
            ".PREPROCESS REPLACEGROUND TRUE",
            ".PREPROCESS REPLACEGROUND FALSE",
            1,
        );
        Self::validate_xdm_replaceground_directives(&policy_off_source, 1, false)?;
        let policy_off_plan = self.static_dc_plan_for_source_with_execution_dir(
            &hspice_path,
            policy_off_source.clone(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let policy_off_netlist = Self::parse_netlist_with_expression_dialect(
            &policy_off_source,
            &hspice_path,
            ExpressionDialect::Xyce,
        )
        .map_err(|error| format!("policy-off HSPICE parse failed: {error}"))?;
        // Parser canonicalization intentionally represents an explicit FALSE
        // policy as `None`: both select literal authored-node semantics.
        Self::validate_xdm_replaceground_effective_options(
            &policy_off_netlist.options,
            false,
            "policy-off HSPICE",
        )?;
        let policy_off_structure =
            Self::xdm_replaceground_element_snapshot(&policy_off_netlist, kind)?;
        let authored_alias = kind.authored_alias().to_ascii_lowercase();
        if !policy_off_structure
            .iter()
            .any(|element| element.nodes.iter().any(|node| node == &authored_alias))
        {
            return Err(format!(
                "policy-off projection lost authored alias '{}' instead of retaining it literally",
                kind.authored_alias()
            ));
        }
        if policy_off_structure == canonical_structure {
            return Err(
                "REPLACEGROUND policy-off projection did not alter the circuit topology"
                    .to_string(),
            );
        }
        match self.run_static_dc_results(&policy_off_plan, start) {
            Err(SimulationError::Aborted) => {
                return Err(format!(
                    "shared XDM REPLACEGROUND deadline expired ({}ms)",
                    self.config.max_time_per_test_ms
                ));
            }
            Err(_) => {}
            Ok((policy_off_run_netlist, policy_off_results)) => {
                let policy_off_table = self.dc_results_to_prn_table(
                    &policy_off_plan,
                    &policy_off_run_netlist,
                    &policy_off_results,
                )?;
                let policy_off_comparison = self.compare_xdm_replaceground_tables(
                    &projected_table,
                    &policy_off_table,
                    &projected_results,
                    &policy_off_results,
                );
                if policy_off_comparison.is_ok_and(|mismatches| mismatches.is_empty()) {
                    return Err(
                        "REPLACEGROUND policy-off circuit unexpectedly reproduced the grounded output"
                            .to_string(),
                    );
                }
            }
        }
        self.check_xdm_replaceground_deadline(start, "policy-off causality")?;
        Ok(())
    }

    pub(super) fn validate_xdm_replaceground_identity(
        role: &str,
        record: &str,
        bytes: &[u8],
        expected: (usize, &str),
    ) -> Result<(), String> {
        let actual_hash = blake3::hash(bytes).to_hex().to_string();
        if bytes.len() != expected.0 || actual_hash != expected.1 {
            return Err(format!(
                "{role} identity changed for '{record}': expected {} bytes / {}, got {} bytes / {actual_hash}",
                expected.0,
                expected.1,
                bytes.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_xdm_replaceground_effective_options(
        options: &rspice_core::netlist::SimulationOptions,
        replace_ground: bool,
        role: &str,
    ) -> Result<(), String> {
        let expected_replace_ground = replace_ground.then_some(true);
        if options.replace_ground != expected_replace_ground
            || options.remove_unused.is_some()
            || options.tnom.map(Value::to_bits) != Some(25.0f64.to_bits())
            || options.measure_fail_output.is_some()
            || options.measure_default_value.is_some()
            || options.measure_use_cont_files.is_some()
            || !options.hb_num_frequencies.is_empty()
            || options.nonlinear_continuation.is_some()
            || options.reltol.is_some()
            || options.abstol.is_some()
            || options.vntol.is_some()
            || options.iabstol.is_some()
            || options.residual_reltol.is_some()
            || options.gmin.is_some()
            || options.method.is_some()
            || options.trtol.is_some()
            || options.timeint_reltol.is_some()
            || options.timeint_abstol.is_some()
            || options.transient_lte_reference.is_some()
            || options.transient_new_bp_stepping.is_some()
            || options.ramptime.is_some()
            || options.digital_delay_type.is_some()
            || options.xspice_event_trace_save.is_some()
            || options.itl1.is_some()
            || options.itl2.is_some()
            || options.itl4.is_some()
            || options.itl6.is_some()
            || options.chgtol.is_some()
            || options.pivtol.is_some()
            || options.temp.is_some()
            || options.seed.is_some()
            || options.allow_simplified_mos.is_some()
            || options.auto_bridge.is_some()
            || options.auto_bridge_show_generated.is_some()
            || options.auto_bridge_family.is_some()
            || !options.auto_bridge_templates.is_empty()
            || !options.auto_bridge_param_names.is_empty()
            || options.topology_supernode.is_some()
            || options.device_zero_resistance_tol.is_some()
            || options.b3soi_gmin_scaling.is_some()
            || options.device_try_to_compact.is_some()
        {
            return Err(format!(
                "{role} XDM REPLACEGROUND deck has unexpected effective option state; required only REPLACEGROUND={replace_ground} and DEVICE TNOM=25, got {options:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_xdm_replaceground_directives(
        source: &str,
        expected_count: usize,
        expected_value: bool,
    ) -> Result<(), String> {
        let mut values = Vec::new();
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim();
            let fields = trimmed.split_whitespace().collect::<Vec<_>>();
            if !fields
                .first()
                .is_some_and(|field| field.eq_ignore_ascii_case(".PREPROCESS"))
            {
                continue;
            }
            if fields.len() != 3 || !fields[1].eq_ignore_ascii_case("REPLACEGROUND") {
                return Err(format!(
                    "XDM REPLACEGROUND projection contains an unsupported .PREPROCESS card: '{trimmed}'"
                ));
            }
            let value = if fields[2].eq_ignore_ascii_case("TRUE") {
                true
            } else if fields[2].eq_ignore_ascii_case("FALSE") {
                false
            } else {
                return Err(format!(
                    "XDM REPLACEGROUND projection has invalid policy value '{}'",
                    fields[2]
                ));
            };
            values.push(value);
        }
        if values.len() != expected_count || values.iter().any(|value| *value != expected_value) {
            return Err(format!(
                "XDM REPLACEGROUND directive contract expected {expected_count} card(s) set to {expected_value}, got {values:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_xdm_replaceground_plan(
        plan: &XyceStaticDcPlan,
        _kind: XyceXdmReplaceGroundKind,
        role: &str,
    ) -> Result<(), String> {
        if plan.dc_data.is_some() || !plan.steps.is_empty() || !plan.diagnostics.is_empty() {
            return Err(format!(
                "{role} XDM REPLACEGROUND plan contains DATA/STEP/diagnostic state outside the bounded contract"
            ));
        }
        if plan.print.probes.len() != 2 {
            return Err(format!(
                "{role} XDM REPLACEGROUND plan requires one V(1), V(2) output"
            ));
        }
        if role == "canonical"
            && plan
                .print_format
                .as_deref()
                .is_none_or(|format| !format.eq_ignore_ascii_case("PROBE"))
        {
            return Err(
                "canonical XDM REPLACEGROUND deck lost its PROBE output format".to_string(),
            );
        }
        if role != "canonical" && plan.print_format.is_some() {
            return Err(
                "bounded HSPICE projection unexpectedly rewrote the authored output format"
                    .to_string(),
            );
        }
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if probes != ["v(1)", "v(2)"] {
            return Err(format!(
                "{role} XDM REPLACEGROUND plan has unexpected probes {probes:?}"
            ));
        }
        if !plan.dc.source.eq_ignore_ascii_case("VA")
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 10.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.dc.sweep2.is_some()
        {
            return Err(format!(
                "{role} XDM REPLACEGROUND plan has an unexpected DC sweep: {:?}",
                plan.dc
            ));
        }
        let expected_mode = rspice_core::netlist::DcSweepMode::Linear;
        if !Self::dc_sweep_modes_match_exactly(&plan.dc.mode, &expected_mode) {
            return Err(format!(
                "{role} XDM REPLACEGROUND plan has an unexpected DC sweep mode"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_xdm_replaceground_subcircuit_topology(
        netlist: &Netlist,
        kind: XyceXdmReplaceGroundKind,
    ) -> Result<(), String> {
        if !kind.requires_subcircuit() {
            if !netlist.subcircuits.is_empty() || netlist.elements.len() != 3 {
                return Err(
                    "flat XDM REPLACEGROUND record does not retain exactly three authored top-level elements"
                        .to_string(),
                );
            }
            let authored = netlist
                .elements
                .iter()
                .map(|element| {
                    let value = match &element.kind {
                        ElementKind::VoltageSource(spec) => extract_dc_value(spec),
                        ElementKind::Resistor { value, .. } => *value,
                        _ => Value::NAN,
                    };
                    (
                        element.name.to_ascii_lowercase(),
                        element
                            .nodes
                            .iter()
                            .map(|node| node.to_ascii_lowercase())
                            .collect::<Vec<_>>(),
                        value.to_bits(),
                    )
                })
                .collect::<Vec<_>>();
            let expected = vec![
                (
                    "va".to_string(),
                    vec!["1".to_string(), "0".to_string()],
                    0.0f64.to_bits(),
                ),
                (
                    "r1".to_string(),
                    vec!["1".to_string(), "2".to_string()],
                    10.0f64.to_bits(),
                ),
                (
                    "r2".to_string(),
                    vec!["2".to_string(), "0".to_string()],
                    10.0f64.to_bits(),
                ),
            ];
            if authored != expected {
                return Err(format!(
                    "flat XDM REPLACEGROUND authored snapshot differs: expected={expected:?}, actual={authored:?}"
                ));
            }
            return Ok(());
        }
        if netlist.subcircuits.len() != 1 {
            return Err(format!(
                "subcircuit REPLACEGROUND record requires one definition, found {}",
                netlist.subcircuits.len()
            ));
        }
        let definition = &netlist.subcircuits[0];
        if !definition.name.eq_ignore_ascii_case("subckt_resistor")
            || definition
                .ports
                .iter()
                .map(|port| port.to_ascii_lowercase())
                .collect::<Vec<_>>()
                != ["a", "b"]
            || definition.elements.len() != 3
            || definition.params.len() != 1
            || !definition.params[0].0.eq_ignore_ascii_case("resistance")
            || definition.params[0].1.to_bits() != 1.0f64.to_bits()
        {
            return Err(format!(
                "subcircuit REPLACEGROUND definition is outside the exact three-resistor parameterized topology: {definition:?}"
            ));
        }
        let expected_body = [
            ("r1", ["a", "a1"]),
            ("r2", ["a1", "a2"]),
            ("r3", ["a2", "b"]),
        ];
        for (element, (expected_name, expected_nodes)) in
            definition.elements.iter().zip(expected_body)
        {
            let ElementKind::Resistor {
                value_expr,
                model,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                return Err(format!(
                    "subcircuit authored body element '{}' is not a resistor",
                    element.name
                ));
            };
            if !element.name.eq_ignore_ascii_case(expected_name)
                || element
                    .nodes
                    .iter()
                    .map(|node| node.to_ascii_lowercase())
                    .collect::<Vec<_>>()
                    != expected_nodes
                || value_expr
                    .as_deref()
                    .is_none_or(|expression| !expression.eq_ignore_ascii_case("resistance"))
                || model.is_some()
                || !instance_params.is_empty()
                || !deferred_params.is_empty()
            {
                return Err(format!(
                    "subcircuit authored body snapshot differs at '{}': {element:?}",
                    element.name
                ));
            }
        }
        let instances = netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } => Some((element, subckt_name, params)),
                _ => None,
            })
            .collect::<Vec<_>>();
        if instances.len() != 1
            || !instances[0].0.name.eq_ignore_ascii_case("X1")
            || !instances[0].1.eq_ignore_ascii_case("subckt_resistor")
            || instances[0].0.nodes.len() != 2
            || !instances[0]
                .0
                .nodes
                .iter()
                .map(|node| node.to_ascii_lowercase())
                .eq(["2".to_string(), "0".to_string()])
            || instances[0].2.len() != 1
            || !instances[0].2[0].0.eq_ignore_ascii_case("resistance")
            || instances[0].2[0].1.as_value().map(Value::to_bits) != Some(5.0f64.to_bits())
        {
            return Err(format!(
                "subcircuit REPLACEGROUND instance is outside the exact X1 RESISTANCE=5 topology: {instances:?}"
            ));
        }
        let top_names = netlist
            .elements
            .iter()
            .map(|element| element.name.to_ascii_lowercase())
            .collect::<Vec<_>>();
        if top_names != ["va", "r1", "x1"]
            || !netlist.elements[0]
                .nodes
                .iter()
                .map(String::as_str)
                .eq(["1", "0"])
            || !netlist.elements[1]
                .nodes
                .iter()
                .map(String::as_str)
                .eq(["1", "2"])
            || extract_dc_value(match &netlist.elements[0].kind {
                ElementKind::VoltageSource(spec) => spec,
                _ => return Err("subcircuit top-level VA is not a DC voltage source".to_string()),
            })
            .to_bits()
                != 0.0f64.to_bits()
            || !matches!(&netlist.elements[1].kind, ElementKind::Resistor { value, .. } if value.to_bits() == 15.0f64.to_bits())
        {
            return Err(format!(
                "subcircuit REPLACEGROUND top-level authored snapshot differs: {:?}",
                netlist.elements
            ));
        }
        Ok(())
    }

    pub(super) fn validate_xdm_replaceground_analytic_table(
        table: &XycePrnTable,
        results: &[DcSweepPointResult],
    ) -> Result<(), String> {
        if table.columns.len() != 3
            || !table.columns[0].eq_ignore_ascii_case("Index")
            || !table.columns[1].eq_ignore_ascii_case("V(1)")
            || !table.columns[2].eq_ignore_ascii_case("V(2)")
            || table.rows.len() != 11
        {
            return Err(format!(
                "XDM REPLACEGROUND analytic table requires Index/V(1)/V(2) and 11 rows, got {:?} / {}",
                table.columns,
                table.rows.len()
            ));
        }
        if results.len() != table.rows.len() {
            return Err(format!(
                "XDM REPLACEGROUND raw sweep has {} points for {} table rows",
                results.len(),
                table.rows.len()
            ));
        }
        for (row_index, (row, point)) in table.rows.iter().zip(results).enumerate() {
            if row.len() != 3 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "XDM REPLACEGROUND row {row_index} is not a finite three-column row: {row:?}"
                ));
            }
            let sweep = point.sweep_value;
            if !sweep.is_finite()
                || (row[1] - sweep).abs() > 1.0e-10
                || (row[2] - 0.5 * sweep).abs() > 1.0e-10
            {
                return Err(format!(
                    "XDM REPLACEGROUND analytic invariant failed at row {row_index}: sweep={sweep}, V1={}, V2={} (expected V1=sweep, V2=sweep/2)",
                    row[1], row[2]
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_addresistors_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceAddResistorsKind,
        start: Instant,
    ) -> Result<(), String> {
        let source_bytes = self.validate_addresistors_provenance(deck, kind)?;
        self.check_addresistors_deadline(start, "provenance")?;
        let source = std::str::from_utf8(&source_bytes).map_err(|error| {
            format!(
                "ADDRESISTORS record '{}' is not UTF-8: {error}",
                kind.record()
            )
        })?;
        let original = Self::parse_xyce_netlist(source, &deck.path)
            .map_err(|error| format!("ADDRESISTORS original parse failed: {error}"))?;
        Self::validate_addresistors_original_netlist(&original, kind)?;
        self.check_addresistors_deadline(start, "original parse and snapshot")?;

        if kind.is_transient() {
            self.validate_addresistors_transient_oracle(deck, source, &original, kind, start)
        } else {
            self.validate_addresistors_bridge_oracle(deck, source, &original, kind, start)
        }
    }

    pub(super) fn validate_addresistors_bridge_oracle(
        &self,
        deck: &XyceDeck,
        source: &str,
        original: &Netlist,
        kind: XyceAddResistorsKind,
        start: Instant,
    ) -> Result<(), String> {
        if kind != XyceAddResistorsKind::RedundantBridge {
            return Err("ADDRESISTORS bridge oracle received a transient record".to_string());
        }
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            &deck.path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        Self::validate_removeunused_plan(&plan, XyceRemoveUnusedKind::ReplaceGround)?;
        Self::validate_removeunused_authored_hierarchy(
            original,
            XyceRemoveUnusedKind::ReplaceGround,
        )?;
        Self::validate_removeunused_flattened_topology(
            original,
            XyceRemoveUnusedKind::ReplaceGround,
            false,
        )?;
        let original_results = self.run_addresistors_dc_netlist(&plan, original, start)?;
        let original_table = self.dc_results_to_prn_table(&plan, original, &original_results)?;
        Self::validate_removeunused_analytic_table(
            &original_table,
            &original_results,
            XyceRemoveUnusedKind::ReplaceGround,
        )?;

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let materialized = original
            .materialize_xyce_add_resistors_with_abort(&abort)
            .map_err(|error| format!("ADDRESISTORS bridge materialization failed: {error}"))?;
        Self::validate_addresistors_report(&materialized.report, kind)?;
        Self::validate_addresistors_materialized_netlist(&materialized.netlist, kind)?;
        Self::validate_removeunused_authored_hierarchy(
            &materialized.netlist,
            XyceRemoveUnusedKind::ReplaceGround,
        )?;
        Self::validate_removeunused_flattened_topology(
            &materialized.netlist,
            XyceRemoveUnusedKind::ReplaceGround,
            false,
        )?;
        let replayed = Self::parse_xyce_netlist(&materialized.derived_source, &deck.path)
            .map_err(|error| format!("ADDRESISTORS bridge artifact parse failed: {error}"))?;
        Self::validate_addresistors_replayed_artifact(&replayed, &materialized.report, kind)?;
        let generated_results = self.run_addresistors_dc_netlist(&plan, &replayed, start)?;
        let generated_table = self.dc_results_to_prn_table(&plan, &replayed, &generated_results)?;
        Self::validate_removeunused_analytic_table(
            &generated_table,
            &generated_results,
            XyceRemoveUnusedKind::ReplaceGround,
        )?;
        let mismatches = self.compare_xdm_replaceground_tables(
            &original_table,
            &generated_table,
            &original_results,
            &generated_results,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "ADDRESISTORS bridge original/generated comparison produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ));
        }
        self.check_addresistors_deadline(start, "bridge original/generated comparison")
    }

    pub(super) fn validate_addresistors_original_netlist(
        netlist: &Netlist,
        kind: XyceAddResistorsKind,
    ) -> Result<(), String> {
        if !netlist.diagnostics.is_empty() {
            return Err(format!(
                "ADDRESISTORS original parse produced diagnostics: {:?}",
                netlist.diagnostics
            ));
        }
        Self::validate_addresistors_options(netlist, kind, false)?;
        if kind == XyceAddResistorsKind::RedundantBridge {
            Self::validate_removeunused_authored_hierarchy(
                netlist,
                XyceRemoveUnusedKind::ReplaceGround,
            )
        } else {
            Self::validate_addresistors_authored_hierarchy(netlist, kind)
        }
    }

    pub(super) fn validate_addresistors_materialized_netlist(
        netlist: &Netlist,
        kind: XyceAddResistorsKind,
    ) -> Result<(), String> {
        if !netlist.diagnostics.is_empty() {
            return Err(format!(
                "ADDRESISTORS materialized copy produced diagnostics: {:?}",
                netlist.diagnostics
            ));
        }
        Self::validate_addresistors_options(netlist, kind, true)?;
        if kind != XyceAddResistorsKind::RedundantBridge {
            Self::validate_addresistors_authored_hierarchy(netlist, kind)?;
        }
        Ok(())
    }

    pub(super) fn validate_addresistors_replayed_artifact(
        netlist: &Netlist,
        report: &rspice_core::netlist::XyceAddResistorsReport,
        kind: XyceAddResistorsKind,
    ) -> Result<(), String> {
        if !netlist.diagnostics.is_empty() {
            return Err(format!(
                "ADDRESISTORS replayed artifact produced diagnostics: {:?}",
                netlist.diagnostics
            ));
        }
        Self::validate_addresistors_options(netlist, kind, true)?;
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("ADDRESISTORS artifact did not replay: {error}"))?;
        let artifact_cards = flattened
            .elements
            .iter()
            .filter(|element| {
                let name = element.name.to_ascii_uppercase();
                name.starts_with("RONETERM") || name.starts_with("RNODCPATH")
            })
            .collect::<Vec<_>>();
        if artifact_cards.len() != report.generated.len() {
            return Err(format!(
                "ADDRESISTORS replayed artifact has {} generated card(s), expected {}: {artifact_cards:?}",
                artifact_cards.len(),
                report.generated.len()
            ));
        }
        for generated in &report.generated {
            let element = artifact_cards
                .iter()
                .copied()
                .find(|element| element.name.eq_ignore_ascii_case(&generated.name))
                .ok_or_else(|| {
                    format!(
                        "ADDRESISTORS replayed artifact is missing '{}'",
                        generated.name
                    )
                })?;
            let value = match &element.kind {
                ElementKind::Resistor { value, .. } => *value,
                other => {
                    return Err(format!(
                        "ADDRESISTORS artifact card '{}' replayed as {other:?}",
                        generated.name
                    ));
                }
            };
            if element.nodes != [generated.node.as_str(), "0"]
                || value.to_bits() != generated.resistance.to_bits()
                || !matches!(element.provenance, ElementProvenance::Authored)
            {
                return Err(format!(
                    "ADDRESISTORS artifact card '{}' did not replay canonically: expected node {}/{}, value {}, authored provenance; got {element:?}",
                    generated.name, generated.node, generated.artifact_node, generated.resistance
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_addresistors_options(
        netlist: &Netlist,
        kind: XyceAddResistorsKind,
        materialized: bool,
    ) -> Result<(), String> {
        let options = &netlist.options;
        let policy = options.add_resistors.as_ref();
        if materialized {
            if policy.is_some() {
                return Err(
                    "ADDRESISTORS materialized copy retained an active generation policy"
                        .to_string(),
                );
            }
        } else {
            let policy = policy.ok_or_else(|| {
                "ADDRESISTORS original netlist has no typed generation policy".to_string()
            })?;
            let (one_raw, one_line, nodc_raw, nodc_line) = match kind {
                XyceAddResistorsKind::NoDcPath => ("0.0001", 71, "1", 70),
                XyceAddResistorsKind::OneTerminal => ("1", 69, "0.0001", 68),
                XyceAddResistorsKind::RedundantBridge => ("1G", 108, "1G", 107),
            };
            let one = policy.one_terminal.as_ref();
            let nodc = policy.no_dc_path.as_ref();
            if one.is_none_or(|spec| spec.raw_resistance != one_raw || spec.source_line != one_line)
                || nodc.is_none_or(|spec| {
                    spec.raw_resistance != nodc_raw || spec.source_line != nodc_line
                })
            {
                return Err(format!(
                    "ADDRESISTORS typed policy changed: expected ONE={one_raw}@{one_line}, NODC={nodc_raw}@{nodc_line}; got {policy:?}"
                ));
            }
        }

        let expects_timeint = kind.is_transient();
        let expects_remove = kind == XyceAddResistorsKind::RedundantBridge;
        let expected_types = [
            rspice_core::netlist::RemoveUnusedDeviceType::Capacitor,
            rspice_core::netlist::RemoveUnusedDeviceType::Diode,
            rspice_core::netlist::RemoveUnusedDeviceType::CurrentSource,
            rspice_core::netlist::RemoveUnusedDeviceType::Inductor,
            rspice_core::netlist::RemoveUnusedDeviceType::Mosfet,
            rspice_core::netlist::RemoveUnusedDeviceType::Bjt,
            rspice_core::netlist::RemoveUnusedDeviceType::Resistor,
            rspice_core::netlist::RemoveUnusedDeviceType::VoltageSource,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if options.replace_ground != expects_remove.then_some(true)
            || options
                .remove_unused
                .as_ref()
                .map(|policy| &policy.device_types)
                != expects_remove.then_some(&expected_types)
            || options.timeint_reltol.map(Value::to_bits)
                != expects_timeint.then_some(1.0e-6f64.to_bits())
            || options.timeint_abstol.map(Value::to_bits)
                != expects_timeint.then_some(1.0e-12f64.to_bits())
            || options.measure_fail_output.is_some()
            || options.measure_default_value.is_some()
            || options.measure_use_cont_files.is_some()
            || !options.hb_num_frequencies.is_empty()
            || options.nonlinear_continuation.is_some()
            || options.reltol.is_some()
            || options.abstol.is_some()
            || options.vntol.is_some()
            || options.iabstol.is_some()
            || options.residual_reltol.is_some()
            || options.gmin.is_some()
            || options.method.is_some()
            || options.trtol.is_some()
            || options.transient_lte_reference.is_some()
            || options.transient_new_bp_stepping.is_some()
            || options.ramptime.is_some()
            || options.digital_delay_type.is_some()
            || options.xspice_event_trace_save.is_some()
            || options.itl1.is_some()
            || options.itl2.is_some()
            || options.itl4.is_some()
            || options.itl6.is_some()
            || options.chgtol.is_some()
            || options.pivtol.is_some()
            || options.temp.is_some()
            || options.tnom.is_some()
            || options.seed.is_some()
            || options.allow_simplified_mos.is_some()
            || options.auto_bridge.is_some()
            || options.auto_bridge_show_generated.is_some()
            || options.auto_bridge_family.is_some()
            || !options.auto_bridge_templates.is_empty()
            || !options.auto_bridge_param_names.is_empty()
            || options.topology_supernode.is_some()
            || options.device_zero_resistance_tol.is_some()
            || options.b3soi_gmin_scaling.is_some()
            || options.device_try_to_compact.is_some()
        {
            return Err(format!(
                "ADDRESISTORS record '{}' has unexpected exact options state: {options:?}",
                kind.record()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_addresistors_authored_hierarchy(
        netlist: &Netlist,
        kind: XyceAddResistorsKind,
    ) -> Result<(), String> {
        if !netlist.models.is_empty() || netlist.subcircuits.len() != 1 {
            return Err(format!(
                "ADDRESISTORS transient authored model/subcircuit census changed: models={:?}, subcircuits={:?}",
                netlist.models, netlist.subcircuits
            ));
        }
        let mut top = netlist
            .elements
            .iter()
            .filter(|element| {
                matches!(
                    element.provenance,
                    rspice_core::netlist::ElementProvenance::Authored
                )
            })
            .map(Self::addresistors_element_snapshot)
            .collect::<Result<Vec<_>, _>>()?;
        top.sort_by(|left, right| left.name.cmp(&right.name));
        let definition = &netlist.subcircuits[0];
        if !definition.name.eq_ignore_ascii_case("capacitor")
            || definition.ports != ["1".to_string()]
            || !definition.initial_conditions.is_empty()
            || !definition.node_sets.is_empty()
            || !definition.params.is_empty()
            || !definition.expr_params.is_empty()
            || !definition.string_params.is_empty()
            || !definition.body_params.is_empty()
            || !definition.body_expr_params.is_empty()
            || !definition.body_string_params.is_empty()
            || !definition.body_functions.is_empty()
            || !definition.local_options.is_empty()
            || definition.library_ref.is_some()
            || !definition.nested_subcircuits.is_empty()
        {
            return Err(format!(
                "ADDRESISTORS capacitor definition changed: {definition:?}"
            ));
        }
        let mut body = definition
            .elements
            .iter()
            .map(Self::addresistors_element_snapshot)
            .collect::<Result<Vec<_>, _>>()?;
        body.sort_by(|left, right| left.name.cmp(&right.name));
        let (expected_top, expected_body) = Self::expected_addresistors_authored(kind);
        if top != expected_top || body != expected_body {
            return Err(format!(
                "ADDRESISTORS exact authored hierarchy changed: expected top={expected_top:#?}, body={expected_body:#?}; actual top={top:#?}, body={body:#?}"
            ));
        }
        Ok(())
    }

    pub(super) fn expected_addresistors_authored(
        kind: XyceAddResistorsKind,
    ) -> (
        Vec<XyceAddResistorsElementSnapshot>,
        Vec<XyceAddResistorsElementSnapshot>,
    ) {
        let authored = |name: &str,
                        nodes: &[&str],
                        device: &str,
                        value: Option<Value>,
                        initial: Option<Value>,
                        model: Option<&str>| {
            XyceAddResistorsElementSnapshot {
                name: name.to_ascii_lowercase(),
                nodes: nodes.iter().map(|node| node.to_ascii_lowercase()).collect(),
                kind: device.to_string(),
                value_bits: value.map(Value::to_bits),
                initial_value_bits: initial.map(Value::to_bits),
                model: model.map(str::to_ascii_lowercase),
                provenance: "authored".to_string(),
            }
        };
        let (mut top, mut body) = match kind {
            XyceAddResistorsKind::NoDcPath => (
                vec![
                    authored("Vin", &["1", "0"], "V", Some(1.0), None, None),
                    authored("C1", &["1", "2"], "C", Some(0.5), Some(0.0), None),
                    authored("C2", &["1", "2"], "C", Some(0.5), None, None),
                    authored("X1", &["1"], "X", None, None, Some("capacitor")),
                ],
                vec![
                    authored("C1", &["1", "2"], "C", Some(0.5), None, None),
                    authored("C2", &["1", "2"], "C", Some(0.5), Some(0.0), None),
                ],
            ),
            XyceAddResistorsKind::OneTerminal => (
                vec![
                    authored("Vin", &["1", "0"], "V", Some(1.0), None, None),
                    authored("C1", &["1", "2"], "C", Some(1.0), Some(0.0), None),
                    authored("X1", &["1"], "X", None, None, Some("capacitor")),
                ],
                vec![authored("C2", &["1", "2"], "C", Some(1.0), Some(0.0), None)],
            ),
            XyceAddResistorsKind::RedundantBridge => unreachable!("bridge has its own snapshot"),
        };
        top.sort_by(|left, right| left.name.cmp(&right.name));
        body.sort_by(|left, right| left.name.cmp(&right.name));
        (top, body)
    }

    pub(super) fn validate_addresistors_flattened_topology(
        netlist: &Netlist,
        kind: XyceAddResistorsKind,
        materialized: bool,
    ) -> Result<(), String> {
        if kind == XyceAddResistorsKind::RedundantBridge {
            return Self::validate_removeunused_flattened_topology(
                netlist,
                XyceRemoveUnusedKind::ReplaceGround,
                false,
            );
        }
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("ADDRESISTORS flattening failed: {error}"))?;
        let mut actual = flattened
            .elements
            .iter()
            .map(Self::addresistors_element_snapshot)
            .collect::<Result<Vec<_>, _>>()?;
        actual.sort_by(|left, right| left.name.cmp(&right.name));
        let mut expected = Self::expected_addresistors_flattened(kind, materialized);
        expected.sort_by(|left, right| left.name.cmp(&right.name));
        if actual != expected {
            return Err(format!(
                "ADDRESISTORS {} exact flattened topology changed: expected={expected:#?}, actual={actual:#?}",
                if materialized {
                    "generated"
                } else {
                    "original"
                }
            ));
        }
        Ok(())
    }

    pub(super) fn expected_addresistors_flattened(
        kind: XyceAddResistorsKind,
        materialized: bool,
    ) -> Vec<XyceAddResistorsElementSnapshot> {
        let element = |name: &str,
                       nodes: &[&str],
                       device: &str,
                       value: Value,
                       initial: Option<Value>,
                       provenance: &str| {
            XyceAddResistorsElementSnapshot {
                name: name.to_ascii_lowercase(),
                nodes: nodes.iter().map(|node| node.to_ascii_lowercase()).collect(),
                kind: device.to_string(),
                value_bits: Some(value.to_bits()),
                initial_value_bits: initial.map(Value::to_bits),
                model: None,
                provenance: provenance.to_string(),
            }
        };
        let mut expected = match kind {
            XyceAddResistorsKind::NoDcPath => vec![
                element("Vin", &["1", "0"], "V", 1.0, None, "authored"),
                element("C1", &["1", "2"], "C", 0.5, Some(0.0), "authored"),
                element("C2", &["1", "2"], "C", 0.5, None, "authored"),
                element("X1.C1", &["1", "X1.2"], "C", 0.5, None, "authored"),
                element("X1.C2", &["1", "X1.2"], "C", 0.5, Some(0.0), "authored"),
            ],
            XyceAddResistorsKind::OneTerminal => vec![
                element("Vin", &["1", "0"], "V", 1.0, None, "authored"),
                element("C1", &["1", "2"], "C", 1.0, Some(0.0), "authored"),
                element("X1.C2", &["1", "X1.2"], "C", 1.0, Some(0.0), "authored"),
            ],
            XyceAddResistorsKind::RedundantBridge => unreachable!("bridge uses REMOVEUNUSED"),
        };
        if materialized {
            let (prefix, provenance) = match kind {
                XyceAddResistorsKind::NoDcPath => ("RNODCPATH", "generated:NoDcPath"),
                XyceAddResistorsKind::OneTerminal => ("RONETERM", "generated:OneTerminal"),
                XyceAddResistorsKind::RedundantBridge => unreachable!(),
            };
            expected.extend([
                element(
                    &format!("{prefix}1"),
                    &["2", "0"],
                    "R",
                    1.0,
                    None,
                    provenance,
                ),
                element(
                    &format!("{prefix}2"),
                    &["X1.2", "0"],
                    "R",
                    1.0,
                    None,
                    provenance,
                ),
            ]);
        }
        expected
    }

    pub(super) fn validate_addresistors_report(
        report: &rspice_core::netlist::XyceAddResistorsReport,
        kind: XyceAddResistorsKind,
    ) -> Result<(), String> {
        use rspice_core::netlist::XyceAddResistorMode::{NoDcPath, OneTerminal};
        let expected_configured: Vec<(
            rspice_core::netlist::XyceAddResistorMode,
            &'static str,
            usize,
        )> = match kind {
            XyceAddResistorsKind::NoDcPath => {
                vec![(OneTerminal, "0.0001", 71), (NoDcPath, "1", 70)]
            }
            XyceAddResistorsKind::OneTerminal => {
                vec![(OneTerminal, "1", 69), (NoDcPath, "0.0001", 68)]
            }
            XyceAddResistorsKind::RedundantBridge => {
                vec![(OneTerminal, "1G", 108), (NoDcPath, "1G", 107)]
            }
        };
        if report.configured_modes.len() != expected_configured.len() {
            return Err(format!(
                "ADDRESISTORS configured-mode provenance changed: expected {expected_configured:?}, got {report:?}"
            ));
        }
        for (actual, (mode, raw, line)) in report.configured_modes.iter().zip(expected_configured) {
            if actual.mode != mode || actual.raw_resistance != raw || actual.source_line != line {
                return Err(format!(
                    "ADDRESISTORS configured mode changed: expected {mode:?}/{raw}@{line}, got {actual:?}"
                ));
            }
        }
        let expected_resolved: Vec<(
            rspice_core::netlist::XyceAddResistorMode,
            &'static str,
            Value,
            usize,
        )> = match kind {
            XyceAddResistorsKind::NoDcPath => vec![(NoDcPath, "1", 1.0, 70)],
            XyceAddResistorsKind::OneTerminal => vec![(OneTerminal, "1", 1.0, 69)],
            XyceAddResistorsKind::RedundantBridge => Vec::new(),
        };
        if report.resolved_modes.len() != expected_resolved.len() {
            return Err(format!(
                "ADDRESISTORS emitted-mode resolution changed: expected {expected_resolved:?}, got {report:?}"
            ));
        }
        for (actual, (mode, raw, value, line)) in
            report.resolved_modes.iter().zip(expected_resolved)
        {
            if actual.mode != mode
                || actual.raw_resistance != raw
                || actual.resistance.to_bits() != value.to_bits()
                || actual.source_line != line
            {
                return Err(format!(
                    "ADDRESISTORS resolved-mode provenance changed: expected {mode:?}/{raw}/{value}@{line}, got {actual:?}"
                ));
            }
        }
        let expected_nodes = vec!["2".to_string(), "X1.2".to_string()];
        let (expected_one, expected_nodc) = match kind {
            XyceAddResistorsKind::NoDcPath => (Vec::new(), expected_nodes.clone()),
            XyceAddResistorsKind::OneTerminal => (expected_nodes.clone(), Vec::new()),
            XyceAddResistorsKind::RedundantBridge => (Vec::new(), Vec::new()),
        };
        if report.one_terminal_candidates != expected_one
            || report.no_dc_path_candidates != expected_nodc
        {
            return Err(format!(
                "ADDRESISTORS exact connectivity classification changed: expected ONE={expected_one:?}, NODC={expected_nodc:?}; got ONE={:?}, NODC={:?}",
                report.one_terminal_candidates, report.no_dc_path_candidates
            ));
        }
        let expected_generated: Vec<(
            &'static str,
            &'static str,
            &'static str,
            rspice_core::netlist::XyceAddResistorMode,
            &'static str,
            Value,
        )> = match kind {
            XyceAddResistorsKind::NoDcPath => vec![
                ("RNODCPATH1", "2", "2", NoDcPath, "1", 1.0),
                ("RNODCPATH2", "X1.2", "X1:2", NoDcPath, "1", 1.0),
            ],
            XyceAddResistorsKind::OneTerminal => vec![
                ("RONETERM1", "2", "2", OneTerminal, "1", 1.0),
                ("RONETERM2", "X1.2", "X1:2", OneTerminal, "1", 1.0),
            ],
            XyceAddResistorsKind::RedundantBridge => Vec::new(),
        };
        if report.generated.len() != expected_generated.len() {
            return Err(format!(
                "ADDRESISTORS generated-resistor count changed: expected {}, got {:?}",
                expected_generated.len(),
                report.generated
            ));
        }
        for (actual, (name, node, artifact_node, mode, raw, value)) in
            report.generated.iter().zip(expected_generated)
        {
            if actual.name != name
                || actual.node != node
                || actual.artifact_node != artifact_node
                || actual.mode != mode
                || actual.raw_resistance != raw
                || actual.resistance.to_bits() != value.to_bits()
            {
                return Err(format!(
                    "ADDRESISTORS generated report changed: expected {name}/{node}/{artifact_node}/{mode:?}/{raw}/{value}, got {actual:?}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_addresistors_exp_invariant(table: &XycePrnTable) -> Result<(), String> {
        for (row_index, row) in table.rows.iter().enumerate() {
            let expected = (-row[1]).exp();
            for (column, actual) in [("V(2)", row[2]), ("V(X1:2)", row[3])] {
                let error = (actual - expected).abs();
                if error > 2.0e-4 * expected.abs().max(1.0e-6) + 1.0e-9 {
                    return Err(format!(
                        "ADDRESISTORS analytic exp(-t) invariant failed for {column} at row {row_index}: time={}, expected={expected}, actual={actual}",
                        row[1]
                    ));
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn validate_addresistors_family_census(
        &self,
        family_name: &str,
        expected_complete_count: usize,
        expected_complete_hash: &str,
        expected_content_hash: &str,
        expected_physical_count: usize,
        expected_physical_hash: &str,
        manifest_prefix: &str,
        expected_manifest_count: usize,
        expected_manifest_hash: &str,
    ) -> Result<(), String> {
        let family_dir = self.root.join("Netlists").join(family_name);
        let metadata = fs::symlink_metadata(&family_dir)
            .map_err(|error| format!("ADDRESISTORS family {family_name} is missing: {error}"))?;
        if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
            return Err(format!(
                "ADDRESISTORS family {} must be a regular non-symlink directory",
                family_dir.display()
            ));
        }
        let mut complete = BTreeSet::new();
        let mut content = BTreeSet::new();
        let mut physical = BTreeSet::new();
        for entry in fs::read_dir(&family_dir)
            .map_err(|error| format!("failed to inspect {family_name}: {error}"))?
        {
            let entry = entry.map_err(|error| {
                format!("failed to inspect ADDRESISTORS family member: {error}")
            })?;
            let member_metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
                format!(
                    "failed to inspect ADDRESISTORS family member {}: {error}",
                    entry.path().display()
                )
            })?;
            if !member_metadata.file_type().is_file() || member_metadata.file_type().is_symlink() {
                return Err(format!(
                    "ADDRESISTORS family member {} must be a regular non-symlink file",
                    entry.path().display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| "ADDRESISTORS family filename is not UTF-8".to_string())?
                .to_ascii_lowercase();
            if !complete.insert(name.clone()) {
                return Err(format!(
                    "ADDRESISTORS family contains case-colliding name {name:?}"
                ));
            }
            let bytes = fs::read(entry.path()).map_err(|error| {
                format!(
                    "failed to hash ADDRESISTORS family member {}: {error}",
                    entry.path().display()
                )
            })?;
            content.insert(format!("{name}\0{}", blake3::hash(&bytes).to_hex()));
            if name.ends_with(".cir") {
                physical.insert(name);
            }
        }
        let complete = complete.into_iter().collect::<Vec<_>>();
        let content = content.into_iter().collect::<Vec<_>>();
        let physical = physical.into_iter().collect::<Vec<_>>();
        let complete_hash = blake3::hash(complete.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let physical_hash = blake3::hash(physical.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let manifest = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(manifest_prefix))
            .cloned()
            .collect::<Vec<_>>();
        let manifest_hash = blake3::hash(manifest.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if complete.len() != expected_complete_count
            || complete_hash != expected_complete_hash
            || content.len() != expected_complete_count
            || content_hash != expected_content_hash
            || physical.len() != expected_physical_count
            || physical_hash != expected_physical_hash
            || manifest.len() != expected_manifest_count
            || manifest_hash != expected_manifest_hash
        {
            return Err(format!(
                "ADDRESISTORS {family_name} census changed: complete={}/{complete_hash}, content={}/{content_hash}, physical={}/{physical_hash}, manifest={}/{manifest_hash}",
                complete.len(),
                content.len(),
                physical.len(),
                manifest.len()
            ));
        }
        if family_dir.join("options").exists() {
            return Err(format!(
                "ADDRESISTORS {family_name} unexpectedly contains an options sidecar"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_removeunused_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceRemoveUnusedKind,
        start: Instant,
    ) -> Result<(), String> {
        self.validate_removeunused_provenance(deck, kind)?;
        self.check_removeunused_deadline(start, "provenance")?;

        let source_bytes = fs::read(&deck.path).map_err(|error| {
            format!(
                "failed to read REMOVEUNUSED record {}: {error}",
                deck.path.display()
            )
        })?;
        Self::validate_xdm_replaceground_identity(
            "REMOVEUNUSED source",
            kind.record(),
            &source_bytes,
            kind.source_identity(),
        )?;
        let source = std::str::from_utf8(&source_bytes).map_err(|error| {
            format!(
                "REMOVEUNUSED record '{}' is not UTF-8: {error}",
                kind.record()
            )
        })?;
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            &deck.path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        Self::validate_removeunused_plan(&plan, kind)?;
        self.check_removeunused_deadline(start, "plan validation")?;

        let (netlist, results) =
            self.run_static_dc_results(&plan, start)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!(
                        "REMOVEUNUSED active execution exceeded shared timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    other => format!("REMOVEUNUSED active DC execution failed: {other}"),
                })?;
        Self::validate_removeunused_netlist(&netlist, kind)?;
        Self::validate_removeunused_flattened_topology(&netlist, kind, false)?;
        if results.len() != 21 {
            return Err(format!(
                "REMOVEUNUSED DC execution produced {} points instead of 21",
                results.len()
            ));
        }
        let actual = self.dc_results_to_prn_table(&plan, &netlist, &results)?;
        Self::validate_removeunused_analytic_table(&actual, &results, kind)?;
        let dynamic_gold = Self::removeunused_dynamic_gold_table(&actual, kind)?;
        let mismatches =
            self.compare_xdm_replaceground_tables(&dynamic_gold, &actual, &results, &results)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "REMOVEUNUSED dynamic-gold wrapper comparison produced {} mismatch(es): {:?}",
                mismatches.len(),
                mismatches
            ));
        }
        self.check_removeunused_deadline(start, "active execution and dynamic gold")?;

        // The removed wrapper's dynamic gold derives V(2) from the same run's
        // V(1), so execution alone is common-mode vulnerable.  Disable the
        // preprocessing card and require both the exact pre-removal topology
        // and a failure or numerical distinction from the active run.
        let control = ".PREPROCESS removeunused c,d,i,l,m,q,r,v";
        if source.matches(control).count() != 1 {
            return Err(
                "REMOVEUNUSED source no longer owns exactly one canonical control card".to_string(),
            );
        }
        let policy_off_source = source.replacen(control, &format!("*{control}"), 1);
        let policy_off_plan = self.static_dc_plan_for_source_with_execution_dir(
            &deck.path,
            policy_off_source,
            ExpressionDialect::Xyce,
            None,
        )?;
        let policy_off_netlist =
            Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
                &policy_off_plan.source,
                &deck.path,
                ExpressionDialect::Xyce,
                ParameterRedefinitionPolicy::UseLast,
                None,
            )
            .map_err(|error| format!("REMOVEUNUSED policy-off parse failed: {error}"))?;
        Self::validate_removeunused_flattened_topology(&policy_off_netlist, kind, true)?;
        match self.run_static_dc_results(&policy_off_plan, start) {
            Err(SimulationError::Aborted) => {
                return Err(format!(
                    "REMOVEUNUSED policy-off execution exceeded shared timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ));
            }
            Err(_) => {}
            Ok((policy_off_run_netlist, policy_off_results)) => {
                let policy_off_table = self.dc_results_to_prn_table(
                    &policy_off_plan,
                    &policy_off_run_netlist,
                    &policy_off_results,
                )?;
                if self
                    .compare_xdm_replaceground_tables(
                        &dynamic_gold,
                        &policy_off_table,
                        &results,
                        &policy_off_results,
                    )
                    .is_ok_and(|mismatches| mismatches.is_empty())
                {
                    return Err(
                        "REMOVEUNUSED policy-off circuit unexpectedly reproduced the active output"
                            .to_string(),
                    );
                }
            }
        }
        self.check_removeunused_deadline(start, "policy-off causality")
    }

    pub(super) fn validate_removeunused_plan(
        plan: &XyceStaticDcPlan,
        kind: XyceRemoveUnusedKind,
    ) -> Result<(), String> {
        if !plan.steps.is_empty() || plan.dc_data.is_some() || !plan.diagnostics.is_empty() {
            return Err(format!(
                "REMOVEUNUSED plan contains STEP/DATA/diagnostic state outside the wrapper contract: steps={:?}, data={}, diagnostics={:?}",
                plan.steps,
                plan.dc_data.is_some(),
                plan.diagnostics
            ));
        }
        if plan.print_format.is_some()
            || plan.print.probes != ["V(1)".to_string(), "V(2)".to_string()]
        {
            return Err(format!(
                "REMOVEUNUSED wrapper requires one default .PRINT DC V(1) V(2), got format={:?}, probes={:?}",
                plan.print_format, plan.print.probes
            ));
        }
        if !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.start.to_bits() != (-1.0f64).to_bits()
            || plan.dc.stop.to_bits() != 1.0f64.to_bits()
            || plan.dc.step.to_bits() != 0.1f64.to_bits()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
        {
            return Err(format!(
                "REMOVEUNUSED wrapper has an unexpected DC sweep: {:?}",
                plan.dc
            ));
        }
        let replace_count = plan
            .source
            .lines()
            .filter(|line| {
                line.trim()
                    .eq_ignore_ascii_case(".PREPROCESS replaceground true")
            })
            .count();
        if replace_count != usize::from(kind.replace_ground()) {
            return Err(format!(
                "REMOVEUNUSED record '{}' has {replace_count} REPLACEGROUND controls",
                kind.record()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_removeunused_netlist(
        netlist: &Netlist,
        kind: XyceRemoveUnusedKind,
    ) -> Result<(), String> {
        let expected_types = [
            rspice_core::netlist::RemoveUnusedDeviceType::Capacitor,
            rspice_core::netlist::RemoveUnusedDeviceType::Diode,
            rspice_core::netlist::RemoveUnusedDeviceType::CurrentSource,
            rspice_core::netlist::RemoveUnusedDeviceType::Inductor,
            rspice_core::netlist::RemoveUnusedDeviceType::Mosfet,
            rspice_core::netlist::RemoveUnusedDeviceType::Bjt,
            rspice_core::netlist::RemoveUnusedDeviceType::Resistor,
            rspice_core::netlist::RemoveUnusedDeviceType::VoltageSource,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        let options = &netlist.options;
        if options.replace_ground != kind.replace_ground().then_some(true)
            || options.add_resistors.is_some()
            || options
                .remove_unused
                .as_ref()
                .map(|policy| &policy.device_types)
                != Some(&expected_types)
            || options.measure_fail_output.is_some()
            || options.measure_default_value.is_some()
            || options.measure_use_cont_files.is_some()
            || !options.hb_num_frequencies.is_empty()
            || options.nonlinear_continuation.is_some()
            || options.reltol.is_some()
            || options.abstol.is_some()
            || options.vntol.is_some()
            || options.iabstol.is_some()
            || options.residual_reltol.is_some()
            || options.gmin.is_some()
            || options.method.is_some()
            || options.trtol.is_some()
            || options.timeint_reltol.is_some()
            || options.timeint_abstol.is_some()
            || options.transient_lte_reference.is_some()
            || options.transient_new_bp_stepping.is_some()
            || options.ramptime.is_some()
            || options.digital_delay_type.is_some()
            || options.xspice_event_trace_save.is_some()
            || options.itl1.is_some()
            || options.itl2.is_some()
            || options.itl4.is_some()
            || options.itl6.is_some()
            || options.chgtol.is_some()
            || options.pivtol.is_some()
            || options.temp.is_some()
            || options.tnom.is_some()
            || options.seed.is_some()
            || options.allow_simplified_mos.is_some()
            || options.auto_bridge.is_some()
            || options.auto_bridge_show_generated.is_some()
            || options.auto_bridge_family.is_some()
            || !options.auto_bridge_templates.is_empty()
            || !options.auto_bridge_param_names.is_empty()
            || options.topology_supernode.is_some()
            || options.device_zero_resistance_tol.is_some()
            || options.b3soi_gmin_scaling.is_some()
            || options.device_try_to_compact.is_some()
        {
            return Err(format!(
                "REMOVEUNUSED record '{}' has unexpected exact preprocessing/options state {:?}",
                kind.record(),
                netlist.options
            ));
        }
        if !netlist.diagnostics.is_empty() {
            return Err(format!(
                "REMOVEUNUSED record '{}' produced diagnostics: {:?}",
                kind.record(),
                netlist.diagnostics
            ));
        }
        Self::validate_removeunused_authored_hierarchy(netlist, kind)?;
        Ok(())
    }

    pub(super) fn validate_removeunused_authored_hierarchy(
        netlist: &Netlist,
        kind: XyceRemoveUnusedKind,
    ) -> Result<(), String> {
        let mut models = netlist
            .models
            .iter()
            .map(|model| {
                if !model.params.is_empty()
                    || !model.expr_params.is_empty()
                    || !model.string_params.is_empty()
                    || !model.string_vector_params.is_empty()
                    || !model.real_vector_params.is_empty()
                    || !model.real_vector_expr_params.is_empty()
                    || !model.integer_vector_params.is_empty()
                {
                    return Err(format!(
                        "REMOVEUNUSED model '{}' unexpectedly owns parameters: {model:?}",
                        model.name
                    ));
                }
                Ok((
                    model.name.to_ascii_lowercase(),
                    model.model_type.to_ascii_lowercase(),
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        models.sort();
        let expected_models = vec![
            ("dmod".to_string(), "d".to_string()),
            ("nmod".to_string(), "nmos".to_string()),
            ("qmod".to_string(), "npn".to_string()),
        ];
        if models != expected_models {
            return Err(format!(
                "REMOVEUNUSED authored model census changed: expected={expected_models:?}, actual={models:?}"
            ));
        }
        let mut top = netlist
            .elements
            .iter()
            .map(Self::removeunused_element_snapshot)
            .collect::<Result<Vec<_>, _>>()?;
        top.sort_by(|left, right| left.name.cmp(&right.name));
        let mut definitions = netlist
            .subcircuits
            .iter()
            .map(|definition| {
                if definition.ports != ["1".to_string(), "2".to_string()]
                    || !definition.initial_conditions.is_empty()
                    || !definition.node_sets.is_empty()
                    || !definition.params.is_empty()
                    || !definition.expr_params.is_empty()
                    || !definition.string_params.is_empty()
                    || !definition.body_params.is_empty()
                    || !definition.body_expr_params.is_empty()
                    || !definition.body_string_params.is_empty()
                    || !definition.body_functions.is_empty()
                    || !definition.local_options.is_empty()
                    || definition.library_ref.is_some()
                    || !definition.nested_subcircuits.is_empty()
                {
                    return Err(format!(
                        "REMOVEUNUSED authored definition '{}' has unexpected ports/parameters/local state: {definition:?}",
                        definition.name,
                    ));
                }
                let mut elements = definition
                    .elements
                    .iter()
                    .map(Self::removeunused_element_snapshot)
                    .collect::<Result<Vec<_>, _>>()?;
                elements.sort_by(|left, right| left.name.cmp(&right.name));
                Ok((definition.name.to_ascii_lowercase(), elements))
            })
            .collect::<Result<Vec<_>, String>>()?;
        definitions.sort_by(|left, right| left.0.cmp(&right.0));
        let (expected_top, expected_definitions) = kind.expected_authored_snapshots();
        if top != expected_top || definitions != expected_definitions {
            return Err(format!(
                "REMOVEUNUSED authored hierarchy changed: expected top={expected_top:#?}, definitions={expected_definitions:#?}; actual top={top:#?}, definitions={definitions:#?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_removeunused_flattened_topology(
        netlist: &Netlist,
        kind: XyceRemoveUnusedKind,
        policy_off: bool,
    ) -> Result<(), String> {
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("REMOVEUNUSED flattening failed: {error}"))?;
        let mut snapshot = flattened
            .elements
            .iter()
            .map(Self::removeunused_element_snapshot)
            .collect::<Result<Vec<_>, _>>()?;
        snapshot.sort_by(|left, right| left.name.cmp(&right.name));
        let expected_snapshot = kind.expected_flattened_snapshot(policy_off);
        if snapshot != expected_snapshot {
            return Err(format!(
                "REMOVEUNUSED {} exact flattened snapshot changed: expected={expected_snapshot:#?}, actual={snapshot:#?}",
                if policy_off { "policy-off" } else { "active" }
            ));
        }
        let expected_total = if policy_off {
            30
        } else {
            kind.expected_flattened_element_count()
        };
        if flattened.elements.len() != expected_total {
            return Err(format!(
                "REMOVEUNUSED {} topology contains {} flattened elements instead of {expected_total}",
                if policy_off { "policy-off" } else { "active" },
                flattened.elements.len()
            ));
        }
        let mut counts = BTreeMap::<char, usize>::new();
        let mut redundant_selected = Vec::new();
        for element in &flattened.elements {
            let device = match &element.kind {
                ElementKind::Capacitor { .. } => 'C',
                ElementKind::Diode { .. } => 'D',
                ElementKind::CurrentSource(_) => 'I',
                ElementKind::Inductor { .. } | ElementKind::JilesAthertonInductor { .. } => 'L',
                ElementKind::Mosfet { .. } => 'M',
                ElementKind::Bjt { .. } => 'Q',
                ElementKind::Resistor { .. } => 'R',
                ElementKind::VoltageSource(_) => 'V',
                _ => {
                    return Err(format!(
                        "REMOVEUNUSED bounded topology contains unsupported element {} {:?}",
                        element.name, element.kind
                    ));
                }
            };
            *counts.entry(device).or_default() += 1;
            let redundant = if matches!(device, 'M' | 'Q') {
                element.nodes.len() >= 3
                    && element.nodes[0].eq_ignore_ascii_case(&element.nodes[1])
                    && element.nodes[1].eq_ignore_ascii_case(&element.nodes[2])
            } else {
                element.nodes.len() >= 2 && element.nodes[0].eq_ignore_ascii_case(&element.nodes[1])
            };
            if redundant {
                redundant_selected.push(element.name.clone());
            }
        }
        let expected_counts = if policy_off {
            BTreeMap::from([
                ('C', 5),
                ('D', 3),
                ('I', 3),
                ('L', 3),
                ('M', 3),
                ('Q', 3),
                ('R', 6),
                ('V', 4),
            ])
        } else if kind.replace_ground() {
            BTreeMap::from([('C', 2), ('R', 3), ('V', 1)])
        } else {
            BTreeMap::from([('C', 5), ('M', 3), ('R', 3), ('V', 1)])
        };
        if counts != expected_counts {
            return Err(format!(
                "REMOVEUNUSED {} device census changed: expected={expected_counts:?}, actual={counts:?}",
                if policy_off { "policy-off" } else { "active" }
            ));
        }
        if policy_off {
            let expected_redundant = if kind.replace_ground() { 24 } else { 18 };
            if redundant_selected.len() != expected_redundant {
                return Err(format!(
                    "REMOVEUNUSED policy-off topology exposes {} redundant selected devices instead of {expected_redundant}: {redundant_selected:?}",
                    redundant_selected.len(),
                ));
            }
        } else if !redundant_selected.is_empty() {
            return Err(format!(
                "REMOVEUNUSED active topology retained redundant selected devices: {redundant_selected:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_removeunused_analytic_table(
        table: &XycePrnTable,
        results: &[DcSweepPointResult],
        kind: XyceRemoveUnusedKind,
    ) -> Result<(), String> {
        if table.columns != ["Index", "V(1)", "V(2)"] || table.rows.len() != 21 {
            return Err(format!(
                "REMOVEUNUSED table requires Index/V(1)/V(2) and 21 rows, got {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        if results.len() != table.rows.len() {
            return Err(format!(
                "REMOVEUNUSED execution returned {} sweep points for {} table rows",
                results.len(),
                table.rows.len()
            ));
        }
        let ratio = kind.expected_divider_ratio();
        for (row_index, (row, point)) in table.rows.iter().zip(results).enumerate() {
            if row.len() != 3 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "REMOVEUNUSED row {row_index} is not a finite three-column row: {row:?}"
                ));
            }
            let sweep = point.sweep_value;
            let v1_error = (row[1] - sweep).abs();
            let v2_error = (row[2] - ratio * sweep).abs();
            if v1_error > 1.0e-10 || v2_error > 1.0e-9 {
                return Err(format!(
                    "REMOVEUNUSED analytic invariant failed at row {row_index}: sweep={sweep}, V1={}, V2={}, expected ratio={ratio}",
                    row[1], row[2]
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_startup_diagnostic_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceStartupOracleKind,
        start: Instant,
    ) -> Result<(), String> {
        self.validate_startup_oracle_provenance(deck, kind)?;
        let source_bytes = fs::read(&deck.path).map_err(|error| {
            format!(
                "failed to read startup-diagnostic record {}: {error}",
                deck.path.display()
            )
        })?;
        Self::validate_startup_source_identity(kind, &source_bytes)?;
        let source = std::str::from_utf8(&source_bytes).map_err(|error| {
            format!(
                "startup-diagnostic record '{}' is not UTF-8: {error}",
                kind.record()
            )
        })?;

        if kind == XyceStartupOracleKind::IcNodeSetConflict {
            let policy = kind.conflict_error_policy().ok_or_else(|| {
                "IC/NODESET conflict is missing its removed-wrapper error policy".to_string()
            })?;
            if !policy.requires_nonzero_exit
                || policy.search_streams
                    != XyceUpstreamErrorSearchStreams::EitherCompleteStdoutOrStderr
                || policy.ordered_patterns != ["Cannot set both .IC and .NODESET simultaneously"]
            {
                return Err(
                    "IC/NODESET conflict has an incomplete removed-wrapper error policy"
                        .to_string(),
                );
            }
            let observation = Self::observe_startup_conflict(source, &deck.path)?;
            let expected = XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::ConflictingStartupDirectives,
                identifiers: vec![
                    format!(".IC|{}:16", kind.record()),
                    format!(".NODESET|{}:17", kind.record()),
                ],
            };
            if observation != expected {
                return Err(format!(
                    "IC/NODESET conflict produced the wrong typed expected-failure observation: expected {expected:?}, got {observation:?}"
                ));
            }
            return Ok(());
        }

        let expectation = kind
            .warning_expectation()
            .ok_or_else(|| "startup warning record has no warning expectation".to_string())?;
        if expectation.ordered_upstream_patterns.is_empty()
            || expectation
                .ordered_upstream_patterns
                .iter()
                .any(|pattern| pattern.is_empty())
        {
            return Err(format!(
                "startup warning record '{}' has an incomplete removed-wrapper success-warning policy",
                kind.record()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, &deck.path).map_err(|error| {
            format!(
                "startup warning record '{}' did not parse successfully: {error}",
                kind.record()
            )
        })?;
        Self::validate_startup_warning_observation(&netlist, &deck.path, kind, expectation)?;

        // The removed wrappers required a zero simulator exit, not merely a
        // successful parser return. Execute the complete admitted transient
        // and require a finite, structurally valid result grid.
        let tran = Self::single_tran_analysis(&netlist)?;
        let max_step = Self::transient_family_max_step(&netlist, &tran)?;
        let initial_step = Self::xyce_initial_timestep_for_tran(&tran);
        let engine = self.create_xyce_static_tran_engine(None, initial_step);
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let result = engine
            .run_tran_with_abort(&netlist, tran.stop, max_step, &abort)
            .map_err(|error| {
                format!(
                    "startup warning record '{}' failed native transient execution: {error}",
                    kind.record()
                )
            })?;
        Self::validate_transient_result_time_grid(&result).map_err(|error| {
            format!(
                "startup warning record '{}' produced an invalid transient result: {error}",
                kind.record()
            )
        })
    }

    pub(super) fn validate_startup_warning_observation(
        netlist: &Netlist,
        deck_path: &Path,
        kind: XyceStartupOracleKind,
        expected: XyceStartupWarningExpectation,
    ) -> Result<(), String> {
        let diagnostics = netlist.startup_diagnostics();
        if diagnostics.len() != 1 {
            return Err(format!(
                "startup warning record '{}' produced {} typed startup diagnostics, expected exactly one: {diagnostics:?}",
                kind.record(),
                diagnostics.len()
            ));
        }
        let actual = &diagnostics[0];
        if actual.code != expected.code
            || actual.stage != expected.stage
            || actual.kind != expected.directive
            || actual.canonical_nodes
                != expected
                    .canonical_nodes
                    .iter()
                    .map(|node| (*node).to_string())
                    .collect::<Vec<_>>()
        {
            return Err(format!(
                "startup warning record '{}' produced the wrong typed observation: expected {:?}/{:?}/{:?}/{:?}, got {actual:?}",
                kind.record(),
                expected.directive,
                expected.code,
                expected.stage,
                expected.canonical_nodes
            ));
        }
        if actual.origins.len() != 1 || actual.origins[0].line != expected.line {
            return Err(format!(
                "startup warning record '{}' produced the wrong physical origin: expected line {}, got {:?}",
                kind.record(),
                expected.line,
                actual.origins
            ));
        }
        let origin_path = actual.origins[0].path.as_ref().ok_or_else(|| {
            format!(
                "startup warning record '{}' lost its physical source path",
                kind.record()
            )
        })?;
        if origin_path.canonicalize().ok() != deck_path.canonicalize().ok() {
            return Err(format!(
                "startup warning record '{}' diagnostic origin resolved to {}, not {}",
                kind.record(),
                origin_path.display(),
                deck_path.display()
            ));
        }

        match kind {
            XyceStartupOracleKind::Bug667ScopedGlobalWarning => {
                if actual.scopes.len() != 1 {
                    return Err(format!(
                        "BUG667 startup warning must retain exactly one scoped origin, got {:?}",
                        actual.scopes
                    ));
                }
                match &actual.scopes[0] {
                    StartupDirectiveScope::Subcircuit {
                        qualified_definition,
                        qualified_instances,
                    } if qualified_definition.eq_ignore_ascii_case("IC_SUBCKT")
                        && qualified_instances.is_empty() => {}
                    scope => {
                        return Err(format!(
                            "BUG667 startup warning produced the wrong exact subcircuit scope: {scope:?}"
                        ));
                    }
                }
                let subcircuit = netlist
                    .subcircuits
                    .iter()
                    .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case("IC_SUBCKT"))
                    .ok_or_else(|| "BUG667 IC_SUBCKT definition is missing".to_string())?;
                if subcircuit.initial_conditions.len() != 1
                    || !subcircuit.initial_conditions[0]
                        .node
                        .eq_ignore_ascii_case("mid")
                {
                    return Err(format!(
                        "BUG667 valid sibling .IC card did not survive whole-card global rejection: {:?}",
                        subcircuit.initial_conditions
                    ));
                }
            }
            _ => {
                if actual.scopes.as_slice() != [StartupDirectiveScope::TopLevel] {
                    return Err(format!(
                        "startup warning record '{}' must retain exact top-level scope, got {:?}",
                        kind.record(),
                        actual.scopes
                    ));
                }
            }
        }

        // Startup projection owns one public warning per typed observation.
        // Reject any other parser warning so a newly introduced diagnostic can
        // never be silently admitted by this contract.
        if netlist.diagnostics.len() != 1
            || netlist.diagnostics[0].line != expected.line
            || netlist.diagnostics[0].code != expected.code.as_str()
        {
            return Err(format!(
                "startup warning record '{}' emitted unexpected public parse warnings: {:?}",
                kind.record(),
                netlist.diagnostics
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug702_family_census(
        &self,
        family_dir: &Path,
        record: &str,
    ) -> Result<(), String> {
        let mut physical_names = BTreeSet::new();
        for entry in fs::read_dir(family_dir).map_err(|error| {
            format!(
                "failed to inspect BUG702 family {}: {error}",
                family_dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to inspect BUG702 family entry in {}: {error}",
                    family_dir.display()
                )
            })?;
            let path = entry.path();
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "failed to inspect BUG702 family member {}: {error}",
                    path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "BUG702 family member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("BUG702 family member {} is not UTF-8", path.display()))?
                .to_ascii_lowercase();
            if !physical_names.insert(name.clone()) {
                return Err(format!(
                    "BUG702 family contains case-colliding .cir name {name:?}"
                ));
            }
        }
        let physical_names = physical_names.into_iter().collect::<Vec<_>>();
        let physical_hash = blake3::hash(physical_names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if physical_names.len() != 8 || physical_hash != XYCE_BUG702_PHYSICAL_CENSUS_BLAKE3 {
            return Err(format!(
                "BUG702 physical .cir census changed: expected 8 / {}, got {} / {}",
                XYCE_BUG702_PHYSICAL_CENSUS_BLAKE3,
                physical_names.len(),
                physical_hash
            ));
        }

        let manifest_records =
            self.expected_failure_manifest_family_records("netlists/certification_tests/bug_702/")?;
        let manifest_hash = blake3::hash(manifest_records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if manifest_records.len() != 8 || manifest_hash != XYCE_BUG702_MANIFEST_CENSUS_BLAKE3 {
            return Err(format!(
                "BUG702 manifest census changed: expected 8 / {}, got {} / {}",
                XYCE_BUG702_MANIFEST_CENSUS_BLAKE3,
                manifest_records.len(),
                manifest_hash
            ));
        }
        let manifest_names = manifest_records
            .iter()
            .map(|record| {
                record
                    .rsplit_once('/')
                    .map(|(_, name)| name.to_ascii_lowercase())
                    .ok_or_else(|| format!("BUG702 manifest record {record:?} has no filename"))
            })
            .collect::<Result<BTreeSet<_>, _>>()?
            .into_iter()
            .collect::<Vec<_>>();
        if manifest_names != physical_names {
            return Err(format!(
                "BUG702 manifest/physical census is not a bijection: physical={physical_names:?}, manifest={manifest_names:?}"
            ));
        }
        if !manifest_records.iter().any(|candidate| candidate == record) {
            return Err(format!(
                "BUG702 positive record {record:?} is absent from the pinned manifest census"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug702_positive_typed_semantics(
        kind: XyceBug702PositiveKind,
        deck_path: &Path,
        family_dir: &Path,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let directive = netlist
            .device_initial_conditions
            .as_ref()
            .ok_or_else(|| "BUG702 positive deck lost its typed .INITCOND directive".to_string())?;
        let expected_line = match kind {
            XyceBug702PositiveKind::External | XyceBug702PositiveKind::InlinedSingle => 20,
            XyceBug702PositiveKind::InlinedMultiple => 17,
            XyceBug702PositiveKind::Precedence => 21,
        };
        let directive_path = directive
            .origin
            .path
            .as_deref()
            .ok_or_else(|| "BUG702 .INITCOND directive has no source path".to_string())?;
        if directive.origin.line != expected_line
            || directive_path.canonicalize().ok() != deck_path.canonicalize().ok()
        {
            return Err(format!(
                "BUG702 {:?} directive origin changed: {:?}",
                kind, directive.origin
            ));
        }

        let expected_entries: Vec<(&str, Vec<Value>)> = match kind {
            XyceBug702PositiveKind::InlinedMultiple => {
                vec![("C1", vec![400.0]), ("XNLR1:CABS", vec![0.0])]
            }
            XyceBug702PositiveKind::External | XyceBug702PositiveKind::InlinedSingle => {
                vec![("XiNv1:mn1", vec![2.0, 0.0])]
            }
            XyceBug702PositiveKind::Precedence => {
                vec![("XINV1:MN1", vec![2.0, 0.0])]
            }
        };
        if directive.entries.len() != expected_entries.len() {
            return Err(format!(
                "BUG702 {:?} typed entry count changed: expected {}, got {}",
                kind,
                expected_entries.len(),
                directive.entries.len()
            ));
        }
        for (entry, (expected_device, expected_values)) in
            directive.entries.iter().zip(expected_entries)
        {
            if entry.device != expected_device
                || entry.values.len() != expected_values.len()
                || entry
                    .values
                    .iter()
                    .zip(&expected_values)
                    .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
            {
                return Err(format!(
                    "BUG702 {:?} typed INITCOND entry changed: {:?}",
                    kind, entry
                ));
            }
        }

        match (&kind, &directive.source) {
            (
                XyceBug702PositiveKind::External,
                DeviceInitialConditionSource::File {
                    requested_path,
                    resolved_path: Some(resolved_path),
                    content_identity: Some(content_identity),
                },
            ) => {
                let expected_resource = Self::validate_bug702_resource(
                    family_dir,
                    "initcond.dat",
                    XYCE_BUG702_INITCOND_DATA_BYTES,
                    XYCE_BUG702_INITCOND_DATA_BLAKE3,
                )?;
                if requested_path != "initcond.dat"
                    || resolved_path.canonicalize().ok() != Some(expected_resource)
                    || content_identity != XYCE_BUG702_INITCOND_DATA_BLAKE3
                {
                    return Err(format!(
                        "BUG702 external source provenance changed: {:?}",
                        directive.source
                    ));
                }
            }
            (
                XyceBug702PositiveKind::InlinedMultiple
                | XyceBug702PositiveKind::InlinedSingle
                | XyceBug702PositiveKind::Precedence,
                DeviceInitialConditionSource::Inline,
            ) => {}
            _ => {
                return Err(format!(
                    "BUG702 {:?} INITCOND representation changed: {:?}",
                    kind, directive.source
                ));
            }
        }

        let flattened = rspice_core::netlist::flatten_netlist(netlist)
            .map_err(|error| format!("BUG702 {:?} flattening failed: {error}", kind))?;
        if kind == XyceBug702PositiveKind::InlinedMultiple {
            for (device, expected) in [("C1", 400.0_f64), ("XNLR1.CABS", 0.0_f64)] {
                let element = flattened
                    .iter()
                    .find(|element| element.name.eq_ignore_ascii_case(device))
                    .ok_or_else(|| format!("BUG702 flattened target {device} is missing"))?;
                let ElementKind::Capacitor {
                    initial_voltage: Some(actual),
                    ..
                } = element.kind
                else {
                    return Err(format!(
                        "BUG702 flattened target {device} lost its capacitor IC"
                    ));
                };
                if actual.to_bits() != expected.to_bits() {
                    return Err(format!(
                        "BUG702 flattened target {device} IC changed: expected {expected}, got {actual}"
                    ));
                }
            }
            let inductor = flattened
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("L1"))
                .ok_or_else(|| "BUG702 flattened L1 is missing".to_string())?;
            if !matches!(
                inductor.kind,
                ElementKind::Inductor {
                    initial_current: Some(value),
                    ..
                } if value.to_bits() == 10.0f64.to_bits()
            ) {
                return Err("BUG702 L1 lost its independent authored IC=10A".to_string());
            }
        } else {
            let mos = flattened
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("XINV1.MN1"))
                .ok_or_else(|| "BUG702 flattened XINV1.MN1 is missing".to_string())?;
            let ElementKind::Mosfet {
                instance_params, ..
            } = &mos.kind
            else {
                return Err("BUG702 XINV1.MN1 is not a MOSFET".to_string());
            };
            let ic = instance_params
                .iter()
                .filter(|(name, _)| name.starts_with("IC_"))
                .map(|(name, value)| (name.as_str(), value.to_bits()))
                .collect::<Vec<_>>();
            if ic != [("IC_VDS", 2.0f64.to_bits()), ("IC_VGS", 0.0f64.to_bits())] {
                return Err(format!(
                    "BUG702 {:?} effective MOS IC vector changed: {ic:?}",
                    kind
                ));
            }
        }
        Ok(())
    }

    pub(super) fn bug702_positive_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug702PositiveContract, String>> {
        let kind = XyceBug702PositiveKind::for_record(&deck.relative_path)?;
        Some((|| {
            if deck.section != XyceDeckSection::Netlists {
                return Err("BUG702 positive record is not in the Netlists corpus".to_string());
            }
            let record = Self::normalize_manifest_key(&deck.relative_path);
            if record != kind.record() {
                return Err(format!(
                    "BUG702 positive record path changed: expected {}, got {record}",
                    kind.record()
                ));
            }
            if !self.requires_upstream_wrapper(&deck.relative_path) {
                return Err("BUG702 positive record lost wrapper provenance".to_string());
            }
            let expected_path = self.root.join(Path::new(&deck.relative_path));
            if deck.path.canonicalize().ok() != expected_path.canonicalize().ok() {
                return Err(format!(
                    "BUG702 positive record resolved outside its canonical corpus path: {}",
                    deck.path.display()
                ));
            }
            let metadata = fs::symlink_metadata(&deck.path).map_err(|error| {
                format!(
                    "failed to inspect BUG702 positive record {}: {error}",
                    deck.path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err("BUG702 positive record must be a regular non-symlink file".to_string());
            }
            let family_dir = deck
                .path
                .parent()
                .ok_or_else(|| "BUG702 positive record has no family directory".to_string())?;
            self.validate_bug702_family_census(family_dir, kind.record())?;
            self.validate_bug702_complete_family_provenance(family_dir)?;

            let source_bytes = fs::read(&deck.path)
                .map_err(|error| format!("failed to read BUG702 positive deck: {error}"))?;
            let source_hash = blake3::hash(&source_bytes).to_hex().to_string();
            if source_hash != kind.source_blake3() {
                return Err(format!(
                    "BUG702 {:?} source digest changed: expected {}, got {source_hash}",
                    kind,
                    kind.source_blake3()
                ));
            }
            let source = std::str::from_utf8(&source_bytes)
                .map_err(|error| format!("BUG702 positive deck is not UTF-8: {error}"))?;
            let effective_source = Self::bug702_effective_canonical_source(kind, source)?;
            let canonical_path = self.root.join(kind.canonical_source_record());
            let canonical_bytes = fs::read(&canonical_path).map_err(|error| {
                format!(
                    "failed to read BUG702 canonical source {}: {error}",
                    canonical_path.display()
                )
            })?;
            let expected_canonical_hash = if kind == XyceBug702PositiveKind::InlinedMultiple {
                XYCE_BUG702_CANONICAL_NLRCS10_SOURCE_BLAKE3
            } else {
                XYCE_BUG702_CANONICAL_INV1XIC_SOURCE_BLAKE3
            };
            let canonical_hash = blake3::hash(&canonical_bytes).to_hex().to_string();
            if canonical_hash != expected_canonical_hash
                || effective_source.as_bytes() != canonical_bytes
            {
                return Err(format!(
                    "BUG702 {:?} effective source is not byte-equivalent to canonical {}",
                    kind,
                    canonical_path.display()
                ));
            }

            let netlist = Self::parse_netlist_with_expression_dialect_and_execution_dir(
                source,
                &deck.path,
                ExpressionDialect::Xyce,
                Some(family_dir),
            )
            .map_err(|error| format!("BUG702 {:?} parse failed: {error}", kind))?;
            Self::validate_bug702_positive_typed_semantics(kind, &deck.path, family_dir, &netlist)?;

            let mut plan = self.static_tran_plan_for_path_with_purpose(
                &canonical_path,
                XyceStaticTranPlanPurpose::AbsoluteOracle,
            )?;
            let alias_path = self
                .root
                .join("OutputData/Certification_Tests/BUG_702")
                .join(kind.alias_reference_name());
            let alias_bytes = fs::read(&alias_path).map_err(|error| {
                format!(
                    "failed to read BUG702 alias reference {}: {error}",
                    alias_path.display()
                )
            })?;
            let (expected_bytes, expected_hash) = kind.alias_reference_identity();
            let alias_hash = blake3::hash(&alias_bytes).to_hex().to_string();
            if alias_bytes.len() != expected_bytes || alias_hash != expected_hash {
                return Err(format!(
                    "BUG702 {:?} alias reference changed: expected {expected_bytes} / {expected_hash}, got {} / {alias_hash}",
                    kind,
                    alias_bytes.len()
                ));
            }
            let canonical_reference = fs::read(&plan.reference_path).map_err(|error| {
                format!(
                    "failed to read BUG702 canonical reference {}: {error}",
                    plan.reference_path.display()
                )
            })?;
            if alias_bytes != canonical_reference {
                return Err(format!(
                    "BUG702 {:?} alias reference is not byte-identical to canonical {}",
                    kind,
                    plan.reference_path.display()
                ));
            }
            plan.reference_path = alias_path;
            plan.comparison_mode = XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision: kind.scientific_precision(),
            };
            Ok(XyceBug702PositiveContract {
                kind,
                netlist,
                plan,
            })
        })())
    }

    pub(super) fn validate_missing_subcircuit_ends_dependency(
        deck_path: &Path,
    ) -> Result<PathBuf, String> {
        let dependency = deck_path
            .parent()
            .ok_or_else(|| "missing-.ENDS include deck has no parent directory".to_string())?
            .join("missing.ends");
        let metadata = fs::symlink_metadata(&dependency).map_err(|error| {
            format!(
                "failed to inspect missing-.ENDS include dependency {}: {error}",
                dependency.display()
            )
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(format!(
                "missing-.ENDS include dependency {} must be a regular non-symlink file",
                dependency.display()
            ));
        }
        let bytes = fs::read(&dependency).map_err(|error| {
            format!(
                "failed to read missing-.ENDS include dependency {}: {error}",
                dependency.display()
            )
        })?;
        let digest = blake3::hash(&bytes).to_hex().to_string();
        if bytes.len() != XYCE_SUBCKT_MISSING_ENDS_INCLUDE_FILE_BYTES
            || digest != XYCE_SUBCKT_MISSING_ENDS_INCLUDE_FILE_BLAKE3
        {
            return Err(format!(
                "missing-.ENDS include dependency changed: expected {} bytes / {}, got {} bytes / {}",
                XYCE_SUBCKT_MISSING_ENDS_INCLUDE_FILE_BYTES,
                XYCE_SUBCKT_MISSING_ENDS_INCLUDE_FILE_BLAKE3,
                bytes.len(),
                digest
            ));
        }
        let content = std::str::from_utf8(&bytes).map_err(|error| {
            format!(
                "missing-.ENDS include dependency {} is not UTF-8: {error}",
                dependency.display()
            )
        })?;
        Self::require_expected_failure_source_lines(
            "Message/Subcircuit missing.ends dependency",
            content,
            3,
            &[
                (1, ".subckt testsub a b c"),
                (2, "r1 a b 1"),
                (3, "r2 b c 1"),
            ],
        )?;
        dependency.canonicalize().map_err(|error| {
            format!(
                "failed to canonicalize missing-.ENDS include dependency {}: {error}",
                dependency.display()
            )
        })
    }

    pub(super) fn validate_bug671_binary_fixture(path: &Path) -> Result<(), String> {
        let metadata = fs::symlink_metadata(path).map_err(|err| {
            format!(
                "failed to inspect BUG 671 binary fixture {}: {err}",
                path.display()
            )
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(format!(
                "BUG 671 binary fixture {} must be a regular non-symlink file",
                path.display()
            ));
        }
        let bytes = fs::read(path).map_err(|err| {
            format!(
                "failed to read BUG 671 binary fixture {}: {err}",
                path.display()
            )
        })?;
        if bytes.len() != XYCE_BUG671_FIXTURE_BYTES {
            return Err(format!(
                "BUG 671 binary fixture size changed: expected {}, got {}",
                XYCE_BUG671_FIXTURE_BYTES,
                bytes.len()
            ));
        }
        if bytes.get(..XYCE_BUG671_OLE_MAGIC.len()) != Some(&XYCE_BUG671_OLE_MAGIC) {
            return Err("BUG 671 binary fixture lost its OLE compound-file magic".to_string());
        }
        let digest = blake3::hash(&bytes).to_hex().to_string();
        if digest != XYCE_BUG671_FIXTURE_BLAKE3 {
            return Err(format!(
                "BUG 671 binary fixture digest changed: expected {}, got {}",
                XYCE_BUG671_FIXTURE_BLAKE3, digest
            ));
        }
        Ok(())
    }

    /// Classify HB print probes structurally instead of using substring
    /// matching.  The old `contains("i(")` check misclassified voltage
    /// accessors such as `VI(1)` and `VDB(1)` as branch-current probes.  This
    /// scanner preserves support for voltage-only expressions while rejecting
    /// actual branch, lead-current, power, and device-parameter probes.
    pub(super) fn static_hb_probe_is_unsupported(probe: &str) -> bool {
        let expression = Self::print_expression_inner(probe).unwrap_or(probe);
        let normalized = Self::normalize_probe(expression);
        if normalized.contains('@') || Self::parse_device_parameter_probe(&normalized).is_some() {
            return true;
        }
        if Self::parse_voltage_probe(&normalized).is_some() {
            return false;
        }
        if Self::parse_current_probe(&normalized).is_some()
            || Self::parse_ac_current_probe(&normalized).is_some()
            || Self::parse_lead_current_probe(&normalized).is_some()
            || Self::parse_power_probe(&normalized).is_some()
            || Self::parse_device_operating_point_probe(&normalized).is_some()
        {
            return true;
        }

        let mut index = 0usize;
        while index < expression.len() {
            if let Some(open_index) = Self::print_probe_call_open_index(expression, index) {
                let Ok(close_index) = Self::matching_parenthesis_index(expression, open_index)
                else {
                    return true;
                };
                let call = &expression[index..=close_index];
                let normalized_call = Self::normalize_probe(call);
                if Self::parse_current_probe(&normalized_call).is_some()
                    || Self::parse_ac_current_probe(&normalized_call).is_some()
                    || Self::parse_lead_current_probe(&normalized_call).is_some()
                    || Self::parse_power_probe(&normalized_call).is_some()
                    || Self::parse_device_operating_point_probe(&normalized_call).is_some()
                {
                    return true;
                }
                index = close_index + 1;
                continue;
            }
            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid character boundary");
            index += ch.len_utf8();
        }
        false
    }

    pub(super) fn validate_native_static_prn_wrapper_contract(
        &self,
        contract: XyceStaticDcContract,
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        if matches!(contract, XyceStaticDcContract::WrapperResistorDefault) {
            Self::validate_resistor_default_wrapper_diagnostics(plan)?;
        }

        Ok(())
    }

    pub(super) fn validate_resistor_default_wrapper_diagnostics(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        let default_warning_count = plan
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic
                    .code
                    .eq_ignore_ascii_case("xyce_resistor_missing_value")
                    || diagnostic
                        .code
                        .eq_ignore_ascii_case("xyce_resistor_model_missing_value")
            })
            .count();
        if default_warning_count == 0 {
            return Err(
                "wrapper-origin resistor default contract requires native value-default diagnostics"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn native_static_prn_wrapper_contract(
        relative_path: &str,
        deck_path: &Path,
        source: &str,
    ) -> Result<XyceStaticDcContract, String> {
        if Self::is_native_gnuplot_splot_wrapper_candidate(source) {
            return Ok(XyceStaticDcContract::WrapperGnuplotSplot);
        }

        if Self::is_native_csv_dc_wrapper_candidate(relative_path, source) {
            Self::validate_csv_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperCsv);
        }

        if Self::is_native_csd_dc_wrapper_candidate(relative_path, source) {
            Self::validate_csd_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperCsd);
        }

        if Self::is_native_file_only_prn_wrapper_candidate(relative_path, source) {
            Self::validate_file_only_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperFilePrn);
        }

        if Self::is_native_raw_wrapper_candidate_path(relative_path) {
            Self::validate_raw_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperRaw);
        }

        if Self::is_native_no_output_dc_wrapper_candidate(relative_path, source) {
            Self::validate_no_output_dc_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperNoOutput);
        }

        if Self::is_native_default_prn_wrapper_candidate_path(relative_path)
            || Self::is_native_multiplicity_static_prn_wrapper_candidate_path(relative_path)
        {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_hspice_math_wrapper_candidate(relative_path, source) {
            Self::validate_hspice_math_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperHspiceMath);
        }

        if Self::is_native_hspice_random_wrapper_candidate(relative_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_resistor_default_wrapper_candidate(relative_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperResistorDefault);
        }

        if Self::is_native_resistor_temperature_step_wrapper_candidate(relative_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_semiconductor_resistor_step_wrapper_candidate(relative_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_top_level_execution_dir_wrapper_candidate(deck_path, source) {
            return Ok(XyceStaticDcContract::WrapperTopLevelExecutionDir);
        }

        if Self::is_native_absolute_inc_lib_wrapper_candidate(deck_path, source) {
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_step_data_wrapper_candidate(source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_dc_data_table_wrapper_candidate(source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_empty_wildcard_lead_current_wrapper_candidate(deck_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_subcircuit_node_probe_wrapper_candidate(source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_voltage_accessor_wrapper_candidate(source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperVoltageAccessor);
        }

        if Self::is_native_plain_static_dc_prn_wrapper_candidate(deck_path, source) {
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        Err(Self::upstream_wrapper_required_reason().to_string())
    }

    pub(super) fn validate_default_prn_print_tokens(
        tokens: &[&str],
        allow_wrapper_probe_primary_prn: bool,
    ) -> Result<bool, String> {
        let Some(analysis) = tokens.get(1) else {
            return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
        };
        if !analysis.eq_ignore_ascii_case("DC") {
            return Err(format!(
                "wrapper-origin default .prn contract only covers .PRINT DC, got .PRINT {analysis}"
            ));
        }

        let mut index = 2usize;
        let mut has_file_output = false;
        while index < tokens.len() {
            if let Some((raw_key, raw_value, consumed)) =
                Self::print_option_assignment(tokens, index)
            {
                let key = raw_key.trim().to_ascii_lowercase();
                let value = raw_value.trim().trim_matches(['"', '\'']);
                match key.as_str() {
                    "file" => {
                        has_file_output = true;
                    }
                    "format" if Self::dc_print_format_is_prn_compatible(value) => {}
                    "format"
                        if allow_wrapper_probe_primary_prn
                            && value.eq_ignore_ascii_case("PROBE") => {}
                    "format" => {
                        return Err(format!(
                            "wrapper-origin default .prn contract does not cover FORMAT={value}"
                        ));
                    }
                    _ => {}
                }
                index += consumed;
                continue;
            }
            index += 1;
        }

        Ok(has_file_output)
    }

    pub(super) fn validate_native_output_override_prn_wrapper_contract(
        source: &str,
        expected_analysis: &str,
    ) -> Result<(), String> {
        if !matches!(expected_analysis, "TRAN" | "NOISE") {
            return Err(format!(
                "unsupported output override analysis '{expected_analysis}'"
            ));
        }
        let mut print_count = 0usize;
        let mut probe_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                let tokens = Self::split_print_fields(&trimmed)?;
                let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
                let Some(analysis) = token_refs.get(1).copied() else {
                    return Err("output override .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case(expected_analysis) {
                    return Err(format!(
                        "wrapper-origin {expected_analysis} output override contract does not cover .PRINT {analysis}"
                    ));
                }

                print_count += 1;
                let mut index = 2usize;
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        if raw_key.trim().eq_ignore_ascii_case("FORMAT") {
                            match expected_analysis {
                                "TRAN" => {
                                    Self::static_tran_contract_for_print_format(true, Some(value))
                                        .map_err(|err| {
                                            format!(
                                                "wrapper-origin TRAN output override contract does not cover {err}"
                                            )
                                        })?;
                                }
                                "NOISE" => {
                                    XyceStaticNoiseContract::for_format(Some(value)).map_err(
                                        |err| {
                                            format!(
                                                "wrapper-origin NOISE output override contract does not cover {err}"
                                            )
                                        },
                                    )?;
                                }
                                _ => unreachable!("analysis was validated above"),
                            }
                        }
                        index += consumed;
                        continue;
                    }
                    let normalized = token_refs[index].to_ascii_lowercase();
                    if !Self::is_print_option_token(&normalized) {
                        probe_count += 1;
                    }
                    index += 1;
                }
                continue;
            }
            if expected_analysis == "TRAN"
                && Self::is_extra_wrapper_tran_output_analysis_command(command)
            {
                return Err(format!(
                    "wrapper-origin TRAN output override contract does not cover {command} directives"
                ));
            }
        }

        if print_count == 0 || probe_count == 0 {
            return Err(format!(
                "wrapper-origin {expected_analysis} output override contract requires .PRINT {expected_analysis} probes"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_analytic_rc_initial_sample(
        actual: &XycePrnTable,
        specification: &XyceAnalyticRcSpecification,
    ) -> Result<(), String> {
        const LABEL: &str = "analytic first-order RC";
        let first = actual
            .rows
            .first()
            .ok_or_else(|| format!("{LABEL} simulator output contains no initial sample"))?;
        if actual.columns.len() != 3 || first.len() != actual.columns.len() {
            return Err(format!(
                "{LABEL} initial-sample validation requires one Index/TIME/probe row"
            ));
        }
        if first[0].to_bits() != 0.0f64.to_bits() {
            return Err(format!(
                "{LABEL} first sample has noncanonical index {}",
                first[0]
            ));
        }
        let printed_time = Self::xyce_default_prn_roundtrip(first[1])?;
        let printed_value = Self::xyce_default_prn_roundtrip(first[2])?;
        let printed_initial = Self::xyce_default_prn_roundtrip(specification.initial_voltage)?;
        let normalized_error =
            Self::xyce_verify_normalized_error(printed_initial, printed_value).abs();
        if printed_time != 0.0 || !normalized_error.is_finite() || normalized_error > 1.0 {
            return Err(format!(
                "{LABEL} requires the first serialized sample to be TIME=0 and V(output) within the default xyce_verify bound around {printed_initial}; got TIME={printed_time}, V(output)={printed_value}, normalized error={normalized_error}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_analytic_rc_complete_time_domain(
        actual: &XycePrnTable,
        stop_time: Value,
    ) -> Result<(), String> {
        const LABEL: &str = "analytic first-order RC";
        if !stop_time.is_finite() || stop_time <= 0.0 {
            return Err(format!("{LABEL} has invalid planned stop time {stop_time}"));
        }
        if actual.columns.len() != 3 || actual.rows.len() < 2 {
            return Err(format!(
                "{LABEL} output domain requires at least two Index/TIME/probe rows"
            ));
        }
        let first = actual.rows.first().expect("row count was checked");
        let last = actual.rows.last().expect("row count was checked");
        if first.len() != actual.columns.len() || last.len() != actual.columns.len() {
            return Err(format!(
                "{LABEL} output domain boundary rows do not match the output columns"
            ));
        }
        let printed_first = Self::xyce_default_prn_roundtrip(first[1])?;
        let printed_last = Self::xyce_default_prn_roundtrip(last[1])?;
        let printed_stop = Self::xyce_default_prn_roundtrip(stop_time)?;
        if printed_first != 0.0 || printed_last.to_bits() != printed_stop.to_bits() {
            return Err(format!(
                "{LABEL} output domain must span serialized TIME=0 through .TRAN stop {printed_stop}, got {printed_first} through {printed_last}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug662_default_prn_token_lines(
        lines: &[Vec<String>],
        data_rows: usize,
    ) -> Result<(), String> {
        let expected_header = ["Index", "TIME", "V(N14950)", "V(N15037)"];
        let expected_footer = ["End", "of", "Xyce(TM)", "Simulation"];
        if lines.len() != data_rows + 2
            || lines.first().is_none_or(|line| line != &expected_header)
            || lines.last().is_none_or(|line| line != &expected_footer)
        {
            return Err(format!(
                "BUG 662 serialized PRN must contain the exact header, {data_rows} data rows, and normal completion footer"
            ));
        }
        for (row_index, row) in lines[1..lines.len() - 1].iter().enumerate() {
            if row.len() != expected_header.len() {
                return Err(format!(
                    "BUG 662 serialized PRN data row {row_index} has {} tokens instead of {}",
                    row.len(),
                    expected_header.len()
                ));
            }
            let values = row
                .iter()
                .map(|token| {
                    token.parse::<Value>().map_err(|err| {
                        format!(
                            "BUG 662 serialized PRN data row {row_index} has invalid numeric token '{token}': {err}"
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|value| !value.is_finite())
                || values[0].to_bits() != (row_index as Value).to_bits()
            {
                return Err(format!(
                    "BUG 662 serialized PRN data row {row_index} has non-finite data or a noncanonical Index"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_analytic_sinusoidal_rc_output_domain(
        actual: &XycePrnTable,
        specification: &XyceAnalyticSinusoidalRcSpecification,
        stop_time: Value,
        tolerance: XyceVerifyTransientTolerance,
    ) -> Result<(), String> {
        const LABEL: &str = "analytic sinusoidal first-order RC";
        let tolerance = tolerance.validate()?;
        if !stop_time.is_finite() || stop_time <= 0.0 {
            return Err(format!("{LABEL} has invalid planned stop time {stop_time}"));
        }
        if actual.columns.len() != 3
            || Self::normalize_probe(&actual.columns[2])
                != Self::normalize_probe(&specification.print_expression)
            || actual.rows.len() < 2
        {
            return Err(format!(
                "{LABEL} output domain requires Index/TIME/expression and at least two rows"
            ));
        }
        let first = actual.rows.first().expect("row count was checked");
        let last = actual.rows.last().expect("row count was checked");
        if first.len() != actual.columns.len() || last.len() != actual.columns.len() {
            return Err(format!(
                "{LABEL} output boundary rows do not match the output columns"
            ));
        }
        let printed_first_time = Self::xyce_default_prn_roundtrip(first[1])?;
        let printed_last_time = Self::xyce_default_prn_roundtrip(last[1])?;
        let printed_stop = Self::xyce_default_prn_roundtrip(stop_time)?;
        let printed_first_value = Self::xyce_default_prn_roundtrip(first[2])?;
        let printed_initial = Self::xyce_default_prn_roundtrip(specification.print_offset)?;
        let initial_error = Self::xyce_verify_normalized_error_with_tolerance(
            printed_initial,
            printed_first_value,
            tolerance,
        )
        .abs();
        if first[0].to_bits() != 0.0f64.to_bits()
            || printed_first_time != 0.0
            || printed_last_time.to_bits() != printed_stop.to_bits()
            || !initial_error.is_finite()
            || initial_error > 1.0
        {
            return Err(format!(
                "{LABEL} output must start at Index=0/TIME=0/value within the qualified xyce_verify bound around {printed_initial} and end at .TRAN stop {printed_stop}; got Index={}, TIME/value={printed_first_time}/{printed_first_value}, normalized initial error={initial_error}, final TIME={printed_last_time}",
                first[0]
            ));
        }
        Ok(())
    }

    pub(super) fn validate_analytic_rc_plan(
        plan: &XyceStaticTranPlan,
        source: &XyceAnalyticRcSourceContract,
    ) -> Result<(), String> {
        const LABEL: &str = "analytic first-order RC";
        if plan.contract != XyceStaticTranContract::WrapperStatic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.reference_path.is_file()
        {
            return Err(format!(
                "{LABEL} requires one unstepped default-PRN wrapper output with a generated oracle"
            ));
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step < 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.tran.step.to_bits() != source.tran_step_bits
            || plan.tran.stop.to_bits() != source.tran_stop_bits
        {
            return Err(format!(
                "{LABEL} requires the direct finite '.TRAN step stop' tuple and no START, MAXSTEP, or UIC"
            ));
        }
        let [probe_text] = plan.print.probes.as_slice() else {
            return Err(format!("{LABEL} requires exactly one voltage probe"));
        };
        let probe = Self::parse_voltage_probe(probe_text)
            .ok_or_else(|| format!("{LABEL} probe '{probe_text}' is not a voltage probe"))?;
        if probe.accessor != XyceVoltageAccessor::Value
            || probe.node_neg.is_some()
            || Self::canonical_passive_primary_node_name(&probe.node_pos) != source.probe_node
        {
            return Err(format!(
                "{LABEL} requires the source-qualified single-ended voltage probe"
            ));
        }
        if Self::tran_print_time_scale_factor(&plan.source)?.to_bits() != 1.0f64.to_bits() {
            return Err(format!(
                "{LABEL} generator consumes physical TIME and does not admit output time scaling"
            ));
        }
        if Self::analytic_rc_source_contract(&plan.source)? != *source {
            return Err(format!(
                "{LABEL} plan source changed after source-form qualification"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_analytic_sinusoidal_rc_plan(
        plan: &XyceStaticTranPlan,
        source: &XyceAnalyticSinusoidalRcSourceContract,
    ) -> Result<(), String> {
        const LABEL: &str = "analytic sinusoidal first-order RC";
        if plan.contract != XyceStaticTranContract::WrapperStatic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.reference_path.is_file()
        {
            return Err(format!(
                "{LABEL} requires one unstepped default-PRN wrapper output with a generated oracle"
            ));
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step < 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.tran.step.to_bits() != source.tran_step_bits
            || plan.tran.stop.to_bits() != source.tran_stop_bits
        {
            return Err(format!(
                "{LABEL} requires the direct '.TRAN 0 2e-4' tuple and no START, MAXSTEP, or UIC"
            ));
        }
        let [probe] = plan.print.probes.as_slice() else {
            return Err(format!("{LABEL} requires exactly one print expression"));
        };
        if Self::normalize_probe(probe) != Self::normalize_probe(&source.print_expression) {
            return Err(format!(
                "{LABEL} planned probe differs from direct source provenance"
            ));
        }
        let (probe_node, offset) = Self::analytic_sinusoidal_rc_print_expression(probe)?;
        if probe_node != source.probe_node || offset.to_bits() != source.print_offset_bits {
            return Err(format!(
                "{LABEL} planned print expression changed its node or offset"
            ));
        }
        if Self::tran_print_time_scale_factor(&plan.source)?.to_bits() != 1.0f64.to_bits() {
            return Err(format!(
                "{LABEL} generator consumes physical TIME and does not admit output time scaling"
            ));
        }
        if Self::analytic_sinusoidal_rc_source_contract(&plan.source)? != *source {
            return Err(format!(
                "{LABEL} plan source changed after source-form qualification"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_passive_primary_common_netlist(
        netlist: &Netlist,
        kind: XycePassivePrimaryKind,
    ) -> Result<(), String> {
        if netlist.title.trim().is_empty() {
            return Err("passive primary-value parity requires a nonempty title".to_string());
        }
        if !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err("passive primary-value parity contains auxiliary analysis, hierarchy, external-model, or diagnostic state".to_string());
        }
        if !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(
                "passive primary-value parity does not admit parameters or user functions"
                    .to_string(),
            );
        }
        if netlist.models.len() != 1 {
            return Err(format!(
                "passive primary-value parity requires exactly one model, found {}",
                netlist.models.len()
            ));
        }
        let analysis_matches = matches!(
            (kind, netlist.analyses.as_slice()),
            (
                XycePassivePrimaryKind::CapacitorTran,
                [AnalysisCommand::Tran { .. }]
            ) | (
                XycePassivePrimaryKind::ResistorDc,
                [AnalysisCommand::Dc { .. }]
            )
        );
        if !analysis_matches {
            return Err(format!(
                "passive primary-value {:?} parity requires exactly one matching analysis command",
                kind
            ));
        }
        for element in &netlist.elements {
            if let Some(alias) = element.nodes.iter().find(|node| {
                Self::xyce_ground_alias_name(node)
                    && !Self::passive_primary_name_is_literal_ground(node)
            }) {
                return Err(format!(
                    "element '{}' uses ground alias '{}'; passive primary-value parity requires literal node 0",
                    element.name, alias
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_passive_temperature_model_binding(
        element_model: Option<&str>,
        model: &rspice_core::netlist::ModelDef,
    ) -> Result<(), String> {
        if element_model.is_none_or(|name| !name.eq_ignore_ascii_case(&model.name)) {
            return Err(format!(
                "passive temperature-coefficient override device must bind the unique model '{}'",
                model.name
            ));
        }
        Ok(())
    }

    pub(super) fn validate_delimited_expression_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "delimited-expression parity";
        if !matches!(plan.expression_dialect, ExpressionDialect::Xyce)
            || plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.print.probes.len() != 4
            || plan.dc.sweep2.is_some()
            || plan.dc.primary_spec().points().is_empty()
        {
            return Err(format!(
                "{LABEL} requires one diagnostic-free native linear DC sweep with default indexed PRN output and no extra execution state"
            ));
        }
        let netlist = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .map_err(|err| format!("{LABEL} netlist parse failed: {err}"))?;
        for probe in &plan.print.probes {
            if let Some(expression) = Self::print_expression_inner(probe) {
                Self::validate_dc_probe_expression(expression, &netlist)?;
            } else {
                Self::validate_atomic_dc_probe(&Self::normalize_probe(probe), probe, &netlist)?;
            }
        }
        Self::delimited_expression_source_qualification(&plan.source).map(|_| ())
    }

    pub(super) fn validate_bjt_external_node_print_contract(source: &str) -> Result<(), String> {
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim();
            if !trimmed
                .split_whitespace()
                .next()
                .is_some_and(|command| command.eq_ignore_ascii_case(".print"))
            {
                continue;
            }
            let tokens = Self::split_print_fields(trimmed)?;
            if !tokens
                .get(1)
                .is_some_and(|analysis| analysis.eq_ignore_ascii_case("DC"))
            {
                continue;
            }
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            let mut index = 2usize;
            while index < token_refs.len() {
                if let Some((raw_key, _, _)) = Self::print_option_assignment(&token_refs, index) {
                    return Err(format!(
                        "exact BJT external-node DC requires default primary .prn formatting and does not admit .PRINT assignment {raw_key}"
                    ));
                }
                if token_refs[index].eq_ignore_ascii_case("noindex") {
                    return Err(
                        "exact BJT external-node DC requires the default indexed .prn layout and does not admit .PRINT NOINDEX"
                            .to_string(),
                    );
                }
                index += 1;
            }
        }
        Ok(())
    }

    pub(super) fn validate_solution_dependent_capacitor_expression(
        element_name: &str,
        expression: &str,
        params: &rspice_core::netlist::ParamContext,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison could not prepare capacitor value expression '{expression}' on element '{element_name}': {err}"
            )
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison does not yet support capacitor value expression '{expression}' on element '{element_name}': {err}"
            )
        })?;
        if Self::expression_depends_on_frequency(&ast) {
            return Err(format!(
                "native static .PRINT TRAN comparison does not support frequency-dependent capacitor value expression '{expression}' on element '{element_name}'"
            ));
        }
        if Self::expression_contains_sdt(&ast) {
            return Err(format!(
                "native static .PRINT TRAN comparison does not support stateful SDT in capacitor value expression '{expression}' on element '{element_name}'"
            ));
        }
        if !Self::passive_value_expression_depends_on_runtime_quantity(&ast) {
            return Err(format!(
                "native static .PRINT TRAN comparison requires capacitor value expression '{expression}' on element '{element_name}' to depend on a transient runtime quantity"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_flattened_subcircuit_instances_resolved(
        netlist: &Netlist,
        flattened_elements: &[rspice_core::netlist::Element],
    ) -> Result<(), String> {
        for element in &netlist.elements {
            if !matches!(element.kind, ElementKind::Subcircuit { .. }) {
                continue;
            }
            let prefix = format!("{}.", element.name).to_ascii_lowercase();
            let mut members = flattened_elements
                .iter()
                .filter(|flattened| flattened.name.to_ascii_lowercase().starts_with(&prefix));

            let Some(_member) = members.next() else {
                return Err(format!(
                    "native static .PRINT TRAN comparison could not find flattened members for subcircuit '{}'",
                    element.name
                ));
            };
        }

        Ok(())
    }

    pub(super) fn is_expected_unsupported_runtime_error(err: &SimulationError) -> bool {
        let (SimulationError::Circuit(message) | SimulationError::Netlist(message)) = err else {
            return false;
        };
        let normalized = message.to_ascii_lowercase();
        normalized.contains("unsupported")
            || normalized.contains("not implemented")
            || normalized.contains("does not support")
            || normalized.contains("currently supports")
            || normalized.contains("no native implementation")
            || normalized.contains("no generated verilog-a builtin")
            || normalized.contains("no generated builtin")
            || normalized.contains("not yet")
            || normalized.contains("refusing")
            || normalized.contains("must not run through")
    }

    pub(super) fn netlist_uses_unsupported_ekv3_level301_branch_current_model(
        netlist: &Netlist,
    ) -> bool {
        if Self::elements_use_unsupported_ekv3_level301_branch_current_model(
            &netlist.elements,
            &netlist.models,
            &[],
        ) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_use_unsupported_ekv3_level301_branch_current_model(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
            )
        })
    }

    pub(super) fn elements_use_unsupported_ekv3_level301_branch_current_model(
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
                .is_some_and(|model| {
                    Self::model_is_ekv3_level301(model)
                        && !Self::model_is_ekv3_level301_native_150nm_branch_current(model)
                })
        })
    }

    pub(super) fn validate_print_output_format_compatible(
        existing: Option<&str>,
        incoming: Option<&str>,
        analysis: &str,
        file: Option<&str>,
    ) -> Result<(), String> {
        let existing_key = Self::print_format_key(existing);
        let incoming_key = Self::print_format_key(incoming);
        if existing_key == incoming_key {
            return Ok(());
        }
        let destination = file
            .map(|file| format!("FILE={file}"))
            .unwrap_or_else(|| "primary output".to_string());
        Err(format!(
            "multiple .PRINT {analysis} statements for {destination} use different FORMAT values ({existing_key} and {incoming_key})"
        ))
    }

    pub(super) fn baseline_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        self.ac_analysis_expression_family_contract(deck)
            .or_else(|| self.bjt_external_node_family_contract(deck))
            .or_else(|| self.delimited_expression_family_contract(deck))
            .or_else(|| self.dc_analysis_expression_family_contract(deck))
            .or_else(|| self.sin_expression_family_contract(deck))
            .or_else(|| self.param_expression_family_contract(deck))
            .or_else(|| self.passive_temperature_override_family_contract(deck))
            .or_else(|| self.transient_analysis_expression_family_contract(deck))
            .or_else(|| self.subckt_parameter_precedence_family_contract(deck))
            .or_else(|| self.scoped_model_family_contract(deck))
            .or_else(|| self.subckt_family_contract(deck))
            .or_else(|| self.supernode_family_contract(deck))
    }

    pub(super) fn validate_nested_include_subcircuit_auxiliary_state(
        subcircuit: &SubcircuitDef,
    ) -> Result<(), String> {
        if !subcircuit.initial_conditions.is_empty()
            || !subcircuit.node_sets.is_empty()
            || !subcircuit.params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || subcircuit.library_ref.is_some()
        {
            return Err(format!(
                "subcircuit '{}' contains auxiliary scoped state",
                subcircuit.name
            ));
        }
        Ok(())
    }

    pub(super) fn nested_include_identity_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceNestedIncludeIdentityFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }
        let parent = deck.path.parent()?;
        let entries = fs::read_dir(parent)
            .ok()?
            .collect::<Result<Vec<_>, _>>()
            .ok()?;
        if entries.iter().any(|entry| {
            entry
                .file_type()
                .map_or(true, |file_type| !file_type.is_file())
        }) {
            return None;
        }
        let circuit_paths = entries
            .iter()
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            })
            .collect::<Vec<_>>();
        if circuit_paths.len() != 3
            || !circuit_paths
                .iter()
                .any(|path| Self::same_path(path, &deck.path))
        {
            return None;
        }

        let mut anchor = None;
        let mut workers = Vec::new();
        for path in &circuit_paths {
            let source = fs::read_to_string(path).ok()?;
            let wrapper = self.requires_upstream_wrapper(&self.relative_key(path));
            if source.is_empty() {
                if !wrapper || anchor.replace(path.clone()).is_some() {
                    return None;
                }
                continue;
            }
            if wrapper {
                return None;
            }
            let provenance = Self::nested_include_identity_provenance(&source, path).ok()?;
            let plan = self
                .static_dc_plan_for_path(path, ExpressionDialect::Xyce)
                .ok()?;
            Self::validate_nested_include_identity_dc_plan(&plan).ok()?;
            let netlist = Self::parse_xyce_netlist(&plan.source, path).ok()?;
            let snapshot = Self::nested_include_identity_family_snapshot(&netlist, &plan).ok()?;
            workers.push((path.clone(), provenance, plan, snapshot));
        }
        let anchor = anchor?;
        if workers.len() != 2 {
            return None;
        }
        let repeated = workers.iter().find(|(_, provenance, _, _)| {
            provenance.representation
                == XyceNestedIncludeIdentityRepresentation::RepeatedCanonicalTarget
        })?;
        let split = workers.iter().find(|(_, provenance, _, _)| {
            provenance.representation
                == XyceNestedIncludeIdentityRepresentation::SplitIdenticalTargets
        })?;
        if Self::same_path(&repeated.0, &split.0)
            || repeated.1.canonical_source != split.1.canonical_source
            || repeated.1.expanded_source != split.1.expanded_source
            || repeated.2.print.probes != split.2.print.probes
            || !Self::dc_sweeps_match_exactly(&repeated.2.dc, &split.2.dc)
            || repeated.3 != split.3
        {
            return None;
        }
        let all_support_paths = repeated
            .1
            .support_paths
            .union(&split.1.support_paths)
            .cloned()
            .collect::<BTreeSet<_>>();
        if all_support_paths.len() != 2
            || repeated.1.support_paths.len() != 1
            || split.1.support_paths.len() != 2
        {
            return None;
        }
        let expected_paths = circuit_paths
            .iter()
            .cloned()
            .chain(all_support_paths.iter().cloned())
            .map(|path| path.canonicalize().unwrap_or(path))
            .collect::<BTreeSet<_>>();
        let actual_paths = entries
            .iter()
            .map(|entry| {
                let path = entry.path();
                path.canonicalize().unwrap_or(path)
            })
            .collect::<BTreeSet<_>>();
        if actual_paths != expected_paths {
            return None;
        }

        let artifact_dir = self
            .static_output_reference_path(&repeated.0, "artifact")?
            .parent()?
            .to_path_buf();
        if artifact_dir.try_exists().ok()? {
            if !fs::metadata(&artifact_dir).ok()?.is_dir()
                || fs::read_dir(&artifact_dir).ok()?.next().is_some()
            {
                return None;
            }
        }

        let role = if Self::same_path(&deck.path, &anchor) {
            XyceNestedIncludeIdentityFamilyRole::Anchor
        } else if Self::same_path(&deck.path, &repeated.0) {
            XyceNestedIncludeIdentityFamilyRole::RepeatedTargetBaseline
        } else if Self::same_path(&deck.path, &split.0) {
            XyceNestedIncludeIdentityFamilyRole::SplitIdenticalTargetsMember
        } else {
            return None;
        };
        let target_path = match role {
            XyceNestedIncludeIdentityFamilyRole::Anchor => None,
            XyceNestedIncludeIdentityFamilyRole::RepeatedTargetBaseline => Some(repeated.0.clone()),
            XyceNestedIncludeIdentityFamilyRole::SplitIdenticalTargetsMember => {
                Some(split.0.clone())
            }
        };
        Some(XyceNestedIncludeIdentityFamilyContract {
            relational: XyceBaselineFamilyContract {
                kind: XyceBaselineFamilyKind::NestedIncludeIdentity,
                comparison: XyceBaselineFamilyComparison::ExactPrn,
                family: parent.file_name()?.to_str()?.to_string(),
                baseline_path: repeated.0.clone(),
                member_paths: vec![repeated.0.clone(), split.0.clone()],
                target_path,
            },
            role,
        })
    }

    pub(super) fn age_cap_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceAgeCapFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/") {
            return None;
        }
        let parent = deck.path.parent()?;
        let mut anchor = None;
        let mut workers = Vec::new();
        for entry in fs::read_dir(parent).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            if !entry.file_type().ok()?.is_file() {
                return None;
            }
            let source = fs::read_to_string(&path).ok()?;
            let wrapper = self.requires_upstream_wrapper(&self.relative_key(&path));
            if source.trim().is_empty() {
                if !wrapper || anchor.replace(path).is_some() {
                    return None;
                }
                continue;
            }
            if wrapper {
                return None;
            }
            let plan = self
                .static_tran_family_plan_for_path(
                    &path,
                    XyceStaticTranPlanPurpose::AgeCapRelationalFamily,
                )
                .ok()?;
            Self::validate_age_cap_transient_plan(&plan).ok()?;
            let netlist = Self::parse_xyce_netlist(&plan.source, &path).ok()?;
            let snapshot = Self::age_cap_family_snapshot(&netlist, &plan.print).ok()?;
            workers.push((path, plan, snapshot));
        }
        let anchor = anchor?;
        if workers.len() != 2
            || ![&anchor]
                .into_iter()
                .chain(workers.iter().map(|(path, _, _)| path))
                .any(|path| Self::same_path(path, &deck.path))
        {
            return None;
        }
        let aged = workers.iter().find(|(_, _, snapshot)| {
            snapshot.representation == XyceAgeCapRepresentation::NativeAge
        })?;
        let equivalent = workers.iter().find(|(_, _, snapshot)| {
            snapshot.representation == XyceAgeCapRepresentation::ParameterExpression
        })?;
        if Self::same_path(&aged.0, &equivalent.0)
            || aged.1.print.probes != equivalent.1.print.probes
            || !Self::tran_analyses_match_exactly(&aged.1.tran, &equivalent.1.tran)
            || aged.1.timeint_conststep != equivalent.1.timeint_conststep
            || Self::compare_age_cap_family_snapshots(&aged.2, &equivalent.2).is_err()
        {
            return None;
        }
        let family_paths = [&anchor, &aged.0, &equivalent.0];
        let output_parent = self
            .static_prn_reference_path(&aged.0)?
            .parent()?
            .to_path_buf();
        let prefixes = family_paths
            .iter()
            .filter_map(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| format!("{name}."))
            })
            .collect::<Vec<_>>();
        if prefixes.len() != family_paths.len() {
            return None;
        }
        let mut artifacts = Vec::new();
        for entry in fs::read_dir(&output_parent).ok()? {
            let entry = entry.ok()?;
            let name = entry.file_name();
            let name = name.to_str()?;
            if !prefixes.iter().any(|prefix| name.starts_with(prefix)) {
                continue;
            }
            if !entry.file_type().ok()?.is_file() {
                return None;
            }
            artifacts.push(entry.path());
        }
        let aged_prn = self.static_prn_reference_path(&aged.0)?;
        if artifacts.len() != 1
            || !Self::same_path(&artifacts[0], &aged_prn)
            || fs::metadata(&aged_prn).ok()?.len() == 0
        {
            return None;
        }
        let role = if Self::same_path(&deck.path, &anchor) {
            XyceAgeCapFamilyRole::Anchor
        } else if Self::same_path(&deck.path, &aged.0) {
            XyceAgeCapFamilyRole::AgedBaseline
        } else if Self::same_path(&deck.path, &equivalent.0) {
            XyceAgeCapFamilyRole::EquivalentMember
        } else {
            return None;
        };
        let target_path = match role {
            XyceAgeCapFamilyRole::Anchor => None,
            XyceAgeCapFamilyRole::AgedBaseline => Some(aged.0.clone()),
            XyceAgeCapFamilyRole::EquivalentMember => Some(equivalent.0.clone()),
        };
        Some(XyceAgeCapFamilyContract {
            relational: XyceBaselineFamilyContract {
                kind: XyceBaselineFamilyKind::AgeCap,
                comparison: XyceBaselineFamilyComparison::ExactPrn,
                family: parent.file_name()?.to_str()?.to_string(),
                baseline_path: aged.0.clone(),
                member_paths: vec![aged.0.clone(), equivalent.0.clone()],
                target_path,
            },
            role,
        })
    }

    pub(super) fn diode_model_alias_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceDiodeModelAliasFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }
        let parent = deck.path.parent()?;
        let mut records = Vec::new();
        for entry in fs::read_dir(parent).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            if !entry.file_type().ok()?.is_file() {
                return None;
            }
            let source = fs::read_to_string(&path).ok()?;
            let wrapper = self.requires_upstream_wrapper(&self.relative_key(&path));
            let worker = if source.trim().is_empty() {
                None
            } else {
                let plan = self
                    .static_tran_family_plan_for_path(
                        &path,
                        XyceStaticTranPlanPurpose::RelationalFamily,
                    )
                    .ok()?;
                Self::validate_diode_model_alias_transient_plan(&plan).ok()?;
                let netlist = Self::parse_xyce_netlist(&plan.source, &path).ok()?;
                let snapshot =
                    Self::diode_model_alias_family_snapshot(&netlist, &plan.print).ok()?;
                Some((plan, snapshot))
            };
            records.push((path, wrapper, worker));
        }
        if records.len() != 3
            || !records
                .iter()
                .any(|(path, _, _)| Self::same_path(path, &deck.path))
        {
            return None;
        }
        let anchors = records
            .iter()
            .filter(|(_, wrapper, worker)| *wrapper && worker.is_none())
            .collect::<Vec<_>>();
        let workers = records
            .iter()
            .filter(|(_, wrapper, worker)| !*wrapper && worker.is_some())
            .collect::<Vec<_>>();
        if anchors.len() != 1
            || workers.len() != 2
            || records.iter().any(|(_, wrapper, worker)| {
                (*wrapper && worker.is_some()) || (!*wrapper && worker.is_none())
            })
        {
            return None;
        }
        let canonical = workers.iter().find(|(_, _, worker)| {
            worker.as_ref().is_some_and(|(_, snapshot)| {
                snapshot.representation == XyceDiodeModelAliasRepresentation::Canonical
            })
        })?;
        let alias = workers.iter().find(|(_, _, worker)| {
            worker.as_ref().is_some_and(|(_, snapshot)| {
                snapshot.representation == XyceDiodeModelAliasRepresentation::Alias
            })
        })?;
        if Self::same_path(&canonical.0, &alias.0) {
            return None;
        }
        let (canonical_plan, canonical_snapshot) = canonical.2.as_ref()?;
        let (alias_plan, alias_snapshot) = alias.2.as_ref()?;
        if canonical_plan.print.probes != alias_plan.print.probes
            || !Self::tran_analyses_match_exactly(&canonical_plan.tran, &alias_plan.tran)
            || canonical_plan.timeint_conststep != alias_plan.timeint_conststep
            || canonical_plan.wrapper_tolerance.is_some()
            || alias_plan.wrapper_tolerance.is_some()
            || canonical_plan.comparison_mode != alias_plan.comparison_mode
            || Self::compare_diode_model_alias_family_snapshots(canonical_snapshot, alias_snapshot)
                .is_err()
        {
            return None;
        }

        let artifact_dir = self
            .static_output_reference_path(&canonical.0, "artifact")?
            .parent()?
            .to_path_buf();
        if artifact_dir.try_exists().ok()? {
            if !fs::metadata(&artifact_dir).ok()?.is_dir() {
                return None;
            }
            if let Some(artifact) = fs::read_dir(&artifact_dir).ok()?.next() {
                let artifact = artifact.ok()?;
                artifact.file_type().ok()?;
                return None;
            }
        }

        let anchor_path = anchors[0].0.clone();
        let role = if Self::same_path(&deck.path, &anchor_path) {
            XyceDiodeModelAliasFamilyRole::Anchor
        } else if Self::same_path(&deck.path, &canonical.0) {
            XyceDiodeModelAliasFamilyRole::CanonicalBaseline
        } else if Self::same_path(&deck.path, &alias.0) {
            XyceDiodeModelAliasFamilyRole::AliasMember
        } else {
            return None;
        };
        let target_path = match role {
            XyceDiodeModelAliasFamilyRole::Anchor => None,
            XyceDiodeModelAliasFamilyRole::CanonicalBaseline => Some(canonical.0.clone()),
            XyceDiodeModelAliasFamilyRole::AliasMember => Some(alias.0.clone()),
        };
        Some(XyceDiodeModelAliasFamilyContract {
            relational: XyceBaselineFamilyContract {
                kind: XyceBaselineFamilyKind::DiodeModelAlias,
                comparison: XyceBaselineFamilyComparison::ExactPrn,
                family: parent.file_name()?.to_str()?.to_string(),
                baseline_path: canonical.0.clone(),
                member_paths: vec![canonical.0.clone(), alias.0.clone()],
                target_path,
            },
            role,
        })
    }

    pub(super) fn switch_state_case_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceSwitchStateCaseFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }
        let parent = deck.path.parent()?;
        let mut records = Vec::new();
        for entry in fs::read_dir(parent).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            if !entry.file_type().ok()?.is_file() {
                return None;
            }
            let artifact_prefix = format!("{}.", path.file_name()?.to_str()?).to_ascii_lowercase();
            let artifact_dir = self
                .static_output_reference_path(&path, "artifact")?
                .parent()?
                .to_path_buf();
            if artifact_dir.try_exists().ok()? {
                if !fs::metadata(&artifact_dir).ok()?.is_dir() {
                    return None;
                }
                for artifact in fs::read_dir(&artifact_dir).ok()? {
                    let artifact = artifact.ok()?;
                    let name = artifact.file_name();
                    let name = name.to_str()?.to_ascii_lowercase();
                    artifact.file_type().ok()?;
                    if name.starts_with(&artifact_prefix) {
                        return None;
                    }
                }
            }
            let source = fs::read_to_string(&path).ok()?;
            let wrapper = self.requires_upstream_wrapper(&self.relative_key(&path));
            if source.trim().is_empty() {
                records.push((path, wrapper, None));
                continue;
            }
            let plan = self
                .static_tran_family_plan_for_path(
                    &path,
                    XyceStaticTranPlanPurpose::RelationalFamily,
                )
                .ok()?;
            Self::validate_switch_state_case_transient_plan(&plan).ok()?;
            let netlist = Self::parse_xyce_netlist(&plan.source, &path).ok()?;
            let snapshot = Self::switch_state_case_family_snapshot(&netlist, &plan.print).ok()?;
            records.push((path, wrapper, Some((plan, snapshot))));
        }
        if records.len() != 3
            || !records
                .iter()
                .any(|(path, _, _)| Self::same_path(path, &deck.path))
        {
            return None;
        }
        let anchors = records
            .iter()
            .filter(|(_, wrapper, worker)| *wrapper && worker.is_none())
            .collect::<Vec<_>>();
        let workers = records
            .iter()
            .filter(|(_, wrapper, worker)| !*wrapper && worker.is_some())
            .collect::<Vec<_>>();
        if anchors.len() != 1
            || workers.len() != 2
            || records.iter().any(|(_, wrapper, worker)| {
                (*wrapper && worker.is_some()) || (!*wrapper && worker.is_none())
            })
        {
            return None;
        }
        let uppercase = workers.iter().find(|(_, _, worker)| {
            worker.as_ref().is_some_and(|(_, snapshot)| {
                snapshot.representation == XyceSwitchStateCaseRepresentation::Uppercase
            })
        })?;
        let lowercase = workers.iter().find(|(_, _, worker)| {
            worker.as_ref().is_some_and(|(_, snapshot)| {
                snapshot.representation == XyceSwitchStateCaseRepresentation::Lowercase
            })
        })?;
        if Self::same_path(&uppercase.0, &lowercase.0) {
            return None;
        }
        let (_, uppercase_snapshot) = uppercase.2.as_ref()?;
        let (_, lowercase_snapshot) = lowercase.2.as_ref()?;
        Self::compare_switch_state_case_family_snapshots(uppercase_snapshot, lowercase_snapshot)
            .ok()?;
        let anchor_path = anchors[0].0.clone();
        let role = if Self::same_path(&deck.path, &anchor_path) {
            XyceSwitchStateCaseFamilyRole::Anchor
        } else if Self::same_path(&deck.path, &uppercase.0) {
            XyceSwitchStateCaseFamilyRole::UppercaseBaseline
        } else if Self::same_path(&deck.path, &lowercase.0) {
            XyceSwitchStateCaseFamilyRole::LowercaseMember
        } else {
            return None;
        };
        let target_path = match role {
            XyceSwitchStateCaseFamilyRole::Anchor => None,
            XyceSwitchStateCaseFamilyRole::UppercaseBaseline => Some(uppercase.0.clone()),
            XyceSwitchStateCaseFamilyRole::LowercaseMember => Some(lowercase.0.clone()),
        };
        Some(XyceSwitchStateCaseFamilyContract {
            relational: XyceBaselineFamilyContract {
                kind: XyceBaselineFamilyKind::SwitchStateCase,
                comparison: XyceBaselineFamilyComparison::ExactPrn,
                family: parent.file_name()?.to_str()?.to_string(),
                baseline_path: uppercase.0.clone(),
                member_paths: vec![uppercase.0.clone(), lowercase.0.clone()],
                target_path,
            },
            role,
        })
    }

    pub(super) fn delimited_expression_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }
        let parent = deck.path.parent()?;
        let mut records = Vec::new();
        for entry in fs::read_dir(parent).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            if !entry.file_type().ok()?.is_file()
                || fs::metadata(&path)
                    .ok()
                    .is_none_or(|metadata| metadata.len() == 0)
                || self
                    .static_prn_reference_path(&path)
                    .is_some_and(|reference| reference.is_file())
            {
                return None;
            }
            let wrapper = self.requires_upstream_wrapper(&self.relative_key(&path));
            let source = fs::read_to_string(&path).ok()?;
            let (representation, _) =
                Self::delimited_expression_source_qualification(&source).ok()?;
            if wrapper != (representation == XyceDelimitedExpressionRepresentation::SingleQuoted) {
                return None;
            }
            let plan = self
                .static_dc_plan_for_path(&path, ExpressionDialect::Xyce)
                .ok()?;
            Self::validate_delimited_expression_plan(&plan).ok()?;
            let netlist = Self::parse_xyce_netlist(&plan.source, &path).ok()?;
            let snapshot = Self::delimited_expression_family_snapshot(&plan, &netlist).ok()?;
            records.push((path, wrapper, plan, snapshot));
        }
        if records.len() != 2
            || !records
                .iter()
                .any(|(path, _, _, _)| Self::same_path(path, &deck.path))
            || records.iter().filter(|(_, wrapper, _, _)| *wrapper).count() != 1
        {
            return None;
        }
        let baseline = records.iter().find(|(_, wrapper, _, _)| !wrapper)?;
        let target = records.iter().find(|(_, wrapper, _, _)| *wrapper)?;
        if baseline.2.print_format.is_some()
            || target.2.print_format.is_some()
            || baseline.2.print.probes != target.2.print.probes
            || !Self::dc_sweeps_match_exactly(&baseline.2.dc, &target.2.dc)
            || Self::compare_delimited_expression_snapshots(&baseline.3, &target.3).is_err()
        {
            return None;
        }
        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::DelimitedExpression,
            comparison: XyceBaselineFamilyComparison::ExactPrnCaseInsensitive,
            family: parent.file_name()?.to_str()?.to_string(),
            baseline_path: baseline.0.clone(),
            member_paths: vec![baseline.0.clone(), target.0.clone()],
            target_path: Some(deck.path.clone()),
        })
    }

    pub(super) fn analytic_rc_wrapper_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceAnalyticRcContract, String>> {
        let source = self.analytic_generated_wrapper_source(deck)?;
        if !Self::is_analytic_rc_wrapper_candidate(&source) {
            return None;
        }
        Some((|| {
            let source_contract = Self::analytic_rc_source_contract(&source)?;
            let plan = self.static_tran_plan_for_path_with_purpose(
                &deck.path,
                XyceStaticTranPlanPurpose::AnalyticOracle,
            )?;
            Self::validate_analytic_rc_plan(&plan, &source_contract)?;
            let netlist = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
                .map_err(|err| format!("netlist parser rejected analytic RC deck: {err}"))?;
            let specification = Self::analytic_rc_specification(&netlist, &plan, &source_contract)?;
            Ok(XyceAnalyticRcContract {
                plan,
                specification,
            })
        })())
    }

    pub(super) fn analytic_sinusoidal_rc_wrapper_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceAnalyticSinusoidalRcContract, String>> {
        let source = self.analytic_generated_wrapper_source(deck)?;
        if !Self::is_analytic_sinusoidal_rc_wrapper_candidate(&source) {
            return None;
        }
        Some((|| {
            let source_contract = Self::analytic_sinusoidal_rc_source_contract(&source)?;
            let plan = self.static_tran_plan_for_path_with_purpose(
                &deck.path,
                XyceStaticTranPlanPurpose::AnalyticOracle,
            )?;
            Self::validate_analytic_sinusoidal_rc_plan(&plan, &source_contract)?;
            let netlist =
                Self::parse_xyce_netlist(&plan.source, &plan.deck_path).map_err(|err| {
                    format!("netlist parser rejected analytic sinusoidal RC deck: {err}")
                })?;
            let specification =
                Self::analytic_sinusoidal_rc_specification(&netlist, &plan, &source_contract)?;
            let tolerance = XyceVerifyTransientTolerance {
                relative: Value::from_bits(source_contract.verify_reltol_bits),
                absolute: Value::from_bits(source_contract.verify_abstol_bits),
                zero: XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE,
                absolute_difference: XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE,
                offset: 0.0,
            }
            .validate()?;
            Ok(XyceAnalyticSinusoidalRcContract {
                plan,
                specification,
                tolerance,
            })
        })())
    }

    pub(super) fn resistor_dtemp_relational_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceResistorDtempContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let role = match relative.as_str() {
            XYCE_RESISTOR_DTEMP_OWNER_RECORD => XyceResistorDtempRole::Owner,
            XYCE_RESISTOR_DTEMP_REFERENCE_RECORD => XyceResistorDtempRole::Reference,
            _ => return None,
        };
        Some((|| {
            let parent = deck
                .path
                .parent()
                .ok_or_else(|| "resistor DTEMP record has no sibling directory".to_string())?;
            let owner_path = parent.join("res_dtemp.cir");
            let reference_path = parent.join("res_ref.cir");
            if Self::normalize_manifest_key(&self.relative_key(&owner_path))
                != XYCE_RESISTOR_DTEMP_OWNER_RECORD
                || Self::normalize_manifest_key(&self.relative_key(&reference_path))
                    != XYCE_RESISTOR_DTEMP_REFERENCE_RECORD
            {
                return Err(
                    "owner/reference paths do not form the exact manifest-owned sibling pair"
                        .to_string(),
                );
            }
            if !self.requires_upstream_wrapper(XYCE_RESISTOR_DTEMP_OWNER_RECORD)
                || self.requires_upstream_wrapper(XYCE_RESISTOR_DTEMP_REFERENCE_RECORD)
            {
                return Err(
                    "exactly res_dtemp.cir must own the removed upstream wrapper".to_string(),
                );
            }
            for (member_role, path) in [("owner", &owner_path), ("reference", &reference_path)] {
                let metadata = fs::metadata(path)
                    .map_err(|err| format!("could not inspect {member_role} sibling: {err}"))?;
                if !metadata.is_file() || metadata.len() == 0 {
                    return Err(format!(
                        "resistor DTEMP {member_role} sibling must be a nonempty regular file"
                    ));
                }
                self.reject_wrapper_output_artifacts(path)
                    .map_err(|err| format!("resistor DTEMP {member_role} {err}"))?;
            }

            let owner_plan = self.static_dc_plan_for_path(&owner_path, ExpressionDialect::Xyce)?;
            let reference_plan =
                self.static_dc_plan_for_path(&reference_path, ExpressionDialect::Xyce)?;
            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_path)
                .map_err(|err| format!("owner netlist parse failed: {err}"))?;
            let reference_netlist =
                Self::parse_xyce_netlist(&reference_plan.source, &reference_path)
                    .map_err(|err| format!("reference netlist parse failed: {err}"))?;
            let owner_snapshot = Self::resistor_dtemp_snapshot(
                &owner_plan,
                &owner_netlist,
                XyceResistorDtempRole::Owner,
            )?;
            let reference_snapshot = Self::resistor_dtemp_snapshot(
                &reference_plan,
                &reference_netlist,
                XyceResistorDtempRole::Reference,
            )?;
            if owner_snapshot != reference_snapshot {
                return Err(format!(
                    "owner/reference semantics differ after TEMP-versus-DTEMP normalization: owner={owner_snapshot:?}, reference={reference_snapshot:?}"
                ));
            }
            Ok(XyceResistorDtempContract {
                owner_plan,
                reference_plan,
                role,
            })
        })())
    }

    pub(super) fn bug647_resistor_relational_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug647ResistorContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let role = match relative.as_str() {
            XYCE_BUG647_RESISTOR_OWNER_RECORD => XyceBug647ResistorRole::Owner,
            XYCE_BUG647_RESISTOR_REFERENCE_RECORD => {
                XyceBug647ResistorRole::ModelParameterReference
            }
            _ => return None,
        };
        Some((|| {
            let parent = deck
                .path
                .parent()
                .ok_or_else(|| "BUG 647 resistor record has no sibling directory".to_string())?;
            let owner_path = parent.join("semic_resistor.cir");
            let reference_path = parent.join("semic_resistor_modpar.cir");
            if Self::normalize_manifest_key(&self.relative_key(&owner_path))
                != XYCE_BUG647_RESISTOR_OWNER_RECORD
                || Self::normalize_manifest_key(&self.relative_key(&reference_path))
                    != XYCE_BUG647_RESISTOR_REFERENCE_RECORD
            {
                return Err("owner/reference paths are not the exact BUG 647 sibling pair".into());
            }
            if !self.requires_upstream_wrapper(XYCE_BUG647_RESISTOR_OWNER_RECORD)
                || self.requires_upstream_wrapper(XYCE_BUG647_RESISTOR_REFERENCE_RECORD)
            {
                return Err(
                    "exactly semic_resistor.cir must own the removed upstream wrapper".into(),
                );
            }
            for (member_role, path) in [
                ("instance-parameter owner", &owner_path),
                ("model-parameter reference", &reference_path),
            ] {
                let metadata = fs::metadata(path)
                    .map_err(|err| format!("could not inspect {member_role}: {err}"))?;
                if !metadata.is_file() || metadata.len() == 0 {
                    return Err(format!("{member_role} must be a nonempty regular file"));
                }
                self.reject_wrapper_output_artifacts(path)
                    .map_err(|err| format!("{member_role} {err}"))?;
            }
            let owner_plan = self.static_dc_plan_for_path(&owner_path, ExpressionDialect::Xyce)?;
            let reference_plan =
                self.static_dc_plan_for_path(&reference_path, ExpressionDialect::Xyce)?;
            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_path)
                .map_err(|err| format!("instance-parameter owner parse failed: {err}"))?;
            let reference_netlist =
                Self::parse_xyce_netlist(&reference_plan.source, &reference_path)
                    .map_err(|err| format!("model-parameter reference parse failed: {err}"))?;
            Self::validate_bug647_resistor_member(
                &owner_plan,
                &owner_netlist,
                XyceBug647ResistorRole::Owner,
            )?;
            Self::validate_bug647_resistor_member(
                &reference_plan,
                &reference_netlist,
                XyceBug647ResistorRole::ModelParameterReference,
            )?;
            Ok(XyceBug647ResistorContract {
                owner_plan,
                reference_plan,
                role,
            })
        })())
    }

    pub(super) fn bug655_continuation_relational_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug655ContinuationContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let role = match relative.as_str() {
            XYCE_BUG655_CONTINUATION_OWNER_RECORD => XyceBug655ContinuationRole::ColumnZeroOwner,
            XYCE_BUG655_CONTINUATION_REFERENCE_RECORD => {
                XyceBug655ContinuationRole::LeadingSpaceReference
            }
            _ => return None,
        };
        Some((|| {
            let parent = deck
                .path
                .parent()
                .ok_or_else(|| "BUG 655 record has no sibling directory".to_string())?;
            let owner_path = parent.join("contLine.cir");
            let reference_path = parent.join("contLine_with_spaces.cir");
            if Self::normalize_manifest_key(&self.relative_key(&owner_path))
                != XYCE_BUG655_CONTINUATION_OWNER_RECORD
                || Self::normalize_manifest_key(&self.relative_key(&reference_path))
                    != XYCE_BUG655_CONTINUATION_REFERENCE_RECORD
            {
                return Err("owner/reference paths are not the exact BUG 655 sibling pair".into());
            }
            if !self.requires_upstream_wrapper(XYCE_BUG655_CONTINUATION_OWNER_RECORD)
                || self.requires_upstream_wrapper(XYCE_BUG655_CONTINUATION_REFERENCE_RECORD)
            {
                return Err("exactly contLine.cir must own the removed upstream wrapper".into());
            }

            let mut directory_entries = fs::read_dir(parent)
                .map_err(|err| format!("could not inspect BUG 655 directory: {err}"))?
                .map(|entry| {
                    let entry =
                        entry.map_err(|err| format!("could not inspect BUG 655 entry: {err}"))?;
                    let file_type = entry
                        .file_type()
                        .map_err(|err| format!("could not inspect BUG 655 entry type: {err}"))?;
                    let name = entry.file_name().into_string().map_err(|_| {
                        "BUG 655 directory contains a non-Unicode entry".to_string()
                    })?;
                    Ok((name, file_type.is_file()))
                })
                .collect::<Result<Vec<_>, String>>()?;
            directory_entries.sort();
            if directory_entries
                != [
                    ("contLine.cir".to_string(), true),
                    ("contLine_with_spaces.cir".to_string(), true),
                ]
            {
                return Err(format!(
                    "BUG 655 directory must contain exactly its two regular .cir records, got {directory_entries:?}"
                ));
            }
            for (member_role, path) in [
                ("column-zero continuation owner", &owner_path),
                ("leading-space continuation reference", &reference_path),
            ] {
                let metadata = fs::metadata(path)
                    .map_err(|err| format!("could not inspect {member_role}: {err}"))?;
                if !metadata.is_file() || metadata.len() == 0 {
                    return Err(format!("{member_role} must be a nonempty regular file"));
                }
                self.reject_wrapper_output_artifacts(path)
                    .map_err(|err| format!("{member_role} {err}"))?;
            }

            let owner_source = fs::read_to_string(&owner_path)
                .map_err(|err| format!("failed to read BUG 655 owner: {err}"))?;
            let reference_source = fs::read_to_string(&reference_path)
                .map_err(|err| format!("failed to read BUG 655 reference: {err}"))?;
            Self::validate_bug655_continuation_source_pair(&owner_source, &reference_source)?;

            let owner_plan = self.static_dc_plan_for_path(&owner_path, ExpressionDialect::Xyce)?;
            let reference_plan =
                self.static_dc_plan_for_path(&reference_path, ExpressionDialect::Xyce)?;
            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_path)
                .map_err(|err| format!("BUG 655 owner parse failed: {err}"))?;
            let reference_netlist =
                Self::parse_xyce_netlist(&reference_plan.source, &reference_path)
                    .map_err(|err| format!("BUG 655 reference parse failed: {err}"))?;
            Self::validate_bug655_continuation_member(
                &owner_plan,
                &owner_netlist,
                XyceBug655ContinuationRole::ColumnZeroOwner,
            )?;
            Self::validate_bug655_continuation_member(
                &reference_plan,
                &reference_netlist,
                XyceBug655ContinuationRole::LeadingSpaceReference,
            )?;
            Self::validate_bug655_continuation_semantic_identity(
                &owner_netlist,
                &reference_netlist,
            )?;

            Ok(XyceBug655ContinuationContract {
                owner_plan,
                reference_plan,
                role,
            })
        })())
    }

    pub(super) fn bug662_long_header_relational_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug662HeaderContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let role = match relative.as_str() {
            XYCE_BUG662_LONG_HEADER_OWNER_RECORD => XyceBug662HeaderRole::LongHeaderOwner,
            XYCE_BUG662_SHORT_HEADER_REFERENCE_RECORD => XyceBug662HeaderRole::ShortHeaderReference,
            _ => return None,
        };
        Some((|| {
            let parent = deck
                .path
                .parent()
                .ok_or_else(|| "BUG 662 record has no sibling directory".to_string())?;
            let owner_path = parent.join("headerLineLengthMoreThan256.cir");
            let reference_path = parent.join("headerLineLengthLessThan256.cir");
            if Self::normalize_manifest_key(&self.relative_key(&owner_path))
                != XYCE_BUG662_LONG_HEADER_OWNER_RECORD
                || Self::normalize_manifest_key(&self.relative_key(&reference_path))
                    != XYCE_BUG662_SHORT_HEADER_REFERENCE_RECORD
            {
                return Err("owner/reference paths are not the exact BUG 662 sibling pair".into());
            }
            if !self.requires_upstream_wrapper(XYCE_BUG662_LONG_HEADER_OWNER_RECORD)
                || self.requires_upstream_wrapper(XYCE_BUG662_SHORT_HEADER_REFERENCE_RECORD)
            {
                return Err(
                    "exactly headerLineLengthMoreThan256.cir must own the removed wrapper".into(),
                );
            }
            for (member_role, path) in [
                ("long-header owner", &owner_path),
                ("short-header reference", &reference_path),
            ] {
                let metadata = fs::metadata(path)
                    .map_err(|err| format!("could not inspect {member_role}: {err}"))?;
                if !metadata.is_file() || metadata.len() == 0 {
                    return Err(format!("{member_role} must be a nonempty regular file"));
                }
                self.reject_wrapper_output_artifacts(path)
                    .map_err(|err| format!("{member_role} {err}"))?;
            }

            let owner_source = fs::read_to_string(&owner_path)
                .map_err(|err| format!("failed to read long-header owner: {err}"))?;
            let reference_source = fs::read_to_string(&reference_path)
                .map_err(|err| format!("failed to read short-header reference: {err}"))?;
            Self::validate_bug662_header_source_pair(&owner_source, &reference_source)?;

            let owner_plan = self.static_tran_plan_for_path_with_purpose(
                &owner_path,
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            )?;
            let reference_plan = self.static_tran_plan_for_path_with_purpose(
                &reference_path,
                XyceStaticTranPlanPurpose::RelationalFamily,
            )?;
            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_path)
                .map_err(|err| format!("long-header owner parse failed: {err}"))?;
            let reference_netlist =
                Self::parse_xyce_netlist(&reference_plan.source, &reference_path)
                    .map_err(|err| format!("short-header reference parse failed: {err}"))?;
            Self::validate_bug662_header_member(
                &owner_plan,
                &owner_netlist,
                XyceBug662HeaderRole::LongHeaderOwner,
            )?;
            Self::validate_bug662_header_member(
                &reference_plan,
                &reference_netlist,
                XyceBug662HeaderRole::ShortHeaderReference,
            )?;
            Ok(XyceBug662HeaderContract {
                owner_plan,
                reference_plan,
                role,
            })
        })())
    }

    pub(super) fn bug667_nodeset_relational_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug667NodesetContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let role = match relative.as_str() {
            XYCE_BUG667_NODESET_OWNER_RECORD => XyceBug667NodesetRole::ScopedOwner,
            XYCE_BUG667_NODESET_REFERENCE_RECORD => {
                XyceBug667NodesetRole::ExplicitHierarchicalReference
            }
            _ => return None,
        };
        Some((|| {
            let parent = deck
                .path
                .parent()
                .ok_or_else(|| "BUG 667 NODESET record has no sibling directory".to_string())?;
            let owner_path = parent.join("nodeset_in_subckt.cir");
            let reference_path = parent.join("nodeset_not_in_subckt.cir");
            if Self::normalize_manifest_key(&self.relative_key(&owner_path))
                != XYCE_BUG667_NODESET_OWNER_RECORD
                || Self::normalize_manifest_key(&self.relative_key(&reference_path))
                    != XYCE_BUG667_NODESET_REFERENCE_RECORD
            {
                return Err("owner/reference paths are not the exact BUG 667 sibling pair".into());
            }
            if !self.requires_upstream_wrapper(XYCE_BUG667_NODESET_OWNER_RECORD)
                || self.requires_upstream_wrapper(XYCE_BUG667_NODESET_REFERENCE_RECORD)
            {
                return Err(
                    "exactly nodeset_in_subckt.cir must own the removed upstream wrapper".into(),
                );
            }
            for (member_role, path) in [
                ("subcircuit-scoped owner", &owner_path),
                ("explicit hierarchical reference", &reference_path),
            ] {
                let metadata = fs::metadata(path)
                    .map_err(|err| format!("could not inspect {member_role}: {err}"))?;
                if !metadata.is_file() || metadata.len() == 0 {
                    return Err(format!("{member_role} must be a nonempty regular file"));
                }
                self.reject_wrapper_output_artifacts(path)
                    .map_err(|err| format!("{member_role} {err}"))?;
            }

            let owner_plan = self.static_tran_plan_for_path_with_purpose(
                &owner_path,
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            )?;
            let reference_plan = self.static_tran_plan_for_path_with_purpose(
                &reference_path,
                XyceStaticTranPlanPurpose::RelationalFamily,
            )?;
            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_path)
                .map_err(|err| format!("subcircuit-scoped owner parse failed: {err}"))?;
            let reference_netlist =
                Self::parse_xyce_netlist(&reference_plan.source, &reference_path).map_err(
                    |err| format!("explicit hierarchical reference parse failed: {err}"),
                )?;
            Self::validate_bug667_nodeset_member(
                &owner_plan,
                &owner_netlist,
                XyceBug667NodesetRole::ScopedOwner,
            )?;
            Self::validate_bug667_nodeset_member(
                &reference_plan,
                &reference_netlist,
                XyceBug667NodesetRole::ExplicitHierarchicalReference,
            )?;

            let owner_nodesets = Self::bug667_effective_nodeset_map(&owner_netlist)?;
            let reference_nodesets = Self::bug667_effective_nodeset_map(&reference_netlist)?;
            let expected_nodesets = BTreeMap::from([
                ("n15967".to_string(), 0.5f64.to_bits()),
                ("x_x1.mid".to_string(), 0.5f64.to_bits()),
            ]);
            if owner_nodesets != expected_nodesets
                || reference_nodesets != expected_nodesets
                || owner_nodesets != reference_nodesets
            {
                return Err(format!(
                    "BUG 667 effective NODESET maps differ from the exact hierarchical equivalence contract: owner={owner_nodesets:?} reference={reference_nodesets:?}"
                ));
            }

            Ok(XyceBug667NodesetContract {
                owner_plan,
                reference_plan,
                role,
            })
        })())
    }

    pub(super) fn validate_bug754_member(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        role: XyceBug754GlobalParameterRole,
    ) -> Result<(), String> {
        let requires_wrapper = role == XyceBug754GlobalParameterRole::GlobalParameterOwner;
        let expected_contract = if requires_wrapper {
            XyceStaticDcContract::WrapperDefault
        } else {
            XyceStaticDcContract::PlainStatic
        };
        if Self::static_dc_contract_for_print_format(
            requires_wrapper,
            plan.print_format.as_deref(),
        )? != expected_contract
        {
            return Err(format!(
                "BUG 754 {role:?} does not map to the required {expected_contract:?} contract"
            ));
        }
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("Vdrain")
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 1.0f64.to_bits()
            || plan.dc.step.to_bits() != 0.001f64.to_bits()
            || plan
                .print
                .probes
                .iter()
                .map(|probe| Self::normalize_probe(probe))
                .ne(["v(drain)", "v(gate)", "i(vdrain)"])
        {
            return Err(format!(
                "BUG 754 member requires one diagnostic-free Vdrain 0:0.001:1 linear DC sweep and ordered default V(drain), V(gate), I(Vdrain) output; actual plan={plan:?}"
            ));
        }
        if netlist.elements.len() != 4
            || netlist.models.len() != 1
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc { .. }])
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(
                "BUG 754 member admits exactly one Level-1 NMOS, three DC sources, one DC analysis, and no auxiliary state"
                    .into(),
            );
        }
        let mut numeric_params = netlist.params.numeric_parameters();
        numeric_params.sort_by(|left, right| {
            left.0
                .to_ascii_lowercase()
                .cmp(&right.0.to_ascii_lowercase())
        });
        match role {
            XyceBug754GlobalParameterRole::GlobalParameterOwner => {
                let expected = [("vdi", 1.0f64.to_bits()), ("vgi", 0.5f64.to_bits())];
                let actual = numeric_params
                    .iter()
                    .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
                    .collect::<Vec<_>>();
                if actual
                    != expected
                        .into_iter()
                        .map(|(name, bits)| (name.to_string(), bits))
                        .collect::<Vec<_>>()
                {
                    return Err(format!(
                        "BUG 754 owner must retain exactly VDI=1.0 and VGI=0.5 global parameters, got {actual:?}"
                    ));
                }
            }
            XyceBug754GlobalParameterRole::LiteralReference if numeric_params.is_empty() => {}
            XyceBug754GlobalParameterRole::LiteralReference => {
                return Err("BUG 754 literal reference must not declare numeric parameters".into());
            }
        }
        Self::bug754_global_parameter_snapshot(plan, netlist).map(|_| ())
    }

    pub(super) fn validate_bug667_nodeset_member(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        role: XyceBug667NodesetRole,
    ) -> Result<(), String> {
        Self::validate_bug667_nodeset_statement_envelope(&plan.source)?;
        let expected_contract = match role {
            XyceBug667NodesetRole::ScopedOwner => XyceStaticTranContract::WrapperStatic,
            XyceBug667NodesetRole::ExplicitHierarchicalReference => {
                XyceStaticTranContract::PlainStatic
            }
        };
        let expected_probes = [
            "V(N15206)",
            "V(N15971)",
            "V(N15554)",
            "V(N15997)",
            "V(N16554)",
            "V(N16997)",
        ];
        if plan.contract != expected_contract
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.print.probes != expected_probes
            || plan.reference_path.is_file()
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 10e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "BUG 667 member requires one ordinary default-PRN '.TRAN 0 10ms' with the exact six ordered probes and no STEP, output override, tolerance, reference artifact, START, MAXSTEP, or UIC state: contract={:?} probes={:?} tran={:?}",
                plan.contract, plan.print.probes, plan.tran
            ));
        }
        if netlist.title.trim() != "*Analysis directives:"
            || netlist.elements.len() != 14
            || netlist.subcircuits.len() != 1
            || !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }])
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(
                "BUG 667 member admits only the canonical 14-element RC/PULSE harness, one two-element subcircuit, two NODESET hints, and one TRAN analysis"
                    .into(),
            );
        }

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let name = element.name.to_ascii_lowercase();
            if elements.insert(name.clone(), element).is_some() {
                return Err(format!(
                    "BUG 667 member contains duplicate element '{name}'"
                ));
            }
        }
        let expected_names = [
            "r_r1", "r_r2", "c_c1", "r_r3", "v_v1", "v_v2", "r_r4", "r_r5", "c_c2", "r_r6", "v_v3",
            "r_r7", "x_x1", "r_r8",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
        if elements.keys().cloned().collect::<BTreeSet<_>>() != expected_names {
            return Err(format!(
                "BUG 667 top-level element inventory changed: {:?}",
                elements.keys().collect::<Vec<_>>()
            ));
        }

        for (name, nodes, resistance) in [
            ("r_r1", ["N15206", "N15971"], 1e3),
            ("r_r2", ["N15975", "N15971"], 10.0),
            ("r_r3", ["N16095", "0"], 10.0),
            ("r_r4", ["N15554", "N15997"], 1e3),
            ("r_r5", ["N15967", "N15997"], 10.0),
            ("r_r6", ["N16112", "0"], 10.0),
            ("r_r7", ["N16554", "N16997"], 1e3),
            ("r_r8", ["N17112", "0"], 10.0),
        ] {
            Self::validate_bug667_scalar_resistor(elements[name], nodes, resistance)?;
        }
        for (name, nodes) in [
            ("c_c1", ["N16095", "N15975"]),
            ("c_c2", ["N16112", "N15967"]),
        ] {
            Self::validate_bug667_scalar_capacitor(elements[name], nodes, 1e-6)?;
        }
        for (name, node) in [("v_v1", "N15206"), ("v_v2", "N15554"), ("v_v3", "N16554")] {
            let source = elements[name];
            if source.nodes != [node, "0"]
                || !matches!(
                    source.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pulse {
                        v1,
                        v2,
                        delay,
                        rise,
                        fall,
                        width,
                        period,
                        phase,
                        width_defaults_to_zero: false,
                    }) if v1.to_bits() == 0.0f64.to_bits()
                        && v2.to_bits() == 1.0f64.to_bits()
                        && delay.to_bits() == 0.0f64.to_bits()
                        && rise.to_bits() == 1e-3f64.to_bits()
                        && fall.to_bits() == 1e-3f64.to_bits()
                        && width.to_bits() == 5e-3f64.to_bits()
                        && period.to_bits() == 1.0f64.to_bits()
                        && phase.to_bits() == 0.0f64.to_bits()
                )
            {
                return Err(format!("{name} topology or PULSE waveform changed"));
            }
        }

        let instance = elements["x_x1"];
        let ElementKind::Subcircuit {
            subckt_name,
            params,
        } = &instance.kind
        else {
            return Err("X_X1 is not the canonical subcircuit instance".into());
        };
        if instance.nodes != ["N16997", "N17112"]
            || !subckt_name.eq_ignore_ascii_case("NODESET_Subckt")
        {
            return Err("X_X1 topology or subcircuit binding changed".into());
        }
        match role {
            XyceBug667NodesetRole::ScopedOwner => match params.as_slice() {
                [(name, ParametricValue::Resolved(value))]
                    if name.eq_ignore_ascii_case("vmid") && value.to_bits() == 0.5f64.to_bits() => {
                }
                _ => return Err("owner X_X1 must retain the exact vmid=0.5 override".into()),
            },
            XyceBug667NodesetRole::ExplicitHierarchicalReference if params.is_empty() => {}
            XyceBug667NodesetRole::ExplicitHierarchicalReference => {
                return Err("reference X_X1 must not carry an instance parameter".into());
            }
        }

        let subcircuit = &netlist.subcircuits[0];
        if !subcircuit.name.eq_ignore_ascii_case("NODESET_Subckt")
            || subcircuit.ports.len() != 2
            || !subcircuit.ports[0].eq_ignore_ascii_case("in")
            || !subcircuit.ports[1].eq_ignore_ascii_case("out")
            || subcircuit.elements.len() != 2
            || !subcircuit.initial_conditions.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || subcircuit.library_ref.is_some()
            || !subcircuit.nested_subcircuits.is_empty()
        {
            return Err("NODESET_Subckt definition shape or auxiliary state changed".into());
        }
        let sub_resistor = subcircuit
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R1"))
            .ok_or_else(|| "NODESET_Subckt is missing R1".to_string())?;
        Self::validate_bug667_scalar_resistor(sub_resistor, ["in", "mid"], 10.0)?;
        let sub_capacitor = subcircuit
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("C1"))
            .ok_or_else(|| "NODESET_Subckt is missing C1".to_string())?;
        Self::validate_bug667_scalar_capacitor(sub_capacitor, ["mid", "out"], 1e-6)?;

        match role {
            XyceBug667NodesetRole::ScopedOwner => {
                if subcircuit.params.len() != 1
                    || !subcircuit.params[0].0.eq_ignore_ascii_case("vmid")
                    || subcircuit.params[0].1.to_bits() != 5.0f64.to_bits()
                    || subcircuit.node_sets.len() != 1
                    || !netlist.node_sets.iter().any(|nodeset| {
                        nodeset.node.eq_ignore_ascii_case("N15967")
                            && nodeset.voltage.to_bits() == 0.5f64.to_bits()
                            && nodeset.voltage_expr.is_none()
                    })
                {
                    return Err(
                        "owner must retain vmid=5.0 formal, vmid=0.5 instance override, local V(mid)={vmid}, and common V(N15967)=0.5"
                            .into(),
                    );
                }
                let local = &subcircuit.node_sets[0];
                if !local.node.eq_ignore_ascii_case("mid")
                    || local.voltage.to_bits() != 5.0f64.to_bits()
                    || local
                        .voltage_expr
                        .as_deref()
                        .is_none_or(|expr| !expr.eq_ignore_ascii_case("vmid"))
                    || netlist.node_sets.len() != 1
                {
                    return Err(
                        "owner local NODESET must remain deferred as V(mid)={vmid} before instance-scope flattening"
                            .into(),
                    );
                }
            }
            XyceBug667NodesetRole::ExplicitHierarchicalReference => {
                if !subcircuit.params.is_empty()
                    || !subcircuit.node_sets.is_empty()
                    || netlist.node_sets.len() != 2
                {
                    return Err(
                        "reference subcircuit must have no parameters/local NODESET and exactly two top-level NODESET hints"
                            .into(),
                    );
                }
                let raw = netlist
                    .node_sets
                    .iter()
                    .map(|nodeset| {
                        (
                            nodeset.node.to_ascii_lowercase().replace(':', "."),
                            nodeset.voltage.to_bits(),
                            nodeset.voltage_expr.is_none(),
                        )
                    })
                    .collect::<BTreeSet<_>>();
                if raw
                    != BTreeSet::from([
                        ("n15967".to_string(), 0.5f64.to_bits(), true),
                        ("x_x1.mid".to_string(), 0.5f64.to_bits(), true),
                    ])
                {
                    return Err(format!(
                        "reference raw top-level NODESET representation changed: {raw:?}"
                    ));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_bug667_scalar_resistor(
        element: &rspice_core::netlist::Element,
        nodes: [&str; 2],
        expected: Value,
    ) -> Result<(), String> {
        let ElementKind::Resistor {
            value,
            value_expr: None,
            model: None,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return Err(format!(
                "BUG 667 element '{}' is not a scalar resistor",
                element.name
            ));
        };
        if element.nodes.len() != 2
            || !element.nodes[0].eq_ignore_ascii_case(nodes[0])
            || !element.nodes[1].eq_ignore_ascii_case(nodes[1])
            || value.to_bits() != expected.to_bits()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!(
                "BUG 667 resistor '{}' topology, value, or parameter state changed",
                element.name
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug667_scalar_capacitor(
        element: &rspice_core::netlist::Element,
        nodes: [&str; 2],
        expected: Value,
    ) -> Result<(), String> {
        let ElementKind::Capacitor {
            value,
            value_expr: None,
            initial_voltage: None,
            model: None,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return Err(format!(
                "BUG 667 element '{}' is not a scalar capacitor",
                element.name
            ));
        };
        if element.nodes.len() != 2
            || !element.nodes[0].eq_ignore_ascii_case(nodes[0])
            || !element.nodes[1].eq_ignore_ascii_case(nodes[1])
            || value.to_bits() != expected.to_bits()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!(
                "BUG 667 capacitor '{}' topology, value, or parameter state changed",
                element.name
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug667_nodeset_statement_envelope(source: &str) -> Result<(), String> {
        let body = source.split_once('\n').map_or("", |(_, body)| body);
        let mut counts = BTreeMap::<String, usize>::new();
        for line in Self::logical_netlist_lines(body) {
            let statement = Self::strip_netlist_comment(&line).trim();
            if statement.is_empty() {
                continue;
            }
            let head = statement
                .split_ascii_whitespace()
                .next()
                .unwrap_or_default()
                .to_ascii_lowercase();
            let key = if head.starts_with('.') {
                match head.as_str() {
                    ".tran" | ".print" | ".nodeset" | ".subckt" | ".ends" | ".end" => head,
                    _ => return Err(format!("unrelated directive '{head}' in BUG 667 pair")),
                }
            } else {
                match head.as_bytes().first().map(u8::to_ascii_lowercase) {
                    Some(b'r') => "r".to_string(),
                    Some(b'c') => "c".to_string(),
                    Some(b'v') => "v".to_string(),
                    Some(b'x') => "x".to_string(),
                    _ => return Err(format!("unrelated element '{head}' in BUG 667 pair")),
                }
            };
            *counts.entry(key).or_default() += 1;
        }
        for (key, expected) in [
            ("r", 9),
            ("c", 3),
            ("v", 3),
            ("x", 1),
            (".tran", 1),
            (".print", 1),
            (".nodeset", 2),
            (".subckt", 1),
            (".ends", 1),
            (".end", 1),
        ] {
            if counts.remove(key) != Some(expected) {
                return Err(format!(
                    "BUG 667 statement count for '{key}' must be {expected}"
                ));
            }
        }
        if !counts.is_empty() {
            return Err(format!("BUG 667 source has extra statements: {counts:?}"));
        }
        Ok(())
    }

    pub(super) fn validate_bug655_continuation_member(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        role: XyceBug655ContinuationRole,
    ) -> Result<(), String> {
        let requires_wrapper = role == XyceBug655ContinuationRole::ColumnZeroOwner;
        let expected_contract = if requires_wrapper {
            XyceStaticDcContract::WrapperDefault
        } else {
            XyceStaticDcContract::PlainStatic
        };
        if Self::static_dc_contract_for_print_format(
            requires_wrapper,
            plan.print_format.as_deref(),
        )? != expected_contract
        {
            return Err(format!(
                "BUG 655 {role:?} does not map to the required {expected_contract:?} DC contract"
            ));
        }
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("I1")
            || plan.dc.start.to_bits() != (-100.0f64 * 1.0e-6).to_bits()
            || plan.dc.stop.to_bits() != (100.0f64 * 1.0e-6).to_bits()
            || plan.dc.step.to_bits() != (10.0f64 * 1.0e-6).to_bits()
            || plan
                .print
                .probes
                .iter()
                .map(|probe| Self::normalize_probe(probe))
                .ne(["i(i1)", "v(3)"])
        {
            return Err(format!(
                "BUG 655 member requires one diagnostic-free I1 -100u:10u:100u DC sweep and ordered default I(I1), V(3) output; actual execution_dir={:?} dc_data={} format={:?} steps={} diagnostics={:?} source={} mode={:?} start={} stop={} step={} sweep2={} probes={:?}",
                plan.execution_dir,
                plan.dc_data.is_some(),
                plan.print_format,
                plan.steps.len(),
                plan.diagnostics,
                plan.dc.source,
                plan.dc.mode,
                plan.dc.start,
                plan.dc.stop,
                plan.dc.step,
                plan.dc.sweep2.is_some(),
                plan.print.probes
            ));
        }
        if netlist.analyses.len() != 1
            || !matches!(netlist.analyses[0], AnalysisCommand::Dc { .. })
            || netlist.elements.len() != 5
            || netlist.models.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(
                "BUG 655 member admits exactly VCC, I1, R1, R2, Q1, one NPN model, and one DC analysis"
                    .into(),
            );
        }
        let actual = Self::bug655_continuation_snapshot(netlist)?;
        let expected = Self::bug655_expected_continuation_snapshot();
        if actual != expected {
            return Err(format!(
                "BUG 655 canonical circuit/model fingerprint changed: actual={actual:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug655_continuation_semantic_identity(
        owner: &Netlist,
        reference: &Netlist,
    ) -> Result<(), String> {
        let owner = Self::bug655_continuation_snapshot(owner)?;
        let reference = Self::bug655_continuation_snapshot(reference)?;
        if owner != reference {
            return Err(format!(
                "BUG 655 continuation parsing produced different logical circuits: owner={owner:?} reference={reference:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug662_header_member(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        role: XyceBug662HeaderRole,
    ) -> Result<(), String> {
        if plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.print.probes != ["V(N14950)", "V(N15037)"]
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != (100.0f64 * 1e-9).to_bits()
            || plan.tran.start.map(Value::to_bits) != Some(0.0f64.to_bits())
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "BUG 662 member requires one default two-probe .TRAN 0 100ns 0 plan without output overrides, STEPs, or tolerance directives; actual output_override={} conststep={} steps={} tolerance={:?} mode={:?} probes={:?} tran={:?}",
                plan.output_override,
                plan.timeint_conststep,
                plan.steps.len(),
                plan.wrapper_tolerance,
                plan.comparison_mode,
                plan.print.probes,
                plan.tran
            ));
        }
        let expected_contract = match role {
            XyceBug662HeaderRole::LongHeaderOwner => XyceStaticTranContract::WrapperStatic,
            XyceBug662HeaderRole::ShortHeaderReference => XyceStaticTranContract::PlainStatic,
        };
        if plan.contract != expected_contract {
            return Err(format!(
                "BUG 662 {role:?} has unexpected transient plan contract {:?}",
                plan.contract
            ));
        }
        if netlist.elements.len() != 4
            || netlist.analyses.len() != 1
            || !matches!(netlist.analyses[0], AnalysisCommand::Tran { .. })
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(
                "BUG 662 member admits exactly one T line, two resistors, one PULSE source, and one TRAN analysis"
                    .into(),
            );
        }

        let element = |name: &str| {
            netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("BUG 662 member is missing {name}"))
        };
        let line = element("T_T1")?;
        if line.nodes != ["N14950", "0", "N15037", "0"]
            || !matches!(
                line.kind,
                ElementKind::TransmissionLine {
                    z0: Some(z0),
                    td: Some(td),
                    freq: None,
                    nl: None,
                    model: None,
                } if z0.to_bits() == 50.0f64.to_bits()
                    && td.to_bits() == 10e-9f64.to_bits()
            )
        {
            return Err("BUG 662 T_T1 topology or lossless-line parameters changed".into());
        }
        for (name, nodes) in [("R_R1", ["N14553", "N14950"]), ("R_R2", ["N15037", "0"])] {
            let resistor = element(name)?;
            let ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } = &resistor.kind
            else {
                return Err(format!("{name} is not a canonical scalar resistor"));
            };
            let mut params = instance_params
                .iter()
                .map(|(param, value)| (param.to_ascii_lowercase(), value.to_bits()))
                .collect::<Vec<_>>();
            params.sort();
            let mut expected_params = vec![
                ("r".to_string(), 50.0f64.to_bits()),
                ("tc".to_string(), 0.0f64.to_bits()),
                ("tc2".to_string(), 0.0f64.to_bits()),
            ];
            expected_params.sort();
            if resistor.nodes != nodes
                || value.to_bits() != 50.0f64.to_bits()
                || !deferred_params.is_empty()
                || params != expected_params
            {
                return Err(format!(
                    "{name} topology, value, or TC=0,0 parameters changed: nodes={:?} value={} params={params:?} deferred={deferred_params:?}",
                    resistor.nodes, value
                ));
            }
        }
        let source = element("V_V1")?;
        if source.nodes != ["N14553", "0"]
            || !matches!(
                source.kind,
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pulse {
                    v1,
                    v2,
                    delay,
                    rise,
                    fall,
                    width,
                    period,
                    phase,
                    width_defaults_to_zero: false,
                }) if v1.to_bits() == 0.0f64.to_bits()
                    && v2.to_bits() == 5.0f64.to_bits()
                    && delay.to_bits() == 0.0f64.to_bits()
                    && rise.to_bits() == 0.1e-9f64.to_bits()
                    && fall.to_bits() == 0.1e-9f64.to_bits()
                    && width.to_bits() == 5e-9f64.to_bits()
                    && period.to_bits() == 25e-9f64.to_bits()
                    && phase.to_bits() == 0.0f64.to_bits()
            )
        {
            return Err("BUG 662 V_V1 topology or PULSE waveform changed".into());
        }
        Ok(())
    }

    pub(super) fn validate_bug647_resistor_member(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        role: XyceBug647ResistorRole,
    ) -> Result<(), String> {
        Self::validate_bug647_resistor_statement_envelope(&plan.source)?;
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !Self::bug647_resistor_diagnostics_are_exact(&plan.diagnostics)
            || plan.steps.len() != 4
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || !plan.dc.source.eq_ignore_ascii_case("VIN")
            || plan.print.probes.len() != 6
        {
            return Err("BUG 647 resistor member requires four nested linear STEPs, one VIN 0:1:5 DC sweep, and one default six-probe PRN".into());
        }
        let dc_count = netlist
            .analyses
            .iter()
            .filter(|analysis| matches!(analysis, AnalysisCommand::Dc { .. }))
            .count();
        let step_count = netlist
            .analyses
            .iter()
            .filter(|analysis| matches!(analysis, AnalysisCommand::Step(_)))
            .count();
        if netlist.analyses.len() != 5
            || dc_count != 1
            || step_count != 4
            || netlist.elements.len() != 3
            || netlist.models.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !Self::bug647_resistor_diagnostics_are_exact(&netlist.diagnostics)
        {
            return Err("BUG 647 resistor member admits only R1, VIN, VMON, RMOD, one DC analysis, and four STEP analyses".into());
        }

        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if probes != ["v(1)", "i(vmon)", "r1:w", "r1:tc1", "r1:tc2", "r1:temp"] {
            return Err("BUG 647 resistor ordered probe contract changed".into());
        }

        let step = |index: usize,
                    target: StepTarget,
                    name: &str,
                    parameter: Option<&str>,
                    start: Value,
                    stop: Value,
                    increment: Value|
         -> Result<(), String> {
            let command = &plan.steps[index];
            let StepSweep::Linear {
                start: actual_start,
                stop: actual_stop,
                step: actual_step,
            } = command.sweep
            else {
                return Err(format!("STEP {} is not linear", index + 1));
            };
            if command.target != target
                || !command.name.eq_ignore_ascii_case(name)
                || command.param_name.as_deref().map(str::to_ascii_lowercase)
                    != parameter.map(str::to_ascii_lowercase)
                || actual_start.to_bits() != start.to_bits()
                || actual_stop.to_bits() != stop.to_bits()
                || actual_step.to_bits() != increment.to_bits()
            {
                return Err(format!(
                    "STEP {} target, order, or grid changed: actual={command:?}, expected target={target:?} name={name} parameter={parameter:?} grid=[{start}, {stop}, {increment}]",
                    index + 1
                ));
            }
            Ok(())
        };
        match role {
            XyceBug647ResistorRole::Owner => {
                step(
                    0,
                    StepTarget::Device,
                    "R1",
                    Some("W"),
                    1e-6,
                    5.0 * 1e-6,
                    1e-6,
                )?;
                step(1, StepTarget::Device, "R1", Some("TEMP"), 30.0, 35.0, 1.0)?;
                step(2, StepTarget::Device, "R1", Some("TC1"), 1e-2, 3e-2, 1e-2)?;
                step(3, StepTarget::Device, "R1", Some("TC2"), 1e-4, 3e-4, 1e-4)?;
            }
            XyceBug647ResistorRole::ModelParameterReference => {
                step(
                    0,
                    StepTarget::Device,
                    "RMOD",
                    Some("DEFW"),
                    1e-6,
                    5.0 * 1e-6,
                    1e-6,
                )?;
                step(1, StepTarget::Temp, "TEMP", None, 30.0, 35.0, 1.0)?;
                step(2, StepTarget::Device, "RMOD", Some("TC1"), 1e-2, 3e-2, 1e-2)?;
                step(3, StepTarget::Device, "RMOD", Some("TC2"), 1e-4, 3e-4, 1e-4)?;
            }
        }

        let resistor = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R1"))
            .ok_or_else(|| "BUG 647 resistor member has no R1".to_string())?;
        if resistor.nodes != ["2", "0"] {
            return Err("R1 topology changed".into());
        }
        let ElementKind::Resistor {
            value,
            model: Some(model),
            instance_params,
            deferred_params,
            value_expr: None,
            ..
        } = &resistor.kind
        else {
            return Err("R1 must be a scalar modeled resistor".into());
        };
        if !model.eq_ignore_ascii_case("RMOD") || !deferred_params.is_empty() {
            return Err("R1 model binding or deferred parameters changed".into());
        }
        let marker_entries = instance_params
            .iter()
            .filter(|(name, _)| {
                name.eq_ignore_ascii_case(rspice_core::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER)
            })
            .collect::<Vec<_>>();
        if value.to_bits() != 0.0f64.to_bits()
            || marker_entries.len() != 1
            || marker_entries[0].1.to_bits() != 1.0f64.to_bits()
        {
            return Err(
                "R1 must retain the exact value-less Xyce modeled-resistor representation".into(),
            );
        }
        let mut params = instance_params
            .iter()
            .filter(|(name, _)| {
                !name.eq_ignore_ascii_case(rspice_core::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER)
            })
            .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
            .collect::<Vec<_>>();
        params.sort();
        let mut expected_instance = match role {
            XyceBug647ResistorRole::Owner => vec![
                ("l".to_string(), 1e-3f64.to_bits()),
                ("tc1".to_string(), 1e-2f64.to_bits()),
                ("tc2".to_string(), 1e-4f64.to_bits()),
                ("w".to_string(), 1e-6f64.to_bits()),
            ],
            XyceBug647ResistorRole::ModelParameterReference => {
                vec![("l".to_string(), 1e-3f64.to_bits())]
            }
        };
        expected_instance.sort();
        if params != expected_instance {
            return Err(format!(
                "R1 instance parameter representation changed: {params:?}"
            ));
        }

        for (name, nodes, expected) in [("VIN", ["1", "0"], 5.0f64), ("VMON", ["1", "2"], 0.0f64)] {
            let source = netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("missing {name}"))?;
            if source.nodes != nodes
                || !matches!(
                    source.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                        if value.to_bits() == expected.to_bits()
                )
            {
                return Err(format!("{name} topology or DC value changed"));
            }
        }

        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("RMOD")
            || !matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "R" | "RES" | "RESISTOR"
            )
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err("RMOD must remain one native scalar resistor model".into());
        }
        let mut model_params = model
            .params
            .iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
            .collect::<Vec<_>>();
        model_params.sort();
        let mut expected_model = match role {
            XyceBug647ResistorRole::Owner => vec![
                ("rsh".to_string(), 1.0f64.to_bits()),
                ("tnom".to_string(), 27.0f64.to_bits()),
                ("w".to_string(), 1e-6f64.to_bits()),
            ],
            XyceBug647ResistorRole::ModelParameterReference => vec![
                ("defw".to_string(), 1e-6f64.to_bits()),
                ("rsh".to_string(), 1.0f64.to_bits()),
                ("tc1".to_string(), 1e-2f64.to_bits()),
                ("tc2".to_string(), 1e-4f64.to_bits()),
                ("tnom".to_string(), 27.0f64.to_bits()),
            ],
        };
        expected_model.sort();
        if model_params != expected_model {
            return Err(format!(
                "RMOD parameter representation changed: {model_params:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug647_resistor_statement_envelope(source: &str) -> Result<(), String> {
        let body = source.split_once('\n').map_or("", |(_, body)| body);
        let mut counts = BTreeMap::<String, usize>::new();
        for line in Self::logical_netlist_lines(body) {
            let statement = Self::strip_netlist_comment(&line).trim();
            if statement.is_empty() {
                continue;
            }
            let head = statement
                .split_ascii_whitespace()
                .next()
                .unwrap_or_default()
                .to_ascii_lowercase();
            let key = if head.starts_with('.') {
                match head.as_str() {
                    ".dc" | ".model" | ".print" | ".step" | ".end" => head,
                    _ => return Err(format!("unrelated directive '{head}' in BUG 647 pair")),
                }
            } else {
                match head.as_bytes().first().map(u8::to_ascii_lowercase) {
                    Some(b'r') => "r".to_string(),
                    Some(b'v') => "v".to_string(),
                    _ => return Err(format!("unrelated element '{head}' in BUG 647 pair")),
                }
            };
            *counts.entry(key).or_default() += 1;
        }
        for (key, expected) in [
            ("r", 1),
            ("v", 2),
            (".dc", 1),
            (".model", 1),
            (".print", 1),
            (".step", 4),
            (".end", 1),
        ] {
            if counts.remove(key) != Some(expected) {
                return Err(format!(
                    "BUG 647 statement count for '{key}' must be {expected}"
                ));
            }
        }
        if !counts.is_empty() {
            return Err(format!("BUG 647 source has extra statements: {counts:?}"));
        }
        Ok(())
    }

    pub(super) fn validate_resistor_dtemp_statement_envelope(
        source: &str,
        role: XyceResistorDtempRole,
    ) -> Result<(), String> {
        let body = source.split_once('\n').map_or("", |(_, body)| body);
        let mut counts = BTreeMap::<String, usize>::new();
        for line in Self::logical_netlist_lines(body) {
            let statement = Self::strip_netlist_comment(&line).trim();
            if statement.is_empty() {
                continue;
            }
            let head = statement
                .split_ascii_whitespace()
                .next()
                .unwrap_or_default()
                .to_ascii_lowercase();
            let key = if head.starts_with('.') {
                match head.as_str() {
                    ".param" | ".options" | ".dc" | ".model" | ".print" | ".step" | ".end" => head,
                    _ => {
                        return Err(format!(
                            "unrelated directive '{head}' is outside the resistor DTEMP envelope"
                        ));
                    }
                }
            } else {
                match head.as_bytes().first().map(u8::to_ascii_lowercase) {
                    Some(b'r') => "r".to_string(),
                    Some(b'v') => "v".to_string(),
                    _ => {
                        return Err(format!(
                            "unrelated element '{head}' is outside the resistor DTEMP envelope"
                        ));
                    }
                }
            };
            *counts.entry(key).or_default() += 1;
        }
        for key in ["r", "v", ".dc", ".model", ".print", ".step", ".end"] {
            if counts.remove(key) != Some(1) {
                return Err(format!(
                    "resistor DTEMP statement count for '{key}' must be one"
                ));
            }
        }
        match role {
            XyceResistorDtempRole::Owner => {
                if counts.remove(".param") != Some(1) || counts.remove(".options") != Some(1) {
                    return Err(
                        "DTEMP owner requires exactly one .PARAM and one .OPTIONS statement"
                            .to_string(),
                    );
                }
            }
            XyceResistorDtempRole::Reference => {
                if counts.contains_key(".param") || counts.contains_key(".options") {
                    return Err(
                        "TEMP reference must not carry .PARAM or .OPTIONS state".to_string()
                    );
                }
            }
        }
        if !counts.is_empty() {
            return Err(format!(
                "resistor DTEMP source contains extra statements: {counts:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn passive_primary_value_composite_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XycePassivePrimaryCompositeContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/") {
            return None;
        }
        let parent = deck.path.parent()?;
        let directory_entries = fs::read_dir(parent)
            .ok()?
            .collect::<Result<Vec<_>, _>>()
            .ok()?;
        let mut cir_paths = Vec::new();
        for entry in directory_entries {
            let path = entry.path();
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            if !entry.file_type().ok()?.is_file() {
                return None;
            }
            cir_paths.push(path);
        }
        if cir_paths.len() != 5
            || !cir_paths
                .iter()
                .any(|path| Self::same_path(path, &deck.path))
        {
            return None;
        }
        cir_paths.sort_by(|left, right| {
            left.file_name()
                .unwrap_or_default()
                .cmp(right.file_name().unwrap_or_default())
        });

        let mut paths_by_stem = BTreeMap::new();
        for path in &cir_paths {
            let stem = path.file_stem()?.to_str()?.to_ascii_lowercase();
            if stem.is_empty() || paths_by_stem.insert(stem, path.clone()).is_some() {
                return None;
            }
        }

        let mut owner_paths = Vec::new();
        let mut member_paths = Vec::new();
        for path in &cir_paths {
            let metadata = fs::metadata(path).ok()?;
            if metadata.len() == 0 {
                owner_paths.push(path.clone());
            } else {
                member_paths.push(path.clone());
            }
        }
        let [owner_path] = owner_paths.as_slice() else {
            return None;
        };
        if member_paths.len() != 4
            || !self.requires_upstream_wrapper(&self.relative_key(owner_path))
            || member_paths
                .iter()
                .any(|path| self.requires_upstream_wrapper(&self.relative_key(path)))
        {
            return None;
        }

        let member_stems = member_paths
            .iter()
            .map(|path| path.file_stem()?.to_str().map(str::to_ascii_lowercase))
            .collect::<Option<BTreeSet<_>>>()?;
        let target_stems = member_stems
            .iter()
            .filter(|stem| stem.ends_with("-bug"))
            .cloned()
            .collect::<Vec<_>>();
        if target_stems.len() != 2 {
            return None;
        }

        let mut pair_contracts = Vec::new();
        let mut consumed_stems = BTreeSet::new();
        for target_stem in target_stems {
            let baseline_stem = target_stem.strip_suffix("-bug")?;
            if baseline_stem.is_empty()
                || baseline_stem.ends_with("-bug")
                || !member_stems.contains(baseline_stem)
                || !consumed_stems.insert(baseline_stem.to_string())
                || !consumed_stems.insert(target_stem.clone())
            {
                return None;
            }
            let baseline_path = paths_by_stem.get(baseline_stem)?.clone();
            let target_path = paths_by_stem.get(&target_stem)?.clone();
            let baseline_analysis = self
                .baseline_family_analysis_for_path(&baseline_path)
                .ok()?;
            let target_analysis = self.baseline_family_analysis_for_path(&target_path).ok()?;
            if baseline_analysis != target_analysis {
                return None;
            }
            pair_contracts.push((
                baseline_analysis,
                baseline_stem.to_string(),
                baseline_path,
                target_path,
            ));
        }
        if consumed_stems != member_stems {
            return None;
        }

        let target_path = (!Self::same_path(&deck.path, owner_path)).then(|| deck.path.clone());
        let mut capacitor_tran = None;
        let mut resistor_dc = None;
        for (analysis, pair_family, baseline_path, positional_path) in pair_contracts {
            let (kind, slot) = match analysis {
                XyceBaselineFamilyAnalysis::Ac => return None,
                XyceBaselineFamilyAnalysis::Tran => (
                    XyceBaselineFamilyKind::PassiveCapPrimaryValue,
                    &mut capacitor_tran,
                ),
                XyceBaselineFamilyAnalysis::Dc => (
                    XyceBaselineFamilyKind::PassiveResPrimaryValue,
                    &mut resistor_dc,
                ),
            };
            if slot.is_some() {
                return None;
            }
            let pair_target_path = target_path.as_ref().and_then(|target| {
                (Self::same_path(target, &baseline_path)
                    || Self::same_path(target, &positional_path))
                .then(|| target.clone())
            });
            *slot = Some(XyceBaselineFamilyContract {
                kind,
                comparison: XyceBaselineFamilyComparison::ExactPrn,
                family: pair_family,
                baseline_path: baseline_path.clone(),
                member_paths: vec![baseline_path, positional_path],
                target_path: pair_target_path,
            });
        }

        Some(XycePassivePrimaryCompositeContract {
            family: owner_path.file_stem()?.to_str()?.to_string(),
            owner_path: owner_path.clone(),
            capacitor_tran: capacitor_tran?,
            resistor_dc: resistor_dc?,
            target_path,
        })
    }

    pub(super) fn passive_temperature_override_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/") || self.requires_upstream_wrapper(&relative_path)
        {
            return None;
        }

        let parent = deck.path.parent()?;
        let target_stem = deck.path.file_stem()?.to_str()?;
        const TARGET_SUFFIX: &str = "_instance";
        if target_stem.len() <= TARGET_SUFFIX.len()
            || !target_stem[target_stem.len() - TARGET_SUFFIX.len()..]
                .eq_ignore_ascii_case(TARGET_SUFFIX)
        {
            return None;
        }
        let family = &target_stem[..target_stem.len() - TARGET_SUFFIX.len()];
        if family.is_empty()
            || !family
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            return None;
        }

        let baseline_path = parent.join(format!("{family}.cir"));
        let target_path = parent.join(format!("{family}_instance.cir"));
        if !Self::same_path(&deck.path, &target_path)
            || [&baseline_path, &target_path].iter().any(|path| {
                fs::metadata(path)
                    .ok()
                    .is_none_or(|metadata| !metadata.is_file() || metadata.len() == 0)
            })
        {
            return None;
        }

        let baseline_relative = self.relative_key(&baseline_path);
        if self.requires_upstream_wrapper(&baseline_relative)
            || self
                .static_prn_reference_path(&baseline_path)
                .is_none_or(|path| !path.is_file())
            || self
                .static_prn_reference_path(&target_path)
                .is_some_and(|path| path.is_file())
        {
            return None;
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::PassiveTemperatureOverride,
            comparison: XyceBaselineFamilyComparison::Exact,
            family: family.to_string(),
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path, target_path.clone()],
            target_path: Some(target_path),
        })
    }

    pub(super) fn sin_expression_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }

        let file_name = deck.path.file_name()?.to_str()?;
        let parent = deck.path.parent()?;
        if self.requires_upstream_wrapper(&deck.relative_path)
            && fs::metadata(&deck.path)
                .ok()
                .is_some_and(|metadata| metadata.len() == 0)
        {
            let family = file_name.strip_suffix(".cir")?;
            if family.is_empty() {
                return None;
            }
            return self.sin_expression_family_contract_for(parent, family, None);
        }

        let file_name_lower = file_name.to_ascii_lowercase();
        let suffix = if file_name_lower.ends_with("_vsrc.cir") {
            "_vsrc.cir"
        } else if file_name_lower.ends_with("_expr.cir") {
            "_expr.cir"
        } else {
            return None;
        };
        let family = file_name.get(..file_name.len().checked_sub(suffix.len())?)?;
        if family.is_empty() {
            return None;
        }
        self.sin_expression_family_contract_for(parent, family, Some(deck.path.clone()))
    }

    pub(super) fn sin_expression_family_contract_for(
        &self,
        parent: &Path,
        family: &str,
        target_path: Option<PathBuf>,
    ) -> Option<XyceBaselineFamilyContract> {
        let owner_path = parent.join(format!("{family}.cir"));
        let baseline_path = parent.join(format!("{family}_vsrc.cir"));
        let expression_path = parent.join(format!("{family}_expr.cir"));
        let variant_prefix = format!("{}_", family.to_ascii_lowercase());
        let variant_suffixes = fs::read_dir(parent)
            .ok()?
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                if !path.is_file()
                    || !path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                {
                    return None;
                }
                let stem = path.file_stem()?.to_str()?.to_ascii_lowercase();
                stem.strip_prefix(&variant_prefix).map(ToString::to_string)
            })
            .collect::<BTreeSet<_>>();
        let required_suffixes = ["vsrc".to_string(), "expr".to_string()]
            .into_iter()
            .collect::<BTreeSet<_>>();
        if !owner_path.is_file()
            || !baseline_path.is_file()
            || !expression_path.is_file()
            || variant_suffixes != required_suffixes
            || fs::metadata(&owner_path)
                .ok()
                .is_none_or(|metadata| metadata.len() != 0)
        {
            return None;
        }

        let owner_relative = self.relative_key(&owner_path);
        let baseline_relative = self.relative_key(&baseline_path);
        let expression_relative = self.relative_key(&expression_path);
        if !self.requires_upstream_wrapper(&owner_relative)
            || self.requires_upstream_wrapper(&baseline_relative)
            || self.requires_upstream_wrapper(&expression_relative)
        {
            return None;
        }
        if let Some(target_path) = target_path.as_ref()
            && !Self::same_path(target_path, &baseline_path)
            && !Self::same_path(target_path, &expression_path)
        {
            return None;
        }

        // The upstream sidecar invokes the generic toleranced verifier, while
        // the behavioral member explicitly states that it must match the
        // independent-source baseline exactly. RSpice enforces that stronger
        // invariant only after the source forms, topology, analysis tuple, and
        // output contract have all qualified through the bounded checks above.
        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::SinExpression,
            comparison: XyceBaselineFamilyComparison::Exact,
            family: family.to_string(),
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path, expression_path],
            target_path,
        })
    }

    pub(super) fn bjt_external_node_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/bjt_extnode/") {
            return None;
        }

        let file_name = deck.path.file_name()?.to_str()?;
        let parent = deck.path.parent()?;
        if self.requires_upstream_wrapper(&deck.relative_path)
            && fs::metadata(&deck.path)
                .ok()
                .is_some_and(|metadata| metadata.len() == 0)
        {
            let family = file_name.strip_suffix(".cir")?;
            if family.is_empty() || family.chars().last().is_some_and(|ch| ch.is_ascii_digit()) {
                return None;
            }
            return self.bjt_external_node_family_contract_for(parent, family, None);
        }

        let (family, member_index) = Self::parse_bjt_external_node_member_file_name(file_name)?;
        if !matches!(member_index, 1 | 2) {
            return None;
        }
        self.bjt_external_node_family_contract_for(parent, &family, Some(deck.path.clone()))
    }

    pub(super) fn bjt_external_node_family_contract_for(
        &self,
        parent: &Path,
        family: &str,
        target_path: Option<PathBuf>,
    ) -> Option<XyceBaselineFamilyContract> {
        let owner_path = parent.join(format!("{family}.cir"));
        let baseline_path = parent.join(format!("{family}1.cir"));
        let explicit_path = parent.join(format!("{family}2.cir"));
        let family_lower = family.to_ascii_lowercase();
        let numbered_suffixes = fs::read_dir(parent)
            .ok()?
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                if !path.is_file()
                    || !path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                {
                    return None;
                }
                let stem = path.file_stem()?.to_str()?;
                let stem_lower = stem.to_ascii_lowercase();
                let suffix = stem_lower.strip_prefix(&family_lower)?.to_string();
                (!suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit()))
                    .then_some(suffix)
            })
            .collect::<BTreeSet<_>>();
        let required_suffixes = ["1".to_string(), "2".to_string()]
            .into_iter()
            .collect::<BTreeSet<_>>();
        if !owner_path.is_file()
            || !baseline_path.is_file()
            || !explicit_path.is_file()
            || numbered_suffixes != required_suffixes
            || fs::metadata(&owner_path)
                .ok()
                .is_none_or(|metadata| metadata.len() != 0)
        {
            return None;
        }
        let owner_relative = self.relative_key(&owner_path);
        let baseline_relative = self.relative_key(&baseline_path);
        let explicit_relative = self.relative_key(&explicit_path);
        if !self.requires_upstream_wrapper(&owner_relative)
            || self.requires_upstream_wrapper(&baseline_relative)
            || self.requires_upstream_wrapper(&explicit_relative)
        {
            return None;
        }
        if let Some(target_path) = target_path.as_ref()
            && !Self::same_path(target_path, &baseline_path)
            && !Self::same_path(target_path, &explicit_path)
        {
            return None;
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::BjtExternalNode,
            comparison: XyceBaselineFamilyComparison::Exact,
            family: family.to_string(),
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path, explicit_path],
            target_path,
        })
    }

    pub(super) fn scoped_model_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }

        let file_name = deck.path.file_name()?.to_str()?;
        let parent = deck.path.parent()?;
        let (owner_name, baseline_name) = if let Some(stem) = file_name.strip_suffix("_noscope.cir")
        {
            (format!("{stem}.cir"), file_name.to_string())
        } else {
            let stem = file_name.strip_suffix(".cir")?;
            (file_name.to_string(), format!("{stem}_noscope.cir"))
        };
        let owner_path = parent.join(&owner_name);
        let baseline_path = parent.join(&baseline_name);
        if !owner_path.is_file()
            || !baseline_path.is_file()
            || (!Self::same_path(&deck.path, &owner_path)
                && !Self::same_path(&deck.path, &baseline_path))
        {
            return None;
        }
        let owner_relative = self.relative_key(&owner_path);
        let baseline_relative = self.relative_key(&baseline_path);
        if !self.requires_upstream_wrapper(&owner_relative)
            || self.requires_upstream_wrapper(&baseline_relative)
        {
            return None;
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::ScopedModel,
            comparison: XyceBaselineFamilyComparison::Exact,
            family: owner_name.trim_end_matches(".cir").to_string(),
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path, owner_path],
            target_path: Some(deck.path.clone()),
        })
    }

    pub(super) fn subckt_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/subckt/") {
            return None;
        }

        let file_name = deck.path.file_name()?.to_str()?;
        let parent = deck.path.parent()?;
        if self.requires_upstream_wrapper(&deck.relative_path)
            && let Some(family) = Self::parse_subckt_wrapper_file_name(file_name)
            && fs::metadata(&deck.path)
                .ok()
                .is_some_and(|metadata| metadata.len() == 0)
        {
            return self.subckt_family_contract_for(parent, &family, None);
        }

        let family = Self::parse_subckt_family_member_file_name(file_name)?;
        let wrapper_relative = format!("Netlists/SUBCKT/subckt_{family}.cir");
        if !self.requires_upstream_wrapper(&wrapper_relative) {
            return None;
        }
        self.subckt_family_contract_for(parent, &family, Some(deck.path.clone()))
    }

    pub(super) fn subckt_family_contract_for(
        &self,
        parent: &Path,
        family: &str,
        target_path: Option<PathBuf>,
    ) -> Option<XyceBaselineFamilyContract> {
        let mut member_paths = Vec::new();
        for entry in fs::read_dir(parent).ok()?.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file_name = path.file_name().and_then(|name| name.to_str())?;
            if Self::parse_subckt_family_member_file_name(file_name)
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(family))
            {
                member_paths.push(path);
            }
        }
        member_paths.sort_by(|left, right| {
            left.file_name()
                .unwrap_or_default()
                .cmp(right.file_name().unwrap_or_default())
        });

        let baseline_path = parent.join(format!("subckt_{family}0.cir"));
        if !member_paths
            .iter()
            .any(|member| Self::same_path(member, &baseline_path))
        {
            return None;
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::Subckt,
            comparison: XyceBaselineFamilyComparison::Toleranced,
            family: family.to_string(),
            baseline_path,
            member_paths,
            target_path,
        })
    }

    pub(super) fn supernode_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/supernode/") {
            return None;
        }

        let file_name = deck.path.file_name()?.to_str()?;
        let parent = deck.path.parent()?;
        if file_name.eq_ignore_ascii_case("supernode1.cir")
            && self.requires_upstream_wrapper(&deck.relative_path)
        {
            return self.supernode1_family_contract_for(parent, None);
        }

        if matches!(
            file_name.to_ascii_lowercase().as_str(),
            "supernode1a.cir" | "supernode1b.cir"
        ) {
            let wrapper_relative = "Netlists/SUPERNODE/supernode1.cir";
            if self.requires_upstream_wrapper(wrapper_relative) {
                return self.supernode1_family_contract_for(parent, Some(deck.path.clone()));
            }
        }

        None
    }

    pub(super) fn supernode1_family_contract_for(
        &self,
        parent: &Path,
        target_path: Option<PathBuf>,
    ) -> Option<XyceBaselineFamilyContract> {
        let mut member_paths = Vec::new();
        for file_name in ["supernode1.cir", "supernode1a.cir", "supernode1b.cir"] {
            let path = parent.join(file_name);
            if !path.is_file() {
                return None;
            }
            member_paths.push(path);
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::Supernode,
            comparison: XyceBaselineFamilyComparison::Toleranced,
            family: "supernode1".to_string(),
            baseline_path: parent.join("supernode1.cir"),
            member_paths,
            target_path,
        })
    }
}
