use super::*;
use crate::engine_bridge::EngineBridge;
use crate::transient_checkpoint::{CheckpointExecution, TransientCheckpointRequest};
use rspice_core::{NoAbort, ResourceLimits};
use rspice_results::transient_checkpoint::{
    TransientCheckpointEvidence, TransientCheckpointLibrary,
};
use rspice_simulation_contract::tran_checkpoint::TranCheckpointDraft;
use std::sync::Mutex;

fn experiment() -> SnapshotParts {
    let mut input = parts();
    let spec = AnalysisSpec::Transient {
        stop_time: 20e-9,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: Some(1e-9),
        uic: false,
    };
    let config = crate::analysis_preparation::analysis_spec_to_config(&spec).unwrap();
    input.executable_netlist =
        "Resume points\n.param r=1k\nVDD in 0 sin(1 0.1 1meg)\nR1 in out {r}\nC1 out 0 1p\n.end\n"
            .into();
    input.tasks = vec![prepared(
        "tran",
        "Transient",
        QueuedAnalysis {
            analysis_line: config.to_spice(),
            spec,
            config: Some(config),
            numeric_override: None,
            spec_options: SpecExecutionOptions {
                tran_checkpoint: Some(TransientCheckpointRequest {
                    times: vec![7e-9, 14e-9, 20e-9],
                    resume: None,
                }),
                ..Default::default()
            },
        },
    )];
    input
}

fn execute(
    task: &PreparedTask,
    source: &str,
) -> (
    crate::results::SimulationResult,
    Vec<TransientCheckpointEvidence>,
) {
    let images = Mutex::new(Vec::new());
    let observer = |bytes: &[u8]| {
        images
            .lock()
            .unwrap()
            .push(TransientCheckpointEvidence::from_bytes(bytes.to_vec().into()).unwrap());
        Ok(())
    };
    let config = crate::analysis_preparation::analysis_spec_to_config(&task.task.spec).unwrap();
    let result = EngineBridge::new()
        .run_with_transient_checkpoint(
            &config,
            task.executable_netlist_override
                .as_deref()
                .unwrap_or(source),
            None,
            task.execution_environment.clone(),
            CheckpointExecution {
                request: task.task.spec_options.tran_checkpoint.as_ref().unwrap(),
                observer: &observer,
            },
            &NoAbort,
        )
        .unwrap();
    (result, images.into_inner().unwrap())
}

fn select(input: &mut SnapshotParts, images: Vec<TransientCheckpointEvidence>) {
    input.tasks[0] = input.tasks[0].clone().with_transient_resumes(images);
}

#[test]
fn transient_checkpoint_authored_points_route_immutable_inputs_and_continue_exactly() {
    for run_set in [
        global_parameter_run_set("r", &["1k", "2k"]),
        global_temperature_run_set(&["27", "85"]),
    ] {
        let mut input = experiment();
        input.run_set = Some(run_set.clone());
        let fresh = PreparedRunSnapshot::new(input).unwrap();
        assert_eq!(fresh.tasks.len(), 2);
        let baselines: Vec<_> = fresh
            .tasks
            .iter()
            .map(|task| execute(task, &fresh.executable_netlist))
            .collect();
        let images: Vec<_> = baselines
            .iter()
            .map(|(_, images)| images[0].clone())
            .collect();
        let mut library = TransientCheckpointLibrary::default();
        for (i, image) in images.iter().enumerate() {
            library.insert(format!("point-{i}"), image.clone()).unwrap();
        }
        let draft = TranCheckpointDraft {
            interval: Some("7n".into()),
            resume: images.iter().map(|image| image.digest()).collect(),
        };
        let selected = crate::transient_checkpoint::preparation::selected_images(
            std::iter::empty::<&rspice_results::analysis_result::AnalysisResult<()>>(),
            &library,
            Some(&draft),
        )
        .unwrap();
        for image in &images {
            library.remove(image.digest());
        }
        let mut resumed = experiment();
        resumed.run_set = Some(run_set);
        select(&mut resumed, selected);
        let resumed = PreparedRunSnapshot::new(resumed).unwrap();
        for (i, task) in resumed.tasks.iter().enumerate() {
            let request = task.task.spec_options.tran_checkpoint.as_ref().unwrap();
            assert_eq!(
                request.resume.as_ref().unwrap().digest(),
                images[i].digest()
            );
            assert!(
                request
                    .times
                    .iter()
                    .all(|time| *time > images[i].accepted_time())
            );
            let (result, final_images) = execute(task, &resumed.executable_netlist);
            let crate::results::SimulationResult::Transient {
                time: actual_time,
                waveforms: actual,
                ..
            } = result
            else {
                panic!()
            };
            let crate::results::SimulationResult::Transient {
                time, waveforms, ..
            } = &baselines[i].0
            else {
                panic!()
            };
            let seam = time
                .iter()
                .position(|time| *time == images[i].accepted_time())
                .unwrap();
            assert_eq!(actual_time, time[seam..]);
            for (name, waveform) in actual {
                assert_eq!(waveform.y_values, waveforms[&name].y_values[seam..]);
            }
            assert_eq!(final_images.last(), baselines[i].1.last());
        }
    }
}

