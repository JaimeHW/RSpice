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
        (
            ".model qmod MEXTRAM504 ()",
            "MEXTRAM 504",
            "bjt504va",
            "veriloga-model-bjt504va",
        ),
        (
            ".model qmod BJT504TVA ()",
            "MEXTRAM 504 self-heating",
            "bjt504tva",
            "veriloga-model-bjt504tva",
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

/// A built-in self-heating model's thermal terminal sits on a net whose value
/// is a temperature, listed in kelvin as a runtime module's is.
#[cfg(feature = "veriloga-model-bjt505t-va")]
#[test]
fn a_built_in_thermal_net_is_listed_in_kelvin() {
    let deck = "\
* MEXTRAM 505 self-heating, thermal net on the card
VCC c 0 DC 1
VB b 0 DC 0
RB b bx 10k
RC c cx 1k
Q1 cx bx 0 0 dt qmod
RT dt 0 1G
.model qmod MEXTRAM505T ()
.op
.end
";
    let netlist = Netlist::parse(deck).expect("self-heating MEXTRAM deck parses");
    let result = engine()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    let signals = rspice_core::execution::operating_point_projection_signals(&result)
        .expect("the operating point lists its values");
    let unit = |name: &str| {
        signals
            .iter()
            .find(|signal| {
                signal
                    .descriptor()
                    .display_name()
                    .eq_ignore_ascii_case(name)
            })
            .unwrap_or_else(|| panic!("{name} is not listed"))
            .descriptor()
            .unit()
            .symbol()
    };
    assert_eq!(unit("V(dt)"), "K");
    assert_eq!(unit("V(cx)"), "V");
}

/// Xyce registers Mextram 504.12.1 as Q LEVEL=504. Under the Xyce dialect a
/// LEVEL=504 card runs that release's own module, a PNP card as a PNP, and is
/// never handed to Mextram 505.
#[cfg(feature = "veriloga-model-bjt504va")]
#[test]
fn an_xyce_level_504_card_runs_mextram_504_with_the_card_polarity() {
    use rspice_core::engine::SpiceDialect;
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let mut collector_currents = Vec::new();
    for (polarity, vcc, vb) in [("NPN", 1.0, 0.75), ("PNP", -1.0, -0.75)] {
        let deck = format!(
            "* Xyce Mextram 504 by level
             VCC c 0 DC {vcc}
             VB b 0 DC {vb}
             Q1 c b 0 0 qmod
             .model qmod {polarity} LEVEL=504
             .end
"
        );
        let netlist = Netlist::parse(&deck).expect("Mextram 504 deck parses");
        let circuit = engine
            .build_circuit(&netlist)
            .expect("an Xyce LEVEL=504 card builds through the generated backend");
        let report = circuit.device_op_report();
        let entry = report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case("Q1"))
            .expect("Q1 is reported");
        assert_eq!(entry.device_kind.to_ascii_uppercase(), "BJT504VA");
        let op = engine
            .run_dc_op(&netlist)
            .expect("the Mextram 504 operating point solves");
        let current = op
            .branch_currents
            .iter()
            .zip(&op.branch_names)
            .find(|(_, name)| name.eq_ignore_ascii_case("VCC"))
            .map(|(current, _)| *current)
            .expect("VCC branch current");
        collector_currents.push(current);
    }
    // A forward-biased NPN draws collector current out of VCC's positive
    // terminal and the mirrored PNP returns it: the two currents are equal and
    // opposite only if the PNP card ran with TYPE=-1.
    assert!(
        collector_currents[0].abs() > 1.0e-9,
        "{collector_currents:?}"
    );
    assert!(
        (collector_currents[0] + collector_currents[1]).abs()
            <= 1.0e-9 * collector_currents[0].abs(),
        "NPN and PNP Mextram 504 must mirror: {collector_currents:?}"
    );
}

