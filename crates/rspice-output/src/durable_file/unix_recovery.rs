//! Unix crash recovery for atomic publication.
//!
//! On Unix a publication is made recoverable by a sidecar record written
//! before the rename and retired only after the new content is verified, so an
//! interrupted write always leaves either the old file or a record naming what
//! was in flight — never a half-published target with nothing to explain it. A
//! record whose endpoints are already settled is retired rather than replayed.

use super::*;

#[cfg(unix)]
pub(super) fn unix_recovery_blocked(
    target: &Path,
    active: &ActiveRecoveryRecord,
    message: impl Into<String>,
) -> PublicationFailure {
    let backup = recovery_backup_path(target, active.record.backup_id);
    PublicationFailure::Uncertain {
        message: message.into(),
        recovery_paths: recovery_paths(&[target, &active.path, &backup]),
    }
}

#[cfg(unix)]
pub(super) fn retire_unix_recovery_record_exact(
    active: &ActiveRecoveryRecord,
) -> Result<(), PublicationFailure> {
    match read_windows_recovery_record(&active.path) {
        Ok(Some(record)) if record == active.record => {}
        observation => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "native recovery record changed before authenticated retirement: {observation:?}"
                ),
                recovery_paths: recovery_paths(&[&active.path]),
            });
        }
    }
    match std::fs::remove_file(&active.path) {
        Ok(()) => sync_parent_of(&active.path).map_err(PublicationFailure::Safe),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            Err(PublicationFailure::Uncertain {
                message: "native recovery record disappeared during authenticated retirement"
                    .to_owned(),
                recovery_paths: Vec::new(),
            })
        }
        Err(error) => Err(PublicationFailure::Safe(error)),
    }
}

#[cfg(unix)]
pub(super) fn unix_aborted_endpoint_is_settled(
    role: ObservedFileRole,
    _before_present: bool,
) -> bool {
    // The Aborted marker is appended only after the transaction verified that
    // its successor was returned to staging (or removed from a new target).
    // Any later readable/missing canonical state belongs to the outside world
    // and must be preserved while stale RSpice evidence is retired.
    !matches!(role, ObservedFileRole::Unreadable)
}

