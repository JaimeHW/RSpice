//! Stimulus specifications and the deterministic vector sets behind them.
//!
//! Three kinds of pattern, in this order, for every circuit:
//!
//! 1. **Corners.** Zero, one, all-ones, the sign bit, the alternating words,
//!    and whatever else the circuit's own function makes a boundary — the
//!    highest and lowest priority channel, a carry that propagates the whole
//!    width, a decimal digit at nine.
//! 2. **Walking patterns.** One bit set, or one bit clear, swept across the
//!    widest operand. A structural defect that touches one bit slice shows up
//!    here and nowhere else.
//! 3. **Pseudo-random patterns.** From [`Lcg`], a splitmix64 sequence with a
//!    per-circuit constant seed written into the source. Nothing consults a
//!    clock, an environment variable, or a thread-local generator, so the
//!    `.stim` files regenerate byte for byte on every host.
//!
//! The ECC circuits get a fourth kind: an error-injection sweep that flips
//! each codeword bit in turn, and — for the double-error-detecting one —
//! pairs.

use super::netlist::Dir;
use super::reference::{Values, put};
use super::structural;

/// One port of a stimulus, in declaration order.
#[derive(Debug, Clone)]
pub struct StimPort {
    pub name: String,
    pub width: u32,
    pub input: bool,
}

/// Everything needed to write one `.stim` file.
#[derive(Debug, Clone)]
pub struct Spec {
    pub module: String,
    pub ports: Vec<StimPort>,
    /// `(port, half period)` for a clocked circuit.
    pub clock: Option<(String, u64)>,
    pub step: u64,
    pub settle: u64,
    /// One entry per vector, keyed by port name; scalar-ported circuits carry
    /// one entry per bit.
    pub vectors: Vec<Values>,
    /// The manifest note, which is what a reader sees before the file.
    pub note: String,
    /// The manifest's oracle column.
    pub oracles: &'static str,
}

impl Spec {
    /// Driven inputs, in declaration order and excluding the clock.
    pub fn driven(&self) -> Vec<&StimPort> {
        self.ports
            .iter()
            .filter(|port| port.input)
            .filter(|port| {
                self.clock
                    .as_ref()
                    .is_none_or(|(clock, _)| *clock != port.name)
            })
            .collect()
    }

    pub fn observed(&self) -> Vec<&StimPort> {
        self.ports.iter().filter(|port| !port.input).collect()
    }
}

/// splitmix64, with its published constants.
///
/// A named, self-contained generator rather than a crate dependency: the
/// vectors are part of the corpus and their provenance has to be readable in
/// the file that produces them.
pub struct Lcg(u64);

impl Lcg {
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// The next value, truncated to `width` bits.
    pub fn bits(&mut self, width: u32) -> u64 {
        self.next() & mask(width)
    }
}

fn mask(width: u32) -> u64 {
    if width >= 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    }
}

/// The boundary values of a `width`-bit operand.
fn corners(width: u32) -> Vec<u64> {
    let full = mask(width);
    vec![
        0,
        1,
        full,
        full >> 1,
        1 << (width - 1),
        0xAAAA_AAAA_AAAA_AAAA & full,
        0x5555_5555_5555_5555 & full,
    ]
}

/// The stimulus specification for one circuit.
pub fn spec(name: &str) -> Option<Spec> {
    let (clock, step, settle) = if is_clocked(name) {
        (Some(("clk".to_string(), 5u64)), 10u64, 8u64)
    } else {
        (None, 10, 5)
    };
    let ports = ports(name)?;
    Some(Spec {
        module: name.to_string(),
        ports,
        clock,
        step,
        settle,
        vectors: vectors(name),
        note: note(name).to_string(),
        oracles: "iverilog,verilator",
    })
}

fn is_clocked(name: &str) -> bool {
    matches!(name, "lfsr32" | "pipe_mac8")
}

