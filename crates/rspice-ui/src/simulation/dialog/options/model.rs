//! Simulator options.
//!
//! The tolerances, limits, and integration settings a run is executed under.

use super::{DampingStrategy, IntegrationMethod, MatrixSolver, ValidationError};

/// A named numerical policy: its label and how to build it.
pub type NamedPreset = (&'static str, fn() -> SimulationOptions);

/// The SPICE default transient truncation-error tolerance.
const fn default_trtol() -> f64 {
    7.0
}

/// Complete simulation options matching Cadence Spectre.
///
/// These options control all aspects of simulation accuracy, convergence,
/// and performance. Default values match industry-standard SPICE defaults.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(from = "PersistedSimulationOptions")]
pub struct SimulationOptions {
    pub reltol: f64,
    pub residual_reltol: f64,
    pub vntol: f64,
    pub abstol: f64,
    pub iabstol: f64,
    pub chgtol: f64,
    pub pivrel: f64,
    pub pivtol: f64,
    pub itl1: usize,
    pub itl4: usize,
    /// Transient truncation-error tolerance. Scales the local truncation
    /// error the timestep controller will accept, so it is the one knob that
    /// trades transient run time against waveform fidelity directly.
    #[serde(default = "default_trtol")]
    pub trtol: f64,
    /// Relative bound on the accepted local truncation error. `None` leaves
    /// the engine's own bound in force rather than asserting one.
    #[serde(default)]
    pub transient_lte_reltol: Option<f64>,
    /// Absolute bound on the accepted local truncation error.
    #[serde(default)]
    pub transient_lte_abstol: Option<f64>,
    /// Seed for the deck's statistical expression functions — `agauss`,
    /// `gauss`, `unif`, `aunif`, and two-argument `limit`.
    ///
    /// `None` leaves the engine's default stream. Setting it is what makes a
    /// deck containing statistical parameters reproducible: without a stated
    /// seed the same project can be re-run and disagree with itself, and a
    /// result nobody can reproduce is not evidence.
    #[serde(default)]
    pub statistical_seed: Option<u64>,
    pub gmin_stepping: bool,
    pub source_stepping: bool,
    pub pseudo_transient: bool,
    pub arc_length: bool,
    pub gmin: f64,
    pub damping: DampingStrategy,
    pub method: IntegrationMethod,
    pub solver: MatrixSolver,
    pub bypass_enabled: bool,
    pub bypass_reltol: f64,
    pub bypass_abstol: f64,
    pub min_timestep: f64,
    pub max_timestep: f64,
    pub timestep_factor: f64,
    pub temp: f64,
    pub tnom: f64,
    pub verbose: bool,
    pub save_internals: bool,
}

/// Persisted options. New fields serialize; retired fields only decode.
///
/// `itl2` named a DC-transfer-curve iteration budget that no solver path
/// reads: the sweep and the operating point share one Newton budget. It is
/// accepted so earlier projects still open, and is never written back.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedSimulationOptions {
    #[serde(default)]
    #[allow(dead_code)]
    itl2: serde::de::IgnoredAny,
    reltol: f64,
    residual_reltol: f64,
    vntol: f64,
    abstol: f64,
    iabstol: f64,
    chgtol: f64,
    pivrel: f64,
    pivtol: f64,
    itl1: usize,
    itl4: usize,
    #[serde(default = "default_trtol")]
    trtol: f64,
    #[serde(default)]
    transient_lte_reltol: Option<f64>,
    #[serde(default)]
    transient_lte_abstol: Option<f64>,
    #[serde(default)]
    statistical_seed: Option<u64>,
    gmin_stepping: bool,
    source_stepping: bool,
    pseudo_transient: bool,
    arc_length: bool,
    gmin: f64,
    damping: DampingStrategy,
    method: IntegrationMethod,
    solver: MatrixSolver,
    bypass_enabled: bool,
    bypass_reltol: f64,
    bypass_abstol: f64,
    min_timestep: f64,
    max_timestep: f64,
    timestep_factor: f64,
    temp: f64,
    tnom: f64,
    verbose: bool,
    save_internals: bool,
}

