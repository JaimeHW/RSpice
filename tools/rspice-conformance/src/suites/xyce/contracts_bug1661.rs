use super::*;

const BUG1661_LABEL: &str = "BUG_1661 top-level global-node expression wrapper";

impl XyceTestRunner {
    fn bug1661_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug1661_typed_contract(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
        )?;
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 2
                    || !print.probes[0].eq_ignore_ascii_case("V($g_1)")
                    || !print.probes[1].eq_ignore_ascii_case("V(1)")
            })
            || plan.tran.step.to_bits() != 1.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 4.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{BUG1661_LABEL} transient plan changed: {plan:?}"));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{BUG1661_LABEL} no longer parses: {error}"))?;
        if netlist.title != "Test to highlight global nodes bug"
            || netlist.elements.len() != 4
            || !netlist.models.is_empty()
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
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{BUG1661_LABEL} typed envelope changed"));
        }

        let [voltage, resistor_global, behavioral, resistor_local] = netlist.elements.as_slice()
        else {
            unreachable!("BUG1661 typed element count was checked");
        };
        let expected_points: [(Value, Value); 4] = [
            (0.0, 0.0),
            (1.0e-3, 1.0e-3),
            (2.0e-3, 4.0e-3),
            (3.0e-3, 9.0e-3),
        ];
        if !voltage.name.eq_ignore_ascii_case("V1")
            || !Self::bug1661_nodes_match(&voltage.nodes, &["$g_1", "0"])
            || voltage.provenance != ElementProvenance::Authored
            || !matches!(&voltage.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Pwl { points, delay, repeat_from }
            ) if points.len() == expected_points.len()
                && points.iter().zip(expected_points).all(|((time, value), (expected_time, expected_value))|
                    time.to_bits() == expected_time.to_bits()
                        && value.to_bits() == expected_value.to_bits())
                && delay.to_bits() == 0.0f64.to_bits()
                && repeat_from.is_none())
            || !resistor_global.name.eq_ignore_ascii_case("R1")
            || !Self::bug1661_nodes_match(&resistor_global.nodes, &["$g_1", "0"])
            || resistor_global.provenance != ElementProvenance::Authored
            || !matches!(&resistor_global.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !behavioral.name.eq_ignore_ascii_case("B1")
            || !Self::bug1661_nodes_match(&behavioral.nodes, &["1", "0"])
            || behavioral.provenance != ElementProvenance::Authored
            || !matches!(&behavioral.kind, ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
                multiplicity,
            } if expression.chars().filter(|character| !character.is_whitespace()).collect::<String>()
                    .eq_ignore_ascii_case("V($g_1)")
                && tc1.to_bits() == 0.0f64.to_bits()
                && tc2.to_bits() == 0.0f64.to_bits()
                && multiplicity.value.to_bits() == 1.0f64.to_bits()
                && multiplicity.value_expr.is_none()
                && !multiplicity.given)
            || !resistor_local.name.eq_ignore_ascii_case("R2")
            || !Self::bug1661_nodes_match(&resistor_local.nodes, &["1", "0"])
            || resistor_local.provenance != ElementProvenance::Authored
            || !matches!(&resistor_local.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!(
                "{BUG1661_LABEL} exact PWL/global-node/behavioral topology changed"
            ));
        }

        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
        } if step.to_bits() == 1.0e-9f64.to_bits()
            && stop.to_bits() == 4.0e-3f64.to_bits())
        {
            return Err(format!("{BUG1661_LABEL} typed TRAN command changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.origin.line != 8
            || request
                .origin
                .path
                .as_deref()
                .is_none_or(|origin_path| !Self::same_path(origin_path, path))
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .zip([("V", "$g_1"), ("V", "1")])
                .any(|(dependency, (operator, symbol))| {
                    dependency.kind != OutputSymbolKind::Node
                        || dependency.expression
                        || !dependency.operator.eq_ignore_ascii_case(operator)
                        || !dependency.symbol.eq_ignore_ascii_case(symbol)
                })
        {
            return Err(format!(
                "{BUG1661_LABEL} typed .PRINT request changed: {request:?}"
            ));
        }
        Ok(plan)
    }

    fn validate_bug1661_serialized_prn_equality(
        table: &XycePrnTable,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V($g_1)")
            || !table.columns[3].eq_ignore_ascii_case("V(1)")
            || table.rows.len() < 2
        {
            return Err(format!(
                "{BUG1661_LABEL} PRN schema changed: columns={:?}, rows={}",
                table.columns,
                table.rows.len()
            ));
        }
        for (row_index, row) in table.rows.iter().enumerate() {
            if row_index % 1_024 == 0 && abort.is_aborted() {
                return Err(format!(
                    "{BUG1661_LABEL} deadline expired while comparing PRN rows"
                ));
            }
            if row.len() != 4 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "{BUG1661_LABEL} PRN row {row_index} is malformed or non-finite: {row:?}"
                ));
            }
            if row[0] != row_index as Value {
                return Err(format!(
                    "{BUG1661_LABEL} PRN index changed at row {row_index}: {}",
                    row[0]
                ));
            }
            if row_index > 0 && row[1] <= table.rows[row_index - 1][1] {
                return Err(format!(
                    "{BUG1661_LABEL} PRN time grid does not increase at row {row_index}: {} <= {}",
                    row[1],
                    table.rows[row_index - 1][1]
                ));
            }
            let global = Self::xyce_default_prn_roundtrip(row[2])?;
            let behavioral = Self::xyce_default_prn_roundtrip(row[3])?;
            if global != behavioral {
                return Err(format!(
                    "{BUG1661_LABEL} serialized PRN columns differ at row {row_index}, time {}: {global} != {behavioral}",
                    row[1]
                ));
            }
        }
        let first = table.rows.first().expect("BUG1661 row count was checked");
        let last = table.rows.last().expect("BUG1661 row count was checked");
        let final_value = Self::xyce_default_prn_roundtrip(last[2])?;
        let expected_final = Self::xyce_default_prn_roundtrip(9.0e-3)?;
        let milestones = [(1.0e-3, 1.0e-3), (2.0e-3, 4.0e-3), (3.0e-3, 9.0e-3)];
        let milestones_match = milestones
            .into_iter()
            .all(|(expected_time, expected_value)| {
                let Ok(expected_time) = Self::xyce_default_prn_roundtrip(expected_time) else {
                    return false;
                };
                let Ok(expected_value) = Self::xyce_default_prn_roundtrip(expected_value) else {
                    return false;
                };
                table.rows.iter().any(|row| {
                    Self::xyce_default_prn_roundtrip(row[1]).is_ok_and(|time| time == expected_time)
                        && Self::xyce_default_prn_roundtrip(row[2])
                            .is_ok_and(|value| value == expected_value)
                })
            });
        if first[1].abs() > 1.0e-15
            || Self::xyce_default_prn_roundtrip(first[2])? != Self::xyce_default_prn_roundtrip(0.0)?
            || (last[1] - 4.0e-3).abs() > 1.0e-12
            || final_value != expected_final
            || !milestones_match
        {
            return Err(format!(
                "{BUG1661_LABEL} produced an invalid or trivial time/value envelope: first={first:?}, last={last:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1661_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1661_LABEL} deadline expired before validation"
            ));
        }
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        let plan = self.validate_bug1661_typed_contract(&source, &deck.path)?;
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| format!("{BUG1661_LABEL} execution failed: {error}"))?;
        let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{BUG1661_LABEL} output observation failed: {error}"))?;
        Self::validate_bug1661_serialized_prn_equality(&table, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{BUG1661_LABEL} execution exceeded timeout"));
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

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(XYCE_BUG1661_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1661_PATH.to_string(),
        }
    }

    #[test]
    fn bug1661_typed_contract_preserves_implicit_global_node_and_behavioral_reference() {
        let root = corpus_root();
        let path = root.join(XYCE_BUG1661_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG1661 source");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1661_typed_contract(&source, &path)
            .expect("canonical BUG1661 typed contract passes");
        let temporary = tempfile::Builder::new()
            .prefix("rspice-xyce-bug1661-typed-mutation-")
            .tempdir()
            .expect("create BUG1661 mutation root");
        let mutated_path = temporary.path().join(XYCE_BUG1661_PATH);
        fs::create_dir_all(mutated_path.parent().expect("BUG1661 deck has a family"))
            .expect("create BUG1661 mutation family");
        fs::write(&mutated_path, &source).expect("copy canonical BUG1661 source");
        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1661_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG1661 wrapper manifest");
        let fixture_runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        fixture_runner
            .validate_bug1661_typed_contract(&source, &mutated_path)
            .expect("canonical BUG1661 typed contract passes outside the corpus");
        for mutation in [
            source.replacen("V={V($g_1)}", "V={V($g_2)}", 1),
            source.replacen("3ms 9mV", "3ms 8mV", 1),
            source.replacen("R1 $g_1 0 1", "R1 $g_1 0 2", 1),
            source.replacen("v($g_1) v(1)", "v($g_1) v(2)", 1),
            source.replacen(".tran 1ns 4ms", ".tran 1ns 5ms", 1),
        ] {
            fs::write(&mutated_path, &mutation).expect("write BUG1661 semantic mutation");
            assert!(
                fixture_runner
                    .validate_bug1661_typed_contract(&mutation, &mutated_path)
                    .is_err(),
                "BUG1661 semantic mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug1661_prn_predicate_uses_default_serialized_numeric_equality() {
        let start = Instant::now();
        let abort = DeadlineAbort::new(start, 10_000);
        let columns = vec![
            "Index".to_string(),
            "TIME".to_string(),
            "V($g_1)".to_string(),
            "V(1)".to_string(),
        ];
        let equal_after_serialization = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 0.0, 0.0, -0.0],
                vec![1.0, 1.0e-3, 1.00000000001e-3, 1.00000000002e-3],
                vec![2.0, 2.0e-3, 4.00000000001e-3, 4.00000000002e-3],
                vec![3.0, 3.0e-3, 9.00000000001e-3, 9.00000000002e-3],
                vec![4.0, 4.0e-3, 9.00000000001e-3, 9.00000000002e-3],
            ],
        };
        XyceTestRunner::validate_bug1661_serialized_prn_equality(
            &equal_after_serialization,
            &abort,
        )
        .expect("values equal after default PRN serialization pass");

        let unequal = XycePrnTable {
            columns,
            rows: vec![vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 4.0e-3, 9.0e-3, 8.0e-3]],
        };
        assert!(
            XyceTestRunner::validate_bug1661_serialized_prn_equality(&unequal, &abort).is_err()
        );

        let row_zero_mismatch = XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "TIME".to_string(),
                "V($g_1)".to_string(),
                "V(1)".to_string(),
            ],
            rows: vec![
                vec![0.0, 0.0, 0.0, 1.0],
                vec![1.0, 1.0e-3, 1.0e-3, 1.0e-3],
                vec![2.0, 2.0e-3, 4.0e-3, 4.0e-3],
                vec![3.0, 3.0e-3, 9.0e-3, 9.0e-3],
                vec![4.0, 4.0e-3, 9.0e-3, 9.0e-3],
            ],
        };
        assert!(
            XyceTestRunner::validate_bug1661_serialized_prn_equality(&row_zero_mismatch, &abort,)
                .is_err(),
            "RSpice must close the historical wrapper's Index=0 false-positive"
        );
    }

    #[test]
    fn bug1661_oracle_executes_the_exact_global_node_relation() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1661_oracle(&canonical_deck(&root), Instant::now())
            .expect("canonical BUG1661 relational wrapper passes");
    }

    #[test]
    fn bug1661_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG1661 deadline");
        assert!(
            runner
                .validate_bug1661_oracle(&canonical_deck(&root), start)
                .is_err()
        );
    }
}
