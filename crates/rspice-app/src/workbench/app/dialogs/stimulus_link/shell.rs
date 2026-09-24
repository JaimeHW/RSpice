//! The parts both directions of the stimulus link dialog are built from.
//!
//! Adopting and saving are one transaction over one instance against one
//! library, and a reader moving between them should meet the same surface with
//! a different verb on it. So the header, the pane split, the captions, the
//! card block and the fact list live here and are called twice, rather than
//! being written twice and drifting apart on a margin.
//!
//! Nothing here decides anything. Every string these painters take is already
//! owned somewhere else — the netlister's card, the model's refusal, the
//! library's provenance word — and this module only places them.

use egui::{Align, Layout, Rect, Sense, Stroke, Ui, vec2};

use crate::state::stimulus_library::provenance::ProvenanceState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// One definition's evaluated mini, kept for as long as the dialog is open.
///
/// The cache and the evaluation that fills it belong to the painter of the
/// picture, because the Stimulus Library's browser holds exactly the same list
/// against exactly the same cost.
pub(super) use crate::properties::source_preview::MiniCache;

/// How this dialog spells a number, which is how the library browser spells it
/// too — see [`crate::properties::source_preview::display_spelling`].
pub(super) use crate::properties::source_preview::display_spelling;

/// The vertical pitch of a fact row, and the inset every pane keeps.
pub(super) const FACT_PITCH: f32 = 22.0;
/// The inset a pane keeps from the dialog edge and from the hairline.
pub(super) const PANE_INSET: i8 = 16;

/// What the header says about the instance this transaction is over.
pub(super) struct Identity<'a> {
    /// `STIMULUS LIBRARY`.
    pub eyebrow: &'a str,
    /// `Adopt a definition onto V1`.
    pub title: &'a str,
    /// The instance's card exactly as the netlister writes it.
    pub card: &'a str,
    /// Where this instance stands with the library, in the library's words.
    pub chip: &'a str,
    /// The state behind the chip, which decides its tone.
    pub provenance: ProvenanceState,
}

