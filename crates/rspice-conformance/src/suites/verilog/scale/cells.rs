//! The cell library the scale circuits are mapped onto.
//!
//! Two families, and the difference between them is the point of the
//! `sec32`/`sec32n` pair: [`xor2`] instantiates the `xor` primitive, while
//! [`xor2n`] builds the same function out of four `nand` gates. A circuit is
//! written once against one of these and the other spelling is a different
//! netlist computing the same function.
//!
//! Everything here is fan-in 2 unless a caller asks for more. That is the
//! mapping a NAND2/NOR2/INV library forces, and it is why the gate counts
//! below resemble a synthesised netlist rather than a minimal equation count.
//! The few deliberately wide cells in the suite are written at their call
//! sites, where the reason for the width is visible.

use super::netlist::{Builder, Gate};

/// `~a`
pub fn inv(b: &mut Builder, p: &str, a: &str) -> String {
    b.not(p, a)
}

/// `~(a & c)`
pub fn nand2(b: &mut Builder, p: &str, a: &str, c: &str) -> String {
    b.two(Gate::Nand, p, a, c)
}

/// `a & c`, as the library builds it: a NAND and an inverter.
pub fn and2(b: &mut Builder, p: &str, a: &str, c: &str) -> String {
    let n = nand2(b, p, a, c);
    inv(b, p, &n)
}

/// `a | c`, as the library builds it: a NOR and an inverter.
pub fn or2(b: &mut Builder, p: &str, a: &str, c: &str) -> String {
    let n = b.two(Gate::Nor, p, a, c);
    inv(b, p, &n)
}

/// `a ^ c` as one `xor` primitive.
pub fn xor2(b: &mut Builder, p: &str, a: &str, c: &str) -> String {
    b.two(Gate::Xor, p, a, c)
}

/// `~(a ^ c)` as one `xnor` primitive.
pub fn xnor2(b: &mut Builder, p: &str, a: &str, c: &str) -> String {
    b.two(Gate::Xnor, p, a, c)
}

/// `a ^ c` from four NANDs.
///
/// The classic four-gate expansion: `n0 = ~(a&c)`, then `~(a & n0)` and
/// `~(c & n0)` NANDed together. Written out rather than reduced because the
/// whole point of the NAND twin of a circuit is that the XOR is *not* a
/// primitive there.
pub fn xor2n(b: &mut Builder, p: &str, a: &str, c: &str) -> String {
    let n0 = nand2(b, p, a, c);
    let n1 = nand2(b, p, a, &n0);
    let n2 = nand2(b, p, c, &n0);
    nand2(b, p, &n1, &n2)
}

/// `~(a ^ c)` from four NANDs and an inverter.
pub fn xnor2n(b: &mut Builder, p: &str, a: &str, c: &str) -> String {
    let x = xor2n(b, p, a, c);
    inv(b, p, &x)
}

/// Balanced fan-in-2 XOR tree using the `xor` primitive.
pub fn xor_tree(b: &mut Builder, p: &str, terms: &[String]) -> String {
    tree(b, p, terms, &xor2)
}

/// Balanced fan-in-2 XOR tree in NAND form.
pub fn xor_tree_n(b: &mut Builder, p: &str, terms: &[String]) -> String {
    tree(b, p, terms, &xor2n)
}

/// Balanced fan-in-2 OR tree in NAND/NOR form.
pub fn or_tree(b: &mut Builder, p: &str, terms: &[String]) -> String {
    tree(b, p, terms, &or2)
}

/// Balanced fan-in-2 AND tree in NAND form.
pub fn and_tree(b: &mut Builder, p: &str, terms: &[String]) -> String {
    tree(b, p, terms, &and2)
}

fn tree(
    b: &mut Builder,
    p: &str,
    terms: &[String],
    cell: &dyn Fn(&mut Builder, &str, &str, &str) -> String,
) -> String {
    assert!(!terms.is_empty(), "an empty tree has no value");
    let mut level = terms.to_vec();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        for pair in level.chunks(2) {
            if pair.len() == 1 {
                next.push(pair[0].clone());
            } else {
                next.push(cell(b, p, &pair[0], &pair[1]));
            }
        }
        level = next;
    }
    level.remove(0)
}

/// A two-way multiplexer in NAND form: `sel ? d1 : d0`, three gates.
///
/// `nsel` is passed in rather than derived so a column of muxes sharing one
/// select shares one inverter, which is what a mapper does.
pub fn mux2(b: &mut Builder, p: &str, sel: &str, nsel: &str, d0: &str, d1: &str) -> String {
    let hit = nand2(b, p, sel, d1);
    let miss = nand2(b, p, nsel, d0);
    nand2(b, p, &hit, &miss)
}

