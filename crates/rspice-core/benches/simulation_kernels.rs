use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rspice_core::{Engine, Netlist};

fn parse_netlist(source: &str) -> Netlist {
    Netlist::parse(source).expect("benchmark fixture should parse")
}

fn resistor_ladder_netlist(section_count: usize) -> Netlist {
    let mut source = String::from("Resistor ladder benchmark\nV1 n0 0 DC 1\n");
    for section in 0..section_count {
        let left = format!("n{section}");
        let right = format!("n{}", section + 1);
        source.push_str(&format!("Rseries{section} {left} {right} 100\n"));
        source.push_str(&format!("Rshunt{section} {right} 0 10k\n"));
    }
    source.push_str(".end\n");
    parse_netlist(&source)
}

fn ac_lowpass_netlist() -> Netlist {
    parse_netlist(
        "AC lowpass benchmark\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 1u\n\
         .end",
    )
}

fn transient_step_netlist() -> Netlist {
    parse_netlist(
        "Transient RC benchmark\n\
         V1 in 0 PULSE(0 1 100u 1n 1n 2m 4m)\n\
         R1 in out 1k\n\
         C1 out 0 1u IC=0\n\
         .tran 10u 2m\n\
         .end",
    )
}

fn pz_lowpass_netlist() -> Netlist {
    parse_netlist(
        "Pole-zero benchmark\n\
         I1 in 0 DC 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 1u\n\
         .end",
    )
}

fn bench_dc_operating_point(c: &mut Criterion) {
    let engine = Engine::default();
    let netlist = resistor_ladder_netlist(512);

    c.bench_function("core_dc_op_resistor_ladder_512", |b| {
        b.iter(|| {
            let result = engine
                .run_dc_op(black_box(&netlist))
                .expect("dc operating point");
            black_box(result.node_voltages.len());
        });
    });
}

fn bench_transient_solver(c: &mut Criterion) {
    let engine = Engine::default();
    let netlist = transient_step_netlist();

    c.bench_function("core_tran_rc_step_2ms", |b| {
        b.iter(|| {
            let result = engine
                .run_tran(black_box(&netlist), 2e-3, 10e-6)
                .expect("transient analysis");
            black_box(result.time.len());
        });
    });
}

fn bench_ac_sweep(c: &mut Criterion) {
    let engine = Engine::default();
    let netlist = ac_lowpass_netlist();
    let frequencies: Vec<f64> = (0..512)
        .map(|index| {
            let ratio = index as f64 / 511.0;
            10.0_f64.powf(0.0 + ratio * 6.0)
        })
        .collect();

    c.bench_function("core_ac_lowpass_512pts", |b| {
        b.iter(|| {
            let result = engine
                .run_ac(black_box(&netlist), black_box(&frequencies))
                .expect("ac sweep");
            black_box(result.len());
        });
    });
}

fn bench_pole_zero(c: &mut Criterion) {
    let engine = Engine::default();
    let netlist = pz_lowpass_netlist();

    c.bench_function("core_pz_lowpass", |b| {
        b.iter(|| {
            let result = engine
                .run_pz(black_box(&netlist), 1, 2)
                .expect("pole-zero analysis");
            black_box((result.poles.len(), result.zeros.len()));
        });
    });
}

fn bench_config() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = simulation_kernels;
    config = bench_config();
    targets = bench_dc_operating_point, bench_transient_solver, bench_ac_sweep, bench_pole_zero
}
criterion_main!(simulation_kernels);
