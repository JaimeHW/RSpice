//! Which written form a Han character is lettered in.
//!
//! One Han code point is drawn differently in Simplified Chinese, in
//! Traditional Chinese as Taiwan and as Hong Kong write it, in Japanese and in
//! Korean, and a face draws one of those forms. egui never asks a face for a
//! language's forms, so the form a reader sees is that of the first face in
//! the chain that has the character: the order of the CJK faces is the choice
//! of form.

/// A written form of the Han characters, and of the faces that draw it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HanVariant {
    Simplified,
    Traditional,
    HongKong,
    Japanese,
    Korean,
}

impl HanVariant {
    pub const ALL: [Self; 5] = [
        Self::Simplified,
        Self::Traditional,
        Self::HongKong,
        Self::Japanese,
        Self::Korean,
    ];

    const fn is_chinese(self) -> bool {
        matches!(self, Self::Simplified | Self::Traditional | Self::HongKong)
    }

    /// The order the CJK faces stand in when this form is wanted: this form
    /// first, the Chinese form closest to it next, then the rest in a fixed
    /// order, so a character this form's face lacks still letters from one.
    #[must_use]
    pub const fn order(self) -> [Self; 5] {
        use HanVariant::{HongKong, Japanese, Korean, Simplified, Traditional};
        match self {
            Simplified => [Simplified, Traditional, HongKong, Japanese, Korean],
            Traditional => [Traditional, HongKong, Simplified, Japanese, Korean],
            HongKong => [HongKong, Traditional, Simplified, Japanese, Korean],
            Japanese => [Japanese, Simplified, Traditional, HongKong, Korean],
            Korean => [Korean, Simplified, Traditional, HongKong, Japanese],
        }
    }
}

/// The form the user asked for in Preferences, or none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum HanPreference {
    /// The document's language decides, then the operating system's.
    #[default]
    Auto,
    Prefer(HanVariant),
}

/// What a language tag says about the Han form.
enum Tagged {
    /// A language that is not written with Han characters.
    Unrelated,
    /// Chinese, without saying which form.
    Chinese,
    Form(HanVariant),
}

/// Reads a BCP 47 tag (`zh-Hant-TW`), a POSIX locale (`ja_JP.UTF-8`) or an
/// ISO 639 code (`ko`, `jpn`).
fn read_tag(tag: &str) -> Tagged {
    let tag = tag.split(['.', '@']).next().unwrap_or_default();
    let mut subtags = tag
        .split(['-', '_'])
        .map(|subtag| subtag.trim().to_ascii_lowercase());
    let language = subtags.next().unwrap_or_default();
    match language.as_str() {
        "ja" | "jpn" => Tagged::Form(HanVariant::Japanese),
        "ko" | "kor" => Tagged::Form(HanVariant::Korean),
        "yue" => Tagged::Form(HanVariant::HongKong),
        "zh" | "zho" | "chi" | "cmn" => {
            // The script subtag decides between the simplified and the
            // traditional forms; the region decides the rest.
            let mut traditional = None;
            let mut region = None;
            for subtag in subtags {
                match subtag.as_str() {
                    "hans" => traditional = Some(false),
                    "hant" => traditional = Some(true),
                    "hk" | "mo" => region = Some(HanVariant::HongKong),
                    "tw" => region = Some(HanVariant::Traditional),
                    "cn" | "sg" | "my" => region = Some(HanVariant::Simplified),
                    _ => {}
                }
            }
            match (traditional, region) {
                (Some(false), _) => Tagged::Form(HanVariant::Simplified),
                (Some(true), Some(HanVariant::HongKong)) => Tagged::Form(HanVariant::HongKong),
                (Some(true), _) => Tagged::Form(HanVariant::Traditional),
                (None, Some(form)) => Tagged::Form(form),
                (None, None) => Tagged::Chinese,
            }
        }
        _ => Tagged::Unrelated,
    }
}

