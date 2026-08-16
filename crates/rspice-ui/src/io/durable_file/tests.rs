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

/// Every pathname a publication derives hangs off the destination's parent,
/// and the longest of them — a lease name carrying a staging suffix — adds 126
/// characters to it. The classic Windows pathname limit would refuse those
/// derivations for any parent deep enough, so a destination must be named in
/// the form that limit exempts; otherwise how deeply a user filed a project
/// decides whether it can be saved at all.
#[cfg(windows)]
#[test]
fn publication_completes_where_the_derived_pathnames_outgrow_the_windows_limit() {
    // A lease name is 85 characters and a staging suffix adds 41 more.
    const DERIVED_SIDECAR_LEN: usize = 126;
    // The longest pathname a non-verbatim spelling may reach.
    const LEGACY_PATH_LIMIT: usize = 259;
    // How far past that limit the deepest derivation must reach, so the fixture
    // proves the property instead of sitting on its boundary.
    const REQUIRED_OVERRUN: usize = 64;

    let root = unique_temp_dir("deep-destination");
    // The fixture nests to its own depth rather than inheriting one: on a
    // machine whose temporary directory is short, every derived pathname would
    // otherwise fit under the limit and this would assert nothing.
    let mut parent = root.clone();
    while parent.as_os_str().len() + 1 + DERIVED_SIDECAR_LEN
        < LEGACY_PATH_LIMIT + REQUIRED_OVERRUN
    {
        parent = parent.join("nested-project-directory");
    }
    std::fs::create_dir_all(&parent).expect("create the deep destination directory");
    let target = parent.join("state.json");

    atomic_write_bytes(&target, b"deep predecessor").expect("publish predecessor");
    let accepted = observe_expected_content(&target).expect("observe published predecessor");
    compare_exchange_bytes(&target, accepted, b"deep successor").expect("publish successor");

    assert_eq!(
        std::fs::read(&target).expect("read successor"),
        b"deep successor"
    );
    assert_no_windows_recovery_artifacts(&target);
    assert_only_target_and_lease(&parent, &target);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

/// The half of a Windows security descriptor that compare-and-exchange owns.
///
/// `ReplaceFileW` does not carry the predecessor's descriptor over wholesale:
/// it re-derives the inherited ACEs from the destination directory as it
/// publishes, so the inherited half of the published file describes that
/// directory rather than the file that was replaced. Asserting whole-
/// descriptor equality therefore asserts a property of the ambient directory,
/// which is why the earlier form of this test held on a developer profile and
/// failed on the CI runner's.
///
/// What publication does owe the predecessor is its explicit contract: the
/// owner, the group, and the ACEs the file states in its own right, which
/// `compare_exchange_bytes` stages onto the successor before the replace.
///
/// Proving that needs a predecessor holding something a freshly created
/// sibling cannot inherit. Without the distinguishing ACE below, the
/// predecessor and the staged temporary inherit identical descriptors from the
/// one directory they share, and the assertion passes even with the staging
/// step deleted outright.
#[cfg(windows)]
#[test]
fn compare_exchange_preserves_the_existing_windows_security_contract() {
    const DISTINGUISHING_ACE: &str = "(A;;FR;;;WD)";

    let root = unique_temp_dir("cas-windows-security");
    let target = root.join("state.json");
    std::fs::write(&target, b"accepted").expect("write predecessor");
    grant_explicit_windows_ace(&target, DISTINGUISHING_ACE);

    let predecessor = windows_security_sddl(&target).expect("read predecessor contract");
    assert!(
        explicit_aces(&predecessor)
            .iter()
            .any(|ace| ace == DISTINGUISHING_ACE),
        "the fixture must hold an explicit ACE that a staged sibling cannot inherit: {predecessor}"
    );
    let accepted: [u8; 32] = Sha256::digest(b"accepted").into();

    compare_exchange_bytes(
        &target,
        ExpectedContent::Digest(accepted),
        b"verified successor",
    )
    .expect("publish successor with predecessor security");

    let successor = windows_security_sddl(&target).expect("read successor contract");
    assert_eq!(
        security_header(&successor),
        security_header(&predecessor),
        "owner, group, and DACL control must survive publication\n{predecessor}\n{successor}"
    );
    assert_eq!(
        explicit_aces(&successor),
        explicit_aces(&predecessor),
        "explicit access must survive publication\n{predecessor}\n{successor}"
    );
    assert_eq!(
        std::fs::read(&target).expect("read successor"),
        b"verified successor"
    );
    assert_no_windows_recovery_artifacts(&target);
    assert_only_target_and_lease(&root, &target);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

/// Add one explicit ACE to whatever `path` inherited, leaving the inherited
/// ACEs and their order alone. An explicit allow precedes the inherited allows
/// in canonical order, so it is prepended.
#[cfg(windows)]
fn grant_explicit_windows_ace(path: &Path, ace: &str) {
    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Authorization::{
        ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
    };
    use windows_sys::Win32::Security::SetFileSecurityW;
    use windows_sys::Win32::Security::{DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR};

    let current = windows_security_sddl(path).expect("read fixture contract");
    let dacl = current
        .find("D:")
        .expect("fixture descriptor states a DACL");
    let first_ace = current[dacl..]
        .find('(')
        .map_or(current.len(), |offset| dacl + offset);
    let requested = format!(
        "{}{ace}{}",
        &current[dacl..first_ace],
        &current[first_ace..]
    );

    let text = requested
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();
    let mut descriptor: PSECURITY_DESCRIPTOR = std::ptr::null_mut();
    let converted = unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            text.as_ptr(),
            SDDL_REVISION_1,
            &mut descriptor,
            std::ptr::null_mut(),
        )
    };
    assert!(
        converted != 0,
        "build '{requested}': {}",
        io::Error::last_os_error()
    );
    let applied =
        unsafe { SetFileSecurityW(wide(path).as_ptr(), DACL_SECURITY_INFORMATION, descriptor) };
    let failure = io::Error::last_os_error();
    unsafe { LocalFree(descriptor.cast()) };
    assert!(applied != 0, "apply '{requested}': {failure}");
}

