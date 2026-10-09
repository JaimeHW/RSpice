//! Source-based continuation transport, without saving interpreter scratch.
use super::*;
use rspice_veriloga::canonical_ir::digital_eval::{DigitalEvalScratch, checkpoint::*};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

fn transport<T: Serialize + DeserializeOwned>(value: &T) -> T {
    serde_json::from_slice(&serde_json::to_vec(value).unwrap()).unwrap()
}

fn restore(plan: &CanonicalDigitalPlan, state: &DigitalResumeState) -> DigitalResumeState {
    // A newly decoded plan and context cannot reuse the original evaluator's cache.
    let plan = transport(plan);
    DigitalCheckpointReader::new(&plan, DigitalCheckpointLimits::default())
        .unwrap()
        .restore_resume(&transport(&DigitalResumeCheckpoint::capture(state)))
        .unwrap()
}

#[test]
fn checkpoint_analog_barriers_preserve_captured_operands_and_do_not_replay_writes() {
    let mut h = Harness::from_source(
        r#"
module barrier(p); inout p; electrical p;
real a,b,captured,bias; reg choose; reg [7:0] prefix,deferred;
analog begin a=V(p); b=2*V(p); I(p)<+V(p)/1000; end
initial begin : work
  integer local;
  prefix=prefix+1; bias=2; local=7; deferred<=19;
  captured=bias+(choose ? a+b : 9.0)+local;
  #1; captured=captured+a;
end
endmodule
"#,
    );
    h.set("prefix", "00000000");
    h.set("choose", "1");
    let first = expect_suspended(h.start(0));
    let frame = restore(&h.plan, first.resume_state());
    h.set("prefix", "01100100");
    h.set("choose", "0");
    h.set_real("bias", 100.0);
    h.set_analog("a", 3.0);
    let second = expect_suspended(h.resume(0, &frame));
    assert_eq!(
        second.wait(),
        &DigitalWaitRequest::AnalogSample(h.probe("b"))
    );
    let frame = restore(&h.plan, second.resume_state());
    h.set_analog("a", 100.0);
    h.set_analog("b", 4.0);
    let delay = expect_suspended(h.resume(0, &frame));
    assert_eq!(h.get_real("captured"), 16.0);
    assert_eq!(h.get("prefix"), "01100100");
    assert_eq!(h.deferred_count(), 1);
    h.flush_nonblocking();
    assert_eq!(h.get("deferred"), "00010011");
    let frame = restore(&h.plan, delay.resume_state());
    h.set_analog("a", 5.0);
    expect_finished(h.resume(0, &frame));
    assert_eq!(h.get_real("captured"), 21.0);
}

#[test]
fn checkpoint_delayed_assignments_retain_wide_unknown_bits_signed_values_and_real_bits() {
    let mut h = Harness::from_source(
        r#"
module captured;
reg [128:0] source,q; integer signed_source,signed_q; real input_real,r;
initial begin
  signed_source=signed_source; input_real=input_real;
  q = #1 source;
  signed_q = #1 signed_source;
  r = #1 input_real;
  r = #1 input_real;
end
endmodule
"#,
    );
    let bits = format!("x{}z", "10".repeat(63) + "1");
    assert_eq!(bits.len(), 129);
    h.set("source", &bits);
    h.set("signed_source", &format!("{:032b}", (-17i32) as u32));
    h.set_real("input_real", -0.0);
    let wait = expect_suspended(h.start(0));
    let frame = restore(&h.plan, wait.resume_state());
    h.set("source", &"0".repeat(129));
    let wait = expect_suspended(h.resume(0, &frame));
    assert_eq!(h.get("q"), bits);
    let frame = restore(&h.plan, wait.resume_state());
    h.set("signed_source", &"0".repeat(32));
    let wait = expect_suspended(h.resume(0, &frame));
    assert_eq!(h.get("signed_q"), format!("{:032b}", (-17i32) as u32));
    let frame = restore(&h.plan, wait.resume_state());
    let nan_bits = 0xfff8_1234_5678_9abcu64;
    h.set_real("input_real", f64::from_bits(nan_bits));
    let wait = expect_suspended(h.resume(0, &frame));
    assert_eq!(h.get_real("r").to_bits(), (-0.0f64).to_bits());
    let frame = restore(&h.plan, wait.resume_state());
    h.set_real("input_real", 123.0);
    expect_finished(h.resume(0, &frame));
    assert_eq!(h.get_real("r").to_bits(), nan_bits);
}

