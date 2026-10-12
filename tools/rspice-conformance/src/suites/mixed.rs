//! The mixed-signal reference corpus (`tests/mixed-corpus/`).
//!
//! Twelve designs, each mixing SPICE devices, XSPICE code models and
//! Verilog-AMS modules, run every analysis the plan lists for them and are
//! compared against oracles with tolerances written down before the first
//! comparison. The outcome of every design × variant × analysis × route cell is
//! recorded in the design's `outcomes.tsv`, and the run fails on any cell that
//! moved without being re-recorded. That table is the work queue for the mixed
//! simulation plan.
//!
//! # Execution
//!
//! A case runs through `rspice_simulation`'s headless preparation
//! ([`rspice_simulation::execution::prepare_headless_run`]) and its runner: the
//! code the desktop app, the browser worker and `rspice study` dispatch. The
//! deck's analysis card becomes the task graph; the rest of the deck is the
//! circuit. HDL sources a deck names are compiled and sealed exactly as the
//! application seals project Verilog-A, so they reach the engine as prepared
//! runtimes rather than ambient files. Only an analysis kind the headless
//! contract cannot express falls back to the [`rspice_core::Engine`] entry
//! (`case-result.json` then records `dispatch = "engine"`); [`dispatch`] lists
//! which and why.
//!
//! The route is the build: the default features are the interpreter route,
//! `--features veriloga-native` the native JIT route. There is no runtime
//! switch, so each route is one build of `rspice-mixed-corpus`.
//!
//! # Process isolation
//!
//! `run` spawns one `case` child per variant × analysis with the design's
//! budget; the child writes `case-result.json` (schema `rspice-mixed-case/1`).
//! A missing or unreadable result is a crash.
//!
//! # Comparison
//!
//! Quantities (`tolerance.toml`) are measures, never raw samples: an oracle's
//! grid is its own. RSpice and the oracle are measured by the same code
//! ([`measure`]) and compared with `|a − e| ≤ abs + rel·|e|` ([`compare`]).
//! Captured oracles (ngspice O1, Xyce O4, the held-digital O5 decks run by
//! either) are checked-in tables under `ref/`, each bound to the BLAKE3 of the
//! oracle deck it came from. O5 itself (RSpice on the held-digital deck) and the
//! analytic O6 expectations ([`analytic`]) are evaluated live.
//!
//! # Classification
//!
//! [`classify`] decides `pass`, `fail`, `refused`, `wrong` or `unverified` in
//! that order of evaluation. A refusal counts only when it names a capability
//! token, a reason, the instance and the analysis; a refusal missing one of them
//! is `wrong(malformed-refusal)`, and one naming a different analysis is
//! `wrong(misnamed-analysis)`.

pub mod analytic;
pub mod case;
pub mod classify;
pub mod compare;
pub mod design;
pub mod dispatch;
pub mod events;
pub mod measure;
pub mod oracle;
pub mod outcomes;
pub mod probes;
pub mod runner;

use std::path::{Path, PathBuf};

/// Which build of the simulator produced a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Route {
    /// Bytecode-VM Verilog-A (`rspice-core/veriloga`).
    Interp,
    /// x64/AArch64 native JIT Verilog-A with no interpreter fallback.
    Native,
    /// The browser worker (P4.7), as an external producer.
    Worker,
    /// The desktop application (P4), as an external producer.
    App,
}

impl Route {
    /// The route this binary was compiled as.
    pub const fn compiled() -> Self {
        if cfg!(feature = "veriloga-native") {
            Self::Native
        } else {
            Self::Interp
        }
    }

    pub const fn token(self) -> &'static str {
        match self {
            Self::Interp => "interp",
            Self::Native => "native",
            Self::Worker => "worker",
            Self::App => "app",
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "interp" => Some(Self::Interp),
            "native" => Some(Self::Native),
            "worker" => Some(Self::Worker),
            "app" => Some(Self::App),
            _ => None,
        }
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.token())
    }
}

/// The five outcomes of one cell, best first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Outcome {
    Pass,
    Unverified,
    Refused,
    Fail,
    Wrong,
}

impl Outcome {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Unverified => "unverified",
            Self::Refused => "refused",
            Self::Fail => "fail",
            Self::Wrong => "wrong",
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "pass" => Some(Self::Pass),
            "unverified" => Some(Self::Unverified),
            "refused" => Some(Self::Refused),
            "fail" => Some(Self::Fail),
            "wrong" => Some(Self::Wrong),
            _ => None,
        }
    }

    /// Rank in the ratchet order pass > unverified > refused > fail > wrong;
    /// higher is better.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Pass => 4,
            Self::Unverified => 3,
            Self::Refused => 2,
            Self::Fail => 1,
            Self::Wrong => 0,
        }
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.token())
    }
}

/// `tests/mixed-corpus/` at the workspace root.
pub fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus")
        .components()
        .collect()
}

/// The workspace root, for `rspice_rev` and repository-relative paths.
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .components()
        .collect()
}
