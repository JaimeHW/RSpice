use super::*;
use rspice_core::netlist::{DiagnosticSeverity, SourceSpec};
use std::io::Read as _;

pub(super) const BUG45_CONTRACT: &str =
    "bug45_unknown_expression_model_parameter_warning_wrapper_owner";

const LABEL: &str = "BUG_45_SON unknown expression-valued diode parameter";
pub(super) const RECORD: &str = "netlists/certification_tests/bug_45_son/diode.cir";
const DECK_PATH: &str = "Netlists/Certification_Tests/BUG_45_SON/diode.cir";
const FAMILY_PATH: &str = "Netlists/Certification_Tests/BUG_45_SON";
const GOLD_PATH: &str = "OutputData/Certification_Tests/BUG_45_SON/diode.cir.prn";
const OUTPUT_FAMILY_PATH: &str = "OutputData/Certification_Tests/BUG_45_SON";
const SOURCE_BYTES: usize = 894;
const SOURCE_SHA256: &str = "0603e04698f28393b96bd78a9d41ab6028c2280c369602965cb5c807945c89e3";
const GOLD_BYTES: usize = 19_605;
const GOLD_SHA256: &str = "abb78e9f089ae8c0417c6233c89bc02cafe65f73b3caa180da76636e520b90a1";
const MAX_SOURCE_BYTES: u64 = 64 * 1024;
const MAX_GOLD_BYTES: u64 = 1024 * 1024;
const MAX_MANIFEST_BYTES: u64 = 1024 * 1024;
const MAX_DIRECTORY_ENTRIES: usize = 16_384;
const WARNING_CODE: &str = "xyce-unknown-diode-model-parameter";
const WARNING_MESSAGE: &str =
    "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter ignored.";
const HISTORICAL_LOCATION_PATTERN: &str = "Netlist warning in file diode.cir at or near line 29";
const HISTORICAL_PARAMETER_PATTERN: &str =
    "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter";
const UPSTREAM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const UPSTREAM_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_FAMILY_TREE: &str = "286b07d36a178073becb433fb444f398177c495d";
const UPSTREAM_OUTPUT_FAMILY_TREE: &str = "330cf6f9fc42228ab7992a5c60666324246bec45";
const UPSTREAM_RELEASE: &str = "Release-7.10";
const HISTORICAL_ARTIFACT_COUNT: usize = 11;
const HISTORICAL_RECORD_COUNT: usize = HISTORICAL_ARTIFACT_COUNT + 1;
// Filled from the sorted immutable record stream below. Keeping the stream's
// aggregate identity independent of each record catches accidental record
// deletion, renaming, or dependency narrowing.
const HISTORICAL_RECORD_BYTES: usize = 2_540;
const HISTORICAL_RECORDS_SHA256: &str =
    "5e0603959b3da4a609f8ce3bdb736bae3890e313d40909835fe5dd8ebafa43aa";

const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); HISTORICAL_ARTIFACT_COUNT] = [
    (
        "Netlists/Certification_Tests/BUG_45_SON/Manifest.txt",
        28,
        "1dab4054f91d32af45d6aa5549a6494d87a7450b",
        "9bccfb4a95c260e0c7e695f4ac4d6a7c7050437f06121b9e9e95a9444935bb4e",
    ),
    (
        DECK_PATH,
        SOURCE_BYTES,
        "3114775c622739592fad63ec4f20ffd35151f51f",
        SOURCE_SHA256,
    ),
    (
        "Netlists/Certification_Tests/BUG_45_SON/diode.cir.sh",
        1_973,
        "935b5d23bd6b2b82e52db1b42bf5c22175ad8ff7",
        "049cc1a79563dceb08e3f602c2da15ff6d4982961d3f0b818604ca3e6fd92aa0",
    ),
    (
        "Netlists/Certification_Tests/BUG_45_SON/tags",
        52,
        "69c3b3e170672d1ff0355ff04cb9301999ad5414",
        "4e62cc650841b0d7b3951b3c225dd187c41c02f77b65e2d147bbbdf2e6f5cda7",
    ),
    (
        GOLD_PATH,
        GOLD_BYTES,
        "4f7931db24b7ce1d10a3ada285ab983b307ebbaa",
        GOLD_SHA256,
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_404,
        "17def57575eb3fd703978fd0634a58d6e679a3de",
        "a86524def2895930f2bb697c058850f231df6d623c279bd7482f30c5c39090b4",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "5809bf44e921762c87b658f096d34f81aca5ccfb",
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
    ),
    (
        "TestScripts/XyceVerify/DCSources.pm",
        2_739,
        "5e2c06cc593fb9e89cefb221f274901b227342eb",
        "b2ddcab5ad5a89c428b9b4430190fa27ef7106da7e7afeb31452c81890a9a006",
    ),
    (
        "TestScripts/XyceVerify/DCSweep.pm",
        9_301,
        "dbd97a554c93829be74ff8a004f7b97f507be591",
        "2246da2374e6cce3ea516a50e472fb07f7481e8b0effb20d4a650e6b6cb1eda0",
    ),
    (
        "TestScripts/XyceVerify/StepSweep.pm",
        8_731,
        "6ba454fc66c19d883c7a8e29c4894eaf364b1f4b",
        "84b2d485c1848f2e456463de8a5015205d87c3db8a6d070547d6f9464618fed6",
    ),
    (
        "TestScripts/valgrind_check.sh",
        786,
        "7a3a431ef05542b3bf32fa4782e6865b769d0929",
        "2dbf1d439d3ef2c607e5d7ccbf413748e2f9155f7013d6d550a43342b23af58d",
    ),
];

