//! Browser PDK persistence transaction owner.
//!
//! IndexedDB is asynchronous, so browser configuration candidates remain
//! staged until exact durable readback completes. No administration surface
//! may mutate live PDK authority while startup hydration or a publication is
//! pending.

use std::collections::VecDeque;

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::state::model_library::ModelLibraryManager;
use crate::state::pdk_config::{
    BrowserPdkConfigReceipt, BrowserPdkConfigRestore, BrowserPdkStorageDurability,
    BrowserPdkStorageStatus, PdkConfig, start_browser_pdk_config_load,
    start_browser_pdk_config_save,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::design_history::publish_model_library_set_candidate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrowserPdkPhase {
    Dormant,
    Loading,
    Ready,
    Publishing,
    Failed,
}

struct BrowserPdkOwner {
    phase: BrowserPdkPhase,
    operation_generation: u64,
    receipt: Option<BrowserPdkConfigReceipt>,
    storage_status: BrowserPdkStorageStatus,
    last_error: Option<String>,
}

impl Default for BrowserPdkOwner {
    fn default() -> Self {
        Self {
            phase: BrowserPdkPhase::Dormant,
            operation_generation: 0,
            receipt: None,
            storage_status: BrowserPdkStorageStatus::default(),
            last_error: None,
        }
    }
}

/// Work the caller wants run once the commit has actually landed, in the
/// frame that observes it.
type AfterBrowserPdkCommit = Option<Box<dyn FnOnce(&Context)>>;

enum BrowserPdkPublicationIntent {
    Administration {
        title: String,
        message: String,
        after_commit: AfterBrowserPdkCommit,
    },
    Settings(Box<BrowserPdkSettingsIntent>),
}

struct BrowserPdkSettingsIntent {
    previous: PdkConfig,
    model_libraries: ModelLibraryManager,
    loaded: usize,
}

enum BrowserPdkCompletion {
    Restore {
        operation_generation: u64,
        result: Result<BrowserPdkConfigRestore, String>,
    },
    Publication {
        operation_generation: u64,
        candidate: PdkConfig,
        intent: Box<BrowserPdkPublicationIntent>,
        result: Result<BrowserPdkConfigReceipt, String>,
    },
    RejectedSettingsRollback {
        operation_generation: u64,
        publication_error: String,
        result: Result<BrowserPdkConfigReceipt, String>,
    },
}

thread_local! {
    static BROWSER_PDK_OWNER: std::cell::RefCell<BrowserPdkOwner> =
        std::cell::RefCell::new(BrowserPdkOwner::default());
    static BROWSER_PDK_COMPLETIONS: std::cell::RefCell<VecDeque<BrowserPdkCompletion>> =
        const { std::cell::RefCell::new(VecDeque::new()) };
}

pub(super) fn initialize_browser_pdk_persistence(ctx: &Context) {
    start_restore(ctx);
}

fn start_restore(ctx: &Context) {
    let operation_generation = BROWSER_PDK_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        owner.operation_generation = owner.operation_generation.wrapping_add(1).max(1);
        owner.phase = BrowserPdkPhase::Loading;
        owner.operation_generation
    });
    let repaint = ctx.clone();
    start_browser_pdk_config_load(PdkConfig::default_config_path(), move |result| {
        BROWSER_PDK_COMPLETIONS.with(|queue| {
            queue.borrow_mut().push_back(BrowserPdkCompletion::Restore {
                operation_generation,
                result: result.map_err(|error| error.to_string()),
            });
        });
        repaint.request_repaint();
    });
}

impl RSpiceApp {
    pub(crate) fn start_browser_pdk_administration_publication(
        &mut self,
        ctx: &Context,
        candidate: PdkConfig,
        title: impl Into<String>,
        message: impl Into<String>,
        after_commit: impl FnOnce(&Context) + 'static,
    ) -> Result<(), String> {
        start_publication(
            ctx,
            candidate,
            BrowserPdkPublicationIntent::Administration {
                title: title.into(),
                message: message.into(),
                after_commit: Some(Box::new(after_commit)),
            },
        )
    }

