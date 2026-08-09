//! Synchronous bridge from the primary solver module to installed secondary modules.

use std::cell::RefCell;

type BrowserDispatcher = dyn Fn(&str, &str, u32) -> Result<i32, String>;

thread_local! {
    static BROWSER_DISPATCHER: RefCell<Option<Box<BrowserDispatcher>>> = const { RefCell::new(None) };
}

/// Install the worker-owned synchronous secondary-module dispatcher.
///
/// Installation replaces any previous callback after a worker package reload.
/// The callback receives an authenticated model cache key, exact export name,
/// and primary-memory evaluation-frame offset.
pub fn install_browser_dispatcher(
    dispatcher: impl Fn(&str, &str, u32) -> Result<i32, String> + 'static,
) {
    BROWSER_DISPATCHER.with(|slot| {
        *slot.borrow_mut() = Some(Box::new(dispatcher));
    });
}

pub(crate) fn dispatch_model_entry(
    cache_key: &str,
    export_name: &str,
    frame_offset: u32,
) -> Result<i32, String> {
    BROWSER_DISPATCHER.with(|slot| {
        let slot = slot.borrow();
        let dispatcher = slot
            .as_ref()
            .ok_or_else(|| "browser WASM JIT dispatcher is not installed".to_owned())?;
        dispatcher(cache_key, export_name, frame_offset)
    })
}
