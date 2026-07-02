//! Native XSPICE analog code models pinned against ngspice code-model semantics.

use rspice_core::Complex64;
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_core::xspice::{
    AnalogInputConnection, AnalysisType, CmContext, CodeModel, CodeModelRegistry, PortConnection,
    PortType, XspiceInstance, clear_registered_data_files, register_data_file,
};
use std::sync::{Mutex, MutexGuard};

static DATA_FILE_REGISTRY_LOCK: Mutex<()> = Mutex::new(());

struct DataFileRegistryGuard {
    _lock: MutexGuard<'static, ()>,
}

impl DataFileRegistryGuard {
    fn new() -> Self {
        let lock = DATA_FILE_REGISTRY_LOCK
            .lock()
            .expect("lock XSPICE data-file registry test guard");
        clear_registered_data_files().expect("clear XSPICE data-file registry");
        Self { _lock: lock }
    }
}

impl Drop for DataFileRegistryGuard {
    fn drop(&mut self) {
        let _ = clear_registered_data_files();
    }
}

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("operating point solves");
    let idx = op
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from OP result"));
    op.node_voltages[idx]
}

fn op_error(deck: &str) -> String {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::default()
        .run_dc_op(&netlist)
        .expect_err("operating point must fail")
        .to_string()
}

#[test]
fn official_xtradev_reactive_aliases_build_through_native_resolution() {
    let cap_deck = "\
* Official ngspice xtradev capacitor name
acap %hd[n 0] capmod
.model capmod capacitor (c=1n ic=0.25)
.end
";
    let cap_netlist = Netlist::parse(cap_deck).expect("official capacitor deck parses");
    let cap_circuit = Engine::default()
        .build_circuit(&cap_netlist)
        .expect("official capacitor alias builds");
    assert!(
        !cap_circuit.has_xspice_devices(),
        "official capacitor alias should use the native reactive lowering"
    );

    let ind_deck = "\
* Official ngspice xtradev inductor name
aind %gd[n 0] indmod
.model indmod inductor (l=1u ic=0.1)
.end
";
    let ind_netlist = Netlist::parse(ind_deck).expect("official inductor deck parses");
    let ind_circuit = Engine::default()
        .build_circuit(&ind_netlist)
        .expect("official inductor alias builds");
    assert!(
        ind_circuit.has_xspice_devices(),
        "official inductor alias should resolve to the XSPICE conductance-output model"
    );
}

#[test]
fn print_param_types_accepts_official_parameter_channels() {
    let deck = "\
* ngspice print_param_types example contract
v1 in 0 dc 1
vmeas sense 0 dc 0
a_dbg [in %vd(in 0) %i(vmeas) %vnam(vmeas)] dbg
.model dbg print_param_types (
+ integer=7
+ real=2.5
+ complex=<4.0 5.0>
+ string=\"hello\"
+ integer_array=[1 2 3]
+ real_array=[1.25 2.5]
+ complex_array=[<11.0 12.0> <13.0 14.0>]
+ string_array=[alpha beta])
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("print_param_types builds through XSPICE");

    assert!(
        circuit.has_xspice_devices(),
        "print_param_types instance should be registered and instantiated"
    );
}

fn transient_node_series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[idx]
}

fn value_at_time(times: &[f64], values: &[f64], target: f64) -> f64 {
    assert_eq!(
        times.len(),
        values.len(),
        "time/value waveform lengths must match"
    );
    let first_time = *times.first().expect("waveform has samples");
    let first_value = *values.first().expect("waveform has samples");
    if target <= first_time {
        return first_value;
    }

    for (time_pair, value_pair) in times.windows(2).zip(values.windows(2)) {
        let (t0, t1) = (time_pair[0], time_pair[1]);
        if target <= t1 {
            let (v0, v1) = (value_pair[0], value_pair[1]);
            let span = t1 - t0;
            if span.abs() <= f64::EPSILON {
                return v1;
            }
            let alpha = (target - t0) / span;
            return v0 + alpha * (v1 - v0);
        }
    }

    *values.last().expect("waveform has samples")
}

fn ac_voltage(deck: &str, node: &str) -> Complex64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_ac(&netlist, &[1.0e3])
        .expect("ac solves")
        .into_iter()
        .next()
        .expect("one ac result");
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    result.voltages[idx]
}

fn ac_voltage_at(deck: &str, node: &str, freq: f64) -> Complex64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_ac(&netlist, &[freq])
        .expect("ac solves")
        .into_iter()
        .next()
        .expect("one ac result");
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    result.voltages[idx]
}

fn complex_partial(partials: &[(String, Complex64)], name: &str, context: &str) -> Complex64 {
    partials
        .iter()
        .find_map(|(partial_name, value)| partial_name.eq_ignore_ascii_case(name).then_some(*value))
        .unwrap_or_else(|| panic!("{context} partial {name} missing from {partials:?}"))
}

#[test]
fn xspice_tline_ac_partials_match_official_hyperbolic_impedances() {
    let model = rspice_core::xspice::models::GenericTransmissionLine;
    let mut ctx = CmContext::new();
    ctx.set_param("l", 0.125);
    ctx.set_param("z", 75.0);
    ctx.set_param("a", 3.0);

    let frequency = 2.0e9;
    let alpha = (10.0_f64.powf(0.05 * 3.0)).ln() / 2.0;
    let beta = std::f64::consts::TAU * frequency / 299_792_458.0;
    let gamma_l = Complex64::new(alpha, beta) * 0.125;
    let z = Complex64::new(75.0, 0.0);
    let expected_z11 = z / gamma_l.tanh();
    let expected_z21 = z / gamma_l.sinh();

    let in_partials = model.output_input_ac_partials(&ctx, "in", frequency);
    let out_partials = model.output_input_ac_partials(&ctx, "out", frequency);

    assert!(
        (complex_partial(&in_partials, "in", "tline in") - expected_z11).norm() < 1.0e-9,
        "tline AC self partial should be z/tanh(gamma*l), got {in_partials:?}"
    );
    assert!(
        (complex_partial(&in_partials, "out", "tline in") - expected_z21).norm() < 1.0e-9,
        "tline AC transfer partial should be z/sinh(gamma*l), got {in_partials:?}"
    );
    assert!(
        (complex_partial(&out_partials, "out", "tline out") - expected_z11).norm() < 1.0e-9,
        "tline AC output self partial should be z/tanh(gamma*l), got {out_partials:?}"
    );
    assert!(
        (complex_partial(&out_partials, "in", "tline out") - expected_z21).norm() < 1.0e-9,
        "tline AC reverse transfer partial should be z/sinh(gamma*l), got {out_partials:?}"
    );
}

#[test]
fn xspice_tline_transient_uses_delayed_remote_history_after_time_of_flight() {
    let model = rspice_core::xspice::models::GenericTransmissionLine;
    let mut ctx = CmContext::new();
    let impedance = 50.0;
    let delay = 1.0e-9;
    ctx.analysis = AnalysisType::Transient;
    ctx.set_param("l", delay * 299_792_458.0);
    ctx.set_param("z", impedance);
    ctx.set_param("a", 0.0);

    ctx.time = 0.0;
    ctx.set_input_analog("V1sens", 1.0);
    ctx.set_input_analog("V2sens", 2.0);
    ctx.set_input_analog("in", 0.01);
    ctx.set_input_analog("out", 0.02);
    model.evaluate(&mut ctx).expect("tline evaluates at t=0");

    assert!((ctx.output("in") - 3.5).abs() < 1.0e-12);
    assert!((ctx.output("out") - 2.5).abs() < 1.0e-12);

    ctx.time = 0.5e-9;
    ctx.set_input_analog("V1sens", 4.0);
    ctx.set_input_analog("V2sens", 8.0);
    ctx.set_input_analog("in", 0.03);
    ctx.set_input_analog("out", 0.04);
    model
        .evaluate(&mut ctx)
        .expect("tline records pre-flight history sample");

    ctx.time = 1.5e-9;
    ctx.set_input_analog("V1sens", 20.0);
    ctx.set_input_analog("V2sens", 30.0);
    ctx.set_input_analog("in", 0.07);
    ctx.set_input_analog("out", 0.11);
    model
        .evaluate(&mut ctx)
        .expect("tline evaluates delayed transient branch");

    assert!(
        (ctx.output("in") - 13.5).abs() < 1.0e-12,
        "tline input-side output should combine delayed remote V/I with present local current"
    );
    assert!(
        (ctx.output("out") - 11.0).abs() < 1.0e-12,
        "tline output-side output should combine delayed remote V/I with present local current"
    );
}

#[test]
fn xspice_cpline_ac_partials_match_official_even_odd_matrix() {
    let model = rspice_core::xspice::models::CoupledTransmissionLine;
    let mut ctx = CmContext::new();
    ctx.set_param("l", 0.2);
    ctx.set_param("ze", 80.0);
    ctx.set_param("zo", 40.0);
    ctx.set_param("ere", 2.25);
    ctx.set_param("ero", 1.44);
    ctx.set_param("ae", 1.5);
    ctx.set_param("ao", 3.0);

    let frequency = 1.0e9;
    let omega = std::f64::consts::TAU * frequency;
    let length = 0.2;
    let arg_e = Complex64::new(
        10.0_f64.powf(0.05 * 1.5).ln() * length / 2.0,
        omega * length / 299_792_458.0 * 2.25_f64.sqrt(),
    );
    let arg_o = Complex64::new(
        10.0_f64.powf(0.05 * 3.0).ln() * length / 2.0,
        omega * length / 299_792_458.0 * 1.44_f64.sqrt(),
    );
    let ze = Complex64::new(80.0, 0.0);
    let zo = Complex64::new(40.0, 0.0);
    let z11 = zo / (arg_o.tanh() * 2.0) + ze / (arg_e.tanh() * 2.0);
    let z12 = zo / (arg_o.sinh() * 2.0) + ze / (arg_e.sinh() * 2.0);
    let z13 = ze / (arg_e.sinh() * 2.0) - zo / (arg_o.sinh() * 2.0);
    let z14 = ze / (arg_e.tanh() * 2.0) - zo / (arg_o.tanh() * 2.0);

    let p1 = model.output_input_ac_partials(&ctx, "p1", frequency);
    let p3 = model.output_input_ac_partials(&ctx, "p3", frequency);

    assert!((complex_partial(&p1, "p1", "cpline p1") - z11).norm() < 1.0e-9);
    assert!((complex_partial(&p1, "p2", "cpline p1") - z12).norm() < 1.0e-9);
    assert!((complex_partial(&p1, "p3", "cpline p1") - z13).norm() < 1.0e-9);
    assert!((complex_partial(&p1, "p4", "cpline p1") - z14).norm() < 1.0e-9);
    assert!((complex_partial(&p3, "p1", "cpline p3") - z13).norm() < 1.0e-9);
    assert!((complex_partial(&p3, "p2", "cpline p3") - z14).norm() < 1.0e-9);
    assert!((complex_partial(&p3, "p3", "cpline p3") - z11).norm() < 1.0e-9);
    assert!((complex_partial(&p3, "p4", "cpline p3") - z12).norm() < 1.0e-9);
}

#[test]
fn xspice_cpline_transient_uses_delayed_even_odd_modal_history() {
    let model = rspice_core::xspice::models::CoupledTransmissionLine;
    let mut ctx = CmContext::new();
    let delay = 1.0e-9;
    ctx.analysis = AnalysisType::Transient;
    ctx.set_param("l", delay * 299_792_458.0);
    ctx.set_param("ze", 80.0);
    ctx.set_param("zo", 40.0);
    ctx.set_param("ere", 1.0);
    ctx.set_param("ero", 1.0);
    ctx.set_param("ae", 0.0);
    ctx.set_param("ao", 0.0);

    ctx.time = 0.0;
    for (name, value) in [
        ("p1s", 1.0),
        ("p2s", 2.0),
        ("p3s", 3.0),
        ("p4s", 4.0),
        ("p1", 0.01),
        ("p2", 0.02),
        ("p3", 0.03),
        ("p4", 0.04),
    ] {
        ctx.set_input_analog(name, value);
    }
    model.evaluate(&mut ctx).expect("cpline evaluates at t=0");
    for port in ["p1", "p2", "p3", "p4"] {
        assert_eq!(
            ctx.output(port),
            0.0,
            "cpline transient branch is delayed before time of flight"
        );
    }

    ctx.time = 0.5e-9;
    for (name, value) in [
        ("p1s", 4.0),
        ("p2s", 8.0),
        ("p3s", 10.0),
        ("p4s", 2.0),
        ("p1", 0.03),
        ("p2", 0.04),
        ("p3", 0.05),
        ("p4", 0.02),
    ] {
        ctx.set_input_analog(name, value);
    }
    model
        .evaluate(&mut ctx)
        .expect("cpline records pre-flight history sample");

    ctx.time = 1.5e-9;
    for (name, value) in [
        ("p1s", 20.0),
        ("p2s", 30.0),
        ("p3s", 40.0),
        ("p4s", 50.0),
        ("p1", 0.07),
        ("p2", 0.11),
        ("p3", 0.13),
        ("p4", 0.17),
    ] {
        ctx.set_input_analog(name, value);
    }
    model
        .evaluate(&mut ctx)
        .expect("cpline evaluates delayed transient branch");

    assert!((ctx.output("p1") - 19.0).abs() < 1.0e-12);
    assert!((ctx.output("p2") - 15.4).abs() < 1.0e-12);
    assert!((ctx.output("p3") - 13.8).abs() < 1.0e-12);
    assert!((ctx.output("p4") - 25.4).abs() < 1.0e-12);
}

#[test]
fn xspice_waveform_oscillators_accept_official_current_output_ports() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["sine", "square", "triangle"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        XspiceInstance::new(
            format!("a_{model_name}"),
            model,
            vec![
                PortConnection::Analog(1),
                PortConnection::CurrentOutput { pos: 2, neg: 0 },
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| panic!("{model_name} must accept official %id output ports: {err}"));
    }
}

#[test]
fn xspice_waveform_oscillators_accept_and_clamp_frequency_tables_outside_official_bounds() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["sine", "square", "triangle"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        XspiceInstance::new(
            format!("a_{model_name}_bad_freq"),
            model.clone(),
            vec![PortConnection::Analog(1), PortConnection::Analog(2)],
            &[],
            &[],
            &[
                ("cntl_array".to_string(), vec![0.0, 1.0]),
                ("freq_array".to_string(), vec![-1.0, 1.0e3]),
            ],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("{model_name} must accept and clamp freq_array values below the official limit: {err}")
        });

        let err = XspiceInstance::new(
            format!("a_{model_name}_short_table"),
            model,
            vec![PortConnection::Analog(1), PortConnection::Analog(2)],
            &[],
            &[],
            &[
                ("cntl_array".to_string(), vec![0.0]),
                ("freq_array".to_string(), vec![1.0e3]),
            ],
            &[],
        )
        .expect_err("official waveform oscillator table vector lower bound must be enforced");

        assert!(
            err.to_string().contains("at least 2"),
            "{model_name} one-point tables should be rejected like ngspice, got {err}"
        );
    }
}

#[test]
fn waveform_oscillators_accept_descending_control_tables_like_ngspice() {
    let deck = "\
* XSPICE waveform oscillator descending control table oracle
vctrl ctrl 0 dc 0
asine ctrl sine_out sine_osc
asquare ctrl square_out square_osc
atri ctrl tri_out tri_osc
.model sine_osc sine (cntl_array=[1 0] freq_array=[1e9 2e9] out_low=-1 out_high=1)
.model square_osc square (cntl_array=[1 0] freq_array=[1e9 2e9] out_low=-1 out_high=1 duty_cycle=0.5 rise_time=0.01n fall_time=0.01n)
.model tri_osc triangle (cntl_array=[1 0] freq_array=[1e9 2e9] out_low=-1 out_high=1 duty_cycle=0.5)
rsine sine_out 0 1meg
rsquare square_out 0 1meg
rtri tri_out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 0.6e-9, 0.005e-9)
        .expect("descending waveform control tables solve like ngspice");

    let sine = transient_node_series(&result, "sine_out");
    let square = transient_node_series(&result, "square_out");
    let triangle = transient_node_series(&result, "tri_out");

    let sine_quarter = value_at_time(&result.time, sine, 0.125e-9);
    let square_low = value_at_time(&result.time, square, 0.125e-9);
    let triangle_mid = value_at_time(&result.time, triangle, 0.125e-9);

    assert!(
        (sine_quarter - 1.0).abs() < 5.0e-2,
        "ngspice uses the descending first segment as 2 GHz, sine quarter-cycle got {sine_quarter}"
    );
    assert!(
        square_low < -0.9,
        "ngspice square output is still low at the 2 GHz quarter-cycle, got {square_low}"
    );
    assert!(
        triangle_mid.abs() < 8.0e-2,
        "ngspice triangle is near midpoint at the 2 GHz quarter-cycle, got {triangle_mid}"
    );
}

#[test]
fn sine_clamps_frequency_table_entries_before_interpolation_like_ngspice() {
    let deck = "\
* XSPICE sine clamps frequency table entries before interpolation
vctrl ctrl 0 dc 0.5
asine ctrl out sine_osc
.model sine_osc sine (cntl_array=[0 1] freq_array=[-1000 1000] out_low=-1 out_high=1)
rload out 0 1k
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 0.30e-3, 10.0e-6)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let quarter = value_at_time(&result.time, out, 0.25e-3);
    assert!(
        (quarter - std::f64::consts::FRAC_1_SQRT_2).abs() < 3.0e-2,
        "ngspice clamps freq_array[0] to 0 before interpolating to 500 Hz; got {quarter}"
    );
}

