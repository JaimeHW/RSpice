//! Simulation setup — the typed analysis-configuration model behind the
//! Simulate view.
//!
//! Every analysis owns exactly one state struct: draft string buffers plus
//! a `to_config()` parse/validate step. The Simulate view edits these
//! structs and the controller consumes the very same structs when it
//! builds the run plan, so what you see is what runs. `enabled` is the run
//! set and `analysis_order` is its stable execution order.

pub(in crate::workbench) mod analysis_drafts;
pub(in crate::workbench) mod plan_catalog;

use std::collections::HashSet;

/// The nominal/reference operating point selected in the workbench chrome.
///
/// This is execution state, not display state: temperature is copied into the
/// effective solver options and process is used when resolving model-library
/// sections for a run. It is also what an axis the run set does not declare
/// resolves to, so it is one type rather than two that have to agree.
pub type ReferencePvtPoint = crate::simulation::run_set::ReferencePoint;

/// Plan-owned result delivery and retention policy. These controls are part of
/// the executable plan rather than project-global UI preferences: switching a
/// plan switches the policy, and a prepared snapshot authenticates it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationSavePolicy {
    /// How the plan chooses the quantities retained in each result dataset.
    #[serde(default)]
    pub output_selection_mode: crate::state::OutputSelectionMode,
    /// Maximum retained datasets produced by this plan. Golden baselines are
    /// exempt and may make the limit temporarily unenforceable.
    pub retained_dataset_limit: usize,
    /// Hard preflight ceiling for the bounded saved-output forecast.
    pub maximum_storage_bytes: u64,
    /// Whether contracts requesting live delivery may open a live stream.
    pub live_streaming_enabled: bool,
    /// Whether accepted transient samples are retained as failure diagnostics
    /// if the final solve fails or is interrupted.
    pub retain_failure_diagnostics: bool,
}

impl Default for SimulationSavePolicy {
    fn default() -> Self {
        Self {
            output_selection_mode: crate::state::OutputSelectionMode::Automatic,
            retained_dataset_limit: 20,
            maximum_storage_bytes: 10 * 1024 * 1024 * 1024,
            live_streaming_enabled: true,
            retain_failure_diagnostics: true,
        }
    }
}

impl SimulationSavePolicy {
    pub fn validate(self) -> Result<(), String> {
        if self.retained_dataset_limit == 0 || self.retained_dataset_limit > 10_000 {
            return Err("Plan retention must be from 1 through 10,000 datasets.".to_owned());
        }
        if self.maximum_storage_bytes == 0 {
            return Err("Plan saved-output storage budget must be greater than zero.".to_owned());
        }
        Ok(())
    }
}

/// `.tran` draft. SI suffixes allowed; "auto" max step defers to the
/// engine's LTE control.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranSetup {
    /// Stop time.
    pub stop: String,
    /// Suggested step time.
    pub step: String,
    /// Output start time.
    pub start: String,
    /// Max timestep, or "auto".
    pub max_step: String,
    /// Skip the DC operating point and use initial conditions.
    pub uic: bool,
}

impl Default for TranSetup {
    fn default() -> Self {
        Self {
            stop: "1m".to_owned(),
            step: "10n".to_owned(),
            start: "0".to_owned(),
            max_step: "auto".to_owned(),
            uic: false,
        }
    }
}

/// `.ac` draft — DISTO rides on the same sweep.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcSetup {
    /// Start frequency.
    pub fstart: String,
    /// Stop frequency.
    pub fstop: String,
    /// Points per decade/octave, or total when linear.
    pub points: String,
    /// 0 = decade, 1 = octave, 2 = linear.
    pub sweep: usize,
}

impl Default for AcSetup {
    fn default() -> Self {
        Self {
            fstart: "1".to_owned(),
            fstop: "1G".to_owned(),
            points: "101".to_owned(),
            sweep: 0,
        }
    }
}

