use super::*;

const LABEL: &str = "BUG_1035_SON simple RC AC DATA equivalence";
const OWNER_CONTRACT: &str = "bug1035_simple_rc_ac_data_relational_wrapper_owner";
const REFERENCE_CONTRACT: &str = "bug1035_simple_rc_ac_data_baseline_reference";
const GRID: [Value; 9] = [1.0, 10.0, 100.0, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1035SonRole {
    DataOwner,
    BaselineReference,
}

impl Bug1035SonRole {
    const ALL: [Self; 2] = [Self::DataOwner, Self::BaselineReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| record == role.record())
    }

    fn path(self) -> &'static str {
        match self {
            Self::DataOwner => "Netlists/Certification_Tests/BUG_1035_SON/RC_simple_data.cir",
            Self::BaselineReference => {
                "Netlists/Certification_Tests/BUG_1035_SON/RC_simple_baseline.cir"
            }
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::DataOwner => "netlists/certification_tests/bug_1035_son/rc_simple_data.cir",
            Self::BaselineReference => {
                "netlists/certification_tests/bug_1035_son/rc_simple_baseline.cir"
            }
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::DataOwner => OWNER_CONTRACT,
            Self::BaselineReference => REFERENCE_CONTRACT,
        }
    }
}

#[derive(Debug)]
struct Bug1035Worker {
    plan: XyceRelationalAcPlan,
    netlist: Netlist,
}

impl XyceTestRunner {
    fn bug1035_worker(
        &self,
        role: Bug1035SonRole,
        raw: &[u8],
        abort: &dyn AbortSignal,
    ) -> Result<Bug1035Worker, String> {
        let source = std::str::from_utf8(raw)
            .map_err(|error| format!("{LABEL} source is not UTF-8: {error}"))?
            .to_string();
        if source.lines().any(|line| {
            let line = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            line.starts_with(".inc") || line.starts_with(".include") || line.starts_with(".lib")
        }) {
            return Err(format!(
                "{LABEL} unexpectedly acquired a filesystem dependency"
            ));
        }
        let path = self.root.join(role.path());
        let output = Self::canonical_print_output_request(&source, "AC", false)?
            .ok_or_else(|| format!("{LABEL} lost PRINT AC"))?;
        if output.format.is_some() || output.file.is_some() || output.probes != ["v(1)"] {
            return Err(format!("{LABEL} {role:?} PRINT schema changed"));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        };
        let netlist =
            Netlist::parse_with_path_and_options_and_abort(&source, &path, options, abort)
                .map_err(|error| format!("{LABEL} {role:?} parse failed: {error}"))?;
        let ac = Self::single_ac_analysis(&netlist)?;
        if !Self::step_commands(&netlist)?.is_empty() {
            return Err(format!("{LABEL} {role:?} acquired a STEP analysis"));
        }
        let plan = XyceRelationalAcPlan {
            deck_path: path,
            source,
            print: XycePrintRequest {
                probes: output.probes,
            },
            ac,
            frequency_bound: false,
        };
        Self::validate_bug1035_worker(role, &plan, &netlist)?;
        Ok(Bug1035Worker { plan, netlist })
    }

