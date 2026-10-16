//! Connection truth the extractor owns: what joins, what does not, and the
//! name a joined net carries.

use super::*;
use crate::schematic::bus::{Bus, BusDeclaration};
use crate::schematic::wire::Wire;

fn extracted(document: &SchematicDocument) -> ExtractedConnectivity {
    extract(document, |_| None, |_| None)
}

/// A label spelled as a bus member names that member's bit, whether or not
/// any tap anywhere selects it: the array tool writes exactly these labels.
#[test]
fn a_label_spelled_as_a_bus_member_joins_the_bit_without_a_tap() {
    let document = SchematicDocument {
        buses: vec![
            Bus::new(
                1,
                vec![Point::origin(), Point::new(100, 0)],
                Some(BusDeclaration::parse("DATA[7:0]").expect("declaration")),
            )
            .expect("bus"),
        ],
        wires: vec![Wire::segment(2, Point::new(0, 40), Point::new(40, 40))],
        net_labels: vec![NetLabel::new(3, Point::new(20, 40), "DATA[3]")],
        ..SchematicDocument::default()
    };

    let extracted = extracted(&document);

    let labelled = extracted.net_at(Point::new(20, 40)).expect("the label's net");
    assert_eq!(labelled.spice_name(), "DATA#3");
    assert_eq!(
        extracted
            .nets
            .iter()
            .filter(|net| display_net_name(&document, &net.spice_name()) == "DATA[3]")
            .count(),
        1,
        "one net shows as DATA[3]"
    );
}
