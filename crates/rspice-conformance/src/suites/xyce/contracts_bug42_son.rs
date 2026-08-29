use super::*;
use std::io::Read as _;

const LABEL: &str = "BUG_42_SON archived deprecated .LIB alias failure";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_42_SON";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_42_son/";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_42_SON";
const ARCHIVE_NAME: &str = "SON_BUGS_NOT_NEEDING_TEST_CASES";
const ARCHIVE_PATH: &str = "Netlists/Certification_Tests/SON_BUGS_NOT_NEEDING_TEST_CASES";
const ARCHIVE_MAP_KEY: &str = "../son_bugs_not_needing_test_cases";
const OWNER_NAME: &str = "bug42.cir";
const SIDECAR_NAME: &str = "resistance.txt";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_42_SON/bug42.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_42_son/bug42.cir";
const CONTRACT: &str = "archived_expected_failure_bug42_deprecated_lib_alias";

const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const PRETRIM_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";

const HISTORICAL_MANIFEST: &[u8] = b"bug42.cir\nbug42.cir.sh\nresistance.txt\ntags\n";
const HISTORICAL_TAGS: &[u8] = b"exclude\n";
const ARCHIVAL_RATIONALE_FIRST_LINE: usize = 299;
const ARCHIVAL_RATIONALE: &[u8] =
    b"BUG 42.  The feature tested by this bug is now deprecated, so the test is \nnow excluded.\n";
const ARCHIVAL_RATIONALE_SHA256: &str =
    "3ac382e62812bae74b6ff1e1583b9c54b19eb010e049f87da180a75ded9a0e8c";
const BUG51_CROSS_REFERENCE: &str = "BUG 51:  Failure of existing test, BUG_42_SON.";

const EXPECTED_PARSE_LINE: usize = 3;
const EXPECTED_PARSE_MESSAGE: &str = "Library section 'resistance.txt' missing .ENDL";
const UPSTREAM_DIAGNOSTIC: &str = "Netlist error: Could not find .ENDL statement for '.LIB RESISTANCE.TXT'.  Maybe '.LIB RESISTANCE.TXT <library_name>' or '.INC RESISTANCE.TXT' was intended.";

const HISTORICAL_CONTENT_BYTES: usize = 46_897;
const HISTORICAL_STREAM_BYTES: usize = 2_439;
const HISTORICAL_STREAM_SHA256: &str =
    "71e9acf1028e1625bdfd49300880e982221a6d8ff1c4402d7ba45ac8d819d2eb";
const HISTORICAL_STREAM_BLAKE3: &str =
    "b1be2abb969e17a06fd47be2e4336c18eaa73d711bbd9f58d1f0be31fdd7351d";

// Path, canonical-LF bytes, Git blob, SHA-256, and BLAKE3. The archived
// SON_BUGS record is deliberately part of this exact Release-7.10 stream:
// it is the upstream authority for why the otherwise retained wrapper is no
// longer registered as an active regression.
const HISTORICAL: [(&str, usize, &str, &str, &str); 6] = [
    (
        "Netlists/Certification_Tests/BUG_42_SON/Manifest.txt",
        43,
        "f87a3ce1d6138fb68ff34395af2e9df21014a5ea",
        "fb63c7ca6211a6b61130480fcf0b5169f7053e79fcb7df9e78de54d9025b820a",
        "da9414079736e61a50156cfea5846a7320f5332834b660a76187e0a282bb2c4b",
    ),
    (
        OWNER_PATH,
        117,
        "7f5e44ba0aa8f75fcfb8ccdc035f05d553b8d42c",
        "ead230ea6158cb649d09127054d950ce70b682b4bcc7f9c03eb012562cce9a24",
        "ac1a9aeeebfac11340094144474ae2c46232659a5948207685bfc5b884e92bcf",
    ),
    (
        "Netlists/Certification_Tests/BUG_42_SON/bug42.cir.sh",
        1_001,
        "0ea243d6bc4c01973ad5691ccbf61bf1f42d1748",
        "58af0b7310a438c917707c0e9cfa76fe35c633bdcdcfbf36a8085feb12527275",
        "19e619a8e58c4284207b8808230d9e89561bf0b42fa38b1d6b059f2de9e19bd7",
    ),
    (
        "Netlists/Certification_Tests/BUG_42_SON/resistance.txt",
        16,
        "4145cd113e2425ee32881847bb8a98c8842f694f",
        "031fbf1dd5e36d247db16e696a7563f1cb5b9624d97e1da0973a11e14c7468fc",
        "fc7c4bc8baae0b61b76b1a05e81fdcc75a64626bf72d0e70dd088b6a6a377d8d",
    ),
    (
        "Netlists/Certification_Tests/BUG_42_SON/tags",
        8,
        "9ba870ea76f7e4fc9169e11d0612d5f3899a9822",
        "4e6c23fcd9140520f152d969561caee073952c6027b65f96b07cd01da70432e1",
        "c76ce284ba1075a571da61a7e0227cb9734e4965b3ebdf09e920a45dccc758f5",
    ),
    (
        ARCHIVE_PATH,
        45_712,
        "dbc61cb7b742f2351b7a816f8a547990f2ab333f",
        "be79999a20838103ee258c51ec8e9c569c3642ee576ca29dd296909762d3a544",
        "b75c18f1276afa3d030c4b2f61b8ea5835d585a580149385f9473941bb0504f8",
    ),
];