#[test]
fn checkpoint_event_baselines_survive_changed_storage_and_rebuild_pure_programs() {
    let mut h = Harness::from_source(
        r#"
module events; reg a,b,done; real r;
initial begin r=r; done=0; @(posedge (a & b) or (r + 1.0)) done=1; end
endmodule
"#,
    );
    h.set("a", "0");
    h.set("b", "1");
    h.set_real("r", 0.0);
    let wait = expect_suspended(h.start(0));
    let (DigitalWaitRequest::Expressions(event), frame) = wait.into_parts() else {
        panic!("expressions");
    };
    let image = transport(&DigitalExpressionCheckpoint::capture(&event));
    h.set("a", "1");
    let plan = transport(&h.plan);
    let mut reader =
        DigitalCheckpointReader::new(&plan, DigitalCheckpointLimits::default()).unwrap();
    let mut restored = reader.restore_expression(&image).unwrap();
    assert_eq!(restored.dependencies(), event.dependencies());
    let a = h.signal("a");
    let r = h.signal("r");
    let mut scratch = DigitalEvalScratch::new();
    assert!(
        restored
            .observe(&plan, a, &mut h.store, &mut scratch)
            .unwrap()
    );
    assert!(
        !restored
            .observe(&plan, a, &mut h.store, &mut scratch)
            .unwrap()
    );
    h.set_real("r", 2.0);
    assert!(
        restored
            .observe(&plan, r, &mut h.store, &mut scratch)
            .unwrap()
    );
    assert_eq!(
        h.get("done"),
        "0",
        "rebuilding/observing a subscription must not execute its process"
    );
    let frame = reader
        .restore_resume(&transport(&DigitalResumeCheckpoint::capture(&frame)))
        .unwrap();
    expect_finished(h.resume(0, &frame));
    assert_eq!(h.get("done"), "1");
}

#[test]
fn checkpoint_pending_updates_preserve_selected_targets_repeat_counts_and_real_payloads() {
    let mut h = Harness::from_source(
        r#"
module updates; reg [95:0] n; reg [7:0] q[0:1]; reg clk; integer index; real input_real,r;
initial begin
  input_real=input_real;
  q[0]=0; q[1]=0; index=0;
  q[index][5:2] <= repeat(n) @(posedge clk) 4'b10xz;
  r <= #3 input_real;
  index=1; n=0;
end
endmodule
"#,
    );
    h.set("n", &format!("{}1{}", "0".repeat(31), "0".repeat(64)));
    let bits = 0x7ff8_0000_0000_4321u64;
    h.set_real("input_real", f64::from_bits(bits));
    expect_finished(h.start(0));
    let pending = std::mem::take(&mut h.store.deferred);
    assert_eq!(pending.len(), 2);
    let images: Vec<_> = pending
        .iter()
        .map(|update| transport(&DigitalDeferredCheckpoint::capture(&h.plan, update)))
        .collect();
    let mut reader =
        DigitalCheckpointReader::new(&h.plan, DigitalCheckpointLimits::default()).unwrap();
    let updates: Vec<_> = images
        .iter()
        .map(|image| reader.restore_deferred(image).unwrap())
        .collect();
    drop(reader);
    let Some(DigitalWaitRequest::Repeated { count, event }) = &updates[0].wait else {
        panic!("repeat");
    };
    assert_eq!(
        count.remaining().spelling(),
        format!("{}1{}", "0".repeat(31), "0".repeat(64))
    );
    let mut count = count.clone();
    assert!(!count.consume());
    assert_eq!(
        count.remaining().spelling(),
        "0".repeat(32) + &"1".repeat(64)
    );
    assert!(matches!(event.as_ref(), DigitalWaitRequest::Event(_)));
    h.set("q[0]", "11000011");
    h.set_real("input_real", 10.0);
    for update in &updates {
        apply_deferred(&h.plan, &mut h.store, update).unwrap();
    }
    assert_eq!(h.get("q[0]"), "1110xz11");
    assert_eq!(h.get("q[1]"), "00000000");
    assert_eq!(h.get_real("r").to_bits(), bits);

    let wait_image = serde_json::to_value(DigitalWaitCheckpoint::capture(
        &h.plan,
        updates[0].wait.as_ref().unwrap(),
    ))
    .unwrap();
    for alteration in 0..4 {
        let mut image = wait_image.clone();
        match alteration {
            0 => image["counts"][0]["bval"][0] = json!(1),
            1 => image["counts"][0]["aval"] = json!([0, 0, 0]),
            2 => image["counts"] = json!([image["counts"][0].clone(), image["counts"][0].clone()]),
            _ => image["base"] = json!({"Delay": 1}),
        }
        let image: DigitalWaitCheckpoint = serde_json::from_value(image).unwrap();
        assert!(
            DigitalCheckpointReader::new(&h.plan, DigitalCheckpointLimits::default())
                .unwrap()
                .restore_wait(&image)
                .is_err()
        );
    }
    let mut reader =
        DigitalCheckpointReader::new(&h.plan, DigitalCheckpointLimits::default()).unwrap();
    let restored = reader
        .restore_count(&transport(&DigitalEventCountCheckpoint::capture(&count)))
        .unwrap();
    assert_eq!(restored.remaining(), count.remaining());
}

