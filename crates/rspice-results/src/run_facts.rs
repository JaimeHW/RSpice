//! What a run set the engine to, recorded with the run as it starts.
//!
//! The temperatures, process corners and supplies its solves were set to,
//! the design variables and the options its deck stated, what it stepped
//! across its points, the plan it ran and the simulator that ran it: what a
//! reader of a run at 85 °C would otherwise
//! find only by opening the deck it executed. It is read where the run was
//! prepared, from the deck it executes and the points its tasks solve, so it
//! states what ran rather than what the setup says now.
//!
//! It is the run's account of itself, not result content: no digest or
//! identity of the run reads it.

/// One name the run's deck set, and the value it set it to, as the deck
/// spells it: `rload` and `2k`, or `RELTOL` and `1e-4`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunSetting {
    pub name: String,
    pub value: String,
}

/// A name the run stepped across its points, and each value it took there,
/// once, in the order the points took them.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunSweep {
    pub name: String,
    pub values: Vec<String>,
}

/// What a run set the engine to.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunFacts {
    /// Every temperature the run's solves were set to, in °C, ascending and
    /// once each.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub temperatures: Vec<f64>,
    /// Every process corner a point of the run solved at, once each, in the
    /// order its points first took them. Empty for a run whose solves named
    /// no corner: a deck run against its own models.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<String>,
    /// Every supply a point of the run set, in volts, ascending and once
    /// each. Empty for a run that left the deck's own supply standing.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplies: Vec<f64>,
    /// The design variables at the deck's top level, in the order it sets
    /// them, each at the last value it was given.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<RunSetting>,
    /// The parameters and sources stepped across the run's points, other
    /// than the temperature, in the order the points first step them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sweeps: Vec<RunSweep>,
    /// The simulator options the deck sets away from their defaults, in the
    /// order it sets them, each at the last value it was given.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<RunSetting>,
    /// The simulator that ran it, with its version and build: `RSpice 0.1.0
    /// (build 1a2b3c4)`. Empty until the run that records it says.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub simulator: String,
    /// The plan it ran, by the name the plan had when it ran. Empty for a
    /// run no plan produced: a manual deck's.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub plan: String,
}

impl RunFacts {
    /// Note that a solve was set to `celsius`. A temperature already noted
    /// is noted once; one that is not finite names no temperature.
    pub fn add_temperature(&mut self, celsius: f64) {
        if !celsius.is_finite() {
            return;
        }
        if let Err(place) = self
            .temperatures
            .binary_search_by(|held| held.total_cmp(&celsius))
        {
            self.temperatures.insert(place, celsius);
        }
    }

    /// Note that a point solved at the process corner `process` and the
    /// supply `volts`, where it set one.
    pub fn add_corner(&mut self, process: &str, volts: Option<f64>) {
        if !self.processes.iter().any(|held| held == process) {
            self.processes.push(process.to_owned());
        }
        if let Some(volts) = volts.filter(|volts| volts.is_finite())
            && let Err(place) = self
                .supplies
                .binary_search_by(|held| held.total_cmp(&volts))
        {
            self.supplies.insert(place, volts);
        }
    }

    /// The point a result of this run solved at, in the words that tell it
    /// from the run's other points: of its process, supply, temperature and
    /// swept values, those the run took more than one of, `125 °C` or
    /// `FF, r=2k`. `None` where the run solved at one point.
    #[must_use]
    pub fn point_words(&self, point: &crate::provenance::AnalysisResultPvtPoint) -> Option<String> {
        use crate::number_format::spice_coordinate;
        let mut words = Vec::new();
        if self.processes.len() > 1 {
            words.push(point.process().to_owned());
        }
        if self.supplies.len() > 1
            && let Some(volts) = point.supply_voltage()
        {
            words.push(spice_coordinate(volts, "V"));
        }
        if self.temperatures.len() > 1 {
            words.push(spice_coordinate(point.temperature_celsius(), "°C"));
        }
        for swept in point.swept_values() {
            if self
                .sweep_of(&swept.name)
                .is_some_and(|sweep| sweep.values.len() > 1)
            {
                words.push(format!("{}={}", swept.name, swept.value));
            }
        }
        (!words.is_empty()).then(|| words.join(", "))
    }

    /// Set the design variable `name` to `value`: a name set before keeps
    /// its place and takes the later value, as the engine reads it.
    pub fn set_variable(&mut self, name: &str, value: &str) {
        set(&mut self.variables, name, value);
    }

    /// Set the option `name` to `value`, as [`Self::set_variable`] does.
    pub fn set_option(&mut self, name: &str, value: &str) {
        set(&mut self.options, name, value);
    }

    /// Note that a point stepped `name` to `value`.
    pub fn add_swept(&mut self, name: &str, value: &str) {
        let at = match self
            .sweeps
            .iter()
            .position(|sweep| sweep.name.eq_ignore_ascii_case(name))
        {
            Some(at) => at,
            None => {
                self.sweeps.push(RunSweep {
                    name: name.to_owned(),
                    values: Vec::new(),
                });
                self.sweeps.len() - 1
            }
        };
        let values = &mut self.sweeps[at].values;
        if !values.iter().any(|held| held == value) {
            values.push(value.to_owned());
        }
    }

    /// The sweep that steps `name`, a design variable's name say.
    #[must_use]
    pub fn sweep_of(&self, name: &str) -> Option<&RunSweep> {
        self.sweeps
            .iter()
            .find(|sweep| sweep.name.eq_ignore_ascii_case(name))
    }
}

/// Set `name` to `value` in `settings`, a name set before keeping its place.
fn set(settings: &mut Vec<RunSetting>, name: &str, value: &str) {
    match settings
        .iter_mut()
        .find(|setting| setting.name.eq_ignore_ascii_case(name))
    {
        Some(setting) => value.clone_into(&mut setting.value),
        None => settings.push(RunSetting {
            name: name.to_owned(),
            value: value.to_owned(),
        }),
    }
}