/// The dialog's own header: what this is, what it is about, and where the
/// instance stands.
///
/// The kit's header paints an eyebrow, a title and a close mark, which is one
/// line short of what this transaction needs: the card the reader is about to
/// change is the subject of both modes, and a dialog that named the instance
/// without showing what it currently says would make the reader trust a
/// sentence instead of reading the deck. Returns whether the close mark was
/// pressed.
pub(super) fn header(ui: &mut Ui, identity: &Identity<'_>) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let chip_color = crate::properties::tabbed_dialog::provenance_colour(ui, identity.provenance);
    let chip_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let chip_track = measured_width(ui, identity.chip, chip_font.clone()).clamp(80.0, 210.0);
    // Placed against one allocated band rather than by nesting a right-aligned
    // row inside a left-aligned one: that nesting reads its track from the
    // parent's *remaining* rect, and a child that has already claimed its width
    // leaves the right-hand group free to run off the surface, which is where
    // the provenance chip ended up the first time this header was written.
    let (band, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), HEADER_HEIGHT), Sense::hover());
    ui.painter().rect_filled(band, 0.0, c.bg_panel);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            true,
            format!("{}. {} {}", identity.title, identity.card, identity.chip),
        )
    });

    let close_rect = Rect::from_min_size(
        egui::pos2(band.right() - 14.0 - CLOSE_WIDTH, band.top() + 12.0),
        vec2(CLOSE_WIDTH, 26.0),
    );
    let mut close_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(close_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let closed = crate::ui::widgets::IconButton::new(crate::ui::icons::Icon::Close)
        .size(CLOSE_WIDTH, 26.0)
        .tooltip("Close (Esc)")
        .show(&mut close_ui)
        .clicked();

    // The provenance reads as a chip rather than as a run of coloured text:
    // it is the one thing in this header that is a *state* rather than a name,
    // and the tone alone does not say so to a reader who cannot use it. The
    // tone itself stays the component editor's, through one owner.
    let chip_rect = Rect::from_min_size(
        egui::pos2(
            close_rect.left() - 10.0 - chip_track - 2.0 * CHIP_PAD,
            band.top() + 16.0,
        ),
        vec2(chip_track + 2.0 * CHIP_PAD, 18.0),
    );
    ui.painter().rect(
        chip_rect,
        3.0,
        c.bg_inset,
        Stroke::new(1.0, chip_color.gamma_multiply(0.55)),
        egui::StrokeKind::Inside,
    );
    // `interact`, never `allocate_rect`: allocating a rectangle inside a band
    // that has already been claimed rewinds the cursor to that rectangle's
    // bottom, and every band below this one would then be painted over the
    // header.
    if painted_run(
        ui,
        chip_rect.shrink2(vec2(CHIP_PAD, 0.0)),
        true,
        identity.chip,
        chip_font,
        chip_color,
    ) {
        let id = ui.id().with("stimulus-link-header-chip");
        ui.interact(chip_rect, id, Sense::hover())
            .on_hover_text(identity.chip);
    }

    let left = band.left() + 20.0;
    let text_right = (chip_rect.left() - 12.0).max(left + 120.0);
    let mut eyebrow = egui::text::LayoutJob::default();
    eyebrow.append(
        identity.eyebrow,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: c.text_faint,
            extra_letter_spacing: 0.09 * tokens::FS_0,
            ..Default::default()
        },
    );
    let eyebrow = ui.fonts_mut(|fonts| fonts.layout_job(eyebrow));
    ui.painter()
        .galley(egui::pos2(left, band.top() + 12.0), eyebrow, c.text_faint);
    painted_run(
        ui,
        Rect::from_min_max(
            egui::pos2(left, band.top() + 29.0),
            egui::pos2(text_right, band.top() + 48.0),
        ),
        false,
        identity.title,
        theme::sans(tokens::FS_3, FontWeight::SemiBold),
        c.text,
    );
    let card_rect = Rect::from_min_max(
        egui::pos2(left, band.top() + 51.0),
        egui::pos2(band.right() - 14.0, band.top() + 65.0),
    );
    if painted_run(
        ui,
        card_rect,
        false,
        identity.card,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_dim,
    ) {
        let id = ui.id().with("stimulus-link-header-card");
        ui.interact(card_rect, id, Sense::hover())
            .on_hover_text(identity.card);
    }
    hairline(ui);
    closed
}

/// The header band: twelve points of lead, three stacked lines, twelve below.
pub(super) const HEADER_HEIGHT: f32 = 77.0;
/// The close mark's target.
const CLOSE_WIDTH: f32 = 30.0;
/// Air inside the provenance chip, either side of its word.
const CHIP_PAD: f32 = 8.0;

/// A full-width rule at the cursor, the seam every band of this dialog is
/// separated by.
pub(super) fn hairline(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.border);
}

/// The body's two panes: a fixed-width pane, one hairline, and a pane that
/// fills what is left.
pub(super) fn panes(
    ui: &mut Ui,
    left_width: f32,
    height: f32,
    left: impl FnOnce(&mut Ui),
    right: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.allocate_ui_with_layout(
            vec2(left_width, height),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_min_size(vec2(left_width, height));
                left(ui);
            },
        );
        let (divider, _) = ui.allocate_exact_size(vec2(1.0, height), Sense::hover());
        ui.painter().rect_filled(divider, 0.0, t.color.border);
        let remaining = ui.available_width();
        ui.allocate_ui_with_layout(
            vec2(remaining, height),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_min_size(vec2(remaining, height));
                right(ui);
            },
        );
    });
}

