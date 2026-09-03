//! `rspice-circuit-v1` document interpretation and include binding.
//!
//! The cloud control plane treats revision documents as opaque bounded JSON;
//! this executor owns the schema. Version 1 carries at most one content
//! source: a verbatim UTF-8 SPICE deck in `netlist_utf8`, or the reserved
//! empty `components` form used by the deterministic release smoke request.
//! Everything in the document is customer-controlled: schema violations are
//! canonical `status: failed` outcomes, never process faults.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::wire::{EngineArtifact, decode_digest};

/// Reserved include name under which an optionally configured built-in
/// model-library bundle (`RSPICE_ENGINE_MODEL_LIBRARY_PATH`) is exposed to
/// decks. The name matches the canonical component file name the production
/// worker image binds.
pub const MODEL_LIBRARY_INCLUDE_NAME: &str = "rspice-model-library";

/// The only revision-document schema this build interprets.
pub const CIRCUIT_DOCUMENT_SCHEMA: &str = "rspice-circuit-v1";

/// A bounded customer-facing rejection: becomes a `status: failed` response.
pub struct CircuitRejection {
    pub failure_code: &'static str,
    pub failure_detail: String,
}

impl CircuitRejection {
    fn new(failure_code: &'static str, failure_detail: impl Into<String>) -> Self {
        Self {
            failure_code,
            failure_detail: failure_detail.into(),
        }
    }
}

/// The interpreted content of a validated `rspice-circuit-v1` document.
pub enum CircuitContent {
    /// The reserved empty-circuit form: no elements, no analyses to run.
    Empty,
    /// A SPICE deck with every include already resolved from bound sources.
    Deck { expanded_netlist: String },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CircuitDocumentV1 {
    schema: String,
    #[serde(default)]
    components: Option<Vec<Value>>,
    #[serde(default)]
    netlist_utf8: Option<String>,
}

/// Validates the document envelope and resolves every `.include` against the
/// request's materialized artifacts and the optional built-in model bundle.
///
/// Include resolution never touches the filesystem outside the verified
/// sources handed in: a deck can only reference bytes whose identity the
/// worker already committed to the revision digest, or the byte-verified
/// model bundle the deployment binds. Anything else fails closed.
pub fn interpret_document(
    document: &Value,
    include_sources: &IncludeSources,
) -> Result<CircuitContent, CircuitRejection> {
    let document: CircuitDocumentV1 = serde_json::from_value(document.clone()).map_err(|_| {
        CircuitRejection::new(
            "document.invalid",
            "The circuit document is not a valid rspice-circuit-v1 object.",
        )
    })?;
    if document.schema != CIRCUIT_DOCUMENT_SCHEMA {
        return Err(CircuitRejection::new(
            "document.unsupported_schema",
            format!(
                "This engine supports circuit documents with schema {CIRCUIT_DOCUMENT_SCHEMA}; \
                 the revision declares {:?}.",
                document.schema
            ),
        ));
    }
    if document.components.as_ref().is_some_and(|c| !c.is_empty()) {
        return Err(CircuitRejection::new(
            "document.components_unsupported",
            "Structured component documents are not supported by this engine build; \
             supply the circuit as netlist_utf8.",
        ));
    }
    let Some(netlist) = document.netlist_utf8 else {
        return Ok(CircuitContent::Empty);
    };
    if netlist.is_empty() {
        return Err(CircuitRejection::new(
            "document.netlist_empty",
            "netlist_utf8 must not be empty; omit it entirely for an empty circuit.",
        ));
    }
    if netlist.contains('\0') {
        return Err(CircuitRejection::new(
            "document.netlist_invalid",
            "netlist_utf8 must not contain NUL bytes.",
        ));
    }
    let expanded_netlist = expand_includes(&netlist, include_sources)?;
    Ok(CircuitContent::Deck { expanded_netlist })
}

/// Byte-verified include sources: request artifacts by exact file name plus
/// the optional deployment-bound model bundle.
pub struct IncludeSources {
    sources: HashMap<String, String>,
}

impl IncludeSources {
    /// Reads and re-verifies every artifact the request manifests, plus the
    /// optional model bundle. The worker already streamed and hashed these
    /// bytes; hashing them again here means a corrupted or swapped file
    /// between materialization and launch cannot reach the parser.
    ///
    /// I/O or verification failure is an environment fault (`Err(String)`),
    /// not a customer outcome: the worker guaranteed these files.
    pub fn bind(
        artifacts: &[EngineArtifact],
        model_library_path: Option<&Path>,
    ) -> Result<Self, String> {
        let mut sources = HashMap::new();
        for artifact in artifacts {
            let bytes = std::fs::read(&artifact.path)
                .map_err(|error| format!("artifact {} is unreadable: {error}", artifact.path))?;
            if bytes.len() as u64 != artifact.size_bytes {
                return Err(format!(
                    "artifact {} is {} bytes; the manifest committed to {}",
                    artifact.path,
                    bytes.len(),
                    artifact.size_bytes
                ));
            }
            let expected = decode_digest(&artifact.sha256)
                .ok_or_else(|| format!("artifact {} digest is malformed", artifact.path))?;
            let actual: [u8; 32] = Sha256::digest(&bytes).into();
            if actual != expected {
                return Err(format!(
                    "artifact {} bytes do not match their manifested SHA-256",
                    artifact.path
                ));
            }
            let Some(file_name) = artifact.file_name.clone() else {
                continue;
            };
            let Ok(text) = String::from_utf8(bytes) else {
                // Binary attachments simply are not include-addressable;
                // referencing one from the deck reports include_unresolved.
                continue;
            };
            sources.insert(file_name, text);
        }
        if let Some(path) = model_library_path {
            let bytes = std::fs::read(path)
                .map_err(|error| format!("configured model library is unreadable: {error}"))?;
            let text = String::from_utf8(bytes)
                .map_err(|_| "configured model library is not UTF-8".to_owned())?;
            sources.insert(MODEL_LIBRARY_INCLUDE_NAME.to_owned(), text);
        }
        Ok(Self { sources })
    }

