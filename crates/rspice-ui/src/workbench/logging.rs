//! Logging defaults shared by native entry points.
//!
//! Which filter a desktop launch starts from, and the environment variable that
//! overrides it. The logger those build is
//! [`crate::diagnostics::engine_log::StudioLogger`], which sends every record
//! to this filter exactly as before and additionally offers the engine's own
//! lines to the run that produced them.

/// Default native logging filter.
///
/// Routine startup is quiet; warnings and errors remain visible, while noisy
/// graphics-backend probe warnings stay out of normal desktop stderr.
#[cfg(not(target_arch = "wasm32"))]
pub fn native_default_filter() -> &'static str {
    "warn,rspice_ui=warn,rspice_core=warn,rspice_veriloga=warn,wgpu_core=error,wgpu_hal=error,naga=error"
}

/// Environment variable for native GUI log-filter overrides.
#[cfg(not(target_arch = "wasm32"))]
pub fn native_filter_env_var() -> &'static str {
    "RSPICE_LOG"
}

/// Native GUI logging environment.
#[cfg(not(target_arch = "wasm32"))]
pub fn native_log_env() -> env_logger::Env<'static> {
    env_logger::Env::new().filter_or(native_filter_env_var(), native_default_filter())
}

/// Install the studio's logger as the process logger for the desktop binary.
///
/// Replaces the bare `env_logger::Builder::init()` this entry point used to
/// call. The logger itself is
/// [`crate::diagnostics::engine_log::StudioLogger`]: what the application
/// reports about itself is that module's subject, and the run sink it feeds
/// has to be reachable from the layers that execute a run. What stays here is
/// what this file has always owned — which filter a desktop launch starts from.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_studio_logger() {
    crate::diagnostics::engine_log::install_studio_logger(
        env_logger::Builder::from_env(native_log_env()).build(),
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use log::{Level, Record};
    use std::{
        env,
        ffi::OsString,
        sync::{Mutex, MutexGuard},
    };

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn native_default_filter_suppresses_gpu_backend_probe_warnings() {
        let filter = native_default_filter();

        assert!(filter.contains("rspice_ui=warn"));
        assert!(!filter.contains("rspice_ui=info"));
        assert!(filter.contains("warn"));
        assert!(filter.contains("wgpu_core=error"));
        assert!(filter.contains("wgpu_hal=error"));
        assert!(filter.contains("naga=error"));
        assert!(!filter.contains("wgpu_core=warn"));
        assert!(!filter.contains("wgpu_hal=warn"));
    }

    #[test]
    fn native_log_env_ignores_generic_rust_log_and_honors_rspice_log() {
        let _guard = ScopedEnv::lock();
        let _rust_log = ScopedEnv::capture("RUST_LOG");
        let _rspice_log = ScopedEnv::capture(native_filter_env_var());
        set_env("RUST_LOG", Some("warn"));
        set_env(native_filter_env_var(), None);

        let backend_probe = Record::builder()
            .level(Level::Warn)
            .target("wgpu_hal::vulkan::instance")
            .args(format_args!("validation layer probe"))
            .build();

        let logger = env_logger::Builder::from_env(native_log_env()).build();
        assert!(!logger.matches(&backend_probe));

        let dependency_warning = Record::builder()
            .level(Level::Warn)
            .target("eframe::native")
            .args(format_args!("windowing warning"))
            .build();
        assert!(logger.matches(&dependency_warning));

        set_env(native_filter_env_var(), Some("warn"));

        let logger = env_logger::Builder::from_env(native_log_env()).build();
        assert!(logger.matches(&backend_probe));
    }

    struct ScopedEnv {
        key: &'static str,
        value: Option<OsString>,
    }

    impl ScopedEnv {
        fn lock() -> MutexGuard<'static, ()> {
            ENV_LOCK.lock().expect("logging env test lock poisoned")
        }

        fn capture(key: &'static str) -> Self {
            Self {
                key,
                value: env::var_os(key),
            }
        }
    }

    impl Drop for ScopedEnv {
        fn drop(&mut self) {
            set_env_os(self.key, self.value.as_ref());
        }
    }

    fn set_env(key: &'static str, value: Option<&str>) {
        set_env_os(key, value.map(OsString::from).as_ref());
    }

    fn set_env_os(key: &'static str, value: Option<&OsString>) {
        // SAFETY: these tests serialize mutations to the relevant process
        // environment and restore each variable before releasing the lock.
        unsafe {
            match value {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
        }
    }
}
