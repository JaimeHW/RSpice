//! Saved-plan preflight and frozen-task integration cases.

use super::*;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};

#[test]
fn study_options_and_commands_read_the_exact_authored_draft() {
    use crate::services::simulation_runner::CornerBaseMode;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .unwrap();

    let mut mc = state.sim_setup.mc.clone();
    mc.ensure_initialized();
    mc.histogram_bins = "31".into();
    mc.num_runs = "17".into();
    state.sim_setup.mc.histogram_bins = "99".into();
    state.sim_setup.mc.num_runs = "99".into();
    let draft = AnalysisDraft::MonteCarlo(mc);
    let spec = controller.analysis_draft_spec(&state, &draft).unwrap();
    assert!(
        controller
            .analysis_spec_to_spice_line(&state, &draft, &spec)
            .unwrap()
            .starts_with(".mc 17 ")
    );
    let options = controller
        .analysis_spec_execution_options(&state, &draft, None, &spec, &sealed)
        .unwrap();
    assert_eq!(options.mc_histogram_bins, Some(31));

    let mut temp = state.sim_setup.temp.clone();
    temp.ensure_initialized();
    temp.specific_temps = "11, 22".into();
    state.sim_setup.temp.specific_temps = "not a temperature".into();
    let draft = AnalysisDraft::Temperature(temp);
    let spec = controller.analysis_draft_spec(&state, &draft).unwrap();
    assert_eq!(
        controller
            .analysis_spec_to_spice_line(&state, &draft, &spec)
            .unwrap(),
        ".step temp list 11 22"
    );
    let options = controller
        .analysis_spec_execution_options(&state, &draft, None, &spec, &sealed)
        .unwrap();
    assert_eq!(options.temp.unwrap().temperatures_c, vec![11.0, 22.0]);

    let mut corner = state.sim_setup.corner.clone();
    corner.ensure_initialized();
    corner.base_analysis_idx = 3;
    state.sim_setup.corner.base_analysis_idx = 0;
    let draft = AnalysisDraft::Corner(corner);
    let spec = controller.analysis_draft_spec(&state, &draft).unwrap();
    let options = controller
        .analysis_spec_execution_options(&state, &draft, None, &spec, &sealed)
        .unwrap();
    assert!(matches!(
        options.corner.unwrap().base_mode,
        CornerBaseMode::Op
    ));

    state.sim_setup.pss.ensure_initialized();
    state.sim_setup.pss.tone_sources = "VIN".into();
    let mut pac = state.sim_setup.pac.clone();
    pac.ensure_initialized();
    pac.pac_magnitude = "2.5".into();
    state.sim_setup.pac.pac_magnitude = "9".into();
    let draft = AnalysisDraft::Pac(pac);
    let options = controller
        .analysis_spec_execution_options(&state, &draft, None, &AnalysisSpec::Pac, &sealed)
        .unwrap();
    assert_eq!(options.pac.unwrap().pac_magnitude, 2.5);

    let mut pss = state.sim_setup.pss.clone();
    pss.ensure_initialized();
    pss.tone_sources = "VIN".into();
    pss.fund_freq = "1k".into();
    state.sim_setup.pss = pss.clone();
    state.sim_setup.pss.fund_freq = "2k".into();
    let draft = AnalysisDraft::Pss(pss);
    let spec = controller.analysis_draft_spec(&state, &draft).unwrap();
    assert!(
        matches!(&spec, AnalysisSpec::Pss { fundamental_freq, .. } if *fundamental_freq == 1e3)
    );
    assert!(
        controller
            .analysis_spec_to_spice_line(&state, &draft, &spec)
            .unwrap()
            .starts_with(".pss fund=1k ")
    );

    let mut hb = state.sim_setup.hb.clone();
    hb.ensure_initialized();
    hb.fundamental = "1meg".into();
    state.sim_setup.hb = hb.clone();
    state.sim_setup.hb.fundamental = "2meg".into();
    let draft = AnalysisDraft::HarmonicBalance(hb);
    let spec = controller.analysis_draft_spec(&state, &draft).unwrap();
    assert!(
        matches!(&spec, AnalysisSpec::HarmonicBalance { tones, .. } if tones[0].frequency == 1e6)
    );
    assert!(
        controller
            .analysis_spec_to_spice_line(&state, &draft, &spec)
            .unwrap()
            .starts_with(".hb 1000000 ")
    );
}

