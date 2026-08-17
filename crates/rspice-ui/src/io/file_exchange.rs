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
        let outcome = match rfd::FileDialog::new()
            .add_filter(kind.label, kind.extensions)
            .pick_file()
        {
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
        let outcome = match rfd::FileDialog::new()
            .add_filter(kind.label, kind.extensions)
            .set_file_name(&file_name)
            .save_file()
        {
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
}
