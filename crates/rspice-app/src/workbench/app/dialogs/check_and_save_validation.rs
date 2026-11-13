//! Deterministic validation evidence for the mockup-owned Check and save workflow.
//!
//! The dialog never treats presentation state as save authority. A report binds
//! the exact project baseline, active buffer, hierarchy, libraries, generated
//! netlist, and effective publication scope. The report is re-derived at commit
//! time and must match byte-for-byte before a validated revision may be appended.

use std::collections::BTreeMap;

use serde::Serialize;
use sha2::{Digest as _, Sha256};

use crate::product::ContentDigest;
use crate::state::{
    CellViewRef, SymbolResolver, ValidatedRevisionDependency, ValidationFindingCounts,
};
use crate::workbench::lifecycle::project_lifecycle::{
    SaveScope, accepted_generation, effective_save_scope, snapshot,
};
use rspice_design::drc::{DrcLocation, DrcSeverity, DrcViolation};
use rspice_design::hierarchy::HierarchySource;
use rspice_simulation::netlist_gen::generate_netlist_hierarchical;

use crate::workbench::app_state::design_checks::CheckScope;
use crate::workbench::app_state::{AppState, Model, PlatformState, SlotFacts};
use crate::workbench::framework::feature::{CodePort, DesignPort};
use crate::workbench::lifecycle::project_lifecycle::{
    LifecycleSource, LifecycleView, ResultPresentationSource,
};
use crate::workbench::state::{WorkbenchSession, Workspace};

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

/// What a report is collected from: the model and the platform services'
/// state, and what the model's design checks read of the features; the
/// workspace in front and the result presentation, which the project's
/// publication snapshot reads as the lifecycle does; and what design and code
/// show.
#[derive(Clone, Copy)]
pub(crate) struct ValidationReads<'a> {
    pub(crate) model: &'a Model,
    pub(crate) platform: &'a PlatformState,
    pub(crate) facts: SlotFacts,
    pub(crate) workspace: Workspace,
    pub(crate) presentation: &'a dyn ResultPresentationSource,
    pub(crate) design: &'a dyn DesignPort,
    pub(crate) code: &'a dyn CodePort,
}

impl<'a> ValidationReads<'a> {
    /// What a report reads, from the whole application state.
    pub(crate) fn of(state: &'a AppState) -> Self {
        let lifecycle = state.lifecycle_view();
        Self {
            model: lifecycle.model,
            platform: lifecycle.platform,
            facts: state.slot_facts(),
            workspace: lifecycle.workspace,
            presentation: lifecycle.presentation,
            design: &*state.design,
            code: &*state.code,
        }
    }
}

