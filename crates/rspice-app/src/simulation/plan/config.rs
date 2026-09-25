//! Application adapters for portable analysis drafts.
//!
//! The tagged draft and its portable policy live in the simulation contract.
//! This module retains circuit-derived prerequisite repair and integration
//! checks against the application.

#[cfg(test)]
use crate::simulation::config::{
    AcSweepType, NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
};
use crate::simulation::dialog::PssDialogState;
use rspice_simulation_contract::drafts::TranSetup;
pub(super) use rspice_simulation_contract::plan_dependency::{
    DependencyConfigurationIssue, dependency_configuration_issue, fourier_requirement,
    periodic_state_requirement,
};

use super::AnalysisKind;

mod periodic_network;
mod qpnoise;
pub use qpnoise::{
    QpnoiseLatticeSelection, QpnoiseOutputDraft, QpnoiseSourceSelection, QuasiPeriodicNoiseDraft,
};
#[cfg(test)]
use rspice_simulation_contract::quasi_periodic_draft::validate_qpss;
pub use rspice_simulation_contract::quasi_periodic_draft::{
    QpssDraft, QpxfSidebandSelection, QpxfSourceSelection, QuasiPeriodicAcDraft,
    QuasiPeriodicTransferDraft,
};

pub use periodic_network::PeriodicNetworkDraft;
pub use rspice_simulation_contract::drafts::DistoDraft;
pub(crate) use rspice_simulation_contract::drafts::dc_mismatch_share_threshold;
pub use rspice_simulation_contract::drafts::{
    AcDataDraft, DcMismatchDraft, FftDraft, FrequencySweepDraft, NoiseDraft, TransientNoiseDraft,
};
#[cfg(test)]
use rspice_simulation_contract::drafts::{validate_dc_mismatch, validate_transient_noise};
pub use rspice_simulation_contract::hbnoise_draft::HbNoiseDraft;
pub use rspice_simulation_contract::periodic_network_draft::NetworkPortDraft;
#[cfg(test)]
use rspice_simulation_contract::periodic_network_draft::{
    validate_periodic_network, validate_psp_network,
};

pub use rspice_simulation_contract::analysis_draft::AnalysisDraft;

/// Circuit-derived inputs required to synthesize or reuse prerequisites whose
/// configuration depends on elaborated source identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisDependencyRepairContext {
    periodic_sources: Result<ExactPeriodicSourceContract, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExactPeriodicSourceContract {
    names: Vec<String>,
    netlist_source: String,
}

impl AnalysisDependencyRepairContext {
    /// Context for callers that cannot authenticate the circuit source set.
    #[must_use]
    pub fn periodic_sources_unavailable(detail: impl Into<String>) -> Self {
        let detail = detail.into();
        let detail = detail.trim();
        Self {
            periodic_sources: Err(if detail.is_empty() {
                "the exact elaborated periodic-source catalog is unavailable".to_owned()
            } else {
                detail.to_owned()
            }),
        }
    }

    /// Authenticate and retain the exact elaborated source contract used by
    /// PSS. Keeping the executable source with the discovered identities lets
    /// repair validate waveform periodicity and commensurability against the
    /// candidate fundamental before committing a lifecycle transaction.
    ///
    /// An empty, successfully elaborated catalog is authoritative: it can
    /// validate an autonomous PSS with no driven tones. It must remain
    /// distinct from an unavailable catalog, because only the latter prevents
    /// safe reuse of an existing oscillator prerequisite.
    pub fn exact_periodic_sources(netlist_source: impl Into<String>) -> Result<Self, String> {
        let netlist_source = netlist_source.into();
        let netlist = rspice_core::Netlist::parse(&netlist_source)
            .map_err(|error| format!("the periodic-source circuit could not be parsed: {error}"))?;
        let names = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .transient_source_names(&netlist)
            .map_err(|error| {
                format!("the periodic-source circuit could not be elaborated: {error}")
            })?;
        Ok(Self {
            periodic_sources: Ok(ExactPeriodicSourceContract {
                names,
                netlist_source,
            }),
        })
    }

    fn periodic_sources(&self) -> Result<&[String], String> {
        self.periodic_sources
            .as_ref()
            .map(|contract| contract.names.as_slice())
            .map_err(|detail| detail.clone())
    }

    pub(super) fn availability_error(&self) -> Option<&str> {
        self.periodic_sources.as_ref().err().map(String::as_str)
    }

