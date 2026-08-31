//! Behavioural reference models for the scale circuits.
//!
//! These are the suite's independent statement of what each circuit computes.
//! They implement the *specification* — `u32` multiplication for the array
//! multiplier, wrapping addition and the textbook overflow identity for the
//! ALUs, the Hamming construction for the ECC circuits, a priority scan for
//! the interrupt controller — and they share no code at all with the
//! generator. Nothing here knows what a gate is, and nothing here was written
//! by running the circuit and recording what came out.
//!
//! Two conventions make that possible:
//!
//! * Port values are unsigned integers keyed by port name. A vector port is
//!   one entry; a scalar-ported circuit's `a0 .. a31` are gathered into one
//!   integer by [`word`] and written back out by [`put`], with index `i`
//!   weighing `2^i`.
//! * A clocked circuit is a fold over its whole vector sequence, because that
//!   is what a register makes it: [`evaluate_sequence`] returns one output map
//!   per vector, in order.

use super::circuits::hamming_data_positions;
use super::sequential::LFSR32_TAPS;
use std::collections::BTreeMap;

/// Port values for one vector, keyed by port name.
pub type Values = BTreeMap<String, u64>;

/// Gather `stem0 .. stem{width-1}` into one integer, index `i` at weight
/// `2^i`. Falls back to the vector port `stem` when the scalars are absent.
pub fn word(values: &Values, stem: &str, width: u32) -> u64 {
    if let Some(value) = values.get(stem) {
        return *value;
    }
    (0..width)
        .map(|bit| {
            let name = format!("{stem}{bit}");
            let value = values
                .get(&name)
                .unwrap_or_else(|| panic!("no value for port '{name}'"));
            (*value & 1) << bit
        })
        .fold(0, |acc, bit| acc | bit)
}

/// Write `value` as `stem0 .. stem{width-1}`.
pub fn put(values: &mut Values, stem: &str, width: u32, value: u64) {
    for bit in 0..width {
        values.insert(format!("{stem}{bit}"), value >> bit & 1);
    }
}

fn mask(width: u32) -> u64 {
    if width >= 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    }
}

fn parity(value: u64) -> u64 {
    u64::from(value.count_ones() % 2)
}

/// Reference outputs for one vector of a combinational scale circuit.
///
/// Panics on a name the suite does not contain, so a typo in a caller is a
/// failure rather than an empty expectation that agrees with everything.
pub fn evaluate(name: &str, inputs: &Values) -> Values {
    match name {
        "intc27" => intc27(inputs),
        "sec32" | "sec32n" => sec32(inputs),
        "secded16" => secded16(inputs),
        "alu8" => alu8(inputs),
        "alu12c" => alu12c(inputs),
        "alu8bcd" => alu8bcd(inputs),
        "alu9d" => alu9d(inputs),
        "mul16" => mul16(inputs),
        "addcmp32" => addcmp32(inputs),
        other => panic!("'{other}' has no combinational reference model"),
    }
}

/// Reference outputs for a clocked scale circuit, one map per vector.
pub fn evaluate_sequence(name: &str, vectors: &[Values]) -> Vec<Values> {
    match name {
        "lfsr32" => lfsr32(vectors),
        "pipe_mac8" => pipe_mac8(vectors),
        other => panic!("'{other}' has no sequential reference model"),
    }
}

// ===========================================================================
// Combinational models
// ===========================================================================

/// Mask a request word, take the highest surviving channel, and say whether
/// anything survived at all.
fn intc27(inputs: &Values) -> Values {
    let armed = inputs["req"] & inputs["msk"];
    let enable = inputs["gen"] & 1;
    let live = if enable == 1 { armed } else { 0 };
    let id = if live == 0 {
        0
    } else {
        u64::from(63 - live.leading_zeros())
    };
    let mut out = Values::new();
    out.insert("id".into(), id);
    out.insert("vld".into(), u64::from(live != 0));
    out.insert("any".into(), u64::from(armed != 0));
    out
}

