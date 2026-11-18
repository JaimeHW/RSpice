//! The recent-projects list, one list wherever it is drawn: the launcher's
//! Projects page and the no-project landing. A table of Project, Location,
//! Last opened and the row's state and pin, under its column names. Each row
//! shows its full path and when it was last opened on hover or long-press
//! and, for a file an open found gone, a way to point the row at where it
//! went or take it off the list.

use std::path::Path;

use egui::{
    Align2, FontId, Id, Rect, Response, ScrollArea, Sense, Stroke, Ui, Vec2, WidgetInfo,
    WidgetType, pos2, vec2,
};

use crate::services::recent_files::{RecentFile, RecentKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Problem, paint_problem_mark};
use crate::workbench::design_system::{WorkbenchIcon, elide_middle, elide_text};
use crate::workbench::project::workspace_state::{ProjectLauncherFilter, ProjectLauncherSort};

/// What the reader asked of one recent row, besides opening it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecentRowAction {
    /// Pin the row, or unpin it.
    TogglePin(std::path::PathBuf),
    /// Point the row at the file where it now is, and open it.
    Locate(RecentFile),
    /// Take the row off the list.
    Remove(std::path::PathBuf),
}

/// When a recent project was last opened, on the reader's clock: the list is
/// this computer's own record of what it opened.
pub(crate) fn last_opened(opened_at_unix_ms: u64) -> String {
    if opened_at_unix_ms == 0 {
        return "\u{2014}".to_owned();
    }
    crate::time_compat::local_date_time(opened_at_unix_ms).map_or_else(
        || "\u{2014}".to_owned(),
        |stamp| {
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}",
                stamp.year(),
                u8::from(stamp.month()),
                stamp.day(),
                stamp.hour(),
                stamp.minute()
            )
        },
    )
}

/// Everything a row shortens: its full path, when it was last opened and how
/// long ago, and that the file was not found when it was.
pub(crate) fn full_text(recent: &RecentFile) -> String {
    let mut text = crate::services::recent_files::BrowserRecent::of(&recent.path).map_or_else(
        || recent.path.display().to_string(),
        |browser| format!("{}\n{}", browser.file_name, browser.store.location()),
    );
    if recent.opened_at_unix_ms != 0 {
        let now: u64 = crate::time_compat::unix_epoch()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);
        text.push_str(&format!(
            "\nLast opened {} ({})",
            last_opened(recent.opened_at_unix_ms),
            crate::time_compat::relative_age(std::time::Duration::from_millis(
                now.saturating_sub(recent.opened_at_unix_ms)
            ))
        ));
    }
    if recent.missing {
        text.push_str("\nThe file was not found when RSpice last tried to open it.");
    }
    text
}

/// Show `text` while `response` is hovered, and from a long-press until the
/// next press anywhere: a touch reader has no hover.
pub(crate) fn hover_and_long_press(ui: &Ui, response: Response, text: &str) -> Response {
    let held = Id::new("workbench.recent_rows.long_press");
    if response.long_touched() {
        ui.ctx()
            .data_mut(|data| data.insert_temp(held, response.id));
    } else if ui.input(|input| input.pointer.any_pressed())
        && ui
            .ctx()
            .data(|data| data.get_temp::<Id>(held))
            .is_some_and(|id| id == response.id)
    {
        ui.ctx().data_mut(|data| data.remove::<Id>(held));
    }
    let pressed_open = ui
        .ctx()
        .data(|data| data.get_temp::<Id>(held))
        .is_some_and(|id| id == response.id);
    if pressed_open {
        response.show_tooltip_text(text);
        response
    } else {
        response.on_hover_text(text)
    }
}

/// The side of a row's pin, at the metrics in force.
pub(crate) fn pin_side(ui: &Ui) -> f32 {
    let t = Tokens::get(ui.ctx());
    if t.metrics.is_touch() {
        t.metrics.touch_target()
    } else {
        24.0
    }
}

/// A row's pin, centred in `rect`: lit when the row is pinned. Returns
/// whether it was pressed.
pub(crate) fn pin_toggle(ui: &mut Ui, rect: Rect, pinned: bool, name: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let side = pin_side(ui).min(rect.height()).min(rect.width());
    let rect = Rect::from_center_size(rect.center(), Vec2::splat(side));
    let response = ui.interact(rect, ui.id().with(("recent-pin", name)), Sense::click());
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    WorkbenchIcon::Pin.paint(
        ui.painter(),
        Rect::from_center_size(rect.center(), Vec2::splat(12.0)),
        if pinned {
            t.color.accent
        } else if response.hovered() {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let label = if pinned {
        format!("Unpin {name}")
    } else {
        format!("Pin {name}")
    };
    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, true, pinned, &label));
    theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_text(if pinned { "Unpin" } else { "Pin to the top" });
    response.clicked()
}

