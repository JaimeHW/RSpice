//! The recovery journal, and the Windows publication path that writes it.
//!
//! A publication records what it is about to do in a sidecar slot before it
//! touches the target, and retires that slot only once the new content is
//! verified — so an interrupted write is always explained by a record naming
//! the predecessor and successor digests. The record format and slot
//! selection are shared; the calls that publish and roll back are Windows'.

use super::*;

pub(super) fn read_windows_recovery_record(path: &Path) -> io::Result<Option<RecoveryRecord>> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let file_len = file.metadata()?.len();
    if file_len < RECOVERY_HEADER_LEN as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("recovery bundle '{}' is truncated", path.display()),
        ));
    }
    let mut header = [0_u8; RECOVERY_HEADER_LEN];
    file.read_exact(&mut header)?;
    let allowed_flags = RECOVERY_FLAG_BEFORE_PRESENT | RECOVERY_FLAG_READ_ONLY;
    if header[..16] != RECOVERY_HEADER_MAGIC
        || header[89..96].iter().any(|byte| *byte != 0)
        || header[88] & !allowed_flags != 0
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("recovery bundle '{}' has an invalid header", path.display()),
        ));
    }
    let expected_header_checksum: [u8; 32] = Sha256::new()
        .chain_update(b"rspice-windows-recovery-header\0v1\0")
        .chain_update(&header[..RECOVERY_HEADER_PREFIX_LEN])
        .finalize()
        .into();
    let mut header_checksum = [0_u8; 32];
    header_checksum.copy_from_slice(&header[RECOVERY_HEADER_PREFIX_LEN..RECOVERY_HEADER_LEN]);
    if header_checksum != expected_header_checksum {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' failed header verification",
                path.display()
            ),
        ));
    }

    let mut generation = [0_u8; 8];
    generation.copy_from_slice(&header[16..24]);
    let generation = u64::from_le_bytes(generation);
    if generation == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("recovery bundle '{}' has generation zero", path.display()),
        ));
    }
    let mut transaction = [0_u8; 16];
    transaction.copy_from_slice(&header[24..40]);
    let mut backup_id = [0_u8; 16];
    backup_id.copy_from_slice(&header[40..56]);
    let mut target_binding = [0_u8; 32];
    target_binding.copy_from_slice(&header[56..88]);
    let before_present = header[88] & RECOVERY_FLAG_BEFORE_PRESENT != 0;
    let mut predecessor_len = [0_u8; 8];
    predecessor_len.copy_from_slice(&header[96..104]);
    let predecessor_len = u64::from_le_bytes(predecessor_len);
    let mut successor_len = [0_u8; 8];
    successor_len.copy_from_slice(&header[104..112]);
    let successor_len = u64::from_le_bytes(successor_len);
    if predecessor_len > MAX_HASHED_FILE_BYTES || successor_len > MAX_HASHED_FILE_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' declares an image larger than the supported {MAX_HASHED_FILE_BYTES} byte limit",
                path.display()
            ),
        ));
    }
    let mut predecessor_digest = [0_u8; 32];
    predecessor_digest.copy_from_slice(&header[112..144]);
    let mut successor_digest = [0_u8; 32];
    successor_digest.copy_from_slice(&header[144..176]);
    let empty_digest: [u8; 32] = Sha256::digest([]).into();
    if !before_present && (predecessor_len != 0 || predecessor_digest != empty_digest) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' has bytes for an absent predecessor",
                path.display()
            ),
        ));
    }
    let payload_end = (RECOVERY_HEADER_LEN as u64)
        .checked_add(predecessor_len)
        .and_then(|len| len.checked_add(successor_len))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "recovery size overflow"))?;
    if file_len < payload_end {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("recovery bundle '{}' has a truncated image", path.display()),
        ));
    }
    if hash_exact_section(&mut file, predecessor_len)? != predecessor_digest
        || hash_exact_section(&mut file, successor_len)? != successor_digest
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' failed image verification",
                path.display()
            ),
        ));
    }

    let trailing = file_len - payload_end;
    if trailing > RECOVERY_RESOLUTION_LEN as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' has unexpected trailing data",
                path.display()
            ),
        ));
    }
    let transaction = Uuid::from_bytes(transaction);
    let state = if trailing == RECOVERY_RESOLUTION_LEN as u64 {
        let mut trailer = [0_u8; RECOVERY_RESOLUTION_LEN];
        file.read_exact(&mut trailer)?;
        decode_recovery_resolution(&trailer, generation, transaction, header_checksum)
            .map(RecoveryRecordState::Resolved)
            .unwrap_or(RecoveryRecordState::Prepared)
    } else {
        // A torn append can only lose resolution, never either retained image.
        // Treat it as Prepared and infer the outcome from exact target bytes.
        RecoveryRecordState::Prepared
    };
    Ok(Some(RecoveryRecord {
        generation,
        transaction,
        backup_id: Uuid::from_bytes(backup_id),
        target_binding,
        before_present,
        predecessor_len,
        successor_len,
        predecessor_digest,
        successor_digest,
        predecessor_read_only: header[88] & RECOVERY_FLAG_READ_ONLY != 0,
        header_checksum,
        state,
    }))
}

