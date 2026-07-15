//! Mockup-governed command palette (Ctrl+K).
//!
//! This is a dedicated command-search modal, not a generic dialog. Its
//! geometry and visual hierarchy mirror the reviewed workbench mockup: a
//! 650x560 desktop surface with a 48 px search row, a 37 px horizontally
//! scrollable scope strip, 40 px result rows, and a 31 px keyboard footer.
//! At the mockup's 560 px breakpoint the surface uses 6 px viewport gutters
//! and the footer collapses to the local-index count. The local index is a
//! typed projection of the production command registry plus the current
//! project's real cellviews, loaded models, and active-run waveforms. Every
//! row resolves back to its authoritative object before it executes.

use std::borrow::Cow;

use egui::{
    Align, Context, FocusDirection, Id, Key, Margin, Modifiers, Order, Rect, Sense, Stroke, Ui,
    UiBuilder, UiKind, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Mode, Tokens};
use crate::workbench::commands::{
    Command, CommandAvailability, command_catalog, current_command_platform,
};

use super::RSpiceApp;

const PALETTE_ID: &str = "rspice.command-palette";
const DIALOG_LABEL: &str = "Search and run a command";
const DIALOG_DESCRIPTION: &str = "Type to filter commands, cellviews, models, and signals. Use Up and Down Arrow to select a result and Enter to open it.";

const DESKTOP_MAX_WIDTH: f32 = 650.0;
const DESKTOP_MAX_HEIGHT: f32 = 560.0;
const DESKTOP_HORIZONTAL_GUTTERS: f32 = 28.0;
const DESKTOP_VERTICAL_GUTTERS: f32 = 48.0;
const DESKTOP_TOP_FRACTION: f32 = 0.08;
const DESKTOP_TOP_MIN: f32 = 36.0;
const NARROW_MAX_WIDTH: f32 = 560.0;
const NARROW_GUTTER: f32 = 6.0;

const SEARCH_HEIGHT: f32 = 48.0;
const SCOPE_HEIGHT: f32 = 37.0;
// 44 px coarse-pointer chip + 7 px block padding on both sides + 1 px rule.
const SCOPE_TOUCH_HEIGHT: f32 = 59.0;
const SCOPE_CHIP_HEIGHT: f32 = 22.0;
const TOUCH_TARGET: f32 = 44.0;
const FOOTER_HEIGHT: f32 = 31.0;
const RESULT_ROW_HEIGHT: f32 = 40.0;
const RESULT_ICON_COLUMN: f32 = 28.0;
const RESULT_ICON_SIDE: f32 = 26.0;
const RESULT_GROUP_HEIGHT: f32 = 22.0;
const MAX_RECENT: usize = 8;
const SUGGESTION_LIMIT: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq)]
struct PaletteLayout {
    surface: Rect,
    narrow: bool,
    coarse: bool,
    scope_height: f32,
}

impl PaletteLayout {
    fn resolve(viewport: Rect, has_touch_screen: bool) -> Self {
        let narrow = viewport.width() <= NARROW_MAX_WIDTH;
        let coarse = narrow || has_touch_screen;
        let surface = if narrow {
            Rect::from_min_max(
                viewport.min + vec2(NARROW_GUTTER, NARROW_GUTTER),
                viewport.max - vec2(NARROW_GUTTER, NARROW_GUTTER),
            )
        } else {
            let width =
                DESKTOP_MAX_WIDTH.min((viewport.width() - DESKTOP_HORIZONTAL_GUTTERS).max(1.0));
            let height =
                DESKTOP_MAX_HEIGHT.min((viewport.height() - DESKTOP_VERTICAL_GUTTERS).max(1.0));
            let top_offset = (viewport.height() * DESKTOP_TOP_FRACTION).max(DESKTOP_TOP_MIN);
            Rect::from_min_size(
                pos2(
                    viewport.center().x - width * 0.5,
                    viewport.top() + top_offset,
                ),
                vec2(width, height),
            )
        };
        Self {
            surface,
            narrow,
            coarse,
            scope_height: if coarse {
                SCOPE_TOUCH_HEIGHT
            } else {
                SCOPE_HEIGHT
            },
        }
    }

