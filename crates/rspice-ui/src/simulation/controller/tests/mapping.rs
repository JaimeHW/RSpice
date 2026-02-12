use super::*;

// -------------------------------------------------------------------------
// Config to AnalysisType Mapping Tests
// -------------------------------------------------------------------------

#[test]
fn test_config_to_analysis_type_dc_op() {
    let controller = SimulationController::new();
    assert_eq!(
        controller.config_to_analysis_type(&AnalysisConfig::DcOp),
        crate::state::AnalysisType::DcOp
    );
}

#[test]
fn test_config_to_analysis_type_dc_sweep() {
    let controller = SimulationController::new();
    let config = AnalysisConfig::DcSweep(DcSweepConfig {
        source: "V1".to_string(),
        start: 0.0,
        stop: 5.0,
        step: 0.1,
        source2: None,
        start2: None,
        stop2: None,
        step2: None,
    });
    assert_eq!(
        controller.config_to_analysis_type(&config),
        crate::state::AnalysisType::DcSweep
    );
}

#[test]
fn test_config_to_analysis_type_transient() {
    let controller = SimulationController::new();
    let config = AnalysisConfig::Transient(TransientAnalysisConfig {
        stop_time: 1e-6,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    });
    assert_eq!(
        controller.config_to_analysis_type(&config),
        crate::state::AnalysisType::Transient
    );
}

#[test]
fn test_config_to_analysis_type_ac() {
    let controller = SimulationController::new();
    let config = AnalysisConfig::Ac(AcAnalysisConfig {
        start_freq: 1.0,
        stop_freq: 1e9,
        num_points: 101,
        sweep_type: AcSweepType::Decade,
    });
    assert_eq!(
        controller.config_to_analysis_type(&config),
        crate::state::AnalysisType::Ac
    );
}

#[test]
fn test_config_to_analysis_type_all_variants() {
    use crate::simulation::config::{
        AcSweepType, NoiseAnalysisConfig, PoleZeroConfig, PzAnalysisType, SensitivityConfig,
    };
    let controller = SimulationController::new();

    // Noise - uses reference_node: String (not Option), sweep_type, num_points
    let noise_config = AnalysisConfig::Noise(NoiseAnalysisConfig {
        output_node: "out".to_string(),
        reference_node: "0".to_string(),
        input_source: "V1".to_string(),
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1e6,
    });
    assert_eq!(
        controller.config_to_analysis_type(&noise_config),
        crate::state::AnalysisType::Noise
    );

    // PoleZero - uses input_node, input_ref, output_node, output_ref, transfer_type, analysis_type
    let pz_config = AnalysisConfig::PoleZero(PoleZeroConfig {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "VOL".to_string(),
        analysis_type: PzAnalysisType::PoleZero,
    });
    assert_eq!(
        controller.config_to_analysis_type(&pz_config),
        crate::state::AnalysisType::PoleZero
    );

    // Sensitivity - uses output_var, ac_mode, frequency
    let sens_config = AnalysisConfig::Sensitivity(SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: None,
    });
    assert_eq!(
        controller.config_to_analysis_type(&sens_config),
        crate::state::AnalysisType::Sensitivity
    );
}

#[test]
fn test_spec_to_analysis_type_preserves_advanced_categories() {
    let controller = SimulationController::new();
    let cases = [
        (AnalysisSpec::Tf, crate::state::AnalysisType::Tf),
        (
            AnalysisSpec::Disto {
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 10,
                sweep: FrequencySweep::Decade,
                f2_over_f1: Some(1.5),
            },
            crate::state::AnalysisType::Disto,
        ),
        (AnalysisSpec::Pac, crate::state::AnalysisType::Pac),
        (AnalysisSpec::Pnoise, crate::state::AnalysisType::Pnoise),
        (AnalysisSpec::Pxf, crate::state::AnalysisType::Pxf),
        (AnalysisSpec::Pstb, crate::state::AnalysisType::Pstb),
        (
            AnalysisSpec::Stb {
                probe_node: "L1".to_string(),
                start_freq: 1.0,
                stop_freq: 1e6,
                points_per_decade: 10,
            },
            crate::state::AnalysisType::Stb,
        ),
        (
            AnalysisSpec::Reliability {
                target_years: vec![1.0, 5.0],
                enable_hci: true,
                enable_nbti: false,
                enable_em: false,
                min_stress_voltage: 0.05,
            },
            crate::state::AnalysisType::Reliability,
        ),
        (
            AnalysisSpec::Optimization {
                variables: vec![OptimizationVariable {
                    name: "X".to_string(),
                    min: 0.0,
                    max: 1.0,
                    initial: 0.5,
                }],
                objective_node: "out".to_string(),
                objective_ref: "0".to_string(),
                goal: OptimizationGoal::Minimize,
                target: None,
                algorithm: OptimizationAlgorithm::PatternSearch,
                max_iterations: 10,
                cost_tolerance: 1e-6,
                fd_step: 1e-3,
                initial_step: 0.1,
                min_step: 1e-5,
            },
            crate::state::AnalysisType::Optimization,
        ),
        (
            AnalysisSpec::Soa {
                stop_time: 1e-6,
                step_time: 1e-9,
                check_vgs_max: true,
                max_vgs: 1.2,
                check_vds_max: true,
                max_vds: 3.3,
                check_vbe_max: false,
                max_vbe: 0.9,
                check_vce_max: false,
                max_vce: 5.0,
            },
            crate::state::AnalysisType::Soa,
        ),
        (
            AnalysisSpec::SParameter {
                start_freq: 1e6,
                stop_freq: 1e9,
                points_per_unit: 10,
                sweep: FrequencySweep::Decade,
                z0: 50.0,
                ports: vec![
                    SpPort {
                        node_pos: "in".to_string(),
                        node_neg: "0".to_string(),
                        z0: None,
                    },
                    SpPort {
                        node_pos: "out".to_string(),
                        node_neg: "0".to_string(),
                        z0: Some(60.0),
                    },
                ],
            },
            crate::state::AnalysisType::SParameter,
        ),
        (
            AnalysisSpec::Envelope {
                fundamental_freq: 1e9,
                stop_time: 1e-6,
                num_harmonics: 9,
                max_step: None,
            },
            crate::state::AnalysisType::Envelope,
        ),
        (
            AnalysisSpec::Fourier {
                fundamental_freq: 1e6,
                num_harmonics: 11,
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                start_time: 0.0,
                stop_time: 10e-6,
            },
            crate::state::AnalysisType::Fourier,
        ),
    ];

    for (spec, expected) in cases {
        assert_eq!(
            controller.spec_to_analysis_type(&spec),
            expected,
            "unexpected analysis type mapping for {:?}",
            spec.run_type()
        );
    }
}
