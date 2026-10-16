//! Waivers are document edits: each is recorded or removed as one undoable
//! step of the cellview it is stored with.
use super::{DocumentEdit, Schematic};
use crate::drc::{DrcWaiver, WaiverEditRefusal};

impl Schematic {
    /// Record `waiver` on this document, as one undoable edit.
    ///
    /// Refused before an operation opens, so a refused waiver leaves no step
    /// behind.
    pub fn waive(&mut self, waiver: DrcWaiver) -> Result<DocumentEdit<()>, WaiverEditRefusal> {
        let mut waivers = self.document.waivers.clone();
        waivers.waive(waiver)?;
        self.begin_operation("waive design-rule finding");
        self.document.waivers = waivers;
        Ok(DocumentEdit {
            value: (),
            committed: self.end_operation(),
        })
    }

    /// Remove the waiver stored for `rule` on `object`, as one undoable
    /// edit. The value says whether there was one.
    pub fn remove_waiver(
        &mut self,
        rule: &str,
        object: &str,
    ) -> Result<DocumentEdit<bool>, WaiverEditRefusal> {
        let mut waivers = self.document.waivers.clone();
        let removed = waivers.remove(rule, object)?.is_some();
        if !removed {
            return Ok(DocumentEdit {
                value: false,
                committed: false,
            });
        }
        self.begin_operation("remove design-rule waiver");
        self.document.waivers = waivers;
        Ok(DocumentEdit {
            value: true,
            committed: self.end_operation(),
        })
    }
}
