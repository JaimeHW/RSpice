//! How a verified number is spelled: no prefix letter ever stands alone.

use crate::verification::{difference_text, format_bound, format_value};

/// A number with no unit takes no SI prefix, and neither does a difference
/// from it; a difference from a value printed without a prefix is printed
/// without one too, and from a prefixed value keeps its prefix.
#[test]
fn a_prefix_never_stands_alone() {
    assert_eq!(format_value(12.6, ""), "12.60");
    assert_eq!(format_value(7.5e-4, ""), "0.0007500");
    assert_eq!(format_bound(1.2e-3, ""), "0.0012");
    assert_eq!(difference_text(-0.5, 12.6, ""), "-0.50");
    assert_eq!(difference_text(-0.6, 39.4, "dB"), "-0.60 dB");
    assert_eq!(difference_text(-0.5, 12.6, "V"), "-0.50 V");
    assert_eq!(difference_text(4.0e-5, 1.16e-3, "A"), "+40 \u{b5}A");
    assert_eq!(difference_text(0.0, 1.2e-3, "A"), "0.000 mA");
}
