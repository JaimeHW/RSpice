//! Where a schematic's text stands, in world units.
//!
//! The canvas, the SVG export and the printed page each place an instance's
//! name and value, a net label's name and connector tab, a bus declaration,
//! a tap's bit slice and a probe's reference through this one layout, so a
//! reviewer finds every label in the same place on screen, in an export and
//! on paper. It is pure geometry: a renderer scales and offsets it into its
//! own space and sets the type itself.

use rspice_design_model::{Point, design_management::CrossSheetPortDirection};

use super::{component::Component, component_type::ComponentType, net_label::NetLabel};
use crate::symbol::SymbolDocument;

/// An instance name's type size, in world units.
pub const INSTANCE_NAME_WORLD: f32 = 10.0;
/// An instance value's type size, in world units.
pub const INSTANCE_VALUE_WORLD: f32 = 9.0;
/// The small electrical text: bus declarations, tap slices, probe
/// references.
pub const SMALL_TEXT_WORLD: f32 = 9.0;

/// A bus declaration's bottom-left corner, from the bus's last point.
pub const BUS_DECLARATION_OFFSET: LayoutPoint = LayoutPoint::new(8.0, -8.0);
/// A tap slice's bottom-left corner, from the tap's connection point.
pub const TAP_SLICE_OFFSET: LayoutPoint = LayoutPoint::new(5.0, -7.0);
/// A probe reference's bottom-left corner, from the probe's position.
pub const PROBE_LABEL_OFFSET: LayoutPoint = LayoutPoint::new(13.0, -9.0);

/// A net label's type size, in world units.
pub const NET_LABEL_FONT_WORLD: f32 = 11.0;
/// The estimated advance of one glyph of a net label's name, in world units.
pub const NET_LABEL_GLYPH_WIDTH_WORLD: f32 = 6.7;
/// Half the diagonal of a net label's attachment diamond, in world units.
pub const NET_LABEL_DIAMOND_HALF_WORLD: f32 = 2.25;
/// What a net label with a blank name reads.
pub const UNNAMED_NET_LABEL: &str = "<unnamed>";
/// A connector tab's stroke, as a fraction of its height.
pub const CONNECTOR_TAB_STROKE_PER_HEIGHT: f32 = 0.085;

const NET_LABEL_OFFSET: LayoutPoint = LayoutPoint::new(7.0, -5.0);
/// Gap between a connector's name and its tab.
pub const CONNECTOR_TAB_GAP_WORLD: f32 = 3.0;
/// Width of a connector tab. Its height follows the name, so the pair reads
/// as one object.
pub const CONNECTOR_TAB_WIDTH_WORLD: f32 = 12.0;
/// One em of a line box: Plex's ascent plus descent.
pub const LINE_BOX_EM: f32 = 1.3;
/// Plex's ascent, in em: a line's baseline sits this far below its top.
pub const TEXT_ASCENT_EM: f32 = 1.025;
/// A run's estimated advance per glyph, in em: wide enough for Plex Sans and
/// Plex Mono alike, so a box built on it holds the text it stands for.
pub const GLYPH_ADVANCE_EM: f32 = 0.62;

/// A point in whatever linear space the caller evaluates the layout in:
/// world units, screen points or page units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutPoint {
    pub x: f32,
    pub y: f32,
}

impl LayoutPoint {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// An axis-aligned box in the same space as its points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutRect {
    pub min: LayoutPoint,
    pub max: LayoutPoint,
}

impl LayoutRect {
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn center(&self) -> LayoutPoint {
        LayoutPoint::new(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
        )
    }
}

/// Which end of a run of text, on one axis, sits on its anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEdge {
    /// The left or top edge.
    Start,
    Center,
    /// The right or bottom edge.
    End,
}

/// A run of text's anchor and how the text hangs off it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacedText {
    pub anchor: LayoutPoint,
    pub horizontal: TextEdge,
    pub vertical: TextEdge,
}

impl PlacedText {
    const fn new(anchor: LayoutPoint, horizontal: TextEdge, vertical: TextEdge) -> Self {
        Self {
            anchor,
            horizontal,
            vertical,
        }
    }
}

/// Where a primitive's name and value stand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InstanceLabelLayout {
    pub name: PlacedText,
    pub value: PlacedText,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerminalAxis {
    Horizontal,
    Vertical,
    Mixed,
}

