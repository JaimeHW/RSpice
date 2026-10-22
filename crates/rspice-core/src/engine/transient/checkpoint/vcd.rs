//! The VCD files the `d_vcd` stimuli replay.
//!
//! A `d_vcd` instance reads its file once, when it is initialized, and then
//! replays it from cursors in its context state, which the checkpoint carries
//! like any other model's. The file is the half the image does not carry: a
//! run resumed against an edited file would read the edited changes and
//! index them with cursors the original file's changes left. The netlist
//! identity hashes the file as well, but its refusal names a digest, not the
//! file. This section records each instance's file and the BLAKE3 digest of
//! the bytes it read, and a resume compares them before anything else, so the
//! refusal names the file.
use super::*;

/// Format 62 adds the files the `d_vcd` stimuli replay.
pub(super) const FORMAT_VERSION: u32 = 62;

/// One instance's file, and the digest of the bytes it read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReplayedFile {
    pub(super) instance: String,
    pub(super) path: String,
    pub(super) digest: blake3::Hash,
}

pub(super) fn capture(circuit: &CircuitData) -> Vec<ReplayedFile> {
    circuit
        .xspice_instances
        .iter()
        .filter_map(|instance| {
            let (path, digest) = crate::xspice::models::vcd_source_file(instance)?;
            Some(ReplayedFile {
                instance: instance.name.clone(),
                path,
                digest,
            })
        })
        .collect()
}

/// Refuse a resume when a file a stimulus replays no longer holds the bytes
/// the checkpointed run read.
pub(super) fn validate_files(files: &[ReplayedFile]) -> Result<(), String> {
    for file in files {
        match crate::xspice::models::vcd_source_digest(&file.path) {
            Ok(digest) if digest == file.digest => {}
            Ok(_) => {
                return Err(format!(
                    "the VCD file '{}' that d_vcd instance {} replays has changed since the \
                     checkpoint was taken; refusing to resume against an edited stimulus",
                    file.path, file.instance
                ));
            }
            Err(error) => {
                return Err(format!(
                    "the VCD file '{}' that d_vcd instance {} replays cannot be read to check it \
                     against the checkpoint ({error}); refusing to resume",
                    file.path, file.instance
                ));
            }
        }
    }
    Ok(())
}

/// Values a parsed image retains, for the checkpoint memory budget.
pub(super) fn retained_values(files: &[ReplayedFile]) -> usize {
    files
        .iter()
        .map(|file| file.instance.len().saturating_add(file.path.len()))
        .fold(1, usize::saturating_add)
}

pub(super) fn write(
    out: &mut String,
    files: &[ReplayedFile],
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!("vcd_files {}\n", files.len()));
    for (index, file) in files.iter().enumerate() {
        poll_checkpoint_abort(abort, index)?;
        // The path is a JSON string, so a space or a control character in it
        // stays inside its field.
        let path = serde_json::to_string(&file.path)
            .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        out.push_str(&format!(
            "vcd_file {} {} {path}\n",
            file.instance,
            file.digest.to_hex()
        ));
    }
    Ok(())
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    version: u32,
    budget: &mut CheckpointParseBudget,
) -> Result<Vec<ReplayedFile>, String> {
    // A file older than this section predates `d_vcd`.
    if version < FORMAT_VERSION {
        return Ok(Vec::new());
    }
    let header = lines.next().ok_or("missing 'vcd_files' section")?;
    let count = parse_count_header(header, "vcd_files")?;
    let mut files = allocate_checkpoint_capacity(count, "vcd_files", budget)?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'vcd_files' truncated at row {row}"))?;
        let malformed = || format!("malformed 'vcd_file' row: '{line}'");
        let mut fields = line.splitn(4, ' ');
        if fields.next() != Some("vcd_file") {
            return Err(malformed());
        }
        let (Some(instance), Some(digest), Some(path)) =
            (fields.next(), fields.next(), fields.next())
        else {
            return Err(malformed());
        };
        let digest = blake3::Hash::from_hex(digest).map_err(|_| malformed())?;
        let path = serde_json::from_str::<String>(path).map_err(|_| malformed())?;
        if instance.is_empty() {
            return Err(malformed());
        }
        files.push(ReplayedFile {
            instance: instance.to_string(),
            path,
            digest,
        });
    }
    Ok(files)
}
