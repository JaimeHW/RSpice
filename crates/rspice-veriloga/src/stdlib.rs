//! Built-in Verilog-AMS standard headers
//!
//! Compact models universally start with `include "disciplines.vams"` and
//! `include "constants.vams"`. When those files are not found on disk, the
//! preprocessor falls back to these built-in copies so that models compile
//! out of the box, matching the behavior of commercial simulators that ship
//! the standard headers.
//!
//! Constant values follow the Verilog-AMS LRM 2.4 `constants.vams`, with its
//! default NIST 1998 physical constants, as OpenVAF does, so that models
//! calibrated against other LRM-compliant simulators reproduce the same
//! numbers.

/// Standard `disciplines.vams` header (Verilog-AMS LRM 2.4, Annex D).
pub const DISCIPLINES_VAMS: &str = r#"
`ifdef DISCIPLINES_VAMS
`else
`define DISCIPLINES_VAMS 1

discipline logic;
  domain discrete;
enddiscipline

discipline ddiscrete;
  domain discrete;
enddiscipline

nature Current;
  units      = "A";
  access     = I;
  idt_nature = Charge;
  abstol     = 1e-12;
endnature

nature Charge;
  units      = "coul";
  access     = Q;
  ddt_nature = Current;
  abstol     = 1e-14;
endnature

nature Voltage;
  units      = "V";
  access     = V;
  idt_nature = Flux;
  abstol     = 1e-6;
endnature

nature Flux;
  units      = "Wb";
  access     = Phi;
  ddt_nature = Voltage;
  abstol     = 1e-9;
endnature

discipline electrical;
  potential Voltage;
  flow      Current;
enddiscipline

discipline voltage;
  potential Voltage;
enddiscipline

discipline current;
  flow Current;
enddiscipline

nature Magneto_Motive_Force;
  units      = "A*turn";
  access     = MMF;
  abstol     = 1e-12;
endnature

discipline magnetic;
  potential Magneto_Motive_Force;
  flow      Flux;
enddiscipline

nature Temperature;
  units      = "K";
  access     = Temp;
  abstol     = 1e-4;
endnature

nature Power;
  units      = "W";
  access     = Pwr;
  abstol     = 1e-9;
endnature

discipline thermal;
  potential Temperature;
  flow      Power;
enddiscipline

nature Position;
  units      = "m";
  access     = Pos;
  ddt_nature = Velocity;
  abstol     = 1e-6;
endnature

nature Velocity;
  units      = "m/s";
  access     = Vel;
  ddt_nature = Acceleration;
  idt_nature = Position;
  abstol     = 1e-6;
endnature

nature Acceleration;
  units      = "m/s^2";
  access     = Acc;
  ddt_nature = Impulse;
  idt_nature = Velocity;
  abstol     = 1e-6;
endnature

nature Impulse;
  units      = "m/s^3";
  access     = Imp;
  idt_nature = Acceleration;
  abstol     = 1e-6;
endnature

nature Force;
  units      = "N";
  access     = F;
  abstol     = 1e-6;
endnature

discipline kinematic;
  potential Position;
  flow      Force;
enddiscipline

discipline kinematic_v;
  potential Velocity;
  flow      Force;
enddiscipline

nature Angle;
  units      = "rads";
  access     = Theta;
  ddt_nature = Angular_Velocity;
  abstol     = 1e-6;
endnature

nature Angular_Velocity;
  units      = "rads/s";
  access     = Omega;
  ddt_nature = Angular_Acceleration;
  idt_nature = Angle;
  abstol     = 1e-6;
endnature

nature Angular_Acceleration;
  units      = "rads/s^2";
  access     = Alpha;
  idt_nature = Angular_Velocity;
  abstol     = 1e-6;
endnature

nature Angular_Force;
  units      = "N*m";
  access     = Tau;
  abstol     = 1e-6;
endnature

discipline rotational;
  potential Angle;
  flow      Angular_Force;
enddiscipline

discipline rotational_omega;
  potential Angular_Velocity;
  flow      Angular_Force;
enddiscipline

`endif
"#;