/// The Hamming decoder, from the construction rather than from a netlist: the
/// syndrome is the received check word XOR the check word the data implies,
/// and a non-zero syndrome is read as the codeword position that flipped.
fn sec32(inputs: &Values) -> Values {
    const PARITY: u32 = 6;
    let positions = hamming_data_positions(PARITY, 32);
    let data = inputs["d"];
    let check = inputs["c"];

    let mut syndrome = 0u64;
    for k in 0..PARITY {
        let mut bit = check >> k & 1;
        for (index, position) in positions.iter().enumerate() {
            if position >> k & 1 == 1 {
                bit ^= data >> index & 1;
            }
        }
        syndrome |= bit << k;
    }

    let mut corrected = data;
    for (index, position) in positions.iter().enumerate() {
        if syndrome == u64::from(*position) {
            corrected ^= 1 << index;
        }
    }

    let mut out = Values::new();
    out.insert("q".into(), corrected);
    out.insert("syn".into(), syndrome);
    out.insert("err".into(), u64::from(syndrome != 0));
    out
}

/// The extended-Hamming decoder. The overall parity bit is what separates a
/// single error from a double one: a syndrome naming a position is acted on
/// only when the overall parity also disagrees.
fn secded16(inputs: &Values) -> Values {
    const PARITY: u32 = 5;
    let positions = hamming_data_positions(PARITY, 16);
    let data = inputs["d"];
    let check = inputs["c"];
    let overall = inputs["p"] & 1;

    let mut syndrome = 0u64;
    for m in 0..PARITY {
        let mut bit = check >> m & 1;
        for (index, position) in positions.iter().enumerate() {
            if position >> m & 1 == 1 {
                bit ^= data >> index & 1;
            }
        }
        syndrome |= bit << m;
    }
    let mismatch = overall ^ parity(data) ^ parity(check);
    let single = syndrome != 0 && mismatch == 1;
    let double = syndrome != 0 && mismatch == 0;

    let mut corrected = data;
    let mut check_out = check;
    if single {
        for (index, position) in positions.iter().enumerate() {
            if syndrome == u64::from(*position) {
                corrected ^= 1 << index;
            }
        }
        for m in 0..PARITY {
            if syndrome == 1 << m {
                check_out ^= 1 << m;
            }
        }
    }

    let mut reencoded = 0u64;
    for m in 0..PARITY {
        let mut bit = 0u64;
        for (index, position) in positions.iter().enumerate() {
            if position >> m & 1 == 1 {
                bit ^= corrected >> index & 1;
            }
        }
        reencoded |= bit << m;
    }

    let mut out = Values::new();
    out.insert("q".into(), corrected);
    out.insert("kc".into(), check_out);
    out.insert("re".into(), reencoded);
    out.insert("syn".into(), syndrome);
    out.insert("sec".into(), u64::from(single));
    out.insert("ded".into(), u64::from(double));
    out
}

/// The result of one arithmetic-and-logic evaluation.
struct AluAnswer {
    y: u64,
    cout: u64,
    ovf: u64,
    zero: u64,
    neg: u64,
    par: u64,
}

/// The eight-function ALU of [`super::circuits::alu8`].
///
/// The carry chain runs on every function, so `cout` and `ovf` describe
/// `a + (b ^ sub) + cin` whatever `f` selects. Overflow is the standard signed
/// identity — the sum's sign differs from both operands' — rather than
/// anything read off a carry chain.
fn alu8_core(a: u64, b: u64, f: u64, cin: u64) -> AluAnswer {
    const WIDTH: u32 = 8;
    let full = mask(WIDTH);
    let operand = if f == 1 { !b & full } else { b };
    let total = a + operand + cin;
    let sum = total & full;
    let cout = total >> WIDTH & 1;
    let ovf = ((a ^ sum) & (operand ^ sum)) >> (WIDTH - 1) & 1;
    let y = match f {
        0 | 1 => sum,
        2 => a & b,
        3 => a | b,
        4 => a ^ b,
        5 => !(a ^ b) & full,
        6 => a,
        7 => b,
        other => panic!("alu8 has no function {other}"),
    };
    AluAnswer {
        cout,
        ovf,
        zero: u64::from(y == 0),
        neg: y >> (WIDTH - 1) & 1,
        par: parity(y),
        y,
    }
}

fn alu8(inputs: &Values) -> Values {
    let answer = alu8_core(inputs["a"], inputs["b"], inputs["f"], inputs["cin"] & 1);
    let mut out = Values::new();
    out.insert("y".into(), answer.y);
    out.insert("cout".into(), answer.cout);
    out.insert("ovf".into(), answer.ovf);
    out.insert("zero".into(), answer.zero);
    out
}

