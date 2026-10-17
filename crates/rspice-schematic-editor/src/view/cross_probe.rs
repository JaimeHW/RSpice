//! Selection and highlighting from host-authorized retained conductor geometry.

use super::design_view::object_is_on_active_sheet;
use crate::session::EditorSession;
use rspice_app_types::hierarchy_path::{InstancePath, ProbeTarget};
use rspice_design::schematic::component::{Component, component_for_engine_device};
use rspice_design::schematic::document::SchematicDocument;
use rspice_design_model::Point;
use rspice_design_model::design_management::SheetCatalog;
use std::collections::HashMap;

/// A synchronous view of the active drawing and its retained result geometry.
/// The host supplies `None` when the map's document or topology receipt is stale.
/// Workspace routing and result-map ownership remain with that host.
pub struct CrossProbeView<'a> {
    pub document: &'a SchematicDocument,
    /// The cell view's sheets, so only what the active sheet draws is ever
    /// selected; `None` for a cell view with no sheet catalog.
    pub sheet_catalog: Option<&'a SheetCatalog>,
    pub occurrence: &'a InstancePath,
    /// The active sheet's net points, in the coordinates that sheet draws.
    pub net_to_points: Option<&'a HashMap<String, Vec<Point>>>,
}

impl CrossProbeView<'_> {
    /// The drawn instance a run's device name refers to, if the active sheet
    /// draws it.
    fn drawn_device(&self, device: &str) -> Option<&Component> {
        component_for_engine_device(&self.document.components, self.occurrence, device)
            .filter(|component| object_is_on_active_sheet(self.sheet_catalog, component.id))
    }
}

/// The name inside a `V(...)` or `I(...)` wrapper, or `None` when the text
/// is anything else.
///
/// An expression such as `V(out)-V(in)` is deliberately rejected: it wraps
/// two signals, not one, and treating its interior as a net name would name
/// a conductor that does not exist.
pub fn wrapped_signal_name(name: &str, prefix: char) -> Option<&str> {
    let name = name.trim();
    let (head, tail) = name.split_once('(')?;
    if !head.eq_ignore_ascii_case(&prefix.to_string()) || !tail.ends_with(')') {
        return None;
    }
    let inner = tail[..tail.len() - 1].trim();
    (!inner.is_empty() && !inner.contains(['(', ')'])).then_some(inner)
}

/// Why a result signal could not be located on the schematic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocateSignalError {
    /// The signal is an expression or a device current, not a node voltage,
    /// so no single conductor carries it.
    NotANet,
    /// No retained cross-probe map matches the drawing as it stands, so the
    /// net's geometry is unknown.
    NoCurrentMap,
    /// The map is current but knows no conductor by that name.
    UnknownNet(String),
    /// The signal is read inside another instance than the one this tab is
    /// editing, so no conductor on screen carries it.
    OtherOccurrence(String),
}

impl LocateSignalError {
    pub fn message(&self, signal: &str) -> String {
        match self {
            Self::NotANet => {
                format!("{signal} is derived, not a node voltage — no single conductor carries it.")
            }
            Self::NoCurrentMap => {
                "The schematic changed since this result was produced; run again to cross-probe it."
                    .to_owned()
            }
            Self::UnknownNet(net) => {
                format!("The open sheet has no conductor named {net}.")
            }
            Self::OtherOccurrence(occurrence) => {
                format!(
                    "{signal} is read inside {occurrence}; descend into that instance to see it."
                )
            }
        }
    }
}

