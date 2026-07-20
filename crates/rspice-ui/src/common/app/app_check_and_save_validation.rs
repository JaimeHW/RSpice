//! Deterministic validation evidence for the mockup-owned Check and save workflow.
//!
//! The dialog never treats presentation state as save authority. A report binds
//! the exact project baseline, active buffer, hierarchy, libraries, generated
//! netlist, and effective publication scope. The report is re-derived at commit
//! time and must match byte-for-byte before a validated revision may be appended.

use std::collections::BTreeMap;

use serde::Serialize;
use sha2::{Digest as _, Sha256};

use crate::common::project_lifecycle::{
    SaveScope, accepted_generation, effective_save_scope, snapshot,
};
use crate::product::ContentDigest;
use crate::services::drc::{DrcConfig, DrcSeverity, run_drc_check_with_hierarchy_and_config};
use crate::simulation::netlist_gen::{HierarchySource, generate_netlist_hierarchical};
use crate::state::{
    CellViewRef, SymbolResolver, ValidatedRevisionDependency, ValidationFindingCounts,
};

use super::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CheckAndSaveFindingLevel {
    Blocker,
    Advisory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct CheckAndSaveFinding {
    pub(crate) identity: String,
    pub(crate) level: CheckAndSaveFindingLevel,
    pub(crate) source: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckAndSaveValidationReport {
    effective_scope: SaveScope,
    active_view: CellViewRef,
    project_id: String,
    project_revision: u64,
    accepted_generation: u64,
    design_execution_epoch: u64,
    active_schematic_epoch: u64,
    active_topology_version: u64,
    library_revision: u64,
    document_scope_label: String,
    blockers: Vec<CheckAndSaveFinding>,
    advisories: Vec<CheckAndSaveFinding>,
    dependencies: Vec<ValidatedRevisionDependency>,
    receipt_digest: ContentDigest,
}

impl CheckAndSaveValidationReport {
    pub(crate) fn collect(state: &AppState) -> Result<Self, String> {
        let effective_scope = effective_save_scope(state, SaveScope::ActiveDocument);
        let active_view = state.workspace.active_schematic_reference();
        let project_id = state.workspace.project.id().to_string();
        let project_revision = state.workspace.project.revision().get();
        let accepted_generation = accepted_generation(state);
        let mut findings = BTreeMap::<String, CheckAndSaveFinding>::new();
        let mut dependencies = BTreeMap::<String, ContentDigest>::new();

        if !state.project_lifecycle.project_open {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "authority",
                "project-closed",
                "No project is open.",
            );
        }
        if state.workbench.safe_mode.project_read_only() {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "authority",
                "safe-mode-read-only",
                "Safe mode opened this project read-only.",
            );
        }
        if state.schematic.read_only || state.active_view_read_only() {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "authority",
                "schematic-read-only",
                "The active schematic or its owning library is read-only.",
            );
        }
        if crate::common::project_lifecycle::operation_in_progress(state) {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "authority",
                "lifecycle-operation-in-progress",
                "Another project lifecycle operation is already in progress.",
            );
        }

        if let Err(error) = state.workspace.project.validate() {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "project",
                "descriptor-invalid",
                format!("Project descriptor is invalid: {error}"),
            );
        }
        if let Err(error) = state
            .model_library_manager
            .validate_attached_technology(state.workspace.project.technology_binding())
        {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "models",
                "technology-binding-invalid",
                error,
            );
        }
        match state
            .model_library_manager
            .reference_process_model_cards(state.sim_setup.reference_pvt.process)
        {
            Ok(cards) => {
                dependencies.insert(
                    "models:reference-process-cards".to_owned(),
                    digest_serializable(&cards)?,
                );
            }
            Err(error) => {
                insert_finding(
                    &mut findings,
                    CheckAndSaveFindingLevel::Blocker,
                    "models",
                    "model-source-seal-failed",
                    error,
                );
            }
        }

        let mut buffers = state.workspace.schematic_buffers.clone();
        buffers.insert(active_view.key(), state.schematic.clone());
        let hierarchy = HierarchySource::from_workspace(&state.library_manager, &buffers);
        let resolution = state.workspace.resolve_hierarchy_with_active(
            &state.library_manager,
            &active_view,
            &state.schematic,
        );
        for binding in &resolution.bindings {
            if !binding.status.is_resolved() {
                insert_finding(
                    &mut findings,
                    CheckAndSaveFindingLevel::Blocker,
                    "hierarchy",
                    &format!("{}:{}", binding.reference.key(), binding.status.label()),
                    binding.diagnostic.clone().unwrap_or_else(|| {
                        format!(
                            "Hierarchy binding {} is {}.",
                            binding.reference.display_path(),
                            binding.status.label()
                        )
                    }),
                );
            }
            if binding.used_review_fallback {
                insert_finding(
                    &mut findings,
                    CheckAndSaveFindingLevel::Advisory,
                    "hierarchy",
                    &format!("{}:reviewed-fallback", binding.reference.key()),
                    format!(
                        "Configuration uses reviewed fallback for {} at {}.",
                        binding.reference.display_path(),
                        binding.instance_paths.join(", ")
                    ),
                );
            }
        }

        let execution_projection = state.workspace.configuration_execution_projection(
            &state.library_manager,
            &active_view,
            &state.schematic,
        );
        match &execution_projection {
            Ok(projection) => {
                let root = projection
                    .root_schematic()
                    .expect("a successful execution projection has a materialized root");
                let configured_hierarchy =
                    HierarchySource::from_execution_projection(&state.library_manager, projection);
                let drc = run_drc_check_with_hierarchy_and_config(
                    root,
                    &configured_hierarchy,
                    DrcConfig {
                        check_missing_ground: true,
                        ..DrcConfig::default()
                    },
                );
                if !drc.completed {
                    insert_finding(
                        &mut findings,
                        CheckAndSaveFindingLevel::Blocker,
                        "checks",
                        "configured-root:incomplete",
                        "Design checks did not complete for the configured simulation root."
                            .to_owned(),
                    );
                }
                for violation in drc.violations() {
                    let level = if violation.severity >= DrcSeverity::Error {
                        CheckAndSaveFindingLevel::Blocker
                    } else {
                        CheckAndSaveFindingLevel::Advisory
                    };
                    insert_finding(
                        &mut findings,
                        level,
                        "configured-root checks",
                        &format!(
                            "configured-root:{:?}:{:?}:{}",
                            violation.violation_type, violation.location, violation.message
                        ),
                        format!("Configured root: {}", violation.message),
                    );
                }
                let generated = generate_netlist_hierarchical(root, &[], &configured_hierarchy);
                for error in &generated.errors {
                    insert_finding(
                        &mut findings,
                        CheckAndSaveFindingLevel::Blocker,
                        "configured-root netlist",
                        &format!("configured-root:{error}"),
                        format!("Configured root: {error}"),
                    );
                }
                for warning in &generated.warnings {
                    insert_finding(
                        &mut findings,
                        CheckAndSaveFindingLevel::Advisory,
                        "configured-root netlist",
                        &format!("configured-root:{warning}"),
                        format!("Configured root: {warning}"),
                    );
                }
                let generated_source = state
                    .workspace
                    .bind_generated_netlist_provenance(generated.netlist);
                match crate::simulation::controller::prepared_run::expand_generated_dependencies(
                    &generated_source,
                    root.current_file.as_deref(),
                ) {
                    Ok((sealed_source, sealed_dependencies)) => {
                        dependencies.insert(
                            "configuration:executable-netlist".to_owned(),
                            digest_serializable(&sealed_source)?,
                        );
                        for dependency in sealed_dependencies {
                            dependencies.insert(
                                format!(
                                    "configuration:source:{}:{}",
                                    dependency.resolved_path().display(),
                                    dependency.selected_section().unwrap_or("<entire-source>")
                                ),
                                digest_serializable(&dependency.source())?,
                            );
                        }
                    }
                    Err(error) => insert_finding(
                        &mut findings,
                        CheckAndSaveFindingLevel::Blocker,
                        "configured-root source seal",
                        "configured-root:source-seal-failed",
                        format!("Configured root source sealing failed: {error}"),
                    ),
                }
            }
            Err(error) => insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "configuration",
                "execution-projection-invalid",
                error.to_string(),
            ),
        }

        let mut documents = match effective_scope {
            SaveScope::ActiveDocument => vec![(active_view.key(), state.schematic.clone())],
            SaveScope::AllDocuments => buffers
                .iter()
                .map(|(key, schematic)| (key.clone(), schematic.clone()))
                .collect::<Vec<_>>(),
        };
        documents.sort_by(|left, right| left.0.cmp(&right.0));
        let root_schematic_key = state.workspace.simulation_root_reference().key();
        let symbol_resolver = SymbolResolver::new(&state.library_manager, &buffers);
        for (key, schematic) in &documents {
            if let Err(error) = schematic.validated_revisions.validate() {
                insert_finding(
                    &mut findings,
                    CheckAndSaveFindingLevel::Blocker,
                    "revision history",
                    &format!("{key}:invalid-journal"),
                    format!("Validated revision history for {key} is invalid: {error}"),
                );
            }
            if key != &root_schematic_key || execution_projection.is_err() {
                let drc = run_drc_check_with_hierarchy_and_config(
                    schematic,
                    &hierarchy,
                    DrcConfig::default(),
                );
                if !drc.completed {
                    insert_finding(
                        &mut findings,
                        CheckAndSaveFindingLevel::Blocker,
                        "checks",
                        &format!("{key}:incomplete"),
                        format!("Design checks did not complete for {key}."),
                    );
                }
                for violation in drc.violations() {
                    let level = if violation.severity >= DrcSeverity::Error {
                        CheckAndSaveFindingLevel::Blocker
                    } else {
                        CheckAndSaveFindingLevel::Advisory
                    };
                    let discriminator = format!(
                        "{key}:{:?}:{:?}:{}",
                        violation.violation_type, violation.location, violation.message
                    );
                    insert_finding(
                        &mut findings,
                        level,
                        "design checks",
                        &discriminator,
                        format!("{key}: {}", violation.message),
                    );
                }
            }
            validate_component_contracts(
                key,
                schematic,
                &symbol_resolver,
                &state.property_registry,
                &mut findings,
            );
            if key != &root_schematic_key || execution_projection.is_err() {
                let generated = generate_netlist_hierarchical(schematic, &[], &hierarchy);
                for error in generated.errors {
                    let discriminator = format!("{key}:{error}");
                    insert_finding(
                        &mut findings,
                        CheckAndSaveFindingLevel::Blocker,
                        "netlist",
                        &discriminator,
                        format!("{key}: {error}"),
                    );
                }
                for warning in generated.warnings {
                    let discriminator = format!("{key}:{warning}");
                    insert_finding(
                        &mut findings,
                        CheckAndSaveFindingLevel::Advisory,
                        "netlist",
                        &discriminator,
                        format!("{key}: {warning}"),
                    );
                }
            }
            dependencies.insert(format!("schematic:{key}"), digest_serializable(schematic)?);
        }

        match snapshot(state) {
            Ok(project) => {
                dependencies.insert(
                    "project:publication-snapshot".to_owned(),
                    digest_serializable(&project)?,
                );
            }
            Err(error) => insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "project",
                "publication-snapshot-invalid",
                format!("The project cannot be serialized for publication: {error}"),
            ),
        }
        dependencies.insert(
            "project:descriptor".to_owned(),
            digest_serializable(&state.workspace.project)?,
        );
        dependencies.insert(
            "project:libraries".to_owned(),
            digest_serializable(&state.library_manager)?,
        );

        let mut blockers = Vec::new();
        let mut advisories = Vec::new();
        for finding in findings.into_values() {
            match finding.level {
                CheckAndSaveFindingLevel::Blocker => blockers.push(finding),
                CheckAndSaveFindingLevel::Advisory => advisories.push(finding),
            }
        }
        let dependencies = dependencies
            .into_iter()
            .map(|(identity, digest)| ValidatedRevisionDependency::new(identity, digest))
            .collect::<Vec<_>>();
        let document_scope_label = match effective_scope {
            SaveScope::ActiveDocument => {
                format!("{} \u{00b7} {}", active_view.cell, active_view.view)
            }
            SaveScope::AllDocuments => {
                "All project documents \u{00b7} first canonical save".to_owned()
            }
        };

        #[derive(Serialize)]
        struct ReceiptMaterial<'a> {
            schema: &'static str,
            scope: &'static str,
            active_view: &'a CellViewRef,
            project_id: &'a str,
            project_revision: u64,
            accepted_generation: u64,
            design_execution_epoch: u64,
            active_schematic_epoch: u64,
            active_topology_version: u64,
            library_revision: u64,
            blockers: &'a [CheckAndSaveFinding],
            advisories: &'a [CheckAndSaveFinding],
            dependencies: &'a [ValidatedRevisionDependency],
        }
        let receipt_digest = digest_serializable(&ReceiptMaterial {
            schema: "rspice-check-and-save-validation/v1",
            scope: match effective_scope {
                SaveScope::ActiveDocument => "active_document",
                SaveScope::AllDocuments => "all_documents",
            },
            active_view: &active_view,
            project_id: &project_id,
            project_revision,
            accepted_generation,
            design_execution_epoch: state.design_execution_epoch,
            active_schematic_epoch: state.active_schematic_epoch,
            active_topology_version: state.schematic.topology_version(),
            library_revision: state.library_manager.revision(),
            blockers: &blockers,
            advisories: &advisories,
            dependencies: &dependencies,
        })?;

        Ok(Self {
            effective_scope,
            active_view,
            project_id,
            project_revision,
            accepted_generation,
            design_execution_epoch: state.design_execution_epoch,
            active_schematic_epoch: state.active_schematic_epoch,
            active_topology_version: state.schematic.topology_version(),
            library_revision: state.library_manager.revision(),
            document_scope_label,
            blockers,
            advisories,
            dependencies,
            receipt_digest,
        })
    }

    pub(crate) fn can_save(&self) -> bool {
        self.blockers.is_empty()
    }

    pub(crate) fn source_is_current(&self, state: &AppState) -> bool {
        self.project_id == state.workspace.project.id().to_string()
            && self.project_revision == state.workspace.project.revision().get()
            && self.accepted_generation == accepted_generation(state)
            && self.design_execution_epoch == state.design_execution_epoch
            && self.active_schematic_epoch == state.active_schematic_epoch
            && self.active_topology_version == state.schematic.topology_version()
            && self.library_revision == state.library_manager.revision()
            && self.active_view == state.workspace.active_schematic_reference()
    }

    pub(crate) fn authority_is_resolved(&self) -> bool {
        self.blockers
            .iter()
            .all(|finding| finding.source != "authority")
    }

    pub(crate) fn dependencies_are_enumerated(&self) -> bool {
        !self.dependencies.is_empty()
            && self.blockers.iter().all(|finding| {
                !matches!(finding.source.as_str(), "hierarchy" | "models" | "project")
            })
    }

    pub(crate) fn blockers(&self) -> &[CheckAndSaveFinding] {
        &self.blockers
    }

    pub(crate) fn advisories(&self) -> &[CheckAndSaveFinding] {
        &self.advisories
    }

    pub(crate) fn counts(&self) -> ValidationFindingCounts {
        ValidationFindingCounts {
            blockers: u32::try_from(self.blockers.len()).unwrap_or(u32::MAX),
            advisories: u32::try_from(self.advisories.len()).unwrap_or(u32::MAX),
        }
    }

    pub(crate) fn checks_label(&self) -> String {
        format!(
            "{} blockers \u{00b7} {} advisories",
            self.blockers.len(),
            self.advisories.len()
        )
    }

    pub(crate) fn document_scope_label(&self) -> &str {
        &self.document_scope_label
    }

    pub(crate) fn project_id(&self) -> &str {
        &self.project_id
    }

    pub(crate) const fn project_revision(&self) -> u64 {
        self.project_revision
    }

    pub(crate) fn active_view(&self) -> &CellViewRef {
        &self.active_view
    }

    pub(crate) fn dependencies(&self) -> &[ValidatedRevisionDependency] {
        &self.dependencies
    }

    pub(crate) const fn receipt_digest(&self) -> ContentDigest {
        self.receipt_digest
    }
}

