use super::*;
use rspice_core::netlist::{ElementKind, ElementProvenance, SourceSpec};

const LABEL: &str = "MOSFET_ParamAliases classic-MOS parameter alias relation";
const FAMILY_PATH: &str = "Netlists/MOSFET_ParamAliases";

type NormalizedModels = Vec<(String, String, Vec<(String, u64)>)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum MosfetParamAliasRole {
    Level1Owner,
    Level1Alt,
    Level2Owner,
    Level2Alt,
    Level3Owner,
    Level3Alt,
    Level6Owner,
    Level6Alt,
}

impl MosfetParamAliasRole {
    const ALL: [Self; 8] = [
        Self::Level1Owner,
        Self::Level1Alt,
        Self::Level2Owner,
        Self::Level2Alt,
        Self::Level3Owner,
        Self::Level3Alt,
        Self::Level6Owner,
        Self::Level6Alt,
    ];
    #[cfg(test)]
    const OWNERS: [Self; 4] = [
        Self::Level1Owner,
        Self::Level2Owner,
        Self::Level3Owner,
        Self::Level6Owner,
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    fn level(self) -> u8 {
        match self {
            Self::Level1Owner | Self::Level1Alt => 1,
            Self::Level2Owner | Self::Level2Alt => 2,
            Self::Level3Owner | Self::Level3Alt => 3,
            Self::Level6Owner | Self::Level6Alt => 6,
        }
    }

    fn is_owner(self) -> bool {
        matches!(
            self,
            Self::Level1Owner | Self::Level2Owner | Self::Level3Owner | Self::Level6Owner
        )
    }

    fn owner(self) -> Self {
        match self.level() {
            1 => Self::Level1Owner,
            2 => Self::Level2Owner,
            3 => Self::Level3Owner,
            6 => Self::Level6Owner,
            _ => unreachable!(),
        }
    }
    fn alt(self) -> Self {
        match self.level() {
            1 => Self::Level1Alt,
            2 => Self::Level2Alt,
            3 => Self::Level3Alt,
            6 => Self::Level6Alt,
            _ => unreachable!(),
        }
    }

    fn file_name(self) -> String {
        if self.is_owner() {
            format!("invert{}.cir", self.level())
        } else {
            format!("invert{}_alt.cir", self.level())
        }
    }
    fn path(self) -> String {
        format!("{FAMILY_PATH}/{}", self.file_name())
    }
    fn record(self) -> String {
        XyceTestRunner::normalize_manifest_key(&self.path())
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::Level1Owner => "mosfet_param_alias_level1_wrapper_owner",
            Self::Level1Alt => "mosfet_param_alias_level1_alt_worker",
            Self::Level2Owner => "mosfet_param_alias_level2_wrapper_owner",
            Self::Level2Alt => "mosfet_param_alias_level2_alt_worker",
            Self::Level3Owner => "mosfet_param_alias_level3_wrapper_owner",
            Self::Level3Alt => "mosfet_param_alias_level3_alt_worker",
            Self::Level6Owner => "mosfet_param_alias_level6_wrapper_owner",
            Self::Level6Alt => "mosfet_param_alias_level6_alt_worker",
        }
    }
}

