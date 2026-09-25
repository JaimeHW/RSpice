//! QPSS settings shared by authoring and execution.
use super::types::{AnalysisSpec, HbToneSpec};
use rspice_core::engine::{QpssConfig, QpssOscillator};

pub use rspice_simulation_contract::quasi_periodic_controls::QpssControls;

impl AnalysisSpec {
    pub(crate) fn from_qpss_config(config: QpssConfig) -> Self {
        let tones = config
            .grid
            .frequencies_hz
            .iter()
            .zip(&config.grid.harmonics)
            .map(|(frequency, harmonics)| HbToneSpec::new(*frequency, *harmonics))
            .collect();
        Self::Qpss {
            tones,
            max_iterations: config.solver.max_iterations,
            relative_tolerance: config.solver.relative_tolerance,
            autonomous: config.oscillator.is_some(),
            oscillator_node: config
                .oscillator
                .as_ref()
                .map(|oscillator| oscillator.node.clone()),
            controls: QpssControls {
                current_absolute_tolerance: config.solver.current_absolute_tolerance,
                voltage_absolute_tolerance: config.solver.voltage_absolute_tolerance,
                max_backtracks: config.solver.max_backtracks,
                linear: config.solver.linear,
                max_mixing_order: config.grid.max_mixing_order,
                sampling: config.grid.sampling,
                initial_state: config.initial_state,
                source_tones: config.source_tones,
                oscillator: config.oscillator,
            },
        }
    }

    pub fn qpss_config(&self) -> Result<QpssConfig, String> {
        let Self::Qpss {
            tones,
            max_iterations,
            relative_tolerance,
            autonomous,
            oscillator_node,
            controls,
            ..
        } = self
        else {
            return Err("expected a QPSS specification".into());
        };
        let mut controls = controls.clone();
        if *autonomous {
            let node = oscillator_node
                .as_ref()
                .filter(|node| !node.trim().is_empty())
                .ok_or("autonomous QPSS requires an oscillator node")?;
            if controls.oscillator.is_none() {
                controls.oscillator = Some(QpssOscillator::new(0, node.clone()));
            }
            if controls
                .oscillator
                .as_ref()
                .is_some_and(|oscillator| !oscillator.node.eq_ignore_ascii_case(node))
            {
                return Err("QPSS oscillator node differs from its controls".into());
            }
        } else if controls.oscillator.is_some() {
            return Err("QPSS oscillator controls require autonomous mode".into());
        }
        controls.to_core_config(tones, *max_iterations, *relative_tolerance)
    }

    #[cfg(test)]
    pub fn driven_qpss_config(&self) -> Result<QpssConfig, String> {
        let config = self.qpss_config()?;
        if config.oscillator.is_some() {
            return Err("a driven QPSS configuration cannot use an autonomous oscillator".into());
        }
        Ok(config)
    }
}
