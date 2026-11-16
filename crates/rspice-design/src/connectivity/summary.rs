//! Connectivity summaries shared by design inspection, publication and simulation preparation.

use super::{
    deck_bit_name, extract_with_hierarchy, net_at_schematic_point,
    terminal_positions_with_hierarchy,
};
use crate::hierarchy::HierarchySource;
use crate::library::metadata::{
    VERILOGA_PORTS_KEY, decode_source_view_ports, module_interface_view,
};
use crate::schematic::component::{BuiltinXspicePortType, Component, LibraryCellInstance};
use crate::schematic::document::SchematicDocument;
/// What a net carries at run, and which event side a converted net has.
pub use rspice_core_base::event_net::{EventOnlyNetKind, NetDomain};
use rspice_core_base::event_net::{NetEndpoints, PortCarrier};
use rspice_design_model::Point;
use rspice_design_model::port::PortDirection;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};

/// Electrical class of a design net.
///
/// The class is derived from declared design intent — the resolved ground
/// net, a declared `dir=supply` interface port, or a power net label — and
/// never guessed from the spelling of a name alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetClass {
    /// The simulation reference node.
    Ground,
    /// A declared supply/power rail.
    Supply,
    /// Any other conductor.
    Signal,
}

impl NetClass {
    /// Lower-case vocabulary shared with the inspector and the navigator.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Ground => "ground",
            Self::Supply => "supply",
            Self::Signal => "signal",
        }
    }
}

/// One component terminal bound to a net.
#[derive(Debug, Clone)]
pub struct NetTerminal {
    /// Owning component, for selection and cross-probing.
    pub component_id: u64,
    /// Instance reference designator as drawn.
    pub reference: String,
    /// Terminal name on that instance.
    pub pin: String,
}

/// One electrical net of the open schematic, summarized for navigation:
/// the rail's Nets segment, the net inspector, cross-probe highlighting,
/// search.
#[derive(Debug, Clone)]
pub struct DesignNet {
    /// SPICE name ("0", a label/port name, or autonamed `netN`).
    pub name: String,
    /// `true` when the name comes from an authored label, interface port, or
    /// ground symbol rather than the netlister's isolated-node fallback.
    pub authored_name: bool,
    /// Electrical class of the conductor.
    pub class: NetClass,
    /// Component terminals on this net, in document order.
    pub terminals: Vec<NetTerminal>,
    /// Declared direction when the net is an interface port of the cell.
    pub port: Option<PortDirection>,
    /// Wires belonging to this net (for canvas highlighting).
    pub wire_ids: Vec<u64>,
    /// What the net carries at run: electrical, logic, real, or a mix the
    /// engine converts. Read from the engine's own rule over what each pin
    /// on it carries, so it is known before anything has run.
    pub domain: NetDomain,
}

impl DesignNet {
    /// Component terminals bound to this net.
    pub fn pin_count(&self) -> usize {
        self.terminals.len()
    }

    /// `true` when the net is an interface port of the cell.
    pub const fn is_port(&self) -> bool {
        self.port.is_some()
    }

    /// `true` when the net is one pin that nothing reaches: no wire, no other
    /// pin, no port, no name. The design check finds that pin not connected,
    /// and the deck gives it a node of its own.
    pub fn is_unconnected_pin(&self) -> bool {
        self.class != NetClass::Ground
            && self.port.is_none()
            && !self.authored_name
            && self.wire_ids.is_empty()
            && self.terminals.len() == 1
    }

    /// The words the net's kind is shown in. A pin nothing reaches shows what
    /// the pin carries and that it is not connected, as the design check
    /// words it, rather than the converter a run would give its lone node.
    pub fn kind_label(&self) -> std::borrow::Cow<'static, str> {
        if !self.is_unconnected_pin() {
            return self.domain.label().into();
        }
        let carrier = match self.domain {
            NetDomain::Logic | NetDomain::Mixed(EventOnlyNetKind::Digital) => PortCarrier::Logic,
            NetDomain::Real | NetDomain::Mixed(EventOnlyNetKind::Real) => PortCarrier::Real,
            NetDomain::Electrical | NetDomain::LogicAndReal => PortCarrier::Electrical,
        };
        format!("{} · not connected", carrier.label()).into()
    }
}

