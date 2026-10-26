//! The device-local shortcut library store over the product's own commands.

use crate::state::shortcuts::ShortcutProfileLibrary;
use crate::workbench::shortcuts::artifacts::{canonical_json_bytes, hex_digest, sha256};
use crate::workbench::shortcuts::library_persistence::*;
use serde_json::Value;
use std::cell::RefCell;

#[derive(Default)]
struct MemoryStore {
    bytes: RefCell<Option<Vec<u8>>>,
}

impl ShortcutLibraryStore for MemoryStore {
    fn read(&self) -> Result<Option<Vec<u8>>, ShortcutLibraryPersistenceError> {
        Ok(self.bytes.borrow().clone())
    }

    fn compare_exchange(
        &self,
        expected: StoreExpectation,
        bytes: &[u8],
    ) -> Result<(), ShortcutLibraryPersistenceError> {
        let actual = self.bytes.borrow().as_deref().map(sha256);
        let matches = match expected {
            StoreExpectation::Missing => actual.is_none(),
            StoreExpectation::Digest(expected) => actual == Some(expected),
        };
        if !matches {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.compare-exchange",
                "memory store changed",
            ));
        }
        *self.bytes.borrow_mut() = Some(bytes.to_vec());
        Ok(())
    }
}

fn edited(predecessor: &PersistedShortcutProfileLibrary) -> ShortcutProfileLibrary {
    let mut candidate = predecessor.library().clone();
    candidate
        .edit_active(|profile| {
            profile
                .policies_mut()
                .set_chord_timeout(crate::state::shortcuts::ChordTimeoutPolicy::ThreeSeconds);
        })
        .unwrap();
    candidate
}

