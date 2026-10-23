use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "ISSUE_202 redefined-parameter mode matrix";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Issue202Role {
    Owner,
    First,
    Last,
}

impl Issue202Role {
    const ALL: [Self; 3] = [Self::Owner, Self::First, Self::Last];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_ISSUE202_OWNER_CONTRACT,
            Self::First => XYCE_ISSUE202_FIRST_CONTRACT,
            Self::Last => XYCE_ISSUE202_LAST_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::Owner => XYCE_ISSUE202_OWNER_PATH,
            Self::First => XYCE_ISSUE202_FIRST_PATH,
            Self::Last => XYCE_ISSUE202_LAST_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Owner => XYCE_ISSUE202_OWNER_RECORD,
            Self::First => XYCE_ISSUE202_FIRST_RECORD,
            Self::Last => XYCE_ISSUE202_LAST_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("ISSUE202 path has name")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Issue202Mode {
    NoOption,
    Ignore,
    Warning,
    Warn,
    UseFirst,
    UseFirstWarn,
    UseLast,
    UseLastWarn,
}

impl Issue202Mode {
    const SUCCESSFUL: [Self; 8] = [
        Self::NoOption,
        Self::Ignore,
        Self::Warning,
        Self::Warn,
        Self::UseFirst,
        Self::UseFirstWarn,
        Self::UseLast,
        Self::UseLastWarn,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::NoOption => "nooption",
            Self::Ignore => "ignore",
            Self::Warning => "warning",
            Self::Warn => "warn",
            Self::UseFirst => "usefirst",
            Self::UseFirstWarn => "usefirstwarn",
            Self::UseLast => "uselast",
            Self::UseLastWarn => "uselastwarn",
        }
    }

    fn policies(
        self,
    ) -> (
        ParameterRedefinitionPolicy,
        rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy,
    ) {
        use rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy as Diagnostic;
        match self {
            Self::NoOption | Self::Ignore | Self::UseFirst => {
                (ParameterRedefinitionPolicy::UseFirst, Diagnostic::Silent)
            }
            Self::Warning | Self::Warn | Self::UseFirstWarn => {
                (ParameterRedefinitionPolicy::UseFirst, Diagnostic::Warning)
            }
            Self::UseLast => (ParameterRedefinitionPolicy::UseLast, Diagnostic::Silent),
            Self::UseLastWarn => (ParameterRedefinitionPolicy::UseLast, Diagnostic::Warning),
        }
    }

    fn control(self) -> Issue202Role {
        match self {
            Self::UseLast | Self::UseLastWarn => Issue202Role::Last,
            _ => Issue202Role::First,
        }
    }
}

