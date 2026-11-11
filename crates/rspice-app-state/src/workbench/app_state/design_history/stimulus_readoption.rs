//! Re-adopting a stimulus definition: one instance that already adopted it
//! takes the library's current revision back, as one component transaction.

use crate::workbench::app_state::model::Model;

impl Model {
    /// Copy the library's current revision of `definition` back onto one
    /// instance that already adopted it, and say what happened.
    ///
    /// Narrower than adoption on purpose, because the model is: an instance
    /// naming some other definition is adopting, not re-adopting, and
    /// `readopt_onto` is where that refusal is written. Every surface that
    /// offers the verb — the Studio's Excitations rows, the component editor's
    /// evidence pane, the schematic's own menu — comes through here, so one
    /// re-adoption is one transaction and one console sentence wherever it is
    /// asked for.
    pub fn readopt_stimulus(
        &mut self,
        component_id: u64,
        definition: &str,
    ) -> Result<String, String> {
        let held = self
            .workspace
            .content
            .stimulus_library
            .get(definition)
            .cloned()
            .ok_or_else(|| format!("This project defines no '{definition}' to re-adopt."))?;
        let expected = self
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .cloned()
            .ok_or_else(|| "That instance is no longer on this sheet.".to_owned())?;
        let mut candidate = expected.clone();
        held.readopt_onto(&mut candidate)?;
        let reference = candidate.spice_instance_name();
        self.edit_component_transaction(&expected, candidate, "re-adopt stimulus definition")?;
        Ok(format!(
            "{reference} re-adopted {definition} r{}; its card is the library's again.",
            held.revision()
        ))
    }
}
