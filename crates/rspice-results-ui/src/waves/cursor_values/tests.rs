//! A readout's values read in the reader's number format.

use super::*;

#[test]
fn a_slope_carries_an_si_prefix_unless_its_unit_carries_its_own_scale() {
    // 0.1 V over 0.5 ps, at four digits with the zeros kept.
    assert_eq!(slope_text(2.0e11, "V", "s", 4), "200.0 GV/s");
    assert_eq!(slope_text(-1.5e-3, "A", "s", 4), "\u{2212}1.500 mA/s");
    // A decibel slope is never prefixed ("kdB" is no unit) and reads to
    // the hundredth like every decibel.
    assert_eq!(slope_text(-20.0, "dB", "dec", 4), "\u{2212}20.00 dB/dec");
}

#[test]
fn a_frequency_reads_at_the_readers_significant_digits_in_the_readers_unit() {
    // 1/Δt of a 1.000139 ms span: the fixed-decimal frequency formatter read
    // "999.8610193 Hz" beside the span's seven digits.
    let hertz = QuantityPresentationPolicy::default();
    assert_eq!(frequency_text(1.0 / 1.000139e-3, 7, hertz), "999.8610 Hz");
    let radians = QuantityPresentationPolicy {
        frequency_display: FrequencyDisplay::RadiansPerSecond,
        ..QuantityPresentationPolicy::default()
    };
    assert_eq!(frequency_text(1.0e3, 4, radians), "6.283 krad/s");
}
