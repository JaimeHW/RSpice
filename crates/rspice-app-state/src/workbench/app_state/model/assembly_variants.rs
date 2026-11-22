//! The assembly variant the design is seen as, and what is authored in it.
//!
//! A variant is chosen in the document bar and its parts are changed in the
//! inspector. Both are reviewed design-management transactions, one undo step
//! each, so an undo puts a part's override and the chosen variant back
//! together. Every change here is to what a run reads, so the netlist is
//! generated afresh after it.

use super::Model;
use crate::product::ObjectRevision;
use crate::state::{
    AssemblyVariant, AssemblyVariantId, DesignManagementCatalog, SchematicObjectKey,
    VariantObjectOverride,
};
use crate::workbench::app_state::design_history::DesignManagementHistoryEntry;
use std::collections::BTreeMap;

/// What the chooser and the status line call the design with no variant.
pub const BASE_DESIGN: &str = "Base design";

impl Model {
    /// The variant the design is netlisted, drawn and inspected as; `None`
    /// is the base design.
    pub fn chosen_assembly_variant(&self) -> Option<&AssemblyVariant> {
        self.workspace.content.design_management.variants().active()
    }

    /// Make `id` the variant the design is seen as, or with `None` the base
    /// design, answering what the status bar says of it.
    ///
    /// Choosing the variant already chosen is inert: it publishes nothing,
    /// so a chooser redrawn every frame cannot advance the project revision.
    pub fn choose_assembly_variant(
        &mut self,
        id: Option<AssemblyVariantId>,
    ) -> Result<String, String> {
        let variants = self.workspace.content.design_management.variants();
        let name = match id {
            Some(id) => variants
                .find(id)
                .map(|variant| variant.name().to_owned())
                .ok_or_else(|| "That variant is no longer part of the project.".to_owned())?,
            None => BASE_DESIGN.to_owned(),
        };
        if variants.active_variant_id() == id {
            return Ok(format!("{name} is already shown."));
        }
        let before = (*self.workspace.content.design_management).clone();
        let mut candidate = before.clone();
        candidate
            .variants_mut()
            .choose(id)
            .map_err(|error| error.to_string())?;
        self.publish_design_management_candidate(&format!("Choose {name}"), before, candidate)?;
        Ok(format!("Showing {name}."))
    }

    /// Change what the chosen variant does to some parts of the open cell
    /// view, as one undo step called `description`.
    ///
    /// Each edit names a part and its new override; `None` removes the
    /// variant's own override, so the part reads as its parent variant, or
    /// the base design, has it.
    pub fn set_variant_overrides(
        &mut self,
        description: &str,
        edits: BTreeMap<SchematicObjectKey, Option<VariantObjectOverride>>,
    ) -> Result<(), String> {
        if self.schematic_edit_read_only() {
            return Err("The schematic is read-only, so its variants stay as they are.".to_owned());
        }
        let variant = self
            .chosen_assembly_variant()
            .ok_or_else(|| "Choose a variant in the document bar first.".to_owned())?;
        let (id, revision) = (variant.id(), variant.revision());
        let before = (*self.workspace.content.design_management).clone();
        let mut candidate = before.clone();
        candidate
            .variants_mut()
            .set_overrides(id, revision, edits)
            .map_err(|error| error.to_string())?;
        self.publish_design_management_candidate(description, before, candidate)?;
        Ok(())
    }

