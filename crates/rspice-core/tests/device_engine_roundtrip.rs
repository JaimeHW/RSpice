//! Device-model checks whose evidence is a deck the engine solves.
//!
//! The model-level assertions stay in each device module's own unit tests.
//! A check that needs `Engine` to elaborate and solve a deck lives here,
//! outside the crate, so `device` never reaches up to the engine to check
//! itself.

#[path = "common/family_src.rs"]
mod family_src;

use rspice_core::{Engine, Netlist, SimulationConfig};

/// The floating-body DC equilibrium must rise into forward bias as the gate
/// turns the device on with the drain at 1.5 V (the RampVg2 bias), driven by
/// impact ionization charging the body until the source diode clamps it.
/// This is the DC anchor the RampVg2 transient relaxes toward.
#[test]
fn floating_body_dc_equilibrium_is_forward_biased() {
    let model = n1_card_text();
    let solve_vb = |vg: f64| -> f64 {
        let deck = format!(
            "*ramp op\nvd d 0 dc 1.5\nvs s 0 dc 0\nve e 0 dc 0\nvg g 0 dc {vg}\nm1 d g s e n1 w=10u l=0.25u\n.option gmin=1e-20 itl1=500\n.op\n{model}\n.end\n"
        );
        let netlist = Netlist::parse(&deck).expect("parse");
        let engine = Engine::new(SimulationConfig::default());
        let res = engine.run_dc_op(&netlist).expect("dc op");
        res.try_voltage_named("m1.__body.internal")
            .expect("body node")
    };
    // Off (Vg=0): body near the t=0 RampVg2 reference (~0.092 V).
    let vb0 = solve_vb(0.0);
    assert!((vb0 - 0.0917).abs() < 5e-3, "Vb(Vg=0)={vb0:.4e}");
    // Saturation (Vg=1): strong impact ionization forward-biases the body.
    let vb1 = solve_vb(1.0);
    assert!(vb1 > 0.4 && vb1 < 0.7, "Vb(Vg=1)={vb1:.4e}");
}

/// End-to-end check: a tied-body NMOS at the t4 first operating point
/// (Vg=0, Vd=0.05, Vb=-0.3) must reproduce the checked-in ngspice reference
/// drain current to within 0.1%. Exercises the builder dispatch, the SOI
/// stamping consistency (cdreq vs. linearized conductances), and the
/// `VOFF=-.14` model-card sign parsing all at once.
#[test]
fn t4_first_point_matches_ngspice_reference() {
    let model = n1_card_text();
    let deck = format!(
        "*t4 single point\nvd d 0 dc 0.05\nvs s 0 dc 0\nve e 0 dc 0\nvg g 0 dc 0\nvb b 0 dc -0.3\nm1 d g s e b n1 w=10u l=0.25u\n.option gmin=1e-25 itl1=500\n.op\n{model}\n.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("parse t4 single point deck");
    let engine = Engine::new(SimulationConfig::default());
    let res = engine.run_dc_op(&netlist).expect("dc op");
    // vs is the 2nd voltage source (branch index 1); i(vs) == drain current.
    let i_vs = res.branch_currents[1];
    // ngspice t4.out row 0: vs#branch = 4.146227e-12 at Vg=0, Vb=-0.3.
    let reference = 4.146227e-12;
    assert!(
        (i_vs - reference).abs() <= 1e-3 * reference.abs(),
        "i(vs)={i_vs:.6e} vs reference {reference:.6e}"
    );
}

/// The `N1` NMOS model card text from `tests/bsim3soidd/nmosdd.mod`, kept in
/// sync with `n1_params` in the B3SOIDD unit tests; written out here so the
/// decks above do not depend on the test working directory.
fn n1_card_text() -> &'static str {
    ".Model N1 NMOS Level=56\n\
     +TNOM=27 TOX=4.5E-09 TSI=5e-8 TBOX=8E-08\n\
     +MOBMOD=0 CAPMOD=3 SHMOD=0\n\
     +PARAMCHK=0 WINT=0 LINT=-2E-08\n\
     +VTH0=.52 K1=.39 K2=.1 K3=0\n\
     +KB1=.95 K3B=2.2 NLX=7.2E-08\n\
     +DVT0=.55 DVT1=.28 DVT2=-1.4\n\
     +DVT0W=0 DVT1W=0 DVT2W=0\n\
     +NCH=3.3E+17 NSUB=1E+15 NGATE=1E+20\n\
     +DVBD0=60.0 DVBD1=1.1 VBSA=0.0\n\
     +KB3=2.2 DELP=0.02\n\
     +ABP=0.9 MXC=0.9 ADICE0=0.93\n\
     +KBJT1=1.0E-08 EDL=.0000005\n\
     +NDIODE=1.13 NTUN=14.0\n\
     +ISBJT=2e-6 ISDIF=1e-6 ISTUN=0.0 ISREC=1e-5\n\
     +XBJT=0.01 XDIF=0.01 XREC=0.01 XTUN=0.001\n\
     +U0=352 UA=1.3E-11 UB=1.7E-18 UC=-4E-10\n\
     +W0=1.16E-06 AGS=.25 A1=0 A2=1\n\
     +B0=.01 B1=10\n\
     +RDSW=700 PRWG=0 PRWB=-.2 WR=1\n\
     +RBODY=0.0 RBSH=0.0\n\
     +A0=1.4 KETA=-.67 VSAT=135000\n\
     +DWG=0 DWB=0\n\
     +ALPHA0=0.0 ALPHA1=1.5 BETA0=20.5\n\
     +AII=1.2 BII=0.1e-7 CII=0.8 DII=0.6\n\
     +VOFF=-.14 NFACTOR=.7 CDSC=.00002 CDSCB=0\n\
     +CDSCD=0 CIT=0\n\
     +PCLM=2.9 PVAG=12 PDIBLC1=.18 PDIBLC2=.004\n\
     +PDIBLCB=-.234 DROUT=.2\n\
     +DELTA=.01 ETA0=.01 ETAB=0\n\
     +DSUB=.3 RTH0=.006\n\
     +CLC=.0000001 CLE=.6 CF=1E-20 CKAPPA=.6\n\
     +CGDL=1E-20 CGSL=1E-20 KT1=-.3 KT1L=0\n\
     +KT2=.022 UTE=-1.5 UA1=4.31E-09 UB1=-7.61E-18\n\
     +UC1=-5.6E-11 PRT=760 AT=22400\n\
     +CGSO=1e-10 CGDO=1e-10 CJSWG=5e-10 TT=3e-10\n\
     +ASD=0.3 CSDESW=1e-12\n"
}

// ===================== BSIM4 v4.8 charge companions =====================
//
// Instances the engine built from `models45.lib`, sampled through their raw
// charge, companion and periodic F/Q stamps.

mod bsim4v8_charge {
    use super::family_src::read_family_src;
    use rspice_core::device::{Bsim4v8Device, MatrixStamper};
    use rspice_core::{Engine, Netlist, Value};

    const MODELS45: &str = "device/mosfet/bsim4v8/testdata/models45.lib";