#[cfg(unix)]
pub(super) fn retire_settled_unix_recovery(target: &Path) -> Result<(), PublicationFailure> {
    let Some(active) = select_active_windows_recovery(target)? else {
        return Ok(());
    };
    for (slot_index, slot) in recovery_slot_paths(target).iter().enumerate() {
        if slot_index != active.slot_index && !matches!(slot.try_exists(), Ok(false)) {
            return Err(unix_recovery_blocked(
                target,
                &active,
                format!(
                    "inactive native recovery slot '{}' is still present",
                    slot.display()
                ),
            ));
        }
    }

    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let endpoint_is_settled = match active.record.state {
        RecoveryRecordState::Resolved(PublicationResolution::Committed) => matches!(
            target_role,
            ObservedFileRole::Replacement | ObservedFileRole::Both
        ),
        RecoveryRecordState::Resolved(PublicationResolution::Aborted) => {
            unix_aborted_endpoint_is_settled(target_role, active.record.before_present)
        }
        RecoveryRecordState::Prepared => false,
    };
    let backup = recovery_backup_path(target, active.record.backup_id);
    let backup_role = observe_file_role(
        &backup,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    if !endpoint_is_settled || !matches!(backup_role, ObservedFileRole::Missing) {
        return Err(unix_recovery_blocked(
            target,
            &active,
            format!(
                "native recovery is not settled against its exact endpoint ({:?}, {target_role:?}, {backup_role:?})",
                active.record.state
            ),
        ));
    }
    retire_unix_recovery_record_exact(&active)
}

#[cfg(unix)]
pub(super) fn resolve_unix_recovery_slot(
    target: &Path,
    displaced: Option<&Path>,
    successor_digest: [u8; 32],
    resolution: PublicationResolution,
) -> Result<(), PublicationFailure> {
    let active =
        select_active_windows_recovery(target)?.ok_or_else(|| PublicationFailure::Uncertain {
            message: "native publication has no durable recovery generation".to_owned(),
            recovery_paths: recovery_paths(&[target]),
        })?;
    if active.record.successor_digest != successor_digest {
        return Err(unix_recovery_blocked(
            target,
            &active,
            "native recovery successor does not match the transaction being resolved",
        ));
    }
    let backup = recovery_backup_path(target, active.record.backup_id);
    if displaced.is_some_and(|path| path != backup) {
        return Err(unix_recovery_blocked(
            target,
            &active,
            "displaced predecessor is not the active native recovery sidecar",
        ));
    }
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let backup_role = observe_file_role(
        &backup,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let exact_endpoint = match resolution {
        PublicationResolution::Committed => {
            matches!(
                target_role,
                ObservedFileRole::Replacement | ObservedFileRole::Both
            ) && if active.record.before_present {
                matches!(
                    backup_role,
                    ObservedFileRole::Replaced | ObservedFileRole::Both
                )
            } else {
                matches!(backup_role, ObservedFileRole::Missing)
            }
        }
        PublicationResolution::Aborted => {
            if active.record.before_present {
                !matches!(target_role, ObservedFileRole::Unreadable)
                    && matches!(
                        backup_role,
                        ObservedFileRole::Replacement
                            | ObservedFileRole::Both
                            | ObservedFileRole::Missing
                    )
            } else {
                !matches!(target_role, ObservedFileRole::Unreadable)
                    && matches!(
                        backup_role,
                        ObservedFileRole::Replacement | ObservedFileRole::Missing
                    )
            }
        }
    };
    if !exact_endpoint {
        return Err(unix_recovery_blocked(
            target,
            &active,
            format!(
                "native recovery resolution {resolution:?} does not match exact endpoint roles {target_role:?}/{backup_role:?}"
            ),
        ));
    }
    append_windows_recovery_resolution(&active, resolution)
}

#[cfg(unix)]
pub(super) fn settle_prepared_unix_abort(
    target: &Path,
    active: &ActiveRecoveryRecord,
) -> Result<(), PublicationFailure> {
    append_windows_recovery_resolution(active, PublicationResolution::Aborted)?;
    let backup = recovery_backup_path(target, active.record.backup_id);
    remove_verified_recovery_sidecar(
        &backup,
        ObservedFileRole::Replacement,
        active.record.predecessor_digest,
        active.record.successor_digest,
    )
    .map_err(|error| unix_recovery_blocked(target, active, error.to_string()))?;
    retire_settled_unix_recovery(target)
}

#[cfg(unix)]
pub(super) fn reconcile_unix_recovery_slots(target: &Path) -> Result<(), PublicationFailure> {
    let Some(active) = select_active_windows_recovery(target)? else {
        return Ok(());
    };
    if !matches!(active.record.state, RecoveryRecordState::Prepared) {
        let backup = recovery_backup_path(target, active.record.backup_id);
        let target_role = observe_file_role(
            target,
            active.record.predecessor_digest,
            active.record.successor_digest,
        );
        let backup_role = observe_file_role(
            &backup,
            active.record.predecessor_digest,
            active.record.successor_digest,
        );
        let (endpoint_is_settled, expected_role) = match active.record.state {
            RecoveryRecordState::Resolved(PublicationResolution::Committed) => (
                matches!(
                    target_role,
                    ObservedFileRole::Replacement | ObservedFileRole::Both
                ),
                ObservedFileRole::Replaced,
            ),
            RecoveryRecordState::Resolved(PublicationResolution::Aborted) => (
                unix_aborted_endpoint_is_settled(target_role, active.record.before_present),
                ObservedFileRole::Replacement,
            ),
            RecoveryRecordState::Prepared => unreachable!(),
        };
        if !endpoint_is_settled
            || (!matches!(backup_role, ObservedFileRole::Missing)
                && backup_role != expected_role
                && !matches!(backup_role, ObservedFileRole::Both))
        {
            return Err(unix_recovery_blocked(
                target,
                &active,
                format!(
                    "resolved native recovery cannot be cleaned against endpoint roles {target_role:?}/{backup_role:?}"
                ),
            ));
        }
        remove_verified_recovery_sidecar(
            &backup,
            expected_role,
            active.record.predecessor_digest,
            active.record.successor_digest,
        )
        .map_err(|error| unix_recovery_blocked(target, &active, error.to_string()))?;
        return retire_settled_unix_recovery(target);
    }

    let backup = recovery_backup_path(target, active.record.backup_id);
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let backup_role = observe_file_role(
        &backup,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    if active.record.before_present {
        if matches!(
            target_role,
            ObservedFileRole::Replaced | ObservedFileRole::Both
        ) && matches!(
            backup_role,
            ObservedFileRole::Replacement | ObservedFileRole::Both | ObservedFileRole::Missing
        ) {
            return settle_prepared_unix_abort(target, &active);
        }
        if matches!(target_role, ObservedFileRole::Replacement)
            && matches!(backup_role, ObservedFileRole::Replaced)
        {
            exchange_paths_checked(
                target,
                &backup,
                active.record.successor_digest,
                active.record.predecessor_digest,
            )?;
            sync_parent_of(target).map_err(|error| {
                unix_recovery_blocked(
                    target,
                    &active,
                    format!("restart rollback could not be synchronized: {error}"),
                )
            })?;
            return settle_prepared_unix_abort(target, &active);
        }
    } else {
        if matches!(target_role, ObservedFileRole::Missing)
            && matches!(
                backup_role,
                ObservedFileRole::Replacement | ObservedFileRole::Missing
            )
        {
            return settle_prepared_unix_abort(target, &active);
        }
        if matches!(target_role, ObservedFileRole::Replacement)
            && matches!(backup_role, ObservedFileRole::Missing)
        {
            move_no_replace_checked(target, &backup, active.record.successor_digest)?;
            sync_parent_of(target).map_err(|error| {
                unix_recovery_blocked(
                    target,
                    &active,
                    format!("new-target restart rollback could not be synchronized: {error}"),
                )
            })?;
            return settle_prepared_unix_abort(target, &active);
        }
    }
    Err(unix_recovery_blocked(
        target,
        &active,
        format!(
            "Prepared native recovery cannot be reconciled with exact endpoint roles {target_role:?}/{backup_role:?}; no pathname was changed"
        ),
    ))
}

#[cfg(unix)]
pub(super) fn prepare_unix_recovery(
    staged: &Path,
    target: &Path,
    predecessor_digest: Option<[u8; 32]>,
    successor_digest: [u8; 32],
) -> Result<(ActiveRecoveryRecord, PathBuf), PublicationFailure> {
    let backup_id = Uuid::new_v4();
    let backup = recovery_backup_path(target, backup_id);
    move_no_replace_checked(staged, &backup, successor_digest)?;
    sync_parent_of(&backup).map_err(|error| PublicationFailure::Uncertain {
        message: format!(
            "native successor sidecar was prepared but its directory entry could not be synchronized: {error}"
        ),
        recovery_paths: recovery_paths(&[staged, &backup, target]),
    })?;
    match install_recovery_bundle(
        target,
        &backup,
        predecessor_digest,
        successor_digest,
        backup_id,
    ) {
        Ok(active) => Ok((active, backup)),
        Err(PublicationFailure::Safe(error)) => {
            move_no_replace_checked(&backup, staged, successor_digest).map_err(|failure| {
                recovery_step_failure(
                    "native recovery installation failed and staging could not be restored",
                    failure,
                    &[staged, &backup, target],
                )
            })?;
            sync_parent_of(staged).map_err(|sync_error| PublicationFailure::Uncertain {
                message: format!(
                    "native recovery installation failed ({error}) and restored staging could not be synchronized: {sync_error}"
                ),
                recovery_paths: recovery_paths(&[staged, target]),
            })?;
            Err(PublicationFailure::Safe(error))
        }
        Err(error @ PublicationFailure::Uncertain { .. }) => Err(error),
    }
}

#[cfg(unix)]
pub(super) fn abort_unpublished_unix_recovery(
    staged: &Path,
    target: &Path,
    active: &ActiveRecoveryRecord,
    backup: &Path,
    successor_digest: [u8; 32],
    publication_error: io::Error,
) -> Result<Option<PathBuf>, PublicationFailure> {
    move_no_replace_checked(backup, staged, successor_digest).map_err(|failure| {
        recovery_step_failure(
            "native publication failed before mutation and staging could not be restored",
            failure,
            &[staged, backup, target, &active.path],
        )
    })?;
    sync_parent_of(staged).map_err(|error| PublicationFailure::Uncertain {
        message: format!(
            "native publication failed ({publication_error}) and restored staging could not be synchronized: {error}"
        ),
        recovery_paths: recovery_paths(&[staged, target, &active.path]),
    })?;
    append_windows_recovery_resolution(active, PublicationResolution::Aborted)?;
    retire_settled_unix_recovery(target)?;
    Err(PublicationFailure::Safe(publication_error))
}

#[cfg(unix)]
pub(super) fn publish_retaining_predecessor(
    staged: &Path,
    target: &Path,
    staged_digest: [u8; 32],
) -> Result<Option<PathBuf>, PublicationFailure> {
    let predecessor_digest = current_digest(target).map_err(PublicationFailure::Safe)?;
    let (active, backup) =
        prepare_unix_recovery(staged, target, predecessor_digest, staged_digest)?;
    let publication = match predecessor_digest {
        Some(predecessor_digest) => {
            exchange_paths_capturing_predecessor(&backup, target, staged_digest, predecessor_digest)
                .map(|()| Some(backup.clone()))
        }
        None => move_no_replace_checked(&backup, target, staged_digest).map(|()| None),
    };
    match publication {
        Ok(displaced) => Ok(displaced),
        Err(PublicationFailure::Safe(error)) => {
            abort_unpublished_unix_recovery(staged, target, &active, &backup, staged_digest, error)
        }
        Err(error @ PublicationFailure::Uncertain { .. }) => Err(error),
    }
}

#[cfg(unix)]
pub(super) fn rollback_retaining_successor(
    target: &Path,
    predecessor: &Path,
) -> Result<PathBuf, PublicationFailure> {
    let successor_digest = current_digest(target)
        .map_err(PublicationFailure::Safe)?
        .ok_or_else(|| {
            PublicationFailure::Safe(io::Error::new(
                io::ErrorKind::NotFound,
                format!("rollback target '{}' no longer exists", target.display()),
            ))
        })?;
    let predecessor_digest = current_digest(predecessor)
        .map_err(PublicationFailure::Safe)?
        .ok_or_else(|| {
            PublicationFailure::Safe(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "rollback predecessor '{}' no longer exists",
                    predecessor.display()
                ),
            ))
        })?;
    exchange_paths_checked(target, predecessor, successor_digest, predecessor_digest)?;
    Ok(predecessor.to_path_buf())
}

#[cfg(unix)]
pub(super) fn exchange_paths_capturing_predecessor(
    staged: &Path,
    target: &Path,
    staged_digest: [u8; 32],
    observed_predecessor_digest: [u8; 32],
) -> Result<(), PublicationFailure> {
    use rustix::fs::{CWD, RenameFlags, renameat_with};

    let raw_result =
        renameat_with(CWD, staged, CWD, target, RenameFlags::EXCHANGE).map_err(io::Error::from);
    let observed_staged = current_digest(staged);
    let observed_target = current_digest(target);

    // The canonical endpoint now contains the staged successor and the swap
    // pathname captured some complete predecessor. It may differ from the
    // earlier observation because a non-cooperating writer won the final race;
    // the caller hashes these captured bytes and rolls them back as a conflict.
    if matches!(observed_target, Ok(Some(digest)) if digest == staged_digest)
        && matches!(observed_staged, Ok(Some(_)))
    {
        return Ok(());
    }

    // The staged inode is still named at its input path, so the exchange did
    // not consume it. Any target change is external and must remain untouched.
    if matches!(observed_staged, Ok(Some(digest)) if digest == staged_digest) {
        return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
            || {
                io::Error::other(format!(
                    "atomic exchange of '{}' and '{}' reported success without consuming staging (previous target digest {observed_predecessor_digest:02x?})",
                    staged.display(),
                    target.display()
                ))
            },
        )));
    }

    let raw_message = match raw_result {
        Ok(()) => "the platform call reported success".to_owned(),
        Err(error) => error.to_string(),
    };
    Err(PublicationFailure::Uncertain {
        message: format!(
            "atomic predecessor capture for '{}' has an unverified postcondition ({raw_message}); swap digest: {observed_staged:?}; target digest: {observed_target:?}",
            target.display()
        ),
        recovery_paths: recovery_paths(&[staged, target]),
    })
}

