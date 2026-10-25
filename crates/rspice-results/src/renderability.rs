//! Whether a retained analysis is evidence a result sheet can draw.
//!
//! The Results sheets, the printed page and the visualization document all ask
//! these questions of the same immutable datasets, so the rule is stated once,
//! here. Several answers cost a walk of every retained sample; each such walk
//! is reported to the caller's `note` as an [`EvidenceScan`] just before it is
//! performed, so a host that holds its frames to memoized work can count them.

use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_result::AnalysisResult;
use crate::analysis_type::AnalysisType;
use crate::family_metadata::AnalysisResultFamilyMetadata;
use crate::network_matrix::{SParameterTraceIdentity, trace_identity};
use crate::result_presentation::ResultViewer;
use crate::waveform::RetainedTrace;

/// One retained analysis with its traces' display choices.
type Analysis = AnalysisResult<RetainedTrace>;

/// One whole-dataset walk a renderability question performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceScan {
    /// `AnalysisResult::validate_retained_evidence`: every retained sample of
    /// every waveform in one analysis.
    EvidenceValidation,
    /// One retained ordinary-noise spectrum.
    NoiseSpectrum,
    /// One retained raw frequency curve, the distortion family.
    RawFrequencyCurve,
    /// One retained discrete complex coefficient spectrum.
    HarmonicSpectrum,
    /// One retained phase-noise trace.
    PhaseNoiseSpectrum,
    /// One retained S-parameter trace: every frequency and every complex
    /// coefficient behind it.
    SParameterTrace,
    /// The sample grid and values of one legacy event projection.
    EventProjection,
}

/// A structural question a tab strip asks about a retained analysis whose
/// answer costs a walk of every retained sample.
///
/// Every one of these is a *shape* question — can this sheet draw this
/// evidence? — that the retained vectors alone can answer, and every one of
/// them was asked directly by the availability gates, about every analysis in
/// the run, on every frame. The magnitude/phase half of the Bode question is
/// genuinely cheap and is answered without coming here; the rest are not, so
/// hosts memoize the answer per dataset generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralGate {
    /// A frequency response or a raw distortion curve the Bode sheet can
    /// draw; see [`bode_analysis_is_renderable`].
    BodeResponse,
    /// An ordinary-noise spectrum; see [`ordinary_noise_spectrum_is_renderable`].
    OrdinaryNoiseSpectrum,
    /// A discrete complex coefficient spectrum; see
    /// [`harmonic_spectrum_is_renderable`].
    HarmonicSpectrum,
    /// An explicitly-labelled phase-noise trace with a retained carrier; see
    /// [`phase_noise_is_renderable`].
    PhaseNoiseSpectrum,
    /// S-parameter traces with per-port reference impedances; see
    /// [`smith_structure_is_renderable`].
    SParameterStructure,
    NetworkMatrix,
    /// Exact event histories or validated legacy event projections.
    EventHistory,
}

/// One structural gate answered by walking the evidence.
pub fn structural_gate_is_answered(
    gate: StructuralGate,
    analysis: &Analysis,
    note: impl FnMut(EvidenceScan),
) -> bool {
    match gate {
        StructuralGate::BodeResponse => bode_analysis_is_renderable(analysis, note),
        StructuralGate::OrdinaryNoiseSpectrum => {
            ordinary_noise_spectrum_is_renderable(analysis, note)
        }
        StructuralGate::HarmonicSpectrum => harmonic_spectrum_is_renderable(analysis, note),
        StructuralGate::PhaseNoiseSpectrum => phase_noise_is_renderable(analysis, note),
        StructuralGate::SParameterStructure => smith_structure_is_renderable(analysis, note),
        StructuralGate::NetworkMatrix => network_matrix_structure_is_renderable(analysis, note),
        StructuralGate::EventHistory => event_history_is_renderable(analysis, note),
    }
}

