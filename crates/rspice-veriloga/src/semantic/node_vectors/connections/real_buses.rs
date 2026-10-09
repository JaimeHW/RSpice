//! Expand a real bus word or part in authored lane order, without copy drivers.
use super::*;
use crate::ast::PackedSelect;

impl ConnectionScope {
    pub(super) fn append_real_bus(
        &self,
        actual: &Expression,
        output: &mut Vec<Expression>,
    ) -> CompileResult<bool> {
        let name = match actual {
            Expression::Identifier(id) => &id.name,
            Expression::ArrayAccess(access) => &access.array,
            Expression::Digital(DigitalExpr::PartSelect(select)) => &select.name,
            Expression::Digital(DigitalExpr::ArraySelect(select)) => &select.name,
            _ => return Ok(false),
        };
        let Some(bus) = self.real_buses.get(name) else {
            return Ok(false);
        };
        let rank = bus.dimensions.len() - 1;
        let invalid = || {
            error(
                format!(
                    "real bus '{name}' requires {rank} array coordinates before its bus selection"
                ),
                actual.span(),
            )
        };
        let (indices, selected) = match actual {
            Expression::Identifier(_) => (Vec::new(), None),
            Expression::ArrayAccess(access) if rank == 0 => (
                Vec::new(),
                Some((access.index.as_ref(), access.index.as_ref())),
            ),
            Expression::ArrayAccess(access) => (vec![access.index.as_ref()], None),
            Expression::Digital(DigitalExpr::PartSelect(select)) => {
                (Vec::new(), Some((select.msb.as_ref(), select.lsb.as_ref())))
            }
            Expression::Digital(DigitalExpr::ArraySelect(select)) => {
                let (indices, selected) = select.split(rank).ok_or_else(invalid)?;
                let selected = selected.map(|select| match select {
                    PackedSelect::Bit(index) => (index.as_ref(), index.as_ref()),
                    PackedSelect::Part { msb, lsb } => (msb.as_ref(), lsb.as_ref()),
                });
                (indices, selected)
            }
            _ => unreachable!("recognized bus connection"),
        };
        if indices.len() != rank {
            return Err(invalid());
        }
        // Dynamic scalar input reads remain ordinary expressions in the parent
        // scope. A static topology connection must resolve all coordinates.
        let Ok(mut indices) = indices
            .into_iter()
            .map(|index| self.coordinate(index))
            .collect::<CompileResult<Vec<_>>>()
        else {
            return Ok(false);
        };
        let range = match selected {
            Some((msb, lsb)) => {
                let (Ok(msb), Ok(lsb)) = (self.coordinate(msb), self.coordinate(lsb)) else {
                    return Ok(false);
                };
                VectorBounds { msb, lsb }
            }
            None => bus.bounds,
        };
        if !bus.bounds.contains(range.msb)
            || !bus.bounds.contains(range.lsb)
            || (range.msb != range.lsb
                && (range.msb > range.lsb) != (bus.bounds.msb > bus.bounds.lsb))
        {
            return Err(error(
                "real bus selection requires in-range bounds with the declared direction",
                actual.span(),
            ));
        }
        if output.len().saturating_add(range.width() as usize) > MAX_DIGITAL_VECTOR_WIDTH as usize {
            return Err(error(
                "real bus connection exceeds the lane limit",
                actual.span(),
            ));
        }
        indices.push(range.msb);
        for coordinate in range.indices_msb_first() {
            *indices.last_mut().unwrap() = coordinate;
            output.push(Expression::Identifier(Identifier {
                name: bus.lane(&indices, actual.span())?,
                span: actual.span(),
            }));
        }
        Ok(true)
    }
}
