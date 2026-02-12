use std::collections::HashSet;

use super::RSpiceApp;

pub(super) fn set_default_if_blank(field: &mut String, default: &str) {
    if field.trim().is_empty() {
        field.clear();
        field.push_str(default);
    }
}

pub(super) fn toggle_enabled_analysis(enabled_analyses: &mut HashSet<usize>, index: usize) {
    if !enabled_analyses.insert(index) {
        enabled_analyses.remove(&index);
    }
}

impl RSpiceApp {
    pub(super) fn ensure_simulation_setup_defaults(&mut self) {
        // Transient
        set_default_if_blank(&mut self.state.dialogs.tran_stop, "1m");
        set_default_if_blank(&mut self.state.dialogs.tran_step, "10n");
        set_default_if_blank(&mut self.state.dialogs.tran_start, "0");
        set_default_if_blank(&mut self.state.dialogs.tran_maxstep, "auto");
        // AC
        set_default_if_blank(&mut self.state.dialogs.ac_fstart, "1");
        set_default_if_blank(&mut self.state.dialogs.ac_fstop, "1G");
        set_default_if_blank(&mut self.state.dialogs.ac_points, "101");
        set_default_if_blank(&mut self.state.dialogs.disto_f2_over_f1, "2.0");
        // DC
        set_default_if_blank(&mut self.state.dialogs.dc_source, "V1");
        set_default_if_blank(&mut self.state.dialogs.dc_start, "0");
        set_default_if_blank(&mut self.state.dialogs.dc_stop, "5");
        set_default_if_blank(&mut self.state.dialogs.dc_step, "0.01");
        set_default_if_blank(&mut self.state.dialogs.dc_source2, "V2");
        set_default_if_blank(&mut self.state.dialogs.dc_start2, "0");
        set_default_if_blank(&mut self.state.dialogs.dc_stop2, "3.3");
        set_default_if_blank(&mut self.state.dialogs.dc_step2, "0.1");
        // Noise
        set_default_if_blank(&mut self.state.dialogs.noise_output, "out");
        set_default_if_blank(&mut self.state.dialogs.noise_ref, "0");
        set_default_if_blank(&mut self.state.dialogs.noise_input, "V1");
        set_default_if_blank(&mut self.state.dialogs.noise_fstart, "1");
        set_default_if_blank(&mut self.state.dialogs.noise_fstop, "100Meg");
        // Pole-Zero
        set_default_if_blank(&mut self.state.dialogs.pz_input, "in");
        set_default_if_blank(&mut self.state.dialogs.pz_output, "out");
        // Sensitivity
        set_default_if_blank(&mut self.state.dialogs.sens_output, "V(out)");
        // Monte Carlo
        set_default_if_blank(&mut self.state.dialogs.mc_runs, "100");
        set_default_if_blank(&mut self.state.dialogs.mc_seed, "0");
        // PSS
        set_default_if_blank(&mut self.state.dialogs.pss_fund, "1Meg");
        set_default_if_blank(&mut self.state.dialogs.pss_harmonics, "10");
        set_default_if_blank(&mut self.state.dialogs.pss_maxiter, "100");
        // STB
        set_default_if_blank(&mut self.state.dialogs.stb_probe, "istb");
        set_default_if_blank(&mut self.state.dialogs.stb_fstart, "1");
        set_default_if_blank(&mut self.state.dialogs.stb_fstop, "100Meg");
        // Temperature
        set_default_if_blank(&mut self.state.dialogs.temp_start, "-40");
        set_default_if_blank(&mut self.state.dialogs.temp_stop, "125");
        set_default_if_blank(&mut self.state.dialogs.temp_step, "25");
    }
}
