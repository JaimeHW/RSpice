//! Typed DC operating-point (`.OP`) configuration.
//!
//! These values are the exact eight controls in the Simulation Studio OP
//! form.  Legacy controls remain deserialize-only at the bottom of
//! [`OpDialogState`] so old projects can be migrated without allowing the
//! retired shell to influence new execution.

use serde::{Deserialize, Serialize};

use crate::product::ContentDigest;

use crate::product::ProcessCorner;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpTemperatureMode {
    #[default]
    PvtRunSet,
    Nominal27C,
    Explicit,
    ActiveRunSetAxis,
}

impl OpTemperatureMode {
    pub const ALL: [Self; 4] = [
        Self::PvtRunSet,
        Self::Nominal27C,
        Self::Explicit,
        Self::ActiveRunSetAxis,
    ];
    /// Exact Simulation Studio choice label.
    #[cfg(test)]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::PvtRunSet => "PVT run set",
            Self::Nominal27C => "Nominal temperature \u{00b7} 27 \u{00b0}C",
            Self::Explicit => "Explicit temperature\u{2026}",
            Self::ActiveRunSetAxis => "Inherit active run-set axis",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpInitialGuess {
    #[default]
    Automatic,
    PreviousConverged,
    UserNodeVoltages,
    ZeroState,
}

impl OpInitialGuess {
    pub const ALL: [Self; 4] = [
        Self::Automatic,
        Self::PreviousConverged,
        Self::UserNodeVoltages,
        Self::ZeroState,
    ];
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Automatic => "Automatic",
            Self::PreviousConverged => "Previous converged solution",
            Self::UserNodeVoltages => "User node voltages",
            Self::ZeroState => "Zero state",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpNodeInitialization {
    #[default]
    UseIcAndNodeset,
    IgnoreIcAndNodeset,
    ForceIcValues,
    ValidateOnly,
}

impl OpNodeInitialization {
    pub const ALL: [Self; 4] = [
        Self::UseIcAndNodeset,
        Self::IgnoreIcAndNodeset,
        Self::ForceIcValues,
        Self::ValidateOnly,
    ];
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::UseIcAndNodeset => "Use IC / nodeset",
            Self::IgnoreIcAndNodeset => "Ignore IC and nodeset",
            Self::ForceIcValues => "Force .ic values",
            Self::ValidateOnly => "Validate initialization only",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpHomotopy {
    #[default]
    Adaptive,
    SourceStepping,
    GminStepping,
    PseudoTransient,
    None,
}

impl OpHomotopy {
    pub const ALL: [Self; 5] = [
        Self::Adaptive,
        Self::SourceStepping,
        Self::GminStepping,
        Self::PseudoTransient,
        Self::None,
    ];
    /// Exact Simulation Studio choice label.
    #[cfg(test)]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Adaptive => "Adaptive",
            Self::SourceStepping => "Source stepping",
            Self::GminStepping => "Gmin stepping",
            Self::PseudoTransient => "Pseudo-transient",
            Self::None => "None",
        }
    }

    /// Whether this choice assigns the continuation aids itself.
    ///
    /// [`Self::Adaptive`] states no opinion, so whatever the deck's
    /// `.OPTIONS` and the accuracy tier resolved to is what the solve uses.
    /// Every other choice is an instruction, and it is applied *after* both,
    /// which is what makes it an owner: an authored `GMINSTEPPING` under any
    /// of them would be read from the deck and then overwritten.
    #[must_use]
    pub const fn owns_continuation_aids(self) -> bool {
        !matches!(self, Self::Adaptive)
    }

