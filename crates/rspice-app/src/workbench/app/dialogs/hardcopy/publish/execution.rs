//! Target-specific hardcopy publication execution and completion polling.
//!
//! The module owns worker lifetime and cancellation boundaries so the dialog
//! controller handles only validated publication state transitions.

use super::*;

#[cfg(target_arch = "wasm32")]
pub(super) fn start_browser_publication(
    app: &RSpiceApp,
    plan: std::sync::Arc<HardcopyPlan>,
    source: std::sync::Arc<ResolvedHardcopyDocument>,
    metadata: HardcopySceneMetadata,
    staged_mapping: StagedPrintMappingPersistence,
    destination: BrowserPublicationDestination,
) -> Result<(), String> {
    if PUBLICATION.with(|active| active.borrow().is_some()) || super::super::worker::is_active() {
        return Err("A browser hardcopy operation is already active.".to_owned());
    }
    let package_multi_part = matches!(
        &destination,
        BrowserPublicationDestination::Export {
            multi_part: true,
            ..
        }
    );
    let ticket = super::super::worker::start_publication(
        &plan,
        &source,
        metadata,
        package_multi_part,
        app.state.dialogs.hardcopy.source_resolution_generation,
        app.state.dialogs.hardcopy.preview_generation,
        repaint_context(),
    )?;
    PUBLICATION.with(|active| {
        *active.borrow_mut() = Some(ActiveBrowserPublication {
            ticket,
            plan,
            source,
            staged_mapping,
            destination,
        });
    });
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(in crate::workbench::app::dialogs::hardcopy) fn poll_publication(app: &mut RSpiceApp) {
    let completed = PUBLICATION.with(|active| {
        let mut active = active.borrow_mut();
        let result = active
            .as_ref()
            .and_then(|publication| super::super::worker::poll(publication.ticket));
        result.map(|result| {
            (
                active
                    .take()
                    .expect("completed browser publication remains owned"),
                result,
            )
        })
    });
    let Some((publication, result)) = completed else {
        return;
    };
    let current = &app.state.dialogs.hardcopy;
    if !current.open
        || current.source_resolution_generation != publication.ticket.epoch
        || current.preview_generation != publication.ticket.generation
        || current
            .preview_plan
            .as_ref()
            .map(|plan| plan.content_digest())
            != Some(publication.ticket_plan_digest())
        || current
            .resolved_document
            .as_ref()
            .map(|source| source.authority().content_digest())
            != Some(publication.ticket_source_digest())
    {
        app.state.dialogs.hardcopy.busy = false;
        if app.state.dialogs.hardcopy.open {
            app.state.dialogs.hardcopy.error = Some(
                "The hardcopy source or plan changed before browser publication completed."
                    .to_owned(),
            );
        }
        return;
    }
    let plan = publication.plan;
    let source = publication.source;
    let buffers = match result {
        Ok(buffers) => buffers,
        Err(error) => {
            app.state.dialogs.hardcopy.busy = false;
            app.state.dialogs.hardcopy.error = Some(record_render_failure(app, &plan, error));
            return;
        }
    };

    let completion = match publication.destination {
        BrowserPublicationDestination::Print { reservation } => (|| {
            let rendered = decode_browser_publication(&plan, &source, buffers)
                .map_err(|error| record_render_failure(app, &plan, error))?;
            let outcome = finalize_browser_print(reservation, &plan, &rendered)
                .map_err(|error| record_print_failure(app, &plan, error, 0));
            outcome.and_then(|outcome| {
                let receipt =
                    HardcopyReceipt::record(&plan, outcome).map_err(|error| error.to_string())?;
                retain_hardcopy_receipt(app, receipt)?;
                commit_print_mapping_persistence(app, publication.staged_mapping)?;
                let pages = rendered.page_count();
                Ok(format!(
                    "Browser print handoff accepted for {pages} page{}; confirm the browser print dialog.",
                    if pages == 1 { "" } else { "s" }
                ))
            })
        })(),
        BrowserPublicationDestination::Export {
            path,
            destination,
            media_type,
            multi_part,
        } => {
            if multi_part {
                super::super::worker::decode_packaged_publication(&plan, &source, buffers)
                    .map_err(|error| record_render_failure(app, &plan, error))
                    .and_then(|packaged| {
                        app.export_workflow_io
                            .write_bytes_file_observed(&destination, &packaged.bytes, media_type)
                            .map_err(|error| {
                                record_export_failure(
                                    app,
                                    &plan,
                                    format!("Could not publish hardcopy: {error}"),
                                )
                            })?;
                        let receipt = HardcopyReceipt::record(
                            &plan,
                            HardcopyOutcome::ArtifactExported {
                                artifact: packaged.artifact,
                            },
                        )
                        .map_err(|error| error.to_string())?;
                        retain_hardcopy_receipt(app, receipt)?;
                        commit_print_mapping_persistence(app, publication.staged_mapping)?;
                        Ok(export_completion_message(
                            "hardcopy",
                            &path,
                            Some(format!(
                                "{} page{} \u{00b7} deterministic ZIP package",
                                packaged.page_count,
                                if packaged.page_count == 1 { "" } else { "s" },
                            )),
                            app.export_workflow_io.as_ref(),
                        ))
                    })
            } else {
                decode_browser_publication(&plan, &source, buffers)
                    .map_err(|error| record_render_failure(app, &plan, error))
                    .and_then(|rendered| {
                        export_bytes_and_identity(&rendered, &plan, false).and_then(
                            |(bytes, artifact)| {
                                app.export_workflow_io
                                    .write_bytes_file_observed(&destination, &bytes, media_type)
                                    .map_err(|error| {
                                        record_export_failure(
                                            app,
                                            &plan,
                                            format!("Could not publish hardcopy: {error}"),
                                        )
                                    })?;
                                let receipt = HardcopyReceipt::record(
                                    &plan,
                                    HardcopyOutcome::ArtifactExported { artifact },
                                )
                                .map_err(|error| error.to_string())?;
                                retain_hardcopy_receipt(app, receipt)?;
                                commit_print_mapping_persistence(app, publication.staged_mapping)?;
                                Ok(export_completion_message(
                                    "hardcopy",
                                    &path,
                                    Some(format!(
                                        "{} page{} \u{00b7} {}",
                                        rendered.page_count(),
                                        if rendered.page_count() == 1 { "" } else { "s" },
                                        if rendered.format().is_vector() {
                                            "vector"
                                        } else {
                                            "raster"
                                        }
                                    )),
                                    app.export_workflow_io.as_ref(),
                                ))
                            },
                        )
                    })
            }
        }
    };
    app.state.dialogs.hardcopy.busy = false;
    match completion {
        Ok(message) => {
            let message = app.state.dialogs.hardcopy.last_receipt.as_ref().map_or(
                message.clone(),
                |receipt| {
                    format!(
                        "{message} Receipt {} recorded in project history.",
                        receipt.id()
                    )
                },
            );
            app.state.push_user_message(ConsoleMessage::info(message));
            app.state.dialogs.hardcopy.close();
        }
        Err(error) => {
            app.state.dialogs.hardcopy.error = Some(error);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn decode_browser_publication(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    mut buffers: Vec<Vec<u8>>,
) -> Result<RenderedHardcopyPublication, String> {
    if buffers.len() < 2 {
        return Err("Browser publication returned no artifact payload.".to_owned());
    }
    let manifest = buffers.remove(0);
    RenderedHardcopyPublication::from_worker_transfer(plan, source, &manifest, buffers)
        .map_err(|error| error.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn start_native_publication(
    app: &RSpiceApp,
    plan: std::sync::Arc<HardcopyPlan>,
    source: std::sync::Arc<ResolvedHardcopyDocument>,
    metadata: HardcopySceneMetadata,
    staged_mapping: StagedPrintMappingPersistence,
    operation: super::super::execution::ExecutionOperation,
    destination: NativePublicationDestination,
) -> Result<(), String> {
    if PUBLICATION.with(|active| active.borrow().is_some())
        || FINALIZATION.with(|active| active.borrow().is_some())
        || super::super::execution::is_active()
        || super::super::finalize::is_active()
    {
        return Err("A native hardcopy operation is already active.".to_owned());
    }
    let ticket = super::super::execution::start(
        plan.clone(),
        source.clone(),
        metadata,
        app.state.dialogs.hardcopy.preview_generation,
        operation,
        repaint_context(),
    )?;
    PUBLICATION.with(|active| {
        *active.borrow_mut() = Some(ActiveNativePublication {
            ticket,
            plan,
            staged_mapping,
            destination,
        });
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::workbench::app::dialogs::hardcopy) fn poll_publication(app: &mut RSpiceApp) {
    if poll_native_finalization(app) {
        return;
    }
    let completed = PUBLICATION.with(|active| {
        let mut active = active.borrow_mut();
        let result = active
            .as_ref()
            .and_then(|publication| super::super::execution::poll(publication.ticket));
        result.map(|result| {
            (
                active
                    .take()
                    .expect("completed native publication remains owned"),
                result,
            )
        })
    });
    let Some((publication, result)) = completed else {
        return;
    };
    let current = &app.state.dialogs.hardcopy;
    if current.open
        && (current.preview_generation != publication.ticket.generation
            || current
                .preview_plan
                .as_ref()
                .map(|plan| plan.content_digest())
                != Some(publication.ticket.plan_digest)
            || current
                .resolved_document
                .as_ref()
                .map(|source| source.authority().content_digest())
                != Some(publication.ticket.source_digest))
    {
        app.state.dialogs.hardcopy.busy = false;
        if app.state.dialogs.hardcopy.open {
            app.state.dialogs.hardcopy.error = Some(
                "The hardcopy source or plan changed before native rendering completed.".to_owned(),
            );
        }
        return;
    }
    let plan = publication.plan;
    let operation = match result {
        Err(error) => Err(record_render_failure(app, &plan, error)),
        Ok(completion) if completion.ticket != publication.ticket => Err(record_render_failure(
            app,
            &plan,
            "Native hardcopy rendering returned a stale completion ticket.".to_owned(),
        )),
        Ok(completion) => match (publication.destination, completion.payload) {
            (
                NativePublicationDestination::Print {
                    printer_id,
                    capabilities_digest,
                    cancellation,
                },
                super::super::execution::ExecutionPayload::NativePrinter(pages),
            ) => Ok((
                super::super::finalize::FinalizationOperation::Print {
                    pages,
                    printer_id: printer_id.clone(),
                    capabilities_digest,
                    cancellation: cancellation.clone(),
                },
                NativePublicationDestination::Print {
                    printer_id,
                    capabilities_digest,
                    cancellation,
                },
            )),
            (
                NativePublicationDestination::DesktopPrintHandoff,
                super::super::execution::ExecutionPayload::Publication(rendered),
            ) => Ok((
                super::super::finalize::FinalizationOperation::DesktopPrintHandoff {
                    publication: rendered,
                },
                NativePublicationDestination::DesktopPrintHandoff,
            )),
            (
                NativePublicationDestination::Export {
                    path,
                    destination,
                    media_type,
                    multi_part,
                },
                super::super::execution::ExecutionPayload::Publication(rendered),
            ) => Ok((
                super::super::finalize::FinalizationOperation::Export {
                    publication: rendered,
                    destination: destination.clone(),
                    media_type,
                    multi_part,
                },
                NativePublicationDestination::Export {
                    path,
                    destination,
                    media_type,
                    multi_part,
                },
            )),
            _ => Err(record_render_failure(
                app,
                &plan,
                "Native hardcopy rendering returned the wrong payload type.".to_owned(),
            )),
        },
    };
    match operation {
        Ok((operation, destination)) => {
            let ticket =
                match super::super::finalize::start(
                    plan.clone(),
                    publication.ticket.generation,
                    operation,
                    repaint_context(),
                ) {
                    Ok(ticket) => ticket,
                    Err(error) => {
                        app.state.dialogs.hardcopy.busy = false;
                        app.state.dialogs.hardcopy.error = Some(
                            record_native_finalization_failure(app, &plan, &destination, error),
                        );
                        return;
                    }
                };
            FINALIZATION.with(|active| {
                *active.borrow_mut() = Some(ActiveNativeFinalization {
                    ticket,
                    plan,
                    source_digest: publication.ticket.source_digest,
                    staged_mapping: publication.staged_mapping,
                    destination,
                });
            });
        }
        Err(error) => {
            app.state.dialogs.hardcopy.busy = false;
            app.state.dialogs.hardcopy.error = Some(error);
        }
    }
}