/// The dominant terminal direction after the instance's turn. Mirroring
/// never changes an axis span and a quarter turn swaps the spans, so this
/// reads the static offsets and allocates nothing.
fn infer_terminal_axis(component: &Component) -> TerminalAxis {
    let offsets = component.kind.terminal_offsets();
    if offsets.len() < 2 {
        return TerminalAxis::Mixed;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (_, p) in offsets {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    let mut span_x = max_x - min_x;
    let mut span_y = max_y - min_y;
    if component.rotation.is_vertical() {
        std::mem::swap(&mut span_x, &mut span_y);
    }

    // One grid of hysteresis keeps a near-square span from flapping.
    const AXIS_HYSTERESIS: i32 = 10;
    if span_x > span_y + AXIS_HYSTERESIS {
        TerminalAxis::Horizontal
    } else if span_y > span_x + AXIS_HYSTERESIS {
        TerminalAxis::Vertical
    } else {
        TerminalAxis::Mixed
    }
}

/// Where an instance's name and value stand, clear of its turned body: above
/// and below when its pins run across, left and right when they run up and
/// down, and name above with value to the right for a tall device. Mirroring
/// leaves them where they are.
///
/// Linear in `origin` and `scale`: `origin` is the instance's position and
/// `scale` the size of one world unit in the caller's space, so the canvas
/// evaluates it in screen points and an export in page units.
pub fn instance_label_layout(
    origin: LayoutPoint,
    scale: f32,
    component: &Component,
) -> InstanceLabelLayout {
    use TextEdge::{Center, End, Start};

    let (width, height) = component.symbol_dimensions();
    let (eff_w, eff_h) = if component.rotation.is_vertical() {
        (height as f32, width as f32)
    } else {
        (width as f32, height as f32)
    };

    let is_tall_device = matches!(
        component.kind,
        ComponentType::NpnBjt
            | ComponentType::PnpBjt
            | ComponentType::NpnBjt4
            | ComponentType::PnpBjt4
            | ComponentType::NpnBjt5
            | ComponentType::PnpBjt5
            | ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::Njfet
            | ComponentType::Pjfet
            | ComponentType::Nmesfet
            | ComponentType::Pmesfet
            | ComponentType::NVdmos
            | ComponentType::PVdmos
            | ComponentType::NmosSoi
            | ComponentType::PmosSoi
    );

    let margin = 4.0 * scale;
    let half_w = (eff_w / 2.0) * scale;
    let half_h = (eff_h / 2.0) * scale;
    let (x, y) = (origin.x, origin.y);
    let above = PlacedText::new(LayoutPoint::new(x, y - half_h - margin), Center, End);
    let right = PlacedText::new(LayoutPoint::new(x + half_w + margin, y), Start, Center);

    if is_tall_device {
        return InstanceLabelLayout {
            name: above,
            value: right,
        };
    }
    // A mixed axis falls back to the turned body's aspect ratio.
    let use_left_right = match infer_terminal_axis(component) {
        TerminalAxis::Vertical => true,
        TerminalAxis::Mixed => eff_h > eff_w,
        TerminalAxis::Horizontal => false,
    };
    if use_left_right {
        InstanceLabelLayout {
            name: PlacedText::new(LayoutPoint::new(x - half_w - margin, y), End, Center),
            value: right,
        }
    } else {
        InstanceLabelLayout {
            name: above,
            value: PlacedText::new(LayoutPoint::new(x, y + half_h + margin), Center, Start),
        }
    }
}

/// Where an instance of an authored symbol letters its name or value: the
/// symbol's own `anchor`, turned with the instance, in world units.
///
/// The text stays upright. The authored run grows rightward from its anchor
/// and its justification turns with the instance, so the run still grows
/// away from the body. A run turned to grow vertically, which upright text
/// cannot, hangs below or stands above its anchor, and starts or ends on the
/// side of the body the anchor is on.
pub fn symbol_label_placement(
    component: &Component,
    document: &SymbolDocument,
    anchor: Point,
) -> PlacedText {
    let place = |point: Point| component.transform_point(point - document.origin);
    let local = place(anchor);
    let at = LayoutPoint::new(
        (component.pos.x + local.x) as f32,
        (component.pos.y + local.y) as f32,
    );
    let growth = component.transform_point(Point::new(1, 0));
    if growth.x != 0 {
        let horizontal = if growth.x > 0 {
            TextEdge::Start
        } else {
            TextEdge::End
        };
        return PlacedText::new(at, horizontal, TextEdge::Center);
    }
    let horizontal = document
        .drawn_body_bounds()
        .map_or(TextEdge::Center, |(min, max)| {
            let (min, max) = (place(min), place(max));
            if local.x > min.x.max(max.x) {
                TextEdge::Start
            } else if local.x < min.x.min(max.x) {
                TextEdge::End
            } else {
                TextEdge::Center
            }
        });
    let vertical = if growth.y > 0 {
        TextEdge::Start
    } else {
        TextEdge::End
    };
    PlacedText::new(at, horizontal, vertical)
}

/// The baselines of `lines` stacked lines of `size` type hung off `y` the way
/// `vertical` hangs the block, top line first: one line box per line, each
/// baseline [`TEXT_ASCENT_EM`] below its line's top.
pub fn text_baselines(
    y: f32,
    vertical: TextEdge,
    size: f32,
    lines: usize,
) -> impl Iterator<Item = f32> {
    let advance = LINE_BOX_EM * size;
    let height = advance * lines as f32;
    let top = match vertical {
        TextEdge::Start => y,
        TextEdge::Center => y - height * 0.5,
        TextEdge::End => y - height,
    };
    (0..lines).map(move |line| top + advance * line as f32 + TEXT_ASCENT_EM * size)
}

/// The box a block of text takes when hung off its anchor: the widest
/// line's estimated advance across, and a line box per line down.
pub fn text_box(placed: PlacedText, text: &str, size: f32) -> LayoutRect {
    let lines = text.lines().count().max(1);
    let glyphs = text
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let width = glyphs as f32 * size * GLYPH_ADVANCE_EM;
    let height = lines as f32 * size * LINE_BOX_EM;
    let left = match placed.horizontal {
        TextEdge::Start => placed.anchor.x,
        TextEdge::Center => placed.anchor.x - width * 0.5,
        TextEdge::End => placed.anchor.x - width,
    };
    let top = match placed.vertical {
        TextEdge::Start => placed.anchor.y,
        TextEdge::Center => placed.anchor.y - height * 0.5,
        TextEdge::End => placed.anchor.y - height,
    };
    LayoutRect {
        min: LayoutPoint::new(left, top),
        max: LayoutPoint::new(left + width, top + height),
    }
}

/// What a net label reads: its name, or a placeholder for a blank one.
pub fn net_label_display_name(label: &NetLabel) -> &str {
    if label.name.trim().is_empty() {
        UNNAMED_NET_LABEL
    } else {
        &label.name
    }
}

/// The world corner a net label's name stands on: the bottom-left of its
/// text box.
pub fn net_label_name_corner(label: &NetLabel) -> LayoutPoint {
    LayoutPoint::new(
        label.pos.x as f32 + NET_LABEL_OFFSET.x,
        label.pos.y as f32 + NET_LABEL_OFFSET.y,
    )
}

/// A net label in world units, as a zoomless drawing places it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NetLabelLayout<'a> {
    pub name: &'a str,
    pub font_size: f32,
    /// The bottom-left corner of the name's text box.
    pub corner: LayoutPoint,
    /// Half the diagonal of the attachment diamond.
    pub diamond_half: f32,
    /// An off-sheet connector's tab after a name of the estimated width.
    pub tab: Option<(LayoutRect, CrossSheetPortDirection)>,
}

pub fn net_label_layout(label: &NetLabel) -> NetLabelLayout<'_> {
    let name = net_label_display_name(label);
    let corner = net_label_name_corner(label);
    let height = NET_LABEL_FONT_WORLD * LINE_BOX_EM;
    let tab = label.kind.off_sheet_direction().map(|direction| {
        let min = LayoutPoint::new(
            corner.x
                + name.chars().count() as f32 * NET_LABEL_GLYPH_WIDTH_WORLD
                + CONNECTOR_TAB_GAP_WORLD,
            corner.y - height,
        );
        (
            LayoutRect {
                min,
                max: LayoutPoint::new(min.x + CONNECTOR_TAB_WIDTH_WORLD, min.y + height),
            },
            direction,
        )
    });
    NetLabelLayout {
        name,
        font_size: NET_LABEL_FONT_WORLD,
        corner,
        diamond_half: NET_LABEL_DIAMOND_HALF_WORLD,
        tab,
    }
}

