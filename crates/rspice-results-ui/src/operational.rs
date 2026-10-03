//! Operational status presentation over host-classified retained evidence.

use egui::{Ui, WidgetInfo, WidgetType};
use rspice_ui_kit::{
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultOperationalCategory {
    Normal,
    Empty,
    Loading,
    Partial,
    Warning,
    Error,
    Recovery,
}

/// Canonical viewer-state vocabulary from `result-data-contract.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultOperationalState {
    Complete,
    NoProject,
    NoDataset,
    Loading,
    Streaming,
    Partial,
    Stale,
    Failed,
    Corrupted,
    Unsupported,
    Offline,
    StorageDenied,
    LowMemory,
    RendererLoss,
    InterruptedOperation,
    Recovered,
}

impl ResultOperationalState {
    pub const ALL: [Self; 16] = [
        Self::Complete,
        Self::NoProject,
        Self::NoDataset,
        Self::Loading,
        Self::Streaming,
        Self::Partial,
        Self::Stale,
        Self::Failed,
        Self::Corrupted,
        Self::Unsupported,
        Self::Offline,
        Self::StorageDenied,
        Self::LowMemory,
        Self::RendererLoss,
        Self::InterruptedOperation,
        Self::Recovered,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::NoProject => "no-project",
            Self::NoDataset => "no-dataset",
            Self::Loading => "loading",
            Self::Streaming => "streaming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Failed => "failed",
            Self::Corrupted => "corrupted",
            Self::Unsupported => "unsupported",
            Self::Offline => "offline",
            Self::StorageDenied => "storage-denied",
            Self::LowMemory => "low-memory",
            Self::RendererLoss => "renderer-loss",
            Self::InterruptedOperation => "interrupted-operation",
            Self::Recovered => "recovered",
        }
    }

    pub const fn category(self) -> ResultOperationalCategory {
        match self {
            Self::Complete => ResultOperationalCategory::Normal,
            Self::NoProject | Self::NoDataset => ResultOperationalCategory::Empty,
            Self::Loading | Self::Streaming => ResultOperationalCategory::Loading,
            Self::Partial => ResultOperationalCategory::Partial,
            Self::Stale
            | Self::Unsupported
            | Self::Offline
            | Self::LowMemory
            | Self::InterruptedOperation => ResultOperationalCategory::Warning,
            Self::Failed | Self::Corrupted | Self::StorageDenied | Self::RendererLoss => {
                ResultOperationalCategory::Error
            }
            Self::Recovered => ResultOperationalCategory::Recovery,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Complete => "Complete",
            Self::NoProject => "No project",
            Self::NoDataset => "No compatible dataset",
            Self::Loading => "Loading metadata",
            Self::Streaming => "Streaming",
            Self::Partial => "Partial result",
            Self::Stale => "Stale source",
            Self::Failed => "Result operation failed",
            Self::Corrupted => "Integrity failure",
            Self::Unsupported => "Unsupported viewer contract",
            Self::Offline => "Offline",
            Self::StorageDenied => "Storage unavailable",
            Self::LowMemory => "Memory pressure",
            Self::RendererLoss => "Renderer unavailable",
            Self::InterruptedOperation => "Operation interrupted",
            Self::Recovered => "Recovered",
        }
    }

    pub const fn message(self) -> &'static str {
        match self {
            Self::Complete => {
                "All declared source scopes are available and exact-data consumers resolve normally."
            }
            Self::NoProject => "No project owns a result document or dataset binding.",
            Self::NoDataset => {
                "The viewer document exists but has no compatible immutable dataset binding."
            }
            Self::Loading => {
                "Manifest and schema metadata are loading; numeric access has not been claimed."
            }
            Self::Streaming => {
                "Verified chunks are arriving progressively; unavailable ranges remain explicit."
            }
            Self::Partial => {
                "Only the disclosed verified scope is available; absent scope is not inferred or interpolated."
            }
            Self::Stale => "A dependency changed after this immutable result revision was created.",
            Self::Failed => {
                "The requested viewer operation failed without replacing the prior valid state."
            }
            Self::Corrupted => {
                "Dataset bytes or structure failed verification and numeric access is quarantined."
            }
            Self::Unsupported => {
                "The required analysis, producer, transform, or representation is unavailable."
            }
            Self::Offline => {
                "Remote-only source chunks or services are unavailable; cached verified scope remains identified."
            }
            Self::StorageDenied => {
                "Quota, permission, or durable storage policy blocked the requested operation."
            }
            Self::LowMemory => {
                "Rendering detail was reduced under policy; exact engineering queries remain source-backed."
            }
            Self::RendererLoss => {
                "The visual renderer was lost; source data and exact structured access remain intact."
            }
            Self::InterruptedOperation => {
                "An import, comparison, derivation, or publication operation stopped before commit."
            }
            Self::Recovered => {
                "The viewer was reconstructed from verified document and dataset state after a recorded failure."
            }
        }
    }

    pub const fn recovery(self) -> &'static str {
        match self {
            Self::Complete => "No recovery is required.",
            Self::NoProject => {
                "Open or create a project, then return to the preserved viewer route."
            }
            Self::NoDataset => {
                "Bind a compatible dataset or choose another viewer without fabricating values."
            }
            Self::Loading => "Cancel or retry while retaining the document and source identities.",
            Self::Streaming => "Pause, cancel, or resume from the last verified chunk.",
            Self::Partial => "Continue acquisition or inspect the exact available-scope table.",
            Self::Stale => {
                "Keep historical review, compare revisions, or run again from a reviewed current plan."
            }
            Self::Failed => "Inspect diagnostics and retry the exact failed boundary.",
            Self::Corrupted => {
                "Verify another retained copy, restore from a trusted artifact, or keep quarantined diagnostics."
            }
            Self::Unsupported => {
                "Install or select an exact compatible capability; no fallback viewer is substituted."
            }
            Self::Offline => {
                "Reconnect or continue with the exact cached scope without implying completeness."
            }
            Self::StorageDenied => {
                "Choose an authorized destination, free scoped storage, or cancel without source mutation."
            }
            Self::LowMemory => {
                "Release cached views, reduce visible scope, or retry after pressure clears."
            }
            Self::RendererLoss => {
                "Restart the renderer and restore viewport, selection, and presentation from the document revision."
            }
            Self::InterruptedOperation => {
                "Resume from the retained checkpoint or remove the incomplete candidate."
            }
            Self::Recovered => "Review the recovery receipt before continuing or publishing.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultOperationalStatus {
    pub state: ResultOperationalState,
    pub detail: Option<String>,
    pub blocks_visuals: bool,
    pub dismissible: bool,
}

