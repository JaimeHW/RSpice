//! Where a compact BJT model's equations come from, and what happens when
//! this build does not carry them.
//!
//! HICUM and MEXTRAM are delivered by the Verilog-A model program: RSpice owns
//! the parser/routing, the result identity, and the analysis-capability
//! integration for them, and never a hand-written approximation of their
//! equations. A `.MODEL` card naming one of those families therefore has
//! exactly two outcomes — it routes to the compiled generated module, or it is
//! refused with a rejection that names the module and the feature that
//! supplies it. It must never fall back to the native Gummel-Poon or VBIC
//! equations, which are a different model.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn bjt_deck(model_card: &str) -> String {
    format!(
        "* compact-model routing boundary\n\
         VCC c 0 DC 2\n\
         VB b 0 DC 0.7\n\
         RB b bx 1k\n\
         Q1 c bx 0 qmod\n\
         {model_card}\n\
         .end\n"
    )
}

/// Whether this build compiled one generated module, so a case that tests the
/// refusal can step aside when the module is actually present.
fn module_is_compiled(module: &str) -> bool {
    #[cfg(feature = "veriloga-builtins-base")]
    {
        rspice_core::device::veriloga_builtins::builtins::builtin_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case(module))
    }
    #[cfg(not(feature = "veriloga-builtins-base"))]
    {
        let _ = module;
        false
    }
}

fn build_error(model_card: &str) -> String {
    let netlist = Netlist::parse(&bjt_deck(model_card)).expect("routing fixture parses");
    engine()
        .build_circuit(&netlist)
        .expect_err("a model this build does not carry must be refused")
        .to_string()
}

/// Every Verilog-A-delivered BJT family this build does not compile is refused
/// by name, whichever alias the deck used for the model type.
#[test]
fn a_verilog_a_delivered_bjt_family_names_its_module_and_feature() {
    let cases = [
        (
            ".model qmod HICUML2 ()",
            "HICUM/L2",
            "hicumL2va",
            "veriloga-model-hicuml2va",
        ),
        (
            ".model qmod hicumL2va ()",
            "HICUM/L2",
            "hicumL2va",
            "veriloga-model-hicuml2va",
        ),
        (
            ".model qmod HICUML0 ()",
            "HICUM/L0",
            "hicumL0va",
            "veriloga-model-hicuml0va",
        ),
        (
            ".model qmod MEXTRAM505 ()",
            "MEXTRAM 505",
            "bjt505_va",
            "veriloga-model-bjt505-va",
        ),
        (
            ".model qmod BJT505T_VA ()",
            "MEXTRAM 505 self-heating",
            "bjt505t_va",
            "veriloga-model-bjt505t-va",
        ),
    ];

    let mut refusals = 0usize;
    for (model_card, family, module, feature) in cases {
        // A build that compiled this module routes the card instead of
        // refusing it; the boundary under test is the refusal.
        if module_is_compiled(module) {
            continue;
        }
        refusals += 1;
        let message = build_error(model_card);
        assert!(
            message.contains(family),
            "{model_card}: the rejection must name the compact-model family: {message}"
        );
        assert!(
            message.contains(module),
            "{model_card}: the rejection must name the generated module: {message}"
        );
        assert!(
            message.contains(feature),
            "{model_card}: the rejection must name the feature that supplies it: {message}"
        );
        assert!(
            message.contains("Verilog-A model program")
                && message.contains("not compiled into this build"),
            "{model_card}: the rejection must say who owns the model and why it is absent: {message}"
        );
        assert!(
            message.contains("must not fall back"),
            "{model_card}: the rejection must forbid substituting another BJT model: {message}"
        );
        assert!(
            !message.contains("expected NPN, PNP, or LPNP"),
            "{model_card}: a delivered compact model is not a malformed type name: {message}"
        );
    }
    assert!(
        refusals > 0,
        "no build compiles every delivered BJT module, so at least one case must exercise the \
         routing boundary"
    );
}

/// A model type nobody delivers still reads as a malformed card, so the new
/// boundary did not swallow the ordinary diagnostic.
#[test]
fn an_unknown_bjt_model_type_still_reads_as_a_malformed_card() {
    let message = build_error(".model qmod NOTABJT ()");
    assert!(
        message.contains("incompatible type") && message.contains("expected NPN, PNP, or LPNP"),
        "an unrecognized type must keep its own diagnostic: {message}"
    );
}

