//! Embedded font setup.
//!
//! The design system uses IBM Plex Sans for UI text and IBM Plex Mono for
//! data (values, netlists, readouts). Both are embedded so the application
//! renders identically on every machine — no system-font fallback drift.
//! The faces are licensed under the SIL Open Font License
//! (see `assets/fonts/OFL-IBMPlex.txt`).
//!
//! egui has no variable-weight axis, so the Medium and SemiBold weights are
//! registered as named [`FontFamily`] entries and selected explicitly by
//! widgets that need emphasis.

use egui::{Context, FontData, FontDefinitions, FontFamily};

/// Family name for medium-weight UI text (500).
const SANS_MEDIUM: &str = "plex-sans-medium";
/// Family name for semibold UI text (600).
const SANS_SEMIBOLD: &str = "plex-sans-semibold";
/// Family name for medium-weight mono text (500).
const MONO_MEDIUM: &str = "plex-mono-medium";

/// Proportional UI family (regular weight).
pub fn sans() -> FontFamily {
    FontFamily::Proportional
}

/// Monospace data family (regular weight).
pub fn mono() -> FontFamily {
    FontFamily::Monospace
}

// The named families are constructed once: `FontFamily::Name` holds an
// `Arc<str>`, and these getters run for nearly every widget every frame —
// cloning the Arc beats re-allocating the name each call.
static SANS_MEDIUM_FAMILY: std::sync::LazyLock<FontFamily> =
    std::sync::LazyLock::new(|| FontFamily::Name(SANS_MEDIUM.into()));
static SANS_SEMIBOLD_FAMILY: std::sync::LazyLock<FontFamily> =
    std::sync::LazyLock::new(|| FontFamily::Name(SANS_SEMIBOLD.into()));
static MONO_MEDIUM_FAMILY: std::sync::LazyLock<FontFamily> =
    std::sync::LazyLock::new(|| FontFamily::Name(MONO_MEDIUM.into()));

/// Medium-weight (500) UI family — labels, buttons, emphasized values.
pub fn sans_medium() -> FontFamily {
    SANS_MEDIUM_FAMILY.clone()
}

/// SemiBold (600) UI family — section headers, brand, headings.
pub fn sans_semibold() -> FontFamily {
    SANS_SEMIBOLD_FAMILY.clone()
}

/// Medium-weight (500) mono family — emphasized data values.
pub fn mono_medium() -> FontFamily {
    MONO_MEDIUM_FAMILY.clone()
}

/// Build the application font definitions from RSpice's pinned IBM Plex faces.
///
/// The direct `egui` dependency deliberately disables its default-font
/// feature. Shipping Ubuntu, Hack, and two emoji fonts behind these complete
/// product faces added more than a megabyte to every browser image and made
/// typography depend on an unused fallback set.
pub fn font_definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "plex-sans".to_owned(),
        FontData::from_static(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/fonts/IBMPlexSans-Regular.ttf"
        )))
        .into(),
    );
    fonts.font_data.insert(
        SANS_MEDIUM.to_owned(),
        FontData::from_static(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/fonts/IBMPlexSans-Medium.ttf"
        )))
        .into(),
    );
    fonts.font_data.insert(
        SANS_SEMIBOLD.to_owned(),
        FontData::from_static(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/fonts/IBMPlexSans-SemiBold.ttf"
        )))
        .into(),
    );
    fonts.font_data.insert(
        "plex-mono".to_owned(),
        FontData::from_static(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/fonts/IBMPlexMono-Regular.ttf"
        )))
        .into(),
    );
    fonts.font_data.insert(
        MONO_MEDIUM.to_owned(),
        FontData::from_static(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/fonts/IBMPlexMono-Medium.ttf"
        )))
        .into(),
    );

    // Prepend our faces so they take priority; egui defaults stay as fallback.
    let proportional = fonts.families.entry(FontFamily::Proportional).or_default();
    proportional.insert(0, "plex-sans".to_owned());

    // Plex Mono carries no Greek: Δ, Ω and μ would letter as a replacement
    // box. Plex Sans carries them, so the mono families fall back to it
    // before egui's defaults.
    let monospace = fonts.families.entry(FontFamily::Monospace).or_default();
    monospace.insert(0, "plex-mono".to_owned());
    monospace.insert(1, "plex-sans".to_owned());

    // Weighted families fall back to the regular cut, then egui defaults.
    let base_proportional = fonts.families[&FontFamily::Proportional].clone();
    let mut base_monospace = fonts.families[&FontFamily::Monospace].clone();
    // A medium mono run borrows from the medium sans, so a borrowed glyph
    // keeps its weight.
    base_monospace.insert(1, SANS_MEDIUM.to_owned());

    let mut sans_medium_stack = vec![SANS_MEDIUM.to_owned()];
    sans_medium_stack.extend(base_proportional.iter().cloned());
    fonts
        .families
        .insert(FontFamily::Name(SANS_MEDIUM.into()), sans_medium_stack);

    let mut sans_semibold_stack = vec![SANS_SEMIBOLD.to_owned()];
    sans_semibold_stack.extend(base_proportional.iter().cloned());
    fonts
        .families
        .insert(FontFamily::Name(SANS_SEMIBOLD.into()), sans_semibold_stack);

    let mut mono_medium_stack = vec![MONO_MEDIUM.to_owned()];
    mono_medium_stack.extend(base_monospace.iter().cloned());
    fonts
        .families
        .insert(FontFamily::Name(MONO_MEDIUM.into()), mono_medium_stack);

    fonts
}

