//! A results store blessed as a golden. A store is the directory of segments a run writes, not
//! one result file, so it is converted to the golden's own format (a chunk at a time where the
//! converter can) in a private staging file, and those converted bytes are what the bless
//! validates, compares and publishes.

use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::cli::{CliError, Config, ConvertArgs, InputFormat, OutputFormat};
use crate::commands::waveform_io::{ResultSnapshot, detect_format};

const GOLDEN_FORMATS: &str = ".raw, .csv, .tsv, .json, .h5 or .vcd";

/// The refusal for a golden that names a results store: a golden is one result file.
pub(super) fn store_golden_refusal(golden: &Path) -> CliError {
    CliError::InvalidArgument {
        message: format!(
            "--bless cannot write the golden '{}': a results store (.rsr) is the directory a run \
             writes, not a golden file",
            golden.display()
        ),
        suggestion: Some(format!(
            "name the golden for the format to keep: {GOLDEN_FORMATS}"
        )),
    }
}

/// The store at `result` converted to `golden`'s format and read back as the snapshot a bless
/// validates and publishes: an existing golden's own format, otherwise the one its extension
/// names.
pub(super) fn snapshot(
    result: &Path,
    golden: &Path,
    config: &Config,
) -> Result<ResultSnapshot, CliError> {
    let limits = config.resources.limits();
    let to = golden_format(golden, limits.max_external_data_bytes)?;
    let staging = Staging::create()?;
    // The staged file carries the golden's name, so it reads back as the golden's format.
    let staged = staging
        .0
        .join(golden.file_name().unwrap_or("golden".as_ref()));
    crate::commands::convert::execute(
        ConvertArgs {
            section: None,
            input: result.to_path_buf(),
            output: staged.clone(),
            from: Some(InputFormat::Rsr),
            to,
            variables: Vec::new(),
            start: None,
            stop: None,
            expand_buses: false,
        },
        config,
        true,
    )?;
    // The converted bytes are the golden's, so a limit they exceed is the golden's.
    ResultSnapshot::read(&staged, limits).map_err(|error| match error {
        CliError::ResourceLimit { source, .. } => CliError::ResourceLimit {
            path: golden.to_path_buf(),
            source,
        },
        error => error,
    })
}

fn golden_format(golden: &Path, limit: usize) -> Result<OutputFormat, CliError> {
    Ok(match detect_format(golden) {
        InputFormat::Raw | InputFormat::RawAscii => {
            if raw_values_are_text(golden, limit)? {
                OutputFormat::RawAscii
            } else {
                OutputFormat::Raw
            }
        }
        InputFormat::Csv => OutputFormat::Csv,
        InputFormat::Tsv => OutputFormat::Tsv,
        InputFormat::Json => OutputFormat::Json,
        InputFormat::Hdf5 => OutputFormat::Hdf5,
        InputFormat::Vcd => OutputFormat::Vcd,
        InputFormat::Touchstone => {
            return Err(CliError::InvalidArgument {
                message: format!(
                    "--bless cannot write a transient's results store as the Touchstone golden \
                     '{}'",
                    golden.display()
                ),
                suggestion: Some(format!("bless the store as a {GOLDEN_FORMATS} golden")),
            });
        }
        InputFormat::Rsr => return Err(store_golden_refusal(golden)),
    })
}

/// Whether the existing rawfile at `path` holds its values as text (`Values:`) rather than as
/// binary (`Binary:`). The header is read a line at a time up to its marker, skipping a UTF-16
/// header's NULs. A path that is not a regular file, or a header with neither marker, reads as
/// binary; the golden's own reader then reports what is wrong with it.
fn raw_values_are_text(path: &Path, limit: usize) -> Result<bool, CliError> {
    if !std::fs::metadata(path).is_ok_and(|metadata| metadata.is_file()) {
        return Ok(false);
    }
    let read_error = |source| CliError::InputReadError {
        path: path.to_path_buf(),
        source,
    };
    let file = std::fs::File::open(path).map_err(read_error)?;
    let limit = u64::try_from(limit).unwrap_or(u64::MAX);
    let mut reader = std::io::BufReader::new(file.take(limit));
    let mut line = Vec::new();
    loop {
        line.clear();
        if reader.read_until(b'\n', &mut line).map_err(read_error)? == 0 {
            return Ok(false);
        }
        line.retain(|&byte| byte != 0);
        let text = line
            .strip_prefix(b"\xef\xbb\xbf".as_slice())
            .or_else(|| line.strip_prefix(b"\xff\xfe".as_slice()))
            .unwrap_or(&line)
            .trim_ascii_start();
        let marker = |name: &[u8]| {
            text.get(..name.len())
                .is_some_and(|start| start.eq_ignore_ascii_case(name))
        };
        if marker(b"Values:") {
            return Ok(true);
        }
        if marker(b"Binary:") {
            return Ok(false);
        }
    }
}

/// A private directory for the converted store, removed with everything in it when dropped.
struct Staging(PathBuf);

impl Staging {
    fn create() -> Result<Self, CliError> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_nanos());
        let dir = std::env::temp_dir().join(format!(
            "rspice-bless-{}-{nanos}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dir).map_err(|source| CliError::output_error(&dir, source))?;
        Ok(Self(dir))
    }
}

impl Drop for Staging {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
