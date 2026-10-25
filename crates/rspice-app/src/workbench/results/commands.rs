//! The commands the results feature declares, as the registry lists them:
//! importing, exporting and clearing result datasets, the result viewers,
//! the result document's forms and tools, and the hops back out of a run.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use egui::Key;
use rspice_results::result_presentation::ResultViewer;

use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};
use crate::workbench::localization::OPEN_TASK_DECK;

/// One command of the results feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultsCommand {
    ImportResultDataset,
    ExportWaveformsCsv,
    ClearResults,
    ToggleLinkedCursors,
    ToggleResultsSplit,
    DatasetManifestBrowser,
    /// Activate Results on the selected run's retained dataset. The affordances
    /// that hop into Results — the completion notice, the jobs history, the
    /// engine chip, the plan heading — select the run first and then route
    /// through this one command, so the palette can reach the same place.
    OpenRunInResults,
    /// Return from a retained dataset to the simulation plan that produced it,
    /// selecting the producing analysis instance on the Analyses page.
    OpenProducingPlan,
    /// Open the source the selected run's engine was actually handed. The Jobs
    /// manager row and the Netlist run strip both route here; this is the
    /// third way in, and the only one reachable without first finding the row
    /// that offers it.
    OpenTaskDeck,
    /// Narrow the console to what one check-marked Data Browser quantity's
    /// producer wrote. The browser's context menus offer the same hop on the
    /// row they were opened on.
    RevealProducerLog,
    CreateResultDocument,
    WaveformCalculator,
    ExpressionDiagnostics,
    CompareResultDatasets,
    MeasurementLibrary,
    FamilySlicing,
    ResultViewer(ResultViewer),
    EditSpecifications,
    AddVisualizationPane,
    VisualizationDocumentProperties,
}

const NONE: &[ShortcutBinding] = &[];

const fn primary(
    key: Key,
    primary: bool,
    alt: bool,
    shift: bool,
    label: &'static str,
) -> ShortcutBinding {
    ShortcutBinding {
        chord: ShortcutChord::new(key, primary, alt, shift, label),
        platforms: &CommandPlatform::ALL,
        kind: ShortcutKind::Primary,
    }
}

const TOGGLE_RESULTS_SPLIT: &[ShortcutBinding] =
    &[primary(Key::Backslash, true, false, false, "Ctrl+\\")];
const LINKED_CURSORS: &[ShortcutBinding] = &[primary(Key::C, false, false, true, "Shift+C")];

/// A command listed in the palette that runs with or without a project.
const ANYWHERE: CommandFlags = CommandFlags::PALETTE_VISIBLE;
/// A command that needs an open project.
const PROJECT: CommandFlags = ANYWHERE.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
/// A command that needs an open project and waits for its operation to end.
const PROJECT_TRANSACTION: CommandFlags =
    PROJECT.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);

impl ResultsCommand {
    /// Every command of the feature.
    pub(crate) const ALL: [Self; 44] = [
        Self::ImportResultDataset,
        Self::ExportWaveformsCsv,
        Self::ResultViewer(ResultViewer::Waves),
        Self::ToggleResultsSplit,
        Self::ClearResults,
        Self::ToggleLinkedCursors,
        Self::DatasetManifestBrowser,
        Self::OpenRunInResults,
        Self::OpenProducingPlan,
        Self::OpenTaskDeck,
        Self::RevealProducerLog,
        Self::CreateResultDocument,
        Self::WaveformCalculator,
        Self::ExpressionDiagnostics,
        Self::CompareResultDatasets,
        Self::MeasurementLibrary,
        Self::FamilySlicing,
        Self::ResultViewer(ResultViewer::Bode),
        Self::ResultViewer(ResultViewer::DcSweep),
        Self::ResultViewer(ResultViewer::Fft),
        Self::ResultViewer(ResultViewer::HarmonicBalance),
        Self::ResultViewer(ResultViewer::PhaseNoise),
        Self::ResultViewer(ResultViewer::Eye),
        Self::ResultViewer(ResultViewer::Hist),
        Self::ResultViewer(ResultViewer::Op),
        Self::ResultViewer(ResultViewer::NoiseContrib),
        Self::ResultViewer(ResultViewer::Contribution),
        Self::ResultViewer(ResultViewer::TransferFunction),
        Self::ResultViewer(ResultViewer::Specs),
        Self::ResultViewer(ResultViewer::Table),
        Self::ResultViewer(ResultViewer::Nyquist),
        Self::ResultViewer(ResultViewer::Smith),
        Self::ResultViewer(ResultViewer::Polar),
        Self::ResultViewer(ResultViewer::NetworkMatrix),
        Self::ResultViewer(ResultViewer::PoleZero),
        Self::ResultViewer(ResultViewer::Scatter),
        Self::ResultViewer(ResultViewer::BoxViolin),
        Self::ResultViewer(ResultViewer::Events),
        Self::ResultViewer(ResultViewer::Soa),
        Self::ResultViewer(ResultViewer::Optimization),
        Self::ResultViewer(ResultViewer::Manifest),
        Self::EditSpecifications,
        Self::AddVisualizationPane,
        Self::VisualizationDocumentProperties,
    ];