    /// Assign the continuation aids this choice owns, on top of an already
    /// resolved configuration.
    ///
    /// One writer, two readers: `engine_bridge::dc` builds the operating
    /// point's engine from the configuration this leaves behind, and the
    /// advanced-options panel reports the same fields as that analysis's
    /// effective values. A second copy of this match at either site is how
    /// the panel would come to state a flag the solve does not use.
    pub fn apply(self, config: &mut rspice_core::SimulationConfig) {
        use rspice_core::config::NonlinearContinuationMode;

        let convergence = &mut config.convergence_config;
        let (source, gmin, pseudo, continuation) = match self {
            Self::Adaptive => return,
            Self::SourceStepping => (
                true,
                false,
                false,
                Some(NonlinearContinuationMode::SimultaneousSourceStep),
            ),
            Self::GminStepping => (false, true, false, None),
            Self::PseudoTransient => (false, false, true, None),
            Self::None => (false, false, false, None),
        };
        convergence.source_stepping = source;
        convergence.gmin_stepping = gmin;
        convergence.pseudo_transient = pseudo;
        // No choice offered here is arc-length continuation, so every one of
        // them turns it off rather than leaving a fourth aid running that the
        // reader did not select.
        convergence.arc_length = false;
        convergence.nonlinear_continuation = continuation;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpAnnotation {
    #[default]
    VoltagesAndCurrents,
    VoltagesOnly,
    VoltagesAndDeviceOp,
    None,
}

impl OpAnnotation {
    pub const ALL: [Self; 4] = [
        Self::VoltagesAndCurrents,
        Self::VoltagesOnly,
        Self::VoltagesAndDeviceOp,
        Self::None,
    ];
    /// Exact Simulation Studio choice label.
    #[cfg(test)]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::VoltagesAndCurrents => "Voltages + currents",
            Self::VoltagesOnly => "Voltages only",
            Self::VoltagesAndDeviceOp => "Voltages + device OP",
            Self::None => "None",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpDeviceDetail {
    #[default]
    SelectedAndViolations,
    AllDevices,
    ViolationsOnly,
    None,
}

impl OpDeviceDetail {
    pub const ALL: [Self; 4] = [
        Self::SelectedAndViolations,
        Self::AllDevices,
        Self::ViolationsOnly,
        Self::None,
    ];
    #[cfg(test)]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::SelectedAndViolations => "Selected + violations",
            Self::AllDevices => "All devices",
            Self::ViolationsOnly => "Violations only",
            Self::None => "None",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpSaveDevice {
    #[default]
    Enabled,
    Disabled,
    FinalPointOnly,
}

impl OpSaveDevice {
    pub const ALL: [Self; 3] = [Self::Enabled, Self::Disabled, Self::FinalPointOnly];
    #[cfg(test)]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
            Self::FinalPointOnly => "Final point only",
        }
    }
}

/// The solver-effort tier, shared with every other analysis that offers one.
///
/// See [`crate::simulation::accuracy`] for the contract a tier name carries.
pub type OpAccuracy = crate::simulation::accuracy::AnalysisAccuracy;

/// Complete, identity-bound MNA state retained by an earlier accepted OP.
/// Node order excludes ground and is followed by branch order in `solution`,
/// exactly matching the core engine's MNA vector contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpPreviousState {
    pub source_content_digest: ContentDigest,
    pub producer_snapshot_digest: ContentDigest,
    pub producer_result_digest: ContentDigest,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub solution: Vec<f64>,
}

impl OpPreviousState {
    fn validate(&self) -> Result<(), String> {
        let valid_identity = |name: &str| {
            !name.is_empty() && name.trim() == name && !name.chars().any(char::is_whitespace)
        };
        if self.node_names.iter().any(|name| !valid_identity(name))
            || self.branch_names.iter().any(|name| !valid_identity(name))
            || self
                .node_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("0"))
        {
            return Err(
                "Previous operating-point MNA ordering contains an invalid identity".into(),
            );
        }
        let unique = |names: &[String]| {
            let mut identities = std::collections::HashSet::with_capacity(names.len());
            names
                .iter()
                .all(|name| identities.insert(name.to_ascii_lowercase()))
        };
        if !unique(&self.node_names) || !unique(&self.branch_names) {
            return Err(
                "Previous operating-point MNA ordering contains duplicate identities".into(),
            );
        }
        if self
            .node_names
            .len()
            .saturating_add(self.branch_names.len())
            != self.solution.len()
            || self.solution.is_empty()
            || self.solution.iter().any(|value| !value.is_finite())
        {
            return Err("Previous operating-point MNA state is incomplete or non-finite".into());
        }
        Ok(())
    }
}

/// Position of this OP inside its bound run-point sequence. A standalone OP
/// has one point at index zero, which is also its final point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpRunPointContext {
    pub index: usize,
    pub count: usize,
    /// Exact process corner bound to this dispatched point.
    #[serde(default)]
    pub process: ProcessCorner,
    /// Exact supply voltage and reference voltage used to scale independent
    /// DC supplies for this point. These are either both present or both
    /// absent.
    #[serde(default)]
    pub supply_voltage: Option<f64>,
    #[serde(default)]
    pub nominal_supply_voltage: Option<f64>,
    /// Exact independent voltage-source instances forming the supply domain.
    #[serde(default)]
    pub supply_source_names: Vec<String>,
}

