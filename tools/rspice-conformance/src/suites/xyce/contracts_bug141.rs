use super::*;

const LABEL: &str = "BUG_141_SON native solver-backend completion";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_141_SON/bug_141-empty.cir";
const ABS_PATH: &str = "Netlists/Certification_Tests/BUG_141_SON/abs.cir";
const POLY_PATH: &str = "Netlists/Certification_Tests/BUG_141_SON/poly.cir";
const DIG_COUNT_PATH: &str = "Netlists/Certification_Tests/BUG_141_SON/dig_count.cir";
const BUG138_PATH: &str = "Netlists/Certification_Tests/BUG_141_SON/bug_138_1.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_141_son/bug_141-empty.cir";
const ABS_RECORD: &str = "netlists/certification_tests/bug_141_son/abs.cir";
const POLY_RECORD: &str = "netlists/certification_tests/bug_141_son/poly.cir";
const DIG_COUNT_RECORD: &str = "netlists/certification_tests/bug_141_son/dig_count.cir";
const BUG138_RECORD: &str = "netlists/certification_tests/bug_141_son/bug_138_1.cir";
const OWNER_CONTRACT: &str = "bug141_native_solver_backend_completion_wrapper_owner";
const WORKER_CONTRACT: &str = "bug141_native_solver_backend_completion_worker";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug141Role {
    WrapperOwner,
    Abs,
    Poly,
    DigCount,
    Bug138,
}

impl Bug141Role {
    const WORKERS: [Self; 4] = [Self::Abs, Self::Poly, Self::DigCount, Self::Bug138];

    pub(super) fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            OWNER_RECORD => Some(Self::WrapperOwner),
            ABS_RECORD => Some(Self::Abs),
            POLY_RECORD => Some(Self::Poly),
            DIG_COUNT_RECORD => Some(Self::DigCount),
            BUG138_RECORD => Some(Self::Bug138),
            _ => None,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        if self == Self::WrapperOwner {
            OWNER_CONTRACT
        } else {
            WORKER_CONTRACT
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => OWNER_PATH,
            Self::Abs => ABS_PATH,
            Self::Poly => POLY_PATH,
            Self::DigCount => DIG_COUNT_PATH,
            Self::Bug138 => BUG138_PATH,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::WrapperOwner => "comments-only wrapper owner",
            Self::Abs => "ABS DC worker",
            Self::Poly => "behavioral POLY DC worker",
            Self::DigCount => "digital-counter transient worker",
            Self::Bug138 => "dependent-source POLY DC worker",
        }
    }

    fn expected_dc(self) -> Option<(&'static str, Value, Value, Value, &'static [&'static str])> {
        match self {
            Self::Abs => Some(("VS", -10.0, 10.0, 1.0, &["V(1)", "V(3)"])),
            Self::Poly | Self::Bug138 => {
                Some(("VINPUT", -4.0, 4.0, 1.0, &["V(1)", "V(2)", "V(3)", "V(4)"]))
            }
            Self::WrapperOwner | Self::DigCount => None,
        }
    }
}

