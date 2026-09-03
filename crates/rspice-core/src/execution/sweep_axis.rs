//! Declared unit of a swept `.DC`/`.STEP` variable.
//!
//! A sweep names an independent source, the ambient temperature, or a
//! parameter. The first two have a unit the simulator knows; the third has a
//! unit the deck never declared, which is [`SignalUnit::Unspecified`] and not
//! the pure ratio [`SignalUnit::Dimensionless`] would assert. Deciding that
//! per frontend is how one surface comes to label a swept parameter as
//! dimensionless while another labels it volts, so the rule lives here.

use super::schema::SignalUnit;

/// Unit of the sweep axis a `.DC` or `.STEP` card varies.
///
/// Source names follow the parser's own element-prefix contract: a `V`
/// instance is a voltage source and an `I` instance is a current source.
/// `TEMP`/`TEMPER` is the ambient temperature in degrees Celsius. Anything
/// else is a swept parameter or device parameter.
pub fn sweep_axis_unit(source: &str) -> SignalUnit {
    let source = source.trim();
    if source.eq_ignore_ascii_case("temp") || source.eq_ignore_ascii_case("temper") {
        return SignalUnit::Custom("degC".to_owned());
    }
    match source
        .chars()
        .next()
        .map(|first| first.to_ascii_uppercase())
    {
        Some('V') => SignalUnit::Volt,
        Some('I') => SignalUnit::Ampere,
        _ => SignalUnit::Unspecified,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_source_carries_its_excitation_unit_and_a_parameter_declares_none() {
        assert_eq!(sweep_axis_unit("V1"), SignalUnit::Volt);
        assert_eq!(sweep_axis_unit(" v1 "), SignalUnit::Volt);
        assert_eq!(sweep_axis_unit("Ibias"), SignalUnit::Ampere);
        assert_eq!(
            sweep_axis_unit("TEMP"),
            SignalUnit::Custom("degC".to_owned())
        );
        assert_eq!(
            sweep_axis_unit("temper"),
            SignalUnit::Custom("degC".to_owned())
        );
        assert_eq!(sweep_axis_unit("rval"), SignalUnit::Unspecified);
        assert_eq!(sweep_axis_unit(""), SignalUnit::Unspecified);
    }
}
