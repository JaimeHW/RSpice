//! S-parameter port dialog state.

use super::format::format_freq;
use super::{SpConfig, SpPortConfig, SpSweepType};
use crate::simulation::dialog::options::parse_si_value;
use crate::simulation::placed_sources::{PlacedRfPort, duplicate_port_numbers};

/// Where an S-parameter run's ports come from.
///
/// A port is a Z0 plane the run drives and measures, and there are two places a
/// design can declare one. The schematic places `RF Port` components, which the
/// netlist generator emits as `P` cards; the analysis form can instead name node
/// pairs, which the runner materializes as Thevenin generators behind Z0 when —
/// and only when — the deck declares none of its own
/// (`services::simulation_runner::sparameter::resolve_ports`).
///
/// Both at once is not a third mode, it is the ambiguity this switch exists to
/// end. The deck's own ports always win at the solver, so an ad-hoc table
/// standing beside placed ports never reaches the network; what it does reach is
/// everything downstream that reads the spec's port list as the run's identity —
/// the Touchstone header's reference impedances and the saved-output contract's
/// `S(i,j)` bound. One truth, chosen here.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpPortSource {
    /// The `RF Port` components the design places. Their `P` cards are the
    /// ports; nothing is synthesized.
    #[default]
    Placed,
    /// Node pairs typed on this form, for a netlist-first project whose deck
    /// places nothing.
    AdHoc,
}

impl SpPortSource {
    pub const ALL: [Self; 2] = [Self::Placed, Self::AdHoc];

    /// Exact Simulation Studio choice label.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Placed => "From placed RF ports",
            Self::AdHoc => "Ad-hoc node ports",
        }
    }

    /// Index into [`SpPortSource::ALL`], which is what the choice row holds.
    #[must_use]
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|mode| *mode == self)
            .unwrap_or_default()
    }
}

/// Dialog state with string buffers for SI-prefix input
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpPortDialogState {
    /// Positive node name.
    pub node_pos: String,
    /// Differential mode flag.
    pub differential: bool,
    /// Negative node name for differential mode.
    pub node_neg: String,
    /// Port-specific reference impedance override enabled.
    pub z0_override: bool,
    /// Port-specific reference impedance text buffer.
    pub z0: String,
}

impl SpPortDialogState {
    pub(super) fn single_ended(node_pos: impl Into<String>) -> Self {
        Self {
            node_pos: node_pos.into(),
            differential: false,
            node_neg: "0".to_string(),
            z0_override: false,
            z0: String::new(),
        }
    }

    fn from_port_config(port: &SpPortConfig) -> Self {
        Self {
            node_pos: port.node_pos.clone(),
            differential: port.is_differential(),
            node_neg: port.node_neg.clone(),
            z0_override: port.z0.is_some(),
            z0: port.z0.map(|value| value.to_string()).unwrap_or_default(),
        }
    }

    fn to_port_config(&self, number: u32) -> Result<SpPortConfig, String> {
        let mut port = if self.differential {
            SpPortConfig::differential(number, &self.node_pos, &self.node_neg)
        } else {
            SpPortConfig::single_ended(number, &self.node_pos)
        };
        if self.z0_override {
            let z0 = parse_si_value(&self.z0)
                .map_err(|e| format!("Invalid Port {} Z0: {}", number, e))?;
            port.z0 = Some(z0);
        }
        Ok(port)
    }
}