/// Install the embedded fonts into an egui context. Idempotent and cheap to
/// guard with a once-flag at the call site; egui rebuilds its glyph atlas on
/// every call, so avoid calling this per-frame.
pub fn install(ctx: &Context) {
    ctx.set_fonts(font_definitions());
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // Measured against egui's own default set rather than as a bare
        // count: this crate disables egui's `default_fonts`, but a workspace
        // build unifies features with crates that keep them (rspice-viewer),
        // and then `FontDefinitions::default()` already carries egui's four
        // faces. What this crate owns is that it adds exactly its five
        // governed IBM Plex faces on top of whatever egui provides.
        let egui_default_faces = FontDefinitions::default().font_data.len();
        assert_eq!(
            fonts.font_data.len(),
            egui_default_faces + 5,
            "the product embeds exactly its five governed IBM Plex faces"
        );
        for face in [
            "plex-sans",
            SANS_MEDIUM,
            SANS_SEMIBOLD,
            "plex-mono",
            MONO_MEDIUM,
        ] {
            assert!(fonts.font_data.contains_key(face), "missing face {face}");
        }
        for family in [sans_medium(), sans_semibold(), mono_medium()] {
            assert!(
                fonts.families.contains_key(&family),
                "missing family {family:?}"
            );
        }
        // Plex must be the first face on the default families.
        assert_eq!(fonts.families[&sans()][0], "plex-sans");
        assert_eq!(fonts.families[&mono()][0], "plex-mono");
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
    /// point, which no face carries (egui's icon font fills part of the
    /// first private-use block) and so letters exactly that replacement.
    pub(crate) fn letters(glyph: &str, font: egui::FontId) -> bool {
        let painted = ink(glyph, font.clone());
        let blank = ink(" ", font.clone());
        painted != blank && painted != ink("\u{10fffd}", font)
    }

    /// Plex Mono has no Greek. The mono families borrow Δ, Ω and μ from Plex
    /// Sans, at the matching weight, instead of lettering a replacement box.
    #[test]
    fn the_mono_families_letter_greek_from_plex_sans() {
        use crate::theme::{FontWeight, mono as mono_font};
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
        assert_eq!(fonts.families[&mono()][..2], ["plex-mono", "plex-sans"]);
        assert_eq!(
            fonts.families[&mono_medium()][..3],
            [MONO_MEDIUM, "plex-mono", SANS_MEDIUM]
        );
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
        use crate::theme::{FontWeight, mono as mono_font, sans as sans_font};
        let fonts = font_definitions();
        for (family, weight) in [
            (sans_font(10.0, FontWeight::Regular).family, 400),
            (sans_font(10.0, FontWeight::Medium).family, 500),
            (sans_font(10.0, FontWeight::SemiBold).family, 600),
            (mono_font(10.0, FontWeight::Regular).family, 400),
            (mono_font(10.0, FontWeight::Medium).family, 500),
        ] {
            let face = &fonts.families[&family][0];
            assert_eq!(
                weight_class(&fonts.font_data[face].font),
                weight,
                "{family:?} must letter from a face of weight {weight}"
            );
        }
    }
}
