//! Physical-unit vocabulary shared by result producers and consumers.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SignalUnit {
    Volt,
    Ampere,
    Ohm,
    Siemens,
    Watt,
    Hertz,
    Second,
    Degree,
    Radian,
    /// Angular frequency, including complex Laplace-plane roots.
    RadianPerSecond,
    Dimensionless,
    Logic,
    /// The producing analysis knows of no physical unit for this quantity.
    ///
    /// This is not [`Self::Dimensionless`]: a reflection coefficient really is
    /// a pure ratio, while a `.DC` sweep over a user parameter or a `.FOUR` of
    /// a braced expression has a unit the deck simply never declared. Folding
    /// the second case onto the first asserts a physical fact nobody stated,
    /// and spelling it `Custom("unspecified")` invents a symbol that no
    /// consumer can distinguish from a real one.
    Unspecified,
    Custom(String),
}

impl SignalUnit {
    /// Short symbol a published artifact spells this unit with.
    ///
    /// One table, because two surfaces that disagree on how a volt is written
    /// publish two files a reader cannot compare: the engine adapter's
    /// measurement manifest and the command line's HDF5 columns both come
    /// through here.
    ///
    /// `SignalUnit` is `#[non_exhaustive]`, so a future core unit renders as
    /// its own lower-case tag rather than being folded onto an existing
    /// symbol: an unrecognized unit must never be reported as a different one.
    #[must_use]
    pub fn symbol(&self) -> String {
        match self {
            Self::Volt => "V".to_owned(),
            Self::Ampere => "A".to_owned(),
            Self::Ohm => "ohm".to_owned(),
            Self::Siemens => "S".to_owned(),
            Self::Watt => "W".to_owned(),
            Self::Hertz => "Hz".to_owned(),
            Self::Second => "s".to_owned(),
            Self::Degree => "deg".to_owned(),
            Self::Radian => "rad".to_owned(),
            Self::RadianPerSecond => "rad/s".to_owned(),
            Self::Dimensionless => "1".to_owned(),
            Self::Logic => "logic".to_owned(),
            Self::Custom(symbol) => symbol.clone(),
            other => format!("{other:?}").to_ascii_lowercase(),
        }
    }
}

/// Unit of the sweep axis a `.DC` or `.STEP` card varies.
///
/// A sweep names an independent source, the ambient temperature, or a
/// parameter. The first two have a unit the simulator knows; the third has a
/// unit the deck never declared, which is [`SignalUnit::Unspecified`] and not
/// the pure ratio [`SignalUnit::Dimensionless`] would assert. Deciding that
/// per frontend is how one surface comes to label a swept parameter as
/// dimensionless while another labels it volts, so the rule lives here.
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
