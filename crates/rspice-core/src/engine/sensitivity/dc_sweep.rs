//! Xyce's `.SENS` study: the DC sensitivities of each objective at every
//! point of the deck's `.DC` sweep, or at its operating point when it has
//! none (N_NLS_Sensitivity.C, run once per converged DC step).

use super::super::dc::{DcSweepPoint, DcSweepResult, DcSweepVariable};
use super::{
    AcSensitivityLocation, AcSensitivityTarget, DcSensitivityDeck, DcSensitivityPointDeck, Engine,
    ResidualProbeCache, SimulationError,
};
use crate::abort_signal::AbortSignal;
use crate::analysis::sensitivity::{AcSensitivityOutput, ElementType, SensitivityResult};
use crate::netlist::{AnalysisCommand, DcSecondSweep, DcSweepSpec};
use crate::solver::{SimulationResult, StaticMatrix};
use crate::{CircuitData, Netlist, Value};

/// One point of a Xyce `.SENS` study.
#[derive(Debug, Clone)]
pub struct DcSweepSensitivityPoint {
    /// The primary `.DC` sweep value; zero at an operating point.
    pub sweep_value: Value,
    /// The operating point every derivative was taken at.
    pub result: SimulationResult,
    /// One result per objective, in card order. Each row's `vector_name` is
    /// the parameter as the card names it, in the card's order; a pattern
    /// that selects several parameters names each `OWNER:PARAMETER`.
    pub objectives: Vec<SensitivityResult>,
}

/// One `.SENS` card: its objective, its printed columns and the filters
/// that select them.
type XyceObjective = (AcSensitivityOutput, Vec<SensitivityColumn>, Vec<String>);

/// What a refusal at one sweep point does to a study read from the sweep.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PointRefusal {
    /// The sweep stops with it: the study is all the sweep is run for.
    StopsTheSweep,
    /// The study ends with it and the sweep finishes: the sweep is also the
    /// deck's `.DC` analysis.
    EndsTheStudy,
}

/// One printed derivative column and the target it reads.
struct SensitivityColumn {
    name: String,
    element: String,
    element_type: ElementType,
    parameter: String,
}

impl SensitivityColumn {
    fn of(name: String, target: &AcSensitivityTarget) -> Self {
        Self {
            name,
            element: target.element.clone(),
            element_type: target.element_type,
            parameter: target.parameter.clone(),
        }
    }

    /// The filter that selects exactly this column's target.
    fn filter(&self) -> String {
        if self.element_type == ElementType::DesignParameter {
            format!("PARAM:{}", self.element)
        } else {
            format!("{}:{}", self.element, self.parameter)
        }
    }
}

impl Engine {
    /// Run a Xyce `.SENS` study: each card is one `OBJFUNC` objective, and
    /// its filters are the card's `PARAM` list.
    ///
    /// The derivatives are taken at every point of the deck's first `.DC`
    /// card, the second source outermost, or at the operating point when the
    /// deck has no `.DC` card. A parameter is named as Xyce names it: an
    /// independent source's `DCV0` is its DC value, a bare name is a design
    /// parameter, and a BJT model parameter answers to each of its aliases.
    /// A parameter the owner does not author (a model default such as `NF`)
    /// has no sensitivity target and its column is left out with a warning;
    /// an owner the circuit does not have is refused by name.
    pub fn run_sensitivity_dc_sweep_with_abort(
        &self,
        netlist: &Netlist,
        cards: &[&AnalysisCommand],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepSensitivityPoint>, SimulationError> {
        let objectives = self.xyce_sensitivity_objectives(netlist, cards, abort)?;
        let sweep = netlist.analyses.iter().find_map(|analysis| match analysis {
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode,
                sweep2,
                restart,
            } => Some((
                source,
                DcSweepSpec {
                    start: *start,
                    stop: *stop,
                    step: *step,
                    mode: mode.clone(),
                    restart: *restart,
                },
                sweep2.as_ref(),
            )),
            _ => None,
        });
        match sweep {
            Some((source, spec, sweep2)) => {
                let (_, points) = self.dc_sensitivity_sweep(
                    netlist,
                    source,
                    &spec,
                    sweep2,
                    &objectives,
                    PointRefusal::StopsTheSweep,
                    abort,
                )?;
                points
            }
            None => {
                let (outputs, filters) = Self::xyce_objective_inputs(&objectives);
                let (studies, result) =
                    self.dc_sensitivity_operating_point(netlist, &outputs, &filters, abort)?;
                Ok(vec![Self::xyce_sensitivity_point(
                    0.0,
                    result,
                    studies,
                    &objectives,
                )?])
            }
        }
    }