    /// Publish `candidate` as the project's design management, recorded as
    /// one undo step called `description` that restores `before`.
    ///
    /// This is the one route a reviewed design-management change takes: its
    /// renames reach the drawings in the same step, and a run prepared from
    /// the old catalog is not reused.
    pub fn publish_design_management_candidate(
        &mut self,
        description: &str,
        before: DesignManagementCatalog,
        candidate: DesignManagementCatalog,
    ) -> Result<ObjectRevision, String> {
        candidate.validate().map_err(|error| error.to_string())?;
        let schematic_tx = self.prepare_design_management_schematic_transaction(&candidate)?;
        let owner = self.workspace.content.active_schematic_reference();
        let committed_revision = self
            .workspace
            .content
            .replace_design_management(candidate)
            .map_err(|error| error.to_string())?;
        self.apply_design_management_schematic_transaction(&schematic_tx);
        let after = (*self.workspace.content.design_management).clone();
        self.record_design_management_transaction(DesignManagementHistoryEntry {
            description: description.to_owned(),
            owner,
            before,
            after,
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            references: schematic_tx.references,
            committed_revision,
        });
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.netlist_artifacts.current_generation_input_digest = None;
        Ok(committed_revision)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::pdk_config::PdkConfig;
    use crate::state::{
        AssemblyVariantDraft, ComponentType, Point, VariantInheritance, VariantQualificationPlan,
    };
    use crate::workbench::app_state::design_history::HistoryEffects;

    fn model() -> Model {
        Model::session_start(
            PdkConfig::default(),
            None,
            crate::workbench::workspace::restore_model_library_manager,
        )
    }

    /// A resistor on the open schematic, and its key in a variant.
    fn resistor(model: &mut Model) -> SchematicObjectKey {
        let id = model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        SchematicObjectKey::new(
            &model.workspace.content.active_schematic_reference().key(),
            id,
        )
        .unwrap()
    }

    /// Variants `names`, each derived from the one before, published as
    /// the project already has them; the first is chosen.
    fn variants(model: &mut Model, names: &[&str]) -> Vec<AssemblyVariantId> {
        let mut catalog = (*model.workspace.content.design_management).clone();
        let mut ids: Vec<AssemblyVariantId> = Vec::new();
        for name in names {
            let id = catalog
                .variants_mut()
                .create(AssemblyVariantDraft {
                    name: (*name).to_owned(),
                    parent_id: ids.last().copied(),
                    inheritance: VariantInheritance::OverrideChangedObjectsOnly,
                    qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
                    overrides: BTreeMap::new(),
                })
                .unwrap();
            ids.push(id);
        }
        model
            .workspace
            .content
            .replace_design_management(catalog)
            .unwrap();
        ids
    }

    fn undo(model: &mut Model) {
        let staged = model.stage_project_undo().unwrap().expect("a step to undo");
        let mut effects = HistoryEffects::default();
        let applied = model
            .apply_project_step(staged, None, &mut effects)
            .unwrap();
        model.finish_project_step(applied).unwrap();
    }

    fn chosen(model: &Model) -> Option<String> {
        model
            .chosen_assembly_variant()
            .map(|variant| variant.name().to_owned())
    }

    fn own(
        model: &Model,
        variant: AssemblyVariantId,
        key: &SchematicObjectKey,
    ) -> Option<VariantObjectOverride> {
        model
            .workspace
            .content
            .design_management
            .variants()
            .find(variant)
            .unwrap()
            .definition()
            .overrides
            .get(key)
            .cloned()
    }

    fn do_not_populate() -> VariantObjectOverride {
        VariantObjectOverride::DoNotPopulate {
            approval_reference: String::new(),
        }
    }

    #[test]
    fn undo_puts_back_a_parts_override_and_the_chosen_variant_together() {
        let mut model = model();
        let key = resistor(&mut model);
        let ids = variants(&mut model, &["Industrial", "Automotive"]);

        model.choose_assembly_variant(Some(ids[1])).unwrap();
        model
            .set_variant_overrides(
                "Mark R1 do not populate in Automotive",
                BTreeMap::from([(key.clone(), Some(do_not_populate()))]),
            )
            .unwrap();
        assert_eq!(
            model.choose_assembly_variant(None).unwrap(),
            "Showing Base design."
        );
        assert_eq!(chosen(&model), None, "the base design can be chosen");

        undo(&mut model);
        assert_eq!(chosen(&model).as_deref(), Some("Automotive"));
        assert_eq!(own(&model, ids[1], &key), Some(do_not_populate()));
        undo(&mut model);
        assert_eq!(
            own(&model, ids[1], &key),
            None,
            "one step removes the override"
        );
        assert_eq!(chosen(&model).as_deref(), Some("Automotive"));
        undo(&mut model);
        assert_eq!(chosen(&model).as_deref(), Some("Industrial"));
    }

    #[test]
    fn a_parent_edit_and_the_repinning_of_its_children_are_one_undo_step() {
        let mut model = model();
        let key = resistor(&mut model);
        let ids = variants(&mut model, &["Industrial", "Automotive"]);
        let before = (*model.workspace.content.design_management).clone();

        model
            .set_variant_overrides(
                "Mark R1 do not populate in Industrial",
                BTreeMap::from([(key.clone(), Some(do_not_populate()))]),
            )
            .expect("a variant with a child can be edited");
        let catalog = model.workspace.content.design_management.variants();
        let parent = catalog.find(ids[0]).unwrap();
        let pin = catalog
            .find(ids[1])
            .unwrap()
            .definition()
            .parent
            .clone()
            .unwrap();
        assert_eq!(
            pin.revision,
            parent.revision(),
            "the child follows its parent"
        );
        assert_eq!(
            catalog.resolve(ids[1]).unwrap().overrides.get(&key),
            Some(&do_not_populate()),
            "the child inherits the parent's change"
        );

        undo(&mut model);
        assert_eq!(
            model.workspace.content.design_management.variants(),
            before.variants(),
            "one undo puts the parent and its child back"
        );
    }

    #[test]
    fn choosing_the_variant_already_chosen_publishes_nothing() {
        let mut model = model();
        let ids = variants(&mut model, &["Industrial"]);
        let revision = model.workspace.content.project.revision();
        model.choose_assembly_variant(Some(ids[0])).unwrap();
        assert_eq!(model.workspace.content.project.revision(), revision);
        assert!(!model.can_undo_project_design());
    }
}