/// Xyce's PSP LEVELs reach the PSP releases it registers them for, and a
/// PMOS card runs as a PMOS. PSP's defaults are not polarity-symmetric, so
/// the default NMOS at +1 V and PMOS at -1 V draw different currents; each
/// must be the one Xyce 7.10 draws for the same card.
#[cfg(all(
    feature = "veriloga-model-psp102va",
    feature = "veriloga-model-psp103va",
    feature = "veriloga-model-psp103tva"
))]
#[test]
fn xyce_psp_levels_run_their_release_with_the_card_polarity() {
    use rspice_core::engine::SpiceDialect;
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    for (level, module, xyce) in [
        ("102", "PSP102VA", [-5.58343334e-5, 5.33090534e-5]),
        ("103", "PSP103VA", [-1.11098110e-4, 1.05259187e-4]),
        ("1031", "PSP103TVA", [-1.11098110e-4, 1.05259187e-4]),
    ] {
        let mut drain_currents = Vec::new();
        // PSP103TVA has a fifth, thermal terminal.
        let thermal = if level == "1031" { " t" } else { "" };
        for (polarity, v) in [("NMOS", 1.0), ("PMOS", -1.0)] {
            let deck = format!(
                "* Xyce PSP by level
                 VD d 0 DC {v}
                 VG g 0 DC {v}
                 M1 d g 0 0{thermal} mmod W=1u L=1u
                 .model mmod {polarity} LEVEL={level}
                 .end
"
            );
            let netlist = Netlist::parse(&deck).expect("PSP deck parses");
            let circuit = engine
                .build_circuit(&netlist)
                .expect("an Xyce PSP LEVEL card builds through the generated backend");
            let report = circuit.device_op_report();
            let entry = report
                .entries
                .iter()
                .find(|entry| entry.name.eq_ignore_ascii_case("M1"))
                .expect("M1 is reported");
            assert_eq!(
                entry.device_kind.to_ascii_uppercase(),
                module,
                "LEVEL={level}"
            );
            let op = engine
                .run_dc_op(&netlist)
                .expect("the PSP operating point solves");
            let current = op
                .branch_currents
                .iter()
                .zip(&op.branch_names)
                .find(|(_, name)| name.eq_ignore_ascii_case("VD"))
                .map(|(current, _)| *current)
                .expect("VD branch current");
            drain_currents.push(current);
        }
        for (current, expected) in drain_currents.iter().zip(xyce) {
            assert!(
                (current - expected).abs() <= 1.0e-6 * expected.abs(),
                "LEVEL={level}: {drain_currents:?}, Xyce draws {xyce:?}"
            );
        }
    }
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

#[cfg(all(
    feature = "veriloga-model-vbic13",
    feature = "veriloga-model-vbic13-3t-et",
    feature = "veriloga-model-vbic13-4t"
))]
mod xyce_vbic_generated {
    use super::*;
    use rspice_core::engine::{ConvergenceConfig, SpiceDialect};