    /// The command `key` names, if the feature declares one.
    pub(crate) fn from_key(key: CommandKey) -> Option<Self> {
        Self::ALL.into_iter().find(|command| command.key() == key)
    }

    /// The key a shortcut profile names this command by.
    pub(crate) const fn key(self) -> CommandKey {
        match self {
            Self::ImportResultDataset => CommandKey::new("import-dataset"),
            Self::ExportWaveformsCsv => ids::EXPORT_RESULTS,
            Self::ClearResults => CommandKey::new("clear-results"),
            Self::ToggleLinkedCursors => CommandKey::new("toggle-linked-cursors"),
            Self::ToggleResultsSplit => ids::TOGGLE_SPLIT_VIEW,
            Self::DatasetManifestBrowser => CommandKey::new("dataset-browser"),
            Self::OpenRunInResults => ids::OPEN_RUN_IN_RESULTS,
            Self::OpenProducingPlan => CommandKey::new("open-producing-plan"),
            Self::OpenTaskDeck => CommandKey::new("open-task-deck"),
            Self::RevealProducerLog => CommandKey::new("reveal-producer-log"),
            Self::CreateResultDocument => CommandKey::new("create-result-document"),
            Self::WaveformCalculator => ids::CALCULATOR,
            Self::ExpressionDiagnostics => ids::EXPRESSION_DIAGNOSTICS,
            Self::CompareResultDatasets => CommandKey::new("compare-datasets"),
            Self::MeasurementLibrary => ids::MEASUREMENT_LIBRARY,
            Self::FamilySlicing => ids::FAMILY_SLICING,
            Self::ResultViewer(viewer) => viewer_key(viewer),
            Self::EditSpecifications => ids::EDIT_SPECIFICATIONS,
            Self::AddVisualizationPane => CommandKey::new("visualization-add-pane"),
            Self::VisualizationDocumentProperties => CommandKey::new("plot-properties"),
        }
    }

