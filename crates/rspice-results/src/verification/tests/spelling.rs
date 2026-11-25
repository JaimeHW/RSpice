//! How a verified number is spelled: no prefix letter ever stands alone.

use crate::verification::{difference_text, format_bound, format_value};

/// A number with no unit takes no SI prefix, and neither does a difference
/// from it; a difference from a value printed without a prefix is printed
/// without one too, and from a prefixed value keeps its prefix.
#[test]
fn a_prefix_never_stands_alone() {
    assert_eq!(format_value(12.6, ""), "12.6");
    assert_eq!(format_value(7.5e-4, ""), "0.00075");
    assert_eq!(format_bound(1.2e-3, ""), "0.0012");
    assert_eq!(difference_text(-0.5, 12.6, ""), "\u{2212}0.5");
    assert_eq!(difference_text(-0.6, 39.4, "dB"), "\u{2212}0.6 dB");
    assert_eq!(difference_text(-0.5, 12.6, "V"), "\u{2212}0.5 V");
    assert_eq!(difference_text(4.0e-5, 1.16e-3, "A"), "+40 \u{b5}A");
    assert_eq!(difference_text(0.0, 1.2e-3, "A"), "0.0 mA");
}

/// A difference from a value in degrees stands against its number, as the
/// value does: a margin of 5° under 45°, of a quarter degree over 45.12°.
#[test]
fn a_difference_in_degrees_stands_against_its_number() {
    assert_eq!(format_value(45.0, "\u{b0}"), "45\u{b0}");
    assert_eq!(difference_text(-5.0, 45.0, "\u{b0}"), "\u{2212}5\u{b0}");
    assert_eq!(difference_text(0.25, 45.12, "\u{b0}"), "+0.25\u{b0}");
    assert_eq!(difference_text(0.0, 45.12, "\u{b0}"), "0.00\u{b0}");
}
