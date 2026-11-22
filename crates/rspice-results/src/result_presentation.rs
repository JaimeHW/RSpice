//! Durable presentation identities and annotations for retained results.
//!
//! Shared by project I/O and the workbench. Rendering, session caches, and
//! editor state remain in the workbench; none can redefine this wire contract.

use serde::{Deserialize, Serialize};

use crate::analysis_type::AnalysisType;
#[cfg(feature = "engine-evidence")]
use crate::{analysis_result::AnalysisResult, run::SimulationRun, waveform::RetainedWaveform};
use rspice_vocabulary::product::{AnalysisInstanceId, DatasetId};

/// Whether a result viewer renders one waveform pane stack per analysis.
pub const fn viewer_uses_wave_stack(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    )
}

/// The result viewers, in tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ResultViewer {
    /// Stacked waveform strips, one per analysis.
    #[default]
    Waves,
    /// Swept-source or swept-parameter DC transfer curves.
    DcSweep,
    /// Loop-gain stability view with margin markers.
    Bode,
    /// Spectrum with harmonic markers.
    Fft,
    /// Retained complex harmonic-balance coefficient spectrum.
    HarmonicBalance,
    /// Periodic phase-noise spectrum versus offset frequency.
    PhaseNoise,
    /// Eye diagram with compliance mask.
    Eye,
    /// Monte-Carlo distribution.
    Hist,
    /// Per-device operating-point inspector (Spectre-style OP info).
    Op,
    /// Ordinary-noise root spectral density with contributor evidence.
    NoiseContrib,
    /// Ranked signed parameter-sensitivity contributions.
    Contribution,
    /// Scalar DC transfer gain and input/output resistances.
    TransferFunction,
    /// The retained samples of one analysis, as rows.
    Table,
    /// Nyquist loop-gain stability surface.
    Nyquist,
    /// A periodic orbit's Floquet multipliers on the unit circle, beside
    /// their magnitudes, exponents, verdicts and margins.
    Floquet,
    /// A network's S-parameters, and the two-port figures read from them,
    /// against frequency on rectangular axes.
    SParameters,
    /// Smith-chart RF/network surface.
    Smith,
    /// One retained complex response on the polar plane.
    Polar,
    /// Complex-plane pole-zero surface.
    PoleZero,
    /// Two measured Monte-Carlo columns read as a correlation.
    Scatter,
    /// The shape of a retained population against its requirement.
    BoxViolin,
    /// Committed XSPICE digital and real-valued event history.
    Events,
    /// Safe-operating-area rule evidence with per-rule stress history.
    Soa,
    /// Optimizer cost convergence and the candidate history behind it.
    Optimization,
    /// A measured value of every point of a sweep, corner run or temperature
    /// step, against a swept quantity.
    Sweep,
    /// Exact single-ended and mixed-mode power-wave matrices.
    NetworkMatrix,
}

