//! Refused allocations and failed publications are typed, not fatal.
//!
//! Two fail-closed contracts meet here.
//!
//! *Allocation.* The planner and the resource layer size every large buffer
//! with `try_reserve`, so a request the allocator cannot satisfy has to come
//! back as a typed `Allocation`/`ResourceLimit` error. The alternative — the
//! infallible `Vec` path — aborts the process, which for an embedded engine
//! means taking the host application down with it. These tests ask for
//! allocations no host can serve and prove the refusal is a value.
//!
//! *Publication.* A checkpoint save is a transaction: it stages, flushes,
//! synchronizes, and only then replaces the destination. `rspice_output`'s
//! qualification seam makes each of those phases fail on demand, so the
//! "old checkpoint or no checkpoint" claim is checked at every one of them
//! rather than only at the phases an ordinary test can reach.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::{
    Engine, OutputCommitPhase, SimulationConfig, SimulationError, TransientCheckpoint,
    TransientCheckpointEncoding,
};
use rspice_core::execution::{
    AxisKind, DeckPlan, DeckPlanError, RunAxis, RunAxisValue, numeric_run_coordinate_id,
};
use rspice_core::netlist::Netlist;
use rspice_core::netlist::multi_run::try_expand_multi_run_with_limits_and_abort;
use rspice_core::{ResourceKind, ResourceLimits};
use rspice_output::fault::{ArmedFaults, ArtifactFault, ArtifactFaultPoint};

//=============================================================================
// Allocation refusals
//=============================================================================

/// An axis product far larger than any host can back with memory, but small
/// enough that the accounting arithmetic itself does not overflow `usize`.
///
/// The planner must therefore reach a real `try_reserve` and be refused by the
/// allocator rather than by a checked-multiplication guard.
fn enormous_plan() -> DeckPlan {
    let axes = (0..3)
        .map(|index| {
            RunAxis::new(
                AxisKind::Step,
                format!("param:p{index}"),
                (0..4_000)
                    .map(|value| RunAxisValue::Numeric(f64::from(value) + 1.0))
                    .collect(),
            )
            .expect("a dense numeric axis is valid")
        })
        .collect();
    DeckPlan::new(axes, Vec::new()).expect("three ordered STEP axes are a valid plan")
}

#[test]
fn a_coordinate_set_no_host_can_hold_is_refused_as_a_typed_allocation_failure() {
    let plan = enormous_plan();
    // 4_000^3 = 6.4e10 coordinates. With unlimited policy limits the only
    // thing that can stop it is the allocator.
    let error = plan
        .coordinates_with_abort(&ResourceLimits::unlimited(), &NoAbort)
        .expect_err("no allocator can back 6.4e10 run coordinates");
    assert!(
        matches!(error, DeckPlanError::Allocation { .. }),
        "a refused allocation must be a typed value, not an abort: {error:?}"
    );
}

#[test]
fn the_same_coordinate_set_is_refused_by_policy_before_the_allocator_sees_it() {
    let plan = enormous_plan();
    let error = plan
        .coordinates_with_abort(&ResourceLimits::default(), &NoAbort)
        .expect_err("the default batch-run limit is far below 6.4e10");
    match error {
        DeckPlanError::ResourceLimit(limit) => {
            assert_eq!(limit.resource, ResourceKind::BatchRuns);
            assert_eq!(limit.limit, ResourceLimits::default().max_batch_runs);
        }
        other => panic!("expected a preflighted batch-run refusal, got {other:?}"),
    }
}

#[test]
fn a_coordinate_identity_request_too_large_to_hold_is_refused_not_aborted() {
    // `numeric_run_coordinate_id` sizes its canonical buffer from the caller's
    // slice, so a slice this long is impossible to build; the reachable
    // allocation contract is that a *plan* whose assignment accounting
    // overflows is refused with a typed accounting error rather than wrapping.
    let axes = (0..2)
        .map(|index| {
            RunAxis::new(
                AxisKind::Step,
                format!("param:q{index}"),
                (0..3)
                    .map(|value| RunAxisValue::Numeric(f64::from(value) + 1.0))
                    .collect(),
            )
            .expect("a small numeric axis is valid")
        })
        .collect();
    let plan = DeckPlan::new(axes, Vec::new()).expect("two ordered STEP axes are a valid plan");
    let coordinates = plan
        .coordinates_with_abort(&ResourceLimits::default(), &NoAbort)
        .expect("a nine-coordinate plan materializes");
    assert_eq!(coordinates.len(), 9);

    // A non-finite assignment is refused rather than hashed, which is the
    // fail-closed half of the same contract.
    let refused = numeric_run_coordinate_id(&[(AxisKind::Step, "param:q0", f64::NAN)], 0);
    assert!(matches!(
        refused,
        Err(DeckPlanError::NonFiniteAxisValue { .. })
    ));
}

