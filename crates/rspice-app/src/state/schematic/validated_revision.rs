//! App coordination for validated-save history, undo and live probe preservation.

use rspice_design::schematic::validated_revision::ValidatedRevisionSource;
pub use rspice_design::schematic::validated_revision::{
    AdvisoryDisposition, MAX_VALIDATED_REVISION_NOTE_LEN, ValidatedRevisionDependency,
    ValidatedRevisionError, ValidatedRevisionJournal, ValidatedRevisionObjectDelta,
    ValidatedRevisionRequest, ValidatedRevisionSemanticDelta, ValidatedSchematicRevision,
    ValidatedSchematicRevisionId, ValidationFindingCounts,
};

use super::document::SchematicDocument;
use super::{SchematicSnapshot, SchematicState};
use crate::product::ContentDigest;
use crate::time_compat::checked_unix_time_ms;

impl SchematicState {
    fn validated_revision_source(&self) -> ValidatedRevisionSource<'_> {
        ValidatedRevisionSource {
            grid_size: self.document.grid_size,
            document_policy: self.document.document_policy,
            components: &self.document.components,
            wires: &self.document.wires,
            buses: &self.document.buses,
            bus_taps: &self.document.bus_taps,
            junctions: &self.document.junctions,
            net_labels: &self.document.net_labels,
            design_notes: &self.document.design_notes,
            documentation_shapes: &self.document.documentation_shapes,
            connections: &self.document.connections,
        }
    }

    pub(crate) fn validated_design_content_digest(
        &self,
    ) -> Result<ContentDigest, ValidatedRevisionError> {
        self.validated_revision_source().design_content_digest()
    }

    pub fn seed_accepted_revision_baseline(
        &mut self,
        accepted: &SchematicState,
        project_id: &str,
        project_revision: u64,
        view_identity: &str,
    ) -> Result<Option<ValidatedSchematicRevisionId>, ValidatedRevisionError> {
        self.document
            .validated_revisions
            .seed_accepted_revision_baseline(
                accepted.validated_revision_source(),
                project_id,
                project_revision,
                view_identity,
                checked_unix_time_ms,
            )
    }

    pub fn append_validated_revision(
        &mut self,
        request: ValidatedRevisionRequest,
    ) -> Result<ValidatedSchematicRevisionId, ValidatedRevisionError> {
        let SchematicDocument {
            grid_size,
            document_policy,
            components,
            wires,
            buses,
            bus_taps,
            junctions,
            net_labels,
            design_notes,
            documentation_shapes,
            connections,
            validated_revisions,
            ..
        } = &mut self.document;
        let source = ValidatedRevisionSource {
            grid_size: *grid_size,
            document_policy: *document_policy,
            components,
            wires,
            buses,
            bus_taps,
            junctions,
            net_labels,
            design_notes,
            documentation_shapes,
            connections,
        };
        let id =
            validated_revisions.append_validated_revision(request, source, checked_unix_time_ms)?;
        self.is_dirty = true;
        Ok(id)
    }

    pub fn remove_unpublished_validated_revision(
        &mut self,
        id: ValidatedSchematicRevisionId,
    ) -> Result<(), ValidatedRevisionError> {
        self.document
            .validated_revisions
            .remove_unpublished_tail(id)
    }

    pub fn restore_validated_revision(
        &mut self,
        id: ValidatedSchematicRevisionId,
    ) -> Result<(), ValidatedRevisionError> {
        if self.read_only {
            return Err(ValidatedRevisionError::ReadOnly);
        }
        let snapshot = self.document.validated_revisions.revision_source(id)?;
        let target = SchematicSnapshot {
            document_policy: snapshot.document_policy,
            grid_size: snapshot.grid_size,
            components: snapshot.components.to_vec(),
            wires: snapshot.wires.to_vec(),
            buses: snapshot.buses.to_vec(),
            bus_taps: snapshot.bus_taps.to_vec(),
            junctions: snapshot.junctions.to_vec(),
            net_labels: snapshot.net_labels.to_vec(),
            design_notes: snapshot.design_notes.to_vec(),
            documentation_shapes: snapshot.documentation_shapes.to_vec(),
            // Probe flags are simulation-output requests rather than
            // validated electrical topology. A design revision restore must
            // therefore preserve the live output markers instead of silently
            // deleting them.
            probes: self.document.probes.clone(),
            connections: snapshot.connections.to_vec(),
            // A stored revision restores the drawing. Sheet membership belongs
            // to the project catalog, which still holds the live one.
            sheet_assignments: std::collections::BTreeMap::new(),
        };
        if target.is_equal_document(&self.document) {
            return Err(ValidatedRevisionError::AlreadyCurrent);
        }
        let changed = self.with_undo("restore validated schematic revision", move |state| {
            state.apply_snapshot(&target);
        });
        if changed {
            Ok(())
        } else {
            Err(ValidatedRevisionError::AlreadyCurrent)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, NetNamingPolicy, Point};
    use uuid::Uuid;

    fn request(state: &SchematicState, project_id: Uuid) -> ValidatedRevisionRequest {
        let design_digest = state
            .validated_design_content_digest()
            .expect("snapshot digest");
        ValidatedRevisionRequest {
            project_id: project_id.to_string(),
            project_revision: 3,
            view_identity: "user/top/schematic".to_owned(),
            revision_note: "Validate compensation network".to_owned(),
            author: "Local project editor".to_owned(),
            validation_receipt_digest: design_digest,
            finding_counts: ValidationFindingCounts {
                blockers: 0,
                advisories: 1,
            },
            dependencies: vec![ValidatedRevisionDependency::new(
                "schematic:user/top/schematic",
                design_digest,
            )],
            advisory_dispositions: vec![AdvisoryDisposition::accepted(
                "drc:unconnected_pin:R1.+",
                "Validated compensation network",
            )],
        }
    }

    #[test]
    fn failed_revision_clock_preserves_the_journal_and_working_design() {
        let project_id = Uuid::new_v4();
        let accepted = SchematicState::default();
        let mut state = accepted.clone();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let design = state.validated_design_content_digest().unwrap();
        let journal = state.document.validated_revisions.clone();
        let dirty = state.is_dirty;
        for epoch in [
            Err("clock unavailable"),
            Ok(std::time::Duration::ZERO),
            Ok(std::time::Duration::MAX),
        ] {
            crate::time_compat::with_unix_epoch(epoch, || {
                assert!(matches!(
                    state.seed_accepted_revision_baseline(
                        &accepted,
                        &project_id.to_string(),
                        3,
                        "user/top/schematic"
                    ),
                    Err(ValidatedRevisionError::ClockUnavailable(_))
                ));
                assert!(matches!(
                    state.append_validated_revision(request(&state, project_id)),
                    Err(ValidatedRevisionError::ClockUnavailable(_))
                ));
            });
            assert_eq!(state.document.validated_revisions, journal);
            assert_eq!(state.is_dirty, dirty);
            assert_eq!(state.validated_design_content_digest().unwrap(), design);
        }
        state
            .append_validated_revision(request(&state, project_id))
            .unwrap();
        assert_eq!(state.document.validated_revisions.records().len(), 1);
    }

    #[test]
    fn journal_retains_restorable_hash_linked_revisions() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let first = state
            .append_validated_revision(request(&state, project_id))
            .expect("first revision");
        state.document.components[0].value = "2k".to_owned();
        let second = state
            .append_validated_revision(request(&state, project_id))
            .expect("second revision");

        state
            .document
            .validated_revisions
            .validate()
            .expect("valid journal");
        assert_eq!(state.document.validated_revisions.records().len(), 2);
        assert_ne!(first, second);
        state
            .restore_validated_revision(first)
            .expect("restore first");
        assert_eq!(state.document.components[0].value, "1k");
    }

    #[test]
    fn restore_recovers_document_semantics_and_grid() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        state.document.document_policy.net_naming = NetNamingPolicy::SpiceCompatibleRelaxed;
        state.document.grid_size = 4;
        let saved = state
            .append_validated_revision(request(&state, project_id))
            .expect("validated revision");

        state.document.document_policy.net_naming = NetNamingPolicy::StrictCaseSensitive;
        state.document.grid_size = 10;
        state.restore_validated_revision(saved).expect("restore");

        assert_eq!(
            state.document.document_policy.net_naming,
            NetNamingPolicy::SpiceCompatibleRelaxed
        );
        assert_eq!(state.document.grid_size, 4);
    }

    #[test]
    fn restore_fails_closed_for_read_only_and_already_current_documents() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let saved = state
            .append_validated_revision(request(&state, project_id))
            .expect("validated revision");
        assert_eq!(
            state.restore_validated_revision(saved),
            Err(ValidatedRevisionError::AlreadyCurrent)
        );

        state.document.components[0].value = "3k".to_owned();
        let changed = state.document.components.clone();
        state.read_only = true;
        assert_eq!(
            state.restore_validated_revision(saved),
            Err(ValidatedRevisionError::ReadOnly)
        );
        assert_eq!(state.document.components, changed);
    }

    #[test]
    fn semantic_delta_reports_stable_object_changes() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let first = state
            .append_validated_revision(request(&state, project_id))
            .expect("first revision");
        state.document.components[0].value = "2k".to_owned();
        state.add_component(ComponentType::Capacitor, Point::new(20, 10));
        state.document.grid_size = 8;
        let second = state
            .append_validated_revision(request(&state, project_id))
            .expect("second revision");
        let first = state
            .document
            .validated_revisions
            .records()
            .iter()
            .find(|record| record.id() == first)
            .unwrap();
        let second = state
            .document
            .validated_revisions
            .records()
            .iter()
            .find(|record| record.id() == second)
            .unwrap();
        let delta = first.semantic_delta_to(second);
        assert_eq!(delta.components.added, 1);
        assert_eq!(delta.components.modified, 1);
        assert!(delta.grid_changed);
    }

    #[test]
    fn unpublished_removal_is_tail_only() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::origin());
        let first = state
            .append_validated_revision(request(&state, project_id))
            .expect("first revision");
        let second = state
            .append_validated_revision(request(&state, project_id))
            .expect("second revision");
        assert_eq!(
            state.remove_unpublished_validated_revision(first),
            Err(ValidatedRevisionError::NotUnpublishedTail)
        );
        state
            .remove_unpublished_validated_revision(second)
            .expect("remove exact tail");
        assert_eq!(state.document.validated_revisions.records().len(), 1);
    }

    #[test]
    fn migrated_baseline_preserves_prior_accepted_design() {
        let project_id = Uuid::new_v4();
        let mut accepted = SchematicState::default();
        accepted.add_component(ComponentType::Capacitor, Point::new(20, 20));
        let mut working = accepted.clone();
        working.document.components[0].value = "2p".to_owned();
        let baseline = working
            .seed_accepted_revision_baseline(
                &accepted,
                &project_id.to_string(),
                7,
                "user/top/schematic",
            )
            .expect("seed baseline")
            .expect("baseline id");
        working
            .append_validated_revision(request(&working, project_id))
            .expect("validated successor");
        working
            .restore_validated_revision(baseline)
            .expect("restore baseline");
        assert_eq!(working.document.components[0].value, "1u");
    }

    #[test]
    fn blockers_and_undisposed_advisories_fail_closed() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        let mut blocked = request(&state, project_id);
        blocked.finding_counts.blockers = 1;
        assert_eq!(
            state.append_validated_revision(blocked),
            Err(ValidatedRevisionError::BlockingFindings(1))
        );
        let mut undisposed = request(&state, project_id);
        undisposed.advisory_dispositions.clear();
        assert_eq!(
            state.append_validated_revision(undisposed),
            Err(ValidatedRevisionError::AdvisoryDispositionCount {
                expected: 1,
                actual: 0,
            })
        );
    }
}