/// The sixteen-function core the wider ALUs share.
///
/// * `0..3` run through the adder with `b`, `~b`, zero and all-ones
/// * `4..9` are the six two-operand logic functions
/// * `10..13` pass or invert one operand
/// * `14, 15` shift `a` left or right by `b & 7`, filling with zero
fn alu16_core(a: u64, b: u64, op: u64, cy: u64, width: u32) -> AluAnswer {
    let full = mask(width);
    let operand = match op {
        0 => b,
        1 => !b & full,
        2 => 0,
        3 => full,
        _ => 0,
    };
    let total = a + operand + cy;
    let sum = total & full;
    let cout = total >> width & 1;
    let ovf = ((a ^ sum) & (operand ^ sum)) >> (width - 1) & 1;
    let amount = b & 7;
    let y = match op {
        0..=3 => sum,
        4 => a & b,
        5 => a | b,
        6 => a ^ b,
        7 => !(a ^ b) & full,
        8 => !(a & b) & full,
        9 => !(a | b) & full,
        10 => a,
        11 => b,
        12 => !a & full,
        13 => !b & full,
        14 => a << amount & full,
        15 => a >> amount,
        other => panic!("the ALU core has no function {other}"),
    };
    AluAnswer {
        cout,
        ovf,
        zero: u64::from(y == 0),
        neg: y >> (width - 1) & 1,
        par: parity(y),
        y,
    }
}

/// The 12-bit datapath and its control decoder.
///
/// `mode[0]` forces the carry in high; `mode[1]` disables the outputs, and a
/// disabled output is zero — including `zero` itself, which is an AND of the
/// core's zero detect with the enable and therefore reads low when the outputs
/// are off.
fn alu12c(inputs: &Values) -> Values {
    const WIDTH: u32 = 12;
    let mode = inputs["mode"];
    let enable = u64::from(mode >> 1 & 1 == 0);
    let carry = (inputs["cin"] & 1) | (mode & 1);
    let answer = alu16_core(inputs["a"], inputs["b"], inputs["op"], carry, WIDTH);

    let mut out = Values::new();
    for (name, value) in [
        ("y", answer.y),
        ("cout", answer.cout),
        ("ovf", answer.ovf),
        ("zero", answer.zero),
        ("neg", answer.neg),
        ("par", answer.par),
    ] {
        out.insert(name.into(), if enable == 1 { value } else { 0 });
    }
    out
}

/// The BCD half of [`super::circuits_wide::alu8bcd`].
///
/// The decimal adjust, stated as the specification states it, and defined for
/// every four-bit input rather than only for decimal digits:
///
/// * the second operand per digit is `b` for add, and `((~b) + 10) mod 16` for
///   subtract, which is `9 - b` for any digit that is a decimal digit
/// * a four-bit binary add produces `t` and a carry `c`
/// * `gt9 = c | (t[3] & (t[2] | t[1]))`
/// * the digit result is `(t + 6) mod 16` when `gt9`, and `t` otherwise
/// * `gt9` carries into the next digit
///
/// Returns `(result, carry out, auxiliary carry)`.
fn bcd_add(a: u64, b: u64, subtract: bool, cin: u64) -> (u64, u64, u64) {
    let mut carry = cin;
    let mut result = 0u64;
    let mut aux = 0u64;
    for digit in 0..2u32 {
        let left = a >> (4 * digit) & 0xF;
        let right = b >> (4 * digit) & 0xF;
        let operand = if subtract {
            (!right & 0xF).wrapping_add(10) & 0xF
        } else {
            right
        };
        let total = left + operand + carry;
        let raw = total & 0xF;
        let carry4 = total >> 4 & 1;
        let above = carry4 | (raw >> 3 & 1) & ((raw >> 2 & 1) | (raw >> 1 & 1));
        let adjusted = if above == 1 { (raw + 6) & 0xF } else { raw };
        result |= adjusted << (4 * digit);
        if digit == 0 {
            aux = above;
        }
        carry = above;
    }
    (result, carry, aux)
}

