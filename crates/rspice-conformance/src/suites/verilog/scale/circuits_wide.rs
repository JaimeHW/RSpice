//! The five larger circuits of the scale suite.
//!
//! Same rule as [`super::circuits`]: designed from the function each doc
//! comment states and from nothing else.

use super::cells::*;
use super::circuits::{bits, design, ports, provenance};
use super::netlist::{Builder, Design, Gate};

/// The sixteen functions of the shared ALU core, in select order.
///
/// The first four run through the adder with a different second operand each
/// (`b`, `~b`, zero, all-ones), which is how one carry chain serves add,
/// subtract, increment and decrement.
pub const ALU16_FUNCTIONS: [&str; 16] = [
    "add", "sub", "addz", "subz", "and", "or", "xor", "xnor", "nand", "nor", "pass_a", "pass_b",
    "not_a", "not_b", "shl", "shr",
];

/// Everything one ALU core produces.
pub struct AluCore {
    pub y: Vec<String>,
    pub cout: String,
    pub ovf: String,
    pub zero: String,
    pub neg: String,
    pub par: String,
}

/// A `width`-bit sixteen-function ALU, built into `n`.
///
/// `sel` is the one-hot function select, `cy` the carry in, and `lo`/`hi` the
/// constant nets the second-operand mux needs. The carry chain runs whatever
/// the function is, so `cout` and `ovf` always describe the arithmetic path.
///
/// The shifters are three-stage barrels driven by `b[2:0]`: `shl` is a
/// zero-filling left shift by that amount and `shr` the matching right shift.
/// Three stages cover a shift of zero to seven, which is the whole range for
/// the nine- and twelve-bit datapaths this core is used at.
pub fn alu_core(
    n: &mut Builder,
    p: &str,
    a: &[String],
    b: &[String],
    sel: &[String],
    cy: &str,
    lo: &str,
    hi: &str,
) -> AluCore {
    let width = a.len();
    assert_eq!(width, b.len(), "an ALU's operands must be the same width");
    assert_eq!(sel.len(), 16, "the core decodes sixteen functions");

    let not_b: Vec<String> = (0..width)
        .map(|i| inv(n, &format!("{p}_nb"), &b[i]))
        .collect();
    let not_a: Vec<String> = (0..width)
        .map(|i| inv(n, &format!("{p}_na"), &a[i]))
        .collect();

    // Second operand: `b`, `~b`, zero or all-ones, by the four arithmetic
    // selects. Any other function leaves it at zero, which is harmless
    // because the adder's output is not selected then.
    let arith_sel = sel[0..4].to_vec();
    let beff: Vec<String> = (0..width)
        .map(|i| {
            let sources = vec![b[i].clone(), not_b[i].clone(), lo.to_string(), hi.to_string()];
            mux_onehot(n, &format!("{p}_bs"), &arith_sel, &sources, 2)
        })
        .collect();
    let (sum, carries) = ripple_add(n, &format!("{p}_add"), a, &beff, cy);

    let and_unit: Vec<String> = (0..width)
        .map(|i| and2(n, &format!("{p}_and"), &a[i], &b[i]))
        .collect();
    let or_unit: Vec<String> = (0..width)
        .map(|i| or2(n, &format!("{p}_or"), &a[i], &b[i]))
        .collect();
    let xor_unit: Vec<String> = (0..width)
        .map(|i| xor2n(n, &format!("{p}_xor"), &a[i], &b[i]))
        .collect();
    let xnor_unit: Vec<String> = (0..width)
        .map(|i| xnor2n(n, &format!("{p}_xnor"), &a[i], &b[i]))
        .collect();
    let nand_unit: Vec<String> = (0..width)
        .map(|i| nand2(n, &format!("{p}_nand"), &a[i], &b[i]))
        .collect();
    let nor_unit: Vec<String> = (0..width)
        .map(|i| n.two(Gate::Nor, &format!("{p}_nor"), &a[i], &b[i]))
        .collect();

    let shl = barrel(n, &format!("{p}_shl"), a, b, lo, true);
    let shr = barrel(n, &format!("{p}_shr"), a, b, lo, false);

    let mut y = Vec::with_capacity(width);
    for i in 0..width {
        let sources = vec![
            sum[i].clone(),
            sum[i].clone(),
            sum[i].clone(),
            sum[i].clone(),
            and_unit[i].clone(),
            or_unit[i].clone(),
            xor_unit[i].clone(),
            xnor_unit[i].clone(),
            nand_unit[i].clone(),
            nor_unit[i].clone(),
            a[i].clone(),
            b[i].clone(),
            not_a[i].clone(),
            not_b[i].clone(),
            shl[i].clone(),
            shr[i].clone(),
        ];
        y.push(mux_onehot(n, &format!("{p}_mux"), sel, &sources, 4));
    }

    let any = or_tree(n, &format!("{p}_z"), &y);
    let zero = inv(n, &format!("{p}_z"), &any);
    let ovf = xor2n(
        n,
        &format!("{p}_ovf"),
        &carries[width - 1],
        &carries[width - 2],
    );
    let par = xor_tree_n(n, &format!("{p}_par"), &y);
    AluCore {
        cout: carries[width - 1].clone(),
        neg: y[width - 1].clone(),
        y,
        ovf,
        zero,
        par,
    }
}

