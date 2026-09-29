//! Deterministic native RSpice waveform bundle publication.
//!
//! Both public bundle identities deliberately share one embedded waveform
//! dataset schema. The manifest schema distinguishes the artifact contract;
//! its digest binds the exact canonical `dataset.json` bytes.

mod reader;
#[cfg(feature = "result-waveform")]
pub mod result;
mod writer;

pub use reader::{NativeBundleReadLimits, decode_native_bundle};
pub use writer::encode_native_bundle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeBundleKind {
    Result,
    Dataset,
}

impl NativeBundleKind {
    pub const fn manifest_schema(self) -> &'static str {
        match self {
            Self::Result => "rspice-result-bundle/1",
            Self::Dataset => "rspice-dataset-bundle/1",
        }
    }

    pub const fn extension(self) -> &'static str {
        match self {
            Self::Result => "rspiceresult",
            Self::Dataset => "rspicedata",
        }
    }

    pub const fn media_type(self) -> &'static str {
        match self {
            Self::Result => "application/vnd.rspice.result+zip",
            Self::Dataset => "application/vnd.rspice.dataset+zip",
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Result => "RSpice Result Bundle",
            Self::Dataset => "RSpice Dataset Bundle",
        }
    }
}

#[derive(Debug)]
pub struct NativeBundleDataset<'a> {
    pub analysis: crate::WaveformDomain,
    pub coordinate_name: &'a str,
    pub coordinate: &'a [f64],
    pub signals: Vec<NativeBundleSignal<'a>>,
}

#[derive(Debug)]
pub struct NativeBundleSignal<'a> {
    pub name: &'a str,
    pub unit: Option<&'a str>,
    pub values: NativeBundleSignalValues<'a>,
}

#[derive(Debug)]
pub enum NativeBundleSignalValues<'a> {
    Real(&'a [f64]),
    Complex { real: &'a [f64], imag: &'a [f64] },
}

/// A native bundle validation or encoding/decoding failure.
#[derive(Debug)]
pub enum NativeBundleError {
    InvalidData(String),
    AnalysisDomain(crate::UnsupportedWaveformDomain),
    SignalRepresentation {
        name: String,
        values: bool,
        real: bool,
        imag: bool,
    },
    Json {
        context: &'static str,
        source: serde_json::Error,
    },
    Zip {
        context: String,
        source: zip::result::ZipError,
    },
    Io {
        context: String,
        source: std::io::Error,
    },
}

impl std::fmt::Display for NativeBundleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidData(detail) => formatter.write_str(detail),
            Self::AnalysisDomain(source) => source.fmt(formatter),
            Self::SignalRepresentation { name, .. } => write!(
                formatter,
                "signal '{name}' must provide either values or both real and imag"
            ),
            Self::Json { context, source } => write!(formatter, "{context}: {source}"),
            Self::Zip { context, source } => write!(formatter, "{context}: {source}"),
            Self::Io { context, source } => write!(formatter, "{context}: {source}"),
        }
    }
}

impl std::error::Error for NativeBundleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidData(_) | Self::SignalRepresentation { .. } => None,
            Self::AnalysisDomain(source) => Some(source),
            Self::Json { source, .. } => Some(source),
            Self::Zip { source, .. } => Some(source),
            Self::Io { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests;
