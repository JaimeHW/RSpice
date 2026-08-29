use super::*;
use rspice_core::engine::TransientCheckpointEncoding;
use rspice_core::netlist::{SimulationOptions, SourceSpec};
use std::io::Read as _;

const LABEL: &str = "BUG_442 packed/unpacked transmission-line restart wrapper";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_442";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_442/";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_442";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_442/exclude";

pub(super) const BUG442_OWNER_CONTRACT: &str = "bug442_packed_unpacked_tline_restart_wrapper_owner";
pub(super) const BUG442_WORKER_CONTRACT: &str = "bug442_packed_unpacked_tline_restart_worker";

const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const PRETRIM_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";

const HISTORICAL_CONTENT_BYTES: usize = 139_141;
const HISTORICAL_STREAM_BYTES: usize = 4_851;
const HISTORICAL_STREAM_SHA256: &str =
    "b6dfdd86b8f5f017d499b319469d916d884cbd278e7eaef8d47a75ccf325309b";
const HISTORICAL_STREAM_BLAKE3: &str =
    "7882ed220a5cc4622e36138ad7d094cf5b59977ca128a4a710d676245ed8482a";

// Complete Release-7.10 family plus the two scripts that define its dispatch
// and numerical result. Blob identities are from the immutable pre-trim tree;
// content identities are over canonical LF bytes from the upstream checkout.
const HISTORICAL: [(&str, usize, &str, &str, &str); 12] = [
    (
        "Netlists/Certification_Tests/BUG_442/Manifest.txt",
        151,
        "703bc9a6751a12490cd9782ab0716f3299997f6f",
        "f2c116720819ee04db3a307bb127c267639175adb12fd28e9c45a697fbe75c0b",
        "295e7009473494ebd274518f09ebd4b0f0814a59cf19b3d98d99f749e0985b5d",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/README",
        1_983,
        "8bbaedd4eca97357edfb1ce07e5624535096d9f6",
        "60ab223e027c1666d2bc9ed16f7a5a8f3bcde082c0d130a53a6fddd292d1672e",
        "a9ebd48f68aad3336852335a1f464341c74a3be8e1a25afe8bcaec42a4b03cef",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/bug_442.cir",
        0,
        "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/bug_442.cir.sh",
        3_428,
        "c3c44839af4910a75d5d2798be30241c72ab983c",
        "2b3f46c49bae6de940204eaaa22d062ec1cd866ffba1c6e8ea9bf76155e6e78a",
        "9b8bcec7710b1b8e2713254fa8defaca3e9170dbe5baaa35e32162d091b4f1bc",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/bug_442_baseline.cir",
        1_363,
        "b39204c21b255e142fc233c715d5a61298ac1d2a",
        "ee266c41b800a063a39b9652a5c39077a458a304e4da2908786e06e54bc5f84b",
        "7e545f884948b872939b04c007cbe4f633be212626a12f444f7a6b50ab6d5ee9",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/bug_442_baseline_unpacked.cir",
        1_388,
        "e14ceec27cb9b07f2eef43fee9567909319f6486",
        "333d00cb317ed0b3be4b5a09d9a4252650c3c34697b96e03ff44f3ff9f372dfe",
        "a85188adb4de574e5545e5fedc0b42a21d195639c3afda2f23420b3720df2d30",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/bug_442_restarted.cir",
        1_350,
        "93f2d13ac2a209dded60dc4f89dce79d7286ab84",
        "ec043c518a8faa2dac6cf6b6677ab854d8ea25173586022494d4e554e4463115",
        "99adc306a853b6977218b5e6d6e816fa5c0e6e123ab3b74700ebc742f7634e3a",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/bug_442_restarted_unpacked.cir",
        1_368,
        "b574b6526a5057d9ec8bba0a5c65b4d8fb974d5b",
        "f62282385d6c209f76bcab4cfb3cf98ed07d09c86039358a60b55da9d77de49d",
        "978d13e28e78e2fb39863aad46f2ec0732ea5eeee6ef38d6f9b59d399d5d6263",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/exclude",
        104,
        "bee9272fb39bd485999907666dee2bcb7b7a3133",
        "915347ae37961be2efe28428e99a9d4415cea2a3913afed6396787fa8909e36a",
        "1e2b29ee2d4e561182103618e912c45add3da3fbd8e3d31707930fe7ce422ccc",
    ),
    (
        "Netlists/Certification_Tests/BUG_442/tags",
        36,
        "cc36a03c3218557982cb915eacf205d988e70f54",
        "31946a5db8220bb4d89a7b5d35ceade89e20fe3d3cb94d1789bae98abb3e7939",
        "a50f664b0d02b5233435ae392adc3ab2bc78c35c72ae3b349a1cfe3e6cb75b61",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_404,
        "17def57575eb3fd703978fd0634a58d6e679a3de",
        "a86524def2895930f2bb697c058850f231df6d623c279bd7482f30c5c39090b4",
        "11483735d34385359bbe0f981cbc8767c11e6c2b60c3f8723d49e2761479023a",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "5809bf44e921762c87b658f096d34f81aca5ccfb",
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];

const RETAINED_CONTENT_BYTES: usize = 7_452;
const RETAINED_STREAM_BYTES: usize = 761;
const RETAINED_STREAM_SHA256: &str =
    "20c0bf508238ab590295560e2921fced328e2b3b83b5246e9fbad6e6313c576a";
const RETAINED_STREAM_BLAKE3: &str =
    "ed7b8f1bc4baf71a7e1d59febd9eb0735f95c885fa77dc6997abc6f6f0576448";
const RETAINED: [(&str, usize, &str, &str); 6] = [
    (
        "README",
        1_983,
        "60ab223e027c1666d2bc9ed16f7a5a8f3bcde082c0d130a53a6fddd292d1672e",
        "a9ebd48f68aad3336852335a1f464341c74a3be8e1a25afe8bcaec42a4b03cef",
    ),
    (
        "bug_442.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "bug_442_baseline.cir",
        1_363,
        "ee266c41b800a063a39b9652a5c39077a458a304e4da2908786e06e54bc5f84b",
        "7e545f884948b872939b04c007cbe4f633be212626a12f444f7a6b50ab6d5ee9",
    ),
    (
        "bug_442_baseline_unpacked.cir",
        1_388,
        "333d00cb317ed0b3be4b5a09d9a4252650c3c34697b96e03ff44f3ff9f372dfe",
        "a85188adb4de574e5545e5fedc0b42a21d195639c3afda2f23420b3720df2d30",
    ),
    (
        "bug_442_restarted.cir",
        1_350,
        "ec043c518a8faa2dac6cf6b6677ab854d8ea25173586022494d4e554e4463115",
        "99adc306a853b6977218b5e6d6e816fa5c0e6e123ab3b74700ebc742f7634e3a",
    ),
    (
        "bug_442_restarted_unpacked.cir",
        1_368,
        "f62282385d6c209f76bcab4cfb3cf98ed07d09c86039358a60b55da9d77de49d",
        "978d13e28e78e2fb39863aad46f2ec0732ea5eeee6ef38d6f9b59d399d5d6263",
    ),
];

const NANOSECOND: Value = 1.0e-9;
const PRINT_STEP: Value = 0.25 * NANOSECOND;
const FINAL_STOP: Value = 50.0 * NANOSECOND;
const RESTART_INTERVAL: Value = 5.0 * NANOSECOND;
const RESTART_TARGET: Value = 10.0 * NANOSECOND;
const SAVE_TIMES: [Value; 3] = [0.0, RESTART_INTERVAL, RESTART_TARGET];
const MAX_MANIFEST_BYTES: usize = 1_048_576;
const MAX_DIRECTORY_ENTRIES: usize = 16_384;
const MAX_CHECKPOINT_BYTES: usize = 16 * 1024 * 1024;
const MAX_RESULT_ROWS: usize = 200_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug442Role {
    WrapperOwner,
    PackedBaseline,
    UnpackedBaseline,
    PackedRestarted,
    UnpackedRestarted,
}

impl Bug442Role {
    pub(super) const ALL: [Self; 5] = [
        Self::WrapperOwner,
        Self::PackedBaseline,
        Self::UnpackedBaseline,
        Self::PackedRestarted,
        Self::UnpackedRestarted,
    ];
    const WORKERS: [Self; 4] = [
        Self::PackedBaseline,
        Self::UnpackedBaseline,
        Self::PackedRestarted,
        Self::UnpackedRestarted,
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => BUG442_OWNER_CONTRACT,
            _ => BUG442_WORKER_CONTRACT,
        }
    }

    const fn file_name(self) -> &'static str {
        match self {
            Self::WrapperOwner => "bug_442.cir",
            Self::PackedBaseline => "bug_442_baseline.cir",
            Self::UnpackedBaseline => "bug_442_baseline_unpacked.cir",
            Self::PackedRestarted => "bug_442_restarted.cir",
            Self::UnpackedRestarted => "bug_442_restarted_unpacked.cir",
        }
    }

    pub(super) fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => "Netlists/Certification_Tests/BUG_442/bug_442.cir",
            Self::PackedBaseline => "Netlists/Certification_Tests/BUG_442/bug_442_baseline.cir",
            Self::UnpackedBaseline => {
                "Netlists/Certification_Tests/BUG_442/bug_442_baseline_unpacked.cir"
            }
            Self::PackedRestarted => "Netlists/Certification_Tests/BUG_442/bug_442_restarted.cir",
            Self::UnpackedRestarted => {
                "Netlists/Certification_Tests/BUG_442/bug_442_restarted_unpacked.cir"
            }
        }
    }

    pub(super) fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => "netlists/certification_tests/bug_442/bug_442.cir",
            Self::PackedBaseline => "netlists/certification_tests/bug_442/bug_442_baseline.cir",
            Self::UnpackedBaseline => {
                "netlists/certification_tests/bug_442/bug_442_baseline_unpacked.cir"
            }
            Self::PackedRestarted => "netlists/certification_tests/bug_442/bug_442_restarted.cir",
            Self::UnpackedRestarted => {
                "netlists/certification_tests/bug_442/bug_442_restarted_unpacked.cir"
            }
        }
    }

    const fn print_file(self) -> Option<&'static str> {
        match self {
            Self::WrapperOwner => None,
            Self::PackedBaseline => Some("bug_442_baseline.cir.prn"),
            Self::UnpackedBaseline => Some("bug_442_baseline_unpacked.cir.prn"),
            Self::PackedRestarted => Some("bug_442_restarted.cir.prn"),
            Self::UnpackedRestarted => Some("bug_442_restarted_unpacked.cir.prn"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bug442RuntimeSeal {
    retained: BTreeMap<String, Vec<u8>>,
    manifest_rows: BTreeSet<String>,
    exclusion_rows: BTreeSet<String>,
}

impl XyceTestRunner {
    fn bug442_record_stream_identities() -> ((usize, String, String), (usize, String, String)) {
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

    fn validate_bug442_record_streams() -> Result<(), String> {
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
        let (historical, retained) = Self::bug442_record_stream_identities();
        let malformed = HISTORICAL.iter().any(|record| {
            record.2.len() != 40
                || record.3.len() != 64
                || record.4.len() != 64
                || record
                    .2
                    .bytes()
                    .chain(record.3.bytes())
                    .chain(record.4.bytes())
                    .any(|byte| !byte.is_ascii_hexdigit() || byte.is_ascii_uppercase())
        });
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || PRETRIM_NETLISTS_TREE != UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
            || HISTORICAL.len() != 12
            || historical_unique.len() != HISTORICAL.len()
            || historical_content != HISTORICAL_CONTENT_BYTES
            || malformed
            || historical.0 != HISTORICAL_STREAM_BYTES
            || historical.1 != HISTORICAL_STREAM_SHA256
            || historical.2 != HISTORICAL_STREAM_BLAKE3
            || RETAINED.len() != 6
            || retained_unique.len() != RETAINED.len()
            || retained_content != RETAINED_CONTENT_BYTES
            || retained.0 != RETAINED_STREAM_BYTES
            || retained.1 != RETAINED_STREAM_SHA256
            || retained.2 != RETAINED_STREAM_BLAKE3
        {
            return Err(format!(
                "{LABEL} immutable identities changed: historical={historical:?}/{historical_content}, retained={retained:?}/{retained_content}"
            ));
        }
        Ok(())
    }

    fn find_bug442_exact_child(
        parent: &Path,
        expected: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Option<PathBuf>, String> {
        let metadata = fs::symlink_metadata(parent)
            .map_err(|error| format!("failed to inspect {LABEL} parent: {error}"))?;
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
                return Err(format!("{LABEL} parent exceeds its census envelope"));
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

    fn bug442_exact_child(
        parent: &Path,
        expected: &str,
        abort: &dyn AbortSignal,
    ) -> Result<PathBuf, String> {
        Self::find_bug442_exact_child(parent, expected, abort)?
            .ok_or_else(|| format!("{LABEL} is missing exact path component {expected:?}"))
    }

    fn bug442_family_directory(&self, abort: &dyn AbortSignal) -> Result<PathBuf, String> {
        let netlists = Self::bug442_exact_child(&self.root, "Netlists", abort)?;
        let certification = Self::bug442_exact_child(&netlists, "Certification_Tests", abort)?;
        let family = Self::bug442_exact_child(&certification, "BUG_442", abort)?;
        let metadata = fs::symlink_metadata(&family)
            .map_err(|error| format!("failed to inspect {LABEL} family directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        Ok(family)
    }

    fn read_bug442_bounded_text(
        path: &Path,
        max_bytes: usize,
        purpose: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<u8>, String> {
        let metadata = fs::symlink_metadata(path)
            .map_err(|error| format!("failed to inspect {LABEL} {purpose}: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "{LABEL} {purpose} must be a regular non-symlink file"
            ));
        }
        if metadata.len() > max_bytes as u64 {
            return Err(format!("{LABEL} {purpose} exceeds its bounded envelope"));
        }
        let mut bytes = Vec::with_capacity((metadata.len() as usize).min(max_bytes));
        fs::File::open(path)
            .map_err(|error| format!("failed to open {LABEL} {purpose}: {error}"))?
            .take((max_bytes + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("failed to read {LABEL} {purpose}: {error}"))?;
        if bytes.len() > max_bytes || abort.is_aborted() {
            return Err(format!(
                "{LABEL} {purpose} read exceeded its envelope or deadline"
            ));
        }
        Self::canonical_lf_text_identity(&format!("{LABEL} {purpose}"), &bytes)
    }

    fn read_bug442_family(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug442_record_streams()?;
        let family = self.bug442_family_directory(abort)?;
        let expected = RETAINED
            .into_iter()
            .map(|record| (record.0.to_ascii_lowercase(), record))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for (index, entry) in fs::read_dir(&family)
            .map_err(|error| format!("failed to enumerate {LABEL}: {error}"))?
            .enumerate()
        {
            if index > RETAINED.len() {
                return Err(format!(
                    "{LABEL} retained census exceeds its exact envelope"
                ));
            }
            if abort.is_aborted() {
                return Err(format!("{LABEL} source census aborted"));
            }
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
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
            let canonical = Self::read_bug442_bounded_text(
                &entry.path(),
                cap,
                &format!("member {name}"),
                abort,
            )?;
            let sha = format!("{:x}", Sha256::digest(&canonical));
            let b3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes || sha != expected_sha || b3 != expected_b3 {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha={sha}, b3={b3}",
                    canonical.len()
                ));
            }
            std::str::from_utf8(&canonical)
                .map_err(|error| format!("{LABEL} member {name:?} is not UTF-8: {error}"))?;
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

    fn read_bug442_manifest_rows(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeSet<String>, String> {
        let path = Self::bug442_exact_child(&self.root, HARNESS_MANIFEST_FILE, abort)?;
        let bytes =
            Self::read_bug442_bounded_text(&path, MAX_MANIFEST_BYTES, "harness manifest", abort)?;
        let source = std::str::from_utf8(&bytes)
            .map_err(|error| format!("{LABEL} harness manifest is not UTF-8: {error}"))?;
        let expected = format!(
            "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}",
            Bug442Role::WrapperOwner.path()
        );
        let mut rows = BTreeSet::new();
        for (index, line) in source.lines().enumerate() {
            let candidate = line
                .replace('\\', "/")
                .to_ascii_lowercase()
                .contains(FAMILY_PREFIX);
            if candidate && (line != expected || !rows.insert(line.to_string())) {
                return Err(format!(
                    "{LABEL} harness manifest line {} changed: {line:?}",
                    index + 1
                ));
            }
        }
        if rows != BTreeSet::from([expected]) {
            return Err(format!(
                "{LABEL} owner-only manifest census changed: {rows:?}"
            ));
        }
        Ok(rows)
    }

    fn read_bug442_exclusion_rows(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeSet<String>, String> {
        let path = Self::bug442_exact_child(&self.root, UPSTREAM_EXCLUSIONS_MANIFEST_FILE, abort)?;
        let bytes = Self::read_bug442_bounded_text(
            &path,
            MAX_MANIFEST_BYTES,
            "upstream exclusions manifest",
            abort,
        )?;
        let source = std::str::from_utf8(&bytes)
            .map_err(|error| format!("{LABEL} exclusions manifest is not UTF-8: {error}"))?;
        let expected = Bug442Role::WORKERS
            .map(|role| {
                format!(
                    "{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{BUG442_WORKER_CONTRACT}",
                    role.path()
                )
            })
            .into_iter()
            .collect::<BTreeSet<_>>();
        let mut rows = BTreeSet::new();
        for (index, line) in source.lines().enumerate() {
            let candidate = line
                .replace('\\', "/")
                .to_ascii_lowercase()
                .contains(FAMILY_PREFIX);
            if candidate && (!expected.contains(line) || !rows.insert(line.to_string())) {
                return Err(format!(
                    "{LABEL} exclusions line {} changed: {line:?}",
                    index + 1
                ));
            }
        }
        if rows != expected {
            return Err(format!("{LABEL} exclusion census changed: {rows:?}"));
        }

        let parsed = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions are invalid: {error}"))?;
        let actual = parsed
            .keys()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .cloned()
            .collect::<BTreeSet<_>>();
        let expected_records = Bug442Role::WORKERS
            .map(|role| role.record().to_string())
            .into_iter()
            .collect::<BTreeSet<_>>();
        if actual != expected_records || parsed.contains_key(Bug442Role::WrapperOwner.record()) {
            return Err(format!(
                "{LABEL} parsed exclusion ownership changed: {actual:?}"
            ));
        }
        for role in Bug442Role::WORKERS {
            let exclusion = parsed
                .get(role.record())
                .ok_or_else(|| format!("{LABEL} lost {role:?} exclusion"))?;
            if exclusion.source != EXCLUSION_SOURCE
                || !matches!(
                    &exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == BUG442_WORKER_CONTRACT
                )
            {
                return Err(format!(
                    "{LABEL} {role:?} qualification changed: {exclusion:?}"
                ));
            }
        }
        Ok(rows)
    }

    fn validate_bug442_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug442Role,
        abort: &dyn AbortSignal,
    ) -> Result<Bug442RuntimeSeal, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("{LABEL} recognized {role:?} path is not canonical"));
        }
        let retained = self.read_bug442_family(abort)?;
        let manifest_rows = self.read_bug442_manifest_rows(abort)?;
        let exclusion_rows = self.read_bug442_exclusion_rows(abort)?;
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        for role in Bug442Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(role.path()))
                .map_err(|error| format!("{LABEL} {} {error}", role.file_name()))?;
        }
        Ok(Bug442RuntimeSeal {
            retained,
            manifest_rows,
            exclusion_rows,
        })
    }

    fn validate_bug442_role_source(role: Bug442Role, source: &str) -> Result<(), String> {
        if role == Bug442Role::WrapperOwner {
            return source
                .is_empty()
                .then_some(())
                .ok_or_else(|| format!("{LABEL} wrapper owner must remain exactly zero bytes"));
        }
        let expected_restart = match role {
            Bug442Role::PackedBaseline => ".options restart job=trans_test initial_interval=5n",
            Bug442Role::UnpackedBaseline => {
                ".options restart pack=0 job=trans_test_unpacked initial_interval=5n"
            }
            Bug442Role::PackedRestarted => ".options restart file=trans_test1e-08",
            Bug442Role::UnpackedRestarted => ".options restart file=trans_test_unpacked1e-08",
            Bug442Role::WrapperOwner => unreachable!(),
        };
        let expected = [
            "transmission line circuit".to_string(),
            "vin 1 0 pulse(0 5 0 0.1n 0.1n 5n 25n)".to_string(),
            "rin 1 2 50".to_string(),
            "tline 2 0 3 0 z0=50 td=10n".to_string(),
            "rl 3 0 50".to_string(),
            ".tran 0.25n 50n".to_string(),
            format!(
                ".print tran file={} v(2) v(3)",
                role.print_file().expect("worker has print file")
            ),
            expected_restart.to_string(),
            ".end".to_string(),
        ];
        let significant = source
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('*'))
            .map(str::to_ascii_lowercase)
            .collect::<Vec<_>>();
        if significant != expected {
            return Err(format!(
                "{LABEL} {role:?} authored source envelope changed: {significant:?}"
            ));
        }
        Ok(())
    }

    fn bug442_element<'a>(
        netlist: &'a Netlist,
        name: &str,
    ) -> Result<&'a rspice_core::netlist::Element, String> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| format!("{LABEL} is missing element {name}"))
    }

    fn bug442_resistor_matches(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
    ) -> bool {
        element.provenance == ElementProvenance::Authored
            && element.name.eq_ignore_ascii_case(name)
            && element.nodes.iter().map(String::as_str).eq(nodes)
            && matches!(
                &element.kind,
                ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == 50.0f64.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
            )
    }

    fn validate_bug442_topology(netlist: &Netlist) -> Result<(), String> {
        let vin = Self::bug442_element(netlist, "VIN")?;
        let rin = Self::bug442_element(netlist, "RIN")?;
        let tline = Self::bug442_element(netlist, "TLINE")?;
        let rl = Self::bug442_element(netlist, "RL")?;
        let pulse = matches!(
            &vin.kind,
            ElementKind::VoltageSource(SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                pulse_count,
                width_defaults_to_zero: false,
            }) if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 5.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && rise.to_bits() == (0.1 * NANOSECOND).to_bits()
                && fall.to_bits() == (0.1 * NANOSECOND).to_bits()
                && width.to_bits() == (5.0 * NANOSECOND).to_bits()
                && period.to_bits() == (25.0 * NANOSECOND).to_bits()
                && pulse_count.to_bits() == 0.0f64.to_bits()
        );
        let line = tline.provenance == ElementProvenance::Authored
            && tline.name.eq_ignore_ascii_case("TLINE")
            && tline
                .nodes
                .iter()
                .map(String::as_str)
                .eq(["2", "0", "3", "0"])
            && matches!(
                &tline.kind,
                ElementKind::TransmissionLine {
                    z0: Some(z0),
                    td: Some(td),
                    freq: None,
                    nl: None,
                    model: None,
                } if z0.to_bits() == 50.0f64.to_bits()
                    && td.to_bits() == (10.0 * NANOSECOND).to_bits()
            );
        if vin.provenance != ElementProvenance::Authored
            || !vin.nodes.iter().map(String::as_str).eq(["1", "0"])
            || !pulse
            || !Self::bug442_resistor_matches(rin, "RIN", ["1", "2"])
            || !line
            || !Self::bug442_resistor_matches(rl, "RL", ["3", "0"])
        {
            return Err(format!(
                "{LABEL} exact pulse/resistor/lossless-line topology changed: {:?}",
                netlist.elements
            ));
        }
        Ok(())
    }

    fn bug442_restart_options_match(role: Bug442Role, netlist: &Netlist) -> bool {
        let mut remaining = netlist.options.clone();
        let restart = remaining.restart.take();
        if format!("{remaining:?}") != format!("{:?}", SimulationOptions::default()) {
            return false;
        }
        let Some(restart) = restart else {
            return false;
        };
        if restart.print_timeint_options.is_some()
            || restart.start_time.is_some()
            || !restart.intervals.is_empty()
        {
            return false;
        }
        match role {
            Bug442Role::PackedBaseline => {
                restart.pack.is_none()
                    && restart.job.as_deref() == Some("trans_test")
                    && restart.file.is_none()
                    && restart.initial_interval.map(Value::to_bits)
                        == Some(RESTART_INTERVAL.to_bits())
            }
            Bug442Role::UnpackedBaseline => {
                restart.pack == Some(false)
                    && restart.job.as_deref() == Some("trans_test_unpacked")
                    && restart.file.is_none()
                    && restart.initial_interval.map(Value::to_bits)
                        == Some(RESTART_INTERVAL.to_bits())
            }
            Bug442Role::PackedRestarted => {
                restart.pack.is_none()
                    && restart.job.is_none()
                    && restart.file.as_deref() == Some("trans_test1e-08")
                    && restart.initial_interval.is_none()
            }
            Bug442Role::UnpackedRestarted => {
                restart.pack.is_none()
                    && restart.job.is_none()
                    && restart.file.as_deref() == Some("trans_test_unpacked1e-08")
                    && restart.initial_interval.is_none()
            }
            Bug442Role::WrapperOwner => false,
        }
    }

    fn bug442_worker_plan(
        &self,
        role: Bug442Role,
        retained: &BTreeMap<String, Vec<u8>>,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if role == Bug442Role::WrapperOwner {
            return Err(format!("{LABEL} wrapper owner is not a simulation deck"));
        }
        let source_bytes = retained
            .get(&role.file_name().to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost {role:?} source"))?;
        let source = std::str::from_utf8(source_bytes)
            .map_err(|error| format!("{LABEL} {role:?} source is not UTF-8: {error}"))?;
        Self::validate_bug442_role_source(role, source)?;
        if Self::contains_control_block(source) {
            return Err(format!("{LABEL} does not admit simulator scripting blocks"));
        }
        Self::reject_unsupported_source_directives(source)?;
        let path = self.root.join(role.path());
        let netlist = Self::parse_xyce_netlist(source, &path)
            .map_err(|error| format!("{LABEL} {role:?} parse failed: {error}"))?;
        if netlist.title != "Transmission Line Circuit"
            || netlist.elements.len() != 4
            || netlist.analyses.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || netlist.output_requests.len() != 1
            || !netlist.measurements.is_empty()
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
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || netlist.source_text.as_deref() != Some(source)
            || netlist.source_path.as_deref() != Some(path.as_path())
            || !Self::bug442_restart_options_match(role, &netlist)
        {
            return Err(format!(
                "{LABEL} {role:?} typed envelope changed: analyses={:?}, outputs={:?}, options={:?}",
                netlist.analyses, netlist.output_requests, netlist.options
            ));
        }
        if !matches!(
            netlist.analyses.as_slice(),
            [AnalysisCommand::Tran {
                step,
                stop,
                start: None,
                max_step: None,
                uic: false,
            }] if step.to_bits() == PRINT_STEP.to_bits()
                && stop.to_bits() == FINAL_STOP.to_bits()
        ) {
            return Err(format!(
                "{LABEL} {role:?} .TRAN changed: {:?}",
                netlist.analyses
            ));
        }
        Self::validate_bug442_topology(&netlist)?;
        validate_output_symbols(&netlist)
            .map_err(|error| format!("{LABEL} {role:?} output symbols changed: {error}"))?;
        let outputs = Self::print_output_requests(source, "TRAN")?;
        if !matches!(
            outputs.as_slice(),
            [output] if output.format.is_none()
                && output.file.as_deref() == role.print_file()
                && output.probes.iter().map(String::as_str).eq(["V(2)", "V(3)"])
        ) {
            return Err(format!(
                "{LABEL} {role:?} FILE/PRINT contract changed: {outputs:?}"
            ));
        }
        let tran = Self::single_tran_analysis(&netlist)?;
        let plan = XyceStaticTranPlan {
            deck_path: path.clone(),
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: vec!["V(2)".into(), "V(3)".into()],
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
            || plan.tran.step.to_bits() != PRINT_STEP.to_bits()
            || plan.tran.stop.to_bits() != FINAL_STOP.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} {role:?} manual transient plan changed: {plan:?}"
            ));
        }
        Ok((plan, netlist))
    }

    fn bug442_require_result_horizon(
        label: &str,
        result: &TransientResult,
        start: Value,
        stop: Value,
    ) -> Result<(), String> {
        Self::validate_transient_result_time_grid(result)?;
        if result.time.is_empty()
            || result.time.len() > MAX_RESULT_ROWS
            || result.time.first().map(|value| value.to_bits()) != Some(start.to_bits())
            || result.time.last().map(|value| value.to_bits()) != Some(stop.to_bits())
            || result
                .voltages
                .iter()
                .chain(&result.branch_currents)
                .flat_map(|waveform| waveform.iter())
                .any(|value| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} {label} result horizon/framing changed: rows={}, endpoints={:?}/{:?}",
                result.time.len(),
                result.time.first(),
                result.time.last()
            ));
        }
        Ok(())
    }

    fn bug442_require_tline_history(checkpoint: &TransientCheckpoint) -> Result<(), String> {
        let text = checkpoint.to_text();
        let header = text
            .lines()
            .find(|line| line.starts_with("tline_state "))
            .ok_or_else(|| format!("{LABEL} checkpoint omitted TLINE state"))?;
        let fields = header.split_whitespace().collect::<Vec<_>>();
        let counts = fields
            .get(8..11)
            .ok_or_else(|| format!("{LABEL} malformed TLINE state header: {header:?}"))?;
        let has_history = counts
            .iter()
            .any(|field| field.parse::<usize>().is_ok_and(|count| count > 0));
        if !text.lines().any(|line| line == "tline_state_available 1")
            || !text.lines().any(|line| line == "tline_blockers 0")
            || !text.lines().any(|line| line == "tline_states 1")
            || fields
                .get(1)
                .is_none_or(|name| !name.eq_ignore_ascii_case("TLINE"))
            || !has_history
        {
            return Err(format!(
                "{LABEL} checkpoint did not retain real TLINE history: {header:?}"
            ));
        }
        Ok(())
    }

    fn bug442_round_trip_checkpoint(
        checkpoint: &TransientCheckpoint,
        encoding: TransientCheckpointEncoding,
    ) -> Result<(TransientCheckpoint, Vec<u8>), String> {
        Self::bug442_require_tline_history(checkpoint)?;
        let encoded = checkpoint
            .to_bytes(encoding)
            .map_err(|error| format!("{LABEL} {encoding:?} encoding failed: {error}"))?;
        if encoded.is_empty() || encoded.len() > MAX_CHECKPOINT_BYTES {
            return Err(format!(
                "{LABEL} {encoding:?} checkpoint violated byte envelope: {}",
                encoded.len()
            ));
        }
        let restored =
            TransientCheckpoint::from_bytes_with_encoding(&encoded, encoding, MAX_CHECKPOINT_BYTES)
                .map_err(|error| format!("{LABEL} {encoding:?} decoding failed: {error}"))?;
        let wrong_encoding = match encoding {
            TransientCheckpointEncoding::Unpacked => TransientCheckpointEncoding::Packed,
            TransientCheckpointEncoding::Packed => TransientCheckpointEncoding::Unpacked,
        };
        if TransientCheckpoint::from_bytes_with_encoding(
            &encoded,
            wrong_encoding,
            MAX_CHECKPOINT_BYTES,
        )
        .is_ok()
        {
            return Err(format!(
                "{LABEL} {encoding:?} checkpoint was accepted by the {wrong_encoding:?} decoder"
            ));
        }
        if restored != *checkpoint {
            return Err(format!("{LABEL} {encoding:?} checkpoint was not bit-exact"));
        }
        Self::bug442_require_tline_history(&restored)?;
        Ok((restored, encoded))
    }

    fn compare_bug442_restart_tables(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let expected = ["Index", "TIME", "V(2)", "V(3)"];
        for (label, table) in [("GOOD", good), ("TEST", test)] {
            if table.columns.iter().map(String::as_str).ne(expected)
                || table.rows.is_empty()
                || table.rows.len() > MAX_RESULT_ROWS
                || table.rows.iter().any(|row| {
                    row.len() != expected.len() || row.iter().any(|value| !value.is_finite())
                })
            {
                return Err(format!("{LABEL} {label} PRN framing changed"));
            }
        }
        self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            good,
            test,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )
    }

    fn bug442_require_comparison(
        &self,
        label: &str,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<(), String> {
        let mismatches = self.compare_bug442_restart_tables(good, test)?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} {label} produced {} Release-7.10 xyce_verify mismatch(es): {mismatches:?}",
                mismatches.len()
            ))
        }
    }

    fn bug442_run_baseline(
        &self,
        role: Bug442Role,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientCheckpoint), String> {
        let max_step = Self::transient_family_max_step(netlist, &plan.tran)?;
        let engine = self
            .create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(&plan.tran));
        let (result, checkpoints) = engine
            .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                netlist,
                FINAL_STOP,
                max_step,
                TransientStartupMode::from_uic(false),
                &SAVE_TIMES,
                abort,
            )
            .map_err(|error| format!("{LABEL} {role:?} baseline failed: {error}"))?;
        Self::bug442_require_result_horizon(&format!("{role:?}"), &result, 0.0, FINAL_STOP)?;
        if checkpoints.len() != SAVE_TIMES.len()
            || checkpoints
                .iter()
                .map(|scheduled| scheduled.nominal_time.to_bits())
                .ne(SAVE_TIMES.map(Value::to_bits))
        {
            return Err(format!("{LABEL} {role:?} checkpoint schedule changed"));
        }
        let checkpoint = checkpoints
            .into_iter()
            .find(|scheduled| scheduled.nominal_time.to_bits() == RESTART_TARGET.to_bits())
            .ok_or_else(|| format!("{LABEL} {role:?} omitted the historical 10 ns checkpoint"))?
            .checkpoint;
        if checkpoint.time.to_bits() != RESTART_TARGET.to_bits() {
            return Err(format!(
                "{LABEL} {role:?} checkpoint captured at {:.17e}, expected exact 10 ns",
                checkpoint.time
            ));
        }
        Self::bug442_require_tline_history(&checkpoint)?;
        Ok((result, checkpoint))
    }

    fn bug442_resume(
        &self,
        role: Bug442Role,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        checkpoint: &TransientCheckpoint,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, String> {
        let max_step = Self::transient_family_max_step(netlist, &plan.tran)?;
        let engine = self
            .create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(&plan.tran));
        let (result, final_checkpoint) = engine
            .run_tran_restart_resume_with_abort(netlist, checkpoint, FINAL_STOP, max_step, abort)
            .map_err(|error| format!("{LABEL} {role:?} resume failed: {error}"))?;
        Self::bug442_require_result_horizon(
            &format!("{role:?}"),
            &result,
            RESTART_TARGET,
            FINAL_STOP,
        )?;
        if final_checkpoint.time.to_bits() != FINAL_STOP.to_bits() {
            return Err(format!("{LABEL} {role:?} final checkpoint horizon changed"));
        }
        Ok(result)
    }

    pub(super) fn validate_bug442_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug442Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let before = self.validate_bug442_provenance(deck, role, &abort)?;
        for member in Bug442Role::ALL {
            let source = before
                .retained
                .get(&member.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {member:?}"))?;
            Self::validate_bug442_role_source(
                member,
                std::str::from_utf8(source)
                    .map_err(|error| format!("{LABEL} {member:?} is not UTF-8: {error}"))?,
            )?;
        }
        let (packed_baseline_plan, packed_baseline_netlist) =
            self.bug442_worker_plan(Bug442Role::PackedBaseline, &before.retained)?;
        let (unpacked_baseline_plan, unpacked_baseline_netlist) =
            self.bug442_worker_plan(Bug442Role::UnpackedBaseline, &before.retained)?;
        let (packed_restarted_plan, packed_restarted_netlist) =
            self.bug442_worker_plan(Bug442Role::PackedRestarted, &before.retained)?;
        let (unpacked_restarted_plan, unpacked_restarted_netlist) =
            self.bug442_worker_plan(Bug442Role::UnpackedRestarted, &before.retained)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }

        let (packed_baseline, packed_checkpoint) = self.bug442_run_baseline(
            Bug442Role::PackedBaseline,
            &packed_baseline_plan,
            &packed_baseline_netlist,
            &abort,
        )?;
        let (unpacked_baseline, unpacked_checkpoint) = self.bug442_run_baseline(
            Bug442Role::UnpackedBaseline,
            &unpacked_baseline_plan,
            &unpacked_baseline_netlist,
            &abort,
        )?;
        let (packed_checkpoint, packed_bytes) = Self::bug442_round_trip_checkpoint(
            &packed_checkpoint,
            TransientCheckpointEncoding::Packed,
        )?;
        let (unpacked_checkpoint, unpacked_bytes) = Self::bug442_round_trip_checkpoint(
            &unpacked_checkpoint,
            TransientCheckpointEncoding::Unpacked,
        )?;
        if packed_bytes == unpacked_bytes
            || unpacked_bytes.as_slice() != unpacked_checkpoint.to_text().as_bytes()
        {
            return Err(format!(
                "{LABEL} packed and unpacked branches did not use genuinely distinct encodings"
            ));
        }
        let packed_restarted = self.bug442_resume(
            Bug442Role::PackedRestarted,
            &packed_restarted_plan,
            &packed_restarted_netlist,
            &packed_checkpoint,
            &abort,
        )?;
        let unpacked_restarted = self.bug442_resume(
            Bug442Role::UnpackedRestarted,
            &unpacked_restarted_plan,
            &unpacked_restarted_netlist,
            &unpacked_checkpoint,
            &abort,
        )?;

        let packed_baseline_table = Self::transient_family_result_to_prn_table(
            &packed_baseline_plan,
            &packed_baseline_netlist,
            &packed_baseline,
        )?;
        let unpacked_baseline_table = Self::transient_family_result_to_prn_table(
            &unpacked_baseline_plan,
            &unpacked_baseline_netlist,
            &unpacked_baseline,
        )?;
        let packed_restarted_table = Self::transient_family_result_to_prn_table(
            &packed_restarted_plan,
            &packed_restarted_netlist,
            &packed_restarted,
        )?;
        let unpacked_restarted_table = Self::transient_family_result_to_prn_table(
            &unpacked_restarted_plan,
            &unpacked_restarted_netlist,
            &unpacked_restarted,
        )?;
        self.bug442_require_comparison(
            "packed baseline GOOD vs packed restart TEST",
            &packed_baseline_table,
            &packed_restarted_table,
        )?;
        self.bug442_require_comparison(
            "unpacked baseline GOOD vs unpacked restart TEST",
            &unpacked_baseline_table,
            &unpacked_restarted_table,
        )?;
        self.bug442_require_comparison(
            "packed restart GOOD vs unpacked restart TEST",
            &packed_restarted_table,
            &unpacked_restarted_table,
        )?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded its shared deadline"));
        }
        let after = self.validate_bug442_provenance(deck, role, &abort)?;
        if before != after {
            return Err(format!("{LABEL} provenance changed during execution"));
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

    fn deck(root: &Path, role: Bug442Role) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            relative_path: role.path().to_string(),
            section: XyceDeckSection::Netlists,
        }
    }

    fn test_abort() -> DeadlineAbort {
        DeadlineAbort::new(Instant::now(), 30_000)
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug442-{label}-"))
            .tempdir()
            .expect("create BUG442 fixture");
        let root = temporary.path();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG442 family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED {
            fs::copy(canonical.join(name), family.join(name)).expect("copy retained BUG442 file");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n",
                Bug442Role::WrapperOwner.path()
            ),
        )
        .expect("write BUG442 manifest");
        let mut exclusions = vec![
            format!("schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}"),
            format!("source_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}"),
            format!("source_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}"),
        ];
        exclusions.extend(Bug442Role::WORKERS.map(|role| {
            format!(
                "{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{BUG442_WORKER_CONTRACT}",
                role.path()
            )
        }));
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            exclusions.join("\n") + "\n",
        )
        .expect("write BUG442 exclusions");
        let owner = deck(root, Bug442Role::WrapperOwner);
        (temporary, owner)
    }

    #[test]
    fn bug442_roles_paths_contracts_and_stream_seals_are_exact() {
        let records = Bug442Role::ALL
            .map(Bug442Role::record)
            .into_iter()
            .collect::<BTreeSet<_>>();
        assert_eq!(records.len(), Bug442Role::ALL.len());
        for role in Bug442Role::ALL {
            assert_eq!(Bug442Role::for_record(role.path()), Some(role));
            assert_eq!(Bug442Role::for_record(role.record()), Some(role));
            assert_eq!(
                role.contract(),
                if role == Bug442Role::WrapperOwner {
                    BUG442_OWNER_CONTRACT
                } else {
                    BUG442_WORKER_CONTRACT
                }
            );
        }
        assert_eq!(
            Bug442Role::for_record("Netlists/Certification_Tests/BUG_442/bug_442.CIR"),
            Some(Bug442Role::WrapperOwner),
            "routing may recognize normalized case so provenance can reject its noncanonical path"
        );
        for near_miss in [
            "Netlists/Certification_Tests/BUG_442/bug_442.cir.sh",
            "Netlists/Certification_Tests/BUG_442/invented.cir",
            "Netlists/Certification_Tests/BUG_442x/bug_442.cir",
        ] {
            assert_eq!(
                Bug442Role::for_record(near_miss),
                None,
                "claimed {near_miss}"
            );
        }
        XyceTestRunner::validate_bug442_record_streams()
            .expect("BUG442 historical and retained streams remain exact");
    }

    #[test]
    fn bug442_retained_provenance_and_all_worker_plans_are_exact() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug442Role::ALL {
            let seal = runner
                .validate_bug442_provenance(&deck(&root, role), role, &test_abort())
                .unwrap_or_else(|error| panic!("{role:?}: {error}"));
            if role != Bug442Role::WrapperOwner {
                runner
                    .bug442_worker_plan(role, &seal.retained)
                    .unwrap_or_else(|error| panic!("{role:?}: {error}"));
            }
        }
    }

    #[test]
    fn bug442_source_and_metadata_counterfactuals_fail_closed() {
        for (label, mutate) in [
            ("source", 0usize),
            ("census", 1),
            ("manifest", 2),
            ("exclusion", 3),
            ("output", 4),
            ("artifact", 5),
        ] {
            let (temporary, owner) = fixture(label);
            let root = temporary.path();
            match mutate {
                0 => fs::write(
                    root.join(Bug442Role::PackedBaseline.path()),
                    "Transmission Line Circuit\nVIN 1 0 PULSE(0 4 0 0.1N 0.1N 5N 25N)\n.END\n",
                )
                .expect("mutate BUG442 source"),
                1 => fs::write(
                    root.join(FAMILY_DIRECTORY).join("invented.cir"),
                    "* drift\n",
                )
                .expect("add BUG442 member"),
                2 => fs::write(
                    root.join(HARNESS_MANIFEST_FILE),
                    format!(
                        "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n",
                        Bug442Role::PackedBaseline.path()
                    ),
                )
                .expect("mutate BUG442 manifest"),
                3 => {
                    let path = root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
                    let text = fs::read_to_string(&path).expect("read BUG442 exclusions");
                    fs::write(
                        &path,
                        text.replacen(EXCLUSION_SOURCE, "invented/exclude", 1),
                    )
                    .expect("mutate BUG442 exclusion");
                }
                4 => fs::create_dir_all(root.join(OUTPUT_DIRECTORY))
                    .expect("invent BUG442 OutputData"),
                5 => fs::write(
                    root.join(FAMILY_DIRECTORY).join("trans_test1e-08"),
                    "stale restart state",
                )
                .expect("invent BUG442 restart artifact"),
                _ => unreachable!(),
            }
            let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
            assert!(
                runner
                    .validate_bug442_provenance(&owner, Bug442Role::WrapperOwner, &test_abort())
                    .is_err(),
                "{label} drift was accepted"
            );
        }
    }

    #[test]
    fn bug442_source_semantic_mutations_are_rejected_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let seal = runner
            .validate_bug442_provenance(
                &deck(&root, Bug442Role::WrapperOwner),
                Bug442Role::WrapperOwner,
                &test_abort(),
            )
            .unwrap();
        for (role, old, new) in [
            (Bug442Role::PackedBaseline, "Z0=50", "Z0=49"),
            (Bug442Role::PackedBaseline, "TD=10N", "TD=11N"),
            (Bug442Role::PackedBaseline, "0.25N 50N", "0.5N 50N"),
            (Bug442Role::PackedBaseline, "V(2) V(3)", "V(3) V(2)"),
            (Bug442Role::UnpackedBaseline, "pack=0", "pack=1"),
            (
                Bug442Role::PackedRestarted,
                "trans_test1e-08",
                "trans_test2e-08",
            ),
        ] {
            let mut changed = seal.retained.clone();
            let key = role.file_name().to_ascii_lowercase();
            let source = String::from_utf8(changed[&key].clone()).unwrap();
            changed.insert(key, source.replacen(old, new, 1).into_bytes());
            assert!(
                runner.bug442_worker_plan(role, &changed).is_err(),
                "{role:?} admitted mutation {old:?}->{new:?}"
            );
        }
    }

    #[test]
    fn bug442_comparator_is_directional_and_rejects_history_loss() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let columns = ["Index", "TIME", "V(2)", "V(3)"]
            .map(str::to_string)
            .to_vec();
        let good = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 10.0e-9, 100.0, 2.5],
                vec![1.0, 20.0e-9, 100.0, 2.5],
            ],
        };
        let near = XycePrnTable {
            columns: columns.clone(),
            rows: vec![vec![0.0, 10.0e-9, 99.0, 2.5], vec![1.0, 20.0e-9, 99.0, 2.5]],
        };
        assert!(
            runner
                .compare_bug442_restart_tables(&good, &near)
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_bug442_restart_tables(&near, &good)
                .unwrap()
                .is_empty()
        );
        let mut lost = near;
        lost.rows[0][3] = 0.0;
        lost.rows[1][3] = 0.0;
        assert!(
            !runner
                .compare_bug442_restart_tables(&good, &lost)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn bug442_packed_codec_corruption_and_history_loss_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let seal = runner
            .validate_bug442_provenance(
                &deck(&root, Bug442Role::WrapperOwner),
                Bug442Role::WrapperOwner,
                &test_abort(),
            )
            .unwrap();
        let (plan, netlist) = runner
            .bug442_worker_plan(Bug442Role::PackedBaseline, &seal.retained)
            .unwrap();
        let (_, checkpoint) = runner
            .bug442_run_baseline(Bug442Role::PackedBaseline, &plan, &netlist, &test_abort())
            .unwrap();
        let (_, mut packed) = XyceTestRunner::bug442_round_trip_checkpoint(
            &checkpoint,
            TransientCheckpointEncoding::Packed,
        )
        .unwrap();
        let last = packed.len() - 1;
        packed[last] ^= 0x01;
        assert!(
            TransientCheckpoint::from_bytes_with_encoding(
                &packed,
                TransientCheckpointEncoding::Packed,
                MAX_CHECKPOINT_BYTES,
            )
            .is_err()
        );
        let history_lost = checkpoint
            .to_text()
            .replace("tline_state_available 1", "tline_state_available 0");
        assert!(TransientCheckpoint::from_text(&history_lost).is_err());
    }

    #[test]
    fn bug442_expired_shared_deadline_fails_before_work() {
        let root = corpus_root();
        let config = XyceRunnerConfig {
            max_time_per_test_ms: 1,
            ..XyceRunnerConfig::default()
        };
        let runner = XyceTestRunner::new(&root, config);
        let owner = deck(&root, Bug442Role::WrapperOwner);
        assert!(
            runner
                .validate_bug442_oracle(
                    &owner,
                    Bug442Role::WrapperOwner,
                    Instant::now() - Duration::from_secs(1),
                )
                .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn bug442_symlinked_member_fails_closed() {
        use std::os::unix::fs::symlink;
        let (temporary, owner) = fixture("symlink");
        let path = temporary.path().join(Bug442Role::PackedBaseline.path());
        let target = path.with_extension("real");
        fs::rename(&path, &target).unwrap();
        symlink(&target, &path).unwrap();
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug442_provenance(&owner, Bug442Role::WrapperOwner, &test_abort())
                .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn bug442_symlinked_family_directory_fails_closed() {
        use std::os::unix::fs::symlink;
        let (temporary, owner) = fixture("directory-symlink");
        let family = temporary.path().join(FAMILY_DIRECTORY);
        let target = temporary.path().join("real-bug442-family");
        fs::rename(&family, &target).unwrap();
        symlink(&target, &family).unwrap();
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug442_provenance(&owner, Bug442Role::WrapperOwner, &test_abort())
                .is_err()
        );
    }
}
