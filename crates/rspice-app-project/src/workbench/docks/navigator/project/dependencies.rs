//! The navigator's dependency rail: the design and model libraries the
//! project depends on, each with its status, in the table's order under the
//! filter the table reads. The rows are built when what a status reads
//! changes, not on every frame.

use std::borrow::Cow;

use egui::{Rect, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, paint_problem_mark};
use crate::workbench::design_system::{WorkbenchIcon, section_header};
use crate::workbench::project::dependency_status::{
    DependencyKind, refresh_source_drift, settle_selection, visible_dependency_rows,
};

use super::super::nav_row_response;
use super::ProjectNavigator;

/// Three cells of the row's mono meta face: wider than a mark and its gap.
const MARK_ROOM: &str = "   ";

pub(super) fn project_dependency_navigator(ui: &mut Ui, nav: &mut ProjectNavigator<'_>) {
    // The rail draws before the page, so it is the first to bring the drift
    // report up to the catalogue the statuses describe.
    refresh_source_drift(ui.ctx(), nav.model);
    let (rows, visible) = visible_dependency_rows(ui.ctx(), nav.model, nav.filter);
    settle_selection(&rows, &visible, nav.dependency_selection);
    section_header(ui, "Dependencies", Some(&visible.len().to_string()));
    for &index in visible.iter() {
        let row = &rows[index];
        let icon = match row.library.kind {
            DependencyKind::Design => WorkbenchIcon::Project,
            DependencyKind::Model => WorkbenchIcon::Models,
        };
        let selected = nav.dependency_selection.as_ref() == Some(&row.library);
        let problem = row.status.mark.problem();
        // A marked status keeps the mark's room in front of its word, so the
        // name is elided short of the mark rather than run under it.
        let meta = match problem {
            Some(_) => Cow::Owned(format!("{MARK_ROOM}{}", row.status.label)),
            None => Cow::Borrowed(row.status.label),
        };
        let response = nav_row_response(ui, icon, &row.library.name, selected, Some(&meta));
        if let Some(problem) = problem {
            paint_status_mark(ui, response.rect, row.status.label, problem);
            let announced =
                problem.announce(&format!("{}, {}", row.library.name, row.status.label));
            response.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    &announced,
                )
            });
        }
        if response.clicked() {
            *nav.dependency_selection = Some(row.library.clone());
        }
    }
}

/// The status's mark, set in front of the row's status word, where the
/// table and the inspector set it in front of theirs.
fn paint_status_mark(ui: &Ui, row: Rect, label: &str, problem: Problem) {
    let t = Tokens::get(ui.ctx());
    let label_width = ui
        .painter()
        .layout_no_wrap(
            label.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        )
        .size()
        .x;
    let right = row.right() - 8.0 - label_width - (PROBLEM_MARK_ADVANCE - PROBLEM_MARK_SIDE);
    paint_problem_mark(
        ui.painter(),
        Rect::from_center_size(
            pos2(right - PROBLEM_MARK_SIDE * 0.5, row.center().y),
            vec2(PROBLEM_MARK_SIDE, PROBLEM_MARK_SIDE),
        ),
        problem,
    );
}
