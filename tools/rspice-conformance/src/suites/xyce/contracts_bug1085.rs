use super::*;

const LABEL: &str = "BUG_1085_SON user-function I0 relational family";

pub(super) const XYCE_BUG1085_WRAPPER_OWNER_CONTRACT: &str =
    "bug1085_user_function_i0_relational_wrapper_owner";
pub(super) const XYCE_BUG1085_REFERENCE_BASELINE_CONTRACT: &str =
    "bug1085_user_function_alpha_renamed_reference_baseline";
pub(super) const XYCE_BUG1085_FAMILY: &str = "Certification_Tests/BUG_1085_SON/bug1085son";
pub(super) const XYCE_BUG1085_FAMILY_DIR: &str = "Netlists/Certification_Tests/BUG_1085_SON";
pub(super) const XYCE_BUG1085_OWNER_PATH: &str =
    "Netlists/Certification_Tests/BUG_1085_SON/bug1085son.cir";
pub(super) const XYCE_BUG1085_CONTROL_PATH: &str =
    "Netlists/Certification_Tests/BUG_1085_SON/bug1085sonRef.cir";
pub(super) const XYCE_BUG1085_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_1085_son/bug1085son.cir";
pub(super) const XYCE_BUG1085_CONTROL_RECORD: &str =
    "netlists/certification_tests/bug_1085_son/bug1085sonref.cir";
const XYCE_BUG1085_HISTORICAL_EXCLUDE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1085_SON/exclude";
const XYCE_BUG1085_PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const XYCE_BUG1085_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1085_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1085_HISTORICAL_RECORD_COUNT: usize = 5;
const XYCE_BUG1085_HISTORICAL_RECORD_BYTES: usize = 1_188;
const XYCE_BUG1085_HISTORICAL_RECORDS_SHA256: &str =
    "bbf195b19e8ccf2068f69917303ff1ebcf0f8164798b40203a189828580415cb";
const XYCE_BUG1085_HISTORICAL_RECORDS_BLAKE3: &str =
    "b245ba1d09aee2c1311ca9b2db3923011d3fcbb9ce4de474fb2c8db3a49f0ae1";

const XYCE_BUG1085_RETAINED_SOURCES: [(&str, usize, &str, &str); 2] = [
    (
        "bug1085son.cir",
        787,
        "4e254194ffb25b86e2e95ab344f0504290f51707b2505104d655f100bf39e414",
        "58066213cd1cabf5377c677d9482f88e6c2974da23bef87b0415d1cace7ca126",
    ),
    (
        "bug1085sonRef.cir",
        801,
        "60502e13e53e47dcaa4af0b80a8a8af8ddef41d6725a8cd0f9c0647bada54c2f",
        "42b6d7c5d9ec3ec0427a9e534afc769ee01ce018fc9d7e71ec358fd5955c4e82",
    ),
];

