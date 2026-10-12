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

    /// Decibels, which the core carries as a declared custom symbol.
    pub(crate) fn decibel() -> Self {
        Self::Custom("dB".to_owned())
    }
}

/// The spelling of a unit in published JSON documents.
///
/// One wire form, because a result document and the scalars it carries must
/// encode a volt the same way; it lives with the vocabulary it encodes, so
/// neither producer owns it.
pub(crate) mod wire {
    use super::SignalUnit;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "unit", rename_all = "snake_case", deny_unknown_fields)]
    pub(crate) enum SignalUnitWire {
        Volt,
        Ampere,
        Ohm,
        Siemens,
        Watt,
        Hertz,
        Second,
        Degree,
        Radian,
        RadianPerSecond,
        Dimensionless,
        Logic,
        Unspecified,
        Custom { symbol: String },
    }

    impl From<&SignalUnit> for SignalUnitWire {
        fn from(unit: &SignalUnit) -> Self {
            match unit {
                SignalUnit::Volt => Self::Volt,
                SignalUnit::Ampere => Self::Ampere,
                SignalUnit::Ohm => Self::Ohm,
                SignalUnit::Siemens => Self::Siemens,
                SignalUnit::Watt => Self::Watt,
                SignalUnit::Hertz => Self::Hertz,
                SignalUnit::Second => Self::Second,
                SignalUnit::Degree => Self::Degree,
                SignalUnit::Radian => Self::Radian,
                SignalUnit::RadianPerSecond => Self::RadianPerSecond,
                SignalUnit::Dimensionless => Self::Dimensionless,
                SignalUnit::Logic => Self::Logic,
                SignalUnit::Unspecified => Self::Unspecified,
                SignalUnit::Custom(symbol) => Self::Custom {
                    symbol: symbol.clone(),
                },
            }
        }
    }

    impl From<SignalUnitWire> for SignalUnit {
        fn from(unit: SignalUnitWire) -> Self {
            match unit {
                SignalUnitWire::Volt => Self::Volt,
                SignalUnitWire::Ampere => Self::Ampere,
                SignalUnitWire::Ohm => Self::Ohm,
                SignalUnitWire::Siemens => Self::Siemens,
                SignalUnitWire::Watt => Self::Watt,
                SignalUnitWire::Hertz => Self::Hertz,
                SignalUnitWire::Second => Self::Second,
                SignalUnitWire::Degree => Self::Degree,
                SignalUnitWire::Radian => Self::Radian,
                SignalUnitWire::RadianPerSecond => Self::RadianPerSecond,
                SignalUnitWire::Dimensionless => Self::Dimensionless,
                SignalUnitWire::Logic => Self::Logic,
                SignalUnitWire::Unspecified => Self::Unspecified,
                SignalUnitWire::Custom { symbol } => Self::Custom(symbol),
            }
        }
    }

    /// `#[serde(with = ...)]` form of a required unit.
    pub(crate) mod required {
        use super::*;

        pub(crate) fn serialize<S: Serializer>(
            value: &SignalUnit,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            SignalUnitWire::from(value).serialize(serializer)
        }

        pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<SignalUnit, D::Error> {
            Ok(SignalUnitWire::deserialize(deserializer)?.into())
        }
    }

    /// `#[serde(with = ...)]` form of an optional unit.
    pub(crate) mod optional {
        use super::*;

        pub(crate) fn serialize<S: Serializer>(
            value: &Option<SignalUnit>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(unit) => serializer.serialize_some(&SignalUnitWire::from(unit)),
                None => serializer.serialize_none(),
            }
        }

        pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<SignalUnit>, D::Error> {
            Ok(Option::<SignalUnitWire>::deserialize(deserializer)?.map(SignalUnit::from))
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
