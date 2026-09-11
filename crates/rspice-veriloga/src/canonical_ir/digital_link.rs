//! Link immutable instance plans into one digital scheduling and resolution domain.
//!
//! Whole-port net connections collapse; variable outputs retain their storage
//! and contribute through an ordinary continuous driver. Local CFG value and
//! block IDs remain local to their process. Signals, drivers, processes, probes
//! and source-file identities are relocated before the combined plan is sealed.

use std::collections::{BTreeMap, BTreeSet};

use super::digital::*;
use super::ids::{DigitalAnalogProbeId, DigitalProcessId, DigitalSignalId};
use super::{
    CfgTerminator, CfgValueKind, CfgValueType, CompilerPhase, DigitalWait, IrDiagnostic,
    SourceSpanRef, SsaBuilder,
};
use crate::PipelineControl;

type LinkResult<T> = Result<T, Vec<IrDiagnostic>>;

/// Direction of a resolved instance port at the linking boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalLinkDirection {
    Input,
    Output,
    Inout,
}

/// A port's resolved signal identity; a port alias need not match a signal name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalLinkPort {
    pub name: String,
    pub signal: DigitalSignalId,
    pub direction: DigitalLinkDirection,
}

/// One elaborated instance. Its plan and port declarations stay immutable.
pub struct DigitalLinkInstance<'a> {
    pub name: &'a str,
    pub plan: &'a CanonicalDigitalPlan,
    pub ports: &'a [DigitalLinkPort],
}

/// A whole-port connection to a named resolved net. Endpoints are instance/port
/// pairs. All endpoints must have the same value domain and width; their packed
/// index ranges may differ. Partial-port and width-converting connections need
/// explicit elaborated adapters before this linking stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalLinkNet {
    pub name: String,
    pub ports: Vec<(String, String)>,
}

/// Local-to-design identities for an instance's analog adapter and debugger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalLinkedInstance {
    pub name: String,
    pub signals: Vec<DigitalSignalId>,
    pub processes: Vec<DigitalProcessId>,
    pub drivers: BTreeMap<DigitalDriverId, DigitalDriverId>,
    pub connection_processes: Vec<DigitalProcessId>,
    pub analog_probes: Vec<DigitalAnalogProbeId>,
    /// Original file ID to design file ID; byte spans retain their source offsets.
    pub source_files: BTreeMap<u32, u32>,
}

/// A sealed plan plus names and ownership needed by circuit integration.
pub struct LinkedDigitalPlan {
    pub plan: CanonicalDigitalPlan,
    /// Includes collapsed instance-local names and every authored net alias.
    pub signal_names: BTreeMap<String, DigitalSignalId>,
    /// Sorted by instance name, matching deterministic process allocation.
    pub instances: Vec<DigitalLinkedInstance>,
}

fn error(detail: impl Into<String>) -> Vec<IrDiagnostic> {
    vec![IrDiagnostic::global_error(CompilerPhase::Artifact, detail)]
}

fn index(value: usize, entity: &str) -> LinkResult<u32> {
    u32::try_from(value).map_err(|_| error(format!("linked digital {entity} count exceeds u32")))
}

fn check_cancel(control: &dyn PipelineControl) -> LinkResult<()> {
    if control.is_cancelled() {
        Err(error("digital linking cancelled"))
    } else {
        Ok(())
    }
}

fn root(parents: &mut [usize], mut node: usize) -> usize {
    let mut result = node;
    while parents[result] != result {
        result = parents[result];
    }
    while parents[node] != node {
        let next = parents[node];
        parents[node] = result;
        node = next;
    }
    result
}

fn join(parents: &mut [usize], left: usize, right: usize) {
    let left = root(parents, left);
    let right = root(parents, right);
    parents[left.max(right)] = left.min(right);
}

struct Layout<'a> {
    instance: &'a DigitalLinkInstance<'a>,
    first_signal: usize,
    port_names: BTreeMap<&'a str, (usize, DigitalLinkDirection)>,
    map: DigitalLinkedInstance,
}

