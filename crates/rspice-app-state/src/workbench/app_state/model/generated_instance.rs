//! Where the selected schematic instance sits in the retained generated
//! deck, read from the model alone.

use crate::state::{GeneratedSourceMapEntry, NetlistDocument};

use super::Model;

/// Where one selected schematic instance appears in the retained generated
/// deck, and whether that deck still matches the project it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedInstanceLocation {
    /// One-based generated line carrying the instance's own card.
    pub line: usize,
    /// Emitted SPICE identity written on that line.
    pub instance: String,
    /// Whether the retained artifact predates the current project input.
    pub stale: bool,
}

impl Model {
    /// Locate the selected schematic instance in the retained generated deck.
    ///
    /// The error is the exact reason the jump is unavailable, worded for the
    /// disabled control that reports it, so availability and execution can
    /// never disagree about why. A stale artifact still resolves — it is what
    /// the project last generated — and the caller says so rather than
    /// refusing.
    pub fn selected_instance_generated_line(
        &self,
    ) -> Result<GeneratedInstanceLocation, &'static str> {
        const NO_DECK: &str =
            "no generated netlist yet — open the Netlist workspace to generate one";

        let component_id = self
            .schematic
            .session
            .editor
            .selection
            .single_component()
            .ok_or("select one instance")?;
        if self.netlist_artifacts.generated_source.trim().is_empty() {
            return Err(NO_DECK);
        }
        let artifact = self
            .netlist_artifacts
            .generated_document
            .as_ref()
            .and_then(NetlistDocument::generated_artifact)
            .ok_or(NO_DECK)?;
        let identity = GeneratedSourceMapEntry::component_identity_for(
            &self.workspace.content.active_view.key(),
            component_id,
        );
        let line = artifact
            .generated_lines_for_component(&identity)
            .first()
            .copied()
            .ok_or("no netlist line for this instance")?;
        Ok(GeneratedInstanceLocation {
            line,
            instance: artifact
                .source_map_entry(line)
                .and_then(GeneratedSourceMapEntry::instance_identity)
                .unwrap_or_default()
                .to_owned(),
            stale: self.netlist_artifacts.generated_input_digest
                != self.netlist_artifacts.current_generation_input_digest,
        })
    }
}
