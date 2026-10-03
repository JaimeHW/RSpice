//! Retained and projected waveform units and value formatting.

use super::TraceKind;
use rspice_results::analysis_type::AnalysisType;
use rspice_ui_kit::plot::{fmt_si_significant, fmt_significant};

/// Case-insensitive prefix test for a signal-name accessor (`V(`, `i(`).
fn starts_with_accessor(name: &str, accessor: &str) -> bool {
    name.get(..accessor.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(accessor))
}

/// Peel the derived-projection wrappers off a display name so the
/// underlying accessor is visible: `re(V(out))` is still volts.
fn unwrap_projection(name: &str) -> &str {
    let name = name.trim_start();
    for wrapper in ["re(", "im(", "mag(", "abs("] {
        if starts_with_accessor(name, wrapper) {
            return name[wrapper.len()..].trim_start();
        }
    }
    name.strip_prefix('|').unwrap_or(name)
}

/// The engineering unit a trace is measured in.
///
/// This is the axis model: a signal owns its unit, and the analysis default
/// applies only where the name carries no accessor to read it from. Without
/// this, a transient strip carrying `V(out)` and `I(R1)` would label both
/// against the analysis' nominal volts and quietly misreport the current.
/// The noise sheet reports amplitude density in a unit that already carries
/// its own SI prefix.
pub const NOISE_DENSITY_UNIT: &str = "nV/√Hz";

/// Format a value in a pane's unit.
///
/// A unit that already carries an SI prefix takes no second one: 1.79 µV/√Hz
/// of output noise reads as `1786.13 nV/√Hz`, never as `1.78613 knV/√Hz`.
pub fn fmt_in_unit(value: f64, unit: &str, significant_digits: usize) -> String {
    if matches!(unit, NOISE_DENSITY_UNIT | "nA/√Hz" | "ns/√Hz") {
        return fmt_significant(value, significant_digits, &format!(" {unit}"));
    }
    fmt_si_significant(value, unit, significant_digits)
}

/// The projections a trace kind performs own their own unit: a decibel
/// magnitude, a phase, and an amplitude-density projection are all computed
/// here, so the source series' retained unit does not describe them.
/// Everything else reads the source directly and takes its unit.
pub(super) fn signal_unit<'a>(
    name: &str,
    kind: TraceKind,
    retained_unit: Option<&'a str>,
    analysis_unit: &'a str,
) -> &'a str {
    match kind {
        TraceKind::MagnitudeDb if retained_unit == Some("ratio") => "dBc",
        TraceKind::MagnitudeDb => "dB",
        TraceKind::PhaseDeg => "°",
        TraceKind::PhaseRad => "rad",
        TraceKind::NoiseDensity => match retained_unit {
            Some("A²/Hz" | "A^2/Hz") => "nA/√Hz",
            Some("s²/Hz" | "s^2/Hz") => "ns/√Hz",
            _ => NOISE_DENSITY_UNIT,
        },
        TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
            quantity_unit(name, retained_unit, analysis_unit)
        }
    }
}

/// The unit a raw retained series reads in.
///
/// The producer's own statement is the best evidence there is, so it is read
/// first. Only a waveform that states none — every project written before the
/// unit was retained, and every series a conversion invents rather than
/// carries — falls back to its name's accessor and then to the analysis'
/// nominal quantity.
fn quantity_unit<'a>(
    name: &str,
    retained_unit: Option<&'a str>,
    analysis_unit: &'a str,
) -> &'a str {
    if let Some(unit) = retained_unit {
        return unit;
    }
    let name = unwrap_projection(name);
    if starts_with_accessor(name, "i(") {
        "A"
    } else if starts_with_accessor(name, "v(") {
        "V"
    } else if starts_with_accessor(name, "p(") {
        "W"
    } else {
        analysis_unit
    }
}

/// The unit a retained waveform reads in when it states none of its own and
/// its name declares no accessor — the analysis' own quantity.
///
/// This is the last resort of the chain and the one owner of that mapping.
/// Every surface that labels a raw dataset signal reads it through
/// [`browser_signal_unit`]; a surface that guesses instead will label a noise
/// density or a decibel magnitude as volts.
pub const fn analysis_default_unit(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::Pstb => "",
        kind if kind.is_bode_response() => "dB",
        AnalysisType::Noise | AnalysisType::Pnoise | AnalysisType::Hbnoise => "V^2/Hz",
        _ => "V",
    }
}

/// Unit for a raw dataset waveform in the results data browser, which carries
/// no trace-kind projection: the retained unit decides, then the name's
/// accessor however wrapped, and the analysis default applies only where
/// there is neither.
///
/// Pass `None` for a signal that has no retained waveform behind it — a saved
/// expression, say, whose unit only its text can suggest.
pub fn browser_signal_unit<'a>(
    name: &str,
    retained_unit: Option<&'a str>,
    analysis_unit: &'a str,
) -> &'a str {
    let name = name.trim_start();
    if retained_unit.is_none() && starts_with_accessor(name, "phase(") {
        return "°";
    }
    quantity_unit(name, retained_unit, analysis_unit)
}