impl ResultOperationalStatus {
    pub fn canonical(state: ResultOperationalState, blocks_visuals: bool) -> Self {
        Self {
            state,
            detail: None,
            blocks_visuals,
            dismissible: false,
        }
    }
}

/// Display facts for a failure that the host has verified is still on screen.
pub struct FailureSiteControl<'a> {
    pub named: usize,
    pub elided: usize,
    pub headline: &'a str,
    pub marked: bool,
}

/// Presentation outcome; the host applies dismiss and highlight requests.
#[derive(Default)]
pub struct OperationalResponse {
    pub blocks_visuals: bool,
    pub dismiss: bool,
    pub highlight: bool,
}

/// Query failure-site display facts only when a card will be shown.
pub fn show<'a>(
    ui: &mut Ui,
    status: &ResultOperationalStatus,
    failure_site: impl FnOnce() -> Option<FailureSiteControl<'a>>,
) -> OperationalResponse {
    if status.state == ResultOperationalState::Complete {
        return OperationalResponse::default();
    }
    // An empty well is not an alarm. It reads the way an empty schematic
    // sheet does: guidance set on the canvas, with no card behind it.
    if status.blocks_visuals && status.state.category() == ResultOperationalCategory::Empty {
        let response = show_empty_hint(ui, status);
        announce_status(ui, &response, status);
        return OperationalResponse {
            blocks_visuals: true,
            ..Default::default()
        };
    }
    let t = Tokens::get(ui.ctx());
    let accent = match status.state.category() {
        ResultOperationalCategory::Normal | ResultOperationalCategory::Recovery => t.color.ok,
        ResultOperationalCategory::Empty | ResultOperationalCategory::Loading => t.color.info,
        ResultOperationalCategory::Partial | ResultOperationalCategory::Warning => t.color.warn,
        ResultOperationalCategory::Error => t.color.err,
    };
    let offer = failure_site();
    let card = OperationalCard {
        status,
        accent,
        offer: offer.as_ref(),
        floating: status.blocks_visuals,
    };
    let mut actions = OperationalResponse {
        blocks_visuals: status.blocks_visuals,
        ..Default::default()
    };
    let response = if status.blocks_visuals {
        card.show_centered(ui, &mut actions)
    } else {
        card.show(ui, &mut actions)
    };
    announce_status(ui, &response, status);
    actions
}

