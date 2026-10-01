//! SOA draft, worker, numerical and retained-evidence contracts.

use super::*;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;

mod curves;

fn execute(spec: AnalysisSpec, deck: &str) -> AnalysisResult {
    let result = run_spec_request(
        &EngineBridge::new(),
        spec,
        Default::default(),
        deck,
        None,
        &Default::default(),
        &rspice_core::NoAbort,
    )
    .unwrap();
    crate::result_conversion::convert(result, AnalysisType::Soa, "SOA", || 0.0)
}

fn round_trip_payload(retained: &AnalysisResult) {
    let mut restored = retained.clone();
    restored.result_payload =
        serde_json::from_slice(&serde_json::to_vec(&retained.result_payload).unwrap()).unwrap();
    assert_eq!(restored.result_payload, retained.result_payload);
    assert_eq!(
        restored
            .result_data_ref()
            .digest(rspice_results::result_digest::ResultDigestEncoding::CURRENT),
        retained
            .result_data_ref()
            .digest(rspice_results::result_digest::ResultDigestEncoding::CURRENT)
    );
    restored.validate_retained_evidence().unwrap();
}

#[test]
fn soa_directional_limits_survive_draft_preparation_worker_requests_and_retained_results() {
    use rspice_results::safety::SoAParameter;
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_simulation_contract::soa_rule::SoaRuleConfig;
    use rspice_simulation_contract::worker_soa::WorkerSoAParameter;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let mut config = SoaConfig {
        stop_time: 1e-9,
        step_time: 1e-10,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        ..Default::default()
    };
    let parameters = [
        SoAParameter::Vgs,
        SoAParameter::Vds,
        SoAParameter::Vgd,
        SoAParameter::Vbe,
        SoAParameter::Vce,
        SoAParameter::Vbc,
        SoAParameter::Id,
        SoAParameter::Ic,
        SoAParameter::Ig,
        SoAParameter::Is,
        SoAParameter::Ib,
        SoAParameter::Ie,
    ];
    for base in parameters {
        let (positive, negative) = base.directional_pair().unwrap();
        for parameter in [base, positive, negative] {
            config.rules.push(SoaRuleConfig {
                duration_mode: Default::default(),
                minimum_duration_s: None,
                current_envelope: None,
                power_derating: None,
                voltage_basis: Default::default(),
                parameter,
                max_value: if parameter.polarity().is_some() {
                    0.0
                } else {
                    10.0
                },
                devices: vec![],
                models: vec![],
            });
            let wire = WorkerSoAParameter::from(parameter);
            let wire: WorkerSoAParameter =
                serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
            assert_eq!(SoAParameter::from(wire), parameter);
        }
    }
    config.rules.push(SoaRuleConfig {
        duration_mode: Default::default(),
        minimum_duration_s: None,
        current_envelope: None,
        power_derating: None,
        voltage_basis: Default::default(),
        parameter: SoAParameter::Pdiss,
        max_value: 0.001,
        devices: vec![],
        models: vec![],
    });
    let draft = SoaDialogState::from_config(&config);
    assert_eq!(draft.to_config().unwrap(), config);
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let wire: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    let restored = AnalysisSpec::from(wire);
    assert_eq!(restored, spec);
    spec = restored;
    // Both transistor polarities are driven at known signed terminal voltages.
    // Conduction flows into N-device drain/collector and out of P devices.
    let deck = "directional limits\nVd d 0 1\nVg g 0 2\nVs s 0 2\nVpg pg 0 0\nVpd pd 0 1\nVc c 0 1.5\nVb b 0 0.65\nVpc pc 0 0.5\nVpb pb 0 1.35\nMN d g 0 0 NM W=10u L=1u\nMP pd pg s s PM W=10u L=1u\nQN c b 0 NPN\nQP pc pb s PNP\n.model NM NMOS LEVEL=1 VTO=1 KP=1m\n.model PM PMOS LEVEL=1 VTO=-1 KP=1m\n.model NPN NPN IS=1e-14 BF=100\n.model PNP PNP IS=1e-14 BF=100\n.end\n";
    rspice_core::netlist::parse_netlist(deck).expect("the plan statement remains parseable");
    let retained = &execute(spec.clone(), deck);
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa { evaluations, .. }) =
        &retained.result_payload
    else {
        panic!("SOA evidence");
    };
    assert_eq!(evaluations.len(), 76);
    let trace = |name: &str| {
        &retained
            .waveforms
            .iter()
            .find(|w| w.name == name)
            .unwrap()
            .y
    };
    for (device, bases) in [
        ("MN", &parameters[..3]),
        ("MP", &parameters[..3]),
        ("QN", &parameters[3..6]),
        ("QP", &parameters[3..6]),
    ] {
        for base in bases {
            let magnitude = trace(&format!("SOA_{}({device})", base.stress_code()));
            let positive = trace(&format!("SOA_{}_POS({device})", base.stress_code()));
            let negative = trace(&format!("SOA_{}_NEG({device})", base.stress_code()));
            for i in 0..magnitude.len() {
                assert_eq!(positive[i] + negative[i], magnitude[i]);
                assert!(positive[i] == 0.0 || negative[i] == 0.0);
            }
        }
    }
    for (device, code, sign) in [
        ("MN", "ID", "POS"),
        ("MP", "ID", "NEG"),
        ("QN", "IC", "POS"),
        ("QP", "IC", "NEG"),
    ] {
        let conducting = trace(&format!("SOA_{code}_{sign}({device})"));
        assert!(
            conducting.iter().all(|v| *v > 1e-5),
            "{device}: {conducting:?}"
        );
        let opposite = if sign == "POS" { "NEG" } else { "POS" };
        assert!(
            trace(&format!("SOA_{code}_{opposite}({device})"))
                .iter()
                .all(|v| *v == 0.0)
        );
    }
    for (device, incoming, outgoing, polarity) in [
        ("MN", "ID", "IS", "NEG"),
        ("MP", "ID", "IS", "POS"),
        ("QN", "IC", "IE", "NEG"),
        ("QP", "IC", "IE", "POS"),
    ] {
        let primary = trace(&format!("SOA_{incoming}({device})"));
        let returning = trace(&format!("SOA_{outgoing}_{polarity}({device})"));
        let factor = if device.starts_with('Q') { 1.01 } else { 1.0 };
        for (incoming, outgoing) in primary.iter().zip(returning.iter()) {
            assert!((outgoing - factor * incoming).abs() < 1e-10 + incoming * 1e-6);
        }
        if device.starts_with('Q') {
            let base = trace(&format!(
                "SOA_IB_{}({device})",
                if device == "QN" { "POS" } else { "NEG" }
            ));
            for (base, collector) in base.iter().zip(primary.iter()) {
                assert!((base * 100.0 - collector).abs() < 1e-10 + collector * 1e-6);
            }
        } else {
            assert!(
                trace(&format!("SOA_IG({device})"))
                    .iter()
                    .all(|i| i.abs() < 1e-10)
            );
        }
    }
    for (name, expected) in [
        ("SOA_VGS_POS(MN)", 2.0),
        ("SOA_VGS_NEG(MP)", 2.0),
        ("SOA_VBC_NEG(QN)", 0.85),
        ("SOA_VBC_POS(QP)", 0.85),
    ] {
        assert!(trace(name).iter().all(|v| (*v - expected).abs() < 1e-12));
    }
    for (device, current, factor) in [
        ("MN", "ID", 1.0),
        ("MP", "ID", 1.0),
        ("QN", "IC", 1.5065),
        ("QP", "IC", 1.5065),
    ] {
        let power = trace(&format!("SOA_PDISS({device})"));
        let magnitude = trace(&format!("SOA_{current}({device})"));
        for (power, current) in power.iter().zip(magnitude.iter()) {
            assert!((*power - factor * current).abs() < 1e-10 + current * 1e-6);
            assert!(*power > 0.0);
        }
    }
    round_trip_payload(retained);
}