#[test]
fn sine_preserves_negative_high_side_frequency_extrapolation_like_ngspice() {
    let deck = "\
* XSPICE sine keeps high-side negative frequency extrapolation
vctrl ctrl 0 dc 2
asine ctrl out sine_osc
.model sine_osc sine (cntl_array=[0 1] freq_array=[1g 1e-16] out_low=-1 out_high=1)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 0.3e-9, 0.025e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let quarter = value_at_time(&result.time, out, 0.25e-9);
    assert!(
        quarter < -0.95,
        "ngspice only clamps low-side nonpositive extrapolated frequency; high-side negative extrapolation should produce a negative quarter-cycle, got {quarter}"
    );
}

#[test]
fn triangle_preserves_negative_high_side_frequency_extrapolation_like_ngspice() {
    let deck = "\
* XSPICE triangle keeps high-side negative frequency extrapolation
vctrl ctrl 0 dc 2
atri ctrl out triangle_osc
.model triangle_osc triangle (cntl_array=[0 1] freq_array=[1g 1e-16] out_low=-1 out_high=1 duty_cycle=0.5)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 0.3e-9, 0.025e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let quarter = value_at_time(&result.time, out, 0.25e-9);
    assert!(
        (quarter + 2.0).abs() < 8.0e-2,
        "ngspice triangle uses the previous negative phase without wrapping here; expected about -2 V at 0.25 ns, got {quarter}"
    );
}

#[test]
fn square_preserves_negative_high_side_frequency_rise_window_like_ngspice() {
    let deck = "\
* XSPICE square keeps high-side negative frequency rise window
vctrl ctrl 0 dc 2
asq ctrl out square_osc
.model square_osc square (cntl_array=[0 1] freq_array=[1g 1e-16] out_low=-1 out_high=1 duty_cycle=0.5 rise_time=1n fall_time=0.01n)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 0.3e-9, 0.025e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let quarter = value_at_time(&result.time, out, 0.25e-9);
    assert!(
        (quarter - 0.5).abs() < 8.0e-2,
        "ngspice square follows the stored negative-frequency rise window here; expected about 0.5 V at 0.25 ns, got {quarter}"
    );
}

#[test]
fn waveform_oscillators_clamp_duty_cycle_outside_official_bounds_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["square", "triangle"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));
        for duty in [-0.25, 1.2] {
            XspiceInstance::new(
                format!("a_{model_name}_{duty}"),
                model.clone(),
                vec![PortConnection::Analog(1), PortConnection::Analog(2)],
                &[(String::from("duty_cycle"), duty)],
                &[],
                &[],
                &[],
            )
            .unwrap_or_else(|err| {
                panic!("{model_name} must clamp duty_cycle={duty} like ngspice: {err}")
            });
        }
    }

    let deck = "\
* XSPICE triangle clamps duty_cycle before computing transient shape
vctrl ctrl 0 dc 0
alow ctrl low tri_low
ahigh ctrl high tri_high
.model tri_low triangle (cntl_array=[0 1] freq_array=[1e9 1e9] out_low=-1 out_high=1 duty_cycle=-0.25)
.model tri_high triangle (cntl_array=[0 1] freq_array=[1e9 1e9] out_low=-1 out_high=1 duty_cycle=1.2)
rlow low 0 1meg
rhigh high 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.0e-9, 0.05e-9)
        .expect("transient solves");
    let low = transient_node_series(&result, "low");
    let high = transient_node_series(&result, "high");

    let low_mid = value_at_time(&result.time, low, 0.5e-9);
    let high_mid = value_at_time(&result.time, high, 0.5e-9);
    assert!(
        low_mid.abs() < 5.0e-2,
        "ngspice clamps duty_cycle below 1e-6 before triangle interpolation; got {low_mid}"
    );
    assert!(
        high_mid.abs() < 5.0e-2,
        "ngspice clamps duty_cycle above 0.999999 before triangle interpolation; got {high_mid}"
    );
}

#[test]
fn square_accepts_unbounded_negative_transition_times_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("square").expect("square is registered");

    XspiceInstance::new(
        "a_square_negative_transition_times",
        model,
        vec![PortConnection::Analog(1), PortConnection::Analog(2)],
        &[
            ("rise_time".to_string(), -0.05e-9),
            ("fall_time".to_string(), -0.05e-9),
        ],
        &[],
        &[],
        &[],
    )
    .expect("ngspice square ifspec leaves rise_time and fall_time unbounded");

    let deck = "\
* XSPICE square negative transition time oracle
vctrl ctrl 0 dc 0
asq ctrl out sq
.model sq square (cntl_array=[0 1] freq_array=[1e9 1e9] out_low=-1 out_high=1 duty_cycle=0.5 rise_time=-0.05n fall_time=-0.05n)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.2e-9, 0.005e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    for target in [0.45e-9, 0.55e-9, 1.05e-9] {
        let got = value_at_time(&result.time, out, target);
        assert!(
            (got + 1.0).abs() < 1.0e-9,
            "ngspice keeps square output low with negative transition times at {target:e}s, got {got}"
        );
    }
}

#[test]
fn xspice_scalar_analog_models_accept_official_current_output_ports() {
    let registry = CodeModelRegistry::with_builtins();
    let cases = [
        (
            "climit",
            vec![
                PortConnection::Analog(1),
                PortConnection::Analog(2),
                PortConnection::Analog(3),
                PortConnection::CurrentOutput { pos: 4, neg: 0 },
            ],
        ),
        (
            "hyst",
            vec![
                PortConnection::Analog(1),
                PortConnection::CurrentOutput { pos: 2, neg: 0 },
            ],
        ),
        (
            "slew",
            vec![
                PortConnection::Analog(1),
                PortConnection::CurrentOutput { pos: 2, neg: 0 },
            ],
        ),
        (
            "astate",
            vec![
                PortConnection::Analog(1),
                PortConnection::CurrentOutput { pos: 2, neg: 0 },
            ],
        ),
        (
            "real_to_v",
            vec![
                PortConnection::Real(1),
                PortConnection::CurrentOutput { pos: 2, neg: 0 },
            ],
        ),
    ];

    for (model_name, connections) in cases {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        XspiceInstance::new(
            format!("a_{model_name}"),
            model,
            connections,
            &[],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| panic!("{model_name} must accept official %id output ports: {err}"));
    }
}

#[test]
fn xspice_delay_accepts_official_differential_output_and_current_control_port() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("delay").expect("delay is registered");

    XspiceInstance::new(
        "a_delay",
        model,
        vec![
            PortConnection::Analog(1),
            PortConnection::Differential(2, 0),
            PortConnection::CurrentProbe {
                pos: 3,
                neg: 0,
                branch_ordinal: 1,
            },
        ],
        &[],
        &[],
        &[],
        &[],
    )
    .unwrap_or_else(|err| {
        panic!("delay must accept official %vd output and current-style cntrl ports: {err}")
    });
}

#[test]
fn xspice_multi_input_pwl_accepts_official_differential_current_ports() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry
        .get("multi_input_pwl")
        .expect("multi_input_pwl is registered");
    let ports = model.ports();
    assert!(
        ports[0]
            .allowed_types
            .contains(&PortType::DifferentialCurrent),
        "multi_input_pwl input must advertise official [vd,id] allowed types"
    );
    assert!(
        ports[1]
            .allowed_types
            .contains(&PortType::DifferentialCurrent),
        "multi_input_pwl output must advertise official [vd,id] allowed types"
    );
    assert!(
        !ports[0].allowed_types.contains(&PortType::Current),
        "multi_input_pwl input must not accept scalar %i"
    );
    assert!(
        !ports[1].allowed_types.contains(&PortType::Current),
        "multi_input_pwl output must not accept scalar %i"
    );

    XspiceInstance::new(
        "a_multi_current",
        model,
        vec![
            PortConnection::TypedAnalogVector(vec![
                AnalogInputConnection::CurrentProbe {
                    pos: 1,
                    neg: 0,
                    branch_ordinal: 1,
                },
                AnalogInputConnection::CurrentProbe {
                    pos: 2,
                    neg: 0,
                    branch_ordinal: 2,
                },
            ]),
            PortConnection::CurrentOutput { pos: 3, neg: 0 },
        ],
        &[],
        &[],
        &[
            ("x".to_string(), vec![0.0, 1.0]),
            ("y".to_string(), vec![0.0, 1.0]),
        ],
        &[],
    )
    .unwrap_or_else(|err| {
        panic!("multi_input_pwl must accept official %id input and output ports: {err}")
    });
}

#[test]
fn xspice_multi_input_pwl_rejects_scalar_current_ports() {
    let deck = "\
* XSPICE multi_input_pwl scalar current rejection
vmon1 in1 0 dc 0
vmon2 in2 0 dc 0
abad [%i(vmon1) %i(vmon2)] %vd(out 0) lut
.model lut multi_input_pwl (x=[0 1] y=[0 1] model=\"and\")
.end
";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let err = Engine::default()
        .build_circuit(&netlist)
        .expect_err("multi_input_pwl must reject scalar %i because official types are [vd,id]")
        .to_string();

    assert!(
        err.contains("does not allow explicit Current"),
        "unexpected error: {err}"
    );
}

#[test]
fn xspice_gain_accepts_official_differential_voltage_input() {
    let deck = "\
* XSPICE gain differential-voltage input
vp p 0 dc 3 ac 1
vn n 0 dc 1 ac -1
again %vd[p n] out amp
.model amp gain (gain=2 in_offset=0.5 out_offset=1)
rload out 0 1meg
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 6.0).abs() < 1.0e-9,
        "gain should accept %vd input and apply offset/gain/output offset, got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 4.0).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "gain AC should use the %vd small-signal input difference, got {ac_out}"
    );
}

#[test]
fn xspice_gain_scalar_current_output_percent_i_drives_node_like_ngspice() {
    let deck = "\
* XSPICE gain scalar current output syntax
v1 in 0 dc 1
again in %i out amp
.model amp gain (gain=1e-3)
rload out 0 1k
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out + 1.0).abs() < 1.0e-9,
        "ngspice scalar %i output should source current from out to ground, got {out}"
    );
}

#[test]
fn xspice_gain_current_output_transient_remains_linear() {
    let deck = "\
* Reduced ngspice polarity.deck regression
v1 in 0 0.0 sin(0 1 1k)
a1 in %i out amp
.model amp gain (gain=10)
rout out 0 1k
.tran 1e-6 3e-5
.end
";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 3.0e-5, 1.0e-6)
        .expect("affine XSPICE current-output transient should solve directly");
    let out = transient_node_series(&result, "out");
    let sample_time = 2.0e-5;
    let got = value_at_time(&result.time, out, sample_time);
    let expected = -10.0 * 1.0e3 * (2.0 * std::f64::consts::PI * 1.0e3 * sample_time).sin();

    assert!(
        (got - expected).abs() < 1.0,
        "gain %i transient output should follow the affine current source, expected {expected}, got {got}"
    );
}

#[test]
fn limit_smooths_and_linearizes_like_ngspice() {
    let missing_bounds = "\
* XSPICE limit requires explicit output bounds
vin in 0 dc 2
alim in out lim
.model lim limit (gain=3 in_offset=1)
rload out 0 1meg
.op
.end
";
    let message = op_error(missing_bounds);
    assert!(
        message.contains("out_lower_limit"),
        "ngspice requires limit out_lower_limit/out_upper_limit; got {message}"
    );

    let deck = "\
* XSPICE limit lower/pass/fraction oracle
vlowin in_low 0 dc -1
vmid in_mid 0 dc 0.4
vsmooth in_smooth 0 dc 0.9 ac 1
alow in_low out_low hard
amid in_mid out_mid hard
asmooth in_smooth out_smooth frac
.model hard limit (gain=1 in_offset=0 out_lower_limit=0 out_upper_limit=1 limit_range=0 fraction=false)
.model frac limit (gain=1 in_offset=0 out_lower_limit=0 out_upper_limit=1 limit_range=0.2 fraction=true)
rlow out_low 0 1meg
rmid out_mid 0 1meg
rsmooth out_smooth 0 1meg
.op
.ac lin 1 1k 1k
.end
";

    let out_low = op_voltage(deck, "out_low");
    let out_mid = op_voltage(deck, "out_mid");
    let out_smooth = op_voltage(deck, "out_smooth");
    let ac_smooth = ac_voltage(deck, "out_smooth");

    assert!(
        out_low.abs() < 1.0e-9,
        "limit lower hard clamp should match ngspice: got {out_low}"
    );
    assert!(
        (out_mid - 0.4).abs() < 1.0e-9,
        "limit linear region should match ngspice: got {out_mid}"
    );
    assert!(
        (out_smooth - 0.8875).abs() < 1.0e-9,
        "limit fraction smoothing should match ngspice: got {out_smooth}"
    );
    assert!(
        (ac_smooth.re - 0.75).abs() < 1.0e-9 && ac_smooth.im.abs() < 1.0e-12,
        "limit AC derivative should match the smoothed corner slope, got {ac_smooth}"
    );
}

#[test]
fn climit_hard_limits_to_controlled_upper_bound_like_ngspice() {
    // ngspice climit:
    //   raw = gain * (in + in_offset)
    //   upper = V(cntl_upper) - upper_delta
    //   lower = V(cntl_lower) + lower_delta
    // With limit_range=0 this is a hard clamp.
    let deck = "\
* XSPICE climit hard upper rail
vin in 0 dc 2
vhi hi 0 dc 1.6
vlo lo 0 dc 0.2
aclip in hi lo out clim
.model clim climit (gain=2 in_offset=0 lower_delta=0.05 upper_delta=0.1 limit_range=0 fraction=0)
rl out 0 1k
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!((out - 1.5).abs() < 1e-9, "climit upper clamp: got {out}");
}

#[test]
fn climit_linearizes_control_limit_inputs_in_ac() {
    let deck = "\
* XSPICE climit upper limit control AC partial
vin in 0 dc 10 ac 0
vhi hi 0 dc 2 ac 1
vlo lo 0 dc 0 ac 0
aclip in hi lo out clim
.model clim climit (gain=1 lower_delta=0 upper_delta=0 limit_range=0.1 fraction=0)
rl out 0 1k
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 2.0).abs() < 1.0e-9,
        "climit should hard-limit to the upper control input, got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 1.0).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "climit AC should include the upper control input partial, got {ac_out}"
    );
}

#[test]
fn climit_negative_limit_range_uses_ngspice_threshold_math() {
    // ngspice does not constrain limit_range to non-negative values. A negative
    // range moves the lower threshold below the controlled lower limit, so this
    // point remains in the linear region instead of clamping to 0 V.
    let deck = "\
* XSPICE climit negative limit_range
vin in 0 dc -0.1
vhi hi 0 dc 1
vlo lo 0 dc 0
aclip in hi lo out clim
.model clim climit (gain=1 in_offset=0 lower_delta=0 upper_delta=0 limit_range=-0.2 fraction=0)
rl out 0 1k
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out + 0.1).abs() < 1e-9,
        "climit negative limit_range should stay linear like ngspice: got {out}"
    );
}

#[test]
fn climit_lower_linear_and_fraction_smoothing_match_ngspice() {
    let deck = "\
* XSPICE climit lower/pass/fraction oracle
vlowin in_low 0 dc -1
vmid in_mid 0 dc 0.4
vsmooth in_smooth 0 dc 0.9
vhi hi 0 dc 1
vlo lo 0 dc 0
alow in_low hi lo out_low hard
amid in_mid hi lo out_mid hard
asmooth in_smooth hi lo out_smooth frac
.model hard climit (gain=1 in_offset=0 lower_delta=0 upper_delta=0 limit_range=0 fraction=0)
.model frac climit (gain=1 in_offset=0 lower_delta=0 upper_delta=0 limit_range=0.2 fraction=1)
rlow out_low 0 1k
rmid out_mid 0 1k
rsmooth out_smooth 0 1k
.op
.end
";

    let out_low = op_voltage(deck, "out_low");
    let out_mid = op_voltage(deck, "out_mid");
    let out_smooth = op_voltage(deck, "out_smooth");

    assert!(
        out_low.abs() < 1e-9,
        "climit lower hard clamp should match ngspice: got {out_low}"
    );
    assert!(
        (out_mid - 0.4).abs() < 1e-9,
        "climit linear region should match ngspice: got {out_mid}"
    );
    assert!(
        (out_smooth - 0.8875).abs() < 1e-9,
        "climit fraction smoothing should match ngspice: got {out_smooth}"
    );
}

#[test]
fn xspice_official_divide_alias_matches_divider() {
    let deck = "\
* XSPICE official divide alias
vnum num 0 dc 1
vden den 0 dc 2
adiv num den out div_alias
.model div_alias divide (out_gain=1 out_offset=0 den_lower_limit=1e-10)
rl out 0 1k
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 0.5).abs() < 1e-9,
        "divide alias should match divider behavior: got {out}"
    );
}

#[test]
fn xspice_divide_applies_official_offsets_gains_and_ac_partials() {
    let deck = "\
* XSPICE official divide offsets/gains
vnum num 0 dc 4 ac 1
vden den 0 dc 1 ac 2
adiv num den out div
.model div divide (num_offset=1 num_gain=2 den_offset=1 den_gain=4 den_lower_limit=1e-10 den_domain=1e-16 fraction=false out_gain=3 out_offset=5)
rload out 0 1meg
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 8.75).abs() < 1.0e-9,
        "divide DC output should apply official offsets/gains before output gain/offset, got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re + 3.0).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "divide AC output should include numerator and denominator partials, got {ac_out}"
    );
}