impl Default for OpRunPointContext {
    fn default() -> Self {
        Self {
            index: 0,
            count: 1,
            process: ProcessCorner::TT,
            supply_voltage: None,
            nominal_supply_voltage: None,
            supply_source_names: Vec::new(),
        }
    }
}

impl OpRunPointContext {
    #[must_use]
    pub const fn is_final(&self) -> bool {
        self.count > 0 && self.index + 1 == self.count
    }
}

/// Exact execution and retention policy for a single operating-point task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpConfig {
    pub temperature_mode: OpTemperatureMode,
    /// Resolved temperature carried into the immutable task, in Celsius.
    pub temperature_celsius: f64,
    pub initial_guess: OpInitialGuess,
    pub node_initialization: OpNodeInitialization,
    pub homotopy: OpHomotopy,
    pub annotation: OpAnnotation,
    pub device_detail: OpDeviceDetail,
    pub save_device_op: OpSaveDevice,
    pub accuracy: OpAccuracy,
    /// Stable netlist instance names captured from the schematic selection
    /// when the immutable task is prepared.
    #[serde(default)]
    pub selected_devices: Vec<String>,
    /// Complete prior state selected from retained history when the authored
    /// initial-guess policy requests it.
    #[serde(default)]
    pub previous_state: Option<OpPreviousState>,
    /// Canonical device identities supplied by retained SOA evidence.
    #[serde(default)]
    pub violation_devices: Vec<String>,
    /// Executable-source identity of the retained SOA evidence supplying
    /// `violation_devices`. Present exactly when that list is non-empty.
    #[serde(default)]
    pub violation_source_content_digest: Option<ContentDigest>,
    /// Exact run-point position used by final-point-only retention.
    #[serde(default)]
    pub run_point: OpRunPointContext,
}

impl Default for OpConfig {
    fn default() -> Self {
        Self {
            temperature_mode: OpTemperatureMode::PvtRunSet,
            temperature_celsius: 27.0,
            initial_guess: OpInitialGuess::Automatic,
            node_initialization: OpNodeInitialization::UseIcAndNodeset,
            homotopy: OpHomotopy::Adaptive,
            annotation: OpAnnotation::VoltagesAndCurrents,
            device_detail: OpDeviceDetail::SelectedAndViolations,
            save_device_op: OpSaveDevice::Enabled,
            accuracy: OpAccuracy::Balanced,
            selected_devices: Vec::new(),
            previous_state: None,
            violation_devices: Vec::new(),
            violation_source_content_digest: None,
            run_point: OpRunPointContext::default(),
        }
    }
}

