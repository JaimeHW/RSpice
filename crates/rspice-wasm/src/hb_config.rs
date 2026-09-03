//! Harmonic-balance configuration for one authored `.HB` card.
//!
//! The resolution itself — the default harmonic order, the multi-tone common
//! basis, and the `.OPTIONS HBINT NUMFREQ` collocation rule — belongs to
//! `rspice_core::analysis::HbConfig::from_hb_card`. This module only hands it
//! the card's tone list and the deck's authored harmonic-order option, and
//! turns its typed failure into a browser error.

use rspice_core::Netlist;
use rspice_core::analysis::HbConfig;

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
    HbConfig::from_hb_card(frequencies, &netlist.options.hb_num_frequencies).map_err(|error| {
        Box::new(WasmError::invalid_argument(format!(
            "invalid .HB card: {error}"
        )))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::analysis::HbTone;

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

    /// An authored harmonic-order list now resolves through the shared core
    /// rule instead of being refused: `NUMFREQ` pins the minimal `2N+1`
    /// collocation grid, which is the Xyce contract.
    #[test]
    fn an_authored_harmonic_order_list_pins_the_core_collocation_grid() {
        let deck = netlist(".OPTIONS HBINT NUMFREQ=5\n.HB 1G\n");
        let config =
            hb_config_for_tones(&deck, &[1.0e9]).expect("an authored NUMFREQ now resolves");
        assert_eq!(config.num_harmonics, 5);
        assert_eq!(config.collocation_points, Some(11));
        assert_eq!(
            config,
            HbConfig::from_hb_card(&[1.0e9], &deck.options.hb_num_frequencies)
                .expect("the core resolver agrees"),
            "the browser adds nothing of its own to the core resolution"
        );
    }

    /// A single authored order is broadcast across every tone by the shared
    /// rule, not by a browser-local loop.
    #[test]
    fn an_authored_order_broadcasts_across_a_multi_tone_card() {
        let deck = netlist(".OPTIONS HBINT NUMFREQ=4\n.HB 900MEG 800MEG\n");
        let config =
            hb_config_for_tones(&deck, &[9.0e8, 8.0e8]).expect("a broadcast order resolves");
        assert_eq!(
            config
                .tones
                .iter()
                .map(|tone| tone.num_harmonics)
                .collect::<Vec<_>>(),
            vec![4, 4]
        );
    }
}
