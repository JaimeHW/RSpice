//! Executable inventory for C00. A declaration is not a qualification result.
//! Refresh the periodic snapshot deliberately with
//! RSPICE_UPDATE_CORE_REQUIREMENTS=1 cargo test -p rspice-core --lib requirements.
//! The JSON ledger keeps numerical test-development obligations open.

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::{Value as Json, json};

use super::periodic_capability::{
    CapabilitySupport, PeriodicCapability, PeriodicDeviceFamily, periodic_capability_descriptor,
};
use crate::netlist::AnalysisCommand;

const LEDGER: &str = include_str!("../../tests/testdata/qualification/core-requirements-v1.json");

fn variant(command: &AnalysisCommand) -> &'static str {
    use AnalysisCommand as A;
    match command {
        A::Op => "Op",
        A::Dc { .. } => "Dc",
        A::Ac { .. } => "Ac",
        A::AcData { .. } => "AcData",
        A::Hb(_) => "Hb",
        A::Qpss(_) => "Qpss",
        A::Qpac(_) => "Qpac",
        A::Qpxf(_) => "Qpxf",
        A::Qpnoise(_) => "Qpnoise",
        A::Sp { .. } => "Sp",
        A::Stb { .. } => "Stb",
        A::Disto { .. } => "Disto",
        A::Tran { .. } => "Tran",
        A::Noise { .. } => "Noise",
        A::NoiseData { .. } => "NoiseData",
        A::PoleZero { .. } => "PoleZero",
        A::Sensitivity { .. } => "Sensitivity",
        A::Tf { .. } => "Tf",
        A::Four { .. } => "Four",
        A::MonteCarlo(_) => "MonteCarlo",
        A::Step(_) => "Step",
        A::Temp { .. } => "Temp",
        A::Pss(_) => "Pss",
        A::Pac(_) => "Pac",
        A::Pxf(_) => "Pxf",
        A::Pnoise(_) => "Pnoise",
        A::Pstb(_) => "Pstb",
        A::Envelope(_) => "Envelope",
        A::DcMatch(_) => "DcMatch",
    }
}

/// Exhaustive ownership forces a newly added family to receive a disposition.
fn family_package(family: PeriodicDeviceFamily) -> &'static str {
    use PeriodicDeviceFamily as F;
    match family {
        F::Resistor
        | F::ResistorBranch
        | F::Capacitor
        | F::XyceMemristor
        | F::VoltageSwitch
        | F::CurrentSwitch
        | F::GenericSwitch
        | F::TransmissionLine
        | F::CoupledTransmissionLine
        | F::InductorCoupling
        | F::CoupledInductorPair
        | F::MultiWindingTransformer
        | F::JilesAthertonInductor
        | F::XyceCoreGroup
        | F::BehavioralSource => "C07",
        F::Diode
        | F::Bjt
        | F::Mosfet
        | F::Bsim3v3
        | F::Bsim4v8
        | F::B3SoiDd
        | F::B3SoiFd
        | F::B3SoiPd
        | F::Ekv26
        | F::Ekv3
        | F::Vdmos
        | F::Jfet => "C06",
        F::XspiceInstance | F::RuntimeVerilogA | F::GeneratedVerilogA => "C09",
        F::MixedSignalHost => "C11",
        F::Inductor
        | F::VoltageSource
        | F::CurrentSource
        | F::Vcvs
        | F::Vccs
        | F::Cccs
        | F::Ccvs => "C14",
    }
}

fn predicate(capability: PeriodicCapability) -> &'static str {
    use PeriodicCapability as C;
    match capability {
        C::PeriodicResidualJacobian => "periodic_carrier_residual_gaps",
        C::DynamicStateDescriptor => "dynamic_state_descriptor_gaps",
        C::PeriodicSmallSignalDescriptor => "periodic_carrier_descriptor_gaps",
        C::NoiseSources => "cyclostationary_noise_gaps",
        C::PssStateMap => "pss_state_gaps",
        C::EnvelopeContinuation => "envelope_gaps",
    }
}

#[test]
fn periodic_declarations_and_restrictions_match_requirements() {
    let mut rows = Vec::new();
    for family in PeriodicDeviceFamily::ALL {
        let descriptor = periodic_capability_descriptor(family);
        for capability in PeriodicCapability::ALL {
            let support = descriptor.support(capability);
            let (status, condition) = match support {
                CapabilitySupport::Complete => ("complete", None),
                CapabilitySupport::Inapplicable => ("inapplicable", None),
                CapabilitySupport::Restricted(reason) => ("restricted", Some(reason)),
                CapabilitySupport::Absent(reason) => ("absent", Some(reason)),
            };
            let owner = if support.admits_every_instance() {
                "C14"
            } else if capability == PeriodicCapability::NoiseSources {
                "C08"
            } else {
                family_package(family)
            };
            rows.push(json!([
                format!("{family:?}"),
                format!("{capability:?}"),
                status,
                condition,
                owner,
                predicate(capability)
            ]));
        }
    }
    let actual = json!({
        "schema": "rspice-core-periodic-requirements",
        "version": 1,
        "evidence": "Declarations and admission-query inventory only. Numerical qualification remains required.",
        "columns": ["family", "contract", "declaration", "condition", "package", "instance_predicate"],
        "rows": rows
    });
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/testdata/qualification/core-periodic-requirements-v1.json");
    if std::env::var_os("RSPICE_UPDATE_CORE_REQUIREMENTS").as_deref()
        == Some(std::ffi::OsStr::new("1"))
    {
        // One row per line keeps the full 38-by-six inventory reviewable.
        let mut rendered = serde_json::to_string_pretty(&json!({
            "schema": actual["schema"], "version": actual["version"],
            "evidence": actual["evidence"], "columns": actual["columns"]
        }))
        .unwrap();
        rendered.truncate(rendered.len() - 2);
        rendered.push_str(",\n  \"rows\": [\n");
        let rows = actual["rows"].as_array().unwrap();
        for (index, row) in rows.iter().enumerate() {
            rendered.push_str("    ");
            rendered.push_str(&serde_json::to_string(row).unwrap());
            rendered.push_str(if index + 1 == rows.len() { "\n" } else { ",\n" });
        }
        rendered.push_str("  ]\n}\n");
        std::fs::write(&path, rendered).unwrap();
    }
    let expected: Json = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(
        actual, expected,
        "review every declaration/restriction change and its implementation evidence before updating the requirements snapshot"
    );
}

