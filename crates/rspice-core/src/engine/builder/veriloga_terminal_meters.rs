//! Terminal currents of Verilog-A instances, metered where a deck asks for them.
//!
//! Neither Verilog-A route keeps a current a deck can read at the solution. A
//! runtime-compiled body stamps its terminal rows and keeps nothing; a
//! generated body keeps the flow it stamped at its last evaluation, which is
//! the iterate the final linear solve started from rather than the point that
//! solve produced. So a terminal current a deck names is measured by an ideal
//! zero-volt source in series with that terminal, between the deck's net and a
//! private node the instance connects to instead.
//!
//! The meter's branch unknown is the current entering the terminal in the
//! solved system itself, at every point of every analysis: the resistive and
//! the charge part of every contribution, a potential contribution's branch
//! current, and — because the meter sits outside the instance's converters —
//! what a mixed module's boundary drives. A terminal nobody names gets no
//! meter, so the matrix of a run that asks for no such current is the matrix
//! it always was.
//!
//! The spelling is the one RSpice already uses for a device's lead current:
//! `@x1[ia]` (also `x1:ia`, `N(x1:ia)`, and Xyce's `IA(x1)` for a one-letter
//! port), where the label is `i` followed by the port name in lower case — the
//! label a generated model's catalog already gives each terminal. `I(x1)`
//! names the first terminal of an instance with at most two, as it does for
//! every two-terminal SPICE device; with more it names no single current and
//! is refused, naming the labels that do.
//!
//! A generated model an ordinary SPICE card routes to — `R1 in a r2_cmc`, or a
//! diode card whose model selects `diode_cmc` — is the same device and is
//! metered the same way, under the card's name: `I(r1)` means on it what it
//! means on the native resistor. Its terminals are the ones the routing will
//! instantiate it on, and a card's own spelling of a terminal current, a
//! diode's `@d1[id]`, reads the same meter as the catalog's label.
//!
//! The meters are inserted once the Verilog-A sources are resolved, because a
//! port's position and name come from its module, and before any device or
//! boundary is built, because both must see the private node as the
//! instance's own terminal.

use super::*;
use crate::engine::UnsupportedCapabilityError;
use crate::netlist::{
    Element, OutputSymbolKind, SaveSet, SaveSignal, canonical_symbol, hierarchy_pattern_matches,
    is_current_projection_accessor, measure_output_dependencies,
};

/// Capability token of every refusal this module makes.
const CAPABILITY: &str = "output.veriloga.terminal_current";

/// One Verilog-A instance's ports, as the deck's card connects them.
pub(super) struct InstancePorts {
    /// Module (or generated model) name, for diagnostics.
    pub(super) module: String,
    /// What a diagnostic calls the instance: an X-card's "Verilog-A
    /// instance", or the "generated Verilog-A device" an ordinary SPICE card
    /// routes to.
    pub(super) subject: &'static str,
    /// Whether the instance runs on the runtime route, whose instances report
    /// no device parameter other than these terminal currents.
    pub(super) runtime: bool,
    pub(super) ports: Vec<InstancePort>,
}

/// What an X-card instance is called in a diagnostic.
const INSTANCE: &str = "Verilog-A instance";

pub(super) struct InstancePort {
    /// Port name as the module declares it.
    pub(super) name: String,
    /// Current label: `i` followed by the port name in lower case.
    pub(super) label: String,
    /// Other labels the card gives this terminal's current, lower case: a
    /// diode card's `id` for its anode.
    pub(super) aliases: Vec<String>,
    /// The X-card node this port is connected to; `None` when the card leaves
    /// it unconnected.
    pub(super) node: Option<usize>,
    /// Nets the port spans: one, or one per bit of a vector discrete port.
    pub(super) width: usize,
    /// A mixed module's discrete port, which reaches its net through a
    /// converter that belongs to the instance.
    pub(super) discrete: bool,
}

