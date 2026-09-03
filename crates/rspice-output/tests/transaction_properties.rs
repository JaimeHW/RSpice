//! Transaction laws for artifact publication, proved under injected failure.
//!
//! Rule 8 of the engineering contract — output is transactional — says a
//! failed or cancelled run leaves either the old complete artifact or no
//! artifact, never a partially written replacement. That is a claim about
//! failure paths, so it is stated here as a law over *every* publication
//! phase: for each [`ArtifactFaultPoint`], for each member of a generated set,
//! and for both an I/O failure and a refused allocation, the transaction must
//! end with every destination byte-identical to what it was and no staging or
//! predecessor file left behind.
//!
//! Two success-path laws frame it: a committed set replaces every destination,
//! and an abandoned set replaces none.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

use rspice_output::fault::{ArmedFaults, ArtifactFault, ArtifactFaultKind, ArtifactFaultPoint};
use rspice_output::{
    AtomicArtifactError, AtomicArtifactSet, DestinationState, PREDECESSOR_MARKER, StagedArtifact,
    stale_artifacts, write_atomic,
};

fn runner(seed: u64, cases: u32) -> TestRunner {
    let mut entropy = [0_u8; 32];
    entropy[..8].copy_from_slice(&seed.to_le_bytes());
    TestRunner::new_with_rng(
        Config {
            cases,
            failure_persistence: None,
            ..Config::default()
        },
        TestRng::from_seed(RngAlgorithm::ChaCha, &entropy),
    )
}

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

/// A uniquely named directory removed when the guard drops.
struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(tag: &str) -> Self {
        let id = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("rspice-txn-{}-{id}-{tag}", std::process::id()));
        std::fs::create_dir_all(&path).expect("create a unique transaction test directory");
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

/// One generated transaction: how many members, which of them already have a
/// published predecessor, and whether the set carries a manifest member.
#[derive(Debug, Clone)]
struct Transaction {
    member_count: usize,
    predecessors: Vec<bool>,
    with_manifest: bool,
}

fn transaction() -> impl Strategy<Value = Transaction> {
    (1_usize..4, any::<bool>()).prop_flat_map(|(member_count, with_manifest)| {
        prop::collection::vec(any::<bool>(), member_count..=member_count).prop_map(
            move |predecessors| Transaction {
                member_count,
                predecessors,
                with_manifest,
            },
        )
    })
}

impl Transaction {
    fn destinations(&self, directory: &TestDirectory) -> Vec<PathBuf> {
        let mut paths = (0..self.member_count)
            .map(|index| directory.path().join(format!("coordinate-{index}.csv")))
            .collect::<Vec<_>>();
        if self.with_manifest {
            paths.push(directory.path().join("manifest.json"));
        }
        paths
    }

    /// Publish the predecessors this case declares and return the exact
    /// directory contents the transaction must preserve on failure.
    fn seed_predecessors(&self, paths: &[PathBuf]) -> BTreeMap<PathBuf, Option<Vec<u8>>> {
        let mut before = BTreeMap::new();
        for (index, path) in paths.iter().enumerate() {
            let exists = self.predecessors.get(index).copied().unwrap_or(true);
            if exists {
                let bytes = format!("old bytes for {index}\n").into_bytes();
                std::fs::write(path, &bytes).expect("publish a predecessor artifact");
                before.insert(path.clone(), Some(bytes));
            } else {
                before.insert(path.clone(), None);
            }
        }
        before
    }
}

fn successor_bytes(index: usize) -> Vec<u8> {
    format!("new bytes for {index}, long enough to differ in length\n").into_bytes()
}

/// Stage every member of `paths` into a fresh set, manifest last when asked.
fn stage_set(paths: &[PathBuf], with_manifest: bool) -> Result<AtomicArtifactSet, std::io::Error> {
    let mut set = AtomicArtifactSet::new();
    for (index, path) in paths.iter().enumerate() {
        let staged = StagedArtifact::stage(path, |writer: &mut dyn Write| {
            writer.write_all(&successor_bytes(index))
        })
        .map_err(std::io::Error::other)?;
        let joined = if with_manifest && index + 1 == paths.len() {
            set.adopt_manifest(staged)
        } else {
            set.adopt(staged)
        };
        joined.map_err(std::io::Error::other)?;
    }
    Ok(set)
}