/// The marks inside a connector tab, in whatever space `rect` is in: the
/// schematic port glyph's arrow with its head at the end the signal travels
/// toward, both heads when the net is bidirectional, and a barred stem for
/// a supply.
pub fn connector_tab_marks(
    rect: LayoutRect,
    direction: CrossSheetPortDirection,
) -> Vec<[LayoutPoint; 2]> {
    let center = rect.center();
    let reach = rect.width() * 0.28;
    let head = rect.width() * 0.16;
    let rise = rect.height() * 0.20;
    let left = LayoutPoint::new(center.x - reach, center.y);
    let right = LayoutPoint::new(center.x + reach, center.y);

    if matches!(direction, CrossSheetPortDirection::Supply) {
        return vec![
            [
                LayoutPoint::new(left.x, center.y - rise),
                LayoutPoint::new(right.x, center.y - rise),
            ],
            [
                LayoutPoint::new(center.x, center.y - rise),
                LayoutPoint::new(center.x, center.y + rise),
            ],
        ];
    }

    let mut marks = vec![[left, right]];
    if matches!(
        direction,
        CrossSheetPortDirection::Input | CrossSheetPortDirection::InOut
    ) {
        marks.push([left, LayoutPoint::new(left.x + head, center.y - rise)]);
        marks.push([left, LayoutPoint::new(left.x + head, center.y + rise)]);
    }
    if matches!(
        direction,
        CrossSheetPortDirection::Output | CrossSheetPortDirection::InOut
    ) {
        marks.push([right, LayoutPoint::new(right.x - head, center.y - rise)]);
        marks.push([right, LayoutPoint::new(right.x - head, center.y + rise)]);
    }
    marks
}
