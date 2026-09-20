//! Native card resolution with frequency-grid preflight and lossless request authoring.
use super::*;
use crate::ResourceLimits;
use crate::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
use crate::netlist::{QpacCard, QpacSweep};

fn request_fields(card: &QpacCard) -> Result<QpacRequest, SimulationError> {
    let mut solver = QuasiPeriodicAcConfig::default();
    if let Some(method) = &card.linear_solver {
        solver.linear.method = match method.to_ascii_uppercase().as_str() {
            "AUTO" => QuasiPeriodicLinearMethod::Auto,
            "DIRECT" => QuasiPeriodicLinearMethod::Direct,
            "KRYLOV" => QuasiPeriodicLinearMethod::Krylov,
            _ => return Err(qpac_error("SOLVER must be AUTO, DIRECT or KRYLOV")),
        };
    }
    if let Some(v) = card.krylov_restart {
        solver.linear.restart = v;
    }
    if let Some(v) = card.krylov_cycles {
        solver.linear.max_cycles = v;
    }
    if let Some(v) = card.linear_tolerance {
        solver.linear.relative_tolerance = v;
    }
    if let Some(v) = card.current_absolute_tolerance {
        solver.current_absolute_tolerance = v;
    }
    if let Some(v) = card.voltage_absolute_tolerance {
        solver.voltage_absolute_tolerance = v;
    }
    let request = QpacRequest {
        offsets_hz: vec![0.0],
        input_source: card.input_source.clone(),
        input_lattice: card.input_lattice.clone(),
        output_node: card.output_node.clone(),
        output_ref: card.output_ref.clone(),
        output_lattice: card.output_lattice.clone(),
        magnitude: card.magnitude.unwrap_or(1.0),
        phase_degrees: card.phase_degrees.unwrap_or(0.0),
        solver,
    };
    request.validate()?;
    Ok(request)
}

impl QpacRequest {
    /// Validate authoring and bound the frequency count without allocating a
    /// generated sweep. Suitable for interactive configuration editors.
    pub fn validate_qpac_card(
        card: &QpacCard,
        limits: &ResourceLimits,
    ) -> Result<usize, SimulationError> {
        request_fields(card)?;
        super::super::frequency_sweep::validate(&card.sweep, limits)
    }

    pub fn from_qpac_card(card: &QpacCard) -> Result<Self, SimulationError> {
        Self::from_qpac_card_with_abort(card, &ResourceLimits::default(), &NoAbort)
    }

    pub fn from_qpac_card_with_abort(
        card: &QpacCard,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        check_abort(abort)?;
        Self::validate_qpac_card(card, limits)?;
        let offsets_hz = super::super::frequency_sweep::resolve(&card.sweep, limits, abort)?;
        let mut request = request_fields(card)?;
        request.offsets_hz = offsets_hz;
        request.validate()?;
        check_abort(abort)?;
        Ok(request)
    }

    /// An explicit-grid card preserves exactly the resolved request. A card
    /// authored with a generated sweep retains that spelling on QpacCard.
    pub fn to_spice(&self) -> Result<String, SimulationError> {
        self.validate()?;
        Ok(QpacCard {
            sweep: QpacSweep::Explicit(self.offsets_hz.clone()),
            input_source: self.input_source.clone(),
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            input_lattice: self.input_lattice.clone(),
            output_lattice: self.output_lattice.clone(),
            magnitude: Some(self.magnitude),
            phase_degrees: Some(self.phase_degrees),
            linear_solver: Some(
                match self.solver.linear.method {
                    QuasiPeriodicLinearMethod::Auto => "AUTO",
                    QuasiPeriodicLinearMethod::Direct => "DIRECT",
                    QuasiPeriodicLinearMethod::Krylov => "KRYLOV",
                }
                .into(),
            ),
            krylov_restart: Some(self.solver.linear.restart),
            krylov_cycles: Some(self.solver.linear.max_cycles),
            linear_tolerance: Some(self.solver.linear.relative_tolerance),
            current_absolute_tolerance: Some(self.solver.current_absolute_tolerance),
            voltage_absolute_tolerance: Some(self.solver.voltage_absolute_tolerance),
        }
        .to_spice())
    }
}

impl Engine {
    pub fn run_qpac_card_from_qpss_with_abort(
        &self,
        netlist: &Netlist,
        card: &QpacCard,
        point: &QpssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<QpacAnalysisResult, SimulationError> {
        check_abort(abort)?;
        let engine = self.resolved_for_netlist(netlist);
        let request =
            QpacRequest::from_qpac_card_with_abort(card, &engine.config.resource_limits, abort)?;
        engine.run_qpac_from_qpss_with_abort(netlist, request, point, abort)
    }
}

#[cfg(test)]
mod tests;
