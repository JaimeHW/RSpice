//! Root analysis operands bind without changing card/result ownership.
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::expr::{ParamContext, eval_expression};
use rspice_core::netlist::{AnalysisCommand, NetlistParseOptions, OutputDirectiveKind};
use rspice_core::{Engine, Netlist};

const DIALECTS: [ExpressionDialect; 2] = [ExpressionDialect::Ngspice, ExpressionDialect::Xyce];

fn parse(source: &str, expression_dialect: ExpressionDialect) -> Netlist {
    Netlist::parse_with_options(
        source,
        NetlistParseOptions {
            expression_dialect,
            ..Default::default()
        },
    )
    .unwrap_or_else(|error| panic!("{expression_dialect:?}: {error}\n{source}"))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_bare_signed_and_retained_operands_bind_before_public_dc_execution() {
    for dialect in DIALECTS {
        for operand in ["{stop}", "stop", "+stop"] {
            for declarations in [
                ".param stop={later}\n.param later=1\n",
                ".global_param stop={later}\n.param later=1\n",
            ] {
                let source = format!(
                    "Forward DC\nV1 in 0 0\nR1 in out 1k\nR2 out 0 1k\n.DC V1 -stop {operand} .5\n{declarations}.end\n"
                );
                let netlist = parse(&source, dialect);
                let AnalysisCommand::Dc {
                    source,
                    start,
                    stop,
                    step,
                    ..
                } = &netlist.analyses[0]
                else {
                    panic!("DC")
                };
                assert_eq!((*start, *stop, *step), (-1.0, 1.0, 0.5));
                let points = Engine::default()
                    .run_dc_sweep(&netlist, source, *start, *stop, *step)
                    .unwrap();
                assert_eq!(points.len(), 5);
                for (voltage, point) in points {
                    assert!(
                        (point.try_voltage_named("out").unwrap() - voltage / 2.0).abs() < 1e-10
                    );
                }
            }
        }
        let netlist = parse(
            "Already declared chain\n.param stop={later}\n.param later=1\n.DC V1 0 {stop} .1\n.end\n",
            dialect,
        );
        assert!(matches!(
            netlist.analyses[0],
            AnalysisCommand::Dc { stop: 1.0, .. }
        ));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn pending_cards_capture_known_complex_values_functions_and_lazy_branches() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Captured analysis\n.param known=2 z={3+4j}\n.func scale(x) {known*x}\n.DC V1 {if(1,scale(2)+img(z),missing)} {later} 1\n.param known=20 z={30+40j}\n.func scale(x) {100*x}\n.param later=10\n.end\n",
            dialect,
        );
        assert!(
            matches!(
                netlist.analyses[0],
                AnalysisCommand::Dc {
                    start: 8.0,
                    stop: 10.0,
                    ..
                }
            ),
            "{:?}",
            netlist.analyses
        );
        assert_eq!(netlist.params.get("known"), Some(20.0));
        assert_eq!(netlist.params.get_complex("z").unwrap().im, 40.0);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ready_pending_and_nonanalysis_outputs_keep_authored_order() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Ordered cards\nV1 out 0 1\nR1 out 0 1k\n.OP\n.DC V1 0 {last} .1\n.print tran v(out)\n.FOUR {fund} v(out)\n.meas tran peak max v(out)\n.FFT v(out) NP={count}\n.save v(out)\n.AC LIN 2 1 10\n.FFT v(out) NP=64\n.param last=1 fund=1k count=32\n.end\n",
            dialect,
        );
        assert!(matches!(
            netlist.analyses.as_slice(),
            [
                AnalysisCommand::Op,
                AnalysisCommand::Dc { .. },
                AnalysisCommand::Four { .. },
                AnalysisCommand::Ac { .. }
            ]
        ));
        assert_eq!(
            netlist
                .output_requests
                .iter()
                .map(|request| request.directive)
                .collect::<Vec<_>>(),
            [
                OutputDirectiveKind::Print,
                OutputDirectiveKind::Four,
                OutputDirectiveKind::Measure,
                OutputDirectiveKind::Fft,
                OutputDirectiveKind::Save,
                OutputDirectiveKind::Fft
            ]
        );
        assert_eq!(
            netlist
                .output_requests
                .iter()
                .map(|request| request.origin.line)
                .collect::<Vec<_>>(),
            [6, 7, 8, 9, 10, 12]
        );
        assert_eq!(netlist.fft_analyses.len(), 2);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_and_scoped_numeric_families_reuse_the_literal_card_grammar() {
    for dialect in DIALECTS {
        for (card, value) in [
            (".AC LIN @ 1 10", "2"),
            (".TRAN 1n 1u @", "1n"),
            (".TRAN 1n 1u 0 @", "2n"),
            (".TEMP 25 @", "85"),
            (".STEP PARAM gain LIST 1 @", "2"),
            (".MC @ uniform .1 seed 37", "2"),
            (".LIN SPARCALC=@", "0"),
            (".NOISE V(out) V1 LIN @ 1 10", "2"),
            (".SENS V(out) AC LIN @ 1 10", "2"),
            (".DISTO LIN @ 1 10", "2"),
            (".SP LIN @ 1 10", "2"),
            (".STB LIN @ 1 10 PROBE=V1", "2"),
            (".DCMATCH OUT=V(out) SIGMA=@", "2"),
            (".FOUR @ V(out)", "1k"),
            (".FFT V(out) NP=@", "32"),
            (".HB @", "1G"),
            (".PSS FUND=@", "1G"),
            (".QPSS @ 1.414G HARMS=(2,2)", "1G"),
            (
                ".QPSS 1G 1.414G HARMS=(2,2)\n.QPAC LIN @ 1 10 SOURCE=V1 OUT=V(out) INLATTICE=(0,0) OUTLATTICE=(0,0)",
                "2",
            ),
            (
                ".QPSS 1G 1.414G HARMS=(2,2)\n.QPXF LIN @ 1 10 OUT=V(out) OUTLATTICE=(0,0)",
                "2",
            ),
            (
                ".QPSS 1G 1.414G HARMS=(2,2)\n.QPNOISE LIN @ 1 10 OUT=V(out) OUTLATTICE=(0,0)",
                "2",
            ),
            (".HB 1G\n.PAC LIN @ 1 10 INPUT=V1 OUT=V(out)", "2"),
            (".HB 1G\n.PXF LIN @ 1 10 INPUT=V1 OUT=V(out)", "2"),
            (".HB 1G\n.PNOISE LIN @ 1 10 OUT=V(out)", "2"),
            (".PSS FUND=1G\n.PSTB PROBE=V1 MAXHARM=@", "2"),
            (".HB 1G\n.ENVELOPE TSTOP=@", "1u"),
        ] {
            for (opening, closing) in [("", ""), (".subckt child p\n", ".ends\n")] {
                let deferred = parse(
                    &format!(
                        "Forward grammar\n{opening}{}\n.param later={{base}}\n.param base={value}\n{closing}.end\n",
                        card.replace('@', "{later}")
                    ),
                    dialect,
                );
                let literal = parse(
                    &format!("Literal grammar\n{}\n.end\n", card.replace('@', value)),
                    dialect,
                );
                assert_eq!(
                    format!("{:?}", deferred.analyses),
                    format!("{:?}", literal.analyses),
                    "{dialect:?}: {card}"
                );
                assert_eq!(
                    format!("{:?}", deferred.lin_analysis),
                    format!("{:?}", literal.lin_analysis),
                    "{card}"
                );
                assert_eq!(
                    format!("{:?}", deferred.fft_analyses),
                    format!("{:?}", literal.fft_analyses),
                    "{card}"
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_draws_follow_existing_binding_phases_and_share_parameter_samples() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Statistical analysis\n.options seed=37\n.param shared={aunif(0,1)+base}\n.DC V1 {aunif(0,1)} {shared} 1\n.AC LIN 2 {aunif(2,1)} 10\nV1 out 0 DC {shared}\n.model rm R (R={shared})\n.param base=4 eager={aunif(0,1)}\n.end\n",
            dialect,
        );
        let mut expected = ParamContext::new();
        expected.set_expression_dialect(dialect);
        expected.set_random_seed(37);
        let ac = eval_expression("aunif(2,1)", &expected).unwrap();
        assert_eq!(
            netlist.params.get("eager"),
            Some(eval_expression("aunif(0,1)", &expected).unwrap())
        );
        let shared = 4.0 + eval_expression("aunif(0,1)", &expected).unwrap();
        assert_eq!(netlist.params.get("shared"), Some(shared));
        let AnalysisCommand::Dc { start, stop, .. } = netlist.analyses[0] else {
            panic!("DC")
        };
        assert_eq!(start, eval_expression("aunif(0,1)", &expected).unwrap());
        assert_eq!(stop, shared);
        let AnalysisCommand::Ac { start_freq, .. } = netlist.analyses[1] else {
            panic!("AC")
        };
        assert_eq!(start_freq, ac);
        assert!(
            netlist.models[0]
                .params
                .iter()
                .any(|(name, value)| name == "R" && *value == shared)
        );
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            eval_expression("aunif(0,1)", &expected).unwrap()
        );
        assert!(
            (Engine::default()
                .run_dc_op(&netlist)
                .unwrap()
                .try_voltage_named("out")
                .unwrap()
                - shared)
                .abs()
                < 1e-10
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_constraints_fail_at_the_later_authored_card() {
    for (cards, line, message) in [
        (
            ".LIN SPARCALC={flag}\n.LIN SPARCALC=0\n.param flag=0",
            3,
            "only once",
        ),
        (
            ".TRAN 1n 1u NOISEFMAX={band}\n.TR 1n 1u NOISEFMAX=2meg\n.param band=1meg",
            3,
            "different transient-noise",
        ),
        (
            ".TRAN 1n 1u NOISEFMAX=2meg\n.TR 1n 1u NOISEFMAX={band}\n.param band=1meg",
            3,
            "different transient-noise",
        ),
    ] {
        let error = Netlist::parse(&format!("Deferred constraints\n{cards}\n.end\n")).unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
        assert!(
            matches!(error, rspice_core::netlist::ParseError::Syntax { line: actual, .. } if actual == line),
            "{error}"
        );
    }
    let error = Netlist::parse("Exact seed\n.TRAN 1n {stop} NOISEFMAX=1meg NOISESEED={seed}\n.param stop=1u seed=37\n.end\n").unwrap_err();
    assert!(
        error.to_string().contains("integer numeric literal"),
        "{error}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn completed_child_bindings_do_not_leak_into_pending_root_cards() {
    let error = Netlist::parse("Scope separation\n.DC V1 0 {local} .1\n.subckt child p\n.param local=1\nR1 p 0 1k\n.ends\n.end\n").unwrap_err();
    assert!(
        error.to_string().to_ascii_uppercase().contains("LOCAL"),
        "{error}"
    );
    let netlist = Netlist::parse("Scope ordering\n.DC V1 0 {later} .1\n.subckt child p\n.param local=2\n.AC LIN 2 1 {local}\nR1 p 0 1k\n.ends\n.param later=1 local=20\n.end\n").unwrap();
    assert!(matches!(
        netlist.analyses.as_slice(),
        [
            AnalysisCommand::Dc { stop: 1.0, .. },
            AnalysisCommand::Ac { stop_freq: 2.0, .. }
        ]
    ));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_ac_and_transient_schedules_execute_the_bound_values() {
    let netlist = Netlist::parse("Physical schedules\nV1 in 0 1 AC 1\nR1 in out 1k\nR2 out 0 1k\n.AC LIN {count} {low} {high}\n.TRAN {step} {stop}\n.param count=2 low=10 high=100 step=1n stop=10n\n.end\n").unwrap();
    let AnalysisCommand::Ac {
        points,
        start_freq,
        stop_freq,
        ..
    } = netlist.analyses[0]
    else {
        panic!("AC")
    };
    assert_eq!((points, start_freq, stop_freq), (2, 10.0, 100.0));
    let engine = Engine::default();
    for point in engine.run_ac(&netlist, &[start_freq, stop_freq]).unwrap() {
        let out = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!((point.voltages[out].re - 0.5).abs() < 1e-10);
        assert!(point.voltages[out].im.abs() < 1e-10);
    }
    let AnalysisCommand::Tran { step, stop, .. } = netlist.analyses[1] else {
        panic!("TRAN")
    };
    assert_eq!((step, stop), (1e-9, 1e-8));
    let transient = engine.run_tran(&netlist, stop, step).unwrap();
    assert!((transient.time.last().unwrap() - stop).abs() < 1e-18);
    for voltage in transient.try_voltage_waveform_named("out").unwrap() {
        assert!((voltage - 0.5).abs() < 1e-10);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn analysis_warnings_merge_with_parameter_diagnostics_in_source_order() {
    let netlist = Netlist::parse_with_options("Warning order\n.DC V1 {start} 0 1\n.param gain=1\n.param gain=2\n.FFT V(out) START=-1 NP=32\n.DC V2 {start} 0 1\n.param start=1\n.end\n", NetlistParseOptions {
        expression_dialect: ExpressionDialect::Xyce,
        parameter_redefinition_diagnostic_policy: rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Warning,
        ..Default::default()
    }).unwrap();
    assert_eq!(
        netlist
            .diagnostics
            .iter()
            .map(|warning| warning.line)
            .collect::<Vec<_>>(),
        [2, 4, 5, 6]
    );
    for warning in &netlist.diagnostics {
        assert_eq!(
            warning.origin,
            Some(rspice_core::netlist::NetlistSourceLocation::in_memory(
                warning.line
            ))
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_monte_carlo_keeps_authenticated_reporting_spans() {
    use rspice_core::engine::MonteCarloStudyConfig;
    let identity = |source: &str| {
        let netlist = Netlist::parse(source).unwrap();
        Engine::default()
            .new_monte_carlo_checkpoint(
                &netlist,
                &MonteCarloStudyConfig::new(2, 37, vec!["V(out)".into()]),
                [42; 32],
                &rspice_core::NoAbort,
            )
            .unwrap()
            .population_identity()
    };
    let source = "Deferred MC\nV1 out 0 1\nR1 out 0 1k\n.mc {trials} uniform .1\n+ seed 37 START=2 CONFIDENCE=95\n.param trials=2\n.end\n";
    assert_eq!(
        identity(source),
        identity(&source.replace("START=2 CONFIDENCE=95", "START=3 CONFIDENCE=90"))
    );
    assert_ne!(
        identity(source),
        identity(&source.replace("uniform .1", "uniform .2"))
    );
    assert_ne!(
        identity(source),
        identity(&source.replace("{trials}", "{trials+0}"))
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn deferred_included_cards_keep_physical_errors_and_warnings() {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "rspice-forward-analysis-{}-{nonce}",
        std::process::id()
    ));
    std::fs::create_dir(&directory).unwrap();
    let child = directory.join("analysis.inc");
    std::fs::write(&child, ".DC V1 0 {missing} .1\n").unwrap();
    let error = Netlist::parse_with_path(
        "Include analysis\n.include analysis.inc\n.end\n",
        &directory.join("root.cir"),
    )
    .unwrap_err();
    assert!(error.to_string().contains("analysis.inc:1:"), "{error}");
    std::fs::write(
        &child,
        ".FFT V(out) START=-1 NP={count}\n.FFT V(out) START=-1 NP=32\n",
    )
    .unwrap();
    let netlist = Netlist::parse_with_path("Include warnings\n.DC V1 0 {later} .1\n.include analysis.inc\n.param count=32 later=1\n.end\n", &directory.join("root.cir")).unwrap();
    let physical_child = child.canonicalize().unwrap();
    std::fs::write(&child, ".subckt child p\n.DC V1 0 {missing} .1\n.ends\n").unwrap();
    let error = Netlist::parse_with_path(
        "Scoped include\n.include analysis.inc\n.end\n",
        &directory.join("root.cir"),
    )
    .unwrap_err();
    assert!(error.to_string().contains("analysis.inc:2:"), "{error}");
    std::fs::write(&child, ".subckt child p\n.FFT V(out) START=-1 NP={count}\n.param count={base}\n.param base=32\n.ends\n").unwrap();
    let scoped = Netlist::parse_with_path(
        "Scoped warnings\n.include analysis.inc\n.end\n",
        &directory.join("root.cir"),
    )
    .unwrap();
    assert_eq!(scoped.diagnostics.len(), 1);
    assert_eq!(scoped.diagnostics[0].origin.as_ref().unwrap().line, 2);
    assert_eq!(
        scoped.diagnostics[0].origin.as_ref().unwrap().path.as_ref(),
        Some(&physical_child)
    );
    std::fs::remove_dir_all(&directory).unwrap();
    assert_eq!(netlist.diagnostics.len(), 2);
    for (index, warning) in netlist.diagnostics.iter().enumerate() {
        assert_eq!(warning.line, index + 1);
        let origin = warning
            .origin
            .as_ref()
            .expect("staged warning has an owner");
        assert_eq!(origin.line, index + 1);
        assert_eq!(origin.path.as_ref(), Some(&physical_child));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scoped_operands_bind_local_header_parent_and_later_root_declarations() {
    for dialect in DIALECTS {
        for source in [
            "Scoped\n.subckt child p\n.DC V1 -stop +stop .5\n.param stop={later}\n.param later=1\n.ends\n.end\n",
            "Header\n.param stop=90\n.subckt child p params: stop={later}\n.DC V1 -stop +stop .5\n.param later=1\n.ends\n.end\n",
            "Parent\n.subckt parent p\n.subckt child q\n.DC V1 -stop +stop .5\n.ends\n.param stop={later}\n.param later=1\n.ends\n.end\n",
            "Root\n.subckt child p\n.DC V1 -stop +stop .5\n.ends\n.param stop={later}\n.param later=1\n.end\n",
        ] {
            let netlist = parse(source, dialect);
            assert!(
                matches!(
                    netlist.analyses[0],
                    AnalysisCommand::Dc {
                        start: -1.0,
                        stop: 1.0,
                        step: 0.5,
                        ..
                    }
                ),
                "{:?}",
                netlist.analyses
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scoped_numeric_resumption_preserves_owner_samples_and_operand_order() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Scoped samples\n.options seed=73\n.subckt parent p\n.param alias={base+aunif(0,1)}\n.subckt child q\n.param base=100\n.DC V1 {aunif(0,1)+alias+aunif(0,1)} {limit} 1\n.param limit=100\n.ends\n.subckt sibling q\n.DC V1 0 {alias} 1\n.ends\n.DC V1 0 {alias} 1\n.param base=5\n.ends\n.end\n",
            dialect,
        );
        let mut expected = ParamContext::new();
        expected.set_expression_dialect(dialect);
        expected.set_random_seed(73);
        let first = eval_expression("aunif(0,1)", &expected).unwrap();
        let alias = 5.0 + eval_expression("aunif(0,1)", &expected).unwrap();
        let last = eval_expression("aunif(0,1)", &expected).unwrap();
        let AnalysisCommand::Dc { start, stop, .. } = netlist.analyses[0] else {
            panic!("DC")
        };
        assert_eq!(start, (first + alias) + last);
        assert_eq!(stop, 100.0);
        for card in &netlist.analyses[1..] {
            let AnalysisCommand::Dc { stop, .. } = card else {
                panic!("DC")
            };
            assert_eq!(*stop, alias);
        }
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            eval_expression("aunif(0,1)", &expected).unwrap()
        );
        assert_eq!(netlist.params.get("alias"), None);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scoped_binding_is_demand_driven_and_keeps_instance_overrides() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Instance scopes\nV1 in 0 1\nX1 in out cell gain=3\nRload out 0 1k\n.subckt cell a b params: gain=1\n.param unused={instance_only+missing}\n.param resistance={gain*base}\nR1 a b {resistance}\n.DC V1 0 {if(1,stop,unused)} .5\n.param stop=1 base=1k\n.ends\n.end\n",
            dialect,
        );
        let AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            ..
        } = &netlist.analyses[0]
        else {
            panic!("DC")
        };
        // An unused invalid body parameter is deliberately retained. Prove
        // parsing did not force it; elaborate a valid unused body below.
        assert_eq!((*start, *stop, *step), (0.0, 1.0, 0.5));
        assert_eq!(source.to_ascii_uppercase(), "V1");
        let valid = parse(
            "Instance scopes\nV1 in 0 1\nX1 in out cell gain=3\nRload out 0 1k\n.subckt cell a b params: gain=1\n.param resistance={gain*base}\nR1 a b {resistance}\n.DC V1 0 {stop} .5\n.param stop=1 base=1k\n.ends\n.end\n",
            dialect,
        );
        let points = Engine::default()
            .run_dc_sweep(&valid, "V1", 0.0, 1.0, 0.5)
            .unwrap();
        assert_eq!(points.len(), 3);
        for (voltage, point) in points {
            assert!((point.try_voltage_named("out").unwrap() - voltage / 4.0).abs() < 1e-10);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scoped_pending_cards_keep_captured_complex_functions_and_sibling_separation() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Scope captures\n.subckt parent p\n.param known=2 z={3+4j}\n.func scale(x) {known*x}\n.DC V1 {scale(2)+img(z)} {later} 1\n.param known=20 z={30+40j}\n.func scale(x) {100*x}\n.param later=10\n.ends\n.subckt sibling p\n.DC V1 0 {later} 1\n.param later=30\n.ends\n.end\n",
            dialect,
        );
        assert!(matches!(
            netlist.analyses[0],
            AnalysisCommand::Dc {
                start: 8.0,
                stop: 10.0,
                ..
            }
        ));
        assert!(matches!(
            netlist.analyses[1],
            AnalysisCommand::Dc { stop: 30.0, .. }
        ));
        let invalid = "No sibling binding\n.subckt first p\n.DC V1 0 {only_second} 1\n.ends\n.subckt second p\n.param only_second=10\n.ends\n.end\n";
        assert!(
            Netlist::parse_with_options(
                invalid,
                NetlistParseOptions {
                    expression_dialect: dialect,
                    ..Default::default()
                }
            )
            .is_err()
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn suspended_scoped_operands_do_not_replay_earlier_fields_or_lazy_draws() {
    for dialect in DIALECTS {
        let mut expected = ParamContext::new();
        expected.set_expression_dialect(dialect);
        expected.set_random_seed(73);
        let start = eval_expression("aunif(0,1)", &expected).unwrap();
        assert!(eval_expression("aunif(1,.1)", &expected).unwrap() > 0.0);
        let stop = 9.0 + eval_expression("aunif(0,1)", &expected).unwrap();
        let step = eval_expression("abs(aunif(0,1))+.01", &expected).unwrap();
        let netlist = parse(
            "Operand suspension\n.options seed=73\n.subckt cell p\n.param z={3+4j}\n.param shared={base+img(z)+aunif(0,1)}\n.param unused={missing}\n.DC V1 {aunif(0,1)} {if(aunif(1,.1)>0,shared,unused)} {abs(aunif(0,1))+.01}\n.param base=5\n.ends\n.end\n",
            dialect,
        );
        let AnalysisCommand::Dc {
            start: actual_start,
            stop: actual_stop,
            step: actual_step,
            ..
        } = netlist.analyses[0]
        else {
            panic!("DC")
        };
        assert_eq!(
            (actual_start, actual_stop, actual_step),
            (start, stop, step)
        );
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            eval_expression("aunif(0,1)", &expected).unwrap()
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn owner_cache_does_not_replace_an_inherited_child_snapshot() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Inherited snapshot\n.options seed=73\n.subckt parent p\n.param shared=2\n.subckt child q\n.param local={other+shared}\n.DC V1 0 {local} 1\n.ends\n.param shared={base+aunif(0,1)}\n.param other={shared+1}\n.param base=5\n.ends\n.end\n",
            dialect,
        );
        let mut expected = ParamContext::new();
        expected.set_expression_dialect(dialect);
        expected.set_random_seed(73);
        let shared = 5.0 + eval_expression("aunif(0,1)", &expected).unwrap();
        let AnalysisCommand::Dc { stop, .. } = netlist.analyses[0] else {
            panic!("DC")
        };
        assert_eq!(stop, (shared + 1.0) + 2.0);
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            eval_expression("aunif(0,1)", &expected).unwrap()
        );
    }
}