/// Name the state to assistive technology on whatever response shows it.
fn announce_status(ui: &Ui, response: &egui::Response, status: &ResultOperationalStatus) {
    let accessible = format!(
        "{} status, {}: {} {}",
        status.state.id(),
        status.state.label(),
        status.detail.as_deref().unwrap_or(status.state.message()),
        status.state.recovery()
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, accessible.as_str()));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(
            if status.state.category() == ResultOperationalCategory::Error {
                egui::accesskit::Role::Alert
            } else {
                egui::accesskit::Role::Status
            },
        );
        node.set_label(accessible);
    });
}

/// The card's own geometry, as `.viewer-operational-state` defines it: a
/// bounded column centred in the stage, never a strip as long as whichever
/// sentence the state happens to carry.
const OPERATIONAL_CARD_MAX_WIDTH: f32 = 560.0;
const OPERATIONAL_CARD_GUTTER: f32 = 16.0;
const OPERATIONAL_CARD_PADDING: i8 = 14;
const OPERATIONAL_CARD_ICON: f32 = 20.0;
const OPERATIONAL_CARD_ICON_GAP: f32 = 10.0;
/// What the banner form keeps clear of the bar above and the viewer below.
const OPERATIONAL_BANNER_INSET: i8 = 8;

/// The empty schematic sheet's hint, as `schematic::view::scene` paints it:
/// a medium title in dim text over regular lines in faint text.
const EMPTY_HINT_TITLE_SIZE: f32 = 15.0;
const EMPTY_HINT_LINE_SIZE: f32 = tokens::FS_1;
/// Centre-to-centre spacing of that hint's rows: title to first line, then
/// line to line.
const EMPTY_HINT_TITLE_PITCH: f32 = 24.0;
const EMPTY_HINT_LINE_PITCH: f32 = 20.0;

/// Paint an empty state the way the schematic paints an empty sheet: bare
/// centred text on the canvas, with no card, border, or glyph.
///
/// The copy wraps inside the card's bounded column, so a long sentence breaks
/// into centred lines instead of running under the well's edges. A block
/// taller than its well starts at the top, like the card.
fn show_empty_hint(ui: &Ui, status: &ResultOperationalStatus) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let well = ui.available_rect_before_wrap();
    let wrap =
        (well.width() - 2.0 * OPERATIONAL_CARD_GUTTER).clamp(1.0, OPERATIONAL_CARD_MAX_WIDTH);
    let painter = ui.painter();
    let set = |copy: &str, font: egui::FontId, color: egui::Color32| {
        let mut job = egui::text::LayoutJob::simple(copy.to_owned(), font, color, wrap);
        job.halign = egui::Align::Center;
        painter.layout_job(job)
    };

    let mut rows = vec![set(
        status.state.label(),
        theme::sans(EMPTY_HINT_TITLE_SIZE, FontWeight::Medium),
        t.color.text_dim,
    )];
    let line = theme::sans(EMPTY_HINT_LINE_SIZE, FontWeight::Regular);
    rows.extend(
        status
            .detail
            .as_deref()
            .into_iter()
            .chain([status.state.message(), status.state.recovery()])
            .map(|copy| set(copy, line.clone(), t.color.text_faint)),
    );

    // Each galley is set in one font, so its rows share one height. The pitch
    // is measured between adjacent rows' centres, which keeps unwrapped copy
    // exactly where the schematic puts its lines.
    let row_height = |galley: &egui::Galley| galley.size().y / galley.rows.len().max(1) as f32;
    let mut offsets = Vec::with_capacity(rows.len());
    let mut height = 0.0;
    for (index, galley) in rows.iter().enumerate() {
        if let Some(previous) = index.checked_sub(1).map(|previous| &rows[previous]) {
            let pitch = if index == 1 {
                EMPTY_HINT_TITLE_PITCH
            } else {
                EMPTY_HINT_LINE_PITCH
            };
            height += (pitch - 0.5 * (row_height(previous) + row_height(galley))).max(0.0);
        }
        offsets.push(height);
        height += galley.size().y;
    }

    let top = (well.center().y - 0.5 * height)
        .max(well.top() + OPERATIONAL_CARD_GUTTER)
        .round();
    let mut painted = egui::Rect::NOTHING;
    for (galley, offset) in rows.into_iter().zip(offsets) {
        let origin = egui::pos2(well.center().x, top + offset);
        painted = painted.union(galley.rect.translate(origin.to_vec2()));
        painter.galley(origin, galley, t.color.text_faint);
    }
    ui.interact(
        painted,
        ui.id().with("result-operational-empty-hint"),
        egui::Sense::hover(),
    )
}

