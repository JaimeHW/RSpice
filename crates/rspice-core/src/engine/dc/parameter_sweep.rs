//! A `.DC` sweep of a design parameter that reaches nothing but independent
//! sources' values.
//!
//! Xyce sets a swept parameter and pushes it into the instances that read it
//! (N_ANP_SweepParam.C, `updateCurrentVal` then `setParam`), and Spectre's
//! `dc param=` sweep does the same; neither reads the deck again. Re-reading
//! the deck at every point is the general answer here, because a parameter
//! can reach anything the elaboration decides: a model card, a subcircuit
//! instance's parameters, an element count. When the elaboration of the
//! first point, observed through every parameter scope it reads, shows the
//! parameter reaching only root independent sources, every later point's
//! deck is the first point's with those sources read again from their own
//! cards, and the circuit that point builds is the one the re-read deck
//! would build.

use super::super::{Engine, SimulationError};
use crate::abort_signal::AbortSignal;
use crate::netlist::{ElementKind, ParamContext, ParameterReach, ParameterReachSite, SourceSpec};
use crate::{Netlist, Value};

/// A root independent source the parameter reaches: where it sits in the
/// deck and the card text and scope its specification is read from.
struct ReachedSource {
    element: usize,
    text: String,
    scope: ParamContext,
    current: bool,
}

/// How each point of a `.DC` parameter sweep after the first gets its deck:
/// from the first point's, with the swept binding and the specifications of
/// the sources that read it set to the point's value.
pub(super) struct DcParameterSweepPlan {
    base: Netlist,
    parameter: String,
    global: bool,
    sources: Vec<ReachedSource>,
    moved: Vec<String>,
}

impl DcParameterSweepPlan {
    /// Whether every statement that read the parameter is a root card that
    /// read it into one independent source and added nothing else.
    fn reaches_only_sources(reach: &ParameterReach) -> bool {
        reach.is_complete()
            && reach.sites.iter().all(|site| {
                matches!(
                    site,
                    ParameterReachSite::Card {
                        in_subcircuit: false,
                        elements,
                        sources,
                        ..
                    } if elements.len() == 1
                        && sources.len() == 1
                        && sources[0].element.eq_ignore_ascii_case(&elements[0])
                )
            })
    }

    /// The plan for `deck`, `netlist` replayed with the parameter at
    /// `value`, given what that replay and its build were seen to read;
    /// `None` unless the parameter provably reaches nothing but root
    /// independent sources' specifications.
    fn new(
        netlist: &Netlist,
        deck: &Netlist,
        parameter: &str,
        value: Value,
        reach: ParameterReach,
    ) -> Option<Self> {
        if !Self::reaches_only_sources(&reach) {
            return None;
        }
        let parameter = parameter.to_ascii_uppercase();
        let global = !netlist.params.has_parameter_binding(&parameter)
            && netlist.params.has_any_parameter_binding(&parameter);
        let mut sources = Vec::new();
        for site in reach.sites {
            let ParameterReachSite::Card {
                in_subcircuit: false,
                elements,
                sources: specs,
                ..
            } = site
            else {
                return None;
            };
            let ([element], [spec]) = (elements.as_slice(), specs.as_slice()) else {
                return None;
            };
            let mut named = deck
                .elements
                .iter()
                .enumerate()
                .filter(|(_, candidate)| candidate.name.eq_ignore_ascii_case(element));
            let (Some((index, found)), None) = (named.next(), named.next()) else {
                return None;
            };
            let current = match &found.kind {
                ElementKind::VoltageSource(_) => false,
                ElementKind::CurrentSource(_) => true,
                _ => return None,
            };
            sources.push(ReachedSource {
                element: index,
                text: spec.text.clone(),
                scope: spec.scope.clone(),
                current,
            });
        }
        // A retained device override is applied over the replayed deck,
        // so it, not the card, decides a source it names.
        let overlay = deck.ast_overlay();
        if sources.iter().any(|source| {
            let name = &deck.elements[source.element].name;
            overlay
                .device_parameters
                .keys()
                .any(|(device, _)| device.eq_ignore_ascii_case(name))
        }) {
            return None;
        }
        let moved = sources
            .iter()
            .map(|source| deck.elements[source.element].name.clone())
            .collect();
        let plan = Self {
            base: deck.clone(),
            parameter,
            global,
            sources,
            moved,
        };
        // Read again at the observed value, every source must come back as
        // the deck read it (as cloned, so without the ended observation).
        for source in &plan.sources {
            let reread = plan.read(source, value).ok()?;
            let (ElementKind::VoltageSource(read) | ElementKind::CurrentSource(read)) =
                &plan.base.elements[source.element].kind
            else {
                return None;
            };
            if format!("{reread:?}") != format!("{read:?}") {
                return None;
            }
        }
        Some(plan)
    }

    fn bind(&self, scope: &mut ParamContext, value: Value) {
        if self.global {
            scope.set_global(&self.parameter, value);
        } else {
            scope.set(&self.parameter, value);
        }
    }

    fn read(&self, source: &ReachedSource, value: Value) -> Result<SourceSpec, SimulationError> {
        let mut scope = source.scope.clone();
        self.bind(&mut scope, value);
        crate::netlist::parse_source_spec_text(&source.text, 0, &scope).map_err(|error| {
            SimulationError::Netlist(format!(
                "Failed to read '{}' for parameter {} = {value}: {error}",
                self.base.elements[source.element].name, self.parameter
            ))
        })
    }

