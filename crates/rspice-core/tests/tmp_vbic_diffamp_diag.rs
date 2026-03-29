use rspice_core::Netlist;
use rspice_core::analysis::IntegrationMethod;
use rspice_core::engine::{ConvergenceConfig, Engine, SimulationConfig};
use rspice_core::testing::{TestRunner, TestRunnerConfig};

struct TestLogger;

impl log::Log for TestLogger {
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            eprintln!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: TestLogger = TestLogger;
static LOGGER_INIT: std::sync::Once = std::sync::Once::new();

fn init_test_logger() {
    LOGGER_INIT.call_once(|| {
        log::set_logger(&LOGGER).expect("install test logger");
        log::set_max_level(log::LevelFilter::Debug);
    });
}

fn diffamp_paths() -> (std::path::PathBuf, std::path::PathBuf) {
    let base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic");
    (base.join("diffamp.cir"), base.join("diffamp.out"))
}

fn load_diffamp_reference_tran() -> Vec<(f64, f64)> {
    let (_, out_path) = diffamp_paths();
    let content = std::fs::read_to_string(&out_path).expect("read diffamp reference");
    let mut in_tran = false;
    let mut samples = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("Index")
            && trimmed.to_ascii_lowercase().contains("time")
            && trimmed.to_ascii_lowercase().contains("v(e1_p)")
        {
            in_tran = true;
            continue;
        }
        if !in_tran {
            continue;
        }
        if trimmed.starts_with("Total elapsed time") {
            break;
        }
        if trimmed.starts_with('-') || trimmed.starts_with("Index") {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let Some(_idx) = parts.next() else {
            continue;
        };
        let Some(time_s) = parts.next() else {
            continue;
        };
        let Some(value_s) = parts.next() else {
            continue;
        };
        let Ok(time) = time_s.parse::<f64>() else {
            continue;
        };
        let Ok(value) = value_s.parse::<f64>() else {
            continue;
        };
        samples.push((time, value));
    }

    samples
}

fn sample_named_voltage(
    result: &rspice_core::engine::TransientResult,
    node_name: &str,
    target_time: f64,
) -> f64 {
    let node_idx = result
        .node_index_named(node_name)
        .expect("node should exist");
    let waveform = &result.voltages[node_idx - 1];
    if result.time.is_empty() || waveform.is_empty() {
        return 0.0;
    }
    if target_time <= result.time[0] {
        return waveform[0];
    }
    if let Some((&last_t, &last_v)) = result.time.last().zip(waveform.last()) {
        if target_time >= last_t {
            return last_v;
        }
    }

    for idx in 1..result.time.len() {
        let t1 = result.time[idx];
        if t1 < target_time {
            continue;
        }
        let t0 = result.time[idx - 1];
        let v0 = waveform[idx - 1];
        let v1 = waveform[idx];
        let alpha = if (t1 - t0).abs() > 0.0 {
            (target_time - t0) / (t1 - t0)
        } else {
            0.0
        };
        return v0 + alpha * (v1 - v0);
    }

    *waveform.last().unwrap_or(&0.0)
}

fn run_temp_diffamp_variant(source: &str) -> rspice_core::testing::TestResult {
    let (_, out_path) = diffamp_paths();
    let temp_dir = std::env::temp_dir().join(format!(
        "rspice-diffamp-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time")
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).expect("create temp diffamp dir");
    std::fs::write(temp_dir.join("diffamp.cir"), source).expect("write temp diffamp deck");
    std::fs::copy(&out_path, temp_dir.join("diffamp.out")).expect("copy diffamp reference");

    let mut cfg = TestRunnerConfig::default();
    cfg.max_time_per_test_ms = 1_200_000;
    let runner = TestRunner::new(temp_dir.clone(), cfg);
    let result = runner.run_test(&temp_dir.join("diffamp.cir"));

    let _ = std::fs::remove_dir_all(&temp_dir);
    result
}

#[test]
#[ignore]
fn debug_vbic_diffamp_transient_logs() {
    init_test_logger();
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let netlist = Netlist::parse(&source).expect("parse diffamp deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-15;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let op = engine.run_dc_op(&netlist).expect("dc op");
    eprintln!(
        "dc key nodes: Q5_C={:.12e} Q6_C={:.12e} I1_N={:.12e} Q9_B={:.12e}",
        op.try_voltage_named("Q5_C").unwrap_or_default(),
        op.try_voltage_named("Q6_C").unwrap_or_default(),
        op.try_voltage_named("I1_N").unwrap_or_default(),
        op.try_voltage_named("Q9_B").unwrap_or_default()
    );

    let tran_with_td = engine.run_tran(&netlist, 5e-9, 1e-10);
    eprintln!("tran_with_td_result={tran_with_td:?}");

    let source_no_td = source.replace(" TD=2e-11", " TD=0");
    let netlist_no_td = Netlist::parse(&source_no_td).expect("parse diffamp deck without TD");
    let tran_no_td = engine.run_tran(&netlist_no_td, 5e-9, 1e-10);
    eprintln!("tran_no_td_result={tran_no_td:?}");
}

#[test]
#[ignore]
fn debug_vbic_diffamp_no_td_only() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let source_no_td = source.replace(" TD=2e-11", " TD=0");
    let netlist = Netlist::parse(&source_no_td).expect("parse diffamp deck without TD");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    eprintln!("starting no-td dc op");
    let op = engine.run_dc_op(&netlist).expect("dc op no-td");
    eprintln!(
        "no-td dc key nodes: Q5_C={:.12e} Q6_C={:.12e} I1_N={:.12e} Q9_B={:.12e}",
        op.try_voltage_named("Q5_C").unwrap_or_default(),
        op.try_voltage_named("Q6_C").unwrap_or_default(),
        op.try_voltage_named("I1_N").unwrap_or_default(),
        op.try_voltage_named("Q9_B").unwrap_or_default()
    );

    eprintln!("starting no-td tran");
    let t0 = std::time::Instant::now();
    let tran = engine.run_tran(&netlist, 1e-9, 1e-9);
    let elapsed = t0.elapsed();
    match tran {
        Ok(result) => {
            eprintln!(
                "no-td tran ok: points={} final_t={:.3e}s elapsed={:.3?}",
                result.time.len(),
                result.time.last().copied().unwrap_or_default(),
                elapsed
            );
        }
        Err(err) => {
            eprintln!("no-td tran err after {:.3?}: {err}", elapsed);
        }
    }
}

#[test]
#[ignore]
fn debug_vbic_fo_three_node_vs_collector_substrate() {
    let base_deck = r#"VBIC Output High-Voltage Reference
V1 V1_P V1_N 0.0
VB V1_N 0 0.75
VC Q1_C 0 4.1
Q1 Q1_C V1_P 0 N1
.OP
.MODEL N1 NPN LEVEL=4
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300
.end
"#;

    let mut config = SimulationConfig::default();
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let netlist_three = Netlist::parse(base_deck).expect("parse 3-node deck");
    let op_three = engine.run_dc_op(&netlist_three).expect("dc op 3-node");
    let vc_idx_three = op_three
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VC"))
        .expect("VC branch in 3-node deck");
    let vb_idx_three = op_three
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VB"))
        .expect("VB branch in 3-node deck");

    let coll_tied_deck = base_deck.replace("Q1 Q1_C V1_P 0 N1", "Q1 Q1_C V1_P 0 Q1_C N1");
    let netlist_four = Netlist::parse(&coll_tied_deck).expect("parse collector-tied deck");
    let op_four = engine
        .run_dc_op(&netlist_four)
        .expect("dc op collector-tied deck");
    let vc_idx_four = op_four
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VC"))
        .expect("VC branch in 4-node deck");
    let vb_idx_four = op_four
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VB"))
        .expect("VB branch in 4-node deck");

    eprintln!(
        "3-node:  -i(vc)={:.12e}, -i(vb)={:.12e}",
        -op_three.branch_currents[vc_idx_three], -op_three.branch_currents[vb_idx_three]
    );
    eprintln!(
        "4-node (sub=collector): -i(vc)={:.12e}, -i(vb)={:.12e}",
        -op_four.branch_currents[vc_idx_four], -op_four.branch_currents[vb_idx_four]
    );

    let no_rth_deck = base_deck.replace(" RTH=300", " RTH=0");
    let netlist_no_rth = Netlist::parse(&no_rth_deck).expect("parse no-rth deck");
    let op_no_rth = engine.run_dc_op(&netlist_no_rth).expect("dc op no-rth");
    let vc_idx_no_rth = op_no_rth
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VC"))
        .expect("VC branch in no-rth deck");
    let vb_idx_no_rth = op_no_rth
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VB"))
        .expect("VB branch in no-rth deck");
    eprintln!(
        "no-RTH: -i(vc)={:.12e}, -i(vb)={:.12e}",
        -op_no_rth.branch_currents[vc_idx_no_rth], -op_no_rth.branch_currents[vb_idx_no_rth]
    );

    let no_avc1_deck = base_deck.replace(" AVC1=2", " AVC1=0");
    let netlist_no_avc1 = Netlist::parse(&no_avc1_deck).expect("parse no-avc1 deck");
    let op_no_avc1 = engine.run_dc_op(&netlist_no_avc1).expect("dc op no-avc1");
    let vc_idx_no_avc1 = op_no_avc1
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VC"))
        .expect("VC branch in no-avc1 deck");
    let vb_idx_no_avc1 = op_no_avc1
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VB"))
        .expect("VB branch in no-avc1 deck");
    eprintln!(
        "no-AVC1: -i(vc)={:.12e}, -i(vb)={:.12e}",
        -op_no_avc1.branch_currents[vc_idx_no_avc1], -op_no_avc1.branch_currents[vb_idx_no_avc1]
    );

    let selft_off_deck = base_deck.replace(" RTH=300", " RTH=300 SELFT=0");
    let netlist_selft_off = Netlist::parse(&selft_off_deck).expect("parse selft-off deck");
    let op_selft_off = engine
        .run_dc_op(&netlist_selft_off)
        .expect("dc op selft-off");
    let vc_idx_selft_off = op_selft_off
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VC"))
        .expect("VC branch in selft-off deck");
    let vb_idx_selft_off = op_selft_off
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VB"))
        .expect("VB branch in selft-off deck");
    eprintln!(
        "SELFT=0: -i(vc)={:.12e}, -i(vb)={:.12e}",
        -op_selft_off.branch_currents[vc_idx_selft_off],
        -op_selft_off.branch_currents[vb_idx_selft_off]
    );

    let selft_on_deck = base_deck.replace(" RTH=300", " RTH=300 SELFT=1");
    let netlist_selft_on = Netlist::parse(&selft_on_deck).expect("parse selft-on deck");
    let op_selft_on = engine.run_dc_op(&netlist_selft_on).expect("dc op selft-on");
    let vc_idx_selft_on = op_selft_on
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VC"))
        .expect("VC branch in selft-on deck");
    let vb_idx_selft_on = op_selft_on
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VB"))
        .expect("VB branch in selft-on deck");
    eprintln!(
        "SELFT=1: -i(vc)={:.12e}, -i(vb)={:.12e}",
        -op_selft_on.branch_currents[vc_idx_selft_on],
        -op_selft_on.branch_currents[vb_idx_selft_on]
    );
}

#[test]
#[ignore]
fn debug_vbic_ceamp_ac_delta() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/CEamp.cir");
    let out_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/CEamp.out");
    let source = std::fs::read_to_string(&deck_path).expect("read ceamp deck");
    let netlist = Netlist::parse(&source).expect("parse ceamp deck");
    let out_content = std::fs::read_to_string(&out_path).expect("read ceamp reference");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-15;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let mut freqs: Vec<f64> = Vec::new();
    let fstart: f64 = 1.0e5;
    let fstop: f64 = 1.0e10;
    let points_per_decade = 100usize;
    let decades = (fstop / fstart).log10();
    let total_points = (decades * points_per_decade as f64).ceil() as usize;
    for i in 0..=total_points {
        let f = fstart * 10f64.powf(i as f64 / points_per_decade as f64);
        if f <= fstop {
            freqs.push(f);
        }
    }

