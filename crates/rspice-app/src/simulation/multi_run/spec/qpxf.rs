//! QPXF controls, legacy-compatible specifications and native-card interchange.
use super::types::AnalysisSpec;
use crate::simulation::multi_run::FrequencySweep;
use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
use rspice_core::engine::{
    QpxfFrequencyAxis, QpxfInputLattices, QpxfOutput, QpxfRequest, QpxfSources,
};
use rspice_core::netlist::{
    FreqVariation, PeriodicSweep, QpacSweep, QpxfCard, QpxfCardLattices, QpxfCardOutput,
    QpxfCardSources,
};

pub use rspice_simulation_contract::quasi_periodic_controls::QpxfControls;

impl AnalysisSpec {
    pub(crate) fn qpxf_card(&self) -> Result<QpxfCard, String> {
        let Self::Qpxf {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            input_source,
            output_node,
            output_ref,
            input_lattice,
            output_lattice,
            group_delay,
            controls,
        } = self
        else {
            return Err("QPXF configuration requested for another analysis".into());
        };
        let card = QpxfCard {
            sweep: match &controls.explicit_frequencies {
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
            frequency_axis: Some(
                match controls.frequency_axis {
                    QpxfFrequencyAxis::Output => "OUTPUT",
                    QpxfFrequencyAxis::Offset => "OFFSET",
                }
                .into(),
            ),
            input_sources: match &controls.input_sources {
                None => QpxfCardSources::Named(vec![input_source.clone()]),
                Some(QpxfSources::AllIndependent) => QpxfCardSources::AllIndependent,
                Some(QpxfSources::Named(names)) => QpxfCardSources::Named(names.clone()),
            },
            input_lattices: match &controls.input_lattices {
                None => QpxfCardLattices::Explicit(vec![input_lattice.clone()]),
                Some(QpxfInputLattices::AllRetained) => QpxfCardLattices::AllRetained,
                Some(QpxfInputLattices::Explicit(tuples)) => {
                    QpxfCardLattices::Explicit(tuples.clone())
                }
                Some(QpxfInputLattices::MaxOrders(orders)) => {
                    QpxfCardLattices::MaxOrders(orders.clone())
                }
            },
            output: match &controls.branch_current {
                None => QpxfCardOutput::Voltage {
                    positive: output_node.clone(),
                    negative: output_ref.clone(),
                },
                Some(branch) => QpxfCardOutput::BranchCurrent {
                    branch: branch.clone(),
                },
            },
            output_lattice: output_lattice.clone(),
            linear_solver: Some(
                match controls.solver.method {
                    QuasiPeriodicLinearMethod::Auto => "AUTO",
                    QuasiPeriodicLinearMethod::Direct => "DIRECT",
                    QuasiPeriodicLinearMethod::Krylov => "KRYLOV",
                }
                .into(),
            ),
            krylov_restart: Some(controls.solver.restart),
            krylov_cycles: Some(controls.solver.max_cycles),
            linear_tolerance: Some(controls.solver.relative_tolerance),
            group_delay: Some(*group_delay),
            group_delay_magnitude_floor: Some(controls.group_delay_magnitude_floor),
        };
        QpxfRequest::validate_qpxf_card(&card, &rspice_core::ResourceLimits::default())
            .map_err(|e| e.to_string())?;
        Ok(card)
    }
    pub(crate) fn from_qpxf_card(card: &QpxfCard) -> Result<Self, String> {
        let request = QpxfRequest::from_qpxf_card(card).map_err(|e| e.to_string())?;
        let (start_freq, stop_freq, points_per_unit, sweep, explicit_frequencies) =
            match &card.sweep {
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
        // Canonicalize one-item selections to their legacy spelling so native
        // import, older saves and new editor drafts share one request identity.
        let (input_source, input_sources) = match request.input_sources {
            QpxfSources::Named(mut names) if names.len() == 1 => (names.remove(0), None),
            selected => (String::new(), Some(selected)),
        };
        let (input_lattice, input_lattices) = match request.input_lattices {
            QpxfInputLattices::Explicit(mut tuples) if tuples.len() == 1 => {
                (tuples.remove(0), None)
            }
            selected => (Vec::new(), Some(selected)),
        };
        let (output_node, output_ref, branch_current) = match request.output {
            QpxfOutput::Voltage { positive, negative } => (positive, negative, None),
            QpxfOutput::BranchCurrent { branch } => (String::new(), String::new(), Some(branch)),
        };
        Ok(Self::Qpxf {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            input_source,
            output_node,
            output_ref,
            input_lattice,
            output_lattice: request.output_lattice,
            group_delay: request.group_delay,
            controls: QpxfControls {
                frequency_axis: request.frequency_axis,
                explicit_frequencies,
                input_sources,
                input_lattices,
                branch_current,
                solver: request.linear,
                group_delay_magnitude_floor: request.group_delay_magnitude_floor,
            },
        })
    }
}