pub(super) fn seal_file_image(
    source_path: &Path,
    destination: &mut File,
    expected_len: u64,
    expected_digest: [u8; 32],
) -> io::Result<()> {
    let mut source = File::open(source_path)?;
    let mut total = 0_u64;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = source.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        total = total.checked_add(read as u64).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "recovery image size overflow")
        })?;
        if total > MAX_HASHED_FILE_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "recovery image '{}' exceeds the supported {MAX_HASHED_FILE_BYTES} byte limit",
                    source_path.display()
                ),
            ));
        }
        hasher.update(&buffer[..read]);
        destination.write_all(&buffer[..read])?;
    }
    let actual_digest: [u8; 32] = hasher.finalize().into();
    if total != expected_len || actual_digest != expected_digest {
        return Err(io::Error::other(format!(
            "'{}' changed while its recovery image was being sealed",
            source_path.display()
        )));
    }
    Ok(())
}

pub(super) fn target_binding_digest(path: &Path) -> io::Result<[u8; 32]> {
    let absolute = std::path::absolute(path)?;
    let mut hasher = Sha256::new();
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;
        hasher.update(b"rspice-unix-recovery-target\0v1\0");
        hasher.update(absolute.as_os_str().as_bytes());
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;
        hasher.update(b"rspice-windows-recovery-target\0v1\0");
        for unit in absolute.as_os_str().encode_wide() {
            hasher.update(unit.to_le_bytes());
        }
    }
    Ok(hasher.finalize().into())
}

pub(super) fn select_active_windows_recovery(
    target: &Path,
) -> Result<Option<ActiveRecoveryRecord>, PublicationFailure> {
    let binding = target_binding_digest(target).map_err(PublicationFailure::Safe)?;
    let slots = recovery_slot_paths(target);
    let mut valid = Vec::new();
    let mut invalid = Vec::new();

    for (slot_index, path) in slots.iter().enumerate() {
        match read_windows_recovery_record(path) {
            Ok(Some(record)) if record.target_binding == binding => {
                valid.push(ActiveRecoveryRecord {
                    slot_index,
                    path: path.clone(),
                    record,
                });
            }
            Ok(Some(_)) => invalid.push((
                path.clone(),
                "target binding does not match the canonical pathname".to_owned(),
            )),
            Ok(None) => {}
            Err(error) => invalid.push((path.clone(), error.to_string())),
        }
    }

    if !invalid.is_empty() {
        return Err(PublicationFailure::Uncertain {
            message: format!(
                "foreign or corrupted recovery evidence exists for '{}': {invalid:?}",
                target.display()
            ),
            recovery_paths: invalid.into_iter().map(|(path, _)| path).collect(),
        });
    }

    match valid.len() {
        0 => Ok(None),
        1 => Ok(valid.pop()),
        2 => {
            let right = valid.pop().expect("two records have a right member");
            let left = valid.pop().expect("two records have a left member");
            if left.record.generation == right.record.generation {
                if left.record == right.record {
                    return Ok(Some(left));
                }
                return Err(PublicationFailure::Uncertain {
                    message: format!(
                        "recovery slots for '{}' contain ambiguous generation {} transactions",
                        target.display(),
                        left.record.generation
                    ),
                    recovery_paths: vec![left.path, right.path],
                });
            }
            Ok(Some(if left.record.generation > right.record.generation {
                left
            } else {
                right
            }))
        }
        _ => unreachable!("there are exactly two recovery slots"),
    }
}

#[cfg(windows)]
pub(super) fn install_windows_recovery_bundle(
    target: &Path,
    staged: &Path,
    predecessor_digest: [u8; 32],
    successor_digest: [u8; 32],
) -> Result<ActiveRecoveryRecord, PublicationFailure> {
    install_recovery_bundle(
        target,
        staged,
        Some(predecessor_digest),
        successor_digest,
        Uuid::new_v4(),
    )
}

