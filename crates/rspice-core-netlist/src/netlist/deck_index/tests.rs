use super::*;

fn index(deck: &str) -> DeckIndex {
    DeckIndex::build(&[DeckIndexSource::copied(
        "deck.cir",
        deck,
        SourceKind::DeckRoot,
    )])
}

fn symbol<'a>(index: &'a DeckIndex, kind: DeckSymbolKind, name: &str) -> Vec<DeckSymbol<'a>> {
    index
        .symbols()
        .filter(|symbol| symbol.kind() == kind && symbol.name() == name)
        .collect()
}

fn reads<'a>(index: &'a DeckIndex, name: &str) -> Vec<DeckReference<'a>> {
    index
        .references()
        .filter(|reference| reference.name().eq_ignore_ascii_case(name))
        .collect()
}

#[test]
fn a_number_with_a_suffix_reads_no_parameter_of_the_suffix_name() {
    let deck =
        "turns\n.param n=10\n.param cload=10n\nC1 a 0 10n\nC2 a 0 {n*1p}\nR1 a 0 {2*n}\n.end\n";
    let index = index(deck);
    let n = index
        .symbols()
        .position(|symbol| symbol.name() == "n")
        .expect("n is defined");
    let lines = index
        .references_to(n)
        .map(|reference| reference.location().line)
        .collect::<Vec<_>>();
    assert_eq!(lines, [5, 6], "only the expressions read n");
    let tokens = index.card_tokens(index.card_on_line(0, 4).unwrap());
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
        .position(|symbol| symbol.name() == "gain" && symbol.location().line == 3)
        .expect("the formal is a definition");
    let global = index
        .symbols()
        .position(|symbol| symbol.name() == "gain" && symbol.location().line == 2)
        .expect("the global is a definition");
    assert_ne!(
        index.symbol(formal).unwrap().scope(),
        0,
        "the formal is local"
    );
    let inside = reads(&index, "gain")
        .into_iter()
        .find(|reference| reference.location().line == 4)
        .expect("the body reads gain");
    assert_eq!(inside.target(), Some(formal));
    let outside = reads(&index, "gain")
        .into_iter()
        .find(|reference| reference.location().line == 7)
        .expect("the top level reads gain");
    assert_eq!(outside.target(), Some(global));
    assert!(
        reads(&index, "gain")
            .iter()
            .all(|reference| reference.location().line != 6),
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
    assert_eq!(w0[0].location().line, 4);
    assert_eq!(w0[0].location().column, 6);
    assert_eq!(&deck[w0[0].location().span.clone()], "w0");
    assert!(w0[0].target().is_some());
    let l0 = reads(&index, "l0");
    assert_eq!(l0.len(), 1, "a bare value reads its parameter");
    let card = index
        .card_on_line(0, 4)
        .expect("the continuation is the card's");
    assert_eq!(index.card(card).unwrap().lines(), 3..=4);
    assert_eq!(index.card(card).unwrap().head(), "M1");
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
    let opamp_index = opamp[0].id();
    let use_site = reads(&index, "opamp")
        .into_iter()
        .find(|reference| reference.location().line == 6)
        .expect("the X card names its subcircuit on the continuation");
    assert_eq!(use_site.target(), Some(opamp_index));
    let x1 = symbol(&index, DeckSymbolKind::Instance, "X1");
    assert_eq!(x1[0].target(), Some(opamp_index));
    let x1_nodes = index
        .nodes()
        .filter(|node| node.card() == x1[0].card())
        .map(|node| node.name())
        .collect::<Vec<_>>();
    assert_eq!(
        x1_nodes,
        ["in", "out", "vdd", "vss"],
        "out is a node, not the subcircuit"
    );
    assert_eq!(
        index
            .scope(opamp[0].opens().unwrap())
            .unwrap()
            .ports()
            .len(),
        5
    );
}

#[test]
fn a_read_in_an_include_file_carries_that_files_identity() {
    let deck = "top\n.include \"models/lib.sp\"\n.param vt=0.7\nD1 a 0 dmod\n.end\n";
    let library = "* library\n.model dmod d is=1e-14 vj={vt}\n";
    let index = DeckIndex::build(&[
        DeckIndexSource::copied("deck.cir", deck, SourceKind::DeckRoot),
        DeckIndexSource::copied("models/lib.sp", library, SourceKind::Included),
    ]);
    let vt = reads(&index, "vt");
    assert_eq!(vt.len(), 1);
    assert_eq!(
        index.file_identity(vt[0].location().file),
        Some("models/lib.sp")
    );
    assert_eq!(vt[0].location().line, 2);
    let dmod_definition = index
        .symbol(reads(&index, "dmod")[0].target().unwrap())
        .unwrap();
    assert_eq!(
        index.file_identity(dmod_definition.location().file),
        Some("models/lib.sp")
    );
    assert_eq!(index.includes().next().unwrap().path(), "models/lib.sp");
    assert_eq!(
        index
            .cards()
            .filter(|card| card.kind() == DeckCardKind::Title)
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
        .map(|card| (card.kind(), card.lines()))
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
fn an_analysis_or_options_card_is_known_by_the_parser_s_own_spellings() {
    let deck = "bench
R1 a 0 1k
.tr 1n 1u
.lin sparcalc=1
.mc 10
.opts reltol=1e-4
                .temp 27
.end
";
    let kinds = index(deck)
        .cards()
        .map(|card| (card.head().into_owned(), card.kind()))
        .collect::<Vec<_>>();
    for (head, kind) in [
        (".tr", DeckCardKind::Analysis),
        (".lin", DeckCardKind::Analysis),
        (".mc", DeckCardKind::Analysis),
        (".opts", DeckCardKind::Option),
        (".temp", DeckCardKind::Directive),
    ] {
        assert!(
            kinds.contains(&(head.to_owned(), kind)),
            "{head} is not {kind:?}: {kinds:?}"
        );
    }
}

#[test]
fn a_transistor_reads_the_model_its_fields_name_not_its_area() {
    let deck =
        "bjt\nQ1 c b e QN 2\nQ2 c b e s QN\nD1 c 0 1N4148\n.model QN npn\n.model 1N4148 d\n.end\n";
    let index = index(deck);
    let model = reads(&index, "QN");
    assert_eq!(model.len(), 2);
    assert!(model.iter().all(|reference| reference.target().is_some()));
    assert!(reads(&index, "2").is_empty());
    assert!(
        reads(&index, "1N4148")[0].target().is_some(),
        "a digit-led model name is a name"
    );
    let q2_nodes = index
        .nodes()
        .filter(|node| node.location().line == 3)
        .count();
    assert_eq!(q2_nodes, 4, "a substrate node counts");
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "Q1")[0].detail(),
        "QN"
    );
}

#[test]
fn a_model_defined_in_a_selected_library_section_binds_first() {
    let deck = "corners\n.lib \"models.lib\" SS\nM1 d g s b nch\n.end\n";
    let library =
        ".lib TT\n.model nch nmos vto=0.5\n.endl TT\n.lib SS\n.model nch nmos vto=0.6\n.endl SS\n";
    let index = DeckIndex::build(&[
        DeckIndexSource::copied("deck.cir", deck, SourceKind::DeckRoot),
        DeckIndexSource::copied("models.lib", library, SourceKind::Included),
    ]);
    let target = reads(&index, "nch")[0].target().expect("nch is defined");
    assert_eq!(
        index.symbol(target).unwrap().section().as_deref(),
        Some("SS")
    );
    assert_eq!(index.symbol(target).unwrap().location().line, 5);
    assert_eq!(
        index.includes().next().unwrap().section().as_deref(),
        Some("SS")
    );
}

#[test]
fn ground_and_global_nodes_belong_to_the_top_level() {
    let deck = "inverter\n.global vdd\n.subckt inv in out\nM1 out in vdd vdd p\nM2 out in 0 0 n\n.ends\n.end\n";
    let index = DeckIndex::build(&[DeckIndexSource::copied(
        "lib.sp",
        deck,
        SourceKind::DeckRoot,
    )]);
    for node in index.nodes() {
        let global = matches!(&*node.name(), "vdd" | "0");
        assert_eq!(node.scope() == 0, global, "{node:?}");
    }
}

#[test]
fn a_function_body_reads_no_parameter_of_a_formal_name() {
    let deck = "func\n.param x=1 k=2\n.func scale(x) {x*k}\nR1 a 0 {scale(3)+sin(x)}\n.end\n";
    let index = index(deck);
    let x_reads = reads(&index, "x");
    assert_eq!(x_reads.len(), 1, "only the use outside the body reads x");
    assert_eq!(x_reads[0].location().line, 4);
    assert_eq!(reads(&index, "k").len(), 1);
    let scale = reads(&index, "scale");
    assert_eq!(scale.len(), 1);
    assert_eq!(scale[0].kind(), DeckSymbolKind::Function);
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
            .all(|reference| reference.kind() != DeckSymbolKind::Model),
        "no part of an expression is a model: {:?}",
        index.references().collect::<Vec<_>>()
    );
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "R9")[0].detail(),
        "{1 * k}"
    );
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "C1")[0].detail(),
        "{ k * 1p }"
    );
    assert_eq!(reads(&index, "k").len(), 2);
    let nodes = index.nodes().map(|node| node.name()).collect::<Vec<_>>();
    assert_eq!(nodes, ["a", "b", "b", "0"]);
}