    /// Whether this PSS draft's source selection can execute.
    ///
    /// The closed periodic-source contract is a *driven* rule and is asked only
    /// of a driven solve. An autonomous run has no tone list to close over: the
    /// engine's `PssConfig` carries no tone field at all
    /// (`rspice-core/src/analysis/pss/config.rs`), the period comes from the
    /// oscillator node, and `validate_periodic_source_contract` says in its own
    /// first line that it defines "a driven PSS period"
    /// (`rspice-core/src/engine/transient.rs:1349-1352`).
    ///
    /// Asking it anyway made autonomous PSS unsatisfiable on any circuit that
    /// places a transient source, which is every oscillator with a startup
    /// kick: the driven contract refuses an omitted source, and the autonomous
    /// mode refuses a tone list, so neither an empty selection nor a full one
    /// could pass. What the engine actually does with those sources is stated
    /// on the form rather than refused here.
    pub(crate) fn validate_pss_sources(&self, draft: &PssDialogState) -> Result<(), String> {
        let config = draft
            .to_config()
            .map_err(|detail| format!("PSS configuration is invalid: {detail}"))?;
        if config.osc_mode {
            return Ok(());
        }
        self.validate_periodic_source_selection(&config.tone_sources)?;
        let contract = self
            .periodic_sources
            .as_ref()
            .map_err(|detail| detail.clone())?;
        let netlist = rspice_core::Netlist::parse(&contract.netlist_source).map_err(|error| {
            format!("the authenticated periodic-source circuit is no longer parseable: {error}")
        })?;
        rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .validate_pss_source_contract_with_abort(
                &netlist,
                &config.tone_sources,
                &rspice_core::analysis::PssConfig::new(config.fund_freq)
                    .with_points_per_period(config.points_per_period)
                    .with_harmonics(config.num_harmonics.max(1)),
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| format!("PSS periodic-source contract is invalid: {error}"))
    }

    fn validate_periodic_source_selection(&self, selected: &[String]) -> Result<(), String> {
        let expected = self.periodic_sources()?;
        let missing = selected
            .iter()
            .filter(|requested| {
                !expected
                    .iter()
                    .any(|available| available.eq_ignore_ascii_case(requested))
            })
            .cloned()
            .collect::<Vec<_>>();
        let omitted = expected
            .iter()
            .filter(|available| {
                !selected
                    .iter()
                    .any(|requested| requested.eq_ignore_ascii_case(available))
            })
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() || !omitted.is_empty() {
            let mut details = Vec::new();
            if !missing.is_empty() {
                details.push(format!("unknown: {}", missing.join(", ")));
            }
            if !omitted.is_empty() {
                details.push(format!("omitted: {}", omitted.join(", ")));
            }
            return Err(format!(
                "PSS tones do not match the exact elaborated periodic-source catalog ({})",
                details.join("; ")
            ));
        }
        Ok(())
    }
}

impl Default for AnalysisDependencyRepairContext {
    fn default() -> Self {
        Self::periodic_sources_unavailable(
            "the exact elaborated periodic-source catalog is unavailable",
        )
    }
}

pub(super) fn prerequisite_draft_for(
    dependent: &AnalysisDraft,
    prerequisite: AnalysisKind,
    context: &AnalysisDependencyRepairContext,
) -> Result<AnalysisDraft, String> {
    if prerequisite == AnalysisKind::Transient
        && let AnalysisDraft::Fourier(fourier) = dependent
    {
        let requirement = fourier_requirement(fourier)
            .map_err(|detail| format!("Fourier configuration is invalid: {detail}"))?;
        let required_interval = requirement.required_sample_interval()?;
        // Preserve margin against text round-tripping and future solver output
        // interpolation by targeting 10 samples for every highest-basis cycle.
        let interval = required_interval * 0.8;
        return Ok(AnalysisDraft::Transient(TranSetup {
            stop: format!("{:.12e}", requirement.stop_time),
            step: format!("{interval:.12e}"),
            start: format!("{:.12e}", requirement.start_time),
            max_step: format!("{interval:.12e}"),
            uic: false,
        }));
    }
    if prerequisite == AnalysisKind::Transient
        && let AnalysisDraft::Fft(fft) = dependent
    {
        let request = fft
            .to_request()
            .map_err(|detail| format!("FFT configuration is invalid: {detail}"))?;
        // Only an authored STOP can size a transient. Without one the card
        // takes the transient's own stop time, and the default transient is
        // exactly the run the author has not yet constrained.
        let Some(stop) = request.stop else {
            return Ok(AnalysisDraft::for_kind(prerequisite));
        };
        let start = request.start.unwrap_or(0.0);
        let step = (stop - start) / request.points as f64;
        return Ok(AnalysisDraft::Transient(TranSetup {
            stop: format!("{stop:.12e}"),
            step: format!("{step:.12e}"),
            start: format!("{:.12e}", 0.0),
            max_step: format!("{step:.12e}"),
            uic: false,
        }));
    }
    if prerequisite == AnalysisKind::Pss {
        if matches!(
            dependent,
            AnalysisDraft::Pac(_)
                | AnalysisDraft::Pnoise(_)
                | AnalysisDraft::Pxf(_)
                | AnalysisDraft::Pstb(_)
        ) {
            periodic_state_requirement(dependent)?;
        }
        let sources = context.periodic_sources()?;
        let draft = PssDialogState {
            tone_sources: sources.join(", "),
            ..Default::default()
        };
        context.validate_pss_sources(&draft)?;
        let draft = AnalysisDraft::Pss(draft);
        if let Some(issue) = dependency_configuration_issue(dependent, &draft) {
            return Err(issue.detail().to_owned());
        }
        return Ok(draft);
    }
    let draft = AnalysisDraft::for_kind(prerequisite);
    // A synthesized carrier is refused here for the same reason a chosen one
    // is: a repair that inserted it would leave the plan holding a dependency
    // its own contract rejects, with no further repair to offer.
    if prerequisite == AnalysisKind::HarmonicBalance
        && let Some(issue) = dependency_configuration_issue(dependent, &draft)
    {
        return Err(issue.detail().to_owned());
    }
    Ok(draft)
}