/// A one-hot multiplexer: `OR over i of (sel[i] & data[i])`.
///
/// The product terms come out of NANDs already inverted, so the OR of the
/// products is the AND of the inverted products, inverted once at the end.
/// `fan_in` is the width of the cells that AND tree is built from; the wider
/// ALUs select one of sixteen sources per bit and a two-input tree there would
/// cost more than the function units it selects between.
pub fn mux_onehot(
    b: &mut Builder,
    p: &str,
    sel: &[String],
    data: &[String],
    fan_in: usize,
) -> String {
    assert_eq!(
        sel.len(),
        data.len(),
        "a one-hot mux needs one select per input"
    );
    let terms: Vec<String> = sel
        .iter()
        .zip(data)
        .map(|(s, d)| nand2(b, p, s, d))
        .collect();
    let combined = b.reduce(Gate::And, p, &terms, fan_in);
    inv(b, p, &combined)
}

/// Full adder in NAND form: nine gates, `(sum, carry)`.
///
/// `n0 = ~(a & c)` and `n3 = ~((a^c) & cin)` are shared between the sum and
/// the carry, which is what makes the cell nine gates rather than eleven.
pub fn fa_nand(b: &mut Builder, p: &str, a: &str, c: &str, cin: &str) -> (String, String) {
    let n0 = nand2(b, p, a, c);
    let n1 = nand2(b, p, a, &n0);
    let n2 = nand2(b, p, c, &n0);
    let axc = nand2(b, p, &n1, &n2);
    let n3 = nand2(b, p, &axc, cin);
    let n4 = nand2(b, p, &axc, &n3);
    let n5 = nand2(b, p, cin, &n3);
    let sum = nand2(b, p, &n4, &n5);
    let carry = nand2(b, p, &n0, &n3);
    (sum, carry)
}

/// Half adder in NAND form: six gates, `(sum, carry)`.
pub fn ha_nand(b: &mut Builder, p: &str, a: &str, c: &str) -> (String, String) {
    let sum = xor2n(b, p, a, c);
    let carry = and2(b, p, a, c);
    (sum, carry)
}

/// A ripple-carry adder in NAND form. Returns `(sum bits, carry bits)` where
/// `carry[i]` is the carry *out* of bit `i`, so `carry.last()` is the carry
/// out of the adder and `carry[width - 2]` is what an overflow flag needs.
pub fn ripple_add(
    b: &mut Builder,
    p: &str,
    x: &[String],
    y: &[String],
    cin: &str,
) -> (Vec<String>, Vec<String>) {
    assert_eq!(x.len(), y.len(), "an adder's operands must be the same width");
    let mut sums = Vec::with_capacity(x.len());
    let mut carries = Vec::with_capacity(x.len());
    let mut carry = cin.to_string();
    for (left, right) in x.iter().zip(y) {
        let (sum, next) = fa_nand(b, p, left, right, &carry);
        sums.push(sum);
        carries.push(next.clone());
        carry = next;
    }
    (sums, carries)
}

