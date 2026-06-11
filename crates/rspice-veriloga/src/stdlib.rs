//! Built-in Verilog-AMS standard headers
//!
//! Compact models universally start with `include "disciplines.vams"` and
//! `include "constants.vams"`. When those files are not found on disk, the
//! preprocessor falls back to these built-in copies so that models compile
//! out of the box, matching the behavior of commercial simulators that ship
//! the standard headers.
//!
//! Constant values follow the Verilog-AMS LRM 2.4 `constants.vams` so that
//! models calibrated against other LRM-compliant simulators reproduce the
//! same numbers.

/// Standard `disciplines.vams` header (Verilog-AMS LRM 2.4, Annex D).
pub const DISCIPLINES_VAMS: &str = r#"
`ifdef DISCIPLINES_VAMS
`else
`define DISCIPLINES_VAMS 1

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

/// Standard `constants.vams` header (Verilog-AMS LRM 2.4, Annex D).
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

// Physical constants (per Verilog-AMS LRM 2.4)
// Charge of an electron in coulombs
`define P_Q         1.602176462e-19
// Speed of light in a vacuum in meters/second
`define P_C         2.99792458e8
// Boltzmann's constant in joules/kelvin
`define P_K         1.3806226e-23
// Planck's constant in joules*second
`define P_H         6.6260755e-34
// Permittivity of vacuum in farads/meter
`define P_EPS0      8.85418792394420013968e-12
// Permeability of vacuum in henrys/meter
`define P_U0        (4.0e-7 * `M_PI)
// Zero Celsius in kelvin
`define P_CELSIUS0  273.15

`endif
"#;

/// Resolve a built-in standard header by include filename.
///
/// Recognizes both the modern `.vams` names and the legacy `.h` names used
/// by older ADMS-era compact models.
pub fn builtin_include(filename: &str) -> Option<&'static str> {
    // Compare against the basename so `include "vams/disciplines.vams" also
    // resolves when only the path prefix differs.
    let basename = filename
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(filename)
        .to_ascii_lowercase();

    match basename.as_str() {
        "disciplines.vams" | "disciplines.h" | "discipline.vams" | "discipline.h" => {
            Some(DISCIPLINES_VAMS)
        }
        "constants.vams" | "constants.h" => Some(CONSTANTS_VAMS),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