    fn regions(self) -> PaletteRegions {
        let inner = self.surface.shrink(1.0);
        let search = Rect::from_min_size(inner.min, vec2(inner.width(), SEARCH_HEIGHT));
        let scope = Rect::from_min_size(
            pos2(inner.left(), search.bottom()),
            vec2(inner.width(), self.scope_height),
        );
        let footer = Rect::from_min_max(
            pos2(
                inner.left(),
                (inner.bottom() - FOOTER_HEIGHT).max(scope.bottom()),
            ),
            inner.max,
        );
        let results = Rect::from_min_max(
            pos2(inner.left(), scope.bottom()),
            pos2(inner.right(), footer.top().max(scope.bottom())),
        );
        PaletteRegions {
            search,
            scope,
            results,
            footer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PaletteRegions {
    search: Rect,
    scope: Rect,
    results: Rect,
    footer: Rect,
}

/// Search groups from the reviewed command-dialog mockup. Every indexed item
/// has exactly one primary scope; the Signals scope is backed only by actual
/// active-run waveform records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
enum PaletteScope {
    #[default]
    All,
    Navigate,
    Commands,
    Design,
    Simulation,
    Results,
    Verification,
    Models,
    Automation,
    Signals,
    Help,
}

impl PaletteScope {
    const ALL: [Self; 11] = [
        Self::All,
        Self::Navigate,
        Self::Commands,
        Self::Design,
        Self::Simulation,
        Self::Results,
        Self::Verification,
        Self::Models,
        Self::Automation,
        Self::Signals,
        Self::Help,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Navigate => "Navigate",
            Self::Commands => "Commands",
            Self::Design => "Design",
            Self::Simulation => "Simulation",
            Self::Results => "Results",
            Self::Verification => "Verification",
            Self::Models => "Models",
            Self::Automation => "Automation",
            Self::Signals => "Signals",
            Self::Help => "Help",
        }
    }

    fn for_command(command: Command) -> Self {
        if matches!(command, Command::OpenWorkspace(_) | Command::ProjectPage(_)) {
            return Self::Navigate;
        }
        match command.spec().group {
            "Navigate" | "Project" => Self::Navigate,
            "Design" => Self::Design,
            "Simulate" => Self::Simulation,
            "Results" => Self::Results,
            "Verify" => Self::Verification,
            "Models" => Self::Models,
            "Automation" => Self::Automation,
            "Help" => Self::Help,
            // File, Edit, View, and Window are general application commands,
            // matching the mockup's Commands projection.
            _ => Self::Commands,
        }
    }

    #[cfg(test)]
    fn contains(self, command: Command) -> bool {
        self == Self::All || Self::for_command(command) == self
    }

    fn detail(self, registry_group: &'static str) -> &'static str {
        match self {
            Self::All => "Registered application command",
            Self::Navigate => "Open or switch an engineering workspace",
            Self::Commands => match registry_group {
                "File" => "File and project command",
                "Edit" => "Editing command",
                "View" => "Active-view command",
                "Window" => "Workbench window command",
                _ => "Registered application command",
            },
            Self::Design => "Schematic design and editing command",
            Self::Simulation => "Simulation setup or execution command",
            Self::Results => "Results inspection or dataset command",
            Self::Verification => "Verification and design-check command",
            Self::Models => "Model, library, or PDK command",
            Self::Automation => "Automation and scripting command",
            Self::Signals => "Signal in the active result dataset",
            Self::Help => "Product help and information command",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct PaletteFocusState {
    prior_focus: Option<Id>,
    last_seen_pass: u64,
}

#[derive(Debug, Clone, Copy, Default)]
struct PaletteKeys {
    previous: bool,
    next: bool,
    activate: bool,
    close: bool,
    scope_previous: bool,
    scope_next: bool,
}

impl PaletteKeys {
    fn take(ctx: &Context, scope_has_focus: bool) -> Self {
        ctx.input_mut(|input| Self {
            previous: input.consume_key(Modifiers::NONE, Key::ArrowUp),
            next: input.consume_key(Modifiers::NONE, Key::ArrowDown),
            activate: input.consume_key(Modifiers::NONE, Key::Enter),
            close: input.consume_key(Modifiers::NONE, Key::Escape),
            scope_previous: scope_has_focus && input.consume_key(Modifiers::NONE, Key::ArrowLeft),
            scope_next: scope_has_focus && input.consume_key(Modifiers::NONE, Key::ArrowRight),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct CellViewEntry {
    reference: crate::state::CellViewRef,
    view_type: crate::state::ViewType,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModelEntry {
    library: String,
    model: String,
    model_type: String,
    level: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SignalEntry {
    dataset_id: crate::product::DatasetId,
    analysis_id: u64,
    waveform_index: usize,
    name: String,
    run_label: String,
    analysis_label: String,
    visible: bool,
}

/// A palette target always retains the typed identity needed to resolve and
/// execute the production object represented by its row. Resource entries are
/// deliberately view-only for recency; only registry commands have a stable
/// command-history identity.
#[derive(Debug, Clone, PartialEq)]
enum PaletteEntry {
    Command(Command),
    CellView(CellViewEntry),
    Model(ModelEntry),
    Signal(SignalEntry),
}

impl PaletteEntry {
    fn scope(&self) -> PaletteScope {
        match self {
            Self::Command(command) => PaletteScope::for_command(*command),
            Self::CellView(_) => PaletteScope::Design,
            Self::Model(_) => PaletteScope::Models,
            Self::Signal(_) => PaletteScope::Signals,
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Command(command) => Cow::Borrowed(command_name(*command)),
            Self::CellView(entry) => Cow::Owned(format!(
                "Open {} / {}",
                entry.reference.cell, entry.reference.view
            )),
            Self::Model(entry) => Cow::Owned(format!("Browse {} model", entry.model)),
            Self::Signal(entry) => Cow::Borrowed(&entry.name),
        }
    }

    fn detail(&self) -> String {
        match self {
            Self::Command(command) => PaletteScope::for_command(*command)
                .detail(command.spec().group)
                .to_owned(),
            Self::CellView(entry) => {
                let path = entry.reference.display_path();
                if entry.description.trim().is_empty() {
                    format!("{path} · {} cellview", entry.view_type.display_name())
                } else {
                    format!("{path} · {}", entry.description)
                }
            }
            Self::Model(entry) => {
                let classification =
                    format!("{} · {} · {}", entry.library, entry.model_type, entry.level);
                if entry.description.trim().is_empty() {
                    classification
                } else {
                    format!("{classification} · {}", entry.description)
                }
            }
            Self::Signal(entry) => format!(
                "{} / {} · {}",
                entry.run_label,
                entry.analysis_label,
                if entry.visible { "plotted" } else { "hidden" }
            ),
        }
    }

    fn search_metadata(&self) -> String {
        match self {
            Self::Command(command) => {
                let scope = PaletteScope::for_command(*command);
                let spec = command.spec();
                format!(
                    "{} {} {} {}",
                    spec.id,
                    spec.group,
                    scope.label(),
                    scope.detail(spec.group)
                )
            }
            Self::CellView(entry) => format!(
                "{} {} {} {} {} Design cell cellview",
                entry.reference.library,
                entry.reference.cell,
                entry.reference.view,
                entry.view_type.display_name(),
                entry.description
            ),
            Self::Model(entry) => format!(
                "{} {} {} {} {} Models model library",
                entry.library, entry.model, entry.model_type, entry.level, entry.description
            ),
            Self::Signal(entry) => format!(
                "{} {} {} Signals signal waveform {}",
                entry.name,
                entry.run_label,
                entry.analysis_label,
                if entry.visible { "plotted" } else { "hidden" }
            ),
        }
    }

    fn shortcut_label(&self) -> &'static str {
        match self {
            Self::Command(command) => command.shortcut_label(current_command_platform()),
            Self::CellView(_) | Self::Model(_) | Self::Signal(_) => "",
        }
    }

    fn command(&self) -> Option<Command> {
        match self {
            Self::Command(command) => Some(*command),
            Self::CellView(_) | Self::Model(_) | Self::Signal(_) => None,
        }
    }

    fn availability(&self, app: &RSpiceApp) -> CommandAvailability {
        match self {
            Self::Command(command) => command.availability(app),
            Self::CellView(_) | Self::Model(_) | Self::Signal(_) => CommandAvailability::Available,
        }
    }

    fn execute(self, app: &mut RSpiceApp) -> Result<(), String> {
        match self {
            Self::Command(command) => {
                command.execute(app);
                Ok(())
            }
            Self::CellView(entry) => {
                let exists = app
                    .state
                    .library_manager
                    .get_library(&entry.reference.library)
                    .and_then(|library| library.get_cell(&entry.reference.cell))
                    .and_then(|cell| cell.get_view(&entry.reference.view))
                    .is_some_and(|view| view.view_type == entry.view_type);
                if !exists {
                    return Err(format!(
                        "Cellview '{}' is no longer present in the current project",
                        entry.reference.display_path()
                    ));
                }
                app.state.open_workspace_view(entry.reference);
                app.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Design);
                Ok(())
            }
            Self::Model(entry) => {
                let exists = app
                    .state
                    .model_library_manager
                    .get_library(&entry.library)
                    .is_some_and(|library| library.get_model(&entry.model).is_some());
                if !exists {
                    return Err(format!(
                        "Model '{}:{}' is no longer present in the loaded catalog",
                        entry.library, entry.model
                    ));
                }
                app.state
                    .model_library_manager
                    .select_library(&entry.library);
                app.state.model_library_manager.filter_text = entry.model.clone();
                app.state.model_library_manager.filter_type = None;
                app.state.workbench.selected_model = Some(entry.model);
                app.state.workbench.models_page = crate::workbench::state::ModelsPage::Catalog;
                app.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Models);
                Ok(())
            }
            Self::Signal(entry) => {
                let run_index = app
                    .state
                    .simulation
                    .runs
                    .iter()
                    .position(|run| run.dataset_id == entry.dataset_id)
                    .ok_or_else(|| {
                        format!(
                            "Result dataset for signal '{}' is no longer loaded",
                            entry.name
                        )
                    })?;
                let analysis_index = app.state.simulation.runs[run_index]
                    .analyses
                    .iter()
                    .position(|analysis| analysis.id == entry.analysis_id)
                    .ok_or_else(|| {
                        format!("Analysis for signal '{}' is no longer loaded", entry.name)
                    })?;
                let waveform_matches = app.state.simulation.runs[run_index].analyses
                    [analysis_index]
                    .waveforms
                    .get(entry.waveform_index)
                    .is_some_and(|waveform| waveform.name == entry.name);
                if !waveform_matches {
                    return Err(format!(
                        "Signal '{}' is no longer present in the selected analysis",
                        entry.name
                    ));
                }

                app.state.simulation.select_run(run_index);
                app.state.simulation.select_analysis(analysis_index);
                app.state.simulation.runs[run_index].analyses[analysis_index].waveforms
                    [entry.waveform_index]
                    .visible = true;
                if let Some(waveform) = app.state.simulation.waveforms.get_mut(entry.waveform_index)
                {
                    waveform.visible = true;
                }
                app.state.simulation.data_version =
                    app.state.simulation.data_version.wrapping_add(1);
                app.state.ui.results.viewer = crate::workbench::ResultViewer::Waves;
                app.state.ui.results.hidden_strips.remove(&analysis_index);
                app.state.ui.results.maximized_strip = None;
                app.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Results);
                Ok(())
            }
        }
    }
}

struct PaletteRow {
    entry: PaletteEntry,
    marks: Vec<usize>,
    blocked: Option<&'static str>,
}

impl PaletteRow {
    fn detail(&self) -> String {
        self.blocked.map_or_else(
            || self.entry.detail(),
            |reason| format!("Unavailable: {reason}"),
        )
    }

    fn section_label(&self) -> &'static str {
        self.entry.scope().label()
    }
}

fn palette_id() -> Id {
    Id::new(PALETTE_ID)
}

fn search_id() -> Id {
    palette_id().with("search")
}

fn list_id() -> Id {
    palette_id().with("results")
}

fn scope_id(scope: PaletteScope) -> Id {
    palette_id().with(("scope", scope as u8))
}

fn scope_group_id() -> Id {
    palette_id().with("scope-group")
}

fn result_id(index: usize) -> Id {
    palette_id().with(("result", index))
}

/// Everything searchable is a real registry entry. The registry already
/// excludes the palette itself, context-only Cancel, and private shell verbs.
fn palette_commands() -> impl Iterator<Item = Command> {
    command_catalog()
}

fn command_name(command: Command) -> &'static str {
    command.spec().label.trim_end_matches('…')
}

/// How well text matches the query (lower sorts first), with matching display
/// character indices for accent rendering.
fn match_spans(text: &str, query: &str) -> Option<(u8, Vec<usize>)> {
    if query.is_empty() {
        return Some((3, Vec::new()));
    }
    let text_lower: Vec<char> = text.to_lowercase().chars().collect();
    let query_lower: Vec<char> = query.to_lowercase().chars().collect();

    if let Some(at) = find_chars(&text_lower, &query_lower) {
        let rank = if at == 0 {
            0
        } else if text_lower[at - 1].is_whitespace() {
            1
        } else {
            2
        };
        return Some((rank, (at..at + query_lower.len()).collect()));
    }

    let mut marks = Vec::with_capacity(query_lower.len());
    let mut next = 0;
    for (index, ch) in text_lower.iter().enumerate() {
        if next < query_lower.len() && *ch == query_lower[next] {
            marks.push(index);
            next += 1;
        }
    }
    (next == query_lower.len()).then_some((4, marks))
}

fn find_chars(haystack: &[char], needle: &[char]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    (0..=haystack.len() - needle.len()).find(|&at| haystack[at..at + needle.len()] == *needle)
}

fn match_entry(entry: &PaletteEntry, query: &str) -> Option<(u8, Vec<usize>)> {
    let terms = query.split_whitespace().collect::<Vec<_>>();
    if terms.is_empty() {
        return Some((3, Vec::new()));
    }

    let name = entry.name();
    let metadata = entry.search_metadata().to_lowercase();
    let mut rank = 0;
    let mut marks = Vec::new();
    for term in terms {
        if let Some((term_rank, term_marks)) = match_spans(&name, term) {
            rank = rank.max(term_rank);
            marks.extend(term_marks);
        } else if metadata.contains(&term.to_lowercase()) {
            rank = rank.max(3);
        } else {
            return None;
        }
    }
    marks.sort_unstable();
    marks.dedup();
    Some((rank, marks))
}

impl RSpiceApp {
    fn palette_entries(&self) -> Vec<PaletteEntry> {
        let mut entries = palette_commands()
            .map(PaletteEntry::Command)
            .collect::<Vec<_>>();

        let mut design_libraries = self
            .state
            .library_manager
            .libraries_by_key()
            .map(|(_, library)| library)
            .collect::<Vec<_>>();
        design_libraries.sort_by(|left, right| left.name.cmp(&right.name));
        for library in design_libraries {
            for cell in library.cells_sorted() {
                for view in cell.views_sorted() {
                    entries.push(PaletteEntry::CellView(CellViewEntry {
                        reference: crate::state::CellViewRef::new(
                            &library.name,
                            &cell.name,
                            &view.name,
                        ),
                        view_type: view.view_type,
                        description: cell.description.clone(),
                    }));
                }
            }
        }

        for library in self.state.model_library_manager.libraries_sorted() {
            let mut models = library.models.values().collect::<Vec<_>>();
            models.sort_by(|left, right| left.name.cmp(&right.name));
            for model in models {
                entries.push(PaletteEntry::Model(ModelEntry {
                    library: library.name.clone(),
                    model: model.name.clone(),
                    model_type: model.model_type.display_name().to_owned(),
                    level: model.level.display_name().to_owned(),
                    description: model.description.clone(),
                }));
            }
        }

        if let Some(run) = self.state.simulation.active_run() {
            for analysis in &run.analyses {
                for (waveform_index, waveform) in analysis.waveforms.iter().enumerate() {
                    entries.push(PaletteEntry::Signal(SignalEntry {
                        dataset_id: run.dataset_id,
                        analysis_id: analysis.id,
                        waveform_index,
                        name: waveform.name.clone(),
                        run_label: run.label.clone(),
                        analysis_label: analysis.label.clone(),
                        visible: waveform.visible,
                    }));
                }
            }
        }

        entries
    }

    fn palette_rows(&self, query: &str, scope: PaletteScope) -> Vec<PaletteRow> {
        let row = |entry: PaletteEntry, marks: Vec<usize>| {
            let blocked = match entry.availability(self) {
                CommandAvailability::Available => None,
                CommandAvailability::Disabled(reason) => Some(reason),
                // Hidden is a registry instruction, not a disabled state. It
                // must never leak into search as a visible pseudo-command.
                CommandAvailability::Hidden => return None,
            };
            Some(PaletteRow {
                entry,
                marks,
                blocked,
            })
        };
        let entries = self.palette_entries();

        if query.trim().is_empty() {
            if scope != PaletteScope::All {
                return entries
                    .into_iter()
                    .filter(|entry| entry.scope() == scope)
                    .filter_map(|entry| row(entry, Vec::new()))
                    .collect();
            }

            let recents = &self.state.dialogs.command_palette.recent;
            let workspace_scope = match self.state.workbench.workspace {
                crate::workbench::state::Workspace::Project => PaletteScope::Navigate,
                crate::workbench::state::Workspace::Design => PaletteScope::Design,
                crate::workbench::state::Workspace::Simulate => PaletteScope::Simulation,
                crate::workbench::state::Workspace::Results => PaletteScope::Results,
                crate::workbench::state::Workspace::Verify => PaletteScope::Verification,
                crate::workbench::state::Workspace::Models => PaletteScope::Models,
                crate::workbench::state::Workspace::Netlist => PaletteScope::Automation,
            };
            let mut suggestions = entries
                .into_iter()
                .enumerate()
                .filter_map(|(registry_index, entry)| {
                    let recent_index = entry.command().and_then(|command| {
                        recents.iter().position(|candidate| *candidate == command)
                    });
                    let entry_scope = entry.scope();
                    let has_shortcut = !entry.shortcut_label().is_empty();
                    (recent_index.is_some()
                        || entry_scope == workspace_scope
                        || entry_scope == PaletteScope::Navigate
                        || has_shortcut)
                        .then_some((
                            registry_index,
                            recent_index,
                            entry_scope,
                            has_shortcut,
                            entry,
                        ))
                })
                .collect::<Vec<_>>();
            suggestions.sort_by_key(
                |(registry_index, recent_index, entry_scope, has_shortcut, _)| {
                    let context_rank = if *entry_scope == workspace_scope {
                        0
                    } else if *has_shortcut {
                        1
                    } else if *entry_scope == PaletteScope::Navigate {
                        2
                    } else {
                        3
                    };
                    (
                        recent_index.unwrap_or(usize::MAX),
                        context_rank,
                        *registry_index,
                    )
                },
            );
            return suggestions
                .into_iter()
                .take(SUGGESTION_LIMIT)
                .filter_map(|(_, _, _, _, entry)| row(entry, Vec::new()))
                .collect();
        }

        let mut ranked = entries
            .into_iter()
            .filter(|entry| scope == PaletteScope::All || entry.scope() == scope)
            .filter_map(|entry| {
                let (rank, marks) = match_entry(&entry, query)?;
                row(entry, marks).map(|row| (rank, row))
            })
            .collect::<Vec<_>>();
        // Stable rank preserves the command registry and resource catalogs'
        // deterministic order for equal-quality matches.
        ranked.sort_by_key(|(rank, _)| *rank);
        ranked.into_iter().map(|(_, row)| row).collect()
    }

    pub(super) fn render_command_palette(&mut self, ctx: &Context) {
        if !self.state.dialogs.command_palette.open {
            return;
        }

        let viewport = ctx.content_rect();
        let layout = PaletteLayout::resolve(viewport, ctx.input(|input| input.has_touch_screen()));
        let regions = layout.regions();
        let t = Tokens::get(ctx);
        let c = t.color;
        let area_id = palette_id();
        let focus_state_id = area_id.with("focus-state");
        let scope_state_id = area_id.with("scope-state");
        let opened_this_pass = begin_focus_session(ctx, focus_state_id);

        let mut scope = ctx
            .data(|data| data.get_temp::<PaletteScope>(scope_state_id))
            .unwrap_or_default();
        if opened_this_pass {
            scope = PaletteScope::All;
        }

        let focused_before = ctx.memory(|memory| memory.focused());
        let focused_scope = PaletteScope::ALL
            .iter()
            .position(|candidate| focused_before == Some(scope_id(*candidate)));
        let keys = PaletteKeys::take(ctx, focused_scope.is_some());
        if let Some(index) = focused_scope {
            let next = if keys.scope_previous {
                index.checked_sub(1).unwrap_or(PaletteScope::ALL.len() - 1)
            } else if keys.scope_next {
                (index + 1) % PaletteScope::ALL.len()
            } else {
                index
            };
            if next != index {
                scope = PaletteScope::ALL[next];
                self.state.dialogs.command_palette.selected = 0;
                ctx.memory_mut(|memory| memory.request_focus(scope_id(scope)));
            } else if keys.activate {
                scope = PaletteScope::ALL[index];
                self.state.dialogs.command_palette.selected = 0;
            }
        }

        let area = egui::Area::new(area_id)
            .kind(UiKind::Modal)
            .order(Order::Foreground)
            .fixed_pos(viewport.min)
            .sense(Sense::focusable_noninteractive());
        let modal_layer = area.layer();
        ctx.memory_mut(|memory| memory.set_modal_layer(modal_layer));
        if opened_this_pass || !focus_is_within_modal(ctx, modal_layer) {
            ctx.memory_mut(|memory| {
                memory.request_focus(search_id());
                if opened_this_pass {
                    memory.move_focus(FocusDirection::None);
                }
            });
        }

        let mut run = None;
        let mut activate_handled = focused_scope.is_some() && keys.activate;
        let mut search_response_id = None;
        let mut active_result_id = None;
        let local_index_count = self
            .palette_entries()
            .into_iter()
            .filter(|entry| entry.availability(self) != CommandAvailability::Hidden)
            .count();
        let area_response = area.show(ctx, |ui| {
            // A pointer-only scrim blocks the workbench without polluting the
            // keyboard or assistive-technology focus order.
            ui.allocate_rect(
                viewport,
                Sense::click_and_drag().difference(Sense::focusable_noninteractive()),
            );
            let backdrop = if t.mode == Mode::Dark {
                egui::Color32::from_rgba_unmultiplied(2, 6, 8, 158)
            } else {
                egui::Color32::from_rgba_unmultiplied(41, 46, 50, 97)
            };
            ui.painter().rect_filled(viewport, 0.0, backdrop);
            ui.painter().add(t.shadow().as_shape(layout.surface, 8.0));
            ui.painter().rect(
                layout.surface,
                8.0,
                c.bg_elevated,
                Stroke::new(1.0, c.border_strong),
                egui::StrokeKind::Inside,
            );

            let mut surface = ui.new_child(
                UiBuilder::new()
                    .max_rect(layout.surface.shrink(1.0))
                    .layout(egui::Layout::top_down(Align::Min)),
            );
            surface.set_clip_rect(layout.surface.shrink(1.0));

            let search_response = render_search_row(
                &mut surface,
                regions.search,
                &mut self.state.dialogs.command_palette.query,
            );
            search_response_id = Some(search_response.id);
            if search_response.changed() {
                self.state.dialogs.command_palette.selected = 0;
            }
            if self.state.dialogs.command_palette.want_focus || opened_this_pass {
                search_response.request_focus();
                self.state.dialogs.command_palette.want_focus = false;
            }

            let scope_changed = render_scope_row(
                &mut surface,
                regions.scope,
                layout.coarse,
                &mut scope,
                focused_before,
                keys.activate,
            );
            if scope_changed {
                self.state.dialogs.command_palette.selected = 0;
            }

            let rows = self.palette_rows(&self.state.dialogs.command_palette.query, scope);
            let selected = &mut self.state.dialogs.command_palette.selected;
            if rows.is_empty() {
                *selected = 0;
            } else {
                *selected = (*selected).min(rows.len() - 1);
                if keys.next {
                    *selected = (*selected + 1) % rows.len();
                }
                if keys.previous {
                    *selected = selected.checked_sub(1).unwrap_or(rows.len() - 1);
                }
                active_result_id = Some(result_id(*selected));
            }

            let focused_result = rows
                .iter()
                .enumerate()
                .find(|(index, _)| focused_before == Some(result_id(*index)))
                .map(|(index, row)| (index, row.entry.clone(), row.blocked.is_none()));
            if keys.activate
                && focused_scope.is_none()
                && let Some((index, entry, runnable)) = focused_result
            {
                activate_handled = true;
                *selected = index;
                if runnable {
                    run = Some(entry);
                }
            }

            let clicked = render_results(
                &mut surface,
                regions.results,
                &rows,
                *selected,
                keys.previous || keys.next,
            );
            if clicked.is_some() {
                run = clicked;
            }

            if keys.activate && !activate_handled && run.is_none() {
                run = rows
                    .get(*selected)
                    .filter(|row| row.blocked.is_none())
                    .map(|row| row.entry.clone());
            }

            render_footer(
                &mut surface,
                regions.footer,
                layout.narrow,
                rows.len(),
                local_index_count,
                self.state.dialogs.command_palette.query.trim().is_empty()
                    && scope == PaletteScope::All,
                self.state.workbench.engineering_profile.label(),
            );
        });

        area_response
            .response
            .widget_info(|| WidgetInfo::labeled(WidgetType::Window, true, DIALOG_LABEL));
        ctx.accesskit_node_builder(area_response.response.id, |node| {
            node.set_role(egui::accesskit::Role::Dialog);
            node.set_label(DIALOG_LABEL);
            node.set_description(DIALOG_DESCRIPTION);
            node.set_modal();
        });

        if let Some(search_response_id) = search_response_id {
            ctx.accesskit_node_builder(search_response_id, |node| {
                node.set_role(egui::accesskit::Role::ComboBox);
                node.set_label("Command search");
                node.set_description(DIALOG_DESCRIPTION);
                node.set_expanded(true);
                node.set_controls([list_id().accesskit_id()]);
                if let Some(active_result_id) = active_result_id {
                    node.set_active_descendant(active_result_id.accesskit_id());
                } else {
                    node.clear_active_descendant();
                }
            });
        }

        ctx.data_mut(|data| data.insert_temp(scope_state_id, scope));

        let close = keys.close || run.is_some();
        if close {
            self.state.dialogs.command_palette.open = false;
            ctx.data_mut(|data| {
                data.remove_temp::<PaletteScope>(scope_state_id);
            });
            restore_focus(ctx, focus_state_id, search_id(), modal_layer);
        }

        if let Some(entry) = run {
            if let Some(command) = entry.command() {
                let recent = &mut self.state.dialogs.command_palette.recent;
                recent.retain(|candidate| *candidate != command);
                recent.insert(0, command);
                recent.truncate(MAX_RECENT);
            }
            if let Err(error) = entry.execute(self) {
                self.state
                    .push_user_message(crate::common::app::ConsoleMessage::warning(error));
            }
        }
    }
}

fn render_search_row(surface: &mut Ui, rect: Rect, query: &mut String) -> egui::Response {
    let t = Tokens::get(surface.ctx());
    let c = t.color;
    surface.painter().rect_filled(rect, 0.0, c.bg_inset);
    surface
        .painter()
        .hline(rect.x_range(), rect.bottom(), Stroke::new(1.0, c.border));

    let search_icon =
        Rect::from_center_size(pos2(rect.left() + 20.0, rect.center().y), vec2(16.0, 16.0));
    paint_search_icon(surface, search_icon, c.text_dim);

    let escape_width = kbd_width(surface, "Esc", 18.0);
    let escape_rect = Rect::from_center_size(
        pos2(rect.right() - 12.0 - escape_width * 0.5, rect.center().y),
        vec2(escape_width, 18.0),
    );
    paint_kbd(surface, escape_rect, "Esc", 11.0);

    let input_rect = Rect::from_min_max(
        pos2(search_icon.right() + 9.0, rect.top() + 1.0),
        pos2(escape_rect.left() - 9.0, rect.bottom() - 1.0),
    );
    let mut input_ui = surface.new_child(
        UiBuilder::new()
            .max_rect(input_rect)
            .layout(egui::Layout::left_to_right(Align::Center)),
    );
    let response = input_ui.add_sized(
        input_rect.size(),
        egui::TextEdit::singleline(query)
            .id(search_id())
            .font(theme::sans(tokens::FS_2, FontWeight::Regular))
            .hint_text("Search commands, cells, signals, models…")
            .frame(egui::Frame::NONE)
            .margin(Margin::same(0))
            .vertical_align(Align::Center),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::ComboBox, true, "Command search"));
    theme::paint_focus_ring(surface, &response, input_rect);
    response
}

fn render_scope_row(
    surface: &mut Ui,
    rect: Rect,
    coarse: bool,
    selected: &mut PaletteScope,
    focused_before: Option<Id>,
    activate: bool,
) -> bool {
    let t = Tokens::get(surface.ctx());
    let c = t.color;
    surface.painter().rect_filled(rect, 0.0, c.bg_elevated);
    surface
        .painter()
        .hline(rect.x_range(), rect.bottom(), Stroke::new(1.0, c.border));
    let group_response = surface.interact(rect, scope_group_id(), Sense::hover());
    group_response
        .widget_info(|| WidgetInfo::labeled(WidgetType::RadioGroup, true, "Search scopes"));
    surface
        .ctx()
        .accesskit_node_builder(group_response.id, |node| {
            node.set_role(egui::accesskit::Role::RadioGroup);
            node.set_label("Search scopes");
        });

    let chip_height = if coarse {
        TOUCH_TARGET
    } else {
        SCOPE_CHIP_HEIGHT
    };
    let inner = Rect::from_min_max(
        pos2(rect.left() + 10.0, rect.top() + 7.0),
        pos2(
            rect.right() - 10.0,
            (rect.top() + 7.0 + chip_height).min(rect.bottom()),
        ),
    );
    let mut host = surface.new_child(
        UiBuilder::new()
            .max_rect(inner)
            .layout(egui::Layout::left_to_right(Align::Center))
            .accessibility_parent(scope_group_id()),
    );
    let mut changed = false;
    egui::ScrollArea::horizontal()
        .id_salt(palette_id().with("scope-scroll"))
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .auto_shrink([false, false])
        .show(&mut host, |ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 5.0;
                for scope in PaletteScope::ALL {
                    let label = scope.label();
                    let text_width = ui.fonts_mut(|fonts| {
                        fonts
                            .layout_no_wrap(
                                label.to_owned(),
                                theme::sans(tokens::FS_0, FontWeight::Regular),
                                c.text,
                            )
                            .size()
                            .x
                    });
                    let width = (text_width + 16.0).ceil();
                    let (_, chip_rect) = ui.allocate_space(vec2(width, chip_height));
                    let response = ui.interact(chip_rect, scope_id(scope), Sense::click());
                    let active = *selected == scope;
                    response.widget_info(|| {
                        WidgetInfo::selected(WidgetType::RadioButton, true, active, label)
                    });
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        node.set_role(egui::accesskit::Role::RadioButton);
                        node.set_label(label);
                        node.set_selected(active);
                    });
                    let fill = if active {
                        c.bg_active
                    } else if response.hovered() {
                        c.bg_hover
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    if fill != egui::Color32::TRANSPARENT {
                        ui.painter().rect_filled(chip_rect, 2.0, fill);
                    }
                    if active {
                        ui.painter().rect_stroke(
                            chip_rect,
                            2.0,
                            Stroke::new(1.0, c.border_strong),
                            egui::StrokeKind::Inside,
                        );
                        ui.painter().hline(
                            (chip_rect.left() + 1.0)..=(chip_rect.right() - 1.0),
                            chip_rect.bottom() - 1.0,
                            Stroke::new(2.0, c.accent),
                        );
                    }
                    ui.painter().text(
                        chip_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        label,
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        if active { c.text } else { c.text_dim },
                    );
                    theme::paint_focus_ring(ui, &response, chip_rect);

                    if response.clicked() || (activate && focused_before == Some(scope_id(scope))) {
                        changed |= *selected != scope;
                        *selected = scope;
                        if response.clicked() {
                            ui.memory_mut(|memory| memory.request_focus(search_id()));
                        }
                    }
                }
            });
        });
    changed
}

fn render_results(
    surface: &mut Ui,
    rect: Rect,
    rows: &[PaletteRow],
    selected: usize,
    keyboard_moved: bool,
) -> Option<PaletteEntry> {
    let c = Tokens::get(surface.ctx()).color;
    surface.painter().rect_filled(rect, 0.0, c.bg_elevated);
    let list_response = surface.interact(rect, list_id(), Sense::hover());
    list_response.widget_info(|| WidgetInfo::labeled(WidgetType::Other, true, "Matching commands"));
    surface
        .ctx()
        .accesskit_node_builder(list_response.id, |node| {
            node.set_role(egui::accesskit::Role::ListBox);
            node.set_label("Matching commands");
        });

    let mut list_ui = surface.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(Align::Min))
            .accessibility_parent(list_id()),
    );
    list_ui.spacing_mut().item_spacing.y = 0.0;
    let mut run = None;
    egui::ScrollArea::vertical()
        .id_salt(palette_id().with("result-scroll"))
        .auto_shrink([false, false])
        .show(&mut list_ui, |ui| {
            ui.set_min_width(rect.width());
            ui.add_space(5.0);
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                let available = (ui.available_width() - 10.0).max(1.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.add_space(5.0);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.set_width(available);
                        if rows.is_empty() {
                            render_empty_results(ui, available);
                        } else {
                            let mut previous_section = None;
                            for (index, row) in rows.iter().enumerate() {
                                let section = row.section_label();
                                if previous_section != Some(section) {
                                    render_group_label(ui, section, available);
                                    previous_section = Some(section);
                                }
                                let response =
                                    render_result_row(ui, row, index, index == selected, available);
                                if index == selected && keyboard_moved {
                                    response.scroll_to_me(Some(Align::Center));
                                }
                                if row.blocked.is_none() && response.clicked() {
                                    run = Some(row.entry.clone());
                                }
                            }
                        }
                    });
                    ui.add_space(5.0);
                });
            });
            ui.add_space(5.0);
        });
    run
}

