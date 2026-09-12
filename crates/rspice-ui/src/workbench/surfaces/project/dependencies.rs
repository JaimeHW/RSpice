//! The project dependency ledger.
//!
//! Rows are derived from the live design-library catalog, model-source
//! authority, and the exact technology binding accepted by the project. This
//! module does not maintain a second lock model: export serializes the same
//! derived view shown on screen.

use serde::Serialize;

use super::*;
use crate::state::model_library::ModelSourceAuthority;
use crate::workbench::app_state::AppState;
use crate::workbench::design_system::property_row_toned;

const DEPENDENCY_SPLIT_BREAKPOINT: f32 = 640.0;
const DEPENDENCY_LEFT_SHARE: f32 = 0.40;
/// Width under which the ledger drops its digest column.
const DEPENDENCY_COMPACT_WIDTH: f32 = 560.0;
/// Shortest the split panes are ever drawn, so a very short viewport still
/// opens a readable ledger rather than seating an action bar under a header.
const DEPENDENCY_MIN_PANE_HEIGHT: f32 = 260.0;
/// Narrowest strip that still seats the field and the facts on one line.
const DEPENDENCY_STRIP_ONE_LINE: f32 = 520.0;
/// Vertical padding inside a panel's action bar.
const DEPENDENCY_ACTION_BAR_PAD: i8 = 6;
/// Painted in a column the resource on that row carries no value for.
const DEPENDENCY_UNSET: &str = "\u{2014}";

const LEDGER_COLUMNS: [&str; 6] = [
    "DEPENDENCY",
    "TYPE",
    "VERSION",
    "SOURCE",
    "DIGEST",
    "STATUS",
];
const LEDGER_FRACTIONS: [f32; 6] = [0.24, 0.14, 0.09, 0.26, 0.11, 0.16];
/// Version and digest are identities rather than prose, so both take the mono
/// face and stay comparable down the column.
const LEDGER_MONO_COLUMNS: [usize; 2] = [2, 4];
const LEDGER_COMPACT_COLUMNS: [&str; 5] = ["DEPENDENCY", "TYPE", "VERSION", "SOURCE", "STATUS"];
const LEDGER_COMPACT_FRACTIONS: [f32; 5] = [0.26, 0.15, 0.14, 0.24, 0.21];
const LEDGER_COMPACT_MONO_COLUMNS: [usize; 1] = [2];

#[derive(Debug, Clone, Serialize)]
struct DependencyManifest {
    schema_version: u16,
    project_id: String,
    project_revision: u64,
    technology: Option<TechnologyManifest>,
    resources: Vec<DependencyRow>,
}

#[derive(Debug, Clone, Serialize)]
struct TechnologyManifest {
    label: String,
    model_library: String,
    root_source: String,
    source_count: usize,
    dependency_edge_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DependencyRow {
    identity: String,
    resource_type: String,
    authority: String,
    version: String,
    source: String,
    digest: String,
    status: String,
}

pub(super) fn dependencies(ui: &mut Ui, app: &mut RSpiceApp) {
    // One projection of live authority per frame, read by the strip, the
    // ledger and the exported manifest alike.
    let rows = dependency_rows(&app.state);
    dependency_context_strip(ui, &mut app.state, &rows);
    let width = visible_workspace_width(ui);
    if width < DEPENDENCY_SPLIT_BREAKPOINT {
        dependency_pane(ui, width, 0.0, |ui| {
            technology_binding_panel(ui, app);
            dependency_inventory(ui, &mut app.state, &rows, width, 0.0);
        });
        return;
    }
    let pane_height = visible_workspace_height(ui).max(DEPENDENCY_MIN_PANE_HEIGHT);
    let left_width = (width * DEPENDENCY_LEFT_SHARE).floor().max(240.0);
    let right_width = (width - left_width - 1.0).max(320.0);
    let shown = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 1.0;
        dependency_pane(ui, left_width, pane_height, |ui| {
            technology_binding_panel(ui, app);
        });
        dependency_pane(ui, right_width, pane_height, |ui| {
            dependency_inventory(ui, &mut app.state, &rows, right_width, pane_height);
        });
    });
    ui.painter().vline(
        shown.response.rect.left() + left_width + 0.5,
        shown.response.rect.y_range(),
        Stroke::new(1.0, Tokens::get(ui.ctx()).color.border_strong),
    );
}

