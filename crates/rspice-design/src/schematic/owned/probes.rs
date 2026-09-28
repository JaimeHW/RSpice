//! Probe flags are durable document edits; editor selection remains above them.
use super::super::probe::SchematicProbe;
use super::{DocumentEdit, Schematic};
use rspice_app_types::product::{SavedOutputId, SimulationPlanId};
use rspice_design_model::Point;
impl Schematic {
    pub fn bind_probe_saved_output(
        &mut self,
        existing_id: u64,
        binding: Option<(SimulationPlanId, SavedOutputId)>,
    ) -> DocumentEdit<bool> {
        self.begin_operation("bind schematic probe output");
        let mut mutated = false;
        if let Some(probe) = self
            .document
            .probes
            .iter_mut()
            .find(|probe| probe.id == existing_id)
            && let Some((plan_id, output_id)) = binding
        {
            probe.bind_saved_output(plan_id, output_id);
            mutated = true;
        }
        DocumentEdit {
            value: mutated,
            committed: self.end_operation(),
        }
    }
    pub fn place_schematic_probe(
        &mut self,
        position: Point,
        label: Option<String>,
        source_expression: Option<String>,
        binding: Option<(SimulationPlanId, SavedOutputId)>,
    ) -> DocumentEdit<Option<u64>> {
        self.begin_operation("place schematic probe");
        let id = self.allocate_id();
        let reference = label.clone().unwrap_or_else(|| format!("P{id}"));
        let mut probe_id = None;
        if let Ok(mut probe) =
            SchematicProbe::new(id, position, reference, source_expression.clone())
        {
            if let Some((plan_id, output_id)) = binding {
                probe.bind_saved_output(plan_id, output_id);
            }
            self.document.probes.push(probe);
            probe_id = Some(id);
        }
        DocumentEdit {
            value: probe_id,
            committed: self.end_operation(),
        }
    }
    pub fn edit_probe_intent(
        &mut self,
        id: u64,
        enabled: bool,
        plot_on_materialization: bool,
    ) -> DocumentEdit<bool> {
        self.begin_operation("edit schematic probe");
        let mut mutated = false;
        if let Some(live) = self.document.probes.iter_mut().find(|probe| probe.id == id) {
            live.enabled = enabled;
            live.plot_on_materialization = plot_on_materialization;
            mutated = true;
        }
        DocumentEdit {
            value: mutated,
            committed: self.end_operation(),
        }
    }
}
