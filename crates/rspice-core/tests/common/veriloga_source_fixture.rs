//! A throwaway directory tree for decks that name a Verilog-A source by a
//! relative path.
//!
//! The tree lives under the system temp directory, well away from the process
//! working directory, so a deck that only resolves because the model happens
//! to sit in the current directory cannot pass by accident.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// A one-ohm-per-thousand resistor, so a deck's answer names the file it read.
pub const RESISTOR_MODULE: &str = r#"`include "disciplines.vams"
module dres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

/// The `r` a module file must declare for [`divider_deck`] to answer `0.5`.
pub const HALF_DIVIDER_OHMS: f64 = 1000.0;

/// A Verilog-A resistor module of an explicit value.
pub fn resistor_module(ohms: f64) -> String {
    RESISTOR_MODULE.replace("1000.0", &format!("{ohms:?}"))
}

/// `V1 - R1 - X1`: one volt across a 1k resistor and the Verilog-A device, so
/// `out` reads `r / (1000 + r)`.
pub fn divider_deck(source_directive: &str) -> String {
    format!(
        "* a deck that names its Verilog-A source by a relative path\n\
         V1 in 0 1\n\
         .include \"vals.cir\"\n\
         X1 out 0 dres\n\
         {source_directive}\n\
         .end\n"
    )
}

/// The `.include`d half of [`divider_deck`], which must also resolve beside
/// the deck.
pub const DIVIDER_VALUES: &str = "* values included beside the deck\nR1 in out 1k\n";

/// The `out` voltage of [`divider_deck`] for a module of `ohms`.
pub fn divider_out(ohms: f64) -> f64 {
    ohms / (1000.0 + ohms)
}

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A unique temporary directory tree, removed when the test ends.
pub struct Fixture(PathBuf);

impl Fixture {
    pub fn new(label: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "rspice_va_source_{label}_{}_{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create the fixture root");
        Self(root)
    }

    pub fn root(&self) -> &Path {
        &self.0
    }

    /// Write one file, creating the directories above it.
    pub fn write(&self, relative: &str, contents: &str) -> PathBuf {
        let path = self.0.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create a fixture directory");
        }
        std::fs::write(&path, contents).expect("write a fixture file");
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
