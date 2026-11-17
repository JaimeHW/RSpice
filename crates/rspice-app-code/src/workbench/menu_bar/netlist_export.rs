//! Netlist menu actions.
//!
//! Generates the design's deck for File ▸ Export ▸ Netlist and the Netlist
//! workspace's generation and export, and records what the generator raised
//! beside it.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub fn action_export_generated_netlist_with_options(
    model: &mut crate::workbench::app_state::Model,
    wb: &mut crate::workbench::state::WorkbenchSession,
    bundle_dependencies: bool,
    include_source_map: bool,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> bool {
    let Some(document) = model.netlist_artifacts.generated_document.as_ref() else {
        wb.prompt("There is no generated netlist to export.");
        return false;
    };
    let Some(artifact) = document.generated_artifact().cloned() else {
        wb.prompt("The selected document is not a generated netlist.");
        return false;
    };
    if model.netlist_artifacts.generation_error.is_some()
        || model.netlist_artifacts.generated_input_digest
            != model.netlist_artifacts.current_generation_input_digest
        || model.netlist_artifacts.generated_input_digest
            != Some(artifact.provenance().input().digest())
    {
        wb.prompt("The generated netlist is out of date; regenerate it to export.");
        return false;
    }
    if include_source_map && !bundle_dependencies {
        wb.prompt("A source map is exported only with a self-contained bundle.");
        return false;
    }
    if !bundle_dependencies && !artifact.dependencies().is_empty() {
        wb.prompt(
            "This netlist includes project or external files; export it as a self-contained \
             bundle.",
        );
        return false;
    }
    if !bundle_dependencies {
        return publish_generated_source(model, &artifact, io);
    }

    let bundle = match crate::io::build_generated_bundle(&artifact, include_source_map) {
        Ok(bundle) => bundle,
        Err(error) => {
            model.report_failure("Netlist bundle export failed", error.to_string());
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
            model.report_failure("Netlist bundle export failed", error.to_string());
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
            model.record(
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
            model.report_failure("Netlist bundle export failed", error.to_string());
            false
        }
    }
}

fn publish_generated_source(
    model: &mut crate::workbench::app_state::Model,
    artifact: &crate::state::GeneratedArtifact,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> bool {
    let source = artifact.source();
    let extension = "spice";
    let filter_name = "SPICE Netlist";
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
            model.report_failure("Netlist export failed", error.to_string());
            return false;
        }
    }) else {
        return false;
    };
    crate::io::file_exchange::ensure_file_extension(&mut path, extension);
    let result = io
        .observe_destination(&path)
        .and_then(|destination| io.write_text_file_observed(&destination, source));
    match result {
        Ok(()) => {
            model.record(
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
            model.report_failure("Netlist export failed", error.to_string());
            false
        }
    }
}

/// A deck the generator wrote, and the warnings it raised beside it.
pub struct GeneratedNetlist {
    pub source: String,
    pub warnings: Vec<String>,
    /// What each line of `source` was written for, one entry per line.
    pub line_map: Vec<rspice_simulation::netlist_gen::WrittenLine>,
}

/// The shell's `build_menu_netlist`, over the model.
pub fn build_menu_netlist_in(
    model: &crate::workbench::app_state::Model,
) -> Result<GeneratedNetlist, Vec<String>> {
    let execution_projection = model
        .workspace
        .configuration_execution_projection(
            &model.library_manager,
            &model.workspace.content.active_view,
            &model.schematic,
        )
        .map_err(|error| vec![error.to_string()])?;
    let root_reference = execution_projection.root().clone();
    let root_schematic = execution_projection
        .root_schematic()
        .expect("a successful execution projection has a materialized root");
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        model.library_manager.catalog(),
        &execution_projection,
    );
    let analysis_instances = model
        .sim_setup
        .analysis_plan
        .as_ref()
        .into_iter()
        .flat_map(|plan| plan.instances())
        .filter(|instance| instance.enabled())
        .map(crate::simulation::plan::AnalysisInstance::id)
        .collect::<Vec<_>>();
    let Some(plan_id) = model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
    else {
        return Err(vec![
            "the design has no active simulation plan to write the deck from".to_owned(),
        ]);
    };
    let Some(plan_payload) = model.workspace.content.plan_data(plan_id) else {
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
    // The generator's own text stays for the line map, which the Added lines
    // join as lines written for nothing.
    let netlist = rspice_simulation::netlist_gen::append_added_lines(
        generation.netlist.clone(),
        &root_reference,
        &model.workspace.content.added_lines,
    )
    .map_err(|refused| vec![refused.to_string()])?;

    let spice_netlist = model
        .workspace
        .content
        .bind_generated_netlist_provenance(netlist);
    let spice_netlist = crate::simulation::controller::SimulationController::append_plan_measurements_to_generated_netlist(model, &spice_netlist)
        .map_err(|error| vec![error.to_string()])?;
    let line_map = carry_line_map(&generation.netlist, generation.line_map, &spice_netlist);
    Ok(GeneratedNetlist {
        source: spice_netlist,
        warnings: generation.warnings,
        line_map,
    })
}

