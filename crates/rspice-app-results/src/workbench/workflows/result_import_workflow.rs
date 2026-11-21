//! Validated external result-dataset ingest.
//!
//! Supported interchange adapters are deliberately classified as
//! legacy-unattributed external evidence. They become immutable retained runs,
//! but never claim native-solver provenance or prepared-plan authority.

use crate::diagnostics::{LogSeverity, LogSource};
pub(crate) use crate::state::ResultImportFormat;
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType,
    ResultImportSource, RetainedTrace, SimulationRunLifecycle, SimulationRunProvenance,
};
use crate::ui::tokens::Tokens;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::import::{
    RESULT_IMPORT_PREVIEW_ROWS, ResultImportDialogState, ResultImportJob, ResultImportPlots,
    ResultImportSampleFacts, ResultImportStage,
};
use crate::workbench::results_job::{JobContext, JobOutcome, JobState, JobStop, ResultsJob};
use rspice_formats::result_import::identify_result_import_format;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::{io::Read as _, path::Path};

#[path = "result_import_adapters.rs"]
mod adapters;

pub(crate) const RESULT_DATASET_FILTER: (&str, &[&str]) = (
    "Result dataset",
    &[
        "rspiceresult",
        "rspicedata",
        "csv",
        "tsv",
        "tab",
        "s1p",
        "s2p",
        "s3p",
        "s4p",
        "snp",
        "ts",
        "h5",
        "hdf5",
        "arrow",
        "feather",
        "parquet",
        "npy",
        "npz",
        "mat",
        "raw",
        "psfascii",
        "vcd",
        "fst",
    ],
);
/// A signal name longer than this is a damaged file: no simulator or instrument writes one.
const MAX_SIGNAL_NAME_BYTES: usize = 1_024;

/// The most values one byte of a source can stand for: what DEFLATE, the widest compression
/// any reader here accepts, expands one byte to at most. A source that states more rows,
/// columns or values than its bytes can hold is damaged, and is refused as such; how much
/// real data a source holds is never a reason to refuse it.
const MOST_VALUES_PER_SOURCE_BYTE: usize = 1_032;

thread_local! {
    /// The size of the source being read on this thread, which bounds what it can state.
    static SOURCE_BYTES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Read a source of `bytes` bytes on this thread until dropped.
struct ReadingSource(usize);

impl ReadingSource {
    fn of(bytes: usize) -> Self {
        Self(SOURCE_BYTES.with(|cell| cell.replace(bytes)))
    }
}

impl Drop for ReadingSource {
    fn drop(&mut self) {
        SOURCE_BYTES.with(|cell| cell.set(self.0));
    }
}

/// The most rows, columns or values the source being read can hold.
pub(crate) fn source_capacity() -> usize {
    SOURCE_BYTES
        .with(std::cell::Cell::get)
        .max(1)
        .saturating_mul(MOST_VALUES_PER_SOURCE_BYTE)
}

/// What a source being read may state before it is damaged: as many rows, columns and values
/// as its bytes can hold, and names of a length a real file uses.
pub(crate) fn waveform_import_limits()
-> rspice_results::result_import::waveforms::WaveformImportLimits {
    rspice_results::result_import::waveforms::WaveformImportLimits {
        min_rows: MIN_RESULT_ROWS,
        max_rows: source_capacity(),
        max_columns: source_capacity(),
        max_values: source_capacity(),
        max_signal_name_bytes: MAX_SIGNAL_NAME_BYTES,
    }
}

/// What an export of a retained run admits: every row and value the run holds, with names this
/// product reads back.
pub(crate) fn exported_waveform_limits()
-> rspice_results::result_import::waveforms::WaveformImportLimits {
    rspice_results::result_import::waveforms::WaveformImportLimits {
        min_rows: MIN_RESULT_ROWS,
        max_rows: usize::MAX,
        max_columns: usize::MAX,
        max_values: usize::MAX,
        max_signal_name_bytes: MAX_SIGNAL_NAME_BYTES,
    }
}
const MAX_HEADER_BYTES: usize = 256;
/// The names a source may give its coordinate, in the order a reader prefers
/// them when a file offers more than one.
///
/// A format with no header of its own — a bare HDF5 root, an NPZ archive, a
/// MATLAB workspace — states which of its arrays is the abscissa only by
/// naming it, so this list is the whole of that contract. It was written out
/// three times, once per reader, and the *writers* held a fourth copy: a
/// coordinate published under a name the readers did not know would have come
/// back as an anonymous sweep, and nothing would have failed until someone
/// reopened the file.
pub(crate) const RESULT_COORDINATE_NAMES: [&str; 5] = ["time", "frequency", "freq", "sweep", "x"];

/// The fewest coordinate samples a source may carry and still be a result.
///
/// **One.** A `.OP` is a single point by construction, and this product's own
/// HDF5, MAT and NumPy exports publish one — so a rule that wanted two refused
/// to reopen a file it had just written, which is the one thing an interchange
/// format must not do. The command line settled the same rule on its side of
/// the boundary first (`rspice-cli`'s `MIN_RESULT_SAMPLES`, also one) and this
/// constant is what agrees with it.
///
/// Zero is not a result: an empty coordinate has no row to draw, no shape for
/// a column to match, and nothing for a domain check to check.
///
/// Nothing downstream needs two. The waveform sheets draw whatever the run
/// kept — a lone sample is one of the degenerate shapes
/// `documents::result_document::waves` is pinned against — and the sheets that
/// do need a span (Bode, phase noise, the raw frequency curve) already ask for
/// one themselves and decline to render rather than refusing the dataset.
const MIN_RESULT_ROWS: usize = 1;
const RESULT_IMPORT_WINDOW_MARGIN: f32 = 24.0;
const RESULT_IMPORT_FOOTER_RESERVE: f32 = 82.0;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedResultDataset {
    pub(crate) source_format: ResultImportFormat,
    pub analysis_type: AnalysisType,
    pub(crate) coordinate_name: String,
    pub coordinate_unit: Option<String>,
    pub(crate) sample_count: usize,
    pub waveforms: Vec<RetainedTrace>,
    pub(crate) family_metadata: Option<AnalysisResultFamilyMetadata>,
    pub(crate) delimiter: u8,
    /// What the adapter had to decide that the source did not state, in the
    /// author's terms. An import that is complete but not literal — a
    /// four-state VCD landing on an analog grid, say — says so here rather
    /// than silently, and the review stage and the console both repeat it.
    pub(crate) notes: Vec<String>,
    /// The exact event history and bus declarations the source carried, when
    /// it carried any.
    ///
    /// A digital source states *events*, not samples: four-state codes at the
    /// times a writer recorded them, and — for a vector variable — which
    /// conductors are one word. `waveforms` above is the analog-grid
    /// projection of that, which the table and waveform sheets read; this is
    /// the evidence itself, which the Events sheet and the dump exporter
    /// read. An imported result carries both, and neither is derived from the
    /// other.
    pub(crate) event_payload: Option<AnalysisResultPayload>,
    /// Where and in what order a dump declared the variables of `event_payload`.
    pub(crate) event_dump: Option<rspice_results::result_import::ImportedEventDump>,
    /// The source's other analyses, when it holds several; committed beside
    /// this one in the same run.
    pub(crate) plots: Option<Arc<ResultImportPlots>>,
}

pub(crate) fn import_result_dataset(state: &mut ResultsParts<'_>) -> bool {
    if let Some(reason) = result_import_block_reason(state.model, state.platform) {
        state.prompt(reason);
        return false;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(path) = rfd::FileDialog::new()
            .add_filter(RESULT_DATASET_FILTER.0, RESULT_DATASET_FILTER.1)
            .set_title("Import result dataset")
            .pick_file()
        else {
            return false;
        };
        import_result_dataset_from_path(state, &path)
    }

    #[cfg(target_arch = "wasm32")]
    {
        match start_browser_result_dataset_import() {
            Ok(()) => true,
            Err(error) => {
                record_import_failure(state, &error);
                false
            }
        }
    }
}

/// Start reading the result dataset at `path`, the file the reader picked:
/// the dialog opens on the read's progress and its Cancel, and moves to
/// review when the source is parsed and its samples are kept. Refused at
/// once, with the reason, when no result can be imported now.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn import_result_dataset_from_path(state: &mut ResultsParts<'_>, path: &Path) -> bool {
    if let Some(reason) = result_import_block_reason(state.model, state.platform) {
        record_import_failure(state, &reason);
        return false;
    }
    let display_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let source = path.to_path_buf();
    let source_name = display_name.clone();
    let job = ResultsJob::start("result-import", move |job| {
        let bytes = read_native_file(&source, job)?;
        let log = companion_log(&source, job)?;
        stage_source(job, &source_name, &bytes, log.as_deref())
    });
    open_reading_draft(state, display_name, job);
    true
}

