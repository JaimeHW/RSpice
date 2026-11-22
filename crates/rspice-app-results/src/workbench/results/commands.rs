//! The commands the results feature declares, as the registry lists them:
//! importing, exporting and clearing result datasets, the result viewers,
//! the result document's forms and tools, and the hops back out of a run.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use egui::Key;
use rspice_results::result_presentation::ResultViewer;
pub use rspice_results_ui::waves::pointer::AtPointer;

use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};
use crate::workbench::localization::OPEN_TASK_DECK;

/// One command of the results feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultsCommand {
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
    AddVisualizationPane,
    VisualizationDocumentProperties,
    /// Move the stepping cursor to the largest sample of the trace it reads.
    CursorToMaximum,
    /// Move the stepping cursor to the smallest sample of the trace it reads.
    CursorToMinimum,
    /// Move the stepping cursor to the next peak of the trace it reads.
    CursorToNextPeak,
    /// Move the stepping cursor to the previous peak of the trace it reads.
    CursorToPreviousPeak,
    /// Move the stepping cursor to the next crossing of the crossing level.
    CursorToNextCrossing,
    /// Move the stepping cursor to the previous crossing of the level.
    CursorToPreviousCrossing,
    /// Move the stepping cursor to the next place the compared runs part by
    /// more than the tolerance.
    CursorToNextDifference,
    /// Move the stepping cursor to the previous place the compared runs
    /// part by more than the tolerance.
    CursorToPreviousDifference,
    /// Move the stepping cursor to the next sample of the trace it reads,
    /// the next edge of a selected logic lane.
    CursorToNextSample,
    /// Move the stepping cursor to the previous sample of the trace it
    /// reads, the previous edge of a selected logic lane.
    CursorToPreviousSample,
    /// Arm or disarm the A and B cursors.
    ToggleCursorTool,
    /// Hand the arrow keys and the jumps to the other cursor of the pair.
    SwitchSteppingCursor,
    /// Place a point marker on the trace at cursor A.
    MarkerAtCursor,
    /// Let go of the Y ranges the active strip's panes were pinned to, so
    /// each refits to the X window in view.
    FitYToVisibleX,
    /// Go forward to the waveform view Zoom previous stepped back from.
    ZoomNext,
    /// Offer what the active strip's curves may be plotted against.
    PlotAgainst,
    /// Keep the view in front as a result document of the project.
    SaveViewAsDocument,
    /// Give the saved result document shown another name.
    RenameDocument,
    /// Copy the saved result document shown as a document of its own.
    DuplicateDocument,
    /// Delete the saved result document shown.
    DeleteDocument,
    /// Put back every run deleted since the project opened.
    RestoreDeletedRuns,
    /// Draw this many runs of the shown run's plan just before it behind
    /// it, thinner; none for zero.
    OverlayPrevious(u8),
    /// Open the calculator holding the selected trace.
    SendTraceToCalculator,
    /// Show the spectrum of the selected trace.
    TraceSpectrum,
    /// Place or open what a key names where the pointer stands over a
    /// strip's plot; from a menu, where the Cursors page's Add would.
    AtPointer(AtPointer),
    /// Draw or leave out the minor grid between the labelled ticks.
    ToggleMinorGrid,
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
// The plot keys, as ViVA's bindkeys give the same commands bare letters.
// Each acts in Results only, where the sheet in front takes it.
const CURSOR_TOOL: &[ShortcutBinding] = &[primary(Key::C, false, false, false, "C")];
const NEXT_SAMPLE: &[ShortcutBinding] = &[primary(Key::ArrowRight, false, false, false, "Right")];
const PREVIOUS_SAMPLE: &[ShortcutBinding] = &[primary(Key::ArrowLeft, false, false, false, "Left")];
const NEXT_PEAK: &[ShortcutBinding] =
    &[primary(Key::ArrowRight, false, false, true, "Shift+Right")];
const PREVIOUS_PEAK: &[ShortcutBinding] =
    &[primary(Key::ArrowLeft, false, false, true, "Shift+Left")];
const NEXT_CROSSING: &[ShortcutBinding] =
    &[primary(Key::ArrowRight, false, true, false, "Alt+Right")];
const PREVIOUS_CROSSING: &[ShortcutBinding] =
    &[primary(Key::ArrowLeft, false, true, false, "Alt+Left")];
