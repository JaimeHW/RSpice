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
            Self::Dimensionless => "1".to_owned(),
            Self::Logic => "logic".to_owned(),
            Self::Custom(symbol) => symbol.clone(),
            other => format!("{other:?}").to_ascii_lowercase(),
        }
    }
}