/// The dialog, open on a source being read.
fn open_reading_draft(
    state: &mut ResultsParts<'_>,
    source_name: String,
    job: ResultsJob<StagedResultImport>,
) {
    *state.result_import = ResultImportDialogState {
        open: true,
        source_name,
        job: Some(ResultImportJob::Reading(job)),
        ..ResultImportDialogState::default()
    };
}

/// A refused import raises a notice with the reason, which offers the
/// Console line it wrote, as a failed export does. It was a Console line
/// alone, so a refused file looked like nothing had happened.
fn record_import_failure(state: &mut ResultsParts<'_>, error: &str) {
    state.report_failure("Result dataset import failed", error);
}

fn result_import_block_reason(model: &Model, platform: &PlatformState) -> Option<String> {
    if !platform.project_lifecycle.is_open() {
        return Some("Open a project before importing a result dataset.".to_owned());
    }
    if model.safe_mode.project_read_only() {
        return Some("The project is read-only; result history cannot be changed.".to_owned());
    }
    if model.simulation.execution.has_active_execution() {
        return Some(
            "Wait for the active simulation execution to finish before importing result data."
                .to_owned(),
        );
    }
    None
}

/// The simulator's log beside a rawfile, when there is one that reads:
/// LTspice writes `deck.log` beside `deck.raw`, and it names the runs of a
/// `stepped` rawfile. Any other source has no use for it.
#[cfg(not(target_arch = "wasm32"))]
fn companion_log(path: &Path, job: &JobContext) -> Result<Option<Vec<u8>>, JobStop> {
    let raw = path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("raw"));
    let log = path.with_extension("log");
    if !raw || !log.is_file() {
        return Ok(None);
    }
    match read_native_file(&log, job) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(JobStop::Cancelled) => Err(JobStop::Cancelled),
        Err(JobStop::Failed(_)) => Ok(None),
    }
}

/// Bytes read between two reports of a read's progress and two looks at its
/// Cancel.
#[cfg(not(target_arch = "wasm32"))]
const READ_CHUNK_BYTES: u64 = 8 * 1024 * 1024;

/// The whole of the file at `path`, read in chunks that report progress and
/// stop at Cancel. A file larger than this computer can hold is refused
/// before a byte is read, in those words.
#[cfg(not(target_arch = "wasm32"))]
fn read_native_file(path: &Path, job: &JobContext) -> Result<Vec<u8>, JobStop> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| format!("could not open {}: {error}", path.display()))?;
    let size = file
        .metadata()
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
        .len();
    let mut bytes = Vec::new();
    usize::try_from(size)
        .ok()
        .and_then(|size| bytes.try_reserve_exact(size).ok())
        .ok_or_else(|| {
            format!(
                "{} is {size} bytes, more than this computer has memory to read",
                path.display()
            )
        })?;
    job.step("Reading", size);
    loop {
        job.check()?;
        let read = (&mut file)
            .take(READ_CHUNK_BYTES)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        if read == 0 {
            return Ok(bytes);
        }
        job.done(bytes.len() as u64);
    }
}

/// Parse `bytes`, keep their samples in results store segments, and read
/// what the review pages show of them: the work an import's job does once
/// the source is read. Stops at Cancel between its steps; what it kept by
/// then is deleted as it is dropped.
fn stage_source(
    job: &JobContext,
    source_name: &str,
    bytes: &[u8],
    companion_log: Option<&[u8]>,
) -> Result<StagedResultImport, JobStop> {
    job.step("Parsing", 0);
    job.check()?;
    let mut parsed = parse_result_dataset_beside(source_name, bytes, companion_log)?;
    job.check()?;
    let facts = ResultImportSampleFacts::read(&parsed.waveforms);
    keep_parsed_samples(job, source_name, &mut parsed)?;
    Ok(StagedResultImport {
        source_name: source_name.to_owned(),
        parsed,
        facts,
    })
}

/// Put the samples of every analysis `parsed` holds into results store
/// segments, so the draft under review and the run it becomes hold none in
/// memory. An analysis the store refuses keeps its samples in memory, as a
/// finished run's does, and the review notes why.
fn keep_parsed_samples(
    job: &JobContext,
    source_name: &str,
    parsed: &mut ParsedResultDataset,
) -> Result<(), JobStop> {
    let further = parsed.plots.as_ref().map_or(0, |plots| plots.further.len());
    job.step("Keeping samples", 1 + further as u64);
    let format = parsed.source_format;
    let keep = |analysis_type: AnalysisType,
                coordinate: (&str, &Option<String>),
                waveforms: &mut Vec<RetainedTrace>| {
        let mut analysis =
            AnalysisResult::new(1, analysis_type, format!("Imported \u{00b7} {source_name}"))
                .with_waveforms(std::mem::take(waveforms));
        analysis.import_source = Some(ResultImportSource {
            source_name: source_name.to_owned(),
            format,
            coordinate: Some(rspice_results::result_import::ResultImportCoordinate {
                name: coordinate.0.to_owned(),
                unit: coordinate.1.clone(),
            }),
            family_member: None,
            event_dump: None,
        });
        let kept = crate::state::run_store::store_analysis(&mut analysis.data);
        *waveforms = std::mem::take(&mut analysis.data.waveforms);
        kept.err()
    };
    let mut refused = Vec::new();
    refused.extend(keep(
        parsed.analysis_type,
        (&parsed.coordinate_name, &parsed.coordinate_unit),
        &mut parsed.waveforms,
    ));
    job.done(1);
    if let Some(plots) = parsed.plots.as_mut() {
        let plots = Arc::make_mut(plots);
        for (index, plot) in plots.further.iter_mut().enumerate() {
            job.check()?;
            refused.extend(keep(
                plot.analysis_type,
                (&plot.coordinate_name, &plot.coordinate_unit),
                &mut plot.waveforms,
            ));
            job.done(2 + index as u64);
        }
    }
    parsed.notes.extend(
        refused
            .into_iter()
            .map(|reason| format!("its samples stay in memory: {reason}")),
    );
    job.check()
}

/// A source parsed off the UI thread, its samples kept, with what the review
/// pages show of them.
#[derive(Debug)]
pub struct StagedResultImport {
    source_name: String,
    parsed: ParsedResultDataset,
    facts: ResultImportSampleFacts,
}

/// Parse a selected file into a reviewable draft without mutating retained
/// simulation history. The work an import's job does, done here and now.
pub fn stage_imported_result_dataset(
    state: &mut ResultsParts<'_>,
    source_name: &str,
    bytes: &[u8],
) -> Result<(), String> {
    stage_imported_result_dataset_beside(state, source_name, bytes, None)
}

/// [`stage_imported_result_dataset`], with the simulator's log that sat
/// beside the selected file.
fn stage_imported_result_dataset_beside(
    state: &mut ResultsParts<'_>,
    source_name: &str,
    bytes: &[u8],
    companion_log: Option<&[u8]>,
) -> Result<(), String> {
    if let Some(reason) = result_import_block_reason(state.model, state.platform) {
        return Err(reason);
    }
    match stage_source(&JobContext::detached(), source_name, bytes, companion_log) {
        Ok(staged) => {
            open_review_draft(state, staged);
            Ok(())
        }
        Err(JobStop::Failed(reason)) => Err(reason),
        Err(JobStop::Cancelled) => Err("the import was cancelled".to_owned()),
    }
}

/// The dialog, open on the review of a staged source.
fn open_review_draft(state: &mut ResultsParts<'_>, staged: StagedResultImport) {
    let StagedResultImport {
        source_name,
        parsed,
        facts,
    } = staged;
    let signal_count = parsed.waveforms.len();
    *state.result_import = ResultImportDialogState {
        open: true,
        stage: ResultImportStage::Detect,
        source_name,
        source_format_id: parsed.source_format.canonical_id().to_owned(),
        delimiter: parsed.delimiter,
        analysis_type: parsed.analysis_type,
        coordinate_name: parsed.coordinate_name,
        coordinate_unit: parsed.coordinate_unit,
        sample_count: parsed.sample_count,
        waveforms: Arc::new(parsed.waveforms),
        family_metadata: parsed.family_metadata,
        notes: parsed.notes,
        event_payload: parsed.event_payload,
        event_dump: parsed.event_dump,
        plots: parsed.plots,
        selected_signals: vec![true; signal_count],
        validation_error: None,
        job: None,
        sample_facts: Arc::new(facts),
    };
}

