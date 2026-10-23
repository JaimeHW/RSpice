//! What a new session opens on, for the workbench fields the slots now own.
//!
//! These fields were initialized by hand in the workbench's `Default`. The
//! slots derive theirs, except `SimulateState`, so each value a hand-written
//! initializer chose is pinned here as a literal: a variant reordered in its
//! enum, or a derived default replacing the written one, changes what every
//! new session shows.

use super::*;
use crate::workbench::lifecycle::project_lifecycle::ProjectCloseDestination;
use crate::workbench::state::{
    DesignPanel, LibraryCellviewPage, ProjectLauncherFilter, ProjectLauncherPage,
    ProjectLauncherSort, ProjectPage, SimulationPage, VerificationPage,
};

#[test]
fn moved_workbench_fields_keep_the_defaults_the_workbench_gave_them() {
    let simulate = SimulateState::default();
    // The plan opens on the transient analysis's tab, not on the first row.
    assert_eq!(simulate.active_analysis, 1);
    assert_eq!(simulate.verification_page, VerificationPage::Yield);
    assert_eq!(simulate.simulation_page, SimulationPage::Analyses);

    let project = ProjectState::default();
    assert_eq!(
        project.project_launcher_sort,
        ProjectLauncherSort::LastOpened
    );
    assert_eq!(project.project_launcher_filter, ProjectLauncherFilter::All);
    assert_eq!(project.project_launcher_page, ProjectLauncherPage::Projects);
    assert_eq!(
        project.project_close_destination,
        ProjectCloseDestination::EmptyWorkbench
    );
    assert_eq!(project.project_page, ProjectPage::Overview);
    assert_eq!(
        project.library_cellview_page,
        LibraryCellviewPage::Libraries
    );

    assert_eq!(DesignState::default().design_panel, DesignPanel::Navigator);
    assert_eq!(
        ModelsState::default().models_page,
        crate::state::ModelsPage::Models
    );
}
