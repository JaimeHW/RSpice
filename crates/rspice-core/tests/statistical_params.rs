//! End-to-end tests for the statistical expression functions
//! (`gauss`/`agauss`/`unif`/`aunif`/2-arg `limit`) and `.options seed=`.
//!
//! These pin the contracts a Monte-Carlo / mismatch flow depends on:
//! every instance evaluation draws a distinct value from one netlist-wide
//! stream, the whole sequence is reproducible for a given seed, and the
//! seed option works no matter where it appears in the deck.

use rspice_core::netlist::{ElementKind, Netlist, flatten_netlist};

/// Collect flattened resistor values in netlist order.
fn flattened_resistor_values(netlist: &Netlist) -> Vec<f64> {
    flatten_netlist(netlist)
        .expect("flatten")
        .iter()
        .filter_map(|element| match &element.kind {
            ElementKind::Resistor { value, .. } => Some(*value),
            _ => None,
        })
        .collect()
}

#[test]
fn instance_param_draws_are_distinct_and_reproducible() {
    let deck = "\
* per-instance mismatch via agauss
.options seed=7
.subckt unit a b mult=1
r1 a b {1k*mult}
.ends
x1 1 0 unit mult={agauss(1,0.05,1)}
x2 2 0 unit mult={agauss(1,0.05,1)}
.end
";
    let first = flattened_resistor_values(&Netlist::parse(deck).expect("parse"));
    assert_eq!(first.len(), 2, "expected two flattened resistors");
    assert_ne!(
        first[0], first[1],
        "instances must draw distinct mismatch values"
    );
    for v in &first {
        assert!(
            (v - 1000.0).abs() < 500.0,
            "draw {v} implausibly far from nominal 1k (agauss sigma 50)"
        );
    }

    // Identical deck, identical seed: identical draws.
    let replay = flattened_resistor_values(&Netlist::parse(deck).expect("parse"));
    assert_eq!(first, replay, "same seed must reproduce the same draws");

    // Different seed: different draws.
    let reseeded_deck = deck.replace("seed=7", "seed=8");
    let reseeded = flattened_resistor_values(&Netlist::parse(&reseeded_deck).expect("parse"));
    assert_ne!(first, reseeded, "a different seed must change the draws");
}

#[test]
fn param_statements_share_one_seeded_stream() {
    let deck = "\
* params drawing from the shared stream
.options seed=3
.param a={agauss(0,1,1)}
.param b={agauss(0,1,1)}
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let a = netlist.params.get("a").expect("param a");
    let b = netlist.params.get("b").expect("param b");
    assert_ne!(a, b, "consecutive .param draws must differ");

    let netlist2 = Netlist::parse(deck).expect("parse");
    assert_eq!(a, netlist2.params.get("a").unwrap());
    assert_eq!(b, netlist2.params.get("b").unwrap());
}

#[test]
fn seed_option_is_position_independent() {
    let seed_first = "\
* seed before params
.options seed=11
.param a={aunif(1,0.5)}
.end
";
    let seed_last = "\
* seed after params
.param a={aunif(1,0.5)}
.options seed=11
.end
";
    let first = Netlist::parse(seed_first).expect("parse");
    let last = Netlist::parse(seed_last).expect("parse");
    assert_eq!(
        first.params.get("a"),
        last.params.get("a"),
        ".options seed must apply regardless of its position in the deck"
    );
    assert_eq!(first.options.seed, Some(11));
    assert_eq!(last.options.seed, Some(11));
}

#[test]
fn seed_option_supports_continuation_and_rndseed_alias() {
    let deck = "\
* seed via continuation line
.options reltol=1e-4
+ rndseed = 21
.param a={aunif(0,1)}
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(netlist.options.seed, Some(21));

    let explicit = "\
* same seed inline
.options rndseed=21
.param a={aunif(0,1)}
.end
";
    let other = Netlist::parse(explicit).expect("parse");
    assert_eq!(
        netlist.params.get("a"),
        other.params.get("a"),
        "continuation-line seed must behave like the inline form"
    );
}

