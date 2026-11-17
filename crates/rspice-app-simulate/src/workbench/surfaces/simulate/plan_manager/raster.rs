//! The routes and viewports the plan manager's contract tests gate, and its
//! sources, held once so every gate reads the same set.

#![cfg(not(target_arch = "wasm32"))]

use egui::Vec2;

use crate::workbench::state::SimulationPlanManagerMode;

/// James's real viewport. Display scaling makes the usable area smaller than
/// a naive test window, and the product rule is that a dialog fits without
/// scrolling — so this, not a comfortable size, is what fit means here.
pub const REAL_VIEWPORT: Vec2 = egui::Vec2::new(1024.0, 640.0);

/// The three viewports every route is gated at.
///
/// 1024x640 is the real desktop one — display scaling makes the usable area
/// smaller than a comfortable test window. 820 is where `WideWorkflow` stops
/// being a fixed-width panel and becomes the whole viewport, and 560x900 is the
/// portrait shape a global command can open this dialog into. The names are the
/// ones the contract tests' two browse gates already use for the same three
/// shapes.
pub const GATED_VIEWPORTS: [(&str, Vec2); 3] = [
    ("desktop", REAL_VIEWPORT),
    ("full-viewport landscape", egui::Vec2::new(820.0, 640.0)),
    ("portrait", egui::Vec2::new(560.0, 900.0)),
];

/// Every route the manager dispatches to, in dispatch order; the contract
/// tests' `route_position` keeps it complete.
pub const EVERY_ROUTE: [SimulationPlanManagerMode; 8] = [
    SimulationPlanManagerMode::Browse,
    SimulationPlanManagerMode::Create,
    SimulationPlanManagerMode::Rename,
    SimulationPlanManagerMode::ConfirmArchive,
    SimulationPlanManagerMode::Compare,
    SimulationPlanManagerMode::Export,
    SimulationPlanManagerMode::Import,
    SimulationPlanManagerMode::Campaign,
];

/// Every source file of the plan manager, which the elision gate reads to tell
/// a string the module wrote with an ellipsis from one elided to fit.
pub const PLAN_MANAGER_SOURCES: &[(&str, &str)] = &[
    ("plan_manager.rs", include_str!("../plan_manager.rs")),
    ("tests.rs", include_str!("tests.rs")),
    ("campaign.rs", include_str!("campaign.rs")),
    ("compare.rs", include_str!("compare.rs")),
    ("create.rs", include_str!("create.rs")),
    ("exchange.rs", include_str!("exchange.rs")),
    ("kit.rs", include_str!("kit.rs")),
    ("lifecycle.rs", include_str!("lifecycle.rs")),
    ("records.rs", include_str!("records.rs")),
];
