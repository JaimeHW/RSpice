//! Ordinary-noise hardcopy bindings use the sheet's offering and canonical trace rules.

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
