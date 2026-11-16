//! The Simulate and Verify workspaces' pages: which page a command opens,
//! the order each workspace's navigator lists them in, and the page a
//! session reopens on. The session keeps them by name, so they sit in the
//! shared model with the Models workspace's pages.

use serde::{Deserialize, Serialize};

/// A view of the Verify workspace. Each has one name, [`Self::label`],
/// wherever the reader meets it: the menu, the navigator, the palette, the
/// view's toolbar and the inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VerificationPage {
    /// Every specification of the plan judged against the run: where Verify
    /// opens.
    #[default]
    Specifications,
    Corners,
    MonteCarlo,
    Checks,
    Reference,
}

impl VerificationPage {
    pub const NAVIGATION: [Self; 5] = [
        Self::Specifications,
        Self::Corners,
        Self::MonteCarlo,
        Self::Checks,
        Self::Reference,
    ];

    /// The view's one name.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Specifications => "Specifications",
            Self::Corners => "Corners",
            Self::MonteCarlo => "Monte Carlo",
            Self::Checks => "Checks",
            Self::Reference => "Reference",
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
    RunSet,
    Models,
    ConnectRules,
    Solver,
    Save,
}

impl SimulationPage {
    /// Navigation order, matching the order a plan is authored in: the
    /// analyses first, then what they are parameterized by, what they produce,
    /// what they are swept over, what they are bound against, what converts at
    /// their analog/digital boundaries, how they are solved, and what is kept.
    /// What judges them is Verify's.
    pub const NAVIGATION: [Self; 9] = [
        Self::Analyses,
        Self::Excitations,
        Self::Variables,
        Self::Outputs,
        Self::RunSet,
        Self::Models,
        Self::ConnectRules,
        Self::Solver,
        Self::Save,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Analyses => "Analyses",
            Self::Excitations => "Excitations",
            Self::Variables => "Design variables",
            Self::Outputs => "Outputs & expressions",
            Self::RunSet => "PVT, sweeps & variation",
            Self::Models => "Models & sections",
            Self::ConnectRules => "Connect rules",
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
            Self::RunSet => "PVT, sweeps & variation",
            Self::Models => "Models & sections",
            Self::ConnectRules => "Connect rules & inserted converters",
            Self::Solver => "Solver, convergence & per-analysis overrides",
            Self::Save => "Save, streaming & retention policy",
        }
    }
}