/// Select the conductor a result signal names, closing the probe loop from
/// the results workspace back to the drawing.
///
/// Fails closed rather than guessing: the retained cross-probe map must
/// belong to the open cell at its current topology, or the geometry it
/// holds describes a different drawing.
///
/// A trace name is an address, not a string: `V(x1.n1)` names the leaf `n1`
/// inside `/X1`, and only the leaf is drawn on a sheet. The scope is therefore
/// resolved against the tab's own occurrence and the leaf is what the geometry
/// is looked up by, so a node read in another instance says so instead of
/// selecting a same-named conductor here.
pub fn select_signal_conductor(
    view: &CrossProbeView<'_>,
    editor: &mut EditorSession,
    signal: &str,
) -> Result<String, LocateSignalError> {
    let (net, points) = locate_signal_conductor(view, signal)?;

    let wires: Vec<u64> = wires_touching(view, &points);
    editor.selection.clear();
    for wire in &wires {
        editor.selection.select_wire(*wire);
    }
    editor
        .net_highlight
        .highlight_wires(wires.into_iter().collect());
    editor.center_request = points
        .iter()
        .copied()
        .min_by_key(|point| (point.y, point.x));
    Ok(net)
}

/// Resolve one signal to the conductor geometry the open sheet draws for it.
///
/// The address rules live here so every caller reads the same map the same
/// way, whether it is locating one probed node or every node a failed run
/// named.
fn locate_signal_conductor(
    view: &CrossProbeView<'_>,
    signal: &str,
) -> Result<(String, Vec<Point>), LocateSignalError> {
    let (name, points) = borrow_signal_conductor(view, signal)?;
    Ok((name.to_owned(), points.to_vec()))
}

/// [`locate_signal_conductor`] without the copy.
///
/// The address rules are here rather than in the owning form because a caller
/// that only needs to know *whether* a name resolves runs every frame a row
/// carrying it is on screen. Cloning a net's whole point list to answer a
/// yes/no question would put that cost in the paint path.
fn borrow_signal_conductor<'a>(
    view: &CrossProbeView<'a>,
    signal: &str,
) -> Result<(&'a str, &'a [Point]), LocateSignalError> {
    let wrapped = wrapped_signal_name(signal, 'V').ok_or(LocateSignalError::NotANet)?;
    let target = ProbeTarget::parse_legacy(wrapped).map_err(|_| LocateSignalError::NotANet)?;
    let Some(net_to_points) = view.net_to_points else {
        return Err(LocateSignalError::NoCurrentMap);
    };
    // The trace's scope is spelled by written instance names — `xamp` for a
    // cell drawn `amp` — so it is compared with the tab's drawn occurrence by
    // those names.
    let occurrence = view.occurrence;
    if !target.scope.same_engine_scope(occurrence) {
        return Err(LocateSignalError::OtherOccurrence(target.scope.to_string()));
    }
    // Report the net as the design spells it, not as the trace happened to.
    net_to_points
        .iter()
        .find(|(name, points)| name.eq_ignore_ascii_case(&target.leaf) && !points.is_empty())
        .map(|(name, points)| (name.as_str(), points.as_slice()))
        .ok_or(LocateSignalError::UnknownNet(target.leaf.clone()))
}

/// How many of the objects a failed run named this sheet currently draws.
///
/// The question [`select_failure_sites`] answers by doing it. A surface that
/// offers the jump has to know before the click, or it offers an affordance
/// that refuses — so this reads the same map by the same rules and marks
/// nothing. `Err` is the whole-request refusal; `Ok(0)` means the map is
/// current and draws none of these names.
pub fn drawn_failure_site_count(
    view: &CrossProbeView<'_>,
    nets: &[String],
    devices: &[String],
) -> Result<usize, LocateSignalError> {
    if view.net_to_points.is_none() {
        return Err(LocateSignalError::NoCurrentMap);
    }
    let drawn_nets = nets
        .iter()
        .filter(|net| borrow_signal_conductor(view, &format!("V({net})")).is_ok())
        .count();
    let drawn_devices = devices
        .iter()
        .filter(|device| view.drawn_device(device).is_some())
        .count();
    Ok(drawn_nets + drawn_devices)
}

