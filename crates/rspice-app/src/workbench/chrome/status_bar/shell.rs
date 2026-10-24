//! The status bar's platform segment: where the application is running.

#[cfg(target_arch = "wasm32")]
pub(in crate::workbench) fn platform_label() -> String {
    "Browser · WASM worker".to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::workbench) fn platform_label() -> String {
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    format!(
        "Desktop · local {threads} thread{}",
        if threads == 1 { "" } else { "s" }
    )
}