    pub(crate) fn start_browser_pdk_settings_publication(
        &mut self,
        ctx: &Context,
        candidate: PdkConfig,
        model_libraries: ModelLibraryManager,
        loaded: usize,
    ) -> Result<(), String> {
        start_publication(
            ctx,
            candidate,
            BrowserPdkPublicationIntent::Settings(Box::new(BrowserPdkSettingsIntent {
                previous: self.state.pdk_config.clone(),
                model_libraries,
                loaded,
            })),
        )
    }

    pub(crate) fn browser_pdk_persistence_ready(&self) -> bool {
        BROWSER_PDK_OWNER.with(|owner| owner.borrow().phase == BrowserPdkPhase::Ready)
    }

    pub(crate) fn browser_pdk_persistence_retryable(&self) -> bool {
        BROWSER_PDK_OWNER.with(|owner| owner.borrow().phase == BrowserPdkPhase::Failed)
    }

    pub(crate) fn browser_pdk_persistence_degraded(&self) -> bool {
        BROWSER_PDK_OWNER.with(|owner| {
            let owner = owner.borrow();
            owner.phase == BrowserPdkPhase::Ready
                && owner.storage_status.durability != BrowserPdkStorageDurability::Persistent
        })
    }

    pub(crate) fn browser_pdk_persistence_status_message(&self) -> String {
        BROWSER_PDK_OWNER.with(|owner| {
            let owner = owner.borrow();
            match owner.phase {
                BrowserPdkPhase::Dormant => {
                    "Browser PDK storage has not been initialized.".to_owned()
                }
                BrowserPdkPhase::Loading => owner.last_error.as_ref().map_or_else(
                    || "Loading and validating browser PDK storage…".to_owned(),
                    |error| {
                        format!(
                            "Reloading authoritative browser PDK state after a storage failure: {error}"
                        )
                    },
                ),
                BrowserPdkPhase::Publishing => {
                    "Publishing and reading back the browser PDK transaction…".to_owned()
                }
                BrowserPdkPhase::Failed => owner.last_error.as_ref().map_or_else(
                    || "Browser PDK storage is unavailable.".to_owned(),
                    |error| format!("Browser PDK storage is unavailable: {error}"),
                ),
                BrowserPdkPhase::Ready => storage_status_message(owner.storage_status),
            }
        })
    }

    pub(crate) fn retry_browser_pdk_persistence(&mut self, ctx: &Context) -> Result<(), String> {
        let retryable =
            BROWSER_PDK_OWNER.with(|owner| owner.borrow().phase == BrowserPdkPhase::Failed);
        if !retryable {
            return Err("browser PDK storage is not in a retryable failed state".to_owned());
        }
        start_restore(ctx);
        Ok(())
    }

    pub(super) fn poll_browser_pdk_persistence(&mut self, ctx: &Context) {
        let completions =
            BROWSER_PDK_COMPLETIONS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
        for completion in completions {
            match completion {
                BrowserPdkCompletion::Restore {
                    operation_generation,
                    result,
                } => self.finish_browser_pdk_restore(ctx, operation_generation, result),
                BrowserPdkCompletion::Publication {
                    operation_generation,
                    candidate,
                    intent,
                    result,
                } => self.finish_browser_pdk_publication(
                    ctx,
                    operation_generation,
                    candidate,
                    *intent,
                    result,
                ),
                BrowserPdkCompletion::RejectedSettingsRollback {
                    operation_generation,
                    publication_error,
                    result,
                } => self.finish_rejected_browser_pdk_settings_rollback(
                    ctx,
                    operation_generation,
                    &publication_error,
                    result,
                ),
            }
        }
    }

