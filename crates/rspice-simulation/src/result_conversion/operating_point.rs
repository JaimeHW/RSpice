//! Retained operating-point options and exact solved state.

use rspice_results::analysis_payload::AnalysisResultPayload;

pub(super) fn operating_point_payload(
    config: &rspice_simulation_contract::config::OpConfig,
    validated_startup_directives: usize,
    mna_node_names: Vec<String>,
    mna_branch_names: Vec<String>,
    mna_solution: Vec<f64>,
) -> rspice_results::analysis_payload::AnalysisResultPayload {
    use rspice_results::operating_point::*;
    use rspice_simulation_contract::config::*;
    AnalysisResultPayload::OperatingPoint {
        temperature_mode: match config.temperature_mode {
            OpTemperatureMode::PvtRunSet => OperatingPointTemperatureEvidence::PvtRunSet,
            OpTemperatureMode::Nominal27C => OperatingPointTemperatureEvidence::Nominal27C,
            OpTemperatureMode::Explicit => OperatingPointTemperatureEvidence::Explicit,
            OpTemperatureMode::ActiveRunSetAxis => {
                OperatingPointTemperatureEvidence::ActiveRunSetAxis
            }
        },
        temperature_celsius: config.temperature_celsius,
        initial_guess: match config.initial_guess {
            OpInitialGuess::Automatic => OperatingPointInitialGuessEvidence::Automatic,
            OpInitialGuess::PreviousConverged => {
                OperatingPointInitialGuessEvidence::PreviousConverged
            }
            OpInitialGuess::UserNodeVoltages => {
                OperatingPointInitialGuessEvidence::UserNodeVoltages
            }
            OpInitialGuess::ZeroState => OperatingPointInitialGuessEvidence::ZeroState,
            OpInitialGuess::PreviousCompatible => {
                OperatingPointInitialGuessEvidence::PreviousCompatible
            }
        },
        node_initialization: match config.node_initialization {
            OpNodeInitialization::UseIcAndNodeset => {
                OperatingPointNodeInitializationEvidence::UseIcAndNodeset
            }
            OpNodeInitialization::IgnoreIcAndNodeset => {
                OperatingPointNodeInitializationEvidence::IgnoreIcAndNodeset
            }
            OpNodeInitialization::ForceIcValues => {
                OperatingPointNodeInitializationEvidence::ForceIcValues
            }
            OpNodeInitialization::ValidateOnly => {
                OperatingPointNodeInitializationEvidence::ValidateOnly
            }
        },
        homotopy: match config.homotopy {
            OpHomotopy::Adaptive => OperatingPointHomotopyEvidence::Adaptive,
            OpHomotopy::SourceStepping => OperatingPointHomotopyEvidence::SourceStepping,
            OpHomotopy::GminStepping => OperatingPointHomotopyEvidence::GminStepping,
            OpHomotopy::PseudoTransient => OperatingPointHomotopyEvidence::PseudoTransient,
            OpHomotopy::None => OperatingPointHomotopyEvidence::None,
        },
        annotation: match config.annotation {
            OpAnnotation::VoltagesAndCurrents => {
                OperatingPointAnnotationEvidence::VoltagesAndCurrents
            }
            OpAnnotation::VoltagesOnly => OperatingPointAnnotationEvidence::VoltagesOnly,
            OpAnnotation::VoltagesAndDeviceOp => {
                OperatingPointAnnotationEvidence::VoltagesAndDeviceOp
            }
            OpAnnotation::None => OperatingPointAnnotationEvidence::None,
        },
        device_detail: match config.device_detail {
            OpDeviceDetail::SelectedAndViolations => {
                OperatingPointDeviceDetailEvidence::SelectedAndViolations
            }
            OpDeviceDetail::AllDevices => OperatingPointDeviceDetailEvidence::AllDevices,
            OpDeviceDetail::ViolationsOnly => OperatingPointDeviceDetailEvidence::ViolationsOnly,
            OpDeviceDetail::None => OperatingPointDeviceDetailEvidence::None,
        },
        save_device_op: match config.save_device_op {
            OpSaveDevice::Enabled => OperatingPointSaveDeviceEvidence::Enabled,
            OpSaveDevice::Disabled => OperatingPointSaveDeviceEvidence::Disabled,
            OpSaveDevice::FinalPointOnly => OperatingPointSaveDeviceEvidence::FinalPointOnly,
        },
        accuracy: match config.accuracy {
            OpAccuracy::Fast => OperatingPointAccuracyEvidence::Fast,
            OpAccuracy::Balanced => OperatingPointAccuracyEvidence::Balanced,
            OpAccuracy::Accurate => OperatingPointAccuracyEvidence::Accurate,
            OpAccuracy::Robust => OperatingPointAccuracyEvidence::Robust,
        },
        selected_devices: config.selected_devices.clone(),
        violation_devices: config.violation_devices.clone(),
        violation_source_content_digest: config.violation_source_content_digest,
        validated_startup_directives: u64::try_from(validated_startup_directives)
            .unwrap_or(u64::MAX),
        mna_node_names,
        mna_branch_names,
        mna_solution,
        effective_source_content_digest: None,
        previous_state: config
            .previous_state
            .as_ref()
            .filter(|_| config.initial_guess.uses_previous_state())
            .map(|previous| OperatingPointPreviousStateEvidence {
                source_content_digest: previous.source_content_digest,
                producer_snapshot_digest: previous.producer_snapshot_digest,
                producer_result_digest: previous.producer_result_digest,
            }),
        run_point_index: u64::try_from(config.run_point.index).unwrap_or(u64::MAX),
        run_point_count: u64::try_from(config.run_point.count).unwrap_or(u64::MAX),
        run_point_process: match config.run_point.process {
            rspice_app_types::product::ProcessCorner::TT => OperatingPointProcessEvidence::TT,
            rspice_app_types::product::ProcessCorner::SS => OperatingPointProcessEvidence::SS,
            rspice_app_types::product::ProcessCorner::FF => OperatingPointProcessEvidence::FF,
            rspice_app_types::product::ProcessCorner::SF => OperatingPointProcessEvidence::SF,
            rspice_app_types::product::ProcessCorner::FS => OperatingPointProcessEvidence::FS,
        },
        run_point_supply_voltage: config.run_point.supply_voltage,
        run_point_nominal_supply_voltage: config.run_point.nominal_supply_voltage,
    }
}