#[test]
fn pvt_selected_bases_persist_clone_and_freeze_exact_settings() {
    use crate::services::simulation_runner::CornerBaseMode;
    use crate::simulation::plan::{
        AnalysisNumericOverride, NumericOverrideOption as O, SimulationPlan,
    };
    use crate::simulation::runner::worker_contract::WorkerCornerBaseMode;
    for wrapper in [AnalysisKind::Temperature, AnalysisKind::Corner] {
        for kind in [
            AnalysisKind::OperatingPoint,
            AnalysisKind::Transient,
            AnalysisKind::Ac,
            AnalysisKind::DcSweep,
        ] {
            let mut state = AppState::default();
            state.sim_setup.analysis_plan = Some(SimulationPlan::empty());
            let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
            let op = plan.insert(AnalysisKind::OperatingPoint).unwrap().0;
            let base = plan.insert(kind).unwrap().0;
            let other = plan.insert(kind).unwrap().0;
            for id in [base, other] {
                for prerequisite in kind.prerequisites() {
                    plan.bind_dependency(id, *prerequisite, op).unwrap();
                }
            }
            plan.edit(base, |draft| match draft {
                AnalysisDraft::OperatingPoint(draft) => {
                    draft.node_initialization_idx = 2;
                    draft.initial_guess_idx = 2;
                    draft.accuracy_idx = 2;
                    draft.homotopy_idx = 4;
                }
                AnalysisDraft::Transient(draft) => {
                    draft.stop = "0.003".into();
                    draft.step = "0.00001".into();
                    draft.start = "0.001".into();
                    draft.max_step = "0.000002".into();
                    draft.uic = true;
                }
                AnalysisDraft::Ac(draft) => {
                    draft.fstart = "100".into();
                    draft.fstop = "1000".into();
                    draft.points = "7".into();
                    draft.sweep = 1;
                }
                AnalysisDraft::DcSweep(draft) => {
                    draft.source = "Vselected".into();
                    draft.mode = 1;
                    draft.values = "0 0.5 1 0.5".into();
                    draft.nested = true;
                    draft.source2 = "Vouter".into();
                    draft.mode2 = 1;
                    draft.values2 = "1 2".into();
                }
                _ => unreachable!(),
            })
            .unwrap();
            let mut numeric = AnalysisNumericOverride::default();
            numeric
                .set_for_instance(kind, plan.solver_ownership(base), O::Gmin, "1e-9")
                .unwrap();
            plan.set_numeric_override(base, Some(numeric)).unwrap();
            let study = plan.insert(wrapper).unwrap().0;
            plan.edit(study, |draft| match draft {
                AnalysisDraft::Temperature(draft) => {
                    draft.base_analysis = Some(base);
                    draft.base_idx = 3;
                }
                AnalysisDraft::Corner(draft) => {
                    draft.base_analysis = Some(base);
                    draft.base_analysis_idx = 2;
                }
                _ => unreachable!(),
            })
            .unwrap();
            let mut numeric = AnalysisNumericOverride::default();
            numeric
                .set_for_instance(wrapper, plan.solver_ownership(study), O::Pivrel, "1e-4")
                .unwrap();
            plan.set_numeric_override(study, Some(numeric)).unwrap();
            let mut restored: SimulationPlan =
                serde_json::from_value(serde_json::to_value(&*plan).unwrap()).unwrap();
            restored.prepare_after_restore();
            assert_eq!(
                restored
                    .instance(study)
                    .unwrap()
                    .draft()
                    .pvt_base_analysis(),
                Some(base)
            );
            let cloned = restored.clone_as_new().unwrap();
            assert_eq!(
                cloned.instances()[3].draft().pvt_base_analysis(),
                Some(cloned.instances()[1].id())
            );
            let frozen = restored.freeze().unwrap();
            // Shared setup and later live base edits must not replace frozen values.
            state.sim_setup.tran.stop = "invalid".into();
            state.sim_setup.ac.fstop = "invalid".into();
            state.sim_setup.dc.source = "Vwrong".into();
            state
                .sim_setup
                .analysis_plan
                .as_mut()
                .unwrap()
                .edit(base, |draft| {
                    *draft = AnalysisDraft::for_kind(kind);
                })
                .unwrap();
            let sealed = state
                .model_library_manager
                .seal_execution_sources()
                .unwrap();
            let controller = SimulationController::new();
            let tasks = controller
                .build_queue_from_plan(&state, &frozen, &sealed)
                .unwrap();
            let task = tasks
                .iter()
                .find(|task| task.instance_id() == study)
                .unwrap();
            let queued = task.queued_analysis();
            let mode = queued
                .spec_options
                .temp
                .as_ref()
                .map(|config| &config.base_mode)
                .or_else(|| {
                    queued
                        .spec_options
                        .corner
                        .as_ref()
                        .map(|config| &config.base_mode)
                })
                .unwrap();
            let wire = WorkerCornerBaseMode::from(mode);
            let restored_wire: WorkerCornerBaseMode =
                serde_json::from_value(serde_json::to_value(&wire).unwrap()).unwrap();
            assert_eq!(wire, restored_wire);
            match mode {
                CornerBaseMode::ConfiguredOp(config) => {
                    assert_eq!(
                        config.node_initialization,
                        crate::simulation::dialog::OpNodeInitialization::ForceIcValues
                    );
                    assert_eq!(
                        config.initial_guess,
                        crate::simulation::dialog::OpInitialGuess::UserNodeVoltages
                    );
                    assert_eq!(config.homotopy, crate::simulation::dialog::OpHomotopy::None);
                }
                CornerBaseMode::TransientWindow {
                    stop_time,
                    step_time,
                    start_time,
                    max_timestep,
                    uic,
                } => {
                    assert_eq!(
                        (*stop_time, *step_time, *start_time, *max_timestep, *uic),
                        (0.003, 0.00001, 0.001, Some(0.000002), true)
                    );
                }
                CornerBaseMode::Ac {
                    start_freq,
                    stop_freq,
                    points_per_unit,
                    sweep,
                } => {
                    assert_eq!(
                        (*start_freq, *stop_freq, *points_per_unit),
                        (100.0, 1000.0, 7)
                    );
                    assert_eq!(
                        *sweep,
                        crate::services::simulation_runner::CornerFrequencySweep::Octave
                    );
                }
                CornerBaseMode::DcSweepNested {
                    source_name,
                    source2,
                    modes,
                    ..
                } => {
                    assert_eq!(
                        (source_name.as_str(), source2.as_str()),
                        ("Vselected", "Vouter")
                    );
                    assert_eq!(
                        modes.primary.spec(0.0, 0.0, 1.0).mode,
                        rspice_core::netlist::DcSweepMode::List(vec![0.0, 0.5, 1.0, 0.5])
                    );
                }
                other => panic!("wrong bound base: {other:?}"),
            }
            let numeric = queued.numeric_override.as_ref().unwrap();
            assert!(numeric.stated(O::Gmin).is_some());
            assert!(numeric.stated(O::Pivrel).is_some());
            restored.set_enabled(base, false).unwrap();
            let errors = controller
                .build_queue_from_plan(&state, &restored.freeze().unwrap(), &sealed)
                .unwrap_err();
            assert!(
                errors
                    .iter()
                    .any(|error| error.contains("missing or disabled")),
                "{errors:?}"
            );
        }
    }
}

#[test]
fn qp_study_freezes_the_exact_producer_and_complete_consumer_controls() {
    use crate::simulation::dialog::{McDialogState, mc::McConfig};
    use crate::simulation::plan::{QpnoiseOutputDraft, QpssDraft};
    use crate::simulation::runner::study::StudyAnalysis;
    for kind in [
        AnalysisKind::Qpss,
        AnalysisKind::Qpac,
        AnalysisKind::Qpxf,
        AnalysisKind::Qpnoise,
    ] {
        let mut state = AppState::default();
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
        let (producer, _) = plan.insert(AnalysisKind::Qpss).unwrap();
        plan.bind_dependency(producer, AnalysisKind::OperatingPoint, op)
            .unwrap();
        plan.edit(producer, |draft| {
            *draft = AnalysisDraft::Qpss(QpssDraft {
                tones: "1k,1414.2135623730951".into(),
                harmonics: "1,1".into(),
                relative_tolerance: "1e-8".into(),
                max_backtracks: "7".into(),
                collocation_points: "8,8".into(),
                source_tones: "V1=1;I1=2".into(),
                dc_initialization: true,
                ..Default::default()
            })
        })
        .unwrap();
        let consumer = if kind == AnalysisKind::Qpss {
            producer
        } else {
            let (id, _) = plan.insert(kind).unwrap();
            plan.bind_dependency(id, AnalysisKind::Qpss, producer)
                .unwrap();
            plan.edit(id, |draft| match draft {
                AnalysisDraft::Qpac(d) => {
                    d.explicit_offsets = "100,300,700".into();
                    d.magnitude = ".002".into();
                    d.phase_degrees = "73".into();
                    d.input_lattice = "1,-1".into();
                    d.output_lattice = "1,-1".into();
                }
                AnalysisDraft::Qpxf(d) => {
                    d.explicit_frequencies = "-100,0,117".into();
                    d.group_delay = true;
                    d.input_lattice = "1,-1".into();
                    d.output_lattice = "1,-1".into();
                }
                AnalysisDraft::Qpnoise(d) => {
                    d.explicit_frequencies = "100,300,700".into();
                    d.band_start = "150".into();
                    d.band_stop = "600".into();
                    d.additional_outputs = vec![QpnoiseOutputDraft {
                        current: true,
                        branch: "V1".into(),
                        ..Default::default()
                    }];
                }
                _ => unreachable!(),
            })
            .unwrap();
            id
        };
        let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
        plan.edit(mc, |draft| {
            *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                base_analysis: Some(consumer),
                measurements: vec![
                    if kind == AnalysisKind::Qpss {
                        "tuple:1,0:magnitude:V(out)"
                    } else {
                        "bin:0:real:result"
                    }
                    .into(),
                ],
                ..Default::default()
            }))
        })
        .unwrap();
        let mut producer_numerics = crate::simulation::plan::AnalysisNumericOverride::default();
        producer_numerics
            .set_for_instance(
                AnalysisKind::Qpss,
                Default::default(),
                crate::simulation::plan::NumericOverrideOption::Gmin,
                "1e-9",
            )
            .unwrap();
        plan.set_numeric_override(producer, Some(producer_numerics))
            .unwrap();
        let frozen = plan.freeze().unwrap();
        plan.edit(producer, |draft| {
            let AnalysisDraft::Qpss(d) = draft else {
                unreachable!()
            };
            d.relative_tolerance = ".001".into();
        })
        .unwrap();
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap();
        let queue = SimulationController::new()
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap();
        let inherited = &queue
            .iter()
            .find(|task| task.instance_id() == consumer)
            .unwrap()
            .queued_analysis()
            .numeric_override;
        let carrier = &queue
            .iter()
            .find(|task| task.instance_id() == producer)
            .unwrap()
            .queued_analysis()
            .numeric_override;
        assert!(carrier.is_some());
        assert_eq!(
            inherited, carrier,
            "consumer must authenticate the carrier's numerical circuit"
        );
        let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
        let base = task
            .queued_analysis()
            .spec_options
            .study_base
            .as_ref()
            .unwrap();
        let StudyAnalysis::Qpss(config) = &base.analysis else {
            panic!("configured QPSS")
        };
        let spec = &config.request;
        assert_eq!(config.operating_point.instance_id, op);
        assert_eq!(config.operating_point.source_revision, frozen.revision());
        assert_eq!(
            *spec,
            queue
                .iter()
                .find(|task| task.instance_id() == producer)
                .unwrap()
                .queued_analysis()
                .spec
        );
        let AnalysisSpec::Qpss {
            relative_tolerance,
            controls,
            ..
        } = spec
        else {
            panic!("QPSS")
        };
        assert_eq!(*relative_tolerance, 1e-8);
        assert_eq!(controls.max_backtracks, 7);
        if kind == AnalysisKind::Qpss {
            assert!(base.postprocess.is_none());
        } else {
            let post = base.postprocess.as_ref().unwrap();
            assert_eq!(post.producer_instance_id, producer);
            assert_eq!(post.producer_source_revision, frozen.revision());
            assert_eq!(
                post.request,
                queue
                    .iter()
                    .find(|task| task.instance_id() == consumer)
                    .unwrap()
                    .queued_analysis()
                    .spec
            );
        }
        for change in 0..6 {
            let mut changed = task.queued_analysis().clone();
            let base = changed.spec_options.study_base.as_mut().unwrap();
            if change >= 2 {
                let StudyAnalysis::Qpss(config) = &mut base.analysis else {
                    unreachable!()
                };
                match change {
                    2 => {
                        config.operating_point.instance_id =
                            crate::product::AnalysisInstanceId::new()
                    }
                    3 => config.operating_point.numeric_options = ".options GMIN=1e-5".into(),
                    4 => config.operating_point.config.temperature_celsius += 10.0,
                    _ => {
                        config.operating_point.config.node_initialization =
                            crate::simulation::dialog::OpNodeInitialization::IgnoreIcAndNodeset
                    }
                }
            } else if change == 0 || base.postprocess.is_none() {
                let StudyAnalysis::Qpss(config) = &mut base.analysis else {
                    unreachable!()
                };
                let AnalysisSpec::Qpss { controls, .. } = &mut config.request else {
                    unreachable!()
                };
                controls.max_backtracks += 1;
            } else {
                match &mut base.postprocess.as_mut().unwrap().request {
                    AnalysisSpec::Qpac { controls, .. } => controls.phase_degrees += 1.0,
                    AnalysisSpec::Qpxf { group_delay, .. } => *group_delay = false,
                    AnalysisSpec::Qpnoise { controls, .. } => {
                        controls.integration_band = Some([200.0, 500.0])
                    }
                    _ => unreachable!(),
                }
            }
            assert_ne!(
                task.config_digest(),
                PreparedTask::new(mc, task.source_revision(), vec![], "MC", changed)
                    .config_digest()
            );
        }
    }
}

