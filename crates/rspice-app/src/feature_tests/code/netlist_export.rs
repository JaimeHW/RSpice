//! A Spectre export of a design holding a Verilog-A device, saved through
//! File ▸ Export ▸ Netlist on the application.

use crate::workbench::app::veriloga_flow_tests::{CELL, MODULE, wired_device};
use crate::workbench::menu_bar::action_export_netlist_with_io;
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;

/// A save dialog that answers with `self.0` and writes that reach the disk.
struct SavesTo(std::path::PathBuf);

impl ExportWorkflowIo for SavesTo {
    fn show_save_dialog(
        &self,
        _config: crate::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
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
        _dataset: &crate::io::WaveformDataset,
        _path: &std::path::Path,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn a_spectre_export_copies_its_sources_beside_the_deck_and_names_them_relatively() {
    let (mut user, device) = wired_device();
    let folder = std::env::temp_dir().join(format!("rspice_spectre_export_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    let deck_path = folder.join("amp.scs");

    action_export_netlist_with_io(
        &mut user.app.state,
        crate::io::NetlistFormat::Spectre,
        &SavesTo(deck_path.clone()),
    );

    let deck = std::fs::read_to_string(&deck_path).expect("the deck is written");
    assert!(!deck.contains("__rspice_project__"), "{deck}");
    let relative = deck
        .lines()
        .find_map(|line| line.strip_prefix("ahdl_include "))
        .unwrap_or_else(|| panic!("the deck includes its source:\n{deck}"))
        .trim_matches('"');
    assert!(!std::path::Path::new(relative).is_absolute(), "{relative}");
    assert_eq!(
        std::fs::read_to_string(folder.join(relative)).expect("the source is beside the deck"),
        MODULE
    );
    assert!(
        deck.contains(&format!(
            "// {}/{CELL}/veriloga (module {CELL})",
            device.view.library
        )),
        "{deck}"
    );
    assert!(
        deck.lines()
            .any(|line| line.starts_with('X') && line.split(' ').any(|token| token == CELL)),
        "the instance names the module the source declares:\n{deck}"
    );
    let read = rspice_core::netlist::Netlist::parse_with_path(&deck, &deck_path)
        .unwrap_or_else(|error| panic!("the deck reads back: {error}\n{deck}"));
    assert!(
        read.elements
            .iter()
            .any(|element| element.name.eq_ignore_ascii_case("X1")),
        "{deck}"
    );
    let _ = std::fs::remove_dir_all(&folder);
}
