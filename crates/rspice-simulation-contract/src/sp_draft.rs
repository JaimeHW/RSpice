//! S-parameter port dialog state.

use crate::options::parse_si_value;
use crate::sp_config::{SpConfig, SpPortConfig, SpSweepType};
use rspice_app_types::quantity::parse_engineering_value;
use std::collections::HashMap;

/// Borrowed schematic port fields needed to validate an S-parameter run.
/// The app projects its placed RF ports here without moving schematic state
/// into the portable analysis contract.
#[derive(Debug, Clone, Copy)]
pub struct SpPlacedPort<'a> {
    pub reference: &'a str,
    pub port_number: u32,
    pub z0: &'a str,
    pub nets: &'a [String],
}

/// Sorted port numbers claimed more than once by a design roster.
#[must_use]
pub fn duplicate_port_numbers(numbers: impl IntoIterator<Item = u32>) -> Vec<u32> {
    let mut claims: HashMap<u32, usize> = HashMap::new();
    for number in numbers {
        *claims.entry(number).or_default() += 1;
    }
    let mut duplicates: Vec<u32> = claims
        .into_iter()
        .filter(|(_, claimed)| *claimed > 1)
        .map(|(number, _)| number)
        .collect();
    duplicates.sort_unstable();
    duplicates
}

/// Where an S-parameter run's ports come from.
///
/// The schematic places RF Port components, or this form names node pairs.
/// The engine resolves authored ports during circuit elaboration and uses
/// configured pairs only when that circuit declares none. Solved references
/// travel with the result and qualify Touchstone export and impedance readouts.
/// This choice controls which configuration the form edits and validates.
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
    fn single_ended(node_pos: impl Into<String>) -> Self {
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    /// Touchstone format version the export writes: `1` for an unversioned v1
    /// file, `2` for a `[Version] 2.0` file.
    ///
    /// Defaulted so a project saved before the chooser existed opens on the
    /// version its exports were already being written in, rather than on
    /// `u32::default()`, which is not a format at all.
    #[serde(default = "default_touchstone_version")]
    pub touchstone_version: u32,
    /// Initialized flag
    #[serde(skip)]
    pub initialized: bool,
}

impl Default for SpDialogState {
    fn default() -> Self {
        Self::from_config(&SpConfig::default())
    }
}

/// The version an export was written in before the form could choose one.
fn default_touchstone_version() -> u32 {
    SpConfig::default().touchstone_version
}

/// The two versions the export writer can actually produce, in chooser order.
///
/// Both are real paths through `WaveformWriter::touchstone_text`: v1 opens on
/// the option line alone, v2 on `[Version] 2.0` with the keyword block the
/// specification requires. The chooser offers exactly these because offering a
/// third would be offering a file the writer cannot write. v1 additionally
/// cannot carry per-port reference impedances, and the writer refuses that
/// combination by name rather than dropping them.
pub const TOUCHSTONE_VERSIONS: [u32; 2] = [1, 2];

/// Chooser labels for [`TOUCHSTONE_VERSIONS`], in the same order.
///
/// A v1 file states no version of its own, so it is spelled as the writer
/// writes it rather than as a specification revision the file never names.
pub const TOUCHSTONE_VERSION_LABELS: [&str; 2] = ["v1", "v2.0"];

impl SpDialogState {
    fn default_port_node(index: usize) -> String {
        match index {
            0 => "IN".to_string(),
            1 => "OUT".to_string(),
            _ => format!("P{}", index + 1),
        }
    }

