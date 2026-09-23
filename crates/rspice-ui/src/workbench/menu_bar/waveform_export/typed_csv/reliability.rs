//! Long-form export of request, primary stress, aging and electrical evidence.
use super::*;
use rspice_core::engine::ReliabilityRunResult;

pub(super) fn prepare(response: &ReliabilityRunResult) -> Option<PreparedTypedResultCsv> {
    response
        .validate_retained_payload_with_abort(
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .ok()?;
    let mut contents = String::from(
        "record,phase_index,device,model,lifetime_years,time_s,quantity,value,unit,detail\n",
    );
    let mut rows = 0usize;
    let mut row = |record: &str,
                   phase: Option<usize>,
                   device: &str,
                   model: &str,
                   years: Option<f64>,
                   time: Option<f64>,
                   quantity: &str,
                   value: Option<f64>,
                   unit: &str,
                   detail: &str| {
        let number = |v: Option<f64>| v.map_or_else(String::new, |v| format!("{v:.17e}"));
        let fields = [
            record.into(),
            phase.map_or_else(String::new, |v| v.to_string()),
            device.into(),
            model.into(),
            number(years),
            number(time),
            quantity.into(),
            number(value),
            unit.into(),
            detail.into(),
        ];
        contents.push_str(
            &fields
                .iter()
                .map(|v| csv_text(v))
                .collect::<Vec<_>>()
                .join(","),
        );
        contents.push('\n');
        rows += 1;
    };
    row(
        "request",
        None,
        "",
        "",
        None,
        None,
        "complete_request",
        None,
        "",
        &serde_json::to_string(&response.stress.request).ok()?,
    );
    for phase in &response.stress.phases {
        for device in &phase.devices {
            for (time, stress) in phase.time_s.iter().zip(&device.samples) {
                for (name, value, unit) in [
                    ("Vgs", stress.gate_source_v, "V"),
                    ("Vds", stress.drain_source_v, "V"),
                    ("temperature", stress.temperature_k, "K"),
                    ("current_density", stress.current_density_a_per_m2, "A/m2"),
                ] {
                    row(
                        "stress",
                        Some(phase.phase_index),
                        &device.device,
                        &device.compact_model,
                        None,
                        Some(*time),
                        name,
                        Some(value),
                        unit,
                        "",
                    );
                }
            }
        }
    }
    for checkpoint in &response.stress.checkpoints {
        for device in &checkpoint.devices {
            for contribution in &device.contributions {
                row(
                    "aging",
                    None,
                    &device.device,
                    &contribution.model_id,
                    Some(checkpoint.years),
                    None,
                    if contribution.trap_occupancies.is_empty() {
                        "equivalent_age"
                    } else {
                        "elapsed_history"
                    },
                    Some(contribution.equivalent_seconds),
                    "s",
                    "",
                );
                for trap in &contribution.trap_occupancies {
                    row(
                        "trap_occupancy",
                        None,
                        &device.device,
                        &contribution.model_id,
                        Some(checkpoint.years),
                        None,
                        &trap.trap_id,
                        Some(trap.occupancy),
                        "1",
                        "",
                    );
                }
                if let Some(value) = contribution.electromigration_lifetime_fraction {
                    row(
                        "aging",
                        None,
                        &device.device,
                        &contribution.model_id,
                        Some(checkpoint.years),
                        None,
                        "EM_consumed_lifetime",
                        Some(value),
                        "1",
                        "not a failure probability",
                    );
                }
                for parameter in &contribution.parameters {
                    row(
                        "shift",
                        None,
                        &device.device,
                        &contribution.model_id,
                        Some(checkpoint.years),
                        None,
                        &parameter.parameter,
                        Some(parameter.shift),
                        if parameter.update
                            == rspice_core::analysis::reliability::AgingParameterUpdate::Relative
                        {
                            "1"
                        } else {
                            "model units"
                        },
                        &format!("{:?}", parameter.update),
                    );
                }
            }
        }
    }
    for point in &response.aged {
        for parameter in &point.parameters {
            for (kind, value) in [
                ("fresh_parameter", parameter.fresh_value),
                ("aged_parameter", parameter.aged_value),
            ] {
                row(
                    kind,
                    Some(point.phase_index),
                    &parameter.device,
                    &parameter.compact_model,
                    Some(point.years),
                    None,
                    &parameter.parameter,
                    Some(value),
                    "model units",
                    &format!("{:?}", parameter.update),
                );
            }
        }
    }
    for (kind, phase, years, point) in response
        .stress
        .phases
        .iter()
        .map(|p| {
            (
                "fresh_operating_point",
                p.phase_index,
                None,
                p.fresh_operating_point.as_ref().unwrap(),
            )
        })
        .chain(response.aged.iter().map(|p| {
            (
                "aged_operating_point",
                p.phase_index,
                Some(p.years),
                &p.operating_point,
            )
        }))
    {
        for (map, unit, domain) in [
            (&point.voltages, "V", "node_voltage"),
            (&point.branch_currents, "A", "branch_current"),
            (
                &point.device_observables,
                "device units",
                "device_observable",
            ),
        ] {
            for (name, value) in map {
                row(
                    kind,
                    Some(phase),
                    "",
                    "",
                    years,
                    None,
                    name,
                    Some(*value),
                    unit,
                    domain,
                );
            }
        }
    }
    Some(PreparedTypedResultCsv {
        default_name: "reliability-mission.csv",
        contents,
        detail: format!("{rows} retained mission records"),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn reliability_mission_csv_preserves_stress_shifts_and_electrical_coordinates() {
        for recovery in [false, true] {
            let analysis = if recovery {
                crate::simulation::SimulationResult::reliability_recovery_retained_test_fixture()
            } else {
                crate::simulation::SimulationResult::reliability_mission_retained_test_fixture()
            };
            let export = super::super::prepare_typed_result_csv(&analysis).unwrap();
            let mut csv = csv::Reader::from_reader(export.contents.as_bytes());
            assert_eq!(csv.headers().unwrap().len(), 10);
            let records: Vec<_> = csv.records().collect::<Result<_, _>>().unwrap();
            for kind in [
                "request",
                "stress",
                "aging",
                "shift",
                "fresh_parameter",
                "aged_parameter",
                "fresh_operating_point",
                "aged_operating_point",
            ] {
                assert!(records.iter().any(|r| &r[0] == kind), "missing {kind}");
            }
            let request = records.iter().find(|r| &r[0] == "request").unwrap();
            let _: rspice_core::analysis::reliability::ReliabilityRunRequest =
                serde_json::from_str(&request[9]).unwrap();
            assert!(
                records
                    .iter()
                    .any(|r| &r[6] == "Vgs" && r[7].parse::<f64>().unwrap() < 0.0)
            );
            if recovery {
                let occupations: Vec<_> = records
                    .iter()
                    .filter(|r| &r[0] == "trap_occupancy")
                    .collect();
                assert_eq!(occupations.len(), 2);
                for (row, expected) in occupations.iter().zip([
                    1.0 - (-2.0f64).exp(),
                    (1.0 - (-2.0f64).exp()) * (-3.0f64).exp(),
                ]) {
                    assert_eq!(&row[6], "interface");
                    assert_eq!(&row[8], "1");
                    assert!((row[7].parse::<f64>().unwrap() - expected).abs() < 1e-12);
                }
                assert!(records.iter().any(|r| &r[6] == "elapsed_history"));
                assert!(!records.iter().any(|r| &r[6] == "equivalent_age"));
            }
        }
    }
}