/// Whether one analysis carries an ordinary-noise spectrum a reader may see.
pub fn ordinary_noise_spectrum_is_renderable(
    analysis: &Analysis,
    mut note: impl FnMut(EvidenceScan),
) -> bool {
    crate::noise_spectrum::ordinary_noise_spectrum_is_renderable(
        analysis.success,
        analysis.analysis_type,
        &analysis.waveforms,
        || note(EvidenceScan::NoiseSpectrum),
    )
}

/// Whether one analysis carries a discrete complex coefficient spectrum.
pub fn harmonic_spectrum_is_renderable(
    analysis: &Analysis,
    mut note: impl FnMut(EvidenceScan),
) -> bool {
    crate::harmonic_spectrum::analysis_is_renderable(
        analysis.success,
        analysis.analysis_type,
        &analysis.waveforms,
        || note(EvidenceScan::HarmonicSpectrum),
    )
}

/// Whether one analysis carries an explicitly-labelled phase-noise trace with
/// a retained carrier.
pub fn phase_noise_is_renderable(analysis: &Analysis, mut note: impl FnMut(EvidenceScan)) -> bool {
    crate::phase_noise::phase_noise_is_renderable(
        analysis.success,
        analysis.analysis_type,
        analysis.family_metadata.as_ref(),
        &analysis.waveforms,
        || note(EvidenceScan::PhaseNoiseSpectrum),
    )
}

/// Whether the tab strip offers a frequency-response sheet for one analysis.
///
/// The success gate belongs to both branches. A Bode summary resolves off the
/// retained magnitude and phase vectors alone, so a failed AC solve still
/// produces one — and the sheet behind this offering refuses a failed solve
/// outright, because margins read off vectors the engine emitted before
/// giving up are not measurements.
///
/// The magnitude/phase branch is a question of shape — are the traces a Bode
/// sheet needs present? — so it is asked of the trace names and not of the
/// summary, which would measure the whole response first. The distortion
/// family retains plain frequency curves with no `|…|`/`phase(…)` pairing to
/// resolve, so its branch has to prove the curve itself is well formed, which
/// is a walk of every retained sample.
pub fn bode_analysis_is_renderable(
    analysis: &Analysis,
    mut note: impl FnMut(EvidenceScan),
) -> bool {
    analysis.success
        && (crate::bode::retained::ac_bode_shape_for_analysis(
            analysis.analysis_type,
            &analysis.waveforms,
            0,
            |waveform| waveform.visible,
        )
        .is_some()
            || (analysis.analysis_type.is_raw_frequency_curve()
                && analysis
                    .waveforms
                    .iter()
                    .any(|waveform| raw_frequency_curve_is_renderable(waveform, &mut note))))
}

/// Whether one retained distortion curve is drawable: paired coordinates, a
/// positive finite abscissa at every sample, and a strictly ascending sweep.
fn raw_frequency_curve_is_renderable(
    waveform: &RetainedTrace,
    note: &mut impl FnMut(EvidenceScan),
) -> bool {
    if !waveform.visible || waveform.x.len() != waveform.y.len() || waveform.x.len() < 2 {
        return false;
    }
    note(EvidenceScan::RawFrequencyCurve);
    waveform
        .x
        .iter()
        .zip(waveform.y.iter())
        .all(|(&x, &y)| x.is_finite() && x > 0.0 && y.is_finite())
        && waveform.x.windows(2).all(|pair| pair[0] < pair[1])
}

/// Whether one QPNOISE analysis carries a spectrum a reader may see.
pub fn qpnoise_spectrum_is_renderable(analysis: &Analysis) -> bool {
    analysis.success
        && crate::noise_spectrum::qpnoise_is_renderable(analysis)
        && analysis.validate_retained_evidence().is_ok()
}

/// Analyses whose selection states which noise result the reader means. A
/// selection outside this family — a transient carried over from another
/// viewer — says nothing about noise at all.
fn is_noise_analysis(analysis_type: AnalysisType) -> bool {
    matches!(
        analysis_type,
        AnalysisType::Noise | AnalysisType::Pnoise | AnalysisType::Hbnoise | AnalysisType::Qpnoise
    )
}