#[test]
fn flattener_resolves_deferred_draws_per_instance() {
    // `nomm` is defined after the instances, so the instance expressions
    // cannot resolve at parse time and must be drawn by the flattener —
    // exercising the shared-stream handoff into hierarchy expansion.
    let deck = "\
* deferred draws resolved during flattening
.options seed=5
.subckt unit a b mult=1
r1 a b {1k*mult}
.ends
x1 1 0 unit mult={aunif(nomm,0.2)}
x2 2 0 unit mult={aunif(nomm,0.2)}
.param nomm=1
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let values = flattened_resistor_values(&netlist);
    assert_eq!(values.len(), 2, "expected two flattened resistors");
    assert_ne!(
        values[0], values[1],
        "flatten-time draws must be distinct per instance"
    );
    for v in &values {
        assert!(
            (v - 1000.0).abs() <= 200.0 + 1e-9,
            "resistor {v} outside aunif(1,0.2)*1k bounds"
        );
    }

    // Full parse+flatten reproducibility.
    let replay = flattened_resistor_values(&Netlist::parse(deck).expect("parse"));
    assert_eq!(values, replay, "parse+flatten must be reproducible");
}

#[test]
fn invalid_seed_values_are_rejected() {
    let negative = "\
* bad seed
.options seed=-3
.end
";
    assert!(
        Netlist::parse(negative).is_err(),
        "negative seed must be rejected"
    );

    let fractional = "\
* bad seed
.options seed=1.5
.end
";
    assert!(
        Netlist::parse(fractional).is_err(),
        "fractional seed must be rejected"
    );
}

#[test]
fn full_width_seed_literals_preserve_the_recorded_integer() {
    for seed in [
        0_u64,
        1,
        (1_u64 << 53) - 1,
        1_u64 << 53,
        (1_u64 << 53) + 1,
        u64::MAX - 1,
        u64::MAX,
    ] {
        for key in ["seed", "rndseed"] {
            let deck = format!("full-width seed\n.options {key}={seed}\n.end\n");
            let parsed = Netlist::parse(&deck).expect("a u64 seed is valid");
            assert_eq!(parsed.options.seed, Some(seed), "{key}={seed} was rounded");
        }
    }
}

#[test]
fn full_width_seed_prescan_selects_the_authored_random_stream() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};

    for seed in [(1_u64 << 53) + 1, u64::MAX - 1, u64::MAX] {
        let mut context = ParamContext::new();
        context.set_random_seed(seed);
        let expected = eval_expression("aunif(0,1)", &context).unwrap();
        for options in [
            format!(".options seed={seed}\n.param sample={{aunif(0,1)}}"),
            format!(".param sample={{aunif(0,1)}}\n.options seed={seed}"),
            format!(
                ".options seed=1\n.param sample={{aunif(0,1)}}\n.options reltol=1e-4\n+ rndseed = {seed}"
            ),
        ] {
            let deck = format!("full-width seeded draw\n{options}\n.end\n");
            let parsed = Netlist::parse(&deck).expect("full-width seeded deck parses");
            assert_eq!(
                parsed.params.get("sample").unwrap().to_bits(),
                expected.to_bits(),
                "the pre-scan selected a different random stream for {seed}"
            );
        }
    }
}

#[test]
fn seed_overflow_and_noninteger_literals_are_rejected_without_rounding() {
    for value in [
        "18446744073709551616",
        "18446744073709551617",
        "9007199254740993.5",
        "1.0000000001",
        "0.0000000001",
    ] {
        let deck = format!("invalid seed\n.options seed={value}\n.end\n");
        assert!(
            Netlist::parse(&deck).is_err(),
            "invalid seed {value} was accepted"
        );
    }
}