/// A three-stage barrel shifter driven by `amount[2:0]`, zero filling.
fn barrel(
    n: &mut Builder,
    p: &str,
    data: &[String],
    amount: &[String],
    lo: &str,
    left: bool,
) -> Vec<String> {
    let width = data.len();
    let mut current = data.to_vec();
    for stage in 0..3usize {
        let distance = 1usize << stage;
        let select = &amount[stage];
        let clear = inv(n, p, select);
        current = (0..width)
            .map(|i| {
                let shifted = if left {
                    i.checked_sub(distance)
                        .map(|source| current[source].clone())
                        .unwrap_or_else(|| lo.to_string())
                } else {
                    current
                        .get(i + distance)
                        .cloned()
                        .unwrap_or_else(|| lo.to_string())
                };
                mux2(n, p, select, &clear, &current[i], &shifted)
            })
            .collect();
    }
    current
}

/// One-hot decode of a four-bit opcode, fan-in 2.
fn decode16(n: &mut Builder, p: &str, op: &[String]) -> Vec<String> {
    let complement: Vec<String> = (0..4).map(|k| inv(n, p, &op[k])).collect();
    (0..16)
        .map(|code| {
            let literal = |k: usize| {
                if code >> k & 1 == 1 {
                    op[k].clone()
                } else {
                    complement[k].clone()
                }
            };
            let low = and2(n, p, &literal(0), &literal(1));
            let high = and2(n, p, &literal(2), &literal(3));
            and2(n, p, &low, &high)
        })
        .collect()
}

fn scalar_names(stem: &str, count: usize) -> Vec<String> {
    (0..count).map(|index| format!("{stem}{index}")).collect()
}

// ===========================================================================
// alu12c
// ===========================================================================

/// A 12-bit ALU datapath under a separate control decoder.
///
/// Three modules: `alu12c_ctl` turns a four-bit opcode and two mode bits into
/// a one-hot function select, an output enable and a forced carry;
/// `alu12c_dp` is the datapath; `alu12c` wires them together and splits its
/// vector ports into the scalar nets an instance port can be connected to.
///
/// The control block ends in **three-stage buffer chains**, one per select
/// line, which is where a mapper puts the drive for a signal that crosses into
/// a datapath and fans out across every bit slice. Inside the datapath the
/// fanout pass then builds a second tier of buffers on the same signals.
///
/// * `mode[0]` forces the carry in high, which turns `addz` into an increment
/// * `mode[1]` disables the outputs, which drives every one of them to zero
pub fn alu12c() -> Design {
    const WIDTH: usize = 12;

    // --- control ---------------------------------------------------------
    let mut ctl = Builder::new("alu12c_ctl");
    ctl.doc("Control decoder for alu12c: a four-bit opcode and two mode bits");
    ctl.doc("into a one-hot function select, an output enable and a forced");
    ctl.doc("carry, each buffered for the datapath it drives.");
    let op_names = scalar_names("op", 4);
    for name in &op_names {
        ctl.input(name, 1);
    }
    ctl.input("m0", 1);
    ctl.input("m1", 1);
    let sel_names = scalar_names("s", 16);
    for name in &sel_names {
        ctl.output(name, 1);
    }
    ctl.output("oe", 1);
    ctl.output("cyf", 1);
    let decoded = decode16(&mut ctl, "dec", &op_names);
    for (index, net) in decoded.iter().enumerate() {
        // Three stages of drive, then the port.
        let driven = ctl.buffer_chain("drv", net, 2);
        ctl.drive(Gate::Buf, &sel_names[index], &[driven]);
    }
    let enable = ctl.not("oe", "m1");
    let enable = ctl.buffer_chain("oed", &enable, 2);
    ctl.drive(Gate::Buf, "oe", &[enable]);
    let forced = ctl.buffer_chain("cyd", "m0", 2);
    ctl.drive(Gate::Buf, "cyf", &[forced]);

    // --- datapath --------------------------------------------------------
    let mut dp = Builder::new("alu12c_dp");
    dp.doc("The alu12c datapath: sixteen functions over twelve bits, with the");
    dp.doc("carry chain running on every one of them.");
    let a_names = scalar_names("a", WIDTH);
    let b_names = scalar_names("b", WIDTH);
    for name in a_names.iter().chain(&b_names) {
        dp.input(name, 1);
    }
    dp.input("cin", 1);
    for name in &sel_names {
        dp.input(name, 1);
    }
    dp.input("oe", 1);
    dp.input("cyf", 1);
    let y_names = scalar_names("y", WIDTH);
    for name in &y_names {
        dp.output(name, 1);
    }
    for name in ["cout", "ovf", "zero", "neg", "par"] {
        dp.output(name, 1);
    }
    let lo = dp.tie("lo", false);
    let hi = dp.tie("hi", true);
    let carry_in = or2(&mut dp, "cy", "cin", "cyf");
    let core = alu_core(
        &mut dp, "c", &a_names, &b_names, &sel_names, &carry_in, &lo, &hi,
    );
    for (index, net) in core.y.iter().enumerate() {
        dp.drive(Gate::And, &y_names[index], &[net.clone(), "oe".to_string()]);
    }
    for (port, net) in [
        ("cout", &core.cout),
        ("ovf", &core.ovf),
        ("zero", &core.zero),
        ("neg", &core.neg),
        ("par", &core.par),
    ] {
        dp.drive(Gate::And, port, &[net.clone(), "oe".to_string()]);
    }
    dp.limit_fanout(4, &[]);

    // --- top -------------------------------------------------------------
    let mut top = Builder::new("alu12c");
    top.doc("A 12-bit ALU datapath under a separate control decoder.");
    ports(
        &mut top,
        &[
            ("a", 12, true),
            ("b", 12, true),
            ("op", 4, true),
            ("mode", 2, true),
            ("cin", 1, true),
            ("y", 12, false),
            ("cout", 1, false),
            ("ovf", 1, false),
            ("zero", 1, false),
            ("neg", 1, false),
            ("par", 1, false),
        ],
    );
    let a_wires = top.split("a", 12);
    let b_wires = top.split("b", 12);
    let op_wires = top.split("op", 4);
    let mode_wires = top.split("mode", 2);
    let cin_wire = top.buffer("cinw", "cin");
    let sel_wires: Vec<String> = (0..16).map(|k| top.wire(&format!("sw{k}"))).collect();
    let oe_wire = top.wire("oew");
    let cyf_wire = top.wire("cyfw");
    let y_wires: Vec<String> = (0..WIDTH).map(|i| top.wire(&format!("yw{i}"))).collect();
    let flag_wires: Vec<String> = ["cout", "ovf", "zero", "neg", "par"]
        .iter()
        .map(|name| top.wire(&format!("{name}w")))
        .collect();

    let mut ctl_conns: Vec<(&str, String)> = Vec::new();
    for (index, name) in op_names.iter().enumerate() {
        ctl_conns.push((name.as_str(), op_wires[index].clone()));
    }
    ctl_conns.push(("m0", mode_wires[0].clone()));
    ctl_conns.push(("m1", mode_wires[1].clone()));
    for (index, name) in sel_names.iter().enumerate() {
        ctl_conns.push((name.as_str(), sel_wires[index].clone()));
    }
    ctl_conns.push(("oe", oe_wire.clone()));
    ctl_conns.push(("cyf", cyf_wire.clone()));
    top.instance("alu12c_ctl", "u_ctl", &ctl_conns);

    let mut dp_conns: Vec<(&str, String)> = Vec::new();
    for (index, name) in a_names.iter().enumerate() {
        dp_conns.push((name.as_str(), a_wires[index].clone()));
    }
    for (index, name) in b_names.iter().enumerate() {
        dp_conns.push((name.as_str(), b_wires[index].clone()));
    }
    dp_conns.push(("cin", cin_wire));
    for (index, name) in sel_names.iter().enumerate() {
        dp_conns.push((name.as_str(), sel_wires[index].clone()));
    }
    dp_conns.push(("oe", oe_wire));
    for (index, name) in y_names.iter().enumerate() {
        dp_conns.push((name.as_str(), y_wires[index].clone()));
    }
    for (index, name) in ["cout", "ovf", "zero", "neg", "par"].iter().enumerate() {
        dp_conns.push((name, flag_wires[index].clone()));
    }
    dp_conns.push(("cyf", cyf_wire));
    top.instance("alu12c_dp", "u_dp", &dp_conns);

    top.join("y", &y_wires);
    for (index, name) in ["cout", "ovf", "zero", "neg", "par"].iter().enumerate() {
        top.drive(Gate::Buf, name, &[flag_wires[index].clone()]);
    }

    design(
        "alu12c",
        &provenance("alu12c -- 12-bit ALU datapath under a separate control decoder."),
        vec![ctl.finish(), dp.finish(), top.finish()],
    )
}

