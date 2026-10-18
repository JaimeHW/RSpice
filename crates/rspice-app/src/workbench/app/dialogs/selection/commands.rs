//! Delete, Cut, Paste, Duplicate, Select all and Join collinear wires,
//! acting immediately.
//!
//! Every professional schematic editor treats these as direct commands
//! with undo as the safety net; a modal in front of Ctrl+D is a modal the
//! reader dismisses without reading. What the old review surface computed and
//! showed up front is still computed — which nets a deletion touches, which
//! saved outputs and probes still name them, which nets a cut leaves open —
//! and is reported on the status bar afterwards, as the answer to the gesture.

use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap, HashSet};

use crate::schematic::view::schematic_symbol_context;
use crate::schematic::view::sheet_visibility::{
    off_active_sheet, selectable_objects_on_active_sheet, selection_filtered_to_active_sheet,
    with_active_wire_topology,
};
use crate::state::{CellViewRef, DuplicateExternalNets, Point, SchematicState, Selection};
use rspice_design::connectivity::summary::{
    DesignNet, design_nets_with_hierarchy, projection_nets,
};
use rspice_design::hierarchy::HierarchySource;
use rspice_design::schematic::clipboard::ClipboardData;

use crate::workbench::app::dialogs::state::FloatingPasteOrigin;
use crate::workbench::app_state::AppState;

pub(in crate::workbench) mod floating;

impl AppState {
    /// Delete what is selected on the active sheet.
    ///
    /// The impact report is built before the objects are gone, because that is the
    /// only moment the design still knows which nets they were on.
    pub(crate) fn delete_schematic_selection(&mut self) -> bool {
        let state = self;
        if refuse_read_only(state) {
            return false;
        }
        let mut target =
            selection_filtered_to_active_sheet(state, &state.schematic.session.editor.selection);
        promote_wire_handles_to_complete_wires(&state.schematic, &mut target);
        let count = complete_selection_count(&state.schematic, &target);
        if count == 0 {
            refuse_empty(state);
            return false;
        }

        let impact = delete_dependency_impact(state, &target);
        let previous_selection =
            std::mem::replace(&mut state.schematic.session.editor.selection, target);
        if !delete_on_active_sheet(state) {
            state.schematic.session.editor.selection = previous_selection;
            state.prompt("Nothing was deleted.");
            return false;
        }
        state.sync_active_schematic_to_workspace();

        let headline = format!("Deleted {}.", object_count(count));
        state.prompt(match impact.detail() {
            Some(detail) => format!("{headline} {detail}"),
            None => headline,
        });
        true
    }

    /// Copy what is selected on the active sheet, then delete it.
    pub(crate) fn cut_schematic_selection(&mut self) -> bool {
        let state = self;
        if refuse_read_only(state) {
            return false;
        }
        let target =
            selection_filtered_to_active_sheet(state, &state.schematic.session.editor.selection);
        let count = complete_selection_count(&state.schematic, &target);
        if count == 0 {
            refuse_empty(state);
            return false;
        }

        let open_nets = cut_open_net_count(state, &target);
        let previous_selection =
            std::mem::replace(&mut state.schematic.session.editor.selection, target);
        let Some(copy) = state
            .capture_active_schematic_selection()
            .filter(|copy| !copy.is_empty())
        else {
            state.schematic.session.editor.selection = previous_selection;
            refuse_empty(state);
            return false;
        };
        if !delete_on_active_sheet(state) {
            state.schematic.session.editor.selection = previous_selection;
            state.prompt("Nothing was cut.");
            return false;
        }
        // The clipboard changes only once the cut has landed.
        state.ui.schematic_clipboard = copy;
        state.sync_active_schematic_to_workspace();

        state.prompt(match open_nets {
            Ok(0) | Err(_) => format!("Cut {}.", object_count(count)),
            Ok(open) => format!("Cut {}; {}.", object_count(count), open_nets_clause(open)),
        });
        true
    }