#[test]
fn exact_seed_notation_preserves_decimal_exponents_and_engineering_scales() {
    for (literal, expected) in [
        ("1.0", 1_u64),
        ("+001.000", 1),
        (".1k", 100),
        ("1e3", 1000),
        ("1e3k", 1_000_000),
        ("5e6MIL", 127),
        ("1e7MIL", 254),
        ("1000m", 1),
        ("1e15f", 1),
        ("0", 0),
        ("-0.0", 0),
        ("9007199254740993.000", (1_u64 << 53) + 1),
        ("9.007199254740993e15", (1_u64 << 53) + 1),
        ("9007199254740.993k", (1_u64 << 53) + 1),
        ("18446744073709551615.0", u64::MAX),
        ("1.8446744073709551615e19", u64::MAX),
    ] {
        let deck = format!("exact seed notation\n.options seed={literal}\n.end\n");
        let parsed = Netlist::parse(&deck).expect(literal);
        assert_eq!(parsed.options.seed, Some(expected), "SEED={literal}");
    }
}

#[test]
fn seed_grammar_without_equals_selects_the_recorded_random_stream() {
    let expected =
        Netlist::parse("seed grammar\n.options seed=37\n.param sample={aunif(0,1)}\n.end\n")
            .unwrap();
    for option in [".options seed 37", ".options rndseed + 37"] {
        let deck = format!("seed grammar\n.param sample={{aunif(0,1)}}\n{option}\n.end\n");
        let parsed = Netlist::parse(&deck).unwrap();
        assert_eq!(parsed.options.seed, Some(37));
        assert_eq!(
            parsed.params.get("sample"),
            expected.params.get("sample"),
            "{option}"
        );
    }
}

#[test]
fn seed_grammar_allows_a_value_on_an_options_continuation_line() {
    let expected =
        Netlist::parse("seed continuation\n.options seed=37\n.param sample={aunif(0,1)}\n.end\n")
            .unwrap();
    let parsed = Netlist::parse(
        "seed continuation\n.param sample={aunif(0,1)}\n.options seed =\n+ 37\n.end\n",
    )
    .expect("continuation whitespace is valid between an option and its value");
    assert_eq!(parsed.options.seed, Some(37));
    assert_eq!(parsed.params.get("sample"), expected.params.get("sample"));
}

#[test]
fn seed_context_accepted_scoped_cards_select_the_recorded_stream() {
    use rspice_core::netlist::expr::{DEFAULT_RANDOM_SEED, ParamContext, eval_expression};

    let mut mismatches = Vec::new();
    for package in [
        "",
        "DEVICE",
        "NONLIN",
        "LOCA",
        "XSPICE",
        "TOPOLOGY",
        "OUTPUT",
        "TIMEINT",
        "FFT",
        "MEASURE",
        "HBINT",
        "LINSOL-HB",
        "NONLIN-HB",
        "NONLIN-TRAN",
        "NONLIN-TRANSIENT",
        "RESTART",
    ] {
        let assignments: &[&str] = if matches!(
            package,
            "" | "DEVICE" | "NONLIN" | "LOCA" | "XSPICE" | "TOPOLOGY" | "OUTPUT"
        ) {
            &["seed=37", "seed 37"]
        } else {
            &["seed=37"]
        };
        for assignment in assignments {
            let option = format!(".options {package} {assignment}");
            let deck = format!("scoped seed\n{option}\n.param sample={{aunif(0,1)}}\n.end\n");
            let parsed = Netlist::parse(&deck).expect(&option);
            let mut context = ParamContext::new();
            context.set_random_seed(parsed.options.seed.unwrap_or(DEFAULT_RANDOM_SEED));
            let expected = eval_expression("aunif(0,1)", &context).unwrap();
            if parsed.params.get("sample").unwrap().to_bits() != expected.to_bits() {
                mismatches.push(format!(
                    "{option}: recorded {:?} but selected another stream",
                    parsed.options.seed
                ));
            }
        }
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

#[test]
fn seed_context_parameter_values_and_quoted_text_do_not_author_a_seed() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};

    for option in [
        ".options abstol=seed",
        ".options abstol seed",
        ".options restart seed 3",
        ".options output initial_interval=1n seed 3",
        ".options restart file=\"record seed=37\"",
    ] {
        let deck =
            format!("seed as data\n.param seed=2n\n{option}\n.param sample={{aunif(0,1)}}\n.end\n");
        let parsed = Netlist::parse(&deck).expect(option);
        assert_eq!(parsed.options.seed, None, "{option}");
        let expected = eval_expression("aunif(0,1)", &ParamContext::new()).unwrap();
        assert_eq!(
            parsed.params.get("sample").unwrap().to_bits(),
            expected.to_bits(),
            "{option}"
        );
    }
}

