//! The navigator's recovery rail: the project's checkpoints, told apart by
//! how long ago each was taken.

use egui::Ui;

use crate::workbench::design_system::{WorkbenchIcon, section_header};

use super::super::nav_row;
use super::ProjectNavigator;

pub(super) fn project_recovery_navigator(ui: &mut Ui, nav: &mut ProjectNavigator<'_>) {
    let query = nav.filter.trim().to_ascii_lowercase();
    let checkpoints = nav
        .checkpoint_recovery
        .checkpoints
        .iter()
        .filter(|checkpoint| {
            query.is_empty()
                || format!(
                    "{} {} {} {}",
                    checkpoint.title(),
                    checkpoint.reason().label(),
                    checkpoint.project_revision(),
                    checkpoint.checkpoint_id()
                )
                .to_ascii_lowercase()
                .contains(&query)
        })
        .cloned()
        .collect::<Vec<_>>();
    if nav.checkpoint_selection.is_none() {
        *nav.checkpoint_selection = checkpoints
            .first()
            .map(|checkpoint| checkpoint.checkpoint_id().to_string());
    }
    section_header(
        ui,
        "Project checkpoints",
        Some(&checkpoints.len().to_string()),
    );
    // Every checkpoint is restorable, so the meta that said so told the
    // reader nothing and left several "Autosave" rows indistinguishable. The
    // time is what tells them apart.
    let now_unix_ms = crate::time_compat::checked_unix_time_ms().ok();
    for checkpoint in &checkpoints {
        let id = checkpoint.checkpoint_id().to_string();
        let age = checkpoint_age(checkpoint.created_unix_ms(), now_unix_ms);
        if nav_row(
            ui,
            WorkbenchIcon::History,
            checkpoint.title(),
            nav.checkpoint_selection.as_deref() == Some(id.as_str()),
            Some(&age),
        ) {
            *nav.checkpoint_selection = Some(id);
        }
    }
}

/// How long ago a checkpoint was taken, for the rail's row meta.
///
/// A checkpoint stamped after the clock's present says the clock moved, not
/// how old the checkpoint is, so that case reports nothing.
fn checkpoint_age(created_unix_ms: u64, now_unix_ms: Option<u64>) -> String {
    let Some(elapsed) = now_unix_ms
        .filter(|_| created_unix_ms != 0)
        .and_then(|now| now.checked_sub(created_unix_ms))
    else {
        return String::new();
    };
    crate::time_compat::relative_age(std::time::Duration::from_millis(elapsed))
}

// Named `tests` rather than for the rails it covers: `source_guard` locates a
// file's production half by this exact boundary, and a module under any other
// name leaves every guard that scans this file panicking instead of scanning.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_checkpoint_with_no_usable_stamp_reports_no_age_rather_than_a_wrong_one() {
        assert_eq!(checkpoint_age(0, Some(10_000)), "");
        assert_eq!(checkpoint_age(20_000, Some(10_000)), "");
        assert_eq!(checkpoint_age(10_000, None), "");
        assert_eq!(checkpoint_age(9_000, Some(10_000)), "just now");
        assert_eq!(checkpoint_age(10_000, Some(190_000)), "3 min ago");
        assert_eq!(checkpoint_age(0, None), "");
    }
}
