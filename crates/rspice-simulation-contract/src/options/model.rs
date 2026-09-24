//! Simulator options.
//!
//! The tolerances, limits, and integration settings a run is executed under.

use super::{
    DampingStrategy, IntegrationMethod, MatrixSolver, SimulationCompatibility, ValidationError,
};

/// A named numerical policy: its label and how to build it.
pub type NamedPreset = (&'static str, fn() -> SimulationOptions);

/// The SPICE default transient truncation-error tolerance.
const fn default_trtol() -> f64 {
    7.0
}

/// Solver tolerances, limits, and numerical policies authored by a simulation plan.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(from = "PersistedSimulationOptions")]
pub struct SimulationOptions {
    /// Numerical and available device-model variants, not the source syntax.
    #[serde(default, skip_serializing_if = "SimulationCompatibility::is_inherited")]
    pub compatibility: SimulationCompatibility,
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
    pub temp: f64,
    pub tnom: f64,
}

/// Persisted options. New fields serialize; retired fields only decode.
///
/// Every field below that decodes into nothing named a control the engine
/// never read, so a project written before it was retired still opens and
/// simply stops carrying it:
///
/// - `itl2` was a DC-transfer-curve iteration budget, but the sweep and the
///   operating point share one Newton budget.
/// - `timestep_factor` claimed to set the transient step growth ratio, which
///   is the compile-time `constants::TIMESTEP_GROWTH_MAX`, not a setting.
/// - `verbose` fed `ConvergenceConfig::verbose`, which no solver path reads.
/// - `save_internals` had no engine field at all; internal device nodes are
///   requested per signal on a `.SAVE` card.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedSimulationOptions {
    #[serde(default)]
    compatibility: SimulationCompatibility,
    #[serde(default, rename = "itl2")]
    _itl2: serde::de::IgnoredAny,
    #[serde(default, rename = "timestep_factor")]
    _timestep_factor: serde::de::IgnoredAny,
    #[serde(default, rename = "verbose")]
    _verbose: serde::de::IgnoredAny,
    #[serde(default, rename = "save_internals")]
    _save_internals: serde::de::IgnoredAny,
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
    temp: f64,
    tnom: f64,
}

