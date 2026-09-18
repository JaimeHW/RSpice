//! Turning a configured analysis into a SPICE control line.
//!
//! One place where the typed analysis setup becomes the text the engine
//! parses. Sweeps, Monte Carlo, and corner runs each expand into the command
//! form their analysis expects.

use super::*;

use crate::services::simulation_runner::splice_before_terminal_end_card;

impl SimulationController {
    /// The engine directive one draft emits, against an already-projected
    /// state.
    ///
    /// The three-step dance — preview spec, legacy-index fallback, spice line —
    /// was written out twice: once in the queue builder that produces the
    /// executable deck, once in the ratchet that proves those directives parse.
    /// A surface that wanted to *show* an operator the statement their analysis
    /// emits would have written it a third time, and a displayed statement that
    /// is a re-spelling of the emitted one is worse than showing nothing: it
    /// reads as the deck and is not the deck.
    ///
    /// `state` must already carry `draft` in its legacy singleton slots — the
    /// builders read the projection, not the draft — because projecting here
    /// would mean cloning the whole application state per call. Callers that
    /// price many drafts project once and re-project per draft.
    pub(crate) fn analysis_draft_directive(
        &self,
        state: &AppState,
        draft: &crate::simulation::plan::AnalysisDraft,
    ) -> Result<String, String> {
        let spec = self.analysis_draft_spec(state, draft)?;
        self.analysis_spec_to_spice_line(state, &spec)
    }

    /// The analysis specification one draft resolves to, against an
    /// already-projected state.
    ///
    /// Two of the three steps above: the manifest preview spec, and the
    /// legacy-index fallback for the kinds that have no preview. It is a
    /// function of its own because the queue builder needs the spec as well as
    /// the line — the execution options a task carries are read off the spec —
    /// and so could not simply call [`Self::analysis_draft_directive`]. What it
    /// did instead was write the fallback out a second time, and one derivation
    /// in two places is exactly the drift this pair exists to stop: the queue
    /// and the statement an operator is shown have to resolve the same spec, or
    /// the studio is displaying a deck it will not dispatch.
    pub(super) fn analysis_draft_spec(
        &self,
        state: &AppState,
        draft: &crate::simulation::plan::AnalysisDraft,
    ) -> Result<AnalysisSpec, String> {
        match self.build_manifest_preview_spec(state, draft) {
            Ok(Some(spec)) => Ok(spec),
            Ok(None) => self.build_analysis_spec_for_index(state, draft.kind().legacy_index()),
            Err(error) => Err(error),
        }
    }

