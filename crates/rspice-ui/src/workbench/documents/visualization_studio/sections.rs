//! The section editors behind the studio's left navigation.
//!
//! Each section edits one part of the visualization document — document
//! properties, axes, traces, cursors, markers, measurements — and every edit
//! is committed through the document's transaction rather than mutating the
//! displayed state directly, so a rejected edit leaves the studio showing
//! exactly what it showed before.

use super::*;

pub(super) fn show_active_section(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    if compact {
        match app.state.workbench.visualization_studio.touch_pane {
            VisualizationTouchPane::Sections => return,
            VisualizationTouchPane::Inspector => {
                viewer_inspector(ui, app, true);
                return;
            }
            VisualizationTouchPane::Actions => {
                actions_sheet(ui, app);
                return;
            }
            VisualizationTouchPane::Stage => {}
        }
    }

    match app.state.workbench.visualization_studio.section {
        VisualizationSection::Document => document_section(ui, app),
        VisualizationSection::Viewers => viewers_section(ui, app, compact),
        VisualizationSection::Axes => axes_section(ui, app),
        VisualizationSection::Families => families_section(ui, app),
        VisualizationSection::Measurements => measurements_section(ui, app),
        VisualizationSection::LargeData => large_data_section(ui, app),
        VisualizationSection::ExportReport => export_section(ui, app),
    }
}

pub(super) fn section_heading(ui: &mut Ui, section: VisualizationSection) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.label(
                RichText::new(format!("VIZ · {}", section.label().to_uppercase()))
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text_faint),
            );
            ui.label(
                RichText::new(section.title())
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                RichText::new(section.description())
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
    separator(ui, t.color.border);
}

pub(super) fn section_scroll(ui: &mut Ui, id: &'static str, content: impl FnOnce(&mut Ui)) {
    Frame::NONE
        .inner_margin(Margin {
            left: 12,
            right: 12,
            top: 0,
            bottom: 12,
        })
        .show(ui, |ui| {
            ScrollArea::both()
                .id_salt(id)
                .auto_shrink([false, false])
                .show(ui, content);
        });
}

pub(super) fn document_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::Document);
    section_scroll(ui, "visualization.document", |ui| {
        Grid::new("visualization.document.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in [
                    "Pane",
                    "Viewer",
                    "Dataset",
                    "X link",
                    "Cursor group",
                    "Page",
                ] {
                    table_header(ui, label);
                }
                ui.end_row();
                if app.state.workbench.visualization_studio.panes.is_empty() {
                    ui.label("—");
                    ui.label("No panes in this result document");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
                for pane in &app.state.workbench.visualization_studio.panes {
                    ui.monospace(format!("{:02}", pane.id));
                    ui.label(pane.viewer.label());
                    ui.monospace(short_dataset(pane.dataset_id));
                    ui.monospace(
                        pane.x_link
                            .map_or_else(|| "none".to_owned(), |id| format!("x-{id}")),
                    );
                    ui.monospace(
                        pane.cursor_group
                            .map_or_else(|| "none".to_owned(), |id| format!("cursor-{id}")),
                    );
                    ui.label(&pane.page);
                    ui.end_row();
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, app, "Add pane…", VisualizationDock::AddPane);
            dock_action(ui, app, "Reorder panes…", VisualizationDock::ReorderPanes);
            dock_action(ui, app, "Link groups…", VisualizationDock::LinkGroups);
            dock_action(ui, app, "Page editor…", VisualizationDock::PageEditor);
        });
    });
}

