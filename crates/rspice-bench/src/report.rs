//! Common immutable envelope for non-macro benchmark artifacts.

use crate::error::BenchError;
use crate::provenance::{self, HostInfo, ToolProvenance};
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[derive(Serialize)]
struct ArtifactEnvelope<'a, T: Serialize> {
    schema_version: u32,
    benchmark_id: &'static str,
    generated_at_unix_ms: u128,
    trusted: bool,
    tool: ToolProvenance,
    host: HostInfo,
    payload: &'a T,
}

/// Writes a no-clobber, same-directory atomic JSON artifact.
pub fn write<T: Serialize>(
    path: &Path,
    benchmark_id: &'static str,
    payload: &T,
    release_required: bool,
) -> Result<(), BenchError> {
    if path.exists() {
        return Err(BenchError::BenchmarkPolicy {
            message: format!(
                "report `{}` already exists; benchmark artifacts are immutable",
                path.display()
            ),
        });
    }
    let tool = provenance::tool();
    let trusted = (!release_required || tool.profile == "release")
        && tool.git_commit.is_some()
        && tool.git_dirty == Some(false);
    let envelope = ArtifactEnvelope {
        schema_version: 1,
        benchmark_id,
        generated_at_unix_ms: provenance::unix_time_ms(),
        trusted,
        tool,
        host: provenance::host(),
        payload,
    };
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty());
    if let Some(parent) = parent {
        fs::create_dir_all(parent).map_err(|error| {
            BenchError::io(
                format!("failed to create report directory `{}`", parent.display()),
                error,
            )
        })?;
    }
    let directory = parent.unwrap_or_else(|| Path::new("."));
    let mut temp = NamedTempFile::new_in(directory).map_err(|error| {
        BenchError::io(
            format!(
                "failed to create temporary report in `{}`",
                directory.display()
            ),
            error,
        )
    })?;
    serde_json::to_writer_pretty(temp.as_file_mut(), &envelope).map_err(|source| {
        BenchError::Json {
            context: format!("failed to serialize {benchmark_id} report"),
            source,
        }
    })?;
    temp.as_file_mut()
        .write_all(b"\n")
        .and_then(|()| temp.as_file_mut().flush())
        .and_then(|()| temp.as_file().sync_all())
        .map_err(|error| BenchError::io("failed to flush benchmark report", error))?;
    temp.persist_noclobber(path).map_err(|error| {
        BenchError::io(
            format!("failed to publish immutable report `{}`", path.display()),
            error.error,
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Payload {
        samples: Vec<f64>,
    }

    #[test]
    fn focused_reports_publish_once_with_the_common_envelope() {
        let directory = tempfile::tempdir().expect("temporary report directory");
        let path = directory.path().join("report.json");
        let payload = Payload {
            samples: vec![1.0, 2.0, 3.0],
        };
        write(&path, "rspice-klu", &payload, false).expect("first publication succeeds");

        let value: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).expect("published report remains readable"))
                .expect("published report is JSON");
        assert_eq!(value["schema_version"], 1);
        assert_eq!(value["benchmark_id"], "rspice-klu");
        assert_eq!(
            value["payload"]["samples"],
            serde_json::json!([1.0, 2.0, 3.0])
        );
        assert!(value["tool"]["executable_blake3"].is_string());
        assert!(value["host"]["fingerprint"].is_string());

        let original = fs::read(&path).expect("capture immutable bytes");
        let error = write(&path, "rspice-klu", &payload, false)
            .expect_err("second publication cannot clobber evidence");
        assert!(error.to_string().contains("already exists"));
        assert_eq!(fs::read(&path).expect("report still exists"), original);
    }
}
