use super::*;
use rspice_core::netlist::{BjtType, SaveSignal, SimulationOptions, SourceSpec};
use std::io::Read as _;

const LABEL: &str = "BUG_805 legacy BJT model-alias relation";
const FAMILY_PATH: &str = "Netlists/Certification_Tests/BUG_805";
const OUTPUT_PATH: &str = "OutputData/Certification_Tests/BUG_805";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_805/exclude";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_TAG: &str = "Release-7.10.0";

const HISTORICAL_RECORD_COUNT: usize = 15;
const HISTORICAL_RECORD_BYTES: usize = 3_526;
const HISTORICAL_RECORDS_SHA256: &str =
    "5e43a606aeae05ce29891a96c36219d4ec6b054113904b4bb2bb5f8c969b1ff8";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "e6b551b861c953123f45309f951553c9425e261f40431d7600a7e90df30a92de";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 15] = [
    (
        "Netlists/Certification_Tests/BUG_805/CMakeLists.txt",
        1_736,
        "a33a75338f9eca0a8d307799458dc3663810e26833a482c390d2471294380485",
        "47d1106aff4eb1031de4e3b0ed2f93fb2c491e196996151b986b41f4a2b92bcd",
    ),
    (
        "Netlists/Certification_Tests/BUG_805/Manifest.txt",
        101,
        "26480e549fb2b5f90712e1d38da4a64ae973736f640e9e2fdc25ea71a6565d6f",
        "1d8bc35045fa0a2b5d909e082603f53f2b49069699555d2917efab18e762e712",
    ),
    (
        "Netlists/Certification_Tests/BUG_805/README",
        1_182,
        "6db5c056cb4e766826c7100baddddd76bf46773b9b9aa3ccd1cd0cb60cbc0947",
        "1b8d043a3bf95cd5d133a84b44cdc7b277be793e960c6f2209d7fb41fa497d02",
    ),
    (
        "Netlists/Certification_Tests/BUG_805/bug_805.cir",
        2,
        "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070",
        "5896d7c81fa3a2eee0aa6139c752d40a1408b7e083aa940d1ece11d61d6c0e3e",
    ),
    (
        "Netlists/Certification_Tests/BUG_805/bug_805.cir.sh",
        2_912,
        "d658ed70afb94d7120a5ddcfc2180f707c37cabe55d1addf12c50a1994b04880",
        "c6775522a9e17488bba66bb1c48ee6d7df9ba09c00d7a7fa746784a8b8eba8a8",
    ),
    (
        "Netlists/Certification_Tests/BUG_805/colpitts_osc1.cir",
        2_855,
        "96a0e07bfca827438497877d950d1e02d412584eb5d48e37c7388eac73fc9e12",
        "1a6e60baa29cb510ba831eda0484521d0b2025e2595607480a3cbe986f23aca3",
    ),
    (
        "Netlists/Certification_Tests/BUG_805/colpitts_osc2.cir",
        2_765,
        "cd9744ca18fdc52116a27593ea11e5d8e67100e3e0f3a5cd8dfc0f100fd51094",
        "1fb5614e2e280b57a972a366ffc5e882f619de027022737538975ed149ba928e",
    ),
    (
        "Netlists/Certification_Tests/BUG_805/colpitts_osc3.cir",
        2_774,
        "93df6309f0665da18319bb06199aff30b2d724689967a86eeb28e3041a748d94",
        "c588a19b27feab271f835185cd4906367e33c9c91b1f5f20a8b22168d50036b4",
    ),
    (
        EXCLUSION_SOURCE,
        94,
        "93a52ce898e6b91ec5d5102cefc59c27b7fc915d60182cf0729b2ab552446979",
        "f81e64da212bbf3f1a53dd070aa2875403a0f21fabd5808ac5c6c8360274c2ca",
    ),
    (
        "Netlists/Certification_Tests/BUG_805/tags",
        30,
        "01f9975c946fe100a59fbc0a2861ba225e8e95a5775e90834aa1dc741bf1f64f",
        "be0f3f7cedfb378e713d55d643600a29db2c6e1e739628bdc926ebbb2963988c",
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

const RETAINED_RECORD_COUNT: usize = 5;
const RETAINED_RECORD_BYTES: usize = 744;
const RETAINED_RECORDS_SHA256: &str =
    "fb78e48952bbf76e47e4e4561a1bc9127f16491f3a83606d73540b6ee8bed16c";
const RETAINED_RECORDS_BLAKE3: &str =
    "297f2e85044ba3f41dea3ddb84327189a90baf8bb6ea5f8bd21a57269fc2cc81";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 5] = [
    (
        "README",
        1_182,
        "6db5c056cb4e766826c7100baddddd76bf46773b9b9aa3ccd1cd0cb60cbc0947",
        "1b8d043a3bf95cd5d133a84b44cdc7b277be793e960c6f2209d7fb41fa497d02",
    ),
    (
        "bug_805.cir",
        2,
        "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070",
        "5896d7c81fa3a2eee0aa6139c752d40a1408b7e083aa940d1ece11d61d6c0e3e",
    ),
    (
        "colpitts_osc1.cir",
        2_855,
        "96a0e07bfca827438497877d950d1e02d412584eb5d48e37c7388eac73fc9e12",
        "1a6e60baa29cb510ba831eda0484521d0b2025e2595607480a3cbe986f23aca3",
    ),
    (
        "colpitts_osc2.cir",
        2_765,
        "cd9744ca18fdc52116a27593ea11e5d8e67100e3e0f3a5cd8dfc0f100fd51094",
        "1fb5614e2e280b57a972a366ffc5e882f619de027022737538975ed149ba928e",
    ),
    (
        "colpitts_osc3.cir",
        2_774,
        "93df6309f0665da18319bb06199aff30b2d724689967a86eeb28e3041a748d94",
        "c588a19b27feab271f835185cd4906367e33c9c91b1f5f20a8b22168d50036b4",
    ),
];

const CANONICAL_MODEL_PARAMS: [(&str, Value); 40] = [
    ("IS", 3.97589e-14),
    ("BF", 195.3412),
    ("NF", 1.0040078),
    ("VAF", 53.081),
    ("IKF", 0.976),
    ("ISE", 1.60241e-14),
    ("NE", 1.4791931),
    ("BR", 1.1107942),
    ("NR", 0.9928261),
    ("VAR", 11.3571702),
    ("IKR", 2.4993953),
    ("ISC", 1.88505e-12),
    ("NC", 1.1838278),
    ("RB", 56.5826472),
    ("IRB", 1.50459e-4),
    ("RBM", 5.2592283),
    ("RE", 0.0402974),
    ("RC", 0.4208),
    ("CJE", 2.56e-11),
    ("VJE", 0.682256),
    ("MJE", 0.3358856),
    ("TF", 3.3e-10),
    ("XTF", 6.0),
    ("VTF", 0.574),
    ("ITF", 0.32),
    ("PTF", 25.832),
    ("CJC", 1.40625e-11),
    ("VJC", 0.5417393),
    ("MJC", 0.4547893),
    ("XCJC", 1.0),
    ("TR", 3.2e-7),
    ("CJS", 0.0),
    ("VJS", 0.75),
    ("MJS", 0.0),
    ("XTB", 1.6486),
    ("EG", 1.11),
    ("XTI", 5.8315),
    ("KF", 0.0),
    ("AF", 1.0),
    ("FC", 0.83),
];

type CapturedMembers = BTreeMap<String, Vec<u8>>;
type CanonicalModel = BTreeMap<String, u64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Bug805Representation {
    Owner,
    Canonical,
    PspiceAliases,
    HspiceAliases,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Bug805Role(Bug805Representation);

impl Bug805Role {
    const ALL: [Self; 4] = [
        Self(Bug805Representation::Owner),
        Self(Bug805Representation::Canonical),
        Self(Bug805Representation::PspiceAliases),
        Self(Bug805Representation::HspiceAliases),
    ];
    const WORKERS: [Self; 3] = [
        Self(Bug805Representation::Canonical),
        Self(Bug805Representation::PspiceAliases),
        Self(Bug805Representation::HspiceAliases),
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    fn file_name(self) -> &'static str {
        match self.0 {
            Bug805Representation::Owner => "bug_805.cir",
            Bug805Representation::Canonical => "colpitts_osc1.cir",
            Bug805Representation::PspiceAliases => "colpitts_osc2.cir",
            Bug805Representation::HspiceAliases => "colpitts_osc3.cir",
        }
    }

    fn path(self) -> String {
        format!("{FAMILY_PATH}/{}", self.file_name())
    }

    fn record(self) -> String {
        XyceTestRunner::normalize_manifest_key(&self.path())
    }

    pub(super) fn contract(self) -> &'static str {
        match self.0 {
            Bug805Representation::Owner => "bug805_bjt_model_alias_wrapper_owner",
            Bug805Representation::Canonical => "bug805_bjt_model_canonical_worker",
            Bug805Representation::PspiceAliases => "bug805_bjt_model_pspice_alias_worker",
            Bug805Representation::HspiceAliases => "bug805_bjt_model_hspice_alias_worker",
        }
    }

    fn raw_model_keys(self) -> &'static [&'static str] {
        const CANONICAL: [&str; 40] = [
            "IS", "BF", "NF", "VAF", "IKF", "ISE", "NE", "BR", "NR", "VAR", "IKR", "ISC", "NC",
            "RB", "IRB", "RBM", "RE", "RC", "CJE", "VJE", "MJE", "TF", "XTF", "VTF", "ITF", "PTF",
            "CJC", "VJC", "MJC", "XCJC", "TR", "CJS", "VJS", "MJS", "XTB", "EG", "XTI", "KF", "AF",
            "FC",
        ];
        const PSPICE: [&str; 40] = [
            "IS", "BFM", "NF", "VA", "IK", "JLE", "NLE", "BRM", "NR", "VB", "JBR", "JLC", "NC",
            "RB", "JRB", "RBM", "RE", "RC", "CJE", "PE", "ME", "TF", "XTF", "VTF", "JTF", "PTF",
            "CJC", "PC", "MC", "CDIS", "TR", "CCS", "PS", "MS", "TB", "EG", "PT", "KF", "AF", "FC",
        ];
        const HSPICE: [&str; 40] = [
            "IS", "BFM", "NF", "VBF", "JBF", "JLE", "NLE", "BRM", "NR", "VRB", "JBR", "JLC", "NC",
            "RB", "IOB", "RBM", "RE", "RC", "CJE", "PE", "ME", "TF", "XTF", "VTF", "JTF", "PTF",
            "CJC", "PC", "MC", "CDIS", "TR", "CSUB", "PSUB", "ESUB", "TCB", "EG", "PT", "KF", "AF",
            "FC",
        ];
        match self.0 {
            Bug805Representation::Canonical => &CANONICAL,
            Bug805Representation::PspiceAliases => &PSPICE,
            Bug805Representation::HspiceAliases => &HSPICE,
            Bug805Representation::Owner => &[],
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug805_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, blake3)| {
                format!("{UPSTREAM_COMMIT}\t{UPSTREAM_TAG}\t{path}\t{bytes}\t{sha256}\t{blake3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug805_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug805_historical_oracle_provenance_records();
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

    fn validate_bug805_directory(&self) -> Result<CapturedMembers, String> {
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

    fn validate_bug805_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug805Role,
    ) -> Result<CapturedMembers, String> {
        Self::validate_bug805_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} role is not at its canonical path"
            ));
        }

        let prefix = "netlists/certification_tests/bug_805/";
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(prefix))
            .collect::<BTreeSet<_>>();
        let expected_owners = [Bug805Role(Bug805Representation::Owner).record()]
            .into_iter()
            .collect::<BTreeSet<_>>();
        if owners != expected_owners {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let family_rows = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<BTreeMap<_, _>>();
        let expected_records = Bug805Role::WORKERS
            .into_iter()
            .map(Bug805Role::record)
            .collect::<BTreeSet<_>>();
        if family_rows
            .keys()
            .map(|record| record.to_string())
            .collect::<BTreeSet<_>>()
            != expected_records
        {
            return Err(format!("{LABEL} exclusion census changed"));
        }
        for worker in Bug805Role::WORKERS {
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
            Err(error) => {
                return Err(format!(
                    "failed to inspect {LABEL} OutputData family: {error}"
                ));
            }
        }
        self.validate_bug805_directory()
    }

    fn bug805_nodes_match(nodes: &[String], expected: &[&str]) -> bool {
        nodes.len() == expected.len()
            && nodes
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn bug805_canonical_param_name(name: &str) -> Option<&'static str> {
        Some(match name.to_ascii_uppercase().as_str() {
            "BF" | "BFM" => "BF",
            "VAF" | "VA" | "VBF" => "VAF",
            "IKF" | "IK" | "JBF" => "IKF",
            "ISE" | "JLE" => "ISE",
            "NE" | "NLE" => "NE",
            "BR" | "BRM" => "BR",
            "VAR" | "VB" | "VRB" => "VAR",
            "IKR" | "JBR" => "IKR",
            "ISC" | "JLC" => "ISC",
            "IRB" | "JRB" | "IOB" => "IRB",
            "VJE" | "PE" => "VJE",
            "MJE" | "ME" => "MJE",
            "ITF" | "JTF" => "ITF",
            "VJC" | "PC" => "VJC",
            "MJC" | "MC" => "MJC",
            "XCJC" | "CDIS" => "XCJC",
            "CJS" | "CCS" | "CSUB" => "CJS",
            "VJS" | "PS" | "PSUB" => "VJS",
            "MJS" | "MS" | "ESUB" => "MJS",
            "XTB" | "TB" | "TCB" => "XTB",
            "XTI" | "PT" => "XTI",
            "IS" => "IS",
            "NF" => "NF",
            "NR" => "NR",
            "NC" => "NC",
            "RB" => "RB",
            "RBM" => "RBM",
            "RE" => "RE",
            "RC" => "RC",
            "CJE" => "CJE",
            "TF" => "TF",
            "XTF" => "XTF",
            "VTF" => "VTF",
            "PTF" => "PTF",
            "CJC" => "CJC",
            "TR" => "TR",
            "EG" => "EG",
            "KF" => "KF",
            "AF" => "AF",
            "FC" => "FC",
            _ => return None,
        })
    }

    fn validate_bug805_model(
        role: Bug805Role,
        model: &rspice_core::netlist::ModelDef,
    ) -> Result<CanonicalModel, String> {
        if role.0 == Bug805Representation::Owner
            || !model.name.eq_ignore_ascii_case("Q222200")
            || !model.model_type.eq_ignore_ascii_case("NPN")
            || model.params.len() != CANONICAL_MODEL_PARAMS.len()
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} {} model shape changed", role.file_name()));
        }
        let raw_keys = model
            .params
            .iter()
            .map(|(name, _)| name.to_ascii_uppercase())
            .collect::<Vec<_>>();
        if raw_keys
            != role
                .raw_model_keys()
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>()
        {
            return Err(format!(
                "{LABEL} {} raw alias spellings changed: {raw_keys:?}",
                role.file_name()
            ));
        }
        let mut normalized = BTreeMap::new();
        for (name, value) in &model.params {
            let canonical = Self::bug805_canonical_param_name(name)
                .ok_or_else(|| format!("{LABEL} unrecognized model parameter {name:?}"))?;
            if !value.is_finite()
                || normalized
                    .insert(canonical.to_string(), value.to_bits())
                    .is_some()
            {
                return Err(format!(
                    "{LABEL} {} model aliases became ambiguous",
                    role.file_name()
                ));
            }
        }
        let expected = CANONICAL_MODEL_PARAMS
            .into_iter()
            .map(|(name, value)| (name.to_string(), value.to_bits()))
            .collect::<BTreeMap<_, _>>();
        if normalized != expected {
            return Err(format!(
                "{LABEL} {} normalized physical model changed",
                role.file_name()
            ));
        }
        Ok(normalized)
    }

    fn validate_bug805_options(options: &SimulationOptions) -> Result<(), String> {
        if options.device_voltage_limiting != Some(true)
            || options.device_debug_level != Some(0)
            || options.timeint_reltol.map(Value::to_bits) != Some(1.0e-4f64.to_bits())
            || options.timeint_abstol.map(Value::to_bits) != Some(1.0e-9f64.to_bits())
        {
            return Err(format!("{LABEL} active options changed: {options:?}"));
        }
        let mut remaining = options.clone();
        remaining.device_voltage_limiting = None;
        remaining.device_debug_level = None;
        remaining.timeint_reltol = None;
        remaining.timeint_abstol = None;
        if format!("{remaining:?}") != format!("{:?}", SimulationOptions::default()) {
            return Err(format!("{LABEL} acquired an extra option: {remaining:?}"));
        }
        Ok(())
    }

    fn validate_bug805_passive(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        value: Value,
        kind: char,
    ) -> Result<(), String> {
        let matches_kind = match kind {
            'R' => matches!(&element.kind, ElementKind::Resistor {
                value: actual, value_expr: None, model: None, instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty()),
            'C' => matches!(&element.kind, ElementKind::Capacitor {
                value: actual, value_expr: None, initial_voltage: None, model: None,
                instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty()),
            'L' => matches!(&element.kind, ElementKind::Inductor {
                value: actual, value_expr: None, initial_current: None, model: None,
                instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty()),
            _ => false,
        };
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || !Self::bug805_nodes_match(&element.nodes, &nodes)
            || !matches_kind
        {
            return Err(format!("{LABEL} passive {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_bug805_netlist(
        role: Bug805Role,
        netlist: &Netlist,
    ) -> Result<CanonicalModel, String> {
        if role.0 == Bug805Representation::Owner
            || netlist.title != "Colpitts oscilator"
            || netlist.elements.len() != 20
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
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
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !Self::netlist_is_native_bug805_bjt_envelope(netlist)
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        Self::validate_bug805_options(&netlist.options)?;
        let names = netlist
            .elements
            .iter()
            .map(|element| element.name.to_ascii_uppercase())
            .collect::<Vec<_>>();
        let expected_names = [
            "R6", "R5", "R4", "L1", "R7", "C20", "C8", "C10", "C9", "C2", "C3", "C7", "C6", "C5",
            "C4", "R16", "R15", "V2", "R17", "Q2",
        ];
        if names != expected_names {
            return Err(format!("{LABEL} authored element order changed: {names:?}"));
        }
        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        for (name, nodes, value) in [
            ("R6", ["0", "2"], 15.0 * 1.0e3),
            ("R5", ["0", "3"], 15.0 * 1.0e3),
            ("R4", ["0", "4"], 100.0 * 1.0e3),
            ("R7", ["1", "5"], 0.37),
            ("R16", ["0", "q2b"], 22.0 * 1.0e3),
            ("R15", ["q2b", "q2c"], 47.0 * 1.0e3),
            ("R17", ["q2e", "0"], 2.2 * 1.0e3),
        ] {
            Self::validate_bug805_passive(
                elements
                    .get(&name.to_ascii_lowercase())
                    .ok_or_else(|| format!("{LABEL} lost {name}"))?,
                name,
                nodes,
                value,
                'R',
            )?;
        }
        for (name, nodes, value) in [
            ("C20", ["4", "0"], 30.0 * 1.0e-12),
            ("C8", ["1", "4"], 82.0 * 1.0e-12),
            ("C10", ["3", "0"], 270.0 * 1.0e-12),
            ("C9", ["1", "3"], 10.0 * 1.0e-12),
            ("C2", ["2", "0"], 47.0 * 1.0e-12),
            ("C3", ["1", "2"], 10.0 * 1.0e-12),
            ("C7", ["1", "0"], 82.0 * 1.0e-12),
            ("C6", ["1", "q2b"], 3.3 * 1.0e-9),
            ("C5", ["q2e", "0"], 2.7 * 1.0e-9),
            ("C4", ["q2b", "q2e"], 2.7 * 1.0e-9),
        ] {
            Self::validate_bug805_passive(
                elements
                    .get(&name.to_ascii_lowercase())
                    .ok_or_else(|| format!("{LABEL} lost {name}"))?,
                name,
                nodes,
                value,
                'C',
            )?;
        }
        Self::validate_bug805_passive(
            elements
                .get("l1")
                .ok_or_else(|| format!("{LABEL} lost L1"))?,
            "L1",
            ["5", "0"],
            2.5 * 1.0e-6,
            'L',
        )?;
        let source = elements
            .get("v2")
            .ok_or_else(|| format!("{LABEL} lost V2"))?;
        let expected_points: [(Value, Value); 2] = [(0.0, 0.0), (5.0 * 1.0e-9, 7.4)];
        if source.provenance != ElementProvenance::Authored
            || !Self::bug805_nodes_match(&source.nodes, &["q2c", "0"])
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Pwl {
                points, delay, repeat_from,
            }) if points.len() == expected_points.len()
                && points.iter().zip(expected_points).all(|((time, value), (expected_time, expected_value))|
                    time.to_bits() == expected_time.to_bits()
                        && value.to_bits() == expected_value.to_bits())
                && delay.to_bits() == 0.0f64.to_bits()
                && repeat_from.is_none())
        {
            return Err(format!("{LABEL} source V2 changed: {source:?}"));
        }
        let transistor = elements
            .get("q2")
            .ok_or_else(|| format!("{LABEL} lost Q2"))?;
        if transistor.provenance != ElementProvenance::Authored
            || !Self::bug805_nodes_match(&transistor.nodes, &["q2c", "q2b", "q2e"])
            || !matches!(&transistor.kind, ElementKind::Bjt {
                model, bjt_type: BjtType::Npn, instance_params, deferred_params,
            } if model.eq_ignore_ascii_case("Q222200")
                && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} transistor Q2 changed: {transistor:?}"));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step, stop, start: None, max_step: None, uic: false,
        } if step.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == (80.0f64 * 1.0e-6).to_bits())
        {
            return Err(format!("{LABEL} transient analysis changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .zip(["q2b", "q2e"])
                .any(|(dependency, expected)| {
                    dependency.kind != OutputSymbolKind::Node
                        || !dependency.operator.eq_ignore_ascii_case("V")
                        || !dependency.symbol.eq_ignore_ascii_case(expected)
                        || dependency.expression
                })
        {
            return Err(format!("{LABEL} typed PRINT request changed: {request:?}"));
        }
        if !matches!(netlist.saves.signals.as_slice(),
            [SaveSignal::Voltage(left), SaveSignal::Voltage(right)]
                if left.eq_ignore_ascii_case("q2b") && right.eq_ignore_ascii_case("q2e"))
        {
            return Err(format!(
                "{LABEL} SaveSet changed: {:?}",
                netlist.saves.signals
            ));
        }
        Self::validate_bug805_model(role, &netlist.models[0])
    }

    fn bug805_plan(&self, role: Bug805Role) -> Result<XyceStaticTranPlan, String> {
        if role.0 == Bug805Representation::Owner {
            return Err(format!("{LABEL} owner is a wrapper anchor, not a worker"));
        }
        let plan = self.static_tran_plan_for_path_with_purpose(
            &self.root.join(role.path()),
            XyceStaticTranPlanPurpose::Bug805RelationalFamily,
        )?;
        let print = plan
            .print
            .as_ref()
            .ok_or_else(|| format!("{LABEL} {} lost PRINT", role.file_name()))?;
        if !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != (80.0f64 * 1.0e-6).to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || print.probes.len() != 2
            || !print.probes[0].eq_ignore_ascii_case("V(q2b)")
            || !print.probes[1].eq_ignore_ascii_case("V(q2e)")
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(plan)
    }

    fn validate_bug805_table(role: Bug805Role, table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(q2b)")
            || !table.columns[3].eq_ignore_ascii_case("V(q2e)")
            || table.rows.len() < 20
        {
            return Err(format!("{LABEL} {} output shape changed", role.file_name()));
        }
        let mut prior_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 4
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || prior_time.is_some_and(|prior| row[1] <= prior)
            {
                return Err(format!(
                    "{LABEL} {} output row {index} is malformed",
                    role.file_name()
                ));
            }
            prior_time = Some(row[1]);
        }
        let first = &table.rows[0];
        let last = table.rows.last().expect("BUG805 table is nonempty");
        let ranges = [2usize, 3].map(|column| {
            let minimum = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::INFINITY, Value::min);
            let maximum = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::NEG_INFINITY, Value::max);
            maximum - minimum
        });
        if first[1].to_bits() != 0.0f64.to_bits()
            || (last[1] - 80.0e-6).abs() > 1.0e-14
            || ranges.iter().any(|range| *range < 0.1)
        {
            return Err(format!(
                "{LABEL} {} output became vacuous",
                role.file_name()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug805_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug805Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let members = self.validate_bug805_provenance(deck, role)?;
        let mut tables = BTreeMap::new();
        let mut physical_model = None;
        // Preserve the Release wrapper's three independent-run order.
        for worker in Bug805Role::WORKERS {
            if abort.is_aborted() {
                return Err(format!("{LABEL} deadline expired between independent runs"));
            }
            let plan = self.bug805_plan(worker)?;
            let bytes = members
                .get(&worker.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", worker.file_name()))?;
            if plan.source.as_bytes() != bytes.as_slice() {
                return Err(format!(
                    "{LABEL} {} changed between reads",
                    worker.file_name()
                ));
            }
            let parsed = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
                .map_err(|error| format!("{LABEL} {} parse failed: {error}", worker.file_name()))?;
            let normalized = Self::validate_bug805_netlist(worker, &parsed)?;
            if physical_model
                .as_ref()
                .is_some_and(|expected| expected != &normalized)
            {
                return Err(format!(
                    "{LABEL} aliases no longer describe one physical model"
                ));
            }
            physical_model = Some(normalized);
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!("{LABEL} {} execution failed: {other}", worker.file_name()),
                })?;
            Self::validate_bug805_netlist(worker, &netlist)?;
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
            Self::validate_bug805_table(worker, &table)?;
            tables.insert(worker, table);
        }
        let canonical = tables
            .get(&Bug805Role(Bug805Representation::Canonical))
            .expect("canonical BUG805 worker ran");
        for worker in [
            Bug805Role(Bug805Representation::PspiceAliases),
            Bug805Role(Bug805Representation::HspiceAliases),
        ] {
            let mismatches = self.compare_serialized_default_prn_tables(
                canonical,
                tables.get(&worker).expect("all BUG805 workers ran"),
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {} default-PRN relation failed: {mismatches:?}",
                    worker.file_name()
                ));
            }
        }
        self.validate_bug805_provenance(deck, role)?;
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

    fn deck(root: &Path, role: Bug805Role) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug805-{label}-"))
            .tempdir()
            .expect("create BUG805 fixture");
        let root = temporary.path();
        let family = root.join(FAMILY_PATH);
        fs::create_dir_all(&family).expect("create BUG805 family");
        let canonical = corpus_root().join(FAMILY_PATH);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG805 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{FAMILY_PATH}/bug_805.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG805 wrapper manifest");
        let exclusions = Bug805Role::WORKERS
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
        )
        .expect("write BUG805 exclusion manifest");
        let role = Bug805Role(Bug805Representation::Owner);
        let owner = deck(root, role);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, owner, runner)
    }

    #[test]
    fn bug805_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug805_historical_oracle_provenance()
            .expect("BUG805 Release provenance remains exact");
    }

    #[test]
    fn bug805_worker_models_normalize_to_one_exact_physical_card() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let mut normalized = None;
        for role in Bug805Role::WORKERS {
            let plan = runner.bug805_plan(role).expect("canonical BUG805 plan");
            let netlist = XyceTestRunner::parse_xyce_netlist(&plan.source, &plan.deck_path)
                .expect("canonical BUG805 netlist");
            let model = XyceTestRunner::validate_bug805_netlist(role, &netlist)
                .expect("canonical BUG805 typed contract");
            assert!(
                normalized
                    .as_ref()
                    .is_none_or(|expected| expected == &model)
            );
            normalized = Some(model);
        }
    }

    #[test]
    fn bug805_owner_executes_all_three_independent_alias_workers() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let role = Bug805Role(Bug805Representation::Owner);
        runner
            .validate_bug805_oracle(&deck(&root, role), role, Instant::now())
            .expect("BUG805 native alias relation");
    }

    #[test]
    fn bug805_typed_and_numeric_counterfactuals_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let role = Bug805Role(Bug805Representation::PspiceAliases);
        let plan = runner.bug805_plan(role).expect("canonical BUG805 plan");
        let changed = plan.source.replacen("BFM = 195.3412", "BF = 195.3412", 1);
        let netlist = XyceTestRunner::parse_xyce_netlist(&changed, &plan.deck_path)
            .expect("mutated BUG805 model still parses");
        assert!(XyceTestRunner::validate_bug805_netlist(role, &netlist).is_err());

        let table = XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "TIME".to_string(),
                "V(q2b)".to_string(),
                "V(q2e)".to_string(),
            ],
            rows: vec![vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 1.0e-6, 1.0, 0.5]],
        };
        let mut wrong = table.clone();
        wrong.rows[1][2] += 1.0e-3;
        assert!(
            !runner
                .compare_serialized_default_prn_tables(&table, &wrong)
                .expect("counterfactual default-PRN comparison")
                .is_empty()
        );
    }

    #[test]
    fn bug805_provenance_mutations_fail_closed() {
        let (_temporary, owner, runner) = fixture("extra");
        runner
            .validate_bug805_provenance(&owner, Bug805Role(Bug805Representation::Owner))
            .expect("canonical BUG805 fixture");
        fs::write(runner.root.join(FAMILY_PATH).join("unexpected.cir"), "x\n")
            .expect("write extra BUG805 member");
        assert!(
            runner
                .validate_bug805_provenance(&owner, Bug805Role(Bug805Representation::Owner),)
                .is_err()
        );

        let (_temporary, owner, runner) = fixture("owner");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("mutate BUG805 owner");
        assert!(
            runner
                .validate_bug805_provenance(&owner, Bug805Role(Bug805Representation::Owner),)
                .is_err()
        );

        let (temporary, owner, runner) = fixture("exclusion");
        let path = temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let content = fs::read_to_string(&path).expect("read BUG805 exclusions");
        fs::write(
            &path,
            content.replacen(
                "bug805_bjt_model_pspice_alias_worker",
                "bug805_bjt_model_hspice_alias_worker",
                1,
            ),
        )
        .expect("mutate BUG805 qualification");
        assert!(
            runner
                .validate_bug805_provenance(&owner, Bug805Role(Bug805Representation::Owner),)
                .is_err()
        );

        let (temporary, owner, runner) = fixture("source");
        fs::write(
            temporary.path().join(FAMILY_PATH).join("colpitts_osc3.cir"),
            "changed\n",
        )
        .expect("mutate BUG805 source");
        assert!(
            runner
                .validate_bug805_provenance(&owner, Bug805Role(Bug805Representation::Owner),)
                .is_err()
        );

        let (_temporary, owner, runner) = fixture("output");
        fs::create_dir_all(runner.root.join(OUTPUT_PATH)).expect("invent BUG805 output");
        assert!(
            runner
                .validate_bug805_provenance(&owner, Bug805Role(Bug805Representation::Owner),)
                .is_err()
        );
    }

    #[test]
    fn bug805_expired_deadline_rejects_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let role = Bug805Role(Bug805Representation::Owner);
        assert!(
            runner
                .validate_bug805_oracle(
                    &deck(&root, role),
                    role,
                    Instant::now() - Duration::from_millis(10),
                )
                .is_err()
        );
    }
}
