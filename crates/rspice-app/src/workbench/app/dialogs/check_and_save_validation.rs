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
use crate::state::{
    CellViewRef, SymbolResolver, ValidatedRevisionDependency, ValidationFindingCounts,
};
use crate::workbench::lifecycle::project_lifecycle::{
    SaveScope, accepted_generation, effective_save_scope, snapshot,
};
use rspice_design::drc::{
    CheckReach, DrcConfig, DrcLocation, DrcSeverity, DrcViolation, run_check_of_cellview,
};
use rspice_design::hierarchy::HierarchySource;
use rspice_design::library::ViewType;
use rspice_simulation::netlist_gen::generate_netlist_hierarchical;

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
        let active_view = state.workspace.content.active_schematic_reference();
        let project_id = state.workspace.content.project.id().to_string();
        let project_revision = state.workspace.content.project.revision().get();
        let accepted_generation = accepted_generation(state);
        let mut findings = BTreeMap::<String, CheckAndSaveFinding>::new();
        let mut dependencies = BTreeMap::<String, ContentDigest>::new();

        if !state.project_lifecycle.is_open() {
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
        if state.schematic.session.read_only || state.active_view_read_only() {
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

        if let Err(error) = state.workspace.content.project.validate() {
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
            let discriminator = if state
                .workspace
                .content
                .project
                .technology_binding()
                .is_none()
            {
                "technology-required-by-plan"
            } else if state
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
        let mut buffers = state
            .workspace
            .content
            .schematic_buffers
            .iter()
            .map(|(key, schematic)| (key.clone(), schematic))
            .collect::<std::collections::HashMap<_, _>>();
        buffers.insert(active_view.key(), state.schematic.editor_ref().design);
        let inspection = state.workspace.inspect_design_projection(
            &state.library_manager,
            &active_view,
            &state.schematic,
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
        let root_is_active = state
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
            HierarchySource::from_execution_projection(state.library_manager.catalog(), projection)
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
        let stale_interface_instances = state.schematic.stale_instance_interfaces(
            &state.library_manager,
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
                state
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
        if let Some((reference, message)) =
            crate::workbench::documents::code_workspace::interface_check_finding(
                &state.ui.code_workspace.veriloga,
                &state.workspace.content.project_sources,
            )
        {
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
                vec![(active_view.key(), state.schematic.editor_ref().design)]
            }
            SaveScope::AllDocuments => buffers
                .iter()
                .map(|(key, schematic)| (key.clone(), *schematic))
                .collect::<Vec<_>>(),
        };
        documents.sort_by(|left, right| left.0.cmp(&right.0));
        let root_schematic_key = state.workspace.content.simulation_root_reference().key();
        // The cellviews the configured root's walk reaches, which it checks
        // with everything below them as the configuration binds them.
        let placed_by_root = hierarchy_bindings
            .iter()
            .filter(|binding| binding.status.is_resolved())
            .map(|binding| binding.reference.key().to_ascii_lowercase())
            .collect::<std::collections::HashSet<_>>();
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
                let drc = run_check_of_cellview(
                    root,
                    projection.root(),
                    configured_hierarchy,
                    DrcConfig {
                        check_missing_ground: true,
                        ..DrcConfig::default()
                    },
                    CheckReach::Hierarchy,
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
                walked_findings.push(WalkedFindings {
                    key: projection.root().key(),
                    source: "configured-root checks",
                    label: "Configured root".to_owned(),
                    discriminator: "configured-root".to_owned(),
                    active: root_is_active,
                    violations: drc.violations().to_vec(),
                });
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
                let generated_source = state
                    .workspace
                    .content
                    .bind_generated_netlist_provenance(generated.netlist);
                match rspice_simulation::netlist_preparation::dependencies::expand_generated_dependencies(
                    &generated_source,
                    root.current_file(),
                    &rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(state.workspace.content.project.include_search_paths(), state.workspace.content.project.data_root()),
                    state.model_library_manager.catalog(),
                    state.model_library_manager.resolution_records(),
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

        let symbol_resolver = SymbolResolver::new(&state.library_manager, &buffers);
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
            // hierarchy. Every other document is checked as the projection
            // materialized it — multi-sheet pages namespaced apart, the active
            // variant applied — because that is the document a run would read.
            //
            // One the root's walk reaches is checked for its own findings
            // only: the root's walk states what lies below it, bound as the
            // configuration binds it. One the root does not place is checked
            // through everything it places, as its own drawing binds it,
            // because no other walk reaches that.
            //
            // A cell takes its reference from whatever places it, so it is not
            // asked for a ground. A document the root does not place and that
            // presents no interface, or is a testbench, is a design of its own,
            // and it is.
            let projected = projected_document(execution_projection.as_ref().ok(), key)
                .filter(|_| !key.eq_ignore_ascii_case(&root_schematic_key));
            let reference = cell_view_reference(key);
            if let (Some(hierarchy), Some(schematic), Some(reference)) =
                (configured_hierarchy.as_ref(), projected, reference.as_ref())
            {
                let placed = placed_by_root.contains(&key.to_ascii_lowercase());
                let own_design = !placed
                    && (!schematic.document().has_interface()
                        || hierarchy.resolved_view_type(reference) == ViewType::Testbench);
                let drc = run_check_of_cellview(
                    schematic,
                    reference,
                    hierarchy,
                    DrcConfig {
                        check_missing_ground: own_design,
                        ..DrcConfig::default()
                    },
                    if placed {
                        CheckReach::Cellview
                    } else {
                        CheckReach::Hierarchy
                    },
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
                        format!("{key}: {}", violation.message),
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
                state,
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
                    Some(path) => format!("{}, {path}: {}", walked.label, violation.message),
                    None => format!("{}: {}", walked.label, violation.message),
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
            digest_serializable(&state.workspace.content.project)?,
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
        self.project_id == state.workspace.content.project.id().to_string()
            && self.project_revision == state.workspace.content.project.revision().get()
            && self.accepted_generation == accepted_generation(state)
            && self.design_execution_epoch == state.design_execution_epoch
            && self.active_schematic_epoch == state.active_schematic_epoch
            && self.active_topology_version == state.schematic.topology_version()
            && self.library_revision == state.library_manager.revision()
            && self.active_view == state.workspace.content.active_schematic_reference()
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

/// Resolve dynamic contracts without replacing the active editor's sheet.
fn component_property_findings(
    state: &AppState,
    component: &crate::state::Component,
) -> Vec<(String, String)> {
    use crate::properties::property_bridge::validate_component_properties;
    use crate::workbench::app::actions::property_edit::{
        authoritative_component_property_sheet, cell_instance_identity_sheet,
    };

    if component.kind != crate::state::ComponentType::CellInstance {
        return validate_component_properties(
            component,
            state.property_registry.get(component.kind),
        );
    }
    let sheet = match authoritative_component_property_sheet(state, component) {
        Ok(sheet) => sheet.unwrap_or_else(cell_instance_identity_sheet),
        Err(error) => return vec![("schema".to_owned(), error)],
    };
    validate_component_properties(component, Some(&sheet))
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
    state: &AppState,
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
        for (field, error) in component_property_findings(state, component)
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
        for (key, value) in crate::workbench::app::undeclared_instance_parameters(state, component)
        {
            insert_located_finding(
                findings,
                CheckAndSaveFindingLevel::Blocker,
                "parameters",
                &format!("{component_identity}:undeclared:{key}"),
                format!(
                    "{document_key}: {} sets {key}={value}, which {}/{} does not declare; \
                     select it and press Remove beside {key} in the inspector.",
                    component.name, binding.library, binding.cell
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
                    "{document_key}: {} pin '{}' has {:?} symbol metadata.",
                    component.name, issue.pin_name, issue.kind
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

/// The cell view a `library/cell/view` buffer key names.
fn cell_view_reference(key: &str) -> Option<CellViewRef> {
    let mut segments = key.split('/');
    match (
        segments.next(),
        segments.next(),
        segments.next(),
        segments.next(),
    ) {
        (Some(library), Some(cell), Some(view), None) => {
            Some(CellViewRef::new(library, cell, view))
        }
        _ => None,
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

fn finding_level(violation: &DrcViolation) -> CheckAndSaveFindingLevel {
    if violation.severity >= DrcSeverity::Error {
        CheckAndSaveFindingLevel::Blocker
    } else {
        CheckAndSaveFindingLevel::Advisory
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
    fn hierarchical_instances_validate_without_an_open_property_dialog() {
        let state = crate::workbench::examples::hierarchy_reference::build().state;
        assert!(
            state
                .property_registry
                .get(ComponentType::CellInstance)
                .is_none()
        );
        let report = CheckAndSaveValidationReport::collect(&state).unwrap();
        let parameters = report
            .blockers()
            .iter()
            .filter(|finding| finding.source == "parameters")
            .collect::<Vec<_>>();
        assert!(parameters.is_empty(), "{parameters:?}");
    }

    #[test]
    fn omitted_instances_do_not_reappear_in_validated_save_hierarchy_findings() {
        let mut state = crate::workbench::examples::hierarchy_reference::build().state;
        let baseline = CheckAndSaveValidationReport::collect(&state).unwrap();
        assert!(
            baseline
                .blockers()
                .iter()
                .all(|finding| finding.source != "hierarchy")
        );
        crate::workbench::examples::hierarchy_reference::omit_missing_instance(&mut state);
        let authored = serde_json::to_value(&state.schematic).unwrap();
        let report = CheckAndSaveValidationReport::collect(&state).unwrap();
        assert!(
            report
                .blockers()
                .iter()
                .all(|finding| finding.source != "hierarchy"),
            "{:?}",
            report.blockers()
        );
        assert_eq!(serde_json::to_value(&state.schematic).unwrap(), authored);
    }

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
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == resistor)
            .expect("the placed resistor is retained");
        placed.name = "Q1".to_owned();
        let position = placed.pos;

        let report = CheckAndSaveValidationReport::collect(&state).expect("report");
        // One refused designator is one finding. The netlister writes this
        // resistor as `RQ1` and the run starts, so it advises rather than
        // blocks.
        let naming = report
            .blockers()
            .iter()
            .chain(report.advisories())
            .filter(|finding| finding.message.contains("'Q1'"))
            .collect::<Vec<_>>();
        assert_eq!(naming.len(), 1, "{naming:?}");
        let located = naming[0];
        assert_eq!(located.level, CheckAndSaveFindingLevel::Advisory);
        assert!(located.message.contains("RQ1"), "{}", located.message);
        assert!(
            matches!(
                &located.location,
                Some(DrcLocation::Component { id, .. }) if *id == resistor
            ),
            "a designator refused on the active document names its instance: {located:?}"
        );

        let anchor = finding_anchor(&state, located).expect("a located finding resolves an anchor");
        state.jump_to_log_anchor(anchor);
        assert_eq!(
            state.schematic.session.editor.center_request,
            Some(position)
        );
        assert!(
            state
                .schematic
                .session
                .editor
                .selection
                .has_component(resistor)
        );

        // A finding about the project rather than the drawing offers no jump.
        let unlocated = CheckAndSaveFinding {
            location: None,
            ..located.clone()
        };
        assert!(finding_anchor(&state, &unlocated).is_none());
    }

    /// A first save checks every document, and every cellview exactly once.
    /// A leaf with one open resistor, placed twice by a cell the root places
    /// twice, used to be stated by the root at four occurrences, by the middle
    /// cell at two and by the leaf itself — fourteen rows for two open legs.
    #[test]
    fn a_first_save_states_each_cellviews_findings_once() {
        let mut state = AppState::default();
        let mut work = crate::state::Library::new("work");
        for name in ["leaf", "mid"] {
            let mut cell = crate::state::Cell::new(name);
            cell.add_view(crate::state::View::new(
                "schematic",
                crate::state::ViewType::Schematic,
            ));
            work.add_cell(cell);
        }
        state.library_manager.add_library(work);
        let placed = |cell: &str| {
            let mut binding = crate::state::LibraryCellInstance::new("work", cell, "schematic");
            binding.bind_interface(&[]);
            binding
        };

        let mut leaf = crate::state::SchematicState::default();
        leaf.add_component(ComponentType::Resistor, Point::new(30, 0));
        state
            .workspace
            .insert_schematic_editor("work/leaf/schematic".to_owned(), leaf);
        let mut mid = crate::state::SchematicState::default();
        mid.add_library_cell_component(Point::new(100, 0), placed("leaf"));
        mid.add_library_cell_component(Point::new(300, 0), placed("leaf"));
        state
            .workspace
            .insert_schematic_editor("work/mid/schematic".to_owned(), mid);
        state
            .schematic
            .add_library_cell_component(Point::new(100, 0), placed("mid"));
        state
            .schematic
            .add_library_cell_component(Point::new(300, 0), placed("mid"));
        state.sync_active_schematic_to_workspace();

        let report = CheckAndSaveValidationReport::collect(&state).expect("report");
        assert_eq!(report.effective_scope, SaveScope::AllDocuments);

        let open_legs = report
            .blockers()
            .iter()
            .chain(report.advisories())
            .filter(|finding| finding.message.contains("is not connected"))
            .collect::<Vec<_>>();
        assert_eq!(open_legs.len(), 2, "{open_legs:?}");
        assert!(
            open_legs
                .iter()
                .all(|finding| finding.message.starts_with("work/leaf/schematic: ")),
            "each is stated by the leaf that holds it: {open_legs:?}"
        );
        // No cell is asked for a ground of its own: it takes its reference
        // from whatever places it.
        assert!(
            report.blockers().iter().all(|finding| !finding
                .message
                .starts_with("work/leaf/schematic: Circuit has no ground")
                && !finding
                    .message
                    .starts_with("work/mid/schematic: Circuit has no ground")),
            "{:?}",
            report.blockers()
        );
    }

    /// A port named `name` declaring `direction`.
    fn add_port(schematic: &mut crate::state::SchematicState, name: &str, direction: &str) {
        let port = schematic.add_component(ComponentType::Port, Point::new(20, 0));
        let placed = schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == port)
            .expect("the placed port is retained");
        placed.value = name.to_owned();
        placed.params = format!("dir={direction}");
    }

    /// Place `cell`'s schematic in `schematic` as `name`, presenting `ports`.
    fn place(
        schematic: &mut crate::state::SchematicState,
        cell: &str,
        name: &str,
        ports: &[crate::state::PortSpec],
        at: Point,
    ) {
        let mut binding = crate::state::LibraryCellInstance::new("work", cell, "schematic");
        binding.bind_interface(ports);
        let instance = schematic.add_library_cell_component(at, binding);
        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == instance)
            .expect("the placed instance is retained")
            .name = name.to_owned();
    }

    /// A library `work` holding `cells`, each with the listed schematic views.
    fn add_cells(state: &mut AppState, cells: &[(&str, &[&str])]) {
        let mut work = crate::state::Library::new("work");
        for &(name, views) in cells {
            let mut cell = crate::state::Cell::new(name);
            for &view in views {
                cell.add_view(crate::state::View::new(
                    view,
                    crate::state::ViewType::Schematic,
                ));
            }
            work.add_cell(cell);
        }
        state.library_manager.add_library(work);
    }

    /// The configuration binds `mid`'s instance of `leaf` to a view whose pin
    /// is an input, where the placed view declares it bidirectional. `mid`'s
    /// own check, reading the placed binding, finds an open pin; the root's
    /// walk finds an undriven input. They are two findings, and the second is
    /// the one the run refuses, so the save must state it too.
    #[test]
    fn a_finding_only_the_configuration_makes_is_stated_beside_the_cells_own() {
        let mut state = AppState::default();
        add_cells(
            &mut state,
            &[
                ("leaf", &["schematic", "schematic_v2"]),
                ("mid", &["schematic"]),
            ],
        );
        for (view, direction) in [("schematic", "inout"), ("schematic_v2", "in")] {
            let mut leaf = crate::state::SchematicState::default();
            add_port(&mut leaf, "a", direction);
            state
                .workspace
                .insert_schematic_editor(format!("work/leaf/{view}"), leaf);
        }
        let mut mid = crate::state::SchematicState::default();
        place(
            &mut mid,
            "leaf",
            "XA",
            &[crate::state::PortSpec {
                name: "a".to_owned(),
                direction: crate::state::PortDirection::InOut,
            }],
            Point::new(100, 0),
        );
        state
            .workspace
            .insert_schematic_editor("work/mid/schematic".to_owned(), mid);
        place(&mut state.schematic, "mid", "XM", &[], Point::new(100, 0));
        state.sync_active_schematic_to_workspace();
        let root = state.workspace.content.active_view.clone();
        state
            .workspace
            .content
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Second leaf".to_owned(),
                root,
                dut_path: "/XM".to_owned(),
                executable_view_policy: vec!["schematic_v2".to_owned(), "schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "check and save test".to_owned(),
            })
            .expect("the fixture configuration is well formed");

        let report = CheckAndSaveValidationReport::collect(&state).expect("report");

        assert!(
            report
                .advisories()
                .iter()
                .any(|finding| finding.message == "work/mid/schematic: Pin XA.a is not connected"),
            "{:?}",
            report.advisories()
        );
        assert!(
            report
                .blockers()
                .iter()
                .any(|finding| finding.message
                    == "Configured root, /XM: Input pin XA.a has no driver"),
            "{:?}",
            report.blockers()
        );
    }

    /// A document the configured root does not place is checked through
    /// what it places, which no other check reaches; and where it is a design
    /// of its own rather than a cell, it is asked for a ground.
    #[test]
    fn an_unplaced_document_is_checked_through_its_hierarchy() {
        let mut state = AppState::default();
        add_cells(
            &mut state,
            &[
                ("supply", &["schematic"]),
                ("bench", &["schematic"]),
                ("stray", &["schematic"]),
                ("buffer", &["schematic"]),
            ],
        );
        // One source holds vdd! against ground. Placed twice, the two copies
        // are a loop, which neither copy's cell can see.
        let mut supply = crate::state::SchematicState::default();
        let source = supply.add_component(ComponentType::VoltageSource, Point::new(30, 0));
        let terminals = supply
            .document()
            .components
            .iter()
            .find(|component| component.id == source)
            .map(|component| component.terminal_positions())
            .expect("the source is placed");
        for (id, (pin, node)) in (900..).zip([("+", "vdd!"), ("-", "0")]) {
            let point = terminals
                .iter()
                .find_map(|(name, point)| (*name == pin).then_some(*point))
                .expect("a voltage source has both terminals");
            supply
                .document_mut_for_test()
                .net_labels
                .push(crate::state::NetLabel::new(id, point, node));
        }
        state
            .workspace
            .insert_schematic_editor("work/supply/schematic".to_owned(), supply);
        let mut bench = crate::state::SchematicState::default();
        place(&mut bench, "supply", "X1", &[], Point::new(100, 0));
        place(&mut bench, "supply", "X2", &[], Point::new(300, 0));
        state
            .workspace
            .insert_schematic_editor("work/bench/schematic".to_owned(), bench);

        // A resistor on its own, with no ground anywhere: a design that cannot
        // solve. The same resistor behind a port is a cell, which takes its
        // reference from whatever places it.
        let mut stray = crate::state::SchematicState::default();
        stray.add_component(ComponentType::Resistor, Point::new(30, 0));
        let mut buffer = stray.clone();
        add_port(&mut buffer, "a", "inout");
        state
            .workspace
            .insert_schematic_editor("work/stray/schematic".to_owned(), stray);
        state
            .workspace
            .insert_schematic_editor("work/buffer/schematic".to_owned(), buffer);
        state.sync_active_schematic_to_workspace();

        let report = CheckAndSaveValidationReport::collect(&state).expect("report");
        assert_eq!(report.effective_scope, SaveScope::AllDocuments);

        let loops = report
            .blockers()
            .iter()
            .filter(|finding| finding.message.contains("form a closed loop"))
            .collect::<Vec<_>>();
        assert_eq!(loops.len(), 1, "{:?}", report.blockers());
        assert!(
            loops[0]
                .message
                .starts_with("work/bench/schematic, /X1: 2 voltage sources"),
            "{}",
            loops[0].message
        );

        let unground = |key: &str| {
            report.blockers().iter().any(|finding| {
                finding
                    .message
                    .starts_with(&format!("{key}: Circuit has no ground"))
            })
        };
        assert!(unground("work/stray/schematic"), "{:?}", report.blockers());
        assert!(
            !unground("work/buffer/schematic"),
            "{:?}",
            report.blockers()
        );
        assert!(!unground("work/bench/schematic"), "{:?}", report.blockers());
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
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == port)
            .expect("the placed port is retained")
            .value = "a".to_owned();
        state
            .workspace
            .insert_schematic_editor("work/div/schematic".to_owned(), master);

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
            .document_mut_for_test()
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
            .insert_schematic_editor("work/amp/schematic".to_owned(), master);

        let binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
        let instance = state
            .schematic
            .add_library_cell_component(Point::new(100, 0), binding);
        state
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == instance)
            .expect("the placed instance is retained")
            .name = "X1".to_owned();
        state.sync_active_schematic_to_workspace();

        let root = state.workspace.content.active_view.clone();
        state
            .workspace
            .content
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
        let root = state.workspace.content.active_view.clone();
        state
            .workspace
            .content
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
