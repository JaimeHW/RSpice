//! Emit the resolved project hierarchy into the deck.

use super::*;

impl<'a> NetlistGenerator<'a> {
    /// Emit one `.SUBCKT` definition per master this schematic (transitively)
    /// instantiates. Runs after includes and before instances, so a definition
    /// precedes its first use, and it publishes the master index the instance
    /// pass then names its X-lines from.
    pub(super) fn generate_subcircuit_definitions(&mut self) {
        let Some(hierarchy) = self.hierarchy else {
            return;
        };
        let index = std::rc::Rc::new(MasterIndex::build(
            hierarchy,
            self.schematic,
            &self.hierarchy_path,
        ));
        self.record_defects(index.defects().to_vec());
        self.emission_map
            .extend(index.emission_map().iter().cloned());
        let emission = MasterIndex::emit(&index, hierarchy, self.source_data);
        self.masters = Some(index);

        self.errors.extend(emission.errors);
        self.warnings.extend(emission.warnings);
        self.record_defects(emission.defects);
        if !emission.blocks.is_empty() {
            self.lines.push("* Cell definitions".to_owned());
            self.lines.extend(emission.blocks);
            self.lines.push(String::new());
        }
    }

    /// Retain each typed defect and its rendering. The string half is what
    /// every existing consumer reads; the typed half is what a repair action
    /// can be attached to.
    fn record_defects(&mut self, defects: Vec<NetlistDefect>) {
        for defect in defects {
            self.errors.push(defect.to_string());
            self.defects.push(defect);
        }
    }
}
