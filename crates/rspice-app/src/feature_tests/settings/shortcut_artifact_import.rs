//! Selecting and detecting a shortcut artifact to import, against the product's own commands.

#[cfg(not(target_arch = "wasm32"))]
use std::cell::Cell;

use crate::workbench::shortcuts::artifacts::import_io::*;
use crate::workbench::shortcuts::artifacts::{DetectedShortcutArtifact, VscodeHostPlatform};
use std::path::{Path, PathBuf};

const NATIVE_JSONC: &[u8] = br#"{
    // original source comment must survive as provenance
    "format": "rspice.shortcuts/1",
    "profile": { "commands": {}, },
}"#;

#[derive(Clone)]
#[cfg(not(target_arch = "wasm32"))]
struct FakeIo {
    platform: Result<VscodeHostPlatform, ShortcutArtifactImportError>,
    selected: Result<Option<PathBuf>, String>,
    bytes: Result<Vec<u8>, String>,
    reads: Cell<usize>,
    picks: Cell<usize>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FakeIo {
    fn ready(bytes: Vec<u8>) -> Self {
        Self {
            platform: Ok(VscodeHostPlatform::Windows),
            selected: Ok(Some(PathBuf::from("private/keybindings.jsonc"))),
            bytes: Ok(bytes),
            reads: Cell::new(0),
            picks: Cell::new(0),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ShortcutArtifactImportIo for FakeIo {
    fn host_platform(&self) -> Result<VscodeHostPlatform, ShortcutArtifactImportError> {
        self.platform.clone()
    }

    fn show_open_dialog(&self) -> Result<Option<PathBuf>, String> {
        self.picks.set(self.picks.get() + 1);
        self.selected.clone()
    }

    fn read_exact_bytes(&self, _path: &Path) -> Result<Vec<u8>, String> {
        self.reads.set(self.reads.get() + 1);
        self.bytes.clone()
    }
}

fn ready(outcome: ShortcutArtifactImportOutcome) -> ReadyShortcutArtifactSource {
    let ShortcutArtifactImportOutcome::Ready(source) = outcome else {
        panic!("source must be ready")
    };
    *source
}

fn reset_browser_test_state() {
    if matches!(
        cancel_browser_shortcut_artifact_import(),
        BrowserShortcutArtifactImportCancelOutcome::StillOwned
    ) {
        let _ = crate::workbench::browser::file_import::cancel_active_text_import();
    }
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
        *slot.borrow_mut() = None;
    });
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_TOKEN.with(|slot| slot.set(None));
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn native_cancel_is_explicit_and_does_not_read_or_claim_success() {
    let io = FakeIo {
        selected: Ok(None),
        ..FakeIo::ready(Vec::new())
    };

    let outcome = import_shortcut_artifact_source_with_io(
        &io,
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .expect("cancel is valid");

    assert_eq!(outcome, ShortcutArtifactImportOutcome::Cancelled);
    assert_eq!(io.picks.get(), 1);
    assert_eq!(io.reads.get(), 0);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn native_jsonc_retains_exact_bytes_digest_and_stable_filename() {
    let io = FakeIo::ready(NATIVE_JSONC.to_vec());

    let source = ready(
        import_shortcut_artifact_source_with_io(
            &io,
            crate::workbench::commands::vocabulary::command_registry(),
        )
        .expect("source detects"),
    );

    assert_eq!(source.source_name(), "keybindings.jsonc");
    assert_eq!(source.source_bytes(), NATIVE_JSONC);
    assert_eq!(
        source.source_digest(),
        crate::workbench::shortcuts::artifacts::sha256(NATIVE_JSONC)
    );
    let DetectedShortcutArtifact::RSpice(artifact) = source.detected() else {
        panic!("native object must remain a native RSpice artifact")
    };
    assert_eq!(artifact.artifact().source_name(), "keybindings.jsonc");
    assert_eq!(artifact.artifact().source_digest(), source.source_digest());
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn native_boundary_classifies_picker_read_limit_utf8_and_detection_errors() {
    let picker = FakeIo {
        selected: Err("dialog unavailable".to_owned()),
        ..FakeIo::ready(Vec::new())
    };
    assert_eq!(
        import_shortcut_artifact_source_with_io(
            &picker,
            crate::workbench::commands::vocabulary::command_registry()
        )
        .unwrap_err()
        .kind(),
        ShortcutArtifactImportErrorKind::Picker
    );

    let read = FakeIo {
        bytes: Err("media failure".to_owned()),
        ..FakeIo::ready(Vec::new())
    };
    assert_eq!(
        import_shortcut_artifact_source_with_io(
            &read,
            crate::workbench::commands::vocabulary::command_registry()
        )
        .unwrap_err()
        .kind(),
        ShortcutArtifactImportErrorKind::Read
    );

    let oversized = FakeIo::ready(vec![
        b' ';
        usize::try_from(MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES + 1)
            .expect("test limit fits usize")
    ]);
    assert_eq!(
        import_shortcut_artifact_source_with_io(
            &oversized,
            crate::workbench::commands::vocabulary::command_registry()
        )
        .unwrap_err()
        .kind(),
        ShortcutArtifactImportErrorKind::ByteLimit
    );

    let utf8 = FakeIo::ready(vec![0xff]);
    assert_eq!(
        import_shortcut_artifact_source_with_io(
            &utf8,
            crate::workbench::commands::vocabulary::command_registry()
        )
        .unwrap_err()
        .kind(),
        ShortcutArtifactImportErrorKind::InvalidUtf8
    );

    let detection = FakeIo::ready(b"not json".to_vec());
    let error = import_shortcut_artifact_source_with_io(
        &detection,
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap_err();
    assert_eq!(error.kind(), ShortcutArtifactImportErrorKind::Detection);
    assert_eq!(error.cause_code(), Some("shortcut-adapter.invalid-json"));
    assert_eq!(error.source_name(), Some("keybindings.jsonc"));
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn unsupported_platform_fails_before_picker_or_read() {
    let io = FakeIo {
        platform: Err(ShortcutArtifactImportError::new(
            ShortcutArtifactImportErrorKind::UnsupportedPlatform,
            "unsupported test host",
        )),
        ..FakeIo::ready(NATIVE_JSONC.to_vec())
    };

    let error = import_shortcut_artifact_source_with_io(
        &io,
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        ShortcutArtifactImportErrorKind::UnsupportedPlatform
    );
    assert_eq!(io.picks.get(), 0);
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn platform_labels_map_only_windows_macos_and_linux() {
    for (label, expected) in [
        ("Win32", VscodeHostPlatform::Windows),
        ("Windows", VscodeHostPlatform::Windows),
        ("MacIntel", VscodeHostPlatform::Macos),
        ("macOS", VscodeHostPlatform::Macos),
        ("iPhone", VscodeHostPlatform::Macos),
        ("iPad", VscodeHostPlatform::Macos),
        ("iPod", VscodeHostPlatform::Macos),
        ("Linux x86_64", VscodeHostPlatform::Linux),
        ("Android", VscodeHostPlatform::Linux),
        ("Android armv8l", VscodeHostPlatform::Linux),
    ] {
        assert_eq!(platform_from_label(label).unwrap(), expected);
    }
    for unsupported in ["", "FreeBSD", "Chrome OS"] {
        assert_eq!(
            platform_from_label(unsupported).unwrap_err().kind(),
            ShortcutArtifactImportErrorKind::UnsupportedPlatform
        );
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn detection_is_deterministic_and_does_not_mutate_live_shortcuts() {
    let live = crate::state::shortcuts::ShortcutProfileLibrary::default();
    let before = serde_json::to_value(&live).expect("live library serializes");
    let first = import_shortcut_artifact_source_with_io(
        &FakeIo::ready(NATIVE_JSONC.to_vec()),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .expect("first detects");
    let second = import_shortcut_artifact_source_with_io(
        &FakeIo::ready(NATIVE_JSONC.to_vec()),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .expect("second detects");

    assert_eq!(first, second);
    assert_eq!(
        serde_json::to_value(&live).expect("live library still serializes"),
        before
    );
}

#[test]
fn browser_picker_cancel_polls_as_explicit_cancelled_outcome() {
    reset_browser_test_state();
    let token = begin_browser_shortcut_artifact_import().expect("lease starts");
    let result = browser_picker_result(
        VscodeHostPlatform::Windows,
        Ok(None),
        crate::workbench::commands::vocabulary::command_registry(),
    );
    assert!(complete_browser_shortcut_artifact_import(token, result));

    assert_eq!(
        poll_browser_shortcut_artifact_import(),
        Some(Ok(ShortcutArtifactImportOutcome::Cancelled))
    );
    reset_browser_test_state();
}

#[test]
fn stale_browser_completion_cannot_replace_or_release_current_lease() {
    reset_browser_test_state();
    let stale = begin_browser_shortcut_artifact_import().expect("first lease starts");
    assert_eq!(
        crate::workbench::browser::file_import::cancel_active_text_import(),
        Some(stale)
    );
    let current = begin_browser_shortcut_artifact_import().expect("replacement starts");

    assert!(!complete_browser_shortcut_artifact_import(
        stale,
        Ok(ShortcutArtifactImportOutcome::Cancelled)
    ));
    assert!(poll_browser_shortcut_artifact_import().is_none());
    assert!(crate::workbench::browser::file_import::text_import_is_current(current));

    assert!(complete_browser_shortcut_artifact_import(
        current,
        Ok(ShortcutArtifactImportOutcome::Cancelled)
    ));
    assert_eq!(
        poll_browser_shortcut_artifact_import(),
        Some(Ok(ShortcutArtifactImportOutcome::Cancelled))
    );
    reset_browser_test_state();
}

#[test]
fn owned_browser_cancel_clears_completion_and_makes_late_callback_inert() {
    reset_browser_test_state();
    let cancelled = begin_browser_shortcut_artifact_import().expect("lease starts");
    assert!(complete_browser_shortcut_artifact_import(
        cancelled,
        Ok(ShortcutArtifactImportOutcome::Cancelled)
    ));

    assert_eq!(
        cancel_browser_shortcut_artifact_import(),
        BrowserShortcutArtifactImportCancelOutcome::Cancelled
    );
    assert!(poll_browser_shortcut_artifact_import().is_none());
    assert!(!complete_browser_shortcut_artifact_import(
        cancelled,
        Ok(ShortcutArtifactImportOutcome::Cancelled)
    ));

    let replacement = begin_browser_shortcut_artifact_import().expect("replacement starts");
    assert!(crate::workbench::browser::file_import::text_import_is_current(replacement));
    assert!(!complete_browser_shortcut_artifact_import(
        cancelled,
        Ok(ShortcutArtifactImportOutcome::Cancelled)
    ));
    assert!(crate::workbench::browser::file_import::text_import_is_current(replacement));
    assert_eq!(
        cancel_browser_shortcut_artifact_import(),
        BrowserShortcutArtifactImportCancelOutcome::Cancelled
    );
    reset_browser_test_state();
}

#[test]
fn browser_cancel_never_cancels_another_workflows_lease() {
    reset_browser_test_state();
    let other = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Project,
    )
    .expect("other workflow starts");

    assert_eq!(
        cancel_browser_shortcut_artifact_import(),
        BrowserShortcutArtifactImportCancelOutcome::AlreadyReleased
    );
    assert!(crate::workbench::browser::file_import::text_import_is_current(other));
    assert!(crate::workbench::browser::file_import::finish_text_import(
        other
    ));
    reset_browser_test_state();
}

#[test]
fn stale_shortcut_picker_cancel_is_a_terminal_released_outcome() {
    reset_browser_test_state();
    let stale = begin_browser_shortcut_artifact_import().expect("shortcut picker starts");
    assert_eq!(
        crate::workbench::browser::file_import::cancel_active_text_import(),
        Some(stale)
    );

    assert_eq!(
        cancel_browser_shortcut_artifact_import(),
        BrowserShortcutArtifactImportCancelOutcome::AlreadyReleased
    );
    assert!(poll_browser_shortcut_artifact_import().is_none());
    reset_browser_test_state();
}

#[test]
fn browser_read_contract_preserves_typed_limit_utf8_and_detection_failures() {
    let limit = browser_picker_result(
        VscodeHostPlatform::Windows,
        Err(format!(
            "Selected huge.json exceeds the supported {MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES}-byte size limit"
        )),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap_err();
    assert_eq!(limit.kind(), ShortcutArtifactImportErrorKind::ByteLimit);

    let utf8 = browser_picker_result(
        VscodeHostPlatform::Windows,
        Err("Selected file is not valid UTF-8: invalid byte".to_owned()),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap_err();
    assert_eq!(utf8.kind(), ShortcutArtifactImportErrorKind::InvalidUtf8);

    let detection = browser_picker_result(
        VscodeHostPlatform::Linux,
        Ok(Some(
            crate::workbench::browser::file_import::PickedTextFile {
                name: "keybindings.jsonc".to_owned(),
                contents: "[invalid]".to_owned(),
                original_bytes: None,
            },
        )),
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap_err();
    assert_eq!(detection.kind(), ShortcutArtifactImportErrorKind::Detection);
    assert_eq!(
        detection.cause_code(),
        Some("shortcut-adapter.invalid-json")
    );
}