/// The one noise analysis a run's ordinary-noise surfaces bind to, given which
/// analyses carry a renderable spectrum.
///
/// A selected noise analysis binds strictly. If it carries no renderable
/// ordinary spectrum — a phase-noise result, or one whose solve failed — the
/// binding is empty; quietly substituting a neighbouring result would put a
/// different analysis's contributors under the reader's selection. The
/// run-wide fallback applies only when the selection expresses no noise
/// intent for the binding to honour.
///
/// The sheet answers `renderable` from its memo and the printed page walks the
/// evidence, but both must reach the same analysis, so the rule takes the
/// oracle as a parameter rather than being written out twice.
pub fn select_ordinary_noise_analysis<A: AsRef<Analysis>>(
    globally_selected: Option<usize>,
    analyses: &[A],
    mut renderable: impl FnMut(&A) -> bool,
) -> Option<usize> {
    if let Some(selected) = globally_selected
        && let Some(analysis) = analyses.get(selected)
        && is_noise_analysis(analysis.as_ref().analysis_type)
    {
        return renderable(analysis).then_some(selected);
    }
    analyses.iter().position(renderable)
}

/// The one noise analysis a run's noise surfaces bind to, QPNOISE included,
/// with renderability resolved by walking the evidence.
pub fn selected_noise_analysis_index<A: AsRef<Analysis>>(
    globally_selected: Option<usize>,
    analyses: &[A],
    mut note: impl FnMut(EvidenceScan),
) -> Option<usize> {
    if let Some(index) = globally_selected
        && let Some(analysis) = analyses.get(index)
        && analysis.as_ref().analysis_type == AnalysisType::Qpnoise
    {
        return qpnoise_spectrum_is_renderable(analysis.as_ref()).then_some(index);
    }
    select_ordinary_noise_analysis(globally_selected, analyses, |analysis| {
        ordinary_noise_spectrum_is_renderable(analysis.as_ref(), &mut note)
    })
    .or_else(|| {
        // A failed explicitly selected noise analysis must remain selected.
        if globally_selected
            .and_then(|i| analyses.get(i))
            .is_some_and(|a| {
                matches!(
                    a.as_ref().analysis_type,
                    AnalysisType::Noise | AnalysisType::Hbnoise | AnalysisType::Pnoise
                )
            })
        {
            None
        } else {
            analyses
                .iter()
                .position(|analysis| qpnoise_spectrum_is_renderable(analysis.as_ref()))
        }
    })
}

/// Whether one retained S-parameter trace is drawable: complex coefficients
/// paired with every frequency, every frequency positive and strictly
/// ascending, and every coefficient finite.
fn s_parameter_trace_is_well_formed(
    waveform: &RetainedTrace,
    note: &mut impl FnMut(EvidenceScan),
) -> bool {
    let Some(complex) = waveform.complex.as_ref() else {
        return false;
    };
    if waveform.x.is_empty()
        || waveform.x.len() != complex.real.len()
        || waveform.x.len() != complex.imag.len()
    {
        return false;
    }
    note(EvidenceScan::SParameterTrace);
    waveform
        .x
        .iter()
        .all(|frequency| frequency.is_finite() && *frequency > 0.0)
        && waveform.x.windows(2).all(|pair| pair[0] < pair[1])
        && complex
            .real
            .iter()
            .chain(complex.imag.iter())
            .all(|value| value.is_finite())
}

/// Whether the Smith sheet can draw one analysis, its retained evidence
/// validated as well.
pub fn smith_analysis_is_renderable(
    analysis: &Analysis,
    mut note: impl FnMut(EvidenceScan),
) -> bool {
    note(EvidenceScan::EvidenceValidation);
    analysis.validate_retained_evidence().is_ok() && smith_structure_is_renderable(analysis, note)
}