/// Dialog state with string buffers for SI-prefix input
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpDialogState {
    /// Start frequency buffer
    pub start_freq: String,
    /// Stop frequency buffer
    pub stop_freq: String,
    /// Number of points buffer
    pub num_points: String,
    /// Sweep type index (0=decade, 1=octave, 2=linear)
    pub sweep_type_idx: usize,
    /// Reference impedance buffer
    pub z0: String,
    /// Editable port definitions. Retained whichever source is in force, so
    /// that switching back to [`SpPortSource::AdHoc`] restores the table the
    /// user wrote; read only in that mode.
    pub ports: Vec<SpPortDialogState>,
    /// Which declaration of the run's ports this analysis reads, as an index
    /// into [`SpPortSource::ALL`].
    ///
    /// `None` is not a third position: it is a form that has never been told,
    /// and it resolves to the answer the design itself gives — placed ports if
    /// the sheet places any, the ad-hoc table if it does not
    /// ([`SpDialogState::port_source`]). Every project saved before the switch
    /// existed loads as `None`, which is right for both kinds of project: a
    /// netlist-first one keeps its table, and a bench that places ports gets
    /// the ports the solver was already measuring.
    #[serde(default)]
    pub port_source_idx: Option<usize>,
    /// Enable noise analysis
    pub do_noise: bool,
    /// Enable Touchstone export
    pub touchstone_export: bool,
    /// Touchstone format version (1 or 2).
    pub touchstone_version: u32,
    /// Initialized flag
    #[serde(skip)]
    pub initialized: bool,
}

impl SpDialogState {
    pub(super) fn default_port_node(index: usize) -> String {
        match index {
            0 => "IN".to_string(),
            1 => "OUT".to_string(),
            _ => format!("P{}", index + 1),
        }
    }

    fn ensure_min_ports(&mut self) {
        while self.ports.len() < 2 {
            let node = Self::default_port_node(self.ports.len());
            self.ports.push(SpPortDialogState::single_ended(node));
        }
    }

    /// Initialize from config
    pub fn from_config(config: &SpConfig) -> Self {
        let mut sorted_ports = config.ports.clone();
        sorted_ports.sort_by_key(|port| port.number);
        let mut port_states: Vec<SpPortDialogState> = sorted_ports
            .iter()
            .map(SpPortDialogState::from_port_config)
            .collect();
        if port_states.is_empty() {
            port_states.push(SpPortDialogState::single_ended("IN"));
            port_states.push(SpPortDialogState::single_ended("OUT"));
        }
        while port_states.len() < 2 {
            let node = Self::default_port_node(port_states.len());
            port_states.push(SpPortDialogState::single_ended(node));
        }

        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                SpSweepType::Decade => 0,
                SpSweepType::Octave => 1,
                SpSweepType::Linear => 2,
            },
            z0: config.z0.to_string(),
            ports: port_states,
            port_source_idx: None,
            do_noise: config.do_noise,
            touchstone_export: config.touchstone_export,
            touchstone_version: config.touchstone_version.clamp(1, 2),
            initialized: true,
        }
    }

    /// Which declaration of the ports this analysis reads.
    ///
    /// `placed_port_count` is the design's answer, and it decides only the
    /// unchosen case: once the form has been told, the choice stands however
    /// the sheet is edited, because silently moving an analysis off the ports
    /// it was configured against is the failure this switch replaces.
    #[must_use]
    pub fn port_source(&self, placed_port_count: usize) -> SpPortSource {
        match self
            .port_source_idx
            .and_then(|index| SpPortSource::ALL.get(index).copied())
        {
            Some(source) => source,
            None if placed_port_count > 0 => SpPortSource::Placed,
            None => SpPortSource::AdHoc,
        }
    }

    /// Convert to config, resolved against the ports the design places.
    ///
    /// `design` is the sheet's placed RF ports, or `None` from a caller that
    /// cannot see the schematic. `None` is not "the design places nothing": a
    /// surface holding only the simulation setup must not refuse a placed-mode
    /// analysis for placing nothing it was never shown, so it validates the
    /// sweep, the impedance and the table it does own, and leaves the port
    /// roster to the dispatching caller that can read the design.
    pub fn to_config(&self, design: Option<&[PlacedRfPort]>) -> Result<SpConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;
        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;
        let points: u32 = self.num_points.parse().map_err(|_| "Invalid point count")?;
        let z0: f64 = self.z0.parse().map_err(|_| "Invalid Z0")?;

        let sweep_type = match self.sweep_type_idx {
            0 => SpSweepType::Decade,
            1 => SpSweepType::Octave,
            _ => SpSweepType::Linear,
        };

        let ports = self.resolve_ports(design)?;

        let config = SpConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            z0,
            ports,
            do_noise: self.do_noise,
            touchstone_export: self.touchstone_export,
            touchstone_version: self.touchstone_version.clamp(1, 2),
        };

        config.validate()?;
        Ok(config)
    }

    /// The ports this analysis would run, in the order the matrix indexes them.
    fn resolve_ports(&self, design: Option<&[PlacedRfPort]>) -> Result<Vec<SpPortConfig>, String> {
        match (self.port_source(design.map_or(0, <[_]>::len)), design) {
            // The placed `P` cards are the ports. What is built here is a
            // description of them, not a second declaration: nothing is
            // synthesized from it, and it exists so that everything reading the
            // spec's port list — the Touchstone header, the saved-output bound
            // — reads the run's real roster.
            (SpPortSource::Placed, Some(placed)) => placed_port_configs(placed),
            // Placed mode with the design out of view. The roster is the
            // dispatching caller's to check; an empty table is a legitimate
            // `SpConfig` for exactly this reason.
            (SpPortSource::Placed, None) => Ok(Vec::new()),
            (SpPortSource::AdHoc, design) => {
                if let Some(placed) = design.filter(|placed| !placed.is_empty()) {
                    return Err(ad_hoc_beside_placed_ports(placed));
                }
                self.ports
                    .iter()
                    .enumerate()
                    .map(|(idx, port)| port.to_port_config((idx + 1) as u32))
                    .collect()
            }
        }
    }

    /// Why this analysis has no port roster, if it has none.
    ///
    /// The same resolution dispatch performs, asked without building a whole
    /// configuration, so the form can state the refusal beside the ports it is
    /// about rather than leaving it to surface only once a run is attempted.
    #[must_use]
    pub fn port_roster_error(&self, placed: &[PlacedRfPort]) -> Option<String> {
        self.resolve_ports(Some(placed)).err()
    }

    /// Initialize defaults if not already
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            let port_source_idx = self.port_source_idx;
            *self = Self::from_config(&SpConfig::default());
            // A decoded project carries its choice on an uninitialized state,
            // and re-initializing must not throw it away: that would drop every
            // saved analysis back to the design's answer.
            self.port_source_idx = port_source_idx;
        }
        self.ensure_min_ports();
    }
}

