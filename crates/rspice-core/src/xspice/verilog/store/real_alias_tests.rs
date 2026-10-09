//! Real-net views use the normal resolver, trial state and event scheduler.
use super::*;
use crate::xspice::event_scheduler::{EventTarget, SchedulerLimits, TimeResolution};
use crate::xspice::verilog::host::DigitalHost;
use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::digital::DigitalRealAlias;

fn compiled(source: &str) -> CanonicalDigitalPlan {
    VerilogACompiler::default()
        .compile_canonical_ir_module(source, None)
        .unwrap()
        .digital
}
fn id(plan: &CanonicalDigitalPlan, name: &str) -> DigitalSignalId {
    plan.signals
        .iter()
        .find(|signal| signal.name == name)
        .unwrap()
        .id
}
fn join(plan: &mut CanonicalDigitalPlan, left: &str, right: &str) {
    let left = id(plan, left);
    let alias = DigitalRealAlias {
        left,
        right: id(plan, right),
        span: plan.signal(left).unwrap().span,
    };
    *plan = plan.clone().with_real_aliases([alias]).unwrap();
}
fn drive(store: &mut DigitalSignalStore, signal: DigitalSignalId, value: f64) {
    store.drive_real_signal(DigitalRealDrive {
        driver: DigitalDriverId { signal, index: 0 },
        value,
    });
}
fn target(name: &str) -> EventTarget {
    EventTarget {
        node_id: 0,
        port_name: "out".into(),
        driver_index: 0,
        instance: name.into(),
    }
}

#[test]
fn real_alias_groups_resolve_original_drivers_once_and_restore_trial_state() {
    for (kind, expected) in [
        ("wrealsum", 6.0),
        ("wrealavg", 3.0),
        ("wrealmin", 2.0),
        ("wrealmax", 4.0),
    ] {
        let mut plan = compiled(&format!(
            "module top; {kind} a,b,c,unrelated; assign a=2.0; assign c=4.0; endmodule"
        ));
        for (a, b) in [("a", "b"), ("b", "c"), ("c", "a"), ("a", "b"), ("b", "b")] {
            join(&mut plan, a, b);
        }
        plan.validate().unwrap();
        let ids = [id(&plan, "a"), id(&plan, "b"), id(&plan, "c")];
        let mut store = DigitalSignalStore::new(&plan);
        store.attach_external_reals(&ids, &[]).unwrap();
        drive(&mut store, ids[0], 2.0);
        store.take_external_changes();
        drive(&mut store, ids[2], 4.0);
        for signal in ids {
            assert_eq!(store.real_value(signal), Some(expected), "{kind}");
        }
        assert_eq!(store.real_value(id(&plan, "unrelated")), Some(0.0));
        store
            .force_real(id(&plan, "unrelated"), 7.0, &plan)
            .unwrap();
        assert_eq!(store.real_value(id(&plan, "unrelated")), Some(7.0));
        for signal in ids {
            assert_eq!(store.real_value(signal), Some(expected));
        }
        let changes = store.take_external_changes();
        assert_eq!(changes.len(), 3, "{kind}");
        for (index, change) in changes.iter().enumerate() {
            assert_eq!(change.starts_publication(), index == 0);
        }
        let accepted = store.clone();
        drive(&mut store, ids[2], -7.0);
        store = accepted;
        for signal in ids {
            assert_eq!(store.real_value(signal), Some(expected));
        }
        let mut fresh = DigitalSignalStore::new(&plan);
        fresh.inherit_bit_connections(&store);
        for signal in ids {
            assert_eq!(fresh.real_value(signal), Some(0.0));
        }
        drive(&mut fresh, ids[0], 2.0);
        drive(&mut fresh, ids[2], 4.0);
        for signal in ids {
            assert_eq!(fresh.real_value(signal), Some(expected));
        }
        assert!(store.check_force_real(ids[1], &plan).is_err());
    }
}