const MAXIMUM: &[ShortcutBinding] = &[primary(Key::ArrowUp, false, false, true, "Shift+Up")];
const MINIMUM: &[ShortcutBinding] = &[primary(Key::ArrowDown, false, false, true, "Shift+Down")];
const SWITCH_CURSOR: &[ShortcutBinding] = &[primary(Key::S, false, false, true, "Shift+S")];
const MARKER_AT_CURSOR: &[ShortcutBinding] = &[primary(Key::M, false, false, true, "Shift+M")];
const CALCULATOR: &[ShortcutBinding] = &[primary(Key::E, false, false, false, "E")];
const ADD_MEASUREMENT: &[ShortcutBinding] = &[primary(Key::E, false, false, true, "Shift+E")];
const COMPARE: &[ShortcutBinding] = &[primary(Key::B, false, false, true, "Shift+B")];
const SAVE_VIEW: &[ShortcutBinding] = &[primary(Key::N, false, false, true, "Shift+N")];
// The keys that act where the pointer stands, ViVA's letters for the same
// commands; L for a label. Q and G are the shell's Properties and Cycle grid
// display, whose edit context reaches Results, so the marker's properties
// and the minor grid take them with Shift.
const CURSOR_A: &[ShortcutBinding] = &[primary(Key::A, false, false, false, "A")];
const CURSOR_B: &[ShortcutBinding] = &[primary(Key::B, false, false, false, "B")];
const HORIZONTAL_CURSOR: &[ShortcutBinding] = &[primary(Key::H, false, false, false, "H")];
const VERTICAL_MARKER: &[ShortcutBinding] = &[primary(Key::V, false, false, false, "V")];
const POINT_MARKER: &[ShortcutBinding] = &[primary(Key::M, false, false, false, "M")];
const DELTA_MARKER: &[ShortcutBinding] = &[primary(Key::D, false, false, false, "D")];
const NOTE: &[ShortcutBinding] = &[primary(Key::L, false, false, false, "L")];
const PROPERTIES: &[ShortcutBinding] = &[primary(Key::Q, false, false, true, "Shift+Q")];
const MINOR_GRID: &[ShortcutBinding] = &[primary(Key::G, false, false, true, "Shift+G")];
/// Forward again through the views Zoom previous stepped back from.
const ZOOM_NEXT: &[ShortcutBinding] = &[primary(Key::Z, false, false, true, "Shift+Z")];

/// A command listed in the palette that runs with or without a project.
const ANYWHERE: CommandFlags = CommandFlags::PALETTE_VISIBLE;
/// A command that needs an open project.
const PROJECT: CommandFlags = ANYWHERE.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
/// A command that needs an open project and waits for its operation to end.
const PROJECT_TRANSACTION: CommandFlags =
    PROJECT.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);
/// A hop to where a run came from, which states why when it cannot go.
const PROJECT_HOP: CommandFlags = PROJECT.with(CommandFlags::STATES_ITS_REFUSAL, true);

impl ResultsCommand {
    /// Every command of the feature.
    pub const ALL: [Self; 80] = [
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
        Self::ResultViewer(ResultViewer::Table),
        Self::ResultViewer(ResultViewer::Nyquist),
        Self::ResultViewer(ResultViewer::SParameters),
        Self::ResultViewer(ResultViewer::Smith),
        Self::ResultViewer(ResultViewer::Polar),
        Self::ResultViewer(ResultViewer::NetworkMatrix),
        Self::ResultViewer(ResultViewer::PoleZero),
        Self::ResultViewer(ResultViewer::Floquet),
        Self::ResultViewer(ResultViewer::Scatter),
        Self::ResultViewer(ResultViewer::BoxViolin),
        Self::ResultViewer(ResultViewer::Events),
        Self::ResultViewer(ResultViewer::Soa),
        Self::ResultViewer(ResultViewer::Optimization),
        Self::ResultViewer(ResultViewer::Sweep),
        Self::AddVisualizationPane,
        Self::VisualizationDocumentProperties,
        Self::CursorToMaximum,
        Self::CursorToMinimum,
        Self::CursorToNextPeak,
        Self::CursorToPreviousPeak,
        Self::CursorToNextCrossing,
        Self::CursorToPreviousCrossing,
        Self::CursorToNextDifference,
        Self::CursorToPreviousDifference,
        Self::CursorToNextSample,
        Self::CursorToPreviousSample,
        Self::ToggleCursorTool,
        Self::SwitchSteppingCursor,
        Self::MarkerAtCursor,
        Self::FitYToVisibleX,
        Self::ZoomNext,
        Self::PlotAgainst,
        Self::SaveViewAsDocument,
        Self::RenameDocument,
        Self::DuplicateDocument,
        Self::DeleteDocument,
        Self::RestoreDeletedRuns,
        Self::OverlayPrevious(0),
        Self::OverlayPrevious(1),
        Self::OverlayPrevious(2),
        Self::OverlayPrevious(3),
        Self::SendTraceToCalculator,
        Self::TraceSpectrum,
        Self::AtPointer(AtPointer::CursorA),
        Self::AtPointer(AtPointer::CursorB),
        Self::AtPointer(AtPointer::HorizontalCursor),
        Self::AtPointer(AtPointer::VerticalMarker),
        Self::AtPointer(AtPointer::PointMarker),
        Self::AtPointer(AtPointer::DeltaMarker),
        Self::AtPointer(AtPointer::Note),
        Self::AtPointer(AtPointer::Properties),
        Self::ToggleMinorGrid,
    ];

