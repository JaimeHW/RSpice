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

/// Build the application font definitions: IBM Plex first, egui's bundled
/// fonts retained as glyph-coverage fallbacks (symbols, emoji).
pub fn font_definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "plex-sans".to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/IBMPlexSans-Regular.ttf")),
    );
    fonts.font_data.insert(
        SANS_MEDIUM.to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/IBMPlexSans-Medium.ttf")),
    );
    fonts.font_data.insert(
        SANS_SEMIBOLD.to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/fonts/IBMPlexSans-SemiBold.ttf"
        )),
    );
    fonts.font_data.insert(
        "plex-mono".to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/IBMPlexMono-Regular.ttf")),
    );
    fonts.font_data.insert(
        MONO_MEDIUM.to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/IBMPlexMono-Medium.ttf")),
    );

    // Prepend our faces so they take priority; egui defaults stay as fallback.
    let proportional = fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default();
    proportional.insert(0, "plex-sans".to_owned());

    let monospace = fonts.families.entry(FontFamily::Monospace).or_default();
    monospace.insert(0, "plex-mono".to_owned());

    // Weighted families fall back to the regular cut, then egui defaults.
    let base_proportional = fonts.families[&FontFamily::Proportional].clone();
    let base_monospace = fonts.families[&FontFamily::Monospace].clone();

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
}
