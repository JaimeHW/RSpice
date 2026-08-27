//! Choosing a file to read, choosing a destination to write, and decoding a
//! bounded UTF-8 file — on the desktop and in the browser alike.
//!
//! A dialog and a surface had each grown their own copy of this, and both
//! copies were the same pair of `cfg(target_arch)` branches: a blocking
//! `rfd::FileDialog` on the desktop, a spawned `rfd::AsyncFileDialog` in the
//! browser, and the ceiling on the read spelled out on either side. A normal
//! desktop build does not compile the wasm half, so each copy carried a branch
//! nobody was checking.
//!
//! # One shape on both platforms
//!
//! A caller starts an exchange with [`open_file`] or [`save_file`] and collects
//! the result with [`take_opened`] or [`take_saved`] on a later frame. The
//! second step is what makes a single API possible: a browser cannot hand back
//! a file synchronously, so the click can only *start* the read. The desktop
//! picker blocks and could return its answer directly, but it posts to the same
//! mailbox instead — which leaves the call sites with no `cfg` at all, and so
//! with no branch that only one platform exercises.
//!
//! The mailbox is a slot of frame-context temp data under the caller's
//! `egui::Id`. It belongs to this session's picker, never to the project, and
//! taking a result removes it. A caller whose surface has closed since the pick
//! must still take the result in order to drop it: left in place, it would be
//! delivered to whatever next opens under the same id.
//!
//! # Why the destination is compared, not overwritten
//!
//! A desktop save publishes through `durable_file`'s compare-and-exchange
//! rather than a plain write. What the picker returns is an observation, and
//! between observing the destination and writing to it something else may have
//! put a file there. Writing on the strength of the stale observation is how a
//! file the reader never agreed to replace gets replaced.
//!
//! # What the browser cannot say
//!
//! `rfd`'s wasm save returns a handle without prompting, and reports the write
//! as successful whether the reader accepts or dismisses the card it then
//! shows. A browser save therefore never reports a cancellation. It does still
//! put a card in front of the reader, which is why saves route through `rfd`
//! rather than through the browser download helper: that one synthesizes the
//! anchor click itself, so the file goes wherever the user agent puts downloads
//! without the reader being asked at all.
//!
//! # Why a test build never opens a dialog
//!
//! The desktop picker blocks the calling thread on a real window. A unit test
//! that reaches a control which starts one therefore stops dead until a person
//! sitting at the machine closes it — and the studio's press sweep does reach
//! that control, so an unattended run hangs and an attended one records
//! whichever answer the person gave. Under `cfg(test)` the two `rfd` calls are
//! replaced by [`chosen_source`] and [`chosen_destination`], which answer from
//! a thread-local script and default to a cancellation. Only the dialog is
//! replaced: the ceiling, the decode and the compare-and-exchange publication
//! all run exactly as they do on the desktop, so a scripted path that cannot be
//! read produces the refusal a mis-picked file produces.

use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

use egui::{Context, Id};

/// What a picker offers, and how its refusals name the file.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FileKind {
    /// The filter's label in the picker, e.g. `"Design variable spec sheet"`.
    pub(crate) label: &'static str,
    /// The extensions the filter accepts, without a leading dot. The first
    /// also names the encoding a decode failure asks for.
    pub(crate) extensions: &'static [&'static str],
    /// How a message names the file, e.g. `"the spec sheet"`. Every message
    /// this module produces leads with it, so a caller whose surface writes
    /// sentence case can capitalize it here and get sentence case back.
    pub(crate) subject: &'static str,
    /// What to call the file when the chosen path's own name is not UTF-8.
    pub(crate) fallback_name: &'static str,
}

/// A file the reader chose to open, already decoded.
#[derive(Debug, Clone)]
pub(crate) struct OpenedFile {
    pub(crate) name: String,
    pub(crate) text: String,
}