/// Live net summary: connectivity + ports + labels + ground, no instance
/// generation. Cheap enough to recompute on topology change; callers cache
/// by `topology_version`.
pub fn design_nets(schematic: &impl AsRef<SchematicDocument>) -> Vec<DesignNet> {
    let schematic = schematic.as_ref();
    collect_design_nets(schematic, None, &mut Vec::new())
        .into_iter()
        .map(|(net, _)| net)
        .collect()
}

/// Live net summary with project hierarchy/symbol resolution enabled.
pub fn design_nets_with_hierarchy(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: &HierarchySource<'_>,
) -> Vec<DesignNet> {
    let schematic = schematic.as_ref();
    collect_design_nets(schematic, Some(hierarchy), &mut Vec::new())
        .into_iter()
        .map(|(net, _)| net)
        .collect()
}

/// Net summary of one cell view of a frozen design projection, extracted once
/// and then retained by the projection itself.
///
/// The projection keeps the slot type-erased so the design model never names
/// a generator type; the downcast back to [`DesignNet`] belongs here, where
/// the type is owned. A cell view the projection does not carry has no nets
/// rather than an error: the projection is the authority on which views exist.
pub fn projection_nets(
    catalog: &crate::library::LibraryCatalog,
    projection: &crate::projection::DesignProjection,
    cell_view_key: &str,
) -> Arc<Vec<DesignNet>> {
    let extract = || -> Arc<Vec<DesignNet>> {
        Arc::new(
            projection
                .schematic_buffers()
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(cell_view_key))
                .map(|(_, schematic)| {
                    design_nets_with_hierarchy(
                        schematic,
                        &HierarchySource::from_design_projection(catalog, projection),
                    )
                })
                .unwrap_or_default(),
        )
    };
    projection
        .memo_nets(cell_view_key, || extract() as Arc<dyn Any + Send + Sync>)
        .downcast::<Vec<DesignNet>>()
        .unwrap_or_else(|_| extract())
}

/// The nets of the sheet `occurrence` opens, each interface port's net
/// reading the kind of what it joins through the instance pins above it, as
/// the flattened deck the engine plans with does.
///
/// A master read alone sees only its own side of a port: its `en` reaching
/// one inverter input is one event endpoint. Opened under an instance whose
/// pin another inverter drives, the run's net is that whole net, so the
/// port's net reads it. The parent's net at the pin already counts what
/// meets inside the master, so the kind is the outermost net the port chain
/// reaches. Retained by the projection per occurrence; a root occurrence is
/// [`projection_nets`] itself.
pub fn occurrence_nets(
    catalog: &crate::library::LibraryCatalog,
    projection: &crate::projection::DesignProjection,
    occurrence: &crate::occurrence::DocumentOccurrence,
) -> Arc<Vec<DesignNet>> {
    let viewed = occurrence.terminal_master().key();
    let nets = projection_nets(catalog, projection, &viewed);
    if occurrence.steps.is_empty() {
        return nets;
    }
    let path = occurrence
        .steps
        .iter()
        .map(|step| step.instance_name.as_str())
        .collect::<Vec<_>>()
        .join("/");
    let memo = format!("{viewed}@{}/{path}", occurrence.root.key());
    let extract = || -> Arc<Vec<DesignNet>> {
        let mut joined = (*nets).clone();
        join_through_ports(catalog, projection, occurrence, &mut joined);
        Arc::new(joined)
    };
    projection
        .memo_nets(&memo, || extract() as Arc<dyn Any + Send + Sync>)
        .downcast::<Vec<DesignNet>>()
        .unwrap_or_else(|_| extract())
}