/// The glyph that stands for a category before its sentence is read.
const fn operational_icon(category: ResultOperationalCategory) -> WorkbenchIcon {
    match category {
        ResultOperationalCategory::Error
        | ResultOperationalCategory::Warning
        | ResultOperationalCategory::Partial => WorkbenchIcon::Warning,
        ResultOperationalCategory::Loading => WorkbenchIcon::Refresh,
        ResultOperationalCategory::Recovery | ResultOperationalCategory::Normal => {
            WorkbenchIcon::Success
        }
        ResultOperationalCategory::Empty => WorkbenchIcon::Info,
    }
}

/// One operational card: the category glyph, then the state's label, detail,
/// message and recovery, then whatever this state lets the reader act on.
struct OperationalCard<'a> {
    status: &'a ResultOperationalStatus,
    accent: egui::Color32,
    offer: Option<&'a FailureSiteControl<'a>>,
    /// The card that owns the well — elevated and shadowed over the canvas —
    /// rather than the banner attached above a still-usable viewer.
    floating: bool,
}

impl OperationalCard<'_> {
    /// Draw the card across the width its caller already fixed.
    fn show(&self, ui: &mut Ui, actions: &mut OperationalResponse) -> egui::Response {
        let t = Tokens::get(ui.ctx());
        // Severity outlines the card; an empty or loading state is not an
        // alarm and keeps the ordinary strong border under its heading.
        let border = match self.status.state.category() {
            ResultOperationalCategory::Normal
            | ResultOperationalCategory::Empty
            | ResultOperationalCategory::Loading => t.color.border_strong,
            ResultOperationalCategory::Partial
            | ResultOperationalCategory::Warning
            | ResultOperationalCategory::Error
            | ResultOperationalCategory::Recovery => self.accent,
        };
        let mut frame = egui::Frame::NONE
            .fill(if self.floating {
                t.color.bg_elevated
            } else {
                t.color.bg_panel
            })
            .stroke(egui::Stroke::new(1.0, border))
            .corner_radius(t.radius)
            .inner_margin(egui::Margin::same(OPERATIONAL_CARD_PADDING));
        if self.floating {
            frame = frame.shadow(t.shadow());
        } else {
            // The banner is inset from the well it warns about, so it neither
            // butts against the bar above it nor against the viewer below.
            frame = frame.outer_margin(egui::Margin::same(OPERATIONAL_BANNER_INSET));
        }
        frame
            .show(ui, |ui| {
                // Both forms are laid out to a width their caller fixed, so
                // the copy wraps inside the card rather than setting its
                // width — a state's longest sentence is not a layout.
                let width = ui.available_width();
                ui.set_width(width);
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = OPERATIONAL_CARD_ICON_GAP;
                    let (glyph, _) = ui.allocate_exact_size(
                        egui::Vec2::splat(OPERATIONAL_CARD_ICON),
                        egui::Sense::hover(),
                    );
                    operational_icon(self.status.state.category()).paint(
                        ui.painter(),
                        glyph,
                        self.accent,
                    );
                    let copy = (width - OPERATIONAL_CARD_ICON - OPERATIONAL_CARD_ICON_GAP).max(1.0);
                    ui.allocate_ui_with_layout(
                        egui::vec2(copy, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(copy);
                            ui.spacing_mut().item_spacing.y = 3.0;
                            ui.label(
                                egui::RichText::new(self.status.state.label())
                                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                                    .color(self.accent),
                            );
                            if let Some(detail) = self.status.detail.as_deref() {
                                ui.label(
                                    egui::RichText::new(detail)
                                        .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                                        .color(t.color.text),
                                );
                            }
                            ui.label(
                                egui::RichText::new(self.status.state.message())
                                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            );
                            ui.label(
                                egui::RichText::new(self.status.state.recovery())
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_faint),
                            );
                        },
                    );
                });
                if self.offer.is_some() || self.status.dismissible {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if let Some(offer) = self.offer {
                            actions.highlight = show_failure_site_control(ui, offer);
                        }
                        if self.status.dismissible {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    actions.dismiss =
                                        rspice_ui_kit::widgets::Button::new("Dismiss")
                                            .show(ui)
                                            .on_hover_text("Dismiss this recorded runtime notice")
                                            .clicked();
                                },
                            );
                        }
                    });
                }
            })
            .response
    }

    /// Centre the card in the document well it has taken over.
    ///
    /// The card is measured at its final width first, so it lands on the
    /// well's own centre. What this replaces was a guessed 172 pt height
    /// halved against the well's left edge, which put the card somewhere
    /// different for every state and in the middle for none of them.
    fn show_centered(&self, ui: &mut Ui, actions: &mut OperationalResponse) -> egui::Response {
        let well = ui.available_rect_before_wrap();
        let width =
            (well.width() - 2.0 * OPERATIONAL_CARD_GUTTER).clamp(1.0, OPERATIONAL_CARD_MAX_WIDTH);

        let mut probe = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("result-operational-state-measure")
                .max_rect(egui::Rect::from_min_size(
                    well.min,
                    egui::vec2(width, well.height().max(OPERATIONAL_CARD_MAX_WIDTH)),
                ))
                .layout(egui::Layout::top_down(egui::Align::Min))
                .sizing_pass()
                .invisible(),
        );
        // The measuring pass presses nothing; what it reports is thrown away.
        self.show(&mut probe, &mut OperationalResponse::default());
        let height = probe.min_rect().height();

        let top = (well.center().y - height * 0.5).max(well.top() + OPERATIONAL_CARD_GUTTER);
        let mut card = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(egui::Rect::from_min_size(
                    egui::pos2((well.center().x - width * 0.5).round(), top.round()),
                    egui::vec2(width, height),
                ))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        self.show(&mut card, actions)
    }
}

