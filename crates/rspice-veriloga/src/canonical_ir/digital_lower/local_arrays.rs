//! Static local arrays use the same element store and operations as module arrays.
use super::super::digital::{DigitalArray, DigitalArrayRef, DigitalLocalStorage};
use super::*;
use crate::array_values::initializer_elements;

impl ProcessLowerer<'_> {
    pub(super) fn array_value_type(&self, name: &str) -> Option<crate::array_values::ArrayType> {
        use crate::array_values::{ArrayType, ElementType};
        if let Some(array) = self.array_declaration(name) {
            return Some(ArrayType {
                layout: array.layout().ok()?,
                element: if self.real_signal(array.storage.base) {
                    ElementType::Real
                } else {
                    ElementType::Integral {
                        width: self.width_of(array.storage.base),
                        signed: self.signed_signal(array.storage.base),
                    }
                },
            });
        }
        self.analog_array(name)?;
        let variable = &self.analog_variables[name];
        let element = match variable.quantity {
            super::super::digital::DigitalAnalogQuantity::RealVariable => ElementType::Real,
            super::super::digital::DigitalAnalogQuantity::IntegerVariable => {
                ElementType::Integral {
                    width: 32,
                    signed: true,
                }
            }
            _ => return None,
        };
        let layout = if variable.declared_dimensions.is_empty() {
            variable.array_layout()?
        } else {
            crate::array_index::UnpackedArrayLayout::new(&variable.declared_dimensions, 65_536)
                .ok()?
        };
        Some(ArrayType { layout, element })
    }

    pub(super) fn array_assignment(
        &mut self,
        block: BlockId,
        assign: &DigitalAssign,
    ) -> Option<Result<(Vec<DigitalLValue>, Vec<ValueId>), String>> {
        let DigitalLValue::Identifier { name, span } = &assign.target else {
            return None;
        };
        self.array_declaration(name)?;
        let array = self.array_value_type(name).expect("validated target array");
        let elements =
            match crate::array_values::assignment_elements(&assign.value, &array, |name| {
                self.array_value_type(name)
            }) {
                Ok(elements) => elements,
                Err(error) => return Some(Err(error)),
            };
        let mut targets = Vec::with_capacity(elements.len());
        let mut values = Vec::with_capacity(elements.len());
        for (ordinal, element) in elements.into_iter().enumerate() {
            let target = crate::array_values::digital_target(name, &array.layout, ordinal, *span);
            let value = if self.lvalue_is_real(&target) {
                self.real_expression(block, &element)
            } else {
                let width = self.lvalue_width(&target);
                self.assigned_value(block, &element, width)
            };
            targets.push(target);
            values.push(value);
        }
        Some(Ok((targets, values)))
    }

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
        let span = dimensions[0].span;
        let mut axes = Vec::with_capacity(dimensions.len());
        for dimension in dimensions {
            let mut reads = BTreeSet::new();
            collect_expression_reads(&dimension.start, &mut reads);
            collect_expression_reads(&dimension.end, &mut reads);
            let bounds = if reads.iter().any(|name| scope_names.contains(name)) {
                None
            } else {
                self.constant(&dimension.start)
                    .zip(self.constant(&dimension.end))
            };
            let Some(bounds) = bounds else {
                self.error(
                    "process-local array bounds must be constant signed 64-bit integers",
                    dimension.span,
                );
                return;
            };
            axes.push(bounds);
        }
        let Ok(layout) = crate::array_index::UnpackedArrayLayout::new(&axes, 65536) else {
            self.error(
                "process-local arrays support at most 65536 elements with representable indices",
                span,
            );
            return;
        };
        let len = layout.len() as u32;
        let (msb, lsb) = if axes.len() == 1 {
            axes[0]
        } else {
            (0, i64::from(len) - 1)
        };
        let dimensions = if axes.len() > 1 { axes } else { Vec::new() };
        if let Some(signal) = self.analog_local_signal(local) {
            let Some(declared) = self.arrays.get(&signal) else {
                self.invariant("analog local array has no shared layout", span);
                return;
            };
            let array = declared.storage;
            if array.lower != msb.min(lsb) || array.len != len || declared.dimensions != dimensions
            {
                self.invariant("analog local array disagrees with its shared layout", span);
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
                span,
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
                "process-local array signal IDs exceed supported storage",
                span,
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
        let metadata = DigitalArray {
            dimensions,
            name: storage_name.into(),
            bounds: (msb, lsb),
            storage: array,
        };
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
                name: metadata
                    .element_name_with_layout(&layout, offset as usize)
                    .expect("local array element"),
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
        self.local_arrays.push(metadata);
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
        let (_, array) = self.locals[usize::from(local)].array.expect("local array");
        let declaration = self
            .local_arrays
            .iter()
            .find(|declared| declared.storage.base == array.base)
            .or_else(|| self.arrays.get(&array.base))
            .expect("local array metadata");
        let layout = declaration.layout().expect("validated local array");
        let elements = match initializer_elements(expression, &layout) {
            Ok(elements) => elements,
            Err(message) => {
                self.error(message, expression.span());
                return;
            }
        };
        let real = self.local_is_real(local);
        let width = self.local_width(local);
        // Evaluate the complete pattern before publishing it. Bounds preserve
        // authored order even though the element store uses increasing indices.
        let values: Vec<_> = elements
            .into_iter()
            .map(|value| {
                if real {
                    self.real_expression(block, value)
                } else {
                    let value = self.assigned_value(block, value, width);
                    self.resize(block, value, width, false)
                }
            })
            .collect();
        for (offset, value) in values.into_iter().enumerate() {
            let slot = layout
                .declaration_slot(offset)
                .expect("initializer element");
            let signal = DigitalSignalId::from(usize::from(array.base) + slot);
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
