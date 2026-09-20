use super::*;
use std::io::Read as _;

const LABEL: &str = "BUG_411 duplicate model-parameter error wrapper";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_411";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_411/";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_411";
const OWNER_NAME: &str = "perl-generated.cir";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_411/perl-generated.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_411/perl-generated.cir";
const CONTRACT: &str = "bug411_duplicate_model_parameter_wrapper_owner";

const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const PRETRIM_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";

const HISTORICAL_MANIFEST: &[u8] =
    b"README\nperl-generated.cir\nperl-generated.cir.sh\ntags\noptions\n";
const HISTORICAL_TAGS: &[u8] = b"serial, parallel, nightly, mos3\nerrorexit\n";
const HISTORICAL_OPTIONS: &[u8] = b"timelimit=30\n";
const UPSTREAM_DIAGNOSTIC_PREFIX: &str = "in file perl-generated.cir at or near line 2195";
const UPSTREAM_DIAGNOSTIC: &str =
    "Device model CD4012_PMOS: Duplicate specification of parameter NFS";

const HISTORICAL_CONTENT_BYTES: usize = 140_542;
const HISTORICAL_STREAM_BYTES: usize = 1_901;
const HISTORICAL_STREAM_SHA256: &str =
    "0e4a21fad95a291e0817492ec028ab5d0df688c75a0c0dee77c1e552b6cf1bec";
const HISTORICAL_STREAM_BLAKE3: &str =
    "a87ee30f7059ac84b0b96f73c78f6dd9eb2272ed752590c75f511c9b600fe05d";