    fn validate_bug1035_worker(
        role: Bug1035SonRole,
        plan: &XyceRelationalAcPlan,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if plan.print.probes != ["v(1)"]
            || plan.ac.frequencies.len() != GRID.len()
            || plan
                .ac
                .frequencies
                .iter()
                .zip(GRID)
                .any(|(a, b)| (*a - b).abs() > b * 64.0 * f64::EPSILON)
        {
            return Err(format!("{LABEL} {role:?} plan/grid changed"));
        }
        if netlist.elements.len() != 3
            || netlist.output_requests.len() != 1
            || netlist.analyses.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(format!("{LABEL} {role:?} typed envelope changed"));
        }
        let find = |name: &str| {
            netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
        };
        let nodes = |element: &rspice_core::netlist::Element, expected: &[&str]| {
            element
                .nodes
                .iter()
                .map(String::as_str)
                .eq(expected.iter().copied())
        };
        let source = find("Isrc");
        let resistor = find("R1");
        let capacitor = find("C1");
        if !source.is_some_and(|e| e.provenance == ElementProvenance::Authored && nodes(e, &["1","0"]) && matches!(&e.kind, ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::Ac { magnitude, phase }) if magnitude.to_bits()==1.0f64.to_bits() && phase.to_bits()==0.0f64.to_bits()))
            || !resistor.is_some_and(|e| e.provenance == ElementProvenance::Authored && nodes(e, &["1","0"]) && matches!(&e.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params } if value.to_bits()==1000.0f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty()))
            || !capacitor.is_some_and(|e| e.provenance == ElementProvenance::Authored && nodes(e, &["1","0"]) && matches!(&e.kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params } if value.to_bits()==2e-6f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())) {
            return Err(format!("{LABEL} {role:?} topology changed"));
        }
        match role {
            Bug1035SonRole::BaselineReference if plan.ac.data_points().is_none() && netlist.data_tables.is_empty() => {}
            Bug1035SonRole::DataOwner if plan.ac.data_points().is_some_and(|points| points.len()==9 && points.iter().zip(GRID).all(|(point, frequency)| point.frequency.to_bits()==frequency.to_bits() && matches!(point.overrides.as_slice(), [(name, value)] if name.eq_ignore_ascii_case("FREQ") && value.to_bits()==frequency.to_bits()))) && matches!(netlist.data_tables.as_slice(), [table] if table.name.eq_ignore_ascii_case("eric") && table.params == ["FREQ"] && table.rows.len()==9 && table.rows.iter().zip(GRID).all(|(row, f)| row.as_slice()==[f])) => {}
            _ => return Err(format!(
                "{LABEL} {role:?} AC DATA representation changed: points={:?}, tables={:?}",
                plan.ac.data_points(), netlist.data_tables
            )),
        }
        Ok(())
    }

    fn run_bug1035_worker(
        &self,
        role: Bug1035SonRole,
        worker: &Bug1035Worker,
        abort: &dyn AbortSignal,
    ) -> Result<XycePrnTable, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before {role:?}"));
        }
        let engine = self.create_xyce_engine();
        let table = match role {
            Bug1035SonRole::BaselineReference => {
                let results = engine
                    .run_ac_with_abort(&worker.netlist, &worker.plan.ac.frequencies, abort)
                    .map_err(|error| format!("{LABEL} baseline failed: {error}"))?;
                Self::ac_family_result_to_prn_table(&worker.plan.print, &worker.netlist, &results)?
            }
            Bug1035SonRole::DataOwner => {
                let (rows, results) = engine
                    .run_ac_data_with_abort(&worker.netlist, "eric", abort)
                    .map_err(|error| format!("{LABEL} DATA owner failed: {error}"))?;
                let points = rows
                    .into_iter()
                    .zip(results)
                    .map(|(netlist, result)| XyceAcDataPointResult { netlist, result })
                    .collect::<Vec<_>>();
                Self::ac_family_data_points_to_prn_table(&worker.plan.print, &points)?
            }
        };
        Self::validate_bug1035_table(role, &table)?;
        Ok(table)
    }

    fn validate_bug1035_table(role: Bug1035SonRole, table: &XycePrnTable) -> Result<(), String> {
        if table.columns != ["Index", "FREQ", "Re(v(1))", "Im(v(1))"] || table.rows.len() != 9 {
            return Err(format!(
                "{LABEL} {role:?} table schema changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        for (index, (row, frequency)) in table.rows.iter().zip(GRID).enumerate() {
            let b = std::f64::consts::TAU * frequency * 2e-6;
            let denominator = 1e-6 + b * b;
            let expected_re = -0.001 / denominator;
            let expected_im = b / denominator;
            // The relational surface is the historical default PRN stream,
            // whose scientific fields carry eight fractional digits. Bind
            // the independently derived complex response within one
            // conservative final printed digit, far inside ACComparator.
            if row.len() != 4
                || row.iter().any(|v| !v.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || (row[1] - frequency).abs() > frequency * 64.0 * f64::EPSILON
                || (row[2] - expected_re).abs() > 5.1e-9 * expected_re.abs()
                || (row[3] - expected_im).abs() > 5.1e-9 * expected_im.abs()
            {
                return Err(format!(
                    "{LABEL} {role:?} analytic row {index} failed: {row:?}"
                ));
            }
        }
        Ok(())
    }

    fn validate_bug1035_relation(
        &self,
        baseline: &XycePrnTable,
        owner: &XycePrnTable,
    ) -> Result<(), String> {
        Self::validate_bug1035_table(Bug1035SonRole::BaselineReference, baseline)?;
        Self::validate_bug1035_table(Bug1035SonRole::DataOwner, owner)?;
        let good =
            Self::xyce_legacy_compact_prn_for_comparison(baseline, &PrintDelimiter::Whitespace)?;
        let test =
            Self::xyce_legacy_compact_prn_for_comparison(owner, &PrintDelimiter::Whitespace)?;
        if good.as_bytes() == test.as_bytes() {
            return Ok(());
        }
        let tolerance = XyceAcComparatorTolerance::new(6e-5, 1e-4, 1e-6, 1e-6)?;
        let mismatches =
            self.compare_ac_comparator_tables_with_tolerance(baseline, owner, tolerance)?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} directional ACComparator found {} mismatch(es)",
                mismatches.len()
            ))
        }
    }

    pub(super) fn validate_bug1035_son_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let source = |role: Bug1035SonRole| {
            fs::read(self.root.join(role.path()))
                .map_err(|error| format!("{LABEL} cannot read {role:?}: {error}"))
        };
        let baseline = self.bug1035_worker(
            Bug1035SonRole::BaselineReference,
            &source(Bug1035SonRole::BaselineReference)?,
            &abort,
        )?;
        let owner = self.bug1035_worker(
            Bug1035SonRole::DataOwner,
            &source(Bug1035SonRole::DataOwner)?,
            &abort,
        )?;
        let baseline_table =
            self.run_bug1035_worker(Bug1035SonRole::BaselineReference, &baseline, &abort)?;
        let owner_table = self.run_bug1035_worker(Bug1035SonRole::DataOwner, &owner, &abort)?;
        self.validate_bug1035_relation(&baseline_table, &owner_table)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("tests/xyce"),
            Default::default(),
        )
    }
    #[test]
    fn bug1035_exact_pair_executes_and_matches_analytic_oracle() {
        let runner = runner();
        runner.validate_bug1035_son_oracle(Instant::now()).unwrap();
    }
    #[test]
    fn bug1035_data_grid_and_topology_mutations_fail() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30000);
        let raw = fs::read(runner.root.join(Bug1035SonRole::DataOwner.path())).unwrap();
        let text = String::from_utf8(raw).unwrap();
        for changed in [
            text.replacen("1.00000000e+04", "2.00000000e+04", 1),
            text.replacen("C1 1 0 2e-6", "C1 1 0 3e-6", 1),
            text.replacen("v(1)", "v(0)", 1),
        ] {
            assert!(
                runner
                    .bug1035_worker(Bug1035SonRole::DataOwner, changed.as_bytes(), &abort)
                    .is_err()
            );
        }
    }
    #[test]
    fn bug1035_relation_uses_directional_comparator_fallback_but_rejects_shared_wrong() {
        let runner = runner();
        let rows = GRID
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                let b = std::f64::consts::TAU * f * 2e-6;
                let d = 1e-6 + b * b;
                vec![i as Value, f, -0.001 / d, b / d]
            })
            .collect();
        let baseline = XycePrnTable {
            columns: vec![
                "Index".into(),
                "FREQ".into(),
                "Re(v(1))".into(),
                "Im(v(1))".into(),
            ],
            rows,
        };
        let mut close = baseline.clone();
        close.rows[8][2] += 1e-18;
        let baseline_bytes = XyceTestRunner::xyce_legacy_compact_prn_for_comparison(
            &baseline,
            &PrintDelimiter::Whitespace,
        )
        .unwrap();
        let close_bytes = XyceTestRunner::xyce_legacy_compact_prn_for_comparison(
            &close,
            &PrintDelimiter::Whitespace,
        )
        .unwrap();
        assert_ne!(
            baseline_bytes, close_bytes,
            "counterfactual must reach historical comparator fallback"
        );
        runner.validate_bug1035_relation(&baseline, &close).unwrap();
        let tolerance = XyceAcComparatorTolerance::new(6e-5, 1e-4, 1e-6, 1e-6).unwrap();
        let mut directional_test = baseline.clone();
        directional_test.rows[8][3] = baseline.rows[8][3] * (1.0 - 0.000_099_995);
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &baseline,
                    &directional_test,
                    tolerance,
                )
                .unwrap()
                .is_empty(),
            "Release baseline-GOOD, DATA-TEST ordering must pass"
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &directional_test,
                    &baseline,
                    tolerance,
                )
                .unwrap()
                .is_empty(),
            "reversing ACComparator's directional denominator must fail"
        );
        let mut wrong = baseline.clone();
        wrong.rows[2][2] += 1.0;
        assert!(runner.validate_bug1035_relation(&wrong, &wrong).is_err());
    }
    #[test]
    fn bug1035_release_comparator_boundaries_are_exact() {
        let runner = runner();
        let table = |frequency: Value, value: Value| XycePrnTable {
            columns: vec!["Index".into(), "FREQ".into(), "VALUE".into()],
            rows: vec![vec![0.0, frequency, value]],
        };

        let absolute = XyceAcComparatorTolerance::new(0.125, 0.25, 0.0, 0.125).unwrap();
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 1.0),
                    &table(1.0, 1.125),
                    absolute,
                )
                .unwrap()
                .is_empty()
        );

        let relative = XyceAcComparatorTolerance::new(0.5, 0.25, 0.0, 0.125).unwrap();
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 1.0),
                    &table(1.0, 1.25),
                    relative,
                )
                .unwrap()
                .is_empty()
        );

        let zero = XyceAcComparatorTolerance::new(0.125, 1.0, 0.5, 0.125).unwrap();
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 0.25),
                    &table(1.0, 0.5 - f64::EPSILON),
                    zero,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 0.25),
                    &table(1.0, 0.5),
                    zero,
                )
                .unwrap()
                .is_empty()
        );

        let frequency = XyceAcComparatorTolerance::new(0.125, 0.25, 0.0, 0.125).unwrap();
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 1.0),
                    &table(1.125, 1.0),
                    frequency,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 1.0),
                    &table(f64::from_bits(1.125f64.to_bits() + 1), 1.0),
                    frequency,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(0.0, 1.0),
                    &table(-1.0, 1.0),
                    frequency,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(0.0, 1.0),
                    &table(0.25, 1.0),
                    frequency,
                )
                .unwrap()
                .is_empty()
        );
    }
    #[test]
    fn bug1035_data_execution_honors_abort_during_engine_entry() {
        let runner = runner();
        let raw = fs::read(runner.root.join(Bug1035SonRole::DataOwner.path())).unwrap();
        let worker = runner
            .bug1035_worker(
                Bug1035SonRole::DataOwner,
                &raw,
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
        assert!(matches!(
            runner.create_xyce_engine().run_ac_data_with_abort(
                &worker.netlist,
                "eric",
                &rspice_core::abort_signal::ImmediateAbort
            ),
            Err(SimulationError::Aborted)
        ));
    }
    #[test]
    fn bug1035_expired_deadline_fails_before_preparation() {
        let canonical = runner();
        let runner = XyceTestRunner::new(
            &canonical.root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..Default::default()
            },
        );
        assert!(
            runner
                .validate_bug1035_son_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }
}