fn instance_names(index: &DeckIndex) -> Vec<String> {
    index
        .symbols()
        .filter(|symbol| symbol.kind() == DeckSymbolKind::Instance)
        .map(|symbol| symbol.name().into_owned())
        .collect()
}

#[test]
fn comments_are_where_the_engine_reads_them_and_a_pspice_node_is_no_comment() {
    let deck = "pspice\n# a line the engine skips\nV_V1 in 0 1\nR1 in $N_0001 1k $load\nC1 $N_0001 0 1n\n.end\n";
    let index = index(deck);
    assert_eq!(instance_names(&index), ["V_V1", "R1", "C1"]);
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "R1")[0].detail(),
        "1k"
    );
    assert!(
        index.nodes().any(|node| node.name() == "$N_0001"),
        "a PSpice $N_ name is a node"
    );
    assert!(
        reads(&index, "load").is_empty(),
        "the comment reads nothing"
    );
}

#[test]
fn a_spectre_source_is_read_as_spectre_past_its_block_comments() {
    let source = "// spectre deck\nsimulator lang=spectre\n/* a block comment\n   swept over two lines */\nparameters vdd=1.8\nsubckt amp (in out)\nparameters gain=2\nR1 (in out) resistor r=1k*gain\nends amp\nV1 (vdd 0) vsource dc=vdd\nX1 (vdd out) amp\ntran1 tran stop=1u\n";
    let index = DeckIndex::build(&[DeckIndexSource::copied(
        "amp.scs",
        source,
        SourceKind::Spectre,
    )]);
    assert_eq!(instance_names(&index), ["R1", "V1", "X1"]);
    let amp = &symbol(&index, DeckSymbolKind::Subcircuit, "amp")[0];
    assert_eq!(amp.detail(), "2 ports");
    let scope = amp.opens().expect("the subcircuit opens a scope");
    assert_eq!(index.scope(scope).unwrap().ports(), ["in", "out"]);
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "R1")[0].scope(),
        scope
    );
    assert_eq!(
        symbol(&index, DeckSymbolKind::Parameter, "gain")[0].scope(),
        scope
    );
    let x1 = &symbol(&index, DeckSymbolKind::Instance, "X1")[0];
    assert_eq!(x1.detail(), "amp");
    assert_eq!(
        reads(&index, "amp")
            .iter()
            .filter(|read| read.target().is_some())
            .count(),
        2,
        "the instance and the ends both read the subcircuit"
    );
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "V1")[0].detail(),
        "vsource"
    );
    assert!(
        reads(&index, "vsource").is_empty(),
        "a primitive is no read"
    );
    assert_eq!(reads(&index, "vdd").len(), 1, "dc=vdd reads the parameter");
    let analysis = index
        .cards()
        .find(|card| card.head() == "tran1")
        .expect("the analysis is a card");
    assert_eq!(analysis.kind(), DeckCardKind::Analysis);
    assert!(
        index
            .cards()
            .all(|card| card.head() != "swept" && card.head() != "/*"),
        "nothing in a block comment is a card"
    );
}

