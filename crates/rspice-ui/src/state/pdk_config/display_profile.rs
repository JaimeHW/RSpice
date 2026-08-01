//! Versioned PDK display profiles.
//!
//! Display profiles are presentation overlays. They are bound to one exact
//! signed technology manifest and cannot alter layer identity, purpose,
//! connectivity, stream mapping, or any other technology semantic.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::product::ContentDigest;

use super::technology_package::{
    PdkAdministrativeAuthority, PdkTechnologyBinding, ValidatedPdkTechnologyPackage,
};

pub const MAX_PDK_DISPLAY_PROFILES: usize = 1_024;
pub const MAX_PDK_DISPLAY_PROFILE_REVISIONS: usize = 16_384;
pub const MAX_PDK_DISPLAY_PROFILE_ENTRIES: usize = 16_384;
pub const MAX_PDK_DISPLAY_PROFILE_RECEIPTS: usize = 16_384;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkDisplayProfileScope {
    #[default]
    PersonalDevice,
    Project,
    Organization,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkDisplayFillStyle {
    #[default]
    Solid,
    Diagonal,
    Crosshatch,
    Dots,
    Hollow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkDisplayLayerStyle {
    pub layer: String,
    pub purpose: String,
    pub visible: bool,
    pub selectable: bool,
    pub screen_rgba: [u8; 4],
    pub screen_fill: PdkDisplayFillStyle,
    pub print_rgba: [u8; 4],
    pub print_fill: PdkDisplayFillStyle,
    pub outline_width_milli_px: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkDisplayProfileDraft {
    pub profile_id: String,
    pub label: String,
    pub scope: PdkDisplayProfileScope,
    pub technology: PdkTechnologyBinding,
    pub dim_unrelated: bool,
    pub hidden_objects_pickable: bool,
    pub selection_rgba: [u8; 4],
    pub entries: Vec<PdkDisplayLayerStyle>,
}

impl PdkDisplayProfileDraft {
    #[must_use]
    pub fn signed_defaults(
        package: &ValidatedPdkTechnologyPackage,
        profile_id: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        let entries = package
            .manifest()
            .layers
            .iter()
            .flat_map(|layer| {
                layer
                    .purposes
                    .iter()
                    .map(move |purpose| PdkDisplayLayerStyle {
                        layer: layer.name.clone(),
                        purpose: purpose.clone(),
                        visible: true,
                        selectable: true,
                        screen_rgba: layer.display_rgba,
                        screen_fill: PdkDisplayFillStyle::Solid,
                        print_rgba: [0, 0, 0, layer.display_rgba[3]],
                        print_fill: PdkDisplayFillStyle::Solid,
                        outline_width_milli_px: 1_000,
                    })
            })
            .collect();
        Self {
            profile_id: profile_id.into(),
            label: label.into(),
            scope: PdkDisplayProfileScope::PersonalDevice,
            technology: package.binding(),
            dim_unrelated: true,
            hidden_objects_pickable: false,
            selection_rgba: [242, 184, 36, 255],
            entries,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkDisplayProfileRevision {
    pub profile_id: String,
    pub revision: u64,
    pub label: String,
    pub scope: PdkDisplayProfileScope,
    pub technology: PdkTechnologyBinding,
    pub dim_unrelated: bool,
    pub hidden_objects_pickable: bool,
    pub selection_rgba: [u8; 4],
    pub entries: Vec<PdkDisplayLayerStyle>,
    pub content_digest: ContentDigest,
}

impl PdkDisplayProfileRevision {
    #[must_use]
    pub fn draft(&self) -> PdkDisplayProfileDraft {
        PdkDisplayProfileDraft {
            profile_id: self.profile_id.clone(),
            label: self.label.clone(),
            scope: self.scope,
            technology: self.technology.clone(),
            dim_unrelated: self.dim_unrelated,
            hidden_objects_pickable: self.hidden_objects_pickable,
            selection_rgba: self.selection_rgba,
            entries: self.entries.clone(),
        }
    }

    fn calculate_digest(&self) -> Result<ContentDigest, PdkDisplayProfileError> {
        #[derive(Serialize)]
        struct Payload<'a> {
            profile_id: &'a str,
            revision: u64,
            label: &'a str,
            scope: PdkDisplayProfileScope,
            technology: &'a PdkTechnologyBinding,
            dim_unrelated: bool,
            hidden_objects_pickable: bool,
            selection_rgba: [u8; 4],
            entries: &'a [PdkDisplayLayerStyle],
        }
        digest_json(&Payload {
            profile_id: &self.profile_id,
            revision: self.revision,
            label: &self.label,
            scope: self.scope,
            technology: &self.technology,
            dim_unrelated: self.dim_unrelated,
            hidden_objects_pickable: self.hidden_objects_pickable,
            selection_rgba: self.selection_rgba,
            entries: &self.entries,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkDisplayProfileBinding {
    pub profile_id: String,
    pub revision: u64,
    pub technology_manifest_digest: ContentDigest,
    pub profile_digest: ContentDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkDisplayProfileAuditAction {
    PublishAndActivate,
    Activate,
    Rollback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkDisplayProfileAuditReceipt {
    pub sequence: u64,
    pub action: PdkDisplayProfileAuditAction,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub target: PdkDisplayProfileBinding,
    pub before_active: Option<PdkDisplayProfileBinding>,
    pub after_active: PdkDisplayProfileBinding,
    pub previous_receipt_digest: Option<ContentDigest>,
    pub receipt_digest: ContentDigest,
}

impl PdkDisplayProfileAuditReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, PdkDisplayProfileError> {
        #[derive(Serialize)]
        struct Payload<'a> {
            sequence: u64,
            action: PdkDisplayProfileAuditAction,
            actor_id: &'a str,
            authority_id: &'a str,
            reason: &'a str,
            target: &'a PdkDisplayProfileBinding,
            before_active: &'a Option<PdkDisplayProfileBinding>,
            after_active: &'a PdkDisplayProfileBinding,
            previous_receipt_digest: Option<ContentDigest>,
        }
        digest_json(&Payload {
            sequence: self.sequence,
            action: self.action,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            target: &self.target,
            before_active: &self.before_active,
            after_active: &self.after_active,
            previous_receipt_digest: self.previous_receipt_digest,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkDisplayProfileRegistry {
    #[serde(default)]
    revisions: Vec<PdkDisplayProfileRevision>,
    #[serde(default)]
    active: Option<PdkDisplayProfileBinding>,
    #[serde(default)]
    audit: Vec<PdkDisplayProfileAuditReceipt>,
}

impl PdkDisplayProfileRegistry {
    #[must_use]
    pub fn revisions(&self) -> &[PdkDisplayProfileRevision] {
        &self.revisions
    }

    #[must_use]
    pub fn active_binding(&self) -> Option<&PdkDisplayProfileBinding> {
        self.active.as_ref()
    }

    #[must_use]
    pub fn audit(&self) -> &[PdkDisplayProfileAuditReceipt] {
        &self.audit
    }

    pub fn publish_and_activate(
        &mut self,
        package: &ValidatedPdkTechnologyPackage,
        draft: PdkDisplayProfileDraft,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkDisplayProfileAuditReceipt, PdkDisplayProfileError> {
        self.validate_audit_chain()?;
        validate_authority(authority)?;
        validate_text("reason", reason, 1_024)?;
        validate_draft(&draft, package)?;
        if self.revisions.len() >= MAX_PDK_DISPLAY_PROFILE_REVISIONS {
            return Err(PdkDisplayProfileError::LimitExceeded(format!(
                "display-profile revisions exceed {MAX_PDK_DISPLAY_PROFILE_REVISIONS}"
            )));
        }
        let profile_count = self
            .revisions
            .iter()
            .map(|revision| revision.profile_id.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        if !profile_count.contains(&draft.profile_id.to_ascii_lowercase())
            && profile_count.len() >= MAX_PDK_DISPLAY_PROFILES
        {
            return Err(PdkDisplayProfileError::LimitExceeded(format!(
                "display profiles exceed {MAX_PDK_DISPLAY_PROFILES}"
            )));
        }
        if let Some(existing) = self
            .revisions
            .iter()
            .find(|revision| revision.profile_id.eq_ignore_ascii_case(&draft.profile_id))
        {
            if existing.technology != draft.technology {
                return Err(PdkDisplayProfileError::InvalidTransition(format!(
                    "profile '{}' is permanently bound to technology {} {}",
                    draft.profile_id, existing.technology.package_id, existing.technology.revision
                )));
            }
        }
        let revision_number = self
            .revisions
            .iter()
            .filter(|revision| revision.profile_id.eq_ignore_ascii_case(&draft.profile_id))
            .map(|revision| revision.revision)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| {
                PdkDisplayProfileError::LimitExceeded(
                    "display-profile revision space is exhausted".to_owned(),
                )
            })?;
        let mut revision = PdkDisplayProfileRevision {
            profile_id: draft.profile_id,
            revision: revision_number,
            label: draft.label,
            scope: draft.scope,
            technology: draft.technology,
            dim_unrelated: draft.dim_unrelated,
            hidden_objects_pickable: draft.hidden_objects_pickable,
            selection_rgba: draft.selection_rgba,
            entries: draft.entries,
            content_digest: ContentDigest::from_bytes([0; 32]),
        };
        revision.content_digest = revision.calculate_digest()?;
        let binding = binding_for(&revision);
        let receipt = self.next_receipt(
            PdkDisplayProfileAuditAction::PublishAndActivate,
            authority,
            reason,
            binding.clone(),
        )?;
        let mut candidate = self.clone();
        candidate.revisions.push(revision);
        candidate.sort_revisions();
        candidate.active = Some(binding);
        candidate.audit.push(receipt.clone());
        candidate.validate_audit_chain()?;
        *self = candidate;
        Ok(receipt)
    }

    pub fn activate(
        &mut self,
        package: &ValidatedPdkTechnologyPackage,
        profile_id: &str,
        revision: u64,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkDisplayProfileAuditReceipt, PdkDisplayProfileError> {
        self.activate_as(
            PdkDisplayProfileAuditAction::Activate,
            package,
            profile_id,
            revision,
            authority,
            reason,
        )
    }

    pub fn rollback_to(
        &mut self,
        package: &ValidatedPdkTechnologyPackage,
        profile_id: &str,
        revision: u64,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkDisplayProfileAuditReceipt, PdkDisplayProfileError> {
        let target = self
            .find_revision(profile_id, revision)
            .map(binding_for)
            .ok_or_else(|| PdkDisplayProfileError::MissingRevision {
                profile_id: profile_id.to_owned(),
                revision,
            })?;
        let appeared_before = self
            .audit
            .iter()
            .any(|receipt| receipt.after_active == target);
        if !appeared_before {
            return Err(PdkDisplayProfileError::InvalidTransition(format!(
                "{profile_id} revision {revision} has never been active"
            )));
        }
        self.activate_as(
            PdkDisplayProfileAuditAction::Rollback,
            package,
            profile_id,
            revision,
            authority,
            reason,
        )
    }

    #[must_use]
    pub fn active_for_package(
        &self,
        package: &ValidatedPdkTechnologyPackage,
    ) -> Option<&PdkDisplayProfileRevision> {
        if self.validate_audit_chain().is_err() {
            return None;
        }
        let active = self.active.as_ref()?;
        if active.technology_manifest_digest != package.manifest_digest() {
            return None;
        }
        let revision = self.find_revision(&active.profile_id, active.revision)?;
        if binding_for(revision) != *active || validate_revision(revision, package).is_err() {
            return None;
        }
        Some(revision)
    }

    pub fn validate_audit_chain(&self) -> Result<(), PdkDisplayProfileError> {
        if self.revisions.len() > MAX_PDK_DISPLAY_PROFILE_REVISIONS {
            return Err(PdkDisplayProfileError::LimitExceeded(format!(
                "display-profile revisions exceed {MAX_PDK_DISPLAY_PROFILE_REVISIONS}"
            )));
        }
        if self.audit.len() > MAX_PDK_DISPLAY_PROFILE_RECEIPTS {
            return Err(PdkDisplayProfileError::LimitExceeded(format!(
                "display-profile receipts exceed {MAX_PDK_DISPLAY_PROFILE_RECEIPTS}"
            )));
        }
        let mut identities = BTreeSet::new();
        let mut maximum_revision = BTreeMap::<String, u64>::new();
        let mut profile_technologies = BTreeMap::<String, PdkTechnologyBinding>::new();
        for (index, revision) in self.revisions.iter().enumerate() {
            validate_identifier(
                &format!("revisions[{index}].profile_id"),
                &revision.profile_id,
            )?;
            validate_text(&format!("revisions[{index}].label"), &revision.label, 128)?;
            validate_binding(&revision.technology)?;
            if revision.revision == 0 {
                return Err(PdkDisplayProfileError::Corrupted(
                    "revision numbers must be greater than zero".to_owned(),
                ));
            }
            let identity = (revision.profile_id.to_ascii_lowercase(), revision.revision);
            match profile_technologies
                .entry(identity.0.clone())
                .or_insert_with(|| revision.technology.clone())
            {
                binding if *binding != revision.technology => {
                    return Err(PdkDisplayProfileError::Corrupted(format!(
                        "profile '{}' crosses immutable technology bindings",
                        revision.profile_id
                    )));
                }
                _ => {}
            }
            if !identities.insert(identity.clone()) {
                return Err(PdkDisplayProfileError::Corrupted(format!(
                    "duplicate profile revision {} {}",
                    revision.profile_id, revision.revision
                )));
            }
            let expected = maximum_revision
                .entry(identity.0)
                .and_modify(|maximum| *maximum = (*maximum).max(revision.revision))
                .or_insert(revision.revision);
            let _ = expected;
            if revision.calculate_digest()? != revision.content_digest {
                return Err(PdkDisplayProfileError::Corrupted(format!(
                    "profile {} revision {} content digest is invalid",
                    revision.profile_id, revision.revision
                )));
            }
        }
        if maximum_revision.values().any(|maximum| *maximum == 0) {
            return Err(PdkDisplayProfileError::Corrupted(
                "profile revision space is invalid".to_owned(),
            ));
        }

        let mut expected_active = None;
        let mut previous = None;
        let mut published = BTreeSet::new();
        for (index, receipt) in self.audit.iter().enumerate() {
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| {
                    PdkDisplayProfileError::LimitExceeded(
                        "display-profile receipt sequence is exhausted".to_owned(),
                    )
                })?;
            if receipt.sequence != expected_sequence
                || receipt.previous_receipt_digest != previous
                || receipt.before_active != expected_active
                || receipt.target != receipt.after_active
                || receipt.calculate_digest()? != receipt.receipt_digest
            {
                return Err(PdkDisplayProfileError::Corrupted(format!(
                    "display-profile receipt #{} has invalid sequence, transition, or digest linkage",
                    receipt.sequence
                )));
            }
            validate_authority(&PdkAdministrativeAuthority {
                actor_id: receipt.actor_id.clone(),
                authority_id: receipt.authority_id.clone(),
            })?;
            validate_text("receipt.reason", &receipt.reason, 1_024)?;
            let revision = self
                .find_revision(&receipt.target.profile_id, receipt.target.revision)
                .ok_or_else(|| {
                    PdkDisplayProfileError::Corrupted(format!(
                        "receipt #{} references a missing profile revision",
                        receipt.sequence
                    ))
                })?;
            if binding_for(revision) != receipt.target {
                return Err(PdkDisplayProfileError::Corrupted(format!(
                    "receipt #{} target digest does not match the immutable revision",
                    receipt.sequence
                )));
            }
            match receipt.action {
                PdkDisplayProfileAuditAction::PublishAndActivate => {
                    let key = (
                        receipt.target.profile_id.to_ascii_lowercase(),
                        receipt.target.revision,
                        receipt.target.technology_manifest_digest,
                        receipt.target.profile_digest,
                    );
                    if !published.insert(key) {
                        return Err(PdkDisplayProfileError::Corrupted(format!(
                            "receipt #{} republishes an immutable revision",
                            receipt.sequence
                        )));
                    }
                    let publishes = self.audit[..=index]
                        .iter()
                        .filter(|candidate| {
                            candidate.action == PdkDisplayProfileAuditAction::PublishAndActivate
                                && candidate
                                    .target
                                    .profile_id
                                    .eq_ignore_ascii_case(&receipt.target.profile_id)
                        })
                        .count();
                    if u64::try_from(publishes).ok() != Some(receipt.target.revision) {
                        return Err(PdkDisplayProfileError::Corrupted(format!(
                            "receipt #{} does not publish the next immutable revision",
                            receipt.sequence
                        )));
                    }
                }
                PdkDisplayProfileAuditAction::Activate => {
                    if !published.contains(&(
                        receipt.target.profile_id.to_ascii_lowercase(),
                        receipt.target.revision,
                        receipt.target.technology_manifest_digest,
                        receipt.target.profile_digest,
                    )) {
                        return Err(PdkDisplayProfileError::Corrupted(format!(
                            "receipt #{} activates a revision before publication",
                            receipt.sequence
                        )));
                    }
                }
                PdkDisplayProfileAuditAction::Rollback => {
                    let key = (
                        receipt.target.profile_id.to_ascii_lowercase(),
                        receipt.target.revision,
                        receipt.target.technology_manifest_digest,
                        receipt.target.profile_digest,
                    );
                    if !published.contains(&key)
                        || !self.audit[..index]
                            .iter()
                            .any(|candidate| candidate.after_active == receipt.target)
                    {
                        return Err(PdkDisplayProfileError::Corrupted(format!(
                            "receipt #{} rolls back to a revision without prior active lineage",
                            receipt.sequence
                        )));
                    }
                }
            }
            if receipt.before_active.as_ref() == Some(&receipt.after_active) {
                return Err(PdkDisplayProfileError::Corrupted(format!(
                    "receipt #{} records a no-op activation",
                    receipt.sequence
                )));
            }
            expected_active = Some(receipt.after_active.clone());
            previous = Some(receipt.receipt_digest);
        }
        if self.active != expected_active {
            return Err(PdkDisplayProfileError::Corrupted(
                "active display profile does not match the final receipt".to_owned(),
            ));
        }
        let expected_publications = self
            .revisions
            .iter()
            .map(|revision| {
                let binding = binding_for(revision);
                (
                    binding.profile_id.to_ascii_lowercase(),
                    binding.revision,
                    binding.technology_manifest_digest,
                    binding.profile_digest,
                )
            })
            .collect::<BTreeSet<_>>();
        if published != expected_publications {
            return Err(PdkDisplayProfileError::Corrupted(
                "immutable display-profile revisions do not match publication receipts".to_owned(),
            ));
        }
        Ok(())
    }

    fn activate_as(
        &mut self,
        action: PdkDisplayProfileAuditAction,
        package: &ValidatedPdkTechnologyPackage,
        profile_id: &str,
        revision_number: u64,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkDisplayProfileAuditReceipt, PdkDisplayProfileError> {
        self.validate_audit_chain()?;
        validate_authority(authority)?;
        validate_text("reason", reason, 1_024)?;
        let revision = self
            .find_revision(profile_id, revision_number)
            .ok_or_else(|| PdkDisplayProfileError::MissingRevision {
                profile_id: profile_id.to_owned(),
                revision: revision_number,
            })?;
        validate_revision(revision, package)?;
        let binding = binding_for(revision);
        if self.active.as_ref() == Some(&binding) {
            return Err(PdkDisplayProfileError::InvalidTransition(format!(
                "{} revision {} is already active",
                binding.profile_id, binding.revision
            )));
        }
        let receipt = self.next_receipt(action, authority, reason, binding.clone())?;
        let mut candidate = self.clone();
        candidate.active = Some(binding);
        candidate.audit.push(receipt.clone());
        candidate.validate_audit_chain()?;
        *self = candidate;
        Ok(receipt)
    }

    fn next_receipt(
        &self,
        action: PdkDisplayProfileAuditAction,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
        binding: PdkDisplayProfileBinding,
    ) -> Result<PdkDisplayProfileAuditReceipt, PdkDisplayProfileError> {
        if self.audit.len() >= MAX_PDK_DISPLAY_PROFILE_RECEIPTS {
            return Err(PdkDisplayProfileError::LimitExceeded(format!(
                "display-profile receipts exceed {MAX_PDK_DISPLAY_PROFILE_RECEIPTS}"
            )));
        }
        let sequence = u64::try_from(self.audit.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| {
                PdkDisplayProfileError::LimitExceeded(
                    "display-profile receipt sequence is exhausted".to_owned(),
                )
            })?;
        let mut receipt = PdkDisplayProfileAuditReceipt {
            sequence,
            action,
            actor_id: authority.actor_id.clone(),
            authority_id: authority.authority_id.clone(),
            reason: reason.to_owned(),
            target: binding.clone(),
            before_active: self.active.clone(),
            after_active: binding,
            previous_receipt_digest: self.audit.last().map(|receipt| receipt.receipt_digest),
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;
        Ok(receipt)
    }

    fn find_revision(&self, profile_id: &str, revision: u64) -> Option<&PdkDisplayProfileRevision> {
        self.revisions.iter().find(|candidate| {
            candidate.profile_id.eq_ignore_ascii_case(profile_id) && candidate.revision == revision
        })
    }

    fn sort_revisions(&mut self) {
        self.revisions.sort_by(|left, right| {
            (left.profile_id.to_ascii_lowercase(), left.revision)
                .cmp(&(right.profile_id.to_ascii_lowercase(), right.revision))
        });
    }
}

fn validate_draft(
    draft: &PdkDisplayProfileDraft,
    package: &ValidatedPdkTechnologyPackage,
) -> Result<(), PdkDisplayProfileError> {
    let mut revision = PdkDisplayProfileRevision {
        profile_id: draft.profile_id.clone(),
        revision: 1,
        label: draft.label.clone(),
        scope: draft.scope,
        technology: draft.technology.clone(),
        dim_unrelated: draft.dim_unrelated,
        hidden_objects_pickable: draft.hidden_objects_pickable,
        selection_rgba: draft.selection_rgba,
        entries: draft.entries.clone(),
        content_digest: ContentDigest::from_bytes([0; 32]),
    };
    revision.content_digest = revision.calculate_digest()?;
    validate_revision(&revision, package)
}

fn validate_revision(
    revision: &PdkDisplayProfileRevision,
    package: &ValidatedPdkTechnologyPackage,
) -> Result<(), PdkDisplayProfileError> {
    validate_identifier("profile_id", &revision.profile_id)?;
    validate_text("label", &revision.label, 128)?;
    if revision.scope != PdkDisplayProfileScope::PersonalDevice {
        return Err(PdkDisplayProfileError::InvalidField(format!(
            "{} scope requires a connected project or organization policy repository; only personal-device profiles are currently publishable",
            match revision.scope {
                PdkDisplayProfileScope::PersonalDevice => "personal-device",
                PdkDisplayProfileScope::Project => "project",
                PdkDisplayProfileScope::Organization => "organization",
            }
        )));
    }
    validate_binding(&revision.technology)?;
    if revision.technology != package.binding() {
        return Err(PdkDisplayProfileError::TechnologyMismatch {
            expected: package.binding(),
            actual: revision.technology.clone(),
        });
    }
    if revision.entries.is_empty() || revision.entries.len() > MAX_PDK_DISPLAY_PROFILE_ENTRIES {
        return Err(PdkDisplayProfileError::LimitExceeded(format!(
            "display-profile entries must contain 1..={MAX_PDK_DISPLAY_PROFILE_ENTRIES} rows"
        )));
    }
    if revision.selection_rgba[3] == 0 {
        return Err(PdkDisplayProfileError::InvalidField(
            "selection color must not be fully transparent".to_owned(),
        ));
    }
    let expected = package
        .manifest()
        .layers
        .iter()
        .flat_map(|layer| {
            layer.purposes.iter().map(move |purpose| {
                (
                    layer.name.to_ascii_lowercase(),
                    purpose.to_ascii_lowercase(),
                )
            })
        })
        .collect::<BTreeSet<_>>();
    let mut actual = BTreeSet::new();
    for (index, entry) in revision.entries.iter().enumerate() {
        validate_identifier(&format!("entries[{index}].layer"), &entry.layer)?;
        validate_identifier(&format!("entries[{index}].purpose"), &entry.purpose)?;
        if entry.outline_width_milli_px > 16_000 {
            return Err(PdkDisplayProfileError::InvalidField(format!(
                "entries[{index}].outline_width_milli_px exceeds 16000"
            )));
        }
        if entry.visible && entry.screen_rgba[3] == 0 {
            return Err(PdkDisplayProfileError::InvalidField(format!(
                "entries[{index}] is visible but fully transparent"
            )));
        }
        let identity = (
            entry.layer.to_ascii_lowercase(),
            entry.purpose.to_ascii_lowercase(),
        );
        if !actual.insert(identity.clone()) {
            return Err(PdkDisplayProfileError::Duplicate(format!(
                "{}/{}",
                entry.layer, entry.purpose
            )));
        }
        if !expected.contains(&identity) {
            return Err(PdkDisplayProfileError::UnknownLayerPurpose(format!(
                "{}/{}",
                entry.layer, entry.purpose
            )));
        }
    }
    if actual != expected {
        let missing = expected
            .difference(&actual)
            .map(|(layer, purpose)| format!("{layer}/{purpose}"))
            .collect::<Vec<_>>();
        return Err(PdkDisplayProfileError::MissingLayerPurposes(missing));
    }
    Ok(())
}

fn binding_for(revision: &PdkDisplayProfileRevision) -> PdkDisplayProfileBinding {
    PdkDisplayProfileBinding {
        profile_id: revision.profile_id.clone(),
        revision: revision.revision,
        technology_manifest_digest: revision.technology.manifest_digest,
        profile_digest: revision.content_digest,
    }
}

fn validate_binding(binding: &PdkTechnologyBinding) -> Result<(), PdkDisplayProfileError> {
    validate_identifier("technology.package_id", &binding.package_id)?;
    validate_text("technology.revision", &binding.revision, 64)
}

fn validate_authority(
    authority: &PdkAdministrativeAuthority,
) -> Result<(), PdkDisplayProfileError> {
    authority
        .validate()
        .map_err(|error| PdkDisplayProfileError::InvalidAuthority(error.to_string()))
}

fn validate_identifier(path: &str, value: &str) -> Result<(), PdkDisplayProfileError> {
    validate_text(path, value, 128)?;
    if !value.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'-' | b'_' | b'.' | b':' | b'@')
    }) {
        return Err(PdkDisplayProfileError::InvalidField(format!(
            "{path} must use lowercase ASCII identifier characters"
        )));
    }
    Ok(())
}

fn validate_text(path: &str, value: &str, maximum: usize) -> Result<(), PdkDisplayProfileError> {
    if value.trim() != value || value.is_empty() || value.len() > maximum {
        return Err(PdkDisplayProfileError::InvalidField(format!(
            "{path} must contain 1..={maximum} bytes without surrounding whitespace"
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(PdkDisplayProfileError::InvalidField(format!(
            "{path} contains a control character"
        )));
    }
    Ok(())
}

fn digest_json(value: &impl Serialize) -> Result<ContentDigest, PdkDisplayProfileError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| PdkDisplayProfileError::Serialization(error.to_string()))?;
    Ok(ContentDigest::from_bytes(Sha256::digest(bytes).into()))
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PdkDisplayProfileError {
    #[error("invalid display-profile field: {0}")]
    InvalidField(String),
    #[error("invalid display-profile authority: {0}")]
    InvalidAuthority(String),
    #[error("duplicate display layer/purpose: {0}")]
    Duplicate(String),
    #[error("unknown display layer/purpose: {0}")]
    UnknownLayerPurpose(String),
    #[error("display profile omits layer purposes: {0:?}")]
    MissingLayerPurposes(Vec<String>),
    #[error("display profile is bound to {actual:?}, expected {expected:?}")]
    TechnologyMismatch {
        expected: PdkTechnologyBinding,
        actual: PdkTechnologyBinding,
    },
    #[error("display profile '{profile_id}' revision {revision} does not exist")]
    MissingRevision { profile_id: String, revision: u64 },
    #[error("invalid display-profile transition: {0}")]
    InvalidTransition(String),
    #[error("display-profile registry is corrupted: {0}")]
    Corrupted(String),
    #[error("display-profile limit exceeded: {0}")]
    LimitExceeded(String),
    #[error("display-profile serialization failed: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::pdk_config::technology_package::tests::fixture_archive;

    fn fixture() -> (ValidatedPdkTechnologyPackage, PdkAdministrativeAuthority) {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = super::super::technology_package::PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "install display fixture")
            .expect("install");
        (registry.validated_packages()[0].clone(), authority)
    }

    #[test]
    fn publication_is_immutable_versioned_and_exactly_package_bound() {
        let (package, authority) = fixture();
        let mut registry = PdkDisplayProfileRegistry::default();
        let mut draft =
            PdkDisplayProfileDraft::signed_defaults(&package, "layout-dark", "Layout dark");
        let first = registry
            .publish_and_activate(&package, draft.clone(), &authority, "initial profile")
            .expect("publish");
        draft.entries[0].screen_rgba = [20, 200, 80, 255];
        let second = registry
            .publish_and_activate(&package, draft, &authority, "improve active contrast")
            .expect("publish revision");

        assert_eq!(first.target.revision, 1);
        assert_eq!(second.target.revision, 2);
        assert_ne!(first.target.profile_digest, second.target.profile_digest);
        assert_eq!(registry.revisions().len(), 2);
        assert_eq!(
            registry
                .active_for_package(&package)
                .map(|profile| profile.revision),
            Some(2)
        );
        registry.validate_audit_chain().expect("audit");
    }

    #[test]
    fn incomplete_or_foreign_layer_contract_fails_closed() {
        let (package, authority) = fixture();
        let mut registry = PdkDisplayProfileRegistry::default();
        let mut missing =
            PdkDisplayProfileDraft::signed_defaults(&package, "layout-dark", "Layout dark");
        missing.entries.pop();
        assert!(matches!(
            registry.publish_and_activate(&package, missing, &authority, "missing row"),
            Err(PdkDisplayProfileError::MissingLayerPurposes(_))
        ));

        let mut foreign =
            PdkDisplayProfileDraft::signed_defaults(&package, "layout-dark", "Layout dark");
        foreign.entries[0].layer = "unknown".to_owned();
        assert!(matches!(
            registry.publish_and_activate(&package, foreign, &authority, "foreign row"),
            Err(PdkDisplayProfileError::UnknownLayerPurpose(_))
        ));
        assert!(registry.revisions().is_empty());
        assert!(registry.audit().is_empty());
    }

    #[test]
    fn unavailable_project_and_organization_scopes_fail_closed() {
        let (package, authority) = fixture();
        let mut registry = PdkDisplayProfileRegistry::default();
        for scope in [
            PdkDisplayProfileScope::Project,
            PdkDisplayProfileScope::Organization,
        ] {
            let mut draft =
                PdkDisplayProfileDraft::signed_defaults(&package, "layout-dark", "Layout dark");
            draft.scope = scope;
            assert!(matches!(
                registry.publish_and_activate(
                    &package,
                    draft,
                    &authority,
                    "reject unavailable scope"
                ),
                Err(PdkDisplayProfileError::InvalidField(_))
            ));
        }
        assert!(registry.revisions().is_empty());
        assert!(registry.audit().is_empty());
    }

    #[test]
    fn rollback_requires_exact_previously_active_revision() {
        let (package, authority) = fixture();
        let mut registry = PdkDisplayProfileRegistry::default();
        let mut draft =
            PdkDisplayProfileDraft::signed_defaults(&package, "layout-dark", "Layout dark");
        registry
            .publish_and_activate(&package, draft.clone(), &authority, "revision one")
            .expect("publish one");
        draft.label = "Layout dark adjusted".to_owned();
        registry
            .publish_and_activate(&package, draft, &authority, "revision two")
            .expect("publish two");
        let receipt = registry
            .rollback_to(
                &package,
                "layout-dark",
                1,
                &authority,
                "restore known display",
            )
            .expect("rollback");

        assert_eq!(receipt.action, PdkDisplayProfileAuditAction::Rollback);
        assert_eq!(receipt.target.revision, 1);
        assert_eq!(
            registry
                .active_for_package(&package)
                .map(|profile| profile.revision),
            Some(1)
        );
    }

    #[test]
    fn tampered_revision_or_receipt_is_rejected() {
        let (package, authority) = fixture();
        let mut registry = PdkDisplayProfileRegistry::default();
        registry
            .publish_and_activate(
                &package,
                PdkDisplayProfileDraft::signed_defaults(&package, "layout-dark", "Layout dark"),
                &authority,
                "initial profile",
            )
            .expect("publish");
        let mut revision_tamper = registry.clone();
        revision_tamper.revisions[0].entries[0].visible = false;
        assert!(matches!(
            revision_tamper.validate_audit_chain(),
            Err(PdkDisplayProfileError::Corrupted(_))
        ));
        let mut receipt_tamper = registry;
        receipt_tamper.audit[0].reason = "rewritten".to_owned();
        assert!(matches!(
            receipt_tamper.validate_audit_chain(),
            Err(PdkDisplayProfileError::Corrupted(_))
        ));
    }
}