/// The generator's line map carried onto the netlist it became.
///
/// The provenance comments and the plan's measurements are only ever inserted
/// between the generator's lines, so each of those keeps what it was written
/// for, and an inserted line is written for nothing.
fn carry_line_map(
    written: &str,
    map: Vec<rspice_simulation::netlist_gen::WrittenLine>,
    netlist: &str,
) -> Vec<rspice_simulation::netlist_gen::WrittenLine> {
    let mut written = written.split('\n').zip(map).peekable();
    netlist
        .split('\n')
        .map(|line| match written.peek() {
            Some((text, _)) if *text == line => {
                written.next().map(|(_, origin)| origin).unwrap_or_default()
            }
            _ => rspice_simulation::netlist_gen::WrittenLine::default(),
        })
        .collect()
}

/// One Console line for a deck the reader asked for that could not be written:
/// what was asked, and every reason the generator gave.
pub fn netlist_failure_text(action: &str, reasons: &[String]) -> String {
    format!("{action} failed: {}", reasons.join("; "))
}

/// The shell's `record_netlist_warnings`, in the model's Console.
pub fn record_netlist_warnings_in(
    model: &mut crate::workbench::app_state::Model,
    warnings: Vec<String>,
) {
    for warning in warnings {
        model.record(
            crate::diagnostics::LogSeverity::Warning,
            crate::diagnostics::LogSource::Studio,
            warning,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What the export wrote, and where.
    #[derive(Default)]
    struct CaptureIo {
        written: std::cell::RefCell<Option<(std::path::PathBuf, String)>>,
    }

    impl ExportWorkflowIo for CaptureIo {
        fn show_save_dialog(
            &self,
            _config: SaveDialogConfig<'_>,
        ) -> Result<Option<std::path::PathBuf>, String> {
            Ok(Some(std::path::PathBuf::from("generated")))
        }

        fn write_text_file(&self, path: &std::path::Path, contents: &str) -> Result<(), String> {
            self.written
                .replace(Some((path.to_path_buf(), contents.to_owned())));
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &std::path::Path,
        ) -> Result<(), String> {
            Ok(())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            false
        }
    }

    #[test]
    fn spice_generated_exports_preserve_exact_source_bytes() {
        use crate::product::{ContentDigest, ObjectRevision};
        use crate::state::{GeneratedArtifact, GeneratedProvenance, GenerationInput};

        let source = "title μ\r\nR1 out 0 1k\r\n.end\r\n";
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "export-test",
                GenerationInput::new(ObjectRevision::INITIAL, ContentDigest::from_bytes([7; 32])),
            )
            .expect("provenance"),
            source.as_bytes().to_vec(),
            Vec::new(),
            Vec::new(),
        )
        .expect("artifact");
        let mut model = crate::workbench::app_state::Model::session_start(
            crate::state::pdk_config::PdkConfig::default(),
            None,
            rspice_app_state::workbench::workspace::restore_model_library_manager,
        );
        let io = CaptureIo::default();

        assert!(publish_generated_source(&mut model, &artifact, &io));
        let (path, written) = io.written.take().expect("the deck is written");
        assert_eq!(written.as_bytes(), source.as_bytes());
        assert_eq!(path.extension().and_then(|e| e.to_str()), Some("spice"));
    }
}
