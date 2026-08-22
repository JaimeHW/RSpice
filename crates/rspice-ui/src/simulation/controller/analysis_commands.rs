//! Turning a configured analysis into a SPICE control line.
//!
//! One place where the typed analysis setup becomes the text the engine
//! parses. Sweeps, Monte Carlo, and corner runs each expand into the command
//! form their analysis expects.

use super::*;

fn is_terminal_end_card(line: &str) -> bool {
    line.split_whitespace()
        .next()
        .is_some_and(|directive| directive.eq_ignore_ascii_case(".end"))
}

/// Insert a block of cards immediately before the deck's terminal `.end`.
///
/// Anything after `.end` is not part of the circuit, so a card appended to the
/// file would parse as nothing at all. A deck without a terminal `.end` takes
/// the block at its foot. The trailing newline of the original is preserved so
/// splicing twice does not change how the deck ends.
pub(in crate::simulation) fn splice_before_terminal_end_card(netlist: &str, block: &str) -> String {
    if block.is_empty() {
        return netlist.to_owned();
    }
    let mut lines: Vec<String> = netlist.lines().map(str::to_owned).collect();
    let insertion_idx = lines
        .iter()
        .position(|line| is_terminal_end_card(line))
        .unwrap_or(lines.len());
    lines.splice(
        insertion_idx..insertion_idx,
        block.lines().map(str::to_owned),
    );

    let mut merged = lines.join("\n");
    if netlist.ends_with('\n') {
        merged.push('\n');
    }
    merged
}

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
        let spec = match self.build_manifest_preview_spec(state, draft) {
            Ok(Some(spec)) => spec,
            Ok(None) => self.build_analysis_spec_for_index(state, draft.kind().legacy_index())?,
            Err(error) => return Err(error),
        };
        self.analysis_spec_to_spice_line(state, &spec)
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
            AnalysisSpec::Psp {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                max_sideband,
                ..
            } => Ok(format!(
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
            )),
            AnalysisSpec::Hbsp {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                max_sideband,
                ..
            } => Ok(format!(
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
            )),
            AnalysisSpec::Hbnoise {
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
            } => Ok(format!(
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
            )),
            AnalysisSpec::Tf {
                input_source,
                output_expression,
                ..
            } => Ok(format!(
                ".tf {} {}",
                output_expression.trim(),
                input_source.trim()
            )),
            AnalysisSpec::Qpss { .. }
            | AnalysisSpec::Qpac { .. }
            | AnalysisSpec::Qpnoise { .. }
            | AnalysisSpec::Qpxf { .. }
            | AnalysisSpec::TransientNoise { .. }
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
        AnalysisSpec::TransientNoise { .. } => AnalysisKind::TransientNoise,
        AnalysisSpec::DcMismatch { .. } => AnalysisKind::DcMismatch,
        AnalysisSpec::Reliability { .. } => AnalysisKind::Reliability,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