fn render_empty_results(ui: &mut Ui, width: f32) {
    let c = Tokens::get(ui.ctx()).color;
    let (rect, response) = ui.allocate_exact_size(vec2(width, RESULT_ROW_HEIGHT), Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            true,
            "No matching command, cellview, model, or signal",
        )
    });
    ui.painter().text(
        pos2(rect.left() + 8.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "No matching command, cellview, model, or signal.",
        theme::sans(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );
}

fn render_group_label(ui: &mut Ui, label: &str, width: f32) {
    let c = Tokens::get(ui.ctx()).color;
    let (rect, response) = ui.allocate_exact_size(vec2(width, RESULT_GROUP_HEIGHT), Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, label));
    let label = label.to_uppercase();
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &label,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: c.text_faint,
            extra_letter_spacing: 0.08 * tokens::FS_0,
            ..Default::default()
        },
    );
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    ui.painter().galley(
        pos2(rect.left() + 8.0, rect.center().y - galley.size().y * 0.5),
        galley,
        c.text_faint,
    );
}

fn render_result_row(
    ui: &mut Ui,
    row: &PaletteRow,
    index: usize,
    selected: bool,
    width: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (_, rect) = ui.allocate_space(vec2(width, RESULT_ROW_HEIGHT));
    let response = ui.interact(rect, result_id(index), Sense::click());
    let name = row.entry.name();
    let detail = row.detail();
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            row.blocked.is_none(),
            selected,
            name.as_ref(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::ListBoxOption);
        node.set_label(name.as_ref());
        node.set_description(detail.clone());
        node.set_selected(selected);
        if row.blocked.is_some() {
            node.set_disabled();
        }
        let shortcut = row.entry.shortcut_label();
        if !shortcut.is_empty() && row.blocked.is_none() {
            node.set_keyboard_shortcut(shortcut);
        }
    });

    if selected || response.hovered() {
        ui.painter().rect_filled(rect, 4.0, c.bg_hover);
    }
    if selected {
        ui.painter().rect_filled(
            Rect::from_min_size(rect.min, vec2(2.0, rect.height())),
            0.0,
            c.accent,
        );
    }

    let icon_rect = Rect::from_center_size(
        pos2(
            rect.left() + 8.0 + RESULT_ICON_COLUMN * 0.5,
            rect.center().y,
        ),
        vec2(RESULT_ICON_SIDE, RESULT_ICON_SIDE),
    );
    ui.painter().rect(
        icon_rect,
        3.0,
        c.bg_inset,
        Stroke::new(1.0, c.border),
        egui::StrokeKind::Inside,
    );
    row_icon(&row.entry).paint(
        ui.painter(),
        icon_rect.shrink(5.0),
        if row.blocked.is_some() {
            c.text_faint
        } else {
            c.accent
        },
    );

    let trailing = if row.blocked.is_some() {
        "Unavailable"
    } else {
        row.entry.shortcut_label()
    };
    let trailing_width = if trailing.is_empty() {
        0.0
    } else if row.blocked.is_some() {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(
                    trailing.to_owned(),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                )
                .size()
                .x
        })
    } else {
        kbd_width(ui, trailing, 18.0)
    };
    let trailing_rect = Rect::from_center_size(
        pos2(rect.right() - 8.0 - trailing_width * 0.5, rect.center().y),
        vec2(trailing_width, 18.0),
    );
    if !trailing.is_empty() {
        if row.blocked.is_some() {
            ui.painter().text(
                trailing_rect.center(),
                egui::Align2::CENTER_CENTER,
                trailing,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_faint,
            );
        } else {
            paint_kbd(ui, trailing_rect, trailing, tokens::FS_0);
        }
    }

    let body_left = rect.left() + 8.0 + RESULT_ICON_COLUMN + 9.0;
    let body_right = if trailing.is_empty() {
        rect.right() - 8.0
    } else {
        trailing_rect.left() - 9.0
    };
    let body_rect = Rect::from_min_max(
        pos2(body_left, rect.top() + 4.0),
        pos2(body_right.max(body_left), rect.bottom() - 4.0),
    );
    let painter = ui.painter().with_clip_rect(body_rect);
    let base_color = if row.blocked.is_some() {
        c.text_faint
    } else {
        c.text
    };
    let name_galley = matched_name_galley(ui, &name, &row.marks, base_color, c.accent);
    painter.galley(body_rect.min, name_galley, base_color);
    painter.text(
        pos2(body_rect.left(), body_rect.top() + 15.0),
        egui::Align2::LEFT_TOP,
        detail,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );

    theme::paint_focus_ring(ui, &response, rect);
    if row.blocked.is_none() {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response.on_hover_cursor(egui::CursorIcon::NotAllowed)
    }
}