pub(super) fn axes_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::Axes);
    section_scroll(ui, "visualization.axes", |ui| {
        Grid::new("visualization.axes.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Axis", "Quantity", "Transform", "Range", "Ticks", "Unit"] {
                    table_header(ui, label);
                }
                ui.end_row();
                let Some(analysis) = app.state.simulation.active_analysis() else {
                    ui.label("—");
                    ui.label("No active result dataset");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                    return;
                };
                if let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible)
                {
                    let (x0, x1) = waveform.x_range();
                    let (y0, y1) = waveform.y_range();
                    let frequency = matches!(
                        analysis.analysis_type,
                        crate::state::AnalysisType::Ac | crate::state::AnalysisType::Noise
                    );
                    axis_row(
                        ui,
                        "X1",
                        if frequency {
                            "frequency"
                        } else {
                            "time / sweep"
                        },
                        if frequency { "log10" } else { "linear" },
                        (x0, x1),
                        if frequency { "decade" } else { "engineering" },
                        if frequency { "Hz" } else { "source" },
                    );
                    axis_row(
                        ui,
                        "Y1L",
                        &waveform.name,
                        "linear",
                        (y0, y1),
                        "engineering",
                        "source",
                    );
                    if waveform.complex.is_some() {
                        axis_row(
                            ui,
                            "Y1R",
                            "complex projection",
                            "phase",
                            (-180.0, 180.0),
                            "45°",
                            "deg",
                        );
                    }
                } else {
                    ui.label("—");
                    ui.label("Active analysis has no visible waveform");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
            });
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
                labeled_combo(
                    ui,
                    "Autoscale",
                    app.state.workbench.visualization_studio.autoscale.label(),
                    |ui| {
                        for value in VisualizationAutoscale::ALL {
                            let configured = match value {
                                VisualizationAutoscale::RobustVisible => true,
                                VisualizationAutoscale::ExactExtrema => {
                                    app.state.ui.results.viewer == ResultViewer::Waves
                                }
                                VisualizationAutoscale::SpecificationBounds => {
                                    specification_bound_fit(&app.state).is_some()
                                }
                            };
                            ui.add_enabled_ui(configured, |ui| {
                                ui.selectable_value(
                                    &mut app.state.workbench.visualization_studio.autoscale,
                                    value,
                                    value.label(),
                                );
                            })
                            .response
                            .on_disabled_hover_text(
                                "This fit policy is unavailable for the active renderer or requires a quantity-mapped axis limit.",
                            );
                        }
                    },
                );
                labeled_combo(
                    ui,
                    "Complex projection",
                    app.state
                        .workbench
                        .visualization_studio
                        .complex_projection
                        .label(),
                    |ui| {
                        for value in ComplexProjection::ALL {
                            ui.selectable_value(
                                &mut app.state.workbench.visualization_studio.complex_projection,
                                value,
                                value.label(),
                            );
                        }
                    },
                );
                let fit_blocker = fit_block_reason(&app.state);
                let fit = Button::new("Fit active view")
                    .enabled(fit_blocker.is_none())
                    .show(ui);
                let fit = if let Some(reason) = fit_blocker {
                    fit.on_disabled_hover_text(reason)
                } else {
                    fit
                };
                if fit.clicked() {
                    fit_active_view(app);
                }
            });
    });
}

pub(super) fn axis_row(
    ui: &mut Ui,
    axis: &str,
    quantity: &str,
    transform: &str,
    range: (f64, f64),
    ticks: &str,
    unit: &str,
) {
    ui.monospace(axis);
    ui.label(quantity);
    ui.monospace(transform);
    ui.monospace(format!("{:.6e}…{:.6e}", range.0, range.1));
    ui.label(ticks);
    ui.monospace(unit);
    ui.end_row();
}

