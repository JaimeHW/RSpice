//! The vocabulary of labels a device operating-point report may carry.
//!
//! A [`DeviceOpEntry`](super::DeviceOpEntry) names its device family, its
//! operating region and each reported quantity with a `&'static str`. Frontends
//! persist those reports and read them back by interning the stored text into
//! the same `&'static str`, so a reader can only restore a label it already
//! knows. That makes the emitter's label set part of the file format rather
//! than an internal detail: a family that reports a quantity nobody listed
//! writes a project that cannot be read, and refusing the write is the only
//! honest outcome left.
//!
//! `OpLabel` is that set, expressed as a type. It has no constructor from an
//! arbitrary string, so the report emitter cannot name a quantity the reader
//! would fail to resolve, and [`resolve_op_label`] is the reader's side of the
//! same table. A generated Verilog-A catalog contributes its own labels, which
//! are resolved from the catalog itself for the same reason: the compiled
//! catalog is the only place that knows them.

/// A label that may appear in a persisted operating-point report.
///
/// Copy-cheap: the value is the interned text itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OpLabel(&'static str);

impl OpLabel {
    /// The interned text this label is written as.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// A label supplied by the compiled Verilog-A catalog.
    ///
    /// Restricted to this crate because the catalog is the only authority that
    /// can vouch for such a label: [`resolve_op_label`] resolves it by asking
    /// the same catalog, and a caller outside the engine has nothing to check
    /// against.
    #[cfg(feature = "veriloga-builtins-base")]
    #[inline]
    pub(crate) const fn generated(text: &'static str) -> Self {
        Self(text)
    }
}

impl std::fmt::Display for OpLabel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

impl PartialEq<str> for OpLabel {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for OpLabel {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<OpLabel> for str {
    fn eq(&self, other: &OpLabel) -> bool {
        self == other.0
    }
}

impl PartialEq<OpLabel> for &str {
    fn eq(&self, other: &OpLabel) -> bool {
        *self == other.0
    }
}

/// Declare the vocabulary once: the constants the emitter writes, the listing
/// a consumer checks itself against, and the reader's lookup are all the same
/// table, so none of them can be extended without the others.
macro_rules! op_labels {
    ($($(#[$attr:meta])* $name:ident = $text:literal,)+) => {
        impl OpLabel {
            $($(#[$attr])* pub const $name: Self = Self($text);)+
        }

        /// Every label this build's emitters can produce, apart from those a
        /// generated Verilog-A catalog contributes.
        pub const OP_LABELS: &[OpLabel] = &[$(OpLabel::$name,)+];

        fn declared_op_label(text: &str) -> Option<&'static str> {
            match text {
                $($text => Some($text),)+
                _ => None,
            }
        }
    };
}

op_labels! {
    // Device families.
    MOSFET = "MOSFET",
    MOS9 = "MOS9",
    BSIM3 = "BSIM3",
    BSIM4 = "BSIM4",
    B3SOIFD = "B3SOIFD",
    B3SOIDD = "B3SOIDD",
    B3SOIPD = "B3SOIPD",
    EKV26 = "EKV26",
    EKV3 = "EKV3",
    VDMOS = "VDMOS",
    BJT = "BJT",
    DIODE = "DIODE",
    RESISTOR = "RESISTOR",
    CAPACITOR = "CAPACITOR",
    JFET = "JFET",
    JFET2 = "JFET2",
    JFET2_XYCE = "JFET2_XYCE",
    MESFET = "MESFET",
    HFET1 = "HFET1",
    HFET2 = "HFET2",
    NONLINEAR_CORE = "NONLINEAR_CORE",

    // Operating regions.
    CUTOFF = "cutoff",
    LINEAR = "linear",
    TRIODE = "triode",
    SATURATION = "saturation",
    QUASI_SATURATION = "quasi-saturation",
    SUBTHRESHOLD = "subthreshold",
    BODY_DIODE = "body-diode",
    ACTIVE = "active",
    REVERSE = "reverse",
    FORWARD = "forward",

    // Reported quantities.
    ID = "id",
    IS = "is",
    IC = "ic",
    IB = "ib",
    IE = "ie",
    IGS = "igs",
    IGD = "igd",
    IDIODE = "idiode",
    VD = "vd",
    VGS = "vgs",
    VDS = "vds",
    VBS = "vbs",
    VBE = "vbe",
    VCE = "vce",
    VTH = "vth",
    VDSAT = "vdsat",
    OUTPUT_VDSAT = "output_vdsat",
    GM = "gm",
    GDS = "gds",
    GD = "gd",
    CD = "cd",
    GMB = "gmb",
    GMBS = "gmbs",
    BETA = "beta",
    POWER = "power",
    R = "r",
    C = "c",
    TEMP = "temp",
    M = "m",
    H = "h",
    B = "b",
}

/// Resolve persisted text back to the interned label this build emits.
///
/// This is the reader's half of the vocabulary, and the reason the vocabulary
/// outlives its emitters: a label stays resolvable after the family that used
/// to report it changes, so a project written by an older build still opens.
pub fn resolve_op_label(text: &str) -> Option<&'static str> {
    declared_op_label(text).or_else(|| generated_catalog_op_label(text))
}

/// Labels contributed by the Verilog-A models compiled into this build.
///
/// A model's canonical name is its device family, and each external terminal
/// names the current entering it. Both are generated, so the catalog answers
/// for them; a build without the catalog has no such labels to resolve.
fn generated_catalog_op_label(text: &str) -> Option<&'static str> {
    for descriptor in crate::device::veriloga_builtins::generated_veriloga_model_descriptors() {
        if descriptor.model_name == text {
            return Some(descriptor.model_name);
        }
        for terminal in descriptor.terminals {
            if terminal.current_parameter == text {
                return Some(terminal.current_parameter);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_label_resolves_to_itself() {
        for label in OP_LABELS {
            assert_eq!(
                resolve_op_label(label.as_str()),
                Some(label.as_str()),
                "{label} is declared but does not resolve"
            );
        }
    }

    #[test]
    fn the_vocabulary_has_no_duplicate_text() {
        let mut seen = std::collections::HashSet::new();
        for label in OP_LABELS {
            assert!(
                seen.insert(label.as_str()),
                "{label} is declared twice; the reader would resolve only the first"
            );
        }
    }

    #[test]
    fn text_outside_the_vocabulary_does_not_resolve() {
        assert_eq!(resolve_op_label("not-a-reported-quantity"), None);
        assert_eq!(resolve_op_label("ID"), None, "labels are case-sensitive");
    }
}