impl ResultViewer {
    /// Compact command/status label.
    pub fn label(self) -> &'static str {
        match self {
            ResultViewer::Waves => "WAVES",
            ResultViewer::DcSweep => "DC",
            ResultViewer::Bode => "BODE",
            ResultViewer::Fft => "FFT",
            ResultViewer::HarmonicBalance => "HB",
            ResultViewer::PhaseNoise => "PNOISE",
            ResultViewer::Eye => "EYE",
            ResultViewer::Hist => "HIST",
            ResultViewer::Op => "OP",
            ResultViewer::NoiseContrib => "NOISE",
            ResultViewer::Contribution => "SENS",
            ResultViewer::TransferFunction => "XF",
            ResultViewer::Table => "TABLE",
            ResultViewer::Nyquist => "NYQ",
            ResultViewer::Floquet => "FLOQUET",
            ResultViewer::SParameters => "SPARAM",
            ResultViewer::Smith => "SMITH",
            ResultViewer::Polar => "POLAR",
            ResultViewer::NetworkMatrix => "NETWORK",
            ResultViewer::PoleZero => "PZ",
            ResultViewer::Scatter => "SCATTER",
            ResultViewer::BoxViolin => "DIST",
            ResultViewer::Events => "EVENTS",
            ResultViewer::Soa => "SOA",
            ResultViewer::Optimization => "OPT",
            ResultViewer::Sweep => "SWEEP",
        }
    }

    const PRIMARY: [ResultViewer; 26] = [
        ResultViewer::Waves,
        ResultViewer::DcSweep,
        ResultViewer::Bode,
        ResultViewer::NoiseContrib,
        ResultViewer::Nyquist,
        ResultViewer::Fft,
        ResultViewer::HarmonicBalance,
        ResultViewer::PhaseNoise,
        ResultViewer::SParameters,
        ResultViewer::Smith,
        ResultViewer::Polar,
        ResultViewer::NetworkMatrix,
        ResultViewer::TransferFunction,
        ResultViewer::Contribution,
        ResultViewer::Op,
        ResultViewer::Table,
        ResultViewer::Hist,
        ResultViewer::Scatter,
        ResultViewer::BoxViolin,
        ResultViewer::Eye,
        ResultViewer::PoleZero,
        ResultViewer::Floquet,
        // Specialist sheets last: each needs evidence an ordinary run does not
        // produce — XSPICE event nodes, or a whole campaign analysis kind — so
        // leading with them would push the everyday sheets rightward for the
        // sake of tabs that are usually dim.
        ResultViewer::Sweep,
        ResultViewer::Events,
        ResultViewer::Soa,
        ResultViewer::Optimization,
    ];
    /// Every result presentation, in the established tab order.
    pub fn all() -> impl Iterator<Item = ResultViewer> {
        Self::PRIMARY.into_iter()
    }

    /// Human-readable document-tab label from the upgraded Results mockup.
    pub const fn tab_label(self) -> &'static str {
        match self {
            ResultViewer::Waves => "Waves",
            ResultViewer::DcSweep => "DC Sweep",
            ResultViewer::Bode => "Bode",
            ResultViewer::Fft => "FFT",
            ResultViewer::HarmonicBalance => "Harmonics",
            ResultViewer::PhaseNoise => "Phase Noise",
            ResultViewer::Eye => "Eye",
            ResultViewer::Hist => "Histogram",
            ResultViewer::Op => "OP",
            ResultViewer::NoiseContrib => "Noise",
            ResultViewer::Contribution => "Sensitivity",
            ResultViewer::TransferFunction => "Transfer Function",
            ResultViewer::Table => "Table",
            ResultViewer::Nyquist => "Nyquist",
            ResultViewer::Floquet => "Floquet",
            ResultViewer::SParameters => "S-Parameters",
            ResultViewer::Smith => "Smith",
            ResultViewer::Polar => "Polar",
            ResultViewer::NetworkMatrix => "Network Matrix",
            ResultViewer::PoleZero => "PZ",
            ResultViewer::Scatter => "Scatter",
            ResultViewer::BoxViolin => "Distribution",
            ResultViewer::Events => "Events",
            ResultViewer::Soa => "SOA",
            ResultViewer::Optimization => "Optimization",
            ResultViewer::Sweep => "Sweep",
        }
    }

    /// The catalog viewer document this sheet renders.
    ///
    /// One owner on purpose. Three copies of this map existed — in the
    /// persistent-document layer, the Studio, and the workbench state — and
    /// they disagreed about the noise sheet: two said `viewer-bode`, the
    /// Studio's own said `viewer-spectrum`, which the catalog rejects for a
    /// `noise` analysis. Whether a noise sheet could be pinned into a
    /// document depended on which path asked.
    pub const fn viewer_document_id(self) -> &'static str {
        match self {
            ResultViewer::Waves | ResultViewer::DcSweep | ResultViewer::Sweep => "viewer-waveform",
            ResultViewer::Bode | ResultViewer::Nyquist | ResultViewer::NoiseContrib => {
                "viewer-bode"
            }
            ResultViewer::Fft | ResultViewer::HarmonicBalance => "viewer-spectrum",
            ResultViewer::PhaseNoise => "viewer-phase-noise",
            ResultViewer::Eye => "eye-viewer",
            ResultViewer::Hist => "viewer-histogram",
            ResultViewer::Op | ResultViewer::Table => "viewer-table",
            ResultViewer::Contribution => "viewer-contribution",
            ResultViewer::TransferFunction => "viewer-transfer-function",
            ResultViewer::SParameters => "viewer-s-parameters",
            ResultViewer::Smith => "viewer-smith",
            ResultViewer::Polar => "viewer-polar",
            ResultViewer::NetworkMatrix => "viewer-mixed-mode-network",
            ResultViewer::PoleZero => "viewer-pz",
            ResultViewer::Floquet => "viewer-floquet",
            ResultViewer::Scatter => "viewer-scatter",
            ResultViewer::BoxViolin => "viewer-box-violin",
            ResultViewer::Events => "viewer-digital-events",
            ResultViewer::Soa => "viewer-soa",
            ResultViewer::Optimization => "viewer-optimization",
        }
    }

    /// The sheet that draws a retained pane bound to this viewer document.
    ///
    /// The inverse of [`Self::viewer_document_id`] is not one-to-one — two
    /// sheets render `viewer-table` — so the choice is made once, here. Table
    /// is the answer, because `renderer_supports_analysis` admits a
    /// `viewer-table` pane on retained waveforms, which is what Table reads.
    pub fn from_viewer_document_id(id: &str) -> Option<Self> {
        Some(match id {
            "viewer-waveform" => ResultViewer::Waves,
            "viewer-bode" => ResultViewer::Bode,
            "viewer-spectrum" => ResultViewer::Fft,
            "viewer-phase-noise" => ResultViewer::PhaseNoise,
            "viewer-s-parameters" => ResultViewer::SParameters,
            "viewer-smith" => ResultViewer::Smith,
            "viewer-polar" => ResultViewer::Polar,
            "viewer-mixed-mode-network" => ResultViewer::NetworkMatrix,
            "viewer-table" => ResultViewer::Table,
            "viewer-histogram" => ResultViewer::Hist,
            "viewer-scatter" => ResultViewer::Scatter,
            "viewer-box-violin" => ResultViewer::BoxViolin,
            "eye-viewer" => ResultViewer::Eye,
            "viewer-pz" => ResultViewer::PoleZero,
            "viewer-floquet" => ResultViewer::Floquet,
            "viewer-contribution" => ResultViewer::Contribution,
            "viewer-transfer-function" => ResultViewer::TransferFunction,
            "viewer-digital-events" => ResultViewer::Events,
            "viewer-soa" => ResultViewer::Soa,
            "viewer-optimization" => ResultViewer::Optimization,
            _ => return None,
        })
    }

    /// The sheets an analysis of `kind` opens on, best first. The first is
    /// its default sheet; the rest stand in, in order, when a run's evidence
    /// cannot fill it (a transient that kept only event nodes has no curve
    /// for Waves, so it opens on Events).
    ///
    /// The one table of default sheets: a run opening, a finished run and an
    /// imported run all choose from it, so no caller restates the mapping.
    pub const fn default_sheets(kind: AnalysisType) -> &'static [ResultViewer] {
        use AnalysisType as A;
        use ResultViewer as V;
        match kind {
            A::DcOp => &[V::Op, V::Table],
            A::DcSweep => &[V::DcSweep, V::Table],
            A::Ac | A::Disto | A::Pac | A::Pxf | A::Qpac | A::Qpxf => &[V::Bode, V::Table],
            A::Stb => &[V::Bode, V::Nyquist, V::Table],
            // A periodic stability result is indexed by Floquet mode, not by
            // frequency: its multipliers are drawn on the unit circle.
            A::Pstb => &[V::Floquet, V::Table],
            A::Transient | A::TransientNoise | A::Envelope => &[V::Waves, V::Events, V::Table],
            // A steady state opens on its waveforms; its stability, where
            // the run computed it, is a sheet beside them.
            A::Pss => &[V::Waves, V::Floquet, V::Events, V::Table],
            A::Soa => &[V::Soa, V::Waves, V::Table],
            A::Noise | A::Hbnoise | A::Qpnoise => &[V::NoiseContrib, V::PhaseNoise, V::Table],
            A::Pnoise => &[V::PhaseNoise, V::NoiseContrib, V::Table],
            A::PoleZero => &[V::PoleZero, V::Table],
            A::Tf => &[V::TransferFunction],
            A::Sensitivity | A::DcMismatch => &[V::Contribution, V::Table],
            A::MonteCarlo => &[
                V::Hist,
                V::BoxViolin,
                V::Scatter,
                V::Waves,
                V::Bode,
                V::Table,
            ],
            // A sweep held in one analysis keeps one value per point: what it
            // measured is read against what was swept.
            A::Parametric | A::Corner => &[V::Sweep, V::Table],
            A::Optimization => &[V::Optimization, V::Table],
            A::SParameter | A::Hbsp | A::Psp => &[
                V::SParameters,
                V::Smith,
                V::NetworkMatrix,
                V::Polar,
                V::Table,
            ],
            A::Fourier | A::HarmonicBalance | A::Qpss => &[V::HarmonicBalance, V::Waves, V::Table],
        }
    }

    /// The sheet an analysis of `kind` opens on.
    pub const fn default_sheet(kind: AnalysisType) -> ResultViewer {
        Self::default_sheets(kind)[0]
    }

    /// The sheet an analysis of `kind` draws on while its run is still
    /// writing: the waveform stack its samples stream into. An analysis whose
    /// results arrive only when it ends has none.
    pub const fn live_sheet(kind: AnalysisType) -> Option<ResultViewer> {
        if kind.is_time_domain() {
            Some(ResultViewer::Waves)
        } else if matches!(kind, AnalysisType::DcSweep) {
            Some(ResultViewer::DcSweep)
        } else if kind.is_bode_response() {
            Some(ResultViewer::Bode)
        } else if matches!(
            kind,
            AnalysisType::Noise | AnalysisType::Hbnoise | AnalysisType::Qpnoise
        ) {
            Some(ResultViewer::NoiseContrib)
        } else {
            None
        }
    }
}