pub(super) fn families_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::Families);
    let active_dataset = app.state.simulation.active_run().map(|run| run.dataset_id);
    let rows: Vec<_> = app
        .state
        .simulation
        .runs
        .iter()
        .map(|run| {
            let samples = run
                .analyses
                .iter()
                .flat_map(|analysis| &analysis.waveforms)
                .map(|waveform| waveform.x.len().min(waveform.y.len()))
                .sum::<usize>();
            (
                run.dataset_id,
                run.label.clone(),
                run.analyses.len(),
                samples,
                Some(run.dataset_id) == active_dataset,
                app.state.simulation.is_dataset_overlaid(run.dataset_id),
            )
        })
        .collect();
    section_scroll(ui, "visualization.families", |ui| {
        Grid::new("visualization.families.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Dataset", "Run", "Analyses", "Samples", "Role", "Display"] {
                    table_header(ui, label);
                }
                ui.end_row();
                if rows.is_empty() {
                    ui.label("—");
                    ui.label("No retained datasets");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
                let mut overlay_change = None;
                for (dataset_id, label, analyses, samples, active, overlaid) in &rows {
                    ui.monospace(short_dataset(*dataset_id));
                    ui.label(label);
                    ui.monospace(analyses.to_string());
                    ui.monospace(engineering_count(*samples));
                    ui.label(if *active {
                        "active family"
                    } else {
                        "retained family"
                    });
                    if *active {
                        ui.label("always visible");
                    } else if ui.checkbox(&mut overlaid.clone(), "Overlay").changed() {
                        overlay_change = Some(*dataset_id);
                    }
                    ui.end_row();
                }
                if let Some(dataset_id) = overlay_change {
                    app.state.simulation.toggle_dataset_overlay(dataset_id);
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, app, "Slice and pivot…", VisualizationDock::FamilySlice);
            dock_action(
                ui,
                app,
                "Visual encoding…",
                VisualizationDock::FamilyEncoding,
            );
            dock_action(ui, app, "Advanced filter…", VisualizationDock::FamilyFilter);
        });
        concept_banner(
            ui,
            "Dataset overlays use stable dataset identities. Missing analyses remain absent; the viewer never invents family points or generated trace indices.",
        );
    });
}

pub(super) fn measurements_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::Measurements);
    section_scroll(ui, "visualization.measurements", |ui| {
        Grid::new("visualization.measurements.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Item", "Type", "Definition", "Unit", "Consumers", "Status"] {
                    table_header(ui, label);
                }
                ui.end_row();
                let strip = app.state.simulation.active_analysis_idx.unwrap_or_default();
                let expressions = app
                    .state
                    .ui
                    .results
                    .exprs
                    .get(&strip)
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                for measurement in &app.state.workbench.visualization_studio.measurements {
                    measurement_row(
                        ui,
                        &format!("M{}", measurement.id),
                        "scalar measurement",
                        &measurement.expression,
                        "source-derived",
                        &short_dataset(measurement.dataset_id),
                        &format!("{:.9e}", measurement.value),
                    );
                }
                for (index, expression) in expressions.iter().enumerate() {
                    measurement_row(
                        ui,
                        &format!("expr-{}", index + 1),
                        "expression",
                        &expression.text,
                        "source-derived",
                        "active pane",
                        if expression.visible {
                            "visible"
                        } else {
                            "hidden"
                        },
                    );
                }
                if app.state.ui.results.cursors.a.is_some()
                    || app.state.ui.results.cursors.b.is_some()
                {
                    measurement_row(
                        ui,
                        "A / B",
                        "linked cursors",
                        "exact source coordinates",
                        "source",
                        "compatible panes",
                        if app.state.ui.results.linked_cursors {
                            "linked"
                        } else {
                            "pane local"
                        },
                    );
                }
                for marker in &app.state.workbench.visualization_studio.markers {
                    measurement_row(
                        ui,
                        &marker.label,
                        "sample marker",
                        &format!(
                            "{}[{}] @ {:.9e}",
                            marker.waveform_name, marker.sample_index, marker.x
                        ),
                        "source",
                        "active pane",
                        "exact",
                    );
                }
                for annotation in &app.state.workbench.visualization_studio.annotations {
                    measurement_row(
                        ui,
                        &format!("NOTE-{}", annotation.id),
                        "review annotation",
                        &annotation.text,
                        "—",
                        "result document",
                        "open",
                    );
                }
                if app
                    .state
                    .workbench
                    .visualization_studio
                    .measurements
                    .is_empty()
                    && expressions.is_empty()
                    && app.state.ui.results.cursors.a.is_none()
                    && app.state.workbench.visualization_studio.markers.is_empty()
                    && app
                        .state
                        .workbench
                        .visualization_studio
                        .annotations
                        .is_empty()
                {
                    ui.label("—");
                    ui.label("No derived or review entities");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, app, "New measurement…", VisualizationDock::Measurement);
            dock_action(ui, app, "Cursor manager…", VisualizationDock::CursorManager);
            dock_action(ui, app, "New annotation…", VisualizationDock::Annotation);
        });
    });
}

