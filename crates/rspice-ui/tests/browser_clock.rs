//! Execute the production clock boundary in a JS runtime. The same test runs
//! under wasm-bindgen-test in Node, a browser window, or a dedicated worker.

#![cfg(target_arch = "wasm32")]

#[path = "../src/time_compat.rs"]
mod time_compat;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen(inline_js = "
export function replaceDateNow(value) {
    const previous = Date.now;
    Date.now = () => value;
    return previous;
}
export function restoreDateNow(previous) { Date.now = previous; }
")]
extern "C" {
    #[wasm_bindgen(js_name = replaceDateNow)]
    fn replace_date_now(value: f64) -> js_sys::Function;
    #[wasm_bindgen(js_name = restoreDateNow)]
    fn restore_date_now(previous: &js_sys::Function);
}

struct RestoreWallClock(js_sys::Function);

impl Drop for RestoreWallClock {
    fn drop(&mut self) {
        restore_date_now(&self.0);
    }
}

#[wasm_bindgen_test]
fn wall_clock_adjustments_do_not_change_elapsed_time() {
    let _restore = RestoreWallClock(replace_date_now(1_700_000_000_123.0));
    assert_eq!(time_compat::unix_time_ms(), 1_700_000_000_123);
    let started = time_compat::Instant::now();

    replace_date_now(1_700_086_400_123.0);
    assert_eq!(time_compat::unix_time_ms(), 1_700_086_400_123);
    assert!(started.elapsed().as_secs() < 10);

    replace_date_now(1.0);
    let elapsed = started.elapsed();
    assert_eq!(time_compat::unix_time_ms(), 1);
    assert!(elapsed.as_secs() < 10);
    assert!(started.elapsed() >= elapsed);

    for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY, f64::MAX] {
        replace_date_now(invalid);
        assert!(time_compat::checked_unix_epoch().is_err());
        assert!(time_compat::unix_epoch().is_zero());
    }
}