#[test]
fn a_simulator_switch_hands_the_lines_after_it_to_spectre_and_back() {
    let deck = "* mixed\nR1 a 0 1k\nsimulator lang=spectre\nR2 (a b) resistor r=2k\n// a Spectre comment\nsimulator lang=spice\nR3 b 0 3k\n.end\n";
    let index = DeckIndex::build(&[DeckIndexSource::copied(
        "mixed.cir",
        deck,
        SourceKind::Included,
    )]);
    let names = instance_names(&index);
    assert_eq!(names, ["R1", "R2", "R3"]);
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "R2")[0].detail(),
        "resistor"
    );
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "R3")[0].detail(),
        "3k"
    );
}

#[test]
fn a_spectre_line_ending_in_a_backslash_is_continued_by_the_next() {
    let source = "simulator lang=spectre\nparameters vdd=1.8 \\\n    gain=2\nX1 (in \\\n  out) amp\nsubckt amp (a b)\nends amp\n";
    let index = DeckIndex::build(&[DeckIndexSource::copied(
        "amp.scs",
        source,
        SourceKind::Spectre,
    )]);
    let parameters = index
        .cards()
        .find(|card| card.head() == "parameters")
        .expect("the parameters statement is a card");
    assert_eq!(parameters.lines(), 2..=3, "one card over both lines");
    let gain = &symbol(&index, DeckSymbolKind::Parameter, "gain")[0];
    assert_eq!((gain.location().line, &*gain.detail()), (3, "2"));
    assert!(
        index.cards().all(|card| card.head() != "gain=2"),
        "the continued line is no card of its own"
    );
    let x1 = index.card_on_line(0, 5).expect("X1 spans line 5");
    assert_eq!(index.card(x1).unwrap().head(), "X1");
    assert_eq!(
        symbol(&index, DeckSymbolKind::Instance, "X1")[0].detail(),
        "amp"
    );
    let tokens = index.card_tokens(x1);
    assert!(
        tokens
            .iter()
            .all(|token| &source[token.span.clone()] != "\\"),
        "the backslash is no token"
    );
}