pub(super) fn measurement_row(
    ui: &mut Ui,
    item: &str,
    kind: &str,
    definition: &str,
    unit: &str,
    consumers: &str,
    status: &str,
) {
    ui.monospace(item);
    ui.label(kind);
    ui.monospace(definition);
    ui.monospace(unit);
    ui.label(consumers);
    ui.label(status);
    ui.end_row();
}

pub(super) fn large_data_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::LargeData);
    section_scroll(ui, "visualization.large-data", |ui| {
        let previous = app.state.workbench.visualization_studio.display_lod;
        ui.horizontal_wrapped(|ui| {
            labeled_combo(
                ui,
                "Display LOD",
                app.state.workbench.visualization_studio.display_lod.label(),
                |ui| {
                    for value in DisplayLodPolicy::ALL {
                        ui.selectable_value(
                            &mut app.state.workbench.visualization_studio.display_lod,
                            value,
                            value.label(),
                        );
                    }
                },
            );
            numeric_policy(
                ui,
                "Tile memory",
                &mut app.state.workbench.visualization_studio.tile_memory_mib,
                64..=16_384,
                "MiB",
            );
        });
        // A property row is a full-width label/value row with fixed columns,
        // so the cache policy is stated under the two controls rather than
        // claiming whatever is left of their wrapped row.
        property_row(ui, "Disk cache", "Not configured · no filesystem writes");
        ui.add_space(10.0);
        Grid::new("visualization.large-data.policies")
            .num_columns(2)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                policy_row(
                    ui,
                    "Exact cursor query",
                    "Read original f64/complex source samples on demand",
                );
                policy_row(
                    ui,
                    "Remote streaming",
                    "Local immutable dataset registry; remote sources fail closed",
                );
                policy_row(
                    ui,
                    "Backpressure",
                    "Preserve solver output · delay presentation cache",
                );
                policy_row(
                    ui,
                    "Source precision",
                    "Measurements and exports bypass display LOD",
                );
            });
        if previous != app.state.workbench.visualization_studio.display_lod {
            apply_lod_policy(app);
        }
        concept_banner(
            ui,
            "Decimation and level-of-detail affect rendering only. Measurements, exports, and cursor exact-value requests operate on the immutable source dataset.",
        );
    });
}

pub(super) fn export_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::ExportReport);
    section_scroll(ui, "visualization.export", |ui| {
        Grid::new("visualization.export.table")
            .num_columns(5)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Output", "Format", "Precision", "Layout", "Provenance"] {
                    table_header(ui, label);
                }
                ui.end_row();
                export_row(
                    ui,
                    "Active engineering viewer",
                    "PNG",
                    "rendered pixels",
                    "active viewport",
                    "dataset + revision in document",
                );
                export_row(
                    ui,
                    "Engineering dataset",
                    "CSV",
                    "full stored f64",
                    "shared-axis table",
                    "source analysis identity",
                );
            });
        ui.add_space(10.0);
        let exact_export_available = active_studio_exact_export_available(&app.state);
        let figure_export_available = active_studio_figure_export_available(&app.state);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, app, "Edit report pages…", VisualizationDock::PageEditor);
            if Button::new("Export exact data…")
                .accent()
                .enabled(exact_export_available)
                .show(ui)
                .clicked()
            {
                app.state.ui.export_csv_requested = true;
            }
            if Button::new("Export viewer figure…")
                .enabled(figure_export_available)
                .show(ui)
                .clicked()
            {
                app.state.ui.export_figure_requested = true;
            }
        });
        concept_banner(
            ui,
            "Every enabled export action is backed by a real writer. Formats without an installed writer are not offered and no placeholder artifact is created.",
        );
    });
}

pub(super) fn export_row(
    ui: &mut Ui,
    output: &str,
    format: &str,
    precision: &str,
    layout: &str,
    provenance: &str,
) {
    ui.label(output);
    ui.monospace(format);
    ui.label(precision);
    ui.label(layout);
    ui.label(provenance);
    ui.end_row();
}

