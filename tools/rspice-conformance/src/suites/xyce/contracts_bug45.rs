use super::*;
use rspice_core::netlist::{DiagnosticSeverity, SourceSpec};

pub(super) const BUG45_CONTRACT: &str =
    "bug45_unknown_expression_model_parameter_warning_wrapper_owner";

const LABEL: &str = "BUG_45_SON unknown expression-valued diode parameter";
pub(super) const RECORD: &str = "netlists/certification_tests/bug_45_son/diode.cir";
#[cfg(test)]
const DECK_PATH: &str = "Netlists/Certification_Tests/BUG_45_SON/diode.cir";
const GOLD_PATH: &str = "OutputData/Certification_Tests/BUG_45_SON/diode.cir.prn";
const WARNING_CODE: &str = "xyce-unknown-diode-model-parameter";
const WARNING_MESSAGE: &str =
    "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter ignored.";
const HISTORICAL_LOCATION_PATTERN: &str = "Netlist warning in file diode.cir at or near line 29";
const HISTORICAL_PARAMETER_PATTERN: &str =
    "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter";
impl XyceTestRunner {
    fn read_bug45_sources(&self, deck: &XyceDeck) -> Result<(Vec<u8>, Vec<u8>), String> {
        Ok((
            Self::read_corpus_canonical(LABEL, &deck.path)?,
            Self::read_corpus_canonical(LABEL, &self.root.join(GOLD_PATH))?,
        ))
    }

    fn bug45_nodes_match(element: &rspice_core::netlist::Element, expected: &[&str]) -> bool {
        element.provenance == ElementProvenance::Authored
            && element
                .nodes
                .iter()
                .map(String::as_str)
                .eq(expected.iter().copied())
    }

