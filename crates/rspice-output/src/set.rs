//! Transactional publication of a set of artifacts.
//!
//! A logical result is often several files: one artifact per swept
//! coordinate, a transient artifact beside its FFT artifact, or a set of
//! result documents beside the manifest that describes them. Publishing them
//! one at a time makes a cancellation or a failure between two members
//! visible as a complete-looking result that is missing part of itself.
//!
//! [`AtomicArtifactSet`] removes that window. Every member is written to a
//! staging file beside its own destination and completely flushed and
//! synchronized before any destination is touched. The commit then snapshots
//! every predecessor, replaces the destinations in order, and — if any
//! replacement fails — restores every predecessor byte-identically and
//! removes the destinations that did not previously exist. The manifest
//! member, when the caller stages one, is always committed last.

use std::collections::HashSet;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::{
    AmbientFaults, AtomicArtifactError, AtomicArtifactFile, CommitOperation, DestinationState,
    FaultHooks, FaultPoint, PREDECESSOR_MARKER, PreparedAtomicArtifact, create_sibling_file,
    remove_artifact_durably, restore_artifact_durably, sync_parent_directory,
};

/// Why a staged artifact could not join a set.
#[derive(Debug, Error)]
pub enum SetMembershipError {
    /// Two members of one set would replace the same destination, so a
    /// rollback could not restore a well-defined predecessor.
    #[error("artifact set already publishes {}", .0.display())]
    DuplicateDestination(PathBuf),
    /// A set has at most one manifest member.
    #[error("artifact set already has a manifest member at {}", .0.display())]
    DuplicateManifest(PathBuf),
}

/// What a failed set commit left behind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RollbackOutcome {
    /// Every predecessor is byte-identical again and no destination that was
    /// previously absent exists.
    PredecessorsRestored,
    /// Rollback could not finish. The destinations named in `failures` are
    /// indeterminate and any snapshot listed in `retained` is the last copy
    /// of the corresponding predecessor.
    Incomplete {
        failures: Vec<String>,
        retained: Vec<PathBuf>,
    },
}

impl std::fmt::Display for RollbackOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PredecessorsRestored => formatter.write_str("the predecessor set was restored"),
            Self::Incomplete { failures, retained } => write!(
                formatter,
                "rollback did not finish: {}{}",
                failures.join("; "),
                retained_suffix(retained)
            ),
        }
    }
}

fn retained_suffix(retained: &[PathBuf]) -> String {
    if retained.is_empty() {
        return String::new();
    }
    format!(
        "; retained predecessor snapshots: {}",
        retained
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// Typed failure phases for set publication.
#[derive(Debug, Error)]
pub enum AtomicArtifactSetError<E>
where
    E: std::error::Error + 'static,
{
    /// A member could not be staged. No destination was touched.
    #[error("artifact set member {} could not be staged: {source}", destination.display())]
    Stage {
        destination: PathBuf,
        #[source]
        source: AtomicArtifactError<E>,
    },
    /// A staged member could not join the set. No destination was touched.
    #[error(transparent)]
    Membership(#[from] SetMembershipError),
    /// A predecessor could not be snapshotted. No destination was touched.
    #[error(
        "artifact set predecessor {} could not be captured: {source}{}",
        destination.display(),
        retained_suffix(retained)
    )]
    Predecessor {
        destination: PathBuf,
        retained: Vec<PathBuf>,
        #[source]
        source: io::Error,
    },
    /// A member could not be committed; `outcome` reports what the rollback
    /// restored.
    #[error(
        "artifact set member {} could not be committed: {source}; {outcome}",
        destination.display()
    )]
    Commit {
        destination: PathBuf,
        outcome: RollbackOutcome,
        #[source]
        source: AtomicArtifactError<io::Error>,
    },
    /// Every destination was published, but the predecessor snapshots or the
    /// directory entries could not be finalized.
    #[error("published artifact set could not be finalized: {failures}")]
    Finalize { failures: String },
}

/// One artifact written completely into a staging file beside its
/// destination, waiting to join a set.
///
/// Staging is the expensive part of publication, so it is available without
/// the set: a caller that produces members concurrently stages each one on
/// its own thread and then joins them under its own synchronization.
/// Dropping a staged artifact removes only its staging file.
#[derive(Debug)]
pub struct StagedArtifact {
    destination: PathBuf,
    prepared: PreparedAtomicArtifact,
}