pub(super) fn install_recovery_bundle(
    target: &Path,
    staged: &Path,
    predecessor_digest: Option<[u8; 32]>,
    successor_digest: [u8; 32],
    backup_id: Uuid,
) -> Result<ActiveRecoveryRecord, PublicationFailure> {
    let active = select_active_windows_recovery(target)?;
    let generation =
        active
            .as_ref()
            .map_or(Ok(1), |active| {
                active.record.generation.checked_add(1).ok_or_else(|| {
                    io::Error::other("native recovery generation counter is exhausted")
                })
            })
            .map_err(PublicationFailure::Safe)?;
    let slot_index = active.as_ref().map_or(0, |active| 1 - active.slot_index);
    let path = recovery_slot_paths(target)[slot_index].clone();
    let transaction = Uuid::new_v4();
    let target_binding = target_binding_digest(target).map_err(PublicationFailure::Safe)?;
    let successor_metadata = std::fs::metadata(staged).map_err(PublicationFailure::Safe)?;
    let predecessor_metadata = match predecessor_digest {
        Some(_) => Some(std::fs::metadata(target).map_err(PublicationFailure::Safe)?),
        None => {
            if !matches!(target.try_exists(), Ok(false)) {
                return Err(PublicationFailure::Safe(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "recovery expected missing destination '{}'",
                        target.display()
                    ),
                )));
            }
            None
        }
    };
    let predecessor_len = predecessor_metadata
        .as_ref()
        .map_or(0, std::fs::Metadata::len);
    let successor_len = successor_metadata.len();
    if predecessor_len > MAX_HASHED_FILE_BYTES || successor_len > MAX_HASHED_FILE_BYTES {
        return Err(PublicationFailure::Safe(io::Error::new(
            io::ErrorKind::FileTooLarge,
            format!(
                "recovery images exceed the supported {MAX_HASHED_FILE_BYTES} byte per-image limit"
            ),
        )));
    }
    let predecessor_read_only = predecessor_metadata
        .as_ref()
        .is_some_and(|metadata| metadata.permissions().readonly());
    let predecessor_digest = predecessor_digest.unwrap_or_else(|| Sha256::digest([]).into());
    let before_present = predecessor_metadata.is_some();
    let header = encode_recovery_header(RecoveryHeaderInput {
        generation,
        transaction,
        backup_id,
        target_binding,
        before_present,
        predecessor_len,
        successor_len,
        predecessor_digest,
        successor_digest,
        predecessor_read_only,
    });
    let mut header_checksum = [0_u8; 32];
    header_checksum.copy_from_slice(&header[RECOVERY_HEADER_PREFIX_LEN..]);
    let expected = RecoveryRecord {
        generation,
        transaction,
        backup_id,
        target_binding,
        before_present,
        predecessor_len,
        successor_len,
        predecessor_digest,
        successor_digest,
        predecessor_read_only,
        header_checksum,
        state: RecoveryRecordState::Prepared,
    };

    let (temporary, mut file) = create_unique_sibling(&path).map_err(PublicationFailure::Safe)?;
    let sealed = (|| -> io::Result<()> {
        file.write_all(&header)?;
        if before_present {
            seal_file_image(target, &mut file, predecessor_len, predecessor_digest)?;
        }
        seal_file_image(staged, &mut file, successor_len, successor_digest)?;
        file.flush()?;
        file.sync_all()
    })();
    drop(file);
    if let Err(error) = sealed {
        remove_unpublished_temp(&temporary);
        return Err(PublicationFailure::Safe(error));
    }
    match read_windows_recovery_record(&temporary) {
        Ok(Some(record)) if record == expected => {}
        observation => {
            remove_unpublished_temp(&temporary);
            return Err(PublicationFailure::Safe(io::Error::other(format!(
                "new recovery bundle failed reopen verification: {observation:?}"
            ))));
        }
    }

    // A recovery slot is an ownership record, never an overwrite target. If a
    // foreign or raced pathname exists, preserve both files and fail closed.
    let raw_result = move_no_replace_raw(&temporary, &path);
    let installed = read_windows_recovery_record(&path);
    if matches!(installed, Ok(Some(ref record)) if record == &expected) {
        sync_parent_of(&path).map_err(|error| PublicationFailure::Uncertain {
            message: format!(
                "recovery generation {generation} was installed but its directory entry could not be synchronized: {error}"
            ),
            recovery_paths: vec![path.clone()],
        })?;
        return Ok(ActiveRecoveryRecord {
            slot_index,
            path,
            record: expected,
        });
    }
    if matches!(read_windows_recovery_record(&temporary), Ok(Some(ref record)) if record == &expected)
    {
        remove_unpublished_temp(&temporary);
        return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
            || {
                io::Error::other(
                    "recovery-slot move reported success without installing the bundle",
                )
            },
        )));
    }
    let raw_message = raw_result
        .map(|()| "the platform call reported success".to_owned())
        .unwrap_or_else(|error| error.to_string());
    Err(PublicationFailure::Uncertain {
        message: format!(
            "recovery generation {generation} installation has an unverified postcondition ({raw_message}); installed observation: {installed:?}"
        ),
        recovery_paths: recovery_paths(&[&temporary, &path]),
    })
}

pub(super) fn append_windows_recovery_resolution(
    active: &ActiveRecoveryRecord,
    resolution: PublicationResolution,
) -> Result<(), PublicationFailure> {
    let observed = read_windows_recovery_record(&active.path).map_err(|error| {
        PublicationFailure::Uncertain {
            message: format!("active recovery bundle could not be reopened: {error}"),
            recovery_paths: vec![active.path.clone()],
        }
    })?;
    let Some(mut observed) = observed else {
        return Err(PublicationFailure::Uncertain {
            message: "active recovery bundle disappeared before resolution".to_owned(),
            recovery_paths: vec![active.path.clone()],
        });
    };
    let observed_state = observed.state;
    observed.state = RecoveryRecordState::Prepared;
    let mut expected_identity = active.record.clone();
    expected_identity.state = RecoveryRecordState::Prepared;
    if observed != expected_identity {
        return Err(PublicationFailure::Uncertain {
            message: "active recovery generation changed before resolution".to_owned(),
            recovery_paths: vec![active.path.clone()],
        });
    }
    match observed_state {
        RecoveryRecordState::Resolved(existing) if existing == resolution => return Ok(()),
        RecoveryRecordState::Resolved(existing) => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "recovery transaction is already resolved as {existing:?}, not {resolution:?}"
                ),
                recovery_paths: vec![active.path.clone()],
            });
        }
        RecoveryRecordState::Prepared => {}
    }
    let payload_end = (RECOVERY_HEADER_LEN as u64)
        .checked_add(active.record.predecessor_len)
        .and_then(|len| len.checked_add(active.record.successor_len))
        .ok_or_else(|| PublicationFailure::Uncertain {
            message: "recovery bundle payload size overflow".to_owned(),
            recovery_paths: vec![active.path.clone()],
        })?;
    let trailer = encode_recovery_resolution(&active.record, resolution);
    let write_result = (|| -> io::Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&active.path)?;
        file.set_len(payload_end)?;
        file.seek(std::io::SeekFrom::Start(payload_end))?;
        file.write_all(&trailer)?;
        file.flush()?;
        file.sync_all()
    })();
    if let Err(error) = write_result {
        return Err(PublicationFailure::Uncertain {
            message: format!("recovery resolution could not be synchronized: {error}"),
            recovery_paths: vec![active.path.clone()],
        });
    }
    let mut expected_resolved = active.record.clone();
    expected_resolved.state = RecoveryRecordState::Resolved(resolution);
    match read_windows_recovery_record(&active.path) {
        Ok(Some(record)) if record == expected_resolved => Ok(()),
        observation => Err(PublicationFailure::Uncertain {
            message: format!(
                "recovery resolution did not survive reopen verification: {observation:?}"
            ),
            recovery_paths: vec![active.path.clone()],
        }),
    }
}