/// Give each interface-port net of `nets` (the occurrence's sheet) the kind
/// of the outermost net its port chain reaches, one level at a time.
fn join_through_ports(
    catalog: &crate::library::LibraryCatalog,
    projection: &crate::projection::DesignProjection,
    occurrence: &crate::occurrence::DocumentOccurrence,
    nets: &mut [DesignNet],
) {
    let hierarchy = HierarchySource::from_design_projection(catalog, projection);
    let sheet = |master: &rspice_design_model::cell_view::CellViewRef| {
        let key = master.key();
        projection
            .schematic_buffers()
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(&key))
            .map(|(_, schematic)| AsRef::<SchematicDocument>::as_ref(schematic))
    };
    let masters: Vec<_> = std::iter::once(&occurrence.root)
        .chain(occurrence.steps.iter().map(|step| &step.master))
        .collect();
    // Each net, by its index here and its name at the level reached; a net
    // that is no port's conductor at a level stops there.
    let mut reached: Vec<(usize, String)> = nets
        .iter()
        .enumerate()
        .map(|(index, net)| (index, net.name.clone()))
        .collect();
    for level in (1..masters.len()).rev() {
        let (Some(inner), Some(parent)) = (sheet(masters[level]), sheet(masters[level - 1])) else {
            return;
        };
        let name = &occurrence.steps[level - 1].instance_name;
        let Some(instance) = parent.components.iter().find(|component| {
            component.name.eq_ignore_ascii_case(name)
                || component.spice_instance_name().eq_ignore_ascii_case(name)
        }) else {
            return;
        };
        let extracted = extract_with_hierarchy(parent, Some(&hierarchy));
        let parent_nets = projection_nets(catalog, projection, &masters[level - 1].key());
        let ports = inner.interface_ports();
        reached.retain_mut(|(index, net)| {
            let Some(outer) = port_conductor(&ports, net)
                .and_then(|conductor| {
                    parent_net_at(parent, instance, conductor, &extracted, &hierarchy)
                })
                .and_then(|outer| {
                    parent_nets
                        .iter()
                        .find(|candidate| candidate.name.eq_ignore_ascii_case(&outer))
                })
            else {
                return false;
            };
            nets[*index].domain = outer.domain;
            net.clone_from(&outer.name);
            true
        });
        if reached.is_empty() {
            return;
        }
    }
}

/// The interface port of a sheet whose conductor the net `name` is: the
/// port's name, and for a vector port which bit, most significant first.
fn port_conductor<'a>(
    ports: &'a [rspice_design_model::port::PortSpec],
    name: &str,
) -> Option<(&'a str, Option<usize>)> {
    ports.iter().find_map(|port| {
        if port.name.eq_ignore_ascii_case(name) {
            return Some((port.name.as_str(), None));
        }
        let declaration = crate::schematic::bus::declared_vector(&port.name)?;
        declaration
            .members()
            .iter()
            .position(|member| {
                deck_bit_name(&declaration.name, member.index).eq_ignore_ascii_case(name)
            })
            .map(|position| (port.name.as_str(), Some(position)))
    })
}

/// The name of the net in `parent` on `instance`'s pin for a port conductor:
/// the scalar net on the pin, or the bit of the bus the vector pin meets.
/// A placement with no symbol numbers its pins; its binding's terminal order
/// says which port each number is.
fn parent_net_at(
    parent: &SchematicDocument,
    instance: &Component,
    (port, bit): (&str, Option<usize>),
    extracted: &super::ExtractedConnectivity,
    hierarchy: &HierarchySource<'_>,
) -> Option<String> {
    let order = instance
        .library_cell
        .as_ref()
        .map_or(&[][..], |binding| binding.terminal_order.as_slice());
    let position = terminal_positions_with_hierarchy(instance, Some(hierarchy))
        .into_iter()
        .enumerate()
        .find(|(index, (pin, _))| {
            pin.eq_ignore_ascii_case(port)
                || order
                    .get(*index)
                    .is_some_and(|name| name.eq_ignore_ascii_case(port))
        })
        .map(|(_, (_, position))| position)?;
    let Some(bit) = bit else {
        return net_at_schematic_point(parent, &extracted.nets, &extracted.point_to_net, position)
            .map(super::Net::spice_name);
    };
    let vector = extracted
        .vector_nets
        .nets
        .iter()
        .find(|vector| vector.terminals.contains(&position))?;
    let member = vector.declaration.members().into_iter().nth(bit)?;
    Some(deck_bit_name(&vector.declaration.name, member.index))
}

