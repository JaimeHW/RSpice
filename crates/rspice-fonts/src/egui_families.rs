//! The chains as egui font families.

use std::sync::LazyLock;

use egui::{FontData, FontDefinitions, FontFamily};

use crate::{Face, Role};

impl Role {
    /// The egui family the role letters in. The regular sans and mono roles
    /// are egui's two built-in families, so unstyled text letters through
    /// them; a weighted role's family is named after its first face.
    #[must_use]
    pub fn egui_family(self) -> FontFamily {
        // A named family holds an `Arc<str>` and this runs for nearly every
        // widget every frame, so each is built once and cloned.
        static FAMILIES: LazyLock<[FontFamily; Role::ALL.len()]> = LazyLock::new(|| {
            Role::ALL.map(|role| match role {
                Role::Sans => FontFamily::Proportional,
                Role::Mono => FontFamily::Monospace,
                weighted => FontFamily::Name(weighted.primary().key().into()),
            })
        });
        FAMILIES[self as usize].clone()
    }
}

/// Every embedded face, and one family per role holding the role's chain.
///
/// The set starts empty rather than from egui's defaults, so a build whose
/// egui keeps its default fonts still letters only these faces, and what no
/// face has letters as the chain's replacement square.
#[must_use]
pub fn egui_font_definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::empty();
    for face in Face::ALL {
        fonts.font_data.insert(
            face.key().to_owned(),
            FontData::from_static(face.bytes()).into(),
        );
    }
    for role in Role::ALL {
        fonts.families.insert(
            role.egui_family(),
            role.chain()
                .iter()
                .map(|face| face.key().to_owned())
                .collect(),
        );
    }
    fonts
}
