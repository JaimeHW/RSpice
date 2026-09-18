//! Periodic steady-state (PSS) analysis configuration.
//!
//! The dialog owns the whole Simulation Studio PSS contract: every control the
//! engine's `.PSS` card carries and this form can hold. Text is retained until
//! preflight so incomplete edits can be saved without silently changing the
//! last valid execution request.

use serde::{Deserialize, Deserializer, Serialize};

use super::options::{IntegrationMethod, parse_si_value};

/// Fully parsed PSS contract.
#[derive(Debug, Clone, PartialEq)]
pub struct PssConfig {
    /// Integration method the shooting solve's inner transients run under, or
    /// `None` for the engine's own default.
    ///
    /// This replaced a "solver mode" that had one executable position. The
    /// engine's shooting solver is the only periodic steady-state path there
    /// is — harmonic balance is its own analysis with its own card — so a
    /// chooser over formulations was a control with nothing to choose. What the
    /// card does carry is `METHOD=`, and that is a real choice: the inner
    /// transient of every shooting period integrates under it.
    pub integration_method: Option<IntegrationMethod>,
    /// Driven fundamental, or the initial frequency estimate for an oscillator.
    pub fund_freq: f64,
    /// Exact named periodic large-signal sources participating in this solve.
    pub tone_sources: Vec<String>,
    /// Number of periods used to settle before the shooting solve.
    pub tstab_periods: usize,
    /// Stabilization window in seconds, or `0` to take it from the period
    /// count above.
    ///
    /// The engine resolves the two in `PssConfig::effective_tstab`: a positive
    /// `tstab` *is* the window, and only a zero one defers to
    /// `tstab_periods * period`. So this is an override rather than an
    /// addition, and the field says so.
    pub tstab: f64,
    /// Shooting-Newton correction limit on each integration grid.
    pub max_iterations: usize,
    /// Absolute periodicity tolerance, in each coordinate's own SI unit.
    ///
    /// A coordinate converges on this or on the relative tolerance above,
    /// which is what lets a node resting at zero converge at all.
    pub abstol: f64,
    /// Newton damping factor. The engine admits `[0.1, 1.0]` and clamps
    /// anything else, so this form refuses outside it rather than accepting a
    /// number the solve will not use.
    pub damping: f64,
    /// Largest relative period correction one autonomous iteration may take.
    pub max_period_change: f64,
    /// Integration samples retained per solved period.
    pub points_per_period: usize,
    /// Relative periodicity tolerance.
    pub tolerance: f64,
    /// Enable autonomous period refinement.
    pub osc_mode: bool,
    /// Voltage node used for autonomous period detection.
    pub osc_node: String,
    /// Number of harmonics retained in the result; zero retains none.
    pub num_harmonics: usize,
}

impl Default for PssConfig {
    fn default() -> Self {
        Self {
            integration_method: None,
            fund_freq: 1.0e3,
            // A tone is one named source in the user's own circuit, and a
            // default cannot know one. Driven validation requires at least one,
            // so an empty list asks for it instead of inventing it.
            tone_sources: Vec::new(),
            tstab_periods: 20,
            // The engine's own card defaults, so an untouched form asks the
            // engine for exactly what a bare `.PSS` card asks it for.
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            points_per_period: 512,
            tolerance: 1.0e-7,
            osc_mode: false,
            osc_node: String::new(),
            num_harmonics: 20,
        }
    }
}