/// A destination the reader chose, already written.
#[derive(Debug, Clone)]
pub(crate) struct SavedFile {
    pub(crate) name: String,
}

/// What one exchange produced: the file, or `None` where the reader cancelled.
///
/// Cancellation is a choice rather than a failure, so it is a variant of the
/// success case. A caller that treats it as an error has to recognise its own
/// cancellation message to keep from reporting one.
pub(crate) type Outcome<T> = Result<Option<T>, String>;

type Mailbox<T> = Arc<Mutex<Option<Outcome<T>>>>;

/// Ask the reader for a file to read, bounded at `max_bytes` and decoded as
/// UTF-8. Collect the answer with [`take_opened`] under the same `id`.
///
/// The `Err` here means the exchange never started — an `id` already has a
/// picker open. Whatever the picker itself produces arrives through the
/// mailbox.
pub(crate) fn open_file(
    ctx: &Context,
    id: Id,
    kind: FileKind,
    max_bytes: usize,
) -> Result<(), String> {
    let mailbox = claim::<OpenedFile>(ctx, id, kind)?;

    #[cfg(not(target_arch = "wasm32"))]
    {
        let outcome = match chosen_source(kind) {
            Some(path) => read_bounded_utf8(&path, kind, max_bytes).map(|text| {
                Some(OpenedFile {
                    name: chosen_name(&path, kind),
                    text,
                })
            }),
            None => Ok(None),
        };
        deliver(ctx, &mailbox, outcome);
    }

    #[cfg(target_arch = "wasm32")]
    {
        let repaint = ctx.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let outcome = match rfd::AsyncFileDialog::new()
                .add_filter(kind.label, kind.extensions)
                .pick_file()
                .await
            {
                Some(file) => {
                    let name = file.file_name();
                    decode_bounded_utf8(file.read().await, kind, max_bytes)
                        .map(|text| Some(OpenedFile { name, text }))
                }
                None => Ok(None),
            };
            deliver(&repaint, &mailbox, outcome);
        });
    }

    Ok(())
}

/// Ask the reader for a destination and write `bytes` to it. Collect the
/// answer with [`take_saved`] under the same `id`.
///
/// `file_name` is what the picker opens with, not where the file lands: the
/// reader chooses that.
pub(crate) fn save_file(
    ctx: &Context,
    id: Id,
    kind: FileKind,
    file_name: String,
    bytes: Vec<u8>,
) -> Result<(), String> {
    let mailbox = claim::<SavedFile>(ctx, id, kind)?;

    #[cfg(not(target_arch = "wasm32"))]
    {
        let outcome = match chosen_destination(kind, &file_name) {
            Some(path) => publish(&path, &bytes, kind).map(Some),
            None => Ok(None),
        };
        deliver(ctx, &mailbox, outcome);
    }

    #[cfg(target_arch = "wasm32")]
    {
        let repaint = ctx.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let outcome = match rfd::AsyncFileDialog::new()
                .add_filter(kind.label, kind.extensions)
                .set_file_name(&file_name)
                .save_file()
                .await
            {
                Some(handle) => match handle.write(&bytes).await {
                    Ok(()) => Ok(Some(SavedFile {
                        name: handle.file_name(),
                    })),
                    Err(error) => Err(format!("{} could not be published: {error}", kind.subject)),
                },
                None => Ok(None),
            };
            deliver(&repaint, &mailbox, outcome);
        });
    }

    Ok(())
}

/// Take the result of an [`open_file`] started under `id`, if one has arrived.
pub(crate) fn take_opened(ctx: &Context, id: Id) -> Option<Outcome<OpenedFile>> {
    take(ctx, id)
}

/// Take the result of a [`save_file`] started under `id`, if one has arrived.
pub(crate) fn take_saved(ctx: &Context, id: Id) -> Option<Outcome<SavedFile>> {
    take(ctx, id)
}