#[test]
fn soa_temperature_limits_survive_draft_worker_execution_and_retained_results() {
    use rspice_results::safety::SoAParameter;
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_simulation_contract::soa_rule::SoaRuleConfig;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let config = SoaConfig {
        stop_time: 1e-9,
        step_time: 1e-10,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        rules: vec![SoaRuleConfig {
            duration_mode: Default::default(),
            minimum_duration_s: None,
            current_envelope: None,
            power_derating: None,
            voltage_basis: Default::default(),
            parameter: SoAParameter::Temp,
            max_value: rspice_core::constants::celsius_to_kelvin(-30.0),
            devices: vec!["M1".into(), "Q1".into()],
            models: vec![],
        }],
        ..Default::default()
    };
    let mut draft = SoaDialogState::from_config(&config);
    assert_eq!(draft.rules[0].max_value, "-30");
    assert!(draft.rules[0].is_temperature());
    assert_eq!(draft.to_config().unwrap(), config);
    draft.rules[0].max_value = "-273.15".into();
    assert!(draft.to_config().unwrap_err().contains("-273.15"));
    draft.rules[0].max_value = "-30".into();
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let decoded: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    assert_eq!(AnalysisSpec::from(decoded.clone()), spec);
    spec = AnalysisSpec::from(decoded);
    let deck = "temperature limits\nVd d 0 1\nVg g 0 2\nVb b 0 .5\nM1 d g 0 0 mm TEMP=-40\nQ1 d b 0 qm TEMP=85\n.model mm NMOS LEVEL=1 VTO=1 KP=1m\n.model qm NPN IS=1e-16 BF=100\n.end\n";
    let retained = &execute(spec.clone(), deck);
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa { evaluations, .. }) =
        &retained.result_payload
    else {
        panic!("SOA evidence");
    };
    assert_eq!(evaluations.len(), 2);
    for (device, kelvin) in [("M1", 233.15), ("Q1", 358.15)] {
        let evaluation = evaluations.iter().find(|e| e.device_id == device).unwrap();
        assert_eq!(evaluation.unit, "K");
        assert_eq!(evaluation.limit_value, config.rules[0].max_value);
        assert!((evaluation.worst_actual_value - kelvin).abs() < 1e-10);
        let trace = retained
            .waveforms
            .iter()
            .find(|w| w.name == format!("SOA_TEMP({device})"))
            .unwrap();
        assert!(trace.y.iter().all(|value| (value - kelvin).abs() < 1e-10));
    }
    round_trip_payload(retained);
}

