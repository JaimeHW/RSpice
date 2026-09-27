//! Replacing an instance in place.
//!
//! Swapping the master behind a placed instance while preserving what the
//! designer set: parameter overrides that the new master also accepts, and
//! wiring to pins that survive the swap. Refuses the replacement outright
//! when the new master cannot carry the existing connections.

use super::super::*;
use rspice_design::schematic::replacement::*;
use rspice_design::schematic::replacement_edit::{self, InstanceReplacement, ReplacementContext};

impl SchematicState {
    /// Capture immutable authority for the exact selected component using its
    /// built-in or generated symbol contract.
    pub fn replacement_authority(
        &self,
    ) -> Result<SchematicReplacementAuthority, SchematicReplacementError> {
        let component_id = self
            .selection
            .single_component()
            .ok_or(SchematicReplacementError::SelectExactlyOneInstance)?;
        let component = self
            .document
            .components
            .iter()
            .find(|component| component.id == component_id)
            .ok_or(SchematicReplacementError::SourceInstanceMissing { component_id })?;
        self.replacement_authority_with_spec(SchematicReplacementSourceSpec::from_component(
            component,
        )?)
    }

    fn replacement_context(&self) -> ReplacementContext {
        ReplacementContext {
            selected_component: self.selection.single_component(),
            topology_version: self.topology_version(),
        }
    }

    /// Capture validated source evidence without retaining a library manager.
    pub fn replacement_authority_with_spec(
        &self,
        source_spec: SchematicReplacementSourceSpec,
    ) -> Result<SchematicReplacementAuthority, SchematicReplacementError> {
        if self.read_only {
            return Err(SchematicReplacementError::ReadOnly);
        }
        replacement_edit::replacement_authority_with_spec(
            &self.document,
            self.replacement_context(),
            source_spec,
        )
    }

    /// Analyze the exact candidate against current selection and topology.
    pub fn preview_instance_replacement(
        &self,
        authority: &SchematicReplacementAuthority,
        target: &SchematicReplacementTargetSpec,
    ) -> Result<SchematicReplacementPreview, SchematicReplacementError> {
        if self.read_only {
            return Err(SchematicReplacementError::ReadOnly);
        }
        replacement_edit::preview_instance_replacement(
            &self.document,
            self.replacement_context(),
            authority,
            target,
        )
    }

