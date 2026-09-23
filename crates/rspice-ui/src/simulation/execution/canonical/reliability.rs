//! Exact request identity for calibration and every mission/binding field.

use super::CanonicalWriter;
use crate::simulation::reliability_engine::ReliabilityStudy;
use rspice_core::analysis::reliability::*;

pub(super) fn encode(w: &mut CanonicalWriter, study: &ReliabilityStudy) {
    w.domain("reliability-study-v1");
    let pack = &study.model_pack;
    w.u64(u64::from(pack.schema_version));
    for text in [
        &pack.id,
        &pack.process,
        &pack.source,
        &pack.license,
        &pack.characterization,
    ] {
        w.string(text);
    }
    w.u8(match pack.qualification {
        AgingQualification::PublicReference => 0,
        AgingQualification::UserCharacterized => 1,
        AgingQualification::FoundryQualified => 2,
    });
    w.sequence(pack.models.len());
    for model in &pack.models {
        w.string(&model.id);
        w.string(&model.applicability);
        w.u8(match model.mechanism {
            AgingMechanism::Hci => 0,
            AgingMechanism::Nbti => 1,
            AgingMechanism::Electromigration => 2,
        });
        for range in [
            model.validity.gate_source_v,
            model.validity.drain_source_v,
            model.validity.temperature_k,
            model.validity.current_density_a_per_m2,
        ] {
            w.f64(range.min);
            w.f64(range.max);
        }
        w.f64(model.validity.max_equivalent_seconds);
        match &model.law {
            AgingLaw::TabulatedTwoState { table } => {
                w.u8(2);
                for axis in [
                    &table.gate_source_v,
                    &table.drain_source_v,
                    &table.temperature_k,
                ] {
                    w.sequence(axis.len());
                    for value in axis {
                        w.f64(*value);
                    }
                }
                w.u8(match table.interpolation {
                    AgingRateInterpolation::Linear => 0,
                    AgingRateInterpolation::Logarithmic => 1,
                });
                w.sequence(table.traps.len());
                for trap in &table.traps {
                    w.string(&trap.id);
                    w.f64(trap.initial_occupancy);
                    for rates in [&trap.capture_rates_per_s, &trap.emission_rates_per_s] {
                        w.sequence(rates.len());
                        for rate in rates {
                            w.f64(*rate);
                        }
                    }
                    w.sequence(trap.parameters.len());
                    for parameter in &trap.parameters {
                        w.string(&parameter.parameter);
                        w.f64(parameter.shift_per_occupancy);
                        w.u8(match parameter.update {
                            AgingParameterUpdate::Additive => 0,
                            AgingParameterUpdate::Relative => 1,
                        });
                    }
                }
            }
            AgingLaw::EquivalentTimePower {
                reference_time_s,
                reference_gate_magnitude_v,
                reference_drain_magnitude_v,
                reference_temperature_k,
                gate_polarity,
                clock_gate_exponent,
                clock_drain_exponent,
                clock_activation_energy_ev,
                time_exponent,
                parameters,
            } => {
                w.u8(0);
                for value in [
                    reference_time_s,
                    reference_gate_magnitude_v,
                    reference_drain_magnitude_v,
                    reference_temperature_k,
                    clock_gate_exponent,
                    clock_drain_exponent,
                    clock_activation_energy_ev,
                    time_exponent,
                ] {
                    w.f64(*value);
                }
                w.u8(match gate_polarity {
                    AgingGatePolarity::Positive => 0,
                    AgingGatePolarity::Negative => 1,
                    AgingGatePolarity::Either => 2,
                });
                w.sequence(parameters.len());
                for parameter in parameters {
                    w.string(&parameter.parameter);
                    w.f64(parameter.scale_at_reference_time);
                    w.u8(match parameter.update {
                        AgingParameterUpdate::Additive => 0,
                        AgingParameterUpdate::Relative => 1,
                    });
                }
            }
            AgingLaw::BlackElectromigration {
                reference_lifetime_s,
                reference_current_density_a_per_m2,
                reference_temperature_k,
                current_exponent,
                activation_energy_ev,
            } => {
                w.u8(1);
                for value in [
                    reference_lifetime_s,
                    reference_current_density_a_per_m2,
                    reference_temperature_k,
                    current_exponent,
                    activation_energy_ev,
                ] {
                    w.f64(*value);
                }
            }
        }
    }
    w.sequence(study.bindings.len());
    for binding in &study.bindings {
        w.string(&binding.device);
        w.string(&binding.compact_model);
        w.sequence(binding.aging_models.len());
        for model in &binding.aging_models {
            w.string(model);
        }
        w.option(binding.conductor_area_m2.as_ref(), |w, area| w.f64(*area));
    }
    w.sequence(study.mission.len());
    for phase in &study.mission {
        w.string(&phase.name);
        w.f64(phase.duration_s);
        w.f64(phase.temperature_c);
        w.sequence(phase.parameters.len());
        for (name, value) in &phase.parameters {
            w.string(name);
            w.f64(*value);
        }
    }
    w.bool(study.repeat_mission);
    w.option(study.transient_stress.as_ref(), |w, window| {
        w.f64(window.step_s);
        w.f64(window.stop_s);
        w.f64(window.start_s);
        w.option(window.max_step_s.as_ref(), |w, value| w.f64(*value));
        w.bool(window.use_initial_conditions);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::multi_run::AnalysisSpec;
    use crate::simulation::reliability_engine::tests::{fixture, spec};

    fn digest(spec: &AnalysisSpec) -> crate::product::ContentDigest {
        let mut w = CanonicalWriter::new("test");
        super::super::analysis_spec::encode_analysis_spec(&mut w, spec);
        w.finish()
    }

    fn mutations(value: &serde_json::Value) -> Vec<serde_json::Value> {
        use serde_json::Value;
        match value {
            Value::Object(map) => map
                .iter()
                .flat_map(|(key, item)| {
                    mutations(item).into_iter().map(move |changed| {
                        let mut cloned = map.clone();
                        cloned.insert(key.clone(), changed);
                        Value::Object(cloned)
                    })
                })
                .collect(),
            Value::Array(values) => values
                .iter()
                .enumerate()
                .flat_map(|(index, item)| {
                    mutations(item).into_iter().map(move |changed| {
                        let mut cloned = values.clone();
                        cloned[index] = changed;
                        Value::Array(cloned)
                    })
                })
                .collect(),
            Value::Bool(b) => vec![Value::Bool(!b)],
            Value::Number(n) => vec![serde_json::json!(n.as_f64().unwrap() * 1.1 + 0.01)],
            Value::String(s) => vec![Value::String(format!("{s}-changed"))],
            Value::Null => Vec::new(),
        }
    }

    #[test]
    fn reliability_study_identity_covers_numeric_fields_provenance_and_mission_order() {
        for study in [
            fixture(),
            crate::simulation::reliability_engine::tests::recovery_fixture(),
        ] {
            let original = digest(&spec(Some(study.clone())));
            let mut checked = 0;
            for mutation in mutations(&serde_json::to_value(&study).unwrap()) {
                if let Ok(changed) = serde_json::from_value::<ReliabilityStudy>(mutation) {
                    assert_ne!(original, digest(&spec(Some(changed))));
                    checked += 1;
                }
            }
            assert!(checked > 35);
            let mut reversed = study;
            reversed.mission.reverse();
            assert_ne!(original, digest(&spec(Some(reversed))));
        }
        let legacy_spec = spec(None);
        let mut legacy = CanonicalWriter::new("test");
        legacy.domain("analysis-spec");
        legacy.u8(super::super::analysis_spec::analysis_kind_tag(&legacy_spec));
        legacy.sequence(2);
        legacy.f64(1.0000000123);
        legacy.f64(10.0);
        legacy.bool(false);
        legacy.bool(true);
        legacy.bool(false);
        legacy.f64(0.0);
        assert_eq!(digest(&legacy_spec), legacy.finish());
    }
}