#[test]
fn seed_context_explicit_option_after_output_schedule_is_unambiguous() {
    let parsed = Netlist::parse("seed after schedule\n.param seed=2n\n.options output initial_interval=1n seed 3 seed=37\n.param sample={aunif(0,1)}\n.end\n")
        .expect("the equals sign distinguishes the option from a schedule parameter");
    assert_eq!(parsed.options.seed, Some(37));
}

#[test]
fn seed_context_bare_option_after_output_schedule_selects_the_recorded_stream() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};

    let parsed = Netlist::parse("seed after schedule\n.options output initial_interval=1n 2n 3n seed 37\n.param sample={aunif(0,1)}\n.end\n")
        .expect("an undefined parameter name ends the positional schedule");
    assert_eq!(parsed.options.seed, Some(37));
    let mut context = ParamContext::new();
    context.set_random_seed(37);
    assert_eq!(
        parsed.params.get("sample").unwrap().to_bits(),
        eval_expression("aunif(0,1)", &context).unwrap().to_bits()
    );
}

#[test]
fn seed_context_inactive_and_terminated_cards_do_not_select_a_stream() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};

    for source in [
        "seed gate\n.if 0\n.options seed=37\n.endif\n.param sample={aunif(0,1)}\n.end\n",
        "seed gate\n.if 0\n.options seed=18446744073709551616\n.endif\n.param sample={aunif(0,1)}\n.end\n",
        "seed gate\n.param sample={aunif(0,1)}\n.end\n.options seed=37\n",
    ] {
        let parsed = Netlist::parse(source).expect(source);
        assert_eq!(parsed.options.seed, None);
        assert_eq!(
            parsed.params.get("sample").unwrap().to_bits(),
            eval_expression("aunif(0,1)", &ParamContext::new())
                .unwrap()
                .to_bits(),
            "{source}"
        );
    }
}

#[test]
fn seed_context_stable_conditional_selection_replays_the_parameter_sequence() {
    let conditional = Netlist::parse("conditional seed\n.param choose=1\n.if choose\n.options seed=37\n.else\n.options seed=41\n.endif\n.param first={aunif(0,1)} second={aunif(0,1)}\n.end\n").unwrap();
    let direct = Netlist::parse(
        "direct seed\n.options seed=37\n.param first={aunif(0,1)} second={aunif(0,1)}\n.end\n",
    )
    .unwrap();
    assert_eq!(conditional.options.seed, Some(37));
    for name in ["first", "second"] {
        assert_eq!(conditional.params.get(name), direct.params.get(name));
    }
}

#[test]
fn seed_context_random_dependent_seed_cycles_are_rejected() {
    let error = Netlist::parse("circular seed\n.param draw={aunif(0,1)}\n.if draw>0.3\n.options seed=1\n.else\n.options seed=37\n.endif\n.end\n")
        .expect_err("the selected seed alternates between 1 and 37");
    assert!(
        error.to_string().contains("SEED selection changes"),
        "{error}"
    );
}