/// `.dc` draft with the optional nested secondary sweep.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DcSetup {
    /// Swept source name.
    pub source: String,
    /// Sweep start value.
    pub start: String,
    /// Sweep stop value.
    pub stop: String,
    /// Sweep step.
    pub step: String,
    /// Nested secondary sweep enabled.
    pub nested: bool,
    /// Secondary source name.
    pub source2: String,
    /// Secondary start.
    pub start2: String,
    /// Secondary stop.
    pub stop2: String,
    /// Secondary step.
    pub step2: String,
}

impl Default for DcSetup {
    fn default() -> Self {
        Self {
            source: "V1".to_owned(),
            start: "0".to_owned(),
            stop: "5".to_owned(),
            step: "0.01".to_owned(),
            nested: false,
            source2: "V2".to_owned(),
            start2: "0".to_owned(),
            stop2: "3.3".to_owned(),
            step2: "0.1".to_owned(),
        }
    }
}

/// `.noise` draft.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoiseSetup {
    /// Output node.
    pub output: String,
    /// Reference node (0 = ground).
    pub reference: String,
    /// Input source.
    pub input: String,
    /// Start frequency.
    pub fstart: String,
    /// Stop frequency.
    pub fstop: String,
}

impl Default for NoiseSetup {
    fn default() -> Self {
        Self {
            output: "out".to_owned(),
            reference: "0".to_owned(),
            input: "V1".to_owned(),
            fstart: "1".to_owned(),
            fstop: "100Meg".to_owned(),
        }
    }
}

