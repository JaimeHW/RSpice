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

    /// Whether the state's card offers Run again: a result the design moved
    /// on from, and a run that was cancelled or cut short, with or without
    /// the part it computed. A failed run offers what marks its failure
    /// instead.
    pub const fn offers_run_again(self) -> bool {
        matches!(
            self,
            Self::Stale | Self::InterruptedOperation | Self::Partial
        )
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
            Self::Failed => "The run failed; its log says why.",
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
            Self::Failed => "Fix the cause and run the plan again.",
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
    /// What the status says in place of its state's own label, tone and
    /// message, where those would misstate it.
    pub wording: Option<StatusWording>,
}

/// What a status says under its name and detail: what happened, when that
/// is not the detail alone, then what the reader can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusLines {
    pub message: Option<&'static str>,
    pub recovery: &'static str,
}

/// A status's words and tone where they are not its state's own: a run that
/// completed with some of its analyses failed reads as how many failed, in
/// the warning tone, rather than as a failed run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusWording {
    pub label: String,
    pub category: ResultOperationalCategory,
    pub message: &'static str,
}

impl ResultOperationalStatus {
    pub fn canonical(state: ResultOperationalState, blocks_visuals: bool) -> Self {
        Self {
            state,
            detail: None,
            blocks_visuals,
            dismissible: false,
            wording: None,
        }
    }

    /// What the status is called: its wording's label, else its state's.
    pub fn label(&self) -> &str {
        self.wording
            .as_ref()
            .map_or(self.state.label(), |wording| wording.label.as_str())
    }

    /// The tone the status is marked in: its wording's, else its state's.
    pub fn category(&self) -> ResultOperationalCategory {
        self.wording
            .as_ref()
            .map_or(self.state.category(), |wording| wording.category)
    }