/// The caption above a strip: what it shows at the left, and at the right the
/// window it was drawn over.
///
/// Outside the plot on purpose. Drawn inside, a caption is a line of text the
/// curve crosses, which is what the first version of this surface did.
pub(super) fn caption_row(ui: &mut Ui, left: &str, right: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), CAPTION_HEIGHT), Sense::hover());
    let announcement = match right {
        Some(right) => format!("{left} over {right}"),
        None => left.to_owned(),
    };
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &announcement));
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let right_width = right.map_or(0.0, |right| measured_width(ui, right, mono.clone()));
    let mut elided = false;
    if let Some(right) = right {
        let track =
            Rect::from_min_max(egui::pos2(rect.right() - right_width, rect.top()), rect.max);
        elided |= painted_run(ui, track, true, right, mono, t.color.text_faint);
    }
    let track = Rect::from_min_max(
        rect.min,
        egui::pos2(
            (rect.right() - right_width - 12.0).max(rect.left() + 1.0),
            rect.bottom(),
        ),
    );
    elided |= painted_run(
        ui,
        track,
        false,
        left,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    if elided {
        response.on_hover_text(announcement);
    }
}

/// One caption line's height.
pub(super) const CAPTION_HEIGHT: f32 = 16.0;

/// One inset mono block holding the card lines a transaction is about.
///
/// `after` is the card the instance would carry once the transaction lands.
/// When it is absent the block states the one card there is; when it equals the
/// card already on the instance the block says so, because a diff of a card
/// against itself is two identical lines and a reader looking for the
/// difference between them.
pub(super) fn card_block(ui: &mut Ui, before: &str, after: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let identical = after.is_some_and(|after| after == before);
    egui::Frame::NONE
        .fill(c.bg_inset)
        .stroke(Stroke::new(1.0, c.border))
        .corner_radius(3.0)
        .inner_margin(egui::Margin::same(CARD_BLOCK_INSET as i8))
        .show(ui, |ui| {
            // Claim the whole track the pane offers, so this well's right edge
            // is the waveform well's right edge and the facts list's: three
            // bands of one pane that stopped at three different x was the first
            // thing a reader noticed about it.
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = CARD_GAP;
            match after {
                Some(after) if !identical => {
                    signed_card(ui, "\u{2212}", c.err, before, c.text_dim);
                    signed_card(ui, "+", c.ok, after, c.text);
                }
                _ => signed_card(ui, " ", c.text_faint, before, c.text),
            }
        });
}

/// The height a card block of `lines` cards claims.
///
/// Every card is held to one elided line, so the block's height is arithmetic
/// rather than a measurement: a deck line that wrapped would move the whole
/// right pane, and the strips above it are the part of this surface a reader
/// is comparing across two picks.
pub(super) fn card_block_height(lines: usize) -> f32 {
    let lines = lines.max(1) as f32;
    lines * CARD_LINE + (lines - 1.0) * CARD_GAP + 2.0 * CARD_BLOCK_INSET + 2.0
}

/// One card line's height, the gap between two of them, and the well's inset.
const CARD_LINE: f32 = 15.0;
const CARD_GAP: f32 = 3.0;
const CARD_BLOCK_INSET: f32 = 8.0;