impl OpConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.temperature_celsius.is_finite() || self.temperature_celsius <= -273.15 {
            return Err(
                "Operating-point temperature must be finite and above absolute zero".into(),
            );
        }
        if self.selected_devices.iter().any(|name| {
            name.is_empty() || name.trim() != name || name.chars().any(char::is_whitespace)
        }) {
            return Err(
                "Selected operating-point device identities must be canonical netlist names".into(),
            );
        }
        if self
            .selected_devices
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        {
            return Err(
                "Selected operating-point device identities must be sorted and unique".into(),
            );
        }
        if self.violation_devices.is_empty() != self.violation_source_content_digest.is_none() {
            return Err(
                "Operating-point SOA device evidence must carry its executable-source identity"
                    .into(),
            );
        }
        if self.violation_devices.iter().any(|name| {
            name.is_empty() || name.trim() != name || name.chars().any(char::is_whitespace)
        }) || self
            .violation_devices
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        {
            return Err(
                "Operating-point violation device identities must be canonical, sorted, and unique"
                    .into(),
            );
        }
        if let Some(previous) = &self.previous_state {
            previous.validate()?;
        }
        if self.run_point.count == 0 || self.run_point.index >= self.run_point.count {
            return Err(
                "Operating-point run-point position is outside its retained sequence".into(),
            );
        }
        match (
            self.run_point.supply_voltage,
            self.run_point.nominal_supply_voltage,
        ) {
            (None, None) => {}
            (Some(supply), Some(nominal))
                if supply.is_finite() && supply > 0.0 && nominal.is_finite() && nominal > 0.0 => {}
            _ => {
                return Err(
                    "Operating-point PVT supply and nominal voltages must be paired positive finite values"
                        .into(),
                );
            }
        }
        let startup_compatible = match self.initial_guess {
            OpInitialGuess::Automatic => true,
            OpInitialGuess::PreviousConverged | OpInitialGuess::ZeroState => matches!(
                self.node_initialization,
                OpNodeInitialization::IgnoreIcAndNodeset | OpNodeInitialization::ValidateOnly
            ),
            OpInitialGuess::UserNodeVoltages => matches!(
                self.node_initialization,
                OpNodeInitialization::UseIcAndNodeset | OpNodeInitialization::ForceIcValues
            ),
        };
        if !startup_compatible {
            return Err(format!(
                "Operating-point initial guess '{}' is incompatible with node initialization '{}'",
                self.initial_guess.display_name(),
                self.node_initialization.display_name()
            ));
        }
        Ok(())
    }

    /// Validate contextual bindings after immutable task preparation.
    pub fn validate_for_execution(&self) -> Result<(), String> {
        self.validate()?;
        if self.initial_guess == OpInitialGuess::PreviousConverged && self.previous_state.is_none()
        {
            return Err("Previous converged solution requires an identity-compatible retained OP state; none is available".into());
        }
        if self.device_detail == OpDeviceDetail::ViolationsOnly
            && (self.violation_devices.is_empty() || self.violation_source_content_digest.is_none())
        {
            return Err("Violations-only device detail requires retained SOA warning or violation evidence; none is available".into());
        }
        Ok(())
    }

    pub fn to_spice(&self) -> String {
        ".op".to_owned()
    }
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpDialogState {
    #[serde(default = "missing_selection_index")]
    pub temperature_mode_idx: usize,
    #[serde(default = "default_temperature")]
    pub temperature: String,
    #[serde(default)]
    pub initial_guess_idx: usize,
    #[serde(default)]
    pub node_initialization_idx: usize,
    #[serde(default)]
    pub homotopy_idx: usize,
    #[serde(default)]
    pub annotation_idx: usize,
    #[serde(default)]
    pub device_detail_idx: usize,
    #[serde(default)]
    pub save_device_op_idx: usize,
    #[serde(default = "default_accuracy_idx")]
    pub accuracy_idx: usize,

    #[serde(default, skip_serializing)]
    pub save_all: Option<bool>,
    #[serde(default, skip_serializing)]
    pub save_op_info: Option<bool>,
    #[serde(default, skip_serializing)]
    pub source_stepping: Option<bool>,
    #[serde(default, skip_serializing)]
    pub gmin_steps: Option<String>,
    #[serde(skip)]
    pub initialized: bool,
}

impl Default for OpDialogState {
    fn default() -> Self {
        Self::from_config(&OpConfig::default())
    }
}

