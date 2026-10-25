use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "TR_TRAN transient-analysis alias relation";
const OWNER_PATH: &str = "Netlists/TR_TRAN/tr2.cir";
const REFERENCE_PATH: &str = "Netlists/TR_TRAN/tran2.cir";
const OWNER_RECORD: &str = "netlists/tr_tran/tr2.cir";
const REFERENCE_RECORD: &str = "netlists/tr_tran/tran2.cir";
const OWNER_CONTRACT: &str = "tr_tran_short_alias_wrapper_owner";
const REFERENCE_CONTRACT: &str = "tr_tran_long_form_reference";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TrTranRole {
    ShortOwner,
    LongReference,
}

impl TrTranRole {
    const ALL: [Self; 2] = [Self::ShortOwner, Self::LongReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::ShortOwner => OWNER_CONTRACT,
            Self::LongReference => REFERENCE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::ShortOwner => OWNER_PATH,
            Self::LongReference => REFERENCE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ShortOwner => OWNER_RECORD,
            Self::LongReference => REFERENCE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path().rsplit('/').next().expect("TR_TRAN file name")
    }
}

impl XyceTestRunner {
    fn validate_tr_tran_typed_contract(
        &self,
        role: TrTranRole,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist, XyceAnalyticRcSpecification), String> {
        let purpose = match role {
            TrTranRole::ShortOwner => XyceStaticTranPlanPurpose::AnalyticOracle,
            TrTranRole::LongReference => XyceStaticTranPlanPurpose::RelationalFamily,
        };
        let plan = self.static_tran_plan_for_path_with_purpose(path, purpose)?;
        let expected_contract = match role {
            TrTranRole::ShortOwner => XyceStaticTranContract::WrapperStatic,
            TrTranRole::LongReference => XyceStaticTranContract::PlainStatic,
        };
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 5.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 1 || !print.probes[0].eq_ignore_ascii_case("V(1)")
            })
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        let comp_tolerances = Self::xyce_verify_comp_tolerances(source, &["V(1)".to_string()])?;
        if comp_tolerances.as_slice().first().is_none_or(|tolerance| {
            comp_tolerances.len() != 1
                || tolerance.relative.to_bits() != 5.0e-3f64.to_bits()
                || tolerance.absolute.to_bits() != 5.0e-7f64.to_bits()
                || tolerance.zero.to_bits() != XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE.to_bits()
                || tolerance.absolute_difference.to_bits()
                    != XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE.to_bits()
                || tolerance.offset.to_bits() != 0.0f64.to_bits()
        }) {
            return Err(format!(
                "{LABEL} {} *COMP tolerance changed",
                role.file_name()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.file_name()))?;
        if netlist.elements.len() != 3
            || netlist.analyses.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !Self::analytic_timeint_only_options_match(&netlist.options, None, None, None, None)
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let [capacitor, resistor, voltage] = netlist.elements.as_slice() else {
            unreachable!("TR_TRAN element count checked")
        };
        if !capacitor.name.eq_ignore_ascii_case("C1")
            || capacitor.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&capacitor.nodes, &["1", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value, value_expr: None, initial_voltage: Some(initial_voltage), model: None,
                instance_params, deferred_params,
            } if value.to_bits() == 1.0e-6f64.to_bits()
                && initial_voltage.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
            || !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&resistor.nodes, &["1", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value, value_expr: None, model: None, instance_params, deferred_params,
            } if value.to_bits() == 1.0e3f64.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
            || !voltage.name.eq_ignore_ascii_case("V1")
            || voltage.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&voltage.nodes, &["2", "0"])
            || !matches!(&voltage.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} passive topology changed",
                role.file_name()
            ));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step, stop, start: None, max_step: None, uic: false,
        } if step.to_bits() == 0.0f64.to_bits() && stop.to_bits() == 5.0e-3f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed transient changed",
                role.file_name()
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 1
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || request.dependencies[0].expression
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
            || !matches!(netlist.saves.signals.as_slice(),
                [rspice_core::netlist::SaveSignal::Voltage(node)] if node == "1")
        {
            return Err(format!(
                "{LABEL} {} PRINT request changed",
                role.file_name()
            ));
        }
        Ok((
            plan,
            netlist,
            XyceAnalyticRcSpecification {
                output_node: "1".to_string(),
                source_value: 0.0,
                initial_voltage: 1.0,
                resistance: 1.0e3,
                capacitance: 1.0e-6,
                time_constant: 1.0e-3,
            },
        ))
    }

    fn validate_tr_tran_table(
        role: TrTranRole,
        table: &XycePrnTable,
        stop: Value,
    ) -> Result<(), String> {
        if table.columns.len() != 3
            || !table.columns[0].eq_ignore_ascii_case("Index")
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(1)")
            || table.rows.len() < 3
        {
            return Err(format!(
                "{LABEL} {} table shape changed: {table:?}",
                role.file_name()
            ));
        }
        let mut prior_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 3
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || prior_time.is_some_and(|prior| row[1] <= prior)
            {
                return Err(format!(
                    "{LABEL} {} row {index} is malformed",
                    role.file_name()
                ));
            }
            prior_time = Some(row[1]);
        }
        let first = &table.rows[0];
        let last = table.rows.last().expect("TR_TRAN table is nonempty");
        if first[1].to_bits() != 0.0f64.to_bits()
            || (first[2] - 1.0).abs() > 1.0e-12
            || (last[1] - stop).abs() > stop.abs().max(1.0) * 1.0e-12
            || last[2] >= 0.02
        {
            return Err(format!(
                "{LABEL} {} domain or decay changed",
                role.file_name()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_tr_tran_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let read = |member_role: TrTranRole| {
            fs::read_to_string(self.root.join(member_role.path())).map_err(|error| {
                format!("{LABEL} cannot read {}: {error}", member_role.file_name())
            })
        };
        let owner_source = read(TrTranRole::ShortOwner)?;
        let reference_source = read(TrTranRole::LongReference)?;
        let normalized_owner = owner_source
            .replace(".print TR v(1)", ".print TRAN v(1)")
            .replace(".TR 0 5ms", ".TRAN 0 5ms");
        if normalized_owner.as_bytes() != reference_source.as_bytes() {
            return Err(format!(
                "{LABEL} sources no longer differ only by TR/TRAN aliases"
            ));
        }

        let run = |member_role: TrTranRole| {
            let source = read(member_role)?;
            let (plan, _parsed, specification) = self.validate_tr_tran_typed_contract(
                member_role,
                &source,
                &self.root.join(member_role.path()),
            )?;
            let (executed, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!("{LABEL} execution failed: {other}"),
                })?;
            let table = Self::transient_family_result_to_prn_table(&plan, &executed, &result)?;
            Self::validate_tr_tran_table(member_role, &table, plan.tran.stop)?;
            Self::validate_analytic_rc_initial_sample(&table, &specification)?;
            Self::validate_analytic_rc_complete_time_domain(&table, plan.tran.stop)?;
            let analytic = Self::analytic_rc_reference_table(&table, &specification)?;
            let tolerance = XyceVerifyTransientTolerance {
                relative: 5.0e-3,
                absolute: 5.0e-7,
                zero: XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE,
                absolute_difference: XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE,
                offset: 0.0,
            };
            let mismatches = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
                &analytic,
                &table,
                tolerance,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {} violated the analytic RC oracle: {mismatches:?}",
                    member_role.file_name()
                ));
            }
            Ok::<_, String>(table)
        };

        let owner = run(TrTranRole::ShortOwner)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired between independent runs"));
        }
        let reference = run(TrTranRole::LongReference)?;
        let mismatches = self.compare_serialized_default_prn_tables(&owner, &reference)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} failed its historical byte relation: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce")
    }

    #[test]
    fn tr_tran_both_roles_parse_and_execute_as_transient() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        for role in TrTranRole::ALL {
            let path = corpus_root().join(role.path());
            let source = fs::read_to_string(&path).expect("read TR_TRAN member");
            let (plan, netlist, _) = runner
                .validate_tr_tran_typed_contract(role, &source, &path)
                .expect("TR and TRAN must share typed transient semantics");
            assert!(matches!(
                netlist.analyses.as_slice(),
                [AnalysisCommand::Tran { .. }]
            ));
            assert_eq!(
                netlist.output_requests[0].analysis,
                Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            );
            assert_eq!(plan.tran.stop.to_bits(), 5.0e-3f64.to_bits());
        }
    }

    #[test]
    fn tr_tran_live_oracle_executes_both_roles() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner
            .validate_tr_tran_oracle(Instant::now())
            .expect("TR/TRAN relation should execute natively");
    }

    #[test]
    fn tr_tran_analytic_oracle_rejects_shared_wrong_decay() {
        let mut table = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(1)".into()],
            rows: vec![
                vec![0.0, 0.0, 1.0],
                vec![1.0, 1.0e-3, (-1.0f64).exp()],
                vec![2.0, 5.0e-3, (-5.0f64).exp()],
            ],
        };
        XyceTestRunner::validate_tr_tran_table(TrTranRole::ShortOwner, &table, 5.0e-3)
            .expect("canonical decay-shaped table passes structural guard");
        table.rows[1][2] = 0.0;
        let specification = XyceAnalyticRcSpecification {
            output_node: "1".into(),
            source_value: 0.0,
            initial_voltage: 1.0,
            resistance: 1.0e3,
            capacitance: 1.0e-6,
            time_constant: 1.0e-3,
        };
        let reference = XyceTestRunner::analytic_rc_reference_table(&table, &specification)
            .expect("build counterfactual analytic table");
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let mismatches = runner
            .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                &reference,
                &table,
                XyceVerifyTransientTolerance {
                    relative: 5.0e-3,
                    absolute: 5.0e-7,
                    zero: XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE,
                    absolute_difference: XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE,
                    offset: 0.0,
                },
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )
            .expect("compare counterfactual table");
        assert!(!mismatches.is_empty());
    }

    #[test]
    fn tr_tran_typed_contract_rejects_analysis_probe_topology_and_domain_drift() {
        for (label, from, to) in [
            ("analysis", ".TR 0 5ms", ".DC V1 0 1 1"),
            ("probe", ".print TR v(1)", ".print TR v(2)"),
            ("capacitance", "c1 1 0 1uF IC=1", "c1 1 0 2uF IC=1"),
            ("stop", ".TR 0 5ms", ".TR 0 6ms"),
        ] {
            let temporary = tempfile::tempdir().expect("create TR_TRAN mutation root");
            let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
            let path = runner.root.join(OWNER_PATH);
            fs::create_dir_all(path.parent().expect("owner has a parent"))
                .expect("create TR_TRAN mutation family");
            let canonical = fs::read_to_string(corpus_root().join(OWNER_PATH))
                .expect("read canonical short form");
            let mutated = canonical.replacen(from, to, 1);
            assert_ne!(mutated, canonical, "mutation {label} must apply");
            fs::write(&path, &mutated).expect("write typed mutation");
            assert!(
                runner
                    .validate_tr_tran_typed_contract(TrTranRole::ShortOwner, &mutated, &path,)
                    .is_err(),
                "typed mutation {label} must fail closed"
            );
        }
    }

    #[test]
    fn tr_tran_expired_deadline_fails_closed() {
        let mut runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner.config.max_time_per_test_ms = 1;
        assert!(
            runner
                .validate_tr_tran_oracle(Instant::now() - Duration::from_millis(10))
                .is_err()
        );
    }
}