impl PssConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.fund_freq.is_finite() || self.fund_freq <= 0.0 {
            return Err("Fundamental frequency must be finite and positive".to_owned());
        }
        if !self.osc_mode && self.tone_sources.is_empty() {
            return Err("At least one periodic tone source is required".to_owned());
        }
        // A tone list under autonomous mode is a control that does nothing.
        // The engine's own PSS configuration carries no tone field
        // (`rspice-core/src/analysis/pss/config.rs`): a driven solve uses the
        // list only to close the periodic-source contract before dispatch, and
        // an autonomous solve takes its period from the oscillator node
        // instead. Refused rather than ignored, so the form never shows an
        // authored list the run will not read — and naming the tones makes the
        // refusal actionable: the fix is to delete exactly these.
        if self.osc_mode && !self.tone_sources.is_empty() {
            return Err(format!(
                "An autonomous oscillator takes its period from the oscillator node and reads no \
                 tone list; remove {} from the tone source list",
                named_tones(&self.tone_sources)
            ));
        }
        for (index, source) in self.tone_sources.iter().enumerate() {
            if source.trim().is_empty() || source.chars().any(char::is_control) {
                return Err(format!(
                    "Tone source {} is not a valid source name",
                    index + 1
                ));
            }
            if self.tone_sources[..index]
                .iter()
                .any(|prior| prior.eq_ignore_ascii_case(source))
            {
                return Err(format!("Tone source '{source}' is listed more than once"));
            }
        }
        if self.points_per_period < 16 {
            return Err("Shooting points must be at least 16".to_owned());
        }
        let minimum_points = self
            .num_harmonics
            .max(1)
            .checked_mul(2)
            .ok_or_else(|| "Save harmonics exceeds the supported numeric range".to_owned())?;
        if self.points_per_period < minimum_points {
            return Err(
                "Shooting points must be at least twice the retained harmonic count".to_owned(),
            );
        }
        if !self.tolerance.is_finite() || self.tolerance <= 0.0 {
            return Err("Period tolerance must be finite and positive".to_owned());
        }
        // The engine's own bound, taken here so the form refuses what the card
        // cannot spell rather than letting preparation refuse the deck:
        // `.PSS TSTABPERIODS=` is a whole number of at least one
        // (`rspice-core/src/netlist/parser/periodic_cards.rs`, the
        // `TSTABPERIODS` arm), and `TSTAB=0` does not mean "no stabilization"
        // — it means "take the window from the period count" instead.
        if self.tstab_periods == 0 {
            return Err("Stabilization cycles must be at least 1".to_owned());
        }
        // Each bound below is the engine's own, taken from the card arm that
        // reads the keyword and from `PssConfig::validate`. A looser one here
        // would queue a run the deck cannot spell; a tighter one would refuse
        // a deck the engine accepts.
        if !self.tstab.is_finite() || self.tstab < 0.0 {
            return Err("Stabilization time must be finite and non-negative".to_owned());
        }
        if self.max_iterations == 0 {
            return Err("Max iterations must be at least 1".to_owned());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("Absolute tolerance must be finite and positive".to_owned());
        }
        // `PssConfig::with_damping` clamps to this range and the card's
        // `DAMPING` arm refuses outside it. Refused rather than clamped, so a
        // form showing 0.05 never runs a solve damped at 0.1.
        if !self.damping.is_finite() || !(0.1..=1.0).contains(&self.damping) {
            return Err("Damping must be between 0.1 and 1".to_owned());
        }
        if !self.max_period_change.is_finite() || self.max_period_change <= 0.0 {
            return Err("Max period change must be finite and positive".to_owned());
        }
        if self.osc_mode && self.osc_node.trim().is_empty() {
            return Err("Oscillator node is required for an autonomous oscillator".to_owned());
        }
        Ok(())
    }

    /// The `.PSS` card the engine reads, in the engine's own grammar.
    ///
    /// `rspice-core/src/netlist/parser/periodic_cards.rs::parse_pss_command`
    /// accepts two disjoint forms: ngspice's positional oscillator card
    /// `.PSS gfreq tstab oscnode psspoints harms sciter`, and a pure keyword
    /// card. The studio writes the keyword form, for two reasons the engine
    /// states itself: the positional form is autonomous by construction — it
    /// names the node the period is detected on, so a driven solve cannot be
    /// written in it at all — and two of its six fields, a stabilization time
    /// in seconds and a Newton-iteration limit, are quantities this form does
    /// not hold.
    ///
    /// No `tones=` key: the engine's `.PSS` has no tone field, and it needs
    /// none. `Engine::validate_periodic_source_contract` accepts exactly the
    /// complete elaborated source set of the deck and refuses any proper
    /// subset, so the sources in the deck *are* the tone list, and a second
    /// spelling of the same fact on the card would only be a key the engine
    /// refuses.
    pub fn to_spice(&self) -> String {
        let mut card = format!(
            ".pss fund={} autonomous={}",
            format_freq(self.fund_freq),
            if self.osc_mode { "yes" } else { "no" },
        );
        // `OSCNODE` on a card that also says `AUTONOMOUS=NO` is a conflict the
        // engine refuses by name, so the node is written only where it means
        // something.
        if self.osc_mode && !self.osc_node.trim().is_empty() {
            card.push_str(&format!(" oscnode={}", self.osc_node.trim()));
        }
        card.push_str(&format!(" tstabperiods={}", self.tstab_periods));
        // `TSTAB=0` is not "no stabilization" — the engine reads a zero window
        // as "take it from the period count" — so a zero is the absent key
        // rather than a written one. Every other control below is written
        // unconditionally: the card is what the run executed, and a key left
        // off because it happens to hold a default is a card that stops saying
        // so when the engine's default moves.
        if self.tstab > 0.0 {
            card.push_str(&format!(" tstab={:.17e}", self.tstab));
        }
        card.push_str(&format!(" points={}", self.points_per_period));
        // `HARMS` is a whole number of at least one; the card has no spelling
        // for retaining none. Nothing is lost by clamping: the shooting solve
        // is handed `num_harmonics.max(1)` whatever the form says
        // (`services::simulation_runner::pss::core_pss_config`), and a
        // retention count of zero is the studio's own choice not to queue a
        // spectrum, which is a result decision rather than an engine input.
        card.push_str(&format!(" harms={}", self.num_harmonics.max(1)));
        card.push_str(&format!(" tol={:.17e}", self.tolerance));
        card.push_str(&format!(" abstol={:.17e}", self.abstol));
        card.push_str(&format!(" maxiter={}", self.max_iterations));
        card.push_str(&format!(" damping={}", self.damping));
        // Written on a driven card too, where the engine parses it and never
        // reads it: the period is only an unknown in an autonomous solve. It
        // is still part of the configuration the engine was handed, and the
        // card is the record of that configuration.
        card.push_str(&format!(" maxperiodchange={}", self.max_period_change));
        // `METHOD=` is the one keyword with no spelling for "engine default":
        // the card's absent key *is* that setting, and `PssConfig`'s
        // `integration_method` is an `Option` for the same reason. So an
        // unchosen method writes nothing rather than naming the hybrid the
        // engine happens to default to today.
        if let Some(method) = self.integration_method {
            card.push_str(&format!(" method={}", method.spice_name()));
        }
        card
    }
}

