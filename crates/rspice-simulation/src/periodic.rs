//! Periodic analysis configuration, validation and engine lowering.

mod hb;
mod pac;
mod pnoise;
mod pss;
mod pstb;
mod pxf;
mod worker;

pub use hb::{HbRunConfig, HbToneRunConfig, build_core_hb_config};
pub use pac::PacRunConfig;
pub(crate) use pac::core_pac_sweep;
pub use pnoise::{PnoiseReference, PnoiseRunConfig, PnoiseRunError};
pub use pss::{PssRunConfig, build_core_pss_config, validate_pss_config};
pub use pstb::PstbRunConfig;
pub use pxf::PxfRunConfig;
// The sweep field type of the run configurations above.
pub use rspice_simulation_contract::config::FrequencySweep;

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