#[cfg(windows)]
pub(super) fn recovery_sidecar_roles(
    target: &Path,
    record: &RecoveryRecord,
) -> (PathBuf, ObservedFileRole, PathBuf, ObservedFileRole) {
    let backup = recovery_backup_path(target, record.backup_id);
    let rollback = recovery_rollback_path(target, record.backup_id);
    let backup_role =
        observe_file_role(&backup, record.predecessor_digest, record.successor_digest);
    let rollback_role = observe_file_role(
        &rollback,
        record.predecessor_digest,
        record.successor_digest,
    );
    (backup, backup_role, rollback, rollback_role)
}

#[cfg(windows)]
pub(super) fn exact_or_missing(role: ObservedFileRole, exact: ObservedFileRole) -> bool {
    matches!(role, ObservedFileRole::Missing | ObservedFileRole::Both) || role == exact
}

#[cfg(windows)]
pub(super) fn recovery_blocked(
    target: &Path,
    active: &ActiveRecoveryRecord,
    message: impl Into<String>,
) -> PublicationFailure {
    let slots = recovery_slot_paths(target);
    let backup = recovery_backup_path(target, active.record.backup_id);
    let rollback = recovery_rollback_path(target, active.record.backup_id);
    PublicationFailure::Uncertain {
        message: message.into(),
        recovery_paths: recovery_paths(&[target, &slots[0], &slots[1], &backup, &rollback]),
    }
}

#[cfg(windows)]
pub(super) fn resolve_windows_recovery_slot(
    target: &Path,
    successor_digest: [u8; 32],
    resolution: PublicationResolution,
) -> Result<(), PublicationFailure> {
    let active =
        select_active_windows_recovery(target)?.ok_or_else(|| PublicationFailure::Uncertain {
            message: "publication has no durable recovery generation".to_owned(),
            recovery_paths: vec![target.to_path_buf()],
        })?;
    if active.record.successor_digest != successor_digest {
        return Err(recovery_blocked(
            target,
            &active,
            "active recovery generation does not describe the staged successor",
        ));
    }
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let (_backup, backup_role, _rollback, rollback_role) =
        recovery_sidecar_roles(target, &active.record);
    let endpoint_is_valid = match resolution {
        PublicationResolution::Committed => {
            matches!(
                target_role,
                ObservedFileRole::Replacement | ObservedFileRole::Both
            ) && exact_or_missing(backup_role, ObservedFileRole::Replaced)
                && matches!(rollback_role, ObservedFileRole::Missing)
        }
        PublicationResolution::Aborted => {
            (matches!(
                target_role,
                ObservedFileRole::Replaced | ObservedFileRole::Both
            ) || (!active.record.before_present
                && matches!(target_role, ObservedFileRole::Missing)))
                && exact_or_missing(backup_role, ObservedFileRole::Replaced)
                && exact_or_missing(rollback_role, ObservedFileRole::Replacement)
        }
    };
    if !endpoint_is_valid {
        return Err(recovery_blocked(
            target,
            &active,
            format!(
                "recovery resolution {resolution:?} is inconsistent with target {target_role:?}, backup {backup_role:?}, and rollback evidence {rollback_role:?}"
            ),
        ));
    }
    append_windows_recovery_resolution(&active, resolution)
}

pub(super) fn remove_verified_recovery_sidecar(
    path: &Path,
    expected_role: ObservedFileRole,
    predecessor_digest: [u8; 32],
    successor_digest: [u8; 32],
) -> io::Result<()> {
    let role = observe_file_role(path, predecessor_digest, successor_digest);
    if matches!(role, ObservedFileRole::Missing) {
        return Ok(());
    }
    if role != expected_role && !matches!(role, ObservedFileRole::Both) {
        return Err(io::Error::other(format!(
            "recovery sidecar '{}' changed before cleanup ({role:?})",
            path.display()
        )));
    }
    std::fs::remove_file(path)?;
    sync_parent_of(path)
}