    /// The command `key` names, if the feature declares one.
    pub fn from_key(key: CommandKey) -> Option<Self> {
        Self::ALL.into_iter().find(|command| command.key() == key)
    }

    /// The key a shortcut profile names this command by.
    pub const fn key(self) -> CommandKey {
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
            Self::AddVisualizationPane => CommandKey::new("visualization-add-pane"),
            Self::VisualizationDocumentProperties => CommandKey::new("plot-properties"),
            Self::CursorToMaximum => CommandKey::new("cursor-to-maximum"),
            Self::CursorToMinimum => CommandKey::new("cursor-to-minimum"),
            Self::CursorToNextPeak => CommandKey::new("cursor-to-next-peak"),
            Self::CursorToPreviousPeak => CommandKey::new("cursor-to-previous-peak"),
            Self::CursorToNextCrossing => CommandKey::new("cursor-to-next-crossing"),
            Self::CursorToPreviousCrossing => CommandKey::new("cursor-to-previous-crossing"),
            Self::CursorToNextDifference => CommandKey::new("cursor-to-next-difference"),
            Self::CursorToPreviousDifference => CommandKey::new("cursor-to-previous-difference"),
            Self::CursorToNextSample => CommandKey::new("cursor-to-next-sample"),
            Self::CursorToPreviousSample => CommandKey::new("cursor-to-previous-sample"),
            Self::ToggleCursorTool => CommandKey::new("toggle-cursor-tool"),
            Self::SwitchSteppingCursor => CommandKey::new("switch-stepping-cursor"),
            Self::MarkerAtCursor => CommandKey::new("marker-at-cursor"),
            Self::FitYToVisibleX => CommandKey::new("fit-y-to-visible-x"),
            Self::ZoomNext => CommandKey::new("zoom-next"),
            Self::PlotAgainst => CommandKey::new("plot-against"),
            Self::SaveViewAsDocument => CommandKey::new("save-view-as-document"),
            Self::RenameDocument => CommandKey::new("rename-result-document"),
            Self::DuplicateDocument => CommandKey::new("duplicate-result-document"),
            Self::DeleteDocument => CommandKey::new("delete-result-document"),
            Self::RestoreDeletedRuns => CommandKey::new("restore-deleted-runs"),
            Self::OverlayPrevious(0) => CommandKey::new("overlay-previous-runs-off"),
            Self::OverlayPrevious(1) => CommandKey::new("overlay-previous-run"),
            Self::OverlayPrevious(2) => CommandKey::new("overlay-previous-2-runs"),
            Self::OverlayPrevious(_) => CommandKey::new("overlay-previous-3-runs"),
            Self::SendTraceToCalculator => CommandKey::new("send-trace-to-calculator"),
            Self::TraceSpectrum => CommandKey::new("trace-spectrum"),
            Self::AtPointer(AtPointer::CursorA) => CommandKey::new("cursor-a-at-pointer"),
            Self::AtPointer(AtPointer::CursorB) => CommandKey::new("cursor-b-at-pointer"),
            Self::AtPointer(AtPointer::HorizontalCursor) => {
                CommandKey::new("horizontal-cursor-at-pointer")
            }
            Self::AtPointer(AtPointer::VerticalMarker) => {
                CommandKey::new("vertical-marker-at-pointer")
            }
            Self::AtPointer(AtPointer::PointMarker) => CommandKey::new("point-marker-at-pointer"),
            Self::AtPointer(AtPointer::DeltaMarker) => CommandKey::new("delta-marker-at-pointer"),
            Self::AtPointer(AtPointer::Note) => CommandKey::new("note-at-pointer"),
            Self::AtPointer(AtPointer::Properties) => CommandKey::new("marker-properties"),
            Self::ToggleMinorGrid => CommandKey::new("toggle-minor-grid"),
        }
    }

    /// What the command declares about itself. Every one is listed in the
    /// palette.
    pub const fn declared_spec(self) -> CommandSpec {
        let (label, group, flags, context, bindings) = match self {
            Self::ImportResultDataset => (
                "Import result dataset…",
                "Results",
                PROJECT_TRANSACTION,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::ExportWaveformsCsv => (
                "Export result dataset…",
                "Results",
                PROJECT,
                ShortcutContext::Global,
                NONE,
            ),
            Self::ClearResults => (
                "Clear result history…",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::ToggleLinkedCursors => (
                "Link A/B cursors",
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
                "Data browser",
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
                PROJECT_HOP,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::OpenTaskDeck => (
                OPEN_TASK_DECK,
                "Results",
                PROJECT_HOP,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::RevealProducerLog => (
                "Open producing run log",
                "Results",
                PROJECT_HOP,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::CreateResultDocument => (
                "New result document…",
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
                CALCULATOR,
            ),
            Self::ExpressionDiagnostics => (
                "Expression diagnostics…",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::CompareResultDatasets => (
                "Compare datasets…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                COMPARE,
            ),
            Self::MeasurementLibrary => (
                "Add measurement…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                ADD_MEASUREMENT,
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
            Self::AddVisualizationPane => (
                "Add pane…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::VisualizationDocumentProperties => (
                "Plot properties…",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            // The arrows step the same cursor sample by sample, Shift+arrow
            // by peak and Alt+arrow by crossing, as Left and Right step a
            // logic lane from edge to edge.
            Self::CursorToMaximum => cursor_command("Cursor to maximum", MAXIMUM),
            Self::CursorToMinimum => cursor_command("Cursor to minimum", MINIMUM),
            Self::CursorToNextPeak => cursor_command("Cursor to next peak", NEXT_PEAK),
            Self::CursorToPreviousPeak => cursor_command("Cursor to previous peak", PREVIOUS_PEAK),
            Self::CursorToNextCrossing => cursor_command("Cursor to next crossing", NEXT_CROSSING),
            Self::CursorToPreviousCrossing => {
                cursor_command("Cursor to previous crossing", PREVIOUS_CROSSING)
            }
            Self::CursorToNextDifference => cursor_command("Cursor to next difference", NONE),
            Self::CursorToPreviousDifference => {
                cursor_command("Cursor to previous difference", NONE)
            }
            Self::CursorToNextSample => cursor_command("Cursor to next sample", NEXT_SAMPLE),
            Self::CursorToPreviousSample => {
                cursor_command("Cursor to previous sample", PREVIOUS_SAMPLE)
            }
            Self::ToggleCursorTool => cursor_command("A and B cursors", CURSOR_TOOL),
            Self::SwitchSteppingCursor => cursor_command("Step the other cursor", SWITCH_CURSOR),
            Self::MarkerAtCursor => cursor_command("Place marker at cursor A", MARKER_AT_CURSOR),
            Self::AtPointer(AtPointer::CursorA) => cursor_command("Place cursor A", CURSOR_A),
            Self::AtPointer(AtPointer::CursorB) => cursor_command("Place cursor B", CURSOR_B),
            Self::AtPointer(AtPointer::HorizontalCursor) => {
                cursor_command("Add horizontal cursor", HORIZONTAL_CURSOR)
            }
            Self::AtPointer(AtPointer::VerticalMarker) => {
                cursor_command("Add vertical marker", VERTICAL_MARKER)
            }
            Self::AtPointer(AtPointer::PointMarker) => {
                cursor_command("Add point marker", POINT_MARKER)
            }
            Self::AtPointer(AtPointer::DeltaMarker) => {
                cursor_command("Add delta marker", DELTA_MARKER)
            }
            Self::AtPointer(AtPointer::Note) => cursor_command("Add note", NOTE),
            Self::AtPointer(AtPointer::Properties) => {
                cursor_command("Marker properties\u{2026}", PROPERTIES)
            }
            Self::ToggleMinorGrid => cursor_command("Minor grid", MINOR_GRID),
            Self::FitYToVisibleX => (
                "Fit Y to visible X",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::ZoomNext => (
                "Zoom next",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                ZOOM_NEXT,
            ),
            Self::PlotAgainst => (
                "Plot against\u{2026}",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::SaveViewAsDocument => (
                "Save view as document",
                "Results",
                PROJECT_TRANSACTION,
                ShortcutContext::ResultsWorkspace,
                SAVE_VIEW,
            ),
            Self::RenameDocument => (
                "Rename document…",
                "Results",
                PROJECT_TRANSACTION,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::DuplicateDocument => (
                "Duplicate document",
                "Results",
                PROJECT_TRANSACTION,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::DeleteDocument => (
                "Delete document",
                "Results",
                PROJECT_TRANSACTION,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::RestoreDeletedRuns => (
                "Restore deleted runs",
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::OverlayPrevious(depth) => (
                match depth {
                    0 => "Overlay no previous run",
                    1 => "Overlay the previous run",
                    2 => "Overlay the previous 2 runs",
                    _ => "Overlay the previous 3 runs",
                },
                "Results",
                ANYWHERE,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::SendTraceToCalculator => (
                "Send selected trace to calculator",
                "Results",
                PROJECT,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::TraceSpectrum => (
                "Spectrum of selected trace",
                "Results",
                ANYWHERE,
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

/// What a cursor command declares: a Results palette entry with `bindings`.
const fn cursor_command(
    label: &'static str,
    bindings: &'static [ShortcutBinding],
) -> (
    &'static str,
    &'static str,
    CommandFlags,
    ShortcutContext,
    &'static [ShortcutBinding],
) {
    (
        label,
        "Results",
        ANYWHERE,
        ShortcutContext::ResultsWorkspace,
        bindings,
    )
}

/// The key of the command that opens `viewer`.
const fn viewer_key(viewer: ResultViewer) -> CommandKey {
    match viewer {
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
        ResultViewer::Table => CommandKey::new("result-table"),
        ResultViewer::Nyquist => CommandKey::new("result-nyquist"),
        ResultViewer::SParameters => CommandKey::new("result-s-parameters"),
        ResultViewer::Smith => CommandKey::new("result-smith"),
        ResultViewer::Polar => CommandKey::new("result-polar"),
        ResultViewer::NetworkMatrix => CommandKey::new("result-network-matrix"),
        ResultViewer::PoleZero => CommandKey::new("result-pole-zero"),
        ResultViewer::Floquet => CommandKey::new("result-floquet"),
        ResultViewer::Scatter => CommandKey::new("result-scatter"),
        ResultViewer::BoxViolin => CommandKey::new("result-distribution"),
        ResultViewer::Events => CommandKey::new("result-events"),
        ResultViewer::Soa => ids::RESULT_SAFE_OPERATING_AREA,
        ResultViewer::Optimization => CommandKey::new("result-optimization"),
        ResultViewer::Sweep => CommandKey::new("result-sweep"),
    }
}

/// The label of the command that opens `viewer`.
const fn viewer_label(viewer: ResultViewer) -> &'static str {
    match viewer {
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
        ResultViewer::Table => "Open sample table",
        ResultViewer::Nyquist => "Open Nyquist viewer",
        ResultViewer::SParameters => "Open S-parameter viewer",
        ResultViewer::Smith => "Open Smith-chart viewer",
        ResultViewer::Polar => "Open polar-response viewer",
        ResultViewer::NetworkMatrix => "Open network matrix viewer",
        ResultViewer::PoleZero => "Open pole-zero viewer",
        ResultViewer::Floquet => "Open Floquet viewer",
        ResultViewer::Scatter => "Open correlation scatter viewer",
        ResultViewer::BoxViolin => "Open box and violin distribution viewer",
        ResultViewer::Events => "Open event history viewer",
        ResultViewer::Soa => "Open safe-operating-area viewer",
        ResultViewer::Optimization => "Open optimization viewer",
        ResultViewer::Sweep => "Open sweep viewer",
    }
}
