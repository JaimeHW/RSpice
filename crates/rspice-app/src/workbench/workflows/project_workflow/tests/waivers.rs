//! A waiver is stored in the document of the cellview it waives, so it is
//! saved with that document and read back with it.

use super::*;
use crate::state::Point;
use crate::workbench::app_state::diagnostic_store::{DiagnosticInputs, DiagnosticRecord};
use crate::workbench::app_state::waivers::WaiverAction;
use crate::workbench::lifecycle::project_lifecycle::{
    DestinationAuthority, SaveScope, save_native,
};

const COLLISION: &str = "DRC-CASE-COLLIDING-NET-NAMES";

const REASON: &str = "sense is tied to SENSE on purpose";

/// Run the Check command and return its case collision.
fn collision(state: &mut AppState) -> DiagnosticRecord {
    state
        .run_active_design_checks()
        .expect("the checks run on the drawing");
    let subject = state.workspace.content.active_schematic_reference();
    state
        .diagnostics
        .batch(&DiagnosticInputs::DesignCheck { subject })
        .expect("the checks published their findings")
        .records
        .iter()
        .find(|record| &*record.key.code == COLLISION)
        .cloned()
        .expect("the collision is found")
}

#[test]
fn a_waiver_survives_save_and_reopen() {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    state
        .schematic
        .with_undo("Label the collision", |schematic| {
            schematic.add_net_label(Point::new(600, 600), "sense".to_owned());
            schematic.add_net_label(Point::new(600, 700), "SENSE".to_owned());
        });
    state.sync_active_schematic_to_workspace();
    let target = match collision(&mut state).waiver_action {
        Some(WaiverAction::Waive(target)) => target,
        other => panic!("a case collision may be waived: {other:?}"),
    };
    state
        .waive_finding(&target, REASON)
        .expect("a case collision may be waived");
    let stored = state.schematic.document().waivers.clone();

    let path =
        std::env::temp_dir().join(format!("rspice-waiver-{}.rspiceproj", uuid::Uuid::new_v4()));
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("the project saves");
    let mut reopened = AppState::default();
    let opened = load_project_from_path(&mut reopened, &path);
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
    assert!(opened, "the saved project reopens");

    assert_eq!(reopened.schematic.document().waivers, stored);
    let finding = collision(&mut reopened);
    let waiver = finding
        .canonical
        .suppression
        .as_ref()
        .expect("the reopened finding is waived");
    assert_eq!(waiver.reason, REASON);
    assert_eq!(waiver.author, stored.entries()[0].author);
    assert!(matches!(
        finding.waiver_action,
        Some(WaiverAction::Remove(_))
    ));
}
