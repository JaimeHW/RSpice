use super::*;

/// The class of each word of `line`, read from the start of a file, in order:
/// `(text, class)` for every span.
fn classes(line: &str) -> Vec<(&str, SyntaxClass)> {
    read(&[line]).remove(0)
}

/// Each line of `lines` read from the state the line before it left.
fn read<'a>(lines: &[&'a str]) -> Vec<Vec<(&'a str, SyntaxClass)>> {
    let mut state = LineState::default();
    lines
        .iter()
        .map(|line| {
            let (spans, exit) = tokenize_line(line, state);
            state = exit;
            spans
                .into_iter()
                .map(|span| (&line[span.range], span.class))
                .collect()
        })
        .collect()
}

fn class_of(line: &str, word: &str) -> SyntaxClass {
    classes(line)
        .into_iter()
        .find(|(text, _)| *text == word)
        .unwrap_or_else(|| panic!("{word:?} is a span of {line:?}: {:?}", classes(line)))
        .1
}

#[test]
fn every_reserved_word_of_the_language_reads_as_a_keyword() {
    for word in [
        "module",
        "macromodule",
        "endmodule",
        "connectmodule",
        "input",
        "output",
        "inout",
        "real",
        "integer",
        "string",
        "genvar",
        "parameter",
        "localparam",
        "aliasparam",
        "discipline",
        "enddiscipline",
        "nature",
        "endnature",
        "potential",
        "flow",
        "domain",
        "discrete",
        "continuous",
        "wire",
        "wreal",
        "reg",
        "ground",
        "analog",
        "initial",
        "always",
        "begin",
        "end",
        "if",
        "else",
        "case",
        "casex",
        "casez",
        "endcase",
        "default",
        "for",
        "while",
        "repeat",
        "forever",
        "function",
        "endfunction",
        "generate",
        "endgenerate",
        "from",
        "exclude",
        "inf",
        "abstol",
        "access",
        "units",
        "idt_nature",
        "ddt_nature",
        "posedge",
        "negedge",
        "connectrules",
        "endconnectrules",
    ] {
        assert_eq!(classes(word), [(word, SyntaxClass::Keyword)], "{word}");
    }
}

#[test]
fn disciplines_natures_and_access_functions_read_as_disciplines() {
    for (line, word) in [
        ("electrical a, b;", "electrical"),
        ("thermal t;", "thermal"),
        ("magnetic m;", "magnetic"),
        ("voltage v1;", "voltage"),
        ("potential Voltage;", "Voltage"),
        ("flow Current;", "Current"),
        ("discipline my_wire;", "my_wire"),
        ("nature my_nature;", "my_nature"),
        ("I(a, b) <+ x;", "I"),
        ("y = V(out);", "V"),
        ("Pwr(t) <+ p;", "Pwr"),
    ] {
        assert_eq!(class_of(line, word), SyntaxClass::Discipline, "{line}");
    }
    // An access name that is not called is an ordinary name.
    assert_eq!(class_of("V = 1;", "V"), SyntaxClass::Text);
}

#[test]
fn system_tasks_operators_and_contributions_have_their_own_classes() {
    assert_eq!(
        class_of("t = $temperature;", "$temperature"),
        SyntaxClass::SystemTask
    );
    assert_eq!(
        class_of("$strobe(\"x\");", "$strobe"),
        SyntaxClass::SystemTask
    );
    for (line, word) in [
        ("q = ddt(c * v);", "ddt"),
        ("x = idt(v, 0);", "idt"),
        ("y = transition(d, 0, 1n);", "transition"),
        ("h = laplace_nd(v, {1}, {1, 1});", "laplace_nd"),
        ("z = zi_zp (v, 1, 2, 1n);", "zi_zp"),
        ("i = limexp(v);", "limexp"),
        ("I(a) <+ white_noise(4, \"thermal\");", "white_noise"),
        ("if (analysis(\"ac\"))", "analysis"),
        ("@(cross(v, 1))", "cross"),
        ("@(timer(0, 1n))", "timer"),
        ("@(above(v))", "above"),
        ("@(initial_step)", "initial_step"),
    ] {
        assert_eq!(class_of(line, word), SyntaxClass::AnalogOperator, "{line}");
    }
    // A variable that shares an operator's name is not called.
    assert_eq!(class_of("ddt = 1;", "ddt"), SyntaxClass::Text);
    assert_eq!(class_of("I(a) <+ V(a);", "<+"), SyntaxClass::Contribution);
    assert_eq!(class_of("x <= y;", "<="), SyntaxClass::Text);
}