#[cfg(windows)]
pub(super) fn reconcile_windows_recovery_slots(target: &Path) -> Result<(), PublicationFailure> {
    let Some(active) = select_active_windows_recovery(target)? else {
        return retire_settled_windows_recovery(target);
    };
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let (backup, backup_role, rollback, rollback_role) =
        recovery_sidecar_roles(target, &active.record);
    let action = recovery_reconciliation(
        active.record.state,
        target_role,
        active.record.before_present,
    );
    match action {
        RecoveryReconciliation::PreserveAndBlock => Err(recovery_blocked(
            target,
            &active,
            format!(
                "recovery generation {} ({:?}) cannot be reconciled with target state {target_role:?}; no pathname was changed",
                active.record.generation, active.record.state
            ),
        )),
        RecoveryReconciliation::Resolve(PublicationResolution::Aborted) => {
            if !exact_or_missing(backup_role, ObservedFileRole::Replaced)
                || !exact_or_missing(rollback_role, ObservedFileRole::Replacement)
            {
                return Err(recovery_blocked(
                    target,
                    &active,
                    format!(
                        "prepared abort has foreign backup evidence ({backup_role:?}, {rollback_role:?})"
                    ),
                ));
            }
            append_windows_recovery_resolution(&active, PublicationResolution::Aborted)?;
            remove_verified_recovery_sidecar(
                &backup,
                ObservedFileRole::Replaced,
                active.record.predecessor_digest,
                active.record.successor_digest,
            )
            .and_then(|()| {
                remove_verified_recovery_sidecar(
                    &rollback,
                    ObservedFileRole::Replacement,
                    active.record.predecessor_digest,
                    active.record.successor_digest,
                )
            })
            .map_err(|error| recovery_blocked(target, &active, error.to_string()))?;
            retire_settled_windows_recovery(target)
        }
        RecoveryReconciliation::Resolve(PublicationResolution::Committed) => {
            unreachable!("Prepared recovery is never inferred committed")
        }
        RecoveryReconciliation::AcceptResolved => {
            let evidence_is_valid = match active.record.state {
                RecoveryRecordState::Resolved(PublicationResolution::Committed) => {
                    exact_or_missing(backup_role, ObservedFileRole::Replaced)
                        && matches!(rollback_role, ObservedFileRole::Missing)
                }
                RecoveryRecordState::Resolved(PublicationResolution::Aborted) => {
                    exact_or_missing(backup_role, ObservedFileRole::Replaced)
                        && exact_or_missing(rollback_role, ObservedFileRole::Replacement)
                }
                RecoveryRecordState::Prepared => false,
            };
            if !evidence_is_valid {
                return Err(recovery_blocked(
                    target,
                    &active,
                    format!(
                        "resolved recovery has foreign backup evidence ({backup_role:?}, {rollback_role:?})"
                    ),
                ));
            }
            remove_verified_recovery_sidecar(
                &backup,
                ObservedFileRole::Replaced,
                active.record.predecessor_digest,
                active.record.successor_digest,
            )
            .and_then(|()| {
                remove_verified_recovery_sidecar(
                    &rollback,
                    ObservedFileRole::Replacement,
                    active.record.predecessor_digest,
                    active.record.successor_digest,
                )
            })
            .map_err(|error| recovery_blocked(target, &active, error.to_string()))?;
            retire_settled_windows_recovery(target)
        }
    }
}

#[cfg(windows)]
pub(super) fn remove_windows_retirement_tombstone(
    target: &Path,
    path: &Path,
    expected: Option<&RecoveryRecord>,
) -> Result<(), PublicationFailure> {
    let record = match read_windows_recovery_record(path) {
        Ok(Some(record)) => record,
        Ok(None) => return Ok(()),
        Err(error) => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "retirement tombstone '{}' is foreign or corrupted: {error}",
                    path.display()
                ),
                recovery_paths: vec![path.to_path_buf()],
            });
        }
    };
    let binding = target_binding_digest(target).map_err(PublicationFailure::Safe)?;
    if record.target_binding != binding || expected.is_some_and(|expected| expected != &record) {
        return Err(PublicationFailure::Uncertain {
            message: format!(
                "retirement tombstone '{}' is not owned by the recovery generation being retired",
                path.display()
            ),
            recovery_paths: vec![path.to_path_buf()],
        });
    }
    let target_role = observe_file_role(target, record.predecessor_digest, record.successor_digest);
    let (backup, backup_role, rollback, rollback_role) = recovery_sidecar_roles(target, &record);
    if recovery_reconciliation(record.state, target_role, record.before_present)
        != RecoveryReconciliation::AcceptResolved
        || !matches!(backup_role, ObservedFileRole::Missing)
        || !matches!(rollback_role, ObservedFileRole::Missing)
    {
        return Err(PublicationFailure::Uncertain {
            message: format!(
                "retirement tombstone '{}' is not settled against the exact canonical endpoint ({target_role:?}, {backup_role:?}, {rollback_role:?})",
                path.display()
            ),
            recovery_paths: recovery_paths(&[path, target, &backup, &rollback]),
        });
    }
    match std::fs::remove_file(path) {
        Ok(()) => sync_parent_of(path).map_err(PublicationFailure::Safe),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(PublicationFailure::Safe(error)),
    }
}

#[cfg(windows)]
pub(super) fn retire_windows_recovery_slot(
    target: &Path,
    slot: &Path,
    expected: &RecoveryRecord,
) -> Result<(), PublicationFailure> {
    let tombstone = recovery_retired_path(slot);
    if matches!(slot.try_exists(), Ok(false)) {
        return remove_windows_retirement_tombstone(target, &tombstone, Some(expected));
    }
    match read_windows_recovery_record(slot) {
        Ok(Some(record)) if &record == expected => {}
        observation => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "recovery slot '{}' changed before authenticated retirement: {observation:?}",
                    slot.display()
                ),
                recovery_paths: recovery_paths(&[slot, &tombstone]),
            });
        }
    }
    remove_windows_retirement_tombstone(target, &tombstone, None)?;

    let raw_result = move_no_replace_raw(slot, &tombstone);
    match slot.try_exists() {
        Ok(false) => {}
        Ok(true) => {
            return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
                || {
                    io::Error::other(format!(
                        "recovery retirement move reported success without consuming '{}'",
                        slot.display()
                    ))
                },
            )));
        }
        Err(error) => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "could not verify removal of recovery slot '{}': {error}",
                    slot.display()
                ),
                recovery_paths: recovery_paths(&[slot, &tombstone]),
            });
        }
    }

    match read_windows_recovery_record(&tombstone) {
        Ok(Some(record)) if &record == expected => {}
        Ok(None) => {
            return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
                || {
                    io::Error::other(format!(
                        "resolved recovery slot '{}' disappeared during retirement",
                        slot.display()
                    ))
                },
            )));
        }
        observation => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "resolved recovery slot changed while being retired: {observation:?}"
                ),
                recovery_paths: recovery_paths(&[&tombstone]),
            });
        }
    }

    remove_windows_retirement_tombstone(target, &tombstone, Some(expected))
}

