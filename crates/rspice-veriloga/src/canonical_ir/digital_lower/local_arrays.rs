//! Static local arrays use the same element store and operations as module arrays.
use super::super::digital::{DigitalArray, DigitalArrayRef, DigitalLocalStorage};
use super::*;

impl ProcessLowerer<'_> {
    pub(super) fn declare_local_array(
        &mut self,
        block: BlockId,
        local: DigitalLocalId,
        dimensions: &[crate::ast::ArrayDimension],
        scope_names: &BTreeSet<String>,
    ) {
        if dimensions.is_empty() {
            return;
        }
        let [dimension] = dimensions else {
            self.error(
                "multidimensional process-local arrays require multidimensional storage lowering",
                dimensions[0].span,
            );
            return;
        };
        let mut reads = BTreeSet::new();
        collect_expression_reads(&dimension.start, &mut reads);
        collect_expression_reads(&dimension.end, &mut reads);
        let bounds = if reads.iter().any(|name| scope_names.contains(name)) {
            None
        } else {
            self.constant(&dimension.start)
                .zip(self.constant(&dimension.end))
        };
        let Some((msb, lsb)) = bounds else {
            self.error(
                "process-local array bounds must be constant signed 64-bit integers",
                dimension.span,
            );
            return;
        };
        let Some(len) = msb
            .abs_diff(lsb)
            .checked_add(1)
            .and_then(|len| u32::try_from(len).ok())
        else {
            self.error(
                "process-local array extent exceeds supported storage",
                dimension.span,
            );
            return;
        };
        if let Some(signal) = self.analog_local_signal(local) {
            let Some(array) = self.arrays.get(&signal).copied() else {
                self.invariant("analog local array has no shared layout", dimension.span);
                return;
            };
            if array.lower != msb.min(lsb) || array.len != len {
                self.invariant(
                    "analog local array disagrees with its shared layout",
                    dimension.span,
                );
                return;
            }
            let initial = self.read_local(block, local);
            for offset in 0..array.len {
                let index = array.lower + i64::from(offset);
                let signal = array.element(index).expect("validated shared array");
                self.bind_local_identity(local, signal, Some(index));
                self.builder.push(
                    block,
                    CfgValueType::Effect,
                    CfgValueKind::DigitalBlockingWrite {
                        target: DigitalWriteTarget {
                            signal,
                            select: DigitalWriteSelect::Whole,
                        },
                        value: initial,
                    },
                );
            }
            self.locals[usize::from(local)].array = Some((VectorBounds { msb, lsb }, array));
            self.locals[usize::from(local)].shared = Some(array.base);
            return;
        }
        let Some(base) = u32::try_from(self.signals.len()).ok() else {
            self.error(
                "process-local array signal IDs exceed supported storage",
                dimension.span,
            );
            return;
        };
        let array = DigitalArrayRef {
            base: DigitalSignalId::new(base),
            lower: msb.min(lsb),
            len,
        };
        if array.cell_range().is_none() {
            self.error(
                "process-local arrays support at most 65536 elements with representable indices",
                dimension.span,
            );
            return;
        }
        let initial = self.read_local(block, local);
        let declaration = &self.locals[usize::from(local)];
        let name = declaration.name.clone().expect("source local array");
        let process = self
            .process
            .expect("array declarations belong to a process");
        let mut storage_name = format!("$local:{process}:{local}:{name}");
        loop {
            let prefix = format!("{storage_name}[");
            if !self
                .signals
                .iter()
                .any(|signal| signal.name == storage_name || signal.name.starts_with(&prefix))
            {
                break;
            }
            storage_name.insert(0, '$');
        }
        for offset in 0..len {
            let index = array.lower + i64::from(offset);
            let signal = DigitalSignalId::new(base + offset);
            self.signals.push(DigitalSignal {
                local: Some(DigitalLocalStorage {
                    process,
                    declaration: local,
                    name: name.clone(),
                    element: Some(index),
                }),
                initial_value: None,
                id: signal,
                name: format!("{storage_name}[{index}]").into(),
                kind: if declaration.real {
                    DigitalSignalKind::Real(DigitalRealResolution::Single)
                } else {
                    DigitalSignalKind::FourState
                },
                width: declaration.width,
                bounds: declaration
                    .packed
                    .then_some((declaration.bounds.msb, declaration.bounds.lsb)),
                signed: declaration.signed,
                integer: declaration.integer,
                procedurally_assignable: true,
                span: declaration.span.into(),
            });
            self.builder.push(
                block,
                CfgValueType::Effect,
                CfgValueKind::DigitalBlockingWrite {
                    target: DigitalWriteTarget {
                        signal,
                        select: DigitalWriteSelect::Whole,
                    },
                    value: initial,
                },
            );
        }
        let bounds = VectorBounds { msb, lsb };
        self.local_arrays.push(DigitalArray {
            name: storage_name.into(),
            bounds: (msb, lsb),
            storage: array,
        });
        self.locals[usize::from(local)].array = Some((bounds, array));
        // Shared marks this declaration as stored across waits; it is never
        // carried as a scalar resume argument or promoted a second time.
        self.locals[usize::from(local)].shared = Some(array.base);
    }

    pub(super) fn initialize_local_array(
        &mut self,
        block: BlockId,
        local: DigitalLocalId,
        expression: &Expression,
    ) {
        let (bounds, array) = self.locals[usize::from(local)].array.expect("local array");
        let Expression::ArrayLiteral(literal) = expression else {
            self.error(
                "process-local array initializer requires an array literal",
                expression.span(),
            );
            return;
        };
        if literal.first_replication().is_some() {
            self.error(
                "replicated array initialization requires element-pattern expansion",
                expression.span(),
            );
            return;
        }
        if literal.elements.len() != array.len as usize {
            self.error(
                format!(
                    "process-local array initializer requires {} elements, found {}",
                    array.len,
                    literal.elements.len()
                ),
                expression.span(),
            );
            return;
        }
        let real = self.local_is_real(local);
        let width = self.local_width(local);
        // Evaluate the complete pattern before publishing it. Bounds preserve
        // authored order even though the element store uses increasing indices.
        let values: Vec<_> = literal
            .elements
            .iter()
            .map(|element| {
                let ArrayLiteralElement::Value(value) = element else {
                    unreachable!("replication rejected")
                };
                if real {
                    self.real_expression(block, value)
                } else {
                    let value = self.assigned_value(block, value, width);
                    self.resize(block, value, width, false)
                }
            })
            .collect();
        for (offset, value) in values.into_iter().enumerate() {
            let index = if bounds.msb <= bounds.lsb {
                bounds.msb + offset as i64
            } else {
                bounds.msb - offset as i64
            };
            let signal = array
                .element(index)
                .expect("initializer index in declared bounds");
            self.builder.push(
                block,
                CfgValueType::Effect,
                CfgValueKind::DigitalBlockingWrite {
                    target: DigitalWriteTarget {
                        signal,
                        select: DigitalWriteSelect::Whole,
                    },
                    value,
                },
            );
        }
    }

    pub(super) fn local_read_dependencies(&self, local: DigitalLocalId) -> Vec<DigitalSignalId> {
        let declaration = &self.locals[usize::from(local)];
        if let Some((_, array)) = declaration.array {
            array
                .cell_range()
                .expect("validated local array")
                .map(DigitalSignalId::new)
                .collect()
        } else {
            declaration.shared.into_iter().collect()
        }
    }
}
