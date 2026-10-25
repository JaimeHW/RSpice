use super::*;

const LABEL: &str = "TIA/TRAP passive analytic RC wrappers";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum XyceTiaPassiveAnalyticKind {
    CapacitorInstanceIc,
    PositionalIc,
}

impl XyceTiaPassiveAnalyticKind {
    const ALL: [Self; 2] = [Self::CapacitorInstanceIc, Self::PositionalIc];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|kind| kind.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::CapacitorInstanceIc => "tia_trap_capacitor_instance_ic_analytic_wrapper",
            Self::PositionalIc => "tia_trap_positional_ic_analytic_wrapper",
        }
    }

    #[cfg(test)]
    fn path(self) -> &'static str {
        match self {
            Self::CapacitorInstanceIc => "Netlists/TIA/TRAP/CAPACITOR/capacitor.cir",
            Self::PositionalIc => "Netlists/TIA/TRAP/IC_AND_NODESET/ic_cap2.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::CapacitorInstanceIc => "netlists/tia/trap/capacitor/capacitor.cir",
            Self::PositionalIc => "netlists/tia/trap/ic_and_nodeset/ic_cap2.cir",
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::CapacitorInstanceIc => "capacitor.cir",
            Self::PositionalIc => "ic_cap2.cir",
        }
    }
}