#[test]
fn analysis_inventory_parses_and_matches_control_execution() {
    use super::control::ControlCircuit;
    use crate::abort_signal::NoAbort;
    use crate::control_protocol::ControlCommand;
    use crate::netlist::ParamContext;

    let ledger: Json = serde_json::from_str(LEDGER).unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut variants = BTreeSet::new();
    for row in ledger["analyses"].as_array().unwrap() {
        let name = row["variant"].as_str().unwrap();
        assert!(variants.insert(name), "duplicate analysis {name}");
        let card = row["card"].as_str().unwrap();
        let source = if name == "Stb" {
            format!(
                "Core stability requirements\nE1 out 0 sense 0 -10\nV1 out drive 0\nR1 drive sense 1k\nC1 sense 0 1u\n{card}\n.end\n"
            )
        } else {
            format!(
                "Core requirements\n.param r=1k\nV1 in 0 1 AC 1 DISTOF1 1m DISTOF2 1m\nR1 in out {{r}}\nR2 out 0 1k\n.data grid freq\n1\n10\n.enddata\n{card}\n.end\n"
            )
        };
        let mut netlist =
            crate::Netlist::parse(&source).unwrap_or_else(|error| panic!("{name}: {error}"));
        if name == "DcMatch" {
            netlist
                .spectre_statistics
                .variations
                .push(crate::netlist::SpectreVariation {
                    line: 1,
                    scope: crate::netlist::SpectreVariationScope::Mismatch,
                    parameter: "r".into(),
                    distribution: crate::netlist::SpectreDistribution::Gaussian,
                    spread: crate::netlist::SpectreSpread::StandardDeviation("10".into()),
                    percent: false,
                    bounds: None,
                });
        }
        let [command] = netlist.analyses.as_slice() else {
            panic!("{name}: inventory card must name exactly one command");
        };
        assert_eq!(variant(command), name);
        assert_eq!(
            ControlCircuit::validate_analysis_support(command, 1).is_ok(),
            row["control_host"] == "implemented",
            "{name}: static admission must match execution"
        );
        // `run` selects standalone producers. Result-dependent Fourier and
        // sweep/temperature cards need a producer; their presence alone does
        // not constitute an executable analysis. Their missing control-host
        // behavior still belongs to C04 and must not be marked implemented.
        let needs_producer = matches!(
            command,
            AnalysisCommand::Four { .. } | AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. }
        );
        let route = &row["route"];
        let implementation =
            std::fs::read_to_string(root.join(route["path"].as_str().unwrap())).unwrap();
        assert!(
            implementation.contains(&format!("pub fn {}(", route["symbol"].as_str().unwrap())),
            "{name}: stale public route reference"
        );
        let mut host = ControlCircuit::new(netlist).unwrap();
        let outcome = host.execute(
            &super::Engine::default(),
            &ControlCommand {
                name: "run".into(),
                arguments: String::new(),
                line: 1,
            },
            &ParamContext::new(),
            &NoAbort,
        );
        match row["control_host"].as_str().unwrap() {
            "implemented" => {
                outcome.unwrap_or_else(|error| panic!("{name}: {error}"));
                assert_eq!(host.datasets().len(), 1);
            }
            "missing" => {
                let error = outcome
                    .expect_err(
                        "new control support needs a ledger update and numerical qualification",
                    )
                    .to_string();
                let expected = if needs_producer {
                    "run has no declarative analysis to execute"
                } else {
                    "no control-host execution handler"
                };
                assert!(error.contains(expected), "{name}: {error}");
                assert!(host.datasets().is_empty());
            }
            status => panic!("unknown control implementation status {status}"),
        }
    }
    assert_eq!(
        variants.len(),
        29,
        "review the inventory when AnalysisCommand grows"
    );
}

#[test]
fn every_requirement_has_an_owner_and_qualification_task() {
    let ledger: Json = serde_json::from_str(LEDGER).unwrap();
    let packages: BTreeSet<_> = ledger["packages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect();
    assert_eq!(packages.len(), 15);
    for index in 0..15 {
        assert!(packages.contains(format!("C{index:02}").as_str()));
    }
    for row in ledger["packages"].as_array().unwrap() {
        assert!(!row["qualification_task"].as_str().unwrap().is_empty());
        assert!(matches!(
            row["status"].as_str(),
            Some("open" | "in_progress" | "qualified")
        ));
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for section in ["analyses", "guards", "ignored_tests"] {
        for row in ledger[section].as_array().unwrap() {
            assert!(packages.contains(row["owner"].as_str().unwrap()));
            if let Some(path) = row["path"].as_str() {
                let source = std::fs::read_to_string(root.join(path)).unwrap();
                assert!(
                    source.contains(row["symbol"].as_str().unwrap()),
                    "stale {section} reference {path}"
                );
            }
        }
    }
    for path in ledger["existing_evidence"].as_array().unwrap() {
        assert!(root.join(path.as_str().unwrap()).is_file());
    }
}