/// One side of the split, drawn to the full height of the workspace so the two
/// panels end on the same line and the divider between them runs all the way
/// down rather than stopping under whichever panel ran out of content first.
fn dependency_pane(ui: &mut Ui, width: f32, height: f32, content: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
        // `allocate_ui_with_layout` advances the cursor by the content width
        // rather than the requested track, so the track is asserted inside —
        // and `set_width` is what stops a child from laying itself out against
        // the central panel's unconstrained width and painting past the clip.
        ui.set_width(width);
        ui.set_min_height(height);
        content(ui);
    });
}

fn dependency_context_strip(ui: &mut Ui, state: &mut AppState, rows: &[DependencyRow]) {
    let t = Tokens::get(ui.ctx());
    let filter = dependency_filter(state);
    let visible = rows.iter().filter(|row| row_matches(row, &filter)).count();
    let resolved = rows.iter().filter(|row| row_is_resolved(row)).count();
    let review = rows.len().saturating_sub(resolved);
    let band = (visible_workspace_width(ui) - 20.0).max(1.0);
    // No rule under this strip: the panel header below opens with one of its
    // own, and two hairlines a pixel apart read as one thick, uneven line.
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            // A `Frame` shrinks to its content, so a band that is not told to
            // take the width stops partway across the workspace — and the
            // width it must take is the visible one.
            ui.set_width(band);
            ui.spacing_mut().item_spacing.y = 4.0;
            if band < DEPENDENCY_STRIP_ONE_LINE {
                // A phone-width strip gives the field its own line and drops
                // the facts under it, rather than breaking one mid-value.
                dependency_filter_field(ui, state, band);
                ui.horizontal(|ui| {
                    dependency_context_facts(ui, resolved, review, visible, rows.len(), &filter);
                });
                return;
            }
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                dependency_filter_field(ui, state, (band * 0.38).clamp(180.0, 340.0));
                ui.separator();
                dependency_context_facts(ui, resolved, review, visible, rows.len(), &filter);
            });
        });
}

fn dependency_filter_field(ui: &mut Ui, state: &mut AppState, width: f32) {
    let search = ui.add_sized(
        [width, Tokens::get(ui.ctx()).metrics.ctl_h],
        egui::TextEdit::singleline(&mut state.workbench.project_dependency_filter)
            .hint_text("Dependency, type, source, version\u{2026}"),
    );
    ui.ctx().accesskit_node_builder(search.id, |node| {
        node.set_label("Filter project dependencies");
    });
}

fn dependency_context_facts(
    ui: &mut Ui,
    resolved: usize,
    review: usize,
    visible: usize,
    total: usize,
    filter: &str,
) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.x = 10.0;
    context_fact(
        ui,
        "Closure",
        &format!("{resolved} resolved \u{00b7} {review} review"),
        if review == 0 {
            t.color.ok
        } else {
            t.color.warn
        },
    );
    // The ledger names its own count in its header; the ratio is only worth a
    // fact while a filter is hiding something.
    if !filter.is_empty() {
        ui.separator();
        context_fact(
            ui,
            "Filter",
            &format!("{visible} of {total} match"),
            t.color.text_dim,
        );
    }
}

