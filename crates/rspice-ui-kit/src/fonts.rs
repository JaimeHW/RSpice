//! Embedded font setup.
//!
//! The design system uses IBM Plex Sans for UI text and IBM Plex Mono for
//! data (values, netlists, readouts). Both are embedded so the application
//! renders identically on every machine — no system-font fallback drift.
//! The faces, and the chain each family falls back through, come from
//! `rspice-fonts`, which the hardcopy letters through too: a character the
//! canvas borrows from another face prints from that face, and one no face
//! has paints and prints as the same square. A desktop letters what no
//! embedded face has from the faces installed there (`system`), after the
//! whole embedded chain, so every character an embedded face has still
//! letters identically everywhere.
//!
//! egui has no variable-weight axis, so the Medium and SemiBold weights are
//! registered as named [`FontFamily`] entries and selected explicitly by
//! widgets that need emphasis.

use egui::{Context, FontDefinitions, FontFamily};
use rspice_fonts::Role;

#[cfg(not(target_arch = "wasm32"))]
pub mod system;

/// Proportional UI family (regular weight).
pub fn sans() -> FontFamily {
    FontFamily::Proportional
}

/// Monospace data family (regular weight).
pub fn mono() -> FontFamily {
    FontFamily::Monospace
}

/// Medium-weight (500) UI family — labels, buttons, emphasized values.
pub fn sans_medium() -> FontFamily {
    Role::SansMedium.egui_family()
}

/// SemiBold (600) UI family — section headers, brand, headings.
pub fn sans_semibold() -> FontFamily {
    Role::SansSemibold.egui_family()
}

/// Medium-weight (500) mono family — emphasized data values.
pub fn mono_medium() -> FontFamily {
    Role::MonoMedium.egui_family()
}

/// The role a family letters, taking an unknown family as the sans.
pub(crate) fn role_of(family: &FontFamily) -> Role {
    Role::ALL
        .into_iter()
        .find(|role| role.egui_family() == *family)
        .unwrap_or(Role::Sans)
}

/// The application font definitions: every face `rspice-fonts` embeds, and
/// one family per role holding that role's chain.
///
/// The direct `egui` dependency deliberately disables its default-font
/// feature. Shipping Ubuntu, Hack, and two emoji fonts behind these complete
/// product faces added more than a megabyte to every browser image and made
/// typography depend on an unused fallback set.
pub fn font_definitions() -> FontDefinitions {
    rspice_fonts::egui_font_definitions()
}

/// Install the application's fonts into an egui context: every embedded face
/// and, on a desktop, every installed face that has joined its families
/// (`system`). An install never drops a joined face, whoever makes it, so
/// text lettered from one never falls back to the square. Idempotent and
/// cheap to guard with a once-flag at the call site; egui rebuilds its glyph
/// atlas whenever the set changes, so avoid calling this per-frame.
pub fn install(ctx: &Context) {
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(fonts) = system::definitions_with_joined(ctx) {
        ctx.set_fonts(fonts);
        return;
    }
    ctx.set_fonts(font_definitions());
}

#[cfg(test)]
mod tests {
    use rspice_fonts::Face;

    use super::*;
    use crate::theme::{FontWeight, mono as mono_font, sans as sans_font};

    #[test]
    fn embedded_fonts_are_valid_ttf() {
        // TTF magic: 0x00010000 (or "OTTO" for CFF outlines).
        for (name, data) in font_definitions().font_data {
            let head = &data.font[..4];
            assert!(
                head == [0x00, 0x01, 0x00, 0x00] || head == b"OTTO".as_slice(),
                "font {name} is not a valid sfnt container"
            );
        }
    }

    #[test]
    fn weighted_families_are_registered() {
        let fonts = font_definitions();
        // A bare count, in every build: a workspace build unifies features
        // with crates that keep egui's `default_fonts`, and the set must not
        // take on egui's four faces there.
        let embedded = if cfg!(target_arch = "wasm32") { 6 } else { 12 };
        assert_eq!(
            fonts.font_data.len(),
            embedded,
            "the product embeds exactly its five IBM Plex faces and RSpice Symbols,              and on a desktop IBM Plex Sans Arabic, Hebrew and Thai in two weights"
        );
        for face in Face::ALL {
            assert!(
                fonts.font_data.contains_key(face.key()),
                "missing face {face:?}"
            );
        }
        for family in [sans_medium(), sans_semibold(), mono_medium()] {
            assert!(
                fonts.families.contains_key(&family),
                "missing family {family:?}"
            );
        }
        // Plex must be the first face on the default families.
        assert_eq!(fonts.families[&sans()][0], Face::SansRegular.key());
        assert_eq!(fonts.families[&mono()][0], Face::MonoRegular.key());
    }

