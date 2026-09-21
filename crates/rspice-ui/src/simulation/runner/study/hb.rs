//! Configured OP initialization on each varied harmonic-balance study circuit.
use super::*;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::SimulationResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyHbConfig {
    pub request: AnalysisSpec,
    pub operating_point: StudyOperatingPoint,
}

impl StudyHbConfig {
    pub(super) fn validate(&self) -> Result<(), String> {
        if !matches!(self.request, AnalysisSpec::HarmonicBalance { .. }) {
            return Err("An HB study requires a harmonic-balance specification".into());
        }
        self.request.validate()?;
        self.operating_point.config.validate()
    }

    pub(super) fn run_with_circuit(
        &self,
        engine: &rspice_core::Engine,
        circuit: &rspice_core::Netlist,
        numeric_options: &str,
        abort: &dyn AbortSignal,
    ) -> Result<(rspice_core::Netlist, SimulationResult), SimulationError> {
        self.validate().map_err(SimulationError::InvalidConfig)?;
        // The HB overlay decides startup; it must not leak into the OP solve.
        let controls = super::pss::circuit_with_options(circuit, numeric_options, abort)?;
        let (physical, seed) = if controls.options.hb_time_domain_mode
            == Some(rspice_core::netlist::XyceHbTimeDomainMode::Direct)
        {
            let (physical, _) = self.operating_point.physical_circuit(circuit, abort)?;
            (physical, None)
        } else {
            let (physical, seed) = self.operating_point.run(engine, circuit, abort)?;
            (physical, Some(seed))
        };
        let mut physical = super::pss::circuit_with_options(&physical, numeric_options, abort)?;
        physical.options.temp = Some(self.operating_point.config.temperature_celsius);
        let result = super::super::spec::run_hb_seeded_study_on_materialized(
            self.request.clone(),
            &physical,
            seed.as_ref(),
            abort,
        )?;
        Ok((physical, result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::dialog::{OpConfig, OpTemperatureMode};
    use crate::simulation::multi_run::HbToneSpec;
    use rspice_core::{Engine, Netlist, NoAbort};

    #[test]
    fn hb_study_bound_op_controls_all_startup_modes_and_supply_once() {
        let mut config = StudyHbConfig {
            request: AnalysisSpec::HarmonicBalance {
                tones: vec![HbToneSpec::new(1000.0, 3)],
                reltol: 1e-8,
                abstol: 1e-12,
                max_iterations: 40,
                damping: 1.0,
                min_damping: 0.01,
                oversample: 4,
                collocation_points: None,
                max_mixing_order: 3,
                use_krylov: false,
                gmres_restart: 12,
                source_stepping: false,
                use_exact_jacobian: true,
                verbose: false,
            },
            operating_point: StudyOperatingPoint {
                instance_id: AnalysisInstanceId::new(),
                source_revision: ObjectRevision::INITIAL,
                config: OpConfig {
                    temperature_mode: OpTemperatureMode::Explicit,
                    temperature_celsius: 37.0,
                    ..Default::default()
                },
                numeric_options: ".options GMIN=1e-7".into(),
            },
        };
        config.operating_point.config.run_point.supply_voltage = Some(2.0);
        config
            .operating_point
            .config
            .run_point
            .nominal_supply_voltage = Some(1.0);
        config.operating_point.config.run_point.supply_source_names = vec!["V1".into()];
        for r in [1000.0, 2000.0] {
            let circuit = Netlist::parse(&format!("HB trial\nV1 in 0 DC .2 AC .1\nRS in out {r} TC1=.01\nRL out 0 1k\nC1 out 0 100n\n.options TNOM=27 TEMP=12\n.end\n")).unwrap();
            for mode in [None, Some(0), Some(1), Some(2)] {
                let options = format!(
                    ".options GMIN=0\n{}",
                    mode.map(|mode| format!(".options HBINT TAHB={mode}"))
                        .unwrap_or_default()
                );
                config.operating_point.numeric_options = ".options invalid_option=1".into();
                let attempt =
                    config.run_with_circuit(&Engine::default(), &circuit, &options, &NoAbort);
                let (physical, result) = if mode == Some(0) {
                    attempt.expect("zero start must not execute OP-only numerical options")
                } else {
                    assert!(
                        attempt.is_err(),
                        "seeded startup must execute the selected OP configuration"
                    );
                    config.operating_point.numeric_options = ".options GMIN=1e-7".into();
                    config
                        .run_with_circuit(&Engine::default(), &circuit, &options, &NoAbort)
                        .unwrap()
                };
                assert_eq!(physical.options.temp, Some(37.0));
                let dc = result
                    .study_measurement("bin:0:real:V(out)")
                    .unwrap()
                    .value
                    .unwrap();
                assert!(
                    (dc - 0.4 * 1000.0 / (1000.0 + 1.1 * r)).abs() < 1e-9,
                    "{mode:?}: {dc}"
                );
            }
        }
        let base = StudyRunConfig {
            instance_id: AnalysisInstanceId::new(),
            source_revision: ObjectRevision::INITIAL,
            analysis: StudyAnalysis::Hb(Box::new(config)),
            postprocess: None,
            analysis_line: ".hb 1000".into(),
            numeric_options: String::new(),
            measurements: vec!["bin:0:real:V(out)".into()],
            histogram_bins: 5,
            constraints: vec![],
            objective_terms: vec![],
        };
        let environment = MonteCarloEnvironment {
            temperature_celsius: 52.0,
            supply_voltage: Some(3.0),
            nominal_supply_voltage: Some(1.0),
            supply_source_names: vec!["V1".into()],
        };
        let StudyAnalysis::Hb(explicit) = analysis_for_environment(&base, Some(&environment))
        else {
            unreachable!()
        };
        assert_eq!(explicit.operating_point.config.temperature_celsius, 37.0);
        assert_eq!(
            explicit.operating_point.config.run_point.supply_voltage,
            None
        );
        let mut base = base;
        let StudyAnalysis::Hb(config) = &mut base.analysis else {
            unreachable!()
        };
        config.operating_point.config.temperature_mode = OpTemperatureMode::PvtRunSet;
        let StudyAnalysis::Hb(inherited) = analysis_for_environment(&base, Some(&environment))
        else {
            unreachable!()
        };
        assert_eq!(inherited.operating_point.config.temperature_celsius, 52.0);
    }
}
