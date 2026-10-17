//! One binding for a root `.PARAM` or parameter function that the deck
//! defines more than once.
//!
//! The parser binds a top-level reader to the value its parameter holds when
//! the reader's card is read, while subcircuit bodies and retained
//! expressions bind after the whole deck has been read. Under the default
//! [`UseLast`] policy a redefinition further down the deck would therefore
//! reach only some of the readers. ngspice deletes every earlier definition
//! of a duplicated name before it evaluates anything (`inp_sort_params`,
//! "mark earlier one to skip"), and a redefined `.FUNC` body likewise
//! reaches every call, so every reader sees the last definition; a deck that
//! appends `.param W=...` to change a design variable relies on it.
//!
//! The first pass records each binding a redefinition superseded. When one
//! of them differs from the name's final binding, the deck is parsed again
//! with that final binding pinned at every definition except the one
//! [`binding_definition`] selects (the last), which still evaluates its own
//! expression. A last definition that reads another redefined name can move
//! under the pins, so passes repeat until no superseded binding differs. Each
//! pass settles at least one more link of a chain that does not read itself,
//! so a deck still unsettled after one pass per redefined name, beyond the
//! first, has a redefinition that feeds back into itself, and is refused.
//!
//! [`UseLast`]: crate::netlist::ParameterRedefinitionPolicy::UseLast
//! [`binding_definition`]: crate::netlist::ParameterRedefinitionPolicy::binding_definition

use super::temperature::TemperaturePassError;
use crate::netlist::expr::{
    CapturedStatisticalParameter, ComplexDirection, ComplexValue, ExprError, ParamContext,
};
use crate::netlist::{Netlist, ParseError};
use std::collections::HashMap;
use std::sync::Arc;

/// The namespace a root definition binds in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum RootDefinitionKind {
    /// `.PARAM`, `.PARAMS` or `.CSPARAM`.
    Parameter,
    /// `.GLOBAL_PARAM`.
    GlobalParameter,
    /// `.FUNC`, or `.PARAM name(args)=...`.
    Function,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RootBinding {
    name: String,
    kind: RootDefinitionKind,
}

#[derive(Debug, Clone, PartialEq)]
enum Bound {
    Number(ComplexValue),
    Text(String),
    Function { args: Vec<String>, body: String },
}

impl RootBinding {
    fn bound(&self, params: &ParamContext) -> Option<Bound> {
        let number = match self.kind {
            RootDefinitionKind::Parameter => params.get_complex(&self.name),
            RootDefinitionKind::GlobalParameter => params.get_global_complex(&self.name),
            RootDefinitionKind::Function => {
                return params
                    .get_function(&self.name)
                    .map(|function| Bound::Function {
                        args: function.args.clone(),
                        body: function.body.clone(),
                    });
            }
        };
        number.map(Bound::Number).or_else(|| {
            params
                .get_string(&self.name)
                .map(|text| Bound::Text(text.to_owned()))
        })
    }
}

/// The last definition of a name, bound at every earlier definition of it.
#[derive(Debug, Clone)]
pub(super) struct Pin {
    bound: Bound,
    direction: Result<ComplexDirection, ExprError>,
    capture: Option<Arc<CapturedStatisticalParameter>>,
    definitions: usize,
}

impl Pin {
    /// Replace what an earlier definition bound, after its own expression has
    /// been read and any random draws it makes consumed, with the last one.
    pub(super) fn bind(self, name: &str, kind: RootDefinitionKind, params: &mut ParamContext) {
        let global = kind == RootDefinitionKind::GlobalParameter;
        match self.bound {
            Bound::Number(value) if global => params.set_global_complex(name, value),
            Bound::Number(value) => params.set_complex(name, value),
            Bound::Text(text) if global => params.set_global_string(name, text),
            Bound::Text(text) => params.set_string(name, text),
            Bound::Function { args, body } => {
                params.define_function(name, args, &body);
                return;
            }
        }
        params.retain_parameter_direction(name, global, Some(self.direction));
        params.retain_statistical_parameter_capture(name, self.capture);
    }
}

/// Root redefinitions observed in one pass, and the pins that pass applies.
#[derive(Debug, Default)]
pub(super) struct RootRedefinitions {
    pins: HashMap<RootBinding, Pin>,
    definitions: HashMap<RootBinding, usize>,
    superseded: Vec<(RootBinding, Bound)>,
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
    /// return the pin to bind once its own expression has been read.
    pub(super) fn define(
        &mut self,
        name: &str,
        kind: RootDefinitionKind,
        params: &ParamContext,
    ) -> Option<Pin> {
        let binding = RootBinding {
            name: name.to_ascii_uppercase(),
            kind,
        };
        let ordinal = {
            let count = self.definitions.entry(binding.clone()).or_default();
            *count += 1;
            *count - 1
        };
        let policy = params.parameter_redefinition_policy();
        let pin = self
            .pins
            .get(&binding)
            .filter(|pin| ordinal != policy.binding_definition(pin.definitions))
            .cloned();
        if ordinal > 0
            && let Some(current) = binding.bound(params)
        {
            self.superseded.push((binding, current));
        }
        pin
    }

    /// The pins the next pass needs and the first name that needs one, or
    /// `None` when every superseded binding already equals its final one.
    fn next_pass(&self, params: &ParamContext) -> Option<(Self, String)> {
        let stale = self
            .superseded
            .iter()
            .filter(|(binding, superseded)| {
                binding
                    .bound(params)
                    .is_some_and(|last| last != *superseded)
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
                    bound: binding.bound(params)?,
                    direction: params.parameter_direction(&binding.name),
                    capture: (binding.kind == RootDefinitionKind::Parameter)
                        .then(|| params.statistical_parameter_capture(&binding.name))
                        .flatten(),
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

    /// The names this pass saw defined more than once.
    fn redefined_names(&self) -> usize {
        self.definitions
            .values()
            .filter(|&&count| count > 1)
            .count()
    }
}

/// Parse until every reader of a redefined root parameter or function sees
/// its last definition.
///
/// The first pass discovers the redefinitions, and each pinned pass after it
/// settles at least one more redefined name of a chain that does not read
/// itself, so a deck is refused only when one pass per redefined name has not
/// settled it.
pub(super) fn parse_with_settled_root_redefinitions<T>(
    mut parse: impl FnMut(&mut RootRedefinitions) -> Result<(Netlist, T), TemperaturePassError>,
) -> Result<(Netlist, T), TemperaturePassError> {
    let mut redefinitions = RootRedefinitions::default();
    let mut pinned_passes = 0;
    loop {
        let (netlist, carried) = parse(&mut redefinitions)?;
        let Some((next, unsettled)) = redefinitions.next_pass(&netlist.params) else {
            return Ok((netlist, carried));
        };
        if pinned_passes == redefinitions.redefined_names() {
            return Err(ParseError::InvalidValue(format!(
                "Parameter {unsettled} does not settle: its last .PARAM definition reads a value that its own redefinition changes; remove the circular redefinition"
            ))
            .into());
        }
        pinned_passes += 1;
        redefinitions = next;
    }
}
