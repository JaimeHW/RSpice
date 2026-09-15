//! Admission contract while native GP transient excess phase is being qualified.
//! These refusals are temporary capability boundaries, not phase qualification.

use rspice_core::engine::{
    CompressionConfig, Engine, SimulationConfig, SpiceDialect, TransientStartupMode,
};
use rspice_core::{Netlist, SimulationError, SimulationErrorCategory, SimulationErrorCode};

fn deck(parameters: &str) -> Netlist {
    Netlist::parse(&format!(
        "GP transient phase\nVC c 0 2\nVB b 0 DC .7 SIN(.7 1u 1G)\nQ1 c b 0 model\n.model model NPN IS=1e-16 BF=100 BR=1 {parameters}\n.options RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 GMIN=0\n.end\n"
    ))
    .unwrap()
}

fn assert_phase_refusal(error: SimulationError, instance: &str) {
    let descriptor = error.descriptor();
    assert_eq!(descriptor.code, SimulationErrorCode::UnsupportedCapability);
    assert_eq!(descriptor.category, SimulationErrorCategory::Capability);
    let SimulationError::UnsupportedCapability(refusal) = &error else {
        panic!("expected a capability refusal: {error}");
    };
    assert_eq!(refusal.capability, "analysis.tran.bjt_excess_phase");
    let message = error.to_string();
    assert!(
        message.contains(instance) && message.contains("PTF"),
        "{message}"
    );
    assert!(
        message.contains("not yet supported") && message.contains("history"),
        "{message}"
    );
}

#[test]
fn gp_transient_must_not_silently_ignore_nonzero_ptf() {
    // The uncorrected engine returned exactly the same 508 time points,
    // voltages and currents for PTF=0 and PTF=90 in this 1 GHz circuit.
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for model in [
            rspice_core::GpTransientPhaseModel::ExactDelay,
            rspice_core::GpTransientPhaseModel::NgspiceWeil,
        ] {
            let mut config = SimulationConfig::default().with_spice_dialect(dialect);
            config.gp_transient_phase_model = model;
            let engine = Engine::new(config);
            for private in ["", "RB=100 RBM=20 IRB=1e-5 RE=1 RC=2"] {
                for phase in ["90", "21", "-21", "1e-200"] {
                    let source = deck(&format!("TF=1n PTF={phase} {private}"));
                    for startup in [
                        TransientStartupMode::OperatingPoint,
                        TransientStartupMode::Uic,
                    ] {
                        assert_phase_refusal(
                            engine
                                .run_tran_with_startup_mode(&source, 2e-9, 4e-12, startup)
                                .unwrap_err(),
                            "Q1",
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn gp_phase_refusal_covers_checkpoint_preflight_and_output_routes() {
    let engine = Engine::default();
    let source = deck("TF=1n PTF=21 RB=100 RBM=20 IRB=1e-5");
    assert_phase_refusal(
        engine.preflight_transient_checkpoint(&source).unwrap_err(),
        "Q1",
    );
    assert_phase_refusal(
        engine
            .run_tran_checkpointed(&source, 2e-9, 4e-12)
            .unwrap_err(),
        "Q1",
    );
    assert_phase_refusal(
        engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &source,
                2e-9,
                4e-12,
                TransientStartupMode::OperatingPoint,
                &[1e-9],
            )
            .unwrap_err(),
        "Q1",
    );
    assert_phase_refusal(
        engine
            .run_tran_compressed(&source, 2e-9, 4e-12, CompressionConfig::default())
            .unwrap_err(),
        "Q1",
    );
}

#[test]
fn gp_zero_phase_preserves_transient_and_checkpoint_behavior() {
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice));
    for private in ["", "RB=100 RBM=20 IRB=1e-5 RE=1 RC=2"] {
        for (base, disabled) in [("TF=1n", "TF=1n PTF=-0"), ("TF=0", "TF=0 PTF=90")] {
            let a = deck(&format!("{base} {private}"));
            let b = deck(&format!("{disabled} {private}"));
            assert!(
                engine
                    .preflight_transient_checkpoint(&b)
                    .unwrap()
                    .is_resumable()
            );
            let a = engine.run_tran(&a, 2e-9, 4e-12).unwrap();
            let (b, checkpoint) = engine.run_tran_checkpointed(&b, 2e-9, 4e-12).unwrap();
            assert_eq!(a.time, b.time);
            assert_eq!(a.voltages, b.voltages);
            assert_eq!(a.branch_currents, b.branch_currents);
            assert_eq!(checkpoint.time, 2e-9);
        }
    }
}

#[test]
fn gp_unused_phase_model_does_not_block_an_unaffected_circuit() {
    let source = Netlist::parse(
        "unused phase model\nV1 in 0 1\nR1 in 0 1k\n.model unused NPN TF=1n PTF=90\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    assert!(
        engine
            .preflight_transient_checkpoint(&source)
            .unwrap()
            .is_resumable()
    );
    engine.run_tran(&source, 1e-9, 1e-10).unwrap();
}

#[test]
fn gp_hierarchical_phase_refusal_names_the_elaborated_instance() {
    let source = Netlist::parse(
        "hierarchical phase\nVC c 0 2\nVB b 0 .7\nXstage c b stage\n.subckt stage c b\nQphase c b 0 local\n.model local NPN TF=1n PTF=21\n.ends\n.end\n",
    ).unwrap();
    let engine = Engine::default();
    let error = engine.run_tran(&source, 1e-9, 1e-10).unwrap_err();
    assert!(
        error
            .to_string()
            .to_ascii_lowercase()
            .contains("xstage.qphase")
    );
    assert_phase_refusal(error, "PTF");
}