fn ports(name: &str) -> Option<Vec<StimPort>> {
    if let Some(design) = structural(name) {
        return Some(
            design
                .top_module()
                .ports
                .iter()
                .map(|port| StimPort {
                    name: port.name.clone(),
                    width: port.width,
                    input: port.dir == Dir::In,
                })
                .collect(),
        );
    }
    let spec: &[(&str, u32, bool)] = match name {
        "lfsr32" => &[
            ("clk", 1, true),
            ("rst", 1, true),
            ("ld", 1, true),
            ("seed", 32, true),
            ("q", 32, false),
            ("fb", 1, false),
            ("msb", 1, false),
        ],
        "pipe_mac8" => &[
            ("clk", 1, true),
            ("rst", 1, true),
            ("en", 1, true),
            ("clr", 1, true),
            ("a", 8, true),
            ("b", 8, true),
            ("prod", 16, false),
            ("acc", 24, false),
            ("vld", 1, false),
        ],
        _ => return None,
    };
    Some(
        spec.iter()
            .map(|(name, width, input)| StimPort {
                name: (*name).to_string(),
                width: *width,
                input: *input,
            })
            .collect(),
    )
}

fn note(name: &str) -> &'static str {
    match name {
        "intc27" => {
            "27-channel priority interrupt controller; walking requests, mask sweeps, seeded patterns"
        }
        "sec32" => {
            "32-bit Hamming SEC decoder in xor/xnor primitives; every codeword bit flipped in turn"
        }
        "sec32n" => {
            "sec32's function in NAND gates only, on sec32's own vectors; the two must agree"
        }
        "alu8" => "8-bit ALU over all eight functions, carry corners and seeded operands",
        "secded16" => {
            "16-bit SEC/DED decoder; single-error and double-error injection over the codeword"
        }
        "alu12c" => "12-bit ALU datapath under a separate control decoder; two module instances",
        "alu8bcd" => "8-bit ALU with binary and decimal arithmetic paths and a mode select",
        "alu9d" => {
            "two 9-bit ALU slices under shared control, with a comparator across their results"
        }
        "mul16" => {
            "16 by 16 array multiplier, 240 adder-cell instances and a carry chain hundreds of levels deep"
        }
        "addcmp32" => "32-bit three-operand adder, comparator and parity network; 205 scalar ports",
        "lfsr32" => {
            "32-bit Fibonacci LFSR; reset, free running, seed load, and the all-zero fixed point"
        }
        "pipe_mac8" => {
            "three-stage pipelined 8-bit multiply-accumulate; non-blocking stage handover"
        }
        _ => "",
    }
}

/// The vector set for one circuit.
pub fn vectors(name: &str) -> Vec<Values> {
    match name {
        "intc27" => intc27(),
        // The NAND twin is driven by its primitive twin's vectors, because
        // the point of the pair is that the two agree on identical input.
        "sec32" | "sec32n" => sec32(),
        "secded16" => secded16(),
        "alu8" => alu8(),
        "alu12c" => alu12c(),
        "alu8bcd" => alu8bcd(),
        "alu9d" => alu9d(),
        "mul16" => mul16(),
        "addcmp32" => addcmp32(),
        "lfsr32" => lfsr32(),
        "pipe_mac8" => pipe_mac8(),
        other => panic!("'{other}' is not a scale circuit"),
    }
}

fn row(pairs: &[(&str, u64)]) -> Values {
    pairs
        .iter()
        .map(|(name, value)| ((*name).to_string(), *value))
        .collect()
}

// ===========================================================================
// Per-circuit vector sets
// ===========================================================================

fn intc27() -> Vec<Values> {
    const CHANNELS: u32 = 27;
    let all = mask(CHANNELS);
    let mut out = Vec::new();
    // One request at a time: the priority encoder must name each channel.
    for channel in 0..CHANNELS {
        out.push(row(&[("req", 1 << channel), ("msk", all), ("gen", 1)]));
    }
    // Every request but one: the winner is channel 26 unless 26 is the one
    // masked off, which is the only case that makes the mask visible at the
    // top of the chain.
    for channel in 0..CHANNELS {
        out.push(row(&[
            ("req", all),
            ("msk", all ^ (1 << channel)),
            ("gen", 1),
        ]));
    }
    for (req, msk, global) in [
        (0, all, 1),
        (all, all, 0),
        (all, 0, 1),
        (all, all, 1),
        (1, 1, 0),
        (1 << 26, all, 1),
    ] {
        out.push(row(&[("req", req), ("msk", msk), ("gen", global)]));
    }
    let mut rng = Lcg::new(0x1C27_0001);
    for index in 0..48 {
        out.push(row(&[
            ("req", rng.bits(CHANNELS)),
            ("msk", rng.bits(CHANNELS)),
            ("gen", u64::from(index % 5 != 4)),
        ]));
    }
    out
}

