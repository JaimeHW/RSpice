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

    pub fn from_spice(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRAP" | "TRAPEZOIDAL" => Some(IntegrationMethod::Trap),
            "EULER" | "BE" => Some(IntegrationMethod::Euler),
            "GEAR" | "BDF" => Some(IntegrationMethod::Gear),
            "GEAR2" => Some(IntegrationMethod::Gear2),
            "TRAPGEAR" | "AUTO" => Some(IntegrationMethod::TrapGear),
            "GEAR2ONLY" => Some(IntegrationMethod::Gear2Only),
            _ => None,
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
    /// LU decomposition with partial pivoting.
    #[default]
    Lu,
    /// Sparse LU (for large circuits).
    SparseLu,
    /// Iterative GMRES (for very large circuits).
    Gmres,
    /// Direct KLU solver (SuiteSparse).
    Klu,
}

impl MatrixSolver {
    pub fn display_name(&self) -> &'static str {
        match self {
            MatrixSolver::Lu => "LU Decomposition",
            MatrixSolver::SparseLu => "Sparse LU",
            MatrixSolver::Gmres => "GMRES (Iterative)",
            MatrixSolver::Klu => "KLU (SuiteSparse)",
        }
    }

    pub fn all() -> &'static [MatrixSolver] {
        &[
            MatrixSolver::Lu,
            MatrixSolver::SparseLu,
            MatrixSolver::Gmres,
            MatrixSolver::Klu,
        ]
    }
}
