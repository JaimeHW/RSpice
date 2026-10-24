use super::*;

const LABEL: &str = "BUG_1284 transient-restart transmission-line family";

/// Native relational contract reported by the zero-byte wrapper owner.
pub(super) const BUG1284_OWNER_CONTRACT: &str =
    "bug1284_transient_restart_relational_wrapper_owner";
/// Native relational contract reported by each independently qualified worker.
pub(super) const BUG1284_WORKER_CONTRACT: &str = "bug1284_transient_restart_relational_worker";

const RESTART_JOB: &str = "trans_test";
const RESTART_FILE: &str = "trans_test2e-08";
// An authored `N` suffix reads as the double nearest the decimal it writes, so
// the deck's horizons are the decimals themselves (`50N` is 5e-8, not the
// double-rounded product `50.0 * 1e-9`).
const PRINT_STEP: Value = 0.25e-9;
const FIRST_STOP: Value = 20.0e-9;
const FINAL_STOP: Value = 50.0e-9;
const RESTART_INTERVAL: Value = 5.0e-9;
// The restart cadence accumulates (`current + initial_interval`), so the third
// save is 5n + 5n + 5n = 1.5000000000000002e-8, not the decimal 15n.
const HISTORICAL_SAVE_TIMES: [Value; 5] = [
    0.0,
    RESTART_INTERVAL,
    RESTART_INTERVAL + RESTART_INTERVAL,
    RESTART_INTERVAL + RESTART_INTERVAL + RESTART_INTERVAL,
    RESTART_INTERVAL + RESTART_INTERVAL + RESTART_INTERVAL + RESTART_INTERVAL,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1284Role {
    WrapperOwner,
    Baseline,
    First,
    Restarted,
}

impl Bug1284Role {
    pub(super) const ALL: [Self; 4] = [
        Self::WrapperOwner,
        Self::Baseline,
        Self::First,
        Self::Restarted,
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => BUG1284_OWNER_CONTRACT,
            Self::Baseline | Self::First | Self::Restarted => BUG1284_WORKER_CONTRACT,
        }
    }

    pub(super) const fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => "Netlists/Certification_Tests/BUG_1284/bug_1284.cir",
            Self::Baseline => "Netlists/Certification_Tests/BUG_1284/bug_1284_baseline.cir",
            Self::First => "Netlists/Certification_Tests/BUG_1284/bug_1284_first.cir",
            Self::Restarted => "Netlists/Certification_Tests/BUG_1284/bug_1284_restarted.cir",
        }
    }

    pub(super) fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => "netlists/certification_tests/bug_1284/bug_1284.cir",
            Self::Baseline => "netlists/certification_tests/bug_1284/bug_1284_baseline.cir",
            Self::First => "netlists/certification_tests/bug_1284/bug_1284_first.cir",
            Self::Restarted => "netlists/certification_tests/bug_1284/bug_1284_restarted.cir",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg(test)]
pub(super) struct Bug1284RestartOracle {
    pub print_step: Value,
    pub first_stop: Value,
    pub final_stop: Value,
    pub restart_interval: Value,
    pub save_times: [Value; 5],
    pub restart_job: &'static str,
    pub restart_file: &'static str,
}

impl XyceTestRunner {
    #[cfg(test)]
    pub(super) fn bug1284_restart_oracle() -> Bug1284RestartOracle {
        Bug1284RestartOracle {
            print_step: PRINT_STEP,
            first_stop: FIRST_STOP,
            final_stop: FINAL_STOP,
            restart_interval: RESTART_INTERVAL,
            save_times: HISTORICAL_SAVE_TIMES,
            restart_job: RESTART_JOB,
            restart_file: RESTART_FILE,
        }
    }

