//! The GF180MCU suite: a released foundry PDK, evaluated against ngspice.
//!
//! # What this corpus is for
//!
//! Not a new kind of oracle — the reference is ngspice-46, the same authority
//! [`ngspice`](super::ngspice) uses. What is new is the *material*. The
//! vendored ngspice corpus is a regression suite: decks written as tests,
//! against models chosen to be testable. This is GlobalFoundries' production
//! 180nm PDK — subcircuit-wrapped compact models, process corner libraries,
//! statistical switches, characterisation sweeps spanning a dozen decades of
//! current from −40°C to 175°C. It is what a customer's deck actually looks
//! like, and none of it appears in the other corpora.
//!
//! Getting a released PDK to load at all took three ingestion fixes, each of
//! which had made RSpice the only simulator that could not read it: a
//! mislabelled `.ENDL`, model parameters citing a `.param` defined later in
//! the corner-library expansion, and whitespace around `=` in an instance
//! parameter list. That is the return this corpus pays.
//!
//! # A correction worth recording
//!
//! This suite was first built against the spreadsheet columns shipped in
//! `180MCU_SPICE_DATA`, on the reading that they are the measured silicon the
//! upstream harness compares to. They are not. The columns are named after
//! corner libraries (`diode_typical`, `diode_ff`, `diode_ss`) and reproduce
//! ngspice's reverse-bias answer on the same deck to four significant
//! figures: they are simulation output, captured at each corner. Upstream's
//! own driver prints its comparison rather than asserting on it, which is
//! consistent with nobody having relied on the distinction.
//!
//! Building a gate on data whose provenance could not be established would
//! have been the one thing a conformance suite must never do, so the
//! reference here is ngspice's answer on the exact vendored deck — generated
//! once at vendoring time and checked in beside it, the same arrangement the
//! ngspice corpus uses for its `.out` captures.
//!
//! # Tolerance
//!
//! Because both sides evaluate the same model from the same card, agreement
//! should be tight — sub-percent — and the default bound says so. That makes
//! this a genuinely sensitive instrument: a defect in parameter binding,
//! temperature scaling, or corner selection moves these curves far more than
//! the bound allows. It is a much stronger claim than the loose
//! model-versus-silicon band the original reading would have supported.
//!
//! Comparison skips points below a floor set relative to each curve's own
//! peak, because a diode sweep crosses zero and relative error against a
//! reference value that is itself at the numerical floor is noise.
//!
//! A sub-percent bound only means something because every deck disables the
//! PDK's statistical mismatch. Left at the PDK default, the device wrappers
//! apply a random per-instance `delvto` worth roughly ±19% of subthreshold
//! drain current, and ngspice does not fix its seed — it answers differently
//! on consecutive runs of the same deck, so the reference would be a draw
//! nothing could reproduce. See `tests/gf180mcu/RSPICE-VENDORING.md`.
//!
//! # What is covered
//!
//! MOSFET I-V across five device flavours, four geometries, four temperatures
//! and six gate biases — 612 cases. Diode I-V adds 216 more, covering the
//! PDK's nine junction diodes across three corners, two geometries and four
//! temperatures; every one of those cards is `d level = 3`, so this group is
//! what holds RSpice's ngspice-native geometric diode — its tunneling branch
//! and TLEV/TLEVC temperature families in particular — to the reference.
//!
//! See `tests/gf180mcu/RSPICE-VENDORING.md` for the rest, including the C-V
//! measures, which probe device internal capacitance through an ngspice
//! `@device[param]` form this suite does not yet express.

use rspice_core::engine::{ConvergenceConfig, SimulationConfig};
use rspice_core::{Engine, Netlist};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

mod compare;
mod discovery;
mod manifest;

pub use compare::{DeviceMismatch, compare_series};
pub use manifest::DeviceContract;

use super::deadline::DeadlineAbort;

