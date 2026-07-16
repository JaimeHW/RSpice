use super::*;

fn is_terminal_end_card(line: &str) -> bool {
    line.split_whitespace()
        .next()
        .is_some_and(|directive| directive.eq_ignore_ascii_case(".end"))
}

impl SimulationController {
    pub(super) fn analysis_spec_to_spice_line(
        &self,
        state: &AppState,
        spec: &AnalysisSpec,
    ) -> Result<String, String> {
        match spec {
            AnalysisSpec::MonteCarlo => self.build_monte_carlo_command(state),
            AnalysisSpec::Parametric => self.build_temperature_step_command(state),
            AnalysisSpec::Corner => self.build_corner_temp_command(state),
            AnalysisSpec::Pss { .. } => self.build_pss_command(state),
            AnalysisSpec::Stb { .. } => self.build_stb_command(state),
            AnalysisSpec::HarmonicBalance { .. } => self.build_harmonic_balance_command(state),
            AnalysisSpec::SParameter { .. } => self.build_sp_command(state),
            AnalysisSpec::Envelope { .. } => self.build_envelope_command(state),
            AnalysisSpec::Fourier { .. } => self.build_fourier_command(state),
            AnalysisSpec::Reliability { .. } => self.build_reliability_command(state),
            AnalysisSpec::Optimization { .. } => self.build_optimization_command(state),
            AnalysisSpec::Soa { .. } => self.build_soa_command(state),
            AnalysisSpec::Disto { .. } => self.build_disto_command(state),
            AnalysisSpec::Pac => self.build_pac_command(state),
            AnalysisSpec::Pnoise => self.build_pnoise_command(state),
            AnalysisSpec::Pxf => self.build_pxf_command(state),
            AnalysisSpec::Pstb => self.build_pstb_command(state),
            AnalysisSpec::Tf => self.build_tf_command(state),
            AnalysisSpec::Qpss { .. }
            | AnalysisSpec::Hbsp { .. }
            | AnalysisSpec::Hbnoise { .. }
            | AnalysisSpec::Psp { .. }
            | AnalysisSpec::Qpac { .. }
            | AnalysisSpec::Qpnoise { .. }
            | AnalysisSpec::Qpxf { .. }
            | AnalysisSpec::TransientNoise { .. }
            | AnalysisSpec::DcMismatch { .. } => Err(format!(
                "{} is configured but cannot produce an engine directive: {}",
                spec.run_type().display_name(),
                manifest_spec_execution_blocker(spec)
            )),
            _ => self
                .analysis_spec_to_config(state, spec)
                .map(|cfg| cfg.to_spice()),
        }
    }

    pub(super) fn build_monte_carlo_command(&self, state: &AppState) -> Result<String, String> {
        let mut mc_state = state.sim_setup.mc.clone();
        mc_state.ensure_initialized();
        let mc_cfg = mc_state
            .to_config()
            .map_err(|e| format!("invalid Monte Carlo settings: {}", e))?;

        let dist_keyword = match mc_cfg.distribution {
            crate::simulation::dialog::mc::McDistribution::Gaussian => "GAUSS",
            crate::simulation::dialog::mc::McDistribution::Uniform => "UNIFORM",
            crate::simulation::dialog::mc::McDistribution::WorstCase => "WORSTCASE",
        };
        let relative_spread = (mc_cfg.variation_pct / 100.0).abs();
        let mut cmd = format!(
            ".mc {} DIST {} SPREAD {:.12e}",
            mc_cfg.num_runs, dist_keyword, relative_spread
        );
        if mc_cfg.seed > 0 {
            cmd.push_str(&format!(" SEED {}", mc_cfg.seed));
        }
        Ok(cmd)
    }

    pub(super) fn build_temperature_step_command(
        &self,
        state: &AppState,
    ) -> Result<String, String> {
        let mut temp_state = state.sim_setup.temp.clone();
        temp_state.ensure_initialized();
        let temp_cfg = temp_state
            .to_config()
            .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;

        if !temp_cfg.specific_temps.is_empty() {
            let values: Vec<String> = temp_cfg
                .specific_temps
                .iter()
                .map(|t| format!("{:.12e}", t))
                .collect();
            Ok(format!(".step temp list {}", values.join(" ")))
        } else {
            Ok(format!(
                ".step temp {:.12e} {:.12e} {:.12e}",
                temp_cfg.temp_start, temp_cfg.temp_stop, temp_cfg.temp_step
            ))
        }
    }

