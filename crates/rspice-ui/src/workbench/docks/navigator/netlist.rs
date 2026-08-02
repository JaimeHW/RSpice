//! The netlist navigator: outline, diff, and source mapping.
//!
//! The outline is projected from the canonical netlist document rather than
//! from the editor buffer, so what it lists is what would actually be
//! simulated. Source mapping resolves a line back to the include and directive
//! that produced it, which is the only way a generated line can be traced to
//! the deck an engineer wrote.

use super::*;

pub(super) fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.ui.netlist.active_document
        == crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
    {
        netlist_diff(ui, app);
        return;
    }
    let root_label = active_netlist_artifact_name(&app.state);
    let projection = NetlistNavigatorProjection::from_source(
        &app.state.simulation.netlist_content,
        &app.state.workbench.navigator_query,
        &root_label,
        app.state.ui.netlist.active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated,
    );
    let active_line = app.state.ui.netlist.cursor_line.saturating_add(1);
    let touch_targets =
        app.state.workbench.coarse_pointer || ui.ctx().content_rect().width() <= 820.0;

    ScrollArea::vertical()
        .id_salt("workbench.netlist.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            if !projection.structure_rows.is_empty() {
                section_header(
                    ui,
                    "Structure",
                    Some(&format!("{} lines", projection.line_count)),
                );
                for row in &projection.structure_rows {
                    let selected = row.contains_line(active_line);
                    if netlist_outline_row(ui, row, selected, touch_targets)
                        && let Some(line) = row.target_line
                    {
                        // Keep selection feedback immediate and hand the exact
                        // one-based declaration to the editor's caret/scroll
                        // transaction for the next document frame.
                        app.state.ui.netlist.cursor_line = line.saturating_sub(1);
                        app.state.ui.netlist.requested_line = Some(line);
                    }
                }
            }

            if !projection.include_rows.is_empty() {
                section_header(ui, "Includes", Some(netlist_dependency_status(&app.state)));
                for row in &projection.include_rows {
                    let selected = row.contains_line(active_line);
                    if netlist_outline_row(ui, row, selected, touch_targets)
                        && let Some(line) = row.target_line
                    {
                        app.state.ui.netlist.cursor_line = line.saturating_sub(1);
                        app.state.ui.netlist.requested_line = Some(line);
                    }
                }
            }

            if projection.show_source_mapping {
                section_header(ui, "Source mapping", None);
                netlist_source_mapping(ui, app, active_line);
            }

            if projection.is_empty() {
                muted(
                    ui,
                    "No symbol, directive, or source line matches this filter.",
                );
            }
        });
}

