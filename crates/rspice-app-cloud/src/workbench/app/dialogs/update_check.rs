//! Check for updates: whether rspice.app announces a newer RSpice than this
//! one, asked when the reader chooses the command and never otherwise.
//!
//! One card in four states: checking; up to date; a newer version, with
//! Download and Release notes, which open the pages the release names; and
//! failed, with why in the footer and Try again. The fetch runs on a thread
//! of its own and the card reads its answer as it draws, so the frame never
//! waits on the network.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use egui::{Context, RichText};

use crate::services::update_check::{
    self, CheckFailure, INSTALLED_VERSION, Release, is_newer, read_manifest,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize};

const TITLE: &str = "Check for updates";

/// How often the card looks for the check's answer while it waits.
const POLL: Duration = Duration::from_millis(100);

/// What one check answers: the manifest's bytes, or why there are none.
type Answer = Result<Vec<u8>, CheckFailure>;

/// Where a check gets the manifest's bytes: rspice.app, or the stand-in a
/// test gives it.
#[derive(Clone)]
pub struct ReleaseSource(Arc<dyn Fn() -> Answer + Send + Sync>);

impl ReleaseSource {
    pub fn new(fetch: impl Fn() -> Answer + Send + Sync + 'static) -> Self {
        Self(Arc::new(fetch))
    }
}

impl Default for ReleaseSource {
    /// rspice.app. A test build reaches no network: until a test gives it a
    /// source, its check fails as unreachable.
    fn default() -> Self {
        if cfg!(any(test, feature = "test-support")) {
            Self::new(|| Err(CheckFailure::Unreachable))
        } else {
            Self::new(update_check::fetch_manifest)
        }
    }
}

impl std::fmt::Debug for ReleaseSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ReleaseSource")
    }
}

/// Where the check stands.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum UpdateCheckPhase {
    /// Waiting for rspice.app.
    #[default]
    Checking,
    /// The installed version is the latest.
    UpToDate,
    /// rspice.app announces this newer release.
    Newer(Release),
    /// The check could not tell, for this reason.
    Failed(CheckFailure),
}

/// The Check for updates dialog: whether it is open, where its check
/// stands, and the answer of the check in flight.
#[derive(Debug, Clone, Default)]
pub struct UpdateCheckDialogState {
    pub open: bool,
    phase: UpdateCheckPhase,
    /// Where the check in flight leaves its answer. A check the reader
    /// walked away from keeps writing into a slot nothing reads.
    answer: Option<Arc<Mutex<Option<Answer>>>>,
    source: ReleaseSource,
}

impl UpdateCheckDialogState {
    pub fn phase(&self) -> &UpdateCheckPhase {
        &self.phase
    }

    /// Ask `source` instead of rspice.app from the next check on.
    #[cfg(any(test, feature = "test-support"))]
    pub fn use_source(&mut self, source: ReleaseSource) {
        self.source = source;
    }

    /// Start a check: the dialog shows Checking until its answer comes.
    fn start(&mut self) {
        let slot = Arc::new(Mutex::new(None));
        self.answer = Some(Arc::clone(&slot));
        self.phase = UpdateCheckPhase::Checking;
        let fetch = Arc::clone(&self.source.0);
        run_detached(slot, move || fetch());
    }

    /// Take the answer of the check in flight, once it has come.
    fn settle(&mut self) {
        let Some(slot) = &self.answer else {
            return;
        };
        let answer = slot.lock().ok().and_then(|mut answer| answer.take());
        if let Some(answer) = answer {
            self.answer = None;
            self.phase = phase_for(answer, INSTALLED_VERSION);
        }
    }

    fn close(&mut self) {
        self.open = false;
        self.answer = None;
    }
}

/// Run `fetch` on a thread of its own, leaving its answer in `slot`. Should
/// no thread start, the answer is that rspice.app could not be reached.
#[cfg(not(target_arch = "wasm32"))]
fn run_detached(slot: Arc<Mutex<Option<Answer>>>, fetch: impl FnOnce() -> Answer + Send + 'static) {
    let fallback = Arc::clone(&slot);
    let spawned = std::thread::Builder::new()
        .name("rspice-update-check".to_owned())
        .spawn(move || {
            let answer = fetch();
            if let Ok(mut slot) = slot.lock() {
                *slot = Some(answer);
            }
        });
    if spawned.is_err()
        && let Ok(mut slot) = fallback.lock()
    {
        *slot = Some(Err(CheckFailure::Unreachable));
    }
}