/// The self-heating MEXTRAM variant is a separately compiled module, so it has
/// its own routing identity: `MEXTRAM505T` must select `bjt505t_va` and not
/// the isothermal `bjt505_va`.
#[cfg(feature = "veriloga-model-bjt505t-va")]
#[test]
fn the_self_heating_mextram_card_routes_to_its_own_module() {
    let deck = "\
* MEXTRAM 505 with self-heating
VCC c 0 DC 1
VB b 0 DC 0
RB b bx 10k
RC c cx 1k
Q1 cx bx 0 0 qmod
.model qmod MEXTRAM505T ()
.end
";
    let netlist = Netlist::parse(deck).expect("self-heating MEXTRAM deck parses");
    let circuit = engine()
        .build_circuit(&netlist)
        .expect("MEXTRAM505T builds through its own generated module");
    assert!(circuit.has_generated_veriloga_devices());
    let report = circuit.device_op_report();
    let entries = report
        .entries
        .iter()
        .filter(|entry| entry.name.eq_ignore_ascii_case("Q1"))
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].device_kind.to_ascii_uppercase(),
        "BJT505T_VA",
        "the self-heating card must not route to the isothermal module"
    );
}

#[cfg(feature = "veriloga-model-bjt505-va")]
mod compiled_mextram {
    use super::*;
    use rspice_core::analysis::PssConfig;
    use rspice_core::analysis::harmonic_balance::HbConfig;
    use rspice_core::device::veriloga_builtins::builtins;

    const MEXTRAM_DECK: &str = "\
* MEXTRAM 505 routed to its compiled generated module
VCC c 0 DC 1
VB b 0 DC 0 AC 1 SIN(0 0.01 1meg)
RB b bx 10k
RC c cx 1k
Q1 cx bx 0 0 qmod
.model qmod MEXTRAM505 ()
.end
";

    fn netlist() -> Netlist {
        Netlist::parse(MEXTRAM_DECK).expect("MEXTRAM routing deck parses")
    }

    /// The card selects the compiled module and nothing else: no native BJT is
    /// created alongside it, and the operating-point report identifies the
    /// generated device by its module name.
    #[test]
    fn a_mextram_card_routes_to_the_compiled_generated_module() {
        assert!(
            builtins::builtin_names()
                .iter()
                .any(|name| name.eq_ignore_ascii_case("bjt505_va")),
            "the MEXTRAM 505 generated artifact must be compiled under its feature"
        );

        let circuit = engine()
            .build_circuit(&netlist())
            .expect("a MEXTRAM 505 card builds through the generated backend");
        assert!(circuit.has_generated_veriloga_devices());
        let report = circuit.device_op_report();
        let entries = report
            .entries
            .iter()
            .filter(|entry| entry.name.eq_ignore_ascii_case("Q1"))
            .collect::<Vec<_>>();
        assert_eq!(
            entries.len(),
            1,
            "MEXTRAM 505 must produce exactly one device report, not a generated model plus a \
             native BJT"
        );
        assert_eq!(
            entries[0].device_kind.to_ascii_uppercase(),
            "BJT505_VA",
            "the report must identify the generated module that owns the equations"
        );
    }

    #[test]
    fn op_ac_and_tran_results_retain_the_authored_signal_identity() {
        let netlist = netlist();
        let op = engine()
            .run_dc_op(&netlist)
            .expect("the routed MEXTRAM operating point solves");
        for node in ["C", "B", "BX", "CX"] {
            assert!(
                op.node_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(node)),
                "the operating point must retain node '{node}': {:?}",
                op.node_names
            );
        }
        assert!(op.node_voltages.iter().all(|value| value.is_finite()));

        let ac = engine()
            .run_ac(&netlist, &[1.0e3, 1.0e6])
            .expect("the routed MEXTRAM small-signal sweep solves");
        assert_eq!(ac.len(), 2);
        for point in &ac {
            assert!(
                point
                    .node_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case("CX")),
                "each AC point must retain the authored output node"
            );
            assert!(
                point
                    .voltages
                    .iter()
                    .all(|value| value.re.is_finite() && value.im.is_finite())
            );
        }

        let tran = engine()
            .run_tran(&netlist, 3.0e-6, 1.0e-8)
            .expect("the routed MEXTRAM transient solves");
        assert!(tran.time.len() > 10);
        assert!(
            tran.node_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("CX")),
            "the transient result must retain the authored output node"
        );
    }

    /// The periodic analyses answer from the declared capability table: a
    /// generated Verilog-A compact model has no exact periodic MNA descriptor,
    /// so harmonic balance refuses it by name instead of linearizing something
    /// else.
    #[test]
    fn the_periodic_analyses_refuse_a_generated_compact_model_by_capability() {
        let netlist = netlist();
        let hb = engine()
            .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
            .expect_err("a generated compact model has no exact periodic descriptor")
            .to_string();
        assert!(
            hb.contains("generated Verilog-A compact-model equations"),
            "harmonic balance must name the missing capability: {hb}"
        );

        let pss = engine()
            .run_pss_with_continuation_state(
                &netlist,
                PssConfig::new(1.0e6)
                    .with_harmonics(2)
                    .with_points_per_period(16)
                    .with_tstab_periods(0)
                    .with_tolerance(1.0e-6),
            )
            .expect_err("a generated compact model's integration state is not captured")
            .to_string();
        assert!(
            pss.contains("generated Verilog-A integration state"),
            "the PSS period map must name the uncaptured state: {pss}"
        );
    }
}