#[test]
fn transient_checkpoint_preparation_refuses_wrong_missing_ambiguous_and_malformed_selections() {
    let fresh = PreparedRunSnapshot::new(experiment()).unwrap();
    let (_, images) = execute(&fresh.tasks[0], &fresh.executable_netlist);
    for case in [
        "circuit",
        "temperature",
        "supply",
        "uic",
        "solver",
        "horizon",
        "ambiguous",
        "schedule",
        "unused",
        "unmatched",
        "forged",
    ] {
        let mut input = experiment();
        select(&mut input, vec![images[0].clone()]);
        match case {
            "circuit" => {
                input.executable_netlist = input.executable_netlist.replace("r=1k", "r=2k")
            }
            "temperature" => input.run_set = Some(global_temperature_run_set(&["85"])),
            "supply" => input.run_set = Some(global_supply_temperature_run_set("1.8", "27")),
            "uic" => {
                let AnalysisSpec::Transient { uic, .. } = &mut input.tasks[0].task.spec else {
                    unreachable!()
                };
                *uic = true;
                input.tasks[0].task.config = Some(
                    crate::analysis_preparation::analysis_spec_to_config(&input.tasks[0].task.spec)
                        .unwrap(),
                );
            }
            "solver" => {
                input.executable_netlist = input
                    .executable_netlist
                    .replace(".end", ".options reltol=1e-5\n.end")
            }
            "horizon" => select(&mut input, vec![images.last().unwrap().clone()]),
            "ambiguous" => select(&mut input, images[..2].to_vec()),
            "schedule" => {
                input.tasks[0]
                    .task
                    .spec_options
                    .tran_checkpoint
                    .as_mut()
                    .unwrap()
                    .times = vec![f64::NAN]
            }
            "unused" => {
                let mut other = experiment();
                other.executable_netlist = other.executable_netlist.replace("r=1k", "r=2k");
                let other = PreparedRunSnapshot::new(other).unwrap();
                let (_, other_images) = execute(&other.tasks[0], &other.executable_netlist);
                select(&mut input, vec![images[0].clone(), other_images[0].clone()]);
            }
            "unmatched" => input.run_set = Some(global_parameter_run_set("r", &["1k", "2k"])),
            "forged" => input.tasks[0].config_digest = ContentDigest::from_bytes([19; 32]),
            _ => unreachable!(),
        }
        if case != "forged" {
            input.tasks[0].config_digest = input.tasks[0].payload_digest();
        }
        assert!(PreparedRunSnapshot::new(input).is_err(), "{case}");
    }
    let draft = TranCheckpointDraft {
        interval: None,
        resume: vec![images[0].digest()],
    };
    assert!(
        crate::transient_checkpoint::preparation::selected_images(
            std::iter::empty::<&rspice_results::analysis_result::AnalysisResult<()>>(),
            &TransientCheckpointLibrary::default(),
            Some(&draft)
        )
        .unwrap_err()
        .contains("unavailable")
    );
    let mut input = experiment();
    select(&mut input, vec![images[0].clone()]);
    let abort = Arc::new(std::sync::atomic::AtomicBool::new(true));
    assert!(
        super::super::transient_checkpoint::route_resumes(
            &mut input,
            ResourceLimits::default(),
            &abort
        )
        .unwrap_err()
        .is_aborted()
    );
    let mut limits = ResourceLimits::default();
    limits.max_result_values = 1;
    let error = super::super::transient_checkpoint::route_resumes(&mut input, limits, &NoAbort)
        .unwrap_err();
    assert_eq!(
        error.resource_limit().unwrap().resource,
        rspice_core::ResourceKind::ResultValues
    );
    input.execution_limits = limits;
    let error = PreparedRunSnapshot::new(input).err().unwrap();
    assert_eq!(
        error.resource_limit().unwrap().resource,
        rspice_core::ResourceKind::ResultValues
    );
}