impl XyceTestRunner {
    fn bug141_schema_matches(columns: &[String], expected: &[&str]) -> bool {
        columns.len() == expected.len()
            && columns
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn bug141_dc_diagnostics_match(
        role: Bug141Role,
        diagnostics: &[rspice_core::netlist::ParseDiagnostic],
    ) -> bool {
        match role {
            Bug141Role::Abs => diagnostics.is_empty(),
            Bug141Role::Poly | Bug141Role::Bug138 => {
                diagnostics.len() == 2
                    && diagnostics.iter().all(|diagnostic| {
                        diagnostic.code == "unknown-option"
                            && diagnostic.severity
                                == rspice_core::netlist::DiagnosticSeverity::Warning
                    })
                    && diagnostics[0].message == "unknown .options key 'LINSOL' ignored"
                    && diagnostics[1].message
                        == "unknown .options key 'TR_SINGLETON_FILTER' ignored"
            }
            Bug141Role::WrapperOwner | Bug141Role::DigCount => false,
        }
    }

    fn validate_bug141_dc_worker(
        &self,
        role: Bug141Role,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticDcPlan, Netlist), String> {
        let (expected_source, expected_start, expected_stop, expected_step, expected_probes) = role
            .expected_dc()
            .ok_or_else(|| format!("{LABEL} {} is not a DC worker", role.label()))?;
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !Self::bug141_dc_diagnostics_match(role, &plan.diagnostics)
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case(expected_source)
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != expected_start.to_bits()
            || plan.dc.stop.to_bits() != expected_stop.to_bits()
            || plan.dc.step.to_bits() != expected_step.to_bits()
            || plan.dc.sweep2.is_some()
            || !Self::bug141_schema_matches(&plan.print.probes, expected_probes)
        {
            return Err(format!(
                "{LABEL} {} DC plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.label()))?;
        if !Self::bug141_dc_diagnostics_match(role, &netlist.diagnostics)
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || netlist.options.method.is_some()
            || netlist.options.matrix_solver.is_some()
            || !matches!(
                &netlist.analyses[0],
                AnalysisCommand::Dc {
                    source,
                    start,
                    stop,
                    step,
                    mode: DcSweepMode::Linear,
                    sweep2: None,
                    ..
                } if source.eq_ignore_ascii_case(expected_source)
                    && start.to_bits() == expected_start.to_bits()
                    && stop.to_bits() == expected_stop.to_bits()
                    && step.to_bits() == expected_step.to_bits()
            )
        {
            return Err(format!(
                "{LABEL} {} typed DC envelope changed",
                role.label()
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != expected_probes.len()
        {
            return Err(format!("{LABEL} {} typed .PRINT DC changed", role.label()));
        }
        Ok((plan, netlist))
    }

    fn validate_bug141_dig_count_initial_states(netlist: &Netlist) -> Result<(), String> {
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("{LABEL} digital-counter flattening failed: {error}"))?;
        let expected = [
            ("xs1.xl1.nand2a", 1.0f64),
            ("xs2.xl1.nand2a", 0.0f64),
            ("xs3.xl1.nand2a", 1.0f64),
            ("xs4.xl1.nand2a", 0.0f64),
        ];
        let mut observed = BTreeMap::new();
        for element in &flattened.elements {
            let key = element.name.to_ascii_lowercase();
            if !expected.iter().any(|(name, _)| *name == key) {
                continue;
            }
            let ElementKind::Xspice { model, params, .. } = &element.kind else {
                return Err(format!(
                    "{LABEL} flattened latch member {} is not a typed XSPICE gate",
                    element.name
                ));
            };
            if !model.eq_ignore_ascii_case("xyce_legacy_d_nand") {
                return Err(format!(
                    "{LABEL} flattened latch member {} changed model to {model:?}",
                    element.name
                ));
            }
            let ic = params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("IC"))
                .map(|(_, value)| *value)
                .ok_or_else(|| {
                    format!(
                        "{LABEL} flattened latch member {} lost its IC",
                        element.name
                    )
                })?;
            if observed.insert(key, ic).is_some() {
                return Err(format!(
                    "{LABEL} flattened latch IC census contains a duplicate"
                ));
            }
        }
        for (name, expected_value) in expected {
            let actual = observed.get(name).ok_or_else(|| {
                format!("{LABEL} flattened digital counter lost latch member {name}")
            })?;
            if actual.to_bits() != expected_value.to_bits() {
                return Err(format!(
                    "{LABEL} flattened digital counter latch {name} has IC={actual}, expected {expected_value}"
                ));
            }
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{LABEL} flattened digital-counter latch IC census changed: {observed:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug141_dig_count_worker(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        let expected_probes = [
            "V(TRIGGER)",
            "V(OUT_3)",
            "V(OUT_2)",
            "V(OUT_1)",
            "V(OUT_0)",
            "V(XS1:XL1:TRIGINV)",
        ];
        let print = plan.require_print("BUG141 digital-counter completion")?;
        if plan.deck_path != path
            || plan.source != source
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.tran.step.to_bits() != 10e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 1.2e-6f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || !Self::bug141_schema_matches(&print.probes, &expected_probes)
        {
            return Err(format!(
                "{LABEL} digital-counter transient plan changed: {plan:?}"
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} digital counter no longer parses: {error}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || netlist.options.method.is_some()
            || netlist.options.matrix_solver.is_some()
            || !matches!(
                &netlist.analyses[0],
                AnalysisCommand::Tran {
                    step,
                    stop,
                    start: None,
                    max_step: None,
                    uic: false,
                } if step.to_bits() == 10e-9f64.to_bits()
                    && stop.to_bits() == 1.2e-6f64.to_bits()
            )
        {
            return Err(format!("{LABEL} digital-counter typed envelope changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.dependencies.len() != expected_probes.len()
        {
            return Err(format!("{LABEL} digital-counter typed .PRINT TRAN changed"));
        }
        Self::validate_bug141_dig_count_initial_states(&netlist)?;
        Ok((plan, netlist))
    }

    fn validate_bug141_table(
        role: Bug141Role,
        backend: rspice_core::solver::RealSolverBackend,
        table: &XycePrnTable,
        expected_columns: &[&str],
        expected_rows: Option<usize>,
    ) -> Result<(), String> {
        if table.columns.first().is_none_or(|column| column != "Index")
            || !Self::bug141_schema_matches(&table.columns[1..], expected_columns)
            || table.rows.is_empty()
            || expected_rows.is_some_and(|expected| table.rows.len() != expected)
            || table.rows.iter().enumerate().any(|(index, row)| {
                row.len() != table.columns.len()
                    || row[0].to_bits() != (index as Value).to_bits()
                    || row.iter().any(|value| !value.is_finite())
            })
        {
            return Err(format!(
                "{LABEL} {} {backend:?} produced an incomplete requested PRN table: columns={:?}, rows={}",
                role.label(),
                table.columns,
                table.rows.len()
            ));
        }
        Ok(())
    }

    fn run_bug141_dc_backend(
        &self,
        role: Bug141Role,
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        backend: rspice_core::solver::RealSolverBackend,
        start: Instant,
    ) -> Result<(), String> {
        let run_netlist = netlist.clone();
        let mut config = self.xyce_engine_config(None);
        config.matrix_solver = Some(backend);
        let engine = Engine::new(config);
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let results = engine
            .run_dc_sweep2_spec_with_report_and_abort(
                &run_netlist,
                &plan.dc.source,
                &plan.dc.primary_spec(),
                plan.dc.sweep2.as_ref(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} {} {backend:?} failed: {error}", role.label()))?;
        let (_, _, _, _, probes) = role.expected_dc().expect("DC worker has a DC schema");
        let expected_rows = match role {
            Bug141Role::Abs => 21,
            Bug141Role::Poly | Bug141Role::Bug138 => 9,
            Bug141Role::WrapperOwner | Bug141Role::DigCount => unreachable!(),
        };
        if results.len() != expected_rows
            || results.iter().any(|point| {
                !point.sweep_value.is_finite()
                    || point
                        .result
                        .node_voltages
                        .iter()
                        .chain(point.result.branch_currents.iter())
                        .any(|value| !value.is_finite())
                    || !point.device_op_report.labels_resolve()
            })
        {
            return Err(format!(
                "{LABEL} {} {backend:?} did not complete its finite authored DC sweep",
                role.label()
            ));
        }
        let table = self
            .dc_results_to_prn_table(plan, &run_netlist, &results)
            .map_err(|error| {
                format!(
                    "{LABEL} {} {backend:?} PRN materialization failed: {error}",
                    role.label()
                )
            })?;
        Self::validate_bug141_table(role, backend, &table, probes, Some(expected_rows))
    }

    fn run_bug141_dig_count_backend(
        &self,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        backend: rspice_core::solver::RealSolverBackend,
        start: Instant,
    ) -> Result<(), String> {
        let run_netlist = netlist.clone();
        // This is a completion-only contract with no numerical gold. Preserve
        // Xyce's native DELMAX policy and source/event breakpoints instead of
        // injecting the harness's pointwise-oracle policy of 200 accepted
        // steps per source transition.
        let max_step = Self::transient_max_step_with_solver_ceiling(
            &run_netlist,
            &plan.tran,
            None,
            Self::transient_oracle_solver_max_step_for_netlist(&run_netlist, &plan.tran),
            false,
        )?;
        let mut config = self.xyce_engine_config(None);
        config.matrix_solver = Some(backend);
        config.transient_initial_timestep = Self::xyce_initial_timestep_for_tran(&plan.tran);
        // This completion-only wrapper authored no METHOD. Preserve the
        // Xyce engine configuration's native TrapGear policy instead of
        // inventing a Trapezoidal integration-method oracle.
        let engine = Engine::new(config);
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let result = engine
            .run_tran_with_startup_mode_and_abort(
                &run_netlist,
                plan.tran.stop,
                max_step,
                rspice_core::engine::TransientStartupMode::from_uic(plan.tran.uic),
                &abort,
            )
            .map_err(|error| format!("{LABEL} digital counter {backend:?} failed: {error}"))?;
        let table = Self::transient_family_result_to_prn_table(plan, &run_netlist, &result)
            .map_err(|error| {
                format!("{LABEL} digital counter {backend:?} PRN materialization failed: {error}")
            })?;
        let expected_columns = [
            "TIME",
            "V(TRIGGER)",
            "V(OUT_3)",
            "V(OUT_2)",
            "V(OUT_1)",
            "V(OUT_0)",
            "V(XS1:XL1:TRIGINV)",
        ];
        Self::validate_bug141_table(
            Bug141Role::DigCount,
            backend,
            &table,
            &expected_columns,
            None,
        )?;
        if table.rows.len() < 2
            || table.rows.first().is_none_or(|row| row[1].abs() > 1e-18)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - plan.tran.stop).abs() > 1e-15)
            || table.rows.windows(2).any(|rows| rows[0][1] >= rows[1][1])
        {
            return Err(format!(
                "{LABEL} digital counter {backend:?} did not span the complete nontrivial 0-to-1.2us time domain"
            ));
        }
        Ok(())
    }

    fn bug141_join_backend_cell(
        role: Bug141Role,
        backend: rspice_core::solver::RealSolverBackend,
        joined: std::thread::Result<Result<(), String>>,
    ) -> Result<(), String> {
        match joined {
            Ok(result) => result,
            Err(payload) => {
                let panic = payload.downcast_ref::<String>().map_or_else(
                    || {
                        payload.downcast_ref::<&str>().map_or_else(
                            || "unknown panic payload".to_string(),
                            |message| (*message).to_string(),
                        )
                    },
                    Clone::clone,
                );
                Err(format!(
                    "{LABEL} {} {backend:?} worker thread panicked: {panic}",
                    role.label()
                ))
            }
        }
    }

    fn run_bug141_backend_pair<F>(&self, role: Bug141Role, run: F) -> Result<(), String>
    where
        F: Fn(rspice_core::solver::RealSolverBackend) -> Result<(), String> + Sync,
    {
        let (klu, faer) = std::thread::scope(|scope| {
            let run = &run;
            let klu = scope.spawn(move || run(rspice_core::solver::RealSolverBackend::Klu));
            let faer = scope.spawn(move || run(rspice_core::solver::RealSolverBackend::Faer));
            // Join both independent cells before selecting an error. This
            // preserves the family-wide conjunction and prevents a KLU
            // failure from detaching a still-running Faer simulation.
            let klu = Self::bug141_join_backend_cell(
                role,
                rspice_core::solver::RealSolverBackend::Klu,
                klu.join(),
            );
            let faer = Self::bug141_join_backend_cell(
                role,
                rspice_core::solver::RealSolverBackend::Faer,
                faer.join(),
            );
            (klu, faer)
        });
        // KLU is reported first for deterministic diagnostics, independent of
        // which scoped worker happened to finish first.
        klu?;
        faer?;
        Ok(())
    }

    fn execute_bug141_worker(
        &self,
        role: Bug141Role,
        source: &str,
        start: Instant,
    ) -> Result<(), String> {
        let path = self.root.join(role.path());
        match role {
            Bug141Role::Abs | Bug141Role::Poly | Bug141Role::Bug138 => {
                let (plan, netlist) = self.validate_bug141_dc_worker(role, source, &path)?;
                self.run_bug141_backend_pair(role, |backend| {
                    self.run_bug141_dc_backend(role, &plan, &netlist, backend, start)
                })?;
            }
            Bug141Role::DigCount => {
                let (plan, netlist) = self.validate_bug141_dig_count_worker(source, &path)?;
                self.run_bug141_backend_pair(role, |backend| {
                    self.run_bug141_dig_count_backend(&plan, &netlist, backend, start)
                })?;
            }
            Bug141Role::WrapperOwner => {
                return Err(format!("{LABEL} comments-only owner is not executable"));
            }
        }
        Ok(())
    }

    pub(super) fn validate_bug141_oracle(
        &self,
        role: Bug141Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        for worker in Bug141Role::WORKERS {
            if role != Bug141Role::WrapperOwner && role != worker {
                continue;
            }
            let path = self.root.join(worker.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            self.execute_bug141_worker(worker, &source, start)?;
        }
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
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
    fn bug141_workers_lock_typed_plans_and_latch_initial_states() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in [Bug141Role::Abs, Bug141Role::Poly, Bug141Role::Bug138] {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read canonical BUG141 DC worker");
            runner
                .validate_bug141_dc_worker(role, &source, &path)
                .expect("canonical BUG141 DC worker remains typed");
        }
        let path = root.join(DIG_COUNT_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG141 digital worker");
        let (plan, netlist) = runner
            .validate_bug141_dig_count_worker(&source, &path)
            .expect("canonical BUG141 digital worker preserves latch ICs");
        let completion_max_step = XyceTestRunner::transient_max_step_with_solver_ceiling(
            &netlist,
            &plan.tran,
            None,
            XyceTestRunner::transient_oracle_solver_max_step_for_netlist(&netlist, &plan.tran),
            false,
        )
        .expect("BUG141 completion execution ceiling resolves");
        assert_eq!(
            completion_max_step.to_bits(),
            (0.1 * plan.tran.stop).to_bits(),
            "completion-only BUG141 execution preserves native Xyce DELMAX instead of injecting a source-resolution oracle"
        );
    }

    #[test]
    fn bug141_parallel_backend_join_is_complete_deterministic_and_panic_safe() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let faer_completed = Arc::new(AtomicBool::new(false));
        let faer_observer = Arc::clone(&faer_completed);
        let error = runner
            .run_bug141_backend_pair(Bug141Role::Abs, move |backend| match backend {
                rspice_core::solver::RealSolverBackend::Klu => Err("KLU sentinel".to_string()),
                rspice_core::solver::RealSolverBackend::Faer => {
                    faer_observer.store(true, Ordering::SeqCst);
                    Err("Faer sentinel".to_string())
                }
                rspice_core::solver::RealSolverBackend::Auto => unreachable!(),
            })
            .expect_err("paired backend failures must fail closed");
        assert!(faer_completed.load(Ordering::SeqCst));
        assert_eq!(error, "KLU sentinel");

        let error = runner
            .run_bug141_backend_pair(Bug141Role::Abs, |backend| match backend {
                rspice_core::solver::RealSolverBackend::Klu => Ok(()),
                rspice_core::solver::RealSolverBackend::Faer => {
                    panic!("Faer panic sentinel")
                }
                rspice_core::solver::RealSolverBackend::Auto => unreachable!(),
            })
            .expect_err("a backend worker panic must become a contract error");
        assert!(error.contains("Faer worker thread panicked"));
        assert!(error.contains("Faer panic sentinel"));
    }

    #[test]
    fn bug141_oracle_rejects_an_expired_deadline() {
        let mut runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner.config.max_time_per_test_ms = 1;
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG141 deadline");
        let error = runner
            .validate_bug141_oracle(Bug141Role::WrapperOwner, expired_start)
            .expect_err("an expired BUG141 deadline must fail closed");
        assert!(error.contains("deadline"));
    }
}
