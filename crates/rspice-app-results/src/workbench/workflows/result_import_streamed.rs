//! A binary rawfile imported as it is read: its rows go into a results store segment a block at
//! a time, checked as they pass, and no more of the file is held than one read of it. The review facts
//! are gathered on the way, so nothing reads the segment again before the review opens.
//!
//! Only a rawfile the streamed reader reads is imported this way: one ordinary binary plot. Every
//! other source is read whole, as before. The segment is the one the whole-file import keeps for
//! the same file, byte for byte. The engine is [`rspice_formats::spice_raw::streamed_import`],
//! which the browser's import worker runs too.

use super::*;
use crate::state::run_store::{PushRows, RunMeta, StoreError, keep_segment};
use rspice_formats::spice_raw::streamed_import::{RawImport, RawImportStart};

/// The most of a file read before its header must have ended for its rows to be read as they
/// arrive.
const HEAD_BYTES: u64 = 1 << 20;

/// Bytes read between two feeds of the reader, two reports of progress and two looks at Cancel.
const FEED_BYTES: usize = 8 << 20;

/// The rawfile at `path` imported as it is read, when it is one plot of binary rows: `None` for
/// any other source, which the caller reads whole. Cancel, or a refusal, keeps nothing.
pub(super) fn stream_spice_raw(
    job: &JobContext,
    path: &Path,
    source_name: &str,
) -> Result<Option<StagedResultImport>, JobStop> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| format!("could not open {}: {error}", path.display()))?;
    let length = file
        .metadata()
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
        .len();
    let mut head = Vec::new();
    (&mut file)
        .take(HEAD_BYTES)
        .read_to_end(&mut head)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let format = ResultImportFormat::SpiceRaw;
    if head.is_empty() || identify_result_import_format(source_name, &head).ok() != Some(format) {
        return Ok(None);
    }
    let mut import = match RawImport::start(&head, length, &raw_import_policy())? {
        RawImportStart::Streamed(import) => import,
        RawImportStart::WholeFile => return Ok(None),
    };
    let schema = import.schema();
    let meta = RunMeta {
        analysis: format!("Imported \u{00b7} {source_name}"),
        ..RunMeta::default()
    };
    job.step(RESULT_IMPORT_READ_STEP, length);
    let mut stopped = None;
    let kept = keep_segment(&schema, &meta, &mut |push: &mut PushRows<'_>| {
        read_into(&mut import, job, &mut file, path, &head, push).map_err(|stop| {
            stopped = Some(stop);
            StoreError::Io {
                kind: std::io::ErrorKind::Interrupted,
                message: "the import stopped".to_owned(),
            }
        })
    });
    let segment = match kept {
        Ok(segment) => segment,
        Err(reason) => return Err(stopped.unwrap_or(JobStop::Failed(reason))),
    };
    Ok(Some(staged_from_kept(
        source_name,
        &segment,
        import.imported()?,
    )?))
}

/// Every row of the plot, checked and pushed into the segment: the rows in `head`, the file's
/// first bytes, then the rest of `file` from where `head` ended.
fn read_into(
    import: &mut RawImport,
    job: &JobContext,
    file: &mut std::fs::File,
    path: &Path,
    head: &[u8],
    push: &mut PushRows<'_>,
) -> Result<(), JobStop> {
    let mut pushed = |import: &mut RawImport| -> Result<(), JobStop> {
        let (x, values) = import.pending();
        if !x.is_empty() {
            push(x, values).map_err(|error| {
                JobStop::Failed(format!(
                    "the results store could not keep the samples: {error}"
                ))
            })?;
        }
        import.clear_pending();
        Ok(())
    };
    import.feed(head.get(import.data_offset()..).unwrap_or_default())?;
    pushed(import)?;
    let mut read = head.len() as u64;
    job.done(read);
    let mut buffer = vec![0; FEED_BYTES];
    loop {
        job.check()?;
        let count = file
            .read(&mut buffer)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        if count == 0 {
            break;
        }
        import.feed(&buffer[..count])?;
        pushed(import)?;
        read += count as u64;
        job.done(read);
    }
    import.finish()?;
    pushed(import)
}

#[cfg(test)]
#[path = "result_import_streamed/tests.rs"]
mod tests;
