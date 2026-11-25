//! Authentication of frozen report figures.

use super::*;
use rspice_results::report_figure::{REPORT_FIGURE_MEDIA_TYPE, ReportFigure};

/// The plot a frozen figure keeps, once its payload proves to be the one
/// its digest names, and a plot print can lay out.
pub(super) fn validate_frozen_report_figure(
    block_id: ReportBlockId,
    artifact: &FrozenReportArtifact,
) -> Result<ReportFigure, HardcopySourceError> {
    if artifact.media_type() != REPORT_FIGURE_MEDIA_TYPE {
        return Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
            block_id,
            kind: "frozen plot figure",
            reason: format!(
                "unsupported artifact media type `{}`; expected {REPORT_FIGURE_MEDIA_TYPE}",
                artifact.media_type()
            ),
        });
    }
    let computed = ContentDigest::from_bytes(Sha256::digest(artifact.payload()).into());
    if computed != artifact.content_digest() {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} artifact digest does not authenticate its payload"
        )));
    }
    ReportFigure::read(artifact).map_err(|error| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} is not a plot print can lay out: {error}"
        ))
    })
}
