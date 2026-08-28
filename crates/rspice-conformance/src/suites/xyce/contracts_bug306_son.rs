use super::*;
use rspice_core::netlist::SourceSpec;
use std::io::Read as _;

const LABEL: &str = "BUG_306_SON numeric/string TIMEINT METHOD equivalence";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_306_SON";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_306_son/";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_306_SON/exclude";
const UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";

const GEAR_OWNER_CONTRACT: &str = "bug306_gear_numeric_method_wrapper_owner";
const GEAR_CONTROL_CONTRACT: &str = "bug306_gear_string_method_release_control";
const TRAP_OWNER_CONTRACT: &str = "bug306_trap_numeric_method_wrapper_owner";
const TRAP_CONTROL_CONTRACT: &str = "bug306_trap_string_method_release_control";
const TRAPEZOIDAL_CONTROL_CONTRACT: &str = "bug306_trapezoidal_string_method_release_control";

const HISTORICAL_RECORD_COUNT: usize = 10;
const HISTORICAL_RECORD_BYTES: usize = 2_777;
const HISTORICAL_RECORDS_SHA256: &str =
    "4ff6a94cafb3641006dc1280f8d82633f8d975223146824f56212e7fd8ebc41c";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "04bb09f05dcc824f2fda4e52a86a725692f11f2ea5cffeff08dda86e9871c943";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); HISTORICAL_RECORD_COUNT] = [
    (
        "Netlists/Certification_Tests/BUG_306_SON/CMakeLists.txt",
        3_581,
        "6d1b784b2c8ce7357e377a3a7d9aeb6b4dddcb60cafd5fc59f7087733aab8a45",
        "163e7002bc1ba6ea4477289bdc58be1e85349d05fcecd4827352d8ed6dc06057",
    ),
    (
        "Netlists/Certification_Tests/BUG_306_SON/Manifest.txt",
        170,
        "edf20fdc030d28bd22a17f03e8ff64a8e486a86f454413ef6e946256aac75bbe",
        "0d45b6f3a1b64276da569c1fef3a6cc68fb49fb86501b0d09bff054a4c8875e1",
    ),
    (
        "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_gear.cir.sh",
        1_249,
        "857e332ab89989301cb6d40c0f8d352829a0543d89b191524d524391aff39351",
        "01cd10d8b16d23ecd1a5894420f3100c868aa5580c1dd2087d8cfa092e33c938",
    ),
    (
        "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap.cir.sh",
        1_422,
        "079dac0c649f064dfea17117f378e087b713ccb704d2e4473eebf29150428349",
        "77d06d00cabf5953182fb15e1e684f604150d7cd2b20df6644481d0d139b55fd",
    ),
    (
        EXCLUSION_SOURCE,
        80,
        "f5e64c2a1872c6d921636b543675bc6017719e67c30788957284ee5ac298f088",
        "50e42484fb86896e1d26bc6673dc49944243edf13d79c2a8f8434ee2b3d861da",
    ),
    (
        "Netlists/Certification_Tests/BUG_306_SON/tags",
        38,
        "56cf09829c83d897e6f322221f1ca6552e0e3ceae0eb5fd0fab85a6c28b0315f",
        "b01fc5545144483490f647d622fd38227bc9d12d90e5e91f60b4040e0749157d",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
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
];

const RETAINED_RECORD_COUNT: usize = 5;
const RETAINED_RECORD_BYTES: usize = 989;
const RETAINED_RECORDS_SHA256: &str =
    "8c7c7566053ea4c53b14804557c942bf72d291b122843360bb8c35b10641cd13";
const RETAINED_RECORDS_BLAKE3: &str =
    "7667f73b8b6d9c4d047b1ec2c07b72468fd39e84c9fc24295f690786dfc7984f";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); RETAINED_RECORD_COUNT] = [
    (
        "lead_bjt_gear.cir",
        726,
        "f8d88aa940bed3cb5f0bc9ccaf9077c2bf96a47ad0747a524973466ecd613b1b",
        "2371cbc7452ba811a919f782c57c38b650a41e4ddff43cb5e1ee25b4be7736b8",
    ),
    (
        "lead_bjt_gear_strings.cir",
        729,
        "6fc42c5e5625d3d6f405a39f85851b1d39ce58b8ff3ba4d07ef2396abd293314",
        "bfb6d3921bc3861ce654ce37d8538c3a9e96a88a834d1f18c4d0a2daafe84162",
    ),
    (
        "lead_bjt_trap.cir",
        726,
        "642a3e6728105cc39abd025891efca018e82a09dd2d5ed8811d03bc216a27643",
        "94695b26b1c6eb5443c9f68676f347493bf10603d5ec0cc39eb3acdd6b951063",
    ),
    (
        "lead_bjt_trap_strings.cir",
        729,
        "1471b50a17fd4eecc3b515d371f88cc61ba6363bb39fed9b78bd92fd3d2e0fbb",
        "0984f2ee5322763286df071235888356cc771dca8dc3ba16d3ec780fdc002141",
    ),
    (
        "lead_bjt_trap_strings2.cir",
        736,
        "8ac56917163731b3d5d4a4da0cda3fcb72ff4e3f23dc606d02d8b4987cfb40fd",
        "5b6d2f0f7fcccd321e24725826c5a9e493cdb9a396d16a5088488550b5379c4b",
    ),
];

