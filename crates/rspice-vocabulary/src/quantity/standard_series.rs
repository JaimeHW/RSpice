//! IEC 60063 preferred-number series for resistor, capacitor and inductor
//! values, and stepping a value text along one.

use serde::{Deserialize, Serialize};

use super::engineering::{format_engineering_value, parse_engineering_value, unit_tail};

/// The preferred-number series a project draws standard values from.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ESeries {
    E6,
    E12,
    #[default]
    E24,
    E48,
    E96,
    E192,
}

/// E192 mantissas; E96 and E48 are every second and every fourth member.
const E192: [u16; 192] = [
    100, 101, 102, 104, 105, 106, 107, 109, 110, 111, 113, 114, 115, 117, 118, 120, 121, 123, 124,
    126, 127, 129, 130, 132, 133, 135, 137, 138, 140, 142, 143, 145, 147, 149, 150, 152, 154, 156,
    158, 160, 162, 164, 165, 167, 169, 172, 174, 176, 178, 180, 182, 184, 187, 189, 191, 193, 196,
    198, 200, 203, 205, 208, 210, 213, 215, 218, 221, 223, 226, 229, 232, 234, 237, 240, 243, 246,
    249, 252, 255, 258, 261, 264, 267, 271, 274, 277, 280, 284, 287, 291, 294, 298, 301, 305, 309,
    312, 316, 320, 324, 328, 332, 336, 340, 344, 348, 352, 357, 361, 365, 370, 374, 379, 383, 388,
    392, 397, 402, 407, 412, 417, 422, 427, 432, 437, 442, 448, 453, 459, 464, 470, 475, 481, 487,
    493, 499, 505, 511, 517, 523, 530, 536, 542, 549, 556, 562, 569, 576, 583, 590, 597, 604, 612,
    619, 626, 634, 642, 649, 657, 665, 673, 681, 690, 698, 706, 715, 723, 732, 741, 750, 759, 768,
    777, 787, 796, 806, 816, 825, 835, 845, 856, 866, 876, 887, 898, 909, 920, 931, 942, 953, 965,
    976, 988,
];

/// E24 mantissas; E12 and E6 are every second and every fourth member. The
/// two-digit series are not subsets of the three-digit ones (E24 has 4.3,
/// E192 has 4.32).
const E24: [u16; 24] = [
    100, 110, 120, 130, 150, 160, 180, 200, 220, 240, 270, 300, 330, 360, 390, 430, 470, 510, 560,
    620, 680, 750, 820, 910,
];

impl ESeries {
    pub const ALL: [Self; 6] = [
        Self::E6,
        Self::E12,
        Self::E24,
        Self::E48,
        Self::E96,
        Self::E192,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::E6 => "E6",
            Self::E12 => "E12",
            Self::E24 => "E24",
            Self::E48 => "E48",
            Self::E96 => "E96",
            Self::E192 => "E192",
        }
    }

    /// The series' mantissas in one decade, scaled to `100..1000`.
    fn mantissas(self) -> impl Iterator<Item = u16> {
        let (table, stride): (&'static [u16], usize) = match self {
            Self::E6 => (&E24, 4),
            Self::E12 => (&E24, 2),
            Self::E24 => (&E24, 1),
            Self::E48 => (&E192, 4),
            Self::E96 => (&E192, 2),
            Self::E192 => (&E192, 1),
        };
        table.iter().copied().step_by(stride)
    }

    /// Every member of the decade holding `value` and of the decades either
    /// side, ascending. `value` must be finite and positive.
    fn neighbourhood(self, value: f64) -> Vec<f64> {
        let decade = value.log10().floor() as i32 - 2;
        (decade - 1..=decade + 1)
            .flat_map(|exponent| {
                self.mantissas()
                    .map(move |mantissa| standard_value(mantissa, exponent))
            })
            .collect()
    }

    /// The next member strictly above (`up`) or below `value`. Crossing a
    /// decade is seamless: E12 steps 8.2k up to 10k and 1k down to 820.
    pub fn step(self, value: f64, up: bool) -> Option<f64> {
        if !value.is_finite() || value <= 0.0 {
            return None;
        }
        let members = self.neighbourhood(value);
        if up {
            members.into_iter().find(|member| *member > value * (1.0 + 1e-9))
        } else {
            members.into_iter().rev().find(|member| *member < value * (1.0 - 1e-9))
        }
    }

    /// The member nearest `value` on a logarithmic scale, the way tolerance
    /// bands are spaced.
    pub fn snap(self, value: f64) -> Option<f64> {
        if !value.is_finite() || value <= 0.0 {
            return None;
        }
        self.neighbourhood(value).into_iter().min_by(|a, b| {
            let da = (a.ln() - value.ln()).abs();
            let db = (b.ln() - value.ln()).abs();
            da.total_cmp(&db)
        })
    }

    /// Step a component value text, keeping its unit: `4.7kΩ` steps to
    /// `5.1kΩ` in E24. `None` for an expression, a parameter reference or a
    /// non-positive value, which have no neighbour in a series.
    pub fn step_text(self, text: &str, up: bool) -> Option<String> {
        let value = parse_engineering_value(text).ok()?;
        let unit = unit_tail(text)?;
        self.step(value, up)
            .map(|next| format!("{}{unit}", format_engineering_value(next)))
    }

    /// Snap a component value text to the nearest member, keeping its unit.
    pub fn snap_text(self, text: &str) -> Option<String> {
        let value = parse_engineering_value(text).ok()?;
        let unit = unit_tail(text)?;
        self.snap(value)
            .map(|nearest| format!("{}{unit}", format_engineering_value(nearest)))
    }
}

