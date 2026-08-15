use super::*;
use rspice_core::netlist::{OutputSymbolKind, SaveSignal, SourceSpec};
use std::io::Read as _;

const LABEL: &str = "BUG_805_SON include-partition mutual-inductor relation";
const FAMILY_PATH: &str = "Netlists/Certification_Tests/BUG_805_SON";
const OUTPUT_PATH: &str = "OutputData/Certification_Tests/BUG_805_SON";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_805_SON/exclude";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_TAG: &str = "Release-7.10.0";

const HISTORICAL_RECORD_COUNT: usize = 11;
const HISTORICAL_RECORD_BYTES: usize = 2_693;
const HISTORICAL_RECORDS_SHA256: &str =
    "113d6d97f014a8f62e8e68f0fa33302e87c78a78f94b7416d2a8f01c68ae584b";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "fa2634b80b55f4959d1ca09ffbec457980ef5909f091ab02b678861b55dd4e1f";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 11] = [
    (
        "Netlists/Certification_Tests/BUG_805_SON/CMakeLists.txt",
        2_319,
        "1cd4b64619d5bd5bfb570de08859242625612af889329feb0bb6c0471474ae55",
        "fa4e1b4fe60cd11c961b62853b5a3037afcd86df79e1382d0f08a6aa49daeffb",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/Manifest.txt",
        123,
        "3d9a3e31a63a187cd32ccb41a9b3cccf0234cb0b14d606328eb9d5d3bdc7504f",
        "1e777949e28a2b6094f31f6b4a9f69cf1fa4a293aeb5cb41251be2b2873d8830",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/bug805_all.cir",
        2_526,
        "548b435bc278fcd435174d1bc5e48b1d19d429c2e334f6d952cb4051e13cd4d5",
        "811f8cd715252dbc1df94ebe78068f488724f6896af4b836844ec8e7e9162273",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/bug805_all.cir.sh",
        1_932,
        "7b1833bc2323184bca2b30fdc3e17ab089268c2549beb0723d974d9d738943ba",
        "24b756bfcb69d7f4635f3207440473bdb4d349d572a4081eed8fc4ae30d03e22",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/bug805_top.cir",
        2_409,
        "e48b9bf7ed1ad7094dea5c335f8e12d319cdac3de7205a8f7d89e303de5457c8",
        "744f992372abc7b5900efbec3a85ff52f903fc99d16ad69b1fea85d0029d5cab",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/bug805_top2.cir",
        2_410,
        "1cf7c0ea588ac18530cc458e0ac29ec7947e406e1975b2bb11a50253f32358c0",
        "267cacebcc7f71bee529118857500a296a57051f2b26fc91b448c36316595372",
    ),
    (
        EXCLUSION_SOURCE,
        109,
        "e5e656095d4ee4c320aa753f064a1cedfd00107708b1dc5eaa59ecfd53650dcc",
        "059739b75e4ddbff3dfec9f5db5d066e8cbe5f15292956c6ede51798b6c3dece",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/mil_test1.lib",
        165,
        "973ceffe8e6dcc0460dcd43998cad8f8f385e9ecc8516545fc77034d95df9d9e",
        "c50a28b7e0ea28b6606b80e171b033e3c221aa95040559329fad26927d4246c4",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/mil_test2a.lib",
        194,
        "32d28cdd2e3cc498e85a6d6c017080b5d02dd35568fef6e986cbfb7a2c4b65a0",
        "4569f2e4fea9f1da518a52f8e13d907b79a1e46a3fb5378416af00bdf43b45cf",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/mil_test2b.lib",
        170,
        "23cb040cd52ac6bb8a608ca35c8bb7b444bc18f2df5d1ea6e21f47f2f2388d2d",
        "5593230d9efde2ddb9dd98203f9b93344a0dce0ef81a5ddaf534d025b6e5a3b4",
    ),
    (
        "Netlists/Certification_Tests/BUG_805_SON/tags",
        27,
        "4b8ea8000b121cc87df41e65f9ac43dece3eab840c84e864357b3c1b9021cdcb",
        "c3661f9aadf0946ebc823f672267a34f3f9485b8a002ff0a2ff956cf0c9dc430",
    ),
];

