use super::*;
use rspice_core::analysis::{MeasureResult, evaluate_tran_measurements};
use rspice_core::netlist::measure::{
    ExtremaOutput, MeasurePrintPolicy, MeasureStatement, MeasureType,
};
use rspice_core::netlist::{
    OutputRequest, OutputSymbolDependency, SaveSignal, SimulationOptions, SourceSpec,
};
use std::io::Read as _;

const LABEL: &str = "BUG_440_SON lead-current measurement wrappers";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_440_SON";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_440_son/";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_440_SON";
const DEFERRED_OWNER_RECORD: &str = "netlists/certification_tests/bug_440_son/bug440.cir";

const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const PRETRIM_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";

const HISTORICAL_CONTENT_BYTES: usize = 84_652;
const HISTORICAL_STREAM_BYTES: usize = 3_649;
const HISTORICAL_STREAM_SHA256: &str =
    "478f383c993a06e7f7531ef6a6488a13bde5c140e2f635ac81f1ae3ee17a23c0";
const HISTORICAL_STREAM_BLAKE3: &str =
    "275991fb2df35a5280743cb4cee29de41fd691ec986c5e6089dd81d7540aba3e";
const HISTORICAL: [(&str, usize, &str, &str, &str); 9] = [
    (
        "Netlists/Certification_Tests/BUG_440_SON/Manifest.txt",
        84,
        "82713233d518435f2f286ed6a3847cc00daefa65",
        "f68d7dd8d1c8506bb2f0b459851f8e89c5d399adb34a2fbe058397d05d99cad4",
        "69b6d78ba3dc0bef6408aac1dbeb41ae480dcab0f2620c562ab6a3d685c329d6",
    ),
    (
        "Netlists/Certification_Tests/BUG_440_SON/bug440.cir",
        880,
        "c55e4c31bbdc64d6c47f5f9256005802e3e32343",
        "f1986b3e35d4da9f8962dca0fe7581c0e477cb27eb380b330d2f2b0e94bab5c3",
        "2bae8a71cac99163c0b437d69236f53f3e0ba2a1f16d4d49037f3bee6316083f",
    ),
    (
        "Netlists/Certification_Tests/BUG_440_SON/bug440.cir.sh",
        5_022,
        "dcb2f14787e88d86b6a3105627cd57d2eec88ff9",
        "b17db356a309c3dcdfc6a98ce8d28a5c9afe484ff5e95bfa41d30cb2db118a1b",
        "776836e9f18e099d2ec8688c6feffa4f73e522144e95d49e96e9d368f9160df7",
    ),
    (
        "Netlists/Certification_Tests/BUG_440_SON/bug440a.cir",
        212,
        "010825a9e4be6276a5bdb19d8f585d5a369fc524",
        "89033beca3384b93008eb46128895af91c91d3d8b142b9c60000ac9866e8d92f",
        "c1809effaad13c9fab3486da34cc99bb860df6b5118e54fb4f54de4270c2f4ef",
    ),
    (
        "Netlists/Certification_Tests/BUG_440_SON/bug440a.cir.sh",
        4_759,
        "ab501c2d8eabb5c0ac8bd6572f4e7508a00c61fc",
        "4a042d8a29c08d5fc0cb93c26e50117c4a80e51f71a5a9f30f8f64ec7ed6a179",
        "28f3a76b43785abd9867b6de12d5828213d5a0a0145c0694dea08f64b4278305",
    ),
    (
        "Netlists/Certification_Tests/BUG_440_SON/bug440b.cir",
        447,
        "7d87333e8facda570304a23fe0cca6b3777c29ff",
        "cf8d34515ff04f14f03fb0620b58fcc62478b6d3086746851ee769203cbb16e1",
        "75e32e1bf86f41ab0fd9dc4d2d3b067773823154f22893eeb5502452ca0718f8",
    ),
    (
        "Netlists/Certification_Tests/BUG_440_SON/bug440b.cir.sh",
        4_812,
        "5a7634ad9df6408f90d4bdb203fa8d3480d8e75f",
        "995f3f6d9e432fb6b4eaec7939011e0978f460bd7a0d2b53533c8eb11b8e5870",
        "6500f00a7e8decd1fc288d51f65e973e68f1a2994c30bb49f36debd533f3af38",
    ),
    (
        "Netlists/Certification_Tests/BUG_440_SON/tags",
        32,
        "b10b12760d62a7c29e906858947fb20bd4f9859a",
        "150a1ba6a65b079efe2d9a7ad9183b9a0d1fb33a3e3940987cc8887c8c624640",
        "9606c6660adfe92d17c201bd8b975486503e32a94dc84ceedb81e3341087402d",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_404,
        "17def57575eb3fd703978fd0634a58d6e679a3de",
        "a86524def2895930f2bb697c058850f231df6d623c279bd7482f30c5c39090b4",
        "11483735d34385359bbe0f981cbc8767c11e6c2b60c3f8723d49e2761479023a",
    ),
];

const RETAINED_CONTENT_BYTES: usize = 1_539;
const RETAINED_STREAM_BYTES: usize = 364;
const RETAINED_STREAM_SHA256: &str =
    "cdb51062d3e7f9603fa4ddd99ff0dad3d4566d0ba47b29f0459e08d07c7f0f70";
const RETAINED_STREAM_BLAKE3: &str =
    "d4bf0a778d7ae873a763e5080903248f08709d3c9c72b6334b81fdfd8cb608c7";
const RETAINED: [(&str, usize, &str, &str); 3] = [
    (
        "bug440.cir",
        880,
        "f1986b3e35d4da9f8962dca0fe7581c0e477cb27eb380b330d2f2b0e94bab5c3",
        "2bae8a71cac99163c0b437d69236f53f3e0ba2a1f16d4d49037f3bee6316083f",
    ),
    (
        "bug440a.cir",
        212,
        "89033beca3384b93008eb46128895af91c91d3d8b142b9c60000ac9866e8d92f",
        "c1809effaad13c9fab3486da34cc99bb860df6b5118e54fb4f54de4270c2f4ef",
    ),
    (
        "bug440b.cir",
        447,
        "cf8d34515ff04f14f03fb0620b58fcc62478b6d3086746851ee769203cbb16e1",
        "75e32e1bf86f41ab0fd9dc4d2d3b067773823154f22893eeb5502452ca0718f8",
    ),
];

const WRAPPER_ABS_TOL: Value = 1.0e-6;
const WRAPPER_REL_TOL: Value = 0.02;
const WRAPPER_ZERO_TOL: Value = 1.0e-5;
const MAX_STREAM_ROWS: usize = 256;
const MAX_STREAM_BYTES: usize = 64_000;
const MAX_MANIFEST_BYTES: usize = 1_048_576;
const MAX_DIRECTORY_ENTRIES: usize = 16_384;
const PRN_SCIENTIFIC_PRECISION: i32 = 8;
const RESISTOR_TRAN_STEP: Value = 1.0e-6;
// Match Xyce's suffix evaluation (`10 * 1e-6`) instead of decimal-literal
// folding so the sealed typed AST comparison is bit exact.
const RESISTOR_TRAN_STOP: Value = 10.0 * 1.0e-6;
const RESISTOR_SOURCE_FREQUENCY: Value = 100.0e3;
const RESISTOR_SOURCE_ADVANCE: Value = 2.5 * 1.0e-6;
const BEHAVIORAL_TRAN_STEP: Value = 1.0;
const BEHAVIORAL_TRAN_STOP: Value = 12.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Bug440Role {
    Resistor,
    Behavioral,
}

impl Bug440Role {
    const ALL: [Self; 2] = [Self::Resistor, Self::Behavioral];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::Resistor => "bug440a.cir",
            Self::Behavioral => "bug440b.cir",
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::Resistor => "Netlists/Certification_Tests/BUG_440_SON/bug440a.cir",
            Self::Behavioral => "Netlists/Certification_Tests/BUG_440_SON/bug440b.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Resistor => "netlists/certification_tests/bug_440_son/bug440a.cir",
            Self::Behavioral => "netlists/certification_tests/bug_440_son/bug440b.cir",
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::Resistor => "bug440a_resistor_lead_current_measurement_wrapper_owner",
            Self::Behavioral => "bug440b_behavioral_source_lead_current_measurement_wrapper_owner",
        }
    }

    fn expected_probes(self) -> &'static [&'static str] {
        match self {
            Self::Resistor => &["V(1)", "I(RTEST)"],
            Self::Behavioral => &["I(R1)", "I(B2)"],
        }
    }

    fn expected_measure_names(self) -> &'static [&'static str] {
        match self {
            Self::Resistor => &["max1", "max2"],
            Self::Behavioral => &["min1", "min2"],
        }
    }

    const fn tran_step(self) -> Value {
        match self {
            Self::Resistor => RESISTOR_TRAN_STEP,
            Self::Behavioral => BEHAVIORAL_TRAN_STEP,
        }
    }

    const fn tran_stop(self) -> Value {
        match self {
            Self::Resistor => RESISTOR_TRAN_STOP,
            Self::Behavioral => BEHAVIORAL_TRAN_STOP,
        }
    }
}

