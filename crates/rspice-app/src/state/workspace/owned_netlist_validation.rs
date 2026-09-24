//! Validation of project-owned top-deck paths and exact document projections.

use std::collections::HashSet;

use super::{
    MAX_OWNED_NETLIST_HISTORY_BYTES, MAX_OWNED_NETLIST_HISTORY_REVISIONS, NetlistLineEnding,
    NetlistSourceDialect, NetlistTextEncoding, OwnedNetlistDescriptor,
    OwnedNetlistRevisionSnapshot,
};

impl OwnedNetlistRevisionSnapshot {
    pub fn from_document(
        document: &crate::state::NetlistDocument,
        message: impl Into<String>,
        source_encoding: NetlistTextEncoding,
        source_line_ending: NetlistLineEnding,
    ) -> Result<Self, String> {
        let snapshot = Self {
            document_revision: document.revision().get(),
            content_digest: document.content_digest(),
            source: document.source().to_owned(),
            dependencies: document.dependencies().to_vec(),
            owned_includes: Vec::new(),
            message: message.into(),
            source_encoding,
            source_line_ending,
        };
        snapshot.validate()?;
        Ok(snapshot)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.document_revision == 0 {
            return Err("owned netlist history revision must be non-zero".to_owned());
        }
        if self.content_digest != crate::state::content_digest(&self.source) {
            return Err("owned netlist history digest does not identify its source".to_owned());
        }
        if self.message.trim().is_empty()
            || self.message != self.message.trim()
            || self.message.chars().count() > 240
            || self.message.chars().any(char::is_control)
        {
            return Err("owned netlist history message is invalid".to_owned());
        }
        let mut document_ids = HashSet::new();
        let mut identities = HashSet::new();
        for include in &self.owned_includes {
            include.validate()?;
            if !document_ids.insert(include.document_id)
                || !identities.insert(include.logical_identity.as_str())
            {
                return Err("owned netlist history repeats an include identity".to_owned());
            }
            let dependency = self
                .dependencies
                .iter()
                .find(|dependency| {
                    dependency.locator().logical_identity() == include.logical_identity
                })
                .ok_or_else(|| {
                    format!(
                        "owned netlist history include '{}' is absent from its dependency closure",
                        include.logical_identity
                    )
                })?;
            let source = dependency.source().ok_or_else(|| {
                format!(
                    "owned netlist history include '{}' has no retained source",
                    include.logical_identity
                )
            })?;
            if include.content_digest != crate::state::content_digest(source) {
                return Err(format!(
                    "owned netlist history include '{}' digest does not identify its retained source",
                    include.logical_identity
                ));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        self.source.len()
            + self.message.len()
            + self
                .dependencies
                .iter()
                .filter_map(crate::state::DependencyMetadata::source_bytes)
                .map(<[u8]>::len)
                .sum::<usize>()
            + self
                .owned_includes
                .iter()
                .map(|include| include.logical_identity.len() + include.display_name.len() + 80)
                .sum::<usize>()
    }
}

/// Validate one portable, project-relative top-deck path. Forward slashes are
/// the persisted separator on every platform; native publication paths remain
/// separate in `netlist_source_path`.
pub fn validate_owned_netlist_artifact_path(path: &str) -> Result<(), String> {
    if path.is_empty()
        || path != path.trim()
        || path.len() > 4_096
        || path.chars().any(char::is_control)
        || path.contains('\\')
        || path.starts_with('/')
        || path.ends_with('/')
        || path
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
        || path.contains(':')
    {
        return Err(
            "owned top-deck path must be a trimmed, portable project-relative path".to_owned(),
        );
    }
    Ok(())
}

pub(super) fn validate_owned_netlist_projection(
    document: &crate::state::NetlistDocument,
    descriptor: &OwnedNetlistDescriptor,
    source: &str,
) -> Result<(), String> {
    if descriptor.deck_id.is_nil() {
        return Err("owned top-deck identity cannot be nil".to_owned());
    }
    validate_owned_netlist_artifact_path(&descriptor.artifact_name)?;
    if document.ownership() == crate::state::DocumentOwnership::Generated {
        return Err("project-owned netlist document cannot have generated ownership".to_owned());
    }
    if document.source() != source {
        return Err("canonical document bytes differ from the owned source projection".to_owned());
    }

    let declared_dialect = descriptor
        .imported_dialect
        .unwrap_or(NetlistSourceDialect::RSpice);
    let expected_profile = declared_dialect.execution_profile();
    if descriptor.execution_profile.is_some() && descriptor.execution_profile != expected_profile {
        return Err(format!(
            "owned netlist dialect {} does not match its recorded execution profile",
            declared_dialect.label()
        ));
    }
    if declared_dialect.requires_compatibility_review() {
        let reviewed = descriptor.compatibility_reviewed
            && descriptor.execution_profile == expected_profile
            && expected_profile.is_some();
        let quarantined =
            !descriptor.compatibility_reviewed && descriptor.execution_profile.is_none();
        if !reviewed && !quarantined {
            return Err(format!(
                "owned non-canonical netlist dialect {} has neither an exact reviewed executable profile nor a fail-closed quarantine",
                declared_dialect.label()
            ));
        }
    }

    let mut previous_revision = 0_u64;
    for record in &descriptor.save_history {
        if record.document_revision == 0
            || record.document_revision <= previous_revision
            || record.document_revision > document.revision().get()
            || record.message.trim().is_empty()
            || record.message != record.message.trim()
            || record.message.chars().any(char::is_control)
        {
            return Err(
                "owned source save history is not strictly revision ordered or has an invalid message"
                    .to_owned(),
            );
        }
        previous_revision = record.document_revision;
    }
    if descriptor.revision_history.len() > MAX_OWNED_NETLIST_HISTORY_REVISIONS {
        return Err("owned source revision history exceeds its bounded entry limit".to_owned());
    }
    previous_revision = 0;
    let mut retained_bytes = 0_usize;
    for snapshot in &descriptor.revision_history {
        snapshot.validate()?;
        if snapshot.document_revision <= previous_revision
            || snapshot.document_revision > document.revision().get()
        {
            return Err(
                "owned source revision history is not strictly revision ordered".to_owned(),
            );
        }
        retained_bytes = retained_bytes
            .checked_add(snapshot.retained_bytes())
            .ok_or_else(|| "owned source revision history size overflowed".to_owned())?;
        previous_revision = snapshot.document_revision;
    }
    if retained_bytes > MAX_OWNED_NETLIST_HISTORY_BYTES {
        return Err("owned source revision history exceeds its bounded byte limit".to_owned());
    }
    if descriptor.owned_includes.len() > crate::state::MAX_PROJECT_SOURCE_FILES {
        return Err("owned include catalog exceeds the project file limit".to_owned());
    }
    let mut include_ids = HashSet::new();
    let mut include_identities = HashSet::new();
    for include in &descriptor.owned_includes {
        include.validate()?;
        if !include_ids.insert(include.document_id)
            || !include_identities.insert(include.logical_identity.as_str())
        {
            return Err("owned include identities must be unique".to_owned());
        }
        let dependency = document
            .dependencies()
            .iter()
            .find(|dependency| dependency.locator().logical_identity() == include.logical_identity)
            .ok_or_else(|| {
                format!(
                    "owned include '{}' is absent from the canonical dependency closure",
                    include.logical_identity
                )
            })?;
        let dependency_source = dependency.source().ok_or_else(|| {
            format!(
                "owned include '{}' has no retained source bytes",
                include.logical_identity
            )
        })?;
        if include.content_digest != crate::state::content_digest(dependency_source) {
            return Err(format!(
                "owned include '{}' digest does not identify its retained bytes",
                include.logical_identity
            ));
        }
    }
    Ok(())
}