fn matched_name_galley(
    ui: &mut Ui,
    name: &str,
    marks: &[usize],
    base_color: egui::Color32,
    accent: egui::Color32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::default();
    let mut next_mark = 0;
    for (index, ch) in name.chars().enumerate() {
        let marked = next_mark < marks.len() && marks[next_mark] == index;
        if marked {
            next_mark += 1;
        }
        let mut buffer = [0_u8; 4];
        job.append(
            ch.encode_utf8(&mut buffer),
            0.0,
            egui::TextFormat {
                font_id: theme::sans(
                    tokens::FS_0,
                    if marked {
                        FontWeight::Medium
                    } else {
                        FontWeight::Regular
                    },
                ),
                color: if marked { accent } else { base_color },
                ..Default::default()
            },
        );
    }
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

fn row_icon(entry: &PaletteEntry) -> Icon {
    let PaletteEntry::Command(command) = entry else {
        return match entry {
            PaletteEntry::CellView(_) => Icon::Schematic,
            PaletteEntry::Model(_) => Icon::Library,
            PaletteEntry::Signal(_) => Icon::Results,
            PaletteEntry::Command(_) => unreachable!("command handled above"),
        };
    };
    match command {
        Command::PlaceWire => Icon::Wire,
        Command::PlaceLabel => Icon::NetLabel,
        Command::PlaceProbe => Icon::Probe,
        Command::RunSimulation => Icon::Run,
        Command::StopSimulation => Icon::Stop,
        Command::RunChecks | Command::CheckAndSave | Command::PreflightChecks => Icon::Check,
        Command::OpenProject
        | Command::ProjectLauncher
        | Command::RecentProjects
        | Command::OpenDocument
        | Command::ModelBrowser => Icon::Folder,
        Command::NewProject
        | Command::Save
        | Command::SaveAs
        | Command::SaveAll
        | Command::ImportNetlist
        | Command::ImportVerilogA
        | Command::ExportSchematicSvg
        | Command::ExportWaveformsCsv
        | Command::ExportNetlist(_) => Icon::File,
        _ => match PaletteScope::for_command(*command) {
            PaletteScope::Navigate => Icon::Folder,
            PaletteScope::Design => Icon::Schematic,
            PaletteScope::Simulation => Icon::Simulate,
            PaletteScope::Results => Icon::Results,
            PaletteScope::Verification => Icon::Check,
            PaletteScope::Models => Icon::Library,
            PaletteScope::Signals => Icon::Probe,
            PaletteScope::Help => Icon::Brand,
            PaletteScope::All | PaletteScope::Commands | PaletteScope::Automation => Icon::File,
        },
    }
}

fn render_footer(
    surface: &mut Ui,
    rect: Rect,
    narrow: bool,
    shown: usize,
    local_index_count: usize,
    suggestions: bool,
    engineering_profile: &str,
) {
    let c = Tokens::get(surface.ctx()).color;
    surface.painter().rect_filled(rect, 0.0, c.bg_panel);
    surface
        .painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, c.border));
    let provider = format!(
        "{shown} {} · {local_index_count} searchable in {engineering_profile}",
        if suggestions { "suggested" } else { "shown" }
    );
    let provider_galley = surface.fonts_mut(|fonts| {
        fonts.layout_no_wrap(
            provider,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        )
    });

    if narrow {
        surface.painter().galley(
            rect.center() - provider_galley.size() * 0.5,
            provider_galley,
            c.text_faint,
        );
        return;
    }

    let provider_pos = pos2(
        rect.right() - 10.0 - provider_galley.size().x,
        rect.center().y - provider_galley.size().y * 0.5,
    );
    surface
        .painter()
        .galley(provider_pos, provider_galley, c.text_faint);

    let mut x = rect.left() + 10.0;
    x = paint_footer_hint(surface, rect, x, &["↑", "↓"], "navigate");
    x += 13.0;
    x = paint_footer_hint(surface, rect, x, &["Enter"], "open");
    x += 13.0;
    let _ = paint_footer_hint(surface, rect, x, &["Tab"], "actions");
}