#[test]
fn periodic_rf_study_freezes_all_consumer_options_and_exact_pss_op_chain() {
    use crate::simulation::dialog::{McDialogState, mc::McConfig};
    use crate::simulation::runner::study::{StudyAnalysis, StudyPeriodicOptions};
    for kind in [
        AnalysisKind::Pac,
        AnalysisKind::Pxf,
        AnalysisKind::Pnoise,
        AnalysisKind::Pstb,
        AnalysisKind::Psp,
    ] {
        let mut state = AppState::default();
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
        let (pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
        plan.bind_dependency(pss, AnalysisKind::OperatingPoint, op)
            .unwrap();
        plan.edit(pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                unreachable!()
            };
            draft.tone_sources = "VIN".into();
        })
        .unwrap();
        let (consumer, _) = plan.insert(kind).unwrap();
        plan.bind_dependency(consumer, AnalysisKind::Pss, pss)
            .unwrap();
        plan.edit(consumer, |draft| match draft {
            AnalysisDraft::Pac(d) => {
                d.pac_magnitude = "2.5".into();
                d.sideband_min = "-1".into();
                d.sideband_max = "0".into();
                d.reltol = "2e-7".into();
            }
            AnalysisDraft::Pxf(d) => {
                d.input_sideband = "-1".into();
                d.output_sideband = "1".into();
                d.max_sideband = "2".into();
            }
            AnalysisDraft::Pnoise(d) => {
                d.input_sideband = "-1".into();
                d.output_sideband = "1".into();
                d.max_sideband = "2".into();
                d.noise_summary = true;
            }
            AnalysisDraft::Pstb(d) => {
                d.stability_threshold = "1.01".into();
                d.detect_subharmonics = false;
            }
            AnalysisDraft::Psp(d) => {
                d.max_sideband = "2".into();
                d.mixed_mode = true;
            }
            _ => unreachable!(),
        })
        .unwrap();
        let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
        plan.edit(mc, |draft| {
            *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                base_analysis: Some(consumer),
                measurements: vec!["bin:0:real:result".into()],
                ..Default::default()
            }))
        })
        .unwrap();
        let mut producer_numerics = crate::simulation::plan::AnalysisNumericOverride::default();
        producer_numerics
            .set_for_instance(
                AnalysisKind::Pss,
                Default::default(),
                crate::simulation::plan::NumericOverrideOption::Gmin,
                "1e-9",
            )
            .unwrap();
        plan.set_numeric_override(pss, Some(producer_numerics))
            .unwrap();
        let frozen = plan.freeze().unwrap();
        plan.edit(consumer, |draft| *draft = AnalysisDraft::for_kind(kind))
            .unwrap();
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap();
        let queue = SimulationController::new()
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap();
        let inherited = &queue
            .iter()
            .find(|task| task.instance_id() == consumer)
            .unwrap()
            .queued_analysis()
            .numeric_override;
        let carrier = &queue
            .iter()
            .find(|task| task.instance_id() == pss)
            .unwrap()
            .queued_analysis()
            .numeric_override;
        assert!(carrier.is_some());
        assert_eq!(
            inherited, carrier,
            "consumer must authenticate the carrier's numerical circuit"
        );
        let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
        let consumer_task = queue
            .iter()
            .find(|task| task.instance_id() == consumer)
            .unwrap();
        let base = task
            .queued_analysis()
            .spec_options
            .study_base
            .as_ref()
            .unwrap();
        let StudyAnalysis::Pss(pss_config) = &base.analysis else {
            panic!("PSS")
        };
        assert_eq!(pss_config.operating_point.instance_id, op);
        let post = base.postprocess.as_ref().unwrap();
        assert_eq!(post.producer_instance_id, pss);
        assert_eq!(post.request, consumer_task.queued_analysis().spec);
        match &post.periodic_options {
            Some(StudyPeriodicOptions::Pac(c)) => {
                assert_eq!(
                    Some(c),
                    consumer_task.queued_analysis().spec_options.pac.as_ref()
                );
                assert_eq!(c.pac_magnitude, 2.5);
            }
            Some(StudyPeriodicOptions::Pxf(c)) => {
                assert_eq!(
                    Some(c),
                    consumer_task.queued_analysis().spec_options.pxf.as_ref()
                );
                assert_eq!(c.input_sideband, -1);
            }
            Some(StudyPeriodicOptions::Pnoise(c)) => {
                assert_eq!(
                    Some(c),
                    consumer_task.queued_analysis().spec_options.pnoise.as_ref()
                );
                assert_eq!(c.output_sideband, 1);
            }
            Some(StudyPeriodicOptions::Pstb(c)) => {
                assert_eq!(
                    Some(c),
                    consumer_task.queued_analysis().spec_options.pstb.as_ref()
                );
                assert_eq!(c.stability_threshold, 1.01);
            }
            None => assert!(matches!(
                post.request,
                AnalysisSpec::Psp {
                    mixed_mode: true,
                    ..
                }
            )),
        }
        let mut changed = task.queued_analysis().clone();
        let post = changed
            .spec_options
            .study_base
            .as_mut()
            .unwrap()
            .postprocess
            .as_mut()
            .unwrap();
        match &mut post.periodic_options {
            Some(StudyPeriodicOptions::Pac(c)) => c.pac_magnitude = 3.0,
            Some(StudyPeriodicOptions::Pxf(c)) => c.input_sideband = 0,
            Some(StudyPeriodicOptions::Pnoise(c)) => c.output_sideband = 0,
            Some(StudyPeriodicOptions::Pstb(c)) => c.detect_subharmonics = true,
            None => {
                let AnalysisSpec::Psp { mixed_mode, .. } = &mut post.request else {
                    unreachable!()
                };
                *mixed_mode = false;
            }
        }
        assert_ne!(
            task.config_digest(),
            PreparedTask::new(mc, task.source_revision(), vec![], "MC", changed).config_digest()
        );
    }
}