#[test]
fn xspice_divide_smooths_denominator_lower_limit_like_ngspice() {
    let deck = "\
* XSPICE divide denominator limiting
vnum num 0 dc 2
vzero zero 0 dc 0
vsmooth smooth 0 dc 0.75
vfrac frac 0 dc 9.5
ahard num zero out_hard div_abs
asmooth num smooth out_smooth div_abs
afrac num frac out_frac div_frac
.model div_abs divide (den_lower_limit=1 den_domain=0.5 fraction=false)
.model div_frac divide (den_lower_limit=10 den_domain=0.1 fraction=true)
rh out_hard 0 1meg
rs out_smooth 0 1meg
rf out_frac 0 1meg
.op
.end
";

    let out_hard = op_voltage(deck, "out_hard");
    let out_smooth = op_voltage(deck, "out_smooth");
    let out_frac = op_voltage(deck, "out_frac");

    assert!(
        (out_hard - 2.0).abs() < 1.0e-9,
        "divide should hard-limit zero denominator to positive lower limit, got {out_hard}"
    );
    assert!(
        (out_smooth - (2.0 / 1.03125)).abs() < 1.0e-9,
        "divide should use ngspice parabolic smoothing near the absolute lower limit, got {out_smooth}"
    );
    assert!(
        (out_frac - (2.0 / 10.0625)).abs() < 1.0e-9,
        "divide fraction=true should scale den_domain by den_lower_limit, got {out_frac}"
    );
}

#[test]
fn xspice_divide_clamps_denominator_lower_limit_to_official_minimum() {
    let deck = "\
* XSPICE divide den_lower_limit lower-bound clamp
vnum num 0 dc 2
vden den 0 dc 0
adiv num den out div
.model div divide (den_lower_limit=0 den_domain=1e-16)
rload out 0 1
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 2.0e10).abs() <= 1.0e3,
        "divide should clamp den_lower_limit=0 to 1e-10 like ngspice, got {out}"
    );
}

#[test]
fn xspice_official_int_alias_matches_integrator() {
    let deck = "\
* XSPICE official int alias
vin in 0 dc 0
aint in out int_alias
.model int_alias int (gain=1 out_ic=1.25 out_lower_limit=-10 out_upper_limit=10)
rl out 0 1k
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 1.25).abs() < 1e-9,
        "int alias should match integrator initial output behavior: got {out}"
    );
}

#[test]
fn integrator_transient_partial_matches_ngspice_integration_coefficient() {
    let model = rspice_core::xspice::models::Integrator;
    let mut ctx = CmContext::new();
    ctx.analysis = AnalysisType::Transient;
    ctx.set_param("in_offset", 0.0);
    ctx.set_param("gain", 1.0);
    ctx.set_param("out_ic", 1.0);
    ctx.set_param("out_lower_limit", -10.0);
    ctx.set_param("out_upper_limit", 10.0);
    ctx.set_param("limit_range", 1.0e-6);

    model.init(&mut ctx).expect("int init");
    ctx.set_input_analog("in", 2.0);
    model.evaluate(&mut ctx).expect("int evaluates at t=0");
    assert!(
        (ctx.output("out") - 1.0).abs() < 1.0e-12 && ctx.partial("out").abs() < 1.0e-12,
        "ngspice int starts from out_ic with zero input partial"
    );

    ctx.advance_state();
    ctx.time = 0.25;
    ctx.timestep = 0.25;
    ctx.set_input_analog("in", 4.0);
    model
        .evaluate(&mut ctx)
        .expect("int evaluates transient step");

    let partials = model.output_input_partials(&ctx, "out");
    assert!(
        (ctx.output("out") - 1.75).abs() < 1.0e-12,
        "int output should use trapezoidal integration, got {}",
        ctx.output("out")
    );
    assert_eq!(
        partials.len(),
        1,
        "int should expose one scalar input partial"
    );
    assert!(
        partials[0].0 == "in" && (partials[0].1 - 0.125).abs() < 1.0e-12,
        "ngspice int transient partial should be dt / 2 for gain=1, got {partials:?}"
    );
}

#[test]
fn xspice_official_d_dt_alias_matches_differentiator() {
    let deck = "\
* XSPICE official d_dt alias
vin in 0 dc 0
adiff in out ddt_alias
.model ddt_alias d_dt (gain=1 out_offset=2 out_lower_limit=-10 out_upper_limit=10)
rl out 0 1k
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        out.abs() < 1e-9,
        "ngspice d_dt emits zero derivative output at the initial point before applying out_offset: got {out}"
    );
}

#[test]
fn d_dt_transient_partial_matches_ngspice_gain_over_timestep() {
    let model = rspice_core::xspice::models::Differentiator;
    let mut ctx = CmContext::new();
    ctx.analysis = AnalysisType::Transient;
    ctx.set_param("gain", 1.0e-9);
    ctx.set_param("out_offset", 0.5);
    ctx.set_param("out_lower_limit", -10.0);
    ctx.set_param("out_upper_limit", 10.0);
    ctx.set_param("limit_range", 1.0e-6);

    model.init(&mut ctx).expect("d_dt init");
    ctx.set_input_analog("in", 1.0);
    model.evaluate(&mut ctx).expect("d_dt evaluates at t=0");
    assert!(
        ctx.output("out").abs() < 1.0e-12 && ctx.partial("out").abs() < 1.0e-12,
        "ngspice d_dt starts with zero output and zero input partial"
    );

    ctx.advance_state();
    ctx.time = 1.0e-9;
    ctx.timestep = 1.0e-9;
    ctx.set_input_analog("in", 3.0);
    model
        .evaluate(&mut ctx)
        .expect("d_dt evaluates transient step");

    let partials = model.output_input_partials(&ctx, "out");
    assert!(
        (ctx.output("out") - 2.5).abs() < 1.0e-12,
        "d_dt output should be gain * delta / dt + out_offset, got {}",
        ctx.output("out")
    );
    assert_eq!(
        partials.len(),
        1,
        "d_dt should expose one scalar input partial"
    );
    assert!(
        partials[0].0 == "in" && (partials[0].1 - 1.0).abs() < 1.0e-12,
        "ngspice d_dt transient partial should be gain / dt, got {partials:?}"
    );
}

#[test]
fn int_and_d_dt_require_output_bounds_like_ngspice() {
    let missing_int_bounds = "\
* XSPICE int requires explicit output bounds
vin in 0 dc 0
aint in out imod
.model imod int (gain=1 out_ic=1)
rload out 0 1meg
.op
.end
";
    let message = op_error(missing_int_bounds);
    assert!(
        message.contains("out_lower_limit"),
        "ngspice requires int out_lower_limit/out_upper_limit; got {message}"
    );

    let missing_d_dt_bounds = "\
* XSPICE d_dt requires explicit output bounds
vin in 0 dc 0
adiff in out dmod
.model dmod d_dt (gain=1 out_offset=2)
rload out 0 1meg
.op
.end
";
    let message = op_error(missing_d_dt_bounds);
    assert!(
        message.contains("out_lower_limit"),
        "ngspice requires d_dt out_lower_limit/out_upper_limit; got {message}"
    );
}

#[test]
fn xspice_int_ac_gain_matches_official_gain_over_s() {
    let deck = "\
* XSPICE int AC transfer
vin in 0 dc 0 ac 1
aint in out int_alias
.model int_alias int (gain=2 out_ic=0 out_lower_limit=-10 out_upper_limit=10)
rload out 0 1meg
.ac lin 1 1k 1k
.end
";

    let out = ac_voltage(deck, "out");
    let expected_im = -2.0 / (2.0 * std::f64::consts::PI * 1.0e3);
    assert!(
        out.re.abs() < 1.0e-12 && (out.im - expected_im).abs() < 1.0e-12,
        "int AC gain should be gain/s, got {out}"
    );
}

#[test]
fn xspice_d_dt_ac_gain_matches_official_s_gain() {
    let deck = "\
* XSPICE d_dt AC transfer
vin in 0 dc 0 ac 1
adiff in out ddt_alias
.model ddt_alias d_dt (gain=0.5 out_offset=0 out_lower_limit=-1e9 out_upper_limit=1e9)
rload out 0 1meg
.ac lin 1 1k 1k
.end
";

    let out = ac_voltage(deck, "out");
    let expected_im = 0.5 * 2.0 * std::f64::consts::PI * 1.0e3;
    assert!(
        out.re.abs() < 1.0e-9 && (out.im - expected_im).abs() < 1.0e-6,
        "d_dt AC gain should be s*gain, got {out}"
    );
}

#[test]
fn xspice_gain_linearizes_voltage_output_in_ac() {
    let deck = "\
* XSPICE gain AC linearization
vin in 0 dc 1 ac 1
again in out amp
.model amp gain (gain=3 in_offset=0.25 out_offset=5)
rload out 0 1k
.ac lin 1 1k 1k
.end
";

    let out = ac_voltage(deck, "out");

    assert!(
        (out.re - 3.0).abs() < 1e-9 && out.im.abs() < 1e-12,
        "gain AC output should be the small-signal gain, independent of DC offsets: got {out}"
    );
}

#[test]
fn xspice_summer_linearizes_vector_inputs_in_ac() {
    let deck = "\
* XSPICE summer vector-input AC linearization
v1 in1 0 dc 1 ac 1
v2 in2 0 dc 2 ac 2
asum in1 in2 out sum
.model sum summer (in_gain=[3 4] in_offset=[0.5 -1] out_gain=2 out_offset=1)
rload out 0 1k
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 18.0).abs() < 1.0e-9,
        "summer DC output should apply vector offsets/gains and output gain, got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 22.0).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "summer AC output should include both vector input gains, got {ac_out}"
    );
}

#[test]
fn xspice_mult_uses_official_vector_input_contract_and_ac_partials() {
    let deck = "\
* XSPICE mult vector-input contract
v1 in1 0 dc 1 ac 1
v2 in2 0 dc 2 ac 2
v3 in3 0 dc 3 ac -1
amul in1 in2 in3 out mul
.model mul mult (in_gain=[2 3 4] in_offset=[0.5 -1 0] out_gain=2 out_offset=1)
rload out 0 1meg
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 217.0).abs() < 1.0e-9,
        "mult DC output should use all vector inputs, offsets, gains, and output gain, got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 504.0).abs() < 1.0e-8 && ac_out.im.abs() < 1.0e-12,
        "mult AC output should include every vector input partial, got {ac_out}"
    );
}

#[test]
fn xspice_analog_vector_input_models_reject_ports_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["summer", "mult"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        let err = XspiceInstance::new(
            format!("a_{model_name}_short_in"),
            model,
            vec![
                PortConnection::AnalogVector(vec![1]),
                PortConnection::Analog(2),
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("official analog vector input lower bound must be enforced");

        assert!(
            err.to_string().contains("in") && err.to_string().contains("at least 2"),
            "{model_name} one-element input vector should be rejected like ngspice, got {err}"
        );
    }
}

#[test]
fn xspice_analog_vector_input_models_reject_parameter_vectors_that_do_not_match_input_width() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["summer", "mult"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        let mut instance = XspiceInstance::new(
            format!("a_{model_name}_short_gain"),
            model,
            vec![
                PortConnection::AnalogVector(vec![1, 2]),
                PortConnection::Analog(3),
            ],
            &[],
            &[],
            &[("in_gain".to_string(), vec![3.0])],
            &[],
        )
        .expect("instance construction records vector parameter for init validation");

        let err = instance
            .init()
            .expect_err("official vector parameter bound to input port width must be enforced");
        assert!(
            err.to_string().contains("in_gain")
                && err.to_string().contains("input width")
                && err.to_string().contains("2"),
            "{model_name} one-value in_gain should be rejected like ngspice, got {err}"
        );
    }
}

#[test]
fn xspice_remaining_analog_vector_ports_reject_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();

    let multi = registry
        .get("multi_input_pwl")
        .expect("multi_input_pwl is registered");
    let err = XspiceInstance::new(
        "amulti_short_in",
        multi,
        vec![
            PortConnection::AnalogVector(vec![1]),
            PortConnection::Differential(2, 0),
        ],
        &[],
        &[],
        &[
            ("x".to_string(), vec![0.0, 1.0]),
            ("y".to_string(), vec![0.0, 1.0]),
        ],
        &[],
    )
    .expect_err("official multi_input_pwl input vector lower bound must be enforced");
    assert!(
        err.to_string().contains("in") && err.to_string().contains("at least 2"),
        "multi_input_pwl one-element input vector should be rejected like ngspice, got {err}"
    );

    let spice2poly = registry
        .get("spice2poly")
        .expect("spice2poly is registered");
    let err = XspiceInstance::new(
        "apoly_empty_in",
        spice2poly,
        vec![
            PortConnection::AnalogVector(Vec::new()),
            PortConnection::Differential(1, 0),
        ],
        &[],
        &[],
        &[("coef".to_string(), vec![1.0, 2.0])],
        &[],
    )
    .expect_err("official spice2poly input vector lower bound must be enforced");
    assert!(
        err.to_string().contains("in") && err.to_string().contains("at least 1"),
        "spice2poly empty input vector should be rejected like ngspice, got {err}"
    );
}

#[test]
fn xspice_xfer_omitted_table_defaults_to_unity_like_ngspice() {
    let deck = "\
* XSPICE xfer omitted table default
vin in 0 dc 1 ac 1
axfer in out xf
.model xf xfer
rload out 0 1meg
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 1.0).abs() < 1.0e-9,
        "ngspice xfer default table is unity in operating point, got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 1.0).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "ngspice xfer default table is unity in AC, got {ac_out}"
    );
}

#[test]
fn xspice_xfer_table_stamps_complex_ac_gain() {
    let deck = "\
* XSPICE xfer complex AC table
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (table=[1 0 0 10 20 90 100 0 180] db=true rad=false)
rload out 0 1meg
.end
";

    let low = ac_voltage_at(deck, "out", 1.0);
    let mid = ac_voltage_at(deck, "out", 10.0);
    let high = ac_voltage_at(deck, "out", 100.0);

    assert!(
        (low - Complex64::new(1.0, 0.0)).norm() < 1.0e-9,
        "xfer low-frequency point should be unity real gain, got {low}"
    );
    assert!(
        (mid - Complex64::new(0.0, 10.0)).norm() < 1.0e-8,
        "xfer 20 dB / 90 deg point should be +j10, got {mid}"
    );
    assert!(
        (high - Complex64::new(-1.0, 0.0)).norm() < 1.0e-8,
        "xfer 180 degree point should invert the AC input, got {high}"
    );
}

#[test]
fn xspice_xfer_clamps_span_and_offset_lower_bounds_like_ngspice() {
    let deck = "\
* XSPICE xfer span/offset lower-bound clamp oracle
vin in 0 dc 0 ac 1
axlow in outlow xlow
axoff in outoff xoff
.model xlow xfer (table=[1 0 0] span=2 offset=1 db=true rad=false)
.model xoff xfer (table=[1 0 0] span=3 offset=0 db=true rad=false)
rlo outlow 0 1meg
rfo outoff 0 1meg
.ac lin 1 1 1
.end
";

    let out_low = ac_voltage(deck, "outlow");
    assert!(
        (out_low.re - 1.0).abs() < 1.0e-9 && out_low.im.abs() < 1.0e-12,
        "ngspice clamps span below 3 to 3 before evaluating xfer tables, got {out_low}"
    );

    let out_offset = ac_voltage(deck, "outoff");
    assert!(
        (out_offset.re - 1.0).abs() < 1.0e-9 && out_offset.im.abs() < 1.0e-12,
        "ngspice clamps offset below 1 to 1 before evaluating xfer tables, got {out_offset}"
    );
}

#[test]
fn xspice_xfer_accepts_single_negative_frequency_row_like_ngspice() {
    let deck = "\
* XSPICE xfer single-row negative-frequency oracle
vin in 0 dc 1
axfer in out xf
.model xf xfer (table=[-1 2 0] db=false rad=false)
rload out 0 1meg
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 2.0).abs() < 1.0e-9,
        "single-row xfer tables with a negative frequency should behave like ngspice's constant gain, got {out}"
    );
}

#[test]
fn xspice_xfer_malformed_table_rows_disable_gain_like_ngspice() {
    let deck = "\
* XSPICE xfer malformed table
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (table=[1 0 10 20 90] span=3 offset=1)
rload out 0 1meg
.end
";

    let out = ac_voltage(deck, "out");

    assert!(
        out.norm() < 1.0e-12,
        "ngspice returns from malformed inline xfer tables without stamping gain, got {out}"
    );
}

#[test]
fn xspice_xfer_rejects_table_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("xfer").expect("xfer is registered");

    let err = XspiceInstance::new(
        "axfer_short_table",
        model,
        vec![PortConnection::Analog(1), PortConnection::Analog(2)],
        &[],
        &[],
        &[("table".to_string(), vec![1.0, 0.0])],
        &[],
    )
    .expect_err("official xfer table vector lower bound must be enforced");

    assert!(
        err.to_string().contains("at least 3"),
        "xfer table with fewer than one full row should be rejected like ngspice, got {err}"
    );
}

#[test]
fn xspice_xfer_reads_touchstone_ri_file() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "ri"
    ));
    std::fs::write(
        &path,
        "\
# Hz S RI R 50
1 1 0
2 0 1
",
    )
    .expect("write touchstone fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer touchstone RI file
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=3 offset=1)
rload out 0 1meg
.end
"
    );

    let low = ac_voltage_at(&deck, "out", 1.0);
    let high = ac_voltage_at(&deck, "out", 2.0);
    let _ = std::fs::remove_file(&path);

    assert!(
        (low - Complex64::new(1.0, 0.0)).norm() < 1.0e-9,
        "Touchstone first RI row should be unity real gain, got {low}"
    );
    assert!(
        (high - Complex64::new(0.0, 1.0)).norm() < 1.0e-9,
        "Touchstone second RI row should be +j gain, got {high}"
    );
}