    fn bug1284_worker_plan(
        &self,
        role: Bug1284Role,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if role == Bug1284Role::WrapperOwner {
            return Err(format!("{LABEL} wrapper owner is not a simulation deck"));
        }
        let deck = XyceDeck {
            path: self.root.join(role.path()),
            relative_path: role.path().to_string(),
            section: XyceDeckSection::Netlists,
        };
        let plan = self.static_tran_plan_for_deck_with_purpose(
            &deck,
            XyceStaticTranPlanPurpose::Bug1284TransientRestartRelationalFamily,
        )?;
        let netlist = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .map_err(|error| format!("{LABEL} {role:?} parse failed: {error}"))?;
        let print = plan.require_print(LABEL)?;
        let expected_stop = if role == Bug1284Role::First {
            FIRST_STOP
        } else {
            FINAL_STOP
        };
        let restart_is_exact = match (role, netlist.options.restart.as_ref()) {
            (Bug1284Role::Baseline, None) => true,
            (Bug1284Role::First, Some(restart)) => {
                restart.pack.is_none()
                    && restart.print_timeint_options.is_none()
                    && restart.job.as_deref() == Some(RESTART_JOB)
                    && restart.start_time.is_none()
                    && restart.file.is_none()
                    && restart.initial_interval.map(Value::to_bits)
                        == Some(RESTART_INTERVAL.to_bits())
                    && restart.intervals.is_empty()
            }
            (Bug1284Role::Restarted, Some(restart)) => {
                restart.pack.is_none()
                    && restart.print_timeint_options.is_none()
                    && restart.job.is_none()
                    && restart.start_time.is_none()
                    && restart.file.as_deref() == Some(RESTART_FILE)
                    && restart.initial_interval.is_none()
                    && restart.intervals.is_empty()
            }
            _ => false,
        };
        if plan.deck_path != deck.path
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.tran.step.to_bits() != PRINT_STEP.to_bits()
            || plan.tran.stop.to_bits() != expected_stop.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || print.probes.iter().map(String::as_str).ne(["V(2)", "V(3)"])
            || !restart_is_exact
        {
            return Err(format!(
                "{LABEL} {role:?} parsed plan/restart semantics changed: plan={plan:?}, restart={:?}",
                netlist.options.restart
            ));
        }
        Ok((plan, netlist))
    }

    fn bug1284_round_trip_checkpoint(
        checkpoint: &TransientCheckpoint,
    ) -> Result<TransientCheckpoint, String> {
        let text = checkpoint.to_text();
        let restored = TransientCheckpoint::from_text(&text)
            .map_err(|error| format!("{LABEL} checkpoint parse failed: {error}"))?;
        if restored != *checkpoint || restored.to_text() != text {
            return Err(format!(
                "{LABEL} checkpoint at t={:.17e} did not round-trip bit-exactly",
                checkpoint.time
            ));
        }
        Ok(restored)
    }

    fn bug1284_require_result_horizon(
        role: &str,
        result: &TransientResult,
        start: Value,
        stop: Value,
    ) -> Result<(), String> {
        Self::validate_transient_result_time_grid(result)?;
        let actual_start = result
            .time
            .first()
            .copied()
            .ok_or_else(|| format!("{LABEL} {role} produced no samples"))?;
        let actual_stop = result.time.last().copied().expect("checked nonempty");
        if actual_start.to_bits() != start.to_bits() || actual_stop.to_bits() != stop.to_bits() {
            return Err(format!(
                "{LABEL} {role} horizon changed: [{actual_start:.17e}, {actual_stop:.17e}], expected [{start:.17e}, {stop:.17e}]"
            ));
        }
        Ok(())
    }

