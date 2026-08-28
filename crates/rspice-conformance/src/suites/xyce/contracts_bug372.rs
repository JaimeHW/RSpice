use super::*;
use rspice_core::netlist::SourceSpec;
use std::io::Read as _;

const LABEL: &str = "BUG_372 native MOS multiplicity wrapper family";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_372";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_372";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_372/";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_372/exclude";
const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const TOOLS_PM_SHA256: &str = "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3";
const XYCE_VERIFY_SHA256: &str = "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3";

const HISTORICAL_CONTENT_BYTES: usize = 227_245;
const HISTORICAL_STREAM_BYTES: usize = 8_062;
const HISTORICAL_STREAM_SHA256: &str =
    "6871159f953af2b27f0b78dc849e787e51e5cbce404944e51913bbcccc724fe5";
const HISTORICAL_STREAM_BLAKE3: &str =
    "752eb2930dd93cdc7b22be84b73b997a34ee31a475e173478f7eee1375b52b51";
const HISTORICAL: [(&str, usize, &str); 34] = [
    (
        "Netlists/Certification_Tests/BUG_372/CMakeLists.txt",
        11_511,
        "044387701642c758bde53b950a5bf0f42540b249",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/Manifest.txt",
        495,
        "0805ce9c1201e2cd913ee61e57abcf2ba6a9f9f5",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/README",
        1_944,
        "8cc6098b4c5f9bf4badf91b33e4fec6b80b30bb1",
    ),
    (
        EXCLUSION_SOURCE,
        200,
        "b79c3c059a8cec7a170e79a3109f54ab008260ef",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert1-sim.cir",
        1_142,
        "8ce547a449b93abeb717564824dc015acbf71b69",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert1.cir",
        1_343,
        "70722b238a90216b36e16b2b9e01e132c6da86ca",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert1.cir.sh",
        1_469,
        "70ab8b10198c2c68d51a5be8879146dcbae549c5",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert1.cir.tags",
        32,
        "2d8d97e134be2abca179261035726ff3d81fa617",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert2-sim.cir",
        1_172,
        "5466b4a46bd5b59e9945f8b8ad41f018ad2bc8c5",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert2.cir",
        1_342,
        "2797a3c9ed8d3d2e7acbceb6bf84ca1f9c4e2daa",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert2.cir.sh",
        1_469,
        "fc9f6954ad259a940ce6c621e93ff53fb14998c7",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert2.cir.tags",
        34,
        "241fd8405014fb89a125fed892e7927516ba3446",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert3-sim.cir",
        1_766,
        "2d08cd32b6bf265152c0e897924a42e01d15604b",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert3.cir",
        1_956,
        "1b9c6267bee6a3935fd3560289e53e86733d3f72",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert3.cir.sh",
        1_469,
        "0665d462a8477e4e83eee83dc4d44be2a1e43d01",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert3.cir.tags",
        32,
        "5ad6845e8575a8b5bc7d18860c36abffe0cbf98e",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert6-sim.cir",
        1_142,
        "985a1ecaa07f63f2ce91ec231d937c3089c2af85",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert6.cir",
        1_343,
        "2b775627a8c8b8aa465fcaaa99423260a9416499",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert6.cir.sh",
        1_469,
        "813066d9fe34f570a3fd3a492d9ca0a6385104dc",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert6.cir.tags",
        32,
        "080114fac76e4562197d2890c677b5c7477800dc",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_b3soi-sim.cir",
        6_133,
        "237bda074f7c32c81ad1a567dae12e3a55956eff",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_b3soi.cir",
        6_311,
        "cb72a99ecf82346476ee63bd1f36da123cb3d36e",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_b3soi.cir.sh",
        1_518,
        "086b913734e4029d77774c68e1034a4e23b2f0b5",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_b3soi.cir.tags",
        33,
        "7ddb7342c336c72e1ddb34f1f31f53ad0df8316e",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_bsim3-sim.cir",
        5_328,
        "27f2b9f0af18db322f1f1fd815f62020fed31d2e",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_bsim3.cir",
        9_801,
        "d45fd56234141fb68ab47b6931339dfe55aeca7e",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_bsim3.cir.sh",
        1_520,
        "43ab598659c385a0d5a0c39d2e9ffe78bea2ca9a",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_bsim3.cir.tags",
        33,
        "d4a8f52a6ac8b34a0502b895bd424d495806cff7",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_bsim4-sim.cir",
        14_663,
        "41549f128ac4856fb27787a0ca611093a8b70a26",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_bsim4.cir",
        21_333,
        "40b8e02ac9fbb59f993f8152f0b4bc16c2b6cb9e",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_bsim4.cir.sh",
        1_503,
        "782c3a1a802fd82d2b2bf4fc69ef29bb63103d6a",
    ),
    (
        "Netlists/Certification_Tests/BUG_372/invert_bsim4.cir.tags",
        33,
        "183a2fec28272c27367dc1aeb20d49a2feedbcb1",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        TOOLS_PM_SHA256,
    ),
    ("TestScripts/xyce_verify.pl", 59_566, XYCE_VERIFY_SHA256),
];

const RETAINED: [(&str, usize, &str); 15] = [
    (
        "README",
        1_944,
        "d790ca9c0cd976d657dbbc963d182eb12d9d3895067606e3089d79a26ae91a41",
    ),
    (
        "invert1-sim.cir",
        1_142,
        "0210d74d502c9084594b83bd66eafe760c0c01f155963b631744da6d74ddad64",
    ),
    (
        "invert1.cir",
        1_343,
        "f3593fcfdb325c2bae6230c4a26eb170d3da9cbda58ed555779c57c53b284f7d",
    ),
    (
        "invert2-sim.cir",
        1_172,
        "ec8e9d8c6fef6f1c3c88e67721a4d892ae7802fb58275b5ab86b31977536d199",
    ),
    (
        "invert2.cir",
        1_342,
        "743f9929ad0aeff7ec56707f8f9899158720da39935f44550555b8a1a99350ae",
    ),
    (
        "invert3-sim.cir",
        1_766,
        "afc1d50a80f0e6e63d3bf15f42c357fccba96733c55168447e9216fb85530422",
    ),
    (
        "invert3.cir",
        1_956,
        "30281ab916000c3ce76b4525c51ea7076eda38ed70d274016efd6232690c42ba",
    ),
    (
        "invert6-sim.cir",
        1_142,
        "680a248a389bd87011fe179a19bb450e587e4914289593e348d64a8f25dc57c9",
    ),
    (
        "invert6.cir",
        1_343,
        "dcc28c094a64526ca7c2a8d2d8bf33ebe57786b8c50439d51213712c07b33e1c",
    ),
    (
        "invert_b3soi-sim.cir",
        6_133,
        "cae0d5c9953ed3b5989af9356507becd2816fb6d737b40fc8a860c0c406446dd",
    ),
    (
        "invert_b3soi.cir",
        6_311,
        "3b0a3ed9c34a651e65469c5cf37876625e00797a8ab89c6da92ab2ce9166b02f",
    ),
    (
        "invert_bsim3-sim.cir",
        5_328,
        "49d082d1fe93e8aecd4a9b86b88d2376b5084d92929dce9a3db3c5eaa5831992",
    ),
    (
        "invert_bsim3.cir",
        9_801,
        "66ab668eacacf9b5c5b14453ddb8524418346990a2d4c65388e81973ff420eeb",
    ),
    (
        "invert_bsim4-sim.cir",
        14_663,
        "7f3617e63d297f3a1695f9494b6d8cf4eaa0d38fc12e9d23b292ca67dc2b84e1",
    ),
    (
        "invert_bsim4.cir",
        21_333,
        "15cc01d3463521a9d249c7817ae3123ff16b3378b48fa3ec81c2d9def6027358",
    ),
];
const RETAINED_STREAM_BYTES: usize = 1_839;
const RETAINED_STREAM_SHA256: &str =
    "cbd1f3982b37afe69ad86db788fc1c472dc1c17d0d89c865b1531663ad1736b9";