// ===========================================================================
// alu8bcd
// ===========================================================================

/// An 8-bit ALU with a binary and a decimal arithmetic path.
///
/// The binary half is the shared sixteen-function core. Beside it runs a
/// two-digit BCD adder, and `dec` chooses which one the add and subtract
/// functions read.
///
/// The decimal path is the textbook decimal adjust, stated here because the
/// reference model has to agree with it on every input, valid BCD or not:
///
/// * the second operand per digit is `b` for add, and `((~b) + 10) mod 16` for
///   subtract, which is `9 - b` for any digit that is a decimal digit
/// * a four-bit binary add produces `t` and a carry `c`
/// * `gt9 = c | (t[3] & (t[2] | t[1]))`
/// * the digit result is `(t + 6) mod 16` when `gt9`, and `t` otherwise
/// * `gt9` is the carry into the next digit, and the low digit's is `ac`
///
/// `inval` reports that one of the four input digits is above nine, which is
/// the only thing that tells a caller the decimal answer is not decimal.
pub fn alu8bcd() -> Design {
    const WIDTH: usize = 8;
    let mut n = Builder::new("alu8bcd");
    n.doc("An 8-bit ALU with binary and BCD arithmetic paths and a mode");
    n.doc("select, plus parity over both operands and the result.");
    ports(
        &mut n,
        &[
            ("a", 8, true),
            ("b", 8, true),
            ("f", 4, true),
            ("cin", 1, true),
            ("dec", 1, true),
            ("y", 8, false),
            ("cout", 1, false),
            ("ovf", 1, false),
            ("zero", 1, false),
            ("ac", 1, false),
            ("inval", 1, false),
            ("pa", 1, false),
            ("pb", 1, false),
            ("py", 1, false),
        ],
    );
    let a = bits("a", 8);
    let b = bits("b", 8);
    let f = bits("f", 4);
    let lo = n.tie("lo", false);
    let hi = n.tie("hi", true);

    let sel = decode16(&mut n, "dec", &f);
    let core = alu_core(&mut n, "c", &a, &b, &sel, "cin", &lo, &hi);

    // --- decimal path ----------------------------------------------------
    let subtract = n.buffer("sb", &sel[1]);
    let nsubtract = n.not("sb", &subtract);
    let mut digit_carry = "cin".to_string();
    let mut bcd = Vec::with_capacity(WIDTH);
    let mut aux = String::new();
    for digit in 0..2usize {
        let base = digit * 4;
        let a_digit: Vec<String> = (0..4).map(|bit| a[base + bit].clone()).collect();
        let b_digit: Vec<String> = (0..4).map(|bit| b[base + bit].clone()).collect();

        // `(~b + 10) mod 16`, the nine's complement of a decimal digit.
        let inverted: Vec<String> = (0..4)
            .map(|bit| inv(&mut n, "nc", &b_digit[bit]))
            .collect();
        let ten = vec![lo.clone(), hi.clone(), lo.clone(), hi.clone()];
        let (nines, _) = ripple_add(&mut n, "nc", &inverted, &ten, &lo);

        let operand: Vec<String> = (0..4)
            .map(|bit| mux2(&mut n, "bs", &subtract, &nsubtract, &b_digit[bit], &nines[bit]))
            .collect();
        let (raw, raw_carry) = ripple_add(&mut n, "dadd", &a_digit, &operand, &digit_carry);

        let upper = or2(&mut n, "gt9", &raw[1], &raw[2]);
        let high = and2(&mut n, "gt9", &raw[3], &upper);
        let gt9 = or2(&mut n, "gt9", &raw_carry[3], &high);

        // Adding six is adding `{0, gt9, gt9, 0}`: the correction and the
        // decision to apply it are the same signal.
        let six = vec![lo.clone(), gt9.clone(), gt9.clone(), lo.clone()];
        let (adjusted, _) = ripple_add(&mut n, "dadj", &raw, &six, &lo);
        bcd.extend(adjusted);
        if digit == 0 {
            aux = gt9.clone();
        }
        digit_carry = gt9;
    }
    let bcd_cout = digit_carry;
    n.drive(Gate::Buf, "ac", &[aux]);

    // Digit validity: above nine is `d[3] & (d[2] | d[1])`.
    let mut invalid_terms = Vec::new();
    for operand in [&a, &b] {
        for digit in 0..2usize {
            let base = digit * 4;
            let upper = or2(&mut n, "iv", &operand[base + 1], &operand[base + 2]);
            invalid_terms.push(and2(&mut n, "iv", &operand[base + 3], &upper));
        }
    }
    let invalid = or_tree(&mut n, "iv", &invalid_terms);
    n.drive(Gate::Buf, "inval", &[invalid]);

    // --- mode select -----------------------------------------------------
    let arith = or2(&mut n, "ub", &sel[0], &sel[1]);
    let use_bcd = and2(&mut n, "ub", &arith, "dec");
    let keep_binary = n.not("ub", &use_bcd);
    let mut result = Vec::with_capacity(WIDTH);
    for i in 0..WIDTH {
        let value = mux2(
            &mut n,
            "ysel",
            &use_bcd,
            &keep_binary,
            &core.y[i],
            &bcd[i],
        );
        n.drive(Gate::Buf, &format!("y[{i}]"), &[value.clone()]);
        result.push(value);
    }
    let cout = mux2(
        &mut n,
        "cosel",
        &use_bcd,
        &keep_binary,
        &core.cout,
        &bcd_cout,
    );
    n.drive(Gate::Buf, "cout", &[cout]);
    n.drive(Gate::Buf, "ovf", &[core.ovf.clone()]);

    let any = or_tree(&mut n, "z", &result);
    n.drive(Gate::Not, "zero", &[any]);
    let pa = xor_tree_n(&mut n, "pa", &a);
    n.drive(Gate::Buf, "pa", &[pa]);
    let pb = xor_tree_n(&mut n, "pb", &b);
    n.drive(Gate::Buf, "pb", &[pb]);
    let py = xor_tree_n(&mut n, "py", &result);
    n.drive(Gate::Buf, "py", &[py]);

    n.limit_fanout(4, &[]);
    design(
        "alu8bcd",
        &provenance("alu8bcd -- 8-bit ALU with binary and decimal arithmetic paths."),
        vec![n.finish()],
    )
}