#[test]
fn xspice_xfer_file_accepts_numeric_prefix_on_final_value_like_ngspice() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "numeric_prefix"
    ));
    std::fs::write(
        &path,
        "\
# Hz S RI R 50
1 1 0v
",
    )
    .expect("write numeric-prefix touchstone fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer Touchstone numeric-prefix compatibility
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=3 offset=1)
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 1.0);
    let _ = std::fs::remove_file(&path);

    assert!(
        (out - Complex64::new(1.0, 0.0)).norm() < 1.0e-9,
        "Touchstone final value should allow a numeric prefix like ngspice sscanf, got {out}"
    );
}

#[test]
fn xspice_xfer_file_ignores_touchstone_frequency_units_like_ngspice() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "unit"
    ));
    std::fs::write(
        &path,
        "\
# Hz S RI R 50
1 1 0
2 0 1
",
    )
    .expect("write touchstone fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer Touchstone frequency unit compatibility
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=3 offset=1)
rload out 0 1meg
.end
"
    );

    let low = ac_voltage_at(&deck, "out", 1.0);
    let high = ac_voltage_at(&deck, "out", 1.0e6);
    let _ = std::fs::remove_file(&path);

    assert!(
        (low - Complex64::new(1.0, 0.0)).norm() < 1.0e-9,
        "ngspice ignores Touchstone frequency units for xfer files, so 1 Hz should hit row one, got {low}"
    );
    assert!(
        (high - Complex64::new(0.0, 1.0)).norm() < 1.0e-9,
        "ngspice ignores Touchstone frequency units for xfer files, so 1 MHz should clamp to row two, got {high}"
    );
}

#[test]
fn xspice_xfer_file_option_tokens_are_case_sensitive_like_ngspice() {
    let ri_path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "lower_ri"
    ));
    let db_path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "lower_db"
    ));
    std::fs::write(
        &ri_path,
        "\
# Hz S ri R 50
1 1 2
",
    )
    .expect("write lowercase RI touchstone fixture");
    std::fs::write(
        &db_path,
        "\
# Hz S db R 50
1 6 0
",
    )
    .expect("write lowercase DB touchstone fixture");
    let ri_file = ri_path.to_string_lossy().replace('\\', "/");
    let db_file = db_path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer Touchstone option token case compatibility
vin in 0 dc 0 ac 1
axri in outri xri
axdb in outdb xdb
.model xri xfer (file=\"{ri_file}\" span=3 offset=1)
.model xdb xfer (file=\"{db_file}\" span=3 offset=1)
rri outri 0 1meg
rdb outdb 0 1meg
.end
"
    );

    let lower_ri = ac_voltage_at(&deck, "outri", 1.0);
    let lower_db = ac_voltage_at(&deck, "outdb", 1.0);
    let _ = std::fs::remove_file(&ri_path);
    let _ = std::fs::remove_file(&db_path);

    assert!(
        (lower_ri - Complex64::from_polar(1.0, 2.0_f64.to_radians())).norm() < 1.0e-9,
        "lowercase ri must be ignored like ngspice and treated as magnitude/degree, got {lower_ri}"
    );
    assert!(
        (lower_db - Complex64::new(6.0, 0.0)).norm() < 1.0e-9,
        "lowercase db must be ignored like ngspice and leave magnitude linear, got {lower_db}"
    );
}

#[test]
fn xspice_xfer_file_missing_option_hash_returns_without_gain_like_ngspice() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "leading_option"
    ));
    std::fs::write(&path, " # Hz S RI R 50\n1 1 0\n")
        .expect("write leading whitespace option fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer leading option whitespace
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=3 offset=1)
rload out 0 1meg
.end
"
    );

    let out = ac_voltage(&deck, "out");
    let _ = std::fs::remove_file(&path);

    assert!(
        out.norm() < 1.0e-12,
        "leading whitespace before the Touchstone option-line hash should make ngspice return without stamping gain, got {out:?}"
    );
}

#[test]
fn xspice_xfer_file_ignores_data_lines_with_leading_junk_like_ngspice() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "leading_junk"
    ));
    std::fs::write(
        &path,
        "\
# Hz S RI R 50
junk 1 1 0
2 0 1
",
    )
    .expect("write leading junk data fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer leading junk data line
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=3 offset=1)
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 1.0);
    let _ = std::fs::remove_file(&path);

    assert!(
        (out - Complex64::new(0.0, 1.0)).norm() < 1.0e-9,
        "ngspice ignores xfer file data lines that do not start with a number, got {out}"
    );
}

#[test]
fn xspice_xfer_file_ignores_incomplete_trailing_data_like_ngspice() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "incomplete"
    ));
    std::fs::write(
        &path,
        "\
# Hz S RI R 50
1 1 0
2 0
",
    )
    .expect("write incomplete trailing data fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer incomplete trailing file data
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=3 offset=1)
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 10.0);
    let _ = std::fs::remove_file(&path);

    assert!(
        (out - Complex64::new(1.0, 0.0)).norm() < 1.0e-9,
        "ngspice keeps complete xfer file rows before incomplete trailing data, got {out}"
    );
}

#[test]
fn xspice_xfer_file_accepts_negative_and_unordered_frequencies_like_ngspice() {
    let negative_path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "negative"
    ));
    let unordered_path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "unordered"
    ));
    std::fs::write(
        &negative_path,
        "\
# Hz S RI R 50
-1 2 0
",
    )
    .expect("write negative frequency fixture");
    std::fs::write(
        &unordered_path,
        "\
# Hz S RI R 50
2 2 0
1 1 0
",
    )
    .expect("write unordered frequency fixture");
    let negative_file = negative_path.to_string_lossy().replace('\\', "/");
    let unordered_file = unordered_path.to_string_lossy().replace('\\', "/");
    let negative_deck = format!(
        "\
* XSPICE xfer negative file frequency
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{negative_file}\" span=3 offset=1)
rload out 0 1meg
.end
"
    );
    let unordered_deck = format!(
        "\
* XSPICE xfer unordered file frequencies
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{unordered_file}\" span=3 offset=1)
rload out 0 1meg
.end
"
    );

    let negative_out = ac_voltage_at(&negative_deck, "out", 1.0);
    let unordered_out = ac_voltage_at(&unordered_deck, "out", 1.5);
    let _ = std::fs::remove_file(&negative_path);
    let _ = std::fs::remove_file(&unordered_path);

    assert!(
        (negative_out - Complex64::new(2.0, 0.0)).norm() < 1.0e-9,
        "ngspice accepts single-row xfer files with negative frequency, got {negative_out}"
    );
    assert!(
        (unordered_out - Complex64::new(2.0, 0.0)).norm() < 1.0e-9,
        "ngspice accepts unordered xfer file frequencies and uses the first row below it, got {unordered_out}"
    );
}

#[test]
fn xspice_xfer_file_explicit_ri_overrides_touchstone_option_line_like_ngspice() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s1p",
        std::process::id(),
        "ri_override"
    ));
    std::fs::write(
        &path,
        "\
# MHz S MA R 50
1 1 2
",
    )
    .expect("write touchstone fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer explicit file-format override
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=3 offset=1 r_i=true)
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 1.0e6);
    let _ = std::fs::remove_file(&path);

    assert!(
        (out - Complex64::new(1.0, 2.0)).norm() < 1.0e-9,
        "explicit r_i=true must override the Touchstone option line like ngspice, got {out}"
    );
}

#[test]
fn xspice_xfer_file_ignores_touchstone_two_port_noise_rows_like_ngspice() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s2p",
        std::process::id(),
        "noise"
    ));
    std::fs::write(
        &path,
        "\
# Hz S MA R 50
1 2 0 9 0 9 0 9 0
2 0.1 0 1 0
",
    )
    .expect("write touchstone fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer two-port Touchstone noise row
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=9 offset=1 db=false)
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 10.0);
    let _ = std::fs::remove_file(&path);

    assert!(
        (out - Complex64::new(2.0, 0.0)).norm() < 1.0e-9,
        "five-column two-port noise rows must be ignored like ngspice, got {out}"
    );
}

#[test]
fn xspice_xfer_file_reads_at_most_nine_values_per_line_like_ngspice() {
    let path = std::env::temp_dir().join(format!(
        "rspice_xfer_touchstone_{}_{}.s2p",
        std::process::id(),
        "line_limit"
    ));
    std::fs::write(
        &path,
        "\
# Hz S RI R 50
1 1 0 9 0 9 0 9 0 2 0 1 9 0 9 0 9 0
",
    )
    .expect("write touchstone fixture");
    let file = path.to_string_lossy().replace('\\', "/");
    let deck = format!(
        "\
* XSPICE xfer long Touchstone data line
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"{file}\" span=9 offset=1)
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 10.0);
    let _ = std::fs::remove_file(&path);

    assert!(
        (out - Complex64::new(1.0, 0.0)).norm() < 1.0e-9,
        "ngspice reads at most nine values from each xfer file data line, got {out}"
    );
}

#[test]
fn xspice_xfer_reads_registered_virtual_touchstone_file() {
    let _guard = DataFileRegistryGuard::new();
    register_data_file(
        "virtual://xfer/ri.s1p",
        "\
# MHz S RI R 50
1 1 0
2 0 1
",
    )
    .expect("register virtual Touchstone data");
    let deck = "\
* XSPICE xfer virtual Touchstone file
vin in 0 dc 0 ac 1
axfer in out xf
.model xf xfer (file=\"virtual://xfer/ri.s1p\" span=3 offset=1)
rload out 0 1meg
.end
";

    let low = ac_voltage_at(deck, "out", 1.0);
    let high = ac_voltage_at(deck, "out", 2.0);

    assert!(
        (low - Complex64::new(1.0, 0.0)).norm() < 1.0e-9,
        "virtual Touchstone first RI row should be unity real gain, got {low}"
    );
    assert!(
        (high - Complex64::new(0.0, 1.0)).norm() < 1.0e-9,
        "virtual Touchstone second RI row should be +j gain, got {high}"
    );
}

#[test]
fn xspice_s_xfer_lowpass_stamps_complex_ac_gain() {
    let wc = 2.0 * std::f64::consts::PI * 1.0e3;
    let deck = format!(
        "\
* XSPICE s_xfer low-pass AC
vin in 0 dc 0 ac 1
af in out filt
.model filt s_xfer (gain=1 num_coeff=[1] den_coeff=[1 1] denormalized_freq={wc})
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 1.0e3);
    let expected = Complex64::new(0.5, -0.5);
    assert!(
        (out - expected).norm() < 1.0e-8,
        "s_xfer low-pass corner should be 1/(1+j), got {out}"
    );
}

#[test]
fn xspice_s_xfer_highpass_uses_descending_coefficient_order() {
    let wc = 2.0 * std::f64::consts::PI * 1.0e3;
    let deck = format!(
        "\
* XSPICE s_xfer high-pass AC
vin in 0 dc 0 ac 1
af in out filt
.model filt s_xfer (gain=1 num_coeff=[1 0] den_coeff=[1 1] denormalized_freq={wc})
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 1.0e3);
    let expected = Complex64::new(0.5, 0.5);
    assert!(
        (out - expected).norm() < 1.0e-8,
        "s_xfer high-pass corner should be j/(1+j), got {out}"
    );
}

#[test]
fn xspice_s_xfer_normalizes_nonunity_highest_denominator_coefficient_like_ngspice() {
    let deck = "\
* XSPICE s_xfer denominator normalization
vin in 0 dc 0 ac 1
af in out filt
.model filt s_xfer (gain=4 num_coeff=[2] den_coeff=[2 2])
rload out 0 1meg
.end
";

    let out = ac_voltage_at(deck, "out", 1.0 / (2.0 * std::f64::consts::PI));
    let expected = Complex64::new(2.0, -2.0);
    assert!(
        (out - expected).norm() < 1.0e-8,
        "ngspice divides denominator coefficients and gain by the highest-order denominator coefficient, got {out}"
    );
}

#[test]
fn xspice_s_xfer_order_zero_uses_normalized_feedthrough_without_state() {
    let deck = "\
* XSPICE s_xfer order-zero normalized feedthrough
vin in 0 dc 0.5 ac 1
af in out filt
.model filt s_xfer (gain=6 num_coeff=[3] den_coeff=[2])
rload out 0 1meg
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 4.5).abs() < 1.0e-9,
        "order-zero s_xfer OP should use gain*num[0]/den[0] after normalization, got {out}"
    );

    let ac_out = ac_voltage_at(deck, "out", 1.0e3);
    assert!(
        (ac_out - Complex64::new(9.0, 0.0)).norm() < 1.0e-9,
        "order-zero s_xfer AC gain should use gain*num[0]/den[0] after normalization, got {ac_out}"
    );

    let model = rspice_core::xspice::models::SXfer;
    let mut ctx = CmContext::new();
    ctx.analysis = AnalysisType::Transient;
    ctx.timestep = 1.0e-9;
    ctx.set_param("gain", 6.0);
    ctx.set_real_vector_param("num_coeff", vec![3.0]);
    ctx.set_real_vector_param("den_coeff", vec![2.0]);

    model.init(&mut ctx).expect("s_xfer init");
    ctx.set_input_analog("in", 0.5);
    model.evaluate(&mut ctx).expect("s_xfer evaluates");

    let partials = model.output_input_partials(&ctx, "out");
    assert!(
        (ctx.output("out") - 4.5).abs() < 1.0e-12,
        "order-zero s_xfer transient output should be pure normalized feedthrough, got {}",
        ctx.output("out")
    );
    assert!(
        partials == vec![("in".to_string(), 9.0)],
        "order-zero s_xfer transient partial should match the normalized feedthrough gain, got {partials:?}"
    );
}

#[test]
fn xspice_s_xfer_accepts_negative_denormalized_freq_like_ngspice() {
    let wc = -2.0 * std::f64::consts::PI;
    let deck = format!(
        "\
* XSPICE s_xfer negative denormalized_freq AC
vin in 0 dc 0 ac 1
af in out filt
.model filt s_xfer (gain=1 num_coeff=[1] den_coeff=[1 1] denormalized_freq={wc})
rload out 0 1meg
.end
"
    );

    let out = ac_voltage_at(&deck, "out", 1.0);
    let expected = Complex64::new(0.5, 0.5);
    assert!(
        (out - expected).norm() < 1.0e-8,
        "ngspice accepts negative denormalized_freq and returns 0.5+j0.5, got {out}"
    );
}

#[test]
fn xspice_s_xfer_dc_uses_time_zero_integrator_state_like_ngspice() {
    let deck = "\
* XSPICE s_xfer DC time-zero integrator topology
vin in 0 dc 1
alow in out_low lowpass
aic in out_ic lowpass_ic
ahigh in out_high highpass
.model lowpass s_xfer (gain=1 num_coeff=[1] den_coeff=[1 1])
.model lowpass_ic s_xfer (gain=1 num_coeff=[1] den_coeff=[1 1] int_ic=[2])
.model highpass s_xfer (gain=1 num_coeff=[1 0] den_coeff=[1 1])
rlo out_low 0 1meg
ric out_ic 0 1meg
rhi out_high 0 1meg
.op
.end
";

    let low = op_voltage(deck, "out_low");
    let initial_condition = op_voltage(deck, "out_ic");
    let high = op_voltage(deck, "out_high");

    assert!(
        low.abs() < 1.0e-12,
        "ngspice s_xfer DC low-pass starts from the default integrator state, got {low}"
    );
    assert!(
        (initial_condition - 2.0).abs() < 1.0e-12,
        "ngspice s_xfer DC low-pass uses int_ic as the initial integrator output, got {initial_condition}"
    );
    assert!(
        (high - 1.0).abs() < 1.0e-12,
        "ngspice s_xfer DC high-pass includes the time-zero pseudo-input term, got {high}"
    );
}

#[test]
fn xspice_s_xfer_improper_transfer_order_returns_without_gain_like_ngspice() {
    let deck = "\
* XSPICE s_xfer improper transfer function
vin in 0 dc 0 ac 1
af in out filt
.model filt s_xfer (num_coeff=[1 0 0] den_coeff=[1 1])
rload out 0 1meg
.end
";

    let out = ac_voltage(deck, "out");
    assert!(
        out.norm() < 1.0e-12,
        "ngspice returns from s_xfer without stamping gain when numerator order is too high, got {out:?}"
    );
}

#[test]
fn xspice_s_xfer_rejects_coeff_vectors_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("s_xfer").expect("s_xfer is registered");

    for param_name in ["num_coeff", "den_coeff"] {
        let err = XspiceInstance::new(
            format!("a_s_xfer_empty_{param_name}"),
            model.clone(),
            vec![PortConnection::Analog(1), PortConnection::Analog(2)],
            &[],
            &[],
            &[(param_name.to_string(), Vec::new())],
            &[],
        )
        .expect_err("official s_xfer coefficient vector lower bound must be enforced");

        assert!(
            err.to_string().contains(param_name) && err.to_string().contains("at least 1"),
            "s_xfer explicit empty {param_name} vector should be rejected like ngspice, got {err}"
        );
    }
}

#[test]
fn xspice_s_xfer_rejects_missing_required_coeff_vectors_at_construction() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("s_xfer").expect("s_xfer is registered");

    for (case, real_vectors) in [
        ("num_coeff", vec![("den_coeff".to_string(), vec![1.0])]),
        ("den_coeff", vec![("num_coeff".to_string(), vec![1.0])]),
    ] {
        let err = XspiceInstance::new(
            format!("a_s_xfer_missing_{case}"),
            model.clone(),
            vec![PortConnection::Analog(1), PortConnection::Analog(2)],
            &[],
            &[],
            &real_vectors,
            &[],
        )
        .expect_err("official s_xfer coefficient arrays have no defaults");

        assert!(
            err.to_string().contains(case),
            "missing s_xfer {case} should be rejected at construction like ngspice, got {err}"
        );
    }
}