impl XyceTestRunner {
    fn validate_tia_passive_analytic_typed_contract(
        &self,
        kind: XyceTiaPassiveAnalyticKind,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, XyceAnalyticRcSpecification), String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::AnalyticOracle,
        )?;
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 5.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 1 || !print.probes[0].eq_ignore_ascii_case("V(1)")
            })
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                kind.file_name()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", kind.file_name()))?;
        if netlist.elements.len() != 3
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                kind.file_name()
            ));
        }
        let expected_lte = match kind {
            XyceTiaPassiveAnalyticKind::CapacitorInstanceIc => {
                Some(TransientLteReference::PointGlobal)
            }
            XyceTiaPassiveAnalyticKind::PositionalIc => None,
        };
        if !Self::analytic_timeint_only_options_match(
            &netlist.options,
            None,
            None,
            Some("7"),
            expected_lte,
        ) {
            return Err(format!("{LABEL} {} option state changed", kind.file_name()));
        }
        let [capacitor, resistor, voltage] = netlist.elements.as_slice() else {
            unreachable!("TIA passive element count was checked");
        };
        if !capacitor.name.eq_ignore_ascii_case("C1")
            || capacitor.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&capacitor.nodes, &["1", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0e-6f64.to_bits()
                && initial_voltage.map(Value::to_bits) == match kind {
                    XyceTiaPassiveAnalyticKind::CapacitorInstanceIc => Some(1.0f64.to_bits()),
                    XyceTiaPassiveAnalyticKind::PositionalIc => None,
                }
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&resistor.nodes, &["1", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0e3f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !voltage.name.eq_ignore_ascii_case("V1")
            || voltage.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&voltage.nodes, &["2", "0"])
            || !matches!(&voltage.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Dc(value)
            ) if value.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} passive topology changed",
                kind.file_name()
            ));
        }
        match kind {
            XyceTiaPassiveAnalyticKind::CapacitorInstanceIc
                if !netlist.initial_conditions.is_empty() =>
            {
                return Err(format!(
                    "{LABEL} capacitor-local IC acquired a .IC directive"
                ));
            }
            XyceTiaPassiveAnalyticKind::PositionalIc
                if !matches!(netlist.initial_conditions.as_slice(), [condition]
                    if condition.node.eq_ignore_ascii_case("1")
                        && condition.voltage.to_bits() == 1.0f64.to_bits()
                        && condition.voltage_expr.is_none()) =>
            {
                return Err(format!("{LABEL} positional .IC changed"));
            }
            _ => {}
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
        } if step.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == 5.0e-3f64.to_bits())
        {
            return Err(format!("{LABEL} {} typed TRAN changed", kind.file_name()));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 1
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || request.dependencies[0].expression
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
        {
            return Err(format!(
                "{LABEL} {} PRINT request changed",
                kind.file_name()
            ));
        }
        let effective_capacitance = Self::effective_capacitor_value(&netlist, "C1")
            .filter(|value| value.to_bits() == 1.0e-6f64.to_bits())
            .ok_or_else(|| format!("{LABEL} capacitance did not resolve exactly"))?;
        let effective_resistance = Self::effective_resistor_value(&netlist, "R1")?
            .filter(|value| value.to_bits() == 1.0e3f64.to_bits())
            .ok_or_else(|| format!("{LABEL} resistance did not resolve exactly"))?;
        let time_constant = effective_resistance * effective_capacitance;
        if time_constant.to_bits() != 1.0e-3f64.to_bits() {
            return Err(format!("{LABEL} time constant changed to {time_constant}"));
        }
        Ok((
            plan,
            XyceAnalyticRcSpecification {
                output_node: "1".to_string(),
                source_value: 0.0,
                initial_voltage: 1.0,
                resistance: effective_resistance,
                capacitance: effective_capacitance,
                time_constant,
            },
        ))
    }

    pub(super) fn validate_tia_passive_analytic_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceTiaPassiveAnalyticKind,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before planning"));
        }
        let bytes = fs::read(&deck.path)
            .map_err(|error| format!("failed to read {LABEL} {}: {error}", kind.file_name()))?;
        let source = std::str::from_utf8(&bytes)
            .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", kind.file_name()))?;
        let (plan, specification) =
            self.validate_tia_passive_analytic_typed_contract(kind, source, &deck.path)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} validation exceeded deadline"));
        }
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                other => format!("{LABEL} execution failed: {other}"),
            })?;
        let actual = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{LABEL} output observation failed: {error}"))?;
        Self::validate_analytic_rc_initial_sample(&actual, &specification)?;
        Self::validate_analytic_rc_complete_time_domain(&actual, plan.tran.stop)?;
        let reference = Self::analytic_rc_reference_table(&actual, &specification)?;
        let mismatches = self.compare_xyce_verify_transient_tables(&reference, &actual)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} {} produced {} xyce_verify mismatch(es): {mismatches:?}",
                kind.file_name(),
                mismatches.len()
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} comparison exceeded deadline"));
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

    fn canonical_deck(root: &Path, kind: XyceTiaPassiveAnalyticKind) -> XyceDeck {
        XyceDeck {
            path: root.join(kind.path()),
            section: XyceDeckSection::Netlists,
            relative_path: kind.path().to_string(),
        }
    }

    fn typed_fixture(
        label: &str,
        kind: XyceTiaPassiveAnalyticKind,
        source: &str,
    ) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-tia-passive-typed-{label}-"))
            .tempdir()
            .expect("create typed fixture");
        let root = temporary.path();
        let deck = canonical_deck(root, kind);
        fs::create_dir_all(deck.path.parent().expect("candidate parent"))
            .expect("create candidate parent");
        fs::write(&deck.path, source).expect("write typed fixture source");
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n", kind.path()),
        )
        .expect("write typed fixture wrapper manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn tia_passive_candidates_have_exact_typed_contracts() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for kind in XyceTiaPassiveAnalyticKind::ALL {
            let path = root.join(kind.path());
            let source = fs::read_to_string(&path).expect("read TIA passive source");
            runner
                .validate_tia_passive_analytic_typed_contract(kind, &source, &path)
                .expect("canonical TIA passive typed contract passes");
        }
    }

    #[test]
    fn tia_passive_typed_mutations_fail_closed() {
        let root = corpus_root();
        for kind in XyceTiaPassiveAnalyticKind::ALL {
            let canonical = fs::read_to_string(root.join(kind.path()))
                .expect("read canonical TIA passive source");
            let mutations = match kind {
                XyceTiaPassiveAnalyticKind::CapacitorInstanceIc => vec![
                    canonical.replacen("IC=1", "IC=2", 1),
                    canonical.replacen("method=7", "method=8", 1),
                    canonical.replacen("newlte=1", "newlte=2", 1),
                    canonical.replacen("R1 1 2 1K", "R1 1 2 2K", 1),
                    canonical.replacen(".print tran v(1)", ".print tran v(2)", 1),
                    canonical.replacen(".tran 0 5ms", ".tran 0 6ms", 1),
                ],
                XyceTiaPassiveAnalyticKind::PositionalIc => vec![
                    canonical.replacen(".ic 1 1.0", ".ic 2 1.0", 1),
                    canonical.replacen(".ic 1 1.0", ".ic 1 2.0", 1),
                    canonical.replacen("method=7", "method=8", 1),
                    canonical.replacen("R1 1 2 1K", "R1 1 2 2K", 1),
                    canonical.replacen(".print tran v(1)", ".print tran v(2)", 1),
                    canonical.replacen(".tran 0 5ms", ".tran 0 6ms", 1),
                ],
            };
            for (index, mutation) in mutations.into_iter().enumerate() {
                let (_temporary, deck, runner) =
                    typed_fixture(&format!("{kind:?}-{index}"), kind, &mutation);
                assert!(
                    runner
                        .validate_tia_passive_analytic_typed_contract(kind, &mutation, &deck.path,)
                        .is_err(),
                    "{kind:?} mutation {index} must fail closed"
                );
            }
        }
    }

    #[test]
    fn tia_passive_oracles_execute_both_initial_condition_forms() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for kind in XyceTiaPassiveAnalyticKind::ALL {
            let deck = canonical_deck(&root, kind);
            runner
                .validate_tia_passive_analytic_oracle(&deck, kind, Instant::now())
                .expect("canonical TIA passive oracle passes");
        }
    }

    #[test]
    fn tia_passive_oracle_rejects_expired_deadline() {
        let root = corpus_root();
        let config = XyceRunnerConfig {
            max_time_per_test_ms: 1,
            ..XyceRunnerConfig::default()
        };
        let runner = XyceTestRunner::new(&root, config);
        let deck = canonical_deck(&root, XyceTiaPassiveAnalyticKind::PositionalIc);
        assert!(
            runner
                .validate_tia_passive_analytic_oracle(
                    &deck,
                    XyceTiaPassiveAnalyticKind::PositionalIc,
                    Instant::now() - Duration::from_millis(10),
                )
                .is_err()
        );
    }
}
