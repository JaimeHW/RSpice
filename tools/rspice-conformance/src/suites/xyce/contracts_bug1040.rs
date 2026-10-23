use super::*;

const BUG1040_LABEL: &str = "BUG_1040_SON NOOP/operating-point equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Bug1040Worker {
    OperatingPoint,
    Noop,
}

impl Bug1040Worker {
    const ALL: [Self; 2] = [Self::OperatingPoint, Self::Noop];

    fn path(self) -> &'static str {
        match self {
            Self::OperatingPoint => XYCE_BUG1040_OP_PATH,
            Self::Noop => XYCE_BUG1040_NOOP_PATH,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::OperatingPoint => "operating-point worker",
            Self::Noop => "NOOP worker",
        }
    }

    fn uses_noop(self) -> bool {
        matches!(self, Self::Noop)
    }
}

impl XyceTestRunner {
    fn bug1040_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug1040_model(model: &rspice_core::netlist::ModelDef) -> bool {
        let expected: [(&str, Value); 14] = [
            ("IS", 4.0e-10),
            ("RS", 0.105),
            ("N", 1.48),
            ("TT", 8.0e-7),
            ("CJO", 1.95e-11),
            ("VJ", 0.4),
            ("M", 0.38),
            ("EG", 1.36),
            ("XTI", -8.0),
            ("KF", 0.0),
            ("AF", 1.0),
            ("FC", 0.9),
            ("BV", 600.0),
            ("IBV", 1.0e-4),
        ];
        model.name.eq_ignore_ascii_case("D1N3940")
            && model.model_type.eq_ignore_ascii_case("D")
            && model.params.len() == expected.len()
            && expected.iter().all(|(expected_name, expected_value)| {
                model.params.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case(expected_name)
                        && value.to_bits() == expected_value.to_bits()
                })
            })
            && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
    }

    fn validate_bug1040_worker(
        &self,
        role: Bug1040Worker,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        let expected_probes = ["V(1)", "V(2)"];
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != expected_probes.len()
                    || print
                        .probes
                        .iter()
                        .zip(expected_probes)
                        .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            })
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 2.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic != role.uses_noop()
        {
            return Err(format!(
                "{BUG1040_LABEL} {} plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG1040_LABEL} {} no longer parses: {error}", role.label())
        })?;
        if netlist.elements.len() != 3
            || netlist.models.len() != 1
            || !Self::validate_bug1040_model(&netlist.models[0])
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{BUG1040_LABEL} {} typed envelope changed: elements={:?}, models={:?}, analyses={:?}, outputs={:?}, diagnostics={:?}",
                role.label(),
                netlist.elements,
                netlist.models,
                netlist.analyses,
                netlist.output_requests,
                netlist.diagnostics
            ));
        }
        let [resistor, diode, capacitor] = netlist.elements.as_slice() else {
            unreachable!("BUG1040 element count was checked")
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::bug1040_nodes_match(&resistor.nodes, &["1", "2"])
            || resistor.provenance != ElementProvenance::Authored
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1_000.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !diode.name.eq_ignore_ascii_case("D1")
            || !Self::bug1040_nodes_match(&diode.nodes, &["2", "0"])
            || diode.provenance != ElementProvenance::Authored
            || !matches!(&diode.kind, ElementKind::Diode {
                model,
                instance_params,
                deferred_params,
            } if model.eq_ignore_ascii_case("D1N3940")
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !capacitor.name.eq_ignore_ascii_case("C1")
            || !Self::bug1040_nodes_match(&capacitor.nodes, &["1", "0"])
            || capacitor.provenance != ElementProvenance::Authored
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: Some(initial_voltage),
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0e-6f64.to_bits()
                && initial_voltage.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!(
                "{BUG1040_LABEL} {} topology changed: {:?}",
                role.label(),
                netlist.elements
            ));
        }
        Ok(plan)
    }

    fn run_bug1040_worker(
        &self,
        role: Bug1040Worker,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|error| format!("{BUG1040_LABEL} {} failed: {error}", role.label()))?;
        let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|error| format!("{BUG1040_LABEL} {} PRN failed: {error}", role.label()))?;
        let max_voltage = table
            .rows
            .iter()
            .flat_map(|row| row.get(2..4).into_iter().flatten())
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        let final_voltage = table
            .rows
            .last()
            .and_then(|row| row.get(2))
            .copied()
            .unwrap_or(Value::NAN);
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(1)")
            || !table.columns[3].eq_ignore_ascii_case("V(2)")
            || table.rows.len() < 3
            || table
                .rows
                .iter()
                .any(|row| row.len() != 4 || row.iter().any(|value| !value.is_finite()))
            || table.rows.first().is_none_or(|row| row[1].abs() > 1.0e-15)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 2.0e-3).abs() > 1.0e-12)
            || max_voltage < 0.5
            || final_voltage.abs() >= max_voltage
        {
            return Err(format!(
                "{BUG1040_LABEL} {} produced an invalid or trivial table: columns={:?}, rows={}, max={max_voltage}, final={final_voltage}",
                role.label(),
                table.columns,
                table.rows.len()
            ));
        }
        Ok(table)
    }

    pub(super) fn validate_bug1040_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1040_LABEL} deadline expired before validation"
            ));
        }
        let mut outputs = BTreeMap::new();
        for role in Bug1040Worker::ALL {
            let path = self.root.join(role.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let plan = self.validate_bug1040_worker(role, &source, &path)?;
            outputs.insert(role, self.run_bug1040_worker(role, &plan, start)?);
        }
        let good = outputs
            .get(&Bug1040Worker::OperatingPoint)
            .expect("both BUG1040 workers ran");
        let test = outputs
            .get(&Bug1040Worker::Noop)
            .expect("both BUG1040 workers ran");
        let mismatches = self
            .compare_xyce_verify_transient_tables(good, test)
            .map_err(|error| format!("{BUG1040_LABEL} xyce_verify relation failed: {error}"))?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG1040_LABEL} NOOP differs from operating-point startup: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{BUG1040_LABEL} execution exceeded timeout"));
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
    fn bug1040_workers_preserve_the_exact_startup_delta() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1040Worker::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1040 worker");
            runner
                .validate_bug1040_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
            assert!(
                runner
                    .validate_bug1040_worker(
                        role,
                        &source.replacen("R1 1 2 1K", "R1 1 2 2K", 1),
                        &path
                    )
                    .is_err()
            );
        }
    }

    #[test]
    fn bug1040_oracle_runs_both_startup_modes() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner
            .validate_bug1040_oracle(Instant::now())
            .expect("canonical BUG1040 relation passes");
    }

    #[test]
    fn bug1040_oracle_rejects_an_expired_deadline() {
        let runner = XyceTestRunner::new(
            corpus_root(),
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_bug1040_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }
}
