//! The gate-level scale suite: twelve original benchmark circuits, their
//! generator, and the reference models they are checked against.
//!
//! # Provenance, and why it is the point
//!
//! These circuits exist to give the digital corpus a workload at the scale a
//! gate-level benchmark family provides, without taking a netlist from one.
//! Every design here was written **from a published function description and
//! from nothing else**: an interrupt controller resolves priorities, a Hamming
//! decoder corrects the bit its syndrome names, an array multiplier
//! accumulates partial products. No benchmark netlist, in any format, from any
//! source, was opened while writing this generator, and none is needed to read
//! it: the function of each circuit is stated in the doc comment above its
//! builder, and the structure follows from textbook digital design. The
//! provenance chain for every file under `tests/verilog/scale/` therefore
//! starts and ends in this crate.
//!
//! That is why none of them is named after a benchmark circuit and why none of
//! them claims to reproduce one. What they reproduce is the *kind of work* a
//! benchmark family makes a simulator do — deep carry chains, wide fanout,
//! reconvergent XOR trees, hundreds of module instances, hundreds of scalar
//! ports, and one pair of circuits computing the same function through
//! completely different gates.
//!
//! # How a circuit is checked
//!
//! Three independent statements, and no two of them share a source:
//!
//! 1. **A reference model.** [`reference`] implements each circuit's function
//!    in Rust, at the level of the specification: `u32` arithmetic for the
//!    multiplier, wrapping add and the standard overflow identity for the
//!    ALUs, the Hamming construction for the ECC circuits, a priority scan for
//!    the interrupt controller. The models know nothing about gates and share
//!    no code with the generator.
//! 2. **The structural-equivalence pair.** `sec32` and `sec32n` are the same
//!    function in two structures — `xor` primitives against NAND gates only —
//!    and must agree on every vector.
//! 3. **Icarus and Verilator.** The `.v` files are ordinary corpus cases, so
//!    the existing oracle arms run them as soon as either binary is installed.
//!
//! # Vectors
//!
//! Deterministic and pinned: corner values, walking ones and zeros, and
//! patterns from a fixed-seed linear congruential generator whose constants
//! are in [`vectors`]. Nothing reads a clock or a thread-local RNG, so the
//! `.stim` files regenerate byte for byte.
//!
//! # The generator is the source of truth
//!
//! `tests/verilog/scale/` is generated output, checked in so the corpus is
//! readable and so the oracle arms have files to hand to a simulator. The
//! determinism test regenerates everything and byte-compares, so editing a
//! vendored file is a failing test rather than a silent divergence.

pub mod cells;
pub mod circuits;
pub mod circuits_wide;
pub mod netlist;
pub mod reference;
pub mod sequential;
pub mod vectors;

use netlist::{Design, Metrics};
use std::path::{Path, PathBuf};

/// Every circuit in the suite, in the order they are reported.
pub const NAMES: [&str; 12] = [
    "intc27",
    "sec32",
    "alu8",
    "sec32n",
    "secded16",
    "alu12c",
    "alu8bcd",
    "alu9d",
    "mul16",
    "addcmp32",
    "lfsr32",
    "pipe_mac8",
];

/// The ten structural circuits, in suite order.
///
/// Rebuilt on each call rather than cached: a builder is cheap, and a cached
/// design would make it possible for one caller's mutation to reach another.
pub fn structural(name: &str) -> Option<Design> {
    Some(match name {
        "intc27" => circuits::intc27(),
        "sec32" => circuits::sec32(circuits::EccStyle::Primitive),
        "sec32n" => circuits::sec32(circuits::EccStyle::NandOnly),
        "alu8" => circuits::alu8(),
        "secded16" => circuits::secded16(),
        "alu12c" => circuits_wide::alu12c(),
        "alu8bcd" => circuits_wide::alu8bcd(),
        "alu9d" => circuits_wide::alu9d(),
        "mul16" => circuits_wide::mul16(),
        "addcmp32" => circuits_wide::addcmp32(),
        _ => return None,
    })
}

/// The Verilog source of one circuit, structural or behavioural.
pub fn verilog(name: &str) -> Option<String> {
    if let Some(design) = structural(name) {
        return Some(netlist::emit(&design));
    }
    sequential::source(name)
}

/// Structural metrics for one circuit, or `None` for the behavioural pair.
pub fn metrics(name: &str) -> Option<Metrics> {
    structural(name).map(|design| netlist::measure(&design))
}

/// Root of the vendored scale corpus.
pub fn scale_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the conformance crate sits two levels below the workspace root")
        .join("tests")
        .join("verilog")
        .join("scale")
}
