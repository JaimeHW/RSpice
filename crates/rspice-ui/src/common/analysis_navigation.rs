//! Analysis Navigation Rules
//!
//! Canonical UI-routing policy for analysis results:
//! - preferred status icon in results browser
//! - preferred bottom panel tab
//! - preferred specialized viewer

use crate::common::app::BottomPanelTab;
use crate::state::AnalysisType;
use crate::viewers::ActiveViewer;

const WAVEFORM_ONLY: &[ActiveViewer] = &[ActiveViewer::Waveform];
const AC_VIEWERS: &[ActiveViewer] = &[
    ActiveViewer::BodePlot,
    ActiveViewer::Nyquist,
    ActiveViewer::SmithChart,
    ActiveViewer::Waveform,
];
const NOISE_VIEWERS: &[ActiveViewer] = &[ActiveViewer::BodePlot, ActiveViewer::Waveform];
const STABILITY_VIEWERS: &[ActiveViewer] = &[
    ActiveViewer::Nyquist,
    ActiveViewer::BodePlot,
    ActiveViewer::Waveform,
];
const POLE_ZERO_VIEWERS: &[ActiveViewer] = &[
    ActiveViewer::PoleZero,
    ActiveViewer::BodePlot,
    ActiveViewer::Waveform,
];
const HISTOGRAM_VIEWERS: &[ActiveViewer] = &[ActiveViewer::Histogram, ActiveViewer::Waveform];
const SPARAM_VIEWERS: &[ActiveViewer] = &[
    ActiveViewer::SmithChart,
    ActiveViewer::BodePlot,
    ActiveViewer::Nyquist,
    ActiveViewer::Waveform,
];
const FFT_VIEWERS: &[ActiveViewer] = &[ActiveViewer::Fft, ActiveViewer::Waveform];

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
    preferred_viewers(analysis_type)[0]
}

/// Priority-ordered viewers for an analysis result.
///
/// The first entry is the canonical preferred viewer.
#[inline]
pub fn preferred_viewers(analysis_type: AnalysisType) -> &'static [ActiveViewer] {
    match analysis_type {
        AnalysisType::DcOp => WAVEFORM_ONLY,
        AnalysisType::DcSweep | AnalysisType::Transient | AnalysisType::Envelope => WAVEFORM_ONLY,
        AnalysisType::Ac
        | AnalysisType::Disto
        | AnalysisType::Tf
        | AnalysisType::Pac
        | AnalysisType::Pxf => AC_VIEWERS,
        AnalysisType::Noise | AnalysisType::Pnoise => NOISE_VIEWERS,
        AnalysisType::PoleZero => POLE_ZERO_VIEWERS,
        AnalysisType::Sensitivity => WAVEFORM_ONLY,
        AnalysisType::Pstb | AnalysisType::Stb => STABILITY_VIEWERS,
        AnalysisType::MonteCarlo | AnalysisType::Corner | AnalysisType::Parametric => {
            HISTOGRAM_VIEWERS
        }
        AnalysisType::Reliability | AnalysisType::Optimization | AnalysisType::Soa => WAVEFORM_ONLY,
        AnalysisType::SParameter => SPARAM_VIEWERS,
        AnalysisType::Fourier => FFT_VIEWERS,
        AnalysisType::HarmonicBalance | AnalysisType::Pss => WAVEFORM_ONLY,
    }
}

