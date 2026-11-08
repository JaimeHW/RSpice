//! The Verilog-A block words, the folds they make and the indentation a
//! closing word takes.

use super::*;

/// Only a word of the source opens or closes a block: the same word in a
/// comment, a string, an attribute or a block comment carried from the line
/// before is text.
#[test]
fn block_words_are_read_from_the_source_and_never_from_its_comments() {
    use BlockKind::{BeginEnd, Case, Conditional, Function, Module};
    let words = |line: &str| block_words(line, LineState::default()).0;
    assert_eq!(
        words("module amp(a); analog function real f;"),
        [BlockWord::Open(Module), BlockWord::Open(Function)]
    );
    assert_eq!(
        words("end else begin : named"),
        [BlockWord::Close(BeginEnd), BlockWord::Open(BeginEnd)]
    );
    assert_eq!(
        words("  case (x) 1: y = 0; endcase"),
        [BlockWord::Open(Case), BlockWord::Close(Case)]
    );
    assert_eq!(words("`ifdef FAST"), [BlockWord::Open(Conditional)]);
    assert_eq!(words("`elsif SLOW"), [BlockWord::Branch]);
    assert_eq!(words("`else"), [BlockWord::Branch]);
    assert_eq!(words("`endif // begin"), [BlockWord::Close(Conditional)]);
    assert!(words("x = 1; // begin end").is_empty());
    assert!(words(r#"$strobe("begin end");"#).is_empty());
    assert!(words("(* desc = \"end\" *) real x;").is_empty());
    assert!(words("begin_x = end_y;").is_empty());

    let (opened, inside) = block_words("x = 1; /* begin", LineState::default());
    assert!(opened.is_empty());
    let (closed, after) = block_words("end */ begin", inside);
    assert_eq!(closed, [BlockWord::Open(BeginEnd)]);
    assert_eq!(block_words("end", after).0, [BlockWord::Close(BeginEnd)]);
}

/// A Verilog-A source whose blocks nest every way a model writes them: a
/// function in a module, a conditional with two branches, `end else begin`,
/// a case, and block words in a block comment, a line comment and a string.
const VERILOGA_BLOCKS: &str = "/* a block comment: begin
   end */
module amp(inp, out);
  analog function real clip;
    input x;
    begin
      clip = x;
    end
  endfunction
  `ifdef FAST
    parameter real g = 2;
  `else
    parameter real g = 1;
  `endif
  analog begin
    if (g > 1) begin
      V(out) <+ g * V(inp);
    end else begin
      V(out) <+ V(inp); // end
    end
    case (g)
      1: $strobe(\"end begin\");
    endcase
  end
endmodule
";

#[test]
fn verilog_a_folds_each_block_from_its_opening_line_through_its_closing_one() {
    let model = TextDocumentModel::from_source(VERILOGA_BLOCKS);
    assert_eq!(
        veriloga_fold_regions(&model),
        vec![
            (2, 25),
            (3, 9),
            (5, 8),
            (9, 11),
            (11, 14),
            (14, 24),
            (15, 17),
            (17, 20),
            (20, 23),
        ],
        "an `else and an `end else begin` keep their line as the next branch's header"
    );
}

#[test]
fn a_closing_word_typed_alone_takes_its_line_back_to_its_block() {
    let typed = |source: &str| {
        let mut model = TextDocumentModel::from_source(source);
        model.set_selections(vec![TextSelection::caret(model.len_chars())]);
        dedent_closing_word(&mut model);
        let once = model.to_source();
        dedent_closing_word(&mut model);
        assert_eq!(model.to_source(), once, "a second reading changes nothing");
        assert_eq!(
            model.selections(),
            &[TextSelection::caret(model.len_chars())],
            "the caret stays after the word"
        );
        once
    };
    assert_eq!(
        typed("  analog begin\n    x = 1;\n    end"),
        "  analog begin\n    x = 1;\n  end"
    );
    assert_eq!(
        typed("  case (x)\n    1: begin\n      y = 0;\n    end\n    endcase"),
        "  case (x)\n    1: begin\n      y = 0;\n    end\n  endcase",
        "an `end` closed the item's block, so `endcase` closes the case"
    );
    assert_eq!(
        typed("`ifdef FAST\n    real g;\n    `else"),
        "`ifdef FAST\n    real g;\n`else"
    );
    assert_eq!(
        typed("`ifdef FAST\n    real g;\n`else\n    real h;\n    `endif"),
        "`ifdef FAST\n    real g;\n`else\n    real h;\n`endif"
    );
    // A word that only starts like one, a word after code, and a word
    // already at its block's indentation stay where they are.
    for unchanged in [
        "  analog begin\n    endpoint",
        "  analog begin\n    x = 1; end",
        "  analog begin\n  end",
        "    end",
    ] {
        assert_eq!(typed(unchanged), unchanged);
    }
}
