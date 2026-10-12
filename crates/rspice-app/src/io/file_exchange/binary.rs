//! Packed checkpoint files use the same picker ownership and bounded mailbox.
use super::*;

#[derive(Debug, Clone)]
pub(crate) struct OpenedBinaryFile {
    pub(crate) name: String,
    pub(crate) bytes: Vec<u8>,
}

pub(crate) fn open_binary_file(
    ctx: &Context,
    id: Id,
    kind: FileKind,
    max_bytes: usize,
) -> Result<(), String> {
    let mailbox = claim::<OpenedBinaryFile>(ctx, id, kind)?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        let outcome = match chosen_source(kind) {
            Some(path) => read_bounded_bytes(&path, kind, max_bytes).map(|bytes| {
                Some(OpenedBinaryFile {
                    name: chosen_name(&path, kind),
                    bytes,
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
                Some(file) => read_browser_file(&file, kind, max_bytes).await.map(Some),
                None => Ok(None),
            };
            deliver(&repaint, &mailbox, outcome);
        });
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
async fn read_browser_file(
    file: &rfd::FileHandle,
    kind: FileKind,
    max_bytes: usize,
) -> Result<OpenedBinaryFile, String> {
    let size = file.inner().size();
    if !size.is_finite() || size < 0.0 || size > max_bytes as f64 {
        return Err(format!(
            "{} exceeds the {max_bytes} byte limit.",
            kind.subject
        ));
    }
    // File is immutable. Inspect its size before allocating, and handle a read
    // rejection instead of leaving a picker mailbox pending indefinitely.
    let buffer = wasm_bindgen_futures::JsFuture::from(file.inner().array_buffer())
        .await
        .map_err(|error| format!("{} could not be read: {error:?}", kind.subject))?;
    let bytes = js_sys::Uint8Array::new(&buffer);
    if bytes.length() as usize > max_bytes {
        return Err(format!(
            "{} exceeds the {max_bytes} byte limit.",
            kind.subject
        ));
    }
    Ok(OpenedBinaryFile {
        name: file.file_name(),
        bytes: bytes.to_vec(),
    })
}

pub(crate) fn take_opened_binary(ctx: &Context, id: Id) -> Option<Outcome<OpenedBinaryFile>> {
    take(ctx, id)
}
