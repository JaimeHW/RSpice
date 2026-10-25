//! The application's shared persisted model and the pure logic over it.
//!
//! Everything a project or session persists that two or more workspaces read
//! lives here: the schematic, the simulation and its plans, the workspace and
//! library managers, design checks and history, diagnostics, preferences and
//! the notice queue. Nothing here draws: the crate uses egui value types only,
//! never `Ui`, `Context` or `Painter`.
//!
//! `rspice-app` re-exports each module at the path it has always had, so the
//! modules keep the layout they had inside it.

#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
#![allow(clippy::too_many_arguments)]
// A diagnostic listing's `len` counts rows to index; an empty listing is told
// apart by the reason it is empty, which it carries, not by a bare flag.
#![allow(clippy::len_without_is_empty)]
#![deny(clippy::print_stdout, clippy::print_stderr)]
#![cfg_attr(test, allow(clippy::field_reassign_with_default))]

pub mod diagnostics;
#[cfg(any(test, feature = "test-support"))]
pub mod fixture_root;
pub mod product;
pub mod quantity;
pub mod results;
pub mod source_revision;
pub mod state;
pub mod time_compat;

pub mod simulation {
    pub mod placed_sources;
    pub mod plan;
    pub mod stimulus_realize;
    pub mod table_route;
}

pub mod workbench {
    pub mod app {
        pub mod schematic {
            pub mod edit_authority;
        }
    }
    pub mod app_state {
        pub mod cell_usage;
        pub mod design_checks;
        pub mod design_history;
        pub mod diagnostic_store;
        pub mod model;
        pub mod rule_deck;
        pub mod run_identity;
        pub mod schematic_preferences;
        pub mod sim_setup;
        pub mod symbol_contract;
        pub mod waivers;
    }
    pub mod examples;
    pub mod preferences;
    pub mod workspace;
}