pub(super) fn viewers_section(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    section_heading(ui, VisualizationSection::Viewers);
    viewer_toolbar(ui, app, compact);
    let height = ui.available_height().max(1.0);
    if compact {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), height),
            Layout::top_down(Align::Min),
            |ui| viewer_stage(ui, app),
        );
        return;
    }

    // A horizontally scrollable ancestor may expose a logical available
    // width wider than the visible canvas. The mockup columns are viewport
    // columns, so clamp their allocation to the active clip rectangle.
    let available = visible_available_width(
        ui.available_width(),
        ui.cursor().left(),
        ui.clip_rect().right(),
    );
    let (library_width, inspector_width) = if available <= NARROW_VIEWER_BREAKPOINT {
        (158.0, 196.0)
    } else {
        (190.0, 224.0)
    };
    // `allocate_ui_with_layout` is allowed to grow beyond its requested size
    // when a descendant reports a larger minimum. The exact-data table and
    // long status strings therefore used to steal width from the inspector at
    // 1280 px even though the mockup declares fixed 190/224 px side columns.
    // Reserve and clip all three column rectangles up front so content can
    // scroll or elide within its owner, never resize a sibling pane.
    let (rect, _) = ui.allocate_exact_size(vec2(available, height), Sense::hover());
    let [library_rect, stage_rect, inspector_rect] =
        viewer_column_rects(rect, library_width, inspector_width);
    let t = Tokens::get(ui.ctx());
    for x in [library_rect.right() + 0.5, inspector_rect.left() - 0.5] {
        ui.painter()
            .vline(x, rect.y_range(), Stroke::new(1.0, t.color.border_strong));
    }

    let mut library_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(library_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    library_ui.set_clip_rect(library_ui.clip_rect().intersect(library_rect));
    viewer_library(&mut library_ui, app);

    let mut stage_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(stage_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    stage_ui.set_clip_rect(stage_ui.clip_rect().intersect(stage_rect));
    viewer_stage(&mut stage_ui, app);

    let mut inspector_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inspector_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    inspector_ui.set_clip_rect(inspector_ui.clip_rect().intersect(inspector_rect));
    viewer_inspector(&mut inspector_ui, app, false);
}

pub(super) fn viewer_toolbar(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    let t = Tokens::get(ui.ctx());
    let bar = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.set_min_height(if compact {
                44.0
            } else {
                bar_content_height(VIEWER_TOOLBAR_HEIGHT, VIEWER_TOOLBAR_VERTICAL_MARGIN)
            });
            ScrollArea::horizontal()
                .id_salt("visualization.viewer-toolbar")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        let waveform_coordinates =
                            app.state.ui.results.viewer == ResultViewer::Waves;
                        for tool in ViewerTool::ALL {
                            let active = app.state.workbench.visualization_studio.tool == tool;
                            if ui
                                .add_sized(
                                    [
                                        if compact { 64.0 } else { 54.0 },
                                        if compact { 42.0 } else { 26.0 },
                                    ],
                                    egui::Button::new(tool.label()).selected(active),
                                )
                                .clicked()
                            {
                                app.state.workbench.visualization_studio.tool = tool;
                            }
                        }
                        toolbar_action(ui, "Add trace", || open_trace_manager(app));
                        toolbar_action(ui, "Edit axis", || {
                            app.state.workbench.visualization_studio.section =
                                VisualizationSection::Axes;
                        });
                        toolbar_action_enabled(
                            ui,
                            "Add cursor",
                            waveform_coordinates,
                            "Exact source cursors are available in the waveform renderer",
                            || add_cursor_at_midpoint(app),
                        );
                        toolbar_action_enabled(
                            ui,
                            "Add marker",
                            waveform_coordinates,
                            "Exact source markers are available in the waveform renderer",
                            || add_marker_at_midpoint(app),
                        );
                        toolbar_action(ui, "Measure", || {
                            open_dock(app, VisualizationDock::Measurement);
                        });
                        toolbar_action(ui, "Annotate", || {
                            open_dock(app, VisualizationDock::Annotation);
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            toolbar_action(ui, "Export…", || export_document(app));
                            toolbar_action(ui, "Axes & display…", || {
                                app.state.workbench.visualization_studio.section =
                                    VisualizationSection::Axes;
                            });
                            let fit_blocker = fit_block_reason(&app.state);
                            let fit =
                                ui.add_enabled(fit_blocker.is_none(), egui::Button::new("Fit"));
                            let fit = if let Some(reason) = fit_blocker {
                                fit.on_disabled_hover_text(reason)
                            } else {
                                fit
                            };
                            if fit.clicked() {
                                fit_active_view(app);
                            }
                            if ui
                                .add_enabled(waveform_coordinates, egui::Button::new("+"))
                                .on_hover_text("Zoom in")
                                .clicked()
                            {
                                zoom_active(app, 1.25);
                            }
                            ui.label(
                                RichText::new(format!(
                                    "{}%",
                                    (app.state.workbench.visualization_studio.zoom * 100.0).round()
                                        as u32
                                ))
                                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.text_dim),
                            );
                            if ui
                                .add_enabled(waveform_coordinates, egui::Button::new("−"))
                                .on_hover_text("Zoom out")
                                .clicked()
                            {
                                zoom_active(app, 0.8);
                            }
                        });
                    });
                });
        });
    paint_bottom_rule(ui, bar.response.rect, t.color.border_strong);
}