    /// Duplicate what is selected: the copy floats one grid step off the
    /// current paste anchor until a click places it.
    pub(crate) fn duplicate_schematic_selection(&mut self) -> bool {
        let state = self;
        let anchor = state.schematic_paste_anchor() + state.schematic_grid_step();
        state.duplicate_schematic_selection_at(anchor)
    }

    /// One grid step down and to the right: where a duplicate lands from the
    /// point it was asked for, so it is seen beside its source and stays on
    /// the grid.
    pub(crate) fn schematic_grid_step(&self) -> Point {
        let grid = self.schematic.document().grid_size.max(1);
        Point::new(grid, grid)
    }

    /// Duplicate what is selected: the copy floats at `anchor` and follows
    /// the pointer until a click places it.
    ///
    /// The user's clipboard is theirs: Duplicate carries its own copy and never
    /// touches it.
    pub(crate) fn duplicate_schematic_selection_at(&mut self, anchor: Point) -> bool {
        let state = self;
        if refuse_read_only(state) {
            return false;
        }
        let target =
            selection_filtered_to_active_sheet(state, &state.schematic.session.editor.selection);
        let count = complete_selection_count(&state.schematic, &target);
        if count == 0 {
            refuse_empty(state);
            return false;
        }

        let previous_selection =
            std::mem::replace(&mut state.schematic.session.editor.selection, target);
        let Some(mut copy) = state
            .capture_active_schematic_selection()
            .filter(|copy| !copy.is_empty())
        else {
            state.schematic.session.editor.selection = previous_selection;
            refuse_empty(state);
            return false;
        };

        // A design whose hierarchy does not resolve cannot name its nets, and a
        // Duplicate that refuses for that reason is a Duplicate that stops working
        // on exactly the designs a reader is trying to repair. It copies anyway and
        // says the connections were not carried.
        let mut unresolved_nets = false;
        let kept = if state.ui.duplicate_external_nets
            == DuplicateExternalNets::PreserveNamedNetAttachment
        {
            match named_external_attachments(state, &copy) {
                Ok(attachments) => copy.preserve_named_net_attachments(attachments),
                Err(_) => {
                    unresolved_nets = true;
                    0
                }
            }
        } else {
            0
        };

        let origin = FloatingPasteOrigin::Duplicate {
            count,
            kept,
            unresolved_nets,
        };
        if let Err(reason) = state.float_schematic_objects(&copy, anchor, origin) {
            state.schematic.session.editor.selection = previous_selection;
            state.prompt(reason);
            return false;
        }
        true
    }

    /// Float the app's clipboard at `anchor`, whichever cell view it was
    /// copied from, until a click places it. Returns whether anything floats.
    pub(crate) fn paste_schematic_clipboard_at(&mut self, anchor: Point) -> Result<bool, String> {
        if self.schematic_edit_read_only() || !self.ui.schematic_clipboard.has_content() {
            return Ok(false);
        }
        let clipboard = self.ui.schematic_clipboard.clone();
        self.float_schematic_objects(&clipboard, anchor, FloatingPasteOrigin::Paste)?;
        Ok(true)
    }

    /// Paste `objects` at `anchor` into the active cell view as one undo step,
    /// joined to the wiring they land on. The pasted objects become the
    /// selection.
    fn paste_schematic_objects_at(
        &mut self,
        objects: &ClipboardData,
        anchor: Point,
    ) -> Result<bool, String> {
        if self.schematic_edit_read_only() {
            return Ok(false);
        }
        let objects = bound_to_active_cell(self, objects)?;
        let symbols = schematic_symbol_context(self);
        // The undo step opens with every sheet present; the cell view's other
        // sheets, which share these coordinates, step out of reach only while
        // the pasted wires and pins are joined to this one. The paste itself
        // names its parts against every sheet and places its dots on this one.
        self.schematic.begin_operation("paste");
        let pasted =
            self.schematic
                .paste_at_checked(&objects, anchor, off_active_sheet(&self.workspace));
        if matches!(pasted, Ok(true)) {
            with_active_wire_topology(self, |schematic, scope| {
                schematic.connect_pasted_selection(
                    |component| symbols.terminal_points(component),
                    scope,
                );
            });
        }
        self.schematic.end_operation();
        if matches!(pasted, Ok(true)) {
            self.sync_active_schematic_to_workspace();
        }
        pasted
    }