// ===========================================================================
// alu9d
// ===========================================================================

/// Two independent 9-bit ALU slices under one control decoder, with a
/// magnitude comparator across their results.
///
/// Four modules: the shared decoder, one slice instantiated twice, and a
/// comparator, under a top that splits its vector ports into scalars. The two
/// slices see the same function select and different operands, so the
/// comparator's answer depends on both halves of the circuit at once — a
/// defect in either slice shows up on `eq`, `lt` or `gt` as well as on that
/// slice's own result.
pub fn alu9d() -> Design {
    const WIDTH: usize = 9;

    let mut ctl = Builder::new("alu9d_ctl");
    ctl.doc("The shared control decoder: one four-bit opcode into the one-hot");
    ctl.doc("select both slices read, buffered for the fanout.");
    let op_names = scalar_names("op", 4);
    for name in &op_names {
        ctl.input(name, 1);
    }
    let sel_names = scalar_names("s", 16);
    for name in &sel_names {
        ctl.output(name, 1);
    }
    let decoded = decode16(&mut ctl, "dec", &op_names);
    for (index, net) in decoded.iter().enumerate() {
        let driven = ctl.buffer_chain("drv", net, 2);
        ctl.drive(Gate::Buf, &sel_names[index], &[driven]);
    }

    let mut slice = Builder::new("alu9d_slice");
    slice.doc("One 9-bit slice: the shared sixteen-function ALU core.");
    let a_names = scalar_names("a", WIDTH);
    let b_names = scalar_names("b", WIDTH);
    for name in a_names.iter().chain(&b_names) {
        slice.input(name, 1);
    }
    slice.input("cin", 1);
    for name in &sel_names {
        slice.input(name, 1);
    }
    let y_names = scalar_names("y", WIDTH);
    for name in &y_names {
        slice.output(name, 1);
    }
    for name in ["cout", "ovf", "zero"] {
        slice.output(name, 1);
    }
    let lo = slice.tie("lo", false);
    let hi = slice.tie("hi", true);
    let core = alu_core(
        &mut slice,
        "c",
        &a_names,
        &b_names,
        &sel_names,
        "cin",
        &lo,
        &hi,
    );
    for (index, net) in core.y.iter().enumerate() {
        slice.drive(Gate::Buf, &y_names[index], &[net.clone()]);
    }
    slice.drive(Gate::Buf, "cout", &[core.cout.clone()]);
    slice.drive(Gate::Buf, "ovf", &[core.ovf.clone()]);
    slice.drive(Gate::Buf, "zero", &[core.zero.clone()]);
    slice.limit_fanout(4, &[]);

    let mut cmp = Builder::new("alu9d_cmp");
    cmp.doc("An unsigned magnitude comparator across the two slice results.");
    let left_names = scalar_names("l", WIDTH);
    let right_names = scalar_names("r", WIDTH);
    for name in left_names.iter().chain(&right_names) {
        cmp.input(name, 1);
    }
    for name in ["eq", "lt", "gt"] {
        cmp.output(name, 1);
    }
    let same: Vec<String> = (0..WIDTH)
        .map(|i| xnor2n(&mut cmp, "eqb", &left_names[i], &right_names[i]))
        .collect();
    let equal = and_tree(&mut cmp, "eq", &same);
    cmp.drive(Gate::Buf, "eq", &[equal]);
    // From the top bit down: a position decides only when every higher
    // position agreed.
    let mut prefix = vec![String::new(); WIDTH];
    prefix[WIDTH - 1] = cmp.tie("one", true);
    for i in (0..WIDTH - 1).rev() {
        prefix[i] = and2(&mut cmp, "pfx", &prefix[i + 1], &same[i + 1]);
    }
    let mut greater = Vec::with_capacity(WIDTH);
    let mut lesser = Vec::with_capacity(WIDTH);
    for i in 0..WIDTH {
        let not_right = inv(&mut cmp, "nr", &right_names[i]);
        let not_left = inv(&mut cmp, "nl", &left_names[i]);
        let above = and2(&mut cmp, "gtb", &left_names[i], &not_right);
        let below = and2(&mut cmp, "ltb", &not_left, &right_names[i]);
        greater.push(and2(&mut cmp, "gtt", &prefix[i], &above));
        lesser.push(and2(&mut cmp, "ltt", &prefix[i], &below));
    }
    let gt = or_tree(&mut cmp, "gt", &greater);
    cmp.drive(Gate::Buf, "gt", &[gt]);
    let lt = or_tree(&mut cmp, "lt", &lesser);
    cmp.drive(Gate::Buf, "lt", &[lt]);
    cmp.limit_fanout(4, &[]);

    let mut top = Builder::new("alu9d");
    top.doc("Two independent 9-bit ALU slices under one control decoder, with");
    top.doc("a magnitude comparator across their results.");
    ports(
        &mut top,
        &[
            ("a0", 9, true),
            ("b0", 9, true),
            ("a1", 9, true),
            ("b1", 9, true),
            ("op", 4, true),
            ("cin0", 1, true),
            ("cin1", 1, true),
            ("y0", 9, false),
            ("y1", 9, false),
            ("f0", 3, false),
            ("f1", 3, false),
            ("eq", 1, false),
            ("lt", 1, false),
            ("gt", 1, false),
        ],
    );
    let sel_wires: Vec<String> = (0..16).map(|k| top.wire(&format!("sw{k}"))).collect();
    let mut ctl_conns: Vec<(&str, String)> = Vec::new();
    let op_wires = top.split("op", 4);
    for (index, name) in op_names.iter().enumerate() {
        ctl_conns.push((name.as_str(), op_wires[index].clone()));
    }
    for (index, name) in sel_names.iter().enumerate() {
        ctl_conns.push((name.as_str(), sel_wires[index].clone()));
    }
    top.instance("alu9d_ctl", "u_ctl", &ctl_conns);

    let mut results: Vec<Vec<String>> = Vec::new();
    for lane in 0..2usize {
        let a_wires = top.split(&format!("a{lane}"), 9);
        let b_wires = top.split(&format!("b{lane}"), 9);
        let cin_wire = top.buffer(&format!("cw{lane}"), &format!("cin{lane}"));
        let y_wires: Vec<String> = (0..WIDTH)
            .map(|i| top.wire(&format!("y{lane}w{i}")))
            .collect();
        let flags: Vec<String> = (0..3)
            .map(|i| top.wire(&format!("f{lane}w{i}")))
            .collect();
        let mut conns: Vec<(&str, String)> = Vec::new();
        for (index, name) in a_names.iter().enumerate() {
            conns.push((name.as_str(), a_wires[index].clone()));
        }
        for (index, name) in b_names.iter().enumerate() {
            conns.push((name.as_str(), b_wires[index].clone()));
        }
        conns.push(("cin", cin_wire));
        for (index, name) in sel_names.iter().enumerate() {
            conns.push((name.as_str(), sel_wires[index].clone()));
        }
        for (index, name) in y_names.iter().enumerate() {
            conns.push((name.as_str(), y_wires[index].clone()));
        }
        for (index, name) in ["cout", "ovf", "zero"].iter().enumerate() {
            conns.push((name, flags[index].clone()));
        }
        top.instance("alu9d_slice", &format!("u_slice{lane}"), &conns);
        top.join(&format!("y{lane}"), &y_wires);
        top.join(&format!("f{lane}"), &flags);
        results.push(y_wires);
    }

    let mut cmp_conns: Vec<(&str, String)> = Vec::new();
    for (index, name) in left_names.iter().enumerate() {
        cmp_conns.push((name.as_str(), results[0][index].clone()));
    }
    for (index, name) in right_names.iter().enumerate() {
        cmp_conns.push((name.as_str(), results[1][index].clone()));
    }
    let cmp_out: Vec<String> = ["eq", "lt", "gt"]
        .iter()
        .map(|name| top.wire(&format!("{name}w")))
        .collect();
    for (index, name) in ["eq", "lt", "gt"].iter().enumerate() {
        cmp_conns.push((name, cmp_out[index].clone()));
    }
    top.instance("alu9d_cmp", "u_cmp", &cmp_conns);
    for (index, name) in ["eq", "lt", "gt"].iter().enumerate() {
        top.drive(Gate::Buf, name, &[cmp_out[index].clone()]);
    }

    design(
        "alu9d",
        &provenance("alu9d -- two 9-bit ALU slices, shared control, result comparator."),
        vec![ctl.finish(), slice.finish(), cmp.finish(), top.finish()],
    )
}