#[test]
fn soa_body_and_backgate_limits_follow_model_pins_through_draft_and_retained_results() {
    use rspice_results::safety::SoAParameter;
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_simulation_contract::soa_rule::SoaRuleConfig;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let parameters = [
        SoAParameter::Vbs,
        SoAParameter::Vbd,
        SoAParameter::Vgb,
        SoAParameter::Ibulk,
        SoAParameter::Ves,
        SoAParameter::Ved,
        SoAParameter::Vge,
        SoAParameter::Ibackgate,
        SoAParameter::VbodyBackgate,
    ];
    let mut config = SoaConfig {
        stop_time: 1e-9,
        step_time: 1e-10,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        ..Default::default()
    };
    for base in parameters {
        let (positive, negative) = base.directional_pair().unwrap();
        for parameter in [base, positive, negative] {
            config.rules.push(SoaRuleConfig {
                duration_mode: Default::default(),
                minimum_duration_s: None,
                current_envelope: None,
                power_derating: None,
                voltage_basis: Default::default(),
                parameter,
                max_value: 10.0,
                devices: vec![],
                models: vec![],
            });
        }
    }
    let draft = SoaDialogState::from_config(&config);
    assert_eq!(draft.to_config().unwrap(), config);
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let wire: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    let restored = AnalysisSpec::from(wire);
    assert_eq!(restored, spec);
    spec = restored;
    for level in [55, 56, 57] {
        // The fourth SOI pin is E=+0.2V; its fifth body contact is B=-0.1V.
        // A four-terminal floating-body instance has E but no external B.
        let deck = format!(
            "Body terminal roles\nVd d 0 1\nVg g 0 1\nVb b 0 -0.1\nVe e 0 0.2\nMN d g 0 b NM W=4u L=1u\nX1 d g b e CELL\n.subckt CELL d g b e\nMT d g 0 e b NS W=4u L=1u\nMF d g 0 e NS W=4u L=1u\n.model NS NMOS LEVEL={level} CAPMOD=2\n.ends CELL\n.model NM NMOS LEVEL=1 VTO=0.7 KP=100u\n.end\n"
        );

        let retained = &execute(spec.clone(), &deck);
        assert!(
            retained.success,
            "LEVEL={level}: {:?}",
            retained.error_message
        );
        retained.validate_retained_evidence().unwrap();
        let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa {
            evaluations, ..
        }) = &retained.result_payload
        else {
            panic!("SOA evidence")
        };
        assert_eq!(evaluations.len(), 51);
        let trace = |device: &str, parameter: SoAParameter| {
            &retained
                .waveforms
                .iter()
                .find(|w| w.name == format!("SOA_{}({device})", parameter.stress_code()))
                .unwrap()
                .y
        };
        for (device, bases) in [
            ("MN", &parameters[..4]),
            ("X1.MT", &parameters[..]),
            ("X1.MF", &parameters[4..8]),
        ] {
            for base in bases {
                let (positive, negative) = base.directional_pair().unwrap();
                let magnitude = trace(device, *base);
                for (index, value) in magnitude.iter().enumerate() {
                    assert!(
                        (trace(device, positive)[index] + trace(device, negative)[index] - value)
                            .abs()
                            < 1e-14
                    );
                }
                assert!(
                    evaluations
                        .iter()
                        .filter(|e| e.device_id == device)
                        .all(|e| e.sample_count == magnitude.len() as u64)
                );
            }
        }
        for (device, quantity, expected) in [
            ("MN", SoAParameter::VbsNegative, 0.1),
            ("MN", SoAParameter::VbdNegative, 1.1),
            ("MN", SoAParameter::VgbPositive, 1.1),
            ("X1.MT", SoAParameter::VbsNegative, 0.1),
            ("X1.MT", SoAParameter::VbdNegative, 1.1),
            ("X1.MT", SoAParameter::VgbPositive, 1.1),
            ("X1.MT", SoAParameter::VbodyBackgateNegative, 0.3),
            ("X1.MT", SoAParameter::VesPositive, 0.2),
            ("X1.MF", SoAParameter::VesPositive, 0.2),
            ("X1.MF", SoAParameter::VedNegative, 0.8),
            ("X1.MF", SoAParameter::VgePositive, 0.8),
        ] {
            assert!(
                trace(device, quantity)
                    .iter()
                    .all(|v| (v - expected).abs() < 1e-8),
                "LEVEL={level} {device} {quantity:?}"
            );
        }
        assert!(
            !retained
                .waveforms
                .iter()
                .any(|w| w.name == "SOA_IBULK(X1.MF)")
        );
        round_trip_payload(retained);
    }
}

#[test]
fn soa_diode_and_bjt_substrate_limits_survive_draft_worker_and_retained_results() {
    use rspice_results::safety::SoAParameter;
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_simulation_contract::soa_rule::SoaRuleConfig;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let parameters = [
        SoAParameter::Vcsub,
        SoAParameter::Vbsub,
        SoAParameter::Vesub,
        SoAParameter::Isub,
        SoAParameter::Vak,
        SoAParameter::Ia,
    ];
    let mut config = SoaConfig {
        stop_time: 1e-9,
        step_time: 1e-10,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        ..Default::default()
    };
    for base in parameters {
        let (positive, negative) = base.directional_pair().unwrap();
        for parameter in [base, positive, negative] {
            config.rules.push(SoaRuleConfig {
                duration_mode: Default::default(),
                minimum_duration_s: None,
                current_envelope: None,
                power_derating: None,
                voltage_basis: Default::default(),
                parameter,
                max_value: 10.0,
                devices: vec![],
                models: vec![],
            });
        }
    }
    for (parameter, max_value) in [(SoAParameter::Pdiss, 0.1), (SoAParameter::Temp, 400.0)] {
        config.rules.push(SoaRuleConfig {
            duration_mode: Default::default(),
            minimum_duration_s: None,
            current_envelope: None,
            power_derating: None,
            voltage_basis: Default::default(),
            parameter,
            max_value,
            devices: vec!["D1".into(), "X1:D2".into()],
            models: vec![],
        });
    }
    let draft = SoaDialogState::from_config(&config);
    assert_eq!(draft.to_config().unwrap(), config);
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let wire: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    let restored = AnalysisSpec::from(wire);
    assert_eq!(restored, spec);
    spec = restored;
    let deck = "Diode and substrate rules\nVc c 0 1.5\nVb b 0 0.65\nVs sub 0 -0.2\nVa a 0 0.4\nVr r 0 -2\nQG c b 0 sub GP\nQV c b 0 sub VB\nQT c b 0 0 VT\nQ3 c b 0 GP\nD1 a 0 DM TEMP=85\nX1 r CELL\n.subckt CELL a\nD2 a 0 DM DTEMP=10\n.model DM D IS=1e-12 RS=10\n.ends CELL\n.model GP NPN IS=1e-14 BF=100\n.model VB NPN LEVEL=12 IS=1e-16 BF=100\n.model VT NPN LEVEL=11 IS=1e-16 BF=100\n.model DM D IS=1e-12 RS=10\n.end\n";

    let retained = &execute(spec.clone(), deck);
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa { evaluations, .. }) =
        &retained.result_payload
    else {
        panic!("SOA evidence")
    };
    assert_eq!(evaluations.len(), 40);
    let trace = |device: &str, parameter: SoAParameter| {
        &retained
            .waveforms
            .iter()
            .find(|w| w.name == format!("SOA_{}({device})", parameter.stress_code()))
            .unwrap()
            .y
    };
    for (device, bases) in [
        ("QG", &parameters[..4]),
        ("QV", &parameters[..4]),
        ("D1", &parameters[4..]),
        ("X1.D2", &parameters[4..]),
    ] {
        for base in bases {
            let (positive, negative) = base.directional_pair().unwrap();
            let magnitude = trace(device, *base);
            for (index, value) in magnitude.iter().enumerate() {
                assert!(
                    (trace(device, positive)[index] + trace(device, negative)[index] - value).abs()
                        < 1e-14
                );
            }
            assert!(
                evaluations
                    .iter()
                    .filter(|e| e.device_id == device)
                    .all(|e| e.sample_count == magnitude.len() as u64)
            );
        }
    }
    for (device, quantity, expected) in [
        ("QG", SoAParameter::VcsubPositive, 1.7),
        ("QV", SoAParameter::VcsubPositive, 1.7),
        ("QG", SoAParameter::VbsubPositive, 0.85),
        ("QV", SoAParameter::VesubPositive, 0.2),
        ("D1", SoAParameter::VakPositive, 0.4),
        ("X1.D2", SoAParameter::VakNegative, 2.0),
        ("D1", SoAParameter::Temp, 358.15),
        ("X1.D2", SoAParameter::Temp, 310.15),
    ] {
        assert!(
            trace(device, quantity)
                .iter()
                .all(|v| (v - expected).abs() < 1e-8),
            "{device} {quantity:?}"
        );
    }
    assert!(
        !retained
            .waveforms
            .iter()
            .any(|w| w.name == "SOA_ISUB(QT)" || w.name == "SOA_ISUB(Q3)")
    );
    for (device, direction, voltage) in [
        ("D1", SoAParameter::IaPositive, 0.4),
        ("X1.D2", SoAParameter::IaNegative, 2.0),
    ] {
        let current = trace(device, direction);
        assert!(current.iter().all(|v| *v > 0.0));
        for (i, p) in current
            .iter()
            .zip(trace(device, SoAParameter::Pdiss).iter())
        {
            assert!(
                (p - voltage * i).abs() < 1e-12 + 1e-6 * p.abs(),
                "{device}: P={p}, I={i}"
            );
        }
    }
    round_trip_payload(retained);
}