    fn finish_browser_pdk_restore(
        &mut self,
        ctx: &Context,
        operation_generation: u64,
        result: Result<BrowserPdkConfigRestore, String>,
    ) {
        let current = BROWSER_PDK_OWNER.with(|owner| {
            let owner = owner.borrow();
            owner.operation_generation == operation_generation
                && owner.phase == BrowserPdkPhase::Loading
        });
        if !current {
            return;
        }

        match result {
            Ok(mut restored) => {
                let storage_status = restored.storage_status;
                let trust_store = restored.config.publisher_trust_store.clone();
                let validation = restored
                    .config
                    .technology_registry
                    .revalidate_installed(&trust_store);
                let mut model_library = self.state.model_library_manager.clone();
                let model_load = model_library
                    .replace_from_pdk_config(Some(&self.state.pdk_config), &mut restored.config);
                self.state.pdk_config = restored.config;
                if model_load.is_ok() {
                    self.state.model_library_manager = model_library;
                }
                self.invalidate_simulation_preflight();
                BROWSER_PDK_OWNER.with(|owner| {
                    let mut owner = owner.borrow_mut();
                    owner.receipt = restored.receipt;
                    owner.storage_status = storage_status;
                    owner.last_error = None;
                    owner.phase = BrowserPdkPhase::Ready;
                });

                if let Some(warning) = storage_durability_warning(storage_status) {
                    self.state
                        .push_user_message(ConsoleMessage::warning(warning.to_owned()));
                    self.state.ui.toasts.warn_with_title(
                        ctx,
                        "Browser PDK storage is not durable",
                        warning,
                    );
                }
                if restored.migrated_legacy_record {
                    self.state.push_user_message(ConsoleMessage::info(
                        "PDK configuration migrated from legacy browser Web Storage to the content-addressed IndexedDB store."
                            .to_owned(),
                    ));
                }
                if let Err(errors) = validation {
                    for error in errors {
                        self.state.push_user_message(ConsoleMessage::error(format!(
                            "PDK technology trust blocked during browser restore: {error}"
                        )));
                    }
                }
                if let Err(errors) = model_load {
                    for error in errors {
                        self.state.push_user_message(ConsoleMessage::error(format!(
                            "PDK model library restore failed: {error}"
                        )));
                    }
                }
            }
            Err(error) => {
                BROWSER_PDK_OWNER.with(|owner| {
                    let mut owner = owner.borrow_mut();
                    owner.phase = BrowserPdkPhase::Failed;
                    owner.last_error = Some(error.clone());
                });
                let message = format!(
                    "PDK browser storage could not be restored. No persisted PDK authority was loaded: {error}"
                );
                self.state
                    .push_user_message(ConsoleMessage::error(message.clone()));
                self.state
                    .ui
                    .toasts
                    .error_with_title(ctx, "PDK storage unavailable", message);
            }
        }
    }

