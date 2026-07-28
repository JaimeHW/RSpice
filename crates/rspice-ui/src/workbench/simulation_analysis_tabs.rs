//! Names for the persisted analysis-index space.
//!
//! `SimSetup` stores which analyses a plan contains as bare `usize` indices
//! and bounds them with [`ANALYSIS_COUNT`], so these numbers are on disk in
//! saved projects. The constants below are what those numbers mean. Only a
//! few are named in code today -- `TAB_TRANSIENT` and `ANALYSIS_COUNT` in
//! `app_state::sim_setup`, plus `TAB_AC`/`TAB_NOISE` in tests -- but the rest
//! are not spare parts: they are the decode table for a live encoding, and
//! deleting them would leave `17` in a project file with nothing in the tree
//! saying it is the transfer function.
//!
//! Hence the blanket allow. Its removal condition is the encoding, not the
//! usage count: when plans persist an analysis *kind* rather than a tab index,
//! this module and the allow go together. The Simulate workspace already
//! builds its own catalog from `AnalysisKind`
//! (`workbench::surfaces::simulate::catalog`), so that change is the natural
//! next step rather than a hypothetical.
#![allow(dead_code)]

pub const TAB_DC_OP: usize = 0;
pub const TAB_TRANSIENT: usize = 1;
pub const TAB_AC: usize = 2;
pub const TAB_DC_SWEEP: usize = 3;
pub const TAB_NOISE: usize = 4;
pub const TAB_POLE_ZERO: usize = 5;
pub const TAB_SENSITIVITY: usize = 6;
pub const TAB_MONTE_CARLO: usize = 7;
pub const TAB_PSS: usize = 8;
pub const TAB_STB: usize = 9;
pub const TAB_TEMPERATURE: usize = 10;
pub const TAB_HARMONIC_BALANCE: usize = 11;
pub const TAB_SPARAMETER: usize = 12;
pub const TAB_PAC: usize = 13;
pub const TAB_PNOISE: usize = 14;
pub const TAB_PXF: usize = 15;
pub const TAB_PSTB: usize = 16;
pub const TAB_TRANSFER_FUNCTION: usize = 17;
pub const TAB_CORNER: usize = 18;
pub const TAB_ENVELOPE: usize = 19;
pub const TAB_FOURIER: usize = 20;
pub const TAB_RELIABILITY: usize = 21;
pub const TAB_OPTIMIZATION: usize = 22;
pub const TAB_SOA: usize = 23;
pub const TAB_DISTO: usize = 24;
pub const ANALYSIS_COUNT: usize = TAB_DISTO + 1;

// `SIMULATION_ANALYSIS_CATEGORIES` and `QUICK_RUN_ANALYSES` used to sit here:
// two static tables grouping and labelling these indices for a menu. Unlike
// the constants above they encoded nothing -- they were presentation data with
// no reader. The Simulate workspace groups and labels from `AnalysisKind` in
// `surfaces::simulate::catalog`, which stays in step with the analyses that
// actually run; a second hand-maintained table could only drift out of it.