impl From<PersistedSimulationOptions> for SimulationOptions {
    fn from(fields: PersistedSimulationOptions) -> Self {
        Self {
            reltol: fields.reltol,
            residual_reltol: fields.residual_reltol,
            vntol: fields.vntol,
            abstol: fields.abstol,
            iabstol: fields.iabstol,
            chgtol: fields.chgtol,
            pivrel: fields.pivrel,
            pivtol: fields.pivtol,
            itl1: fields.itl1,
            itl4: fields.itl4,
            trtol: fields.trtol,
            transient_lte_reltol: fields.transient_lte_reltol,
            transient_lte_abstol: fields.transient_lte_abstol,
            statistical_seed: fields.statistical_seed,
            gmin_stepping: fields.gmin_stepping,
            source_stepping: fields.source_stepping,
            pseudo_transient: fields.pseudo_transient,
            arc_length: fields.arc_length,
            gmin: fields.gmin,
            damping: fields.damping,
            method: fields.method,
            solver: fields.solver,
            bypass_enabled: fields.bypass_enabled,
            bypass_reltol: fields.bypass_reltol,
            bypass_abstol: fields.bypass_abstol,
            min_timestep: fields.min_timestep,
            max_timestep: fields.max_timestep,
            timestep_factor: fields.timestep_factor,
            temp: fields.temp,
            tnom: fields.tnom,
            verbose: fields.verbose,
            save_internals: fields.save_internals,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_selection_and_pivrel_reach_core_configuration() {
        for (solver, expected) in [
            (MatrixSolver::Lu, None),
            (
                MatrixSolver::SparseLu,
                Some(rspice_core::solver::RealSolverBackend::Faer),
            ),
            (
                MatrixSolver::Klu,
                Some(rspice_core::solver::RealSolverBackend::Klu),
            ),
            (MatrixSolver::Gmres, None),
        ] {
            let options = SimulationOptions {
                solver,
                pivrel: 0.125,
                pivtol: 2.5e-14,
                ..SimulationOptions::default()
            };
            let config = options.resolve_simulation_config(None);
            assert_eq!(config.matrix_solver, expected);
            assert_eq!(config.matrix_pivot_tolerance, 0.125);
            assert_eq!(config.matrix_absolute_pivot_tolerance, 2.5e-14);
        }
    }

    #[test]
    fn truncation_error_controls_reach_the_core_configuration() {
        let options = SimulationOptions {
            trtol: 1.5,
            transient_lte_reltol: Some(2.5e-4),
            transient_lte_abstol: Some(3.5e-9),
            ..SimulationOptions::default()
        };

        let overrides = options.simulation_config_overrides();

        assert_eq!(overrides.transient_trtol, Some(1.5));
        assert_eq!(overrides.transient_lte_reltol, Some(2.5e-4));
        assert_eq!(overrides.transient_lte_abstol, Some(3.5e-9));
    }

    #[test]
    fn an_unset_truncation_bound_leaves_the_engines_own_in_force() {
        let overrides = SimulationOptions::default().simulation_config_overrides();

        assert_eq!(
            overrides.transient_trtol,
            Some(7.0),
            "TRTOL always has a value, and it is the SPICE default"
        );
        assert_eq!(overrides.transient_lte_reltol, None);
        assert_eq!(overrides.transient_lte_abstol, None);
    }

    #[test]
    fn projects_written_before_trtol_existed_open_on_the_spice_default() {
        let mut persisted =
            serde_json::to_value(SimulationOptions::default()).expect("options encode");
        let object = persisted.as_object_mut().expect("options are an object");
        object.remove("trtol");
        object.remove("transient_lte_reltol");
        object.remove("transient_lte_abstol");

        let restored: SimulationOptions =
            serde_json::from_value(persisted).expect("legacy options decode");

        assert_eq!(restored.trtol, 7.0);
        assert_eq!(restored.transient_lte_reltol, None);
    }

    #[test]
    fn a_stated_statistical_seed_reaches_the_deck() {
        let options = SimulationOptions {
            statistical_seed: Some(20260811),
            ..SimulationOptions::default()
        };

        assert!(
            options.to_spice_options().contains("SEED=20260811"),
            "the parser seeds the statistical stream from the deck, so the seed must be in it"
        );
    }

    #[test]
    fn no_seed_emits_no_seed_line() {
        assert!(!SimulationOptions::default().to_spice_options().contains("SEED"));
    }

    #[test]
    fn spice_export_preserves_nondefault_pivot_controls() {
        let options = SimulationOptions {
            pivrel: 0.25,
            pivtol: 2.0e-14,
            ..SimulationOptions::default()
        };
        let text = options.to_spice_options();
        assert!(text.contains("PIVREL=2.50e-1"));
        assert!(text.contains("PIVTOL=2.00e-14"));
    }
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            reltol: 1e-3,
            residual_reltol: 1e-3,
            vntol: 1e-6,
            abstol: 1e-12,
            iabstol: 1e-12,
            chgtol: 1e-14,
            pivrel: 1e-3,
            pivtol: 1e-13,
            itl1: 50,
            itl4: 6,
            trtol: default_trtol(),
            transient_lte_reltol: None,
            transient_lte_abstol: None,
            statistical_seed: None,
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: false,
            gmin: 1e-12,
            damping: DampingStrategy::VoltageLimiting,
            method: IntegrationMethod::TrapGear,
            solver: MatrixSolver::Lu,
            bypass_enabled: false,
            bypass_reltol: 1e-3,
            bypass_abstol: 1e-6,
            min_timestep: 1e-15,
            max_timestep: 1e-3,
            timestep_factor: 8.0,
            temp: 27.0,
            tnom: 27.0,
            verbose: false,
            save_internals: false,
        }
    }
}

