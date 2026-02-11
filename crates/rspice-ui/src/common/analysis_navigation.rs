//! Analysis Navigation Rules
//!
//! Canonical UI-routing policy for analysis results:
//! - preferred status icon in results browser
//! - preferred bottom panel tab
//! - preferred specialized viewer

use crate::common::app::BottomPanelTab;
use crate::state::AnalysisType;
use crate::viewers::ActiveViewer;

/// Short icon tag for results-browser rows.
#[inline]
pub fn analysis_icon(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::DcOp => "[OP]",
        AnalysisType::DcSweep => "[DC]",
        AnalysisType::Ac => "[AC]",
        AnalysisType::Disto => "[DI]",
        AnalysisType::Transient => "[TR]",
        AnalysisType::Noise => "[NO]",
        AnalysisType::PoleZero => "[PZ]",
        AnalysisType::Tf => "[TF]",
        AnalysisType::Sensitivity => "[SN]",
        AnalysisType::Pac => "[PAC]",
        AnalysisType::Pnoise => "[PN]",
        AnalysisType::Pxf => "[PXF]",
        AnalysisType::Pstb => "[PSB]",
        AnalysisType::Stb => "[STB]",
        AnalysisType::MonteCarlo => "[MC]",
        AnalysisType::Parametric => "[PA]",
        AnalysisType::Corner => "[CR]",
        AnalysisType::Reliability => "[REL]",
        AnalysisType::Optimization => "[OPT]",
        AnalysisType::Soa => "[SOA]",
        AnalysisType::SParameter => "[SP]",
        AnalysisType::Envelope => "[ENV]",
        AnalysisType::Fourier => "[FOU]",
        AnalysisType::HarmonicBalance => "[HB]",
        AnalysisType::Pss => "[PS]",
    }
}

/// Preferred bottom tab when activating a completed analysis result.
#[inline]
pub fn preferred_bottom_tab(analysis_type: AnalysisType) -> BottomPanelTab {
    match analysis_type {
        AnalysisType::DcOp => BottomPanelTab::Log,
        AnalysisType::DcSweep
        | AnalysisType::Ac
        | AnalysisType::Disto
        | AnalysisType::Transient
        | AnalysisType::Noise
        | AnalysisType::PoleZero
        | AnalysisType::Tf
        | AnalysisType::Sensitivity
        | AnalysisType::Pac
        | AnalysisType::Pnoise
        | AnalysisType::Pxf
        | AnalysisType::Pstb
        | AnalysisType::Stb
        | AnalysisType::MonteCarlo
        | AnalysisType::Parametric
        | AnalysisType::Corner
        | AnalysisType::Reliability
        | AnalysisType::Optimization
        | AnalysisType::Soa
        | AnalysisType::SParameter
        | AnalysisType::Envelope
        | AnalysisType::Fourier
        | AnalysisType::HarmonicBalance
        | AnalysisType::Pss => BottomPanelTab::Waveform,
    }
}

/// Preferred specialized viewer for an analysis result.
#[inline]
pub fn preferred_viewer(analysis_type: AnalysisType) -> ActiveViewer {
    match analysis_type {
        AnalysisType::DcOp => ActiveViewer::Waveform,
        AnalysisType::DcSweep | AnalysisType::Transient | AnalysisType::Envelope => {
            ActiveViewer::Waveform
        }
        AnalysisType::Ac
        | AnalysisType::Disto
        | AnalysisType::Tf
        | AnalysisType::Pac
        | AnalysisType::Pxf => ActiveViewer::BodePlot,
        AnalysisType::Noise | AnalysisType::Pnoise => ActiveViewer::BodePlot,
        AnalysisType::PoleZero => ActiveViewer::PoleZero,
        AnalysisType::Sensitivity => ActiveViewer::Waveform,
        AnalysisType::Pstb | AnalysisType::Stb => ActiveViewer::Nyquist,
        AnalysisType::MonteCarlo | AnalysisType::Corner | AnalysisType::Parametric => {
            ActiveViewer::Histogram
        }
        AnalysisType::Reliability | AnalysisType::Optimization | AnalysisType::Soa => {
            ActiveViewer::Waveform
        }
        AnalysisType::SParameter => ActiveViewer::SmithChart,
        AnalysisType::Fourier => ActiveViewer::Fft,
        AnalysisType::HarmonicBalance | AnalysisType::Pss => ActiveViewer::Waveform,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_ANALYSIS_TYPES: [AnalysisType; 25] = [
        AnalysisType::DcOp,
        AnalysisType::DcSweep,
        AnalysisType::Ac,
        AnalysisType::Disto,
        AnalysisType::Transient,
        AnalysisType::Noise,
        AnalysisType::PoleZero,
        AnalysisType::Tf,
        AnalysisType::Sensitivity,
        AnalysisType::Pac,
        AnalysisType::Pnoise,
        AnalysisType::Pxf,
        AnalysisType::Pstb,
        AnalysisType::Stb,
        AnalysisType::MonteCarlo,
        AnalysisType::Parametric,
        AnalysisType::Corner,
        AnalysisType::Reliability,
        AnalysisType::Optimization,
        AnalysisType::Soa,
        AnalysisType::SParameter,
        AnalysisType::Envelope,
        AnalysisType::Fourier,
        AnalysisType::HarmonicBalance,
        AnalysisType::Pss,
    ];

    #[test]
    fn analysis_icon_has_coverage_for_all_types() {
        for analysis in ALL_ANALYSIS_TYPES {
            let icon = analysis_icon(analysis);
            assert!(
                !icon.is_empty(),
                "analysis icon cannot be empty for {:?}",
                analysis
            );
        }
    }

    #[test]
    fn preferred_bottom_tab_has_coverage_for_all_types() {
        for analysis in ALL_ANALYSIS_TYPES {
            let _ = preferred_bottom_tab(analysis);
        }
    }

    #[test]
    fn preferred_viewer_has_coverage_for_all_types() {
        for analysis in ALL_ANALYSIS_TYPES {
            let _ = preferred_viewer(analysis);
        }
    }

    #[test]
    fn preferred_bottom_tab_routes_dcop_to_log() {
        assert_eq!(preferred_bottom_tab(AnalysisType::DcOp), BottomPanelTab::Log);
    }

    #[test]
    fn preferred_viewer_routes_specialized_analysis_types() {
        assert_eq!(preferred_viewer(AnalysisType::Ac), ActiveViewer::BodePlot);
        assert_eq!(
            preferred_viewer(AnalysisType::SParameter),
            ActiveViewer::SmithChart
        );
        assert_eq!(preferred_viewer(AnalysisType::PoleZero), ActiveViewer::PoleZero);
        assert_eq!(preferred_viewer(AnalysisType::Pstb), ActiveViewer::Nyquist);
        assert_eq!(
            preferred_viewer(AnalysisType::MonteCarlo),
            ActiveViewer::Histogram
        );
        assert_eq!(preferred_viewer(AnalysisType::Fourier), ActiveViewer::Fft);
    }
}
