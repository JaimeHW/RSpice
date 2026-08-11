//! Simulator option enumerations, including the integration methods.

/// Integration method for transient analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(from = "PersistedIntegrationMethod")]
pub enum IntegrationMethod {
    /// Trapezoidal rule (A-stable, 2nd order).
    Trap,
    /// Backward Euler (L-stable, 1st order).
    Euler,
    /// Second-order Gear (BDF-2), the only Gear order the engine integrates
    /// at.
    Gear2,
    /// Automatic trap/gear switching (Spectre default).
    #[default]
    TrapGear,
}

/// Persisted integration method. Current names serialize; retired ones only
/// decode.
///
/// The engine has exactly one Gear integrator and it is second order, so
/// every retired name here was another way of asking for `Gear2`. `Gear2Only`
/// named a Gear phase with no trap phase, which is what `Gear2` already is,
/// and the deck spelling it emitted — `METHOD=GEAR2ONLY` — matched no method
/// the engine knows, so a project that selected it ran the automatic
/// trap/gear default instead. `Gear` was the plain BDF label and resolved to
/// the same `Gear2` the survivor does. Both decode onto it.
#[derive(serde::Deserialize)]
enum PersistedIntegrationMethod {
    Trap,
    Euler,
    Gear,
    Gear2,
    TrapGear,
    Gear2Only,
}

impl From<PersistedIntegrationMethod> for IntegrationMethod {
    fn from(method: PersistedIntegrationMethod) -> Self {
        match method {
            PersistedIntegrationMethod::Trap => Self::Trap,
            PersistedIntegrationMethod::Euler => Self::Euler,
            PersistedIntegrationMethod::Gear
            | PersistedIntegrationMethod::Gear2
            | PersistedIntegrationMethod::Gear2Only => Self::Gear2,
            PersistedIntegrationMethod::TrapGear => Self::TrapGear,
        }
    }
}

impl IntegrationMethod {
    pub fn display_name(&self) -> &'static str {
        match self {
            IntegrationMethod::Trap => "Trapezoidal",
            IntegrationMethod::Euler => "Backward Euler",
            IntegrationMethod::Gear2 => "Gear-2 (BDF)",
            IntegrationMethod::TrapGear => "Trap/Gear (Auto)",
        }
    }

    pub fn spice_name(&self) -> &'static str {
        match self {
            IntegrationMethod::Trap => "TRAP",
            IntegrationMethod::Euler => "EULER",
            IntegrationMethod::Gear2 => "GEAR2",
            IntegrationMethod::TrapGear => "TRAPGEAR",
        }
    }

    pub fn all() -> &'static [IntegrationMethod] {
        &[
            IntegrationMethod::Trap,
            IntegrationMethod::Euler,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
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

    /// The `.OPTIONS DAMPING` spelling the engine's parser accepts.
    pub fn spice_name(&self) -> &'static str {
        match self {
            DampingStrategy::None => "NONE",
            DampingStrategy::LineSearch => "LINESEARCH",
            DampingStrategy::VoltageLimiting => "VOLTAGELIMITING",
            DampingStrategy::BankRose => "BANKROSE",
            DampingStrategy::Combined => "COMBINED",
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

    /// Return the explicit core-backend override selected by the user.
    ///
    /// `Lu` is the UI's automatic setting, so it must leave backend selection
    /// to the simulation dialect and matrix profile. The retired `Gmres`
    /// value is treated the same way when loading older settings.
    pub fn core_backend_override(self) -> Option<rspice_core::solver::RealSolverBackend> {
        match self {
            MatrixSolver::Lu | MatrixSolver::Gmres => None,
            MatrixSolver::SparseLu => Some(rspice_core::solver::RealSolverBackend::Faer),
            MatrixSolver::Klu => Some(rspice_core::solver::RealSolverBackend::Klu),
        }
    }

    /// The `.OPTIONS SOLVER` spelling for an explicitly chosen backend.
    ///
    /// Derived from [`Self::core_backend_override`] so the deck can never name
    /// a backend the resolved configuration would not have selected, and so
    /// the automatic settings stay silent rather than pinning `AUTO` and
    /// blocking the dialect profile.
    pub fn spice_name(self) -> Option<&'static str> {
        match self.core_backend_override()? {
            rspice_core::solver::RealSolverBackend::Auto => Some("AUTO"),
            rspice_core::solver::RealSolverBackend::Klu => Some("KLU"),
            rspice_core::solver::RealSolverBackend::Faer => Some("FAER"),
        }
    }
}