#[derive(Debug)]
struct Bug45ProvenanceSeal {
    source: Vec<u8>,
    retained_gold: Vec<u8>,
}

#[derive(Debug)]
struct Bug45Directory {
    path: PathBuf,
    // On Windows these handles deny delete/write sharing for every resolved
    // component. On Unix the terminal handle is used for descriptor-relative
    // enumeration and openat traversal; retaining the chain also makes the
    // anchored provenance explicit.
    guards: Vec<fs::File>,
}

impl XyceTestRunner {
    #[cfg(not(unix))]
    fn bug45_open_regular_nofollow(path: &Path, record_label: &str) -> Result<fs::File, String> {
        let named = fs::symlink_metadata(path)
            .map_err(|error| format!("failed to inspect {LABEL} {record_label}: {error}"))?;
        if named.file_type().is_symlink() || !named.file_type().is_file() {
            return Err(format!(
                "{LABEL} {record_label} must be a regular non-symlink file"
            ));
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt as _;
            use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

            if named.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(format!(
                    "{LABEL} {record_label} must not be a reparse point"
                ));
            }
        }

        let mut options = fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;

            options.custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt as _;
            use windows_sys::Win32::Storage::FileSystem::{
                FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ,
            };

            options
                .share_mode(FILE_SHARE_READ)
                .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
        }
        let file = options
            .open(path)
            .map_err(|error| format!("failed to open {LABEL} {record_label}: {error}"))?;
        let opened = file
            .metadata()
            .map_err(|error| format!("failed to inspect opened {LABEL} {record_label}: {error}"))?;
        if !opened.file_type().is_file() {
            return Err(format!(
                "{LABEL} {record_label} did not open as a regular file"
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;

            if opened.dev() != named.dev() || opened.ino() != named.ino() {
                return Err(format!(
                    "{LABEL} {record_label} changed while it was opened"
                ));
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt as _;
            use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

            if opened.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(format!("{LABEL} {record_label} opened as a reparse point"));
            }
        }
        Ok(file)
    }

    fn bug45_open_directory_nofollow(path: &Path, record_label: &str) -> Result<fs::File, String> {
        let named = fs::symlink_metadata(path)
            .map_err(|error| format!("failed to inspect {LABEL} {record_label}: {error}"))?;
        if named.file_type().is_symlink() || !named.file_type().is_dir() {
            return Err(format!(
                "{LABEL} {record_label} must be a regular non-symlink directory"
            ));
        }
        let mut options = fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;

            options.custom_flags(
                (rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::DIRECTORY).bits() as i32,
            );
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt as _;
            use windows_sys::Win32::Storage::FileSystem::{
                FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ,
            };

            options
                .share_mode(FILE_SHARE_READ)
                .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT);
        }
        let file = options
            .open(path)
            .map_err(|error| format!("failed to open {LABEL} {record_label}: {error}"))?;
        let opened = file
            .metadata()
            .map_err(|error| format!("failed to inspect opened {LABEL} {record_label}: {error}"))?;
        if !opened.file_type().is_dir() {
            return Err(format!(
                "{LABEL} {record_label} did not open as a directory"
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;

            if opened.dev() != named.dev() || opened.ino() != named.ino() {
                return Err(format!(
                    "{LABEL} {record_label} changed while it was opened"
                ));
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt as _;
            use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

            if opened.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(format!("{LABEL} {record_label} opened as a reparse point"));
            }
        }
        Ok(file)
    }

    fn bug45_root_directory(&self) -> Result<Bug45Directory, String> {
        Ok(Bug45Directory {
            path: self.root.clone(),
            guards: vec![Self::bug45_open_directory_nofollow(
                &self.root,
                "corpus root",
            )?],
        })
    }

    fn bug45_directory_member_names(
        directory: &Bug45Directory,
        record_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<String>, String> {
        let mut names = Vec::new();
        #[cfg(unix)]
        {
            let guard = directory
                .guards
                .last()
                .ok_or_else(|| format!("{LABEL} {record_label} lost its directory anchor"))?;
            let entries = rustix::fs::Dir::read_from(guard).map_err(|error| {
                format!("failed to enumerate opened {LABEL} {record_label}: {error}")
            })?;
            for (index, entry) in entries.enumerate() {
                if abort.is_aborted() {
                    return Err(format!(
                        "{LABEL} deadline expired while enumerating {record_label}"
                    ));
                }
                if index >= MAX_DIRECTORY_ENTRIES {
                    return Err(format!(
                        "{LABEL} {record_label} exceeds its census envelope"
                    ));
                }
                let entry = entry.map_err(|error| {
                    format!("failed to inspect opened {LABEL} {record_label}: {error}")
                })?;
                let name = entry.file_name().to_str().map_err(|error| {
                    format!("{LABEL} {record_label} member name is not UTF-8: {error}")
                })?;
                if matches!(name, "." | "..") {
                    continue;
                }
                names.push(name.to_string());
            }
        }
        #[cfg(not(unix))]
        {
            for (index, entry) in fs::read_dir(&directory.path)
                .map_err(|error| format!("failed to enumerate {LABEL} {record_label}: {error}"))?
                .enumerate()
            {
                if abort.is_aborted() {
                    return Err(format!(
                        "{LABEL} deadline expired while enumerating {record_label}"
                    ));
                }
                if index >= MAX_DIRECTORY_ENTRIES {
                    return Err(format!(
                        "{LABEL} {record_label} exceeds its census envelope"
                    ));
                }
                let entry = entry.map_err(|error| {
                    format!("failed to inspect {LABEL} {record_label}: {error}")
                })?;
                names.push(
                    entry
                        .file_name()
                        .to_str()
                        .ok_or_else(|| format!("{LABEL} {record_label} member name is not UTF-8"))?
                        .to_string(),
                );
            }
        }
        Ok(names)
    }

    fn bug45_exact_member_name(
        directory: &Bug45Directory,
        expected: &str,
        record_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<String, String> {
        let matches = Self::bug45_directory_member_names(directory, record_label, abort)?
            .into_iter()
            .filter(|name| name.eq_ignore_ascii_case(expected))
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [name] if name == expected => Ok(name.clone()),
            [] => Err(format!("{LABEL} is missing path component {expected:?}")),
            _ => Err(format!(
                "{LABEL} path component {expected:?} changed case or became ambiguous: {matches:?}"
            )),
        }
    }

    fn bug45_open_directory_member_nofollow(
        directory: &Bug45Directory,
        name: &str,
        record_label: &str,
    ) -> Result<fs::File, String> {
        #[cfg(unix)]
        {
            let guard = directory
                .guards
                .last()
                .ok_or_else(|| format!("{LABEL} {record_label} lost its directory anchor"))?;
            let fd = rustix::fs::openat(
                guard,
                name,
                rustix::fs::OFlags::RDONLY
                    | rustix::fs::OFlags::CLOEXEC
                    | rustix::fs::OFlags::NOFOLLOW
                    | rustix::fs::OFlags::DIRECTORY,
                rustix::fs::Mode::empty(),
            )
            .map_err(|error| format!("failed to open anchored {LABEL} {record_label}: {error}"))?;
            let file: fs::File = fd.into();
            if !file
                .metadata()
                .map_err(|error| {
                    format!("failed to inspect opened {LABEL} {record_label}: {error}")
                })?
                .is_dir()
            {
                return Err(format!(
                    "{LABEL} {record_label} did not open as a directory"
                ));
            }
            return Ok(file);
        }
        #[cfg(not(unix))]
        Self::bug45_open_directory_nofollow(&directory.path.join(name), record_label)
    }

    fn bug45_open_file_member_nofollow(
        directory: &Bug45Directory,
        name: &str,
        record_label: &str,
    ) -> Result<fs::File, String> {
        #[cfg(unix)]
        {
            let guard = directory
                .guards
                .last()
                .ok_or_else(|| format!("{LABEL} {record_label} lost its directory anchor"))?;
            let fd = rustix::fs::openat(
                guard,
                name,
                rustix::fs::OFlags::RDONLY
                    | rustix::fs::OFlags::CLOEXEC
                    | rustix::fs::OFlags::NONBLOCK
                    | rustix::fs::OFlags::NOFOLLOW,
                rustix::fs::Mode::empty(),
            )
            .map_err(|error| format!("failed to open anchored {LABEL} {record_label}: {error}"))?;
            let file: fs::File = fd.into();
            if !file
                .metadata()
                .map_err(|error| {
                    format!("failed to inspect opened {LABEL} {record_label}: {error}")
                })?
                .is_file()
            {
                return Err(format!(
                    "{LABEL} {record_label} did not open as a regular file"
                ));
            }
            return Ok(file);
        }
        #[cfg(not(unix))]
        Self::bug45_open_regular_nofollow(&directory.path.join(name), record_label)
    }

    fn bug45_exact_child_directory(
        mut parent: Bug45Directory,
        expected: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Bug45Directory, String> {
        let name = Self::bug45_exact_member_name(&parent, expected, "parent directory", abort)?;
        let guard = Self::bug45_open_directory_member_nofollow(&parent, &name, expected)?;
        parent.path.push(&name);
        parent.guards.push(guard);
        Ok(parent)
    }

    fn bug45_exact_child_file(
        parent: &Bug45Directory,
        expected: &str,
        record_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<fs::File, String> {
        let name = Self::bug45_exact_member_name(parent, expected, "parent directory", abort)?;
        Self::bug45_open_file_member_nofollow(parent, &name, record_label)
    }

    fn bug45_optional_exact_child_file(
        parent: &Bug45Directory,
        expected: &str,
        record_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Option<fs::File>, String> {
        let matches = Self::bug45_directory_member_names(parent, "corpus root", abort)?
            .into_iter()
            .filter(|name| name.eq_ignore_ascii_case(expected))
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [name] if name == expected => {
                Self::bug45_open_file_member_nofollow(parent, name, record_label).map(Some)
            }
            [] => Ok(None),
            _ => Err(format!(
                "{LABEL} optional path component {expected:?} changed case or became ambiguous: {matches:?}"
            )),
        }
    }

    fn bug45_historical_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, blob, sha256)| {
                format!("{UPSTREAM_COMMIT}\t{UPSTREAM_RELEASE}\t{path}\t{bytes}\t{blob}\t{sha256}")
            })
            .collect::<Vec<_>>();
        records.push(format!(
            "{UPSTREAM_COMMIT}\t{UPSTREAM_RELEASE}\tTREE\tNetlists\t{UPSTREAM_NETLISTS_TREE}\t{FAMILY_PATH}\t{UPSTREAM_FAMILY_TREE}\t{OUTPUT_FAMILY_PATH}\t{UPSTREAM_OUTPUT_FAMILY_TREE}"
        ));
        records.sort();
        records
    }

    fn validate_bug45_historical_provenance() -> Result<(), String> {
        let records = Self::bug45_historical_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        if UPSTREAM_EXCLUSIONS_SOURCE_COMMIT != UPSTREAM_COMMIT
            || UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE != UPSTREAM_NETLISTS_TREE
            || records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
        {
            return Err(format!(
                "{LABEL} Release-7.10 historical stream changed: commit={UPSTREAM_COMMIT}, netlists_tree={UPSTREAM_NETLISTS_TREE}, family_tree={UPSTREAM_FAMILY_TREE}, records={}/{}, sha256={sha256}",
                records.len(),
                stream.len()
            ));
        }
        if HISTORICAL_LOCATION_PATTERN != "Netlist warning in file diode.cir at or near line 29"
            || HISTORICAL_PARAMETER_PATTERN
                != "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter"
        {
            return Err(format!(
                "{LABEL} historical ordered warning predicates changed"
            ));
        }
        Ok(())
    }

    fn bug45_read_bounded_opened_raw(
        file: fs::File,
        max_bytes: u64,
        record_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<u8>, String> {
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired before reading {record_label}"
            ));
        }
        let metadata = file
            .metadata()
            .map_err(|error| format!("failed to inspect opened {LABEL} {record_label}: {error}"))?;
        if metadata.len() > max_bytes {
            return Err(format!(
                "{LABEL} {record_label} exceeds its {max_bytes}-byte read bound"
            ));
        }
        let mut bytes = Vec::with_capacity((metadata.len() as usize).min(max_bytes as usize));
        file.take(max_bytes + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("failed to read {LABEL} {record_label}: {error}"))?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired while reading {record_label}"
            ));
        }
        if bytes.len() as u64 > max_bytes {
            return Err(format!(
                "{LABEL} {record_label} exceeds its {max_bytes}-byte read bound"
            ));
        }
        Ok(bytes)
    }

    fn bug45_read_bounded_opened_canonical(
        file: fs::File,
        max_bytes: u64,
        record_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<u8>, String> {
        let bytes = Self::bug45_read_bounded_opened_raw(file, max_bytes, record_label, abort)?;
        Self::canonical_lf_text_identity(LABEL, &bytes)
    }

    fn bug45_read_opened_canonical_record(
        file: fs::File,
        max_bytes: u64,
        expected_bytes: usize,
        expected_sha256: &str,
        record_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<u8>, String> {
        let canonical =
            Self::bug45_read_bounded_opened_canonical(file, max_bytes, record_label, abort)?;
        let sha256 = format!("{:x}", Sha256::digest(&canonical));
        if canonical.len() != expected_bytes || sha256 != expected_sha256 {
            return Err(format!(
                "{LABEL} {record_label} identity changed: expected {expected_bytes}/{expected_sha256}, got {}/{sha256}",
                canonical.len()
            ));
        }
        Ok(canonical)
    }

    fn bug45_family_directories(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<(Bug45Directory, Bug45Directory), String> {
        let netlists =
            Self::bug45_exact_child_directory(self.bug45_root_directory()?, "Netlists", abort)?;
        let netlist_certification =
            Self::bug45_exact_child_directory(netlists, "Certification_Tests", abort)?;
        let netlist_family =
            Self::bug45_exact_child_directory(netlist_certification, "BUG_45_SON", abort)?;
        let output =
            Self::bug45_exact_child_directory(self.bug45_root_directory()?, "OutputData", abort)?;
        let output_certification =
            Self::bug45_exact_child_directory(output, "Certification_Tests", abort)?;
        let output_family =
            Self::bug45_exact_child_directory(output_certification, "BUG_45_SON", abort)?;
        Ok((netlist_family, output_family))
    }

    fn validate_bug45_live_harness_row(&self, abort: &dyn AbortSignal) -> Result<(), String> {
        let root = self.bug45_root_directory()?;
        let file =
            Self::bug45_exact_child_file(&root, HARNESS_MANIFEST_FILE, "harness manifest", abort)?;
        let bytes = Self::bug45_read_bounded_opened_canonical(
            file,
            MAX_MANIFEST_BYTES,
            "harness manifest",
            abort,
        )?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|error| format!("{LABEL} harness manifest is not UTF-8: {error}"))?;
        let family_prefix = "netlists/certification_tests/bug_45_son/";
        let expected = format!("{DECK_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}");
        let mut rows = Vec::new();
        for line in text.lines() {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} deadline expired while validating the harness manifest"
                ));
            }
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let normalized = Self::normalize_manifest_key(trimmed);
            let record_is_family = line.split_once('\t').is_some_and(|(record, _)| {
                Self::normalize_manifest_key(record).starts_with(family_prefix)
            });
            if record_is_family || normalized.contains(family_prefix) {
                if line != expected {
                    return Err(format!(
                        "{LABEL} live harness family row is not byte-exact: {line:?}"
                    ));
                }
                rows.push(line);
            }
        }
        if rows != [expected.as_str()] {
            return Err(format!(
                "{LABEL} requires one exact live harness row {expected:?}, found {rows:?}"
            ));
        }
        Ok(())
    }

    fn bug45_validate_single_member_directory(
        directory: &Bug45Directory,
        expected_name: &str,
        max_bytes: u64,
        expected_bytes: usize,
        expected_sha256: &str,
        record_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<u8>, String> {
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired before enumerating {record_label}"
            ));
        }
        let mut selected_name = None;
        let mut observed = BTreeSet::new();
        for name in Self::bug45_directory_member_names(directory, record_label, abort)? {
            let key = name.to_ascii_lowercase();
            if !observed.insert(key) {
                return Err(format!(
                    "{LABEL} {record_label} contains a case-colliding member {name:?}"
                ));
            }
            if name != expected_name {
                return Err(format!(
                    "{LABEL} {record_label} acquired unexpected member {name:?}"
                ));
            }
            selected_name = Some(name);
        }
        if observed.len() != 1 {
            return Err(format!(
                "{LABEL} {record_label} census changed: expected one member, found {}",
                observed.len()
            ));
        }
        let name =
            selected_name.ok_or_else(|| format!("{LABEL} {record_label} lost {expected_name}"))?;
        let file = Self::bug45_open_file_member_nofollow(directory, &name, record_label)?;
        Self::bug45_read_opened_canonical_record(
            file,
            max_bytes,
            expected_bytes,
            expected_sha256,
            record_label,
            abort,
        )
    }

    fn validate_bug45_provenance(
        &self,
        deck: &XyceDeck,
        abort: &dyn AbortSignal,
    ) -> Result<Bug45ProvenanceSeal, String> {
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired before provenance validation"
            ));
        }
        Self::validate_bug45_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != RECORD
            || !Self::same_path(&deck.path, &self.root.join(DECK_PATH))
        {
            return Err(format!(
                "recognized {LABEL} record is not backed by its exact canonical Netlists path"
            ));
        }
        self.validate_bug45_live_harness_row(abort)?;

        let family_prefix = "netlists/certification_tests/bug_45_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners.len() != 1
            || owners[0].as_str() != RECORD
            || !self.requires_upstream_wrapper(RECORD)
        {
            return Err(format!(
                "{LABEL} requires diode.cir as its exact sole removed-wrapper owner, found {owners:?}"
            ));
        }
        let root = self.bug45_root_directory()?;
        let exclusions_file = Self::bug45_exact_child_file(
            &root,
            UPSTREAM_EXCLUSIONS_MANIFEST_FILE,
            "upstream exclusions manifest",
            abort,
        )?;
        let exclusions_bytes = Self::bug45_read_bounded_opened_raw(
            exclusions_file,
            MAX_MANIFEST_BYTES,
            "upstream exclusions manifest",
            abort,
        )?;
        let is_vendored_corpus = Self::bug45_optional_exact_child_file(
            &root,
            "RSPICE-VENDORING.md",
            "vendoring provenance",
            abort,
        )?
        .is_some();
        let exclusions_path = self.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let exclusions = Self::parse_upstream_exclusions_bytes(
            &self.root,
            &exclusions_path,
            is_vendored_corpus,
            &exclusions_bytes,
        )
        .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired while validating exclusions"
            ));
        }
        let family_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if !family_exclusions.is_empty() {
            return Err(format!(
                "{LABEL} must not acquire an upstream-exclusion disposition: {family_exclusions:?}"
            ));
        }

        let (netlist_family, output_family) = self.bug45_family_directories(abort)?;
        let source = Self::bug45_validate_single_member_directory(
            &netlist_family,
            "diode.cir",
            MAX_SOURCE_BYTES,
            SOURCE_BYTES,
            SOURCE_SHA256,
            "retained Netlists family",
            abort,
        )?;
        let retained_gold = Self::bug45_validate_single_member_directory(
            &output_family,
            "diode.cir.prn",
            MAX_GOLD_BYTES,
            GOLD_BYTES,
            GOLD_SHA256,
            "retained OutputData family",
            abort,
        )?;
        Ok(Bug45ProvenanceSeal {
            source,
            retained_gold,
        })
    }

    fn bug45_nodes_match(element: &rspice_core::netlist::Element, expected: &[&str]) -> bool {
        element.provenance == ElementProvenance::Authored
            && element
                .nodes
                .iter()
                .map(String::as_str)
                .eq(expected.iter().copied())
    }

    fn bug45_validate_resistor(
        elements: &BTreeMap<String, &rspice_core::netlist::Element>,
        name: &str,
        nodes: &[&str],
        expected_value: Value,
    ) -> Result<(), String> {
        let element = elements
            .get(&name.to_ascii_uppercase())
            .ok_or_else(|| format!("{LABEL} lost {name}"))?;
        if !Self::bug45_nodes_match(element, nodes)
            || !matches!(&element.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == expected_value.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!(
                "{LABEL} {name} topology/value changed: {element:?}"
            ));
        }
        Ok(())
    }

    fn bug45_validate_warning(netlist: &Netlist, deck_path: &Path) -> Result<(), String> {
        if netlist.diagnostics.len() != 1 {
            return Err(format!(
                "{LABEL} requires exactly one ignored-parameter warning, found {:?}",
                netlist.diagnostics
            ));
        }
        let diagnostic = &netlist.diagnostics[0];
        let rendered = diagnostic.xyce_legacy_warning_lines();
        if diagnostic.severity != DiagnosticSeverity::Warning
            || diagnostic.code != WARNING_CODE
            || diagnostic.line != 29
            || diagnostic.origin.as_ref().map(|origin| origin.line) != Some(29)
            || diagnostic
                .origin
                .as_ref()
                .and_then(|origin| origin.path.as_deref())
                != Some(deck_path)
            || diagnostic.message != WARNING_MESSAGE
            || rendered
                != Some([
                    HISTORICAL_LOCATION_PATTERN.to_string(),
                    WARNING_MESSAGE.to_string(),
                ])
        {
            return Err(format!(
                "{LABEL} structured warning no longer reproduces the two ordered historical predicates {HISTORICAL_LOCATION_PATTERN:?} / {HISTORICAL_PARAMETER_PATTERN:?}: {diagnostic:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug45_typed_contract(
        &self,
        deck: &XyceDeck,
        source: &str,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_deck_with_sealed_source_and_purpose(
            deck,
            source,
            XyceStaticTranPlanPurpose::AbsoluteOracle,
        )?;
        let expected_gold = self.root.join(GOLD_PATH);
        let probes = plan
            .require_print(LABEL)?
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.deck_path != deck.path
            || !matches!(&plan.oracle, XyceStaticTranOracle::Waveform(path) if Self::same_path(path, &expected_gold))
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.wrapper_tolerance.is_some()
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.tran.step.to_bits() != 2.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 2.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || probes != ["v(3)", "v(2)", "v(4)"]
        {
            return Err(format!(
                "{LABEL} exact TRAN/PRINT/oracle plan changed: {plan:?}"
            ));
        }
        plan.validate_executable_oracle_shape()?;

        let netlist = Self::parse_xyce_netlist(source, &deck.path)
            .map_err(|error| format!("{LABEL} source no longer parses: {error}"))?;
        if netlist.elements.len() != 9
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.fft_analyses.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} typed envelope changed: elements={}, models={}, analyses={}, outputs={}",
                netlist.elements.len(),
                netlist.models.len(),
                netlist.analyses.len(),
                netlist.output_requests.len()
            ));
        }
        Self::bug45_validate_warning(&netlist, &deck.path)?;

        let model = &netlist.models[0];
        let mut model_params = model
            .params
            .iter()
            .map(|(name, value)| (name.to_ascii_uppercase(), value.to_bits()))
            .collect::<Vec<_>>();
        model_params.sort();
        let expected_model_values: [(&str, Value); 15] = [
            ("AF", 1.0),
            ("BOGOPARAM", 3.0),
            ("BV", 600.0),
            ("CJO", 1.95e-11),
            ("EG", 1.36),
            ("FC", 0.9),
            ("IBV", 1.0e-4),
            ("IS", 4.0e-10),
            ("KF", 0.0),
            ("M", 0.38),
            ("N", 1.48),
            ("RS", 0.105),
            ("TT", 8.0e-7),
            ("VJ", 0.4),
            ("XTI", -8.0),
        ];
        let mut expected_model_params = expected_model_values
            .into_iter()
            .map(|(name, value)| (name.to_string(), value.to_bits()))
            .collect::<Vec<_>>();
        expected_model_params.sort();
        if model.name != "D1N3940"
            || model.model_type != "D"
            || model_params != expected_model_params
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} exact D1N3940 model changed: {model:?}"));
        }

        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_uppercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.len() != 9 {
            return Err(format!("{LABEL} element-name census changed"));
        }
        let vcc = elements
            .get("VCC")
            .ok_or_else(|| format!("{LABEL} lost VCC"))?;
        if !Self::bug45_nodes_match(vcc, &["1", "0"])
            || !matches!(&vcc.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 5.0f64.to_bits())
        {
            return Err(format!("{LABEL} VCC changed: {vcc:?}"));
        }
        let vin = elements
            .get("VIN")
            .ok_or_else(|| format!("{LABEL} lost VIN"))?;
        if !Self::bug45_nodes_match(vin, &["3", "0"])
            || !matches!(&vin.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            }) if offset.to_bits() == 0.0f64.to_bits()
                && amplitude.to_bits() == 10.0f64.to_bits()
                && frequency.to_bits() == 1_000.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!("{LABEL} VIN changed: {vin:?}"));
        }
        for (name, nodes) in [("D1", ["2", "1"]), ("D2", ["0", "2"])] {
            let diode = elements
                .get(name)
                .ok_or_else(|| format!("{LABEL} lost {name}"))?;
            if !Self::bug45_nodes_match(diode, &nodes)
                || !matches!(&diode.kind, ElementKind::Diode {
                    model,
                    instance_params,
                    deferred_params,
                } if model == "D1N3940"
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
            {
                return Err(format!("{LABEL} {name} changed: {diode:?}"));
            }
        }
        Self::bug45_validate_resistor(&elements, "R1", &["2", "3"], 1_000.0)?;
        Self::bug45_validate_resistor(&elements, "R2", &["1", "2"], 3_300.0)?;
        Self::bug45_validate_resistor(&elements, "R3", &["2", "0"], 3_300.0)?;
        Self::bug45_validate_resistor(&elements, "R4", &["4", "0"], 5_600.0)?;
        let capacitor = elements
            .get("C1")
            .ok_or_else(|| format!("{LABEL} lost C1"))?;
        if !Self::bug45_nodes_match(capacitor, &["2", "4"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 4.699_999_999_999_999_5e-7f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} C1 changed: {capacitor:?}"));
        }
        Ok(plan)
    }

    fn bug45_compare_actual_good_to_retained_test(
        &self,
        actual: &XycePrnTable,
        retained: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let mut mismatches = self.compare_xyce_verify_transient_tables(actual, retained)?;
        // The historical API names its first operand GOOD rather than
        // "expected". Preserve the public RSpice mismatch convention after
        // applying the directional metric: retained gold is expected and the
        // newly simulated waveform is actual.
        for mismatch in &mut mismatches {
            std::mem::swap(&mut mismatch.expected, &mut mismatch.actual);
        }
        Ok(mismatches)
    }

    fn bug45_validate_output_table(table: &XycePrnTable, retained: bool) -> Result<(), String> {
        if table.columns != ["Index", "TIME", "V(3)", "V(2)", "V(4)"]
            || table.rows.len() < 2
            || retained && table.rows.len() != 250
        {
            return Err(format!(
                "{LABEL} {} output framing changed: columns={:?}, rows={}",
                if retained { "retained" } else { "fresh" },
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 5
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || previous.is_some_and(|time| row[1] <= time)
            {
                return Err(format!(
                    "{LABEL} {} row {index} is malformed: {row:?}",
                    if retained { "retained" } else { "fresh" }
                ));
            }
            previous = Some(row[1]);
        }
        let first = table.rows[0][1];
        let last = table.rows.last().expect("nonempty table")[1];
        if first.to_bits() != 0.0f64.to_bits() || last.to_bits() != 2.0e-3f64.to_bits() {
            return Err(format!(
                "{LABEL} {} output domain changed: {first}..{last}",
                if retained { "retained" } else { "fresh" }
            ));
        }
        Ok(())
    }

    pub(super) fn run_bug45_contract(&self, deck: &XyceDeck, start: Instant) -> XyceTestResult {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let seal = match self.validate_bug45_provenance(deck, &abort) {
            Ok(seal) => seal,
            Err(error) => {
                return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
            }
        };
        let source = match String::from_utf8(seal.source) {
            Ok(source) => source,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} source is not UTF-8: {error}"),
                    Vec::new(),
                );
            }
        };
        let plan = match self.validate_bug45_typed_contract(deck, &source) {
            Ok(plan) => plan,
            Err(error) => {
                return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
            }
        };
        if let Err(error) = plan.require_waveform_reference_path(LABEL) {
            return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
        }
        let retained_gold = match String::from_utf8(seal.retained_gold) {
            Ok(retained_gold) => retained_gold,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} retained PRN is not UTF-8: {error}"),
                    Vec::new(),
                );
            }
        };
        let retained = match Self::parse_xyce_verify_tran_reference_table(&retained_gold) {
            Ok(table) => table,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} retained PRN is invalid: {error}"),
                    Vec::new(),
                );
            }
        };
        if let Err(error) = Self::bug45_validate_output_table(&retained, true) {
            return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
        }

        let (netlist, result) = match self.run_transient_family_plan(&plan, start, None, None) {
            Ok(run) => run,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!(
                        "{LABEL} simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} simulation failed instead of exiting normally: {error}"),
                    Vec::new(),
                );
            }
        };
        if let Err(error) = Self::bug45_validate_warning(&netlist, &deck.path) {
            return self.failure_result(
                deck,
                start,
                BUG45_CONTRACT,
                format!("{LABEL} executed netlist warning contract changed: {error}"),
                Vec::new(),
            );
        }
        let actual = match Self::transient_family_result_to_prn_table(&plan, &netlist, &result) {
            Ok(table) => table,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} fresh PRN serialization failed: {error}"),
                    Vec::new(),
                );
            }
        };
        if let Err(error) = Self::bug45_validate_output_table(&actual, false) {
            return self.failure_result(deck, start, BUG45_CONTRACT, error, Vec::new());
        }
        let mismatches = match self.bug45_compare_actual_good_to_retained_test(&actual, &retained) {
            Ok(mismatches) => mismatches,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    BUG45_CONTRACT,
                    format!("{LABEL} directional xyce_verify comparison failed: {error}"),
                    Vec::new(),
                );
            }
        };
        if !mismatches.is_empty() {
            return self.failure_result(
                deck,
                start,
                BUG45_CONTRACT,
                format!(
                    "{} {LABEL} actual-GOOD/retained-TEST integrated-RMS mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            );
        }
        if let Err(error) = self.validate_bug45_provenance(deck, &abort) {
            return self.failure_result(
                deck,
                start,
                BUG45_CONTRACT,
                format!("{LABEL} post-execution provenance changed: {error}"),
                Vec::new(),
            );
        }
        self.passed_or_tran_side_output_failure(
            deck,
            start,
            BUG45_CONTRACT,
            &plan,
            &netlist,
            &result,
        )
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

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(DECK_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: DECK_PATH.to_string(),
        }
    }

    fn bug45_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug45-{label}-"))
            .tempdir()
            .expect("create BUG45 fixture root");
        let root = temporary.path();
        fs::create_dir_all(root.join(FAMILY_PATH)).expect("create BUG45 family");
        fs::create_dir_all(root.join(OUTPUT_FAMILY_PATH)).expect("create BUG45 output family");
        fs::copy(source_root.join(DECK_PATH), root.join(DECK_PATH)).expect("copy BUG45 source");
        fs::copy(source_root.join(GOLD_PATH), root.join(GOLD_PATH)).expect("copy BUG45 gold");
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{DECK_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG45 harness manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG45 exclusion manifest");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug45_historical_and_retained_provenance_is_exact() {
        XyceTestRunner::validate_bug45_historical_provenance()
            .expect("BUG45 Release-7.10 historical stream remains exact");
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug45_provenance(&canonical_deck(&root), &rspice_core::abort_signal::NoAbort)
            .expect("BUG45 retained provenance remains exact");
    }

    #[test]
    fn bug45_typed_warning_model_topology_and_plan_are_exact() {
        let root = corpus_root();
        let deck = canonical_deck(&root);
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let source = fs::read_to_string(&deck.path).expect("read BUG45 source");
        runner
            .validate_bug45_typed_contract(&deck, &source)
            .expect("canonical BUG45 typed contract passes");
        for mutation in [
            source.replace("BOGOPARAM", "OTHERPARAM"),
            source.replace("{1+2}", "{1+3}"),
            source.replace("BOGOPARAM", "TLEV"),
            source.replace(".TRAN 2ns 2ms", ".TRAN 3ns 2ms"),
            source.replace("V(3) V(2) V(4)", "V(3) V(4) V(2)"),
            source.replace("D2 0 2 D1N3940", "D2 2 0 D1N3940"),
        ] {
            assert!(
                runner
                    .validate_bug45_typed_contract(&deck, &mutation)
                    .is_err(),
                "BUG45 semantic counterfactual must fail closed"
            );
        }
    }

    #[test]
    fn bug45_provenance_rejects_family_gold_and_manifest_drift() {
        let (_temporary, deck, runner) = bug45_fixture("family-drift");
        runner
            .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
            .expect("canonical BUG45 fixture passes");
        fs::write(
            deck.path.parent().expect("family path").join("extra"),
            "drift\n",
        )
        .expect("write family drift");
        assert!(
            runner
                .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
                .is_err()
        );

        let (_temporary, deck, runner) = bug45_fixture("gold-drift");
        fs::write(runner.root.join(GOLD_PATH), "changed\n").expect("mutate gold");
        assert!(
            runner
                .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
                .is_err()
        );

        let (_temporary, deck, runner) = bug45_fixture("manifest-drift");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("mutate manifest");
        assert!(
            runner
                .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
                .is_err(),
            "the live manifest must be revalidated on the same runner"
        );

        for (label, row) in [
            (
                "manifest-whitespace",
                format!(" {DECK_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
            ),
            (
                "manifest-malformed-family-row",
                format!("{DECK_PATH} {REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
            ),
        ] {
            let (_temporary, deck, runner) = bug45_fixture(label);
            fs::write(runner.root.join(HARNESS_MANIFEST_FILE), row)
                .expect("mutate BUG45 family row");
            assert!(
                runner
                    .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
                    .is_err(),
                "{label} must fail closed"
            );
        }

        let (_temporary, deck, runner) = bug45_fixture("path-case-drift");
        fs::rename(runner.root.join("Netlists"), runner.root.join("netlists"))
            .expect("change canonical Netlists component case");
        assert!(
            runner
                .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
                .is_err(),
            "case-drifted intermediate components must fail closed"
        );

        let (_temporary, deck, runner) = bug45_fixture("missing-exclusions-ledger");
        fs::remove_file(runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE))
            .expect("remove fixture exclusions ledger");
        assert!(
            runner
                .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
                .is_err(),
            "a missing exclusions ledger must fail closed"
        );

        let (_temporary, deck, runner) = bug45_fixture("exclusions-ledger-case-drift");
        fs::rename(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            runner.root.join("rspice-upstream-exclusions.tsv"),
        )
        .expect("change fixture exclusions ledger case");
        assert!(
            runner
                .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
                .is_err(),
            "a case-drifted exclusions ledger must fail closed"
        );

        let (_temporary, deck, runner) = bug45_fixture("exclusions-ledger-crlf");
        let exclusions_path = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let crlf = fs::read_to_string(&exclusions_path)
            .expect("read fixture exclusions ledger")
            .replace('\n', "\r\n");
        fs::write(&exclusions_path, crlf).expect("write CRLF exclusions ledger");
        let error = runner
            .validate_bug45_provenance(&deck, &rspice_core::abort_signal::NoAbort)
            .expect_err("a CRLF exclusions ledger must fail closed");
        assert!(
            error.contains("canonical LF text"),
            "unexpected CRLF rejection: {error}"
        );
    }

    #[test]
    fn bug45_provenance_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let expired_start = Instant::now()
            .checked_sub(Duration::from_secs(1))
            .expect("construct expired BUG45 deadline");
        let abort = DeadlineAbort::new(expired_start, 1);
        let error = runner
            .validate_bug45_provenance(&canonical_deck(&root), &abort)
            .expect_err("an expired BUG45 deadline must fail closed");
        assert!(error.contains("deadline expired"), "{error}");
    }

    #[test]
    fn bug45_public_route_rejects_an_injected_cached_exclusion() {
        let (_temporary, deck, runner) = bug45_fixture("injected-exclusion");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{DECK_PATH}\t{FAMILY_PATH}/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("inject BUG45 exclusion");
        let changed = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        let result = changed.run_test(&deck.path);
        assert!(
            !result.passed,
            "injected BUG45 exclusion must fail: {result:?}"
        );
        assert!(!result.upstream_excluded);
        assert_eq!(result.contract, BUG45_CONTRACT);
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|error| error.contains("must not acquire an upstream-exclusion")),
            "{result:?}"
        );
    }

    #[test]
    fn bug45_comparator_uses_fresh_actual_as_good() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let table = |times: &[Value], values: [Value; 3]| XycePrnTable {
            columns: vec![
                "Index".into(),
                "TIME".into(),
                "V(3)".into(),
                "V(2)".into(),
                "V(4)".into(),
            ],
            rows: times
                .iter()
                .enumerate()
                .map(|(index, time)| vec![index as Value, *time, values[0], values[1], values[2]])
                .collect(),
        };
        let actual_good = table(&[0.0, 1.0, 2.0], [1.0, 2.0, 3.0]);
        let retained_test = table(&[0.0, 1.0], [1.0, 2.0, 3.0]);
        assert!(
            runner
                .bug45_compare_actual_good_to_retained_test(&actual_good, &retained_test)
                .expect("actual-GOOD comparison is valid")
                .is_empty()
        );
        assert!(
            runner
                .compare_xyce_verify_transient_tables(&retained_test, &actual_good)
                .is_err(),
            "reversing GOOD/TEST must fail the directional domain contract"
        );

        let actual_good = table(&[0.0, 1.0, 2.0], [10.0, 20.0, 30.0]);
        let retained_test = table(&[0.0, 1.0], [0.5, 1.0, 1.5]);
        let mismatches = runner
            .bug45_compare_actual_good_to_retained_test(&actual_good, &retained_test)
            .expect("asymmetric actual-GOOD comparison is valid");
        assert_eq!(mismatches.len(), 3);
        assert_eq!(mismatches[0].expected.to_bits(), 0.5f64.to_bits());
        assert_eq!(mismatches[0].actual.to_bits(), 10.0f64.to_bits());
    }
}