/// Register a mailbox for `id`, refusing when one is already in flight.
///
/// The refusal matters on the desktop as much as in the browser: a second
/// picker under the same id would post over the first one's answer, so the
/// reader's earlier choice would vanish with nothing said.
fn claim<T>(ctx: &Context, id: Id, kind: FileKind) -> Result<Mailbox<T>, String>
where
    T: Clone + Send + Sync + 'static,
{
    if ctx.data(|data| data.get_temp::<Mailbox<T>>(id)).is_some() {
        return Err(format!("{} picker is already open.", kind.subject));
    }
    let mailbox = Mailbox::<T>::default();
    ctx.data_mut(|data| data.insert_temp(id, mailbox.clone()));
    Ok(mailbox)
}

/// Post an outcome and wake the frame loop, so a result that arrives while the
/// application is idle is still collected.
fn deliver<T>(ctx: &Context, mailbox: &Mailbox<T>, outcome: Outcome<T>)
where
    T: Clone + Send + Sync + 'static,
{
    if let Ok(mut slot) = mailbox.lock() {
        *slot = Some(outcome);
    }
    ctx.request_repaint();
}

fn take<T>(ctx: &Context, id: Id) -> Option<Outcome<T>>
where
    T: Clone + Send + Sync + 'static,
{
    let mailbox = ctx.data(|data| data.get_temp::<Mailbox<T>>(id))?;
    let outcome = mailbox.lock().ok().and_then(|mut slot| slot.take())?;
    ctx.data_mut(|data| data.remove::<Mailbox<T>>(id));
    Some(outcome)
}

// ------------------------------------------------------- the desktop pickers
//
// One pair of one-line functions, so the dialog is the only thing a test build
// stands in for. Everything the picker's answer is put through afterwards is
// the production code path on both sides of the `cfg`.

/// Ask the reader which file to read.
#[cfg(all(not(target_arch = "wasm32"), not(test)))]
fn chosen_source(kind: FileKind) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .add_filter(kind.label, kind.extensions)
        .pick_file()
}

/// Ask the reader where to write, opening on `file_name`.
#[cfg(all(not(target_arch = "wasm32"), not(test)))]
fn chosen_destination(kind: FileKind, file_name: &str) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .add_filter(kind.label, kind.extensions)
        .set_file_name(file_name)
        .save_file()
}

/// What a test build answers a desktop picker with, in place of the dialog.
///
/// A refusal is not a variant because the desktop picker has none: `rfd`
/// reports a cancellation and nothing else, and every error this module can
/// produce comes from what it does with the path afterwards. So a test that
/// wants a refusal scripts a path that cannot be read, which is the failure a
/// reader actually reaches.
#[cfg(all(not(target_arch = "wasm32"), test))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ScriptedChoice {
    /// The reader chose this path.
    Chose(std::path::PathBuf),
    /// The reader cancelled.
    Cancelled,
}

#[cfg(all(not(target_arch = "wasm32"), test))]
thread_local! {
    /// Answers waiting for the next pickers this thread opens, in order.
    ///
    /// Thread-local rather than shared: libtest runs tests in parallel on
    /// threads of their own, so a shared queue would let one test's answer land
    /// in another test's picker.
    static SCRIPTED_CHOICES: std::cell::RefCell<std::collections::VecDeque<ScriptedChoice>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };

    /// The filter label of every picker this thread has opened and not yet
    /// accounted for. A press that reached a dialog instead of the seam leaves
    /// nothing here, which is how a test tells the two apart.
    static PICKERS_OPENED: std::cell::RefCell<Vec<&'static str>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// Answer the next picker this thread opens with `choice`.
#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) fn script_next_choice(choice: ScriptedChoice) {
    SCRIPTED_CHOICES.with_borrow_mut(|queue| queue.push_back(choice));
}

/// How many scripted answers are still waiting.
///
/// A test that scripted a pick and finds one still queued pressed something
/// that never opened a picker at all.
#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) fn scripted_choices_remaining() -> usize {
    SCRIPTED_CHOICES.with_borrow(std::collections::VecDeque::len)
}

