//! Waveform interpretation over the existing bounded core SPICE RAW reader.

use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal};
use rspice_core::io::RawParseError;
use std::io::Cursor;

#[derive(Debug)]
pub enum RawReadError {
    Parse(RawParseError),
    NoVariables,
    Coordinate(String),
}

impl std::fmt::Display for RawReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(source) => source.fmt(f),
            Self::NoVariables => f.write_str("rawfile contains no variables"),
            Self::Coordinate(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for RawReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(source) => Some(source),
            Self::NoVariables | Self::Coordinate(_) => None,
        }
    }
}

pub fn decode_spice_raw(
    bytes: &[u8],
    limits: rspice_core::ResourceLimits,
) -> Result<DecodedNumericDataset, RawReadError> {
    let parsed = rspice_core::io::parse_raw_reader_with_limits(&mut Cursor::new(bytes), limits)
        .map_err(RawReadError::Parse)?;
    parsed
        .validate_real_coordinate()
        .map_err(RawReadError::Parse)?;
    let ordinal = parsed.has_ordinal_axis().map_err(RawReadError::Parse)?;
    let domain = coordinate_domain(&parsed, ordinal)?;
    let units = rspice_core::io::ltspice_raw::raw_table_units(&parsed.header)
        .map_err(RawReadError::Parse)?
        .unwrap_or_else(|| {
            let mut units = vec![None; parsed.variables.len()];
            if !ordinal && let Some(coordinate) = parsed.variables.first() {
                // Ordinary SPICE RAW declares SI quantities rather than a
                // separate unit. Explicit table metadata above may instead
                // declare an unknown unit and must never inherit this fallback.
                let unit = match coordinate.var_type.trim().to_ascii_lowercase().as_str() {
                    "time" => Some("s"),
                    "frequency" => Some("Hz"),
                    "voltage" => Some("V"),
                    "current" => Some("A"),
                    "index" => Some("1"),
                    _ => match domain {
                        crate::WaveformDomain::Transient => Some("s"),
                        crate::WaveformDomain::Ac => Some("Hz"),
                        crate::WaveformDomain::DcSweep => None,
                    },
                };
                units[0] = unit.map(str::to_owned);
            }
            units
        });
    let mut waveforms = parsed.waveforms.into_iter().zip(units).peekable();
    let (coordinate_name, coordinate, coordinate_unit) = if ordinal {
        let (first, _) = waveforms.peek().ok_or(RawReadError::NoVariables)?;
        (
            "point".to_owned(),
            (0..first.y.len()).map(|index| index as f64).collect(),
            Some("1".to_owned()),
        )
    } else {
        let (scale, unit) = waveforms.next().ok_or(RawReadError::NoVariables)?;
        (scale.name, scale.y, unit)
    };
    let mut signals = Vec::new();
    for (waveform, unit) in waveforms {
        signals.push(DecodedNumericSignal {
            name: waveform.name,
            real: waveform.y,
            imag: waveform.y_imag,
            unit,
        });
    }
    let mut dataset = DecodedNumericDataset {
        domain,
        coordinate_name,
        coordinate_unit,
        coordinate,
        signals,
    };
    dataset
        .normalize_coordinate_unit()
        .map_err(RawReadError::Coordinate)?;
    Ok(dataset)
}