fn directory_state(paths: &[PathBuf]) -> BTreeMap<PathBuf, Option<Vec<u8>>> {
    paths
        .iter()
        .map(|path| (path.clone(), std::fs::read(path).ok()))
        .collect()
}

/// No staging file and no predecessor snapshot may outlive a transaction.
fn residue(directory: &TestDirectory, paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut left = Vec::new();
    for path in paths {
        left.extend(stale_artifacts(path).expect("list staging files"));
    }
    for entry in std::fs::read_dir(directory.path()).expect("read the artifact directory") {
        let entry = entry.expect("read a directory entry");
        if entry
            .file_name()
            .to_string_lossy()
            .contains(PREDECESSOR_MARKER)
        {
            left.push(entry.path());
        }
    }
    left.sort();
    left.dedup();
    left
}

const FAULT_POINTS: &[ArtifactFaultPoint] = &[
    ArtifactFaultPoint::AfterStagingCreated,
    ArtifactFaultPoint::BeforeFlush,
    ArtifactFaultPoint::AfterFlush,
    ArtifactFaultPoint::BeforeCommit,
    ArtifactFaultPoint::Replace,
    ArtifactFaultPoint::SyncParent,
    ArtifactFaultPoint::StageMember,
    ArtifactFaultPoint::CapturePredecessor,
    ArtifactFaultPoint::CommitManifest,
];

#[test]
fn law_a_committed_set_replaces_every_destination() {
    runner(0x0517_0001, 48)
        .run(&transaction(), |transaction| {
            let directory = TestDirectory::new("commit");
            let paths = transaction.destinations(&directory);
            transaction.seed_predecessors(&paths);

            let set = stage_set(&paths, transaction.with_manifest).expect("every member stages");
            set.commit().expect("a complete set commits");

            for (index, path) in paths.iter().enumerate() {
                prop_assert_eq!(
                    std::fs::read(path).expect("a committed member exists"),
                    successor_bytes(index)
                );
            }
            prop_assert!(residue(&directory, &paths).is_empty());
            Ok(())
        })
        .expect("a committed set publishes every member");
}

#[test]
fn law_an_abandoned_set_replaces_no_destination() {
    runner(0x0517_0002, 48)
        .run(&transaction(), |transaction| {
            let directory = TestDirectory::new("abandon");
            let paths = transaction.destinations(&directory);
            let before = transaction.seed_predecessors(&paths);

            drop(stage_set(&paths, transaction.with_manifest).expect("every member stages"));

            prop_assert_eq!(directory_state(&paths), before);
            prop_assert!(residue(&directory, &paths).is_empty());
            Ok(())
        })
        .expect("dropping a set leaves every destination as it was");
}

#[test]
fn law_an_injected_failure_leaves_the_old_artifacts_or_none() {
    runner(0x0517_0003, 24)
        .run(
            &(transaction(), 0_usize..FAULT_POINTS.len(), any::<bool>()),
            |(transaction, point_index, allocation)| {
                let point = FAULT_POINTS[point_index];
                let member_count =
                    transaction.member_count + usize::from(transaction.with_manifest);
                for member in 0..member_count {
                    let directory = TestDirectory::new("fault");
                    let paths = transaction.destinations(&directory);
                    let before = transaction.seed_predecessors(&paths);

                    let fault = if allocation {
                        ArtifactFault::allocation(point)
                    } else {
                        ArtifactFault::io(point)
                    }
                    .on_member(member);

                    let outcome = {
                        let armed = ArmedFaults::arm(fault);
                        let staged = stage_set(&paths, transaction.with_manifest);
                        let outcome = match staged {
                            Err(error) => Err(error.to_string()),
                            Ok(set) => set.commit().map_err(|error| error.to_string()),
                        };
                        // A fault that never fired proves nothing; the phases
                        // that only exist for a manifest are skipped instead.
                        if armed.fired() == 0 {
                            continue;
                        }
                        outcome
                    };

                    prop_assert!(
                        outcome.is_err(),
                        "publication succeeded even though {} failed",
                        point.as_str()
                    );
                    prop_assert_eq!(
                        directory_state(&paths),
                        before.clone(),
                        "{} left a destination changed",
                        point.as_str()
                    );
                    prop_assert!(
                        residue(&directory, &paths).is_empty(),
                        "{} left a staging file or predecessor snapshot behind",
                        point.as_str()
                    );
                }
                Ok(())
            },
        )
        .expect("a failed transaction leaves the old artifacts or none");
}