#[cfg(windows)]
pub(super) fn retire_settled_windows_recovery(target: &Path) -> Result<(), PublicationFailure> {
    let Some(active) = select_active_windows_recovery(target)? else {
        for slot in recovery_slot_paths(target) {
            remove_windows_retirement_tombstone(target, &recovery_retired_path(&slot), None)?;
        }
        return Ok(());
    };
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    if recovery_reconciliation(
        active.record.state,
        target_role,
        active.record.before_present,
    ) != RecoveryReconciliation::AcceptResolved
    {
        return Err(recovery_blocked(
            target,
            &active,
            "Prepared or unsettled recovery evidence cannot be retired",
        ));
    }
    let (_backup, backup_role, _rollback, rollback_role) =
        recovery_sidecar_roles(target, &active.record);
    if !matches!(backup_role, ObservedFileRole::Missing)
        || !matches!(rollback_role, ObservedFileRole::Missing)
    {
        return Err(recovery_blocked(
            target,
            &active,
            format!(
                "resolved recovery cannot be retired before sidecar cleanup ({backup_role:?}, {rollback_role:?})"
            ),
        ));
    }

    let slots = recovery_slot_paths(target);
    let inactive_index = 1 - active.slot_index;
    match read_windows_recovery_record(&slots[inactive_index]) {
        Ok(Some(inactive)) => {
            retire_windows_recovery_slot(target, &slots[inactive_index], &inactive)?;
            sync_parent_of(&slots[inactive_index]).map_err(PublicationFailure::Safe)?;
        }
        Ok(None) => remove_windows_retirement_tombstone(
            target,
            &recovery_retired_path(&slots[inactive_index]),
            None,
        )?,
        Err(error) => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "inactive recovery slot '{}' is foreign or corrupted: {error}",
                    slots[inactive_index].display()
                ),
                recovery_paths: vec![slots[inactive_index].clone()],
            });
        }
    }

    let verified =
        select_active_windows_recovery(target)?.ok_or_else(|| PublicationFailure::Uncertain {
            message: "newest recovery generation disappeared before final retirement".to_owned(),
            recovery_paths: recovery_paths(&[target, &slots[active.slot_index]]),
        })?;
    if verified.path != active.path || verified.record != active.record {
        return Err(recovery_blocked(
            target,
            &verified,
            "newest recovery generation changed during retirement",
        ));
    }
    retire_windows_recovery_slot(target, &slots[active.slot_index], &active.record)?;
    sync_parent_of(&slots[active.slot_index]).map_err(PublicationFailure::Safe)
}

#[cfg(windows)]
pub(super) fn publish_retaining_predecessor(
    staged: &Path,
    target: &Path,
    staged_digest: [u8; 32],
) -> Result<Option<PathBuf>, PublicationFailure> {
    let predecessor_digest = current_digest(target).map_err(PublicationFailure::Safe)?;
    let Some(predecessor_digest) = predecessor_digest else {
        move_no_replace_checked(staged, target, staged_digest)?;
        return Ok(None);
    };

    let recovery =
        install_windows_recovery_bundle(target, staged, predecessor_digest, staged_digest)?;
    let displaced = recovery_backup_path(target, recovery.record.backup_id);
    match replace_file_checked(
        target,
        staged,
        &displaced,
        predecessor_digest,
        staged_digest,
    ) {
        Ok(()) => match current_digest(&displaced) {
            Ok(Some(digest)) if digest == predecessor_digest => Ok(Some(displaced)),
            observation => Err(recovery_blocked(
                target,
                &recovery,
                format!(
                    "ReplaceFileW successor is visible, but its actual backup does not match the bundled predecessor: {observation:?}"
                ),
            )),
        },
        Err(PublicationFailure::Safe(error)) => {
            resolve_windows_recovery_slot(
                target,
                staged_digest,
                PublicationResolution::Aborted,
            )
            .map_err(|failure| {
                recovery_step_failure(
                    "ReplaceFileW was rolled back, but the Prepared recovery generation could not be durably aborted",
                    failure,
                    &[target, staged, &displaced, &recovery.path],
                )
            })?;
            Err(PublicationFailure::Safe(error))
        }
        Err(PublicationFailure::Uncertain {
            message,
            recovery_paths: paths,
        }) => Err(PublicationFailure::Uncertain {
            message,
            recovery_paths: merge_recovery_paths(paths, [recovery.path, displaced]),
        }),
    }
}