    /// Select every object on the active sheet the selection filter admits.
    pub(crate) fn select_all_schematic_objects(&mut self) -> bool {
        let state = self;
        let selection = selectable_objects_on_active_sheet(state);
        let count = selection.count();
        if count == 0 {
            state.prompt("Nothing matches the selection filter.");
            return false;
        }
        state.schematic.session.editor.selection = selection;
        // Selecting changes no document, so the buffer's selection is written on
        // its own rather than through the whole save-and-revalidate path an edit
        // takes.
        let active_key = state.workspace.content.active_schematic_reference().key();
        if let Some(mut buffer) = state.workspace.schematic_editor_mut(&active_key) {
            buffer.editor.session.editor.selection =
                state.schematic.session.editor.selection.clone();
        }
        state.prompt(format!("Selected {}.", object_count(count)));
        true
    }

    /// Join Collinear Wires on the active sheet, as one undo step: every pair
    /// of wires that runs on one line, end to end or overlapping, becomes one
    /// wire, and the junction policy tidies what is left. Returns whether
    /// anything was joined.
    pub(crate) fn join_collinear_wires_on_active_sheet(&mut self) -> bool {
        if refuse_read_only(self) {
            return false;
        }
        let automatic_junctions = self.schematic.session.editor.automatic_junctions;
        // The undo step opens with every sheet present; the cell view's
        // other sheets, which share these coordinates, step out of reach
        // only while this one's wires are joined.
        self.schematic.begin_operation("join collinear wires");
        with_active_wire_topology(self, |schematic, scope| {
            if schematic.join_collinear_wires(scope) {
                schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions, scope);
            }
        });
        let joined = self.schematic.end_operation();
        if joined {
            self.sync_active_schematic_to_workspace();
        }
        self.prompt(if joined {
            "Joined collinear wires."
        } else {
            "No collinear wires to join on this sheet."
        });
        joined
    }
}

/// Whether the schematic refuses edits. Silent: every place these commands are
/// offered, the menu, the context menu and the canvas keys, already greys them
/// on a read-only schematic.
fn refuse_read_only(state: &AppState) -> bool {
    state.schematic_edit_read_only()
}

fn refuse_empty(state: &mut AppState) {
    state.prompt("Select something first.");
}

/// Delete the selection as one undo step. The step opens with every sheet
/// present, so undo puts every sheet back; the cell view's other sheets,
/// which share these coordinates, are out of reach while the objects go, so
/// the dots the deletion tidies are judged by this sheet's wires alone.
///
/// Taking a wire or a part away can also make a dot due: a wire that ran
/// through a crossing leaves a T behind, or a pin on a wire with nothing
/// else crossing there. Under automatic junctions those dots are placed in
/// the same step.
fn delete_on_active_sheet(state: &mut AppState) -> bool {
    let selection = &state.schematic.session.editor.selection;
    let removes_conductors = !selection.wires.is_empty() || !selection.components.is_empty();
    state.schematic.begin_operation("delete selection");
    with_active_wire_topology(state, |schematic, scope| {
        schematic.delete_selection();
        if removes_conductors && schematic.session.editor.automatic_junctions {
            schematic.update_wire_junctions(scope);
        }
    });
    state.schematic.end_operation()
}

/// `objects` as the active cell view may receive them. A copy can come from
/// another cell view, so each hierarchical instance is bound again to its
/// master's interface as this project now holds it, and an instance of the
/// cell being edited is refused: the cell would contain itself. Names are
/// drawn fresh by the paste itself.
fn bound_to_active_cell<'a>(
    state: &AppState,
    objects: &'a ClipboardData,
) -> Result<Cow<'a, ClipboardData>, String> {
    let active = &state.workspace.content.active_view;
    let mut bound = Cow::Borrowed(objects);
    for (index, component) in objects.components.iter().enumerate() {
        let Some(binding) = component.library_cell.as_ref() else {
            continue;
        };
        let master = CellViewRef::new(&binding.library, &binding.cell, &binding.view);
        let buffer = state.workspace.content.schematic_buffers.get(&master.key());
        if binding.library == active.library
            && binding.cell == active.cell
            && (binding.view == active.view || buffer.is_some())
        {
            return Err(format!(
                "Cannot paste {}: it is an instance of {}/{}, the cell being edited, which cannot contain itself.",
                component.name, binding.library, binding.cell
            ));
        }
        let Some(buffer) = buffer else {
            continue;
        };
        let mut rebound = binding.clone();
        rebound.bind_interface(&buffer.document().interface_ports());
        bound.to_mut().components[index].library_cell = Some(rebound);
    }
    Ok(bound)
}