const RETAINED_STREAM_BYTES: usize = 241;
const RETAINED_STREAM_SHA256: &str =
    "a1ce46b740ac30b97db4f1664b93f3d389e1b05c2fa629c3207a3abcc6ae6dc6";
const RETAINED_STREAM_BLAKE3: &str =
    "baa91ba35564e0ec17269e6498bebba199fbc3c9c5cb308ddf3fabf951682afb";
const RETAINED: [(&str, usize, &str, &str); 2] = [
    (
        OWNER_NAME,
        117,
        "ead230ea6158cb649d09127054d950ce70b682b4bcc7f9c03eb012562cce9a24",
        "ac1a9aeeebfac11340094144474ae2c46232659a5948207685bfc5b884e92bcf",
    ),
    (
        SIDECAR_NAME,
        16,
        "031fbf1dd5e36d247db16e696a7563f1cb5b9624d97e1da0973a11e14c7468fc",
        "fc7c4bc8baae0b61b76b1a05e81fdcc75a64626bf72d0e70dd088b6a6a377d8d",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug42SonRole;

impl Bug42SonRole {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(record) == OWNER_RECORD).then_some(Self)
    }

    pub(super) const fn contract(self) -> &'static str {
        CONTRACT
    }
}

#[derive(Debug, Clone, Copy)]
enum Bug42ParseMode {
    Filesystem,
    SealedRootOnly,
}