    /// What the status says under its name and detail. The one place its
    /// words are picked: the card, a mark's popup, the empty hint, a hover
    /// and assistive technology all read these.
    ///
    /// A failure is an analysis's or a run's own, in its own run: its reason
    /// is the detail, then what to do. An action that failed over a sheet
    /// (an export, a copy, a file picker) is never the result's state; the
    /// toast and the log report it.
    pub fn lines(&self) -> StatusLines {
        let message = match &self.wording {
            Some(wording) => Some(wording.message),
            // The reason is the detail: it is not said twice.
            None if self.state == ResultOperationalState::Failed && self.detail.is_some() => None,
            None => Some(self.state.message()),
        };
        StatusLines {
            message,
            recovery: self.state.recovery(),
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

/// Display facts for the card's Run again, which runs the plan through the
/// same command as Run plan: why that command cannot run now, if it cannot.
pub struct RunAgainControl {
    pub unavailable: Option<&'static str>,
}

/// What a state's card can act on, as the host verified it: the failure's
/// named objects, and whether the plan can run again.
#[derive(Default)]
pub struct CardOffers<'a> {
    pub failure_site: Option<FailureSiteControl<'a>>,
    pub run_again: Option<RunAgainControl>,
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

/// Presentation outcome; the host applies dismiss, highlight and run-again
/// requests.
#[derive(Default)]
pub struct OperationalResponse {
    pub blocks_visuals: bool,
    pub dismiss: bool,
    pub highlight: bool,
    pub run_again: bool,
}

/// Whether `status` leaves the viewer readable and says so beside it: a
/// mark on the dock's Plot page and on the data browser's run row, rather
/// than anything in the well.
pub fn marks_beside_viewer(status: &ResultOperationalStatus) -> bool {
    status.state != ResultOperationalState::Complete && !status.blocks_visuals
}

/// Present `status` in the well when it takes the well over: the card of
/// the state, or the hint of an empty well. A state the viewer stays
/// readable under is marked beside it instead, so the well spends nothing on
/// it. Query what the card offers only when a card will be shown.
pub fn show<'a>(
    ui: &mut Ui,
    status: &ResultOperationalStatus,
    offers: impl FnOnce() -> CardOffers<'a>,
) -> OperationalResponse {
    if status.state == ResultOperationalState::Complete || marks_beside_viewer(status) {
        return OperationalResponse::default();
    }
    // An empty well is not an alarm. It reads the way an empty schematic
    // sheet does: guidance set on the canvas, with no card behind it.
    if status.blocks_visuals && status.category() == ResultOperationalCategory::Empty {
        let response = show_empty_hint(ui, status);
        announce_status(ui, &response, status);
        return OperationalResponse {
            blocks_visuals: true,
            ..Default::default()
        };
    }
    let t = Tokens::get(ui.ctx());
    let offers = offers();
    let card = OperationalCard {
        status,
        icon_tone: icon_tone(status.category(), &t.color),
        offers: &offers,
    };
    let mut actions = OperationalResponse {
        blocks_visuals: true,
        ..Default::default()
    };
    let response = card.show_centered(ui, &mut actions);
    announce_status(ui, &response, status);
    actions
}

/// A state the viewer stays readable under, as a band or a row carries it:
/// the category's glyph and the state's name, in a face of `size`, at the
/// height of the controls around it, so the well beneath keeps every point
/// of its height for the plot. Its hover reads the whole sentence; a press
/// opens it, with whatever the state offers. Query what it offers only when
/// that opens.
pub fn show_mark<'a>(
    ui: &mut Ui,
    status: &ResultOperationalStatus,
    size: f32,
    offers: impl FnOnce() -> CardOffers<'a>,
) -> OperationalResponse {
    let t = Tokens::get(ui.ctx());
    let category = status.category();
    let tone = icon_tone(category, &t.color);
    let name = mark_name(ui, status, size, t.color.text);
    let height = ui.spacing().interact_size.y;
    let width = mark_width_of(&name);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let glyph = egui::Rect::from_center_size(
        egui::pos2(
            rect.left() + OPERATIONAL_MARK_PADDING_X + 0.5 * OPERATIONAL_MARK_ICON,
            rect.center().y,
        ),
        egui::Vec2::splat(OPERATIONAL_MARK_ICON),
    );
    operational_icon(category).paint(ui.painter(), glyph, tone);
    let origin = egui::pos2(
        glyph.right() + OPERATIONAL_MARK_ICON_GAP,
        (rect.center().y - 0.5 * name.size().y).round(),
    );
    ui.painter().galley(origin, name, t.color.text);
    theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_text(sentence(status));
    announce_status(ui, &response, status);
    let mut actions = OperationalResponse::default();
    egui::Popup::menu(&response).show(|ui| {
        ui.set_max_width(OPERATIONAL_MARK_POPUP_WIDTH);
        let offers = offers();
        OperationalCard {
            status,
            icon_tone: tone,
            offers: &offers,
        }
        .contents(ui, &mut actions);
    });
    actions
}

/// How wide [`show_mark`] draws `status` with its name in a face of `size`,
/// for a row that sets the mark among what it paints.
pub fn mark_width(ui: &Ui, status: &ResultOperationalStatus, size: f32) -> f32 {
    mark_width_of(&mark_name(ui, status, size, egui::Color32::PLACEHOLDER))
}

fn mark_name(
    ui: &Ui,
    status: &ResultOperationalStatus,
    size: f32,
    color: egui::Color32,
) -> std::sync::Arc<egui::Galley> {
    ui.painter().layout_no_wrap(
        status.label().to_owned(),
        theme::sans(size, FontWeight::Medium),
        color,
    )
}

fn mark_width_of(name: &egui::Galley) -> f32 {
    2.0 * OPERATIONAL_MARK_PADDING_X
        + OPERATIONAL_MARK_ICON
        + OPERATIONAL_MARK_ICON_GAP
        + name.size().x
}

/// Everything a state says, as one sentence for a hover.
pub fn sentence(status: &ResultOperationalStatus) -> String {
    let lines = status.lines();
    [
        Some(status.label()),
        status.detail.as_deref(),
        lines.message,
    ]
    .into_iter()
    .flatten()
    .chain([lines.recovery])
    .collect::<Vec<_>>()
    .join(" · ")
}

/// Paint the glyph that stands for `status` in `rect`, in its category's
/// tone: what a tab carries after its name for the state of what it shows.
pub fn paint_status_glyph(ui: &Ui, rect: egui::Rect, status: &ResultOperationalStatus) {
    let t = Tokens::get(ui.ctx());
    let category = status.category();
    operational_icon(category).paint(ui.painter(), rect, icon_tone(category, &t.color));
}

/// Name the state to assistive technology on whatever response shows it, in
/// the words the screen shows: the role says it is a status.
fn announce_status(ui: &Ui, response: &egui::Response, status: &ResultOperationalStatus) {
    let lines = status.lines();
    let accessible = [status.detail.as_deref(), lines.message]
        .into_iter()
        .flatten()
        .chain([lines.recovery])
        .collect::<Vec<_>>()
        .join(" ");
    let accessible = format!("{}: {accessible}", status.label());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, accessible.as_str()));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(if status.category() == ResultOperationalCategory::Error {
            egui::accesskit::Role::Alert
        } else {
            egui::accesskit::Role::Status
        });
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
/// The mark's glyph, at the size of a toolbar icon, and the gap before the
/// state's name; its inset either side of the two, as a compact button
/// insets its label; and the width its popup wraps the state's copy at.
const OPERATIONAL_MARK_ICON: f32 = 14.0;
const OPERATIONAL_MARK_ICON_GAP: f32 = 7.0;
pub const OPERATIONAL_MARK_PADDING_X: f32 = 6.0;
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
        status.label(),
        theme::sans(EMPTY_HINT_TITLE_SIZE, FontWeight::Medium),
        t.color.text_dim,
    )];
    let line = theme::sans(EMPTY_HINT_LINE_SIZE, FontWeight::Regular);
    let lines = status.lines();
    rows.extend(
        [status.detail.as_deref(), lines.message]
            .into_iter()
            .flatten()
            .chain([lines.recovery])
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
    offers: &'a CardOffers<'a>,
}