impl LifecycleSource for ValidationReads<'_> {
    fn lifecycle_view(&self) -> LifecycleView<'_> {
        LifecycleView {
            model: self.model,
            platform: self.platform,
            workspace: self.workspace,
            presentation: self.presentation,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CheckAndSaveValidationReport {
    pub(crate) effective_scope: SaveScope,
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
    pub(crate) fn collect(reads: &ValidationReads<'_>) -> Result<Self, String> {
        let effective_scope = effective_save_scope(reads.platform, SaveScope::ActiveDocument);
        let active_view = reads.model.workspace.content.active_schematic_reference();
        let project_id = reads.model.workspace.content.project.id().to_string();
        let project_revision = reads.model.workspace.content.project.revision().get();
        let accepted_generation = accepted_generation(reads.platform);
        let mut findings = BTreeMap::<String, CheckAndSaveFinding>::new();
        let mut dependencies = BTreeMap::<String, ContentDigest>::new();

        if !reads.platform.project_lifecycle.is_open() {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "authority",
                "project-closed",
                "No project is open.",
            );
        }
        if reads.model.safe_mode.project_read_only() {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "authority",
                "safe-mode-read-only",
                "Safe mode opened this project read-only.",
            );
        }
        if reads.model.schematic.session.read_only || reads.model.active_view_read_only() {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "authority",
                "schematic-read-only",
                "The active schematic or its owning library is read-only.",
            );
        }
        if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(reads.platform) {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Blocker,
                "authority",
                "lifecycle-operation-in-progress",
                "Another project lifecycle operation is already in progress.",
            );
        }

        if let Err(error) = reads.model.workspace.content.project.validate() {
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
        if let Err(error) = reads
            .model
            .technology_inputs()
            .technology_gate_block_reason()
        {
            let discriminator = if reads
                .model
                .workspace
                .content
                .project
                .technology_binding()
                .is_none()
            {
                "technology-required-by-plan"
            } else if reads
                .model
                .workspace
                .content
                .project
                .technology_change_audit()
                .is_empty()
            {
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
        match reads.model.run_reference_model_cards() {
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
        let mut buffers = reads
            .model
            .workspace
            .content
            .schematic_buffers
            .iter()
            .map(|(key, schematic)| (key.clone(), schematic))
            .collect::<std::collections::HashMap<_, _>>();
        buffers.insert(active_view.key(), reads.model.schematic.editor_ref().design);
        let inspection = reads.model.workspace.inspect_design_projection(
            &reads.model.library_manager,
            &active_view,
            &reads.model.schematic,
        );
        let execution_projection = inspection
            .clone()
            .and_then(|projection| projection.into_execution());
        let hierarchy_bindings = inspection.as_ref().map_or(&[][..], |projection| {
            projection.hierarchy_resolution().bindings.as_slice()
        });
        for binding in hierarchy_bindings {
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

        // Which document the canvas is showing decides which findings can
        // offer a place to go — see [`CheckAndSaveFinding::location`].
        let active_key = active_view.key();
        let root_is_active = reads
            .model
            .workspace
            .content
            .simulation_root_reference()
            .key()
            .eq_ignore_ascii_case(&active_key);

        // Every hierarchy-derived check below reads this one projection. Where
        // it does not resolve, its error is the whole finding: re-deriving the
        // same checks from the live editor buffers would seal a receipt for a
        // design the project does not have.
        let configured_hierarchy = execution_projection.as_ref().ok().map(|projection| {
            HierarchySource::from_execution_projection(
                reads.model.library_manager.catalog(),
                projection,
            )
        });

        // A stale instance is the one netlist blocker with a one-click remedy,
        // so it is stated as its own finding rather than left inside the
        // generator's prose. The generator still reports why the deck cannot
        // be emitted; this names what to do about it, and the dialog offers
        // the repair beside it.
        //
        // The masters it is measured against are the projection's, so this can
        // never offer a repair for an instance the deck would still emit, or
        // withhold it from one the deck refuses.
        //
        // A Verilog-A master's interface is its view's, which the projection
        // refuses to resolve while an instance disagrees with it, so those
        // instances are found whether or not the projection resolved.
        let no_masters = std::collections::HashMap::new();
        let stale_interface_instances = reads.model.schematic.stale_instance_interfaces(
            &reads.model.library_manager,
            execution_projection
                .as_ref()
                .map_or(&no_masters, |projection| projection.schematic_buffers()),
        );
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
                reads
                    .model
                    .schematic
                    .document()
                    .components
                    .iter()
                    .find(|component| component.name == *instance)
                    .map(|component| DrcLocation::Component {
                        id: component.id,
                        name: component.name.clone(),
                    }),
            );
        }
        // A cell's schematic defines its ports, and a module that compiled to
        // different ones is refused by any run that binds it. Saving such a
        // project is legitimate work in progress, so it advises.
        if let Some((reference, message)) = reads.code.interface_check_finding(
            &reads.model.workspace.content.project_sources,
            &reads.model.library_manager,
        ) {
            insert_finding(
                &mut findings,
                CheckAndSaveFindingLevel::Advisory,
                "verilog-a interface",
                &format!("{}:ports-differ-from-schematic", reference.key()),
                message,
            );
        }

        let mut documents = match effective_scope {
            SaveScope::ActiveDocument => {
                vec![(active_view.key(), reads.model.schematic.editor_ref().design)]
            }
            SaveScope::AllDocuments => buffers
                .iter()
                .map(|(key, schematic)| (key.clone(), *schematic))
                .collect::<Vec<_>>(),
        };
        documents.sort_by(|left, right| left.0.cmp(&right.0));
        let root_schematic_key = reads
            .model
            .workspace
            .content
            .simulation_root_reference()
            .key();
        let plan = DesignCheckPlan::of(reads.model, reads.platform);
        // Every defect is stated once. A document in scope states its own
        // findings, below, where they can point at the canvas. A walk through
        // a hierarchy — the configured root's, or an unplaced document's —
        // then states each finding below it that no document's own check
        // already stated with the same rule, cellview and object. Skipping
        // whole cellviews instead dropped the findings a configuration makes
        // that the drawing alone does not: an input the configured master
        // declares where the drawn one declares a bidirectional pin.
        let mut stated_by_documents = std::collections::HashSet::<String>::new();
        let mut walked_findings = Vec::<WalkedFindings>::new();

        match &execution_projection {
            Ok(projection) => {
                let root = projection
                    .root_schematic()
                    .expect("a successful execution projection has a materialized root");
                let configured_hierarchy = configured_hierarchy
                    .as_ref()
                    .expect("a resolved projection binds a hierarchy source");
                // Check Design's receipt for the root: the check, the
                // configuration and the waivers the page lists its findings
                // under and the run gate judges the root under, so a
                // validated save blocks on exactly what both say.
                match plan.root.as_ref().and_then(|root| {
                    reads
                        .model
                        .current_design_check(root, CheckScope::Design, reads.facts)
                }) {
                    Some(drc) => walked_findings.push(WalkedFindings {
                        key: projection.root().key(),
                        source: "configured-root checks",
                        label: "Configured root".to_owned(),
                        discriminator: "configured-root".to_owned(),
                        active: root_is_active,
                        violations: drc.violations().to_vec(),
                    }),
                    None => insert_finding(
                        &mut findings,
                        CheckAndSaveFindingLevel::Blocker,
                        "checks",
                        "configured-root:incomplete",
                        "Design checks have no result for the configured simulation root as it \
                         stands."
                            .to_owned(),
                    ),
                }
                let generated = generate_netlist_hierarchical(
                    root,
                    &[],
                    configured_hierarchy,
                    &rspice_simulation::netlist_gen::NetlistSourceData::new(
                        &crate::simulation::table_route::SourceFiles,
                    ),
                );
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
                let generated_source = reads
                    .model
                    .workspace
                    .content
                    .bind_generated_netlist_provenance(generated.netlist);
                match rspice_simulation::netlist_preparation::dependencies::expand_generated_dependencies(
                    &generated_source,
                    root.current_file(),
                    &reads.model.include_search_chain(),
                    reads.model.model_library_manager.catalog(),
                    reads.model.model_library_manager.resolution_records(),
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

        let symbol_resolver = SymbolResolver::new(&reads.model.library_manager, &buffers);
        for (key, schematic) in &documents {
            if let Err(error) = schematic.document().validated_revisions.validate() {
                insert_finding(
                    &mut findings,
                    CheckAndSaveFindingLevel::Blocker,
                    "revision history",
                    &format!("{key}:invalid-journal"),
                    format!("Validated revision history for {key} is invalid: {error}"),
                );
            }
            // The root is already checked above, through the configured
            // hierarchy. Every other document the plan names is read off the
            // check of it that the plan asks for, published as any check is.
            let projected = projected_document(execution_projection.as_ref().ok(), key)
                .filter(|_| !key.eq_ignore_ascii_case(&root_schematic_key));
            let planned = plan
                .documents
                .iter()
                .find(|(reference, _)| reference.key().eq_ignore_ascii_case(key));
            let checked = planned.map(|(reference, scope)| {
                reads
                    .model
                    .current_design_check(reference, *scope, reads.facts)
            });
            if let Some(None) = checked {
                insert_finding(
                    &mut findings,
                    CheckAndSaveFindingLevel::Blocker,
                    "checks",
                    &format!("{key}:incomplete"),
                    format!("Design checks have no result for {key} as it stands."),
                );
            }
            if let Some(Some(drc)) = checked {
                let (below, own): (Vec<_>, Vec<_>) =
                    drc.violations().iter().cloned().partition(|violation| {
                        matches!(violation.location, DrcLocation::InsideInstance { .. })
                    });
                for violation in &own {
                    stated_by_documents.insert(defect_identity(violation, key));
                    insert_located_finding(
                        &mut findings,
                        finding_level(violation),
                        "design checks",
                        &format!(
                            "{key}:{:?}:{:?}:{}",
                            violation.violation_type, violation.location, violation.message
                        ),
                        format!("{key}: {}", finding_message(violation)),
                        key.eq_ignore_ascii_case(&active_key)
                            .then(|| violation.location.clone()),
                    );
                }
                if !below.is_empty() {
                    walked_findings.push(WalkedFindings {
                        key: key.clone(),
                        source: "design checks",
                        label: key.clone(),
                        discriminator: key.clone(),
                        active: key.eq_ignore_ascii_case(&active_key),
                        violations: below,
                    });
                }
            }
            validate_component_contracts(
                key,
                schematic,
                &symbol_resolver,
                reads,
                key.eq_ignore_ascii_case(&active_key),
                &mut findings,
            );
            if let (Some(hierarchy), Some(schematic)) = (configured_hierarchy.as_ref(), projected) {
                let generated = generate_netlist_hierarchical(
                    schematic,
                    &[],
                    hierarchy,
                    &rspice_simulation::netlist_gen::NetlistSourceData::new(
                        &crate::simulation::table_route::SourceFiles,
                    ),
                );
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

        // The walks state what the documents' own checks did not, the
        // configured root's first. A walk states every occurrence of what it
        // finds, since each names its own path; only what an earlier statement
        // already made is left out.
        let mut stated = stated_by_documents;
        for walked in walked_findings {
            let mut identities = Vec::new();
            for violation in &walked.violations {
                let identity = defect_identity(violation, &walked.key);
                let path = match &violation.location {
                    DrcLocation::InsideInstance { path, .. } => {
                        if stated.contains(&identity) {
                            continue;
                        }
                        Some(path)
                    }
                    _ => None,
                };
                identities.push(identity);
                // A finding inside an occurrence says which one, or two
                // occurrences of one defect read as the same sentence twice.
                let message = match path {
                    Some(path) => {
                        format!("{}, {path}: {}", walked.label, finding_message(violation))
                    }
                    None => format!("{}: {}", walked.label, finding_message(violation)),
                };
                insert_located_finding(
                    &mut findings,
                    finding_level(violation),
                    walked.source,
                    &format!(
                        "{}:{:?}:{:?}:{}",
                        walked.discriminator,
                        violation.violation_type,
                        violation.location,
                        violation.message
                    ),
                    message,
                    walked.active.then(|| violation.location.clone()),
                );
            }
            stated.extend(identities);
        }

        match snapshot(reads) {
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
            digest_serializable(&reads.model.workspace.content.project)?,
        );
        dependencies.insert(
            "project:libraries".to_owned(),
            digest_serializable(&reads.model.library_manager)?,
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
            SaveScope::AllDocuments => "All project documents (first save)".to_owned(),
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
            design_execution_epoch: reads.model.design_execution_epoch,
            active_schematic_epoch: reads.model.active_schematic_epoch,
            active_topology_version: reads.model.schematic.topology_version(),
            library_revision: reads.model.library_manager.revision(),
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
            design_execution_epoch: reads.model.design_execution_epoch,
            active_schematic_epoch: reads.model.active_schematic_epoch,
            active_topology_version: reads.model.schematic.topology_version(),
            library_revision: reads.model.library_manager.revision(),
            document_scope_label,
            stale_interface_instances,
            blockers,
            advisories,
            dependencies,
            receipt_digest,
        })
    }

    /// [`Self::collect`] in the whole application state.
    pub(crate) fn collect_in(state: &AppState) -> Result<Self, String> {
        Self::collect(&ValidationReads::of(state))
    }

    /// The report the dialog opens on, its checks run first in a copy of
    /// `state`, so the state a test built is left as it was.
    #[cfg(test)]
    pub(crate) fn validate(state: &AppState) -> Result<Self, String> {
        let mut state = state.clone();
        run_design_checks_in(&mut state);
        Self::collect_in(&state)
    }

    pub(crate) fn can_save(&self) -> bool {
        self.blockers.is_empty()
    }

    pub(crate) fn source_is_current(&self, reads: &ValidationReads<'_>) -> bool {
        self.project_id == reads.model.workspace.content.project.id().to_string()
            && self.project_revision == reads.model.workspace.content.project.revision().get()
            && self.accepted_generation == accepted_generation(reads.platform)
            && self.design_execution_epoch == reads.model.design_execution_epoch
            && self.active_schematic_epoch == reads.model.active_schematic_epoch
            && self.active_topology_version == reads.model.schematic.topology_version()
            && self.library_revision == reads.model.library_manager.revision()
            && self.active_view == reads.model.workspace.content.active_schematic_reference()
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
fn validate_component_contracts<S: AsRef<rspice_design::schematic::document::SchematicDocument>>(
    document_key: &str,
    schematic: &impl AsRef<rspice_design::schematic::document::SchematicDocument>,
    symbol_resolver: &SymbolResolver<'_, S>,
    reads: &ValidationReads<'_>,
    locatable: bool,
    findings: &mut BTreeMap<String, CheckAndSaveFinding>,
) {
    let schematic = schematic.as_ref();
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
                            "{document_key}: cell symbol pin '{}' {}.",
                            issue.pin_name,
                            pin_issue_text(issue.kind)
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
    // A reference the schema refuses is the design checks' finding, at the
    // severity they give it — an error only where the deck cannot write the
    // instance — so it is not stated a second time here. What is stated here
    // is the collision the design checks do not see: two designators that
    // differ only by case, which the deck folds into one.
    let mut designators = BTreeMap::<String, Vec<(u64, &str)>>::new();
    for component in &schematic.components {
        let component_identity = format!("{document_key}:{}", component.id);
        if !component.kind.spice_prefix().is_empty() {
            designators
                .entry(component.name.to_ascii_lowercase())
                .or_default()
                .push((component.id, component.name.as_str()));
        }

        // The reference is the one property whose emptiness is the design
        // checks' finding rather than the schema's.
        for (field, error) in reads
            .design
            .component_property_findings(reads.model, component)
            .into_iter()
            .filter(|(field, _)| field != "name")
        {
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
        // A value for a parameter the module no longer declares is refused by
        // the netlister; it is named here, with its repair, rather than left
        // to read as a deck error.
        for (key, value) in reads.model.undeclared_instance_parameters(component) {
            let owner = if reads.model.model_card_owns_parameter(component, &key) {
                "its model card sets".to_owned()
            } else {
                format!("{}/{} does not declare", binding.library, binding.cell)
            };
            insert_located_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "parameters",
                &format!("{component_identity}:undeclared:{key}"),
                format!(
                    "{document_key}: {} sets {key}={value}, which {owner}; \
                     select it and press Remove beside {key} in the inspector.",
                    component.name
                ),
                at(component),
            );
        }
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
                    "{document_key}: {} pin '{}' {}.",
                    component.name,
                    issue.pin_name,
                    pin_issue_text(issue.kind)
                ),
                at(component),
            );
        }
    }
    for (name, placed) in designators {
        // One spelling used twice is the design checks' duplicate name.
        let one_spelling = placed.iter().all(|(_, spelling)| *spelling == placed[0].1);
        if placed.len() > 1 && !one_spelling {
            let ids = placed.iter().map(|(id, _)| *id).collect::<Vec<_>>();
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
) -> Option<&'a rspice_design::projection::ProjectedSchematic> {
    projection?
        .schematic_buffers()
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(cell_view_key))
        .map(|(_, schematic)| schematic)
}

/// The design-check findings one walk through a hierarchy made, held until
/// every document has stated its own.
struct WalkedFindings {
    /// The cellview the walk started from.
    key: String,
    source: &'static str,
    /// What the statement names the walk by.
    label: String,
    /// What the finding identity is scoped by.
    discriminator: String,
    /// The walk started from the document the canvas shows, so its findings
    /// can offer a place to go.
    active: bool,
    violations: Vec<DrcViolation>,
}

/// Run each design check a report decides from that does not describe the
/// design as it stands, through the one path every check publishes through,
/// so the page lists what the report reads.
///
/// A design checked since its last edit is not checked again, so a report
/// re-derived at commit reads the evidence the reader reviewed. A check that
/// cannot run leaves no current result, which the report states as a
/// blocker of its own.
pub(crate) fn run_design_checks(
    model: &mut Model,
    platform: &PlatformState,
    facts: SlotFacts,
    wb: &mut WorkbenchSession,
) {
    let plan = DesignCheckPlan::of(model, platform);
    for (subject, scope) in plan
        .root
        .map(|root| (root, CheckScope::Design))
        .into_iter()
        .chain(plan.documents)
    {
        let (_, prompt) = model.ensure_design_check(subject, scope, facts);
        if let Some(prompt) = prompt {
            wb.prompt(prompt);
        }
    }
}

/// [`run_design_checks`] in the whole application state.
pub(crate) fn run_design_checks_in(state: &mut AppState) {
    let facts = state.slot_facts();
    run_design_checks(
        &mut state.model,
        &state.platform,
        facts,
        &mut state.workbench,
    );
}

/// The design checks a report decides from, and how far each reaches.
///
/// The configured root is checked as Check Design checks it. Every other
/// document the save writes is checked as the projection materialized it —
/// multi-sheet pages namespaced apart, the active variant applied — because
/// that is the document a run would read: for its own findings where the
/// root's walk reaches it, since that walk states what lies below it, bound
/// as the configuration binds it; through everything it places where the
/// root does not place it, since no other walk reaches that. Each is checked
/// under the configuration the project checks that cellview under
/// everywhere else, which is also what decides whether it is a design of its
/// own that must reach a ground.
///
/// Nothing is checked where the configuration does not resolve: the report
/// states that instead.
struct DesignCheckPlan {
    root: Option<CellViewRef>,
    documents: Vec<(CellViewRef, CheckScope)>,
}

impl DesignCheckPlan {
    fn of(model: &Model, platform: &PlatformState) -> Self {
        let active_view = model.workspace.content.active_schematic_reference();
        let inspection = model.workspace.inspect_design_projection(
            &model.library_manager,
            &active_view,
            &model.schematic,
        );
        let execution = inspection
            .clone()
            .and_then(|projection| projection.into_execution());
        let (Ok(inspection), Ok(execution)) = (inspection, execution) else {
            return Self {
                root: None,
                documents: Vec::new(),
            };
        };
        let root = model.workspace.content.simulation_root_reference();
        let placed_by_root = inspection
            .hierarchy_resolution()
            .bindings
            .iter()
            .filter(|binding| binding.status.is_resolved())
            .map(|binding| binding.reference.key().to_ascii_lowercase())
            .collect::<std::collections::HashSet<_>>();
        let mut keys = match effective_save_scope(platform, SaveScope::ActiveDocument) {
            SaveScope::ActiveDocument => vec![active_view.key()],
            SaveScope::AllDocuments => model
                .workspace
                .content
                .schematic_buffers
                .keys()
                .cloned()
                .chain(std::iter::once(active_view.key()))
                .collect(),
        };
        keys.sort();
        keys.dedup();
        let documents = keys
            .into_iter()
            .filter(|key| {
                !key.eq_ignore_ascii_case(&root.key())
                    && projected_document(Some(&execution), key).is_some()
            })
            .filter_map(|key| {
                let scope = if placed_by_root.contains(&key.to_ascii_lowercase()) {
                    CheckScope::Cellview
                } else {
                    CheckScope::Hierarchy
                };
                Some((CellViewRef::from_key(&key)?, scope))
            })
            .collect();
        Self {
            root: Some(root),
            documents,
        }
    }
}

/// What makes two design-check findings one defect: the rule, the cellview it
/// is drawn in, and the object it names there. A finding below the checked
/// cellview names that cellview and object inside the occurrence; the path is
/// only where the cellview occurs, so it is not part of the defect.
fn defect_identity(violation: &DrcViolation, checked: &str) -> String {
    let (cellview, location) = match &violation.location {
        DrcLocation::InsideInstance { view, inner, .. } => (view.key(), inner.as_ref()),
        location => (checked.to_owned(), location),
    };
    format!(
        "{:?}|{}|{:?}",
        violation.violation_type,
        cellview.to_ascii_lowercase(),
        location
    )
}

/// A finding blocks the save when it counts as an error. A waived one is
/// listed, as the page lists it, and blocks nothing: it used to block like
/// any other, so a waiver the page and the run honoured still stopped a save.
fn finding_level(violation: &DrcViolation) -> CheckAndSaveFindingLevel {
    if violation.counts() && violation.severity >= DrcSeverity::Error {
        CheckAndSaveFindingLevel::Blocker
    } else {
        CheckAndSaveFindingLevel::Advisory
    }
}

/// What a finding says, and that a waiver covers it where one does.
fn finding_message(violation: &DrcViolation) -> std::borrow::Cow<'_, str> {
    if violation.counts() {
        std::borrow::Cow::Borrowed(&violation.message)
    } else {
        std::borrow::Cow::Owned(format!("{} (waived)", violation.message))
    }
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

/// The digest of `value` as canonical JSON, every object's keys in order.
///
/// A hash map serializes in its own iteration order, and two maps holding the
/// same entries iterate differently when they were filled in another order:
/// the library catalog rebuilt between the check and the save would digest
/// differently with nothing changed. Sorting the keys makes the digest the
/// value's own.
fn digest_serializable(value: &impl Serialize) -> Result<ContentDigest, String> {
    fn sorted(value: serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(entries) => serde_json::Value::Object(
                entries
                    .into_iter()
                    .map(|(key, value)| (key, sorted(value)))
                    .collect::<BTreeMap<_, _>>()
                    .into_iter()
                    .collect(),
            ),
            serde_json::Value::Array(items) => {
                serde_json::Value::Array(items.into_iter().map(sorted).collect())
            }
            scalar => scalar,
        }
    }
    serde_json::to_value(value)
        .and_then(|value| serde_json::to_vec(&sorted(value)))
        .map(|bytes| digest_bytes(&bytes))
        .map_err(|error| format!("validation results could not be serialized: {error}"))
}

/// What a symbol-pin issue says about its pin, in a finding's words.
const fn pin_issue_text(
    kind: rspice_design::resolved_symbol::ResolvedSymbolIssueKind,
) -> &'static str {
    use rspice_design::resolved_symbol::ResolvedSymbolIssueKind;
    match kind {
        ResolvedSymbolIssueKind::UnplacedPin => "is not placed on its symbol",
        ResolvedSymbolIssueKind::OrphanedPin => "is on the symbol but matches no port",
        ResolvedSymbolIssueKind::PinOffGrid => "is off the grid on its symbol",
        ResolvedSymbolIssueKind::InvalidMetadata => "has invalid symbol metadata",
    }
}

fn digest_bytes(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What Check and Save validated is digested as canonical JSON: the same
    /// library catalog, built in two insertion orders, digests alike though
    /// its hash maps iterate differently.
    #[test]
    fn a_catalog_built_in_another_order_digests_alike() {
        use crate::state::{Cell, Library, LibraryManager, View, ViewType};

        let names = (0..24)
            .map(|index| format!("cell{index:02}"))
            .collect::<Vec<_>>();
        let built = |order: Vec<&String>| {
            let mut library = Library::new("work");
            for name in order {
                let mut cell = Cell::new(name);
                cell.add_view(View::new("schematic", ViewType::Schematic));
                library.add_cell(cell);
            }
            let mut manager = LibraryManager::default();
            manager.add_library(library);
            digest_serializable(&manager).expect("a catalog serializes")
        };
        assert_eq!(
            built(names.iter().collect()),
            built(names.iter().rev().collect())
        );
    }
}