#[test]
fn soa_intrinsic_voltage_rules_survive_draft_worker_and_retained_results() {
    use rspice_results::safety::{SoAParameter, SoaVoltageBasis};
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_simulation_contract::soa_rule::SoaRuleConfig;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let mut config = SoaConfig {
        stop_time: 1e-9,
        step_time: 1e-10,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        ..Default::default()
    };
    for (device, parameter, voltage_basis) in [
        ("M1", SoAParameter::Vgs, SoaVoltageBasis::ExternalTerminals),
        (
            "M1",
            SoAParameter::VgsNegative,
            SoaVoltageBasis::IntrinsicNodes,
        ),
        ("Q1", SoAParameter::Vbe, SoaVoltageBasis::ExternalTerminals),
        (
            "Q1",
            SoAParameter::VbePositive,
            SoaVoltageBasis::IntrinsicNodes,
        ),
        ("D1", SoAParameter::Vak, SoaVoltageBasis::ExternalTerminals),
        (
            "D1",
            SoAParameter::VakPositive,
            SoaVoltageBasis::IntrinsicNodes,
        ),
        ("MF", SoAParameter::Vbs, SoaVoltageBasis::IntrinsicNodes),
    ] {
        config.rules.push(SoaRuleConfig {
            duration_mode: Default::default(),
            minimum_duration_s: None,
            current_envelope: None,
            power_derating: None,
            parameter,
            voltage_basis,
            max_value: 10.0,
            devices: vec![device.into()],
            models: vec![],
        });
    }
    let draft = SoaDialogState::from_config(&config);
    assert_eq!(draft.to_config().unwrap(), config);
    assert!(config.to_spice().contains("basis=intrinsic"));
    let mut old_rule = serde_json::to_value(&config.rules[0]).unwrap();
    old_rule.as_object_mut().unwrap().remove("voltage_basis");
    assert_eq!(
        serde_json::from_value::<SoaRuleConfig>(old_rule)
            .unwrap()
            .voltage_basis,
        SoaVoltageBasis::ExternalTerminals
    );
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let wire: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    let restored = AnalysisSpec::from(wire);
    assert_eq!(restored, spec);
    spec = restored;
    let deck = "Intrinsic SOA\n\
        Vd d 0 -2\nVg g 0 -1.2\nM1 d g 0 0 PM W=10u L=1u NRD=1 NRS=1\n\
        .model PM PMOS LEVEL=54 VTH0=-0.4 TOXE=3n U0=0.02 RSH=100\n\
        Vc c 0 2\nVb b 0 0.7\nQ1 c b 0 QM\n\
        .model QM NPN IS=1e-14 BF=100 RC=100 RB=100 RE=10\n\
        Va a 0 0.8\nD1 a 0 DM\n.model DM D IS=1e-12 RS=100\n\
        Ve e 0 0.2\nVn n 0 1\nMF n n 0 e SOI W=10u L=1u\n\
        .model SOI NMOS LEVEL=55 VTH0=0.4 U0=0.02 TOX=10n TSI=100n TBOX=300n\n.end\n";

    let retained = &execute(spec.clone(), deck);
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa { evaluations, .. }) =
        &retained.result_payload
    else {
        panic!("SOA evidence")
    };
    assert_eq!(evaluations.len(), 7);
    assert_eq!(
        evaluations
            .iter()
            .filter(|entry| entry
                .description
                .contains("intrinsic electrical model nodes"))
            .count(),
        4
    );
    let trace = |device: &str, parameter: SoAParameter| {
        &retained
            .waveforms
            .iter()
            .find(|wave| wave.name == format!("SOA_{}({device})", parameter.stress_code()))
            .unwrap()
            .y
    };
    for (device, external, intrinsic, applied) in [
        ("M1", SoAParameter::Vgs, SoAParameter::VgsNegative, 1.2),
        ("Q1", SoAParameter::Vbe, SoAParameter::VbePositive, 0.7),
        ("D1", SoAParameter::Vak, SoAParameter::VakPositive, 0.8),
    ] {
        for (outside, inside) in trace(device, external)
            .iter()
            .zip(trace(device, intrinsic).iter())
        {
            assert!((outside - applied).abs() < 1e-8);
            assert!(
                *inside > 0.1 && *inside < applied - 1e-4,
                "{device} intrinsic={inside} external={outside}"
            );
        }
    }
    assert!(
        trace("MF", SoAParameter::Vbs)
            .iter()
            .all(|value| value.is_finite())
    );
    round_trip_payload(retained);
}