/// The analysis a run shows when nothing has chosen one, from the kinds of
/// its analyses in run order.
///
/// A run opens where its curves are: on its first analysis whose default
/// sheet is a waveform stack that analysis `fills` (asked with the
/// analysis' index), as ADE plots a finished run's waveforms. A run with no
/// curves opens on its first analysis.
pub fn opening_analysis(
    kinds: &[AnalysisType],
    mut fills: impl FnMut(usize, ResultViewer) -> bool,
) -> Option<usize> {
    kinds
        .iter()
        .enumerate()
        .find(|(index, kind)| {
            let sheet = ResultViewer::default_sheet(**kind);
            viewer_uses_wave_stack(sheet) && fills(*index, sheet)
        })
        .map(|(index, _)| index)
        .or_else(|| (!kinds.is_empty()).then_some(0))
}

/// The sheet the shown analysis opens on: the first of its default sheets
/// that is `available`, else the first available sheet in tab order. The
/// tabs offer only the shown analysis' views, so nothing here can open a
/// sheet of another analysis.
pub fn opening_sheet(
    shown: Option<AnalysisType>,
    mut available: impl FnMut(ResultViewer) -> bool,
) -> Option<ResultViewer> {
    shown
        .into_iter()
        .flat_map(|kind| ResultViewer::default_sheets(kind).iter().copied())
        .find(|sheet| available(*sheet))
        .or_else(|| ResultViewer::all().find(|sheet| available(*sheet)))
}

