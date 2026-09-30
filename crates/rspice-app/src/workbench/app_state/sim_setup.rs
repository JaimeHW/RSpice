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

use rspice_simulation_contract::run_set::ReferencePoint;

/// The nominal/reference operating point selected in the workbench chrome.
///
/// This is execution state, not display state: temperature is copied into the
/// effective solver options and process is used when resolving model-library
/// sections for a run. It is also what an axis the run set does not declare
/// resolves to, so it is one type rather than two that have to agree.
pub type ReferencePvtPoint = ReferencePoint;

pub use rspice_simulation_contract::output_policy::SimulationSavePolicy;

pub use rspice_simulation_contract::drafts::{AcSetup, DcSetup, TranSetup};

/// Editor state that belongs to the current workbench session, never to a
/// persisted simulation plan.
#[derive(Debug, Clone, Default)]
pub struct SimSetupEditorSession {
    /// Draft buffers edited before effective solver options are committed.
    pub options_draft: crate::simulation::dialog::OptionsDialogState,
    /// Add-analysis palette visibility and keyboard interaction state.
    pub palette_open: bool,
    pub palette_query: String,
    pub palette_active: usize,
    pub palette_scroll_to_active: bool,
}

/// Workbench setup: portable simulation state plus transient editor state.
#[derive(Debug, Clone, Default)]
pub struct SimSetupState {
    pub(crate) setup: rspice_simulation_contract::setup_state::SimulationSetup,
    pub session: SimSetupEditorSession,
}

impl std::ops::Deref for SimSetupState {
    type Target = rspice_simulation_contract::setup_state::SimulationSetup;

    fn deref(&self) -> &Self::Target {
        &self.setup
    }
}

impl std::ops::DerefMut for SimSetupState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.setup
    }
}

impl serde::Serialize for SimSetupState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&self.setup, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for SimSetupState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            setup: serde::Deserialize::deserialize(deserializer)?,
            session: SimSetupEditorSession::default(),
        })
    }
}

