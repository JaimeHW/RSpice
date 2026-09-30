//! Periodic analysis configuration, validation and engine lowering.

mod pac;
mod pnoise;
mod pstb;
mod pxf;
mod worker;

pub use pac::{PacFrequencySweep, PacRunConfig};
pub use pnoise::{PnoiseFrequencySweep, PnoiseReference, PnoiseRunConfig, PnoiseRunError};
pub use pstb::PstbRunConfig;
pub use pxf::{PxfFrequencySweep, PxfRunConfig};

/// Normalize a periodic-analysis output name from a node or `V(node)` spelling.
pub fn normalize_pac_node_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 3
        && trimmed
            .get(0..2)
            .map(|prefix| prefix.eq_ignore_ascii_case("V("))
            .unwrap_or(false)
        && trimmed.ends_with(')')
    {
        return trimmed[2..trimmed.len() - 1].trim().to_string();
    }
    trimmed.to_string()
}
