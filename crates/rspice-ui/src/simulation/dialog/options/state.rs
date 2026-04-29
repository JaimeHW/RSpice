use super::{
    DampingStrategy, IntegrationMethod, MatrixSolver, SimulationOptions, format_si_value,
    parse_si_value,
};

/// UI state for options dialog (string buffers for text editing).
#[derive(Debug, Clone)]
pub struct OptionsDialogState {
    /// Active tab (0=Accuracy, 1=Convergence, 2=Algorithm, 3=Limits, 4=Advanced)
    pub active_tab: usize,
    pub reltol: String,
    pub residual_reltol: String,
    pub abstol: String,
    pub vntol: String,
    pub iabstol: String,
    pub chgtol: String,
    pub itl1: String,
    pub itl4: String,
    pub gmin: String,
    pub gmin_stepping: bool,
    pub source_stepping: bool,
    pub pseudo_transient: bool,
    pub damping: usize,
    pub method: usize,
    pub solver: usize,
    pub bypass_enabled: bool,
    pub bypass_reltol: String,
    pub bypass_abstol: String,
    pub min_timestep: String,
    pub max_timestep: String,
    pub timestep_factor: String,
    pub temp: String,
    pub tnom: String,
    pub verbose: bool,
    pub save_internals: bool,
}

impl Default for OptionsDialogState {
    fn default() -> Self {
        Self::from_options(&SimulationOptions::default())
    }
}

impl OptionsDialogState {
    /// Create dialog state from options.
    pub fn from_options(opts: &SimulationOptions) -> Self {
        Self {
            active_tab: 0,
            reltol: format_si_value(opts.reltol),
            residual_reltol: format_si_value(opts.residual_reltol),
            abstol: format_si_value(opts.abstol),
            vntol: format_si_value(opts.vntol),
            iabstol: format_si_value(opts.iabstol),
            chgtol: format_si_value(opts.chgtol),
            itl1: opts.itl1.to_string(),
            itl4: opts.itl4.to_string(),
            gmin: format_si_value(opts.gmin),
            gmin_stepping: opts.gmin_stepping,
            source_stepping: opts.source_stepping,
            pseudo_transient: opts.pseudo_transient,
            damping: DampingStrategy::all()
                .iter()
                .position(|d| *d == opts.damping)
                .unwrap_or(0),
            method: IntegrationMethod::all()
                .iter()
                .position(|m| *m == opts.method)
                .unwrap_or(0),
            solver: MatrixSolver::all()
                .iter()
                .position(|s| *s == opts.solver)
                .unwrap_or(0),
            bypass_enabled: opts.bypass_enabled,
            bypass_reltol: format_si_value(opts.bypass_reltol),
            bypass_abstol: format_si_value(opts.bypass_abstol),
            min_timestep: format_si_value(opts.min_timestep),
            max_timestep: format_si_value(opts.max_timestep),
            timestep_factor: opts.timestep_factor.to_string(),
            temp: format!("{:.1}", opts.temp),
            tnom: format!("{:.1}", opts.tnom),
            verbose: opts.verbose,
            save_internals: opts.save_internals,
        }
    }

    /// Convert dialog state to options (with validation).
    pub fn to_options(&self) -> Result<SimulationOptions, Vec<String>> {
        let mut errors = Vec::new();

        let reltol = parse_field(&self.reltol, "reltol", 1e-3, &mut errors);
        let residual_reltol =
            parse_field(&self.residual_reltol, "residual_reltol", 1e-3, &mut errors);
        let abstol = parse_field(&self.abstol, "abstol", 1e-12, &mut errors);
        let vntol = parse_field(&self.vntol, "vntol", 1e-6, &mut errors);
        let iabstol = parse_field(&self.iabstol, "iabstol", 1e-12, &mut errors);
        let chgtol = parse_field(&self.chgtol, "chgtol", 1e-14, &mut errors);
        let gmin = parse_field(&self.gmin, "gmin", 1e-12, &mut errors);
        let min_timestep = parse_field(&self.min_timestep, "min_timestep", 1e-15, &mut errors);
        let max_timestep = parse_field(&self.max_timestep, "max_timestep", 1e-3, &mut errors);
        let bypass_reltol = parse_field(&self.bypass_reltol, "bypass_reltol", 1e-3, &mut errors);
        let bypass_abstol = parse_field(&self.bypass_abstol, "bypass_abstol", 1e-6, &mut errors);

        let itl1 = parse_usize_field(&self.itl1, "itl1", 50, &mut errors);
        let itl4 = parse_usize_field(&self.itl4, "itl4", 6, &mut errors);
        let timestep_factor =
            parse_float_field(&self.timestep_factor, "timestep_factor", 8.0, &mut errors);
        let temp = parse_float_field(&self.temp, "temp", 27.0, &mut errors);
        let tnom = parse_float_field(&self.tnom, "tnom", 27.0, &mut errors);

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(SimulationOptions {
            reltol,
            residual_reltol,
            abstol,
            vntol,
            iabstol,
            chgtol,
            pivrel: 1e-3,
            pivtol: 1e-13,
            itl1,
            itl2: 100,
            itl4,
            gmin_stepping: self.gmin_stepping,
            source_stepping: self.source_stepping,
            pseudo_transient: self.pseudo_transient,
            arc_length: false,
            gmin,
            damping: DampingStrategy::all()[self.damping.min(DampingStrategy::all().len() - 1)],
            method: IntegrationMethod::all()[self.method.min(IntegrationMethod::all().len() - 1)],
            solver: MatrixSolver::all()[self.solver.min(MatrixSolver::all().len() - 1)],
            bypass_enabled: self.bypass_enabled,
            bypass_reltol,
            bypass_abstol,
            min_timestep,
            max_timestep,
            timestep_factor,
            temp,
            tnom,
            verbose: self.verbose,
            save_internals: self.save_internals,
        })
    }
}

fn parse_field(text: &str, name: &str, fallback: f64, errors: &mut Vec<String>) -> f64 {
    parse_si_value(text).unwrap_or_else(|e| {
        errors.push(format!("{}: {}", name, e));
        fallback
    })
}

fn parse_usize_field(text: &str, name: &str, fallback: usize, errors: &mut Vec<String>) -> usize {
    text.parse().unwrap_or_else(|_| {
        errors.push(format!("{}: invalid integer", name));
        fallback
    })
}

fn parse_float_field(text: &str, name: &str, fallback: f64, errors: &mut Vec<String>) -> f64 {
    text.parse().unwrap_or_else(|_| {
        errors.push(format!("{}: invalid number", name));
        fallback
    })
}
