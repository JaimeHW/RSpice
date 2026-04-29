use super::{
    DampingStrategy, IntegrationMethod, MatrixSolver, ParseError, ValidationError, parse_si_value,
};

/// Complete simulation options matching Cadence Spectre.
///
/// These options control all aspects of simulation accuracy, convergence,
/// and performance. Default values match industry-standard SPICE defaults.
#[derive(Debug, Clone)]
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
    pub itl2: usize,
    pub itl4: usize,
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
            itl2: 100,
            itl4: 6,
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

    pub fn temp_kelvin(&self) -> f64 {
        self.temp + 273.15
    }

    pub fn tnom_kelvin(&self) -> f64 {
        self.tnom + 273.15
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

    fn core_integration_method(&self) -> rspice_core::analysis::IntegrationMethod {
        match self.method {
            IntegrationMethod::Trap => rspice_core::analysis::IntegrationMethod::Trapezoidal,
            IntegrationMethod::Euler => rspice_core::analysis::IntegrationMethod::BackwardEuler,
            IntegrationMethod::Gear => rspice_core::analysis::IntegrationMethod::Gear2,
            IntegrationMethod::Gear2 => rspice_core::analysis::IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear => rspice_core::analysis::IntegrationMethod::TrapGear,
            IntegrationMethod::Gear2Only => rspice_core::analysis::IntegrationMethod::Gear2,
        }
    }

    fn simulation_config_overrides(&self) -> rspice_core::SimulationConfigOverrides {
        rspice_core::SimulationConfigOverrides {
            temperature_kelvin: Some(self.temp_kelvin()),
            max_iterations: Some(self.itl1),
            min_timestep: Some(self.min_timestep),
            max_timestep: Some(self.max_timestep),
            integration_method: Some(self.core_integration_method()),
            convergence_preset: None,
            reltol: Some(self.reltol),
            abstol: Some(self.abstol),
            voltage_abstol: Some(self.vntol),
            current_abstol: Some(self.iabstol),
            charge_abstol: Some(self.chgtol),
            residual_reltol: Some(self.residual_reltol),
            gmin_initial: Some(self.gmin),
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

        sim_config
    }

    /// Convert to rspice_core SimulationConfig for engine use.
    pub fn to_simulation_config(&self) -> rspice_core::engine::SimulationConfig {
        self.resolve_simulation_config(None)
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

        if self.temp < -273.15 {
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
        if self.itl1 != default.itl1 {
            lines.push(format!("+ ITL1={}", self.itl1));
        }
        if self.itl4 != default.itl4 {
            lines.push(format!("+ ITL4={}", self.itl4));
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

    /// Parse from SPICE .options lines.
    pub fn from_spice_options(text: &str) -> Result<Self, ParseError> {
        let mut opts = Self::default();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('*') {
                continue;
            }

            let content = line
                .trim_start_matches(".OPTIONS")
                .trim_start_matches(".options")
                .trim_start_matches('+')
                .trim();

            for part in content.split_whitespace() {
                if let Some((key, val)) = part.split_once('=') {
                    match key.to_uppercase().as_str() {
                        "RELTOL" => opts.reltol = parse_si_value(val)?,
                        "RESIDUAL_RELTOL" | "RESRELTOL" => {
                            opts.residual_reltol = parse_si_value(val)?
                        }
                        "ABSTOL" => opts.abstol = parse_si_value(val)?,
                        "VNTOL" => opts.vntol = parse_si_value(val)?,
                        "IABSTOL" => opts.iabstol = parse_si_value(val)?,
                        "CHGTOL" => opts.chgtol = parse_si_value(val)?,
                        "PIVREL" => opts.pivrel = parse_si_value(val)?,
                        "PIVTOL" => opts.pivtol = parse_si_value(val)?,
                        "GMIN" => opts.gmin = parse_si_value(val)?,
                        "ITL1" => opts.itl1 = parse_usize_option(val)?,
                        "ITL2" => opts.itl2 = parse_usize_option(val)?,
                        "ITL4" => opts.itl4 = parse_usize_option(val)?,
                        "METHOD" => {
                            if let Some(m) = IntegrationMethod::from_spice(val) {
                                opts.method = m;
                            }
                        }
                        "TEMP" => opts.temp = parse_si_value(val)?,
                        "TNOM" => opts.tnom = parse_si_value(val)?,
                        _ => {}
                    }
                }
            }
        }

        Ok(opts)
    }
}

fn parse_usize_option(value: &str) -> Result<usize, ParseError> {
    value
        .parse()
        .map_err(|_| ParseError::InvalidNumber(value.to_string()))
}