fn alu8bcd(inputs: &Values) -> Values {
    const WIDTH: u32 = 8;
    let a = inputs["a"];
    let b = inputs["b"];
    let f = inputs["f"];
    let cin = inputs["cin"] & 1;
    let decimal = inputs["dec"] & 1;

    let binary = alu16_core(a, b, f, cin, WIDTH);
    let (decimal_sum, decimal_cout, aux) = bcd_add(a, b, f == 1, cin);
    let use_decimal = decimal == 1 && (f == 0 || f == 1);

    let y = if use_decimal { decimal_sum } else { binary.y };
    let cout = if use_decimal {
        decimal_cout
    } else {
        binary.cout
    };
    // A digit is invalid when it is above nine, whichever operand it is in and
    // whichever mode the unit is running.
    let invalid = [a, b]
        .iter()
        .flat_map(|value| (0..2u32).map(move |digit| value >> (4 * digit) & 0xF))
        .any(|digit| digit > 9);

    let mut out = Values::new();
    out.insert("y".into(), y);
    out.insert("cout".into(), cout);
    out.insert("ovf".into(), binary.ovf);
    out.insert("zero".into(), u64::from(y == 0));
    out.insert("ac".into(), aux);
    out.insert("inval".into(), u64::from(invalid));
    out.insert("pa".into(), parity(a));
    out.insert("pb".into(), parity(b));
    out.insert("py".into(), parity(y));
    out
}

fn alu9d(inputs: &Values) -> Values {
    const WIDTH: u32 = 9;
    let mut out = Values::new();
    let mut results = Vec::with_capacity(2);
    for lane in 0..2usize {
        let answer = alu16_core(
            inputs[&format!("a{lane}")],
            inputs[&format!("b{lane}")],
            inputs["op"],
            inputs[&format!("cin{lane}")] & 1,
            WIDTH,
        );
        // `f{lane}` packs the slice's flags: carry at bit 0, overflow at bit
        // 1, zero at bit 2, which is the order the slice's ports are joined.
        out.insert(format!("y{lane}"), answer.y);
        out.insert(
            format!("f{lane}"),
            answer.cout | answer.ovf << 1 | answer.zero << 2,
        );
        results.push(answer.y);
    }
    out.insert("eq".into(), u64::from(results[0] == results[1]));
    out.insert("lt".into(), u64::from(results[0] < results[1]));
    out.insert("gt".into(), u64::from(results[0] > results[1]));
    out
}

fn mul16(inputs: &Values) -> Values {
    let mut out = Values::new();
    out.insert("p".into(), inputs["a"] * inputs["b"]);
    out
}

/// The widest circuit, from its architecture rather than its gates.
///
/// The carry-save stage reduces three operands to a sum and a carry vector,
/// and the prefix adder then computes `cs + (cc << 1) + cin` in 32 bits. That
/// distinction matters for the flags and only for the flags: the sum is
/// `a + (b ^ sub) + m + cin` either way, which this model asserts, but `cout`
/// and `ovf` belong to the two-operand addition the adder actually performs,
/// and the carry the shift pushes off the top is not in it.
fn addcmp32(inputs: &Values) -> Values {
    const WIDTH: u32 = 32;
    let full = mask(WIDTH);
    let a = word(inputs, "a", WIDTH);
    let b = word(inputs, "b", WIDTH);
    let m = word(inputs, "m", WIDTH);
    let subtract = word(inputs, "sub", 1) & 1;
    let cin = word(inputs, "cin", 1) & 1;
    let enable = word(inputs, "oe", 1) & 1;

    let operand = if subtract == 1 { !b & full } else { b };
    let carry_save_sum = a ^ operand ^ m;
    let carry_save_carry = (a & operand) | (a & m) | (operand & m);

    let shifted = carry_save_carry << 1 & full;
    let total = carry_save_sum + shifted + cin;
    let sum = total & full;
    let cout = total >> WIDTH & 1;
    let low = (carry_save_sum & full >> 1) + (shifted & full >> 1) + cin;
    let carry_into_top = low >> (WIDTH - 1) & 1;
    let ovf = cout ^ carry_into_top;
    assert_eq!(
        sum,
        (a + operand + m + cin) & full,
        "the carry-save stage must not change the sum"
    );

    let mut out = Values::new();
    let gate = |value: u64| if enable == 1 { value } else { 0 };
    put(&mut out, "s", WIDTH, gate(sum));
    put(&mut out, "cs", WIDTH, gate(carry_save_sum));
    put(&mut out, "cc", WIDTH, gate(carry_save_carry));
    for (name, value) in [
        ("cout", cout),
        ("ovf", ovf),
        ("eq", u64::from(a == b)),
        ("lt", u64::from(a < b)),
        ("gt", u64::from(a > b)),
        ("zero", u64::from(sum == 0)),
        ("pa", parity(a)),
        ("pb", parity(b)),
        ("pm", parity(m)),
        ("ps", parity(sum)),
    ] {
        out.insert(name.into(), gate(value));
    }
    out
}