// ═══════════════════════════════════════════════════════════════════════════════
// Results
// ═══════════════════════════════════════════════════════════════════════════════

/// The outcome of one device case.
#[derive(Debug, Clone)]
pub struct DeviceResult {
    /// Case name, which is the manifest key and the deck stem.
    pub name: String,
    /// Whether the case met its contract.
    pub passed: bool,
    /// The contract the case was judged against.
    pub contract: DeviceContract,
    /// Worst relative deviation from the reference curve, as a percentage.
    pub worst_error_pct: f64,
    /// The bound this case had to stay within, as a percentage.
    pub allowed_pct: f64,
    /// Reference-side disagreement recorded for this case, as a percentage.
    pub reference_pct: f64,
    /// Where the worst deviation occurred.
    pub worst: Option<DeviceMismatch>,
    /// Why the case failed, when it did.
    pub error: Option<String>,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: u128,
}

/// Aggregate statistics for a suite run.
#[derive(Debug, Clone, Default)]
pub struct DeviceStatistics {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub reference_only: usize,
    pub total_time_ms: u128,
    /// Worst relative deviation seen across every compared case.
    pub worst_error_pct: f64,
}

impl DeviceStatistics {
    pub fn collect(results: &[DeviceResult]) -> Self {
        let mut stats = Self::default();
        for result in results {
            stats.total += 1;
            stats.total_time_ms += result.duration_ms;
            if result.passed {
                stats.passed += 1;
            } else {
                stats.failed += 1;
            }
            if matches!(result.contract, DeviceContract::ReferenceOnly) {
                stats.reference_only += 1;
            } else {
                stats.worst_error_pct = stats.worst_error_pct.max(result.worst_error_pct);
            }
        }
        stats
    }

    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.passed as f64 / self.total as f64 * 100.0
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

/// Runner configuration.
#[derive(Debug, Clone)]
pub struct DeviceConfig {
    /// Multiplier applied to any reference-side disagreement a case records.
    ///
    /// Zero for a captured reference, which agrees with itself, so this only
    /// bites for a corpus whose reference is not self-consistent.
    pub reference_margin: f64,
    /// The bound every case must meet, as a percentage.
    ///
    /// Both sides evaluate the same model card, so this is tight by design:
    /// the point of the suite is that a defect in parameter binding,
    /// temperature scaling, or corner selection moves these curves far more
    /// than a percent.
    pub minimum_tolerance_pct: f64,
    /// Reference points below this fraction of their curve peak are skipped.
    ///
    /// A diode I-V sweep crosses zero and spans a dozen decades. Near the
    /// crossing the reference is at the numerical floor of the run that
    /// produced it, and relative error against noise manufactures enormous
    /// disagreements from physically identical currents.
    pub measurement_floor: f64,
    /// Time budget per case, in milliseconds.
    pub max_time_per_case_ms: u128,
    /// Emit a line per case as it runs.
    pub verbose: bool,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            reference_margin: 1.5,
            minimum_tolerance_pct: 1.0,
            measurement_floor: 1e-6,
            max_time_per_case_ms: 60_000,
            verbose: false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Runner
// ═══════════════════════════════════════════════════════════════════════════════

/// Runs the GF180MCU device corpus against its vendored references.
pub struct DeviceRunner {
    config: DeviceConfig,
    root: PathBuf,
    manifest: HashMap<String, manifest::ManifestEntry>,
}

impl DeviceRunner {
    /// Create a runner rooted at the workspace `tests/` directory.
    pub fn new(tests_dir: &Path, config: DeviceConfig) -> Self {
        let root = tests_dir.join("gf180mcu");
        let root = root.canonicalize().unwrap_or(root);
        let manifest = manifest::load(&root);
        Self {
            config,
            root,
            manifest,
        }
    }

    pub fn config(&self) -> &DeviceConfig {
        &self.config
    }