/// The control that marks what the failure named, or takes the marking back.
///
/// It says how many objects it will mark before it is pressed, because "mark
/// the offending nodes" is a different proposition at four nodes than at
/// thirty — and it says how many the engine measured but did not name, so the
/// marking is not read as the complete set.
///
/// The workbench's own Button: it already marks its response disabled and
/// paints its own focus ring, so nothing is given up by drawing it in the
/// chrome every other control on this surface uses.
fn show_failure_site_control(ui: &mut Ui, offer: &FailureSiteControl<'_>) -> bool {
    // "Objects", not "nodes": a site is a node *or* a branch current, which the
    // schematic draws as a device (`ConvergenceSiteKind`). The console's anchor
    // hint has always said "objects", and this control marks the same set.
    let label = if offer.marked {
        "Clear highlighted sites".to_owned()
    } else if offer.named == 1 {
        "Highlight the 1 object this run named".to_owned()
    } else {
        format!("Highlight the {} objects this run named", offer.named)
    };
    let hint = if offer.marked {
        "Remove the marking from the drawing".to_owned()
    } else if offer.elided > 0 {
        format!(
            "{} — {} named, {} more measured but not named",
            offer.headline, offer.named, offer.elided
        )
    } else {
        format!(
            "{} — marks all {} on the drawing",
            offer.headline, offer.named
        )
    };
    let clicked = rspice_ui_kit::widgets::Button::new(&label)
        .show(ui)
        .on_hover_text(&hint)
        .clicked();
    if !offer.marked && offer.elided > 0 {
        let t = Tokens::get(ui.ctx());
        ui.label(
            egui::RichText::new(format!(
                "{} more were measured but not named.",
                offer.elided
            ))
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
        );
    }
    clicked
}

#[cfg(test)]
mod tests;