impl StagedArtifact {
    /// Serialize one member into a staging file beside `destination` and
    /// complete all of its durability work.
    pub fn stage<E, F>(destination: &Path, write: F) -> Result<Self, AtomicArtifactSetError<E>>
    where
        E: std::error::Error + 'static,
        F: FnOnce(&mut dyn Write) -> Result<(), E>,
    {
        Self::stage_impl(destination, write, &mut AmbientFaults)
    }

    /// Destination this member will replace when its set commits.
    #[must_use]
    pub fn destination(&self) -> &Path {
        &self.destination
    }

    pub(crate) fn stage_impl<E, F, H>(
        destination: &Path,
        write: F,
        hooks: &mut H,
    ) -> Result<Self, AtomicArtifactSetError<E>>
    where
        E: std::error::Error + 'static,
        F: FnOnce(&mut dyn Write) -> Result<(), E>,
        H: FaultHooks,
    {
        let stage_error = |source| AtomicArtifactSetError::Stage {
            destination: destination.to_path_buf(),
            source,
        };
        let artifact =
            AtomicArtifactFile::prepare_impl::<E, _>(destination, hooks).map_err(stage_error)?;
        let mut writer = BufWriter::new(artifact);
        if let Err(source) = hooks
            .check(FaultPoint::SetStage)
            .map_err(AtomicArtifactError::Prepare)
            .and_then(|()| write(&mut writer).map_err(AtomicArtifactError::Write))
        {
            drop(writer);
            return Err(stage_error(source));
        }
        let artifact = writer
            .into_inner()
            .map_err(|error| AtomicArtifactError::Flush {
                operation: crate::FlushOperation::FlushBuffer,
                source: error.into_error(),
            })
            .map_err(stage_error)?;
        let prepared = artifact
            .prepare_for_commit_impl::<E, _>(hooks)
            .map_err(stage_error)?;
        Ok(Self {
            destination: destination.to_path_buf(),
            prepared,
        })
    }
}

/// A set of artifacts published as one transaction.
///
/// Dropping a set that was not committed removes every staging file and
/// leaves every destination byte-identical.
#[derive(Debug, Default)]
pub struct AtomicArtifactSet {
    members: Vec<SetMember>,
    manifest: Option<SetMember>,
}

#[derive(Debug)]
struct SetMember {
    destination: PathBuf,
    prepared: Option<PreparedAtomicArtifact>,
    predecessor: Predecessor,
    committed: bool,
}

impl SetMember {
    fn new(staged: StagedArtifact) -> Self {
        Self {
            destination: staged.destination,
            prepared: Some(staged.prepared),
            predecessor: Predecessor::Absent,
            committed: false,
        }
    }
}

impl AtomicArtifactSet {
    /// An empty transaction.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Destinations this transaction will replace, in commit order.
    pub fn destinations(&self) -> impl Iterator<Item = &Path> {
        self.members
            .iter()
            .chain(self.manifest.as_ref())
            .map(|member| member.destination.as_path())
    }

    /// Stage one member and join it to the set.
    pub fn stage<E, F>(
        &mut self,
        destination: &Path,
        write: F,
    ) -> Result<(), AtomicArtifactSetError<E>>
    where
        E: std::error::Error + 'static,
        F: FnOnce(&mut dyn Write) -> Result<(), E>,
    {
        let staged = StagedArtifact::stage(destination, write)?;
        self.adopt(staged)?;
        Ok(())
    }

    /// Join an already staged member to the set.
    pub fn adopt(&mut self, staged: StagedArtifact) -> Result<(), SetMembershipError> {
        self.reject_duplicate(&staged.destination)?;
        self.members.push(SetMember::new(staged));
        Ok(())
    }

    /// Join an already staged manifest to the set.
    pub fn adopt_manifest(&mut self, staged: StagedArtifact) -> Result<(), SetMembershipError> {
        if let Some(manifest) = &self.manifest {
            return Err(SetMembershipError::DuplicateManifest(
                manifest.destination.clone(),
            ));
        }
        self.reject_duplicate(&staged.destination)?;
        self.manifest = Some(SetMember::new(staged));
        Ok(())
    }

