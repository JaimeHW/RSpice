//! App coordination for validated-save history, undo and live probe preservation.

pub use rspice_design::schematic::validated_revision::{
    AdvisoryDisposition, MAX_VALIDATED_REVISION_NOTE_LEN, ValidatedRevisionDependency,
    ValidatedRevisionError, ValidatedRevisionJournal, ValidatedRevisionObjectDelta,
    ValidatedRevisionRequest, ValidatedRevisionSemanticDelta, ValidatedSchematicRevision,
    ValidatedSchematicRevisionId, ValidationFindingCounts,
};

use super::{SchematicSnapshot, SchematicState};
use crate::product::ContentDigest;
use crate::time_compat::checked_unix_time_ms;

impl SchematicState {
    pub(crate) fn validated_design_content_digest(
        &self,
    ) -> Result<ContentDigest, ValidatedRevisionError> {
        self.design.validated_design_content_digest()
    }

    pub fn seed_accepted_revision_baseline(
        &mut self,
        accepted: &SchematicState,
        project_id: &str,
        project_revision: u64,
        view_identity: &str,
    ) -> Result<Option<ValidatedSchematicRevisionId>, ValidatedRevisionError> {
        self.design.seed_accepted_revision_baseline(
            &accepted.design,
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
        let id = self
            .design
            .append_validated_revision(request, checked_unix_time_ms)?;
        self.is_dirty = true;
        Ok(id)
    }

    pub fn remove_unpublished_validated_revision(
        &mut self,
        id: ValidatedSchematicRevisionId,
    ) -> Result<(), ValidatedRevisionError> {
        self.design.remove_unpublished_validated_revision(id)
    }

    pub fn restore_validated_revision(
        &mut self,
        id: ValidatedSchematicRevisionId,
    ) -> Result<(), ValidatedRevisionError> {
        if self.read_only {
            return Err(ValidatedRevisionError::ReadOnly);
        }
        let snapshot = self
            .design
            .document()
            .validated_revisions
            .revision_source(id)?;
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
            probes: self.design.document().probes.clone(),
            connections: snapshot.connections.to_vec(),
            // A stored revision restores the drawing. Sheet membership belongs
            // to the project catalog, which still holds the live one.
            sheet_assignments: std::collections::BTreeMap::new(),
        };
        if target.is_equal_document(&self.design.document()) {
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
        let journal = state.design.document().validated_revisions.clone();
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
            assert_eq!(state.design.document().validated_revisions, journal);
            assert_eq!(state.is_dirty, dirty);
            assert_eq!(state.validated_design_content_digest().unwrap(), design);
        }
        state
            .append_validated_revision(request(&state, project_id))
            .unwrap();
        assert_eq!(
            state.design.document().validated_revisions.records().len(),
            1
        );
    }

    #[test]
    fn journal_retains_restorable_hash_linked_revisions() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let first = state
            .append_validated_revision(request(&state, project_id))
            .expect("first revision");
        state.design.document_mut_for_test().components[0].value = "2k".to_owned();
        let second = state
            .append_validated_revision(request(&state, project_id))
            .expect("second revision");

        state
            .design
            .document()
            .validated_revisions
            .validate()
            .expect("valid journal");
        assert_eq!(
            state.design.document().validated_revisions.records().len(),
            2
        );
        assert_ne!(first, second);
        state
            .restore_validated_revision(first)
            .expect("restore first");
        assert_eq!(state.design.document().components[0].value, "1k");
    }

    #[test]
    fn restore_recovers_document_semantics_and_grid() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        state
            .design
            .document_mut_for_test()
            .document_policy
            .net_naming = NetNamingPolicy::SpiceCompatibleRelaxed;
        state.design.document_mut_for_test().grid_size = 4;
        let saved = state
            .append_validated_revision(request(&state, project_id))
            .expect("validated revision");

        state
            .design
            .document_mut_for_test()
            .document_policy
            .net_naming = NetNamingPolicy::StrictCaseSensitive;
        state.design.document_mut_for_test().grid_size = 10;
        state.restore_validated_revision(saved).expect("restore");

        assert_eq!(
            state.design.document().document_policy.net_naming,
            NetNamingPolicy::SpiceCompatibleRelaxed
        );
        assert_eq!(state.design.document().grid_size, 4);
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

        state.design.document_mut_for_test().components[0].value = "3k".to_owned();
        let changed = state.design.document().components.clone();
        state.read_only = true;
        assert_eq!(
            state.restore_validated_revision(saved),
            Err(ValidatedRevisionError::ReadOnly)
        );
        assert_eq!(state.design.document().components, changed);
    }

    #[test]
    fn semantic_delta_reports_stable_object_changes() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let first = state
            .append_validated_revision(request(&state, project_id))
            .expect("first revision");
        state.design.document_mut_for_test().components[0].value = "2k".to_owned();
        state.add_component(ComponentType::Capacitor, Point::new(20, 10));
        state.design.document_mut_for_test().grid_size = 8;
        let second = state
            .append_validated_revision(request(&state, project_id))
            .expect("second revision");
        let first = state
            .design
            .document()
            .validated_revisions
            .records()
            .iter()
            .find(|record| record.id() == first)
            .unwrap();
        let second = state
            .design
            .document()
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
        assert_eq!(
            state.design.document().validated_revisions.records().len(),
            1
        );
    }

    #[test]
    fn migrated_baseline_preserves_prior_accepted_design() {
        let project_id = Uuid::new_v4();
        let mut accepted = SchematicState::default();
        accepted.add_component(ComponentType::Capacitor, Point::new(20, 20));
        let mut working = accepted.clone();
        working.design.document_mut_for_test().components[0].value = "2p".to_owned();
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
        assert_eq!(working.design.document().components[0].value, "1u");
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

impl SchematicState {
    pub(crate) fn copy_without_validated_revisions(&self) -> Self {
        self.clone_with_design(self.design.copy_without_validated_revisions())
    }

    pub(crate) fn record_validated_save_revision(
        &mut self,
        request: ValidatedRevisionRequest,
        original_journal: ValidatedRevisionJournal,
        original_dirty: bool,
    ) -> Result<
        (
            ValidatedSchematicRevisionId,
            ValidatedRevisionJournal,
            ValidatedRevisionJournal,
            ContentDigest,
        ),
        String,
    > {
        match self.design.record_validated_save_revision(
            request,
            original_journal,
            checked_unix_time_ms,
        ) {
            Ok(record) => {
                self.is_dirty = true;
                Ok(record)
            }
            Err(error) => {
                self.is_dirty = original_dirty;
                Err(error)
            }
        }
    }

    pub(crate) fn rollback_validated_save_journal(
        &mut self,
        original_journal: &ValidatedRevisionJournal,
        expected_journal: &ValidatedRevisionJournal,
        expected_design_digest: crate::product::ContentDigest,
        original_dirty: bool,
    ) -> Result<(), String> {
        let changed = self.design.rollback_validated_save_journal(
            original_journal,
            expected_journal,
            expected_design_digest,
        )?;
        self.is_dirty = original_dirty || changed;
        Ok(())
    }
}
