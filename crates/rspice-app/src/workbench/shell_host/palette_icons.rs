//! The glyphs the command palette gives commands of its own, before it falls
//! back to the glyph of a command's scope.

use crate::state::command_catalog::CommandKey;
use crate::ui::icons::Icon;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::models::commands::ModelsCommand;
use crate::workbench::results::commands::ResultsCommand;

/// The glyph the palette gives `command`; `None` for a command that wears its
/// scope's.
pub(super) fn palette_icon(command: Command) -> Option<Icon> {
    let icon = match command {
        Command::PlaceWire | Command::JoinCollinearWires => Icon::Wire,
        Command::PlaceBus => Icon::Bus,
        Command::PlaceBusTap => Icon::Bus,
        Command::PlaceJunction => Icon::Grid,
        Command::PlaceLabel => Icon::NetLabel,
        Command::PlaceProbe => Icon::Probe,
        Command::PlacePin => Icon::Pin,
        Command::PlaceText => Icon::NetLabel,
        Command::PlaceShape => Icon::Grid,
        Command::MoveSelection | Command::StretchSelection => Icon::Select,
        Command::ArraySelection => Icon::Grid,
        Command::ReplaceInstance => Icon::Refresh,
        Command::CreateHierarchy => Icon::Folder,
        Command::ConnectivityManager => Icon::Bus,
        Command::ConfigurationSets => Icon::Grid,
        Command::RuleSetup => Icon::Check,
        Command::RunSimulation => Icon::Run,
        Command::StopSimulation => Icon::Stop,
        Command::CheckCellview
        | Command::CheckDesign
        | Command::CheckAndSave
        | Command::PreflightChecks => Icon::Check,
        Command::OpenProject
        | Command::ProjectLauncher
        | Command::RecentProjects
        | Command::OpenDocument => Icon::Folder,
        Command::NewProject
        | Command::Save
        | Command::SaveAs
        | Command::SaveAll
        | Command::ImportNetlist
        | Command::ImportVerilogA
        | Command::ExportNetlist(_) => Icon::File,
        _ => return None,
    };
    Some(icon)
}

/// The glyph the palette gives a feature's command; `None` for one that
/// wears its scope's.
pub(super) fn feature_palette_icon(key: CommandKey) -> Option<Icon> {
    if let Some(command) = HardcopyCommand::from_key(key) {
        return match command {
            HardcopyCommand::ExportSchematicSvg => Some(Icon::File),
            _ => None,
        };
    }
    if ModelsCommand::from_key(key) == Some(ModelsCommand::ModelBrowser) {
        return Some(Icon::Folder);
    }
    match ResultsCommand::from_key(key)? {
        ResultsCommand::ExportWaveformsCsv => Some(Icon::File),
        _ => None,
    }
}
