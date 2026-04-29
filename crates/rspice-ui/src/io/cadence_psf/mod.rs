//! Cadence PSF native binary parsing.
//!
//! This module provides a local parser for Cadence/Spectre PSF binary payloads
//! so waveform import does not depend on external crates.

#![allow(clippy::too_many_arguments)]

mod binary_io;
mod cadence_psf_type_meta;
mod channels;
mod sections;
mod toc;
mod types;
mod value_decode;
mod values;

pub use self::types::{
    CadencePsfError, NamedComplexSignal, NamedRealSignal, ParsedCadencePsfBinary,
};

use self::binary_io::{
    parse_string, read_f64, read_i32, read_u8_padded, read_u32, skip_opaque_scalar,
};
use self::cadence_psf_type_meta::TypeMetaCache;
use self::channels::{
    pad_untouched_channels, push_named_channel_cached, resolve_array_element_type,
};
use self::types::{
    ArrayElementType, DataType, NumericSample, SignalChannel, SignalRef, SignalValues, TypeDecl,
    TypeKind,
};

use self::channels::qualify_signal_name;
use self::sections::{flatten_traces, parse_header, parse_sweeps, parse_traces, parse_types};
use self::toc::{SectionKind, parse_toc};
use self::values::parse_values;

pub fn parse_cadence_psf_binary(data: &[u8]) -> Result<ParsedCadencePsfBinary, CadencePsfError> {
    let toc = parse_toc(data)?;
    let header = parse_header(data, toc.section(SectionKind::Header)?)?;
    let types = parse_types(data, toc.section(SectionKind::Type)?)?;
    let sweeps = parse_sweeps(data, toc.section(SectionKind::Sweep)?)?;
    let traces = parse_traces(data, toc.section(SectionKind::Trace)?)?;
    let mut signal_values = parse_values(
        data,
        toc.section(SectionKind::Value)?,
        &header,
        &types,
        &sweeps,
        &traces,
    )?;

    let flat_traces = flatten_traces(&traces);
    let mut real_signals = Vec::new();
    let mut complex_signals = Vec::new();

    for signal in flat_traces {
        if let Some(channels) = signal_values.remove(&signal.id) {
            for channel in channels {
                let name = qualify_signal_name(&signal.name, &channel.suffix);
                match channel.values {
                    SignalValues::Real(values) => {
                        real_signals.push(NamedRealSignal { name, values })
                    }
                    SignalValues::Complex(values) => {
                        complex_signals.push(NamedComplexSignal { name, values })
                    }
                }
            }
        }
    }

    let mut sweep_signals = Vec::new();
    for sweep in &sweeps {
        if let Some(channels) = signal_values.remove(&sweep.id) {
            for channel in channels {
                if let SignalValues::Real(values) = channel.values {
                    sweep_signals.push(NamedRealSignal {
                        name: qualify_signal_name(&sweep.name, &channel.suffix),
                        values,
                    });
                }
            }
        }
    }

    Ok(ParsedCadencePsfBinary {
        header,
        sweeps: sweep_signals,
        real_signals,
        complex_signals,
    })
}