impl From<PersistedSimulationOptions> for SimulationOptions {
    fn from(fields: PersistedSimulationOptions) -> Self {
        Self {
            compatibility: fields.compatibility,
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
            temp: fields.temp,
            tnom: fields.tnom,
        }
    }
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            compatibility: SimulationCompatibility::Inherit,
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
            temp: 27.0,
            tnom: 27.0,
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
            method: IntegrationMethod::Gear2,
            damping: DampingStrategy::Combined,
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
    /// pressed": editing a numerical field leaves the preset. Compatibility
    /// is selected independently and is preserved when applying a preset.
    /// Returns `None` for options that match no preset.
    #[must_use]
    pub fn preset_name(&self) -> Option<&'static str> {
        let current = serde_json::to_vec(self).ok()?;
        Self::PRESETS.iter().find_map(|(label, build)| {
            let mut preset = build();
            preset.compatibility = self.compatibility;
            serde_json::to_vec(&preset)
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

    fn simulation_config_overrides(&self) -> rspice_core::SimulationConfigOverrides {
        rspice_core::SimulationConfigOverrides {
            temperature_kelvin: Some(self.temp_kelvin()),
            max_iterations: Some(self.itl1),
            min_timestep: Some(self.min_timestep),
            max_timestep: Some(self.max_timestep),
            integration_method: Some(self.method.core()),
            transient_trtol: Some(self.trtol),
            transient_event_flux_abstol: None,
            transient_lte_reltol: self.transient_lte_reltol,
            transient_lte_abstol: self.transient_lte_abstol,
            transient_timeint_max_timestep: None,
            transient_use_device_max_timestep: None,
            transient_error_control: None,
            transient_min_steps_between_breakpoints: None,
            transient_timeint_nlmin: None,
            transient_timeint_nlmax: None,
            transient_timeint_min_order: None,
            transient_timeint_max_order: None,
            transient_timesteps_reversal: None,
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
            device_voltage_limiting: None,
            spice_dialect: self.compatibility.core_override(),
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
        sim_config.convergence_config.gmin_target = sim_config
            .convergence_config
            .gmin_target
            .min(sim_config.convergence_config.gmin_initial);
        sim_config.matrix_solver = self.solver.core_backend_override();
        sim_config.matrix_pivot_tolerance = self.pivrel;
        sim_config.matrix_absolute_pivot_tolerance = self.pivtol;

        sim_config
    }

    /// Validate the options before they become an effective solver policy.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        for (name, value) in [
            ("reltol", self.reltol),
            ("residual_reltol", self.residual_reltol),
            ("vntol", self.vntol),
            ("abstol", self.abstol),
            ("iabstol", self.iabstol),
            ("chgtol", self.chgtol),
            ("pivtol", self.pivtol),
            ("trtol", self.trtol),
        ] {
            if !value.is_finite() || value <= 0.0 {
                errors.push(ValidationError::InvalidTolerance(name, value));
            }
        }
        if !self.pivrel.is_finite() || self.pivrel <= 0.0 || self.pivrel > 1.0 {
            errors.push(ValidationError::InvalidPivotRelative(self.pivrel));
        }
        for (name, value) in [
            ("gmin", self.gmin),
            ("bypass_reltol", self.bypass_reltol),
            ("bypass_abstol", self.bypass_abstol),
        ] {
            if !value.is_finite() || value < 0.0 {
                errors.push(ValidationError::InvalidNonNegative(name, value));
            }
        }
        for (name, value) in [
            ("transient_lte_reltol", self.transient_lte_reltol),
            ("transient_lte_abstol", self.transient_lte_abstol),
        ] {
            if let Some(value) = value
                && (!value.is_finite() || value <= 0.0)
            {
                errors.push(ValidationError::InvalidTolerance(name, value));
            }
        }
        for (name, value) in [("itl1", self.itl1), ("itl4", self.itl4)] {
            if value == 0 {
                errors.push(ValidationError::InvalidIteration(name, value));
            }
        }
        for (name, value) in [
            ("min_timestep", self.min_timestep),
            ("max_timestep", self.max_timestep),
        ] {
            if !value.is_finite() || value <= 0.0 {
                errors.push(ValidationError::InvalidTimestep(name, value));
            }
        }
        if self.min_timestep >= self.max_timestep {
            errors.push(ValidationError::TimestepOrder(
                self.min_timestep,
                self.max_timestep,
            ));
        }
        for (name, value) in [("temp", self.temp), ("tnom", self.tnom)] {
            if !value.is_finite() || value <= -273.15 {
                errors.push(ValidationError::InvalidTemperature(name, value));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Export as SPICE .options string without rounding authored values.
    /// Exact default comparisons also preserve changes smaller than a
    /// display tolerance; these bytes are solver input.
    pub fn to_spice_options(&self) -> String {
        let mut lines = vec![".OPTIONS".to_string()];
        let default = Self::default();

        if let Some(policy) = self.compatibility.option_value() {
            lines.push(format!("+ RSPICE_DIALECT={policy}"));
        }
        if self.reltol != default.reltol {
            lines.push(format!("+ RELTOL={:e}", self.reltol));
        }
        if self.residual_reltol != default.residual_reltol {
            lines.push(format!("+ RESIDUAL_RELTOL={:e}", self.residual_reltol));
        }
        if self.abstol != default.abstol {
            lines.push(format!("+ ABSTOL={:e}", self.abstol));
        }
        if self.vntol != default.vntol {
            lines.push(format!("+ VNTOL={:e}", self.vntol));
        }
        if self.iabstol != default.iabstol {
            lines.push(format!("+ IABSTOL={:e}", self.iabstol));
        }
        if self.chgtol != default.chgtol {
            lines.push(format!("+ CHGTOL={:e}", self.chgtol));
        }
        if self.pivrel.to_bits() != default.pivrel.to_bits() {
            lines.push(format!("+ PIVREL={:e}", self.pivrel));
        }
        // The product policy's default is deliberately non-zero while the
        // core fallback is zero. Always state it so the ledger and solve
        // cannot disagree when the user leaves this field untouched.
        lines.push(format!("+ PIVTOL={:e}", self.pivtol));
        if self.itl1 != default.itl1 {
            lines.push(format!("+ ITL1={}", self.itl1));
        }
        // The product's transient Newton budget differs from the core
        // fallback. Omission would execute ten iterations while Studio shows
        // six, so this value is an explicit part of every prepared deck.
        lines.push(format!("+ ITL4={}", self.itl4));
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
        if self.temp != default.temp {
            lines.push(format!("+ TEMP={}", self.temp));
        }
        if self.tnom != default.tnom {
            lines.push(format!("+ TNOM={}", self.tnom));
        }
        if self.gmin != default.gmin {
            lines.push(format!("+ GMIN={:e}", self.gmin));
        }
        if self.gmin_stepping != default.gmin_stepping {
            lines.push(format!("+ GMINSTEPPING={}", u8::from(self.gmin_stepping)));
        }
        if self.source_stepping != default.source_stepping {
            lines.push(format!(
                "+ SOURCESTEPPING={}",
                u8::from(self.source_stepping)
            ));
        }
        if self.pseudo_transient != default.pseudo_transient {
            lines.push(format!(
                "+ PSEUDOTRANSIENT={}",
                u8::from(self.pseudo_transient)
            ));
        }
        if self.arc_length != default.arc_length {
            lines.push(format!("+ ARCLENGTH={}", u8::from(self.arc_length)));
        }
        // Unscoped, so the bounds ride the global card and re-scope nothing
        // after them. They are stated only alongside an enabled bypass: with
        // the feature off they select nothing, and emitting them would put a
        // key in the deck that changes no result.
        if self.bypass_enabled != default.bypass_enabled {
            lines.push(format!("+ BYPASS={}", u8::from(self.bypass_enabled)));
        }
        if self.bypass_enabled {
            if self.bypass_reltol != default.bypass_reltol {
                lines.push(format!("+ BYPASSRELTOL={:e}", self.bypass_reltol));
            }
            if self.bypass_abstol != default.bypass_abstol {
                lines.push(format!("+ BYPASSABSTOL={:e}", self.bypass_abstol));
            }
        }
        if self.damping != default.damping {
            lines.push(format!("+ DAMPING={}", self.damping.spice_name()));
        }
        if let Some(backend) = self.solver.spice_name() {
            lines.push(format!("+ SOLVER={backend}"));
        }
        // The run's step ceiling is unscoped. `TIMEINT DELMAX` is the time
        // integrator's own ceiling and belongs to the per-analysis override
        // record; stating the plan's ceiling under that key would make one of
        // the two bounds unstatable.
        // The core fallback is unbounded; Studio's product policy is not.
        lines.push(format!("+ MAXTIMESTEP={:e}", self.max_timestep));

        // The parser's package selector stays in force for the rest of the
        // `.OPTIONS` command it appears on, so a scoped key placed among the
        // global ones would re-scope every key after it. The timestep
        // integrator's settings therefore get their own card.
        let mut timeint = Vec::new();
        if let Some(reltol) = self.transient_lte_reltol {
            timeint.push(format!("+ RELTOL={reltol:e}"));
        }
        if let Some(abstol) = self.transient_lte_abstol {
            timeint.push(format!("+ ABSTOL={abstol:e}"));
        }
        // The product floor is lower than the core fallback and therefore
        // must be stated even for the untouched shipping policy.
        timeint.push(format!("+ MINTIMESTEP={:e}", self.min_timestep));
        if !timeint.is_empty() {
            // A card whose only content is the global header states nothing,
            // and would re-scope nothing either; drop it rather than emit it.
            if lines.len() == 1 {
                lines.clear();
            }
            lines.push(".OPTIONS TIMEINT".to_string());
            lines.append(&mut timeint);
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests;