/// One card line behind its sign, as a single laid-out run so the sign cannot
/// drift away from the text it marks, elided into the block's own width so a
/// long PWL card cannot reflow the pane beneath it.
fn signed_card(
    ui: &mut Ui,
    sign: &str,
    sign_color: egui::Color32,
    card: &str,
    color: egui::Color32,
) {
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut job = egui::text::LayoutJob::default();
    job.append(
        sign,
        0.0,
        egui::TextFormat {
            font_id: font.clone(),
            color: sign_color,
            ..Default::default()
        },
    );
    job.append(
        card,
        6.0,
        egui::TextFormat {
            font_id: font,
            color,
            ..Default::default()
        },
    );
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), CARD_LINE), Sense::hover());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, card));
    job.wrap.max_width = rect.width().max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.overflow_character = Some('\u{2026}');
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let elided = galley.elided;
    ui.painter().galley(
        egui::pos2(rect.left(), rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    if elided {
        response.on_hover_text(card);
    }
}

/// A list of label/value facts on one pitch, with the labels in their own
/// column so the values line up as a column of their own.
///
/// The values are right of their labels rather than right of the pane: a value
/// flush to the far edge is a value the reader has to track across empty space
/// to reach, which is what the first version of the save mode did at 370
/// points of it.
pub(super) fn facts(ui: &mut Ui, label_width: f32, rows: &[(&str, Fact<'_>)]) {
    let t = Tokens::get(ui.ctx());
    for (label, value) in rows {
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), FACT_PITCH), Sense::hover());
        // A painted run reaches no accessibility tree on its own, and these two
        // runs are one fact: announced together, they read as the sentence a
        // sighted reader assembles from the columns.
        let announcement = format!("{label} {}", value.spoken());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &announcement)
        });
        ui.painter().text(
            rect.left_center(),
            egui::Align2::LEFT_CENTER,
            *label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        let mut track = Rect::from_min_max(
            egui::pos2(rect.left() + label_width, rect.top()),
            rect.right_bottom(),
        );
        let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
        let elided = match value {
            Fact::Plain(text) => painted_run(ui, track, false, text, mono, t.color.text),
            Fact::Transition(from, to) => {
                let before = painted_run(ui, track, false, from, mono.clone(), t.color.text_dim);
                let width = measured_width(ui, from, mono.clone()).min(track.width());
                track = Rect::from_min_max(
                    egui::pos2(track.left() + width + 7.0, track.top()),
                    track.max,
                );
                let arrow = Rect::from_center_size(
                    egui::pos2(track.left() + 5.0, track.center().y),
                    vec2(10.0, 8.0),
                );
                arrow_mark(ui, arrow, t.color.text_faint);
                track = Rect::from_min_max(egui::pos2(arrow.right() + 7.0, track.top()), track.max);
                before | painted_run(ui, track, false, to, mono, t.color.text)
            }
        };
        if elided {
            response.on_hover_text(value.spoken());
        }
    }
}

/// One fact's value: a state, or a change from one state to another.
pub(super) enum Fact<'a> {
    /// A value that simply is what it is.
    Plain(&'a str),
    /// Before and after, with an arrow between them.
    Transition(&'a str, &'a str),
}

impl Fact<'_> {
    /// The whole fact as one line, for a hover and for a screen reader.
    fn spoken(&self) -> String {
        match self {
            Self::Plain(text) => (*text).to_owned(),
            Self::Transition(from, to) => format!("{from} becomes {to}"),
        }
    }
}

/// The arrow between the two halves of a transition.
///
/// Painted rather than typeset: the bundled Plex faces carry no arrow, so a
/// character here would rasterize as a tofu box on the one row whose whole
/// meaning is the direction it points.
fn arrow_mark(ui: &Ui, rect: Rect, ink: egui::Color32) {
    let stroke = Stroke::new(1.0, ink);
    let middle = rect.center().y;
    ui.painter().hline(rect.x_range(), middle, stroke);
    for dy in [-3.0_f32, 3.0] {
        ui.painter().line_segment(
            [
                egui::pos2(rect.right(), middle),
                egui::pos2(rect.right() - 3.5, middle + dy),
            ],
            stroke,
        );
    }
}

/// How wide one run lays out, for placing what follows it.
pub(super) fn measured_width(ui: &mut Ui, text: &str, font: egui::FontId) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE)
            .size()
            .x
    })
}

/// One run of text, vertically centred in `rect` and elided into its width
/// rather than wrapped. Reports whether it was elided, so the caller can state
/// it whole on hover.
///
/// Painted rather than added as a widget throughout this dialog: every one of
/// these runs sits inside a rectangle whose geometry the surface decides, and
/// several sit inside click targets, where a label would swallow the press.
pub(super) fn painted_run(
    ui: &mut Ui,
    rect: Rect,
    right_aligned: bool,
    text: &str,
    font: egui::FontId,
    color: egui::Color32,
) -> bool {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_owned(), font, color);
    job.wrap.max_width = rect.width().max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.overflow_character = Some('\u{2026}');
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let elided = galley.elided;
    let x = if right_aligned {
        rect.right() - galley.size().x
    } else {
        rect.left()
    };
    ui.painter().galley(
        egui::pos2(x, rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    elided
}
