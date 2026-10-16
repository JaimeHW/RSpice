//! The faces a hydrated scene's monospace text is lettered in.
//!
//! A drawing sheet is laid out against IBM Plex Mono: its character budgets,
//! cap heights and baselines are measurements of that face, and its title
//! block sets the document identifier apart in the Medium cut. egui carries
//! one face per family and has no weight axis, so each cut the contract
//! names is installed as a family of its own; a scene drawn in egui's default
//! monospace would letter every field in another face, at one weight.
//! The faces are licensed under the SIL Open Font License
//! (see `assets/fonts/OFL-IBMPlex.txt`).

use std::sync::LazyLock;

use egui::{Context, FontData, FontDefinitions, FontFamily, FontId};
use rspice_publication_contract::TextFont;

/// Family a contract [`TextFont::Monospace`] line is lettered in (400).
const MONO: &str = "rspice-plex-mono";
/// Family a contract [`TextFont::MonospaceMedium`] line is lettered in (500).
const MONO_MEDIUM: &str = "rspice-plex-mono-medium";

static MONO_FAMILY: LazyLock<FontFamily> = LazyLock::new(|| FontFamily::Name(MONO.into()));
static MONO_MEDIUM_FAMILY: LazyLock<FontFamily> =
    LazyLock::new(|| FontFamily::Name(MONO_MEDIUM.into()));

/// IBM Plex Mono Regular, as the product ships it.
pub const IBM_PLEX_MONO_REGULAR: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/fonts/IBMPlexMono-Regular.ttf"
));
/// IBM Plex Mono Medium, as the product ships it.
pub const IBM_PLEX_MONO_MEDIUM: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/fonts/IBMPlexMono-Medium.ttf"
));

/// egui's own definitions with the two Plex Mono cuts added as families of
/// their own. Each falls back to egui's monospace for a glyph Plex lacks.
#[must_use]
pub fn font_definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let fallback = fonts
        .families
        .get(&FontFamily::Monospace)
        .cloned()
        .unwrap_or_default();
    for (name, data) in [
        (MONO, IBM_PLEX_MONO_REGULAR),
        (MONO_MEDIUM, IBM_PLEX_MONO_MEDIUM),
    ] {
        fonts
            .font_data
            .insert(name.to_owned(), FontData::from_static(data).into());
        let mut stack = vec![name.to_owned()];
        stack.extend(fallback.iter().cloned());
        fonts.families.insert(FontFamily::Name(name.into()), stack);
    }
    fonts
}

/// Install the scene faces into a context. egui rebuilds its glyph atlas on
/// every call, so this runs once, when the runtime starts.
pub fn install(ctx: &Context) {
    ctx.set_fonts(font_definitions());
}

/// The font a contract text line is lettered in at `size` points.
#[must_use]
pub fn text_font(font: TextFont, size: f32) -> FontId {
    match font {
        // Sans text keeps egui's proportional face; only the sheet's
        // monospace lettering is measured against a particular face.
        TextFont::Sans | TextFont::SansSemibold => FontId::proportional(size),
        TextFont::Monospace => FontId::new(size, MONO_FAMILY.clone()),
        TextFont::MonospaceMedium => FontId::new(size, MONO_MEDIUM_FAMILY.clone()),
    }
}

#[cfg(test)]
mod tests {
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
    fn each_monospace_weight_letters_from_its_own_cut() {
        let fonts = font_definitions();
        for (font, weight) in [(TextFont::Monospace, 400), (TextFont::MonospaceMedium, 500)] {
            let family = text_font(font, 10.0).family;
            let face = fonts.families[&family][0].clone();
            assert_eq!(
                weight_class(&fonts.font_data[&face].font),
                weight,
                "{font:?} must letter from a face of weight {weight}, not fall back"
            );
        }
        // A glyph Plex lacks still resolves through egui's own monospace.
        for family in [MONO_FAMILY.clone(), MONO_MEDIUM_FAMILY.clone()] {
            assert!(fonts.families[&family].len() > 1);
        }
    }
}
