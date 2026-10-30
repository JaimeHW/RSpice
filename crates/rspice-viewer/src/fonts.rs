//! The faces a hydrated scene is lettered in.
//!
//! A drawing sheet is laid out against IBM Plex: its character budgets, cap
//! heights and baselines are measurements of those faces, and its title
//! block sets the document identifier apart in the Medium cut. The viewer
//! installs the faces and fallback chains of `rspice-fonts`, the ones the
//! product paints and prints with, so each line letters in the face and
//! weight the contract names, a symbol Plex lacks letters from RSpice
//! Symbols, and a character no face has letters as the same square.
//! The faces are licensed under the SIL Open Font License
//! (see `assets/fonts/OFL-IBMPlex.txt` and `assets/fonts/OFL-RSpiceSymbols.txt`).

use egui::{Context, FontDefinitions, FontId};
use rspice_fonts::Role;
use rspice_publication_contract::TextFont;

/// Every face `rspice-fonts` embeds, and one family per role. egui's own
/// default faces are not installed: a sheet is never lettered in them.
#[must_use]
pub fn font_definitions() -> FontDefinitions {
    rspice_fonts::egui_font_definitions()
}

/// Install the scene faces into a context. egui rebuilds its glyph atlas on
/// every call, so this runs once, when the runtime starts.
pub fn install(ctx: &Context) {
    ctx.set_fonts(font_definitions());
}

/// The role a contract text line is lettered in.
const fn role(font: TextFont) -> Role {
    match font {
        TextFont::Sans => Role::Sans,
        TextFont::SansSemibold => Role::SansSemibold,
        TextFont::Monospace => Role::Mono,
        TextFont::MonospaceMedium => Role::MonoMedium,
    }
}

/// The font a contract text line is lettered in at `size` points.
#[must_use]
pub fn text_font(font: TextFont, size: f32) -> FontId {
    FontId::new(size, role(font).egui_family())
}

#[cfg(test)]
mod tests {
    use rspice_fonts::Face;

    use super::*;

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
    fn each_contract_font_letters_from_its_own_cut() {
        let fonts = font_definitions();
        for (font, family, weight) in [
            (TextFont::Sans, "IBM Plex Sans", 400),
            (TextFont::SansSemibold, "IBM Plex Sans", 600),
            (TextFont::Monospace, "IBM Plex Mono", 400),
            (TextFont::MonospaceMedium, "IBM Plex Mono", 500),
        ] {
            let chain = &fonts.families[&text_font(font, 10.0).family];
            let face = Face::ALL
                .into_iter()
                .find(|face| face.key() == chain[0])
                .expect("a family starts with an embedded face");
            assert_eq!(face.family(), family, "{font:?}");
            assert_eq!(
                weight_class(&fonts.font_data[&chain[0]].font),
                weight,
                "{font:?} must letter from a face of weight {weight}, not fall back"
            );
            // A symbol Plex lacks, and the replacement square, letter from
            // RSpice Symbols at the end of every chain.
            assert_eq!(chain.last().map(String::as_str), Some(Face::Symbols.key()));
        }
        // egui's default faces are not installed.
        assert_eq!(fonts.font_data.len(), Face::ALL.len());
    }
}