/// The strip under a row whose file was not found: the two ways on, at the
/// row's right edge. The row's state column already says "missing" with its
/// mark. Returns the action pressed.
pub(crate) fn missing_strip(
    ui: &mut Ui,
    rect: Rect,
    recent: &RecentFile,
) -> Option<RecentRowAction> {
    let mut action = None;
    let mut strip = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    strip.spacing_mut().item_spacing.x = 6.0;
    if Button::new("Remove").show(&mut strip).clicked() {
        action = Some(RecentRowAction::Remove(recent.path.clone()));
    }
    // A browser has no path to point the row at: its file is picked again
    // through Open project, which lists it anew.
    if !cfg!(target_arch = "wasm32") && Button::new("Locate\u{2026}").show(&mut strip).clicked() {
        action = Some(RecentRowAction::Locate(recent.clone()));
    }
    action
}

/// The height of the strip [`missing_strip`] draws.
pub(crate) fn missing_strip_height(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.ctl_h + 8.0
}

/// One recent project as the list shows it.
#[derive(Debug, Clone)]
pub(crate) struct RecentProject {
    pub(crate) recent: RecentFile,
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) last_opened: String,
    /// The project open now: it wears the accent bar.
    pub(crate) current: bool,
    /// It opens from here. A browser opens only a project whose file it
    /// keeps a binding for; the others are reached through Browse.
    pub(crate) available: bool,
}

/// A recent project's name: its file's, without the extension.
pub(crate) fn project_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Unnamed project")
        .to_owned()
}

/// Whether a project pinned or not is listed under `filter`.
pub(crate) const fn matches_filter(filter: ProjectLauncherFilter, pinned: bool) -> bool {
    match filter {
        ProjectLauncherFilter::All => true,
        ProjectLauncherFilter::Pinned => pinned,
    }
}

/// The recent projects `query` and `filter` leave, in `sort`'s order. The
/// project open now, at `current`'s path, goes by its own name.
pub(crate) fn recent_projects(
    recent_files: &[RecentFile],
    query: &str,
    filter: ProjectLauncherFilter,
    sort: ProjectLauncherSort,
    current: Option<(&Path, &str)>,
) -> Vec<RecentProject> {
    let query = query.trim().to_lowercase();
    let mut projects = recent_files
        .iter()
        .filter(|recent| recent.kind == RecentKind::Project)
        .filter(|recent| matches_filter(filter, recent.pinned))
        .filter_map(|recent| {
            let current_name = current
                .filter(|(path, _)| *path == recent.path)
                .map(|(_, name)| name);
            let name = current_name.map_or_else(|| project_name(&recent.path), str::to_owned);
            let location = crate::services::recent_files::location(recent);
            let matches = query.is_empty()
                || name.to_lowercase().contains(&query)
                || location.to_lowercase().contains(&query);
            matches.then(|| RecentProject {
                recent: recent.clone(),
                name,
                location,
                last_opened: last_opened(recent.opened_at_unix_ms),
                current: current_name.is_some(),
                available: path_available(&recent.path),
            })
        })
        .collect::<Vec<_>>();
    match sort {
        ProjectLauncherSort::LastOpened => {}
        ProjectLauncherSort::Name => projects.sort_by(|left, right| {
            left.name
                .to_lowercase()
                .cmp(&right.name.to_lowercase())
                .then_with(|| left.location.cmp(&right.location))
        }),
    }
    projects
}

#[cfg(not(target_arch = "wasm32"))]
fn path_available(_path: &Path) -> bool {
    // Filesystem availability is verified only by the open itself. Probing
    // local, mapped, removable, or UNC paths from every paint pass can stall
    // the UI; an open that finds the file gone marks its row missing.
    true
}

/// A browser reopens only the projects it keeps a binding for.
#[cfg(target_arch = "wasm32")]
fn path_available(path: &Path) -> bool {
    crate::services::recent_files::BrowserRecent::of(path).is_some()
}