/// What a deck asks of device currents, by spelling.
pub(super) struct CurrentRequests {
    /// Every `I(name)` probe, wildcards included, as a selection.
    currents: SaveSet,
    /// `I(name)` probes naming one device, in canonical spelling.
    exact_currents: Vec<String>,
    /// `@device[param]` probes: canonical device, lower-case parameter.
    parameters: Vec<(String, String)>,
    /// Bare `device:param` probes, in the same form. A bare name may also be
    /// a node, so one that names no terminal is not refused here.
    bare_parameters: Vec<(String, String)>,
    /// Device accessors inside output cards and measurements: upper-case
    /// operator, canonical symbol (possibly a pattern).
    accessors: Vec<(String, String)>,
}

impl CurrentRequests {
    pub(super) fn of(netlist: &Netlist) -> Self {
        let mut requests = Self {
            currents: SaveSet::default(),
            exact_currents: Vec::new(),
            parameters: Vec::new(),
            bare_parameters: Vec::new(),
            accessors: Vec::new(),
        };
        let probes = netlist.saves.signals.iter().chain(
            netlist
                .output_requests
                .iter()
                .flat_map(|request| request.operand_kinds.iter())
                .filter_map(|kind| match kind {
                    crate::netlist::OutputOperandKind::Probe(signal) => Some(signal),
                    crate::netlist::OutputOperandKind::Expression { .. } => None,
                }),
        );
        for signal in probes {
            match signal {
                SaveSignal::Current(name) => {
                    requests.currents.signals.push(signal.clone());
                    if !is_pattern(name) {
                        requests.exact_currents.push(canonical_symbol(name));
                    }
                }
                SaveSignal::DeviceParam { device, param } => requests
                    .parameters
                    .push((canonical_symbol(device), param.trim().to_ascii_lowercase())),
                SaveSignal::Raw(name) => {
                    if let Some((device, param)) = name.trim().rsplit_once(':')
                        && !device.trim().is_empty()
                        && !param.trim().is_empty()
                    {
                        requests
                            .bare_parameters
                            .push((canonical_symbol(device), param.trim().to_ascii_lowercase()));
                    }
                }
                _ => {}
            }
        }
        let measured = netlist
            .measurements
            .iter()
            .flat_map(measure_output_dependencies);
        let authored = netlist
            .output_requests
            .iter()
            .flat_map(|request| request.dependencies.iter().cloned());
        for dependency in authored.chain(measured) {
            if dependency.kind == OutputSymbolKind::Device {
                requests.accessors.push((
                    dependency.operator.trim().to_ascii_uppercase(),
                    canonical_symbol(&dependency.symbol),
                ));
            }
        }
        requests
    }

    fn is_empty(&self) -> bool {
        self.currents.signals.is_empty()
            && self.parameters.is_empty()
            && self.bare_parameters.is_empty()
            && self.accessors.is_empty()
    }

    /// Whether any request could name a current of `instance`. Every test
    /// below implies this one, so an element it rejects is passed over without
    /// resolving its ports.
    fn mentions(&self, instance: &str) -> bool {
        let canonical = canonical_symbol(instance);
        self.selects_instance_current(instance)
            || self
                .parameters
                .iter()
                .chain(&self.bare_parameters)
                .any(|(device, _)| *device == canonical)
            || self
                .accessors
                .iter()
                .any(|(_, symbol)| hierarchy_pattern_matches(symbol, &canonical))
    }

    /// Whether `I(instance)` — or a projection of it such as `IM(instance)` —
    /// is asked for, by name or by pattern.
    fn selects_instance_current(&self, instance: &str) -> bool {
        let canonical = canonical_symbol(instance);
        (!self.currents.signals.is_empty() && self.currents.selects(&format!("I({instance})")))
            || self.accessors.iter().any(|(operator, symbol)| {
                is_current_projection_accessor(operator)
                    && hierarchy_pattern_matches(symbol, &canonical)
            })
    }

    /// Whether `I(instance)` is asked for by name rather than by a pattern
    /// that happens to match it.
    fn names_instance_current(&self, instance: &str) -> bool {
        let canonical = canonical_symbol(instance);
        self.exact_currents.contains(&canonical)
            || self.accessors.iter().any(|(operator, symbol)| {
                is_current_projection_accessor(operator) && *symbol == canonical
            })
    }