    /// What the command declares about itself. Every one is listed in the
    /// palette.
    pub(crate) const fn declared_spec(self) -> CommandSpec {
        let (label, group, flags, context, bindings) = match self {
            Self::ImportResultDataset => (
                "Import result dataset…",
                "Results",
                PROJECT_TRANSACTION,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::ExportWaveformsCsv => (
                "Export dataset…",
                "Results",
                PROJECT,
                ShortcutContext::Global,
                NONE,
            ),
            Self::ClearResults => (
                "Clear result history",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::ToggleLinkedCursors => (
                "Linked A/B cursors",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                LINKED_CURSORS,
            ),
            Self::ToggleResultsSplit => (
                "Split with results",
                "Window",
                PROJECT,
                ShortcutContext::Global,
                TOGGLE_RESULTS_SPLIT,
            ),
            Self::DatasetManifestBrowser => (
                "Dataset and manifest browser…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::OpenRunInResults => (
                "Open run in Results",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::OpenProducingPlan => (
                "Open producing simulation plan",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::OpenTaskDeck => (
                OPEN_TASK_DECK,
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::RevealProducerLog => (
                "Open producing run log",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::CreateResultDocument => (
                "Create result document…",
                "Results",
                PROJECT_TRANSACTION,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::WaveformCalculator => (
                "Calculator…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::ExpressionDiagnostics => (
                "Expression and unit diagnostics…",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::CompareResultDatasets => (
                "Add result comparison…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::MeasurementLibrary => (
                "Measurement library…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::FamilySlicing => (
                "Family slicing and pivot…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::ResultViewer(viewer) => (
                viewer_label(viewer),
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::EditSpecifications => (
                "Edit specification matrix",
                "Results",
                PROJECT,
                ShortcutContext::VerificationWorkspace,
                NONE,
            ),
            Self::AddVisualizationPane => (
                "Add pane…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::VisualizationDocumentProperties => (
                "Plot document properties…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
        };
        CommandSpec {
            key: self.key(),
            label,
            group,
            flags,
            context,
            bindings,
            owner: CommandOwner::Results,
        }
    }
}

/// The key of the command that opens `viewer`.
const fn viewer_key(viewer: ResultViewer) -> CommandKey {
    match viewer {
        ResultViewer::Manifest => CommandKey::new("result-manifest"),
        ResultViewer::Waves => ids::RESULT_WAVEFORMS,
        ResultViewer::DcSweep => CommandKey::new("result-dc-sweep"),
        ResultViewer::Bode => CommandKey::new("result-bode"),
        ResultViewer::Fft => CommandKey::new("result-fft"),
        ResultViewer::HarmonicBalance => CommandKey::new("result-harmonic-balance"),
        ResultViewer::PhaseNoise => CommandKey::new("result-phase-noise"),
        ResultViewer::Eye => CommandKey::new("result-eye"),
        ResultViewer::Hist => CommandKey::new("result-histogram"),
        ResultViewer::Op => ids::RESULT_OPERATING_POINT,
        ResultViewer::NoiseContrib => CommandKey::new("result-noise"),
        ResultViewer::Contribution => CommandKey::new("result-sensitivity-contribution"),
        ResultViewer::TransferFunction => CommandKey::new("result-transfer-function"),
        ResultViewer::Specs => ids::RESULT_SPECIFICATIONS,
        ResultViewer::Table => CommandKey::new("result-table"),
        ResultViewer::Nyquist => CommandKey::new("result-nyquist"),
        ResultViewer::Smith => CommandKey::new("result-smith"),
        ResultViewer::Polar => CommandKey::new("result-polar"),
        ResultViewer::NetworkMatrix => CommandKey::new("result-network-matrix"),
        ResultViewer::PoleZero => CommandKey::new("result-pole-zero"),
        ResultViewer::Scatter => CommandKey::new("result-scatter"),
        ResultViewer::BoxViolin => CommandKey::new("result-distribution"),
        ResultViewer::Events => CommandKey::new("result-events"),
        ResultViewer::Soa => CommandKey::new("result-safe-operating-area"),
        ResultViewer::Optimization => CommandKey::new("result-optimization"),
    }
}

/// The label of the command that opens `viewer`.
const fn viewer_label(viewer: ResultViewer) -> &'static str {
    match viewer {
        ResultViewer::Manifest => "Open dataset manifest",
        // Named for the sheet it selects, not for the workspace it lands
        // in: opening the results workspace is the route that restores
        // whatever sheet was last open, and two palette rows reading "Open
        // results workspace" would be indistinguishable.
        ResultViewer::Waves => "Open waveform viewer",
        ResultViewer::DcSweep => "Open DC sweep viewer",
        ResultViewer::Bode => "Open Bode viewer",
        ResultViewer::Fft => "Open FFT viewer",
        ResultViewer::HarmonicBalance => "Open HB tones viewer",
        ResultViewer::PhaseNoise => "Open phase-noise viewer",
        ResultViewer::Eye => "Open eye-diagram viewer",
        ResultViewer::Hist => "Open histogram viewer",
        ResultViewer::Op => "Open operating-point viewer",
        ResultViewer::NoiseContrib => "Open noise-contribution viewer",
        ResultViewer::Contribution => "Open sensitivity-contribution viewer",
        ResultViewer::TransferFunction => "Open transfer-function viewer",
        ResultViewer::Specs => "Open specification results",
        ResultViewer::Table => "Open sample table",
        ResultViewer::Nyquist => "Open Nyquist viewer",
        ResultViewer::Smith => "Open Smith-chart viewer",
        ResultViewer::Polar => "Open polar-response viewer",
        ResultViewer::NetworkMatrix => "Open network matrix viewer",
        ResultViewer::PoleZero => "Open pole-zero viewer",
        ResultViewer::Scatter => "Open correlation scatter viewer",
        ResultViewer::BoxViolin => "Open box and violin distribution viewer",
        ResultViewer::Events => "Open event history viewer",
        ResultViewer::Soa => "Open safe-operating-area viewer",
        ResultViewer::Optimization => "Open optimization viewer",
    }
}
