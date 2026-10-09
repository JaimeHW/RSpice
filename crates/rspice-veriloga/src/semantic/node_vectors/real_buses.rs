//! Real buses retain array storage and expose scalar net views at boundaries.
use super::*;
use crate::array_index::UnpackedArrayLayout;
use crate::ast::{ArrayDimension, DigitalDeclItem, DigitalNetDecl};

#[derive(Debug, Clone)]
pub(crate) struct RealBus {
    pub bounds: VectorBounds,
    pub dimensions: Vec<VectorBounds>,
    /// Views in storage order; connections select them in declaration order.
    pub lanes: Vec<SmolStr>,
}

impl RealBus {
    pub fn lane(&self, coordinates: &[i64], span: Span) -> CompileResult<SmolStr> {
        let axes: Vec<_> = self
            .dimensions
            .iter()
            .map(|axis| (axis.msb, axis.lsb))
            .collect();
        let layout = UnpackedArrayLayout::new(&axes, SemanticAnalyzer::MAX_ARRAY_ELEMENTS)
            .expect("validated real bus shape");
        let offset = layout.slot(coordinates, 0).map_err(|_| {
            error(
                "real bus connection requires an in-range coordinate for each array axis",
                span,
            )
        })?;
        Ok(self.lanes[offset].clone())
    }
}

pub(super) fn expand(
    source: &Module,
    expanded: &mut Module,
    nodes: &mut PhysicalNodes,
    constants: &DigitalConstants,
    used: &mut HashSet<SmolStr>,
) -> CompileResult<()> {
    let mut added = Vec::new();
    let mut count = 0usize;
    for declaration in &mut expanded.digital_nets {
        if !declaration.kind.is_real() {
            continue;
        }
        let Some(range) = declaration.range.take() else {
            continue;
        };
        let bus_bounds = bounds(&range, constants, source.time_scale)?;
        for item in &mut declaration.items {
            if nodes.real_buses.contains_key(&item.name) {
                return Err(error(
                    format!("real bus '{}' is declared more than once", item.name),
                    item.span,
                ));
            }
            let ports: Vec<_> = source
                .port_declarations
                .iter()
                .filter(|port| port.names.contains(&item.name))
                .collect();
            if !ports.is_empty() && !item.dimensions.is_empty() {
                return Err(error(
                    "unpacked formal arrays require array port elaboration",
                    item.span,
                ));
            }
            for port in &ports {
                if port
                    .range
                    .as_ref()
                    .map(|range| bounds(range, constants, source.time_scale))
                    .transpose()?
                    != Some(bus_bounds)
                {
                    return Err(error(
                        format!(
                            "real port '{}' and its net declaration require identical bus bounds",
                            item.name
                        ),
                        port.span,
                    ));
                }
                if port.direction == PortDirection::Input {
                    nodes.real_input_buses.insert(item.name.clone());
                }
            }
            let mut dimensions = Vec::new();
            for axis in &item.dimensions {
                dimensions.push(VectorBounds {
                    msb: integer(&axis.start, constants, source.time_scale)?,
                    lsb: integer(&axis.end, constants, source.time_scale)?,
                });
            }
            dimensions.push(bus_bounds);
            let axes: Vec<_> = dimensions.iter().map(|axis| (axis.msb, axis.lsb)).collect();
            let layout = UnpackedArrayLayout::new(&axes, SemanticAnalyzer::MAX_ARRAY_ELEMENTS)
                .map_err(|_| {
                    error(
                        "real bus array exceeds the supported element limit",
                        item.span,
                    )
                })?;
            count = count
                .checked_add(layout.len())
                .ok_or_else(|| error("real bus lane count overflows", item.span))?;
            if count > MAX_PARAMETER_ARRAY_ELEMENTS as usize {
                return Err(error(
                    "real bus declarations exceed the module lane limit",
                    item.span,
                ));
            }
            let discipline = source
                .nets
                .iter()
                .find(|net| net.names.contains(&item.name))
                .and_then(|net| net.discipline.clone())
                .or_else(|| ports.iter().find_map(|port| port.discipline.clone()));
            let mut lanes = Vec::with_capacity(layout.len());
            for offset in 0..layout.len() {
                let mut lane: SmolStr =
                    crate::array_index::element_name(&item.name, &layout, offset).into();
                while !used.insert(lane.clone()) {
                    lane = format!("{lane}_").into();
                }
                nodes.real_aliases.insert(
                    lane.clone(),
                    DigitalElementAlias {
                        array: item.name.clone(),
                        offset: offset as u32,
                    },
                );
                lanes.push(lane);
            }
            added.push(DigitalNetDecl {
                kind: declaration.kind,
                signedness: declaration.signedness,
                range: None,
                items: lanes
                    .iter()
                    .map(|name| DigitalDeclItem {
                        name: name.clone(),
                        dimensions: Vec::new(),
                        init: None,
                        span: item.span,
                    })
                    .collect(),
                span: declaration.span,
            });
            if let Some(discipline) = discipline {
                expanded.nets.push(NetDecl {
                    dimensions: Vec::new(),
                    discipline: Some(discipline),
                    names: lanes.clone(),
                    range: None,
                    is_ground: false,
                    is_internal: true,
                    span: item.span,
                });
            }
            item.dimensions.push(ArrayDimension {
                start: range.msb.clone(),
                end: range.lsb.clone(),
                span: range.span,
            });
            nodes.real_buses.insert(
                item.name.clone(),
                RealBus {
                    bounds: bus_bounds,
                    dimensions,
                    lanes,
                },
            );
        }
    }
    if nodes.real_buses.is_empty() {
        return Ok(());
    }
    expanded.digital_nets.extend(added);
    let names = |name: &SmolStr| -> CompileResult<Vec<SmolStr>> {
        match nodes.real_buses.get(name) {
            Some(bus) => bus
                .bounds
                .indices_msb_first()
                .map(|index| bus.lane(&[index], source.span))
                .collect(),
            None => Ok(vec![name.clone()]),
        }
    };
    let mut ports = Vec::new();
    for port in &expanded.ports {
        ports.extend(names(&port.name)?.into_iter().map(|name| Port {
            name,
            span: port.span,
        }));
    }
    expanded.ports = ports;
    for (name, lanes) in &mut nodes.ports {
        if nodes.real_buses.contains_key(name) {
            *lanes = names(name)?;
        }
    }
    let mut declarations = Vec::new();
    for declaration in &expanded.port_declarations {
        for name in &declaration.names {
            let mut port = declaration.clone();
            port.names = names(name)?;
            if nodes.real_buses.contains_key(name) {
                port.range = None;
            }
            declarations.push(port);
        }
    }
    expanded.port_declarations = declarations;
    // Keep ranges on other names in a mixed discipline declaration.
    let mut nets = Vec::new();
    for declaration in &expanded.nets {
        for name in &declaration.names {
            let mut net = declaration.clone();
            net.names = vec![name.clone()];
            if let Some(bus) = nodes.real_buses.get(name) {
                if let Some(range) = &net.range {
                    if bounds(range, constants, source.time_scale)? != bus.bounds {
                        return Err(error(
                            "real net and discipline declarations require identical bus bounds",
                            net.span,
                        ));
                    }
                }
                net.range = None;
            }
            nets.push(net);
        }
    }
    expanded.nets = nets;
    Ok(())
}
