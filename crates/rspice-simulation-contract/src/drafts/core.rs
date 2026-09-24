//! Core persisted simulator setup drafts.

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
    /// Sweep out to the stop value and back again as one continued solve.
    ///
    /// Defaulted on read so a project saved before retracing existed opens as
    /// the one-way sweep it was authored as, rather than failing to load.
    #[serde(default)]
    pub hysteresis: bool,
    /// 0 = linear, 1 = ordered list, 2 = decade, 3 = octave.
    #[serde(default)]
    pub mode: usize,
    #[serde(default)]
    pub values: String,
    #[serde(default = "default_dc_points")]
    pub points: String,
    #[serde(default)]
    pub mode2: usize,
    #[serde(default)]
    pub values2: String,
    #[serde(default = "default_dc_points")]
    pub points2: String,
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
            hysteresis: false,
            mode: 0,
            values: "0 1 2 3 4 5".into(),
            points: default_dc_points(),
            mode2: 0,
            values2: "0 1 2 3.3".into(),
            points2: default_dc_points(),
        }
    }
}

fn default_dc_points() -> String {
    "10".into()
}

impl DcSetup {
    pub fn to_config(&self) -> Result<crate::config::DcSweepConfig, String> {
        use crate::config::{DcAxisMode, DcSweepConfig, DcSweepModes};
        use crate::spice_value::parse_spice_value_checked as parse;
        let primary = DcAxisMode::from_draft(self.mode, &self.values, &self.points)?;
        let axis = |mode: &DcAxisMode, start: &str, stop: &str, step: &str| {
            if let DcAxisMode::List { values } = mode {
                Ok((values[0], *values.last().unwrap(), 0.0))
            } else {
                let step = if matches!(mode, DcAxisMode::Linear) {
                    parse(step)?
                } else {
                    0.0
                };
                Ok::<_, String>((parse(start)?, parse(stop)?, step))
            }
        };
        let (start, stop, step) = axis(&primary, &self.start, &self.stop, &self.step)?;
        let mut config = DcSweepConfig {
            source: self.source.trim().into(),
            start,
            stop,
            step,
            hysteresis: self.hysteresis,
            modes: DcSweepModes {
                primary,
                secondary: DcAxisMode::Linear,
            },
            ..Default::default()
        };
        if self.nested {
            config.modes.secondary =
                DcAxisMode::from_draft(self.mode2, &self.values2, &self.points2)?;
            let (start, stop, step) = axis(
                &config.modes.secondary,
                &self.start2,
                &self.stop2,
                &self.step2,
            )?;
            config.source2 = Some(self.source2.trim().into());
            config.start2 = Some(start);
            config.stop2 = Some(stop);
            config.step2 = Some(step);
        }
        config.validate().map_err(|errors| errors.join("; "))?;
        Ok(config)
    }
}

/// DISTO draft with its own AC sweep and optional second-tone ratio.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DistoDraft {
    pub sweep: AcSetup,
    /// Empty or `auto` selects single-tone harmonic distortion.
    pub f2_over_f1: String,
}