const RETAINED_RECORD_COUNT: usize = 6;
const RETAINED_RECORD_BYTES: usize = 896;
const RETAINED_RECORDS_SHA256: &str =
    "20a07efa1ceb31ac7ff11fda28e941f4762c72cf8d558b9c84cef87a1dd3864b";
const RETAINED_RECORDS_BLAKE3: &str =
    "c558213395b8e49ad204b61d253dbb15496b22eea56abdfd9d6f68cdb954fae5";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 6] = [
    (
        "bug805_all.cir",
        2_526,
        "548b435bc278fcd435174d1bc5e48b1d19d429c2e334f6d952cb4051e13cd4d5",
        "811f8cd715252dbc1df94ebe78068f488724f6896af4b836844ec8e7e9162273",
    ),
    (
        "bug805_top.cir",
        2_409,
        "e48b9bf7ed1ad7094dea5c335f8e12d319cdac3de7205a8f7d89e303de5457c8",
        "744f992372abc7b5900efbec3a85ff52f903fc99d16ad69b1fea85d0029d5cab",
    ),
    (
        "bug805_top2.cir",
        2_410,
        "1cf7c0ea588ac18530cc458e0ac29ec7947e406e1975b2bb11a50253f32358c0",
        "267cacebcc7f71bee529118857500a296a57051f2b26fc91b448c36316595372",
    ),
    (
        "mil_test1.lib",
        165,
        "973ceffe8e6dcc0460dcd43998cad8f8f385e9ecc8516545fc77034d95df9d9e",
        "c50a28b7e0ea28b6606b80e171b033e3c221aa95040559329fad26927d4246c4",
    ),
    (
        "mil_test2a.lib",
        194,
        "32d28cdd2e3cc498e85a6d6c017080b5d02dd35568fef6e986cbfb7a2c4b65a0",
        "4569f2e4fea9f1da518a52f8e13d907b79a1e46a3fb5378416af00bdf43b45cf",
    ),
    (
        "mil_test2b.lib",
        170,
        "23cb040cd52ac6bb8a608ca35c8bb7b444bc18f2df5d1ea6e21f47f2f2388d2d",
        "5593230d9efde2ddb9dd98203f9b93344a0dce0ef81a5ddaf534d025b6e5a3b4",
    ),
];

type CapturedMembers = BTreeMap<String, Vec<u8>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug805SonRole {
    InlineOwner,
    SingleInclude,
    NestedInclude,
}