const RESULT_COLUMNS: usize = 5;
const PRINT_PRECISION: usize = 10;
const PRINT_WIDTH: usize = 19;
const MAX_ADAPTIVE_RESULT_POINTS: usize = 4_096;
const MAX_KCL_RESIDUAL: Value = 1.0e-6;
// Xyce-suffix parsing evaluates `20us` as `20.0 * 1.0e-6`; preserve that
// canonical parsed value rather than silently substituting the adjacent
// literal produced by spelling `20.0e-6` directly in Rust.
const TRANSIENT_STOP: Value = 1.999_999_999_999_999_8e-5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug306SonRole {
    GearNumericOwner,
    GearStringControl,
    TrapNumericOwner,
    TrapStringControl,
    TrapezoidalStringControl,
}

impl Bug306SonRole {
    const ALL: [Self; 5] = [
        Self::GearNumericOwner,
        Self::GearStringControl,
        Self::TrapNumericOwner,
        Self::TrapStringControl,
        Self::TrapezoidalStringControl,
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::GearNumericOwner => "lead_bjt_gear.cir",
            Self::GearStringControl => "lead_bjt_gear_strings.cir",
            Self::TrapNumericOwner => "lead_bjt_trap.cir",
            Self::TrapStringControl => "lead_bjt_trap_strings.cir",
            Self::TrapezoidalStringControl => "lead_bjt_trap_strings2.cir",
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::GearNumericOwner => "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_gear.cir",
            Self::GearStringControl => {
                "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_gear_strings.cir"
            }
            Self::TrapNumericOwner => "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap.cir",
            Self::TrapStringControl => {
                "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap_strings.cir"
            }
            Self::TrapezoidalStringControl => {
                "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap_strings2.cir"
            }
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::GearNumericOwner => "netlists/certification_tests/bug_306_son/lead_bjt_gear.cir",
            Self::GearStringControl => {
                "netlists/certification_tests/bug_306_son/lead_bjt_gear_strings.cir"
            }
            Self::TrapNumericOwner => "netlists/certification_tests/bug_306_son/lead_bjt_trap.cir",
            Self::TrapStringControl => {
                "netlists/certification_tests/bug_306_son/lead_bjt_trap_strings.cir"
            }
            Self::TrapezoidalStringControl => {
                "netlists/certification_tests/bug_306_son/lead_bjt_trap_strings2.cir"
            }
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::GearNumericOwner => GEAR_OWNER_CONTRACT,
            Self::GearStringControl => GEAR_CONTROL_CONTRACT,
            Self::TrapNumericOwner => TRAP_OWNER_CONTRACT,
            Self::TrapStringControl => TRAP_CONTROL_CONTRACT,
            Self::TrapezoidalStringControl => TRAPEZOIDAL_CONTROL_CONTRACT,
        }
    }

    fn authored_method(self) -> &'static str {
        match self {
            Self::GearNumericOwner => "8",
            Self::GearStringControl => "gear",
            Self::TrapNumericOwner => "7",
            Self::TrapStringControl => "trap",
            Self::TrapezoidalStringControl => "trapezoidal",
        }
    }

    fn is_owner(self) -> bool {
        matches!(self, Self::GearNumericOwner | Self::TrapNumericOwner)
    }

    fn purpose(self) -> XyceStaticTranPlanPurpose {
        if self.is_owner() {
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
        } else {
            XyceStaticTranPlanPurpose::RelationalFamily
        }
    }
}

#[derive(Debug)]
struct Bug306Worker {
    plan: XyceStaticTranPlan,
    netlist: Netlist,
}

