//! Cheap native-card preflight, exact grid resolution and authenticated QPXF execution.
use super::*;
use crate::ResourceLimits;
use crate::analysis::quasi_periodic::{QuasiPeriodicLinearConfig, QuasiPeriodicLinearMethod};
use crate::netlist::{QpacSweep, QpxfCard, QpxfCardLattices, QpxfCardOutput, QpxfCardSources};

fn request_fields(card: &QpxfCard) -> Result<QpxfRequest, SimulationError> {
    let frequency_axis = match card
        .frequency_axis
        .as_deref()
        .unwrap_or("OUTPUT")
        .to_ascii_uppercase()
        .as_str()
    {
        "OUTPUT" => QpxfFrequencyAxis::Output,
        "OFFSET" => QpxfFrequencyAxis::Offset,
        _ => return Err(qpxf_error("AXIS must be OUTPUT or OFFSET")),
    };
    let mut linear = QuasiPeriodicLinearConfig::default();
    linear.method = match card
        .linear_solver
        .as_deref()
        .unwrap_or("AUTO")
        .to_ascii_uppercase()
        .as_str()
    {
        "AUTO" => QuasiPeriodicLinearMethod::Auto,
        "DIRECT" => QuasiPeriodicLinearMethod::Direct,
        "KRYLOV" => QuasiPeriodicLinearMethod::Krylov,
        _ => return Err(qpxf_error("SOLVER must be AUTO, DIRECT or KRYLOV")),
    };
    if let Some(v) = card.krylov_restart {
        linear.restart = v;
    }
    if let Some(v) = card.krylov_cycles {
        linear.max_cycles = v;
    }
    if let Some(v) = card.linear_tolerance {
        linear.relative_tolerance = v;
    }
    let request = QpxfRequest {
        frequencies_hz: vec![0.0],
        frequency_axis,
        linear,
        output_lattice: card.output_lattice.clone(),
        input_sources: match &card.input_sources {
            QpxfCardSources::AllIndependent => QpxfSources::AllIndependent,
            QpxfCardSources::Named(names) => QpxfSources::Named(names.clone()),
        },
        input_lattices: match &card.input_lattices {
            QpxfCardLattices::AllRetained => QpxfInputLattices::AllRetained,
            QpxfCardLattices::Explicit(tuples) => QpxfInputLattices::Explicit(tuples.clone()),
            QpxfCardLattices::MaxOrders(orders) => QpxfInputLattices::MaxOrders(orders.clone()),
        },
        output: match &card.output {
            QpxfCardOutput::Voltage { positive, negative } => QpxfOutput::Voltage {
                positive: positive.clone(),
                negative: negative.clone(),
            },
            QpxfCardOutput::BranchCurrent { branch } => QpxfOutput::BranchCurrent {
                branch: branch.clone(),
            },
        },
        group_delay: card.group_delay.unwrap_or(false),
        group_delay_magnitude_floor: card.group_delay_magnitude_floor.unwrap_or(0.0),
    };
    request.validate()?;
    Ok(request)
}
impl QpxfRequest {
    /// Bound the sweep and validate every option without materializing generated
    /// frequencies, for per-frame editor validation and front-end preflight.
    pub fn validate_qpxf_card(
        card: &QpxfCard,
        limits: &ResourceLimits,
    ) -> Result<usize, SimulationError> {
        request_fields(card)?;
        super::super::frequency_sweep::validate(&card.sweep, limits)
    }

    pub fn from_qpxf_card(card: &QpxfCard) -> Result<Self, SimulationError> {
        Self::from_qpxf_card_with_abort(card, &ResourceLimits::default(), &NoAbort)
    }
    pub fn from_qpxf_card_with_abort(
        card: &QpxfCard,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        check_abort(abort)?;
        Self::validate_qpxf_card(card, limits)?;
        let frequencies_hz = super::super::frequency_sweep::resolve(&card.sweep, limits, abort)?;
        let mut request = request_fields(card)?;
        request.frequencies_hz = frequencies_hz;
        request.validate()?;
        check_abort(abort)?;
        Ok(request)
    }
    /// Render the resolved request without rounding or inventing source scaling.
    pub fn to_spice(&self) -> Result<String, SimulationError> {
        self.validate()?;
        Ok(QpxfCard {
            sweep: QpacSweep::Explicit(self.frequencies_hz.clone()),
            frequency_axis: Some(
                match self.frequency_axis {
                    QpxfFrequencyAxis::Output => "OUTPUT",
                    QpxfFrequencyAxis::Offset => "OFFSET",
                }
                .into(),
            ),
            input_sources: match &self.input_sources {
                QpxfSources::AllIndependent => QpxfCardSources::AllIndependent,
                QpxfSources::Named(names) => QpxfCardSources::Named(names.clone()),
            },
            input_lattices: match &self.input_lattices {
                QpxfInputLattices::AllRetained => QpxfCardLattices::AllRetained,
                QpxfInputLattices::Explicit(tuples) => QpxfCardLattices::Explicit(tuples.clone()),
                QpxfInputLattices::MaxOrders(orders) => QpxfCardLattices::MaxOrders(orders.clone()),
            },
            output: match &self.output {
                QpxfOutput::Voltage { positive, negative } => QpxfCardOutput::Voltage {
                    positive: positive.clone(),
                    negative: negative.clone(),
                },
                QpxfOutput::BranchCurrent { branch } => QpxfCardOutput::BranchCurrent {
                    branch: branch.clone(),
                },
            },
            output_lattice: self.output_lattice.clone(),
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
            group_delay: Some(self.group_delay),
            group_delay_magnitude_floor: Some(self.group_delay_magnitude_floor),
        }
        .to_spice())
    }
}
impl Engine {
    pub fn run_qpxf_card_from_qpss_with_abort(
        &self,
        netlist: &Netlist,
        card: &QpxfCard,
        point: &QpssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<QpxfAnalysisResult, SimulationError> {
        check_abort(abort)?;
        let engine = self.resolved_for_netlist(netlist);
        let request =
            QpxfRequest::from_qpxf_card_with_abort(card, &engine.config.resource_limits, abort)?;
        engine.run_qpxf_from_qpss_with_abort(netlist, request, point, abort)
    }
}
#[cfg(test)]
mod tests;
