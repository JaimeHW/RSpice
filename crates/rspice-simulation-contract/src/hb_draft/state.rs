//! Harmonic balance tone dialog state.

use super::format::format_freq;
use super::{HbConfig, HbSolverType, HbToneConfig};
use crate::options::parse_si_value;

/// Dialog state with string buffers for SI-prefix input
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HbToneDialogState {
    /// Tone frequency buffer.
    pub frequency: String,
    /// Harmonics buffer.
    pub harmonics: String,
    /// Optional name.
    pub name: String,
    /// Optional source routing.
    pub source: String,
}

impl HbToneDialogState {
    fn from_tone_config(tone: &HbToneConfig) -> Self {
        Self {
            frequency: format_freq(tone.frequency),
            harmonics: tone.harmonics.to_string(),
            name: tone.name.clone(),
            source: tone.source.clone().unwrap_or_default(),
        }
    }
}

/// Dialog state with string buffers for SI-prefix input
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HbDialogState {
    /// Fundamental frequency buffer
    pub fundamental: String,
    /// Number of harmonics buffer
    pub harmonics: String,
    /// Retired: no control ever wrote this, and the primary tone's label is
    /// the one the specification builder gives it. Accepted so a project
    /// saved while the field existed still opens, then dropped.
    #[serde(default, rename = "fundamental_name", skip_serializing)]
    _fundamental_name: Option<serde::de::IgnoredAny>,
    /// Primary tone source buffer
    pub fundamental_source: String,
    /// Oversample factor buffer
    pub oversample: String,
    /// Maximum intermodulation order buffer.
    #[serde(default = "default_max_mixing_order")]
    pub max_mixing_order: String,
    /// Relative tolerance buffer
    pub reltol: String,
    /// Absolute residual tolerance buffer.
    #[serde(default = "default_abstol")]
    pub abstol: String,
    /// Max iterations buffer
    pub maxiter: String,
    /// Damping factor buffer
    pub damping: String,
    /// Line-search step floor buffer.
    #[serde(default = "default_min_damping")]
    pub min_damping: String,
    /// GMRES restart buffer
    pub gmres_restart: String,
    /// Exact collocation-grid buffer; empty asks for the automatic grid.
    #[serde(default)]
    pub collocation_points: String,
    /// Solver type index
    pub solver_idx: usize,
    /// Source stepping enabled
    pub source_stepping: bool,
    /// Exact real-split Jacobian enabled.
    #[serde(default = "default_use_exact_jacobian")]
    pub use_exact_jacobian: bool,
    /// Whether the solve traces its Newton iterations to the Console.
    ///
    /// Defaulted on read: a draft written before the Console could show that
    /// trace ran without it, which is exactly what `false` means.
    #[serde(default)]
    pub verbose: bool,
    /// Additional tone rows.
    pub additional_tones: Vec<HbToneDialogState>,
    /// Initialized flag
    #[serde(skip)]
    pub initialized: bool,
}

/// What a draft written before the solver controls existed opens with.
///
/// Each one is the value the engine used for that field while no control
/// reached it, so a saved project keeps the run it described rather than
/// silently acquiring the empty string's refusal. The struct carries no
/// `#[serde(default)]` of its own, so a key with no attribute here is one a
/// stored draft is required to hold.
fn default_max_mixing_order() -> String {
    HbConfig::default().max_mixing_order.to_string()
}

fn default_abstol() -> String {
    format_tolerance(HbConfig::default().abstol)
}

fn default_min_damping() -> String {
    HbConfig::default().min_damping.to_string()
}

fn default_use_exact_jacobian() -> bool {
    HbConfig::default().use_exact_jacobian
}

/// Tolerances are authored as one significant digit and an exponent, which is
/// how every tolerance this studio shows is written.
fn format_tolerance(value: f64) -> String {
    format!("{value:.0e}")
}