    struct Sample {
        matrix: Vec<Vec<Value>>,
        rhs: Vec<Value>,
        terms: Vec<usize>,
    }

    impl Sample {
        fn new(n: usize) -> Self {
            Self {
                matrix: vec![vec![0.0; n]; n],
                rhs: vec![0.0; n],
                terms: Vec::new(),
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
                self.terms.push(row);
            }
        }
    }

    fn companion(device: &Bsim4v8Device, state: &[Value], gain: Value) -> Sample {
        let (charge, mode) = device.charge_at_with_probe(state, true);
        let (qg, qmid, qb, qd, qbs, qbd) =
            device.trnqs_state_charges_with_probe(&charge, state, true);
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
            let models = read_family_src(MODELS45).replace(
                "level=54 version=4.8",
                "level=54 version=4.8 trnqsmod=1 rgatemod=3 rbodymod=1 rdsmod=1 rshg=1000",
            );
            let model = if pmos { "p90" } else { "n45" };
            let deck = format!(
                "NQS middle gate\nM1 d g s b {model} w=2u l=90n nf=2 m=2 \
                 ad=0.2p as=0.3p pd=4.4u ps=5u nrd=1 nrs=1\n{models}\n.end\n"
            );
            let circuit = Engine::default()
                .build_circuit(&Netlist::parse(&deck).unwrap())
                .unwrap();
            let device = &circuit.bsim4v8_devices()[0];
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

    #[test]
    fn bsim4_periodic_physical_fq_matches_native_companion_and_derivatives() {
        // Cover each topology selector without a redundant Cartesian expansion.
        for (rgate, rbody, rds, capmod, xpart) in [
            (0, 0, 0, 0, 0.4),
            (1, 0, 0, 1, 0.5),
            (2, 1, 1, 2, 1.0),
            (3, 2, 1, 2, 0.4),
            (3, 1, 1, 1, -1.0),
        ] {
            for pmos in [false, true] {
                for nqs in [false, true] {
                    let options = format!(
                        "level=54 version=4.8 trnqsmod={} rgatemod={rgate} rbodymod={rbody} rdsmod={rds} rshg=1000 capmod={capmod}",
                        usize::from(nqs),
                    );
                    let models = read_family_src(MODELS45)
                        .replace("level=54 version=4.8", &options)
                        .replace("xpart=0", &format!("xpart={xpart}"));
                    let model = if pmos { "p90" } else { "n45" };
                    let deck = format!(
                        "BSIM4 FQ\nM1 d g s b {model} w=2u l=90n nf=2 m=2 \
                         ad=0.2p as=0.3p pd=4.4u ps=5u nrd=1 nrs=1 off\n{models}\n.end\n"
                    );
                    let circuit = Engine::default()
                        .build_circuit(&Netlist::parse(&deck).unwrap())
                        .unwrap();
                    let device = &circuit.bsim4v8_devices()[0];
                    assert_eq!(device.core.model.cap_mod, capmod);
                    assert_eq!(device.core.model.xpart, xpart);
                    for reverse in [false, true] {
                        let drain = if reverse { -0.3 } else { 0.8 };
                        let mut state = vec![0.0; circuit.matrix_size()];
                        for (node, value) in device.periodic_coupling_nodes().into_iter().zip([
                            drain + 0.01,
                            drain,
                            1.0,
                            0.9,
                            0.7,
                            0.0,
                            0.05,
                            -0.3,
                            -0.2,
                            -0.23,
                            -0.26,
                            1e-7,
                        ]) {
                            if node > 0 {
                                state[node - 1] = value * device.core.mtype;
                            }
                        }
                        let before = format!("{device:?}");
                        let sample = |state: &[Value]| {
                            let (mut f, mut q) =
                                (Sample::new(state.len()), Sample::new(state.len()));
                            device.stamp_periodic_fq(state, &mut f, &mut q).unwrap();
                            (f, q)
                        };
                        let (f, q) = sample(&state);
                        let (zero_f, zero_q) = sample(&vec![0.0; state.len()]);
                        assert_eq!(
                            f.terms, zero_f.terms,
                            "zero currents retain their contributor slots"
                        );
                        assert_eq!(
                            q.terms, zero_q.terms,
                            "zero charges retain their contributor slots"
                        );
                        let (charge, mode) = device.charge_at_with_probe(&state, true);
                        let gain = 1e10;
                        let mut native = if nqs {
                            companion(device, &state, gain)
                        } else {
                            Sample::new(state.len())
                        };
                        device.stamp_static_probe(&state, &mut native);
                        if nqs {
                            native.stamp(
                                device.node_charge_deficit,
                                device.node_charge_deficit,
                                -device.multiplier * device.eval_gmin().max(1e-12),
                            );
                        } else {
                            device.stamp_charge_companion_with_probe(
                                &charge,
                                mode,
                                gain,
                                gain * charge.qg_state(),
                                gain * charge.qgmid_state(),
                                gain * charge.qb_state_for_rbody(device.rbody_enabled()),
                                gain * charge.qd_state(),
                                gain * charge.qbs,
                                gain * charge.qbd,
                                &state,
                                &mut native,
                                true,
                            );
                        }
                        for (row, residual) in native.residual(&state).into_iter().enumerate() {
                            let expected = -f.rhs[row] - gain * q.rhs[row];
                            assert!(
                                (residual - expected).abs() < 1e-12 + 1e-8 * expected.abs(),
                                "companion rgate={rgate} rbody={rbody} nqs={nqs} pmos={pmos} reverse={reverse} row={row}: {residual} vs {expected}"
                            );
                        }
                        for column in 0..state.len() {
                            let h = if column + 1 == device.node_charge_deficit {
                                1e-9
                            } else {
                                1e-6
                            };
                            let mut plus = state.clone();
                            let mut minus = state.clone();
                            plus[column] += h;
                            minus[column] -= h;
                            let (fp, qp) = sample(&plus);
                            let (fm, qm) = sample(&minus);
                            for (label, actual, plus, minus, floor) in
                                [("F", &f, &fp, &fm, 1e-9), ("Q", &q, &qp, &qm, 2e-19)]
                            {
                                assert_eq!(actual.terms, plus.terms);
                                assert_eq!(actual.terms, minus.terms);
                                for row in 0..state.len() {
                                    let fd = -(plus.rhs[row] - minus.rhs[row]) / (2.0 * h);
                                    let analytic = actual.matrix[row][column];
                                    assert!(
                                        (fd - analytic).abs()
                                            < floor + 5e-4 * fd.abs().max(analytic.abs()),
                                        "{label} rgate={rgate} rbody={rbody} nqs={nqs} pmos={pmos} reverse={reverse} ({row},{column}): {analytic} vs {fd}"
                                    );
                                }
                            }
                        }
                        // Differentiate the actual stored charges, including split
                        // body and middle-gate routing, along a physical trajectory.
                        let mut rates: Vec<Value> = (0..state.len())
                            .map(|i| ((i * 7 % 13) as Value - 6.0) * 0.13)
                            .collect();
                        if nqs {
                            rates[device.node_charge_deficit - 1] = 2e-7;
                        }
                        let (_, history_rates) =
                            device.periodic_history_sample(&state, &rates).unwrap();
                        let h = 1e-6;
                        let plus: Vec<_> =
                            state.iter().zip(&rates).map(|(x, dx)| x + h * dx).collect();
                        let minus: Vec<_> =
                            state.iter().zip(&rates).map(|(x, dx)| x - h * dx).collect();
                        let (qp, _) = device.periodic_history_sample(&plus, &rates).unwrap();
                        let (qm, _) = device.periodic_history_sample(&minus, &rates).unwrap();
                        for row in 0..8 {
                            let fd = (qp[row] - qm[row]) / (2.0 * h);
                            let actual = history_rates[row];
                            assert!(
                                (actual - fd).abs() < 2e-22 + 5e-4 * actual.abs().max(fd.abs()),
                                "history rgate={rgate} rbody={rbody} nqs={nqs} pmos={pmos} reverse={reverse} row={row}: {actual} vs {fd}"
                            );
                        }
                        if nqs {
                            let (storage, derivative) = device.shooting_nqs_state(&state);
                            let analytic: Value = device
                                .periodic_coupling_nodes()
                                .into_iter()
                                .zip(derivative)
                                .filter(|(node, _)| *node > 0)
                                .map(|(node, c)| c * rates[node - 1])
                                .sum();
                            let fd = (device.shooting_nqs_state(&plus).0
                                - device.shooting_nqs_state(&minus).0)
                                / (2.0 * h);
                            assert!((analytic - fd).abs() < 1e-13 + 5e-4 * fd.abs());
                            let mut moved = state.clone();
                            moved[device.node_gate - 1] += 0.1;
                            moved[device.node_charge_deficit - 1] =
                                device.shooting_nqs_seed(storage, &moved);
                            assert!((device.shooting_nqs_state(&moved).0 - storage).abs() < 1e-18);
                        }
                        let (currents, charges) = device.periodic_lead_fq(&state).unwrap();
                        assert!(currents.iter().sum::<Value>().abs() < 1e-14);
                        assert!(charges.iter().sum::<Value>().abs() < 1e-25);
                        assert_eq!(format!("{device:?}"), before);
                    }
                }
            }
        }
    }
}

// ===================== B3SOI stamp derivatives =====================

mod b3soi_derivatives {
    //! The B3SOI stamps must be the derivative of the residual they stamp.
    //!
    //! The residual of one instance is its share of the assembled transient
    //! system, `r(x) = A(x)·x - b(x)`: the static stamp plus the charge companion
    //! with backward-Euler gain `ag0` against a frozen history,
    //! `cq = ag0·(q(x) - q_ref)`. Newton contracts quadratically only when the
    //! stamped `A(x)` is `dr/dx`, so every stamped entry is compared against a
    //! difference quotient of `r` itself. DD, FD and PD are separate ports, so
    //! each is checked in both conduction modes, for both polarities, with and
    //! without its self-heating node.