    /// The `.DC` analysis [`Self::run_dc_analysis_with_abort`] runs, and the
    /// deck's Xyce `.SENS` study read from that same sweep, as Xyce takes its
    /// sensitivities at each DC step it solves: one sweep for both. The
    /// caller names the deck's first `.DC` card, the one the study sweeps.
    ///
    /// The analysis is the sweep's own result whatever the study does: a
    /// study the deck refuses, or that a point refuses, leaves the analysis
    /// to finish and is returned as its own error, exactly the error
    /// [`Self::run_sensitivity_dc_sweep_with_abort`] reports. Only a stopped
    /// run stops both.
    #[allow(clippy::type_complexity)]
    pub fn run_dc_analysis_with_xyce_sensitivity_and_abort(
        &self,
        netlist: &Netlist,
        source: &str,
        primary: &DcSweepSpec,
        outer: Option<&DcSecondSweep>,
        cards: &[&AnalysisCommand],
        abort: &dyn AbortSignal,
    ) -> Result<
        (
            DcSweepResult,
            Result<Vec<DcSweepSensitivityPoint>, SimulationError>,
        ),
        SimulationError,
    > {
        let objectives = match self.xyce_sensitivity_objectives(netlist, cards, abort) {
            Ok(objectives) => objectives,
            Err(error) if Self::stops_the_run(&error, abort) => return Err(error),
            Err(error) => {
                let result =
                    self.run_dc_analysis_with_abort(netlist, source, primary, outer, abort)?;
                return Ok((result, Err(error)));
            }
        };
        self.dc_sensitivity_sweep(
            netlist,
            source,
            primary,
            outer,
            &objectives,
            PointRefusal::EndsTheStudy,
            abort,
        )
    }

    /// Each card's objective, its printed columns and the filters that
    /// select them, refused as the study refuses a card.
    fn xyce_sensitivity_objectives(
        &self,
        netlist: &Netlist,
        cards: &[&AnalysisCommand],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<XyceObjective>, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        let flat = engine.flattened_sensitivity_netlist(netlist, abort)?;
        let candidates = Self::collect_ac_sensitivity_targets(
            &flat,
            engine.config.spice_dialect,
            engine.config.resource_limits,
        )?
        .into_iter()
        .filter(Self::dc_sensitivity_target_active)
        .chain(Self::collect_design_parameter_targets(netlist))
        .collect::<Vec<_>>();
        let resolver = super::super::NodeResolver::build_with_abort(&engine, netlist, abort)?;
        let mut objectives = Vec::with_capacity(cards.len());
        for card in cards {
            let AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                output_is_current,
                filters,
                ac_sweep: None,
            } = card
            else {
                return Err(SimulationError::Netlist(
                    "a Xyce .SENS study was given a card that is not a DC .SENS objective"
                        .to_owned(),
                ));
            };
            if output_node.starts_with('{') {
                return Err(SimulationError::Circuit(format!(
                    ".SENS objective {output_node} is an expression; a DC sensitivity objective \
                     is one node voltage V(a) or V(a,b), or one branch current I(x)"
                )));
            }
            let output = if *output_is_current {
                AcSensitivityOutput::BranchCurrent(output_node.clone())
            } else {
                AcSensitivityOutput::Voltage {
                    positive: resolver.resolve(output_node, ".SENS objective")?,
                    negative: resolver
                        .resolve_reference(reference_node.as_deref(), ".SENS objective")?,
                }
            };
            let columns = Self::xyce_sensitivity_columns(&flat, &candidates, filters)?;
            let mut selected = columns
                .iter()
                .map(SensitivityColumn::filter)
                .collect::<Vec<_>>();
            selected.sort();
            selected.dedup();
            objectives.push((output, columns, selected));
        }