    /// Publish every member, or none of them.
    ///
    /// On success every destination holds its new complete bytes and no
    /// staging file or predecessor snapshot remains. On a commit failure the
    /// error reports the failing destination and what the rollback restored.
    pub fn commit(self) -> Result<(), AtomicArtifactSetError<io::Error>> {
        self.commit_impl(&mut AmbientFaults)
    }

    fn reject_duplicate(&self, destination: &Path) -> Result<(), SetMembershipError> {
        if self
            .destinations()
            .any(|existing| same_destination(existing, destination))
        {
            return Err(SetMembershipError::DuplicateDestination(
                destination.to_path_buf(),
            ));
        }
        Ok(())
    }

    pub(crate) fn commit_impl<H>(
        mut self,
        hooks: &mut H,
    ) -> Result<(), AtomicArtifactSetError<io::Error>>
    where
        H: FaultHooks,
    {
        let mut members = std::mem::take(&mut self.members);
        let manifest_index = self.manifest.as_ref().map(|_| members.len());
        members.extend(self.manifest.take());

        for index in 0..members.len() {
            hooks.enter_member(index);
            let capture = hooks
                .check(FaultPoint::SetPredecessor)
                .and_then(|()| capture_predecessor(&members[index].destination));
            match capture {
                Ok(predecessor) => members[index].predecessor = predecessor,
                Err(source) => {
                    let destination = members[index].destination.clone();
                    let retained = discard_predecessor_snapshots(&mut members);
                    return Err(AtomicArtifactSetError::Predecessor {
                        destination,
                        retained,
                        source,
                    });
                }
            }
        }

        for index in 0..members.len() {
            hooks.enter_member(index);
            let prepared = match members[index].prepared.take() {
                Some(prepared) => prepared,
                // Commit consumes the set and visits each member once, so
                // this is unreachable; it fails the transaction closed rather
                // than panicking if that ever stops being true.
                None => {
                    let destination = members[index].destination.clone();
                    let outcome = rollback(&mut members);
                    return Err(AtomicArtifactSetError::Commit {
                        destination,
                        outcome,
                        source: AtomicArtifactError::Commit {
                            operation: CommitOperation::PreCommit,
                            destination_state: DestinationState::Unchanged,
                            source: io::Error::new(
                                io::ErrorKind::BrokenPipe,
                                "artifact set member was already consumed",
                            ),
                        },
                    });
                }
            };
            let manifest_fault = if manifest_index == Some(index) {
                hooks.check(FaultPoint::SetManifestCommit)
            } else {
                Ok(())
            };
            let result = manifest_fault
                .map_err(|source| AtomicArtifactError::Commit {
                    operation: CommitOperation::PreCommit,
                    destination_state: DestinationState::Unchanged,
                    source,
                })
                .and_then(|()| prepared.commit_impl::<io::Error, _>(hooks));
            match result {
                Ok(()) => members[index].committed = true,
                Err(source) => {
                    members[index].committed = matches!(
                        source,
                        AtomicArtifactError::Commit {
                            destination_state: DestinationState::PublishedDurabilityUncertain,
                            ..
                        }
                    );
                    let destination = members[index].destination.clone();
                    let outcome = rollback(&mut members);
                    return Err(AtomicArtifactSetError::Commit {
                        destination,
                        outcome,
                        source,
                    });
                }
            }
        }

        let mut failures = discard_predecessor_snapshot_failures(&mut members);
        if let Err(error) = sync_member_parents(&members) {
            failures.push(error);
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(AtomicArtifactSetError::Finalize {
                failures: failures.join("; "),
            })
        }
    }
}

/// Whether two destinations name the same artifact for rollback purposes.
///
/// Windows resolves paths case-insensitively, so two members whose names
/// differ only in case would replace one another.
fn same_destination(left: &Path, right: &Path) -> bool {
    if cfg!(windows) {
        let (left, right) = (left.as_os_str(), right.as_os_str());
        left.as_encoded_bytes()
            .eq_ignore_ascii_case(right.as_encoded_bytes())
    } else {
        left == right
    }
}