#[test]
fn soa_model_voltage_ratings_survive_draft_worker_and_retained_results() {
    use rspice_results::safety::SoAParameter;
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_simulation_contract::soa_rule::SoaRuleConfig;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let config = SoaConfig {
        import_model_voltage_ratings: true,
        stop_time: 1e-9,
        step_time: 1e-10,
        check_vgs_max: true,
        max_vgs: 0.1, // Imported asymmetric ratings replace this default.
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        rules: vec![SoaRuleConfig {
            duration_mode: Default::default(),
            minimum_duration_s: None,
            current_envelope: None,
            power_derating: None,
            parameter: SoAParameter::VgsPositive,
            voltage_basis: Default::default(),
            max_value: 0.75,
            devices: vec!["X1.M1".into()],
            models: vec![],
        }],
        ..Default::default()
    };
    let draft = SoaDialogState::from_config(&config);
    assert_eq!(draft.to_config().unwrap(), config);
    assert!(config.to_spice().contains("model_voltage_ratings=on"));
    let mut old = serde_json::to_value(&draft).unwrap();
    old.as_object_mut()
        .unwrap()
        .remove("import_model_voltage_ratings");
    assert!(
        !serde_json::from_value::<SoaDialogState>(old)
            .unwrap()
            .import_model_voltage_ratings
    );
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let wire: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    let restored = AnalysisSpec::from(wire);
    assert_eq!(restored, spec);
    spec = restored;
    let deck = "Model rated SOA\nVd d 0 -2\nVg g 0 -1.2\n\
        .model PM.1 PMOS LEVEL=54 LMIN=0.5u LMAX=2u VTH0=-0.4 TOXE=3n U0=0.02 RSH=100 VGS_MAX=1 VGSR_MAX=0.5\n\
        .model PM.2 PMOS LEVEL=54 LMIN=2u LMAX=5u VTH0=-0.4 TOXE=3n U0=0.02 VGS_MAX=9\n\
        .subckt CELL d g\nM1 d g 0 0 PM W=10u L=1u NRD=1 NRS=1\n.ends\nX1 d g CELL\n\
        Va a 0 0.8\nD1 a 0 DM\n.model DM D IS=1e-12 RS=100 FV_MAX=0.7 BV_MAX=20\n\
        Vc c 0 2\nVb b 0 0.7\nQ1 c b 0 QM\n.model QM NPN IS=1e-14 BF=100 RB=100 RE=10 VBE_MAX=0.65\n\
        Vdn dn 0 2\nVgn gn 0 1.1\nVgp gp 0 -1.1\nM9N dn gn 0 0 N9 W=10u L=1u\nM9P d gp 0 0 P9 W=10u L=1u\n\
        .model N9 NMOS LEVEL=9 DVT0=2.2 VTH0=0.4 VGS_MAX=1 VGSR_MAX=0.5\n\
        .model P9 PMOS LEVEL=9 DVT0=2.2 VTH0=-0.4 VGS_MAX=1 VGSR_MAX=0.5\n\
        MVN dn gn 0 NV\nMVP d gp 0 PV\n\
        .model NV NMOS LEVEL=18 VTO=2 VGS_MAX=1 VGSR_MAX=0.5\n\
        .model PV VDMOS PCHAN=1 VTO=-2 VGS_MAX=1 VGSR_MAX=0.5\n.end\n";

    let retained = &execute(spec.clone(), deck);
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa { evaluations, .. }) =
        &retained.result_payload
    else {
        panic!("SOA evidence")
    };
    assert_eq!(evaluations.len(), 13);
    let find = |device: &str, parameter| {
        evaluations
            .iter()
            .find(|r| r.device_id == device && r.parameter == parameter)
            .unwrap()
    };
    let pm = find(
        "X1.M1",
        rspice_results::soa_evidence::SoaParameterEvidence::GateSourceVoltageNegative,
    );
    assert_eq!(pm.limit_value, 1.0);
    assert!(
        pm.description.contains("PM.1")
            && pm.description.contains("VGS_MAX")
            && pm.description.contains("intrinsic")
    );
    assert!(pm.worst_actual_value > 1.0 && pm.worst_actual_value < 1.2);
    let manual = find(
        "X1.M1",
        rspice_results::soa_evidence::SoaParameterEvidence::GateSourceVoltagePositive,
    );
    assert_eq!(manual.limit_value, 0.75);
    assert!(!manual.description.contains("Model"));
    let diode = find(
        "D1",
        rspice_results::soa_evidence::SoaParameterEvidence::AnodeCathodeVoltagePositive,
    );
    assert_eq!(diode.limit_value, 0.7);
    assert!((diode.worst_actual_value - 0.8).abs() < 1e-8);
    assert!(diode.description.contains("authored terminals"));
    let bjt = find(
        "Q1",
        rspice_results::soa_evidence::SoaParameterEvidence::BaseEmitterVoltage,
    );
    assert_eq!(bjt.limit_value, 0.65);
    assert!(bjt.worst_actual_value < 0.7 - 1e-4);
    for (device, p_channel, basis) in [
        ("M9N", false, "intrinsic"),
        ("M9P", true, "intrinsic"),
        ("MVN", false, "authored terminals"),
        ("MVP", true, "authored terminals"),
    ] {
        use rspice_results::soa_evidence::{SoaParameterEvidence::*, SoaRuleVerdictEvidence};
        let forward = find(
            device,
            if p_channel {
                GateSourceVoltageNegative
            } else {
                GateSourceVoltagePositive
            },
        );
        let reverse = find(
            device,
            if p_channel {
                GateSourceVoltagePositive
            } else {
                GateSourceVoltageNegative
            },
        );
        assert_eq!(forward.limit_value, 1.0);
        assert!(
            (forward.worst_actual_value - 1.1).abs() < 1e-8,
            "{device}: {forward:?}"
        );
        assert_eq!(forward.verdict, SoaRuleVerdictEvidence::Violation);
        assert!(forward.description.contains(basis));
        assert_eq!(reverse.limit_value, 0.5);
        assert_eq!(reverse.worst_actual_value, 0.0);
        assert_eq!(reverse.verdict, SoaRuleVerdictEvidence::Pass);
    }
    round_trip_payload(retained);
}