impl XyceTestRunner {
    fn bug42_record_stream_identities() -> ((usize, String, String), (usize, String, String)) {
        let mut historical = HISTORICAL
            .iter()
            .map(|(path, bytes, blob, sha, b3)| {
                format!(
                    "{PRETRIM_COMMIT}\t{PRETRIM_NETLISTS_TREE}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{RELEASE_TAG_OBJECT}\t{path}\t{bytes}\t{blob}\t{sha}\t{b3}"
                )
            })
            .collect::<Vec<_>>();
        historical.sort();
        let historical = historical.join("\n");
        let mut retained = RETAINED
            .iter()
            .map(|(name, bytes, sha, ..)| format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha}"))
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

    fn bug42_embedded_record(path: &str, content: &[u8]) -> bool {
        HISTORICAL.iter().any(|record| {
            record.0 == path
                && record.1 == content.len()
                && format!("{:x}", Sha256::digest(content)) == record.3
                && blake3::hash(content).to_hex().as_str() == record.4
        })
    }

    pub(super) fn validate_bug42_historical_provenance() -> Result<(), String> {
        let content_bytes = HISTORICAL.iter().map(|record| record.1).sum::<usize>();
        let unique = HISTORICAL
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let identities_well_formed = HISTORICAL.iter().all(|record| {
            record.2.len() == 40
                && record.2.bytes().all(|byte| byte.is_ascii_hexdigit())
                && record.3.len() == 64
                && record.3.bytes().all(|byte| byte.is_ascii_hexdigit())
                && record.4.len() == 64
                && record.4.bytes().all(|byte| byte.is_ascii_hexdigit())
        });
        let (historical, retained) = Self::bug42_record_stream_identities();
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || PRETRIM_NETLISTS_TREE != UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
            || UPSTREAM_COMMIT.len() != 40
            || RELEASE_TAG != "Release-7.10.0"
            || RELEASE_TAG_OBJECT.len() != 40
            || HISTORICAL.len() != 6
            || unique.len() != HISTORICAL.len()
            || content_bytes != HISTORICAL_CONTENT_BYTES
            || !identities_well_formed
            || historical.0 != HISTORICAL_STREAM_BYTES
            || historical.1 != HISTORICAL_STREAM_SHA256
            || historical.2 != HISTORICAL_STREAM_BLAKE3
            || retained.0 != RETAINED_STREAM_BYTES
            || retained.1 != RETAINED_STREAM_SHA256
            || retained.2 != RETAINED_STREAM_BLAKE3
            || !Self::bug42_embedded_record(
                "Netlists/Certification_Tests/BUG_42_SON/Manifest.txt",
                HISTORICAL_MANIFEST,
            )
            || !Self::bug42_embedded_record(
                "Netlists/Certification_Tests/BUG_42_SON/tags",
                HISTORICAL_TAGS,
            )
            || ARCHIVAL_RATIONALE_FIRST_LINE != 299
            || format!("{:x}", Sha256::digest(ARCHIVAL_RATIONALE)) != ARCHIVAL_RATIONALE_SHA256
        {
            return Err(format!(
                "{LABEL} Release-7.10 provenance changed: historical={}/{content_bytes}/{historical:?}; retained={retained:?}",
                HISTORICAL.len()
            ));
        }
        Ok(())
    }

    fn read_bug42_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug42_historical_provenance()?;
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
                    "{LABEL} member {name:?} changed: bytes={}, sha={sha}, blake3={b3}",
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

    fn read_bug42_archive(&self, abort: &dyn AbortSignal) -> Result<Vec<u8>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} archival-document validation aborted"));
        }
        let archive_path = self.root.join(ARCHIVE_PATH);
        let parent = archive_path
            .parent()
            .ok_or_else(|| format!("{LABEL} archival document has no parent"))?;
        let parent_metadata = fs::symlink_metadata(parent)
            .map_err(|error| format!("failed to inspect {LABEL} archive parent: {error}"))?;
        if parent_metadata.file_type().is_symlink() || !parent_metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} archive parent must be a regular non-symlink directory"
            ));
        }

        let mut matched_names = Vec::new();
        for entry in fs::read_dir(parent)
            .map_err(|error| format!("failed to enumerate {LABEL} archive parent: {error}"))?
        {
            if abort.is_aborted() {
                return Err(format!("{LABEL} archival-document validation aborted"));
            }
            let entry =
                entry.map_err(|error| format!("failed to inspect archive entry: {error}"))?;
            let file_name = entry.file_name();
            let Some(name) = file_name.to_str() else {
                continue;
            };
            if name.eq_ignore_ascii_case(ARCHIVE_NAME) {
                matched_names.push(name.to_string());
            }
        }
        if matched_names.as_slice() != [ARCHIVE_NAME] {
            return Err(format!(
                "{LABEL} archival-document case/census changed: {matched_names:?}"
            ));
        }

        let metadata = fs::symlink_metadata(&archive_path)
            .map_err(|error| format!("failed to inspect {LABEL} archival document: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "{LABEL} archival document must be a regular non-symlink file"
            ));
        }
        let expected = HISTORICAL
            .iter()
            .find(|record| record.0 == ARCHIVE_PATH)
            .ok_or_else(|| format!("{LABEL} lost its archival-document identity"))?;
        let cap = expected
            .1
            .checked_mul(2)
            .and_then(|value| value.checked_add(3))
            .ok_or_else(|| format!("{LABEL} archival-document size bound overflowed"))?;
        if metadata.len() > cap as u64 {
            return Err(format!(
                "{LABEL} archival document exceeds its bounded envelope"
            ));
        }
        let mut bytes = Vec::with_capacity((metadata.len() as usize).min(cap));
        fs::File::open(&archive_path)
            .map_err(|error| format!("failed to open {LABEL} archival document: {error}"))?
            .take((cap + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("failed to read {LABEL} archival document: {error}"))?;
        if bytes.len() > cap || abort.is_aborted() {
            return Err(format!(
                "{LABEL} archival-document bounded read grew or was aborted"
            ));
        }
        let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
        let sha = format!("{:x}", Sha256::digest(&canonical));
        let b3 = blake3::hash(&canonical).to_hex().to_string();
        if canonical.len() != expected.1 || sha != expected.3 || b3 != expected.4 {
            return Err(format!(
                "{LABEL} archival document changed: bytes={}, sha={sha}, blake3={b3}",
                canonical.len()
            ));
        }

        let text = std::str::from_utf8(&canonical)
            .map_err(|error| format!("{LABEL} archival document is not UTF-8: {error}"))?;
        let lines = text.lines().collect::<Vec<_>>();
        let cross_reference_lines = lines
            .iter()
            .enumerate()
            .filter_map(|(index, line)| (*line == BUG51_CROSS_REFERENCE).then_some(index + 1))
            .collect::<Vec<_>>();
        let rationale = lines
            .get(ARCHIVAL_RATIONALE_FIRST_LINE - 1..=ARCHIVAL_RATIONALE_FIRST_LINE)
            .map(|lines| format!("{}\n{}\n", lines[0], lines[1]));
        if cross_reference_lines != [88, 123]
            || rationale.as_deref().map(str::as_bytes) != Some(ARCHIVAL_RATIONALE)
        {
            return Err(format!(
                "{LABEL} archival rationale/cross-reference changed: BUG51={cross_reference_lines:?}"
            ));
        }
        Ok(canonical)
    }

    fn validate_bug42_provenance(
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
                "{LABEL} tags=exclude archive must not acquire a manifest exclusion: {family_exclusions:?}"
            ));
        }
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        self.reject_wrapper_output_artifacts(&self.root.join(OWNER_PATH))
            .map_err(|error| format!("{LABEL} {OWNER_PATH} {error}"))?;
        let mut sources = self.read_bug42_directory(abort)?;
        let archive = self.read_bug42_archive(abort)?;
        if sources
            .insert(ARCHIVE_MAP_KEY.to_string(), archive)
            .is_some()
        {
            return Err(format!("{LABEL} archival source-map key collided"));
        }
        Ok(sources)
    }

    fn validate_bug42_source_shape(source: &str) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if !source.ends_with('\n')
            || lines.len() != 9
            || lines[0] != "*Test to make sure that .lib is a synonym for .inc"
            || !lines[1].is_empty()
            || lines[2] != ".lib resistance.txt"
            || !lines[3].is_empty()
            || lines[4] != "V1 1 0 DC 1"
            || lines[5] != "R1 1 0 res"
            || !lines[6].is_empty()
            || lines[7] != ".DC V1 1 1 0.1"
            || lines[8] != ".END"
        {
            return Err(format!("{LABEL} exact nine-line source envelope changed"));
        }
        Ok(())
    }

    fn bug42_parse_options() -> NetlistParseOptions {
        NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        }
    }

    fn parse_bug42_expected_failure(
        source: &str,
        deck_path: &Path,
        mode: Bug42ParseMode,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let parsed = match mode {
            Bug42ParseMode::Filesystem => Netlist::parse_with_path_and_options_and_abort(
                source,
                deck_path,
                Self::bug42_parse_options(),
                abort,
            ),
            Bug42ParseMode::SealedRootOnly => {
                let sealed =
                    SealedSourceBundle::try_new([(deck_path.to_path_buf(), source.to_string())])
                        .map_err(|error| format!("{LABEL} sealed root is invalid: {error}"))?;
                if sealed.len() != 1 {
                    return Err(format!("{LABEL} sealed-root census changed"));
                }
                Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
                    source,
                    deck_path,
                    sealed,
                    Self::bug42_parse_options(),
                    abort,
                )
            }
        };
        match parsed {
            Err(rspice_core::netlist::ParseWithAbortError::Parse(ParseError::Syntax {
                line,
                message,
            })) if line == EXPECTED_PARSE_LINE && message == EXPECTED_PARSE_MESSAGE => Ok(()),
            Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => Err(format!(
                "{LABEL} {mode:?} produced the wrong typed parse failure: {error:?}"
            )),
            Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                Err(format!("{LABEL} {mode:?} parsing exceeded its deadline"))
            }
            Ok(_) => Err(format!("{LABEL} {mode:?} unexpectedly parsed successfully")),
        }
    }

    fn bug42_upstream_diagnostic(source: &str) -> Result<String, String> {
        let directive = source
            .lines()
            .nth(EXPECTED_PARSE_LINE - 1)
            .ok_or_else(|| format!("{LABEL} lost its .LIB line"))?;
        let tokens = directive.split_ascii_whitespace().collect::<Vec<_>>();
        if tokens.len() != 2 || !tokens[0].eq_ignore_ascii_case(".LIB") {
            return Err(format!("{LABEL} .LIB diagnostic source changed"));
        }
        let library = tokens[1].to_ascii_uppercase();
        Ok(format!(
            "Netlist error: Could not find .ENDL statement for '.LIB {library}'.  Maybe '.LIB {library} <library_name>' or '.INC {library}' was intended."
        ))
    }

    fn bug42_wrapper_exit_code(simulator_exit: i32) -> i32 {
        if simulator_exit == 0 { 0 } else { 10 }
    }

    fn bug42_positive_include_netlists(
        &self,
        members: &BTreeMap<String, Vec<u8>>,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, Netlist), String> {
        let source = std::str::from_utf8(
            members
                .get(&OWNER_NAME.to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost its authenticated owner"))?,
        )
        .map_err(|error| format!("{LABEL} owner is not UTF-8: {error}"))?;
        let sidecar = std::str::from_utf8(
            members
                .get(&SIDECAR_NAME.to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost its authenticated include sidecar"))?,
        )
        .map_err(|error| format!("{LABEL} sidecar is not UTF-8: {error}"))?;
        let include_source = source.replacen(".lib resistance.txt", ".inc resistance.txt", 1);
        if include_source == source || include_source.matches(".inc resistance.txt").count() != 1 {
            return Err(format!("{LABEL} positive-control rewrite changed"));
        }
        let filesystem = Netlist::parse_with_path_and_options_and_abort(
            &include_source,
            deck_path,
            Self::bug42_parse_options(),
            abort,
        )
        .map_err(|error| format!("{LABEL} filesystem .INC positive control failed: {error}"))?;
        let sidecar_path = deck_path
            .parent()
            .ok_or_else(|| format!("{LABEL} owner has no family directory"))?
            .join(SIDECAR_NAME);
        let sealed = SealedSourceBundle::try_new_with_edges(
            [
                (deck_path.to_path_buf(), include_source.clone()),
                (sidecar_path.clone(), sidecar.to_string()),
            ],
            [SealedSourceEdge {
                owner: deck_path.to_path_buf(),
                requested_path: SIDECAR_NAME.to_string(),
                target: sidecar_path,
            }],
        )
        .map_err(|error| format!("{LABEL} positive-control closure is invalid: {error}"))?;
        if sealed.len() != 2 {
            return Err(format!("{LABEL} positive-control source census changed"));
        }
        let sealed = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
            &include_source,
            deck_path,
            sealed,
            Self::bug42_parse_options(),
            abort,
        )
        .map_err(|error| format!("{LABEL} sealed .INC positive control failed: {error}"))?;
        Ok((filesystem, sealed))
    }

    fn validate_bug42_positive_include(netlist: &Netlist) -> Result<(), String> {
        let resistance = netlist
            .params
            .get("RES")
            .ok_or_else(|| format!("{LABEL} .INC did not consume resistance.txt"))?;
        if resistance.to_bits() != 1_000.0f64.to_bits()
            || netlist.params.all_params().len() != 1
            || netlist.elements.len() != 2
            || netlist.analyses.len() != 1
            || !netlist.output_requests.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.data_tables.is_empty()
        {
            return Err(format!("{LABEL} .INC positive-control envelope changed"));
        }
        let voltage = &netlist.elements[0];
        if !voltage.name.eq_ignore_ascii_case("V1")
            || voltage.nodes != ["1", "0"]
            || !matches!(
                &voltage.kind,
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == 1.0f64.to_bits()
            )
        {
            return Err(format!("{LABEL} .INC voltage-source topology changed"));
        }
        let resistor = &netlist.elements[1];
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            deferred_params,
        } = &resistor.kind
        else {
            return Err(format!("{LABEL} .INC R1 is no longer a resistor"));
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
            || value.to_bits() != resistance.to_bits()
            || value_expr.is_some()
            || model.is_some()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!("{LABEL} .INC resistor topology changed"));
        }
        let AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2,
        } = &netlist.analyses[0]
        else {
            return Err(format!("{LABEL} .INC lost its DC analysis"));
        };
        if !source.eq_ignore_ascii_case("V1")
            || start.to_bits() != 1.0f64.to_bits()
            || stop.to_bits() != 1.0f64.to_bits()
            || step.to_bits() != 0.1f64.to_bits()
            || !matches!(mode, DcSweepMode::Linear)
            || sweep2.is_some()
        {
            return Err(format!("{LABEL} .INC DC request changed"));
        }
        Ok(())
    }

    pub(super) fn validate_bug42_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let before = self.validate_bug42_provenance(deck, &abort)?;
        let source = std::str::from_utf8(
            before
                .get(&OWNER_NAME.to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost its sealed owner"))?,
        )
        .map_err(|error| format!("{LABEL} owner is not UTF-8: {error}"))?;
        Self::validate_bug42_source_shape(source)?;
        Self::parse_bug42_expected_failure(source, &deck.path, Bug42ParseMode::Filesystem, &abort)?;
        Self::parse_bug42_expected_failure(
            source,
            &deck.path,
            Bug42ParseMode::SealedRootOnly,
            &abort,
        )?;
        let diagnostic = Self::bug42_upstream_diagnostic(source)?;
        if diagnostic != UPSTREAM_DIAGNOSTIC || Self::bug42_wrapper_exit_code(1) != 10 {
            return Err(format!(
                "{LABEL} historical diagnostic/wrapper projection changed: {diagnostic:?}"
            ));
        }
        let (filesystem_positive, sealed_positive) =
            self.bug42_positive_include_netlists(&before, &deck.path, &abort)?;
        Self::validate_bug42_positive_include(&filesystem_positive)?;
        Self::validate_bug42_positive_include(&sealed_positive)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded its deadline"));
        }
        let after = self.validate_bug42_provenance(deck, &abort)?;
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

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: OWNER_PATH.to_string(),
        }
    }

    fn bug42_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug42-{label}-"))
            .tempdir()
            .expect("create BUG42 fixture root");
        let root = temporary.path();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG42 family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG42 member");
        }
        fs::copy(corpus_root().join(ARCHIVE_PATH), root.join(ARCHIVE_PATH))
            .expect("copy BUG42 archival document");
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG42 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG42 exclusions");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug42_route_is_exactly_owner_only() {
        assert_eq!(Bug42SonRole::for_record(OWNER_PATH), Some(Bug42SonRole));
        assert_eq!(Bug42SonRole::for_record(OWNER_RECORD), Some(Bug42SonRole));
        for rejected in [
            "Netlists/Certification_Tests/BUG_42_SON/resistance.txt",
            "Netlists/Certification_Tests/BUG_42_SON/bug42.cir.sh",
            "Netlists/Certification_Tests/BUG_42_SON/sibling.cir",
            "Netlists/Certification_Tests/BUG_42/bug42.cir",
            "OutputData/Certification_Tests/BUG_42_SON/bug42.cir.prn",
        ] {
            assert_eq!(
                Bug42SonRole::for_record(rejected),
                None,
                "accepted {rejected}"
            );
        }
    }

    #[test]
    fn bug42_historical_archive_and_retained_census_are_sealed() {
        XyceTestRunner::validate_bug42_historical_provenance().unwrap();
        assert_eq!(HISTORICAL_TAGS, b"exclude\n");
        assert_eq!(ARCHIVAL_RATIONALE_FIRST_LINE, 299);
        assert_eq!(
            ARCHIVAL_RATIONALE,
            b"BUG 42.  The feature tested by this bug is now deprecated, so the test is \nnow excluded.\n"
        );
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        assert_eq!(runner().read_bug42_directory(&abort).unwrap().len(), 2);
        let archive = runner().read_bug42_archive(&abort).unwrap();
        assert_eq!(archive.len(), 45_712);

        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG42 archive deadline");
        let expired = DeadlineAbort::new(expired_start, 1);
        let failure = runner()
            .read_bug42_archive(&expired)
            .expect_err("expired BUG42 archival read must abort");
        assert!(failure.contains("aborted"), "{failure}");
    }

    #[test]
    fn bug42_checked_in_owner_has_exact_typed_failure_in_both_resolvers() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let members = runner.read_bug42_directory(&abort).unwrap();
        let source = std::str::from_utf8(members.get(OWNER_NAME).unwrap()).unwrap();
        let path = runner.root.join(OWNER_PATH);
        XyceTestRunner::validate_bug42_source_shape(source).unwrap();
        XyceTestRunner::parse_bug42_expected_failure(
            source,
            &path,
            Bug42ParseMode::Filesystem,
            &abort,
        )
        .unwrap();
        XyceTestRunner::parse_bug42_expected_failure(
            source,
            &path,
            Bug42ParseMode::SealedRootOnly,
            &abort,
        )
        .unwrap();
        assert_eq!(
            XyceTestRunner::bug42_upstream_diagnostic(source).unwrap(),
            UPSTREAM_DIAGNOSTIC
        );
    }

    #[test]
    fn bug42_inc_positive_control_consumes_exact_sidecar_and_topology() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let members = runner.read_bug42_directory(&abort).unwrap();
        let (filesystem, sealed) = runner
            .bug42_positive_include_netlists(&members, &runner.root.join(OWNER_PATH), &abort)
            .unwrap();
        XyceTestRunner::validate_bug42_positive_include(&filesystem).unwrap();
        XyceTestRunner::validate_bug42_positive_include(&sealed).unwrap();

        let mut poisoned = members;
        poisoned.insert(SIDECAR_NAME.to_string(), b".param res = 2K\n".to_vec());
        let (filesystem, poisoned) = runner
            .bug42_positive_include_netlists(&poisoned, &runner.root.join(OWNER_PATH), &abort)
            .unwrap();
        XyceTestRunner::validate_bug42_positive_include(&filesystem).unwrap();
        assert!(XyceTestRunner::validate_bug42_positive_include(&poisoned).is_err());
    }

    #[test]
    fn bug42_stale_wrapper_exit_semantics_are_exact() {
        assert_eq!(XyceTestRunner::bug42_wrapper_exit_code(0), 0);
        for exit in [-9, -1, 1, 2, 10, 127] {
            assert_eq!(XyceTestRunner::bug42_wrapper_exit_code(exit), 10);
        }
    }

    #[test]
    fn bug42_full_oracle_observes_its_shared_deadline() {
        let (_temporary, deck, runner) = bug42_fixture("deadline");
        let expired_ms = u64::try_from(runner.config.max_time_per_test_ms.saturating_add(1))
            .expect("BUG42 test deadline fits u64 milliseconds");
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(expired_ms))
            .expect("construct expired BUG42 full-oracle deadline");
        let failure = runner
            .validate_bug42_oracle(&deck, expired_start)
            .expect_err("expired BUG42 full oracle must abort");
        assert!(failure.contains("aborted"), "{failure}");
    }

    #[test]
    fn bug42_counterfactuals_fail_closed() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let members = runner.read_bug42_directory(&abort).unwrap();
        let source = std::str::from_utf8(members.get(OWNER_NAME).unwrap()).unwrap();
        let path = runner.root.join(OWNER_PATH);

        let repaired = source.replace(".lib resistance.txt", ".inc resistance.txt");
        assert!(
            XyceTestRunner::parse_bug42_expected_failure(
                &repaired,
                &path,
                Bug42ParseMode::Filesystem,
                &abort,
            )
            .is_err()
        );
        let wrong_library = source.replace("resistance.txt", "other.txt");
        assert!(
            XyceTestRunner::parse_bug42_expected_failure(
                &wrong_library,
                &path,
                Bug42ParseMode::SealedRootOnly,
                &abort,
            )
            .is_err()
        );
        let relocated = format!(
            "{}\n{}",
            &source[..source.find(".lib").unwrap()],
            &source[source.find(".lib").unwrap()..]
        );
        assert!(
            XyceTestRunner::parse_bug42_expected_failure(
                &relocated,
                &path,
                Bug42ParseMode::SealedRootOnly,
                &abort,
            )
            .is_err()
        );
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG42 deadline");
        let expired = DeadlineAbort::new(expired_start, 1);
        let failure = XyceTestRunner::parse_bug42_expected_failure(
            source,
            &path,
            Bug42ParseMode::Filesystem,
            &expired,
        )
        .expect_err("expired BUG42 parse must abort");
        assert!(failure.contains("deadline"), "{failure}");
    }

    #[test]
    fn bug42_provenance_rejects_source_metadata_and_artifact_drift() {
        let assert_valid = |runner: &XyceTestRunner, deck: &XyceDeck| {
            let abort = DeadlineAbort::new(Instant::now(), 30_000);
            runner
                .validate_bug42_provenance(deck, &abort)
                .expect("canonical BUG42 fixture is valid");
        };
        let assert_rejected = |runner: &XyceTestRunner, deck: &XyceDeck| {
            let abort = DeadlineAbort::new(Instant::now(), 30_000);
            assert!(runner.validate_bug42_provenance(deck, &abort).is_err());
        };

        let (_temporary, deck, runner) = bug42_fixture("source");
        assert_valid(&runner, &deck);
        fs::write(&deck.path, "* drift\n").expect("mutate BUG42 owner");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("sidecar");
        assert_valid(&runner, &deck);
        fs::write(
            runner.root.join(FAMILY_DIRECTORY).join(SIDECAR_NAME),
            ".param res = 2K\n",
        )
        .expect("mutate BUG42 sidecar");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("archive");
        assert_valid(&runner, &deck);
        fs::write(runner.root.join(ARCHIVE_PATH), "archival drift\n")
            .expect("mutate BUG42 archival document");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("archive-case");
        assert_valid(&runner, &deck);
        let archive = runner.root.join(ARCHIVE_PATH);
        let staged = archive.with_file_name("son-bugs.case-change");
        fs::rename(&archive, &staged).expect("stage BUG42 archive case drift");
        fs::rename(
            &staged,
            archive.with_file_name("son_bugs_not_needing_test_cases"),
        )
        .expect("apply BUG42 archive case drift");
        assert_rejected(&runner, &deck);

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;

            let (_temporary, deck, runner) = bug42_fixture("archive-symlink");
            assert_valid(&runner, &deck);
            let archive = runner.root.join(ARCHIVE_PATH);
            let replacement = archive.with_file_name("SON_BUGS_ARCHIVE_REAL");
            fs::rename(&archive, &replacement).expect("stage BUG42 archive symlink");
            symlink(&replacement, &archive).expect("create BUG42 archive symlink");
            assert_rejected(&runner, &deck);
        }

        let (_temporary, deck, runner) = bug42_fixture("manifest");
        assert_valid(&runner, &deck);
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove BUG42 wrapper owner");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("exclusion");
        assert_valid(&runner, &deck);
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{OWNER_PATH}\tNetlists/Certification_Tests/BUG_42_SON/tags\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG42 exclusion");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("extra-member");
        assert_valid(&runner, &deck);
        fs::write(runner.root.join(FAMILY_DIRECTORY).join("unexpected"), "x")
            .expect("add BUG42 member");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("non-file-member");
        assert_valid(&runner, &deck);
        fs::create_dir(runner.root.join(FAMILY_DIRECTORY).join("nested"))
            .expect("add non-file BUG42 member");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("case");
        assert_valid(&runner, &deck);
        let family = runner.root.join(FAMILY_DIRECTORY);
        let intermediate = family.join("resistance.case-change");
        fs::rename(family.join(SIDECAR_NAME), &intermediate).expect("stage BUG42 case drift");
        fs::rename(intermediate, family.join("Resistance.txt")).expect("apply BUG42 case drift");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("output");
        assert_valid(&runner, &deck);
        fs::create_dir_all(runner.root.join(OUTPUT_DIRECTORY)).expect("create forbidden gold");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("artifact");
        assert_valid(&runner, &deck);
        fs::write(deck.path.with_extension("cir.out"), "invented output")
            .expect("create forbidden wrapper artifact");
        assert_rejected(&runner, &deck);

        let (_temporary, deck, runner) = bug42_fixture("wrong-section");
        assert_valid(&runner, &deck);
        let wrong_section = XyceDeck {
            path: deck.path.clone(),
            section: XyceDeckSection::Other,
            relative_path: deck.relative_path.clone(),
        };
        assert_rejected(&runner, &wrong_section);

        let (_temporary, deck, runner) = bug42_fixture("wrong-path");
        assert_valid(&runner, &deck);
        let wrong_path = XyceDeck {
            path: deck.path.clone(),
            section: deck.section,
            relative_path: "Netlists/Certification_Tests/BUG_42_SON/sibling.cir".into(),
        };
        assert_rejected(&runner, &wrong_path);
    }

    #[test]
    fn bug42_checked_in_owner_executes_complete_oracle() {
        let runner = runner();
        let result = runner.run_test(runner.root.join(OWNER_PATH));
        assert!(result.passed, "BUG42 failed: {:?}", result.error);
        assert_eq!(result.contract, CONTRACT);
        assert!(!result.expected_unsupported);
        assert!(!result.upstream_excluded);
    }
}