#[test]
fn checkpoint_rejects_corrupted_frames_planes_subscriptions_and_pending_updates() {
    let mut h = Harness::from_source(
        "module bad; reg [32:0] a,q; reg b; initial begin q = #1 a; @(posedge (a[0] & b)) q<=#2 a; end endmodule",
    );
    h.set("a", &"1".repeat(33));
    h.set("b", "0");
    let first = expect_suspended(h.start(0));
    let original =
        serde_json::to_value(DigitalResumeCheckpoint::capture(first.resume_state())).unwrap();
    let reject = |mutate: &dyn Fn(&mut Value)| {
        let mut value = original.clone();
        mutate(&mut value);
        let image: DigitalResumeCheckpoint = serde_json::from_value(value).unwrap();
        assert!(
            DigitalCheckpointReader::new(&h.plan, DigitalCheckpointLimits::default())
                .unwrap()
                .restore_resume(&image)
                .is_err()
        );
    };
    reject(&|v| v["header"]["version"] = json!(999));
    reject(&|v| v["header"]["plan"][0] = json!(255 - v["header"]["plan"][0].as_u64().unwrap()));
    reject(&|v| v["process"] = json!(u32::MAX));
    reject(&|v| v["block"] = json!(u32::MAX));
    reject(&|v| v["block"] = json!(h.plan.processes[0].function.entry.index()));
    reject(&|v| v["analog_instruction"] = json!(u64::MAX));
    reject(&|v| v["arguments"] = json!([]));
    let packed = original["arguments"]
        .as_array()
        .unwrap()
        .iter()
        .position(|v| v.get("FourState").is_some())
        .expect("captured RHS");
    reject(&|v| v["arguments"][packed] = json!({"Integer": 1}));
    reject(&|v| v["arguments"][packed]["FourState"]["width"] = json!(32));
    reject(&|v| v["arguments"][packed]["FourState"]["aval"] = json!([1]));
    reject(&|v| v["arguments"][packed]["FourState"]["bval"][1] = json!(2));
    let second = expect_suspended(h.resume(0, first.resume_state()));
    let (DigitalWaitRequest::Expressions(event), frame) = second.into_parts() else {
        panic!("expression");
    };
    let mut image = serde_json::to_value(DigitalExpressionCheckpoint::capture(&event)).unwrap();
    image["states"][0]["term"]["edge"] = json!("Negedge");
    let image: DigitalExpressionCheckpoint = serde_json::from_value(image).unwrap();
    assert!(
        DigitalCheckpointReader::new(&h.plan, DigitalCheckpointLimits::default())
            .unwrap()
            .restore_expression(&image)
            .is_err()
    );
    expect_finished(h.resume(0, &frame));
    let original = serde_json::to_value(DigitalDeferredCheckpoint::capture(
        &h.plan,
        &h.store.deferred[0],
    ))
    .unwrap();
    for (field, value) in [
        ("region", json!("Monitor")),
        ("value", json!({"Real": 0})),
        ("target", json!({"signal": u32::MAX, "select": "Whole"})),
    ] {
        let mut image = original.clone();
        image[field] = value;
        let image: DigitalDeferredCheckpoint = serde_json::from_value(image).unwrap();
        assert!(
            DigitalCheckpointReader::new(&h.plan, DigitalCheckpointLimits::default())
                .unwrap()
                .restore_deferred(&image)
                .is_err()
        );
    }
    let mut image = original;
    image["wait"]["base"]["Delay"] = json!(-1);
    let image: DigitalDeferredCheckpoint = serde_json::from_value(image).unwrap();
    assert!(
        DigitalCheckpointReader::new(&h.plan, DigitalCheckpointLimits::default())
            .unwrap()
            .restore_deferred(&image)
            .is_err()
    );
}

#[test]
fn checkpoint_scalar_decoding_has_cumulative_budgets_and_never_normalizes_bad_planes() {
    let h = Harness::from_source("module budget; reg q; initial q=1; endmodule");
    let image = DigitalScalarCheckpoint::capture(&DigitalScalar::FourState(
        FourStateValue::from_u64(33, 7),
    ));
    let mut reader = DigitalCheckpointReader::new(
        &h.plan,
        DigitalCheckpointLimits {
            max_packed_bits: 65,
            ..Default::default()
        },
    )
    .unwrap();
    reader.restore_scalar(&transport(&image)).unwrap();
    assert!(reader.restore_scalar(&image).is_err());
    let mut reader = DigitalCheckpointReader::new(
        &h.plan,
        DigitalCheckpointLimits {
            max_items: 1,
            ..Default::default()
        },
    )
    .unwrap();
    reader.restore_scalar(&image).unwrap();
    assert!(reader.restore_scalar(&image).is_err());
    for (width, aval, bval) in [
        (0, vec![], vec![]),
        (u32::MAX, vec![], vec![]),
        (33, vec![0], vec![0, 0]),
        (33, vec![0, 2], vec![0, 0]),
        (32, vec![0, 0], vec![0]),
    ] {
        let image: DigitalScalarCheckpoint =
            serde_json::from_value(json!({"FourState": {"width":width,"aval":aval,"bval":bval}}))
                .unwrap();
        assert!(
            DigitalCheckpointReader::new(&h.plan, DigitalCheckpointLimits::default())
                .unwrap()
                .restore_scalar(&image)
                .is_err()
        );
    }
}
