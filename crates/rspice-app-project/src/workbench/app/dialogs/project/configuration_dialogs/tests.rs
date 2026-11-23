//! The configuration dialogs as a reader meets them.

use super::*;

#[test]
fn starting_from_default_writes_every_view_default_stops_at() {
    let (views, stops) = StartFrom::Default.lists();
    assert_eq!(views[..3], ["schematic", "extracted", "spice"]);
    let default_stops = DEFAULT_STOP_VIEW_TYPES
        .iter()
        .map(ViewType::display_name)
        .collect::<Vec<_>>();
    assert_eq!(stops, default_stops);
    for stop in &stops {
        assert!(
            views.contains(stop),
            "stop {stop:?} must be in the view list {views:?}"
        );
    }
}

/// A mixed-signal configuration binds every behavioural view Default does:
/// a cell placed by its Verilog or Verilog-AMS view is not left unbound.
#[test]
fn mixed_signal_searches_and_stops_at_every_behavioural_view_default_does() {
    let (views, stops) = StartFrom::MixedSignal.lists();
    let (_, default_stops) = StartFrom::Default.lists();
    for view in default_stops
        .iter()
        .filter(|view| view.starts_with("verilog"))
    {
        assert!(views.contains(view), "{view:?} not in {views:?}");
        assert!(stops.contains(view), "{view:?} not in {stops:?}");
    }
    assert_eq!(views[0], "schematic");
}
