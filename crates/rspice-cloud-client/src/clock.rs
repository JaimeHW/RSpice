use time::OffsetDateTime;

pub(crate) fn current_time_utc() -> Option<OffsetDateTime> {
    platform_time_utc()
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_time_utc() -> Option<OffsetDateTime> {
    Some(OffsetDateTime::now_utc())
}

#[cfg(target_arch = "wasm32")]
fn platform_time_utc() -> Option<OffsetDateTime> {
    offset_datetime_from_epoch_millis(js_sys::Date::now())
}

#[cfg(target_arch = "wasm32")]
fn offset_datetime_from_epoch_millis(value: f64) -> Option<OffsetDateTime> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
        return None;
    }
    let whole_seconds = (value / 1_000.0).floor();
    if whole_seconds > i64::MAX as f64 {
        return None;
    }
    let remaining_millis = value - whole_seconds * 1_000.0;
    if !(0.0..1_000.0).contains(&remaining_millis) {
        return None;
    }
    let whole_seconds = whole_seconds as i64;
    let remaining_millis = remaining_millis as u32;
    OffsetDateTime::from_unix_timestamp(whole_seconds)
        .ok()?
        .replace_nanosecond(remaining_millis.checked_mul(1_000_000)?)
        .ok()
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn javascript_epoch_milliseconds_are_converted_exactly_and_fail_closed() {
        let epoch = offset_datetime_from_epoch_millis(0.0).expect("Unix epoch");
        assert_eq!(epoch.unix_timestamp(), 0);
        assert_eq!(epoch.nanosecond(), 0);

        let subsecond =
            offset_datetime_from_epoch_millis(1_234.0).expect("exact millisecond timestamp");
        assert_eq!(subsecond.unix_timestamp(), 1);
        assert_eq!(subsecond.nanosecond(), 234_000_000);

        for invalid in [f64::NAN, f64::INFINITY, -1.0, 0.5, f64::MAX] {
            assert!(offset_datetime_from_epoch_millis(invalid).is_none());
        }
    }
}
