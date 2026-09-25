//! QPAC authoring, native-card interchange and active numerical controls.
use super::types::AnalysisSpec;
use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
use rspice_core::engine::QpacRequest;
use rspice_core::netlist::{FreqVariation, PeriodicSweep, QpacCard, QpacSweep};
use rspice_simulation_contract::config::FrequencySweep;

pub use rspice_simulation_contract::quasi_periodic_controls::QpacControls;

impl AnalysisSpec {
    pub(crate) fn qpac_card(&self) -> Result<QpacCard, String> {
        let Self::Qpac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            input_source,
            output_node,
            output_ref,
            input_lattice,
            output_lattice,
            controls,
        } = self
        else {
            return Err("QPAC configuration requested for another analysis".into());
        };
        let card = QpacCard {
            sweep: match &controls.explicit_offsets {
                Some(values) => QpacSweep::Explicit(values.clone()),
                None => QpacSweep::Generated(PeriodicSweep {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points: *points_per_unit,
                    variation: match sweep {
                        FrequencySweep::Linear => FreqVariation::Lin,
                        FrequencySweep::Decade => FreqVariation::Dec,
                        FrequencySweep::Octave => FreqVariation::Oct,
                    },
                }),
            },
            input_source: input_source.clone(),
            output_node: output_node.clone(),
            output_ref: output_ref.clone(),
            input_lattice: input_lattice.clone(),
            output_lattice: output_lattice.clone(),
            magnitude: Some(controls.magnitude),
            phase_degrees: Some(controls.phase_degrees),
            linear_solver: Some(
                match controls.solver.linear.method {
                    QuasiPeriodicLinearMethod::Auto => "AUTO",
                    QuasiPeriodicLinearMethod::Direct => "DIRECT",
                    QuasiPeriodicLinearMethod::Krylov => "KRYLOV",
                }
                .into(),
            ),
            krylov_restart: Some(controls.solver.linear.restart),
            krylov_cycles: Some(controls.solver.linear.max_cycles),
            linear_tolerance: Some(controls.solver.linear.relative_tolerance),
            current_absolute_tolerance: Some(controls.solver.current_absolute_tolerance),
            voltage_absolute_tolerance: Some(controls.solver.voltage_absolute_tolerance),
        };
        QpacRequest::validate_qpac_card(&card, &rspice_core::ResourceLimits::default())
            .map_err(|e| e.to_string())?;
        Ok(card)
    }

    pub(crate) fn from_qpac_card(card: &QpacCard) -> Result<Self, String> {
        let request = QpacRequest::from_qpac_card(card).map_err(|e| e.to_string())?;
        let (start_freq, stop_freq, points_per_unit, sweep, explicit_offsets) = match &card.sweep {
            QpacSweep::Generated(s) => (
                s.start_freq,
                s.stop_freq,
                s.points,
                match s.variation {
                    FreqVariation::Lin => FrequencySweep::Linear,
                    FreqVariation::Dec => FrequencySweep::Decade,
                    FreqVariation::Oct => FrequencySweep::Octave,
                },
                None,
            ),
            QpacSweep::Explicit(values) => {
                (0.0, 0.0, 1, FrequencySweep::Linear, Some(values.clone()))
            }
        };
        Ok(Self::Qpac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            input_source: request.input_source,
            output_node: request.output_node,
            output_ref: request.output_ref,
            input_lattice: request.input_lattice,
            output_lattice: request.output_lattice,
            controls: QpacControls {
                magnitude: request.magnitude,
                phase_degrees: request.phase_degrees,
                solver: request.solver,
                explicit_offsets,
            },
        })
    }
}