/// A carry-save (3:2) compressor over three operands.
///
/// Returns `(sum, carry)`, where `carry[i]` is weighted at `2^(i+1)`; a caller
/// adds `carry` shifted one place left to `sum` to finish the addition.
pub fn compress_3to2(
    b: &mut Builder,
    p: &str,
    x: &[String],
    y: &[String],
    z: &[String],
) -> (Vec<String>, Vec<String>) {
    assert_eq!(x.len(), y.len());
    assert_eq!(x.len(), z.len());
    let mut sums = Vec::with_capacity(x.len());
    let mut carries = Vec::with_capacity(x.len());
    for index in 0..x.len() {
        let (sum, carry) = fa_nand(b, p, &x[index], &y[index], &z[index]);
        sums.push(sum);
        carries.push(carry);
    }
    (sums, carries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suites::verilog::scale::netlist::{Design, measure};

    /// Exhaustively check one combinational cell against a closure, by
    /// simulating the emitted gate list directly.
    fn check<F>(inputs: usize, build: F, truth: &dyn Fn(&[bool]) -> Vec<bool>)
    where
        F: Fn(&mut Builder) -> Vec<String>,
    {
        let mut b = Builder::new("probe");
        for index in 0..inputs {
            b.input(&format!("i{index}"), 1);
        }
        let outputs = build(&mut b);
        for (index, net) in outputs.iter().enumerate() {
            b.output(&format!("o{index}"), 1);
            b.drive(Gate::Buf, &format!("o{index}"), &[net.clone()]);
        }
        let module = b.finish();

        for pattern in 0..(1u32 << inputs) {
            let bits: Vec<bool> = (0..inputs).map(|i| pattern >> i & 1 == 1).collect();
            let mut values = std::collections::BTreeMap::new();
            for (index, bit) in bits.iter().enumerate() {
                values.insert(format!("i{index}"), *bit);
            }
            for gate in &module.gates {
                let ins: Vec<bool> = gate
                    .inputs
                    .iter()
                    .map(|net| *values.get(net).expect("a gate reads a driven net"))
                    .collect();
                values.insert(gate.output.clone(), gate.kind.eval(&ins));
            }
            let got: Vec<bool> = (0..outputs.len())
                .map(|index| values[&format!("o{index}")])
                .collect();
            assert_eq!(got, truth(&bits), "pattern {pattern:b}");
        }
    }

    #[test]
    fn the_nand_xor_matches_the_xor_primitive() {
        check(
            2,
            |b| vec![xor2n(b, "t", "i0", "i1")],
            &|bits| vec![bits[0] ^ bits[1]],
        );
        check(
            2,
            |b| vec![xnor2n(b, "t", "i0", "i1")],
            &|bits| vec![!(bits[0] ^ bits[1])],
        );
    }

    #[test]
    fn the_mapped_and_or_cells_match_their_operators() {
        check(2, |b| vec![and2(b, "t", "i0", "i1")], &|bits| {
            vec![bits[0] && bits[1]]
        });
        check(2, |b| vec![or2(b, "t", "i0", "i1")], &|bits| {
            vec![bits[0] || bits[1]]
        });
    }

    #[test]
    fn the_full_adder_matches_its_truth_table() {
        check(
            3,
            |b| {
                let (sum, carry) = fa_nand(b, "t", "i0", "i1", "i2");
                vec![sum, carry]
            },
            &|bits| {
                let total = bits.iter().filter(|bit| **bit).count();
                vec![total % 2 == 1, total >= 2]
            },
        );
    }

    #[test]
    fn the_half_adder_matches_its_truth_table() {
        check(
            2,
            |b| {
                let (sum, carry) = ha_nand(b, "t", "i0", "i1");
                vec![sum, carry]
            },
            &|bits| vec![bits[0] ^ bits[1], bits[0] && bits[1]],
        );
    }

    #[test]
    fn the_two_way_mux_selects() {
        check(
            3,
            |b| {
                let nsel = inv(b, "t", "i0");
                vec![mux2(b, "t", "i0", &nsel, "i1", "i2")]
            },
            &|bits| vec![if bits[0] { bits[2] } else { bits[1] }],
        );
    }

    #[test]
    fn a_ripple_adder_adds() {
        check(
            5,
            |b| {
                let x = vec!["i0".to_string(), "i1".to_string()];
                let y = vec!["i2".to_string(), "i3".to_string()];
                let (sums, carries) = ripple_add(b, "t", &x, &y, "i4");
                vec![sums[0].clone(), sums[1].clone(), carries[1].clone()]
            },
            &|bits| {
                let x = u32::from(bits[0]) + 2 * u32::from(bits[1]);
                let y = u32::from(bits[2]) + 2 * u32::from(bits[3]);
                let total = x + y + u32::from(bits[4]);
                vec![total & 1 == 1, total >> 1 & 1 == 1, total >> 2 & 1 == 1]
            },
        );
    }

    #[test]
    fn a_one_hot_mux_passes_the_selected_input() {
        // Two selects, two data bits, with the illegal all-zero and all-one
        // select patterns included: a one-hot mux built this way reads 0 and
        // the OR of its inputs respectively, and that is a defined answer
        // rather than an undefined one.
        check(
            4,
            |b| {
                let sel = vec!["i0".to_string(), "i1".to_string()];
                let data = vec!["i2".to_string(), "i3".to_string()];
                vec![mux_onehot(b, "t", &sel, &data, 2)]
            },
            &|bits| vec![(bits[0] && bits[2]) || (bits[1] && bits[3])],
        );
    }

    #[test]
    fn a_probe_module_measures_as_a_design() {
        let mut b = Builder::new("probe");
        b.input("a", 1);
        b.input("c", 1);
        b.output("y", 1);
        let value = xor2n(&mut b, "t", "a", "c");
        b.drive(Gate::Buf, "y", &[value]);
        let design = Design {
            top: "probe".into(),
            header: Vec::new(),
            modules: vec![b.finish()],
        };
        let metrics = measure(&design);
        assert_eq!(metrics.gates, 5);
        assert_eq!(metrics.depth, 4);
    }
}