fn parsed_result_from_draft(
    draft: &ResultImportDialogState,
) -> Result<ParsedResultDataset, String> {
    if draft.source_name.trim().is_empty() {
        return Err("The import source name is empty.".to_owned());
    }
    let coordinate_name = draft.coordinate_name.trim();
    if coordinate_name.is_empty() {
        return Err("Enter a coordinate name before committing the import.".to_owned());
    }
    rspice_results::result_import::ResultImportCoordinate {
        name: coordinate_name.to_owned(),
        unit: draft.coordinate_unit.clone(),
    }
    .validate_domain(draft.analysis_type)?;
    if !matches!(
        draft.analysis_type,
        AnalysisType::Transient
            | AnalysisType::Ac
            | AnalysisType::DcSweep
            | AnalysisType::SParameter
    ) {
        return Err(
            "Select a supported transient, AC, DC-sweep, or S-parameter domain.".to_owned(),
        );
    }
    let source_format = ResultImportFormat::from_canonical_id(&draft.source_format_id)
        .ok_or_else(|| "The staged import format identity is missing or invalid.".to_owned())?;
    let touchstone = matches!(
        source_format,
        ResultImportFormat::TouchstoneV1 | ResultImportFormat::TouchstoneV2
    );
    if touchstone && draft.analysis_type != AnalysisType::SParameter {
        return Err("Touchstone network data must remain in the S-parameter domain.".to_owned());
    }
    if !touchstone && draft.analysis_type == AnalysisType::SParameter {
        return Err(
            "S-parameter mapping requires a Touchstone source with reference-impedance authority."
                .to_owned(),
        );
    }
    if draft.selected_signals.len() != draft.waveforms.len() {
        return Err("The signal mapping no longer matches the parsed source.".to_owned());
    }
    let waveforms = draft
        .waveforms
        .iter()
        .zip(&draft.selected_signals)
        .filter_map(|(waveform, selected)| selected.then_some(waveform.clone()))
        .collect::<Vec<_>>();
    if waveforms.is_empty() {
        return Err("Select at least one signal to import.".to_owned());
    }
    if waveforms.iter().any(|waveform| {
        waveform.x.len() != waveform.y.len()
            || (!touchstone && waveform.x.len() != draft.sample_count)
    }) {
        return Err("A selected signal no longer matches the detected coordinate grid.".to_owned());
    }
    if matches!(
        draft.analysis_type,
        AnalysisType::Ac | AnalysisType::SParameter
    ) {
        let facts = sample_facts_of(draft);
        let off_grid = facts
            .frequency_grid
            .iter()
            .zip(&draft.selected_signals)
            .any(|(grid, selected)| *selected && !grid);
        if off_grid {
            return Err("Frequency coordinates must all be greater than zero.".to_owned());
        }
    }
    if draft.analysis_type == AnalysisType::SParameter {
        let Some(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
            ..
        }) = draft.family_metadata.as_ref()
        else {
            return Err(
                "Touchstone import is missing per-port reference-impedance authority.".to_owned(),
            );
        };
        if reference_impedances_ohm.is_empty()
            || waveforms.iter().any(|waveform| {
                waveform.complex.is_none() && !matches!(waveform.name.as_str(), "Fmin" | "Rn")
            })
        {
            return Err("Touchstone import contains incomplete complex network data.".to_owned());
        }
    }
    Ok(ParsedResultDataset {
        source_format,
        analysis_type: draft.analysis_type,
        coordinate_name: coordinate_name.to_owned(),
        coordinate_unit: draft.coordinate_unit.clone(),
        sample_count: draft.sample_count,
        waveforms,
        family_metadata: draft.family_metadata.clone(),
        delimiter: draft.delimiter,
        notes: draft.notes.clone(),
        event_payload: draft.event_payload.clone(),
        event_dump: draft.event_dump.clone(),
        plots: draft.plots.clone(),
    })
}

/// The draft's sample facts: those its job read, or, for a draft made
/// without one, read now.
fn sample_facts_of(
    draft: &ResultImportDialogState,
) -> std::borrow::Cow<'_, ResultImportSampleFacts> {
    if draft.sample_facts.frequency_grid.len() == draft.waveforms.len() {
        std::borrow::Cow::Borrowed(&draft.sample_facts)
    } else {
        std::borrow::Cow::Owned(ResultImportSampleFacts::read(&draft.waveforms))
    }
}

/// Atomically commit the currently reviewed draft into retained result
/// history. Any failure restores the simulation and workbench selections that
/// existed before the transaction.
pub fn commit_result_import_draft(state: &mut ResultsParts<'_>) -> Result<(), String> {
    if let Some(reason) = result_import_block_reason(state.model, state.platform) {
        return Err(reason);
    }
    let draft = state.result_import.clone();
    if !draft.open {
        return Err("No result import draft is open.".to_owned());
    }
    let parsed = parsed_result_from_draft(&draft)?;
    commit_parsed_result_dataset(state, &draft.source_name, parsed)?;
    *state.result_import = ResultImportDialogState::default();
    Ok(())
}

/// Start committing the reviewed draft: its analyses are built and checked
/// off the UI thread while the dialog shows the work and its Cancel, and the
/// run is added when they are ready. Cancel returns to the review.
pub(crate) fn start_result_import_commit(state: &mut ResultsParts<'_>) -> Result<(), String> {
    if let Some(reason) = result_import_block_reason(state.model, state.platform) {
        return Err(reason);
    }
    let parsed = parsed_result_from_draft(state.result_import)?;
    let source_name = state.result_import.source_name.clone();
    let job = ResultsJob::start("result-import-commit", move |job| {
        job.step("Checking", 0);
        job.check()?;
        Ok(keep_reviewed_draft(&source_name, parsed)?)
    });
    state.result_import.job = Some(ResultImportJob::Committing(job));
    Ok(())
}

fn commit_parsed_result_dataset(
    state: &mut ResultsParts<'_>,
    source_name: &str,
    parsed: ParsedResultDataset,
) -> Result<(), String> {
    let kept = keep_reviewed_draft(source_name, parsed)?;
    insert_kept_import(state, kept)
}

/// The analyses a reviewed draft becomes, built and checked, before the run
/// that holds them is added.
#[derive(Debug)]
pub struct KeptResultImport {
    source_name: String,
    analyses: Vec<AnalysisResult>,
    analysis_type: AnalysisType,
    signal_count: usize,
    sample_count: usize,
    bus_count: usize,
    notes: Vec<String>,
}

/// Build and check the analyses `parsed` becomes. Reads no history, so it
/// runs off the UI thread.
fn keep_reviewed_draft(
    source_name: &str,
    parsed: ParsedResultDataset,
) -> Result<KeptResultImport, String> {
    let analysis_type = parsed.analysis_type;
    let sample_count = parsed.sample_count;
    let signal_count = parsed.waveforms.len();
    let notes = parsed.notes.clone();
    let mut bus_count = 0;
    let mut analyses = Vec::new();
    let reviewed = ImportedAnalysis {
        title: parsed.plots.as_ref().map(|plots| plots.reviewed.clone()),
        family_member: parsed
            .plots
            .as_ref()
            .and_then(|plots| plots.reviewed_member.clone()),
        analysis_type,
        coordinate_name: parsed.coordinate_name,
        coordinate_unit: parsed.coordinate_unit,
        waveforms: parsed.waveforms,
        event_payload: parsed.event_payload,
        event_dump: parsed.event_dump,
    };
    let mut reviewed = reviewed.retained(source_name, parsed.source_format, &mut bus_count)?;
    if let Some(metadata) = parsed.family_metadata {
        metadata.validate_for(analysis_type)?;
        reviewed = reviewed.with_family_metadata(metadata);
    }
    reviewed.validate_retained_evidence()?;
    analyses.push(reviewed);
    // A source of several analyses lands as one run that holds them all, the
    // reviewed one first and the rest in source order, each as written.
    for plot in parsed.plots.iter().flat_map(|plots| &plots.further) {
        let further = ImportedAnalysis {
            title: Some(plot.name.clone()),
            family_member: plot.family_member.clone(),
            analysis_type: plot.analysis_type,
            coordinate_name: plot.coordinate_name.clone(),
            coordinate_unit: plot.coordinate_unit.clone(),
            waveforms: plot.waveforms.clone(),
            event_payload: plot.event_payload.clone(),
            // A further analysis is a rawfile's plot, never a dump's.
            event_dump: None,
        }
        .retained(source_name, parsed.source_format, &mut bus_count)?;
        further.validate_retained_evidence()?;
        analyses.push(further);
    }
    Ok(KeptResultImport {
        source_name: source_name.to_owned(),
        analyses,
        analysis_type,
        signal_count,
        sample_count,
        bus_count,
        notes,
    })
}