#[test]
fn xspice_s_xfer_lowpass_transient_step_response_is_stable() {
    let wc = 2.0 * std::f64::consts::PI * 1.0e3;
    let tau = 1.0 / wc;
    let deck = format!(
        "\
* XSPICE s_xfer low-pass transient
vin in 0 pulse(0 1 0 1n 1n 1 2)
af in out filt
.model filt s_xfer (gain=1 num_coeff=[1] den_coeff=[1 1] denormalized_freq={wc})
rload out 0 1meg
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 3.0 * tau, tau / 200.0)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    for target in [0.5 * tau, tau, 2.0 * tau] {
        let got = value_at_time(&result.time, out, target);
        let expected = 1.0 - (-target / tau).exp();
        assert!(
            (got - expected).abs() < 0.025,
            "s_xfer low-pass step at {target:e}s: got {got}, expected {expected}"
        );
    }
}

#[test]
fn hyst_preserves_rising_and_falling_history_like_ngspice() {
    let deck = "\
* XSPICE hyst rising/falling branch oracle
vin in 0 pwl(0 0 1n 0.5 2n 1.3 3n 0.5 4n -0.3)
ahyst in out hmod
.model hmod hyst (in_low=0 in_high=1 hyst=0.2 out_lower_limit=0 out_upper_limit=10 input_domain=0.01 fraction=false)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 4.0e-9, 25.0e-12)
        .expect("transient solves");

    let out = transient_node_series(&result, "out");
    let rising_mid = value_at_time(&result.time, out, 1.0e-9);
    let high = value_at_time(&result.time, out, 2.0e-9);
    let falling_mid = value_at_time(&result.time, out, 3.0e-9);
    let reset_low = value_at_time(&result.time, out, 4.0e-9);

    assert!(
        (rising_mid - 3.0).abs() < 0.15,
        "rising branch at input 0.5 should be near 3 V, got {rising_mid}"
    );
    assert!(
        (high - 10.0).abs() < 0.15,
        "hyst should switch to the upper branch after input exceeds in_high+hyst, got {high}"
    );
    assert!(
        (falling_mid - 7.0).abs() < 0.15,
        "falling branch at the same input 0.5 should be near 7 V, got {falling_mid}"
    );
    assert!(
        reset_low.abs() < 0.15,
        "hyst should reset to the lower branch below in_low-hyst, got {reset_low}"
    );
}

#[test]
fn hyst_accepts_unbounded_negative_input_domain_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("hyst").expect("hyst is registered");

    XspiceInstance::new(
        "a_hyst_negative_input_domain",
        model,
        vec![PortConnection::Analog(1), PortConnection::Analog(2)],
        &[("input_domain".to_string(), -0.1)],
        &[],
        &[],
        &[],
    )
    .expect("ngspice hyst ifspec leaves input_domain unbounded");

    let out = op_voltage(
        "\
* XSPICE hyst negative input_domain oracle
vin in 0 0.5
ahyst in out hmod
.model hmod hyst (in_low=0 in_high=1 hyst=0.2 out_lower_limit=0 out_upper_limit=10 input_domain=-0.1 fraction=false)
rload out 0 1meg
.end
",
        "out",
    );

    assert!(
        (out - 3.0).abs() < 1e-9,
        "ngspice applies negative input_domain directly and returns 3 V at input 0.5, got {out}"
    );
}

#[test]
fn hyst_clamps_negative_hysteresis_width_to_official_minimum() {
    let deck = "\
* XSPICE hyst negative hysteresis clamp oracle
vin in 0 dc 0.5
ahyst in out hmod
.model hmod hyst (in_low=0 in_high=1 hyst=-0.2 out_lower_limit=0 out_upper_limit=10 input_domain=0.01 fraction=false)
rload out 0 1
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 5.0).abs() < 1.0e-9,
        "hyst should clamp negative hysteresis width to 0 like ngspice, got {out}"
    );
}

#[test]
fn hyst_accepts_reversed_input_thresholds_like_ngspice_source() {
    let deck = "\
* XSPICE hyst reversed threshold oracle
vin in 0 dc 0.5
ahyst in out hmod
.model hmod hyst (in_low=1 in_high=0 hyst=0 out_lower_limit=0 out_upper_limit=10 input_domain=0 fraction=false)
rload out 0 1
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 10.0).abs() < 1.0e-9,
        "ngspice hyst source permits reversed thresholds and follows the falling branch, got {out}"
    );
}

#[test]
fn slew_limits_rising_and_falling_transient_slopes_like_ngspice() {
    let deck = "\
* XSPICE slew rise/fall oracle
vin in 0 pwl(0 0 1p 2 2.5n 2 2.501n 0 4n 0)
aslew in out sl
.model sl slew (rise_slope=1e9 fall_slope=2e9)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 4.0e-9, 25.0e-12)
        .expect("transient solves");

    let out = transient_node_series(&result, "out");
    let rise_half_ns = value_at_time(&result.time, out, 0.5e-9);
    let rise_one_and_half_ns = value_at_time(&result.time, out, 1.5e-9);
    let settled_high = value_at_time(&result.time, out, 2.25e-9);
    let falling_quarter_ns = value_at_time(&result.time, out, 2.75e-9);
    let settled_low = value_at_time(&result.time, out, 3.6e-9);

    assert!(
        (rise_half_ns - 0.5).abs() < 0.08,
        "1 V/ns rise limit should put output near 0.5 V at 0.5 ns, got {rise_half_ns}"
    );
    assert!(
        (rise_one_and_half_ns - 1.5).abs() < 0.08,
        "1 V/ns rise limit should put output near 1.5 V at 1.5 ns, got {rise_one_and_half_ns}"
    );
    assert!(
        (settled_high - 2.0).abs() < 0.08,
        "slew output should catch the 2 V input before the falling edge, got {settled_high}"
    );
    assert!(
        (falling_quarter_ns - 1.5).abs() < 0.1,
        "2 V/ns fall limit should put output near 1.5 V at 0.25 ns into the fall, got {falling_quarter_ns}"
    );
    assert!(
        settled_low.abs() < 0.1,
        "slew output should settle back to 0 V after the falling limit, got {settled_low}"
    );
}

#[test]
fn slew_accepts_unbounded_negative_slope_parameters_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("slew").expect("slew is registered");

    XspiceInstance::new(
        "a_slew_negative_slopes",
        model,
        vec![PortConnection::Analog(1), PortConnection::Analog(2)],
        &[
            ("rise_slope".to_string(), -1.0e9),
            ("fall_slope".to_string(), -2.0e9),
        ],
        &[],
        &[],
        &[],
    )
    .expect("ngspice slew ifspec leaves rise_slope and fall_slope unbounded");

    let deck = "\
* XSPICE slew negative slope oracle
vin in 0 pulse(0 1 1n 1p 1p 1n 3n)
aslew in out sl
.model sl slew (rise_slope=-1e9 fall_slope=-2e9)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 4.0e-9, 100.0e-12)
        .expect("transient solves");

    let out = transient_node_series(&result, "out");
    let at_one_ns = value_at_time(&result.time, out, 1.0e-9);
    let at_four_ns = value_at_time(&result.time, out, 4.0e-9);

    assert!(
        (at_one_ns - 2.0).abs() < 0.1,
        "ngspice negative slew slopes produce about 2 V at 1 ns, got {at_one_ns}"
    );
    assert!(
        (at_four_ns - 8.0).abs() < 0.1,
        "ngspice negative slew slopes continue the ramp to about 8 V at 4 ns, got {at_four_ns}"
    );
}

#[test]
fn delay_outputs_time_shifted_analog_history() {
    let deck = "\
* XSPICE analog delay-line oracle
vin in 0 pwl(0 0 1n 1 2n 2 3n 3 4n 4)
adelay in out null dly
.model dly delay (delay=1n buffer_size=64)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 4.0e-9, 0.25e-9)
        .expect("transient solves");

    let out = transient_node_series(&result, "out");
    let before_delay = value_at_time(&result.time, out, 0.5e-9);
    let delayed_mid = value_at_time(&result.time, out, 1.5e-9);
    let delayed_late = value_at_time(&result.time, out, 3.5e-9);

    assert!(
        before_delay.abs() < 0.08,
        "delay should hold the start value before enough history exists, got {before_delay}"
    );
    assert!(
        (delayed_mid - 0.5).abs() < 0.08,
        "1 ns delay should output the input value from 0.5 ns at 1.5 ns, got {delayed_mid}"
    );
    assert!(
        (delayed_late - 2.5).abs() < 0.08,
        "1 ns delay should output the input value from 2.5 ns at 3.5 ns, got {delayed_late}"
    );
}

#[test]
fn delay_omitted_buffer_size_uses_tran_window_like_ngspice() {
    let deck = "\
* XSPICE delay omitted buffer_size run-context oracle
vin in 0 pwl(0 0 2u 2)
adelay in out null dly
.model dly delay (delay=1.5u)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 2.0e-6, 1.0e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");
    let got = value_at_time(&result.time, out, 2.0e-6);

    assert!(
        (got - 0.5).abs() < 0.05,
        "ngspice sizes omitted delay buffer from TSTOP/TSTEP, so 1.5 us delay at 2 us should read the 0.5 us input; got {got}"
    );
}

#[test]
fn delay_clamps_nonpositive_buffer_size_to_official_minimum() {
    let deck = "\
* XSPICE delay buffer_size lower-bound clamp oracle
vin0 in0 0 dc 1
vinn inn 0 dc 1
vinf inf 0 dc 1
adel0 in0 out0 null dly0
adeln inn outn null dlyn
adelf inf outf null dlyf
.model dly0 delay (delay=0 buffer_size=0)
.model dlyn delay (delay=0 buffer_size=-1)
.model dlyf delay (delay=0 buffer_size=1.9)
rload0 out0 0 1
rloadn outn 0 1
rloadf outf 0 1
.op
.end
";

    let out_zero = op_voltage(deck, "out0");
    let out_negative = op_voltage(deck, "outn");
    let out_fractional = op_voltage(deck, "outf");
    assert!(
        (out_zero - 1.0).abs() < 1.0e-9,
        "delay should clamp buffer_size=0 to 1 like ngspice, got {out_zero}"
    );
    assert!(
        (out_negative - 1.0).abs() < 1.0e-9,
        "delay should clamp negative buffer_size to 1 like ngspice, got {out_negative}"
    );
    assert!(
        (out_fractional - 1.0).abs() < 1.0e-9,
        "delay should round fractional integer buffer_size like ngspice, got {out_fractional}"
    );
}

#[test]
fn delay_clamps_negative_controlled_delay_bounds_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("delay").expect("delay is registered");

    XspiceInstance::new(
        "a_delay_negative_bounds",
        model,
        vec![
            PortConnection::Analog(1),
            PortConnection::Analog(2),
            PortConnection::Analog(3),
        ],
        &[
            ("has_delay_cnt".to_string(), 1.0),
            ("delmin".to_string(), -1.0e-9),
            ("delmax".to_string(), -2.0e-9),
        ],
        &[],
        &[],
        &[],
    )
    .expect("ngspice clamps negative delmin/delmax to 0 instead of rejecting");

    let deck = "\
* XSPICE delay negative controlled-delay bounds oracle
vin in 0 pwl(0 0 1n 1 2n 1)
vctrl ctrl 0 dc 0.75
adel in out ctrl dmod
.model dmod delay (has_delay_cnt=true delmin=-1n delmax=-2n buffer_size=128)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.0e-9, 0.01e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    for (target, expected) in [(0.25e-9, 0.25), (0.50e-9, 0.50), (0.75e-9, 0.75)] {
        let got = value_at_time(&result.time, out, target);
        assert!(
            (got - expected).abs() < 0.02,
            "ngspice clamps negative controlled delay bounds to zero at {target:e}s: got {got}, expected {expected}"
        );
    }
}

#[test]
fn delay_negative_controlled_delmax_resets_positive_delmin_like_ngspice() {
    let deck = "\
* XSPICE delay negative delmax resets delmin oracle
vin in 0 pwl(0 0 1n 1)
vctrl ctrl 0 dc 0.5
adel in out ctrl dmod
.model dmod delay (has_delay_cnt=true delmin=1n delmax=-1n buffer_size=128)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.0e-9, 0.01e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");
    let got = value_at_time(&result.time, out, 0.75e-9);

    assert!(
        (got - 0.75).abs() < 0.03,
        "ngspice cm_delay resets positive delmin when provided delmax is negative; got {got}"
    );
}

#[test]
fn delay_omitted_controlled_delmax_defaults_to_tran_stop_like_ngspice() {
    let deck = "\
* XSPICE delay omitted delmax transient-context oracle
vin in 0 pwl(0 0 1n 1 2n 2 3n 3 4n 4)
vctrl ctrl 0 dc 0.5
adel in out ctrl dmod
.model dmod delay (has_delay_cnt=true delmin=0 buffer_size=128)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 4.0e-9, 0.25e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");
    let got = value_at_time(&result.time, out, 3.0e-9);

    assert!(
        (got - 1.0).abs() < 0.08,
        "ngspice uses TSTOP as omitted controlled-delay delmax, so 50% control at 3 ns should read the 1 ns input; got {got}"
    );
}

#[test]
fn delay_clamps_controlled_delmax_to_tran_stop_like_ngspice() {
    let deck = "\
* XSPICE delay delmax TSTOP clamp oracle
vin in 0 pwl(0 0 1n 1 2n 2 3n 3 4n 4)
vctrl ctrl 0 dc 0.5
adel in out ctrl dmod
.model dmod delay (has_delay_cnt=true delmin=0 delmax=10n buffer_size=128)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 4.0e-9, 0.25e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");
    let got = value_at_time(&result.time, out, 3.0e-9);

    assert!(
        (got - 1.0).abs() < 0.08,
        "ngspice clamps controlled-delay delmax to TSTOP, so 50% control at 3 ns should read the 1 ns input; got {got}"
    );
}

#[test]
fn delay_treats_sub_tstep_delay_as_zero_like_ngspice_source() {
    let model = rspice_core::xspice::models::AnalogDelayLine;
    let mut ctx = CmContext::new();
    ctx.analysis = AnalysisType::Transient;
    ctx.timestep = 0.25e-9;
    ctx.set_param("delay", 0.1e-9);
    ctx.set_param("buffer_size", 16.0);

    model.init(&mut ctx).expect("delay init");
    ctx.set_input_analog("in", 0.0);
    model
        .evaluate(&mut ctx)
        .expect("delay evaluates first sample");

    ctx.time = 0.25e-9;
    ctx.set_input_analog("in", 0.25);
    model
        .evaluate(&mut ctx)
        .expect("delay evaluates sub-TSTEP sample");

    let got = ctx.output("out");
    assert!(
        (got - 0.25).abs() < 1.0e-12,
        "ngspice zeros delay smaller than TSTEP, got {got}"
    );
}

#[test]
fn astate_clamps_state_number_to_official_range_like_ngspice() {
    let deck = "\
* XSPICE astate_no bounds oracle
vinl inl 0 dc 1
vinh inh 0 dc 2
vinf inf 0 dc 3
al inl outl stl
ah inh outh sth
af inf outf stf
.model stl astate (astate_no=-1)
.model sth astate (astate_no=4)
.model stf astate (astate_no=1.9)
rloadl outl 0 1meg
rloadh outh 0 1meg
rloadf outf 0 1meg
.op
.end
";

    let out_low = op_voltage(deck, "outl");
    let out_high = op_voltage(deck, "outh");
    let out_fractional = op_voltage(deck, "outf");
    assert!(
        (out_low - 1.0).abs() < 1.0e-9,
        "ngspice clamps astate_no below 0 to 0, got {out_low}"
    );
    assert!(
        (out_high - 2.0).abs() < 1.0e-9,
        "ngspice clamps astate_no above 3 to 3, got {out_high}"
    );
    assert!(
        (out_fractional - 3.0).abs() < 1.0e-9,
        "ngspice rounds fractional astate_no before use, got {out_fractional}"
    );
}

#[test]
fn astate_returns_selected_previous_transient_state() {
    let deck = "\
* XSPICE analog state return oracle
vin in 0 pwl(0 0 1n 1 2n 2 3n 3 4n 4)
ast in out st
.model st astate (astate_no=2)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 4.0e-9, 0.5e-9)
        .expect("transient solves");

    let input = transient_node_series(&result, "in");
    let out = transient_node_series(&result, "out");
    assert!(
        input.len() >= 5,
        "astate test requires enough accepted samples, got {:?}",
        result.time
    );

    for index in 3..input.len() {
        let expected = input[index - 2];
        assert!(
            (out[index] - expected).abs() < 1.0e-8,
            "astate_no=2 should return the input from two accepted samples back at t={:.3e}: got {}, expected {}; times={:?}",
            result.time[index],
            out[index],
            expected,
            result.time
        );
    }
}

#[test]
fn oneshot_dc_output_is_low_and_ac_gain_is_zero() {
    let op_deck = "\
* XSPICE oneshot DC output
vclk clk 0 dc 1
aone clk null null out os
.model os oneshot (out_low=-2 out_high=3)
rload out 0 1meg
.op
.end
";
    let out = op_voltage(op_deck, "out");
    assert!(
        (out + 2.0).abs() < 1.0e-9,
        "oneshot DC output should be out_low: got {out}"
    );

    let ac_deck = "\
* XSPICE oneshot AC gain
vclk clk 0 dc 1 ac 1
aone clk null null out os
.model os oneshot (out_low=-2 out_high=3)
rload out 0 1meg
.ac lin 1 1k 1k
.end
";
    let out = ac_voltage(ac_deck, "out");
    assert!(
        out.re.abs() < 1.0e-12 && out.im.abs() < 1.0e-12,
        "oneshot AC output should have zero small-signal gain: got {out}"
    );
}