fn paint_footer_hint(ui: &mut Ui, rect: Rect, mut x: f32, keys: &[&str], label: &str) -> f32 {
    let c = Tokens::get(ui.ctx()).color;
    for key in keys {
        let width = kbd_width(ui, key, 16.0);
        let key_rect =
            Rect::from_center_size(pos2(x + width * 0.5, rect.center().y), vec2(width, 16.0));
        paint_kbd(ui, key_rect, key, tokens::FS_0);
        x += width + 2.0;
    }
    let galley = ui.fonts_mut(|fonts| {
        fonts.layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        )
    });
    ui.painter().galley(
        pos2(x, rect.center().y - galley.size().y * 0.5),
        galley.clone(),
        c.text_faint,
    );
    x + galley.size().x
}

fn kbd_width(ui: &mut Ui, text: &str, height: f32) -> f32 {
    let c = Tokens::get(ui.ctx()).color;
    let text_width = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                text.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_faint,
            )
            .size()
            .x
    });
    (text_width + if height <= 16.0 { 6.0 } else { 8.0 }).max(if height <= 16.0 {
        18.0
    } else {
        19.0
    })
}

fn paint_kbd(ui: &mut Ui, rect: Rect, text: &str, font_size: f32) {
    let c = Tokens::get(ui.ctx()).color;
    ui.painter().rect(
        rect,
        3.0,
        c.bg_panel_2,
        Stroke::new(1.0, c.border),
        egui::StrokeKind::Inside,
    );
    ui.painter().hline(
        (rect.left() + 1.0)..=(rect.right() - 1.0),
        rect.bottom() - 1.0,
        Stroke::new(1.0, c.border_strong),
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        theme::mono(font_size, FontWeight::Regular),
        c.text_faint,
    );
}