    #[test]
    fn multiplier_aliases_match_parallel_devices_in_dc_and_ac() {
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            convergence_config: ConvergenceConfig {
                // A circuit-level shunt does not scale with device count.
                gmin_target: 0.0,
                ..Default::default()
            },
            ..Default::default()
        });
        for (level, extra) in [(11, ""), (11, " th"), (12, " 0"), (12, " 0 th")] {
            let deck = |parameters: &str| {
                Netlist::parse(&format!(
                "Generated VBIC multiplicity\nVc c 0 1.5\nVb b 0 DC 0.6 AC 1\nVth th 0 0\n\
                 .param scale_a=2 scale_m=3\n\
                 Q1 c b 0{extra} vm SW_ET=0 {parameters}\n\
                 .model vm NPN LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=10 RBX=10 RBI=10 RE=10 RBP=10 RS=10 CJE=1p CJC=1p\n.end\n"
            )).unwrap()
            };
            let solve = |parameters: &str| {
                let netlist = deck(parameters);
                (
                    engine.run_dc_op(&netlist).unwrap(),
                    engine.run_ac(&netlist, &[1e7]).unwrap(),
                )
            };
            // ngspice-46's VBIC takes area and m as instance parameters and
            // gives 6.000x the unit current for area=2 m=3 (positional area
            // too). MULT is another name for M.
            for (parameters, reference, factor) in [
                ("AREA=2 M=3", "M=1", 6.0),
                ("2 M=3", "M=1", 6.0),
                ("AREA={scale_a} M={scale_m}", "M=1", 6.0),
                ("MULT=4", "M=4", 1.0),
            ] {
                let (dc_unit, ac_unit) = solve(reference);
                let (dc, ac) = solve(parameters);
                for branch in ["VC", "VB"] {
                    let expected = factor * dc_unit.branch_current_named(branch).unwrap();
                    let actual = dc.branch_current_named(branch).unwrap();
                    assert!(
                        (actual - expected).abs() <= 1e-8 * expected.abs().max(1e-20),
                        "LEVEL={level}{extra}, {parameters}, DC {branch}: {actual:e} != {expected:e}"
                    );
                    let column = ac_unit[0]
                        .branch_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case(branch))
                        .unwrap();
                    let expected = ac_unit[0].currents[column] * factor;
                    let column = ac[0]
                        .branch_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case(branch))
                        .unwrap();
                    let actual = ac[0].currents[column];
                    assert!(
                        (actual - expected).norm() <= 1e-8 * expected.norm().max(1e-20),
                        "LEVEL={level}{extra}, {parameters}, AC {branch}: {actual:?} != {expected:?}"
                    );
                }
            }
        }
    }

    /// M is the multiplicity factor Xyce's VBIC 1.3 itself declares, so a
    /// card that sets it answers with Xyce's currents.
    #[test]
    fn the_multiplicity_factor_matches_xyce() {
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..Default::default()
        });
        // Xyce 7.10, `.DC Vc 1.5 1.5 1`, printing I(Vc) and I(Vb).
        for (multiplicity, xyce) in [
            (1, [-1.18658207e-06, -1.18642938e-08]),
            (3, [-3.55974621e-06, -3.55928813e-08]),
        ] {
            let netlist = Netlist::parse(&format!(
                "Generated VBIC multiplicity\nVc c 0 1.5\nVb b 0 0.6\n\
                 Q1 c b 0 vm SW_ET=0 M={multiplicity}\n\
                 .model vm NPN LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=10 RBX=10 RBI=10 RE=10 RBP=10 RS=10 CJE=1p CJC=1p\n.end\n"
            ))
            .unwrap();
            let dc = engine.run_dc_op(&netlist).unwrap();
            for (branch, expected) in ["VC", "VB"].into_iter().zip(xyce) {
                let actual = dc.branch_current_named(branch).unwrap();
                assert!(
                    (actual - expected).abs() <= 1e-6 * expected.abs(),
                    "M={multiplicity}, {branch}: {actual:e} != Xyce {expected:e}"
                );
            }
        }
    }

    #[test]
    fn every_optional_node_connects_to_the_generated_equations() {
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            convergence_config: ConvergenceConfig {
                // The final GMIN floor shunts a circuit node but not a net the
                // instance owns, and neither Xyce nor ngspice adds that floor.
                gmin_target: 0.0,
                ..Default::default()
            },
            ..Default::default()
        });
        let exposed_name = |index| {
            if index == 1 {
                "0:1".to_string()
            } else {
                format!("exposed_{index}")
            }
        };
        let optional = [
            ("dt", 20.0),
            ("cx", 1.4),
            ("ci", 1.3),
            ("bx", 0.58),
            ("bi", 0.55),
            ("ei", 0.01),
        ];
        for level in [11, 12] {
            let substrate = if level == 12 { " 0" } else { "" };
            for count in 0..=optional.len() {
                // Independently attach the same voltage sources to the
                // canonical internal names and to the optional Q terminals.
                let deck = |exposed: bool| {
                    let connections = if exposed {
                        (0..count)
                            .map(|index| format!(" {}", exposed_name(index)))
                            .collect::<String>()
                    } else {
                        String::new()
                    };
                    let mut deck = format!(
                        "VBIC optional connections\nVc c 0 1.5\nVb b 0 0.6\nQ1 c b 0{substrate}{connections} vm SW_ET=0\n\
                         .model vm NPN LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=10 RBX=10 RBI=10 RE=10 RBP=10 RS=10 RTH=1000\n"
                    );
                    for (index, (name, value)) in optional[..count].iter().enumerate() {
                        let node = if exposed {
                            exposed_name(index)
                        } else {
                            format!("Q1.__{name}.internal")
                        };
                        deck.push_str(&format!("Vdrive{index} {node} 0 {value}\n"));
                    }
                    deck.push_str(".end\n");
                    let mut netlist = Netlist::parse(&deck).unwrap();
                    if !exposed {
                        // Authored labels are normalized by the parser; use
                        // the generated adapter's exact internal spelling.
                        for element in &mut netlist.elements {
                            for node in &mut element.nodes {
                                for (name, _) in &optional[..count] {
                                    let internal = format!("Q1.__{name}.internal");
                                    if node.eq_ignore_ascii_case(&internal) {
                                        *node = internal;
                                    }
                                }
                            }
                        }
                    }
                    netlist
                };
                let reference = engine.run_dc_op(&deck(false)).unwrap();
                let actual = engine.run_dc_op(&deck(true)).unwrap();
                for branch in &reference.branch_names {
                    let expected = reference.branch_current_named(branch).unwrap();
                    let value = actual.branch_current_named(branch).unwrap();
                    assert!(
                        (expected - value).abs() <= 1e-10 * expected.abs().max(1e-10),
                        "LEVEL={level}, {count} optional nodes, {branch}: {expected:e} != {value:e}"
                    );
                }
            }
        }
    }

    #[cfg(feature = "veriloga-builtins-noise")]
    #[test]
    fn default_card_common_emitter_stages_reach_xyce_noise() {
        // A default-card VBIC 1.3 stage reaches its operating point only when
        // the model's own pnjlim limiting owns the Newton steps.
        // Xyce 7.10, OutputData/VANOISE/<deck>.cir.NOISE.prn: frequency,
        // sqrt(ONOISE) and sqrt(INOISE) at the first and last rows.
        let rows = [
            (1.0e-2, 1.10013248e-07, 8.27098679e-05),
            (9.93340009e+02, 1.26368536e-08, 3.20339250e-09),
        ];
        for (name, source) in [
            (
                "commonEmitterBjt_vbic13",
                include_str!("../../../tests/xyce/Netlists/VANOISE/commonEmitterBjt_vbic13.cir"),
            ),
            (
                "commonEmitterBjt_vbic13_3T",
                include_str!("../../../tests/xyce/Netlists/VANOISE/commonEmitterBjt_vbic13_3T.cir"),
            ),
        ] {
            let netlist = Netlist::parse_with_options(
                source,
                rspice_core::netlist::NetlistParseOptions {
                    expression_dialect: rspice_core::config::ExpressionDialect::Xyce,
                    ..Default::default()
                },
            )
            .unwrap_or_else(|error| panic!("{name} parses: {error}"));
            // The Xyce suite's solver settings: device RELTOL=1e-4.
            let mut convergence_config = ConvergenceConfig::robust();
            convergence_config.voltage_reltol = 1.0e-4;
            let engine = Engine::new(SimulationConfig {
                spice_dialect: SpiceDialect::Xyce,
                convergence_config,
                max_iterations: SimulationConfig::default().max_iterations.max(1200),
                temperature: 300.15,
                ..Default::default()
            });
            let frequencies = rows.map(|(frequency, _, _)| frequency);
            let points = engine
                .run_noise_named_with_input_source(
                    &netlist,
                    "OUT",
                    None,
                    "V1",
                    &frequencies,
                    300.15,
                )
                .unwrap_or_else(|error| panic!("{name} noise runs: {error}"));
            for (point, (frequency, onoise, inoise)) in points.iter().zip(rows) {
                for (label, got, expected) in [
                    ("sqrt(ONOISE)", point.output_noise_density.sqrt(), onoise),
                    ("sqrt(INOISE)", point.input_referred_density.sqrt(), inoise),
                ] {
                    assert!(
                        ((got - expected) / expected).abs() <= 1e-3,
                        "{name} {label} at {frequency} Hz: {got:e}, Xyce 7.10 gives {expected:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn too_many_optional_q_terminals_are_rejected() {
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..Default::default()
        });
        for (level, maximum) in [(11, 9), (12, 10)] {
            let nodes = (0..=maximum)
                .map(|index| format!(" n{index}"))
                .collect::<String>();
            let netlist = Netlist::parse(&format!(
                "VBIC terminal count\nQ1{nodes} vm\n.model vm NPN LEVEL={level}\n.end\n"
            ))
            .unwrap();
            let error = engine.build_circuit(&netlist).unwrap_err().to_string();
            assert!(
                error.contains(&format!("at most {maximum} terminals")),
                "{error}"
            );
        }
    }
}
