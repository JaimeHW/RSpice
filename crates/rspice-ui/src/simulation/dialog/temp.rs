//! Temperature Sweep Analysis Configuration
//!
//! Configuration for temperature sweep analysis.
//! Runs a base analysis across a range of temperatures.
//!
//! The dimension is authored either as a range or as an explicit list. The
//! list is the user's, not a fixed preset: a design qualified at four named
//! temperatures is qualified at the four its own programme names, and a
//! sweep that can only offer -40/25/85/125 forces the reader to fake the
//! rest with a range that hits them by accident.

use serde::{Deserialize, Deserializer};

use crate::simulation::run_set::{ReferencePoint, RunSetState};

/// Where a temperature sweep's temperatures come from.
///
/// The plan may declare a temperature axis on PVT, sweeps & variation. This
/// instance either reads that declaration — so a qualification programme's
/// temperatures are authored once and cannot drift from the sweep that runs
/// them — or states its own, which is what this analysis has always done.
///
/// [`Self::Explicit`] is the default, and must stay so: a project saved before
/// this switch existed authored a range, and loading it into inherit would run
/// a sweep nobody asked for.
///
/// # What inheriting does and does not change
///
/// It removes *drift*: the temperatures are authored once, on the run-set page,
/// and this instance expands exactly those. It does not change *who expands
/// them* — an inheriting instance still walks the list itself, exactly as an
/// explicit one walks its range.
///
/// So neither mode makes this analysis composable with an enabled global
/// multi-point Run Set. Both own an expansion, and two expansions over one run
/// is the ambiguity `snapshot::participation` refuses by name. The case
/// inheriting is for is the other one, and it is common: the axis is declared
/// so the whole plan *can* be crossed by it, but is left disabled, and this one
/// analysis executes it without keeping a second copy of the numbers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TempAxisMode {
    /// The Start/Stop/Step range, or the explicit list, held on this form.
    #[default]
    Explicit,
    /// The temperatures the plan's run-set axis declares.
    InheritRunSetAxis,
}

impl TempAxisMode {
    pub const ALL: [Self; 2] = [Self::Explicit, Self::InheritRunSetAxis];

    /// Exact Simulation Studio choice label.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Explicit => "Explicit temperatures",
            Self::InheritRunSetAxis => "Inherit run-set axis",
        }
    }
}

/// Base analysis for temperature sweep
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TempBaseAnalysis {
    #[default]
    Op,
    Transient,
    Ac,
    Dc,
}

/// Temperature sweep configuration
#[derive(Debug, Clone)]
pub struct TempConfig {
    /// Start temperature (Celsius)
    pub temp_start: f64,
    /// Stop temperature (Celsius)
    pub temp_stop: f64,
    /// Temperature step (Celsius)
    pub temp_step: f64,
    /// Base analysis type
    pub base_analysis: TempBaseAnalysis,
    /// Explicit temperatures (Celsius). When non-empty this is the dimension,
    /// and the range is not consulted.
    pub specific_temps: Vec<f64>,
}

impl Default for TempConfig {
    fn default() -> Self {
        Self {
            temp_start: -40.0,
            temp_stop: 125.0,
            temp_step: 25.0,
            base_analysis: TempBaseAnalysis::Op,
            specific_temps: Vec::new(),
        }
    }
}

impl TempConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.temp_start.is_finite()
            || !self.temp_stop.is_finite()
            || !self.temp_step.is_finite()
        {
            return Err("Temperature range values must be finite".into());
        }
        if self.temp_start < -273.15 || self.temp_stop < -273.15 {
            return Err("Temperature cannot be below absolute zero (-273.15°C)".into());
        }
        for temperature in &self.specific_temps {
            if !temperature.is_finite() {
                return Err("Temperature values must be finite".into());
            }
            if *temperature < -273.15 {
                return Err("Temperature cannot be below absolute zero (-273.15°C)".into());
            }
        }
        if self.specific_temps.is_empty() {
            if self.temp_step.abs() < 0.01 {
                return Err("Temperature step too small".into());
            }
            if (self.temp_stop > self.temp_start && self.temp_step < 0.0)
                || (self.temp_stop < self.temp_start && self.temp_step > 0.0)
            {
                return Err("Step sign must match sweep direction".into());
            }
        }
        Ok(())
    }

    pub fn num_temps(&self) -> usize {
        if !self.specific_temps.is_empty() {
            return self.specific_temps.len();
        }
        if self.temp_step.abs() < 0.01 {
            return 1;
        }
        ((self.temp_stop - self.temp_start) / self.temp_step)
            .abs()
            .floor() as usize
            + 1
    }
}

/// The temperatures the retired `corner_temps` preset stood for.
const RETIRED_CORNER_TEMPERATURES: &str = "-40, 25, 85, 125";

