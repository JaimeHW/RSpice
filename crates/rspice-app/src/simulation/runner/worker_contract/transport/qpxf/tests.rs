//! QPXF rejects malformed or unauthenticated worker packets.
use super::*;

#[test]
fn qpxf_transport_rejects_short_inline_and_altered_complex_buffers() {
    let SimulationResult::Qpxf { response, .. } = SimulationResult::qpxf_test_fixture() else {
        unreachable!()
    };
    let mut buffers = Vec::new();
    let transport =
        WorkerQpxfResultTransport::from_response(Arc::unwrap_or_clone(response), &mut buffers);
    assert_eq!(
        buffers.len(),
        2,
        "buffer count is independent of sweep length"
    );
    let mut short = transport.clone();
    short.imaginary = WorkerF64Series::from_vec(vec![0.0], &mut buffers);
    assert!(short.into_response(&buffers).is_err());
    let mut inline = transport.clone();
    inline.real = WorkerF64Series::Inline(vec![0.0; inline.imaginary.len()]);
    assert!(inline.into_response(&buffers).is_err());
    let mut json = serde_json::to_value(&transport).unwrap();
    // Alter scalar metadata without recomputing its complete-result identity.
    json["metadata"]["request"]["group_delay_magnitude_floor"] = serde_json::json!(0.003);
    let altered: WorkerQpxfResultTransport = serde_json::from_value(json).unwrap();
    assert!(altered.into_response(&buffers).is_err());
    let WorkerF64Series::Buffer { buffer, .. } = transport.real else {
        panic!("complex spectrum was not transferred")
    };
    buffers[buffer][0] += 0.1;
    assert!(transport.into_response(&buffers).is_err());
}