impl Bug805SonRole {
    const ALL: [Self; 3] = [Self::InlineOwner, Self::SingleInclude, Self::NestedInclude];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    fn path(self) -> &'static str {
        match self {
            Self::InlineOwner => "Netlists/Certification_Tests/BUG_805_SON/bug805_all.cir",
            Self::SingleInclude => "Netlists/Certification_Tests/BUG_805_SON/bug805_top.cir",
            Self::NestedInclude => "Netlists/Certification_Tests/BUG_805_SON/bug805_top2.cir",
        }
    }

    fn record(self) -> String {
        XyceTestRunner::normalize_manifest_key(self.path())
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("BUG805SON path has a name")
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::InlineOwner => "bug805son_inline_wrapper_owner",
            Self::SingleInclude => "bug805son_single_include_worker",
            Self::NestedInclude => "bug805son_nested_include_worker",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug805_son_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, blake3)| {
                format!("{UPSTREAM_COMMIT}\t{UPSTREAM_TAG}\t{path}\t{bytes}\t{sha256}\t{blake3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug805_son_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug805_son_historical_oracle_provenance_records();
        let stream = records.join("\n");
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != HISTORICAL_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} Release provenance changed"));
        }
        Ok(())
    }

    fn validate_bug805_son_directory(&self) -> Result<CapturedMembers, String> {
        let directory = self.root.join(FAMILY_PATH);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = RETAINED_ARTIFACTS
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut members = BTreeMap::new();
        let mut records = Vec::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} directory: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect {LABEL}: {error}"))?;
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
            if members.contains_key(&key) {
                return Err(format!("{LABEL} contains a case collision for {name:?}"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name {
                return Err(format!("{LABEL} member case changed: {name:?}"));
            }
            let physical_cap = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| format!("{LABEL} retained-member bound overflow"))?;
            if metadata.len() > physical_cap as u64 {
                return Err(format!("{LABEL} member {name:?} is oversized"));
            }
            let file = fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} {name:?}: {error}"))?;
            let mut bytes = Vec::with_capacity(metadata.len() as usize);
            file.take((physical_cap + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} {name:?}: {error}"))?;
            if bytes.len() > physical_cap {
                return Err(format!("{LABEL} member {name:?} is oversized"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            if canonical.len() != expected_bytes
                || format!("{:x}", Sha256::digest(&canonical)) != expected_sha256
                || blake3::hash(&canonical).to_hex().as_str() != expected_blake3
            {
                return Err(format!("{LABEL} retained member {name:?} changed"));
            }
            records.push(format!(
                "{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
            members.insert(key, bytes);
        }
        if members.len() != expected.len() {
            return Err(format!("{LABEL} lost a retained member"));
        }
        records.sort();
        let stream = records.join("\n");
        if records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != RETAINED_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != RETAINED_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} retained census changed"));
        }
        Ok(members)
    }

    fn validate_bug805_son_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug805SonRole,
    ) -> Result<CapturedMembers, String> {
        Self::validate_bug805_son_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} role is not at its canonical path"
            ));
        }
        let prefix = "netlists/certification_tests/bug_805_son/";
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(prefix))
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([Bug805SonRole::InlineOwner.record()]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let family_rows = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<BTreeMap<_, _>>();
        let expected_workers = [Bug805SonRole::SingleInclude, Bug805SonRole::NestedInclude];
        let expected_records = expected_workers
            .into_iter()
            .map(Bug805SonRole::record)
            .collect::<BTreeSet<_>>();
        if family_rows
            .keys()
            .map(|record| record.to_string())
            .collect::<BTreeSet<_>>()
            != expected_records
        {
            return Err(format!("{LABEL} exclusion census changed"));
        }
        for worker in expected_workers {
            let row = family_rows
                .get(&worker.record())
                .ok_or_else(|| format!("{LABEL} lost {} exclusion row", worker.file_name()))?;
            if row.source != EXCLUSION_SOURCE
                || !matches!(&row.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == worker.contract())
            {
                return Err(format!(
                    "{LABEL} {} qualification changed",
                    worker.file_name()
                ));
            }
        }
        match fs::symlink_metadata(self.root.join(OUTPUT_PATH)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => return Err(format!("{LABEL} must not acquire an OutputData family")),
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
        }
        for member_role in Bug805SonRole::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member_role.path()))
                .map_err(|error| format!("{LABEL} {} {error}", member_role.file_name()))?;
        }
        self.validate_bug805_son_directory()
    }

    fn bug805_son_nodes_match(nodes: &[String], expected: &[&str]) -> bool {
        nodes.len() == expected.len()
            && nodes
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug805_son_passive(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        value: Value,
        inductor: bool,
    ) -> Result<(), String> {
        let kind_matches = if inductor {
            matches!(&element.kind, ElementKind::Inductor {
                value: actual, value_expr: None, initial_current: None, model: None,
                instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
        } else {
            matches!(&element.kind, ElementKind::Resistor {
                value: actual, value_expr: None, model: None, instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
        };
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || !Self::bug805_son_nodes_match(&element.nodes, &nodes)
            || !kind_matches
        {
            return Err(format!("{LABEL} passive {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_bug805_son_coupling(
        element: &rspice_core::netlist::Element,
        name: &str,
        right: &str,
        coefficient: Value,
    ) -> Result<(), String> {
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || !element.nodes.is_empty()
            || !matches!(&element.kind, ElementKind::Coupling { inductors, coefficient: actual, model: None }
                if actual.to_bits() == coefficient.to_bits()
                    && inductors.len() == 2
                    && inductors[0].eq_ignore_ascii_case("L1")
                    && inductors[1].eq_ignore_ascii_case(right))
        {
            return Err(format!("{LABEL} coupling {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn bug805_son_expected_probes() -> Vec<String> {
        ["i(vs)", "v(2)", "v(3)", "v(4)"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    fn validate_bug805_son_netlist(netlist: &Netlist) -> Result<(), String> {
        if netlist.elements.len() != 9
            || netlist.analyses.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !Self::analytic_timeint_only_options_match(&netlist.options, None, None, None, None)
        {
            return Err(format!("{LABEL} typed envelope changed"));
        }
        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from(["vs", "r1", "l1", "r2", "l2", "k1", "r3", "l3", "k2"])
        {
            return Err(format!("{LABEL} element inventory changed"));
        }
        let source = elements["vs"];
        if source.provenance != ElementProvenance::Authored
            || !Self::bug805_son_nodes_match(&source.nodes, &["1", "0"])
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                offset, amplitude, frequency, delay, damping, phase,
            }) if offset.to_bits() == 0.0f64.to_bits()
                && amplitude.to_bits() == 169.7f64.to_bits()
                && frequency.to_bits() == 60.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!("{LABEL} source changed: {source:?}"));
        }
        Self::validate_bug805_son_passive(elements["r1"], "R1", ["1", "2"], 1.0e3, false)?;
        Self::validate_bug805_son_passive(elements["l1"], "L1", ["2", "0"], 1.0e-3, true)?;
        Self::validate_bug805_son_passive(elements["r2"], "R2", ["3", "0"], 1.0e3, false)?;
        Self::validate_bug805_son_passive(elements["l2"], "L2", ["3", "0"], 1.0e-3, true)?;
        Self::validate_bug805_son_coupling(elements["k1"], "K1", "L2", 0.75)?;
        Self::validate_bug805_son_passive(elements["r3"], "R3", ["4", "0"], 1.0e3, false)?;
        Self::validate_bug805_son_passive(elements["l3"], "L3", ["4", "0"], 1.0e-3, true)?;
        Self::validate_bug805_son_coupling(elements["k2"], "K2", "L3", 0.8)?;

        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step, stop, start: None, max_step: None, uic: false,
        } if step.to_bits() == (100.0f64 * 1.0e-6).to_bits()
            && stop.to_bits() == (25.0e-3f64).to_bits())
        {
            return Err(format!("{LABEL} typed transient changed"));
        }
        let request = &netlist.output_requests[0];
        let expected_probes = Self::bug805_son_expected_probes();
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 4
            || request
                .dependencies
                .iter()
                .enumerate()
                .any(|(index, dependency)| {
                    dependency.expression
                        || if index == 0 {
                            dependency.kind != OutputSymbolKind::Device
                                || !dependency.operator.eq_ignore_ascii_case("I")
                                || !dependency.symbol.eq_ignore_ascii_case("VS")
                        } else {
                            dependency.kind != OutputSymbolKind::Node
                                || !dependency.operator.eq_ignore_ascii_case("V")
                                || dependency.symbol != (index + 1).to_string()
                        }
                })
        {
            return Err(format!("{LABEL} typed PRINT request changed: {request:?}"));
        }
        let saves = netlist
            .saves
            .signals
            .iter()
            .map(|signal| match signal {
                SaveSignal::Current(name) => format!("i({})", name.to_ascii_lowercase()),
                SaveSignal::Voltage(node) => format!("v({})", node.to_ascii_lowercase()),
                other => format!("{other:?}"),
            })
            .collect::<Vec<_>>();
        if saves != expected_probes {
            return Err(format!("{LABEL} SaveSet changed: {saves:?}"));
        }
        Ok(())
    }

    fn bug805_son_plan(&self, role: Bug805SonRole) -> Result<XyceStaticTranPlan, String> {
        let purpose = if role == Bug805SonRole::InlineOwner {
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
        } else {
            XyceStaticTranPlanPurpose::RelationalFamily
        };
        let plan =
            self.static_tran_plan_for_path_with_purpose(&self.root.join(role.path()), purpose)?;
        let print = plan
            .print
            .as_ref()
            .ok_or_else(|| format!("{LABEL} lost PRINT"))?;
        let expected_contract = if role == Bug805SonRole::InlineOwner {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        let probes = print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != (100.0f64 * 1.0e-6).to_bits()
            || plan.tran.stop.to_bits() != (25.0e-3f64).to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || probes != Self::bug805_son_expected_probes()
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(plan)
    }

    fn validate_bug805_son_table(table: &XycePrnTable) -> Result<(), String> {
        let expected = ["Index", "TIME", "I(VS)", "V(2)", "V(3)", "V(4)"];
        if table.columns.len() != expected.len()
            || table
                .columns
                .iter()
                .zip(expected)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() < 3
        {
            return Err(format!("{LABEL} output shape changed"));
        }
        let mut prior_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != expected.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || prior_time.is_some_and(|prior| row[1] <= prior)
            {
                return Err(format!("{LABEL} output row {index} is malformed"));
            }
            prior_time = Some(row[1]);
        }
        let first = &table.rows[0];
        let last = table.rows.last().expect("BUG805SON table is nonempty");
        let maxima = [2usize, 3, 4, 5].map(|column| {
            table
                .rows
                .iter()
                .map(|row| row[column].abs())
                .fold(0.0, Value::max)
        });
        let proportional_rows = table
            .rows
            .iter()
            .filter(|row| row[1] >= 1.0e-3)
            .filter(|row| row[4].abs().max(row[5].abs()) >= 1.0e-6)
            .collect::<Vec<_>>();
        let proportional = !proportional_rows.is_empty()
            && proportional_rows.iter().all(|row| {
                let residual = 0.8 * row[4] - 0.75 * row[5];
                residual.abs() <= 1.0e-8 + 1.0e-6 * row[4].abs().max(row[5].abs())
            });
        if first[1].to_bits() != 0.0f64.to_bits()
            || first[2..]
                .iter()
                .any(|value| value.to_bits() != 0.0f64.to_bits())
            || (last[1] - 25.0e-3).abs() > 1.0e-14
            || !(0.14..0.20).contains(&maxima[0])
            || !(0.04..0.09).contains(&maxima[1])
            || !(0.03..0.07).contains(&maxima[2])
            || !(0.03..0.08).contains(&maxima[3])
            || !proportional
        {
            return Err(format!(
                "{LABEL} output became vacuous or lost coupling: maxima={maxima:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug805_son_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug805SonRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let members = self.validate_bug805_son_provenance(deck, role)?;
        let mut tables = BTreeMap::new();
        for member_role in Bug805SonRole::ALL {
            if abort.is_aborted() {
                return Err(format!("{LABEL} deadline expired between independent runs"));
            }
            let plan = self.bug805_son_plan(member_role)?;
            let source = members
                .get(&member_role.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", member_role.file_name()))?;
            if plan.source.as_bytes() != source.as_slice() {
                return Err(format!(
                    "{LABEL} {} changed between reads",
                    member_role.file_name()
                ));
            }
            let parsed =
                Self::parse_xyce_netlist(&plan.source, &plan.deck_path).map_err(|error| {
                    format!("{LABEL} {} parse failed: {error}", member_role.file_name())
                })?;
            Self::validate_bug805_son_netlist(&parsed)?;
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!(
                        "{LABEL} {} execution failed: {other}",
                        member_role.file_name()
                    ),
                })?;
            Self::validate_bug805_son_netlist(&netlist)?;
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
            Self::validate_bug805_son_table(&table)?;
            tables.insert(member_role, table);
        }
        let owner = tables
            .get(&Bug805SonRole::InlineOwner)
            .expect("BUG805SON owner ran");
        for worker in [Bug805SonRole::SingleInclude, Bug805SonRole::NestedInclude] {
            let mismatches = self.compare_serialized_default_prn_tables(
                owner,
                tables.get(&worker).expect("BUG805SON worker ran"),
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {} default-PRN relation failed: {mismatches:?}",
                    worker.file_name()
                ));
            }
        }
        self.validate_bug805_son_provenance(deck, role)?;
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

    fn deck(root: &Path, role: Bug805SonRole) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path().to_string(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug805son-{label}-"))
            .tempdir()
            .expect("create BUG805SON fixture");
        let root = temporary.path();
        let family = root.join(FAMILY_PATH);
        fs::create_dir_all(&family).expect("create BUG805SON family");
        let canonical = corpus_root().join(FAMILY_PATH);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG805SON member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{FAMILY_PATH}/bug805_all.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG805SON wrapper manifest");
        let exclusions = [Bug805SonRole::SingleInclude, Bug805SonRole::NestedInclude]
            .into_iter()
            .map(|role| {
                format!(
                    "{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{}",
                    role.path(),
                    role.contract()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{exclusions}\n"
            ),
        ).expect("write BUG805SON exclusion manifest");
        let owner = deck(root, Bug805SonRole::InlineOwner);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, owner, runner)
    }

    #[test]
    fn bug805_son_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug805_son_historical_oracle_provenance()
            .expect("BUG805SON Release provenance remains exact");
    }

    #[test]
    fn bug805_son_all_roles_execute_the_include_relation() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug805SonRole::ALL {
            runner
                .validate_bug805_son_oracle(&deck(&root, role), role, Instant::now())
                .expect("BUG805SON native include relation");
        }
    }

    #[test]
    fn bug805_son_typed_and_numeric_counterfactuals_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let plan = runner
            .bug805_son_plan(Bug805SonRole::InlineOwner)
            .expect("canonical BUG805SON plan");
        let changed = plan.source.replacen("K2 L1 L3 0.8", "K2 L1 L3 0.7", 1);
        let netlist = XyceTestRunner::parse_xyce_netlist(&changed, &plan.deck_path)
            .expect("mutated BUG805SON still parses");
        assert!(XyceTestRunner::validate_bug805_son_netlist(&netlist).is_err());

        let table = XycePrnTable {
            columns: ["Index", "TIME", "I(VS)", "V(2)", "V(3)", "V(4)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                vec![1.0, 1.0e-3, -0.16, 0.06, 0.045, 0.048],
                vec![2.0, 25.0e-3, 0.16, -0.06, -0.045, -0.048],
            ],
        };
        XyceTestRunner::validate_bug805_son_table(&table).expect("coupled table passes");
        let mut uncoupled = table.clone();
        for row in &mut uncoupled.rows {
            row[4] = 0.0;
            row[5] = 0.0;
        }
        assert!(XyceTestRunner::validate_bug805_son_table(&uncoupled).is_err());
        let mut unequal = table.clone();
        unequal.rows[1][4] += 1.0e-3;
        assert!(
            !runner
                .compare_serialized_default_prn_tables(&table, &unequal)
                .expect("counterfactual PRN compare")
                .is_empty()
        );
    }

    #[test]
    fn bug805_son_provenance_mutations_fail_closed() {
        let (_temporary, owner, runner) = fixture("extra");
        runner
            .validate_bug805_son_provenance(&owner, Bug805SonRole::InlineOwner)
            .expect("canonical BUG805SON fixture");
        fs::write(runner.root.join(FAMILY_PATH).join("unexpected.cir"), "x\n")
            .expect("write extra BUG805SON member");
        assert!(
            runner
                .validate_bug805_son_provenance(&owner, Bug805SonRole::InlineOwner)
                .is_err()
        );

        let (_temporary, owner, runner) = fixture("owner");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("mutate BUG805SON owner");
        assert!(
            runner
                .validate_bug805_son_provenance(&owner, Bug805SonRole::InlineOwner)
                .is_err()
        );

        let (temporary, owner, runner) = fixture("exclusion");
        let path = temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let content = fs::read_to_string(&path).expect("read BUG805SON exclusions");
        fs::write(
            &path,
            content.replacen(
                "bug805son_single_include_worker",
                "bug805son_nested_include_worker",
                1,
            ),
        )
        .expect("mutate BUG805SON qualification");
        assert!(
            runner
                .validate_bug805_son_provenance(&owner, Bug805SonRole::InlineOwner)
                .is_err()
        );

        let (temporary, owner, runner) = fixture("source");
        fs::write(
            temporary.path().join(FAMILY_PATH).join("mil_test2b.lib"),
            "changed\n",
        )
        .expect("mutate BUG805SON include");
        assert!(
            runner
                .validate_bug805_son_provenance(&owner, Bug805SonRole::InlineOwner)
                .is_err()
        );

        let (_temporary, owner, runner) = fixture("output");
        fs::create_dir_all(runner.root.join(OUTPUT_PATH)).expect("invent BUG805SON output");
        assert!(
            runner
                .validate_bug805_son_provenance(&owner, Bug805SonRole::InlineOwner)
                .is_err()
        );
    }

    #[test]
    fn bug805_son_expired_deadline_rejects_before_execution() {
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
                .validate_bug805_son_oracle(
                    &deck(&root, Bug805SonRole::InlineOwner),
                    Bug805SonRole::InlineOwner,
                    Instant::now() - Duration::from_millis(10),
                )
                .is_err()
        );
    }
}