    fn bug1284_compare_restart_relation(
        &self,
        label: &str,
        baseline: &XycePrnTable,
        restarted: &XycePrnTable,
    ) -> Result<(), String> {
        let mismatches = self.compare_bug1284_restart_tables(baseline, restarted)?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} {label} produced {} default xyce_verify mismatch(es): {mismatches:?}",
                mismatches.len()
            ))
        }
    }

    pub(super) fn validate_bug1284_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let (baseline_plan, baseline_netlist) = self.bug1284_worker_plan(Bug1284Role::Baseline)?;
        let (first_plan, first_netlist) = self.bug1284_worker_plan(Bug1284Role::First)?;
        let (restarted_plan, restarted_netlist) =
            self.bug1284_worker_plan(Bug1284Role::Restarted)?;

        let baseline_max_step =
            Self::transient_family_max_step(&baseline_netlist, &baseline_plan.tran)?;
        let first_max_step = Self::transient_family_max_step(&first_netlist, &first_plan.tran)?;
        let startup_mode = TransientStartupMode::from_uic(false);
        let baseline_engine = self.create_xyce_static_tran_engine(
            None,
            Self::xyce_initial_timestep_for_tran(&baseline_plan.tran),
        );
        let baseline_result = baseline_engine
            .run_tran_with_startup_mode_and_abort(
                &baseline_netlist,
                FINAL_STOP,
                baseline_max_step,
                startup_mode,
                &abort,
            )
            .map_err(|error| format!("{LABEL} baseline run failed: {error}"))?;
        Self::bug1284_require_result_horizon("baseline", &baseline_result, 0.0, FINAL_STOP)?;

        let first_engine = self.create_xyce_static_tran_engine(
            None,
            Self::xyce_initial_timestep_for_tran(&first_plan.tran),
        );
        let (first_result, checkpoints) = first_engine
            .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                &first_netlist,
                FIRST_STOP,
                first_max_step,
                startup_mode,
                &HISTORICAL_SAVE_TIMES,
                &abort,
            )
            .map_err(|error| format!("{LABEL} first/checkpoint run failed: {error}"))?;
        Self::bug1284_require_result_horizon("first", &first_result, 0.0, FIRST_STOP)?;
        if checkpoints.len() != HISTORICAL_SAVE_TIMES.len()
            || checkpoints
                .iter()
                .map(|scheduled| scheduled.nominal_time.to_bits())
                .ne(HISTORICAL_SAVE_TIMES.map(Value::to_bits))
        {
            return Err(format!(
                "{LABEL} nominal checkpoint schedule changed: {:?}",
                checkpoints
                    .iter()
                    .map(|scheduled| scheduled.nominal_time)
                    .collect::<Vec<_>>()
            ));
        }
        for (index, scheduled) in checkpoints.iter().enumerate() {
            let actual = scheduled.checkpoint.time;
            let next_nominal = HISTORICAL_SAVE_TIMES.get(index + 1).copied();
            if !actual.is_finite()
                || actual < scheduled.nominal_time
                || next_nominal.is_some_and(|next| actual >= next)
                || actual > FIRST_STOP
            {
                return Err(format!(
                    "{LABEL} checkpoint for nominal {:.17e} was captured outside its accepted-step window at {actual:.17e}; next nominal={next_nominal:?}",
                    scheduled.nominal_time
                ));
            }
        }

        let checkpoint_at = |time: Value| -> Result<TransientCheckpoint, String> {
            let scheduled = checkpoints
                .iter()
                .find(|scheduled| scheduled.nominal_time.to_bits() == time.to_bits())
                .ok_or_else(|| format!("{LABEL} omitted checkpoint at {time:.17e}"))?;
            Self::bug1284_round_trip_checkpoint(&scheduled.checkpoint)
        };
        let historical_checkpoint = checkpoint_at(FIRST_STOP)?;
        let strengthened_checkpoint = checkpoint_at(RESTART_INTERVAL)?;
        let resume =
            |label: &str, checkpoint: &TransientCheckpoint| -> Result<TransientResult, String> {
                let engine = self.create_xyce_static_tran_engine(
                    None,
                    Self::xyce_initial_timestep_for_tran(&restarted_plan.tran),
                );
                let (result, final_checkpoint) = engine
                    .run_tran_restart_resume_with_abort(
                        &restarted_netlist,
                        checkpoint,
                        FINAL_STOP,
                        first_max_step,
                        &abort,
                    )
                    .map_err(|error| format!("{LABEL} {label} resume failed: {error}"))?;
                let final_checkpoint = final_checkpoint
                    .into_checkpoint()
                    .map_err(|error| format!("{LABEL} {label} resume ended: {error}"))?;
                Self::bug1284_require_result_horizon(label, &result, checkpoint.time, FINAL_STOP)?;
                if final_checkpoint.time.to_bits() != FINAL_STOP.to_bits() {
                    return Err(format!(
                        "{LABEL} {label} final checkpoint stopped at {:.17e}",
                        final_checkpoint.time
                    ));
                }
                Ok(result)
            };
        let historical_result = resume("historical 20 ns", &historical_checkpoint)?;
        let strengthened_result = resume("strengthened 5 ns", &strengthened_checkpoint)?;

        let baseline_table = Self::transient_family_result_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_result,
        )?;
        let historical_table = Self::transient_family_result_to_prn_table(
            &restarted_plan,
            &restarted_netlist,
            &historical_result,
        )?;
        let strengthened_table = Self::transient_family_result_to_prn_table(
            &restarted_plan,
            &restarted_netlist,
            &strengthened_result,
        )?;
        self.bug1284_compare_restart_relation(
            "historical 20 ns relation",
            &baseline_table,
            &historical_table,
        )?;
        self.bug1284_compare_restart_relation(
            "strengthened nonquiescent 5 ns relation",
            &baseline_table,
            &strengthened_table,
        )?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded its deadline"));
        }
        Ok(())
    }

    /// Apply the unmodified Release-7.10 default `xyce_verify` transient metric.
    /// The baseline is the GOOD table and the resumed tail is the TEST table.
    pub(super) fn compare_bug1284_restart_tables(
        &self,
        baseline: &XycePrnTable,
        restarted: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let expected_columns = ["Index", "TIME", "V(2)", "V(3)"];
        for (label, table) in [("baseline", baseline), ("restarted", restarted)] {
            if table
                .columns
                .iter()
                .map(String::as_str)
                .ne(expected_columns)
            {
                return Err(format!(
                    "{LABEL} {label} projection changed: {:?}",
                    table.columns
                ));
            }
        }
        self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            baseline,
            restarted,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )
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
    fn bug1284_roles_paths_and_future_contracts_are_bijective() {
        let records = Bug1284Role::ALL
            .into_iter()
            .map(Bug1284Role::record)
            .collect::<BTreeSet<_>>();
        assert_eq!(records.len(), Bug1284Role::ALL.len());
        for role in Bug1284Role::ALL {
            assert_eq!(Bug1284Role::for_record(role.path()), Some(role));
            assert_eq!(Bug1284Role::for_record(role.record()), Some(role));
            assert_eq!(
                role.contract(),
                if role == Bug1284Role::WrapperOwner {
                    BUG1284_OWNER_CONTRACT
                } else {
                    BUG1284_WORKER_CONTRACT
                }
            );
        }
        assert_eq!(Bug1284Role::for_record("BUG_1284/invented.cir"), None);
    }

    #[test]
    fn bug1284_historical_restart_schedule_and_filename_are_exact() {
        let oracle = XyceTestRunner::bug1284_restart_oracle();
        assert_eq!(oracle.print_step.to_bits(), 0.25e-9f64.to_bits());
        assert_eq!(oracle.first_stop.to_bits(), 20.0e-9f64.to_bits());
        assert_eq!(oracle.final_stop.to_bits(), 50.0e-9f64.to_bits());
        assert_eq!(oracle.restart_interval.to_bits(), 5.0e-9f64.to_bits());
        // The accumulated cadence: the third save is one ulp above 15n.
        assert_eq!(
            oracle.save_times.map(Value::to_bits),
            [0.0, 5.0e-9, 10.0e-9, 1.500_000_000_000_000_2e-8, 20.0e-9].map(Value::to_bits)
        );
        assert_eq!(oracle.restart_job, "trans_test");
        assert_eq!(oracle.restart_file, "trans_test2e-08");
    }

    #[test]
    fn bug1284_restart_options_cannot_masquerade_as_analytic_timeint_only_options() {
        let path = corpus_root().join(Bug1284Role::First.path());
        let source = fs::read_to_string(&path).expect("read BUG1284 first-run deck");
        let netlist = XyceTestRunner::parse_xyce_netlist(&source, &path)
            .expect("parse BUG1284 first-run deck");
        assert!(netlist.options.restart.is_some());
        assert!(!XyceTestRunner::analytic_timeint_only_options_match(
            &netlist.options,
            None,
            None,
            None,
            None,
        ));
    }

    #[test]
    fn bug1284_default_integrated_rms_oracle_accepts_equal_tail_and_rejects_history_loss() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let columns = ["Index", "TIME", "V(2)", "V(3)"]
            .map(str::to_string)
            .to_vec();
        let baseline = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 0.0, 0.0, 0.0],
                vec![1.0, 5.0e-9, 2.5, 0.0],
                vec![2.0, 10.0e-9, 2.5, 2.5],
                vec![3.0, 20.0e-9, 0.0, 2.5],
                vec![4.0, 50.0e-9, 0.0, 0.0],
            ],
        };
        let restarted = XycePrnTable {
            columns: columns.clone(),
            rows: baseline.rows[2..].to_vec(),
        };
        assert!(
            runner
                .compare_bug1284_restart_tables(&baseline, &restarted)
                .expect("compare exact resumed tail")
                .is_empty()
        );

        let mut history_lost = restarted;
        history_lost.rows[0][3] = 0.0;
        history_lost.rows[1][3] = 0.0;
        assert!(
            !runner
                .compare_bug1284_restart_tables(&baseline, &history_lost)
                .expect("compare lost transmission-line history")
                .is_empty()
        );

        let malformed = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(2)".into()],
            rows: vec![vec![0.0, 0.0, 0.0], vec![1.0, 1.0, 0.0]],
        };
        assert!(
            runner
                .compare_bug1284_restart_tables(&baseline, &malformed)
                .is_err()
        );
    }
}
