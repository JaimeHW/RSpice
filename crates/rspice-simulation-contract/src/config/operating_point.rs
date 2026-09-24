//! Operating-point execution settings, retained evidence bindings, and validation.

use serde::{Deserialize, Serialize};

use rspice_app_types::product::{ContentDigest, ProcessCorner};
pub use rspice_results::operating_point::OpPreviousState;

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
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpInitialGuess {
    #[default]
    Automatic,
    PreviousConverged,
    UserNodeVoltages,
    ZeroState,
    /// Use a retained solution as a fresh solve's guess after matching its MNA names.
    PreviousCompatible,
}

impl OpInitialGuess {
    pub const fn uses_previous_state(self) -> bool {
        matches!(self, Self::PreviousConverged | Self::PreviousCompatible)
    }

    pub const ALL: [Self; 5] = [
        Self::Automatic,
        Self::PreviousConverged,
        Self::UserNodeVoltages,
        Self::ZeroState,
        Self::PreviousCompatible,
    ];
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Automatic => "Automatic",
            Self::PreviousConverged => "Previous converged solution",
            Self::UserNodeVoltages => "User node voltages",
            Self::ZeroState => "Zero state",
            Self::PreviousCompatible => "Previous solution, compatible circuit",
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
}

/// The solver-effort tier, shared with every other analysis that offers one.
///
/// See [`crate::accuracy`] for the contract a tier name carries.
pub type OpAccuracy = crate::accuracy::AnalysisAccuracy;

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
            OpInitialGuess::PreviousConverged
            | OpInitialGuess::PreviousCompatible
            | OpInitialGuess::ZeroState => matches!(
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
        if self.initial_guess.uses_previous_state() && self.previous_state.is_none() {
            return Err(
                "Previous-solution startup requires a retained OP state; none is available".into(),
            );
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

#[cfg(test)]
mod tests {
    use super::*;

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
                .contains("requires a retained OP state")
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
                if initial_guess.uses_previous_state() {
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
                    OpInitialGuess::PreviousConverged
                    | OpInitialGuess::PreviousCompatible
                    | OpInitialGuess::ZeroState => matches!(
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