    /// Whether a terminal's current is asked for under its label.
    fn selects_terminal(&self, instance: &str, label: &str) -> bool {
        let canonical = canonical_symbol(instance);
        self.parameters
            .iter()
            .chain(&self.bare_parameters)
            .any(|(device, param)| *device == canonical && param == label)
            || self.accessors.iter().any(|(operator, symbol)| {
                !is_current_projection_accessor(operator)
                    && operator.eq_ignore_ascii_case(label)
                    && hierarchy_pattern_matches(symbol, &canonical)
            })
    }

    /// Parameters asked of `instance` by name, lower case.
    fn parameters_of<'a>(&'a self, instance: &str) -> impl Iterator<Item = &'a str> + 'a {
        let canonical = canonical_symbol(instance);
        self.parameters
            .iter()
            .filter(move |(device, _)| *device == canonical)
            .map(|(_, param)| param.as_str())
    }
}

fn is_pattern(name: &str) -> bool {
    name.contains('*') || name.contains('?')
}

/// The label a terminal's current is saved under: `i` and the port name.
///
/// A generated model's catalog states its own labels, so only a runtime
/// module's ports are labelled here.
#[cfg(feature = "veriloga")]
fn current_label(port: &str) -> String {
    format!("i{}", port.to_ascii_lowercase())
}

/// Ports of an X-card instance of a runtime-compiled module.
///
/// The card names one node per port, in the module's port order, except that
/// a mixed module's vector discrete port takes one node per bit; an analog
/// instance may leave trailing ports unconnected.
#[cfg(feature = "veriloga")]
pub(super) fn runtime_instance_ports(
    element: &Element,
    entry: &veriloga_cache::CachedVerilogAModel,
) -> InstancePorts {
    let names = &entry.model.terminal_names;
    let mixed = entry
        .canonical_ir
        .as_deref()
        .filter(|artifact| artifact.digital.has_executable_content());
    let widths = mixed.map_or_else(
        || vec![1; names.len()],
        |artifact| mixed_modules::port_node_widths(artifact).collect(),
    );
    let mut cursor = 0usize;
    let ports = names
        .iter()
        .zip(widths.into_iter().chain(std::iter::repeat(1)))
        .map(|(name, width)| {
            let node = (cursor < element.nodes.len()).then_some(cursor);
            cursor = cursor.saturating_add(width);
            InstancePort {
                name: name.to_string(),
                label: current_label(name),
                aliases: Vec::new(),
                node,
                width,
                discrete: mixed.is_some_and(|artifact| {
                    artifact
                        .digital
                        .signals
                        .iter()
                        .any(|signal| signal.name == name.as_str())
                }),
            }
        })
        .collect();
    InstancePorts {
        module: entry.model.name.to_string(),
        subject: INSTANCE,
        runtime: true,
        ports,
    }
}

/// Ports of a generated model, labelled as the model's catalog labels them.
///
/// The first `authored` terminals connect to the card's nodes in order; the
/// rest are the card's omitted terminals. `aliases` are the card's other
/// spellings of a terminal's current.
#[cfg(feature = "veriloga-builtins-base")]
fn catalog_ports(
    model: &str,
    authored: usize,
    aliases: &[crate::device::veriloga_builtins::GeneratedTerminalCurrentAlias],
    subject: &'static str,
) -> Option<InstancePorts> {
    let descriptor = crate::device::veriloga_builtins::generated_veriloga_model_descriptors()
        .iter()
        .find(|descriptor| descriptor.model_name.eq_ignore_ascii_case(model))?;
    let ports = descriptor
        .terminals
        .iter()
        .enumerate()
        .map(|(index, terminal)| InstancePort {
            name: terminal.name.to_owned(),
            label: terminal.current_parameter.to_ascii_lowercase(),
            aliases: aliases
                .iter()
                .filter(|alias| alias.terminal.eq_ignore_ascii_case(terminal.name))
                .map(|alias| alias.parameter.to_ascii_lowercase())
                .collect(),
            node: (index < authored).then_some(index),
            width: 1,
            discrete: false,
        })
        .collect();
    Some(InstancePorts {
        module: descriptor.model_name.to_owned(),
        subject,
        runtime: false,
        ports,
    })
}