#[test]
fn numbers_keep_their_scale_factors_sizes_and_units() {
    for number in [
        "10", "1.5", "1e-3", "2.5E+6", "1_000", "1k", "2.5M", "3u", "4n", "5p", "6f", "7a", "8T",
        "9G", "10m", "1K", "1'b0", "4'hF", "8'sd255", "'hFF", "2'b1x", "12'o7_7",
    ] {
        let line = format!("x = {number};");
        assert_eq!(
            classes(&line),
            [
                ("x", SyntaxClass::Text),
                ("=", SyntaxClass::Text),
                (number, SyntaxClass::Number),
                (";", SyntaxClass::Text),
            ],
            "{number}"
        );
    }
    // A unit glued to a scale factor is part of the number.
    assert_eq!(
        classes("t = 10ns;"),
        [
            ("t", SyntaxClass::Text),
            ("=", SyntaxClass::Text),
            ("10n", SyntaxClass::Number),
            ("s", SyntaxClass::Number),
            (";", SyntaxClass::Text),
        ]
    );
}

#[test]
fn a_sized_literal_leaves_the_rest_of_its_line_as_code() {
    assert_eq!(
        classes("x = 1'b0; y = \"s\"; // done"),
        [
            ("x", SyntaxClass::Text),
            ("=", SyntaxClass::Text),
            ("1'b0", SyntaxClass::Number),
            (";", SyntaxClass::Text),
            ("y", SyntaxClass::Text),
            ("=", SyntaxClass::Text),
            ("\"s\"", SyntaxClass::String),
            (";", SyntaxClass::Text),
            ("// done", SyntaxClass::Comment),
        ]
    );
    assert_eq!(class_of("q = 4'hF + k;", "k"), SyntaxClass::Text);
}

#[test]
fn a_hash_is_a_delay_never_a_comment() {
    assert_eq!(
        classes("#10 x = 1;"),
        [
            ("#", SyntaxClass::Text),
            ("10", SyntaxClass::Number),
            ("x", SyntaxClass::Text),
            ("=", SyntaxClass::Text),
            ("1", SyntaxClass::Number),
            (";", SyntaxClass::Text),
        ]
    );
    assert_eq!(class_of("r1 #(.r(1k)) x(a, b);", "1k"), SyntaxClass::Number);
}

#[test]
fn block_comments_run_across_lines_and_end_where_they_close() {
    let lines = read(&[
        "x = 1; /* a comment",
        "   that goes on, \"with a quote",
        "and ends */ y = 2; /* one */ z = 3;",
        "w = 4;",
    ]);
    assert_eq!(
        lines[0],
        [
            ("x", SyntaxClass::Text),
            ("=", SyntaxClass::Text),
            ("1", SyntaxClass::Number),
            (";", SyntaxClass::Text),
            ("/* a comment", SyntaxClass::Comment),
        ]
    );
    assert_eq!(
        lines[1],
        [("   that goes on, \"with a quote", SyntaxClass::Comment)]
    );
    assert_eq!(lines[2][0], ("and ends */", SyntaxClass::Comment));
    assert_eq!(lines[2][1], ("y", SyntaxClass::Text));
    assert!(lines[2].contains(&("/* one */", SyntaxClass::Comment)));
    assert!(lines[2].contains(&("z", SyntaxClass::Text)));
    assert_eq!(lines[3][0], ("w", SyntaxClass::Text));
    // A comment marker inside a string is text of the string.
    assert_eq!(
        classes("s = \"/* not */\"; t = 1;")[2],
        ("\"/* not */\"", SyntaxClass::String)
    );
}

#[test]
fn strings_continue_over_an_escaped_line_break() {
    let lines = read(&["s = \"first \\", "second\"; t = 1;"]);
    assert_eq!(lines[0][2], ("\"first \\", SyntaxClass::String));
    assert_eq!(lines[1][0], ("second\"", SyntaxClass::String));
    assert_eq!(lines[1][2], ("t", SyntaxClass::Text));
    // An unterminated string without the escape ends with its line.
    let lines = read(&["s = \"open", "t = 1;"]);
    assert_eq!(lines[1][0], ("t", SyntaxClass::Text));
}