#[test]
fn a_spice_line_ending_in_a_backslash_is_not_continued() {
    let deck = "title\nR1 a b 1k \\\nR2 b 0 2k\n.end\n";
    let index = index(deck);
    assert_eq!(instance_names(&index), ["R1", "R2"]);
}

#[test]
fn a_voltage_probe_names_the_nodes_it_reads_and_a_current_probe_none() {
    let deck = "probes\nR1 out a 1k\nB1 b 0 V={v(out)*2}\n.tran 1n 10n\n\
                .print tran v(out) v(a,b) i(R1)\n.meas tran pk max v(out)\n.end\n";
    let index = index(deck);
    let probed = index
        .nodes()
        // Past B1's own terminals: what the probes name.
        .filter(|node| (node.location().line, node.location().column) > (3, 6))
        .map(|node| {
            assert_eq!(node.name(), &deck[node.location().span.clone()]);
            (&deck[node.location().span.clone()], node.location().line)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        probed,
        [("out", 3), ("out", 5), ("a", 5), ("b", 5), ("out", 6),]
    );
    assert!(index.nodes().all(|node| node.name() != "R1"));
}

#[test]
fn a_name_resolves_through_the_scopes_a_reader_sees() {
    let deck = "scopes\n.param w=1\n.subckt cell a b\n.param l=2\nR1 a b {w*l}\n.ends\n.end\n";
    let index = index(deck);
    let inner = index.scope_on_line(0, 5);
    let w = index.resolve(DeckSymbolKind::Parameter, "W", inner);
    assert_eq!(
        w.map(|symbol| index.symbol(symbol).unwrap().location().line),
        Some(2)
    );
    assert!(index.resolve(DeckSymbolKind::Parameter, "l", 0).is_none());
    assert!(
        index
            .resolve(DeckSymbolKind::Parameter, "l", inner)
            .is_some()
    );
}

/// The definitions a reader binds by scope are listed in source order without
/// the deck's devices, and resolve as they did among every name.
#[test]
fn definitions_leave_out_the_devices() {
    let index = index(
        "* t\n.param w=1\nR1 a b {w}\n.subckt amp in out\nR2 in out 1k\n.ends\n.model dm d\nX1 a b amp\nD1 a 0 dm\n",
    );
    let defined = index
        .definitions()
        .map(|symbol| (symbol.kind(), symbol.name().into_owned()))
        .collect::<Vec<_>>();
    assert_eq!(
        defined
            .iter()
            .map(|(kind, name)| (*kind, name.as_str()))
            .collect::<Vec<_>>(),
        [
            (DeckSymbolKind::Parameter, "w"),
            (DeckSymbolKind::Subcircuit, "amp"),
            (DeckSymbolKind::Model, "dm"),
        ]
    );
    let devices = index
        .symbols()
        .filter(|symbol| symbol.kind() == DeckSymbolKind::Instance)
        .count();
    assert_eq!(devices + defined.len(), index.symbol_count());
    assert!(index.resolve(DeckSymbolKind::Model, "DM", 0).is_some());
    assert!(index.resolve(DeckSymbolKind::Instance, "x1", 0).is_some());
}

/// Finding a name by where it is written gives what reading every name did,
/// at every byte and on every line of a deck and an include that write names
/// edge to edge, nested in expressions and probes, in subcircuits and over
/// continuation lines.
#[test]
fn a_name_found_by_place_is_the_one_every_name_read_finds() {
    let deck = "\
places
.param w=1 l={w*2}
.func f(a) {a+w}
.subckt amp in out gain=2
R1 in out {gain*w}
X2 in mid sub
+ k=3
.ends
.subckt sub p q k=1
Rk p q {k}
.ends
.model dm d is=1e-14
X1 a b amp gain={f(l)}
D1 a 0 dm
K1 L1 L2 0.5
L1 a 0 1u
L2 b 0 1u
.save v(a,b) i(R1)
.end
";
    let include = "R9 x y {w}\nX9 x y amp\n";
    let index = DeckIndex::build(&[
        DeckIndexSource::copied("deck.cir", deck, SourceKind::DeckRoot),
        DeckIndexSource::copied("inc.lib", include, SourceKind::Included),
    ]);
    assert!(index.reference_count() > 0 && index.node_count() > 0);
    let scanned_name_at = |file: usize, byte: usize| {
        let covers = |location: &DeckLocation| {
            location.file == file && location.span.start <= byte && byte <= location.span.end
        };
        if let Some(at) = index
            .references()
            .position(|reference| covers(&reference.location()))
        {
            return Some(DeckNameAt::Reference(at));
        }
        index
            .symbols()
            .position(|symbol| covers(&symbol.location()))
            .map(DeckNameAt::Definition)
    };
    for (file, text) in [(0, deck), (1, include), (2, "")] {
        for byte in 0..=text.len() + 2 {
            assert_eq!(
                index.name_at(file, byte),
                scanned_name_at(file, byte),
                "file {file}, byte {byte}"
            );
        }
        for line in 0..=text.lines().count() + 1 {
            let on = |location: &DeckLocation| location.file == file && location.line == line;
            let scanned = |locations: Vec<(usize, DeckLocation)>| {
                locations
                    .into_iter()
                    .filter(|(_, location)| on(location))
                    .map(|(at, _)| at)
                    .collect::<Vec<_>>()
            };
            let mut symbols = index
                .symbols_on_line(file, line)
                .map(|(at, _)| at)
                .collect::<Vec<_>>();
            symbols.sort_unstable();
            assert_eq!(
                symbols,
                scanned(index.symbols().map(|s| s.location()).enumerate().collect()),
                "symbols on line {line} of file {file}"
            );
            assert_eq!(
                index
                    .references_on_line(file, line)
                    .map(|(at, _)| at)
                    .collect::<Vec<_>>(),
                scanned(
                    index
                        .references()
                        .map(|r| r.location())
                        .enumerate()
                        .collect()
                ),
                "reads on line {line} of file {file}"
            );
            assert_eq!(
                index
                    .nodes_on_line(file, line)
                    .map(|(at, _)| at)
                    .collect::<Vec<_>>(),
                scanned(index.nodes().map(|n| n.location()).enumerate().collect()),
                "nodes on line {line} of file {file}"
            );
        }
    }
    for symbol in 0..index.symbol_count() + 1 {
        let found = index.references_to(symbol).collect::<Vec<_>>();
        let scanned = index
            .references()
            .filter(|reference| reference.target() == Some(symbol))
            .collect::<Vec<_>>();
        assert_eq!(found, scanned, "the reads of symbol {symbol}");
    }
    let bound = (0..index.symbol_count())
        .filter(|symbol| index.references_to(*symbol).next().is_some())
        .count();
    assert!(
        bound >= 4,
        "the deck reads w, f, l, gain, k, amp, sub and dm"
    );
}

/// A deck of `blocks` subcircuits and an instance of each, long enough to be
/// read in several pages.
fn long_deck(blocks: usize) -> String {
    use std::fmt::Write as _;
    let mut deck =
        String::from("long deck\n.param w=1 l={w*2}\n.global vdd\n.model dm d is=1e-14\n");
    for block in 0..blocks {
        let _ = write!(
            deck,
            ".subckt cell{block} a b k=1\nR1 a b {{k*w}}\nD1 a vdd dm\nX2 a mid\n+ sub\n.ends cell{block}\nX{block} n{block} n{} cell{block} k={{l}}\n",
            block + 1
        );
    }
    deck.push_str(".subckt sub p q\nR1 p q 1k\n.ends sub\n.end\n");
    deck
}

/// The first place two runs of views differ, if any.
fn first_difference<T: PartialEq + std::fmt::Debug>(
    what: &str,
    one: impl Iterator<Item = T>,
    two: impl Iterator<Item = T>,
) -> Option<String> {
    let mut one = one.fuse();
    let mut two = two.fuse();
    for at in 0.. {
        match (one.next(), two.next()) {
            (None, None) => return None,
            (left, right) if left != right => {
                return Some(format!("{what} {at}: {left:?} against {right:?}"));
            }
            _ => {}
        }
    }
    None
}

/// An index updated through an edit answers what one built from the edited
/// text answers: an insertion, a deletion and a replacement, inside a
/// statement, across lines and on a scope's `.subckt` and `.ends` lines. An
/// edit that changes what the deck defines or how the lines after it read
/// takes a new build; every other one reads its own page again and shares
/// the rest.
#[test]
fn an_index_updated_through_an_edit_answers_as_one_built_from_the_edited_text() {
    let deck = long_deck(1500);
    let anchor = deck.find(".subckt cell700 ").expect("block 700");
    let build = |text: &SharedDeckText| {
        DeckIndex::build(&[DeckIndexSource::new(
            "deck.cir",
            Arc::clone(text),
            SourceKind::DeckRoot,
        )])
    };
    let old: SharedDeckText = Arc::new(deck.clone());
    let index = build(&old);
    assert!(index.pages.len() >= 3, "the deck is read in several pages");
    // What is edited, the text after block 700 opens it replaces, the
    // replacement, and whether the edit reads its own page again.
    let cases: &[(&str, &str, &str, bool)] = &[
        ("a value inside a statement", "{k*w}", "{k*w*2}", true),
        ("a device inserted", "R1 a b", "R9 a b {l}\nR1 a b", true),
        ("a device deleted", "D1 a vdd dm\n", "", true),
        (
            "a continuation joined across lines",
            "mid\n+ sub",
            "mid sub",
            true,
        ),
        (
            "an edit across two statements",
            "{k*w}\nD1 a",
            "{k}\nD7 a",
            true,
        ),
        (
            "a port added on the .subckt line",
            "cell700 a b",
            "cell700 a b c",
            true,
        ),
        (
            "the name on the .ends line",
            ".ends cell700",
            ".ends cellx",
            true,
        ),
        (
            "the subcircuit renamed on its .subckt line",
            "cell700 a b",
            "cellx a b",
            false,
        ),
        ("the .ends line deleted", ".ends cell700\n", "", false),
        ("a parameter defined", "R1 a b", ".param z=1\nR1 a b", false),
        ("a .end inserted", "D1 a vdd dm\n", ".end\n", false),
    ];
    for (what, replaced, replacement, incremental) in cases {
        let at = anchor + deck[anchor..].find(replaced).expect("the text to replace");
        let edited_text = format!(
            "{}{replacement}{}",
            &deck[..at],
            &deck[at + replaced.len()..]
        );
        let new: SharedDeckText = Arc::new(edited_text.clone());
        let change = DeckTextChange::between(&*old, &*new).expect("the texts differ");
        assert!(change.start >= at, "{what}");
        assert_eq!(
            &deck[..change.start],
            &edited_text[..change.start],
            "{what}"
        );
        assert_eq!(
            &deck[change.old_end..],
            &edited_text[change.new_end..],
            "{what}"
        );
        let updated = index.edited(0, change, Arc::clone(&new));
        assert_eq!(updated.is_some(), *incremental, "{what}");
        let Some(updated) = updated else {
            continue;
        };
        let built = build(&new);
        let shared = index
            .pages
            .iter()
            .zip(&updated.pages)
            .filter(|(one, two)| Arc::ptr_eq(&one.page, &two.page))
            .count();
        assert_eq!(shared, index.pages.len() - 1, "{what}: one page read again");
        let difference = first_difference("card", updated.cards(), built.cards())
            .or_else(|| first_difference("symbol", updated.symbols(), built.symbols()))
            .or_else(|| first_difference("read", updated.references(), built.references()))
            .or_else(|| first_difference("node", updated.nodes(), built.nodes()))
            .or_else(|| first_difference("include", updated.includes(), built.includes()))
            .or_else(|| first_difference("scope", updated.scopes(), built.scopes()));
        assert_eq!(difference, None, "{what}");
        assert!(updated == built, "{what}");
        let around = change.start.saturating_sub(80)..(change.new_end + 80).min(new.len());
        for byte in around.clone() {
            assert_eq!(
                updated.name_at(0, byte),
                built.name_at(0, byte),
                "{what}: the name at {byte}"
            );
            assert_eq!(
                updated.definition_at(0, byte),
                built.definition_at(0, byte),
                "{what}: the definition at {byte}"
            );
        }
        let first_line = edited_text[..around.start].lines().count();
        for line in first_line..first_line + 8 {
            let card = updated.card_on_line(0, line);
            assert_eq!(card, built.card_on_line(0, line), "{what}: line {line}");
            if let Some(card) = card {
                assert_eq!(updated.card_tokens(card), built.card_tokens(card), "{what}");
            }
            assert_eq!(
                updated.scope_on_line(0, line),
                built.scope_on_line(0, line),
                "{what}: the scope of line {line}"
            );
        }
    }
}

/// A text held in pieces of a few bytes, as a rope holds a large one.
struct Pieces(Vec<String>);

impl DeckText for Pieces {
    fn len(&self) -> usize {
        self.0.iter().map(String::len).sum()
    }

    fn slice(&self, range: Range<usize>) -> Cow<'_, str> {
        Cow::Owned(self.0.concat().get(range).unwrap_or_default().to_owned())
    }

    fn pieces(&self, from: usize) -> Box<dyn Iterator<Item = &str> + '_> {
        let mut skip = from;
        Box::new(self.0.iter().filter_map(move |piece| {
            let cut = skip.min(piece.len());
            skip -= cut;
            (cut < piece.len()).then(|| &piece[cut..])
        }))
    }

    fn pieces_back(&self, to: usize) -> Box<dyn Iterator<Item = &str> + '_> {
        let mut start = 0;
        let mut held = Vec::new();
        for piece in &self.0 {
            if start < to {
                held.push(&piece[..(to - start).min(piece.len())]);
            }
            start += piece.len();
        }
        Box::new(held.into_iter().rev())
    }
}

