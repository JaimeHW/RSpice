//! The hardcopy studio's preview column: the page desk, its toolbar, and the
//! facts strip under it.
//!
//! Every page shown is the renderer's own raster of the planned page, cached
//! in one of two rotating texture slots by its digest, so the desk shows
//! exactly what export and print will produce.

use super::*;

pub(super) fn preview_column(ui: &mut Ui, draft: &mut HardcopyDialogState, height: f32) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(vec2(width, height));
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        // The three bands divide the column's exact assigned height between
        // them. The page desk takes what the toolbar and the estimate do not,
        // so the preview grows with the surface instead of staying a thumbnail.
        let pages = draft
            .preview_plan
            .as_ref()
            .map_or(0, |plan| plan.pagination().pages().len() as u32);
        let mut view = PreviewView {
            page: draft.preview_page,
            zoom: draft.preview_zoom_percent,
        };
        let toolbar_height = preview_toolbar_height(ui, &view, pages, width);
        let facts = preview_facts(draft);
        let facts_height = facts.len() as f32 * FACT_ROW_HEIGHT + FACT_STRIP_INSET;
        let desk_height = (height - facts_height - toolbar_height).max(160.0);
        preview_desk(ui, draft, width, desk_height);
        preview_toolbar(ui, &mut view, pages, width);
        if view.page != draft.preview_page {
            draft.preview_page = view.page;
            draft.invalidate_preview_raster();
        }
        draft.preview_zoom_percent = view.zoom;
        preview_facts_strip(ui, &facts, width, facts_height);
    });
}

/// The band the toolbar needs, measured rather than assumed.
///
/// Its controls are the design system's and its row wraps when they cannot
/// share the column, so the number of rows is a fact about this width and this
/// control set — not about whether the shell is in touch mode. A sizing pass
/// lays the same row out invisibly and reports what it took, which is exact at
/// every width; reserving a second row for every touch surface cost the page
/// desk a whole row of page wherever one row was enough.
pub(super) fn preview_toolbar_height(
    ui: &mut Ui,
    view: &PreviewView,
    pages: u32,
    width: f32,
) -> f32 {
    let mut probe = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(Rect::from_min_size(
                ui.max_rect().min,
                vec2(width, ui.max_rect().height().max(width)),
            ))
            .layout(Layout::top_down(Align::Min))
            .sizing_pass()
            .invisible(),
    );
    preview_toolbar(&mut probe, &mut { *view }, pages, width)
}

/// What the preview toolbar edits. The toolbar is laid out twice — once
/// invisibly to measure its band — so it edits a copy the measuring pass can
/// throw away rather than the draft itself.
#[derive(Clone, Copy)]
pub(super) struct PreviewView {
    pub(super) page: u32,
    pub(super) zoom: u16,
}

fn preview_desk(ui: &mut Ui, draft: &mut HardcopyDialogState, width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_inset).show(ui, |ui| {
        ui.set_min_size(vec2(width, height));
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        let Some(count) = draft
            .preview_plan
            .as_ref()
            .map(|plan| plan.pagination().pages().len().clamp(1, 2))
        else {
            ui.allocate_ui_with_layout(
                vec2(width, height),
                Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.add(
                        egui::Label::new("Resolve the configuration to render an exact page.")
                            .wrap(),
                    );
                },
            );
            return;
        };
        // The card is sized from the page, not the other way round: the white
        // area is the sheet plus its margin, so the desk never shows a
        // letterboxed page inside a card of some other proportion.
        let aspect = draft.preview.as_ref().map_or_else(
            || match draft.orientation {
                Orientation::Portrait => 1.0 / std::f32::consts::SQRT_2,
                _ => std::f32::consts::SQRT_2,
            },
            |preview| preview.width() as f32 / preview.height() as f32,
        );
        let columns = count as f32;
        let max_card = vec2(
            ((width - 40.0) - 16.0 * (columns - 1.0)) / columns,
            height - 40.0,
        );
        let image = fit_aspect(
            vec2((max_card.x - 20.0).max(40.0), (max_card.y - 38.0).max(40.0)),
            aspect,
        );
        let card = vec2(image.x + 20.0, image.y + 38.0);
        let group_width = card.x * columns + 16.0 * (columns - 1.0);
        let mut selected_adjacent = None;
        ui.allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
            ui.set_width(width);
            ui.add_space(((height - card.y) * 0.5).max(0.0));
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 16.0;
                ui.add_space(((width - group_width) * 0.5).max(0.0));
                let pages = [draft.preview.as_ref(), draft.preview_adjacent.as_ref()]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                for slot in 0..count {
                    if let Some(preview) = pages.get(slot).copied() {
                        let selected =
                            preview.page_number() == draft.preview_page.saturating_add(1);
                        if preview_card(
                            ui,
                            preview,
                            selected,
                            draft.preview_zoom_percent,
                            slot,
                            card,
                        ) && !selected
                        {
                            selected_adjacent = Some(preview.page_number().saturating_sub(1));
                        }
                    } else {
                        preview_placeholder(ui, slot == 0, card);
                    }
                }
            });
        });
        if let Some(page) = selected_adjacent {
            draft.preview_page = page;
            draft.invalidate_preview_raster();
        }
    });
}