    /// The vendored corpus root.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Manifest keys with no deck behind them.
    pub fn orphaned_manifest_entries(&self) -> Vec<String> {
        let mut orphans: Vec<String> = self
            .manifest
            .keys()
            .filter(|key| !self.deck_path(key).is_file())
            .cloned()
            .collect();
        orphans.sort();
        orphans
    }

    /// Engine used for every case.
    ///
    /// Robust convergence, and no temperature of its own: each case is a
    /// characterisation point at a stated temperature, the deck says which
    /// with a `.temp` card, and the engine resolves that card itself. A
    /// suite-level temperature would override the card and silently compare
    /// every case against the wrong measured curve.
    fn engine(&self) -> Engine {
        let defaults = SimulationConfig::default();
        Engine::new(SimulationConfig {
            max_iterations: defaults.max_iterations.max(1200),
            convergence_config: ConvergenceConfig::robust(),
            // The dialect is left at its default rather than pinned to
            // `Ngspice`, which the sibling suites do. GF180MCU's MOS cards are
            // binned across dotted names (`nmos_3p3.0`, `.1`, …) selected by
            // W and L, and upstream reaches them through ngspice's HSPICE
            // compatibility mode — `set ngbehavior=hs` in the PDK's
            // `.spiceinit`. Pinning the ngspice dialect here is *not* that
            // mode, and it costs the binning: every MOS instance falls back
            // to looking for a plain `nmos_3p3` model card and fails to
            // resolve. The default dialect selects the bins correctly.
            ..defaults
        })
    }

    /// Run every discovered case.
    pub fn run_corpus(&self) -> Vec<DeviceResult> {
        self.discover()
            .into_iter()
            .map(|case| {
                let result = self.run_case(&case);
                if self.config.verbose {
                    eprintln!(
                        "  {:<7} {case}  {:.2}% (ngspice {:.2}%, allowed {:.2}%)",
                        if result.passed { "ok" } else { "FAILED" },
                        result.worst_error_pct,
                        result.reference_pct,
                        result.allowed_pct
                    );
                }
                result
            })
            .collect()
    }

    /// Run one case and judge it against its contract.
    pub fn run_case(&self, case: &str) -> DeviceResult {
        let start = Instant::now();
        let entry = self.manifest.get(case).copied().unwrap_or_default();
        let allowed = (entry.reference_pct * self.config.reference_margin)
            .max(self.config.minimum_tolerance_pct);

        let mut result = DeviceResult {
            name: case.to_string(),
            passed: false,
            contract: entry.contract,
            worst_error_pct: 0.0,
            allowed_pct: allowed,
            reference_pct: entry.reference_pct,
            worst: None,
            error: None,
            duration_ms: 0,
        };

        match self.evaluate(case, allowed) {
            Ok((worst_pct, worst)) => {
                result.worst_error_pct = worst_pct;
                result.worst = worst;
                result.passed = match entry.contract {
                    // No comparable reference: required to run, nothing more.
                    DeviceContract::ReferenceOnly => true,
                    // Running at all makes the recorded gap stale.
                    DeviceContract::ExpectedUnsupported => false,
                    DeviceContract::Ngspice => worst_pct <= allowed,
                };
                if !result.passed {
                    result.error = Some(match entry.contract {
                        DeviceContract::ExpectedUnsupported => String::from(
                            "case is recorded as exercising an unimplemented device \
                             model, but it ran and compared — promote the manifest row \
                             to a real comparison",
                        ),
                        _ => format!(
                            "worst deviation from the reference curve {worst_pct:.2}% \
                             exceeds the {allowed:.2}% allowed for this case"
                        ),
                    });
                }
            }
            Err(error) => {
                // A recorded gap is *supposed* to fail here, and the failure
                // is the evidence the row is still accurate.
                if matches!(entry.contract, DeviceContract::ExpectedUnsupported) {
                    result.passed = true;
                }
                result.error = Some(error);
            }
        }

        result.duration_ms = start.elapsed().as_millis();
        result
    }
}
