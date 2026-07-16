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
    Ready(Box<PersistedShortcutProfileLibrary>),
    #[cfg(target_arch = "wasm32")]
    Initializing(crate::common::shortcut_library_persistence::BrowserShortcutLibraryWriteToken),
    Missing,
    Incompatible(RetainedShortcutLibraryBytes),
    Corrupt(RetainedShortcutLibraryBytes),
    Unavailable(String),
}

impl ShortcutLibraryPersistenceRuntime {
    pub(crate) fn persisted(&self) -> Option<&PersistedShortcutProfileLibrary> {
        match self {
            Self::Ready(persisted) => Some(persisted.as_ref()),
            _ => None,
        }
    }

    pub(crate) fn blocked_reason(&self) -> Option<String> {
        match self {
            Self::Uninitialized => Some("shortcut library storage is not initialized".to_owned()),
            Self::Ready(_) => None,
            #[cfg(target_arch = "wasm32")]
            Self::Initializing(_) => {
                Some("shortcut library storage is still initializing".to_owned())
            }
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
        }
    }
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

    fn install_persisted_shortcut_library(&mut self, persisted: PersistedShortcutProfileLibrary) {
        *self.ui.preferences.shortcut_profiles_mut() = persisted.library().clone();
        self.shortcut_resolver.reset();
        self.shortcut_library_persistence =
            ShortcutLibraryPersistenceRuntime::Ready(Box::new(persisted));
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_shortcut_library_persistence(&mut self) {
        let token = match &self.shortcut_library_persistence {
            ShortcutLibraryPersistenceRuntime::Initializing(token) => *token,
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
            Ok(persisted) => self.install_persisted_shortcut_library(persisted),
            Err(error) => {
                log::error!("Could not initialize canonical shortcut library: {error}");
                self.shortcut_library_persistence =
                    ShortcutLibraryPersistenceRuntime::Unavailable(error.to_string());
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
                } => self.install_persisted_shortcut_library(*persisted),
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
                    self.install_persisted_shortcut_library(*persisted);
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