/// All analysis configuration plus the engine options, in one place.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimSetupState {
    /// Authoritative reference PVT point used by nominal analyses.
    pub reference_pvt: ReferencePvtPoint,
    /// Global process, supply, and temperature space applied to every
    /// executable analysis in the active plan.
    ///
    /// This is deliberately independent of the legacy Corner analysis draft:
    /// the Run Set page configures where the whole plan executes, while a
    /// Corner analysis remains an analysis instance with its own base mode.
    #[serde(default = "default_global_run_set")]
    pub run_set: crate::simulation::run_set::RunSetState,
    /// Ordered, content-pinned model libraries consumed by this plan.
    /// Absence is an explicit empty closure; execution never falls back to
    /// every library currently loaded in the project manager.
    #[serde(default)]
    pub model_bindings: Vec<crate::state::model_library::SimulationPlanModelBinding>,
    /// Result storage, live delivery, and per-plan history policy.
    #[serde(default)]
    pub save_policy: SimulationSavePolicy,
    /// Validated project-unique name of the active simulation plan.
    #[serde(default)]
    pub active_plan_name: crate::workbench::app_state::SimulationPlanName,
    /// Immutable source identity and revision when the active plan is a clone.
    #[serde(default)]
    pub active_plan_lineage: crate::workbench::app_state::SimulationPlanLineage,
    /// Complete inactive plans retained in deterministic catalog order.
    #[serde(default)]
    pub inactive_plans: Vec<crate::workbench::app_state::StoredSimulationPlan>,
    /// Stable, revisioned analysis-instance plan. `None` is accepted only
    /// while reading schema-3 projects/sessions and must be deterministically
    /// migrated before validation, editing, or execution.
    #[serde(default)]
    pub analysis_plan: Option<crate::simulation::plan::SimulationPlan>,
    /// Enabled analysis indices.
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "deserialize_analysis_set"
    )]
    pub enabled: HashSet<usize>,
    /// Stable execution order. Enabled analyses absent from this vector are
    /// appended deterministically; disabled entries are ignored and removed
    /// from the persisted normalized plan.
    #[serde(default, skip_serializing)]
    pub analysis_order: Vec<usize>,
    /// Transient sweep.
    #[serde(default, skip_serializing)]
    pub tran: TranSetup,
    /// AC sweep.
    #[serde(default, skip_serializing)]
    pub ac: AcSetup,
    /// DISTO secondary tone ratio f2/f1 (empty = single-tone HD).
    #[serde(default, skip_serializing)]
    pub disto_f2_over_f1: String,
    /// DC transfer sweep.
    #[serde(default, skip_serializing)]
    pub dc: DcSetup,
    /// Noise analysis.
    #[serde(default, skip_serializing)]
    pub noise: NoiseSetup,
    /// DC operating point.
    #[serde(default, skip_serializing)]
    pub op: crate::simulation::dialog::op::OpDialogState,
    /// Pole-zero extraction.
    #[serde(default, skip_serializing)]
    pub pz: crate::simulation::dialog::pz::PzDialogState,
    /// Sensitivity.
    #[serde(default, skip_serializing)]
    pub sens: crate::simulation::dialog::sens::SensDialogState,
    /// Monte Carlo.
    #[serde(default, skip_serializing)]
    pub mc: crate::simulation::dialog::mc::McDialogState,
    /// Periodic steady state.
    #[serde(default, skip_serializing)]
    pub pss: crate::simulation::dialog::pss::PssDialogState,
    /// Loop stability.
    #[serde(default, skip_serializing)]
    pub stb: crate::simulation::dialog::stb::StbDialogState,
    /// Temperature sweep.
    #[serde(default, skip_serializing)]
    pub temp: crate::simulation::dialog::temp::TempDialogState,
    /// Harmonic balance.
    #[serde(default, skip_serializing)]
    pub hb: crate::simulation::dialog::hb::HbDialogState,
    /// S-parameters.
    #[serde(default, skip_serializing)]
    pub sp: crate::simulation::dialog::sp::SpDialogState,
    /// Periodic AC.
    #[serde(default, skip_serializing)]
    pub pac: crate::simulation::dialog::pac::PacDialogState,
    /// Periodic noise.
    #[serde(default, skip_serializing)]
    pub pnoise: crate::simulation::dialog::pnoise::PnoiseDialogState,
    /// Periodic transfer.
    #[serde(default, skip_serializing)]
    pub pxf: crate::simulation::dialog::pxf::PxfDialogState,
    /// Periodic stability.
    #[serde(default, skip_serializing)]
    pub pstb: crate::simulation::dialog::pstb::PstbDialogState,
    /// Transfer function.
    #[serde(default, skip_serializing)]
    pub xf: crate::simulation::dialog::xf::XfDialogState,
    /// Process corners.
    #[serde(default, skip_serializing)]
    pub corner: crate::simulation::dialog::corner::CornerDialogState,
    /// Envelope transient.
    #[serde(default, skip_serializing)]
    pub envelope: crate::simulation::dialog::envelope::EnvelopeDialogState,
    /// Fourier.
    #[serde(default, skip_serializing)]
    pub fourier: crate::simulation::dialog::fourier::FourierDialogState,
    /// Reliability / aging.
    #[serde(default, skip_serializing)]
    pub reliability: crate::simulation::dialog::reliability::ReliabilityDialogState,
    /// Optimization.
    #[serde(default, skip_serializing)]
    pub optimization: crate::simulation::dialog::optimization::OptimizationDialogState,
    /// Safe operating area.
    #[serde(default, skip_serializing)]
    pub soa: crate::simulation::dialog::soa::SoaDialogState,
    /// Effective engine options (validated).
    pub options: crate::simulation::dialog::SimulationOptions,
    /// Draft buffers the Solver & convergence page edits before a commit.
    ///
    /// Never persisted: a project stores the effective options, and a draft
    /// that outlived the session would reopen a plan on a value no run used.
    #[serde(skip)]
    pub options_draft: crate::simulation::dialog::OptionsDialogState,
    /// Analyses listed in the run-set card beyond the always-listed core —
    /// exotics stay listed (dimmed) when unticked, until removed.
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "deserialize_analysis_set"
    )]
    pub listed: HashSet<usize>,
    /// Add-analysis palette open (anchored to the card action).
    #[serde(skip)]
    pub palette_open: bool,
    /// Palette filter query.
    #[serde(skip)]
    pub palette_query: String,
    /// Keyboard-active row in the palette's filtered list.
    #[serde(skip)]
    pub palette_active: usize,
    /// One-shot request to reveal the keyboard-active catalog row after open.
    ///
    /// The scroll area's retained viewport must not reopen halfway through the
    /// catalog after a previous browse session.
    #[serde(skip)]
    pub palette_scroll_to_active: bool,
}

