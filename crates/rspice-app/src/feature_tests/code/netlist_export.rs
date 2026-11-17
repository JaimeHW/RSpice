//! File ▸ Export ▸ Netlist on the application: the SPICE export run outside
//! the application against the application's own run.

use crate::workbench::app::veriloga_flow_tests::{MODULE, run_plan, wired_device};
use rspice_app_code::workbench::menu_bar::netlist_actions::action_export_netlist_with_io;
use rspice_app_services::workbench::workflows::export_workflow::ExportWorkflowIo;

/// A save dialog that answers with `self.0` and writes that reach the disk.
struct SavesTo(std::path::PathBuf);

impl ExportWorkflowIo for SavesTo {
    fn show_save_dialog(
        &self,
        _config: rspice_app_services::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
    ) -> Result<Option<std::path::PathBuf>, String> {
        Ok(Some(self.0.clone()))
    }

    fn write_text_file(&self, path: &std::path::Path, contents: &str) -> Result<(), String> {
        let folder = path.parent().expect("the export has a folder");
        std::fs::create_dir_all(folder).map_err(|error| error.to_string())?;
        std::fs::write(path, contents).map_err(|error| error.to_string())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &rspice_app_services::io::WaveformDataset,
        _path: &std::path::Path,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// File ▸ Export ▸ Netlist in SPICE, saved to `path`.
pub(crate) fn export_spice(app: &mut crate::RSpiceApp, path: &std::path::Path) {
    action_export_netlist_with_io(
        &mut app.state.model,
        &mut app.state.workbench,
        &SavesTo(path.to_path_buf()),
    );
    if !path.exists() {
        panic!(
            "the export writes the deck: {:?} {:?}",
            app.state
                .workbench
                .prompt
                .as_ref()
                .map(|prompt| &prompt.text),
            app.state
                .model
                .notices
                .take()
                .into_iter()
                .map(|notice| format!("{}: {}", notice.title, notice.detail))
                .collect::<Vec<_>>()
        );
    }
}

/// A SPICE export runs outside the application as the design runs in it.
/// The device's Verilog-A is copied beside the deck and named by its
/// relative path, so the folder moved whole still runs; the deck carries the
/// plan's transient, and on the deck route it drives the device's output
/// through the same samples, to the bit, as the application's Run plan.
#[test]
fn a_spice_export_moved_whole_runs_as_the_application_runs_the_design() {
    let (mut user, device) = wired_device();
    let outcome = run_plan(&mut user, &device);
    let folder = std::env::temp_dir().join(format!("rspice_spice_export_{}", std::process::id()));
    let moved = folder.with_extension("moved");
    for stale in [&folder, &moved] {
        let _ = std::fs::remove_dir_all(stale);
    }
    export_spice(&mut user.app, &folder.join("amp.spice"));
    std::fs::rename(&folder, &moved).expect("the export moves whole");
    let path = moved.join("amp.spice");
    let deck = std::fs::read_to_string(&path).expect("the deck is written");

    assert!(!deck.contains("__rspice_"), "{deck}");
    assert!(
        deck.lines()
            .any(|line| line.trim_start().to_ascii_lowercase().starts_with(".tran")),
        "the deck carries the plan's transient:\n{deck}"
    );
    let (relative, alias) = deck
        .lines()
        .find_map(|line| {
            let rest = line.strip_prefix(".VERILOGA ")?;
            let (path, alias) = rest.trim_start_matches('"').split_once("\" ")?;
            Some((path.to_owned(), alias.trim().to_owned()))
        })
        .unwrap_or_else(|| panic!("the deck names its source:\n{deck}"));
    assert!(!std::path::Path::new(&relative).is_absolute(), "{relative}");
    assert_eq!(
        std::fs::read_to_string(moved.join(&relative)).expect("the source is beside the deck"),
        MODULE
    );
    let out = deck
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .find(|tokens| tokens.len() > 5 && tokens[5].eq_ignore_ascii_case(&alias))
        .map(|tokens| tokens[3].to_owned())
        .unwrap_or_else(|| panic!("the deck instantiates {alias}:\n{deck}"));

    let run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch_at(
        &deck,
        &path,
        rspice_results::run::SimulationRunLifecycle::Completed,
    );
    let _ = std::fs::remove_dir_all(&moved);
    // The deck route names a node's voltage by the node and keeps every
    // solver point; Run plan samples it on the transient's output grid. The
    // same solve sampled on that grid is the same trace.
    let name = format!("V({out})");
    let waves = || {
        run.analyses
            .iter()
            .flat_map(|analysis| analysis.waveforms.iter())
    };
    let exported = waves()
        .find(|wave| wave.name.eq_ignore_ascii_case(&out))
        .unwrap_or_else(|| {
            panic!(
                "the exported run keeps {name}: {:?}",
                waves().map(|wave| wave.name.as_str()).collect::<Vec<_>>()
            )
        });
    let sampled = rspice_simulation::output_contract::materialization::resample_selected_and_final(
        exported,
        output_grid(&deck),
    )
    .expect("the exported trace samples on the output grid");
    assert_eq!(
        sampled.y.len(),
        outcome.out_volts.len(),
        "{name} on the output grid"
    );
    for (exported, ran) in sampled.y.iter().zip(&outcome.out_volts) {
        assert_eq!(
            exported.to_bits(),
            ran.to_bits(),
            "{name}: {exported} against {ran}"
        );
    }
}

/// The output grid of the deck's `.tran` card: its step, from zero to its stop.
pub(crate) fn output_grid(
    deck: &str,
) -> rspice_simulation::output_contract::TransientSelectionGrid {
    let card = deck
        .lines()
        .find(|line| line.to_ascii_lowercase().starts_with(".tran "))
        .unwrap_or_else(|| panic!("the deck carries a transient:\n{deck}"));
    let values = card
        .split_whitespace()
        .skip(1)
        .map(|value| value.parse::<f64>().unwrap_or_else(|_| panic!("{card}")))
        .collect::<Vec<_>>();
    rspice_simulation::output_contract::TransientSelectionGrid {
        start: 0.0,
        step: values[0],
        stop: values[1],
    }
}