impl OperationalCard<'_> {
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
            operational_icon(self.status.category()).paint(ui.painter(), glyph, self.icon_tone);
            let copy = (width - OPERATIONAL_CARD_ICON - OPERATIONAL_CARD_ICON_GAP).max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(copy, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(copy);
                    ui.spacing_mut().item_spacing.y = 3.0;
                    ui.label(
                        egui::RichText::new(self.status.label())
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
                    let lines = self.status.lines();
                    if let Some(message) = lines.message {
                        ui.label(
                            egui::RichText::new(message)
                                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                    }
                    ui.label(
                        egui::RichText::new(lines.recovery)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                },
            );
        });
        let run_again = self
            .offers
            .run_again
            .as_ref()
            .filter(|_| self.status.state.offers_run_again());
        let failure_site = self.offers.failure_site.as_ref();
        if failure_site.is_some() || run_again.is_some() || self.status.dismissible {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if let Some(control) = run_again {
                    actions.run_again = run_again_button(ui, control);
                }
                if let Some(offer) = failure_site {
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
    let clicked = failure_site_button(ui, offer);
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

/// Run again: the card's one primary action, which runs the plan as Run
/// plan does, and is unavailable for the reason that command gives.
fn run_again_button(ui: &mut Ui, control: &RunAgainControl) -> bool {
    let response = rspice_ui_kit::widgets::Button::new("Run again")
        .accent()
        .enabled(control.unavailable.is_none())
        .show(ui);
    match control.unavailable {
        Some(reason) => response.on_disabled_hover_text(reason),
        None => response.on_hover_text("Run the plan again, as Run plan does"),
    }
    .clicked()
}

/// The marking control alone.
fn failure_site_button(ui: &mut Ui, offer: &FailureSiteControl<'_>) -> bool {
    rspice_ui_kit::widgets::Button::new(&failure_site_label(offer))
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
