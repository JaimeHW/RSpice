//! C5 — LDO with a serially loaded trim DAC (design §3 C5).
//!
//! Every C5 quantity compares against the live held-digital O5 run or a
//! captured O5-O1/O5-O4 reference; the design states no analytic expectation.

use super::Registry;

pub const REGISTRY: Registry = &[];
