//! Stable source-diagnostic identity and collection bounds shared by producers and views.

use std::collections::HashSet;

pub const MAX_DIAGNOSTICS: usize = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticPosition {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRange {
    pub start: DiagnosticPosition,
    pub end: DiagnosticPosition,
}

pub fn diagnostic_range(
    byte_range: Option<&std::ops::Range<usize>>,
    line: Option<usize>,
    column: Option<usize>,
) -> Option<DiagnosticRange> {
    if byte_range.is_none() && line.is_none() && column.is_none() {
        return None;
    }
    let byte_range = byte_range.cloned().unwrap_or(0..0);
    let start = DiagnosticPosition {
        line: line.unwrap_or(0),
        column: column.unwrap_or(0),
        byte_offset: byte_range.start,
    };
    Some(DiagnosticRange {
        start,
        end: DiagnosticPosition {
            byte_offset: byte_range.end,
            ..start
        },
    })
}

pub fn stable_diagnostic_id(
    source: &str,
    code: &str,
    document_id: &str,
    range: Option<DiagnosticRange>,
    revision: u64,
    validation_id: &str,
    message: &str,
) -> uuid::Uuid {
    const NAMESPACE: uuid::Uuid = uuid::Uuid::from_u128(0x9cc9d141_951a_5553_b787_b2c80c98c43f);
    let mut identity = String::with_capacity(
        source.len() + code.len() + document_id.len() + validation_id.len() + message.len() + 96,
    );
    for value in [source, code, document_id, validation_id, message] {
        identity.push_str(&value.len().to_string());
        identity.push(':');
        identity.push_str(value);
        identity.push('|');
    }
    identity.push_str(&revision.to_string());
    if let Some(range) = range {
        use std::fmt::Write as _;
        let _ = write!(
            identity,
            "|{}:{}:{}-{}:{}:{}",
            range.start.line,
            range.start.column,
            range.start.byte_offset,
            range.end.line,
            range.end.column,
            range.end.byte_offset
        );
    }
    uuid::Uuid::new_v5(&NAMESPACE, identity.as_bytes())
}

pub fn validate_diagnostic_count(count: usize) -> Result<(), String> {
    if count > MAX_DIAGNOSTICS {
        return Err(format!(
            "Diagnostic collection contains {count} records; the supported maximum is {MAX_DIAGNOSTICS}."
        ));
    }
    Ok(())
}

pub fn insert_diagnostic_id(ids: &mut HashSet<uuid::Uuid>, id: uuid::Uuid) -> Result<(), String> {
    if !ids.insert(id) {
        return Err(format!(
            "Diagnostic collection contains duplicate canonical ID {id}."
        ));
    }
    Ok(())
}
