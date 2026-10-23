use super::*;

const BUG1162_LABEL: &str = "BUG_1162_SON inconsistent DC sweep";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1162Role {
    Owner,
    Baseline,
    Linear,
    Decade,
    Octave,
}

impl Bug1162Role {
    const ALL: [Self; 5] = [
        Self::Owner,
        Self::Baseline,
        Self::Linear,
        Self::Decade,
        Self::Octave,
    ];
    const WORKERS: [Self; 4] = [Self::Baseline, Self::Linear, Self::Decade, Self::Octave];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::Owner => XYCE_BUG1162_OWNER_PATH,
            Self::Baseline => XYCE_BUG1162_BASELINE_PATH,
            Self::Linear => XYCE_BUG1162_LINEAR_PATH,
            Self::Decade => XYCE_BUG1162_DECADE_PATH,
            Self::Octave => XYCE_BUG1162_OCTAVE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Owner => XYCE_BUG1162_OWNER_RECORD,
            Self::Baseline => XYCE_BUG1162_BASELINE_RECORD,
            Self::Linear => XYCE_BUG1162_LINEAR_RECORD,
            Self::Decade => XYCE_BUG1162_DECADE_RECORD,
            Self::Octave => XYCE_BUG1162_OCTAVE_RECORD,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_BUG1162_OWNER_CONTRACT,
            Self::Baseline => XYCE_BUG1162_BASELINE_CONTRACT,
            Self::Linear => XYCE_BUG1162_LINEAR_CONTRACT,
            Self::Decade => XYCE_BUG1162_DECADE_CONTRACT,
            Self::Octave => XYCE_BUG1162_OCTAVE_CONTRACT,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Owner => "wrapper owner",
            Self::Baseline => "one-point baseline",
            Self::Linear => "inconsistent linear control",
            Self::Decade => "inconsistent decade control",
            Self::Octave => "inconsistent octave control",
        }
    }

    fn expected_mode(self) -> DcSweepMode {
        match self {
            Self::Owner => unreachable!("the empty owner has no analysis"),
            Self::Baseline | Self::Linear => DcSweepMode::Linear,
            Self::Decade => DcSweepMode::Decade {
                points_per_decade: 4,
            },
            Self::Octave => DcSweepMode::Octave {
                points_per_octave: 4,
            },
        }
    }

    fn expected_warning_label(self) -> Option<&'static str> {
        match self {
            Self::Owner | Self::Baseline => None,
            Self::Linear => Some("Linear"),
            Self::Decade => Some("Decade"),
            Self::Octave => Some("Octave"),
        }
    }

    fn expected_warning_line(self) -> Option<usize> {
        match self {
            Self::Linear => Some(11),
            Self::Decade | Self::Octave => Some(12),
            Self::Owner | Self::Baseline => None,
        }
    }
}

