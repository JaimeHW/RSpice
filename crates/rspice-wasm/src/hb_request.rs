//! Harmonic-balance configuration for one authored `.HB` card.
//!
//! `HbConfig::new` and `HbConfig::multi_tone` are the core constructors and
//! carry the core defaults, including the default harmonic order and the
//! common-basis rule for a multi-tone spectrum. This module only turns the
//! card's tone list into one of those two calls.
//!
//! It deliberately does not implement `.OPTIONS HBINT NUMFREQ`. Resolving an
//! authored harmonic-order list against a tone list — broadcasting a single
//! order, the minimal `2*N+1` collocation grid a explicit `NUMFREQ` selects —
//! is analysis semantics that `rspice-core` does not expose, and inventing a
//! second interpretation here is exactly the drift the shared planning
//! contract exists to prevent. A deck that sets it is refused by name.

use rspice_core::Netlist;
use rspice_core::analysis::{HbConfig, HbTone};

use crate::DetailedWasmResult;
use crate::errors::WasmError;

/// The harmonic-balance configuration one authored `.HB` card resolves to.
///
/// It is retained after the run because `.ENVELOPE` continues the same
/// carrier and must be given the identical configuration.
#[derive(Debug, Clone)]
pub(crate) struct HbRequest {
    pub(crate) config: HbConfig,
}

/// Resolve one authored `.HB` tone list into a core configuration.
pub(crate) fn hb_request_for_tones(
    netlist: &Netlist,
    frequencies: &[f64],
) -> DetailedWasmResult<HbRequest> {
    if !netlist.options.hb_num_frequencies.is_empty() {
        return Err(Box::new(WasmError::new(
            ".OPTIONS HBINT NUMFREQ is not executable by the browser API: rspice-core exposes no resolver from an authored harmonic-order list to an HbConfig collocation grid".to_owned(),
            "unsupported_deck_analysis",
            "unsupported_feature",
        )));
    }
    let mut unique = std::collections::BTreeSet::new();
    for (index, frequency) in frequencies.iter().enumerate() {
        if !frequency.is_finite() || *frequency <= 0.0 {
            return Err(Box::new(WasmError::invalid_argument(format!(
                ".HB tone frequency at index {index} must be positive and finite, got {frequency}"
            ))));
        }
        if !unique.insert(frequency.to_bits()) {
            return Err(Box::new(WasmError::invalid_argument(format!(
                ".HB tone frequency {frequency} is listed more than once"
            ))));
        }
    }
    let config = match frequencies {
        [] => {
            return Err(Box::new(WasmError::invalid_argument(
                ".HB requires at least one tone frequency".to_owned(),
            )));
        }
        [single] => HbConfig::new(*single),
        tones => {
            let default_harmonics = HbConfig::new(tones[0]).num_harmonics;
            let mut resolved = Vec::new();
            resolved.try_reserve_exact(tones.len()).map_err(|_| {
                Box::new(WasmError::new(
                    "could not allocate .HB tone configurations".to_owned(),
                    "result_allocation_failed",
                    "analysis_setup",
                ))
            })?;
            for (index, frequency) in tones.iter().enumerate() {
                resolved.push(
                    HbTone::new(*frequency, default_harmonics)
                        .with_name(format!("tone{}", index + 1)),
                );
            }
            HbConfig::multi_tone(resolved)
        }
    };
    Ok(HbRequest { config })
}