impl HbDialogState {
    /// Initialize from config
    pub fn from_config(config: &HbConfig) -> Self {
        Self {
            fundamental: format_freq(config.fundamental_freq),
            harmonics: config.num_harmonics.to_string(),
            _fundamental_name: None,
            fundamental_source: config.fundamental_source.clone().unwrap_or_default(),
            oversample: config.oversample.to_string(),
            max_mixing_order: config.max_mixing_order.to_string(),
            reltol: format_tolerance(config.reltol),
            abstol: format_tolerance(config.abstol),
            maxiter: config.maxiter.to_string(),
            damping: config.damping.to_string(),
            min_damping: config.min_damping.to_string(),
            gmres_restart: config.gmres_restart.to_string(),
            collocation_points: config
                .collocation_points
                .map(|points| points.to_string())
                .unwrap_or_default(),
            solver_idx: match config.solver {
                HbSolverType::Newton => 0,
                HbSolverType::Krylov => 1,
            },
            source_stepping: config.source_stepping,
            use_exact_jacobian: config.use_exact_jacobian,
            verbose: config.verbose,
            additional_tones: config
                .additional_tones
                .iter()
                .map(HbToneDialogState::from_tone_config)
                .collect(),
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<HbConfig, String> {
        let fundamental = parse_si_value(&self.fundamental)
            .map_err(|e| format!("Invalid fundamental frequency: {}", e))?;

        let harmonics: u32 = self
            .harmonics
            .parse()
            .map_err(|_| "Invalid harmonics count")?;

        let oversample: u32 = self
            .oversample
            .parse()
            .map_err(|_| "Invalid oversample factor")?;

        let max_mixing_order: u32 = self
            .max_mixing_order
            .parse()
            .map_err(|_| "Invalid mixing order")?;

        let reltol =
            parse_si_value(&self.reltol).map_err(|e| format!("Invalid tolerance: {}", e))?;

        let abstol = parse_si_value(&self.abstol)
            .map_err(|e| format!("Invalid absolute tolerance: {}", e))?;

        let maxiter: u32 = self.maxiter.parse().map_err(|_| "Invalid max iterations")?;

        let damping: f64 = self.damping.parse().map_err(|_| "Invalid damping factor")?;

        let min_damping: f64 = self
            .min_damping
            .parse()
            .map_err(|_| "Invalid damping floor")?;

        let solver = match self.solver_idx {
            0 => HbSolverType::Newton,
            _ => HbSolverType::Krylov,
        };

        let gmres_restart: u32 = self
            .gmres_restart
            .parse()
            .map_err(|_| "Invalid GMRES restart")?;

        // Empty is the automatic grid, which is a configuration and not an
        // omission: the solver picks its own oversampled power-of-two FFT
        // size, and an authored count pins the minimal odd grid instead.
        let collocation_points = match self.collocation_points.trim() {
            "" => None,
            text => Some(text.parse().map_err(|_| "Invalid collocation points")?),
        };

        let mut config = HbConfig {
            fundamental_freq: fundamental,
            num_harmonics: harmonics,
            fundamental_source: if self.fundamental_source.trim().is_empty() {
                None
            } else {
                Some(self.fundamental_source.trim().to_string())
            },
            additional_tones: Vec::new(),
            oversample,
            max_mixing_order,
            reltol,
            abstol,
            maxiter,
            damping,
            min_damping,
            collocation_points,
            solver,
            gmres_restart,
            source_stepping: self.source_stepping,
            use_exact_jacobian: self.use_exact_jacobian,
            verbose: self.verbose,
        };

        for (idx, tone) in self.additional_tones.iter().enumerate() {
            let is_empty = tone.frequency.trim().is_empty()
                && tone.harmonics.trim().is_empty()
                && tone.name.trim().is_empty()
                && tone.source.trim().is_empty();
            if is_empty {
                continue;
            }

            let freq = parse_si_value(&tone.frequency)
                .map_err(|e| format!("Invalid tone {} frequency: {}", idx + 2, e))?;
            let harm: u32 = tone
                .harmonics
                .parse()
                .map_err(|_| format!("Invalid tone {} harmonics", idx + 2))?;

            let mut tone_cfg =
                HbToneConfig::new(freq, harm).with_name(if tone.name.trim().is_empty() {
                    format!("tone{}", idx + 2)
                } else {
                    tone.name.trim().to_string()
                });
            if !tone.source.trim().is_empty() {
                tone_cfg = tone_cfg.with_source(tone.source.trim().to_string());
            }
            config.additional_tones.push(tone_cfg);
        }

        config.validate()?;
        Ok(config)
    }

    /// Initialize defaults if not already
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&HbConfig::default());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A draft saved while the primary tone carried a name still opens.
    ///
    /// The key is accepted and dropped: no control ever wrote it, and the
    /// specification builder labels the primary tone itself, so restoring the
    /// value would put a name into the digest that nothing can author.
    #[test]
    fn a_saved_draft_that_carries_a_fundamental_name_still_opens() {
        let mut saved = serde_json::to_value(HbDialogState::from_config(&HbConfig::default()))
            .expect("the draft serializes");
        saved
            .as_object_mut()
            .expect("the draft is an object")
            .insert("fundamental_name".to_owned(), "mytone".into());

        let restored: HbDialogState =
            serde_json::from_value(saved).expect("a draft carrying the retired key still opens");
        restored.to_config().expect("the restored draft configures");

        let written = serde_json::to_value(&restored).expect("the draft serializes");
        assert!(
            !written
                .as_object()
                .expect("the draft is an object")
                .contains_key("fundamental_name"),
            "the retired key is never written again: {written}"
        );
    }

    /// A project saved before the solver controls existed still opens, and
    /// opens on the run it described.
    ///
    /// The draft carries no struct-level `#[serde(default)]`, so each new key
    /// needs its own default and each default has to be the value the engine
    /// used while nothing authored the field. Anything else would silently
    /// re-solve a saved design at a different mixing order or a different
    /// Jacobian.
    #[test]
    fn an_hb_draft_written_before_the_solver_controls_opens_with_the_engine_defaults() {
        let mut saved = serde_json::to_value(HbDialogState::from_config(&HbConfig::default()))
            .expect("the draft serializes");
        let object = saved.as_object_mut().expect("the draft is an object");
        for key in [
            "max_mixing_order",
            "abstol",
            "min_damping",
            "collocation_points",
            "use_exact_jacobian",
            "verbose",
        ] {
            assert!(object.remove(key).is_some(), "{key} is a draft key");
        }

        let restored: HbDialogState =
            serde_json::from_value(saved).expect("a draft without the solver controls opens");
        let config = restored.to_config().expect("the restored draft configures");
        let engine = HbConfig::default();
        assert_eq!(config.max_mixing_order, engine.max_mixing_order);
        assert_eq!(config.abstol, engine.abstol);
        assert_eq!(config.min_damping, engine.min_damping);
        assert_eq!(config.collocation_points, None);
        assert_eq!(config.use_exact_jacobian, engine.use_exact_jacobian);
        assert!(
            !config.verbose,
            "a draft that never asked for a trace gets none"
        );
    }

    /// Every control the form paints reaches the configuration it authors.
    ///
    /// Buffer-to-field, with a distinct value per field, so a transposition
    /// between two fields of the same type is visible. The draft's own
    /// `to_config` is the only place these strings become numbers.
    #[test]
    fn every_hb_solver_buffer_configures_its_own_field() {
        let mut state = HbDialogState::from_config(&HbConfig::default());
        state.max_mixing_order = "7".to_owned();
        state.reltol = "1e-7".to_owned();
        state.abstol = "1e-11".to_owned();
        state.damping = "0.5".to_owned();
        state.min_damping = "0.25".to_owned();
        state.gmres_restart = "16".to_owned();
        state.collocation_points = "37".to_owned();
        state.use_exact_jacobian = false;
        state.verbose = true;

        let config = state.to_config().expect("the authored draft configures");
        assert_eq!(config.max_mixing_order, 7);
        assert_eq!(config.reltol, 1e-7);
        assert_eq!(config.abstol, 1e-11);
        assert_eq!(config.damping, 0.5);
        assert_eq!(config.min_damping, 0.25);
        assert_eq!(config.gmres_restart, 16);
        assert_eq!(config.collocation_points, Some(37));
        assert!(!config.use_exact_jacobian);
        assert!(
            config.verbose,
            "the solver trace the form asked for reaches the configuration"
        );
    }

    /// The solver trace survives a save and an open.
    ///
    /// It is the one control on this form whose whole product is text in the
    /// Console, so a draft that forgot it would be a run that silently stopped
    /// explaining itself.
    #[test]
    fn a_saved_hb_draft_restores_the_solver_trace_it_asked_for() {
        let mut state = HbDialogState::from_config(&HbConfig::default());
        state.verbose = true;

        let saved = serde_json::to_string(&state).expect("the draft serializes");
        let restored: HbDialogState = serde_json::from_str(&saved).expect("the saved draft opens");
        assert!(restored.verbose);
        assert!(
            restored
                .to_config()
                .expect("the restored draft configures")
                .verbose
        );
    }

    /// The floor is bounded by the factor it is a floor for, and the factor by
    /// the domain the engine's builder honours.
    #[test]
    fn the_hb_damping_domain_is_the_one_the_engine_honours() {
        let mut state = HbDialogState::from_config(&HbConfig::default());
        state.damping = "0.05".to_owned();
        let error = state.to_config().expect_err("0.05 is below the domain");
        assert!(error.contains("between 0.1 and 1"), "{error}");

        state.damping = "0.2".to_owned();
        state.min_damping = "0.5".to_owned();
        let error = state
            .to_config()
            .expect_err("a floor above the factor is not a floor");
        assert!(error.contains("Damping floor"), "{error}");
    }
}
