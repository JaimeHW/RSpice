//! Strict optional sampling controls for `.PNOISE`.

use super::*;
use crate::analysis::pnoise::{
    PeriodicNoiseEdge, PeriodicNoiseEdgeDirection, PeriodicNoiseSampling,
};

#[derive(Default)]
pub(super) struct SamplingFields {
    mode: Option<String>,
    phase: Option<Value>,
    edge: EdgeFields,
    reference: Option<(String, Option<String>)>,
    reference_edge: EdgeFields,
    periods: Option<u32>,
}

#[derive(Default)]
struct EdgeFields {
    threshold: Option<Value>,
    direction: Option<PeriodicNoiseEdgeDirection>,
    occurrence: Option<usize>,
    tolerance: Option<Value>,
    minimum_slew: Option<Value>,
}

impl EdgeFields {
    fn is_present(&self) -> bool {
        self.threshold.is_some()
            || self.direction.is_some()
            || self.occurrence.is_some()
            || self.tolerance.is_some()
            || self.minimum_slew.is_some()
    }

    fn build(self) -> PeriodicNoiseEdge {
        let defaults = PeriodicNoiseEdge::default();
        PeriodicNoiseEdge {
            threshold_volts: self.threshold.unwrap_or(defaults.threshold_volts),
            direction: self.direction.unwrap_or(defaults.direction),
            occurrence: self.occurrence.unwrap_or(defaults.occurrence),
            phase_tolerance_degrees: self.tolerance.unwrap_or(defaults.phase_tolerance_degrees),
            minimum_slew_volts_per_second: self
                .minimum_slew
                .unwrap_or(defaults.minimum_slew_volts_per_second),
        }
    }
}

impl SamplingFields {
    /// Return false without consuming tokens when this is not a sampling key.
    pub(super) fn read(
        &mut self,
        keyword: &str,
        stream: &mut TokenStream,
        line: usize,
        params: &ParamContext,
    ) -> Result<bool, ParseError> {
        const CARD: AnalysisCard = AnalysisCard::Pnoise;
        match keyword {
            "SAMPLING" => {
                let value = card_name(stream, line, CARD, "SAMPLING")?.to_ascii_uppercase();
                if !matches!(value.as_str(), "PHASE" | "EDGE" | "DELAY") {
                    return Err(card_error(
                        CARD,
                        line,
                        AnalysisCardIssue::InvalidChoice {
                            field: "SAMPLING",
                            value,
                            expected: "PHASE, EDGE or DELAY",
                        },
                    ));
                }
                bind_once(&mut self.mode, value, CARD, line, "SAMPLING")?;
            }
            "SAMPLEPHASE" => bind_once(
                &mut self.phase,
                card_number(
                    stream,
                    line,
                    params,
                    CARD,
                    "SAMPLEPHASE",
                    "a finite phase in degrees",
                    |_| true,
                )?,
                CARD,
                line,
                "SAMPLEPHASE",
            )?,
            "REFOUT" => bind_once(
                &mut self.reference,
                card_output_probe(stream, line, CARD)?,
                CARD,
                line,
                "REFOUT",
            )?,
            "PERIODS" => bind_once(
                &mut self.periods,
                card_number(
                    stream,
                    line,
                    params,
                    CARD,
                    "PERIODS",
                    "a whole number in 0..4294967295",
                    |n| n >= 0.0 && n <= u32::MAX as Value && n.fract() == 0.0,
                )? as u32,
                CARD,
                line,
                "PERIODS",
            )?,
            _ => {
                let (edge, field) = match keyword {
                    "THRESHOLD" => (&mut self.edge, "THRESHOLD"),
                    "DIRECTION" => (&mut self.edge, "DIRECTION"),
                    "OCCURRENCE" => (&mut self.edge, "OCCURRENCE"),
                    "PHASETOL" => (&mut self.edge, "PHASETOL"),
                    "MINSLEW" => (&mut self.edge, "MINSLEW"),
                    "REFTHRESHOLD" => (&mut self.reference_edge, "REFTHRESHOLD"),
                    "REFDIRECTION" => (&mut self.reference_edge, "REFDIRECTION"),
                    "REFOCCURRENCE" => (&mut self.reference_edge, "REFOCCURRENCE"),
                    "REFPHASETOL" => (&mut self.reference_edge, "REFPHASETOL"),
                    "REFMINSLEW" => (&mut self.reference_edge, "REFMINSLEW"),
                    _ => return Ok(false),
                };
                match field.trim_start_matches("REF") {
                    "THRESHOLD" => bind_once(
                        &mut edge.threshold,
                        card_number(
                            stream,
                            line,
                            params,
                            CARD,
                            field,
                            "a finite threshold in volts",
                            |_| true,
                        )?,
                        CARD,
                        line,
                        field,
                    )?,
                    "DIRECTION" => {
                        let value = card_name(stream, line, CARD, field)?.to_ascii_uppercase();
                        let direction = match value.as_str() {
                            "RISING" => PeriodicNoiseEdgeDirection::Rising,
                            "FALLING" => PeriodicNoiseEdgeDirection::Falling,
                            "EITHER" => PeriodicNoiseEdgeDirection::Either,
                            _ => {
                                return Err(card_error(
                                    CARD,
                                    line,
                                    AnalysisCardIssue::InvalidChoice {
                                        field,
                                        value,
                                        expected: "RISING, FALLING or EITHER",
                                    },
                                ));
                            }
                        };
                        bind_once(&mut edge.direction, direction, CARD, line, field)?;
                    }
                    "OCCURRENCE" => bind_once(
                        &mut edge.occurrence,
                        card_count(stream, line, params, CARD, field, 1)?,
                        CARD,
                        line,
                        field,
                    )?,
                    "PHASETOL" => bind_once(
                        &mut edge.tolerance,
                        card_number(
                            stream,
                            line,
                            params,
                            CARD,
                            field,
                            "a phase tolerance in 1e-12..1 degree",
                            |v| (1e-12..=1.0).contains(&v),
                        )?,
                        CARD,
                        line,
                        field,
                    )?,
                    "MINSLEW" => bind_once(
                        &mut edge.minimum_slew,
                        card_number(
                            stream,
                            line,
                            params,
                            CARD,
                            field,
                            "a nonnegative slew in V/s",
                            |v| v >= 0.0,
                        )?,
                        CARD,
                        line,
                        field,
                    )?,
                    _ => unreachable!(),
                }
            }
        }
        Ok(true)
    }