/// Stable identity of one retained analysis within one immutable dataset.
///
/// Current results use the exact prepared-task identity. A legacy result has
/// no such provenance, so its run-local analysis id is safe only when paired
/// with the immutable dataset id. Neither representation depends on vector
/// position, which prevents presentation state from moving to another
/// analysis when retained results are reordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnalysisPresentationKey {
    dataset_id: DatasetId,
    source: AnalysisPresentationSource,
}

/// Which authored analysis a result came from, independent of the dataset it
/// was solved into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisPresentationSource {
    Prepared(AnalysisInstanceId),
    Legacy(u64),
}

impl AnalysisPresentationKey {
    #[cfg(feature = "engine-evidence")]
    pub fn new<W: AsRef<RetainedWaveform>>(
        dataset_id: DatasetId,
        analysis: &AnalysisResult<W>,
    ) -> Self {
        let source = analysis.provenance().map_or(
            AnalysisPresentationSource::Legacy(analysis.id),
            |provenance| AnalysisPresentationSource::Prepared(provenance.source_instance_id()),
        );
        Self { dataset_id, source }
    }

    pub const fn dataset_id(self) -> DatasetId {
        self.dataset_id
    }

    /// The authored analysis this key names, without the dataset one run of
    /// it produced.
    ///
    /// Presentation decisions the reader makes about "the transient" are
    /// about the analysis, not about the one solve of it that happened to be
    /// on screen when they made them.
    pub const fn authored(self) -> AnalysisPresentationSource {
        self.source
    }

