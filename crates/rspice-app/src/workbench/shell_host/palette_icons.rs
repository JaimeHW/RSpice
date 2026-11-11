//! The glyphs the command palette gives commands of its own, before it falls
//! back to the glyph of a command's scope.

use crate::state::command_catalog::CommandKey;
use crate::ui::icons::Icon;
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::models::commands::ModelsCommand;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::simulate::commands::SimulateCommand;

/// The glyph the palette gives `command`; `None` for a command that wears its
/// scope's.
pub(super) fn palette_icon(command: Command) -> Option<Icon> {
    let icon = match command {
        Command::SelectTool => Icon::Select,
        Command::JoinCollinearWires => Icon::Wire,
        Command::MoveSelection | Command::StretchSelection => Icon::Select,
        Command::ArraySelection => Icon::Grid,
        Command::SwapToRSpiceSymbols => Icon::Refresh,
        Command::ConfigurationSets => Icon::Grid,
        Command::CheckCellview | Command::CheckDesign => Icon::Check,
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
            | CodeCommand::ExportNetlist(_)
            | CodeCommand::ExportBillOfMaterials => Some(Icon::File),
            _ => None,
        };
    }
    if let Some(command) = DesignCommand::from_key(key) {
        return match command {
            DesignCommand::ConnectivityManager => Some(Icon::Bus),
            DesignCommand::RuleSetup => Some(Icon::Check),
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
            | ProjectCommand::SaveAll => Some(Icon::File),
            _ => None,
        };
    }
    match ResultsCommand::from_key(key)? {
        ResultsCommand::ExportWaveformsCsv => Some(Icon::File),
        _ => None,
    }
}
