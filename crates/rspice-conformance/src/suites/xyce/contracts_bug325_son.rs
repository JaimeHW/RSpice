use super::*;
use rspice_core::netlist::SourceSpec;
use std::io::Read as _;

const LABEL: &str = "BUG_325_SON VBIC multiplicity equivalence";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_325_SON";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_325_SON";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_325_son/";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_325_SON/exclude";
const OWNER_CONTRACT: &str = "bug325_vbic_multiplicity_wrapper_owner";
const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const GENERATED_MODEL: &str = "vbic13";
const GENERATED_SOURCE_DIGEST: &str = "aa00e2e747501388";
const GENERATED_CHECKPOINT_IDENTITY: &str =
    "1c950f1c0ae955382f550740b2be24f583b4978276cf67107be777a13273559e";

const OWNER_NAME: &str = "vbic_3T_et_cf.cir";
const CONTROL_NAME: &str = "vbic_3T_et_cf_m2.cir";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_325_SON/vbic_3T_et_cf.cir";
const CONTROL_PATH: &str = "Netlists/Certification_Tests/BUG_325_SON/vbic_3T_et_cf_m2.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_325_son/vbic_3t_et_cf.cir";
const CONTROL_RECORD: &str = "netlists/certification_tests/bug_325_son/vbic_3t_et_cf_m2.cir";

const HISTORICAL_CONTENT_BYTES: usize = 152_359;
const HISTORICAL_STREAM_BYTES: usize = 3_334;
const HISTORICAL_STREAM_SHA256: &str =
    "a2d388458b59bb4a3bc40f6b3503ed76bce013cdcb5c33bce592a0cce631f9ed";
const HISTORICAL_STREAM_BLAKE3: &str =
    "c856faa907c4d3efcfb9570359bb9c5d7d74ff74b8e9cdff539cb2fb6c6e9dec";
