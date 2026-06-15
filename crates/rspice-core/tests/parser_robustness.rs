//! Parser robustness: malformed input must produce `Err`, never a panic.
//!
//! Deterministic chaos testing — a seeded xorshift stream drives byte
//! flips, truncations, token splices, and special-character injection
//! over a seed corpus of representative decks, and every mutant goes
//! through the full text entry points (`Netlist::parse`, multi-run
//! expansion, SPEF). The streams are fixed, so a failure reproduces
//! exactly; the panicking input is printed for triage.
//!
//! This is the in-tree, all-platform tier of the roadmap's robustness
//! gate ("fuzzing: zero panics on malformed input"); a libFuzzer setup
//! can layer on top later without replacing it.

use std::panic::{AssertUnwindSafe, catch_unwind};

use rspice_core::Netlist;
use rspice_core::netlist::multi_run::expand_multi_run;
use rspice_core::netlist::spef::SpefFile;

/// Deterministic xorshift64* stream.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound.max(1) as u64) as usize
    }
}

const SEED_DECKS: &[&str] = &[
    // Plain RC transient.
    "rc deck\nV1 in 0 DC 1 PULSE(0 1 0 1n 1n 1u 2u)\nR1 in out 4.7k\nC1 out 0 {cap*2}\n.param cap=1p\n.tran 1n 10u\n.print tran v(out)\n.end\n",
    // Hierarchy, models, measurements, conditionals.
    ".subckt amp a y params: gain=10\nE1 y 0 a 0 {gain}\n.ends\nXA in out amp gain=20\nV1 in 0 AC 1 SIN(0 1 1k)\n.model dm d is=1e-14 n=1.5\nD1 out 0 dm\n.if (1)\nR9 out 0 1meg\n.endif\n.meas tran vpk MAX v(out)\n.ac dec 10 1 1meg\n.end\n",
    // Multi-run constructs.
    "alters\n.param rl=1k vdd=1\nV1 a 0 {vdd}\nR1 a 0 {rl}\n.data tbl vdd rl\n1.0 1k\n2.5 2k\n.enddata\n.dc data=tbl\n.alter hot\n.param rl=500\n.temp 125\n.end\n",
    // Sources and continuation lines.
    "sources\nI1 0 n1 PWL(0 0 1u 1m\n+ 2u 0)\nG1 n2 0 n1 0 2m\nL1 n1 n2 10u\nB1 n3 0 V=v(n1)*v(n1)\n.options reltol=1e-4\n.four 1k v(n1)\n.end\n",
];

const SPEF_SEED: &str = "*SPEF \"IEEE 1481-2009\"\n*DESIGN \"d\"\n*DIVIDER /\n*DELIMITER :\n*BUS_DELIMITER [ ]\n*T_UNIT 1 NS\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*L_UNIT 1 HENRY\n*NAME_MAP\n*1 net1\n*2 X1\n*PORTS\n*1 I\n*D_NET *1 1.0\n*CONN\n*P *1 I\n*I *2:A I\n*CAP\n1 *1:1 0.5\n*RES\n1 *1 *1:1 2.0\n*END\n";

/// Characters that historically trip SPICE tokenizers.
const SPICE_NOISE: &[u8] = b"{}()'\"=$*+.,:;\\/[]%@!~^&|<>?#\x00\x7f\xc3\xa9";

fn mutate(rng: &mut Rng, seed: &str) -> String {
    let mut bytes = seed.as_bytes().to_vec();
    match rng.below(5) {
        // Truncate at a random point.
        0 => {
            let cut = rng.below(bytes.len() + 1);
            bytes.truncate(cut);
        }
        // Flip a handful of bytes.
        1 => {
            for _ in 0..1 + rng.below(8) {
                if bytes.is_empty() {
                    break;
                }
                let pos = rng.below(bytes.len());
                bytes[pos] = (rng.next() & 0xff) as u8;
            }
        }
        // Inject tokenizer-hostile characters.
        2 => {
            for _ in 0..1 + rng.below(6) {
                let pos = rng.below(bytes.len() + 1);
                let ch = SPICE_NOISE[rng.below(SPICE_NOISE.len())];
                bytes.insert(pos, ch);
            }
        }
        // Duplicate a random slice somewhere else (token splice).
        3 => {
            if !bytes.is_empty() {
                let start = rng.below(bytes.len());
                let len = rng.below((bytes.len() - start).min(24) + 1);
                let slice: Vec<u8> = bytes[start..start + len].to_vec();
                let at = rng.below(bytes.len() + 1);
                for (offset, b) in slice.into_iter().enumerate() {
                    bytes.insert(at + offset, b);
                }
            }
        }
        // Delete a random slice.
        _ => {
            if !bytes.is_empty() {
                let start = rng.below(bytes.len());
                let len = rng.below((bytes.len() - start).min(24) + 1);
                bytes.drain(start..start + len);
            }
        }
    }
    // Parsers take &str: keep mutants valid UTF-8 (lossy round-trip).
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Drive `target` over `rounds` mutants of each seed; panic with the
/// offending input if the target panics.
fn chaos<F: Fn(&str)>(name: &str, seeds: &[&str], seed_base: u64, rounds: usize, target: F) {
    for (seed_idx, seed) in seeds.iter().enumerate() {
        let mut rng =
            Rng::new(seed_base ^ (seed_idx as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        for round in 0..rounds {
            let input = mutate(&mut rng, seed);
            let result = catch_unwind(AssertUnwindSafe(|| target(&input)));
            assert!(
                result.is_ok(),
                "{name} panicked (seed {seed_idx}, round {round}) on input:\n---\n{input}\n---"
            );
        }
    }
}

/// Every truncation prefix of every seed must parse without panicking —
/// the cheapest exhaustive slice of the input space.
#[test]
fn netlist_parse_survives_every_truncation() {
    for seed in SEED_DECKS {
        for cut in 0..=seed.len() {
            if !seed.is_char_boundary(cut) {
                continue;
            }
            let input = &seed[..cut];
            let result = catch_unwind(AssertUnwindSafe(|| {
                let _ = Netlist::parse(input);
            }));
            assert!(result.is_ok(), "parse panicked on truncation:\n{input}");
        }
    }
}

#[test]
fn netlist_parse_survives_chaos() {
    chaos("Netlist::parse", SEED_DECKS, 0x5EED_0001, 5000, |input| {
        let _ = Netlist::parse(input);
    });
}

#[test]
fn multi_run_expansion_survives_chaos() {
    chaos("expand_multi_run", SEED_DECKS, 0x5EED_0002, 4000, |input| {
        // Expansion output must itself parse without panicking.
        for deck in expand_multi_run(input) {
            let _ = Netlist::parse(&deck.source);
        }
    });
}

#[test]
fn spef_parse_survives_chaos() {
    chaos(
        "SpefFile::parse",
        &[SPEF_SEED],
        0x5EED_0003,
        8000,
        |input| {
            let _ = SpefFile::parse(input);
        },
    );
}

#[test]
fn spef_parse_survives_every_truncation() {
    for cut in 0..=SPEF_SEED.len() {
        if !SPEF_SEED.is_char_boundary(cut) {
            continue;
        }
        let input = &SPEF_SEED[..cut];
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = SpefFile::parse(input);
        }));
        assert!(
            result.is_ok(),
            "SPEF parse panicked on truncation:\n{input}"
        );
    }
}