#[test]
fn soa_derating_survives_draft_worker_thermal_transient_and_retained_results() {
    use rspice_results::safety::{SoAParameter, SoaPowerDerating};
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_simulation_contract::soa_rule::SoaRuleConfig;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let curve = SoaPowerDerating {
        reference_temperature_kelvin: 313.15,
        watts_per_kelvin: 0.001,
    };
    let config = SoaConfig {
        observation: rspice_simulation_contract::soa_observation::SoaObservationConfig {
            thresholds: rspice_results::safety::SoaThresholds {
                warning_fraction: None,
                critical_fraction: None,
            },
            ..Default::default()
        },
        stop_time: 1e-9,
        step_time: 1e-10,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        rules: vec![SoaRuleConfig {
            duration_mode: Default::default(),
            minimum_duration_s: Some(1e-12),
            current_envelope: None,
            power_derating: Some(curve),
            voltage_basis: Default::default(),
            parameter: SoAParameter::Pdiss,
            max_value: 0.05,
            devices: vec!["Q1".into()],
            models: vec![],
        }],
        ..Default::default()
    };
    let mut draft = SoaDialogState::from_config(&config);
    assert_eq!(draft.to_config().unwrap(), config);
    assert!(config.to_spice().contains("watts_per_k=0.001"));
    draft.rules[0].derating_temperature_celsius = "-273.15".into();
    assert!(draft.to_config().is_err());
    draft = SoaDialogState::from_config(&config);
    let mut old = serde_json::to_value(&config.rules[0]).unwrap();
    old.as_object_mut().unwrap().remove("power_derating");
    assert!(
        serde_json::from_value::<SoaRuleConfig>(old)
            .unwrap()
            .power_derating
            .is_none()
    );
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let wire: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    assert_eq!(AnalysisSpec::from(wire.clone()), spec);
    spec = AnalysisSpec::from(wire);
    let deck = "Derating\nVc c 0 1.2\nVb b 0 .5\nVth th 0 PWL(0 0 1n 74)\nQ1 c b 0 th vm SW_ET=0\n.model vm NPN LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCI=0 RBI=0 RTH=1000 TMINCLIP=-50 TMAXCLIP=100\n.temp 27\n.end\n";

    let retained = &execute(spec.clone(), deck);
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa {
        source_history: _,
        evaluations,
        violations,
    }) = &retained.result_payload
    else {
        panic!("SOA evidence")
    };
    assert_eq!(evaluations[0].thresholds, config.observation.thresholds);
    assert!(violations.iter().all(|event| event.severity
        == rspice_results::soa_evidence::SoaViolationSeverityEvidence::Violation));
    assert_eq!(evaluations.len(), 1);
    assert_eq!(evaluations[0].derating.unwrap().curve, curve);
    assert_eq!(evaluations[0].limit_value, 0.0);
    let trace = |name: &str| retained.waveforms.iter().find(|w| w.name == name).unwrap();
    let limits = trace("SOA_PDISS_LIMIT(Q1)");
    let temps = trace("SOA_PDISS_TEMPERATURE(Q1)");
    assert_eq!(temps.unit.as_deref(), Some("K"));
    assert_eq!(limits.unit.as_deref(), Some("W"));
    assert!(temps.y.last().unwrap() - temps.y[0] > 70.0);
    assert!(limits.y.contains(&0.05));
    assert!(limits.y.contains(&0.0));
    for (&temp, &limit) in temps.y.iter().zip(limits.y.iter()) {
        assert_eq!(limit, curve.limit(0.05, temp));
    }
    for event in violations {
        let i = limits.x.iter().position(|t| *t == event.time_s).unwrap();
        assert_eq!(event.limit_value, limits.y[i]);
    }
    let mut tampered = retained.clone();
    let wave = tampered
        .waveforms
        .iter_mut()
        .find(|w| w.name == "SOA_PDISS_LIMIT(Q1)")
        .unwrap();
    std::sync::Arc::make_mut(&mut wave.y)[0] += 0.001;
    assert!(tampered.validate_retained_evidence().is_err());
    let mut missing = retained.clone();
    missing
        .waveforms
        .retain(|w| w.name != "SOA_PDISS_TEMPERATURE(Q1)");
    assert!(missing.validate_retained_evidence().is_err());
    round_trip_payload(retained);
}