#[test]
fn oneshot_generates_edge_triggered_pulse_shape() {
    let deck = "\
* XSPICE oneshot pulse timing
vclk clk 0 pwl(0 0 0.9n 0 1n 1 5n 1)
aone clk null null out os
.model os oneshot (cntl_array=[0 1] pw_array=[1n 1n] clk_trig=0.5 out_low=0 out_high=5 rise_delay=0.2n rise_time=0.2n fall_delay=0.1n fall_time=0.2n)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 3.0e-9, 0.1e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let before_rise = value_at_time(&result.time, out, 1.1e-9);
    let rising_mid = value_at_time(&result.time, out, 1.25e-9);
    let high = value_at_time(&result.time, out, 1.6e-9);
    let falling_mid = value_at_time(&result.time, out, 2.55e-9);
    let final_low = value_at_time(&result.time, out, 2.75e-9);

    assert!(
        before_rise.abs() < 0.08,
        "oneshot should remain low before rise_delay expires: got {before_rise}"
    );
    assert!(
        (rising_mid - 2.5).abs() < 0.15,
        "oneshot should rise linearly through midpoint: got {rising_mid}"
    );
    assert!(
        (high - 5.0).abs() < 0.08,
        "oneshot should hold high during pulse width: got {high}"
    );
    assert!(
        (falling_mid - 2.5).abs() < 0.15,
        "oneshot should fall linearly through midpoint: got {falling_mid}"
    );
    assert!(
        final_low.abs() < 0.08,
        "oneshot should return low after fall_time: got {final_low}"
    );
}

#[test]
fn oneshot_clear_resets_active_pulse_low() {
    let deck = "\
* XSPICE oneshot clear reset
vclk clk 0 pwl(0 0 0.9n 0 1n 1 5n 1)
vclr clr 0 pwl(0 0 1.6n 0 1.61n 1 5n 1)
aone clk null clr out os
.model os oneshot (cntl_array=[0 1] pw_array=[3n 3n] clk_trig=0.5 out_low=0 out_high=5 rise_delay=0.1n rise_time=0.1n fall_delay=0.1n fall_time=0.1n)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 2.2e-9, 0.25e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let before_clear = value_at_time(&result.time, out, 1.5e-9);
    let after_clear = value_at_time(&result.time, out, 1.8e-9);

    assert!(
        (before_clear - 5.0).abs() < 0.1,
        "oneshot pulse should be high before clear: got {before_clear}"
    );
    assert!(
        after_clear.abs() < 0.1,
        "oneshot clear input should force output low: got {after_clear}"
    );
}

#[test]
fn oneshot_retrigger_extends_active_pulse_when_enabled() {
    let deck = "\
* XSPICE oneshot retrigger behavior
vclk clk 0 pwl(0 0 0.9n 0 1n 1 1.1n 0 1.2n 1 3n 1)
alocked clk null null out_locked os_locked
aretrig clk null null out_retrig os_retrig
.model os_locked oneshot (cntl_array=[0 1] pw_array=[0.5n 0.5n] clk_trig=0.5 out_low=0 out_high=5 rise_delay=0 rise_time=0.05n fall_delay=0 fall_time=0.1n retrig=false)
.model os_retrig oneshot (cntl_array=[0 1] pw_array=[0.5n 0.5n] clk_trig=0.5 out_low=0 out_high=5 rise_delay=0 rise_time=0.05n fall_delay=0 fall_time=0.1n retrig=true)
rlock out_locked 0 1meg
rre out_retrig 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 2.0e-9, 0.1e-9)
        .expect("transient solves");

    let locked = transient_node_series(&result, "out_locked");
    let retrig = transient_node_series(&result, "out_retrig");
    let locked_late = value_at_time(&result.time, locked, 1.62e-9);
    let retrig_late = value_at_time(&result.time, retrig, 1.62e-9);

    assert!(
        locked_late < 2.0,
        "non-retriggerable oneshot should be falling or low after original pulse: got {locked_late}"
    );
    assert!(
        retrig_late > 4.5,
        "retriggerable oneshot should extend the active pulse: got {retrig_late}"
    );
}

#[test]
fn oneshot_retrigger_with_reversed_levels_restarts_ramp_like_ngspice() {
    let deck = "\
* XSPICE oneshot reversed-level retrigger oracle
vclk clk 0 pwl(0 0 0.50n 0 0.501n 1 0.70n 1 0.701n 0 0.90n 0 0.901n 1 1.10n 1 1.101n 0)
aos clk null null out os
.model os oneshot (cntl_array=[0 1] pw_array=[0.5n 0.5n] clk_trig=0.5 out_low=5 out_high=0 rise_delay=0 rise_time=0.05n fall_delay=0 fall_time=0.1n retrig=true)
rout out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.3e-9, 0.001e-9)
        .expect("transient solves");

    let out = transient_node_series(&result, "out");
    let out920 = value_at_time(&result.time, out, 0.92e-9);
    let out930 = value_at_time(&result.time, out, 0.93e-9);
    let out950 = value_at_time(&result.time, out, 0.95e-9);

    assert!(
        out920 > 2.5 && out920 < 3.5,
        "ngspice restarts the reversed low-to-high ramp on retrigger, got {out920}"
    );
    assert!(
        out930 > 1.5 && out930 < 2.5,
        "ngspice continues the restarted reversed ramp at 0.93ns, got {out930}"
    );
    assert!(
        out950.abs() < 0.3,
        "ngspice reaches the reversed high level again by 0.95ns, got {out950}"
    );
}

#[test]
fn oneshot_rejects_invalid_control_tables() {
    let deck = "\
* XSPICE oneshot invalid vectors
vclk clk 0 dc 0
aone clk null null out os
.model os oneshot (cntl_array=[0] pw_array=[1n])
rload out 0 1meg
.op
.end
";
    let err = op_error(deck);
    assert!(
        err.to_ascii_lowercase().contains("cntl_array"),
        "unexpected oneshot validation error: {err}"
    );
}

#[test]
fn oneshot_rejects_control_tables_below_official_minimum_length() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("oneshot").expect("oneshot is registered");

    let err = XspiceInstance::new(
        "a_oneshot_short_table",
        model,
        vec![
            PortConnection::Analog(1),
            PortConnection::Null,
            PortConnection::Null,
            PortConnection::Analog(2),
        ],
        &[],
        &[],
        &[
            ("cntl_array".to_string(), vec![0.0]),
            ("pw_array".to_string(), vec![1.0e-9]),
        ],
        &[],
    )
    .expect_err("official oneshot table vector lower bound must be enforced");

    assert!(
        err.to_string().contains("at least 2"),
        "oneshot one-point control table should be rejected like ngspice, got {err}"
    );
}

#[test]
fn oneshot_accepts_descending_control_table_like_ngspice() {
    let deck = "\
* XSPICE oneshot descending control table oracle
vclk clk 0 pwl(0 0 0.1n 0 0.11n 1 5n 1)
aos clk null null out os
.model os oneshot (cntl_array=[1 0] pw_array=[1n 2n] clk_trig=0.5 out_low=0 out_high=5 rise_delay=0 rise_time=0.05n fall_delay=0 fall_time=0.05n)
rload out 0 1k
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 3.0e-9, 0.02e-9)
        .expect("descending oneshot control table solves like ngspice");
    let out = transient_node_series(&result, "out");

    let high = value_at_time(&result.time, out, 1.5e-9);
    let low = value_at_time(&result.time, out, 2.3e-9);

    assert!(
        high > 4.5,
        "ngspice treats the descending table as a 2 ns pulse at null control, got {high}"
    );
    assert!(
        low < 0.05,
        "ngspice pulse has fallen back low after the 2 ns pulse, got {low}"
    );
}

#[test]
fn xspice_pwl_linearizes_table_slope_in_ac() {
    let deck = "\
* XSPICE pwl AC linearization
vin in 0 dc 0.5 ac 1
apwl in out lut
.model lut pwl (x_array=[0 1 2] y_array=[0 10 30] input_domain=0.01 fraction=false limit=false)
rload out 0 1k
.ac lin 1 1k 1k
.end
";

    let out = ac_voltage(deck, "out");

    assert!(
        (out.re - 10.0).abs() < 1e-9 && out.im.abs() < 1e-12,
        "pwl AC output should use the operating-point table slope: got {out}"
    );
}

#[test]
fn oneshot_clamps_negative_pulse_width_table_entries_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("oneshot").expect("oneshot is registered");

    XspiceInstance::new(
        "a_oneshot_negative_pulse_width",
        model,
        vec![
            PortConnection::Analog(1),
            PortConnection::Null,
            PortConnection::Null,
            PortConnection::Analog(2),
        ],
        &[],
        &[],
        &[
            ("cntl_array".to_string(), vec![0.0, 1.0]),
            ("pw_array".to_string(), vec![-1.0e-9, -1.0e-9]),
        ],
        &[],
    )
    .expect("ngspice clamps pw_array entries below the official lower limit to 0");

    let deck = "\
* XSPICE oneshot negative pulse width oracle
vclk clk 0 pulse(0 1 0 0.01n 0.01n 0.2n 0.4n)
aos clk null null out os
.model os oneshot (cntl_array=[0 1] pw_array=[-1n -1n] clk_trig=0.5 out_low=0 out_high=5 rise_delay=0 rise_time=0.05n fall_delay=0 fall_time=0.05n)
rload out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 0.3e-9, 0.002e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let early_fall = value_at_time(&result.time, out, 0.05e-9);
    let late_fall = value_at_time(&result.time, out, 0.10e-9);
    let settled_low = value_at_time(&result.time, out, 0.15e-9);

    assert!(
        (early_fall - 4.344).abs() < 0.25,
        "ngspice clamps negative pw_array to 0 and has begun falling by 0.05 ns, got {early_fall}"
    );
    assert!(
        (late_fall - 0.656).abs() < 0.25,
        "ngspice zero-width pulse is almost low by 0.10 ns, got {late_fall}"
    );
    assert!(
        settled_low.abs() < 0.05,
        "ngspice zero-width pulse is low by 0.15 ns, got {settled_low}"
    );
}

#[test]
fn xspice_waveform_oscillators_match_official_transient_shape() {
    let deck = "\
* XSPICE official waveform oscillators
vctrl ctrl 0 dc 0
asine ctrl sine_out sine_osc
asquare ctrl square_out square_osc
atri ctrl triangle_out triangle_osc
.model sine_osc sine (cntl_array=[0 1] freq_array=[1e9 1e9] out_low=-1 out_high=1)
.model square_osc square (cntl_array=[0 1] freq_array=[1e9 1e9] out_low=-1 out_high=1 duty_cycle=0.5 rise_time=0.05n fall_time=0.05n)
.model triangle_osc triangle (cntl_array=[0 1] freq_array=[1e9 1e9] out_low=-1 out_high=1 duty_cycle=0.5)
rsine sine_out 0 1meg
rsquare square_out 0 1meg
rtri triangle_out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.1e-9, 0.025e-9)
        .expect("transient solves");

    let sine = transient_node_series(&result, "sine_out");
    let square = transient_node_series(&result, "square_out");
    let triangle = transient_node_series(&result, "triangle_out");

    let sine_quarter = value_at_time(&result.time, sine, 0.25e-9);
    let sine_half = value_at_time(&result.time, sine, 0.5e-9);
    let square_low = value_at_time(&result.time, square, 0.25e-9);
    let square_rising = value_at_time(&result.time, square, 0.525e-9);
    let square_high = value_at_time(&result.time, square, 0.75e-9);
    let square_falling = value_at_time(&result.time, square, 1.025e-9);
    let triangle_quarter = value_at_time(&result.time, triangle, 0.25e-9);
    let triangle_half = value_at_time(&result.time, triangle, 0.5e-9);
    let triangle_three_quarter = value_at_time(&result.time, triangle, 0.75e-9);
    let triangle_cycle = value_at_time(&result.time, triangle, 1.0e-9);

    assert!(
        (sine_quarter - 1.0).abs() < 2.0e-2,
        "sine reaches the high peak at quarter-cycle: got {sine_quarter}"
    );
    assert!(
        sine_half.abs() < 2.0e-2,
        "sine returns to center at half-cycle: got {sine_half}"
    );
    assert!(
        (square_low + 1.0).abs() < 2.0e-2,
        "square starts low before the duty-cycle edge: got {square_low}"
    );
    assert!(
        square_rising.abs() < 6.0e-2,
        "square rises linearly during rise_time: got {square_rising}"
    );
    assert!(
        (square_high - 1.0).abs() < 2.0e-2,
        "square holds high during duty window: got {square_high}"
    );
    assert!(
        square_falling.abs() < 6.0e-2,
        "square falls linearly during fall_time: got {square_falling}"
    );
    assert!(
        triangle_quarter.abs() < 3.0e-2,
        "triangle is halfway through its rising ramp at quarter-cycle: got {triangle_quarter}"
    );
    assert!(
        (triangle_half - 1.0).abs() < 2.0e-2,
        "triangle reaches high peak at duty-cycle boundary: got {triangle_half}"
    );
    assert!(
        triangle_three_quarter.abs() < 3.0e-2,
        "triangle is halfway through its falling ramp at three-quarter-cycle: got {triangle_three_quarter}"
    );
    assert!(
        (triangle_cycle + 1.0).abs() < 2.0e-2,
        "triangle reaches low peak at cycle boundary: got {triangle_cycle}"
    );
}

#[test]
fn xspice_waveform_oscillators_request_edge_breakpoints() {
    let deck = "\
* XSPICE official waveform oscillator breakpoints
vctrl ctrl 0 dc 0
asquare ctrl square_out square_osc
atri ctrl triangle_out triangle_osc
.model square_osc square (cntl_array=[0 1] freq_array=[1e9 1e9] out_low=-1 out_high=1 duty_cycle=0.5 rise_time=0.05n fall_time=0.05n)
.model triangle_osc triangle (cntl_array=[0 1] freq_array=[1e9 1e9] out_low=-1 out_high=1 duty_cycle=0.5)
rsquare square_out 0 1meg
rtri triangle_out 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.1e-9, 0.4e-9)
        .expect("transient solves");

    for breakpoint in [0.5e-9, 0.55e-9, 1.0e-9, 1.05e-9] {
        assert!(
            result
                .time
                .iter()
                .any(|time| (*time - breakpoint).abs() < 1.0e-18),
            "expected accepted transient time at XSPICE breakpoint {breakpoint:e}; times={:?}",
            result.time
        );
    }
}

