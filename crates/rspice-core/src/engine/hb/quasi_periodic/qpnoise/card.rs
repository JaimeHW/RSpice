//! Native QPNOISE options resolve to the same authenticated engine request.
use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
use crate::netlist::{
    QpacSweep, QpnoiseCard, QpnoiseCardLattices, QpnoiseCardOutput, QpnoiseCardSources,
    QpxfCardOutput,
};
fn fields(card: &QpnoiseCard) -> Result<QpnoiseRequest, SimulationError> {
    let choice = |value: &Option<String>, default: &str| {
        value.as_deref().unwrap_or(default).to_ascii_uppercase()
    };
    let frequency_axis = match choice(&card.frequency_axis, "OUTPUT").as_str() {
        "OUTPUT" => QpnoiseFrequencyAxis::Output,
        "OFFSET" => QpnoiseFrequencyAxis::Offset,
        _ => return Err(qpnoise_error("AXIS must be OUTPUT or OFFSET")),
    };
    let mut linear = QuasiPeriodicLinearConfig::default();
    linear.method = match choice(&card.linear_solver, "AUTO").as_str() {
        "AUTO" => QuasiPeriodicLinearMethod::Auto,
        "DIRECT" => QuasiPeriodicLinearMethod::Direct,
        "KRYLOV" => QuasiPeriodicLinearMethod::Krylov,
        _ => return Err(qpnoise_error("SOLVER must be AUTO, DIRECT or KRYLOV")),
    };
    if let Some(value) = card.krylov_restart {
        linear.restart = value;
    }
    if let Some(value) = card.krylov_cycles {
        linear.max_cycles = value;
    }
    if let Some(value) = card.linear_tolerance {
        linear.relative_tolerance = value;
    }
    let input = match (&card.input_source, &card.input_lattice) {
        (None, None) => None,
        (Some(source), Some(lattice)) => Some(QpnoiseInput {
            source: source.clone(),
            lattice: lattice.clone(),
        }),
        _ => {
            return Err(qpnoise_error(
                "SOURCE and INLATTICE must be supplied together for input referral",
            ));
        }
    };
    let integration = if card.integrated_noise.unwrap_or(true) {
        Some(QpnoiseIntegration {
            band_hz: card.integration_band,
            method: match choice(&card.integration_method, "LINEAR").as_str() {
                "LINEAR" => QpnoiseIntegrationMethod::Linear,
                "LOGLOG" => QpnoiseIntegrationMethod::LogLog,
                _ => return Err(qpnoise_error("INTEGRATION must be LINEAR or LOGLOG")),
            },
        })
    } else {
        if card.integration_band.is_some() || card.integration_method.is_some() {
            return Err(qpnoise_error("BAND and INTEGRATION require INTEGRATED=YES"));
        }
        None
    };
    let noise_figure = if card.noise_figure.unwrap_or(false) {
        Some(QpnoiseNoiseFigure {
            source_resistor: card
                .source_resistor
                .clone()
                .ok_or_else(|| qpnoise_error("NOISEFIGURE requires SOURCERESISTOR"))?,
            reference_temperature: card.reference_temperature.unwrap_or(290.0),
            reference_lattices: card.reference_lattices.clone(),
        })
    } else {
        if card.source_resistor.is_some()
            || card.reference_temperature.is_some()
            || card.reference_lattices.is_some()
        {
            return Err(qpnoise_error(
                "source resistor, temperature and reference tuples require NOISEFIGURE=YES",
            ));
        }
        None
    };
    let request = QpnoiseRequest {
        frequencies_hz: vec![0.0],
        frequency_axis,
        input,
        integration,
        noise_figure,
        linear,
        contributor_ranking: card.contributor_ranking.unwrap_or(true),
        outputs: card
            .outputs
            .iter()
            .map(|o| QpnoiseOutput {
                observation: match &o.observation {
                    QpxfCardOutput::Voltage { positive, negative } => QpnoiseObservation::Voltage {
                        positive: positive.clone(),
                        negative: negative.clone(),
                    },
                    QpxfCardOutput::BranchCurrent { branch } => QpnoiseObservation::BranchCurrent {
                        branch: branch.clone(),
                    },
                },
                lattice: o.lattice.clone(),
            })
            .collect(),
        input_lattices: match &card.noise_lattices {
            QpnoiseCardLattices::AllRetained => QpnoiseLattices::AllRetained,
            QpnoiseCardLattices::Explicit(tuples) => QpnoiseLattices::Explicit {
                tuples: tuples.clone(),
            },
            QpnoiseCardLattices::MaxOrders(orders) => QpnoiseLattices::MaxOrders {
                orders: orders.clone(),
            },
            QpnoiseCardLattices::Range { minimum, maximum } => QpnoiseLattices::Range {
                minimum: minimum.clone(),
                maximum: maximum.clone(),
            },
        },
        sources: match &card.noise_sources {
            QpnoiseCardSources::All => QpnoiseSources::All,
            QpnoiseCardSources::Only(names) => QpnoiseSources::Only(names.clone()),
            QpnoiseCardSources::Except(names) => QpnoiseSources::Except(names.clone()),
        },
    };
    request.validate()?;
    Ok(request)
}
impl QpnoiseRequest {
    pub fn validate_qpnoise_card(
        card: &QpnoiseCard,
        limits: &ResourceLimits,
    ) -> Result<usize, SimulationError> {
        fields(card)?;
        let count = super::super::frequency_sweep::validate(&card.sweep, limits)?;
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            count.saturating_mul(card.outputs.len()),
            limits.max_analysis_points,
        )?;
        Ok(count)
    }
    pub fn from_qpnoise_card(card: &QpnoiseCard) -> Result<Self, SimulationError> {
        Self::from_qpnoise_card_with_abort(card, &ResourceLimits::default(), &NoAbort)
    }
    pub fn from_qpnoise_card_with_abort(
        card: &QpnoiseCard,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        check_abort(abort)?;
        Self::validate_qpnoise_card(card, limits)?;
        let mut request = fields(card)?;
        request.frequencies_hz =
            super::super::frequency_sweep::resolve(&card.sweep, limits, abort)?;
        request.validate()?;
        Ok(request)
    }
    /// Preserve all resolved options and exact authored frequency coordinates.
    pub fn to_spice(&self) -> Result<String, SimulationError> {
        self.validate()?;
        Ok(self.to_qpnoise_card().to_spice())
    }
    pub fn to_qpnoise_card(&self) -> QpnoiseCard {
        QpnoiseCard {
            sweep: QpacSweep::Explicit(self.frequencies_hz.clone()),
            frequency_axis: Some(
                match self.frequency_axis {
                    QpnoiseFrequencyAxis::Output => "OUTPUT",
                    QpnoiseFrequencyAxis::Offset => "OFFSET",
                }
                .into(),
            ),
            outputs: self
                .outputs
                .iter()
                .map(|o| QpnoiseCardOutput {
                    observation: match &o.observation {
                        QpnoiseObservation::Voltage { positive, negative } => {
                            QpxfCardOutput::Voltage {
                                positive: positive.clone(),
                                negative: negative.clone(),
                            }
                        }
                        QpnoiseObservation::BranchCurrent { branch } => {
                            QpxfCardOutput::BranchCurrent {
                                branch: branch.clone(),
                            }
                        }
                    },
                    lattice: o.lattice.clone(),
                })
                .collect(),
            input_source: self.input.as_ref().map(|i| i.source.clone()),
            input_lattice: self.input.as_ref().map(|i| i.lattice.clone()),
            noise_lattices: match &self.input_lattices {
                QpnoiseLattices::AllRetained => QpnoiseCardLattices::AllRetained,
                QpnoiseLattices::Explicit { tuples } => {
                    QpnoiseCardLattices::Explicit(tuples.clone())
                }
                QpnoiseLattices::MaxOrders { orders } => {
                    QpnoiseCardLattices::MaxOrders(orders.clone())
                }
                QpnoiseLattices::Range { minimum, maximum } => QpnoiseCardLattices::Range {
                    minimum: minimum.clone(),
                    maximum: maximum.clone(),
                },
            },
            noise_sources: match &self.sources {
                QpnoiseSources::All => QpnoiseCardSources::All,
                QpnoiseSources::Only(names) => QpnoiseCardSources::Only(names.clone()),
                QpnoiseSources::Except(names) => QpnoiseCardSources::Except(names.clone()),
            },
            integrated_noise: Some(self.integration.is_some()),
            integration_band: self.integration.as_ref().and_then(|i| i.band_hz),
            integration_method: self.integration.as_ref().map(|i| {
                match i.method {
                    QpnoiseIntegrationMethod::Linear => "LINEAR",
                    QpnoiseIntegrationMethod::LogLog => "LOGLOG",
                }
                .into()
            }),
            contributor_ranking: Some(self.contributor_ranking),
            noise_figure: Some(self.noise_figure.is_some()),
            source_resistor: self
                .noise_figure
                .as_ref()
                .map(|f| f.source_resistor.clone()),
            reference_temperature: self.noise_figure.as_ref().map(|f| f.reference_temperature),
            reference_lattices: self
                .noise_figure
                .as_ref()
                .and_then(|f| f.reference_lattices.clone()),
            linear_solver: Some(
                match self.linear.method {
                    QuasiPeriodicLinearMethod::Auto => "AUTO",
                    QuasiPeriodicLinearMethod::Direct => "DIRECT",
                    QuasiPeriodicLinearMethod::Krylov => "KRYLOV",
                }
                .into(),
            ),
            krylov_restart: Some(self.linear.restart),
            krylov_cycles: Some(self.linear.max_cycles),
            linear_tolerance: Some(self.linear.relative_tolerance),
        }
    }
}
impl Engine {
    pub fn run_qpnoise_card_from_qpss_with_abort(
        &self,
        netlist: &Netlist,
        card: &QpnoiseCard,
        point: &QpssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<QpnoiseAnalysisResult, SimulationError> {
        check_abort(abort)?;
        let engine = self.resolved_for_netlist(netlist);
        let request = QpnoiseRequest::from_qpnoise_card_with_abort(
            card,
            &engine.config.resource_limits,
            abort,
        )?;
        engine.run_qpnoise_from_qpss_with_abort(netlist, request, point, abort)
    }
}
#[cfg(test)]
mod tests;