impl OpDialogState {
    pub fn from_config(config: &OpConfig) -> Self {
        Self {
            temperature_mode_idx: index_of(&OpTemperatureMode::ALL, config.temperature_mode),
            temperature: config.temperature_celsius.to_string(),
            initial_guess_idx: index_of(&OpInitialGuess::ALL, config.initial_guess),
            node_initialization_idx: index_of(
                &OpNodeInitialization::ALL,
                config.node_initialization,
            ),
            homotopy_idx: index_of(&OpHomotopy::ALL, config.homotopy),
            annotation_idx: index_of(&OpAnnotation::ALL, config.annotation),
            device_detail_idx: index_of(&OpDeviceDetail::ALL, config.device_detail),
            save_device_op_idx: index_of(&OpSaveDevice::ALL, config.save_device_op),
            accuracy_idx: index_of(&OpAccuracy::ALL, config.accuracy),
            save_all: None,
            save_op_info: None,
            source_stepping: None,
            gmin_steps: None,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<OpConfig, String> {
        if let Some(legacy_gmin_steps) = self.gmin_steps.as_deref()
            && legacy_gmin_steps.trim().parse::<usize>().is_err()
        {
            return Err(
                "Legacy operating-point gmin_steps must be a non-negative integer".to_owned(),
            );
        }
        let temperature_mode = selected(
            &OpTemperatureMode::ALL,
            self.temperature_mode_idx,
            "temperature",
        )?;
        let temperature_celsius = match temperature_mode {
            OpTemperatureMode::Nominal27C => 27.0,
            _ => self
                .temperature
                .parse()
                .map_err(|_| "Invalid operating-point temperature")?,
        };
        let config = OpConfig {
            temperature_mode,
            temperature_celsius,
            initial_guess: selected(
                &OpInitialGuess::ALL,
                self.initial_guess_idx,
                "initial guess",
            )?,
            node_initialization: selected(
                &OpNodeInitialization::ALL,
                self.node_initialization_idx,
                "node initialization",
            )?,
            homotopy: selected(&OpHomotopy::ALL, self.homotopy_idx, "homotopy strategy")?,
            annotation: selected(&OpAnnotation::ALL, self.annotation_idx, "annotation")?,
            device_detail: selected(
                &OpDeviceDetail::ALL,
                self.device_detail_idx,
                "device detail",
            )?,
            save_device_op: selected(
                &OpSaveDevice::ALL,
                self.save_device_op_idx,
                "device OP save",
            )?,
            accuracy: selected(&OpAccuracy::ALL, self.accuracy_idx, "accuracy")?,
            selected_devices: Vec::new(),
            previous_state: None,
            violation_devices: Vec::new(),
            violation_source_content_digest: None,
            run_point: OpRunPointContext::default(),
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::default();
        }
    }

    pub fn prepare_after_restore(&mut self) {
        // Plans written before temperature-source selection existed carried
        // an authored scalar temperature. Preserve that value as explicit;
        // treating it as the new index-zero PVT policy would silently replace
        // it with the workspace reference temperature during preparation.
        if self.temperature_mode_idx == usize::MAX {
            self.temperature_mode_idx =
                index_of(&OpTemperatureMode::ALL, OpTemperatureMode::Explicit);
        }
        if self.source_stepping == Some(true) {
            self.homotopy_idx = index_of(&OpHomotopy::ALL, OpHomotopy::SourceStepping);
        } else if self
            .gmin_steps
            .as_deref()
            .and_then(|value| value.trim().parse::<usize>().ok())
            .is_some_and(|value| value > 0)
        {
            self.homotopy_idx = index_of(&OpHomotopy::ALL, OpHomotopy::GminStepping);
        }
        if let Some(save_all) = self.save_all {
            self.annotation_idx = index_of(
                &OpAnnotation::ALL,
                if save_all {
                    OpAnnotation::VoltagesAndCurrents
                } else {
                    OpAnnotation::None
                },
            );
        }
        if let Some(save_op) = self.save_op_info {
            self.device_detail_idx = index_of(
                &OpDeviceDetail::ALL,
                if save_op {
                    OpDeviceDetail::AllDevices
                } else {
                    OpDeviceDetail::None
                },
            );
            self.save_device_op_idx = index_of(
                &OpSaveDevice::ALL,
                if save_op {
                    OpSaveDevice::Enabled
                } else {
                    OpSaveDevice::Disabled
                },
            );
        }
        clamp_index(&mut self.temperature_mode_idx, OpTemperatureMode::ALL.len());
        clamp_index(&mut self.initial_guess_idx, OpInitialGuess::ALL.len());
        clamp_index(
            &mut self.node_initialization_idx,
            OpNodeInitialization::ALL.len(),
        );
        clamp_index(&mut self.homotopy_idx, OpHomotopy::ALL.len());
        clamp_index(&mut self.annotation_idx, OpAnnotation::ALL.len());
        clamp_index(&mut self.device_detail_idx, OpDeviceDetail::ALL.len());
        clamp_index(&mut self.save_device_op_idx, OpSaveDevice::ALL.len());
        clamp_index(&mut self.accuracy_idx, OpAccuracy::ALL.len());
        self.initialized = true;
    }
}

fn selected<T: Copy>(values: &[T], index: usize, label: &str) -> Result<T, String> {
    values
        .get(index)
        .copied()
        .ok_or_else(|| format!("Operating-point {label} selection is invalid"))
}

fn index_of<T: PartialEq>(values: &[T], value: T) -> usize {
    values
        .iter()
        .position(|candidate| *candidate == value)
        .unwrap_or(0)
}

fn clamp_index(index: &mut usize, len: usize) {
    *index = (*index).min(len.saturating_sub(1));
}

fn default_temperature() -> String {
    "27".to_owned()
}
const fn missing_selection_index() -> usize {
    usize::MAX
}
const fn default_accuracy_idx() -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mockup_contract_defaults_and_choice_labels_are_exact() {
        let state = OpDialogState::default();
        let config = state.to_config().expect("default OP contract");
        assert_eq!(config, OpConfig::default());
        assert_eq!(
            OpTemperatureMode::ALL.map(OpTemperatureMode::display_name),
            [
                "PVT run set",
                "Nominal temperature \u{00b7} 27 \u{00b0}C",
                "Explicit temperature\u{2026}",
                "Inherit active run-set axis"
            ]
        );
        assert_eq!(
            OpInitialGuess::ALL.map(OpInitialGuess::display_name),
            [
                "Automatic",
                "Previous converged solution",
                "User node voltages",
                "Zero state"
            ]
        );
        assert_eq!(
            OpNodeInitialization::ALL.map(OpNodeInitialization::display_name),
            [
                "Use IC / nodeset",
                "Ignore IC and nodeset",
                "Force .ic values",
                "Validate initialization only"
            ]
        );
        assert_eq!(
            OpHomotopy::ALL.map(OpHomotopy::display_name),
            [
                "Adaptive",
                "Source stepping",
                "Gmin stepping",
                "Pseudo-transient",
                "None"
            ]
        );
        assert_eq!(
            OpAnnotation::ALL.map(OpAnnotation::display_name),
            [
                "Voltages + currents",
                "Voltages only",
                "Voltages + device OP",
                "None"
            ]
        );
        assert_eq!(
            OpDeviceDetail::ALL.map(OpDeviceDetail::display_name),
            [
                "Selected + violations",
                "All devices",
                "Violations only",
                "None"
            ]
        );
        assert_eq!(
            OpSaveDevice::ALL.map(OpSaveDevice::display_name),
            ["Enabled", "Disabled", "Final point only"]
        );
        assert_eq!(
            OpAccuracy::ALL.map(OpAccuracy::display_name),
            ["Fast", "Balanced", "Accurate", "Robust"]
        );
    }

    #[test]
    fn legacy_controls_migrate_and_never_serialize_again() {
        let mut restored: OpDialogState = serde_json::from_str(r#"{"save_all":false,"save_op_info":true,"temperature":"88","source_stepping":true,"gmin_steps":"0"}"#).unwrap();
        restored.prepare_after_restore();
        assert_eq!(restored.temperature, "88");
        assert_eq!(
            restored.temperature_mode_idx,
            index_of(&OpTemperatureMode::ALL, OpTemperatureMode::Explicit)
        );
        assert_eq!(restored.to_config().unwrap().temperature_celsius, 88.0);
        assert_eq!(restored.homotopy_idx, 1);
        assert_eq!(restored.annotation_idx, 3);
        assert_eq!(restored.device_detail_idx, 1);
        let encoded = serde_json::to_value(restored).unwrap();
        for retired in ["save_all", "save_op_info", "source_stepping", "gmin_steps"] {
            assert!(encoded.get(retired).is_none(), "{retired}");
        }
    }

    #[test]
    fn corrupt_legacy_gmin_steps_fails_closed() {
        for value in ["", "abc", "-1", "1.5"] {
            let mut restored: OpDialogState =
                serde_json::from_str(&format!(r#"{{"temperature":"27","gmin_steps":"{value}"}}"#))
                    .unwrap();
            restored.prepare_after_restore();
            assert!(
                restored.to_config().unwrap_err().contains("gmin_steps"),
                "legacy value {value:?} must not silently select a homotopy mode"
            );
        }
    }

    #[test]
    fn contextual_choices_fail_closed_until_real_evidence_is_bound() {
        let mut config = OpConfig {
            initial_guess: OpInitialGuess::PreviousConverged,
            node_initialization: OpNodeInitialization::IgnoreIcAndNodeset,
            ..OpConfig::default()
        };
        assert!(
            config
                .validate_for_execution()
                .unwrap_err()
                .contains("identity-compatible retained OP state")
        );
        config.previous_state = Some(OpPreviousState {
            source_content_digest: ContentDigest::from_bytes([1; 32]),
            producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
            producer_result_digest: ContentDigest::from_bytes([3; 32]),
            node_names: vec!["out".to_owned()],
            branch_names: vec!["V1".to_owned()],
            solution: vec![0.5, -0.5e-3],
        });
        config
            .validate_for_execution()
            .expect("complete prior state enables the policy");

        config.initial_guess = OpInitialGuess::Automatic;
        config.device_detail = OpDeviceDetail::ViolationsOnly;
        assert!(
            config
                .validate_for_execution()
                .unwrap_err()
                .contains("SOA warning or violation evidence")
        );
        config.violation_devices = vec!["M1".to_owned()];
        config.violation_source_content_digest = Some(ContentDigest::from_bytes([1; 32]));
        config
            .validate_for_execution()
            .expect("retained SOA identities enable violations-only detail");
    }

    #[test]
    fn previous_state_rejects_case_insensitive_duplicate_mna_identities() {
        let previous = OpPreviousState {
            source_content_digest: ContentDigest::from_bytes([1; 32]),
            producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
            producer_result_digest: ContentDigest::from_bytes([3; 32]),
            node_names: vec!["out".to_owned(), "OUT".to_owned()],
            branch_names: vec!["V1".to_owned()],
            solution: vec![0.5, 0.5, -0.5e-3],
        };
        assert!(previous.validate().unwrap_err().contains("duplicate"));
    }

    #[test]
    fn final_point_only_has_exact_single_and_multi_point_semantics() {
        let standalone = OpRunPointContext::default();
        assert!(standalone.is_final());
        assert!(
            !OpRunPointContext {
                index: 0,
                count: 2,
                ..OpRunPointContext::default()
            }
            .is_final()
        );
        assert!(
            OpRunPointContext {
                index: 1,
                count: 2,
                ..OpRunPointContext::default()
            }
            .is_final()
        );
    }

    #[test]
    fn startup_policy_matrix_rejects_every_semantically_conflicting_pair() {
        for initial_guess in OpInitialGuess::ALL {
            for node_initialization in OpNodeInitialization::ALL {
                let mut config = OpConfig {
                    initial_guess,
                    node_initialization,
                    ..OpConfig::default()
                };
                if initial_guess == OpInitialGuess::PreviousConverged {
                    config.previous_state = Some(OpPreviousState {
                        source_content_digest: ContentDigest::from_bytes([1; 32]),
                        producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
                        producer_result_digest: ContentDigest::from_bytes([3; 32]),
                        node_names: vec!["out".to_owned()],
                        branch_names: Vec::new(),
                        solution: vec![0.5],
                    });
                }
                let expected_valid = match initial_guess {
                    OpInitialGuess::Automatic => true,
                    OpInitialGuess::PreviousConverged | OpInitialGuess::ZeroState => matches!(
                        node_initialization,
                        OpNodeInitialization::IgnoreIcAndNodeset
                            | OpNodeInitialization::ValidateOnly
                    ),
                    OpInitialGuess::UserNodeVoltages => matches!(
                        node_initialization,
                        OpNodeInitialization::UseIcAndNodeset | OpNodeInitialization::ForceIcValues
                    ),
                };
                assert_eq!(
                    config.validate_for_execution().is_ok(),
                    expected_valid,
                    "{} + {}",
                    initial_guess.display_name(),
                    node_initialization.display_name()
                );
            }
        }
    }
}
