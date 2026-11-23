//! Retained snapshots share complete records and validation, but never edits.

use super::*;

fn validation_passes() -> usize {
    rspice_formats::project_results::RESULT_VALIDATIONS.with(std::cell::Cell::get)
}

#[test]
fn result_snapshot_clones_share_complete_history_records() {
    let state = super::waveform_storage::complex_history();
    let snapshot = crate::io::capture_simulation_results(&state);
    let candidate = snapshot.clone();
    assert_eq!(
        snapshot.runs.as_ptr(),
        candidate.runs.as_ptr(),
        "cloning a snapshot copied retained run records"
    );
    assert_eq!(
        snapshot.runs[0].analyses.as_ptr(),
        candidate.runs[0].analyses.as_ptr()
    );
    let mut edited = candidate;
    edited.runs[0].label.push_str(" changed");
    assert_ne!(snapshot.runs.as_ptr(), edited.runs.as_ptr());
    assert_ne!(snapshot.runs[0].label, edited.runs[0].label);
}

#[test]
fn immutable_result_validation_is_shared_and_mutation_invalidates_both_verdicts() {
    for samples in [2, 100_000] {
        let mut state = super::waveform_storage::complex_history();
        state.retained.runs[0].analyses[0].waveforms = vec![RetainedTrace::new(
            "V(out)",
            (1..=samples).map(|n| n as f64).collect::<Vec<_>>(),
            vec![1.0; samples],
            "#ffffff",
        )];
        let accepted = super::waveform_storage::records_held_in_memory(&state);
        let mut candidate = accepted.clone();
        let frozen = serde_json::to_vec(&accepted).unwrap();
        let before = validation_passes();
        for _ in 0..6 {
            accepted.validate().unwrap();
            candidate.validate().unwrap();
        }
        assert_eq!(
            validation_passes() - before,
            1,
            "{samples} samples: unchanged clones repeated numerical validation"
        );
        assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);

        // Valid metadata changes must detach even when no sample changes.
        candidate.retained_dataset_limit = Some(1);
        let before = validation_passes();
        candidate.validate().unwrap();
        candidate.validate().unwrap();
        accepted.validate().unwrap();
        assert_eq!(validation_passes() - before, 1);
        assert_eq!(accepted.retained_dataset_limit, None);

        // A nested edit cannot inherit an earlier success. Repeating the
        // same invalid query is cached too, until the data is repaired.
        candidate.runs[0].analyses[0].waveforms[0].y.make_mut()[1] = 9.0;
        let before = validation_passes();
        let failure = candidate.validate().unwrap_err();
        assert_eq!(candidate.validate().unwrap_err(), failure);
        accepted.validate().unwrap();
        assert_eq!(validation_passes() - before, 1);
        assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);

        candidate.runs[0].analyses[0].waveforms[0].y.make_mut()[1] = 1.0;
        let before = validation_passes();
        candidate.validate().unwrap();
        candidate.validate().unwrap();
        assert_eq!(validation_passes() - before, 1);

        let reopened: ProjectSimulationResults = serde_json::from_slice(&frozen).unwrap();
        let before = validation_passes();
        reopened.validate().unwrap();
        reopened.validate().unwrap();
        assert_eq!(
            validation_passes() - before,
            1,
            "serialized content cannot import cached validation authority"
        );
        assert_eq!(serde_json::to_vec(&reopened).unwrap(), frozen);
    }
}

/// A run of one AC analysis of `signals` traces on `points` frequencies, kept as the controller
/// keeps a finished analysis.
fn large_ac_run(id: u64, points: usize, signals: usize) -> SimulationRun {
    let frequencies: Vec<f64> = (1..=points).map(|n| n as f64 * 10.0).collect();
    let waveforms = (0..signals)
        .map(|k| {
            let y: Vec<f64> = frequencies
                .iter()
                .map(|f| (f * (k + 1) as f64 * 1.0e-4).sin() + id as f64)
                .collect();
            RetainedTrace::new(format!("V(n{k})"), frequencies.clone(), y, "#ffffff")
        })
        .collect();
    let mut run = SimulationRun::new(id);
    run.keep_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(waveforms));
    super::seal_legacy_unattributed(&mut run);
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    run
}

