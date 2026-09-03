//! Staging and publication of one request's result artifacts.
//!
//! Everything a card produces -- its own shared result document and the second
//! documents it publishes beside it -- is staged in memory and written only
//! after every directive has succeeded, so a failed run leaves `results/` empty
//! and the response is the single source of truth about declared outputs. The
//! artifact count and the byte budget are checked before a single solve and
//! again as each document is encoded.

// This module was split out of `execute.rs` and still works against
// the executor's own wire types, failures and imports, so it takes the
// parent's imports rather than restating them.
use super::*;

/// Everything one executed analysis contributes to the response.
pub(super) struct AnalysisOutcome {
    pub(super) measurements: Vec<Measurement>,
    pub(super) artifacts: Vec<PendingArtifact>,
    pub(super) schema_signature: ResultSchemaSignature,
}

/// Run one materialized analysis and stage its typed artifacts.
#[allow(clippy::too_many_arguments)]
// The identity a result document carries — analysis, coordinate, topology,
// namespaces — is exactly this many independent inputs, and threading them
// through a struct would only move the same list one call earlier.
pub(super) fn execute_analysis(
    engine: &Engine,
    netlist: &Netlist,
    plan: &DeckPlan,
    analysis: &MaterializedAnalysis,
    peers: &[MaterializedAnalysis],
    coordinate: &RunCoordinate,
    topology: TopologyFingerprint,
    has_axes: bool,
    abort: &dyn AbortSignal,
    byte_limit: u64,
) -> Result<AnalysisOutcome, DirectiveFailure> {
    let projection = run_directive(
        engine,
        netlist,
        plan,
        analysis,
        peers,
        coordinate.stable_id(),
        abort,
    )?;
    let namespaces = ResultNamespaces {
        output: analysis.output_namespace().components().join("/"),
        checkpoint: analysis.checkpoint_namespace().components().join("/"),
    };
    let coordinate_identity = has_axes.then(|| ResultCoordinate::from_run_coordinate(coordinate));
    let identify = |builder: rspice_core::execution::AnalysisResultDocumentBuilder| {
        let mut builder = builder
            .topology_fingerprint(topology)
            .namespaces(namespaces.clone());
        if let Some(coordinate) = coordinate_identity.clone() {
            builder = builder.coordinate(coordinate);
        }
        builder
    };

    let document = identify(projection.builder)
        .build_with_abort(abort)
        .map_err(crate::failure::map_result_document_error)?;
    let schema_signature = ResultSchemaSignature::from_document(&document);
    let measurements = measurements_from_document(&document, abort)?;

    let file_stem = artifact_stem(analysis, coordinate, has_axes);
    let artifact = encode_result_artifact(
        format!("{file_stem}.result.json"),
        &document,
        abort,
        byte_limit,
    )?;
    let mut artifacts = vec![artifact];

    // A card's second results — port noise beside `.SP`, one spectrum per
    // authored `.FFT` card and one per authored `.FOUR` operand beside their
    // transient — are complete shared documents with their own identities. They
    // are staged in this analysis's own artifact list, so the whole set is
    // published in one transaction or not at all, and each takes the parent's
    // artifact stem plus its own namespace component so no two can collide.
    for child in projection.children {
        let document = identify(child.builder)
            .build_with_abort(abort)
            .map_err(crate::failure::map_result_document_error)?;
        let remaining = remaining_bytes(&artifacts, byte_limit)?;
        artifacts.push(encode_result_artifact(
            format!("{file_stem}.{}.result.json", child.namespace),
            &document,
            abort,
            remaining,
        )?);
    }

    // Each artifact was individually bounded; this proves the set together
    // still fits the budget this analysis was given.
    let _remaining = remaining_bytes(&artifacts, byte_limit)?;

    Ok(AnalysisOutcome {
        measurements,
        artifacts,
        schema_signature,
    })
}

/// Bytes of the analysis budget still unused after the staged artifacts.
fn remaining_bytes(
    artifacts: &[PendingArtifact],
    byte_limit: u64,
) -> Result<u64, DirectiveFailure> {
    artifacts
        .iter()
        .try_fold(0u64, |total, artifact| {
            total.checked_add(artifact.content.len() as u64)
        })
        .and_then(|used| byte_limit.checked_sub(used))
        .ok_or(DirectiveFailure::ResultSetBytes)
}

/// Canonical artifact stem for one analysis at one coordinate.
fn artifact_stem(
    analysis: &MaterializedAnalysis,
    coordinate: &RunCoordinate,
    has_axes: bool,
) -> String {
    let analysis_component = analysis.output_namespace().analysis_component();
    if has_axes {
        format!("{}__{analysis_component}", coordinate.stable_tag())
    } else {
        analysis_component
    }
}