/// Check bits for a data word, from the Hamming construction.
fn encode(data: u64, parity: u32, width: usize) -> u64 {
    let positions = super::circuits::hamming_data_positions(parity, width);
    let mut check = 0u64;
    for k in 0..parity {
        let mut bit = 0u64;
        for (index, position) in positions.iter().enumerate() {
            if position >> k & 1 == 1 {
                bit ^= data >> index & 1;
            }
        }
        check |= bit << k;
    }
    check
}

fn sec32() -> Vec<Values> {
    let mut out = Vec::new();
    let mut rng = Lcg::new(0x5EC3_0002);
    let mut words: Vec<u64> = corners(32);
    for _ in 0..5 {
        words.push(rng.bits(32));
    }

    // Clean codewords first: the syndrome must be zero and the data must pass
    // through untouched.
    for data in &words {
        out.push(row(&[("d", *data), ("c", encode(*data, 6, 32))]));
    }
    // One flipped bit at each of the 38 codeword positions, over a rotating
    // set of base words.
    for position in 0..38usize {
        let data = words[position % words.len()];
        let check = encode(data, 6, 32);
        if position < 32 {
            out.push(row(&[("d", data ^ 1 << position), ("c", check)]));
        } else {
            out.push(row(&[("d", data), ("c", check ^ 1 << (position - 32))]));
        }
    }
    // Two flips: a single-error corrector cannot fix these and does not claim
    // to, but it must still produce the syndrome the construction says.
    for pair in 0..10usize {
        let data = words[pair % words.len()];
        let check = encode(data, 6, 32);
        let first = pair * 3 % 32;
        let second = (pair * 7 + 1) % 32;
        out.push(row(&[("d", data ^ 1 << first ^ 1 << second), ("c", check)]));
    }
    out
}

fn secded16() -> Vec<Values> {
    let mut out = Vec::new();
    let mut rng = Lcg::new(0x5ED1_0003);
    let mut words: Vec<u64> = corners(16);
    for _ in 0..5 {
        words.push(rng.bits(16));
    }
    let encode16 = |data: u64| -> (u64, u64) {
        let check = encode(data, 5, 16);
        let overall = u64::from((data.count_ones() + check.count_ones()) % 2 == 1);
        (check, overall)
    };

    for data in &words {
        let (check, overall) = encode16(*data);
        out.push(row(&[("d", *data), ("c", check), ("p", overall)]));
    }
    // Single errors across all 22 codeword bits: 16 data, 5 check, and the
    // overall parity bit itself, which the decoder must not mistake for a
    // data error.
    for position in 0..22usize {
        let data = words[position % words.len()];
        let (check, overall) = encode16(data);
        let (d, c, p) = match position {
            0..=15 => (data ^ 1 << position, check, overall),
            16..=20 => (data, check ^ 1 << (position - 16), overall),
            _ => (data, check, overall ^ 1),
        };
        out.push(row(&[("d", d), ("c", c), ("p", p)]));
    }
    // Double errors, which must raise `ded` and correct nothing.
    for pair in 0..20usize {
        let data = words[pair % words.len()];
        let (check, overall) = encode16(data);
        let first = pair % 16;
        let second = (pair * 5 + 3) % 16;
        if first == second {
            out.push(row(&[
                ("d", data ^ 1 << first),
                ("c", check ^ 1),
                ("p", overall),
            ]));
        } else {
            out.push(row(&[
                ("d", data ^ 1 << first ^ 1 << second),
                ("c", check),
                ("p", overall),
            ]));
        }
    }
    out
}