#[cfg(windows)]
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
    let active =
        select_active_windows_recovery(target)?.ok_or_else(|| PublicationFailure::Uncertain {
            message: "rollback has no active recovery generation".to_owned(),
            recovery_paths: vec![target.to_path_buf(), predecessor.to_path_buf()],
        })?;
    let recorded_predecessor = recovery_backup_path(target, active.record.backup_id);
    if predecessor != recorded_predecessor {
        return Err(recovery_blocked(
            target,
            &active,
            format!(
                "rollback predecessor '{}' is not the active transaction backup '{}'",
                predecessor.display(),
                recorded_predecessor.display()
            ),
        ));
    }
    let successor = recovery_rollback_path(target, active.record.backup_id);
    if !matches!(successor.try_exists(), Ok(false)) {
        return Err(recovery_blocked(
            target,
            &active,
            "rollback successor path already exists",
        ));
    }
    replace_file_checked(
        target,
        predecessor,
        &successor,
        successor_digest,
        predecessor_digest,
    )?;
    Ok(successor)
}

#[cfg(windows)]
pub(super) fn replace_file_checked(
    replaced: &Path,
    replacement: &Path,
    backup: &Path,
    replaced_digest: [u8; 32],
    replacement_digest: [u8; 32],
) -> Result<(), PublicationFailure> {
    use windows_sys::Win32::Storage::FileSystem::{REPLACEFILE_IGNORE_ACL_ERRORS, ReplaceFileW};

    // ReplaceFileW requests WRITE_DAC on the destination while merging its
    // ACL. Managed project locations may permit ordinary replacement while
    // withholding that privilege. The staged sibling must already have the
    // exact owner/group/DACL contract before we ignore only that redundant
    // merge step; a custom or weaker descriptor fails closed.
    verify_windows_file_security_equivalence(replaced, replacement)
        .map_err(PublicationFailure::Safe)?;
    let replaced_w = wide(replaced);
    let replacement_w = wide(replacement);
    let backup_w = wide(backup);
    // REPLACEFILE_WRITE_THROUGH is explicitly unsupported. Durability is
    // established by synchronizing both staged bytes and the parent metadata
    // boundary around this call instead of relying on that inert flag.
    let result = unsafe {
        ReplaceFileW(
            replaced_w.as_ptr(),
            replacement_w.as_ptr(),
            backup_w.as_ptr(),
            REPLACEFILE_IGNORE_ACL_ERRORS,
            std::ptr::null(),
            std::ptr::null(),
        )
    };
    let reported_success = result != 0;
    let call_error = if reported_success {
        io::Error::other(format!(
            "ReplaceFileW reported success without its verified postcondition for '{}'",
            replaced.display()
        ))
    } else {
        io::Error::last_os_error()
    };
    let observation = observe_replace(
        replaced,
        replacement,
        backup,
        replaced_digest,
        replacement_digest,
    );

    if reported_success && observation.success_is_complete() {
        return Ok(());
    }

    match replacement_failure_recovery(observation) {
        ReplaceFailureRecovery::InputIntact => Err(PublicationFailure::Safe(call_error)),
        ReplaceFailureRecovery::RestoreBackupIntoReplaced => {
            move_no_replace_checked(backup, replaced, replaced_digest).map_err(|failure| {
                recovery_step_failure(
                    "ReplaceFileW left the replaced path missing and its backup could not be restored",
                    failure,
                    &[replaced, replacement, backup],
                )
            })?;
            let restored = observe_replace(
                replaced,
                replacement,
                backup,
                replaced_digest,
                replacement_digest,
            );
            if restored.input_is_intact() {
                Err(PublicationFailure::Safe(call_error))
            } else {
                Err(PublicationFailure::Uncertain {
                    message: format!(
                        "ReplaceFileW backup restoration produced an unexpected state: {restored:?}"
                    ),
                    recovery_paths: recovery_paths(&[replaced, replacement, backup]),
                })
            }
        }
        ReplaceFailureRecovery::RestoreCompletedReplacement => {
            move_no_replace_checked(replaced, replacement, replacement_digest).map_err(
                |failure| {
                    recovery_step_failure(
                        "ReplaceFileW failed after completing replacement and the successor could not be returned to staging",
                        failure,
                        &[replaced, replacement, backup],
                    )
                },
            )?;
            move_no_replace_checked(backup, replaced, replaced_digest).map_err(|failure| {
                recovery_step_failure(
                    "ReplaceFileW completed despite reporting failure and its predecessor backup could not be restored",
                    failure,
                    &[replaced, replacement, backup],
                )
            })?;
            let restored = observe_replace(
                replaced,
                replacement,
                backup,
                replaced_digest,
                replacement_digest,
            );
            if restored.input_is_intact() {
                Err(PublicationFailure::Safe(call_error))
            } else {
                Err(PublicationFailure::Uncertain {
                    message: format!(
                        "ReplaceFileW completed despite reporting failure and rollback produced an unexpected state: {restored:?}"
                    ),
                    recovery_paths: recovery_paths(&[replaced, replacement, backup]),
                })
            }
        }
        ReplaceFailureRecovery::Uncertain => Err(PublicationFailure::Uncertain {
            message: format!(
                "ReplaceFileW for '{}' returned an unverified state after {call_error}: {observation:?}",
                replaced.display()
            ),
            recovery_paths: recovery_paths(&[replaced, replacement, backup]),
        }),
    }
}