pub(super) fn dependency_candidate_context_issue(
    prerequisite: AnalysisKind,
    candidate: &AnalysisDraft,
    context: &AnalysisDependencyRepairContext,
) -> Option<String> {
    if prerequisite != AnalysisKind::Pss {
        return None;
    }
    let AnalysisDraft::Pss(pss) = candidate else {
        return Some("the prerequisite does not contain a PSS draft".to_owned());
    };
    context.validate_pss_sources(pss).err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_fft_specification_restores_from_the_wire_with_engine_defaults() {
        use crate::simulation::multi_run::AnalysisSpec;

        // The wire twin: an `AnalysisSpec::Fft` sealed before an optional
        // qualifier was authorable restores with that qualifier unauthored,
        // which is what the engine reads as its own default.
        let wire = r#"{"Fft":{"request":{"output":"V(OUT)","points":256,"window":"RECT"}}}"#;
        let spec: AnalysisSpec = serde_json::from_str(wire).expect("the sealed spec restores");
        let AnalysisSpec::Fft { request } = &spec else {
            panic!("an FFT specification");
        };
        assert_eq!(request.start, None);
        assert_eq!(request.stop, None);
        assert_eq!(request.format, None);
        assert_eq!(request.alfa, None);
        assert_eq!(request.to_card(), ".fft V(OUT) NP=256 WINDOW=RECT");
        assert!(spec.validate().is_ok());
    }

    #[test]
    fn all_kinds_have_exact_tagged_drafts_and_legacy_mapping() {
        for (index, kind) in AnalysisKind::ALL.into_iter().enumerate() {
            let draft = AnalysisDraft::for_kind(kind);
            assert_eq!(draft.kind(), kind);
            assert_eq!(draft.legacy_index(), index);
            assert_eq!(
                AnalysisKind::from_legacy_index(index)
                    .map(AnalysisDraft::for_kind)
                    .expect("known index")
                    .kind(),
                kind
            );
            let value = serde_json::to_value(&draft).expect("draft serializes");
            assert_eq!(value["kind"], kind.stable_id());
        }
        assert!(AnalysisKind::from_legacy_index(AnalysisKind::ALL.len()).is_none());
    }

    #[test]
    fn restore_preserves_raw_edits_and_only_repairs_lazy_sentinel() {
        let mut draft = AnalysisDraft::for_kind(AnalysisKind::OperatingPoint);
        let AnalysisDraft::OperatingPoint(state) = &mut draft else {
            panic!("expected OP draft");
        };
        state.temperature = "unfinished(".to_owned();
        let json = serde_json::to_string(&draft).expect("draft serializes");
        assert!(!json.contains("initialized"));

        let mut restored: AnalysisDraft = serde_json::from_str(&json).expect("draft restores");
        let AnalysisDraft::OperatingPoint(state) = &restored else {
            panic!("expected OP draft");
        };
        assert!(!state.initialized);
        assert_eq!(state.temperature, "unfinished(");

        restored.prepare_after_restore();
        let AnalysisDraft::OperatingPoint(state) = restored else {
            panic!("expected OP draft");
        };
        assert!(state.initialized);
        assert_eq!(state.temperature, "unfinished(");
    }

    #[test]
    fn envelope_periodic_initializer_is_owned_by_the_envelope_task() {
        let mut draft = AnalysisDraft::for_kind(AnalysisKind::Envelope);
        for selection in 0..=2 {
            let AnalysisDraft::Envelope(state) = &mut draft else {
                panic!("expected Envelope draft");
            };
            state.initial_periodic_solve_idx = selection;
            assert!(draft.prerequisite_roles().is_empty());
        }
    }

    /// An empty elaborated source set is a fact; an unreadable one is not.
    ///
    /// Asked of a driven solve, which is the only mode the catalog is
    /// load-bearing for: the driven period is defined by closing over the
    /// elaborated set, so a set that could not be read has to fail closed. An
    /// autonomous solve takes its period from the oscillator node and reads no
    /// tone list, so it needs neither answer.
    #[test]
    fn exact_periodic_source_context_distinguishes_empty_from_unavailable() {
        let exact_empty = AnalysisDependencyRepairContext::exact_periodic_sources(
            "driven fixture\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.end\n",
        )
        .expect("an elaborated source set is authoritative");
        let mut driven = PssDialogState::default();
        driven.tone_sources = "V1".to_owned();
        exact_empty
            .validate_pss_sources(&driven)
            .expect("a driven PSS closing over the exact source set commits");

        let unavailable = AnalysisDependencyRepairContext::default();
        assert!(
            unavailable
                .validate_pss_sources(&driven)
                .unwrap_err()
                .contains("unavailable")
        );

        let mut autonomous = PssDialogState::default();
        autonomous.tone_sources.clear();
        autonomous.osc_mode = true;
        autonomous.osc_node = "out".to_owned();
        unavailable
            .validate_pss_sources(&autonomous)
            .expect("an autonomous solve does not consult the driven-source catalog at all");
    }

    /// An oscillator with a startup kick can run an autonomous PSS.
    ///
    /// It could not run one either way before. The autonomous mode refuses a
    /// tone list, and the driven periodic-source contract refuses an omitted
    /// source — and a kick PULSE is an elaborated transient source, so an empty
    /// selection failed the second rule and a full one failed the first.
    ///
    /// The engine has no such rule. `PssConfig` carries no tone field at all
    /// (`rspice-core/src/analysis/pss/config.rs`), and the shooting solve
    /// evaluates every transient source at the window time whatever the tone
    /// list said (`rspice-core/src/engine/pss.rs`, the `update_transient_rhs`
    /// pair in `pss_stamp_system`). The contract that was being applied says in
    /// its own first line that it defines a *driven* period
    /// (`rspice-core/src/engine/transient.rs:1349-1352`).
    #[test]
    fn an_autonomous_pss_runs_beside_a_startup_kick_that_a_driven_one_refuses() {
        let kicked = AnalysisDependencyRepairContext::exact_periodic_sources(
            "oscillator fixture\nVKICK n1 0 PULSE(0 1 0 1n 1n 10n 1)\nL1 n1 0 1u\nC1 n1 0 1p\n.end\n",
        )
        .expect("the kicked oscillator's source identity elaborates");

        let mut autonomous = PssDialogState::default();
        autonomous.tone_sources.clear();
        autonomous.osc_mode = true;
        autonomous.osc_node = "n1".to_owned();
        kicked
            .validate_pss_sources(&autonomous)
            .expect("an autonomous solve reads no tone list, so the kick omits nothing");

        // The driven rule is unchanged, and still names the source it is about:
        // an omitted transient source there really does drive a solve whose
        // period is defined by the list that leaves it out.
        let mut driven = PssDialogState::default();
        driven.tone_sources.clear();
        let error = driven
            .to_config()
            .expect_err("a driven solve with no tone is refused before the catalog is consulted");
        assert!(error.contains("periodic tone source"), "{error}");

        let mut driven = PssDialogState::default();
        driven.tone_sources = "VSTALE".to_owned();
        let error = kicked
            .validate_pss_sources(&driven)
            .expect_err("a driven solve must close over the elaborated source set");
        assert!(error.contains("omitted: VKICK"), "{error}");

        // And the tone list stays a control that ships connected: under
        // autonomous it is read by nothing, so authoring one is refused rather
        // than silently ignored.
        let mut contradictory = PssDialogState::default();
        contradictory.osc_mode = true;
        contradictory.osc_node = "n1".to_owned();
        contradictory.tone_sources = "VKICK".to_owned();
        let error = contradictory
            .to_config()
            .expect_err("an autonomous solve has no tone list to author");
        assert!(error.contains("reads no tone list"), "{error}");
    }

    #[test]
    fn exact_periodic_source_context_rejects_unknown_and_omitted_tones() {
        let context = AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nVLO lo 0 SIN(0 1 1k)\nVRF rf 0 SIN(0 1 2k)\nR1 lo 0 1k\nR2 rf 0 1k\n.end\n",
        )
        .expect("test source catalog is exact");
        let mut pss = PssDialogState::default();
        pss.tone_sources = "vlo, VSTALE".to_owned();

        let error = context
            .validate_pss_sources(&pss)
            .expect_err("unknown and omitted tones fail closed");
        assert!(error.contains("unknown: VSTALE"));
        assert!(error.contains("omitted: VRF"));
    }

    #[test]
    fn exact_periodic_source_context_rejects_nonperiodic_or_incommensurate_waveforms() {
        let incommensurate = AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nV1 out 0 SIN(0 1 1.1k)\nR1 out 0 1k\n.end\n",
        )
        .expect("source identity elaborates");
        let mut pss = PssDialogState::default();
        pss.tone_sources = "V1".to_owned();
        let error = incommensurate
            .validate_pss_sources(&pss)
            .expect_err("repair cannot commit an incommensurate PSS");
        assert!(error.contains("frequencies must be integer multiples"));

        let pwl = AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nV1 out 0 PWL(0 0 1u 1)\nR1 out 0 1k\n.end\n",
        )
        .expect("PWL source identity elaborates");
        let error = pwl
            .validate_pss_sources(&pss)
            .expect_err("repair cannot claim an unauthenticated PWL period");
        assert!(error.contains("source 'V1' is not certified periodic"));
    }

    #[test]
    fn periodic_source_preflight_resolves_edges_from_the_drafts_actual_grid() {
        let context = AnalysisDependencyRepairContext::exact_periodic_sources(
            "source defaults\nV1 in 0 PULSE(0 1 0.7u 0 0 0.28u 1u)\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        ).unwrap();
        let mut draft = PssDialogState::default();
        draft.fund_freq = "1meg".to_owned();
        draft.tone_sources = "V1".to_owned();
        draft.num_harmonics = "4".to_owned();
        for (points, periodic) in [(32, false), (512, true)] {
            draft.points_per_period = points.to_string();
            let result = context.validate_pss_sources(&draft);
            assert_eq!(result.is_ok(), periodic, "{points}: {result:?}");
        }
    }

    #[test]
    fn noise_and_disto_own_independent_sweep_drafts() {
        let mut ac = AnalysisDraft::for_kind(AnalysisKind::Ac);
        let noise = AnalysisDraft::for_kind(AnalysisKind::Noise);
        let disto = AnalysisDraft::for_kind(AnalysisKind::Disto);
        let AnalysisDraft::Ac(ac_draft) = &mut ac else {
            panic!("expected AC draft");
        };
        ac_draft.points = "777".to_owned();

        let AnalysisDraft::Noise(noise) = noise else {
            panic!("expected noise draft");
        };
        let AnalysisDraft::Disto(disto) = disto else {
            panic!("expected DISTO draft");
        };
        assert_eq!(noise.points, "30");
        assert_eq!(noise.sweep, NoiseSweepType::Decade);
        assert_eq!(disto.sweep.points, "101");
    }

    #[test]
    fn noise_defaults_match_the_eight_field_mockup_contract() {
        let draft = NoiseDraft::default();
        assert_eq!(draft.sweep, NoiseSweepType::Decade);
        assert_eq!(draft.points, "30");
        assert_eq!(draft.fstart, "10");
        assert_eq!(draft.fstop, "1Meg");
        // The two fields that name the user's circuit open unset, and
        // conversion says which one it is still waiting for.
        assert_eq!(draft.output, "");
        assert_eq!(draft.input, "");
        assert_eq!(draft.contribution_detail, NoiseContributionDetail::Top50);
        assert_eq!(draft.integration_mode, NoiseIntegrationMode::Enabled);
        assert_eq!(
            draft.to_config().expect_err("no output node is named"),
            "output node or expression is required"
        );
        let named_output = NoiseDraft {
            output: "out".to_owned(),
            ..NoiseDraft::default()
        };
        assert_eq!(
            named_output
                .to_config()
                .expect_err("no input source is named"),
            "input source is required"
        );
        assert!(
            NoiseDraft {
                input: "V1".to_owned(),
                ..named_output
            }
            .to_config()
            .is_ok()
        );
    }

    #[test]
    fn legacy_noise_draft_migrates_without_inventing_active_settings() {
        let json = r#"{
            "kind":"noise",
            "draft":{
                "output":"legacy_out",
                "reference":"legacy_ref",
                "input":"VLEGACY",
                "fstart":"2",
                "fstop":"2Meg",
                "points":"17",
                "sweep":1
            }
        }"#;
        let restored: AnalysisDraft = serde_json::from_str(json).expect("legacy draft migrates");
        let AnalysisDraft::Noise(restored) = restored else {
            panic!("expected migrated noise draft");
        };
        assert_eq!(restored.sweep, NoiseSweepType::Octave);
        assert_eq!(restored.contribution_detail, NoiseContributionDetail::Top50);
        assert_eq!(restored.integration_mode, NoiseIntegrationMode::Enabled);
        let config = restored
            .to_config()
            .expect("migrated draft remains executable");
        assert_eq!(config.output_node, "legacy_out");
        assert_eq!(config.reference_node, "legacy_ref");
        assert_eq!(config.input_source, "VLEGACY");
        assert_eq!(config.sweep_type, AcSweepType::Octave);
        assert_eq!(config.num_points, 17);
    }

    #[test]
    fn current_noise_draft_round_trips_every_owned_field() {
        let draft = NoiseDraft {
            output: "sensor_p,sensor_n".to_owned(),
            reference: "legacy_reference_is_retained".to_owned(),
            input: "IIN_CAL".to_owned(),
            fstart: "11".to_owned(),
            fstop: "9Meg".to_owned(),
            points: "41".to_owned(),
            sweep: NoiseSweepType::ExplicitFrequencyList,
            explicit_frequencies: "11 1k 9Meg".to_owned(),
            contribution_detail: NoiseContributionDetail::SummaryOnly,
            integration_mode: NoiseIntegrationMode::Disabled,
        };
        let json = serde_json::to_string(&draft).expect("current draft serializes");
        assert!(json.contains("\"sweep\":\"explicit_frequency_list\""));
        let restored: NoiseDraft = serde_json::from_str(&json).expect("current draft restores");
        assert_eq!(restored, draft);
    }

    #[test]
    fn invalid_legacy_noise_sweep_is_retained_and_rejected() {
        let json = r#"{
            "kind":"noise",
            "draft":{
                "output":"out",
                "reference":"0",
                "input":"V1",
                "fstart":"1",
                "fstop":"1Meg",
                "points":"10",
                "sweep":99
            }
        }"#;
        let restored: AnalysisDraft =
            serde_json::from_str(json).expect("invalid index is retained");
        let AnalysisDraft::Noise(restored) = restored else {
            panic!("expected noise draft");
        };
        assert_eq!(restored.sweep, NoiseSweepType::Unsupported(99));
        assert!(restored.to_config().is_err());
    }

    #[test]
    fn noise_differential_output_and_explicit_axis_convert_exactly() {
        let draft = NoiseDraft {
            output: "V(sensor_p, sensor_n)".to_owned(),
            input: "IIN_CAL".to_owned(),
            sweep: NoiseSweepType::ExplicitFrequencyList,
            explicit_frequencies: "10, 1k; 1Meg".to_owned(),
            contribution_detail: NoiseContributionDetail::AllContributors,
            integration_mode: NoiseIntegrationMode::OutputNoiseOnly,
            ..NoiseDraft::default()
        };
        let config = draft.to_config().expect("exact draft converts");
        assert_eq!(config.output_node, "sensor_p");
        assert_eq!(config.reference_node, "sensor_n");
        assert_eq!(config.input_source, "IIN_CAL");
        assert_eq!(config.explicit_frequencies, Some(vec![10.0, 1.0e3, 1.0e6]));
        assert_eq!(
            config.contribution_detail,
            NoiseContributionDetail::AllContributors
        );
        assert_eq!(
            config.integration_mode,
            NoiseIntegrationMode::OutputNoiseOnly
        );
    }

    #[test]
    fn noise_validation_rejects_invalid_output_input_and_frequency_axes() {
        for draft in [
            NoiseDraft {
                output: "V(a,b,c)".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                input: " ".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                output: "V(out\n)\n.op".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                input: "VIN\n.tran 1n 1u".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                fstop: "1".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                sweep: NoiseSweepType::ExplicitFrequencyList,
                explicit_frequencies: "10, 10".to_owned(),
                ..NoiseDraft::default()
            },
        ] {
            assert!(draft.to_config().is_err());
            assert!(
                AnalysisDraft::Noise(draft.clone())
                    .manifest_configuration_error()
                    .is_some()
            );
        }
    }

    #[test]
    fn transfer_function_summary_uses_only_the_current_dc_contract() {
        let AnalysisDraft::TransferFunction(mut draft) =
            AnalysisDraft::for_kind(AnalysisKind::TransferFunction)
        else {
            panic!("expected a transfer-function draft");
        };
        draft.input_source = "V1".to_owned();
        draft.output_expression = "V(out)".to_owned();
        assert_eq!(
            AnalysisDraft::TransferFunction(draft)
                .manifest_summary()
                .as_deref(),
            Some("V(out) <- V1 - DC operating point")
        );
    }

    #[test]
    fn missing_manifest_analysis_drafts_are_typed_validated_and_round_trip() {
        for kind in [
            AnalysisKind::Qpss,
            AnalysisKind::Hbsp,
            AnalysisKind::Hbnoise,
            AnalysisKind::Psp,
            AnalysisKind::Qpac,
            AnalysisKind::Qpnoise,
            AnalysisKind::Qpxf,
            AnalysisKind::TransientNoise,
            AnalysisKind::DcMismatch,
            AnalysisKind::AcData,
        ] {
            let draft = AnalysisDraft::for_kind(kind);
            assert_eq!(draft.kind(), kind);
            assert!(draft.manifest_configuration_error().is_none(), "{kind}");
            assert!(draft.manifest_summary().is_some(), "{kind}");
            let bytes = serde_json::to_vec(&draft).expect("draft serializes");
            let restored: AnalysisDraft =
                serde_json::from_slice(&bytes).expect("draft deserializes");
            assert_eq!(restored.kind(), kind);
        }
    }

    /// A plan that stated only its frequencies still opens, referring to the
    /// table the writer will generate.
    ///
    /// The draft carries `deny_unknown_fields`, so a key it does not know is
    /// refused and an absent key is the `serde(default)` — this is the test
    /// that the default is the generated table name rather than an empty
    /// string, which would refuse the restored plan at its first validation.
    #[test]
    fn a_saved_frequency_table_plan_reopens_on_the_generated_table_name() {
        let saved = serde_json::json!({ "frequencies": "1k, 10k, 100k" });
        let draft: AcDataDraft =
            serde_json::from_value(saved).expect("a plan that stated only its axis still opens");
        assert_eq!(
            draft.table_name,
            crate::simulation::config::AC_FREQUENCY_TABLE
        );
        assert_eq!(draft.frequencies, "1k, 10k, 100k");
        let config = draft.to_config().expect("the restored plan is executable");
        assert_eq!(config.frequencies, vec![1.0e3, 1.0e4, 1.0e5]);

        // And the tagged draft round-trips under its own serde name, which is
        // what a saved project holds.
        let tagged = serde_json::to_string(&AnalysisDraft::AcData(draft))
            .expect("the tagged draft serializes");
        assert!(tagged.contains("\"acdata\""), "{tagged}");
        let restored: AnalysisDraft =
            serde_json::from_str(&tagged).expect("the tagged draft deserializes");
        assert_eq!(restored.kind(), AnalysisKind::AcData);
    }

    /// A plan saved before the share threshold existed still opens, listing
    /// the contributors it was listing.
    ///
    /// Same shim, same reason as the noise floor below: `deny_unknown_fields`
    /// says nothing about an absent key, so `serde(default)` is what lets the
    /// plan open at all, and this is the test that it is there. Empty rather
    /// than `0` is the value that reopens: both mean the same card, and empty
    /// is the spelling this form canonicalizes to.
    #[test]
    fn a_plan_saved_before_the_share_threshold_field_opens_without_one() {
        let saved = serde_json::json!({
            "output_expression": "V(out)",
            "sigma_multiplier": "1",
            "contributor_limit": "10",
            "include_process": false,
            "include_mismatch": true,
            "normalized_contributions": true
        });
        let draft: DcMismatchDraft =
            serde_json::from_value(saved).expect("a plan saved before the threshold still opens");
        assert!(
            draft.share_threshold.is_empty(),
            "a saved plan must reopen on the untrimmed list it ran: {:?}",
            draft.share_threshold
        );
        assert_eq!(draft.output_expression, "V(out)");
        assert_eq!(draft.contributor_limit, "10");
        assert!(validate_dc_mismatch(&draft).is_none());
    }

    /// An authored zero is the unauthored card, and reaches the run as one.
    ///
    /// The engine's own default threshold is exactly zero. Two
    /// specifications that differ only in how that zero was spelled would
    /// give one analysis two plan digests, so the two spellings are
    /// canonicalized to one before the specification is built.
    #[test]
    fn a_share_threshold_of_zero_is_the_unauthored_card() {
        assert_eq!(dc_mismatch_share_threshold(""), Ok(None));
        assert_eq!(dc_mismatch_share_threshold("   "), Ok(None));
        assert_eq!(dc_mismatch_share_threshold("0"), Ok(None));
        assert_eq!(dc_mismatch_share_threshold("0.0"), Ok(None));
        assert_eq!(dc_mismatch_share_threshold("0.05"), Ok(Some(0.05)));
        assert_eq!(dc_mismatch_share_threshold("1"), Ok(Some(1.0)));
        assert!(dc_mismatch_share_threshold("half").is_err());

        // And the range belongs to the card, so an out-of-range share is
        // refused by the specification rather than here.
        let mut draft = DcMismatchDraft::default();
        draft.share_threshold = "1.5".to_owned();
        let refusal = validate_dc_mismatch(&draft).expect("a share above one is refused");
        assert!(
            refusal.contains("THRESHOLD must be a variance share in [0, 1]"),
            "{refusal}"
        );
    }

    /// A plan saved before the noise floor field existed still opens, running
    /// the band it was running.
    ///
    /// The draft carries `deny_unknown_fields`, which refuses keys it does not
    /// know and says nothing about keys that are absent — so the shim is the
    /// `serde(default)`, and this is the test that it is there. The restored
    /// value has to be the empty field rather than any frequency: empty is the
    /// engine's `1/tstop` derivation, which is what that saved plan asked for.
    #[test]
    fn a_plan_saved_before_the_noise_floor_field_opens_with_the_engine_default() {
        let saved = serde_json::json!({
            "stop_time": "1u",
            "step_time": "1n",
            "start_time": "0",
            "max_step": "10n",
            "seed": "1",
            "noise_fmax": "10G",
            "scale": "1",
            "use_initial_conditions": false
        });
        let draft: TransientNoiseDraft =
            serde_json::from_value(saved).expect("a plan saved before the floor still opens");
        assert!(
            draft.noise_fmin.is_empty(),
            "a saved plan must reopen on the derivation it ran, not on a frequency: {:?}",
            draft.noise_fmin
        );
        // Every field it did carry is still the field it carried.
        assert_eq!(draft.stop_time, "1u");
        assert_eq!(draft.noise_fmax, "10G");
        assert_eq!(draft.seed, "1");
        assert_eq!(draft.scale, "1");
        assert!(validate_transient_noise(&draft).is_none());
        // And the run that plan resolves to asks for no floor at all, which is
        // the fact the saved bytes were making.
        assert!(
            AnalysisDraft::TransientNoise(draft)
                .manifest_configuration_error()
                .is_none()
        );
    }

    #[test]
    fn manifest_draft_validation_rejects_incomplete_configuration() {
        let mut qpss = QpssDraft::default();
        qpss.tones = "1G".to_owned();
        assert!(validate_qpss(&qpss).is_some());

        let mut network = PeriodicNetworkDraft::default();
        network.ports[0].node_pos.clear();
        assert!(validate_periodic_network(&network).is_some());

        let mut psp = PeriodicNetworkDraft::default();
        psp.mixed_mode = true;
        assert!(
            validate_psp_network(&psp).is_none(),
            "an even equal-impedance port list supports mixed-mode conversion"
        );
        psp.noise_parameters = true;
        assert!(validate_psp_network(&psp).is_none());

        let mut hbsp = PeriodicNetworkDraft::default();
        hbsp.noise_parameters = true;
        assert!(validate_periodic_network(&hbsp).is_none());

        let mut tnoise = TransientNoiseDraft::default();
        tnoise.seed = "0".to_owned();
        tnoise.scale = "0".to_owned();
        assert!(validate_transient_noise(&tnoise).is_none());

        // An empty noise floor is the engine's derivation and refuses
        // nothing; an authored one has to sit under the ceiling.
        let mut floor = TransientNoiseDraft::default();
        assert!(floor.noise_fmin.is_empty());
        assert!(validate_transient_noise(&floor).is_none());
        floor.noise_fmin = "1k".to_owned();
        assert!(validate_transient_noise(&floor).is_none());
        floor.noise_fmin = "100G".to_owned();
        assert!(validate_transient_noise(&floor).is_some());

        let mut mismatch = DcMismatchDraft::default();
        mismatch.include_mismatch = false;
        assert!(validate_dc_mismatch(&mismatch).is_some());

        // Zero contributors is the card's own spelling of "list every one",
        // so the draft accepts it rather than refusing a legal card.
        let mut all = DcMismatchDraft::default();
        all.contributor_limit = "0".to_owned();
        assert!(
            validate_dc_mismatch(&all).is_none(),
            "{:?}",
            validate_dc_mismatch(&all)
        );
    }

    /// A fresh DC mismatch draft is the bare card `.DCMATCH OUT=V(out)`.
    ///
    /// Read from the engine rather than pinned to numbers: the card's
    /// defaults are parsed out of a bare `.DCMATCH` line and compared with
    /// the draft's, so a default Studio run and a hand-written card are the
    /// same analysis by construction. The draft used to open at `3` sigma
    /// over `25` contributors, which named a study no card described.
    #[test]
    fn a_default_dc_mismatch_draft_is_the_bare_engine_card() {
        let draft = DcMismatchDraft::default();
        assert!(validate_dc_mismatch(&draft).is_none());

        let netlist = rspice_core::netlist::Netlist::parse(
            "dc mismatch defaults\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n.DCMATCH \
             OUT=V(out)\n.end\n",
        )
        .expect("a bare .DCMATCH card parses");
        let [rspice_core::netlist::AnalysisCommand::DcMatch(card)] = netlist.analyses.as_slice()
        else {
            panic!("the deck holds one .DCMATCH: {:?}", netlist.analyses);
        };

        assert_eq!(
            draft
                .sigma_multiplier
                .parse::<f64>()
                .expect("the default multiplier is a number"),
            card.sigma_multiplier
        );
        assert_eq!(
            draft
                .contributor_limit
                .parse::<usize>()
                .expect("the default limit is a count"),
            card.contributor_limit
        );
        assert_eq!(draft.include_mismatch, card.mismatch);
        assert_eq!(draft.include_process, card.process);
        // The parser canonicalizes the probe to upper case; the probe itself
        // is the same one.
        assert_eq!(
            draft.output_expression.to_ascii_uppercase(),
            format!("V({})", card.output_node)
        );
    }

    #[test]
    fn periodic_network_draft_accepts_discovery_single_ports_and_zero_sideband() {
        let mut draft = PeriodicNetworkDraft::default();
        draft.max_sideband = "0".to_owned();
        draft.ports.truncate(1);
        assert!(validate_periodic_network(&draft).is_none());
        draft.ports.clear();
        assert!(validate_periodic_network(&draft).is_none());
        for invalid in ["-1", "1.5", "2147483648"] {
            draft.max_sideband = invalid.to_owned();
            assert!(validate_periodic_network(&draft).is_some());
        }
    }
    #[test]
    fn hbnoise_reference_drafts_round_trip_and_legacy_defaults_remain_disabled() {
        let mut draft = HbNoiseDraft::default();
        draft.input_sideband = "-2".into();
        draft.output_sideband = "1".into();
        draft.noise_figure = true;
        draft.source_resistor = "Rs".into();
        draft.reference_temperature = "325".into();
        let expected = draft.noise_reference().unwrap();
        let json = serde_json::to_string(&draft).unwrap();
        let ron = ron::to_string(&draft).unwrap();
        let from_json: HbNoiseDraft = serde_json::from_str(&json).unwrap();
        assert_eq!(from_json.sidebands().unwrap(), (-2, 1));
        let from_ron: HbNoiseDraft = ron::from_str(&ron).unwrap();
        assert_eq!(from_json.noise_reference().unwrap(), expected);
        assert_eq!(from_ron.noise_reference().unwrap(), expected);
        let mut legacy = serde_json::to_value(HbNoiseDraft::default()).unwrap();
        legacy.as_object_mut().unwrap().remove("source_resistor");
        legacy
            .as_object_mut()
            .unwrap()
            .remove("reference_temperature");
        legacy.as_object_mut().unwrap().remove("input_sideband");
        legacy.as_object_mut().unwrap().remove("output_sideband");
        let restored: HbNoiseDraft = serde_json::from_value(legacy).unwrap();
        assert_eq!(restored.sidebands().unwrap(), (0, 0));
        assert!(!restored.noise_figure);
        assert_eq!(restored.noise_reference().unwrap(), None);
        assert_eq!(restored.reference_temperature, "290");
    }
}