/// The change between two texts is the run from the first byte that differs
/// to the last, however the texts are cut into pieces.
#[test]
fn the_change_between_two_texts_spans_from_the_first_difference_to_the_last() {
    let pieces = |text: &str, size: usize| {
        Pieces(
            text.as_bytes()
                .chunks(size)
                .map(|chunk| String::from_utf8(chunk.to_vec()).expect("ASCII"))
                .collect(),
        )
    };
    let old = "R1 a b 1k\nR2 b 0 2k\n";
    for (new, start, old_end, new_end) in [
        ("R1 a b 1k\nR2 b 0 3k\n", 17, 18, 18),
        ("R1 a b 1k\nR2 b 0 2k\nR3 b 0 4k\n", 20, 20, 30),
        ("R2 b 0 2k\n", 1, 11, 1),
        ("XR1 a b 1k\nR2 b 0 2k\n", 0, 0, 1),
    ] {
        for size in [1, 3, 7, 64] {
            for other in [1, 4, 64] {
                assert_eq!(
                    DeckTextChange::between(&pieces(old, size), &pieces(new, other)),
                    Some(DeckTextChange {
                        start,
                        old_end,
                        new_end
                    }),
                    "{new:?} in pieces of {size} and {other}"
                );
            }
        }
    }
    assert_eq!(
        DeckTextChange::between(&pieces(old, 3), &pieces(old, 5)),
        None
    );
}