/// Whether a raw dataset waveform name reads a current, however wrapped.
pub fn browser_signal_is_current(name: &str) -> bool {
    let name = name.trim_start();
    let name = if starts_with_accessor(name, "phase(") {
        name["phase(".len()..].trim_start()
    } else {
        name
    };
    starts_with_accessor(unwrap_projection(name), "i(")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_prefixed_unit_never_takes_a_second_prefix() {
        // Output noise of a gain-of-1000 stage is micro-volts per root hertz; in
        // a unit that already says "nano" that must stay a plain number.
        assert_eq!(
            fmt_in_unit(1786.131, NOISE_DENSITY_UNIT, 6),
            "1786.13 nV/√Hz"
        );
        assert_eq!(fmt_in_unit(4.07, NOISE_DENSITY_UNIT, 3), "4.07 nV/√Hz");
        // Base units keep their SI scaling.
        assert_eq!(fmt_in_unit(1.5e-3, "A", 3), "1.50 mA");
    }

    #[test]
    fn a_signal_owns_its_unit_rather_than_inheriting_the_analysis_default() {
        // The accessor in the name is authoritative where the run retained no
        // unit of its own.
        assert_eq!(signal_unit("V(out)", TraceKind::Value, None, "V"), "V");
        assert_eq!(signal_unit("I(R1)", TraceKind::Value, None, "V"), "A");
        assert_eq!(signal_unit("i(vsense)", TraceKind::Value, None, "V"), "A");
        assert_eq!(signal_unit("P(M1)", TraceKind::Value, None, "V"), "W");

        // Derived projections keep the underlying signal's unit.
        assert_eq!(signal_unit("re(V(out))", TraceKind::Real, None, ""), "V");
        assert_eq!(
            signal_unit("im(I(R1))", TraceKind::Imaginary, None, ""),
            "A"
        );

        // The analysis default applies only where neither the run nor the name
        // carries anything to read a unit from.
        assert_eq!(
            signal_unit("onoise", TraceKind::Value, None, "V^2/Hz"),
            "V^2/Hz"
        );

        // A retained unit outranks both: the producer measured the samples and
        // said so, and no name convention covers a violation count or a cost.
        assert_eq!(
            signal_unit("SOA_VIOLATION_COUNT", TraceKind::Value, Some("count"), "V"),
            "count"
        );
        assert_eq!(
            signal_unit("OPT_COST", TraceKind::Value, Some("cost"), "V"),
            "cost"
        );

        // Derived kinds have their own units regardless of the source.
        assert_eq!(
            signal_unit("V(out)", TraceKind::MagnitudeDb, None, "V"),
            "dB"
        );
        assert_eq!(signal_unit("V(out)", TraceKind::PhaseDeg, None, "V"), "°");
        assert_eq!(
            signal_unit("V(out)", TraceKind::MagnitudeDb, Some("V"), "V"),
            "dB",
            "a dB projection is computed here and is not in the source's unit"
        );
        assert_eq!(
            signal_unit("V(out) 2f1/F1", TraceKind::MagnitudeDb, Some("ratio"), "V"),
            "dBc",
            "a distortion product ratio projects relative to its retained F1 reference"
        );
        assert_eq!(
            signal_unit("onoise", TraceKind::NoiseDensity, Some("V^2/Hz"), "V^2/Hz"),
            NOISE_DENSITY_UNIT
        );
    }

    #[test]
    fn browser_classification_reads_the_accessor_through_derived_wrappers() {
        for (name, unit, current) in [
            ("V(out)", "V", false),
            ("I(C1)", "A", true),
            ("|V(OUT)|", "V", false),
            ("|I(VIN)|", "A", true),
            ("phase(V(OUT))", "°", false),
            ("phase(I(VIN))", "°", true),
            ("re(I(R1))", "A", true),
            ("V(段.data#3)", "V", false),
            ("I(段)", "A", true),
            ("re(I(段))", "A", true),
            ("phase(V(段))", "°", false),
            ("v(µ)", "V", false),
            ("段", "V^2/Hz", false),
            ("🧪", "V^2/Hz", false),
            ("onoise", "V^2/Hz", false),
        ] {
            assert_eq!(browser_signal_unit(name, None, "V^2/Hz"), unit, "{name}");
            assert_eq!(browser_signal_is_current(name), current, "{name}");
        }
    }

    #[test]
    fn browser_reads_the_retained_unit_before_it_guesses_from_the_name() {
        // The defect this closes: a retained count and a retained cost carry no
        // accessor, so the browser filed both under the analysis default — volts.
        assert_eq!(
            browser_signal_unit("SOA_VIOLATION_COUNT", Some("count"), "V"),
            "count"
        );
        assert_eq!(browser_signal_unit("OPT_COST", Some("cost"), "V"), "cost");
        assert_eq!(browser_signal_unit("THD(%)", Some("%"), "dB"), "%");

        // A waveform that states nothing — every project written before the unit
        // was retained — reads exactly as it did before.
        assert_eq!(browser_signal_unit("SOA_VIOLATION_COUNT", None, "V"), "V");
        assert_eq!(browser_signal_unit("V(out)", None, "V"), "V");

        // A stated unit is the producer's measurement of its own samples, so it
        // outranks the name — including the phase convention.
        assert_eq!(browser_signal_unit("I(R1)", Some("A"), "V"), "A");
        assert_eq!(browser_signal_unit("phase(V(out))", Some("°"), "dB"), "°");
        assert_eq!(browser_signal_unit("phase(V(out))", None, "dB"), "°");
    }
}