fn context_fact(ui: &mut Ui, label: &str, value: &str, color: Color32) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
    ui.label(
        egui::RichText::new(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

/// A panel's action bar, seated on the floor of its pane under a rule.
///
/// Buttons that simply follow the last row read as dropped into the middle of
/// the panel, which is what this page did: the reader cannot tell what they
/// act on, and the space below them belongs to nothing.
fn dependency_action_bar(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, DEPENDENCY_ACTION_BAR_PAD))
        .show(ui, |ui| {
            // A `Frame` shrinks to its content, and an action bar that stops
            // short of the pane edge reads as a floating box.
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                content(ui);
            });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn dependency_action_bar_height(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.ctl_h + 2.0 * f32::from(DEPENDENCY_ACTION_BAR_PAD)
}

/// Free height between a panel's content and its action bar, or nothing when
/// the content already reaches the floor.
fn dependency_panel_free_height(ui: &Ui, pane_height: f32, top: f32) -> f32 {
    pane_height - (ui.cursor().top() - top) - dependency_action_bar_height(ui)
}

fn technology_binding_panel(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 0.0;
    let binding = app.state.workspace.project.technology_binding().cloned();
    let contract = app.state.validate_project_technology_contract();
    let contract_ok = binding.is_some() && contract.is_ok();
    workspace_table_panel_header(
        ui,
        "Technology & simulator binding",
        if contract_ok { "PINNED" } else { "REVIEW" },
        if contract_ok {
            t.color.ok
        } else {
            t.color.warn
        },
    );
    if let Some(binding) = binding.as_ref() {
        property_row(ui, "Technology", &binding.display_label());
        property_row(ui, "Model library", binding.model_library());
        let source = binding.root_source().display().to_string();
        property_row(ui, "Root source", &source).on_hover_text(source.as_str());
        let files = binding.source_closure().len();
        let edges = binding.source_edges().len();
        property_row(
            ui,
            "Source closure",
            &format!(
                "{files} file{} \u{00b7} {edges} edge{}",
                plural_suffix(files),
                plural_suffix(edges)
            ),
        );
    } else {
        // An unset value is not a value: toning these faint keeps the reader
        // on the rows below that do report live state.
        property_row_toned(ui, "Technology", "Not attached", t.color.text_faint);
        property_row_toned(
            ui,
            "Model library",
            "No executable model set",
            t.color.text_faint,
        );
        property_row_toned(
            ui,
            "Root source",
            "No authenticated root source",
            t.color.text_faint,
        );
        property_row_toned(
            ui,
            "Source closure",
            "No pinned source files",
            t.color.text_faint,
        );
    }
    if let Some(pin) = binding
        .as_ref()
        .and_then(crate::state::ProjectTechnologyBinding::signed_package)
    {
        property_row(
            ui,
            "Signed package",
            &format!(
                "{} \u{00b7} {} \u{00b7} {} nm \u{00b7} {}",
                pin.package_id(),
                pin.revision(),
                pin.process_node_nm(),
                pin.stack_name()
            ),
        );
        property_row(
            ui,
            "Publisher authority",
            &format!(
                "{} \u{00b7} key {}",
                pin.publisher_id(),
                pin.signing_key_id()
            ),
        );
        property_row(
            ui,
            "Package digests",
            &format!(
                "manifest {} \u{00b7} archive {}",
                short_identity(&pin.manifest_digest().to_string()),
                short_identity(&pin.archive_digest().to_string())
            ),
        );
    } else {
        property_row_toned(ui, "Signed package", "Not attached", t.color.text_faint);
    }
    let libraries = app.state.model_library_manager.library_count();
    let models = app.state.model_library_manager.total_model_count();
    property_row(
        ui,
        "Model catalog",
        &format!(
            "{libraries} librar{} \u{00b7} {models} model{}",
            if libraries == 1 { "y" } else { "ies" },
            plural_suffix(models)
        ),
    );
    property_row(
        ui,
        "Execution",
        crate::state::ExecutionTarget::current().label(),
    );
    technology_contract_note(
        ui,
        &contract.err().unwrap_or_else(|| {
            "Model sources, signed package bytes, publisher signature, and runtime compatibility validate exactly."
                .to_owned()
        }),
        contract_ok,
    );
    dependency_action_bar(ui, |ui| {
        if Button::new(TECHNOLOGY_SURFACE_ACTION)
            .accent()
            .show(ui)
            .clicked()
        {
            open_technology_attachment_dialog(app);
        }
        if Button::new("Model paths\u{2026}").show(ui).clicked() {
            Command::PdkSettings.execute(app);
        }
    });
    // The panel ends here rather than at the floor of the pane, so close it.
    // `min_rect` already reaches the floor — the pane asserts its own height so
    // the divider runs the whole way down — and the cursor is what tracks the
    // content.
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

/// The verdict on the project's technology contract, carried by a rule in the
/// verdict's own colour rather than by a tinted box that reads as a tooltip
/// stranded between the property list and the buttons.
fn technology_contract_note(ui: &mut Ui, message: &str, contract_ok: bool) {
    let t = Tokens::get(ui.ctx());
    let tone = if contract_ok {
        t.color.ok
    } else {
        t.color.warn
    };
    let shown = egui::Frame::new()
        .fill(theme::mix(t.color.bg_panel, tone, 0.05))
        .inner_margin(Margin {
            left: 12,
            right: 10,
            top: 7,
            bottom: 7,
        })
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    egui::RichText::new(message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    let rect = shown.response.rect;
    ui.painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    ui.painter().rect_filled(
        Rect::from_min_size(rect.left_top(), vec2(2.0, rect.height())),
        0.0,
        tone,
    );
}

fn dependency_inventory(
    ui: &mut Ui,
    state: &mut AppState,
    rows: &[DependencyRow],
    pane_width: f32,
    pane_height: f32,
) {
    let t = Tokens::get(ui.ctx());
    let top = ui.cursor().top();
    ui.spacing_mut().item_spacing.y = 0.0;
    let filter = dependency_filter(state);
    let visible = rows
        .iter()
        .filter(|row| row_matches(row, &filter))
        .collect::<Vec<_>>();
    workspace_table_panel_header(
        ui,
        "Resolved dependency ledger",
        &format!("{} SHOWN", visible.len()),
        if visible.is_empty() {
            t.color.text_faint
        } else {
            t.color.ok
        },
    );
    // The pane owns the column grid, not the unconstrained central panel: a
    // ledger bound to the wrong width paints its status column underneath the
    // Inspector dock. Both layouts elide into the pane, so the columns are
    // always complete and the table never scrolls sideways.
    let width = pane_width.min(visible_workspace_width(ui)).max(1.0);
    let compact = width < DEPENDENCY_COMPACT_WIDTH;
    if compact {
        workspace_table_row(
            ui,
            width,
            LEDGER_COMPACT_COLUMNS,
            LEDGER_COMPACT_FRACTIONS,
            WorkspaceRowKind::Header,
            &[],
            &[],
        );
    } else {
        workspace_table_row(
            ui,
            width,
            LEDGER_COLUMNS,
            LEDGER_FRACTIONS,
            WorkspaceRowKind::Header,
            &[],
            &[],
        );
    }
    if rows.is_empty() {
        workspace_empty_table_row(
            ui,
            width,
            "No design or executable model dependencies are currently registered.",
        );
    } else if visible.is_empty() {
        workspace_empty_table_row(ui, width, "No dependency matches the current filter.");
    }
    let mut selected = None;
    for row in &visible {
        let kind = WorkspaceRowKind::body(
            state.workbench.project_dependency_selection.as_deref() == Some(row.identity.as_str()),
        );
        let status = status_label(&row.status);
        let tone = status_tone(&t, &row.status);
        let response = if compact {
            let cells = [
                row.identity.as_str(),
                row.resource_type.as_str(),
                row.version.as_str(),
                row.source.as_str(),
                status.as_str(),
            ];
            workspace_table_row(
                ui,
                width,
                cells,
                LEDGER_COMPACT_FRACTIONS,
                kind,
                &LEDGER_COMPACT_MONO_COLUMNS,
                &cell_tones(&cells, 4, tone, t.color.text_faint),
            )
            .0
        } else {
            let cells = [
                row.identity.as_str(),
                row.resource_type.as_str(),
                row.version.as_str(),
                row.source.as_str(),
                row.digest.as_str(),
                status.as_str(),
            ];
            workspace_table_row(
                ui,
                width,
                cells,
                LEDGER_FRACTIONS,
                kind,
                &LEDGER_MONO_COLUMNS,
                &cell_tones(&cells, 5, tone, t.color.text_faint),
            )
            .0
        };
        // Authority is the one field with no column of its own; the row still
        // carries it rather than spending a column on a value that repeats.
        if response
            .on_hover_text(format!(
                "{}\nAuthority: {}\nSource: {}",
                row.identity, row.authority, row.source
            ))
            .clicked()
        {
            selected = Some(row.identity.clone());
        }
    }
    if let Some(identity) = selected {
        state.workbench.project_dependency_selection = Some(identity);
    }
    if compact {
        ledger_grid_floor(ui, width, pane_height, top, LEDGER_COMPACT_FRACTIONS);
    } else {
        ledger_grid_floor(ui, width, pane_height, top, LEDGER_FRACTIONS);
    }
    dependency_action_bar(ui, |ui| {
        if Button::new("Export manifest\u{2026}").show(ui).clicked() {
            export_dependency_manifest(ui.ctx(), state);
        }
    });
}

/// Carry the ledger's column rules down to the action bar.
///
/// A table that simply stops leaves its pane looking half-drawn; continuing
/// the grid says the columns are complete and the rows have run out.
fn ledger_grid_floor<const N: usize>(
    ui: &mut Ui,
    width: f32,
    pane_height: f32,
    top: f32,
    fractions: [f32; N],
) {
    let free = dependency_panel_free_height(ui, pane_height, top);
    if free <= 0.0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(width, free), Sense::hover());
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    let mut left = rect.left();
    for fraction in fractions.iter().take(N.saturating_sub(1)) {
        left += rect.width() * fraction;
        painter.vline(left, rect.y_range(), Stroke::new(1.0, t.color.border));
    }
}

fn dependency_filter(state: &AppState) -> String {
    state
        .workbench
        .project_dependency_filter
        .trim()
        .to_ascii_lowercase()
}

/// Whether a resource is resolved against live authority, as opposed to
/// awaiting review.
fn row_is_resolved(row: &DependencyRow) -> bool {
    matches!(row.status.as_str(), "accepted" | "available" | "embedded")
}

fn status_tone(t: &Tokens, status: &str) -> Color32 {
    match status {
        "accepted" | "available" | "embedded" => t.color.ok,
        "catalog only" => t.color.text_dim,
        _ => t.color.warn,
    }
}

/// A status reads as a verdict in a column of verdicts, so it is capitalized
/// like one. The serialized manifest keeps the lowercase key.
fn status_label(status: &str) -> String {
    let mut characters = status.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + characters.as_str()
    })
}

