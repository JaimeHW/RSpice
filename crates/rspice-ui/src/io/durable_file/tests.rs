//! Tests for atomic publication and crash recovery.
//!
//! The suite asserts what must survive an interrupted write: only the target
//! and its lease are left behind, no recovery artifacts leak, and an existing
//! file's mode is preserved across replacement. These are the invariants a
//! partially-completed publication would break.

use super::*;

#[test]
fn failed_staging_preserves_target_and_removes_unique_temp() {
    let root = unique_temp_dir("failed-stage");
    let target = root.join("state.json");
    std::fs::write(&target, b"durable predecessor").expect("write predecessor");

    let result = atomic_write_with::<io::Error>(&target, |file| {
        file.write_all(b"partial successor")?;
        Err(io::Error::other("injected serializer failure"))
    });

    assert!(result.is_err());
    assert_eq!(
        std::fs::read(&target).expect("read predecessor"),
        b"durable predecessor"
    );
    assert_only_target_and_lease(&root, &target);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn successful_publication_replaces_target_without_fixed_temp_names() {
    let root = unique_temp_dir("success");
    let target = root.join("state.json");
    std::fs::write(&target, b"old").expect("write predecessor");

    atomic_write_bytes(&target, b"new durable bytes").expect("publish successor");

    assert_eq!(
        std::fs::read(&target).expect("read successor"),
        b"new durable bytes"
    );
    assert_only_target_and_lease(&root, &target);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn compare_exchange_rejects_external_change_without_touching_target() {
    let root = unique_temp_dir("cas-conflict");
    let target = root.join("state.json");
    std::fs::write(&target, b"accepted").expect("write accepted bytes");
    let accepted: [u8; 32] = Sha256::digest(b"accepted").into();
    std::fs::write(&target, b"external revision").expect("write external revision");

    let result = compare_exchange_bytes(
        &target,
        ExpectedContent::Digest(accepted),
        b"local successor",
    );

    assert!(matches!(result, Err(CompareExchangeError::Conflict { .. })));
    assert_eq!(
        std::fs::read(&target).expect("read target"),
        b"external revision"
    );
    assert_only_target_and_lease(&root, &target);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn compare_exchange_publishes_exact_successor_and_releases_predecessor_evidence() {
    let root = unique_temp_dir("cas-success");
    let target = root.join("state.json");
    std::fs::write(&target, b"accepted").expect("write accepted bytes");
    let accepted: [u8; 32] = Sha256::digest(b"accepted").into();

    compare_exchange_bytes(
        &target,
        ExpectedContent::Digest(accepted),
        b"verified successor",
    )
    .expect("publish exact successor");

    assert_eq!(
        std::fs::read(&target).expect("read successor"),
        b"verified successor"
    );
    #[cfg(windows)]
    assert_no_windows_recovery_artifacts(&target);
    assert_only_target_and_lease(&root, &target);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn compare_exchange_preserves_the_existing_windows_security_contract() {
    let root = unique_temp_dir("cas-windows-security");
    let target = root.join("state.json");
    let contract_reference = root.join("security-reference.json");
    std::fs::write(&target, b"accepted").expect("write predecessor");
    std::fs::write(&contract_reference, b"reference").expect("write contract reference");
    copy_windows_file_security(&target, &contract_reference)
        .expect("capture predecessor security contract");
    let accepted: [u8; 32] = Sha256::digest(b"accepted").into();

    compare_exchange_bytes(
        &target,
        ExpectedContent::Digest(accepted),
        b"verified successor",
    )
    .expect("publish successor with predecessor security");

    verify_windows_file_security_equivalence(&contract_reference, &target)
        .expect("published successor retained predecessor security contract");
    assert_eq!(
        std::fs::read(&target).expect("read successor"),
        b"verified successor"
    );
    std::fs::remove_file(contract_reference).expect("remove contract reference");
    assert_no_windows_recovery_artifacts(&target);
    assert_only_target_and_lease(&root, &target);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn replace_failure_recovery_covers_every_documented_postcondition() {
    use ObservedFileRole::{Missing, Other, Replaced, Replacement, Unreadable};
    use ReplaceFailureRecovery::{
        InputIntact, RestoreBackupIntoReplaced, RestoreCompletedReplacement, Uncertain,
    };

    let cases = [
        (
            "1175 unable to open replacement keeps both named inputs",
            ReplaceObservation {
                replaced: Replaced,
                replacement: Replacement,
                backup: Missing,
            },
            InputIntact,
        ),
        (
            "1176 failure with a requested backup keeps both named inputs",
            ReplaceObservation {
                replaced: Replaced,
                replacement: Replacement,
                backup: Missing,
            },
            InputIntact,
        ),
        (
            "1176 failure without backup evidence is indeterminate",
            ReplaceObservation {
                replaced: Missing,
                replacement: Replacement,
                backup: Missing,
            },
            Uncertain,
        ),
        (
            "1177 partial merge with backup can restore the replaced path",
            ReplaceObservation {
                replaced: Missing,
                replacement: Replacement,
                backup: Replaced,
            },
            RestoreBackupIntoReplaced,
        ),
        (
            "1177 partial merge without backup evidence is indeterminate",
            ReplaceObservation {
                replaced: Missing,
                replacement: Replacement,
                backup: Missing,
            },
            Uncertain,
        ),
        (
            "other documented errors leave both named inputs unchanged",
            ReplaceObservation {
                replaced: Replaced,
                replacement: Replacement,
                backup: Missing,
            },
            InputIntact,
        ),
        (
            "failure reported after a complete replacement is rolled back",
            ReplaceObservation {
                replaced: Replacement,
                replacement: Missing,
                backup: Replaced,
            },
            RestoreCompletedReplacement,
        ),
        (
            "unreadable or foreign bytes are never guessed away",
            ReplaceObservation {
                replaced: Unreadable,
                replacement: Other,
                backup: Replaced,
            },
            Uncertain,
        ),
    ];

    for (label, observation, expected) in cases {
        assert_eq!(
            replacement_failure_recovery(observation),
            expected,
            "{label}"
        );
    }
}

#[cfg(windows)]
#[test]
fn recovery_reconciliation_exhaustively_refuses_inference_and_resurrection() {
    use ObservedFileRole::{Both, Missing, Other, Replaced, Replacement, Unreadable};
    use PublicationResolution::{Aborted, Committed};
    use RecoveryReconciliation::{AcceptResolved, PreserveAndBlock, Resolve};
    use RecoveryRecordState::{Prepared, Resolved};

    let roles = [Missing, Replaced, Replacement, Both, Other, Unreadable];
    let cases = [
        (
            Prepared,
            [
                PreserveAndBlock,
                Resolve(Aborted),
                PreserveAndBlock,
                Resolve(Aborted),
                PreserveAndBlock,
                PreserveAndBlock,
            ],
            [
                Resolve(Aborted),
                Resolve(Aborted),
                PreserveAndBlock,
                Resolve(Aborted),
                PreserveAndBlock,
                PreserveAndBlock,
            ],
        ),
        (
            Resolved(Committed),
            [
                PreserveAndBlock,
                PreserveAndBlock,
                AcceptResolved,
                AcceptResolved,
                PreserveAndBlock,
                PreserveAndBlock,
            ],
            [
                PreserveAndBlock,
                PreserveAndBlock,
                AcceptResolved,
                AcceptResolved,
                PreserveAndBlock,
                PreserveAndBlock,
            ],
        ),
        (
            Resolved(Aborted),
            [
                PreserveAndBlock,
                AcceptResolved,
                PreserveAndBlock,
                AcceptResolved,
                PreserveAndBlock,
                PreserveAndBlock,
            ],
            [
                AcceptResolved,
                AcceptResolved,
                PreserveAndBlock,
                AcceptResolved,
                PreserveAndBlock,
                PreserveAndBlock,
            ],
        ),
    ];

    for (state, present_expectations, absent_expectations) in cases {
        for (index, role) in roles.into_iter().enumerate() {
            assert_eq!(
                recovery_reconciliation(state, role, true),
                present_expectations[index],
                "present predecessor: {state:?}, {role:?}"
            );
            assert_eq!(
                recovery_reconciliation(state, role, false),
                absent_expectations[index],
                "absent predecessor: {state:?}, {role:?}"
            );
        }
    }
}

#[cfg(windows)]
#[test]
fn recovery_resolution_torn_at_every_byte_remains_prepared() {
    let root = unique_temp_dir("torn-resolution");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    let before = b"sealed predecessor";
    let after = b"sealed successor";
    std::fs::write(&target, before).expect("write predecessor");
    std::fs::write(&staged, after).expect("write successor");
    let before_digest = Sha256::digest(before).into();
    let after_digest = Sha256::digest(after).into();
    let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
        .expect("install bundle");
    let base = std::fs::read(&active.path).expect("read prepared bundle");
    let trailer = encode_recovery_resolution(&active.record, PublicationResolution::Committed);

    for prefix_len in 0..RECOVERY_RESOLUTION_LEN {
        let mut torn = base.clone();
        torn.extend_from_slice(&trailer[..prefix_len]);
        std::fs::write(&active.path, torn).expect("write torn resolution");
        let record = read_windows_recovery_record(&active.path)
            .expect("read torn bundle")
            .expect("bundle exists");
        assert_eq!(
            record.state,
            RecoveryRecordState::Prepared,
            "resolution prefix length {prefix_len}"
        );
    }

    let mut complete = base;
    complete.extend_from_slice(&trailer);
    std::fs::write(&active.path, complete).expect("write complete resolution");
    assert_eq!(
        read_windows_recovery_record(&active.path)
            .expect("read complete bundle")
            .expect("bundle exists")
            .state,
        RecoveryRecordState::Resolved(PublicationResolution::Committed)
    );
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn recovery_slots_never_replace_an_occupied_or_invalid_inactive_slot() {
    let root = unique_temp_dir("slot-generations");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    std::fs::write(&target, b"before").expect("write predecessor");
    std::fs::write(&staged, b"after-one").expect("write successor one");
    let before_digest = Sha256::digest(b"before").into();
    let first_after = Sha256::digest(b"after-one").into();
    let first = install_windows_recovery_bundle(&target, &staged, before_digest, first_after)
        .expect("install first generation");
    assert_eq!((first.slot_index, first.record.generation), (0, 1));
    append_windows_recovery_resolution(&first, PublicationResolution::Aborted)
        .expect("abort first generation");

    std::fs::write(&staged, b"after-two").expect("write successor two");
    let second_after = Sha256::digest(b"after-two").into();
    let second = install_windows_recovery_bundle(&target, &staged, before_digest, second_after)
        .expect("install second generation");
    assert_eq!((second.slot_index, second.record.generation), (1, 2));
    append_windows_recovery_resolution(&second, PublicationResolution::Aborted)
        .expect("abort second generation");

    std::fs::write(&staged, b"after-three").expect("write successor three");
    let third_after = Sha256::digest(b"after-three").into();
    assert!(matches!(
        install_windows_recovery_bundle(&target, &staged, before_digest, third_after),
        Err(PublicationFailure::Safe(_))
    ));

    let inactive = recovery_slot_paths(&target)[0].clone();
    let foreign = b"torn inactive generation";
    std::fs::write(&inactive, foreign).expect("corrupt inactive slot");
    assert!(matches!(
        select_active_windows_recovery(&target),
        Err(PublicationFailure::Uncertain { .. })
    ));
    assert!(matches!(
        install_windows_recovery_bundle(&target, &staged, before_digest, third_after),
        Err(PublicationFailure::Uncertain { .. })
    ));
    assert_eq!(
        std::fs::read(&inactive).expect("read preserved foreign slot"),
        foreign
    );
    assert_eq!(std::fs::read(&target).expect("target unchanged"), b"before");
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn recovery_slot_binding_ambiguity_and_generation_overflow_fail_closed() {
    fn reseal_header(bytes: &mut [u8]) {
        let checksum: [u8; 32] = Sha256::new()
            .chain_update(b"rspice-windows-recovery-header\0v1\0")
            .chain_update(&bytes[..RECOVERY_HEADER_PREFIX_LEN])
            .finalize()
            .into();
        bytes[RECOVERY_HEADER_PREFIX_LEN..RECOVERY_HEADER_LEN].copy_from_slice(&checksum);
    }

    let root = unique_temp_dir("slot-identity");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    std::fs::write(&target, b"before").expect("write predecessor");
    std::fs::write(&staged, b"after").expect("write successor");
    let before_digest = Sha256::digest(b"before").into();
    let after_digest = Sha256::digest(b"after").into();
    let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
        .expect("install generation");
    let inactive = recovery_slot_paths(&target)[1].clone();
    let original = std::fs::read(&active.path).expect("read active bundle");

    let mut wrong_binding = original.clone();
    wrong_binding[56] ^= 0x80;
    reseal_header(&mut wrong_binding);
    std::fs::write(&inactive, &wrong_binding).expect("write wrong-binding inactive slot");
    assert!(matches!(
        select_active_windows_recovery(&target),
        Err(PublicationFailure::Uncertain { .. })
    ));
    assert_eq!(
        std::fs::read(&inactive).expect("wrong-binding record is preserved"),
        wrong_binding
    );

    let mut ambiguous = original.clone();
    ambiguous[24..40].copy_from_slice(Uuid::new_v4().as_bytes());
    reseal_header(&mut ambiguous);
    std::fs::write(&inactive, ambiguous).expect("write equal-generation transaction");
    assert!(matches!(
        select_active_windows_recovery(&target),
        Err(PublicationFailure::Uncertain { .. })
    ));

    std::fs::remove_file(&inactive).expect("remove ambiguous slot");
    let mut exhausted = original;
    exhausted[16..24].copy_from_slice(&u64::MAX.to_le_bytes());
    reseal_header(&mut exhausted);
    std::fs::write(&active.path, exhausted).expect("write maximum generation");
    assert!(matches!(
        install_windows_recovery_bundle(&target, &staged, before_digest, after_digest),
        Err(PublicationFailure::Safe(_))
    ));
    assert_eq!(std::fs::read(&target).expect("target unchanged"), b"before");
    assert_eq!(
        select_active_windows_recovery(&target)
            .expect("maximum generation remains valid")
            .expect("active exists")
            .record
            .generation,
        u64::MAX
    );
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn prepared_successor_and_missing_target_are_preserved_without_replay() {
    for (label, remove_target) in [("successor", false), ("missing", true)] {
        let root = unique_temp_dir(label);
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        let before = b"before";
        let after = b"after";
        std::fs::write(&target, before).expect("write predecessor");
        std::fs::write(&staged, after).expect("write successor");
        install_windows_recovery_bundle(
            &target,
            &staged,
            Sha256::digest(before).into(),
            Sha256::digest(after).into(),
        )
        .expect("install prepared bundle");
        if remove_target {
            std::fs::remove_file(&target).expect("remove canonical target");
        } else {
            std::fs::write(&target, after).expect("expose uncommitted successor");
        }

        assert!(matches!(
            reconcile_windows_recovery_slots(&target),
            Err(PublicationFailure::Uncertain { .. })
        ));
        if remove_target {
            assert!(!target.exists(), "missing target must not be resurrected");
        } else {
            assert_eq!(
                std::fs::read(&target).expect("read preserved successor"),
                after,
                "existing successor must not be overwritten"
            );
        }
        std::fs::remove_dir_all(root).expect("remove fixture");
    }
}

#[cfg(windows)]
#[test]
fn prepared_predecessor_is_aborted_and_raced_backup_is_never_discarded() {
    let root = unique_temp_dir("prepared-abort");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    std::fs::write(&target, b"before").expect("write predecessor");
    std::fs::write(&staged, b"after").expect("write successor");
    let before_digest = Sha256::digest(b"before").into();
    let after_digest = Sha256::digest(b"after").into();
    let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
        .expect("install prepared bundle");

    reconcile_windows_recovery_slots(&target).expect("settle exact predecessor as aborted");
    assert!(
        select_active_windows_recovery(&target)
            .expect("read retired slots")
            .is_none(),
        "settled abort must retire its recovery generation"
    );
    assert!(!active.path.exists());

    std::fs::write(&staged, b"after-two").expect("write next successor");
    let next_after = Sha256::digest(b"after-two").into();
    let next = install_windows_recovery_bundle(&target, &staged, before_digest, next_after)
        .expect("install next prepared generation");
    std::fs::write(&target, b"after-two").expect("expose successor");
    let backup = recovery_backup_path(&target, next.record.backup_id);
    std::fs::write(&backup, b"external raced backup").expect("write raced backup");
    assert!(matches!(
        resolve_windows_recovery_slot(&target, next_after, PublicationResolution::Committed),
        Err(PublicationFailure::Uncertain { .. })
    ));
    assert_eq!(
        std::fs::read(&backup).expect("raced evidence preserved"),
        b"external raced backup"
    );
    assert_eq!(
        read_windows_recovery_record(&next.path)
            .expect("read prepared bundle")
            .expect("bundle exists")
            .state,
        RecoveryRecordState::Prepared
    );
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn startup_settlement_retires_committed_records_and_retirement_is_idempotent() {
    let root = unique_temp_dir("startup-retirement");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    std::fs::write(&target, b"before").expect("write predecessor");
    std::fs::write(&staged, b"after").expect("write successor");
    let before_digest = Sha256::digest(b"before").into();
    let after_digest = Sha256::digest(b"after").into();
    let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
        .expect("install prepared bundle");
    let backup = recovery_backup_path(&target, active.record.backup_id);
    std::fs::write(&backup, b"before").expect("write displaced predecessor");
    std::fs::write(&target, b"after").expect("publish successor");
    resolve_windows_recovery_slot(&target, after_digest, PublicationResolution::Committed)
        .expect("resolve committed generation");

    reconcile_windows_recovery_slots(&target).expect("settle startup generation");
    assert_no_windows_recovery_artifacts(&target);
    retire_settled_windows_recovery(&target).expect("first idempotent retirement");
    retire_settled_windows_recovery(&target).expect("second idempotent retirement");
    reconcile_windows_recovery_slots(&target).expect("idempotent reconciliation");
    assert_eq!(std::fs::read(&target).expect("read successor"), b"after");
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn retirement_never_deletes_or_replaces_foreign_tombstones() {
    let root = unique_temp_dir("foreign-retirement-tombstone");
    let target = root.join("state.json");
    std::fs::write(&target, b"before").expect("write target");
    let inactive_tombstone = recovery_retired_path(&recovery_slot_paths(&target)[0]);
    let foreign = b"unrelated user tombstone data";
    std::fs::write(&inactive_tombstone, foreign).expect("write foreign tombstone");

    assert!(matches!(
        retire_settled_windows_recovery(&target),
        Err(PublicationFailure::Uncertain { .. })
    ));
    assert_eq!(
        std::fs::read(&inactive_tombstone).expect("read preserved tombstone"),
        foreign
    );

    std::fs::remove_file(&inactive_tombstone).expect("remove first fixture tombstone");
    let staged = root.join("staged.json");
    std::fs::write(&staged, b"after").expect("write staged successor");
    let active = install_windows_recovery_bundle(
        &target,
        &staged,
        Sha256::digest(b"before").into(),
        Sha256::digest(b"after").into(),
    )
    .expect("install recovery generation");
    append_windows_recovery_resolution(&active, PublicationResolution::Aborted)
        .expect("resolve aborted generation");
    let active_tombstone = recovery_retired_path(&active.path);
    std::fs::write(&active_tombstone, foreign).expect("write active foreign tombstone");

    assert!(matches!(
        retire_settled_windows_recovery(&target),
        Err(PublicationFailure::Uncertain { .. })
    ));
    assert_eq!(
        std::fs::read(&active_tombstone).expect("read active foreign tombstone"),
        foreign
    );
    assert!(active.path.exists(), "owned slot must remain named");
    assert_eq!(std::fs::read(&target).expect("target unchanged"), b"before");
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn external_edit_after_successful_cas_reconciles_without_stale_record() {
    let root = unique_temp_dir("post-save-external-edit");
    let target = root.join("state.json");
    std::fs::write(&target, b"accepted").expect("write accepted destination");
    compare_exchange_bytes(
        &target,
        ExpectedContent::Digest(Sha256::digest(b"accepted").into()),
        b"saved successor",
    )
    .expect("publish successor");
    assert_no_windows_recovery_artifacts(&target);

    std::fs::write(&target, b"legitimate external edit").expect("write external edit");
    reconcile_publication(&target).expect("reconcile after external edit");
    assert_eq!(
        std::fs::read(&target).expect("read external edit"),
        b"legitimate external edit"
    );
    assert_no_windows_recovery_artifacts(&target);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn committed_generation_never_overwrites_an_external_predecessor_restore() {
    let root = unique_temp_dir("committed-restore");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    std::fs::write(&target, b"before").expect("write predecessor");
    std::fs::write(&staged, b"after").expect("write successor");
    let before_digest = Sha256::digest(b"before").into();
    let after_digest = Sha256::digest(b"after").into();
    let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
        .expect("install prepared bundle");
    let backup = recovery_backup_path(&target, active.record.backup_id);
    std::fs::write(&backup, b"before").expect("write exact displaced predecessor");
    std::fs::write(&target, b"after").expect("publish successor");
    resolve_windows_recovery_slot(&target, after_digest, PublicationResolution::Committed)
        .expect("commit exact endpoint");
    std::fs::remove_file(&backup).expect("simulate completed cleanup");
    std::fs::write(&target, b"before").expect("externally restore predecessor");

    assert!(matches!(
        reconcile_windows_recovery_slots(&target),
        Err(PublicationFailure::Uncertain { .. })
    ));
    assert_eq!(std::fs::read(&target).expect("read target"), b"before");
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn expected_content_observation_never_grants_unconditional_overwrite() {
    let root = unique_temp_dir("observe-cas");
    let target = root.join("state.json");
    assert_eq!(
        observe_expected_content(&target).expect("observe missing destination"),
        ExpectedContent::Missing
    );
    std::fs::write(&target, b"accepted bytes").expect("write destination");
    assert_eq!(
        observe_expected_content(&target).expect("observe exact destination"),
        ExpectedContent::Digest(Sha256::digest(b"accepted bytes").into())
    );
    reconcile_publication(&target).expect("explicit reconciliation is idempotent");
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn compare_exchange_requires_missing_destination_when_requested() {
    let root = unique_temp_dir("cas-missing");
    let target = root.join("state.json");
    std::fs::write(&target, b"someone won").expect("write competing file");

    let result = compare_exchange_bytes(&target, ExpectedContent::Missing, b"local successor");

    assert!(matches!(result, Err(CompareExchangeError::Conflict { .. })));
    assert_eq!(std::fs::read(&target).unwrap(), b"someone won");
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn destination_lease_serializes_writers_and_reuses_one_stable_inode() {
    let root = unique_temp_dir("lease");
    let target = root.join("state.json");
    let first = DestinationLease::acquire(&target).expect("acquire first lease");
    assert!(matches!(
        DestinationLease::acquire(&target),
        Err(CompareExchangeError::LeaseBusy(path)) if path == lease_path(&target)
    ));

    drop(first);
    let reacquired = DestinationLease::acquire(&target).expect("reacquire released lease");
    drop(reacquired);
    assert_eq!(
        std::fs::read_dir(&root).expect("read fixture").count(),
        1,
        "the permanent lease inode is bounded to one file per destination"
    );
    assert!(lease_path(&target).is_file());
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn foreign_lease_collision_is_rejected_without_modification() {
    let root = unique_temp_dir("foreign-lease");
    let target = root.join("state.json");
    let lease = lease_path(&target);
    let foreign = b"unrelated user data at the collision path";
    std::fs::write(&lease, foreign).expect("write foreign lease collision");

    assert!(matches!(
        DestinationLease::acquire(&target),
        Err(CompareExchangeError::Io(_))
    ));
    assert_eq!(
        std::fs::read(&lease).expect("read foreign collision"),
        foreign
    );
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn hard_linked_lease_collision_never_modifies_its_other_name() {
    let root = unique_temp_dir("hard-linked-lease");
    let target = root.join("state.json");
    let victim = root.join("important-user-data.bin");
    let victim_bytes = b"must survive lease acquisition exactly";
    std::fs::write(&victim, victim_bytes).expect("write victim");
    std::fs::hard_link(&victim, lease_path(&target)).expect("create hard-link collision");

    assert!(matches!(
        DestinationLease::acquire(&target),
        Err(CompareExchangeError::Io(_))
    ));
    assert_eq!(std::fs::read(&victim).expect("read victim"), victim_bytes);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn symbolic_link_lease_collision_is_never_followed() {
    use std::os::unix::fs::symlink;

    let root = unique_temp_dir("symlink-lease");
    let target = root.join("state.json");
    let victim = root.join("important-user-data.bin");
    let victim_bytes = b"must survive symlink lease acquisition exactly";
    std::fs::write(&victim, victim_bytes).expect("write victim");
    symlink(&victim, lease_path(&target)).expect("create symlink collision");

    assert!(matches!(
        DestinationLease::acquire(&target),
        Err(CompareExchangeError::Io(_))
    ));
    assert_eq!(std::fs::read(&victim).expect("read victim"), victim_bytes);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn prepared_unix_exchange_is_rolled_back_during_restart_reconciliation() {
    let root = unique_temp_dir("unix-prepared-exchange");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    let before = b"external predecessor";
    let after = b"uncommitted successor";
    std::fs::write(&target, before).expect("write predecessor");
    std::fs::write(&staged, after).expect("write successor");
    let before_digest = Sha256::digest(before).into();
    let after_digest = Sha256::digest(after).into();
    let (active, backup) =
        prepare_unix_recovery(&staged, &target, Some(before_digest), after_digest)
            .expect("prepare durable recovery");
    exchange_paths_checked(&backup, &target, after_digest, before_digest)
        .expect("simulate crash after exchange");

    reconcile_unix_recovery_slots(&target).expect("rollback prepared exchange");

    assert_eq!(std::fs::read(&target).expect("read predecessor"), before);
    assert!(!backup.exists());
    assert!(!active.path.exists());
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn prepared_unix_new_target_is_removed_during_restart_reconciliation() {
    let root = unique_temp_dir("unix-prepared-new-target");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    let after = b"uncommitted successor";
    std::fs::write(&staged, after).expect("write successor");
    let after_digest = Sha256::digest(after).into();
    let (active, backup) = prepare_unix_recovery(&staged, &target, None, after_digest)
        .expect("prepare missing-target recovery");
    move_no_replace_checked(&backup, &target, after_digest).expect("simulate crash after publish");

    reconcile_unix_recovery_slots(&target).expect("rollback prepared new target");

    assert!(!target.exists());
    assert!(!backup.exists());
    assert!(!active.path.exists());
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn committed_unix_exchange_survives_restart_and_retires_both_evidence_paths() {
    let root = unique_temp_dir("unix-committed-exchange");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    let before = b"accepted predecessor";
    let after = b"committed successor";
    std::fs::write(&target, before).expect("write predecessor");
    std::fs::write(&staged, after).expect("write successor");
    let before_digest = Sha256::digest(before).into();
    let after_digest = Sha256::digest(after).into();
    let (active, backup) =
        prepare_unix_recovery(&staged, &target, Some(before_digest), after_digest)
            .expect("prepare durable recovery");
    exchange_paths_checked(&backup, &target, after_digest, before_digest)
        .expect("publish successor");
    sync_parent_of(&target).expect("sync publication");
    append_windows_recovery_resolution(&active, PublicationResolution::Committed)
        .expect("commit recovery generation");

    reconcile_unix_recovery_slots(&target).expect("settle committed exchange");

    assert_eq!(std::fs::read(&target).expect("read successor"), after);
    assert!(!backup.exists());
    assert!(!active.path.exists());
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn unix_reconciliation_preserves_foreign_endpoint_and_all_recovery_evidence() {
    let root = unique_temp_dir("unix-foreign-endpoint");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    let before = b"accepted predecessor";
    let after = b"uncommitted successor";
    std::fs::write(&target, before).expect("write predecessor");
    std::fs::write(&staged, after).expect("write successor");
    let before_digest = Sha256::digest(before).into();
    let after_digest = Sha256::digest(after).into();
    let (active, backup) =
        prepare_unix_recovery(&staged, &target, Some(before_digest), after_digest)
            .expect("prepare durable recovery");
    std::fs::write(&target, b"foreign bytes").expect("write external endpoint");

    assert!(matches!(
        reconcile_unix_recovery_slots(&target),
        Err(PublicationFailure::Uncertain { .. })
    ));
    assert_eq!(
        std::fs::read(&target).expect("read foreign endpoint"),
        b"foreign bytes"
    );
    assert_eq!(std::fs::read(&backup).expect("read successor"), after);
    assert!(active.path.exists());
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn unix_exchange_captures_and_restores_a_last_instant_external_edit() {
    let root = unique_temp_dir("unix-raced-predecessor");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    let accepted = b"accepted predecessor";
    let external = b"last instant external edit";
    let successor = b"local successor";
    std::fs::write(&target, accepted).expect("write accepted predecessor");
    std::fs::write(&staged, successor).expect("write successor");
    let accepted_digest = Sha256::digest(accepted).into();
    let successor_digest = Sha256::digest(successor).into();
    let (active, backup) =
        prepare_unix_recovery(&staged, &target, Some(accepted_digest), successor_digest)
            .expect("prepare durable recovery");
    std::fs::write(&target, external).expect("race external predecessor");

    exchange_paths_capturing_predecessor(&backup, &target, successor_digest, accepted_digest)
        .expect("capture raced predecessor");
    assert_eq!(std::fs::read(&backup).expect("read capture"), external);
    rollback_retaining_successor(&target, &backup).expect("restore raced predecessor");
    sync_parent_of(&target).expect("sync rollback");
    resolve_unix_recovery_slot(
        &target,
        Some(&backup),
        successor_digest,
        PublicationResolution::Aborted,
    )
    .expect("resolve aborted transaction");
    remove_verified_recovery_sidecar(
        &backup,
        ObservedFileRole::Replacement,
        accepted_digest,
        successor_digest,
    )
    .expect("remove local successor evidence");
    retire_settled_unix_recovery(&target).expect("retire aborted record");

    assert_eq!(
        std::fs::read(&target).expect("read restored edit"),
        external
    );
    assert!(!backup.exists());
    assert!(!active.path.exists());
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn unix_failed_exchange_preserves_an_external_deletion_and_restores_staging() {
    let root = unique_temp_dir("unix-raced-deletion");
    let target = root.join("state.json");
    let staged = root.join("staged.json");
    let accepted = b"accepted predecessor";
    let successor = b"local successor";
    std::fs::write(&target, accepted).expect("write accepted predecessor");
    std::fs::write(&staged, successor).expect("write successor");
    let accepted_digest = Sha256::digest(accepted).into();
    let successor_digest = Sha256::digest(successor).into();
    let (active, backup) =
        prepare_unix_recovery(&staged, &target, Some(accepted_digest), successor_digest)
            .expect("prepare durable recovery");
    std::fs::remove_file(&target).expect("race external deletion");
    let error =
        exchange_paths_capturing_predecessor(&backup, &target, successor_digest, accepted_digest)
            .expect_err("exchange must report intact staging");
    let PublicationFailure::Safe(error) = error else {
        panic!("intact staging must be a safe publication failure");
    };
    assert!(matches!(
        abort_unpublished_unix_recovery(
            &staged,
            &target,
            &active,
            &backup,
            successor_digest,
            error,
        ),
        Err(PublicationFailure::Safe(_))
    ));

    assert!(
        !target.exists(),
        "external deletion must not be resurrected"
    );
    assert_eq!(std::fs::read(&staged).expect("read staging"), successor);
    assert!(!backup.exists());
    assert!(!active.path.exists());
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn replacement_preserves_existing_unix_mode() {
    use std::os::unix::fs::PermissionsExt as _;

    let root = unique_temp_dir("permissions");
    let target = root.join("state.json");
    std::fs::write(&target, b"old").expect("write predecessor");
    std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o640))
        .expect("set predecessor mode");

    atomic_write_bytes(&target, b"new").expect("publish successor");

    assert_eq!(
        std::fs::metadata(&target)
            .expect("read metadata")
            .permissions()
            .mode()
            & 0o777,
        0o640
    );
    std::fs::remove_dir_all(root).expect("remove fixture");
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "rspice-durable-file-{label}-{}-{}",
        std::process::id(),
        Uuid::new_v4()
    ));
    std::fs::create_dir_all(&root).expect("create fixture");
    root
}

fn assert_only_target_and_lease(root: &Path, target: &Path) {
    let mut actual = std::fs::read_dir(root)
        .expect("read fixture")
        .map(|entry| entry.expect("entry").path())
        .collect::<Vec<_>>();
    let mut expected = vec![target.to_path_buf(), lease_path(target)];
    #[cfg(windows)]
    expected.extend(
        recovery_slot_paths(target)
            .into_iter()
            .filter(|path| path.exists()),
    );
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[cfg(windows)]
fn assert_no_windows_recovery_artifacts(target: &Path) {
    for slot in recovery_slot_paths(target) {
        assert!(!slot.exists(), "stale recovery slot: {}", slot.display());
        let retired = recovery_retired_path(&slot);
        assert!(
            !retired.exists(),
            "stale retirement tombstone: {}",
            retired.display()
        );
    }
}
