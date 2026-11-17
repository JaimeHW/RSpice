use super::*;

fn index(deck: &str) -> DeckIndex {
    DeckIndex::build(
        &[DeckIndexSource {
            identity: "deck.cir",
            text: deck,
        }],
        ExpressionDialect::Ngspice,
    )
}

fn symbol<'a>(index: &'a DeckIndex, kind: DeckSymbolKind, name: &str) -> Vec<&'a DeckSymbol> {
    index
        .symbols()
        .iter()
        .filter(|symbol| symbol.kind == kind && symbol.name == name)
        .collect()
}

fn reads<'a>(index: &'a DeckIndex, name: &str) -> Vec<&'a DeckReference> {
    index
        .references()
        .iter()
        .filter(|reference| reference.name.eq_ignore_ascii_case(name))
        .collect()
}

#[test]
fn a_number_with_a_suffix_reads_no_parameter_of_the_suffix_name() {
    let deck =
        "turns\n.param n=10\n.param cload=10n\nC1 a 0 10n\nC2 a 0 {n*1p}\nR1 a 0 {2*n}\n.end\n";
    let index = index(deck);
    let n = index
        .symbols()
        .iter()
        .position(|symbol| symbol.name == "n")
        .expect("n is defined");
    let lines = index
        .references_to(n)
        .map(|reference| reference.location.line)
        .collect::<Vec<_>>();
    assert_eq!(lines, [5, 6], "only the expressions read n");
    let tokens = index.card_tokens(index.card_on_line(0, 4).unwrap(), deck);
    let kinds = tokens.iter().map(|token| token.kind).collect::<Vec<_>>();
    assert_eq!(
        kinds,
        [
            DeckTokenKind::Name,
            DeckTokenKind::Name,
            DeckTokenKind::Number,
            DeckTokenKind::Number
        ]
    );
    assert_eq!(&deck[tokens[3].span.clone()], "10n");
}

#[test]
fn a_read_inside_a_subcircuit_binds_its_formal_before_the_global() {
    let deck = "\
formals
.param gain=2
.subckt cell in out params: gain=1
E1 out 0 in 0 {gain}
.ends cell
X1 a b cell gain=3
R1 a 0 {gain}
.end
";
    let index = index(deck);
    let formal = index
        .symbols()
        .iter()
        .position(|symbol| symbol.name == "gain" && symbol.location.line == 3)
        .expect("the formal is a definition");
    let global = index
        .symbols()
        .iter()
        .position(|symbol| symbol.name == "gain" && symbol.location.line == 2)
        .expect("the global is a definition");
    assert_ne!(index.symbols()[formal].scope, 0, "the formal is local");
    let inside = reads(&index, "gain")
        .into_iter()
        .find(|reference| reference.location.line == 4)
        .expect("the body reads gain");
    assert_eq!(inside.target, Some(formal));
    let outside = reads(&index, "gain")
        .into_iter()
        .find(|reference| reference.location.line == 7)
        .expect("the top level reads gain");
    assert_eq!(outside.target, Some(global));
    assert!(
        reads(&index, "gain")
            .iter()
            .all(|reference| reference.location.line != 6),
        "an instance's parameter name is not a read"
    );
}

#[test]
fn a_read_on_a_continuation_line_is_found_where_it_is_written() {
    let deck = "\
cont
.param w0=1u l0=180n
M1 d g s b nch
+ w={w0} l=l0
.model nch nmos level=1
.end
";
    let index = index(deck);
    let w0 = reads(&index, "w0");
    assert_eq!(w0.len(), 1);
    assert_eq!(w0[0].location.line, 4);
    assert_eq!(w0[0].location.column, 6);
    assert_eq!(&deck[w0[0].location.span.clone()], "w0");
    assert!(w0[0].target.is_some());
    let l0 = reads(&index, "l0");
    assert_eq!(l0.len(), 1, "a bare value reads its parameter");
    let card = index
        .card_on_line(0, 4)
        .expect("the continuation is the card's");
    assert_eq!(index.cards()[card].lines, 3..=4);
    assert_eq!(index.cards()[card].head, "M1");
}

#[test]
fn an_instance_written_on_two_lines_reads_its_subcircuit() {
    let deck = "\
two lines
.subckt opamp inp inn out vdd vss
R1 inp out 1k
.ends opamp
X1 in out
+ vdd vss opamp
.end
";
    let index = index(deck);
    let opamp = symbol(&index, DeckSymbolKind::Subcircuit, "opamp");
    assert_eq!(opamp.len(), 1);
    let opamp_index = index
        .symbols()
        .iter()
        .position(|symbol| symbol == opamp[0])
        .unwrap();
    let use_site = reads(&index, "opamp")
        .into_iter()
        .find(|reference| reference.location.line == 6)
        .expect("the X card names its subcircuit on the continuation");
    assert_eq!(use_site.target, Some(opamp_index));
    let x1 = symbol(&index, DeckSymbolKind::Instance, "X1");
    assert_eq!(x1[0].target, Some(opamp_index));
    let x1_nodes = index
        .nodes()
        .iter()
        .filter(|node| node.card == x1[0].card)
        .map(|node| node.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        x1_nodes,
        ["in", "out", "vdd", "vss"],
        "out is a node, not the subcircuit"
    );
    assert_eq!(index.scopes()[opamp[0].opens.unwrap()].ports.len(), 5);
}