    let ac = engine.run_ac(&netlist, &freqs).expect("run ac");
    let branch_idx = ac
        .first()
        .and_then(|p| {
            p.branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("Vmeas"))
        })
        .expect("Vmeas branch index");

    let mut ours = Vec::with_capacity(ac.len());
    let mut ours_phase = Vec::with_capacity(ac.len());
    for point in &ac {
        let current = point.currents.get(branch_idx).copied().unwrap_or_default();
        let db = 20.0 * current.norm().max(1e-12).log10();
        let phase = current.arg();
        ours.push((point.frequency, db));
        ours_phase.push((point.frequency, phase));
    }

    let source_no_td = source.replace(" TD=2e-11", " TD=0");
    let netlist_no_td = Netlist::parse(&source_no_td).expect("parse ceamp no-td deck");
    let ac_no_td = engine.run_ac(&netlist_no_td, &freqs).expect("run ac no-td");
    let branch_idx_no_td = ac_no_td
        .first()
        .and_then(|p| {
            p.branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("Vmeas"))
        })
        .expect("Vmeas branch index no-td");
    let mut ours_no_td = Vec::with_capacity(ac_no_td.len());
    let mut ours_no_td_phase = Vec::with_capacity(ac_no_td.len());
    for point in &ac_no_td {
        let current = point
            .currents
            .get(branch_idx_no_td)
            .copied()
            .unwrap_or_default();
        let db = 20.0 * current.norm().max(1e-12).log10();
        let phase = current.arg();
        ours_no_td.push((point.frequency, db));
        ours_no_td_phase.push((point.frequency, phase));
    }

    let mut refs = Vec::new();
    for line in out_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !(trimmed
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false))
        {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let _idx = parts.next();
        let Some(freq_s) = parts.next() else {
            continue;
        };
        let Some(db_s) = parts.next() else {
            continue;
        };
        let Ok(freq) = freq_s.parse::<f64>() else {
            continue;
        };
        let Ok(db) = db_s.parse::<f64>() else {
            continue;
        };
        refs.push((freq, db));
    }

    eprintln!(
        "freq_hz, ref_db, our_db, delta_db, our_no_td_db, delta_no_td, our_phase, our_no_td_phase"
    );
    for (freq, ref_db) in refs {
        if !(1.0e5..=5.0e5).contains(&freq) {
            continue;
        }
        let mut our_db = f64::NAN;
        let mut our_no_td_db = f64::NAN;
        let mut best_err = f64::INFINITY;
        for (f, db) in &ours {
            let err = (*f - freq).abs();
            if err < best_err {
                best_err = err;
                our_db = *db;
            }
        }
        let mut best_err_no_td = f64::INFINITY;
        for (f, db) in &ours_no_td {
            let err = (*f - freq).abs();
            if err < best_err_no_td {
                best_err_no_td = err;
                our_no_td_db = *db;
            }
        }
        let mut our_phase = f64::NAN;
        let mut best_phase_err = f64::INFINITY;
        for (f, ph) in &ours_phase {
            let err = (*f - freq).abs();
            if err < best_phase_err {
                best_phase_err = err;
                our_phase = *ph;
            }
        }
        let mut our_no_td_phase = f64::NAN;
        let mut best_phase_no_td_err = f64::INFINITY;
        for (f, ph) in &ours_no_td_phase {
            let err = (*f - freq).abs();
            if err < best_phase_no_td_err {
                best_phase_no_td_err = err;
                our_no_td_phase = *ph;
            }
        }
        eprintln!(
            "{freq:.0}, {ref_db:.9}, {our_db:.9}, {delta:.9}, {our_no_td_db:.9}, {delta_no_td:.9}, {our_phase:.9}, {our_no_td_phase:.9}",
            delta = our_db - ref_db,
            delta_no_td = our_no_td_db - ref_db
        );
    }
}

