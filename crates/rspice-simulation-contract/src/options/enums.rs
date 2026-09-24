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

    /// The method a deck's `METHOD=` spelling names.
    ///
    /// The engine accepts several dialects for each integrator — SPICE's
    /// `TRAP`/`GEAR`, Xyce's numeric `7`/`8`, the hybrid's `AUTO` — and the
    /// table lives in `rspice_core::numerics::integration` where the enum
    /// does. This is the reader's side of it, and it is held to the engine's
    /// by `a_deck_method_spelling_means_the_same_thing_to_both_readers`: every
    /// spelling here is put through the engine's own `.PSS` parser and must
    /// come back as the same integrator, and a spelling the engine refuses
    /// must be refused here.
    pub fn from_spice_name(spelling: &str) -> Option<Self> {
        let matches = |candidates: &[&str]| {
            candidates
                .iter()
                .any(|candidate| spelling.eq_ignore_ascii_case(candidate))
        };
        if matches(&["TRAP", "TRAPEZOIDAL", "TRAPEZOID", "ONESTEP", "7"]) {
            Some(Self::Trap)
        } else if matches(&["EULER", "BE", "BACKWARDEULER"]) {
            Some(Self::Euler)
        } else if matches(&["GEAR", "BDF", "GEAR2", "8"]) {
            Some(Self::Gear2)
        } else if matches(&["TRAPGEAR", "AUTO"]) {
            Some(Self::TrapGear)
        } else {
            None
        }
    }

    /// Every deck spelling this reader accepts, grouped by the method it
    /// names.
    ///
    /// Exposed so the agreement test can enumerate them; nothing shipped
    /// reads it.
    #[cfg(test)]
    pub fn spice_spellings() -> &'static [&'static str] {
        &[
            "TRAP",
            "TRAPEZOIDAL",
            "TRAPEZOID",
            "ONESTEP",
            "7",
            "EULER",
            "BE",
            "BACKWARDEULER",
            "GEAR",
            "BDF",
            "GEAR2",
            "8",
            "TRAPGEAR",
            "AUTO",
        ]
    }

    /// The engine's own integrator this names.
    ///
    /// One owner for the mapping. The global options model resolved it into a
    /// `SimulationConfigOverrides` and PSS resolves it into a `PssConfig`, and
    /// two copies of a four-arm translation is how one of them ends up sending
    /// Gear where the other sends trapezoidal.
    pub fn core(self) -> rspice_core::numerics::integration::IntegrationMethod {
        use rspice_core::numerics::integration::IntegrationMethod as Core;
        match self {
            Self::Trap => Core::Trapezoidal,
            Self::Euler => Core::BackwardEuler,
            Self::Gear2 => Core::Gear2,
            Self::TrapGear => Core::TrapGear,
        }
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

    /// The studio's spelling of a resolved engine value.
    ///
    /// The inverse of `options::model`'s `core_damping_strategy`, for the one
    /// reader that reports what a resolved `SimulationConfig` holds rather
    /// than what someone authored: the advanced-options panel, when the
    /// analysis's accuracy tier assigned the strategy after the deck.
    #[must_use]
    pub const fn from_core(strategy: rspice_core::engine::DampingStrategy) -> Self {
        match strategy {
            rspice_core::engine::DampingStrategy::None => Self::None,
            rspice_core::engine::DampingStrategy::LineSearch => Self::LineSearch,
            rspice_core::engine::DampingStrategy::VoltageLimiting => Self::VoltageLimiting,
            rspice_core::engine::DampingStrategy::BankRose => Self::BankRose,
            rspice_core::engine::DampingStrategy::Combined => Self::Combined,
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

/// How a harmonic-balance solve builds its first iterate.
///
/// Xyce's `.OPTIONS HBINT TAHB`. The deck carries the integer, which is why
/// [`Self::spice_name`] returns a digit rather than a word: the engine's parser
/// reads `0`, `1` and `2` and nothing else. What a reader chooses is the
/// sentence, because "transient-assisted" is the fact and `1` is the encoding.
///
/// There is no automatic setting. Stating nothing leaves the engine on its own
/// default DC seed (`engine/hb.rs:1150`), and an analysis returns to it by
/// clearing the option rather than by naming a fourth mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum HbTimeDomainMode {
    /// `TAHB=0`: solve in the frequency domain from the zero state, with
    /// neither a transient nor a DC initial-state construction.
    #[default]
    Direct,
    /// `TAHB=1`: run a transient over the fundamental period first and seed
    /// the harmonics from its trajectory.
    TransientAssisted,
    /// `TAHB=2`: seed every collocation point from one DC operating point.
    DcOperatingPoint,
}

impl HbTimeDomainMode {
    /// What the chooser offers and the ledger reports.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Direct => "Frequency domain only",
            Self::TransientAssisted => "Transient-assisted",
            Self::DcOperatingPoint => "DC operating point",
        }
    }

    /// The `.OPTIONS HBINT TAHB` spelling the engine's parser accepts.
    #[must_use]
    pub const fn spice_name(self) -> &'static str {
        match self {
            Self::Direct => "0",
            Self::TransientAssisted => "1",
            Self::DcOperatingPoint => "2",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Direct,
            Self::TransientAssisted,
            Self::DcOperatingPoint,
        ]
    }
}

#[cfg(test)]
mod tests {
    /// A deck `METHOD=` spelling means the same integrator to both readers.
    ///
    /// [`super::IntegrationMethod::from_spice_name`] is
    /// this crate's copy of a table that lives in the engine, so it is held to
    /// the engine's by running every spelling through the engine's own `.PSS`
    /// parser. A spelling neither accepts is checked too: a table that said
    /// yes to everything would pass the first half alone.
    #[test]
    fn a_deck_method_spelling_means_the_same_thing_to_both_readers() {
        use super::IntegrationMethod;
        use rspice_core::Netlist;
        use rspice_core::netlist::AnalysisCommand;

        const CIRCUIT: &str = "pss method\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        let engine_method = |spelling: &str| {
            let deck = format!("{CIRCUIT}.pss fund=1Meg method={spelling}\n.end\n");
            let netlist = Netlist::parse(&deck).ok()?;
            match netlist.analyses.into_iter().next() {
                Some(AnalysisCommand::Pss(card)) => card.integration_method,
                other => panic!("expected a .PSS card, got {other:?}"),
            }
        };

        for spelling in IntegrationMethod::spice_spellings() {
            let reader = IntegrationMethod::from_spice_name(spelling)
                .unwrap_or_else(|| panic!("this reader accepts {spelling}"));
            assert_eq!(
                engine_method(spelling),
                Some(reader.core()),
                "`method={spelling}` names a different integrator to each reader"
            );
        }
        for refused in ["bdf2", "gear3", "spectre", ""] {
            assert_eq!(
                IntegrationMethod::from_spice_name(refused),
                None,
                "`method={refused}` is not a method the engine integrates under"
            );
            assert_eq!(engine_method(refused), None, "{refused}");
        }
    }
}