/// The pickers this thread has opened since this was last called, by the label
/// their filter announces, and clear the record.
#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) fn take_pickers_opened() -> Vec<&'static str> {
    PICKERS_OPENED.with_borrow_mut(std::mem::take)
}

/// The answer this thread scripted, or a cancellation.
///
/// Cancellation is the default because it is the one answer that leaves
/// nothing behind: an unscripted picker in a test build reads exactly as a
/// reader who opened the dialog and closed it again.
#[cfg(all(not(target_arch = "wasm32"), test))]
fn next_scripted_choice(kind: FileKind) -> Option<std::path::PathBuf> {
    PICKERS_OPENED.with_borrow_mut(|opened| opened.push(kind.label));
    match SCRIPTED_CHOICES.with_borrow_mut(std::collections::VecDeque::pop_front) {
        Some(ScriptedChoice::Chose(path)) => Some(path),
        Some(ScriptedChoice::Cancelled) | None => None,
    }
}

#[cfg(all(not(target_arch = "wasm32"), test))]
fn chosen_source(kind: FileKind) -> Option<std::path::PathBuf> {
    next_scripted_choice(kind)
}

#[cfg(all(not(target_arch = "wasm32"), test))]
fn chosen_destination(kind: FileKind, _file_name: &str) -> Option<std::path::PathBuf> {
    next_scripted_choice(kind)
}

/// Refuse an oversized payload before any of it is decoded, then decode.
///
/// The ceiling is checked on the bytes rather than on the decoded text so that
/// a mis-picked binary dump is refused by size, not by whichever of its bytes
/// happens not to be UTF-8.
fn decode_bounded_utf8(bytes: Vec<u8>, kind: FileKind, max_bytes: usize) -> Result<String, String> {
    if bytes.len() > max_bytes {
        return Err(format!(
            "{} exceeds the {max_bytes} byte limit.",
            kind.subject
        ));
    }
    String::from_utf8(bytes).map_err(|_| {
        format!(
            "{} must be UTF-8 {}.",
            kind.subject,
            kind.extensions
                .first()
                .map_or_else(|| "text".to_owned(), |extension| extension.to_uppercase())
        )
    })
}

/// Read at most one byte past the ceiling, so an oversized file is recognised
/// without being loaded.
#[cfg(not(target_arch = "wasm32"))]
fn read_bounded_utf8(path: &Path, kind: FileKind, max_bytes: usize) -> Result<String, String> {
    use std::io::Read as _;

    let file = std::fs::File::open(path)
        .map_err(|error| format!("{} could not be opened: {error}", kind.subject))?;
    let mut bytes = Vec::new();
    file.take((max_bytes + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("{} could not be read: {error}", kind.subject))?;
    decode_bounded_utf8(bytes, kind, max_bytes)
}

#[cfg(not(target_arch = "wasm32"))]
fn chosen_name(path: &Path, kind: FileKind) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(kind.fallback_name)
        .to_owned()
}