/// Add the run that holds `kept`, as one change the reader can undo by
/// deleting the run, and show it.
fn insert_kept_import(state: &mut ResultsParts<'_>, kept: KeptResultImport) -> Result<(), String> {
    let KeptResultImport {
        source_name,
        analyses,
        analysis_type,
        signal_count,
        sample_count,
        bus_count,
        notes,
    } = kept;
    let source_name = source_name.as_str();
    let further_count = analyses.len() - 1;

    let previous_simulation = state.model.simulation.clone();
    let previous_workbench = state.workbench.clone();

    let (run_sequence, dataset_id) = {
        state.model.hold_document_datasets();
        let run = state.model.simulation.start_imported_run()?;
        run.job_id = None;
        run.execution_target = None;
        run.label = format!("Imported · {source_name}");
        for analysis in analyses {
            run.keep_analysis(analysis);
        }
        let run_sequence = run.id;
        let dataset_id = run.dataset_id;
        // Reading a dataset does not establish the foreign solver's lifecycle
        // or elapsed time. Keep that absence explicit, as with other imported
        // legacy evidence, instead of fabricating a completed native execution.
        let sealed = run
            .restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .and_then(|()| run.restore_lifecycle(SimulationRunLifecycle::LegacyUnknown, 0.0));
        if let Err(error) = sealed {
            state.model.simulation = previous_simulation;
            return Err(format!("could not seal imported result history: {error}"));
        }
        (run_sequence, dataset_id)
    };

    state.model.simulation.complete_run();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    if !crate::workbench::chrome::document_bar::results::show_result_dataset(state, &dataset_id) {
        state.model.simulation = previous_simulation;
        *state.workbench = previous_workbench;
        return Err(
            "The imported dataset could not be activated; no result history was changed."
                .to_owned(),
        );
    }
    state.results.session.viewer = if analysis_type == AnalysisType::SParameter {
        crate::workbench::ResultViewer::Smith
    } else {
        crate::workbench::ResultViewer::Waves
    };
    state
        .analysis
        .synchronize_specialized_viewer_cache_authority(state.model);
    // One line says what arrived and that RSpice did not simulate it; the
    // run history carries the run as imported, so nothing more is owed here.
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "Imported {source_name} as Run {run_sequence}: {signal_count} signals \u{00d7} \
             {sample_count} samples ({}){}{}. It was not simulated by RSpice.",
            analysis_domain_label(analysis_type),
            match bus_count {
                0 => String::new(),
                1 => ", with 1 digital bus".to_owned(),
                count => format!(", with {count} digital buses"),
            },
            match further_count {
                0 => String::new(),
                1 => ", and 1 further analysis".to_owned(),
                count => format!(", and {count} further analyses"),
            }
        ),
    );
    for note in notes {
        state.record(
            LogSeverity::Warning,
            LogSource::Studio,
            format!("{source_name}: {note}"),
        );
    }
    Ok(())
}

/// One analysis of an import on its way into retained history.
struct ImportedAnalysis {
    /// The source's own title, when it names its analyses.
    title: Option<String>,
    /// The `.step` run the source says this analysis is.
    family_member: Option<rspice_results::provenance::AnalysisResultFamilyMember>,
    analysis_type: AnalysisType,
    coordinate_name: String,
    coordinate_unit: Option<String>,
    waveforms: Vec<RetainedTrace>,
    event_payload: Option<AnalysisResultPayload>,
    event_dump: Option<rspice_results::result_import::ImportedEventDump>,
}

impl ImportedAnalysis {
    fn retained(
        self,
        source_name: &str,
        format: ResultImportFormat,
        bus_count: &mut usize,
    ) -> Result<AnalysisResult, String> {
        if !matches!(
            self.analysis_type,
            AnalysisType::Transient
                | AnalysisType::Ac
                | AnalysisType::DcSweep
                | AnalysisType::SParameter
        ) {
            return Err("import inferred an unsupported analysis domain".to_owned());
        }
        let coordinate_name = self.coordinate_name;
        let label = match (self.title, self.analysis_type) {
            (Some(title), _) => format!("Imported · {title}"),
            (None, AnalysisType::Transient) => format!("Imported transient · {coordinate_name}"),
            (None, AnalysisType::Ac) => format!("Imported AC · {coordinate_name}"),
            (None, AnalysisType::DcSweep) => format!("Imported DC sweep · {coordinate_name}"),
            (None, _) => format!("Imported S-parameters · {coordinate_name}"),
        };
        let mut analysis =
            AnalysisResult::new(1, self.analysis_type, label).with_waveforms(self.waveforms);
        // The exact event history goes in whole. It is not a projection of the
        // columns above and is not narrowed by the signal selection: a bus
        // declaration over a member the reader had unticked would name a trace
        // that is not there, and the validator refuses exactly that.
        if let Some(payload) = self.event_payload {
            payload.validate_for(self.analysis_type)?;
            if let AnalysisResultPayload::TransientEvents { digital_buses, .. } = &payload {
                *bus_count += digital_buses.len();
            }
            analysis = analysis.with_result_payload(payload);
        }
        analysis.import_source = Some(ResultImportSource {
            source_name: source_name.to_owned(),
            format,
            coordinate: Some(rspice_results::result_import::ResultImportCoordinate {
                name: coordinate_name,
                unit: self.coordinate_unit,
            }),
            family_member: self.family_member,
            event_dump: self.event_dump,
        });
        Ok(analysis)
    }
}

/// Render the staged external-result import transaction.
///
/// Closing or cancelling this window only discards the runtime draft. The
/// retained simulation model is touched exclusively by the final commit.
pub(crate) fn show_result_import_dialog(ctx: &egui::Context, state: &mut ResultsParts<'_>) {
    if !state.result_import.open {
        return;
    }
    if let Some(job) = state.result_import.job.clone() {
        show_result_import_job(ctx, state, &job);
        return;
    }

    let mut draft = state.result_import.clone();
    let mut window_open = true;
    let mut cancel = false;
    let mut go_back = false;
    let mut advance = false;
    let mut commit = false;
    let tokens = Tokens::get(ctx);
    let (max_window_size, body_height) = result_import_dialog_geometry(ctx.content_rect().size());
    let max_window_width = max_window_size.x;
    let max_window_height = max_window_size.y;

    egui::Window::new("Import result dataset")
        .id(egui::Id::new("rspice.result-import"))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(true)
        .default_width(860.0_f32.min(max_window_width))
        .min_width(680.0_f32.min(max_window_width))
        .max_width(max_window_width)
        .min_height(520.0_f32.min(max_window_height))
        .max_height(max_window_height)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("rspice.result-import.body")
                .max_height(body_height)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("RESULTS · EXTERNAL DATASET")
                            .monospace()
                            .color(tokens.color.text_dim),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        "Review the detected structure and mapping before RSpice adds these results to the project history.",
                    );
                    ui.add_space(10.0);
                    result_import_stage_header(ui, draft.stage);
                    ui.separator();

                    match draft.stage {
                        ResultImportStage::Detect => result_import_detect_page(ui, &draft),
                        ResultImportStage::Map => result_import_map_page(ui, &mut draft),
                        ResultImportStage::Validate => result_import_validate_page(ui, &draft),
                    }

                    ui.add_space(8.0);
                    if let Some(error) = draft.validation_error.as_deref() {
                        ui.colored_label(tokens.color.err, error);
                    }
                });

            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                match draft.stage {
                    ResultImportStage::Detect => {
                        advance = ui.button("Map signals").clicked();
                    }
                    ResultImportStage::Map => {
                        let valid = parsed_result_from_draft(&draft).is_ok();
                        advance = ui
                            .add_enabled(valid, egui::Button::new("Validate import"))
                            .clicked();
                        go_back = ui.button("Back").clicked();
                    }
                    ResultImportStage::Validate => {
                        let valid = parsed_result_from_draft(&draft).is_ok()
                            && result_import_block_reason(state.model, state.platform).is_none();
                        commit = ui
                            .add_enabled(valid, egui::Button::new("Import"))
                            .clicked();
                        go_back = ui.button("Back").clicked();
                    }
                }
                cancel = ui.button("Cancel").clicked();
            });
        });

    if !window_open || cancel {
        discard_result_import_draft(state);
        return;
    }
    if go_back {
        draft.stage = match draft.stage {
            ResultImportStage::Detect | ResultImportStage::Map => ResultImportStage::Detect,
            ResultImportStage::Validate => ResultImportStage::Map,
        };
        draft.validation_error = None;
    } else if advance {
        draft.stage = match draft.stage {
            ResultImportStage::Detect => ResultImportStage::Map,
            ResultImportStage::Map => ResultImportStage::Validate,
            ResultImportStage::Validate => ResultImportStage::Validate,
        };
        draft.validation_error = parsed_result_from_draft(&draft).err();
    }

    *state.result_import = draft;
    if let Some(Err(error)) = commit.then(|| start_result_import_commit(state)) {
        state.result_import.open = true;
        state.result_import.stage = ResultImportStage::Validate;
        state.result_import.validation_error = Some(error);
    }
}

