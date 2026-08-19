//! Deterministic validation evidence for the mockup-owned Check and save workflow.
//!
//! The dialog never treats presentation state as save authority. A report binds
//! the exact project baseline, active buffer, hierarchy, libraries, generated
//! netlist, and effective publication scope. The report is re-derived at commit
//! time and must match byte-for-byte before a validated revision may be appended.

use std::collections::BTreeMap;

use serde::Serialize;
use sha2::{Digest as _, Sha256};

use crate::diagnostics::LogAnchor;
use crate::product::ContentDigest;
use crate::services::drc::{
    DrcConfig, DrcLocation, DrcSeverity, run_drc_check_with_hierarchy_and_config,
};
use crate::simulation::netlist_gen::{HierarchySource, generate_netlist_hierarchical};
use crate::state::{
    CellViewRef, SymbolResolver, ValidatedRevisionDependency, ValidationFindingCounts,
};
use crate::workbench::lifecycle::project_lifecycle::{
    SaveScope, accepted_generation, effective_save_scope, snapshot,
};

use crate::workbench::app_state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CheckAndSaveFindingLevel {
    Blocker,
    Advisory,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct CheckAndSaveFinding {
    pub(crate) identity: String,
    pub(crate) level: CheckAndSaveFindingLevel,
    pub(crate) source: String,
    pub(crate) message: String,
    /// Where the finding points on the canvas, when it points anywhere the
    /// author is currently looking.
    ///
    /// Only findings about the active document carry one. The location
    /// vocabulary names an object or a coordinate without naming the drawing
    /// that holds it, so a coordinate carried over from another sheet would
    /// send the canvas to a spot in a document nobody asked to see.
    pub(crate) location: Option<DrcLocation>,
}

/// Where a located finding sends the canvas, through the one resolver every
/// other finding surface uses.
pub(crate) fn finding_anchor(state: &AppState, finding: &CheckAndSaveFinding) -> Option<LogAnchor> {
    crate::schematic::view::violations::location_anchor(state, finding.location.as_ref()?)
}

#[derive(Debug, Clone, PartialEq)]
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
    /// Instances in the active document the one-click interface repair can
    /// act on, by reference designator. The dialog offers the repair for
    /// exactly these, so the row it shows can never fail on the instance it
    /// names.
    stale_interface_instances: Vec<String>,
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
        if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(state) {
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
        // A project need not have a technology; if it has one it must resolve
        // exactly, and if the plan needs one it must have one. The three states
        // carry different remedies, so each owns a distinct finding identity.
        if let Err(error) = state.technology_gate_block_reason() {
            let discriminator = if state.workspace.project.technology_binding().is_none() {
                "technology-required-by-plan"
            } else if state.workspace.project.technology_change_audit().is_empty() {
                "technology-binding-unaudited"
            } else {
                "technology-binding-invalid"
            };
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "technology",
                discriminator,
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

        // What is about to be written: the persisted documents with the live
        // editor buffer over its own. Digests, revision journals, and symbol
        // contracts are statements about these; nothing hierarchy-derived is.
        let mut buffers = state.workspace.schematic_buffers.clone();
        buffers.insert(active_view.key(), state.schematic.clone());
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
            // A warning is a resolved binding the configuration honours
            // differently than it reads. It blocks nothing, but a save that
            // did not record it would seal a design whose configured intent
            // and executed hierarchy disagree with nobody told.
            for warning in &binding.warnings {
                insert_finding(
                    &mut findings,
                    CheckAndSaveFindingLevel::Advisory,
                    "hierarchy",
                    &format!("{}:warning:{warning}", binding.reference.key()),
                    format!(
                        "Configuration warning for {}: {warning}",
                        binding.reference.display_path()
                    ),
                );
            }
        }

        // A stale instance is the one netlist blocker with a one-click remedy,
        // so it is stated as its own finding rather than left inside the
        // generator's prose. The generator still reports why the deck cannot
        // be emitted; this names what to do about it, and the dialog offers
        // the repair beside it.
        let stale_interface_instances = state
            .schematic
            .stale_instance_interfaces(&state.workspace.schematic_buffers);
        for instance in &stale_interface_instances {
            insert_located_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "stale interface",
                instance,
                format!(
                    "Instance '{instance}' no longer presents its master's interface; \
                     select it and run \"Update instance interface\"."
                ),
                state
                    .schematic
                    .components
                    .iter()
                    .find(|component| component.name == *instance)
                    .map(|component| DrcLocation::Component {
                        id: component.id,
                        name: component.name.clone(),
                    }),
            );
        }

        // Which document the canvas is showing decides which findings can
        // offer a place to go — see [`CheckAndSaveFinding::location`].
        let active_key = active_view.key();
        let root_is_active = state
            .workspace
            .simulation_root_reference()
            .key()
            .eq_ignore_ascii_case(&active_key);

        let execution_projection = state.workspace.configuration_execution_projection(
            &state.library_manager,
            &active_view,
            &state.schematic,
        );
        // Every hierarchy-derived check below reads this one projection. Where
        // it does not resolve, its error is the whole finding: re-deriving the
        // same checks from the live editor buffers would seal a receipt for a
        // design the project does not have.
        let configured_hierarchy = execution_projection.as_ref().ok().map(|projection| {
            HierarchySource::from_execution_projection(&state.library_manager, projection)
        });
        match &execution_projection {
            Ok(projection) => {
                let root = projection
                    .root_schematic()
                    .expect("a successful execution projection has a materialized root");
                let configured_hierarchy = configured_hierarchy
                    .as_ref()
                    .expect("a resolved projection binds a hierarchy source");
                let drc = run_drc_check_with_hierarchy_and_config(
                    root,
                    configured_hierarchy,
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
                    insert_located_finding(
                        &mut findings,
                        level,
                        "configured-root checks",
                        &format!(
                            "configured-root:{:?}:{:?}:{}",
                            violation.violation_type, violation.location, violation.message
                        ),
                        format!("Configured root: {}", violation.message),
                        root_is_active.then(|| violation.location.clone()),
                    );
                }
                let generated = generate_netlist_hierarchical(root, &[], configured_hierarchy);
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
                    &state.workspace.project.include_search_chain(),
                    &state.model_library_manager,
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
            // The root is already checked above, through the configured
            // hierarchy. Every other document is checked as the projection
            // materialized it — multi-sheet pages namespaced apart, the active
            // variant applied — because that is the document a run would read.
            let projected = projected_document(execution_projection.as_ref().ok(), key)
                .filter(|_| key != &root_schematic_key);
            if let (Some(hierarchy), Some(schematic)) = (configured_hierarchy.as_ref(), projected) {
                let drc = run_drc_check_with_hierarchy_and_config(
                    schematic,
                    hierarchy,
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
                    insert_located_finding(
                        &mut findings,
                        level,
                        "design checks",
                        &discriminator,
                        format!("{key}: {}", violation.message),
                        key.eq_ignore_ascii_case(&active_key)
                            .then(|| violation.location.clone()),
                    );
                }
            }
            validate_component_contracts(
                key,
                schematic,
                &symbol_resolver,
                &state.property_registry,
                key.eq_ignore_ascii_case(&active_key),
                &mut findings,
            );
            if let (Some(hierarchy), Some(schematic)) = (configured_hierarchy.as_ref(), projected) {
                let generated = generate_netlist_hierarchical(schematic, &[], hierarchy);
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
            stale_interface_instances,
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

    pub(crate) fn stale_interface_instances(&self) -> &[String] {
        &self.stale_interface_instances
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

/// Check one document's naming, parameter and pin contracts.
///
/// `locatable` states whether this document is the one the canvas is showing.
/// Only then may a finding carry the object it is about, because a location
/// names an object without naming the drawing that holds it.
fn validate_component_contracts(
    document_key: &str,
    schematic: &crate::state::SchematicState,
    symbol_resolver: &SymbolResolver<'_>,
    property_registry: &crate::state::PropertyRegistry,
    locatable: bool,
    findings: &mut BTreeMap<String, CheckAndSaveFinding>,
) {
    let segments = document_key.split('/').collect::<Vec<_>>();
    if segments.len() == 3 && !schematic.interface_ports().is_empty() {
        let reference = CellViewRef::new(segments[0], segments[1], segments[2]);
        match symbol_resolver.resolve_reference(&reference) {
            Some(symbol) => {
                for issue in symbol.issues() {
                    // A symbol-pin location names the view it belongs to, so it
                    // is offered whatever the canvas is currently showing.
                    insert_located_finding(
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
                        Some(DrcLocation::SymbolPin {
                            reference: reference.clone(),
                            pin_name: issue.pin_name.clone(),
                            point: issue.point,
                        }),
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
    let at = |component: &crate::state::Component| {
        locatable.then(|| DrcLocation::Component {
            id: component.id,
            name: component.name.clone(),
        })
    };
    let mut designators = BTreeMap::<String, Vec<u64>>::new();
    for component in &schematic.components {
        let component_identity = format!("{document_key}:{}", component.id);
        if !component.kind.spice_prefix().is_empty() {
            if let Err(error) = component.validate_reference_designator(component.name.trim()) {
                insert_located_finding(
                    findings,
                    CheckAndSaveFindingLevel::Blocker,
                    "naming",
                    &format!("{component_identity}:{}", component.name),
                    format!("{document_key}: {}: {error}", component.name),
                    at(component),
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
            insert_located_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "parameters",
                &format!("{component_identity}:{field}:{error}"),
                format!(
                    "{document_key}: {} property {field}: {error}",
                    component.name
                ),
                at(component),
            );
        }

        let Some(binding) = component.library_cell.as_ref() else {
            continue;
        };
        let Some(symbol) = symbol_resolver.resolve_binding(binding) else {
            insert_located_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "pins",
                &format!("{component_identity}:unresolved-symbol"),
                format!(
                    "{document_key}: {} has no resolvable symbol or interface contract.",
                    component.name
                ),
                at(component),
            );
            continue;
        };
        for issue in symbol.issues() {
            insert_located_finding(
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
                at(component),
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

/// One cell view exactly as the projection materialized it, or nothing when
/// the configuration did not resolve. Nothing is the honest answer: a
/// hierarchy-derived finding read off the live editor buffer would name a
/// design the receipt is not sealing.
fn projected_document<'a>(
    projection: Option<&'a crate::state::workspace::ConfigurationExecutionProjection>,
    cell_view_key: &str,
) -> Option<&'a crate::state::SchematicState> {
    projection?
        .schematic_buffers()
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(cell_view_key))
        .map(|(_, schematic)| schematic)
}

fn insert_finding(
    findings: &mut BTreeMap<String, CheckAndSaveFinding>,
    level: CheckAndSaveFindingLevel,
    source: &str,
    discriminator: &str,
    message: impl Into<String>,
) {
    insert_located_finding(findings, level, source, discriminator, message, None);
}

fn insert_located_finding(
    findings: &mut BTreeMap<String, CheckAndSaveFinding>,
    level: CheckAndSaveFindingLevel,
    source: &str,
    discriminator: &str,
    message: impl Into<String>,
    location: Option<DrcLocation>,
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
            location,
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

    /// A finding that knows where it is has to be able to take the author
    /// there. The location resolves through the one anchor resolver every
    /// finding surface uses, and the jump through the one owner of centering
    /// and selection, so a console row and a canvas badge for the same finding
    /// cannot land in two places.
    #[test]
    fn a_located_finding_centers_and_selects_the_object_it_names() {
        let mut state = AppState::default();
        let resistor = state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(40, 40));
        let placed = state
            .schematic
            .components
            .iter_mut()
            .find(|component| component.id == resistor)
            .expect("the placed resistor is retained");
        placed.name = "Q1".to_owned();
        let position = placed.pos;

        let report = CheckAndSaveValidationReport::collect(&state).expect("report");
        let located = report
            .blockers()
            .iter()
            .find(|finding| {
                matches!(
                    &finding.location,
                    Some(DrcLocation::Component { id, .. }) if *id == resistor
                )
            })
            .expect("a designator refused on the active document names its instance");
        assert_eq!(located.source, "naming");

        let anchor = finding_anchor(&state, located).expect("a located finding resolves an anchor");
        state.jump_to_log_anchor(anchor);
        assert_eq!(state.schematic.center_request, Some(position));
        assert!(state.schematic.selection.has_component(resistor));

        // A finding about the project rather than the drawing offers no jump.
        let unlocated = report
            .blockers()
            .iter()
            .find(|finding| finding.location.is_none())
            .expect("the missing-ground finding is about the design, not one object");
        assert!(finding_anchor(&state, unlocated).is_none());
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

    /// The netlist blocker with a one-click remedy has to be findable as such,
    /// or the dialog can only repeat the generator's prose at the author.
    #[test]
    fn a_stale_instance_interface_is_a_blocker_that_names_its_repair() {
        let mut state = AppState::default();
        let mut work = crate::state::Library::new("work");
        let mut div = crate::state::Cell::new("div");
        div.add_view(crate::state::View::new(
            "schematic",
            crate::state::ViewType::Schematic,
        ));
        work.add_cell(div);
        state.library_manager.add_library(work);

        let mut master = crate::state::SchematicState::default();
        let port = master.add_component(ComponentType::Port, Point::new(20, 0));
        master
            .components
            .iter_mut()
            .find(|component| component.id == port)
            .expect("the placed port is retained")
            .value = "a".to_owned();
        state
            .workspace
            .schematic_buffers
            .insert("work/div/schematic".to_owned(), master);

        let mut binding = crate::state::LibraryCellInstance::new("work", "div", "schematic");
        binding.bind_interface(&[crate::state::PortSpec {
            name: "old".to_owned(),
            direction: crate::state::PortDirection::InOut,
        }]);
        let instance = state
            .schematic
            .add_library_cell_component(Point::new(100, 0), binding);
        state
            .schematic
            .components
            .iter_mut()
            .find(|component| component.id == instance)
            .expect("the placed instance is retained")
            .name = "X1".to_owned();

        let report = CheckAndSaveValidationReport::collect(&state).expect("report");

        assert_eq!(report.stale_interface_instances().len(), 1);
        assert_eq!(report.stale_interface_instances()[0], "X1");
        assert!(
            report.blockers().iter().any(|finding| {
                finding.source == "stale interface" && finding.message.contains("X1")
            }),
            "the repairable blocker states the instance: {:?}",
            report.blockers()
        );
    }

    /// A configuration whose stop view names a schematic.
    ///
    /// A schematic is not a terminal implementation, so the resolver descends
    /// into it and records a warning on the otherwise resolved binding: the
    /// configuration is honoured differently than it reads.
    fn state_with_a_stop_view_warning() -> AppState {
        let mut state = AppState::default();
        let mut work = crate::state::Library::new("work");
        let mut amp = crate::state::Cell::new("amp");
        amp.add_view(crate::state::View::new(
            "schematic",
            crate::state::ViewType::Schematic,
        ));
        work.add_cell(amp);
        state.library_manager.add_library(work);

        let mut master = crate::state::SchematicState::default();
        master.add_component(ComponentType::Resistor, Point::new(30, 0));
        state
            .workspace
            .schematic_buffers
            .insert("work/amp/schematic".to_owned(), master);

        let binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
        let instance = state
            .schematic
            .add_library_cell_component(Point::new(100, 0), binding);
        state
            .schematic
            .components
            .iter_mut()
            .find(|component| component.id == instance)
            .expect("the placed instance is retained")
            .name = "X1".to_owned();
        state.sync_active_schematic_to_workspace();

        let root = state.workspace.active_view.clone();
        state
            .workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Stop at a schematic".to_owned(),
                root,
                dut_path: "/X1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: vec!["schematic".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "projection consumer test".to_owned(),
            })
            .expect("the fixture configuration is well formed");
        state
    }

    #[test]
    fn a_configuration_honoured_differently_than_it_reads_is_an_advisory() {
        let state = state_with_a_stop_view_warning();

        let report = CheckAndSaveValidationReport::collect(&state).expect("report");

        assert!(
            report.advisories().iter().any(|finding| {
                finding.source == "hierarchy" && finding.message.contains("stop view")
            }),
            "the reviewed save records the configuration warning: {:?}",
            report.advisories()
        );
    }

    #[test]
    fn an_unresolved_configuration_is_stated_rather_than_checked_from_editor_buffers() {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 10));
        state.sync_active_schematic_to_workspace();
        let root = state.workspace.active_view.clone();
        state
            .workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Unresolvable DUT".to_owned(),
                root,
                dut_path: "/XABSENT".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "projection consumer test".to_owned(),
            })
            .expect("the fixture configuration is well formed");

        let report = CheckAndSaveValidationReport::collect(&state).expect("report");

        assert!(
            report
                .blockers()
                .iter()
                .any(|finding| finding.message.contains("XABSENT")),
            "the report states the projection's own reason: {:?}",
            report.blockers()
        );
        assert!(
            !report
                .blockers()
                .iter()
                .chain(report.advisories())
                .any(|finding| finding.source == "netlist" || finding.source == "design checks"),
            "no check may be derived from the editor buffers once the \
             configuration is refused: {:?}",
            report.blockers()
        );
    }
}