#[test]
fn law_a_single_file_publication_is_all_or_nothing_under_every_fault() {
    runner(0x0517_0004, 32)
        .run(
            &(0_usize..FAULT_POINTS.len(), any::<bool>(), any::<bool>()),
            |(point_index, allocation, has_predecessor)| {
                let point = FAULT_POINTS[point_index];
                let directory = TestDirectory::new("single");
                let destination = directory.path().join("result.csv");
                let previous = has_predecessor.then(|| b"old single artifact\n".to_vec());
                if let Some(bytes) = &previous {
                    std::fs::write(&destination, bytes).expect("publish a predecessor");
                }

                let fault = if allocation {
                    ArtifactFault::allocation(point)
                } else {
                    ArtifactFault::io(point)
                };
                let successor = b"a new complete artifact\n".to_vec();
                let armed = ArmedFaults::arm(fault);
                let outcome = write_atomic(&destination, |writer: &mut dyn Write| {
                    writer.write_all(&successor)
                });
                let fired = armed.fired();
                drop(armed);

                if fired == 0 {
                    // Set-only phases are not reached by a single publication.
                    prop_assert!(outcome.is_ok());
                    return Ok(());
                }
                let error = outcome.expect_err(&format!("{} did not fail", point.as_str()));
                // A parent-directory synchronization failure happens after the
                // replace, so the destination holds the *complete* successor
                // and the error says its durability is uncertain. Every other
                // phase leaves the predecessor. Either way the destination is
                // one whole artifact, never a partial one.
                let published = matches!(
                    error,
                    AtomicArtifactError::Commit {
                        destination_state: DestinationState::PublishedDurabilityUncertain,
                        ..
                    }
                );
                let expected = if published { Some(successor) } else { previous };
                prop_assert_eq!(
                    std::fs::read(&destination).ok(),
                    expected,
                    "{} left neither the old nor the complete new artifact",
                    point.as_str()
                );
                prop_assert!(residue(&directory, std::slice::from_ref(&destination)).is_empty());
                Ok(())
            },
        )
        .expect("a single artifact is published completely or not at all");
}

#[test]
fn an_injected_allocation_refusal_keeps_its_out_of_memory_class() {
    let directory = TestDirectory::new("oom-class");
    let destination = directory.path().join("result.csv");
    std::fs::write(&destination, b"old bytes\n").expect("publish a predecessor");

    let armed = ArmedFaults::arm(ArtifactFault::allocation(ArtifactFaultPoint::Replace));
    let error = write_atomic(&destination, |writer: &mut dyn Write| {
        writer.write_all(b"successor\n")
    })
    .expect_err("the injected allocation refusal fails the publication");
    assert_eq!(armed.fired(), 1);
    assert_eq!(
        error.into_io_error().kind(),
        std::io::ErrorKind::OutOfMemory,
        "a refused allocation must not be reported as a plain I/O fault"
    );
    assert_eq!(
        std::fs::read(&destination).expect("the predecessor survives"),
        b"old bytes\n"
    );
    assert_eq!(
        ArtifactFault::allocation(ArtifactFaultPoint::Replace).kind(),
        ArtifactFaultKind::Allocation
    );
}

#[test]
fn nothing_is_injected_into_a_thread_that_armed_no_fault() {
    let directory = TestDirectory::new("unarmed");
    let destination = directory.path().join("result.csv");
    write_atomic(&destination, |writer: &mut dyn Write| {
        writer.write_all(b"published\n")
    })
    .expect("an unarmed thread publishes normally");
    assert_eq!(
        std::fs::read(&destination).expect("the artifact exists"),
        b"published\n"
    );
}
