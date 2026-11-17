//! The glyphs the command palette gives commands of its own, before it falls
//! back to the glyph of a command's scope.

use crate::workbench::commands::vocabulary::ShellCommand;
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_hardcopy::workbench::hardcopy::commands::HardcopyCommand;
use rspice_app_models::workbench::models::commands::ModelsCommand;
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_results::workbench::results::commands::ResultsCommand;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::state::command_catalog::CommandKey;
use rspice_app_workbench::ui::icons::Icon;

/// The glyph the palette gives `command`; `None` for a command that wears its
/// scope's.
pub(super) fn palette_icon(command: ShellCommand) -> Option<Icon> {
    let icon = match command {
        ShellCommand::ConfigurationSets => Icon::Grid,
        _ => return None,
    };
    Some(icon)
}

/// The glyph the palette gives a feature's command; `None` for one that
/// wears its scope's.
pub(super) fn feature_palette_icon(key: CommandKey) -> Option<Icon> {
    if HardcopyCommand::from_key(key).is_some() {
        return None;
    }
    if let Some(command) = SimulateCommand::from_key(key) {
        return match command {
            SimulateCommand::Run => Some(Icon::Run),
            SimulateCommand::Stop => Some(Icon::Stop),
            SimulateCommand::Preflight => Some(Icon::Check),
            _ => None,
        };
    }
    if let Some(command) = CodeCommand::from_key(key) {
        return match command {
            CodeCommand::ImportNetlist
            | CodeCommand::ImportVerilogA
            | CodeCommand::ExportNetlist
            | CodeCommand::ExportBillOfMaterials => Some(Icon::File),
            _ => None,
        };
    }
    if let Some(command) = DesignCommand::from_key(key) {
        return match command {
            DesignCommand::ConnectivityManager => Some(Icon::Bus),
            DesignCommand::RuleSetup => Some(Icon::Check),
            DesignCommand::SelectTool => Some(Icon::Select),
            // Placing an instance picks one from the library.
            DesignCommand::PlaceInstance => Some(Icon::Library),
            DesignCommand::PlaceOffSheetConnector => Some(Icon::ChevronRight),
            DesignCommand::PlaceWire => Some(Icon::Wire),
            DesignCommand::PlaceBus | DesignCommand::PlaceBusTap => Some(Icon::Bus),
            DesignCommand::PlaceJunction | DesignCommand::PlaceShape => Some(Icon::Grid),
            DesignCommand::PlaceLabel | DesignCommand::PlaceText => Some(Icon::NetLabel),
            DesignCommand::PlaceProbe => Some(Icon::Probe),
            DesignCommand::PlacePin => Some(Icon::Pin),
            DesignCommand::ReplaceInstance => Some(Icon::Refresh),
            DesignCommand::CreateHierarchy => Some(Icon::Folder),
            DesignCommand::CheckCellview | DesignCommand::CheckDesign => Some(Icon::Check),
            DesignCommand::JoinCollinearWires => Some(Icon::Wire),
            DesignCommand::MoveSelection | DesignCommand::StretchSelection => Some(Icon::Select),
            DesignCommand::ArraySelection => Some(Icon::Grid),
            DesignCommand::SwapToRSpiceSymbols => Some(Icon::Refresh),
            _ => None,
        };
    }
    if ModelsCommand::from_key(key) == Some(ModelsCommand::ModelBrowser) {
        return Some(Icon::Folder);
    }
    if let Some(command) = ProjectCommand::from_key(key) {
        return match command {
            ProjectCommand::CheckAndSave => Some(Icon::Check),
            ProjectCommand::OpenProject
            | ProjectCommand::ProjectLauncher
            | ProjectCommand::OpenSchematic => Some(Icon::Folder),
            ProjectCommand::NewProject
            | ProjectCommand::Save
            | ProjectCommand::SaveAs
            | ProjectCommand::SaveCopy
            | ProjectCommand::SaveAll => Some(Icon::File),
            _ => None,
        };
    }
    match ResultsCommand::from_key(key)? {
        ResultsCommand::ExportWaveformsCsv => Some(Icon::File),
        _ => None,
    }
}
