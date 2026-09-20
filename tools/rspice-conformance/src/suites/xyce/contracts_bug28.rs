use super::*;
use rspice_core::netlist::SourceSpec;

const BUG28_LABEL: &str = "BUG_28_SON subcircuit-parameter equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug28Role {
    LocalOwner,
    LiteralControl,
    GlobalControl,
}

impl Bug28Role {
    const ALL: [Self; 3] = [Self::LocalOwner, Self::LiteralControl, Self::GlobalControl];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::LocalOwner => XYCE_BUG28SON_OWNER_PATH,
            Self::LiteralControl => XYCE_BUG28SON_LITERAL_PATH,
            Self::GlobalControl => XYCE_BUG28SON_GLOBAL_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::LocalOwner => XYCE_BUG28SON_OWNER_RECORD,
            Self::LiteralControl => XYCE_BUG28SON_LITERAL_RECORD,
            Self::GlobalControl => XYCE_BUG28SON_GLOBAL_RECORD,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::LocalOwner => XYCE_BUG28SON_OWNER_CONTRACT,
            Self::LiteralControl => XYCE_BUG28SON_LITERAL_CONTRACT,
            Self::GlobalControl => XYCE_BUG28SON_GLOBAL_CONTRACT,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("BUG28 path has a file name")
    }

    fn label(self) -> &'static str {
        match self {
            Self::LocalOwner => "local-parameter owner",
            Self::LiteralControl => "literal control",
            Self::GlobalControl => "global-parameter control",
        }
    }

    fn plan_purpose(self) -> XyceStaticTranPlanPurpose {
        match self {
            Self::LocalOwner => XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            Self::LiteralControl | Self::GlobalControl => {
                XyceStaticTranPlanPurpose::RelationalFamily
            }
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug28_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG28SON_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG28SON_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG28SON_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug28_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug28_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG28SON_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG28SON_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG28SON_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG28SON_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG28_LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug28_directory(
        directory: &Path,
        expected_artifacts: &[(&str, usize, &str, &str)],
        label: &str,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {label} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{label} directory {} must be a regular non-symlink directory",
                directory.display()
            ));
        }
        let expected = expected_artifacts
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {label} directory: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {label} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{label} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{label} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!(
                    "{label} directory contains case-colliding member {name:?}"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{label} directory acquired unexpected member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{label} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {label} member {name:?}: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(label, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{label} member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    canonical.len()
                ));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{label} directory census changed: expected {} members, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug28_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug28Role,
    ) -> Result<BTreeMap<Bug28Role, Vec<u8>>, String> {
        Self::validate_bug28_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {BUG28_LABEL} {} is not backed by its canonical path",
                role.label()
            ));
        }

        let family_prefix = "netlists/certification_tests/bug_28_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let expected_owners = BTreeSet::from([
            XYCE_BUG28SON_OWNER_RECORD,
            "netlists/certification_tests/bug_28_son/bug_28_son4.cir",
            "netlists/certification_tests/bug_28_son/bug_28_son5.cir",
        ]);
        if owners != expected_owners {
            return Err(format!(
                "{BUG28_LABEL} wrapper ownership changed: {owners:?}"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG28_LABEL} exclusion manifest is invalid: {error}"))?;
        for owner in &expected_owners {
            if exclusions.contains_key(*owner) {
                return Err(format!(
                    "{BUG28_LABEL} wrapper owner {owner} must not be excluded"
                ));
            }
        }
        let expected_exclusions = BTreeSet::from([
            XYCE_BUG28SON_GLOBAL_RECORD,
            XYCE_BUG28SON_LITERAL_RECORD,
            "netlists/certification_tests/bug_28_son/bug_28_son4_1.cir",
            "netlists/certification_tests/bug_28_son/bug_28_son4_2.cir",
            "netlists/certification_tests/bug_28_son/bug_28_son4_3.cir",
            "netlists/certification_tests/bug_28_son/bug_28_son5_1.cir",
            "netlists/certification_tests/bug_28_son/bug_28_son5_2.cir",
            "netlists/certification_tests/bug_28_son/bug_28_son5_3.cir",
        ]);
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(family_prefix))
            .collect::<BTreeMap<_, _>>();
        if family_exclusions
            .keys()
            .map(|record| record.as_str())
            .collect::<BTreeSet<_>>()
            != expected_exclusions
        {
            return Err(format!(
                "{BUG28_LABEL} exclusion-family census changed: {:?}",
                family_exclusions.keys()
            ));
        }
        for (record, exclusion) in family_exclusions {
            let expected_contract = match record.as_str() {
                XYCE_BUG28SON_LITERAL_RECORD => Some(XYCE_BUG28SON_LITERAL_CONTRACT),
                XYCE_BUG28SON_GLOBAL_RECORD => Some(XYCE_BUG28SON_GLOBAL_CONTRACT),
                _ => None,
            };
            let correct = exclusion.source == XYCE_BUG28SON_EXCLUSION_SOURCE
                && match expected_contract {
                    Some(contract) => matches!(&exclusion.disposition,
                        XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                            if expected_contract == contract),
                    None => matches!(
                        &exclusion.disposition,
                        XyceUpstreamExclusionDisposition::Excluded
                    ),
                };
            if !correct {
                return Err(format!(
                    "{BUG28_LABEL} qualification changed for {record}: {exclusion:?}"
                ));
            }
        }

        let family = self.root.join("Netlists/Certification_Tests/BUG_28_SON");
        let source_members = Self::validate_bug28_directory(
            &family,
            &XYCE_BUG28SON_RETAINED_ARTIFACTS,
            "BUG_28_SON retained source",
        )?;
        let output = self.root.join("OutputData/Certification_Tests/BUG_28_SON");
        Self::validate_bug28_directory(
            &output,
            &XYCE_BUG28SON_RETAINED_OUTPUTS,
            "BUG_28_SON retained output",
        )?;

        let mut sources = BTreeMap::new();
        for member_role in Bug28Role::ALL {
            self.reject_wrapper_output_artifacts(&family.join(member_role.file_name()))
                .map_err(|error| format!("{BUG28_LABEL} {} {error}", member_role.label()))?;
            sources.insert(
                member_role,
                source_members
                    .get(&member_role.file_name().to_ascii_lowercase())
                    .cloned()
                    .ok_or_else(|| format!("{BUG28_LABEL} lost {}", member_role.file_name()))?,
            );
        }
        Ok(sources)
    }

    fn exact_passive(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        expected: Value,
        kind: char,
    ) -> bool {
        if !element.name.eq_ignore_ascii_case(name)
            || !Self::nodes_match(&element.nodes, &nodes)
            || element.provenance != ElementProvenance::Authored
        {
            return false;
        }
        match (&element.kind, kind) {
            (
                ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                },
                'R',
            ) => {
                value.to_bits() == expected.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
            }
            (
                ElementKind::Capacitor {
                    value,
                    value_expr: None,
                    initial_voltage: None,
                    model: None,
                    instance_params,
                    deferred_params,
                },
                'C',
            ) => {
                value.to_bits() == expected.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
            }
            (
                ElementKind::Inductor {
                    value,
                    value_expr: None,
                    initial_current: None,
                    model: None,
                    instance_params,
                    deferred_params,
                },
                'L',
            ) => {
                value.to_bits() == expected.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
            }
            _ => false,
        }
    }

    fn nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug28_worker(
        &self,
        role: Bug28Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(path, role.plan_purpose())?;
        let expected_contract = if role == Bug28Role::LocalOwner {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.contract != expected_contract
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan
                .print
                .as_ref()
                .is_none_or(|print| print.probes != ["v(node1)", "v(node2)"])
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 1.0e-4f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{BUG28_LABEL} {} transient plan changed: {plan:?}",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{BUG28_LABEL} {} no longer parses: {error}", role.label()))?;
        if netlist.elements.len() != 4
            || netlist.subcircuits.len() != 1
            || !netlist.models.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{BUG28_LABEL} {} top-level envelope changed",
                role.label()
            ));
        }
        let global_values = role == Bug28Role::GlobalControl
            && !netlist.params.has_parameter_binding("PL1")
            && !netlist.params.has_parameter_binding("PL2")
            && netlist
                .params
                .get("PL1")
                .is_some_and(|value| value.to_bits() == 6.67e-6f64.to_bits())
            && netlist
                .params
                .get("PL2")
                .is_some_and(|value| value.to_bits() == 4.65e-6f64.to_bits())
            && netlist.params.numeric_parameters().len() == 2;
        if (role == Bug28Role::GlobalControl) != global_values
            || (role != Bug28Role::GlobalControl && !netlist.params.numeric_parameters().is_empty())
        {
            return Err(format!(
                "{BUG28_LABEL} {} global parameter namespace changed",
                role.label()
            ));
        }

        let instance = &netlist.elements[0];
        if !instance.name.eq_ignore_ascii_case("XTEST")
            || !Self::nodes_match(&instance.nodes, &["node1", "node2"])
            || instance.provenance != ElementProvenance::Authored
            || !matches!(&instance.kind, ElementKind::Subcircuit { subckt_name, params }
                if subckt_name.eq_ignore_ascii_case("TRANSFORMER_V2") && params.is_empty())
            || !Self::exact_passive(&netlist.elements[1], "R1", ["node1", "0"], 100.0, 'R')
            || !Self::exact_passive(&netlist.elements[2], "R2", ["node2", "0"], 100.0, 'R')
            || !matches!(
                &netlist.elements[3].kind,
                ElementKind::VoltageSource(SourceSpec::Sin { .. })
            )
        {
            return Err(format!(
                "{BUG28_LABEL} {} top-level topology changed: {:?}",
                role.label(),
                netlist.elements
            ));
        }
        match &netlist.elements[3].kind {
            ElementKind::VoltageSource(SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            }) if netlist.elements[3].name.eq_ignore_ascii_case("VSRC")
                && Self::nodes_match(&netlist.elements[3].nodes, &["node1", "0"])
                && offset.to_bits() == 0.0f64.to_bits()
                && amplitude.to_bits() == 10.0f64.to_bits()
                && frequency.to_bits() == 100_000.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits() => {}
            _ => return Err(format!("{BUG28_LABEL} {} source changed", role.label())),
        }

        let subckt = &netlist.subcircuits[0];
        let expected_body: [(&str, Value); 9] = [
            ("PC1", 2.4e-9),
            ("PC2", 1.64e-15),
            ("PCO", 28.9e-9),
            ("PRC1", 0.26e-6),
            ("PRC2", 7.6),
            ("PRHI", 0.34e-6),
            ("PRL1", 47.6e-9),
            ("PRL2", 0.2),
            ("PRLO", 0.233e-6),
        ];
        let mut body = subckt.body_params.clone();
        body.sort_by(|left, right| left.0.cmp(&right.0));
        let mut expected = expected_body
            .iter()
            .map(|(name, value)| ((*name).to_string(), *value))
            .collect::<Vec<_>>();
        if role != Bug28Role::GlobalControl {
            expected.extend([("PL1".to_string(), 6.67e-6), ("PL2".to_string(), 4.65e-6)]);
        }
        expected.sort_by(|left, right| left.0.cmp(&right.0));
        if !subckt.name.eq_ignore_ascii_case("TRANSFORMER_V2")
            || !Self::nodes_match(&subckt.ports, &["trans_start", "trans_end"])
            || subckt.elements.len() != 14
            || !subckt.params.is_empty()
            || !subckt.expr_params.is_empty()
            || !subckt.body_expr_params.is_empty()
            || body.len() != expected.len()
            || !body.iter().zip(&expected).all(
                |((name, value), (expected_name, expected_value))| {
                    name.eq_ignore_ascii_case(expected_name)
                        && value.to_bits() == expected_value.to_bits()
                },
            )
            || !subckt.body_string_params.is_empty()
            || !subckt.body_functions.is_empty()
            || !subckt.initial_conditions.is_empty()
            || !subckt.node_sets.is_empty()
            || !subckt.local_options.is_empty()
            || subckt.library_ref.is_some()
            || !subckt.nested_subcircuits.is_empty()
        {
            return Err(format!(
                "{BUG28_LABEL} {} subcircuit parameter scope changed",
                role.label()
            ));
        }

        let l1 = &subckt.elements[5];
        let l2 = &subckt.elements[8];
        let expected_l1_expression = (role != Bug28Role::LiteralControl).then_some("pL1");
        let expected_l2_expression = (role != Bug28Role::LiteralControl).then_some("pL2");
        let exact_authored_l = |element: &rspice_core::netlist::Element,
                                name: &str,
                                nodes: [&str; 2],
                                value: Value,
                                expression: Option<&str>| {
            element.name.eq_ignore_ascii_case(name)
                && Self::nodes_match(&element.nodes, &nodes)
                && element.provenance == ElementProvenance::Authored
                && matches!(&element.kind, ElementKind::Inductor {
                    value: actual,
                    value_expr,
                    initial_current: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if expression.map_or_else(
                        || actual.to_bits() == value.to_bits(),
                        |_| actual.is_nan(),
                    )
                    && match (value_expr.as_deref(), expression) {
                        (Some(actual), Some(expected)) => actual.eq_ignore_ascii_case(expected),
                        (None, None) => true,
                        _ => false,
                    }
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
        };
        if !exact_authored_l(
            l1,
            "L1",
            ["NODE1", "NODE4"],
            6.67e-6,
            expected_l1_expression,
        ) || !exact_authored_l(
            l2,
            "L2",
            ["NODE2", "NODE5"],
            4.65e-6,
            expected_l2_expression,
        ) || !matches!(&subckt.elements[9].kind, ElementKind::Coupling { inductors, coefficient, model: None }
                if subckt.elements[9].name.eq_ignore_ascii_case("KTRANS")
                    && inductors.len() == 2
                    && inductors.iter().zip(["L1", "L2"]).all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
                    && coefficient.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{BUG28_LABEL} {} authored inductance causality changed",
                role.label()
            ));
        }

        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .zip(["node1", "node2"])
                .any(|(dependency, node)| {
                    dependency.kind != OutputSymbolKind::Node
                        || !dependency.operator.eq_ignore_ascii_case("V")
                        || !dependency.symbol.eq_ignore_ascii_case(node)
                })
        {
            return Err(format!(
                "{BUG28_LABEL} {} typed .PRINT request changed",
                role.label()
            ));
        }

        let flattened = flatten_netlist_with_models(&netlist)
            .map_err(|error| format!("{BUG28_LABEL} {} flatten failed: {error}", role.label()))?;
        let flattened_l = |name: &str, nodes: [&str; 2], value: Value| {
            flattened.elements.iter().any(|element| {
                element.name.eq_ignore_ascii_case(name)
                    && Self::nodes_match(&element.nodes, &nodes)
                    && matches!(&element.kind, ElementKind::Inductor {
                        value: actual,
                        value_expr: None,
                        initial_current: None,
                        model: None,
                        instance_params,
                        deferred_params,
                    } if actual.to_bits() == value.to_bits()
                        && instance_params.is_empty()
                        && deferred_params.is_empty())
            })
        };
        if flattened.elements.len() != 17
            || !flattened.scoped_models.is_empty()
            || !flattened_l("XTEST.L1", ["XTEST.NODE1", "XTEST.NODE4"], 6.67e-6)
            || !flattened_l("XTEST.L2", ["XTEST.NODE2", "XTEST.NODE5"], 4.65e-6)
            || !flattened.elements.iter().any(|element|
                element.name.eq_ignore_ascii_case("XTEST.KTRANS")
                    && matches!(&element.kind, ElementKind::Coupling { inductors, coefficient, model: None }
                        if inductors.len() == 2
                            && inductors.iter().zip(["XTEST.L1", "XTEST.L2"]).all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
                            && coefficient.to_bits() == 1.0f64.to_bits()))
        {
            return Err(format!(
                "{BUG28_LABEL} {} flattened coupled-inductor circuit changed: {:?}",
                role.label(), flattened.elements
            ));
        }
        Ok(plan)
    }

    fn run_bug28_worker(
        &self,
        role: Bug28Role,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|error| format!("{BUG28_LABEL} {} failed: {error}", role.label()))?;
        let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|error| format!("{BUG28_LABEL} {} PRN failed: {error}", role.label()))?;
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(node1)")
            || !table.columns[3].eq_ignore_ascii_case("V(node2)")
            || table.rows.len() < 2
            || table
                .rows
                .iter()
                .any(|row| row.len() != 4 || row.iter().any(|value| !value.is_finite()))
            || table.rows.first().is_none_or(|row| row[1].abs() > 1.0e-15)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 1.0e-4).abs() > 1.0e-12)
            || table
                .rows
                .iter()
                .map(|row| row[2].abs())
                .fold(0.0, Value::max)
                < 1.0
            || table
                .rows
                .iter()
                .map(|row| row[3].abs())
                .fold(0.0, Value::max)
                < 1.0e-3
        {
            return Err(format!(
                "{BUG28_LABEL} {} produced an invalid or trivial transient table",
                role.label()
            ));
        }
        Ok(table)
    }

    fn compare_bug28_relation(
        &self,
        good_label: &str,
        good: &XycePrnTable,
        test_label: &str,
        test: &XycePrnTable,
    ) -> Result<(), String> {
        if self
            .compare_serialized_default_prn_tables(good, test)
            .is_ok_and(|mismatches| mismatches.is_empty())
        {
            return Ok(());
        }
        let mismatches = self
            .compare_xyce_verify_transient_tables(good, test)
            .map_err(|error| {
                format!(
                    "{BUG28_LABEL} {good_label}/{test_label} xyce_verify fallback failed: {error}"
                )
            })?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{BUG28_LABEL} {good_label} GOOD versus {test_label} TEST differs: {mismatches:?}"
            ))
        }
    }

    pub(super) fn validate_bug28_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug28Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG28_LABEL} deadline expired before validation"));
        }
        let sources = self.validate_bug28_provenance(deck, role)?;
        let mut outputs = BTreeMap::new();
        for worker_role in Bug28Role::ALL {
            let source = std::str::from_utf8(
                sources
                    .get(&worker_role)
                    .ok_or_else(|| format!("{BUG28_LABEL} lost {}", worker_role.label()))?,
            )
            .map_err(|error| {
                format!(
                    "{BUG28_LABEL} {} is not UTF-8: {error}",
                    worker_role.label()
                )
            })?;
            let path = self.root.join(worker_role.path());
            let plan = self.validate_bug28_worker(worker_role, source, &path)?;
            outputs.insert(
                worker_role,
                self.run_bug28_worker(worker_role, &plan, start)?,
            );
        }
        let owner = outputs
            .get(&Bug28Role::LocalOwner)
            .expect("all BUG28 workers ran");
        let literal = outputs
            .get(&Bug28Role::LiteralControl)
            .expect("all BUG28 workers ran");
        let global = outputs
            .get(&Bug28Role::GlobalControl)
            .expect("all BUG28 workers ran");
        self.compare_bug28_relation("local owner", owner, "literal control", literal)?;
        self.compare_bug28_relation("global control", global, "literal control", literal)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG28_LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        self.validate_bug28_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG28_LABEL} final provenance exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
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

    fn bug28_exclusion_manifest(literal_disposition: &str) -> String {
        let rows = [
            format!(
                "{XYCE_BUG28SON_GLOBAL_PATH}\t{XYCE_BUG28SON_EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{XYCE_BUG28SON_GLOBAL_CONTRACT}"
            ),
            format!(
                "{XYCE_BUG28SON_LITERAL_PATH}\t{XYCE_BUG28SON_EXCLUSION_SOURCE}\t{literal_disposition}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_28_SON/bug_28_son4_1.cir\t{XYCE_BUG28SON_EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_28_SON/bug_28_son4_2.cir\t{XYCE_BUG28SON_EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_28_SON/bug_28_son4_3.cir\t{XYCE_BUG28SON_EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_28_SON/bug_28_son5_1.cir\t{XYCE_BUG28SON_EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_28_SON/bug_28_son5_2.cir\t{XYCE_BUG28SON_EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_28_SON/bug_28_son5_3.cir\t{XYCE_BUG28SON_EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
        ];
        format!(
            "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\n",
            rows.join("\n")
        )
    }

    fn bug28_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug28-{label}-"))
            .tempdir()
            .expect("create BUG28 fixture root");
        let root = temporary.path().to_path_buf();
        let source_family = root.join("Netlists/Certification_Tests/BUG_28_SON");
        let output_family = root.join("OutputData/Certification_Tests/BUG_28_SON");
        fs::create_dir_all(&source_family).expect("create BUG28 source family");
        fs::create_dir_all(&output_family).expect("create BUG28 output family");
        let canonical = corpus_root();
        for (name, ..) in XYCE_BUG28SON_RETAINED_ARTIFACTS {
            fs::copy(
                canonical
                    .join("Netlists/Certification_Tests/BUG_28_SON")
                    .join(name),
                source_family.join(name),
            )
            .expect("copy canonical BUG28 source member");
        }
        for (name, ..) in XYCE_BUG28SON_RETAINED_OUTPUTS {
            fs::copy(
                canonical
                    .join("OutputData/Certification_Tests/BUG_28_SON")
                    .join(name),
                output_family.join(name),
            )
            .expect("copy canonical BUG28 output member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{XYCE_BUG28SON_OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\nNetlists/Certification_Tests/BUG_28_SON/bug_28_son4.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\nNetlists/Certification_Tests/BUG_28_SON/bug_28_son5.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"
            ),
        )
        .expect("write BUG28 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            bug28_exclusion_manifest(&format!(
                "{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{XYCE_BUG28SON_LITERAL_CONTRACT}"
            )),
        )
        .expect("write BUG28 exclusion manifest");
        let deck = XyceDeck {
            path: root.join(XYCE_BUG28SON_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG28SON_OWNER_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug28_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug28_historical_oracle_provenance()
            .expect("Release-7.10 BUG28 son3 provenance remains exact");
    }

    #[test]
    fn bug28_workers_preserve_local_literal_and_global_parameter_causality() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug28Role::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG28 worker");
            runner
                .validate_bug28_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
            let mutation = source.replacen("6.67e-6", "6.68e-6", 1);
            assert!(
                runner
                    .validate_bug28_worker(role, &mutation, &path)
                    .is_err()
            );
        }
    }

    #[test]
    fn bug28_oracle_runs_all_three_exact_roles() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug28Role::ALL {
            let deck = XyceDeck {
                path: root.join(role.path()),
                section: XyceDeckSection::Netlists,
                relative_path: role.path().to_string(),
            };
            runner
                .validate_bug28_oracle(&deck, role, Instant::now())
                .expect("canonical BUG28 relational oracle passes");
        }
    }

    #[test]
    fn bug28_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            path: root.join(XYCE_BUG28SON_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG28SON_OWNER_PATH.to_string(),
        };
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("expired start");
        assert!(
            runner
                .validate_bug28_oracle(&deck, Bug28Role::LocalOwner, start)
                .is_err()
        );
    }

    #[test]
    fn bug28_provenance_rejects_source_output_ownership_and_qualification_drift() {
        let (_temporary, deck, runner) = bug28_fixture("source-drift");
        runner
            .validate_bug28_provenance(&deck, Bug28Role::LocalOwner)
            .expect("canonical BUG28 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG28 deck has parent")
                .join("unexpected.out"),
            "stale wrapper output\n",
        )
        .expect("write stale BUG28 artifact");
        assert!(
            runner
                .validate_bug28_provenance(&deck, Bug28Role::LocalOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug28_fixture("output-drift");
        fs::write(
            runner
                .root
                .join("OutputData/Certification_Tests/BUG_28_SON/bug_28_son3.cir.prn"),
            "invented numerical gold\n",
        )
        .expect("write forbidden BUG28 son3 gold");
        assert!(
            runner
                .validate_bug28_provenance(&deck, Bug28Role::LocalOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug28_fixture("owner-drift");
        fs::write(
            runner.root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{XYCE_BUG28SON_OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\nNetlists/Certification_Tests/BUG_28_SON/bug_28_son4.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"
            ),
        )
        .expect("remove BUG28 son5 owner");
        let refreshed = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(
            refreshed
                .validate_bug28_provenance(&deck, Bug28Role::LocalOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug28_fixture("qualification-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            bug28_exclusion_manifest(UPSTREAM_EXCLUDED_DISPOSITION),
        )
        .expect("demote BUG28 literal control");
        let refreshed = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(
            refreshed
                .validate_bug28_provenance(&deck, Bug28Role::LocalOwner)
                .is_err()
        );
    }
}