impl SimulationOptions {
    /// Create options optimized for fast simulation (loose tolerances).
    pub fn fast() -> Self {
        Self {
            reltol: 1e-2,
            residual_reltol: 1e-2,
            abstol: 1e-9,
            iabstol: 1e-9,
            itl1: 30,
            itl4: 4,
            gmin_stepping: false,
            source_stepping: false,
            pseudo_transient: false,
            bypass_enabled: true,
            ..Default::default()
        }
    }

    /// Create options optimized for accuracy (tight tolerances).
    pub fn accurate() -> Self {
        Self {
            reltol: 1e-4,
            residual_reltol: 1e-4,
            vntol: 1e-7,
            abstol: 1e-14,
            iabstol: 1e-14,
            chgtol: 1e-16,
            itl1: 100,
            itl4: 10,
            min_timestep: 1e-18,
            damping: DampingStrategy::Combined,
            ..Default::default()
        }
    }

    /// Create options optimized for difficult/stiff circuits.
    pub fn robust() -> Self {
        Self {
            reltol: 1e-3,
            residual_reltol: 1e-3,
            itl1: 200,
            itl4: 20,
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: true,
            gmin: 1e-10,
            method: IntegrationMethod::Gear2Only,
            damping: DampingStrategy::Combined,
            timestep_factor: 16.0,
            ..Default::default()
        }
    }

    /// The named presets, in the order a chooser should offer them: loosest
    /// first, then the shipping default, then the two that trade time for
    /// convergence.
    pub const PRESETS: [NamedPreset; 4] = [
        ("Fast", Self::fast),
        ("Balanced", Self::default),
        ("Accurate", Self::accurate),
        ("Robust", Self::robust),
    ];