/// Two texts that write a character differently differ from its first byte
/// to its last: the change never splits a character, however the texts are
/// cut into pieces.
#[test]
fn the_change_between_two_texts_never_splits_a_character() {
    let pieces = |text: &str, size: usize| {
        let characters = text.chars().collect::<Vec<_>>();
        Pieces(
            characters
                .chunks(size)
                .map(|chunk| chunk.iter().collect())
                .collect(),
        )
    };
    for (old, new, start, old_end, new_end) in [
        ("a é b\n", "a è b\n", 2, 4, 4),
        ("x€y", "x₤y", 1, 4, 4),
        ("aéz", "a©z", 1, 3, 3),
        ("é€", "é₤", 2, 5, 5),
    ] {
        for size in [1, 2, 3, 64] {
            for other in [1, 2, 64] {
                assert_eq!(
                    DeckTextChange::between(&pieces(old, size), &pieces(new, other)),
                    Some(DeckTextChange {
                        start,
                        old_end,
                        new_end
                    }),
                    "{old:?} to {new:?} in pieces of {size} and {other}"
                );
            }
        }
    }
}

/// Counts what letting go of an index shows it.
#[derive(Default)]
struct Counted {
    tables: usize,
    pages: usize,
}

impl PageRelease for Counted {
    fn release(&mut self, _: &mut [MaybeUninit<u8>]) {
        self.tables += 1;
    }

    fn page_freed(&mut self) {
        self.pages += 1;
    }
}

/// Letting go of an index frees, page by page, every page it holds the
/// last share of; a page another index still shares is freed by that one.
#[test]
fn letting_go_of_an_index_frees_every_page_it_holds_the_last_share_of() {
    let deck = (0..20_000)
        .map(|at| format!("R{at} node_{at} node_{} 1k\n", at + 1))
        .collect::<String>();
    let index = index(&deck);
    let pages = index
        .pages
        .iter()
        .map(|slot| Arc::downgrade(&slot.page))
        .collect::<Vec<_>>();
    assert!(pages.len() > 1, "the deck spans pages");
    let shared = index.clone();

    let mut first = Counted::default();
    index.let_go(&mut first);
    assert_eq!(first.pages, 0, "the clone shares every page");
    assert!(pages.iter().all(|page| page.strong_count() == 1));

    let mut last = Counted::default();
    shared.let_go(&mut last);
    assert_eq!(last.pages, pages.len(), "each page is freed once");
    assert_eq!(
        last.tables,
        pages.len() * 16,
        "every table of a page is shown"
    );
    assert!(
        pages.iter().all(|page| page.strong_count() == 0),
        "every page is freed"
    );
}
