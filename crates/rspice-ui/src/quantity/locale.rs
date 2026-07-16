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
}