pub(super) fn toolbar_action(ui: &mut Ui, label: &'static str, action: impl FnOnce()) {
    if Button::new(label).ghost().show(ui).clicked() {
        action();
    }
}

pub(super) fn toolbar_action_enabled(
    ui: &mut Ui,
    label: &'static str,
    enabled: bool,
    unavailable_reason: &'static str,
    action: impl FnOnce(),
) {
    if ui
        .add_enabled(enabled, egui::Button::new(label))
        .on_disabled_hover_text(unavailable_reason)
        .clicked()
    {
        action();
    }
}

pub(super) fn viewer_library(ui: &mut Ui, app: &mut RSpiceApp) {
    panel_heading(ui, "Viewer library", &VIEWER_DOCUMENTS.len().to_string());
    let query = &mut app.state.workbench.visualization_studio.viewer_query;
    let t = Tokens::get(ui.ctx());
    Frame::NONE.inner_margin(Margin::same(8)).show(ui, |ui| {
        let response = ui.add_sized(
            [ui.available_width(), 28.0],
            egui::TextEdit::singleline(query)
                .hint_text("Filter viewers")
                .margin(Margin {
                    left: 29,
                    right: 8,
                    top: 4,
                    bottom: 4,
                })
                .desired_width(f32::INFINITY),
        );
        let icon_rect = Rect::from_center_size(
            egui::pos2(response.rect.left() + 13.5, response.rect.center().y),
            Vec2::splat(13.0),
        );
        WorkbenchIcon::Search.paint(ui.painter(), icon_rect, t.color.text_faint);
    });

    let query = query.trim().to_ascii_lowercase();
    let analysis_ids = available_analysis_ids(&app.state);
    let capabilities = ViewerCapabilities {
        analysis_ids: &analysis_ids,
        external_capabilities: &[],
    };
    let mut selected = None;
    ScrollArea::vertical()
        .id_salt("visualization.viewer-library")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for group in ViewerGroup::ALL {
                let rows: Vec<_> = VIEWER_DOCUMENTS
                    .iter()
                    .filter(|definition| definition.group == group)
                    .filter(|definition| {
                        query.is_empty()
                            || definition.title.to_ascii_lowercase().contains(&query)
                            || definition.domain.to_ascii_lowercase().contains(&query)
                    })
                    .collect();
                if rows.is_empty() {
                    continue;
                }
                viewer_group_heading(ui, group.label());
                for definition in rows {
                    let availability =
                        resolved_viewer_availability(&app.state, definition, capabilities);
                    let active = app
                        .state
                        .workbench
                        .visualization_studio
                        .selected_viewer_document
                        == definition.id;
                    let response = viewer_library_row(
                        ui,
                        definition,
                        active,
                        availability.is_ok(),
                        availability.as_ref().err().map(String::as_str),
                    );
                    if availability.is_ok() && response.clicked() {
                        selected = Some((definition.id, availability));
                    }
                }
            }
        });
    if let Some((id, availability)) = selected {
        app.state
            .workbench
            .visualization_studio
            .selected_viewer_document = id.to_owned();
        if let Ok(viewer) = availability {
            add_viewer_pane(app, id, viewer);
        }
    }
}

