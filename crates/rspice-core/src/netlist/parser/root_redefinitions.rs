//! One value for a root `.PARAM` that the deck defines more than once.
//!
//! The parser binds a top-level reader to the value its parameter holds when
//! the reader's card is read, while subcircuit bodies and retained
//! expressions bind after the whole deck has been read. Under the default
//! [`UseLast`] policy a redefinition further down the deck would therefore
//! reach only some of the readers. ngspice deletes every earlier definition
//! of a duplicated name before it evaluates anything (`inp_sort_params`,
//! "mark earlier one to skip"), so every reader sees the last definition; a
//! deck that appends `.param W=...` to change a design variable relies on it.
//!
//! The first pass records each value a redefinition superseded. When one of
//! them differs from the name's final value, the deck is parsed again with
//! that final value pinned at every definition except the last, which still
//! evaluates its own expression. A last definition that reads another
//! redefined name can move under the pins, so passes repeat until no
//! superseded value differs.
//!
//! [`UseLast`]: crate::netlist::ParameterRedefinitionPolicy::UseLast

use super::temperature::TemperaturePassError;
use crate::Value;
use crate::netlist::expr::{ComplexValue, ParamContext};
use crate::netlist::{Netlist, ParseError};
use std::collections::HashMap;

/// One discovery pass plus the pinned passes a chain of redefinitions that
/// read one another can need before it settles.
const MAX_PASSES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RootBinding {
    name: String,
    global: bool,
}

impl RootBinding {
    fn value(&self, params: &ParamContext) -> Option<ComplexValue> {
        if self.global {
            params.get_global_complex(&self.name)
        } else {
            params.get_complex(&self.name)
        }
    }

    fn real_value(&self, params: &ParamContext) -> Option<Value> {
        self.value(params)
            .filter(|value| value.im == 0.0)
            .map(|value| value.re)
    }
}

#[derive(Debug, Clone, Copy)]
struct Pin {
    value: Value,
    definitions: usize,
}

/// Root redefinitions observed in one pass, and the pins that pass applies.
#[derive(Debug, Default)]
pub(super) struct RootRedefinitions {
    pins: HashMap<RootBinding, Pin>,
    definitions: HashMap<RootBinding, usize>,
    superseded: Vec<(RootBinding, ComplexValue)>,
}

impl RootRedefinitions {
    /// A record for the next pass that applies this one's pins.
    pub(super) fn begin_pass(&self) -> Self {
        Self {
            pins: self.pins.clone(),
            ..Self::default()
        }
    }

    /// Record an authoritative root definition of `name` before it binds, and
    /// return the value it must bind in place of its own expression.
    pub(super) fn define(
        &mut self,
        name: &str,
        global: bool,
        params: &ParamContext,
    ) -> Option<Value> {
        let binding = RootBinding {
            name: name.to_ascii_uppercase(),
            global,
        };
        let ordinal = {
            let count = self.definitions.entry(binding.clone()).or_default();
            *count += 1;
            *count
        };
        let pin = self
            .pins
            .get(&binding)
            .filter(|pin| ordinal < pin.definitions)
            .map(|pin| pin.value);
        if ordinal > 1
            && let Some(current) = binding.value(params)
        {
            self.superseded.push((binding, current));
        }
        pin
    }

    /// The pins the next pass needs and the first name that needs one, or
    /// `None` when every superseded value already equals its final value.
    fn next_pass(&self, params: &ParamContext) -> Option<(Self, String)> {
        let stale = self
            .superseded
            .iter()
            .filter(|(binding, superseded)| {
                binding
                    .real_value(params)
                    .is_some_and(|last| ComplexValue::from(last) != *superseded)
            })
            .map(|(binding, _)| binding)
            .collect::<Vec<_>>();
        let first = stale.first()?.name.clone();
        let pins = self
            .pins
            .keys()
            .chain(stale)
            .filter_map(|binding| {
                let pin = Pin {
                    value: binding.real_value(params)?,
                    definitions: *self.definitions.get(binding)?,
                };
                Some((binding.clone(), pin))
            })
            .collect();
        Some((
            Self {
                pins,
                ..Self::default()
            },
            first,
        ))
    }
}

/// Parse until every reader of a redefined root parameter sees its last
/// definition.
pub(super) fn parse_with_settled_root_redefinitions<T>(
    mut parse: impl FnMut(&mut RootRedefinitions) -> Result<(Netlist, T), TemperaturePassError>,
) -> Result<(Netlist, T), TemperaturePassError> {
    let mut redefinitions = RootRedefinitions::default();
    let mut unsettled = String::new();
    for _ in 0..MAX_PASSES {
        let (netlist, carried) = parse(&mut redefinitions)?;
        match redefinitions.next_pass(&netlist.params) {
            Some((next, name)) => {
                redefinitions = next;
                unsettled = name;
            }
            None => return Ok((netlist, carried)),
        }
    }
    Err(ParseError::InvalidValue(format!(
        "Parameter {unsettled} does not settle: its last .PARAM definition reads a value that its own redefinition changes; remove the circular redefinition"
    ))
    .into())
}