#[test]
fn soa_vbic_model_ratings_survive_draft_worker_and_retained_results() {
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_results::soa_evidence::SoaParameterEvidence as Parameter;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let config = SoaConfig {
        import_model_voltage_ratings: true,
        stop_time: 1e-9,
        step_time: 1e-10,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        ..Default::default()
    };
    let draft = SoaDialogState::from_config(&config);
    assert_eq!(draft.to_config().unwrap(), config);
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let wire: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    spec = AnalysisSpec::from(wire);
    let mut deck = String::from(
        "VBIC model ratings\nVcn cn 0 1.2\nVbn bn 0 .6\nVsn sn 0 1.4\nVcp cp 0 -1.2\nVbp bp 0 -.6\nVsp sp 0 -1.4\n",
    );
    for level in [4, 9, 11, 12, 13] {
        for (polarity, kind) in [("N", "NPN"), ("P", "PNP")] {
            let nodes = if polarity == "N" {
                if level == 11 {
                    "cn bn 0 0"
                } else {
                    "cn bn 0 sn"
                }
            } else if level == 11 {
                "cp bp 0 0"
            } else {
                "cp bp 0 sp"
            };
            let substrate = if level == 11 {
                ""
            } else {
                "BVSUB=0.15 VSUBFWD=0.1"
            };
            deck.push_str(&format!("Q{polarity}{level} {nodes} M{polarity}{level}\n.model M{polarity}{level} {kind} LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=2 RCI=5 RBX=3 RBI=10 RE=1 BVBE=.5 BVBC=.4 BVCE=1.1 {substrate}\n"));
        }
    }
    // Parameter inference must use the same external convention without LEVEL.
    deck.push_str("QI cn bn 0 sn inferred\n.model inferred NPN IS=1e-16 RCI=5 VBE_MAX=.5 VBC_MAX=.4 VCE_MAX=1.1 VSUB_MAX=.15 VSUBFWD=.1\n.end\n");

    let retained = &execute(spec.clone(), &deck);
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa { evaluations, .. }) =
        &retained.result_payload
    else {
        panic!("SOA evidence")
    };
    assert_eq!(evaluations.len(), 51);
    for evaluation in evaluations {
        assert!(evaluation.description.contains("authored terminals"));
        let (limit, actual) = match evaluation.parameter {
            Parameter::BaseEmitterVoltage => (0.5, 0.6),
            Parameter::BaseCollectorVoltage => (0.4, 0.6),
            Parameter::CollectorEmitterVoltage => (1.1, 1.2),
            Parameter::CollectorSubstrateVoltage => (0.15, 0.2),
            Parameter::CollectorSubstrateVoltagePositive => {
                assert!(evaluation.device_id.starts_with("QP"));
                (0.1, 0.2)
            }
            Parameter::CollectorSubstrateVoltageNegative => {
                assert!(evaluation.device_id.starts_with("QN") || evaluation.device_id == "QI");
                (0.1, 0.2)
            }
            other => panic!("unexpected {other:?}"),
        };
        assert_eq!(evaluation.limit_value, limit);
        assert!(
            (evaluation.worst_actual_value - actual).abs() < 1e-8,
            "{evaluation:?}"
        );
    }
    round_trip_payload(retained);
}

#[test]
fn soa_thresholds_survive_draft_worker_execution_and_retained_results() {
    use rspice_results::safety::SoaThresholds;
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_results::soa_evidence::{
        SoaRuleVerdictEvidence as Verdict, SoaViolationSeverityEvidence as Severity,
    };
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    for (warning, critical, verdict) in [
        (Some(0.5), Some(2.0), Verdict::Violation),
        (None, Some(1.4), Verdict::Critical),
        (None, None, Verdict::Violation),
    ] {
        let thresholds = SoaThresholds {
            warning_fraction: warning,
            critical_fraction: critical,
        };
        let mut config = SoaConfig {
            import_model_voltage_ratings: true,
            stop_time: 1e-9,
            step_time: 1e-10,
            check_vgs_max: false,
            check_vds_max: false,
            check_vbe_max: false,
            check_vce_max: false,
            ..Default::default()
        };
        config.observation.thresholds = thresholds;
        let mut draft = SoaDialogState::from_config(&config);
        assert_eq!(draft.to_config().unwrap(), config);
        draft.warning_percent = "101".into();
        assert!(draft.to_config().is_err());
        draft = SoaDialogState::from_config(&config);
        draft.critical_percent = "99".into();
        assert!(draft.to_config().is_err());
        draft = SoaDialogState::from_config(&config);
        let draft: SoaDialogState = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
        assert_eq!(draft.to_config().unwrap(), config);
        let mut old = serde_json::to_value(&draft).unwrap();
        for name in ["warning_percent", "critical_percent"] {
            old.as_object_mut().unwrap().remove(name);
        }
        let old: SoaDialogState = serde_json::from_value(old).unwrap();
        assert_eq!(
            old.to_config().unwrap().observation.thresholds,
            SoaThresholds::default()
        );
        let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
        let wire = WorkerAnalysisSpec::from(&spec);
        let wire: WorkerAnalysisSpec =
            serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
        assert_eq!(AnalysisSpec::from(wire.clone()), spec);
        spec = AnalysisSpec::from(wire);
        let deck = "SOA thresholds\nV1 a 0 PWL(0 .4 1n 1.5)\nD1 a 0 DM\n.model DM D IS=1e-30 FV_MAX=1\n.end\n";

        let retained = &execute(spec.clone(), deck);
        assert!(retained.success, "{:?}", retained.error_message);
        retained.validate_retained_evidence().unwrap();
        let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa {
            source_history: _,
            evaluations,
            violations,
        }) = &retained.result_payload
        else {
            panic!("SOA evidence")
        };
        assert_eq!(evaluations.len(), 1);
        assert_eq!(evaluations[0].thresholds, thresholds);
        assert_eq!(evaluations[0].verdict, verdict);
        assert!((evaluations[0].worst_actual_value - 1.5).abs() < 1e-8);
        assert_eq!(
            violations
                .iter()
                .any(|event| event.severity == Severity::Warning),
            warning.is_some()
        );
        assert_eq!(
            violations
                .iter()
                .any(|event| event.severity == Severity::Critical),
            verdict == Verdict::Critical
        );
        assert!(violations.iter().any(|event| event.actual_value > 1.0));
        let mut altered = retained.clone();
        let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa {
            evaluations, ..
        }) = &mut altered.result_payload
        else {
            unreachable!()
        };
        evaluations[0].thresholds = SoaThresholds::default();
        assert!(altered.validate_retained_evidence().is_err());
        round_trip_payload(retained);
    }
}

#[test]
fn soa_duration_survives_draft_worker_execution_and_retained_results() {
    check_soa_duration_round_trip(Default::default(), 2e-9, true);
}

#[test]
fn soa_duration_cumulative_survives_draft_worker_execution_and_retained_results() {
    use rspice_results::safety::SoaDurationMode;
    check_soa_duration_round_trip(
        SoaDurationMode::Cumulative {
            recovery_time_s: None,
        },
        4e-9,
        true,
    );
    check_soa_duration_round_trip(
        SoaDurationMode::Cumulative {
            recovery_time_s: Some(1e-10),
        },
        4e-9,
        false,
    );
}

