#[cfg(any(not(target_arch = "wasm32"), test))]
use super::MAX_SYMBOL_DEFINITION_IMPORT_BYTES;
use super::state::SymbolImportDialogState;

const SYMBOL_SOURCE_FILTER_NAME: &str = "RSpice, SVG, EDIF, or LTspice symbol";
const SYMBOL_SOURCE_EXTENSIONS: &[&str] = &["rspicesym", "json", "svg", "edif", "edf", "asy"];

#[cfg(any(not(target_arch = "wasm32"), test))]
fn validate_source_bytes(name: &str, bytes: Vec<u8>) -> Result<String, String> {
    if bytes.len() as u64 > MAX_SYMBOL_DEFINITION_IMPORT_BYTES {
        return Err(format!(
            "Selected {name} exceeds the supported {MAX_SYMBOL_DEFINITION_IMPORT_BYTES}-byte size limit"
        ));
    }
    String::from_utf8(bytes).map_err(|error| format!("Selected {name} is not valid UTF-8: {error}"))
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn pick_symbol_source(state: &mut SymbolImportDialogState) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter(SYMBOL_SOURCE_FILTER_NAME, SYMBOL_SOURCE_EXTENSIONS)
        .pick_file()
    else {
        return;
    };
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let result = std::fs::metadata(&path)
        .map_err(|error| format!("Could not inspect {name}: {error}"))
        .and_then(|metadata| {
            if metadata.len() > MAX_SYMBOL_DEFINITION_IMPORT_BYTES {
                Err(format!(
                    "Selected {name} exceeds the supported {MAX_SYMBOL_DEFINITION_IMPORT_BYTES}-byte size limit"
                ))
            } else {
                std::fs::read(&path)
                    .map_err(|error| format!("Could not read {name}: {error}"))
                    .and_then(|bytes| validate_source_bytes(&name, bytes))
            }
        });
    match result {
        Ok(text) => state.accept_source(name, text),
        Err(error) => state.source_error = Some(error),
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
enum BrowserSymbolImportResult {
    Cancelled,
    Failed(String),
    Loaded(crate::workbench::browser_file_import::PickedTextFile),
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserSymbolImportCompletion {
    token: crate::workbench::browser_file_import::TextImportToken,
    result: BrowserSymbolImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_SYMBOL_IMPORT_RESULT: std::cell::RefCell<Option<BrowserSymbolImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
pub(super) fn pick_symbol_source(state: &mut SymbolImportDialogState) {
    if state.picker_token.is_some() {
        state.source_error = Some("A symbol definition picker is already open".to_owned());
        return;
    }
    let token = match crate::workbench::browser_file_import::try_begin_text_import(
        crate::workbench::browser_file_import::BrowserTextImportKind::SymbolDefinition,
    ) {
        Ok(token) => token,
        Err(error) => {
            state.source_error = Some(error);
            return;
        }
    };
    state.picker_token = Some(token);
    state.source_error = None;
    crate::workbench::browser_file_import::pick_text_file(
        SYMBOL_SOURCE_FILTER_NAME,
        SYMBOL_SOURCE_EXTENSIONS,
        move |result| {
            if !crate::workbench::browser_file_import::text_import_is_current(token) {
                return;
            }
            let result = match result {
                Ok(Some(file)) => BrowserSymbolImportResult::Loaded(file),
                Ok(None) => BrowserSymbolImportResult::Cancelled,
                Err(error) => BrowserSymbolImportResult::Failed(error),
            };
            BROWSER_SYMBOL_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserSymbolImportCompletion { token, result });
            });
        },
    );
}

#[cfg(target_arch = "wasm32")]
pub(super) fn consume_symbol_source_result(state: &mut SymbolImportDialogState) {
    let Some(completion) = BROWSER_SYMBOL_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return;
    };
    if state.picker_token != Some(completion.token)
        || !crate::workbench::browser_file_import::finish_text_import(completion.token)
    {
        return;
    }
    state.picker_token = None;
    match completion.result {
        BrowserSymbolImportResult::Cancelled => {}
        BrowserSymbolImportResult::Failed(error) => state.source_error = Some(error),
        BrowserSymbolImportResult::Loaded(file) => {
            state.accept_source(file.name, file.contents);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_limit_and_utf8_are_enforced_before_parsing() {
        assert!(validate_source_bytes("valid.svg", b"<svg/>".to_vec()).is_ok());
        assert!(
            validate_source_bytes("bad.svg", vec![0xff])
                .unwrap_err()
                .contains("UTF-8")
        );
        assert!(
            validate_source_bytes(
                "huge.svg",
                vec![0; MAX_SYMBOL_DEFINITION_IMPORT_BYTES as usize + 1]
            )
            .unwrap_err()
            .contains("size limit")
        );
    }

    #[test]
    fn filter_does_not_claim_unsupported_library_formats() {
        assert_eq!(
            SYMBOL_SOURCE_EXTENSIONS,
            ["rspicesym", "json", "svg", "edif", "edf", "asy"]
        );
    }
}