    /// Analysis identity used by retained visualization-document bindings.
    pub fn retained_instance_id(self) -> AnalysisInstanceId {
        match self.source {
            AnalysisPresentationSource::Prepared(id) => id,
            AnalysisPresentationSource::Legacy(id) => AnalysisInstanceId::from_namespace(
                self.dataset_id.as_uuid(),
                format!("legacy-analysis-v1/{id}").as_bytes(),
            ),
        }
    }

    pub fn order_key(self) -> (uuid::Uuid, u8, [u8; 16]) {
        let (kind, source) = match self.source {
            AnalysisPresentationSource::Prepared(id) => (0, *id.as_uuid().as_bytes()),
            AnalysisPresentationSource::Legacy(id) => {
                let mut bytes = [0_u8; 16];
                bytes[8..].copy_from_slice(&id.to_be_bytes());
                (1, bytes)
            }
        };
        (self.dataset_id.as_uuid(), kind, source)
    }

    #[cfg(feature = "engine-evidence")]
    pub fn resolve<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
        self,
        run: &SimulationRun<A>,
    ) -> Option<(usize, &A)> {
        (run.dataset_id == self.dataset_id)
            .then(|| {
                run.analyses
                    .iter()
                    .enumerate()
                    .find(|(_, analysis)| Self::new(run.dataset_id, analysis.as_ref()) == self)
            })
            .flatten()
    }
}

/// Stable identity of one source waveform representation inside an analysis.
///
/// `source_name` follows the retained waveform through reordering. `kind` and
/// `family_group` distinguish real/imaginary, magnitude/phase, and projected
/// family traces that intentionally share the same source waveform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TracePresentationKey {
    pub source_name: String,
    pub kind: u8,
    pub family_group: u64,
}

/// Fully dataset-bound identity of one presented waveform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WaveformPresentationKey {
    pub analysis: AnalysisPresentationKey,
    pub trace: TracePresentationKey,
}

/// Stable identity of one retained source waveform whose quick-view
/// visibility has been overridden for this session.
///
/// The solver-owned [`RetainedTrace`](crate::waveform::RetainedTrace) remains
/// an immutable result. A visibility click records presentation state against
/// the dataset, authored analysis, and source name instead of rewriting the
/// retained run (or its legacy live projection).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceWaveformPresentationKey {
    analysis: AnalysisPresentationKey,
    /// Shared by the key's copies: the reader's ticks, the signals they
    /// set and the edits they record each hold one, and copy no name.
    source_name: std::sync::Arc<str>,
}

impl SourceWaveformPresentationKey {
    pub fn new(analysis: AnalysisPresentationKey, source_name: impl AsRef<str>) -> Self {
        Self {
            analysis,
            source_name: source_name.as_ref().into(),
        }
    }

    #[cfg(feature = "engine-evidence")]
    pub fn resolve<'a, R, A, W>(&self, runs: &'a [R]) -> Option<(usize, usize, usize, &'a W)>
    where
        R: AsRef<SimulationRun<A>>,
        A: AsRef<AnalysisResult<W>> + 'a,
        W: AsRef<RetainedWaveform> + 'a,
    {
        let (run_index, run) = runs
            .iter()
            .enumerate()
            .find(|(_, run)| run.as_ref().dataset_id == self.analysis.dataset_id())?;
        let (analysis_index, analysis) = self.analysis.resolve(run.as_ref())?;
        let mut matches = analysis
            .as_ref()
            .waveforms
            .iter()
            .enumerate()
            .filter(|(_, waveform)| *waveform.as_ref().name == *self.source_name);
        let (waveform_index, waveform) = matches.next()?;
        matches
            .next()
            .is_none()
            .then_some((run_index, analysis_index, waveform_index, waveform))
    }

    pub const fn analysis(&self) -> AnalysisPresentationKey {
        self.analysis
    }

    /// The retained waveform's name.
    pub fn source_name(&self) -> &str {
        &self.source_name
    }
}

/// Stable identity of a non-waveform quantity retained by one immutable
/// analysis: scalar evidence, an exact array, an event stream, a contribution
/// table, or family metadata. `canonical_name` is producer-authored inventory
/// identity, not a translated display label.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultArtifactPresentationKey {
    analysis: AnalysisPresentationKey,
    canonical_name: String,
}