    pub(super) fn build_corner_temp_command(&self, state: &AppState) -> Result<String, String> {
        let mut corner_state = state.sim_setup.corner.clone();
        corner_state.ensure_initialized();
        let corner_cfg = corner_state
            .to_config()
            .map_err(|e| format!("invalid corner settings: {}", e))?;

        if corner_cfg.temperatures.is_empty() {
            return Err("corner analysis requires at least one temperature".to_string());
        }
        let temps: Vec<String> = corner_cfg
            .temperatures
            .iter()
            .map(|temp| format!("{:.12e}", temp))
            .collect();
        Ok(format!(".temp {}", temps.join(" ")))
    }

    pub(super) fn build_pss_command(&self, state: &AppState) -> Result<String, String> {
        let mut pss_state = state.sim_setup.pss.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings: {}", e))?;
        Ok(pss_cfg.to_spice())
    }

    pub(super) fn build_stb_command(&self, state: &AppState) -> Result<String, String> {
        let mut stb_state = state.sim_setup.stb.clone();
        stb_state.ensure_initialized();
        let stb_cfg = stb_state
            .to_config()
            .map_err(|e| format!("invalid STB settings: {}", e))?;
        Ok(stb_cfg.to_spice())
    }

    pub(super) fn build_harmonic_balance_command(
        &self,
        state: &AppState,
    ) -> Result<String, String> {
        let mut hb_state = state.sim_setup.hb.clone();
        hb_state.ensure_initialized();
        let hb_cfg = hb_state
            .to_config()
            .map_err(|e| format!("invalid harmonic balance settings: {}", e))?;
        Ok(hb_cfg.to_spice())
    }

    pub(super) fn build_sp_command(&self, state: &AppState) -> Result<String, String> {
        let mut sp_state = state.sim_setup.sp.clone();
        sp_state.ensure_initialized();
        let sp_cfg = sp_state
            .to_config()
            .map_err(|e| format!("invalid S-parameter settings: {}", e))?;
        Ok(sp_cfg.to_spice())
    }

    pub(super) fn build_envelope_command(&self, state: &AppState) -> Result<String, String> {
        let mut envelope_state = state.sim_setup.envelope.clone();
        envelope_state.ensure_initialized();
        let envelope_cfg = envelope_state
            .to_config()
            .map_err(|e| format!("invalid envelope settings: {}", e))?;
        Ok(envelope_cfg.to_spice())
    }

    pub(super) fn build_fourier_command(&self, state: &AppState) -> Result<String, String> {
        let mut fourier_state = state.sim_setup.fourier.clone();
        fourier_state.ensure_initialized();
        let fourier_cfg = fourier_state
            .to_config()
            .map_err(|e| format!("invalid Fourier settings: {}", e))?;
        Ok(fourier_cfg.to_spice())
    }

    pub(super) fn build_reliability_command(&self, state: &AppState) -> Result<String, String> {
        let mut reliability_state = state.sim_setup.reliability.clone();
        reliability_state.ensure_initialized();
        let reliability_cfg = reliability_state
            .to_config()
            .map_err(|e| format!("invalid reliability settings: {}", e))?;
        Ok(reliability_cfg.to_spice())
    }

    pub(super) fn build_optimization_command(&self, state: &AppState) -> Result<String, String> {
        let mut optimization_state = state.sim_setup.optimization.clone();
        optimization_state.ensure_initialized();
        let optimization_cfg = optimization_state
            .to_config()
            .map_err(|e| format!("invalid optimization settings: {}", e))?;
        Ok(optimization_cfg.to_spice())
    }

