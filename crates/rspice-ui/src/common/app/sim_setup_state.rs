//! Simulation setup — the typed analysis-configuration model behind the
//! Simulate view.
//!
//! Every analysis owns exactly one state struct: draft string buffers plus
//! a `to_config()` parse/validate step. The Simulate view edits these
//! structs and the controller consumes the very same structs when it
//! builds the run plan, so what you see is what runs. `enabled` is the run
//! set and `analysis_order` is its stable execution order.

use std::collections::HashSet;

/// The nominal/reference operating point selected in the workbench chrome.
///
/// This is execution state, not display state: temperature is copied into the
/// effective solver options and process is used when resolving model-library
/// sections for a run.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferencePvtPoint {
    pub process: crate::simulation::dialog::corner::ProcessCorner,
    pub temperature_celsius: f64,
}

impl Default for ReferencePvtPoint {
    fn default() -> Self {
        Self {
            process: crate::simulation::dialog::corner::ProcessCorner::TT,
            temperature_celsius: 27.0,
        }
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
    /// Draft buffers for the options dialog.
    #[serde(skip)]
    pub options_draft: crate::simulation::dialog::OptionsDialogState,
    /// Parse/validation errors from the last options apply.
    #[serde(skip)]
    pub options_errors: Vec<String>,
    /// Options dialog open.
    #[serde(skip)]
    pub options_open: bool,
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
            ..Self::default()
        };
        setup
            .set_reference_pvt(crate::simulation::dialog::corner::ProcessCorner::TT, 27.0)
            .expect("the built-in reference PVT point is valid");
        setup
            .enabled
            .insert(crate::common::simulation_analysis_tabs::TAB_TRANSIENT);
        setup
            .analysis_order
            .push(crate::common::simulation_analysis_tabs::TAB_TRANSIENT);
        setup
    }

    /// Return the exact deterministic execution order for the current run
    /// set. Persisted order wins; newly enabled analyses are appended in
    /// numeric order so legacy and direct-toggle callers remain deterministic.
    pub fn ordered_enabled_indices(&self) -> Vec<usize> {
        let mut ordered = Vec::with_capacity(self.enabled.len());
        let mut seen = HashSet::with_capacity(self.enabled.len());
        for index in &self.analysis_order {
            if *index < crate::common::simulation_analysis_tabs::ANALYSIS_COUNT
                && self.enabled.contains(index)
                && seen.insert(*index)
            {
                ordered.push(*index);
            }
        }
        let mut missing: Vec<_> = self
            .enabled
            .iter()
            .copied()
            .filter(|index| !seen.contains(index))
            .collect();
        missing.sort_unstable();
        ordered.extend(missing);
        ordered
    }

    /// Normalize execution-order storage to the enabled supported analyses.
    pub fn normalize_analysis_order(&mut self) {
        self.analysis_order = self.ordered_enabled_indices();
    }

    /// Rebuild transient editing state after a persisted plan is restored.
    pub(crate) fn prepare_after_restore(&mut self) {
        if let Some(plan) = &mut self.analysis_plan {
            plan.prepare_after_restore();
        }
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
        self.xf.initialized = true;
        self.corner.initialized = true;
        self.envelope.initialized = true;
        self.fourier.initialized = true;
        self.reliability.initialized = true;
        self.optimization.initialized = true;
        self.soa.initialized = true;
        self.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&self.options);
        self.options_errors.clear();
        self.options_open = false;
        self.palette_open = false;
        self.palette_query.clear();
        self.palette_active = 0;
        self.palette_scroll_to_active = false;
        self.refresh_legacy_analysis_projections();
    }

    /// Select the nominal/reference PVT point consumed by subsequent runs.
    pub fn set_reference_pvt(
        &mut self,
        process: crate::simulation::dialog::corner::ProcessCorner,
        temperature_celsius: f64,
    ) -> Result<(), String> {
        if !temperature_celsius.is_finite() {
            return Err("Reference temperature must be finite".to_owned());
        }
        if temperature_celsius < -273.15 {
            return Err("Reference temperature cannot be below absolute zero".to_owned());
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

    /// Synchronize a valid temperature edited in the OP analysis form without
    /// replacing its in-progress text buffer.
    pub fn apply_reference_temperature_from_op(&mut self) -> Result<(), String> {
        let temperature_celsius = self
            .op
            .temperature
            .parse::<f64>()
            .map_err(|_| "Invalid temperature".to_owned())?;
        if !temperature_celsius.is_finite() {
            return Err("Reference temperature must be finite".to_owned());
        }
        if temperature_celsius < -273.15 {
            return Err("Reference temperature cannot be below absolute zero".to_owned());
        }
        self.reference_pvt.temperature_celsius = temperature_celsius;
        self.options.temp = temperature_celsius;
        self.options_draft.temp = format_temperature(temperature_celsius);
        Ok(())
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
                "{} <- {} · {} -> {}",
                self.xf.output_node, self.xf.input_source, self.xf.start_freq, self.xf.stop_freq
            ),
            18 => {
                let corner = &self.corner;
                let flags = [
                    (corner.process_tt, "TT"),
                    (corner.process_ss, "SS"),
                    (corner.process_ff, "FF"),
                    (corner.process_sf, "SF"),
                    (corner.process_fs, "FS"),
                ];
                let on: Vec<&str> = flags.iter().filter(|(f, _)| *f).map(|(_, n)| *n).collect();
                if on.is_empty() {
                    "no corners selected".to_owned()
                } else {
                    on.join(" ")
                }
            }
            19 => format!(
                "f0 {} · to {}",
                self.envelope.fundamental, self.envelope.stop_time
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
            18 => self.corner.to_config().err(),
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
                if !ratio.is_empty()
                    && !ratio.eq_ignore_ascii_case("auto")
                    && let Err(e) = parse(ratio)
                {
                    return field("f2/f1 ratio", e);
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

    /// The SPICE card this analysis writes into the deck, exactly as the
    /// controller will emit it — or the first validation error. Drives the
    /// "Writes" strip in the add-analysis dialog.
    pub fn spice_preview(&self, index: usize) -> Result<String, String> {
        if let Some(error) = self.validation_error(index) {
            return Err(error);
        }
        let sweep_kind = ["dec", "oct", "lin"][self.ac.sweep.min(2)];
        Ok(match index {
            0 => self
                .op
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            1 => {
                let start = self.tran.start.trim();
                let mut card = format!(
                    ".tran {} {} {}",
                    self.tran.step,
                    self.tran.stop,
                    if start.is_empty() { "0" } else { start }
                );
                let max = self.tran.max_step.trim();
                if !max.is_empty() && !max.eq_ignore_ascii_case("auto") {
                    card.push(' ');
                    card.push_str(max);
                }
                if self.tran.uic {
                    card.push_str(" uic");
                }
                card
            }
            2 => format!(
                ".ac {sweep_kind} {} {} {}",
                self.ac.points.trim(),
                self.ac.fstart,
                self.ac.fstop
            ),
            3 => {
                let mut card = format!(
                    ".dc {} {} {} {}",
                    self.dc.source, self.dc.start, self.dc.stop, self.dc.step
                );
                if self.dc.nested {
                    card.push_str(&format!(
                        " {} {} {} {}",
                        self.dc.source2, self.dc.start2, self.dc.stop2, self.dc.step2
                    ));
                }
                card
            }
            4 => {
                let reference = self.noise.reference.trim();
                let output = if reference.is_empty() || reference == "0" {
                    format!("v({})", self.noise.output)
                } else {
                    format!("v({},{})", self.noise.output, reference)
                };
                format!(
                    ".noise {output} {} dec {} {} {}",
                    self.noise.input,
                    self.ac.points.trim(),
                    self.noise.fstart,
                    self.noise.fstop
                )
            }
            5 => self
                .pz
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            6 => self
                .sens
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            7 => self
                .mc
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            8 => self
                .pss
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            9 => self
                .stb
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            10 => self
                .temp
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            11 => self
                .hb
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            12 => self
                .sp
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            13 => self
                .pac
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            14 => self
                .pnoise
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            15 => self
                .pxf
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            16 => self
                .pstb
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            17 => self
                .xf
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            18 => {
                // Corner is a run plan, not a single card — preview the plan.
                let corner = &self.corner;
                let flags = [
                    (corner.process_tt, "TT"),
                    (corner.process_ss, "SS"),
                    (corner.process_ff, "FF"),
                    (corner.process_sf, "SF"),
                    (corner.process_fs, "FS"),
                ];
                let on: Vec<&str> = flags.iter().filter(|(f, _)| *f).map(|(_, n)| *n).collect();
                format!("* corner plan · {}", on.join(" "))
            }
            19 => self
                .envelope
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            20 => self
                .fourier
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            21 => self
                .reliability
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            22 => self
                .optimization
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            23 => self
                .soa
                .to_config()
                .map(|c| c.to_spice())
                .unwrap_or_default(),
            24 => {
                let mut card = format!(
                    ".disto {sweep_kind} {} {} {}",
                    self.ac.points.trim(),
                    self.ac.fstart,
                    self.ac.fstop
                );
                let ratio = self.disto_f2_over_f1.trim();
                if !ratio.is_empty() && !ratio.eq_ignore_ascii_case("auto") {
                    card.push_str(&format!(" f2overf1={ratio}"));
                }
                card
            }
            _ => String::new(),
        })
    }

    /// Fill defaults into any analysis state still at its blank
    /// `Default` (each guards on its own `initialized` flag).
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
    use crate::simulation::dialog::corner::ProcessCorner;

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
            .set_reference_pvt(ProcessCorner::SS, -274.0)
            .expect_err("temperature below absolute zero must fail");

        assert!(error.contains("absolute zero"));
        assert_eq!(setup.reference_pvt, before);
        assert_eq!(setup.options.temp, before.temperature_celsius);
    }
}