fn alu8() -> Vec<Values> {
    let mut out = Vec::new();
    let pairs = [
        (0x00u64, 0x00u64),
        (0xFF, 0xFF),
        (0xFF, 0x01),
        (0x80, 0x80),
        (0x7F, 0x01),
    ];
    for f in 0..8u64 {
        for (a, b) in pairs {
            for cin in 0..2u64 {
                out.push(row(&[("a", a), ("b", b), ("f", f), ("cin", cin)]));
            }
        }
    }
    let mut rng = Lcg::new(0xA108_0004);
    for index in 0..48u64 {
        out.push(row(&[
            ("a", rng.bits(8)),
            ("b", rng.bits(8)),
            ("f", index % 8),
            ("cin", index / 8 % 2),
        ]));
    }
    out
}

fn alu12c() -> Vec<Values> {
    const WIDTH: u32 = 12;
    let full = mask(WIDTH);
    let mut out = Vec::new();
    let pairs = [
        (0u64, 0u64),
        (full, full),
        (full, 1),
        (1 << 11, 1 << 11),
        (0xA5A, 0x5A5),
    ];
    for op in 0..16u64 {
        for (a, b) in pairs {
            out.push(row(&[
                ("a", a),
                ("b", b),
                ("op", op),
                ("mode", 0),
                ("cin", 0),
            ]));
        }
    }
    // The mode bits: forced carry, disabled outputs, and both together.
    for mode in 0..4u64 {
        for op in [0u64, 1, 2, 6, 10, 14] {
            out.push(row(&[
                ("a", 0x0F0),
                ("b", 0x003),
                ("op", op),
                ("mode", mode),
                ("cin", mode & 1),
            ]));
        }
    }
    let mut rng = Lcg::new(0xA12C_0005);
    for index in 0..48u64 {
        out.push(row(&[
            ("a", rng.bits(WIDTH)),
            ("b", rng.bits(WIDTH)),
            ("op", index % 16),
            ("mode", u64::from(index % 7 == 6)),
            ("cin", index / 16 % 2),
        ]));
    }
    out
}

fn alu8bcd() -> Vec<Values> {
    let mut out = Vec::new();
    // Decimal add and subtract across digit boundaries: nine plus one, the
    // carry out of both digits, and a borrow through the low digit.
    let decimal = [
        (0x00u64, 0x00u64),
        (0x09, 0x01),
        (0x27, 0x45),
        (0x88, 0x33),
        (0x99, 0x99),
        (0x50, 0x50),
        (0x42, 0x17),
        (0x10, 0x09),
        (0x00, 0x99),
        (0x45, 0x55),
    ];
    for (a, b) in decimal {
        for f in [0u64, 1] {
            for cin in 0..2u64 {
                out.push(row(&[
                    ("a", a),
                    ("b", b),
                    ("f", f),
                    ("cin", cin),
                    ("dec", 1),
                ]));
            }
        }
    }
    // The same unit in binary, over all sixteen functions.
    for f in 0..16u64 {
        for (a, b) in [(0x00u64, 0x00u64), (0xFF, 0xFF), (0x80, 0x01), (0x3C, 0x5A)] {
            out.push(row(&[
                ("a", a),
                ("b", b),
                ("f", f),
                ("cin", f & 1),
                ("dec", 0),
            ]));
        }
    }
    // Digits above nine, which `inval` must report and the decimal path must
    // still answer for.
    for (a, b) in [(0x0Au64, 0x00u64), (0xF0, 0x01), (0xAB, 0xCD), (0x1A, 0x2B)] {
        for dec in 0..2u64 {
            out.push(row(&[
                ("a", a),
                ("b", b),
                ("f", 0),
                ("cin", 0),
                ("dec", dec),
            ]));
        }
    }
    let mut rng = Lcg::new(0xABCD_0006);
    for index in 0..48u64 {
        out.push(row(&[
            ("a", rng.bits(8)),
            ("b", rng.bits(8)),
            ("f", index % 16),
            ("cin", index / 16 % 2),
            ("dec", u64::from(index % 3 == 0)),
        ]));
    }
    out
}