impl XyceTestRunner {
    fn validate_bug306_print_syntax(source: &str) -> Result<(), String> {
        let directives = source
            .lines()
            .map(Self::strip_netlist_comment)
            .map(str::trim)
            .filter(|line| line.to_ascii_lowercase().starts_with(".print"))
            .collect::<Vec<_>>();
        let [directive] = directives.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one authored .PRINT directive"
            ));
        };
        let tokens = directive.split_ascii_whitespace().collect::<Vec<_>>();
        if tokens.len() != 7
            || !tokens[0].eq_ignore_ascii_case(".print")
            || !tokens[1].eq_ignore_ascii_case("tran")
            || !tokens[2].eq_ignore_ascii_case(&format!("precision={PRINT_PRECISION}"))
            || !tokens[3].eq_ignore_ascii_case(&format!("width={PRINT_WIDTH}"))
            || !tokens[4].eq_ignore_ascii_case("{i(vib)-ib(q1)}")
            || !tokens[5].eq_ignore_ascii_case("{i(vic)-ic(q1)}")
            || !tokens[6].eq_ignore_ascii_case("{i(vie)-ie(q1)}")
        {
            return Err(format!(
                "{LABEL} requires exact TRAN PRECISION={PRINT_PRECISION} WIDTH={PRINT_WIDTH} and ordered expression probes, got {directive:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug306_comp_tolerances(
        source: &str,
    ) -> Result<Vec<XyceVerifyTransientTolerance>, String> {
        let columns = [
            "{I(VIB)-IB(Q1)}".to_string(),
            "{I(VIC)-IC(Q1)}".to_string(),
            "{I(VIE)-IE(Q1)}".to_string(),
        ];
        let tolerances = Self::xyce_verify_comp_tolerances(source, &columns)?;
        if tolerances.len() != columns.len()
            || tolerances.iter().any(|tolerance| {
                tolerance.relative.to_bits() != 0.01f64.to_bits()
                    || tolerance.absolute.to_bits() != 1.0e-6f64.to_bits()
                    || tolerance.zero.to_bits() != 1.0e-7f64.to_bits()
                    || tolerance.absolute_difference.to_bits() != 1.0e-12f64.to_bits()
                    || tolerance.offset.to_bits() != 0.0f64.to_bits()
            })
        {
            return Err(format!(
                "{LABEL} Release *COMP tolerances changed: {tolerances:?}"
            ));
        }
        Ok(tolerances)
    }

    fn validate_bug306_historical(artifacts: &[(&str, usize, &str, &str)]) -> Result<(), String> {
        let mut records = artifacts
            .iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{PRETRIM_COMMIT}\t{UPSTREAM_REGRESSION_COMMIT}\t{UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || artifacts.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
            || content_blake3 != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 provenance changed: records={}, bytes={}, sha256={sha256}, blake3={content_blake3}",
                artifacts.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug306_retained_records() -> Result<(), String> {
        let mut records = RETAINED_ARTIFACTS
            .iter()
            .map(|(name, bytes, sha256, content_blake3)| {
                format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha256}\t{content_blake3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || sha256 != RETAINED_RECORDS_SHA256
            || content_blake3 != RETAINED_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} retained-record identity changed: records={}, bytes={}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn read_bug306_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug306_retained_records()?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} source census aborted"));
        }
        let directory = self.root.join(FAMILY_DIRECTORY);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let expected = RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
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
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
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
                .ok_or_else(|| format!("{LABEL} retained-size bound overflowed"))?;
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
            if bytes.len() > cap || abort.is_aborted() {
                return Err(format!("{LABEL} bounded source read grew or aborted"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha256={sha256}, blake3={content_blake3}",
                    canonical.len()
                ));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{LABEL} retained five-member census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug306_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug306SonRole,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug306_historical(&HISTORICAL_ARTIFACTS)?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("{LABEL} recognized role is not canonical"));
        }
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners
            != BTreeSet::from([
                Bug306SonRole::GearNumericOwner.record(),
                Bug306SonRole::TrapNumericOwner.record(),
            ])
        {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions invalid: {error}"))?;
        for owner in [
            Bug306SonRole::GearNumericOwner,
            Bug306SonRole::TrapNumericOwner,
        ] {
            if exclusions.contains_key(owner.record()) {
                return Err(format!(
                    "{LABEL} owner {} became excluded",
                    owner.file_name()
                ));
            }
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeMap<_, _>>();
        if family.len() != 3 {
            return Err(format!("{LABEL} exclusion census changed: {family:?}"));
        }
        for control in [
            Bug306SonRole::GearStringControl,
            Bug306SonRole::TrapStringControl,
            Bug306SonRole::TrapezoidalStringControl,
        ] {
            let entry = family
                .get(&control.record().to_string())
                .copied()
                .ok_or_else(|| format!("{LABEL} lost {} qualification", control.file_name()))?;
            if entry.source != EXCLUSION_SOURCE
                || !matches!(&entry.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == control.contract())
            {
                return Err(format!(
                    "{LABEL} {} qualification changed: {entry:?}",
                    control.file_name()
                ));
            }
        }
        let members = self.read_bug306_directory(abort)?;
        let output = self.root.join("OutputData/Certification_Tests/BUG_306_SON");
        match fs::symlink_metadata(&output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        for member in Bug306SonRole::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member.path()))
                .map_err(|error| format!("{LABEL} {} {error}", member.file_name()))?;
        }
        Ok(members)
    }

    fn bug306_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug306_model(model: &rspice_core::netlist::ModelDef) -> bool {
        let expected: [(&str, Value); 38] = [
            ("BF", 130.0),
            ("BR", 1.0),
            ("IS", 3.0e-14),
            ("TF", 1.0e-8),
            ("VJS", 0.68),
            ("NE", 1.6),
            ("NC", 2.0),
            ("RB", 450.0),
            ("CJE", 1.0e-6),
            ("CJC", 1.0e-6),
            ("CJS", 1.0e-6),
            ("IKF", 0.002),
            ("IKR", 0.002),
            ("VAF", 50.0),
            ("VAR", 50.0),
            ("NF", 1.0),
            ("ISE", 0.0),
            ("NR", 1.0),
            ("ISC", 0.0),
            ("IRB", 0.0),
            ("RBM", 450.0),
            ("RE", 0.0),
            ("RC", 0.0),
            ("VJE", 0.75),
            ("MJE", 0.33),
            ("XTF", 0.0),
            ("VTF", 100.0),
            ("ITF", 0.0),
            ("PTF", 0.0),
            ("VJC", 0.75),
            ("MJS", 0.0),
            ("XTB", 0.0),
            ("EG", 1.11),
            ("XTI", 3.0),
            ("KF", 0.0),
            ("AF", 1.0),
            ("FC", 0.5),
            ("TNOM", 27.0),
        ];
        model.name.eq_ignore_ascii_case("qjunk")
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

    fn bug306_plan_from_netlist(
        role: Bug306SonRole,
        path: &Path,
        source: &str,
        netlist: &Netlist,
    ) -> Result<XyceStaticTranPlan, String> {
        if Self::contains_control_block(source) {
            return Err(format!("{LABEL} must not acquire a control block"));
        }
        Self::reject_unsupported_source_directives(source)?;
        Self::validate_bug306_print_syntax(source)?;
        let output = Self::single_tran_print_output_request(source)?;
        if output.format.is_some() || output.file.is_some() || output.probes.len() != 3 {
            return Err(format!("{LABEL} {} PRINT schema changed", role.file_name()));
        }
        let plan = XyceStaticTranPlan {
            deck_path: path.to_path_buf(),
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: output.probes,
            }),
            output_override: false,
            timeint_conststep: Self::source_enables_constant_time_step_output(source),
            tran: Self::single_tran_analysis(netlist)?,
            steps: Self::step_commands(netlist)?,
            contract: if role.is_owner() {
                XyceStaticTranContract::WrapperStatic
            } else {
                XyceStaticTranContract::PlainStatic
            },
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        plan.validate_oracle_contract(role.purpose(), role.is_owner())?;
        Ok(plan)
    }

    fn validate_bug306_netlist(
        role: Bug306SonRole,
        netlist: &Netlist,
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        let probes = plan
            .print
            .as_ref()
            .map(|request| {
                request
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if probes != ["{i(vib)-ib(q1)}", "{i(vic)-ic(q1)}", "{i(vie)-ie(q1)}"]
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.tran.step.to_bits() != 1.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != TRANSIENT_STOP.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Self::validate_bug306_comp_tolerances(&plan.source)?;
        if netlist.title.trim_end() != "Lead current test for level 1 BJT"
            || netlist.elements.len() != 4
            || netlist.models.len() != 1
            || !Self::validate_bug306_model(&netlist.models[0])
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
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
            || !netlist
                .options
                .method
                .as_deref()
                .is_some_and(|method| method.eq_ignore_ascii_case(role.authored_method()))
            || netlist.options.nonlin_transient_rhstol.map(Value::to_bits)
                != Some(1.0e-7f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed: elements={:?}, models={:?}, outputs={:?}, options={:?}, diagnostics={:?}",
                role.file_name(),
                netlist.elements,
                netlist.models,
                netlist.output_requests,
                netlist.options,
                netlist.diagnostics
            ));
        }
        let [vie, vic, vib, transistor] = netlist.elements.as_slice() else {
            unreachable!("BUG306 element count was checked")
        };
        let plain_dc_source = |element: &rspice_core::netlist::Element,
                               name: &str,
                               nodes: &[&str],
                               expected: Value| {
            element.provenance == ElementProvenance::Authored
                && element.name.eq_ignore_ascii_case(name)
                && Self::bug306_nodes_match(&element.nodes, nodes)
                && matches!(&element.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                    if value.to_bits() == expected.to_bits())
        };
        if !plain_dc_source(vie, "vie", &["0", "1"], 0.0)
            || !plain_dc_source(vic, "vic", &["0", "3"], 5.0)
            || vib.provenance != ElementProvenance::Authored
            || !vib.name.eq_ignore_ascii_case("vib")
            || !Self::bug306_nodes_match(&vib.nodes, &["0", "2"])
            || !matches!(&vib.kind, ElementKind::VoltageSource(SourceSpec::Pulse {
                v1, v2, delay, rise, fall, width, ..
            }) if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 1.0f64.to_bits()
                && delay.to_bits() == 1.0e-9f64.to_bits()
                && rise.to_bits() == 1.0e-9f64.to_bits()
                && fall.to_bits() == 1.0e-9f64.to_bits()
                && width.to_bits() == 1.0e-6f64.to_bits())
            || transistor.provenance != ElementProvenance::Authored
            || !transistor.name.eq_ignore_ascii_case("q1")
            || !Self::bug306_nodes_match(&transistor.nodes, &["3", "2", "1"])
            || !matches!(&transistor.kind, ElementKind::Bjt {
                model,
                bjt_type: rspice_core::netlist::BjtType::Npn,
                instance_params,
                deferred_params,
            } if model.eq_ignore_ascii_case("qjunk")
                && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!(
                "{LABEL} {} native topology changed",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn prepare_bug306_worker(
        &self,
        members: &BTreeMap<String, Vec<u8>>,
        role: Bug306SonRole,
        abort: &dyn AbortSignal,
    ) -> Result<Bug306Worker, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before planning"));
        }
        let bytes = members
            .get(&role.file_name().to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost {}", role.file_name()))?;
        let source = std::str::from_utf8(bytes)
            .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", role.file_name()))?
            .to_string();
        if source.lines().any(|line| {
            let line = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            line.starts_with(".inc") || line.starts_with(".include") || line.starts_with(".lib")
        }) {
            return Err(format!(
                "{LABEL} unexpectedly acquired a filesystem dependency"
            ));
        }
        let path = self.root.join(role.path());
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        };
        let netlist =
            Netlist::parse_with_path_and_options_and_abort(&source, &path, options, abort)
                .map_err(|error| format!("{LABEL} {} parse failed: {error}", role.file_name()))?;
        let plan = Self::bug306_plan_from_netlist(role, &path, &source, &netlist)?;
        Self::validate_bug306_netlist(role, &netlist, &plan)?;
        Ok(Bug306Worker { plan, netlist })
    }

    fn validate_bug306_table(role: Bug306SonRole, table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != RESULT_COLUMNS
            || !(2..=MAX_ADAPTIVE_RESULT_POINTS).contains(&table.rows.len())
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || table.columns[2..]
                .iter()
                .map(|column| Self::normalize_probe(column))
                .ne(["{i(vib)-ib(q1)}", "{i(vic)-ic(q1)}", "{i(vie)-ie(q1)}"])
        {
            return Err(format!(
                "{LABEL} {} table schema changed: columns={:?}, rows={}",
                role.file_name(),
                table.columns,
                table.rows.len()
            ));
        }
        let scalar_count = table
            .rows
            .len()
            .checked_mul(table.columns.len())
            .ok_or_else(|| format!("{LABEL} result scalar count overflowed"))?;
        if scalar_count > MAX_ADAPTIVE_RESULT_POINTS * RESULT_COLUMNS {
            return Err(format!("{LABEL} result resource envelope changed"));
        }
        let mut max_residual = 0.0f64;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != RESULT_COLUMNS
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || row[1] < 0.0
                || row[1] > TRANSIENT_STOP
                || (index > 0 && row[1] <= table.rows[index - 1][1])
            {
                return Err(format!(
                    "{LABEL} {} row {index} is malformed: {row:?}",
                    role.file_name()
                ));
            }
            max_residual = row[2..]
                .iter()
                .map(|value| value.abs())
                .fold(max_residual, Value::max);
        }
        if table
            .rows
            .first()
            .is_none_or(|row| row[1].to_bits() != 0.0f64.to_bits())
            || table
                .rows
                .last()
                .is_none_or(|row| row[1].to_bits() != TRANSIENT_STOP.to_bits())
            || max_residual > MAX_KCL_RESIDUAL
        {
            return Err(format!(
                "{LABEL} {} KCL/endpoints changed: max residual={max_residual:e}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn execute_bug306_worker(
        &self,
        role: Bug306SonRole,
        worker: &Bug306Worker,
        start: Instant,
        abort: &dyn AbortSignal,
    ) -> Result<XycePrnTable, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let result = self
            .run_transient_family_netlist(&worker.plan, &worker.netlist, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!("{LABEL} exceeded its shared deadline"),
                other => format!("{LABEL} {} execution failed: {other}", role.file_name()),
            })?;
        let nonempty_branches = result
            .branch_currents
            .iter()
            .filter(|values| !values.is_empty())
            .collect::<Vec<_>>();
        let branch_peak = nonempty_branches
            .iter()
            .flat_map(|values| values.iter())
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        let point_count = result.time.len();
        let device_op_schema = result
            .device_op_traces
            .iter()
            .map(|trace| (trace.device_name.as_str(), trace.parameter.as_str()))
            .collect::<Vec<_>>();
        if !(2..=MAX_ADAPTIVE_RESULT_POINTS).contains(&point_count)
            || result.step_sizes.len() != point_count
            || result.time.iter().any(|value| !value.is_finite())
            || result.node_names != ["1", "3", "2", "Q1.__bint"]
            || result.branch_names != ["VIE", "VIC", "VIB", "Q1.__rb"]
            || result.voltages.len() != result.node_names.len()
            || result.voltages.iter().any(|values| {
                values.len() != point_count || values.iter().any(|value| !value.is_finite())
            })
            || nonempty_branches.len() != result.branch_names.len()
            || nonempty_branches.iter().any(|values| {
                values.len() != point_count || values.iter().any(|value| !value.is_finite())
            })
            || result.device_op_traces.len() != 8
            || device_op_schema
                != [
                    ("Q1", "ic"),
                    ("Q1", "ib"),
                    ("Q1", "ie"),
                    ("Q1", "is"),
                    ("Q1", "vbe"),
                    ("Q1", "vce"),
                    ("Q1", "beta"),
                    ("Q1", "gm"),
                ]
            || result.device_op_traces.iter().any(|trace| {
                trace.values.len() != point_count
                    || trace.values.iter().any(|value| !value.is_finite())
            })
            || !result.digital_traces.is_empty()
            || !result.real_traces.is_empty()
            || !result.store_traces.is_empty()
            || !branch_peak.is_finite()
            || branch_peak < 1.0e-5
        {
            return Err(format!(
                "{LABEL} {} retained result envelope changed: time={}, steps={}, nodes={:?}, branches={:?}, active_branches={}, device_ops={device_op_schema:?}, peak={branch_peak:e}",
                role.file_name(),
                result.time.len(),
                result.step_sizes.len(),
                result.node_names,
                result.branch_names,
                nonempty_branches.len(),
            ));
        }
        let table =
            Self::transient_family_result_to_prn_table(&worker.plan, &worker.netlist, &result)
                .map_err(|error| {
                    format!(
                        "{LABEL} {} PRN projection failed: {error}",
                        role.file_name()
                    )
                })?;
        Self::validate_bug306_table(role, &table)?;
        Ok(table)
    }

    fn compare_bug306_relation(
        &self,
        owner_role: Bug306SonRole,
        owner_source: &str,
        good: &XycePrnTable,
        test_role: Bug306SonRole,
        test: &XycePrnTable,
    ) -> Result<(), String> {
        Self::validate_bug306_table(owner_role, good)?;
        Self::validate_bug306_table(test_role, test)?;
        let tolerances = Self::validate_bug306_comp_tolerances(owner_source)?;
        let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            good,
            test,
            &tolerances,
            PRINT_PRECISION,
        )?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} Release relation {} GOOD -> {} TEST found {} mismatch(es): {mismatches:?}",
                owner_role.file_name(),
                test_role.file_name(),
                mismatches.len()
            ))
        }
    }

    pub(super) fn validate_bug306_son_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug306SonRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} shared deadline expired before provenance"));
        }
        let members = self.validate_bug306_provenance(deck, role, &abort)?;
        let mut workers = BTreeMap::new();
        let mut outputs = BTreeMap::new();
        for member_role in Bug306SonRole::ALL {
            let worker = self.prepare_bug306_worker(&members, member_role, &abort)?;
            let table = self.execute_bug306_worker(member_role, &worker, start, &abort)?;
            workers.insert(member_role, worker);
            outputs.insert(member_role, table);
        }
        for (owner, control) in [
            (
                Bug306SonRole::GearNumericOwner,
                Bug306SonRole::GearStringControl,
            ),
            (
                Bug306SonRole::TrapNumericOwner,
                Bug306SonRole::TrapStringControl,
            ),
            (
                Bug306SonRole::TrapNumericOwner,
                Bug306SonRole::TrapezoidalStringControl,
            ),
        ] {
            let owner_worker = workers.get(&owner).expect("all BUG306 workers ran");
            self.compare_bug306_relation(
                owner,
                &owner_worker.plan.source,
                outputs.get(&owner).expect("all BUG306 outputs exist"),
                control,
                outputs.get(&control).expect("all BUG306 outputs exist"),
            )?;
        }
        self.validate_bug306_provenance(deck, role, &abort)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline after provenance"
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

    fn deck(root: &Path, role: Bug306SonRole) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path().to_string(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug306-{label}-"))
            .tempdir()
            .expect("create BUG306 fixture");
        let family = temporary.path().join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG306 family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG306 member");
        }
        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n",
                Bug306SonRole::GearNumericOwner.path(), Bug306SonRole::TrapNumericOwner.path()
            ),
        ).expect("write BUG306 owner manifest");
        let rows = [
            Bug306SonRole::GearStringControl,
            Bug306SonRole::TrapStringControl,
            Bug306SonRole::TrapezoidalStringControl,
        ]
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
            temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{rows}\n"
            ),
        ).expect("write BUG306 exclusion manifest");
        let runner = XyceTestRunner::new(temporary.path(), Default::default());
        (temporary, runner)
    }

    #[test]
    fn bug306_historical_and_retained_provenance_is_exact() {
        XyceTestRunner::validate_bug306_historical(&HISTORICAL_ARTIFACTS).unwrap();
        XyceTestRunner::validate_bug306_retained_records().unwrap();
        let mut changed = HISTORICAL_ARTIFACTS;
        changed[0].1 += 1;
        assert!(XyceTestRunner::validate_bug306_historical(&changed).is_err());
    }

    #[test]
    fn bug306_all_roles_parse_to_exact_native_level1_bjt_methods() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, Default::default());
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let members = runner.read_bug306_directory(&abort).unwrap();
        for role in Bug306SonRole::ALL {
            let worker = runner
                .prepare_bug306_worker(&members, role, &abort)
                .unwrap();
            assert!(
                worker
                    .netlist
                    .options
                    .method
                    .as_deref()
                    .is_some_and(|method| method.eq_ignore_ascii_case(role.authored_method()))
            );
            let base = runner.create_xyce_static_tran_engine_with_step_sizes(
                None,
                None,
                XyceTestRunner::xyce_initial_timestep_for_tran(&worker.plan.tran),
            );
            assert_eq!(
                base.config().integration_method,
                rspice_core::numerics::integration::IntegrationMethod::Trapezoidal
            );
            let resolved = base.resolved_for_netlist(&worker.netlist);
            let expected = match role {
                Bug306SonRole::GearNumericOwner | Bug306SonRole::GearStringControl => {
                    rspice_core::numerics::integration::IntegrationMethod::Gear2
                }
                Bug306SonRole::TrapNumericOwner
                | Bug306SonRole::TrapStringControl
                | Bug306SonRole::TrapezoidalStringControl => {
                    rspice_core::numerics::integration::IntegrationMethod::Trapezoidal
                }
            };
            assert_eq!(resolved.config().integration_method, expected);
        }
    }

    #[test]
    fn bug306_canonical_five_role_oracle_executes() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, Default::default());
        runner
            .validate_bug306_son_oracle(
                &deck(&root, Bug306SonRole::GearNumericOwner),
                Bug306SonRole::GearNumericOwner,
                Instant::now(),
            )
            .unwrap();
    }

    #[test]
    fn bug306_method_model_and_probe_mutations_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, Default::default());
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let members = runner.read_bug306_directory(&abort).unwrap();
        let role = Bug306SonRole::GearNumericOwner;
        let source = std::str::from_utf8(&members[&role.file_name().to_ascii_lowercase()]).unwrap();
        for (label, changed) in [
            ("METHOD", source.replacen("method=8", "method=7", 1)),
            ("model", source.replacen("bf=130", "bf=131", 1)),
            (
                "COMP probe",
                source.replacen("{i(vib)-ib(q1)}", "{i(vib)-ic(q1)}", 1),
            ),
            (
                "PRINT probe",
                source.replacen(
                    ".print tran  PRECISION=10 WIDTH=19 {i(vib)-ib(q1)}",
                    ".print tran  PRECISION=10 WIDTH=19 {i(vib)-ic(q1)}",
                    1,
                ),
            ),
            (
                "PRINT precision",
                source.replacen("PRECISION=10", "PRECISION=9", 1),
            ),
            ("PRINT width", source.replacen("WIDTH=19", "WIDTH=18", 1)),
        ] {
            let mut mutated = members.clone();
            mutated.insert(role.file_name().to_ascii_lowercase(), changed.into_bytes());
            assert!(
                runner
                    .prepare_bug306_worker(&mutated, role, &abort)
                    .is_err(),
                "{label} mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug306_relation_rejects_shared_wrong_and_accepts_release_frame() {
        const SYNTHETIC_ROWS: usize = 64;
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, Default::default());
        let rows = (0..SYNTHETIC_ROWS)
            .map(|index| {
                vec![
                    index as Value,
                    TRANSIENT_STOP * (index as Value / (SYNTHETIC_ROWS - 1) as Value),
                    0.0,
                    0.0,
                    0.0,
                ]
            })
            .collect::<Vec<_>>();
        let baseline = XycePrnTable {
            columns: vec![
                "Index".into(),
                "TIME".into(),
                "{I(VIB)-IB(Q1)}".into(),
                "{I(VIC)-IC(Q1)}".into(),
                "{I(VIE)-IE(Q1)}".into(),
            ],
            rows,
        };
        let source = fs::read_to_string(root.join(Bug306SonRole::GearNumericOwner.path())).unwrap();
        runner
            .compare_bug306_relation(
                Bug306SonRole::GearNumericOwner,
                &source,
                &baseline,
                Bug306SonRole::GearStringControl,
                &baseline,
            )
            .unwrap();
        let mut shared_wrong = baseline.clone();
        for row in &mut shared_wrong.rows {
            row[2] = 1.000_001e-6;
        }
        assert!(
            runner
                .compare_bug306_relation(
                    Bug306SonRole::GearNumericOwner,
                    &source,
                    &shared_wrong,
                    Bug306SonRole::GearStringControl,
                    &shared_wrong,
                )
                .is_err(),
            "identical METHOD aliases must not pass by sharing a physically invalid KCL waveform"
        );
        let mut boundary = baseline.clone();
        for row in &mut boundary.rows {
            row[2] = 1.0e-6;
        }
        runner
            .compare_bug306_relation(
                Bug306SonRole::GearNumericOwner,
                &source,
                &baseline,
                Bug306SonRole::GearStringControl,
                &boundary,
            )
            .expect("Release xyce_verify's normalized-RMS boundary is inclusive");
        let mut wrong = boundary;
        for row in &mut wrong.rows {
            row[2] = 1.000_001e-6;
        }
        assert!(
            runner
                .compare_bug306_relation(
                    Bug306SonRole::GearNumericOwner,
                    &source,
                    &baseline,
                    Bug306SonRole::GearStringControl,
                    &wrong,
                )
                .is_err()
        );

        let columns = vec!["Index".into(), "TIME".into(), "I(VIB)-IB(Q1)".into()];
        let full_domain = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 0.0, 0.0],
                vec![1.0, TRANSIENT_STOP / 2.0, 0.0],
                vec![2.0, TRANSIENT_STOP, 0.0],
            ],
        };
        let interior_domain = XycePrnTable {
            columns,
            rows: vec![
                vec![0.0, TRANSIENT_STOP / 4.0, 0.0],
                vec![1.0, 3.0 * TRANSIENT_STOP / 4.0, 0.0],
            ],
        };
        let tolerance = XyceTestRunner::validate_bug306_comp_tolerances(&source).unwrap()[0];
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                    &full_domain,
                    &interior_domain,
                    tolerance,
                    PRINT_PRECISION,
                )
                .unwrap()
                .is_empty(),
            "the Release wrapper's numeric owner must be the interpolated GOOD waveform"
        );
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                    &interior_domain,
                    &full_domain,
                    tolerance,
                    PRINT_PRECISION,
                )
                .is_err(),
            "reversing the Release GOOD/TEST direction must not silently pass"
        );
    }

    #[test]
    fn bug306_census_rejects_extra_case_oversize_and_gold() {
        let (_temporary, runner) = fixture("extra");
        fs::write(runner.root.join(FAMILY_DIRECTORY).join("extra.cir"), b"x\n").unwrap();
        assert!(
            runner
                .read_bug306_directory(&rspice_core::abort_signal::NoAbort)
                .is_err()
        );

        let (_temporary, runner) = fixture("case");
        fs::rename(
            runner.root.join(FAMILY_DIRECTORY).join("lead_bjt_gear.cir"),
            runner.root.join(FAMILY_DIRECTORY).join("LEAD_BJT_GEAR.cir"),
        )
        .unwrap();
        assert!(
            runner
                .read_bug306_directory(&rspice_core::abort_signal::NoAbort)
                .is_err()
        );

        let (_temporary, runner) = fixture("oversize");
        fs::write(
            runner.root.join(FAMILY_DIRECTORY).join("lead_bjt_gear.cir"),
            vec![b'x'; 2_000],
        )
        .unwrap();
        assert!(
            runner
                .read_bug306_directory(&rspice_core::abort_signal::NoAbort)
                .is_err()
        );

        let (_temporary, runner) = fixture("gold");
        fs::create_dir_all(
            runner
                .root
                .join("OutputData/Certification_Tests/BUG_306_SON"),
        )
        .unwrap();
        assert!(
            runner
                .validate_bug306_provenance(
                    &deck(&runner.root, Bug306SonRole::GearNumericOwner),
                    Bug306SonRole::GearNumericOwner,
                    &rspice_core::abort_signal::NoAbort,
                )
                .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn bug306_census_rejects_symlink_members() {
        use std::os::unix::fs::symlink;
        let (_temporary, runner) = fixture("symlink");
        let family = runner.root.join(FAMILY_DIRECTORY);
        fs::remove_file(family.join("lead_bjt_gear.cir")).unwrap();
        symlink(
            family.join("lead_bjt_trap.cir"),
            family.join("lead_bjt_gear.cir"),
        )
        .unwrap();
        assert!(
            runner
                .read_bug306_directory(&rspice_core::abort_signal::NoAbort)
                .is_err()
        );
    }

    #[test]
    fn bug306_expired_shared_deadline_fails_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..Default::default()
            },
        );
        assert!(
            runner
                .validate_bug306_son_oracle(
                    &deck(&root, Bug306SonRole::TrapNumericOwner),
                    Bug306SonRole::TrapNumericOwner,
                    Instant::now() - Duration::from_secs(1),
                )
                .is_err()
        );
    }
}