        Ok(objectives)
    }

    fn xyce_objective_inputs(
        objectives: &[XyceObjective],
    ) -> (Vec<AcSensitivityOutput>, Vec<&[String]>) {
        objectives
            .iter()
            .map(|(output, _, selected)| (output.clone(), selected.as_slice()))
            .unzip()
    }

    /// One printed point: each objective's rows as its columns name them.
    fn xyce_sensitivity_point(
        sweep_value: Value,
        result: SimulationResult,
        studies: Vec<SensitivityResult>,
        objectives: &[XyceObjective],
    ) -> Result<DcSweepSensitivityPoint, SimulationError> {
        Ok(DcSweepSensitivityPoint {
            sweep_value,
            result,
            objectives: studies
                .into_iter()
                .zip(objectives)
                .map(|(study, (_, columns, _))| Self::xyce_sensitivity_rows(study, columns))
                .collect::<Result<_, _>>()?,
        })
    }

    /// Whether `error` stops the run itself rather than refusing a study.
    fn stops_the_run(error: &SimulationError, abort: &dyn AbortSignal) -> bool {
        abort.is_aborted()
            || matches!(
                error,
                SimulationError::Aborted
                    | SimulationError::TimeLimitExceeded
                    | SimulationError::ModelFinished(_)
            )
    }

    /// Each objective's rows at every point of a `.DC` sweep, solved exactly
    /// as the `.DC` analysis solves them: one sweep, each point continuing
    /// from the last, and every row read at the point it converged to, as
    /// Xyce takes its sensitivities at each successful DC step
    /// (N_ANP_DCSweep.C, `DCSweep::doProcessSuccessfulStep`). Returns the
    /// sweep's analysis and, apart, the study's own outcome.
    ///
    /// A point's rows are read from the tangents of [`DcSensitivityDeck`],
    /// built for each deck the sweep builds a circuit from. A row that
    /// re-solves its point (a design parameter, a linear deck's passive
    /// value, a row no tangent reaches) rebuilds the point's deck as the
    /// sweep defines it.
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    fn dc_sensitivity_sweep(
        &self,
        netlist: &Netlist,
        source: &str,
        primary: &DcSweepSpec,
        sweep2: Option<&DcSecondSweep>,
        objectives: &[XyceObjective],
        refusal: PointRefusal,
        abort: &dyn AbortSignal,
    ) -> Result<
        (
            DcSweepResult,
            Result<Vec<DcSweepSensitivityPoint>, SimulationError>,
        ),
        SimulationError,
    > {
        let (outputs, filters) = Self::xyce_objective_inputs(objectives);
        let mut decks = Vec::new();
        let mut cache = ResidualProbeCache::default();
        let mut tangents = Vec::new();
        let mut refused = None;
        let poll = |_: usize| {
            if abort.is_aborted() {
                Err(SimulationError::Aborted)
            } else {
                Ok(())
            }
        };
        let mut linearize = |point: DcSweepPoint<'_>,
                             circuit: &mut CircuitData,
                             matrix: &mut StaticMatrix,
                             solution: &[Value]|
         -> Result<(), SimulationError> {
            if refused.is_some() {
                return Ok(());
            }
            let mut study = || -> Result<(), SimulationError> {
                if point.rebuilt || decks.is_empty() {
                    decks.push(
                        point
                            .engine
                            .dc_sensitivity_deck(point.deck, &filters, abort)?,
                    );
                    cache = ResidualProbeCache::default();
                }
                let index = decks.len() - 1;
                let deck: &DcSensitivityDeck = &decks[index];
                let point_tangents = if deck.has_tangent_rows() {
                    deck.point_tangents(
                        &mut cache,
                        point.engine,
                        circuit,
                        matrix,
                        solution,
                        &outputs,
                        &poll,
                        abort,
                    )?
                } else {
                    Vec::new()
                };
                tangents.push((index, point_tangents));
                Ok(())
            };
            match study() {
                Err(error)
                    if refusal == PointRefusal::EndsTheStudy
                        && !Self::stops_the_run(&error, abort) =>
                {
                    refused = Some(error);
                    Ok(())
                }
                outcome => outcome,
            }
        };
        let result = self.run_dc_analysis_with_linearization_and_abort(
            netlist,
            source,
            primary,
            sweep2,
            Some(&mut linearize),
            abort,
        )?;
        if let Some(error) = refused {
            return Ok((result, Err(error)));
        }
        let points = (|| -> Result<Vec<DcSweepSensitivityPoint>, SimulationError> {
            if tangents.len() != result.points.len() {
                return Err(SimulationError::Circuit(format!(
                    ".SENS sweep linearized {} of its {} point(s); a circuit with no unknowns \
                     has no DC sensitivities",
                    tangents.len(),
                    result.points.len()
                )));
            }
            let inner_variable = self.dc_sweep_variable(netlist, source);
            let outer_variable =
                sweep2.map(|sweep2| self.dc_sweep_variable(netlist, &sweep2.source));
            let inner_count = result
                .axes
                .last()
                .map_or(1, |axis| axis.values.len())
                .max(1);
            let swept_source = match &inner_variable {
                DcSweepVariable::Source(name) => Some(name.as_str()),
                _ => None,
            };
            result
                .points
                .iter()
                .zip(&tangents)
                .enumerate()
                .map(|(index, (point, (deck, point_tangents)))| {
                    let deck = &decks[*deck];
                    let sweep_value = point.sweep_value;
                    let outer_value = outer_variable
                        .as_ref()
                        .map(|variable| (variable, result.axes[0].values[index / inner_count]));
                    let inner = &inner_variable;
                    let build = move |abort: &dyn AbortSignal| {
                        let (outer_netlist, outer_engine) = match outer_value {
                            Some((variable, value)) => {
                                self.dc_sweep_point(netlist, variable, value, abort)?
                            }
                            None => (netlist.clone(), None),
                        };
                        let (swept, inner_engine) = outer_engine
                            .as_ref()
                            .unwrap_or(self)
                            .dc_sweep_point(&outer_netlist, inner, sweep_value, abort)?;
                        let engine = inner_engine.or(outer_engine);
                        let flat = engine
                            .as_ref()
                            .unwrap_or(self)
                            .flattened_sensitivity_netlist(&swept, abort)?;
                        Ok((engine, swept, flat))
                    };
                    let mut point_deck = DcSensitivityPointDeck::Swept {
                        base: self,
                        build: &build,
                        built: None,
                    };
                    let studies = outputs
                        .iter()
                        .enumerate()
                        .map(|(objective, output)| {
                            self.dc_sensitivity_objective(
                                deck,
                                objective,
                                point_tangents.get(objective),
                                &mut point_deck,
                                swept_source.map(|name| (name, sweep_value)),
                                &point.result,
                                output,
                                None,
                                abort,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    Self::xyce_sensitivity_point(
                        sweep_value,
                        point.result.clone(),
                        studies,
                        objectives,
                    )
                })
                .collect()
        })();
        Ok((result, points))
    }

    /// The printed columns of one objective's `PARAM` list, in card order.
    fn xyce_sensitivity_columns(
        flat: &Netlist,
        candidates: &[AcSensitivityTarget],
        parameters: &[String],
    ) -> Result<Vec<SensitivityColumn>, SimulationError> {
        let qualified = |target: &AcSensitivityTarget| match target.location {
            AcSensitivityLocation::DesignParameter { .. } => target.vector_name.clone(),
            _ => format!("{}:{}", target.element, target.parameter).to_ascii_uppercase(),
        };
        if parameters.is_empty() {
            return Ok(candidates
                .iter()
                .filter(|target| {
                    !matches!(
                        target.location,
                        AcSensitivityLocation::DesignParameter { .. }
                    )
                })
                .map(|target| SensitivityColumn::of(qualified(target), target))
                .collect());
        }
        let mut columns = Vec::new();
        for parameter in parameters {
            let parameter = parameter.trim().to_ascii_uppercase();
            let selected = Self::xyce_sensitivity_spellings(flat, &parameter)
                .iter()
                .find_map(|spelling| {
                    let selected = candidates
                        .iter()
                        .filter(|target| Self::sensitivity_filter_selects(target, spelling))
                        .collect::<Vec<_>>();
                    (!selected.is_empty()).then_some(selected)
                });
            match selected.as_deref() {
                Some([target]) => columns.push(SensitivityColumn::of(parameter, target)),
                Some(targets) => columns.extend(
                    targets
                        .iter()
                        .map(|target| SensitivityColumn::of(qualified(target), target)),
                ),
                None => {
                    let owner = parameter.rsplit_once(':').map(|(owner, _)| owner);
                    let owned = owner.is_some_and(|owner| {
                        flat.elements
                            .iter()
                            .any(|element| element.name.eq_ignore_ascii_case(owner))
                            || flat
                                .models
                                .iter()
                                .any(|model| model.name.eq_ignore_ascii_case(owner))
                    });
                    if !owned {
                        return Err(SimulationError::Circuit(format!(
                            ".SENS parameter {parameter} names no device, model or design \
                             parameter of this circuit"
                        )));
                    }
                    log::warn!(
                        "[sens.unavailable] .SENS parameter {parameter} is not an authored \
                         real parameter of its owner, so it has no DC sensitivity and its \
                         column is omitted"
                    );
                }
            }
        }
        Ok(columns)
    }

    /// The filter spellings that name one Xyce `.SENS` parameter, the
    /// card's own first.
    fn xyce_sensitivity_spellings(flat: &Netlist, parameter: &str) -> Vec<String> {
        let mut spellings = vec![parameter.to_owned()];
        let Some((owner, name)) = parameter.rsplit_once(':') else {
            spellings.push(format!("PARAM:{parameter}"));
            return spellings;
        };
        // Xyce names an independent source's DC value DCV0
        // (N_DEV_Vsrc.C, addPar("DCV0", ...)).
        if name == "DCV0" {
            spellings.push(format!("{owner}:DC"));
        }
        let bjt_model = flat.models.iter().any(|model| {
            model.name.eq_ignore_ascii_case(owner)
                && (model.model_type.eq_ignore_ascii_case("NPN")
                    || model.model_type.eq_ignore_ascii_case("PNP"))
        });
        if bjt_model
            && let Some((_, aliases)) = super::super::builder::BJT_MODEL_ALIAS_GROUPS
                .iter()
                .find(|(_, aliases)| aliases.contains(&name))
        {
            spellings.extend(
                aliases
                    .iter()
                    .filter(|alias| **alias != name)
                    .map(|alias| format!("{owner}:{alias}")),
            );
        }
        spellings
    }

    /// One objective's rows renamed and reordered into its printed columns.
    fn xyce_sensitivity_rows(
        mut study: SensitivityResult,
        columns: &[SensitivityColumn],
    ) -> Result<SensitivityResult, SimulationError> {
        let rows = std::mem::take(&mut study.sensitivities);
        study.sensitivities = columns
            .iter()
            .map(|column| {
                rows.iter()
                    .find(|row| {
                        row.element_type == column.element_type
                            && row.element.eq_ignore_ascii_case(&column.element)
                            && row.parameter.eq_ignore_ascii_case(&column.parameter)
                    })
                    .map(|row| {
                        let mut row = row.clone();
                        row.vector_name = column.name.clone();
                        row
                    })
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            ".SENS parameter {} produced no sensitivity row",
                            column.name
                        ))
                    })
            })
            .collect::<Result<_, _>>()?;
        Ok(study)
    }
}