#[test]
fn preceding_pac_basis_uses_its_exact_frozen_hb_producer() {
    use crate::simulation::dialog::{McDialogState, mc::McConfig};
    use crate::simulation::runner::study::StudyPeriodicOptions;

    let mut state = AppState::default();
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let (first, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
    let (second, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
    for (id, frequency) in [(first, "1meg"), (second, "2meg")] {
        plan.bind_dependency(id, AnalysisKind::OperatingPoint, op)
            .unwrap();
        plan.edit(id, |draft| {
            let AnalysisDraft::HarmonicBalance(draft) = draft else {
                unreachable!()
            };
            draft.fundamental = frequency.into();
        })
        .unwrap();
    }
    let (pac, _) = plan.insert(AnalysisKind::Pac).unwrap();
    plan.bind_dependency(pac, AnalysisKind::HarmonicBalance, first)
        .unwrap();
    let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
    plan.edit(mc, |draft| {
        *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
            base_analysis: Some(pac),
            measurements: vec!["bin:0:real:result".into()],
            ..Default::default()
        }));
    })
    .unwrap();
    let frozen = plan.freeze().unwrap();
    plan.edit(first, |draft| {
        let AnalysisDraft::HarmonicBalance(draft) = draft else {
            unreachable!()
        };
        draft.fundamental = "3meg".into();
    })
    .unwrap();

    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .unwrap();
    let queue = SimulationController::new()
        .build_queue_from_plan(&state, &frozen, &sealed)
        .unwrap();
    let pac_task = queue.iter().find(|task| task.instance_id() == pac).unwrap();
    let pac_options = pac_task
        .queued_analysis()
        .spec_options
        .pac
        .as_ref()
        .unwrap();
    assert_eq!(pac_options.pss_fundamental_freq, 1e6);
    let study = queue
        .iter()
        .find(|task| task.instance_id() == mc)
        .unwrap()
        .queued_analysis()
        .spec_options
        .study_base
        .as_ref()
        .unwrap();
    let post = study.postprocess.as_ref().unwrap();
    assert_eq!(post.producer_instance_id, first);
    let Some(StudyPeriodicOptions::Pac(study_options)) = &post.periodic_options else {
        panic!("PAC study options")
    };
    assert_eq!(study_options, pac_options);
}

#[test]
fn pss_study_freezes_its_exact_op_producer_and_complete_shooting_configuration() {
    use crate::simulation::dialog::{McDialogState, mc::McConfig};
    use crate::simulation::runner::study::StudyAnalysis;
    let mut state = AppState::default();
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let (first, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let (second, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let (pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
    plan.bind_dependency(pss, AnalysisKind::OperatingPoint, first)
        .unwrap();
    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(draft) = draft else {
            unreachable!()
        };
        draft.fund_freq = "1k".into();
        draft.tone_sources = "V1".into();
    })
    .unwrap();
    let mut numerics = crate::simulation::plan::AnalysisNumericOverride::default();
    numerics
        .set_for_instance(
            AnalysisKind::OperatingPoint,
            Default::default(),
            crate::simulation::plan::NumericOverrideOption::Reltol,
            "1e-7",
        )
        .unwrap();
    plan.set_numeric_override(first, Some(numerics)).unwrap();
    let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
    plan.edit(mc, |draft| {
        *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
            base_analysis: Some(pss),
            measurements: vec!["bin:1:magnitude:V(out)".into()],
            ..Default::default()
        }))
    })
    .unwrap();
    let frozen = plan.freeze().unwrap();
    plan.bind_dependency(pss, AnalysisKind::OperatingPoint, second)
        .unwrap();
    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(draft) = draft else {
            unreachable!()
        };
        draft.fund_freq = "2k".into();
    })
    .unwrap();
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .unwrap();
    let queue = SimulationController::new()
        .build_queue_from_plan(&state, &frozen, &sealed)
        .unwrap();
    let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
    let base = task
        .queued_analysis()
        .spec_options
        .study_base
        .as_ref()
        .unwrap();
    let StudyAnalysis::Pss(config) = &base.analysis else {
        panic!("PSS")
    };
    assert_eq!(config.operating_point.instance_id, first);
    assert_eq!(config.operating_point.source_revision, frozen.revision());
    assert!(config.operating_point.numeric_options.contains("RELTOL"));
    assert_eq!(
        config.request,
        queue
            .iter()
            .find(|task| task.instance_id() == pss)
            .unwrap()
            .queued_analysis()
            .spec
    );
    for change in 0..3 {
        let mut queued = task.queued_analysis().clone();
        let StudyAnalysis::Pss(config) =
            &mut queued.spec_options.study_base.as_mut().unwrap().analysis
        else {
            unreachable!()
        };
        match change {
            0 => config.operating_point.instance_id = second,
            1 => config.operating_point.config.temperature_celsius = 85.0,
            _ => config.operating_point.numeric_options = ".options RELTOL=.01".into(),
        }
        assert_ne!(
            task.config_digest(),
            PreparedTask::new(mc, task.source_revision(), vec![], "MC", queued).config_digest()
        );
    }
}