impl SimSetupState {
    /// Fresh setup with the conventional default transient run set.
    pub fn new() -> Self {
        let mut setup = Self {
            setup: rspice_simulation_contract::setup_state::SimulationSetup::new(),
            session: SimSetupEditorSession::default(),
        };
        setup.session.options_draft.temp = setup.reference_pvt.temperature_celsius.to_string();
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
        setup.session.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&setup.options);
        setup
    }

    /// Rebuild portable setup state and the current editor session after load.
    pub(crate) fn prepare_after_restore(&mut self) {
        self.setup.prepare_after_restore();
        self.session.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&self.options);
        self.session.palette_open = false;
        self.session.palette_query.clear();
        self.session.palette_active = 0;
        self.session.palette_scroll_to_active = false;
    }

    /// Select nominal PVT and update the session's numerical-options draft.
    pub fn set_reference_pvt(
        &mut self,
        process: crate::product::ProcessCorner,
        temperature_celsius: f64,
    ) -> Result<(), String> {
        self.setup.set_reference_pvt(process, temperature_celsius)?;
        self.session.options_draft.temp = temperature_celsius.to_string();
        Ok(())
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
            // Which ports, not how many rows: this summary cannot see the
            // design, and stating the ad-hoc table's length for an analysis
            // reading the sheet's placed ports would name a count the run
            // never has.
            12 => match self.sp.port_source_idx.and_then(|index| {
                crate::simulation::dialog::SpPortSource::ALL
                    .get(index)
                    .copied()
            }) {
                Some(crate::simulation::dialog::SpPortSource::Placed) => {
                    format!("placed RF ports · Z0 {}", self.sp.z0)
                }
                _ => format!("{} ports · Z0 {}", self.sp.ports.len(), self.sp.z0),
            },
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
                let run_set = &self.run_set;
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
            21 => format!(
                "{} {} · {}",
                ["minimize", "maximize", "target"][self.optimization.goal_mode.min(2)],
                self.optimization.objective_node,
                ["gradient", "pattern", "anneal"][self.optimization.algorithm.min(2)]
            ),
            22 => {
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
            23 => {
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
        use crate::quantity::spice_value::parse_spice_value_checked as parse;
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
            10 => self.temp.to_config(&self.run_set, self.reference_pvt).err(),
            11 => self.hb.to_config().err(),
            // Design-blind, and it says so: this state holds the simulation
            // setup and not the sheet, so it checks the sweep, the impedance
            // and the ad-hoc table it owns, and leaves the placed-port roster
            // to the dispatching path that can read the design.
            12 => self.sp.to_config(None).err(),
            13 => self.pac.to_config().err(),
            14 => self.pnoise.to_config().err(),
            15 => self.pxf.to_config().err(),
            16 => self.pstb.to_config().err(),
            17 => self.xf.to_config().err(),
            18 => self
                .corner
                .to_config(&self.run_set, self.reference_pvt)
                .err(),
            19 => self.envelope.to_config().err(),
            20 => self.fourier.to_config().err(),
            21 => self.optimization.to_config().err(),
            22 => self.soa.to_config().err(),
            23 => {
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
        use crate::quantity::spice_value::parse_spice_value_checked as parse;
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
        self.optimization.ensure_initialized();
        self.soa.ensure_initialized();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProcessCorner;

    /// A project saved before retracing existed still opens, as the one-way
    /// sweep it was authored as.
    ///
    /// `DcSetup` denies unknown fields, so the guard that matters is the other
    /// direction: a *missing* field must default rather than fail the load. An
    /// engineer who upgrades must not find their saved plans unreadable, and
    /// `false` is not a guess here — every sweep authored before this went one
    /// way, which is exactly what it means.
    #[test]
    fn a_dc_draft_saved_before_retracing_existed_opens_as_a_one_way_sweep() {
        let saved = r#"{
            "source": "VIN",
            "start": "0",
            "stop": "5",
            "step": "0.01",
            "nested": false,
            "source2": "V2",
            "start2": "0",
            "stop2": "3.3",
            "step2": "0.1"
        }"#;

        let restored: DcSetup = serde_json::from_str(saved).expect("an older DC draft loads");

        assert!(!restored.hysteresis);
        assert_eq!(restored.source, "VIN");
        assert_eq!(restored.stop, "5");
    }

    #[test]
    fn solver_options_reference_pvt_preserves_full_precision() {
        let mut setup = SimSetupState::new();
        for temperature in [27.123_456_789, -40.123_456_789, 27.0_f64.next_up()] {
            setup
                .set_reference_pvt(ProcessCorner::FF, temperature)
                .unwrap();
            assert_eq!(
                setup.reference_pvt.temperature_celsius.to_bits(),
                temperature.to_bits()
            );
            assert_eq!(setup.options.temp.to_bits(), temperature.to_bits());
            assert_eq!(
                setup
                    .session
                    .options_draft
                    .temp
                    .parse::<f64>()
                    .unwrap()
                    .to_bits(),
                temperature.to_bits()
            );
            assert_eq!(
                setup.op.temperature.parse::<f64>().unwrap().to_bits(),
                temperature.to_bits()
            );
        }
    }

    #[test]
    fn reference_pvt_is_the_temperature_consumed_by_solver_and_op() {
        let mut setup = SimSetupState::new();

        setup
            .set_reference_pvt(ProcessCorner::FF, -40.0)
            .expect("reference point is valid");

        assert_eq!(setup.reference_pvt.process, ProcessCorner::FF);
        assert_eq!(setup.reference_pvt.temperature_celsius, -40.0);
        assert_eq!(setup.options.temp, -40.0);
        assert_eq!(setup.session.options_draft.temp, "-40");
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
    fn setup_document_keeps_flat_project_wire_and_legacy_read_behavior() {
        let mut setup = SimSetupState::new();
        setup.session.palette_open = true;
        setup.tran.stop = "7m".to_owned();

        let mut wire = serde_json::to_value(&setup).expect("setup serializes");
        let fields = wire.as_object_mut().expect("setup is a flat object");
        assert!(fields.contains_key("analysis_plan"));
        assert!(fields.contains_key("run_set"));
        assert!(fields.contains_key("options"));
        assert!(!fields.contains_key("document"));
        assert!(!fields.contains_key("session"));
        assert!(!fields.contains_key("tran"));

        fields.insert(
            "tran".to_owned(),
            serde_json::to_value(&setup.tran).unwrap(),
        );
        let restored: SimSetupState =
            serde_json::from_value(wire.clone()).expect("schema-3 draft remains readable");
        assert_eq!(restored.tran.stop, "7m");
        assert!(!restored.session.palette_open);
        assert!(
            !serde_json::to_value(&restored)
                .unwrap()
                .as_object()
                .unwrap()
                .contains_key("tran")
        );

        wire.as_object_mut()
            .unwrap()
            .insert("unexpected".to_owned(), serde_json::json!(true));
        assert!(serde_json::from_value::<SimSetupState>(wire).is_err());
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
