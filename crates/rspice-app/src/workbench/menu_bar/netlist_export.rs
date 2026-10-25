//! Netlist menu actions.
//!
//! Generates the design's deck for File ▸ Export ▸ Netlist, View
//! Netlist, and the Netlist workspace's generation and export, and
//! records what the generator raised beside it.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub(crate) fn action_export_netlist_with_io(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let netlist_content =
        if state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
            if state.model.netlist_artifacts.generated_source.is_empty() {
                state.prompt("There is no generated netlist to export.");
                return;
            }
            if state.model.netlist_artifacts.generation_error.is_some()
                || state.model.netlist_artifacts.generated_input_digest
                    != state
                        .model
                        .netlist_artifacts
                        .current_generation_input_digest
            {
                state.prompt("The generated netlist is out of date; regenerate it to export.");
                return;
            }
            match format {
                crate::io::NetlistFormat::Spectre => {
                    super::netlist_compat::spice_to_ahdl_compatible_netlist(
                        &state.model.netlist_artifacts.generated_source,
                    )
                }
                _ => state.model.netlist_artifacts.generated_source.clone(),
            }
        } else {
            if state.model.schematic.document().components.is_empty() {
                state.prompt("No circuit to export. Add components first.");
                return;
            }
            match build_menu_netlist(state, format) {
                Ok(generated) => {
                    record_netlist_warnings(state, generated.warnings);
                    generated.source
                }
                Err(reasons) => {
                    state.report_failure("Netlist export failed", reasons.join("; "));
                    return;
                }
            }
        };

    let default_name = format!(
        "{}.{}",
        state.model.export_file_stem("circuit"),
        format.extension()
    );

    let filter_name = match format {
        crate::io::NetlistFormat::Spectre => "Spectre Netlist",
        crate::io::NetlistFormat::Spice => "SPICE Netlist",
        crate::io::NetlistFormat::Hspice => "HSPICE Netlist",
        crate::io::NetlistFormat::Xyce => "Xyce Netlist",
    };

    let extension = format.extension();

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Netlist",
        default_name: &default_name,
        filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, extension);

            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_text_file_observed(&destination, &netlist_content)
            });
            match export {
                Ok(()) => {
                    state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            filter_name,
                            &path,
                            None,
                            io,
                        ),
                    );
                }
                Err(e) => state.report_failure("Netlist export failed", e.to_string()),
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => state.report_failure("Netlist export failed", e.to_string()),
    }
}

