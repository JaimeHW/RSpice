//! Parser policy for Xyce-style Y-device keywords.
//!
//! RSpice uses `Y` for legacy lossy transmission lines. Xyce also has
//! keyword-style Y devices such as `YDELAY`, `YMEMRISTOR`, and `YLIN`; those
//! must fail explicitly until native implementations exist, not parse as a
//! shifted-node transmission line.

use rspice_core::netlist::Netlist;

#[test]
fn xyce_ydevice_keywords_fail_closed_instead_of_yline_parse() {
    let decks = [
        (
            "YDELAY",
            "* Xyce delay Y-device\nYDELAY delay1 2 0 1 0 TD=10N\n.op\n.end\n",
        ),
        (
            "YMEMRISTOR",
            "* Xyce memristor Y-device\nymemristor mr1 n1 n2 mrm1\n.op\n.end\n",
        ),
        (
            "YLIN",
            "* Xyce linear Y-device\nYLIN YLIN1 1 0 2 0 YLIN_MOD1\n.op\n.end\n",
        ),
    ];

    for (keyword, deck) in decks {
        let message = Netlist::parse(deck)
            .expect_err("Xyce Y-device keyword must fail before Y-line parsing")
            .to_string();
        assert!(
            message.contains(keyword)
                && message.contains("unsupported")
                && message.contains("native"),
            "{keyword} error should identify unsupported native Y-device, got: {message}"
        );
    }
}

#[test]
fn ordinary_yline_transmission_line_still_parses() {
    let deck = "* ordinary Y-line transmission line\n\
                Y1 in 0 out 0 Z0=50 TD=1n\n\
                .op\n\
                .end\n";

    let netlist = Netlist::parse(deck).expect("ordinary Y-line transmission line parses");
    assert_eq!(netlist.elements.len(), 1);
    assert_eq!(netlist.elements[0].name, "Y1");
}