impl XyceTestRunner {
    fn validate_issue202_plan(
        role: Issue202Role,
        plan: &XyceStaticDcPlan,
        expected_diagnostics: usize,
    ) -> Result<(), String> {
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.start.to_bits() != 1.0f64.to_bits()
            || plan.dc.stop.to_bits() != 1.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || probes != ["v(1)", "i(v1)", "r1:r"]
            || plan.diagnostics.len() != expected_diagnostics
        {
            return Err(format!(
                "{LABEL} {} DC plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn validate_issue202_netlist(
        role: Issue202Role,
        netlist: &Netlist,
        expected_resistance: Value,
        diagnostic_policy: rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy,
    ) -> Result<(), String> {
        if !netlist.title.is_empty()
            || netlist.elements.len() != 2
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || netlist.params.get("FOOBAR").map(Value::to_bits)
                != Some(expected_resistance.to_bits())
            || netlist.params.all_params().len() != 1
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let [resistor, source] = netlist.elements.as_slice() else {
            unreachable!("ISSUE202 element count checked")
        };
        if resistor.provenance != ElementProvenance::Authored
            || !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == expected_resistance.to_bits()
                && value_expr.is_none()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || source.provenance != ElementProvenance::Authored
            || !source.name.eq_ignore_ascii_case("V1")
            || source.nodes != ["1", "0"]
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} topology changed: resistor={resistor:?}, source={source:?}",
                role.file_name()
            ));
        }
        let expected_warning_count = if role == Issue202Role::Owner
            && diagnostic_policy
                == rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Warning
        {
            5
        } else {
            0
        };
        if netlist.diagnostics.len() != expected_warning_count {
            return Err(format!(
                "{LABEL} {} diagnostic count changed for {diagnostic_policy:?}: expected {expected_warning_count}, actual {:?}",
                role.file_name(),
                netlist.diagnostics,
            ));
        }
        if expected_warning_count != 0 {
            let selected = match netlist.params.parameter_redefinition_policy() {
                ParameterRedefinitionPolicy::UseFirst => "first",
                ParameterRedefinitionPolicy::UseLast => "last",
            };
            for (offset, diagnostic) in netlist.diagnostics.iter().enumerate() {
                let expected_line = offset + 3;
                if diagnostic.code != "parameter-redefinition"
                    || diagnostic.severity != rspice_core::netlist::DiagnosticSeverity::Warning
                    || diagnostic.line != expected_line
                    || diagnostic.origin.as_ref().is_none_or(|origin| {
                        origin.line != expected_line
                            || origin
                                .path
                                .as_deref()
                                .is_none_or(|path| !path.ends_with(Path::new(role.path())))
                    })
                    || diagnostic.message
                        != format!("Parameter FOOBAR defined more than once. Using {selected} one.")
                {
                    return Err(format!(
                        "{LABEL} warning projection changed: {diagnostic:?}"
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_issue202_table(
        role: Issue202Role,
        table: &XycePrnTable,
        expected_resistance: Value,
    ) -> Result<(), String> {
        if table.columns.len() != 4
            || !table.columns[0].eq_ignore_ascii_case("Index")
            || !table.columns[1].eq_ignore_ascii_case("V(1)")
            || !table.columns[2].eq_ignore_ascii_case("I(V1)")
            || !table.columns[3].eq_ignore_ascii_case("R1:R")
            || table.rows.len() != 1
            || table.rows[0].len() != 4
            || table.rows[0].iter().any(|value| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} {} table shape changed: {table:?}",
                role.file_name()
            ));
        }
        let row = &table.rows[0];
        let expected_current = -1.0 / expected_resistance;
        for (actual, expected, label) in [
            (row[0], 0.0, "Index"),
            (row[1], 1.0, "V(1)"),
            (row[2], expected_current, "I(V1)"),
            (row[3], expected_resistance, "R1:R"),
        ] {
            let actual = Self::xyce_default_prn_roundtrip(actual)?;
            let expected = Self::xyce_default_prn_roundtrip(expected)?;
            if actual != expected {
                return Err(format!(
                    "{LABEL} {} {label} analytic value changed: {actual} != {expected}",
                    role.file_name()
                ));
            }
        }
        Ok(())
    }

    fn issue202_plan(
        &self,
        role: Issue202Role,
        selection: ParameterRedefinitionPolicy,
        diagnostic: rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy,
    ) -> Result<XyceStaticDcPlan, String> {
        let plan = self.static_dc_plan_for_path_with_redefinition_policies(
            &self.root.join(role.path()),
            ExpressionDialect::Xyce,
            selection,
            diagnostic,
        )?;
        Self::validate_issue202_plan(
            role,
            &plan,
            usize::from(
                role == Issue202Role::Owner
                    && diagnostic
                        == rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Warning,
            ) * 5,
        )?;
        Ok(plan)
    }

    pub(super) fn validate_issue202_oracle(&self, start: Instant) -> Result<(), String> {
        use rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy as Diagnostic;
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before execution"
            ));
        }

        let run = |plan: &XyceStaticDcPlan, expected: Value, member_role: Issue202Role| {
            let (netlist, results) = self.run_static_dc_results(plan, start).map_err(|error| {
                format!(
                    "{LABEL} {} execution failed: {error}",
                    member_role.file_name()
                )
            })?;
            Self::validate_issue202_netlist(
                member_role,
                &netlist,
                expected,
                plan.parameter_redefinition_diagnostic_policy,
            )?;
            let table = self.dc_results_to_prn_table(plan, &netlist, &results)?;
            Self::validate_issue202_table(member_role, &table, expected)?;
            Ok::<_, String>((results, table))
        };