#[test]
fn attributes_read_as_attributes_on_one_line_or_several() {
    assert_eq!(
        classes("(* desc=\"gain\" *) parameter real a0 = 1e5;"),
        [
            ("(", SyntaxClass::Attribute),
            ("*", SyntaxClass::Attribute),
            ("desc", SyntaxClass::Attribute),
            ("=", SyntaxClass::Attribute),
            ("\"gain\"", SyntaxClass::String),
            ("*", SyntaxClass::Attribute),
            (")", SyntaxClass::Attribute),
            ("parameter", SyntaxClass::Keyword),
            ("real", SyntaxClass::Keyword),
            ("a0", SyntaxClass::ParameterName),
            ("=", SyntaxClass::Text),
            ("1e5", SyntaxClass::Number),
            (";", SyntaxClass::Text),
        ]
    );
    let lines = read(&["(* desc=\"x\",", "   units=\"V\" *) parameter real p = 1;"]);
    assert_eq!(lines[1][0], ("units", SyntaxClass::Attribute));
    assert!(lines[1].contains(&("parameter", SyntaxClass::Keyword)));
    assert!(lines[1].contains(&("p", SyntaxClass::ParameterName)));
    // `@(*)` is an event control, not an attribute.
    assert_eq!(class_of("@(*) x = 1;", "x"), SyntaxClass::Text);
}

#[test]
fn directives_and_macros_read_as_directives() {
    assert_eq!(
        classes("`include \"disciplines.vams\""),
        [
            ("`include", SyntaxClass::Directive),
            ("\"disciplines.vams\"", SyntaxClass::String),
        ]
    );
    assert_eq!(
        classes("`define GAIN 10"),
        [
            ("`define", SyntaxClass::Directive),
            ("GAIN", SyntaxClass::Directive),
            ("10", SyntaxClass::Number),
        ]
    );
    assert_eq!(class_of("x = `M_PI * 2;", "`M_PI"), SyntaxClass::Directive);
    assert_eq!(class_of("`ifdef DEBUG", "`ifdef"), SyntaxClass::Directive);
}

#[test]
fn declarations_name_modules_ports_and_parameters() {
    let lines = read(&[
        "module amp(inp, inn,",
        "           out);",
        "  input inp, inn;",
        "  inout [3:0] bus;",
        "  output out;",
        "  parameter real gain = 10 from (0:inf), offset = 0;",
        "  localparam integer n = 2, m = n * 2;",
        "  aliasparam g = gain;",
        "  real x;",
        "endmodule",
    ]);
    let class = |line: usize, word: &str| {
        lines[line]
            .iter()
            .find(|(text, _)| *text == word)
            .unwrap_or_else(|| panic!("{word} on line {line}: {:?}", lines[line]))
            .1
    };
    assert_eq!(class(0, "amp"), SyntaxClass::ModuleName);
    assert_eq!(class(0, "inp"), SyntaxClass::PortName);
    assert_eq!(class(1, "out"), SyntaxClass::PortName);
    assert_eq!(class(2, "inn"), SyntaxClass::PortName);
    assert_eq!(class(3, "bus"), SyntaxClass::PortName);
    assert_eq!(class(4, "out"), SyntaxClass::PortName);
    assert_eq!(class(5, "gain"), SyntaxClass::ParameterName);
    assert_eq!(class(5, "offset"), SyntaxClass::ParameterName);
    assert_eq!(class(6, "n"), SyntaxClass::ParameterName);
    assert_eq!(class(6, "m"), SyntaxClass::ParameterName);
    assert_eq!(class(7, "g"), SyntaxClass::ParameterName);
    assert_eq!(
        class(7, "gain"),
        SyntaxClass::Text,
        "a value is not declared"
    );
    assert_eq!(class(8, "x"), SyntaxClass::Text);
}

#[test]
fn a_refused_character_leaves_the_rest_of_the_line_as_code() {
    assert_eq!(
        classes("x = 1 ` ; y = 2;").last(),
        Some(&(";", SyntaxClass::Text))
    );
    assert_eq!(class_of("x = 1.; y = 2;", "y"), SyntaxClass::Text);
    assert_eq!(class_of("x = é + y;", "y"), SyntaxClass::Text);
}