fn alu9d() -> Vec<Values> {
    const WIDTH: u32 = 9;
    let full = mask(WIDTH);
    let mut out = Vec::new();
    let sets = [
        (0u64, 0u64, 0u64, 0u64),
        (full, full, full, full),
        (full, 1, 0, 0),
        (0x155, 0x0AA, 0x0AA, 0x155),
    ];
    for op in 0..16u64 {
        for (a0, b0, a1, b1) in sets {
            out.push(row(&[
                ("a0", a0),
                ("b0", b0),
                ("a1", a1),
                ("b1", b1),
                ("op", op),
                ("cin0", 0),
                ("cin1", 0),
            ]));
        }
    }
    // The comparator, driven deliberately: equal results, then each ordering.
    for (a0, a1) in [(0x100u64, 0x100u64), (0x0FF, 0x100), (0x100, 0x0FF)] {
        for cin in 0..2u64 {
            out.push(row(&[
                ("a0", a0),
                ("b0", 0),
                ("a1", a1),
                ("b1", 0),
                ("op", 10),
                ("cin0", cin),
                ("cin1", cin),
            ]));
        }
    }
    let mut rng = Lcg::new(0xA9D0_0007);
    for index in 0..48u64 {
        out.push(row(&[
            ("a0", rng.bits(WIDTH)),
            ("b0", rng.bits(WIDTH)),
            ("a1", rng.bits(WIDTH)),
            ("b1", rng.bits(WIDTH)),
            ("op", index % 16),
            ("cin0", index % 2),
            ("cin1", index / 2 % 2),
        ]));
    }
    out
}

fn mul16() -> Vec<Values> {
    let mut out = Vec::new();
    for (a, b) in [
        (0u64, 0u64),
        (0, 0xFFFF),
        (0xFFFF, 0),
        (1, 0xFFFF),
        (0xFFFF, 0xFFFF),
        (0x8000, 0x8000),
        (0x00FF, 0x0101),
        (0xAAAA, 0x5555),
        (0x7FFF, 0x0002),
    ] {
        out.push(row(&[("a", a), ("b", b)]));
    }
    // A single bit in each operand in turn: one partial-product row, and the
    // whole carry chain shifted under it.
    for bit in 0..16u32 {
        out.push(row(&[("a", 1 << bit), ("b", 0xFFFF)]));
        out.push(row(&[("a", 0xFFFF), ("b", 1 << bit)]));
    }
    let mut rng = Lcg::new(0x0016_0008);
    for _ in 0..48 {
        out.push(row(&[("a", rng.bits(16)), ("b", rng.bits(16))]));
    }
    out
}

fn addcmp32() -> Vec<Values> {
    const WIDTH: u32 = 32;
    let full = mask(WIDTH);
    let mut out = Vec::new();
    let mut push = |a: u64, b: u64, m: u64, sub: u64, cin: u64, oe: u64| {
        let mut values = Values::new();
        put(&mut values, "a", WIDTH, a);
        put(&mut values, "b", WIDTH, b);
        put(&mut values, "m", WIDTH, m);
        // The three control ports are scalars in their own right rather than
        // one-bit words, so they are named without an index.
        values.insert("sub".into(), sub);
        values.insert("cin".into(), cin);
        values.insert("oe".into(), oe);
        out.push(values);
    };

    for (a, b, m, sub, cin) in [
        (0u64, 0u64, 0u64, 0u64, 0u64),
        (full, 0, 0, 0, 1),
        (full, full, full, 0, 1),
        (full, 1, 0, 0, 0),
        (0x8000_0000, 0x8000_0000, 0, 0, 0),
        (0x7FFF_FFFF, 1, 0, 0, 0),
        (0x1234_5678, 0x1234_5678, 0, 1, 1),
        (0xAAAA_AAAA, 0x5555_5555, 0, 0, 0),
        (0x0000_FFFF, 0xFFFF_0000, 0, 0, 0),
        (1, 2, 4, 0, 0),
    ] {
        push(a, b, m, sub, cin, 1);
    }
    // The output enable, which is the one net in the suite left unbuffered.
    for oe in [0u64, 1] {
        push(0xDEAD_BEEF, 0x1234_5678, 0x0F0F_0F0F, 0, 1, oe);
        push(0, 0, 0, 1, 0, oe);
    }
    // A single bit walked across the first operand against an all-ones
    // second, so every carry position is exercised from a clean state.
    for bit in 0..WIDTH {
        push(1 << bit, full, 0, 0, 0, 1);
    }
    let mut rng = Lcg::new(0xADC3_0009);
    for index in 0..48u64 {
        let a = rng.bits(WIDTH);
        let b = rng.bits(WIDTH);
        let m = rng.bits(WIDTH);
        push(a, b, m, index % 2, index / 2 % 2, 1);
    }
    out
}

