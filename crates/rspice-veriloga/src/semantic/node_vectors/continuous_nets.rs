//! A net type and a discipline describe one net, not two storage objects.
use super::*;

pub(super) fn normalize<'a>(
    module: &'a Module,
    db: &DisciplineDb,
) -> CompileResult<Cow<'a, Module>> {
    let physical: HashMap<_, _> = module
        .nets
        .iter()
        .map(|net| (&net.names, net.discipline.as_ref()))
        .chain(
            module
                .port_declarations
                .iter()
                .map(|port| (&port.names, port.discipline.as_ref())),
        )
        .filter_map(|(names, discipline)| {
            let discipline = discipline?;
            db.get_discipline(discipline)
                .is_some_and(|value| value.domain == Domain::Continuous)
                .then_some((names, discipline))
        })
        .flat_map(|(names, discipline)| names.iter().map(move |name| (name, discipline)))
        .collect();
    if physical.is_empty() {
        return Ok(Cow::Borrowed(module));
    }
    for declaration in &module.digital_variables {
        for item in &declaration.items {
            if let Some(discipline) = physical.get(&item.name) {
                return Err(error(
                    format!(
                        "digital variable '{}' requires a discrete discipline; '{}' is continuous",
                        item.name, discipline
                    ),
                    item.span,
                ));
            }
        }
    }
    // Index declarations once: a generated bank can contain many independent
    // wires, and validating each against every module declaration is quadratic.
    let mut net_shapes: HashMap<_, Vec<_>> = HashMap::new();
    for net in module.nets.iter().filter(|net| !net.is_ground) {
        let dimensions: HashMap<_, _> = net
            .dimensions
            .iter()
            .map(|(name, axes)| (name, axes.as_slice()))
            .collect();
        for name in &net.names {
            if physical.contains_key(name) {
                net_shapes
                    .entry(name)
                    .or_default()
                    .push((net, dimensions.get(name).copied().unwrap_or(&[])));
            }
        }
    }
    let mut port_shapes: HashMap<_, Vec<_>> = HashMap::new();
    for port in &module.port_declarations {
        for name in &port.names {
            if physical.contains_key(name) {
                port_shapes.entry(name).or_default().push(port);
            }
        }
    }
    let constants = DigitalConstants::from_module(module);
    let mut seen = HashSet::new();
    for declaration in &module.digital_nets {
        for item in &declaration.items {
            let Some(discipline) = physical.get(&item.name) else {
                continue;
            };
            if declaration.kind.is_real() {
                return Err(error(
                    format!(
                        "real net '{}' requires a discrete discipline; '{}' is continuous",
                        item.name, discipline
                    ),
                    item.span,
                ));
            }
            if !seen.insert(item.name.clone()) {
                return Err(error(
                    format!(
                        "duplicate net type declaration for continuous net '{}'",
                        item.name
                    ),
                    item.span,
                ));
            }
            // A wire declaration assignment is a continuous *digital* driver,
            // unlike a nodeset on a net-discipline declaration (VAMS 3.6.3.2).
            if let Some(initializer) = &item.init {
                return Err(error(
                    format!(
                        "continuous net '{}' cannot be driven by a digital net declaration assignment",
                        item.name
                    ),
                    initializer.span(),
                ));
            }
            let range = declaration
                .range
                .as_ref()
                .map(|range| bounds(range, &constants, module.time_scale))
                .transpose()?;
            let dimensions = axes(&item.dimensions, &constants, module.time_scale)?;
            for (net, other_dimensions) in net_shapes.get(&item.name).into_iter().flatten() {
                let other_range = net
                    .range
                    .as_ref()
                    .map(|range| bounds(range, &constants, module.time_scale))
                    .transpose()?;
                if range != other_range
                    || dimensions != axes(other_dimensions, &constants, module.time_scale)?
                {
                    return Err(error(
                        format!(
                            "continuous net '{}' has inconsistent wire and discipline shapes",
                            item.name
                        ),
                        net.span,
                    ));
                }
            }
            for port in port_shapes.get(&item.name).into_iter().flatten() {
                let other_range = port
                    .range
                    .as_ref()
                    .map(|range| bounds(range, &constants, module.time_scale))
                    .transpose()?;
                if range != other_range {
                    return Err(error(
                        format!(
                            "continuous port '{}' has inconsistent wire and direction ranges",
                            item.name
                        ),
                        port.span,
                    ));
                }
            }
        }
    }
    if seen.is_empty() {
        return Ok(Cow::Borrowed(module));
    }
    let mut normalized = module.clone();
    for declaration in &mut normalized.digital_nets {
        declaration.items.retain(|item| !seen.contains(&item.name));
    }
    normalized
        .digital_nets
        .retain(|declaration| !declaration.items.is_empty());
    Ok(Cow::Owned(normalized))
}

fn axes(
    dimensions: &[ArrayDimension],
    constants: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> CompileResult<Vec<(i64, i64)>> {
    dimensions
        .iter()
        .map(|axis| {
            Ok((
                integer(&axis.start, constants, time_scale)?,
                integer(&axis.end, constants, time_scale)?,
            ))
        })
        .collect()
}