/// What a row's state column says, if anything, and the mark it earns. A
/// project that opens is the ordinary row and says nothing, and the current
/// project already wears the accent bar.
pub(crate) fn row_state(
    available: bool,
    missing: bool,
    browser: bool,
) -> Option<(&'static str, Option<Problem>)> {
    if !available && browser {
        Some(("browse", None))
    } else if missing {
        Some(("missing", Some(Problem::Warning)))
    } else {
        None
    }
}

fn project_state(project: &RecentProject) -> Option<(&'static str, Option<Problem>)> {
    row_state(
        project.available,
        project.recent.missing,
        cfg!(target_arch = "wasm32"),
    )
}

/// What the reader asked of the list in one frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RecentPick {
    /// Open this project.
    Open(RecentFile),
    /// A row's pin, Locate… or Remove.
    Row(RecentRowAction),
}

/// Below this width the list is one two-line row per project rather than a
/// table: a path column would leave the name no room.
pub(crate) const COMPACT_WIDTH: f32 = 620.0;
/// The narrowest the table is drawn; a narrower list scrolls across it.
const TABLE_MIN_WIDTH: f32 = 720.0;

/// The list in the rest of `ui`: the column names and a row per project,
/// scrolling under the names. `empty` draws what stands under the names
/// when no project is listed.
pub(crate) fn recent_projects_list(
    ui: &mut Ui,
    id_salt: &str,
    projects: &[RecentProject],
    empty: impl FnOnce(&mut Ui),
) -> Option<RecentPick> {
    let mut pick = None;
    let compact = ui.available_width() < COMPACT_WIDTH;
    let scroll = if compact {
        ScrollArea::vertical()
    } else {
        ScrollArea::both()
    };
    scroll
        .id_salt(id_salt)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            if compact {
                // Beside the scroll bar, not under it.
                ui.set_min_width(ui.available_width());
                compact_header(ui);
            } else {
                ui.set_min_width(TABLE_MIN_WIDTH.max(ui.available_width()));
                table_header(ui);
            }
            if projects.is_empty() {
                empty(ui);
            }
            for project in projects {
                let (response, action) = if compact {
                    compact_row(ui, project)
                } else {
                    table_row(ui, project)
                };
                if let Some(action) = action {
                    pick = Some(RecentPick::Row(action));
                } else if project.available && response_activated(ui, &response) {
                    pick = Some(RecentPick::Open(project.recent.clone()));
                }
            }
        });
    pick
}

fn response_activated(ui: &mut Ui, response: &Response) -> bool {
    response.clicked()
        || (response.has_focus()
            && ui.input_mut(|input| {
                input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    || input.consume_key(egui::Modifiers::NONE, egui::Key::Space)
            }))
}

/// A table row's height: the density's, and a finger's reach under a coarse
/// pointer, since a row opens with a press.
fn table_row_height(t: &Tokens) -> f32 {
    if t.metrics.is_touch() {
        t.metrics.touch_target()
    } else {
        t.metrics.ctl_h + 4.0
    }
}

/// The table's columns: the name, the path, the date, and a column that
/// holds the row's state and pin.
fn cell_rects(rect: Rect) -> [Rect; 4] {
    let fractions = [0.27_f32, 0.43, 0.19, 0.11];
    let mut cells = [rect; 4];
    let mut left = rect.left();
    for (index, fraction) in fractions.into_iter().enumerate() {
        let right = if index == fractions.len() - 1 {
            rect.right()
        } else {
            left + rect.width() * fraction
        };
        cells[index] = Rect::from_min_max(pos2(left, rect.top()), pos2(right, rect.bottom()));
        left = right;
    }
    cells
}

/// How a cell's text gives way when it does not fit.
#[derive(Clone, Copy)]
enum Elide {
    End,
    /// The drive and the file name tell two same-named projects apart, so a
    /// path gives way in its middle.
    Middle,
}

fn paint_cell(ui: &Ui, cell: Rect, text: &str, font: FontId, color: egui::Color32, elide: Elide) {
    if text.is_empty() {
        return;
    }
    let inset = cell.shrink2(vec2(8.0, 0.0));
    let text = match elide {
        Elide::End => elide_text(ui, text, &font, inset.width()),
        Elide::Middle => elide_middle(ui, text, &font, inset.width()),
    };
    ui.painter().with_clip_rect(inset).text(
        inset.left_center(),
        Align2::LEFT_CENTER,
        text,
        font,
        color,
    );
}

fn table_header(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width().max(TABLE_MIN_WIDTH), t.metrics.row_h),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    for (cell, name) in cell_rects(rect)
        .into_iter()
        .zip(["Project", "Location", "Last opened"])
    {
        paint_cell(ui, cell, name, font.clone(), t.color.text_faint, Elide::End);
    }
}

