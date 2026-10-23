use super::*;

const BUG271_LABEL: &str = "BUG_271_SON tab-comment RLC success wrapper";

impl XyceTestRunner {
    fn bug271_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug271_typed_contract(
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
                    || !print.probes[0].eq_ignore_ascii_case("V(1)")
                    || !print.probes[1]
                        .chars()
                        .filter(|ch| !ch.is_whitespace())
                        .collect::<String>()
                        .eq_ignore_ascii_case("{I(V1)-1.0}")
            })
            || plan.tran.step.to_bits() != 0.01f64.to_bits()
            || plan.tran.stop.to_bits() != 10.0f64.to_bits()
            || plan.tran.start.map(Value::to_bits) != Some(0.0f64.to_bits())
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{BUG271_LABEL} transient plan changed: {plan:?}"));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{BUG271_LABEL} no longer parses: {error}"))?;
        if netlist.elements.len() != 4
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
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{BUG271_LABEL} typed envelope changed"));
        }
        let [resistor, inductor, capacitor, voltage] = netlist.elements.as_slice() else {
            unreachable!("BUG271 typed element count was checked");
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::bug271_nodes_match(&resistor.nodes, &["1", "2"])
            || resistor.provenance != ElementProvenance::Authored
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 3.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !inductor.name.eq_ignore_ascii_case("L1")
            || !Self::bug271_nodes_match(&inductor.nodes, &["2", "3"])
            || inductor.provenance != ElementProvenance::Authored
            || !matches!(&inductor.kind, ElementKind::Inductor {
                value,
                value_expr: None,
                initial_current: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !capacitor.name.eq_ignore_ascii_case("C1")
            || !Self::bug271_nodes_match(&capacitor.nodes, &["3", "0"])
            || capacitor.provenance != ElementProvenance::Authored
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 0.5f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !voltage.name.eq_ignore_ascii_case("V1")
            || !Self::bug271_nodes_match(&voltage.nodes, &["1", "0"])
            || voltage.provenance != ElementProvenance::Authored
            || !matches!(&voltage.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::DcTransient { dc_value, transient }
            ) if dc_value.to_bits() == 10.0f64.to_bits()
                && matches!(transient.as_ref(), rspice_core::netlist::SourceSpec::Pulse {
                    v1, v2, delay, rise, fall, width, period, pulse_count,
                    width_defaults_to_zero,
                } if v1.to_bits() == 0.0f64.to_bits()
                    && v2.to_bits() == 10.0f64.to_bits()
                    && delay.to_bits() == 0.0f64.to_bits()
                    && rise.to_bits() == 0.0f64.to_bits()
                    && fall.to_bits() == 0.0f64.to_bits()
                    && width.to_bits() == 10.0f64.to_bits()
                    && period.to_bits() == 10.0f64.to_bits()
                    && pulse_count.to_bits() == 0.0f64.to_bits()
                    && !width_defaults_to_zero))
        {
            return Err(format!("{BUG271_LABEL} exact RLC/PULSE topology changed"));
        }
        Ok(plan)
    }

    pub(super) fn validate_bug271_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG271_LABEL} deadline expired before validation"));
        }
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        let plan = self.validate_bug271_typed_contract(&source, &deck.path)?;
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| format!("{BUG271_LABEL} execution failed: {error}"))?;
        let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{BUG271_LABEL} output observation failed: {error}"))?;
        let voltage_peak = table
            .rows
            .iter()
            .filter_map(|row| row.get(2))
            .copied()
            .fold(0.0f64, Value::max);
        let shifted_current_peak = table
            .rows
            .iter()
            .filter_map(|row| row.get(3))
            .map(|value| value.abs())
            .fold(0.0f64, Value::max);
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || table.rows.len() < 100
            || table
                .rows
                .iter()
                .any(|row| row.len() != 4 || row.iter().any(|value| !value.is_finite()))
            || table.rows.first().is_none_or(|row| row[1].abs() > 1.0e-15)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 10.0).abs() > 1.0e-10)
            || (voltage_peak - 10.0).abs() > 1.0e-10
            || shifted_current_peak < 0.5
        {
            return Err(format!(
                "{BUG271_LABEL} produced an invalid or trivial observation: rows={}, Vpeak={voltage_peak}, shifted-I peak={shifted_current_peak}",
                table.rows.len()
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{BUG271_LABEL} execution exceeded timeout"));
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
    fn bug271_typed_contract_preserves_tabs_comments_and_rlc_pulse() {
        let root = corpus_root();
        let path = root.join(XYCE_BUG271_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG271 source");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug271_typed_contract(&source, &path)
            .expect("canonical BUG271 typed contract passes");
        for mutation in [
            source.replacen("r1 1 2 3", "r1 1 2 4", 1),
            source.replacen("\tNote:", "Rogue:", 1),
            source.replacen(".tran  0.01 10 0", ".tran 0.02 10 0", 1),
            source.replacen("{i(v1)-1.0}", "{i(v1)-2.0}", 1),
        ] {
            assert!(
                runner
                    .validate_bug271_typed_contract(&mutation, &path)
                    .is_err(),
                "BUG271 semantic/comment mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug271_oracle_executes_the_exact_success_wrapper() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG271_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG271_PATH.to_string(),
        };
        runner
            .validate_bug271_oracle(&deck, Instant::now())
            .expect("canonical BUG271 success wrapper passes");
    }

    #[test]
    fn bug271_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            path: root.join(XYCE_BUG271_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG271_PATH.to_string(),
        };
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG271 deadline");
        assert!(runner.validate_bug271_oracle(&deck, start).is_err());
    }
}