    pub(super) fn build_soa_command(&self, state: &AppState) -> Result<String, String> {
        let mut soa_state = state.sim_setup.soa.clone();
        soa_state.ensure_initialized();
        let soa_cfg = soa_state
            .to_config()
            .map_err(|e| format!("invalid SOA settings: {}", e))?;
        Ok(soa_cfg.to_spice())
    }

    pub(super) fn build_pac_command(&self, state: &AppState) -> Result<String, String> {
        let mut pac_state = state.sim_setup.pac.clone();
        pac_state.ensure_initialized();
        let pac_cfg = pac_state
            .to_config()
            .map_err(|e| format!("invalid PAC settings: {}", e))?;
        Ok(pac_cfg.to_spice())
    }

    pub(super) fn build_pnoise_command(&self, state: &AppState) -> Result<String, String> {
        let mut pnoise_state = state.sim_setup.pnoise.clone();
        pnoise_state.ensure_initialized();
        let pnoise_cfg = pnoise_state
            .to_config()
            .map_err(|e| format!("invalid PNOISE settings: {}", e))?;
        Ok(pnoise_cfg.to_spice())
    }

    pub(super) fn build_pxf_command(&self, state: &AppState) -> Result<String, String> {
        let mut pxf_state = state.sim_setup.pxf.clone();
        pxf_state.ensure_initialized();
        let pxf_cfg = pxf_state
            .to_config()
            .map_err(|e| format!("invalid PXF settings: {}", e))?;
        Ok(pxf_cfg.to_spice())
    }

    pub(super) fn build_pstb_command(&self, state: &AppState) -> Result<String, String> {
        let mut pstb_state = state.sim_setup.pstb.clone();
        pstb_state.ensure_initialized();
        let pstb_cfg = pstb_state
            .to_config()
            .map_err(|e| format!("invalid PSTB settings: {}", e))?;
        Ok(pstb_cfg.to_spice())
    }

    pub(super) fn build_tf_command(&self, state: &AppState) -> Result<String, String> {
        let mut xf_state = state.sim_setup.xf.clone();
        xf_state.ensure_initialized();
        let xf_cfg = xf_state
            .to_config()
            .map_err(|e| format!("invalid transfer-function settings: {}", e))?;
        Ok(xf_cfg.to_spice())
    }

    pub(super) fn build_disto_command(&self, state: &AppState) -> Result<String, String> {
        let spec = self.build_disto_spec(state)?;
        if let AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            f2_over_f1,
        } = spec
        {
            let mut command = format!(
                ".disto {} {} {} {}",
                sweep.runner_keyword(),
                points_per_unit,
                start_freq,
                stop_freq
            );
            if let Some(ratio) = f2_over_f1 {
                command.push(' ');
                command.push_str(&ratio.to_string());
            }
            Ok(command)
        } else {
            Err("failed to build DISTO command".to_string())
        }
    }

    /// Inject non-default UI simulation options before `.end`.
    pub(super) fn apply_simulation_options_to_netlist(
        netlist: &str,
        options: &crate::simulation::dialog::SimulationOptions,
    ) -> String {
        let options_block = options.to_spice_options();
        let option_lines: Vec<&str> = options_block.lines().collect();
        if option_lines.len() <= 1 {
            return netlist.to_string();
        }

        let mut lines: Vec<String> = netlist.lines().map(|line| line.to_string()).collect();
        let insertion_idx = lines
            .iter()
            .position(|line| is_terminal_end_card(line))
            .unwrap_or(lines.len());
        let injected_lines = option_lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        lines.splice(insertion_idx..insertion_idx, injected_lines);

        let mut merged = lines.join("\n");
        if netlist.ends_with('\n') {
            merged.push('\n');
        }
        merged
    }

    /// Inject the exact model sources selected for the nominal/reference PVT
    /// point. The marker block lets the corner executor replace this binding
    /// per process without retaining or double-applying the reference models.
    pub(super) fn apply_reference_model_bindings_to_netlist(
        netlist: &str,
        model_cards: &[String],
    ) -> String {
        if model_cards.is_empty() {
            return netlist.to_owned();
        }

        let mut lines: Vec<String> = netlist.lines().map(str::to_owned).collect();
        let insertion_idx = lines
            .iter()
            .position(|line| is_terminal_end_card(line))
            .unwrap_or(lines.len());
        let payload = model_cards
            .iter()
            .flat_map(|cards| cards.lines().map(str::to_owned))
            .collect::<Vec<_>>();
        let mut block = Vec::with_capacity(payload.len() + 2);
        block.push(format!(
            "{} {}",
            crate::services::simulation_runner::REFERENCE_MODEL_BINDING_BEGIN,
            payload.len()
        ));
        block.extend(payload);
        block.push(crate::services::simulation_runner::REFERENCE_MODEL_BINDING_END.to_owned());
        lines.splice(insertion_idx..insertion_idx, block);

        let mut merged = lines.join("\n");
        if netlist.ends_with('\n') {
            merged.push('\n');
        }
        merged
    }
}