// ===========================================================================
// mul16
// ===========================================================================

/// A 16 by 16 combinational array multiplier.
///
/// Two child cells — a half adder and a full adder, both NAND-mapped — and 240
/// instances of them. The array accumulates one partial product per row into a
/// running sum that is shifted right by one place each time, so row `i` is a
/// sixteen-bit ripple-carry adder and the bit that falls out of the bottom is
/// product bit `i`. After the last row the remaining seventeen bits are
/// product bits 15 through 31.
///
/// The rows chain end to end, which is what makes this the suite's depth
/// extreme: a carry raised in the first row's low bit can still be propagating
/// through the last row's high bit, and the longest path is hundreds of gate
/// levels for a circuit with only 32 inputs and 32 outputs.
pub fn mul16() -> Design {
    const WIDTH: usize = 16;

    // The two cells drive their ports from their last gate rather than
    // through a buffer. At 240 instances a buffer per port would be 480 gates
    // of nothing, and the array is where the suite's gate count is decided.
    let mut ha = Builder::new("mul16_ha");
    ha.doc("Half adder: four NAND-mapped gates sharing `~(a & b)` between the");
    ha.doc("sum and the carry.");
    for name in ["a", "b"] {
        ha.input(name, 1);
    }
    for name in ["s", "c"] {
        ha.output(name, 1);
    }
    let shared = nand2(&mut ha, "h", "a", "b");
    let left = nand2(&mut ha, "h", "a", &shared);
    let right = nand2(&mut ha, "h", "b", &shared);
    ha.drive(Gate::Nand, "s", &[left, right]);
    ha.drive(Gate::Not, "c", &[shared]);

    let mut fa = Builder::new("mul16_fa");
    fa.doc("Full adder: nine NAND gates, sharing `~(a & b)` and");
    fa.doc("`~((a ^ b) & ci)` between the sum and the carry.");
    for name in ["a", "b", "ci"] {
        fa.input(name, 1);
    }
    for name in ["s", "co"] {
        fa.output(name, 1);
    }
    let shared = nand2(&mut fa, "f", "a", "b");
    let left = nand2(&mut fa, "f", "a", &shared);
    let right = nand2(&mut fa, "f", "b", &shared);
    let half = nand2(&mut fa, "f", &left, &right);
    let carried = nand2(&mut fa, "f", &half, "ci");
    let low = nand2(&mut fa, "f", &half, &carried);
    let high = nand2(&mut fa, "f", "ci", &carried);
    fa.drive(Gate::Nand, "s", &[low, high]);
    fa.drive(Gate::Nand, "co", &[shared, carried]);

    let mut top = Builder::new("mul16");
    top.doc("A 16 by 16 combinational array multiplier: 256 partial products");
    top.doc("accumulated through 240 adder-cell instances.");
    ports(
        &mut top,
        &[("a", 16, true), ("b", 16, true), ("p", 32, false)],
    );
    let lo = top.tie("lo", false);

    // Partial products. `a[j]` and `b[i]` each drive sixteen of these, and the
    // fanout pass turns that into the input buffer trees an array multiplier
    // needs.
    let mut partial = Vec::with_capacity(WIDTH);
    for i in 0..WIDTH {
        let row: Vec<String> = (0..WIDTH)
            .map(|j| {
                let net = top.fresh(&format!("pp{i}"));
                top.drive(Gate::And, &net, &[format!("a[{j}]"), format!("b[{i}]")]);
                net
            })
            .collect();
        partial.push(row);
    }

    // `acc` is seventeen bits after each row; `acc[0]` is the next product bit
    // and the rest is what the following row adds to.
    let mut product = vec![String::new(); 2 * WIDTH];
    let mut acc: Vec<String> = partial[0].clone();
    product[0] = acc[0].clone();
    for i in 1..WIDTH {
        let addend = &partial[i];
        let mut next = Vec::with_capacity(WIDTH + 1);
        let mut carry = String::new();
        for bit in 0..WIDTH {
            // The shifted accumulator: bit `bit` of `acc >> 1`. Every row
            // after the first leaves seventeen bits behind, so only the first
            // shift has a vacated position to fill with zero.
            let left = acc.get(bit + 1).cloned().unwrap_or_else(|| lo.clone());
            let sum = top.wire(&format!("s{i}_{bit}"));
            let cout = top.wire(&format!("c{i}_{bit}"));
            if bit == 0 {
                top.instance(
                    "mul16_ha",
                    &format!("u_ha{i}"),
                    &[
                        ("a", left),
                        ("b", addend[bit].clone()),
                        ("s", sum.clone()),
                        ("c", cout.clone()),
                    ],
                );
            } else {
                top.instance(
                    "mul16_fa",
                    &format!("u_fa{i}_{bit}"),
                    &[
                        ("a", left),
                        ("b", addend[bit].clone()),
                        ("ci", carry.clone()),
                        ("s", sum.clone()),
                        ("co", cout.clone()),
                    ],
                );
            }
            next.push(sum);
            carry = cout;
        }
        next.push(carry);
        acc = next;
        product[i] = acc[0].clone();
    }
    for (index, net) in acc.iter().skip(1).enumerate() {
        product[WIDTH + index] = net.clone();
    }
    for (index, net) in product.iter().enumerate() {
        top.drive(Gate::Buf, &format!("p[{index}]"), &[net.clone()]);
    }
    top.limit_fanout(4, &[]);

    design(
        "mul16",
        &provenance("mul16 -- 16 by 16 combinational array multiplier."),
        vec![ha.finish(), fa.finish(), top.finish()],
    )
}