impl ResultArtifactPresentationKey {
    pub fn new(analysis: AnalysisPresentationKey, canonical_name: impl Into<String>) -> Self {
        Self {
            analysis,
            canonical_name: canonical_name.into(),
        }
    }

    pub fn analysis(&self) -> AnalysisPresentationKey {
        self.analysis
    }

    pub fn canonical_name(&self) -> &str {
        &self.canonical_name
    }

    #[cfg(feature = "engine-evidence")]
    pub fn resolve<'a, R, A, W>(&self, runs: &'a [R]) -> Option<(usize, usize, &'a A)>
    where
        R: AsRef<SimulationRun<A>>,
        A: AsRef<AnalysisResult<W>> + 'a,
        W: AsRef<RetainedWaveform> + 'a,
    {
        let (run_index, run) = runs
            .iter()
            .enumerate()
            .find(|(_, run)| run.as_ref().dataset_id == self.analysis.dataset_id())?;
        let (analysis_index, analysis) = self.analysis.resolve(run.as_ref())?;
        Some((run_index, analysis_index, analysis))
    }
}

/// What a result marker asserts, and therefore how it draws.
///
/// A peak, valley or crossing marker is a search, not a label: it rides the
/// feature of its trace nearest where it was placed and finds that feature
/// again whenever the curve under it changes, as ViVA's peak and crossing
/// markers do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MarkerKind {
    /// A freeform annotation on a sample.
    #[default]
    Note,
    /// The local maximum the trace climbs to from where the marker was put.
    Peak,
    /// The local minimum the trace descends to from where the marker was put.
    Valley,
    /// The crossing of a level nearest where the marker was put.
    Crossing(CrossingLevel),
    /// A limit the design is measured against. Drawn as a limit line,
    /// because a spec constrains the axis position, not one curve.
    Spec,
    /// A vertical marker: a line at its X, reading every trace it crosses.
    Vertical,
    /// The crossing, on the trace another marker rides, of that marker's Y
    /// plus a typed amount: 3 dB below a peak finds a corner. It follows its
    /// marker as the curve changes.
    Offset(MarkerOffset),
}

/// Which marker an offset marker is measured from, by its number, and by how
/// much in its trace's unit; the amount compared bit for bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkerOffset {
    pub from: u64,
    pub by: CrossingLevel,
}

/// The level a crossing marker finds, compared bit for bit so a marker kind
/// stays comparable.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CrossingLevel(pub f64);

impl PartialEq for CrossingLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for CrossingLevel {}

impl MarkerKind {
    /// Every kind a marker may be given. A crossing's level is the trace's
    /// value where the marker is when the kind is chosen; the level here only
    /// names the choice.
    pub const ALL: [MarkerKind; 6] = [
        MarkerKind::Note,
        MarkerKind::Peak,
        MarkerKind::Valley,
        MarkerKind::Crossing(CrossingLevel(0.0)),
        MarkerKind::Vertical,
        MarkerKind::Spec,
    ];