    fn finish_browser_pdk_publication(
        &mut self,
        ctx: &Context,
        operation_generation: u64,
        candidate: PdkConfig,
        intent: BrowserPdkPublicationIntent,
        result: Result<BrowserPdkConfigReceipt, String>,
    ) {
        let current = BROWSER_PDK_OWNER.with(|owner| {
            let owner = owner.borrow();
            owner.operation_generation == operation_generation
                && owner.phase == BrowserPdkPhase::Publishing
        });
        if !current {
            return;
        }

        match result {
            Ok(receipt) => {
                let storage_status = receipt.storage_status;
                match intent {
                    BrowserPdkPublicationIntent::Administration {
                        title,
                        message,
                        after_commit,
                    } => {
                        accept_browser_pdk_receipt(receipt);
                        self.state.pdk_config = candidate;
                        emit_browser_pdk_storage_warning(self, ctx, storage_status);
                        if let Some(after_commit) = after_commit {
                            after_commit(ctx);
                        }
                        self.state
                            .push_user_message(ConsoleMessage::info(message.clone()));
                        self.state.ui.toasts.success(ctx, title, message);
                    }
                    BrowserPdkPublicationIntent::Settings(settings) => {
                        let BrowserPdkSettingsIntent {
                            previous,
                            model_libraries,
                            loaded,
                        } = *settings;
                        let publication = if self.state.project_lifecycle.project_open {
                            publish_model_library_set_candidate(
                                &mut self.state,
                                model_libraries,
                                "apply configured PDK model sources",
                            )
                            .map(|_| ())
                        } else {
                            self.state.model_library_manager = model_libraries;
                            Ok(())
                        };
                        if let Err(error) = publication {
                            if let Err(rollback_error) = start_rejected_settings_rollback(
                                ctx,
                                operation_generation,
                                receipt,
                                previous,
                                error.clone(),
                            ) {
                                fail_browser_pdk_owner(format!(
                                    "{error}; durable PDK rollback could not start: {rollback_error}"
                                ));
                                let message = format!(
                                    "PDK settings could not be published after durable storage committed, and rollback could not start: {error}; {rollback_error}"
                                );
                                self.state
                                    .push_user_message(ConsoleMessage::error(message.clone()));
                                self.state.ui.toasts.error_with_title(
                                    ctx,
                                    "PDK transaction requires recovery",
                                    message,
                                );
                            }
                            return;
                        }
                        accept_browser_pdk_receipt(receipt);
                        self.state.pdk_config = candidate;
                        self.invalidate_simulation_preflight();
                        emit_browser_pdk_storage_warning(self, ctx, storage_status);
                        let message = format!(
                            "PDK settings applied: {loaded} librar{} loaded and durably read back",
                            if loaded == 1 { "y" } else { "ies" }
                        );
                        self.state
                            .push_user_message(ConsoleMessage::info(message.clone()));
                        self.state
                            .ui
                            .toasts
                            .success(ctx, "PDK settings applied", message);
                    }
                }
            }
            Err(error) => {
                BROWSER_PDK_OWNER.with(|owner| {
                    let mut owner = owner.borrow_mut();
                    owner.phase = BrowserPdkPhase::Failed;
                    owner.last_error = Some(error.clone());
                });
                let _ = intent;
                let message = format!(
                    "PDK technology operation was not applied because durable browser publication failed: {error}"
                );
                self.state
                    .push_user_message(ConsoleMessage::error(message.clone()));
                self.state
                    .ui
                    .toasts
                    .error_with_title(ctx, "PDK publication failed", message);
                // Re-read the authoritative head after a conflict or storage
                // failure. A candidate that did not commit is never retried
                // implicitly and never becomes live state.
                start_restore(ctx);
            }
        }
    }

    fn finish_rejected_browser_pdk_settings_rollback(
        &mut self,
        ctx: &Context,
        operation_generation: u64,
        publication_error: &str,
        result: Result<BrowserPdkConfigReceipt, String>,
    ) {
        let current = BROWSER_PDK_OWNER.with(|owner| {
            let owner = owner.borrow();
            owner.operation_generation == operation_generation
                && owner.phase == BrowserPdkPhase::Publishing
        });
        if !current {
            return;
        }
        match result {
            Ok(receipt) => {
                accept_browser_pdk_receipt(receipt);
                let message = format!(
                    "PDK settings were not applied because project publication failed; the durable browser configuration was rolled back exactly: {publication_error}"
                );
                self.state
                    .push_user_message(ConsoleMessage::error(message.clone()));
                self.state
                    .ui
                    .toasts
                    .error_with_title(ctx, "PDK settings not applied", message);
            }
            Err(rollback_error) => {
                let failure = format!(
                    "project publication failed: {publication_error}; durable browser rollback failed: {rollback_error}"
                );
                fail_browser_pdk_owner(failure.clone());
                let message = format!(
                    "PDK settings could not be published and the durable browser rollback failed. Live state remains unchanged; recover browser PDK storage before continuing: {failure}"
                );
                self.state
                    .push_user_message(ConsoleMessage::error(message.clone()));
                self.state.ui.toasts.error_with_title(
                    ctx,
                    "PDK transaction requires recovery",
                    message,
                );
            }
        }
    }
}

fn accept_browser_pdk_receipt(receipt: BrowserPdkConfigReceipt) {
    BROWSER_PDK_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        owner.storage_status = receipt.storage_status;
        owner.receipt = Some(receipt);
        owner.last_error = None;
        owner.phase = BrowserPdkPhase::Ready;
    });
}

fn fail_browser_pdk_owner(error: String) {
    BROWSER_PDK_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        owner.phase = BrowserPdkPhase::Failed;
        owner.last_error = Some(error);
    });
}