// ===========================================================================
// addcmp32
// ===========================================================================

/// A 32-bit three-operand adder, magnitude comparator and parity network, with
/// every port a scalar.
///
/// The widest circuit of the suite: 98 one-bit inputs and 106 one-bit outputs,
/// 204 ports, and no vector anywhere in the header. Hundreds of scalar ports
/// is itself an elaboration workload, and it is the one the published
/// benchmark family imposes everywhere and the rest of this corpus never does.
///
/// * `s = a + (b ^ sub) + m + cin`, in a carry-save stage feeding a
///   Kogge-Stone prefix adder
/// * `cs`, `cc` — the carry-save sum and carry vectors, which are three or
///   four gate levels from the inputs and are what makes this circuit wide and
///   shallow at once
/// * `eq`, `lt`, `gt` — unsigned comparison of `a` against `b`
/// * `pa`, `pb`, `pm`, `ps` — parity over each operand and the sum
/// * `oe` — an output enable ANDed into all 106 outputs, and the one net in
///   the suite deliberately left unbuffered, because a net driving a hundred
///   gate inputs is the thing being stressed
pub fn addcmp32() -> Design {
    const WIDTH: usize = 32;
    let mut n = Builder::new("addcmp32");
    n.doc("A 32-bit three-operand adder, magnitude comparator and parity");
    n.doc("network, with every one of its 204 ports a scalar.");

    let a = scalar_names("a", WIDTH);
    let b = scalar_names("b", WIDTH);
    let m = scalar_names("m", WIDTH);
    for name in a.iter().chain(&b).chain(&m) {
        n.input(name, 1);
    }
    for name in ["sub", "cin", "oe"] {
        n.input(name, 1);
    }
    let s_names = scalar_names("s", WIDTH);
    let cs_names = scalar_names("cs", WIDTH);
    let cc_names = scalar_names("cc", WIDTH);
    for name in s_names.iter().chain(&cs_names).chain(&cc_names) {
        n.output(name, 1);
    }
    let flags = [
        "cout", "ovf", "eq", "lt", "gt", "zero", "pa", "pb", "pm", "ps",
    ];
    for name in flags {
        n.output(name, 1);
    }
    let lo = n.tie("lo", false);

    let beff: Vec<String> = (0..WIDTH)
        .map(|i| xor2n(&mut n, "be", &b[i], "sub"))
        .collect();
    let (csum, ccarry) = compress_3to2(&mut n, "csa", &a, &beff, &m);

    // The carry vector weighs one place more than the sum vector, so the
    // prefix adder sees it shifted.
    let shifted: Vec<String> = std::iter::once(lo.clone())
        .chain(ccarry.iter().take(WIDTH - 1).cloned())
        .collect();
    let (sum, carries) = kogge_stone(&mut n, "ks", &csum, &shifted, "cin");

    let same: Vec<String> = (0..WIDTH)
        .map(|i| xnor2n(&mut n, "eqb", &a[i], &b[i]))
        .collect();
    let (equal, greater, lesser) = prefix_compare(&mut n, "cmp", &a, &b, &same);

    let any = n.reduce(Gate::Or, "zw", &sum, 8);
    let zero = inv(&mut n, "zw", &any);
    let ovf = xor2n(&mut n, "ovf", &carries[WIDTH], &carries[WIDTH - 1]);
    let pa = xor_tree_n(&mut n, "pa", &a);
    let pb = xor_tree_n(&mut n, "pb", &b);
    let pm = xor_tree_n(&mut n, "pm", &m);
    let ps = xor_tree_n(&mut n, "ps", &sum);

    // Everything is gated by one net. `oe` is exempted from the fanout pass
    // below, so it reaches all 106 gates directly.
    n.limit_fanout(4, &["oe"]);
    let mut gated: Vec<(String, String)> = Vec::new();
    for i in 0..WIDTH {
        gated.push((s_names[i].clone(), sum[i].clone()));
        gated.push((cs_names[i].clone(), csum[i].clone()));
        gated.push((cc_names[i].clone(), ccarry[i].clone()));
    }
    for (port, net) in flags.iter().zip([
        carries[WIDTH].clone(),
        ovf,
        equal,
        lesser,
        greater,
        zero,
        pa,
        pb,
        pm,
        ps,
    ]) {
        gated.push(((*port).to_string(), net));
    }
    for (port, net) in &gated {
        n.drive(Gate::And, port, &[net.clone(), "oe".to_string()]);
    }

    design(
        "addcmp32",
        &provenance("addcmp32 -- 32-bit three-operand adder, comparator and parity network."),
        vec![n.finish()],
    )
}