/// Link independently elaborated plans without recompiling source or executing
/// any process. Allocation order is stable under reordering the instance/net
/// input slices. Process-local module scales are preserved; runtime delay
/// conversion uses the finest precision of the linked design.
pub fn link_digital_plans(
    instances: &[DigitalLinkInstance<'_>],
    nets: &[DigitalLinkNet],
    control: &dyn PipelineControl,
) -> LinkResult<LinkedDigitalPlan> {
    check_cancel(control)?;
    if instances.is_empty() {
        return Err(error("a linked digital design needs an instance"));
    }
    let mut instances: Vec<_> = instances.iter().collect();
    instances.sort_by_key(|instance| instance.name);
    let mut slots = Vec::new();
    let mut layouts = Vec::new();
    let mut source_count = 0usize;
    let mut plan = CanonicalDigitalPlan {
        timing: instances[0].plan.timing,
        ..Default::default()
    };
    for instance in instances {
        check_cancel(control)?;
        if instance.name.is_empty()
            || layouts
                .last()
                .is_some_and(|previous: &Layout<'_>| previous.instance.name == instance.name)
        {
            return Err(error(format!(
                "duplicate or empty digital instance name '{}'",
                instance.name
            )));
        }
        instance.plan.validate().map_err(|diagnostics| {
            error(format!(
                "digital instance '{}': {}",
                instance.name,
                diagnostics
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ))
        })?;
        plan.timing.precision_exponent = plan
            .timing
            .precision_exponent
            .min(instance.plan.timing.precision_exponent);
        let mut files = BTreeSet::new();
        for span in instance
            .plan
            .signals
            .iter()
            .map(|item| item.span)
            .chain(instance.plan.processes.iter().map(|item| item.span))
            .chain(instance.plan.drivers.iter().map(|item| item.span))
            .chain(instance.plan.analog_probes.iter().map(|item| item.span))
        {
            files.insert(span.source_file_id);
        }
        let mut source_files = BTreeMap::new();
        for file in files {
            source_files.insert(file, index(source_count, "source file")?);
            source_count += 1;
        }
        let first_signal = slots.len();
        for (offset, signal) in instance.plan.signals.iter().enumerate() {
            if offset & 1023 == 0 {
                check_cancel(control)?;
            }
            index(slots.len(), "signal")?;
            let mut signal = signal.clone();
            signal.name = format!("{}.{}", instance.name, signal.name).into();
            signal.span.source_file_id = source_files[&signal.span.source_file_id];
            slots.push(signal);
        }
        let mut port_names = BTreeMap::new();
        for port in instance.ports {
            if port.name.is_empty() || instance.plan.signal(port.signal).is_none() {
                return Err(error(format!(
                    "digital instance '{}' has invalid port '{}'",
                    instance.name, port.name
                )));
            }
            let binding = (first_signal + usize::from(port.signal), port.direction);
            if port_names.insert(port.name.as_str(), binding).is_some() {
                return Err(error(format!(
                    "digital instance '{}' repeats port '{}'",
                    instance.name, port.name
                )));
            }
        }
        layouts.push(Layout {
            instance,
            first_signal,
            port_names,
            map: DigitalLinkedInstance {
                name: instance.name.into(),
                signals: Vec::new(),
                processes: Vec::new(),
                drivers: BTreeMap::new(),
                connection_processes: Vec::new(),
                analog_probes: Vec::new(),
                source_files,
            },
        });
    }
    let original_signals = slots.len();
    let instance_names: BTreeMap<_, _> = layouts
        .iter()
        .enumerate()
        .map(|(i, layout)| (layout.instance.name, i))
        .collect();
    let mut parents: Vec<_> = (0..slots.len()).collect();
    let mut nets: Vec<_> = nets.iter().collect();
    nets.sort_by_key(|net| &net.name);
    let mut previous_net: Option<&str> = None;
    // source slot, destination slot, owning instance. Variable ports cross
    // through a copy process; net ports join the common resolved signal.
    let mut copies = BTreeSet::new();
    for net in nets {
        check_cancel(control)?;
        if net.name.is_empty() || previous_net == Some(net.name.as_str()) || net.ports.is_empty() {
            return Err(error(format!(
                "digital net '{}' needs a unique nonempty name and at least one port",
                net.name
            )));
        }
        previous_net = Some(&net.name);
        let mut endpoints = BTreeMap::new();
        for (offset, (instance_name, port_name)) in net.ports.iter().enumerate() {
            if offset & 1023 == 0 {
                check_cancel(control)?;
            }
            let &owner = instance_names.get(instance_name.as_str()).ok_or_else(|| {
                error(format!(
                    "digital net '{}' names unknown instance '{instance_name}'",
                    net.name
                ))
            })?;
            let layout = &layouts[owner];
            let &(slot, direction) = layout.port_names.get(port_name.as_str()).ok_or_else(|| error(format!(
                "digital net '{}' names unknown discrete port '{instance_name}.{port_name}'", net.name)))?;
            if endpoints
                .insert((owner, slot), direction)
                .is_some_and(|previous| previous != direction)
            {
                return Err(error(format!(
                    "digital port aliases on '{}' disagree about direction",
                    slots[slot].name
                )));
            }
        }
        let (&(_, first), _) = endpoints.first_key_value().unwrap();
        let mut signal = slots[first].clone();
        signal.name = net.name.clone().into();
        signal.procedurally_assignable = false;
        let net_slot = slots.len();
        index(net_slot, "signal")?;
        slots.push(signal);
        parents.push(net_slot);
        for ((owner, slot), direction) in endpoints {
            if slots[slot].kind != slots[net_slot].kind
                || slots[slot].width != slots[net_slot].width
            {
                return Err(error(format!(
                    "digital net '{}' has incompatible domain, resolution or width at '{}'",
                    net.name, slots[slot].name
                )));
            }
            if slots[slot].procedurally_assignable {
                match direction {
                    DigitalLinkDirection::Output => {
                        copies.insert((slot, net_slot, owner));
                    }
                    DigitalLinkDirection::Input => {
                        copies.insert((net_slot, slot, owner));
                    }
                    DigitalLinkDirection::Inout => {
                        return Err(error(format!(
                            "bidirectional digital port '{}' must be a net",
                            slots[slot].name
                        )));
                    }
                }
            } else {
                join(&mut parents, slot, net_slot);
            }
        }
    }
    let roots: Vec<_> = (0..slots.len())
        .map(|slot| root(&mut parents, slot))
        .collect();
    let mut representatives = BTreeMap::new();
    for (slot, &root) in roots.iter().enumerate() {
        let chosen = representatives.entry(root).or_insert(slot);
        // Prefer the first explicitly named net to an internal port name.
        if *chosen < original_signals && slot >= original_signals {
            *chosen = slot;
        }
    }
    let mut root_ids = BTreeMap::new();
    for (&root, &representative) in &representatives {
        let id = DigitalSignalId::new(index(plan.signals.len(), "signal")?);
        let mut signal = slots[representative].clone();
        signal.id = id;
        // Normalize storage indices while retaining each process's original
        // read-select bounds. Write targets are translated below.
        signal.bounds = (signal.width > 1).then(|| (i64::from(signal.width) - 1, 0));
        plan.signals.push(signal);
        root_ids.insert(root, id);
    }
    let relocation: Vec<_> = roots.iter().map(|root| root_ids[root]).collect();
    let mut signal_names = BTreeMap::new();
    for (slot, signal) in slots.iter().enumerate() {
        let id = relocation[slot];
        if signal_names
            .insert(signal.name.to_string(), id)
            .is_some_and(|previous| previous != id)
        {
            return Err(error(format!(
                "linked digital name '{}' identifies different signals",
                signal.name
            )));
        }
    }
    let mut driver_counts = vec![0u32; plan.signals.len()];
    for layout in &mut layouts {
        check_cancel(control)?;
        let source = layout.instance.plan;
        layout.map.signals =
            relocation[layout.first_signal..layout.first_signal + source.signals.len()].to_vec();
        for probe in &source.analog_probes {
            let id = DigitalAnalogProbeId::new(index(plan.analog_probes.len(), "analog probe")?);
            let mut probe = probe.clone();
            probe.id = id;
            probe.positive = qualify_probe_node(layout.instance.name, &probe.positive).into();
            probe.negative = probe
                .negative
                .map(|node| qualify_probe_node(layout.instance.name, &node).into());
            relocate_span(&mut probe.span, &layout.map);
            layout.map.analog_probes.push(id);
            plan.analog_probes.push(probe);
        }
        for offset in 0..source.processes.len() {
            let count = plan
                .processes
                .len()
                .checked_add(offset)
                .ok_or_else(|| error("linked process count overflow"))?;
            layout
                .map
                .processes
                .push(DigitalProcessId::new(index(count, "process")?));
        }
        let mut drivers = BTreeMap::new();
        for driver in &source.drivers {
            let mut linked = driver.clone();
            relocate_target(&mut linked.target, source, &layout.map);
            linked.id = allocate_driver(linked.target.signal, &mut driver_counts)?;
            linked.process = layout.map.processes[usize::from(driver.process)];
            relocate_span(&mut linked.span, &layout.map);
            drivers.insert(driver.id, linked.id);
            plan.drivers.push(linked);
        }
        for process in &source.processes {
            check_cancel(control)?;
            let mut process = process.clone();
            process.id = layout.map.processes[usize::from(process.id)];
            relocate_span(&mut process.span, &layout.map);
            if let Some(sensitivity) = &mut process.static_sensitivity {
                relocate_terms(&mut sensitivity.terms, &layout.map);
            }
            for block in &mut process.function.blocks {
                if let CfgTerminator::Wait { wait, .. } = &mut block.terminator {
                    relocate_wait(wait, &layout.map);
                }
            }
            for (offset, value) in process.function.values.iter_mut().enumerate() {
                if offset & 1023 == 0 {
                    check_cancel(control)?;
                }
                relocate_value(&mut value.kind, source, &layout.map, &drivers)?;
            }
            plan.processes.push(process);
        }
        layout.map.drivers = drivers;
    }
    // Collapsed aliases of one output connection publish only one driver.
    let copies: BTreeSet<_> = copies
        .into_iter()
        .map(|(source, target, owner)| (relocation[source], relocation[target], owner))
        .collect();
    let mut variable_inputs = BTreeSet::new();
    for (source, target, owner) in copies {
        check_cancel(control)?;
        if plan.signals[usize::from(target)].procedurally_assignable
            && !variable_inputs.insert(target)
        {
            return Err(error(format!(
                "variable input '{}' is connected to multiple independent nets",
                plan.signals[usize::from(target)].name
            )));
        }
        let process = add_copy_process(
            &mut plan,
            source,
            target,
            &layouts[owner],
            &mut driver_counts,
        )?;
        layouts[owner].map.connection_processes.push(process);
    }
    check_cancel(control)?;
    let plan = plan.seal()?;
    Ok(LinkedDigitalPlan {
        plan,
        signal_names,
        instances: layouts.into_iter().map(|layout| layout.map).collect(),
    })
}

fn qualify_probe_node(instance: &str, node: &str) -> String {
    if node == "0" {
        node.into()
    } else {
        format!("{instance}.{node}")
    }
}

fn relocate_span(span: &mut SourceSpanRef, map: &DigitalLinkedInstance) {
    span.source_file_id = map.source_files[&span.source_file_id];
}

fn allocate_driver(signal: DigitalSignalId, counts: &mut [u32]) -> LinkResult<DigitalDriverId> {
    let count = &mut counts[usize::from(signal)];
    let id = DigitalDriverId {
        signal,
        index: *count,
    };
    *count = count
        .checked_add(1)
        .ok_or_else(|| error("linked driver count exceeds u32"))?;
    Ok(id)
}

fn relocate_terms(terms: &mut [DigitalSensitivityTerm], map: &DigitalLinkedInstance) {
    for term in terms {
        term.signal = map.signals[usize::from(term.signal)];
    }
}

fn relocate_wait(wait: &mut DigitalWait, map: &DigitalLinkedInstance) {
    match wait {
        DigitalWait::Event(terms) => relocate_terms(terms, map),
        DigitalWait::Expressions(_) | DigitalWait::Delay(_) => {}
        DigitalWait::Repeat { event, .. } => relocate_wait(event, map),
    }
}

fn relocate_target(
    target: &mut DigitalWriteTarget,
    source: &CanonicalDigitalPlan,
    map: &DigitalLinkedInstance,
) {
    let bounds = source.signals[usize::from(target.signal)].declared_range();
    let position = |index: i64| -> i64 {
        let delta = if bounds.msb >= bounds.lsb {
            i128::from(index) - i128::from(bounds.lsb)
        } else {
            i128::from(bounds.lsb) - i128::from(index)
        };
        // An unrepresentably distant index remains outside the normalized
        // signal. Saturation preserves ignored/clipped out-of-range writes.
        delta.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
    };
    target.select = match target.select {
        DigitalWriteSelect::Whole => DigitalWriteSelect::Whole,
        DigitalWriteSelect::Bit(bit) => DigitalWriteSelect::Bit(position(bit)),
        DigitalWriteSelect::Part { msb, lsb } => DigitalWriteSelect::Part {
            msb: position(msb),
            lsb: position(lsb),
        },
    };
    target.signal = map.signals[usize::from(target.signal)];
}

fn relocate_value(
    kind: &mut CfgValueKind,
    source: &CanonicalDigitalPlan,
    map: &DigitalLinkedInstance,
    drivers: &BTreeMap<DigitalDriverId, DigitalDriverId>,
) -> LinkResult<()> {
    match kind {
        CfgValueKind::DigitalExpression { function, .. } => {
            for value in &mut function.values {
                relocate_value(&mut value.kind, source, map, drivers)?;
            }
        }
        CfgValueKind::DigitalSignalRead { signal }
        | CfgValueKind::DigitalRealSignalRead { signal } => {
            *signal = map.signals[usize::from(*signal)];
        }
        CfgValueKind::DigitalAnalogPotential { probe } => {
            *probe = map.analog_probes[usize::from(*probe)]
        }
        CfgValueKind::DigitalBlockingWrite { target, .. } => relocate_target(target, source, map),
        CfgValueKind::DigitalNonblockingWrite { target, wait, .. } => {
            relocate_target(target, source, map);
            if let Some(wait) = wait {
                relocate_wait(wait, map);
            }
        }
        CfgValueKind::DigitalDriverWrite { target, driver, .. } => {
            relocate_target(target, source, map);
            *driver = drivers[driver];
        }
        CfgValueKind::FourStateConstant(_)
        | CfgValueKind::IntegerConstant(_)
        | CfgValueKind::RealConstant(_)
        | CfgValueKind::BlockParameter
        | CfgValueKind::DigitalTime { .. }
        | CfgValueKind::DigitalDelayTicks { .. }
        | CfgValueKind::DigitalRepeatCount { .. }
        | CfgValueKind::DigitalRealArithmetic { .. }
        | CfgValueKind::DigitalRealCompare { .. }
        | CfgValueKind::DigitalRealSelect { .. }
        | CfgValueKind::DigitalRealToBits { .. }
        | CfgValueKind::DigitalBitsToReal { .. }
        | CfgValueKind::DigitalIntegerToReal { .. }
        | CfgValueKind::DigitalRealToInteger { .. }
        | CfgValueKind::DigitalBitwise { .. }
        | CfgValueKind::DigitalBitwiseNot { .. }
        | CfgValueKind::DigitalLogical { .. }
        | CfgValueKind::DigitalLogicalNot { .. }
        | CfgValueKind::DigitalEquality { .. }
        | CfgValueKind::DigitalCaseMatch { .. }
        | CfgValueKind::DigitalRelational { .. }
        | CfgValueKind::DigitalArithmetic { .. }
        | CfgValueKind::DigitalShift { .. }
        | CfgValueKind::DigitalPartSelect { .. }
        | CfgValueKind::DigitalBitSelect { .. }
        | CfgValueKind::DigitalConcat { .. }
        | CfgValueKind::DigitalSelect { .. } => {}
        // A newly added signal-bearing value must acquire an explicit
        // relocation rule, never silently retain instance-local identities.
        _ => {
            return Err(error(
                "digital linker has no relocation rule for this CFG value",
            ));
        }
    }
    Ok(())
}

fn add_copy_process(
    plan: &mut CanonicalDigitalPlan,
    source: DigitalSignalId,
    target: DigitalSignalId,
    owner: &Layout<'_>,
    counts: &mut [u32],
) -> LinkResult<DigitalProcessId> {
    let id = DigitalProcessId::new(index(plan.processes.len(), "process")?);
    let span = plan.signals[usize::from(source)].span;
    let real = plan.signals[usize::from(source)].kind.is_real();
    let ty = if real {
        CfgValueType::Real
    } else {
        CfgValueType::FourState {
            width: plan.signals[usize::from(source)].width,
        }
    };
    let mut builder = SsaBuilder::new();
    let entry = builder.create_block();
    let value = builder.push(
        entry,
        ty,
        if real {
            CfgValueKind::DigitalRealSignalRead { signal: source }
        } else {
            CfgValueKind::DigitalSignalRead { signal: source }
        },
    );
    let target = DigitalWriteTarget {
        signal: target,
        select: DigitalWriteSelect::Whole,
    };
    let write = if plan.signals[usize::from(target.signal)].procedurally_assignable {
        CfgValueKind::DigitalBlockingWrite { target, value }
    } else {
        let driver = allocate_driver(target.signal, counts)?;
        plan.drivers.push(DigitalDriver {
            id: driver,
            target: target.clone(),
            process: id,
            span,
        });
        CfgValueKind::DigitalDriverWrite {
            driver,
            target,
            value,
        }
    };
    builder.push(entry, CfgValueType::Effect, write);
    builder.set_terminator(
        entry,
        CfgTerminator::Wait {
            wait: DigitalWait::Event(vec![DigitalSensitivityTerm {
                signal: source,
                edge: None,
            }]),
            resume: entry,
            resume_args: Vec::new(),
        },
    );
    builder.seal_all_blocks();
    let function = builder
        .finish(entry)
        .map_err(|detail| error(format!("digital port copy: {detail}")))?;
    plan.processes.push(CfgDigitalProcess {
        id,
        kind: DigitalProcessKind::ContinuousAssign,
        time_scale: owner.instance.plan.timing.root,
        function,
        static_sensitivity: None,
        span,
    });
    Ok(id)
}
