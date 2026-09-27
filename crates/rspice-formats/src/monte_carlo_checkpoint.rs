//! Bounded portable Monte Carlo checkpoint files with canonical evidence validation.

use rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointEvidence;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointFile {
    format: String,
    version: u32,
    checkpoint: MonteCarloCheckpointEvidence,
}

#[derive(Debug)]
pub enum CheckpointFileError {
    SizeLimit,
    Encode(serde_json::Error),
    InvalidDocument(serde_json::Error),
    UnsupportedFormat,
}

impl std::fmt::Display for CheckpointFileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SizeLimit => formatter.write_str("Checkpoint file exceeds its size limit"),
            Self::Encode(error) => std::fmt::Display::fmt(error, formatter),
            Self::InvalidDocument(error) => write!(formatter, "Invalid checkpoint file: {error}"),
            Self::UnsupportedFormat => {
                formatter.write_str("Unsupported Monte Carlo checkpoint file format or version")
            }
        }
    }
}

impl std::error::Error for CheckpointFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encode(error) | Self::InvalidDocument(error) => Some(error),
            Self::SizeLimit | Self::UnsupportedFormat => None,
        }
    }
}

pub fn size_limit() -> usize {
    MonteCarloCheckpointEvidence::byte_limit()
        .div_ceil(3)
        .saturating_mul(4)
        .saturating_add(4096)
}

pub fn encode(checkpoint: &MonteCarloCheckpointEvidence) -> Result<Vec<u8>, CheckpointFileError> {
    serde_json::to_vec(&CheckpointFile {
        format: "rspice.monte-carlo-checkpoint".into(),
        version: 1,
        checkpoint: checkpoint.clone(),
    })
    .map_err(CheckpointFileError::Encode)
}

pub fn decode(source: &str) -> Result<MonteCarloCheckpointEvidence, CheckpointFileError> {
    if source.len() > size_limit() {
        return Err(CheckpointFileError::SizeLimit);
    }
    let file: CheckpointFile =
        serde_json::from_str(source).map_err(CheckpointFileError::InvalidDocument)?;
    if file.format != "rspice.monte-carlo-checkpoint" || file.version != 1 {
        return Err(CheckpointFileError::UnsupportedFormat);
    }
    Ok(file.checkpoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn invalid_documents_retain_the_json_error_and_diagnostic_prefix() {
        for source in [
            "",
            "{",
            "null",
            "{}",
            r#"{"format":"rspice.monte-carlo-checkpoint","version":1}"#,
        ] {
            let error = decode(source).unwrap_err();
            assert!(matches!(error, CheckpointFileError::InvalidDocument(_)));
            assert!(error.source().is_some());
            assert!(error.to_string().starts_with("Invalid checkpoint file: "));
        }
    }
}