    /// Which named preset these options are exactly, if any.
    ///
    /// Compared by serialized value rather than by a remembered "last preset
    /// pressed": editing any field leaves the preset, and reporting otherwise
    /// would misstate what the next run will use. Returns `None` for options
    /// that match no preset, which is a real state rather than an error.
    #[must_use]
    pub fn preset_name(&self) -> Option<&'static str> {
        let current = serde_json::to_vec(self).ok()?;
        Self::PRESETS.iter().find_map(|(label, build)| {
            serde_json::to_vec(&build())
                .ok()
                .filter(|preset| *preset == current)
                .map(|_| *label)
        })
    }

    /// The active preset's name, or `Custom` when the options match none.
    #[must_use]
    pub fn preset_label(&self) -> String {
        self.preset_name()
            .map_or_else(|| "Custom".to_owned(), str::to_owned)
    }

    pub fn temp_kelvin(&self) -> f64 {
        self.temp + 273.15
    }

    fn core_damping_strategy(&self) -> rspice_core::engine::DampingStrategy {
        match self.damping {
            DampingStrategy::None => rspice_core::engine::DampingStrategy::None,
            DampingStrategy::LineSearch => rspice_core::engine::DampingStrategy::LineSearch,
            DampingStrategy::VoltageLimiting => {
                rspice_core::engine::DampingStrategy::VoltageLimiting
            }
            DampingStrategy::BankRose => rspice_core::engine::DampingStrategy::BankRose,
            DampingStrategy::Combined => rspice_core::engine::DampingStrategy::Combined,
        }
    }

    fn core_integration_method(&self) -> rspice_core::numerics::integration::IntegrationMethod {
        match self.method {
            IntegrationMethod::Trap => {
                rspice_core::numerics::integration::IntegrationMethod::Trapezoidal
            }
            IntegrationMethod::Euler => {
                rspice_core::numerics::integration::IntegrationMethod::BackwardEuler
            }
            IntegrationMethod::Gear => rspice_core::numerics::integration::IntegrationMethod::Gear2,
            IntegrationMethod::Gear2 => {
                rspice_core::numerics::integration::IntegrationMethod::Gear2
            }
            IntegrationMethod::TrapGear => {
                rspice_core::numerics::integration::IntegrationMethod::TrapGear
            }
            IntegrationMethod::Gear2Only => {
                rspice_core::numerics::integration::IntegrationMethod::Gear2
            }
        }
    }

    fn simulation_config_overrides(&self) -> rspice_core::SimulationConfigOverrides {
        rspice_core::SimulationConfigOverrides {
            temperature_kelvin: Some(self.temp_kelvin()),
            max_iterations: Some(self.itl1),
            min_timestep: Some(self.min_timestep),
            max_timestep: Some(self.max_timestep),
            integration_method: Some(self.core_integration_method()),
            transient_trtol: Some(self.trtol),
            transient_lte_reltol: self.transient_lte_reltol,
            transient_lte_abstol: self.transient_lte_abstol,
            transient_timeint_max_timestep: None,
            transient_use_device_max_timestep: None,
            transient_nonlinear_reltol: None,
            transient_nonlinear_abstol: None,
            transient_nonlinear_deltaxtol: None,
            transient_nonlinear_rhstol: None,
            transient_nonlinear_max_iterations: None,
            transient_enforce_device_convergence: None,
            transient_nonlinear_nox: None,
            transient_lte_reference: None,
            transient_new_bp_stepping: None,
            convergence_preset: None,
            reltol: Some(self.reltol),
            abstol: Some(self.abstol),
            voltage_abstol: Some(self.vntol),
            current_abstol: Some(self.iabstol),
            charge_abstol: Some(self.chgtol),
            residual_reltol: Some(self.residual_reltol),
            gmin_initial: Some(self.gmin),
            spice_dialect: None,
            jfet_level2_model: None,
            ramptime: None,
            digital_delay_type: None,
        }
    }

    /// Resolve to core simulation config using layered precedence:
    /// core defaults < netlist `.OPTIONS` < UI options.
    pub fn resolve_simulation_config(
        &self,
        netlist_options: Option<&rspice_core::netlist::SimulationOptions>,
    ) -> rspice_core::engine::SimulationConfig {
        use rspice_core::engine::{BypassConfig, SimulationConfig};

        let overrides = self.simulation_config_overrides();
        let mut sim_config = rspice_core::resolve_simulation_config(
            &SimulationConfig::default(),
            netlist_options,
            &overrides,
        );

        sim_config.bypass_config = BypassConfig {
            enabled: self.bypass_enabled,
            reltol: self.bypass_reltol,
            abstol: self.bypass_abstol,
        };

        sim_config.convergence_config.gmin_stepping = self.gmin_stepping;
        sim_config.convergence_config.source_stepping = self.source_stepping;
        sim_config.convergence_config.pseudo_transient = self.pseudo_transient;
        sim_config.convergence_config.arc_length = self.arc_length;
        sim_config.convergence_config.damping_strategy = self.core_damping_strategy();
        sim_config.convergence_config.verbose = self.verbose;
        sim_config.convergence_config.gmin_target = sim_config
            .convergence_config
            .gmin_target
            .min(sim_config.convergence_config.gmin_initial);
        sim_config.matrix_solver = self.solver.core_backend_override();
        sim_config.matrix_pivot_tolerance = self.pivrel;
        sim_config.matrix_absolute_pivot_tolerance = self.pivtol;

        sim_config
    }

    /// Validate all options.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        if self.reltol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("reltol", self.reltol));
        }
        if self.residual_reltol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance(
                "residual_reltol",
                self.residual_reltol,
            ));
        }
        if self.vntol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("vntol", self.vntol));
        }
        if self.abstol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("abstol", self.abstol));
        }
        if self.iabstol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("iabstol", self.iabstol));
        }
        if self.chgtol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("chgtol", self.chgtol));
        }
        if !self.pivrel.is_finite() || self.pivrel <= 0.0 || self.pivrel > 1.0 {
            errors.push(ValidationError::InvalidTolerance("pivrel", self.pivrel));
        }
        if !self.pivtol.is_finite() || self.pivtol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("pivtol", self.pivtol));
        }

        if self.itl1 == 0 {
            errors.push(ValidationError::InvalidIteration("itl1", self.itl1));
        }
        if self.itl4 == 0 {
            errors.push(ValidationError::InvalidIteration("itl4", self.itl4));
        }

        if self.min_timestep <= 0.0 {
            errors.push(ValidationError::InvalidTimestep(
                "min_timestep",
                self.min_timestep,
            ));
        }
        if self.max_timestep <= 0.0 {
            errors.push(ValidationError::InvalidTimestep(
                "max_timestep",
                self.max_timestep,
            ));
        }
        if self.min_timestep >= self.max_timestep {
            errors.push(ValidationError::TimestepOrder(
                self.min_timestep,
                self.max_timestep,
            ));
        }

        if self.temp <= -273.15 {
            errors.push(ValidationError::InvalidTemperature("temp", self.temp));
        }
        if self.tnom < -273.15 {
            errors.push(ValidationError::InvalidTemperature("tnom", self.tnom));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Export as SPICE .options string.
    pub fn to_spice_options(&self) -> String {
        let mut lines = vec![".OPTIONS".to_string()];
        let default = Self::default();

        if (self.reltol - default.reltol).abs() > 1e-15 {
            lines.push(format!("+ RELTOL={:.2e}", self.reltol));
        }
        if (self.residual_reltol - default.residual_reltol).abs() > 1e-15 {
            lines.push(format!("+ RESIDUAL_RELTOL={:.2e}", self.residual_reltol));
        }
        if (self.abstol - default.abstol).abs() > 1e-20 {
            lines.push(format!("+ ABSTOL={:.2e}", self.abstol));
        }
        if (self.vntol - default.vntol).abs() > 1e-12 {
            lines.push(format!("+ VNTOL={:.2e}", self.vntol));
        }
        if self.pivrel.to_bits() != default.pivrel.to_bits() {
            lines.push(format!("+ PIVREL={:.2e}", self.pivrel));
        }
        if self.pivtol.to_bits() != default.pivtol.to_bits() {
            lines.push(format!("+ PIVTOL={:.2e}", self.pivtol));
        }
        if self.itl1 != default.itl1 {
            lines.push(format!("+ ITL1={}", self.itl1));
        }
        if self.itl4 != default.itl4 {
            lines.push(format!("+ ITL4={}", self.itl4));
        }
        if self.trtol.to_bits() != default.trtol.to_bits() {
            lines.push(format!("+ TRTOL={}", self.trtol));
        }
        // Emitted rather than applied through the override path: the parser
        // seeds the statistical stream before any parameter is evaluated, so
        // the seed has to be in the deck to reach the draws it governs.
        if let Some(seed) = self.statistical_seed {
            lines.push(format!("+ SEED={seed}"));
        }
        if self.method != default.method {
            lines.push(format!("+ METHOD={}", self.method.spice_name()));
        }
        if (self.temp - default.temp).abs() > 0.01 {
            lines.push(format!("+ TEMP={:.2}", self.temp));
        }
        if (self.tnom - default.tnom).abs() > 0.01 {
            lines.push(format!("+ TNOM={:.2}", self.tnom));
        }
        if (self.gmin - default.gmin).abs() > 1e-20 {
            lines.push(format!("+ GMIN={:.2e}", self.gmin));
        }

        lines.join("\n")
    }
}