#[test]
fn seed_context_sealed_include_replay_preserves_circuit_and_shared_draws() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::netlist::{NetlistParseOptions, SealedSourceBundle, SealedSourceEdge};

    let root = std::env::temp_dir().join("rspice-sealed-seed-root.cir");
    let child = root.with_file_name("rspice-sealed-seed-child.inc");
    let source =
        "sealed seed\n.options seed=37\n.include child.inc\n.param second={aunif(0,1)}\n.end\n";
    let include = ".if 0\n.options seed=41\n.endif\n.param first={aunif(0,1)}\nR1 1 0 1k\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.to_owned()),
            (child.clone(), include.to_owned()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "child.inc".to_owned(),
            target: child.clone(),
        }],
    )
    .unwrap();
    let parsed = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        source,
        &root,
        bundle,
        NetlistParseOptions::default(),
        &NoAbort,
    )
    .unwrap();
    let direct = Netlist::parse(
        "direct seed\n.options seed=37\n.param first={aunif(0,1)} second={aunif(0,1)}\n.end\n",
    )
    .unwrap();
    assert_eq!(parsed.options.seed, Some(37));
    for name in ["first", "second"] {
        assert_eq!(parsed.params.get(name), direct.params.get(name));
    }
    assert_eq!(parsed.elements.len(), 1);
    assert_eq!(parsed.elements[0].name, "R1");
    assert_eq!(parsed.source_path.as_ref(), Some(&root));
}

mod direct_mc_seed_regressions {
    use rspice_core::netlist::{AnalysisCommand, Netlist};

    fn recorded_seed(source: &str) -> u64 {
        let parsed = Netlist::parse(source).expect(source);
        let AnalysisCommand::MonteCarlo(command) = &parsed.analyses[0] else {
            panic!("missing Monte Carlo command");
        };
        command.seed.expect("authored seed")
    }

    #[test]
    fn full_width_literals_are_not_rounded() {
        for seed in [0, (1_u64 << 53) + 1, u64::MAX - 1, u64::MAX] {
            let source = format!("MC seed\n.mc 2 seed {seed}\n.end\n");
            assert_eq!(recorded_seed(&source), seed, "{source}");
        }
    }

    #[test]
    fn overflow_and_fractional_literals_are_rejected() {
        for literal in [
            "18446744073709551616",
            "18446744073709551617",
            "9007199254740993.5",
            "1.0000000000001",
            "-1",
        ] {
            let source = format!("MC seed\n.mc 2 seed {literal}\n.end\n");
            assert!(
                Netlist::parse(&source).is_err(),
                "invalid seed accepted: {literal}"
            );
        }
    }

    #[test]
    fn engineering_and_decimal_notation_are_exact() {
        for (literal, seed) in [
            (".1k", 100),
            ("1e3k", 1_000_000),
            ("9007199254740.993k", (1_u64 << 53) + 1),
            ("1.8446744073709551615e19", u64::MAX),
        ] {
            let source = format!("MC seed\n.mc 2 seed {literal}\n.end\n");
            assert_eq!(recorded_seed(&source), seed, "{literal}");
        }
    }

    #[test]
    fn existing_parameter_and_expression_inputs_remain_supported() {
        for (input, expected) in [("base", 37), ("{base+5}", 42), ("'base+5'", 42)] {
            let source = format!("MC seed\n.param base=37\n.mc 2 seed {input}\n.end\n");
            assert_eq!(recorded_seed(&source), expected, "{input}");
        }
    }

    #[test]
    fn evaluated_seeds_must_also_be_in_range() {
        for value in ["18446744073709551616", "-1", "0.5"] {
            let source = format!("MC seed\n.param base={value}\n.mc 2 seed base\n.end\n");
            assert!(
                Netlist::parse(&source).is_err(),
                "invalid evaluated seed accepted: {value}"
            );
        }
    }

    #[test]
    fn wrapped_literals_and_optional_equals_preserve_full_width() {
        for spelling in [
            "{18446744073709551615}",
            "'18446744073709551615'",
            "1.8446744073709551615e19",
        ] {
            assert_eq!(
                recorded_seed(&format!("MC seed\n.mc 2 seed={spelling}\n.end\n")),
                u64::MAX
            );
        }
        assert_eq!(recorded_seed("MC seed\n.mc 2 seed - {-37}\n.end\n"), 37);
    }

    #[test]
    fn run_counts_reject_fractional_values_and_overflow() {
        for count in ["0", "-1", "1.0000000000001", "18446744073709551616"] {
            let source = format!("MC runs\n.mc {count}\n.end\n");
            assert!(
                Netlist::parse(&source).is_err(),
                "invalid count accepted: {count}"
            );
        }
    }

