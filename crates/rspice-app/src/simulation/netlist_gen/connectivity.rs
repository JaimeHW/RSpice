//! The generator's view of the one connectivity extraction.
//!
//! Netlisting traces nothing of its own. It asks [`super::extraction`] what the
//! drawing connects and folds the answer into the emission state, so a terminal
//! carries the same node identity in the deck, in the net summary, and in the
//! electrical rule check.

use super::*;

impl<'a> NetlistGenerator<'a> {
    /// Resolve this schematic's connectivity and adopt it.
    ///
    /// Diagnostics arrive in the order the extraction found them and are routed
    /// by whether the deck can be emitted while they stand. A width mismatch
    /// additionally becomes a typed defect, because a repair attaches to the
    /// kind rather than to the text of an error.
    pub(super) fn extract_connectivity(&mut self) {
        let extracted = super::extraction::extract(self.schematic, self.hierarchy);
        for diagnostic in &extracted.diagnostics {
            let destination = if diagnostic.blocking {
                &mut self.errors
            } else {
                &mut self.warnings
            };
            destination.push(diagnostic.message.clone());
        }
        for mismatch in &extracted.vector_nets.mismatches {
            self.defects.push(NetlistDefect::WidthMismatch {
                owner: mismatch.owner.clone(),
                declared_width: mismatch.declared_width,
                found_width: mismatch.found_width,
                detail: mismatch.message(),
            });
        }
        self.nets = extracted.nets;
        self.point_to_net = extracted.point_to_net;
        self.ground_net = extracted.ground_net;
    }
}