/// Resolved terminal names for every placed component, in the same order and
/// with the same authored-symbol authority used by hierarchical netlisting.
///
/// Publication uses this alongside the hierarchy-aware net summary so
/// disconnected pins remain visible without inventing names or falling back
/// to generic pin numbers when an authored symbol is available.
pub fn component_pin_names_with_hierarchy(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: &HierarchySource<'_>,
) -> HashMap<u64, Vec<String>> {
    let schematic = schematic.as_ref();
    schematic
        .components
        .iter()
        .map(|component| {
            (
                component.id,
                terminal_positions_with_hierarchy(component, Some(hierarchy))
                    .into_iter()
                    .map(|(name, _)| name)
                    .collect(),
            )
        })
        .collect()
}

/// Every net of `schematic` with what meets on it, counted through the
/// hierarchy: a pin of a placed schematic counts what meets on its net inside
/// that master, as the flattened deck the engine plans with does.
/// `visiting` holds the masters being read, so a master that places itself
/// is read once.
fn collect_design_nets(
    schematic: &SchematicDocument,
    hierarchy: Option<&HierarchySource<'_>>,
    visiting: &mut Vec<String>,
) -> Vec<(DesignNet, NetEndpoints)> {
    let super::ExtractedConnectivity {
        nets: extracted_nets,
        point_to_net,
        ground_net,
        vector_nets,
        ..
    } = extract_with_hierarchy(schematic, hierarchy);
    let net_at = |point| net_at_schematic_point(schematic, &extracted_nets, &point_to_net, point);

    let ports: HashMap<String, PortDirection> = schematic
        .interface_ports()
        .iter()
        .map(|port| (port.name.to_ascii_lowercase(), port.direction))
        .collect();

    // Terminals are collected in document order so the inspector's
    // connectivity table reads the same way twice for the same drawing.
    let mut terminals: HashMap<usize, Vec<NetTerminal>> = HashMap::new();
    let mut endpoints: HashMap<usize, NetEndpoints> = HashMap::new();
    // What the pin at each vector terminal carries, for the bits of its bus.
    let mut vector_pins = HashMap::new();
    let vector_terminals: HashSet<_> = vector_nets
        .nets
        .iter()
        .flat_map(|vector| vector.terminals.iter().copied())
        .collect();
    // A master's pins are read once, however often it is placed.
    let mut master_pins: HashMap<String, PinCarriers> = HashMap::new();
    let no_carriers = PinCarriers::new();
    for component in &schematic.components {
        // A ground symbol or an interface port is a name for a net, not a
        // conductor on it.
        let conductor = !component.kind.spice_prefix().is_empty();
        let pins = terminal_positions_with_hierarchy(component, hierarchy);
        let contract;
        let carriers = match component.library_cell.as_ref() {
            Some(binding) if binding.builtin_xspice.is_some() => {
                contract = code_model_carriers(binding);
                &contract
            }
            Some(binding) => master_pins
                .entry(HierarchySource::placed_master(binding).key())
                .or_insert_with(|| master_pin_carriers(binding, hierarchy, visiting)),
            None => {
                contract = legacy_code_model_carriers(component, &pins);
                &contract
            }
        };
        let carriers = if conductor { carriers } else { &no_carriers };
        // A placement with no symbol to name its pins numbers them; its
        // binding's terminal order says which port each number is, as the
        // deck writes them.
        let order = component
            .library_cell
            .as_ref()
            .map_or(&[][..], |binding| binding.terminal_order.as_slice());
        for (index, (pin, position)) in pins.into_iter().enumerate() {
            let key = [Some(pin.as_str()), order.get(index).map(String::as_str)]
                .into_iter()
                .flatten()
                .map(str::to_ascii_lowercase)
                .find(|key| carriers.contains_key(key));
            let pin_endpoints = key
                .as_ref()
                .and_then(|key| carriers.get(key))
                .copied()
                .unwrap_or_else(|| one_conductor(PortCarrier::Electrical, false));
            let on_bus = vector_terminals.contains(&position);
            if conductor && on_bus {
                let bits = key.map_or_else(Vec::new, |key| pin_bit_carriers(carriers, &key));
                vector_pins.insert(position, (pin_endpoints, bits));
            }
            if let Some(net) = net_at(position) {
                if conductor && !on_bus {
                    join(endpoints.entry(net.id).or_default(), pin_endpoints);
                }
                terminals.entry(net.id).or_default().push(NetTerminal {
                    component_id: component.id,
                    reference: component.name.clone(),
                    pin,
                });
            }
        }
    }
    // A vector pin puts one conductor on every bit of the bus it meets; a
    // placed schematic's vector pin puts on each bit what meets on that bit
    // inside, bit for bit in declared order, as the deck wires them.
    let net_ids: HashMap<String, usize> = extracted_nets
        .iter()
        .map(|net| (net.spice_name().to_ascii_lowercase(), net.id))
        .collect();
    for vector in &vector_nets.nets {
        for point in &vector.terminals {
            let Some((pin_endpoints, bits)) = vector_pins.get(point) else {
                continue;
            };
            let members = vector.declaration.members();
            let per_bit = bits.len() == members.len();
            for (position, member) in members.iter().enumerate() {
                let name = deck_bit_name(&vector.declaration.name, member.index);
                let carried = if per_bit {
                    bits[position]
                } else {
                    *pin_endpoints
                };
                if let Some(id) = net_ids.get(&name.to_ascii_lowercase()) {
                    join(endpoints.entry(*id).or_default(), carried);
                }
            }
        }
    }
    // A power label anywhere on the net declares it a supply rail.
    let mut power_labelled: HashSet<usize> = HashSet::new();
    for label in &schematic.net_labels {
        if label.is_power_net()
            && !label.is_ground()
            && let Some(net) = net_at(label.pos)
        {
            power_labelled.insert(net.id);
        }
    }

    let mut nets: Vec<(DesignNet, NetEndpoints)> = extracted_nets
        .iter()
        .map(|net| {
            let name = net.spice_name();
            let authored_name = net.label.is_some();
            let port = ports.get(&name.to_ascii_lowercase()).copied();
            let ground = ground_net == Some(net.id) || name == "0";
            let mut met = endpoints.remove(&net.id).unwrap_or_default();
            met.ground = ground;
            let class = if ground {
                NetClass::Ground
            } else if port == Some(PortDirection::Supply) || power_labelled.contains(&net.id) {
                NetClass::Supply
            } else {
                NetClass::Signal
            };
            let net = DesignNet {
                authored_name,
                class,
                port,
                terminals: terminals.remove(&net.id).unwrap_or_default(),
                wire_ids: net.wires.clone(),
                domain: met.domain(),
                name,
            };
            (net, met)
        })
        .collect();

    // Reading order: interface ports, then named nets, then autonamed.
    let autonamed = |name: &str| {
        name.strip_prefix("net")
            .is_some_and(|n| n.chars().all(|c| c.is_ascii_digit()))
    };
    nets.sort_by(|(a, _), (b, _)| {
        (
            !a.is_port(),
            autonamed(&a.name),
            a.name.to_ascii_lowercase(),
        )
            .cmp(&(
                !b.is_port(),
                autonamed(&b.name),
                b.name.to_ascii_lowercase(),
            ))
    });
    nets
}