#[cfg(test)]
mod tests {
    use super::DcSweepSensitivityPoint;
    use crate::abort_signal::NoAbort;
    use crate::config::{ExpressionDialect, SpiceDialect};
    use crate::engine::{Engine, SimulationConfig, SimulationError};
    use crate::netlist::{AnalysisCommand, NetlistParseOptions};
    use crate::{Netlist, Value};

    fn xyce(source: &str) -> Netlist {
        Netlist::parse_with_options(
            source,
            NetlistParseOptions {
                spice_dialect: Some(SpiceDialect::Xyce),
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce .SENS deck parses")
    }

    fn study(netlist: &Netlist) -> Result<Vec<DcSweepSensitivityPoint>, SimulationError> {
        let cards = netlist
            .analyses
            .iter()
            .filter(|card| matches!(card, AnalysisCommand::Sensitivity { .. }))
            .collect::<Vec<_>>();
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .run_sensitivity_dc_sweep_with_abort(netlist, &cards, &NoAbort)
    }

    /// A biased NPN stage swept by its supply. Xyce 7.10 prints its
    /// `.SENS.prn` with `_dir` and `_dir_scaled` columns per parameter. Its
    /// default Newton tolerances stop short of the converged tangent by up to
    /// 0.3% (dV(3)/dVCC = 2.92330360e-02 at VCC = 4); with `.options nonlin
    /// deltaxtol=1e-7 reltol=1e-7` it prints the values asserted here. `QN:NF`
    /// is a default the model does not author, which Xyce differentiates
    /// (N_DEV_BJT.C) but RSpice has no target for, so its column is omitted.
    #[test]
    fn a_xyce_sens_card_differentiates_each_dc_point_under_its_own_names() {
        let netlist = xyce(
            "* bias\nVCC 4 0 DC 5\nRB1 4 1 40k\nRB2 1 0 10k\nRE 2 0 1K\nRC 4 3 6K\n\
             Q1 3 1 2 QN\n.model QN NPN(BF=100 IS=1e-14 VA=100)\n.DC VCC 4 6 1\n\
             .print sens v(3)\n.SENS objfunc={V(3)} param=RB1:R,VCC:DCV0,\n+ QN:bf,QN:nf,QN:vaf\n\
             .options SENSITIVITY direct=1 adjoint=0 outputscaled=1\n.end\n",
        );
        assert_eq!(netlist.options.sensitivity_direct, Some(true));
        assert_eq!(netlist.options.sensitivity_adjoint, Some(false));
        assert_eq!(netlist.options.sensitivity_output_scaled, Some(true));
        assert_eq!(netlist.options.sensitivity_output_unscaled, None);
        let points = study(&netlist).expect("the .SENS study runs at every .DC point");
        let expected: [(Value, Value, [Value; 4]); 3] = [
            (
                4.0,
                2.94942660,
                [
                    7.78325069e-05,
                    2.92863074e-02,
                    -7.47183246e-04,
                    4.23571814e-05,
                ],
            ),
            (
                5.0,
                2.94299588,
                [
                    1.03465607e-04,
                    -3.05744488e-02,
                    -1.55502193e-03,
                    5.57304913e-05,
                ],
            ),
            (
                6.0,
                2.89970892,
                [
                    1.26996924e-04,
                    -5.31171967e-02,
                    -2.39858881e-03,
                    6.42604393e-05,
                ],
            ),
        ];
        assert_eq!(points.len(), expected.len());
        let close = |actual: Value, expected: Value, what: &str| {
            assert!(
                (actual - expected).abs() <= 1e-4 * expected.abs(),
                "{what}: expected {expected:e}, got {actual:e}"
            );
        };
        for (point, (sweep, output, derivatives)) in points.iter().zip(expected) {
            assert_eq!(point.sweep_value, sweep);
            let [objective] = point.objectives.as_slice() else {
                panic!("one objective, got {}", point.objectives.len());
            };
            close(objective.output_value, output, "V(3)");
            close(
                point.result.try_voltage_named("3").unwrap(),
                output,
                ".PRINT SENS V(3)",
            );
            let names = objective
                .sensitivities
                .iter()
                .map(|row| row.vector_name.as_str())
                .collect::<Vec<_>>();
            assert_eq!(names, ["RB1:R", "VCC:DCV0", "QN:BF", "QN:VAF"]);
            for (row, derivative) in objective.sensitivities.iter().zip(derivatives) {
                close(row.absolute, derivative, &row.vector_name);
            }
            // The swept supply is the source's own DC value at each point, so
            // Xyce's scaled column, dO/dp * p / 100, reads it there.
            assert_eq!(objective.sensitivities[1].nominal_value, sweep);
        }
    }

    #[test]
    fn a_xyce_sens_card_refuses_an_unknown_owner_and_an_expression_objective() {
        let divider = "* divider\nV1 1 0 DC 3\nR1 1 2 2k\nR2 2 0 1k\n.op\n";
        let unknown = xyce(&format!(
            "{divider}.SENS objfunc={{V(2)}} param=RX:R\n.end\n"
        ));
        let error = study(&unknown).unwrap_err().to_string();
        assert!(
            error.contains("RX:R") && error.contains("names no device"),
            "{error}"
        );
        let expression = xyce(&format!(
            "{divider}.SENS objfunc={{V(2)*V(1)}} param=R1:R\n.end\n"
        ));
        let error = study(&expression).unwrap_err().to_string();
        assert!(
            error.contains("{V(2)*V(1)}") && error.contains("expression"),
            "{error}"
        );
    }
}