    /// Short label used on the chip and in the marker list.
    pub const fn label(self) -> &'static str {
        match self {
            MarkerKind::Note => "note",
            MarkerKind::Peak => "peak",
            MarkerKind::Valley => "valley",
            MarkerKind::Crossing(_) => "crossing",
            MarkerKind::Spec => "spec",
            MarkerKind::Vertical => "vertical",
            MarkerKind::Offset(_) => "offset",
        }
    }

    /// What choosing this kind asserts, spelled out in the edit dialog.
    pub const fn dialog_label(self) -> &'static str {
        match self {
            MarkerKind::Note => "Note — a remark about this point on the curve",
            MarkerKind::Peak => {
                "Peak — the local maximum it sits on, found again as the curve changes"
            }
            MarkerKind::Valley => {
                "Valley — the local minimum it sits on, found again as the curve changes"
            }
            MarkerKind::Crossing(_) => {
                "Crossing — where the curve crosses its level, found again as the curve changes"
            }
            MarkerKind::Spec => "Spec — a limit line the design is measured against",
            MarkerKind::Vertical => "Vertical — a line at this X, reading every trace it crosses",
            MarkerKind::Offset(_) => {
                "Offset — a typed amount from another marker's level, found again as it moves"
            }
        }
    }

    /// Whether `other` is the same choice, whatever level a crossing holds.
    pub const fn same_choice(self, other: MarkerKind) -> bool {
        matches!(
            (self, other),
            (MarkerKind::Note, MarkerKind::Note)
                | (MarkerKind::Peak, MarkerKind::Peak)
                | (MarkerKind::Valley, MarkerKind::Valley)
                | (MarkerKind::Crossing(_), MarkerKind::Crossing(_))
                | (MarkerKind::Spec, MarkerKind::Spec)
                | (MarkerKind::Vertical, MarkerKind::Vertical)
                | (MarkerKind::Offset(_), MarkerKind::Offset(_))
        )
    }

    /// The feature a search marker finds, or `None` for a marker that stays
    /// where it was put. An offset marker's level is its marker's, found
    /// where that one stands, so it has none of its own.
    pub const fn search(self) -> Option<crate::feature_search::Anchor> {
        use crate::feature_search::Anchor;
        match self {
            MarkerKind::Peak => Some(Anchor::Peak),
            MarkerKind::Valley => Some(Anchor::Valley),
            MarkerKind::Crossing(level) => Some(Anchor::Crossing { level: level.0 }),
            MarkerKind::Note | MarkerKind::Spec | MarkerKind::Vertical | MarkerKind::Offset(_) => {
                None
            }
        }
    }

    /// A spec or vertical marker stands at an X alone, drawn across every
    /// pane, rather than on one curve.
    pub const fn rides_a_trace(self) -> bool {
        !matches!(self, MarkerKind::Spec | MarkerKind::Vertical)
    }
}

/// Stable identity of one unit-scoped waveform pane.
///
/// The analysis key retains the exact dataset identity; the unit is the pane
/// grouping contract and remains independent of transient pane ordinals.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WavePanePresentationKey {
    pub analysis: AnalysisPresentationKey,
    pub unit: String,
}

/// One user expression trace on a waves strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExprTrace {
    /// The calculator expression as typed ("V(out)/V(in)").
    pub text: String,
    #[serde(
        default,
        skip_serializing_if = "crate::saved_output::ComplexExpressionPolicy::is_legacy"
    )]
    pub complex_policy: crate::saved_output::ComplexExpressionPolicy,
    /// Legend-chip visibility.
    #[serde(default = "default_visible")]
    pub visible: bool,
}

fn default_visible() -> bool {
    true
}

/// A named set of signals the reader saved, to put back on the plot in one
/// step. Members are named rather than bound to one run, so a set outlives
/// the run it was saved from and applies to whichever run is active.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalSet {
    pub name: String,
    pub members: Vec<SignalSetMember>,
}