#[test]
fn xspice_multi_input_pwl_selects_controlling_input_like_ngspice() {
    let deck = "\
* XSPICE official multi_input_pwl gates
vlow low 0 dc 0.25
vmid mid 0 dc 0.75
vhi hi 0 dc 1.5
aand low mid hi out_and and_lut
aor low mid hi out_or or_lut
anand low mid hi out_nand nand_lut
anor low mid hi out_nor nor_lut
.model and_lut multi_input_pwl (x=[0 1 2] y=[0 10 20] model=\"and\")
.model or_lut multi_input_pwl (x=[0 1 2] y=[0 10 20] model=\"or\")
.model nand_lut multi_input_pwl (x=[0 1 2] y=[0 10 20] model=\"nand\")
.model nor_lut multi_input_pwl (x=[0 1 2] y=[0 10 20] model=\"nor\")
rand out_and 0 1meg
ror out_or 0 1meg
rnand out_nand 0 1meg
rnor out_nor 0 1meg
.op
.end
";

    let out_and = op_voltage(deck, "out_and");
    let out_or = op_voltage(deck, "out_or");
    let out_nand = op_voltage(deck, "out_nand");
    let out_nor = op_voltage(deck, "out_nor");

    assert!(
        (out_and - 2.5).abs() < 1.0e-9,
        "and mode uses the smallest input through the table: got {out_and}"
    );
    assert!(
        (out_or - 15.0).abs() < 1.0e-9,
        "or mode uses the largest input through the table: got {out_or}"
    );
    assert!(
        (out_nand - 10.0).abs() < 1.0e-9,
        "nand mode uses ngspice reverse-table behavior: got {out_nand}"
    );
    assert!(
        out_nor.abs() < 1.0e-9,
        "nor mode uses ngspice reverse-table behavior: got {out_nor}"
    );
}

#[test]
fn multi_input_pwl_linearizes_controlling_vector_input_in_ac() {
    let deck = "\
* XSPICE multi_input_pwl AC vector-input linearization
vlow low 0 dc 0.25 ac 2
vmid mid 0 dc 0.75 ac 100
vhi hi 0 dc 1.5 ac 100
aand low mid hi out and_lut
.model and_lut multi_input_pwl (x=[0 1 2] y=[0 10 20] model=\"and\")
rload out 0 1meg
.op
.ac lin 1 1k 1k
.end
";

    let ac_out = ac_voltage(deck, "out");

    assert!(
        (ac_out.re - 20.0).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "multi_input_pwl AC should use only the selected controlling input partial, got {ac_out}"
    );
}

#[test]
fn multi_input_pwl_accepts_descending_table_like_ngspice() {
    let deck = "\
* XSPICE multi_input_pwl descending table oracle
vlow low 0 dc 0
vhi hi 0 dc 2
aand low hi out_and and_lut
aor low hi out_or or_lut
anand low hi out_nand nand_lut
anor low hi out_nor nor_lut
.model and_lut multi_input_pwl (x=[1 0] y=[10 20] model=\"and\")
.model or_lut multi_input_pwl (x=[1 0] y=[10 20] model=\"or\")
.model nand_lut multi_input_pwl (x=[1 0] y=[10 20] model=\"nand\")
.model nor_lut multi_input_pwl (x=[1 0] y=[10 20] model=\"nor\")
rand out_and 0 1meg
ror out_or 0 1meg
rnand out_nand 0 1meg
rnor out_nor 0 1meg
.op
.end
";

    let out_and = op_voltage(deck, "out_and");
    let out_or = op_voltage(deck, "out_or");
    let out_nand = op_voltage(deck, "out_nand");
    let out_nor = op_voltage(deck, "out_nor");

    assert!(
        (out_and - 10.0).abs() < 1.0e-9,
        "ngspice accepts descending x tables and applies the low-end guard in and mode: got {out_and}"
    );
    assert!(
        (out_or - 20.0).abs() < 1.0e-9,
        "ngspice accepts descending x tables and applies the high-end guard in or mode: got {out_or}"
    );
    assert!(
        (out_nand - 20.0).abs() < 1.0e-9,
        "ngspice reverse-table nand guard should return the last y value: got {out_nand}"
    );
    assert!(
        (out_nor - 10.0).abs() < 1.0e-9,
        "ngspice reverse-table nor guard should return the first y value: got {out_nor}"
    );
}

#[test]
fn multi_input_pwl_truncates_mismatched_table_vectors_like_ngspice() {
    let deck = "\
* XSPICE multi_input_pwl mismatched x/y vectors
vlow low 0 dc 0.5
vhi hi 0 dc 1.5
aand low hi out and_lut
.model and_lut multi_input_pwl (x=[0 1 2] y=[0 10] model=\"and\")
rload out 0 1meg
.op
.end
";

    let out = op_voltage(deck, "out");

    assert!(
        (out - 5.0).abs() < 1.0e-9,
        "ngspice truncates the longer table vector and evaluates the common prefix; got {out}"
    );
}

#[test]
fn xspice_multi_input_pwl_rejects_table_vectors_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry
        .get("multi_input_pwl")
        .expect("multi_input_pwl is registered");

    let err = XspiceInstance::new(
        "amulti_short_table",
        model,
        vec![
            PortConnection::AnalogVector(vec![1, 2]),
            PortConnection::Differential(3, 0),
        ],
        &[],
        &[],
        &[("x".to_string(), vec![0.0]), ("y".to_string(), vec![1.0])],
        &[],
    )
    .expect_err("official multi_input_pwl table vector lower bound must be enforced");

    assert!(
        err.to_string().contains("at least 2"),
        "multi_input_pwl one-point table arrays should be rejected like ngspice, got {err}"
    );
}

#[test]
fn spice2poly_uses_spice2_order_and_vector_ac_partials() {
    let deck = "\
* XSPICE official spice2poly polynomial
vx x 0 dc 2 ac 1
vy y 0 dc 3 ac 0
apoly x y out poly
.model poly spice2poly (coef=[1 2 3 4 5 6] m=2)
rload out 0 1meg
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 228.0).abs() < 1.0e-8,
        "spice2poly DC value should follow SPICE2 coefficient order: got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 66.0).abs() < 1.0e-8 && ac_out.im.abs() < 1.0e-10,
        "spice2poly AC gain should use vector input partials at the OP: got {ac_out}"
    );
}

#[test]
fn icm_spice2poly_alias_matches_spice2poly_behavior() {
    let deck = "\
* XSPICE official icm_spice2poly alias
vin in 0 dc 2
apoly in out poly
.model poly icm_spice2poly (coef=[1 3])
rload out 0 1meg
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 7.0).abs() < 1.0e-9,
        "icm_spice2poly alias should evaluate the canonical spice2poly model, got {out}"
    );
}

#[test]
fn spice2poly_id_input_reads_inserted_probe_branch_current() {
    let deck = "\
* XSPICE spice2poly differential current input
iin 0 sense dc 2m
rsense sense 0 1k
apoly %id[sense 0] out poly
.model poly spice2poly (coef=[0 1000])
rload out 0 1meg
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 2.0).abs() < 1.0e-9,
        "spice2poly %id input should read the inserted zero-volt probe branch current, got {out}"
    );
}

#[test]
fn spice2poly_vnam_input_reads_existing_voltage_source_current() {
    let deck = "\
* XSPICE spice2poly voltage-source-name current input
iin 0 sense dc 2m
vmon sense 0 dc 0
apoly %vnam vmon out poly
.model poly spice2poly (coef=[0 1000])
rload out 0 1meg
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 2.0).abs() < 1.0e-9,
        "spice2poly %vnam input should read the named voltage-source branch current, got {out}"
    );
}

#[test]
fn spice2poly_rejects_scalar_i_input_like_ngspice_46() {
    let deck = "\
* XSPICE spice2poly scalar current rejection
iin 0 sense dc 2m
vmon sense 0 dc 0
apoly %i vmon out poly
.model poly spice2poly (coef=[0 1000])
rload out 0 1meg
.op
.end
";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let err = Engine::default()
        .build_circuit(&netlist)
        .expect_err("ngspice 46 spice2poly input allows [vd,id,vnam], not scalar %i")
        .to_string();

    assert!(
        err.contains("does not allow explicit Current"),
        "unexpected error: {err}"
    );
}

#[test]
fn spice2poly_explicit_id_output_stamps_current_source() {
    let deck = "\
* XSPICE spice2poly differential current output
vin in 0 dc 2 ac 1
apoly in %id[out 0] poly
.model poly spice2poly (coef=[0 0.001])
rload out 0 1k
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out + 2.0).abs() < 1.0e-9,
        "spice2poly %id output should stamp an output current from out to ground, got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re + 1.0).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-10,
        "spice2poly %id output should use current-output small-signal gain, got {ac_out}"
    );
}

#[test]
fn spice2poly_rejects_missing_or_short_coef_vector() {
    let missing = "\
* XSPICE spice2poly missing coef
vin in 0 dc 1
apoly in out poly
.model poly spice2poly (m=1)
rload out 0 1k
.op
.end
";
    let message = op_error(missing);
    assert!(
        message.contains("coef"),
        "missing coef error should name the required parameter, got {message}"
    );

    let short = "\
* XSPICE spice2poly short coef
vin in 0 dc 1
apoly in out poly
.model poly spice2poly (coef=[1])
rload out 0 1k
.op
.end
";
    let message = op_error(short);
    assert!(
        message.contains("coef") && message.contains("at least 2"),
        "short coef error should name the minimum length, got {message}"
    );
}

#[test]
fn spice2poly_rejects_coef_vector_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry
        .get("spice2poly")
        .expect("spice2poly is registered");

    let err = XspiceInstance::new(
        "apoly_short_coef",
        model,
        vec![
            PortConnection::AnalogVector(vec![1]),
            PortConnection::Differential(2, 0),
        ],
        &[],
        &[],
        &[("coef".to_string(), vec![1.0])],
        &[],
    )
    .expect_err("official spice2poly coef vector lower bound must be enforced");

    assert!(
        err.to_string().contains("coef") && err.to_string().contains("at least 2"),
        "spice2poly one-value coef vector should be rejected like ngspice, got {err}"
    );
}

#[test]
fn xspice_inout_deferred_stamps_participate_in_ac() {
    let deck = "\
* XSPICE official aswitch conductance AC path
vin in 0 dc 1 ac 1
vctrl ctrl 0 dc 1
asw ctrl %gd[in out] sw
.model sw aswitch (cntl_on=1 cntl_off=0 r_on=1000 r_off=1e12 log=false)
rload out 0 1000
.ac lin 1 1k 1k
.end
";

    let out = ac_voltage(deck, "out");
    assert!(
        (out.re - 0.5).abs() < 1.0e-9 && out.im.abs() < 1.0e-12,
        "AC must include XSPICE inout deferred conductance stamps: got {out}"
    );
}

#[test]
fn aswitch_control_ac_uses_operating_point_partial() {
    let deck = "\
* XSPICE official aswitch control AC path
vin in 0 dc 1 ac 0
vctrl ctrl 0 dc 0.5 ac 1
asw ctrl %gd[in out] sw
.model sw aswitch (cntl_on=1 cntl_off=0 r_on=1000 r_off=9000 log=false)
rload out 0 5000
.ac lin 1 1k 1k
.end
";

    let out = ac_voltage(deck, "out");
    assert!(
        (out.re - 0.4).abs() < 1.0e-9 && out.im.abs() < 1.0e-12,
        "aswitch control AC partial must use the DC switch voltage: got {out}"
    );
}

#[test]
fn aswitch_limit_parameter_clamps_resistance_like_ngspice() {
    let deck = "\
* XSPICE aswitch official limit parameter
vin in 0 dc 1
vctrl ctrl 0 dc 2
asw ctrl %gd[in out] sw
.model sw aswitch (cntl_on=1 cntl_off=0 r_on=1000 r_off=100000 log=true limit=true)
rload out 0 1000
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 0.5).abs() < 1.0e-9,
        "aswitch limit=true should clamp to r_on=1k like ngspice, got {out}"
    );
}

#[test]
fn aswitch_clamps_nonpositive_r_on_like_ngspice() {
    let deck = "\
* XSPICE aswitch r_on lower guard
vin in 0 dc 1
vctrl ctrl 0 dc 1
aswzero ctrl %gd[in outz] swzero
aswneg ctrl %gd[in outn] swneg
.model swzero aswitch (cntl_on=1 cntl_off=0 r_on=0 r_off=100000 log=false limit=false)
.model swneg aswitch (cntl_on=1 cntl_off=0 r_on=-1 r_off=100000 log=false limit=false)
rloadz outz 0 1000
rloadn outn 0 1000
.op
.end
";

    let expected = 1000.0 / (1000.0 + 1.0e-3);
    let zero = op_voltage(deck, "outz");
    let negative = op_voltage(deck, "outn");
    assert!(
        (zero - expected).abs() < 1.0e-9,
        "aswitch r_on=0 should clamp to 1e-3 like ngspice, got {zero}"
    );
    assert!(
        (negative - expected).abs() < 1.0e-9,
        "aswitch r_on<0 should clamp to 1e-3 like ngspice, got {negative}"
    );
}

#[test]
fn aswitch_linear_clamps_nonpositive_r_off_like_ngspice() {
    let deck = "\
* XSPICE aswitch r_off lower guard
vin in 0 dc 1
vctrl ctrl 0 dc 0
aswzero ctrl %gd[in outz] swzero
aswneg ctrl %gd[in outn] swneg
.model swzero aswitch (cntl_on=1 cntl_off=0 r_on=1000 r_off=0 log=false limit=false)
.model swneg aswitch (cntl_on=1 cntl_off=0 r_on=1000 r_off=-1 log=false limit=false)
rloadz outz 0 1000
rloadn outn 0 1000
.op
.end
";

    let expected = 1000.0 / (1000.0 + 1.0e-9);
    let zero = op_voltage(deck, "outz");
    let negative = op_voltage(deck, "outn");
    assert!(
        (zero - expected).abs() < 1.0e-9,
        "linear aswitch r_off=0 should clamp computed resistance to 1e-9 like ngspice, got {zero}"
    );
    assert!(
        (negative - expected).abs() < 1.0e-9,
        "linear aswitch r_off<0 should clamp computed resistance to 1e-9 like ngspice, got {negative}"
    );
}

#[test]
fn aswitch_tiny_control_delta_returns_without_stamp_like_ngspice() {
    let deck = "\
* XSPICE aswitch tiny control delta no-op
vin in 0 dc 1
vctrl ctrl 0 dc 0
asw ctrl %gd[in out] sw
.model sw aswitch (cntl_on=0 cntl_off=0 r_on=1000 r_off=100000 log=false limit=false)
rload out 0 1000
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        out.abs() < 1.0e-12,
        "aswitch with control delta below 1e-12 should return without stamping like ngspice, got {out}"
    );
}

#[test]
fn potentiometer_divides_dc_and_ac_with_linear_position() {
    let deck = "\
* XSPICE potentiometer linear divider
vtop top 0 dc 10 ac 1
apot 0 w top pot
.model pot potentiometer (position=0.25 r=10000 log=false)
.op
.ac lin 1 1k 1k
.end
";

    let w_dc = op_voltage(deck, "w");
    assert!(
        (w_dc - 2.5).abs() < 1.0e-9,
        "linear potentiometer DC wiper should follow position: got {w_dc}"
    );

    let w_ac = ac_voltage(deck, "w");
    assert!(
        (w_ac.re - 0.25).abs() < 1.0e-9 && w_ac.im.abs() < 1.0e-12,
        "linear potentiometer AC wiper should use the same conductance split: got {w_ac}"
    );
}

#[test]
fn potentiometer_uses_official_logarithmic_position_split() {
    let deck = "\
* XSPICE potentiometer log divider
vtop top 0 dc 10
apot 0 w top pot
.model pot potentiometer (position=0.5 r=100000 log=true log_multiplier=2)
.op
.end
";

    let w = op_voltage(deck, "w");
    assert!(
        (w - 1.0).abs() < 1.0e-9,
        "log potentiometer should set r_lower = r / 10^(position*log_multiplier): got {w}"
    );
}

#[test]
fn potentiometer_accepts_unbounded_negative_total_resistance_like_ngspice() {
    let deck = "\
* XSPICE potentiometer negative resistance
vtop top 0 dc 10
apot 0 w top pot
.model pot potentiometer (position=0.25 r=-10000 log=false)
.op
.end
";

    let w = op_voltage(deck, "w");
    assert!(
        (w - 2.5).abs() < 1.0e-9,
        "negative potentiometer resistance should divide like ngspice, got {w}"
    );
}

#[test]
fn potentiometer_clamps_out_of_range_position_like_ngspice() {
    let deck = "\
* XSPICE potentiometer out-of-range position clamp
vtop top 0 dc 10
aplow 0 wlow top potlow
aphigh 0 whigh top pothigh
.model potlow potentiometer (position=-0.1 r=10000 log=false)
.model pothigh potentiometer (position=1.1 r=10000 log=false)
.op
.end
";

    let low = op_voltage(deck, "wlow");
    let high = op_voltage(deck, "whigh");
    assert!(
        (low - 1.0e-8).abs() < 1.0e-11,
        "position below zero should clamp to ngspice lower guard, got {low}"
    );
    assert!(
        (high - 10.0).abs() < 1.0e-7,
        "position above one should clamp to ngspice upper guard, got {high}"
    );
}

#[test]
fn potentiometer_rejects_zero_total_resistance() {
    let deck = "\
* XSPICE potentiometer zero resistance
vtop top 0 dc 10
apot 0 w top pot
.model pot potentiometer (position=0.5 r=0)
.op
.end
";

    let message = op_error(deck);
    assert!(
        message.contains("resistance split must be nonzero"),
        "unexpected potentiometer error for r=0: {message}"
    );
}

#[test]
fn pswitch_default_gd_ports_divide_dc_and_ac_like_official_model() {
    let deck = "\
* XSPICE pswitch linear divider
vin in 0 dc 1 ac 1
vctrl ctrl 0 dc 0.5
rin in out 5000
apsw ctrl 0 out 0 psw
.model psw pswitch (cntl_off=0 cntl_on=1 log=false r_on=1000 r_off=9000 r_cntl_in=1e12)
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 0.5).abs() < 1.0e-9,
        "pswitch should interpolate to 5k and divide the DC input in half: got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 0.5).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "pswitch AC self conductance should match the operating-point resistance: got {ac_out}"
    );
}

#[test]
fn pswitch_accepts_negative_control_input_resistance_like_ngspice() {
    let deck = "\
* XSPICE pswitch negative control input resistance
vin in 0 dc 1
vctrl ctrl 0 dc 0.5
rin in out 5000
apsw ctrl 0 out 0 psw
.model psw pswitch (cntl_off=0 cntl_on=1 log=false r_on=1000 r_off=9000 r_cntl_in=-1000)
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 0.5).abs() < 1.0e-9,
        "negative pswitch control input resistance should be accepted like ngspice, got {out}"
    );
}

#[test]
fn pswitch_log_rejects_nonpositive_r_off_like_ngspice_failure() {
    let deck = "\
* XSPICE pswitch logarithmic nonpositive off resistance
vin in 0 dc 1
vctrl ctrl 0 dc 0.5
rin in out 5000
apsw ctrl 0 out 0 psw
.model psw pswitch (cntl_off=0 cntl_on=1 log=true r_on=1000 r_off=0 r_cntl_in=1e12)
.op
.end
";

    let message = op_error(deck);
    assert!(
        message.contains("r_off"),
        "log pswitch r_off=0 should fail instead of silently clamping a NaN path: {message}"
    );
}

#[test]
fn pswitch_control_partial_participates_in_ac() {
    let deck = "\
* XSPICE pswitch control partial
vin in 0 dc 1 ac 0
vctrl ctrl 0 dc 0.5 ac 1
rin in out 5000
apsw ctrl 0 out 0 psw
.model psw pswitch (cntl_off=0 cntl_on=1 log=false r_on=1000 r_off=9000 r_cntl_in=1e12)
.op
.ac lin 1 1k 1k
.end
";

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re + 0.4).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "pswitch AC must include dI(out)/dV(cntl_in), got {ac_out}"
    );
}

#[test]
fn sidiode_forward_region_stamps_dc_and_ac_conductance() {
    let deck = "\
* XSPICE sidiode forward conductance
vin in 0 dc 10 ac 1
rser in out 1000
ad out 0 dmod
.model dmod sidiode (ron=1000 roff=1e12 vfwd=0 vrev=1e30 ilimit=1e30 revilimit=1e30 epsilon=0 revepsilon=0 rrev=0)
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 5.0).abs() < 1.0e-9,
        "sidiode forward region should behave like ron to ground in DC: got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 0.5).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "sidiode forward region should stamp the same small-signal conductance: got {ac_out}"
    );
}