    /// Every pixel a run of `text` paints in `font`.
    pub(crate) fn ink(text: &str, font: egui::FontId) -> Vec<egui::Color32> {
        let canvas = crate::raster::render(egui::vec2(60.0, 30.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(text)
                            .font(font.clone())
                            .color(egui::Color32::WHITE),
                    );
                });
        });
        canvas
            .pixels_in(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(60.0, 30.0),
            ))
            .collect()
    }

    /// Whether `font` letters `glyph` from a face that carries it. A missing
    /// glyph does not leave a blank: it letters the family's replacement,
    /// which has ink. So the glyph is compared with the last private-use code
    /// point, which no face carries and so letters exactly that replacement.
    pub(crate) fn letters(glyph: &str, font: egui::FontId) -> bool {
        let painted = ink(glyph, font.clone());
        let blank = ink(" ", font.clone());
        painted != blank && painted != ink("\u{10fffd}", font)
    }

    /// Every family the theme letters in, at each weight it offers.
    fn every_font() -> [egui::FontId; 5] {
        [
            sans_font(11.0, FontWeight::Regular),
            sans_font(11.0, FontWeight::Medium),
            sans_font(11.0, FontWeight::SemiBold),
            mono_font(11.0, FontWeight::Regular),
            mono_font(11.0, FontWeight::Medium),
        ]
    }

    /// Plex Mono has no Greek. The mono families borrow Δ, Ω and μ from Plex
    /// Sans, at the matching weight, instead of lettering a replacement box.
    #[test]
    fn the_mono_families_letter_greek_from_plex_sans() {
        for weight in [FontWeight::Regular, FontWeight::Medium] {
            for glyph in ["\u{394}", "\u{3a9}", "\u{3bc}"] {
                assert!(
                    letters(glyph, mono_font(11.0, weight)),
                    "mono {weight:?} letters {glyph} as a replacement"
                );
            }
            assert!(letters("A", mono_font(11.0, weight)), "control glyph");
        }
        let fonts = font_definitions();
        assert_eq!(
            fonts.families[&mono()][..2],
            [Face::MonoRegular.key(), Face::SansRegular.key()]
        );
        assert_eq!(
            fonts.families[&mono_medium()][..3],
            [
                Face::MonoMedium.key(),
                Face::MonoRegular.key(),
                Face::SansMedium.key()
            ]
        );
    }

    /// No Plex face has the diameter sign, the ground symbol or the GD&T
    /// characteristics; every family letters them from RSpice Symbols.
    #[test]
    fn every_family_letters_the_engineering_symbols() {
        for font in every_font() {
            for glyph in [
                "\u{2300}", "\u{2205}", "\u{23da}", "\u{232d}", "\u{2316}", "\u{24c2}",
            ] {
                assert!(
                    letters(glyph, font.clone()),
                    "{font:?} letters {glyph} as a replacement"
                );
            }
        }
    }

    /// A character no face has paints as the white medium square, the same
    /// replacement the hardcopy prints, and not as a question mark.
    #[test]
    fn a_character_no_face_has_paints_as_the_replacement_square() {
        for font in every_font() {
            let uncovered = ink("\u{10fffd}", font.clone());
            assert_eq!(uncovered, ink("\u{25fb}", font.clone()), "{font:?}");
            assert_ne!(uncovered, ink("?", font.clone()), "{font:?}");
            assert_ne!(uncovered, ink(" ", font.clone()), "{font:?}");
        }
    }

    /// The `usWeightClass` an sfnt face's `OS/2` table records.
    fn weight_class(face: &[u8]) -> u16 {
        let read_u16 = |at: usize| u16::from_be_bytes([face[at], face[at + 1]]);
        let read_u32 =
            |at: usize| u32::from_be_bytes([face[at], face[at + 1], face[at + 2], face[at + 3]]);
        let tables = usize::from(read_u16(4));
        let os2 = (0..tables)
            .map(|index| 12 + index * 16)
            .find(|&record| &face[record..record + 4] == b"OS/2")
            .expect("an sfnt face carries an OS/2 table");
        read_u16(read_u32(os2 + 8) as usize + 4)
    }

    #[test]
    fn each_weight_letters_from_a_face_of_that_weight() {
        // A family whose first face is another cut draws every line at that
        // cut's weight without failing anything, so check the face itself.
        let fonts = font_definitions();
        for (font, weight) in every_font().into_iter().zip([400, 500, 600, 400, 500]) {
            let face = &fonts.families[&font.family][0];
            assert_eq!(
                weight_class(&fonts.font_data[face].font),
                weight,
                "{:?} must letter from a face of weight {weight}",
                font.family
            );
        }
    }
}