    /// The deck at `value`.
    pub(super) fn point(&self, value: Value) -> Result<Netlist, SimulationError> {
        let mut deck = self.base.clone();
        self.bind(&mut deck.params, value);
        for source in &self.sources {
            let spec = self.read(source, value)?;
            if let ElementKind::VoltageSource(read) | ElementKind::CurrentSource(read) =
                &mut deck.elements[source.element].kind
            {
                *read = spec;
            }
        }
        let mut parameters = deck.ast_overlay().parameters.clone();
        parameters.insert(self.parameter.clone(), value);
        deck.retain_overlay_parameters(parameters);
        Ok(deck)
    }

    /// The root sources each point moves.
    pub(super) fn moved(&self) -> &[String] {
        &self.moved
    }

    /// Whether a point moves a current source, whose value enters the
    /// node equations every residual is assembled from; a voltage source's
    /// enters only its own branch equation, identically in each.
    pub(super) fn moves_current_source(&self) -> bool {
        self.sources.iter().any(|source| source.current)
    }
}

impl Engine {
    /// The deck a `.DC` sweep of `parameter` solves at its first point,
    /// `value`, the number of bindings the parameter has there, and the plan
    /// that gives every later point's deck without reading the deck again,
    /// when the parameter's reach allows one.
    pub(super) fn dc_parameter_sweep_start(
        &self,
        netlist: &Netlist,
        parameter: &str,
        value: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, usize, Option<DcParameterSweepPlan>), SimulationError> {
        let (mut deck, bindings) = Self::replay_parameter_reach(
            netlist,
            parameter,
            value,
            self.config.resource_limits,
            abort,
        )?;
        // A card that draws reads the stream at its own position in the
        // deck, which the card read again on its own would not reproduce.
        let plannable = deck.params.parse_statistical_draws() == 0
            && deck
                .params
                .parameter_reach()
                .is_some_and(|reach| DcParameterSweepPlan::reaches_only_sources(&reach));
        // The flattener and the builder read the deck's scopes too: a
        // plannable point's circuit is built under the same observation.
        let built = plannable
            && self
                .resolved_for_netlist(&deck)
                .build_dc_mode_circuit_with_abort(&deck, abort)
                .is_ok();
        let Some(reach) = deck.params.end_parameter_reach() else {
            return Ok((deck, bindings, None));
        };
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let plan = if built {
            DcParameterSweepPlan::new(netlist, &deck, parameter, value, reach)
        } else {
            None
        };
        Ok((deck, bindings, plan))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    fn first_point(deck: &str, parameter: &str) -> (Netlist, Option<DcParameterSweepPlan>) {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let (_, _, plan) = Engine::default()
            .dc_parameter_sweep_start(&netlist, parameter, 0.0, &NoAbort)
            .expect("the first point reads");
        (netlist, plan)
    }

    fn element(deck: &Netlist, name: &str) -> String {
        deck.elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .map(|element| format!("{:?}", element.kind))
            .expect("element is in the deck")
    }

    #[test]
    fn a_parameter_that_sets_only_a_source_reads_each_point_from_the_first() {
        let deck = "\
bias sweep
.param vin=1
V1 in 0 DC {vin} AC 1 SIN(0 {vin/10} 1k)
R1 in out 1k
D1 out 0 dmod
.model dmod d(is=1e-14)
.dc vin 0 2 0.5
.end
";
        let (netlist, plan) = first_point(deck, "VIN");
        let plan = plan.expect("the parameter reaches only V1");
        assert_eq!(plan.moved(), ["V1"]);
        for value in [0.5, 1.25, 2.0] {
            let derived = plan.point(value).expect("the point reads");
            let (replayed, _) = Engine::create_perturbed_netlist_with_limits_and_abort(
                &netlist,
                "VIN",
                value,
                crate::resource::ResourceLimits::default(),
                &NoAbort,
            )
            .expect("the deck replays");
            assert_eq!(element(&derived, "V1"), element(&replayed, "V1"));
            assert_eq!(element(&derived, "R1"), element(&replayed, "R1"));
            assert_eq!(derived.params.get("VIN"), Some(value));
            assert_eq!(derived.ast_overlay().parameters.get("VIN"), Some(&value));
        }
    }

    #[test]
    fn a_parameter_that_reaches_more_than_sources_is_read_from_the_deck() {
        for (reached, cards) in [
            ("a resistor", "R2 out 0 {vin*1k}"),
            (
                "a parameter",
                ".param half={vin/2}\nV2 x 0 {half}\nR3 x 0 1k",
            ),
            (
                "a model card",
                ".model dmod2 d(is={vin*1e-14})\nD2 out 0 dmod2",
            ),
            (
                "a subcircuit instance",
                ".subckt load a r=1\nR1 a 0 {r}\n.ends\nX1 out load r={vin*1k}",
            ),
            (
                "a source in a subcircuit",
                ".subckt bias a\nV1 a 0 {vin}\n.ends\nX2 b bias\nR4 b 0 1k",
            ),
        ] {
            let deck = format!(
                "bias sweep\n.param vin=1\nV1 in 0 {{vin}}\nR1 in out 1k\n{cards}\n.dc vin 0 2 0.5\n.end\n"
            );
            let (_, plan) = first_point(&deck, "VIN");
            assert!(plan.is_none(), "the parameter reaches {reached}");
        }
    }
}
