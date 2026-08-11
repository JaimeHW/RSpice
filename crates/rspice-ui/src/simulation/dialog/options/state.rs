//! Options dialog state.

use super::{
    DampingStrategy, IntegrationMethod, MatrixSolver, SimulationOptions, format_si_value,
    parse_si_value,
};

/// UI state for options dialog (string buffers for text editing).
///
/// Every field of [`SimulationOptions`] that a surface can edit has a buffer
/// here. A field that had no buffer was silently reset to a literal on the
/// next commit, so a value the user set never survived a round trip — the
/// matrix pivot thresholds, the DC-sweep iteration budget, and arc-length
/// continuation all behaved that way before they were given buffers.
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
    pub trtol: String,
    pub transient_lte_reltol: String,
    pub transient_lte_abstol: String,
    pub statistical_seed: String,
    pub gmin: String,
    pub pivrel: String,
    pub pivtol: String,
    pub gmin_stepping: bool,
    pub source_stepping: bool,
    pub pseudo_transient: bool,
    pub arc_length: bool,
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
            trtol: format_si_value(opts.trtol),
            // An empty buffer is the engine's own bound, not a value of zero.
            transient_lte_reltol: opts
                .transient_lte_reltol
                .map(format_si_value)
                .unwrap_or_default(),
            transient_lte_abstol: opts
                .transient_lte_abstol
                .map(format_si_value)
                .unwrap_or_default(),
            statistical_seed: opts
                .statistical_seed
                .map(|seed| seed.to_string())
                .unwrap_or_default(),
            gmin: format_si_value(opts.gmin),
            pivrel: format_si_value(opts.pivrel),
            pivtol: format_si_value(opts.pivtol),
            gmin_stepping: opts.gmin_stepping,
            source_stepping: opts.source_stepping,
            pseudo_transient: opts.pseudo_transient,
            arc_length: opts.arc_length,
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
        let pivrel = parse_field(&self.pivrel, "pivrel", 1e-3, &mut errors);
        let pivtol = parse_field(&self.pivtol, "pivtol", 1e-13, &mut errors);

        let itl1 = parse_usize_field(&self.itl1, "itl1", 50, &mut errors);
        let itl4 = parse_usize_field(&self.itl4, "itl4", 6, &mut errors);
        let trtol = parse_field(&self.trtol, "trtol", 7.0, &mut errors);
        let transient_lte_reltol =
            parse_optional_field(&self.transient_lte_reltol, "lte_reltol", &mut errors);
        let transient_lte_abstol =
            parse_optional_field(&self.transient_lte_abstol, "lte_abstol", &mut errors);
        let statistical_seed = if self.statistical_seed.trim().is_empty() {
            None
        } else {
            match self.statistical_seed.trim().parse::<u64>() {
                Ok(seed) => Some(seed),
                Err(_) => {
                    errors.push("statistical_seed: invalid integer".to_owned());
                    None
                }
            }
        };
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
            pivrel,
            pivtol,
            itl1,
            itl4,
            trtol,
            transient_lte_reltol,
            transient_lte_abstol,
            statistical_seed,
            gmin_stepping: self.gmin_stepping,
            source_stepping: self.source_stepping,
            pseudo_transient: self.pseudo_transient,
            arc_length: self.arc_length,
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

/// Parse a field whose empty value means "leave the engine's own bound".
///
/// Blank is a legitimate answer here, distinct from zero: a bound of zero
/// would reject every step, so treating an empty buffer as zero would turn a
/// field the reader left alone into one that stops the solve.
fn parse_optional_field(text: &str, name: &str, errors: &mut Vec<String>) -> Option<f64> {
    if text.trim().is_empty() {
        return None;
    }
    match parse_si_value(text) {
        Ok(value) => Some(value),
        Err(error) => {
            errors.push(format!("{}: {}", name, error));
            None
        }
    }
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