fn paint_search_icon(ui: &Ui, rect: Rect, color: egui::Color32) {
    let center = pos2(rect.left() + 6.5, rect.top() + 6.5);
    ui.painter()
        .circle_stroke(center, 4.5, Stroke::new(1.4, color));
    ui.painter().line_segment(
        [
            center + vec2(3.2, 3.2),
            rect.right_bottom() - vec2(1.0, 1.0),
        ],
        Stroke::new(1.4, color),
    );
}

fn begin_focus_session(ctx: &Context, state_id: Id) -> bool {
    let pass = ctx.cumulative_pass_nr();
    let current_focus = ctx.memory(|memory| memory.focused());
    ctx.data_mut(|data| {
        let previous = data.get_temp::<PaletteFocusState>(state_id);
        let continuing =
            previous.is_some_and(|state| pass <= state.last_seen_pass.saturating_add(1));
        let state = match previous {
            Some(mut state) if continuing => {
                state.last_seen_pass = pass;
                state
            }
            _ => PaletteFocusState {
                prior_focus: current_focus,
                last_seen_pass: pass,
            },
        };
        data.insert_temp(state_id, state);
        !continuing
    })
}

fn focus_is_within_modal(ctx: &Context, modal_layer: egui::LayerId) -> bool {
    let Some(focused) = ctx.memory(|memory| memory.focused()) else {
        return false;
    };
    let Some(response) = ctx.read_response(focused) else {
        return false;
    };
    response.layer_id == modal_layer
        || ctx.memory(|memory| memory.is_above_modal_layer(response.layer_id))
}

