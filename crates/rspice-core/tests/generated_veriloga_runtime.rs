#![cfg(feature = "veriloga-builtins")]

use rspice_core::device::veriloga_generated::{GeneratedEvalContext, GeneratedStamper};
use rspice_core::solver::StaticMatrix;

#[test]
fn generated_stamper_linearizes_current_contribution() {
    let voltages = [1.0, 0.5];
    let mut rhs = vec![0.0; 2];
    let mut matrix =
        StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)])
            .expect("static matrix");

    let ctx = GeneratedEvalContext::new(&voltages);
    assert_eq!(ctx.node_voltage(0), 0.0);
    assert_eq!(ctx.node_voltage(1), 1.0);
    assert_eq!(ctx.node_voltage(2), 0.5);

    GeneratedStamper::new(&mut matrix, &mut rhs, &voltages).stamp_current(
        Some(1),
        Some(2),
        0.2,
        &[(1, 0.01), (2, -0.01)],
    );

    let pp = matrix.get_index(0, 0).expect("pp entry").0;
    let pn = matrix.get_index(0, 1).expect("pn entry").0;
    let np = matrix.get_index(1, 0).expect("np entry").0;
    let nn = matrix.get_index(1, 1).expect("nn entry").0;
    let values = matrix.values_mut();
    assert_eq!(values[pp], 0.01);
    assert_eq!(values[pn], -0.01);
    assert_eq!(values[np], -0.01);
    assert_eq!(values[nn], 0.01);

    let equivalent = 0.2 - (0.01 * 1.0 + -0.01 * 0.5);
    assert_eq!(rhs[0], -equivalent);
    assert_eq!(rhs[1], equivalent);
}
