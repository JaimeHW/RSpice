//! The drawing sync's own record of the failures it has already logged, and
//! how it reads a renamed port from the interface a symbol was drawn for.

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

/// A port is renamed when the same port object turns up under another name;
/// a port added or deleted renames nothing, even where one goes and another
/// comes in its place, and two ports trading names are each renamed.
#[test]
fn a_port_object_under_another_name_is_a_rename() {
    use super::renamed_ports;

    let ports = |named: &[(u64, &str)]| {
        named
            .iter()
            .map(|(id, name)| (*id, (*name).to_owned()))
            .collect::<Vec<_>>()
    };
    let renames = |old: &[(u64, &str)], new: &[(u64, &str)]| {
        renamed_ports(&ports(old), &ports(new))
            .into_iter()
            .collect::<Vec<_>>()
    };
    let rename = |from: &str, to: &str| (from.to_owned(), to.to_owned());
    assert_eq!(
        renames(&[(1, "IN"), (2, "OUT")], &[(1, "IN"), (2, "Y")]),
        [rename("OUT", "Y")]
    );
    assert_eq!(
        renames(&[(1, "IN"), (2, "OUT")], &[(1, "in"), (2, "OUT")]),
        []
    );
    assert_eq!(
        renames(&[(1, "IN"), (2, "OUT")], &[(1, "IN"), (3, "B")]),
        []
    );
    assert_eq!(
        renames(&[(1, "IN"), (2, "OUT")], &[(1, "IN"), (2, "OUT"), (3, "B")]),
        []
    );
    assert_eq!(
        renames(&[(1, "A"), (2, "B")], &[(1, "B"), (2, "A")]),
        [rename("A", "B"), rename("B", "A")]
    );
}