pub(crate) fn action_export_generated_netlist_with_options(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
    bundle_dependencies: bool,
    include_source_map: bool,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> bool {
    let Some(document) = state.model.netlist_artifacts.generated_document.as_ref() else {
        state.prompt("There is no generated netlist to export.");
        return false;
    };
    let Some(artifact) = document.generated_artifact().cloned() else {
        state.prompt("The selected document is not a generated netlist.");
        return false;
    };
    if state.model.netlist_artifacts.generation_error.is_some()
        || state.model.netlist_artifacts.generated_input_digest
            != state
                .model
                .netlist_artifacts
                .current_generation_input_digest
        || state.model.netlist_artifacts.generated_input_digest
            != Some(artifact.provenance().input().digest())
    {
        state.prompt("The generated netlist is out of date; regenerate it to export.");
        return false;
    }
    if include_source_map && !bundle_dependencies {
        state.prompt("A source map is exported only with a self-contained bundle.");
        return false;
    }
    if include_source_map && format != crate::io::NetlistFormat::Spice {
        state.prompt("A source map maps SPICE lines; it cannot be attached to another dialect.");
        return false;
    }
    if !bundle_dependencies && !artifact.dependencies().is_empty() {
        state.prompt(
            "This netlist includes project or external files; export it as a self-contained \
             bundle.",
        );
        return false;
    }
    if !bundle_dependencies {
        return publish_generated_source(state, &artifact, format, io);
    }

    let bundle =
        match crate::io::build_generated_bundle(&artifact, format, include_source_map, |source| {
            generated_export_source(source, format)
        }) {
            Ok(bundle) => bundle,
            Err(error) => {
                state.report_failure("Netlist bundle export failed", error.to_string());
                return false;
            }
        };
    let default_name = format!(
        "generated-{}.zip",
        &artifact.content_digest().to_string()[..12]
    );
    let picked = io.show_save_dialog(SaveDialogConfig {
        title: "Export Generated Netlist Bundle",
        default_name: &default_name,
        filter_name: "RSpice Netlist Bundle",
        filter_extensions: &["zip"],
    });
    let Some(mut path) = (match picked {
        Ok(path) => path,
        Err(error) => {
            state.report_failure("Netlist bundle export failed", error.to_string());
            return false;
        }
    }) else {
        return false;
    };
    crate::io::file_exchange::ensure_file_extension(&mut path, "zip");
    let result = io.observe_destination(&path).and_then(|destination| {
        io.write_bytes_file_observed(&destination, &bundle, "application/zip")
    });
    match result {
        Ok(()) => {
            state.record(
                LogSeverity::Info,
                LogSource::Studio,
                crate::workbench::workflows::export_workflow::export_completion_message(
                    "netlist bundle",
                    &path,
                    None,
                    io,
                ),
            );
            true
        }
        Err(error) => {
            state.report_failure("Netlist bundle export failed", error.to_string());
            false
        }
    }
}

fn publish_generated_source(
    state: &mut AppState,
    artifact: &crate::state::GeneratedArtifact,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> bool {
    let source = generated_export_source(artifact.source(), format);
    let extension = format.extension();
    let filter_name = match format {
        crate::io::NetlistFormat::Spectre => "Spectre Netlist",
        crate::io::NetlistFormat::Spice => "SPICE Netlist",
        crate::io::NetlistFormat::Hspice => "HSPICE Netlist",
        crate::io::NetlistFormat::Xyce => "Xyce Netlist",
    };
    let default_name = format!(
        "generated-{}.{}",
        &artifact.content_digest().to_string()[..12],
        extension
    );
    let Some(mut path) = (match io.show_save_dialog(SaveDialogConfig {
        title: "Export Generated Netlist",
        default_name: &default_name,
        filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(path) => path,
        Err(error) => {
            state.report_failure("Netlist export failed", error.to_string());
            return false;
        }
    }) else {
        return false;
    };
    crate::io::file_exchange::ensure_file_extension(&mut path, extension);
    let result = io
        .observe_destination(&path)
        .and_then(|destination| io.write_text_file_observed(&destination, &source));
    match result {
        Ok(()) => {
            state.record(
                LogSeverity::Info,
                LogSource::Studio,
                crate::workbench::workflows::export_workflow::export_completion_message(
                    filter_name,
                    &path,
                    None,
                    io,
                ),
            );
            true
        }
        Err(error) => {
            state.report_failure("Netlist export failed", error.to_string());
            false
        }
    }
}

fn generated_export_source(source: &str, format: crate::io::NetlistFormat) -> String {
    match format {
        crate::io::NetlistFormat::Spectre => {
            super::netlist_compat::spice_to_ahdl_compatible_netlist(source)
        }
        crate::io::NetlistFormat::Spice
        | crate::io::NetlistFormat::Hspice
        | crate::io::NetlistFormat::Xyce => source.to_owned(),
    }
}

pub(crate) fn action_view_netlist(state: &mut AppState) {
    if state.model.schematic.document().components.is_empty() {
        state.prompt("No circuit to netlist. Add components first.");
        return;
    }

    let netlist_content = match build_menu_netlist(state, crate::io::NetlistFormat::Spice) {
        Ok(generated) => {
            record_netlist_warnings(state, generated.warnings);
            generated.source
        }
        Err(reasons) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                netlist_failure_text("Netlist generation", &reasons),
            );
            return;
        }
    };

    // The Netlist workspace this command opens shows the deck itself.
    let total_lines = netlist_content.lines().count();
    state.model.simulation.source.netlist_content = netlist_content;
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!("Generated the netlist ({total_lines} lines)."),
    );
}