/// Stand-ins for a range that an inheriting instance is not running.
///
/// An inherited sweep is an authored list, so the range fields are inert. They
/// still have to hold something that validates, because a range which fails
/// `TempConfig::validate` would refuse a sweep whose temperatures are perfectly
/// good. These are the defaults, and they are never expanded.
const TEMP_RANGE_UNUSED_START: f64 = -40.0;
const TEMP_RANGE_UNUSED_STOP: f64 = 125.0;
const TEMP_RANGE_UNUSED_STEP: f64 = 25.0;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct TempDialogState {
    pub temp_start: String,
    pub temp_stop: String,
    pub temp_step: String,
    pub base_idx: usize,
    /// Explicit temperatures, comma or space separated. Empty means the range
    /// above is the dimension.
    pub specific_temps: String,
    /// Index into [`TempAxisMode::ALL`]. 0 is `Explicit`, which is what a
    /// project without the key loads as.
    pub axis_mode_idx: usize,
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedTempDialogState {
    #[serde(default)]
    temp_start: String,
    #[serde(default)]
    temp_stop: String,
    #[serde(default)]
    temp_step: String,
    #[serde(default)]
    base_idx: usize,
    #[serde(default)]
    specific_temps: Option<String>,
    /// Absent in every project saved before the switch existed, and absent
    /// means `Explicit` — the mode those projects were authored in.
    #[serde(default)]
    axis_mode_idx: usize,
    /// Retired preset for the four qualification temperatures. A project that
    /// had it on migrates to those exact values as an authored list, so the
    /// dimension it ran on is preserved rather than silently reverting to the
    /// range.
    #[serde(default)]
    corner_temps: Option<bool>,
}

impl<'de> Deserialize<'de> for TempDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedTempDialogState::deserialize(deserializer)?;
        let specific_temps = persisted.specific_temps.unwrap_or_default();
        let specific_temps =
            if specific_temps.trim().is_empty() && persisted.corner_temps == Some(true) {
                RETIRED_CORNER_TEMPERATURES.to_owned()
            } else {
                specific_temps
            };
        Ok(Self {
            temp_start: persisted.temp_start,
            temp_stop: persisted.temp_stop,
            temp_step: persisted.temp_step,
            base_idx: persisted.base_idx,
            specific_temps,
            axis_mode_idx: persisted.axis_mode_idx.min(TempAxisMode::ALL.len() - 1),
            initialized: false,
        })
    }
}

impl TempDialogState {
    pub fn from_config(config: &TempConfig) -> Self {
        Self {
            temp_start: format!("{}", config.temp_start),
            temp_stop: format!("{}", config.temp_stop),
            temp_step: format!("{}", config.temp_step),
            base_idx: match config.base_analysis {
                TempBaseAnalysis::Op => 0,
                TempBaseAnalysis::Transient => 1,
                TempBaseAnalysis::Ac => 2,
                TempBaseAnalysis::Dc => 3,
            },
            specific_temps: config
                .specific_temps
                .iter()
                .map(|temperature| format!("{temperature}"))
                .collect::<Vec<_>>()
                .join(", "),
            axis_mode_idx: 0,
            initialized: true,
        }
    }

    /// Which of the two axis owners this instance has chosen.
    #[must_use]
    pub fn axis_mode(&self) -> TempAxisMode {
        TempAxisMode::ALL
            .get(self.axis_mode_idx)
            .copied()
            .unwrap_or_default()
    }

    /// The executable sweep, resolved against the plan's declaration.
    ///
    /// In [`TempAxisMode::Explicit`] the range and list on this form are the
    /// dimension, exactly as they always were. In
    /// [`TempAxisMode::InheritRunSetAxis`] they are not read at all: the
    /// temperatures are the plan's, carried through as an authored list so the
    /// executor expands the same values the run-set page shows.
    pub fn to_config(
        &self,
        run_set: &RunSetState,
        reference: ReferencePoint,
    ) -> Result<TempConfig, String> {
        let base = match self.base_idx {
            0 => TempBaseAnalysis::Op,
            1 => TempBaseAnalysis::Transient,
            2 => TempBaseAnalysis::Ac,
            _ => TempBaseAnalysis::Dc,
        };
        let config = match self.axis_mode() {
            TempAxisMode::Explicit => {
                let start: f64 = self.temp_start.parse().map_err(|_| "Invalid start temp")?;
                let stop: f64 = self.temp_stop.parse().map_err(|_| "Invalid stop temp")?;
                let step: f64 = self.temp_step.parse().map_err(|_| "Invalid step")?;
                TempConfig {
                    temp_start: start,
                    temp_stop: stop,
                    temp_step: step,
                    base_analysis: base,
                    specific_temps: parse_temperature_list(&self.specific_temps)?,
                }
            }
            TempAxisMode::InheritRunSetAxis => {
                let temperatures = run_set
                    .declared_temperatures_celsius(reference)
                    .ok_or_else(|| {
                        "The run-set temperature axis has a value that is not a temperature; \
                         correct it in PVT, sweeps & variation or state temperatures here"
                            .to_owned()
                    })?;
                TempConfig {
                    // The range is retained exactly as authored so that
                    // switching back to Explicit restores the sweep the user
                    // wrote, but it is not what runs.
                    temp_start: self.temp_start.parse().unwrap_or(TEMP_RANGE_UNUSED_START),
                    temp_stop: self.temp_stop.parse().unwrap_or(TEMP_RANGE_UNUSED_STOP),
                    temp_step: self.temp_step.parse().unwrap_or(TEMP_RANGE_UNUSED_STEP),
                    base_analysis: base,
                    specific_temps: temperatures,
                }
            }
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&TempConfig::default());
        }
    }
}