#[cfg(unix)]
pub(super) fn exchange_paths_checked(
    left: &Path,
    right: &Path,
    left_digest: [u8; 32],
    right_digest: [u8; 32],
) -> Result<(), PublicationFailure> {
    use rustix::fs::{CWD, RenameFlags, renameat_with};

    let raw_result =
        renameat_with(CWD, left, CWD, right, RenameFlags::EXCHANGE).map_err(io::Error::from);
    let observed_left = current_digest(left);
    let observed_right = current_digest(right);
    if matches!(observed_left, Ok(Some(digest)) if digest == right_digest)
        && matches!(observed_right, Ok(Some(digest)) if digest == left_digest)
    {
        return Ok(());
    }
    if matches!(observed_left, Ok(Some(digest)) if digest == left_digest)
        && matches!(observed_right, Ok(Some(digest)) if digest == right_digest)
    {
        return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
            || {
                io::Error::other(format!(
                    "atomic exchange of '{}' and '{}' reported success without changing either path",
                    left.display(),
                    right.display()
                ))
            },
        )));
    }

    let raw_message = match raw_result {
        Ok(()) => "the platform call reported success".to_owned(),
        Err(error) => error.to_string(),
    };
    Err(PublicationFailure::Uncertain {
        message: format!(
            "atomic exchange of '{}' and '{}' has an unverified postcondition ({raw_message}); left digest: {observed_left:?}; right digest: {observed_right:?}",
            left.display(),
            right.display()
        ),
        recovery_paths: recovery_paths(&[left, right]),
    })
}

#[cfg(unix)]
pub(super) fn move_no_replace_raw(from: &Path, to: &Path) -> io::Result<()> {
    use rustix::fs::{CWD, RenameFlags, renameat_with};

    renameat_with(CWD, from, CWD, to, RenameFlags::NOREPLACE).map_err(io::Error::from)
}

#[cfg(unix)]
pub(super) fn sync_parent_directory(parent: &Path) -> io::Result<()> {
    File::open(parent)?.sync_all()
}