#[test]
fn sidiode_clamps_official_lower_bounds_like_ngspice() {
    let deck = "\
* XSPICE sidiode official bounds oracle
vinp inp 0 dc 1
vinn inn 0 dc -1
rserp inp outp 1000
rsern inn outn 1000
adp outp 0 dlow
adn outn 0 dlow
.model dlow sidiode (ron=0 roff=0 vfwd=-1 vrev=-1 ilimit=0 revilimit=0 epsilon=-1 revepsilon=-1 rrev=0)
.op
.end
";

    let outp = op_voltage(deck, "outp");
    assert!(
        (outp - 1.0).abs() < 1.0e-9,
        "sidiode lower-bound parameters should clamp like ngspice on the forward branch, got {outp}"
    );

    let outn = op_voltage(deck, "outn");
    assert!(
        (outn + 1.0).abs() < 1.0e-9,
        "sidiode lower-bound parameters should clamp like ngspice on the reverse branch, got {outn}"
    );
}

#[test]
fn zener_reverse_leakage_conductance_participates_in_dc_and_ac() {
    let deck = "\
* XSPICE zener leakage-dominated conductance
vin in 0 dc 0.2 ac 1
rser in out 1000
az out 0 zmod
.model zmod zener (v_breakdown=1 i_rev=1m i_sat=1e-15 limit_switch=false)
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 0.1).abs() < 1.0e-6,
        "zener leakage conductance should dominate the low-voltage DC divider: got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 0.5).abs() < 1.0e-6 && ac_out.im.abs() < 1.0e-12,
        "zener AC conductance should include the leakage derivative: got {ac_out}"
    );
}

#[test]
fn zener_requires_breakdown_voltage_like_ngspice() {
    let deck = "\
* XSPICE zener requires explicit breakdown voltage
vin in 0 dc 0.2
rser in out 1000
az out 0 zmod
.model zmod zener (i_rev=1m i_sat=1e-15 limit_switch=false)
.op
.end
";

    let message = op_error(deck);
    assert!(
        message.contains("v_breakdown"),
        "ngspice requires zener v_breakdown; got {message}"
    );
}

#[test]
fn zener_clamps_breakdown_voltage_to_official_limits_like_ngspice() {
    let deck = "\
* XSPICE zener v_breakdown clamp oracle
vin in 0 dc 0.2
rser_low in out_low 1000
rser_high in out_high 1000
azlow out_low 0 zlow
azhigh out_high 0 zhigh
.model zlow zener (v_breakdown=0 i_rev=1m i_sat=1e-15 limit_switch=false)
.model zhigh zener (v_breakdown=1e7 i_rev=1m i_sat=1e-15 limit_switch=false)
.op
.end
";

    let low = op_voltage(deck, "out_low");
    assert!(
        (low - 9.524036e-9).abs() < 1.0e-12,
        "zener v_breakdown=0 should clamp to 1e-6 like ngspice, got {low}"
    );

    let high = op_voltage(deck, "out_high");
    assert!(
        (high - 0.1999998).abs() < 1.0e-7,
        "zener v_breakdown above 1e6 should clamp like ngspice, got {high}"
    );
}

#[test]
fn zener_clamps_remaining_official_bounds_like_ngspice() {
    let deck = "\
* XSPICE zener remaining official bounds oracle
vin in 0 dc 0.2
rser_low in out_low 1000
rser_high in out_high 1000
azlow out_low 0 zlow
azhigh out_high 0 zhigh
.model zlow zener (v_breakdown=1 i_breakdown=0 r_breakdown=0 i_rev=0 i_sat=0 n_forward=0 limit_switch=false)
.model zhigh zener (v_breakdown=1 i_breakdown=2e-2 r_breakdown=1 i_rev=1u i_sat=1p n_forward=20 limit_switch=false)
.op
.end
";

    let low = op_voltage(deck, "out_low");
    assert!(
        (low - 6.660315e-2).abs() < 1.0e-8,
        "zener lower official bounds should clamp like ngspice, got {low}"
    );

    let high = op_voltage(deck, "out_high");
    assert!(
        (high - 1.998002e-1).abs() < 1.0e-8,
        "zener n_forward above 10 should clamp like ngspice, got {high}"
    );
}

#[test]
fn memristor_dc_and_ac_use_initial_resistance() {
    let deck = "\
* XSPICE memristor initial conductance
vin in 0 dc 10 ac 1
rser in out 1000
amem out 0 mem
.model mem memristor (rinit=1000 rmin=10 rmax=10000 alpha=0 beta=1 vt=0)
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 5.0).abs() < 1.0e-9,
        "memristor DC should use rinit as its operating-point resistance, got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 0.5).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "memristor AC should stamp the rinit small-signal conductance, got {ac_out}"
    );
}

#[test]
fn ilimit_linear_region_drives_output_through_source_resistance() {
    let deck = "\
* XSPICE ilimit linear current source
vin in 0 dc 1 ac 1
alim in null null out lim
.model lim ilimit (r_out_source=1000 r_out_sink=1000 i_limit_source=1 i_limit_sink=1)
rload out 0 1000
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 0.5).abs() < 1.0e-9,
        "ilimit should drive the load through r_out in the linear region: got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 0.5).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "ilimit AC gain should include r_out loading in the linear region: got {ac_out}"
    );
}

#[test]
fn ilimit_clamps_official_parameter_bounds_like_ngspice() {
    let deck = "\
* XSPICE ilimit official bounds oracle
vin_low in_low 0 dc 1
vin_high in_high 0 dc 1
alow in_low null null out_low lowlim
ahigh in_high null null out_high highlim
.model lowlim ilimit (r_out_source=0 r_out_sink=0 i_limit_source=0 i_limit_sink=0 v_pwr_range=0 i_source_range=0 i_sink_range=0 r_out_domain=0)
.model highlim ilimit (r_out_source=1e12 r_out_sink=1e12 i_limit_source=1 i_limit_sink=1 v_pwr_range=1e-6 i_source_range=1e-9 i_sink_range=1e-9 r_out_domain=1e-9)
rload_low out_low 0 1000
rload_high out_high 0 1000
.op
.end
";

    let low = op_voltage(deck, "out_low");
    assert!(
        (low - 1.0e-9).abs() < 1.0e-13,
        "ilimit lower-bound parameters should clamp like ngspice, got {low}"
    );

    let high = op_voltage(deck, "out_high");
    assert!(
        (high - 9.999990e-7).abs() < 1.0e-12,
        "ilimit output resistance above 1e9 should clamp like ngspice, got {high}"
    );
}

#[test]
fn ilimit_source_limit_clamps_output_current() {
    let deck = "\
* XSPICE ilimit source clamp
vin in 0 dc 100
alim in null null out lim
.model lim ilimit (r_out_source=1 r_out_sink=1 i_limit_source=1m i_limit_sink=1 i_source_range=1u)
rload out 0 1000
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 1.0).abs() < 1.0e-9,
        "ilimit source clamp should inject i_limit_source into the load: got {out}"
    );
}

#[test]
fn ilimit_sink_limit_clamps_output_current() {
    let deck = "\
* XSPICE ilimit sink clamp
vin in 0 dc -100
alim in null null out lim
.model lim ilimit (r_out_source=1 r_out_sink=1 i_limit_source=1 i_limit_sink=2m i_sink_range=1u)
rload out 0 1000
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out + 2.0).abs() < 1.0e-9,
        "ilimit sink clamp should draw i_limit_sink from the load: got {out}"
    );
}

#[test]
fn ilimit_id_input_reads_inserted_probe_branch_current() {
    let deck = "\
* XSPICE ilimit differential-current input
iin 0 sense dc 2m ac 1m
rsense sense 0 1k
alim %id[sense 0] null null out lim
.model lim ilimit (gain=1000 r_out_source=1000 r_out_sink=1000 i_limit_source=1 i_limit_sink=1)
rload out 0 1000
.op
.ac lin 1 1k 1k
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out - 1.0).abs() < 1.0e-9,
        "ilimit %id input should read the inserted current probe branch: got {out}"
    );

    let ac_out = ac_voltage(deck, "out");
    assert!(
        (ac_out.re - 0.5).abs() < 1.0e-9 && ac_out.im.abs() < 1.0e-12,
        "ilimit AC should include the %id branch-current control partial: got {ac_out}"
    );
}

#[test]
fn xspice_pwl_interpolates_smooths_extrapolates_and_limits_like_ngspice() {
    let deck = "\
* XSPICE pwl lookup oracle
vbelow below 0 dc -1
vfirst first 0 dc 0
vmid mid 0 dc 0.5
vbreak break 0 dc 1
vabove above 0 dc 3
vlimbelow limbelow 0 dc -1
vlimedgebelow limedgebelow 0 dc -0.005
vlimedgeabove limedgeabove 0 dc 2.005
vlimabove limabove 0 dc 3
abelow below out_below lut
afirst first out_first lut
amid mid out_mid lut
abreak break out_break lut
aabove above out_above lut
alimbelow limbelow out_limbelow lutlim
alimedgebelow limedgebelow out_limedgebelow lutlim
alimedgeabove limedgeabove out_limedgeabove lutlim
alimabove limabove out_limabove lutlim
.model lut pwl (x_array=[0 1 2] y_array=[0 10 30] input_domain=0.01 fraction=false limit=false)
.model lutlim pwl (x_array=[0 1 2] y_array=[0 10 30] input_domain=0.01 fraction=false limit=true)
r1 out_below 0 1meg
r2 out_first 0 1meg
r3 out_mid 0 1meg
r4 out_break 0 1meg
r5 out_above 0 1meg
r6 out_limbelow 0 1meg
r7 out_limedgebelow 0 1meg
r8 out_limedgeabove 0 1meg
r9 out_limabove 0 1meg
.op
.end
";

    let below = op_voltage(deck, "out_below");
    let first = op_voltage(deck, "out_first");
    let mid = op_voltage(deck, "out_mid");
    let break_point = op_voltage(deck, "out_break");
    let above = op_voltage(deck, "out_above");
    let lim_below = op_voltage(deck, "out_limbelow");
    let lim_edge_below = op_voltage(deck, "out_limedgebelow");
    let lim_edge_above = op_voltage(deck, "out_limedgeabove");
    let lim_above = op_voltage(deck, "out_limabove");

    assert!(
        (below + 10.0).abs() < 1e-9,
        "pwl lower extrapolation should match ngspice: got {below}"
    );
    assert!(
        first.abs() < 1e-9,
        "pwl first endpoint should match ngspice: got {first}"
    );
    assert!(
        (mid - 5.0).abs() < 1e-9,
        "pwl interpolation should match ngspice: got {mid}"
    );
    assert!(
        (break_point - 10.025).abs() < 1e-9,
        "pwl smoothed breakpoint should match ngspice: got {break_point}"
    );
    assert!(
        (above - 50.0).abs() < 1e-9,
        "pwl upper extrapolation should match ngspice: got {above}"
    );
    assert!(
        lim_below.abs() < 1e-9,
        "pwl limit=true lower clamp should match ngspice: got {lim_below}"
    );
    assert!(
        (lim_edge_below - 0.00625).abs() < 1e-9,
        "pwl limit=true lower edge smoothing should match ngspice: got {lim_edge_below}"
    );
    assert!(
        (lim_edge_above - 29.9875).abs() < 1e-9,
        "pwl limit=true upper edge smoothing should match ngspice: got {lim_edge_above}"
    );
    assert!(
        (lim_above - 30.0).abs() < 1e-9,
        "pwl limit=true upper clamp should match ngspice: got {lim_above}"
    );
}

#[test]
fn xspice_pwlts_uses_simulation_time_like_ngspice() {
    let deck = "\
* XSPICE pwlts time lookup oracle
apw out lut
alim out_lim lutlim
.model lut pwlts (x_array=[0 1n 2n] y_array=[0 10 30] input_domain=1e-12 fraction=false limit=false)
.model lutlim pwlts (x_array=[0 1n 2n] y_array=[0 10 30] input_domain=1e-12 fraction=false limit=true)
r1 out 0 1meg
r2 out_lim 0 1meg
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 3.0e-9, 0.5e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");
    let out_lim = transient_node_series(&result, "out_lim");

    assert!(
        (value_at_time(&result.time, out, 0.5e-9) - 5.0).abs() < 1e-9,
        "pwlts interpolation at 0.5 ns should match ngspice"
    );
    assert!(
        (value_at_time(&result.time, out, 1.5e-9) - 20.0).abs() < 1e-9,
        "pwlts interpolation at 1.5 ns should match ngspice"
    );
    assert!(
        (value_at_time(&result.time, out, 3.0e-9) - 50.0).abs() < 1e-9,
        "pwlts upper extrapolation should match ngspice"
    );
    assert!(
        (value_at_time(&result.time, out_lim, 3.0e-9) - 30.0).abs() < 1e-9,
        "pwlts limit=true upper clamp should match ngspice"
    );
}

#[test]
fn xspice_pwl_rejects_malformed_lookup_table() {
    let deck = "\
* XSPICE pwl malformed table
vin in 0 dc 0.5
apwl in out lut
.model lut pwl (x_array=[0 1 2] y_array=[0 1] input_domain=0.01 fraction=false)
r1 out 0 1k
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let err = Engine::default()
        .run_dc_op(&netlist)
        .expect_err("mismatched lookup table must fail");
    let message = err.to_string();

    assert!(
        message.contains("x_array") && message.contains("y_array"),
        "malformed lookup table error should identify both arrays, got {message}"
    );
}

#[test]
fn xspice_pwl_accepts_ngspice_input_domain_limit_clamping() {
    // ngspice accepts these values with parameter-limit warnings and clamps
    // them to the official input_domain bounds before evaluating the table.
    let deck = "\
* XSPICE pwl input_domain limit oracle
vbreak break 0 dc 10
azero break out_zero zero
awide break out_wide wide
.model zero pwl (x_array=[0 10 20] y_array=[0 10 30] input_domain=0 fraction=false limit=false)
.model wide pwl (x_array=[0 10 20] y_array=[0 10 30] input_domain=1 fraction=false limit=false)
r1 out_zero 0 1meg
r2 out_wide 0 1meg
.op
.end
";

    let out_zero = op_voltage(deck, "out_zero");
    let out_wide = op_voltage(deck, "out_wide");

    assert!(
        (out_zero - 10.0).abs() < 1e-9,
        "input_domain=0 clamps to ngspice lower limit without visible smoothing: got {out_zero}"
    );
    assert!(
        (out_wide - 10.125).abs() < 1e-9,
        "input_domain=1 clamps to ngspice upper limit 0.5: got {out_wide}"
    );
}

#[test]
fn xspice_pwl_accepts_absolute_input_domain_overlap_warning_like_ngspice() {
    let deck = "\
* XSPICE pwl absolute smoothing-domain overlap warning oracle
vin in 0 dc 0.5
apwl in out lut
.model lut pwl (x_array=[0 0.5 1] y_array=[0 10 30] input_domain=0.4 fraction=false limit=false)
r1 out 0 1meg
.op
.end
";

    let out = op_voltage(deck, "out");

    assert!(
        (out - 12.0).abs() < 1.0e-9,
        "ngspice reports the 50% breakpoint-domain warning but continues smoothing; got {out}"
    );
}

#[test]
fn xspice_lookup_tables_fail_closed_for_invalid_shapes() {
    let cases = [
        (
            "duplicate x",
            "x_array=[0 1 1] y_array=[0 10 20]",
            "strictly increasing",
        ),
        (
            "decreasing x",
            "x_array=[0 2 1] y_array=[0 20 10]",
            "strictly increasing",
        ),
        ("short table", "x_array=[0] y_array=[0]", "at least 2"),
    ];

    for (name, params, needle) in cases {
        let deck = format!(
            "\
* XSPICE pwl invalid table: {name}
vin in 0 dc 0.5
apwl in out lut
.model lut pwl ({params})
r1 out 0 1k
.op
.end
"
        );
        let message = op_error(&deck);
        assert!(
            message.contains(needle),
            "{name}: expected error containing {needle:?}, got {message}"
        );
    }
}

#[test]
fn xspice_lookup_tables_reject_arrays_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();
    let cases = [
        (
            "pwl",
            vec![PortConnection::Analog(1), PortConnection::Analog(2)],
        ),
        ("pwlts", vec![PortConnection::Analog(1)]),
    ];

    for (model_name, ports) in cases {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        let err = XspiceInstance::new(
            format!("a_{model_name}_short_table"),
            model,
            ports,
            &[],
            &[],
            &[
                ("x_array".to_string(), vec![0.0]),
                ("y_array".to_string(), vec![1.0]),
            ],
            &[],
        )
        .expect_err("official lookup table vector lower bound must be enforced");

        assert!(
            err.to_string().contains("at least 2"),
            "{model_name} one-point lookup arrays should be rejected like ngspice, got {err}"
        );
    }
}

#[test]
fn xspice_pwl_rejects_missing_required_lookup_arrays() {
    let deck = "\
* XSPICE pwl missing required table
vin in 0 dc 0.5
apwl in out lut
.model lut pwl (y_array=[0 1])
r1 out 0 1k
.op
.end
";
    let message = op_error(deck);

    assert!(
        message.contains("Missing required parameter: x_array"),
        "missing required lookup array should be explicit, got {message}"
    );
}

#[test]
fn xspice_pwlts_rejects_malformed_lookup_table() {
    let deck = "\
* XSPICE pwlts malformed table
apw out lut
.model lut pwlts (x_array=[0 1n 2n] y_array=[0 1])
r1 out 0 1k
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let err = Engine::default()
        .run_tran(&netlist, 2.0e-9, 1.0e-9)
        .expect_err("mismatched pwlts table must fail");
    let message = err.to_string();

    assert!(
        message.contains("x_array") && message.contains("y_array"),
        "malformed pwlts table error should identify both arrays, got {message}"
    );
}