/// A deck the generator wrote, and the warnings it raised beside it.
pub(crate) struct GeneratedNetlist {
    pub(crate) source: String,
    pub(crate) warnings: Vec<String>,
}

/// Generate the design's deck.
///
/// Nothing here writes the Console. A command that asked for the deck records
/// the outcome; the Netlist page states it where the deck would be. The error
/// is every reason the generator refused, in its order.
pub(crate) fn build_menu_netlist(
    state: &AppState,
    format: crate::io::NetlistFormat,
) -> Result<GeneratedNetlist, Vec<String>> {
    let execution_projection = state
        .model
        .workspace
        .configuration_execution_projection(
            &state.model.library_manager,
            &state.model.workspace.content.active_view,
            &state.model.schematic,
        )
        .map_err(|error| vec![error.to_string()])?;
    let root_reference = execution_projection.root().clone();
    let root_schematic = execution_projection
        .root_schematic()
        .expect("a successful execution projection has a materialized root");
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        state.model.library_manager.catalog(),
        &execution_projection,
    );
    let analysis_instances = state
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .into_iter()
        .flat_map(|plan| plan.instances())
        .filter(|instance| instance.enabled())
        .map(crate::simulation::plan::AnalysisInstance::id)
        .collect::<Vec<_>>();
    let Some(plan_id) = state
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
    else {
        return Err(vec![
            "the design has no active simulation plan to write the deck from".to_owned(),
        ]);
    };
    let Some(plan_payload) = state.model.workspace.content.plan_data(plan_id) else {
        return Err(vec![format!(
            "simulation plan {plan_id} has no configuration of its own"
        )]);
    };
    let generation = rspice_simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
        root_schematic,
        &[],
        &hierarchy,
        &plan_payload.design_variables,
        rspice_simulation::netlist_gen::DesignVariableNetlistContext {
            active_cell: &root_reference,
            analysis_instances: &analysis_instances,
        },
        &rspice_simulation::netlist_gen::NetlistSourceData::new(
            &crate::simulation::table_route::SourceFiles,
        ),
    );

    if !generation.errors.is_empty() {
        // The generator renders each typed defect into `errors` as well, so
        // the defects come first, in their own words, and an error that is
        // only a defect's rendering is not stated a second time.
        let mut reasons = generation
            .defects
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        for error in generation.errors {
            if !reasons.contains(&error) {
                reasons.push(error);
            }
        }
        return Err(reasons);
    }

    let spice_netlist = state
        .model
        .workspace
        .content
        .bind_generated_netlist_provenance(generation.netlist);
    let spice_netlist = crate::simulation::controller::SimulationController::append_plan_measurements_to_generated_netlist(&state.model, &spice_netlist)
        .map_err(|error| vec![error.to_string()])?;
    Ok(GeneratedNetlist {
        source: match format {
            crate::io::NetlistFormat::Spectre => {
                super::netlist_compat::spice_to_ahdl_compatible_netlist(&spice_netlist)
            }
            _ => spice_netlist,
        },
        warnings: generation.warnings,
    })
}

/// One Console line for a deck the reader asked for that could not be written:
/// what was asked, and every reason the generator gave.
pub(crate) fn netlist_failure_text(action: &str, reasons: &[String]) -> String {
    format!("{action} failed: {}", reasons.join("; "))
}

/// Record a generated deck's warnings, once each, as the Console's account of
/// what the generator substituted or ignored.
pub(crate) fn record_netlist_warnings(state: &mut AppState, warnings: Vec<String>) {
    for warning in warnings {
        state.record(
            crate::diagnostics::LogSeverity::Warning,
            crate::diagnostics::LogSource::Studio,
            warning,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_spectre_generated_exports_preserve_exact_source_bytes() {
        let source = "title μ\r\nR1 out 0 1k\r\n.end\r\n";
        for format in [
            crate::io::NetlistFormat::Spice,
            crate::io::NetlistFormat::Hspice,
            crate::io::NetlistFormat::Xyce,
        ] {
            assert_eq!(
                generated_export_source(source, format).as_bytes(),
                source.as_bytes()
            );
        }
    }
}