#[cfg(windows)]
pub(super) fn read_windows_file_security(path: &Path) -> io::Result<Vec<u8>> {
    use windows_sys::Win32::Security::{
        DACL_SECURITY_INFORMATION, GROUP_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
    };

    const OBSERVED_CONTRACT: u32 =
        OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION;

    use windows_sys::Win32::Security::GetFileSecurityW;

    let path = wide(path);
    let mut needed = 0_u32;
    unsafe {
        GetFileSecurityW(
            path.as_ptr(),
            OBSERVED_CONTRACT,
            std::ptr::null_mut(),
            0,
            &mut needed,
        )
    };
    if needed == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut descriptor = vec![0_u8; needed as usize];
    let read = unsafe {
        GetFileSecurityW(
            path.as_ptr(),
            OBSERVED_CONTRACT,
            descriptor.as_mut_ptr().cast(),
            needed,
            &mut needed,
        )
    };
    if read == 0 {
        return Err(io::Error::last_os_error());
    }
    descriptor.truncate(needed as usize);
    Ok(descriptor)
}

#[cfg(windows)]
pub(super) fn copy_windows_file_security(source: &Path, destination: &Path) -> io::Result<()> {
    use windows_sys::Win32::Security::{
        DACL_SECURITY_INFORMATION, GROUP_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
        SetFileSecurityW,
    };

    const OBSERVED_CONTRACT: u32 =
        OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION;
    let mut descriptor = read_windows_file_security(source)?;
    let destination_wide = wide(destination);
    let written = unsafe {
        SetFileSecurityW(
            destination_wide.as_ptr(),
            OBSERVED_CONTRACT,
            descriptor.as_mut_ptr().cast(),
        )
    };
    if written == 0 {
        return Err(io::Error::last_os_error());
    }
    verify_windows_file_security_equivalence(source, destination)
}

/// The SDDL text for the exact owner/group/DACL contract this module stages
/// onto a successor and refuses to publish without.
#[cfg(windows)]
pub(super) fn windows_security_sddl(path: &Path) -> io::Result<String> {
    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Authorization::{
        ConvertSecurityDescriptorToStringSecurityDescriptorW, SDDL_REVISION_1,
    };
    use windows_sys::Win32::Security::{
        DACL_SECURITY_INFORMATION, GROUP_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
    };

    const OBSERVED_CONTRACT: u32 =
        OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION;

    struct LocalSecurityString(*mut u16);
    impl Drop for LocalSecurityString {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    LocalFree(self.0.cast());
                }
            }
        }
    }

    let mut descriptor = read_windows_file_security(path)?;
    let mut text = std::ptr::null_mut();
    let mut text_len = 0_u32;
    let converted = unsafe {
        ConvertSecurityDescriptorToStringSecurityDescriptorW(
            descriptor.as_mut_ptr().cast(),
            SDDL_REVISION_1,
            OBSERVED_CONTRACT,
            &mut text,
            &mut text_len,
        )
    };
    if converted == 0 {
        return Err(io::Error::last_os_error());
    }
    let allocation = LocalSecurityString(text);
    if allocation.0.is_null() {
        return Err(io::Error::other(
            "Windows returned a null canonical security descriptor",
        ));
    }
    let text_len = usize::try_from(text_len)
        .map_err(|_| io::Error::other("Windows security descriptor length overflow"))?;
    let mut sddl =
        String::from_utf16(unsafe { std::slice::from_raw_parts(allocation.0, text_len) })
            .map_err(|_| io::Error::other("Windows returned a malformed security descriptor"))?;
    while sddl.ends_with('\0') {
        sddl.pop();
    }
    Ok(sddl)
}

/// Drop the DACL provenance bits from a canonical descriptor.
///
/// `SetFileSecurityW` preserves every inherited ACE but Windows may not copy
/// the `SE_DACL_AUTO_INHERITED` / `SE_DACL_AUTO_INHERIT_REQ` bookkeeping bits
/// onto the sibling. Those bits record how the ACL was produced; they do not
/// change the owner, group, ACE order, inheritance flags, or effective access
/// that `ReplaceFileW` must preserve.
#[cfg(windows)]
pub(super) fn without_dacl_provenance_flags(mut canonical: String) -> String {
    if let Some(dacl_start) = canonical.find("D:") {
        let flags_start = dacl_start + 2;
        // A DACL that states no ACEs still states its control flags, so the
        // run of flags ends at the first ACE or at the end of the descriptor.
        // Only owner, group, and DACL are ever requested, so nothing but ACEs
        // can follow.
        let flags_end = canonical[flags_start..]
            .find('(')
            .map_or(canonical.len(), |offset| flags_start + offset);
        let flags = canonical[flags_start..flags_end]
            .replace("AI", "")
            .replace("AR", "");
        canonical.replace_range(flags_start..flags_end, &flags);
    }
    canonical
}

#[cfg(windows)]
pub(super) fn verify_windows_file_security_equivalence(
    source: &Path,
    destination: &Path,
) -> io::Result<()> {
    fn canonical(path: &Path) -> io::Result<String> {
        windows_security_sddl(path).map(without_dacl_provenance_flags)
    }

    let expected = canonical(source)?;
    let actual = canonical(destination)?;
    if actual != expected {
        return Err(io::Error::other(format!(
            "staged security descriptor for '{}' ({actual}) does not match '{}' ({expected})",
            destination.display(),
            source.display()
        )));
    }
    Ok(())
}

#[cfg(windows)]
pub(super) fn move_no_replace_raw(from: &Path, to: &Path) -> io::Result<()> {
    use windows_sys::Win32::Storage::FileSystem::{MOVEFILE_WRITE_THROUGH, MoveFileExW};
    let from = wide(from);
    let to = wide(to);
    let moved = unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), MOVEFILE_WRITE_THROUGH) };
    if moved == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
pub(super) fn wide(path: &Path) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt as _;
    path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