const HISTORICAL: [(&str, usize, &str, &str); 12] = [
    (
        "Netlists/Certification_Tests/BUG_325_SON/CMakeLists.txt",
        1_456,
        "3da53e47bd7a0cf745b0a2b8ec491bec6a775dc83a69e17bfb1d8ff10f2ce5de",
        "510bcfca24425f405d918652c122d9bb15cf98f96cff138b6191edbc1c128b6d",
    ),
    (
        "Netlists/Certification_Tests/BUG_325_SON/Manifest.txt",
        73,
        "a73bbc725dc987394a9d88c6081e674c4f1645b733c87aafe3a46ecd7c243378",
        "a754c679cfdcbc13ad531dc74c18c95520a648b6f9b1f733467c2c0095a0f9a7",
    ),
    (
        EXCLUSION_SOURCE,
        21,
        "fc5a94f3bb07a9071448e82ff84d76dddb1223d71d4644b0f1dad23539b86cbf",
        "bf3379fe99ee0493b3eb5f555d79243180b1c62a0ef37d7cd8f2c86cb618a355",
    ),
    (
        "Netlists/Certification_Tests/BUG_325_SON/tags",
        26,
        "2290f54179695bc256cfa2349b7bdddb9bb34ae2a4ea1c0a0b851b74c0ddbf9e",
        "922b2b936eb3c3ceaa87454ca4f2a2b23c2a79ff53860319353fb66c679e7ee3",
    ),
    (
        OWNER_PATH,
        397,
        "cb3d364c884cd0cb8cbfeaf53ba507b17d1be9fbf19b14f2d89ee850ee52215f",
        "ccbdf2c0cd836dc6af0f4c23203724a0f627701b19f3170a40bed965558bf223",
    ),
    (
        "Netlists/Certification_Tests/BUG_325_SON/vbic_3T_et_cf.cir.sh",
        1_509,
        "19504a908d410168f2e9ad3493f633f670785d5d971b843de9f104546279e82f",
        "2d252e6b1c43a3f5a48f140c19a0518195477a191380708749e3f78d1f62e9d8",
    ),
    (
        CONTROL_PATH,
        432,
        "ef85b60484f436c190d4afb56cd2f13ed9a05a26d28fcfd0bec37a59bbaeee36",
        "64c0e7e5fd53b077d789588404a6cf599e1e06bf9871456e515fc91f2e32d82a",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
    (
        "TestScripts/XyceVerify/DCSources.pm",
        2_739,
        "b2ddcab5ad5a89c428b9b4430190fa27ef7106da7e7afeb31452c81890a9a006",
        "0905f9dc79d7c5bdbe17e3c2360cd063d6fcbf41823a410f98b236783d109ad7",
    ),
    (
        "TestScripts/XyceVerify/DCSweep.pm",
        9_301,
        "2246da2374e6cce3ea516a50e472fb07f7481e8b0effb20d4a650e6b6cb1eda0",
        "b9cc7d905d001ebe2ace44936b9631e4bdcbf42bca4d4b34c5866262cd11d9a3",
    ),
    (
        "TestScripts/XyceVerify/StepSweep.pm",
        8_731,
        "84b2d485c1848f2e456463de8a5015205d87c3db8a6d070547d6f9464618fed6",
        "db1b142ab3ae9163bbe02bd68b5b3a6311436adbf27c06d71a5c05df9b6973e7",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];

const RETAINED_STREAM_BYTES: usize = 388;
const RETAINED_STREAM_SHA256: &str =
    "863ef4be4ad80dca58a2fd037afc5e7a757383f41c3e56cc1b9227606ed484a2";
const RETAINED_STREAM_BLAKE3: &str =
    "d5b98ed91d98977038ead73994d6bc61e1dd28f19f0338b40895ae694f50b8e4";
const RETAINED: [(&str, usize, &str, &str); 2] = [
    (
        OWNER_NAME,
        397,
        "cb3d364c884cd0cb8cbfeaf53ba507b17d1be9fbf19b14f2d89ee850ee52215f",
        "ccbdf2c0cd836dc6af0f4c23203724a0f627701b19f3170a40bed965558bf223",
    ),
    (
        CONTROL_NAME,
        432,
        "ef85b60484f436c190d4afb56cd2f13ed9a05a26d28fcfd0bec37a59bbaeee36",
        "64c0e7e5fd53b077d789588404a6cf599e1e06bf9871456e515fc91f2e32d82a",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Bug325SonRole {
    WrapperOwner,
}

impl Bug325SonRole {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(record) == OWNER_RECORD)
            .then_some(Self::WrapperOwner)
    }

    pub(super) const fn contract(self) -> &'static str {
        OWNER_CONTRACT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug325WorkerRole {
    ExplicitPair,
    MultiplicityTwo,
    SingleDeviceCounterfactual,
}

impl Bug325WorkerRole {
    const fn expected_devices(self) -> usize {
        match self {
            Self::ExplicitPair => 2,
            Self::MultiplicityTwo | Self::SingleDeviceCounterfactual => 1,
        }
    }

    const fn expected_m(self) -> Option<Value> {
        match self {
            Self::MultiplicityTwo => Some(2.0),
            Self::ExplicitPair | Self::SingleDeviceCounterfactual => None,
        }
    }
}

#[derive(Debug)]
struct Bug325Run {
    table: XycePrnTable,
    results: Vec<DcSweepPointResult>,
}

impl XyceTestRunner {
    fn validate_bug325_record_streams() -> Result<(), String> {
        let content_bytes = HISTORICAL.iter().map(|record| record.1).sum::<usize>();
        let mut historical = HISTORICAL
            .iter()
            .map(|(path, bytes, sha256, b3)| {
                format!(
                    "{PRETRIM_COMMIT}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{b3}"
                )
            })
            .collect::<Vec<_>>();
        historical.sort();
        let historical = historical.join("\n");
        let historical_sha = format!("{:x}", Sha256::digest(historical.as_bytes()));
        let historical_b3 = blake3::hash(historical.as_bytes()).to_hex().to_string();

        let mut retained = RETAINED
            .iter()
            .map(|(name, bytes, sha256, b3)| {
                format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha256}\t{b3}")
            })
            .collect::<Vec<_>>();
        retained.sort();
        let retained = retained.join("\n");
        let retained_sha = format!("{:x}", Sha256::digest(retained.as_bytes()));
        let retained_b3 = blake3::hash(retained.as_bytes()).to_hex().to_string();

        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || HISTORICAL.len() != 12
            || content_bytes != HISTORICAL_CONTENT_BYTES
            || historical.len() != HISTORICAL_STREAM_BYTES
            || historical_sha != HISTORICAL_STREAM_SHA256
            || historical_b3 != HISTORICAL_STREAM_BLAKE3
            || RETAINED.len() != 2
            || retained.len() != RETAINED_STREAM_BYTES
            || retained_sha != RETAINED_STREAM_SHA256
            || retained_b3 != RETAINED_STREAM_BLAKE3
        {
            return Err(format!(
                "{LABEL} provenance changed: historical={}/{content_bytes}/{}/sha={historical_sha}/b3={historical_b3}; retained={}/{}/sha={retained_sha}/b3={retained_b3}",
                HISTORICAL.len(),
                historical.len(),
                RETAINED.len(),
                retained.len(),
            ));
        }
        Ok(())
    }

    fn read_bug325_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug325_record_streams()?;
        let directory = self.root.join(FAMILY_DIRECTORY);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} directory must be a regular non-symlink directory"
            ));
        }
        let expected = RETAINED
            .into_iter()
            .map(|record| (record.0.to_ascii_lowercase(), record))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} directory: {error}"))?
        {
            if abort.is_aborted() {
                return Err(format!("{LABEL} source census aborted"));
            }
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} member {} is not a regular file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            let Some((expected_name, expected_bytes, expected_sha, expected_b3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name || observed.contains_key(&key) {
                return Err(format!("{LABEL} member case/census changed: {name:?}"));
            }
            let cap = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(3))
                .ok_or_else(|| format!("{LABEL} source-size bound overflowed"))?;
            if metadata.len() > cap as u64 {
                return Err(format!(
                    "{LABEL} member {name:?} exceeds its bounded envelope"
                ));
            }
            let mut bytes = Vec::with_capacity((metadata.len() as usize).min(cap));
            fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} member: {error}"))?
                .take((cap + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} member: {error}"))?;
            if bytes.len() > cap {
                return Err(format!("{LABEL} bounded source read grew"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha = format!("{:x}", Sha256::digest(&canonical));
            let b3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes || sha != expected_sha || b3 != expected_b3 {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha={sha}, b3={b3}",
                    canonical.len()
                ));
            }
            observed.insert(key, canonical);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{LABEL} retained census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug325_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug325SonRole,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        if deck.section != XyceDeckSection::Netlists
            || role != Bug325SonRole::WrapperOwner
            || Self::normalize_manifest_key(&deck.relative_path) != OWNER_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != OWNER_RECORD
            || !Self::same_path(&deck.path, &self.root.join(OWNER_PATH))
        {
            return Err(format!("recognized {LABEL} owner is not canonical"));
        }
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([OWNER_RECORD.to_string()]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions invalid: {error}"))?;
        if exclusions.contains_key(OWNER_RECORD) {
            return Err(format!("{LABEL} owner must not be excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<Vec<_>>();
        let [(record, qualification)] = family.as_slice() else {
            return Err(format!("{LABEL} exclusion census changed: {family:?}"));
        };
        if record.as_str() != CONTROL_RECORD
            || qualification.source != EXCLUSION_SOURCE
            || !matches!(
                qualification.disposition,
                XyceUpstreamExclusionDisposition::Excluded
            )
        {
            return Err(format!(
                "{LABEL} M=2 control exclusion changed: {qualification:?}"
            ));
        }
        let members = self.read_bug325_directory(abort)?;
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        for path in [OWNER_PATH, CONTROL_PATH] {
            self.reject_wrapper_output_artifacts(&self.root.join(path))
                .map_err(|error| format!("{LABEL} {path} {error}"))?;
        }
        Ok(members)
    }

    fn bug325_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug325_model(model: &rspice_core::netlist::ModelDef) -> bool {
        let expected: [(&str, Value); 10] = [
            ("LEVEL", 11.0),
            ("RCX", 10.0),
            ("RCI", 10.0),
            ("RBX", 1.0),
            ("RBI", 10.0),
            ("RE", 1.0),
            ("RBP", 10.0),
            ("RS", 10.0),
            ("IBEN", 1.0e-13),
            ("RTH", 100.0),
        ];
        model.name.eq_ignore_ascii_case("vbicmodel")
            && model.model_type.eq_ignore_ascii_case("npn")
            && model.params.len() == expected.len()
            && expected.iter().all(|(name, value)| {
                model.params.iter().any(|(actual_name, actual_value)| {
                    actual_name.eq_ignore_ascii_case(name)
                        && actual_value.to_bits() == value.to_bits()
                })
            })
            && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
    }

    fn validate_bug325_worker(
        &self,
        role: Bug325WorkerRole,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let expected_grid = (0..26)
            .map(|index| 0.5 + 0.02 * index as Value)
            .collect::<Vec<_>>();
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VBE")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 0.5f64.to_bits()
            || plan.dc.stop.to_bits() != 1.0f64.to_bits()
            || plan.dc.step.to_bits() != 0.02f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.dc.primary_spec().points() != expected_grid
            || plan.print.probes != ["v(bx)", "i(vib)", "i(vic)"]
        {
            return Err(format!("{LABEL} {role:?} DC plan changed: {plan:?}"));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {role:?} no longer parses: {error}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 4 + role.expected_devices()
            || netlist.models.len() != 1
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
            || !Self::validate_bug325_model(&netlist.models[0])
        {
            return Err(format!("{LABEL} {role:?} typed envelope/model changed"));
        }
        let sources = [
            ("vbe", ["bx", "0"]),
            ("vcb", ["cx", "bx"]),
            ("vib", ["bx", "b"]),
            ("vic", ["cx", "c"]),
        ];
        for (element, (name, nodes)) in netlist.elements.iter().take(4).zip(sources) {
            if !element.name.eq_ignore_ascii_case(name)
                || !Self::bug325_nodes_match(&element.nodes, &nodes)
                || element.provenance != ElementProvenance::Authored
                || !matches!(&element.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                    if value.to_bits() == 0.0f64.to_bits())
            {
                return Err(format!(
                    "{LABEL} {role:?} source topology changed: {element:?}"
                ));
            }
        }
        let expected_names = match role {
            Bug325WorkerRole::ExplicitPair => &["q1", "q2"][..],
            Bug325WorkerRole::MultiplicityTwo | Bug325WorkerRole::SingleDeviceCounterfactual => {
                &["q1"][..]
            }
        };
        for (element, expected_name) in netlist.elements.iter().skip(4).zip(expected_names) {
            let expected_m = role.expected_m();
            if !element.name.eq_ignore_ascii_case(expected_name)
                || !Self::bug325_nodes_match(&element.nodes, &["c", "b", "0"])
                || element.provenance != ElementProvenance::Authored
                || !matches!(&element.kind, ElementKind::Bjt {
                    model,
                    bjt_type: rspice_core::netlist::BjtType::Npn,
                    instance_params,
                    deferred_params,
                } if model.eq_ignore_ascii_case("vbicmodel")
                    && deferred_params.is_empty()
                    && match expected_m {
                        None => instance_params.is_empty(),
                        Some(expected) => matches!(instance_params.as_slice(), [(name, value)]
                            if name.eq_ignore_ascii_case("M")
                                && value.to_bits() == expected.to_bits()),
                    })
            {
                return Err(format!(
                    "{LABEL} {role:?} BJT topology changed: {element:?}"
                ));
            }
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.print_delimiter != Some(PrintDelimiter::Whitespace)
            || request.print_precision.is_some()
            || request.print_width.is_some()
            || request
                .operands
                .iter()
                .map(String::as_str)
                .ne(["v(bx)", "i(vib)", "i(vic)"])
            || !request.expressions.is_empty()
            || request.dependencies.len() != 3
        {
            return Err(format!(
                "{LABEL} {role:?} PRINT schema changed: {request:?}"
            ));
        }
        let descriptor =
            rspice_core::device::veriloga_builtins::generated_veriloga_model_descriptor(
                GENERATED_MODEL,
            )
            .ok_or_else(|| format!("{LABEL} generated {GENERATED_MODEL} is not linked"))?;
        if descriptor.model_name != GENERATED_MODEL
            || descriptor.module_name != GENERATED_MODEL
            || descriptor.source_digest != GENERATED_SOURCE_DIGEST
            || descriptor.abi_version
                != rspice_core::device::veriloga_builtins::GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION
            || descriptor.checkpoint_identity != GENERATED_CHECKPOINT_IDENTITY
            || descriptor.terminals.len() != 3
            || descriptor
                .terminals
                .iter()
                .map(|terminal| (terminal.name, terminal.current_parameter))
                .ne([("c", "ic"), ("b", "ib"), ("e", "ie")])
            || descriptor.total_node_count != 12
            || descriptor.internal_node_names
                != ["dt", "cx", "ci", "bx", "bi", "ei", "bp", "xf1", "xf2"]
            || descriptor.branch_count != 0
        {
            return Err(format!(
                "{LABEL} generated VBIC13 identity changed: {descriptor:?}"
            ));
        }
        let circuit = self
            .create_xyce_engine()
            .build_circuit(&netlist)
            .map_err(|error| format!("{LABEL} {role:?} assembly failed: {error}"))?;
        let generated = circuit
            .device_op_report()
            .entries
            .into_iter()
            .filter(|entry| entry.device_kind.eq_ignore_ascii_case(GENERATED_MODEL))
            .collect::<Vec<_>>();
        if !circuit.has_generated_veriloga_devices()
            || generated.len() != role.expected_devices()
            || circuit.num_nodes()
                != match role {
                    Bug325WorkerRole::ExplicitPair => 22,
                    Bug325WorkerRole::MultiplicityTwo
                    | Bug325WorkerRole::SingleDeviceCounterfactual => 13,
                }
            || circuit.num_branches() != 4
            || circuit.device_count() != 4 + role.expected_devices()
            || generated
                .iter()
                .map(|entry| entry.name.to_ascii_lowercase())
                .ne(expected_names.iter().map(|name| name.to_string()))
        {
            return Err(format!(
                "{LABEL} {role:?} did not assemble exclusively through generated VBIC13"
            ));
        }
        Ok(plan)
    }

    fn validate_bug325_current_shape(
        role: Bug325WorkerRole,
        table: &XycePrnTable,
    ) -> Result<(), String> {
        let mut first_currents = None;
        let mut previous_currents = None;
        for (index, row) in table.rows.iter().enumerate() {
            let currents = match row.as_slice() {
                [_, _, ib, ic] => [*ib, *ic],
                _ => return Err(format!("{LABEL} {role:?} current row shape changed")),
            };
            if currents[0] <= 0.0 || currents[1] <= currents[0] {
                return Err(format!(
                    "{LABEL} {role:?} lost positive collector-over-base current ordering at row {index}: ib={:e}, ic={:e}",
                    currents[0], currents[1]
                ));
            }
            if previous_currents.is_some_and(|previous: [Value; 2]| {
                currents
                    .iter()
                    .zip(previous)
                    .any(|(current, previous)| *current <= previous)
            }) {
                return Err(format!(
                    "{LABEL} {role:?} terminal currents are not strictly increasing at row {index}"
                ));
            }
            first_currents.get_or_insert(currents);
            previous_currents = Some(currents);
        }
        let (Some(first), Some(last)) = (first_currents, previous_currents) else {
            return Err(format!("{LABEL} {role:?} has no current observations"));
        };
        if first
            .into_iter()
            .zip(last)
            .any(|(first, last)| last / first < 1.0e5)
        {
            return Err(format!(
                "{LABEL} {role:?} currents lost their required five-decade dynamic range"
            ));
        }
        Ok(())
    }

    fn run_bug325_worker(
        &self,
        role: Bug325WorkerRole,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<Bug325Run, String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{LABEL} {role:?} execution failed: {error}"))?;
        let table = self
            .dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| format!("{LABEL} {role:?} PRN conversion failed: {error}"))?;
        if table.columns != ["Index", "v(bx)", "i(vib)", "i(vic)"]
            || table.rows.len() != 26
            || results.len() != table.rows.len()
        {
            return Err(format!("{LABEL} {role:?} output shape changed"));
        }
        let expected_names = match role {
            Bug325WorkerRole::ExplicitPair => &["q1", "q2"][..],
            Bug325WorkerRole::MultiplicityTwo | Bug325WorkerRole::SingleDeviceCounterfactual => {
                &["q1"][..]
            }
        };
        for (index, (row, result)) in table.rows.iter().zip(&results).enumerate() {
            let expected = 0.5 + 0.02 * index as Value;
            let entries = result
                .device_op_report
                .entries
                .iter()
                .filter(|entry| entry.device_kind.eq_ignore_ascii_case(GENERATED_MODEL))
                .collect::<Vec<_>>();
            if row.len() != 4
                || row.iter().any(|value| !value.is_finite())
                || Self::xyce_default_prn_roundtrip(row[0])?.to_bits() != (index as Value).to_bits()
                || Self::xyce_default_prn_roundtrip(row[1])?.to_bits()
                    != Self::xyce_default_prn_roundtrip(expected)?.to_bits()
                || result.sweep_value.to_bits() != expected.to_bits()
                || !result.device_op_report.labels_resolve()
                || entries.len() != role.expected_devices()
                || entries
                    .iter()
                    .map(|entry| entry.name.to_ascii_lowercase())
                    .ne(expected_names.iter().map(|name| name.to_string()))
                || entries.iter().any(|entry| {
                    entry.params.is_empty()
                        || entry.params.iter().any(|(_, value)| !value.is_finite())
                })
                || !result
                    .result
                    .node_voltages
                    .iter()
                    .chain(result.result.branch_currents.iter())
                    .all(|value| value.is_finite())
                || !result
                    .result
                    .dc_observables
                    .iter()
                    .all(|(_, value)| value.is_finite())
            {
                return Err(format!(
                    "{LABEL} {role:?} run/result linkage failed at row {index}"
                ));
            }
            let raw_vib = result
                .result
                .branch_current_named("vib")
                .ok_or_else(|| format!("{LABEL} {role:?} lost VIB branch at row {index}"))?;
            let raw_vic = result
                .result
                .branch_current_named("vic")
                .ok_or_else(|| format!("{LABEL} {role:?} lost VIC branch at row {index}"))?;
            let raw_bx = result
                .result
                .try_voltage_named("bx")
                .ok_or_else(|| format!("{LABEL} {role:?} lost BX node at row {index}"))?;
            if row[1].to_bits() != raw_bx.to_bits()
                || row[2].to_bits() != raw_vib.to_bits()
                || row[3].to_bits() != raw_vic.to_bits()
            {
                return Err(format!(
                    "{LABEL} {role:?} PRN table detached from raw solution at row {index}"
                ));
            }
            let mut generated_ib = 0.0;
            let mut generated_ic = 0.0;
            for entry in &entries {
                let current = |name: &str| {
                    entry.params.iter().find_map(|(candidate, value)| {
                        candidate.eq_ignore_ascii_case(name).then_some(*value)
                    })
                };
                let ic = current("ic")
                    .ok_or_else(|| format!("{LABEL} {role:?} lost generated IC at row {index}"))?;
                let ib = current("ib")
                    .ok_or_else(|| format!("{LABEL} {role:?} lost generated IB at row {index}"))?;
                let ie = current("ie")
                    .ok_or_else(|| format!("{LABEL} {role:?} lost generated IE at row {index}"))?;
                let scale = ic.abs().max(ib.abs()).max(ie.abs()).max(1.0e-30);
                if (ic + ib + ie).abs() > 1.0e-6 * scale + 1.0e-18 {
                    return Err(format!(
                        "{LABEL} {role:?} generated-device KCL failed at row {index}: ic={ic:e}, ib={ib:e}, ie={ie:e}, sum={:e}",
                        ic + ib + ie
                    ));
                }
                generated_ic += ic;
                generated_ib += ib;
            }
            for (source, generated, probe) in [
                (raw_vib, generated_ib, "I(VIB)"),
                (raw_vic, generated_ic, "I(VIC)"),
            ] {
                let scale = source.abs().max(generated.abs()).max(1.0e-30);
                if (source - generated).abs() > 1.0e-6 * scale + 1.0e-18 {
                    return Err(format!(
                        "{LABEL} {role:?} {probe} is detached from generated leads at row {index}: source={source:e}, generated={generated:e}"
                    ));
                }
            }
            if role == Bug325WorkerRole::ExplicitPair {
                for name in ["ic", "ib", "ie"] {
                    let first = entries[0]
                        .params
                        .iter()
                        .find_map(|(candidate, value)| {
                            candidate.eq_ignore_ascii_case(name).then_some(*value)
                        })
                        .ok_or_else(|| {
                            format!("{LABEL} explicit Q1 lost generated {name} at row {index}")
                        })?;
                    let second = entries[1]
                        .params
                        .iter()
                        .find_map(|(candidate, value)| {
                            candidate.eq_ignore_ascii_case(name).then_some(*value)
                        })
                        .ok_or_else(|| {
                            format!("{LABEL} explicit Q2 lost generated {name} at row {index}")
                        })?;
                    let scale = first.abs().max(second.abs()).max(1.0e-30);
                    if (first - second).abs() > 1.0e-7 * scale + 1.0e-18 {
                        return Err(format!(
                            "{LABEL} explicit Q1/Q2 {name} diverged at row {index}: {first:e} vs {second:e}"
                        ));
                    }
                }
            }
        }
        Self::validate_bug325_current_shape(role, &table)?;
        Ok(Bug325Run { table, results })
    }

    fn bug325_single_device_source(owner: &str) -> Result<String, String> {
        let mut removed = 0usize;
        let mut lines = Vec::new();
        for line in owner.lines() {
            let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();
            if tokens.len() == 5
                && tokens[0].eq_ignore_ascii_case("q2")
                && tokens[1].eq_ignore_ascii_case("c")
                && tokens[2].eq_ignore_ascii_case("b")
                && tokens[3] == "0"
                && tokens[4].eq_ignore_ascii_case("vbicmodel")
            {
                removed += 1;
            } else {
                lines.push(line);
            }
        }
        if removed != 1 {
            return Err(format!(
                "{LABEL} cannot derive the exact single-M1 counterfactual"
            ));
        }
        let mut source = lines.join("\n");
        source.push('\n');
        Ok(source)
    }

    fn validate_bug325_single_counterfactual(
        pair: &XycePrnTable,
        single: &XycePrnTable,
    ) -> Result<(), String> {
        if pair.columns != single.columns || pair.rows.len() != single.rows.len() {
            return Err(format!("{LABEL} single-M1 counterfactual shape changed"));
        }
        let mut nonzero = [false; 2];
        let mut varying = [false; 2];
        let mut previous: [Option<Value>; 2] = [None, None];
        for (index, (pair_row, single_row)) in pair.rows.iter().zip(&single.rows).enumerate() {
            for column in [0usize, 1] {
                if Self::xyce_default_prn_roundtrip(pair_row[column])?.to_bits()
                    != Self::xyce_default_prn_roundtrip(single_row[column])?.to_bits()
                {
                    return Err(format!("{LABEL} single-M1 grid differs at row {index}"));
                }
            }
            for column in [2usize, 3] {
                let expected = 2.0 * single_row[column];
                let scale = pair_row[column].abs().max(expected.abs()).max(1.0e-30);
                if (pair_row[column] - expected).abs() > 1.0e-4 * scale + 1.0e-18 {
                    return Err(format!(
                        "{LABEL} explicit pair is not twice a single M=1 device at row {index}, column {column}: pair={:e}, twice-single={expected:e}",
                        pair_row[column]
                    ));
                }
                let signal = column - 2;
                nonzero[signal] |= pair_row[column] != 0.0 || single_row[column] != 0.0;
                varying[signal] |= previous[signal]
                    .is_some_and(|value| value.to_bits() != pair_row[column].to_bits());
                previous[signal] = Some(pair_row[column]);
            }
        }
        if nonzero.iter().any(|value| !value) || varying.iter().any(|value| !value) {
            return Err(format!(
                "{LABEL} single-M1 counterfactual is vacuous (zero or invariant current)"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug325_son_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug325SonRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before validation"));
        }
        let members = self.validate_bug325_provenance(deck, role, &abort)?;
        let owner_bytes = members
            .get(&OWNER_NAME.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost owner"))?;
        let control_bytes = members
            .get(&CONTROL_NAME.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost M=2 control"))?;
        let owner = std::str::from_utf8(owner_bytes)
            .map_err(|error| format!("{LABEL} owner is not UTF-8: {error}"))?;
        let control = std::str::from_utf8(control_bytes)
            .map_err(|error| format!("{LABEL} M=2 control is not UTF-8: {error}"))?;
        let single = Self::bug325_single_device_source(owner)?;

        let owner_plan = self.validate_bug325_worker(
            Bug325WorkerRole::ExplicitPair,
            owner,
            &self.root.join(OWNER_PATH),
        )?;
        let control_plan = self.validate_bug325_worker(
            Bug325WorkerRole::MultiplicityTwo,
            control,
            &self.root.join(CONTROL_PATH),
        )?;
        let single_path = self
            .root
            .join(FAMILY_DIRECTORY)
            .join("__rspice_single_m1_counterfactual.cir");
        let single_plan = self.validate_bug325_worker(
            Bug325WorkerRole::SingleDeviceCounterfactual,
            &single,
            &single_path,
        )?;

        let owner_run =
            self.run_bug325_worker(Bug325WorkerRole::ExplicitPair, &owner_plan, start)?;
        let control_run =
            self.run_bug325_worker(Bug325WorkerRole::MultiplicityTwo, &control_plan, start)?;
        let single_run = self.run_bug325_worker(
            Bug325WorkerRole::SingleDeviceCounterfactual,
            &single_plan,
            start,
        )?;
        // Preserve the historical Release-7.10 direction: authored explicit pair is GOOD,
        // generated M=2 is TEST. The comparator's denominator is therefore owner-derived.
        let mismatches = self.compare_release_7_10_xyce_verify_dc_tables(
            LABEL,
            &owner_run.table,
            &control_run.table,
            &owner_run.results,
            &control_run.results,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} M=2 TEST differs from explicit-pair GOOD: {mismatches:?}"
            ));
        }
        Self::validate_bug325_single_counterfactual(&owner_run.table, &single_run.table)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded timeout"));
        }
        self.validate_bug325_provenance(deck, role, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} final provenance exceeded timeout"));
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
    fn bug325_roles_route_only_the_wrapper_owner() {
        assert_eq!(
            Bug325SonRole::for_record(OWNER_RECORD),
            Some(Bug325SonRole::WrapperOwner)
        );
        assert_eq!(Bug325SonRole::for_record(CONTROL_RECORD), None);
        assert_eq!(Bug325SonRole::WrapperOwner.contract(), OWNER_CONTRACT);
    }

    #[test]
    fn bug325_record_streams_are_exact() {
        XyceTestRunner::validate_bug325_record_streams().unwrap_or_else(|error| panic!("{error}"));
    }

    #[test]
    fn bug325_retained_census_uses_canonical_lf_and_rejects_content_drift() {
        let source_root = corpus_root().join(FAMILY_DIRECTORY);
        let temporary = tempfile::Builder::new()
            .prefix("rspice-xyce-bug325-census-")
            .tempdir()
            .expect("temporary directory");
        let retained = temporary.path().join(FAMILY_DIRECTORY);
        fs::create_dir_all(&retained).expect("create retained family directory");
        for name in [OWNER_NAME, CONTROL_NAME] {
            let canonical = fs::read(source_root.join(name)).expect("read canonical member");
            let canonical = XyceTestRunner::canonical_lf_text_identity(LABEL, &canonical)
                .expect("canonical source text");
            let crlf = String::from_utf8(canonical)
                .expect("source is UTF-8")
                .replace('\n', "\r\n");
            fs::write(retained.join(name), crlf).expect("write CRLF fixture");
        }
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        runner
            .read_bug325_directory(&abort)
            .expect("canonical-LF identity accepts checkout EOL conversion");
        let owner = retained.join(OWNER_NAME);
        let mut bytes = fs::read(&owner).expect("read owner fixture");
        let position = bytes
            .iter()
            .position(|byte| *byte == b'1')
            .expect("fixture contains mutable content");
        bytes[position] = b'2';
        fs::write(owner, bytes).expect("write drifted fixture");
        assert!(runner.read_bug325_directory(&abort).is_err());
    }

    #[test]
    fn bug325_topology_rejects_model_or_multiplicity_drift() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let path = root.join(OWNER_PATH);
        let source = fs::read_to_string(&path).expect("owner source exists");
        runner
            .validate_bug325_worker(Bug325WorkerRole::ExplicitPair, &source, &path)
            .expect("canonical owner validates");
        let wrong_level = source.replacen("level=11", "level=1", 1);
        assert!(
            runner
                .validate_bug325_worker(Bug325WorkerRole::ExplicitPair, &wrong_level, &path)
                .is_err()
        );
        let wrong_m = source.replacen("q1 c  b  0 vbicmodel", "q1 c b 0 vbicmodel M=2", 1);
        assert!(
            runner
                .validate_bug325_worker(Bug325WorkerRole::ExplicitPair, &wrong_m, &path)
                .is_err()
        );
    }

    #[test]
    fn bug325_single_counterfactual_is_meaningful_and_fail_closed() {
        let owner = XycePrnTable {
            columns: vec![
                "Index".into(),
                "V(bx)".into(),
                "I(vib)".into(),
                "I(vic)".into(),
            ],
            rows: vec![
                vec![0.0, 0.5, -2.0e-6, -4.0e-3],
                vec![1.0, 0.52, -4.0e-6, -8.0e-3],
            ],
        };
        let mut single = owner.clone();
        for row in &mut single.rows {
            row[2] *= 0.5;
            row[3] *= 0.5;
        }
        XyceTestRunner::validate_bug325_single_counterfactual(&owner, &single)
            .expect("two-to-one relation validates");
        single.rows[1][3] = owner.rows[1][3];
        assert!(XyceTestRunner::validate_bug325_single_counterfactual(&owner, &single).is_err());
        let zero = XycePrnTable {
            columns: owner.columns.clone(),
            rows: vec![vec![0.0, 0.5, 0.0, 0.0], vec![1.0, 0.52, 0.0, 0.0]],
        };
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let good_results = [0.5, 0.52]
            .into_iter()
            .map(|sweep_value| DcSweepPointResult {
                sweep_value,
                result: rspice_core::solver::SimulationResult::new(0, 0),
                device_op_report: rspice_core::circuit::DeviceOpReport::default(),
            })
            .collect::<Vec<_>>();
        assert!(
            runner
                .compare_release_7_10_xyce_verify_dc_tables(
                    LABEL,
                    &zero,
                    &zero,
                    &good_results,
                    &good_results,
                )
                .expect("identical zero tables are comparator-compatible")
                .is_empty(),
            "the historical comparator alone cannot establish non-vacuous multiplicity"
        );
        assert!(XyceTestRunner::validate_bug325_single_counterfactual(&zero, &zero).is_err());
        assert!(XyceTestRunner::validate_bug325_single_counterfactual(&owner, &owner).is_err());
    }

    #[test]
    fn bug325_current_shape_rejects_shared_sign_swap_monotonicity_and_range_errors() {
        let canonical = XycePrnTable {
            columns: vec![
                "Index".into(),
                "V(bx)".into(),
                "I(vib)".into(),
                "I(vic)".into(),
            ],
            rows: vec![
                vec![0.0, 0.5, 1.0e-9, 1.0e-8],
                vec![1.0, 0.7, 1.0e-6, 1.0e-5],
                vec![2.0, 1.0, 1.0e-3, 1.0e-2],
            ],
        };
        let validate = |table: &XycePrnTable| {
            XyceTestRunner::validate_bug325_current_shape(Bug325WorkerRole::ExplicitPair, table)
        };
        validate(&canonical).expect("canonical current shape validates");

        let mut negative = canonical.clone();
        negative.rows[0][2] = -negative.rows[0][2];
        assert!(validate(&negative).is_err());

        let mut swapped = canonical.clone();
        for row in &mut swapped.rows {
            row.swap(2, 3);
        }
        assert!(validate(&swapped).is_err());

        let mut nonmonotonic = canonical.clone();
        nonmonotonic.rows[1][3] = nonmonotonic.rows[0][3];
        assert!(validate(&nonmonotonic).is_err());

        let mut weak_growth = canonical;
        weak_growth.rows[2][2] = 1.0e-5;
        weak_growth.rows[2][3] = 1.0e-4;
        assert!(validate(&weak_growth).is_err());
    }

    #[test]
    fn bug325_deadline_fails_before_provenance_or_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = runner
            .discover_tests()
            .into_iter()
            .find(|deck| {
                XyceTestRunner::normalize_manifest_key(&deck.relative_path) == OWNER_RECORD
            })
            .expect("BUG325 owner is discoverable");
        let start = Instant::now()
            .checked_sub(Duration::from_millis(10))
            .expect("monotonic clock can represent expired start");
        let error = runner
            .validate_bug325_son_oracle(&deck, Bug325SonRole::WrapperOwner, start)
            .expect_err("expired deadline must fail closed");
        assert!(
            error.contains("deadline expired"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn bug325_provenance_reloads_wrapper_manifest_for_final_seal() {
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        let temporary = tempfile::Builder::new()
            .prefix("rspice-xyce-bug325-manifest-toctou-")
            .tempdir()
            .expect("temporary directory");
        let family = temporary.path().join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG325 family");
        for name in [OWNER_NAME, CONTROL_NAME] {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG325 member");
        }
        let manifest = temporary.path().join(HARNESS_MANIFEST_FILE);
        fs::write(
            &manifest,
            format!("{OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG325 harness manifest");
        fs::write(
            temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{CONTROL_PATH}\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("write BUG325 exclusion manifest");

        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: temporary.path().join(OWNER_PATH),
            relative_path: OWNER_PATH.to_string(),
            section: XyceDeckSection::Netlists,
        };
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        runner
            .validate_bug325_provenance(&deck, Bug325SonRole::WrapperOwner, &abort)
            .expect("canonical BUG325 provenance");

        fs::write(
            manifest,
            format!("{CONTROL_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("mutate wrapper ownership after runner construction");
        let error = runner
            .validate_bug325_provenance(&deck, Bug325SonRole::WrapperOwner, &abort)
            .expect_err("fresh provenance pass must detect wrapper-manifest drift");
        assert!(
            error.contains("wrapper ownership changed"),
            "unexpected error: {error}"
        );
    }

    #[cfg(feature = "veriloga-model-vbic13")]
    #[test]
    fn bug325_checked_in_owner_executes_full_oracle() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let path = root.join(OWNER_PATH);
        let result = runner.run_test(&path);
        assert!(result.passed, "BUG325 failed: {:?}", result.error);
        assert_eq!(result.contract, OWNER_CONTRACT);
        let control = runner.run_test(root.join(CONTROL_PATH));
        assert!(control.upstream_excluded);
    }
}