#[test]
fn deterministic_create_and_strict_restore() {
    let library = ShortcutProfileLibrary::default();
    let first = encode_store_bytes(
        &library,
        1,
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    let second = encode_store_bytes(
        &library,
        1,
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    assert_eq!(first, second);
    assert!(matches!(
        decode_store_bytes(
            &first,
            crate::workbench::commands::vocabulary::command_registry()
        ),
        ShortcutProfileLibraryRestore::Compatible(_)
    ));
    let mut noncanonical = first;
    noncanonical.pop();
    assert!(matches!(
        decode_store_bytes(
            &noncanonical,
            crate::workbench::commands::vocabulary::command_registry()
        ),
        ShortcutProfileLibraryRestore::Corrupt(_)
    ));
}

#[test]
fn create_is_create_only_and_update_requires_exact_predecessor() {
    let store = MemoryStore::default();
    let first = create_in_store(
        &store,
        &ShortcutProfileLibrary::default(),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    assert!(
        create_in_store(
            &store,
            &ShortcutProfileLibrary::default(),
            crate::workbench::commands::vocabulary::command_registry()
        )
        .is_err()
    );
    let candidate = edited(&first);
    let second = update_in_store(
        &store,
        &first,
        &candidate,
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    assert_eq!(second.token().generation(), 2);
    assert!(
        update_in_store(
            &store,
            &first,
            &edited(&first),
            crate::workbench::commands::vocabulary::command_registry()
        )
        .is_err()
    );
}

#[test]
fn late_external_edits_are_not_clobbered() {
    let store = MemoryStore::default();
    let first = create_in_store(
        &store,
        &ShortcutProfileLibrary::default(),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    *store.bytes.borrow_mut() = Some(b"foreign late edit\n".to_vec());
    assert!(
        update_in_store(
            &store,
            &first,
            &edited(&first),
            crate::workbench::commands::vocabulary::command_registry()
        )
        .is_err()
    );
    assert_eq!(
        store.bytes.borrow().as_deref(),
        Some(&b"foreign late edit\n"[..])
    );
}

#[test]
fn future_and_corrupt_bytes_are_retained_and_never_initialized_over() {
    let store = MemoryStore::default();
    let future = canonical_json_bytes(serde_json::json!({
        "format": SHORTCUT_LIBRARY_STORE_FORMAT,
        "schema-version": 99,
        "generation": 7,
        "payload-digest": "future",
        "library": {},
    }))
    .unwrap();
    *store.bytes.borrow_mut() = Some(future.clone());
    let startup = startup_from_store(
        &store,
        Some(&ShortcutProfileLibrary::default()),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    let ShortcutProfileLibraryStartup::Incompatible(raw) = startup else {
        panic!("future store was not retained");
    };
    assert_eq!(raw.raw_bytes(), future);
    assert_eq!(store.bytes.borrow().as_deref(), Some(future.as_slice()));

    let corrupt = b"{not-json".to_vec();
    *store.bytes.borrow_mut() = Some(corrupt.clone());
    assert!(matches!(
        startup_from_store(
            &store,
            Some(&ShortcutProfileLibrary::default()),
            crate::workbench::commands::vocabulary::command_registry()
        )
        .unwrap(),
        ShortcutProfileLibraryStartup::Corrupt(_)
    ));
    assert_eq!(store.bytes.borrow().as_deref(), Some(corrupt.as_slice()));
}

#[test]
fn missing_store_may_initialize_once_from_compatible_session() {
    let store = MemoryStore::default();
    let startup = startup_from_store(
        &store,
        Some(&ShortcutProfileLibrary::default()),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    let ShortcutProfileLibraryStartup::Ready {
        persisted,
        initialized_from_session,
    } = startup
    else {
        panic!("compatible session was not initialized");
    };
    assert!(initialized_from_session);
    assert_eq!(persisted.token().generation(), 1);
}

#[test]
fn privacy_contract_rejects_project_paths_macros_and_credentials() {
    for field in [
        "project-id",
        "source-path",
        "recent-files",
        "macro-source",
        "credential-token",
        "access-token",
    ] {
        let mut value = serde_json::to_value(ShortcutProfileLibrary::default()).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert(field.to_owned(), Value::String("private".to_owned()));
        assert!(
            validate_device_local_privacy(&value).is_err(),
            "accepted {field}"
        );
    }
    let allowed = serde_json::json!({
        "commands": {"open-project": {"bindings": []}},
        "protected-override-acknowledgements": ["save-project"]
    });
    validate_device_local_privacy(&allowed).unwrap();
}

#[test]
fn current_envelope_with_revision_max_is_retained_as_incompatible() {
    let mut value = serde_json::to_value(ShortcutProfileLibrary::default()).unwrap();
    value["revision"] = Value::from(u64::MAX);
    let payload = canonical_json_bytes(value.clone()).unwrap();
    let bytes = canonical_json_bytes(serde_json::json!({
        "format": SHORTCUT_LIBRARY_STORE_FORMAT,
        "schema-version": SHORTCUT_LIBRARY_STORE_SCHEMA_VERSION,
        "generation": 9,
        "payload-digest": hex_digest(sha256(&payload)),
        "library": value,
    }))
    .unwrap();
    assert!(matches!(
        decode_store_bytes(
            &bytes,
            crate::workbench::commands::vocabulary::command_registry()
        ),
        ShortcutProfileLibraryRestore::Incompatible(_)
    ));
}

#[test]
fn byte_limit_fails_before_store_mutation() {
    let store = MemoryStore::default();
    let oversized = "x".repeat(MAX_SHORTCUT_LIBRARY_STORE_BYTES);
    let library: ShortcutProfileLibrary = serde_json::from_value(serde_json::json!({
        "library-version": 1,
        "active": {"future-field": oversized},
        "named-presets": {},
        "revision": 0
    }))
    .unwrap();
    let error = create_in_store(
        &store,
        &library,
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap_err();
    assert_eq!(error.code(), "shortcut-library.size-limit");
    assert!(store.bytes.borrow().is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_path_backend_round_trips_and_detects_cas_conflict() {
    let root = std::env::temp_dir().join(format!("rspice-shortcut-store-{}", uuid::Uuid::new_v4()));
    let store = NativeShortcutLibraryStore {
        path: root.join(NATIVE_FILENAME),
    };
    let first = create_in_store(
        &store,
        &ShortcutProfileLibrary::default(),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    let restored = restore_from_store(
        &store,
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    assert!(matches!(
        restored,
        ShortcutProfileLibraryRestore::Compatible(_)
    ));
    std::fs::write(&store.path, b"late external edit\n").unwrap();
    assert!(
        update_in_store(
            &store,
            &first,
            &edited(&first),
            crate::workbench::commands::vocabulary::command_registry()
        )
        .is_err()
    );
    assert_eq!(std::fs::read(&store.path).unwrap(), b"late external edit\n");
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn browser_write_state_retains_completion_until_exact_token_poll() {
    let mut state = BrowserWriteLeaseState::default();
    let token = state.begin().unwrap();
    assert!(state.is_current(token));
    assert!(state.complete(token));
    assert!(!state.is_current(token));
    assert!(state.take_completed(token));
    assert!(!state.take_completed(token));
}

#[test]
fn browser_write_state_ignores_cancelled_and_stale_completions() {
    let mut state = BrowserWriteLeaseState::default();
    let cancelled = state.begin().unwrap();
    assert!(state.cancel(cancelled));
    let replacement = state.begin().unwrap();
    assert_ne!(cancelled, replacement);
    assert!(!state.complete(cancelled));
    assert!(state.is_current(replacement));
    assert!(state.complete(replacement));
    assert!(state.take_completed(replacement));
}

#[test]
fn browser_write_state_allows_only_one_owner_until_poll_or_cancel() {
    let mut state = BrowserWriteLeaseState::default();
    let first = state.begin().unwrap();
    assert_eq!(
        state.begin().unwrap_err().code(),
        "shortcut-library.browser-write-pending"
    );
    assert!(state.complete(first));
    assert_eq!(
        state.begin().unwrap_err().code(),
        "shortcut-library.browser-write-pending"
    );
    assert!(state.take_completed(first));
    assert!(state.begin().is_ok());
}

#[test]
fn verified_browser_commit_cannot_be_cancelled_before_cleanup_completes() {
    let mut state = BrowserWriteLeaseState::default();
    let cancellable = state.begin().unwrap();
    assert!(state.cancel(cancellable));

    let committed = state.begin().unwrap();
    assert!(state.mark_commit_verified(committed));
    assert!(!state.cancel(committed));
    assert!(state.complete(committed));
    assert!(!state.cancel(committed));
    assert!(state.take_completed(committed));
    assert!(state.begin().is_ok());
}

#[test]
fn verified_browser_commit_is_authoritative_even_if_lock_release_fails() {
    let release_error = ShortcutLibraryPersistenceError::new(
        "shortcut-library.browser-lock",
        "simulated release failure",
    );
    assert!(finish_verified_browser_commit(Ok(()), Err(release_error)).is_ok());

    let commit_error = ShortcutLibraryPersistenceError::new(
        "shortcut-library.browser-publish",
        "simulated commit failure",
    );
    assert_eq!(
        finish_verified_browser_commit(Err(commit_error.clone()), Ok(())).unwrap_err(),
        commit_error
    );
}

#[test]
fn commit_in_doubt_errors_are_typed_and_never_retryable_as_ordinary_failures() {
    let error = ShortcutLibraryPersistenceError::new(
        "shortcut-library.browser-commit-in-doubt",
        "simulated post-set verification failure",
    );
    assert!(error.is_commit_in_doubt());
    assert!(
        !ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-publish",
            "simulated pre-commit failure",
        )
        .is_commit_in_doubt()
    );
}