/// Standard `constants.vams` header (Verilog-AMS LRM 2.4, Annex D), as
/// OpenVAF ships it: `P_Q`, `P_K`, `P_H` and `P_EPS0` default to the NIST
/// 1998 set, and `PHYSICAL_CONSTANTS_SPICE`, `PHYSICAL_CONSTANTS_OLD` or
/// `PHYSICAL_CONSTANTS_NIST2010` selects another.
pub const CONSTANTS_VAMS: &str = r#"
`ifdef CONSTANTS_VAMS
`else
`define CONSTANTS_VAMS 1

// Mathematical constants
`define M_PI        3.14159265358979323846
`define M_TWO_PI    6.28318530717958647693
`define M_PI_2      1.57079632679489661923
`define M_PI_4      0.78539816339744830962
`define M_1_PI      0.31830988618379067154
`define M_2_PI      0.63661977236758134308
`define M_2_SQRTPI  1.12837916709551257390
`define M_E         2.7182818284590452354
`define M_LOG2E     1.4426950408889634074
`define M_LOG10E    0.43429448190325182765
`define M_LN2       0.69314718055994530942
`define M_LN10      2.30258509299404568402
`define M_SQRT2     1.41421356237309504880
`define M_SQRT1_2   0.70710678118654752440

// Physical constants (Verilog-AMS LRM 2.4, Annex D)
// charge of electron in Coulombs
`define P_Q_SPICE       1.60219e-19
`define P_Q_OLD         1.6021918e-19
`define P_Q_NIST1998    1.602176462e-19
`define P_Q_NIST2010    1.602176565e-19
// speed of light in vacuum in meters/second
`define P_C             2.99792458e8
// Boltzmann's constant in Joules/Kelvin
`define P_K_SPICE       1.38062e-23
`define P_K_OLD         1.3806226e-23
`define P_K_NIST1998    1.3806503e-23
`define P_K_NIST2010    1.3806488e-23
// Planck's constant in Joules*second
`define P_H_SPICE       6.62620e-34
`define P_H_OLD         6.6260755e-34
`define P_H_NIST1998    6.62606876e-34
`define P_H_NIST2010    6.62606957e-34
// permittivity of vacuum in Farads/meter
`define P_EPS0_SPICE    8.854214871e-12
`define P_EPS0_OLD      8.85418792394420013968e-12
`define P_EPS0_NIST1998 8.854187817e-12
`define P_EPS0_NIST2010 8.854187817e-12
// permeability of vacuum in Henrys/meter
`define P_U0            (4.0e-7 * `M_PI)
// zero Celsius in Kelvin
`define P_CELSIUS0      273.15

`ifdef PHYSICAL_CONSTANTS_SPICE
// from UC Berkeley SPICE 3F5
`define P_Q             `P_Q_SPICE
`define P_K             `P_K_SPICE
`define P_H             `P_H_SPICE
`define P_EPS0          `P_EPS0_SPICE

`else
`ifdef PHYSICAL_CONSTANTS_OLD
// from Verilog-A LRM 1.0 and Verilog-AMS LRM 2.0
`define P_Q             `P_Q_OLD
`define P_K             `P_K_OLD
`define P_H             `P_H_OLD
`define P_EPS0          `P_EPS0_OLD

`else
`ifdef PHYSICAL_CONSTANTS_NIST2010
`define P_Q             `P_Q_NIST2010
`define P_K             `P_K_NIST2010
`define P_H             `P_H_NIST2010
`define P_EPS0          `P_EPS0_NIST2010

`else
// use NIST1998 values as in LRM 2.2 - 2.3 for backwards-compatibility
`define P_Q             `P_Q_NIST1998
`define P_K             `P_K_NIST1998
`define P_H             `P_H_NIST1998
`define P_EPS0          `P_EPS0_NIST1998
`endif
`endif
`endif