/// A row's ground: the hover, and the accent bar on the project open now.
fn paint_row_ground(ui: &Ui, rect: Rect, project: &RecentProject, response: &Response) {
    let t = Tokens::get(ui.ctx());
    if project.current || response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    if project.current {
        ui.painter().rect_filled(
            Rect::from_min_size(rect.min, vec2(2.0, rect.height())),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

/// A row presses only when its project opens from here.
fn row_sense(project: &RecentProject) -> Sense {
    if project.available {
        Sense::click()
    } else {
        Sense::hover()
    }
}

/// A table row: name, path elided in the middle, the date it was last
/// opened, and its state and pin; the missing strip under a file an open
/// found gone.
fn table_row(ui: &mut Ui, project: &RecentProject) -> (Response, Option<RecentRowAction>) {
    let t = Tokens::get(ui.ctx());
    let height = table_row_height(&t);
    let strip_height = if project.recent.missing {
        missing_strip_height(ui)
    } else {
        0.0
    };
    let (rect, response) = ui.allocate_exact_size(
        vec2(
            ui.available_width().max(TABLE_MIN_WIDTH),
            height + strip_height,
        ),
        row_sense(project),
    );
    paint_row_ground(ui, rect, project, &response);
    let cells = cell_rects(Rect::from_min_size(rect.min, vec2(rect.width(), height)));
    // A project that cannot be opened from here is faint, not red: its
    // state says why.
    let (name_color, date_color) = if project.available {
        (t.color.text_dim, t.color.text_dim)
    } else {
        (t.color.text_faint, t.color.text_faint)
    };
    let sans = theme::sans(tokens::FS_0, FontWeight::Regular);
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    paint_cell(ui, cells[0], &project.name, sans, name_color, Elide::End);
    paint_cell(
        ui,
        cells[1],
        &project.location,
        mono.clone(),
        t.color.text_faint,
        Elide::Middle,
    );
    paint_cell(
        ui,
        cells[2],
        &project.last_opened,
        mono,
        date_color,
        Elide::End,
    );
    let state = cells[3];
    let pin_side = pin_side(ui).min(state.height());
    let pin_cell = Rect::from_min_max(
        pos2(state.right() - 4.0 - pin_side, state.top()),
        pos2(state.right() - 4.0, state.bottom()),
    );
    if let Some((word, problem)) = project_state(project) {
        state_word(
            ui,
            pos2(pin_cell.left() - 4.0, state.center().y),
            word,
            problem,
        );
    }
    let mut action = pin_toggle(ui, pin_cell, project.recent.pinned, &project.name)
        .then(|| RecentRowAction::TogglePin(project.recent.path.clone()));
    if project.recent.missing {
        let strip = Rect::from_min_max(
            pos2(cells[0].left() + 8.0, state.bottom()),
            pos2(state.right() - 4.0, rect.bottom() - 4.0),
        );
        if let Some(strip_action) = missing_strip(ui, strip, &project.recent) {
            action = Some(strip_action);
        }
    }
    theme::paint_focus_ring(ui, &response, rect);
    (finish_row(ui, project, response), action)
}

/// A row's state word, with its mark when it has one, ending at `right`.
fn state_word(ui: &Ui, right: egui::Pos2, word: &str, problem: Option<Problem>) {
    let t = Tokens::get(ui.ctx());
    let shown = ui.painter().text(
        right,
        Align2::RIGHT_CENTER,
        word,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_dim,
    );
    if let Some(problem) = problem {
        use crate::ui::widgets::PROBLEM_MARK_SIDE;
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(
                    shown.left() - 4.0 - PROBLEM_MARK_SIDE * 0.5,
                    shown.center().y,
                ),
                Vec2::splat(PROBLEM_MARK_SIDE),
            ),
            problem,
        );
    }
}

/// The compact list's head: its two column names, said once.
fn compact_header(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), t.metrics.row_h), Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), "Project, Last opened")
    });
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    ui.painter().text(
        pos2(rect.left() + 8.0, rect.center().y),
        Align2::LEFT_CENTER,
        "Project",
        font.clone(),
        t.color.text_faint,
    );
    ui.painter().text(
        pos2(rect.right() - 8.0, rect.center().y),
        Align2::RIGHT_CENTER,
        "Last opened",
        font,
        t.color.text_faint,
    );
}

/// The space between a compact row's name and the path under it.
const COMPACT_LINE_GAP: f32 = 4.0;