const RETAINED_STREAM_BLAKE3: &str =
    "01cc93fe07522cf930596c69a29e860d421f9caf4dc745276680ef01ea8dcf23";

const QUALIFIED: [Bug372Role; 4] = [
    Bug372Role::Level2,
    Bug372Role::Level3,
    Bug372Role::Bsim3,
    Bug372Role::Bsim4,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug372Role {
    Level2,
    Level3,
    Bsim3,
    Bsim4,
}

impl Bug372Role {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(record).as_str() {
            "netlists/certification_tests/bug_372/invert2.cir" => Some(Self::Level2),
            "netlists/certification_tests/bug_372/invert3.cir" => Some(Self::Level3),
            "netlists/certification_tests/bug_372/invert_bsim3.cir" => Some(Self::Bsim3),
            "netlists/certification_tests/bug_372/invert_bsim4.cir" => Some(Self::Bsim4),
            _ => None,
        }
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::Level2 => "bug372_level2_multiplicity_wrapper_owner",
            Self::Level3 => "bug372_level3_multiplicity_wrapper_owner",
            Self::Bsim3 => "bug372_bsim3_multiplicity_wrapper_owner",
            Self::Bsim4 => "bug372_bsim4_multiplicity_wrapper_owner",
        }
    }

    const fn owner_name(self) -> &'static str {
        match self {
            Self::Level2 => "invert2.cir",
            Self::Level3 => "invert3.cir",
            Self::Bsim3 => "invert_bsim3.cir",
            Self::Bsim4 => "invert_bsim4.cir",
        }
    }

    const fn control_name(self) -> &'static str {
        match self {
            Self::Level2 => "invert2-sim.cir",
            Self::Level3 => "invert3-sim.cir",
            Self::Bsim3 => "invert_bsim3-sim.cir",
            Self::Bsim4 => "invert_bsim4-sim.cir",
        }
    }

    fn owner_path(self) -> String {
        format!("{FAMILY_DIRECTORY}/{}", self.owner_name())
    }

    fn control_path(self) -> String {
        format!("{FAMILY_DIRECTORY}/{}", self.control_name())
    }

    fn owner_record(self) -> String {
        XyceTestRunner::normalize_manifest_key(&self.owner_path())
    }

    const fn multiplicity(self) -> usize {
        match self {
            Self::Level2 | Self::Level3 => 2,
            Self::Bsim3 | Self::Bsim4 => 32,
        }
    }

    const fn model_level(self) -> u8 {
        match self {
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Bsim3 => 9,
            Self::Bsim4 => 14,
        }
    }

    const fn print_precision(self) -> usize {
        match self {
            Self::Level2 | Self::Level3 => XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            Self::Bsim3 | Self::Bsim4 => 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug372Representation {
    ExplicitParallel,
    Multiplicity,
    UnitMultiplicityCounterfactual,
}

impl Bug372Representation {
    const fn expected_m(self, role: Bug372Role) -> Option<Value> {
        match self {
            Self::ExplicitParallel => None,
            Self::Multiplicity => Some(role.multiplicity() as Value),
            Self::UnitMultiplicityCounterfactual => Some(1.0),
        }
    }

    const fn expected_mos(self, role: Bug372Role) -> usize {
        match self {
            Self::ExplicitParallel => 2 * role.multiplicity(),
            Self::Multiplicity | Self::UnitMultiplicityCounterfactual => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct Bug372Worker {
    plan: XyceStaticTranPlan,
    netlist: Netlist,
    representation: Bug372Representation,
}

#[derive(Debug)]
struct Bug372Run {
    table: XycePrnTable,
    result: TransientResult,
}

impl XyceTestRunner {
    fn validate_bug372_historical_provenance() -> Result<(), String> {
        let family_records = HISTORICAL
            .iter()
            .filter(|record| {
                record
                    .0
                    .starts_with("Netlists/Certification_Tests/BUG_372/")
            })
            .collect::<Vec<_>>();
        let unique = HISTORICAL
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let wrappers = family_records
            .iter()
            .filter(|record| record.0.ends_with(".cir.sh"))
            .count();
        let tags = family_records
            .iter()
            .filter(|record| record.0.ends_with(".cir.tags"))
            .count();
        let decks = family_records
            .iter()
            .filter(|record| record.0.ends_with(".cir"))
            .count();
        let bytes = HISTORICAL.iter().map(|record| record.1).sum::<usize>();
        let mut historical_stream = HISTORICAL
            .iter()
            .map(|(path, bytes, identity)| {
                format!(
                    "{PRETRIM_COMMIT}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{RELEASE_TAG_OBJECT}\t{path}\t{bytes}\t{identity}"
                )
            })
            .collect::<Vec<_>>();
        historical_stream.sort();
        let historical_stream = historical_stream.join("\n");
        let historical_sha = format!("{:x}", Sha256::digest(historical_stream.as_bytes()));
        let historical_b3 = blake3::hash(historical_stream.as_bytes())
            .to_hex()
            .to_string();
        let mut retained_stream = RETAINED
            .iter()
            .map(|(name, bytes, sha256)| format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha256}"))
            .collect::<Vec<_>>();
        retained_stream.sort();
        let retained_stream = retained_stream.join("\n");
        let retained_sha = format!("{:x}", Sha256::digest(retained_stream.as_bytes()));
        let retained_b3 = blake3::hash(retained_stream.as_bytes())
            .to_hex()
            .to_string();
        let identities_are_exact = HISTORICAL.iter().all(|(path, _, identity)| {
            let expected = if path.starts_with("TestScripts/") {
                64
            } else {
                40
            };
            identity.len() == expected && identity.bytes().all(|byte| byte.is_ascii_hexdigit())
        });
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || RELEASE_TAG_OBJECT.len() != 40
            || UPSTREAM_COMMIT.len() != 40
            || RELEASE_TAG != "Release-7.10.0"
            || HISTORICAL.len() != 34
            || QUALIFIED.len() != 4
            || family_records.len() != 32
            || wrappers != 7
            || tags != 7
            || decks != 14
            || unique.len() != HISTORICAL.len()
            || bytes != HISTORICAL_CONTENT_BYTES
            || historical_stream.len() != HISTORICAL_STREAM_BYTES
            || historical_sha != HISTORICAL_STREAM_SHA256
            || historical_b3 != HISTORICAL_STREAM_BLAKE3
            || retained_stream.len() != RETAINED_STREAM_BYTES
            || retained_sha != RETAINED_STREAM_SHA256
            || retained_b3 != RETAINED_STREAM_BLAKE3
            || !identities_are_exact
            || HISTORICAL
                .iter()
                .any(|record| record.0.contains("OutputData"))
        {
            return Err(format!(
                "{LABEL} Release provenance changed: records={}, family={}, wrappers={wrappers}, tags={tags}, decks={decks}, content_bytes={bytes}, unique={}, historical_stream={}/sha={historical_sha}/b3={historical_b3}, retained_stream={}/sha={retained_sha}/b3={retained_b3}",
                HISTORICAL.len(),
                family_records.len(),
                unique.len(),
                historical_stream.len(),
                retained_stream.len(),
            ));
        }
        Ok(())
    }

    fn read_bug372_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug372_historical_provenance()?;
        let directory = self.root.join(FAMILY_DIRECTORY);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} source must be a regular non-symlink directory"
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
            let Some((expected_name, expected_bytes, expected_sha)) = expected.get(&key).copied()
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
            if canonical.len() != expected_bytes || sha != expected_sha {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha={sha}",
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

    fn bug372_all_owner_records() -> BTreeSet<String> {
        [
            "invert1.cir",
            "invert2.cir",
            "invert3.cir",
            "invert6.cir",
            "invert_b3soi.cir",
            "invert_bsim3.cir",
            "invert_bsim4.cir",
        ]
        .into_iter()
        .map(|name| Self::normalize_manifest_key(&format!("{FAMILY_DIRECTORY}/{name}")))
        .collect()
    }

    fn bug372_all_control_records() -> BTreeSet<String> {
        [
            "invert1-sim.cir",
            "invert2-sim.cir",
            "invert3-sim.cir",
            "invert6-sim.cir",
            "invert_b3soi-sim.cir",
            "invert_bsim3-sim.cir",
            "invert_bsim4-sim.cir",
        ]
        .into_iter()
        .map(|name| Self::normalize_manifest_key(&format!("{FAMILY_DIRECTORY}/{name}")))
        .collect()
    }

    fn validate_bug372_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug372Role,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        let record = role.owner_record();
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != record
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != record
            || !Self::same_path(&deck.path, &self.root.join(role.owner_path()))
        {
            return Err(format!("{LABEL} recognized owner is not canonical"));
        }
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeSet<_>>();
        let expected_owners = Self::bug372_all_owner_records();
        if owners != expected_owners || !owners.contains(&record) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions invalid: {error}"))?;
        if expected_owners
            .iter()
            .any(|owner| exclusions.contains_key(owner))
        {
            return Err(format!("{LABEL} wrapper owner became excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeMap<_, _>>();
        let expected_controls = Self::bug372_all_control_records();
        if family.len() != expected_controls.len()
            || expected_controls.iter().any(|control| {
                family.get(control).is_none_or(|qualification| {
                    qualification.source != EXCLUSION_SOURCE
                        || qualification.disposition != XyceUpstreamExclusionDisposition::Excluded
                })
            })
        {
            return Err(format!(
                "{LABEL} controls must remain ordinary upstream exclusions: {family:?}"
            ));
        }
        let members = self.read_bug372_directory(abort)?;
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        for (name, _, _) in RETAINED
            .iter()
            .copied()
            .filter(|record| record.0.ends_with(".cir"))
        {
            self.reject_wrapper_output_artifacts(&self.root.join(FAMILY_DIRECTORY).join(name))
                .map_err(|error| format!("{LABEL} {name} {error}"))?;
        }
        Ok(members)
    }

    fn bug372_nodes(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn bug372_param(params: &[(String, Value)], name: &str) -> Option<Value> {
        let values = params
            .iter()
            .filter(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        match values.as_slice() {
            [value] => Some(*value),
            _ => None,
        }
    }

    fn bug372_pulse(
        spec: &SourceSpec,
    ) -> Option<(Value, Value, Value, Value, Value, Value, Value)> {
        match spec {
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                pulse_count,
                width_defaults_to_zero,
            } if pulse_count.to_bits() == 0.0f64.to_bits() && !width_defaults_to_zero => {
                Some((*v1, *v2, *delay, *rise, *fall, *width, *period))
            }
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => Self::bug372_pulse(transient),
            _ => None,
        }
    }

    fn bug372_supply(role: Bug372Role) -> Value {
        if role == Bug372Role::Bsim4 { 3.3 } else { 5.0 }
    }

    fn bug372_period(role: Bug372Role) -> Value {
        if matches!(role, Bug372Role::Level2) {
            3.0 * 1.0e-6
        } else {
            3.01 * 1.0e-6
        }
    }

    fn bug372_model_names(role: Bug372Role) -> (&'static str, &'static str) {
        match role {
            Bug372Role::Level2 | Bug372Role::Level3 => ("CD4012_NMOS", "CD4012_PMOS"),
            Bug372Role::Bsim3 => ("CMOSN", "CMOSP"),
            Bug372Role::Bsim4 => ("N1", "P1"),
        }
    }

    fn validate_bug372_models(role: Bug372Role, netlist: &Netlist) -> Result<(), String> {
        if netlist.models.len() != 2 {
            return Err(format!(
                "{LABEL} {role:?} lost its exact two-model card census"
            ));
        }
        let mut polarities = BTreeSet::new();
        for model in &netlist.models {
            let polarity = if model.model_type.eq_ignore_ascii_case("NMOS") {
                "nmos"
            } else if model.model_type.eq_ignore_ascii_case("PMOS") {
                "pmos"
            } else {
                return Err(format!(
                    "{LABEL} {role:?} model polarity changed: {model:?}"
                ));
            };
            polarities.insert(polarity);
            let (nmos_model, pmos_model) = Self::bug372_model_names(role);
            let expected_name = match polarity {
                "nmos" => nmos_model,
                "pmos" => pmos_model,
                _ => unreachable!(),
            };
            let level = Self::bug372_param(&model.params, "LEVEL");
            if !model.name.eq_ignore_ascii_case(expected_name)
                || level.map(Value::to_bits) != Some((role.model_level() as Value).to_bits())
                || model.params.iter().any(|(_, value)| !value.is_finite())
                || !model.expr_params.is_empty()
                || !model.string_vector_params.is_empty()
                || !model.real_vector_params.is_empty()
                || !model.real_vector_expr_params.is_empty()
                || !model.integer_vector_params.is_empty()
            {
                return Err(format!(
                    "{LABEL} {role:?} exact native model changed: {model:?}"
                ));
            }
            if role == Bug372Role::Bsim4 {
                if !matches!(model.string_params.as_slice(), [(name, version)]
                    if name.eq_ignore_ascii_case("VERSION") && version == "4.6.1")
                {
                    return Err(format!("{LABEL} BSIM4 VERSION=4.6.1 identity changed"));
                }
            } else if !model.string_params.is_empty() {
                return Err(format!(
                    "{LABEL} {role:?} acquired string-valued model fields"
                ));
            }
        }
        if polarities != BTreeSet::from(["nmos", "pmos"]) {
            return Err(format!("{LABEL} {role:?} model polarity census changed"));
        }
        Ok(())
    }

    fn bug372_model_fingerprint(
        netlist: &Netlist,
    ) -> Vec<(String, String, Vec<(String, u64)>, Vec<(String, String)>)> {
        let mut models = netlist
            .models
            .iter()
            .map(|model| {
                let mut numeric = model
                    .params
                    .iter()
                    .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
                    .collect::<Vec<_>>();
                numeric.sort();
                let mut strings = model
                    .string_params
                    .iter()
                    .map(|(name, value)| (name.to_ascii_lowercase(), value.clone()))
                    .collect::<Vec<_>>();
                strings.sort();
                (
                    model.name.to_ascii_lowercase(),
                    model.model_type.to_ascii_lowercase(),
                    numeric,
                    strings,
                )
            })
            .collect::<Vec<_>>();
        models.sort();
        models
    }

    fn bug372_mos_signatures(netlist: &Netlist) -> BTreeSet<String> {
        let elements = netlist
            .subcircuits
            .first()
            .map(|subckt| subckt.elements.as_slice())
            .unwrap_or(netlist.elements.as_slice());
        elements
            .iter()
            .filter_map(|element| {
                let ElementKind::Mosfet {
                    model,
                    instance_params,
                    deferred_params,
                    ..
                } = &element.kind
                else {
                    return None;
                };
                let mut params = instance_params
                    .iter()
                    .filter(|(name, _)| !name.eq_ignore_ascii_case("M"))
                    .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
                    .collect::<Vec<_>>();
                params.sort();
                let nodes = element
                    .nodes
                    .iter()
                    .map(|node| node.to_ascii_lowercase())
                    .collect::<Vec<_>>();
                Some(format!(
                    "{}|{nodes:?}|{params:?}|{deferred_params:?}",
                    model.to_ascii_lowercase()
                ))
            })
            .collect()
    }

    fn validate_bug372_topology(
        role: Bug372Role,
        representation: Bug372Representation,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let hierarchical = matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4);
        let expected_top_elements = if hierarchical {
            6
        } else {
            5 + representation.expected_mos(role)
        };
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != expected_top_elements
            || netlist.models.len() != 2
            || netlist.subcircuits.len() != usize::from(hierarchical)
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} typed envelope changed"
            ));
        }
        Self::validate_bug372_models(role, netlist)?;
        let mos_elements = if hierarchical {
            let subckt = &netlist.subcircuits[0];
            if !subckt.name.eq_ignore_ascii_case("INVERTER")
                || !Self::bug372_nodes(&subckt.ports, &["IN", "OUT", "VDD", "GND"])
                || subckt.elements.len() != representation.expected_mos(role)
                || !subckt.initial_conditions.is_empty()
                || !subckt.node_sets.is_empty()
                || !subckt.params.is_empty()
                || !subckt.expr_params.is_empty()
                || !subckt.string_params.is_empty()
                || !subckt.body_params.is_empty()
                || !subckt.body_expr_params.is_empty()
                || !subckt.body_string_params.is_empty()
                || !subckt.body_functions.is_empty()
                || !subckt.local_options.is_empty()
                || subckt.library_ref.is_some()
                || !subckt.nested_subcircuits.is_empty()
            {
                return Err(format!(
                    "{LABEL} {role:?} exact INVERTER definition changed"
                ));
            }
            subckt.elements.iter().collect::<Vec<_>>()
        } else {
            netlist
                .elements
                .iter()
                .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
                .collect::<Vec<_>>()
        };
        let mut nmos = 0usize;
        let mut pmos = 0usize;
        for element in mos_elements {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                return Err(format!(
                    "{LABEL} {role:?} subcircuit acquired a non-MOS element"
                ));
            };
            let (nmos_model, pmos_model) = Self::bug372_model_names(role);
            let is_n = model.eq_ignore_ascii_case(nmos_model);
            let is_p = model.eq_ignore_ascii_case(pmos_model);
            let expected_nodes: &[&str] = if is_n {
                if matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4) {
                    &["OUT", "IN", "GND", "GND"]
                } else {
                    &["VOUT", "IN", "0", "0"]
                }
            } else if hierarchical {
                &["OUT", "IN", "VDD", "VDD"]
            } else {
                &["VOUT", "IN", "VDD", "VDD"]
            };
            let m = Self::bug372_param(instance_params, "M");
            if element.provenance != ElementProvenance::Authored
                || *compact_syntax
                || !deferred_params.is_empty()
                || !(is_n ^ is_p)
                || !Self::bug372_nodes(&element.nodes, expected_nodes)
                || m.map(Value::to_bits) != representation.expected_m(role).map(Value::to_bits)
                || instance_params.iter().any(|(_, value)| !value.is_finite())
            {
                return Err(format!(
                    "{LABEL} {role:?} MOS topology/multiplicity changed: {element:?}"
                ));
            }
            nmos += usize::from(is_n);
            pmos += usize::from(is_p);
        }
        let expected_per_polarity = representation.expected_mos(role) / 2;
        if nmos != expected_per_polarity || pmos != expected_per_polarity {
            return Err(format!("{LABEL} {role:?} NMOS/PMOS census changed"));
        }

        let find = |name: &str| {
            netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("{LABEL} {role:?} lost top-level {name}"))
        };
        let supply = Self::bug372_supply(role);
        let vdd = find("VDDdev")?;
        let rin = find("RIN")?;
        let vin = find("VIN1")?;
        let r1 = find("R1")?;
        let c2 = find("C2")?;
        let xinv = hierarchical.then(|| find("XINV1")).transpose()?;
        if !Self::bug372_nodes(&vdd.nodes, &["VDD", "0"])
            || !matches!(vdd.kind, ElementKind::VoltageSource(SourceSpec::Dc(value)) if value.to_bits() == supply.to_bits())
            || !Self::bug372_nodes(&rin.nodes, &["IN", "1"])
            || !matches!(&rin.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 1_000.0f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            || !Self::bug372_nodes(&vin.nodes, &["1", "0"])
            || !Self::bug372_nodes(&r1.nodes, &["VOUT", "0"])
            || !matches!(&r1.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 10_000.0f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            || !Self::bug372_nodes(&c2.nodes, &["VOUT", "0"])
            || !matches!(&c2.kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params }
                if value.to_bits() == (0.1f64 * 1.0e-12).to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            || xinv.is_some_and(|xinv| {
                !Self::bug372_nodes(&xinv.nodes, &["IN", "VOUT", "VDD", "0"])
                    || !matches!(&xinv.kind, ElementKind::Subcircuit { subckt_name, params }
                        if subckt_name.eq_ignore_ascii_case("INVERTER") && params.is_empty())
            })
        {
            return Err(format!("{LABEL} {role:?} exact top-level topology changed"));
        }
        let pulse = match &vin.kind {
            ElementKind::VoltageSource(spec) => Self::bug372_pulse(spec),
            _ => None,
        };
        let expected_pulse = (
            supply,
            0.0,
            1.5 * 1.0e-6,
            5.0 * 1.0e-9,
            5.0 * 1.0e-9,
            1.5 * 1.0e-6,
            Self::bug372_period(role),
        );
        if pulse.map(|values| {
            [
                values.0, values.1, values.2, values.3, values.4, values.5, values.6,
            ]
            .map(Value::to_bits)
        }) != Some(
            [
                expected_pulse.0,
                expected_pulse.1,
                expected_pulse.2,
                expected_pulse.3,
                expected_pulse.4,
                expected_pulse.5,
                expected_pulse.6,
            ]
            .map(Value::to_bits),
        ) {
            return Err(format!(
                "{LABEL} {role:?} ideal VIN1 pulse changed: {pulse:?}"
            ));
        }
        Ok(())
    }

    fn bug372_expected_census(
        role: Bug372Role,
        representation: Bug372Representation,
    ) -> (usize, usize, usize, usize, &'static str) {
        let owner = representation == Bug372Representation::ExplicitParallel;
        match (role, owner) {
            (Bug372Role::Level2 | Bug372Role::Level3, true) => (12, 14, 17, 4, "MOSFET"),
            (Bug372Role::Level2 | Bug372Role::Level3, false) => (8, 10, 11, 2, "MOSFET"),
            (Bug372Role::Bsim3, true) => (4, 6, 69, 64, "BSIM3"),
            (Bug372Role::Bsim3, false) => (4, 6, 7, 2, "BSIM3"),
            (Bug372Role::Bsim4, true) => (68, 70, 133, 64, "BSIM4"),
            (Bug372Role::Bsim4, false) => (6, 8, 9, 2, "BSIM4"),
        }
    }

    fn validate_bug372_worker(
        &self,
        role: Bug372Role,
        representation: Bug372Representation,
        plan: XyceStaticTranPlan,
        expected_source: &str,
        path: &Path,
    ) -> Result<Bug372Worker, String> {
        let canonical_plan_source =
            Self::canonical_lf_text_identity(LABEL, plan.source.as_bytes())?;
        let expected_probes: &[&str] = if role == Bug372Role::Level3 {
            &["v(vout)", "v(in)"]
        } else {
            &["v(vout)", "v(in)", "v(1)"]
        };
        if plan.deck_path != path
            || canonical_plan_source.as_slice() != expected_source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.tran.step.to_bits() != (20.0f64 * 1.0e-9).to_bits()
            || plan.tran.stop.to_bits() != (30.0f64 * 1.0e-6).to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print
                    .probes
                    .iter()
                    .map(String::as_str)
                    .ne(expected_probes.iter().copied())
            })
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} transient plan changed: {plan:?}"
            ));
        }
        let netlist = Self::parse_xyce_netlist(expected_source, path)
            .map_err(|error| format!("{LABEL} {role:?} Xyce parse failed: {error}"))?;
        Self::validate_bug372_topology(role, representation, &netlist)?;
        let expected_reltol: Value = match role {
            Bug372Role::Level2 => 1.0e-4,
            Bug372Role::Level3 => 5.0e-5,
            Bug372Role::Bsim3 | Bug372Role::Bsim4 => 5.0e-3,
        };
        let expected_abstol =
            matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4).then_some(1.0e-3f64.to_bits());
        let expected_newlte = (role == Bug372Role::Level3
            && representation != Bug372Representation::ExplicitParallel)
            .then_some(1);
        let expected_tr_partition = (role == Bug372Role::Bsim3
            && representation == Bug372Representation::ExplicitParallel)
            .then_some(false);
        if netlist.options.timeint_reltol.map(Value::to_bits) != Some(expected_reltol.to_bits())
            || netlist.options.timeint_abstol.map(Value::to_bits) != expected_abstol
            || netlist
                .options
                .transient_lte_reference
                .and_then(TransientLteReference::xyce_selector)
                != expected_newlte
            || netlist.options.linsol_tr_partition != expected_tr_partition
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} TIMEINT options changed: {:?}",
                netlist.options
            ));
        }
        let request = &netlist.output_requests[0];
        let expected_precision =
            matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4).then_some(10);
        let expected_width = matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4).then_some(19);
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || request.print_precision != expected_precision
            || request.print_width != expected_width
            || request
                .operands
                .iter()
                .map(String::as_str)
                .ne(expected_probes.iter().copied())
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} exact PRINT changed: {request:?}"
            ));
        }
        let circuit = self
            .create_xyce_engine()
            .build_circuit(&netlist)
            .map_err(|error| format!("{LABEL} {role:?} assembly failed: {error}"))?;
        let (nodes, matrix, devices, op_count, kind) =
            Self::bug372_expected_census(role, representation);
        let branches = circuit.branch_names_sorted();
        let report = circuit.device_op_report();
        let names = report
            .entries
            .iter()
            .map(|entry| entry.name.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        if circuit.has_generated_veriloga_devices()
            || circuit.num_nodes() != nodes
            || circuit.matrix_size() != matrix
            || circuit.device_count() != devices
            || branches.len() != 2
            || branches.iter().map(String::as_str).ne(["VDDDEV", "VIN1"])
            || report.entries.len() != op_count
            || names.len() != op_count
            || !report.labels_resolve()
            || report
                .entries
                .iter()
                .any(|entry| !entry.device_kind.eq_ignore_ascii_case(kind))
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} native-route census changed: nodes={}/{nodes}, matrix={}/{matrix}, devices={}/{devices}, branches={branches:?}, report={report:?}",
                circuit.num_nodes(),
                circuit.matrix_size(),
                circuit.device_count()
            ));
        }
        Ok(Bug372Worker {
            plan,
            netlist,
            representation,
        })
    }

    fn bug372_unit_source(role: Bug372Role, control: &str) -> Result<String, String> {
        let authored = format!("M={}", role.multiplicity());
        let mut occurrences = 0usize;
        let mut lines = Vec::new();
        for line in control.lines() {
            let trimmed = line.trim_start();
            let rewritten = if !trimmed.starts_with('*') {
                occurrences += line
                    .split_ascii_whitespace()
                    .filter(|token| *token == authored)
                    .count();
                line.replace(&format!(" {authored}"), " M=1")
            } else {
                line.to_string()
            };
            lines.push(rewritten);
        }
        if occurrences != 2 {
            return Err(format!(
                "{LABEL} {role:?} cannot derive exact M=1 counterfactual: found {occurrences} {authored} assignments"
            ));
        }
        let mut source = lines.join("\n");
        source.push('\n');
        Ok(source)
    }

    fn validate_bug372_pair_structure(
        role: Bug372Role,
        owner: &Netlist,
        control: &Netlist,
        unit: &Netlist,
    ) -> Result<(), String> {
        let owner_models = Self::bug372_model_fingerprint(owner);
        if owner_models != Self::bug372_model_fingerprint(control)
            || owner_models != Self::bug372_model_fingerprint(unit)
        {
            return Err(format!("{LABEL} {role:?} owner/control model cards differ"));
        }
        let owner_mos = Self::bug372_mos_signatures(owner);
        if owner_mos.len() != 2
            || owner_mos != Self::bug372_mos_signatures(control)
            || owner_mos != Self::bug372_mos_signatures(unit)
        {
            return Err(format!(
                "{LABEL} {role:?} explicit/M/control device semantics differ after removing M"
            ));
        }
        Ok(())
    }

    fn bug372_tolerances(role: Bug372Role) -> Vec<XyceVerifyTransientTolerance> {
        let default = XyceVerifyTransientTolerance::release_7_10_default();
        match role {
            Bug372Role::Level2 => vec![
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 5.0e-7,
                    ..default
                },
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 5.0e-7,
                    zero: 1.0e-8,
                    ..default
                },
                default,
            ],
            Bug372Role::Level3 => vec![
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 1.0e-6,
                    ..default
                },
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 1.0e-6,
                    zero: 1.0e-6,
                    absolute_difference: 1.0e-6,
                    ..default
                },
            ],
            Bug372Role::Bsim3 | Bug372Role::Bsim4 => vec![default; 3],
        }
    }

    fn validate_bug372_comp_source(role: Bug372Role, source: &str) -> Result<(), String> {
        let probes = if role == Bug372Role::Level3 {
            vec!["v(vout)".to_string(), "v(in)".to_string()]
        } else {
            vec![
                "v(vout)".to_string(),
                "v(in)".to_string(),
                "v(1)".to_string(),
            ]
        };
        let expected = Self::bug372_tolerances(role);
        if matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4) {
            if Self::source_has_comp_directive(source) {
                return Err(format!(
                    "{LABEL} {role:?} unexpectedly acquired *COMP overrides"
                ));
            }
        } else {
            let actual = Self::xyce_verify_comp_tolerances(source, &probes)
                .map_err(|error| format!("{LABEL} {role:?} *COMP parsing failed: {error}"))?;
            if actual != expected {
                return Err(format!(
                    "{LABEL} {role:?} exact Release *COMP tolerances changed: {actual:?}"
                ));
            }
        }
        Ok(())
    }

    fn bug372_node_trace<'a>(
        role: Bug372Role,
        result: &'a TransientResult,
        node: &str,
    ) -> Result<&'a [Value], String> {
        let index = result
            .node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(node))
            .ok_or_else(|| format!("{LABEL} {role:?} result lost node {node}"))?;
        result
            .voltages
            .get(index)
            .map(Vec::as_slice)
            .filter(|trace| trace.len() == result.time.len())
            .ok_or_else(|| format!("{LABEL} {role:?} node {node} trace is incomplete"))
    }

    fn bug372_pulse_value(role: Bug372Role, time: Value) -> Value {
        let v1 = Self::bug372_supply(role);
        let v2 = 0.0;
        let delay = 1.5 * 1.0e-6;
        let rise = 5.0 * 1.0e-9;
        let fall = 5.0 * 1.0e-9;
        let width = 1.5 * 1.0e-6;
        let period = Self::bug372_period(role);
        if time <= delay {
            return v1;
        }
        let phase = (time - delay).rem_euclid(period);
        if phase < rise {
            v1 + (v2 - v1) * phase / rise
        } else if phase < rise + width {
            v2
        } else if phase < rise + width + fall {
            v2 + (v1 - v2) * (phase - rise - width) / fall
        } else {
            v1
        }
    }

    fn validate_bug372_nontrivial_columns(
        role: Bug372Role,
        table: &XycePrnTable,
    ) -> Result<(), String> {
        if !(100..=20_000).contains(&table.rows.len()) {
            return Err(format!("{LABEL} {role:?} output row envelope changed"));
        }
        let mut ranges = [(Value::INFINITY, Value::NEG_INFINITY); 2];
        for row in &table.rows {
            if row.len() != table.columns.len() || row.iter().any(|value| !value.is_finite()) {
                return Err(format!("{LABEL} {role:?} output contains malformed values"));
            }
            for (range, value) in ranges.iter_mut().zip(&row[2..4]) {
                range.0 = range.0.min(*value);
                range.1 = range.1.max(*value);
            }
        }
        let supply = Self::bug372_supply(role);
        let bounded = ranges
            .iter()
            .all(|(min, max)| *min >= -1.0 && *max <= supply + 1.0);
        // The BSIM4 control represents 32 devices behind the authored 1 kOhm
        // input resistor, so its physical gate loading limits V(IN) to about
        // 0.229*VDD even while the printed ideal V(1) pulse spans full VDD.
        let vin_span_fraction = if role == Bug372Role::Bsim4 {
            0.20
        } else {
            0.55
        };
        if !bounded
            || ranges[0].1 - ranges[0].0 < 0.55 * supply
            || ranges[1].1 - ranges[1].0 < vin_span_fraction * supply
        {
            return Err(format!(
                "{LABEL} {role:?} waveform became shared-zero, invariant, or out of range: {ranges:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug372_result(
        role: Bug372Role,
        representation: Bug372Representation,
        table: &XycePrnTable,
        result: &TransientResult,
    ) -> Result<(), String> {
        let expected_columns: &[&str] = if role == Bug372Role::Level3 {
            &["Index", "TIME", "V(VOUT)", "V(IN)"]
        } else {
            &["Index", "TIME", "V(VOUT)", "V(IN)", "V(1)"]
        };
        if table.columns.len() != expected_columns.len()
            || table
                .columns
                .iter()
                .zip(expected_columns)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() != result.time.len()
            || result.step_sizes.len() != result.time.len()
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} PRN/result shape changed"
            ));
        }
        Self::validate_bug372_nontrivial_columns(role, table)?;
        let vout = Self::bug372_node_trace(role, result, "VOUT")?;
        let vin = Self::bug372_node_trace(role, result, "IN")?;
        let v1 = (expected_columns.len() == 5)
            .then(|| Self::bug372_node_trace(role, result, "1"))
            .transpose()?;
        let mut previous = None;
        for (index, (row, time)) in table.rows.iter().zip(&result.time).enumerate() {
            let expected_source = Self::bug372_pulse_value(role, *time);
            if row[0].to_bits() != (index as Value).to_bits()
                || row[1].to_bits() != time.to_bits()
                || row[2].to_bits() != vout[index].to_bits()
                || row[3].to_bits() != vin[index].to_bits()
                || v1.is_some_and(|trace| {
                    row[4].to_bits() != trace[index].to_bits()
                        || (trace[index] - expected_source).abs() > 1.0e-8
                })
                || previous.is_some_and(|prior| *time <= prior)
            {
                return Err(format!(
                    "{LABEL} {role:?} {representation:?} raw solution/ideal-source linkage failed at row {index}: {row:?}"
                ));
            }
            previous = Some(*time);
        }
        let first = table.rows.first().expect("validated nonempty");
        let last = table.rows.last().expect("validated nonempty");
        if first[1].abs() > 1.0e-18 || (last[1] - 30.0e-6).abs() > 1.0e-13 {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} transient horizon changed: first={first:?}, last={last:?}"
            ));
        }
        Ok(())
    }

    fn run_bug372_worker(
        &self,
        role: Bug372Role,
        worker: &Bug372Worker,
        start: Instant,
    ) -> Result<Bug372Run, String> {
        let max_step = Self::transient_family_max_step(&worker.netlist, &worker.plan.tran)
            .map_err(|error| format!("{LABEL} {role:?} max-step validation failed: {error}"))?;
        let engine = self.create_xyce_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let result = engine
            .run_tran_with_startup_mode_and_abort(
                &worker.netlist,
                worker.plan.tran.stop,
                max_step,
                TransientStartupMode::from_uic(worker.plan.tran.uic),
                &abort,
            )
            .map_err(|error| {
                format!(
                    "{LABEL} {role:?} {:?} execution failed: {error}",
                    worker.representation
                )
            })?;
        let quality = engine.convergence_quality();
        if quality.gmin_stepping_count != 0
            || quality.source_stepping_count != 0
            || quality.force_accepted_points != 0
            || !quality.force_accepted_indices.is_empty()
            || quality.failure_diagnostic.is_some()
        {
            return Err(format!(
                "{LABEL} {role:?} {:?} returned a numerically unqualified waveform: {quality:?}",
                worker.representation
            ));
        }
        let table =
            Self::transient_family_result_to_prn_table(&worker.plan, &worker.netlist, &result)?;
        Self::validate_bug372_result(role, worker.representation, &table, &result)?;
        Ok(Bug372Run { table, result })
    }

    fn validate_bug372_relation(
        &self,
        role: Bug372Role,
        owner: &Bug372Run,
        control: &Bug372Run,
        unit: &Bug372Run,
    ) -> Result<(), String> {
        if std::ptr::eq(owner, control)
            || std::ptr::eq(owner, unit)
            || std::ptr::eq(control, unit)
            || owner.result.time.as_ptr() == control.result.time.as_ptr()
            || owner.result.time.as_ptr() == unit.result.time.as_ptr()
        {
            return Err(format!("{LABEL} {role:?} independent runs became aliased"));
        }
        let tolerances = Self::bug372_tolerances(role);
        let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            &owner.table,
            &control.table,
            &tolerances,
            role.print_precision(),
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} {role:?} M-control TEST differs from explicit-parallel GOOD: {mismatches:?}"
            ));
        }
        let ignored_m = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            &owner.table,
            &unit.table,
            &tolerances,
            role.print_precision(),
        )?;
        if ignored_m.is_empty() {
            return Err(format!(
                "{LABEL} {role:?} M=1 counterfactual still passes, so multiplicity is not independently observed"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug372_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug372Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let members = self.validate_bug372_provenance(deck, role, &abort)?;
        let source = |name: &str| -> Result<&str, String> {
            std::str::from_utf8(
                members
                    .get(&name.to_ascii_lowercase())
                    .ok_or_else(|| format!("{LABEL} lost retained member {name}"))?,
            )
            .map_err(|error| format!("{LABEL} member {name} is not UTF-8: {error}"))
        };
        let owner_source = source(role.owner_name())?;
        let control_source = source(role.control_name())?;
        Self::validate_bug372_comp_source(role, owner_source)?;
        let unit_source = Self::bug372_unit_source(role, control_source)?;
        let purpose = XyceStaticTranPlanPurpose::Bug372MultiplicityRelationalFamily;
        let owner_path = self.root.join(role.owner_path());
        let control_path = self.root.join(role.control_path());
        let owner_plan = self.static_tran_plan_for_sealed_source_with_purpose(
            &owner_path,
            owner_source,
            purpose,
        )?;
        let control_plan = self.static_tran_plan_for_sealed_source_with_purpose(
            &control_path,
            control_source,
            purpose,
        )?;
        let unit_path = self.root.join(FAMILY_DIRECTORY).join(format!(
            "__rspice_{}_unit_m_counterfactual.cir",
            role.owner_name()
        ));
        let mut unit_plan = control_plan.clone();
        unit_plan.deck_path = unit_path.clone();
        unit_plan.source = unit_source.clone();
        let owner = self.validate_bug372_worker(
            role,
            Bug372Representation::ExplicitParallel,
            owner_plan,
            owner_source,
            &owner_path,
        )?;
        let control = self.validate_bug372_worker(
            role,
            Bug372Representation::Multiplicity,
            control_plan,
            control_source,
            &control_path,
        )?;
        let unit = self.validate_bug372_worker(
            role,
            Bug372Representation::UnitMultiplicityCounterfactual,
            unit_plan,
            &unit_source,
            &unit_path,
        )?;
        Self::validate_bug372_pair_structure(
            role,
            &owner.netlist,
            &control.netlist,
            &unit.netlist,
        )?;
        let owner_run = self.run_bug372_worker(role, &owner, start)?;
        let control_run = self.run_bug372_worker(role, &control, start)?;
        let unit_run = self.run_bug372_worker(role, &unit, start)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} {role:?} execution exceeded deadline"));
        }
        self.validate_bug372_relation(role, &owner_run, &control_run, &unit_run)?;
        self.validate_bug372_provenance(deck, role, &abort)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} {role:?} final provenance exceeded deadline"
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

    #[test]
    fn bug372_routes_exactly_four_owner_records() {
        for role in QUALIFIED {
            assert_eq!(Bug372Role::for_record(&role.owner_record()), Some(role));
            assert_eq!(Bug372Role::for_record(&role.control_path()), None);
        }
        for unsupported in ["invert1.cir", "invert6.cir", "invert_b3soi.cir"] {
            assert_eq!(
                Bug372Role::for_record(&format!("{FAMILY_DIRECTORY}/{unsupported}")),
                None
            );
        }
    }

    #[test]
    fn bug372_historical_and_retained_provenance_is_exact() {
        XyceTestRunner::validate_bug372_historical_provenance().unwrap();
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let members = runner.read_bug372_directory(&abort).unwrap();
        assert_eq!(members.len(), RETAINED.len());
    }

    #[test]
    fn bug372_level2_checked_in_owner_executes_complete_oracle() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let result = runner.run_test(root.join(Bug372Role::Level2.owner_path()));
        assert!(result.passed, "BUG372 LEVEL2 failed: {:?}", result.error);
        assert_eq!(result.contract, Bug372Role::Level2.contract());
    }

    #[test]
    fn bug372_unit_counterfactual_rewrites_only_the_two_authored_m_assignments() {
        let root = corpus_root();
        for role in QUALIFIED {
            let control = fs::read_to_string(root.join(role.control_path()))
                .expect("checked-in BUG372 control exists");
            let unit = XyceTestRunner::bug372_unit_source(role, &control)
                .expect("exact M=1 counterfactual derives");
            assert_eq!(
                unit.split_ascii_whitespace()
                    .filter(|token| *token == "M=1")
                    .count(),
                2
            );
            assert_eq!(
                unit.split_ascii_whitespace()
                    .filter(|token| *token == format!("M={}", role.multiplicity()))
                    .count(),
                0
            );
            assert_eq!(
                unit.matches("TNOM=27").count(),
                control.matches("TNOM=27").count(),
                "counterfactual must not rewrite model parameters containing the digit 2"
            );
            let path = root.join(role.control_path());
            let netlist = XyceTestRunner::parse_xyce_netlist(&unit, &path).unwrap();
            XyceTestRunner::validate_bug372_topology(
                role,
                Bug372Representation::UnitMultiplicityCounterfactual,
                &netlist,
            )
            .unwrap();
        }
    }

    #[test]
    fn bug372_optional_parameter_lookup_is_non_panicking_and_exact() {
        let params = vec![("L".to_string(), 5.0e-6)];
        assert_eq!(XyceTestRunner::bug372_param(&params, "M"), None);
        assert_eq!(XyceTestRunner::bug372_param(&params, "LEVEL"), None);
        let one = vec![("M".to_string(), 2.0)];
        assert_eq!(XyceTestRunner::bug372_param(&one, "m"), Some(2.0));
        let repeated = vec![("M".to_string(), 2.0), ("m".to_string(), 2.0)];
        assert_eq!(XyceTestRunner::bug372_param(&repeated, "M"), None);
    }

    #[test]
    fn bug372_sealed_source_planner_does_not_reopen_the_deck_path() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let source = fs::read_to_string(runner.root.join(Bug372Role::Level2.owner_path())).unwrap();
        let missing_path = runner
            .root
            .join(FAMILY_DIRECTORY)
            .join("__rspice_bug372_missing_owner.cir");
        assert!(!missing_path.exists());
        let deck = XyceDeck {
            path: missing_path.clone(),
            section: XyceDeckSection::Netlists,
            relative_path: Bug372Role::Level2.owner_path(),
        };
        let plan = runner
            .static_tran_plan_for_deck_with_sealed_source_and_purpose(
                &deck,
                &source,
                XyceStaticTranPlanPurpose::Bug372MultiplicityRelationalFamily,
            )
            .expect("authenticated source plans without reading the nonexistent deck path");
        assert_eq!(plan.deck_path, missing_path);
        assert_eq!(plan.source, source);
    }

    #[test]
    fn bug372_topology_rejects_a_wrong_native_model_route() {
        let root = corpus_root();
        let path = root.join(Bug372Role::Level2.owner_path());
        let source = fs::read_to_string(&path).expect("checked-in BUG372 owner exists");
        let netlist = XyceTestRunner::parse_xyce_netlist(&source, &path).unwrap();
        XyceTestRunner::validate_bug372_topology(
            Bug372Role::Level2,
            Bug372Representation::ExplicitParallel,
            &netlist,
        )
        .unwrap();

        let wrong_level = source.replacen("LEVEL = 2", "LEVEL = 9", 1);
        assert_ne!(wrong_level, source);
        let wrong = XyceTestRunner::parse_xyce_netlist(&wrong_level, &path).unwrap();
        assert!(
            XyceTestRunner::validate_bug372_topology(
                Bug372Role::Level2,
                Bug372Representation::ExplicitParallel,
                &wrong,
            )
            .is_err()
        );
    }

    #[test]
    fn bug372_shared_zero_can_pass_history_but_not_the_independent_waveform_gate() {
        let columns = vec![
            "Index".into(),
            "TIME".into(),
            "V(VOUT)".into(),
            "V(IN)".into(),
            "V(1)".into(),
        ];
        let rows = (0..100)
            .map(|index| vec![index as Value, index as Value * 1.0e-9, 0.0, 0.0, 0.0])
            .collect::<Vec<_>>();
        let shared_zero = XycePrnTable { columns, rows };
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &shared_zero,
                    &shared_zero,
                    &XyceTestRunner::bug372_tolerances(Bug372Role::Level2),
                    Bug372Role::Level2.print_precision(),
                )
                .unwrap()
                .is_empty(),
            "the historical comparator alone admits identical shared-wrong streams"
        );
        assert!(
            XyceTestRunner::validate_bug372_nontrivial_columns(Bug372Role::Level2, &shared_zero,)
                .is_err()
        );
    }

    #[test]
    fn bug372_release_direction_and_threshold_are_fail_closed() {
        let table = |value: Value| XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(X)".into()],
            rows: vec![vec![0.0, 0.0, value], vec![1.0, 1.0, value]],
        };
        let good = table(1.0);
        let threshold = table(0.0);
        let over_threshold = table(-0.01);
        let tolerance = XyceVerifyTransientTolerance {
            relative: 1.0,
            absolute: 1.0e-12,
            zero: 0.0,
            absolute_difference: 0.0,
            offset: 0.0,
        };
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &good,
                    &threshold,
                    &[tolerance],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .unwrap()
                .is_empty(),
            "GOOD-denominator comparison must accept the exact Release threshold"
        );
        assert!(
            !runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &threshold,
                    &good,
                    &[tolerance],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .unwrap()
                .is_empty(),
            "reversing GOOD and TEST must not silently change the denominator"
        );
        assert!(
            !runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &good,
                    &over_threshold,
                    &[tolerance],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .unwrap()
                .is_empty(),
            "integrated normalized RMS values above one must fail"
        );
    }
}