#[test]
fn a_read_in_an_include_file_carries_that_files_identity() {
    let deck = "top\n.include \"models/lib.sp\"\n.param vt=0.7\nD1 a 0 dmod\n.end\n";
    let library = "* library\n.model dmod d is=1e-14 vj={vt}\n";
    let index = DeckIndex::build(
        &[
            DeckIndexSource {
                identity: "deck.cir",
                text: deck,
            },
            DeckIndexSource {
                identity: "models/lib.sp",
                text: library,
            },
        ],
        ExpressionDialect::Ngspice,
    );
    let vt = reads(&index, "vt");
    assert_eq!(vt.len(), 1);
    assert_eq!(
        index.file_identity(vt[0].location.file),
        Some("models/lib.sp")
    );
    assert_eq!(vt[0].location.line, 2);
    let dmod_definition = index.symbols()[reads(&index, "dmod")[0].target.unwrap()].clone();
    assert_eq!(
        index.file_identity(dmod_definition.location.file),
        Some("models/lib.sp")
    );
    assert_eq!(index.includes()[0].path, "models/lib.sp");
    assert_eq!(
        index
            .cards()
            .iter()
            .filter(|card| card.kind == DeckCardKind::Title)
            .count(),
        1,
        "an include file has no title"
    );
}

#[test]
fn the_first_line_is_the_title_whatever_it_says_and_control_lines_are_no_instances() {
    let deck = "R1 a b 1k\nV1 a 0 1\n.control\nrun\nplot v(a)\n.endc\n.op\n.end\nR9 x y 1\n";
    let index = index(deck);
    let kinds = index
        .cards()
        .iter()
        .map(|card| (card.kind, card.lines.clone()))
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        [
            (DeckCardKind::Title, 1..=1),
            (DeckCardKind::Instance, 2..=2),
            (DeckCardKind::Control, 3..=6),
            (DeckCardKind::Analysis, 7..=7),
            (DeckCardKind::End, 8..=8),
        ]
    );
    assert_eq!(symbol(&index, DeckSymbolKind::Instance, "V1").len(), 1);
}

#[test]
fn a_transistor_reads_the_model_its_fields_name_not_its_area() {
    let deck =
        "bjt\nQ1 c b e QN 2\nQ2 c b e s QN\nD1 c 0 1N4148\n.model QN npn\n.model 1N4148 d\n.end\n";
    let index = index(deck);
    let model = reads(&index, "QN");
    assert_eq!(model.len(), 2);
    assert!(model.iter().all(|reference| reference.target.is_some()));
    assert!(reads(&index, "2").is_empty());
    assert!(
        reads(&index, "1N4148")[0].target.is_some(),
        "a digit-led model name is a name"
    );
    let q2_nodes = index
        .nodes()
        .iter()
        .filter(|node| node.location.line == 3)
        .count();
    assert_eq!(q2_nodes, 4, "a substrate node counts");
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "Q1")[0].detail,
        "QN"
    );
}

#[test]
fn a_model_defined_in_a_selected_library_section_binds_first() {
    let deck = "corners\n.lib \"models.lib\" SS\nM1 d g s b nch\n.end\n";
    let library =
        ".lib TT\n.model nch nmos vto=0.5\n.endl TT\n.lib SS\n.model nch nmos vto=0.6\n.endl SS\n";
    let index = DeckIndex::build(
        &[
            DeckIndexSource {
                identity: "deck.cir",
                text: deck,
            },
            DeckIndexSource {
                identity: "models.lib",
                text: library,
            },
        ],
        ExpressionDialect::Ngspice,
    );
    let target = reads(&index, "nch")[0].target.expect("nch is defined");
    assert_eq!(index.symbols()[target].section.as_deref(), Some("SS"));
    assert_eq!(index.symbols()[target].location.line, 5);
    assert_eq!(index.includes()[0].section.as_deref(), Some("SS"));
}

#[test]
fn ground_and_global_nodes_belong_to_the_top_level() {
    let deck = "inverter\n.global vdd\n.subckt inv in out\nM1 out in vdd vdd p\nM2 out in 0 0 n\n.ends\n.end\n";
    let index = DeckIndex::build(
        &[DeckIndexSource {
            identity: "lib.sp",
            text: deck,
        }],
        ExpressionDialect::Ngspice,
    );
    for node in index.nodes() {
        let global = matches!(node.name.as_str(), "vdd" | "0");
        assert_eq!(node.scope == 0, global, "{node:?}");
    }
}

#[test]
fn a_function_body_reads_no_parameter_of_a_formal_name() {
    let deck = "func\n.param x=1 k=2\n.func scale(x) {x*k}\nR1 a 0 {scale(3)+sin(x)}\n.end\n";
    let index = index(deck);
    let x_reads = reads(&index, "x");
    assert_eq!(x_reads.len(), 1, "only the use outside the body reads x");
    assert_eq!(x_reads[0].location.line, 4);
    assert_eq!(reads(&index, "k").len(), 1);
    let scale = reads(&index, "scale");
    assert_eq!(scale.len(), 1);
    assert_eq!(scale[0].kind, DeckSymbolKind::Function);
    assert!(
        reads(&index, "sin").is_empty(),
        "a built-in function is no read"
    );
}

#[test]
fn an_expression_written_with_blanks_is_one_field() {
    let deck = "spaced
.param k=2
R9 a b {1 * k}
C1 b 0 { k * 1p }
.end
";
    let index = index(deck);
    assert!(
        index
            .references()
            .iter()
            .all(|reference| reference.kind != DeckSymbolKind::Model),
        "no part of an expression is a model: {:?}",
        index.references()
    );
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "R9")[0].detail,
        "{1 * k}"
    );
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "C1")[0].detail,
        "{ k * 1p }"
    );
    assert_eq!(reads(&index, "k").len(), 2);
    let nodes = index
        .nodes()
        .iter()
        .map(|node| node.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(nodes, ["a", "b", "b", "0"]);
}