/// The state a destination had before the transaction touched it.
#[derive(Debug)]
enum Predecessor {
    /// Nothing existed; rollback removes a committed destination.
    Absent,
    /// A regular file existed and was snapshotted at `snapshot`.
    File { snapshot: PathBuf },
    /// A directory existed. A regular file cannot atomically replace a
    /// directory, so the commit fails and the directory is left alone. Other
    /// non-regular objects fail closed in [`capture_predecessor`] because
    /// `rename` replaces them on Unix.
    Directory,
}

fn capture_predecessor(destination: &Path) -> io::Result<Predecessor> {
    let metadata = match std::fs::symlink_metadata(destination) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Predecessor::Absent),
        Err(error) => return Err(error),
    };
    if metadata.file_type().is_dir() {
        return Ok(Predecessor::Directory);
    }
    if !metadata.file_type().is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "artifact predecessor {} is neither a regular file nor a directory",
                destination.display()
            ),
        ));
    }
    Ok(Predecessor::File {
        snapshot: snapshot_predecessor(destination)?,
    })
}

/// Take a byte-exact snapshot of an existing artifact.
///
/// A hard link is preferred because it is constant time and shares the
/// predecessor's data blocks. Filesystems that do not implement links (FAT
/// and exFAT volumes are the common case for an exported result directory)
/// get a synchronized copy instead, which produces the same bytes.
fn snapshot_predecessor(destination: &Path) -> io::Result<PathBuf> {
    // The reservation proves the name is unused; the name itself carries this
    // process id and a process-local serial, so releasing it for the link
    // cannot collide with another RSpice writer.
    let (snapshot, reservation) = create_sibling_file(destination, PREDECESSOR_MARKER, ".bak")?;
    drop(reservation);
    if std::fs::remove_file(&snapshot).is_ok() && std::fs::hard_link(destination, &snapshot).is_ok()
    {
        return Ok(snapshot);
    }

    let mut source = std::fs::File::open(destination)?;
    let mut copy = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&snapshot)?;
    if let Err(error) = io::copy(&mut source, &mut copy).and_then(|_| copy.sync_all()) {
        let _ = std::fs::remove_file(&snapshot);
        return Err(error);
    }
    Ok(snapshot)
}

/// Restore every predecessor in reverse commit order.
fn rollback(members: &mut [SetMember]) -> RollbackOutcome {
    let mut failures = Vec::new();
    let mut retained = Vec::new();
    for member in members.iter_mut().rev() {
        let restored = match &member.predecessor {
            Predecessor::Absent if member.committed => remove_artifact_durably(&member.destination),
            Predecessor::Absent | Predecessor::Directory => Ok(()),
            Predecessor::File { snapshot } => {
                if member.committed {
                    restore_artifact_durably(snapshot, &member.destination)
                } else {
                    remove_artifact_durably(snapshot)
                }
            }
        };
        match restored {
            Ok(()) => member.predecessor = Predecessor::Absent,
            Err(error) => {
                failures.push(format!("{}: {error}", member.destination.display()));
                if let Predecessor::File { snapshot } = &member.predecessor {
                    retained.push(snapshot.clone());
                }
            }
        }
        member.prepared.take();
    }
    if let Err(error) = sync_member_parents(members) {
        failures.push(error);
    }
    if failures.is_empty() {
        RollbackOutcome::PredecessorsRestored
    } else {
        RollbackOutcome::Incomplete { failures, retained }
    }
}

/// Remove the snapshots taken so far without touching any destination.
fn discard_predecessor_snapshots(members: &mut [SetMember]) -> Vec<PathBuf> {
    let mut retained = Vec::new();
    for member in members.iter_mut() {
        if let Predecessor::File { snapshot } = &member.predecessor {
            match remove_artifact_durably(snapshot) {
                Ok(()) => member.predecessor = Predecessor::Absent,
                Err(_) => retained.push(snapshot.clone()),
            }
        }
    }
    retained
}

