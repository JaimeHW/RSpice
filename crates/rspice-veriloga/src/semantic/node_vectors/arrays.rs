//! Conservative net arrays are elaborated into ordinary scalar physical nodes.
use super::*;
use crate::array_index::UnpackedArrayLayout;

#[derive(Debug, Clone)]
pub(crate) struct NodeArray {
    pub dimensions: Vec<VectorBounds>,
    pub bus: Option<VectorBounds>,
    pub lanes: Vec<SmolStr>,
    layout: UnpackedArrayLayout,
}

impl NodeArray {
    pub fn lane(&self, coordinates: &[i64], span: Span) -> CompileResult<SmolStr> {
        let offset = self.layout.slot(coordinates, 0).map_err(|_| error(
            "physical array access requires an in-range constant coordinate for every dimension", span,
        ))?;
        Ok(self.lanes[offset].clone())
    }

    pub fn select(
        &self,
        prefix: &[Expression],
        select: Option<&PackedSelect>,
        constants: &DigitalConstants,
        time_scale: crate::time_scale::ModuleTimeScale,
        span: Span,
    ) -> CompileResult<(Vec<SmolStr>, bool)> {
        let rank = self.dimensions.len() - usize::from(self.bus.is_some());
        let mut coordinates = prefix
            .iter()
            .map(|index| integer(index, constants, time_scale))
            .collect::<CompileResult<Vec<_>>>()?;
        let mut select = select;
        if coordinates.len() < rank {
            let Some(PackedSelect::Bit(index)) = select else {
                return Err(error(
                    "physical array connection is missing array coordinates",
                    span,
                ));
            };
            coordinates.push(integer(index, constants, time_scale)?);
            select = None;
        }
        if coordinates.len() != rank {
            return Err(error(
                "physical array connection has the wrong number of coordinates",
                span,
            ));
        }
        let Some(bus) = self.bus else {
            if select.is_some() {
                return Err(error(
                    "a scalar physical array element has no bus selection",
                    span,
                ));
            }
            return Ok((vec![self.lane(&coordinates, span)?], false));
        };
        let (range, vector) = match select {
            None => (bus, true),
            Some(PackedSelect::Bit(index)) => {
                let index = integer(index, constants, time_scale)?;
                (
                    VectorBounds {
                        msb: index,
                        lsb: index,
                    },
                    false,
                )
            }
            Some(PackedSelect::Part { msb, lsb }) => (
                VectorBounds {
                    msb: integer(msb, constants, time_scale)?,
                    lsb: integer(lsb, constants, time_scale)?,
                },
                true,
            ),
        };
        if !bus.contains(range.msb)
            || !bus.contains(range.lsb)
            || (range.msb != range.lsb && (range.msb > range.lsb) != (bus.msb > bus.lsb))
        {
            return Err(error(
                "physical array bus selection disagrees with its declared bounds or direction",
                span,
            ));
        }
        let mut lanes = Vec::new();
        coordinates.push(range.msb);
        for index in range.indices_msb_first() {
            *coordinates.last_mut().unwrap() = index;
            lanes.push(self.lane(&coordinates, span)?);
        }
        Ok((lanes, vector))
    }
}

pub(super) fn expand(
    source: &Module,
    expanded: &mut Module,
    nodes: &mut PhysicalNodes,
    constants: &DigitalConstants,
    used: &mut HashSet<SmolStr>,
    db: &DisciplineDb,
    mut count: u64,
) -> CompileResult<u64> {
    for declaration in &source.nets {
        let physical = declaration.is_ground
            || declaration.discipline.as_ref().is_some_and(|name| {
                db.get_discipline(name)
                    .is_some_and(|discipline| discipline.domain == Domain::Continuous)
            });
        if !physical {
            continue;
        }
        for (name, dimensions) in &declaration.dimensions {
            if source.ports.iter().any(|port| port.name == *name) {
                return Err(error(
                    "unpacked physical formal arrays require array port elaboration",
                    declaration.span,
                ));
            }
            let mut axes = dimensions
                .iter()
                .map(|axis| {
                    Ok(VectorBounds {
                        msb: integer(&axis.start, constants, source.time_scale)?,
                        lsb: integer(&axis.end, constants, source.time_scale)?,
                    })
                })
                .collect::<CompileResult<Vec<_>>>()?;
            let bus = declaration
                .range
                .as_ref()
                .map(|range| bounds(range, constants, source.time_scale))
                .transpose()?;
            if let Some(bus) = bus {
                axes.push(bus);
            }
            if let Some(previous) = nodes.arrays.get(name) {
                if previous.dimensions != axes || previous.bus != bus {
                    return Err(error(
                        format!("physical array '{name}' has inconsistent declarations"),
                        declaration.span,
                    ));
                }
                continue;
            }
            let shape: Vec<_> = axes.iter().map(|axis| (axis.msb, axis.lsb)).collect();
            let layout = UnpackedArrayLayout::new(&shape, SemanticAnalyzer::MAX_ARRAY_ELEMENTS)
                .map_err(|_| {
                    error(
                        "physical array exceeds the supported element limit",
                        declaration.span,
                    )
                })?;
            count = count
                .checked_add(layout.len() as u64)
                .ok_or_else(|| error("physical lane count overflows", declaration.span))?;
            if count > MAX_PARAMETER_ARRAY_ELEMENTS {
                return Err(error(
                    "physical declarations exceed the module lane limit",
                    declaration.span,
                ));
            }
            let mut lanes = Vec::with_capacity(layout.len());
            for offset in 0..layout.len() {
                let mut lane: SmolStr =
                    crate::array_index::element_name(name, &layout, offset).into();
                while !used.insert(lane.clone()) {
                    lane = format!("{lane}_").into();
                }
                lanes.push(lane);
            }
            nodes.arrays.insert(
                name.clone(),
                NodeArray {
                    dimensions: axes,
                    bus,
                    lanes,
                    layout,
                },
            );
        }
    }
    let mut declarations = Vec::new();
    for declaration in &expanded.nets {
        for name in &declaration.names {
            let mut net = declaration.clone();
            net.names = vec![name.clone()];
            net.dimensions.retain(|(target, _)| target == name);
            if let Some(array) = nodes.arrays.get(name) {
                if !declaration.is_ground && net.dimensions.is_empty() {
                    return Err(error(
                        format!(
                            "physical array '{name}' requires identical dimensions in every declaration"
                        ),
                        declaration.span,
                    ));
                }
                if declaration
                    .range
                    .as_ref()
                    .map(|range| bounds(range, constants, source.time_scale))
                    .transpose()?
                    .is_some_and(|range| Some(range) != array.bus)
                {
                    return Err(error(
                        "ground and physical array declarations have inconsistent bus ranges",
                        declaration.span,
                    ));
                }
                net.names = array.lanes.clone();
                net.range = None;
                net.dimensions.clear();
            }
            declarations.push(net);
        }
    }
    expanded.nets = declarations;
    Ok(count)
}