/// Raw persisted PSS editor state.
#[derive(Debug, Clone, Serialize)]
pub struct PssDialogState {
    /// Index into the integration-method chooser: `0` is the engine's own
    /// default, and the rest are [`IntegrationMethod::all`] in order.
    pub integration_method_idx: usize,
    pub fund_freq: String,
    /// Comma-, semicolon-, or whitespace-separated exact source names.
    pub tone_sources: String,
    pub tstab_periods: String,
    /// Empty is the engine's zero: the period count decides the window.
    pub tstab: String,
    pub max_iterations: String,
    pub abstol: String,
    pub damping: String,
    pub max_period_change: String,
    pub points_per_period: String,
    pub tolerance: String,
    pub osc_mode: bool,
    pub osc_node: String,
    /// Numeric retained harmonic count. Zero explicitly retains no harmonics.
    pub num_harmonics: String,
    #[serde(skip)]
    pub initialized: bool,
}

impl Default for PssDialogState {
    fn default() -> Self {
        let mut state = Self::from_config(&PssConfig::default());
        state.initialized = false;
        state
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedPssDialogState {
    #[serde(default = "default_pss_frequency")]
    fund_freq: String,
    #[serde(default = "default_pss_harmonics")]
    num_harmonics: String,
    /// Removed from the visible/executable contract. Accepted so projects from
    /// the temporary shell still open without assigning it hidden semantics.
    #[serde(default)]
    max_iter: Option<String>,
    /// Retired. It selected a "solver mode" whose only other position was the
    /// legacy HB-PSS formulation, which no run ever executed — validation
    /// refused it by name. Accepted and discarded, so a draft saved with
    /// either value opens as the shooting run it always was rather than
    /// failing to deserialize.
    #[serde(default)]
    method_idx: Option<usize>,
    #[serde(default)]
    integration_method_idx: usize,
    #[serde(default)]
    osc_mode: bool,
    #[serde(default)]
    osc_node: String,
    /// Old projects stored harmonic retention as a boolean in addition to the
    /// count. `false` migrates exactly to a retained count of zero.
    #[serde(default)]
    save_harmonics: Option<bool>,
    /// A project that never named a tone restores as one that never named a
    /// tone; no reader can supply a source name the design does not carry.
    #[serde(default)]
    tone_sources: String,
    #[serde(default = "default_pss_stabilization_cycles")]
    tstab_periods: String,
    /// Absent means the engine's zero, which is what every request written
    /// before this control existed asked for.
    #[serde(default)]
    tstab: String,
    #[serde(default = "default_pss_max_iterations")]
    max_iterations: String,
    #[serde(default = "default_pss_abstol")]
    abstol: String,
    #[serde(default = "default_pss_damping")]
    damping: String,
    #[serde(default = "default_pss_max_period_change")]
    max_period_change: String,
    #[serde(default = "default_pss_shooting_points")]
    points_per_period: String,
    #[serde(default = "default_pss_tolerance")]
    tolerance: String,
}

impl<'de> Deserialize<'de> for PssDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedPssDialogState::deserialize(deserializer)?;
        let _retired_max_iterations = persisted.max_iter;
        let _retired_solver_mode = persisted.method_idx;
        let num_harmonics = if persisted.save_harmonics == Some(false) {
            "0".to_owned()
        } else {
            persisted.num_harmonics
        };
        Ok(Self {
            integration_method_idx: persisted.integration_method_idx,
            fund_freq: persisted.fund_freq,
            tone_sources: persisted.tone_sources,
            tstab_periods: persisted.tstab_periods,
            tstab: persisted.tstab,
            max_iterations: persisted.max_iterations,
            abstol: persisted.abstol,
            damping: persisted.damping,
            max_period_change: persisted.max_period_change,
            points_per_period: persisted.points_per_period,
            tolerance: persisted.tolerance,
            osc_mode: persisted.osc_mode,
            osc_node: persisted.osc_node,
            num_harmonics,
            initialized: false,
        })
    }
}