impl XyceTestRunner {
    fn find_bug440_exact_child(
        parent: &Path,
        expected: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Option<PathBuf>, String> {
        let metadata = fs::symlink_metadata(parent)
            .map_err(|error| format!("failed to inspect {LABEL} parent directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} parent must be a regular non-symlink directory"
            ));
        }
        let mut matches = Vec::new();
        for (index, entry) in fs::read_dir(parent)
            .map_err(|error| format!("failed to enumerate {LABEL} parent: {error}"))?
            .enumerate()
        {
            if index >= MAX_DIRECTORY_ENTRIES {
                return Err(format!(
                    "{LABEL} parent directory exceeds its census envelope"
                ));
            }
            if index % 256 == 0 && abort.is_aborted() {
                return Err(format!("{LABEL} path-case validation aborted"));
            }
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} child: {error}"))?;
            let Some(name) = entry.file_name().to_str().map(str::to_string) else {
                continue;
            };
            if name.eq_ignore_ascii_case(expected) {
                matches.push(name);
            }
        }
        match matches.as_slice() {
            [] => Ok(None),
            [name] if name == expected => Ok(Some(parent.join(name))),
            _ => Err(format!(
                "{LABEL} path component {expected:?} changed case or became ambiguous: {matches:?}"
            )),
        }
    }

    fn bug440_exact_child(
        parent: &Path,
        expected: &str,
        abort: &dyn AbortSignal,
    ) -> Result<PathBuf, String> {
        Self::find_bug440_exact_child(parent, expected, abort)?
            .ok_or_else(|| format!("{LABEL} is missing exact path component {expected:?}"))
    }

    fn bug440_family_directory(&self, abort: &dyn AbortSignal) -> Result<PathBuf, String> {
        let netlists = Self::bug440_exact_child(&self.root, "Netlists", abort)?;
        let certification = Self::bug440_exact_child(&netlists, "Certification_Tests", abort)?;
        Self::bug440_exact_child(&certification, "BUG_440_SON", abort)
    }

    fn bug440_record_stream_identities() -> ((usize, String, String), (usize, String, String)) {
        let mut historical = HISTORICAL
            .iter()
            .map(|(path, bytes, blob, sha, b3)| {
                format!("{PRETRIM_COMMIT}\t{PRETRIM_NETLISTS_TREE}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{RELEASE_TAG_OBJECT}\t{path}\t{bytes}\t{blob}\t{sha}\t{b3}")
            })
            .collect::<Vec<_>>();
        historical.sort();
        let historical = historical.join("\n");
        let mut retained = RETAINED
            .iter()
            .map(|(name, bytes, sha, _)| format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha}"))
            .collect::<Vec<_>>();
        retained.sort();
        let retained = retained.join("\n");
        (
            (
                historical.len(),
                format!("{:x}", Sha256::digest(historical.as_bytes())),
                blake3::hash(historical.as_bytes()).to_hex().to_string(),
            ),
            (
                retained.len(),
                format!("{:x}", Sha256::digest(retained.as_bytes())),
                blake3::hash(retained.as_bytes()).to_hex().to_string(),
            ),
        )
    }

    fn validate_bug440_record_streams() -> Result<(), String> {
        let historical_content = HISTORICAL.iter().map(|record| record.1).sum::<usize>();
        let retained_content = RETAINED.iter().map(|record| record.1).sum::<usize>();
        let historical_unique = HISTORICAL
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let retained_unique = RETAINED
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let (historical, retained) = Self::bug440_record_stream_identities();
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || PRETRIM_NETLISTS_TREE != UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
            || HISTORICAL.len() != 9
            || historical_unique.len() != HISTORICAL.len()
            || historical_content != HISTORICAL_CONTENT_BYTES
            || HISTORICAL.iter().any(|record| {
                record.2.len() != 40
                    || record.3.len() != 64
                    || record.4.len() != 64
                    || !record.2.bytes().all(|byte| byte.is_ascii_hexdigit())
                    || !record.3.bytes().all(|byte| byte.is_ascii_hexdigit())
                    || !record.4.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
            || historical.0 != HISTORICAL_STREAM_BYTES
            || historical.1 != HISTORICAL_STREAM_SHA256
            || historical.2 != HISTORICAL_STREAM_BLAKE3
            || RETAINED.len() != 3
            || retained_unique.len() != RETAINED.len()
            || retained_content != RETAINED_CONTENT_BYTES
            || retained.0 != RETAINED_STREAM_BYTES
            || retained.1 != RETAINED_STREAM_SHA256
            || retained.2 != RETAINED_STREAM_BLAKE3
        {
            return Err(format!(
                "{LABEL} record identities changed: historical={historical:?}/{historical_content}; retained={retained:?}/{retained_content}"
            ));
        }
        Ok(())
    }

    fn read_bug440_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug440_record_streams()?;
        let directory = self.bug440_family_directory(abort)?;
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

    fn validate_bug440_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug440Role,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("{LABEL} recognized owner is not canonical"));
        }
        let owners = self.read_bug440_manifest_owners(abort)?;
        let expected_owners = BTreeSet::from([
            DEFERRED_OWNER_RECORD.to_string(),
            Bug440Role::Resistor.record().to_string(),
            Bug440Role::Behavioral.record().to_string(),
        ]);
        if owners != expected_owners {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        self.preflight_bug440_exclusions(abort)?;
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions invalid: {error}"))?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} exclusions validation aborted"));
        }
        let family_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .collect::<Vec<_>>();
        if !family_exclusions.is_empty() {
            return Err(format!(
                "{LABEL} family must not be excluded: {family_exclusions:?}"
            ));
        }
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        for member in RETAINED {
            self.reject_wrapper_output_artifacts(&self.root.join(FAMILY_DIRECTORY).join(member.0))
                .map_err(|error| format!("{LABEL} {} {error}", member.0))?;
        }
        self.read_bug440_directory(abort)
    }

    fn read_bug440_manifest_owners(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeSet<String>, String> {
        let path = Self::bug440_exact_child(&self.root, HARNESS_MANIFEST_FILE, abort)?;
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect {LABEL} harness manifest: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "{LABEL} harness manifest must be a regular non-symlink file"
            ));
        }
        if metadata.len() > MAX_MANIFEST_BYTES as u64 {
            return Err(format!(
                "{LABEL} harness manifest exceeds its bounded envelope"
            ));
        }
        let mut bytes = Vec::with_capacity((metadata.len() as usize).min(MAX_MANIFEST_BYTES));
        fs::File::open(&path)
            .map_err(|error| format!("failed to open {LABEL} harness manifest: {error}"))?
            .take((MAX_MANIFEST_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("failed to read {LABEL} harness manifest: {error}"))?;
        if bytes.len() > MAX_MANIFEST_BYTES || abort.is_aborted() {
            return Err(format!(
                "{LABEL} harness manifest read exceeded its envelope"
            ));
        }
        let canonical =
            Self::canonical_lf_text_identity(&format!("{LABEL} harness manifest"), &bytes)?;
        let source = std::str::from_utf8(&canonical)
            .map_err(|error| format!("{LABEL} harness manifest is not UTF-8: {error}"))?;
        let expected = BTreeSet::from([
            format!("{FAMILY_DIRECTORY}/bug440.cir"),
            format!("{FAMILY_DIRECTORY}/bug440a.cir"),
            format!("{FAMILY_DIRECTORY}/bug440b.cir"),
        ]);
        let mut owners = BTreeSet::new();
        for (index, line) in source.lines().enumerate() {
            if index % 256 == 0 && abort.is_aborted() {
                return Err(format!("{LABEL} harness manifest validation aborted"));
            }
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let family_candidate = line
                .replace('\\', "/")
                .to_ascii_lowercase()
                .contains(FAMILY_PREFIX);
            let Some((path, contract)) = line.split_once('\t') else {
                if family_candidate {
                    return Err(format!(
                        "{LABEL} harness manifest line {} is malformed",
                        index + 1
                    ));
                }
                continue;
            };
            if !family_candidate {
                continue;
            }
            if !expected.contains(path)
                || contract != REQUIRES_UPSTREAM_WRAPPER_CONTRACT
                || line != format!("{path}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}")
                || !owners.insert(Self::normalize_manifest_key(path))
            {
                return Err(format!(
                    "{LABEL} harness manifest line {} changed: {line:?}",
                    index + 1
                ));
            }
        }
        let expected = BTreeSet::from([
            DEFERRED_OWNER_RECORD.to_string(),
            Bug440Role::Resistor.record().to_string(),
            Bug440Role::Behavioral.record().to_string(),
        ]);
        if owners != expected {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        Ok(owners)
    }

    fn preflight_bug440_exclusions(&self, abort: &dyn AbortSignal) -> Result<(), String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} exclusions preflight aborted"));
        }
        let vendoring = Self::find_bug440_exact_child(&self.root, "RSPICE-VENDORING.md", abort)?;
        let manifest =
            Self::find_bug440_exact_child(&self.root, UPSTREAM_EXCLUSIONS_MANIFEST_FILE, abort)?;
        let Some(path) = manifest else {
            if vendoring.is_some() {
                return Err(format!(
                    "{LABEL} vendored corpus lost its exclusions manifest"
                ));
            }
            return Ok(());
        };
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect {LABEL} exclusions manifest: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "{LABEL} exclusions manifest must be a regular non-symlink file"
            ));
        }
        if metadata.len() > MAX_MANIFEST_BYTES as u64 {
            return Err(format!(
                "{LABEL} exclusions manifest has an invalid bounded length: {}",
                metadata.len()
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} exclusions preflight aborted"));
        }
        Ok(())
    }

    fn validate_bug440_source(
        &self,
        role: Bug440Role,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if Self::contains_control_block(source) {
            return Err(format!(
                "{LABEL} does not admit a simulator scripting block"
            ));
        }
        Self::reject_unsupported_source_directives(source)?;
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} owner no longer parses: {error}"))?;
        let expected_elements = match role {
            Bug440Role::Resistor => 3,
            Bug440Role::Behavioral => 4,
        };
        let expected_title = match role {
            Bug440Role::Resistor => "test of resistor lead current in .measure",
            Bug440Role::Behavioral => "Lead current test for BSRC",
        };
        let mut expected_options = SimulationOptions::default();
        if role == Bug440Role::Behavioral {
            expected_options.method = Some("TRAP".to_string());
            expected_options.nonlin_transient_rhstol = Some(1.0e-7);
        }
        if netlist.elements.len() != expected_elements
            || netlist.title != expected_title
            || netlist.analyses.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || netlist.output_requests.len() != 3
            || netlist.measurements.len() != 2
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.control_dispositions.is_empty()
            || netlist.source_text.as_deref() != Some(source)
            || netlist.source_path.as_deref() != Some(path)
            || format!("{:?}", netlist.options) != format!("{expected_options:?}")
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} typed envelope changed for {role:?}: elements={:?}, analyses={:?}, outputs={:?}, measures={:?}, diagnostics={:?}",
                netlist.elements,
                netlist.analyses,
                netlist.output_requests,
                netlist.measurements,
                netlist.diagnostics
            ));
        }
        if !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
        }] if step.to_bits() == role.tran_step().to_bits()
            && stop.to_bits() == role.tran_stop().to_bits())
        {
            return Err(format!(
                "{LABEL} typed .TRAN changed: {:?}",
                netlist.analyses
            ));
        }
        Self::validate_bug440_topology(role, &netlist)?;
        validate_output_symbols(&netlist)
            .map_err(|error| format!("{LABEL} output symbols do not resolve: {error}"))?;
        let tran = Self::single_tran_analysis(&netlist)?;
        let print = Self::single_tran_print_output_request(source)?;
        let print_line = match role {
            Bug440Role::Resistor => 7,
            Bug440Role::Behavioral => 23,
        };
        if !Self::validate_bug440_print_output(&netlist.output_requests[0], path, role, print_line)
            || !Self::validate_bug440_saves(&netlist.saves.signals, role)
            || print.probes.len() != role.expected_probes().len()
            || !print
                .probes
                .iter()
                .zip(role.expected_probes())
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
            || !Self::validate_bug440_measure(&netlist.measurements[0], role, 0)
            || !Self::validate_bug440_measure(&netlist.measurements[1], role, 1)
            || !Self::validate_bug440_measure_output(&netlist.output_requests[1], path, role, 0)
            || !Self::validate_bug440_measure_output(&netlist.output_requests[2], path, role, 1)
        {
            return Err(format!("{LABEL} print/measure schema changed for {role:?}"));
        }
        let plan = XyceStaticTranPlan {
            deck_path: path.to_path_buf(),
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: print.probes,
            }),
            output_override: false,
            timeint_conststep: false,
            tran,
            steps: Vec::new(),
            contract: XyceStaticTranContract::WrapperStatic,
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        if plan.deck_path != path
            || plan.source != source
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != role.expected_probes().len()
                    || print
                        .probes
                        .iter()
                        .zip(role.expected_probes())
                        .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            })
            || plan.tran.step.to_bits() != role.tran_step().to_bits()
            || plan.tran.stop.to_bits() != role.tran_stop().to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{LABEL} native transient plan changed: {plan:?}"));
        }
        Ok((plan, netlist))
    }

    fn bug440_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn bug440_resistor_matches(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: &[&str],
        value: Value,
    ) -> bool {
        element.provenance == ElementProvenance::Authored
            && element.name.eq_ignore_ascii_case(name)
            && Self::bug440_nodes_match(&element.nodes, nodes)
            && matches!(&element.kind, ElementKind::Resistor {
                value: actual,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
    }

    fn validate_bug440_topology(role: Bug440Role, netlist: &Netlist) -> Result<(), String> {
        let valid = match (role, netlist.elements.as_slice()) {
            (Bug440Role::Resistor, [vdc, vcos, rtest]) => {
                vdc.provenance == ElementProvenance::Authored
                    && vdc.name.eq_ignore_ascii_case("VDC")
                    && Self::bug440_nodes_match(&vdc.nodes, &["1a", "0"])
                    && matches!(&vdc.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                        if value.to_bits() == 1.0f64.to_bits())
                    && vcos.provenance == ElementProvenance::Authored
                    && vcos.name.eq_ignore_ascii_case("VCOS")
                    && Self::bug440_nodes_match(&vcos.nodes, &["1", "1a"])
                    && matches!(&vcos.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                        offset, amplitude, frequency, delay, damping, phase,
                    }) if offset.to_bits() == 0.0f64.to_bits()
                        && amplitude.to_bits() == 5.0f64.to_bits()
                        && frequency.to_bits() == RESISTOR_SOURCE_FREQUENCY.to_bits()
                        && delay.to_bits() == (-RESISTOR_SOURCE_ADVANCE).to_bits()
                        && damping.to_bits() == 0.0f64.to_bits()
                        && phase.to_bits() == 0.0f64.to_bits())
                    && Self::bug440_resistor_matches(rtest, "RTEST", &["1", "0"], 500.0)
            }
            (Bug440Role::Behavioral, [vs, r1, b2, r2]) => {
                let expected_points = (0..=11)
                    .map(|index| (index as Value, (index * index) as Value))
                    .collect::<Vec<_>>();
                vs.provenance == ElementProvenance::Authored
                    && vs.name.eq_ignore_ascii_case("VS")
                    && Self::bug440_nodes_match(&vs.nodes, &["1", "0"])
                    && matches!(&vs.kind, ElementKind::VoltageSource(SourceSpec::Pwl {
                        points,
                        delay,
                        repeat_from: None,
                    }) if points == &expected_points && delay.to_bits() == 0.0f64.to_bits())
                    && Self::bug440_resistor_matches(r1, "R1", &["0", "1"], 1.0)
                    && b2.provenance == ElementProvenance::Authored
                    && b2.name.eq_ignore_ascii_case("B2")
                    && Self::bug440_nodes_match(&b2.nodes, &["2", "0"])
                    && matches!(&b2.kind, ElementKind::BehavioralVoltage {
                        expression,
                        tc1,
                        tc2,
                        multiplicity,
                    } if expression.split_whitespace().collect::<String>().eq_ignore_ascii_case("sqrt(v(1))")
                        && tc1.to_bits() == 0.0f64.to_bits()
                        && tc2.to_bits() == 0.0f64.to_bits()
                        && multiplicity.value.to_bits() == 1.0f64.to_bits()
                        && multiplicity.value_expr.is_none()
                        && !multiplicity.given)
                    && Self::bug440_resistor_matches(r2, "R2", &["0", "2"], 1.0)
            }
            _ => false,
        };
        if !valid {
            return Err(format!(
                "{LABEL} exact authored topology changed for {role:?}: {:?}",
                netlist.elements
            ));
        }
        Ok(())
    }

    fn bug440_dependency_matches(
        dependency: &OutputSymbolDependency,
        kind: OutputSymbolKind,
        symbol: &str,
        expression: bool,
    ) -> bool {
        dependency.kind == kind
            && dependency
                .operator
                .eq_ignore_ascii_case(if kind == OutputSymbolKind::Node {
                    "V"
                } else {
                    "I"
                })
            && dependency.symbol.eq_ignore_ascii_case(symbol)
            && dependency.expression == expression
    }

    fn validate_bug440_print_output(
        output: &OutputRequest,
        path: &Path,
        role: Bug440Role,
        line: usize,
    ) -> bool {
        let dependencies_match = match (role, output.dependencies.as_slice()) {
            (Bug440Role::Resistor, [voltage, current]) => {
                Self::bug440_dependency_matches(voltage, OutputSymbolKind::Node, "1", false)
                    && Self::bug440_dependency_matches(
                        current,
                        OutputSymbolKind::Device,
                        "RTEST",
                        false,
                    )
            }
            (Bug440Role::Behavioral, [r1, b2]) => {
                Self::bug440_dependency_matches(r1, OutputSymbolKind::Device, "R1", false)
                    && Self::bug440_dependency_matches(b2, OutputSymbolKind::Device, "B2", false)
            }
            _ => false,
        };
        let saves_match = role.expected_probes();
        output.directive == OutputDirectiveKind::Print
            && output.analysis == Some(OutputAnalysisKind::Tran)
            && output.origin.line == line
            && output
                .origin
                .path
                .as_deref()
                .is_some_and(|origin| Self::same_path(origin, path))
            && output.name.is_none()
            && output.print_delimiter.as_ref() == Some(&PrintDelimiter::Whitespace)
            && output.print_precision.is_none()
            && output.print_width.is_none()
            && output.operands.len() == saves_match.len()
            && output
                .operands
                .iter()
                .zip(saves_match)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
            && output.expressions.is_empty()
            && dependencies_match
    }

    fn validate_bug440_saves(signals: &[SaveSignal], role: Bug440Role) -> bool {
        match (role, signals) {
            (Bug440Role::Resistor, [SaveSignal::Voltage(node), SaveSignal::Current(device)]) => {
                node.eq_ignore_ascii_case("1") && device.eq_ignore_ascii_case("RTEST")
            }
            (Bug440Role::Behavioral, [SaveSignal::Current(r1), SaveSignal::Current(b2)]) => {
                r1.eq_ignore_ascii_case("R1") && b2.eq_ignore_ascii_case("B2")
            }
            _ => false,
        }
    }

    fn validate_bug440_measure(
        statement: &MeasureStatement,
        role: Bug440Role,
        index: usize,
    ) -> bool {
        let expected_name = role.expected_measure_names()[index];
        let expected_signal = match (role, index) {
            (Bug440Role::Resistor, 0) => "{I(rtest)}",
            (Bug440Role::Resistor, 1) => "I(rtest)",
            (Bug440Role::Behavioral, 0) => "I(R1)",
            (Bug440Role::Behavioral, 1) => "I(B2)",
            _ => return false,
        };
        statement.name.eq_ignore_ascii_case(expected_name)
            && statement.analysis.eq_ignore_ascii_case("TRAN")
            && statement.goal.is_none()
            && statement.tolerance.is_none()
            && statement.default_value.is_none()
            && statement.print_policy == MeasurePrintPolicy::All
            && (matches!((&statement.measure_type, role),
                (MeasureType::Max { signal, from: None, to: None, output: ExtremaOutput::Value }, Bug440Role::Resistor)
                    if signal == expected_signal)
                || matches!((&statement.measure_type, role),
                    (MeasureType::Min { signal, from: None, to: None, output: ExtremaOutput::Value }, Bug440Role::Behavioral)
                        if signal == expected_signal))
    }

    fn validate_bug440_measure_output(
        output: &OutputRequest,
        path: &Path,
        role: Bug440Role,
        index: usize,
    ) -> bool {
        let (line, symbol, expression) = match (role, index) {
            (Bug440Role::Resistor, 0) => (9, "RTEST", true),
            (Bug440Role::Resistor, 1) => (10, "RTEST", false),
            (Bug440Role::Behavioral, 0) => (28, "R1", false),
            (Bug440Role::Behavioral, 1) => (29, "B2", false),
            _ => return false,
        };
        output.directive == OutputDirectiveKind::Measure
            && output.analysis == Some(OutputAnalysisKind::Tran)
            && output
                .name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(role.expected_measure_names()[index]))
            && output.origin.line == line
            && output
                .origin
                .path
                .as_deref()
                .is_some_and(|origin| Self::same_path(origin, path))
            && output.print_delimiter.is_none()
            && output.print_precision.is_none()
            && output.print_width.is_none()
            && output.operands.is_empty()
            && output.expressions.is_empty()
            && matches!(output.dependencies.as_slice(), [dependency]
                if Self::bug440_dependency_matches(dependency, OutputSymbolKind::Device, symbol, expression))
    }

    fn bug440_wrapper_accepts(measured: Value, calculated: Value) -> bool {
        if !measured.is_finite() || !calculated.is_finite() {
            return false;
        }
        let absolute_error = (measured - calculated).abs();
        let relative_error = (calculated != 0.0).then(|| absolute_error / calculated.abs());
        absolute_error <= WRAPPER_ABS_TOL
            && (measured <= WRAPPER_ZERO_TOL
                || relative_error.is_none_or(|error| error <= WRAPPER_REL_TOL))
    }

    fn bug440_mt0_roundtrip(value: Value) -> Result<Value, String> {
        if !value.is_finite() {
            return Err(format!("{LABEL} cannot serialize a non-finite .mt0 value"));
        }
        format!("{value:.6e}")
            .parse::<Value>()
            .map_err(|error| format!("{LABEL} failed to parse serialized .mt0 value: {error}"))
    }

    fn validate_bug440_production_measures(
        role: Bug440Role,
        measurements: &[MeasureResult],
    ) -> Result<[Value; 2], String> {
        if measurements.len() != 2 {
            return Err(format!(
                "{LABEL} production measure census changed: {measurements:?}"
            ));
        }
        let mut values = [0.0; 2];
        for (index, (measurement, expected_name)) in measurements
            .iter()
            .zip(role.expected_measure_names())
            .enumerate()
        {
            let value = measurement.value.ok_or_else(|| {
                format!("{LABEL} production measure {expected_name} has no value: {measurement:?}")
            })?;
            if !measurement.name.eq_ignore_ascii_case(expected_name)
                || !measurement.passed
                || measurement.error.is_some()
                || measurement.expected.is_some()
                || measurement.tolerance.is_some()
                || !value.is_finite()
            {
                return Err(format!(
                    "{LABEL} production measure changed: {measurement:?}"
                ));
            }
            values[index] = value;
        }
        Ok(values)
    }

    fn bug440_resistor_voltage(time: Value) -> Value {
        1.0 + 5.0
            * (std::f64::consts::TAU * RESISTOR_SOURCE_FREQUENCY * (time + RESISTOR_SOURCE_ADVANCE))
                .sin()
    }

    fn bug440_behavioral_pwl_voltage(time: Value) -> Value {
        if time <= 0.0 {
            return 0.0;
        }
        if time >= 11.0 {
            return 121.0;
        }
        let left = time.floor();
        let fraction = time - left;
        let left_value = left * left;
        let right_value = (left + 1.0) * (left + 1.0);
        left_value + fraction * (right_value - left_value)
    }

    fn bug440_prn_rounding_bound(value: Value) -> Value {
        if value == 0.0 {
            return Value::EPSILON;
        }
        let exponent = value.abs().log10().floor() as i32 - PRN_SCIENTIFIC_PRECISION;
        0.5 * 10.0f64.powi(exponent) + 8.0 * Value::EPSILON * value.abs()
    }

    fn validate_bug440_physics(
        role: Bug440Role,
        result: &TransientResult,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        if result.time.len() < 10
            || result.step_sizes.len() != result.time.len()
            || result.time.iter().any(|value| !value.is_finite())
            || result
                .step_sizes
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
            || result.time.windows(2).any(|pair| pair[1] <= pair[0])
        {
            return Err(format!("{LABEL} native transient result shape changed"));
        }
        let voltage = |name: &str| {
            result
                .try_voltage_waveform_named(name)
                .ok_or_else(|| format!("{LABEL} lost V({name})"))
        };
        let current = |name: &str| {
            result
                .try_branch_current_waveform_named(name)
                .ok_or_else(|| format!("{LABEL} lost I({name})"))
        };
        match role {
            Bug440Role::Resistor => {
                let v1 = voltage("1")?;
                let v1a = voltage("1a")?;
                let rtest = current("RTEST")?;
                let vcos = current("VCOS")?;
                let vdc = current("VDC")?;
                let traces = [v1, v1a, rtest, vcos, vdc];
                if traces.iter().any(|trace| {
                    trace.len() != result.time.len() || trace.iter().any(|value| !value.is_finite())
                }) {
                    return Err(format!("{LABEL} resistor trace length changed"));
                }
                let mut source_error: Value = 0.0;
                let mut constitutive_error: Value = 0.0;
                let mut kcl_error: Value = 0.0;
                for index in 0..result.time.len() {
                    if index % 256 == 0 && abort.is_aborted() {
                        return Err(format!("{LABEL} resistor physics validation aborted"));
                    }
                    let expected = Self::bug440_resistor_voltage(result.time[index]);
                    source_error = source_error
                        .max((v1a[index] - 1.0).abs())
                        .max((v1[index] - expected).abs());
                    constitutive_error =
                        constitutive_error.max((rtest[index] - v1[index] / 500.0).abs());
                    kcl_error = kcl_error
                        .max((vcos[index] + rtest[index]).abs())
                        .max((vdc[index] - vcos[index]).abs());
                }
                let min = rtest.iter().copied().fold(Value::INFINITY, Value::min);
                let max = rtest.iter().copied().fold(Value::NEG_INFINITY, Value::max);
                if source_error > 2.0e-9
                    || constitutive_error > 2.0e-10
                    || kcl_error > 2.0e-10
                    || min > -7.0e-3
                    || max < 1.1e-2
                {
                    return Err(format!(
                        "{LABEL} resistor physics changed: source={source_error:e}, constitutive={constitutive_error:e}, kcl={kcl_error:e}, range={min:e}..{max:e}"
                    ));
                }
            }
            Bug440Role::Behavioral => {
                let v1 = voltage("1")?;
                let v2 = voltage("2")?;
                let r1 = current("R1")?;
                let b2 = current("B2")?;
                let vs = current("VS")?;
                let r2 = current("R2")?;
                let traces = [v1, v2, r1, b2, vs, r2];
                if traces.iter().any(|trace| {
                    trace.len() != result.time.len() || trace.iter().any(|value| !value.is_finite())
                }) {
                    return Err(format!("{LABEL} behavioral trace length changed"));
                }
                let mut source_error: Value = 0.0;
                let mut behavioral_error: Value = 0.0;
                let mut constitutive_error: Value = 0.0;
                let mut kcl_error: Value = 0.0;
                for index in 0..result.time.len() {
                    if index % 256 == 0 && abort.is_aborted() {
                        return Err(format!("{LABEL} behavioral physics validation aborted"));
                    }
                    let expected_v1 = Self::bug440_behavioral_pwl_voltage(result.time[index]);
                    source_error = source_error.max((v1[index] - expected_v1).abs());
                    behavioral_error = behavioral_error.max((v2[index] - expected_v1.sqrt()).abs());
                    constitutive_error = constitutive_error
                        .max((r1[index] + v1[index]).abs())
                        .max((r2[index] + v2[index]).abs())
                        .max((b2[index] + v2[index]).abs());
                    kcl_error = kcl_error
                        .max((vs[index] - r1[index]).abs())
                        .max((b2[index] - r2[index]).abs());
                }
                let r1_min = r1.iter().copied().fold(Value::INFINITY, Value::min);
                let b2_min = b2.iter().copied().fold(Value::INFINITY, Value::min);
                if source_error > 2.0e-8
                    || behavioral_error > 2.0e-8
                    || constitutive_error > 2.0e-8
                    || kcl_error > 2.0e-8
                    || (r1_min + 121.0).abs() > 2.0e-8
                    || (b2_min + 11.0).abs() > 2.0e-8
                {
                    return Err(format!(
                        "{LABEL} behavioral physics changed: source={source_error:e}, behavioral={behavioral_error:e}, constitutive={constitutive_error:e}, kcl={kcl_error:e}, minima={r1_min:e}/{b2_min:e}"
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_bug440_serialized_wrapper(
        role: Bug440Role,
        table: &XycePrnTable,
        measured: [Value; 2],
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let expected_columns = match role {
            Bug440Role::Resistor => ["Index", "TIME", "V(1)", "I(RTEST)"].as_slice(),
            Bug440Role::Behavioral => ["Index", "TIME", "I(R1)", "I(B2)"].as_slice(),
        };
        if table.columns.len() != expected_columns.len()
            || !table
                .columns
                .iter()
                .zip(expected_columns)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
            || !(2..=MAX_STREAM_ROWS).contains(&table.rows.len())
        {
            return Err(format!(
                "{LABEL} serialized PRN schema changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        let current_columns = match role {
            Bug440Role::Resistor => [3usize, 3usize],
            Bug440Role::Behavioral => [2usize, 3usize],
        };
        let mut extrema = match role {
            Bug440Role::Resistor => [Value::NEG_INFINITY; 2],
            Bug440Role::Behavioral => [Value::INFINITY; 2],
        };
        let time_tolerance = role.tran_step().abs().max(1.0) * 1.0e-12;
        for (index, row) in table.rows.iter().enumerate() {
            if abort.is_aborted() {
                return Err(format!("{LABEL} serialized wrapper validation aborted"));
            }
            if row.len() != expected_columns.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || index > 0 && row[1] <= table.rows[index - 1][1]
            {
                return Err(format!(
                    "{LABEL} malformed serialized PRN row {index}: {row:?}"
                ));
            }
            let expected_time = row[1];
            let time_bound = Self::bug440_prn_rounding_bound(expected_time);
            match role {
                Bug440Role::Resistor => {
                    let expected_voltage = Self::bug440_resistor_voltage(expected_time);
                    let source_time_bound =
                        [expected_time - time_bound, expected_time + time_bound]
                            .into_iter()
                            .map(Self::bug440_resistor_voltage)
                            .map(|value| (value - expected_voltage).abs())
                            .fold(0.0, Value::max);
                    let voltage_bound = source_time_bound + Self::bug440_prn_rounding_bound(row[2]);
                    let current_bound = Self::bug440_prn_rounding_bound(row[3])
                        + Self::bug440_prn_rounding_bound(row[2]) / 500.0;
                    if (row[2] - expected_voltage).abs() > voltage_bound
                        || (row[3] - row[2] / 500.0).abs() > current_bound
                    {
                        return Err(format!(
                            "{LABEL} serialized resistor physics changed at row {index}: {row:?}"
                        ));
                    }
                }
                Bug440Role::Behavioral => {
                    let expected_voltage = Self::bug440_behavioral_pwl_voltage(expected_time);
                    let time_extrema = [expected_time - time_bound, expected_time + time_bound]
                        .map(Self::bug440_behavioral_pwl_voltage);
                    let voltage_time_bound = time_extrema
                        .into_iter()
                        .map(|value| (value - expected_voltage).abs())
                        .fold(0.0, Value::max);
                    let sqrt_time_bound = time_extrema
                        .into_iter()
                        .map(|value| (value.max(0.0).sqrt() - expected_voltage.sqrt()).abs())
                        .fold(0.0, Value::max);
                    let r1_bound = voltage_time_bound + Self::bug440_prn_rounding_bound(row[2]);
                    let b2_bound = sqrt_time_bound + Self::bug440_prn_rounding_bound(row[3]);
                    if (row[2] + expected_voltage).abs() > r1_bound
                        || (row[3] + expected_voltage.sqrt()).abs() > b2_bound
                    {
                        return Err(format!(
                            "{LABEL} serialized behavioral physics changed at row {index}: {row:?}"
                        ));
                    }
                }
            }
            for output in 0..2 {
                extrema[output] = match role {
                    Bug440Role::Resistor => extrema[output].max(row[current_columns[output]]),
                    Bug440Role::Behavioral => extrema[output].min(row[current_columns[output]]),
                };
            }
        }
        let first = table.rows.first().expect("BUG440 row count was checked");
        let last = table.rows.last().expect("BUG440 row count was checked");
        if first[1].abs() > time_tolerance || (last[1] - role.tran_stop()).abs() > time_tolerance {
            return Err(format!(
                "{LABEL} serialized TIME endpoints changed: first={first:?}, last={last:?}"
            ));
        }
        for index in 0..2 {
            if !Self::bug440_wrapper_accepts(measured[index], extrema[index]) {
                return Err(format!(
                    "{LABEL} historical wrapper rejected measured={:e}, calculated={:e}",
                    measured[index], extrema[index]
                ));
            }
        }
        Ok(())
    }

    fn run_bug440_native(
        &self,
        role: Bug440Role,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        start: Instant,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let engine = self
            .create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(&plan.tran));
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|error| format!("{LABEL} production assembly failed: {error}"))?;
        if circuit.has_generated_veriloga_devices() {
            return Err(format!(
                "{LABEL} unexpectedly routed through a generated device"
            ));
        }
        let max_step = Self::transient_family_max_step(netlist, &plan.tran)?;
        let result = engine
            .run_tran_with_startup_mode_and_abort(
                netlist,
                plan.tran.stop,
                max_step,
                TransientStartupMode::OperatingPoint,
                abort,
            )
            .map_err(|error| format!("{LABEL} native transient failed: {error}"))?;
        let quality = engine.convergence_quality();
        if quality.gmin_stepping_count != 0
            || quality.source_stepping_count != 0
            || quality.force_accepted_points != 0
            || !quality.force_accepted_indices.is_empty()
            || quality.failure_diagnostic.is_some()
        {
            return Err(format!(
                "{LABEL} returned an unqualified transient: {quality:?}"
            ));
        }
        Self::validate_bug440_physics(role, &result, abort)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired after physics validation"));
        }
        let measurements = evaluate_tran_measurements(netlist, &result);
        let measured = Self::validate_bug440_production_measures(role, &measurements)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired after measurement evaluation"
            ));
        }
        let table = Self::transient_family_result_to_prn_table(plan, netlist, &result)
            .map_err(|error| format!("{LABEL} PRN projection failed: {error}"))?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired after PRN projection"));
        }
        let raw = serialize_xyce_prn_sequence(
            std::slice::from_ref(&table),
            &netlist.output_requests[0],
            &netlist.options,
            XycePrnFooter::Simulation,
            XycePrnLimits::new(MAX_STREAM_ROWS, MAX_STREAM_BYTES),
        )
        .map_err(|error| format!("{LABEL} PRN serialization failed: {error}"))?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired after PRN serialization"));
        }
        if raw.len() > MAX_STREAM_BYTES || !raw.ends_with("End of Xyce(TM) Simulation\n") {
            return Err(format!("{LABEL} production PRN framing changed"));
        }
        let parsed = Self::parse_prn_table(&raw)
            .map_err(|error| format!("{LABEL} serialized PRN failed to parse: {error}"))?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired after PRN parsing"));
        }
        let serialized_measured = [
            Self::bug440_mt0_roundtrip(measured[0])?,
            Self::bug440_mt0_roundtrip(measured[1])?,
        ];
        Self::validate_bug440_serialized_wrapper(role, &parsed, serialized_measured, abort)?;
        if abort.is_aborted()
            || start.elapsed().as_millis() > self.config.max_time_per_test_ms.max(1)
        {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        Ok(())
    }

    pub(super) fn validate_bug440_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug440Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let before = self.validate_bug440_provenance(deck, role, &abort)?;
        let owner = before
            .get(&role.file_name().to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost sealed owner"))?;
        let source = std::str::from_utf8(owner)
            .map_err(|error| format!("{LABEL} owner is not UTF-8: {error}"))?;
        let (plan, netlist) = self.validate_bug440_source(role, source, &deck.path)?;
        self.run_bug440_native(role, &plan, &netlist, start, &abort)?;
        let after = self.validate_bug440_provenance(deck, role, &abort)?;
        if before != after {
            return Err(format!("{LABEL} sealed sources changed during execution"));
        }
        if abort.is_aborted()
            || start.elapsed().as_millis() > self.config.max_time_per_test_ms.max(1)
        {
            return Err(format!("{LABEL} final provenance exceeded deadline"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_result(role: Bug440Role) -> TransientResult {
        let row_count = match role {
            Bug440Role::Resistor => 88,
            Bug440Role::Behavioral => 13,
        };
        let step = role.tran_stop() / (row_count - 1) as Value;
        let time = (0..row_count)
            .map(|index| index as Value * step)
            .collect::<Vec<_>>();
        let step_sizes = (0..time.len())
            .map(|index| if index == 0 { 0.0 } else { step })
            .collect::<Vec<_>>();
        let (node_names, voltages, branch_names, branch_currents) = match role {
            Bug440Role::Resistor => {
                let v1 = time
                    .iter()
                    .map(|time| XyceTestRunner::bug440_resistor_voltage(*time))
                    .collect::<Vec<_>>();
                let v1a = vec![1.0; time.len()];
                let resistor = v1.iter().map(|value| value / 500.0).collect::<Vec<_>>();
                let source = resistor.iter().map(|value| -*value).collect::<Vec<_>>();
                (
                    vec!["1".to_string(), "1a".to_string()],
                    vec![v1, v1a],
                    vec!["RTEST".to_string(), "VCOS".to_string(), "VDC".to_string()],
                    vec![resistor, source.clone(), source],
                )
            }
            Bug440Role::Behavioral => {
                let v1 = time
                    .iter()
                    .map(|time| XyceTestRunner::bug440_behavioral_pwl_voltage(*time))
                    .collect::<Vec<_>>();
                let v2 = v1.iter().map(|value| value.sqrt()).collect::<Vec<_>>();
                let r1 = v1.iter().map(|value| -*value).collect::<Vec<_>>();
                let b2 = v2.iter().map(|value| -*value).collect::<Vec<_>>();
                (
                    vec!["1".to_string(), "2".to_string()],
                    vec![v1, v2],
                    vec![
                        "VS".to_string(),
                        "R1".to_string(),
                        "B2".to_string(),
                        "R2".to_string(),
                    ],
                    vec![r1.clone(), r1, b2.clone(), b2],
                )
            }
        };
        TransientResult {
            time,
            step_sizes,
            num_nodes: node_names.len(),
            node_names,
            voltages,
            branch_names,
            branch_currents,
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    fn serialized_table(role: Bug440Role) -> XycePrnTable {
        let result = synthetic_result(role);
        match role {
            Bug440Role::Resistor => XycePrnTable {
                columns: vec![
                    "Index".to_string(),
                    "TIME".to_string(),
                    "V(1)".to_string(),
                    "I(RTEST)".to_string(),
                ],
                rows: result
                    .time
                    .iter()
                    .enumerate()
                    .map(|(index, time)| {
                        vec![
                            index as Value,
                            *time,
                            result.voltages[0][index],
                            result.branch_currents[0][index],
                        ]
                    })
                    .collect(),
            },
            Bug440Role::Behavioral => XycePrnTable {
                columns: vec![
                    "Index".to_string(),
                    "TIME".to_string(),
                    "I(R1)".to_string(),
                    "I(B2)".to_string(),
                ],
                rows: result
                    .time
                    .iter()
                    .enumerate()
                    .map(|(index, time)| {
                        vec![
                            index as Value,
                            *time,
                            result.branch_currents[1][index],
                            result.branch_currents[2][index],
                        ]
                    })
                    .collect(),
            },
        }
    }

    fn measure_result(name: &str, value: Value) -> MeasureResult {
        MeasureResult {
            name: name.to_string(),
            value: Some(value),
            raw_value: Some(value),
            error: None,
            passed: true,
            expected: None,
            tolerance: None,
            failure_limit: None,
            failure_limit_exceeded: false,
            event_axis: Some(0.0),
        }
    }

    fn bug440_fixture() -> (tempfile::TempDir, XyceTestRunner) {
        let temporary = tempfile::tempdir().unwrap();
        let source_root = corpus_root();
        let family = temporary.path().join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).unwrap();
        for (name, ..) in RETAINED {
            fs::copy(
                source_root.join(FAMILY_DIRECTORY).join(name),
                family.join(name),
            )
            .unwrap();
        }
        fs::copy(
            source_root.join(HARNESS_MANIFEST_FILE),
            temporary.path().join(HARNESS_MANIFEST_FILE),
        )
        .unwrap();
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        (temporary, runner)
    }

    fn bug440_fixture_deck(runner: &XyceTestRunner, role: Bug440Role) -> XyceDeck {
        runner
            .discover_netlist_tests()
            .into_iter()
            .find(|deck| {
                XyceTestRunner::normalize_manifest_key(&deck.relative_path) == role.record()
            })
            .unwrap()
    }

    fn test_abort() -> DeadlineAbort {
        DeadlineAbort::new(Instant::now(), 30_000)
    }

    #[test]
    fn bug440_routes_only_the_two_currently_observable_owners() {
        assert_eq!(
            Bug440Role::for_record(Bug440Role::Resistor.record()),
            Some(Bug440Role::Resistor)
        );
        assert_eq!(
            Bug440Role::for_record(Bug440Role::Behavioral.record()),
            Some(Bug440Role::Behavioral)
        );
        for rejected in [
            DEFERRED_OWNER_RECORD,
            "netlists/certification_tests/bug_440_son/bug440a.cir.sh",
            "netlists/certification_tests/bug_440_son/bug440b.cir.extra",
            "outputdata/certification_tests/bug_440_son/bug440a.cir.prn",
            "netlists/certification_tests/bug_440_son_extra/bug440a.cir",
        ] {
            assert_eq!(Bug440Role::for_record(rejected), None, "claimed {rejected}");
        }
    }

    #[test]
    fn bug440_record_streams_are_exact() {
        XyceTestRunner::validate_bug440_record_streams().unwrap();
    }

    #[test]
    fn bug440_wrapper_predicate_preserves_signed_zero_gate() {
        assert!(XyceTestRunner::bug440_wrapper_accepts(-11.0, -11.0));
        assert!(!XyceTestRunner::bug440_wrapper_accepts(-11.0, -10.999_998));
        assert!(XyceTestRunner::bug440_wrapper_accepts(1.0, 1.0 + 0.5e-6));
        assert!(!XyceTestRunner::bug440_wrapper_accepts(1.0, 1.0 + 2.0e-6));
        assert!(!XyceTestRunner::bug440_wrapper_accepts(Value::NAN, 0.0));
        assert!(XyceTestRunner::bug440_wrapper_accepts(-1.0e-6, 0.0));
        assert!(!XyceTestRunner::bug440_wrapper_accepts(-1.01e-6, 0.0));
        assert!(XyceTestRunner::bug440_wrapper_accepts(5.1e-5, 5.0e-5));
        assert!(!XyceTestRunner::bug440_wrapper_accepts(4.09e-5, 4.0e-5));
        assert!(!XyceTestRunner::bug440_wrapper_accepts(
            Value::INFINITY,
            0.0
        ));
    }

    #[test]
    fn bug440_physics_rejects_nonfinite_corrupted_sources_and_kcl() {
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        for role in Bug440Role::ALL {
            XyceTestRunner::validate_bug440_physics(role, &synthetic_result(role), &abort).unwrap();
        }

        let mut nonfinite = synthetic_result(Bug440Role::Resistor);
        nonfinite.voltages[1][2] = Value::NAN;
        assert!(
            XyceTestRunner::validate_bug440_physics(Bug440Role::Resistor, &nonfinite, &abort)
                .is_err()
        );
        let mut wrong_sine = synthetic_result(Bug440Role::Resistor);
        wrong_sine.voltages[0][3] += 0.25;
        assert!(
            XyceTestRunner::validate_bug440_physics(Bug440Role::Resistor, &wrong_sine, &abort)
                .is_err()
        );
        let mut wrong_resistor_kcl = synthetic_result(Bug440Role::Resistor);
        wrong_resistor_kcl.branch_currents[1][4] += 1.0e-3;
        assert!(
            XyceTestRunner::validate_bug440_physics(
                Bug440Role::Resistor,
                &wrong_resistor_kcl,
                &abort
            )
            .is_err()
        );

        let mut wrong_pwl = synthetic_result(Bug440Role::Behavioral);
        wrong_pwl.voltages[0][5] += 1.0;
        assert!(
            XyceTestRunner::validate_bug440_physics(Bug440Role::Behavioral, &wrong_pwl, &abort)
                .is_err()
        );
        let mut nonfinite_behavioral = synthetic_result(Bug440Role::Behavioral);
        nonfinite_behavioral.voltages[1][4] = Value::NAN;
        assert!(
            XyceTestRunner::validate_bug440_physics(
                Bug440Role::Behavioral,
                &nonfinite_behavioral,
                &abort
            )
            .is_err()
        );
        let mut wrong_behavioral_kcl = synthetic_result(Bug440Role::Behavioral);
        wrong_behavioral_kcl.branch_currents[0][6] += 1.0;
        assert!(
            XyceTestRunner::validate_bug440_physics(
                Bug440Role::Behavioral,
                &wrong_behavioral_kcl,
                &abort
            )
            .is_err()
        );
    }

    #[test]
    fn bug440_serialized_wrapper_rejects_schema_grid_and_extrema_counterfactuals() {
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let resistor_measured = [12.0e-3, 12.0e-3];
        let behavioral_measured = [-121.0, -11.0];
        XyceTestRunner::validate_bug440_serialized_wrapper(
            Bug440Role::Resistor,
            &serialized_table(Bug440Role::Resistor),
            resistor_measured,
            &abort,
        )
        .unwrap();
        XyceTestRunner::validate_bug440_serialized_wrapper(
            Bug440Role::Behavioral,
            &serialized_table(Bug440Role::Behavioral),
            behavioral_measured,
            &abort,
        )
        .unwrap();

        let assert_rejected = |role, table: XycePrnTable, measured| {
            assert!(
                XyceTestRunner::validate_bug440_serialized_wrapper(role, &table, measured, &abort)
                    .is_err()
            );
        };
        let mut wrong_column = serialized_table(Bug440Role::Resistor);
        wrong_column.columns[3] = "I(OTHER)".to_string();
        assert_rejected(Bug440Role::Resistor, wrong_column, resistor_measured);
        let mut wrong_grid = serialized_table(Bug440Role::Resistor);
        wrong_grid.rows[1][1] += 0.25e-6;
        assert_rejected(Bug440Role::Resistor, wrong_grid, resistor_measured);
        let mut wrong_index = serialized_table(Bug440Role::Behavioral);
        wrong_index.rows[2][0] = 7.0;
        assert_rejected(Bug440Role::Behavioral, wrong_index, behavioral_measured);
        let mut wrong_count = serialized_table(Bug440Role::Behavioral);
        wrong_count.rows.pop();
        assert_rejected(Bug440Role::Behavioral, wrong_count, behavioral_measured);
        let mut wrong_direction = serialized_table(Bug440Role::Behavioral);
        for row in &mut wrong_direction.rows {
            row[2] = -row[2];
            row[3] = -row[3];
        }
        assert_rejected(Bug440Role::Behavioral, wrong_direction, behavioral_measured);
        let mut swapped_mapping = serialized_table(Bug440Role::Behavioral);
        for row in &mut swapped_mapping.rows {
            row.swap(2, 3);
        }
        assert_rejected(Bug440Role::Behavioral, swapped_mapping, behavioral_measured);
        let mut nonfinite = serialized_table(Bug440Role::Resistor);
        nonfinite.rows[4][3] = Value::NAN;
        assert_rejected(Bug440Role::Resistor, nonfinite, resistor_measured);
    }

    #[test]
    fn bug440_production_measure_validation_rejects_census_order_failure_and_nonfinite() {
        let valid = [
            measure_result("max1", 12.0e-3),
            measure_result("max2", 12.0e-3),
        ];
        XyceTestRunner::validate_bug440_production_measures(Bug440Role::Resistor, &valid).unwrap();
        assert!(
            XyceTestRunner::validate_bug440_production_measures(Bug440Role::Resistor, &valid[..1])
                .is_err()
        );
        let reordered = [valid[1].clone(), valid[0].clone()];
        assert!(
            XyceTestRunner::validate_bug440_production_measures(Bug440Role::Resistor, &reordered)
                .is_err()
        );
        let mut failed = valid.clone();
        failed[0].passed = false;
        failed[0].error = Some("counterfactual failure".to_string());
        assert!(
            XyceTestRunner::validate_bug440_production_measures(Bug440Role::Resistor, &failed)
                .is_err()
        );
        let mut nonfinite = valid;
        nonfinite[1].value = Some(Value::INFINITY);
        assert!(
            XyceTestRunner::validate_bug440_production_measures(Bug440Role::Resistor, &nonfinite)
                .is_err()
        );
    }

    #[test]
    fn bug440_typed_sources_reject_semantic_counterfactuals() {
        let runner = runner();
        for (role, mutations) in [
            (
                Bug440Role::Resistor,
                [
                    ("test of resistor lead current in .measure", "changed title"),
                    ("VDC  1a 0 1.0", "VDC  1a 0 2.0"),
                    ("100K -2.5U", "101K -2.5U"),
                    ("RTEST 1 0 500", "RTEST 1 0 501"),
                    (".TRAN 1US 10US", ".TRAN 2US 10US"),
                    (".print tran v(1)  I(rtest)", ".print tran v(1a) I(rtest)"),
                    ("MAX {I(rtest)}", "MAX I(rtest)"),
                    ("max2 MAX I(rtest)", "max2 MAX I(VCOS)"),
                ]
                .as_slice(),
            ),
            (
                Bug440Role::Behavioral,
                [
                    ("Lead current test for BSRC", "changed title"),
                    ("+ 8S 0.64E2V", "+ 8S 63V"),
                    ("R1   0  1  1", "R1   1  0  1"),
                    ("{SQRT(V(1))}", "{ABS(V(1))}"),
                    ("R2   0  2  1", "R2   2  0  1"),
                    (".TRAN 1S 12S", ".TRAN 2S 12S"),
                    ("I(R1) I(B2)", "I(B2) I(R1)"),
                    ("method=trap", "method=gear"),
                    ("rhstol=1.0e-7", "rhstol=2.0e-7"),
                    ("min2 min I(B2)", "min2 min I(R2)"),
                    ("min1 min I(R1)", "min1 min I(R1) GOAL=1"),
                    ("min1 min I(R1)", "min1 min I(R1) PRINT=NONE"),
                ]
                .as_slice(),
            ),
        ] {
            let path = corpus_root().join(role.path());
            let source = fs::read_to_string(&path).unwrap();
            runner.validate_bug440_source(role, &source, &path).unwrap();
            for (before, after) in mutations {
                let changed = source.replacen(before, after, 1);
                assert_ne!(changed, source, "counterfactual fixture did not change");
                assert!(
                    runner
                        .validate_bug440_source(role, &changed, &path)
                        .is_err(),
                    "accepted {role:?} counterfactual {before:?} -> {after:?}"
                );
            }
        }
    }

    #[test]
    fn bug440_manifest_reader_rejects_case_contract_census_shape_and_size_changes() {
        let valid = [
            format!("{FAMILY_DIRECTORY}/bug440.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"),
            format!("{FAMILY_DIRECTORY}/bug440a.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"),
            format!("{FAMILY_DIRECTORY}/bug440b.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"),
        ]
        .join("\r\n");
        let check = |source: &[u8]| {
            let temporary = tempfile::tempdir().unwrap();
            fs::write(temporary.path().join(HARNESS_MANIFEST_FILE), source).unwrap();
            let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
            runner.read_bug440_manifest_owners(&test_abort())
        };
        check(valid.as_bytes()).unwrap();
        for changed in [
            valid.replace("BUG_440_SON", "bug_440_son"),
            valid.replacen(
                &format!("{FAMILY_DIRECTORY}/bug440a.cir"),
                "Netlists\\Certification_Tests\\BUG_440_SON\\bug440a.cir",
                1,
            ),
            valid.replace(REQUIRES_UPSTREAM_WRAPPER_CONTRACT, "other_contract"),
            valid.replacen(
                "\trequires_upstream_wrapper",
                " requires_upstream_wrapper",
                1,
            ),
            format!("{valid}\r\n{}", valid.lines().next().unwrap()),
            valid.replace(
                &format!("{FAMILY_DIRECTORY}/bug440b.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"),
                "",
            ),
            format!(
                "{valid}\r\n{FAMILY_DIRECTORY}/bug440c.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"
            ),
        ] {
            assert!(check(changed.as_bytes()).is_err(), "accepted {changed:?}");
        }
        let oversized = vec![b'x'; MAX_MANIFEST_BYTES + 1];
        assert!(check(&oversized).is_err());

        let temporary = tempfile::tempdir().unwrap();
        fs::create_dir(temporary.path().join(HARNESS_MANIFEST_FILE)).unwrap();
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(runner.read_bug440_manifest_owners(&test_abort()).is_err());

        let temporary = tempfile::tempdir().unwrap();
        fs::write(temporary.path().join("rspice-harness-manifest.tsv"), &valid).unwrap();
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(runner.read_bug440_manifest_owners(&test_abort()).is_err());
    }

    #[test]
    fn bug440_provenance_rejects_source_census_output_and_exclusion_mutations() {
        let (_temporary, runner) = bug440_fixture();
        for role in Bug440Role::ALL {
            let deck = bug440_fixture_deck(&runner, role);
            runner
                .validate_bug440_provenance(&deck, role, &test_abort())
                .unwrap();
        }

        let (_temporary, runner) = bug440_fixture();
        fs::write(
            runner.root.join(FAMILY_DIRECTORY).join("unexpected.txt"),
            "not an oracle",
        )
        .unwrap();
        let deck = bug440_fixture_deck(&runner, Bug440Role::Resistor);
        assert!(
            runner
                .validate_bug440_provenance(&deck, Bug440Role::Resistor, &test_abort())
                .is_err()
        );

        let (_temporary, runner) = bug440_fixture();
        fs::write(
            runner.root.join(FAMILY_DIRECTORY).join("bug440a.cir"),
            "mutated source\n",
        )
        .unwrap();
        let deck = bug440_fixture_deck(&runner, Bug440Role::Resistor);
        assert!(
            runner
                .validate_bug440_provenance(&deck, Bug440Role::Resistor, &test_abort())
                .is_err()
        );

        let (_temporary, runner) = bug440_fixture();
        fs::create_dir_all(runner.root.join(OUTPUT_DIRECTORY)).unwrap();
        let deck = bug440_fixture_deck(&runner, Bug440Role::Behavioral);
        assert!(
            runner
                .validate_bug440_provenance(&deck, Bug440Role::Behavioral, &test_abort())
                .is_err()
        );

        let (_temporary, runner) = bug440_fixture();
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            vec![b'x'; MAX_MANIFEST_BYTES + 1],
        )
        .unwrap();
        let deck = bug440_fixture_deck(&runner, Bug440Role::Resistor);
        assert!(
            runner
                .validate_bug440_provenance(&deck, Bug440Role::Resistor, &test_abort())
                .is_err()
        );

        let temporary = tempfile::tempdir().unwrap();
        let wrong_case_family = temporary
            .path()
            .join("Netlists/Certification_Tests/bug_440_son");
        fs::create_dir_all(&wrong_case_family).unwrap();
        for (name, ..) in RETAINED {
            fs::copy(
                corpus_root().join(FAMILY_DIRECTORY).join(name),
                wrong_case_family.join(name),
            )
            .unwrap();
        }
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(runner.read_bug440_directory(&test_abort()).is_err());

        let (_temporary, runner) = bug440_fixture();
        let exclusion_path = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        fs::write(
            exclusion_path,
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{FAMILY_DIRECTORY}/bug440b.cir\t{FAMILY_DIRECTORY}/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .unwrap();
        let deck = bug440_fixture_deck(&runner, Bug440Role::Behavioral);
        assert!(
            runner
                .validate_bug440_provenance(&deck, Bug440Role::Behavioral, &test_abort())
                .is_err()
        );
    }

    #[test]
    fn bug440_checked_in_ready_owners_execute_complete_oracles() {
        let runner = runner();
        for role in Bug440Role::ALL {
            let result = runner.run_test(runner.root.join(role.path()));
            assert!(result.passed, "{role:?} failed: {:?}", result.error);
            assert!(!result.expected_unsupported);
            assert!(!result.upstream_excluded);
            assert_eq!(result.contract, role.contract());
        }
        let deferred = runner.run_test(runner.root.join(FAMILY_DIRECTORY).join("bug440.cir"));
        assert!(deferred.passed && deferred.expected_unsupported);
    }

    #[test]
    fn bug440_full_oracle_observes_shared_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        for role in Bug440Role::ALL {
            let deck = runner
                .discover_netlist_tests()
                .into_iter()
                .find(|deck| {
                    XyceTestRunner::normalize_manifest_key(&deck.relative_path) == role.record()
                })
                .unwrap();
            assert!(
                runner
                    .validate_bug440_oracle(&deck, role, Instant::now() - Duration::from_secs(1))
                    .is_err()
            );
        }
    }

    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default())
    }

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce")
    }
}