pub(super) fn artifact_reference(artifact: &PendingArtifact) -> ResultDocumentReference {
    ResultDocumentReference {
        path: format!("results/{}", artifact.file_name),
        content_type: artifact.content_type.clone(),
        schema: rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_SCHEMA.to_owned(),
        schema_version: rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_VERSION,
        result_kind: artifact.result_kind.clone(),
    }
}

/// How many result artifacts one directive of `kind` publishes.
///
/// A directive publishes its own document plus every second document the card
/// produces: one `.FFT` spectrum per authored `.FFT` card and one Fourier
/// document per planned `.FOUR` operand bound to it, and the `.SP` card's
/// port-noise document when it authored `DONOISE`. Counting them here is what
/// lets the artifact-count limit be checked before a single solve.
pub(super) fn artifacts_per_directive(
    netlist: &Netlist,
    plan: &DeckPlan,
    kind: PlannedAnalysisKind,
) -> Option<usize> {
    use rspice_core::netlist::AnalysisCommand;

    let mut count = 1usize;
    if kind == PlannedAnalysisKind::Tran {
        // The plan binds each post-process to one transient, so the worst case
        // for any single transient is every planned post-process of the deck.
        count = count.checked_add(plan.post_process_analyses().len())?;
    }
    if kind == PlannedAnalysisKind::Sp
        && netlist
            .analyses
            .iter()
            .any(|command| matches!(command, AnalysisCommand::Sp { do_noise: true, .. }))
    {
        count = count.checked_add(1)?;
    }
    Some(count)
}

pub(super) fn preflight_planned_artifact_count(
    coordinate_count: usize,
    directive_count: usize,
    per_directive: usize,
) -> Result<(), DirectiveFailure> {
    let count = coordinate_count
        .checked_mul(directive_count)
        .and_then(|count| count.checked_mul(per_directive))
        .ok_or(DirectiveFailure::ResultArtifactLimit)?;
    validate_artifact_budget(count, std::iter::empty())
}

pub(super) fn validate_artifact_budget(
    artifact_count: usize,
    byte_lengths: impl IntoIterator<Item = u64>,
) -> Result<(), DirectiveFailure> {
    if artifact_count > MAX_ENGINE_RESULT_ARTIFACTS {
        return Err(DirectiveFailure::ResultArtifactLimit);
    }
    let mut total = 0u64;
    for length in byte_lengths {
        if !(1..=MAX_ENGINE_ARTIFACT_BYTES).contains(&length) {
            return Err(DirectiveFailure::ResultArtifactBytes);
        }
        total = total
            .checked_add(length)
            .ok_or(DirectiveFailure::ResultSetBytes)?;
        if total > MAX_ENGINE_RETAINED_RESULT_BYTES {
            return Err(DirectiveFailure::ResultSetBytes);
        }
    }
    Ok(())
}

pub(super) fn validate_pending_artifact_budget(
    artifacts: &[PendingArtifact],
) -> Result<(), DirectiveFailure> {
    validate_artifact_budget(
        artifacts.len(),
        artifacts
            .iter()
            .map(|artifact| artifact.content.len() as u64),
    )
}

/// Stages the complete response-gated artifact set, atomically replaces each
/// member, and restores every predecessor on a returned commit failure. The
/// caller emits the success manifest only after this function completes, so
/// an interrupted process never advertises a partial set.
pub fn write_artifacts(results_dir: &Path, artifacts: &[PendingArtifact]) -> Result<(), String> {
    validate_pending_artifact_budget(artifacts)
        .map_err(|_| "result artifact set exceeds the protocol budget".to_owned())?;
    let mut names = HashSet::new();
    for artifact in artifacts {
        let result_path = format!("results/{}", artifact.file_name);
        if !valid_result_path(&result_path) {
            return Err("result artifact set contains an invalid destination name".to_owned());
        }
        if !names.insert(artifact.file_name.to_ascii_lowercase()) {
            return Err("result artifact set contains duplicate destinations".to_owned());
        }
    }
    let mut transaction = AtomicArtifactSet::new();
    for artifact in artifacts {
        let destination = results_dir.join(&artifact.file_name);
        transaction
            .stage::<io::Error, _>(&destination, |writer| {
                writer.write_all(artifact.content.as_bytes())
            })
            .map_err(|error| error.to_string())?;
    }
    transaction.commit().map_err(|error| error.to_string())
}