/// The placed ports as an executable roster, or the reason there is not one.
///
/// Every refusal names the ports it is about. `duplicate_port_numbers` states
/// the collision and deliberately leaves the verdict to the dispatching
/// surface; this is that surface, and the verdict is no — an S-parameter run
/// addresses a port by its number, so two ports answering to one number is a
/// matrix with no defined meaning, and picking a winner would report the
/// network the user did not draw.
fn placed_port_configs(placed: &[PlacedRfPort]) -> Result<Vec<SpPortConfig>, String> {
    if placed.len() < 2 {
        return Err(too_few_placed_ports(placed.len()));
    }
    let duplicates = duplicate_port_numbers(placed);
    if !duplicates.is_empty() {
        return Err(duplicate_placed_port_numbers(placed, &duplicates));
    }
    if let Some(gap) = first_missing_port_number(placed) {
        return Err(non_contiguous_placed_port_numbers(placed, gap));
    }

    Ok(placed
        .iter()
        .map(|port| SpPortConfig {
            number: port.port_number,
            node_pos: terminal(port, 0),
            node_neg: terminal(port, 1),
            // An impedance the placed port states as an expression is not a
            // number this run can normalize to, and the port's own `P` card
            // carries it to the solver either way. Falling back to the run's
            // reference impedance keeps the description honest about what this
            // surface knows rather than inventing a figure.
            z0: crate::quantity::parse_engineering_value(&port.z0)
                .ok()
                .filter(|value| value.is_finite() && *value > 0.0),
        })
        .collect())
}

