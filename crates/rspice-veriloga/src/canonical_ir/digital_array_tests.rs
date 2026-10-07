//! Runtime/serialization/linking checks for unpacked scalar-cell storage.
//! Front-end acceptance is tested separately when source lowering is enabled.

use super::digital::*;
use super::digital_eval::*;
use super::digital_link::{DigitalLinkInstance, link_digital_plans};
use super::ids::*;
use super::*;
use crate::four_state::FourStateBit;

fn id(index: u32) -> DigitalSignalId {
    DigitalSignalId::new(index)
}

#[derive(Default)]
struct Store {
    bits: Vec<FourStateValue>,
    reals: Vec<f64>,
    deferred: Vec<DigitalDeferredUpdate>,
    writes: Vec<DigitalSignalId>,
}

impl Store {
    fn new(plan: &CanonicalDigitalPlan) -> Self {
        Self {
            bits: plan
                .signals
                .iter()
                .map(|s| FourStateValue::zero(s.width.max(1)))
                .collect(),
            reals: vec![0.0; plan.signals.len()],
            ..Self::default()
        }
    }
}

impl DigitalEnvironment for Store {
    fn read_clock(&self) -> Option<DigitalClock> {
        None
    }
    fn read_signal(&self, signal: DigitalSignalId) -> Option<FourStateValue> {
        self.bits.get(usize::from(signal)).cloned()
    }
    fn write_signal(&mut self, signal: DigitalSignalId, value: FourStateValue) {
        self.bits[usize::from(signal)] = value;
        self.writes.push(signal);
    }
    fn read_real_signal(&self, signal: DigitalSignalId) -> Option<f64> {
        self.reals.get(usize::from(signal)).copied()
    }
    fn write_real_signal(&mut self, signal: DigitalSignalId, value: f64) {
        self.reals[usize::from(signal)] = value;
        self.writes.push(signal);
    }
    fn defer_update(&mut self, update: DigitalDeferredUpdate) {
        self.deferred.push(update);
    }
    fn read_analog_potential(&self, _: DigitalAnalogProbeId) -> Option<f64> {
        None
    }
    fn drive_real_signal(&mut self, _: DigitalRealDrive) {
        unreachable!()
    }
    fn drive_signal(&mut self, _: DigitalDrive) {
        unreachable!()
    }
}

fn signal(index: u32, name: String, real: bool, width: u32) -> DigitalSignal {
    DigitalSignal {
        initial_value: None,
        id: id(index),
        name: name.into(),
        kind: if real {
            DigitalSignalKind::Real(DigitalRealResolution::Single)
        } else {
            DigitalSignalKind::FourState
        },
        width: if real { 0 } else { width },
        bounds: (!real).then_some((i64::from(width) - 1, 0)),
        signed: true,
        integer: false,
        procedurally_assignable: true,
        span: SourceSpanRef {
            source_file_id: 0,
            start: 0,
            end: 0,
        },
    }
}

fn process(function: CfgFunction) -> CfgDigitalProcess {
    CfgDigitalProcess {
        time_scale: Default::default(),
        id: DigitalProcessId::new(0),
        kind: DigitalProcessKind::Initial,
        function,
        static_sensitivity: None,
        span: SourceSpanRef {
            source_file_id: 0,
            start: 0,
            end: 0,
        },
    }
}

