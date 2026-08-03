//! Simulator option enumerations, including the integration methods.

/// Integration method for transient analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum IntegrationMethod {
    /// Trapezoidal rule (A-stable, 2nd order).
    Trap,
    /// Backward Euler (L-stable, 1st order).
    Euler,
    /// Gear's method (BDF, stiff circuits).
    Gear,
    /// Second-order Gear (BDF-2).
    Gear2,
    /// Automatic trap/gear switching (Spectre default).
    #[default]
    TrapGear,
    /// Gear only, no trap phase.
    Gear2Only,
}

impl IntegrationMethod {
    pub fn display_name(&self) -> &'static str {
        match self {
            IntegrationMethod::Trap => "Trapezoidal",
            IntegrationMethod::Euler => "Backward Euler",
            IntegrationMethod::Gear => "Gear (BDF)",
            IntegrationMethod::Gear2 => "Gear-2",
            IntegrationMethod::TrapGear => "Trap/Gear (Auto)",
            IntegrationMethod::Gear2Only => "Gear-2 Only",
        }
    }

    pub fn spice_name(&self) -> &'static str {
        match self {
            IntegrationMethod::Trap => "TRAP",
            IntegrationMethod::Euler => "EULER",
            IntegrationMethod::Gear => "GEAR",
            IntegrationMethod::Gear2 => "GEAR2",
            IntegrationMethod::TrapGear => "TRAPGEAR",
            IntegrationMethod::Gear2Only => "GEAR2ONLY",
        }
    }

    pub fn all() -> &'static [IntegrationMethod] {
        &[
            IntegrationMethod::Trap,
            IntegrationMethod::Euler,
            IntegrationMethod::Gear,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
            IntegrationMethod::Gear2Only,
        ]
    }
}

/// Damping strategy for Newton-Raphson convergence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum DampingStrategy {
    /// No damping (full Newton step).
    None,
    /// Backtracking line search (Armijo condition).
    LineSearch,
    /// Junction voltage limiting (SPICE-style).
    #[default]
    VoltageLimiting,
    /// Bank-Rose adaptive damping.
    BankRose,
    /// Combined: voltage limiting + line search.
    Combined,
}

impl DampingStrategy {
    pub fn display_name(&self) -> &'static str {
        match self {
            DampingStrategy::None => "None",
            DampingStrategy::LineSearch => "Line Search",
            DampingStrategy::VoltageLimiting => "Voltage Limiting",
            DampingStrategy::BankRose => "Bank-Rose",
            DampingStrategy::Combined => "Combined",
        }
    }

    pub fn all() -> &'static [DampingStrategy] {
        &[
            DampingStrategy::None,
            DampingStrategy::LineSearch,
            DampingStrategy::VoltageLimiting,
            DampingStrategy::BankRose,
            DampingStrategy::Combined,
        ]
    }
}

/// Matrix solver algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum MatrixSolver {
    /// Automatic measured routing between circuit and supernodal sparse LU.
    #[default]
    Lu,
    /// General supernodal sparse LU.
    SparseLu,
    /// Retired legacy serialized value; no longer presented by the UI.
    Gmres,
    /// RSpice's circuit-specialized sparse LU solver.
    Klu,
}

impl MatrixSolver {
    pub fn display_name(&self) -> &'static str {
        match self {
            MatrixSolver::Lu => "Automatic (Circuit/Supernodal LU)",
            MatrixSolver::SparseLu => "Supernodal Sparse LU",
            MatrixSolver::Gmres => "Automatic (legacy GMRES setting)",
            MatrixSolver::Klu => "RSpice Circuit LU",
        }
    }

    pub fn all() -> &'static [MatrixSolver] {
        &[MatrixSolver::Lu, MatrixSolver::SparseLu, MatrixSolver::Klu]
    }

    pub fn core_backend(self) -> rspice_core::solver::RealSolverBackend {
        match self {
            MatrixSolver::Lu | MatrixSolver::Gmres => rspice_core::solver::RealSolverBackend::Auto,
            MatrixSolver::SparseLu => rspice_core::solver::RealSolverBackend::Faer,
            MatrixSolver::Klu => rspice_core::solver::RealSolverBackend::Klu,
        }
    }
}