// Path, canonical-LF bytes, Git blob, SHA-256, BLAKE3. The first three fields
// form the sealed Release-7.10 record stream; the content digests independently
// identify retained files and the three embedded historical metadata records.
const HISTORICAL: [(&str, usize, &str, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_411/Manifest.txt",
        61,
        "519303ae50a42642dcecbb0b050fcfe5e2eab64b",
        "cd30cba8673fd68ef3449b404d6187af15d30509fce3b2189ee06f31c88bfcfa",
        "d6180da5ef2f4cb4e40c945de4f7e4448f023bb84f0bf00caed43684b77291e5",
    ),
    (
        "Netlists/Certification_Tests/BUG_411/README",
        720,
        "85bbe46ab77ef755430970dd118aeb71994720df",
        "b8d5ea86c6e4ca6a9cc83d2a580a78b6172f07a4e2dce316835dbb140d2415e8",
        "8fc4603aba78cce85bbe47363f504244ecc92ed0c0797b0b70435b25bc8360dd",
    ),
    (
        "Netlists/Certification_Tests/BUG_411/options",
        13,
        "447b78121fc2b096925c7d2f9df9867cc294e6ef",
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
    (
        OWNER_PATH,
        69_857,
        "522d93526e86063cefd4707f766a9840568ea8b0",
        "0655626a97e29ca9cf4edbd74ac5e90f23ea2070e230d94942e9d2c3f04c1e55",
        "c419a5dfdd1f8a0731b34e308149f67379d1bbb3343b9a0bf4ac01a2f5b4108a",
    ),
    (
        "Netlists/Certification_Tests/BUG_411/perl-generated.cir.sh",
        1_445,
        "c63d883f4bb0da1a506bf8dcd52d0c03353607d5",
        "03174cb346e8196b594518d8ff973f1ba5bded1c036eb40b61a3f7b4f18401ca",
        "4b6e94c56a0c14f66ebc4ace334fd1e0039f99fe971f44f70b8c6aacef940671",
    ),
    (
        "Netlists/Certification_Tests/BUG_411/tags",
        42,
        "94f28759c22f56f4981778afa2b47bb78f8ea9e7",
        "ab7ce2f0826a529dceb594583719f2389261ab52588732183a9ca7d7ab9fe230",
        "dca1ee73dcfc8e5330141379b9d99a7f97b86044b61f22c21ead9b7c8937af32",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_404,
        "17def57575eb3fd703978fd0634a58d6e679a3de",
        "a86524def2895930f2bb697c058850f231df6d623c279bd7482f30c5c39090b4",
        "11483735d34385359bbe0f981cbc8767c11e6c2b60c3f8723d49e2761479023a",
    ),
];

const RETAINED_STREAM_BYTES: usize = 352;
const RETAINED_STREAM_SHA256: &str =
    "68f23828c26f65d10b02199d1a289739795dbcbb9a44f8757795094c92dfbd77";
const RETAINED_STREAM_BLAKE3: &str =
    "5afded6dc03d87f3dd13474bcb380b3aaa36fe93381f06d6cf087a967fced169";
const RETAINED: [(&str, usize, &str, &str); 3] = [
    (
        "README",
        720,
        "b8d5ea86c6e4ca6a9cc83d2a580a78b6172f07a4e2dce316835dbb140d2415e8",
        "8fc4603aba78cce85bbe47363f504244ecc92ed0c0797b0b70435b25bc8360dd",
    ),
    (
        "options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
    (
        OWNER_NAME,
        69_857,
        "0655626a97e29ca9cf4edbd74ac5e90f23ea2070e230d94942e9d2c3f04c1e55",
        "c419a5dfdd1f8a0731b34e308149f67379d1bbb3343b9a0bf4ac01a2f5b4108a",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug411Role;

impl Bug411Role {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(record) == OWNER_RECORD).then_some(Self)
    }

    pub(super) const fn contract(self) -> &'static str {
        CONTRACT
    }
}

impl XyceTestRunner {
    fn bug411_record_stream_identities() -> ((usize, String, String), (usize, String, String)) {
        let historical = HISTORICAL
            .iter()
            .map(|(path, bytes, blob, ..)| {
                format!(
                    "{PRETRIM_COMMIT}\t{PRETRIM_NETLISTS_TREE}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{RELEASE_TAG_OBJECT}\t{path}\t{bytes}\t{blob}"
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let retained = RETAINED
            .iter()
            .map(|(name, bytes, sha, ..)| format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha}"))
            .collect::<Vec<_>>()
            .join("\n");
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

    fn validate_bug411_embedded_record(path: &str, content: &[u8]) -> bool {
        HISTORICAL.iter().any(|record| {
            record.0 == path
                && record.1 == content.len()
                && format!("{:x}", Sha256::digest(content)) == record.3
                && blake3::hash(content).to_hex().as_str() == record.4
        })
    }

    pub(super) fn validate_bug411_historical_provenance() -> Result<(), String> {
        let content_bytes = HISTORICAL.iter().map(|record| record.1).sum::<usize>();
        let unique = HISTORICAL
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let (historical, retained) = Self::bug411_record_stream_identities();
        let identities_well_formed = HISTORICAL.iter().all(|record| {
            record.2.len() == 40
                && record.2.bytes().all(|byte| byte.is_ascii_hexdigit())
                && record.3.len() == 64
                && record.3.bytes().all(|byte| byte.is_ascii_hexdigit())
                && record.4.len() == 64
                && record.4.bytes().all(|byte| byte.is_ascii_hexdigit())
        });
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || PRETRIM_NETLISTS_TREE != UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
            || HISTORICAL.len() != 7
            || unique.len() != HISTORICAL.len()
            || content_bytes != HISTORICAL_CONTENT_BYTES
            || !identities_well_formed
            || historical.0 != HISTORICAL_STREAM_BYTES
            || historical.1 != HISTORICAL_STREAM_SHA256
            || historical.2 != HISTORICAL_STREAM_BLAKE3
            || retained.0 != RETAINED_STREAM_BYTES
            || retained.1 != RETAINED_STREAM_SHA256
            || retained.2 != RETAINED_STREAM_BLAKE3
            || !Self::validate_bug411_embedded_record(
                "Netlists/Certification_Tests/BUG_411/Manifest.txt",
                HISTORICAL_MANIFEST,
            )
            || !Self::validate_bug411_embedded_record(
                "Netlists/Certification_Tests/BUG_411/tags",
                HISTORICAL_TAGS,
            )
            || !Self::validate_bug411_embedded_record(
                "Netlists/Certification_Tests/BUG_411/options",
                HISTORICAL_OPTIONS,
            )
        {
            return Err(format!(
                "{LABEL} Release-7.10 provenance changed: historical={}/{content_bytes}/{historical:?}; retained={retained:?}",
                HISTORICAL.len(),
            ));
        }
        Ok(())
    }

    fn read_bug411_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug411_historical_provenance()?;
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
            let entry = entry.map_err(|error| format!("failed to inspect {LABEL}: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
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
            let Some((expected_name, expected_bytes, expected_sha, expected_blake3)) =
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
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha != expected_sha
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha={sha}, blake3={content_blake3}",
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
        if observed.get("options").map(Vec::as_slice) != Some(HISTORICAL_OPTIONS) {
            return Err(format!("{LABEL} retained timelimit metadata changed"));
        }
        Ok(observed)
    }

    fn validate_bug411_provenance(
        &self,
        deck: &XyceDeck,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != OWNER_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != OWNER_RECORD
            || !Self::same_path(&deck.path, &self.root.join(OWNER_PATH))
        {
            return Err(format!("{LABEL} recognized owner is not canonical"));
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
        let family_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .collect::<Vec<_>>();
        if !family_exclusions.is_empty() {
            return Err(format!(
                "{LABEL} active wrapper family must not be excluded: {family_exclusions:?}"
            ));
        }
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        self.reject_wrapper_output_artifacts(&self.root.join(OWNER_PATH))
            .map_err(|error| format!("{LABEL} {OWNER_PATH} {error}"))?;
        self.read_bug411_directory(abort)
    }

    fn bug411_device_card_count(source: &str, prefix: u8) -> usize {
        source
            .lines()
            .filter_map(|line| line.trim_start().split_ascii_whitespace().next())
            .filter(|token| {
                token
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| byte.eq_ignore_ascii_case(&prefix))
                    && !token.starts_with('.')
                    && !token.starts_with('*')
            })
            .count()
    }

    fn validate_bug411_source_shape(source: &str) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if !source.ends_with('\n')
            || lines.len() != 2_214
            || lines.first().copied() != Some("discrete81.esp Boolean Network Circuit")
            || lines
                .last()
                .is_none_or(|line| !line.trim().eq_ignore_ascii_case(".END"))
            || lines.get(1_792).map(|line| line.trim()) != Some(".TRAN .1s 20s")
            || lines.get(2_194).map(|line| line.trim()) != Some(".MODEL cd4012_pmos PMOS (")
            || lines.get(2_202).map(|line| line.trim()) != Some(".MODEL cd4012_nmos NMOS (")
        {
            return Err(format!("{LABEL} physical source envelope changed"));
        }

        let logical = Self::logical_netlist_lines(source);
        let directive_count = |name: &str| {
            logical
                .iter()
                .filter(|line| {
                    line.split_ascii_whitespace()
                        .next()
                        .is_some_and(|token| token.eq_ignore_ascii_case(name))
                })
                .count()
        };
        let subckts = directive_count(".SUBCKT");
        let ends = directive_count(".ENDS");
        let models = directive_count(".MODEL");
        let trans = directive_count(".TRAN");
        let prints = logical
            .iter()
            .filter(|line| {
                line.split_ascii_whitespace()
                    .next()
                    .is_some_and(|token| token.eq_ignore_ascii_case(".PRINT"))
            })
            .collect::<Vec<_>>();
        if subckts != 25
            || ends != 25
            || models != 2
            || trans != 1
            || prints.len() != 1
            || Self::bug411_device_card_count(source, b'X') != 664
            || Self::bug411_device_card_count(source, b'R') != 593
            || Self::bug411_device_card_count(source, b'M') != 20
        {
            return Err(format!(
                "{LABEL} structural census changed: subckt={subckts}, ends={ends}, models={models}, tran={trans}, print={}, X={}, R={}, M={}",
                prints.len(),
                Self::bug411_device_card_count(source, b'X'),
                Self::bug411_device_card_count(source, b'R'),
                Self::bug411_device_card_count(source, b'M'),
            ));
        }
        let print_tokens = prints[0].split_ascii_whitespace().collect::<Vec<_>>();
        if print_tokens.len() != 83
            || !print_tokens[0].eq_ignore_ascii_case(".PRINT")
            || !print_tokens[1].eq_ignore_ascii_case("TRAN")
            || !(0..=80)
                .all(|index| print_tokens[index + 2].eq_ignore_ascii_case(&format!("V(n{index})")))
        {
            return Err(format!("{LABEL} V(n0)..V(n80) output request changed"));
        }
        if logical.iter().any(|line| {
            let directive = line.split_ascii_whitespace().next().unwrap_or_default();
            directive.eq_ignore_ascii_case(".INC")
                || directive.eq_ignore_ascii_case(".INCLUDE")
                || directive.eq_ignore_ascii_case(".LIB")
                || directive.eq_ignore_ascii_case(".MEASURE")
                || directive.eq_ignore_ascii_case(".MEAS")
        }) {
            return Err(format!(
                "{LABEL} acquired external input or invented numerical oracle"
            ));
        }
        Ok(())
    }

    fn parse_bug411_duplicate_model_error(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<rspice_core::netlist::DuplicateModelParameterError, String> {
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        match Netlist::parse_with_path_and_options_and_abort(source, deck_path, options, abort) {
            Err(rspice_core::netlist::ParseWithAbortError::Parse(
                ParseError::DuplicateModelParameter(error),
            )) => Ok(*error),
            Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => Err(format!(
                "{LABEL} produced the wrong parse failure: {error:?}"
            )),
            Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                Err(format!("{LABEL} parsing exceeded its bounded contract"))
            }
            Ok(_) => Err(format!("{LABEL} unexpectedly parsed successfully")),
        }
    }

    pub(super) fn observe_bug411_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<rspice_core::netlist::DuplicateModelParameterError, String> {
        let error = Self::parse_bug411_duplicate_model_error(source, deck_path, abort)?;
        if error.model_name != "CD4012_PMOS"
            || error.canonical_model_name != "CD4012_PMOS"
            || error.parameter_name != "NFS"
            || error.canonical_parameter_name != "NFS"
            || error.model_origin.line != 2_195
            || error
                .model_origin
                .path
                .as_deref()
                .is_none_or(|path| !Self::same_path(path, deck_path))
        {
            return Err(format!("{LABEL} typed failure identity changed: {error:?}"));
        }
        let file_name = error
            .model_origin
            .path
            .as_deref()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{LABEL} diagnostic lost its UTF-8 file name"))?;
        let prefix = format!(
            "in file {file_name} at or near line {}",
            error.model_origin.line
        );
        let diagnostic = format!(
            "Device model {}: Duplicate specification of parameter {}",
            error.canonical_model_name, error.canonical_parameter_name
        );
        if prefix != UPSTREAM_DIAGNOSTIC_PREFIX || diagnostic != UPSTREAM_DIAGNOSTIC {
            return Err(format!(
                "{LABEL} wrapper diagnostic projection changed: {prefix:?} / {diagnostic:?}"
            ));
        }
        let projected = format!("Netlist error {prefix}\n{diagnostic}\n");
        if Self::bug411_wrapper_exit_code(1, "", &projected) != 0 {
            return Err(format!(
                "{LABEL} no longer satisfies the historical wrapper"
            ));
        }
        Ok(error)
    }

    // The historical Perl wrapper used regexes. Its only metacharacter is the
    // dot in `perl-generated.cir`, so this bounded matcher reproduces Perl's
    // one-character (non-newline) dot semantics without adding a runtime regex
    // dependency to the conformance leaf crate.
    fn bug411_regex_find(stream: &str, pattern: &str) -> Option<usize> {
        let stream = stream.as_bytes();
        let pattern = pattern.as_bytes();
        if pattern.len() > stream.len() {
            return None;
        }
        (0..=stream.len() - pattern.len()).find(|start| {
            stream[*start..*start + pattern.len()]
                .iter()
                .zip(pattern)
                .all(|(actual, expected)| {
                    (*expected == b'.' && *actual != b'\n') || actual == expected
                })
        })
    }

    fn bug411_wrapper_stream_matches(stream: &str) -> bool {
        let Some(first) = Self::bug411_regex_find(stream, UPSTREAM_DIAGNOSTIC_PREFIX) else {
            return false;
        };
        let tail = &stream[first + UPSTREAM_DIAGNOSTIC_PREFIX.len()..];
        Self::bug411_regex_find(tail, UPSTREAM_DIAGNOSTIC).is_some()
    }

    fn bug411_wrapper_exit_code(simulator_exit: i32, stdout: &str, stderr: &str) -> i32 {
        if simulator_exit == 0 {
            2
        } else if Self::bug411_wrapper_stream_matches(stderr)
            || Self::bug411_wrapper_stream_matches(stdout)
        {
            0
        } else {
            10
        }
    }

    pub(super) fn validate_bug411_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.clamp(1, 30_000));
        let before = self.validate_bug411_provenance(deck, &abort)?;
        let owner = before
            .get(&OWNER_NAME.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost its sealed owner"))?;
        let source = std::str::from_utf8(owner)
            .map_err(|error| format!("{LABEL} owner is not UTF-8: {error}"))?;
        Self::validate_bug411_source_shape(source)?;
        Self::observe_bug411_failure(source, &deck.path, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded its deadline"));
        }
        let after = self.validate_bug411_provenance(deck, &abort)?;
        if before != after {
            return Err(format!("{LABEL} sealed sources changed during execution"));
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

    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default())
    }

    fn owner_source() -> String {
        fs::read_to_string(corpus_root().join(OWNER_PATH))
            .expect("read BUG411 owner")
            .replace("\r\n", "\n")
    }

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: OWNER_PATH.to_string(),
        }
    }

    fn bug411_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug411-{label}-"))
            .tempdir()
            .expect("create BUG411 fixture root");
        let root = temporary.path();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG411 family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG411 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG411 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG411 exclusions");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug411_route_is_owner_only() {
        assert_eq!(Bug411Role::for_record(OWNER_PATH), Some(Bug411Role));
        assert_eq!(Bug411Role::for_record(OWNER_RECORD), Some(Bug411Role));
        for rejected in [
            "Netlists/Certification_Tests/BUG_411/README",
            "Netlists/Certification_Tests/BUG_411/options",
            "Netlists/Certification_Tests/BUG_411/perl-generated.cir.sh",
            "Netlists/Certification_Tests/BUG_411/sibling.cir",
            "OutputData/Certification_Tests/BUG_411/perl-generated.cir.prn",
        ] {
            assert_eq!(
                Bug411Role::for_record(rejected),
                None,
                "accepted {rejected}"
            );
        }
    }

    #[test]
    fn bug411_historical_and_retained_provenance_are_sealed() {
        XyceTestRunner::validate_bug411_historical_provenance().unwrap();
        assert_eq!(
            HISTORICAL_TAGS,
            b"serial, parallel, nightly, mos3\nerrorexit\n"
        );
        assert_eq!(HISTORICAL_OPTIONS, b"timelimit=30\n");
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        assert_eq!(runner().read_bug411_directory(&abort).unwrap().len(), 3);
    }

    #[test]
    fn bug411_checked_in_owner_has_exact_shape_and_typed_failure() {
        let source = owner_source();
        let path = corpus_root().join(OWNER_PATH);
        XyceTestRunner::validate_bug411_source_shape(&source).unwrap();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let error = XyceTestRunner::observe_bug411_failure(&source, &path, &abort).unwrap();
        assert_eq!(error.model_origin.line, 2_195);
        assert_eq!(error.canonical_model_name, "CD4012_PMOS");
        assert_eq!(error.canonical_parameter_name, "NFS");

        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG411 deadline");
        let expired = DeadlineAbort::new(expired_start, 1);
        let failure = XyceTestRunner::parse_bug411_duplicate_model_error(&source, &path, &expired)
            .expect_err("expired BUG411 observation must abort");
        assert!(failure.contains("bounded contract"), "{failure}");
    }

    #[test]
    fn bug411_full_oracle_enforces_historical_thirty_second_deadline() {
        let (_temporary, deck, runner) = bug411_fixture("deadline");
        assert!(runner.config.max_time_per_test_ms > 30_000);
        let expired_start = Instant::now()
            .checked_sub(Duration::from_secs(31))
            .expect("construct expired BUG411 oracle deadline");
        let failure = runner
            .validate_bug411_oracle(&deck, expired_start)
            .expect_err("BUG411 must retain its historical 30-second timelimit");
        assert!(failure.contains("aborted"), "{failure}");
    }

    #[test]
    fn bug411_first_error_order_and_parser_precedence_fail_closed() {
        let source = owner_source();
        let path = corpus_root().join(OWNER_PATH);
        let abort = DeadlineAbort::new(Instant::now(), 30_000);

        let lowercase = source.replacen("NFS=1E10", "nfs=1E10", 1);
        let error =
            XyceTestRunner::parse_bug411_duplicate_model_error(&lowercase, &path, &abort).unwrap();
        assert_eq!(error.parameter_name, "nfs");
        assert_eq!(error.canonical_parameter_name, "NFS");
        assert_eq!(error.model_origin.line, 2_195);

        let repaired_first = source.replacen("NFS=1E10", "        ", 1);
        let error =
            XyceTestRunner::parse_bug411_duplicate_model_error(&repaired_first, &path, &abort)
                .unwrap();
        assert_eq!(error.canonical_model_name, "CD4012_NMOS");
        assert_eq!(error.canonical_parameter_name, "VMAX");
        assert_eq!(error.model_origin.line, 2_203);

        let earlier = source.replacen("UO = 310", "UO = 310 UO=311", 1);
        let error =
            XyceTestRunner::parse_bug411_duplicate_model_error(&earlier, &path, &abort).unwrap();
        assert_eq!(error.canonical_parameter_name, "UO");
        assert_eq!(error.model_origin.line, 2_195);

        let malformed = source.replacen("NFS=1E10", "NFS=", 1);
        let options = NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..NetlistParseOptions::default()
        };
        let failure =
            Netlist::parse_with_path_and_options_and_abort(&malformed, &path, options, &abort);
        let Err(rspice_core::netlist::ParseWithAbortError::Parse(ParseError::Syntax {
            line,
            message,
        })) = failure
        else {
            panic!("malformed BUG411 duplicate produced {failure:?}");
        };
        assert_eq!(line, 2_195);
        assert!(!message.contains("Duplicate specification"), "{message}");
    }

    #[test]
    fn bug411_historical_wrapper_semantics_are_exact() {
        let canonical = format!(
            "prefix\n{UPSTREAM_DIAGNOSTIC_PREFIX}\nintervening diagnostic\n{UPSTREAM_DIAGNOSTIC}\ntrailer\n"
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, "", &canonical),
            0
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, &canonical, ""),
            0
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(0, "", &canonical),
            2
        );

        let wildcard = canonical.replace("perl-generated.cir", "perl-generatedXcir");
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, "", &wildcard),
            0
        );
        let reversed = format!("{UPSTREAM_DIAGNOSTIC}\n{UPSTREAM_DIAGNOSTIC_PREFIX}\n");
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, "", &reversed),
            10
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(
                1,
                UPSTREAM_DIAGNOSTIC_PREFIX,
                UPSTREAM_DIAGNOSTIC,
            ),
            10
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, "", "unrelated"),
            10
        );
    }

    #[test]
    fn bug411_provenance_rejects_owner_metadata_and_artifact_drift() {
        let assert_valid = |runner: &XyceTestRunner, deck: &XyceDeck| {
            let abort = DeadlineAbort::new(Instant::now(), 30_000);
            runner
                .validate_bug411_provenance(deck, &abort)
                .expect("canonical BUG411 fixture is valid");
        };
        let assert_rejected = |runner: &XyceTestRunner, deck: &XyceDeck| {
            let abort = DeadlineAbort::new(Instant::now(), 30_000);
            assert!(runner.validate_bug411_provenance(deck, &abort).is_err());
        };

        let (_temporary, deck, runner) = bug411_fixture("source");
        assert_valid(&runner, &deck);
        fs::write(&deck.path, "* drift\n").expect("mutate BUG411 owner");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug411_fixture("manifest");
        assert_valid(&runner, &deck);
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove BUG411 wrapper owner");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug411_fixture("exclusion");
        assert_valid(&runner, &deck);
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{OWNER_PATH}\tNetlists/Certification_Tests/BUG_411/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG411 exclusion");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug411_fixture("extra-member");
        assert_valid(&runner, &deck);
        fs::write(runner.root.join(FAMILY_DIRECTORY).join("unexpected"), b"x")
            .expect("add BUG411 member");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug411_fixture("non-file-member");
        assert_valid(&runner, &deck);
        fs::create_dir(runner.root.join(FAMILY_DIRECTORY).join("nested"))
            .expect("add non-file BUG411 member");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug411_fixture("case");
        assert_valid(&runner, &deck);
        let family = runner.root.join(FAMILY_DIRECTORY);
        let intermediate = family.join("README.case-change");
        fs::rename(family.join("README"), &intermediate).expect("stage BUG411 case drift");
        fs::rename(intermediate, family.join("readme")).expect("apply BUG411 case drift");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug411_fixture("output");
        assert_valid(&runner, &deck);
        fs::create_dir_all(runner.root.join(OUTPUT_DIRECTORY)).expect("create forbidden gold");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug411_fixture("artifact");
        assert_valid(&runner, &deck);
        fs::write(deck.path.with_extension("cir.out"), b"invented output")
            .expect("create forbidden wrapper artifact");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug411_fixture("wrong-section");
        assert_valid(&runner, &deck);
        let wrong_section = XyceDeck {
            path: deck.path.clone(),
            section: XyceDeckSection::Other,
            relative_path: deck.relative_path.clone(),
        };
        assert_rejected(&runner, &wrong_section);

        let (_temporary, deck, runner) = bug411_fixture("wrong-path");
        assert_valid(&runner, &deck);
        let wrong_path = XyceDeck {
            path: deck.path.clone(),
            section: deck.section,
            relative_path: "Netlists/Certification_Tests/BUG_411/sibling.cir".into(),
        };
        assert_rejected(&runner, &wrong_path);
    }

    #[test]
    fn bug411_checked_in_owner_executes_complete_oracle() {
        let runner = runner();
        let result = runner.run_test(runner.root.join(OWNER_PATH));
        assert!(result.passed, "BUG411 failed: {:?}", result.error);
        assert_eq!(result.contract, CONTRACT);
    }
}