const XYCE_BUG1085_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 5] = [
    (
        "Netlists/Certification_Tests/BUG_1085_SON/Manifest.txt",
        64,
        "567e0a5c9a971035001868b5cd66aa539b99be841d76600c7eb4b8bd9a518a62",
        "da2db3eb958496f378990496fcc665417329bceab4d8d5a78ef30ae06bbfd40a",
    ),
    (
        "Netlists/Certification_Tests/BUG_1085_SON/bug1085son.cir.sh",
        2_239,
        "07a02f35c2b044e22282e74e2085a57568e44ccc6de5cdd017b6db52cb531435",
        "5525e97b412a34d876195f051d04607442840a175a253db2f370337cda489327",
    ),
    (
        XYCE_BUG1085_HISTORICAL_EXCLUDE_PATH,
        18,
        "dd122a5bbe6215b31486e6f21ccdd7c9f51b44ac656599191580aa28be078bf3",
        "6e39a5b2e896d6d8f85a607ddc466b9100f4f48de9ba34b4d607bdd169911962",
    ),
    (
        "Netlists/Certification_Tests/BUG_1085_SON/tags",
        37,
        "fc71936338459932b30d55524ffbe314e69db3be5f0ef36b164c3ad482b6f5ec",
        "07050ded83e26db24bfe730e0c1d16c2fd7696198dcf807145eaff0bb6164a92",
    ),
    (
        "TestScripts/file_compare.pl",
        7_465,
        "a700143baddab265ca2e74d69541432fb27ae66600c3fee71968797fc78efcb0",
        "04dd69b4e4cfe543a39f663966229be877fa595a7c6c885dadf2173814f85895",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum XyceBug1085UserFunctionRole {
    I0WrapperOwner,
    AlphaRenamedReferenceBaseline,
}

impl XyceBug1085UserFunctionRole {
    pub(super) fn result_contract(self) -> &'static str {
        match self {
            Self::I0WrapperOwner => XYCE_BUG1085_WRAPPER_OWNER_CONTRACT,
            Self::AlphaRenamedReferenceBaseline => XYCE_BUG1085_REFERENCE_BASELINE_CONTRACT,
        }
    }

    pub(super) fn plan_purpose(self) -> XyceStaticTranPlanPurpose {
        match self {
            Self::I0WrapperOwner => XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            Self::AlphaRenamedReferenceBaseline => XyceStaticTranPlanPurpose::RelationalFamily,
        }
    }

    fn leaf_name(self) -> &'static str {
        match self {
            Self::I0WrapperOwner => "I0",
            Self::AlphaRenamedReferenceBaseline => "TEST",
        }
    }

    fn expected_plan_contract(self) -> XyceStaticTranContract {
        match self {
            Self::I0WrapperOwner => XyceStaticTranContract::WrapperStatic,
            Self::AlphaRenamedReferenceBaseline => XyceStaticTranContract::PlainStatic,
        }
    }

    pub(super) fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_BUG1085_OWNER_RECORD => Some(Self::I0WrapperOwner),
            XYCE_BUG1085_CONTROL_RECORD => Some(Self::AlphaRenamedReferenceBaseline),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct XyceBug1085UserFunctionFamilyContract {
    pub(super) relational: XyceBaselineFamilyContract,
    pub(super) owner_path: PathBuf,
    pub(super) baseline_path: PathBuf,
    pub(super) role: XyceBug1085UserFunctionRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug1085UserFunctionRepresentation {
    I0Owner,
    AlphaRenamedTestBaseline,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct XyceBug1085UserFunctionSnapshot {
    representation: XyceBug1085UserFunctionRepresentation,
    tran_bits: [u64; 2],
    ordered_probes: Vec<String>,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    alpha_normalized_functions: BTreeMap<String, (Vec<String>, String)>,
}

impl XyceBug1085UserFunctionSnapshot {
    pub(super) fn from_plan_and_netlist(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
    ) -> Result<Self, String> {
        let functions = netlist
            .params
            .all_functions()
            .into_iter()
            .map(|function| (function.name.clone(), function))
            .collect::<BTreeMap<_, _>>();
        let role = match (functions.contains_key("I0"), functions.contains_key("TEST")) {
            (true, false) => XyceBug1085UserFunctionRole::I0WrapperOwner,
            (false, true) => XyceBug1085UserFunctionRole::AlphaRenamedReferenceBaseline,
            _ => {
                return Err(format!(
                    "{LABEL} requires exactly one I0 owner or TEST alpha-renamed baseline leaf function"
                ));
            }
        };
        let expected_name = match role {
            XyceBug1085UserFunctionRole::I0WrapperOwner => "bug1085son.cir",
            XyceBug1085UserFunctionRole::AlphaRenamedReferenceBaseline => "bug1085sonRef.cir",
        };
        if plan.deck_path.file_name().and_then(|name| name.to_str()) != Some(expected_name) {
            return Err(format!(
                "{LABEL} {role:?} is not backed by its exact physical filename"
            ));
        }
        let expected_contract = role.expected_plan_contract();
        let expected_probes = ["v(x)", "v(y)", "v(z)", "v(f)"];
        let ordered_probes = plan
            .print
            .as_ref()
            .ok_or_else(|| format!("{LABEL} requires one primary .PRINT TRAN"))?
            .probes
            .iter()
            .map(|probe| XyceTestRunner::normalize_probe(probe))
            .collect::<Vec<_>>();
        if !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.tran.step.to_bits() != 2.0e-5f64.to_bits()
            || plan.tran.stop.to_bits() != 2.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || ordered_probes
                .iter()
                .map(String::as_str)
                .ne(expected_probes)
        {
            return Err(format!("{LABEL} {role:?} transient plan changed: {plan:?}"));
        }

        if netlist.title.trim().is_empty()
            || netlist.elements.len() != 8
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || functions.len() != 3
        {
            return Err(format!(
                "{LABEL} {role:?} must remain one flat, diagnostic-free eight-element transient circuit"
            ));
        }

        let mut alpha_normalized_functions = BTreeMap::new();
        for (key, expected_args, expected_body) in [
            ("leaf", vec!["X", "Y"], "y"),
            ("diff", vec!["X", "Y"], "x-y"),
            ("f", vec!["X", "Y", "Z"], "diff(y,z)-4+$leaf(z,x)**2"),
        ] {
            let source_name = match key {
                "leaf" => role.leaf_name(),
                "diff" => "DIFF",
                "f" => "F",
                _ => unreachable!(),
            };
            let function = functions
                .get(source_name)
                .ok_or_else(|| format!("{LABEL} lost function {source_name}"))?;
            let normalized = XyceTestRunner::alpha_normalize_expression(
                &function.body,
                &[(role.leaf_name(), "$leaf")],
            );
            if function.args != expected_args || normalized != expected_body {
                return Err(format!(
                    "{LABEL} function {source_name} changed: args={:?}, body={normalized}",
                    function.args
                ));
            }
            alpha_normalized_functions.insert(key.to_string(), (function.args.clone(), normalized));
        }

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            if element.provenance != ElementProvenance::Authored {
                return Err(format!(
                    "{LABEL} element '{}' is not directly authored",
                    element.name
                ));
            }
            let name = XyceTestRunner::normalize_device_instance_name(&element.name);
            let nodes = element
                .nodes
                .iter()
                .map(|node| node.trim().to_ascii_lowercase())
                .collect::<Vec<_>>();
            let fingerprint = match (name.as_str(), &element.kind) {
                (
                    "vx" | "vy" | "vz",
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pwl {
                        points,
                        delay,
                        repeat_from,
                    }),
                ) => {
                    let expected_nodes = vec![name[1..].to_string(), "0".to_string()];
                    let expected_values: [Value; 3] = match name.as_str() {
                        "vx" => [1.0, 2.0, 0.0],
                        "vy" => [2.0, 2.0, 0.0],
                        "vz" => [3.0, 2.0, 1.0],
                        _ => unreachable!(),
                    };
                    let expected_points = [0.0f64, 1.0e-3, 2.0e-3]
                        .into_iter()
                        .zip(expected_values)
                        .collect::<Vec<_>>();
                    if nodes != expected_nodes
                        || delay.to_bits() != 0.0f64.to_bits()
                        || repeat_from.is_some()
                        || points.len() != expected_points.len()
                        || points.iter().zip(expected_points).any(
                            |((time, value), (expected_time, expected_value))| {
                                time.to_bits() != expected_time.to_bits()
                                    || value.to_bits() != expected_value.to_bits()
                            },
                        )
                    {
                        return Err(format!("{LABEL} {name} PWL definition changed"));
                    }
                    XyceRelationalElementFingerprint {
                        kind: "V:PWL".to_string(),
                        nodes,
                        numeric_bits: points
                            .iter()
                            .flat_map(|(time, value)| [time.to_bits(), value.to_bits()])
                            .collect(),
                        text: Vec::new(),
                    }
                }
                (
                    "bf",
                    ElementKind::BehavioralVoltage {
                        expression,
                        tc1,
                        tc2,
                        multiplicity,
                    },
                ) => {
                    let normalized = XyceTestRunner::alpha_normalize_expression(expression, &[]);
                    if nodes != ["f", "0"]
                        || normalized != "f(v(x),v(y),v(z))"
                        || tc1.to_bits() != 0.0f64.to_bits()
                        || tc2.to_bits() != 0.0f64.to_bits()
                        || multiplicity.value.to_bits() != 1.0f64.to_bits()
                        || multiplicity.value_expr.is_some()
                        || multiplicity.given
                    {
                        return Err(format!("{LABEL} BF behavioral definition changed"));
                    }
                    XyceRelationalElementFingerprint {
                        kind: "B:V".to_string(),
                        nodes,
                        numeric_bits: Vec::new(),
                        text: vec![normalized],
                    }
                }
                (
                    "rx" | "ry" | "rz" | "rf",
                    ElementKind::Resistor {
                        value,
                        value_expr,
                        model,
                        instance_params,
                        deferred_params,
                    },
                ) => {
                    let expected_node = name[1..].to_string();
                    if nodes != [expected_node, "0".to_string()]
                        // SPICE's one-letter `M` suffix is milli, not mega.
                        || value.to_bits() != 1.0e-3f64.to_bits()
                        || value_expr.is_some()
                        || model.is_some()
                        || !instance_params.is_empty()
                        || !deferred_params.is_empty()
                    {
                        return Err(format!("{LABEL} {name} resistor definition changed"));
                    }
                    XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                _ => {
                    return Err(format!(
                        "{LABEL} contains unqualified element '{}' ({:?})",
                        element.name, element.kind
                    ));
                }
            };
            if elements.insert(name.clone(), fingerprint).is_some() {
                return Err(format!("{LABEL} contains duplicate element name {name}"));
            }
        }

        Ok(Self {
            representation: match role {
                XyceBug1085UserFunctionRole::I0WrapperOwner => {
                    XyceBug1085UserFunctionRepresentation::I0Owner
                }
                XyceBug1085UserFunctionRole::AlphaRenamedReferenceBaseline => {
                    XyceBug1085UserFunctionRepresentation::AlphaRenamedTestBaseline
                }
            },
            tran_bits: [plan.tran.step.to_bits(), plan.tran.stop.to_bits()],
            ordered_probes,
            elements,
            alpha_normalized_functions,
        })
    }
}

impl XyceTestRunner {
    pub(super) fn bug1085_user_function_file_compare_tolerance() -> XyceFileCompareTolerance {
        XyceFileCompareTolerance::BUG1085_USER_FUNCTION
    }

    pub(super) fn bug1085_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1085_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1085_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1085_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1085_historical_oracle_records(
        records: &[String],
    ) -> Result<(), String> {
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if XYCE_BUG1085_PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || records.len() != XYCE_BUG1085_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1085_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1085_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1085_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 provenance changed: pretrim={XYCE_BUG1085_PRETRIM_COMMIT}, records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1085_historical_oracle_provenance() -> Result<(), String> {
        Self::validate_bug1085_historical_oracle_records(
            &Self::bug1085_historical_oracle_provenance_records(),
        )
    }

    fn validate_bug1085_source_directory(directory: &Path) -> Result<(), String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {LABEL} source directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG1085_RETAINED_SOURCES
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to enumerate {LABEL} source directory: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!(
                    "{LABEL} source directory contains a case collision"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} member {name:?}: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if name != expected_name
                || canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} member {name:?} identity changed: bytes={}, sha256={sha256}, blake3={content_blake3}",
                    canonical.len()
                ));
            }
        }
        let expected_names = expected.keys().cloned().collect::<BTreeSet<_>>();
        if observed != expected_names {
            return Err(format!(
                "{LABEL} physical source census changed: expected {expected_names:?}, got {observed:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn bug1085_user_function_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug1085UserFunctionFamilyContract, String>> {
        let role = XyceBug1085UserFunctionRole::for_record(&deck.relative_path)?;
        Some((|| {
            if deck.section != XyceDeckSection::Netlists
                || Self::normalize_manifest_key(&self.relative_key(&deck.path))
                    != Self::normalize_manifest_key(&deck.relative_path)
            {
                return Err(format!(
                    "recognized {LABEL} record '{}' is not backed by its exact Netlists path",
                    deck.relative_path
                ));
            }
            let owner_path = self.root.join(XYCE_BUG1085_OWNER_PATH);
            let baseline_path = self.root.join(XYCE_BUG1085_CONTROL_PATH);
            let expected_path = match role {
                XyceBug1085UserFunctionRole::I0WrapperOwner => &owner_path,
                XyceBug1085UserFunctionRole::AlphaRenamedReferenceBaseline => &baseline_path,
            };
            if !Self::same_path(&deck.path, expected_path) {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its canonical path"
                ));
            }
            let contract = XyceBug1085UserFunctionFamilyContract {
                relational: XyceBaselineFamilyContract {
                    kind: XyceBaselineFamilyKind::Bug1085UserFunctionI0,
                    comparison: XyceBaselineFamilyComparison::Release710FileCompare(
                        Self::bug1085_user_function_file_compare_tolerance(),
                    ),
                    family: XYCE_BUG1085_FAMILY.to_string(),
                    baseline_path: baseline_path.clone(),
                    member_paths: vec![owner_path.clone(), baseline_path.clone()],
                    target_path: Some(expected_path.clone()),
                },
                owner_path,
                baseline_path,
                role,
            };
            self.validate_bug1085_user_function_provenance(&contract)?;
            Ok(contract)
        })())
    }

    pub(super) fn validate_bug1085_user_function_provenance(
        &self,
        contract: &XyceBug1085UserFunctionFamilyContract,
    ) -> Result<(), String> {
        Self::validate_bug1085_historical_oracle_provenance()?;
        let expected_target = match contract.role {
            XyceBug1085UserFunctionRole::I0WrapperOwner => &contract.owner_path,
            XyceBug1085UserFunctionRole::AlphaRenamedReferenceBaseline => &contract.baseline_path,
        };
        if contract.relational.kind != XyceBaselineFamilyKind::Bug1085UserFunctionI0
            || contract.relational.comparison
                != XyceBaselineFamilyComparison::Release710FileCompare(
                    Self::bug1085_user_function_file_compare_tolerance(),
                )
            || contract.relational.family != XYCE_BUG1085_FAMILY
            || !Self::same_path(&contract.relational.baseline_path, &contract.baseline_path)
            || contract.relational.member_paths.len() != 2
            || !Self::same_path(&contract.relational.member_paths[0], &contract.owner_path)
            || !Self::same_path(
                &contract.relational.member_paths[1],
                &contract.baseline_path,
            )
            || !contract
                .relational
                .target_path
                .as_ref()
                .is_some_and(|path| Self::same_path(path, expected_target))
        {
            return Err(format!(
                "{LABEL} contract is not the exact reference-GOODFILE/owner-TESTFILE pair"
            ));
        }

        let prefix = "netlists/certification_tests/bug_1085_son/";
        let wrappers = Self::load_upstream_wrapper_decks(&self.root);
        let family_wrappers = wrappers
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if family_wrappers != BTreeSet::from([XYCE_BUG1085_OWNER_RECORD]) {
            return Err(format!(
                "{LABEL} wrapper ownership changed: {family_wrappers:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let family_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let control_exclusion = exclusions.get(XYCE_BUG1085_CONTROL_RECORD);
        if family_exclusions != BTreeSet::from([XYCE_BUG1085_CONTROL_RECORD])
            || control_exclusion.is_none_or(|exclusion| {
                Self::normalize_manifest_key(&exclusion.source)
                    != Self::normalize_manifest_key(XYCE_BUG1085_HISTORICAL_EXCLUDE_PATH)
                    || !matches!(
                        &exclusion.disposition,
                        XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                            expected_contract,
                        } if expected_contract
                            == XYCE_BUG1085_REFERENCE_BASELINE_CONTRACT
                    )
            })
        {
            return Err(format!(
                "{LABEL} reference baseline lost its exact independent qualification"
            ));
        }

        Self::validate_bug1085_source_directory(&self.root.join(XYCE_BUG1085_FAMILY_DIR))?;
        let output_directory = self
            .root
            .join("OutputData/Certification_Tests/BUG_1085_SON");
        match fs::symlink_metadata(&output_directory) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to inspect {LABEL} OutputData directory: {error}"
                ));
            }
            Ok(_) => {
                return Err(format!(
                    "{LABEL} must not acquire fabricated checked-in numerical output"
                ));
            }
        }
        for path in [&contract.owner_path, &contract.baseline_path] {
            self.reject_wrapper_output_artifacts(path)
                .map_err(|error| format!("{LABEL} relational record {error}"))?;
        }

        let owner_plan = self
            .static_tran_plan_for_path_with_purpose(
                &contract.owner_path,
                XyceBug1085UserFunctionRole::I0WrapperOwner.plan_purpose(),
            )
            .map_err(|error| format!("{LABEL} owner plan failed: {error}"))?;
        let baseline_plan = self
            .static_tran_plan_for_path_with_purpose(
                &contract.baseline_path,
                XyceBug1085UserFunctionRole::AlphaRenamedReferenceBaseline.plan_purpose(),
            )
            .map_err(|error| format!("{LABEL} baseline plan failed: {error}"))?;
        let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_plan.deck_path)
            .map_err(|error| format!("{LABEL} owner typed parse failed: {error}"))?;
        let baseline_netlist =
            Self::parse_xyce_netlist(&baseline_plan.source, &baseline_plan.deck_path)
                .map_err(|error| format!("{LABEL} baseline typed parse failed: {error}"))?;
        let owner_snapshot =
            XyceBug1085UserFunctionSnapshot::from_plan_and_netlist(&owner_plan, &owner_netlist)?;
        let baseline_snapshot = XyceBug1085UserFunctionSnapshot::from_plan_and_netlist(
            &baseline_plan,
            &baseline_netlist,
        )?;
        Self::compare_bug1085_user_function_snapshots(&baseline_snapshot, &owner_snapshot)
    }

    pub(super) fn compare_bug1085_user_function_snapshots(
        baseline: &XyceBug1085UserFunctionSnapshot,
        target: &XyceBug1085UserFunctionSnapshot,
    ) -> Result<(), String> {
        if baseline.representation
            != XyceBug1085UserFunctionRepresentation::AlphaRenamedTestBaseline
            || target.representation != XyceBug1085UserFunctionRepresentation::I0Owner
        {
            return Err(format!(
                "{LABEL} must compare TEST reference GOODFILE against I0 owner TESTFILE"
            ));
        }
        if baseline.tran_bits != target.tran_bits
            || baseline.ordered_probes != target.ordered_probes
            || baseline.elements != target.elements
            || baseline.alpha_normalized_functions != target.alpha_normalized_functions
        {
            return Err(format!(
                "{LABEL} differs beyond the sole I0-to-TEST leaf-function alpha-renaming"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1085_user_function_analytic_table(
        table: &XycePrnTable,
    ) -> Result<(), String> {
        let column = |name: &str| {
            table
                .columns
                .iter()
                .position(|column| column.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("{LABEL} output lost {name} column"))
        };
        let time = column("TIME")?;
        let x = column("V(X)")?;
        let y = column("V(Y)")?;
        let z = column("V(Z)")?;
        let f = column("V(F)")?;
        let normalized_columns = table
            .columns
            .iter()
            .map(|column| column.to_ascii_uppercase())
            .collect::<Vec<_>>();
        let normalized_columns = normalized_columns
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let index = table
            .columns
            .iter()
            .position(|column| column.eq_ignore_ascii_case("INDEX"));
        if (!matches!(
            normalized_columns.as_slice(),
            ["TIME", "V(X)", "V(Y)", "V(Z)", "V(F)"]
                | ["INDEX", "TIME", "V(X)", "V(Y)", "V(Z)", "V(F)"]
        )) || table.rows.len() < 3
        {
            return Err(format!(
                "{LABEL} output must contain only optional INDEX plus TIME/V(X)/V(Y)/V(Z)/V(F) and at least three rows, got {:?}",
                table.columns
            ));
        }

        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != table.columns.len()
                || row.iter().any(|value| !value.is_finite())
                || index.is_some_and(|index| row[index] != row_index as Value)
                || row[time] < 0.0
                || row[time] > 2.0e-3
            {
                return Err(format!(
                    "{LABEL} output row {row_index} has an invalid layout, axis, or non-finite value"
                ));
            }
            let expected = row[y] - row[z] - 4.0 + row[x] * row[x];
            let scale = expected.abs().max(row[f].abs()).max(1.0);
            if (row[f] - expected).abs() > 64.0 * Value::EPSILON * scale {
                return Err(format!(
                    "{LABEL} output row {row_index} violates f=y-z-4+x^2: time={}, expected={expected}, actual={}",
                    row[time], row[f]
                ));
            }
        }

        for (expected_time, expected_value) in [(0.0_f64, -4.0_f64), (1.0e-3, 0.0), (2.0e-3, -5.0)]
        {
            let row = table
                .rows
                .iter()
                .find(|row| row[time].to_bits() == expected_time.to_bits())
                .ok_or_else(|| format!("{LABEL} output lost PWL breakpoint {expected_time}"))?;
            if (row[f] - expected_value).abs()
                > 64.0 * Value::EPSILON * expected_value.abs().max(1.0)
            {
                return Err(format!(
                    "{LABEL} output PWL breakpoint {expected_time} has V(F)={}, expected {expected_value}",
                    row[f]
                ));
            }
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
    fn bug1085_historical_provenance_is_exact_and_rejects_drift() {
        let records = XyceTestRunner::bug1085_historical_oracle_provenance_records();
        XyceTestRunner::validate_bug1085_historical_oracle_records(&records)
            .expect("Release-7.10 BUG1085 provenance remains exact");
        let mut changed = records;
        changed[0].push('x');
        assert!(XyceTestRunner::validate_bug1085_historical_oracle_records(&changed).is_err());
    }

    #[test]
    fn bug1085_typed_pair_allows_only_the_leaf_function_alpha_rename() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let owner_path = root.join(XYCE_BUG1085_OWNER_PATH);
        let baseline_path = root.join(XYCE_BUG1085_CONTROL_PATH);
        let owner_plan = runner
            .static_tran_plan_for_path_with_purpose(
                &owner_path,
                XyceBug1085UserFunctionRole::I0WrapperOwner.plan_purpose(),
            )
            .expect("canonical BUG1085 owner plan");
        let baseline_plan = runner
            .static_tran_plan_for_path_with_purpose(
                &baseline_path,
                XyceBug1085UserFunctionRole::AlphaRenamedReferenceBaseline.plan_purpose(),
            )
            .expect("canonical BUG1085 baseline plan");
        let owner_netlist = XyceTestRunner::parse_xyce_netlist(&owner_plan.source, &owner_path)
            .expect("parse canonical BUG1085 owner");
        let baseline_netlist =
            XyceTestRunner::parse_xyce_netlist(&baseline_plan.source, &baseline_path)
                .expect("parse canonical BUG1085 baseline");
        let owner =
            XyceBug1085UserFunctionSnapshot::from_plan_and_netlist(&owner_plan, &owner_netlist)
                .expect("qualify canonical BUG1085 owner");
        let baseline = XyceBug1085UserFunctionSnapshot::from_plan_and_netlist(
            &baseline_plan,
            &baseline_netlist,
        )
        .expect("qualify canonical BUG1085 baseline");
        XyceTestRunner::compare_bug1085_user_function_snapshots(&baseline, &owner)
            .expect("I0 and TEST forms are semantically identical");
        assert!(
            XyceTestRunner::compare_bug1085_user_function_snapshots(&owner, &baseline).is_err()
        );
    }
}
