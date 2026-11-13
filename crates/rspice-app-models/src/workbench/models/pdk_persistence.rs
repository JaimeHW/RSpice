//! Browser PDK persistence transaction owner.
//!
//! IndexedDB is asynchronous, so browser configuration candidates remain
//! staged until exact durable readback completes. Nothing may mutate live
//! PDK authority while startup hydration or a publication is pending.

use std::collections::VecDeque;

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::model_library::ModelLibraryManager;
use crate::state::pdk_config::{
    BrowserPdkConfigReceipt, BrowserPdkConfigRestore, BrowserPdkStorageDurability,
    BrowserPdkStorageStatus, PdkConfig, start_browser_pdk_config_load,
    start_browser_pdk_config_save,
};
use crate::workbench::app_state::design_history::publish_model_library_set_candidate;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::simulation::SimulationController;
use rspice_app_runs::workbench::app_state::run_preflight::PreflightDialogState;

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
}

impl Default for BrowserPdkOwner {
    fn default() -> Self {
        Self {
            phase: BrowserPdkPhase::Dormant,
            operation_generation: 0,
            receipt: None,
        }
    }
}

enum BrowserPdkPublicationIntent {
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

/// What a browser PDK storage completion writes as the frame takes it: the
/// live PDK configuration, the model libraries and the log in the model, the
/// toasts, and the run preflight a configuration change drops. Whether a
/// project is open decides where settled model libraries go.
pub(in crate::workbench) struct BrowserPdkParts<'a> {
    pub(in crate::workbench) model: &'a mut Model,
    pub(in crate::workbench) workbench: &'a mut WorkbenchSession,
    pub(in crate::workbench) platform: &'a PlatformState,
    pub(in crate::workbench) preflight: &'a mut PreflightDialogState,
    pub(in crate::workbench) run_controller: &'a mut SimulationController,
}

impl BrowserPdkParts<'_> {
    /// Drop every authorization and presentation artifact derived from the
    /// configuration a completion replaced: the run preflight and the run it
    /// prepared.
    fn invalidate_simulation_preflight(&mut self) {
        self.preflight.invalidate();
        self.run_controller.clear_prepared_run();
    }
}

pub fn initialize_browser_pdk_persistence(ctx: &Context) {
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
    start_browser_pdk_config_load(
        crate::state::pdk_config::default_config_path(),
        move |result| {
            BROWSER_PDK_COMPLETIONS.with(|queue| {
                queue.borrow_mut().push_back(BrowserPdkCompletion::Restore {
                    operation_generation,
                    result: result.map_err(|error| error.to_string()),
                });
            });
            repaint.request_repaint();
        },
    );
}

pub(crate) fn start_browser_pdk_settings_publication(
    ctx: &Context,
    previous: &PdkConfig,
    candidate: PdkConfig,
    model_libraries: ModelLibraryManager,
    loaded: usize,
) -> Result<(), String> {
    start_publication(
        ctx,
        candidate,
        BrowserPdkPublicationIntent::Settings(Box::new(BrowserPdkSettingsIntent {
            previous: previous.clone(),
            model_libraries,
            loaded,
        })),
    )
}

pub(in crate::workbench) fn poll_browser_pdk_persistence(
    parts: &mut BrowserPdkParts<'_>,
    ctx: &Context,
) {
    let completions =
        BROWSER_PDK_COMPLETIONS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        match completion {
            BrowserPdkCompletion::Restore {
                operation_generation,
                result,
            } => finish_browser_pdk_restore(parts, ctx, operation_generation, result),
            BrowserPdkCompletion::Publication {
                operation_generation,
                candidate,
                intent,
                result,
            } => finish_browser_pdk_publication(
                parts,
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
            } => finish_rejected_browser_pdk_settings_rollback(
                parts,
                operation_generation,
                &publication_error,
                result,
            ),
        }
    }
}

fn finish_browser_pdk_restore(
    parts: &mut BrowserPdkParts<'_>,
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
            let restored_saved_configuration = restored.receipt.is_some();
            let validation = restored.config.technology_registry.revalidate_installed();
            let mut model_library = parts.model.model_library_manager.clone();
            let model_load = model_library
                .replace_from_pdk_config(Some(&parts.model.pdk_config), &mut restored.config);
            parts.model.pdk_config = restored.config;
            if model_load.is_ok() {
                parts.model.model_library_manager = model_library;
            }
            parts.invalidate_simulation_preflight();
            BROWSER_PDK_OWNER.with(|owner| {
                let mut owner = owner.borrow_mut();
                owner.receipt = restored.receipt;
                owner.phase = BrowserPdkPhase::Ready;
            });

            if restored_saved_configuration {
                emit_browser_pdk_storage_warning(parts, ctx, storage_status);
            }
            if let Err(errors) = validation {
                for error in errors {
                    parts.model.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!(
                            "PDK technology restored from browser storage did not validate: \
                             {error}"
                        ),
                    );
                }
            }
            if let Err(errors) = model_load {
                for error in errors {
                    parts.model.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!("PDK model library restore failed: {error}"),
                    );
                }
            }
        }
        Err(error) => {
            BROWSER_PDK_OWNER.with(|owner| {
                let mut owner = owner.borrow_mut();
                owner.phase = BrowserPdkPhase::Failed;
            });
            parts.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!(
                    "The saved PDK configuration could not be restored from browser storage, \
                     so no PDK was loaded: {error}"
                ),
            );
        }
    }
}

