//! Corrupt or expanded noise packets are refused before retaining a result.
use super::*;
#[test]
fn qpnoise_result_transport_uses_one_buffer_and_refuses_corruption() {
    let SimulationResult::Qpnoise { response, .. } = SimulationResult::qpnoise_test_fixture()
    else {
        unreachable!()
    };
    let mut buffers = Vec::new();
    let transport =
        WorkerQpnoiseResultTransport::from_response(Arc::unwrap_or_clone(response), &mut buffers)
            .unwrap();
    assert_eq!(buffers.len(), 1);
    let mut inline = transport.clone();
    inline.values = WorkerF64Series::Inline(vec![0.0; inline.values.len()]);
    assert!(inline.into_response(&buffers).is_err());
    let mut short = transport.clone();
    short.values = WorkerF64Series::Buffer { buffer: 0, len: 1 };
    assert!(short.into_response(&buffers).is_err());
    let mut altered = transport.clone();
    altered.metadata.result.request.contributor_ranking = false;
    assert!(altered.into_response(&buffers).is_err());
    let mut oversized = transport.clone();
    oversized.values = WorkerF64Series::Buffer {
        buffer: 0,
        len: usize::MAX,
    };
    assert!(oversized.into_response(&buffers).is_err());
    let mut forged = buffers.clone();
    forged[0][0] *= 2.0;
    assert!(transport.clone().into_response(&forged).is_err());
    let decoded = transport.into_response(&buffers).unwrap();
    assert_eq!(decoded.outputs.len(), 3);
}