/// Per-cell colours for one ledger row: the status verdict, and the faint tone
/// every placeholder takes so an absent value never competes with a real one.
fn cell_tones<const N: usize>(
    cells: &[&str; N],
    status_column: usize,
    status: Color32,
    unset: Color32,
) -> Vec<(usize, Color32)> {
    let mut tones = Vec::with_capacity(3);
    tones.push((status_column, status));
    for (index, cell) in cells.iter().enumerate() {
        if *cell == DEPENDENCY_UNSET {
            tones.push((index, unset));
        }
    }
    tones
}

fn dependency_rows(state: &AppState) -> Vec<DependencyRow> {
    let mut rows = Vec::new();
    for library in state.library_manager.libraries_sorted() {
        rows.push(DependencyRow {
            identity: library.name.clone(),
            resource_type: "Design library".to_owned(),
            authority: if library.read_only {
                "Read only".to_owned()
            } else {
                "Project writable".to_owned()
            },
            version: nonempty_or_dash(&library.technology),
            source: library.path.as_ref().map_or_else(
                || "Embedded project data".to_owned(),
                |path| display_path(path),
            ),
            digest: "—".to_owned(),
            status: if library.path.is_some() {
                "available".to_owned()
            } else {
                "embedded".to_owned()
            },
        });
    }
    for library in state.model_library_manager.libraries_sorted() {
        let authority = match library.source_authority {
            ModelSourceAuthority::BuiltIn => "Built-in catalog",
            ModelSourceAuthority::External => "External source",
            ModelSourceAuthority::RetainedImport { .. } => "Retained imported source",
            ModelSourceAuthority::ProjectOwned { .. } => "Project owned",
        };
        let status = if !library.source_authority.has_execution_source() {
            "catalog only"
        } else if library.source_closure.is_empty() {
            "source not pinned"
        } else if library.source_contents.len() != library.source_closure.len() {
            "retained bytes incomplete"
        } else {
            "accepted"
        };
        let digest = library
            .source_closure
            .iter()
            .find(|pin| library.root_path.as_ref() == Some(&pin.path))
            .or_else(|| library.source_closure.first())
            .map_or_else(
                || "—".to_owned(),
                |pin| short_identity(&pin.digest.to_string()),
            );
        rows.push(DependencyRow {
            identity: library.name.clone(),
            resource_type: "Model library".to_owned(),
            authority: authority.to_owned(),
            version: nonempty_or_dash(&library.version),
            source: library.root_path.as_ref().map_or_else(
                || "No executable source".to_owned(),
                |path| display_path(path),
            ),
            digest,
            status: status.to_owned(),
        });
    }
    rows.sort_by(|left, right| {
        left.resource_type
            .cmp(&right.resource_type)
            .then_with(|| left.identity.cmp(&right.identity))
    });
    rows
}