`endif
"#;

const BUILTIN_INCLUDE_DOCUMENTS: [(&str, &str); 2] = [
    ("@rspice/stdlib/disciplines.vams", DISCIPLINES_VAMS),
    ("@rspice/stdlib/constants.vams", CONSTANTS_VAMS),
];

const fn builtin_include_source_bytes() -> usize {
    let mut total = 0usize;
    let mut index = 0usize;
    while index < BUILTIN_INCLUDE_DOCUMENTS.len() {
        total = total.saturating_add(BUILTIN_INCLUDE_DOCUMENTS[index].1.len());
        index += 1;
    }
    total
}

/// Maximum number of distinct built-in include documents that preprocessing
/// can add to a dependency closure.
///
/// These compiler-owned documents are outside a project's virtual source
/// bundle and therefore receive a separate, fixed reservation in the sealed
/// virtual-source provider limits.
pub(crate) const BUILTIN_INCLUDE_DOCUMENT_COUNT: usize = BUILTIN_INCLUDE_DOCUMENTS.len();

/// Exact source-byte reservation for all distinct built-in include documents.
pub(crate) const BUILTIN_INCLUDE_SOURCE_BYTES: usize = builtin_include_source_bytes();

/// Resolve a built-in standard header by include filename.
///
/// Recognizes both the modern `.vams` names and the legacy `.h` names used
/// by older ADMS-era compact models.
pub fn builtin_include(filename: &str) -> Option<&'static str> {
    builtin_include_document(filename).map(|(_, source)| source)
}

/// Resolve a built-in standard header to its canonical logical identity and
/// exact versioned contents.
///
/// The canonical identity is independent of the alias used by source code,
/// making dependency receipts reproducible across desktop and WebAssembly.
pub fn builtin_include_document(filename: &str) -> Option<(&'static str, &'static str)> {
    // Compare against the basename so `include "vams/disciplines.vams" also
    // resolves when only the path prefix differs.
    let basename = filename
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(filename)
        .to_ascii_lowercase();

    match basename.as_str() {
        "disciplines.vams" | "disciplines.h" | "discipline.vams" | "discipline.h" => {
            Some(BUILTIN_INCLUDE_DOCUMENTS[0])
        }
        "constants.vams" | "constants.h" => Some(BUILTIN_INCLUDE_DOCUMENTS[1]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `P_Q`, `P_K`, `P_H` and `P_EPS0` as the built-in header expands them
    /// after `prelude`.
    fn physical_constants(prelude: &str) -> [f64; 4] {
        let source = format!("{prelude}\n{CONSTANTS_VAMS}\n`P_Q `P_K `P_H `P_EPS0\n");
        let expanded = crate::preprocessor::Preprocessor::new()
            .preprocess_source(&source)
            .expect("the built-in constants header preprocesses");
        let values = expanded
            .lines()
            .rev()
            .find(|line| !line.trim().is_empty())
            .expect("the expanded constants")
            .split_whitespace()
            .map(|value| value.parse::<f64>().expect("a constant"))
            .collect::<Vec<_>>();
        values.try_into().expect("four constants")
    }

    /// The built-in header is the one OpenVAF compiles against, so a shipped
    /// model generated through it takes OpenVAF's (and the OSDI reference's)
    /// constants: the LRM 2.4 default NIST 1998 set. It once mixed that set's
    /// `P_Q` with the LRM 1.0/2.0 `P_K`, `P_H` and `P_EPS0`, a set no LRM
    /// defines.
    #[test]
    fn physical_constants_are_the_lrm_default_set_openvaf_uses() {
        assert_eq!(
            physical_constants(""),
            [
                1.602176462e-19,
                1.3806503e-23,
                6.62606876e-34,
                8.854187817e-12
            ]
        );
        assert_eq!(
            physical_constants("`define PHYSICAL_CONSTANTS_NIST2010"),
            [
                1.602176565e-19,
                1.3806488e-23,
                6.62606957e-34,
                8.854187817e-12
            ]
        );
        let old_eps0 = "8.85418792394420013968e-12"
            .parse::<f64>()
            .expect("a constant");
        assert_eq!(
            physical_constants("`define PHYSICAL_CONSTANTS_OLD"),
            [1.6021918e-19, 1.3806226e-23, 6.6260755e-34, old_eps0]
        );
        assert_eq!(
            physical_constants("`define PHYSICAL_CONSTANTS_SPICE"),
            [1.60219e-19, 1.38062e-23, 6.62620e-34, 8.854214871e-12]
        );
    }

    #[test]
    fn resolves_standard_header_names() {
        assert!(builtin_include("disciplines.vams").is_some());
        assert!(builtin_include("constants.vams").is_some());
        assert!(builtin_include("discipline.h").is_some());
        assert!(builtin_include("constants.h").is_some());
        assert!(builtin_include("Disciplines.VAMS").is_some());
        assert!(builtin_include("path/to/disciplines.vams").is_some());
        assert!(builtin_include("mymodel.va").is_none());
    }
}
