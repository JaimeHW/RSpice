//! Resolve authored branch shapes to the same scalar lane identities as nodes.
use super::*;

pub(super) fn expand(
    source: &Module,
    expanded: &mut Module,
    nodes: &mut PhysicalNodes,
    constants: &DigitalConstants,
    used: &mut HashSet<SmolStr>,
    mut count: u64,
) -> CompileResult<()> {
    expanded.branches.clear();
    let mut namespace = super::super::hierarchy_connections::declared_names(expanded);
    namespace.extend(nodes.vectors.keys().chain(nodes.arrays.keys()).cloned());
    for branch in &source.branches {
        if !namespace.insert(branch.name.clone()) {
            return Err(error(
                format!("duplicate branch declaration '{}'", branch.name),
                branch.span,
            ));
        }
        if branch.is_port
            && !source.ports.iter().any(|port| port.name == branch.pos)
            && !source
                .foreign_physical
                .get(&branch.pos)
                .is_some_and(|reference| {
                    matches!(reference.kind, ForeignPhysicalKind::Node { is_port: true })
                })
        {
            return Err(error(
                format!("port branch '{}' must name a declared port", branch.name),
                branch.span,
            ));
        }
        let (pos, pos_vector) = terminal(
            &branch.pos,
            &branch.pos_prefix,
            branch.pos_select.as_ref(),
            nodes,
            constants,
            source.time_scale,
            branch.span,
        )?;
        let (neg, neg_vector) = terminal(
            &branch.neg,
            &branch.neg_prefix,
            branch.neg_select.as_ref(),
            nodes,
            constants,
            source.time_scale,
            branch.span,
        )?;
        if pos_vector && neg_vector && pos.len() != neg.len() {
            return Err(error(
                "vector branch terminals must have equal widths",
                branch.span,
            ));
        }
        let width = pos.len().max(neg.len());
        let range = if let Some(range) = &branch.range {
            let range = bounds(range, constants, source.time_scale)?;
            if range.width() as usize != width {
                return Err(error(
                    format!(
                        "branch '{}' range has {} lanes but its terminals require {width}",
                        branch.name,
                        range.width()
                    ),
                    branch.span,
                ));
            }
            Some(range)
        } else if pos_vector || neg_vector {
            Some(VectorBounds {
                msb: 0,
                lsb: width as i64 - 1,
            })
        } else {
            None
        };
        count += width as u64;
        if count > MAX_PARAMETER_ARRAY_ELEMENTS {
            return Err(error(
                "physical declarations exceed the module lane limit",
                branch.span,
            ));
        }
        let lanes = if let Some(range) = range {
            let mut lanes = Vec::with_capacity(width);
            for index in range.indices_msb_first() {
                let mut name: SmolStr = format!("{}[{index}]", branch.name).into();
                while !used.insert(name.clone()) {
                    name = format!("{name}_").into();
                }
                lanes.push(name);
            }
            nodes.branches.insert(
                branch.name.clone(),
                NodeVector {
                    bounds: range,
                    lanes: lanes.clone(),
                },
            );
            lanes
        } else {
            vec![branch.name.clone()]
        };
        for (offset, name) in lanes.into_iter().enumerate() {
            let pos = pos[if pos_vector { offset } else { 0 }].clone();
            let neg = neg[if neg_vector { offset } else { 0 }].clone();
            if branch.is_port {
                nodes.port_branches.insert(name.clone(), pos.clone());
            }
            expanded.branches.push(BranchDecl {
                pos_prefix: Vec::new(),
                neg_prefix: Vec::new(),
                name,
                pos,
                neg,
                pos_select: None,
                neg_select: None,
                range: None,
                is_port: branch.is_port,
                span: branch.span,
            });
        }
    }
    Ok(())
}

pub(in crate::semantic) fn terminal(
    name: &SmolStr,
    prefix: &[Expression],
    select: Option<&PackedSelect>,
    nodes: &PhysicalNodes,
    constants: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    span: Span,
) -> CompileResult<(Vec<SmolStr>, bool)> {
    if let Some(array) = nodes.arrays.get(name) {
        return array.select(prefix, select, constants, time_scale, span);
    }
    if !prefix.is_empty() {
        return Err(error(
            "physical vector terminal has too many coordinates",
            span,
        ));
    }
    let vector = nodes.vectors.get(name);
    let Some(select) = select else {
        return Ok(vector.map_or_else(|| (vec![name.clone()], false), |v| (v.lanes.clone(), true)));
    };
    let vector =
        vector.ok_or_else(|| error(format!("'{name}' is not a physical vector node"), span))?;
    match select {
        PackedSelect::Bit(index) => Ok((
            vec![vector.lane(integer(index, constants, time_scale)?, span)?],
            false,
        )),
        PackedSelect::Part { msb, lsb } => {
            let range = VectorBounds {
                msb: integer(msb, constants, time_scale)?,
                lsb: integer(lsb, constants, time_scale)?,
            };
            if range.width() > MAX_DIGITAL_VECTOR_WIDTH {
                return Err(error(
                    "physical branch selection exceeds the lane limit",
                    span,
                ));
            }
            if range.msb != range.lsb
                && (range.msb > range.lsb) != (vector.bounds.msb > vector.bounds.lsb)
            {
                return Err(error(
                    "physical vector part-select direction disagrees with its declaration",
                    span,
                ));
            }
            Ok((
                range
                    .indices_msb_first()
                    .map(|i| vector.lane(i, span))
                    .collect::<CompileResult<_>>()?,
                true,
            ))
        }
    }
}