/// The active sheet's wires through any of `points`. A wire on a hidden sheet
/// can share the coordinates and is never one of them.
fn wires_touching(view: &CrossProbeView<'_>, points: &[Point]) -> Vec<u64> {
    view.document
        .wires
        .iter()
        .filter(|wire| object_is_on_active_sheet(view.sheet_catalog, wire.id))
        .filter(|wire| points.iter().any(|point| wire.contains_point(*point)))
        .map(|wire| wire.id)
        .collect()
}

/// What a request to mark a failed run's objects could actually mark.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FailureSiteSelection {
    /// Nets marked, spelled as the design spells them.
    pub nets: Vec<String>,
    /// Devices marked, spelled as the schematic spells them.
    pub devices: Vec<String>,
    /// Names the run attributed that this sheet does not draw — a node in
    /// another occurrence, or one the current drawing no longer carries.
    pub unlocated: Vec<String>,
}

impl FailureSiteSelection {
    pub fn is_empty(&self) -> bool {
        self.nets.is_empty() && self.devices.is_empty()
    }
}

/// Mark every design object a failed run named, and centre on the set.
///
/// Multi-object by construction. A convergence failure names the nodes it
/// could not settle; marking only the first would say the failure was about
/// that node, which is a different and false claim.
///
/// One refusal governs the whole request, because currency is a property of
/// the map and not of any one name in it. Names the current sheet does not
/// draw are reported rather than refused, so the caller can say how much of
/// the set it was able to show.
pub fn select_failure_sites(
    view: &CrossProbeView<'_>,
    editor: &mut EditorSession,
    nets: &[String],
    devices: &[String],
) -> Result<FailureSiteSelection, LocateSignalError> {
    if view.net_to_points.is_none() {
        return Err(LocateSignalError::NoCurrentMap);
    }

    let mut selection = FailureSiteSelection::default();
    let mut wires: Vec<u64> = Vec::new();
    let mut points: Vec<Point> = Vec::new();
    for net in nets {
        let signal = format!("V({net})");
        match locate_signal_conductor(view, &signal) {
            Ok((name, net_points)) => {
                wires.extend(wires_touching(view, &net_points));
                points.extend(net_points);
                selection.nets.push(name);
            }
            // The map is current — that was settled above — so a name that
            // does not resolve is one this sheet does not draw, not a reason
            // to abandon the names that do.
            Err(_) => selection.unlocated.push(net.clone()),
        }
    }

    let components: Vec<(u64, String, Point)> = devices
        .iter()
        .filter_map(|device| {
            view.drawn_device(device)
                .map(|component| (component.id, component.spice_instance_name(), component.pos))
                .or_else(|| {
                    selection.unlocated.push(device.clone());
                    None
                })
        })
        .collect();

    editor.selection.clear();
    for wire in &wires {
        editor.selection.select_wire(*wire);
    }
    for (id, name, position) in components {
        editor.selection.select_component(id);
        selection.devices.push(name);
        points.push(position);
    }
    editor
        .net_highlight
        .highlight_wires(wires.into_iter().collect());
    editor.center_request = points
        .iter()
        .copied()
        .min_by_key(|point| (point.y, point.x));
    Ok(selection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::wire::Wire;

    fn fixture() -> (
        SchematicDocument,
        HashMap<String, Vec<Point>>,
        EditorSession,
    ) {
        let a = Point::new(0, 0);
        let b = Point::new(40, 0);
        (
            SchematicDocument {
                wires: vec![Wire::new(1, vec![a, b])],
                ..Default::default()
            },
            HashMap::from([("OUT".to_owned(), vec![a, b])]),
            EditorSession::default(),
        )
    }

    #[test]
    fn locating_a_node_voltage_selects_and_highlights_its_conductor() {
        let (document, points, mut editor) = fixture();
        let occurrence = InstancePath::root();
        let view = CrossProbeView {
            document: &document,
            sheet_catalog: None,
            occurrence: &occurrence,
            net_to_points: Some(&points),
        };

        let net = select_signal_conductor(&view, &mut editor, "V(out)").expect("net resolves");

        assert_eq!(net, "OUT");
        assert!(editor.selection.wires.contains(&1));
        assert!(editor.net_highlight.is_wire_highlighted(1));
        assert_eq!(editor.center_request, Some(Point::new(0, 0)));
    }

    #[test]
    fn a_derived_signal_explains_itself_instead_of_selecting_geometry() {
        let (document, points, mut editor) = fixture();
        let occurrence = InstancePath::root();
        let view = CrossProbeView {
            document: &document,
            sheet_catalog: None,
            occurrence: &occurrence,
            net_to_points: Some(&points),
        };

        let error = select_signal_conductor(&view, &mut editor, "V(out)-V(in)")
            .expect_err("an expression is not a net");

        assert_eq!(error, LocateSignalError::NotANet);
        assert!(error.message("V(out)-V(in)").contains("derived"));
        assert!(editor.selection.is_empty());
    }

    #[test]
    fn an_unknown_net_is_reported_by_name() {
        let (document, points, mut editor) = fixture();
        let occurrence = InstancePath::root();
        let view = CrossProbeView {
            document: &document,
            sheet_catalog: None,
            occurrence: &occurrence,
            net_to_points: Some(&points),
        };

        let error = select_signal_conductor(&view, &mut editor, "V(missing)")
            .expect_err("no conductor carries this net");

        assert_eq!(error, LocateSignalError::UnknownNet("missing".to_owned()));
        assert!(error.message("V(missing)").contains("missing"));
    }

    /// The engine scopes a cell drawn `amp` by the name the deck writes it
    /// under, so its trace is `V(xamp.out)`. On that cell's tab the trace is
    /// this sheet's conductor; on any other occurrence it is not.
    #[test]
    fn a_trace_scoped_by_the_written_instance_name_is_this_occurrences_net() {
        let (document, points, mut editor) = fixture();
        let occurrence = InstancePath::parse("/amp").expect("drawn occurrence");
        let view = CrossProbeView {
            document: &document,
            sheet_catalog: None,
            occurrence: &occurrence,
            net_to_points: Some(&points),
        };

        let net = select_signal_conductor(&view, &mut editor, "V(xamp.out)")
            .expect("the written scope is this occurrence");
        assert_eq!(net, "OUT");

        let other = InstancePath::parse("/amp2").expect("another occurrence");
        let elsewhere = CrossProbeView {
            occurrence: &other,
            ..view
        };
        assert!(matches!(
            select_signal_conductor(&elsewhere, &mut editor, "V(xamp.out)"),
            Err(LocateSignalError::OtherOccurrence(_))
        ));
    }

    /// A failed run names devices as the engine does. On the sheet of `/amp`
    /// the resistor drawn `C4` is `xamp.rc4`; a root device of the same name
    /// is not on this sheet.
    #[test]
    fn failure_devices_are_found_by_their_written_names_in_this_occurrence() {
        use rspice_design::schematic::component::Component;
        use rspice_design::schematic::component_type::ComponentType;

        let (mut document, points, mut editor) = fixture();
        document.components.push(
            Component::new(7, ComponentType::Resistor, Point::new(200, 200))
                .with_name_value("C4", "1k"),
        );
        let occurrence = InstancePath::parse("/amp").expect("drawn occurrence");
        let view = CrossProbeView {
            document: &document,
            sheet_catalog: None,
            occurrence: &occurrence,
            net_to_points: Some(&points),
        };
        let devices = ["xamp.rc4".to_owned(), "rc4".to_owned()];

        assert_eq!(drawn_failure_site_count(&view, &[], &devices), Ok(1));
        let selection =
            select_failure_sites(&view, &mut editor, &[], &devices).expect("the map is current");
        assert_eq!(selection.devices, ["C4"]);
        assert_eq!(selection.unlocated, ["rc4"]);
        assert!(editor.selection.components.contains(&7));
    }
}