fn check_soa_duration_round_trip(
    mode: rspice_results::safety::SoaDurationMode,
    minimum: f64,
    second_qualified: bool,
) {
    use rspice_results::safety::SoAParameter;
    use rspice_simulation_contract::soa_draft::{SoaConfig, SoaDialogState};

    use rspice_simulation_contract::soa_rule::SoaRuleConfig;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    let config = SoaConfig {
        stop_time: 6e-9,
        step_time: 0.25e-9,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        rules: vec![SoaRuleConfig {
            duration_mode: mode,
            minimum_duration_s: Some(minimum),
            current_envelope: None,
            power_derating: None,
            voltage_basis: Default::default(),
            parameter: SoAParameter::Vds,
            max_value: 1.,
            devices: vec!["M1".into()],
            models: vec![],
        }],
        ..Default::default()
    };
    let mut draft = SoaDialogState::from_config(&config);
    draft.rules[0].minimum_duration = format!("{}n", minimum * 1e9);
    assert_eq!(draft.to_config().unwrap(), config);
    assert!(config.to_spice().contains("min_duration=(VDS"));
    assert_eq!(
        config.to_spice().contains("cumulative_duration=(VDS"),
        !mode.is_default()
    );
    draft.rules[0].recovery_time = "0".into();
    assert_eq!(draft.to_config().is_ok(), mode.is_default());
    // Inactive controls keep their authored buffers without preventing execution.
    draft.rules[0].minimum_duration.clear();
    assert!(
        draft.to_config().unwrap().rules[0]
            .duration_mode
            .is_default()
    );
    draft.rules[0].minimum_duration = "0".into();
    assert!(draft.to_config().is_err());
    let draft = SoaDialogState::from_config(&config);
    let draft: SoaDialogState = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
    let mut old = serde_json::to_value(&config.rules[0]).unwrap();
    old.as_object_mut().unwrap().remove("minimum_duration_s");
    old.as_object_mut().unwrap().remove("duration_mode");
    assert!(
        serde_json::from_value::<SoaRuleConfig>(old)
            .unwrap()
            .minimum_duration_s
            .is_none()
    );
    let mut spec = crate::analysis_preparation::build_soa_spec(&draft).unwrap();
    let wire = WorkerAnalysisSpec::from(&spec);
    let wire: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    assert_eq!(AnalysisSpec::from(wire.clone()), spec);
    spec = AnalysisSpec::from(wire);
    let deck = "SOA durations\nV1 d 0 PWL(0 0 1n 4 2n 0 3n 2 5n 2 6n 0)\nM1 d 0 0 0 NM\n.model NM NMOS LEVEL=1 VTO=1\n.end\n";

    let retained = &execute(spec.clone(), deck);
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa {
        source_history: _,
        evaluations,
        violations,
    }) = &retained.result_payload
    else {
        panic!("SOA evidence")
    };
    let evaluation = &evaluations[0];
    let duration = evaluation.duration.unwrap();
    assert_eq!(duration.mode(), mode);
    assert_eq!(duration.qualified_excursions, u64::from(second_qualified));
    assert_eq!(
        duration.rejected_excursions,
        if second_qualified { 1 } else { 2 }
    );
    assert!((duration.total_exceedance_s - 4.5e-9).abs() < 1e-16);
    assert!((duration.longest_excursion_s - 3e-9).abs() < 1e-16);
    assert!((evaluation.worst_actual_value - if second_qualified { 2. } else { 4. }).abs() < 1e-8);
    assert_eq!(
        evaluation.verdict,
        if second_qualified {
            rspice_results::soa_evidence::SoaRuleVerdictEvidence::Critical
        } else {
            rspice_results::soa_evidence::SoaRuleVerdictEvidence::Warning
        }
    );
    assert!(violations.iter().any(|event| event.actual_value > 3.9
        && event.severity == rspice_results::soa_evidence::SoaViolationSeverityEvidence::Warning));
    let count = retained
        .waveforms
        .iter()
        .find(|wave| wave.name == "SOA_VIOLATION_COUNT")
        .unwrap();
    assert_eq!(count.y.last().copied(), Some(violations.len() as f64));
    let mut tampered = retained.clone();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa { evaluations, .. }) =
        &mut tampered.result_payload
    else {
        unreachable!()
    };
    evaluations[0]
        .duration
        .as_mut()
        .unwrap()
        .longest_excursion_s += 1e-9;
    assert!(tampered.validate_retained_evidence().is_err());
    if let Some(cumulative) = duration.cumulative {
        let peak = cumulative
            .recovery_time_s
            .map_or(4.5e-9, |tau| 3e-9 + 1.5e-9 * (-0.75e-9 / tau).exp());
        let final_exposure = cumulative
            .recovery_time_s
            .map_or(peak, |tau| peak * (-0.5e-9 / tau).exp());
        assert!((cumulative.peak_exposure_s - peak).abs() < 1e-16);
        assert!((cumulative.final_exposure_s - final_exposure).abs() < 1e-16);
        for field in 0..3 {
            let mut tampered = retained.clone();
            let Some(rspice_results::analysis_payload::AnalysisResultPayload::Soa {
                evaluations,
                ..
            }) = &mut tampered.result_payload
            else {
                unreachable!()
            };
            let cumulative = evaluations[0]
                .duration
                .as_mut()
                .unwrap()
                .cumulative
                .as_mut()
                .unwrap();
            match field {
                0 => cumulative.recovery_time_s = Some(1e-9),
                1 => cumulative.peak_exposure_s *= 0.9,
                _ => cumulative.final_exposure_s *= 0.9,
            }
            assert_ne!(
                tampered
                    .result_data_ref()
                    .digest(rspice_results::result_digest::ResultDigestEncoding::CURRENT),
                retained
                    .result_data_ref()
                    .digest(rspice_results::result_digest::ResultDigestEncoding::CURRENT)
            );
            assert!(tampered.validate_retained_evidence().is_err());
        }
    }
    round_trip_payload(retained);
}