// ===========================================================================
// Sequential models
// ===========================================================================

/// The linear feedback shift register's recurrence.
///
/// One clock per vector; the observation is taken after the edge, so the first
/// row already shows the state the first edge wrote. `fb` is a continuous
/// assign, so it is the feedback the *next* edge will shift in, computed from
/// the state on show.
fn lfsr32(vectors: &[Values]) -> Vec<Values> {
    let full = mask(32);
    let mut state = 0u64;
    let mut rows = Vec::with_capacity(vectors.len());
    for vector in vectors {
        let feedback = LFSR32_TAPS
            .iter()
            .fold(0u64, |acc, tap| acc ^ (state >> tap & 1));
        state = if vector["rst"] & 1 == 1 {
            1
        } else if vector["ld"] & 1 == 1 {
            vector["seed"]
        } else {
            (state << 1 | feedback) & full
        };
        let next_feedback = LFSR32_TAPS
            .iter()
            .fold(0u64, |acc, tap| acc ^ (state >> tap & 1));
        let mut out = Values::new();
        out.insert("q".into(), state);
        out.insert("fb".into(), next_feedback);
        out.insert("msb".into(), state >> 31 & 1);
        rows.push(out);
    }
    rows
}

/// The pipeline, folded one edge at a time.
///
/// Every write is non-blocking, so each stage reads what the stage before it
/// held *entering* the edge. That is why every right-hand side below reads the
/// pre-edge state and every left-hand side writes the post-edge one.
fn pipe_mac8(vectors: &[Values]) -> Vec<Values> {
    let mut a1 = 0u64;
    let mut b1 = 0u64;
    let mut v1 = 0u64;
    let mut v2 = 0u64;
    let mut prod = 0u64;
    let mut acc = 0u64;
    let mut valid;
    let mut rows = Vec::with_capacity(vectors.len());
    for vector in vectors {
        if vector["rst"] & 1 == 1 {
            a1 = 0;
            b1 = 0;
            v1 = 0;
            v2 = 0;
            prod = 0;
            acc = 0;
            valid = 0;
        } else {
            let next_prod = a1 * b1;
            let next_acc = if vector["clr"] & 1 == 1 {
                0
            } else if v2 == 1 {
                (acc + prod) & mask(24)
            } else {
                acc
            };
            a1 = vector["a"];
            b1 = vector["b"];
            valid = v2;
            v2 = v1;
            v1 = vector["en"] & 1;
            prod = next_prod;
            acc = next_acc;
        }
        let mut out = Values::new();
        out.insert("prod".into(), prod);
        out.insert("acc".into(), acc);
        out.insert("vld".into(), valid);
        rows.push(out);
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(pairs: &[(&str, u64)]) -> Values {
        pairs
            .iter()
            .map(|(name, value)| ((*name).to_string(), *value))
            .collect()
    }

    #[test]
    fn the_controller_reports_the_highest_unmasked_channel() {
        let out = evaluate(
            "intc27",
            &values(&[("req", 0b1010), ("msk", 0x7FF_FFFF), ("gen", 1)]),
        );
        assert_eq!(out["id"], 3);
        assert_eq!(out["vld"], 1);
        assert_eq!(out["any"], 1);

        // The mask can hide the winner, and the global enable can hide
        // everything but the pending flag.
        let out = evaluate(
            "intc27",
            &values(&[("req", 0b1010), ("msk", 0b10), ("gen", 1)]),
        );
        assert_eq!(out["id"], 1);
        let out = evaluate(
            "intc27",
            &values(&[("req", 0b1010), ("msk", 0x7FF_FFFF), ("gen", 0)]),
        );
        assert_eq!((out["id"], out["vld"], out["any"]), (0, 0, 1));
    }

    #[test]
    fn the_hamming_decoder_corrects_the_bit_its_syndrome_names() {
        // A clean codeword: build the check word the data implies, and the
        // syndrome must vanish.
        let positions = hamming_data_positions(6, 32);
        let data = 0x89AB_CDEFu64;
        let mut check = 0u64;
        for k in 0..6u32 {
            let mut bit = 0u64;
            for (index, position) in positions.iter().enumerate() {
                if position >> k & 1 == 1 {
                    bit ^= data >> index & 1;
                }
            }
            check |= bit << k;
        }
        let clean = evaluate("sec32", &values(&[("d", data), ("c", check)]));
        assert_eq!(clean["syn"], 0);
        assert_eq!(clean["err"], 0);
        assert_eq!(clean["q"], data);

        // Flip each data bit in turn; each one must come back.
        for index in 0..32 {
            let out = evaluate("sec32", &values(&[("d", data ^ 1 << index), ("c", check)]));
            assert_eq!(out["q"], data, "data bit {index}");
            assert_eq!(out["syn"], u64::from(positions[index]));
            assert_eq!(out["err"], 1);
        }
        // Flip each check bit; the data must be left alone.
        for k in 0..6 {
            let out = evaluate("sec32", &values(&[("d", data), ("c", check ^ 1 << k)]));
            assert_eq!(out["q"], data, "check bit {k}");
            assert_eq!(out["syn"], 1 << k);
        }
    }

    #[test]
    fn the_secded_decoder_separates_one_error_from_two() {
        let positions = hamming_data_positions(5, 16);
        let data = 0xBEEFu64;
        let mut check = 0u64;
        for m in 0..5u32 {
            let mut bit = 0u64;
            for (index, position) in positions.iter().enumerate() {
                if position >> m & 1 == 1 {
                    bit ^= data >> index & 1;
                }
            }
            check |= bit << m;
        }
        let overall = parity(data) ^ parity(check);
        let clean = evaluate(
            "secded16",
            &values(&[("d", data), ("c", check), ("p", overall)]),
        );
        assert_eq!((clean["sec"], clean["ded"], clean["q"]), (0, 0, data));
        assert_eq!(clean["re"], check);

        // One flipped data bit. The transmitted overall parity is unchanged —
        // that is the point of it — so the receiver sees it disagree.
        let single = evaluate(
            "secded16",
            &values(&[("d", data ^ 1), ("c", check), ("p", overall)]),
        );
        assert_eq!((single["sec"], single["ded"], single["q"]), (1, 0, data));

        let double = evaluate(
            "secded16",
            &values(&[("d", data ^ 0b11), ("c", check), ("p", overall)]),
        );
        assert_eq!((double["sec"], double["ded"]), (0, 1));
        assert_eq!(double["q"], data ^ 0b11, "a double error is not corrected");
    }

    #[test]
    fn the_eight_function_alu_computes_its_table() {
        let out = evaluate(
            "alu8",
            &values(&[("a", 0xF0), ("b", 0x0F), ("f", 0), ("cin", 0)]),
        );
        assert_eq!((out["y"], out["cout"], out["zero"]), (0xFF, 0, 0));

        // 0x80 - 0x01 in two's complement: subtract is `a + ~b + 1`.
        let out = evaluate(
            "alu8",
            &values(&[("a", 0x80), ("b", 0x01), ("f", 1), ("cin", 1)]),
        );
        assert_eq!(out["y"], 0x7F);
        assert_eq!(out["cout"], 1);
        assert_eq!(out["ovf"], 1, "0x80 - 1 overflows a signed byte");

        for (f, expected) in [(2u64, 0x0Cu64), (3, 0x3E), (4, 0x32), (5, 0xCD)] {
            let out = evaluate(
                "alu8",
                &values(&[("a", 0x1C), ("b", 0x2E), ("f", f), ("cin", 0)]),
            );
            assert_eq!(out["y"], expected, "function {f}");
        }
    }

    #[test]
    fn the_decimal_adjust_matches_the_published_rule() {
        // 27 + 45 = 72 in BCD.
        let out = evaluate(
            "alu8bcd",
            &values(&[("a", 0x27), ("b", 0x45), ("f", 0), ("cin", 0), ("dec", 1)]),
        );
        assert_eq!(out["y"], 0x72);
        assert_eq!(out["cout"], 0);
        assert_eq!(out["inval"], 0);

        // 88 + 33 = 121: the carry out is the hundreds digit.
        let out = evaluate(
            "alu8bcd",
            &values(&[("a", 0x88), ("b", 0x33), ("f", 0), ("cin", 0), ("dec", 1)]),
        );
        assert_eq!((out["y"], out["cout"], out["ac"]), (0x21, 1, 1));

        // 42 - 17 = 25, by nine's complement with a carry in.
        let out = evaluate(
            "alu8bcd",
            &values(&[("a", 0x42), ("b", 0x17), ("f", 1), ("cin", 1), ("dec", 1)]),
        );
        assert_eq!(out["y"], 0x25);

        // The same operands in binary mode are a binary subtract.
        let out = evaluate(
            "alu8bcd",
            &values(&[("a", 0x42), ("b", 0x17), ("f", 1), ("cin", 1), ("dec", 0)]),
        );
        assert_eq!(out["y"], 0x2B);

        // A digit above nine is reported whatever the mode says.
        let out = evaluate(
            "alu8bcd",
            &values(&[("a", 0x1A), ("b", 0x00), ("f", 0), ("cin", 0), ("dec", 0)]),
        );
        assert_eq!(out["inval"], 1);
    }

    #[test]
    fn the_wide_adder_agrees_with_itself_about_the_sum() {
        let mut inputs = Values::new();
        put(&mut inputs, "a", 32, 0x1234_5678);
        put(&mut inputs, "b", 32, 0x8765_4321);
        put(&mut inputs, "m", 32, 0x0F0F_0F0F);
        put(&mut inputs, "sub", 1, 0);
        put(&mut inputs, "cin", 1, 1);
        put(&mut inputs, "oe", 1, 1);
        let out = evaluate("addcmp32", &inputs);
        let expected = 0x1234_5678u64 + 0x8765_4321 + 0x0F0F_0F0F + 1;
        assert_eq!(word(&out, "s", 32), expected & 0xFFFF_FFFF);
        assert_eq!(out["cout"], 0);
        assert_eq!(out["lt"], 1);
        assert_eq!(out["gt"], 0);
        assert_eq!(out["eq"], 0);

        // Disabled, every output is zero.
        put(&mut inputs, "oe", 1, 0);
        let out = evaluate("addcmp32", &inputs);
        assert!(out.values().all(|value| *value == 0));
    }

    #[test]
    fn the_shift_register_walks_and_locks_up_on_zero() {
        let step =
            |rst: u64, ld: u64, seed: u64| values(&[("rst", rst), ("ld", ld), ("seed", seed)]);
        let rows = evaluate_sequence(
            "lfsr32",
            &[
                step(1, 0, 0),
                step(0, 0, 0),
                step(0, 0, 0),
                step(0, 1, 0),
                step(0, 0, 0),
            ],
        );
        // Reset loads one. Bit 0 is a tap, so the feedback into the next
        // shift is 1 and the state goes 1, 3, then 6 as the two set bits
        // cancel in the feedback.
        assert_eq!(rows[0]["q"], 1);
        assert_eq!(rows[0]["fb"], 1);
        assert_eq!(rows[1]["q"], 3);
        assert_eq!(rows[1]["fb"], 0);
        assert_eq!(rows[2]["q"], 6);
        assert_eq!(rows[3]["q"], 0, "the seed is loaded verbatim");
        assert_eq!(rows[4]["q"], 0, "zero is a fixed point");
    }

    #[test]
    fn the_pipeline_takes_three_clocks_to_reach_the_accumulator() {
        let step = |rst: u64, en: u64, clr: u64, a: u64, b: u64| {
            values(&[("rst", rst), ("en", en), ("clr", clr), ("a", a), ("b", b)])
        };
        let rows = evaluate_sequence(
            "pipe_mac8",
            &[
                step(1, 0, 0, 0, 0),
                step(0, 1, 0, 3, 5),
                step(0, 0, 0, 0, 0),
                step(0, 0, 0, 0, 0),
                step(0, 0, 0, 0, 0),
            ],
        );
        assert_eq!(rows[0]["acc"], 0);
        assert_eq!(rows[2]["prod"], 15, "the product lands on the second edge");
        assert_eq!(rows[2]["acc"], 0);
        assert_eq!(
            rows[3]["acc"], 15,
            "and reaches the accumulator on the third"
        );
        assert_eq!(rows[3]["vld"], 1);
        assert_eq!(rows[4]["acc"], 15);
    }
}