    use rspice_core::device::mosfet::b3soi::common::SoiCompanionCurrents;
    use rspice_core::device::mosfet::b3soi::dd::eval::B3SoiDdCharge;
    use rspice_core::device::mosfet::b3soi::fd::eval::B3SoiFdCharge;
    use rspice_core::device::mosfet::b3soi::pd::eval::B3SoiPdCharge;
    use rspice_core::device::{B3SoiDd, B3SoiFd, B3SoiPd, MatrixStamper, NonlinearDevice};
    use rspice_core::{CircuitData, Engine, Netlist, NodeId, Value};

    const N1_CARD: &str = include_str!("../../../tests/ngspice/bsim3soidd/nmosdd.mod");
    const P1_CARD: &str = include_str!("../../../tests/ngspice/bsim3soidd/pmosdd.mod");
    const FD_N1_CARD: &str = include_str!("../../../tests/ngspice/bsim3soifd/nmosfd.mod");
    const FD_P1_CARD: &str = include_str!("../../../tests/ngspice/bsim3soifd/pmosfd.mod");
    const PD_N1_CARD: &str = include_str!("../../../tests/ngspice/bsim3soipd/nmospd.mod");
    const PD_P1_CARD: &str = include_str!("../../../tests/ngspice/bsim3soipd/pmospd.mod");

    /// Dense capture of one instance's stamps over every circuit node.
    struct Assembly {
        n: usize,
        a: Vec<Value>,
        b: Vec<Value>,
    }

    impl Assembly {
        fn new(n: usize) -> Self {
            Self {
                n,
                a: vec![0.0; n * n],
                b: vec![0.0; n],
            }
        }

        fn entry(&self, row: NodeId, col: NodeId) -> Value {
            self.a[(row - 1) * self.n + (col - 1)]
        }

        /// `r_i = sum_k A_ik x_k - b_i` and the magnitude of the terms summed,
        /// `sum_k |A_ik x_k| + |b_i|`, which bounds the rounding in `r_i`.
        fn residual(&self, x: &[Value], row: NodeId) -> (Value, Value) {
            let i = row - 1;
            let mut sum = -self.b[i];
            let mut scale = self.b[i].abs();
            for (k, xk) in x.iter().enumerate() {
                let term = self.a[i * self.n + k] * xk;
                sum += term;
                scale += term.abs();
            }
            (sum, scale)
        }
    }

    impl MatrixStamper for Assembly {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            self.a[(row - 1) * self.n + (col - 1)] += value;
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            self.b[index - 1] += value;
        }
    }

    /// The nodes one instance stamps, `0` where it has none.
    struct SoiNodes {
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        e: NodeId,
        body: NodeId,
        p: NodeId,
        temp: NodeId,
    }

    /// What the check drives on each SOI port.
    trait SoiDevice {
        type Charge;
        fn first_in(circuit: &CircuitData) -> Self;
        fn nodes(&self) -> SoiNodes;
        fn polarity(&self) -> Value;
        fn set_transient(&self);
        /// Linearize exactly at `x`, with no Newton limiting.
        fn linearize_at(&mut self, x: &[Value]);
        fn stamp_static(&self, x: &[Value], matrix: &mut Assembly);
        fn charge(&self, x: &[Value]) -> Self::Charge;
        /// `[qg, qb, qd, qe, qth]`.
        fn node_charges(q: &Self::Charge) -> [Value; 5];
        fn stamp_companion(
            &self,
            q: &Self::Charge,
            ag0: Value,
            currents: SoiCompanionCurrents,
            x: &[Value],
            matrix: &mut Assembly,
        );
    }