impl PssDialogState {
    /// The last chooser position `integration_method_idx` may hold: the
    /// engine's default at zero, then every offered method.
    ///
    /// Asked of the draft rather than spelled where a project is read, so that
    /// bound cannot fall behind the method list, and the reader names no more
    /// of this module than the draft it already holds.
    pub(crate) fn last_integration_method_position(&self) -> usize {
        IntegrationMethod::all().len()
    }

    pub fn from_config(config: &PssConfig) -> Self {
        Self {
            integration_method_idx: integration_method_index(config.integration_method),
            fund_freq: format_freq(config.fund_freq),
            tone_sources: config.tone_sources.join(", "),
            tstab_periods: config.tstab_periods.to_string(),
            tstab: if config.tstab > 0.0 {
                format!("{:.e}", config.tstab)
            } else {
                String::new()
            },
            max_iterations: config.max_iterations.to_string(),
            abstol: format!("{:.e}", config.abstol),
            damping: config.damping.to_string(),
            max_period_change: config.max_period_change.to_string(),
            points_per_period: config.points_per_period.to_string(),
            tolerance: format!("{:.e}", config.tolerance),
            osc_mode: config.osc_mode,
            osc_node: config.osc_node.clone(),
            num_harmonics: config.num_harmonics.to_string(),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<PssConfig, String> {
        let fund_freq =
            parse_si_value(&self.fund_freq).map_err(|error| format!("Bad frequency: {error}"))?;
        let tone_sources = parse_tone_sources(&self.tone_sources)?;
        let tstab_periods = parse_usize(&self.tstab_periods, "stabilization cycles")?;
        // An empty well is the engine's zero rather than a parse failure: the
        // field's own hint says the period count decides when it is blank, and
        // clearing it has to be how a reader says that.
        let tstab = if self.tstab.trim().is_empty() {
            0.0
        } else {
            parse_si_value(&self.tstab)
                .map_err(|error| format!("Bad stabilization time: {error}"))?
        };
        let max_iterations = parse_usize(&self.max_iterations, "max iterations")?;
        let abstol = parse_si_value(&self.abstol)
            .map_err(|error| format!("Bad absolute tolerance: {error}"))?;
        let damping =
            parse_si_value(&self.damping).map_err(|error| format!("Bad damping: {error}"))?;
        let max_period_change = parse_si_value(&self.max_period_change)
            .map_err(|error| format!("Bad max period change: {error}"))?;
        let points_per_period = parse_usize(&self.points_per_period, "shooting points")?;
        let tolerance = parse_si_value(&self.tolerance)
            .map_err(|error| format!("Bad period tolerance: {error}"))?;
        let num_harmonics = parse_usize(&self.num_harmonics, "save harmonics")?;
        let config = PssConfig {
            integration_method: integration_method_at(self.integration_method_idx),
            fund_freq,
            tone_sources,
            tstab_periods,
            tstab,
            max_iterations,
            abstol,
            damping,
            max_period_change,
            points_per_period,
            tolerance,
            osc_mode: self.osc_mode,
            osc_node: self.osc_node.trim().to_owned(),
            num_harmonics,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            // Deserialized drafts carry initialized=false but already contain
            // authoritative text. Only the all-empty legacy/default state is
            // replaced; otherwise mark it ready without losing migration data.
            if self.fund_freq.is_empty()
                && self.tone_sources.is_empty()
                && self.tstab_periods.is_empty()
                && self.points_per_period.is_empty()
                && self.tolerance.is_empty()
                && self.num_harmonics.is_empty()
            {
                *self = Self::from_config(&PssConfig::default());
            } else {
                self.initialized = true;
            }
        }
    }
}

/// Where an integration method sits in the chooser.
///
/// Position `0` is the engine's own default, which is not a member of
/// [`IntegrationMethod`] — the card has no keyword for it — so the offered
/// methods start at one.
pub(crate) fn integration_method_index(method: Option<IntegrationMethod>) -> usize {
    match method {
        None => 0,
        Some(method) => IntegrationMethod::all()
            .iter()
            .position(|candidate| *candidate == method)
            .map_or(0, |index| index + 1),
    }
}

/// The method a chooser position names, or `None` for the engine's default.
///
/// An index past the end reads as the default rather than as a panic or a
/// guessed method: the only way to reach one is a draft written by a build
/// that offered more methods than this one, and the engine's default is the
/// setting that build's card would have written had the method been unset.
pub(crate) fn integration_method_at(index: usize) -> Option<IntegrationMethod> {
    index
        .checked_sub(1)
        .and_then(|index| IntegrationMethod::all().get(index).copied())
}

/// The tone names a refusal asks the engineer to delete, quoted and joined so
/// the sentence reads as one instruction rather than a debug list.
fn named_tones(sources: &[String]) -> String {
    let quoted = sources
        .iter()
        .map(|source| format!("'{source}'"))
        .collect::<Vec<_>>();
    match quoted.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

fn parse_tone_sources(value: &str) -> Result<Vec<String>, String> {
    let sources = value
        .split(|character: char| character == ',' || character == ';' || character.is_whitespace())
        .filter(|source| !source.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    Ok(sources)
}

fn parse_usize(value: &str, label: &str) -> Result<usize, String> {
    value
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("Invalid {label}"))
}

fn default_pss_frequency() -> String {
    "1k".to_owned()
}

fn default_pss_harmonics() -> String {
    "20".to_owned()
}

fn default_pss_stabilization_cycles() -> String {
    "20".to_owned()
}

fn default_pss_shooting_points() -> String {
    "512".to_owned()
}

fn default_pss_tolerance() -> String {
    "1e-7".to_owned()
}

fn default_pss_max_iterations() -> String {
    "100".to_owned()
}

fn default_pss_abstol() -> String {
    "1e-12".to_owned()
}

fn default_pss_damping() -> String {
    "1".to_owned()
}

fn default_pss_max_period_change() -> String {
    "0.1".to_owned()
}

fn format_freq(frequency: f64) -> String {
    if frequency >= 1e9 {
        format!("{}G", frequency / 1e9)
    } else if frequency >= 1e6 {
        format!("{}Meg", frequency / 1e6)
    } else if frequency >= 1e3 {
        format!("{}k", frequency / 1e3)
    } else {
        format!("{frequency}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact card, both modes.
    ///
    /// `directive_parse_ratchet` proves the engine reads whatever this writes,
    /// which is the property that matters and the one a string comparison
    /// cannot give. This pins the other half: *which* card was written. A
    /// driven card that quietly acquired an `oscnode=` would still parse, and
    /// would still be a different analysis than the one the form states.
    /// The card an untouched form writes, field for field.
    ///
    /// The two tolerances are spelled through the same formatter the card uses
    /// rather than transcribed: seventeen significant digits of a decimal that
    /// is not representable in binary is a literal nobody can review, and what
    /// this test is for is *which* keys the card carries, in what order.
    #[test]
    fn the_card_states_the_solve_the_form_was_set_to() {
        let solver = format!(
            "tol={:.17e} abstol={:.17e} maxiter=100 damping=1 maxperiodchange=0.1",
            1.0e-7_f64, 1.0e-12_f64
        );
        assert_eq!(
            PssConfig {
                tone_sources: vec!["VIN".to_owned()],
                ..PssConfig::default()
            }
            .to_spice(),
            format!(".pss fund=1k autonomous=no tstabperiods=20 points=512 harms=20 {solver}")
        );
        assert_eq!(
            PssConfig {
                osc_mode: true,
                osc_node: "osc_out".to_owned(),
                ..PssConfig::default()
            }
            .to_spice(),
            format!(
                ".pss fund=1k autonomous=yes oscnode=osc_out tstabperiods=20 points=512 \
                 harms=20 {solver}"
            )
        );
    }

    /// A stabilization time reaches the card only when it is one.
    ///
    /// `TSTAB=0` does not mean "no stabilization" to the engine — a zero
    /// window is what defers to the period count — so writing the zero would
    /// state the same thing twice and writing a nonzero one has to override.
    #[test]
    fn the_card_carries_a_stabilization_time_only_when_the_form_states_one() {
        let driven = PssConfig {
            tone_sources: vec!["VIN".to_owned()],
            ..PssConfig::default()
        };
        assert!(!driven.to_spice().contains("tstab="));
        let card = PssConfig {
            tstab: 3.0e-9,
            ..driven
        }
        .to_spice();
        assert!(
            card.contains(&format!(" tstab={:.17e} ", 3.0e-9_f64)),
            "{card}"
        );
        assert!(
            card.contains(" tstabperiods=20 "),
            "the cycle count is still on the card it no longer decides: {card}"
        );
    }

    /// A chosen integration method reaches the card, and an unchosen one
    /// writes no keyword at all.
    #[test]
    fn the_card_names_the_integration_method_only_once_one_is_chosen() {
        let driven = PssConfig {
            tone_sources: vec!["VIN".to_owned()],
            ..PssConfig::default()
        };
        assert!(
            !driven.to_spice().contains("method="),
            "the engine's default has no keyword spelling: {}",
            driven.to_spice()
        );
        for method in IntegrationMethod::all() {
            let card = PssConfig {
                integration_method: Some(*method),
                ..driven.clone()
            }
            .to_spice();
            assert!(
                card.ends_with(&format!(" method={}", method.spice_name())),
                "{card}"
            );
        }
    }

    /// Every chooser position round trips, and nothing outside the offered set
    /// invents a method.
    #[test]
    fn every_integration_method_position_round_trips_through_the_chooser() {
        assert_eq!(integration_method_at(0), None);
        for (offset, method) in IntegrationMethod::all().iter().enumerate() {
            assert_eq!(integration_method_at(offset + 1), Some(*method));
            assert_eq!(integration_method_index(Some(*method)), offset + 1);
        }
        assert_eq!(integration_method_index(None), 0);
        assert_eq!(
            integration_method_at(IntegrationMethod::all().len() + 1),
            None,
            "a position this build does not offer reads as the engine's default"
        );
    }

    /// Retaining no spectrum still writes a card the engine can read.
    ///
    /// `HARMS` has no zero on the engine's card, and a project saved before
    /// the retained count was a number carries exactly that zero — the
    /// `save_harmonics: false` migration above produces it. The card states
    /// the one harmonic the shooting solve is given either way.
    #[test]
    fn a_card_retaining_no_spectrum_still_states_a_harmonic_the_card_can_hold() {
        assert!(
            PssConfig {
                tone_sources: vec!["VIN".to_owned()],
                num_harmonics: 0,
                ..PssConfig::default()
            }
            .to_spice()
            .contains(" harms=1 "),
        );
    }

    /// A draft written before the tone list existed restores as naming none.
    ///
    /// The default was once the literal `"VIN_DIFF"` — a source name invented
    /// from nothing, which the reader would then see in the Tone sources
    /// field of a plan they never authored it into, and which the deck would
    /// carry. It is empty now, and this pins that with real absent-key JSON:
    /// rewriting the value to `""` would pass even if the default came back.
    #[test]
    fn a_draft_naming_no_tone_source_restores_as_naming_none() {
        let state = PssDialogState {
            tone_sources: "VIN_LO".to_owned(),
            ..PssDialogState::default()
        };
        let mut document: serde_json::Value =
            serde_json::to_value(&state).expect("the draft serializes");
        let object = document
            .as_object_mut()
            .expect("a draft is written as an object");
        assert!(
            object.remove("tone_sources").is_some(),
            "the fixture must state one for its removal to mean anything"
        );

        let restored: PssDialogState =
            serde_json::from_value(document).expect("a draft written before tone lists loads");
        assert_eq!(
            restored.tone_sources, "",
            "no reader can supply a source name the design does not carry"
        );
    }

    #[test]
    fn the_dialog_round_trips_every_field_to_config() {
        let state = PssDialogState {
            integration_method_idx: 0,
            fund_freq: "2.5Meg".to_owned(),
            tone_sources: "VIN_LO, VIN_MOD".to_owned(),
            tstab_periods: "37".to_owned(),
            tstab: "3n".to_owned(),
            max_iterations: "250".to_owned(),
            abstol: "1e-15".to_owned(),
            damping: "0.75".to_owned(),
            max_period_change: "0.25".to_owned(),
            points_per_period: "1024".to_owned(),
            tolerance: "2e-9".to_owned(),
            // Driven, because this fixture names two tones. The oscillator node
            // is still carried: the dialog retains text the current mode does
            // not execute, and this proves a driven round trip does not eat it.
            osc_mode: false,
            osc_node: "osc_out".to_owned(),
            num_harmonics: "31".to_owned(),
            initialized: true,
        };
        let config = state.to_config().expect("valid exact PSS contract");
        assert_eq!(config.integration_method, None);
        assert_eq!(config.fund_freq, 2.5e6);
        assert_eq!(config.tone_sources, ["VIN_LO", "VIN_MOD"]);
        assert_eq!(config.tstab_periods, 37);
        assert_eq!(config.tstab, 3.0e-9);
        assert_eq!(config.max_iterations, 250);
        assert_eq!(config.abstol, 1.0e-15);
        assert_eq!(config.damping, 0.75);
        assert_eq!(config.max_period_change, 0.25);
        assert_eq!(config.points_per_period, 1024);
        assert_eq!(config.tolerance, 2.0e-9);
        assert!(!config.osc_mode);
        assert_eq!(config.osc_node, "osc_out");
        assert_eq!(config.num_harmonics, 31);
    }

    #[test]
    fn an_autonomous_solve_holding_tones_is_refused_and_names_them() {
        let mut state = PssDialogState::from_config(&PssConfig::default());
        state.osc_mode = true;
        state.osc_node = "osc".to_owned();
        state.tone_sources = "VIN_LO, VIN_MOD".to_owned();

        let error = state
            .to_config()
            .expect_err("an autonomous solve cannot also be driven");

        assert!(error.contains("autonomous"), "{error}");
        // The instruction has to name what to delete, or the engineer is left
        // to guess which of the two mutually exclusive controls to give up.
        assert!(error.contains("remove"), "{error}");
        assert!(error.contains("'VIN_LO'"), "{error}");
        assert!(error.contains("'VIN_MOD'"), "{error}");
    }

    /// The contradiction is one rule with two sides, and both are refused. The
    /// driven side already shipped; pinning them together is what stops a later
    /// edit from restoring one and dropping the other.
    #[test]
    fn a_driven_solve_naming_no_tone_is_refused() {
        let mut state = PssDialogState::from_config(&PssConfig::default());
        state.osc_mode = false;
        state.tone_sources.clear();

        let error = state
            .to_config()
            .expect_err("a driven solve has no fundamental without a tone");

        assert!(error.contains("tone source"), "{error}");
    }

    #[test]
    fn a_single_refused_tone_is_named_without_a_conjunction() {
        let mut state = PssDialogState::from_config(&PssConfig::default());
        state.osc_mode = true;
        state.osc_node = "osc".to_owned();
        state.tone_sources = "VIN_LO".to_owned();

        let error = state.to_config().expect_err("still a contradiction");

        assert!(error.contains("remove 'VIN_LO' from"), "{error}");
    }

    #[test]
    fn old_dialog_boolean_retention_migrates_without_guessing() {
        let migrated: PssDialogState = serde_json::from_str(
            r#"{"fund_freq":"1Meg","num_harmonics":"12","max_iter":"50","method_idx":0,"osc_mode":false,"osc_node":"","save_harmonics":false}"#,
        )
        .expect("legacy state migrates");
        assert_eq!(migrated.num_harmonics, "0");
        // The legacy state named no tone, and a reader cannot name one for it.
        assert_eq!(migrated.tone_sources, "");
        assert_eq!(migrated.tstab_periods, "20");
        assert_eq!(migrated.points_per_period, "512");
        assert_eq!(migrated.tolerance, "1e-7");
        // The solver controls a legacy draft never held restore as the engine
        // card's own defaults, so the migrated request is the one that ran.
        assert_eq!(migrated.tstab, "");
        assert_eq!(migrated.max_iterations, "100");
        assert_eq!(migrated.abstol, "1e-12");
        assert_eq!(migrated.damping, "1");
        assert_eq!(migrated.max_period_change, "0.1");
    }

    /// Every bound this form states is the engine's own.
    #[test]
    fn the_solver_controls_are_refused_outside_the_engines_own_bounds() {
        let valid = || PssDialogState {
            tone_sources: "VIN".to_owned(),
            ..PssDialogState::from_config(&PssConfig::default())
        };
        for (field, value, expected) in [
            ("tstab", "-1n", "Stabilization time"),
            ("max_iterations", "0", "Max iterations"),
            ("abstol", "0", "Absolute tolerance"),
            ("damping", "0.05", "Damping"),
            ("damping", "1.5", "Damping"),
            ("max_period_change", "0", "Max period change"),
        ] {
            let mut state = valid();
            match field {
                "tstab" => state.tstab = value.to_owned(),
                "max_iterations" => state.max_iterations = value.to_owned(),
                "abstol" => state.abstol = value.to_owned(),
                "damping" => state.damping = value.to_owned(),
                _ => state.max_period_change = value.to_owned(),
            }
            let error = state
                .to_config()
                .expect_err("{field}={value} is outside the engine's own bound");
            assert!(error.contains(expected), "{field}={value}: {error}");
        }
        // The ends of the damping range are admissible: they are the boundary,
        // not values outside it.
        for damping in ["0.1", "1"] {
            let mut state = valid();
            state.damping = damping.to_owned();
            assert!(state.to_config().is_ok(), "damping={damping}");
        }
    }

    /// The retired solver-mode key opens as what it always ran as.
    ///
    /// `method_idx: 1` selected the legacy HB-PSS formulation, which no run
    /// executed: the dialog refused it, the spec validator refused it, and the
    /// runner refused it. So a draft carrying it was a draft that could not be
    /// run at all, and the honest migration is the shooting solve — the one
    /// the engine has. The key is read and discarded, which is what keeps the
    /// draft loading rather than failing `deny_unknown_fields`.
    #[test]
    fn a_legacy_pss_draft_with_the_retired_mode_opens_as_a_shooting_run() {
        for retired in ["0", "1"] {
            let restored: PssDialogState = serde_json::from_str(&format!(
                r#"{{"fund_freq":"1Meg","num_harmonics":"12","method_idx":{retired},
                     "tone_sources":"VIN","osc_mode":false,"osc_node":""}}"#
            ))
            .expect("a draft written with the retired solver mode loads");
            assert_eq!(
                restored.integration_method_idx, 0,
                "the retired key names no integration method"
            );
            let config = restored
                .to_config()
                .expect("the restored draft is a runnable shooting request");
            assert_eq!(config.integration_method, None);
            assert!(
                !config.to_spice().contains("method="),
                "a migrated draft asks for the engine's own integration method"
            );
        }
    }

    #[test]
    fn autonomous_contract_can_authenticate_an_empty_drive_set() {
        let mut state = PssDialogState::from_config(&PssConfig::default());
        state.osc_mode = true;
        state.osc_node = "osc".to_owned();
        state.tone_sources.clear();
        assert!(state.to_config().is_ok());
    }
}
