//! Which retained ordinary-noise spectrum a hardcopy is allowed to print.
//!
//! The offering and the binding are the sheet's own, re-exported here from
//! `result_document`: an offering that disagrees with the screen is the defect
//! this module exists to prevent, and two copies kept in step by a comment is
//! not a way to prevent it. What remains local is the naming vocabulary the
//! resolver reads a retained trace back through.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RetainedNoiseReference {
    Input,
    Output,
}

pub(super) fn retained_noise_reference(name: &str) -> Option<RetainedNoiseReference> {
    let name = name
        .trim()
        .to_ascii_lowercase()
        .replace([' ', '-', '.'], "");
    if matches!(
        name.as_str(),
        "inoise"
            | "inoise_spectrum"
            | "inoisespectrum"
            | "v(inoise)"
            | "v(inoise_spectrum)"
            | "v(inoisespectrum)"
    ) {
        Some(RetainedNoiseReference::Input)
    } else if matches!(
        name.as_str(),
        "onoise"
            | "onoise_spectrum"
            | "onoisespectrum"
            | "v(onoise)"
            | "v(onoise_spectrum)"
            | "v(onoisespectrum)"
    ) {
        Some(RetainedNoiseReference::Output)
    } else {
        None
    }
}

pub(super) fn retained_noise_contributor(name: &str) -> bool {
    let name = name.trim().to_ascii_lowercase();
    name.starts_with("noise(") && name.ends_with(')')
}

pub(super) fn retained_noise_waveform_is_renderable(waveform: &WaveformData) -> bool {
    if waveform.x.len() != waveform.y.len() || waveform.x.len() < 2 {
        return false;
    }
    if waveform
        .y
        .iter()
        .any(|density| !density.is_finite() || *density <= 0.0)
    {
        return false;
    }
    let mut previous = None;
    for frequency in waveform.x.iter().copied() {
        if !frequency.is_finite()
            || frequency <= 0.0
            || previous.is_some_and(|previous| frequency <= previous)
        {
            return false;
        }
        previous = Some(frequency);
    }
    true
}

pub(super) use crate::workbench::documents::result_document::{
    ordinary_noise_spectrum_is_renderable, selected_noise_analysis_index,
};

#[cfg(test)]
mod tests {
    /// The page reads the sheet's own offering predicates.
    ///
    /// "Mirrors `result_document::bode`" is a comment, and a comment cannot
    /// keep two copies in step: the page and the screen can disagree about
    /// which analysis a reader selected, and the disagreement shows up on
    /// paper under the selected analysis's name. The privacy that forced the
    /// copy is the thing to widen.
    #[test]
    fn the_offering_predicates_are_the_sheets_own_and_not_a_copy_of_them() {
        let shipped = crate::source_guard::without_test_items(include_str!("noise.rs"));
        for mirrored in [
            "fn ordinary_noise_spectrum_is_renderable",
            "fn selected_noise_analysis_index",
            "fn is_noise_analysis",
        ] {
            assert!(
                !shipped.contains(mirrored),
                "the printed page still carries its own `{mirrored}` instead of \
                 calling the one the sheet reads"
            );
        }
    }
}
