//! Runtime ownership of the canonical device-local shortcut library.
//!
//! The eframe session contains a recoverable shortcut snapshot, but it is not
//! publication authority. This owner restores the canonical CAS predecessor
//! before shortcut dispatch and retains incompatible/corrupt bytes without
//! overwriting them. Browser initialization is asynchronous and remains
//! mutation-blocked until its exact token completes.

use crate::workbench::framework::command_registry::Registry;
use crate::workbench::shortcuts::library_persistence::{
    PersistedShortcutProfileLibrary, RetainedShortcutLibraryBytes,
};

use crate::workbench::app_state::Model;
use crate::workbench::framework::requests::{Outbox, ResetShortcutResolver};

#[derive(Debug, Clone, Default)]
pub enum ShortcutLibraryPersistenceRuntime {
    #[default]
    Uninitialized,
    Ready {
        persisted: Box<PersistedShortcutProfileLibrary>,
        /// Browser publications complete between frames. The owning workflow
        /// consumes this once after the durable snapshot has become live.
        publication_completion: Option<Result<(), String>>,
        #[cfg(any(test, feature = "test-support"))]
        volatile_test: bool,
    },
    #[cfg(target_arch = "wasm32")]
    Initializing(
        crate::workbench::shortcuts::library_persistence::BrowserShortcutLibraryWriteToken,
    ),
    #[cfg(target_arch = "wasm32")]
    Publishing {
        predecessor: Box<PersistedShortcutProfileLibrary>,
        token: crate::workbench::shortcuts::library_persistence::BrowserShortcutLibraryWriteToken,
    },
    Missing,
    Incompatible(RetainedShortcutLibraryBytes),
    Corrupt(RetainedShortcutLibraryBytes),
    Unavailable(String),
    #[cfg(target_arch = "wasm32")]
    CommitInDoubt {
        message: String,
        publication_completion: Option<Result<(), String>>,
    },
}

impl ShortcutLibraryPersistenceRuntime {
    pub fn persisted(&self) -> Option<&PersistedShortcutProfileLibrary> {
        match self {
            Self::Ready { persisted, .. } => Some(persisted.as_ref()),
            _ => None,
        }
    }

    pub(crate) fn blocked_reason(&self) -> Option<String> {
        match self {
            Self::Uninitialized => Some("shortcut library storage is not initialized".to_owned()),
            Self::Ready {
                publication_completion: Some(_),
                ..
            } => Some("shortcut library publication is awaiting acknowledgement".to_owned()),
            Self::Ready {
                publication_completion: None,
                ..
            } => None,
            #[cfg(target_arch = "wasm32")]
            Self::Initializing(_) => {
                Some("shortcut library storage is still initializing".to_owned())
            }
            #[cfg(target_arch = "wasm32")]
            Self::Publishing { .. } => Some("shortcut library changes are being saved".to_owned()),
            Self::Missing => Some("the saved shortcut library is missing".to_owned()),
            Self::Incompatible(raw) => Some(format!(
                "the saved shortcut library is from an incompatible version ({})",
                digest_prefix(raw.digest())
            )),
            Self::Corrupt(raw) => Some(format!(
                "the saved shortcut library is corrupt ({})",
                digest_prefix(raw.digest())
            )),
            Self::Unavailable(message) => Some(message.clone()),
            #[cfg(target_arch = "wasm32")]
            Self::CommitInDoubt { message, .. } => Some(message.clone()),
        }
    }
}

/// Result of a persist-before-live shortcut-library publication request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutLibraryPublication {
    Published,
    #[cfg_attr(
        not(target_arch = "wasm32"),
        allow(dead_code, reason = "browser CAS publication completes asynchronously")
    )]
    Pending,
}

/// Exact UI transaction awaiting an asynchronous browser publication.
#[derive(Debug, Clone)]
pub enum ShortcutLibraryPublicationContinuation {
    Editor,
    Policy,
    Import(Box<crate::workbench::shortcuts::artifacts::ShortcutImportReceipt>),
    /// A settings file whose preferences go live once its shortcuts are saved.
    SettingsImport(Box<super::preferences_file::StagedSettingsImport>),
}

/// What the shortcut library's persistence reads and writes, borrowed from
/// the session: the library in use, in the model's preferences, and its
/// store's standing. Its failures go to the model's log. Putting a library in
/// use asks the shell to reset the shortcut resolver, so a chord half-typed
/// under the previous library does not complete under this one.
pub struct ShortcutLibraryParts<'a> {
    pub persistence: &'a mut ShortcutLibraryPersistenceRuntime,
    pub model: &'a mut Model,
    pub outbox: &'a mut Outbox,
}