/// Whether the Smith sheet can draw one analysis, with the retained-evidence
/// verdict left to the caller.
///
/// Split out because the per-frame callers take that verdict from a memo
/// instead of walking every retained complex sample again.
pub fn smith_structure_is_renderable(
    analysis: &Analysis,
    mut note: impl FnMut(EvidenceScan),
) -> bool {
    if !analysis.success
        || !matches!(
            analysis.analysis_type,
            AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp
        )
    {
        return false;
    }
    let Some(AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
        ..
    }) = analysis.family_metadata.as_ref()
    else {
        return false;
    };
    analysis.waveforms.iter().any(|waveform| {
        let Some(identity) = trace_identity(&waveform.name) else {
            return false;
        };
        let port_count = if identity.physical_ports {
            reference_impedances_ohm.len()
        } else if reference_impedances_ohm.len().is_multiple_of(2) {
            reference_impedances_ohm.len() / 2
        } else {
            return false;
        };
        identity.output_port <= port_count
            && identity.input_port <= port_count
            && s_parameter_trace_is_well_formed(waveform, &mut note)
    })
}

/// Whether one analysis retains complete network matrices on a shared
/// frequency grid.
pub fn network_matrix_structure_is_renderable(
    analysis: &Analysis,
    mut note: impl FnMut(EvidenceScan),
) -> bool {
    let Some(matrix) = crate::network_matrix::resolve(analysis) else {
        return false;
    };
    note(EvidenceScan::SParameterTrace);
    matrix.samples_are_finite_and_aligned(analysis)
}

/// Whether one analysis retains exact event histories or a validated legacy
/// event projection.
pub fn event_history_is_renderable(
    analysis: &Analysis,
    mut note: impl FnMut(EvidenceScan),
) -> bool {
    (analysis.success || analysis.is_live_partial())
        && analysis.analysis_type == AnalysisType::Transient
        && (matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
                current_impulses,
                ..
            }) if !digital_traces.is_empty() || !real_traces.is_empty() || current_impulses.is_some()
        ) || analysis.waveforms.iter().any(|waveform| {
            crate::events::projection::waveform_is_event(waveform, || {
                note(EvidenceScan::EventProjection);
            })
        }))
}

/// Whether one analysis is a time-domain result with a visible, paired
/// waveform to plot.
pub fn transient_waveform_is_renderable(analysis: &Analysis) -> bool {
    analysis.success
        && analysis.analysis_type.is_time_domain()
        && analysis.waveforms.iter().any(|waveform| {
            waveform.visible && !waveform.x.is_empty() && waveform.x.len() == waveform.y.len()
        })
}

/// How one retained waveform is a quantity the polar sheet draws.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolarQuantitySource<'a> {
    /// The complex response of an AC or harmonic-balance analysis.
    Response,
    /// One network term of an S-parameter analysis, at a port pair the run
    /// declares.
    NetworkTerm {
        identity: SParameterTraceIdentity,
        reference_impedances_ohm: &'a [f64],
    },
}

/// The per-port reference impedances of a network analysis: present exactly
/// when the polar sheet names the analysis' complex traces as network terms.
pub fn polar_reference_impedances(analysis: &Analysis) -> Option<&[f64]> {
    match (&analysis.analysis_type, analysis.family_metadata.as_ref()) {
        (
            AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp,
            Some(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm,
                ..
            }),
        ) => Some(reference_impedances_ohm),
        _ => None,
    }
}

/// Whether a retained waveform carries a complex series the polar sheet can
/// draw.
///
/// Deliberately O(1): the deep finite/ordering check is
/// [`AnalysisResult::validate_retained_evidence`], which the availability gate
/// resolves through a memo once per dataset generation. Repeating it here
/// would walk every sample of every trace on every frame.
fn complex_is_plottable(waveform: &RetainedTrace) -> bool {
    waveform.complex.as_ref().is_some_and(|complex| {
        !complex.real.is_empty()
            && complex.real.len() == complex.imag.len()
            && complex.real.len() == waveform.x.len()
    })
}

