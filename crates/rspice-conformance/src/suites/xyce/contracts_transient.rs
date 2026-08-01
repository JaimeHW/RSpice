//! Transient-analysis deck contracts.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn validate_measure_cont_step_tran_oracle(
        &self,
        deck: &XyceDeck,
        member: XyceMeasureContStepTranMember,
        start: Instant,
    ) -> Result<(), String> {
        let sources = self.validate_measure_cont_step_tran_provenance(deck, member)?;
        self.check_measure_cont_tran_deadline(start, "stepped provenance")?;

        let mut parsed = Vec::with_capacity(sources.len());
        for (candidate, bytes) in sources {
            let source = std::str::from_utf8(&bytes)
                .map_err(|error| format!("MEASURE_CONT STEP source is not UTF-8: {error}"))?
                .to_string();
            let path = self.root.join(candidate.source_relative_path());
            let netlist = Self::parse_xyce_netlist(&source, &path).map_err(|error| {
                format!(
                    "MEASURE_CONT STEP parse failed for {}: {error}",
                    candidate.source_relative_path()
                )
            })?;
            let output = Self::single_tran_print_output_request(&source)?;
            if output.file.is_some()
                || output
                    .format
                    .as_deref()
                    .is_some_and(|format| !Self::tran_print_format_is_prn_compatible(format))
            {
                return Err(format!(
                    "{} requires one ordinary default-format .PRINT TRAN",
                    candidate.source_relative_path()
                ));
            }
            let print = XycePrintRequest {
                probes: output.probes,
            };
            let tran = Self::single_tran_analysis(&netlist)?;
            Self::validate_measure_cont_step_tran_plan(&netlist, &print, &tran, candidate)?;
            parsed.push((candidate, netlist, tran));
        }

        let main_member = XyceMeasureContStepTranMember::main(member.kind);
        let main_index = parsed
            .iter()
            .position(|(candidate, ..)| *candidate == main_member)
            .ok_or_else(|| "MEASURE_CONT STEP family omitted its owner".to_string())?;
        let main_measurements = format!("{:#?}", parsed[main_index].1.measurements);
        for index in 0..2 {
            let control = XyceMeasureContStepTranMember::control(member.kind, index)
                .ok_or_else(|| "invalid MEASURE_CONT STEP control index".to_string())?;
            let control_netlist = &parsed
                .iter()
                .find(|(candidate, ..)| *candidate == control)
                .ok_or_else(|| format!("MEASURE_CONT STEP family omitted control {index}"))?
                .1;
            if format!("{:#?}", control_netlist.measurements) != main_measurements {
                return Err(format!(
                    "MEASURE_CONT STEP control {index} measurement AST/order differs from its owner"
                ));
            }
        }

        let (_, main_netlist, main_tran) = &parsed[main_index];
        let engine = self
            .create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(main_tran));
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let steps = Self::step_commands(main_netlist)?;
        let materialized = Self::nested_step_runs_for_commands_with_limits_and_abort(
            &engine,
            main_netlist,
            &steps,
            xyce_step_plan_limits(),
            &abort,
        )
        .map_err(|error| format!("MEASURE_CONT STEP expansion failed: {error}"))?;
        if materialized.len() != 2
            || materialized[0].step_values != [150.0]
            || materialized[1].step_values != [200.0]
        {
            return Err(format!(
                "MEASURE_CONT STEP owner produced unexpected ordered step values: {:?}",
                materialized
                    .iter()
                    .map(|run| &run.step_values)
                    .collect::<Vec<_>>()
            ));
        }

        let mut owner_runs = Vec::with_capacity(2);
        for run in materialized {
            let tran = Self::single_tran_analysis(&run.netlist)?;
            owner_runs.push(self.evaluate_measure_cont_step_tran_run(
                run.netlist,
                run.step_values,
                &tran,
                start,
            )?);
        }

        for (index, owner) in owner_runs.iter().enumerate() {
            let control_member = XyceMeasureContStepTranMember::control(member.kind, index)
                .ok_or_else(|| "invalid MEASURE_CONT STEP control index".to_string())?;
            let (_, control_netlist, control_tran) = parsed
                .iter()
                .find(|(candidate, ..)| *candidate == control_member)
                .ok_or_else(|| format!("MEASURE_CONT STEP family omitted control {index}"))?;
            let control = self.evaluate_measure_cont_step_tran_run(
                control_netlist.clone(),
                vec![150.0 + 50.0 * index as Value],
                control_tran,
                start,
            )?;
            Self::compare_measure_cont_step_waveforms(index, owner, &control)?;
            Self::compare_measure_cont_step_measurements(index, owner, &control)?;
            Self::validate_measure_cont_remeasure(
                &owner.netlist,
                &owner.transient,
                None,
                XyceFileCompareTolerance::MEASURE_CONT_STEP_REMEASURE,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
        }

        self.check_measure_cont_tran_deadline(
            start,
            "stepped execution, relational comparison, and remeasure",
        )
    }

    pub(super) fn validate_measure_cont_step_tran_plan(
        netlist: &Netlist,
        print: &XycePrintRequest,
        tran: &XyceTranAnalysis,
        member: XyceMeasureContStepTranMember,
    ) -> Result<(), String> {
        let probes = print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        let steps = Self::step_commands(netlist)?;
        let exact_step = matches!(steps.as_slice(), [StepCommand {
            target: StepTarget::Device,
            name,
            param_name: None,
            sweep: StepSweep::Linear { start, stop, step },
        }] if name.eq_ignore_ascii_case("R1b")
            && start.to_bits() == 150.0f64.to_bits()
            && stop.to_bits() == 200.0f64.to_bits()
            && step.to_bits() == 50.0f64.to_bits());
        if (member.role == XyceMeasureContStepTranRole::Main) != exact_step
            || (member.role != XyceMeasureContStepTranRole::Main && !steps.is_empty())
        {
            return Err(format!(
                "MEASURE_CONT STEP exact owner/control .STEP contract changed: {steps:?}"
            ));
        }
        let expected_analyses = if member.role == XyceMeasureContStepTranRole::Main {
            2
        } else {
            1
        };
        if tran.step.to_bits() != 0.0f64.to_bits()
            || tran.stop.to_bits() != 0.01f64.to_bits()
            || tran.start.is_some()
            || tran.max_step.is_some()
            || tran.uic
            || probes != ["v(1)", "v(1a)", "v(2)"]
            || netlist.analyses.len() != expected_analyses
            || netlist.output_requests.len() != netlist.measurements.len() + 1
            || !netlist.diagnostics.is_empty()
            || netlist.options.measure_use_cont_files != Some(false)
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || netlist.elements.len() != 5
        {
            return Err(format!(
                "MEASURE_CONT STEP exact TRAN/PRINT/topology contract changed for {}",
                member.source_relative_path()
            ));
        }
        let (expected_scalar, expected_continuous) = member.expected_measurement_counts();
        let scalar = netlist
            .measurements
            .iter()
            .filter(|statement| statement.analysis.eq_ignore_ascii_case("TRAN"))
            .count();
        let continuous = netlist
            .measurements
            .iter()
            .filter(|statement| statement.analysis.eq_ignore_ascii_case("TRAN_CONT"))
            .count();
        if scalar != expected_scalar
            || continuous != expected_continuous
            || scalar + continuous != netlist.measurements.len()
            || netlist.measurements.iter().any(|statement| {
                statement.print_policy != rspice_core::analysis::MeasurePrintPolicy::All
            })
        {
            return Err(format!(
                "MEASURE_CONT STEP measurement census changed: TRAN={scalar}/{expected_scalar}, TRAN_CONT={continuous}/{expected_continuous}"
            ));
        }
        let expected_vpwl1 = if member.kind == XyceMeasureContStepTranKind::Derivative {
            &[
                (0.0, 0.1),
                (0.0025, 0.5),
                (0.005, 0.0),
                (0.0074, 0.4),
                (0.01, 0.0),
            ][..]
        } else {
            &[
                (0.0, 0.1),
                (0.0025, 0.5),
                (0.005, 0.0),
                (0.0075, 0.4),
                (0.01, 0.0),
            ][..]
        };
        Self::validate_measure_cont_pwl(netlist, "VPWL1", ["1", "0"], expected_vpwl1)?;
        Self::validate_measure_cont_pwl(netlist, "VPWL2", ["2", "0"], &[(0.0, 0.5), (0.01, 0.0)])?;
        let r1b = match member.role {
            XyceMeasureContStepTranRole::Main => 100.0,
            XyceMeasureContStepTranRole::Control0 => 150.0,
            XyceMeasureContStepTranRole::Control1 => 200.0,
        };
        for (name, nodes, expected) in [
            ("R1a", ["1", "1a"], 100.0),
            ("R1b", ["1a", "0"], r1b),
            ("R2", ["2", "0"], 100.0),
        ] as [(&str, [&str; 2], Value); 3]
        {
            let resistor = netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("MEASURE_CONT STEP is missing {name}"))?;
            if resistor.nodes.len() != nodes.len()
                || resistor
                    .nodes
                    .iter()
                    .zip(nodes)
                    .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
                || !matches!(&resistor.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                    if value.to_bits() == expected.to_bits()
                        && instance_params.is_empty()
                        && deferred_params.is_empty())
            {
                return Err(format!(
                    "MEASURE_CONT STEP resistor {name} changed: {resistor:?}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_tran_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceMeasureContTranKind,
        start: Instant,
    ) -> Result<(), String> {
        let source_bytes = self.validate_measure_cont_tran_provenance(deck, kind)?;
        self.check_measure_cont_tran_deadline(start, "provenance")?;
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("MEASURE_CONT source is not UTF-8: {error}"))?
            .to_string();
        let netlist = Self::parse_xyce_netlist(&source, &deck.path)
            .map_err(|error| format!("MEASURE_CONT parse failed: {error}"))?;
        let print_output = Self::single_tran_print_output_request(&source)?;
        if print_output.file.is_some()
            || print_output
                .format
                .as_deref()
                .is_some_and(|format| !Self::tran_print_format_is_prn_compatible(format))
        {
            return Err("MEASURE_CONT requires one ordinary default-format .PRINT TRAN".into());
        }
        let print = XycePrintRequest {
            probes: print_output.probes,
        };
        let tran = Self::single_tran_analysis(&netlist)?;
        Self::validate_measure_cont_tran_plan(&netlist, &print, &tran, kind)?;

        let reference_path = kind
            .prn()
            .map(|(path, _)| self.root.join(path))
            .unwrap_or_else(|| self.root.join(kind.mt0_relative_path()));
        let plan = XyceStaticTranPlan {
            deck_path: deck.path.clone(),
            reference_path,
            source,
            print,
            output_override: false,
            timeint_conststep: false,
            tran,
            steps: Vec::new(),
            wrapper_tolerance: None,
            contract: XyceStaticTranContract::WrapperStatic,
            comparison_mode: XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision: XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            },
        };
        let result = self
            .run_transient_family_netlist(&plan, &netlist, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!(
                    "MEASURE_CONT native execution exceeded timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                other => format!("MEASURE_CONT native execution failed: {other}"),
            })?;
        self.check_measure_cont_tran_deadline(start, "native execution")?;

        let scalar = rspice_core::analysis::evaluate_tran_measurements(&netlist, &result);
        let continuous =
            rspice_core::analysis::evaluate_tran_continuous_measurements(&netlist, &result);
        let mt0_path = self.root.join(kind.mt0_relative_path());
        let mismatches = self.compare_analysis_measurement_outputs(
            std::slice::from_ref(&mt0_path),
            &[],
            &scalar,
            &continuous,
            XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
            netlist.options.measure_fail_output,
            netlist.options.measure_default_value,
            false,
            &netlist.measurements,
            "TRAN",
            "TRAN_CONT",
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "MEASURE_CONT aggregate mt0 comparison produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ));
        }

        self.validate_measure_cont_gs_semantics(kind, &netlist, &continuous)?;
        self.validate_measure_cont_counterfactual(kind, &netlist, &scalar, &continuous, &mt0_path)?;

        if let Some((relative, _)) = kind.prn() {
            let reference = Self::parse_xyce_verify_tran_reference_file(&self.root.join(relative))?;
            let actual = Self::measure_cont_prn_table_on_reference_grid(
                &plan, &netlist, &result, &reference,
            )?;
            let mismatches = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
                &reference,
                &actual,
                XyceVerifyTransientTolerance::release_7_10_default(),
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "MEASURE_CONT Release 7.10 PRN comparison produced {} mismatch(es): {mismatches:?}",
                    mismatches.len()
                ));
            }
            self.validate_measure_cont_prn_counterfactual(&reference, &actual, &plan, &netlist)?;
            Self::validate_measure_cont_remeasure(
                &netlist,
                &result,
                Some(&actual),
                XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
        } else {
            Self::validate_measure_cont_remeasure(
                &netlist,
                &result,
                None,
                XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
        }
        self.check_measure_cont_tran_deadline(
            start,
            "measurement, PRN, remeasure, and counterfactual validation",
        )
    }

    pub(super) fn validate_measure_cont_tran_plan(
        netlist: &Netlist,
        print: &XycePrintRequest,
        tran: &XyceTranAnalysis,
        kind: XyceMeasureContTranKind,
    ) -> Result<(), String> {
        let probes = print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if tran.step.to_bits() != 0.0f64.to_bits()
            || tran.stop.to_bits() != 0.01f64.to_bits()
            || tran.start.is_some()
            || tran.max_step.is_some()
            || tran.uic
            || probes != kind.expected_print_probes()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != netlist.measurements.len() + 1
            || !netlist.diagnostics.is_empty()
            || netlist.options.measure_use_cont_files != Some(false)
        {
            return Err(format!(
                "MEASURE_CONT exact TRAN/PRINT/options contract changed: tran={tran:?}, probes={probes:?}, analyses={}, outputs={}, diagnostics={}, USE_CONT_FILES={:?}",
                netlist.analyses.len(),
                netlist.output_requests.len(),
                netlist.diagnostics.len(),
                netlist.options.measure_use_cont_files
            ));
        }
        if !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !Self::step_commands(netlist)?.is_empty()
            || netlist.elements.len() != 4
        {
            return Err(
                "MEASURE_CONT acquired unrelated topology, hierarchy, data, or steps".into(),
            );
        }

        let (expected_scalar, expected_continuous) = kind.expected_measurement_counts();
        let scalar = netlist
            .measurements
            .iter()
            .filter(|statement| statement.analysis.eq_ignore_ascii_case("TRAN"))
            .count();
        let continuous = netlist
            .measurements
            .iter()
            .filter(|statement| statement.analysis.eq_ignore_ascii_case("TRAN_CONT"))
            .count();
        if scalar != expected_scalar
            || continuous != expected_continuous
            || scalar + continuous != netlist.measurements.len()
        {
            return Err(format!(
                "MEASURE_CONT declaration census changed: TRAN={scalar}/{expected_scalar}, TRAN_CONT={continuous}/{expected_continuous}, total={}",
                netlist.measurements.len()
            ));
        }
        for statement in &netlist.measurements {
            let valid = match kind {
                XyceMeasureContTranKind::Derivative => matches!(
                    statement.measure_type,
                    rspice_core::analysis::MeasureType::Derivative { .. }
                ),
                XyceMeasureContTranKind::FindWhen => matches!(
                    statement.measure_type,
                    rspice_core::analysis::MeasureType::Find { .. }
                        | rspice_core::analysis::MeasureType::When { .. }
                ),
                XyceMeasureContTranKind::TriggerTarget => {
                    matches!(
                        statement.measure_type,
                        rspice_core::analysis::MeasureType::Delay { .. }
                    )
                }
            };
            if !valid || statement.print_policy != rspice_core::analysis::MeasurePrintPolicy::All {
                return Err(format!(
                    "MEASURE_CONT declaration '{}' changed type or print policy",
                    statement.name
                ));
            }
        }

        let expected_vpwl1 = if kind == XyceMeasureContTranKind::Derivative {
            &[
                (0.0, 0.1),
                (0.0025, 0.5),
                (0.005, 0.0),
                (0.0074, 0.4),
                (0.01, 0.0),
            ][..]
        } else {
            &[
                (0.0, 0.1),
                (0.0025, 0.5),
                (0.005, 0.0),
                (0.0075, 0.4),
                (0.01, 0.0),
            ][..]
        };
        Self::validate_measure_cont_pwl(netlist, "VPWL1", ["1", "0"], expected_vpwl1)?;
        Self::validate_measure_cont_pwl(netlist, "VPWL2", ["2", "0"], &[(0.0, 0.5), (0.01, 0.0)])?;
        for (name, nodes) in [("R1", ["1", "0"]), ("R2", ["2", "0"])] {
            let resistor = netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("MEASURE_CONT is missing {name}"))?;
            if resistor.nodes != nodes
                || !matches!(&resistor.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params } if value.to_bits() == 100.0f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            {
                return Err(format!(
                    "MEASURE_CONT resistor {name} changed exact topology/value"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_abm_transient_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceAbmTransientKind,
        start: Instant,
    ) -> Result<(), String> {
        let source_bytes = self.validate_abm_transient_provenance(deck, kind)?;
        self.check_abm_transient_deadline(start, "provenance")?;
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("ABM transient source is not UTF-8: {error}"))?;
        let plan = self.static_tran_plan_for_path_with_purpose(
            &deck.path,
            XyceStaticTranPlanPurpose::AnalyticOracle,
        )?;
        if plan.source.as_bytes() != source_bytes {
            return Err("ABM transient plan did not preserve the exact source bytes".into());
        }
        let parsed = Self::parse_xyce_netlist(source, &deck.path)
            .map_err(|error| format!("ABM transient parse failed: {error}"))?;
        Self::validate_abm_transient_plan(&plan, &parsed, kind)?;
        Self::validate_abm_transient_topology(&parsed, kind)?;
        self.check_abm_transient_deadline(start, "parse and exact topology")?;

        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!(
                    "ABM transient native execution exceeded shared timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                other => format!("ABM transient native execution failed: {other}"),
            })?;
        Self::validate_abm_transient_topology(&netlist, kind)?;
        let actual = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
        Self::validate_abm_transient_output_domain(&actual, kind)?;
        let gold = Self::abm_transient_dynamic_gold_table(&actual, kind)?;
        let mismatches = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            &gold,
            &actual,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "ABM transient Release 7.10 generated-gold comparison produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ));
        }

        let counterfactual = Self::abm_transient_counterfactual_table(&actual, kind)?;
        let counterfactual_mismatches = self
            .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                &counterfactual,
                &actual,
                XyceVerifyTransientTolerance::release_7_10_default(),
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
        if counterfactual_mismatches.is_empty() {
            return Err(
                "ABM transient counterfactual unexpectedly reproduced the native output".into(),
            );
        }
        self.check_abm_transient_deadline(start, "native execution, generated gold, and causality")
    }

    pub(super) fn validate_abm_transient_family_census(
        &self,
        prefix: &str,
        expected_count: usize,
        expected_names_hash: &str,
        expected_content_hash: &str,
    ) -> Result<(), String> {
        let source_relative_path = match prefix {
            XYCE_ABM_TRANSIENT_TIME_FAMILY_PREFIX => "Netlists/ABM_TIME",
            XYCE_ABM_TRANSIENT_SQRT_FAMILY_PREFIX => "Netlists/ABM_SQRT",
            _ => {
                return Err(format!(
                    "ABM transient family prefix is not recognized: {prefix}"
                ));
            }
        };
        let family = self.root.join(source_relative_path);
        let metadata = fs::symlink_metadata(&family)
            .map_err(|error| format!("failed to inspect ABM transient family: {error}"))?;
        if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
            return Err(format!(
                "ABM transient family {} must be a regular non-symlink directory",
                family.display()
            ));
        }
        let mut names = BTreeSet::new();
        let mut content = BTreeSet::new();
        for entry in fs::read_dir(&family)
            .map_err(|error| format!("failed to read ABM transient family: {error}"))?
        {
            let entry = entry
                .map_err(|error| format!("failed to inspect ABM transient member: {error}"))?;
            let member_metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
                format!(
                    "failed to inspect ABM transient member {}: {error}",
                    entry.path().display()
                )
            })?;
            if !member_metadata.file_type().is_file() || member_metadata.file_type().is_symlink() {
                return Err(format!(
                    "ABM transient member {} must be a regular non-symlink file",
                    entry.path().display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| "ABM transient filename is not UTF-8".to_string())?
                .to_ascii_lowercase();
            if !names.insert(name.clone()) {
                return Err(format!(
                    "ABM transient family has case-colliding name {name:?}"
                ));
            }
            let bytes = fs::read(entry.path())
                .map_err(|error| format!("failed to hash ABM transient member: {error}"))?;
            let canonical_bytes = Self::canonical_lf_text_identity(
                &format!("ABM transient member {}", entry.path().display()),
                &bytes,
            )?;
            content.insert(format!(
                "{name}\0{}",
                blake3::hash(&canonical_bytes).to_hex()
            ));
        }
        let names = names.into_iter().collect::<Vec<_>>();
        let content = content.into_iter().collect::<Vec<_>>();
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
                "ABM transient family census changed: {prefix} names={}/{names_hash}, content={}/{content_hash}",
                names.len(),
                content.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_abm_transient_historical_identities() -> Result<(), String> {
        let mut wrappers = BTreeSet::new();
        let mut perl = BTreeSet::new();
        for kind in XyceAbmTransientKind::ALL {
            let (wrapper_bytes, wrapper_sha256) = kind.historical_wrapper_identity();
            let (perl_bytes, perl_sha256) = kind.historical_perl_identity();
            if wrapper_bytes == 0
                || perl_bytes == 0
                || wrapper_sha256.len() != 64
                || perl_sha256.len() != 64
                || !wrapper_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
                || !perl_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
            {
                return Err("ABM transient historical wrapper/Perl identity is malformed".into());
            }
            wrappers.insert((kind.record(), wrapper_bytes, wrapper_sha256));
            perl.insert((kind.record(), perl_bytes, perl_sha256));
        }
        if wrappers.len() != 3 || perl.len() != 3 {
            return Err("ABM transient historical wrapper/Perl provenance is incomplete".into());
        }
        Ok(())
    }

    pub(super) fn validate_abm_transient_plan(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        kind: XyceAbmTransientKind,
    ) -> Result<(), String> {
        let expected_probes = kind
            .expected_columns()
            .iter()
            .skip(2)
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.tran.step.to_bits() != 1.0f64.to_bits()
            || plan.tran.stop.to_bits() != kind.stop().to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || probes != expected_probes
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.measurements.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(format!(
                "ABM transient exact TRAN/PRINT contract changed: tran={:?}, probes={probes:?}, analyses={}, outputs={}, diagnostics={}",
                plan.tran,
                netlist.analyses.len(),
                netlist.output_requests.len(),
                netlist.diagnostics.len()
            ));
        }
        if netlist.options.replace_ground.is_some()
            || netlist.options.remove_unused.is_some()
            || netlist.options.add_resistors.is_some()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
        {
            return Err(
                "ABM transient deck acquired unrelated model, hierarchy, data, or preprocessing state"
                    .into(),
            );
        }
        Ok(())
    }

    pub(super) fn validate_abm_transient_topology(
        netlist: &Netlist,
        kind: XyceAbmTransientKind,
    ) -> Result<(), String> {
        let expected_points: &[(Value, Value)] = match kind {
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
        };
        let expected_element_count = if kind == XyceAbmTransientKind::SquareRoot {
            6
        } else {
            4
        };
        if netlist.elements.len() != expected_element_count {
            return Err(format!(
                "ABM transient topology has {} elements instead of {expected_element_count}",
                netlist.elements.len()
            ));
        }
        let source = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("VS"))
            .ok_or_else(|| "ABM transient has no VS source".to_string())?;
        let ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pwl {
            points,
            delay,
            repeat_from,
        }) = &source.kind
        else {
            return Err("ABM transient VS is not an inline PWL voltage source".into());
        };
        if source.nodes != ["1", "0"]
            || delay.to_bits() != 0.0f64.to_bits()
            || repeat_from.is_some()
            || points.len() != expected_points.len()
            || points
                .iter()
                .zip(expected_points)
                .any(|(actual, expected)| {
                    actual.0.to_bits() != expected.0.to_bits()
                        || actual.1.to_bits() != expected.1.to_bits()
                })
        {
            return Err("ABM transient exact PWL source contract changed".into());
        }

        let expected_resistors: &[(&str, &str, Value)] = if kind == XyceAbmTransientKind::SquareRoot
        {
            &[("r1", "1", 1.0), ("r2", "2", 1.0), ("r3", "3", 1.0)]
        } else {
            &[("r1", "1", 1000.0), ("r2", "2", 1000.0)]
        };
        for (name, node, expected_value) in expected_resistors {
            let resistor = netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("ABM transient is missing resistor {name}"))?;
            if resistor.nodes != [*node, "0"]
                || !matches!(&resistor.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params } if value.to_bits() == expected_value.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            {
                return Err(format!(
                    "ABM transient resistor {name} changed exact topology"
                ));
            }
        }

        let expected_behavioral: &[(&str, &str, &str)] = match kind {
            XyceAbmTransientKind::DirectTime => &[("b2", "2", "(v(1)*time)")],
            XyceAbmTransientKind::ParameterTime => &[("b2", "2", "(v(1)*partime)")],
            XyceAbmTransientKind::SquareRoot => {
                &[("b2", "2", "sqrt(v(1))"), ("b3", "3", "v(2)**2")]
            }
        };
        for (name, node, expected_expression) in expected_behavioral {
            let behavioral = netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("ABM transient is missing behavioral source {name}"))?;
            let ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
            } = &behavioral.kind
            else {
                return Err(format!(
                    "ABM transient {name} is not a behavioral voltage source"
                ));
            };
            if behavioral.nodes != [*node, "0"]
                || tc1.to_bits() != 0.0f64.to_bits()
                || tc2.to_bits() != 0.0f64.to_bits()
                || Self::normalize_probe(expression) != *expected_expression
            {
                return Err(format!(
                    "ABM transient {name} topology/expression changed: nodes={:?}, expression={expression:?}",
                    behavioral.nodes
                ));
            }
        }

        let mut parameter_expressions = netlist.params.all_parameter_expressions();
        parameter_expressions.sort();
        let expected_params = if kind == XyceAbmTransientKind::ParameterTime {
            vec![("PARTIME".to_string(), "time".to_string())]
        } else {
            Vec::new()
        };
        if parameter_expressions != expected_params
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "ABM transient parameter contract changed: ordinary={parameter_expressions:?}, global={:?}",
                netlist.params.all_global_expressions()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_abm_transient_output_domain(
        table: &XycePrnTable,
        kind: XyceAbmTransientKind,
    ) -> Result<(), String> {
        if table.columns != kind.expected_columns() || table.rows.len() < 3 {
            return Err(format!(
                "ABM transient output layout changed: columns={:?}, rows={}",
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous_time = None;
        let mut observed_nontrivial = false;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != table.columns.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
            {
                return Err(format!(
                    "ABM transient row {index} is malformed or nonfinite: {row:?}"
                ));
            }
            let time = Self::xyce_default_prn_roundtrip(row[1])?;
            let input = Self::xyce_default_prn_roundtrip(row[2])?;
            if previous_time.is_some_and(|previous| time < previous) {
                return Err(format!("ABM transient TIME regressed at row {index}"));
            }
            let expected_input = Self::abm_transient_pwl_value(kind, time)?;
            // The historical Perl oracle observes both TIME and V(1) only
            // after Xyce's default `%.8e` PRN serialization. On a steep PWL
            // segment, the hidden pre-serialization TIME displacement is
            // amplified by the local slope. Account for that quantization
            // explicitly instead of applying an arbitrary loose value
            // tolerance.
            let time_quantization = 5.0e-9 * time.abs().max(1.0);
            let local_slope =
                Self::abm_transient_pwl_max_slope_near(kind, time, time_quantization)?;
            let input_quantization = 5.0e-9 * expected_input.abs().max(input.abs()).max(1.0);
            let input_tolerance =
                input_quantization + local_slope * time_quantization + 8.0 * Value::EPSILON;
            if (input - expected_input).abs() > input_tolerance {
                return Err(format!(
                    "ABM transient PWL causality failed at row {index}: time={time}, input={input}, expected={expected_input}"
                ));
            }
            observed_nontrivial |= time > 0.0 && input.abs() > 1.0;
            previous_time = Some(time);
        }
        let first_time = Self::xyce_default_prn_roundtrip(table.rows[0][1])?;
        let last_time = Self::xyce_default_prn_roundtrip(
            table.rows.last().expect("table has at least three rows")[1],
        )?;
        if first_time.to_bits() != 0.0f64.to_bits()
            || last_time.to_bits() != kind.stop().to_bits()
            || !observed_nontrivial
        {
            return Err(format!(
                "ABM transient time/input domain changed: first={first_time}, last={last_time}, nontrivial={observed_nontrivial}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_addresistors_transient_oracle(
        &self,
        deck: &XyceDeck,
        source: &str,
        original: &Netlist,
        kind: XyceAddResistorsKind,
        start: Instant,
    ) -> Result<(), String> {
        let plan = Self::addresistors_transient_plan(deck, source, original)?;
        Self::validate_addresistors_transient_plan(&plan, original)?;
        Self::validate_addresistors_flattened_topology(original, kind, false)?;

        let original_result = self
            .run_transient_family_netlist(&plan, original, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!(
                    "ADDRESISTORS original transient exceeded shared timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                other => format!("ADDRESISTORS original transient failed: {other}"),
            })?;
        let original_table =
            Self::transient_family_result_to_prn_table(&plan, original, &original_result)?;
        Self::validate_addresistors_original_transient_table(&original_table)?;
        self.check_addresistors_deadline(start, "unchanged original transient")?;

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let materialized = original
            .materialize_xyce_add_resistors_with_abort(&abort)
            .map_err(|error| format!("ADDRESISTORS materialization failed: {error}"))?;
        Self::validate_addresistors_report(&materialized.report, kind)?;
        Self::validate_addresistors_materialized_netlist(&materialized.netlist, kind)?;
        Self::validate_addresistors_flattened_topology(&materialized.netlist, kind, true)?;
        let replayed = Self::parse_xyce_netlist(&materialized.derived_source, &deck.path)
            .map_err(|error| format!("ADDRESISTORS derived artifact parse failed: {error}"))?;
        Self::validate_addresistors_replayed_artifact(&replayed, &materialized.report, kind)?;
        self.check_addresistors_deadline(start, "materialization and generated topology")?;

        let generated_result = self
            .run_transient_family_netlist(&plan, &replayed, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!(
                    "ADDRESISTORS generated transient exceeded shared timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                other => format!("ADDRESISTORS generated transient failed: {other}"),
            })?;
        let generated_table =
            Self::transient_family_result_to_prn_table(&plan, &replayed, &generated_result)?;
        Self::validate_addresistors_transient_schedule(&generated_table)?;
        Self::validate_addresistors_exp_invariant(&generated_table)?;
        let dynamic_gold = Self::addresistors_dynamic_gold_table(&generated_table)?;
        let mismatches = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            &dynamic_gold,
            &generated_table,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "ADDRESISTORS Release 7.10 dynamic-gold comparison produced {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ));
        }
        if self
            .compare_xyce_verify_transient_tables(&dynamic_gold, &original_table)
            .is_ok_and(|mismatches| mismatches.is_empty())
        {
            return Err(
                "ADDRESISTORS policy-off/original waveform unexpectedly matches generated RC decay"
                    .to_string(),
            );
        }
        self.check_addresistors_deadline(start, "generated transient and dynamic gold")
    }

    pub(super) fn validate_addresistors_transient_plan(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if !plan.steps.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.fft_analyses.is_empty()
            || !netlist.measurements.is_empty()
            || netlist.analyses.len() != 1
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.print.probes != ["V(2)".to_string(), "V(X1:2)".to_string()]
            || !matches!(
                plan.comparison_mode,
                XyceStaticTranComparisonMode::Release710IntegratedRms {
                    scientific_precision: XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION
                }
            )
        {
            return Err(format!(
                "ADDRESISTORS transient plan has state outside the removed wrapper contract: {plan:?}"
            ));
        }
        let tran = plan.tran;
        if tran.step.to_bits() != 0.0f64.to_bits()
            || tran.stop.to_bits() != 2.0f64.to_bits()
            || tran.start.map(Value::to_bits) != Some(1.0e-3f64.to_bits())
            || tran.max_step.is_some()
            || tran.uic
        {
            return Err(format!(
                "ADDRESISTORS transient requires exact .TRAN 0 2 1m schedule, got {tran:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_addresistors_original_transient_table(
        table: &XycePrnTable,
    ) -> Result<(), String> {
        Self::validate_addresistors_transient_schedule(table)?;
        for (row_index, row) in table.rows.iter().enumerate() {
            if (row[2] - 1.0).abs() > 1.0e-8 || (row[3] - 1.0).abs() > 1.0e-8 {
                return Err(format!(
                    "ADDRESISTORS unchanged original waveform is not unity at row {row_index}: {row:?}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_addresistors_transient_schedule(
        table: &XycePrnTable,
    ) -> Result<(), String> {
        if table.columns
            != [
                "Index".to_string(),
                "TIME".to_string(),
                "V(2)".to_string(),
                "V(X1:2)".to_string(),
            ]
            || table.rows.len() < 2
        {
            return Err(format!(
                "ADDRESISTORS transient output layout changed: columns={:?}, rows={}",
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous = None;
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != 4 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "ADDRESISTORS transient row {row_index} is not a finite four-column row: {row:?}"
                ));
            }
            if row[0].to_bits() != (row_index as Value).to_bits()
                || previous.is_some_and(|time| time >= row[1])
                || row[1] < 1.0e-3 - 1.0e-12
                || row[1] > 2.0 + 1.0e-12
            {
                return Err(format!(
                    "ADDRESISTORS transient Index/TIME ordering changed at row {row_index}: {row:?}"
                ));
            }
            previous = Some(row[1]);
        }
        let first = table.rows.first().expect("rows checked")[1];
        let last = table.rows.last().expect("rows checked")[1];
        // Xyce deliberately disables interpolation for the first output when
        // TSTART is present (N_ANP_Transient.C `doNotInterpolate`). Therefore
        // the first serialized row is the first accepted point at or after
        // 1 ms, not necessarily an interpolated row at exactly 1 ms.
        if first < 1.0e-3 - 1.0e-12 || (last - 2.0).abs() > 1.0e-12 {
            return Err(format!(
                "ADDRESISTORS output schedule requires first TIME >= 1m and final TIME = 2, got [{first},{last}]"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_static_tran_reference_requirement(
        purpose: XyceStaticTranPlanPurpose,
        contract: XyceStaticTranContract,
        reference_path: &Path,
    ) -> Result<(), String> {
        if purpose.requires_reference_file()
            && contract.requires_reference_file()
            && !reference_path.is_file()
        {
            return Err(format!(
                "no checked-in static .{} oracle at {}",
                contract.reference_extension(),
                reference_path.display()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_native_static_prn_tran_wrapper_contract(
        source: &str,
    ) -> Result<(), String> {
        Self::validate_native_static_prn_tran_wrapper_contract_with_format_mode(source, false)
    }

    pub(super) fn validate_native_noindex_header_tran_wrapper_contract(
        source: &str,
    ) -> Result<(), String> {
        Self::validate_native_static_prn_tran_wrapper_contract(source)?;
        let request = Self::single_tran_print_output_request(source)?;
        let format = request.format.as_deref().ok_or_else(|| {
            "wrapper-origin NOINDEX header contract requires .PRINT TRAN FORMAT=NOINDEX".to_string()
        })?;
        if !format.eq_ignore_ascii_case("NOINDEX") {
            return Err(format!(
                "wrapper-origin NOINDEX header contract requires .PRINT TRAN FORMAT=NOINDEX, got FORMAT={format}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_native_static_prn_tran_wrapper_contract_with_format_mode(
        source: &str,
        allow_wrapper_probe_primary_prn: bool,
    ) -> Result<(), String> {
        let mut primary_tran_print_count = 0usize;
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
                    return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("TRAN") {
                    return Err(format!(
                        "wrapper-origin transient .prn contract does not cover .PRINT {analysis}"
                    ));
                }
                let mut has_file_output = false;
                let mut has_probe = false;
                let mut index = 2usize;
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        match raw_key.trim().to_ascii_lowercase().as_str() {
                            "file" => has_file_output = true,
                            "format" => {
                                if !(Self::tran_print_format_is_prn_compatible(value)
                                    || allow_wrapper_probe_primary_prn
                                        && value.eq_ignore_ascii_case("PROBE"))
                                {
                                    return Err(format!(
                                        "wrapper-origin transient .prn contract does not cover .PRINT TRAN FORMAT={value}"
                                    ));
                                }
                            }
                            _ => {}
                        }
                        index += consumed;
                        continue;
                    }
                    let normalized = token_refs[index].to_ascii_lowercase();
                    if !Self::is_print_option_token(&normalized) {
                        has_probe = true;
                    }
                    index += 1;
                }
                if has_probe && !has_file_output {
                    primary_tran_print_count += 1;
                }
                continue;
            }
            if command.eq_ignore_ascii_case(".measure") || command.eq_ignore_ascii_case(".meas") {
                if Self::is_ignorable_wrapper_tran_measure_side_output(&trimmed)? {
                    continue;
                }
                let fields = Self::split_print_fields(&trimmed)?;
                if fields
                    .get(1)
                    .is_some_and(|field| field.eq_ignore_ascii_case("TRAN"))
                    && fields.get(3).is_some_and(|field| {
                        field.eq_ignore_ascii_case("EQN")
                            || field.eq_ignore_ascii_case("PARAM")
                            || field.to_ascii_uppercase().starts_with("PARAM=")
                    })
                {
                    continue;
                }
                return Err(format!(
                    "wrapper-origin transient .prn contract does not cover {command} directives"
                ));
            }
            if Self::is_extra_wrapper_tran_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin transient .prn contract does not cover {command} directives"
                ));
            }
        }

        match primary_tran_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin transient .prn contract requires one primary .PRINT TRAN statement"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin transient .prn contract requires one primary .PRINT TRAN statement, found {primary_tran_print_count}"
            )),
        }
    }

    pub(super) fn validate_native_static_csv_tran_wrapper_contract(
        source: &str,
    ) -> Result<(), String> {
        let mut primary_tran_print_count = 0usize;
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
                    return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("TRAN") {
                    return Err(format!(
                        "wrapper-origin transient CSV contract does not cover .PRINT {analysis}"
                    ));
                }

                let mut format = None;
                let mut has_file_output = false;
                let mut index = 2usize;
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        match raw_key.trim().to_ascii_lowercase().as_str() {
                            "file" => has_file_output = true,
                            "format" => format = Some(value),
                            _ => {}
                        }
                        index += consumed;
                        continue;
                    }
                    index += 1;
                }

                if has_file_output {
                    return Err(
                        "wrapper-origin transient CSV contract does not cover FILE= side outputs"
                            .to_string(),
                    );
                }
                match format {
                    Some(format) if format.eq_ignore_ascii_case("CSV") => {
                        primary_tran_print_count += 1;
                    }
                    Some(format) => {
                        return Err(format!(
                            "wrapper-origin transient CSV contract does not cover .PRINT TRAN FORMAT={format}"
                        ));
                    }
                    None => {
                        return Err(
                            "wrapper-origin transient CSV contract requires FORMAT=CSV".to_string()
                        );
                    }
                }
                continue;
            }
            if Self::is_extra_wrapper_tran_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin transient CSV contract does not cover {command} directives"
                ));
            }
        }

        match primary_tran_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin transient CSV contract requires one primary .PRINT TRAN statement"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin transient CSV contract requires one primary .PRINT TRAN statement, found {primary_tran_print_count}"
            )),
        }
    }

    pub(super) fn validate_native_static_csd_tran_wrapper_contract(
        source: &str,
    ) -> Result<(), String> {
        let mut primary_tran_print_count = 0usize;
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
                    return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("TRAN") {
                    return Err(format!(
                        "wrapper-origin transient CSDF contract does not cover .PRINT {analysis}"
                    ));
                }
                let mut format = None;
                let mut index = 2usize;
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        match raw_key.trim().to_ascii_lowercase().as_str() {
                            "file" => {
                                return Err(
                                    "wrapper-origin transient CSDF contract does not cover FILE= side outputs"
                                        .to_string(),
                                );
                            }
                            "format" => format = Some(value),
                            _ => {}
                        }
                        index += consumed;
                        continue;
                    }
                    index += 1;
                }
                match format {
                    Some(format) if format.eq_ignore_ascii_case("PROBE") => {
                        primary_tran_print_count += 1;
                    }
                    Some(format) => {
                        return Err(format!(
                            "wrapper-origin transient CSDF contract does not cover .PRINT TRAN FORMAT={format}"
                        ));
                    }
                    None => {
                        return Err(
                            "wrapper-origin transient CSDF contract requires FORMAT=PROBE"
                                .to_string(),
                        );
                    }
                }
                continue;
            }
            if Self::is_extra_wrapper_tran_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin transient CSDF contract does not cover {command} directives"
                ));
            }
        }

        match primary_tran_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin transient CSDF contract requires one primary .PRINT TRAN statement"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin transient CSDF contract requires one primary .PRINT TRAN statement, found {primary_tran_print_count}"
            )),
        }
    }

    pub(super) fn native_static_prn_tran_wrapper_contract(
        deck_path: &Path,
        relative_path: &str,
        source: &str,
        has_prn_oracle: bool,
    ) -> Option<XyceStaticTranContract> {
        if Self::is_native_noindex_header_tran_wrapper_candidate(relative_path, source) {
            return Some(XyceStaticTranContract::WrapperNoIndexHeader);
        }

        if Self::validate_native_pwl_repeat_error_tran_wrapper_contract(deck_path, source).is_ok() {
            return Some(XyceStaticTranContract::WrapperStaticExpectedError);
        }

        if Self::is_native_csd_tran_wrapper_candidate(relative_path, source) {
            return Some(XyceStaticTranContract::WrapperCsd);
        }

        if Self::is_native_csv_tran_wrapper_candidate(relative_path, source) {
            return Some(XyceStaticTranContract::WrapperCsv);
        }

        if Self::is_native_default_prn_tran_wrapper_candidate(relative_path, source)
            || Self::is_native_output_other_prn_tran_wrapper_candidate(relative_path, source)
            || Self::is_native_output_initial_interval_tran_wrapper_candidate(source)
            || Self::is_native_generic_static_prn_tran_wrapper_candidate(
                relative_path,
                source,
                has_prn_oracle,
            )
        {
            return Some(XyceStaticTranContract::WrapperStatic);
        }

        None
    }

    pub(super) fn native_output_override_prn_tran_wrapper_contract(
        source: &str,
    ) -> Result<XyceStaticTranContract, String> {
        Self::validate_native_output_override_prn_tran_wrapper_contract(source)?;
        Ok(XyceStaticTranContract::WrapperStatic)
    }

    pub(super) fn static_tran_contract_for_print_format(
        requires_wrapper: bool,
        format: Option<&str>,
    ) -> Result<XyceStaticTranContract, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::tran_print_format_is_prn_compatible(normalized) {
            return Ok(if requires_wrapper {
                XyceStaticTranContract::WrapperStatic
            } else {
                XyceStaticTranContract::PlainStatic
            });
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok(if requires_wrapper {
                XyceStaticTranContract::WrapperCsv
            } else {
                XyceStaticTranContract::PlainCsv
            });
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok(if requires_wrapper {
                XyceStaticTranContract::WrapperCsd
            } else {
                XyceStaticTranContract::PlainCsd
            });
        }
        Err(format!(
            "native static .PRINT TRAN comparison does not cover FORMAT={normalized}"
        ))
    }

    pub(super) fn validate_native_output_initial_interval_tran_wrapper_contract(
        source: &str,
    ) -> Result<(), String> {
        Self::validate_native_static_prn_tran_wrapper_contract(source)?;

        let tran_prints = Self::print_output_requests(source, "TRAN")?;
        if tran_prints.iter().any(|request| request.file.is_some()) {
            return Err(
                "wrapper-origin initial-interval transient .prn contract does not cover FILE= side outputs"
                    .to_string(),
            );
        }
        let Some(primary_print) = tran_prints.iter().find(|request| request.file.is_none()) else {
            return Err(
                "wrapper-origin initial-interval transient .prn contract requires one primary .PRINT TRAN statement"
                    .to_string(),
            );
        };
        if primary_print.probes.len() != 1 {
            return Err(format!(
                "wrapper-origin initial-interval transient .prn contract currently covers one primary probe, found {}",
                primary_print.probes.len()
            ));
        }

        let mut initial_interval_options = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let normalized = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if normalized.is_empty() {
                continue;
            }
            let Some(command) = normalized.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".options") {
                continue;
            }
            if normalized.contains("output") && normalized.contains("initial_interval") {
                initial_interval_options += 1;
            } else {
                return Err(format!(
                    "wrapper-origin initial-interval transient .prn contract does not cover {command} directives without OUTPUT INITIAL_INTERVAL"
                ));
            }
        }

        match initial_interval_options {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin initial-interval transient .prn contract requires .OPTIONS OUTPUT INITIAL_INTERVAL"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin initial-interval transient .prn contract requires one .OPTIONS OUTPUT INITIAL_INTERVAL directive, found {initial_interval_options}"
            )),
        }
    }

    pub(super) fn validate_native_output_override_prn_tran_wrapper_contract(
        source: &str,
    ) -> Result<(), String> {
        Self::validate_native_output_override_prn_wrapper_contract(source, "TRAN")
    }

    pub(super) fn validate_native_pwl_repeat_error_tran_wrapper_contract(
        deck_path: &Path,
        source: &str,
    ) -> Result<(), String> {
        Self::validate_native_static_prn_tran_wrapper_contract(source)?;
        if !Self::source_may_have_pwl_repeat_option(source) {
            return Err(
                "wrapper-origin PWL repeat error contract requires a primary PWL repeat deck"
                    .to_string(),
            );
        }

        let stem = deck_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| {
                "wrapper-origin PWL repeat error contract requires a .cir filename".to_string()
            })?;
        let sibling_path = deck_path.with_file_name(format!("{stem}RepeatFail.cir"));
        let sibling_source = fs::read_to_string(&sibling_path).map_err(|err| {
            format!(
                "wrapper-origin PWL repeat error contract requires sibling expected-error deck '{}': {err}",
                sibling_path.display()
            )
        })?;

        Self::validate_expected_pwl_repeat_value_error_source(&sibling_source, &sibling_path)
    }

    pub(super) fn validate_transient_stepnum_column(
        reference: &XycePrnTable,
        expected_step_index: usize,
    ) -> Result<(), String> {
        let layout = Self::transient_reference_layout(reference)?;
        let Some(stepnum_column) = layout.stepnum_column else {
            return Ok(());
        };
        let expected_step_index = expected_step_index as Value;
        for (row_index, row) in reference.rows.iter().enumerate() {
            let stepnum = *row.get(stepnum_column).ok_or_else(|| {
                format!("row {row_index} has no STEPNUM column at index {stepnum_column}")
            })?;
            if !stepnum.is_finite() {
                return Err(format!(
                    "row {row_index} has non-finite STEPNUM value {stepnum}"
                ));
            }
            if (stepnum - expected_step_index).abs() > f64::EPSILON {
                return Err(format!(
                    "row {row_index} has STEPNUM {stepnum}, expected {expected_step_index}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn baseline_family_tran_contracts_compatible(
        kind: XyceBaselineFamilyKind,
        baseline: XyceStaticTranContract,
        target: XyceStaticTranContract,
    ) -> bool {
        match kind {
            XyceBaselineFamilyKind::ScopedModel => matches!(
                (baseline, target),
                (
                    XyceStaticTranContract::PlainStatic,
                    XyceStaticTranContract::WrapperStatic
                )
            ),
            XyceBaselineFamilyKind::AcAnalysisExpression
            | XyceBaselineFamilyKind::BjtExternalNode
            | XyceBaselineFamilyKind::DcAnalysisExpression
            | XyceBaselineFamilyKind::DelimitedExpression
            | XyceBaselineFamilyKind::NestedIncludeIdentity
            | XyceBaselineFamilyKind::PassiveResPrimaryValue
            | XyceBaselineFamilyKind::SubcktParameterPrecedence
            | XyceBaselineFamilyKind::SubcktParameterResolution => false,
            XyceBaselineFamilyKind::AgeCap
            | XyceBaselineFamilyKind::DiodeModelAlias
            | XyceBaselineFamilyKind::SwitchStateCase
            | XyceBaselineFamilyKind::SinExpression
            | XyceBaselineFamilyKind::ParamExpression
            | XyceBaselineFamilyKind::PassiveCapPrimaryValue
            | XyceBaselineFamilyKind::PassiveTemperatureOverride
            | XyceBaselineFamilyKind::Subckt
            | XyceBaselineFamilyKind::Supernode => baseline == target,
            XyceBaselineFamilyKind::TransientAnalysisExpression => matches!(
                (baseline, target),
                (
                    XyceStaticTranContract::PlainStatic,
                    XyceStaticTranContract::WrapperStatic
                )
            ),
        }
    }

    pub(super) fn validate_noindex_tran_prn_header(first_line: &str) -> Result<(), String> {
        if first_line.trim().is_empty() {
            return Err("generated .prn output is empty".to_string());
        }
        let normalized = first_line.to_ascii_lowercase();
        if normalized.contains("index") {
            return Err(format!(
                "first output line contains forbidden Index text: {first_line}"
            ));
        }
        if !normalized.contains("time") {
            return Err(format!(
                "first output line does not contain required TIME text: {first_line}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_passive_cap_primary_transient_plan(
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        if plan.contract != XyceStaticTranContract::PlainStatic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
        {
            return Err("capacitor primary-value parity requires one ordinary unstepped default .prn transient output".to_string());
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step <= 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.step > plan.tran.stop
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err("capacitor primary-value parity requires finite '.TRAN step stop' values and no START, MAXSTEP, or UIC".to_string());
        }
        if plan.print.probes.len() != 2 {
            return Err(
                "capacitor primary-value parity requires exactly two ordered probes".to_string(),
            );
        }
        Self::validate_passive_primary_source_forms(
            &plan.source,
            XycePassivePrimaryKind::CapacitorTran,
        )
    }

    pub(super) fn validate_passive_temperature_override_transient_plan(
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "passive temperature-coefficient override parity";
        if plan.contract != XyceStaticTranContract::PlainStatic {
            return Err(format!(
                "{LABEL} requires ordinary primary .prn output, got {:?}",
                plan.contract
            ));
        }
        if !plan.steps.is_empty() || plan.output_override || plan.timeint_conststep {
            return Err(format!(
                "{LABEL} does not admit .STEP, output overrides, or constant-step output"
            ));
        }
        if plan.wrapper_tolerance.is_some() || Self::source_has_comp_directive(&plan.source) {
            return Err(format!(
                "{LABEL} uses the canonical default xyce_verify tolerance and does not admit wrapper or *COMP overrides"
            ));
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step < 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.step > plan.tran.stop
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} requires a finite ordinary transient tuple with 0 <= step <= stop, positive stop, and no UIC"
            ));
        }
        if plan.print.probes.is_empty() {
            return Err(format!(
                "{LABEL} requires at least one ordered .PRINT TRAN probe"
            ));
        }

        let mut model_count = 0usize;
        let mut tran_count = 0usize;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        for line in Self::logical_netlist_lines(&plan.source) {
            let stripped = Self::strip_netlist_comment(&line);
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".model" => model_count += 1,
                ".tran" => tran_count += 1,
                ".print" => print_count += 1,
                ".options" => {}
                ".end" => end_count += 1,
                other => {
                    return Err(format!("{LABEL} does not admit directive '{other}'"));
                }
            }
        }
        if (model_count, tran_count, print_count, end_count) != (1, 1, 1, 1) {
            return Err(format!(
                "{LABEL} requires exactly one .MODEL, .TRAN, .PRINT, and .END; found ({model_count}, {tran_count}, {print_count}, {end_count})"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_transient_analysis_expression_plan(
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "transient-analysis expression parity";
        if !matches!(
            plan.contract,
            XyceStaticTranContract::PlainStatic | XyceStaticTranContract::WrapperStatic
        ) {
            return Err(format!(
                "{LABEL} requires ordinary default .prn output, got {:?}",
                plan.contract
            ));
        }
        if !plan.steps.is_empty() || plan.output_override || plan.timeint_conststep {
            return Err(format!(
                "{LABEL} does not admit .STEP, output overrides, or constant-step output"
            ));
        }
        if plan.wrapper_tolerance.is_some() {
            return Err(format!(
                "{LABEL} uses the canonical default xyce_verify tolerance and does not admit wrapper overrides"
            ));
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step < 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.step > plan.tran.stop
            || plan
                .tran
                .start
                .is_some_and(|value| !value.is_finite() || value < 0.0 || value > plan.tran.stop)
            || plan
                .tran
                .max_step
                .is_some_and(|value| !value.is_finite() || value <= 0.0)
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} requires a finite nonnegative TSTEP, positive TSTOP, bounded optional TSTART, positive optional DTMAX, and no UIC"
            ));
        }
        if plan.print.probes.is_empty() {
            return Err(format!(
                "{LABEL} requires at least one ordered .PRINT TRAN probe"
            ));
        }
        Self::transient_analysis_source_qualification(&plan.source).map(|_| ())
    }

    pub(super) fn validate_diode_model_alias_transient_plan(
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "native diode model-parameter alias equivalence";
        if plan.contract != XyceStaticTranContract::PlainStatic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
        {
            return Err(format!(
                "{LABEL} requires one ordinary, unstepped, adaptive default .prn transient output"
            ));
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step <= 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.step > plan.tran.stop
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} requires finite '.TRAN step stop' values and no START, MAXSTEP, or UIC"
            ));
        }
        if plan.print.probes.len() != 2 {
            return Err(format!("{LABEL} requires exactly two ordered probes"));
        }
        Self::diode_model_alias_source_qualification(&plan.source)?;
        let comp_targets = Self::diode_model_alias_comp_targets(&plan.source)?;
        let expected_targets = [
            "time".to_string(),
            Self::normalize_probe(&plan.print.probes[0]),
            Self::normalize_probe(&plan.print.probes[1]),
        ];
        if comp_targets != expected_targets
            || comp_targets.iter().collect::<BTreeSet<_>>().len() != comp_targets.len()
        {
            return Err(format!(
                "{LABEL} requires unique ordered *COMP targets TIME then the two .PRINT probes"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_switch_state_case_transient_plan(
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "generic-switch initial-state case equivalence";
        if plan.contract != XyceStaticTranContract::PlainStatic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || Self::source_has_comp_directive(&plan.source)
        {
            return Err(format!(
                "{LABEL} requires one ordinary, unstepped, adaptive default .prn transient output"
            ));
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step <= 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.step > plan.tran.stop
            || plan.tran.start.map(Value::to_bits) != Some(0.0f64.to_bits())
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} requires finite '.TRAN step stop 0' values, positive zero START, and no MAXSTEP or UIC"
            ));
        }
        if plan.print.probes.len() != 2 {
            return Err(format!("{LABEL} requires exactly two ordered probes"));
        }
        Self::switch_state_case_source_qualification(&plan.source).map(|_| ())
    }

    pub(super) fn validate_age_cap_transient_plan(plan: &XyceStaticTranPlan) -> Result<(), String> {
        const LABEL: &str = "native capacitor AGE/D equivalence";
        if plan.contract != XyceStaticTranContract::PlainStatic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
        {
            return Err(format!(
                "{LABEL} requires one ordinary, unstepped, adaptive default .prn transient output"
            ));
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step <= 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.step > plan.tran.stop
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} requires finite '.TRAN step stop' values and no START, MAXSTEP, or UIC"
            ));
        }
        if plan.print.probes.len() != 2 {
            return Err(format!("{LABEL} requires exactly two ordered probes"));
        }
        Self::age_cap_source_qualification(&plan.source).map(|_| ())
    }

    pub(super) fn validate_sin_expression_transient_plan(
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        if plan.contract != XyceStaticTranContract::PlainStatic {
            return Err(format!(
                "exact SIN/SPICE_SIN parity requires ordinary primary .prn output, got {:?}",
                plan.contract
            ));
        }
        if !plan.steps.is_empty() {
            return Err("exact SIN/SPICE_SIN parity does not admit .STEP".to_string());
        }
        if plan.output_override {
            return Err("exact SIN/SPICE_SIN parity does not admit an output override".to_string());
        }
        if plan.timeint_conststep {
            return Err(
                "exact SIN/SPICE_SIN parity requires ordinary adaptive transient output"
                    .to_string(),
            );
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step <= 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.step > plan.tran.stop
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "exact SIN/SPICE_SIN parity requires finite '.TRAN step stop' values with 0 < step <= stop and no START, MAXSTEP, or UIC; got step={}, stop={}, start={:?}, max_step={:?}, uic={}",
                plan.tran.step, plan.tran.stop, plan.tran.start, plan.tran.max_step, plan.tran.uic
            ));
        }
        let [probe] = plan.print.probes.as_slice() else {
            return Err(format!(
                "exact SIN/SPICE_SIN parity requires exactly one voltage probe, found {}",
                plan.print.probes.len()
            ));
        };
        let voltage_probe = Self::parse_voltage_probe(probe).ok_or_else(|| {
            format!("exact SIN/SPICE_SIN probe '{probe}' is not an atomic voltage probe")
        })?;
        if voltage_probe.accessor != XyceVoltageAccessor::Value || voltage_probe.node_neg.is_some()
        {
            return Err(format!(
                "exact SIN/SPICE_SIN probe '{probe}' must be one ordinary single-ended voltage"
            ));
        }

        let mut tran_count = 0usize;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        for line in Self::logical_netlist_lines(&plan.source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".tran" => {
                    tran_count += 1;
                    if stripped.split_whitespace().count() != 3 {
                        return Err(
                            "exact SIN/SPICE_SIN parity requires the canonical '.TRAN step stop' form"
                                .to_string(),
                        );
                    }
                }
                ".print" => {
                    print_count += 1;
                    let fields = Self::split_print_fields(stripped)?;
                    if fields.len() != 3
                        || !fields[1].eq_ignore_ascii_case("TRAN")
                        || Self::normalize_probe(&fields[2]) != Self::normalize_probe(probe)
                    {
                        return Err(
                            "exact SIN/SPICE_SIN parity requires one canonical '.PRINT TRAN <voltage>' statement without destination or formatting assignments"
                                .to_string(),
                        );
                    }
                }
                ".end" => {
                    end_count += 1;
                    if !stripped.eq_ignore_ascii_case(".end") {
                        return Err(
                            "exact SIN/SPICE_SIN parity requires an ordinary '.END' statement"
                                .to_string(),
                        );
                    }
                }
                other => {
                    return Err(format!(
                        "exact SIN/SPICE_SIN parity does not admit directive '{other}'"
                    ));
                }
            }
        }
        if (tran_count, print_count, end_count) != (1, 1, 1) {
            return Err(format!(
                "exact SIN/SPICE_SIN parity requires exactly one .TRAN, .PRINT, and .END; found ({tran_count}, {print_count}, {end_count})"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_param_expression_transient_plan(
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "parameter-expression parity";
        if plan.contract != XyceStaticTranContract::PlainStatic {
            return Err(format!(
                "{LABEL} requires ordinary primary .prn output, got {:?}",
                plan.contract
            ));
        }
        if !plan.steps.is_empty() {
            return Err(format!("{LABEL} does not admit .STEP"));
        }
        if plan.output_override {
            return Err(format!("{LABEL} does not admit an output override"));
        }
        if plan.timeint_conststep {
            return Err(format!(
                "{LABEL} requires ordinary adaptive transient output"
            ));
        }
        if !plan.tran.step.is_finite()
            || !plan.tran.stop.is_finite()
            || plan.tran.step <= 0.0
            || plan.tran.stop <= 0.0
            || plan.tran.step > plan.tran.stop
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} requires finite '.TRAN step stop' values with 0 < step <= stop and no START, MAXSTEP, or UIC; got step={}, stop={}, start={:?}, max_step={:?}, uic={}",
                plan.tran.step, plan.tran.stop, plan.tran.start, plan.tran.max_step, plan.tran.uic
            ));
        }
        let [probe] = plan.print.probes.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one voltage probe, found {}",
                plan.print.probes.len()
            ));
        };
        let voltage_probe = Self::parse_voltage_probe(probe)
            .ok_or_else(|| format!("{LABEL} probe '{probe}' is not an atomic voltage probe"))?;
        if voltage_probe.accessor != XyceVoltageAccessor::Value || voltage_probe.node_neg.is_some()
        {
            return Err(format!(
                "{LABEL} probe '{probe}' must be one ordinary single-ended voltage"
            ));
        }
        Self::validate_param_expression_direct_source_forms(&plan.source)?;

        if plan.source.lines().any(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|field| field.eq_ignore_ascii_case("*COMP"))
        }) {
            return Err(format!(
                "{LABEL} uses the unmodified xyce_verify default tolerances and does not admit *COMP directives"
            ));
        }

        let mut param_count = 0usize;
        let mut subckt_count = 0usize;
        let mut ends_count = 0usize;
        let mut print_count = 0usize;
        let mut tran_count = 0usize;
        let mut end_count = 0usize;
        for line in Self::logical_netlist_lines(&plan.source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".param" => {
                    param_count += 1;
                    let fields = Self::split_grouped_whitespace_fields(
                        stripped,
                        "parameter-expression .PARAM statement",
                    )?;
                    let [_, assignment] = fields.as_slice() else {
                        return Err(format!(
                            "{LABEL} requires one canonical '.PARAM name=value' assignment"
                        ));
                    };
                    let Some((name, value)) = assignment.split_once('=') else {
                        return Err(format!(
                            "{LABEL} requires one canonical '.PARAM name=value' assignment"
                        ));
                    };
                    if name.trim().is_empty()
                        || value.trim().is_empty()
                        || value.contains('=')
                        || !Self::is_single_spice_numeric_literal(value)
                    {
                        return Err(format!(
                            "{LABEL} requires one direct finite numeric .PARAM value"
                        ));
                    }
                }
                ".subckt" => {
                    subckt_count += 1;
                    let fields = Self::split_grouped_whitespace_fields(
                        stripped,
                        "parameter-expression .SUBCKT statement",
                    )?;
                    if fields.len() != 8 {
                        return Err(format!(
                            "{LABEL} requires '.SUBCKT name' followed by exactly six ports and no defaults"
                        ));
                    }
                }
                ".ends" => {
                    ends_count += 1;
                    if !stripped.eq_ignore_ascii_case(".ends") {
                        return Err(format!("{LABEL} requires a bare '.ENDS' statement"));
                    }
                }
                ".print" => {
                    print_count += 1;
                    let fields = Self::split_print_fields(stripped)?;
                    if fields.len() != 3
                        || !fields[1].eq_ignore_ascii_case("TRAN")
                        || Self::normalize_probe(&fields[2]) != Self::normalize_probe(probe)
                    {
                        return Err(format!(
                            "{LABEL} requires one canonical '.PRINT TRAN <voltage>' statement without destination or formatting assignments"
                        ));
                    }
                }
                ".tran" => {
                    tran_count += 1;
                    if stripped.split_whitespace().count() != 3 {
                        return Err(format!(
                            "{LABEL} requires the canonical '.TRAN step stop' form"
                        ));
                    }
                }
                ".end" => {
                    end_count += 1;
                    if !stripped.eq_ignore_ascii_case(".end") {
                        return Err(format!("{LABEL} requires an ordinary '.END' statement"));
                    }
                }
                other => {
                    return Err(format!("{LABEL} does not admit directive '{other}'"));
                }
            }
        }
        if (
            param_count,
            subckt_count,
            ends_count,
            print_count,
            tran_count,
            end_count,
        ) != (1, 1, 1, 1, 1, 1)
        {
            return Err(format!(
                "{LABEL} requires exactly one .PARAM, .SUBCKT, .ENDS, .PRINT, .TRAN, and .END; found ({param_count}, {subckt_count}, {ends_count}, {print_count}, {tran_count}, {end_count})"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_transient_execution_envelope(
        netlist: &Netlist,
        estimated_steps: Value,
    ) -> Result<(), String> {
        let size = Self::transient_flattened_problem_size(netlist)?;
        Self::validate_transient_problem_size_envelope(size, estimated_steps)
    }

    pub(super) fn validate_transient_preflight_execution_envelope(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
    ) -> Result<(), String> {
        let estimated_steps = Self::preflight_transient_estimated_steps(netlist, tran);
        let size = Self::transient_hierarchy_problem_size_estimate(netlist)?;
        Self::validate_transient_problem_size_envelope(size, estimated_steps)
    }

    pub(super) fn validate_transient_problem_size_envelope(
        size: XyceTransientProblemSize,
        estimated_steps: Value,
    ) -> Result<(), String> {
        let estimated_element_steps = estimated_steps * size.element_count as Value;
        if estimated_element_steps > MAX_NATIVE_TRAN_ELEMENT_STEPS {
            return Err(format!(
                "transient harness execution envelope supports at most {:.0} native element-step unit(s), but this deck requires about {:.0} ({} flattened element(s) across about {:.0} step(s))",
                MAX_NATIVE_TRAN_ELEMENT_STEPS,
                estimated_element_steps,
                size.element_count,
                estimated_steps
            ));
        }
        let estimated_compact_device_steps = estimated_steps * size.compact_device_count as Value;
        if estimated_compact_device_steps > MAX_NATIVE_TRAN_COMPACT_DEVICE_STEPS {
            return Err(format!(
                "transient harness execution envelope supports at most {:.0} native compact-device step unit(s), but this deck requires about {:.0} ({} flattened compact device(s) across about {:.0} step(s))",
                MAX_NATIVE_TRAN_COMPACT_DEVICE_STEPS,
                estimated_compact_device_steps,
                size.compact_device_count,
                estimated_steps
            ));
        }
        let estimated_node_solve_steps =
            estimated_steps * (size.node_count as Value) * (size.node_count as Value);
        if estimated_node_solve_steps > MAX_NATIVE_TRAN_NODE_SOLVE_STEPS {
            return Err(format!(
                "transient harness execution envelope supports at most {:.0} native node-solve step unit(s), but this deck requires about {:.0} ({} flattened node(s) across about {:.0} step(s))",
                MAX_NATIVE_TRAN_NODE_SOLVE_STEPS,
                estimated_node_solve_steps,
                size.node_count,
                estimated_steps
            ));
        }
        Ok(())
    }

    pub(super) fn validate_static_tran_analysis_contract(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        if !tran.stop.is_finite() || tran.stop <= 0.0 {
            return Err(format!(
                ".TRAN stop time must be finite and positive, got {}",
                tran.stop
            ));
        }
        if !tran.step.is_finite() || tran.step < 0.0 {
            return Err(format!(
                ".TRAN print step must be finite and non-negative, got {}",
                tran.step
            ));
        }
        if let Some(start) = tran.start
            && (!start.is_finite() || start < 0.0 || start > tran.stop)
        {
            return Err(format!(
                ".TRAN start time must be finite and within [0, stop], got {start}"
            ));
        }
        if let Some(max_step) = tran.max_step
            && (!max_step.is_finite() || max_step <= 0.0)
        {
            return Err(format!(
                ".TRAN maximum step must be finite and positive when specified, got {max_step}"
            ));
        }
        // `run_tran` reads UIC from the parsed netlist; the plan carries it so
        // validation still reflects the complete .TRAN command surface.
        let _engine_reads_uic_from_netlist = tran.uic;

        Self::validate_transient_preflight_execution_envelope(netlist, tran)?;
        for probe in &print.probes {
            Self::validate_tran_probe(probe, netlist)?;
        }

        Ok(())
    }

    pub(super) fn validate_static_step_tran_contract(netlist: &Netlist) -> Result<(), String> {
        for element in &netlist.elements {
            match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Self::validate_static_step_tran_source_spec(&element.name, spec)?;
                }
                ElementKind::Resistor { .. } => {
                    Self::validate_static_step_resistor_contract(netlist, &element.name)?;
                }
                ElementKind::Capacitor { .. } => {
                    Self::validate_static_step_capacitor_contract(netlist, &element.name)?
                }
                ElementKind::Inductor { .. } => {
                    Self::validate_static_step_inductor_contract(netlist, &element.name)?;
                }
                ElementKind::Coupling {
                    inductors,
                    coefficient,
                    model,
                } => {
                    Self::validate_static_step_coupling_contract(
                        netlist,
                        &element.name,
                        inductors,
                        *coefficient,
                        model.as_deref(),
                    )?;
                }
                ElementKind::TransmissionLine {
                    z0,
                    td,
                    freq,
                    nl,
                    model,
                } => Self::validate_lossless_transmission_line_contract(
                    netlist,
                    &element.name,
                    element.nodes.len(),
                    *z0,
                    *td,
                    *freq,
                    *nl,
                    model.as_deref(),
                )?,
                ElementKind::BehavioralVoltage { expression, .. }
                | ElementKind::BehavioralCurrent { expression, .. } => {
                    Self::validate_static_step_tran_behavioral_expression(
                        &element.name,
                        expression,
                        &netlist.params,
                    )?;
                }
                ElementKind::Diode { .. } => {
                    Self::validate_static_step_diode_contract(netlist, &element.name)?;
                }
                _ => {
                    return Err(format!(
                        "native .STEP .PRINT TRAN comparison currently supports static R/L/C passives, coupled inductors, native Level=2 temperature-breakdown diodes, independent DC/PULSE/SIN/PWL/PAT sources, and solution-independent behavioral sources; element '{}' requires a broader stepped transient oracle contract",
                        element.name
                    ));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_static_step_tran_source_spec(
        source_name: &str,
        spec: &rspice_core::netlist::SourceSpec,
    ) -> Result<(), String> {
        match spec {
            rspice_core::netlist::SourceSpec::Dc(_)
            | rspice_core::netlist::SourceSpec::Ac { .. }
            | rspice_core::netlist::SourceSpec::DcAc { .. }
            | rspice_core::netlist::SourceSpec::Pulse { .. }
            | rspice_core::netlist::SourceSpec::Sin { .. }
            | rspice_core::netlist::SourceSpec::Pwl { .. }
            | rspice_core::netlist::SourceSpec::PwlFile { .. }
            | rspice_core::netlist::SourceSpec::Pat { .. } => Ok(()),
            rspice_core::netlist::SourceSpec::DcTransient { transient, .. }
            | rspice_core::netlist::SourceSpec::DcAcTransient { transient, .. } => {
                Self::validate_static_step_tran_source_spec(source_name, transient)
            }
            other => Err(format!(
                "native .STEP .PRINT TRAN comparison currently supports independent DC/PULSE/SIN/PWL/PAT sources; source '{source_name}' uses {other:?}"
            )),
        }
    }

    #[cfg(test)]
    pub(super) fn validate_native_transient_contract(netlist: &Netlist) -> Result<(), String> {
        Self::validate_native_transient_contract_for_purpose(
            netlist,
            XyceStaticTranPlanPurpose::AbsoluteOracle,
        )
    }

    pub(super) fn validate_native_relational_transient_contract(
        netlist: &Netlist,
    ) -> Result<(), String> {
        Self::validate_native_transient_contract_for_purpose(
            netlist,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )
    }

    pub(super) fn validate_native_transient_contract_for_purpose(
        netlist: &Netlist,
        purpose: XyceStaticTranPlanPurpose,
    ) -> Result<(), String> {
        if netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }))
        {
            let flattened =
                rspice_core::netlist::flatten_netlist_with_models(netlist).map_err(|err| {
                    format!(
                        "native static .PRINT TRAN comparison could not flatten subcircuits: {err}"
                    )
                })?;
            Self::validate_flattened_subcircuit_instances_resolved(netlist, &flattened.elements)?;
            let mut flat_netlist = netlist.clone();
            flat_netlist.elements = flattened.elements;
            flat_netlist.models.extend(flattened.scoped_models);
            flat_netlist
                .initial_conditions
                .extend(flattened.scoped_initial_conditions);
            flat_netlist.node_sets.extend(flattened.scoped_node_sets);
            flat_netlist.subcircuits.clear();
            return Self::validate_native_transient_contract_for_purpose(&flat_netlist, purpose);
        }

        if purpose == XyceStaticTranPlanPurpose::ScopedModelRelationalFamily {
            return Self::validate_native_scoped_model_relational_transient_contract_flat(netlist);
        }

        let elements = &netlist.elements;
        let params = &netlist.params;
        let has_qualified_bjt = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_transient_level1_npn(netlist, element)
            });
        let has_qualified_irb_bjt = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_device_is_single_native_transient_level1_npn_irb(netlist, element)
            });
        let has_qualified_vbic = purpose.validates_absolute_device_contract()
            && Self::netlist_is_native_transient_vbic_level11_single_bjt(netlist);
        let has_qualified_level1_mos = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_transient_level1_mosfet(netlist, element)
            });
        let has_qualified_level2_mos = purpose.validates_absolute_device_contract()
            && Self::netlist_is_native_transient_level2_mosfet_network(netlist);
        let has_qualified_level3_mos = purpose.validates_absolute_device_contract()
            && Self::netlist_is_native_transient_level3_mosfet_network(netlist);
        let has_qualified_level1_cmos_chain = purpose.validates_absolute_device_contract()
            && Self::netlist_is_native_transient_level1_cmos_chain(netlist);
        let has_qualified_ekv26 = purpose.validates_absolute_device_contract()
            && Self::netlist_is_native_transient_ekv26_pair(netlist);
        let has_qualified_diode = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_absolute_transient_exact_is_diode(netlist, element)
            });
        let has_qualified_tbv_diode = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_absolute_transient_tbv_diode(netlist, element)
            });
        let has_qualified_vdmos = purpose.validates_absolute_device_contract()
            && Self::netlist_is_native_absolute_transient_vdmos_level18(netlist);
        let has_qualified_minimum_diode = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_absolute_transient_minimum_diode(netlist, element)
            });
        let has_qualified_legacy_diode = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_absolute_transient_legacy_diode(netlist, element)
            });
        let has_qualified_cmc_diode = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_generated_cmc_diode(netlist, element)
            });
        let has_qualified_juncap_diode = purpose.validates_absolute_device_contract()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_generated_juncap_diode(netlist, element)
            });
        let has_qualified_level9_bsim3 = purpose.admits_default_level9_bsim3()
            && elements.iter().any(|element| {
                Self::netlist_element_is_native_absolute_transient_level9_bsim3(netlist, element)
            });
        let has_qualified_bsim4_capacitor = purpose.validates_absolute_device_contract()
            && Self::netlist_is_native_transient_bsim4_capacitor(netlist);
        let has_qualified_bsim3_capacitor = purpose.validates_absolute_device_contract()
            && Self::netlist_is_native_transient_bsim3_capacitor(netlist);
        if (has_qualified_bjt
            || has_qualified_irb_bjt
            || has_qualified_vbic
            || has_qualified_level1_mos
            || has_qualified_level2_mos
            || has_qualified_level3_mos
            || has_qualified_level1_cmos_chain
            || has_qualified_ekv26
            || has_qualified_diode
            || has_qualified_tbv_diode
            || has_qualified_vdmos
            || has_qualified_minimum_diode
            || has_qualified_legacy_diode
            || has_qualified_cmc_diode
            || has_qualified_juncap_diode
            || has_qualified_level9_bsim3
            || has_qualified_bsim4_capacitor
            || has_qualified_bsim3_capacitor)
            && !has_qualified_bsim4_capacitor
            && !has_qualified_bsim3_capacitor
            && !Self::native_transient_uses_standard_startup(netlist)
        {
            return Err(
                "native qualified absolute-device transient comparison requires a single ordinary DC operating-point startup at the default 27 C TEMP and TNOM, without UIC/NOOP, .TEMP analyses, explicit initial conditions, or node-set hints"
                    .to_string(),
            );
        }
        for element in elements {
            match &element.kind {
                ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_) => {}
                ElementKind::Vcvs { gain, .. } => {
                    Self::validate_finite_controlled_source_gain(
                        "VCVS",
                        &element.name,
                        "gain",
                        *gain,
                    )?;
                }
                ElementKind::Vccs {
                    transconductance, ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "VCCS",
                        &element.name,
                        "transconductance",
                        *transconductance,
                    )?;
                }
                ElementKind::Cccs {
                    gain,
                    control_element,
                    ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "CCCS",
                        &element.name,
                        "gain",
                        *gain,
                    )?;
                    Self::validate_current_controlled_source_probe(
                        elements,
                        "CCCS",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::Ccvs {
                    transresistance,
                    control_element,
                    ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "CCVS",
                        &element.name,
                        "transresistance",
                        *transresistance,
                    )?;
                    Self::validate_current_controlled_source_probe(
                        elements,
                        "CCVS",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::BehavioralVoltage { expression, .. }
                | ElementKind::BehavioralCurrent { expression, .. } => {
                    Self::validate_transient_behavioral_expression(
                        &element.name,
                        expression,
                        params,
                    )?;
                }
                ElementKind::Resistor { value_expr, .. } => {
                    if let Some(expression) = value_expr {
                        Self::validate_transient_behavioral_expression(
                            &element.name,
                            expression,
                            params,
                        )?;
                    }
                }
                ElementKind::Capacitor { .. } => {
                    Self::validate_static_step_capacitor_contract(netlist, &element.name)?
                }
                ElementKind::Inductor { .. } => {
                    Self::validate_static_step_inductor_contract(netlist, &element.name)?;
                }
                ElementKind::Coupling {
                    inductors,
                    coefficient,
                    model,
                } => {
                    Self::validate_static_step_coupling_contract(
                        netlist,
                        &element.name,
                        inductors,
                        *coefficient,
                        model.as_deref(),
                    )?;
                }
                ElementKind::TransmissionLine {
                    z0,
                    td,
                    freq,
                    nl,
                    model,
                } => Self::validate_lossless_transmission_line_contract(
                    netlist,
                    &element.name,
                    element.nodes.len(),
                    *z0,
                    *td,
                    *freq,
                    *nl,
                    model.as_deref(),
                )?,
                ElementKind::VSwitch { .. } => {}
                ElementKind::ISwitch {
                    control_element, ..
                } => {
                    Self::validate_current_controlled_source_probe(
                        elements,
                        "ISWITCH",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::GenericSwitch {
                    model,
                    control_expression,
                    ..
                } => {
                    if Self::netlist_model_is_current_switch(netlist, model) {
                        let control_element =
                            Self::direct_branch_current_control(control_expression).ok_or_else(
                                || {
                                    format!(
                                        "native static .PRINT TRAN comparison does not support generic ISWITCH element '{}' with CONTROL expression '{}' because it is not a direct branch-current probe",
                                        element.name, control_expression
                                    )
                                },
                            )?;
                        Self::validate_current_controlled_source_probe(
                            elements,
                            "generic ISWITCH",
                            &element.name,
                            &control_element,
                        )?;
                    } else {
                        Self::validate_transient_generic_switch_expression(
                            &element.name,
                            control_expression,
                            params,
                        )?;
                    }
                }
                ElementKind::Bjt { .. }
                    if purpose.validates_absolute_device_contract() && has_qualified_vbic => {}
                ElementKind::Bjt { .. }
                    if purpose.validates_absolute_device_contract()
                        && has_qualified_irb_bjt
                        && Self::netlist_device_is_single_native_transient_level1_npn_irb(
                            netlist, element,
                        ) => {}
                ElementKind::Bjt { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_transient_level1_npn(
                            netlist, element,
                        ) => {}
                ElementKind::Bjt { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_is_native_transient_level1_gp_bjt_network(netlist)
                        && Self::netlist_element_is_native_transient_level1_gp_bjt(
                            netlist, element,
                        ) => {}
                ElementKind::Mosfet { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_is_native_transient_ekv26_pair(netlist)
                        && Self::netlist_element_is_native_transient_ekv26(netlist, element) => {}
                ElementKind::Mosfet { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_is_native_absolute_transient_vdmos_level18(netlist)
                        && Self::netlist_element_is_native_absolute_transient_vdmos_level18(
                            netlist, element,
                        ) => {}
                ElementKind::Mosfet { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_transient_level1_mosfet(
                            netlist, element,
                        ) => {}
                ElementKind::Mosfet { .. }
                    if purpose.validates_absolute_device_contract()
                        && has_qualified_level2_mos
                        && Self::netlist_element_is_native_transient_level2_mosfet(
                            netlist, element,
                        ) => {}
                ElementKind::Mosfet { .. }
                    if purpose.validates_absolute_device_contract()
                        && has_qualified_level3_mos
                        && Self::netlist_element_is_native_transient_level3_mosfet(
                            netlist, element,
                        ) => {}
                ElementKind::Mosfet { .. }
                    if purpose.validates_absolute_device_contract()
                        && has_qualified_level1_cmos_chain
                        && Self::netlist_element_is_native_transient_level1_mosfet_unbounded(
                            netlist, element,
                        ) => {}
                ElementKind::Mosfet { .. }
                    if purpose.admits_default_level9_bsim3()
                        && Self::netlist_element_is_native_absolute_transient_level9_bsim3(
                            netlist, element,
                        ) => {}
                ElementKind::Mosfet { .. } if has_qualified_bsim4_capacitor => {}
                ElementKind::Mosfet { .. } if has_qualified_bsim3_capacitor => {}
                ElementKind::Mosfet { .. }
                    if Self::netlist_device_is_native_b3soi_mosfet(netlist, &element.name) => {}
                ElementKind::Mosfet { .. }
                    if matches!(
                        purpose,
                        XyceStaticTranPlanPurpose::RelationalFamily
                            | XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
                    ) && Self::netlist_device_is_native_relational_mos3(
                        netlist,
                        &element.name,
                    ) => {}
                ElementKind::Jfet { .. }
                    if Self::netlist_device_is_native_classic_jfet(netlist, &element.name) => {}
                ElementKind::Xspice {
                    model,
                    pspice_u_timing,
                    ..
                } if Self::netlist_xspice_model_is_native_transient_tff(netlist, model) => {}
                ElementKind::Xspice {
                    model,
                    pspice_u_timing,
                    ..
                } if Self::netlist_xspice_model_is_native_transient_dig_gate(
                    netlist,
                    model,
                    pspice_u_timing.as_ref(),
                ) => {}
                ElementKind::XyceMemristor { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_xyce_memristor(netlist, element) => {}
                ElementKind::Diode { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_absolute_transient_exact_is_diode(
                            netlist, element,
                        ) => {}
                ElementKind::Diode { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_absolute_transient_tbv_diode(
                            netlist, element,
                        ) => {}
                ElementKind::Diode { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_absolute_transient_minimum_diode(
                            netlist, element,
                        ) => {}
                ElementKind::Diode { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_absolute_transient_legacy_diode(
                            netlist, element,
                        ) => {}
                ElementKind::Diode { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_generated_cmc_diode(
                            netlist, element,
                        ) => {}
                ElementKind::Diode { .. }
                    if purpose.validates_absolute_device_contract()
                        && Self::netlist_element_is_native_generated_juncap_diode(
                            netlist, element,
                        ) => {}
                ElementKind::Diode { .. }
                    if matches!(
                        purpose,
                        XyceStaticTranPlanPurpose::RelationalFamily
                            | XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
                    ) && Self::netlist_device_is_native_relational_legacy_diode(
                        netlist,
                        &element.name,
                    ) => {}
                _ => {
                    return Err(match purpose {
                        XyceStaticTranPlanPurpose::AbsoluteOracle
                        | XyceStaticTranPlanPurpose::AnalyticOracle => format!(
                            "native static .PRINT TRAN comparison currently supports independent, behavioral, static R/L/C, switch, controlled-source, validated native Level-1 NPN and extended Level-1 NPN IRB/RBM, EKV26, validated native VDMOS LEVEL=18 integrated-RMS, bounded native classic MOSFET LEVEL=1/2/3 models, exact IS-only, validated legacy, Level=2 TBV, and validated MINRES/MINCAP legacy-diode models, native B3SOI, and native classic JFET transient decks; element '{}' requires a broader transient oracle contract",
                            element.name
                        ),
                        XyceStaticTranPlanPurpose::DefaultLevel9XyceVerifyOracle => format!(
                            "native Release 7.10 integrated-RMS .PRINT TRAN comparison supports the strict bare Xyce LEVEL=9 BSIM3 subset in addition to the ordinary absolute-device envelope; element '{}' requires a broader transient oracle contract",
                            element.name
                        ),
                        XyceStaticTranPlanPurpose::RelationalFamily
                        | XyceStaticTranPlanPurpose::AgeCapRelationalFamily
                        | XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily => format!(
                            "native relational .PRINT TRAN comparison currently supports independent, behavioral, static R/L/C, switch, controlled-source, native B3SOI, native classic JFET, and validated native MOS3 and legacy-diode subsets; element '{}' requires a broader relational runtime contract",
                            element.name
                        ),
                        XyceStaticTranPlanPurpose::ScopedModelRelationalFamily => format!(
                            "native scoped-model relational .PRINT TRAN comparison currently supports finite independent and behavioral sources, static numeric R/C, finite VCCS, exact scalar IS/BF NPN models, and exact scalar IS diode models; element '{}' requires a broader scoped-model runtime contract",
                            element.name
                        ),
                    });
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_lossless_transmission_line_contract(
        netlist: &Netlist,
        element_name: &str,
        node_count: usize,
        z0: Option<Value>,
        td: Option<Value>,
        freq: Option<Value>,
        nl: Option<Value>,
        model: Option<&str>,
    ) -> Result<(), String> {
        if node_count != 4 {
            return Err(format!(
                "lossless transmission line '{element_name}' requires four electrical terminals, found {node_count}"
            ));
        }
        if let Some(model_name) = model {
            if z0.is_some() || td.is_some() || freq.is_some() || nl.is_some() {
                return Err(format!(
                    "model-backed LTRA element '{element_name}' cannot combine its model with instance Z0, TD, F, or NL overrides"
                ));
            }
            return rspice_core::engine::validate_native_xyce_ltra_model_contract(netlist, model_name)
                .map_err(|error| {
                    format!(
                        "model-backed LTRA element '{element_name}' using model '{model_name}' is not oracle-qualified: {error}"
                    )
                });
        }
        let z0 = z0.ok_or_else(|| {
            format!("lossless transmission line '{element_name}' requires an explicit Z0")
        })?;
        if !z0.is_finite() || z0 <= 0.0 {
            return Err(format!(
                "lossless transmission line '{element_name}' requires finite positive Z0, got {z0}"
            ));
        }
        let delay = match (td, freq, nl) {
            (Some(delay), None, None) => delay,
            (None, Some(frequency), Some(length))
                if frequency.is_finite()
                    && frequency > 0.0
                    && length.is_finite()
                    && length > 0.0 =>
            {
                length / frequency
            }
            (None, Some(frequency), None) if frequency.is_finite() && frequency > 0.0 => {
                0.25 / frequency
            }
            _ => {
                return Err(format!(
                    "lossless transmission line '{element_name}' requires either TD or a finite positive F/NL pair"
                ));
            }
        };
        if !delay.is_finite() || delay <= 0.0 {
            return Err(format!(
                "lossless transmission line '{element_name}' requires finite positive propagation delay, got {delay}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_native_scoped_model_relational_transient_contract_flat(
        netlist: &Netlist,
    ) -> Result<(), String> {
        if !Self::native_transient_uses_standard_startup(netlist) {
            return Err(
                "native scoped-model relational comparison requires ordinary DC operating-point startup at the default 27 C TEMP and TNOM, without UIC/NOOP, .TEMP analyses, explicit initial conditions, or node-set hints"
                    .to_string(),
            );
        }

        for element in &netlist.elements {
            match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Self::validate_scoped_model_relational_source(&element.name, spec)?;
                }
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() => {}
                ElementKind::Capacitor {
                    value,
                    value_expr,
                    initial_voltage,
                    model,
                    instance_params,
                    deferred_params,
                } if value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && initial_voltage.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() => {}
                ElementKind::Vccs {
                    transconductance,
                    transconductance_expr,
                    ..
                } if transconductance_expr.is_none() => {
                    Self::validate_finite_controlled_source_gain(
                        "VCCS",
                        &element.name,
                        "transconductance",
                        *transconductance,
                    )?
                }
                ElementKind::BehavioralVoltage { expression, .. }
                | ElementKind::BehavioralCurrent { expression, .. } => {
                    Self::validate_transient_behavioral_expression(
                        &element.name,
                        expression,
                        &netlist.params,
                    )?;
                }
                ElementKind::Bjt { .. }
                    if Self::netlist_element_is_native_scoped_model_relational_bjt(
                        netlist, element,
                    ) => {}
                ElementKind::Diode { .. }
                    if Self::netlist_element_is_native_exact_is_diode(netlist, element) => {}
                _ => {
                    return Err(format!(
                        "native scoped-model relational comparison does not qualify element '{}'",
                        element.name
                    ));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_static_transient_passive_value_expression(
        device_kind: &str,
        element_name: &str,
        expression: &str,
        params: &rspice_core::netlist::ParamContext,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison could not prepare {device_kind} value expression '{expression}' on element '{element_name}': {err}"
            )
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison does not yet support {device_kind} value expression '{expression}' on element '{element_name}': {err}"
            )
        })?;
        if Self::passive_value_expression_depends_on_runtime_quantity(&ast) {
            return Err(format!(
                "native static .PRINT TRAN comparison does not yet support runtime-dependent {device_kind} value expression '{expression}' on element '{element_name}'"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_transient_behavioral_expression(
        element_name: &str,
        expression: &str,
        params: &rspice_core::netlist::ParamContext,
    ) -> Result<(), String> {
        Self::parse_transient_behavioral_expression(element_name, expression, params).map(drop)
    }

    pub(super) fn validate_static_step_tran_behavioral_expression(
        element_name: &str,
        expression: &str,
        params: &rspice_core::netlist::ParamContext,
    ) -> Result<(), String> {
        // Every parameter-step member is reparsed by
        // `create_perturbed_netlist_multi`, then built by a fresh Engine run.
        // Behavioral programs therefore receive the member's parameter scope
        // and may safely retain ordinary solution probes such as V(...) and
        // I(...), just as they do in an unstepped transient analysis.
        Self::validate_transient_behavioral_expression(element_name, expression, params)
    }

    pub(super) fn validate_transient_generic_switch_expression(
        element_name: &str,
        expression: &str,
        params: &rspice_core::netlist::ParamContext,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison could not prepare generic switch CONTROL expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        let _ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison does not yet support generic switch CONTROL expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        if Self::expression_contains_sdt(&_ast) {
            return Err(format!(
                "native static .PRINT TRAN comparison does not support stateful SDT in generic switch CONTROL expression '{}' on element '{}'",
                expression, element_name
            ));
        }
        // Generic SWITCH controls are linearized against the live Newton
        // solution just like behavioral sources. Keep V(...)/I(...) controls
        // in the native transient contract; only parser/build failures are
        // rejected here.
        Ok(())
    }

    pub(super) fn validate_tran_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
                return Self::validate_atomic_tran_probe(
                    &normalized_expression,
                    expression,
                    netlist,
                );
            }
            let expression_upper = expression.to_ascii_uppercase();
            if netlist.measurements.iter().any(|measurement| {
                measurement.analysis.eq_ignore_ascii_case("TRAN")
                    && matches!(
                        measurement.measure_type,
                        rspice_core::analysis::MeasureType::Equation { .. }
                    )
                    && expression_upper.contains(&measurement.name.to_ascii_uppercase())
            }) {
                let context = Self::print_tran_eval_context(netlist, 0.0);
                rspice_core::netlist::expr::eval_expression(expression, &context).map_err(
                    |err| format!("unsupported .PRINT TRAN expression '{{{expression}}}': {err}"),
                )?;
                return Ok(());
            }
            if Self::stateful_tran_print_expression(probe, netlist)?.is_some() {
                return Self::validate_tran_probe_expression(expression, netlist);
            }
            if Self::print_expression_contains_probe_reference(expression) {
                return Self::validate_tran_probe_expression(expression, netlist);
            }
            let context = Self::print_tran_eval_context(netlist, 0.0);
            rspice_core::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                format!("unsupported .PRINT TRAN expression '{{{expression}}}': {err}")
            })?;
            return Ok(());
        }

        let normalized = Self::normalize_probe(probe);
        Self::validate_atomic_tran_probe(&normalized, probe, netlist)
    }

    pub(super) fn validate_atomic_tran_probe(
        normalized: &str,
        original: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if normalized.eq_ignore_ascii_case("TEMP") || normalized.eq_ignore_ascii_case("TEMPER") {
            return Ok(());
        }
        if let Some((element_name, parameter)) =
            Self::parse_device_operating_point_probe(normalized)
            && parameter.eq_ignore_ascii_case("R")
            && Self::find_native_xyce_memristor_element(netlist, &element_name).is_some()
        {
            return Ok(());
        }
        if let Some((element_name, parameter)) =
            Self::parse_device_operating_point_probe(normalized)
            && Self::netlist_has_xyce_core_namespace(netlist, &element_name)
            && matches!(parameter.to_ascii_lowercase().as_str(), "m" | "h" | "b")
        {
            return Ok(());
        }
        if let Some(voltage_probe) = Self::parse_tran_voltage_probe(normalized) {
            if !voltage_probe.node_pos.is_empty()
                && voltage_probe
                    .node_neg
                    .as_deref()
                    .is_none_or(|node| !node.is_empty())
            {
                return Ok(());
            }
        }
        if let Some(element_name) = Self::parse_current_probe(normalized) {
            if Self::netlist_has_recorded_branch_current(netlist, &element_name) {
                return Ok(());
            }
            if Self::netlist_has_independent_current_source(netlist, &element_name) {
                return Ok(());
            }
            return Err(format!(
                "transient branch-current probe '{}' targets an element without a recorded transient branch current",
                original
            ));
        }
        if let Some(element_name) = Self::parse_power_probe(normalized) {
            if Self::find_recorded_two_terminal_branch_element(netlist, &element_name).is_some() {
                return Ok(());
            }
            return Err(format!(
                "transient power probe '{}' targets an unsupported branch/device",
                original
            ));
        }
        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            match parameter.as_str() {
                "r" if Self::find_resistor_element(netlist, &element_name).is_some()
                    || Self::find_native_xyce_memristor_element(netlist, &element_name)
                        .is_some() =>
                {
                    return Ok(());
                }
                "c" if Self::find_capacitor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "l" if Self::find_inductor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "temp" if Self::resistor_temperature_value(netlist, &element_name)?.is_some() => {
                    return Ok(());
                }
                _ => {}
            }
            return Err(format!(
                "device parameter probe '{}' targets an unsupported transient parameter",
                original
            ));
        }
        if Self::normalize_probe(original) == "time" {
            return Ok(());
        }
        if netlist.measurements.iter().any(|measurement| {
            measurement.analysis.eq_ignore_ascii_case("TRAN")
                && measurement.name.eq_ignore_ascii_case(original)
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Equation { .. }
                )
        }) {
            return Ok(());
        }
        Err(format!("unsupported .PRINT TRAN probe '{}'", original))
    }

    pub(super) fn validate_tran_probe_expression(
        expression: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let probe = format!("{{{expression}}}");
        if let Some(runtime) = Self::stateful_tran_print_expression(&probe, netlist)? {
            for branch in runtime.program.branch_map.keys() {
                let probe = format!("i({branch})");
                Self::validate_atomic_tran_probe(&Self::normalize_probe(&probe), &probe, netlist)?;
            }
            return Ok(());
        }
        let mut call_value = |call: &str| {
            let normalized = Self::normalize_probe(call);
            Self::validate_atomic_tran_probe(&normalized, call, netlist)?;
            Ok(1.0)
        };
        let context = Self::print_tran_eval_context(netlist, 0.0);
        Self::evaluate_print_expression_with_probe_calls(expression, context, &mut call_value)
            .map_err(|err| {
                format!("unsupported .PRINT TRAN expression '{{{expression}}}': {err}")
            })?;
        Ok(())
    }

    pub(super) fn validate_transient_result_time_grid(
        result: &TransientResult,
    ) -> Result<(), String> {
        if result.time.is_empty() {
            return Err("transient result has no time points".to_string());
        }
        for (index, time) in result.time.iter().copied().enumerate() {
            if !time.is_finite() {
                return Err(format!(
                    "transient result time point {index} is non-finite ({time})"
                ));
            }
            if index > 0 && time < result.time[index - 1] {
                return Err(format!(
                    "transient result time grid is not monotonic at point {index}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn transient_analysis_expression_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }
        let parent = deck.path.parent()?;
        // Transient expression qualification has the same paired-family
        // cardinality as its DC counterpart; reject singleton wrapper decks
        // before touching their potentially very large source cards.
        let circuit_count = Self::circuit_file_count(parent)?;
        if circuit_count < 4 || circuit_count % 2 != 0 {
            return None;
        }
        let mut paths = Vec::new();
        let mut baseline_count = 0usize;
        let mut target_count = 0usize;
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
                || self.has_static_tran_reference_oracle(&path)
            {
                return None;
            }
            let member_relative = self.relative_key(&path);
            let wrapper = self.requires_upstream_wrapper(&member_relative);
            let source = fs::read_to_string(&path).ok()?;
            let (representation, _) =
                Self::transient_analysis_source_qualification(&source).ok()?;
            if wrapper
                != (representation == XyceTransientAnalysisRepresentation::ParameterExpression)
            {
                return None;
            }
            if wrapper {
                target_count += 1;
            } else {
                baseline_count += 1;
            }
            paths.push(path);
        }
        if baseline_count < 2 || baseline_count != target_count || paths.len() != 2 * baseline_count
        {
            return None;
        }

        let mut pair_counts = BTreeMap::<(String, String), usize>::new();
        let mut selected = None;
        for path in &paths {
            let relative_path = self.relative_key(path);
            let member = XyceDeck {
                path: path.clone(),
                relative_path,
                section: XyceDeckSection::Netlists,
            };
            let contract = self.transient_analysis_expression_candidate_contract(&member)?;
            let pair = (
                Self::normalize_manifest_key(&self.relative_key(&contract.baseline_path)),
                Self::normalize_manifest_key(
                    &self.relative_key(
                        contract
                            .member_paths
                            .iter()
                            .find(|member| !Self::same_path(member, &contract.baseline_path))?,
                    ),
                ),
            );
            *pair_counts.entry(pair).or_default() += 1;
            if Self::same_path(path, &deck.path) {
                selected = Some(contract);
            }
        }
        if pair_counts.len() != baseline_count || pair_counts.values().any(|count| *count != 2) {
            return None;
        }
        selected
    }

    pub(super) fn transient_analysis_expression_candidate_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/")
            || self.has_static_tran_reference_oracle(&deck.path)
        {
            return None;
        }
        let parent = deck.path.parent()?;
        let source = fs::read_to_string(&deck.path).ok()?;
        let (representation, _) = Self::transient_analysis_source_qualification(&source).ok()?;
        let is_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        if is_wrapper
            != (representation == XyceTransientAnalysisRepresentation::ParameterExpression)
        {
            return None;
        }

        let purpose = if is_wrapper {
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
        } else {
            XyceStaticTranPlanPurpose::RelationalFamily
        };
        let plan = self
            .static_tran_family_plan_for_path(&deck.path, purpose)
            .ok()?;
        Self::validate_transient_analysis_expression_plan(&plan).ok()?;
        let netlist = Self::parse_xyce_netlist(&plan.source, &deck.path).ok()?;
        let snapshot = Self::transient_analysis_expression_snapshot(&netlist, &plan.print).ok()?;
        let time_scale = Self::tran_print_time_scale_factor(&plan.source).ok()?;

        let mut matches = Vec::new();
        for entry in fs::read_dir(parent).ok()? {
            let entry = entry.ok()?;
            let candidate = entry.path();
            if !entry.file_type().ok()?.is_file()
                || Self::same_path(&candidate, &deck.path)
                || !candidate
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                || fs::metadata(&candidate)
                    .ok()
                    .is_none_or(|metadata| metadata.len() == 0)
                || self.has_static_tran_reference_oracle(&candidate)
            {
                continue;
            }
            let candidate_relative = self.relative_key(&candidate);
            let candidate_wrapper = self.requires_upstream_wrapper(&candidate_relative);
            if candidate_wrapper == is_wrapper {
                continue;
            }
            let candidate_source = fs::read_to_string(&candidate).ok()?;
            let Ok((candidate_representation, _)) =
                Self::transient_analysis_source_qualification(&candidate_source)
            else {
                continue;
            };
            if candidate_wrapper
                != (candidate_representation
                    == XyceTransientAnalysisRepresentation::ParameterExpression)
            {
                continue;
            }
            let candidate_purpose = if candidate_wrapper {
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
            } else {
                XyceStaticTranPlanPurpose::RelationalFamily
            };
            let Ok(candidate_plan) =
                self.static_tran_family_plan_for_path(&candidate, candidate_purpose)
            else {
                continue;
            };
            if Self::validate_transient_analysis_expression_plan(&candidate_plan).is_err()
                || plan.print.probes != candidate_plan.print.probes
                || !Self::tran_analyses_match_exactly(&plan.tran, &candidate_plan.tran)
                || plan.timeint_conststep != candidate_plan.timeint_conststep
                || !Self::baseline_family_tran_contracts_compatible(
                    XyceBaselineFamilyKind::TransientAnalysisExpression,
                    if is_wrapper {
                        candidate_plan.contract
                    } else {
                        plan.contract
                    },
                    if is_wrapper {
                        plan.contract
                    } else {
                        candidate_plan.contract
                    },
                )
                || Self::tran_print_time_scale_factor(&candidate_plan.source)
                    .ok()
                    .is_none_or(|candidate_scale| candidate_scale.to_bits() != time_scale.to_bits())
            {
                continue;
            }
            let Ok(candidate_netlist) =
                Self::parse_xyce_netlist(&candidate_plan.source, &candidate)
            else {
                continue;
            };
            let Ok(candidate_snapshot) = Self::transient_analysis_expression_snapshot(
                &candidate_netlist,
                &candidate_plan.print,
            ) else {
                continue;
            };
            let semantic_match = if is_wrapper {
                Self::compare_transient_analysis_expression_snapshots(
                    &candidate_snapshot,
                    &snapshot,
                )
            } else {
                Self::compare_transient_analysis_expression_snapshots(
                    &snapshot,
                    &candidate_snapshot,
                )
            };
            if semantic_match.is_ok() {
                matches.push(candidate);
            }
        }
        let [counterpart] = matches.as_slice() else {
            return None;
        };
        let (baseline_path, target_path) = if is_wrapper {
            (counterpart.clone(), deck.path.clone())
        } else {
            (deck.path.clone(), counterpart.clone())
        };
        let family = format!(
            "{}:{}",
            parent.file_name()?.to_str()?,
            baseline_path.file_stem()?.to_str()?
        );
        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::TransientAnalysisExpression,
            comparison: XyceBaselineFamilyComparison::TolerancedStrict,
            family,
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path, target_path],
            target_path: Some(deck.path.clone()),
        })
    }
}
