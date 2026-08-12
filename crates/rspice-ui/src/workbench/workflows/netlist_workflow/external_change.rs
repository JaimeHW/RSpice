//! Reconciling an owned deck with a change made outside the workspace.
//!
//! The three-way merge uses the last saved digest as its base; a region it
//! cannot reconcile becomes an explicit conflict block rather than a guess.

// Staging an external change reads and digests a host file, which only the
// native build can reopen; the browser gets the refusing stub at the bottom of
// this file.
#[cfg(not(target_arch = "wasm32"))]
use super::bundle::decode_import_bytes;
#[cfg(not(target_arch = "wasm32"))]
use super::import::sha256;
use super::*;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub(super) struct MergeLineEdit {
    start: usize,
    end: usize,
    replacement: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn merge_line_edits(base: &[&str], changed: &[&str]) -> Vec<MergeLineEdit> {
    let diff = similar::TextDiff::from_slices(base, changed);
    diff.ops()
        .iter()
        .filter(|operation| operation.tag() != similar::DiffTag::Equal)
        .map(|operation| {
            let old = operation.old_range();
            let new = operation.new_range();
            MergeLineEdit {
                start: old.start,
                end: old.end,
                replacement: diff.new_slices()[new].concat(),
            }
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn merge_edits_overlap(left: &MergeLineEdit, right: &MergeLineEdit) -> bool {
    if left.start == left.end && right.start == right.end {
        return left.start == right.start;
    }
    if left.start == left.end || right.start == right.end {
        return false;
    }
    left.start.max(right.start) < left.end.min(right.end)
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn render_merge_region(
    base: &[&str],
    start: usize,
    end: usize,
    edits: &[MergeLineEdit],
) -> String {
    let mut output = String::new();
    let mut cursor = start;
    for edit in edits {
        output.push_str(&base[cursor..edit.start].concat());
        output.push_str(&edit.replacement);
        cursor = edit.end;
    }
    output.push_str(&base[cursor..end].concat());
    output
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn edit_intersects_region(edit: &MergeLineEdit, start: usize, end: usize) -> bool {
    if edit.start == edit.end {
        edit.start > start && edit.start < end
    } else {
        edit.start < end && edit.end > start
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn three_way_merge_source(
    base: Option<&str>,
    local: &str,
    external: &str,
) -> (String, usize) {
    if local == external {
        return (local.to_owned(), 0);
    }
    let Some(base) = base else {
        return (merge_conflict_block(local, external), 1);
    };
    if local == base {
        return (external.to_owned(), 0);
    }
    if external == base {
        return (local.to_owned(), 0);
    }

    let base_lines = base.split_inclusive('\n').collect::<Vec<_>>();
    let local_lines = local.split_inclusive('\n').collect::<Vec<_>>();
    let external_lines = external.split_inclusive('\n').collect::<Vec<_>>();
    let local_edits = merge_line_edits(&base_lines, &local_lines);
    let external_edits = merge_line_edits(&base_lines, &external_lines);
    let mut merged = String::with_capacity(local.len().max(external.len()));
    let mut conflicts = 0_usize;
    let mut local_index = 0_usize;
    let mut external_index = 0_usize;
    let mut cursor = 0_usize;
    while local_index < local_edits.len() || external_index < external_edits.len() {
        let local_edit = local_edits.get(local_index);
        let external_edit = external_edits.get(external_index);
        if let (Some(local_edit), Some(external_edit)) = (local_edit, external_edit)
            && merge_edits_overlap(local_edit, external_edit)
        {
            let cluster_start = local_edit.start.min(external_edit.start);
            let mut cluster_end = local_edit.end.max(external_edit.end);
            let mut local_end = local_index + 1;
            let mut external_end = external_index + 1;
            loop {
                let mut extended = false;
                while let Some(edit) = local_edits.get(local_end)
                    && edit_intersects_region(edit, cluster_start, cluster_end)
                {
                    cluster_end = cluster_end.max(edit.end);
                    local_end += 1;
                    extended = true;
                }
                while let Some(edit) = external_edits.get(external_end)
                    && edit_intersects_region(edit, cluster_start, cluster_end)
                {
                    cluster_end = cluster_end.max(edit.end);
                    external_end += 1;
                    extended = true;
                }
                if !extended {
                    break;
                }
            }
            merged.push_str(&base_lines[cursor..cluster_start].concat());
            let base_region = base_lines[cluster_start..cluster_end].concat();
            let local_region = render_merge_region(
                &base_lines,
                cluster_start,
                cluster_end,
                &local_edits[local_index..local_end],
            );
            let external_region = render_merge_region(
                &base_lines,
                cluster_start,
                cluster_end,
                &external_edits[external_index..external_end],
            );
            if local_region == external_region {
                merged.push_str(&local_region);
            } else if local_region == base_region {
                merged.push_str(&external_region);
            } else if external_region == base_region {
                merged.push_str(&local_region);
            } else {
                conflicts += 1;
                merged.push_str(&merge_conflict_block(&local_region, &external_region));
            }
            cursor = cluster_end;
            local_index = local_end;
            external_index = external_end;
            continue;
        }

        let use_local = match (local_edit, external_edit) {
            (Some(local), Some(external)) => local.start <= external.start,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => break,
        };
        let edit = if use_local {
            let edit = &local_edits[local_index];
            local_index += 1;
            edit
        } else {
            let edit = &external_edits[external_index];
            external_index += 1;
            edit
        };
        merged.push_str(&base_lines[cursor..edit.start].concat());
        merged.push_str(&edit.replacement);
        cursor = edit.end;
    }
    merged.push_str(&base_lines[cursor..].concat());
    (merged, conflicts)
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn merge_conflict_block(local: &str, external: &str) -> String {
    let mut merged = String::new();
    merged.push_str("<<<<<<< RSPICE LOCAL\n");
    merged.push_str(local);
    if !local.ends_with('\n') {
        merged.push('\n');
    }
    merged.push_str("=======\n");
    merged.push_str(external);
    if !external.ends_with('\n') {
        merged.push('\n');
    }
    merged.push_str(">>>>>>> EXTERNAL FILE\n");
    merged
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn stage_external_netlist_change(
    state: &mut AppState,
    path: &std::path::Path,
    expected_sha256: [u8; 32],
    local_source: &str,
) -> Result<bool, String> {
    let bytes = std::fs::read(path).map_err(|error| {
        format!("The existing source cannot be reopened for comparison: {error}")
    })?;
    if bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
        return Err(format!(
            "The externally changed source exceeds the supported {}-byte limit.",
            crate::io::project_io::MAX_PROJECT_FILE_BYTES
        ));
    }
    let observed_sha256 = sha256(&bytes);
    if observed_sha256 == expected_sha256 {
        return Ok(false);
    }
    let (external_source, external_encoding) = decode_import_bytes(&bytes)?;
    let descriptor = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .ok_or_else(|| "Owned source metadata is unavailable.".to_owned())?;
    let saved_digest = descriptor
        .save_history
        .last()
        .map(|record| record.content_digest);
    let base_source = saved_digest
        .and_then(|digest| {
            descriptor
                .revision_history
                .iter()
                .rev()
                .find(|snapshot| snapshot.content_digest == digest)
        })
        .or_else(|| {
            descriptor
                .save_history
                .is_empty()
                .then(|| descriptor.revision_history.first())
                .flatten()
        })
        .map(|snapshot| snapshot.source.clone());
    let (merged_source, merge_conflict_count) =
        three_way_merge_source(base_source.as_deref(), local_source, &external_source);
    let comparison = similar::TextDiff::from_lines(local_source, external_source.as_str())
        .unified_diff()
        .context_radius(3)
        .header("RSpice local editor", "External file")
        .to_string();
    state.ui.netlist.external_change = Some(
        crate::workbench::documents::netlist_document::NetlistExternalChangeState {
            path: path.to_path_buf(),
            expected_sha256,
            observed_sha256,
            local_source: local_source.to_owned(),
            external_source,
            base_source,
            merged_source,
            merge_conflict_count,
            comparison,
            external_encoding,
            resolution: crate::workbench::documents::netlist_document::NetlistExternalChangeResolution::Merge,
            error: None,
        },
    );
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    Ok(true)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn apply_staged_external_netlist_change(state: &mut AppState) -> Result<(), String> {
    use crate::workbench::documents::netlist_document::NetlistExternalChangeResolution;

    let review = state
        .ui
        .netlist
        .external_change
        .clone()
        .ok_or_else(|| "No external source change is staged.".to_owned())?;
    let current = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .cloned()
        .ok_or_else(|| "No current owned source document is available.".to_owned())?;
    if current.source() != review.local_source
        || state.workspace.netlist_source_path.as_deref() != Some(review.path.as_path())
        || state
            .workspace
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.external_file_sha256)
            != Some(review.expected_sha256)
    {
        return Err(
            "The source, path, or publication baseline changed while conflict review was open."
                .to_owned(),
        );
    }
    let current_external = std::fs::read(&review.path)
        .map_err(|error| format!("The external source can no longer be read: {error}"))?;
    if sha256(&current_external) != review.observed_sha256 {
        return Err(
            "The external source changed again while conflict review was open. Cancel and review the newer bytes."
                .to_owned(),
        );
    }

    let selected_source = match review.resolution {
        NetlistExternalChangeResolution::Merge => review.merged_source.clone(),
        NetlistExternalChangeResolution::KeepLocal => review.local_source.clone(),
        NetlistExternalChangeResolution::ReloadExternal => review.external_source.clone(),
    };
    let mut candidate = state.clone();
    let mut descriptor = candidate
        .workspace
        .netlist_descriptor
        .take()
        .ok_or_else(|| "Owned source metadata is unavailable.".to_owned())?;
    descriptor.retain_revision(&current, "Working state before external-change resolution")?;
    descriptor.external_file_sha256 = Some(review.observed_sha256);
    if review.resolution != NetlistExternalChangeResolution::KeepLocal {
        descriptor.source_encoding = review.external_encoding;
        descriptor.source_line_ending = crate::state::NetlistLineEnding::detect(&selected_source);
    }
    candidate.workspace.netlist_descriptor = Some(descriptor);

    if selected_source != review.local_source
        && !crate::workbench::documents::netlist_document::replace_owned_source(
            &mut candidate,
            selected_source.clone(),
        )
    {
        return Err(
            "The selected conflict resolution could not update the owned source.".to_owned(),
        );
    }
    let next_document = candidate
        .ui
        .netlist
        .owned_document
        .as_ref()
        .cloned()
        .ok_or_else(|| "The conflict resolution lost the canonical owned document.".to_owned())?;
    candidate
        .workspace
        .netlist_descriptor
        .as_mut()
        .unwrap()
        .retain_revision(
            &next_document,
            match review.resolution {
                NetlistExternalChangeResolution::Merge => "Merged external source change",
                NetlistExternalChangeResolution::KeepLocal => {
                    "Acknowledged external change; retained local source"
                }
                NetlistExternalChangeResolution::ReloadExternal => "Reloaded external source",
            },
        )?;
    candidate.ui.netlist.externally_saved_content_digest = (review.resolution
        == NetlistExternalChangeResolution::ReloadExternal)
        .then(|| crate::state::content_digest(&selected_source));
    candidate.ui.netlist.external_change = None;
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    *state = candidate;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn apply_staged_external_netlist_change(_state: &mut AppState) -> Result<(), String> {
    Err("Browser downloads do not expose reopenable external source authority.".to_owned())
}