/// The browser has no threads to spare and never checks; a check asked of
/// it answers at once.
#[cfg(target_arch = "wasm32")]
fn run_detached(slot: Arc<Mutex<Option<Answer>>>, fetch: impl FnOnce() -> Answer + Send + 'static) {
    if let Ok(mut slot) = slot.lock() {
        *slot = Some(fetch());
    }
}

/// What a check's answer means for `installed`.
fn phase_for(answer: Answer, installed: &str) -> UpdateCheckPhase {
    match answer.and_then(|body| read_manifest(&body)) {
        Err(failure) => UpdateCheckPhase::Failed(failure),
        Ok(release) if is_newer(&release.version, installed) => UpdateCheckPhase::Newer(release),
        Ok(_) => UpdateCheckPhase::UpToDate,
    }
}

/// Open the dialog on a new check of rspice.app.
pub fn open_update_check(dialog: &mut UpdateCheckDialogState) {
    dialog.open = true;
    dialog.start();
}

/// Draw the dialog for this frame, when it is open, and carry out what the
/// reader chose in it.
pub fn render_update_check_dialog(dialog: &mut UpdateCheckDialogState, ctx: &Context) {
    if !dialog.open {
        return;
    }
    dialog.settle();
    if dialog.phase == UpdateCheckPhase::Checking {
        ctx.request_repaint_after(POLL);
    }
    let phase = dialog.phase.clone();
    let sentence = match &phase {
        UpdateCheckPhase::Checking => "Checking rspice.app for the latest version.".to_owned(),
        UpdateCheckPhase::UpToDate => format!("RSpice {INSTALLED_VERSION} is the latest version."),
        UpdateCheckPhase::Newer(release) => format!(
            "RSpice {} is available, released {}. You have {INSTALLED_VERSION}.",
            release.version, release.date
        ),
        UpdateCheckPhase::Failed(_) => {
            format!("RSpice {INSTALLED_VERSION} could not be compared with the latest version.")
        }
    };
    let primary = match &phase {
        UpdateCheckPhase::Checking => "Cancel",
        UpdateCheckPhase::UpToDate => "Close",
        UpdateCheckPhase::Newer(_) => "Download",
        UpdateCheckPhase::Failed(_) => "Try again",
    };
    let mut card = Dialog::new(TITLE, primary)
        .description("Whether rspice.app has a newer version of RSpice than this one.")
        .size(DialogSize::CommandForm)
        .overlay_scrollbar();
    match &phase {
        UpdateCheckPhase::Newer(_) => card = card.secondary("Release notes").ghost("Close"),
        UpdateCheckPhase::Failed(failure) => {
            card = card
                .ghost("Close")
                .hint(failure.sentence())
                .hint_tone(DialogHintTone::Error);
        }
        UpdateCheckPhase::Checking | UpdateCheckPhase::UpToDate => {}
    }
    let choice = card.show(ctx, |ui| {
        let t = Tokens::get(ui.ctx());
        ui.add(
            egui::Label::new(
                RichText::new(sentence)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text),
            )
            .wrap(),
        );
    });
    match (choice, phase) {
        (DialogChoice::None, _) => {}
        (DialogChoice::Primary, UpdateCheckPhase::Newer(release)) => {
            ctx.open_url(egui::OpenUrl::new_tab(release.download_url));
            dialog.close();
        }
        (DialogChoice::Secondary, UpdateCheckPhase::Newer(release)) => {
            ctx.open_url(egui::OpenUrl::new_tab(release.notes_url));
        }
        (DialogChoice::Primary, UpdateCheckPhase::Failed(_)) => dialog.start(),
        (DialogChoice::Secondary, _) => {}
        (DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled, _) => {
            dialog.close();
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const NEWER: &str = r#"{"version": "99.0.0", "date": "2026-11-01", "download_url": "https://rspice.app/download", "notes_url": "https://rspice.app/changelog#v99.0.0"}"#;

    /// A source that answers `body` and counts how often it was asked.
    fn counting(body: Answer) -> (ReleaseSource, Arc<AtomicUsize>) {
        let asked = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&asked);
        let source = ReleaseSource::new(move || {
            counter.fetch_add(1, Ordering::SeqCst);
            body.clone()
        });
        (source, asked)
    }

    fn themed() -> Context {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx
    }

    /// Draw frames until the check settles; the phase it settled in.
    fn settled(dialog: &mut UpdateCheckDialogState, ctx: &Context) -> UpdateCheckPhase {
        for _ in 0..500 {
            let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
                render_update_check_dialog(dialog, ui.ctx());
            });
            if dialog.phase != UpdateCheckPhase::Checking {
                return dialog.phase.clone();
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        panic!("the check never settled");
    }

    #[test]
    fn nothing_is_fetched_until_the_reader_asks() {
        let (source, asked) = counting(Ok(NEWER.as_bytes().to_vec()));
        let mut dialog = UpdateCheckDialogState::default();
        dialog.use_source(source);
        let ctx = themed();
        for _ in 0..3 {
            let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
                render_update_check_dialog(&mut dialog, ui.ctx());
            });
        }
        assert_eq!(asked.load(Ordering::SeqCst), 0);
        assert_eq!(dialog.phase, UpdateCheckPhase::Checking);
        open_update_check(&mut dialog);
        assert!(dialog.open);
        let _ = settled(&mut dialog, &ctx);
        assert_eq!(asked.load(Ordering::SeqCst), 1, "one check, one fetch");
    }

    #[test]
    fn a_check_settles_in_each_state_its_answer_calls_for() {
        let up_to_date = NEWER.replace("99.0.0", INSTALLED_VERSION);
        let foreign = NEWER.replace(
            "https://rspice.app/download",
            "https://example.com/rspice.exe",
        );
        let cases: [(Answer, UpdateCheckPhase); 6] = [
            (
                Ok(NEWER.as_bytes().to_vec()),
                UpdateCheckPhase::Newer(Release {
                    version: "99.0.0".to_owned(),
                    date: "2026-11-01".to_owned(),
                    download_url: "https://rspice.app/download".to_owned(),
                    notes_url: "https://rspice.app/changelog#v99.0.0".to_owned(),
                }),
            ),
            (Ok(up_to_date.into_bytes()), UpdateCheckPhase::UpToDate),
            (
                Ok(b"{not json".to_vec()),
                UpdateCheckPhase::Failed(CheckFailure::Unreadable),
            ),
            (
                Ok(foreign.into_bytes()),
                UpdateCheckPhase::Failed(CheckFailure::ForeignPage),
            ),
            (
                Err(CheckFailure::TimedOut),
                UpdateCheckPhase::Failed(CheckFailure::TimedOut),
            ),
            (
                Err(CheckFailure::Status(404)),
                UpdateCheckPhase::Failed(CheckFailure::Status(404)),
            ),
        ];
        let ctx = themed();
        for (answer, expected) in cases {
            let mut dialog = UpdateCheckDialogState::default();
            dialog.use_source(ReleaseSource::new(move || answer.clone()));
            open_update_check(&mut dialog);
            assert_eq!(dialog.phase, UpdateCheckPhase::Checking);
            assert_eq!(settled(&mut dialog, &ctx), expected);
            assert!(dialog.open, "the answer is shown, not dismissed");
        }
    }

    #[test]
    fn a_test_build_checks_no_network_until_given_a_source() {
        let mut dialog = UpdateCheckDialogState::default();
        open_update_check(&mut dialog);
        assert_eq!(
            settled(&mut dialog, &themed()),
            UpdateCheckPhase::Failed(CheckFailure::Unreachable)
        );
    }

    #[test]
    fn try_again_asks_again_and_closing_drops_the_check_in_flight() {
        let (source, asked) = counting(Err(CheckFailure::Unreachable));
        let mut dialog = UpdateCheckDialogState::default();
        dialog.use_source(source);
        let ctx = themed();
        open_update_check(&mut dialog);
        let _ = settled(&mut dialog, &ctx);
        dialog.start();
        let _ = settled(&mut dialog, &ctx);
        assert_eq!(asked.load(Ordering::SeqCst), 2);
        dialog.start();
        dialog.close();
        assert!(!dialog.open);
        assert!(dialog.answer.is_none(), "nothing waits on a closed dialog");
    }
}
