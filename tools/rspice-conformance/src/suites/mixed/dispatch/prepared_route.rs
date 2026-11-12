//! The application's prepared manual-deck route runs a mixed corpus deck and
//! gets the runner's numbers.
//!
//! The Netlist & Script page runs a deck through
//! `rspice_simulation::execution::preparation::build_prepared_manual_deck`;
//! the runner runs the same file through the headless route. Both seal the
//! deck's HDL through `seal_deck_hdl_sources`, so the same deck gives the
//! same operating point on either.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::time::Duration;

use rspice_simulation::analysis_preparation::AnalysisInputs;
use rspice_simulation::execution::PreparedRunAuthorization;
use rspice_simulation::execution::preparation::{
    ManualSourceInputs, RunPreparationInputs, build_prepared_manual_deck,
};
use rspice_simulation::netlist_gen::{NetlistSourceData, NetlistSourceHost, TableRoute};
use rspice_simulation::project_technology::ProjectTechnologyInputs;

use super::{CaseRequest, headless_case, observation};
use crate::suites::mixed::case::Observation;
use crate::suites::mixed::design::Design;

/// A manual deck reads no schematic source tables; asking for one is a bug.
struct NoSchematicFiles;

impl NetlistSourceHost for NoSchematicFiles {
    fn route_retaining(
        &self,
        _: &str,
        _: Option<&Path>,
        _: Option<&rspice_design::stimulus_library::definition::RetainedPwlFile>,
    ) -> (TableRoute, Option<String>) {
        panic!("a manual deck asked for a schematic source table")
    }
    fn is_file(&self, _: &str) -> std::io::Result<bool> {
        panic!("a manual deck asked for schematic file metadata")
    }
    fn engine_refusal(&self, _: &str) -> Option<String> {
        panic!("a manual deck asked for a schematic source file")
    }
    fn named_file_matches(
        &self,
        _: &str,
        _: &rspice_design::stimulus_library::definition::RetainedPwlFile,
    ) -> Option<bool> {
        panic!("a manual deck asked for a retained-file comparison")
    }
}

fn corpus_design(slug: &str) -> Design {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus")
        .join(slug);
    Design::load(&dir).unwrap_or_else(|error| panic!("{slug} loads: {error}"))
}

/// The deck at `origin` prepared as the Netlist & Script page prepares a
/// saved deck, with no project around it, and run task by task.
fn prepared_manual_run(source: &str, origin: &Path) -> Vec<Observation> {
    let inputs: RunPreparationInputs<'_, (), ()> = RunPreparationInputs {
        analysis: AnalysisInputs {
            sim_setup: &Default::default(),
            schematic: &Default::default(),
            selected_components: &Default::default(),
            selection_occurrence: None,
            runs: &[],
            active_run: None,
            project_revision: Default::default(),
            plan_payloads: &[],
        },
        technology: ProjectTechnologyInputs {
            project: &Default::default(),
            sim_setup: &Default::default(),
            models: &Default::default(),
            resolutions: &Default::default(),
            registry: &Default::default(),
            layouts: &Default::default(),
        },
        libraries: &Default::default(),
        project_sources: &Default::default(),
        design_management: &Default::default(),
        configuration_sets: &Default::default(),
        source_data: NetlistSourceData::new(&NoSchematicFiles),
        schematic_file: None,
        schematic_topology_revision: 0,
        imported_checkpoints: &Default::default(),
        imported_transient_checkpoints: &Default::default(),
        display_waveform_cache_samples: 0,
        max_parallel_workers: None,
        library_folders: &[],
        compilation: None,
        design_checks: Default::default(),
    };
    let snapshot = build_prepared_manual_deck(
        &inputs,
        ManualSourceInputs {
            source,
            descriptor: None,
            generated_artifact: None,
            owned: false,
            origin: Some(origin),
        },
    )
    .unwrap_or_else(|error| panic!("{} prepares: {error}", origin.display()));
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .expect("the snapshot is authorized");
    let mut pending: VecDeque<_> = dispatch.into_tasks();
    let mut artifacts = HashMap::new();
    let mut runner = rspice_simulation::runner::SimulationRunner::new();
    let mut observations = Vec::new();
    while let Some(task) = pending.pop_front() {
        let instance = task.instance_id();
        let resolved = task
            .resolve_dependency_artifacts(&artifacts)
            .expect("dependency artifacts resolve");
        let producer = resolved.artifact_producer().expect("an artifact producer");
        runner
            .start_prepared(resolved, false)
            .unwrap_or_else(|error| panic!("the task starts: {error}"));
        let outcome = loop {
            if let Some(outcome) = runner.poll_result() {
                break outcome;
            }
            runner.drain_engine_log();
            std::thread::sleep(Duration::from_millis(5));
        };
        let simulated = outcome.unwrap_or_else(|error| panic!("the task solves: {error}"));
        if let Some(artifact) = producer
            .capture(&simulated, &pending)
            .expect("the artifact is captured")
        {
            artifacts.insert(instance, artifact);
        }
        observations.push(observation(&simulated, ""));
    }
    observations
}

/// Every deck runs through the runner's headless route and through the
/// application's prepared manual-deck route, and every node voltage and
/// branch current agrees.
#[test]
fn mixed_op_decks_give_the_runners_numbers_on_the_prepared_manual_route() {
    for (slug, variant) in [
        ("c04-sar8", "full"),
        ("c12-ldo-tb", "full"),
        ("c12-ldo-tb", "top"),
    ] {
        let design = corpus_design(slug);
        let origin: PathBuf = design.dir.join("deck").join(format!("{variant}.op.cir"));
        let source = std::fs::read_to_string(&origin).expect("read the deck");
        let request = CaseRequest {
            design: &design,
            variant,
            analysis: "op",
            o5: false,
            budget: Duration::from_secs(300),
        };
        let runner = headless_case(&request, &origin, &source);
        assert_eq!(
            runner.outcome, "completed",
            "{slug} {variant}: {:?}",
            runner.error
        );
        let prepared = prepared_manual_run(&source, &origin);
        assert_eq!(runner.results.len(), 1, "{slug} {variant}");
        assert_eq!(prepared.len(), 1, "{slug} {variant}");
        let (expected, actual) = (&runner.results[0].real, &prepared[0].real);
        assert!(
            expected.keys().any(|name| name.starts_with("v(")),
            "{slug} {variant}: no node voltage in {:?}",
            expected.keys()
        );
        assert_eq!(
            expected.keys().collect::<Vec<_>>(),
            actual.keys().collect::<Vec<_>>(),
            "{slug} {variant}"
        );
        for (name, values) in expected {
            let seen = &actual[name];
            assert_eq!(values.len(), seen.len(), "{slug} {variant} {name}");
            for (want, got) in values.iter().zip(seen) {
                assert!(
                    (want - got).abs() <= 1e-12 + 1e-9 * want.abs(),
                    "{slug} {variant} {name}: runner {want:e}, prepared route {got:e}"
                );
            }
        }
    }
}