/// Every token the compiler reads from a shipped compact model, read the
/// whole file at once, is a span of the same bytes when the editor reads the
/// file a line at a time, and no comment the editor finds covers one.
#[test]
fn line_by_line_reading_agrees_with_the_compiler_on_the_shipped_compact_models() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/veriloga/cmc");
    let mut files = Vec::new();
    let mut folders = vec![root];
    while let Some(folder) = folders.pop() {
        for entry in std::fs::read_dir(&folder).expect("the models folder is readable") {
            let path = entry.expect("a folder entry").path();
            if path.is_dir() {
                folders.push(path);
            } else if path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|extension| {
                    matches!(extension, "va" | "vams" | "include" | "inc" | "h")
                })
            {
                files.push(path);
            }
        }
    }
    assert!(files.len() >= 60, "{} sources", files.len());
    for path in files {
        let bytes = std::fs::read(&path).expect("a source is readable");
        let source = String::from_utf8_lossy(&bytes).replace("\r\n", "\n");
        let mut spans = Vec::new();
        let mut state = LineState::default();
        let mut line_start = 0;
        for line in source.split('\n') {
            let (line_spans, exit) = tokenize_line(line, state);
            state = exit;
            spans.extend(line_spans.into_iter().map(|span| SyntaxSpan {
                range: line_start + span.range.start..line_start + span.range.end,
                class: span.class,
            }));
            line_start += line.len() + 1;
        }
        let ranges = spans
            .iter()
            .map(|span| (span.range.clone(), span.class))
            .collect::<HashMap<_, _>>();
        let comments = spans
            .iter()
            .filter(|span| span.class == SyntaxClass::Comment)
            .map(|span| span.range.clone())
            .collect::<Vec<_>>();
        let mut tokens = 0;
        for range in compiler_tokens(&source) {
            tokens += 1;
            // A string continued over an escaped line break is one token to
            // the compiler and one span on each line it covers.
            let mut start = range.start;
            for piece in source[range.clone()].split('\n') {
                let piece = start..start + piece.len();
                start = piece.end + 1;
                assert!(
                    piece.is_empty() || ranges.contains_key(&piece),
                    "{}: the compiler's token {:?} at {range:?} is not a span of the editor's reading",
                    path.display(),
                    &source[range.clone()]
                );
            }
            assert!(
                !comments
                    .iter()
                    .any(|comment| comment.start < range.end && range.start < comment.end),
                "{}: a comment covers the token {:?}",
                path.display(),
                &source[range]
            );
        }
        assert!(tokens > 0, "{}", path.display());
    }
}

/// The byte ranges of the tokens the compiler's lexer reads from all of
/// `source`, stepping over what it refuses as the editor does.
fn compiler_tokens(source: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut resume = 0;
    while resume < source.len() {
        let mut lexer = Lexer::new(&source[resume..], line_source());
        let refused = loop {
            match lexer.next_token() {
                Ok(token) if token.kind == TokenKind::Eof => break None,
                Ok(token) => {
                    let start = resume + token.span.start as usize;
                    let end = if token.kind == TokenKind::Directive {
                        start + token.text.as_deref().map_or(0, str::len)
                    } else {
                        resume + token.span.end as usize
                    };
                    ranges.push(start..end);
                }
                Err(error) => break Some(error),
            }
        };
        let Some(error) = refused else {
            break;
        };
        let start = resume + error.span.start as usize;
        let end = resume + error.span.end as usize;
        if matches!(
            error.kind,
            LexerErrorKind::UnterminatedComment | LexerErrorKind::UnterminatedString
        ) {
            break;
        }
        resume = if end > start {
            end
        } else {
            start + source[start..].chars().next().map_or(1, char::len_utf8)
        };
    }
    ranges
}

/// An edit rereads the lines after it: opening a block comment turns every
/// line below into comment, and closing it turns them back.
#[test]
fn the_highlighter_rereads_the_lines_after_an_edit() {
    let source = "module m(a);\n  electrical a;\n  analog V(a) <+ 1;\nendmodule\n";
    let mut document = TextDocumentModel::from_source(source);
    let mut highlighter = VerilogAHighlighter::default();
    let class_of_line =
        |highlighter: &mut VerilogAHighlighter, document: &TextDocumentModel, line: usize| {
            let (spans, _) = highlighter.line(document, line);
            spans.first().map(|span| span.class)
        };
    assert_eq!(
        class_of_line(&mut highlighter, &document, 2),
        Some(SyntaxClass::Keyword)
    );
    assert_eq!(
        class_of_line(&mut highlighter, &document, 3),
        Some(SyntaxClass::Keyword)
    );
    assert!(document.synchronize_source(&source.replace("electrical a;", "electrical a; /*")));
    assert_eq!(
        class_of_line(&mut highlighter, &document, 2),
        Some(SyntaxClass::Comment)
    );
    assert_eq!(
        class_of_line(&mut highlighter, &document, 3),
        Some(SyntaxClass::Comment)
    );
    assert!(document.synchronize_source(source));
    assert_eq!(
        class_of_line(&mut highlighter, &document, 3),
        Some(SyntaxClass::Keyword)
    );
    let (spans, text) = highlighter.line(&document, 0);
    assert_eq!(text, "module m(a);");
    assert_eq!(spans[1].class, SyntaxClass::ModuleName);
}