    pub(super) fn build(self, line: usize) -> Result<Option<PeriodicNoiseSampling>, ParseError> {
        let conflict = |field| {
            card_error(
                AnalysisCard::Pnoise,
                line,
                AnalysisCardIssue::ConflictingFields {
                    first: "SAMPLING",
                    second: field,
                },
            )
        };
        let delay_fields =
            self.reference.is_some() || self.reference_edge.is_present() || self.periods.is_some();
        if self.mode.as_deref() != Some("DELAY") && delay_fields {
            return Err(conflict("delay sampling fields"));
        }
        if !matches!(self.mode.as_deref(), Some("EDGE" | "DELAY")) && self.edge.is_present() {
            return Err(conflict("edge sampling fields"));
        }
        if self.mode.as_deref() != Some("PHASE") && self.phase.is_some() {
            return Err(conflict("SAMPLEPHASE"));
        }
        Ok(match self.mode.as_deref() {
            None => None,
            Some("PHASE") => Some(PeriodicNoiseSampling::Phase {
                phase_degrees: self.phase.unwrap_or(0.0),
            }),
            Some("EDGE") => Some(PeriodicNoiseSampling::Edge {
                edge: self.edge.build(),
            }),
            Some("DELAY") => {
                let (node, reference) = self.reference.ok_or_else(|| {
                    card_error(
                        AnalysisCard::Pnoise,
                        line,
                        AnalysisCardIssue::MissingField { field: "REFOUT" },
                    )
                })?;
                Some(PeriodicNoiseSampling::Delay {
                    edge: self.edge.build(),
                    reference_node: node.to_ascii_uppercase(),
                    reference_ref: reference.map(|node| node.to_ascii_uppercase()),
                    reference_edge: self.reference_edge.build(),
                    periods: self.periods.unwrap_or(0),
                })
            }
            Some(_) => unreachable!(),
        })
    }
}