/// Parse an authored temperature list.
///
/// Rejects rather than repairs: a value that is not a number is a typo the
/// reader has to see, not a point to quietly drop from a qualification sweep.
fn parse_temperature_list(text: &str) -> Result<Vec<f64>, String> {
    let mut temperatures = Vec::new();
    for token in text
        .split([',', ';', ' ', '\t', '\n'])
        .map(str::trim)
        .filter(|token| !token.is_empty())
    {
        let temperature: f64 = token
            .parse()
            .map_err(|_| format!("'{token}' is not a temperature"))?;
        temperatures.push(temperature);
    }
    Ok(temperatures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::run_set::{RunSetDimension, RunSetDimensionKind};

    fn reference() -> ReferencePoint {
        ReferencePoint::default()
    }

    /// A plan declaring the conventional qualification axis.
    fn plan() -> RunSetState {
        RunSetState::reference_only()
    }

    /// A plan that declares no temperatures at all.
    fn plan_without_a_temperature_axis() -> RunSetState {
        let mut state = RunSetState::reference_only();
        state
            .dimensions
            .retain(|dimension| dimension.kind != RunSetDimensionKind::Temperature);
        state
    }

    fn inheriting(state: &mut TempDialogState) {
        state.axis_mode_idx = TempAxisMode::ALL
            .iter()
            .position(|mode| *mode == TempAxisMode::InheritRunSetAxis)
            .expect("the inherit mode is offered");
    }

    #[test]
    fn an_authored_list_replaces_the_range() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        state.specific_temps = "-40, 27, 125".to_owned();

        let config = state.to_config(&plan(), reference()).expect("list parses");

        assert_eq!(config.specific_temps, vec![-40.0, 27.0, 125.0]);
        assert_eq!(config.num_temps(), 3);
    }

    #[test]
    fn an_empty_list_leaves_the_range_in_charge() {
        let state = TempDialogState::from_config(&TempConfig::default());

        let config = state.to_config(&plan(), reference()).expect("range parses");

        assert!(config.specific_temps.is_empty());
        assert_eq!(config.num_temps(), 7);
    }

    #[test]
    fn a_range_count_matches_the_executor_when_the_stop_is_not_a_step_point() {
        let config = TempConfig {
            temp_start: -40.0,
            temp_stop: 125.0,
            temp_step: 25.0,
            ..TempConfig::default()
        };

        assert_eq!(config.num_temps(), 7);
    }

    #[test]
    fn non_finite_range_values_are_rejected() {
        let mut config = TempConfig::default();
        config.temp_start = f64::NAN;
        assert!(config.validate().unwrap_err().contains("finite"));
    }

    #[test]
    fn a_malformed_value_is_rejected_rather_than_dropped() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        state.specific_temps = "-40, warm, 125".to_owned();

        let error = state
            .to_config(&plan(), reference())
            .expect_err("a typo must not be skipped");

        assert!(error.contains("warm"), "{error}");
    }

    #[test]
    fn the_retired_corner_preset_migrates_to_its_exact_temperatures() {
        let persisted = r#"{
            "temp_start": "-40",
            "temp_stop": "125",
            "temp_step": "25",
            "base_idx": 0,
            "corner_temps": true
        }"#;

        let state: TempDialogState = serde_json::from_str(persisted).expect("legacy state decodes");

        assert_eq!(state.specific_temps, "-40, 25, 85, 125");

        let encoded = serde_json::to_value(&state).expect("state encodes");
        assert!(encoded.get("corner_temps").is_none());
    }

    #[test]
    fn absolute_zero_is_rejected_in_an_authored_list() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        state.specific_temps = "-300".to_owned();

        assert!(state.to_config(&plan(), reference()).is_err());
    }

    #[test]
    fn an_inheriting_sweep_runs_the_temperatures_the_plan_declares() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        inheriting(&mut state);

        let config = state
            .to_config(&plan(), reference())
            .expect("the plan's axis resolves");

        assert_eq!(config.specific_temps, vec![-40.0, 25.0, 125.0]);
        assert_eq!(
            config.num_temps(),
            3,
            "the plan's three temperatures, not the range's seven"
        );
    }

    /// A declared axis the operator has switched off is still the plan's
    /// statement of which temperatures matter. Disabling it says "do not cross
    /// the whole plan by this", which is exactly the case inheriting exists for.
    #[test]
    fn inheriting_reads_a_declared_axis_even_when_it_is_disabled() {
        let mut run_set = plan();
        assert!(
            run_set
                .enabled_dimension_of(RunSetDimensionKind::Temperature)
                .is_none(),
            "the fixture's axis is declared but not enabled"
        );
        let mut state = TempDialogState::from_config(&TempConfig::default());
        inheriting(&mut state);

        let config = state
            .to_config(&run_set, reference())
            .expect("a declared axis resolves whether or not it is enabled");
        assert_eq!(config.specific_temps, vec![-40.0, 25.0, 125.0]);

        // And it tracks the declaration rather than a copy of it.
        for dimension in &mut run_set.dimensions {
            if dimension.kind == RunSetDimensionKind::Temperature {
                dimension.set_values_from_lines("0\n85", 2);
            }
        }
        let config = state
            .to_config(&run_set, reference())
            .expect("the edited axis resolves");
        assert_eq!(
            config.specific_temps,
            vec![0.0, 85.0],
            "an inherited sweep cannot drift from the axis it reads"
        );
    }

    #[test]
    fn inheriting_with_no_declared_axis_runs_at_the_plans_reference() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        inheriting(&mut state);

        let config = state
            .to_config(&plan_without_a_temperature_axis(), reference())
            .expect("a plan with no axis still resolves to one temperature");

        assert_eq!(config.specific_temps, vec![reference().temperature_celsius]);
        assert_eq!(config.num_temps(), 1);
    }

    /// An axis value that is not a temperature refuses rather than being
    /// skipped, for the same reason an authored typo does: a silently shorter
    /// qualification sweep is the failure worth preventing.
    #[test]
    fn inheriting_refuses_an_axis_value_that_is_not_a_temperature() {
        let mut run_set = plan();
        for dimension in &mut run_set.dimensions {
            if dimension.kind == RunSetDimensionKind::Temperature {
                dimension.set_values_from_lines("-40\nwarm\n125", 2);
            }
        }
        let mut state = TempDialogState::from_config(&TempConfig::default());
        inheriting(&mut state);

        let error = state
            .to_config(&run_set, reference())
            .expect_err("an unparseable axis value must not be dropped");

        assert!(error.contains("not a temperature"), "{error}");
    }

    #[test]
    fn the_mode_round_trips_and_absent_means_explicit() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        inheriting(&mut state);

        let encoded = serde_json::to_string(&state).expect("state encodes");
        let decoded: TempDialogState = serde_json::from_str(&encoded).expect("state decodes");
        assert_eq!(decoded.axis_mode(), TempAxisMode::InheritRunSetAxis);

        // A project saved before the switch existed carries no key at all, and
        // must load as the mode it was authored in.
        let legacy = r#"{
            "temp_start": "-40",
            "temp_stop": "125",
            "temp_step": "25",
            "base_idx": 0
        }"#;
        let legacy: TempDialogState = serde_json::from_str(legacy).expect("legacy state decodes");
        assert_eq!(legacy.axis_mode(), TempAxisMode::Explicit);
        assert_eq!(
            legacy
                .to_config(&plan(), reference())
                .expect("the authored range still runs")
                .num_temps(),
            7,
            "a legacy project keeps sweeping its own range"
        );
    }

    /// An axis card that exists but holds nothing declares no temperatures, so
    /// it resolves the same way as no axis at all rather than as an empty
    /// sweep — an analysis that ran at no temperature would produce nothing.
    #[test]
    fn an_axis_declared_with_no_values_falls_back_to_the_reference() {
        let mut run_set = plan();
        run_set
            .dimensions
            .retain(|dimension| dimension.kind != RunSetDimensionKind::Temperature);
        run_set.dimensions.push(RunSetDimension::new(
            "dimension-temperature",
            RunSetDimensionKind::Temperature,
            &[],
            1,
        ));
        let mut state = TempDialogState::from_config(&TempConfig::default());
        inheriting(&mut state);

        let config = state
            .to_config(&run_set, reference())
            .expect("an empty axis is not a broken one");

        assert_eq!(config.specific_temps, vec![reference().temperature_celsius]);
    }
}