    pub(super) fn analysis_spec_to_spice_line(
        &self,
        state: &AppState,
        spec: &AnalysisSpec,
    ) -> Result<String, String> {
        match spec {
            AnalysisSpec::MonteCarlo { .. } => self.build_monte_carlo_command(state),
            AnalysisSpec::Parametric => self.build_temperature_step_command(state),
            AnalysisSpec::Corner => self.build_corner_temp_command(state),
            AnalysisSpec::Pss { .. } => self.build_pss_command(state),
            AnalysisSpec::Stb { .. } => self.build_stb_command(state),
            AnalysisSpec::HarmonicBalance { .. } => self.build_harmonic_balance_command(state),
            AnalysisSpec::SParameter { .. } => self.build_sp_command(state),
            AnalysisSpec::Envelope { .. } => self.build_envelope_command(state),
            AnalysisSpec::Fourier { .. } => self.build_fourier_command(state),
            AnalysisSpec::Optimization { .. } => self.build_optimization_command(state),
            AnalysisSpec::Soa { .. } => self.build_soa_command(state),
            AnalysisSpec::Disto { .. } => self.build_disto_command(state),
            AnalysisSpec::Pac => self.build_pac_command(state),
            AnalysisSpec::Pnoise => self.build_pnoise_command(state),
            AnalysisSpec::Pxf => self.build_pxf_command(state),
            AnalysisSpec::Pstb => self.build_pstb_command(state),
            AnalysisSpec::Psp { .. } => Self::build_psp_command(spec),
            AnalysisSpec::Hbsp { .. } => Self::build_hbsp_command(spec),
            AnalysisSpec::Hbnoise { .. } => Self::build_hbnoise_command(spec),
            AnalysisSpec::Tf { .. } => Self::build_tf_command(spec),
            AnalysisSpec::TransientNoise { .. } => Self::build_transient_noise_command(spec),
            AnalysisSpec::Qpss { .. }
            | AnalysisSpec::Qpac { .. }
            | AnalysisSpec::Qpnoise { .. }
            | AnalysisSpec::Qpxf { .. }
            | AnalysisSpec::DcMismatch { .. }
            | AnalysisSpec::Reliability { .. } => Err(format!(
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

        let mut cmd = format!(".mc {}", mc_cfg.num_runs);
        if mc_cfg.variation_source.uses_stated_spread() {
            let dist_keyword = match mc_cfg.distribution {
                crate::simulation::dialog::mc::McDistribution::Gaussian => "GAUSS",
                crate::simulation::dialog::mc::McDistribution::Uniform => "UNIFORM",
                crate::simulation::dialog::mc::McDistribution::WorstCase => "WORSTCASE",
            };
            let relative_spread = mc_cfg.variation_pct / 100.0;
            cmd.push_str(&format!(" DIST {dist_keyword} SPREAD {relative_spread}"));
        }
        if let Some(seed) = mc_cfg.seed {
            cmd.push_str(&format!(" SEED {seed}"));
        }
        Ok(cmd)
    }

    /// A temperature as a directive spells it: the shortest decimal that reads
    /// back as the same value.
    ///
    /// `{:.12e}` wrote `-4.000000000000e1` for minus forty. This card is
    /// emitted into a deck *and* shown to the reader as the instance's plan
    /// statement, and neither audience gains anything from twelve mantissa
    /// digits on a value the author typed as `-40`. Rust's default `f64`
    /// display is the shortest decimal that round-trips, so the card still
    /// names the exact value the configuration holds.
    fn directive_temperature(value: f64) -> String {
        value.to_string()
    }

    pub(super) fn build_temperature_step_command(
        &self,
        state: &AppState,
    ) -> Result<String, String> {
        let mut temp_state = state.sim_setup.temp.clone();
        temp_state.ensure_initialized();
        let temp_cfg = temp_state
            .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
            .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;

        if !temp_cfg.specific_temps.is_empty() {
            let values: Vec<String> = temp_cfg
                .specific_temps
                .iter()
                .copied()
                .map(Self::directive_temperature)
                .collect();
            Ok(format!(".step temp list {}", values.join(" ")))
        } else {
            Ok(format!(
                ".step temp {} {} {}",
                Self::directive_temperature(temp_cfg.temp_start),
                Self::directive_temperature(temp_cfg.temp_stop),
                Self::directive_temperature(temp_cfg.temp_step),
            ))
        }
    }

    pub(super) fn build_corner_temp_command(&self, state: &AppState) -> Result<String, String> {
        let mut corner_state = state.sim_setup.corner.clone();
        corner_state.ensure_initialized();
        let corner_cfg = corner_state
            .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
            .map_err(|e| format!("invalid corner settings: {}", e))?;

        if corner_cfg.temperatures.is_empty() {
            return Err("corner analysis requires at least one temperature".to_string());
        }
        let temps: Vec<String> = corner_cfg
            .temperatures
            .iter()
            .copied()
            .map(Self::directive_temperature)
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
        // A probe chosen from the drawing has to still be on it. Checked here
        // because this is the one path every surface takes to a directive, so
        // the plan refuses by name instead of writing a deck the engine will
        // reject for a reason that no longer mentions the schematic.
        if let Some(error) = stb_cfg.deleted_probe_error(&state.schematic.placed_loop_probe_names())
        {
            return Err(error);
        }
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
        let placed = crate::simulation::placed_sources::placed_rf_ports(&state.schematic, None);
        let sp_cfg = sp_state
            .to_config(Some(&placed))
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
        // The same check the stability directive makes, at the same seam and
        // for the same reason: this is the one path every surface takes to a
        // directive, so a plan pointing at a probe the drawing no longer holds
        // is refused by name here rather than in the solver.
        if let Some(error) =
            pstb_cfg.deleted_probe_error(&state.schematic.placed_loop_probe_names())
        {
            return Err(error);
        }
        Ok(pstb_cfg.to_spice())
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

    /// The PSP directive: a periodic scattering sweep, in the marker form the
    /// runner reads.
    ///
    /// Takes the specification rather than the session, as the three builders
    /// below it do. These four kinds have no `sim_setup` slot to read — the
    /// draft is projected into a specification and the specification is the
    /// whole input — which is also why the destructure carries an `else` the
    /// way `build_disto_command` does.
    pub(super) fn build_psp_command(spec: &AnalysisSpec) -> Result<String, String> {
        let AnalysisSpec::Psp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            max_sideband,
            ..
        } = spec
        else {
            return Err("failed to build PSP command".to_string());
        };
        Ok(format!(
            "* RSPICE PSP {} {} {:.16e} {:.16e} MAXSIDEBAND={}",
            match sweep {
                FrequencySweep::Decade => "DEC",
                FrequencySweep::Octave => "OCT",
                FrequencySweep::Linear => "LIN",
            },
            points_per_unit,
            start_freq,
            stop_freq,
            max_sideband
        ))
    }

    /// The HBSP directive: the same scattering sweep, about a harmonic-balance
    /// point rather than a shooting one.
    pub(super) fn build_hbsp_command(spec: &AnalysisSpec) -> Result<String, String> {
        let AnalysisSpec::Hbsp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            max_sideband,
            ..
        } = spec
        else {
            return Err("failed to build HBSP command".to_string());
        };
        Ok(format!(
            "* RSPICE HBSP {} {} {:.16e} {:.16e} MAXSIDEBAND={}",
            match sweep {
                FrequencySweep::Decade => "DEC",
                FrequencySweep::Octave => "OCT",
                FrequencySweep::Linear => "LIN",
            },
            points_per_unit,
            start_freq,
            stop_freq,
            max_sideband
        ))
    }

    /// The HBNOISE directive: the noise measured about a harmonic-balance
    /// point, with both ends of the measurement and what is reported of it.
    pub(super) fn build_hbnoise_command(spec: &AnalysisSpec) -> Result<String, String> {
        let AnalysisSpec::Hbnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            output_node,
            output_ref,
            input_source,
            max_sideband,
            integrated_noise,
            contributor_ranking,
            ..
        } = spec
        else {
            return Err("failed to build HBNOISE command".to_string());
        };
        Ok(format!(
            "* RSPICE HBNOISE {} {} {:.16e} {:.16e} OUT={} REF={} IN={} MAXSIDEBAND={} INTEGRATED={} CONTRIBUTORS={}",
            match sweep {
                FrequencySweep::Decade => "DEC",
                FrequencySweep::Octave => "OCT",
                FrequencySweep::Linear => "LIN",
            },
            points_per_unit,
            start_freq,
            stop_freq,
            output_node.trim(),
            output_ref.trim(),
            input_source.trim(),
            max_sideband,
            integrated_noise,
            contributor_ranking
        ))
    }

    /// The `.tran` directive a transient-noise run executes: the window the
    /// ordinary transient card states, then the noise this run injects into
    /// it.
    ///
    /// The window half is written by [`TransientAnalysisConfig::to_spice`]
    /// rather than formatted again here. Two spellings of the same four
    /// positional fields is how a studio ends up dispatching a window it did
    /// not display, and the noise keywords are the only thing this card adds
    /// to the one the Transient kind already writes.
    ///
    /// `NOISESEED=` is always written, even at the draft's default. A run
    /// dispatched from the Studio has to be reproducible by construction: the
    /// seed the form shows is the seed that ran, and a card that left the
    /// engine to resolve one would name a realization the form never stated.
    /// `NOISESCALE=1` is omitted for the same reason the positional start is:
    /// it is the card's own default and says nothing.
    ///
    /// `NOISEFMIN=` is written only when the form authored one, because its
    /// absence is itself a value — the engine derives `1/tstop`, the longest
    /// period the run can resolve — and a card stating that number would
    /// freeze a derivation the window is still allowed to move.
    ///
    /// Visible across `simulation` rather than to the controller alone, which
    /// the rest of this family is. The runner's dispatch test executes the
    /// card this writes instead of a hand-spelled one, because the seed and
    /// the bandwidth reach the solver *only* through this line — the transient
    /// configuration carries the window and nothing else — so a run fixture
    /// that spelled its own card would prove the engine can be asked for noise
    /// while proving nothing about whether the Studio asks for it.
    pub(in crate::simulation) fn build_transient_noise_command(
        spec: &AnalysisSpec,
    ) -> Result<String, String> {
        let AnalysisSpec::TransientNoise {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            seed,
            noise_fmax,
            noise_fmin,
            scale,
            uic,
        } = spec
        else {
            return Err("failed to build transient noise command".to_string());
        };
        let mut command = TransientAnalysisConfig {
            stop_time: *stop_time,
            step_time: *step_time,
            start_time: *start_time,
            max_timestep: Some(*max_timestep),
            uic: *uic,
        }
        .to_spice();
        command.push_str(&format!(" NOISEFMAX={noise_fmax}"));
        if let Some(noise_fmin) = noise_fmin {
            command.push_str(&format!(" NOISEFMIN={noise_fmin}"));
        }
        command.push_str(&format!(" NOISESEED={seed}"));
        if *scale != 1.0 {
            command.push_str(&format!(" NOISESCALE={scale}"));
        }
        Ok(command)
    }

    /// The `.tf` directive: the output expression, then the source it is
    /// measured against, which is the order the card is read in.
    pub(super) fn build_tf_command(spec: &AnalysisSpec) -> Result<String, String> {
        let AnalysisSpec::Tf {
            input_source,
            output_expression,
            ..
        } = spec
        else {
            return Err("failed to build TF command".to_string());
        };
        Ok(format!(
            ".tf {} {}",
            output_expression.trim(),
            input_source.trim()
        ))
    }

    /// Inject non-default UI simulation options before `.end`.
    pub(crate) fn apply_simulation_options_to_netlist(
        netlist: &str,
        options: &crate::simulation::dialog::SimulationOptions,
    ) -> String {
        let options_block = options.to_spice_options();
        // A lone `.OPTIONS` header states nothing, so an all-default option set
        // leaves the deck alone rather than adding an empty card.
        if options_block.lines().count() <= 1 {
            return netlist.to_string();
        }
        splice_before_terminal_end_card(netlist, &options_block)
    }

    /// Inject the exact model sources selected for the nominal/reference PVT
    /// point. The marker block lets the corner executor replace this binding
    /// per process without retaining or double-applying the reference models.
    pub(crate) fn apply_reference_model_bindings_to_netlist(
        netlist: &str,
        model_cards: &[String],
    ) -> String {
        if model_cards.is_empty() {
            return netlist.to_owned();
        }

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
        splice_before_terminal_end_card(netlist, &block.join("\n"))
    }
}

/// Why this specification cannot reach the engine.
///
/// Deliberately delegates to [`AnalysisKind::execution_blocker`] rather than
/// restating the reason. The catalog row, the editor banner, the insert
/// refusal and this dispatch refusal are four places a reader meets the same
/// fact, and they were drifting: the same unavailable solver was described
/// two different ways depending on which guard the reader hit first. Whoever
/// unblocks a kind now edits exactly one string.
fn manifest_spec_execution_blocker(spec: &AnalysisSpec) -> &'static str {
    const UNMAPPED: &str = "the selected engine capability is unavailable";
    let Some(kind) = manifest_spec_kind(spec) else {
        return UNMAPPED;
    };
    kind.execution_blocker().unwrap_or(UNMAPPED)
}