    #[test]
    fn run_counts_respect_the_callers_analysis_point_limit() {
        use rspice_core::netlist::NetlistParseOptions;
        let mut options = NetlistParseOptions::default();
        options.resource_limits.max_analysis_points = 2;
        Netlist::parse_with_options("MC runs\n.param count=2\n.mc count\n.end\n", options).unwrap();
        let error = Netlist::parse_with_options("MC runs\n.mc 3\n.end\n", options)
            .expect_err("MC must honor the same configured point limit as other analyses");
        assert!(
            matches!(error, rspice_core::netlist::ParseError::ResourceLimit(_)),
            "{error}"
        );
    }
}

#[test]
fn runtime_seed_override_preserves_source_and_controls_all_parameter_draws() {
    use rspice_core::netlist::{
        NetlistParseOptions,
        expr::{ParamContext, eval_expression},
    };
    let source = "runtime seed\n.param first={aunif(0,1)}\n.options seed=7\n.if 0\n.options seed=99\n.endif\n.param second={aunif(0,1)}\n.end ; done\n";
    for seed in [0, (1_u64 << 53) + 1, u64::MAX] {
        let parsed = Netlist::parse_with_options(
            source,
            NetlistParseOptions {
                statistical_seed: Some(seed),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(parsed.source_text.as_deref(), Some(source));
        assert_eq!(parsed.options.seed, Some(seed));
        let mut context = ParamContext::new();
        context.set_random_seed(seed);
        for name in ["first", "second"] {
            assert_eq!(
                parsed.params.get(name).unwrap().to_bits(),
                eval_expression("aunif(0,1)", &context).unwrap().to_bits()
            );
        }
    }
}

#[test]
fn runtime_seed_override_does_not_hide_invalid_authored_seed_options() {
    use rspice_core::netlist::NetlistParseOptions;
    let options = NetlistParseOptions {
        statistical_seed: Some(37),
        ..Default::default()
    };
    assert!(
        Netlist::parse_with_options("invalid option\n.options seed=1.5\n.end\n", options).is_err()
    );
    assert!(
        Netlist::parse_with_options("unclosed conditional\n.if 0\n.end\n.endif\n", options)
            .is_err()
    );
    let empty = Netlist::parse_with_options("", options).unwrap();
    assert_eq!(empty.options.seed, Some(37));
    assert_eq!(empty.params.random().seed(), 37);
}

#[test]
fn runtime_seed_override_reaches_sealed_includes() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::netlist::{NetlistParseOptions, SealedSourceBundle, SealedSourceEdge};

    let root = std::env::temp_dir().join("rspice-runtime-seed-root.cir");
    let child = root.with_file_name("rspice-runtime-seed-child.inc");
    let source =
        "runtime seed\n.options seed=7\n.include child.inc\n.param second={aunif(0,1)}\n.end\n";
    let include = ".options seed=11\n.param first={aunif(0,1)}\nR1 1 0 1k\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.to_owned()),
            (child.clone(), include.to_owned()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "child.inc".to_owned(),
            target: child,
        }],
    )
    .unwrap();
    let options = NetlistParseOptions {
        statistical_seed: Some(u64::MAX),
        ..Default::default()
    };
    let parsed = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        source, &root, bundle, options, &NoAbort,
    )
    .unwrap();
    let direct = Netlist::parse_with_options(
        "direct seed\n.param first={aunif(0,1)} second={aunif(0,1)}\n.end\n",
        options,
    )
    .unwrap();
    for name in ["first", "second"] {
        assert_eq!(parsed.params.get(name), direct.params.get(name));
    }
    assert_eq!(parsed.options.seed, Some(u64::MAX));
    assert_eq!(parsed.source_text.as_deref(), Some(source));
    assert_eq!(parsed.source_path.as_ref(), Some(&root));
    assert_eq!(parsed.elements[0].name, "R1");
}