/// The dialog while its job runs: what it is doing, how far it is, and
/// Cancel; and, when the job ends, what follows from how it ended. Reads the
/// job's progress and nothing it works on.
fn show_result_import_job(
    ctx: &egui::Context,
    state: &mut ResultsParts<'_>,
    job: &ResultImportJob,
) {
    if !poll_result_import_job(state, job) {
        return;
    }
    let progress = job.state();
    let tokens = Tokens::get(ctx);
    let mut cancel = false;
    egui::Window::new("Import result dataset")
        .id(egui::Id::new("rspice.result-import"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(&state.result_import.source_name)
                    .monospace()
                    .color(tokens.color.text_dim),
            );
            let step = match &progress {
                JobState::Running { step, .. } if !step.is_empty() => step.as_str(),
                _ => "Starting",
            };
            let fraction = progress.fraction();
            let text = match fraction {
                Some(fraction) => format!("{step} \u{00b7} {:.0}%", fraction * 100.0),
                None => step.to_owned(),
            };
            ui.add(
                egui::ProgressBar::new(fraction.unwrap_or(0.0))
                    .desired_width(360.0)
                    .text(text),
            );
            ui.add_space(8.0);
            let row = egui::vec2(ui.available_width(), ui.spacing().interact_size.y);
            ui.allocate_ui_with_layout(
                row,
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| cancel = ui.button("Cancel").clicked(),
            );
        });
    if cancel {
        job.cancel();
        match job {
            ResultImportJob::Reading(_) => discard_result_import_draft(state),
            ResultImportJob::Committing(_) => state.result_import.job = None,
        }
        return;
    }
    ctx.request_repaint_after(std::time::Duration::from_millis(100));
}

fn result_import_dialog_geometry(available: egui::Vec2) -> (egui::Vec2, f32) {
    let maximum = egui::vec2(
        (available.x - RESULT_IMPORT_WINDOW_MARGIN).max(1.0),
        (available.y - RESULT_IMPORT_WINDOW_MARGIN).max(1.0),
    );
    let body_height = (maximum.y - RESULT_IMPORT_FOOTER_RESERVE).max(1.0);
    (maximum, body_height)
}

/// Act on how the draft's job ended, once it has: review the staged source,
/// add the committed run, or say why it failed. Whether the job still runs.
fn poll_result_import_job(state: &mut ResultsParts<'_>, job: &ResultImportJob) -> bool {
    match job {
        ResultImportJob::Reading(reading) => match reading.take_outcome() {
            None => return true,
            Some(JobOutcome::Done(staged)) => open_review_draft(state, staged),
            Some(JobOutcome::Failed(reason)) => {
                discard_result_import_draft(state);
                record_import_failure(state, &reason);
            }
            Some(JobOutcome::Cancelled) => discard_result_import_draft(state),
        },
        ResultImportJob::Committing(committing) => {
            let Some(outcome) = committing.take_outcome() else {
                return true;
            };
            state.result_import.job = None;
            match outcome {
                JobOutcome::Done(kept) => match insert_kept_import(state, kept) {
                    Ok(()) => *state.result_import = ResultImportDialogState::default(),
                    Err(error) => state.result_import.validation_error = Some(error),
                },
                JobOutcome::Failed(error) => state.result_import.validation_error = Some(error),
                JobOutcome::Cancelled => {}
            }
        }
    }
    false
}

/// Wait for the draft's job and act on how it ended, as the dialog's next
/// frame would.
#[cfg(test)]
pub(crate) fn finish_result_import_job(state: &mut ResultsParts<'_>) {
    if let Some(job) = state.result_import.job.clone() {
        let ended = match &job {
            ResultImportJob::Reading(reading) => reading.wait(std::time::Duration::from_secs(600)),
            ResultImportJob::Committing(committing) => {
                committing.wait(std::time::Duration::from_secs(600))
            }
        };
        assert!(ended, "the import job ends");
        assert!(!poll_result_import_job(state, &job));
    }
}

/// Close the dialog and drop its draft, stopping any work it runs: what the
/// work kept is deleted with it.
pub fn discard_result_import_draft(state: &mut ResultsParts<'_>) {
    if let Some(job) = &state.result_import.job {
        job.cancel();
    }
    *state.result_import = ResultImportDialogState::default();
}

fn result_import_stage_header(ui: &mut egui::Ui, active: ResultImportStage) {
    ui.horizontal(|ui| {
        for (index, (stage, label)) in [
            (ResultImportStage::Detect, "1  Detect"),
            (ResultImportStage::Map, "2  Map"),
            (ResultImportStage::Validate, "3  Validate"),
        ]
        .into_iter()
        .enumerate()
        {
            let mut label = egui::RichText::new(label);
            if stage == active {
                label = label.strong();
            }
            if stage == active {
                ui.label(label.color(Tokens::get(ui.ctx()).color.accent));
            } else {
                ui.label(label.color(Tokens::get(ui.ctx()).color.text_dim));
            }
            if index < 2 {
                ui.label(egui::RichText::new("›").color(Tokens::get(ui.ctx()).color.text_dim));
            }
        }
    });
}

fn result_import_detect_page(ui: &mut egui::Ui, draft: &ResultImportDialogState) {
    ui.heading("Detected source");
    egui::Grid::new("rspice.result-import.detected")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            result_import_summary_row(ui, "File", &draft.source_name);
            result_import_summary_row(ui, "Format", &draft.source_format_id);
            if matches!(draft.source_format_id.as_str(), "csv-rfc4180" | "tsv") {
                result_import_summary_row(
                    ui,
                    "Delimiter",
                    if draft.delimiter == b'\t' {
                        "Tab"
                    } else {
                        "Comma"
                    },
                );
            }
            result_import_summary_row(
                ui,
                "Inferred domain",
                analysis_domain_label(draft.analysis_type),
            );
            result_import_summary_row(ui, "Coordinate", &draft.coordinate_name);
            result_import_summary_row(
                ui,
                "Coordinate unit",
                draft.coordinate_unit.as_deref().unwrap_or("Unstated"),
            );
            result_import_summary_row(ui, "Signals", &draft.waveforms.len().to_string());
            result_import_summary_row(ui, "Samples per signal", &draft.sample_count.to_string());
            result_import_plot_rows(ui, draft);
        });
    ui.add_space(12.0);
    ui.label(
        "No run or analysis has been created. Continue to confirm the engineering domain, coordinate, and included signals.",
    );
}

fn result_import_map_page(ui: &mut egui::Ui, draft: &mut ResultImportDialogState) {
    ui.heading("Map engineering data");
    if let Some(plots) = draft.plots.as_deref() {
        ui.label(
            egui::RichText::new(format!(
                "Mapping {}. The source's other analyses import as written.",
                plots.reviewed
            ))
            .small()
            .color(Tokens::get(ui.ctx()).color.text_dim),
        );
    }
    egui::Grid::new("rspice.result-import.mapping")
        .num_columns(2)
        .spacing([18.0, 8.0])
        .show(ui, |ui| {
            ui.label("Analysis domain");
            let domain_locked = matches!(
                draft.source_format_id.as_str(),
                "touchstone-v1" | "touchstone-v2"
            );
            ui.add_enabled_ui(!domain_locked, |ui| {
                crate::ui::widgets::Combo::from_id_salt("rspice.result-import.domain")
                    .selected_text(analysis_domain_label(draft.analysis_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut draft.analysis_type,
                            AnalysisType::Transient,
                            "Transient",
                        );
                        ui.selectable_value(&mut draft.analysis_type, AnalysisType::Ac, "AC");
                        ui.selectable_value(
                            &mut draft.analysis_type,
                            AnalysisType::DcSweep,
                            "DC sweep",
                        );
                    });
            });
            ui.end_row();
            ui.label("Coordinate name");
            ui.add(
                egui::TextEdit::singleline(&mut draft.coordinate_name)
                    .id_salt("rspice.result-import.coordinate")
                    .desired_width(320.0),
            );
            ui.end_row();
        });

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.strong("Signals");
        if ui.small_button("Select all").clicked() {
            draft.selected_signals.fill(true);
        }
        if ui.small_button("Select none").clicked() {
            draft.selected_signals.fill(false);
        }
        ui.label(
            egui::RichText::new(format!(
                "{} of {} selected",
                draft
                    .selected_signals
                    .iter()
                    .filter(|selected| **selected)
                    .count(),
                draft.selected_signals.len()
            ))
            .small()
            .color(Tokens::get(ui.ctx()).color.text_dim),
        );
    });
    egui::ScrollArea::vertical()
        .id_salt("rspice.result-import.signals")
        .max_height(132.0)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (waveform, selected) in draft
                .waveforms
                .iter()
                .zip(draft.selected_signals.iter_mut())
            {
                ui.checkbox(selected, result_import_signal_label(waveform));
            }
        });

    ui.add_space(10.0);
    ui.strong("Source preview");
    result_import_preview(ui, draft);
    draft.validation_error = parsed_result_from_draft(draft).err();
}