fn coordinate_domain(
    parsed: &rspice_core::io::ltspice_raw::RawWaveformData,
    ordinal: bool,
) -> Result<crate::WaveformDomain, RawReadError> {
    use crate::WaveformDomain::{Ac, DcSweep, Transient};
    if ordinal {
        return Ok(DcSweep);
    }
    let coordinate = parsed.variables.first().ok_or(RawReadError::NoVariables)?;
    // The declared independent quantity outranks display titles and the
    // storage representation of dependent signals. Sweeps can be complex.
    let declared = match coordinate.var_type.trim().to_ascii_lowercase().as_str() {
        "time" => Some(Transient),
        "frequency" => Some(Ac),
        "index" | "voltage" | "current" | "temperature" | "resistance" | "capacitance"
        | "inductance" | "power" | "conductance" => Some(DcSweep),
        _ => None,
    };
    if let Some(domain) = declared {
        return Ok(domain);
    }
    match coordinate.name.trim().to_ascii_lowercase().as_str() {
        "time" | "t" | "timestamp" => return Ok(Transient),
        "frequency" | "freq" | "hz" => return Ok(Ac),
        "point" | "index" => return Ok(DcSweep),
        _ => {}
    }
    if rspice_core::io::ltspice_raw::raw_table_has_coordinate(&parsed.header)
        .map_err(RawReadError::Parse)?
    {
        return Ok(DcSweep);
    }
    // Legacy sources sometimes omit a meaningful coordinate declaration.
    // Match analysis titles as titles, not substrings such as "ac" in "package".
    Ok(
        match parsed.header.plotname.trim().to_ascii_lowercase().as_str() {
            "transient analysis" | "transient" | "tran" => Transient,
            "ac analysis" | "ac" | "noise spectral density" | "distortion analysis" => Ac,
            "dc transfer characteristic" | "dc sweep" | "dc analysis" => DcSweep,
            _ if parsed.header.is_complex => Ac,
            _ => DcSweep,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binary_fixture(plot: &str, coordinate: &str, complex: bool) -> Vec<u8> {
        let flags = if complex {
            "complex double"
        } else {
            "real double"
        };
        let kind = match coordinate.to_ascii_lowercase().as_str() {
            "time" => "time",
            "frequency" => "frequency",
            _ => "value",
        };
        let mut bytes = format!("Title: fixture\nPlotname: {plot}\nFlags: {flags}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 {coordinate} {kind}\n1 V(out) voltage\nBinary:\n").into_bytes();
        let adjacent = f64::from_bits(1.0_f64.to_bits() + 1);
        let values = if complex {
            vec![2.0, 0.0, adjacent, -0.0]
        } else {
            vec![2.0, adjacent]
        };
        for value in values {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    #[test]
    fn core_raw_projection_preserves_domain_rules_and_exact_samples() {
        use crate::WaveformDomain::{Ac, DcSweep, Transient};
        for (plot, coordinate, complex, domain) in [
            ("unknown", "x", false, DcSweep),
            ("Transient Analysis", "x", false, Transient),
            ("unknown", "TIME", false, Transient),
            ("AC Analysis", "frequency", false, Ac),
            ("unknown", "x", true, Ac),
            ("package transfer", "x", false, DcSweep),
            ("Transient Analysis", "x", true, Transient),
            ("DC transfer characteristic", "x", true, DcSweep),
        ] {
            let bytes = binary_fixture(plot, coordinate, complex);
            let decoded = decode_spice_raw(&bytes, Default::default()).unwrap();
            assert_eq!(decoded.domain, domain);
            assert_eq!(decoded.coordinate_name, coordinate);
            assert_eq!(decoded.coordinate, [2.0]);
            assert_eq!(decoded.signals.len(), 1);
            assert_eq!(decoded.signals[0].name, "V(out)");
            assert_eq!(decoded.signals[0].real[0].to_bits(), 1.0_f64.to_bits() + 1);
            assert!(decoded.signals[0].unit.is_none());
            if complex {
                assert_eq!(
                    decoded.signals[0].imag.as_ref().unwrap()[0].to_bits(),
                    (-0.0_f64).to_bits()
                );
            } else {
                assert!(decoded.signals[0].imag.is_none());
            }
        }
    }

    #[test]
    fn core_raw_refusals_retain_the_parser_and_resource_cause() {
        use std::error::Error as _;
        let bytes = binary_fixture("unknown", "x", false);
        let mut limits = rspice_core::ResourceLimits::default();
        limits.max_external_data_bytes = bytes.len() - 1;
        let error = decode_spice_raw(&bytes, limits).unwrap_err();
        assert!(matches!(
            &error,
            RawReadError::Parse(RawParseError::ResourceLimit(_))
        ));
        let source = error.source().unwrap();
        assert!(source.is::<RawParseError>());
        assert_eq!(error.to_string(), source.to_string());
        let error = decode_spice_raw(b"invalid", Default::default()).unwrap_err();
        assert!(matches!(&error, RawReadError::Parse(_)));
        assert!(error.source().unwrap().is::<RawParseError>());
    }
}