impl XyceTestRunner {
    fn validate_bug1162_worker(
        &self,
        role: Bug1162Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        if role == Bug1162Role::Owner {
            return Err(format!("{BUG1162_LABEL} empty owner is not a worker"));
        }
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let expected_stop: Value = if role == Bug1162Role::Baseline {
            100.0
        } else {
            1.0
        };
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.mode != role.expected_mode()
            || plan.dc.start.to_bits() != 100.0f64.to_bits()
            || plan.dc.stop.to_bits() != expected_stop.to_bits()
            || plan.dc.step.to_bits()
                != (if matches!(role, Bug1162Role::Decade | Bug1162Role::Octave) {
                    4.0f64
                } else {
                    1.0f64
                })
                .to_bits()
            || plan.dc.sweep2.is_some()
            || plan.dc.primary_spec().points() != [100.0]
            || plan.print.probes != ["V(1)", "I(v1)"]
        {
            return Err(format!(
                "{BUG1162_LABEL} {} plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG1162_LABEL} {} no longer parses: {error}", role.label())
        })?;
        match role.expected_warning_label() {
            None if !plan.diagnostics.is_empty() || !netlist.diagnostics.is_empty() => {
                return Err(format!("{BUG1162_LABEL} baseline acquired diagnostics"));
            }
            Some(label) => {
                let expected_prefix = format!("{label} DC or STEP parameters for sweep over V1");
                for diagnostics in [&plan.diagnostics, &netlist.diagnostics] {
                    let [diagnostic] = diagnostics.as_slice() else {
                        return Err(format!(
                            "{BUG1162_LABEL} {} must emit exactly one warning",
                            role.label()
                        ));
                    };
                    if diagnostic.code != "xyce-inconsistent-dc-sweep-direction"
                        || !diagnostic.message.starts_with(&expected_prefix)
                        || Some(diagnostic.line) != role.expected_warning_line()
                        || diagnostic.origin.as_ref().map(|origin| origin.line)
                            != role.expected_warning_line()
                    {
                        return Err(format!(
                            "{BUG1162_LABEL} {} warning changed: {diagnostic:?}",
                            role.label()
                        ));
                    }
                }
            }
            None => {}
        }
        if netlist.elements.len() != 2
            || !netlist.models.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{BUG1162_LABEL} {} typed envelope changed",
                role.label()
            ));
        }
        let [resistor, voltage] = netlist.elements.as_slice() else {
            unreachable!("BUG1162 element count was checked")
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
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
            || !voltage.name.eq_ignore_ascii_case("V1")
            || voltage.nodes != ["1", "0"]
            || voltage.provenance != ElementProvenance::Authored
            || !matches!(&voltage.kind,
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{BUG1162_LABEL} {} topology changed: {:?}",
                role.label(),
                netlist.elements
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != 2
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
            || request.dependencies[0].expression
            || request.dependencies[1].kind != OutputSymbolKind::Device
            || !request.dependencies[1].operator.eq_ignore_ascii_case("I")
            || !request.dependencies[1].symbol.eq_ignore_ascii_case("V1")
            || request.dependencies[1].expression
            || !request.expressions.is_empty()
        {
            return Err(format!(
                "{BUG1162_LABEL} {} output request changed: {request:?}",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn run_bug1162_worker(
        &self,
        role: Bug1162Role,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<(XycePrnTable, Vec<DcSweepPointResult>), String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG1162_LABEL} {} failed: {error}", role.label()))?;
        let [point] = results.as_slice() else {
            return Err(format!(
                "{BUG1162_LABEL} {} produced {} points instead of one",
                role.label(),
                results.len()
            ));
        };
        if point.sweep_value.to_bits() != 100.0f64.to_bits()
            || !point
                .result
                .node_voltages
                .iter()
                .chain(point.result.branch_currents.iter())
                .all(|value| value.is_finite())
            || !point
                .result
                .dc_observables
                .iter()
                .all(|(_, value)| value.is_finite())
            || !point.device_op_report.labels_resolve()
        {
            return Err(format!(
                "{BUG1162_LABEL} {} did not produce a finite 100 V observation",
                role.label()
            ));
        }
        let table = self
            .dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| format!("{BUG1162_LABEL} {} PRN failed: {error}", role.label()))?;
        let [row] = table.rows.as_slice() else {
            return Err(format!("{BUG1162_LABEL} {} PRN lost its row", role.label()));
        };
        if table.columns != ["Index", "V(1)", "I(v1)"]
            || row.len() != 3
            || row.iter().any(|value| !value.is_finite())
            || Self::xyce_default_prn_roundtrip(row[0])?.to_bits() != 0.0f64.to_bits()
            || Self::xyce_default_prn_roundtrip(row[1])?.to_bits() != 100.0f64.to_bits()
            || (Self::xyce_default_prn_roundtrip(row[2])? + 0.1).abs() > 1.0e-12
        {
            return Err(format!(
                "{BUG1162_LABEL} {} one-point physics changed: {:?}/{row:?}",
                role.label(),
                table.columns
            ));
        }
        Ok((table, results))
    }

    pub(super) fn validate_bug1162_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1162_LABEL} deadline expired before validation"
            ));
        }
        let mut output = BTreeMap::new();
        for worker in Bug1162Role::WORKERS {
            let path = self.root.join(worker.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let plan = self.validate_bug1162_worker(worker, &source, &path)?;
            output.insert(worker, self.run_bug1162_worker(worker, &plan, start)?);
        }
        let (good, good_results) = output
            .get(&Bug1162Role::Baseline)
            .expect("BUG1162 baseline ran");
        for worker in [
            Bug1162Role::Linear,
            Bug1162Role::Decade,
            Bug1162Role::Octave,
        ] {
            let (test, test_results) = output.get(&worker).expect("BUG1162 control ran");
            let mismatches = self
                .compare_release_7_10_xyce_verify_dc_tables(
                    BUG1162_LABEL,
                    good,
                    test,
                    good_results,
                    test_results,
                )
                .map_err(|error| {
                    format!(
                        "{BUG1162_LABEL} {} comparison failed: {error}",
                        worker.label()
                    )
                })?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{BUG1162_LABEL} {} differs from baseline: {mismatches:?}",
                    worker.label()
                ));
            }
        }
        if abort.is_aborted() {
            return Err(format!("{BUG1162_LABEL} execution exceeded timeout"));
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
    fn bug1162_workers_preserve_one_point_and_typed_warnings() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1162Role::WORKERS {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1162 worker");
            runner
                .validate_bug1162_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
            assert!(
                runner
                    .validate_bug1162_worker(
                        role,
                        &source.replacen("R1 1 0 1K", "R1 1 0 2K", 1),
                        &path
                    )
                    .is_err()
            );
        }
    }

    #[test]
    fn bug1162_oracle_rejects_an_expired_deadline() {
        let runner = XyceTestRunner::new(
            corpus_root(),
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_bug1162_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }
}