pub(super) fn viewer_group_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let heading = Frame::NONE
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 7,
            bottom: 5,
        })
        .show(ui, |ui| {
            ui.label(
                RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_faint),
            );
        });
    paint_top_rule(ui, heading.response.rect, t.color.border);
    paint_bottom_rule(ui, heading.response.rect, t.color.border);
}

pub(super) fn viewer_library_row(
    ui: &mut Ui,
    definition: &ViewerDocumentDefinition,
    active: bool,
    available: bool,
    reason: Option<&str>,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), 36.0),
        if available {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            available && ui.is_enabled(),
            active,
            definition.title,
        )
    });
    ui.painter().rect_filled(
        rect,
        0.0,
        if active {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            Color32::TRANSPARENT
        },
    );
    if active {
        ui.painter().vline(
            rect.left() + 1.0,
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    }
    let color = if available {
        t.color.text
    } else {
        t.color.text_faint
    };
    ui.painter().text(
        rect.left_top() + vec2(8.0, 8.0),
        egui::Align2::LEFT_TOP,
        definition.title,
        theme::sans(tokens::FS_1, FontWeight::Medium),
        color,
    );
    let detail_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let detail = elide_text_to_width(
        ui,
        if available {
            definition.domain
        } else {
            reason.unwrap_or("Unavailable")
        },
        &detail_font,
        (rect.width() - 16.0).max(1.0),
    );
    ui.painter().text(
        rect.left_bottom() + vec2(8.0, -5.0),
        egui::Align2::LEFT_BOTTOM,
        detail,
        detail_font,
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect);
    if let Some(reason) = reason {
        response.on_hover_text(reason)
    } else {
        response
    }
}

pub(super) fn elide_text_to_width(
    ui: &Ui,
    text: &str,
    font: &egui::FontId,
    maximum_width: f32,
) -> String {
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= maximum_width
    {
        return text.to_owned();
    }
    let mut candidate = text.to_owned();
    while candidate.pop().is_some() {
        let elided = format!("{}…", candidate.trim_end());
        if ui
            .painter()
            .layout_no_wrap(elided.clone(), font.clone(), Color32::WHITE)
            .size()
            .x
            <= maximum_width
        {
            return elided;
        }
    }
    "…".to_owned()
}

pub(super) fn resolved_viewer_availability(
    state: &AppState,
    definition: &ViewerDocumentDefinition,
    capabilities: ViewerCapabilities<'_>,
) -> Result<ResultViewer, String> {
    // Answered first, because it answers a different question. Everything below
    // says the retained dataset cannot feed this view, and names what would:
    // run that analysis, retain that capability, and the row lights up. A view
    // no sheet draws never lights up for any dataset, so telling the reader to
    // go and produce photonics data would send them after something that could
    // not help. The manifest's own release scope says which it is — planned,
    // preview, deferred, or owned by an external producer.
    let Some(viewer) = ResultViewer::from_viewer_document_id(definition.id) else {
        return Err(definition.release.unavailable_reason().to_owned());
    };
    if definition.release != crate::results::viewer_catalog::ViewerReleaseClass::ReleaseTarget {
        return Err(definition.release.unavailable_reason().to_owned());
    }
    match viewer_compatibility(definition.id, capabilities) {
        ViewerCompatibility::Compatible => {}
        ViewerCompatibility::MissingAnalysis {
            accepted_analysis_ids,
        } => {
            return Err(format!(
                "Requires {} analysis data",
                accepted_analysis_ids.join(" / ")
            ));
        }
        ViewerCompatibility::MissingExternalCapability { capability_id } => {
            return Err(format!("Requires {capability_id} result capability"));
        }
        ViewerCompatibility::UnknownDocument => {
            return Err("Viewer identity is not registered".to_owned());
        }
    }
    if !result_document::viewer_is_available(state, viewer) {
        return Err(result_document::viewer_unavailability_reason(state, viewer)
            .unwrap_or("The retained result does not satisfy this viewer contract")
            .to_owned());
    }
    Ok(viewer)
}