/// Ports of an X-card instance of a generated model.
#[cfg(feature = "veriloga-builtins-base")]
pub(super) fn generated_instance_ports(element: &Element, model: &str) -> Option<InstancePorts> {
    catalog_ports(model, element.nodes.len(), &[], INSTANCE)
}

/// Without a generated catalog no X-card names a generated model.
#[cfg(not(feature = "veriloga-builtins-base"))]
pub(super) fn generated_instance_ports(_element: &Element, _model: &str) -> Option<InstancePorts> {
    None
}

/// Ports of the generated device an ordinary SPICE card routes to:
/// `R1 in a r2_cmc`, or a diode card whose model selects `diode_cmc`.
#[cfg(feature = "veriloga-builtins-base")]
pub(super) fn card_routed_ports(
    netlist: &ModelResolution<'_>,
    element: &Element,
    config: &crate::engine::SimulationConfig,
) -> Option<InstancePorts> {
    let card = super::generated_model_routing::generated_card_terminals(netlist, element, config)?;
    catalog_ports(
        card.model_name,
        card.authored,
        card.aliases,
        "generated Verilog-A device",
    )
}

/// Without a generated catalog every SPICE card stays native.
#[cfg(not(feature = "veriloga-builtins-base"))]
pub(super) fn card_routed_ports(
    _netlist: &ModelResolution<'_>,
    _element: &Element,
    _config: &crate::engine::SimulationConfig,
) -> Option<InstancePorts> {
    None
}