/// Whether, and as what, one retained waveform of a successful analysis is a
/// quantity the polar sheet draws. `network` is
/// [`polar_reference_impedances`] of that analysis.
pub fn polar_quantity_source<'a>(
    analysis_type: AnalysisType,
    network: Option<&'a [f64]>,
    waveform: &RetainedTrace,
) -> Option<PolarQuantitySource<'a>> {
    if !complex_is_plottable(waveform) {
        return None;
    }
    let complex = waveform.complex.as_ref()?;
    match network {
        Some(reference_impedances_ohm) => {
            let identity =
                trace_identity(&complex.source_name).or_else(|| trace_identity(&waveform.name))?;
            // Mixed-mode terms are indexed over half as many logical ports as
            // the run declares physical ones, exactly as the Smith sheet
            // reads them.
            let port_count = if identity.physical_ports {
                reference_impedances_ohm.len()
            } else if reference_impedances_ohm.len().is_multiple_of(2) {
                reference_impedances_ohm.len() / 2
            } else {
                return None;
            };
            if identity.output_port > port_count || identity.input_port > port_count {
                return None;
            }
            Some(PolarQuantitySource::NetworkTerm {
                identity,
                reference_impedances_ohm,
            })
        }
        None => matches!(
            analysis_type,
            AnalysisType::Ac | AnalysisType::HarmonicBalance
        )
        .then_some(PolarQuantitySource::Response),
    }
}

/// Whether the polar sheet has any quantity to draw from one analysis.
pub fn retains_polar_quantity(analysis: &Analysis) -> bool {
    if !analysis.success {
        return false;
    }
    let network = polar_reference_impedances(analysis);
    analysis
        .waveforms
        .iter()
        .any(|waveform| polar_quantity_source(analysis.analysis_type, network, waveform).is_some())
}

/// Whether one analysis retained an operating point the OP sheet can show.
pub fn operating_point_evidence_is_renderable(analysis: &Analysis) -> bool {
    analysis.dc_op.is_some()
        || analysis
            .device_op
            .as_ref()
            .is_some_and(|report| !report.is_empty())
        || matches!(
            analysis.result_payload,
            Some(AnalysisResultPayload::OperatingPoint { .. })
        )
}

/// One compatibility predicate shared by context resolution and later viewer
/// operations. It answers whether this retained analysis can be the evidence
/// behind the named viewer, walking the evidence to do so.
pub fn analysis_supports_viewer(
    viewer: ResultViewer,
    analysis: &Analysis,
    mut note: impl FnMut(EvidenceScan),
) -> bool {
    if !analysis.success || analysis.validate_retained_evidence().is_err() {
        return false;
    }
    viewer_can_render(viewer, analysis, |gate| {
        structural_gate_is_answered(gate, analysis, &mut note)
    })
}