pub(super) fn preview_toolbar(ui: &mut Ui, view: &mut PreviewView, count: u32, width: f32) -> f32 {
    let t = Tokens::get(ui.ctx());
    let toolbar = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_width((width - 20.0).max(0.0));
            if count == 0 {
                // Sized rather than justified: a justified row claims the whole
                // rectangle the column had left, and the band is exactly what
                // this function reports back to it.
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), t.metrics.ctl_h),
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.monospace("No valid page");
                    },
                );
                return;
            }
            // Wrapped, not fixed to one row: a column narrow enough to need a
            // second row would otherwise run its last control off the right
            // edge, and the band above reserves whichever count this produces.
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
                if IconButton::new(Icon::ChevronLeft)
                    .tooltip("Previous page")
                    .enabled(view.page > 0)
                    .show(ui)
                    .clicked()
                {
                    view.page -= 1;
                }
                ui.monospace(format!("Page {} / {}", view.page.saturating_add(1), count));
                if IconButton::new(Icon::ChevronRight)
                    .tooltip("Next page")
                    .enabled(view.page + 1 < count)
                    .show(ui)
                    .clicked()
                {
                    view.page += 1;
                }
                // Painted rather than `ui.separator()`, which takes the band's
                // whole height and would set the wrapped row's height from a
                // rule instead of from the controls beside it.
                let (rule, _) = ui.allocate_exact_size(
                    vec2(1.0, Tokens::get(ui.ctx()).metrics.ctl_h),
                    Sense::hover(),
                );
                ui.painter().vline(
                    rule.center().x,
                    rule.y_range().shrink(5.0),
                    Stroke::new(1.0, t.color.border),
                );
                if IconButton::new(Icon::ZoomOut)
                    .tooltip("Zoom out")
                    .show(ui)
                    .clicked()
                {
                    view.zoom = view.zoom.saturating_sub(10).max(25);
                }
                ui.monospace(format!("{}%", view.zoom));
                if IconButton::new(Icon::ZoomIn)
                    .tooltip("Zoom in")
                    .show(ui)
                    .clicked()
                {
                    view.zoom = view.zoom.saturating_add(10).min(200);
                }
                // The fit control is set as a glyph, not a word. It is the
                // fifth member of a row of icon buttons and the same action the
                // rest of the shell spells with this icon; as a labelled button
                // it was also the one control the column could not pay for at
                // the tablet width, and the row wrapped to strand it alone on a
                // second row beneath the other six.
                if IconButton::new(Icon::ZoomFit)
                    .tooltip("Fit page")
                    .show(ui)
                    .clicked()
                {
                    view.zoom = 85;
                }
            });
        });
    ui.painter().hline(
        toolbar.response.rect.x_range(),
        toolbar.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    toolbar.response.rect.height()
}

/// What this configuration will actually produce, as facts rather than as the
/// settings that produced them.
fn preview_facts(draft: &HardcopyDialogState) -> Vec<(&'static str, String)> {
    let pages = draft.preview_plan.as_ref().map_or_else(
        || "no valid page".to_owned(),
        |plan| {
            let pagination = plan.pagination();
            let count = pagination.pages().len();
            let arrangement = if pagination.sections().is_empty() {
                format!(" · {} × {} tiles", pagination.columns(), pagination.rows())
            } else {
                " · section-aware".to_owned()
            };
            format!(
                "{count} page{}{arrangement}",
                if count == 1 { "" } else { "s" }
            )
        },
    );
    vec![
        ("Pages", pages),
        (
            "Media",
            format!(
                "{} · {}",
                short_paper_label(&draft.paper),
                orientation_word(draft.orientation)
            ),
        ),
        ("Target", output_summary(draft.format)),
        ("Scale", scale_label(scale_key(draft.scale)).to_owned()),
        ("Color", color_mapping_label(draft.color_mapping).to_owned()),
    ]
}