fn lfsr32() -> Vec<Values> {
    let mut out = Vec::new();
    // Reset first: every observation after this one is of a defined state,
    // and the model starts from the same place the hardware does.
    for _ in 0..2 {
        out.push(row(&[("rst", 1), ("ld", 0), ("seed", 0)]));
    }
    for _ in 0..40 {
        out.push(row(&[("rst", 0), ("ld", 0), ("seed", 0)]));
    }
    // A seed that puts a tap under the feedback immediately.
    out.push(row(&[("rst", 0), ("ld", 1), ("seed", 0x8000_0001)]));
    for _ in 0..20 {
        out.push(row(&[("rst", 0), ("ld", 0), ("seed", 0)]));
    }
    // The all-zero fixed point, which is a property of the recurrence rather
    // than a defect, and then a reset out of it.
    out.push(row(&[("rst", 0), ("ld", 1), ("seed", 0)]));
    for _ in 0..4 {
        out.push(row(&[("rst", 0), ("ld", 0), ("seed", 0)]));
    }
    out.push(row(&[("rst", 1), ("ld", 0), ("seed", 0)]));
    for _ in 0..4 {
        out.push(row(&[("rst", 0), ("ld", 0), ("seed", 0)]));
    }
    out
}

fn pipe_mac8() -> Vec<Values> {
    let mut out = Vec::new();
    let mut push = |rst: u64, en: u64, clr: u64, a: u64, b: u64| {
        out.push(row(&[
            ("rst", rst),
            ("en", en),
            ("clr", clr),
            ("a", a),
            ("b", b),
        ]));
    };
    for _ in 0..2 {
        push(1, 0, 0, 0, 0);
    }
    // One operand pair, then four idle clocks: the accumulate lands on the
    // third edge and nothing moves after it.
    push(0, 1, 0, 3, 5);
    for _ in 0..4 {
        push(0, 0, 0, 0, 0);
    }
    // A back-to-back stream, so three accumulates are in flight at once.
    for index in 0..12u64 {
        push(0, 1, 0, index + 1, 17 - index);
    }
    for _ in 0..3 {
        push(0, 0, 0, 0, 0);
    }
    // Clear while the pipeline is still draining.
    push(0, 1, 0, 200, 200);
    push(0, 1, 0, 201, 201);
    push(0, 0, 1, 0, 0);
    for _ in 0..3 {
        push(0, 0, 0, 0, 0);
    }
    let mut rng = Lcg::new(0x9AC8_000A);
    for index in 0..30u64 {
        push(
            0,
            u64::from(index % 4 != 3),
            u64::from(index % 11 == 10),
            rng.bits(8),
            rng.bits(8),
        );
    }
    for _ in 0..3 {
        push(0, 0, 0, 0, 0);
    }
    out
}

// ===========================================================================
// Rendering
// ===========================================================================