    fn ensure_min_ports(&mut self) {
        if self.ports.is_empty() {
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

    /// The Touchstone version this draft writes, as an index into
    /// [`TOUCHSTONE_VERSIONS`].
    ///
    /// A stored value the writer does not produce resolves to the nearer
    /// offered one, the way [`Self::to_config`] already clamps it, so the
    /// chooser never paints a position that is not one of its own.
    #[must_use]
    pub fn touchstone_version_index(&self) -> usize {
        usize::from(self.touchstone_version >= 2)
    }

    /// Record a version the chooser selected.
    pub fn set_touchstone_version_index(&mut self, index: usize) {
        if let Some(version) = TOUCHSTONE_VERSIONS.get(index) {
            self.touchstone_version = *version;
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
    pub fn to_config(&self, design: Option<&[SpPlacedPort<'_>]>) -> Result<SpConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;
        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;
        let points: u32 = self
            .num_points
            .trim()
            .parse()
            .map_err(|_| "Invalid point count")?;
        let z0 = parse_si_value(&self.z0).map_err(|error| format!("Invalid Z0: {error}"))?;

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
    fn resolve_ports(
        &self,
        design: Option<&[SpPlacedPort<'_>]>,
    ) -> Result<Vec<SpPortConfig>, String> {
        match (self.port_source(design.map_or(0, <[_]>::len)), design) {
            // Validate the visible placed-port roster. The engine resolves
            // the complete authored circuit and returns its actual references.
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
    pub fn port_roster_error(&self, placed: &[SpPlacedPort<'_>]) -> Option<String> {
        self.resolve_ports(Some(placed)).err()
    }

    /// Complete form initialization without replacing authored values.
    pub fn ensure_initialized(&mut self) {
        // Defaults are assigned at construction. Decoded drafts already carry
        // authored settings, including intentionally invalid input being edited.
        self.initialized = true;
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
fn placed_port_configs(placed: &[SpPlacedPort<'_>]) -> Result<Vec<SpPortConfig>, String> {
    if placed.is_empty() {
        return Err(no_placed_ports());
    }
    let duplicates = duplicate_port_numbers(placed.iter().map(|port| port.port_number));
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
            z0: parse_engineering_value(port.z0)
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
fn terminal(port: &SpPlacedPort<'_>, pin: usize) -> String {
    port.nets
        .get(pin)
        .cloned()
        .unwrap_or_else(|| "unconnected".to_owned())
}

/// The first number an `N`-port matrix needs and the placed ports do not claim.
fn first_missing_port_number(placed: &[SpPlacedPort<'_>]) -> Option<u32> {
    (1..=placed.len() as u32)
        .find(|expected| !placed.iter().any(|port| port.port_number == *expected))
}

/// `P1, P3 and P4`, the ports a sentence is about.
fn port_list(placed: &[SpPlacedPort<'_>]) -> String {
    join_words(placed.iter().map(|port| port.reference.to_owned()))
}

fn join_words(words: impl IntoIterator<Item = String>) -> String {
    let words: Vec<String> = words.into_iter().collect();
    match words.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

fn no_placed_ports() -> String {
    "This analysis reads the design's RF ports and the sheet places none. Place RF Port \
         components, or set Ports to Ad-hoc node ports and name the port nodes here."
        .to_owned()
}

fn duplicate_placed_port_numbers(placed: &[SpPlacedPort<'_>], duplicates: &[u32]) -> String {
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

fn non_contiguous_placed_port_numbers(placed: &[SpPlacedPort<'_>], gap: u32) -> String {
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

fn ad_hoc_beside_placed_ports(placed: &[SpPlacedPort<'_>]) -> String {
    format!(
        "The sheet places {} RF port{} ({}), and a deck's own P cards are what an S-parameter \
         run measures, so the node ports on this form would not be used. Set Ports to From \
         placed RF ports, or delete the placed RF Port components.",
        placed.len(),
        if placed.len() == 1 { "" } else { "s" },
        port_list(placed),
    )
}

/// Frequency formatting for the S-parameter dialog.
fn format_freq(freq: f64) -> String {
    if freq >= 1e9 {
        format!("{}G", freq / 1e9)
    } else if freq >= 1e6 {
        format!("{}Meg", freq / 1e6)
    } else if freq >= 1e3 {
        format!("{}k", freq / 1e3)
    } else {
        format!("{}", freq)
    }
}