#[test]
fn an_oversized_expansion_is_refused_by_its_declared_resource_kind() {
    let source = "resource limit fixture\n\
        .param vdd=1\n\
        V1 a 0 {vdd}\n\
        .data tbl vdd\n\
        1\n\
        2\n\
        3\n\
        .enddata\n\
        .dc data=tbl\n\
        .end\n";
    let mut limits = ResourceLimits::default();
    limits.max_batch_runs = 2;
    let error = try_expand_multi_run_with_limits_and_abort(source, limits, &NoAbort)
        .expect_err("three DATA rows exceed a two-run policy");
    let refusal = error
        .resource_limit_error()
        .expect("a limit refusal keeps its typed resource details");
    assert_eq!(refusal.resource, ResourceKind::BatchRuns);
    assert_eq!(refusal.requested, 3);
    assert_eq!(refusal.limit, 2);
}

//=============================================================================
// Checkpoint publication under injected failure
//=============================================================================

const CHECKPOINT_DECK: &str = "\
checkpoint publication fixture
V1 in 0 PULSE(0 1 0 100n 100n 4u 10u)
R1 in out 1k
C1 out 0 2n
.TRAN 50n 4u
.END
";

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(tag: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let serial = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice_ckpt_fault_{}_{tag}_{serial}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("create a checkpoint fault test directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn checkpoint() -> TransientCheckpoint {
    let netlist = Netlist::parse(CHECKPOINT_DECK).expect("the checkpoint fixture parses");
    let (_, checkpoint) = Engine::new(SimulationConfig::default())
        .run_tran_checkpointed(&netlist, 2.0e-6, 50.0e-9)
        .expect("the first segment solves");
    checkpoint
}

/// Every phase a single-file publication passes through.
const SINGLE_FILE_FAULT_POINTS: &[ArtifactFaultPoint] = &[
    ArtifactFaultPoint::AfterStagingCreated,
    ArtifactFaultPoint::BeforeFlush,
    ArtifactFaultPoint::AfterFlush,
    ArtifactFaultPoint::BeforeCommit,
    ArtifactFaultPoint::Replace,
];

#[test]
fn a_failed_checkpoint_save_leaves_the_previous_checkpoint_byte_identical() {
    let checkpoint = checkpoint();
    for point in SINGLE_FILE_FAULT_POINTS {
        for allocation in [false, true] {
            let directory = TestDirectory::new("previous");
            let path = directory.path().join("segment.ckpt");
            checkpoint
                .save(&path)
                .expect("the predecessor checkpoint publishes");
            let previous = std::fs::read(&path).expect("the predecessor is readable");

            let fault = if allocation {
                ArtifactFault::allocation(*point)
            } else {
                ArtifactFault::io(*point)
            };
            let armed = ArmedFaults::arm(fault);
            let error = checkpoint
                .save_with_encoding_and_abort(&path, TransientCheckpointEncoding::Packed, &NoAbort)
                .expect_err("the injected failure must fail the save");
            assert_eq!(armed.fired(), 1, "{point:?} never fired");
            drop(armed);

            match error {
                SimulationError::OutputCommitFailed(commit) => assert!(
                    !matches!(
                        commit.phase,
                        OutputCommitPhase::Commit {
                            destination_intact: false
                        }
                    ),
                    "{point:?} reported a lost destination: {commit}"
                ),
                other => panic!("{point:?} produced {other:?} instead of a publication failure"),
            }
            assert_eq!(
                std::fs::read(&path).expect("the predecessor still exists"),
                previous,
                "{point:?} replaced a complete checkpoint with a failed one"
            );
            assert!(
                rspice_output::stale_artifacts(&path)
                    .expect("list staging files")
                    .is_empty(),
                "{point:?} left a staging file behind"
            );
            let reloaded = TransientCheckpoint::load(&path).expect("the predecessor still loads");
            assert_eq!(reloaded, checkpoint);
        }
    }
}

#[test]
fn a_failed_first_checkpoint_save_publishes_no_destination_at_all() {
    let checkpoint = checkpoint();
    for point in SINGLE_FILE_FAULT_POINTS {
        let directory = TestDirectory::new("absent");
        let path = directory.path().join("segment.ckpt");

        let armed = ArmedFaults::arm(ArtifactFault::io(*point));
        let error = checkpoint
            .save_with_abort(&path, &NoAbort)
            .expect_err("the injected failure must fail the save");
        assert_eq!(armed.fired(), 1, "{point:?} never fired");
        drop(armed);

        assert!(
            matches!(error, SimulationError::OutputCommitFailed(_)),
            "{point:?} produced {error:?} instead of a publication failure"
        );
        assert!(
            !path.exists(),
            "{point:?} left a partial checkpoint at the destination"
        );
        assert!(
            rspice_output::stale_artifacts(&path)
                .expect("list staging files")
                .is_empty(),
            "{point:?} left a staging file behind"
        );
    }
}

#[test]
fn an_unarmed_checkpoint_save_publishes_normally() {
    let checkpoint = checkpoint();
    let directory = TestDirectory::new("unarmed");
    let path = directory.path().join("segment.ckpt");
    checkpoint
        .save_with_abort(&path, &NoAbort)
        .expect("an unarmed save publishes");
    assert_eq!(
        TransientCheckpoint::load(&path).expect("the published checkpoint loads"),
        checkpoint
    );
}
