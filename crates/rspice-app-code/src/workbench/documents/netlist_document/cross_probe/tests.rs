use super::{CaretName, name_on};
use crate::state::NetlistSourceIndex;

const DECK: &str = "* amplifier\n\
V1 in 0 SIN(0 1 1k)\n\
R1 in out 1k\n\
L1 out 0 1u\n\
L2 b 0 1u\n\
K1 L1 L2 0.9\n\
.subckt buf a y\n\
R9 a y 10\n\
.ends\n\
X1 out mid buf\n\
.end\n";

fn name(line: usize, column: usize) -> Option<CaretName> {
    let index = NetlistSourceIndex::parse(DECK);
    name_on(index.deck(), line, column)
}

fn node(name: &str, top_level: bool) -> Option<CaretName> {
    Some(CaretName::Node {
        name: name.to_owned(),
        top_level,
    })
}

fn element(name: &str, top_level: bool, head: bool) -> Option<CaretName> {
    Some(CaretName::Element {
        name: name.to_owned(),
        top_level,
        head,
    })
}

#[test]
fn a_caret_in_or_at_either_end_of_a_node_reads_that_node() {
    // `R1 in out 1k`: `out` starts at column 6.
    assert_eq!(name(3, 6), node("out", true));
    assert_eq!(name(3, 7), node("out", true));
    assert_eq!(name(3, 9), node("out", true));
    assert_eq!(name(3, 10), None, "the value is not a name");
}

#[test]
fn a_caret_on_an_element_name_reads_the_element_its_card_is_headed_by() {
    assert_eq!(name(3, 0), element("R1", true, true));
    assert_eq!(name(10, 1), element("X1", true, true));
}

#[test]
fn a_coupling_card_reads_the_inductors_it_couples() {
    assert_eq!(name(6, 0), element("K1", true, true));
    assert_eq!(name(6, 3), element("L1", true, false));
}

#[test]
fn a_node_in_a_subcircuit_body_is_not_a_node_of_the_deck() {
    assert_eq!(name(8, 3), node("a", false));
    assert_eq!(name(8, 0), element("R9", false, true));
}

#[test]
fn a_comment_or_a_control_card_has_no_name_to_read() {
    assert_eq!(name(1, 3), None);
    assert_eq!(name(11, 1), None);
}