fn object_count(count: usize) -> String {
    if count == 1 {
        "1 object".to_owned()
    } else {
        format!("{count} objects")
    }
}

fn open_nets_clause(open: usize) -> String {
    if open == 1 {
        "1 net is now open".to_owned()
    } else {
        format!("{open} nets are now open")
    }
}

fn named_net_connections(kept: usize) -> String {
    if kept == 1 {
        "1 named-net connection".to_owned()
    } else {
        format!("{kept} named-net connections")
    }
}

/// Promote fine-grained wire edit handles to the complete conductor that owns
/// them. A corner handle is a way of grabbing a wire, not an object of its own,
/// so Delete removes the wire rather than inventing a partial one.
fn promote_wire_handles_to_complete_wires(
    schematic: &SchematicState,
    selection: &mut Selection,
) -> usize {
    let mut wire_ids = selection
        .wire_segments
        .iter()
        .filter_map(|selected| {
            schematic
                .document()
                .wires
                .iter()
                .find(|wire| {
                    wire.id == selected.wire_id && selected.segment_index < wire.segment_count()
                })
                .map(|wire| wire.id)
        })
        .chain(selection.wire_vertices.iter().filter_map(|selected| {
            schematic
                .document()
                .wires
                .iter()
                .find(|wire| {
                    wire.id == selected.wire_id && selected.vertex_index < wire.vertex_count()
                })
                .map(|wire| wire.id)
        }))
        .collect::<Vec<_>>();
    wire_ids.sort_unstable();
    wire_ids.dedup();
    let promoted_wire_count = wire_ids
        .iter()
        .filter(|wire_id| !selection.has_wire(**wire_id))
        .count();
    selection.wires.extend(wire_ids);
    selection.wire_segments.clear();
    selection.wire_vertices.clear();
    promoted_wire_count
}

