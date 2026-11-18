//! Where the selected schematic instance, or net, sits in the retained
//! generated deck, read from the model alone.

use crate::state::{GeneratedArtifact, GeneratedSourceMapEntry, NetlistDocument};

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

/// Where one net of the open schematic is written in the retained generated
/// deck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedNetLocation {
    /// One-based generated lines of the open cell view's own cards that
    /// connect the net, in order; never empty.
    pub lines: Vec<usize>,
    /// Whether the retained artifact predates the current project input.
    pub stale: bool,
}

/// Why the jump has nothing to locate.
const SELECT_ONE: &str = "select one instance or one net";
/// Why the jump has no netlist to locate it in.
const NO_DECK: &str = "the design's netlist could not be generated";

impl Model {
    /// Why "Show in netlist" cannot be taken for the selection, if it cannot:
    /// it takes one instance, or wires, junctions and net labels of one net.
    ///
    /// Only the selection decides. The jump generates the netlist before it
    /// locates, so a netlist that was never generated, or one older than the
    /// design, is no reason to refuse it — and a line looked up in an older
    /// one would refuse an instance placed since. Whether the instance has a
    /// line at all is the locator's answer, given when the jump is taken.
    #[must_use]
    pub fn show_in_netlist_block(&self) -> Option<&'static str> {
        (self
            .schematic
            .session
            .editor
            .selection
            .single_component()
            .is_none()
            && self.selected_design_net().is_none())
        .then_some(SELECT_ONE)
    }

    /// Locate the selected schematic instance in the retained generated deck.
    ///
    /// The error is the exact reason the jump cannot be made, worded to follow
    /// the jump's refusal. A stale netlist still resolves — it is what the
    /// project last generated, kept when generating again failed — and the
    /// caller says so rather than refusing.
    pub fn selected_instance_generated_line(
        &self,
    ) -> Result<GeneratedInstanceLocation, &'static str> {
        let component_id = self
            .schematic
            .session
            .editor
            .selection
            .single_component()
            .ok_or(SELECT_ONE)?;
        let artifact = self.retained_generated_artifact()?;
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
            stale: self.retained_generated_artifact_is_stale(),
        })
    }

    /// Locate `net` of the open schematic in the retained generated deck:
    /// the lines of the open cell view's own cards that connect it, which
    /// for a master are the lines of its `.subckt` body.
    pub fn net_generated_lines(&self, net: &str) -> Result<GeneratedNetLocation, &'static str> {
        let artifact = self.retained_generated_artifact()?;
        let view = self.workspace.content.active_view.key();
        let lines = artifact
            .source_map()
            .iter()
            .filter(|entry| {
                entry.view_identity().eq_ignore_ascii_case(&view)
                    && entry
                        .nets()
                        .iter()
                        .any(|written| written.eq_ignore_ascii_case(net))
            })
            .map(GeneratedSourceMapEntry::generated_line)
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err("no netlist line connects this net");
        }
        Ok(GeneratedNetLocation {
            lines,
            stale: self.retained_generated_artifact_is_stale(),
        })
    }

    /// The generated deck the project last kept.
    fn retained_generated_artifact(&self) -> Result<&GeneratedArtifact, &'static str> {
        if self.netlist_artifacts.generated_source.trim().is_empty() {
            return Err(NO_DECK);
        }
        self.netlist_artifacts
            .generated_document
            .as_ref()
            .and_then(NetlistDocument::generated_artifact)
            .ok_or(NO_DECK)
    }

    /// Whether the kept deck predates the current project input.
    fn retained_generated_artifact_is_stale(&self) -> bool {
        self.netlist_artifacts.generated_input_digest
            != self.netlist_artifacts.current_generation_input_digest
    }
}
