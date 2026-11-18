//! Netlist workflow tests.

/// Format writes each continuation line `+ ` and its words, and sets the
/// `name=value` pairs of a card's continuation lines in columns; the title,
/// comments and an inline comment stay as written, and a Spectre line,
/// which `+` does not continue, too.
#[test]
fn format_aligns_continuation_lines_and_their_parameter_columns() {
    let deck = "  amp  title  \r\n\
                * a comment   kept\r\n\
                M1 d g 0 0 nch\r\n\
                +W=10u   L=180n\r\n\
                +   nf=2 m=1\r\n\
                R1 a b 1k\r\n\
                +    tc1=0  ; first order\r\n\
                +\r\n\
                .end";
    assert_eq!(
        super::formatted_deck_text(deck, false),
        "  amp  title  \r\n\
         * a comment   kept\r\n\
         M1 d g 0 0 nch\r\n\
         + W=10u L=180n\r\n\
         + nf=2  m=1\r\n\
         R1 a b 1k\r\n\
         + tc1=0  ; first order\r\n\
         +\r\n\
         .end\r\n"
    );
    let spectre = "spectre deck\nr1 (a b) resistor r=1k \\\n+   tc1=0\n";
    assert_eq!(super::formatted_deck_text(spectre, true), spectre);
    let switched = "deck\nsimulator lang=spectre\n+  x=1\nsimulator lang=spice\n+  y=2\n";
    assert_eq!(
        super::formatted_deck_text(switched, false),
        "deck\nsimulator lang=spectre\n+  x=1\nsimulator lang=spice\n+ y=2\n"
    );
}

#[test]
fn deterministic_netlist_format_preserves_line_ending_policy() {
    assert_eq!(
        super::normalize_owned_netlist_whitespace("deck  \r\nR1 1 0 1k\t\r\n.end"),
        "deck  \r\nR1 1 0 1k\r\n.end\r\n"
    );
    assert_eq!(
        super::normalize_owned_netlist_whitespace("deck\n.end\n"),
        "deck\n.end\n"
    );
}
