//! Harmonic-balance configuration for one authored `.HB` card.
//!
//! `HbConfig::new` and `HbConfig::multi_tone` are the core constructors and
//! carry the core defaults, including the default harmonic order and the
//! common-basis rule for a multi-tone spectrum. This module only turns the
//! card's tone list into one of those two calls.
//!
//! It deliberately does not implement `.OPTIONS HBINT NUMFREQ`. Resolving an
//! authored harmonic-order list against a tone list -- broadcasting a single
//! order, selecting the minimal `2*N+1` collocation grid an explicit `NUMFREQ`
//! asks for -- is analysis semantics that `rspice-core` does not expose, and
//! inventing a second interpretation here is exactly the drift the shared
//! planning contract exists to prevent. A deck that sets it is refused by name.

use rspice_core::Netlist;
use rspice_core::analysis::{HbConfig, HbTone};

use crate::DetailedWasmResult;
use crate::errors::WasmError;

/// Resolve one authored `.HB` tone list into a core configuration.
///
/// The deck runner retains the result after the run, because `.ENVELOPE`
/// continues the same carrier and must be given the identical configuration
/// rather than a second one rebuilt from the same card.
pub(crate) fn hb_config_for_tones(
    netlist: &Netlist,
    frequencies: &[f64],
) -> DetailedWasmResult<HbConfig> {
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
    match frequencies {
        [] => Err(Box::new(WasmError::invalid_argument(
            ".HB requires at least one tone frequency".to_owned(),
        ))),
        [single] => Ok(HbConfig::new(*single)),
        tones => {
            // The default harmonic order is core's, read back from the
            // single-tone constructor rather than restated here.
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
            Ok(HbConfig::multi_tone(resolved))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn netlist(cards: &str) -> Netlist {
        Netlist::parse(&format!(
            "hb configuration deck\nV1 in 0 SIN(0 0.1 1G)\nR1 in out 1k\nC1 out 0 1p\n{cards}.END\n"
        ))
        .expect("the HB deck parses")
    }

    /// A single tone becomes the core single-tone configuration, keeping the
    /// core default harmonic order rather than a browser-chosen one.
    #[test]
    fn one_tone_uses_the_core_single_tone_configuration() {
        let deck = netlist(".HB 1G\n");
        let config = hb_config_for_tones(&deck, &[1.0e9]).expect("a one-tone .HB resolves");
        assert_eq!(config.fundamental_freq, 1.0e9);
        assert_eq!(config.num_harmonics, HbConfig::new(1.0e9).num_harmonics);
        assert!(config.tones.is_empty());
    }

    /// Several tones become the core multi-tone configuration, so the common
    /// spectral basis is core's rule and not one invented here.
    #[test]
    fn several_tones_use_the_core_common_basis_rule() {
        let deck = netlist(".HB 900MEG 800MEG\n");
        let config = hb_config_for_tones(&deck, &[9.0e8, 8.0e8]).expect("a two-tone .HB resolves");
        let expected = HbConfig::multi_tone(vec![
            HbTone::new(9.0e8, HbConfig::new(9.0e8).num_harmonics).with_name("tone1"),
            HbTone::new(8.0e8, HbConfig::new(8.0e8).num_harmonics).with_name("tone2"),
        ]);
        assert_eq!(config.fundamental_freq, expected.fundamental_freq);
        assert_eq!(config.num_harmonics, expected.num_harmonics);
        assert_eq!(config.tones.len(), 2);
    }

    /// An impossible tone list fails before any solve.
    #[test]
    fn impossible_tone_lists_fail_closed() {
        let deck = netlist(".HB 1G\n");
        for tones in [vec![], vec![0.0], vec![f64::NAN], vec![1.0e9, 1.0e9]] {
            let error = *hb_config_for_tones(&deck, &tones)
                .expect_err("an impossible .HB tone list must fail closed");
            assert_eq!(error.code, "invalid_argument");
            assert_eq!(error.category, "input_validation");
        }
    }

    /// An authored harmonic-order list is refused by name rather than being
    /// reinterpreted here.
    #[test]
    fn an_authored_harmonic_order_list_is_refused_by_name() {
        let deck = netlist(".OPTIONS HBINT NUMFREQ=5\n.HB 1G\n");
        let error = *hb_config_for_tones(&deck, &[1.0e9])
            .expect_err("NUMFREQ has no browser resolver and must be refused");
        assert_eq!(error.code, "unsupported_deck_analysis");
        assert_eq!(error.category, "unsupported_feature");
        assert!(
            error.message.contains("NUMFREQ"),
            "the refusal names the option it cannot resolve: {}",
            error.message
        );
    }
}
