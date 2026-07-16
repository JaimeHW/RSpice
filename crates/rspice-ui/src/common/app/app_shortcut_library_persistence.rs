//! Runtime ownership of the canonical device-local shortcut library.
//!
//! The eframe session contains a recoverable shortcut snapshot, but it is not
//! publication authority. This owner restores the canonical CAS predecessor
//! before shortcut dispatch and retains incompatible/corrupt bytes without
//! overwriting them. Browser initialization is asynchronous and remains
//! mutation-blocked until its exact token completes.

use crate::common::shortcut_library_persistence::{
    PersistedShortcutProfileLibrary, RetainedShortcutLibraryBytes,
};

use super::AppState;

#[derive(Debug, Clone, Default)]
pub(crate) enum ShortcutLibraryPersistenceRuntime {
    #[default]
    Uninitialized,
    Ready {
        persisted: Box<PersistedShortcutProfileLibrary>,
        /// Browser publications complete between frames. The owning workflow
        /// consumes this once after the durable snapshot has become live.
        publication_completion: Option<Result<(), String>>,
        #[cfg(test)]
        volatile_test: bool,
    },
    #[cfg(target_arch = "wasm32")]
    Initializing(crate::common::shortcut_library_persistence::BrowserShortcutLibraryWriteToken),
    #[cfg(target_arch = "wasm32")]
    Publishing {
        predecessor: Box<PersistedShortcutProfileLibrary>,
        token: crate::common::shortcut_library_persistence::BrowserShortcutLibraryWriteToken,
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
    pub(crate) fn persisted(&self) -> Option<&PersistedShortcutProfileLibrary> {
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
            Self::Missing => Some("canonical shortcut library storage is missing".to_owned()),
            Self::Incompatible(raw) => Some(format!(
                "canonical shortcut library is from an incompatible version ({})",
                digest_prefix(raw.digest())
            )),
            Self::Corrupt(raw) => Some(format!(
                "canonical shortcut library is corrupt ({})",
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
pub(crate) enum ShortcutLibraryPublication {
    Published,
    #[cfg_attr(
        not(target_arch = "wasm32"),
        allow(dead_code, reason = "browser CAS publication completes asynchronously")
    )]
    Pending,
}

/// Exact UI transaction awaiting an asynchronous browser publication.
#[derive(Debug, Clone)]
pub(crate) enum ShortcutLibraryPublicationContinuation {
    Editor,
    Policy,
    Import(Box<crate::common::shortcut_artifacts::ShortcutImportReceipt>),
    Rollback,
}

impl AppState {
    pub(crate) fn initialize_shortcut_library_persistence(&mut self, ctx: &egui::Context) {
        let session_library = self.ui.preferences.shortcut_profiles().clone();

        #[cfg(not(target_arch = "wasm32"))]
        let _ = ctx;

        #[cfg(not(target_arch = "wasm32"))]
        let startup =
            crate::common::shortcut_library_persistence::startup_shortcut_profile_library_native(
                Some(&session_library),
            )
            .map(NativeOrBrowserStartup::Native);

        #[cfg(target_arch = "wasm32")]
        let startup =
            crate::common::shortcut_library_persistence::startup_shortcut_profile_library_browser(
                Some(&session_library),
                ctx,
            )
            .map(NativeOrBrowserStartup::Browser);

        match startup {
            Ok(startup) => self.install_shortcut_library_startup(startup),
            Err(error) => {
                log::error!("Could not initialize canonical shortcut library: {error}");
                self.shortcut_library_persistence =
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
        *self.ui.preferences.shortcut_profiles_mut() = persisted.library().clone();
        self.shortcut_resolver.reset();
        self.shortcut_library_persistence = ShortcutLibraryPersistenceRuntime::Ready {
            persisted: Box::new(persisted),
            publication_completion,
            #[cfg(test)]
            volatile_test: false,
        };
    }

    /// Publish one fully-validated candidate with exact CAS authority. The
    /// live profile is replaced only by the durable result returned from the
    /// persistence backend.
    pub(crate) fn publish_shortcut_library_candidate(
        &mut self,
        candidate: &crate::workbench::shortcuts::ShortcutProfileLibrary,
        ctx: &egui::Context,
    ) -> Result<ShortcutLibraryPublication, String> {
        if let Some(reason) = self.shortcut_library_persistence.blocked_reason() {
            return Err(reason);
        }
        let predecessor = self
            .shortcut_library_persistence
            .persisted()
            .cloned()
            .ok_or_else(|| {
                self.shortcut_library_persistence
                    .blocked_reason()
                    .unwrap_or_else(|| "shortcut library storage is unavailable".to_owned())
            })?;
        if predecessor.library() != self.ui.preferences.shortcut_profiles() {
            return Err(
                "the live shortcut profile no longer matches its canonical storage snapshot"
                    .to_owned(),
            );
        }
        if predecessor.library() == candidate {
            return Ok(ShortcutLibraryPublication::Published);
        }

        #[cfg(test)]
        if matches!(
            self.shortcut_library_persistence,
            ShortcutLibraryPersistenceRuntime::Ready {
                volatile_test: true,
                ..
            }
        ) {
            let generation = predecessor.token().generation().saturating_add(1);
            let persisted =
                PersistedShortcutProfileLibrary::test_snapshot(candidate.clone(), generation);
            *self.ui.preferences.shortcut_profiles_mut() = candidate.clone();
            self.shortcut_resolver.reset();
            self.shortcut_library_persistence = ShortcutLibraryPersistenceRuntime::Ready {
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
                crate::common::shortcut_library_persistence::update_shortcut_profile_library_native(
                    &predecessor,
                    candidate,
                )
                .map_err(|error| error.to_string())?;
            self.install_persisted_shortcut_library(persisted, None);
            Ok(ShortcutLibraryPublication::Published)
        }

        #[cfg(target_arch = "wasm32")]
        {
            let token = crate::common::shortcut_library_persistence::start_update_shortcut_profile_library_browser(
                &predecessor,
                candidate,
                ctx,
            )
            .map_err(|error| error.to_string())?;
            self.shortcut_library_persistence = ShortcutLibraryPersistenceRuntime::Publishing {
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
        match &mut self.shortcut_library_persistence {
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
    pub(crate) fn shortcut_library_publication_completion_pending(&self) -> bool {
        match &self.shortcut_library_persistence {
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
        let (predecessor, token) = match &self.shortcut_library_persistence {
            ShortcutLibraryPersistenceRuntime::Publishing { predecessor, token } => {
                (predecessor.clone(), *token)
            }
            _ => return false,
        };
        if !crate::common::shortcut_library_persistence::cancel_shortcut_profile_library_browser_write(
            token,
        ) {
            return false;
        }
        self.shortcut_library_persistence = ShortcutLibraryPersistenceRuntime::Ready {
            persisted: predecessor,
            publication_completion: None,
            #[cfg(test)]
            volatile_test: false,
        };
        true
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_shortcut_library_persistence(&mut self) {
        let (token, predecessor) = match &self.shortcut_library_persistence {
            ShortcutLibraryPersistenceRuntime::Initializing(token) => (*token, None),
            ShortcutLibraryPersistenceRuntime::Publishing { predecessor, token } => {
                (*token, Some(predecessor.clone()))
            }
            _ => return,
        };
        let Some(result) =
            crate::common::shortcut_library_persistence::poll_shortcut_profile_library_browser_write(
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
                    log::error!("Could not publish canonical shortcut library: {error}");
                    if error.is_commit_in_doubt() {
                        let message = format!(
                            "Canonical shortcut storage is commit-in-doubt and has been made read-only until RSpice reloads it: {error}"
                        );
                        self.shortcut_library_persistence =
                            ShortcutLibraryPersistenceRuntime::CommitInDoubt {
                                publication_completion: Some(Err(message.clone())),
                                message,
                            };
                    } else {
                        self.shortcut_library_persistence =
                            ShortcutLibraryPersistenceRuntime::Ready {
                                persisted: predecessor,
                                publication_completion: Some(Err(error.to_string())),
                                #[cfg(test)]
                                volatile_test: false,
                            };
                    }
                } else {
                    log::error!("Could not initialize canonical shortcut library: {error}");
                    self.shortcut_library_persistence =
                        ShortcutLibraryPersistenceRuntime::Unavailable(error.to_string());
                }
            }
        }
        self.report_shortcut_library_persistence_state();
    }

    fn install_shortcut_library_startup(&mut self, startup: NativeOrBrowserStartup) {
        match startup {
            #[cfg(not(target_arch = "wasm32"))]
            NativeOrBrowserStartup::Native(startup) => match startup {
                crate::common::shortcut_library_persistence::ShortcutProfileLibraryStartup::Ready {
                    persisted,
                    ..
                } => self.install_persisted_shortcut_library(*persisted, None),
                crate::common::shortcut_library_persistence::ShortcutProfileLibraryStartup::Missing => {
                    self.shortcut_library_persistence =
                        ShortcutLibraryPersistenceRuntime::Missing;
                }
                crate::common::shortcut_library_persistence::ShortcutProfileLibraryStartup::Incompatible(raw) => {
                    log::error!("Canonical shortcut library is incompatible: {}", raw.reason());
                    self.shortcut_library_persistence =
                        ShortcutLibraryPersistenceRuntime::Incompatible(raw);
                }
                crate::common::shortcut_library_persistence::ShortcutProfileLibraryStartup::Corrupt(raw) => {
                    log::error!("Canonical shortcut library is corrupt: {}", raw.reason());
                    self.shortcut_library_persistence =
                        ShortcutLibraryPersistenceRuntime::Corrupt(raw);
                }
            },
            #[cfg(target_arch = "wasm32")]
            NativeOrBrowserStartup::Browser(startup) => match startup {
                crate::common::shortcut_library_persistence::BrowserShortcutProfileLibraryStartup::Ready(persisted) => {
                    self.install_persisted_shortcut_library(*persisted, None);
                }
                crate::common::shortcut_library_persistence::BrowserShortcutProfileLibraryStartup::InitializationStarted(token) => {
                    self.shortcut_library_persistence =
                        ShortcutLibraryPersistenceRuntime::Initializing(token);
                }
                crate::common::shortcut_library_persistence::BrowserShortcutProfileLibraryStartup::Missing => {
                    self.shortcut_library_persistence =
                        ShortcutLibraryPersistenceRuntime::Missing;
                }
                crate::common::shortcut_library_persistence::BrowserShortcutProfileLibraryStartup::Incompatible(raw) => {
                    log::error!("Canonical shortcut library is incompatible: {}", raw.reason());
                    self.shortcut_library_persistence =
                        ShortcutLibraryPersistenceRuntime::Incompatible(raw);
                }
                crate::common::shortcut_library_persistence::BrowserShortcutProfileLibraryStartup::Corrupt(raw) => {
                    log::error!("Canonical shortcut library is corrupt: {}", raw.reason());
                    self.shortcut_library_persistence =
                        ShortcutLibraryPersistenceRuntime::Corrupt(raw);
                }
            },
        }
    }

    fn report_shortcut_library_persistence_state(&self) {
        if let Some(persisted) = self.shortcut_library_persistence.persisted() {
            debug_assert_eq!(
                persisted.library(),
                self.ui.preferences.shortcut_profiles(),
                "live shortcuts must be the exact accepted canonical snapshot"
            );
            return;
        }
        if let Some(reason) = self.shortcut_library_persistence.blocked_reason() {
            log::warn!("Shortcut library mutations are unavailable: {reason}");
        }
    }

    #[cfg(test)]
    pub(crate) fn enable_volatile_test_shortcut_persistence(&mut self) {
        let library = self.ui.preferences.shortcut_profiles().clone();
        self.shortcut_library_persistence = ShortcutLibraryPersistenceRuntime::Ready {
            persisted: Box::new(PersistedShortcutProfileLibrary::test_snapshot(library, 1)),
            publication_completion: None,
            volatile_test: true,
        };
    }
}

enum NativeOrBrowserStartup {
    #[cfg(not(target_arch = "wasm32"))]
    Native(crate::common::shortcut_library_persistence::ShortcutProfileLibraryStartup),
    #[cfg(target_arch = "wasm32")]
    Browser(crate::common::shortcut_library_persistence::BrowserShortcutProfileLibraryStartup),
}

fn digest_prefix(digest: [u8; 32]) -> String {
    digest[..6]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unacknowledged_completion_blocks_reentrant_publication() {
        let mut state = AppState::default();
        state.enable_volatile_test_shortcut_persistence();
        let predecessor = state
            .shortcut_library_persistence
            .persisted()
            .expect("volatile predecessor")
            .clone();
        state.shortcut_library_persistence = ShortcutLibraryPersistenceRuntime::Ready {
            persisted: Box::new(predecessor),
            publication_completion: Some(Ok(())),
            volatile_test: true,
        };
        let before = state.ui.preferences.shortcut_profiles().clone();
        let mut candidate = before.clone();
        candidate
            .insert_named_preset(
                "Reentrant candidate",
                crate::workbench::ShortcutPreferences::default(),
                false,
            )
            .unwrap();

        let error = state
            .publish_shortcut_library_candidate(&candidate, &egui::Context::default())
            .unwrap_err();

        assert!(error.contains("awaiting acknowledgement"));
        assert_eq!(state.ui.preferences.shortcut_profiles(), &before);
        assert!(state.shortcut_library_publication_completion_pending());
    }
}
