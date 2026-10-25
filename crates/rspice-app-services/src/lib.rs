//! The application's platform services.
//!
//! The project lifecycle and its document registry, the native and browser
//! project persistence, autosave and crash recovery, project, schematic and
//! waveform file I/O, the managed Automation runtime, recent files, and what
//! the browser build has to do differently. They work on the shared model in
//! `rspice-app-state` and the platform slot they own, and reach no feature.
//!
//! `rspice-app` re-exports each module at the path it has always had, so the
//! modules keep the layout they had inside it.

#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
#![allow(clippy::too_many_arguments)]
#![deny(clippy::print_stdout, clippy::print_stderr)]
#![cfg_attr(
    test,
    allow(clippy::cloned_ref_to_slice_refs, clippy::field_reassign_with_default)
)]

// The model's modules, at the paths the services have always named them by.
#[cfg(test)]
use rspice_app_state::results;
use rspice_app_state::{diagnostics, product, state, time_compat};

#[cfg(not(target_arch = "wasm32"))]
pub mod automation_runtime;
#[cfg(target_arch = "wasm32")]
#[path = "automation_runtime_browser.rs"]
pub mod automation_runtime;
pub mod build_identity;
pub mod io;

pub mod services {
    pub mod live_protocol;
    pub mod model_hub;
    pub mod recent_files;
    pub mod session;
}

pub mod workbench {
    pub mod app_state {
        pub mod session {
            pub mod autosave;
        }
    }
    pub mod browser {
        #[cfg(target_arch = "wasm32")]
        pub mod accessibility;
        pub mod background_alerts;
        #[cfg(target_arch = "wasm32")]
        pub mod download;
        #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
        pub mod file_import;
        #[cfg(all(target_arch = "wasm32", feature = "browser-qualification"))]
        pub mod qualification;
    }
    pub mod lifecycle {
        pub mod project_checkpoint;
        pub mod project_lifecycle;
        pub mod recovery;
        #[cfg(not(target_arch = "wasm32"))]
        pub mod recovery_checkpoint;
    }
    pub mod platform;
    pub mod slots {
        pub mod platform;
    }
    pub mod workflows {
        pub mod export_workflow;
    }
}