/// The owner, group, and DACL control a descriptor declares, without the
/// provenance bits Windows rewrites as it publishes.
#[cfg(windows)]
fn security_header(sddl: &str) -> String {
    let header = sddl.find('(').map_or(sddl, |first_ace| &sddl[..first_ace]);
    without_dacl_provenance_flags(header.to_owned())
}

/// The ACEs a descriptor states in its own right, excluding the inherited half
/// that Windows re-derives from the parent directory on every publication.
#[cfg(windows)]
fn explicit_aces(sddl: &str) -> Vec<String> {
    let mut explicit = Vec::new();
    let mut rest = sddl;
    while let Some(open) = rest.find('(') {
        let Some(close) = rest[open..].find(')') else {
            break;
        };
        let ace = &rest[open..=open + close];
        let inherited = ace
            .trim_matches(['(', ')'])
            .split(';')
            .nth(1)
            .is_some_and(|flags| flags.contains("ID"));
        if !inherited {
            explicit.push(ace.to_owned());
        }
        rest = &rest[open + close + 1..];
    }
    explicit
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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

/// Evidence sealed before destinations were resolved binds this same
/// destination under the pathname a lexical derivation produced. That digest
/// stays owned: it can admit no other file's record, while refusing it would
/// leave a crashed publication behind a destination no later save could
/// reconcile.
#[cfg(windows)]
#[test]
fn evidence_bound_before_the_destination_was_resolved_is_still_owned() {
    let root = unique_temp_dir("pre-resolution-binding");
    let target = internal_target(&root);
    let staged = root.join("staged.json");
    std::fs::write(&target, b"before").expect("write predecessor");
    std::fs::write(&staged, b"after").expect("write successor");
    let sealed = install_windows_recovery_bundle(
        &target,
        &staged,
        Sha256::digest(b"before").into(),
        Sha256::digest(b"after").into(),
    )
    .expect("install generation");

    let lexical = pre_resolution_destination(&target).expect("a resolved destination restates one");
    assert_ne!(lexical, target);
    let legacy = target_binding_digest(&lexical);
    let mut bytes = std::fs::read(&sealed.path).expect("read sealed bundle");
    bytes[56..88].copy_from_slice(&legacy);
    let checksum: [u8; 32] = Sha256::new()
        .chain_update(b"rspice-windows-recovery-header\0v1\0")
        .chain_update(&bytes[..RECOVERY_HEADER_PREFIX_LEN])
        .finalize()
        .into();
    bytes[RECOVERY_HEADER_PREFIX_LEN..RECOVERY_HEADER_LEN].copy_from_slice(&checksum);
    std::fs::write(&sealed.path, bytes).expect("rebind to the lexical pathname");

    let active = select_active_windows_recovery(&target)
        .expect("a lexical binding still names this destination")
        .expect("the sealed generation is active");
    assert_eq!(active.path, sealed.path);
    assert_eq!(active.record.generation, sealed.record.generation);
    assert_eq!(active.record.target_binding, legacy);
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[cfg(windows)]
#[test]
fn prepared_successor_and_missing_target_are_preserved_without_replay() {
    for (label, remove_target) in [("successor", false), ("missing", true)] {
        let root = unique_temp_dir(label);
        let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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

/// One file has one lease however it was reached, because that lease is the
/// only thing that keeps two writers from publishing over each other. A
/// junction gives a directory a second pathname, so a derivation reading the
/// spelling rather than the file it reaches would hang a second lease name in
/// the very directory the first writer is already holding, and neither writer
/// would exclude the other.
#[cfg(windows)]
#[test]
fn a_junction_spelling_and_the_directory_it_reaches_contend_for_one_lease() {
    let root = unique_temp_dir("junction-identity");
    let real = root.join("real");
    let junction = root.join("junction");
    std::fs::create_dir_all(&real).expect("create the destination directory");
    create_directory_junction(&junction, &real);

    let target = resolved_destination(&real.join("state.json"));
    let through_junction = junction.join("state.json");
    std::fs::write(&target, b"accepted").expect("write destination");

    let held = DestinationLease::acquire(&target).expect("acquire destination lease");
    assert!(
        matches!(
            compare_exchange_bytes(
                &through_junction,
                ExpectedContent::Digest(Sha256::digest(b"accepted").into()),
                b"successor through the junction",
            ),
            Err(CompareExchangeError::LeaseBusy(path)) if path == lease_path(&target)
        ),
        "a writer arriving through the junction must be excluded by the lease already held"
    );
    drop(held);
    assert_eq!(
        std::fs::read(&target).expect("read refused destination"),
        b"accepted"
    );
    assert_eq!(
        lease_path(&resolved_destination(&through_junction)),
        lease_path(&target),
        "both spellings reach one file, so both must derive its one lease"
    );
    assert_only_target_and_lease(&real, &target);

    std::fs::remove_dir(&junction).expect("remove the junction");
    assert_eq!(
        std::fs::read(&target).expect("read the destination the junction reached"),
        b"accepted",
        "removing a junction removes the second pathname, never the file"
    );
    std::fs::remove_dir_all(root).expect("remove fixture");
}

/// Give one directory a second pathname. A junction needs no privilege, so a
/// test may create one wherever it can create a directory.
#[cfg(windows)]
fn create_directory_junction(link: &Path, target: &Path) {
    let output = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(link)
        .arg(target)
        .output()
        .expect("run mklink");
    assert!(
        output.status.success(),
        "mklink /J '{}' '{}' exited with {}: {}{}",
        link.display(),
        target.display(),
        output.status,
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from_utf8_lossy(&output.stderr).trim()
    );
    assert!(
        std::fs::symlink_metadata(link)
            .expect("read the junction's own metadata")
            .file_type()
            .is_symlink(),
        "'{}' must be a reparse point reaching '{}'",
        link.display(),
        target.display()
    );
}

#[cfg(unix)]
#[test]
fn symbolic_link_lease_collision_is_never_followed() {
    use std::os::unix::fs::symlink;

    let root = unique_temp_dir("symlink-lease");
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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
    let target = internal_target(&root);
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

/// A destination for a test that drives publication below its entry points.
///
/// An entry point resolves its destination once and derives every lease and
/// recovery pathname from that identity, so a test calling those derivations
/// directly must state the destination the same way; the spelling a fixture
/// happens to hold is one no publication would ever hand them.
fn internal_target(root: &Path) -> PathBuf {
    resolved_destination(&root.join("state.json"))
}

/// Nothing but the destination, its lease, and live recovery slots may remain.
///
/// Every candidate is a name in one directory, so names are what is compared:
/// a resolved derivation and a fixture spelling reach the same entry by
/// different pathnames.
fn assert_only_target_and_lease(root: &Path, target: &Path) {
    fn entry_name(path: &Path) -> std::ffi::OsString {
        path.file_name()
            .expect("a publication pathname names an entry")
            .to_os_string()
    }

    let destination = resolved_destination(target);
    let mut actual = std::fs::read_dir(root)
        .expect("read fixture")
        .map(|entry| entry.expect("entry").file_name())
        .collect::<Vec<_>>();
    let mut expected = vec![entry_name(target), entry_name(&lease_path(&destination))];
    #[cfg(windows)]
    expected.extend(
        recovery_slot_paths(&destination)
            .iter()
            .filter(|path| path.exists())
            .map(|path| entry_name(path)),
    );
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[cfg(windows)]
fn assert_no_windows_recovery_artifacts(target: &Path) {
    for slot in recovery_slot_paths(&resolved_destination(target)) {
        assert!(!slot.exists(), "stale recovery slot: {}", slot.display());
        let retired = recovery_retired_path(&slot);
        assert!(
            !retired.exists(),
            "stale retirement tombstone: {}",
            retired.display()
        );
    }
}