    fn resolve(&self, name: &str) -> Option<&str> {
        self.sources.get(name).map(String::as_str)
    }
}

/// Expands `.include`/`.inc` directives from bound sources only, one level
/// deep. Nested includes are rejected rather than recursively expanded so the
/// expansion is bounded and its cost is proportional to the manifested bytes.
/// A `.lib` directive that names a bound file is rejected with guidance —
/// silently treating it as a whole-file include would change its sectioned
/// semantics — while inline `.lib section` blocks pass through to the parser.
fn expand_includes(netlist: &str, sources: &IncludeSources) -> Result<String, CircuitRejection> {
    let mut expanded = String::with_capacity(netlist.len());
    for line in netlist.split_inclusive('\n') {
        let Some(directive) = include_directive(line) else {
            if lib_file_reference(line, sources) {
                return Err(CircuitRejection::new(
                    "netlist.lib_unsupported",
                    "File-referencing .lib directives are not supported; \
                     use .include for attached model files.",
                ));
            }
            expanded.push_str(line);
            continue;
        };
        let Some(content) = sources.resolve(directive) else {
            return Err(CircuitRejection::new(
                "netlist.include_unresolved",
                format!(
                    "The deck includes {directive:?}, which is not an attached \
                     revision artifact of this run."
                ),
            ));
        };
        if content
            .lines()
            .any(|nested| include_directive(nested).is_some())
        {
            return Err(CircuitRejection::new(
                "netlist.include_nested",
                format!("Included file {directive:?} contains a nested include directive."),
            ));
        }
        // Delimit the splice so a file without a trailing newline cannot
        // fuse with the next deck line, and so diagnostics stay attributable.
        writeln!(expanded, "* begin include {directive}").expect("writing to String cannot fail");
        expanded.push_str(content);
        if !content.ends_with('\n') {
            expanded.push('\n');
        }
        writeln!(expanded, "* end include {directive}").expect("writing to String cannot fail");
    }
    Ok(expanded)
}

/// Parses a `.include`/`.inc` line, returning the referenced name.
fn include_directive(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let lower = trimmed.to_ascii_lowercase();
    let rest = if let Some(rest) = lower.strip_prefix(".include") {
        &trimmed[".include".len()..][..rest.len()]
    } else if let Some(rest) = lower.strip_prefix(".inc") {
        &trimmed[".inc".len()..][..rest.len()]
    } else {
        return None;
    };
    if !rest.starts_with([' ', '\t']) {
        return None;
    }
    let name = rest.trim().trim_end_matches(['\r', '\n']).trim();
    let name = name
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
        .or_else(|| {
            name.strip_prefix('\'')
                .and_then(|inner| inner.strip_suffix('\''))
        })
        .unwrap_or(name);
    (!name.is_empty()).then_some(name)
}

/// Detects a `.lib` line whose first argument names a bound include source,
/// which is the file-referencing (unsupported) form rather than an inline
/// section marker.
fn lib_file_reference(line: &str, sources: &IncludeSources) -> bool {
    let trimmed = line.trim_start();
    if !trimmed.to_ascii_lowercase().starts_with(".lib") {
        return false;
    }
    let rest = trimmed[".lib".len()..].trim();
    let first = rest
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches(['"', '\'']);
    !first.is_empty() && sources.resolve(first).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn no_sources() -> IncludeSources {
        IncludeSources {
            sources: HashMap::new(),
        }
    }

    fn sources(entries: &[(&str, &str)]) -> IncludeSources {
        IncludeSources {
            sources: entries
                .iter()
                .map(|(name, content)| ((*name).to_owned(), (*content).to_owned()))
                .collect(),
        }
    }

    #[test]
    fn the_release_smoke_document_is_the_empty_circuit() {
        let content = interpret_document(
            &json!({"components": [], "schema": "rspice-circuit-v1"}),
            &no_sources(),
        )
        .ok()
        .expect("smoke document must interpret");
        assert!(matches!(content, CircuitContent::Empty));
    }

    #[test]
    fn schema_violations_are_bounded_customer_rejections() {
        for (document, code) in [
            (json!({"schema": "other-v1"}), "document.unsupported_schema"),
            (
                json!({"schema": "rspice-circuit-v1", "extra": 1}),
                "document.invalid",
            ),
            (
                json!({"schema": "rspice-circuit-v1", "components": [{"kind": "r"}]}),
                "document.components_unsupported",
            ),
            (
                json!({"schema": "rspice-circuit-v1", "netlist_utf8": ""}),
                "document.netlist_empty",
            ),
            (
                json!({"schema": "rspice-circuit-v1", "netlist_utf8": "title\0"}),
                "document.netlist_invalid",
            ),
        ] {
            let rejection = interpret_document(&document, &no_sources())
                .err()
                .expect("document must be rejected");
            assert_eq!(rejection.failure_code, code);
        }
    }

    #[test]
    fn includes_resolve_only_from_bound_sources() {
        let deck = "rc divider\n.include \"models.lib\"\nR1 in out 1k\n.op\n.end\n";
        let bound = sources(&[("models.lib", ".model d1 D(Is=1e-14)\n")]);
        let CircuitContent::Deck { expanded_netlist } = interpret_document(
            &json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
            &bound,
        )
        .ok()
        .expect("deck must interpret") else {
            panic!("expected a deck");
        };
        assert!(expanded_netlist.contains(".model d1 D(Is=1e-14)"));
        assert!(!expanded_netlist.to_ascii_lowercase().contains(".include"));

        let rejection = interpret_document(
            &json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
            &no_sources(),
        )
        .err()
        .expect("unresolved include must be rejected");
        assert_eq!(rejection.failure_code, "netlist.include_unresolved");
    }

    #[test]
    fn nested_includes_and_file_lib_references_fail_closed() {
        let nested = sources(&[("outer.lib", ".include inner.lib\n")]);
        let rejection = interpret_document(
            &json!({"schema": "rspice-circuit-v1", "netlist_utf8": "t\n.inc outer.lib\n.end\n"}),
            &nested,
        )
        .err()
        .expect("nested include must be rejected");
        assert_eq!(rejection.failure_code, "netlist.include_nested");

        let bound = sources(&[("vendor.lib", "* models\n")]);
        let rejection = interpret_document(
            &json!({"schema": "rspice-circuit-v1", "netlist_utf8": "t\n.lib vendor.lib tt\n.end\n"}),
            &bound,
        )
        .err()
        .expect("file .lib must be rejected");
        assert_eq!(rejection.failure_code, "netlist.lib_unsupported");
    }

    #[test]
    fn include_directive_parsing_is_exact() {
        assert_eq!(
            include_directive(".include models.lib\n"),
            Some("models.lib")
        );
        assert_eq!(include_directive("  .INC 'a b.lib'\n"), Some("a b.lib"));
        assert_eq!(include_directive(".include \"x.lib\""), Some("x.lib"));
        assert_eq!(include_directive(".includes x\n"), None);
        assert_eq!(include_directive("* .include comment\n"), None);
        assert_eq!(include_directive("R1 a b 1k\n"), None);
        assert_eq!(include_directive(".include\n"), None);
    }
}