    /// Validate before opening history, then commit to the same borrowed document.
    pub fn replace_selected_instance(
        &mut self,
        authority: &SchematicReplacementAuthority,
        target: &SchematicReplacementTargetSpec,
    ) -> Result<SchematicReplacementImpact, SchematicReplacementError> {
        if self.read_only {
            return Err(SchematicReplacementError::ReadOnly);
        }
        let context = self.replacement_context();
        let (document, identity, _, mut edit) = self.document_edit_parts();
        let replacement = InstanceReplacement::prepare(document, context, authority, target)?;
        let impact = replacement.impact();
        let reference_counter = replacement.reference_counter();
        edit.begin(replacement.document(), "replace instance");
        replacement.commit();
        edit.selection.select_only_component(impact.component_id);
        if let Some((prefix, number)) = reference_counter {
            identity.record_component_number(prefix, number);
        }
        edit.mark_topology_changed();
        if !edit.end(document) {
            return Err(SchematicReplacementError::CommitFailed);
        }
        Ok(impact)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::PortDirection;
    use crate::state::{
        LibraryCellInstance, PortSpec, SchematicReplacementParameter, SchematicSnapshot, Wire,
    };

    fn five_pin_binding(cell: &str) -> LibraryCellInstance {
        let mut binding = LibraryCellInstance::new("analog", cell, "symbol");
        binding.terminal_order = ["IN+", "IN-", "V+", "V-", "OUT"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        binding.terminal_dirs = vec![
            PortDirection::In,
            PortDirection::In,
            PortDirection::Supply,
            PortDirection::Supply,
            PortDirection::Out,
        ];
        binding.module_name = Some(cell.to_owned());
        binding
    }

    fn selected_opamp() -> SchematicState {
        let binding = five_pin_binding("OPA189");
        let mut state = SchematicState::default();
        let id = state.add_library_cell_component(Point::new(100, 100), binding);
        let component = state
            .document
            .components
            .iter_mut()
            .find(|item| item.id == id)
            .unwrap();
        component.name = "U1".to_owned();
        component.value = "OPA189".to_owned();
        component.params = "gain=100 ibias=2n vos=1u slew=20Meg en=5n temp=27".to_owned();
        state.selection.select_only_component(id);
        state.recalculate_runtime_state();
        state.clear_undo_history();
        state
    }

    fn opa188_target() -> SchematicReplacementTargetSpec {
        SchematicReplacementTargetSpec::library_cell(five_pin_binding("OPA188"))
            .with_value("OPA188")
            .with_parameters(vec![
                SchematicReplacementParameter::new("gain"),
                SchematicReplacementParameter::new("ibias"),
                SchematicReplacementParameter::new("vos"),
                SchematicReplacementParameter::new("slew"),
                SchematicReplacementParameter::new("en"),
                SchematicReplacementParameter::new("temperature").with_aliases(["temp"]),
                SchematicReplacementParameter::new("gbw").with_default("18Meg"),
                SchematicReplacementParameter::new("vnoise").with_default("5.2n"),
            ])
    }

    #[test]
    fn mockup_opamp_contract_reports_five_pins_and_six_of_eight_parameters() {
        let state = selected_opamp();
        let source = state.document.components[0].clone();
        let source_spec = SchematicReplacementSourceSpec::from_component(&source)
            .unwrap()
            .with_parameter_keys(["gain", "ibias", "vos", "slew", "en", "temp"]);
        let authority = state.replacement_authority_with_spec(source_spec).unwrap();
        let preview = state
            .preview_instance_replacement(&authority, &opa188_target())
            .unwrap();

        assert_eq!(preview.compatibility.mapped_terminal_count, 5);
        assert_eq!(preview.compatibility.target_terminal_count, 5);
        assert_eq!(preview.compatibility.mapped_parameter_count, 6);
        assert_eq!(preview.compatibility.target_parameter_count, 8);
        assert_eq!(preview.component.id, source.id);
        assert_eq!(preview.component.name, "U1");
        assert_eq!(preview.component.pos, source.pos);
        assert_eq!(preview.component.rotation, source.rotation);
        assert_eq!(preview.component.mirror_h, source.mirror_h);
        assert_eq!(preview.component.mirror_v, source.mirror_v);
        assert!(preview.component.params.contains("temperature=27"));
        assert!(preview.component.params.contains("gbw=18Meg"));
    }

    #[test]
    fn commit_is_one_undo_step_and_preserves_clipboard_identity_and_placement() {
        let mut state = selected_opamp();
        let authority = state.replacement_authority().unwrap();
        state
            .clipboard
            .components
            .push(state.document.components[0].clone());
        let clipboard = serde_json::to_value(&state.clipboard).unwrap();
        let before = state.document.components[0].clone();
        let topology = state.topology_version();

        let impact = state
            .replace_selected_instance(&authority, &opa188_target())
            .unwrap();

        let after = &state.document.components[0];
        assert_eq!(after.id, before.id);
        assert_eq!(after.name, before.name);
        assert_eq!(after.pos, before.pos);
        assert_eq!(serde_json::to_value(&state.clipboard).unwrap(), clipboard);
        assert_eq!(impact.component_id, before.id);
        assert_eq!(state.topology_version(), topology.wrapping_add(1));
        assert_eq!(state.undo_description(), Some("replace instance"));
        assert!(state.undo());
        assert_eq!(state.document.components[0], before);
        assert!(
            !state.can_undo(),
            "replacement must create exactly one undo record"
        );
    }

    #[test]
    fn cross_prefix_replacement_preserves_id_and_assigns_a_unique_reference() {
        let mut state = SchematicState::default();
        let resistor_id = state.add_component(ComponentType::Resistor, Point::origin());
        state.add_component(ComponentType::Capacitor, Point::new(100, 0));
        state.selection.select_only_component(resistor_id);
        state.clear_undo_history();
        let authority = state.replacement_authority().unwrap();

        let preview = state
            .preview_instance_replacement(
                &authority,
                &SchematicReplacementTargetSpec::primitive(ComponentType::Capacitor),
            )
            .unwrap();
        assert_eq!(preview.component.id, resistor_id);
        assert_eq!(preview.component.name, "C2");
        state
            .replace_selected_instance(
                &authority,
                &SchematicReplacementTargetSpec::primitive(ComponentType::Capacitor),
            )
            .unwrap();
        assert_eq!(state.document.components[0].id, resistor_id);
        assert_eq!(state.document.components[0].name, "C2");
        let next = state.add_component(ComponentType::Capacitor, Point::new(200, 0));
        assert_eq!(
            state
                .document
                .components
                .iter()
                .find(|component| component.id == next)
                .unwrap()
                .name,
            "C3"
        );
        assert!(state.undo());
        assert_eq!(state.document.components[0].name, "R1");
    }

    #[test]
    fn connected_terminal_alias_retargets_connection_and_wire_endpoint() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(100, 100));
        state.selection.select_only_component(id);
        let terminal = state.document.components[0]
            .terminal_positions()
            .into_iter()
            .find(|(name, _)| *name == "+")
            .unwrap()
            .1;
        let wire_id = state.next_id();
        state
            .document
            .wires
            .push(Wire::segment(wire_id, terminal, Point::new(20, 100)));
        state
            .document
            .connections
            .push(WireConnection::new(wire_id, 0, id, "+"));
        state.recalculate_runtime_state();
        state.clear_undo_history();
        let authority = state.replacement_authority().unwrap();
        let mut binding = LibraryCellInstance::new("analog", "R_ALIAS", "symbol");
        binding.terminal_order = vec!["P".to_owned(), "N".to_owned()];
        binding.terminal_dirs = vec![PortDirection::InOut, PortDirection::InOut];
        let target = SchematicReplacementTargetSpec::library_cell(binding)
            .with_terminals(vec![
                SchematicReplacementTerminal::new("P", Point::new(-30, 0)).with_aliases(["+"]),
                SchematicReplacementTerminal::new("N", Point::new(30, 0)).with_aliases(["-"]),
            ])
            .with_value("R_ALIAS");

        let preview = state
            .preview_instance_replacement(&authority, &target)
            .unwrap();
        assert_eq!(
            preview.wire_edits[0].original_points,
            [Point::new(80, 100), Point::new(20, 100)]
        );
        assert_eq!(
            preview.wire_edits[0].replacement_points,
            [Point::new(70, 100), Point::new(20, 100)]
        );
        assert_eq!(preview.connections[0].terminal_name, "P");
        state
            .replace_selected_instance(&authority, &target)
            .unwrap();
        assert_eq!(state.document.wires[0].points[0], Point::new(70, 100));
        assert_eq!(state.document.connections[0].terminal_name, "P");
    }

