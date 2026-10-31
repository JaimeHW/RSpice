//! The application's cloud feature.
//!
//! The RSpice Cloud account (sign-in, the stored session and the licence),
//! web publication, and the live session with its presence, together with the
//! feature's commands, its state slot and its registration. It works on the
//! shared model in `rspice-app-state`, uses the platform services of
//! `rspice-app-services` and the run tier of `rspice-app-runs`, and registers
//! with the workbench framework of `rspice-app-workbench`. It is the only
//! application crate that names the network stack (the HTTP and WebSocket
//! clients and the typed cloud client), and it reaches no other feature.
//!
//! `rspice-app` re-exports each module at the path it has always had, so the
//! modules keep the layout they had inside it.

#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
#![allow(clippy::too_many_arguments)]
#![deny(clippy::print_stdout, clippy::print_stderr)]
#![cfg_attr(
    test,
    allow(
        clippy::assertions_on_constants,
        clippy::bool_assert_comparison,
        clippy::cloned_ref_to_slice_refs,
        clippy::default_constructed_unit_structs,
        clippy::expect_fun_call,
        clippy::field_reassign_with_default,
        clippy::len_zero,
        clippy::manual_range_contains,
        clippy::manual_repeat_n,
        clippy::needless_range_loop,
        clippy::unnecessary_cast,
        clippy::unnecessary_get_then_check,
        clippy::unnecessary_unwrap,
        clippy::useless_vec
    )
)]

// The model's, the services' and the framework's modules, at the paths the
// cloud feature has always named them by.
use rspice_app_services::io;
use rspice_app_state::{diagnostics, state, time_compat};
use rspice_app_workbench::ui;

pub mod services {
    //! The cloud feature's services, at their application paths.

    pub mod cloud_account;
    #[cfg(windows)]
    pub mod dpapi;
    pub mod license;
    pub(crate) use rspice_app_services::services::live_protocol;
}

pub mod workbench {
    //! The cloud feature's modules, at their application paths.

    pub(crate) use rspice_app_workbench::workbench::design_system;
    pub(crate) use rspice_app_workbench::workbench::routing::navigation::RouteTransitionSource;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_route::SurfaceRoute;
    pub(crate) use rspice_app_workbench::workbench::{framework, state};

    pub mod account_organization;
    pub mod app {
        pub mod dialogs {
            pub(crate) use rspice_app_workbench::workbench::app::dialogs::{
                fact_grid, review_primitives,
            };

            pub mod license;
            pub mod live_session;
            pub mod publish_web;
            pub mod unpublish_web;
        }
    }
    pub mod app_state {
        pub(crate) use rspice_app_runs::workbench::app_state::run_preflight;
        pub(crate) use rspice_app_services::workbench::slots::platform::PlatformState;
        pub(crate) use rspice_app_state::workbench::app_state::model::{self, Model};
    }
    pub mod chrome {
        pub mod presence;
    }
    pub mod cloud;
    pub(crate) mod lifecycle {
        pub(crate) use rspice_app_services::workbench::lifecycle::project_lifecycle;
    }
    pub mod live_session;
    pub mod slots {
        pub(crate) use cloud::CloudState;

        pub mod cloud;
    }
    pub mod user_identity;
}

#[cfg(test)]
mod build_profile_tests {
    /// Development license material and the local-stack endpoints compile in
    /// exactly when Cargo builds the debug profile, whatever else the profile
    /// turns on.
    #[test]
    fn development_material_follows_the_build_profile() {
        assert_eq!(
            cfg!(rspice_development_build),
            env!("RSPICE_BUILD_PROFILE") == "debug"
        );
    }
}
