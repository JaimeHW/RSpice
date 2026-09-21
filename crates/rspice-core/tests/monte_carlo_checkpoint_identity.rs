//! Resume compatibility must omit reporting settings without losing source law.
use rspice_core::engine::MonteCarloStudyConfig;
use rspice_core::{Engine, Netlist, NoAbort};

const SOURCE: &str = "Checkpoint source\n.param r=1k\nV1 in 0 1\nR1 in out {r}\nR2 out 0 1k\n.mc 2 uniform 0.2 seed 37 START=3\n.end\n";
fn identity(netlist: &Netlist) -> [u8; 32] {
    let study = MonteCarloStudyConfig::new(2, 37, vec!["V(out)".into()]);
    Engine::default()
        .new_monte_carlo_checkpoint(netlist, &study, [42; 32], &NoAbort)
        .unwrap()
        .population_identity()
}
fn parse_identity(source: &str) -> [u8; 32] {
    identity(&Netlist::parse(source).unwrap())
}

#[test]
fn monte_carlo_checkpoint_identity_allows_literal_reports_and_continuations() {
    let expected = parse_identity(SOURCE);
    for card in [
        ".mc 9 uniform 0.2 seed 37 START=0 CONFIDENCE=90",
        ".mc 4 uniform 0.2 seed 37 CI BOOTSTRAP RESAMPLES=32 BOOTSEED=99",
        ".mc 9 uniform 0.2\n* comment between physical records\n+ seed 37 START=7 ; inline comment\n+ CONFIDENCE=80 CI=BOOTSTRAP RESAMPLES=64 BOOTSEED=123",
    ] {
        let changed = SOURCE.replace(".mc 2 uniform 0.2 seed 37 START=3", card);
        assert_eq!(expected, parse_identity(&changed), "{card}");
    }
    for (from, to) in [
        ("uniform 0.2", "uniform 0.3"),
        ("seed 37", "seed 38"),
        ("r=1k", "r=2k"),
        ("R2 out 0 1k", "R2 out 0 2k"),
    ] {
        assert_ne!(expected, parse_identity(&SOURCE.replace(from, to)), "{to}");
    }
    let mut edited = Netlist::parse(SOURCE).unwrap();
    edited.source_text = Some(SOURCE.replace(".mc 2", ".mc 3"));
    assert_ne!(
        expected,
        identity(&edited),
        "public source edits cannot borrow stale parser spans"
    );
    let mut edited = Netlist::parse(SOURCE).unwrap();
    if let rspice_core::netlist::AnalysisCommand::MonteCarlo(command) = &mut edited.analyses[0] {
        command.relative_spread = 0.3;
    } else {
        panic!("expected .MC")
    }
    assert_ne!(expected, identity(&edited), "typed edits remain bound");
}

#[test]
fn monte_carlo_checkpoint_identity_preserves_expressions_and_nonexecuted_cards() {
    let expression = SOURCE.replace("START=3", "START=3 CONFIDENCE={90+5}");
    assert_ne!(
        parse_identity(&expression),
        parse_identity(&expression.replace("90+5", "94+1"))
    );
    // These look like MC cards but are not accepted commands in this parse.
    for extra in [".if 0\n.mc 2\n.endif\n", ".control\n* .mc 2\n.endc\n"] {
        let source = SOURCE.replace("V1 in 0 1\n", &format!("V1 in 0 1\n{extra}"));
        assert_ne!(
            parse_identity(&source),
            parse_identity(&source.replacen(".mc 2\n", ".mc 3\n", 1))
        );
    }
    let source = format!("{SOURCE}.mc 2\n");
    assert_ne!(
        parse_identity(&source),
        parse_identity(&format!("{SOURCE}.mc 3\n"))
    );
}

#[test]
fn monte_carlo_checkpoint_identity_keeps_included_source_and_root_ownership() {
    let directory = std::env::temp_dir().join(format!(
        "rspice-mc-source-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&directory).unwrap();
    let root = directory.join("root.cir");
    let include = directory.join("parts.inc");
    std::fs::write(&include, ".param r=1k\nR2 out 0 1k\n").unwrap();
    let source = SOURCE
        .replace(".param r=1k\n", ".include parts.inc\n")
        .replace("R2 out 0 1k\n", "");
    let baseline = identity(&Netlist::parse_with_path(&source, &root).unwrap());
    let changed = source
        .replace(".mc 2", ".mc 8")
        .replace("START=3", "START=0 CONFIDENCE=90");
    assert_eq!(
        baseline,
        identity(&Netlist::parse_with_path(&changed, &root).unwrap())
    );
    std::fs::write(&include, ".param r=1k\nR2 out 0 {unif(1k,0.1)}\n").unwrap();
    assert_ne!(
        baseline,
        identity(&Netlist::parse_with_path(&changed, &root).unwrap())
    );
    std::fs::remove_file(&include).unwrap();
    std::fs::remove_dir(&directory).unwrap();
}

#[test]
fn monte_carlo_checkpoint_identity_observes_xyce_comments_and_parameter_names() {
    let options = rspice_core::netlist::NetlistParseOptions {
        expression_dialect: rspice_core::config::ExpressionDialect::Xyce,
        ..Default::default()
    };
    let parse = |text: &str| identity(&Netlist::parse_with_options(text, options).unwrap());
    let source = SOURCE.replace(".mc 2", " .mc 99\n.mc 2");
    let expected = parse(&source);
    assert_eq!(
        expected,
        parse(&source.replace("START=3", "\n + START=9 CONFIDENCE=90"))
    );
    assert_ne!(
        expected,
        parse(&source.replace(" .mc 99", " .mc 98")),
        "column-one whitespace is an Xyce comment, not a command"
    );
    let filtered = SOURCE.replace("START=3", "START=3 PARAMS START CONFIDENCE");
    assert_ne!(
        parse_identity(&filtered),
        parse_identity(&filtered.replace("PARAMS START CONFIDENCE", "PARAMS START BOOTSEED"))
    );
}