/// What each pin of a placed component puts on its net, by folded pin name:
/// one conductor for a device or a module port, and for a pin of a placed
/// schematic what meets on its net inside the master. A pin it does not name
/// is one electrical conductor.
type PinCarriers = HashMap<String, NetEndpoints>;

/// One conductor carrying `carrier`; `module_port` as [`NetEndpoints::add`].
fn one_conductor(carrier: PortCarrier, module_port: bool) -> NetEndpoints {
    let mut endpoints = NetEndpoints::default();
    endpoints.add(carrier, module_port);
    endpoints
}

/// Count what `more` puts on a net into `net`.
fn join(net: &mut NetEndpoints, more: NetEndpoints) {
    net.ground |= more.ground;
    net.electrical += more.electrical;
    net.logic += more.logic;
    net.real += more.real;
    net.module_ports += more.module_ports;
}

/// The carrier of a code-model port of `port_type`: logic and real ports are
/// event endpoints, every other type is an electrical terminal.
fn code_model_port_carrier(port_type: rspice_core::xspice::PortType) -> PortCarrier {
    match port_type {
        rspice_core::xspice::PortType::Digital => PortCarrier::Logic,
        rspice_core::xspice::PortType::Real => PortCarrier::Real,
        _ => PortCarrier::Electrical,
    }
}