#[test]
fn transient_checkpoint_authored_schedule_is_bounded_and_legacy_draft_stays_unchanged() {
    use crate::transient_checkpoint::preparation::request_from_draft;
    let legacy = rspice_simulation_contract::drafts::TranSetup::default();
    let json = serde_json::to_value(&legacy).unwrap();
    assert!(json.get("checkpoint").is_none());
    assert!(
        serde_json::from_value::<rspice_simulation_contract::drafts::TranSetup>(json)
            .unwrap()
            .checkpoint
            .is_none()
    );
    let mut draft = TranCheckpointDraft {
        interval: Some("7n".into()),
        resume: vec![],
    };
    let request = request_from_draft(Some(&draft), 20e-9).unwrap().unwrap();
    assert_eq!(request.times.len(), 3);
    for (actual, expected) in request.times.iter().zip([7e-9, 14e-9, 20e-9]) {
        assert!((actual - expected).abs() < 1e-23);
    }
    for interval in ["0", "-1n", "NaN", "inf", "1e-300"] {
        draft.interval = Some(interval.into());
        assert!(
            request_from_draft(Some(&draft), 20e-9).is_err(),
            "{interval}"
        );
    }
}

#[test]
fn transient_checkpoint_dependency_windows_and_rebinding_preserve_producer_identity() {
    let fresh = PreparedRunSnapshot::new(experiment()).unwrap();
    let (_, images) = execute(&fresh.tasks[0], &fresh.executable_netlist);
    for (start, forged) in [(0.0, false), (10e-9, false), (10e-9, true)] {
        let mut input = experiment();
        select(&mut input, vec![images[0].clone()]);
        let producer = &input.tasks[0];
        let mut consumer = prepared_with(
            "fourier",
            ObjectRevision::INITIAL,
            vec![producer.instance_id],
            "Fourier",
            QueuedAnalysis {
                numeric_override: None,
                config: None,
                analysis_line: ".four 1meg V(out)".into(),
                spec_options: Default::default(),
                spec: AnalysisSpec::Fourier {
                    fundamental_freq: 1e9,
                    num_harmonics: 1,
                    num_periods: 1,
                    output_node: "out".into(),
                    output_ref: "0".into(),
                    additional_outputs: vec![],
                    start_time: start,
                    stop_time: 20e-9,
                    compute_thd: true,
                    normalize: false,
                },
            },
        );
        let before = if forged {
            ContentDigest::from_bytes([41; 32])
        } else {
            producer.config_digest
        };
        consumer.set_dependency_bindings(vec![PreparedDependencyBinding::transient_trajectory(
            producer.instance_id,
            producer.source_revision,
            before,
        )]);
        input.tasks.push(consumer);
        let outcome = super::super::transient_checkpoint::route_resumes(
            &mut input,
            ResourceLimits::default(),
            &NoAbort,
        );
        if start == 0.0 {
            assert!(
                outcome
                    .unwrap_err()
                    .message()
                    .contains("before the checkpoint")
            );
        } else {
            outcome.unwrap();
            let bound = input.tasks[1].dependency_bindings[0].producer_config_digest();
            if forged {
                assert_eq!(bound, before);
                assert_ne!(bound, input.tasks[0].config_digest);
            } else {
                assert_eq!(bound, input.tasks[0].config_digest);
                assert_ne!(bound, before);
            }
        }
    }
    let mut input = experiment();
    input.executable_netlist = input
        .executable_netlist
        .replace(".end", ".tran 1n 20n NOISEFMAX=1e9\n.end");
    let error = PreparedRunSnapshot::new(input).err().unwrap();
    assert!(error.message().contains("NOISEFMAX"), "{error}");
}