/// Every label an instance's terminal currents are saved under, for a refusal
/// that names the supported spelling.
fn terminal_forms(instance: &str, ports: &InstancePorts) -> String {
    ports
        .ports
        .iter()
        .flat_map(|port| std::iter::once(&port.label).chain(&port.aliases))
        .map(|label| format!("@{instance}[{label}]"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn refusal(instance: &str, module: &str, detail: String) -> SimulationError {
    UnsupportedCapabilityError::new(CAPABILITY, detail)
        .for_instance(instance.to_owned())
        .for_module(module.to_owned())
        .into()
}

/// Insert a meter on every Verilog-A instance terminal the deck asks for.
///
/// Returns the meters, whose branch ordinals [`resolve`] fills in once the
/// circuit is built, and refuses a request that names no single terminal.
pub(super) fn materialize(
    netlist: &Netlist,
    elements: &mut Vec<Element>,
    known_device_names: &mut HashSet<String>,
    max_elements: usize,
    ports_of: impl Fn(&Element) -> Option<InstancePorts>,
    abort: &dyn AbortSignal,
) -> Result<Vec<crate::circuit::VerilogATerminalMeter>, SimulationError> {
    let requests = CurrentRequests::of(netlist);
    if requests.is_empty() {
        return Ok(Vec::new());
    }
    struct Planned {
        element: usize,
        node: usize,
        label: String,
        aliases: Vec<String>,
        instance_current: bool,
        discrete: bool,
    }
    let mut planned = Vec::new();
    for (index, element) in elements.iter().enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !requests.mentions(&element.name) {
            continue;
        }
        let Some(ports) = ports_of(element) else {
            continue;
        };
        let instance = element.name.as_str();
        let module = ports.module.as_str();
        let subject = ports.subject;
        if requests.names_instance_current(instance) && ports.ports.len() > 2 {
            return Err(refusal(
                instance,
                module,
                format!(
                    "`I({instance})` names the {subject} '{instance}' of '{module}', \
                     which has {} terminals and so no single current; save one terminal's \
                     current instead: {}",
                    ports.ports.len(),
                    terminal_forms(instance, &ports)
                ),
            ));
        }
        if ports.runtime {
            for param in requests.parameters_of(instance) {
                if !ports.ports.iter().any(|port| port.label == param) {
                    return Err(refusal(
                        instance,
                        module,
                        format!(
                            "`@{instance}[{param}]` names no terminal of the {subject} \
                             '{instance}' of '{module}'; its terminal currents are {}",
                            terminal_forms(instance, &ports)
                        ),
                    ));
                }
            }
        }
        let instance_current =
            ports.ports.len() <= 2 && requests.selects_instance_current(instance);
        for (position, port) in ports.ports.iter().enumerate() {
            let primary = instance_current && position == 0;
            if !primary
                && !std::iter::once(&port.label)
                    .chain(&port.aliases)
                    .any(|label| requests.selects_terminal(instance, label))
            {
                continue;
            }
            if let Some(other) = ports
                .ports
                .iter()
                .find(|other| other.label == port.label && other.name != port.name)
            {
                return Err(refusal(
                    instance,
                    module,
                    format!(
                        "`@{instance}[{}]` names two terminals of the {subject} \
                         '{instance}' of '{module}', '{}' and '{}', whose names differ only \
                         in case",
                        port.label, port.name, other.name
                    ),
                ));
            }
            if port.width != 1 {
                return Err(refusal(
                    instance,
                    module,
                    format!(
                        "`@{instance}[{}]` names the {}-bit discrete port '{}' of the \
                         {subject} '{instance}' of '{module}'; a terminal current is \
                         reported for a scalar port only",
                        port.label, port.width, port.name
                    ),
                ));
            }
            let Some(node) = port.node else {
                return Err(refusal(
                    instance,
                    module,
                    format!(
                        "`@{instance}[{}]` names the port '{}' of the {subject} \
                         '{instance}' of '{module}', which the card leaves unconnected",
                        port.label, port.name
                    ),
                ));
            };
            planned.push(Planned {
                element: index,
                node,
                label: port.label.clone(),
                aliases: port.aliases.clone(),
                instance_current: primary,
                discrete: port.discrete,
            });
        }
    }
    if planned.is_empty() {
        return Ok(Vec::new());
    }
    ResourceLimitError::ensure(
        ResourceKind::FlattenedElements,
        elements.len().saturating_add(planned.len()),
        max_elements,
    )?;
    let mut nodes = elements
        .iter()
        .flat_map(|element| element.nodes.iter())
        .map(|node| node.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let mut meters = Vec::with_capacity(planned.len());
    for plan in planned {
        let instance = elements[plan.element].name.clone();
        let source = format!("{instance}:{}", plan.label);
        let private = format!("{source}#meter");
        if !known_device_names.insert(source.to_ascii_uppercase())
            || !nodes.insert(private.to_ascii_uppercase())
        {
            return Err(SimulationError::Circuit(format!(
                "the current meter for terminal '{}' of the Verilog-A instance '{instance}' \
                 cannot be named '{source}': the deck already uses that name",
                plan.label
            )));
        }
        let external = std::mem::replace(
            &mut elements[plan.element].nodes[plan.node],
            private.clone(),
        );
        elements.push(Element {
            name: source.clone(),
            kind: ElementKind::VoltageSource(SourceSpec::Dc(0.0)),
            nodes: vec![external.clone(), private.clone()],
            provenance: Default::default(),
        });
        meters.push(crate::circuit::VerilogATerminalMeter {
            instance,
            label: plan.label,
            aliases: plan.aliases,
            instance_current: plan.instance_current,
            discrete: plan.discrete,
            source,
            private_node: private,
            external_node: external,
            branch: 0,
        });
    }
    Ok(meters)
}

/// Register the meters on the built circuit: their private nodes carry no
/// shunt and never become the reference, and each meter learns the branch
/// its current is solved in.
pub(super) fn resolve(
    circuit: &mut CircuitData,
    mut meters: Vec<crate::circuit::VerilogATerminalMeter>,
) -> Result<(), SimulationError> {
    for meter in &mut meters {
        let position = circuit
            .voltage_sources
            .names
            .iter()
            .position(|name| *name == meter.source)
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "the current meter '{}' of the Verilog-A instance '{}' was not built",
                    meter.source, meter.instance
                ))
            })?;
        meter.branch = circuit.voltage_sources.branch_indices[position];
        if let (Some(private), Some(external)) = (
            circuit.get_node_by_name(&meter.private_node),
            circuit.get_node_by_name(&meter.external_node).or(Some(0)),
        ) {
            circuit.terminal_probe_nodes.insert(private, external);
        }
        circuit
            .terminal_probe_source_names
            .insert(meter.source.to_ascii_uppercase());
    }
    circuit.veriloga_terminal_meters = meters;
    Ok(())
}
