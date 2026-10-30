//! Number locale, and display languages.
//!
//! Which decimal and grouping separators the host platform uses, so a typed
//! value is parsed the way the user typed it; and which languages it displays
//! in, so a Han character is lettered in the form its reader expects.

use super::UiNumberLocale;

/// Query the platform's active numeric locale at the application boundary.
/// Portable project files and SPICE decks never call this function.
#[must_use]
pub fn platform_number_locale() -> UiNumberLocale {
    UiNumberLocale {
        decimal_separator: platform_decimal_separator(),
    }
}

#[cfg(target_arch = "wasm32")]
fn platform_decimal_separator() -> Option<char> {
    use wasm_bindgen::JsCast as _;

    let global = js_sys::global();
    let intl = js_sys::Reflect::get(&global, &"Intl".into()).ok()?;
    let constructor = js_sys::Reflect::get(&intl, &"NumberFormat".into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    let formatter = js_sys::Reflect::construct(&constructor, &js_sys::Array::new()).ok()?;
    let format_to_parts = js_sys::Reflect::get(&formatter, &"formatToParts".into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    let parts = format_to_parts
        .call1(&formatter, &wasm_bindgen::JsValue::from_f64(1.1))
        .ok()?;
    let parts = js_sys::Array::from(&parts);
    for part in parts.iter() {
        if js_sys::Reflect::get(&part, &"type".into())
            .ok()
            .and_then(|value| value.as_string())
            .as_deref()
            == Some("decimal")
        {
            return js_sys::Reflect::get(&part, &"value".into())
                .ok()?
                .as_string()?
                .chars()
                .next();
        }
    }
    None
}

#[cfg(windows)]
fn platform_decimal_separator() -> Option<char> {
    use windows_sys::Win32::Globalization::{GetLocaleInfoEx, LOCALE_SDECIMAL};

    let mut value = [0_u16; 4];
    // SAFETY: the buffer is writable for the declared length and the locale
    // name is the Windows-owned user-default sentinel.
    let length = unsafe {
        GetLocaleInfoEx(
            std::ptr::null(),
            LOCALE_SDECIMAL,
            value.as_mut_ptr(),
            value.len() as i32,
        )
    };
    (length > 1)
        .then(|| char::from_u32(u32::from(value[0])))
        .flatten()
}

#[cfg(all(not(windows), not(target_arch = "wasm32")))]
fn platform_decimal_separator() -> Option<char> {
    ["LC_NUMERIC", "LC_ALL", "LANG"]
        .into_iter()
        .find_map(|name| std::env::var(name).ok())
        .and_then(|locale| decimal_separator_for_locale_tag(&locale))
}

/// The languages the platform displays in, most preferred first, as BCP 47
/// tags or POSIX locale names. Empty where the platform does not say.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn platform_display_languages() -> Vec<String> {
    display_languages()
}

#[cfg(windows)]
fn display_languages() -> Vec<String> {
    use windows_sys::Win32::Globalization::{GetUserPreferredUILanguages, MUI_LANGUAGE_NAME};

    let mut count = 0_u32;
    let mut length = 0_u32;
    // SAFETY: a null buffer with a zero length asks only for the length the
    // list needs; both out-pointers are to live locals.
    let sized = unsafe {
        GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut count,
            std::ptr::null_mut(),
            &mut length,
        )
    };
    if sized == 0 || length == 0 {
        return Vec::new();
    }
    let mut names = vec![0_u16; length as usize];
    // SAFETY: the buffer is writable for the length the first call reported.
    let read = unsafe {
        GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut count,
            names.as_mut_ptr(),
            &mut length,
        )
    };
    if read == 0 {
        return Vec::new();
    }
    names
        .split(|&unit| unit == 0)
        .filter(|name| !name.is_empty())
        .map(String::from_utf16_lossy)
        .collect()
}

#[cfg(all(not(windows), not(target_arch = "wasm32")))]
fn display_languages() -> Vec<String> {
    let mut languages: Vec<String> = std::env::var("LANGUAGE")
        .map(|list| list.split(':').map(str::to_owned).collect())
        .unwrap_or_default();
    languages.extend(
        ["LC_ALL", "LC_MESSAGES", "LANG"]
            .into_iter()
            .filter_map(|name| std::env::var(name).ok()),
    );
    languages.retain(|language| !matches!(language.as_str(), "" | "C" | "POSIX"));
    languages
}

#[cfg(any(test, all(not(windows), not(target_arch = "wasm32"))))]
fn decimal_separator_for_locale_tag(locale: &str) -> Option<char> {
    let normalized = locale
        .split(['.', '@'])
        .next()
        .unwrap_or(locale)
        .replace('_', "-")
        .to_ascii_lowercase();
    if normalized.is_empty() || matches!(normalized.as_str(), "c" | "posix") {
        return Some('.');
    }
    let mut parts = normalized.split('-');
    let language = parts.next()?;
    let region = parts.find(|part| part.len() == 2 || part.len() == 3);

    // CLDR decimal conventions. Swiss locales are the material exception
    // among the comma-decimal language families used here.
    if region == Some("ch") {
        return Some('.');
    }
    let comma = matches!(
        language,
        "af" | "az"
            | "be"
            | "bg"
            | "bs"
            | "ca"
            | "cs"
            | "da"
            | "de"
            | "el"
            | "es"
            | "et"
            | "eu"
            | "fi"
            | "fr"
            | "hr"
            | "hu"
            | "id"
            | "is"
            | "it"
            | "lt"
            | "lv"
            | "mk"
            | "nb"
            | "nl"
            | "nn"
            | "pl"
            | "pt"
            | "ro"
            | "ru"
            | "sk"
            | "sl"
            | "sq"
            | "sr"
            | "sv"
            | "tr"
            | "uk"
            | "vi"
    );
    Some(if comma { ',' } else { '.' })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_tags_resolve_numeric_decimal_mark() {
        assert_eq!(decimal_separator_for_locale_tag("de_DE.UTF-8"), Some(','));
        assert_eq!(decimal_separator_for_locale_tag("fr-FR"), Some(','));
        assert_eq!(decimal_separator_for_locale_tag("de-CH"), Some('.'));
        assert_eq!(decimal_separator_for_locale_tag("en_US.UTF-8"), Some('.'));
        assert_eq!(decimal_separator_for_locale_tag("C"), Some('.'));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn display_languages_are_whole_tags() {
        for language in platform_display_languages() {
            assert!(
                !language.is_empty() && !language.contains(['\0', ':']),
                "{language:?}"
            );
        }
    }
}
