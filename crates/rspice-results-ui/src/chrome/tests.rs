//! Fine and coarse pointer containment for Results controls.

use super::*;

/// Under a coarse pointer the shell raises every control to a 44 px
/// target, and these rows have to make room for the controls they hold.
///
/// The old fixed geometry meant a 44 px chip was laid out inside a 31 px
/// band on a tablet — taller than the row containing it — while the title
/// bar, drawers, navigator and console all grew correctly around it. The
/// assertion is the containment, not the numbers: a row that merely got
/// bigger is no use if it is still shorter than its own controls.
#[test]
fn result_bars_make_room_for_a_touch_target() {
    let mut tokens = Tokens::default();
    tokens.metrics.ctl_h = 44.0;
    let touch = ResultBarMetrics::resolve(&tokens);

    assert!(touch.viewer_tab >= 44.0, "{}", touch.viewer_tab);
    assert!(
        touch.instrument_control >= 44.0,
        "{}",
        touch.instrument_control
    );
    assert!(
        touch.viewer_tabs >= touch.viewer_tab,
        "the strip must contain its own tab: {} < {}",
        touch.viewer_tabs,
        touch.viewer_tab
    );
    assert!(
        touch.sheet_bar >= touch.instrument_control,
        "the instrument bar must contain its own controls: {} < {}",
        touch.sheet_bar,
        touch.instrument_control
    );
    assert!(
        touch.structured_strip >= touch.instrument_control,
        "the structured strip must contain its own controls: {} < {}",
        touch.structured_strip,
        touch.instrument_control
    );
}
