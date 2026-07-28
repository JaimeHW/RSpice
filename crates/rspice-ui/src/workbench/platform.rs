//! Platform differences the shell has to spell out.

/// Run `work` on a background thread natively; inline on wasm32 for short UI
/// tasks that are not routed through the browser simulation worker.
pub(crate) fn spawn_or_inline(work: impl FnOnce() + Send + 'static) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(work);
    }
    #[cfg(target_arch = "wasm32")]
    {
        work();
    }
}
