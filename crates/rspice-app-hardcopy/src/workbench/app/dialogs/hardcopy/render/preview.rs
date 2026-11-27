//! The Print and Export dialogs' preview: the page on its desk, and one line
//! under it saying which page it is.
//!
//! Every page shown is the renderer's own raster of the planned page, cached
//! in one of two rotating texture slots by its digest, so the desk shows
//! exactly what export and print will produce. It is rendered at the size it
//! is shown, pixel for pixel, so no line is lost to resampling.

use super::*;
use rspice_hardcopy::render::PREVIEW_DPI;

/// The line under the page: its number and the zoom.
const PAGE_BAR_INSET_Y: i8 = 6;

pub(super) fn preview_column(ui: &mut Ui, draft: &mut HardcopyDialogState, height: f32) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    Frame::NONE.fill(t.color.bg_inset).show(ui, |ui| {
        ui.set_min_size(vec2(width, height));
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        let bar_height = t.metrics.ctl_h + 2.0 * f32::from(PAGE_BAR_INSET_Y);
        let desk_height = (height - bar_height).max(160.0);
        preview_desk(ui, draft, width, desk_height);
        page_bar(ui, draft, width, bar_height);
    });
}

fn preview_desk(ui: &mut Ui, draft: &mut HardcopyDialogState, width: f32, height: f32) {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let Some(paper) = draft.preview_plan.as_ref().map(|plan| {
        let pages = plan.pagination().pages();
        pages
            .get(draft.preview_page as usize)
            .or_else(|| pages.first())
            .map(|page| {
                let (width, height) = page.geometry().physical_size();
                (width.micrometres(), height.micrometres())
            })
    }) else {
        ui.allocate_ui_with_layout(
            vec2(width, height),
            Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                ui.add(egui::Label::new("Finish the setup to see the page.").wrap());
            },
        );
        return;
    };
    // The page is sized from the planned paper, so it does not change shape
    // when its raster arrives.
    let aspect = paper.map_or_else(
        || match draft.orientation {
            Orientation::Portrait => 1.0 / std::f32::consts::SQRT_2,
            _ => std::f32::consts::SQRT_2,
        },
        |(width, height)| width as f32 / height.max(1) as f32,
    );
    let image = fit_aspect(
        vec2((width - 48.0).max(40.0), (height - 48.0).max(40.0)),
        aspect,
    );
    // The page is rendered at the resolution it is shown at, and again when
    // the zoom changes: a raster drawn smaller than it was rendered drops
    // the thin lines that fall between the pixels it samples, and a trace
    // then reads as dashes.
    if let Some((paper_width, _)) = paper {
        let shown =
            image.x * f32::from(draft.preview_zoom_percent) / 85.0 * ui.ctx().pixels_per_point();
        draft.preview_dpi = Some(shown_dpi(shown, paper_width));
    }
    let (desk, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let page = draft.preview.as_deref();
    preview_page(
        ui,
        desk,
        page,
        (draft.preview_zoom_percent, draft.preview_dpi),
        image,
    );
}