/// A compact row's fonts: the project's name over its path.
fn compact_row_fonts() -> (FontId, FontId) {
    (
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
    )
}

/// A compact row's height: its name over its path with a bar's padding
/// above and below, and never less than a touch target, so a larger target
/// or larger text grows the row instead of crowding it.
pub(crate) fn compact_row_height(ui: &Ui) -> f32 {
    let metrics = Tokens::get(ui.ctx()).metrics;
    let (name, location) = compact_row_fonts();
    let lines = ui.fonts_mut(|fonts| fonts.row_height(&name) + fonts.row_height(&location));
    (lines + COMPACT_LINE_GAP + 2.0 * metrics.bar_pad).max(metrics.touch_target())
}

/// A compact row: name over path at the left, the date and the pin at the
/// right, and the missing strip under a file an open found gone.
fn compact_row(ui: &mut Ui, project: &RecentProject) -> (Response, Option<RecentRowAction>) {
    let t = Tokens::get(ui.ctx());
    let row_height = compact_row_height(ui);
    let strip_height = if project.recent.missing {
        missing_strip_height(ui)
    } else {
        0.0
    };
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), row_height + strip_height),
        row_sense(project),
    );
    paint_row_ground(ui, rect, project, &response);
    let main = Rect::from_min_size(rect.min, vec2(rect.width(), row_height));
    let pin_side = pin_side(ui);
    let right_width = 112.0 + pin_side;
    let left_width = (main.width() - right_width - 24.0).max(24.0);
    let (name_font, location_font) = compact_row_fonts();
    // The two lines sit as one block in the middle of the row, the date
    // level with the name and the state word level with the path.
    let (name_h, location_h) = ui.fonts_mut(|fonts| {
        (
            fonts.row_height(&name_font),
            fonts.row_height(&location_font),
        )
    });
    let block_top = main.center().y - (name_h + COMPACT_LINE_GAP + location_h) * 0.5;
    let name_y = block_top + name_h * 0.5;
    let location_y = block_top + name_h + COMPACT_LINE_GAP + location_h * 0.5;
    let name = elide_text(ui, &project.name, &name_font, left_width);
    let location = elide_middle(ui, &project.location, &location_font, left_width);
    ui.painter().text(
        pos2(main.left() + 9.0, name_y),
        Align2::LEFT_CENTER,
        name,
        name_font,
        if project.available {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    ui.painter().text(
        pos2(main.left() + 9.0, location_y),
        Align2::LEFT_CENTER,
        location,
        location_font,
        t.color.text_faint,
    );
    let date_right = main.right() - 9.0 - pin_side - 4.0;
    ui.painter().text(
        pos2(date_right, name_y),
        Align2::RIGHT_CENTER,
        &project.last_opened,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_dim,
    );
    if let Some((word, problem)) = project_state(project) {
        state_word(ui, pos2(date_right, location_y), word, problem);
    }
    let pin_cell = Rect::from_min_max(
        pos2(main.right() - 9.0 - pin_side, main.top()),
        pos2(main.right() - 9.0, main.bottom()),
    );
    let mut action = pin_toggle(ui, pin_cell, project.recent.pinned, &project.name)
        .then(|| RecentRowAction::TogglePin(project.recent.path.clone()));
    if project.recent.missing {
        let strip = Rect::from_min_max(
            pos2(main.left() + 9.0, main.bottom()),
            pos2(main.right() - 9.0, rect.bottom() - 4.0),
        );
        if let Some(strip_action) = missing_strip(ui, strip, &project.recent) {
            action = Some(strip_action);
        }
    }
    theme::paint_focus_ring(ui, &response, rect);
    (finish_row(ui, project, response), action)
}

/// What every row says to assistive technology and on hover or long-press.
/// Each row paints its own focus ring before this.
fn finish_row(ui: &Ui, project: &RecentProject, response: Response) -> Response {
    let enabled = project.available && ui.is_enabled();
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Button,
            enabled,
            format!("Open project {}", project.name),
        )
    });
    let facts = format!("{}; {}", project.location, project.last_opened);
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(match project_state(project) {
            Some((word, Some(problem))) => format!("{facts}; {}", problem.announce(word)),
            Some((word, None)) => format!("{facts}; {word}"),
            None => facts,
        });
    });
    if project.available {
        hover_and_long_press(ui, response, &full_text(&project.recent))
            .on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response.on_hover_text(
            "Reopen this project through Browse so browser file permission can be verified",
        )
    }
}