impl XyceTestRunner {
    fn mosfet_param_alias_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(left, right)| left.eq_ignore_ascii_case(right))
    }

    fn mosfet_param_alias_exact_params(
        actual: &[(String, Value)],
        expected: &[(&str, Value)],
    ) -> bool {
        actual.len() == expected.len()
            && actual.iter().zip(expected).all(
                |((actual_name, actual_value), (expected_name, expected_value))| {
                    actual_name.eq_ignore_ascii_case(expected_name)
                        && actual_value.to_bits() == expected_value.to_bits()
                },
            )
    }

    fn normalized_mosfet_param_alias_models(
        netlist: &Netlist,
        role: MosfetParamAliasRole,
    ) -> Result<NormalizedModels, String> {
        let mut normalized = Vec::new();
        for model in &netlist.models {
            if !model.expr_params.is_empty()
                || !model.string_params.is_empty()
                || !model.string_vector_params.is_empty()
                || !model.real_vector_params.is_empty()
                || !model.real_vector_expr_params.is_empty()
                || !model.integer_vector_params.is_empty()
            {
                return Err(format!(
                    "{LABEL} model {} acquired deferred state",
                    model.name
                ));
            }
            let expected_param_count = if role.level() == 3 { 39 } else { 33 };
            if model.params.len() != expected_param_count {
                return Err(format!(
                    "{LABEL} model {} parameter census changed: expected {expected_param_count}, got {}",
                    model.name,
                    model.params.len()
                ));
            }
            let names = model
                .params
                .iter()
                .map(|(name, _)| name.to_ascii_uppercase())
                .collect::<Vec<_>>();
            let count = |name: &str| {
                names
                    .iter()
                    .filter(|candidate| candidate.as_str() == name)
                    .count()
            };
            let expected_mobility = if role.is_owner() { "UO" } else { "U0" };
            let expected_threshold = if role.is_owner() { "VTO" } else { "VT0" };
            if count(expected_mobility) != 1
                || count(expected_threshold) != 1
                || count(if role.is_owner() { "U0" } else { "UO" }) != 0
                || count(if role.is_owner() { "VT0" } else { "VTO" }) != 0
                || count("VTH0") != 0
                || names.iter().collect::<BTreeSet<_>>().len() != names.len()
            {
                return Err(format!(
                    "{LABEL} model {} alias inventory changed",
                    model.name
                ));
            }
            let mut params = model
                .params
                .iter()
                .map(|(name, value)| {
                    let name = match name.to_ascii_uppercase().as_str() {
                        "UO" | "U0" => "U0".to_string(),
                        "VTO" | "VT0" => "VT0".to_string(),
                        other => other.to_string(),
                    };
                    (name, value.to_bits())
                })
                .collect::<Vec<_>>();
            params.sort();
            normalized.push((
                model.name.to_ascii_uppercase(),
                model.model_type.to_ascii_uppercase(),
                params,
            ));
        }
        normalized.sort();
        Ok(normalized)
    }

    fn validate_mosfet_param_alias_typed_plan(
        &self,
        role: MosfetParamAliasRole,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist, NormalizedModels), String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::ClassicMosParameterAliasRelationalFamily,
        )?;
        let expected_contract = if role.is_owner() {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 20.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 30.0e-6f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 3
                    || !print.probes[0].eq_ignore_ascii_case("V(VOUT)")
                    || !print.probes[1].eq_ignore_ascii_case("V(IN)")
                    || !print.probes[2].eq_ignore_ascii_case("V(1)")
            })
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.file_name()))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 7
            || netlist.models.len() != 2
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
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !Self::netlist_is_native_classic_mos_parameter_alias_envelope(&netlist)
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from(["vdddev", "rin", "vin1", "r1", "c2", "mn1", "mp1"])
        {
            return Err(format!(
                "{LABEL} {} element inventory changed",
                role.file_name()
            ));
        }
        let voltage = elements["vdddev"];
        if voltage.provenance != ElementProvenance::Authored
            || !Self::mosfet_param_alias_nodes_match(&voltage.nodes, &["VDD", "0"])
            || !matches!(voltage.kind, ElementKind::VoltageSource(SourceSpec::Dc(value)) if value.to_bits() == 5.0f64.to_bits())
        {
            return Err(format!("{LABEL} VDD source changed"));
        }
        let pulse = elements["vin1"];
        if pulse.provenance != ElementProvenance::Authored
            || !Self::mosfet_param_alias_nodes_match(&pulse.nodes, &["1", "0"])
            || !matches!(&pulse.kind, ElementKind::VoltageSource(SourceSpec::DcTransient { dc_value, transient }) if dc_value.to_bits() == 5.0f64.to_bits() && matches!(transient.as_ref(), SourceSpec::Pulse { v1, v2, delay, rise, fall, width, period, pulse_count, width_defaults_to_zero } if v1.to_bits() == 5.0f64.to_bits() && v2.to_bits() == 0.0f64.to_bits() && delay.to_bits() == 1.5e-6f64.to_bits() && rise.to_bits() == 5.0e-9f64.to_bits() && fall.to_bits() == 5.0e-9f64.to_bits() && width.to_bits() == 1.5e-6f64.to_bits() && period.to_bits() == 3.0e-6f64.to_bits() && pulse_count.to_bits() == 0.0f64.to_bits() && !width_defaults_to_zero))
        {
            return Err(format!("{LABEL} pulse source changed: {:?}", pulse.kind));
        }
        for (key, nodes, value) in [
            ("rin", ["IN", "1"], 1_000.0_f64),
            ("r1", ["VOUT", "0"], 10_000.0_f64),
        ] {
            let element = elements[key];
            if element.provenance != ElementProvenance::Authored
                || !Self::mosfet_param_alias_nodes_match(&element.nodes, &nodes)
                || !matches!(&element.kind, ElementKind::Resistor { value: actual, value_expr: None, model: None, instance_params, deferred_params } if actual.to_bits() == value.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            {
                return Err(format!("{LABEL} resistor {key} changed"));
            }
        }
        let capacitor = elements["c2"];
        if capacitor.provenance != ElementProvenance::Authored
            || !Self::mosfet_param_alias_nodes_match(&capacitor.nodes, &["VOUT", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params } if value.to_bits() == 0.1e-12f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} output capacitor changed"));
        }
        for (key, nodes, model, l, w) in [
            (
                "mn1",
                ["VOUT", "IN", "0", "0"],
                "CD4012_NMOS",
                5.0e-6,
                175.0e-6,
            ),
            (
                "mp1",
                ["VOUT", "IN", "VDD", "VDD"],
                "CD4012_PMOS",
                5.0e-6,
                270.0e-6,
            ),
        ] {
            let element = elements[key];
            let ElementKind::Mosfet {
                model: actual_model,
                compact_syntax: false,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                return Err(format!("{LABEL} MOSFET {key} changed"));
            };
            if element.provenance != ElementProvenance::Authored
                || !Self::mosfet_param_alias_nodes_match(&element.nodes, &nodes)
                || !actual_model.eq_ignore_ascii_case(model)
                || !Self::mosfet_param_alias_exact_params(instance_params, &[("L", l), ("W", w)])
                || !deferred_params.is_empty()
            {
                return Err(format!(
                    "{LABEL} MOSFET {key} topology changed: {:?}",
                    element.kind
                ));
            }
        }
        for model in &netlist.models {
            if Self::numeric_param_value(&model.params, "LEVEL")
                .is_none_or(|level| level.to_bits() != (role.level() as f64).to_bits())
            {
                return Err(format!("{LABEL} {} model level changed", model.name));
            }
        }
        let tolerances = Self::xyce_verify_comp_tolerances(
            source,
            &plan.print.as_ref().expect("PRINT checked").probes,
        )?;
        let defaults = XyceVerifyTransientTolerance::release_7_10_default();
        let expected = if role.level() == 3 {
            vec![
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 1e-6,
                    ..defaults
                },
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 1e-6,
                    zero: 1e-6,
                    absolute_difference: 1e-6,
                    ..defaults
                },
                defaults,
            ]
        } else {
            vec![
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 5e-7,
                    ..defaults
                },
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 5e-7,
                    zero: 1e-8,
                    ..defaults
                },
                defaults,
            ]
        };
        if tolerances != expected {
            return Err(format!(
                "{LABEL} {} COMP policy changed: {tolerances:?}",
                role.file_name()
            ));
        }
        let models = Self::normalized_mosfet_param_alias_models(&netlist, role)?;
        Ok((plan, netlist, models))
    }

    fn validate_mosfet_param_alias_table(table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != 5
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(VOUT)")
            || !table.columns[3].eq_ignore_ascii_case("V(IN)")
            || !table.columns[4].eq_ignore_ascii_case("V(1)")
            || table.rows.len() < 10
            || table
                .rows
                .iter()
                .any(|row| row.len() != 5 || row.iter().any(|value| !value.is_finite()))
        {
            return Err(format!("{LABEL} produced malformed transient output"));
        }
        for (index, row) in table.rows.iter().enumerate() {
            if row[0].to_bits() != (index as f64).to_bits() {
                return Err(format!("{LABEL} output index sequence changed"));
            }
        }
        for rows in table.rows.windows(2) {
            if rows[1][1] <= rows[0][1] {
                return Err(format!("{LABEL} output index/time grid changed"));
            }
        }
        if table.rows.first().is_none_or(|row| row[1].abs() > 1e-18)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 30.0e-6).abs() > 1e-12)
        {
            return Err(format!("{LABEL} output domain changed"));
        }
        for column in 2..=4 {
            let min = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::INFINITY, Value::min);
            let max = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::NEG_INFINITY, Value::max);
            if max - min < if column == 2 { 0.05 } else { 4.0 } {
                return Err(format!(
                    "{LABEL} output column {} became vacuous",
                    table.columns[column]
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_mosfet_param_alias_oracle(
        &self,
        role: MosfetParamAliasRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let owner = role.owner();
        let alt = role.alt();
        let build = |member: MosfetParamAliasRole| -> Result<(XyceStaticTranPlan, Netlist, NormalizedModels), String> {
            let path = self.root.join(member.path());
            let source = fs::read_to_string(&path).map_err(|error| format!("{LABEL} cannot read {}: {error}", member.file_name()))?;
            self.validate_mosfet_param_alias_typed_plan(member, &source, &path)
        };
        let (owner_plan, _owner_netlist, owner_models) = build(owner)?;
        let (alt_plan, _alt_netlist, alt_models) = build(alt)?;
        if owner_models != alt_models {
            return Err(format!(
                "{LABEL} LEVEL={} normalized model snapshots differ",
                role.level()
            ));
        }
        let run = |member: MosfetParamAliasRole,
                   plan: &XyceStaticTranPlan|
         -> Result<XycePrnTable, String> {
            let (netlist, result) = self
                .run_transient_family_plan(plan, start, None, None)
                .map_err(|error| format!("{LABEL} {} failed: {error}", member.file_name()))?;
            let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)?;
            Self::validate_mosfet_param_alias_table(&table)?;
            Ok(table)
        };
        let owner_table = run(owner, &owner_plan)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired between pair runs"));
        }
        let alt_table = run(alt, &alt_plan)?;
        let tolerances = Self::xyce_verify_comp_tolerances(
            &owner_plan.source,
            &owner_plan.print.as_ref().expect("PRINT checked").probes,
        )?;
        let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            &owner_table,
            &alt_table,
            &tolerances,
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} LEVEL={} owner-good to alt-test comparison failed: {mismatches:?}",
                role.level()
            ));
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
    fn mosfet_param_alias_typed_snapshots_match_only_after_alias_normalization() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for owner in MosfetParamAliasRole::OWNERS {
            let owner_source = fs::read_to_string(root.join(owner.path())).expect("read owner");
            let owner_parsed =
                XyceTestRunner::parse_xyce_netlist(&owner_source, &root.join(owner.path()))
                    .expect("parse owner");
            assert!(
                XyceTestRunner::netlist_is_native_classic_mos_parameter_alias_envelope(
                    &owner_parsed,
                ),
                "LEVEL={} owner is outside native envelope: models={:#?}, elements={:#?}",
                owner.level(),
                owner_parsed.models,
                owner_parsed.elements
            );
            let alt = owner.alt();
            let alt_source = fs::read_to_string(root.join(alt.path())).expect("read alt");
            let (_, _, owner_models) = runner
                .validate_mosfet_param_alias_typed_plan(
                    owner,
                    &owner_source,
                    &root.join(owner.path()),
                )
                .expect("owner plan");
            let (_, _, alt_models) = runner
                .validate_mosfet_param_alias_typed_plan(alt, &alt_source, &root.join(alt.path()))
                .expect("alt plan");
            assert_eq!(owner_models, alt_models);
        }
    }

    #[test]
    fn mosfet_param_alias_mixed_duplicate_and_vth0_cards_fail_closed() {
        let root = corpus_root();
        let role = MosfetParamAliasRole::Level1Owner;
        let source = fs::read_to_string(root.join(role.path())).expect("read owner");
        for (mutated, duplicate_parameter) in [
            (source.replacen("UO = 310", "U0 = 310 UO = 310", 1), None),
            (
                source.replacen("UO = 310", "UO = 310 UO = 310", 1),
                Some("UO"),
            ),
            (source.replacen("VTO = -1.6", "VTH0 = -1.6", 1), None),
        ] {
            match duplicate_parameter {
                Some(expected) => {
                    let error =
                        XyceTestRunner::parse_xyce_netlist(&mutated, &root.join(role.path()))
                            .expect_err("duplicate Xyce model parameter must fail during parsing");
                    let ParseError::DuplicateModelParameter(error) = error else {
                        panic!("unexpected duplicate-model failure: {error:?}");
                    };
                    assert_eq!(error.canonical_model_name, "CD4012_PMOS");
                    assert_eq!(error.canonical_parameter_name, expected);
                    assert_eq!(error.model_origin.line, 20);
                }
                None => {
                    let netlist =
                        XyceTestRunner::parse_xyce_netlist(&mutated, &root.join(role.path()))
                            .expect("non-duplicate alias mutation remains syntactically valid");
                    assert!(
                        XyceTestRunner::normalized_mosfet_param_alias_models(&netlist, role)
                            .is_err(),
                        "mixed and non-Release aliases must remain outside this contract"
                    );
                }
            }
        }
    }

    fn valid_alias_table() -> XycePrnTable {
        let stop = 30.0e-6;
        XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "TIME".to_string(),
                "V(VOUT)".to_string(),
                "V(IN)".to_string(),
                "V(1)".to_string(),
            ],
            rows: (0..10)
                .map(|index| {
                    let fraction = index as f64 / 9.0;
                    vec![
                        index as f64,
                        stop * fraction,
                        5.0 * fraction,
                        5.0 * (1.0 - fraction),
                        5.0 * (1.0 - fraction),
                    ]
                })
                .collect(),
        }
    }

    #[test]
    fn mosfet_param_alias_table_shape_and_relation_fail_closed() {
        let table = valid_alias_table();
        XyceTestRunner::validate_mosfet_param_alias_table(&table)
            .expect("non-vacuous canonical table");

        let mut malformed = table.clone();
        malformed.rows[4][2] = Value::NAN;
        assert!(XyceTestRunner::validate_mosfet_param_alias_table(&malformed).is_err());

        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let tolerances = vec![XyceVerifyTransientTolerance::release_7_10_default(); 3];
        let mut changed = table.clone();
        changed.rows[5][2] += 1.0;
        let mismatches = runner
            .compare_xyce_verify_transient_tables_with_probe_tolerances(
                &table,
                &changed,
                &tolerances,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )
            .expect("counterfactual comparison remains well formed");
        assert!(!mismatches.is_empty());
    }

    #[test]
    fn mosfet_param_alias_all_four_native_pairs_execute() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for owner in MosfetParamAliasRole::OWNERS {
            let alt = owner.alt();
            let owner_source = fs::read_to_string(root.join(owner.path())).expect("read owner");
            let alt_source = fs::read_to_string(root.join(alt.path())).expect("read alt");
            let (owner_plan, _, owner_models) = runner
                .validate_mosfet_param_alias_typed_plan(
                    owner,
                    &owner_source,
                    &root.join(owner.path()),
                )
                .expect("owner plan");
            let (alt_plan, _, alt_models) = runner
                .validate_mosfet_param_alias_typed_plan(alt, &alt_source, &root.join(alt.path()))
                .expect("alt plan");
            assert_eq!(owner_models, alt_models);

            let start = Instant::now();
            let (owner_netlist, owner_result) = runner
                .run_transient_family_plan(&owner_plan, start, None, None)
                .expect("owner native run");
            let (alt_netlist, alt_result) = runner
                .run_transient_family_plan(&alt_plan, start, None, None)
                .expect("alt native run");
            let owner_table = XyceTestRunner::transient_family_result_to_prn_table(
                &owner_plan,
                &owner_netlist,
                &owner_result,
            )
            .expect("owner table");
            let alt_table = XyceTestRunner::transient_family_result_to_prn_table(
                &alt_plan,
                &alt_netlist,
                &alt_result,
            )
            .expect("alt table");
            XyceTestRunner::validate_mosfet_param_alias_table(&owner_table)
                .expect("owner table shape");
            XyceTestRunner::validate_mosfet_param_alias_table(&alt_table).expect("alt table shape");
            let tolerances = XyceTestRunner::xyce_verify_comp_tolerances(
                &owner_plan.source,
                &owner_plan.print.as_ref().expect("PRINT checked").probes,
            )
            .expect("COMP policy");
            let mismatches = runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &owner_table,
                    &alt_table,
                    &tolerances,
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .expect("pair comparison");
            assert!(
                mismatches.is_empty(),
                "LEVEL={} alias pair diverged: {mismatches:?}",
                owner.level()
            );
        }
    }

    #[test]
    fn mosfet_param_alias_expired_shared_deadline_rejects_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_mosfet_param_alias_oracle(
                    MosfetParamAliasRole::Level1Owner,
                    Instant::now() - Duration::from_millis(10),
                )
                .is_err()
        );
    }
}