impl ShortcutLibraryParts<'_> {
    pub fn initialize_shortcut_library_persistence(
        &mut self,
        ctx: &egui::Context,
        registry: &Registry,
    ) {
        let session_library = self.model.preferences.shortcut_profiles().clone();

        #[cfg(not(target_arch = "wasm32"))]
        let _ = ctx;

        #[cfg(not(target_arch = "wasm32"))]
        let startup =
            crate::workbench::shortcuts::library_persistence::startup_shortcut_profile_library_native(
                Some(&session_library),
                registry,
            )
            .map(NativeOrBrowserStartup::Native);

        #[cfg(target_arch = "wasm32")]
        let startup =
            crate::workbench::shortcuts::library_persistence::startup_shortcut_profile_library_browser(
                Some(&session_library),
                ctx,
                registry,
            )
            .map(NativeOrBrowserStartup::Browser);

        match startup {
            Ok(startup) => self.install_shortcut_library_startup(startup),
            Err(error) => {
                self.record_shortcut_library_unavailable(&error);
                *self.persistence =
                    ShortcutLibraryPersistenceRuntime::Unavailable(error.to_string());
            }
        }
        self.report_shortcut_library_persistence_state();
    }

    fn install_persisted_shortcut_library(
        &mut self,
        persisted: PersistedShortcutProfileLibrary,
        publication_completion: Option<Result<(), String>>,
    ) {
        *self.model.preferences.shortcut_profiles_mut() = persisted.library().clone();
        self.outbox.push(ResetShortcutResolver);
        *self.persistence = ShortcutLibraryPersistenceRuntime::Ready {
            persisted: Box::new(persisted),
            publication_completion,
            #[cfg(any(test, feature = "test-support"))]
            volatile_test: false,
        };
    }

    /// Publish one fully-validated candidate with exact CAS authority. The
    /// live profile is replaced only by the durable result returned from the
    /// persistence backend.
    pub fn publish_shortcut_library_candidate(
        &mut self,
        candidate: &crate::state::shortcuts::ShortcutProfileLibrary,
        ctx: &egui::Context,
        registry: &Registry,
    ) -> Result<ShortcutLibraryPublication, String> {
        if let Some(reason) = self.persistence.blocked_reason() {
            return Err(reason);
        }
        let predecessor = self.persistence.persisted().cloned().ok_or_else(|| {
            self.persistence
                .blocked_reason()
                .unwrap_or_else(|| "shortcut library storage is unavailable".to_owned())
        })?;
        if predecessor.library() != self.model.preferences.shortcut_profiles() {
            return Err("the shortcut profile in use no longer matches the saved copy".to_owned());
        }
        if predecessor.library() == candidate {
            return Ok(ShortcutLibraryPublication::Published);
        }

        #[cfg(any(test, feature = "test-support"))]
        if matches!(
            *self.persistence,
            ShortcutLibraryPersistenceRuntime::Ready {
                volatile_test: true,
                ..
            }
        ) {
            let generation = predecessor.token().generation().saturating_add(1);
            let persisted =
                PersistedShortcutProfileLibrary::test_snapshot(candidate.clone(), generation);
            *self.model.preferences.shortcut_profiles_mut() = candidate.clone();
            self.outbox.push(ResetShortcutResolver);
            *self.persistence = ShortcutLibraryPersistenceRuntime::Ready {
                persisted: Box::new(persisted),
                publication_completion: None,
                volatile_test: true,
            };
            return Ok(ShortcutLibraryPublication::Published);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = ctx;
            let persisted =
                crate::workbench::shortcuts::library_persistence::update_shortcut_profile_library_native(
                    &predecessor,
                    candidate,
                    registry,
                )
                .map_err(|error| error.to_string())?;
            self.install_persisted_shortcut_library(persisted, None);
            Ok(ShortcutLibraryPublication::Published)
        }

        #[cfg(target_arch = "wasm32")]
        {
            let token = crate::workbench::shortcuts::library_persistence::start_update_shortcut_profile_library_browser(
                &predecessor,
                candidate,
                ctx,
                registry,
            )
            .map_err(|error| error.to_string())?;
            *self.persistence = ShortcutLibraryPersistenceRuntime::Publishing {
                predecessor: Box::new(predecessor),
                token,
            };
            Ok(ShortcutLibraryPublication::Pending)
        }
    }

    /// Consume the result of a browser publication after the persisted
    /// candidate has become the live shortcut library.
    pub(crate) fn take_shortcut_library_publication_completion(
        &mut self,
    ) -> Option<Result<(), String>> {
        match &mut *self.persistence {
            ShortcutLibraryPersistenceRuntime::Ready {
                publication_completion,
                ..
            } => publication_completion.take(),
            #[cfg(target_arch = "wasm32")]
            ShortcutLibraryPersistenceRuntime::CommitInDoubt {
                publication_completion,
                ..
            } => publication_completion.take(),
            _ => None,
        }
    }

    #[must_use]
    pub fn shortcut_library_publication_completion_pending(&self) -> bool {
        match &*self.persistence {
            ShortcutLibraryPersistenceRuntime::Ready {
                publication_completion,
                ..
            } => publication_completion.is_some(),
            #[cfg(target_arch = "wasm32")]
            ShortcutLibraryPersistenceRuntime::CommitInDoubt {
                publication_completion,
                ..
            } => publication_completion.is_some(),
            _ => false,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn cancel_pending_shortcut_library_publication(&mut self) -> bool {
        let (predecessor, token) = match &*self.persistence {
            ShortcutLibraryPersistenceRuntime::Publishing { predecessor, token } => {
                (predecessor.clone(), *token)
            }
            _ => return false,
        };
        if !crate::workbench::shortcuts::library_persistence::cancel_shortcut_profile_library_browser_write(
            token,
        ) {
            return false;
        }
        *self.persistence = ShortcutLibraryPersistenceRuntime::Ready {
            persisted: predecessor,
            publication_completion: None,
            #[cfg(any(test, feature = "test-support"))]
            volatile_test: false,
        };
        true
    }

    #[cfg(target_arch = "wasm32")]
    pub fn poll_shortcut_library_persistence(&mut self) {
        let (token, predecessor) = match &*self.persistence {
            ShortcutLibraryPersistenceRuntime::Initializing(token) => (*token, None),
            ShortcutLibraryPersistenceRuntime::Publishing { predecessor, token } => {
                (*token, Some(predecessor.clone()))
            }
            _ => return,
        };
        let Some(result) =
            crate::workbench::shortcuts::library_persistence::poll_shortcut_profile_library_browser_write(
                token,
            )
        else {
            return;
        };
        match result {
            Ok(persisted) => {
                let completion = predecessor.is_some().then_some(Ok(()));
                self.install_persisted_shortcut_library(persisted, completion);
            }
            Err(error) => {
                if let Some(predecessor) = predecessor {
                    self.model.record(
                        crate::diagnostics::LogSeverity::Error,
                        crate::diagnostics::LogSource::Studio,
                        format!("Shortcut changes could not be saved: {error}"),
                    );
                    if error.is_commit_in_doubt() {
                        let message = format!(
                            "RSpice cannot tell whether the last shortcut change was saved, so shortcuts are read-only until RSpice reloads them: {error}"
                        );
                        *self.persistence = ShortcutLibraryPersistenceRuntime::CommitInDoubt {
                            publication_completion: Some(Err(message.clone())),
                            message,
                        };
                    } else {
                        *self.persistence = ShortcutLibraryPersistenceRuntime::Ready {
                            persisted: predecessor,
                            publication_completion: Some(Err(error.to_string())),
                            #[cfg(any(test, feature = "test-support"))]
                            volatile_test: false,
                        };
                    }
                } else {
                    self.record_shortcut_library_unavailable(&error);
                    *self.persistence =
                        ShortcutLibraryPersistenceRuntime::Unavailable(error.to_string());
                }
            }
        }
        self.report_shortcut_library_persistence_state();
    }

    pub fn install_shortcut_library_startup(&mut self, startup: NativeOrBrowserStartup) {
        match startup {
            #[cfg(not(target_arch = "wasm32"))]
            NativeOrBrowserStartup::Native(startup) => match startup {
                crate::workbench::shortcuts::library_persistence::ShortcutProfileLibraryStartup::Ready {
                    persisted,
                    ..
                } => self.install_persisted_shortcut_library(*persisted, None),
                crate::workbench::shortcuts::library_persistence::ShortcutProfileLibraryStartup::Missing => {
                    *self.persistence =
                        ShortcutLibraryPersistenceRuntime::Missing;
                }
                crate::workbench::shortcuts::library_persistence::ShortcutProfileLibraryStartup::Incompatible(raw) => {
                    self.record_shortcut_library_unreadable("comes from an incompatible RSpice version", raw.reason());
                    *self.persistence =
                        ShortcutLibraryPersistenceRuntime::Incompatible(raw);
                }
                crate::workbench::shortcuts::library_persistence::ShortcutProfileLibraryStartup::Corrupt(raw) => {
                    self.record_shortcut_library_unreadable("is damaged", raw.reason());
                    *self.persistence =
                        ShortcutLibraryPersistenceRuntime::Corrupt(raw);
                }
            },
            #[cfg(target_arch = "wasm32")]
            NativeOrBrowserStartup::Browser(startup) => match startup {
                crate::workbench::shortcuts::library_persistence::BrowserShortcutProfileLibraryStartup::Ready(persisted) => {
                    self.install_persisted_shortcut_library(*persisted, None);
                }
                crate::workbench::shortcuts::library_persistence::BrowserShortcutProfileLibraryStartup::InitializationStarted(token) => {
                    *self.persistence =
                        ShortcutLibraryPersistenceRuntime::Initializing(token);
                }
                crate::workbench::shortcuts::library_persistence::BrowserShortcutProfileLibraryStartup::Missing => {
                    *self.persistence =
                        ShortcutLibraryPersistenceRuntime::Missing;
                }
                crate::workbench::shortcuts::library_persistence::BrowserShortcutProfileLibraryStartup::Incompatible(raw) => {
                    self.record_shortcut_library_unreadable("comes from an incompatible RSpice version", raw.reason());
                    *self.persistence =
                        ShortcutLibraryPersistenceRuntime::Incompatible(raw);
                }
                crate::workbench::shortcuts::library_persistence::BrowserShortcutProfileLibraryStartup::Corrupt(raw) => {
                    self.record_shortcut_library_unreadable("is damaged", raw.reason());
                    *self.persistence =
                        ShortcutLibraryPersistenceRuntime::Corrupt(raw);
                }
            },
        }
    }

    fn record_shortcut_library_unavailable(&mut self, error: &dyn std::fmt::Display) {
        self.model.record(
            crate::diagnostics::LogSeverity::Error,
            crate::diagnostics::LogSource::Studio,
            format!(
                "Keyboard shortcuts could not be loaded from storage, so changes to them cannot \
                 be saved: {error}"
            ),
        );
    }

    fn record_shortcut_library_unreadable(
        &mut self,
        condition: &str,
        reason: impl std::fmt::Display,
    ) {
        self.model.record(
            crate::diagnostics::LogSeverity::Error,
            crate::diagnostics::LogSource::Studio,
            format!(
                "The stored keyboard shortcut library {condition} and was not loaded: {reason}"
            ),
        );
    }

    fn report_shortcut_library_persistence_state(&self) {
        if let Some(persisted) = self.persistence.persisted() {
            debug_assert_eq!(
                persisted.library(),
                self.model.preferences.shortcut_profiles(),
                "live shortcuts must be the exact accepted canonical snapshot"
            );
            return;
        }
        #[cfg(target_arch = "wasm32")]
        if matches!(
            *self.persistence,
            ShortcutLibraryPersistenceRuntime::Initializing(_)
                | ShortcutLibraryPersistenceRuntime::Publishing { .. }
        ) {
            if let Some(reason) = self.persistence.blocked_reason() {
                log::debug!("Shortcut library persistence transition: {reason}");
            }
            return;
        }
        if let Some(reason) = self.persistence.blocked_reason() {
            log::warn!("Shortcut library mutations are unavailable: {reason}");
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn enable_volatile_test_shortcut_persistence(&mut self) {
        let library = self.model.preferences.shortcut_profiles().clone();
        *self.persistence = ShortcutLibraryPersistenceRuntime::Ready {
            persisted: Box::new(PersistedShortcutProfileLibrary::test_snapshot(library, 1)),
            publication_completion: None,
            volatile_test: true,
        };
    }
}

pub enum NativeOrBrowserStartup {
    #[cfg(not(target_arch = "wasm32"))]
    Native(crate::workbench::shortcuts::library_persistence::ShortcutProfileLibraryStartup),
    #[cfg(target_arch = "wasm32")]
    Browser(crate::workbench::shortcuts::library_persistence::BrowserShortcutProfileLibraryStartup),
}

fn digest_prefix(digest: [u8; 32]) -> String {
    digest[..6]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