/// The pins of a code-model device placed by its own kind (an XSPICE gate
/// from the shelf, with no frozen contract), typed by the engine's code-model
/// table. The terminals are the model's ports in the order the deck writes
/// them; a gate's input terminals all meet its one vector input, so a model
/// whose ports all carry one thing types every terminal so. A device that is
/// not a code model has none.
fn legacy_code_model_carriers(component: &Component, pins: &[(String, Point)]) -> PinCarriers {
    let crate::schematic::device_descriptor::DeviceImplementation::Xspice { model_type } =
        component.kind.descriptor().implementation
    else {
        return PinCarriers::new();
    };
    let Some(ports) = code_model_port_carriers(model_type) else {
        return PinCarriers::new();
    };
    let carrier_of = |index: usize| match ports {
        [first, rest @ ..] if rest.iter().all(|carrier| carrier == first) => Some(*first),
        _ if ports.len() == pins.len() => Some(ports[index]),
        _ => None,
    };
    pins.iter()
        .enumerate()
        .filter_map(|(index, (pin, _))| {
            carrier_of(index)
                .map(|carrier| (pin.to_ascii_lowercase(), one_conductor(carrier, false)))
        })
        .collect()
}

/// The carrier of each port of the code model `model_type`, in port order,
/// read once from the engine's built-in code-model table.
fn code_model_port_carriers(model_type: &str) -> Option<&'static [PortCarrier]> {
    static TABLE: OnceLock<HashMap<&'static str, Vec<PortCarrier>>> = OnceLock::new();
    TABLE
        .get_or_init(|| {
            let registry = rspice_core::xspice::CodeModelRegistry::with_builtins();
            crate::schematic::component_type::ComponentType::ALL
                .iter()
                .filter_map(|kind| match kind.descriptor().implementation {
                    crate::schematic::device_descriptor::DeviceImplementation::Xspice {
                        model_type,
                    } => {
                        let model = registry.get(model_type)?;
                        let carriers = model
                            .ports()
                            .iter()
                            .map(|port| code_model_port_carrier(port.default_type))
                            .collect();
                        Some((model_type, carriers))
                    }
                    _ => None,
                })
                .collect()
        })
        .get(model_type)
        .map(Vec::as_slice)
}

/// The pins of a placed master: what meets on each pin's net inside a
/// schematic master, or the ports an HDL view states.
fn master_pin_carriers(
    binding: &LibraryCellInstance,
    hierarchy: Option<&HierarchySource<'_>>,
    visiting: &mut Vec<String>,
) -> PinCarriers {
    let Some(source) = hierarchy else {
        return PinCarriers::new();
    };
    let master = HierarchySource::placed_master(binding);
    let hdl = source
        .cell_declaration(&master)
        .and_then(|(_, view)| view)
        .is_some_and(|view| view.view_type.is_hdl_source());
    match source.schematic_master_for_binding(binding) {
        Some(inner) if !hdl => schematic_pin_carriers(inner, source, master.key(), visiting),
        _ => module_port_carriers(binding, hierarchy),
    }
}

/// Each interface port of the schematic master `inner`, by what meets on its
/// net inside it.
fn schematic_pin_carriers(
    inner: &SchematicDocument,
    source: &HierarchySource<'_>,
    key: String,
    visiting: &mut Vec<String>,
) -> PinCarriers {
    if visiting.contains(&key) {
        return PinCarriers::new();
    }
    visiting.push(key);
    let nets = collect_design_nets(inner, Some(source), visiting);
    visiting.pop();
    let met_on = |name: &str| {
        nets.iter()
            .find(|(net, _)| net.name.eq_ignore_ascii_case(name))
            .map(|(_, met)| NetEndpoints {
                ground: false,
                ..*met
            })
            .unwrap_or_default()
    };
    let mut carriers = PinCarriers::new();
    for port in inner.interface_ports() {
        let pin = port.name.to_ascii_lowercase();
        let Some(declaration) = crate::schematic::bus::declared_vector(&port.name) else {
            carriers.insert(pin, met_on(&port.name));
            continue;
        };
        // A vector port's bits are its declaration's deck bits inside; the
        // pin as a whole carries all of them, for a pin left open.
        let mut whole = NetEndpoints::default();
        for (position, member) in declaration.members().iter().enumerate() {
            let met = met_on(&deck_bit_name(&declaration.name, member.index));
            join(&mut whole, met);
            carriers.insert(bit_key(&pin, position), met);
        }
        carriers.insert(pin, whole);
    }
    carriers
}