fn result_import_validate_page(ui: &mut egui::Ui, draft: &ResultImportDialogState) {
    ui.heading("Validate import");
    let selected = draft
        .selected_signals
        .iter()
        .filter(|selected| **selected)
        .count();
    egui::Grid::new("rspice.result-import.validation")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            result_import_summary_row(ui, "Source", &draft.source_name);
            result_import_summary_row(ui, "Format", &draft.source_format_id);
            result_import_summary_row(ui, "Domain", analysis_domain_label(draft.analysis_type));
            result_import_summary_row(ui, "Coordinate", &draft.coordinate_name);
            result_import_summary_row(
                ui,
                "Coordinate unit",
                draft.coordinate_unit.as_deref().unwrap_or("Unstated"),
            );
            result_import_summary_row(ui, "Included signals", &selected.to_string());
            result_import_summary_row(ui, "Samples per signal", &draft.sample_count.to_string());
            result_import_plot_rows(ui, draft);
            result_import_summary_row(ui, "Saved as", "Read-only · imported, not from a run");
        });
    ui.add_space(12.0);
    for note in &draft.notes {
        ui.colored_label(Tokens::get(ui.ctx()).color.warn, note);
    }
    if !draft.notes.is_empty() {
        ui.add_space(8.0);
    }
    match parsed_result_from_draft(draft) {
        Ok(_) => {
            let message = match draft.plots.as_deref() {
                Some(plots) => format!(
                    "Validation passed. Import will add one completed retained run of {} analyses and open it in Results.",
                    plots.further.len() + 1
                ),
                None => "Validation passed. Import will add one completed retained run and open it in Results."
                    .to_owned(),
            };
            ui.colored_label(Tokens::get(ui.ctx()).color.ok, message);
        }
        Err(error) => {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
    }
    ui.add_space(10.0);
    result_import_preview(ui, draft);
}

/// The rows that say which analysis of a multi-analysis source is reviewed
/// and what else the import brings in beside it.
fn result_import_plot_rows(ui: &mut egui::Ui, draft: &ResultImportDialogState) {
    let Some(plots) = draft.plots.as_deref() else {
        return;
    };
    result_import_summary_row(ui, "Reviewed analysis", &plots.reviewed);
    for plot in &plots.further {
        result_import_summary_row(
            ui,
            "Also imported",
            &format!(
                "{} · {} signals \u{00d7} {} samples, as written",
                plot.name,
                plot.waveforms.len(),
                plot.sample_count
            ),
        );
    }
}

fn result_import_summary_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(egui::RichText::new(label).strong());
    ui.label(value);
    ui.end_row();
}

fn result_import_signal_label(waveform: &RetainedTrace) -> String {
    waveform.unit.as_deref().map_or_else(
        || waveform.name.clone(),
        |unit| format!("{}  [{}]", waveform.name, unit),
    )
}

fn result_import_preview(ui: &mut egui::Ui, draft: &ResultImportDialogState) {
    const MAX_PREVIEW_ROWS: usize = RESULT_IMPORT_PREVIEW_ROWS;
    const MAX_PREVIEW_SIGNALS: usize = 8;

    let facts = sample_facts_of(draft);
    let included = draft
        .waveforms
        .iter()
        .zip(&facts.first_rows)
        .zip(&draft.selected_signals)
        .filter_map(|(waveform, selected)| selected.then_some(waveform))
        .take(MAX_PREVIEW_SIGNALS)
        .collect::<Vec<_>>();
    if included.is_empty() {
        ui.label(
            egui::RichText::new("Select at least one signal to preview.")
                .color(Tokens::get(ui.ctx()).color.warn),
        );
        return;
    }

    egui::ScrollArea::horizontal()
        .id_salt("rspice.result-import.preview-scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("rspice.result-import.preview")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.strong(&draft.coordinate_name);
                    for (waveform, _) in &included {
                        ui.strong(result_import_signal_label(waveform));
                    }
                    ui.end_row();
                    let rows = included[0].1.len().min(draft.sample_count.max(1));
                    for row in 0..rows.min(MAX_PREVIEW_ROWS) {
                        ui.monospace(format!("{:.6e}", included[0].1[row].0));
                        for (_, first_rows) in &included {
                            ui.monospace(
                                first_rows
                                    .get(row)
                                    .and_then(|(_, sample)| *sample)
                                    .map(|value| format!("{value:.6e}"))
                                    .unwrap_or_else(|| "Unavailable".into()),
                            );
                        }
                        ui.end_row();
                    }
                });
        });
    if draft.sample_count > MAX_PREVIEW_ROWS || included.len() < draft.waveforms.len() {
        ui.label(
            egui::RichText::new(format!(
                "Preview is limited to {MAX_PREVIEW_ROWS} rows and {MAX_PREVIEW_SIGNALS} included signals."
            ))
            .small()
            .color(Tokens::get(ui.ctx()).color.text_dim),
        );
    }
}

fn analysis_domain_label(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::Transient => "transient",
        AnalysisType::Ac => "AC",
        AnalysisType::DcSweep => "DC sweep",
        AnalysisType::SParameter => "S-parameter",
        _ => "unsupported",
    }
}

pub fn parse_result_dataset(
    source_name: &str,
    bytes: &[u8],
) -> Result<ParsedResultDataset, String> {
    parse_result_dataset_beside(source_name, bytes, None)
}

/// [`parse_result_dataset`], with the simulator's log that sat beside the
/// file: a rawfile's log names the runs of an LTspice `.step`.
fn parse_result_dataset_beside(
    source_name: &str,
    bytes: &[u8],
    companion_log: Option<&[u8]>,
) -> Result<ParsedResultDataset, String> {
    if bytes.is_empty() {
        return Err("the selected file is empty".to_owned());
    }
    let _source = ReadingSource::of(bytes.len());
    let format =
        identify_result_import_format(source_name, bytes).map_err(|error| error.to_string())?;
    match format {
        ResultImportFormat::CsvRfc4180 | ResultImportFormat::Tsv => {
            let text = std::str::from_utf8(bytes)
                .map_err(|error| format!("the selected file is not valid UTF-8: {error}"))?;
            let delimiter = if format == ResultImportFormat::Tsv {
                b'\t'
            } else {
                b','
            };
            let mut parsed = parse_delimited_result_dataset(text, delimiter)?;
            parsed.source_format = format;
            Ok(parsed)
        }
        ResultImportFormat::TouchstoneV1 | ResultImportFormat::TouchstoneV2 => {
            parse_touchstone_result_dataset(source_name, bytes, format)
        }
        ResultImportFormat::RSpiceResultBundle | ResultImportFormat::RSpiceDatasetBundle => {
            adapters::parse_native_bundle(bytes, format)
        }
        ResultImportFormat::Hdf5 => adapters::parse_hdf5(bytes, format),
        ResultImportFormat::ArrowIpc => adapters::parse_arrow_ipc(bytes, format),
        ResultImportFormat::Parquet => adapters::parse_parquet(bytes, format),
        ResultImportFormat::NumpyNpy => adapters::parse_npy(bytes, format),
        ResultImportFormat::NumpyNpz => adapters::parse_npz(bytes, format),
        ResultImportFormat::MatlabV5 => adapters::parse_matlab_v5(bytes, format),
        ResultImportFormat::MatlabV73 => adapters::parse_matlab_v73(bytes, format),
        ResultImportFormat::SpiceRaw => adapters::parse_spice_raw(bytes, companion_log, format),
        ResultImportFormat::PsfAscii => adapters::parse_psf_ascii(bytes, format),
        ResultImportFormat::Vcd => adapters::parse_vcd(bytes, format),
        ResultImportFormat::Fst => adapters::parse_fst(bytes, format),
    }
}

fn parse_touchstone_result_dataset(
    source_name: &str,
    bytes: &[u8],
    identified_format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded = rspice_formats::waveform_io::result::decode_touchstone(
        source_name,
        bytes,
        identified_format,
    )
    .map_err(|error| error.to_string())?;
    let mut parsed = adapters::present_imported_waveforms(
        decoded.source_format,
        AnalysisType::SParameter,
        decoded.data,
    );
    parsed.family_metadata = Some(decoded.family_metadata);
    parsed.coordinate_unit = Some("Hz".to_owned());
    parsed.notes = decoded.notes;
    Ok(parsed)
}