/// Publish to the destination the picker returned, only while that destination
/// still holds what the picker observed.
#[cfg(not(target_arch = "wasm32"))]
fn publish(path: &Path, bytes: &[u8], kind: FileKind) -> Result<SavedFile, String> {
    let expected = super::durable_file::observe_expected_content(path).map_err(|error| {
        format!(
            "{} destination could not be authorized: {error}",
            kind.subject
        )
    })?;
    super::durable_file::compare_exchange_bytes(path, expected, bytes)
        .map_err(|error| format!("{} could not be published: {error}", kind.subject))?;
    Ok(SavedFile {
        name: chosen_name(path, kind),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHEET: FileKind = FileKind {
        label: "Design variable spec sheet",
        extensions: &["csv"],
        subject: "the spec sheet",
        fallback_name: "spec-sheet.csv",
    };

    #[test]
    fn the_ceiling_is_measured_on_bytes_not_on_decoded_text() {
        // Four bytes, two characters: a text-length ceiling would accept this.
        let source = "\u{00e9}\u{00e9}".as_bytes().to_vec();
        assert_eq!(source.len(), 4);
        let error = decode_bounded_utf8(source, SHEET, 3).expect_err("over the ceiling");
        assert!(error.contains("exceeds the 3 byte limit"), "{error}");
        assert!(error.starts_with("the spec sheet"), "{error}");
    }

    #[test]
    fn a_decode_failure_names_the_encoding_the_filter_asks_for() {
        let error = decode_bounded_utf8(vec![0xff, 0xfe], SHEET, 64).expect_err("not UTF-8");
        assert_eq!(error, "the spec sheet must be UTF-8 CSV.");
    }

    #[test]
    fn a_payload_at_the_ceiling_is_accepted() {
        assert_eq!(
            decode_bounded_utf8(b"name\n".to_vec(), SHEET, 5).expect("exactly at the ceiling"),
            "name\n"
        );
    }

    /// A second picker under one id would post over the first one's answer, so
    /// the claim is refused rather than the earlier choice being lost.
    #[test]
    fn one_id_holds_one_exchange_at_a_time() {
        let ctx = Context::default();
        let id = Id::new("io.file_exchange.tests.claim");

        let first = claim::<OpenedFile>(&ctx, id, SHEET).expect("the first claim succeeds");
        let error = claim::<OpenedFile>(&ctx, id, SHEET).expect_err("the second is refused");
        assert_eq!(error, "the spec sheet picker is already open.");

        // Nothing has been posted yet, so there is nothing to take.
        assert!(take_opened(&ctx, id).is_none());

        deliver(
            &ctx,
            &first,
            Ok(Some(OpenedFile {
                name: "loads.csv".to_owned(),
                text: "name\n".to_owned(),
            })),
        );
        let opened = take_opened(&ctx, id)
            .expect("the posted outcome arrives")
            .expect("it is not a refusal")
            .expect("it is not a cancellation");
        assert_eq!(opened.name, "loads.csv");

        // Taking the outcome released the id, and it does not arrive twice.
        assert!(take_opened(&ctx, id).is_none());
        claim::<OpenedFile>(&ctx, id, SHEET).expect("the id is free again");
    }

    /// The two directions are separate exchanges even under one id: a save's
    /// mailbox is typed differently from an open's, so neither can collect the
    /// other's answer.
    #[test]
    fn a_save_and_an_open_do_not_collect_each_others_answers() {
        let ctx = Context::default();
        let id = Id::new("io.file_exchange.tests.directions");

        let saving = claim::<SavedFile>(&ctx, id, SHEET).expect("the save claims the id");
        deliver(&ctx, &saving, Ok(None));

        assert!(take_opened(&ctx, id).is_none());
        assert!(
            take_saved(&ctx, id)
                .expect("the save outcome arrives")
                .expect("it is not a refusal")
                .is_none(),
            "a cancellation is `None`, not an error"
        );
    }

    /// An unscripted picker in a test build answers at once, and answers the
    /// way a reader who closed the dialog does.
    ///
    /// This is the property the whole seam exists for: the call returns rather
    /// than blocking on a window nobody is there to close, and it delivers a
    /// cancellation, which is the answer that leaves no trace for a caller to
    /// act on.
    #[test]
    fn an_unscripted_picker_answers_with_a_cancellation_and_does_not_block() {
        let ctx = Context::default();
        let id = Id::new("io.file_exchange.tests.unscripted");

        open_file(&ctx, id, SHEET, 1024).expect("the exchange starts");
        assert!(
            take_opened(&ctx, id)
                .expect("the answer is already waiting")
                .expect("a cancellation is not a refusal")
                .is_none()
        );

        save_file(&ctx, id, SHEET, "loads.csv".to_owned(), b"name\n".to_vec())
            .expect("the exchange starts");
        assert!(
            take_saved(&ctx, id)
                .expect("the answer is already waiting")
                .expect("a cancellation is not a refusal")
                .is_none()
        );

        assert_eq!(
            take_pickers_opened(),
            [SHEET.label, SHEET.label],
            "both directions went through the seam, and neither wrote anything"
        );
    }

    /// A scripted path is read through the production path — the ceiling, the
    /// decode and the name the picker's answer is called by.
    #[test]
    fn a_scripted_path_is_read_the_way_a_picked_one_is() {
        let directory = scratch_directory("read");
        let path = directory.join("loads.csv");
        std::fs::write(&path, "name,value\nvdd,1.8\n").expect("the scratch sheet is written");

        let ctx = Context::default();
        let id = Id::new("io.file_exchange.tests.scripted");
        script_next_choice(ScriptedChoice::Chose(path));
        open_file(&ctx, id, SHEET, 1024).expect("the exchange starts");
        assert_eq!(
            scripted_choices_remaining(),
            0,
            "the picker took the answer"
        );

        let opened = take_opened(&ctx, id)
            .expect("the answer is waiting")
            .expect("the sheet reads")
            .expect("it is not a cancellation");
        assert_eq!(opened.name, "loads.csv");
        assert_eq!(opened.text, "name,value\nvdd,1.8\n");

        std::fs::remove_dir_all(&directory).ok();
    }

    /// A scripted path the platform cannot hand back produces the refusal a
    /// mis-picked file produces, which is how a test reaches the error arm
    /// without a dialog that has no error to give.
    #[test]
    fn a_scripted_path_that_cannot_be_read_refuses_the_way_a_picked_one_does() {
        let ctx = Context::default();
        let id = Id::new("io.file_exchange.tests.unreadable");
        script_next_choice(ScriptedChoice::Chose(
            std::env::temp_dir().join("rspice-file-exchange-no-such-sheet.csv"),
        ));
        open_file(&ctx, id, SHEET, 1024).expect("the exchange starts");

        let error = take_opened(&ctx, id)
            .expect("the answer is waiting")
            .expect_err("an unreadable path is a refusal");
        assert!(
            error.starts_with("the spec sheet could not be opened"),
            "{error}"
        );
    }

    /// The queue is consumed in order, and an explicit cancellation is one of
    /// its answers rather than only the default.
    #[test]
    fn scripted_answers_are_taken_in_the_order_they_were_written() {
        let directory = scratch_directory("ordered");
        let path = directory.join("second.csv");
        std::fs::write(&path, "name\n").expect("the scratch sheet is written");

        let ctx = Context::default();
        script_next_choice(ScriptedChoice::Cancelled);
        script_next_choice(ScriptedChoice::Chose(path));
        assert_eq!(scripted_choices_remaining(), 2);

        let first = Id::new("io.file_exchange.tests.ordered.first");
        open_file(&ctx, first, SHEET, 1024).expect("the exchange starts");
        assert!(
            take_opened(&ctx, first)
                .expect("the answer is waiting")
                .expect("a cancellation is not a refusal")
                .is_none()
        );

        let second = Id::new("io.file_exchange.tests.ordered.second");
        open_file(&ctx, second, SHEET, 1024).expect("the exchange starts");
        assert_eq!(
            take_opened(&ctx, second)
                .expect("the answer is waiting")
                .expect("the sheet reads")
                .expect("it is not a cancellation")
                .name,
            "second.csv"
        );
        assert_eq!(scripted_choices_remaining(), 0);

        std::fs::remove_dir_all(&directory).ok();
    }

    /// A scratch directory of this test's own, so parallel tests never share a
    /// path.
    fn scratch_directory(purpose: &str) -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!(
            "rspice-file-exchange-{purpose}-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("the scratch directory is made");
        directory
    }
}