/// One terminal of a placed port, as the deck spells it.
///
/// A port element has two pins, and a design still being drawn may have neither
/// wired; `placed_rf_ports` reports an unwired pin as `unconnected` rather than
/// dropping it, so there is always a name here.
fn terminal(port: &PlacedRfPort, pin: usize) -> String {
    port.nets
        .get(pin)
        .cloned()
        .unwrap_or_else(|| "unconnected".to_owned())
}

/// The first number an `N`-port matrix needs and the placed ports do not claim.
fn first_missing_port_number(placed: &[PlacedRfPort]) -> Option<u32> {
    (1..=placed.len() as u32)
        .find(|expected| !placed.iter().any(|port| port.port_number == *expected))
}

/// `P1, P3 and P4`, the ports a sentence is about.
fn port_list(placed: &[PlacedRfPort]) -> String {
    join_words(placed.iter().map(|port| port.reference.clone()))
}

fn join_words(words: impl IntoIterator<Item = String>) -> String {
    let words: Vec<String> = words.into_iter().collect();
    match words.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

fn too_few_placed_ports(count: usize) -> String {
    if count == 0 {
        "This analysis reads the design's RF ports and the sheet places none. Place RF Port \
         components, or set Ports to Ad-hoc node ports and name the port nodes here."
            .to_owned()
    } else {
        "An S-parameter run needs at least 2 ports and the sheet places 1. Place a second RF \
         Port component, or set Ports to Ad-hoc node ports and name the port nodes here."
            .to_owned()
    }
}

fn duplicate_placed_port_numbers(placed: &[PlacedRfPort], duplicates: &[u32]) -> String {
    let numbers = join_words(duplicates.iter().map(u32::to_string));
    format!(
        "More than one placed RF port claims port {}{}. An S-parameter run addresses a port by \
         its number, so give each of {} a number of its own.",
        if duplicates.len() == 1 {
            "number "
        } else {
            "numbers "
        },
        numbers,
        port_list(placed),
    )
}

fn non_contiguous_placed_port_numbers(placed: &[PlacedRfPort], gap: u32) -> String {
    let numbered = join_words(
        placed
            .iter()
            .map(|port| format!("{} at {}", port.reference, port.port_number)),
    );
    format!(
        "The placed RF ports skip port {gap}. {} port{} placed — {numbered} — and an \
         S-parameter matrix is indexed from 1 without gaps, so number them 1 to {}.",
        placed.len(),
        if placed.len() == 1 { " is" } else { "s are" },
        placed.len(),
    )
}

fn ad_hoc_beside_placed_ports(placed: &[PlacedRfPort]) -> String {
    format!(
        "The sheet places {} RF port{} ({}), and a deck's own P cards are what an S-parameter \
         run measures, so the node ports on this form would not be used. Set Ports to From \
         placed RF ports, or delete the placed RF Port components.",
        placed.len(),
        if placed.len() == 1 { "" } else { "s" },
        port_list(placed),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::placed_sources::{RfPortMode, placed_rf_ports};
    use crate::state::{Component, ComponentType, Point, SchematicState};

    /// A placed RF port, resolved exactly as the design would resolve it.
    fn placed(references_and_params: &[(&str, &str)]) -> Vec<PlacedRfPort> {
        let mut schematic = SchematicState::default();
        schematic.components = references_and_params
            .iter()
            .enumerate()
            .map(|(index, (name, params))| {
                let mut component =
                    Component::new(index as u64 + 1, ComponentType::RfPort, Point::origin())
                        .with_name_value(*name, "");
                component.params = (*params).to_owned();
                component
            })
            .collect();
        placed_rf_ports(&schematic, None)
    }

    fn two_placed_ports() -> Vec<PlacedRfPort> {
        placed(&[("P1", "port=1 z0=50"), ("P2", "port=2 z0=75")])
    }

    fn dialog() -> SpDialogState {
        SpDialogState::from_config(&SpConfig::default())
    }

    fn choosing(state: &mut SpDialogState, source: SpPortSource) {
        state.port_source_idx = Some(source.index());
    }

    /// The design answers the question the form has not been asked.
    #[test]
    fn an_unchosen_form_reads_the_source_the_design_declares() {
        let state = dialog();
        assert_eq!(state.port_source(2), SpPortSource::Placed);
        assert_eq!(state.port_source(0), SpPortSource::AdHoc);
    }

    /// And once it has been asked, the answer stands: an analysis must not
    /// change which ports it measures because a port was added to the sheet.
    #[test]
    fn a_chosen_source_survives_an_edit_to_the_design() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::AdHoc);
        assert_eq!(state.port_source(2), SpPortSource::AdHoc);
        choosing(&mut state, SpPortSource::Placed);
        assert_eq!(state.port_source(0), SpPortSource::Placed);
    }

    /// A project saved before the switch existed carries no choice, and must
    /// still load — as the design's own answer, which is what the solver was
    /// already giving it.
    #[test]
    fn a_project_without_the_key_loads_unchosen() {
        let saved = serde_json::to_value(dialog()).expect("the dialog serializes");
        let mut without = saved.as_object().expect("an object").clone();
        assert!(
            without.remove("port_source_idx").is_some(),
            "the key is written"
        );
        let decoded: SpDialogState =
            serde_json::from_value(without.into()).expect("a project without the key decodes");

        assert_eq!(decoded.port_source_idx, None);
        assert_eq!(decoded.port_source(1), SpPortSource::Placed);
    }

    /// Re-initializing an undecoded state must not discard the choice that
    /// came with it.
    #[test]
    fn initializing_a_decoded_state_keeps_its_chosen_source() {
        let mut state = SpDialogState {
            port_source_idx: Some(SpPortSource::AdHoc.index()),
            ..Default::default()
        };
        state.ensure_initialized();
        assert_eq!(state.port_source(3), SpPortSource::AdHoc);
    }

    /// The run's ports are the placed ports: their numbers, their nets, their
    /// impedances. Nothing from the form's table reaches the roster.
    #[test]
    fn placed_mode_derives_the_run_ports_from_the_placed_ports() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        state.ports[0].node_pos = "TYPED_IN".to_owned();

        let config = state
            .to_config(Some(&two_placed_ports()))
            .expect("two placed ports resolve");

        assert_eq!(
            config
                .ports
                .iter()
                .map(|port| (port.number, port.z0))
                .collect::<Vec<_>>(),
            vec![(1, Some(50.0)), (2, Some(75.0))]
        );
        assert!(
            config.ports.iter().all(|port| port.node_pos != "TYPED_IN"),
            "the ad-hoc table is not consulted: {:?}",
            config.ports
        );
    }

    /// The table is retained through a placed-mode run, so switching back
    /// restores what the user wrote.
    #[test]
    fn placed_mode_retains_the_ad_hoc_table_it_is_not_reading() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        state.ports[0].node_pos = "TYPED_IN".to_owned();
        state
            .to_config(Some(&two_placed_ports()))
            .expect("two placed ports resolve");

        choosing(&mut state, SpPortSource::AdHoc);
        let config = state.to_config(Some(&[])).expect("the table resolves");
        assert_eq!(config.ports[0].node_pos, "TYPED_IN");
    }

    /// Two ports answering to one number is a matrix with no defined meaning,
    /// and the refusal names the numbers rather than picking a winner.
    #[test]
    fn duplicate_placed_port_numbers_refuse_and_name_the_numbers() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        let ports = placed(&[
            ("PA", "port=1"),
            ("PB", "port=2"),
            ("PC", "port=2"),
            ("PD", "port=3"),
            ("PE", "port=3"),
        ]);

        let error = state
            .to_config(Some(&ports))
            .expect_err("a collision refuses the run");

        assert!(
            error.contains("port numbers 2 and 3"),
            "the colliding numbers are named: {error}"
        );
        assert!(
            error.contains("PA, PB, PC, PD and PE"),
            "the ports are named: {error}"
        );
    }

    /// A gap is not a smaller matrix. The refusal names the number nothing
    /// claims, because that is the one edit that fixes it.
    #[test]
    fn a_gap_in_the_placed_port_numbers_refuses_and_names_the_gap() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        let ports = placed(&[("P1", "port=1"), ("P2", "port=2"), ("P4", "port=4")]);

        let error = state
            .to_config(Some(&ports))
            .expect_err("a gap refuses the run");

        assert!(error.contains("skip port 3"), "{error}");
        assert!(error.contains("P4 at 4"), "{error}");
        assert!(error.contains("number them 1 to 3"), "{error}");
    }

    /// Placed mode against a sheet that places nothing states what to do about
    /// it, both ways.
    #[test]
    fn placed_mode_without_placed_ports_refuses() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);

        let error = state
            .to_config(Some(&[]))
            .expect_err("no placed port is no run");
        assert!(error.contains("the sheet places none"), "{error}");
        assert!(error.contains("Ad-hoc node ports"), "{error}");

        let one = placed(&[("P1", "port=1")]);
        let error = state
            .to_config(Some(&one))
            .expect_err("one placed port is no matrix");
        assert!(error.contains("at least 2 ports"), "{error}");
    }

    /// A caller that cannot see the design must not refuse a placed-mode
    /// analysis for placing nothing it was never shown.
    #[test]
    fn placed_mode_with_the_design_out_of_view_validates_only_what_it_can_see() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);

        let config = state.to_config(None).expect("the sweep still validates");
        assert!(config.ports.is_empty());

        state.stop_freq = "1k".to_owned();
        state.start_freq = "1Meg".to_owned();
        assert!(
            state.to_config(None).is_err(),
            "an inverted sweep is still refused"
        );
    }

    /// Ad-hoc mode beside placed ports is refused rather than warned about.
    ///
    /// The deck's own `P` cards win at the solver, so the table on this form
    /// never reaches the network — but it does reach the spec's port list, and
    /// every reader of that list takes it for the run's roster. Running would
    /// report a network measured on one set of ports under another set's
    /// reference impedances.
    #[test]
    fn ad_hoc_mode_beside_placed_ports_refuses_and_says_why() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::AdHoc);

        let error = state
            .to_config(Some(&two_placed_ports()))
            .expect_err("two declarations of one thing is no run");

        assert!(error.contains("places 2 RF ports"), "{error}");
        assert!(error.contains("P1 and P2"), "{error}");
        assert!(error.contains("would not be used"), "{error}");
        assert!(error.contains("From placed RF ports"), "{error}");
    }

    /// And with nothing placed, ad-hoc mode is exactly what it always was.
    #[test]
    fn ad_hoc_mode_without_placed_ports_is_unchanged() {
        let state = dialog();

        let from_design = state.to_config(Some(&[])).expect("the table resolves");
        let blind = state.to_config(None).expect("the table resolves");

        assert_eq!(from_design.ports.len(), 2);
        assert_eq!(from_design.ports[0].node_pos, "IN");
        assert_eq!(from_design.ports[1].node_pos, "OUT");
        assert_eq!(blind.ports.len(), 2);
    }

    /// An impedance the port states as an expression is not a figure this
    /// surface can normalize to, and the `P` card carries it to the solver
    /// regardless. The description falls back to the run's reference impedance
    /// rather than inventing one.
    #[test]
    fn a_placed_impedance_that_is_not_a_number_falls_back_to_the_runs_z0() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        let ports = placed(&[("P1", "port=1 z0={ZL}"), ("P2", "port=2 z0=50")]);

        let config = state.to_config(Some(&ports)).expect("the roster resolves");

        assert_eq!(config.ports[0].z0, None);
        assert_eq!(config.ports[1].z0, Some(50.0));
    }

    /// The mode label a placed row states is the derivation's own, not a
    /// second spelling of it.
    #[test]
    fn a_placed_port_row_reads_its_mode_from_the_derivation() {
        let ports = placed(&[("P1", "port=1 ac_mag=1"), ("P2", "port=2")]);
        assert_eq!(ports[0].mode, RfPortMode::AcDrive);
        assert_eq!(ports[0].mode.label(), "AC drive");
        assert_eq!(ports[1].mode.label(), "term");
    }
}