impl SimSetupState {
    /// Fresh setup with the conventional default run set — a transient —
    /// so a new project's Run button works out of the box (the engine no
    /// longer falls back to the selected row on an empty set).
    pub fn new() -> Self {
        let mut setup = Self {
            analysis_plan: Some(crate::simulation::plan::SimulationPlan::new()),
            run_set: default_global_run_set(),
            ..Self::default()
        };
        setup
            .set_reference_pvt(crate::product::ProcessCorner::TT, 27.0)
            .expect("the built-in reference PVT point is valid");
        setup
            .enabled
            .insert(crate::workbench::simulation_analysis_tabs::TAB_TRANSIENT);
        setup
            .analysis_order
            .push(crate::workbench::simulation_analysis_tabs::TAB_TRANSIENT);
        setup
    }

    /// Create a fresh project-owned plan using the user's retained numerical
    /// default. The resulting options are copied into the plan: later changes
    /// to Preferences never mutate an existing plan or its reproducibility.
    pub fn new_with_user_preferences(preferences: &crate::workbench::UserPreferences) -> Self {
        use crate::workbench::ChoicePreference;

        let mut setup = Self::new();
        let mut options = match preferences.choice(ChoicePreference::DefaultSolverPreset) {
            1 => crate::simulation::dialog::SimulationOptions::fast(),
            2 => crate::simulation::dialog::SimulationOptions::accurate(),
            3 => crate::simulation::dialog::SimulationOptions::robust(),
            _ => crate::simulation::dialog::SimulationOptions::default(),
        };
        options.temp = setup.reference_pvt.temperature_celsius;
        setup.options = options;
        setup.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&setup.options);
        setup
    }

    /// Rebuild transient editing state after a persisted plan is restored.
    pub(crate) fn prepare_after_restore(&mut self) {
        if let Some(plan) = &mut self.analysis_plan {
            plan.prepare_after_restore();
        }
        self.prepare_plan_catalog_after_restore();
        self.op.initialized = true;
        self.pz.initialized = true;
        self.sens.initialized = true;
        self.mc.initialized = true;
        self.pss.initialized = true;
        self.stb.initialized = true;
        self.temp.initialized = true;
        self.hb.initialized = true;
        self.sp.initialized = true;
        self.pac.initialized = true;
        self.pnoise.initialized = true;
        self.pxf.initialized = true;
        self.pstb.initialized = true;
        self.xf.prepare_after_restore();
        self.corner.initialized = true;
        self.envelope.initialized = true;
        self.fourier.initialized = true;
        self.reliability.initialized = true;
        self.optimization.initialized = true;
        self.soa.initialized = true;
        self.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&self.options);
        self.palette_open = false;
        self.palette_query.clear();
        self.palette_active = 0;
        self.palette_scroll_to_active = false;
        self.refresh_legacy_analysis_projections();
    }

    /// Select the nominal/reference PVT point consumed by subsequent runs.
    pub fn set_reference_pvt(
        &mut self,
        process: crate::product::ProcessCorner,
        temperature_celsius: f64,
    ) -> Result<(), String> {
        if !temperature_celsius.is_finite() {
            return Err("Reference temperature must be finite".to_owned());
        }
        if temperature_celsius <= -273.15 {
            return Err("Reference temperature must be above absolute zero".to_owned());
        }

        self.reference_pvt = ReferencePvtPoint {
            process,
            temperature_celsius,
        };
        self.options.temp = temperature_celsius;
        self.options_draft.temp = format_temperature(temperature_celsius);
        self.op.ensure_initialized();
        self.op.temperature = format_temperature(temperature_celsius);
        Ok(())
    }

    /// The run space this plan resolves to, exactly.
    ///
    /// A plan without an enabled corner analysis runs once, at the reference
    /// point. With one enabled, the corner configuration owns the point set
    /// and this reports what that configuration actually expands to — a
    /// configuration that does not parse contributes no points rather than a
    /// guessed count.
    pub fn run_set_point_count(&self) -> Option<usize> {
        let validation = crate::simulation::run_set::validate(
            &self.run_set,
            self.enabled_analysis_instance_count(),
        );
        validation
            .errors
            .is_empty()
            .then_some(validation.forecast.point_count)
    }

    /// Every temperature this run set asks the engine for, in °C.
    ///
    /// The same rule the corner projection uses: a declared temperature axis
    /// is the request, and without one the reference point is the request —
    /// exactly once, because a plan with no axis runs at one temperature. It
    /// is stated here so a surface asking "is this corner qualified for what
    /// we are about to run" reads the run set rather than guessing from the
    /// reference point alone.
    #[must_use]
    pub fn requested_temperatures_celsius(&self) -> Vec<f64> {
        match self
            .run_set
            .enabled_dimension_of(crate::simulation::run_set::RunSetDimensionKind::Temperature)
        {
            Some(dimension) => dimension.canonical_values(),
            None => vec![self.reference_pvt.temperature_celsius],
        }
    }

    /// Commit globally validated options while keeping the workbench reference
    /// point and OP editor aligned with the temperature the solver will use.
    pub fn commit_options(&mut self, options: &crate::simulation::dialog::SimulationOptions) {
        self.options = options.clone();
        self.reference_pvt.temperature_celsius = options.temp;
        self.op.ensure_initialized();
        self.op.temperature = format_temperature(options.temp);
    }

    /// One-line mono summary of an analysis configuration, for list rows.
    pub fn summary(&self, index: usize) -> String {
        let sweep_kind = ["dec", "oct", "lin"];
        match index {
            0 => format!("T = {} C", self.op.temperature),
            1 => format!(
                "{} -> {} · step {}",
                self.tran.start, self.tran.stop, self.tran.step
            ),
            2 => format!(
                "{} -> {} · {}/{}",
                self.ac.fstart,
                self.ac.fstop,
                self.ac.points,
                sweep_kind[self.ac.sweep.min(2)]
            ),
            3 => {
                let mut text = format!(
                    "{}: {} -> {} · {}",
                    self.dc.source, self.dc.start, self.dc.stop, self.dc.step
                );
                if self.dc.nested {
                    text.push_str(" · nested");
                }
                text
            }
            4 => format!(
                "{} <- {} · {} -> {}",
                self.noise.output, self.noise.input, self.noise.fstart, self.noise.fstop
            ),
            5 => format!("{} -> {}", self.pz.input_pos, self.pz.output_pos),
            6 => self.sens.output_expr.clone(),
            7 => format!(
                "{} runs · {}",
                self.mc.num_runs,
                ["gaussian", "uniform", "worst-case"][self.mc.distribution_idx.min(2)]
            ),
            8 => format!(
                "f0 {} · {} harmonics",
                self.pss.fund_freq, self.pss.num_harmonics
            ),
            9 => format!(
                "{} · {} -> {}",
                self.stb.probe_source, self.stb.start_freq, self.stb.stop_freq
            ),
            10 => format!(
                "{} -> {} C · step {}",
                self.temp.temp_start, self.temp.temp_stop, self.temp.temp_step
            ),
            11 => format!(
                "f0 {} · {} tone{}",
                self.hb.fundamental,
                1 + self.hb.additional_tones.len(),
                if self.hb.additional_tones.is_empty() {
                    ""
                } else {
                    "s"
                }
            ),
            12 => format!("{} ports · Z0 {}", self.sp.ports.len(), self.sp.z0),
            13 => format!(
                "{} <- {} · {} -> {}",
                self.pac.output_node,
                self.pac.input_source,
                self.pac.start_freq,
                self.pac.stop_freq
            ),
            14 => format!(
                "{} · {} -> {}",
                self.pnoise.output_node, self.pnoise.start_freq, self.pnoise.stop_freq
            ),
            15 => format!(
                "{} <- {} · {} -> {}",
                self.pxf.output_node,
                self.pxf.input_source,
                self.pxf.start_freq,
                self.pxf.stop_freq
            ),
            16 => format!(
                "probe {} · {} harmonics",
                self.pstb.probe, self.pstb.max_harmonics
            ),
            17 => format!(
                "{} <- {} - DC operating point",
                self.xf.output_expression, self.xf.input_source
            ),
            18 => {
                let run_set = &self.corner.run_set;
                let axes: Vec<String> = run_set
                    .enabled_dimensions()
                    .map(|dimension| format!("{}×{}", dimension.name, dimension.values.len()))
                    .collect();
                if axes.is_empty() {
                    "no run-space axis enabled".to_owned()
                } else {
                    format!("{} = {} points", axes.join(" "), run_set.point_count())
                }
            }
            19 => format!(
                "tones {} · to {}",
                self.envelope.carrier_tones, self.envelope.stop_time
            ),
            20 => format!(
                "f0 {} · {}h @ {}",
                self.fourier.fundamental, self.fourier.harmonics, self.fourier.output_node
            ),
            21 => {
                let rel = &self.reliability;
                let flags = [
                    (rel.enable_hci, "HCI"),
                    (rel.enable_nbti, "NBTI"),
                    (rel.enable_em, "EM"),
                ];
                let on: Vec<&str> = flags.iter().filter(|(f, _)| *f).map(|(_, n)| *n).collect();
                format!(
                    "{} y · {}",
                    self.reliability.years_csv,
                    if on.is_empty() {
                        "no mechanisms".to_owned()
                    } else {
                        on.join(" ")
                    }
                )
            }
            22 => format!(
                "{} {} · {}",
                ["minimize", "maximize", "target"][self.optimization.goal_mode.min(2)],
                self.optimization.objective_node,
                ["gradient", "pattern", "anneal"][self.optimization.algorithm.min(2)]
            ),
            23 => {
                let soa = &self.soa;
                let flags = [
                    (soa.check_vgs_max, "vgs"),
                    (soa.check_vds_max, "vds"),
                    (soa.check_vbe_max, "vbe"),
                    (soa.check_vce_max, "vce"),
                ];
                let on: Vec<&str> = flags.iter().filter(|(f, _)| *f).map(|(_, n)| *n).collect();
                format!(
                    "to {} · {}",
                    soa.stop_time,
                    if on.is_empty() {
                        "no checks".to_owned()
                    } else {
                        on.join(" ")
                    }
                )
            }
            24 => {
                let ratio = self.disto_f2_over_f1.trim();
                if ratio.is_empty() {
                    format!("{} -> {}", self.ac.fstart, self.ac.fstop)
                } else {
                    format!("{} -> {} · f2/f1 {}", self.ac.fstart, self.ac.fstop, ratio)
                }
            }
            _ => String::new(),
        }
    }

    /// First validation problem in an analysis draft, if any — the same
    /// parse the controller performs when it builds the run plan.
    pub fn validation_error(&self, index: usize) -> Option<String> {
        use crate::simulation::controller::spice_value::parse_spice_value_checked as parse;
        let field = |name: &str, error: String| Some(format!("{name}: {error}"));
        match index {
            0 => self.op.to_config().err(),
            1 => {
                if let Err(e) = parse(&self.tran.stop) {
                    return field("stop time", e);
                }
                if let Err(e) = parse(&self.tran.step) {
                    return field("step time", e);
                }
                if let Err(e) = parse(&self.tran.start) {
                    return field("start time", e);
                }
                let max = self.tran.max_step.trim();
                if !max.is_empty()
                    && !max.eq_ignore_ascii_case("auto")
                    && let Err(e) = parse(max)
                {
                    return field("max step", e);
                }
                None
            }
            2 => self.ac_sweep_error(),
            3 => {
                if self.dc.source.trim().is_empty() {
                    return Some("sweep source is empty".to_owned());
                }
                if let Err(e) = parse(&self.dc.start) {
                    return field("start", e);
                }
                if let Err(e) = parse(&self.dc.stop) {
                    return field("stop", e);
                }
                if let Err(e) = parse(&self.dc.step) {
                    return field("step", e);
                }
                if self.dc.nested {
                    if self.dc.source2.trim().is_empty() {
                        return Some("nested sweep source is empty".to_owned());
                    }
                    if let Err(e) = parse(&self.dc.start2) {
                        return field("nested start", e);
                    }
                    if let Err(e) = parse(&self.dc.stop2) {
                        return field("nested stop", e);
                    }
                    if let Err(e) = parse(&self.dc.step2) {
                        return field("nested step", e);
                    }
                }
                None
            }
            4 => {
                if self.noise.output.trim().is_empty() {
                    return Some("output node is empty".to_owned());
                }
                if let Err(e) = parse(&self.noise.fstart) {
                    return field("start frequency", e);
                }
                if let Err(e) = parse(&self.noise.fstop) {
                    return field("stop frequency", e);
                }
                self.points_error()
            }
            5 => self.pz.to_config().err(),
            6 => self.sens.to_config().err(),
            7 => self.mc.to_config().err(),
            8 => self.pss.to_config().err(),
            9 => self.stb.to_config().err(),
            10 => self.temp.to_config().err(),
            11 => self.hb.to_config().err(),
            12 => self.sp.to_config().err(),
            13 => self.pac.to_config().err(),
            14 => self.pnoise.to_config().err(),
            15 => self.pxf.to_config().err(),
            16 => self.pstb.to_config().err(),
            17 => self.xf.to_config().err(),
            18 => self.corner.to_config(self.reference_pvt).err(),
            19 => self.envelope.to_config().err(),
            20 => self.fourier.to_config().err(),
            21 => self.reliability.to_config().err(),
            22 => self.optimization.to_config().err(),
            23 => self.soa.to_config().err(),
            24 => {
                if let Some(error) = self.ac_sweep_error() {
                    return Some(error);
                }
                let ratio = self.disto_f2_over_f1.trim();
                if !ratio.is_empty() && !ratio.eq_ignore_ascii_case("auto") {
                    match parse(ratio) {
                        Ok(value) if value.is_finite() && value > 0.0 && value < 1.0 => {}
                        Ok(_) => {
                            return Some(
                                "f2/f1 ratio: must be strictly between 0 and 1".to_owned(),
                            );
                        }
                        Err(error) => return field("f2/f1 ratio", error),
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn ac_sweep_error(&self) -> Option<String> {
        use crate::simulation::controller::spice_value::parse_spice_value_checked as parse;
        if let Err(e) = parse(&self.ac.fstart) {
            return Some(format!("start frequency: {e}"));
        }
        if let Err(e) = parse(&self.ac.fstop) {
            return Some(format!("stop frequency: {e}"));
        }
        self.points_error()
    }

    fn points_error(&self) -> Option<String> {
        match self.ac.points.trim().parse::<usize>() {
            Ok(points) if points > 0 => None,
            _ => Some(format!(
                "points: '{}' is not a positive integer",
                self.ac.points
            )),
        }
    }

    /// Fill defaults into any analysis state still at its blank
    /// `Default` (each guards on its own `initialized` flag).
    #[cfg(test)]
    pub fn ensure_initialized(&mut self) {
        self.op.ensure_initialized();
        self.pz.ensure_initialized();
        self.sens.ensure_initialized();
        self.mc.ensure_initialized();
        self.pss.ensure_initialized();
        self.stb.ensure_initialized();
        self.temp.ensure_initialized();
        self.hb.ensure_initialized();
        self.sp.ensure_initialized();
        self.pac.ensure_initialized();
        self.pnoise.ensure_initialized();
        self.pxf.ensure_initialized();
        self.pstb.ensure_initialized();
        self.xf.ensure_initialized();
        self.corner.ensure_initialized();
        self.envelope.ensure_initialized();
        self.fourier.ensure_initialized();
        self.reliability.ensure_initialized();
        self.optimization.ensure_initialized();
        self.soa.ensure_initialized();
    }
}

/// Missing global Run Set data belongs to a project from before the Studio
/// owned this declaration. Migrating it to a one-point reference run avoids
/// inventing 27 PVT tasks and new technology requirements on first open.
fn default_global_run_set() -> crate::simulation::run_set::RunSetState {
    crate::simulation::run_set::RunSetState::reference_only()
}

fn format_temperature(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.6}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
    }
}

fn deserialize_analysis_set<'de, D>(deserializer: D) -> Result<HashSet<usize>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let indices = <Vec<usize> as serde::Deserialize>::deserialize(deserializer)?;
    let mut unique = HashSet::with_capacity(indices.len());
    for index in indices {
        if !unique.insert(index) {
            return Err(serde::de::Error::custom(format!(
                "duplicate analysis index {index}"
            )));
        }
    }
    Ok(unique)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProcessCorner;

    #[test]
    fn reference_pvt_is_the_temperature_consumed_by_solver_and_op() {
        let mut setup = SimSetupState::new();

        setup
            .set_reference_pvt(ProcessCorner::FF, -40.0)
            .expect("reference point is valid");

        assert_eq!(setup.reference_pvt.process, ProcessCorner::FF);
        assert_eq!(setup.reference_pvt.temperature_celsius, -40.0);
        assert_eq!(setup.options.temp, -40.0);
        assert_eq!(setup.options_draft.temp, "-40");
        assert_eq!(setup.op.temperature, "-40");
    }

    #[test]
    fn reference_pvt_rejects_non_physical_temperature_without_mutation() {
        let mut setup = SimSetupState::new();
        let before = setup.reference_pvt;

        let error = setup
            .set_reference_pvt(ProcessCorner::SS, -273.15)
            .expect_err("absolute zero itself must fail");

        assert!(error.contains("absolute zero"));
        assert_eq!(setup.reference_pvt, before);
        assert_eq!(setup.options.temp, before.temperature_celsius);
    }

    #[test]
    fn missing_global_run_set_migrates_to_one_reference_point() {
        let setup = SimSetupState::new();
        let mut persisted = serde_json::to_value(&setup).expect("setup serializes");
        persisted
            .as_object_mut()
            .expect("setup is an object")
            .remove("run_set");

        let migrated: SimSetupState =
            serde_json::from_value(persisted).expect("legacy setup migrates");

        assert_eq!(migrated.run_set.point_count(), 1);
        assert!(migrated.run_set.enabled_dimensions().next().is_none());
    }

    #[test]
    fn legacy_save_policy_migrates_to_automatic_output_selection() {
        let policy = SimulationSavePolicy::default();
        let mut persisted = serde_json::to_value(policy).expect("policy serializes");
        persisted
            .as_object_mut()
            .expect("policy is an object")
            .remove("output_selection_mode");

        let migrated: SimulationSavePolicy =
            serde_json::from_value(persisted).expect("legacy policy migrates");
        assert_eq!(
            migrated.output_selection_mode,
            crate::state::OutputSelectionMode::Automatic
        );
    }
}
