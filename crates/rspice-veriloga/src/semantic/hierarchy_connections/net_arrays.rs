//! Bind statically selected net-array elements at ports. Four-state views share
//! bit identities; real views share the element's complete signal identity.
//! Neither form adds feedback drivers or confuses unpacked and packed coordinates.
use super::*;
use crate::array_index::UnpackedArrayLayout;
use crate::ast::{ArrayAccessExpr, DigitalExpr, PackedSelect, PartSelectExpr};

pub(super) fn prepare(
    file: &AnalyzedFile,
    source: &Module,
    module: &AnalyzedModule,
) -> CompileResult<Option<Arc<SpecializedModule>>> {
    if source.instances.is_empty()
        || !module.digital.signals.iter().any(|signal| {
            signal.unpacked.is_some() && matches!(signal.class, DigitalSignalClass::Net(_))
        })
    {
        return Ok(None);
    }
    let constants = super::super::instance_parameters::constants(source);
    let mut prepared = source.clone();
    let mut used = declared_names(source);
    let mut aliases = module.digital.bit_aliases.clone();
    let mut element_aliases = HashMap::new();
    let mut views: HashMap<(SmolStr, usize), SmolStr> = HashMap::new();
    for (instance_index, instance) in source.instances.iter().enumerate() {
        for (connection_index, connection) in instance.connections.iter().enumerate() {
            let (Connection::Named {
                signal: Some(actual),
                ..
            }
            | Connection::Ordered {
                signal: Some(actual),
                ..
            }) = connection
            else {
                continue;
            };
            let mut value = actual.clone();
            let mut pending = vec![&mut value];
            while let Some(expression) = pending.pop() {
                let selected = selection(module, expression);
                if let Some((declared, indices, packed)) = selected {
                    let indices: Option<Vec<_>> = indices
                        .into_iter()
                        .map(
                            |index| match crate::canonical_ir::digital_lower::elaboration_constant(
                                index,
                                &constants,
                                source.time_scale,
                            ) {
                                Some(crate::numeric_literal::NumericLiteralValue::Integer(
                                    value,
                                )) => Some(value),
                                _ => None,
                            },
                        )
                        .collect();
                    if let Some(indices) = indices {
                        let bounds: Vec<_> = declared
                            .dimensions
                            .iter()
                            .map(|axis| (axis.msb, axis.lsb))
                            .collect();
                        let layout = UnpackedArrayLayout::new(
                            &bounds,
                            super::super::SemanticAnalyzer::MAX_ARRAY_ELEMENTS,
                        )
                        .map_err(|_| {
                            error("invalid net array connection shape", expression.span())
                        })?;
                        if let Ok(offset) = layout.slot(&indices, 0) {
                            let span = expression.span();
                            let key = (declared.name.clone(), offset);
                            let view = if let Some(view) = views.get(&key) {
                                view.clone()
                            } else {
                                if aliases.len().saturating_add(declared.width as usize)
                                    > super::super::MAX_PARAMETER_ARRAY_ELEMENTS as usize
                                {
                                    return Err(error(
                                        "wire array connections exceed the module bit-alias limit",
                                        span,
                                    ));
                                }
                                let endpoint = endpoint(source, module, &declared.name)
                                    .expect("declared array");
                                let view = inputs::temporary_net(
                                    &mut prepared,
                                    &mut used,
                                    format!("__rspice_array_view_{}", views.len()).into(),
                                    &endpoint,
                                    declared,
                                    span,
                                );
                                if declared.class.is_real() {
                                    element_aliases.insert(
                                        view.clone(),
                                        super::super::digital::DigitalElementAlias {
                                            array: declared.name.clone(),
                                            offset: offset as u32,
                                        },
                                    );
                                }
                                for bit in 0..declared.width {
                                    aliases.push(
                                        super::super::digital::ElaboratedDigitalBitAlias {
                                            left: view.clone(),
                                            left_bit: bit,
                                            right: declared.name.clone(),
                                            right_element: Some(offset as u32),
                                            right_bit: bit,
                                            span,
                                        },
                                    );
                                }
                                views.insert(key, view.clone());
                                view
                            };
                            *expression = match packed {
                                None => Expression::Identifier(Identifier { name: view, span }),
                                Some(PackedSelect::Bit(index)) => {
                                    Expression::ArrayAccess(ArrayAccessExpr {
                                        normalized: false,
                                        packed: None,
                                        discrete_validity: None,
                                        array: view,
                                        index,
                                        span,
                                    })
                                }
                                Some(PackedSelect::Part { msb, lsb }) => {
                                    Expression::Digital(DigitalExpr::PartSelect(PartSelectExpr {
                                        name: view,
                                        msb,
                                        lsb,
                                        span,
                                    }))
                                }
                            };
                            continue;
                        }
                    }
                }
                super::super::flow_probes::for_child_mut(expression, &mut |child| {
                    pending.push(child)
                });
            }
            let (Connection::Named { signal, .. } | Connection::Ordered { signal, .. }) =
                &mut prepared.instances[instance_index].connections[connection_index];
            *signal = Some(value);
        }
    }
    if views.is_empty() {
        return Ok(None);
    }
    let mut analyzed = analyze_occurrence(
        file,
        &prepared,
        module.default_transition,
        module.default_discipline.clone(),
    )?;
    retain_parameter_guards(module, &mut analyzed);
    analyzed.hierarchical_connections = module.hierarchical_connections;
    analyzed.digital.bit_aliases = aliases;
    for signal in &mut analyzed.digital.signals {
        if let Some(alias) = element_aliases.remove(&signal.name) {
            signal.element_alias = Some(alias);
        }
    }
    Ok(Some(Arc::new(SpecializedModule {
        source: prepared,
        analyzed,
    })))
}

fn selection<'a>(
    module: &'a AnalyzedModule,
    expression: &'a Expression,
) -> Option<(
    &'a super::super::AnalyzedDigitalSignal,
    Vec<&'a Expression>,
    Option<PackedSelect>,
)> {
    let name = match expression {
        Expression::ArrayAccess(access) => &access.array,
        Expression::Digital(DigitalExpr::ArraySelect(access)) => &access.name,
        _ => return None,
    };
    let declared = module.digital.signals.iter().find(|signal| {
        signal.name == *name
            && signal.unpacked.is_some()
            && matches!(signal.class, DigitalSignalClass::Net(_))
    })?;
    let rank = declared.dimensions.len();
    let (indices, packed) = match expression {
        Expression::ArrayAccess(access) if rank == 1 => (vec![access.index.as_ref()], None),
        Expression::Digital(DigitalExpr::ArraySelect(access)) => {
            let (indices, packed) = access.split(rank)?;
            (indices, packed.cloned())
        }
        _ => return None,
    };
    Some((declared, indices, packed))
}
