//! Waveform interpretation over the existing bounded core SPICE RAW reader.

use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal};
use rspice_core::io::RawParseError;
use std::io::Cursor;

#[derive(Debug)]
pub enum RawReadError {
    Parse(RawParseError),
    NoVariables,
}

impl std::fmt::Display for RawReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(source) => source.fmt(f),
            Self::NoVariables => f.write_str("rawfile contains no variables"),
        }
    }
}

impl std::error::Error for RawReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(source) => Some(source),
            Self::NoVariables => None,
        }
    }
}

pub fn decode_spice_raw(
    bytes: &[u8],
    limits: rspice_core::ResourceLimits,
) -> Result<DecodedNumericDataset, RawReadError> {
    let parsed = rspice_core::io::parse_raw_reader_with_limits(&mut Cursor::new(bytes), limits)
        .map_err(RawReadError::Parse)?;
    let mut waveforms = parsed.waveforms.into_iter();
    let scale = waveforms.next().ok_or(RawReadError::NoVariables)?;
    let coordinate_name = scale.name;
    let coordinate = scale.y;
    let mut signals = Vec::new();
    for waveform in waveforms {
        signals.push(DecodedNumericSignal {
            name: waveform.name,
            real: waveform.y,
            imag: waveform.y_imag,
            unit: None,
        });
    }
    let plot = parsed.header.plotname.to_ascii_lowercase();
    let domain = if parsed.header.is_complex || plot.contains("ac") {
        crate::WaveformDomain::Ac
    } else if plot.contains("tran") || coordinate_name.to_ascii_lowercase().contains("time") {
        crate::WaveformDomain::Transient
    } else {
        crate::WaveformDomain::DcSweep
    };
    Ok(DecodedNumericDataset {
        domain,
        coordinate_name,
        coordinate,
        signals,
    })
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
        let mut bytes = format!("Title: fixture\nPlotname: {plot}\nFlags: {flags}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 {coordinate} time\n1 V(out) voltage\nBinary:\n").into_bytes();
        let adjacent = f64::from_bits(1.0_f64.to_bits() + 1);
        let values = if complex {
            vec![2.0, 99.0, adjacent, -0.0]
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