/// The key a vector pin's bit at `position` (most significant first) is
/// carried under. `#` is in no authored name, so it meets no pin of its own.
fn bit_key(pin: &str, position: usize) -> String {
    format!("{pin}#{position}")
}

/// What each bit of the vector pin `pin` carries, most significant first,
/// when its master says bit by bit; empty when every bit carries the pin's
/// one carrier, as a module's vector port does.
fn pin_bit_carriers(carriers: &PinCarriers, pin: &str) -> Vec<NetEndpoints> {
    (0..)
        .map_while(|position| carriers.get(&bit_key(pin, position)).copied())
        .collect()
}

/// A built-in code model's event pins, as its frozen contract types them.
fn code_model_carriers(binding: &LibraryCellInstance) -> PinCarriers {
    let mut carriers = PinCarriers::new();
    for port in binding
        .builtin_xspice
        .iter()
        .flat_map(|contract| &contract.ports)
    {
        let carrier = match port.port_type {
            BuiltinXspicePortType::Digital => PortCarrier::Logic,
            BuiltinXspicePortType::Real => PortCarrier::Real,
            _ => continue,
        };
        for pin in port
            .terminals
            .iter()
            .filter_map(|terminal| binding.terminal_order.get(*terminal))
        {
            carriers.insert(pin.to_ascii_lowercase(), one_conductor(carrier, false));
        }
    }
    carriers
}

/// A module's pins, as the last clean compile of the HDL view it binds, or
/// that defines its cell's ports, stated them.
fn module_port_carriers(
    binding: &LibraryCellInstance,
    hierarchy: Option<&HierarchySource<'_>>,
) -> PinCarriers {
    let mut carriers = PinCarriers::new();
    let Some((cell, view)) = hierarchy
        .and_then(|source| source.cell_declaration(&HierarchySource::placed_master(binding)))
    else {
        return carriers;
    };
    let ports = view
        .filter(|view| view.view_type.is_hdl_source())
        .or_else(|| module_interface_view(cell))
        .and_then(|view| view.metadata.get(VERILOGA_PORTS_KEY))
        .and_then(|encoded| decode_source_view_ports(encoded));
    for port in ports.into_iter().flatten() {
        carriers.insert(
            port.name.to_ascii_lowercase(),
            one_conductor(port.carrier, true),
        );
    }
    carriers
}

/// Net name for every terminal in the design, keyed by instance and pin.
pub fn net_names_by_terminal(
    schematic: &impl AsRef<SchematicDocument>,
) -> HashMap<(u64, String), String> {
    keyed_by_terminal(design_nets(schematic))
}

pub fn keyed_by_terminal(
    nets: impl IntoIterator<Item = DesignNet>,
) -> HashMap<(u64, String), String> {
    nets.into_iter()
        .flat_map(|net| {
            let name = net.name.clone();
            net.terminals
                .into_iter()
                .map(move |terminal| ((terminal.component_id, terminal.pin), name.clone()))
        })
        .collect()
}

/// This source's nets, in the order its pins are declared.
///
/// A terminal with no net is reported as unconnected rather than skipped: a
/// source driving nothing is exactly what a reader scanning this list is
/// looking for, and dropping the entry would hide it.
pub fn terminal_nets(component: &Component, nets: &HashMap<(u64, String), String>) -> Vec<String> {
    component
        .terminal_positions()
        .into_iter()
        .map(|(pin, _)| {
            nets.get(&(component.id, pin.to_owned()))
                .cloned()
                .unwrap_or_else(|| "unconnected".to_owned())
        })
        .collect()
}

#[cfg(test)]
mod tests;