pub(super) fn active_netlist_artifact_name(state: &crate::workbench::AppState) -> String {
    match state.ui.netlist.active_document {
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated => {
            "generated.sp".to_owned()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource => state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .or_else(|| {
                state
                    .workspace
                    .netlist_source_path
                    .as_deref()
                    .and_then(std::path::Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "owned-source.sp".to_owned()),
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff => {
            "generated.diff".to_owned()
        }
    }
}

pub(super) fn active_canonical_netlist_document(
    state: &crate::workbench::AppState,
) -> Option<&crate::state::NetlistDocument> {
    match state.ui.netlist.active_document {
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated => {
            state.ui.netlist.generated_document.as_ref()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource => {
            state.ui.netlist.owned_document.as_ref()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff => None,
    }
}

pub(super) fn netlist_dependency_status(state: &crate::workbench::AppState) -> &'static str {
    let Some(document) = active_canonical_netlist_document(state) else {
        return "unavailable";
    };
    if document.dependencies().iter().any(|dependency| {
        matches!(
            dependency.resolution(),
            crate::state::DependencyResolution::Missing { .. }
        )
    }) {
        "error"
    } else if document.dependency_graph_is_sealed() {
        "resolved"
    } else {
        "unresolved"
    }
}

pub(super) fn netlist_diff(ui: &mut Ui, app: &mut RSpiceApp) {
    let source = &app.state.ui.netlist.generated_diff_source;
    let additions = source
        .lines()
        .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
        .count();
    let removals = source
        .lines()
        .filter(|line| line.starts_with('-') && !line.starts_with("---"))
        .count();
    let hunks = source.lines().filter(|line| line.starts_with("@@")).count();
    ScrollArea::vertical()
        .id_salt("workbench.netlist.diff.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            section_header(ui, "Revision comparison", Some("read-only"));
            nav_row(
                ui,
                WorkbenchIcon::Compare,
                "generated.diff",
                true,
                Some(&format!("{} lines", source.lines().count())),
            );
            nav_row(
                ui,
                WorkbenchIcon::Add,
                "Added lines",
                false,
                Some(&additions.to_string()),
            );
            nav_row(
                ui,
                WorkbenchIcon::Trash,
                "Removed lines",
                false,
                Some(&removals.to_string()),
            );
            nav_row(
                ui,
                WorkbenchIcon::Code,
                "Changed hunks",
                false,
                Some(&hunks.to_string()),
            );
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NetlistNavigatorRowKind {
    Root,
    Parameters,
    Instances,
    Models,
    Analyses,
    Measurements,
    Include,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorRow {
    pub(super) kind: NetlistNavigatorRowKind,
    pub(super) label: String,
    pub(super) meta: Option<String>,
    pub(super) target_line: Option<usize>,
    pub(super) source_ranges: Vec<(usize, usize)>,
}

impl NetlistNavigatorRow {
    pub(super) fn contains_line(&self, line: usize) -> bool {
        self.source_ranges
            .iter()
            .any(|(start, end)| (*start..=*end).contains(&line))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorProjection {
    pub(super) line_count: usize,
    pub(super) structure_rows: Vec<NetlistNavigatorRow>,
    pub(super) include_rows: Vec<NetlistNavigatorRow>,
    pub(super) show_source_mapping: bool,
}

impl NetlistNavigatorProjection {
    pub(super) fn from_source(
        source: &str,
        query: &str,
        root_label: &str,
        source_mapped: bool,
    ) -> Self {
        let outline = NetlistOutline::parse(source);
        let query = NetlistNavigatorQuery::new(query);
        let entries = outline.entries();

        let mut structure_rows = Vec::with_capacity(6);
        let root_entries = entries
            .iter()
            .filter(|entry| entry.kind() == OutlineEntryKind::Title)
            .collect::<Vec<_>>();
        let root_target = root_entries.first().copied().or_else(|| entries.first());
        if query.matches_group(root_label, &root_entries) {
            structure_rows.push(navigator_group_row(
                NetlistNavigatorRowKind::Root,
                root_label,
                "root".to_owned(),
                root_target,
                &root_entries,
            ));
        }

        push_outline_group(
            &mut structure_rows,
            entries,
            &query,
            NetlistNavigatorRowKind::Parameters,
            "Parameters",
            |kind| kind == OutlineEntryKind::Parameter,
        );
        push_outline_group(
            &mut structure_rows,
            entries,
            &query,
            NetlistNavigatorRowKind::Instances,
            "Instances",
            |kind| kind == OutlineEntryKind::Device,
        );
        push_outline_group(
            &mut structure_rows,
            entries,
            &query,
            NetlistNavigatorRowKind::Models,
            "Model bindings",
            |kind| kind == OutlineEntryKind::Model,
        );
        push_outline_group(
            &mut structure_rows,
            entries,
            &query,
            NetlistNavigatorRowKind::Analyses,
            "Analyses",
            |kind| kind == OutlineEntryKind::Analysis,
        );
        push_outline_group(
            &mut structure_rows,
            entries,
            &query,
            NetlistNavigatorRowKind::Measurements,
            "Measurements",
            |kind| kind == OutlineEntryKind::Measurement,
        );

        let include_rows = entries
            .iter()
            .filter(|entry| {
                matches!(
                    entry.kind(),
                    OutlineEntryKind::Include | OutlineEntryKind::Library
                )
            })
            .filter(|entry| query.matches_entry(entry))
            .map(|entry| NetlistNavigatorRow {
                kind: NetlistNavigatorRowKind::Include,
                label: include_entry_label(entry.label()),
                meta: Some(format!("line {}", entry.line())),
                target_line: Some(entry.line()),
                source_ranges: vec![(entry.line(), entry.end_line())],
            })
            .collect();

        Self {
            line_count: source.lines().count(),
            structure_rows,
            include_rows,
            show_source_mapping: source_mapped
                && (query.matches_text("source mapping")
                    || query.matches_text("provenance")
                    || query.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.structure_rows.is_empty() && self.include_rows.is_empty() && !self.show_source_mapping
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorQuery {
    pub(super) text: String,
    pub(super) line: Option<usize>,
}

impl NetlistNavigatorQuery {
    pub(super) fn new(query: &str) -> Self {
        let text = query.trim().to_lowercase();
        let line_literal = text
            .strip_prefix("line")
            .map(str::trim)
            .unwrap_or(&text)
            .trim_start_matches([':', '#']);
        let line = line_literal.parse::<usize>().ok().filter(|line| *line > 0);
        Self { text, line }
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn matches_text(&self, text: &str) -> bool {
        self.is_empty() || text.to_lowercase().contains(&self.text)
    }

    fn matches_entry(&self, entry: &OutlineEntry) -> bool {
        self.is_empty()
            || self
                .line
                .is_some_and(|line| (entry.line()..=entry.end_line()).contains(&line))
            || entry.label().to_lowercase().contains(&self.text)
    }

    fn matches_group(&self, label: &str, entries: &[&OutlineEntry]) -> bool {
        self.matches_text(label) || entries.iter().any(|entry| self.matches_entry(entry))
    }
}

pub(super) fn push_outline_group(
    rows: &mut Vec<NetlistNavigatorRow>,
    entries: &[OutlineEntry],
    query: &NetlistNavigatorQuery,
    row_kind: NetlistNavigatorRowKind,
    label: &str,
    belongs: impl Fn(OutlineEntryKind) -> bool,
) {
    let group_entries = entries
        .iter()
        .filter(|entry| belongs(entry.kind()))
        .collect::<Vec<_>>();
    if !query.matches_group(label, &group_entries) {
        return;
    }
    let matching_entries = group_entries
        .iter()
        .copied()
        .filter(|entry| query.matches_entry(entry))
        .collect::<Vec<_>>();
    let target = matching_entries
        .first()
        .copied()
        .or_else(|| group_entries.first().copied());
    let selected_entries = if query.is_empty() || query.matches_text(label) {
        &group_entries
    } else {
        &matching_entries
    };
    rows.push(navigator_group_row(
        row_kind,
        label,
        group_entries.len().to_string(),
        target,
        selected_entries,
    ));
}

pub(super) fn navigator_group_row(
    kind: NetlistNavigatorRowKind,
    label: &str,
    meta: String,
    target: Option<&OutlineEntry>,
    entries: &[&OutlineEntry],
) -> NetlistNavigatorRow {
    NetlistNavigatorRow {
        kind,
        label: label.to_owned(),
        meta: Some(meta),
        target_line: target.map(OutlineEntry::line),
        source_ranges: if entries.is_empty() {
            target
                .map(|entry| vec![(entry.line(), entry.end_line())])
                .unwrap_or_default()
        } else {
            entries
                .iter()
                .map(|entry| (entry.line(), entry.end_line()))
                .collect()
        },
    }
}

pub(super) fn include_entry_label(label: &str) -> String {
    label
        .split_once(char::is_whitespace)
        .map_or(label, |(_, locator)| locator)
        .trim_matches(['\'', '"'])
        .to_owned()
}

pub(super) fn netlist_outline_row(
    ui: &mut Ui,
    row: &NetlistNavigatorRow,
    selected: bool,
    touch_target: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let height = if touch_target {
        NETLIST_OUTLINE_TOUCH_ROW_HEIGHT
    } else {
        NETLIST_OUTLINE_ROW_HEIGHT
    };
    let enabled = row.target_line.is_some();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            enabled,
            selected,
            row.label.clone(),
        )
    });
    if selected || (enabled && response.hovered()) {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }

    let icon = netlist_outline_icon(row.kind);
    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(
            rect.left() + NETLIST_OUTLINE_PADDING_X + NETLIST_OUTLINE_ICON_SIZE * 0.5,
            rect.center().y,
        ),
        egui::Vec2::splat(NETLIST_OUTLINE_ICON_SIZE),
    );
    let foreground = if !enabled {
        t.color.text_faint
    } else if selected {
        t.color.text
    } else {
        t.color.text_dim
    };
    icon.paint(ui.painter(), icon_rect, foreground);

    let label_left = icon_rect.right() + NETLIST_OUTLINE_ICON_GAP;
    let meta_width = row.meta.as_ref().map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(
                meta.clone(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x
    });
    let label_right = if row.meta.is_some() {
        rect.right() - NETLIST_OUTLINE_PADDING_X - meta_width - NETLIST_OUTLINE_ICON_GAP
    } else {
        rect.right() - NETLIST_OUTLINE_PADDING_X
    };
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            &row.label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            foreground,
        );
    if let Some(meta) = &row.meta {
        ui.painter().text(
            egui::pos2(rect.right() - NETLIST_OUTLINE_PADDING_X, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            meta,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
}

pub(super) fn netlist_outline_icon(kind: NetlistNavigatorRowKind) -> WorkbenchIcon {
    match kind {
        NetlistNavigatorRowKind::Root => WorkbenchIcon::Code,
        NetlistNavigatorRowKind::Parameters => WorkbenchIcon::Sliders,
        NetlistNavigatorRowKind::Instances => WorkbenchIcon::Component,
        NetlistNavigatorRowKind::Models => WorkbenchIcon::Models,
        NetlistNavigatorRowKind::Analyses => WorkbenchIcon::Simulate,
        NetlistNavigatorRowKind::Measurements => WorkbenchIcon::Target,
        NetlistNavigatorRowKind::Include => WorkbenchIcon::File,
    }
}

pub(super) fn netlist_source_mapping(ui: &mut Ui, app: &mut RSpiceApp, active_line: usize) {
    let t = Tokens::get(ui.ctx());
    let mapping = app
        .state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact().source_map_entry(active_line))
        .map(|entry| {
            (
                entry.cell_identity().to_owned(),
                entry.view_identity().to_owned(),
                entry.instance_identity().map(str::to_owned),
                entry.component_identity().map(str::to_owned),
            )
        });
    let Some((cell, view, instance, component)) = mapping else {
        muted(ui, "No generated provenance is mapped to the active line.");
        return;
    };
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 8,
            bottom: 8,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.label(
                egui::RichText::new(format!("Line {active_line} · {cell}"))
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text_dim),
            );
            ui.label(
                egui::RichText::new(view)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            if let Some(instance) = instance.as_deref() {
                ui.label(
                    egui::RichText::new(format!("Instance {instance}"))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            }
            if let Some(component_identity) = component
                && let Some(component_id) = component_identity
                    .rsplit('/')
                    .next()
                    .and_then(|value| value.parse::<u64>().ok())
                && ui.button("Cross-probe schematic component").clicked()
            {
                app.state.schematic.selection.clear();
                app.state.schematic.selection.select_component(component_id);
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                        "Cross-probed generated line {active_line} to component {component_id}."
                    )));
            }
        });
}

/// A navigator row that states, in the row's own place, why a section is
/// empty — a filtered-out list and an empty design must never look alike.
pub(super) fn empty_navigator_row(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width().max(1.0), t.metrics.row_h.max(29.0)),
        egui::Sense::hover(),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        message,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), message)
    });
}

pub(super) fn nav_row(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
) -> bool {
    nav_row_response(ui, icon, label, selected, meta).clicked()
}

pub(super) fn nav_row_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
) -> Response {
    nav_row_indented_response(ui, icon, label, selected, meta, 0)
}

pub(super) fn nav_row_indented(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> bool {
    nav_row_indented_response(ui, icon, label, selected, meta, level).clicked()
}

pub(super) fn nav_row_indented_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> Response {
    nav_row_indented_styled(ui, icon, label, selected, meta, level, false)
}

pub(super) fn schematic_nav_row_indented_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
    expanded: bool,
    child_guide: bool,
) -> Response {
    nav_row_indented_styled_with_metrics(
        ui,
        icon,
        label,
        selected,
        meta,
        level,
        mono,
        SCHEMATIC_NAV_ROW_HEIGHT,
        SCHEMATIC_NAV_LABEL_SIZE,
        SCHEMATIC_NAV_META_SIZE,
        expanded,
        child_guide,
        // accessibility-pointer-shim: the sense is forwarded to
        // `nav_row_indented_styled_with_metrics`, which allocates the row and
        // owns both its WidgetInfo and its focus ring.
        egui::Sense::click(),
    )
}

pub(super) fn schematic_nav_row_indented_drag_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
    expanded: bool,
    child_guide: bool,
) -> Response {
    nav_row_indented_styled_with_metrics(
        ui,
        icon,
        label,
        selected,
        meta,
        level,
        mono,
        SCHEMATIC_NAV_ROW_HEIGHT,
        SCHEMATIC_NAV_LABEL_SIZE,
        SCHEMATIC_NAV_META_SIZE,
        expanded,
        child_guide,
        // accessibility-pointer-shim: same forwarding wrapper, drag variant.
        egui::Sense::click_and_drag(),
    )
}