/// The order the CJK faces stand in, in every chain.
///
/// The first source with an opinion decides the form: the document's language
/// (its title block's Language code), then the user's preference, then the
/// first of the operating system's display languages written with Han
/// characters, then Simplified Chinese. A document that says only "Chinese"
/// takes its form from a later source that names a Chinese one.
#[must_use]
pub fn han_order<S: AsRef<str>>(
    document_language: Option<&str>,
    preference: HanPreference,
    system_languages: &[S],
) -> [HanVariant; 5] {
    let mut chinese_only = false;
    match document_language.map(read_tag) {
        Some(Tagged::Form(form)) => return form.order(),
        Some(Tagged::Chinese) => chinese_only = true,
        Some(Tagged::Unrelated) | None => {}
    }
    let wanted = |form: HanVariant| !chinese_only || form.is_chinese();
    if let HanPreference::Prefer(form) = preference
        && wanted(form)
    {
        return form.order();
    }
    for language in system_languages {
        match read_tag(language.as_ref()) {
            Tagged::Form(form) if wanted(form) => return form.order(),
            Tagged::Chinese => return HanVariant::Simplified.order(),
            Tagged::Form(_) | Tagged::Unrelated => {}
        }
    }
    HanVariant::Simplified.order()
}

#[cfg(test)]
mod tests {
    use super::HanVariant::{HongKong, Japanese, Korean, Simplified, Traditional};
    use super::*;

    const NONE: [&str; 0] = [];

    fn first(document: Option<&str>, preference: HanPreference, system: &[&str]) -> HanVariant {
        han_order(document, preference, system)[0]
    }

    #[test]
    fn every_order_holds_each_form_once() {
        for form in HanVariant::ALL {
            let mut order = form.order();
            assert_eq!(order[0], form);
            order.sort();
            assert_eq!(order, HanVariant::ALL, "{form:?}");
        }
    }

    #[test]
    fn language_tags_name_their_form() {
        for (tag, form) in [
            ("ja", Japanese),
            ("ja-JP", Japanese),
            ("ja_JP.UTF-8", Japanese),
            ("jpn", Japanese),
            ("ko-KR", Korean),
            ("zh-CN", Simplified),
            ("zh-Hans", Simplified),
            ("zh_SG", Simplified),
            ("zh-TW", Traditional),
            ("zh-Hant", Traditional),
            ("zh-Hant-TW", Traditional),
            ("zh-HK", HongKong),
            ("zh-Hant-HK", HongKong),
            ("zh-MO", HongKong),
            ("yue", HongKong),
            ("ZH-tw", Traditional),
        ] {
            assert_eq!(first(Some(tag), HanPreference::Auto, &NONE), form, "{tag}");
        }
    }

    #[test]
    fn the_document_language_comes_first() {
        let preference = HanPreference::Prefer(Traditional);
        assert_eq!(first(Some("ja"), preference, &["ko-KR"]), Japanese);
    }

    #[test]
    fn a_document_in_another_language_defers_to_the_preference_then_the_system() {
        assert_eq!(
            first(Some("en"), HanPreference::Prefer(Korean), &["ja-JP"]),
            Korean
        );
        assert_eq!(first(Some("de"), HanPreference::Auto, &["ja-JP"]), Japanese);
        assert_eq!(first(None, HanPreference::Auto, &["ja-JP"]), Japanese);
    }

    #[test]
    fn the_first_system_language_written_in_han_decides() {
        assert_eq!(
            first(
                None,
                HanPreference::Auto,
                &["en-US", "fr-FR", "zh-TW", "ja-JP"]
            ),
            Traditional
        );
        assert_eq!(first(None, HanPreference::Auto, &["zh"]), Simplified);
    }

    #[test]
    fn a_document_in_chinese_takes_only_a_chinese_form_from_later_sources() {
        assert_eq!(
            first(Some("zh"), HanPreference::Prefer(HongKong), &NONE),
            HongKong
        );
        assert_eq!(
            first(
                Some("zh"),
                HanPreference::Prefer(Japanese),
                &["ja-JP", "zh-TW"]
            ),
            Traditional
        );
        assert_eq!(
            first(Some("zh"), HanPreference::Prefer(Japanese), &["ko-KR"]),
            Simplified
        );
    }

    #[test]
    fn with_nothing_to_go_on_the_form_is_simplified() {
        assert_eq!(
            han_order(None, HanPreference::Auto, &NONE),
            Simplified.order()
        );
        assert_eq!(
            han_order(Some(""), HanPreference::Auto, &["en-GB", ""]),
            Simplified.order()
        );
    }
}
