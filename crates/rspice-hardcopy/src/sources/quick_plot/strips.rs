//! How a printed plot stacks its strips and states its abscissa.
//!
//! A sheet opens one strip per unit under one shared abscissa, and its X row
//! states the abscissa's quantity and unit once, in the row's left gutter.
//! The page lays the same strips out top to bottom in the plot area and
//! states the abscissa the same way.

use super::*;

/// The space between two strips of one printed plot: room for the lower
/// strip's offset anchor, with no tick row between them.
pub(crate) const STRIP_GAP_UM: i64 = 4_000;

/// What a printed plot's abscissa measures.
#[derive(Debug, Clone, PartialEq)]
pub struct QuickAbscissa {
    pub quantity: String,
    pub unit: String,
    pub scale: AxisScale,
}

impl QuickAbscissa {
    #[must_use]
    pub fn new(quantity: impl Into<String>, unit: impl Into<String>, scale: AxisScale) -> Self {
        Self {
            quantity: quantity.into(),
            unit: unit.into(),
            scale,
        }
    }

    /// What the X row's gutter says beside the abscissa's values, as the
    /// sheet's row says it: "TIME · s", "FREQ · Hz", "VD · V". Time and
    /// frequency are named by their unit; an axis with a unit and no
    /// quantity states the unit alone.
    #[must_use]
    pub fn caption(&self) -> String {
        let name = match self.unit.as_str() {
            "s" => "TIME".to_owned(),
            "Hz" => "FREQ".to_owned(),
            _ => self.quantity.trim().to_ascii_uppercase(),
        };
        match (name.is_empty(), self.unit.is_empty()) {
            (true, _) => self.unit.clone(),
            (false, true) => name,
            (false, false) => format!("{name} \u{b7} {}", self.unit),
        }
    }
}

/// One strip's band of the plot area: its top edge and height.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StripRect {
    pub(crate) top: i64,
    pub(crate) height: i64,
}

impl StripRect {
    /// `point`, mapped into the whole plot area, moved into this strip.
    pub(crate) fn place(self, point: SemanticPoint) -> SemanticPoint {
        let plot_height = i128::from(PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM);
        let offset = i128::from(point.y_um - PLOT_INSET_UM) * i128::from(self.height);
        let y = i64::try_from(offset / plot_height).unwrap_or(0);
        SemanticPoint::new(point.x_um, self.top + y)
    }

    pub(crate) fn bottom(self) -> i64 {
        self.top + self.height
    }
}

/// `count` strips of equal height stacked down a plot area `plot_height`
/// tall, `STRIP_GAP_UM` apart.
pub(crate) fn strip_rects(count: usize, plot_height: i64) -> Vec<StripRect> {
    let count = i64::try_from(count.max(1)).unwrap_or(1);
    let height = (plot_height - (count - 1) * STRIP_GAP_UM) / count;
    (0..count)
        .map(|index| StripRect {
            top: PLOT_INSET_UM + index * (height + STRIP_GAP_UM),
            height,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_abscissa_is_stated_as_the_sheets_x_row_states_it() {
        let caption = |quantity: &str, unit: &str| {
            QuickAbscissa::new(quantity, unit, AxisScale::Linear).caption()
        };
        assert_eq!(caption("Time", "s"), "TIME \u{b7} s");
        assert_eq!(caption("Frequency", "Hz"), "FREQ \u{b7} Hz");
        assert_eq!(caption("vd", "V"), "VD \u{b7} V");
        assert_eq!(caption("", "UI"), "UI");
        assert_eq!(caption("Mode", ""), "MODE");
    }

    #[test]
    fn strips_share_the_plot_area_and_one_strip_fills_it() {
        let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
        let one = strip_rects(1, plot_height);
        assert_eq!(
            one,
            [StripRect {
                top: PLOT_INSET_UM,
                height: plot_height
            }]
        );
        let point = SemanticPoint::new(5, PLOT_INSET_UM + 1_234);
        assert_eq!(one[0].place(point), point);
        let three = strip_rects(3, plot_height);
        assert_eq!(three[1].top, three[0].bottom() + STRIP_GAP_UM);
        assert!(three[2].bottom() <= PLOT_INSET_UM + plot_height);
    }
}