fn manifest_spec_execution_blocker(spec: &AnalysisSpec) -> &'static str {
    match spec {
        AnalysisSpec::Qpss { .. } => "the QPSS spectral-lattice solver is unavailable",
        AnalysisSpec::Hbsp { .. } => "HB large-signal network linearization is unavailable",
        AnalysisSpec::Hbnoise { .. } => "harmonic-balance noise execution is unavailable",
        AnalysisSpec::Psp { .. } => "periodic S-parameter execution is unavailable",
        AnalysisSpec::Qpac { .. } => "QPAC conversion-matrix execution is unavailable",
        AnalysisSpec::Qpnoise { .. } => "quasi-periodic noise execution is unavailable",
        AnalysisSpec::Qpxf { .. } => "quasi-periodic transfer execution is unavailable",
        AnalysisSpec::TransientNoise { .. } => "transient device-noise execution is unavailable",
        AnalysisSpec::DcMismatch { .. } => "DC mismatch contribution extraction is unavailable",
        _ => "the selected engine capability is unavailable",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_binding_is_inserted_after_hierarchical_subcircuits() {
        let deck = "hierarchical\n.subckt child in out\nR1 in out 1k\n.ends child\nX1 in out child\n.op\n.end\n";
        let bound = SimulationController::apply_reference_model_bindings_to_netlist(
            deck,
            &[".model sealed D (IS=1e-12)".to_owned()],
        );

        let subckt_end = bound.find(".ends child").expect("subcircuit end remains");
        let binding = bound
            .find(crate::services::simulation_runner::REFERENCE_MODEL_BINDING_BEGIN)
            .expect("binding marker inserted");
        let terminal_end = bound.rfind("\n.end\n").expect("terminal end remains");
        assert!(subckt_end < binding, "{bound}");
        assert!(binding < terminal_end, "{bound}");
    }

    #[test]
    fn terminal_end_matching_accepts_annotations_but_not_longer_directives() {
        assert!(!is_terminal_end_card(".ends child"));
        assert!(!is_terminal_end_card(".endc"));
        assert!(!is_terminal_end_card(".endl TT"));
        assert!(is_terminal_end_card("  .END  "));
        assert!(is_terminal_end_card(".end ; terminal comment"));
        assert!(is_terminal_end_card(".END $ terminal comment"));
    }

    #[test]
    fn options_and_reference_models_precede_annotated_terminal_end_cards() {
        for terminal in [".end ; terminal comment", ".END $ terminal comment"] {
            let deck = format!("annotated terminal\nR1 1 0 1k\n{terminal}\n");
            let with_options = SimulationController::apply_simulation_options_to_netlist(
                &deck,
                &crate::simulation::dialog::SimulationOptions::fast(),
            );
            let option = with_options.find(".OPTIONS").expect("options inserted");
            let end = with_options.find(terminal).expect("terminal retained");
            assert!(option < end, "{with_options}");

            let with_models = SimulationController::apply_reference_model_bindings_to_netlist(
                &deck,
                &[".model sealed D (IS=1e-12)".to_owned()],
            );
            let model = with_models.find(".model sealed").expect("model inserted");
            let end = with_models.find(terminal).expect("terminal retained");
            assert!(model < end, "{with_models}");
        }
    }
}