fn validate_component_contracts(
    document_key: &str,
    schematic: &crate::state::SchematicState,
    symbol_resolver: &SymbolResolver<'_>,
    property_registry: &crate::state::PropertyRegistry,
    findings: &mut BTreeMap<String, CheckAndSaveFinding>,
) {
    let segments = document_key.split('/').collect::<Vec<_>>();
    if segments.len() == 3 && !schematic.interface_ports().is_empty() {
        let reference = CellViewRef::new(segments[0], segments[1], segments[2]);
        match symbol_resolver.resolve_reference(&reference) {
            Some(symbol) => {
                for issue in symbol.issues() {
                    insert_finding(
                        findings,
                        CheckAndSaveFindingLevel::Blocker,
                        "pins",
                        &format!(
                            "{document_key}:cell-symbol:{:?}:{}:{:?}",
                            issue.kind, issue.pin_name, issue.point
                        ),
                        format!(
                            "{document_key}: cell symbol pin '{}' has {:?} metadata.",
                            issue.pin_name, issue.kind
                        ),
                    );
                }
            }
            None => insert_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "pins",
                &format!("{document_key}:cell-symbol-unresolved"),
                format!(
                    "{document_key}: the schematic interface has ports but no resolvable symbol contract."
                ),
            ),
        }
    }
    let mut designators = BTreeMap::<String, Vec<u64>>::new();
    for component in &schematic.components {
        let component_identity = format!("{document_key}:{}", component.id);
        if !component.kind.spice_prefix().is_empty() {
            if let Err(error) = component
                .kind
                .validate_reference_designator(component.name.trim())
            {
                insert_finding(
                    findings,
                    CheckAndSaveFindingLevel::Blocker,
                    "naming",
                    &format!("{component_identity}:{}", component.name),
                    format!("{document_key}: {}: {error}", component.name),
                );
            }
            designators
                .entry(component.name.to_ascii_lowercase())
                .or_default()
                .push(component.id);
        }

        for (field, error) in crate::properties::property_bridge::validate_component_properties(
            component,
            property_registry,
        ) {
            insert_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "parameters",
                &format!("{component_identity}:{field}:{error}"),
                format!(
                    "{document_key}: {} property {field}: {error}",
                    component.name
                ),
            );
        }

        let Some(binding) = component.library_cell.as_ref() else {
            continue;
        };
        let Some(symbol) = symbol_resolver.resolve_binding(binding) else {
            insert_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "pins",
                &format!("{component_identity}:unresolved-symbol"),
                format!(
                    "{document_key}: {} has no resolvable symbol or interface contract.",
                    component.name
                ),
            );
            continue;
        };
        for issue in symbol.issues() {
            insert_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "pins",
                &format!(
                    "{component_identity}:{:?}:{}:{:?}",
                    issue.kind, issue.pin_name, issue.point
                ),
                format!(
                    "{document_key}: {} pin '{}' has {:?} symbol metadata.",
                    component.name, issue.pin_name, issue.kind
                ),
            );
        }
    }
    for (name, ids) in designators {
        if ids.len() > 1 {
            insert_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "naming",
                &format!("{document_key}:duplicate:{name}:{ids:?}"),
                format!(
                    "{document_key}: SPICE designator '{name}' is used by {} instances; designators are case-insensitively unique.",
                    ids.len()
                ),
            );
        }
    }
}