#[test]
fn real_alias_external_drivers_share_resolution_and_validate_the_whole_group() {
    let mut plan = compiled("module top; wreal a,b,c; assign a=2.0; endmodule");
    join(&mut plan, "a", "b");
    join(&mut plan, "b", "c");
    plan.validate().unwrap();
    let mut store = DigitalSignalStore::new(&plan);
    let b = id(&plan, "b");
    assert!(
        store
            .attach_external_reals(&[b], &[(b, target("external"))])
            .unwrap_err()
            .contains("requires one driver")
    );
    assert!(store.external_reals.is_none());
    let mut external_only = compiled("module top; wreal a,b; endmodule");
    join(&mut external_only, "a", "b");
    let a = id(&external_only, "a");
    let b = id(&external_only, "b");
    let mut external_store = DigitalSignalStore::new(&external_only);
    assert!(
        external_store
            .attach_external_reals(&[], &[(a, target("one")), (b, target("two"))])
            .is_err()
    );
    assert!(external_store.external_reals.is_none());
    external_store
        .attach_external_reals(&[], &[(a, target("one"))])
        .unwrap();
    assert!(external_store.check_force_real(b, &external_only).is_err());
    let mut plan = compiled("module top; wrealsum a,b,c; assign a=2.0; endmodule");
    join(&mut plan, "a", "b");
    join(&mut plan, "b", "c");
    plan.validate().unwrap();
    let ids = [id(&plan, "a"), id(&plan, "b"), id(&plan, "c")];
    let mut store = DigitalSignalStore::new(&plan);
    let drivers = store
        .attach_external_reals(&ids, &[(ids[1], target("one")), (ids[2], target("two"))])
        .unwrap();
    drive(&mut store, ids[0], 2.0);
    store.take_external_changes();
    store.publish_external_bank(&[], &[(drivers[0], 3.0), (drivers[1], 4.0)]);
    for signal in ids {
        assert_eq!(store.real_value(signal), Some(9.0));
    }
    let changes = store.take_external_changes();
    assert_eq!(changes.len(), 3);
    for (index, change) in changes.iter().enumerate() {
        assert_eq!(change.starts_publication(), index == 0);
    }
    let accepted = store.clone();
    store.publish_external_bank(&[], &[(drivers[0], -1.0)]);
    assert_eq!(store.real_value(ids[2]), Some(5.0));
    store = accepted;
    store.publish_external_bank(&[], &[(drivers[1], 1.0)]);
    for signal in ids {
        assert_eq!(store.real_value(signal), Some(6.0));
    }
}

#[test]
fn real_alias_publication_is_atomic_for_computed_event_expressions_and_restart() {
    let mut plan = compiled(
        r#"
`timescale 1ns/1ns
module top;
 wrealsum a,b;
 real level=1;
 integer glitches=0, changes=0;
 assign a=level;
 initial #1 level=2;
 always @(a-b) glitches=glitches+1;
 always @(b) changes=changes+1;
endmodule
"#,
    );
    join(&mut plan, "a", "b");
    plan.validate().unwrap();
    let mut host = DigitalHost::new(
        &plan,
        TimeResolution::new(-9).unwrap(),
        SchedulerLimits::default(),
    );
    host.start().unwrap();
    let before = host.read(id(&plan, "changes")).unwrap().to_u64().unwrap();
    host.advance_to(1).unwrap();
    assert_eq!(host.read(id(&plan, "glitches")).unwrap().to_u64(), Some(0));
    assert_eq!(
        host.read(id(&plan, "changes")).unwrap().to_u64(),
        Some(before + 1)
    );
    assert_eq!(host.read_real(id(&plan, "b")), Some(2.0));
    let mut restarted = host.fresh();
    restarted.start().unwrap();
    restarted.advance_to(1).unwrap();
    assert_eq!(
        restarted.read(id(&plan, "glitches")).unwrap().to_u64(),
        Some(0)
    );
    assert_eq!(restarted.read_real(id(&plan, "b")), Some(2.0));
}