/// A Kogge-Stone prefix adder.
///
/// Returns the sum bits and the carries, where `carries[i]` is the carry into
/// bit `i`; `carries[width]` is the carry out and `carries[width - 1]` is what
/// an overflow flag needs beside it.
fn kogge_stone(
    n: &mut Builder,
    p: &str,
    x: &[String],
    y: &[String],
    cin: &str,
) -> (Vec<String>, Vec<String>) {
    let width = x.len();
    assert_eq!(width, y.len());
    let propagate: Vec<String> = (0..width)
        .map(|i| xor2n(n, &format!("{p}_p"), &x[i], &y[i]))
        .collect();
    let generate: Vec<String> = (0..width)
        .map(|i| and2(n, &format!("{p}_g"), &x[i], &y[i]))
        .collect();

    let mut group_p = propagate.clone();
    let mut group_g = generate.clone();
    let mut distance = 1usize;
    while distance < width {
        let mut next_p = group_p.clone();
        let mut next_g = group_g.clone();
        for i in distance..width {
            let carried = and2(n, &format!("{p}_gg"), &group_p[i], &group_g[i - distance]);
            next_g[i] = or2(n, &format!("{p}_gg"), &group_g[i], &carried);
            next_p[i] = and2(n, &format!("{p}_gp"), &group_p[i], &group_p[i - distance]);
        }
        group_p = next_p;
        group_g = next_g;
        distance *= 2;
    }

    let mut carries = Vec::with_capacity(width + 1);
    carries.push(cin.to_string());
    for i in 0..width {
        let through = and2(n, &format!("{p}_c"), &group_p[i], cin);
        carries.push(or2(n, &format!("{p}_c"), &group_g[i], &through));
    }
    let sum: Vec<String> = (0..width)
        .map(|i| xor2n(n, &format!("{p}_s"), &propagate[i], &carries[i]))
        .collect();
    (sum, carries)
}