fn emit_browser_pdk_storage_warning(
    app: &mut RSpiceApp,
    ctx: &Context,
    storage_status: BrowserPdkStorageStatus,
) {
    if let Some(warning) = storage_durability_warning(storage_status) {
        app.state
            .push_user_message(ConsoleMessage::warning(warning.to_owned()));
        app.state
            .ui
            .toasts
            .warn_with_title(ctx, "Browser PDK storage is not durable", warning);
    }
}

fn start_rejected_settings_rollback(
    ctx: &Context,
    operation_generation: u64,
    committed: BrowserPdkConfigReceipt,
    previous: PdkConfig,
    publication_error: String,
) -> Result<(), String> {
    let repaint = ctx.clone();
    start_browser_pdk_config_save(
        PdkConfig::default_config_path(),
        Some(committed),
        previous,
        move |result| {
            BROWSER_PDK_COMPLETIONS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserPdkCompletion::RejectedSettingsRollback {
                        operation_generation,
                        publication_error,
                        result: result.map_err(|error| error.to_string()),
                    });
            });
            repaint.request_repaint();
        },
    )
    .map_err(|error| error.to_string())
}

fn start_publication(
    ctx: &Context,
    candidate: PdkConfig,
    intent: BrowserPdkPublicationIntent,
) -> Result<(), String> {
    let (operation_generation, expected) = BROWSER_PDK_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        if owner.phase != BrowserPdkPhase::Ready {
            let reason = match owner.phase {
                BrowserPdkPhase::Dormant | BrowserPdkPhase::Loading => {
                    "PDK browser storage is still loading"
                }
                BrowserPdkPhase::Publishing => "another PDK publication is still pending",
                BrowserPdkPhase::Failed => {
                    "PDK browser storage is unavailable; retry storage recovery before publishing"
                }
                BrowserPdkPhase::Ready => unreachable!(),
            };
            return Err(reason.to_owned());
        }
        owner.operation_generation = owner.operation_generation.wrapping_add(1).max(1);
        owner.phase = BrowserPdkPhase::Publishing;
        Ok((owner.operation_generation, owner.receipt.clone()))
    })?;

    let repaint = ctx.clone();
    let completion_candidate = candidate.clone();
    let result = start_browser_pdk_config_save(
        PdkConfig::default_config_path(),
        expected,
        candidate,
        move |result| {
            BROWSER_PDK_COMPLETIONS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserPdkCompletion::Publication {
                        operation_generation,
                        candidate: completion_candidate,
                        intent: Box::new(intent),
                        result: result.map_err(|error| error.to_string()),
                    });
            });
            repaint.request_repaint();
        },
    );
    if let Err(error) = result {
        BROWSER_PDK_OWNER.with(|owner| {
            let mut owner = owner.borrow_mut();
            if owner.operation_generation == operation_generation {
                owner.phase = BrowserPdkPhase::Ready;
            }
        });
        return Err(error.to_string());
    }
    Ok(())
}

fn storage_status_message(status: BrowserPdkStorageStatus) -> String {
    let durability = match status.durability {
        BrowserPdkStorageDurability::Persistent => "durable storage granted",
        BrowserPdkStorageDurability::BestEffort => {
            "best-effort storage only; the browser may evict PDK data under storage pressure"
        }
        BrowserPdkStorageDurability::Unknown => {
            "durability status unavailable; the browser did not expose a persistence decision"
        }
    };
    status.available_bytes().map_or_else(
        || format!("Browser PDK storage is ready; {durability}."),
        |available| {
            format!(
                "Browser PDK storage is ready; {durability}; approximately {} available.",
                crate::simulation::run_set::format_bytes(available)
            )
        },
    )
}

fn storage_durability_warning(status: BrowserPdkStorageStatus) -> Option<&'static str> {
    match status.durability {
        BrowserPdkStorageDurability::Persistent => None,
        BrowserPdkStorageDurability::BestEffort => Some(
            "The browser denied durable origin storage. Installed PDK packages remain usable now, but the browser may evict them under storage pressure; retain the signed source packages for recovery.",
        ),
        BrowserPdkStorageDurability::Unknown => Some(
            "The browser did not expose a durable-storage decision. Installed PDK packages remain usable now, but eviction guarantees are unknown; retain the signed source packages for recovery.",
        ),
    }
}
