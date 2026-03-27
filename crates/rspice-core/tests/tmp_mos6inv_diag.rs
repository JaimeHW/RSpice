use rspice_core::Netlist;
use rspice_core::analysis::IntegrationMethod;
use rspice_core::device::{MosType, Mosfet, NonlinearDevice};
use rspice_core::engine::{ConvergenceConfig, Engine, SimulationConfig};

fn interpolate(time: &[f64], values: &[f64], x: f64) -> f64 {
    if x <= time[0] {
        return values[0];
    }
    if x >= time[time.len() - 1] {
        return values[values.len() - 1];
    }
    let idx = time.partition_point(|t| *t < x);
    if idx == 0 {
        return values[0];
    }
    let x0 = time[idx - 1];
    let x1 = time[idx];
    let y0 = values[idx - 1];
    let y1 = values[idx];
    if (x1 - x0).abs() < 1e-30 {
        y1
    } else {
        y0 + (y1 - y0) * ((x - x0) / (x1 - x0))
    }
}

fn ngspice_level6_id(
    mos_type: MosType,
    vto: f64,
    gamma: f64,
    gamma1: f64,
    sigma: f64,
    phi: f64,
    kc: f64,
    nc: f64,
    kv: f64,
    nv: f64,
    lambda0: f64,
    lambda1: f64,
    l: f64,
    w: f64,
    ld: f64,
    vgs: f64,
    vds: f64,
    vbs: f64,
) -> f64 {
    let p = match mos_type {
        MosType::Nmos => 1.0,
        MosType::Pmos => -1.0,
    };
    let vgs_m = p * vgs;
    let vds_m = p * vds;
    let vbs_m = p * vbs;
    let vbd_m = vbs_m - vds_m;
    let vgd_m = vgs_m - vds_m;

    let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
    let vdshere = vds_m * mode;
    let vbsvbd = if mode > 0.0 { vbs_m } else { vbd_m };
    let vg_active = if mode > 0.0 { vgs_m } else { vgd_m };

    let sqrt_phi = phi.sqrt();
    let sarg1 = if vbsvbd <= 0.0 {
        (phi - vbsvbd).max(0.0).sqrt()
    } else {
        (sqrt_phi - vbsvbd / (2.0 * sqrt_phi.max(1e-12))).max(0.0)
    };
    let von = p * vto + gamma * (sarg1 - sqrt_phi) - gamma1 * vbsvbd - sigma * vdshere;
    let vgon = vg_active - von;
    if vgon <= 0.0 {
        return 0.0;
    }

    let leff = (l - 2.0 * ld).max(1e-12);
    let betac = kc * w / leff;
    let vdsat = kv * vgon.powf(nv);
    let idsat = betac * vgon.powf(nc);
    let lambda = lambda0 - lambda1 * vbsvbd;
    let mut cdrain = idsat * (1.0 + lambda * vdshere);
    if vdsat > vdshere {
        let vdst = vdshere / vdsat;
        let vdst2 = (2.0 - vdst) * vdst;
        cdrain *= vdst2;
    }
    p * mode * cdrain
}

#[test]
#[ignore]
fn debug_mos6inv_step_sensitivity() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/mos6/mos6inv.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read mos6inv deck");
    let netlist = Netlist::parse(&source).expect("parse mos6inv deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let probes = [
        ("2", 1.473_527e-08),
        ("42", 6.323_527e-08),
        ("42", 6.373_238e-08),
    ];

    for max_step in [5e-10, 1e-10, 5e-11, 2e-11, 1e-11] {
        let result = engine
            .run_tran(&netlist, 150e-9, max_step)
            .unwrap_or_else(|err| panic!("run_tran failed for max_step={max_step:e}: {err:?}"));
        eprintln!("max_step={max_step:.3e} points={}", result.time.len());
        for (node, x) in probes {
            let idx = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(node))
                .unwrap_or_else(|| panic!("missing node {node}"));
            let y = interpolate(&result.time, &result.voltages[idx], x);
            eprintln!("  node={node} x={x:.9e} y={y:.12e}");
        }
    }
}

#[test]
#[ignore]
fn debug_mos6inv_gmin_sensitivity() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/mos6/mos6inv.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read mos6inv deck");
    let netlist = Netlist::parse(&source).expect("parse mos6inv deck");

    let probes = [
        ("2", 1.473_527e-08),
        ("42", 6.323_527e-08),
        ("42", 6.373_238e-08),
    ];

    for gmin_target in [1e-15, 1e-14, 1e-13, 1e-12, 1e-11, 1e-10, 1e-9] {
        let mut config = SimulationConfig::default();
        config.max_iterations = config.max_iterations.max(1200);
        config.convergence_config = ConvergenceConfig::robust();
        config.convergence_config.gmin_target = gmin_target;
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.min_timestep = 1e-12;
        config.temperature = 300.15;
        let engine = Engine::new(config);

        let result = engine
            .run_tran(&netlist, 150e-9, 0.5e-9)
            .unwrap_or_else(|err| {
                panic!("run_tran failed for gmin_target={gmin_target:e}: {err:?}")
            });
        eprintln!("gmin_target={gmin_target:.3e} points={}", result.time.len());
        for (node, x) in probes {
            let idx = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(node))
                .unwrap_or_else(|| panic!("missing node {node}"));
            let y = interpolate(&result.time, &result.voltages[idx], x);
            eprintln!("  node={node} x={x:.9e} y={y:.12e}");
        }
    }
}

