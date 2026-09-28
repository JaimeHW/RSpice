//! Probe edits with the same document-local history and selection semantics.

use crate::product::{SavedOutputId, SimulationPlanId};
use crate::state::{Point, SchematicProbe, SchematicState};

impl SchematicState {
    pub(crate) fn bind_probe_saved_output(
        &mut self,
        existing_id: u64,
        binding: Option<(SimulationPlanId, SavedOutputId)>,
    ) {
        self.with_undo("bind schematic probe output", |schematic| {
            if let Some(probe) = schematic
                .document
                .probes
                .iter_mut()
                .find(|probe| probe.id == existing_id)
                && let Some((plan_id, output_id)) = binding
            {
                probe.bind_saved_output(plan_id, output_id);
                schematic.is_dirty = true;
            }
        });
    }

    pub(crate) fn place_schematic_probe(
        &mut self,
        position: Point,
        label: Option<String>,
        source_expression: Option<String>,
        binding: Option<(SimulationPlanId, SavedOutputId)>,
    ) -> Result<u64, String> {
        let mut probe_id = 0;
        let changed = self.with_undo("place schematic probe", |schematic| {
            let id = schematic.next_id();
            let reference = label.clone().unwrap_or_else(|| format!("P{id}"));
            if let Ok(mut probe) =
                SchematicProbe::new(id, position, reference, source_expression.clone())
            {
                if let Some((plan_id, output_id)) = binding {
                    probe.bind_saved_output(plan_id, output_id);
                }
                schematic.document.probes.push(probe);
                schematic.selection.select_only_probe(id);
                schematic.is_dirty = true;
                probe_id = id;
            }
        });
        if !changed || probe_id == 0 {
            return Err("the probe marker did not change the active schematic".to_owned());
        }
        Ok(probe_id)
    }

    pub(crate) fn edit_probe_intent(
        &mut self,
        id: u64,
        enabled: bool,
        plot_on_materialization: bool,
    ) -> bool {
        self.with_undo("edit schematic probe", |schematic| {
            if let Some(live) = schematic
                .document
                .probes
                .iter_mut()
                .find(|probe| probe.id == id)
            {
                live.enabled = enabled;
                live.plot_on_materialization = plot_on_materialization;
                schematic.is_dirty = true;
            }
        })
    }
}