/// `mantissa × 10^exponent`, parsed from its decimal text so 4.7e-12 is the
/// double nearest what a datasheet writes rather than a product of two
/// rounded factors.
fn standard_value(mantissa: u16, exponent: i32) -> f64 {
    format!("{mantissa}e{exponent}")
        .parse()
        .unwrap_or(f64::from(mantissa) * 10f64.powi(exponent))
}

#[cfg(test)]
mod tests {
    use super::ESeries;

    fn walk(series: ESeries, start: &str, up: bool, steps: usize) -> Vec<String> {
        let mut text = start.to_owned();
        (0..steps)
            .map(|_| {
                text = series.step_text(&text, up).expect("a positive value steps");
                text.clone()
            })
            .collect()
    }

    #[test]
    fn series_have_their_iec_member_counts() {
        for (series, count) in [
            (ESeries::E6, 6),
            (ESeries::E12, 12),
            (ESeries::E24, 24),
            (ESeries::E48, 48),
            (ESeries::E96, 96),
            (ESeries::E192, 192),
        ] {
            assert_eq!(series.mantissas().count(), count, "{}", series.label());
        }
        assert!(super::E192.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn e12_steps_across_decade_boundaries_both_ways() {
        assert_eq!(walk(ESeries::E12, "6.8k", true, 3), ["8.2k", "10k", "12k"]);
        assert_eq!(walk(ESeries::E12, "1.2k", false, 3), ["1k", "820", "680"]);
        assert_eq!(walk(ESeries::E12, "82n", true, 2), ["100n", "120n"]);
    }

    #[test]
    fn e24_steps_across_decade_boundaries_and_keeps_the_unit() {
        assert_eq!(walk(ESeries::E24, "8.2kΩ", true, 3), ["9.1kΩ", "10kΩ", "11kΩ"]);
        assert_eq!(walk(ESeries::E24, "11uF", false, 3), ["10uF", "9.1uF", "8.2uF"]);
        assert_eq!(walk(ESeries::E24, "910m", true, 2), ["1", "1.1"]);
        // A value between members steps to the neighbour on the side asked.
        assert_eq!(ESeries::E24.step_text("4.6k", true).as_deref(), Some("4.7k"));
        assert_eq!(ESeries::E24.step_text("4.6k", false).as_deref(), Some("4.3k"));
    }

    #[test]
    fn e96_steps_across_decade_boundaries() {
        assert_eq!(walk(ESeries::E96, "953", true, 3), ["976", "1k", "1.02k"]);
        assert_eq!(walk(ESeries::E96, "102p", false, 3), ["100p", "97.6p", "95.3p"]);
    }

    #[test]
    fn snapping_picks_the_logarithmically_nearest_member() {
        assert_eq!(ESeries::E24.snap_text("4.6k").as_deref(), Some("4.7k"));
        assert_eq!(ESeries::E12.snap_text("9.6k").as_deref(), Some("10k"));
        assert_eq!(ESeries::E96.snap_text("1.234k").as_deref(), Some("1.24k"));
        assert_eq!(ESeries::E6.snap_text("2.5uH").as_deref(), Some("2.2uH"));
    }

    #[test]
    fn expressions_and_non_positive_values_do_not_step() {
        for text in ["{rval}", "rval", "0", "-1k", ""] {
            assert_eq!(ESeries::E24.step_text(text, true), None, "{text}");
            assert_eq!(ESeries::E24.snap_text(text), None, "{text}");
        }
    }
}