#[test]
fn hb_study_freezes_the_selected_instance_and_authenticates_its_native_settings() {
    use crate::simulation::dialog::{
        HbDialogState, McDialogState,
        hb::{HbConfig, HbSolverType, HbToneConfig},
        mc::McConfig,
    };
    use crate::simulation::runner::study::StudyAnalysis;
    let mut state = AppState::default();
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let (hb, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
    plan.bind_dependency(hb, AnalysisKind::OperatingPoint, op)
        .unwrap();
    plan.edit(hb, |draft| {
        *draft = AnalysisDraft::HarmonicBalance(HbDialogState::from_config(&HbConfig {
            fundamental_freq: 1000.0,
            fundamental_source: Some("V1".into()),
            num_harmonics: 2,
            additional_tones: vec![
                HbToneConfig::new(2000.0, 2)
                    .with_source("V2")
                    .with_name("second"),
            ],
            oversample: 3,
            max_mixing_order: 2,
            reltol: 2e-7,
            abstol: 3e-12,
            maxiter: 73,
            damping: 0.8,
            min_damping: 0.02,
            collocation_points: Some(31),
            solver: HbSolverType::Krylov,
            gmres_restart: 17,
            source_stepping: true,
            use_exact_jacobian: false,
            verbose: true,
        }));
    })
    .unwrap();
    let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
    plan.edit(mc, |draft| {
        *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
            base_analysis: Some(hb),
            measurements: vec!["bin:1:magnitude:V(out)".into()],
            ..Default::default()
        }));
    })
    .unwrap();
    let frozen = plan.freeze().unwrap();
    plan.edit(hb, |draft| {
        let AnalysisDraft::HarmonicBalance(draft) = draft else {
            unreachable!()
        };
        draft.fundamental = "3k".into();
        draft.verbose = false;
    })
    .unwrap();
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .unwrap();
    let queue = SimulationController::new()
        .build_queue_from_plan(&state, &frozen, &sealed)
        .unwrap();
    let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
    let base = task
        .queued_analysis()
        .spec_options
        .study_base
        .as_ref()
        .unwrap();
    assert_eq!(base.instance_id, hb);
    let StudyAnalysis::Hb(config) = &base.analysis else {
        panic!("configured HB")
    };
    assert_eq!(config.operating_point.instance_id, op);
    assert_eq!(config.operating_point.source_revision, frozen.revision());
    let spec = &config.request;
    let selected = queue.iter().find(|task| task.instance_id() == hb).unwrap();
    assert_eq!(spec, &selected.queued_analysis().spec);
    assert!(
        matches!(spec, AnalysisSpec::HarmonicBalance { tones, verbose: true, .. } if tones[0].frequency == 1000.0)
    );
    let mut changed = task.queued_analysis().clone();
    let StudyAnalysis::Hb(config) = &mut changed.spec_options.study_base.as_mut().unwrap().analysis
    else {
        unreachable!()
    };
    let AnalysisSpec::HarmonicBalance {
        collocation_points, ..
    } = &mut config.request
    else {
        unreachable!()
    };
    *collocation_points = Some(33);
    assert_ne!(
        task.config_digest(),
        PreparedTask::new(mc, task.source_revision(), vec![], "MC", changed).config_digest()
    );
}

#[test]
fn hb_rf_study_freezes_exact_producer_consumer_and_noise_references() {
    use crate::simulation::dialog::{McDialogState, mc::McConfig};
    use crate::simulation::runner::study::StudyAnalysis;
    for kind in [AnalysisKind::Hbsp, AnalysisKind::Hbnoise] {
        let mut state = AppState::default();
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
        let (first, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
        let (second, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
        for (id, frequency) in [(first, "1meg"), (second, "2meg")] {
            plan.bind_dependency(id, AnalysisKind::OperatingPoint, op)
                .unwrap();
            plan.edit(id, |draft| {
                let AnalysisDraft::HarmonicBalance(draft) = draft else {
                    unreachable!()
                };
                draft.fundamental = frequency.into();
            })
            .unwrap();
        }
        let mut numerics = crate::simulation::plan::AnalysisNumericOverride::default();
        numerics
            .set_for_instance(
                AnalysisKind::HarmonicBalance,
                Default::default(),
                crate::simulation::plan::NumericOverrideOption::Reltol,
                "1e-6",
            )
            .unwrap();
        plan.set_numeric_override(first, Some(numerics)).unwrap();
        let (consumer, _) = plan.insert(kind).unwrap();
        plan.bind_dependency(consumer, AnalysisKind::HarmonicBalance, first)
            .unwrap();
        plan.edit(consumer, |draft| match draft {
            AnalysisDraft::Hbsp(draft) => {
                draft.max_sideband = "1".into();
                draft.noise_parameters = true;
                draft.noise.report_parameters = true;
                draft.noise.input_sideband = "-1".into();
                draft.noise.output_sideband = "1".into();
                draft.noise.reference_temperature = "310".into();
                draft.noise.termination_temperature = "295".into();
            }
            AnalysisDraft::Hbnoise(draft) => {
                draft.max_sideband = "1".into();
                draft.noise_figure = true;
                draft.source_resistor = "RSRC".into();
                draft.reference_temperature = "310".into();
                draft.input_sideband = "-1".into();
                draft.output_sideband = "1".into();
                draft.integrated_noise = false;
                draft.contributor_ranking = false;
            }
            _ => unreachable!(),
        })
        .unwrap();
        let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
        plan.edit(mc, |draft| {
            *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                base_analysis: Some(consumer),
                measurements: vec![
                    if kind == AnalysisKind::Hbsp {
                        "bin:0:real:PN_NF"
                    } else {
                        "bin:0:real:noise_figure_db"
                    }
                    .into(),
                ],
                ..Default::default()
            }));
        })
        .unwrap();
        let frozen = plan.freeze().unwrap();
        plan.edit(first, |draft| {
            let AnalysisDraft::HarmonicBalance(draft) = draft else {
                unreachable!()
            };
            draft.fundamental = "3meg".into();
        })
        .unwrap();
        plan.edit(consumer, |draft| match draft {
            AnalysisDraft::Hbsp(draft) => draft.noise.reference_temperature = "350".into(),
            AnalysisDraft::Hbnoise(draft) => draft.reference_temperature = "350".into(),
            _ => unreachable!(),
        })
        .unwrap();
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap();
        let queue = SimulationController::new()
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap();
        let inherited = &queue
            .iter()
            .find(|task| task.instance_id() == consumer)
            .unwrap()
            .queued_analysis()
            .numeric_override;
        let carrier = &queue
            .iter()
            .find(|task| task.instance_id() == first)
            .unwrap()
            .queued_analysis()
            .numeric_override;
        assert!(carrier.is_some());
        assert_eq!(
            inherited, carrier,
            "consumer must authenticate the carrier's numerical circuit"
        );
        let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
        let base = task
            .queued_analysis()
            .spec_options
            .study_base
            .as_ref()
            .unwrap();
        let StudyAnalysis::Hb(config) = &base.analysis else {
            panic!("configured HB producer")
        };
        assert_eq!(config.operating_point.instance_id, op);
        assert_eq!(config.operating_point.source_revision, frozen.revision());
        let producer = &config.request;
        let selected = queue
            .iter()
            .find(|task| task.instance_id() == first)
            .unwrap();
        assert_eq!(producer, &selected.queued_analysis().spec);
        assert!(
            matches!(producer, AnalysisSpec::HarmonicBalance { tones, .. } if tones[0].frequency == 1e6)
        );
        let post = base.postprocess.as_ref().unwrap();
        assert_eq!(base.instance_id, consumer);
        assert_eq!(post.producer_instance_id, first);
        assert_ne!(post.producer_instance_id, second);
        assert_eq!(post.producer_source_revision, frozen.revision());
        assert!(post.producer_numeric_options.contains("RELTOL"));
        assert_eq!(
            &post.request,
            &queue
                .iter()
                .find(|task| task.instance_id() == consumer)
                .unwrap()
                .queued_analysis()
                .spec
        );
        for change in 0..8 {
            let mut queued = task.queued_analysis().clone();
            let base = queued.spec_options.study_base.as_mut().unwrap();
            let post = base.postprocess.as_mut().unwrap();
            match change {
                0 => post.producer_instance_id = second,
                1 => post.producer_numeric_options = ".OPTIONS RELTOL=0.01".into(),
                2 => match &mut post.request {
                    AnalysisSpec::Hbsp {
                        noise_reference: Some(reference),
                        ..
                    } => reference.reference_temperature_kelvin = 350.0,
                    AnalysisSpec::Hbnoise {
                        noise_reference: Some(reference),
                        ..
                    } => reference.temperature_kelvin = 350.0,
                    _ => unreachable!(),
                },
                3 => {
                    let StudyAnalysis::Hb(config) = &mut base.analysis else {
                        unreachable!()
                    };
                    let AnalysisSpec::HarmonicBalance { tones, .. } = &mut config.request else {
                        unreachable!()
                    };
                    tones[0].frequency = 4e6;
                }
                _ => {
                    let StudyAnalysis::Hb(config) = &mut base.analysis else {
                        unreachable!()
                    };
                    match change {
                        4 => {
                            config.operating_point.instance_id =
                                crate::product::AnalysisInstanceId::new()
                        }
                        5 => config.operating_point.numeric_options = ".options GMIN=1e-5".into(),
                        6 => config.operating_point.config.temperature_celsius += 10.0,
                        _ => {
                            config.operating_point.config.node_initialization =
                                crate::simulation::dialog::OpNodeInitialization::IgnoreIcAndNodeset
                        }
                    }
                }
            }
            assert_ne!(
                task.config_digest(),
                PreparedTask::new(mc, task.source_revision(), vec![], "MC", queued).config_digest()
            );
        }
    }
}