fn preview_facts_strip(ui: &mut Ui, facts: &[(&'static str, String)], width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    let strip = Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            let content_width = (width - 24.0).max(0.0);
            ui.set_width(content_width);
            ui.set_min_height((height - FACT_STRIP_INSET).max(0.0));
            ui.spacing_mut().item_spacing = vec2(10.0, 0.0);
            let label_width = 74.0_f32.min(content_width * 0.4);
            for (label, value) in facts {
                // Each row takes exactly the height the strip reserved for it.
                ui.allocate_ui_with_layout(
                    vec2(content_width, FACT_ROW_HEIGHT),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.allocate_ui_with_layout(
                            vec2(label_width, FACT_ROW_HEIGHT),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.label(
                                    egui::RichText::new(label.to_uppercase())
                                        .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                                        .color(t.color.text_faint),
                                );
                            },
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(value)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text),
                            )
                            .truncate(),
                        );
                    },
                );
            }
        });
    ui.painter().hline(
        strip.response.rect.x_range(),
        strip.response.rect.top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn preview_card(
    ui: &mut Ui,
    preview: &HardcopyPreviewPage,
    selected: bool,
    zoom_percent: u16,
    texture_slot: usize,
    size: Vec2,
) -> bool {
    let texture = preview_texture(ui.ctx(), preview, texture_slot);
    let t = Tokens::get(ui.ctx());
    let inner_width = (size.x - 20.0).max(20.0);
    let image_box = vec2(inner_width, (size.y - 38.0).max(20.0));
    let card = Frame::NONE
        .fill(egui::Color32::from_rgb(245, 244, 239))
        .stroke(Stroke::new(1.0, egui::Color32::from_rgb(155, 155, 150)))
        .shadow(egui::epaint::Shadow {
            offset: [0, 4],
            blur: 12,
            spread: 0,
            color: egui::Color32::from_black_alpha(71),
        })
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(inner_width);
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let (rect, _) = ui.allocate_exact_size(image_box, Sense::hover());
                let aspect = preview.width() as f32 / preview.height() as f32;
                let fitted = fit_aspect(rect.size(), aspect);
                let image_rect = egui::Rect::from_center_size(
                    rect.center(),
                    fitted * (zoom_percent as f32 / 85.0),
                );
                ui.painter().with_clip_rect(rect).image(
                    texture.id(),
                    image_rect,
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
                ui.add_sized(
                    vec2(inner_width, 18.0),
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "Page {} · {}",
                            preview.page_number(),
                            preview.coordinate()
                        ))
                        .monospace()
                        .size(tokens::FS_0)
                        .color(egui::Color32::from_rgb(44, 44, 42)),
                    )
                    .truncate(),
                );
            });
        });
    if selected {
        ui.painter().rect_stroke(
            card.response.rect,
            0.0,
            Stroke::new(2.0, t.color.accent),
            egui::StrokeKind::Outside,
        );
    }
    let response = card.response.interact(Sense::click());
    let label = format!("Page {} · {}", preview.page_number(), preview.coordinate());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            selected,
            label.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, response.rect);
    response.clicked()
}

fn preview_placeholder(ui: &mut Ui, selected: bool, size: Vec2) {
    let t = Tokens::get(ui.ctx());
    let inner_width = (size.x - 20.0).max(20.0);
    let card = Frame::NONE
        .fill(egui::Color32::from_rgb(245, 244, 239))
        .stroke(Stroke::new(1.0, egui::Color32::from_rgb(155, 155, 150)))
        .shadow(egui::epaint::Shadow {
            offset: [0, 4],
            blur: 12,
            spread: 0,
            color: egui::Color32::from_black_alpha(71),
        })
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(inner_width);
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(inner_width, (size.y - 38.0).max(20.0)),
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.spinner();
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(inner_width, 18.0),
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.label(
                            egui::RichText::new("Rendering…")
                                .monospace()
                                .size(tokens::FS_0)
                                .color(egui::Color32::from_rgb(44, 44, 42)),
                        );
                    },
                );
            });
        });
    if selected {
        ui.painter().rect_stroke(
            card.response.rect,
            0.0,
            Stroke::new(2.0, t.color.accent),
            egui::StrokeKind::Outside,
        );
    }
}

fn preview_texture(ctx: &Context, preview: &HardcopyPreviewPage, slot: usize) -> TextureHandle {
    let id = preview_texture_slot_id(slot);
    if let Some(cache) = ctx.data(|data| data.get_temp::<PreviewTexture>(id))
        && cache.digest == preview.digest()
    {
        return cache.handle;
    }
    let image = ColorImage::from_rgba_unmultiplied(
        [preview.width() as usize, preview.height() as usize],
        preview.rgba(),
    );
    let handle = ctx.load_texture(
        "RSpice exact hardcopy preview",
        image,
        TextureOptions::LINEAR,
    );
    ctx.data_mut(|data| {
        data.insert_temp(
            id,
            PreviewTexture {
                digest: preview.digest(),
                handle: handle.clone(),
            },
        );
    });
    handle
}

pub(super) fn preview_texture_slot_id(slot: usize) -> egui::Id {
    egui::Id::new(("rspice-hardcopy-preview-texture", slot.min(1)))
}

fn fit_aspect(max: egui::Vec2, aspect: f32) -> egui::Vec2 {
    if max.x / max.y > aspect {
        vec2(max.y * aspect, max.y)
    } else {
        vec2(max.x, max.x / aspect)
    }
}