/// Two elements, an independently changing address and a sampled output.
fn fixture(lower: i64, real: bool, real_index: bool) -> CanonicalDigitalPlan {
    let array = DigitalArrayRef {
        base: id(0),
        lower,
        len: 2,
    };
    let element_type = if real {
        CfgValueType::Real
    } else {
        CfgValueType::FourState { width: 8 }
    };
    let index_type = if real_index {
        CfgValueType::Real
    } else {
        CfgValueType::FourState { width: 96 }
    };
    let mut builder = SsaBuilder::new();
    let entry = builder.create_block();
    let index = builder.push(
        entry,
        index_type,
        if real_index {
            CfgValueKind::DigitalRealSignalRead { signal: id(2) }
        } else {
            CfgValueKind::DigitalSignalRead { signal: id(2) }
        },
    );
    let a = builder.push_leaf(
        element_type,
        if real {
            CfgValueKind::RealConstant(1.25)
        } else {
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(8, 0x35))
        },
    );
    let b = builder.push_leaf(
        element_type,
        if real {
            CfgValueKind::RealConstant(-3.5)
        } else {
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(8, 0xc9))
        },
    );
    builder.push(
        entry,
        CfgValueType::Effect,
        CfgValueKind::DigitalArrayBlockingWrite {
            array,
            index,
            signed: true,
            value: a,
        },
    );
    let sample = builder.push(
        entry,
        element_type,
        CfgValueKind::DigitalArrayRead {
            array,
            index,
            signed: true,
        },
    );
    builder.push(
        entry,
        CfgValueType::Effect,
        CfgValueKind::DigitalBlockingWrite {
            target: DigitalWriteTarget {
                signal: id(3),
                select: DigitalWriteSelect::Whole,
            },
            value: sample,
        },
    );
    let delay = builder.push_leaf(CfgValueType::Integer, CfgValueKind::IntegerConstant(5));
    builder.push(
        entry,
        CfgValueType::Effect,
        CfgValueKind::DigitalArrayNonblockingWrite {
            array,
            index,
            signed: true,
            value: b,
            region: DigitalSchedulingRegion::NonBlockingAssign,
            wait: Some(DigitalWait::Delay(delay)),
        },
    );
    let next_index = builder.push_leaf(
        index_type,
        if real_index {
            CfgValueKind::RealConstant((lower + 1) as f64)
        } else {
            CfgValueKind::FourStateConstant(FourStateValue::from_integer(96, i128::from(lower) + 1))
        },
    );
    builder.push(
        entry,
        CfgValueType::Effect,
        CfgValueKind::DigitalBlockingWrite {
            target: DigitalWriteTarget {
                signal: id(2),
                select: DigitalWriteSelect::Whole,
            },
            value: next_index,
        },
    );
    builder.set_terminator(entry, CfgTerminator::Return);
    builder.seal_all_blocks();
    CanonicalDigitalPlan {
        signals: vec![
            signal(0, format!("a[{lower}]"), real, 8),
            signal(1, format!("a[{}]", lower + 1), real, 8),
            signal(2, "index".into(), real_index, 96),
            signal(3, "out".into(), real, 8),
        ],
        arrays: vec![DigitalArray {
            name: "a".into(),
            bounds: (lower + 1, lower),
            storage: array,
        }],
        processes: vec![process(builder.finish(entry).unwrap())],
        ..Default::default()
    }
    .seal()
    .unwrap()
}

#[test]
fn digital_arrays_capture_exact_elements_and_typed_values() {
    for lower in [i64::MIN, -2, (1i64 << 53) + 1, i64::MAX - 1] {
        for real in [false, true] {
            for offset in [0, 1] {
                let plan = fixture(lower, real, false);
                let mut store = Store::new(&plan);
                store.bits[2] = FourStateValue::from_integer(96, i128::from(lower) + offset);
                assert_eq!(
                    start(&plan, &plan.processes[0], &mut store).unwrap(),
                    DigitalProcessOutcome::Finished
                );
                let selected = offset as usize;
                let other = 1 - selected;
                assert_eq!(store.writes, [id(selected as u32), id(3), id(2)]);
                assert_eq!(store.deferred.len(), 1);
                let update = store.deferred.pop().unwrap();
                assert_eq!(update.target.signal, id(selected as u32));
                assert_eq!(update.wait, Some(DigitalWaitRequest::Delay(5)));
                // A later address change cannot redirect a captured update.
                store.bits[2] = FourStateValue::from_integer(96, i128::from(lower) + other as i128);
                if real {
                    assert_eq!(store.reals[3], 1.25);
                    assert_eq!(store.reals[selected], 1.25);
                } else {
                    assert_eq!(store.bits[3].to_u64(), Some(0x35));
                    assert_eq!(store.bits[selected].to_u64(), Some(0x35));
                }
                // An in-memory snapshot retains the scalar target and captured RHS.
                let update = update.clone();
                apply_deferred(&plan, &mut store, &update).unwrap();
                if real {
                    assert_eq!(store.reals[selected], -3.5);
                    assert_eq!(store.reals[other], 0.0);
                } else {
                    assert_eq!(store.bits[selected].to_u64(), Some(0xc9));
                    assert_eq!(store.bits[other].to_u64(), Some(0));
                }
            }
        }
    }
    for real in [false, true] {
        let plan = fixture(-2, real, true);
        let mut store = Store::new(&plan);
        store.reals[2] = -1.5;
        start(&plan, &plan.processes[0], &mut store).unwrap();
        assert_eq!(store.deferred[0].target.signal, id(0));
    }
}

#[test]
fn digital_arrays_handle_invalid_addresses_without_aliasing() {
    let addresses = [
        FourStateValue::splat(96, FourStateBit::Unknown),
        FourStateValue::splat(96, FourStateBit::HighImpedance),
        FourStateValue::from_integer(96, 1i128 << 80),
        FourStateValue::from_integer(96, -3),
        FourStateValue::from_integer(96, 0),
    ];
    for real in [false, true] {
        let plan = fixture(-2, real, false);
        for address in &addresses {
            let mut store = Store::new(&plan);
            store.bits[2] = address.clone();
            let outcome = start(&plan, &plan.processes[0], &mut store);
            if real {
                assert!(matches!(
                    outcome,
                    Err(DigitalEvalError::InvalidNumericConversion { .. })
                ));
                assert!(store.writes.is_empty());
            } else {
                assert_eq!(outcome.unwrap(), DigitalProcessOutcome::Finished);
                assert_eq!(
                    store.bits[3],
                    FourStateValue::splat(8, FourStateBit::Unknown)
                );
                assert_eq!(store.writes, [id(3), id(2)]);
            }
            assert!(store.deferred.is_empty());
            assert_eq!(store.bits[0].to_u64(), Some(0));
            assert_eq!(store.bits[1].to_u64(), Some(0));
            assert_eq!(&store.reals[..2], &[0.0, 0.0]);
        }
    }
    let plan = fixture(-2, false, true);
    for address in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1e30] {
        let mut store = Store::new(&plan);
        store.reals[2] = address;
        start(&plan, &plan.processes[0], &mut store).unwrap();
        assert_eq!(
            store.bits[3],
            FourStateValue::splat(8, FourStateBit::Unknown)
        );
        assert!(store.deferred.is_empty());
    }
}

