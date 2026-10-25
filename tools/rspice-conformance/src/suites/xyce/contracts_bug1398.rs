use super::*;

const BUG1398_LABEL: &str = "BUG_1398 inductor-model equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1398Role {
    ModelOwner,
    LiteralControl,
}

impl Bug1398Role {
    const ALL: [Self; 2] = [Self::ModelOwner, Self::LiteralControl];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::ModelOwner => XYCE_BUG1398_OWNER_PATH,
            Self::LiteralControl => XYCE_BUG1398_CONTROL_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ModelOwner => XYCE_BUG1398_OWNER_RECORD,
            Self::LiteralControl => XYCE_BUG1398_CONTROL_RECORD,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::ModelOwner => XYCE_BUG1398_OWNER_CONTRACT,
            Self::LiteralControl => XYCE_BUG1398_CONTROL_CONTRACT,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::ModelOwner => "model-card owner",
            Self::LiteralControl => "literal control",
        }
    }

    fn plan_purpose(self) -> XyceStaticTranPlanPurpose {
        match self {
            Self::ModelOwner => XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            Self::LiteralControl => XyceStaticTranPlanPurpose::RelationalFamily,
        }
    }
}

impl XyceTestRunner {
    fn validate_bug1398_model(model: &rspice_core::netlist::ModelDef) -> bool {
        if !model.model_type.eq_ignore_ascii_case("L")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let expected: &[(&str, Value)] = if model.name.eq_ignore_ascii_case("IND_D") {
            &[("L", 1.0)]
        } else if model.name.eq_ignore_ascii_case("IND_E") {
            &[("L", 2.0)]
        } else if model.name.eq_ignore_ascii_case("IND_F") {
            &[("L", 3.0)]
        } else if model.name.eq_ignore_ascii_case("IND_G") {
            &[("TC1", 0.01), ("TC2", 0.001)]
        } else {
            return false;
        };
        model.params.len() == expected.len()
            && expected.iter().all(|(expected_name, expected_value)| {
                model.params.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case(expected_name)
                        && value.to_bits() == expected_value.to_bits()
                })
            })
    }

    fn validate_bug1398_worker(
        &self,
        role: Bug1398Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(path, role.plan_purpose())?;
        let expected_contract = if role == Bug1398Role::ModelOwner {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        let expected_probes = [
            "V(2a)", "V(2b)", "V(2c)", "V(2d)", "V(2e)", "V(2f)", "V(2g)",
        ];
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
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
            || plan.tran.step.to_bits() != 2.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 8.0e-4f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{BUG1398_LABEL} {} plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG1398_LABEL} {} no longer parses: {error}", role.label())
        })?;
        if netlist.elements.len() != 28
            || netlist.models.len() != 4
            || !netlist.models.iter().all(Self::validate_bug1398_model)
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
            || netlist.options.temp.map(Value::to_bits) != Some(37.0f64.to_bits())
            || netlist.params.numeric_parameters().len() != 6
            || ![
                ("ML1", 1.0_f64),
                ("ML2", 2.0_f64),
                ("ML3", 3.0_f64),
                ("TEMP", 37.0_f64),
                ("TEMPER", 37.0_f64),
                ("VT", 0.026_726_659_112_543_266_f64),
            ]
            .iter()
            .all(|(name, value)| {
                netlist
                    .params
                    .get(name)
                    .is_some_and(|actual| actual.to_bits() == value.to_bits())
            })
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{BUG1398_LABEL} {} typed envelope changed: elements={}, models={:?}, analyses={}, outputs={}, diagnostics={:?}, temp={:?}, numeric={:?}, expressions={:?}",
                role.label(),
                netlist.elements.len(),
                netlist.models,
                netlist.analyses.len(),
                netlist.output_requests.len(),
                netlist.diagnostics,
                netlist.options.temp,
                netlist.params.numeric_parameters(),
                netlist.params.all_parameter_expressions(),
            ));
        }
        let expected_inductors: &[(&str, Value, Option<&str>)] = match role {
            Bug1398Role::ModelOwner => &[
                ("L1a", 0.005, None),
                ("L1b", 0.01, None),
                ("L1c", 0.015, None),
                ("L1d", 0.005, Some("IND_D")),
                ("L1e", 0.005, Some("IND_E")),
                ("L1f", 0.005, Some("IND_F")),
                ("L1g", 0.005, Some("IND_G")),
            ],
            Bug1398Role::LiteralControl => &[
                ("L1a", 0.005, None),
                ("L1b", 0.01, None),
                ("L1c", 0.015, None),
                ("L1d", 0.005, None),
                ("L1e", 0.01, None),
                ("L1f", 0.015, None),
                ("L1g", 0.006_000_000_000_000_001, None),
            ],
        };
        let pulse_delays: [Value; 7] = [1.0e-5, 1.0e-5, 1.0e-5, 4.0e-4, 4.5e-4, 5.0e-4, 5.0e-4];
        for (branch, ((name, expected_value, expected_model), expected_delay)) in
            expected_inductors.iter().zip(pulse_delays).enumerate()
        {
            let suffix = (b'a' + branch as u8) as char;
            let [source, resistor, element, capacitor] =
                &netlist.elements[branch * 4..branch * 4 + 4]
            else {
                unreachable!("BUG1398 typed element count was checked");
            };
            let source_name = format!("VPULSE{suffix}");
            let node_one = format!("1{suffix}");
            let node_two = format!("2{suffix}");
            if !source.name.eq_ignore_ascii_case(&source_name)
                || !Self::nodes_match(&source.nodes, &[node_two.as_str(), node_one.as_str()])
                || source.provenance != ElementProvenance::Authored
                || !matches!(&source.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pulse {
                    v1, v2, delay, rise, fall, width, period, pulse_count, width_defaults_to_zero,
                }) if v1.to_bits() == 0.0f64.to_bits()
                    && v2.to_bits() == 1.0f64.to_bits()
                    && delay.to_bits() == expected_delay.to_bits()
                    && rise.to_bits() == 1.0e-4f64.to_bits()
                    && fall.to_bits() == 1.0e-4f64.to_bits()
                    && width.to_bits() == 1.0e-6f64.to_bits()
                    && period.to_bits() == 1.0e-3f64.to_bits()
                    && pulse_count.to_bits() == 0.0f64.to_bits()
                    && !width_defaults_to_zero)
                || !resistor.name.eq_ignore_ascii_case(&format!("R1{suffix}"))
                || !Self::nodes_match(&resistor.nodes, &[node_one.as_str(), "0"])
                || resistor.provenance != ElementProvenance::Authored
                || !matches!(&resistor.kind, ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == 400.0f64.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
                || !capacitor.name.eq_ignore_ascii_case(&format!("C1{suffix}"))
                || !Self::nodes_match(&capacitor.nodes, &[node_one.as_str(), "0"])
                || capacitor.provenance != ElementProvenance::Authored
                || !matches!(&capacitor.kind, ElementKind::Capacitor {
                    value,
                    value_expr: None,
                    initial_voltage: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == 1.0e-8f64.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
            {
                return Err(format!(
                    "{BUG1398_LABEL} {} branch {suffix} topology changed",
                    role.label()
                ));
            }
            if !matches!(&element.kind, ElementKind::Inductor {
                value,
                value_expr: None,
                initial_current: None,
                model,
                instance_params,
                deferred_params,
            } if value.to_bits() == expected_value.to_bits()
                && match (model.as_deref(), *expected_model) {
                    (Some(actual), Some(expected)) => actual.eq_ignore_ascii_case(expected),
                    (None, None) => true,
                    _ => false,
                }
                && instance_params.is_empty()
                && deferred_params.is_empty())
                || !element.name.eq_ignore_ascii_case(name)
                || !Self::nodes_match(&element.nodes, &[node_two.as_str(), "0"])
                || element.provenance != ElementProvenance::Authored
            {
                return Err(format!(
                    "{BUG1398_LABEL} {} {name} changed: {element:?}",
                    role.label()
                ));
            }
        }
        Ok(plan)
    }

    fn run_bug1398_worker(
        &self,
        role: Bug1398Role,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|error| format!("{BUG1398_LABEL} {} failed: {error}", role.label()))?;
        let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|error| format!("{BUG1398_LABEL} {} PRN failed: {error}", role.label()))?;
        if table.columns.len() != 9
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || table.rows.len() < 3
            || table
                .rows
                .iter()
                .any(|row| row.len() != 9 || row.iter().any(|value| !value.is_finite()))
            || table.rows.first().is_none_or(|row| row[1].abs() > 1.0e-15)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 8.0e-4).abs() > 1.0e-12)
            || table
                .rows
                .iter()
                .flat_map(|row| row[2..].iter())
                .map(|value| value.abs())
                .fold(0.0, Value::max)
                < 1.0e-3
        {
            return Err(format!(
                "{BUG1398_LABEL} {} produced an invalid or trivial table",
                role.label()
            ));
        }
        Ok(table)
    }

    pub(super) fn validate_bug1398_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1398_LABEL} deadline expired before validation"
            ));
        }
        let mut outputs = BTreeMap::new();
        for worker_role in Bug1398Role::ALL {
            let path = self.root.join(worker_role.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let plan = self.validate_bug1398_worker(worker_role, &source, &path)?;
            outputs.insert(
                worker_role,
                self.run_bug1398_worker(worker_role, &plan, start)?,
            );
        }
        let owner = outputs
            .get(&Bug1398Role::ModelOwner)
            .expect("both BUG1398 workers ran");
        let control = outputs
            .get(&Bug1398Role::LiteralControl)
            .expect("both BUG1398 workers ran");
        let mismatches = self
            .compare_xyce_verify_transient_tables(control, owner)
            .map_err(|error| format!("{BUG1398_LABEL} xyce_verify relation failed: {error}"))?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG1398_LABEL} owner differs from control: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{BUG1398_LABEL} execution exceeded timeout"));
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
    fn bug1398_workers_preserve_model_and_literal_inductances() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1398Role::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1398 worker");
            runner
                .validate_bug1398_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
            assert!(
                runner
                    .validate_bug1398_worker(role, &source.replacen("mL1=1.0", "mL1=1.1", 1), &path)
                    .is_err()
            );
        }
    }

    #[test]
    fn bug1398_oracle_runs_both_roles() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner
            .validate_bug1398_oracle(Instant::now())
            .expect("canonical BUG1398 relation passes");
    }

    #[test]
    fn bug1398_oracle_rejects_an_expired_deadline() {
        let runner = XyceTestRunner::new(
            corpus_root(),
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG1398 deadline");
        assert!(runner.validate_bug1398_oracle(start).is_err());
    }
}