/// Whether this viewer has anything to draw from the shape of the retained
/// result.
///
/// Most arms cost nothing — a payload discriminant, a non-empty vector. The
/// ones that do not are asked of `structural` about this same analysis, so the
/// caller decides whether they are walked or read from a memo; see
/// [`StructuralGate`].
pub fn viewer_can_render(
    viewer: ResultViewer,
    analysis: &Analysis,
    mut structural: impl FnMut(StructuralGate) -> bool,
) -> bool {
    match viewer {
        ResultViewer::Waves => {
            analysis.analysis_type.is_time_domain() && !analysis.waveforms.is_empty()
        }
        ResultViewer::DcSweep => {
            analysis.analysis_type == AnalysisType::DcSweep && !analysis.waveforms.is_empty()
        }
        ResultViewer::Bode => structural(StructuralGate::BodeResponse),
        ResultViewer::NoiseContrib => {
            crate::noise_spectrum::qpnoise_is_renderable(analysis)
                || structural(StructuralGate::OrdinaryNoiseSpectrum)
        }
        ResultViewer::Fft | ResultViewer::Eye => {
            analysis.analysis_type.is_time_domain() && !analysis.waveforms.is_empty()
        }
        ResultViewer::HarmonicBalance => structural(StructuralGate::HarmonicSpectrum),
        ResultViewer::PhaseNoise => structural(StructuralGate::PhaseNoiseSpectrum),
        ResultViewer::Nyquist => analysis.waveforms.iter().any(|waveform| {
            waveform.complex.as_ref().is_some_and(|complex| {
                !complex.real.is_empty()
                    && complex.real.len() == complex.imag.len()
                    && complex.real.len() == waveform.x.len()
            })
        }),
        ResultViewer::Smith => structural(StructuralGate::SParameterStructure),
        // The polar sheet draws any retained complex response, so its shape
        // question is the same one Nyquist asks — the analysis kind decides
        // which quantities it can name, not whether it can draw at all.
        ResultViewer::Polar => retains_polar_quantity(analysis),
        ResultViewer::NetworkMatrix => structural(StructuralGate::NetworkMatrix),
        ResultViewer::Scatter | ResultViewer::BoxViolin => matches!(
            analysis.family_metadata,
            Some(AnalysisResultFamilyMetadata::MonteCarlo { ref variables, ref member_measurements, .. })
                if variables.iter().any(|variable| variable.samples.len() >= 2)
                    || member_measurements.len() >= 2
        ),
        // The analysis kind is part of the question, exactly as it is in the
        // tab strip's own gate: the OP sheet renders nothing for an analysis
        // that is not a DC operating point, so a transient that retained its
        // bias solution must not be bindable to the OP sheet from a
        // persistent document either.
        ResultViewer::Op => {
            analysis.analysis_type == AnalysisType::DcOp
                && operating_point_evidence_is_renderable(analysis)
        }
        // Two families rank contributions to one number, and this sheet
        // serves both: a sensitivity report and a DC mismatch spread.
        ResultViewer::Contribution => {
            matches!(
                analysis.result_payload,
                Some(
                    AnalysisResultPayload::Sensitivity { .. }
                        | AnalysisResultPayload::SensitivityStudy { .. }
                        | AnalysisResultPayload::DcMismatch { .. }
                )
            )
        }
        ResultViewer::TransferFunction => {
            matches!(
                analysis.result_payload,
                Some(AnalysisResultPayload::TransferFunction { .. })
            )
        }
        ResultViewer::Specs => {
            !analysis.measurements.is_empty()
                || matches!(
                    analysis.result_payload,
                    Some(AnalysisResultPayload::ScalarMeasurements { .. })
                )
        }
        ResultViewer::Table => {
            !analysis.waveforms.is_empty()
                || operating_point_evidence_is_renderable(analysis)
                || matches!(
                    analysis.result_payload,
                    Some(
                        AnalysisResultPayload::PssFloquet { .. }
                            | AnalysisResultPayload::Pstb { .. }
                    )
                )
        }
        ResultViewer::Hist => matches!(
            analysis.family_metadata,
            Some(AnalysisResultFamilyMetadata::MonteCarlo { ref variables, .. })
                if variables.iter().any(|variable| !variable.samples.is_empty())
        ),
        ResultViewer::PoleZero => {
            matches!(
                analysis.result_payload,
                Some(AnalysisResultPayload::PoleZero { .. })
            )
        }
        ResultViewer::Events => matches!(
            analysis.result_payload,
            Some(AnalysisResultPayload::TransientEvents {
                ref digital_traces,
                ref real_traces,
                ref current_impulses,
                ..
            }) if !digital_traces.is_empty() || !real_traces.is_empty() || current_impulses.is_some()
        ),
        ResultViewer::Soa => matches!(
            analysis.result_payload,
            Some(AnalysisResultPayload::Soa { ref evaluations, .. }) if !evaluations.is_empty()
        ),
        ResultViewer::Optimization => matches!(
            analysis.family_metadata,
            Some(AnalysisResultFamilyMetadata::Optimization { ref iterations, .. }) if !iterations.is_empty()
        ),
        ResultViewer::Manifest => false,
    }
}
