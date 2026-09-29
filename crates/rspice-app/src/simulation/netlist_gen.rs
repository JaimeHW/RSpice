//! Integration of extracted netlisting with editor, workspace and host services.

use crate::state::{
    Component, ComponentType, InstancePath, LibraryCellInstance, Point, SchematicState,
};
use rspice_design::connectivity::summary::projection_nets;
use rspice_design::hierarchy::{ConfigurationExecutionPlan, HierarchySource};
use rspice_design::schematic::document::SchematicDocument;
use rspice_simulation::netlist_gen::{
    NetlistDefect, NetlistGenerator, NetlistResult, NetlistSourceData,
    generate_netlist_hierarchical,
};
use std::collections::HashMap;

mod extraction;
#[path = "netlist_gen/master_index/tests.rs"]
mod master_index;
mod source_tables;
#[path = "netlist_gen/subcircuits/tests.rs"]
mod subcircuits;

/// Flat generation for application integration fixtures through the real file host.
#[cfg(test)]
pub(crate) fn generate_netlist(schematic: &impl AsRef<SchematicDocument>) -> NetlistResult {
    NetlistGenerator::new(
        schematic,
        NetlistSourceData::new(&crate::simulation::table_route::SourceFiles),
    )
    .finish(&[], &[])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn copied_structural_references_reach_emitted_netlist() {
        let mut state = SchematicState::default();
        for (index, kind) in [
            ComponentType::Inductor,
            ComponentType::Inductor,
            ComponentType::CoupledInductor,
            ComponentType::VoltageSource,
            ComponentType::Cccs,
        ]
        .into_iter()
        .enumerate()
        {
            let id = state.add_component(kind, Point::new(index as i32 * 100, 0));
            state.session.selection.select_component(id);
        }
        state.document_mut_for_test().components[0].name = "coil".to_owned();
        state.document_mut_for_test().components[2].params = "inductors=\"Lcoil L2\"".to_owned();
        state.document_mut_for_test().components[2].value = "0.9".to_owned();
        state.document_mut_for_test().components[3].name = "bias".to_owned();
        state.document_mut_for_test().components[4].params = "vref=Vbias".to_owned();
        state.copy_selection();
        assert!(state.paste_at_checked(Point::new(0, 1000)).unwrap());
        let netlist = generate_netlist(&state).netlist;
        for expected in ["K1 Lcoil L2 0.9", "K2 L3 L4 0.9"] {
            assert!(netlist.lines().any(|line| line == expected), "{netlist}");
        }
        let controlled = netlist
            .lines()
            .find(|line| line.starts_with("F2 "))
            .unwrap();
        assert_eq!(
            controlled.split_whitespace().nth(3),
            Some("V2"),
            "{controlled}"
        );
    }
}