fn insert_finding(
    findings: &mut BTreeMap<String, CheckAndSaveFinding>,
    level: CheckAndSaveFindingLevel,
    source: &str,
    discriminator: &str,
    message: impl Into<String>,
) {
    let identity = format!(
        "{}:{}",
        source.replace(' ', "-"),
        digest_bytes(discriminator.as_bytes())
    );
    findings
        .entry(identity.clone())
        .or_insert_with(|| CheckAndSaveFinding {
            identity,
            level,
            source: source.to_owned(),
            message: message.into(),
        });
}

fn digest_serializable(value: &impl Serialize) -> Result<ContentDigest, String> {
    serde_json::to_vec(value)
        .map(|bytes| digest_bytes(&bytes))
        .map_err(|error| format!("validation evidence could not be serialized: {error}"))
}

fn digest_bytes(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point};

    #[test]
    fn validation_is_deterministic_and_stales_after_design_change() {
        let mut state = AppState::default();
        let first = CheckAndSaveValidationReport::collect(&state).expect("first report");
        let second = CheckAndSaveValidationReport::collect(&state).expect("second report");
        assert_eq!(first.receipt_digest(), second.receipt_digest());

        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 10));
        let changed = CheckAndSaveValidationReport::collect(&state).expect("changed report");
        assert_ne!(first.receipt_digest(), changed.receipt_digest());
    }

    #[test]
    fn missing_ground_blocks_validated_save() {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 10));
        let report = CheckAndSaveValidationReport::collect(&state).expect("report");
        assert!(!report.can_save());
        assert!(
            report
                .blockers()
                .iter()
                .any(|finding| { finding.message.to_ascii_lowercase().contains("ground") })
        );
    }

    #[test]
    fn first_save_discloses_all_document_scope() {
        let state = AppState::default();
        let report = CheckAndSaveValidationReport::collect(&state).expect("report");
        assert_eq!(report.effective_scope, SaveScope::AllDocuments);
        assert!(
            report
                .document_scope_label()
                .starts_with("All project documents")
        );
    }
}
