//! Resolve immutable authored inputs before constructing the executable snapshot.
use super::*;
use rspice_results::transient_checkpoint::{
    TransientCheckpointEvidence, TransientCheckpointLibrary,
};
use rspice_simulation_contract::tran_checkpoint::TranCheckpointDraft;

pub(crate) fn request_from_draft(
    draft: Option<&TranCheckpointDraft>,
    stop: f64,
) -> Result<Option<TransientCheckpointRequest>, String> {
    let Some(draft) = draft else { return Ok(None) };
    let mut times = Vec::new();
    if let Some(interval) = draft.capture_interval()? {
        if !stop.is_finite() || stop <= 0.0 {
            return Err("Checkpoint capture requires a finite positive stop time".into());
        }
        let count = (stop / interval).ceil();
        let limit = ResourceLimits::default().max_analysis_points;
        if !count.is_finite() || count > limit as f64 {
            return Err(format!(
                "Transient checkpoint schedule exceeds {limit} capture points; increase the interval"
            ));
        }
        for index in 1..(count as usize) {
            let time = index as f64 * interval;
            if time >= stop {
                break;
            }
            if times.last().is_some_and(|prior| time <= *prior) {
                return Err(
                    "Transient checkpoint interval cannot be represented at this horizon".into(),
                );
            }
            times.push(time);
        }
        times.push(stop);
    }
    if times.is_empty() && draft.resume.is_empty() {
        return Ok(None);
    }
    // Resume bytes are attached only after the exact source/run point exists.
    Ok(Some(TransientCheckpointRequest {
        times,
        resume: None,
    }))
}

/// The images a transient's checkpoint draft resumes from, as preparing it
/// resolves them, in digest order: each selected image from the imported
/// library or from a retained result that holds it, every such result
/// checked as one the image can be resumed against. The reason instead when
/// an image is selected twice, is held nowhere, is held by a result it
/// cannot be resumed against, or when together they exceed the byte limit.
///
/// What a run prepared to resume from checkpoints resumes from: the same
/// answer, asked again, says whether it still does.
pub fn selected_images<'a, W: 'a>(
    analyses: impl Iterator<Item = &'a rspice_results::analysis_result::AnalysisResult<W>>,
    imported: &TransientCheckpointLibrary,
    draft: Option<&TranCheckpointDraft>,
) -> Result<Vec<TransientCheckpointEvidence>, String> {
    let Some(draft) = draft else {
        return Ok(Vec::new());
    };
    let mut selected = std::collections::BTreeMap::new();
    for digest in &draft.resume {
        if selected
            .insert(*digest, imported.get(*digest).cloned())
            .is_some()
        {
            return Err("A transient checkpoint is selected more than once".into());
        }
    }
    for analysis in analyses {
        if let Some(image) = &analysis.transient_checkpoint
            && let Some(slot) = selected.get_mut(&image.digest())
        {
            image.validate_for(analysis)?;
            *slot = Some(image.clone());
        }
    }
    let mut total = 0usize;
    selected.into_iter().map(|(digest, image)| {
        let image = image.ok_or_else(|| format!("Selected transient checkpoint {digest} is unavailable; restore its run, import its file, or clear the selection"))?;
        total = total.saturating_add(image.bytes().len());
        if total > ResourceLimits::default().max_external_data_bytes {
            return Err("Selected transient checkpoints exceed the combined byte limit".into());
        }
        Ok(image)
    }).collect()
}