/// The catalog kind a manifest-only specification came from.
///
/// Only the kinds that can be blocked need an answer here; every other
/// specification reaches the engine and never asks.
const fn manifest_spec_kind(spec: &AnalysisSpec) -> Option<crate::simulation::plan::AnalysisKind> {
    use crate::simulation::plan::AnalysisKind;
    Some(match spec {
        AnalysisSpec::Qpss { .. } => AnalysisKind::Qpss,
        AnalysisSpec::Hbsp { .. } => AnalysisKind::Hbsp,
        AnalysisSpec::Hbnoise { .. } => AnalysisKind::Hbnoise,
        AnalysisSpec::Qpac { .. } => AnalysisKind::Qpac,
        AnalysisSpec::Qpnoise { .. } => AnalysisKind::Qpnoise,
        AnalysisSpec::Qpxf { .. } => AnalysisKind::Qpxf,
        AnalysisSpec::DcMismatch { .. } => AnalysisKind::DcMismatch,
        AnalysisSpec::Reliability { .. } => AnalysisKind::Reliability,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monte_carlo_seed_draft_reaches_the_engine_at_full_width_including_zero() {
        for seed in [
            0_u64,
            (1_u64 << 32) + 1,
            (1_u64 << 53) + 1,
            u64::MAX - 1,
            u64::MAX,
        ] {
            let mut state = AppState::default();
            state.sim_setup.mc.ensure_initialized();
            state.sim_setup.mc.seed = seed.to_string();
            let command = SimulationController::default()
                .build_monte_carlo_command(&state)
                .expect("all u64 seeds are authorable");
            let netlist =
                rspice_core::netlist::Netlist::parse(&format!("MC seed\n{command}\n.end\n"))
                    .unwrap();
            let rspice_core::netlist::AnalysisCommand::MonteCarlo(mc) = &netlist.analyses[0] else {
                panic!("missing MC command")
            };
            assert_eq!(mc.seed, Some(seed), "{command}");
        }
    }

    #[test]
    fn monte_carlo_spread_preserves_the_computed_engine_value() {
        let mut state = AppState::default();
        state.sim_setup.mc.ensure_initialized();
        state.sim_setup.mc.variation_pct = "1.234567891234567".to_owned();
        let command = SimulationController::default()
            .build_monte_carlo_command(&state)
            .unwrap();
        let netlist =
            rspice_core::netlist::Netlist::parse(&format!("MC spread\n{command}\n.end\n")).unwrap();
        let rspice_core::netlist::AnalysisCommand::MonteCarlo(mc) = &netlist.analyses[0] else {
            panic!("missing MC command")
        };
        assert_eq!(
            mc.relative_spread.to_bits(),
            (1.234567891234567_f64 / 100.0).to_bits(),
            "{command}"
        );
    }

    #[test]
    fn monte_carlo_deck_statistics_preserve_inactive_buffers_without_emitting_them() {
        let mut state = AppState::default();
        state.sim_setup.mc.ensure_initialized();
        state.sim_setup.mc.variation_source_idx = 1;
        state.sim_setup.mc.variation_pct = "unfinished(".to_owned();
        state.sim_setup.mc.distribution_idx = usize::MAX;
        let command = SimulationController::default()
            .build_monte_carlo_command(&state)
            .unwrap();
        assert!(!command.contains("DIST"), "{command}");
        assert!(!command.contains("SPREAD"), "{command}");
        assert!(!command.contains("SEED"), "{command}");
        assert_eq!(state.sim_setup.mc.variation_pct, "unfinished(");
        assert_eq!(state.sim_setup.mc.distribution_idx, usize::MAX);
        state.sim_setup.mc.variation_source_idx = 0;
        assert!(
            SimulationController::default()
                .build_monte_carlo_command(&state)
                .is_err()
        );
    }

    /// The directive builder is the one path every surface takes, so it is
    /// where a stale probe reference has to be caught. Checking only in the
    /// form would leave a plan built before the probe was deleted still able
    /// to write a deck naming it.
    #[test]
    fn a_plan_referencing_a_deleted_loop_probe_is_refused_by_name() {
        use crate::simulation::dialog::StbProbeReference;
        use crate::state::{Component, ComponentType, Point};

        let mut state = AppState::default();
        state.sim_setup.stb.ensure_initialized();
        state.sim_setup.stb.probe_source = "VLOOP1".to_owned();
        state.sim_setup.stb.probe_reference = StbProbeReference::Placed;
        // Another probe is drawn, so the remedy has something to offer.
        state.schematic.components.push(
            Component::new(1, ComponentType::LoopProbe, Point::new(0, 0))
                .with_name_value("VLOOP2", ""),
        );

        let error = SimulationController::new()
            .build_stb_command(&state)
            .expect_err("a probe that is not on the schematic is refused");

        assert!(error.contains("VLOOP1"), "{error}");
        assert!(error.contains("VLOOP2"), "{error}");
    }

    /// And the same builder writes the card when the probe is still drawn.
    #[test]
    fn a_plan_referencing_a_placed_loop_probe_writes_its_directive() {
        use crate::simulation::dialog::StbProbeReference;
        use crate::state::{Component, ComponentType, Point};

        let mut state = AppState::default();
        state.sim_setup.stb.ensure_initialized();
        state.sim_setup.stb.probe_source = "VLOOP1".to_owned();
        state.sim_setup.stb.probe_reference = StbProbeReference::Placed;
        state.schematic.components.push(
            Component::new(1, ComponentType::LoopProbe, Point::new(0, 0))
                .with_name_value("VLOOP1", ""),
        );

        let directive = SimulationController::new()
            .build_stb_command(&state)
            .expect("a placed probe reaches the deck");

        assert!(directive.contains("probe=VLOOP1"), "{directive}");
    }

    /// PSTB designates the same element, so it is refused the same way.
    ///
    /// Its field was free text until the picker reached it, which meant a
    /// periodic stability plan could name a probe the drawing had never held
    /// and only find out in the solver. Both directions are checked here
    /// because a refusal that fires unconditionally is as wrong as one that
    /// never fires: a name entered by hand is a claim about someone else's
    /// deck and must still reach the card.
    #[test]
    fn a_periodic_stability_plan_is_refused_by_the_same_probe_name() {
        use crate::simulation::dialog::StbProbeReference;
        use crate::state::{Component, ComponentType, Point};

        let mut state = AppState::default();
        state.sim_setup.pstb.ensure_initialized();
        state.sim_setup.pstb.probe = "VLOOP1".to_owned();
        state.sim_setup.pstb.probe_reference = StbProbeReference::Placed;
        state.schematic.components.push(
            Component::new(1, ComponentType::LoopProbe, Point::new(0, 0))
                .with_name_value("VLOOP2", ""),
        );

        let error = SimulationController::new()
            .build_pstb_command(&state)
            .expect_err("a probe that is not on the schematic is refused");
        assert!(error.contains("VLOOP1"), "{error}");
        assert!(error.contains("VLOOP2"), "{error}");

        state.sim_setup.pstb.probe_reference = StbProbeReference::Entered;
        let directive = SimulationController::new()
            .build_pstb_command(&state)
            .expect("a name entered by hand is the deck's claim, not this design's");
        assert!(directive.contains("probe=VLOOP1"), "{directive}");

        state.sim_setup.pstb.probe = "VLOOP2".to_owned();
        state.sim_setup.pstb.probe_reference = StbProbeReference::Placed;
        let directive = SimulationController::new()
            .build_pstb_command(&state)
            .expect("a placed probe reaches the deck");
        assert!(directive.contains("probe=VLOOP2"), "{directive}");
    }

    /// The reader meets this fact in four places — catalog disposition, the
    /// editor banner, the insert refusal, and this dispatch refusal. They
    /// must all be quoting the same sentence.
    #[test]
    fn dispatch_refusal_quotes_the_catalog_blocker_verbatim() {
        use crate::simulation::plan::AnalysisKind;
        for kind in AnalysisKind::ALL {
            let Some(expected) = kind.execution_blocker() else {
                continue;
            };
            let draft = crate::simulation::plan::AnalysisDraft::for_kind(kind);
            let spec = SimulationController::new()
                .build_manifest_preview_spec(&AppState::default(), &draft)
                .expect("blocked kinds still build a transportable specification")
                .expect("blocked kinds are manifest kinds and have a typed specification");

            assert_eq!(
                manifest_spec_execution_blocker(&spec),
                expected,
                "{} states its blocker differently at dispatch than in the catalog",
                kind.label()
            );
        }
    }

    /// Every blocked kind must be reachable from its specification, or the
    /// refusal quietly degrades to the generic sentence.
    #[test]
    fn every_blocked_kind_is_recoverable_from_its_specification() {
        use crate::simulation::plan::AnalysisKind;
        for kind in AnalysisKind::ALL {
            if kind.execution_blocker().is_none() {
                continue;
            }
            let draft = crate::simulation::plan::AnalysisDraft::for_kind(kind);
            let spec = SimulationController::new()
                .build_manifest_preview_spec(&AppState::default(), &draft)
                .expect("specification builds")
                .expect("blocked kinds have a typed specification");

            assert_eq!(
                manifest_spec_kind(&spec),
                Some(kind),
                "{} has no specification-to-kind mapping",
                kind.label()
            );
        }
    }

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
    fn generated_cards_preserve_end_titles_and_reach_the_parser() {
        for title in [".end", "ordinary title"] {
            for terminal in [".end; done", ".END // done", ".end $ done", ".end"] {
                let deck = format!("{title}\r\nR1 1 0 1k\r\n{terminal}\r\n");
                let options = crate::simulation::dialog::SimulationOptions {
                    reltol: 0.012345,
                    ..Default::default()
                };
                let configured =
                    SimulationController::apply_simulation_options_to_netlist(&deck, &options);
                let bound = SimulationController::apply_reference_model_bindings_to_netlist(
                    &configured,
                    &[".model sealed D (IS=1e-12)".to_owned()],
                );
                let parsed = rspice_core::Netlist::parse(&bound).expect("composed deck parses");
                assert_eq!(parsed.title, title, "{bound}");
                assert_eq!(parsed.options.reltol, Some(0.012345), "{bound}");
                assert!(
                    parsed
                        .models
                        .iter()
                        .any(|model| model.name.eq_ignore_ascii_case("sealed")),
                    "{bound}"
                );
                assert!(
                    bound.starts_with(&format!("{title}\r\nR1 1 0 1k\r\n")),
                    "{bound:?}"
                );
                assert!(bound.ends_with(&format!("{terminal}\r\n")), "{bound:?}");
            }
        }
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
    fn psp_spec() -> AnalysisSpec {
        AnalysisSpec::Psp {
            start_freq: 1.0e6,
            stop_freq: 1.0e9,
            points_per_unit: 11,
            sweep: FrequencySweep::Decade,
            ports: Vec::new(),
            max_sideband: 3,
            mixed_mode: false,
            noise_parameters: false,
        }
    }

    fn hbsp_spec() -> AnalysisSpec {
        AnalysisSpec::Hbsp {
            start_freq: 1.0e3,
            stop_freq: 1.0e5,
            points_per_unit: 7,
            sweep: FrequencySweep::Octave,
            ports: Vec::new(),
            max_sideband: 2,
            mixed_mode: true,
            noise_parameters: true,
        }
    }

    /// Every string field arrives with surrounding space, because the card
    /// trims each one and a fixture that came in clean would not say so.
    fn hbnoise_spec() -> AnalysisSpec {
        AnalysisSpec::Hbnoise {
            start_freq: 1.0e1,
            stop_freq: 1.0e4,
            points_per_unit: 21,
            sweep: FrequencySweep::Linear,
            output_node: " out ".to_owned(),
            output_ref: " 0 ".to_owned(),
            input_source: " vin ".to_owned(),
            max_sideband: 4,
            integrated_noise: true,
            noise_figure: false,
            contributor_ranking: true,
        }
    }

    fn transient_noise_spec() -> AnalysisSpec {
        AnalysisSpec::TransientNoise {
            stop_time: 1.0e-6,
            step_time: 1.0e-9,
            start_time: 2.0e-7,
            max_timestep: 2.5e-10,
            seed: 97,
            noise_fmax: 5.0e8,
            noise_fmin: None,
            scale: 0.5,
            uic: true,
        }
    }

    /// The card the Studio writes is the card the engine reads, field for
    /// field.
    ///
    /// Not a string assertion. The engine's own parser is the only reader of
    /// this line, and a keyword it does not know, a positional field in the
    /// wrong slot, or an exponent it truncates would all pass an expected-text
    /// comparison while producing a run configured by something the deck never
    /// said. So the emitted card is read back through `rspice-core` and every
    /// value is recovered from the `AnalysisCommand::Tran` window and the
    /// `TransientNoiseConfig` the deck's options carry.
    #[test]
    fn a_transient_noise_spec_writes_the_card_the_engine_parses() {
        use rspice_core::netlist::AnalysisCommand;

        for (scale, uic, noise_fmin) in [
            (0.5, true, None),
            (1.0, false, None),
            (1.0, false, Some(1.0e3)),
        ] {
            let AnalysisSpec::TransientNoise {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                seed,
                noise_fmax,
                ..
            } = transient_noise_spec()
            else {
                unreachable!("the fixture is a transient-noise specification");
            };
            let spec = AnalysisSpec::TransientNoise {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                seed,
                noise_fmax,
                noise_fmin,
                scale,
                uic,
            };
            let card = SimulationController::build_transient_noise_command(&spec)
                .expect("a transient-noise specification writes its own card");
            // Pinned as well as read back: the spelling is what a colleague
            // handed this deck reads, and a reader of this test should not
            // have to run a parser to see it.
            assert_eq!(
                card,
                match (uic, noise_fmin) {
                    (true, None) =>
                        ".tran 0.000000001 0.000001 0.0000002 0.00000000025 UIC \
                         NOISEFMAX=500000000 NOISESEED=97 NOISESCALE=0.5",
                    (false, None) =>
                        ".tran 0.000000001 0.000001 0.0000002 0.00000000025 \
                         NOISEFMAX=500000000 NOISESEED=97",
                    _ =>
                        ".tran 0.000000001 0.000001 0.0000002 0.00000000025 \
                         NOISEFMAX=500000000 NOISEFMIN=1000 NOISESEED=97",
                }
            );

            // A scale of exactly one is the card's default and is not written;
            // anything else has to reach the line or the run is quieter than
            // the form that configured it.
            assert_eq!(
                card.contains("NOISESCALE"),
                scale != 1.0,
                "{card} states NOISESCALE against a scale of {scale}"
            );
            // And an unauthored floor stays unwritten, because writing the
            // engine's derivation would freeze it.
            assert_eq!(
                card.contains("NOISEFMIN"),
                noise_fmin.is_some(),
                "{card} states NOISEFMIN against a floor of {noise_fmin:?}"
            );

            let deck = rspice_core::netlist::Netlist::parse(&format!(
                "transient noise card\nV1 in 0 SIN(0 1 1k)\nR1 in 0 1k\n{card}\n.end\n"
            ))
            .unwrap_or_else(|error| panic!("the engine must read `{card}` back: {error}"));

            let [
                AnalysisCommand::Tran {
                    step,
                    stop,
                    start,
                    max_step,
                    uic: parsed_uic,
                },
            ] = deck.analyses.as_slice()
            else {
                panic!("the card is one transient window: {:?}", deck.analyses);
            };
            assert_eq!(*step, step_time, "{card}");
            assert_eq!(*stop, stop_time, "{card}");
            assert_eq!(start.unwrap_or(0.0), start_time, "{card}");
            assert_eq!(*max_step, Some(max_timestep), "{card}");
            assert_eq!(*parsed_uic, uic, "{card}");

            let noise = deck
                .options
                .transient_noise
                .unwrap_or_else(|| panic!("the card turns transient noise on: {card}"));
            assert_eq!(noise.fmax, noise_fmax, "{card}");
            assert_eq!(noise.fmin, noise_fmin, "{card}");
            assert_eq!(noise.seed, Some(seed), "{card}");
            assert_eq!(noise.scale, scale, "{card}");
        }
    }

    fn tf_spec() -> AnalysisSpec {
        AnalysisSpec::Tf {
            input_source: " vin ".to_owned(),
            output_expression: " V(out) ".to_owned(),
            transfer_gain: true,
            input_resistance: false,
            output_resistance: false,
            normalization: crate::simulation::multi_run::TfNormalization::default(),
            accuracy: crate::simulation::multi_run::TfAccuracy::default(),
        }
    }

    /// The four directives the dispatch match used to format inline.
    ///
    /// Pinned rather than derived: these cards have no reader in this crate,
    /// so a builder that changed its sweep keyword, its exponent width or its
    /// trimming would emit a deck the runner reads differently and nothing
    /// else here would notice.
    #[test]
    fn every_statement_builder_emits_what_the_inline_arm_did() {
        assert_eq!(
            SimulationController::build_psp_command(&psp_spec()),
            Ok(
                "* RSPICE PSP DEC 11 1.0000000000000000e6 1.0000000000000000e9 MAXSIDEBAND=3"
                    .to_owned()
            )
        );
        assert_eq!(
            SimulationController::build_hbsp_command(&hbsp_spec()),
            Ok(
                "* RSPICE HBSP OCT 7 1.0000000000000000e3 1.0000000000000000e5 MAXSIDEBAND=2"
                    .to_owned()
            )
        );
        assert_eq!(
            SimulationController::build_hbnoise_command(&hbnoise_spec()),
            Ok(
                "* RSPICE HBNOISE LIN 21 1.0000000000000000e1 1.0000000000000000e4 OUT=out REF=0 \
                IN=vin MAXSIDEBAND=4 INTEGRATED=true CONTRIBUTORS=true"
                    .to_owned()
            )
        );
        assert_eq!(
            SimulationController::build_tf_command(&tf_spec()),
            Ok(".tf V(out) vin".to_owned())
        );
    }

    /// A builder handed another kind's specification refuses by name.
    ///
    /// The destructure's `else` is not decoration: these four are routed by a
    /// `..` pattern, so a variant added beside them could reach the wrong
    /// builder, and the answer has to be a refusal naming the builder rather
    /// than a panic in a running studio.
    #[test]
    fn a_statement_builder_handed_another_kind_refuses_by_name() {
        assert_eq!(
            SimulationController::build_psp_command(&hbsp_spec()),
            Err("failed to build PSP command".to_owned())
        );
        assert_eq!(
            SimulationController::build_hbsp_command(&psp_spec()),
            Err("failed to build HBSP command".to_owned())
        );
        assert_eq!(
            SimulationController::build_hbnoise_command(&tf_spec()),
            Err("failed to build HBNOISE command".to_owned())
        );
        assert_eq!(
            SimulationController::build_tf_command(&hbnoise_spec()),
            Err("failed to build TF command".to_owned())
        );
        assert_eq!(
            SimulationController::build_transient_noise_command(&tf_spec()),
            Err("failed to build transient noise command".to_owned())
        );
    }
}
