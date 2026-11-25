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
            Self::NoDataset => "No results yet",
            Self::Loading => "Working",
            Self::Streaming => "Running",
            Self::Partial => "Partial result",
            Self::Stale => "Out of date",
            Self::Failed => "Failed",
            Self::Corrupted => "Damaged result",
            Self::Unsupported => "Not shown on this sheet",
            Self::Offline => "Offline",
            Self::StorageDenied => "Cannot store",
            Self::LowMemory => "Low memory",
            Self::RendererLoss => "Display lost",
            Self::InterruptedOperation => "Interrupted",
            Self::Recovered => "Recovered",
        }
    }

    pub const fn message(self) -> &'static str {
        match self {
            Self::Complete => "Every result of this run is here.",
            Self::NoProject => "No project is open.",
            Self::NoDataset => "This sheet has no results to show.",
            Self::Loading => "Results appear here when they are ready.",
            Self::Streaming => {
                "The simulation is still running; the sheet shows the results so far."
            }
            Self::Partial => {
                "Only part of this run was saved; the rest is neither shown nor estimated."
            }
            Self::Stale => "The design changed after this run.",
            Self::Failed => {
                "The run or the last action on this sheet failed; what the sheet showed before is kept."
            }
            Self::Corrupted => {
                "This result failed its check when it was read, so none of its values are shown."
            }
            Self::Unsupported => "This sheet cannot show the selected analysis.",
            Self::Offline => {
                "Part of this result is stored online and cannot be reached; what is stored here is shown."
            }
            Self::StorageDenied => "There is no room, or no permission, to store this.",
            Self::LowMemory => {
                "Plots are drawn with less detail to save memory; values you read and export are exact."
            }
            Self::RendererLoss => "The graphics display stopped; the results are safe.",
            Self::InterruptedOperation => "The run or operation stopped before it finished.",
            Self::Recovered => "This sheet was rebuilt from the saved project after a failure.",
        }
    }

    pub const fn recovery(self) -> &'static str {
        match self {
            Self::Complete => "Nothing to do.",
            Self::NoProject => "Open or create a project to see its results.",
            Self::NoDataset => "Run a simulation, or choose another sheet.",
            Self::Loading => "The sheet updates by itself.",
            Self::Streaming => "The sheet updates as results arrive.",
            Self::Partial => "Run again for the whole result, or read the part that is here.",
            Self::Stale => "Read it as a past result, compare it with a new run, or run again.",
            Self::Failed => "Read the message, fix the cause and try again.",
            Self::Corrupted => "Open another saved copy, or run the simulation again.",
            Self::Unsupported => "Choose another sheet, or an analysis this sheet shows.",
            Self::Offline => "Reconnect to see the rest.",
            Self::StorageDenied => {
                "Choose another place, free some space, or cancel; the result itself is unchanged."
            }
            Self::LowMemory => "Close other sheets or narrow the view to draw in full detail.",
            Self::RendererLoss => "Restart the display; the view comes back as it was.",
            Self::InterruptedOperation => "Run it again, or keep what it produced.",
            Self::Recovered => "Check it before you continue or publish.",
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

/// What the control marking a failure's named objects says.
pub(crate) fn failure_site_label(offer: &FailureSiteControl<'_>) -> String {
    // "Objects", not "nodes": a site is a node *or* a branch current, which the
    // schematic draws as a device (`ConvergenceSiteKind`). The console's anchor
    // hint has always said "objects", and this control marks the same set.
    if offer.marked {
        "Clear highlighted sites".to_owned()
    } else if offer.named == 1 {
        "Highlight the 1 object this run named".to_owned()
    } else {
        format!("Highlight the {} objects this run named", offer.named)
    }
}

pub(crate) fn failure_site_hint(offer: &FailureSiteControl<'_>) -> String {
    if offer.marked {
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
    }
}

/// Presentation outcome; the host applies dismiss and highlight requests.
#[derive(Default)]
pub struct OperationalResponse {
    pub blocks_visuals: bool,
    pub dismiss: bool,
    pub highlight: bool,
}

/// Whether `status` leaves the viewer readable and says so beside it: a
/// mark in the sheet's bar, or the pane's first row, rather than anything in
/// the well.
pub fn marks_beside_viewer(status: &ResultOperationalStatus) -> bool {
    status.state != ResultOperationalState::Complete && !status.blocks_visuals
}

/// Present `status` in the well: the card or hint of a state that takes
/// the well over, or, when nothing beside the viewer carries it
/// (`marked_beside` false), the one-line banner over a readable viewer.
/// Query failure-site display facts only when a card will be shown.
pub fn show<'a>(
    ui: &mut Ui,
    status: &ResultOperationalStatus,
    marked_beside: bool,
    failure_site: impl FnOnce() -> Option<FailureSiteControl<'a>>,
) -> OperationalResponse {
    if status.state == ResultOperationalState::Complete
        || (marked_beside && marks_beside_viewer(status))
    {
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
    let offer = failure_site();
    let card = OperationalCard {
        status,
        icon_tone: icon_tone(status.state.category(), &t.color),
        offer: offer.as_ref(),
    };
    let mut actions = OperationalResponse {
        blocks_visuals: status.blocks_visuals,
        ..Default::default()
    };
    let response = if status.blocks_visuals {
        card.show_centered(ui, &mut actions)
    } else {
        card.show_banner(ui, &mut actions)
    };
    announce_status(ui, &response, status);
    actions
}

/// A state the viewer stays readable under, as a bar carries it: the
/// category's glyph and, when `labelled`, the state's name, at the bar's
/// control height, so the well beneath keeps every point of its height for
/// the plot. Its hover reads the whole sentence; a press opens it, with
/// whatever the state offers. Query failure-site display facts only when
/// that opens.
pub fn show_mark<'a>(
    ui: &mut Ui,
    status: &ResultOperationalStatus,
    labelled: bool,
    failure_site: impl FnOnce() -> Option<FailureSiteControl<'a>>,
) -> OperationalResponse {
    let t = Tokens::get(ui.ctx());
    let category = status.state.category();
    let tone = icon_tone(category, &t.color);
    let name = labelled.then(|| {
        ui.painter().layout_no_wrap(
            status.state.label().to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Medium),
            t.color.text,
        )
    });
    let height = ui.spacing().interact_size.y;
    let width = name.as_ref().map_or(height, |name| {
        2.0 * OPERATIONAL_MARK_PADDING_X
            + OPERATIONAL_BANNER_ICON
            + OPERATIONAL_BANNER_ICON_GAP
            + name.size().x
    });
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let glyph_x = if name.is_some() {
        rect.left() + OPERATIONAL_MARK_PADDING_X + 0.5 * OPERATIONAL_BANNER_ICON
    } else {
        rect.center().x
    };
    let glyph = egui::Rect::from_center_size(
        egui::pos2(glyph_x, rect.center().y),
        egui::Vec2::splat(OPERATIONAL_BANNER_ICON),
    );
    operational_icon(category).paint(ui.painter(), glyph, tone);
    if let Some(name) = name {
        let origin = egui::pos2(
            glyph.right() + OPERATIONAL_BANNER_ICON_GAP,
            (rect.center().y - 0.5 * name.size().y).round(),
        );
        ui.painter().galley(origin, name, t.color.text);
    }
    theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_text(sentence(status));
    announce_status(ui, &response, status);
    let mut actions = OperationalResponse::default();
    egui::Popup::menu(&response).show(|ui| {
        ui.set_max_width(OPERATIONAL_MARK_POPUP_WIDTH);
        let offer = failure_site();
        OperationalCard {
            status,
            icon_tone: tone,
            offer: offer.as_ref(),
        }
        .contents(ui, &mut actions);
    });
    actions
}