#[test]
fn digital_arrays_link_validate_and_observe_selected_elements() {
    let source = fixture(-2, false, false);
    let linked = link_digital_plans(
        &[
            DigitalLinkInstance {
                name: "u2",
                plan: &source,
                ports: &[],
            },
            DigitalLinkInstance {
                name: "u1",
                plan: &source,
                ports: &[],
            },
        ],
        &[],
        &crate::NoPipelineControl,
    )
    .unwrap();
    let plan: CanonicalDigitalPlan =
        serde_json::from_str(&serde_json::to_string(&linked.plan).unwrap()).unwrap();
    plan.validate().unwrap();
    assert_eq!(plan.arrays[0].name, "u1.a");
    assert_eq!(plan.arrays[1].name, "u2.a");
    let mut store = Store::new(&plan);
    store.bits[2] = FourStateValue::from_integer(96, -2);
    store.bits[6] = FourStateValue::from_integer(96, -1);
    for process in &plan.processes {
        start(&plan, process, &mut store).unwrap();
    }
    assert_eq!(
        store
            .deferred
            .iter()
            .map(|u| u.target.signal)
            .collect::<Vec<_>>(),
        [id(0), id(5)]
    );

    // Observe a[index] through the actual event machinery, including address changes.
    let mut plan = source.clone();
    let array = plan.arrays[0].storage;
    let mut builder = SsaBuilder::new();
    let entry = builder.create_block();
    let resume = builder.create_block();
    let index = builder.push(
        entry,
        CfgValueType::FourState { width: 96 },
        CfgValueKind::DigitalSignalRead { signal: id(2) },
    );
    let read = builder.push(
        entry,
        CfgValueType::FourState { width: 8 },
        CfgValueKind::DigitalArrayRead {
            array,
            index,
            signed: true,
        },
    );
    builder.set_terminator(
        entry,
        CfgTerminator::Wait {
            wait: DigitalWait::Expressions(vec![DigitalEventExpression {
                value: read,
                edge: None,
            }]),
            resume,
            resume_args: vec![],
        },
    );
    builder.set_terminator(resume, CfgTerminator::Return);
    builder.seal_all_blocks();
    plan.processes[0] = process(builder.finish(entry).unwrap());
    let plan = plan.seal().unwrap();
    let mut store = Store::new(&plan);
    store.bits[2] = FourStateValue::from_integer(96, -2);
    let DigitalProcessOutcome::Suspended(suspension) =
        start(&plan, &plan.processes[0], &mut store).unwrap()
    else {
        panic!("must wait")
    };
    let (DigitalWaitRequest::Expressions(mut event), _) = suspension.into_parts() else {
        panic!("expression wait")
    };
    assert_eq!(event.dependencies(), &[id(0), id(1), id(2)]);
    let mut scratch = DigitalEvalScratch::new();
    store.bits[1] = FourStateValue::from_u64(8, 7);
    assert!(
        !event
            .observe(&plan, id(1), &mut store, &mut scratch)
            .unwrap()
    );
    store.bits[2] = FourStateValue::from_integer(96, -1);
    assert!(
        event
            .observe(&plan, id(2), &mut store, &mut scratch)
            .unwrap()
    );
    store.bits[1] = FourStateValue::from_u64(8, 8);
    assert!(
        event
            .observe(&plan, id(1), &mut store, &mut scratch)
            .unwrap()
    );

    for defect in 0..8 {
        let mut broken = source.clone();
        match defect {
            0 => broken.arrays[0].storage.len = u32::MAX,
            1 => broken.arrays[0].storage.lower = i64::MAX,
            2 => broken.arrays[0].bounds = (1, 0),
            3 => broken.signals[1].signed = false,
            4 => broken.signals[1].procedurally_assignable = false,
            5 => broken.arrays.push(broken.arrays[0].clone()),
            6 => broken.signals[1].name = "wrong".into(),
            7 => {
                let read = broken.processes[0]
                    .function
                    .values
                    .iter_mut()
                    .find(|v| matches!(v.kind, CfgValueKind::DigitalArrayRead { .. }))
                    .unwrap();
                if let CfgValueKind::DigitalArrayRead { array, .. } = &mut read.kind {
                    array.len = 1;
                }
            }
            _ => unreachable!(),
        }
        assert!(broken.seal().is_err(), "accepted defect {defect}");
    }
}