    #[test]
    fn diagonal_pin_displacement_inserts_an_orthogonal_bend_and_remaps_connections() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(100, 100));
        state.selection.select_only_component(id);
        let terminal = state.document.components[0].terminal_positions()[0].1;
        let wire_id = state.next_id();
        state
            .document
            .wires
            .push(Wire::segment(wire_id, terminal, Point::new(20, 100)));
        state
            .document
            .connections
            .push(WireConnection::new(wire_id, 0, id, "+"));
        state.recalculate_runtime_state();
        state.clear_undo_history();
        let authority = state.replacement_authority().unwrap();
        let mut binding = LibraryCellInstance::new("analog", "R_OFFSET", "symbol");
        binding.terminal_order = vec!["P".to_owned(), "N".to_owned()];
        binding.terminal_dirs = vec![PortDirection::InOut, PortDirection::InOut];
        let target = SchematicReplacementTargetSpec::library_cell(binding)
            .with_terminals(vec![
                SchematicReplacementTerminal::new("P", Point::new(-30, -10)).with_aliases(["+"]),
                SchematicReplacementTerminal::new("N", Point::new(30, 0)).with_aliases(["-"]),
            ])
            .with_value("R_OFFSET");

        let preview = state
            .preview_instance_replacement(&authority, &target)
            .unwrap();
        assert_eq!(preview.impact.relocated_wire_points, 1);
        assert!(preview.wire_edits[0].replacement_points.len() >= 3);
        state
            .replace_selected_instance(&authority, &target)
            .unwrap();
        assert!(state.document.wires[0].is_orthogonal());
        assert_eq!(
            state.document.wires[0].points[state.document.connections[0].point_index],
            Point::new(70, 90)
        );
        assert!(state.undo());
        assert_eq!(
            state.document.wires[0].points,
            [terminal, Point::new(20, 100)]
        );
        assert_eq!(state.document.connections[0].point_index, 0);
    }

    #[test]
    fn stale_authority_and_unmapped_connected_pin_are_non_mutating() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(100, 100));
        state.selection.select_only_component(id);
        let authority = state.replacement_authority().unwrap();
        let before = SchematicSnapshot::capture(&state.document);
        state.document.components[0].value = "2k".to_owned();
        assert_eq!(
            state.preview_instance_replacement(
                &authority,
                &SchematicReplacementTargetSpec::primitive(ComponentType::Capacitor)
            ),
            Err(SchematicReplacementError::StaleAuthority)
        );
        state.document.components[0] = authority.source_component().clone();
        assert!(before.is_equal_document(&state.document));

        let terminal = state.document.components[0].terminal_positions()[0].1;
        let wire_id = state.next_id();
        state
            .document
            .wires
            .push(Wire::segment(wire_id, terminal, Point::new(0, 100)));
        state
            .document
            .connections
            .push(WireConnection::new(wire_id, 0, id, "+"));
        state.bump_topology_version();
        let authority = state.replacement_authority().unwrap();
        let mut binding = LibraryCellInstance::new("work", "one_pin", "schematic");
        binding.bind_interface(&[PortSpec {
            name: "only".to_owned(),
            direction: PortDirection::InOut,
        }]);
        let target = SchematicReplacementTargetSpec::library_cell(binding).with_terminals(vec![
            SchematicReplacementTerminal::new("only", Point::new(20, 0)),
        ]);
        let snapshot = SchematicSnapshot::capture(&state.document);
        assert_eq!(
            state.preview_instance_replacement(&authority, &target),
            Err(SchematicReplacementError::UnmappedConnectedTerminal {
                terminal: "+".to_owned()
            })
        );
        assert!(snapshot.is_equal_document(&state.document));
        assert!(!state.can_undo());
    }

    #[test]
    fn invalid_parameter_and_coordinate_contracts_fail_without_panicking() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(i32::MAX, 0));
        state.selection.select_only_component(id);
        assert_eq!(
            state.replacement_authority(),
            Err(SchematicReplacementError::CoordinateOverflow)
        );

        state.document.components[0].pos = Point::origin();
        state.document.components[0].params = "gain".to_owned();
        assert!(matches!(
            state.replacement_authority(),
            Err(SchematicReplacementError::MalformedParameterString { .. })
        ));
    }

    #[test]
    fn required_target_parameters_need_a_mapping_or_lossless_default() {
        let state = selected_opamp();
        let authority = state.replacement_authority().unwrap();
        let missing =
            SchematicReplacementTargetSpec::library_cell(five_pin_binding("OPA_REQUIRED"))
                .with_value("OPA_REQUIRED")
                .with_parameters(vec![
                    SchematicReplacementParameter::new("corner").required(),
                ]);
        assert_eq!(
            state.preview_instance_replacement(&authority, &missing),
            Err(SchematicReplacementError::MissingRequiredParameter {
                parameter: "corner".to_owned()
            })
        );

        let defaulted =
            SchematicReplacementTargetSpec::library_cell(five_pin_binding("OPA_DEFAULT"))
                .with_value("OPA_DEFAULT")
                .with_parameters(vec![
                    SchematicReplacementParameter::new("corner")
                        .required()
                        .with_default(r#""tt slow""#),
                ]);
        let preview = state
            .preview_instance_replacement(&authority, &defaulted)
            .unwrap();
        assert!(preview.component.params.contains(r#"corner="tt slow""#));
    }
}