/// One signal of a [`SignalSet`]: the kind of analysis it was saved from,
/// since `V(out)` of a transient and of an AC analysis are different signals,
/// and the name the browser lists it under.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignalSetMember {
    pub analysis: crate::analysis_type::AnalysisType,
    /// A retained waveform's name, or a typed artifact's canonical name.
    pub name: String,
    /// Whether `name` is an artifact's canonical name (an event net's or a
    /// bus's row) rather than a waveform's.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub artifact: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `viewer_document_id` and `from_viewer_document_id` are one map read in
    /// two directions, so every answer either gives must agree with the other.
    /// Three sheets share `viewer-table`, which is exactly where the second
    /// copy of the inverse had drifted onto a different one.
    #[test]
    fn the_viewer_document_map_agrees_with_itself_in_both_directions() {
        use crate::viewer_catalog::VIEWER_DOCUMENTS;

        for viewer in ResultViewer::all() {
            let document_id = viewer.viewer_document_id();
            assert!(
                VIEWER_DOCUMENTS
                    .iter()
                    .any(|document| document.id == document_id),
                "{viewer:?} claims {document_id}, which the catalog does not publish"
            );
            let drawn_by = ResultViewer::from_viewer_document_id(document_id)
                .unwrap_or_else(|| panic!("{document_id} has no sheet"));
            assert_eq!(
                drawn_by.viewer_document_id(),
                document_id,
                "{document_id} resolves to {drawn_by:?}, which renders something else"
            );
        }
    }

    #[test]
    fn analysis_identities_and_expression_traces_keep_their_wire_form() {
        let legacy = serde_json::json!({
            "dataset_id": "b3c6b2be-c997-4f5d-a06e-714071283df5",
            "source": {"Legacy": 7}
        });
        let identity: AnalysisPresentationKey = serde_json::from_value(legacy.clone()).unwrap();
        assert_eq!(identity.authored(), AnalysisPresentationSource::Legacy(7));
        assert_eq!(serde_json::to_value(identity).unwrap(), legacy);
        let prepared = serde_json::json!({
            "dataset_id": legacy["dataset_id"],
            "source": {"Prepared": "ab428908-f301-423c-bdb7-4cb4b1feef6f"}
        });
        let identity: AnalysisPresentationKey = serde_json::from_value(prepared.clone()).unwrap();
        assert_eq!(serde_json::to_value(identity).unwrap(), prepared);
        let trace: ExprTrace =
            serde_json::from_value(serde_json::json!({"text": "V(out)*2"})).unwrap();
        assert!(trace.visible, "a trace written without visibility is shown");
    }

    const EVERY_KIND: [AnalysisType; 32] = [
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
        AnalysisType::Optimization,
        AnalysisType::Soa,
        AnalysisType::SParameter,
        AnalysisType::Envelope,
        AnalysisType::Fourier,
        AnalysisType::HarmonicBalance,
        AnalysisType::Pss,
        AnalysisType::Qpss,
        AnalysisType::Hbsp,
        AnalysisType::Hbnoise,
        AnalysisType::Psp,
        AnalysisType::Qpac,
        AnalysisType::Qpnoise,
        AnalysisType::Qpxf,
        AnalysisType::TransientNoise,
    ];

    /// Each kind names its sheets once, and the sheet a kind streams into
    /// while its run writes is one of the sheets it opens on, so a run never
    /// leaves the sheet it was drawing on for one it was never offered.
    #[test]
    fn every_analysis_kind_states_its_sheets_and_streams_into_one_of_them() {
        for kind in EVERY_KIND.into_iter().chain([AnalysisType::DcMismatch]) {
            let sheets = ResultViewer::default_sheets(kind);
            assert!(!sheets.is_empty(), "{kind:?}");
            for (index, sheet) in sheets.iter().enumerate() {
                assert!(
                    !sheets[..index].contains(sheet),
                    "{kind:?} names {sheet:?} twice"
                );
            }
            if let Some(live) = ResultViewer::live_sheet(kind) {
                assert!(viewer_uses_wave_stack(live), "{kind:?}");
                assert!(sheets.contains(&live), "{kind:?} streams into {live:?}");
            }
        }
    }

    #[test]
    fn a_run_opens_on_its_first_curves_else_its_first_analysis() {
        let everything = |_, _| true;
        // An operating point ahead of a transient: the transient's curves.
        assert_eq!(
            opening_analysis(&[AnalysisType::DcOp, AnalysisType::Transient], everything),
            Some(1)
        );
        // An AC sweep ahead of a transient: the first curves are the sweep's.
        assert_eq!(
            opening_analysis(&[AnalysisType::Ac, AnalysisType::Transient], everything),
            Some(0)
        );
        // A transient with no curves to fill its stack gives way to the next.
        assert_eq!(
            opening_analysis(
                &[AnalysisType::Transient, AnalysisType::Ac],
                |index, _| index == 1
            ),
            Some(1)
        );
        // No curves at all: the first analysis.
        assert_eq!(
            opening_analysis(&[AnalysisType::DcOp, AnalysisType::Sensitivity], everything),
            Some(0)
        );
        assert_eq!(opening_analysis(&[], everything), None);
    }

    #[test]
    fn the_shown_analysis_opens_on_its_default_sheet_else_any_it_fills() {
        let everything = |_| true;
        assert_eq!(
            opening_sheet(Some(AnalysisType::Sensitivity), everything),
            Some(ResultViewer::Contribution)
        );
        assert_eq!(
            opening_sheet(Some(AnalysisType::Soa), everything),
            Some(ResultViewer::Soa)
        );
        // A transient that kept only its event nodes opens on Events.
        assert_eq!(
            opening_sheet(Some(AnalysisType::Transient), |sheet| sheet
                == ResultViewer::Events),
            Some(ResultViewer::Events)
        );
        // Nothing the table names can be filled: the first sheet that can.
        assert_eq!(
            opening_sheet(Some(AnalysisType::Tf), |sheet| sheet == ResultViewer::Table),
            Some(ResultViewer::Table)
        );
        assert_eq!(opening_sheet(None, |_| false), None);
    }
}