        let first_plan = self.issue202_plan(
            Issue202Role::First,
            ParameterRedefinitionPolicy::UseFirst,
            Diagnostic::Silent,
        )?;
        let last_plan = self.issue202_plan(
            Issue202Role::Last,
            ParameterRedefinitionPolicy::UseLast,
            Diagnostic::Silent,
        )?;
        let (first_results, first_table) = run(&first_plan, 1_000.0, Issue202Role::First)?;
        let (last_results, last_table) = run(&last_plan, 1.0, Issue202Role::Last)?;

        for mode in Issue202Mode::SUCCESSFUL {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} exceeded its shared deadline before {}",
                    mode.label()
                ));
            }
            let (selection, diagnostic) = mode.policies();
            let plan = self.issue202_plan(Issue202Role::Owner, selection, diagnostic)?;
            let control = mode.control();
            let expected = if control == Issue202Role::First {
                1_000.0
            } else {
                1.0
            };
            let (owner_results, owner_table) = run(&plan, expected, Issue202Role::Owner)?;
            let (good_results, good_table) = if control == Issue202Role::First {
                (&first_results, &first_table)
            } else {
                (&last_results, &last_table)
            };
            let mismatches = self.compare_release_7_10_xyce_verify_dc_tables(
                &format!("ISSUE_202 {}", mode.label()),
                good_table,
                &owner_table,
                good_results,
                &owner_results,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {} produced {} mismatch(es): {mismatches:?}",
                    mode.label(),
                    mismatches.len()
                ));
            }
        }

        let owner_source = fs::read_to_string(self.root.join(XYCE_ISSUE202_OWNER_PATH))
            .map_err(|error| format!("failed to read {LABEL} owner: {error}"))?;
        let error = Self::parse_netlist_with_expression_dialect_policies_and_execution_dir(
            &owner_source,
            &self.root.join(XYCE_ISSUE202_OWNER_PATH),
            ExpressionDialect::Xyce,
            ParameterRedefinitionPolicy::UseFirst,
            Diagnostic::Error,
            None,
        )
        .expect_err("ISSUE202 error mode must reject a duplicate");
        let rspice_core::netlist::ParseError::ParameterRedefinition(error) = error else {
            return Err(format!(
                "{LABEL} error mode returned the wrong failure: {error}"
            ));
        };
        if error.canonical_name != "FOOBAR"
            || error.kind != rspice_core::netlist::ParameterDefinitionKind::Parameter
            || error.first_origin.line != 2
            || error.duplicate_origin.line != 3
            || !error
                .to_string()
                .contains("Parameter FOOBAR defined more than once")
        {
            return Err(format!("{LABEL} error-mode diagnostic changed: {error:?}"));
        }

        if abort.is_aborted() {
            return Err(format!("{LABEL} exceeded its shared deadline"));
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
    fn issue202_all_roles_execute_the_complete_mode_matrix() {
        let runner = XyceTestRunner::new(
            corpus_root(),
            XyceRunnerConfig {
                max_time_per_test_ms: 30_000,
                ..XyceRunnerConfig::default()
            },
        );
        runner
            .validate_issue202_oracle(Instant::now())
            .expect("canonical ISSUE202 mode matrix passes");
    }

    #[test]
    fn issue202_expired_deadline_fails_closed() {
        let runner = XyceTestRunner::new(
            corpus_root(),
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_issue202_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }

    #[test]
    fn issue202_relation_rejects_numeric_counterfactuals() {
        let first = XycePrnTable {
            columns: vec!["Index".into(), "V(1)".into(), "I(V1)".into(), "R1:R".into()],
            rows: vec![vec![0.0, 1.0, -1.0e-3, 1_000.0]],
        };
        XyceTestRunner::validate_issue202_table(Issue202Role::First, &first, 1_000.0)
            .expect("canonical first-definition table");
        let mut wrong_current = first.clone();
        wrong_current.rows[0][2] = -2.0e-3;
        assert!(
            XyceTestRunner::validate_issue202_table(Issue202Role::First, &wrong_current, 1_000.0,)
                .is_err()
        );
        let mut wrong_parameter_probe = first;
        wrong_parameter_probe.rows[0][3] = 1.0;
        assert!(
            XyceTestRunner::validate_issue202_table(
                Issue202Role::First,
                &wrong_parameter_probe,
                1_000.0,
            )
            .is_err()
        );
    }
}