/// Everything a state says, as one sentence for a hover.
pub fn sentence(status: &ResultOperationalStatus) -> String {
    [Some(status.state.label()), status.detail.as_deref()]
        .into_iter()
        .flatten()
        .chain([status.state.message(), status.state.recovery()])
        .collect::<Vec<_>>()
        .join(" · ")
}

/// Paint the glyph that stands for `status` in `rect`, in its category's
/// tone: what a tab carries after its name for the state of what it shows.
pub fn paint_status_glyph(ui: &Ui, rect: egui::Rect, status: &ResultOperationalStatus) {
    let t = Tokens::get(ui.ctx());
    let category = status.state.category();
    operational_icon(category).paint(ui.painter(), rect, icon_tone(category, &t.color));
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
/// The banner's own geometry: one line, inset at the sides as the dock's
/// pages inset their text, with its glyph at the size of a toolbar icon and
/// a gap between its parts.
const OPERATIONAL_BANNER_PADDING_X: f32 = 10.0;
const OPERATIONAL_BANNER_ICON: f32 = 14.0;
const OPERATIONAL_BANNER_ICON_GAP: f32 = 7.0;
const OPERATIONAL_BANNER_GAP: f32 = 10.0;
/// The bar mark's inset either side of its glyph and name, as a compact
/// button insets its label, and the width its popup wraps the state's copy at.
const OPERATIONAL_MARK_PADDING_X: f32 = 6.0;
const OPERATIONAL_MARK_POPUP_WIDTH: f32 = 340.0;

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

/// The colour of a card's category glyph: the warning or error tone for a
/// state the reader has to notice, neutral otherwise. The glyph is the only
/// coloured thing on the card; its title and border never are.
fn icon_tone(
    category: ResultOperationalCategory,
    palette: &rspice_ui_kit::palette::Palette,
) -> egui::Color32 {
    match category {
        ResultOperationalCategory::Partial | ResultOperationalCategory::Warning => palette.warn,
        ResultOperationalCategory::Error => palette.err,
        ResultOperationalCategory::Normal
        | ResultOperationalCategory::Recovery
        | ResultOperationalCategory::Empty
        | ResultOperationalCategory::Loading => palette.text_dim,
    }
}

/// One operational card: the category glyph, then the state's label, detail,
/// message and recovery, then whatever this state lets the reader act on.
struct OperationalCard<'a> {
    status: &'a ResultOperationalStatus,
    icon_tone: egui::Color32,
    offer: Option<&'a FailureSiteControl<'a>>,
}

