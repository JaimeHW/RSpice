//! The one-line circuit inventory a run states, read off circuits the builder
//! elaborated.

fn built(deck: &str) -> crate::circuit::CircuitData {
    crate::engine::Engine::default()
        .build_circuit(&crate::Netlist::parse(deck).expect("the deck parses"))
        .expect("the deck builds")
}

#[test]
fn the_inventory_lists_eight_families_largest_first_then_the_rest() {
    let circuit = built(
        "inventory\nV1 in 0 1\nI1 0 b 1m\nR1 in a 1k\nR2 a 0 1k\nR3 b 0 1k\nR4 f 0 1k\n\
         C1 a 0 1p\nL1 a c 1n\nD1 c 0 dmod\nE1 e 0 a 0 2\nG1 0 f a 0 1m\n\
         Q1 a b 0 qmod\n.model dmod d (is=1e-14)\n.model qmod npn (bf=100)\n.end\n",
    );
    assert_eq!(
        circuit.inventory_line(),
        "Circuit: 6 nodes, 12 devices (4 resistor, 1 bjt, 1 capacitor, 1 diode, \
         1 inductor, 1 isource, 1 vccs, 1 vcvs, 1 other)"
    );
    assert_eq!(circuit.device_count(), 12);
}

#[test]
fn a_small_circuit_lists_every_family() {
    let circuit = built("divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n");
    assert_eq!(
        circuit.inventory_line(),
        "Circuit: 2 nodes, 3 devices (2 resistor, 1 vsource)"
    );
}