    fn bug45_validate_resistor(
        elements: &BTreeMap<String, &rspice_core::netlist::Element>,
        name: &str,
        nodes: &[&str],
        expected_value: Value,
    ) -> Result<(), String> {
        let element = elements
            .get(&name.to_ascii_uppercase())
            .ok_or_else(|| format!("{LABEL} lost {name}"))?;
        if !Self::bug45_nodes_match(element, nodes)
            || !matches!(&element.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == expected_value.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!(
                "{LABEL} {name} topology/value changed: {element:?}"
            ));
        }
        Ok(())
    }

    fn bug45_validate_warning(netlist: &Netlist, deck_path: &Path) -> Result<(), String> {
        if netlist.diagnostics.len() != 1 {
            return Err(format!(
                "{LABEL} requires exactly one ignored-parameter warning, found {:?}",
                netlist.diagnostics
            ));
        }
        let diagnostic = &netlist.diagnostics[0];
        let rendered = diagnostic.xyce_legacy_warning_lines();
        if diagnostic.severity != DiagnosticSeverity::Warning
            || diagnostic.code != WARNING_CODE
            || diagnostic.line != 29
            || diagnostic.origin.as_ref().map(|origin| origin.line) != Some(29)
            || diagnostic
                .origin
                .as_ref()
                .and_then(|origin| origin.path.as_deref())
                != Some(deck_path)
            || diagnostic.message != WARNING_MESSAGE
            || rendered
                != Some([
                    HISTORICAL_LOCATION_PATTERN.to_string(),
                    WARNING_MESSAGE.to_string(),
                ])
        {
            return Err(format!(
                "{LABEL} structured warning no longer reproduces the two ordered historical predicates {HISTORICAL_LOCATION_PATTERN:?} / {HISTORICAL_PARAMETER_PATTERN:?}: {diagnostic:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug45_typed_contract(
        &self,
        deck: &XyceDeck,
        source: &str,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_deck_with_sealed_source_and_purpose(
            deck,
            source,
            XyceStaticTranPlanPurpose::AbsoluteOracle,
        )?;
        let expected_gold = self.root.join(GOLD_PATH);
        let probes = plan
            .require_print(LABEL)?
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.deck_path != deck.path
            || !matches!(&plan.oracle, XyceStaticTranOracle::Waveform(path) if Self::same_path(path, &expected_gold))
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.wrapper_tolerance.is_some()
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.tran.step.to_bits() != 2.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 2.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || probes != ["v(3)", "v(2)", "v(4)"]
        {
            return Err(format!(
                "{LABEL} exact TRAN/PRINT/oracle plan changed: {plan:?}"
            ));
        }
        plan.validate_executable_oracle_shape()?;

        let netlist = Self::parse_xyce_netlist(source, &deck.path)
            .map_err(|error| format!("{LABEL} source no longer parses: {error}"))?;
        if netlist.elements.len() != 9
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.fft_analyses.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} typed envelope changed: elements={}, models={}, analyses={}, outputs={}",
                netlist.elements.len(),
                netlist.models.len(),
                netlist.analyses.len(),
                netlist.output_requests.len()
            ));
        }
        Self::bug45_validate_warning(&netlist, &deck.path)?;

        let model = &netlist.models[0];
        let mut model_params = model
            .params
            .iter()
            .map(|(name, value)| (name.to_ascii_uppercase(), value.to_bits()))
            .collect::<Vec<_>>();
        model_params.sort();
        let expected_model_values: [(&str, Value); 15] = [
            ("AF", 1.0),
            ("BOGOPARAM", 3.0),
            ("BV", 600.0),
            ("CJO", 1.95e-11),
            ("EG", 1.36),
            ("FC", 0.9),
            ("IBV", 1.0e-4),
            ("IS", 4.0e-10),
            ("KF", 0.0),
            ("M", 0.38),
            ("N", 1.48),
            ("RS", 0.105),
            ("TT", 8.0e-7),
            ("VJ", 0.4),
            ("XTI", -8.0),
        ];
        let mut expected_model_params = expected_model_values
            .into_iter()
            .map(|(name, value)| (name.to_string(), value.to_bits()))
            .collect::<Vec<_>>();
        expected_model_params.sort();
        if model.name != "D1N3940"
            || model.model_type != "D"
            || model_params != expected_model_params
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} exact D1N3940 model changed: {model:?}"));
        }

        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_uppercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.len() != 9 {
            return Err(format!("{LABEL} element-name census changed"));
        }
        let vcc = elements
            .get("VCC")
            .ok_or_else(|| format!("{LABEL} lost VCC"))?;
        if !Self::bug45_nodes_match(vcc, &["1", "0"])
            || !matches!(&vcc.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 5.0f64.to_bits())
        {
            return Err(format!("{LABEL} VCC changed: {vcc:?}"));
        }
        let vin = elements
            .get("VIN")
            .ok_or_else(|| format!("{LABEL} lost VIN"))?;
        if !Self::bug45_nodes_match(vin, &["3", "0"])
            || !matches!(&vin.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            }) if offset.to_bits() == 0.0f64.to_bits()
                && amplitude.to_bits() == 10.0f64.to_bits()
                && frequency.to_bits() == 1_000.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!("{LABEL} VIN changed: {vin:?}"));
        }
        for (name, nodes) in [("D1", ["2", "1"]), ("D2", ["0", "2"])] {
            let diode = elements
                .get(name)
                .ok_or_else(|| format!("{LABEL} lost {name}"))?;
            if !Self::bug45_nodes_match(diode, &nodes)
                || !matches!(&diode.kind, ElementKind::Diode {
                    model,
                    instance_params,
                    deferred_params,
                } if model == "D1N3940"
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
            {
                return Err(format!("{LABEL} {name} changed: {diode:?}"));
            }
        }
        Self::bug45_validate_resistor(&elements, "R1", &["2", "3"], 1_000.0)?;
        Self::bug45_validate_resistor(&elements, "R2", &["1", "2"], 3_300.0)?;
        Self::bug45_validate_resistor(&elements, "R3", &["2", "0"], 3_300.0)?;
        Self::bug45_validate_resistor(&elements, "R4", &["4", "0"], 5_600.0)?;
        let capacitor = elements
            .get("C1")
            .ok_or_else(|| format!("{LABEL} lost C1"))?;
        if !Self::bug45_nodes_match(capacitor, &["2", "4"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 0.47e-6f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} C1 changed: {capacitor:?}"));
        }
        Ok(plan)
    }

    fn bug45_compare_actual_good_to_retained_test(
        &self,
        actual: &XycePrnTable,
        retained: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let mut mismatches = self.compare_xyce_verify_transient_tables(actual, retained)?;
        // The historical API names its first operand GOOD rather than
        // "expected". Preserve the public RSpice mismatch convention after
        // applying the directional metric: retained gold is expected and the
        // newly simulated waveform is actual.
        for mismatch in &mut mismatches {
            std::mem::swap(&mut mismatch.expected, &mut mismatch.actual);
        }
        Ok(mismatches)
    }

    fn bug45_validate_output_table(table: &XycePrnTable, retained: bool) -> Result<(), String> {
        if table.columns != ["Index", "TIME", "V(3)", "V(2)", "V(4)"]
            || table.rows.len() < 2
            || retained && table.rows.len() != 250
        {
            return Err(format!(
                "{LABEL} {} output framing changed: columns={:?}, rows={}",
                if retained { "retained" } else { "fresh" },
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 5
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || previous.is_some_and(|time| row[1] <= time)
            {
                return Err(format!(
                    "{LABEL} {} row {index} is malformed: {row:?}",
                    if retained { "retained" } else { "fresh" }
                ));
            }
            previous = Some(row[1]);
        }
        let first = table.rows[0][1];
        let last = table.rows.last().expect("nonempty table")[1];
        if first.to_bits() != 0.0f64.to_bits() || last.to_bits() != 2.0e-3f64.to_bits() {
            return Err(format!(
                "{LABEL} {} output domain changed: {first}..{last}",
                if retained { "retained" } else { "fresh" }
            ));
        }
        Ok(())
    }

    pub(super) fn run_bug45_contract(&self, deck: &XyceDeck, start: Instant) -> XyceTestResult {
        let (source, retained_gold) = match self.read_bug45_sources(deck) {
            Ok(sources) => sources,
            Err(error) => {
                return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
            }
        };
        let source = match String::from_utf8(source) {
            Ok(source) => source,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} source is not UTF-8: {error}"),
                    Vec::new(),
                );
            }
        };
        let plan = match self.validate_bug45_typed_contract(deck, &source) {
            Ok(plan) => plan,
            Err(error) => {
                return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
            }
        };
        if let Err(error) = plan.require_waveform_reference_path(LABEL) {
            return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
        }
        let retained_gold = match String::from_utf8(retained_gold) {
            Ok(retained_gold) => retained_gold,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} retained PRN is not UTF-8: {error}"),
                    Vec::new(),
                );
            }
        };
        let retained = match Self::parse_xyce_verify_tran_reference_table(&retained_gold) {
            Ok(table) => table,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} retained PRN is invalid: {error}"),
                    Vec::new(),
                );
            }
        };
        if let Err(error) = Self::bug45_validate_output_table(&retained, true) {
            return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
        }

        let (netlist, result) = match self.run_transient_family_plan(&plan, start, None, None) {
            Ok(run) => run,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!(
                        "{LABEL} simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} simulation failed instead of exiting normally: {error}"),
                    Vec::new(),
                );
            }
        };
        if let Err(error) = Self::bug45_validate_warning(&netlist, &deck.path) {
            return self.failure_result(
                deck,
                start,
                BUG45_CONTRACT,
                format!("{LABEL} executed netlist warning contract changed: {error}"),
                Vec::new(),
            );
        }
        let actual = match Self::transient_family_result_to_prn_table(&plan, &netlist, &result) {
            Ok(table) => table,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} fresh PRN serialization failed: {error}"),
                    Vec::new(),
                );
            }
        };
        if let Err(error) = Self::bug45_validate_output_table(&actual, false) {
            return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
        }
        let mismatches = match self.bug45_compare_actual_good_to_retained_test(&actual, &retained) {
            Ok(mismatches) => mismatches,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} directional xyce_verify comparison failed: {error}"),
                    Vec::new(),
                );
            }
        };
        if !mismatches.is_empty() {
            return self.failure_result(
                deck,
                start,
                BUG45_CONTRACT,
                format!(
                    "{} {LABEL} actual-GOOD/retained-TEST integrated-RMS mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            );
        }
        self.passed_or_tran_side_output_failure(
            deck,
            start,
            BUG45_CONTRACT,
            &plan,
            &netlist,
            &result,
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

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(DECK_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: DECK_PATH.to_string(),
        }
    }

    #[test]
    fn bug45_typed_warning_model_topology_and_plan_are_exact() {
        let root = corpus_root();
        let deck = canonical_deck(&root);
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let source = fs::read_to_string(&deck.path).expect("read BUG45 source");
        runner
            .validate_bug45_typed_contract(&deck, &source)
            .expect("canonical BUG45 typed contract passes");
        for mutation in [
            source.replace("BOGOPARAM", "OTHERPARAM"),
            source.replace("{1+2}", "{1+3}"),
            source.replace("BOGOPARAM", "TLEV"),
            source.replace(".TRAN 2ns 2ms", ".TRAN 3ns 2ms"),
            source.replace("V(3) V(2) V(4)", "V(3) V(4) V(2)"),
            source.replace("D2 0 2 D1N3940", "D2 2 0 D1N3940"),
        ] {
            assert!(
                runner
                    .validate_bug45_typed_contract(&deck, &mutation)
                    .is_err(),
                "BUG45 semantic counterfactual must fail closed"
            );
        }
    }

    #[test]
    fn bug45_comparator_uses_fresh_actual_as_good() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let table = |times: &[Value], values: [Value; 3]| XycePrnTable {
            columns: vec![
                "Index".into(),
                "TIME".into(),
                "V(3)".into(),
                "V(2)".into(),
                "V(4)".into(),
            ],
            rows: times
                .iter()
                .enumerate()
                .map(|(index, time)| vec![index as Value, *time, values[0], values[1], values[2]])
                .collect(),
        };
        let actual_good = table(&[0.0, 1.0, 2.0], [1.0, 2.0, 3.0]);
        let retained_test = table(&[0.0, 1.0], [1.0, 2.0, 3.0]);
        assert!(
            runner
                .bug45_compare_actual_good_to_retained_test(&actual_good, &retained_test)
                .expect("actual-GOOD comparison is valid")
                .is_empty()
        );
        assert!(
            runner
                .compare_xyce_verify_transient_tables(&retained_test, &actual_good)
                .is_err(),
            "reversing GOOD/TEST must fail the directional domain contract"
        );

        let actual_good = table(&[0.0, 1.0, 2.0], [10.0, 20.0, 30.0]);
        let retained_test = table(&[0.0, 1.0], [0.5, 1.0, 1.5]);
        let mismatches = runner
            .bug45_compare_actual_good_to_retained_test(&actual_good, &retained_test)
            .expect("asymmetric actual-GOOD comparison is valid");
        assert_eq!(mismatches.len(), 3);
        assert_eq!(mismatches[0].expected.to_bits(), 0.5f64.to_bits());
        assert_eq!(mismatches[0].actual.to_bits(), 10.0f64.to_bits());
    }
}