#[test]
fn a_project_of_twenty_large_runs_saves_under_a_mebibyte_and_reopens_every_run() {
    const POINTS: usize = 10_000;
    let mut libraries = crate::state::LibraryManager::with_primitives();
    let workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut simulation = SimulationState::default();
    simulation.retained.runs = (1..=20)
        .map(|id| large_ac_run(id, POINTS, 4))
        .collect::<Vec<_>>()
        .into();
    simulation.retained.next_run_id = 20;
    let results = crate::io::capture_simulation_results(&simulation);
    let root = std::env::temp_dir().join(format!("rspice-twenty-runs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let file = root.join("twenty.rspiceproj");
    crate::io::result_segments::keep_segments_now(results.stored_segments(), &file)
        .expect("every run's segment is kept beside the project");
    let project = ProjectSnapshot::new_with_simulation_results(workspace, libraries, results);
    let json = serialize_project_file(&project).expect("the project serializes");
    assert!(
        json.len() < 1 << 20,
        "twenty runs of {} samples each wrote a {} byte project file",
        POINTS * 4,
        json.len()
    );

    rspice_results::store_reader::keep_segments_in(
        &rspice_formats::project_results::segment_folder(&file),
    );
    let loaded = load_project_text(&json, Some(&file)).expect("the project reopens");
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("its runs restore");
    assert_eq!(restored.retained.runs.len(), 20);
    for (before, after) in simulation
        .retained
        .runs
        .iter()
        .zip(restored.retained.runs.iter())
    {
        let (before, after) = (&before.analyses[0], &after.analyses[0]);
        for (left, right) in before.waveforms.iter().zip(&after.waveforms) {
            assert!(right.y.is_stored());
            assert_eq!(right.y.len(), POINTS);
            assert_eq!(right.y.ends(), left.y.ends(), "{}", right.name);
        }
    }
    drop((simulation, restored));
    let _ = std::fs::remove_dir_all(&root);
}

/// A running run's preview is a preview: its record names none of its columns, whether the
/// preview holds its samples in memory or reads them from the run's segment, as a long run's does.
#[test]
fn a_running_runs_record_names_none_of_its_preview_columns_however_it_holds_them() {
    let waveforms = (0..3)
        .map(|k| {
            RetainedTrace::new(
                format!("I(R{k})"),
                vec![0.0, 1.0e-9],
                vec![1.0e-3, 2.0e-3],
                "#ffffff",
            )
        })
        .collect();
    let mut run = SimulationRun::new(1);
    run.keep_analysis(
        AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "Transient")
            .with_waveforms(waveforms),
    );
    run.store_samples()
        .expect("the preview's columns are kept in a segment");
    assert!(!run.holds_samples_in_memory());

    let record = crate::io::capture_run(&run);
    let written = serde_json::to_string(&record).expect("the record is written");
    assert!(!written.contains("I(R1)"), "{written}");
}

#[test]
fn a_saved_file_refuses_samples_held_in_memory_and_other_writers_keep_them() {
    let state = super::waveform_storage::complex_history();
    let record =
        rspice_formats::project_results::ProjectSimulationRun::from(&state.retained.runs[0]);
    let refused = rspice_formats::project_results::saving_file(|| serde_json::to_string(&record))
        .unwrap_err();
    assert!(
        refused
            .to_string()
            .contains(rspice_formats::project_results::SAMPLES_IN_MEMORY),
        "{refused}"
    );
    // A hardcopy worker's snapshot is not a saved file: it carries the samples themselves.
    assert!(
        serde_json::to_string(&record)
            .unwrap()
            .contains("[1.0,2.0]")
    );
    // The capture a save writes keeps the run in the store first, and names it.
    let captured = crate::io::capture_simulation_results(&state);
    let saved = rspice_formats::project_results::saving_file(|| serde_json::to_string(&captured))
        .expect("a captured run is written by reference");
    assert!(!saved.contains("[1.0,2.0]"), "{saved}");
    assert_eq!(captured.stored_segments().len(), 1);
}

/// A snapshot for a reader in another memory, as a browser's hardcopy worker is, carries each
/// stored window's rows beside the window it names: it restores without the segment, with every
/// sample and the run's digest as they were. A saved file that carries rows is refused.
#[test]
fn a_snapshot_for_another_memory_carries_the_rows_of_its_stored_windows() {
    use rspice_formats::project_results::{ProjectSimulationResults, carrying_rows};
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![large_ac_run(1, 64, 2)].into();
    simulation.retained.next_run_id = 1;
    let results = crate::io::capture_simulation_results(&simulation);
    let named = serde_json::to_string(&results).expect("the results are written");
    let carried =
        carrying_rows(|| serde_json::to_string(&results)).expect("the snapshot is written");
    assert!(!named.contains("\"held\""), "{named}");
    assert!(carried.contains("\"held\""), "{carried}");

    let read = carrying_rows(|| serde_json::from_str::<ProjectSimulationResults>(&carried))
        .expect("the snapshot is read");
    let restored = crate::io::simulation_state_from_results(read).expect("its run restores");
    let (before, after) = (&simulation.retained.runs[0], &restored.retained.runs[0]);
    assert_eq!(
        after.dataset_content_digest(),
        before.dataset_content_digest()
    );
    assert_eq!(run_bits(after), run_bits(before));
    for waveform in &after.analyses[0].waveforms {
        for values in [&waveform.x, &waveform.y] {
            let (source, _) = values.stored_window().expect("a stored window");
            assert!(
                source.store().contains("snapshot carries"),
                "{} reads {}",
                waveform.name,
                source.store()
            );
        }
    }

    let refused = serde_json::from_str::<ProjectSimulationResults>(&carried)
        .expect_err("a saved file carries no rows");
    assert!(
        refused.to_string().contains("carries none of their rows"),
        "{refused}"
    );
}

/// Each sample of each waveform of `run`'s first analysis, bit for bit.
fn run_bits(run: &SimulationRun) -> Vec<Vec<u64>> {
    run.analyses[0]
        .waveforms
        .iter()
        .flat_map(|waveform| [&waveform.x, &waveform.y])
        .map(|values| values.samples(..).map(f64::to_bits).collect())
        .collect()
}

/// A project of `runs` stored runs saved as `file` with its segments beside it, then closed: no
/// reader holds any of its segments open. Its text and each run's samples.
fn saved_and_closed(file: &Path, runs: &[u64]) -> (String, Vec<Vec<Vec<u64>>>) {
    let mut libraries = crate::state::LibraryManager::with_primitives();
    let workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut simulation = SimulationState::default();
    simulation.retained.runs = runs
        .iter()
        .map(|&id| large_ac_run(id, 3_000, 2))
        .collect::<Vec<_>>()
        .into();
    simulation.retained.next_run_id = runs.iter().copied().max().unwrap_or(0);
    let samples = simulation.retained.runs.iter().map(run_bits).collect();
    let results = crate::io::capture_simulation_results(&simulation);
    crate::io::result_segments::keep_segments_now(results.stored_segments(), file)
        .expect("every run's segment is kept beside the project");
    let project = ProjectSnapshot::new_with_simulation_results(workspace, libraries, results);
    let json = serialize_project_file(&project).expect("the project serializes");
    drop((project, simulation));
    (json, samples)
}

fn scratch_folder(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("rspice-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn a_project_renamed_outside_rspice_opens_its_runs() {
    let root = scratch_folder("renamed-project");
    let saved = root.join("amp.rspiceproj");
    let (json, samples) = saved_and_closed(&saved, &[101]);
    std::fs::write(&saved, &json).unwrap();
    let renamed = root.join("amp-renamed.rspiceproj");
    std::fs::rename(&saved, &renamed).unwrap();
    assert!(root.join("results").join("amp").is_dir());

    let text = std::fs::read_to_string(&renamed).unwrap();
    let loaded = load_project_text(&text, Some(&renamed)).expect("the renamed project opens");
    assert_eq!(loaded.file.simulation_results_warning, None);
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("its runs restore");
    let run = &restored.retained.runs[0];
    assert_eq!(run.unread_results(), None);
    assert_eq!(run_bits(run), samples[0]);
    drop(restored);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_damaged_run_names_its_damage_and_leaves_the_others_readable() {
    let root = scratch_folder("damaged-run");
    let file = root.join("pair.rspiceproj");
    let (json, samples) = saved_and_closed(&file, &[201, 202]);
    let project: serde_json::Value = serde_json::from_str(&json).unwrap();
    let damaged_digest =
        project["simulation_results"]["runs"][1]["analyses"][0]["waveforms"][0]["y"]["segment"]
            .as_str()
            .unwrap()
            .to_owned();
    let damaged = rspice_formats::project_results::segment_folder(&file)
        .join(format!("{damaged_digest}.rsr"));
    let mut bytes = std::fs::read(&damaged).unwrap();
    bytes[..64].fill(0xa5);
    std::fs::write(&damaged, bytes).unwrap();

    let loaded = load_project_text(&json, Some(&file)).expect("the project opens");
    let warning = loaded.file.simulation_results_warning.clone().unwrap();
    let labels: Vec<_> = loaded
        .file
        .simulation_results
        .runs
        .iter()
        .map(|run| run.label.clone())
        .collect();
    assert!(warning.contains(&labels[1]), "{warning}");
    assert!(!warning.contains(&format!("{} (", labels[0])), "{warning}");
    assert!(warning.contains("is damaged"), "{warning}");
    assert!(!warning.contains("deleted or moved"), "{warning}");
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("the history opens");
    let [good, bad] = [&restored.retained.runs[0], &restored.retained.runs[1]];
    assert_eq!(good.unread_results(), None);
    assert_eq!(run_bits(good), samples[0]);
    let reason = bad
        .unread_results()
        .expect("the damaged run cannot read its samples");
    assert!(reason.contains("is damaged"), "{reason}");
    let y = &bad.analyses[0].waveforms[0].y;
    let failure = y.read_range(0..y.len()).unwrap_err().to_string();
    assert!(failure.contains("is damaged"), "{failure}");
    drop(restored);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn locate_results_reads_runs_from_the_folder_it_is_given() {
    let root = scratch_folder("locate-results");
    let file = root.join("moved.rspiceproj");
    let (json, samples) = saved_and_closed(&file, &[301]);
    let elsewhere = root.join("elsewhere");
    std::fs::rename(root.join("results"), &elsewhere).unwrap();

    let loaded = load_project_text(&json, Some(&file)).expect("the project opens");
    let warning = loaded.file.simulation_results_warning.clone().unwrap();
    assert!(warning.contains("deleted or moved"), "{warning}");
    let mut restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("the history opens");
    assert!(restored.retained.runs[0].unread_results().is_some());

    assert_eq!(
        crate::io::result_segments::locate_results(&mut restored, &root.join("nowhere")),
        0
    );
    let located =
        crate::io::result_segments::locate_results(&mut restored, &elsewhere.join("moved"));
    assert_eq!(located, 1);
    let run = &restored.retained.runs[0];
    assert_eq!(run.unread_results(), None);
    assert_eq!(run_bits(run), samples[0]);
    let waveforms = &run.analyses[0].waveforms;
    assert!(
        rspice_results::waveform::WaveformValues::ptr_eq(&waveforms[0].x, &waveforms[1].x),
        "its signals share one axis"
    );
    drop(restored);
    let _ = std::fs::remove_dir_all(&root);
}

/// The sample at `row` of seeded signal `signal`: a SplitMix64 draw in [-0.5, 0.5), so every
/// sample differs and a test regenerates it instead of holding it.
fn seeded_sample(signal: u64, row: u64) -> f64 {
    let mut z = ((signal << 40) ^ row).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 53) as f64 - 0.5
}

/// A run of four seeded signals holding `bytes` of samples, time axis included, saves beside a
/// project file, closes, and reopens with every sample equal, read back a block at a time.
fn a_seeded_run_round_trips(bytes: u64, name: &str) {
    const SIGNALS: u64 = 4;
    let rows = usize::try_from(bytes / (8 * (SIGNALS + 1))).unwrap();
    let mut libraries = crate::state::LibraryManager::with_primitives();
    let workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut simulation = SimulationState::default();
    {
        let time: Vec<f64> = (0..rows).map(|row| row as f64 * 1.0e-9).collect();
        let waveforms = (0..SIGNALS)
            .map(|signal| {
                let values = (0..rows as u64)
                    .map(|row| seeded_sample(signal, row))
                    .collect::<Vec<f64>>();
                RetainedTrace::new(format!("V(n{signal})"), time.clone(), values, "#ffffff")
            })
            .collect();
        let mut run = SimulationRun::new(1);
        run.keep_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(waveforms),
        );
        super::seal_legacy_unattributed(&mut run);
        run.mark_running().unwrap();
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .unwrap();
        assert!(run.analyses[0].waveforms.iter().all(|w| w.y.is_stored()));
        simulation.retained.runs = vec![run].into();
        simulation.retained.next_run_id = 1;
    }
    let root = scratch_folder(name);
    let file = root.join(format!("{name}.rspiceproj"));
    let results = crate::io::capture_simulation_results(&simulation);
    crate::io::result_segments::keep_segments_now(results.stored_segments(), &file)
        .expect("the run's segment is kept beside the project");
    let project = ProjectSnapshot::new_with_simulation_results(workspace, libraries, results);
    let json = serialize_project_file(&project).expect("the project serializes");
    assert!(
        json.len() < 1 << 20,
        "the project file holds {} bytes",
        json.len()
    );
    std::fs::write(&file, &json).unwrap();
    drop((project, simulation, json));

    let text = std::fs::read_to_string(&file).unwrap();
    let loaded = load_project_text(&text, Some(&file)).expect("the project opens");
    assert_eq!(loaded.file.simulation_results_warning, None);
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("its run restores");
    let analysis = &restored.retained.runs[0].analyses[0];
    assert_eq!(analysis.waveforms.len(), SIGNALS as usize);
    const BLOCK: usize = 1 << 20;
    for (signal, waveform) in analysis.waveforms.iter().enumerate() {
        assert_eq!(waveform.y.len(), rows);
        let mut first = 0;
        while first < rows {
            let last = (first + BLOCK).min(rows);
            let x = waveform.x.read_range(first..last).unwrap();
            let y = waveform.y.read_range(first..last).unwrap();
            for (offset, (x, y)) in x.iter().zip(y.iter()).enumerate() {
                let row = first + offset;
                assert_eq!(x.to_bits(), (row as f64 * 1.0e-9).to_bits(), "x at {row}");
                assert_eq!(
                    y.to_bits(),
                    seeded_sample(signal as u64, row as u64).to_bits(),
                    "{} at {row}",
                    waveform.name
                );
            }
            first = last;
        }
    }
    drop(restored);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_seeded_run_of_64_mib_saves_closes_and_reopens_with_every_sample() {
    a_seeded_run_round_trips(64 << 20, "seeded-64-mib");
}

/// Run every check this thread left for one, as the frames after an open or a save do, until
/// none runs. Started before the frame polls them: a quick check can end before the poll that
/// started it looks at it.
fn run_segment_checks(simulation: &mut SimulationState) {
    let mut checks = Vec::new();
    crate::io::result_segments::start_segment_checks(&mut checks);
    assert!(!checks.is_empty(), "a check was left to run");
    for check in &checks {
        assert!(check.wait(std::time::Duration::from_secs(60)));
    }
    assert!(!crate::io::result_segments::poll_segment_checks(
        simulation,
        &mut checks
    ));
}

/// Opening a project reads none of its kept segments whole first: each one not checked yet is
/// checked by a job of its own after the project opens. Meanwhile its run says it is being
/// checked, not that its results are missing or damaged, and the open names no run as
/// unreadable. The check's end reads the run as the segment, or as what is wrong with it.
#[test]
fn a_project_opens_before_its_results_are_checked_and_reads_them_once_they_are() {
    let root = scratch_folder("checked-after-open");
    let saved = root.join("pair.rspiceproj");
    let (json, samples) = saved_and_closed(&saved, &[401, 402]);
    // Renamed, so no file of the folder is one this process has checked.
    let file = root.join("moved.rspiceproj");
    std::fs::rename(
        root.join("results").join("pair"),
        root.join("results").join("moved"),
    )
    .unwrap();
    let project: serde_json::Value = serde_json::from_str(&json).unwrap();
    let digest_of = |run: usize| {
        project["simulation_results"]["runs"][run]["analyses"][0]["waveforms"][0]["y"]["segment"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    let kept = |run: usize| {
        rspice_formats::project_results::segment_folder(&file)
            .join(format!("{}.rsr", digest_of(run)))
    };
    let mut bytes = std::fs::read(kept(1)).unwrap();
    bytes[..64].fill(0xa5);
    std::fs::write(kept(1), bytes).unwrap();

    let loaded =
        rspice_results::store_reader::opening_unchecked(|| load_project_text(&json, Some(&file)))
            .expect("the project opens");
    assert_eq!(loaded.file.simulation_results_warning, None);
    let good_digest = rspice_results::store_reader::parse_digest(&digest_of(0)).unwrap();
    assert!(
        !rspice_results::store_reader::segment_verified(&kept(0), &good_digest),
        "the open read no segment whole"
    );
    let mut restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("the history opens");
    for run in restored.retained.runs.iter() {
        assert_eq!(run.unread_results(), None);
        assert!(
            run.checking_results().is_some(),
            "{} is being checked",
            run.label
        );
    }

    run_segment_checks(&mut restored);
    let [good, bad] = [&restored.retained.runs[0], &restored.retained.runs[1]];
    assert_eq!(good.checking_results(), None);
    assert_eq!(good.unread_results(), None);
    assert_eq!(run_bits(good), samples[0]);
    let waveforms = &good.analyses[0].waveforms;
    assert!(
        rspice_results::waveform::WaveformValues::ptr_eq(&waveforms[0].x, &waveforms[1].x),
        "its signals share one axis"
    );
    assert_eq!(bad.checking_results(), None);
    let reason = bad.unread_results().expect("the damaged run cannot read");
    assert!(reason.contains("is damaged"), "{reason}");
    drop(restored);
    let _ = std::fs::remove_dir_all(&root);
}

/// On the window's thread, which never waits for a check, a project opens and its runs read as
/// being checked; once the checks end the intact run reads every sample, the damaged one reads
/// as damaged, and no second check is left for the damaged file.
#[test]
fn on_the_window_thread_a_failed_check_reads_as_damage_and_is_left_for_no_other() {
    rspice_results::store_reader::never_wait_for_checks_here();
    let root = scratch_folder("window-thread-checks");
    let saved = root.join("pair.rspiceproj");
    let (json, samples) = saved_and_closed(&saved, &[421, 422]);
    // Renamed, so no file of the folder is one this process has checked.
    let file = root.join("moved.rspiceproj");
    std::fs::rename(
        root.join("results").join("pair"),
        root.join("results").join("moved"),
    )
    .unwrap();
    let project: serde_json::Value = serde_json::from_str(&json).unwrap();
    let damaged = rspice_formats::project_results::segment_folder(&file).join(format!(
        "{}.rsr",
        project["simulation_results"]["runs"][1]["analyses"][0]["waveforms"][0]["y"]["segment"]
            .as_str()
            .unwrap()
    ));
    let mut bytes = std::fs::read(&damaged).unwrap();
    let middle = bytes.len() / 2;
    bytes[middle..middle + 64].fill(0xa5);
    std::fs::write(&damaged, bytes).unwrap();

    let loaded =
        rspice_results::store_reader::opening_unchecked(|| load_project_text(&json, Some(&file)))
            .expect("the project opens");
    let mut restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("the history opens");
    assert!(
        restored
            .retained
            .runs
            .iter()
            .all(|run| run.checking_results().is_some())
    );
    run_segment_checks(&mut restored);
    let [good, bad] = [&restored.retained.runs[0], &restored.retained.runs[1]];
    assert_eq!(good.checking_results(), None);
    assert_eq!(run_bits(good), samples[0]);
    assert_eq!(bad.checking_results(), None);
    let reason = bad.unread_results().expect("the damaged run cannot read");
    assert!(reason.contains("is damaged"), "{reason}");
    assert_eq!(
        rspice_results::store_reader::take_unchecked_segments(),
        Vec::new(),
        "the damaged file is not left for another check"
    );
    drop(restored);
    let _ = std::fs::remove_dir_all(&root);
}

/// A run whose kept segment still waits for its first check is never read as missing. Opened
/// again while the job that checks it has not started, as a recovery reads a project again, it
/// reads every sample: the read runs the check itself. The run the open restored reads every
/// sample too, and the job, let go, reads the file no more.
#[test]
fn a_run_read_while_its_check_waits_joins_the_check() {
    use crate::workbench::results_job::hold_jobs_for_tests;
    let root = scratch_folder("read-joins-its-check");
    let saved = root.join("pair.rspiceproj");
    let (json, samples) = saved_and_closed(&saved, &[411]);
    // Renamed, so no file of the folder is one this process has checked.
    let file = root.join("moved.rspiceproj");
    std::fs::rename(
        root.join("results").join("pair"),
        root.join("results").join("moved"),
    )
    .unwrap();
    let loaded =
        rspice_results::store_reader::opening_unchecked(|| load_project_text(&json, Some(&file)))
            .expect("the project opens");
    let opened = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("the history opens");
    assert!(opened.retained.runs[0].checking_results().is_some());
    hold_jobs_for_tests(true);
    let mut checks = Vec::new();
    crate::io::result_segments::start_segment_checks(&mut checks);
    hold_jobs_for_tests(false);
    assert_eq!(checks.len(), 1, "the open left one check");

    let again = load_project_text(&json, Some(&file)).expect("the project reads again");
    assert_eq!(again.file.simulation_results_warning, None);
    let recovered = crate::io::simulation_state_from_results(again.file.simulation_results)
        .expect("the history reads again");
    let run = &recovered.retained.runs[0];
    assert_eq!(run.unread_results(), None, "read again, the run is there");
    assert_eq!(run_bits(run), samples[0]);
    assert!(!checks[0].is_finished(), "the job never started");
    assert_eq!(run_bits(&opened.retained.runs[0]), samples[0]);
    assert_eq!(opened.retained.runs[0].checking_results(), None);

    checks[0].release();
    let mut simulation = opened;
    assert!(checks[0].wait(std::time::Duration::from_secs(60)));
    assert!(!crate::io::result_segments::poll_segment_checks(
        &mut simulation,
        &mut checks
    ));
    assert_eq!(simulation.retained.runs[0].unread_results(), None);
    assert_eq!(run_bits(&simulation.retained.runs[0]), samples[0]);
    drop((simulation, recovered));
    let _ = std::fs::remove_dir_all(&root);
}

/// A save into a folder that already holds a file under a run's segment name, not checked yet,
/// goes on without reading it: the file is checked by a job of its own, which puts the run's own
/// segment in its place when the file is not the segment.
#[test]
fn a_save_goes_on_while_what_its_folder_holds_is_checked() {
    let root = scratch_folder("save-checks-its-folder");
    let file = root.join("amp.rspiceproj");
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![large_ac_run(501, 3_000, 2)].into();
    let results = crate::io::capture_simulation_results(&simulation);
    let segments = results.stored_segments();
    let source = segments[0].source.clone().expect("the run's own segment");
    let folder = rspice_formats::project_results::segment_folder(&file);
    std::fs::create_dir_all(&folder).unwrap();
    let target = folder.join(rspice_results::store_reader::kept_segment_name(
        &segments[0].digest,
    ));
    // The same segment, damaged between its header and its end.
    let mut bytes = std::fs::read(&source).unwrap();
    let middle = bytes.len() / 4;
    bytes[middle] ^= 0xff;
    std::fs::write(&target, bytes).unwrap();

    let plan = crate::io::result_segments::segments_to_keep(segments.clone(), &file);
    assert!(plan.is_empty(), "the save waits on no check: {plan:?}");
    assert!(rspice_results::store_reader::awaiting_check(&target));
    run_segment_checks(&mut simulation);
    assert!(!rspice_results::store_reader::awaiting_check(&target));
    assert!(
        std::fs::read(&target).unwrap() == std::fs::read(&source).unwrap(),
        "the check put the run's segment in the damaged file's place"
    );
    drop(simulation);
    let _ = std::fs::remove_dir_all(&root);
}

/// Pack with results writes the project and its results folder as one archive, a job. Moved
/// alone to another folder and opened there, also a job, it is a project whose every run reads
/// its samples from beside it.
#[test]
fn a_packed_project_opens_from_a_clean_path_with_every_run_readable() {
    use crate::io::project_pack::{ProjectPackJob, start_pack_open, start_project_pack};
    use crate::workbench::results_job::JobOutcome;
    let patience = std::time::Duration::from_secs(60);
    let root = scratch_folder("packed");
    let file = root.join("amp.rspiceproj");
    let (json, samples) = saved_and_closed(&file, &[601, 602]);
    std::fs::write(&file, &json).unwrap();
    let archive = root.join("amp.rspicepack");
    let pack = start_project_pack(file.clone(), archive.clone());
    assert!(pack.wait(patience));
    let ProjectPackJob::Packing { job, .. } = pack else {
        panic!("a pack is written");
    };
    assert_eq!(job.take_outcome(), Some(JobOutcome::Done(())));

    let clean = scratch_folder("packed-elsewhere");
    let shared = clean.join("shared.rspicepack");
    std::fs::rename(&archive, &shared).unwrap();
    std::fs::remove_dir_all(&root).unwrap();
    let open = start_pack_open(shared);
    assert!(open.wait(patience));
    let ProjectPackJob::Opening { job, .. } = open else {
        panic!("a pack is opened");
    };
    let Some(JobOutcome::Done(project)) = job.take_outcome() else {
        panic!("the pack opens");
    };
    assert_eq!(project, clean.join("shared").join("amp.rspiceproj"));
    let loaded = rspice_results::store_reader::opening_unchecked(|| load_project_file(&project))
        .expect("the packed project opens");
    assert_eq!(loaded.file.simulation_results_warning, None);
    let mut restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("its runs restore");
    run_segment_checks(&mut restored);
    assert_eq!(restored.retained.runs.len(), samples.len());
    for (run, samples) in restored.retained.runs.iter().zip(&samples) {
        assert_eq!(run.unread_results(), None, "{}", run.label);
        assert_eq!(&run_bits(run), samples, "{}", run.label);
    }
    drop(restored);
    let _ = std::fs::remove_dir_all(&clean);
}
