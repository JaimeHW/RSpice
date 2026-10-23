use super::*;

impl XyceTestRunner {
    pub(super) fn netlist_element_is_diode_analytic_oracle_candidate(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        if element.nodes.len() != 2 || !instance_params.is_empty() || !deferred_params.is_empty() {
            return false;
        }
        let Some(model) = Self::find_unique_model_in(&netlist.models, model) else {
            return false;
        };
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "D" | "DIODE"
        ) || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let mut params = model
            .params
            .iter()
            .map(|(name, value)| (name.to_ascii_uppercase(), value.to_bits()))
            .collect::<Vec<_>>();
        params.sort();
        let profiles = [
            vec![
                ("LEVEL".to_string(), 1.0f64.to_bits()),
                ("RS".to_string(), 0.0f64.to_bits()),
            ],
            vec![
                ("BV".to_string(), 1.0f64.to_bits()),
                ("LEVEL".to_string(), 1.0f64.to_bits()),
            ],
            vec![
                ("IS".to_string(), 1.0f64.to_bits()),
                ("LEVEL".to_string(), 1.0f64.to_bits()),
            ],
        ];
        profiles.contains(&params)
    }

    fn validate_diode_analytic_typed_contract(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        kind: XyceDiodeAnalyticKind,
    ) -> Result<(), String> {
        let expected_step: Value = match kind {
            XyceDiodeAnalyticKind::Forward | XyceDiodeAnalyticKind::Breakdown => 1.0e-6,
            XyceDiodeAnalyticKind::Reverse => 1.0e-3,
        };
        let expected_stop: Value = match kind {
            XyceDiodeAnalyticKind::Forward | XyceDiodeAnalyticKind::Breakdown => 10.0,
            XyceDiodeAnalyticKind::Reverse => 9.25,
        };
        let expected_probe = match kind {
            XyceDiodeAnalyticKind::Forward => "v(2)",
            XyceDiodeAnalyticKind::Reverse | XyceDiodeAnalyticKind::Breakdown => "v(1)",
        };
        let probes = plan
            .require_print(kind.label())?
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.tran.step.to_bits() != expected_step.to_bits()
            || plan.tran.stop.to_bits() != expected_stop.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || probes != [expected_probe]
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.measurements.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{} exact TRAN/PRINT/semantic envelope changed",
                kind.label()
            ));
        }
        if netlist.options.timeint_reltol.map(f64::to_bits) != Some(1.0e-6f64.to_bits())
            || netlist.options.timeint_abstol.map(f64::to_bits)
                != match kind {
                    XyceDiodeAnalyticKind::Reverse => None,
                    _ => Some(1.0e-6f64.to_bits()),
                }
            || netlist.options.gmin.map(f64::to_bits)
                != match kind {
                    XyceDiodeAnalyticKind::Forward => Some(0.0f64.to_bits()),
                    _ => None,
                }
        {
            return Err(format!("{} exact numerical options changed", kind.label()));
        }
        match kind {
            XyceDiodeAnalyticKind::Reverse => {
                if netlist.diagnostics.len() != 1
                    || netlist.diagnostics[0].code != "unknown-option"
                    || !netlist.diagnostics[0]
                        .message
                        .contains("TIMEINT.ABSTOL-1E-6")
                {
                    return Err(format!(
                        "{} must preserve Xyce's ignored fused ABSTOL token",
                        kind.label()
                    ));
                }
            }
            _ if !netlist.diagnostics.is_empty() => {
                return Err(format!("{} acquired parser diagnostics", kind.label()));
            }
            _ => {}
        }
        if netlist.models.len() != 1
            || netlist.elements.len()
                != match kind {
                    XyceDiodeAnalyticKind::Forward => 3,
                    _ => 3,
                }
        {
            return Err(format!(
                "{} exact model/topology count changed",
                kind.label()
            ));
        }
        let diode = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("D1"))
            .ok_or_else(|| format!("{} is missing D1", kind.label()))?;
        let expected_diode_nodes: &[&str] = match kind {
            XyceDiodeAnalyticKind::Forward => &["1", "2"],
            _ => &["0", "1"],
        };
        if diode.nodes.iter().map(String::as_str).collect::<Vec<_>>() != expected_diode_nodes
            || !Self::netlist_element_is_diode_analytic_oracle_candidate(netlist, diode)
        {
            return Err(format!("{} D1/model contract changed", kind.label()));
        }
        let capacitor = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("C1"))
            .ok_or_else(|| format!("{} is missing C1", kind.label()))?;
        let (expected_nodes, expected_value, expected_ic): (&[&str], Value, Value) = match kind {
            XyceDiodeAnalyticKind::Forward => (&["2", "0"], 1.0e-12, 0.0),
            XyceDiodeAnalyticKind::Reverse => (&["1", "0"], 10.0, 1.0),
            XyceDiodeAnalyticKind::Breakdown => (&["1", "0"], 1.0, 1.2),
        };
        if capacitor
            .nodes
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            != expected_nodes
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: Some(initial_voltage),
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == expected_value.to_bits()
                && initial_voltage.to_bits() == expected_ic.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{} C1 contract changed", kind.label()));
        }
        let source_name = match kind {
            XyceDiodeAnalyticKind::Forward => "V1",
            _ => "VDUMMY",
        };
        let source = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source_name))
            .ok_or_else(|| format!("{} is missing {source_name}", kind.label()))?;
        let (source_nodes, source_value): (&[&str], Value) = match kind {
            XyceDiodeAnalyticKind::Forward => (&["1", "0"], 0.2),
            _ => (&["2", "0"], 1.0),
        };
        if source.nodes.iter().map(String::as_str).collect::<Vec<_>>() != source_nodes
            || !matches!(&source.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) if value.to_bits() == source_value.to_bits())
        {
            return Err(format!("{} source contract changed", kind.label()));
        }
        Ok(())
    }

    fn validate_diode_analytic_output_domain(
        table: &XycePrnTable,
        kind: XyceDiodeAnalyticKind,
        stop: Value,
    ) -> Result<(), String> {
        let expected_probe = match kind {
            XyceDiodeAnalyticKind::Forward => "V(2)",
            _ => "V(1)",
        };
        if table.columns != ["Index", "TIME", expected_probe] || table.rows.len() < 2 {
            return Err(format!(
                "{} output layout changed: columns={:?}, rows={}",
                kind.label(),
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 3
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || previous_time.is_some_and(|previous| row[1] < previous)
            {
                return Err(format!(
                    "{} output row {index} is malformed: {row:?}",
                    kind.label()
                ));
            }
            previous_time = Some(row[1]);
        }
        let first = Self::xyce_default_prn_roundtrip(table.rows[0][1])?;
        let last = Self::xyce_default_prn_roundtrip(table.rows.last().expect("nonempty")[1])?;
        let expected_stop = Self::xyce_default_prn_roundtrip(stop)?;
        if first.to_bits() != 0.0f64.to_bits() || last.to_bits() != expected_stop.to_bits() {
            return Err(format!(
                "{} output domain changed: first={first}, last={last}, expected={expected_stop}",
                kind.label()
            ));
        }
        Ok(())
    }

    pub(super) fn diode_analytic_reference_table(
        actual: &XycePrnTable,
        kind: XyceDiodeAnalyticKind,
    ) -> Result<XycePrnTable, String> {
        const VT: Value = 0.025_864_186;
        let mut rows = Vec::with_capacity(actual.rows.len());
        for row in &actual.rows {
            let index = row[0];
            let time = Self::xyce_default_prn_roundtrip(row[1])?;
            let voltage = Self::xyce_default_prn_roundtrip(row[2])?;
            let reference = match kind {
                XyceDiodeAnalyticKind::Forward => {
                    let vin: Value = 0.2;
                    let c: Value = 1.0e-12;
                    let isat: Value = 1.0e-14;
                    let exponent = (vin / VT).exp();
                    let alpha = isat / (VT * c);
                    VT * (exponent - (exponent - 1.0) * (-alpha * time).exp()).ln()
                }
                XyceDiodeAnalyticKind::Breakdown => {
                    let initial: Value = 1.2;
                    let breakdown: Value = 1.0;
                    let ibv: Value = 1.0e-3;
                    let c: Value = 1.0;
                    initial
                        - VT * ((-(breakdown - initial) / VT).exp() * ibv / (c * VT) * time + 1.0)
                            .ln()
                }
                XyceDiodeAnalyticKind::Reverse => {
                    let c: Value = 10.0;
                    let initial: Value = 1.0;
                    let a = 3.0 * VT / std::f64::consts::E;
                    let sqrt3 = 3.0f64.sqrt();
                    -(c) * (voltage - initial
                        + a / 3.0 * ((voltage - a) / (initial - a)).abs().ln()
                        - a / 6.0
                            * ((voltage * voltage + a * voltage + a * a)
                                / (initial * initial + a * initial + a * a))
                                .abs()
                                .ln()
                        - a / sqrt3 * (2.0 * voltage / (a * sqrt3) + 1.0 / sqrt3).atan()
                        + a / sqrt3 * (2.0 * initial / (a * sqrt3) + 1.0 / sqrt3).atan())
                }
            };
            if !reference.is_finite() {
                return Err(format!(
                    "{} analytic generator produced a nonfinite reference",
                    kind.label()
                ));
            }
            rows.push(match kind {
                XyceDiodeAnalyticKind::Reverse => {
                    vec![index, Self::xyce_default_prn_roundtrip(reference)?, voltage]
                }
                _ => vec![index, time, Self::xyce_default_prn_roundtrip(reference)?],
            });
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    pub(super) fn validate_diode_analytic_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceDiodeAnalyticKind,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{} exceeded the shared deadline before planning",
                kind.label()
            ));
        }
        let plan = self.static_tran_plan_for_path_with_purpose(
            &deck.path,
            XyceStaticTranPlanPurpose::DiodeAnalyticOracle,
        )?;
        let parsed = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .map_err(|error| format!("{} parse failed: {error}", kind.label()))?;
        Self::validate_diode_analytic_typed_contract(&plan, &parsed, kind)?;
        if abort.is_aborted() {
            return Err(format!(
                "{} parse/typed validation exceeded the shared deadline",
                kind.label()
            ));
        }
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => {
                    format!("{} execution exceeded the shared deadline", kind.label())
                }
                other => format!("{} execution failed: {other}", kind.label()),
            })?;
        Self::validate_diode_analytic_typed_contract(&plan, &netlist, kind)?;
        let actual = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
        Self::validate_diode_analytic_output_domain(&actual, kind, plan.tran.stop)?;
        let reference = Self::diode_analytic_reference_table(&actual, kind)?;
        if kind == XyceDiodeAnalyticKind::Reverse {
            let generated_to_simulation =
                self.compare_xyce_verify_transient_tables(&reference, &actual);
            let simulation_to_generated =
                self.compare_xyce_verify_transient_tables(&actual, &reference);
            let passed = generated_to_simulation
                .as_ref()
                .is_ok_and(|mismatches| mismatches.is_empty())
                || simulation_to_generated
                    .as_ref()
                    .is_ok_and(|mismatches| mismatches.is_empty());
            if !passed {
                return Err(format!(
                    "{} failed both historical xyce_verify directions: generated->simulation={generated_to_simulation:?}, simulation->generated={simulation_to_generated:?}",
                    kind.label(),
                ));
            }
        } else {
            let mismatches = self.compare_xyce_verify_transient_tables(&reference, &actual)?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{} produced {} Release-7.10 xyce_verify mismatch(es): {mismatches:?}",
                    kind.label(),
                    mismatches.len()
                ));
            }
        }
        if abort.is_aborted() {
            return Err(format!(
                "{} comparison exceeded the shared deadline",
                kind.label()
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

    fn canonical_deck(root: &Path, kind: XyceDiodeAnalyticKind) -> XyceDeck {
        XyceDeck {
            path: root.join(kind.path()),
            section: XyceDeckSection::Netlists,
            relative_path: kind.path().to_string(),
        }
    }

    #[test]
    fn diode_analytic_typed_plans_are_exact() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for kind in XyceDiodeAnalyticKind::ALL {
            let deck = canonical_deck(&root, kind);
            let plan = runner
                .static_tran_plan_for_path_with_purpose(
                    &deck.path,
                    XyceStaticTranPlanPurpose::DiodeAnalyticOracle,
                )
                .expect("canonical DIODE_ANALYTIC plan builds");
            let netlist = XyceTestRunner::parse_xyce_netlist(&plan.source, &plan.deck_path)
                .expect("canonical DIODE_ANALYTIC deck parses");
            XyceTestRunner::validate_diode_analytic_typed_contract(&plan, &netlist, kind)
                .expect("canonical DIODE_ANALYTIC typed contract qualifies");
        }
    }

    #[test]
    fn diode_analytic_formulae_are_directional_and_nontrivial() {
        for kind in XyceDiodeAnalyticKind::ALL {
            let probe = if kind == XyceDiodeAnalyticKind::Forward {
                "V(2)"
            } else {
                "V(1)"
            };
            let sample = XycePrnTable {
                columns: vec!["Index".into(), "TIME".into(), probe.into()],
                rows: vec![
                    vec![
                        0.0,
                        0.0,
                        if kind == XyceDiodeAnalyticKind::Forward {
                            0.0
                        } else if kind == XyceDiodeAnalyticKind::Breakdown {
                            1.2
                        } else {
                            1.0
                        },
                    ],
                    vec![1.0, 0.5, 0.9],
                ],
            };
            let reference = XyceTestRunner::diode_analytic_reference_table(&sample, kind)
                .expect("analytic formula is finite");
            assert_eq!(reference.columns, sample.columns);
            assert_ne!(reference.rows[1], sample.rows[1]);
            if kind == XyceDiodeAnalyticKind::Reverse {
                assert_ne!(reference.rows[1][1], sample.rows[1][1]);
                assert_eq!(reference.rows[1][2], sample.rows[1][2]);
            } else {
                assert_eq!(reference.rows[1][1], sample.rows[1][1]);
                assert_ne!(reference.rows[1][2], sample.rows[1][2]);
            }
        }
    }

    #[test]
    fn diode_analytic_typed_admission_rejects_model_drift() {
        let temporary = tempfile::Builder::new()
            .prefix("rspice-xyce-diode-analytic-model-drift-")
            .tempdir()
            .expect("create DIODE_ANALYTIC fixture root");
        let kind = XyceDiodeAnalyticKind::Breakdown;
        let path = temporary.path().join(kind.path());
        fs::create_dir_all(path.parent().expect("family parent")).expect("create family");
        let source =
            fs::read_to_string(corpus_root().join(kind.path())).expect("read canonical source");
        fs::write(&path, &source).expect("copy canonical breakdown deck");
        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            format!("{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n", kind.path()),
        )
        .expect("write DIODE_ANALYTIC wrapper manifest");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        runner
            .static_tran_plan_for_path_with_purpose(
                &path,
                XyceStaticTranPlanPurpose::DiodeAnalyticOracle,
            )
            .expect("canonical breakdown deck passes typed admission outside the corpus");
        fs::write(&path, source.replace("BV=1", "BV=2")).expect("mutate breakdown model");
        assert!(
            runner
                .static_tran_plan_for_path_with_purpose(
                    &path,
                    XyceStaticTranPlanPurpose::DiodeAnalyticOracle,
                )
                .is_err(),
            "a changed diode model must fail typed analytic admission"
        );
    }
}