/// Remove the snapshots of a fully committed set, reporting failures.
fn discard_predecessor_snapshot_failures(members: &mut [SetMember]) -> Vec<String> {
    let mut failures = Vec::new();
    for member in members.iter_mut() {
        if let Predecessor::File { snapshot } = &member.predecessor {
            if let Err(error) = remove_artifact_durably(snapshot) {
                failures.push(format!(
                    "cannot remove predecessor snapshot {}: {error}",
                    snapshot.display()
                ));
            } else {
                member.predecessor = Predecessor::Absent;
            }
        }
    }
    failures
}

fn sync_member_parents(members: &[SetMember]) -> Result<(), String> {
    let mut synchronized = HashSet::new();
    for member in members {
        let parent = crate::destination_parent(&member.destination).to_path_buf();
        if synchronized.insert(parent) {
            sync_parent_directory(&member.destination).map_err(|error| {
                format!(
                    "cannot synchronize artifact set directory for {}: {error}",
                    member.destination.display()
                )
            })?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{stale_artifacts, tests_support::TestDirectory};
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Fires one fault point, optionally only while a chosen member is being
    /// processed.
    struct InjectSetFault {
        point: FaultPoint,
        member: Option<usize>,
        current: usize,
        fired: AtomicUsize,
    }

    impl InjectSetFault {
        fn new(point: FaultPoint, member: Option<usize>) -> Self {
            Self {
                point,
                member,
                current: 0,
                fired: AtomicUsize::new(0),
            }
        }
    }

    impl FaultHooks for InjectSetFault {
        fn check(&mut self, point: FaultPoint) -> io::Result<()> {
            if point != self.point {
                return Ok(());
            }
            if let Some(member) = self.member
                && member != self.current
            {
                return Ok(());
            }
            if self.fired.fetch_add(1, Ordering::Relaxed) > 0 {
                return Ok(());
            }
            Err(io::Error::other(format!("injected {point:?} failure")))
        }

        fn enter_member(&mut self, index: usize) {
            self.current = index;
        }
    }

    fn member_paths(directory: &TestDirectory, count: usize) -> Vec<PathBuf> {
        (0..count)
            .map(|index| directory.path().join(format!("coordinate-{index}.csv")))
            .collect()
    }

    /// Stage the manifest of a set under test.
    fn stage_manifest(
        set: &mut AtomicArtifactSet,
        destination: &Path,
    ) -> Result<(), AtomicArtifactSetError<io::Error>> {
        let staged = StagedArtifact::stage::<io::Error, _>(destination, |writer| {
            writer.write_all(b"{\"members\":3}")
        })?;
        set.adopt_manifest(staged)?;
        Ok(())
    }

    fn seed_set(paths: &[PathBuf], preexisting: bool) {
        if preexisting {
            for (index, path) in paths.iter().enumerate() {
                std::fs::write(path, format!("old {index}")).expect("seed predecessor");
            }
        }
    }

    fn assert_set_unpublished(paths: &[PathBuf], preexisting: bool) {
        for (index, path) in paths.iter().enumerate() {
            if preexisting {
                assert_eq!(
                    std::fs::read(path).expect("read preserved predecessor"),
                    format!("old {index}").as_bytes(),
                    "{} was not restored byte-identically",
                    path.display()
                );
            } else {
                assert!(
                    !path.exists(),
                    "{} was published by a failed set",
                    path.display()
                );
            }
            assert!(
                stale_artifacts(path).expect("list stages").is_empty(),
                "{} kept a staging file",
                path.display()
            );
        }
    }

    fn assert_no_snapshots(directory: &TestDirectory) {
        let leaked = std::fs::read_dir(directory.path())
            .expect("list transaction directory")
            .map(|entry| entry.expect("directory entry").file_name())
            .filter(|name| name.to_string_lossy().contains(PREDECESSOR_MARKER))
            .collect::<Vec<_>>();
        assert!(
            leaked.is_empty(),
            "predecessor snapshots leaked: {leaked:?}"
        );
    }

    fn stage_all(
        set: &mut AtomicArtifactSet,
        paths: &[PathBuf],
        hooks: &mut impl FaultHooks,
    ) -> Result<(), AtomicArtifactSetError<io::Error>> {
        for (index, path) in paths.iter().enumerate() {
            hooks.enter_member(index);
            let staged = StagedArtifact::stage_impl::<io::Error, _, _>(
                path,
                |writer| writer.write_all(format!("new {index}").as_bytes()),
                hooks,
            )?;
            set.adopt(staged)?;
        }
        Ok(())
    }

    #[test]
    fn committed_set_publishes_every_member_and_the_manifest_last() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("set-success");
            let paths = member_paths(&directory, 3);
            seed_set(&paths, preexisting);
            let manifest_path = directory.path().join("set.json");
            let mut set = AtomicArtifactSet::new();
            stage_all(&mut set, &paths, &mut AmbientFaults).expect("stage members");
            stage_manifest(&mut set, &manifest_path).expect("stage manifest");
            assert_eq!(set.destinations().count(), 4);
            assert_eq!(
                set.destinations().last().expect("manifest is last"),
                manifest_path.as_path()
            );

            set.commit().expect("commit artifact set");

            for (index, path) in paths.iter().enumerate() {
                assert_eq!(
                    std::fs::read(path).expect("read published member"),
                    format!("new {index}").as_bytes()
                );
            }
            assert_eq!(
                std::fs::read(&manifest_path).expect("read manifest"),
                b"{\"members\":3}"
            );
            assert_no_snapshots(&directory);
        }
    }

    #[test]
    fn dropping_a_partially_staged_set_publishes_nothing() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("set-cancelled");
            let paths = member_paths(&directory, 3);
            seed_set(&paths, preexisting);
            let mut set = AtomicArtifactSet::new();

            // Cancellation between members: the second coordinate is staged
            // and the third never runs.
            stage_all(&mut set, &paths[..2], &mut AmbientFaults).expect("stage members");
            assert_eq!(set.destinations().count(), 2);
            drop(set);

            assert_set_unpublished(&paths, preexisting);
            assert_no_snapshots(&directory);
        }
    }

    #[test]
    fn staging_and_preparation_faults_publish_nothing() {
        for preexisting in [false, true] {
            for point in [
                FaultPoint::AfterPrepare,
                FaultPoint::SetStage,
                FaultPoint::Flush,
                FaultPoint::AfterFlush,
            ] {
                let directory = TestDirectory::new("set-stage-fault");
                let paths = member_paths(&directory, 3);
                seed_set(&paths, preexisting);
                let mut set = AtomicArtifactSet::new();
                let mut hooks = InjectSetFault::new(point, Some(1));

                let error = stage_all(&mut set, &paths, &mut hooks)
                    .expect_err("injected staging fault must propagate");
                assert!(
                    matches!(&error, AtomicArtifactSetError::Stage { destination, .. }
                        if destination == &paths[1]),
                    "{error}"
                );
                drop(set);

                assert_set_unpublished(&paths, preexisting);
                assert_no_snapshots(&directory);
            }
        }
    }

    #[test]
    fn predecessor_capture_failure_publishes_nothing() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("set-predecessor-fault");
            let paths = member_paths(&directory, 3);
            seed_set(&paths, preexisting);
            let mut set = AtomicArtifactSet::new();
            stage_all(&mut set, &paths, &mut AmbientFaults).expect("stage members");

            let error = set
                .commit_impl(&mut InjectSetFault::new(
                    FaultPoint::SetPredecessor,
                    Some(2),
                ))
                .expect_err("injected predecessor fault must propagate");
            assert!(
                matches!(&error, AtomicArtifactSetError::Predecessor { destination, retained, .. }
                    if destination == &paths[2] && retained.is_empty()),
                "{error}"
            );

            assert_set_unpublished(&paths, preexisting);
            assert_no_snapshots(&directory);
        }
    }

    #[test]
    fn commit_failure_after_earlier_members_restores_every_predecessor() {
        for preexisting in [false, true] {
            for failing in [0, 1, 2] {
                let directory = TestDirectory::new("set-commit-rollback");
                let paths = member_paths(&directory, 3);
                seed_set(&paths, preexisting);
                let mut set = AtomicArtifactSet::new();
                stage_all(&mut set, &paths, &mut AmbientFaults).expect("stage members");

                let error = set
                    .commit_impl(&mut InjectSetFault::new(FaultPoint::Replace, Some(failing)))
                    .expect_err("injected member commit fault must propagate");
                assert!(
                    matches!(&error, AtomicArtifactSetError::Commit { destination, outcome, .. }
                        if destination == &paths[failing]
                            && outcome == &RollbackOutcome::PredecessorsRestored),
                    "{error}"
                );

                assert_set_unpublished(&paths, preexisting);
                assert_no_snapshots(&directory);
            }
        }
    }

    #[test]
    fn published_member_with_uncertain_durability_is_rolled_back() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("set-published-rollback");
            let paths = member_paths(&directory, 3);
            seed_set(&paths, preexisting);
            let mut set = AtomicArtifactSet::new();
            stage_all(&mut set, &paths, &mut AmbientFaults).expect("stage members");

            let error = set
                .commit_impl(&mut InjectSetFault::new(FaultPoint::SyncParent, Some(1)))
                .expect_err("injected durability fault must propagate");
            assert!(
                matches!(&error, AtomicArtifactSetError::Commit { destination, outcome, .. }
                    if destination == &paths[1]
                        && outcome == &RollbackOutcome::PredecessorsRestored),
                "{error}"
            );

            assert_set_unpublished(&paths, preexisting);
            assert_no_snapshots(&directory);
        }
    }

    #[test]
    fn manifest_commit_failure_rolls_back_the_whole_set() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("set-manifest-rollback");
            let paths = member_paths(&directory, 3);
            seed_set(&paths, preexisting);
            let manifest_path = directory.path().join("set.json");
            let mut set = AtomicArtifactSet::new();
            stage_all(&mut set, &paths, &mut AmbientFaults).expect("stage members");
            stage_manifest(&mut set, &manifest_path).expect("stage manifest");

            let error = set
                .commit_impl(&mut InjectSetFault::new(
                    FaultPoint::SetManifestCommit,
                    None,
                ))
                .expect_err("injected manifest fault must propagate");
            assert!(
                matches!(&error, AtomicArtifactSetError::Commit { destination, outcome, .. }
                    if destination == &manifest_path
                        && outcome == &RollbackOutcome::PredecessorsRestored),
                "{error}"
            );

            assert_set_unpublished(&paths, preexisting);
            assert!(!manifest_path.exists(), "manifest survived a rollback");
            assert_no_snapshots(&directory);
        }
    }

    #[test]
    fn a_directory_destination_fails_the_set_without_publishing_members() {
        let directory = TestDirectory::new("set-directory-member");
        let paths = member_paths(&directory, 2);
        seed_set(&paths[..1], true);
        std::fs::create_dir(&paths[1]).expect("create conflicting directory destination");
        let mut set = AtomicArtifactSet::new();
        stage_all(&mut set, &paths, &mut AmbientFaults).expect("stage members");

        let error = set.commit().expect_err("directory member must fail commit");
        assert!(
            matches!(&error, AtomicArtifactSetError::Commit { destination, outcome, .. }
                if destination == &paths[1] && outcome == &RollbackOutcome::PredecessorsRestored),
            "{error}"
        );
        assert_eq!(
            std::fs::read(&paths[0]).expect("read restored predecessor"),
            b"old 0"
        );
        assert!(paths[1].is_dir());
        assert_no_snapshots(&directory);
    }

    #[test]
    fn duplicate_destinations_are_refused_before_any_commit() {
        let directory = TestDirectory::new("set-duplicate");
        let destination = directory.path().join("result.csv");
        let mut set = AtomicArtifactSet::new();
        set.stage::<io::Error, _>(&destination, |writer| writer.write_all(b"first"))
            .expect("stage first member");
        let error = set
            .stage::<io::Error, _>(&destination, |writer| writer.write_all(b"second"))
            .expect_err("duplicate destination must fail closed");
        assert!(
            matches!(
                error,
                AtomicArtifactSetError::Membership(SetMembershipError::DuplicateDestination(_))
            ),
            "{error}"
        );

        let manifest = directory.path().join("set.json");
        stage_manifest(&mut set, &manifest).expect("stage manifest");
        let error = stage_manifest(&mut set, &directory.path().join("other.json"))
            .expect_err("second manifest must fail closed");
        assert!(
            matches!(
                error,
                AtomicArtifactSetError::Membership(SetMembershipError::DuplicateManifest(_))
            ),
            "{error}"
        );
    }
}
