//! The drawing sync's own record of the failures it has already logged.

/// The drawing sync runs every frame; a failure that stands is one record.
#[test]
fn a_standing_sync_failure_is_recorded_once_and_again_after_it_clears() {
    use super::SyncFailures;

    let mut slot = None;
    assert!(SyncFailures::observe(&mut slot, Some("locked")));
    assert!(!SyncFailures::observe(&mut slot, Some("locked")));
    assert!(SyncFailures::observe(&mut slot, Some("missing")));
    assert!(!SyncFailures::observe(&mut slot, None));
    assert!(SyncFailures::observe(&mut slot, Some("missing")));
}