/// Render a `.stim` file.
pub fn render_stim(spec: &Spec) -> String {
    let mut out = String::new();
    out.push_str(super::super::corpus::STIMULUS_HEADER);
    out.push('\n');
    out.push_str("#\n");
    for line in [
        "Generated by rspice-conformance's Verilog scale-suite generator;".to_string(),
        "edit the generator, not this file.".to_string(),
        String::new(),
        spec.note.clone(),
    ] {
        if line.is_empty() {
            out.push_str("#\n");
        } else {
            out.push_str("# ");
            out.push_str(&line);
            out.push('\n');
        }
    }
    out.push('\n');
    out.push_str(&format!("module {}\n\n", spec.module));
    for port in &spec.ports {
        let keyword = if port.input { "input" } else { "output" };
        out.push_str(&format!("{keyword} {} {}\n", port.name, port.width));
    }
    out.push('\n');
    if let Some((clock, half)) = &spec.clock {
        out.push_str(&format!("clock {clock} {half}\n"));
    }
    out.push_str(&format!("step {}\n", spec.step));
    out.push_str(&format!("settle {}\n\n", spec.settle));

    let driven = spec.driven();
    for values in &spec.vectors {
        out.push_str("vector");
        for port in &driven {
            let value = values
                .get(&port.name)
                .unwrap_or_else(|| panic!("no vector value for '{}'", port.name));
            out.push(' ');
            out.push_str(&binary(*value, port.width));
        }
        out.push('\n');
    }
    out
}

/// A value as the trace grammar writes it: `%b`, MSB first, exactly `width`
/// characters.
pub fn binary(value: u64, width: u32) -> String {
    (0..width)
        .rev()
        .map(|bit| if value >> bit & 1 == 1 { '1' } else { '0' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_circuit_has_a_specification_with_vectors() {
        for name in super::super::NAMES {
            let spec = spec(name).unwrap_or_else(|| panic!("{name} has no stimulus"));
            assert!(!spec.vectors.is_empty(), "{name} has no vectors");
            assert!(!spec.driven().is_empty(), "{name} drives nothing");
            assert!(!spec.observed().is_empty(), "{name} observes nothing");
            assert!(!spec.note.is_empty(), "{name} has no manifest note");
            for values in &spec.vectors {
                for port in spec.driven() {
                    let value = values
                        .get(&port.name)
                        .unwrap_or_else(|| panic!("{name}: no value for '{}'", port.name));
                    assert!(
                        *value <= mask(port.width),
                        "{name}: {value} does not fit '{}'",
                        port.name
                    );
                }
            }
        }
    }

    #[test]
    fn the_generator_is_deterministic() {
        for name in super::super::NAMES {
            let first = render_stim(&spec(name).expect("present"));
            let second = render_stim(&spec(name).expect("present"));
            assert_eq!(first, second, "{name} renders differently twice");
            assert!(!first.contains('\r'), "{name} carries a carriage return");
        }
    }

    #[test]
    fn splitmix_is_pinned_to_its_published_constants() {
        // The first three outputs from seed zero. Pinned so a well-meaning
        // edit to the generator cannot quietly reshuffle every vector set.
        let mut rng = Lcg::new(0);
        assert_eq!(rng.next(), 0xE220_A839_7B1D_CDAF);
        assert_eq!(rng.next(), 0x6E78_9E6A_A1B9_65F4);
        assert_eq!(rng.next(), 0x06C4_5D18_8009_454F);
    }

    #[test]
    fn binary_is_msb_first_and_exactly_as_wide_as_the_port() {
        assert_eq!(binary(0b1010, 4), "1010");
        assert_eq!(binary(1, 8), "00000001");
        assert_eq!(binary(0, 1), "0");
        assert_eq!(binary(u64::from(u32::MAX), 32).len(), 32);
    }

    #[test]
    fn the_nand_twin_is_driven_by_its_primitive_twin() {
        let left = spec("sec32").expect("present");
        let right = spec("sec32n").expect("present");
        assert_eq!(left.vectors, right.vectors);
        assert_eq!(
            left.ports
                .iter()
                .map(|port| (port.name.clone(), port.width, port.input))
                .collect::<Vec<_>>(),
            right
                .ports
                .iter()
                .map(|port| (port.name.clone(), port.width, port.input))
                .collect::<Vec<_>>(),
        );
    }
}