fn restore_focus(ctx: &Context, state_id: Id, search: Id, modal_layer: egui::LayerId) {
    let state = ctx.data_mut(|data| data.remove_temp::<PaletteFocusState>(state_id));
    let prior = state.and_then(|state| state.prior_focus).filter(|prior| {
        *prior != search
            && ctx.read_response(*prior).is_some_and(|response| {
                response.layer_id != modal_layer
                    && response.enabled()
                    && response.sense.is_focusable()
            })
    });
    ctx.memory_mut(|memory| {
        if let Some(current) = memory.focused() {
            memory.surrender_focus(current);
        }
        if let Some(prior) = prior {
            memory.request_focus(prior);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::commands::COMMAND_REGISTRY;

    #[test]
    fn desktop_and_phone_layouts_match_the_mockup_contract() {
        let desktop = PaletteLayout::resolve(
            Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0)),
            false,
        );
        assert_eq!(desktop.surface.width(), 650.0);
        assert_eq!(desktop.surface.height(), 560.0);
        assert_eq!(desktop.surface.left(), 395.0);
        assert_eq!(desktop.surface.top(), 72.0);
        assert_eq!(desktop.regions().search.height(), 48.0);
        assert_eq!(desktop.regions().scope.height(), 37.0);
        assert_eq!(desktop.regions().footer.height(), 31.0);
        assert_eq!(RESULT_ROW_HEIGHT, 40.0);
        assert_eq!(RESULT_ICON_COLUMN, 28.0);
        assert_eq!(RESULT_ICON_SIDE, 26.0);

        let phone = PaletteLayout::resolve(
            Rect::from_min_size(pos2(0.0, 0.0), vec2(390.0, 844.0)),
            true,
        );
        assert!(phone.narrow);
        assert_eq!(
            phone.surface,
            Rect::from_min_max(pos2(6.0, 6.0), pos2(384.0, 838.0))
        );
        assert_eq!(phone.scope_height, 59.0);

        let touch_desktop = PaletteLayout::resolve(
            Rect::from_min_size(pos2(0.0, 0.0), vec2(1024.0, 768.0)),
            true,
        );
        assert!(!touch_desktop.narrow);
        assert_eq!(touch_desktop.scope_height, 59.0);
    }

    #[test]
    fn palette_is_a_typed_registry_projection() {
        let commands = palette_commands().collect::<Vec<_>>();
        assert!(!commands.contains(&Command::CommandPalette));
        assert!(!commands.contains(&Command::Cancel));
        assert!(commands.contains(&Command::OpenProject));
        assert!(commands.contains(&Command::GenerateNetlist));
        assert!(
            commands
                .iter()
                .all(|command| COMMAND_REGISTRY.contains(command))
        );
        assert!(commands.iter().all(|command| command.palette_visible()));
    }

    #[test]
    fn registered_command_projection_is_total_without_signal_substitutes() {
        let commands = palette_commands().collect::<Vec<_>>();
        assert!(
            commands
                .iter()
                .copied()
                .all(|command| { PaletteScope::for_command(command) != PaletteScope::All })
        );
        let assigned = PaletteScope::ALL
            .into_iter()
            .skip(1)
            .map(|scope| {
                commands
                    .iter()
                    .copied()
                    .filter(|command| scope.contains(*command))
                    .count()
            })
            .sum::<usize>();
        assert_eq!(assigned, commands.len());
        assert!(
            commands
                .iter()
                .copied()
                .all(|command| PaletteScope::for_command(command) != PaletteScope::Signals),
            "signal rows must represent real result waveforms, not generic commands"
        );
    }

    #[test]
    fn command_scopes_and_metadata_search_only_return_registered_commands() {
        let design = palette_commands()
            .filter(|command| PaletteScope::Design.contains(*command))
            .filter(|command| match_entry(&PaletteEntry::Command(*command), "wire").is_some())
            .collect::<Vec<_>>();
        assert_eq!(design, vec![Command::PlaceWire]);

        assert_eq!(
            PaletteScope::for_command(Command::PlaceProbe),
            PaletteScope::Design
        );
        assert_eq!(
            PaletteScope::for_command(Command::ExportWaveformsCsv),
            PaletteScope::Commands
        );

        let help = palette_commands()
            .filter(|command| PaletteScope::Help.contains(*command))
            .collect::<Vec<_>>();
        assert!(help.contains(&Command::KeyboardShortcuts));
        assert!(help.contains(&Command::About));
    }

    #[test]
    fn fuzzy_matching_marks_display_characters_and_searches_registry_metadata() {
        assert_eq!(
            match_spans("Open generated netlist", "ogn"),
            Some((4, vec![0, 5, 7]))
        );
        let command = PaletteEntry::Command(Command::GenerateNetlist);
        let (_, marks) = match_entry(&command, "generated netlist").expect("label terms match");
        assert!(!marks.is_empty());
        assert!(match_entry(&command, "simulate").is_some());
        assert!(match_entry(&command, "nonexistent-token").is_none());
    }

    #[test]
    fn palette_names_are_canonical_labels_without_ellipsis() {
        assert_eq!(command_name(Command::OpenProject), "Open project");
        assert_eq!(command_name(Command::SaveAll), "Save all");
        assert_eq!(
            command_name(Command::GenerateNetlist),
            "Open generated netlist"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn test_app() -> RSpiceApp {
        RSpiceApp {
            state: crate::common::app::AppState::default(),
            first_frame: false,
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::common::export_workflow::NativeExportWorkflowIo),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn raw_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1280.0, 800.0))),
            events,
            ..Default::default()
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn key_event(key: Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn empty_all_scope_uses_mockup_suggestions_and_explicit_scopes_remain_complete() {
        let mut app = test_app();
        app.state.dialogs.command_palette.recent = vec![Command::PlaceProbe, Command::Save];
        let all = app.palette_rows("", PaletteScope::All);
        assert_eq!(all[0].entry.command(), Some(Command::PlaceProbe));
        assert_eq!(all[1].entry.command(), Some(Command::Save));
        assert_eq!(all.len(), SUGGESTION_LIMIT);
        assert_eq!(
            all.iter()
                .filter(|row| row.entry.command() == Some(Command::Save))
                .count(),
            1
        );
        assert!(all.iter().all(|row| row.section_label() != "Recent"));

        let signals = app.palette_rows("", PaletteScope::Signals);
        assert!(
            signals.is_empty(),
            "no result data means no invented signal rows"
        );
        assert_eq!(MAX_RECENT, 8);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn real_cellview_records_are_searchable_and_open_the_workspace_document() {
        let mut app = test_app();
        let mut library = crate::state::Library::new("precision_analog");
        let mut cell = crate::state::Cell::new("gain_stage");
        cell.description = "Precision front-end amplifier".to_owned();
        cell.add_view(crate::state::View::new(
            "schematic",
            crate::state::ViewType::Schematic,
        ));
        library.add_cell(cell);
        app.state.library_manager.add_library(library);

        let mut rows = app.palette_rows("precision gain_stage", PaletteScope::Design);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].detail().contains("Precision front-end amplifier"));
        let entry = rows.remove(0).entry;
        assert!(matches!(entry, PaletteEntry::CellView(_)));
        entry.execute(&mut app).expect("open real cellview");

        assert_eq!(
            app.state.workspace.active_view,
            crate::state::CellViewRef::new("precision_analog", "gain_stage", "schematic")
        );
        assert_eq!(
            app.state.library_manager.selected_lcv_path().as_deref(),
            Some("precision_analog/gain_stage/schematic")
        );
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Design
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn loaded_model_records_are_searchable_and_select_the_catalog_record() {
        let mut app = test_app();
        let mut library = crate::state::model_library::ModelLibrary::new("foundry_65lp");
        let mut model = crate::state::model_library::DeviceModel::new(
            "nmos_ulvt_precision",
            crate::state::model_library::ModelType::Nmos,
        );
        model.description = "Low-noise qualified input device".to_owned();
        library.add_model(model);
        app.state.model_library_manager.add_library(library);
        app.state.model_library_manager.filter_type =
            Some(crate::state::model_library::ModelType::Pmos);

        let mut rows = app.palette_rows("qualified foundry_65lp", PaletteScope::Models);
        assert_eq!(rows.len(), 1);
        assert!(
            rows[0]
                .detail()
                .contains("Low-noise qualified input device")
        );
        let entry = rows.remove(0).entry;
        assert!(matches!(entry, PaletteEntry::Model(_)));
        entry.execute(&mut app).expect("open real model record");

        assert_eq!(
            app.state.model_library_manager.selected_library.as_deref(),
            Some("foundry_65lp")
        );
        assert_eq!(
            app.state.workbench.selected_model.as_deref(),
            Some("nmos_ulvt_precision")
        );
        assert_eq!(
            app.state.model_library_manager.filter_text,
            "nmos_ulvt_precision"
        );
        assert_eq!(app.state.model_library_manager.filter_type, None);
        assert_eq!(
            app.state.workbench.models_page,
            crate::workbench::state::ModelsPage::Catalog
        );
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Models
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn enter_executes_a_dynamic_row_without_polluting_command_recents() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = test_app();
        let mut library = crate::state::model_library::ModelLibrary::new("vendor_models");
        library.add_model(crate::state::model_library::DeviceModel::new(
            "opa189_a",
            crate::state::model_library::ModelType::Other,
        ));
        app.state.model_library_manager.add_library(library);
        app.state.dialogs.command_palette.open();
        app.state.dialogs.command_palette.query = "opa189_a".to_owned();

        let _ = ctx.run(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
        let _ = ctx.run(raw_input(vec![key_event(Key::Enter)]), |ctx| {
            app.render_command_palette(ctx);
        });

        assert!(!app.state.dialogs.command_palette.open);
        assert_eq!(
            app.state.model_library_manager.selected_library.as_deref(),
            Some("vendor_models")
        );
        assert_eq!(
            app.state.workbench.selected_model.as_deref(),
            Some("opa189_a")
        );
        assert!(app.state.dialogs.command_palette.recent.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn active_run_signals_are_searchable_and_open_the_visible_waveform() {
        let mut app = test_app();
        let stale_waveform =
            crate::state::WaveformData::new("V(stale)", vec![0.0, 1.0], vec![0.0, 0.1], "#777777");
        let mut stale_run = crate::state::SimulationRun::new(40);
        stale_run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "tran")
                .with_waveforms(vec![stale_waveform]),
        );

        let op_waveform =
            crate::state::WaveformData::new("I(VDD)", vec![0.0], vec![0.012], "#55aa99");
        let mut target_waveform = crate::state::WaveformData::new(
            "V(precision_out)",
            vec![0.0, 1.0],
            vec![0.0, 2.5],
            "#cc8844",
        );
        target_waveform.visible = false;
        let mut active_run = crate::state::SimulationRun::new(41);
        active_run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::DcOp, "op")
                .with_waveforms(vec![op_waveform]),
        );
        active_run.add_analysis(
            crate::state::AnalysisResult::new(2, crate::state::AnalysisType::Transient, "tran")
                .with_waveforms(vec![target_waveform]),
        );
        let active_dataset_id = active_run.dataset_id;

        app.state.simulation.runs = vec![stale_run, active_run];
        assert!(app.state.simulation.select_run(1));
        assert!(
            app.palette_rows("V(stale)", PaletteScope::Signals)
                .is_empty(),
            "inactive run history must not leak into the active-signal index"
        );
        let mut rows = app.palette_rows("precision_out tran", PaletteScope::Signals);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].detail().contains("Run 41"));
        assert!(rows[0].detail().contains("hidden"));
        let entry = rows.remove(0).entry;
        assert!(matches!(entry, PaletteEntry::Signal(_)));
        assert_eq!(
            entry.command(),
            None,
            "resource rows do not enter command recents"
        );

        app.state.ui.results.viewer = crate::workbench::ResultViewer::Bode;
        app.state.ui.results.hidden_strips.insert(1);
        app.state.ui.results.maximized_strip = Some(0);
        let version_before = app.state.simulation.data_version;
        entry.execute(&mut app).expect("open real result signal");

        assert_eq!(app.state.simulation.active_run_idx, Some(1));
        assert_eq!(app.state.simulation.active_analysis_idx, Some(1));
        assert_eq!(app.state.simulation.runs[1].dataset_id, active_dataset_id);
        assert!(app.state.simulation.runs[1].analyses[1].waveforms[0].visible);
        assert!(app.state.simulation.waveforms[0].visible);
        assert!(app.state.simulation.data_version > version_before);
        assert_eq!(
            app.state.ui.results.viewer,
            crate::workbench::ResultViewer::Waves
        );
        assert!(!app.state.ui.results.hidden_strips.contains(&1));
        assert_eq!(app.state.ui.results.maximized_strip, None);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Results
        );
        assert!(app.state.dialogs.command_palette.recent.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn accesskit_exposes_modal_combobox_scopes_listbox_and_options() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = test_app();
        app.state.dialogs.command_palette.open();
        let output = ctx.run(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog
                && node.label() == Some(DIALOG_LABEL)
                && node.description() == Some(DIALOG_DESCRIPTION)
                && node.is_modal()
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::ComboBox
                && node.label() == Some("Command search")
                && node.is_expanded() == Some(true)
                && !node.controls().is_empty()
                && node.active_descendant().is_some()
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::ListBox
                && node.label() == Some("Matching commands")
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::RadioGroup
                && node.label() == Some("Search scopes")
        }));
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::RadioButton)
                .count(),
            PaletteScope::ALL.len()
        );
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::ListBoxOption && node.label().is_some()
        }));
        let expected_scope = PaletteLayout::resolve(
            Rect::from_min_size(pos2(0.0, 0.0), vec2(1280.0, 800.0)),
            false,
        )
        .regions()
        .scope;
        assert_eq!(
            ctx.read_response(scope_group_id())
                .expect("scope semantic container")
                .rect,
            expected_scope
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn keyboard_navigation_wraps_and_enter_runs_only_available_commands() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = test_app();
        app.state.dialogs.command_palette.open();
        let row_count = app.palette_rows("", PaletteScope::All).len();
        let _ = ctx.run(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));

        let _ = ctx.run(raw_input(vec![key_event(Key::ArrowDown)]), |ctx| {
            app.render_command_palette(ctx);
        });
        assert_eq!(app.state.dialogs.command_palette.selected, 1);

        app.state.dialogs.command_palette.selected = 0;
        let _ = ctx.run(raw_input(vec![key_event(Key::ArrowUp)]), |ctx| {
            app.render_command_palette(ctx);
        });
        assert_eq!(app.state.dialogs.command_palette.selected, row_count - 1);

        app.state.dialogs.command_palette.query = "Stop active run".to_owned();
        app.state.dialogs.command_palette.selected = 0;
        let _ = ctx.run(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
        let _ = ctx.run(raw_input(vec![key_event(Key::Enter)]), |ctx| {
            app.render_command_palette(ctx);
        });
        assert!(app.state.dialogs.command_palette.open);
        assert!(app.state.dialogs.command_palette.recent.is_empty());

        app.state.dialogs.command_palette.query = "About RSpice".to_owned();
        app.state.dialogs.command_palette.selected = 0;
        let _ = ctx.run(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
        let _ = ctx.run(raw_input(vec![key_event(Key::Enter)]), |ctx| {
            app.render_command_palette(ctx);
        });
        assert!(!app.state.dialogs.command_palette.open);
        assert!(app.state.dialogs.about);
        assert_eq!(
            app.state.dialogs.command_palette.recent.first(),
            Some(&Command::About)
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn tab_moves_to_an_action_without_leaving_the_modal_layer() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = test_app();
        app.state.dialogs.command_palette.open();
        let _ = ctx.run(raw_input(Vec::new()), |ctx| app.render_command_palette(ctx));
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

        let _ = ctx.run(raw_input(vec![key_event(Key::Tab)]), |ctx| {
            app.render_command_palette(ctx);
        });
        let focused = ctx.memory(|memory| memory.focused()).expect("modal focus");
        assert_ne!(focused, search_id());
        let response = ctx.read_response(focused).expect("focused action response");
        assert_eq!(
            response.layer_id,
            egui::LayerId::new(Order::Foreground, palette_id())
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn modal_claims_and_restores_prior_focus_on_escape() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let underlying_id = Id::new("command-palette-test-underlying");
        let mut underlying = String::from("baseline");
        let _ = ctx.run(raw_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id))
                    .request_focus();
            });
        });
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id));

        let mut app = test_app();
        app.state.dialogs.command_palette.open();
        let _ = ctx.run(raw_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
            });
            app.render_command_palette(ctx);
        });
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

        ctx.memory_mut(|memory| memory.request_focus(underlying_id));
        let _ = ctx.run(raw_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
            });
            app.render_command_palette(ctx);
        });
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(search_id()));

        let _ = ctx.run(raw_input(vec![key_event(Key::Escape)]), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id));
            });
            app.render_command_palette(ctx);
        });
        assert!(!app.state.dialogs.command_palette.open);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id));
        assert_eq!(underlying, "baseline");
    }
}