fn complete_selection_count(schematic: &SchematicState, selection: &Selection) -> usize {
    schematic
        .document()
        .components
        .iter()
        .filter(|object| selection.has_component(object.id))
        .count()
        .saturating_add(
            schematic
                .document()
                .wires
                .iter()
                .filter(|object| selection.has_wire(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .document()
                .junctions
                .iter()
                .filter(|object| selection.has_junction(object.pos))
                .count(),
        )
        .saturating_add(
            schematic
                .document()
                .buses
                .iter()
                .filter(|object| selection.has_bus(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .document()
                .bus_taps
                .iter()
                .filter(|object| selection.has_bus_tap(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .document()
                .net_labels
                .iter()
                .filter(|object| selection.has_net_label(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .document()
                .design_notes
                .iter()
                .filter(|object| selection.has_design_note(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .document()
                .documentation_shapes
                .iter()
                .filter(|object| selection.has_documentation_shape(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .document()
                .probes
                .iter()
                .filter(|object| selection.has_probe(object.id))
                .count(),
        )
}

/// What a deletion touches beyond the objects themselves.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct DeleteDependencyImpact {
    nets: Vec<String>,
    saved_outputs: usize,
    probes: usize,
    specifications: usize,
    review_comments: usize,
}

impl DeleteDependencyImpact {
    /// The sentence the console adds after the headline, or `None` when the
    /// deletion touched nothing beyond itself.
    fn detail(&self) -> Option<String> {
        let mut parts = Vec::new();
        if !self.nets.is_empty() {
            parts.push(format!("Nets affected: {}.", name_list(&self.nets)));
        }
        let dependents = [
            (self.saved_outputs, "saved output", "saved outputs"),
            (self.probes, "probe", "probes"),
            (self.specifications, "specification", "specifications"),
            (self.review_comments, "review comment", "review comments"),
        ]
        .into_iter()
        .filter(|(count, _, _)| *count > 0)
        .map(|(count, one, many)| format!("{count} {}", if count == 1 { one } else { many }))
        .collect::<Vec<_>>();
        if !dependents.is_empty() {
            parts.push(format!("Still referenced by {}.", clause_list(&dependents)));
        }
        (!parts.is_empty()).then(|| parts.join(" "))
    }
}

/// `vout, n7`, with a tail count once the list stops being readable.
fn name_list(names: &[String]) -> String {
    const LIMIT: usize = 5;
    if names.len() <= LIMIT {
        return names.join(", ");
    }
    format!(
        "{} and {} more",
        names[..LIMIT].join(", "),
        names.len() - LIMIT
    )
}

/// `2 saved outputs and 1 probe`.
fn clause_list(clauses: &[String]) -> String {
    match clauses.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, head)) => format!("{} and {last}", head.join(", ")),
    }
}

/// Nets of the open view as the configured design resolves it.
fn design_nets(state: &AppState) -> Result<std::sync::Arc<Vec<DesignNet>>, String> {
    let projection = state
        .workspace
        .design_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    Ok(projection_nets(
        state.library_manager.catalog(),
        &projection,
        &state.workspace.content.active_view.key(),
    ))
}

/// What the deletion of `selection` would touch beyond the objects in it.
///
/// A design whose hierarchy does not resolve cannot name its nets; the
/// identities the drawing itself carries — instance names, label names, bus
/// declarations and tap slices — are still enough to find the saved outputs,
/// probes, specifications and review comments that name them, so those are
/// reported either way.
fn delete_dependency_impact(state: &AppState, selection: &Selection) -> DeleteDependencyImpact {
    let resolved_nets = design_nets(state).ok();
    let selected_junctions = selection
        .junctions
        .iter()
        .map(|junction| junction.pos)
        .collect::<HashSet<_>>();
    let selected_component_names = state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| selection.has_component(component.id))
        .map(|component| component.name.clone())
        .collect::<BTreeSet<_>>();
    let selected_label_names = state
        .schematic
        .document()
        .net_labels
        .iter()
        .filter(|label| selection.has_net_label(label.id))
        .map(|label| label.name.clone())
        .collect::<BTreeSet<_>>();

    let mut affected_nets = resolved_nets
        .iter()
        .flat_map(|nets| nets.iter())
        .filter(|net| {
            net.terminals
                .iter()
                .any(|terminal| selection.components.contains(&terminal.component_id))
                || net
                    .wire_ids
                    .iter()
                    .any(|wire_id| selection.wires.contains(wire_id))
                || state.schematic.document().wires.iter().any(|wire| {
                    net.wire_ids.contains(&wire.id)
                        && selected_junctions
                            .iter()
                            .any(|position| wire.contains_point(*position))
                })
                || selected_label_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&net.name))
        })
        .collect::<Vec<_>>();
    affected_nets.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
    });
    affected_nets.dedup_by(|left, right| left.name.eq_ignore_ascii_case(&right.name));

    let selected_bus_names = state
        .schematic
        .document()
        .buses
        .iter()
        .filter(|bus| selection.has_bus(bus.id))
        .filter_map(|bus| bus.declaration.as_ref().map(ToString::to_string))
        .collect::<Vec<_>>();
    let selected_tap_names = state
        .schematic
        .document()
        .bus_taps
        .iter()
        .filter(|tap| selection.has_bus_tap(tap.id))
        .map(|tap| tap.slice.to_string())
        .collect::<Vec<_>>();

    let affected_symbols = affected_nets
        .iter()
        .map(|net| net.name.clone())
        .chain(selected_component_names)
        .chain(selected_label_names)
        .chain(selected_bus_names.iter().cloned())
        .chain(selected_tap_names.iter().cloned())
        .collect::<BTreeSet<_>>();

    let affected_wire_ids = affected_nets
        .iter()
        .flat_map(|net| net.wire_ids.iter().copied())
        .collect::<HashSet<_>>();
    let mut affected_probes = BTreeSet::new();
    for probe in &state.schematic.document().probes {
        let references_symbol = probe
            .source_expression
            .as_deref()
            .is_some_and(|expression| references_any_symbol(expression, &affected_symbols));
        let lies_on_affected_wire = state.schematic.document().wires.iter().any(|wire| {
            affected_wire_ids.contains(&wire.id) && wire.contains_point(probe.position)
        });
        if references_symbol || lies_on_affected_wire {
            affected_probes.insert(probe.reference.clone());
        }
    }

    let mut affected_output_names = BTreeSet::new();
    for record in &state.workspace.content.simulation_plan_payloads {
        for output in &record.payload.saved_outputs {
            if references_any_symbol(&output.source_expression, &affected_symbols) {
                affected_output_names.insert(output.name.clone());
            }
        }
    }
    let mut affected_specifications = BTreeSet::new();
    for spec in &state.workspace.content.specs {
        if affected_output_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(&spec.measurement))
        {
            affected_specifications.insert(spec.measurement.clone());
        }
    }
    for record in &state.workspace.content.simulation_plan_payloads {
        for spec in &record.payload.specs {
            if affected_output_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(&spec.measurement))
            {
                affected_specifications.insert(spec.measurement.clone());
            }
        }
    }

    let mut affected_comments = BTreeSet::new();
    for note in state
        .schematic
        .document()
        .design_notes
        .iter()
        .filter(|note| note.review.is_some())
    {
        let selected = selection.has_design_note(note.id);
        let references_symbol = references_any_symbol(&note.text, &affected_symbols)
            || note.review.as_ref().is_some_and(|review| {
                review
                    .messages
                    .iter()
                    .any(|message| references_any_symbol(&message.body, &affected_symbols))
                    || review.evidence.iter().any(|evidence| {
                        references_any_symbol(&evidence.label, &affected_symbols)
                            || references_any_symbol(&evidence.source_identity, &affected_symbols)
                    })
            });
        if selected || references_symbol {
            let identity = note.review.as_ref().map_or_else(
                || format!("review note #{}", note.id),
                |review| review.record_id.clone(),
            );
            affected_comments.insert(identity);
        }
    }

    let nets = affected_nets
        .iter()
        .map(|net| net.name.clone())
        .chain(selected_bus_names)
        .chain(selected_tap_names)
        .collect::<BTreeSet<_>>();

    DeleteDependencyImpact {
        nets: nets.into_iter().collect(),
        saved_outputs: affected_output_names.len(),
        probes: affected_probes.len(),
        specifications: affected_specifications.len(),
        review_comments: affected_comments.len(),
    }
}

fn references_any_symbol(value: &str, symbols: &BTreeSet<String>) -> bool {
    let value = value.to_ascii_lowercase();
    symbols.iter().any(|symbol| {
        let symbol = symbol.to_ascii_lowercase();
        if symbol.is_empty() {
            return false;
        }
        value.match_indices(&symbol).any(|(start, _)| {
            let end = start + symbol.len();
            let before = (start > 0).then(|| value.as_bytes()[start - 1]);
            let after = (end < value.len()).then(|| value.as_bytes()[end]);
            !before.is_some_and(identifier_byte) && !after.is_some_and(identifier_byte)
        })
    })
}

const fn identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

/// How many nets the removal leaves open, or the reason the design cannot
/// answer. Both the sheet before the cut and the sheet after it are resolved
/// against the same projection, so the difference is the cut's doing and
/// nothing else's.
fn cut_open_net_count(state: &AppState, selection: &Selection) -> Result<usize, String> {
    let selected_components = &selection.components;
    let selected_wires = &selection.wires;
    let selected_junctions = selection
        .junctions
        .iter()
        .map(|junction| junction.pos)
        .collect::<HashSet<_>>();
    let projection = state
        .workspace
        .design_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    let before = projection_nets(
        state.library_manager.catalog(),
        &projection,
        &state.workspace.content.active_view.key(),
    );
    let mut after_schematic = state.schematic.clone();
    after_schematic.session.editor.selection = selection.clone();
    let _ = after_schematic.delete_selection();
    let hierarchy =
        HierarchySource::from_design_projection(state.library_manager.catalog(), &projection);
    let after = design_nets_with_hierarchy(&after_schematic, &hierarchy);

    Ok(before
        .iter()
        .filter(|net| {
            let touched = net
                .terminals
                .iter()
                .any(|terminal| selected_components.contains(&terminal.component_id))
                || net
                    .wire_ids
                    .iter()
                    .any(|wire_id| selected_wires.contains(wire_id))
                || state.schematic.document().wires.iter().any(|wire| {
                    net.wire_ids.contains(&wire.id)
                        && selected_junctions
                            .iter()
                            .any(|point| wire.contains_point(*point))
                });
            if !touched {
                return false;
            }
            let remaining = net
                .terminals
                .iter()
                .filter(|terminal| !selected_components.contains(&terminal.component_id))
                .map(terminal_identity)
                .collect::<HashSet<_>>();
            if remaining.is_empty() {
                return false;
            }
            let largest_retained_group = after
                .iter()
                .map(|candidate| {
                    candidate
                        .terminals
                        .iter()
                        .map(terminal_identity)
                        .filter(|terminal| remaining.contains(terminal))
                        .count()
                })
                .max()
                .unwrap_or(0);
            remaining.len() == 1 || largest_retained_group < remaining.len()
        })
        .count())
}

fn terminal_identity(
    terminal: &rspice_design::connectivity::summary::NetTerminal,
) -> (u64, String) {
    (terminal.component_id, terminal.pin.clone())
}

/// Points where the duplicated set leaves a net that keeps existing outside
/// it, paired with that net's name — or the reason the configured design
/// cannot name them.
///
/// The attachment carries a name into the duplicate, so the name has to be
/// the one the design gives the conductor. A duplicate stamped with an
/// editor-buffer name would attach to a net the run does not have.
fn named_external_attachments(
    state: &AppState,
    copy: &ClipboardData,
) -> Result<Vec<(Point, String)>, String> {
    let selected_components = &state.schematic.session.editor.selection.components;
    let captured_wires = copy
        .wires
        .iter()
        .map(|wire| wire.id)
        .collect::<HashSet<_>>();
    let captured_names = copy
        .net_labels
        .iter()
        .map(|label| label.name.as_str())
        .collect::<HashSet<_>>();
    let symbols = schematic_symbol_context(state);
    let terminal_points = state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| selected_components.contains(&component.id))
        .flat_map(|component| {
            symbols
                .named_terminal_points(component)
                .into_iter()
                .map(move |(pin, point)| ((component.id, pin), point))
        })
        .collect::<HashMap<_, _>>();

    let nets = design_nets(state)?;
    let mut attachments = Vec::new();
    for net in nets.iter() {
        if !net.authored_name
            || crate::state::NetLabel::validate_name(
                &net.name,
                state.schematic.document().document_policy.net_naming,
            )
            .is_err()
        {
            continue;
        }
        let selected_terminals = net
            .terminals
            .iter()
            .filter(|terminal| selected_components.contains(&terminal.component_id))
            .collect::<Vec<_>>();
        if selected_terminals.is_empty() {
            continue;
        }
        let external = net
            .terminals
            .iter()
            .any(|terminal| !selected_components.contains(&terminal.component_id))
            || net
                .wire_ids
                .iter()
                .any(|wire_id| !captured_wires.contains(wire_id))
            || !captured_names.contains(net.name.as_str());
        if !external {
            continue;
        }
        for terminal in selected_terminals {
            if let Some(point) = terminal_points.get(&(terminal.component_id, terminal.pin.clone()))
            {
                attachments.push((*point, net.name.clone()));
            }
        }
    }
    attachments.sort_by(|left, right| {
        (left.0.x, left.0.y, left.1.as_str()).cmp(&(right.0.x, right.0.y, right.1.as_str()))
    });
    attachments.dedup();
    Ok(attachments)
}

#[cfg(test)]
mod tests;
