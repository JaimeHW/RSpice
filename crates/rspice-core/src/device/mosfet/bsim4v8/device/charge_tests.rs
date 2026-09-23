use super::*;

struct Sample {
    matrix: Vec<Vec<Value>>,
    rhs: Vec<Value>,
}

impl Sample {
    fn new(n: usize) -> Self {
        Self {
            matrix: vec![vec![0.0; n]; n],
            rhs: vec![0.0; n],
        }
    }
    fn residual(&self, state: &[Value]) -> Vec<Value> {
        self.matrix
            .iter()
            .zip(&self.rhs)
            .map(|(row, rhs)| row.iter().zip(state).map(|(a, x)| a * x).sum::<Value>() - rhs)
            .collect()
    }
}

impl MatrixStamper for Sample {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        if row > 0 && col > 0 {
            self.matrix[row - 1][col - 1] += value;
        }
    }
    fn stamp_rhs(&mut self, row: usize, value: Value) {
        if row > 0 {
            self.rhs[row - 1] += value;
        }
    }
}

fn companion(device: &Bsim4v8Device, state: &[Value], gain: Value) -> Sample {
    let (charge, mode) = device.charge_at_with_probe(state, true);
    let (qg, qmid, qb, qd, qbs, qbd) = device.trnqs_state_charges_with_probe(&charge, state, true);
    let mut sample = Sample::new(state.len());
    device.stamp_trnqs_charge_companion_with_probe(
        &charge,
        mode,
        gain,
        gain * qg,
        gain * qmid,
        gain * qb,
        gain * qd,
        gain * qbs,
        gain * qbd,
        gain * charge.qchqs,
        gain * device.trnqs_qcdump_state(state),
        state,
        &mut sample,
        true,
    );
    sample
}

#[test]
fn bsim4_nqs_middle_gate_charge_and_companion_follow_native_topology() {
    for pmos in [false, true] {
        let models = include_str!("../testdata/models45.lib").replace(
            "level=54 version=4.8",
            "level=54 version=4.8 trnqsmod=1 rgatemod=3 rbodymod=1 rdsmod=1 rshg=1000",
        );
        let model = if pmos { "p90" } else { "n45" };
        let deck = format!(
            "NQS middle gate\nM1 d g s b {model} w=2u l=90n nf=2 m=2 \
             ad=0.2p as=0.3p pd=4.4u ps=5u nrd=1 nrs=1\n{models}\n.end\n"
        );
        let circuit = crate::Engine::default()
            .build_circuit(&crate::Netlist::parse(&deck).unwrap())
            .unwrap();
        let device = &circuit.bsim4v8.devices[0];
        let mut state = vec![0.0; circuit.matrix_size()];
        for reverse in [false, true] {
            let drain = if reverse { -0.3 } else { 0.8 };
            for (node, value) in [
                (device.node_drain_external, drain + 0.01),
                (device.node_drain, drain),
                (device.node_gate_external, 1.0),
                (device.node_gate_mid, 0.9),
                (device.node_gate, 0.7),
                (device.node_source_external, 0.0),
                (device.node_source, 0.05),
                (device.node_bulk_external, -0.3),
                (device.node_bulk, -0.2),
                (device.node_drain_body, -0.23),
                (device.node_source_body, -0.26),
                (device.node_charge_deficit, 1e-7),
            ] {
                state[node - 1] = value * device.core.mtype;
            }
            let before = format!("{device:?}");
            let (charge, _) = device.charge_at_with_probe(&state, true);
            let (qg, qmid, _, _, _, _) =
                device.trnqs_state_charges_with_probe(&charge, &state, true);
            assert_eq!(qg, 0.0);
            let overlap = charge.qgdo
                + charge.qgso
                + charge.cgbo
                    * device.core.mtype
                    * (state[device.node_gate_mid - 1] - state[device.node_bulk - 1]);
            assert!(overlap.abs() > 1e-18);
            assert!((qmid - overlap).abs() < 1e-28);
            let sample = companion(device, &state, 1e10);
            let step = 1e-6;
            for column in 0..state.len() {
                let mut plus = state.clone();
                let mut minus = state.clone();
                plus[column] += step;
                minus[column] -= step;
                let plus = companion(device, &plus, 1e10).residual(&plus);
                let minus = companion(device, &minus, 1e10).residual(&minus);
                for row in 0..state.len() {
                    let derivative = (plus[row] - minus[row]) / (2.0 * step);
                    let actual = sample.matrix[row][column];
                    assert!(
                        (actual - derivative).abs() < 1e-10 + 5e-5 * derivative.abs(),
                        "pmos={pmos} reverse={reverse} ({row},{column}): {actual} vs {derivative}"
                    );
                }
            }
            assert_eq!(
                format!("{device:?}"),
                before,
                "raw sampling must not advance limiter state"
            );
        }
    }
}