#[test]
#[ignore]
fn debug_vbic_diffamp_no_td_fullwindow() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let source_no_td = source.replace(" TD=2e-11", " TD=0");
    let netlist = Netlist::parse(&source_no_td).expect("parse diffamp no-td deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-15;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let t0 = std::time::Instant::now();
    let tran = engine.run_tran(&netlist, 1e-6, 1e-8);
    let elapsed = t0.elapsed();
    match tran {
        Ok(result) => {
            eprintln!(
                "no-td full-window tran ok: points={} final_t={:.3e}s elapsed={:.3?}",
                result.time.len(),
                result.time.last().copied().unwrap_or_default(),
                elapsed
            );
        }
        Err(err) => {
            eprintln!("no-td full-window tran err after {:.3?}: {err}", elapsed);
        }
    }
}

#[test]
#[ignore]
fn debug_vbic_diffamp_step_stats_short_window() {
    fn summarize(
        label: &str,
        result: &rspice_core::engine::TransientResult,
        elapsed: std::time::Duration,
    ) {
        let mut min_dt = f64::INFINITY;
        let mut max_dt = 0.0_f64;
        let mut sum_dt = 0.0_f64;
        let mut count = 0usize;
        for pair in result.time.windows(2) {
            let dt = pair[1] - pair[0];
            if dt.is_finite() && dt > 0.0 {
                min_dt = min_dt.min(dt);
                max_dt = max_dt.max(dt);
                sum_dt += dt;
                count += 1;
            }
        }
        let mean_dt = if count > 0 {
            sum_dt / count as f64
        } else {
            0.0
        };
        eprintln!(
            "{label}: points={} final_t={:.3e}s elapsed={:.3?} min_dt={:.3e} mean_dt={:.3e} max_dt={:.3e}",
            result.time.len(),
            result.time.last().copied().unwrap_or_default(),
            elapsed,
            min_dt,
            mean_dt,
            max_dt
        );
    }

    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let netlist_td = Netlist::parse(&source).expect("parse diffamp deck");
    let netlist_no_td =
        Netlist::parse(&source.replace(" TD=2e-11", " TD=0")).expect("parse diffamp no-td deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let t0 = std::time::Instant::now();
    let td = engine.run_tran(&netlist_td, 1e-7, 1e-8).expect("tran td");
    summarize("td-on", &td, t0.elapsed());

    let t1 = std::time::Instant::now();
    let no_td = engine
        .run_tran(&netlist_no_td, 1e-7, 1e-8)
        .expect("tran no-td");
    summarize("td-off", &no_td, t1.elapsed());
}

#[test]
#[ignore]
fn debug_vbic_diffamp_progress_10ns() {
    init_test_logger();
    log::set_max_level(log::LevelFilter::Info);
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let netlist_td = Netlist::parse(&source).expect("parse diffamp deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let t0 = std::time::Instant::now();
    let result = engine
        .run_tran(&netlist_td, 1e-8, 1e-8)
        .expect("tran td 10ns");
    eprintln!(
        "10ns run: points={} final_t={:.3e}s elapsed={:.3?}",
        result.time.len(),
        result.time.last().copied().unwrap_or_default(),
        t0.elapsed()
    );
}

#[test]
#[ignore]
fn debug_vbic_diffamp_progress_100ns() {
    init_test_logger();
    log::set_max_level(log::LevelFilter::Info);
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let netlist_td = Netlist::parse(&source).expect("parse diffamp deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let t0 = std::time::Instant::now();
    let result = engine
        .run_tran(&netlist_td, 1e-7, 1e-8)
        .expect("tran td 100ns");
    eprintln!(
        "100ns run: points={} final_t={:.3e}s elapsed={:.3?}",
        result.time.len(),
        result.time.last().copied().unwrap_or_default(),
        t0.elapsed()
    );
}

#[test]
#[ignore]
fn debug_vbic_diffamp_runtime_selft_toggle_10ns() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let source_selft0 = source.replace(" RTH=300", " RTH=300 SELFT=0");

    let netlist_on = Netlist::parse(&source).expect("parse diffamp selft-on");
    let netlist_off = Netlist::parse(&source_selft0).expect("parse diffamp selft-off");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let t0 = std::time::Instant::now();
    let on = engine
        .run_tran(&netlist_on, 1e-8, 1e-8)
        .expect("tran selft-on");
    let elapsed_on = t0.elapsed();

    let t1 = std::time::Instant::now();
    let off = engine
        .run_tran(&netlist_off, 1e-8, 1e-8)
        .expect("tran selft-off");
    let elapsed_off = t1.elapsed();

    eprintln!(
        "selft-on: points={} elapsed={:.3?} | selft-off: points={} elapsed={:.3?}",
        on.time.len(),
        elapsed_on,
        off.time.len(),
        elapsed_off
    );
}

#[test]
#[ignore]
fn debug_vbic_diffamp_runtime_trapgear_vs_trap_10ns() {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let netlist = Netlist::parse(&source).expect("parse diffamp deck");

    let mut trap_cfg = SimulationConfig::default();
    trap_cfg.max_iterations = trap_cfg.max_iterations.max(1200);
    trap_cfg.convergence_config = ConvergenceConfig::robust();
    trap_cfg.integration_method = IntegrationMethod::Trapezoidal;
    trap_cfg.min_timestep = 1e-12;
    trap_cfg.temperature = 300.15;
    let trap_engine = Engine::new(trap_cfg);

    let mut trapgear_cfg = SimulationConfig::default();
    trapgear_cfg.max_iterations = trapgear_cfg.max_iterations.max(1200);
    trapgear_cfg.convergence_config = ConvergenceConfig::robust();
    trapgear_cfg.integration_method = IntegrationMethod::TrapGear;
    trapgear_cfg.min_timestep = 1e-12;
    trapgear_cfg.temperature = 300.15;
    let trapgear_engine = Engine::new(trapgear_cfg);

    let t0 = std::time::Instant::now();
    let trap = trap_engine
        .run_tran(&netlist, 1e-8, 1e-8)
        .expect("trap diffamp 10ns");
    let trap_elapsed = t0.elapsed();

    let t1 = std::time::Instant::now();
    let trapgear = trapgear_engine
        .run_tran(&netlist, 1e-8, 1e-8)
        .expect("trapgear diffamp 10ns");
    let trapgear_elapsed = t1.elapsed();

    eprintln!(
        "trap: points={} elapsed={:.3?} | trapgear: points={} elapsed={:.3?}",
        trap.time.len(),
        trap_elapsed,
        trapgear.time.len(),
        trapgear_elapsed
    );
}

#[test]
#[ignore]
fn debug_vbic_diffamp_focus_no_td_against_reference() {
    let (deck_path, _) = diffamp_paths();
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let source = source
        .replace(".TRAN 1n 1u 0 10n", ".TRAN 1n 6n 0 10n")
        .replace(".OP\n", "")
        .replace(".AC DEC 25 100k 1G\n", "")
        .replace(".print ac v(e1_p)\n", "")
        .replace(" TD=2e-11", " TD=0");
    let result = run_temp_diffamp_variant(&source);
    eprintln!(
        "no-td vs reference: passed={} error={:?} mismatches={:?}",
        result.passed, result.error, result.mismatches
    );
}

#[test]
#[ignore]
fn debug_vbic_diffamp_focus_selft0_against_reference() {
    let (deck_path, _) = diffamp_paths();
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let source = source
        .replace(".TRAN 1n 1u 0 10n", ".TRAN 1n 21n 0 10n")
        .replace(".OP\n", "")
        .replace(".AC DEC 25 100k 1G\n", "")
        .replace(".print ac v(e1_p)\n", "")
        .replace(" RTH=300", " RTH=300 SELFT=0");
    let result = run_temp_diffamp_variant(&source);
    eprintln!(
        "selft0 vs reference: passed={} error={:?} mismatches={:?}",
        result.passed, result.error, result.mismatches
    );
}

#[test]
#[ignore]
fn debug_vbic_diffamp_reference_samples_td_selft_variants() {
    let (deck_path, _) = diffamp_paths();
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let source_no_td = source.replace(" TD=2e-11", " TD=0");
    let source_selft0 = source.replace(" RTH=300", " RTH=300 SELFT=0");

    let netlist_td = Netlist::parse(&source).expect("parse diffamp deck");
    let netlist_no_td = Netlist::parse(&source_no_td).expect("parse diffamp no-td deck");
    let netlist_selft0 = Netlist::parse(&source_selft0).expect("parse diffamp selft-off deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let td = engine
        .run_tran(&netlist_td, 2.1e-8, 1e-8)
        .expect("tran td 21ns");
    let no_td = engine
        .run_tran(&netlist_no_td, 2.1e-8, 1e-8)
        .expect("tran no-td 21ns");
    let selft0 = engine
        .run_tran(&netlist_selft0, 2.1e-8, 1e-8)
        .expect("tran selft-off 21ns");

    let refs = load_diffamp_reference_tran();
    let probe_times = [1.6e-10, 6.4e-10, 2.56e-9, 5.12e-9, 1.024e-8, 2.024e-8];

    eprintln!("time_s, ref, td_on, td_on_err, td_off, td_off_err, selft0, selft0_err");
    for &target_time in &probe_times {
        let reference = refs
            .iter()
            .find(|(time, _)| (*time - target_time).abs() < 1e-15)
            .map(|(_, value)| *value)
            .expect("reference time should exist");
        let td_value = sample_named_voltage(&td, "E1_P", target_time);
        let no_td_value = sample_named_voltage(&no_td, "E1_P", target_time);
        let selft0_value = sample_named_voltage(&selft0, "E1_P", target_time);
        eprintln!(
            "{target_time:.6e}, {reference:.9e}, {td_value:.9e}, {td_err:.9e}, {no_td_value:.9e}, {no_td_err:.9e}, {selft0_value:.9e}, {selft0_err:.9e}",
            td_err = td_value - reference,
            no_td_err = no_td_value - reference,
            selft0_err = selft0_value - reference,
        );
    }
}

#[test]
#[ignore]
fn debug_vbic_diffamp_reference_samples_td_on_10ns() {
    let (deck_path, _) = diffamp_paths();
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let netlist_td = Netlist::parse(&source).expect("parse diffamp deck");
    let netlist_selft0 = Netlist::parse(&source.replace(" RTH=300", " RTH=300 SELFT=0"))
        .expect("parse selft-off deck");
    let netlist_no_td =
        Netlist::parse(&source.replace(" TD=2e-11", " TD=0")).expect("parse no-td deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let td = engine
        .run_tran(&netlist_td, 1.0e-8, 1.0e-8)
        .expect("tran td 10ns");
    let selft0 = engine
        .run_tran(&netlist_selft0, 1.0e-8, 1.0e-8)
        .expect("tran selft0 10ns");
    let no_td = engine
        .run_tran(&netlist_no_td, 1.0e-8, 1.0e-8)
        .expect("tran no-td 10ns");

    let refs = load_diffamp_reference_tran();
    let probe_times = [1.6e-10, 3.2e-10, 6.4e-10, 1.28e-9, 2.56e-9, 5.12e-9];

    eprintln!("time_s, ref, td_on, td_on_err, selft0, selft0_err, no_td, no_td_err");
    for &target_time in &probe_times {
        let reference = refs
            .iter()
            .find(|(time, _)| (*time - target_time).abs() < 1e-15)
            .map(|(_, value)| *value)
            .expect("reference time should exist");
        let td_value = sample_named_voltage(&td, "E1_P", target_time);
        let selft0_value = sample_named_voltage(&selft0, "E1_P", target_time);
        let no_td_value = sample_named_voltage(&no_td, "E1_P", target_time);
        eprintln!(
            "{target_time:.6e}, {reference:.9e}, {td_value:.9e}, {td_err:.9e}, {selft0_value:.9e}, {selft0_err:.9e}, {no_td_value:.9e}, {no_td_err:.9e}",
            td_err = td_value - reference,
            selft0_err = selft0_value - reference,
            no_td_err = no_td_value - reference,
        );
    }
}

#[test]
#[ignore]
fn debug_vbic_diffamp_step_stats_200ps() {
    init_test_logger();
    log::set_max_level(log::LevelFilter::Warn);
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/diffamp.cir");
    let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
    let netlist = Netlist::parse(&source).expect("parse diffamp deck");

    let mut config = SimulationConfig::default();
    config.max_iterations = config.max_iterations.max(1200);
    config.convergence_config = ConvergenceConfig::robust();
    config.integration_method = IntegrationMethod::Trapezoidal;
    config.min_timestep = 1e-12;
    config.temperature = 300.15;
    let engine = Engine::new(config);

    let t0 = std::time::Instant::now();
    let result = engine
        .run_tran(&netlist, 2e-10, 2e-10)
        .expect("tran diffamp 200ps");
    let elapsed = t0.elapsed();

    let mut min_dt = f64::INFINITY;
    let mut max_dt = 0.0_f64;
    let mut sum_dt = 0.0_f64;
    let mut count = 0usize;
    for pair in result.time.windows(2) {
        let dt = pair[1] - pair[0];
        if dt.is_finite() && dt > 0.0 {
            min_dt = min_dt.min(dt);
            max_dt = max_dt.max(dt);
            sum_dt += dt;
            count += 1;
        }
    }
    let mean_dt = if count > 0 {
        sum_dt / count as f64
    } else {
        0.0
    };
    eprintln!(
        "200ps run: points={} final_t={:.3e}s elapsed={:.3?} min_dt={:.3e} mean_dt={:.3e} max_dt={:.3e}",
        result.time.len(),
        result.time.last().copied().unwrap_or_default(),
        elapsed,
        min_dt,
        mean_dt,
        max_dt
    );
}