#[test]
fn spectral_study_freezes_the_bound_transient_and_all_postprocess_settings() {
    use crate::simulation::dialog::{McDialogState, mc::McConfig};
    for kind in [AnalysisKind::Fourier, AnalysisKind::Fft] {
        let mut state = AppState::default();
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        let (first, _) = plan.insert(AnalysisKind::Transient).unwrap();
        let (second, _) = plan.insert(AnalysisKind::Transient).unwrap();
        for (id, stop) in [(first, "1m"), (second, "2m")] {
            plan.edit(id, |draft| {
                let AnalysisDraft::Transient(draft) = draft else {
                    unreachable!()
                };
                draft.stop = stop.into();
                draft.step = "2u".into();
                draft.max_step = "2u".into();
            })
            .unwrap();
        }
        let mut numerics = crate::simulation::plan::AnalysisNumericOverride::default();
        numerics
            .set_for_instance(
                AnalysisKind::Transient,
                Default::default(),
                crate::simulation::plan::NumericOverrideOption::Reltol,
                "1e-6",
            )
            .unwrap();
        plan.set_numeric_override(first, Some(numerics)).unwrap();
        let (spectrum, _) = plan.insert(kind).unwrap();
        plan.edit(spectrum, |draft| match draft {
            AnalysisDraft::Fourier(draft) => {
                *draft = crate::simulation::dialog::FourierDialogState::from_config(
                    &crate::simulation::dialog::fourier::FourierConfig {
                        fundamental_freq: 1000.0,
                        num_harmonics: 5,
                        num_periods: 1,
                        output_node: "out".into(),
                        output_ref: "0".into(),
                        additional_outputs: vec![],
                        start_time: 0.0,
                        stop_time: 0.001,
                        compute_thd: true,
                        normalize: false,
                    },
                )
            }
            AnalysisDraft::Fft(draft) => {
                draft.stop = "1m".into();
                draft.points = 64;
                draft.format = "UNORM".into();
            }
            _ => unreachable!(),
        })
        .unwrap();
        plan.bind_dependency(spectrum, AnalysisKind::Transient, first)
            .unwrap();
        let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
        plan.edit(mc, |draft| {
            *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                base_analysis: Some(spectrum),
                measurements: vec!["bin:1:magnitude".into()],
                ..Default::default()
            }))
        })
        .unwrap();
        let frozen = plan.freeze().unwrap();
        plan.edit(first, |draft| {
            let AnalysisDraft::Transient(draft) = draft else {
                unreachable!()
            };
            draft.stop = "3m".into();
        })
        .unwrap();
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap();
        let queue = SimulationController::new()
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap();
        let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
        let base = task
            .queued_analysis()
            .spec_options
            .study_base
            .as_ref()
            .unwrap();
        let Some(AnalysisConfig::Transient(config)) = base.analysis.as_basic() else {
            panic!("transient producer")
        };
        assert_eq!(config.stop_time, 0.001);
        let post = base.postprocess.as_ref().unwrap();
        assert_eq!(post.producer_instance_id, first);
        assert_ne!(post.producer_instance_id, second);
        assert_eq!(post.producer_source_revision, frozen.revision());
        assert!(post.producer_numeric_options.contains("RELTOL"));
        for change in 0..5 {
            let mut queued = task.queued_analysis().clone();
            let base = queued.spec_options.study_base.as_mut().unwrap();
            let post = base.postprocess.as_mut().unwrap();
            match change {
                0 => post.producer_instance_id = second,
                1 => post.producer_analysis_line.push_str(" UIC"),
                2 => post.producer_numeric_options = ".OPTIONS RELTOL=0.01".into(),
                3 => match &mut post.request {
                    AnalysisSpec::Fft { request } => request.points = 128,
                    AnalysisSpec::Fourier { normalize, .. } => *normalize = true,
                    _ => unreachable!(),
                },
                _ => {
                    let Some(AnalysisConfig::Transient(config)) = base.analysis.as_basic_mut()
                    else {
                        unreachable!()
                    };
                    config.max_timestep = Some(1e-6);
                }
            }
            let changed = PreparedTask::new(mc, task.source_revision(), vec![], "MC", queued);
            assert_ne!(task.config_digest(), changed.config_digest());
        }
    }
}