    macro_rules! soi_device {
        ($device:ty, $charge:ty, $instances:ident, $p:ident) => {
            impl SoiDevice for $device {
                type Charge = $charge;

                fn first_in(circuit: &CircuitData) -> Self {
                    circuit.$instances()[0].clone()
                }

                fn nodes(&self) -> SoiNodes {
                    SoiNodes {
                        drain: self.node_drain,
                        gate: self.node_gate,
                        source: self.node_source,
                        e: self.node_e,
                        body: self.node_body,
                        p: self.$p,
                        temp: self.node_temp,
                    }
                }

                fn polarity(&self) -> Value {
                    self.mtype
                }

                fn set_transient(&self) {
                    self.set_dc_mode(false);
                }

                fn linearize_at(&mut self, x: &[Value]) {
                    self.update_static_linearization(x);
                }

                fn stamp_static(&self, x: &[Value], matrix: &mut Assembly) {
                    self.stamp_nonlinear(x, matrix, &mut []);
                }

                fn charge(&self, x: &[Value]) -> Self::Charge {
                    self.charge_at(x)
                }

                fn node_charges(q: &Self::Charge) -> [Value; 5] {
                    [q.qg, q.qb, q.qd, q.qe, q.qth]
                }

                fn stamp_companion(
                    &self,
                    q: &Self::Charge,
                    ag0: Value,
                    currents: SoiCompanionCurrents,
                    x: &[Value],
                    matrix: &mut Assembly,
                ) {
                    self.stamp_charge_companion(q, ag0, currents, x, matrix);
                }
            }
        };
    }

    soi_device!(B3SoiDd, B3SoiDdCharge, b3soi_dd_devices, node_p);
    // FD reads a body contact for its initial guess only and has no tie node.
    soi_device!(B3SoiFd, B3SoiFdCharge, b3soi_fd_devices, node_body);
    soi_device!(B3SoiPd, B3SoiPdCharge, b3soi_pd_devices, node_p);

    /// Linearize the instance exactly at `x` (no Newton limiting) and capture its
    /// static and charge-companion stamps.
    fn assemble<D: SoiDevice>(dev: &mut D, x: &[Value], ag0: Value, q_ref: &D::Charge) -> Assembly {
        dev.linearize_at(x);
        let mut assembly = Assembly::new(x.len());
        dev.stamp_static(x, &mut assembly);
        if ag0 > 0.0 {
            let q = dev.charge(x);
            let now = D::node_charges(&q);
            let then = D::node_charges(q_ref);
            let cq = |k: usize| ag0 * (now[k] - then[k]);
            dev.stamp_companion(
                &q,
                ag0,
                SoiCompanionCurrents {
                    cqg: cq(0),
                    cqb: cq(1),
                    cqd: cq(2),
                    cqe: cq(3),
                    cqth: cq(4),
                },
                x,
                &mut assembly,
            );
        }
        assembly
    }