fn finish_browser_pdk_publication(
    parts: &mut BrowserPdkParts<'_>,
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
                BrowserPdkPublicationIntent::Settings(settings) => {
                    let BrowserPdkSettingsIntent {
                        previous,
                        model_libraries,
                        loaded,
                    } = *settings;
                    let publication = if parts.platform.project_lifecycle.is_open() {
                        publish_model_library_set_candidate(
                            parts.model,
                            model_libraries,
                            "apply configured PDK model sources",
                        )
                        .map(|_| ())
                    } else {
                        parts.model.model_library_manager = model_libraries;
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
                            fail_browser_pdk_owner();
                            parts.model.record(
                                LogSeverity::Error,
                                LogSource::Studio,
                                format!(
                                    "PDK settings were saved to browser storage but the \
                                     project could not take them, and the saved settings \
                                     could not be rolled back: {error}; {rollback_error}"
                                ),
                            );
                        }
                        return;
                    }
                    accept_browser_pdk_receipt(receipt);
                    parts.model.pdk_config = candidate;
                    parts.invalidate_simulation_preflight();
                    emit_browser_pdk_storage_warning(parts, ctx, storage_status);
                    parts.model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        format!(
                            "PDK settings applied: {} loaded.",
                            crate::ui::accessibility::counted(loaded, "library", "libraries")
                        ),
                    );
                }
            }
        }
        Err(error) => {
            BROWSER_PDK_OWNER.with(|owner| {
                let mut owner = owner.borrow_mut();
                owner.phase = BrowserPdkPhase::Failed;
            });
            let _ = intent;
            parts.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!(
                    "PDK change was not applied because saving it to browser storage failed: \
                     {error}"
                ),
            );
            // Re-read the authoritative head after a conflict or storage
            // failure. A candidate that did not commit is never retried
            // implicitly and never becomes live state.
            start_restore(ctx);
        }
    }
}

fn finish_rejected_browser_pdk_settings_rollback(
    parts: &mut BrowserPdkParts<'_>,
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
            parts.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!(
                    "PDK settings were not applied because the project could not take them; \
                     browser storage was rolled back: {publication_error}"
                ),
            );
        }
        Err(rollback_error) => {
            let failure = format!(
                "the project could not take them: {publication_error}; browser storage \
                 rollback failed: {rollback_error}"
            );
            fail_browser_pdk_owner();
            parts.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!(
                    "PDK settings were not applied, and browser storage could not be rolled \
                     back. The open session is unchanged; recover browser PDK storage before \
                     continuing: {failure}"
                ),
            );
        }
    }
}

fn accept_browser_pdk_receipt(receipt: BrowserPdkConfigReceipt) {
    BROWSER_PDK_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        owner.receipt = Some(receipt);
        owner.phase = BrowserPdkPhase::Ready;
    });
}

fn fail_browser_pdk_owner() {
    BROWSER_PDK_OWNER.with(|owner| owner.borrow_mut().phase = BrowserPdkPhase::Failed);
}

fn emit_browser_pdk_storage_warning(
    parts: &mut BrowserPdkParts<'_>,
    ctx: &Context,
    storage_status: BrowserPdkStorageStatus,
) {
    if let Some(warning) = storage_durability_warning(storage_status) {
        parts
            .model
            .record(LogSeverity::Warning, LogSource::Studio, warning);
        parts
            .workbench
            .toasts
            .warn_with_title(ctx, "Back up browser PDK data", warning);
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
        crate::state::pdk_config::default_config_path(),
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
        crate::state::pdk_config::default_config_path(),
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

fn storage_durability_warning(status: BrowserPdkStorageStatus) -> Option<&'static str> {
    match status.durability {
        BrowserPdkStorageDurability::Persistent => None,
        BrowserPdkStorageDurability::BestEffort => Some(
            "Persistent browser storage has not been granted. The browser may remove saved PDK data under storage pressure; keep the source packages and a configuration backup for recovery.",
        ),
        BrowserPdkStorageDurability::Unknown => Some(
            "Browser storage persistence could not be determined. Keep the source packages and a configuration backup so saved PDK data can be recovered if the browser clears it.",
        ),
    }
}