impl OperationalCard<'_> {
    /// The banner over a viewer that is still read: one line of the design's
    /// row height, flush across the well under the sheet's bar, as the
    /// tracking banner stands. The glyph, the state's name, what happened and
    /// what to do, then what the state offers at the right. A boxed card
    /// inset on every side took four lines and a margin of the well for one
    /// sentence; a sentence longer than the line now ends in an ellipsis and
    /// is read whole on hover, so the strip never grows to hold it.
    fn show_banner(&self, ui: &mut Ui, actions: &mut OperationalResponse) -> egui::Response {
        let t = Tokens::get(ui.ctx());
        let control = rspice_ui_kit::widgets::compact_height(&t.metrics);
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), t.metrics.row_h.max(control)),
            egui::Sense::hover(),
        );
        ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
        let mut row = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect.shrink2(egui::vec2(OPERATIONAL_BANNER_PADDING_X, 0.0)))
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        row.set_clip_rect(rect.intersect(ui.clip_rect()));
        // A standard control in this row stands at the compact height, so
        // the row stays one line whatever the state offers.
        row.spacing_mut().interact_size.y = control;
        row.spacing_mut().item_spacing.x = OPERATIONAL_BANNER_GAP;
        // The actions are set first, from the right, so the copy knows the
        // room they leave.
        if self.status.dismissible {
            actions.dismiss = rspice_ui_kit::widgets::Button::new("Dismiss")
                .compact()
                .show(&mut row)
                .on_hover_text("Dismiss this recorded runtime notice")
                .clicked();
        }
        if let Some(offer) = self.offer {
            if let Some(note) = failure_site_note(offer) {
                row.label(
                    egui::RichText::new(note)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            }
            actions.highlight = failure_site_button(&mut row, offer, true);
        }
        row.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = OPERATIONAL_BANNER_ICON_GAP;
            let (glyph, _) = ui.allocate_exact_size(
                egui::Vec2::splat(OPERATIONAL_BANNER_ICON),
                egui::Sense::hover(),
            );
            operational_icon(self.status.state.category()).paint(
                ui.painter(),
                glyph,
                self.icon_tone,
            );
            let galley = ui
                .painter()
                .layout_job(self.banner_copy(&t, ui.available_width().max(1.0)));
            let (copy, hover) = ui.allocate_exact_size(galley.size(), egui::Sense::hover());
            let elided = galley.elided;
            ui.painter().galley(copy.min, galley, t.color.text);
            if elided {
                hover.on_hover_text(sentence(self.status));
            }
        });
        response
    }

    /// The banner's one line: the state's name in the ink a title takes,
    /// then its detail, what happened and what to do, each quieter than the
    /// last, cut with an ellipsis at `width`.
    fn banner_copy(&self, t: &Tokens, width: f32) -> egui::text::LayoutJob {
        let format = |weight, color| egui::TextFormat {
            font_id: theme::sans(tokens::FS_1, weight),
            color,
            valign: egui::Align::Center,
            ..Default::default()
        };
        let mut job = egui::text::LayoutJob::default();
        job.append(
            self.status.state.label(),
            0.0,
            format(FontWeight::SemiBold, t.color.text),
        );
        if let Some(detail) = self.status.detail.as_deref() {
            job.append(
                detail,
                OPERATIONAL_BANNER_GAP,
                format(FontWeight::Medium, t.color.text),
            );
        }
        job.append(
            self.status.state.message(),
            OPERATIONAL_BANNER_GAP,
            format(FontWeight::Regular, t.color.text_dim),
        );
        job.append(
            self.status.state.recovery(),
            OPERATIONAL_BANNER_GAP,
            format(FontWeight::Regular, t.color.text_faint),
        );
        job.wrap = egui::text::TextWrapping::truncate_at_width(width);
        job
    }

    /// Draw the card across the width its caller already fixed: elevated
    /// and shadowed over the well it has taken over.
    fn show(&self, ui: &mut Ui, actions: &mut OperationalResponse) -> egui::Response {
        let t = Tokens::get(ui.ctx());
        // Every card keeps the same strong border; the glyph alone carries
        // severity.
        egui::Frame::NONE
            .fill(t.color.bg_elevated)
            .stroke(egui::Stroke::new(1.0, t.color.border_strong))
            .corner_radius(t.radius)
            .inner_margin(egui::Margin::same(OPERATIONAL_CARD_PADDING))
            .shadow(t.shadow())
            .show(ui, |ui| self.contents(ui, actions))
            .response
    }

    /// The card's glyph, copy and actions, laid out to a width its caller
    /// fixed, so the copy wraps inside it rather than setting its width — a
    /// state's longest sentence is not a layout. The card and a bar mark's
    /// popup both say it this way.
    fn contents(&self, ui: &mut Ui, actions: &mut OperationalResponse) {
        let t = Tokens::get(ui.ctx());
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
                self.icon_tone,
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
                            .color(t.color.text),
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
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        actions.dismiss = rspice_ui_kit::widgets::Button::new("Dismiss")
                            .show(ui)
                            .on_hover_text("Dismiss this recorded runtime notice")
                            .clicked();
                    });
                }
            });
        }
    }

    /// Centre the card in the document well it has taken over.
    ///
    /// The card is placed by the height it drew at last, at its final width,
    /// so it lands on the well's own centre. A card whose height is new asks
    /// egui to lay the pass out again, so the frame shows it centred and the
    /// card is drawn, and announced to assistive technology, once. What this
    /// replaces was a guessed 172 pt height halved against the well's left
    /// edge, which put the card somewhere different for every state and in the
    /// middle for none of them.
    fn show_centered(&self, ui: &mut Ui, actions: &mut OperationalResponse) -> egui::Response {
        let well = ui.available_rect_before_wrap();
        let width =
            (well.width() - 2.0 * OPERATIONAL_CARD_GUTTER).clamp(1.0, OPERATIONAL_CARD_MAX_WIDTH);

        let height_id = ui.id().with("result-operational-state-height");
        let last_height = ui.ctx().data(|data| data.get_temp::<f32>(height_id));
        let height = last_height.unwrap_or(0.0);

        let top = (well.center().y - height * 0.5).max(well.top() + OPERATIONAL_CARD_GUTTER);
        let mut card = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(egui::Rect::from_min_size(
                    egui::pos2((well.center().x - width * 0.5).round(), top.round()),
                    egui::vec2(width, height),
                ))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        let response = self.show(&mut card, actions);
        let drawn_height = card.min_rect().height();
        if last_height.is_none_or(|height| (height - drawn_height).abs() > 0.5) {
            ui.ctx()
                .data_mut(|data| data.insert_temp(height_id, drawn_height));
            ui.ctx()
                .request_discard("the operational card changed height");
        }
        response
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
    let clicked = failure_site_button(ui, offer, false);
    if let Some(note) = failure_site_note(offer) {
        let t = Tokens::get(ui.ctx());
        ui.label(
            egui::RichText::new(note)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
        );
    }
    clicked
}

/// The marking control alone; a banner's line takes the compact form.
fn failure_site_button(ui: &mut Ui, offer: &FailureSiteControl<'_>, compact: bool) -> bool {
    let label = failure_site_label(offer);
    let button = rspice_ui_kit::widgets::Button::new(&label);
    if compact { button.compact() } else { button }
        .show(ui)
        .on_hover_text(failure_site_hint(offer))
        .clicked()
}

/// How many objects the engine measured but did not name, while the marking
/// is off.
fn failure_site_note(offer: &FailureSiteControl<'_>) -> Option<String> {
    (!offer.marked && offer.elided > 0)
        .then(|| format!("{} more were measured but not named.", offer.elided))
}

#[cfg(test)]
mod tests;