fn parse_delimited_result_dataset(
    text: &str,
    delimiter: u8,
) -> Result<ParsedResultDataset, String> {
    use rspice_formats::delimited::layout::{ColumnKind, complex_pair_name};
    use rspice_formats::delimited::{
        DecodedDelimitedWaveforms, DelimitedReadLimits, decode_delimited_waveforms,
    };

    let DecodedDelimitedWaveforms {
        domain,
        columns: headers,
        coordinate,
        signal_values,
        kinds,
    } = decode_delimited_waveforms(
        text,
        delimiter,
        DelimitedReadLimits {
            max_columns: source_capacity(),
            max_rows: source_capacity(),
            max_header_bytes: MAX_HEADER_BYTES,
            min_rows: MIN_RESULT_ROWS,
        },
    )
    .map_err(|error| error.to_string())?;
    let analysis_type = imported_analysis_type(domain);
    let coordinate = Arc::new(coordinate);
    // A complex signal's two parts come back as the one signal they were: its magnitude drawn,
    // its parts kept exact.
    let mut waveforms = Vec::with_capacity(signal_values.len());
    let mut values = signal_values.into_iter().enumerate();
    while let Some((index, column)) = values.next() {
        let header = &headers[index + 1];
        let unit = header.canonical_unit().map(str::to_owned);
        // No stored colour: the canvas theme's palette colours an imported trace by its slot,
        // as it does a simulated one.
        let mut waveform = if kinds.get(index) == Some(&ColumnKind::ComplexReal)
            && let Some(name) = headers
                .get(index + 2)
                .and_then(|imag| complex_pair_name(&header.name, &imag.name))
            && let Some((_, imag)) = values.next()
        {
            let magnitude = column
                .iter()
                .zip(&imag)
                .map(|(real, imag)| real.hypot(*imag))
                .collect::<Vec<_>>();
            RetainedTrace::new(format!("|{name}|"), Arc::clone(&coordinate), magnitude, "")
                .with_complex_components(name, column, imag)
        } else {
            RetainedTrace::new(header.name.clone(), Arc::clone(&coordinate), column, "")
        };
        waveform.unit = unit;
        waveforms.push(waveform);
    }
    Ok(ParsedResultDataset {
        source_format: if delimiter == b'\t' {
            ResultImportFormat::Tsv
        } else {
            ResultImportFormat::CsvRfc4180
        },
        analysis_type,
        coordinate_name: headers[0].name.clone(),
        coordinate_unit: headers[0].canonical_unit().map(str::to_owned),
        sample_count: coordinate.len(),
        waveforms,
        family_metadata: None,
        delimiter,
        notes: Vec::new(),
        event_payload: None,
        event_dump: None,
        plots: None,
    })
}

fn imported_analysis_type(domain: rspice_formats::WaveformDomain) -> AnalysisType {
    match domain {
        rspice_formats::WaveformDomain::Transient => AnalysisType::Transient,
        rspice_formats::WaveformDomain::Ac => AnalysisType::Ac,
        rspice_formats::WaveformDomain::DcSweep => AnalysisType::DcSweep,
    }
}

#[cfg(target_arch = "wasm32")]
enum BrowserResultDatasetImportResult {
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
struct BrowserResultDatasetImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: BrowserResultDatasetImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_RESULT_DATASET_IMPORT_RESULT:
        std::cell::RefCell<Option<BrowserResultDatasetImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_result_dataset_import() -> Result<(), String> {
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::ResultDataset,
    )?;
    crate::workbench::browser::file_import::pick_text_file(
        RESULT_DATASET_FILTER.0,
        RESULT_DATASET_FILTER.1,
        move |result| {
            if !crate::workbench::browser::file_import::text_import_is_current(token) {
                return;
            }
            let result = match result {
                Ok(Some(file)) => BrowserResultDatasetImportResult::Loaded(file),
                Ok(None) => BrowserResultDatasetImportResult::Cancelled,
                Err(error) => BrowserResultDatasetImportResult::Failed(error),
            };
            BROWSER_RESULT_DATASET_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserResultDatasetImportCompletion { token, result });
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn poll_browser_result_dataset_import(state: &mut ResultsParts<'_>) -> bool {
    let Some(completion) =
        BROWSER_RESULT_DATASET_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return false;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return false;
    }
    match completion.result {
        BrowserResultDatasetImportResult::Loaded(file) => {
            let bytes = file
                .original_bytes
                .as_deref()
                .unwrap_or(file.contents.as_bytes());
            match stage_imported_result_dataset(state, &file.name, bytes) {
                Ok(()) => true,
                Err(error) => {
                    record_import_failure(state, &error);
                    false
                }
            }
        }
        BrowserResultDatasetImportResult::Failed(error) => {
            record_import_failure(state, &error);
            false
        }
        BrowserResultDatasetImportResult::Cancelled => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A refused file says why in a notice that offers its Console line, as
    /// a failed export does. The reason went to the Console alone, so the
    /// import looked as if nothing had happened.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_refused_result_import_raises_a_notice_with_the_reason() {
        let folder =
            std::env::temp_dir().join(format!("rspice-refused-import-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("scope.csv");
        std::fs::write(&path, b"").unwrap();
        let mut state =
            crate::workbench::documents::result_document::test_host::ResultsTestHost::default();

        let imported = import_result_dataset_from_path(&mut state.results_parts(), &path);
        finish_result_import_job(&mut state.results_parts());
        std::fs::remove_dir_all(&folder).unwrap();

        assert!(imported, "the read starts");
        assert!(
            !state.results_parts().result_import.open,
            "a refused source closes the dialog"
        );
        let notices = state.model.notices.take();
        assert_eq!(notices.len(), 1, "{notices:?}");
        assert_eq!(notices[0].title, "Result dataset import failed");
        assert_eq!(notices[0].detail, "the selected file is empty");
        assert!(state.model.simulation.retained.runs.is_empty());
    }

    #[test]
    fn import_dialog_reserves_a_visible_footer_inside_desktop_viewports() {
        for available in [egui::vec2(1_440.0, 900.0), egui::vec2(1_024.0, 768.0)] {
            let (maximum, body) = result_import_dialog_geometry(available);
            assert_eq!(maximum, available - egui::vec2(24.0, 24.0));
            assert!(body > 0.0);
            assert!(body + RESULT_IMPORT_FOOTER_RESERVE <= maximum.y);
            assert!(maximum.x <= available.x);
            assert!(maximum.y <= available.y);
        }
    }

    #[test]
    fn parses_utf8_csv_transient_with_units_and_shared_coordinate() {
        let parsed = parse_result_dataset(
            "scope.csv",
            b"time [ms],V(out) [mV],I(VDD) [uA]\n0,0,10\n0.5,1250,11\n1,2500,12\n",
        )
        .expect("valid transient CSV");

        assert_eq!(parsed.analysis_type, AnalysisType::Transient);
        assert_eq!(parsed.sample_count, 3);
        assert_eq!(parsed.waveforms.len(), 2);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[0.0, 5e-4, 1e-3]);
        assert_eq!(parsed.waveforms[0].y.as_slice(), &[0.0, 1.25, 2.5]);
        assert_eq!(parsed.waveforms[1].y.as_slice(), &[10e-6, 11e-6, 12e-6]);
        assert_eq!(parsed.waveforms[0].unit.as_deref(), Some("V"));
        assert_eq!(parsed.waveforms[1].unit.as_deref(), Some("A"));
        assert!(rspice_results::waveform::WaveformValues::ptr_eq(
            &parsed.waveforms[0].x,
            &parsed.waveforms[1].x
        ));
        assert_eq!(parsed.source_format, ResultImportFormat::CsvRfc4180);
        assert!(parsed.family_metadata.is_none());
    }

    #[test]
    fn format_registry_matches_the_seventeen_contract_import_ids() {
        let ids = ResultImportFormat::ALL.map(ResultImportFormat::canonical_id);
        assert_eq!(
            ids,
            [
                "rspice-result-bundle",
                "rspice-dataset-bundle",
                "csv-rfc4180",
                "tsv",
                "touchstone-v1",
                "touchstone-v2",
                "hdf5",
                "arrow-ipc",
                "parquet",
                "numpy-npy",
                "numpy-npz",
                "matlab-v5",
                "matlab-v7.3",
                "spice-raw",
                "psf-ascii",
                "vcd",
                "fst",
            ]
        );
        for extension in [
            "rspiceresult",
            "rspicedata",
            "csv",
            "tsv",
            "tab",
            "s1p",
            "s2p",
            "s3p",
            "s4p",
            "snp",
            "ts",
            "h5",
            "hdf5",
            "arrow",
            "feather",
            "parquet",
            "npy",
            "npz",
            "mat",
            "raw",
            "psfascii",
            "vcd",
            "fst",
        ] {
            assert!(RESULT_DATASET_FILTER.1.contains(&extension), "{extension}");
        }
    }

    #[test]
    fn structured_formats_reject_malformed_sources_and_signatures_defeat_spoofed_extensions() {
        for (name, bytes, id) in [
            ("samples.npy", b"\x93NUMPY\x01\x00".as_slice(), "numpy-npy"),
            ("table.parquet", b"PAR1payloadPAR1".as_slice(), "parquet"),
            (
                "capture.vcd",
                b"$timescale 1ns $end\n$scope module top $end\n".as_slice(),
                "vcd",
            ),
        ] {
            let error = parse_result_dataset(name, bytes).expect_err("malformed source rejects");
            assert!(error.contains(id), "{error}");
        }

        let error = parse_result_dataset("spoofed.csv", b"PAR1payloadPAR1")
            .expect_err("extension/signature mismatch must reject");
        assert!(error.contains("csv-rfc4180"), "{error}");
        assert!(error.contains("parquet"), "{error}");
        assert!(error.contains("ambiguous"), "{error}");
    }

    #[test]
    fn parses_tsv_frequency_as_ac_and_requires_positive_ordered_frequency() {
        let parsed = parse_result_dataset(
            "response.tsv",
            b"frequency [kHz]\tV(out) [V]\n1\t0.5\n10\t0.25\n100\t0.1\n",
        )
        .expect("valid AC TSV");
        assert_eq!(parsed.analysis_type, AnalysisType::Ac);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[1e3, 1e4, 1e5]);

        let error = parse_result_dataset("response.csv", b"frequency [Hz],V(out) [V]\n1,1\n0,2\n")
            .expect_err("zero frequency rejected");
        assert!(error.contains("greater than zero"));
    }

    #[test]
    fn infers_domain_from_unambiguous_coordinate_units() {
        let transient = parse_result_dataset("capture.csv", b"sample [us],V(out) [V]\n0,0\n1,1\n")
            .expect("time unit identifies transient data");
        assert_eq!(transient.analysis_type, AnalysisType::Transient);

        let ac = parse_result_dataset("capture.csv", b"axis [MHz],V(out) [V]\n1,1\n2,0.5\n")
            .expect("frequency unit identifies AC data");
        assert_eq!(ac.analysis_type, AnalysisType::Ac);
    }

    #[test]
    fn infers_dc_sweep_and_accepts_strictly_decreasing_coordinate() {
        let parsed = parse_result_dataset("sweep.csv", b"V1 [V],V(out) [V]\n5,4.5\n2.5,2.2\n0,0\n")
            .expect("decreasing DC sweep remains ordered");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[5.0, 2.5, 0.0]);
    }