fn row_matches(row: &DependencyRow, filter: &str) -> bool {
    let filter = filter.to_ascii_lowercase();
    filter.is_empty()
        || [
            &row.identity,
            &row.resource_type,
            &row.authority,
            &row.version,
            &row.source,
            &row.digest,
            &row.status,
        ]
        .iter()
        .any(|value| value.to_ascii_lowercase().contains(&filter))
}

fn nonempty_or_dash(value: &str) -> String {
    if value.trim().is_empty() {
        "—".to_owned()
    } else {
        value.to_owned()
    }
}

fn display_path(path: &std::path::Path) -> String {
    path.display().to_string()
}

fn dependency_manifest(state: &AppState) -> DependencyManifest {
    let technology =
        state
            .workspace
            .project
            .technology_binding()
            .map(|binding| TechnologyManifest {
                label: binding.display_label(),
                model_library: binding.model_library().to_owned(),
                root_source: binding.root_source().display().to_string(),
                source_count: binding.source_closure().len(),
                dependency_edge_count: binding.source_edges().len(),
            });
    DependencyManifest {
        schema_version: 1,
        project_id: state.workspace.project.id().to_string(),
        project_revision: state.workspace.project.revision().get(),
        technology,
        resources: dependency_rows(state),
    }
}

fn export_dependency_manifest(ctx: &Context, state: &mut AppState) {
    let contents = match serde_json::to_vec_pretty(&dependency_manifest(state)) {
        Ok(contents) => contents,
        Err(error) => {
            state.ui.toasts.error_with_title(
                ctx,
                "Manifest export failed",
                format!("Dependency manifest serialization failed: {error}"),
            );
            return;
        }
    };
    let filename = format!(
        "{}-dependencies.json",
        state
            .workspace
            .project
            .display_name()
            .chars()
            .map(|character| if character.is_alphanumeric() {
                character
            } else {
                '-'
            })
            .collect::<String>()
            .trim_matches('-')
    );

    match crate::workbench::workflows::export_workflow::publish_project_manifest(
        &filename, &contents,
    ) {
        Ok(Some(receipt)) => state.ui.toasts.success(ctx, "Manifest exported", receipt),
        Ok(None) => {}
        Err(error) => state
            .ui
            .toasts
            .error_with_title(ctx, "Manifest export failed", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependency_filter_matches_every_visible_field() {
        let row = DependencyRow {
            identity: "models_tt".to_owned(),
            resource_type: "Model library".to_owned(),
            authority: "Project owned".to_owned(),
            version: "2.1".to_owned(),
            source: "/models/tt.lib".to_owned(),
            digest: "12ab".to_owned(),
            status: "accepted".to_owned(),
        };
        assert!(row_matches(&row, "project"));
        assert!(row_matches(&row, "tt.lib"));
        assert!(row_matches(&row, "12AB"));
        assert!(!row_matches(&row, "missing"));
    }
}