/// The line under the page: which page of how many, the arrows that turn
/// them where there are several, and the zoom.
fn page_bar(ui: &mut Ui, draft: &mut HardcopyDialogState, width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    let Some(plan) = draft.preview_plan.clone() else {
        ui.add_space(height);
        return;
    };
    let pages = plan.pagination().pages();
    let count = u32::try_from(pages.len()).unwrap_or(u32::MAX);
    let total = plan.pagination().total_page_count();
    let current = pages
        .get(draft.preview_page as usize)
        .map_or(1, |page| page.number());
    let mut page = draft.preview_page;
    let mut zoom = draft.preview_zoom_percent;
    let bar = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, PAGE_BAR_INSET_Y))
        .show(ui, |ui| {
            ui.set_width((width - 20.0).max(0.0));
            ui.set_height(t.metrics.ctl_h);
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                if count > 1
                    && IconButton::new(Icon::ChevronLeft)
                        .tooltip("Previous page")
                        .enabled(page > 0)
                        .show(ui)
                        .clicked()
                {
                    page -= 1;
                }
                let words = if total == 1 {
                    "1 page".to_owned()
                } else {
                    format!("Page {current} of {total}")
                };
                ui.label(
                    egui::RichText::new(words)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text),
                );
                if count > 1
                    && IconButton::new(Icon::ChevronRight)
                        .tooltip("Next page")
                        .enabled(page + 1 < count)
                        .show(ui)
                        .clicked()
                {
                    page += 1;
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    if IconButton::new(Icon::ZoomFit)
                        .tooltip("Fit page")
                        .show(ui)
                        .clicked()
                    {
                        zoom = 85;
                    }
                    if IconButton::new(Icon::ZoomIn)
                        .tooltip("Zoom in")
                        .show(ui)
                        .clicked()
                    {
                        zoom = zoom.saturating_add(10).min(200);
                    }
                    ui.label(
                        egui::RichText::new(format!("{zoom}%"))
                            .font(theme::field_mono())
                            .color(t.color.text_dim),
                    );
                    if IconButton::new(Icon::ZoomOut)
                        .tooltip("Zoom out")
                        .show(ui)
                        .clicked()
                    {
                        zoom = zoom.saturating_sub(10).max(25);
                    }
                });
            });
        });
    ui.painter().hline(
        bar.response.rect.x_range(),
        bar.response.rect.top(),
        Stroke::new(1.0, t.color.border),
    );
    if page != draft.preview_page {
        draft.preview_page = page;
        draft.invalidate_preview_raster();
    }
    draft.preview_zoom_percent = zoom;
}

/// The whole pixels per inch at which paper `paper_um` wide fills `pixels`
/// of the screen without passing it.
fn shown_dpi(pixels: f32, paper_um: u64) -> u16 {
    let dpi = (f64::from(pixels) * 25_400.0 / paper_um.max(1) as f64).floor();
    let (least, most) = (*PREVIEW_DPI.start(), *PREVIEW_DPI.end());
    dpi.clamp(f64::from(least), f64::from(most)) as u16
}

/// The page, white on the desk under a thin neutral edge and a soft shadow,
/// or a blank sheet of its size while it renders. `fitted` is the size the
/// page takes at the fitting zoom; `zoom` is the zoom percent and the
/// resolution the desk shows the page at at that zoom.
fn preview_page(
    ui: &mut Ui,
    desk: Rect,
    preview: Option<&HardcopyPreviewPage>,
    (zoom_percent, shown_dpi): (u16, Option<u16>),
    fitted: Vec2,
) {
    let pixels_per_point = ui.ctx().pixels_per_point();
    // A page rendered for this view is drawn pixel for pixel, on whole
    // pixels; one rendered for another zoom is stretched to its place until
    // its own arrives.
    let drawn = match preview {
        Some(preview) if shown_dpi == Some(preview.dpi()) => {
            vec2(preview.width() as f32, preview.height() as f32) / pixels_per_point
        }
        _ => fitted * (f32::from(zoom_percent) / 85.0),
    };
    let corner = desk.center() - drawn * 0.5;
    let whole = |points: f32| (points * pixels_per_point).round() / pixels_per_point;
    let sheet = Rect::from_min_size(egui::pos2(whole(corner.x), whole(corner.y)), drawn);
    let painter = ui.painter().with_clip_rect(desk);
    painter.add(
        egui::epaint::Shadow {
            offset: [0, 3],
            blur: 14,
            spread: 0,
            color: egui::Color32::from_black_alpha(90),
        }
        .as_shape(sheet, 0),
    );
    painter.rect_filled(sheet, 0.0, egui::Color32::WHITE);
    if let Some(preview) = preview {
        let texture = preview_texture(ui.ctx(), preview, 0);
        painter.image(
            texture.id(),
            sheet,
            Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    } else {
        let side = ui.spacing().interact_size.y;
        egui::Spinner::new()
            .color(egui::Color32::from_gray(150))
            .paint_at(
                ui,
                Rect::from_center_size(sheet.center(), Vec2::splat(side)),
            );
    }
    painter.rect_stroke(
        sheet,
        0.0,
        Stroke::new(1.0, egui::Color32::from_gray(120)),
        egui::StrokeKind::Outside,
    );
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