    /// One bias point: absolute node voltages `[d, g, s, e, body]`. A body
    /// contact sits at the body voltage, so no current flows in a tie resistor.
    struct BiasPoint(&'static str, [Value; 5]);

    struct Instance<D> {
        dev: D,
        n: usize,
        /// Device nodes (row/column labels) in stamp order.
        nodes: Vec<(&'static str, NodeId)>,
    }

    /// Build the first SOI instance of `deck`, whose sources hold every node at
    /// 0 V; the check sets the node voltages itself.
    fn build<D: SoiDevice>(deck: &str) -> Instance<D> {
        let netlist = Netlist::parse(deck).expect("derivative-check deck parses");
        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("derivative-check circuit builds");
        let n = circuit.num_nodes();
        let dev = D::first_in(&circuit);
        dev.set_transient();
        let SoiNodes {
            drain,
            gate,
            source,
            e,
            body,
            p,
            temp,
        } = dev.nodes();
        let mut nodes = vec![
            ("D", drain),
            ("G", gate),
            ("S", source),
            ("E", e),
            ("B", body),
        ];
        if p != body {
            nodes.push(("P", p));
        }
        if temp != 0 {
            nodes.push(("T", temp));
        }
        Instance { dev, n, nodes }
    }

    /// A deck holding one instance of `card` with its terminals at 0 V.
    fn deck(family: &str, card: &str, polarity: &str, terminals: &str) -> String {
        let model = if polarity == "n" { "n1" } else { "p1" };
        format!(
            "* {family} derivative check\n\
             vd d 0 0\nvg g 0 0\nvs s 0 0\nve e 0 0\nvp p 0 0\n\
             m1 {terminals} {model} w=10u l=0.25u\n{card}\n.end\n"
        )
    }

    /// Build one B3SOIDD instance from a model card: a floating body
    /// (`d g s e`), an ideal body tie (`d g s e p`), or a tie through `RBODY`.
    fn instance(card: &str, polarity: &str, body: &str) -> Instance<B3SoiDd> {
        let (terminals, card) = match body {
            "floating" => ("d g s e", card.to_string()),
            "tied" => ("d g s e p", card.to_string()),
            "resistive" => ("d g s e p", card.replace("RBODY = 0.0", "RBODY = 200.0")),
            other => panic!("unknown body case {other}"),
        };
        build(&deck("B3SOIDD", &card, polarity, terminals))
    }

    /// The solution vector at `point`, with the self-heating node, if any, at
    /// `del_temp`.
    fn solution<D: SoiDevice>(
        inst: &Instance<D>,
        point: &BiasPoint,
        del_temp: Value,
    ) -> Vec<Value> {
        let [vd, vg, vs, ve, vb] = point.1;
        let nodes = inst.dev.nodes();
        let mut x = vec![0.0; inst.n];
        for (node, volts) in [
            (nodes.drain, vd),
            (nodes.gate, vg),
            (nodes.source, vs),
            (nodes.e, ve),
            (nodes.p, vb),
            (nodes.body, vb),
            (nodes.temp, del_temp),
        ] {
            if node != 0 {
                x[node - 1] = volts;
            }
        }
        x
    }

    struct Mismatch {
        row: &'static str,
        col: &'static str,
        stamped: Value,
        fd: Value,
        tol: Value,
    }

    /// Difference step in volts.
    const H: Value = 1.0e-6;
    /// Floor on the relative rounding of one evaluation of a residual row,
    /// against the magnitude of the terms it sums: 2^-46, 64 ulps.
    const EPS_EVAL: Value = 1.421_085_471_520_200_4e-14;
    /// Offsets, in steps, of two extra samples off the step's grid.
    ///
    /// The FD drain current is quantized along vd: below threshold `Ids` jumps
    /// about 2e-12 of itself every 1 uV of vd (2.2 uV for NMOS), as ngspice-46's
    /// B3SOIFD does to 17 digits. Samples spaced by that period all read one
    /// phase of the sawtooth, so the fourth difference misses it while the
    /// quotient carries its slope; these two samples sit at other phases.
    const OFF_GRID: [Value; 2] = [0.37, 0.61];

    /// Weights of the quartic through samples at `steps`, evaluated at `t`.
    fn quartic_weights(steps: [Value; 5], t: Value) -> [Value; 5] {
        std::array::from_fn(|k| {
            (0..5)
                .filter(|&j| j != k)
                .map(|j| (t - steps[j]) / (steps[k] - steps[j]))
                .product()
        })
    }

    thread_local! {
        /// `tol / |stamped|` of every non-negligible entry checked on this thread,
        /// reported so a passing run shows how tight the check was.
        static RELATIVE_TOLERANCES: std::cell::RefCell<Vec<Value>> =
            const { std::cell::RefCell::new(Vec::new()) };
    }

    /// Compare every stamped entry against a difference quotient `D(h)` of the
    /// residual.
    ///
    /// The quotient is central, `(r(x+h) - r(x-h)) / 2h`, except for the drain and
    /// source columns of a device at (or within the probe of) vds = 0. There the
    /// normal/inverse mode select makes the residual a function of
    /// `min(vd, vs)`-referenced voltages with a kink, so the stamp is the
    /// derivative of the mode it selected, and the quotient is the second-order
    /// one-sided `(-3r(x) + 4r(x+sh) - r(x+2sh)) / 2h` stepping into that mode.
    /// At vds exactly 0 the reference also switches the parasitic-BJT currents
    /// off (`Vds == 0.0`, b3soiddld.c:2393), so the static residual jumps there
    /// and has no drain or source derivative; those two columns check the charge
    /// companion alone, `A(ag0) - A(0)` against the quotient of `r(ag0) - r(0)`.
    ///
    /// The tolerance is the error of the quotient itself:
    /// - truncation `|D(h) - D(2h)|`, three times the leading `h^2` term;
    /// - rounding: the evaluation noise of the residual, measured as the fourth
    ///   difference `|r(-2h) - 4r(-h) + 6r(0) - 4r(h) + r(2h)|` of the five
    ///   samples (whose own `h^4 r''''` is far below it), or, when larger, 16
    ///   times (the fourth difference's total weight) the departure of the
    ///   `OFF_GRID` samples from the quartic through the five; floored at
    ///   `EPS_EVAL * S_i`, `S_i` the summed term magnitude of row `i`; divided by
    ///   `h`, and by `h/4` for the larger one-sided weights;
    /// - the stamped entry's own rounding, `EPS_EVAL` times its row's largest entry.
    fn check<D: SoiDevice>(inst: &mut Instance<D>, x: &[Value], ag0: Value) -> Vec<Mismatch> {
        let base_q = {
            inst.dev.linearize_at(x);
            inst.dev.charge(x)
        };
        let base = assemble(&mut inst.dev, x, ag0, &base_q);
        let base_static = assemble(&mut inst.dev, x, 0.0, &base_q);
        let mtype = inst.dev.polarity();
        let SoiNodes { drain, source, .. } = inst.dev.nodes();
        let vds = mtype * (x[drain - 1] - x[source - 1]);
        // The terminal step that keeps the device in its selected mode.
        let into_mode = if vds >= 0.0 { mtype } else { -mtype };
        let nodes = inst.nodes.clone();
        let mut out = Vec::new();
        for &(col_label, col) in &nodes {
            if col == 0 {
                continue;
            }
            let drain_or_source = col == drain || col == source;
            let dynamic_only = vds == 0.0 && drain_or_source;
            if dynamic_only && ag0 == 0.0 {
                continue;
            }
            let side = if vds.abs() > 4.0 * H || !drain_or_source {
                None
            } else if col == drain {
                Some(into_mode)
            } else {
                Some(-into_mode)
            };
            let mut probe = |delta: Value| {
                let mut xp = x.to_vec();
                xp[col - 1] += delta;
                let full = assemble(&mut inst.dev, &xp, ag0, &base_q);
                let static_part = dynamic_only.then(|| assemble(&mut inst.dev, &xp, 0.0, &base_q));
                nodes
                    .iter()
                    .map(|&(_, row)| {
                        if row == 0 {
                            return (0.0, 0.0);
                        }
                        let (r, s) = full.residual(&xp, row);
                        match &static_part {
                            Some(asm) => {
                                let (r0, s0) = asm.residual(&xp, row);
                                (r - r0, s + s0)
                            }
                            None => (r, s),
                        }
                    })
                    .collect::<Vec<_>>()
            };
            // Five samples per column: centred on x, or stepping into the mode.
            let (steps, sign, one_sided) = match side {
                None => ([-2.0, -1.0, 0.0, 1.0, 2.0], 1.0, false),
                Some(s) => ([0.0, 1.0, 2.0, 3.0, 4.0], s, true),
            };
            let offsets = steps.map(|k| k * sign * H);
            let samples = offsets.map(&mut probe);
            let off_grid = OFF_GRID.map(|t| (quartic_weights(steps, t), probe(t * sign * H)));
            // A second, independent estimate: the Richardson extrapolation of
            // the same quotient at H_COARSE and 2*H_COARSE. A wrong stamp misses
            // the derivative by the same amount at every step, so it fails
            // both; rounding and quantization move only the fine quotients, and
            // the extrapolation removes the coarse ones' h^2 truncation.
            // B3SOIFD's subthreshold drain current (fd/eval.rs) is quantized
            // along ve on a ~1e-5 V scale, which the fine samples read as a
            // steady slope 3e-8 off the derivative.
            const H_COARSE: Value = 1.0e-3;
            let mut coarse: Option<Vec<Vec<(Value, Value)>>> = None;
            for (r, &(row_label, row)) in nodes.iter().enumerate() {
                if row == 0 {
                    continue;
                }
                let f = samples.each_ref().map(|sample| sample[r].0);
                let scale = samples
                    .iter()
                    .map(|sample| sample[r].1)
                    .fold(0.0, Value::max);
                let departure = off_grid
                    .iter()
                    .map(|(w, sample)| {
                        let fit: Value = w.iter().zip(&f).map(|(w, f)| w * f).sum();
                        (sample[r].0 - fit).abs()
                    })
                    .fold(0.0, Value::max);
                let noise = (f[0] - 4.0 * f[1] + 6.0 * f[2] - 4.0 * f[3] + f[4])
                    .abs()
                    .max(16.0 * departure)
                    .max(EPS_EVAL * scale);
                let (d1, d2, rounding) = if one_sided {
                    (
                        sign * (-3.0 * f[0] + 4.0 * f[1] - f[2]) / (2.0 * H),
                        sign * (-3.0 * f[0] + 4.0 * f[2] - f[4]) / (4.0 * H),
                        4.0 * noise / H,
                    )
                } else {
                    (
                        (f[3] - f[1]) / (2.0 * H),
                        (f[4] - f[0]) / (4.0 * H),
                        noise / H,
                    )
                };
                let row_max = nodes
                    .iter()
                    .filter(|(_, c)| *c != 0)
                    .map(|&(_, c)| base.entry(row, c).abs())
                    .fold(0.0, Value::max);
                let tol = (d1 - d2).abs() + rounding + EPS_EVAL * row_max;
                let stamped = if dynamic_only {
                    base.entry(row, col) - base_static.entry(row, col)
                } else {
                    base.entry(row, col)
                };
                if stamped.abs() > 1e-3 * row_max {
                    RELATIVE_TOLERANCES.with(|t| t.borrow_mut().push(tol / stamped.abs()));
                }
                if (stamped - d1).abs() > tol {
                    let c = coarse.get_or_insert_with(|| {
                        let k: &[Value] = if one_sided {
                            &[1.0, 2.0, 4.0]
                        } else {
                            &[-2.0, -1.0, 1.0, 2.0]
                        };
                        k.iter().map(|k| probe(k * sign * H_COARSE)).collect()
                    });
                    let g = |i: usize| c[i][r].0;
                    let (c1, c2) = if one_sided {
                        (
                            sign * (-3.0 * f[0] + 4.0 * g(0) - g(1)) / (2.0 * H_COARSE),
                            sign * (-3.0 * f[0] + 4.0 * g(1) - g(2)) / (4.0 * H_COARSE),
                        )
                    } else {
                        (
                            (g(2) - g(1)) / (2.0 * H_COARSE),
                            (g(3) - g(0)) / (4.0 * H_COARSE),
                        )
                    };
                    if (stamped - (4.0 * c1 - c2) / 3.0).abs() <= tol {
                        continue;
                    }
                    out.push(Mismatch {
                        row: row_label,
                        col: col_label,
                        stamped,
                        fd: d1,
                        tol,
                    });
                }
            }
        }
        // Leave the instance linearized at the base point.
        inst.dev.linearize_at(x);
        out
    }

    /// The operating point of b3soi_native's vds = 0 boundary deck: drain,
    /// source and back gate at 1.25 V, gate at 0, floating body.
    const BOUNDARY_DECK_OP: BiasPoint = BiasPoint(
        "vds=0 boundary deck op",
        [1.25, 0.0, 1.25, 1.25, 1.251_203_190_83],
    );

    /// `[d, g, s, e, body]` for the N1 card.
    const N_POINTS: &[BiasPoint] = &[
        BOUNDARY_DECK_OP,
        BiasPoint("vds=0 gate off", [1.25, 0.0, 1.25, 1.25, 1.34]),
        BiasPoint("vds=0 gate on", [1.25, 2.5, 1.25, 1.25, 1.30]),
        BiasPoint("vds=+1mV gate off", [1.251, 0.0, 1.25, 1.25, 1.34]),
        BiasPoint("vds=-1mV gate off", [1.249, 0.0, 1.25, 1.25, 1.34]),
        BiasPoint("vds=+1mV gate on", [1.251, 2.5, 1.25, 1.25, 1.30]),
        BiasPoint("vds=-1mV gate on", [1.249, 2.5, 1.25, 1.25, 1.30]),
        BiasPoint("forward subthreshold", [1.5, 0.0, 0.0, 0.0, 0.09]),
        BiasPoint("forward moderate", [1.5, 0.6, 0.0, 0.0, 0.2]),
        BiasPoint("forward strong saturation", [1.5, 2.5, 0.0, 0.0, 0.4]),
        BiasPoint("forward strong linear", [0.1, 2.5, 0.0, 0.0, 0.05]),
        BiasPoint("reverse subthreshold", [0.0, 0.0, 1.5, 0.0, 0.09]),
        BiasPoint("reverse strong saturation", [0.0, 2.5, 1.5, 0.0, 0.4]),
        BiasPoint("reverse body bias", [1.5, 1.0, 0.0, 0.0, -0.3]),
    ];

    /// `[d, g, s, e, body]` for the P1 card.
    const P_POINTS: &[BiasPoint] = &[
        BiasPoint("vds=0 gate off", [1.25, 2.5, 1.25, 1.25, 1.16]),
        BiasPoint("vds=0 gate on", [1.25, 0.0, 1.25, 1.25, 1.20]),
        BiasPoint("forward subthreshold", [1.0, 2.5, 2.5, 1.25, 2.41]),
        BiasPoint("forward strong saturation", [1.0, 0.0, 2.5, 1.25, 2.3]),
        BiasPoint("reverse strong saturation", [2.5, 0.0, 1.0, 1.25, 2.3]),
    ];

    /// Backward-Euler gains: the static stamp alone, then steps of 1 ns, 10 ps,
    /// and the 0.17 ps first step of the vds = 0 boundary deck.
    const AG0: &[(&str, Value)] = &[
        ("static", 0.0),
        ("dt=1e-9", 1.0e9),
        ("dt=1e-11", 1.0e11),
        ("dt=1.7e-13", 1.0 / 1.7e-13),
    ];

    /// The card in the requested dialect: as written (ngspice LEVEL=56), or as
    /// Xyce's LEVEL=10 SOIMOD=1, which routes the same card to the Xyce B3SOIDD.
    fn dialect_card(card: &str, xyce: bool) -> String {
        if !xyce {
            return card.to_string();
        }
        let xyce_card = card.replace("Level= 56", "Level= 10 SOIMOD = 1");
        assert_ne!(xyce_card, card, "the card names LEVEL 56");
        xyce_card
    }

    /// One line per mismatched entry.
    fn mismatch_line(case: &str, m: &Mismatch) -> String {
        format!(
            "{case} dF[{}]/dx[{}]: stamped {:>13.6e} FD {:>13.6e} (err {:.2e}, tol {:.2e})",
            m.row,
            m.col,
            m.stamped,
            m.fd,
            (m.stamped - m.fd).abs(),
            m.tol,
        )
    }

    fn sweep(
        family: &str,
        card: &str,
        polarity: &str,
        points: &[BiasPoint],
        xyce: bool,
    ) -> (usize, Vec<String>) {
        let mut checked = 0;
        let mut lines = Vec::new();
        let card = dialect_card(card, xyce);
        for capmod in ["3", "2"] {
            let card = card.replace("CAPMOD = 3", &format!("CAPMOD = {capmod}"));
            for body in ["floating", "tied", "resistive"] {
                let mut inst = instance(&card, polarity, body);
                for point in points {
                    let x = solution(&inst, point, 0.0);
                    for &(dt_label, ag0) in AG0 {
                        checked += 1;
                        for m in check(&mut inst, &x, ag0) {
                            lines.push(mismatch_line(
                                &format!(
                                    "{family} capmod{capmod} {body:<9} {:<26} {dt_label:<10}",
                                    point.0
                                ),
                                &m,
                            ));
                        }
                    }
                }
            }
        }
        (checked, lines)
    }

    /// Print the mismatches and the tolerance spread, then fail on any mismatch.
    fn report(sweeps: &[(usize, Vec<String>)]) {
        let checked: usize = sweeps.iter().map(|(n, _)| n).sum();
        let lines: Vec<&str> = sweeps
            .iter()
            .flat_map(|(_, l)| l.iter().map(String::as_str))
            .collect();
        for line in &lines {
            println!("{line}");
        }
        let mut rel = RELATIVE_TOLERANCES.with(|t| t.take());
        rel.sort_by(Value::total_cmp);
        if let Some(&max) = rel.last() {
            let at = |q: Value| rel[((rel.len() - 1) as Value * q) as usize];
            println!(
                "{} entries; tolerance / |entry|: median {:.1e}, 90% {:.1e}, 99% {:.1e}, max {max:.1e}",
                rel.len(),
                at(0.5),
                at(0.9),
                at(0.99),
            );
        }
        println!("{} mismatched entries over {checked} checks", lines.len());
        assert!(
            lines.is_empty(),
            "{} stamped entries are not the derivative of the residual over {checked} checks:\n{}",
            lines.len(),
            lines.join("\n")
        );
    }

    fn check_dialect(xyce: bool) {
        report(&[
            sweep("nmos", N1_CARD, "n", N_POINTS, xyce),
            sweep("pmos", P1_CARD, "p", P_POINTS, xyce),
        ]);
    }

    #[test]
    fn stamped_jacobian_is_the_derivative_of_the_stamped_residual() {
        check_dialect(false);
    }

    /// The same check on Xyce's B3SOIDD (LEVEL=10 SOIMOD=1): its charges are
    /// Xyce's, and its stamps their derivatives even where Xyce's own
    /// capacitances leave terms out.
    #[test]
    fn xyce_stamped_jacobian_is_the_derivative_of_the_stamped_residual() {
        check_dialect(true);
    }

    /// `[d, g, s, e, body]` for an N-channel FD or PD card, in both modes; a
    /// P-channel card takes every voltage negated. The reverse points put the
    /// source above the drain, and one forward-biases the body against the mode
    /// drain so its junction carries current.
    const SOI_POINTS: &[BiasPoint] = &[
        BiasPoint("forward subthreshold", [1.5, 0.0, 0.0, 0.0, 0.09]),
        BiasPoint("forward moderate", [1.5, 0.6, 0.0, 0.0, 0.2]),
        BiasPoint("forward strong saturation", [1.5, 2.5, 0.0, 0.0, 0.4]),
        BiasPoint("forward strong linear", [0.1, 2.5, 0.0, 0.0, 0.05]),
        BiasPoint("forward back gate", [1.5, 1.0, 0.0, 1.0, 0.2]),
        BiasPoint("reverse subthreshold", [0.0, 0.0, 1.5, 0.0, 0.09]),
        BiasPoint("reverse moderate", [0.0, 0.8, 1.0, 0.0, 0.3]),
        BiasPoint("reverse strong saturation", [0.0, 2.5, 1.5, 0.0, 0.4]),
        BiasPoint("reverse forward-biased body", [0.0, 0.0, 0.4, 0.0, 0.8]),
        BiasPoint("reverse back gate", [0.0, 1.0, 1.5, 1.0, 0.2]),
    ];

    /// The temperature rise the self-heating cases are linearized at, in kelvin.
    const DEL_TEMP: Value = 2.0;

    /// The card with its self-heating node switched on (the corpus cards carry
    /// `RTH0` and `SHMOD = 0`).
    fn self_heating(card: &str) -> String {
        let on = card
            .replacen("SHMOD = 0", "SHMOD = 1", 1)
            .replacen("SHMOD =0", "SHMOD = 1", 1);
        assert_ne!(on, card, "the card names SHMOD 0");
        on
    }

    /// One body connection: its label, the instance terminals, and whether the
    /// card's `RBODY` is zeroed to make a contact an ideal tie.
    struct Body(&'static str, &'static str, bool);

    /// Sweep one FD or PD card over `SOI_POINTS`, both CAPMODs, the body cases,
    /// self-heating off and on, and the integration `gains`.
    fn sweep_soi<D: SoiDevice>(
        family: &str,
        card: &str,
        polarity: &str,
        bodies: &[Body],
        gains: &[(&str, Value)],
    ) -> (usize, Vec<String>) {
        let mut checked = 0;
        let mut lines = Vec::new();
        let sign = if polarity == "n" { 1.0 } else { -1.0 };
        for capmod in ["2", "3"] {
            let card = card
                .replace("CAPMOD = 3", &format!("CAPMOD = {capmod}"))
                .replace("CAPMOD = 2", &format!("CAPMOD = {capmod}"));
            for heated in [false, true] {
                let card = if heated {
                    self_heating(&card)
                } else {
                    card.clone()
                };
                let del_temp = if heated { DEL_TEMP } else { 0.0 };
                for &Body(body, terminals, ideal) in bodies {
                    let card = if ideal {
                        ideal_tie(&card)
                    } else {
                        card.clone()
                    };
                    let mut inst: Instance<D> = build(&deck(family, &card, polarity, terminals));
                    assert_eq!(
                        inst.dev.nodes().temp != 0,
                        heated,
                        "{family} {polarity} {body}: self-heating node"
                    );
                    for point in SOI_POINTS {
                        let point = BiasPoint(point.0, point.1.map(|v| sign * v));
                        let x = solution(&inst, &point, del_temp);
                        for &(dt_label, ag0) in gains {
                            checked += 1;
                            for m in check(&mut inst, &x, ag0) {
                                lines.push(mismatch_line(
                                    &format!(
                                        "{family} {polarity}mos capmod{capmod} sh{} {body:<9} {:<28} {dt_label:<10}",
                                        u8::from(heated),
                                        point.0
                                    ),
                                    &m,
                                ));
                            }
                        }
                    }
                }
            }
        }
        (checked, lines)
    }

    fn ideal_tie(card: &str) -> String {
        let tied = card.replace("RBODY = 1e0 ", "RBODY = 0.0 ");
        assert_ne!(tied, card, "the card names RBODY 1");
        tied
    }

    /// FD's static stamp. Its charge companion is not yet the derivative of its
    /// charges, so the integration gains are not swept here.
    #[test]
    fn fd_static_stamped_jacobian_is_the_derivative_of_the_stamped_residual() {
        let bodies = [
            Body("floating", "d g s e", false),
            Body("contact", "d g s e p", false),
        ];
        let gains = &AG0[..1];
        report(&[
            sweep_soi::<B3SoiFd>("B3SOIFD", FD_N1_CARD, "n", &bodies, gains),
            sweep_soi::<B3SoiFd>("B3SOIFD", FD_P1_CARD, "p", &bodies, gains),
        ]);
    }

    #[test]
    fn pd_stamped_jacobian_is_the_derivative_of_the_stamped_residual() {
        let bodies = [
            Body("floating", "d g s e", false),
            Body("tied", "d g s e p", true),
            Body("resistive", "d g s e p", false),
        ];
        report(&[
            sweep_soi::<B3SoiPd>("B3SOIPD", PD_N1_CARD, "n", &bodies, AG0),
            sweep_soi::<B3SoiPd>("B3SOIPD", PD_P1_CARD, "p", &bodies, AG0),
        ]);
    }

    /// The boundary deck's transient stalled because its CAPMOD=3 charges carried
    /// ~1e-8 relative rounding noise below threshold, which the integration gain
    /// turns into nA of residual. Along every direction the charges must now be
    /// smooth to rounding: second differences over 1 nV steps no larger than 64
    /// ulps of the largest node charge.
    #[test]
    fn boundary_deck_charges_are_smooth_to_rounding() {
        let mut inst = instance(N1_CARD, "n", "floating");
        let x = solution(&inst, &BOUNDARY_DECK_OP, 0.0);
        for (label, node) in [
            ("body", inst.dev.node_body),
            ("back gate", inst.dev.node_e),
            ("gate", inst.dev.node_gate),
        ] {
            let charges: Vec<[Value; 4]> = (-6..=6)
                .map(|k| {
                    let mut xp = x.clone();
                    xp[node - 1] += Value::from(k) * 1.0e-9;
                    inst.dev.update_static_linearization(&xp);
                    let q = inst.dev.charge_at(&xp);
                    [q.qg, q.qb, q.qd, q.qe]
                })
                .collect();
            let largest = charges[6].iter().fold(0.0, |m: Value, q| m.max(q.abs()));
            for (charge, name) in ["qg", "qb", "qd", "qe"].iter().enumerate() {
                let roughness = charges
                    .windows(3)
                    .map(|w| (w[2][charge] - 2.0 * w[1][charge] + w[0][charge]).abs())
                    .fold(0.0, Value::max);
                assert!(
                    roughness <= EPS_EVAL * largest,
                    "{name} along the {label}: second difference {roughness:.3e} C over 1 nV \
                     exceeds 64 ulps of {largest:.3e} C"
                );
            }
        }
    }
}

// ===================== Verilog-A cross-check =====================
//
// An optional Verilog-A BSIM4.8 source, when supplied locally, is a sibling
// implementation rather than the transcription source for the native port.
// This cross-check reports native-vs-VA deltas at the same fixture biases
// without forcing agreement; the hard bound only guards against gross
// divergence.

#[cfg(feature = "veriloga")]
mod bsim4_veriloga_cross_check {
    use rspice_core::device::mosfet::bsim4v8::{
        Bsim4v8, Bsim4v8Bias, Bsim4v8Geometry, Bsim4v8Model,
    };
    use rspice_core::{Engine, Netlist, Value};
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    const T300: Value = 300.15;
    const GMIN: Value = 1e-12;

    const BSIM4_VA_ENV: &str = "RSPICE_BSIM4_VA";

    fn bsim4_va_path() -> Option<PathBuf> {
        if let Some(raw) = std::env::var_os(BSIM4_VA_ENV) {
            let path = PathBuf::from(raw);
            assert!(
                path.is_file(),
                "{BSIM4_VA_ENV} must point at an externally supplied BSIM4 Verilog-A source file: {}",
                path.display()
            );
            return Some(path);
        }

        let fallback = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/veriloga/bsim4.va");
        fallback.is_file().then_some(fallback)
    }

    /// Terminal drain current -i(vd) of the VA model at a bias point, the
    /// exact recipe of tests/veriloga_bsim4_oracle.rs.
    fn va_drain_current(model: &Path, vgs: Value, vds: Value, temp_c: Option<Value>) -> Value {
        let temp_line = match temp_c {
            Some(t) => format!(".options temp={t}\n"),
            None => String::new(),
        };
        let deck = format!(
            "* bsim4 va bias point\n\
             {temp_line}\
             vg g 0 {vgs}\n\
             vd d 0 {vds}\n\
             XM1 d g 0 0 bsim4va l=1e-7 w=1e-6\n\
             .va \"{}\" bsim4va\n\
             .end\n",
            model.display().to_string().replace('\\', "/")
        );
        let netlist = Netlist::parse(&deck).expect("parse");
        let result = Engine::default()
            .run_tran(&netlist, 2e-9, 1e-9)
            .expect("bias point must converge");
        let idx = result
            .branch_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("vd"))
            .expect("vd branch");
        -result.branch_currents[idx]
            .last()
            .copied()
            .expect("samples")
    }

    /// Terminal drain current of the native port with the all-defaults
    /// card (the VA deck gives no model parameters): channel current plus
    /// the reverse drain-junction current, i.e. what -i(vd) measures.
    fn native_drain_current(tnom_k: Value, temp_k: Value, vgs: Value, vds: Value) -> Value {
        let model = Arc::new(Bsim4v8Model::from_params(&HashMap::new(), false, tnom_k));
        let dev = Bsim4v8::new(
            "m1".to_string(),
            model,
            Bsim4v8Geometry {
                l: 0.1e-6,
                w: 1e-6,
                ..Bsim4v8Geometry::default()
            },
            temp_k,
        )
        .expect("native default device");
        let op = dev
            .eval(Bsim4v8Bias { vds, vgs, vbs: 0.0 }, GMIN, false)
            .expect("eval");
        op.cd - op.cbd
    }

    /// Native (C-transcription) vs Verilog-A at the fixture bias points.
    /// Run with `cargo test -p rspice-core --features veriloga --test
    /// device_engine_roundtrip -- --ignored --nocapture` to see the delta
    /// table.
    #[test]
    #[ignore = "requires an externally supplied models/veriloga/bsim4.va and the veriloga engine"]
    fn native_vs_veriloga_bias_points() {
        let Some(model) = bsim4_va_path() else {
            eprintln!("bsim4.va not present; skipping VA cross-check");
            return;
        };

        // (label, vgs, vds, temp_c). The temperature rows mirror the VA
        // fixture's tnom=25 protocol; the rest run at the 27C default.
        let points: [(&str, Value, Value, Option<Value>); 10] = [
            ("idvg deep subthreshold", 0.0, 1.2, None),
            ("idvg subthreshold", 0.3, 1.2, None),
            ("idvg moderate inversion", 0.6, 1.2, None),
            ("idvg onset", 0.9, 1.2, None),
            ("idvg strong inversion", 1.2, 1.2, None),
            ("idvd linear", 1.2, 0.1, None),
            ("idvd saturation onset", 1.2, 0.6, None),
            ("temp -40C", 1.0, 1.2, Some(-40.0)),
            ("temp 27C", 1.0, 1.2, Some(27.0)),
            ("temp 125C", 1.0, 1.2, Some(125.0)),
        ];

        println!(
            "{:<26} {:>13} {:>13} {:>9}",
            "bias", "native", "veriloga", "delta"
        );
        let mut worst: Value = 0.0;
        for (label, vgs, vds, temp_c) in points {
            let (tnom_k, temp_k) = match temp_c {
                // VA temperature fixtures: tnom=25, swept device temp.
                Some(t) => (25.0 + 273.15, t + 273.15),
                None => (T300, T300),
            };
            let native = native_drain_current(tnom_k, temp_k, vgs, vds);
            let va = va_drain_current(&model, vgs, vds, temp_c);
            let delta = (va - native) / native.abs().max(1e-30);
            worst = worst.max(delta.abs());
            println!(
                "{label:<26} {native:>13.6e} {va:>13.6e} {:>+8.2}%",
                delta * 100.0
            );
            // Documented variant gap is 0.6-1% strong inversion, ~5% deep
            // subthreshold; anything past 15% would mean one of the two
            // models is broken, not a default deviation.
            assert!(
                delta.abs() < 0.15,
                "{label}: native {native:.6e} vs VA {va:.6e} ({:+.2}%)",
                delta * 100.0
            );
        }
        println!("worst |delta| = {:.2}%", worst * 100.0);
    }
}