#[cfg(test)]
fn publish_artifact<E>(
    destination: &Path,
    write: impl FnOnce(&mut dyn std::io::Write) -> Result<(), E>,
) -> Result<(), rspice_output::AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
{
    write_atomic(destination, write)
}

#[cfg(test)]
mod artifact_publication_tests {
    use super::*;
    use rspice_output::{AtomicArtifactError, stale_artifacts};
    use std::path::PathBuf;
    use std::sync::atomic::AtomicU64;

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let serial = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "rspice-engine-adapter-{tag}-{}-{serial}",
                std::process::id()
            ));
            std::fs::create_dir(&path).expect("create adapter publication test directory");
            Self(path)
        }

        fn destination(&self) -> PathBuf {
            self.0.join("result.json")
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    pub(super) fn artifact(file_name: &str, content: &str) -> PendingArtifact {
        PendingArtifact {
            file_name: file_name.to_owned(),
            content_type: "application/json".to_owned(),
            result_kind: "op".to_owned(),
            content: content.to_owned(),
        }
    }

    pub(super) fn seed(destination: &Path, preexisting: bool) {
        if preexisting {
            std::fs::write(destination, b"old complete artifact")
                .expect("seed existing adapter artifact");
        }
    }

    fn assert_old_or_absent(destination: &Path, preexisting: bool) {
        if preexisting {
            assert_eq!(
                std::fs::read(destination).expect("read preserved adapter artifact"),
                b"old complete artifact"
            );
        } else {
            assert!(
                !destination.exists(),
                "failed publication created a destination"
            );
        }
        assert!(
            stale_artifacts(destination)
                .expect("inspect adapter staging artifacts")
                .is_empty(),
            "failed publication left a staging artifact"
        );
    }

    #[test]
    fn artifact_write_failure_preserves_existing_or_absent_destination() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("failure");
            let destination = directory.destination();
            seed(&destination, preexisting);

            let error = publish_artifact(&destination, |writer| -> io::Result<()> {
                writer.write_all(b"partial replacement")?;
                Err(io::Error::other("injected adapter serialization failure"))
            })
            .expect_err("injected artifact write must fail");

            assert!(matches!(error, AtomicArtifactError::Write(_)));
            assert_old_or_absent(&destination, preexisting);
        }
    }

    #[test]
    fn artifact_success_replaces_existing_or_absent_destination() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("success");
            let destination = directory.destination();
            seed(&destination, preexisting);

            write_artifacts(
                &directory.0,
                &[artifact("result.json", "{\"complete\":true}\n")],
            )
            .expect("publish adapter artifact");

            assert_eq!(
                std::fs::read(&destination).expect("read published adapter artifact"),
                b"{\"complete\":true}\n"
            );
            assert!(
                stale_artifacts(&destination)
                    .expect("inspect adapter staging artifacts")
                    .is_empty(),
                "successful publication left a staging artifact"
            );
        }
    }

    #[test]
    fn result_artifact_count_and_byte_budgets_accept_only_the_exact_boundaries() {
        assert!(
            validate_artifact_budget(
                MAX_ENGINE_RESULT_ARTIFACTS,
                [MAX_ENGINE_RETAINED_RESULT_BYTES]
            )
            .is_ok()
        );
        assert!(matches!(
            validate_artifact_budget(1, [MAX_ENGINE_RETAINED_RESULT_BYTES + 1]),
            Err(DirectiveFailure::ResultSetBytes)
        ));
        assert!(matches!(
            validate_artifact_budget(2, [MAX_ENGINE_RETAINED_RESULT_BYTES, 1]),
            Err(DirectiveFailure::ResultSetBytes)
        ));
        assert!(matches!(
            validate_artifact_budget(MAX_ENGINE_RESULT_ARTIFACTS + 1, [0]),
            Err(DirectiveFailure::ResultArtifactLimit)
        ));
        assert!(matches!(
            validate_artifact_budget(1, [MAX_ENGINE_ARTIFACT_BYTES + 1]),
            Err(DirectiveFailure::ResultArtifactBytes)
        ));
        assert!(matches!(
            validate_artifact_budget(1, [0]),
            Err(DirectiveFailure::ResultArtifactBytes)
        ));
    }

    #[test]
    fn transient_default_tmax_uses_the_shared_core_contract() {
        assert!(
            matches!(rspice_core::execution::resolve_transient_maximum_step(1.0e-6, 1.0e-3, None, None), Ok(value) if value == 1.0e-6),
            "TSTEP is the default ceiling when it is smaller"
        );
        assert!(
            matches!(rspice_core::execution::resolve_transient_maximum_step(10.0e-3, 1.0, Some(0.9), None), Ok(value) if (value - 2.0e-3).abs() < 1.0e-15),
            "(TSTOP-TSTART)/50 is the default ceiling when it is smaller"
        );
        assert!(
            matches!(rspice_core::execution::resolve_transient_maximum_step(1.0e-6, 1.0e-3, None, Some(7.0e-6)), Ok(value) if value == 7.0e-6),
            "an explicit valid TMAX overrides both defaults"
        );
        assert!(
            rspice_core::execution::resolve_transient_maximum_step(1.0e-6, 1.0e-3, None, Some(0.0))
                .is_err()
        );
        assert!(
            rspice_core::execution::resolve_transient_maximum_step(1.0e-6, 1.0, Some(1.0), None)
                .is_err()
        );
    }

    #[test]
    fn result_artifact_destinations_are_single_component_and_case_unique() {
        let directory = TestDirectory::new("destination-validation");
        for invalid in [
            "../escape.json",
            "sub/escape.json",
            "sub\\escape.json",
            "C:escape",
        ] {
            let error = write_artifacts(&directory.0, &[artifact(invalid, "complete")])
                .expect_err("invalid artifact destination must fail closed");
            assert!(error.contains("invalid destination name"), "{error}");
        }

        let duplicate = [
            artifact("Result.json", "first"),
            artifact("result.json", "second"),
        ];
        let error = write_artifacts(&directory.0, &duplicate)
            .expect_err("case-folded duplicate destinations must fail closed");
        assert!(error.contains("duplicate destinations"), "{error}");
        assert!(
            std::fs::read_dir(&directory.0)
                .expect("read destination-validation directory")
                .next()
                .is_none()
        );
    }

    #[test]
    fn later_commit_failure_rolls_back_absent_and_preexisting_predecessors() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("set-rollback");
            let first = directory.0.join("first.json");
            seed(&first, preexisting);
            let invalid_second = directory.0.join("second.json");
            std::fs::create_dir(&invalid_second).expect("create commit-failing destination");
            let artifacts = [
                artifact("first.json", "new first"),
                artifact("second.json", "new second"),
            ];

            let error = write_artifacts(&directory.0, &artifacts)
                .expect_err("second destination must fail commit");
            assert!(error.contains("predecessor set was restored"), "{error}");
            assert_old_or_absent(&first, preexisting);
            assert!(invalid_second.is_dir());
            let names = std::fs::read_dir(&directory.0)
                .expect("read transaction directory")
                .map(|entry| entry.expect("read transaction entry").file_name())
                .collect::<Vec<_>>();
            assert!(
                names.iter().all(|name| !name
                    .to_string_lossy()
                    .contains(rspice_output::PREDECESSOR_MARKER)),
                "rollback backup remained: {names:?}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn non_regular_predecessors_fail_before_any_destination_is_committed() {
        use std::os::unix::net::UnixListener;

        let directory = TestDirectory::new("non-regular-predecessor");
        let first = directory.0.join("first.json");
        std::fs::write(&first, b"old first").expect("seed first predecessor");
        let socket = directory.0.join("second.json");
        let _listener = UnixListener::bind(&socket).expect("bind predecessor socket");
        let artifacts = [
            artifact("first.json", "new first"),
            artifact("second.json", "new second"),
        ];

        let error = write_artifacts(&directory.0, &artifacts)
            .expect_err("a socket predecessor must fail closed");
        assert!(
            error.contains("neither a regular file nor a directory"),
            "{error}"
        );
        assert_eq!(
            std::fs::read(&first).expect("read untouched first predecessor"),
            b"old first"
        );
        assert!(socket.exists());
    }

    #[test]
    fn cancelled_axis_planning_returns_a_cancellation_without_artifacts() {
        let netlist = Netlist::parse_validated(
            "cancelled axis plan\n\
             .param r=1k\n\
             V1 in 0 1\n\
             R1 in 0 {r}\n\
             .step param r list 1k 2k\n\
             .op\n\
             .end\n",
        )
        .expect("axis fixture parses");
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            ..SimulationConfig::default()
        });
        let execution = execute_planned_netlist(
            PlannedAnalysisKind::Op,
            "operating_point",
            &netlist,
            "test",
            &engine,
            &rspice_core::abort_signal::ImmediateAbort,
        );
        assert!(execution.artifacts.is_empty());
        match execution.response {
            EngineResponse::Failed { failure_code, .. } => {
                assert_eq!(failure_code, CANCELLED_FAILURE_CODE)
            }
            EngineResponse::Succeeded { .. } => panic!("cancelled planning succeeded"),
        }
    }
}