#[test]
#[ignore]
fn debug_mos6inv_initial_nodes() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/mos6/mos6inv.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read mos6inv deck");
    let netlist = Netlist::parse(&source).expect("parse mos6inv deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let result = engine
        .run_tran(&netlist, 150e-9, 0.5e-9)
        .expect("run_tran mos6inv");

    eprintln!("t0={:.12e}", result.time[0]);
    for node in [
        "11",
        "12",
        "13",
        "2",
        "21",
        "22",
        "23",
        "31",
        "32",
        "33",
        "41",
        "42",
        "43",
        "5",
        "XNDINV1.12",
        "XNDINV1.13",
        "XNDINV1.14",
        "XNDINV1.22",
        "XNDINV1.23",
        "XNDINV4.12",
        "XNDINV4.13",
        "XNDINV4.14",
        "XNDINV4.22",
        "XNDINV4.23",
    ] {
        let idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("missing node {node}"));
        eprintln!("  node={node} value={:.12e}", result.voltages[idx][0]);
    }
}

#[test]
#[ignore]
fn debug_mos6inv_tight_tolerance_sensitivity() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/mos6/mos6inv.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read mos6inv deck");
    let netlist = Netlist::parse(&source).expect("parse mos6inv deck");

    for (label, voltage_abs, current_abs, rel) in [
        ("default-robust", 1e-6, 1e-12, 1e-3),
        ("tight-1", 1e-9, 1e-15, 1e-6),
        ("tight-2", 1e-12, 1e-18, 1e-8),
    ] {
        let mut config = SimulationConfig::default();
        config.max_iterations = 4000;
        config.convergence_config = ConvergenceConfig::robust();
        config.convergence_config.voltage_abstol = voltage_abs;
        config.convergence_config.current_abstol = current_abs;
        config.convergence_config.voltage_reltol = rel;
        config.convergence_config.residual_reltol = rel;
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.min_timestep = 1e-12;
        config.temperature = 300.15;
        let engine = Engine::new(config);

        let result = engine
            .run_tran(&netlist, 150e-9, 0.5e-9)
            .unwrap_or_else(|err| panic!("run_tran failed for {label}: {err:?}"));

        let probe = |name: &str, x: f64| {
            let idx = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap_or_else(|| panic!("missing node {name}"));
            interpolate(&result.time, &result.voltages[idx], x)
        };

        eprintln!(
            "{label}: t0_v2={:.12e} t0_v42={:.12e} v2@14.73527ns={:.12e} v42@63.23527ns={:.12e}",
            probe("2", 0.0),
            probe("42", 0.0),
            probe("2", 1.473_527e-08),
            probe("42", 6.323_527e-08),
        );
    }
}

#[test]
#[ignore]
fn debug_level6_current_matches_literal_ngspice_equations() {
    fn configured(mos_type: MosType) -> Mosfet {
        let mut m = match mos_type {
            MosType::Nmos => Mosfet::new_nmos("M1".to_string(), 1, 2, 3, 4),
            MosType::Pmos => Mosfet::new_pmos("M1".to_string(), 1, 2, 3, 4),
        }
        .with_level(6);
        match mos_type {
            MosType::Nmos => {
                m.vto = 0.69486;
                m.gamma = 0.60309;
                m.kc = 3.8921e-05;
                m.nc = 1.1739;
                m.kv = 0.91602;
                m.nv = 0.87225;
                m.lambda0 = 0.013333;
                m.lambda1 = 0.0046901;
                m.l = 1.0e-6;
                m.w = 5.0e-6;
                m.ld = 0.1e-6;
            }
            MosType::Pmos => {
                m.vto = -0.60865;
                m.gamma = 0.89213;
                m.kc = 6.42696e-06;
                m.nc = 1.6536;
                m.kv = 0.92145;
                m.nv = 0.88345;
                m.lambda0 = 0.018966;
                m.lambda1 = 0.0084012;
                m.l = 1.2e-6;
                m.w = 5.0e-6;
                m.ld = 0.28e-6;
            }
        }
        m.phi = 1.0;
        m
    }

    for mos_type in [MosType::Nmos, MosType::Pmos] {
        let mut max_abs = 0.0;
        let mut worst = (0.0, 0.0, 0.0, 0.0, 0.0);
        for &vgs in &[-5.0, -2.0, -0.5, 0.0, 0.5, 1.0, 2.5, 5.0] {
            for &vds in &[-5.0, -2.0, -0.5, 0.0, 0.5, 1.0, 2.5, 5.0] {
                for &vbs in &[-2.0, -1.0, -0.2, 0.0, 0.2, 1.0, 2.0] {
                    let mut m = configured(mos_type);
                    let vd = vds;
                    let vs = 0.0;
                    let vg = vgs;
                    let vb = vbs;
                    let voltages = [vd, vg, vs, vb];
                    m.update(&voltages);
                    let actual = m.drain_current();
                    let expected = ngspice_level6_id(
                        mos_type, m.vto, m.gamma, m.gamma1, m.sigma, m.phi, m.kc, m.nc, m.kv, m.nv,
                        m.lambda0, m.lambda1, m.l, m.w, m.ld, vgs, vds, vbs,
                    );
                    let err = (actual - expected).abs();
                    if err > max_abs {
                        max_abs = err;
                        worst = (vgs, vds, vbs, expected, actual);
                    }
                }
            }
        }
        eprintln!(
            "mos_type={mos_type:?} max_abs={max_abs:.12e} worst=(vgs={:.3}, vds={:.3}, vbs={:.3}, expected={:.12e}, actual={:.12e})",
            worst.0, worst.1, worst.2, worst.3, worst.4
        );
    }
}