    #[test]
    fn preserves_case_sensitive_siemens_units() {
        let parsed = parse_result_dataset("conductance.csv", b"bias [V],g(out) [mS]\n0,1\n1,2\n")
            .expect("mS is conductance, not milliseconds");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.waveforms[0].y.as_slice(), &[1e-3, 2e-3]);
    }

    #[test]
    fn rejects_duplicate_headers_unknown_units_nonfinite_values_and_reversals() {
        for (source, expected) in [
            (
                b"time [s],V(out) [V],v(OUT) [mV]\n0,0,0\n1,1,1\n".as_slice(),
                "repeats",
            ),
            (
                b"time [fortnight],V(out) [V]\n0,0\n1,1\n".as_slice(),
                "not a recognized engineering unit",
            ),
            (
                b"time [s],V(out) [V]\n0,NaN\n1,1\n".as_slice(),
                "must be finite",
            ),
            (
                b"time [s],V(out) [V]\n0,0\n2,1\n1,2\n".as_slice(),
                "reverses coordinate ordering",
            ),
        ] {
            let error = parse_result_dataset("bad.csv", source).expect_err("invalid import");
            assert!(error.contains(expected), "{error:?}");
        }
    }

    /// One sample is a result; none is not.
    ///
    /// `MIN_RESULT_ROWS` was two, so a single operating point — the one
    /// analysis whose result is always one sample — could be exported by this
    /// product and not reopened by it. A header with no rows under it is the
    /// other end of the same rule, and it is refused where the file is read
    /// rather than by a later step failing for a reason that is not the real
    /// one.
    #[test]
    fn one_sample_is_a_result_and_a_header_with_no_rows_is_refused_by_name() {
        let parsed = parse_result_dataset("op.csv", b"time [s],V(out) [V]\n0,2.5\n")
            .expect("a single operating point is a result");
        assert_eq!(parsed.sample_count, 1);
        assert_eq!(parsed.waveforms[0].y.as_slice(), [2.5]);

        let error = parse_result_dataset("empty.csv", b"time [s],V(out) [V]\n")
            .expect_err("a header with no rows holds no result");
        assert!(error.contains("carries no samples"), "{error:?}");
        assert!(
            error.contains("time"),
            "the refusal names the column: {error:?}"
        );
    }

    /// A binary rawfile of `rows` points of time and seven signals, 64 bytes a
    /// row, written to `path`; returns the generator every value came from.
    #[cfg(not(target_arch = "wasm32"))]
    fn write_generated_rawfile(path: &Path, rows: usize) -> impl Fn(usize, usize) -> f64 {
        use std::io::Write as _;
        let value = |row: usize, signal: usize| -> f64 {
            if signal == 0 {
                return row as f64 * 1.0e-9;
            }
            // SplitMix64 of the row and signal, as a value in [-1, 1).
            let mut z = (row as u64 * 8 + signal as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^= z >> 31;
            (z >> 11) as f64 / (1_u64 << 52) as f64 - 1.0
        };
        let mut file = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
        write!(
            file,
            "Title: generated\nDate: generated\nPlotname: Transient Analysis\n\
             Flags: real forward double\nNo. Variables: 8\nNo. Points: {rows}\n\
             Variables:\n\t0\ttime\ttime\n"
        )
        .unwrap();
        for signal in 1..8 {
            writeln!(file, "\t{signal}\tV(n{signal})\tvoltage").unwrap();
        }
        writeln!(file, "Binary:").unwrap();
        for row in 0..rows {
            for signal in 0..8 {
                file.write_all(&value(row, signal).to_le_bytes()).unwrap();
            }
        }
        file.flush().unwrap();
        value
    }

    /// Import a generated rawfile of `rows` points through the job: its read
    /// reports progress in the file's bytes, a cancel after the first chunk
    /// leaves no draft and no run, and the whole file stages, keeps its
    /// samples in segments and commits as a run holding every sample.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::print_stderr)] // the times are the measurement a big run is made for
    fn import_a_generated_rawfile(rows: usize) {
        use crate::workbench::results_job::cancel_jobs_past_for_tests;
        let folder = std::env::temp_dir().join(format!(
            "rspice-generated-raw-{}-{rows}",
            std::process::id()
        ));
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("generated.raw");
        let value = write_generated_rawfile(&path, rows);
        let size = std::fs::metadata(&path).unwrap().len();

        let job = JobContext::detached();
        let started = std::time::Instant::now();
        let bytes = read_native_file(&path, &job).unwrap();
        let read = started.elapsed();
        assert_eq!(job.reported(), ("Reading".to_owned(), size, size));
        assert_eq!(bytes.len() as u64, size);
        drop(bytes);

        let mut state =
            crate::workbench::documents::result_document::test_host::ResultsTestHost::default();
        state.platform.project_lifecycle.authority.open_session();
        cancel_jobs_past_for_tests(Some(1));
        assert!(import_result_dataset_from_path(
            &mut state.results_parts(),
            &path
        ));
        cancel_jobs_past_for_tests(None);
        finish_result_import_job(&mut state.results_parts());
        assert!(!state.results_parts().result_import.open);
        assert!(
            state.model.notices.take().is_empty(),
            "a cancel is no failure"
        );
        assert!(state.model.simulation.retained.runs.is_empty());

        let started = std::time::Instant::now();
        assert!(import_result_dataset_from_path(
            &mut state.results_parts(),
            &path
        ));
        finish_result_import_job(&mut state.results_parts());
        let staged = started.elapsed();
        {
            let parts = state.results_parts();
            let draft = &parts.result_import;
            assert_eq!(draft.sample_count, rows);
            assert_eq!(draft.waveforms.len(), 7);
            assert!(
                draft
                    .waveforms
                    .iter()
                    .all(|waveform| waveform.y.is_stored())
            );
        }
        let started = std::time::Instant::now();
        start_result_import_commit(&mut state.results_parts()).unwrap();
        finish_result_import_job(&mut state.results_parts());
        let committed = started.elapsed();
        std::fs::remove_dir_all(&folder).unwrap();
        eprintln!(
            "{size} bytes: read in {read:?}, staged in {staged:?}, committed in {committed:?}"
        );

        let run = state
            .model
            .simulation
            .active_run()
            .expect("the import is shown");
        let signals = &run.analyses[0].waveforms;
        assert_eq!(signals.len(), 7);
        for (index, signal) in signals.iter().enumerate() {
            let last = signal.y.read_range(rows - 1..rows).unwrap()[0];
            assert_eq!(last.to_bits(), value(rows - 1, index + 1).to_bits());
        }
        let time = signals[0].x.read_range(rows - 1..rows).unwrap()[0];
        assert_eq!(time.to_bits(), value(rows - 1, 0).to_bits());
    }

    /// A 64 MiB rawfile, the size an import was once refused past.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_64_mib_rawfile_imports_through_the_job_and_cancels_cleanly() {
        import_a_generated_rawfile(1 << 20);
    }
}