/// An unsigned magnitude comparator over a balanced prefix tree.
///
/// Each node combines two adjacent spans: the pair is equal when both halves
/// are, and greater when the high half is greater or the high half is equal
/// and the low half is greater. Returns `(eq, gt, lt)` for the whole word.
fn prefix_compare(
    n: &mut Builder,
    p: &str,
    left: &[String],
    right: &[String],
    same: &[String],
) -> (String, String, String) {
    let width = left.len();
    let mut spans: Vec<(String, String, String)> = (0..width)
        .map(|i| {
            let not_right = inv(n, &format!("{p}_nr"), &right[i]);
            let not_left = inv(n, &format!("{p}_nl"), &left[i]);
            let above = and2(n, &format!("{p}_gt"), &left[i], &not_right);
            let below = and2(n, &format!("{p}_lt"), &not_left, &right[i]);
            (same[i].clone(), above, below)
        })
        .collect();
    while spans.len() > 1 {
        let mut next = Vec::with_capacity(spans.len().div_ceil(2));
        for pair in spans.chunks(2) {
            if pair.len() == 1 {
                next.push(pair[0].clone());
                continue;
            }
            // `pair[1]` is the more significant half.
            let (low_eq, low_gt, low_lt) = &pair[0];
            let (high_eq, high_gt, high_lt) = &pair[1];
            let eq = and2(n, &format!("{p}_e"), high_eq, low_eq);
            let carried_gt = and2(n, &format!("{p}_cg"), high_eq, low_gt);
            let gt = or2(n, &format!("{p}_cg"), high_gt, &carried_gt);
            let carried_lt = and2(n, &format!("{p}_cl"), high_eq, low_lt);
            let lt = or2(n, &format!("{p}_cl"), high_lt, &carried_lt);
            next.push((eq, gt, lt));
        }
        spans = next;
    }
    spans.remove(0)
}
