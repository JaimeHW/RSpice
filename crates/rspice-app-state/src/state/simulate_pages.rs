//! The Simulate and Verify workspaces' pages: which page a command opens,
//! the order each workspace's navigator lists them in, and the page a
//! session reopens on. The session keeps them by name, so they sit in the
//! shared model with the Models workspace's pages.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VerificationPage {
    #[default]
    // A session saved while the placeholder DRC page existed restores to the
    // first page.
    #[serde(
        alias = "Cockpit",
        alias = "Specifications",
        alias = "Checks",
        alias = "Drc"
    )]
    Yield,
    Corners,
    Tuning,
    Optimization,
    DeviceSafety,
    #[serde(alias = "History")]
    Regression,
}

impl VerificationPage {
    pub const NAVIGATION: [Self; 6] = [
        Self::Yield,
        Self::Corners,
        Self::Tuning,
        Self::Optimization,
        Self::DeviceSafety,
        Self::Regression,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Yield => "PVT & Monte Carlo verification",
            Self::Corners => "Process-corner verification",
            Self::Tuning => "Live design-space exploration",
            Self::Optimization => "Optimization candidate",
            Self::DeviceSafety => "Safe-operating-area verification",
            Self::Regression => "Golden regression comparison",
        }
    }
}

/// Simulation Studio setup route.
///
/// The analyses page owns the ordered plan and its per-analysis forms; the
/// remaining pages own one plan-scoped concern each. Every route is backed by
/// state the run actually consumes, so a page can never present a control that
/// does not reach the engine.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum SimulationPage {
    #[default]
    Analyses,
    Excitations,
    Variables,
    Outputs,
    Specifications,
    RunSet,
    Models,
    Solver,
    Save,
}

impl SimulationPage {
    /// Navigation order, matching the order a plan is authored in: the
    /// analyses first, then what they are parameterized by, what they produce,
    /// what judges them, what they are swept over, what they are bound
    /// against, how they are solved, and what is kept.
    pub const NAVIGATION: [Self; 9] = [
        Self::Analyses,
        Self::Excitations,
        Self::Variables,
        Self::Outputs,
        Self::Specifications,
        Self::RunSet,
        Self::Models,
        Self::Solver,
        Self::Save,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Analyses => "Analyses",
            Self::Excitations => "Excitations",
            Self::Variables => "Design variables",
            Self::Outputs => "Outputs & expressions",
            Self::Specifications => "Requirements & specs",
            Self::RunSet => "PVT, sweeps & variation",
            Self::Models => "Models & sections",
            Self::Solver => "Solver & convergence",
            Self::Save => "Save & streaming policy",
        }
    }

    /// Page heading. Longer than the navigation label where the page owns more
    /// than its label implies.
    pub const fn title(self) -> &'static str {
        match self {
            Self::Analyses => "Analyses",
            Self::Excitations => "Excitations & the analyses that read them",
            Self::Variables => "Design variables",
            Self::Outputs => "Outputs & expressions",
            Self::Specifications => "Requirements & specifications",
            Self::RunSet => "PVT, sweeps & variation",
            Self::Models => "Models & sections",
            Self::Solver => "Solver, convergence & per-analysis overrides",
            Self::Save => "Save, streaming & retention policy",
        }
    }
}