#[test]
fn configured_study_freezes_exact_base_and_survives_persistence_and_identity() {
    use crate::simulation::dialog::{McDialogState, mc::McConfig};
    let mut state = AppState::default();
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let (ac, _) = plan.insert(AnalysisKind::Ac).unwrap();
    let (other, _) = plan.insert(AnalysisKind::Ac).unwrap();
    let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
    for (id, stop) in [(ac, "1000"), (other, "9000")] {
        plan.bind_dependency(id, AnalysisKind::OperatingPoint, op)
            .unwrap();
        plan.edit(id, |draft| {
            let AnalysisDraft::Ac(draft) = draft else {
                unreachable!()
            };
            draft.fstart = "1000".into();
            draft.fstop = stop.into();
            draft.sweep = 2;
            draft.points = "1".into();
        })
        .unwrap();
    }
    let mut numerics = crate::simulation::plan::AnalysisNumericOverride::default();
    numerics
        .set_for_instance(
            AnalysisKind::Ac,
            Default::default(),
            crate::simulation::plan::NumericOverrideOption::Reltol,
            "1e-5",
        )
        .unwrap();
    plan.set_numeric_override(ac, Some(numerics)).unwrap();
    let draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
        statistics: Some(
            crate::simulation::dialog::mc::statistics::McStatisticsConfig {
                variations: vec![
                    crate::simulation::dialog::mc::statistics::McParameterVariation {
                        bounds: Some(
                            crate::simulation::dialog::mc::statistics::McParameterBounds {
                                lower: Some(900.0),
                                upper: Some(1100.0),
                                sigma_cutoff: Some(3.0),
                                max_attempts: 10000,
                            },
                        ),
                        parameter: "rval".into(),
                        scope: crate::simulation::dialog::mc::statistics::McScope::Process,
                        distribution: crate::simulation::dialog::mc::statistics::McShape::Gaussian,
                        spread: 10.0,
                        percent: true,
                    },
                ],
                correlations: Vec::new(),
            },
        ),
        variation_source: crate::simulation::dialog::McVariationSource::DeckStatistics,
        base_analysis: Some(ac),
        measurements: vec!["gain".into(), "last:V(out)".into()],
        histogram_bins: 7,
        num_runs: 3,
        ..Default::default()
    }));
    for mut restored in [
        serde_json::from_str::<AnalysisDraft>(&serde_json::to_string(&draft).unwrap()).unwrap(),
        ron::from_str::<AnalysisDraft>(&ron::to_string(&draft).unwrap()).unwrap(),
    ] {
        restored.prepare_after_restore();
        let AnalysisDraft::MonteCarlo(mut restored) = restored else {
            unreachable!()
        };
        restored.ensure_initialized();
        let config = restored.to_config().unwrap();
        assert_eq!(config.base_analysis, Some(ac));
        assert_eq!(config.histogram_bins, 7);
        assert_eq!(
            config.statistics.as_ref().unwrap().variations[0].spread,
            10.0
        );
        assert_eq!(config.measurements, ["gain", "last:V(out)"]);
    }
    plan.edit(mc, |target| *target = draft).unwrap();
    let frozen = plan.freeze().unwrap();
    // Later live edits must not replace the base in this prepared plan.
    plan.edit(ac, |draft| {
        let AnalysisDraft::Ac(draft) = draft else {
            unreachable!()
        };
        draft.fstop = "3000".into();
    })
    .unwrap();
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .unwrap();
    let controller = SimulationController::new();
    let tasks = controller
        .build_queue_from_plan(&state, &frozen, &sealed)
        .unwrap();
    let task = tasks.iter().find(|task| task.instance_id() == mc).unwrap();
    let base = task
        .queued_analysis()
        .spec_options
        .study_base
        .as_ref()
        .unwrap();
    assert_eq!(base.instance_id, ac);
    let Some(AnalysisConfig::Ac(config)) = base.analysis.as_basic() else {
        panic!("AC base")
    };
    assert_eq!(config.stop_freq, 1000.0);
    assert!(base.numeric_options.to_ascii_uppercase().contains("RELTOL"));
    assert_eq!(
        task.queued_analysis()
            .spec_options
            .mc_statistics
            .as_ref()
            .unwrap()
            .variations[0]
            .spread,
        10.0
    );
    for change in 0..15 {
        let mut queued = task.queued_analysis().clone();
        let base = queued.spec_options.study_base.as_mut().unwrap();
        match change {
            0 => base.instance_id = other,
            1 => base.histogram_bins += 1,
            2 => base.measurements = vec!["last:V(out)".into()],
            3 => base.numeric_options = ".OPTIONS RELTOL=0.02".into(),
            4 => {
                let Some(AnalysisConfig::Ac(config)) = base.analysis.as_basic_mut() else {
                    unreachable!()
                };
                config.stop_freq = 2000.0;
            }
            _ => {
                use crate::simulation::dialog::mc::statistics::{McScope, McShape};
                let row = &mut queued
                    .spec_options
                    .mc_statistics
                    .as_mut()
                    .unwrap()
                    .variations[0];
                match change {
                    5 => row.parameter = "other".into(),
                    6 => row.scope = McScope::Mismatch,
                    7 => row.distribution = McShape::Uniform,
                    8 => row.spread = 20.0,
                    9 => row.percent = false,
                    10 => row.bounds.as_mut().unwrap().lower = Some(800.0),
                    11 => row.bounds.as_mut().unwrap().upper = Some(1200.0),
                    12 => row.bounds.as_mut().unwrap().sigma_cutoff = Some(4.0),
                    13 => row.bounds.as_mut().unwrap().max_attempts = 20000,
                    _ => row.bounds = None,
                }
            }
        }
        let changed = PreparedTask::new(mc, task.source_revision(), vec![], "MC", queued);
        assert_ne!(task.config_digest(), changed.config_digest());
    }
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    plan.set_enabled(ac, false).unwrap();
    let frozen = plan.freeze().unwrap();
    let errors = controller
        .build_queue_from_plan(&state, &frozen, &sealed)
        .unwrap_err();
    assert!(
        errors
            .iter()
            .any(|error| error.contains("missing or disabled")),
        "{errors:?}"
    );
}

#[test]
fn configured_optimization_base_persists_and_freezes_before_live_edits() {
    use crate::simulation::dialog::optimization::{OptimizationConfig, OptimizationDialogState};
    use crate::simulation::optimizer::{OptimizationObjectiveGoal, OptimizationObjectiveTerm};
    let objectives = vec![OptimizationObjectiveTerm {
        measurement: "gain".into(),
        unit: "mV".into(),
        goal: OptimizationObjectiveGoal::Target,
        target: Some(2.5),
        scale: 0.5,
        weight: 3.0,
    }];
    let mut state = AppState::default();
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let (ac, _) = plan.insert(AnalysisKind::Ac).unwrap();
    plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
        .unwrap();
    let (opt, _) = plan.insert(AnalysisKind::Optimization).unwrap();
    let mut setup = OptimizationDialogState::from_config(&OptimizationConfig {
        base_analysis: Some(ac),
        objective_measurement: "gain".into(),
        objective_terms: objectives.clone(),
        constraints: vec![crate::simulation::optimizer::OptimizationConstraint {
            measurement: "gain".into(),
            unit: "mV".into(),
            lower: Some(1.0),
            upper: Some(3.0),
            tolerance: 0.01,
            scale: 2.0,
        }],
        ..Default::default()
    });
    // Inactive expression buffers survive switching back without blocking a measured objective.
    setup.objective_expression = "unfinished{".into();
    setup.objective_node.clear();
    setup.target_value = "inactive unfinished".into();
    let draft = AnalysisDraft::Optimization(setup);
    for mut restored in [
        serde_json::from_str::<AnalysisDraft>(&serde_json::to_string(&draft).unwrap()).unwrap(),
        ron::from_str::<AnalysisDraft>(&ron::to_string(&draft).unwrap()).unwrap(),
    ] {
        restored.prepare_after_restore();
        let AnalysisDraft::Optimization(ref mut setup) = restored else {
            unreachable!()
        };
        setup.ensure_initialized();
        let config = setup.to_config().unwrap();
        assert_eq!(config.base_analysis, Some(ac));
        assert_eq!(config.objective_measurement, "gain");
        assert_eq!(setup.objective_expression, "unfinished{");
        assert_eq!(config.objective_terms, objectives);
        assert_eq!(config.constraints[0].upper, Some(3.0));
        assert_eq!(setup.target_value, "inactive unfinished");
        assert!(config.to_spice().contains("weighted_objectives="));
    }
    plan.edit(opt, |target| *target = draft).unwrap();
    let frozen = plan.freeze().unwrap();
    plan.edit(opt, |draft| {
        let AnalysisDraft::Optimization(setup) = draft else {
            unreachable!()
        };
        setup.objective_measurement = "changed".into();
        setup.objective_terms[0].weight = "9".into();
        setup.constraints[0].upper = "99".into();
    })
    .unwrap();
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .unwrap();
    let tasks = SimulationController::new()
        .build_queue_from_plan(&state, &frozen, &sealed)
        .unwrap();
    let task = tasks.iter().find(|task| task.instance_id() == opt).unwrap();
    let base = task
        .queued_analysis()
        .spec_options
        .study_base
        .as_ref()
        .unwrap();
    assert_eq!(base.instance_id, ac);
    assert_eq!(base.measurements, ["gain"]);
    assert_eq!(base.objective_terms, objectives);
    assert_eq!(base.constraints[0].upper, Some(3.0));
    assert_eq!(base.constraints[0].unit, "mV");
    for change in 0..12 {
        let mut queued = task.queued_analysis().clone();
        let term = &mut queued
            .spec_options
            .study_base
            .as_mut()
            .unwrap()
            .objective_terms[0];
        match change {
            10 => term.unit = "V".into(),
            0 => term.weight = 4.0,
            1 => term.scale = 2.0,
            2 => term.target = Some(7.0),
            3 => term.goal = OptimizationObjectiveGoal::Maximize,
            4 => {
                let AnalysisSpec::Optimization { search, .. } = &mut queued.spec else {
                    unreachable!()
                };
                search.variable_domains.insert(
                    "RLOAD".into(),
                    crate::simulation::optimizer::OptimizationVariableDomain::Logarithmic,
                );
            }
            _ => {
                let constraint =
                    &mut queued.spec_options.study_base.as_mut().unwrap().constraints[0];
                match change {
                    11 => constraint.unit = "V".into(),
                    5 => constraint.measurement = "other".into(),
                    6 => constraint.lower = None,
                    7 => constraint.upper = Some(4.0),
                    8 => constraint.tolerance = 0.02,
                    _ => constraint.scale = 3.0,
                }
            }
        }
        let changed = PreparedTask::new(opt, task.source_revision(), vec![], "OPT", queued);
        assert_ne!(task.config_digest(), changed.config_digest());
    }
    assert!(matches!(
        base.analysis.as_basic(),
        Some(AnalysisConfig::Ac(_))
    ));
}

