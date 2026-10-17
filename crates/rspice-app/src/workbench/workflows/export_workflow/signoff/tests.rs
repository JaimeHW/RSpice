//! Where the sign-off report is written, and what it is written from.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use super::*;
use crate::state::{ComponentType, Point};

/// A save dialog that answers `chosen`, and files written in memory.
struct Recorder {
    chosen: Option<PathBuf>,
    /// Files already there before the export.
    existing: Vec<PathBuf>,
    /// The answer to being asked to replace them.
    consent: bool,
    /// Each time the reader was asked, the files they were asked about.
    asked: RefCell<Vec<Vec<PathBuf>>>,
    written: RefCell<Vec<(PathBuf, String)>>,
}

impl Recorder {
    fn choosing(path: Option<&str>) -> Self {
        Self {
            chosen: path.map(PathBuf::from),
            existing: Vec::new(),
            consent: false,
            asked: RefCell::default(),
            written: RefCell::default(),
        }
    }

    /// Files at `paths` already, and the reader answering `consent` when
    /// asked to replace them.
    fn over(mut self, paths: &[&str], consent: bool) -> Self {
        self.existing = paths.iter().map(PathBuf::from).collect();
        self.consent = consent;
        self
    }

    fn written(&self, path: &str) -> Option<String> {
        self.written
            .borrow()
            .iter()
            .find(|(written, _)| written == Path::new(path))
            .map(|(_, contents)| contents.clone())
    }
}

impl ExportWorkflowIo for Recorder {
    fn show_save_dialog(&self, _config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        Ok(self.chosen.clone())
    }

    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        self.written
            .borrow_mut()
            .push((path.to_path_buf(), contents.to_owned()));
        Ok(())
    }

    fn confirm_replace(&self, _title: &str, paths: &[&Path]) -> bool {
        self.asked
            .borrow_mut()
            .push(paths.iter().map(|path| path.to_path_buf()).collect());
        self.consent
    }

    fn observe_destination(&self, path: &Path) -> Result<ObservedExportDestination, String> {
        Ok(if self.existing.iter().any(|existing| existing == path) {
            ObservedExportDestination::occupied(path.to_path_buf())
        } else {
            ObservedExportDestination::backend_managed(path.to_path_buf())
        })
    }

    fn write_waveform_csv(
        &self,
        _dataset: &crate::io::WaveformDataset,
        _path: &Path,
    ) -> Result<(), String> {
        Err("not a waveform export".to_owned())
    }
}

fn open_state() -> AppState {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    state
}

fn input_digest(summary: &str) -> Option<&str> {
    summary
        .lines()
        .find(|line| line.starts_with("Input digest"))
}

#[test]
fn the_summary_goes_where_the_reader_puts_it_and_the_detail_beside_it() {
    let mut state = open_state();
    let io = Recorder::choosing(Some("out/top_signoff.csv"));
    let message = write_signoff_report(&mut state, &io)
        .expect("the report is written")
        .expect("a file was chosen");
    let summary = io
        .written("out/top_signoff.summary")
        .expect("a summary named as the detail is renamed, not overwritten");
    let detail = io.written("out/top_signoff.csv").expect("the detail");
    assert!(summary.starts_with("RSpice design sign-off summary\n"));
    assert!(detail.starts_with("rule,severity,"));
    assert!(message.contains("top_signoff.csv"), "{message}");
}

#[test]
fn a_cancelled_dialog_writes_nothing() {
    let mut state = open_state();
    let io = Recorder::choosing(None);
    assert_eq!(write_signoff_report(&mut state, &io), Ok(None));
    assert!(io.written.borrow().is_empty());
}

/// The CSV beside the summary is a file the reader never chose. One already
/// there is replaced only if they agree when asked; if they do not, neither
/// file is written.
#[test]
fn an_existing_detail_is_never_replaced_without_consent() {
    let mut state = open_state();
    let io = Recorder::choosing(Some("out/top.summary")).over(&["out/top.csv"], false);
    assert_eq!(write_signoff_report(&mut state, &io), Ok(None));
    assert_eq!(*io.asked.borrow(), [vec![PathBuf::from("out/top.csv")]]);
    assert!(io.written.borrow().is_empty(), "{:?}", io.written.borrow());
}

#[test]
fn an_existing_detail_is_replaced_once_the_reader_agrees() {
    let mut state = open_state();
    let io = Recorder::choosing(Some("out/top.summary")).over(&["out/top.csv"], true);
    write_signoff_report(&mut state, &io)
        .expect("the report is written")
        .expect("a file was chosen");
    assert_eq!(io.asked.borrow().len(), 1, "asked once");
    assert!(io.written("out/top.csv").is_some());
    assert!(io.written("out/top.summary").is_some());
}

/// The save dialog asked about the file it returned, so that one is not
/// asked about again; a summary renamed from a chosen CSV name was never
/// asked about, so it is.
#[test]
fn only_files_the_dialog_did_not_return_are_asked_about() {
    let mut state = open_state();
    let io = Recorder::choosing(Some("out/top.summary")).over(&["out/top.summary"], false);
    write_signoff_report(&mut state, &io)
        .expect("the report is written")
        .expect("a file was chosen");
    assert!(io.asked.borrow().is_empty());
    assert!(io.written("out/top.summary").is_some());

    let io =
        Recorder::choosing(Some("out/top.csv")).over(&["out/top.csv", "out/top.summary"], false);
    assert_eq!(write_signoff_report(&mut state, &io), Ok(None));
    assert_eq!(*io.asked.borrow(), [vec![PathBuf::from("out/top.summary")]]);
    assert!(io.written.borrow().is_empty());
}

/// A report prepared before the design moved is not what is written: the
/// export checks the design again and writes what it finds now.
#[test]
fn a_design_that_moved_mid_export_is_checked_again_before_it_is_written() {
    let mut state = open_state();
    let prepared = state
        .signoff_report("2026-10-17T09:00:00Z")
        .expect("the report builds");
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(400, 400));

    let io = Recorder::choosing(Some("top.summary"));
    write_signoff_report(&mut state, &io)
        .expect("the report is written")
        .expect("a file was chosen");
    let written = io.written("top.summary").expect("the summary");
    assert_ne!(input_digest(&written), input_digest(&prepared.summary));
    let now = state
        .signoff_report("2026-10-17T09:00:00Z")
        .expect("the report builds");
    assert_eq!(input_digest(&written), input_digest(&now.summary));
    assert!(
        io.written("top.csv").expect("the detail").contains("R1"),
        "the resistor placed after the first report is in the one written"
    );
}