#[test]
fn frozen_noise_task_keeps_exact_draft_and_reference_pvt() {
    let mut state = AppState::default();
    state.sim_setup.reference_pvt.temperature_celsius = -40.0;
    state.sim_setup.noise.output = "singleton_must_not_leak".to_owned();
    state.sim_setup.ac.points = "777".to_owned();
    let plan = state.sim_setup.analysis_plan.as_mut().expect("stable plan");
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    let (noise, _) = plan.insert(AnalysisKind::Noise).expect("noise inserts");
    plan.edit(noise, |draft| {
        let AnalysisDraft::Noise(draft) = draft else {
            panic!("noise draft")
        };
        draft.output = "V(out,ref)".to_owned();
        draft.input = "VSTIM".to_owned();
        draft.sweep = crate::simulation::config::NoiseSweepType::ExplicitFrequencyList;
        draft.explicit_frequencies = "1, 5, 25".to_owned();
        draft.contribution_detail = crate::simulation::config::NoiseContributionDetail::Top20;
        draft.integration_mode = crate::simulation::config::NoiseIntegrationMode::Disabled;
    })
    .expect("noise edits");
    plan.bind_dependency(noise, AnalysisKind::OperatingPoint, op)
        .expect("noise binds OP");

    let controller = SimulationController::new();
    let frozen = controller
        .build_analysis_plan(&state)
        .expect("plan freezes");
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .expect("model sources seal");
    let tasks = controller
        .build_queue_from_plan(&state, &frozen, &sealed)
        .expect("noise plan compiles");
    let task = tasks
        .iter()
        .find(|task| task.instance_id() == noise)
        .expect("noise task");
    assert!(matches!(
        &task.queued_analysis().spec,
        AnalysisSpec::Noise {
            output_node,
            reference_node,
            input_source,
            explicit_frequencies: Some(frequencies),
            contribution_detail: crate::simulation::config::NoiseContributionDetail::Top20,
            integration_mode: crate::simulation::config::NoiseIntegrationMode::Disabled,
            temperature,
            ..
        } if output_node == "out"
            && reference_node == "ref"
            && input_source == "VSTIM"
            && frequencies == &[1.0, 5.0, 25.0]
            && (*temperature - 233.15).abs() < 1.0e-12
    ));
    let Some(AnalysisConfig::Noise(config)) = &task.queued_analysis().config else {
        panic!("noise config retained")
    };
    assert_eq!(config.output_node, "out");
    assert_eq!(config.input_source, "VSTIM");
    assert_eq!(config.num_points, 3);
    assert!((config.temperature_kelvin - 233.15).abs() < 1.0e-12);
}

#[test]
fn frozen_plan_ids_revisions_and_exact_dependency_bindings_reach_prepared_tasks() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("new state owns a stable plan");
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    let (first_pss, _) = plan.insert(AnalysisKind::Pss).expect("first PSS inserts");
    let (second_pss, _) = plan.insert(AnalysisKind::Pss).expect("second PSS inserts");
    plan.edit(first_pss, |draft| {
        let AnalysisDraft::Pss(draft) = draft else {
            panic!("expected PSS draft");
        };
        draft.fund_freq = "1Meg".to_owned();
        // A driven solve needs a tone, and only the design can name one.
        draft.tone_sources = "VSRC".to_owned();
    })
    .expect("first PSS edits");
    plan.edit(second_pss, |draft| {
        let AnalysisDraft::Pss(draft) = draft else {
            panic!("expected PSS draft");
        };
        draft.fund_freq = "2Meg".to_owned();
        draft.tone_sources = "VSRC".to_owned();
    })
    .expect("second PSS edits");
    plan.bind_dependency(first_pss, AnalysisKind::OperatingPoint, op)
        .expect("first PSS binds OP");
    plan.bind_dependency(second_pss, AnalysisKind::OperatingPoint, op)
        .expect("second PSS binds OP");
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    plan.bind_dependency(pac, AnalysisKind::Pss, first_pss)
        .expect("PAC binds exact first PSS");
    let expected_revision = plan.revision();

    let controller = SimulationController::new();
    let frozen = controller
        .build_analysis_plan(&state)
        .expect("plan freezes");
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .expect("default model sources seal");
    let tasks = controller
        .build_queue_from_plan(&state, &frozen, &sealed)
        .expect("frozen plan compiles");
    let pac_task = tasks
        .iter()
        .find(|task| task.instance_id() == pac)
        .expect("PAC task is present");

    assert_eq!(pac_task.source_revision(), expected_revision);
    assert_eq!(pac_task.dependencies(), &[first_pss]);
    let pac_options = pac_task
        .queued_analysis()
        .spec_options
        .pac
        .as_ref()
        .expect("PAC options compile");
    assert!((pac_options.pss_fundamental_freq - 1.0e6).abs() < 1.0e-6);
    assert!((pac_options.pss_fundamental_freq - 2.0e6).abs() > 1.0);
}

#[test]
fn same_kind_sparameter_instances_freeze_independent_export_policies() {
    let mut state = AppState::default();
    state.schematic.current_file = Some(std::path::PathBuf::from("rf/duplexer.rsch"));
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("new state owns a stable plan");
    let (op_id, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("operating-point prerequisite inserts");
    let (v1_id, _) = plan
        .insert(AnalysisKind::SParameter)
        .expect("first S-parameter analysis inserts");
    let (disabled_id, _) = plan
        .insert(AnalysisKind::SParameter)
        .expect("second S-parameter analysis inserts");
    plan.bind_dependency(v1_id, AnalysisKind::OperatingPoint, op_id)
        .expect("first S-parameter analysis binds OP");
    plan.bind_dependency(disabled_id, AnalysisKind::OperatingPoint, op_id)
        .expect("second S-parameter analysis binds OP");
    plan.edit(v1_id, |draft| {
        let AnalysisDraft::SParameter(draft) = draft else {
            panic!("expected S-parameter draft");
        };
        draft.touchstone_export = true;
        draft.touchstone_version = 1;
    })
    .expect("first policy edits");
    plan.edit(disabled_id, |draft| {
        let AnalysisDraft::SParameter(draft) = draft else {
            panic!("expected S-parameter draft");
        };
        draft.touchstone_export = false;
        draft.touchstone_version = 2;
    })
    .expect("second policy edits");

    let controller = SimulationController::new();
    let frozen = controller
        .build_analysis_plan(&state)
        .expect("plan freezes");
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .expect("default model sources seal");
    let tasks = controller
        .build_queue_from_plan(&state, &frozen, &sealed)
        .expect("same-kind S-parameter tasks compile");
    let first_policy = tasks
        .iter()
        .find(|task| task.instance_id() == v1_id)
        .and_then(|task| task.touchstone_export_policy())
        .expect("first task freezes an explicit policy");
    let second_policy = tasks
        .iter()
        .find(|task| task.instance_id() == disabled_id)
        .and_then(|task| task.touchstone_export_policy())
        .expect("second task freezes an explicit policy");

    assert_eq!(first_policy.version(), Some(1));
    assert!(first_policy.output_path(4, 1, 2).is_some());
    assert_eq!(second_policy.version(), None);
    assert!(second_policy.output_path(4, 2, 2).is_none());
}
