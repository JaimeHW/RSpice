//! Native XSPICE digital code models pinned against ngspice code-model semantics.

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_core::xspice::{
    CodeModelRegistry, ParamType, PortConnection, XspiceInstance, clear_registered_data_files,
    register_data_file,
};
use std::fs;
use std::path::{Path, PathBuf};
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

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

fn transient_node_series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[idx]
}

fn max_between(times: &[f64], values: &[f64], t_start: f64, t_end: f64) -> f64 {
    times
        .iter()
        .zip(values)
        .filter_map(|(&time, &value)| (time >= t_start && time <= t_end).then_some(value))
        .fold(f64::NEG_INFINITY, f64::max)
}

fn min_between(times: &[f64], values: &[f64], t_start: f64, t_end: f64) -> f64 {
    times
        .iter()
        .zip(values)
        .filter_map(|(&time, &value)| (time >= t_start && time <= t_end).then_some(value))
        .fold(f64::INFINITY, f64::min)
}

fn value_near_time(times: &[f64], values: &[f64], target: f64) -> f64 {
    let nearest_delta = times
        .iter()
        .map(|time| (time - target).abs())
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or_else(|| panic!("no samples for target time {target:e}"));
    assert!(
        nearest_delta <= 1.0e-15,
        "expected sample near {target:e}, nearest delta was {nearest_delta:e}"
    );
    times
        .iter()
        .zip(values)
        .filter_map(|(&time, &value)| {
            ((time - target).abs() <= nearest_delta + f64::EPSILON).then_some(value)
        })
        .next_back()
        .expect("nearest sample exists")
}

fn digital_tokens(result: &TransientResult, node: &str) -> Vec<(f64, String)> {
    result
        .digital_trace_named(node)
        .unwrap_or_else(|| panic!("digital trace {node} missing from {:?}", result.node_names))
        .iter()
        .map(|point| (point.time, point.value.to_ngspice_token()))
        .collect()
}

#[test]
fn d_pullup_and_d_pulldown_expose_official_load_parameter() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["d_pullup", "d_pulldown"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));
        let load = model
            .parameters()
            .iter()
            .find(|param| param.name.eq_ignore_ascii_case("load"))
            .unwrap_or_else(|| panic!("{model_name} must expose official load parameter"));

        assert_eq!(
            load.param_type,
            ParamType::Real,
            "{model_name} load parameter must be real"
        );
        assert!(
            (load.default - 1.0e-12).abs() < f64::EPSILON,
            "{model_name} load default must match ngspice official 1e-12 F, got {}",
            load.default
        );
    }
}

#[test]
fn basic_digital_gates_clamp_delays_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();

    for (model_name, connections) in [
        (
            "d_buffer",
            vec![PortConnection::Digital(1), PortConnection::Digital(2)],
        ),
        (
            "d_inverter",
            vec![PortConnection::Digital(1), PortConnection::Digital(2)],
        ),
        (
            "d_and",
            vec![
                PortConnection::DigitalVector(vec![1, 2]),
                PortConnection::Digital(3),
            ],
        ),
        (
            "d_or",
            vec![
                PortConnection::DigitalVector(vec![1, 2]),
                PortConnection::Digital(3),
            ],
        ),
        (
            "d_xor",
            vec![
                PortConnection::DigitalVector(vec![1, 2]),
                PortConnection::Digital(3),
            ],
        ),
        (
            "d_nand",
            vec![
                PortConnection::DigitalVector(vec![1, 2]),
                PortConnection::Digital(3),
            ],
        ),
        (
            "d_nor",
            vec![
                PortConnection::DigitalVector(vec![1, 2]),
                PortConnection::Digital(3),
            ],
        ),
        (
            "d_xnor",
            vec![
                PortConnection::DigitalVector(vec![1, 2]),
                PortConnection::Digital(3),
            ],
        ),
    ] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        for (delay_name, value) in [("rise_delay", 0.0), ("fall_delay", -1.0e-9)] {
            XspiceInstance::new(
                format!("a_{model_name}_{delay_name}_clamp"),
                model.clone(),
                connections.clone(),
                &[(delay_name.to_string(), value)],
                &[],
                &[],
                &[],
            )
            .unwrap_or_else(|err| {
                panic!("{model_name} {delay_name}={value} should clamp like ngspice, got {err}")
            });
        }
    }
}

#[test]
fn d_buffer_clamps_nonpositive_delays_to_official_minimum_in_transient() {
    let result = run_temp_deck(
        "rspice-d-buffer-delay-clamp",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* d_buffer delay lower-bound clamp oracle
a_src [din] src
a_buf [din] [out] buf
.model src d_source (input_file=\"stim.stim\")
.model buf d_buffer (rise_delay=0 fall_delay=-1n inertial_delay=false)
.end
",
        2.2e-9,
        50.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_buffer rise_delay=0 should clamp to 1ps like ngspice, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.001e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_buffer fall_delay below 1ps should clamp to 1ps like ngspice, got {trace:?}"
    );
}

#[test]
fn digital_gate_inputs_honor_ngspice_inverted_port_syntax_end_to_end() {
    let result = run_temp_deck(
        "rspice-digital-inverted-port",
        "0 1s 0s 1s\n",
        "\
* ngspice digital event ports allow per-node inversion with ~
a_src [a b c] src
a_and [a ~b c] [out] gate
.model src d_source (input_file=\"stim.stim\")
.model gate d_and (rise_delay=1p fall_delay=1p)
.end
",
        0.1e-9,
        10.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace.iter().any(|(_, token)| token == "1s"),
        "d_and should see [a ~b c] as [1 1 1] and drive high, got {trace:?}"
    );
}

#[test]
fn digital_gate_bare_scalar_inputs_honor_ngspice_inverted_port_syntax_end_to_end() {
    let result = run_temp_deck(
        "rspice-digital-bare-inverted-port",
        "0 1s 0s 1s\n",
        "\
* ngspice MIF ports allow leading ~ on bare digital connections
a_src [a b c] src
a_and a ~b c [out] gate
.model src d_source (input_file=\"stim.stim\")
.model gate d_and (rise_delay=1p fall_delay=1p)
.end
",
        0.1e-9,
        10.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace.iter().any(|(_, token)| token == "1s"),
        "d_and should group a ~b c as an inverted digital input vector and drive high, got {trace:?}"
    );
}

#[test]
fn basic_digital_vector_gates_reject_input_vector_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["d_and", "d_or", "d_xor", "d_nand", "d_nor", "d_xnor"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        let err = XspiceInstance::new(
            format!("a_{model_name}_short_in"),
            model,
            vec![
                PortConnection::DigitalVector(vec![1]),
                PortConnection::Digital(2),
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("official digital gate input vector lower bound must be enforced");

        assert!(
            err.to_string().contains("in") && err.to_string().contains("at least 2"),
            "{model_name} one-element input vector should be rejected like ngspice, got {err}"
        );
    }
}

#[test]
fn digital_lookup_and_ram_reject_official_vector_port_bounds_at_construction() {
    let registry = CodeModelRegistry::with_builtins();

    let lut = registry.get("d_lut").expect("d_lut is registered");
    let err = XspiceInstance::new(
        "a_d_lut_empty_in",
        lut,
        vec![
            PortConnection::DigitalVector(Vec::new()),
            PortConnection::Digital(1),
        ],
        &[],
        &[("table_values".to_string(), "0".to_string())],
        &[],
        &[],
    )
    .expect_err("official d_lut input vector lower bound must be enforced");
    assert!(
        err.to_string().contains("in") && err.to_string().contains("at least 1"),
        "d_lut empty input vector should be rejected like ngspice, got {err}"
    );

    let ram = registry.get("d_ram").expect("d_ram is registered");
    let err = XspiceInstance::new(
        "a_d_ram_wide_select",
        ram,
        vec![
            PortConnection::DigitalVector(vec![1]),
            PortConnection::DigitalVector(vec![2]),
            PortConnection::DigitalVector(vec![3]),
            PortConnection::Digital(4),
            PortConnection::DigitalVector((5..22).collect()),
        ],
        &[],
        &[],
        &[],
        &[],
    )
    .expect_err("official d_ram select vector upper bound must be enforced");
    assert!(
        err.to_string().contains("select") && err.to_string().contains("at most 16"),
        "d_ram select vector wider than sixteen should be rejected like ngspice, got {err}"
    );
}

#[test]
fn d_ram_accepts_and_clamps_read_delay_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let ram = registry.get("d_ram").expect("d_ram is registered");

    for read_delay in [0.0, -1.0e-9] {
        XspiceInstance::new(
            format!("a_d_ram_read_delay_{read_delay:e}"),
            ram.clone(),
            vec![
                PortConnection::DigitalVector(vec![1]),
                PortConnection::DigitalVector(vec![2]),
                PortConnection::DigitalVector(vec![3]),
                PortConnection::Digital(4),
                PortConnection::DigitalVector(vec![5]),
            ],
            &[("read_delay".to_string(), read_delay)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("d_ram read_delay={read_delay:e} should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn d_ram_accepts_and_clamps_bounded_integer_params_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let ram = registry.get("d_ram").expect("d_ram is registered");

    for (case, param_name, value) in [
        ("select_low", "select_value", -1.0),
        ("select_high", "select_value", 32768.0),
        ("ic_low", "ic", -1.0),
        ("ic_high", "ic", 3.0),
    ] {
        XspiceInstance::new(
            format!("a_d_ram_{case}"),
            ram.clone(),
            vec![
                PortConnection::DigitalVector(vec![1]),
                PortConnection::DigitalVector(vec![2]),
                PortConnection::DigitalVector(vec![3]),
                PortConnection::Digital(4),
                PortConnection::DigitalVector(vec![5]),
            ],
            &[(param_name.to_string(), value)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("d_ram {param_name}={value:e} should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn digital_state_and_bidi_bridge_reject_official_vector_port_bounds_at_construction() {
    let registry = CodeModelRegistry::with_builtins();

    let d_state = registry.get("d_state").expect("d_state is registered");
    let err = XspiceInstance::new(
        "a_d_state_empty_out",
        d_state,
        vec![
            PortConnection::Null,
            PortConnection::Digital(1),
            PortConnection::Null,
            PortConnection::DigitalVector(Vec::new()),
        ],
        &[],
        &[],
        &[],
        &[],
    )
    .expect_err("official d_state output vector lower bound must be enforced");
    assert!(
        err.to_string().contains("out") && err.to_string().contains("at least 1"),
        "d_state empty output vector should be rejected like ngspice, got {err}"
    );

    let bidi = registry
        .get("bidi_bridge")
        .expect("bidi_bridge is registered");
    let err = XspiceInstance::new(
        "a_bidi_empty_a",
        bidi,
        vec![
            PortConnection::AnalogVector(Vec::new()),
            PortConnection::DigitalVector(vec![1]),
            PortConnection::Null,
        ],
        &[],
        &[],
        &[],
        &[],
    )
    .expect_err("official bidi_bridge analog vector lower bound must be enforced");
    assert!(
        err.to_string().contains("a") && err.to_string().contains("at least 1"),
        "bidi_bridge empty analog vector should be rejected like ngspice, got {err}"
    );
}

#[test]
fn bridge_models_accept_and_clamp_timing_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();

    let adc = registry
        .get("adc_bridge")
        .expect("adc_bridge is registered");
    for (delay_name, value) in [("rise_delay", 0.0), ("fall_delay", -1.0e-9)] {
        XspiceInstance::new(
            format!("a_adc_{delay_name}"),
            adc.clone(),
            vec![
                PortConnection::AnalogVector(vec![1]),
                PortConnection::DigitalVector(vec![2]),
            ],
            &[(delay_name.to_string(), value)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("adc_bridge {delay_name}={value:e} should clamp like ngspice, got {err}")
        });
    }

    let dac = registry
        .get("dac_bridge")
        .expect("dac_bridge is registered");
    for (time_name, value) in [("t_rise", 0.0), ("t_fall", -1.0e-9)] {
        XspiceInstance::new(
            format!("a_dac_{time_name}"),
            dac.clone(),
            vec![
                PortConnection::DigitalVector(vec![1]),
                PortConnection::AnalogVector(vec![2]),
            ],
            &[(time_name.to_string(), value)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("dac_bridge {time_name}={value:e} should clamp like ngspice, got {err}")
        });
    }

    let bidi = registry
        .get("bidi_bridge")
        .expect("bidi_bridge is registered");
    for (param_name, value) in [
        ("t_rise", 0.0),
        ("t_fall", -1.0e-9),
        ("rise_delay", 0.0),
        ("fall_delay", -1.0e-9),
    ] {
        XspiceInstance::new(
            format!("a_bidi_{param_name}"),
            bidi.clone(),
            vec![
                PortConnection::AnalogVector(vec![1]),
                PortConnection::DigitalVector(vec![2]),
                PortConnection::Null,
            ],
            &[(param_name.to_string(), value)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("bidi_bridge {param_name}={value:e} should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn bidi_bridge_accepts_and_clamps_resistances_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let bidi = registry
        .get("bidi_bridge")
        .expect("bidi_bridge is registered");

    for (param_name, value) in [
        ("r_stl", 0.0),
        ("r_sth", -1.0),
        ("r_low", 0.0),
        ("r_high", -1.0),
    ] {
        XspiceInstance::new(
            format!("a_bidi_{param_name}"),
            bidi.clone(),
            vec![
                PortConnection::AnalogVector(vec![1]),
                PortConnection::DigitalVector(vec![2]),
                PortConnection::Null,
            ],
            &[(param_name.to_string(), value)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("bidi_bridge {param_name}={value:e} should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn bidi_bridge_accepts_and_clamps_bounded_integer_params_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let bidi = registry
        .get("bidi_bridge")
        .expect("bidi_bridge is registered");

    for (param_name, value) in [
        ("direction", -1.0),
        ("direction", 3.0),
        ("strength", -1.0),
        ("strength", 3.0),
        ("smooth", -1.0),
        ("smooth", 3.0),
    ] {
        XspiceInstance::new(
            format!("a_bidi_{param_name}_{value:e}"),
            bidi.clone(),
            vec![
                PortConnection::AnalogVector(vec![1]),
                PortConnection::DigitalVector(vec![2]),
                PortConnection::Null,
            ],
            &[(param_name.to_string(), value)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("bidi_bridge {param_name}={value:e} should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn adc_and_dac_bridges_use_clamped_timing_minimum_in_transient() {
    let adc_result = run_temp_deck(
        "rspice-adc-bridge-delay-clamp",
        "",
        "\
* adc_bridge delay lower-bound clamp oracle
vin in 0 pulse(0 1 0 1p 1p 1n 2n)
a_adc [in] [dout] adc
.model adc adc_bridge (rise_delay=0 fall_delay=-1n)
.end
",
        1.2e-9,
        100.0e-12,
    );
    let dout = digital_tokens(&adc_result, "dout");
    assert!(
        dout.iter()
            .any(|(time, token)| (*time - 2.0e-12).abs() <= 1.0e-18 && token == "1s"),
        "adc_bridge rise_delay=0 should clamp to 1ps like ngspice, got {dout:?}"
    );
    assert!(
        dout.iter()
            .any(|(time, token)| (*time - 1.003e-9).abs() <= 1.0e-18 && token == "0s"),
        "adc_bridge fall_delay<1ps should clamp to 1ps like ngspice, got {dout:?}"
    );

    let dac_result = run_temp_deck(
        "rspice-dac-bridge-time-clamp",
        "0 0s\n1n 1s\n",
        "\
* dac_bridge transition-time lower-bound clamp oracle
a_src [din] src
a_dac [din] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=1 out_undef=0.5 t_rise=0 t_fall=-1n)
rload out 0 1k
.end
",
        1.2e-9,
        100.0e-12,
    );
    let out = transient_node_series(&dac_result, "out");
    let at_event = value_near_time(&dac_result.time, out, 1.0e-9);
    let early_after_event = max_between(&dac_result.time, out, 1.001e-9, 1.02e-9);
    assert!(
        at_event.abs() <= 1.0e-3,
        "dac_bridge output should still be low at the digital event time, got {at_event}"
    );
    assert!(
        early_after_event > 0.9,
        "dac_bridge t_rise=0 should clamp to 1ps and reach high just after 1ns, got {early_after_event}"
    );
}

#[test]
fn adc_bridge_drives_unknown_inside_threshold_window_like_ngspice() {
    let result = run_temp_deck(
        "rspice-adc-bridge-threshold-unknown",
        "",
        "\
* adc_bridge should drive unknown inside the threshold window
vin in 0 pwl(0 1 1n 1 1.001n 0.5)
a_adc [in] [dout] adc
.model adc adc_bridge (in_low=0.1 in_high=0.9 rise_delay=1p fall_delay=1p)
.end
",
        1.2e-9,
        50.0e-12,
    );

    let dout = digital_tokens(&result, "dout");
    assert!(
        dout.iter()
            .any(|(time, token)| *time > 1.0e-9 && *time < 1.05e-9 && token == "Us"),
        "ngspice adc_bridge emits UNKNOWN inside the in_low/in_high window instead of latching the previous state, got {dout:?}"
    );
}

#[test]
fn adc_bridge_initial_output_has_zero_delay_like_ngspice() {
    let result = run_temp_deck(
        "rspice-adc-bridge-initial-zero-delay",
        "",
        "\
* adc_bridge should drive initial TIME=0 output without rise/fall delay
vin in 0 dc 1
a_adc [in] [dout] adc
.model adc adc_bridge (in_low=0.1 in_high=0.9 rise_delay=100p fall_delay=100p)
.end
",
        0.2e-9,
        50.0e-12,
    );

    let dout = digital_tokens(&result, "dout");
    assert!(
        dout.iter()
            .any(|(time, token)| *time == 0.0 && token == "1s"),
        "ngspice adc_bridge sets TIME=0 outputs immediately without rise/fall delay, got {dout:?}"
    );
}

#[test]
fn digital_oscillators_accept_and_clamp_table_values_outside_official_limits() {
    let registry = CodeModelRegistry::with_builtins();

    let d_osc = registry.get("d_osc").expect("d_osc is registered");
    XspiceInstance::new(
        "a_d_osc_bad_freq",
        d_osc,
        vec![PortConnection::Analog(1), PortConnection::Digital(2)],
        &[],
        &[],
        &[
            ("cntl_array".to_string(), vec![0.0, 1.0]),
            ("freq_array".to_string(), vec![-1.0, 1.0e6]),
        ],
        &[],
    )
    .expect("ngspice accepts and clamps d_osc freq_array values below the official limit");

    let d_pwm = registry.get("d_pwm").expect("d_pwm is registered");
    XspiceInstance::new(
        "a_d_pwm_bad_duty",
        d_pwm,
        vec![PortConnection::Analog(1), PortConnection::Digital(2)],
        &[],
        &[],
        &[
            ("cntl_array".to_string(), vec![-1.0, 1.0]),
            ("dc_array".to_string(), vec![0.0, 1.25]),
        ],
        &[],
    )
    .expect("ngspice accepts and clamps d_pwm dc_array values above the official limit");
}

#[test]
fn digital_oscillators_accept_and_clamp_initial_phase_outside_official_limits() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["d_osc", "d_pwm"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));
        for phase in [-270.0, 720.0] {
            XspiceInstance::new(
                format!("a_{model_name}_{phase}"),
                model.clone(),
                vec![PortConnection::Analog(1), PortConnection::Digital(2)],
                &[("init_phase".to_string(), phase)],
                &[],
                &[],
                &[],
            )
            .unwrap_or_else(|err| {
                panic!("{model_name} must clamp init_phase={phase} like ngspice: {err}")
            });
        }
    }

    let result = run_temp_deck(
        "rspice-d-osc-init-phase-clamp",
        "",
        "\
* d_osc clamps init_phase below the official lower bound before timing
vctrl ctrl 0 0
aosc ctrl [out] osc
.model osc d_osc (cntl_array=[0 1] freq_array=[1g 1g] duty_cycle=0.25 init_phase=-270)
.end
",
        0.8e-9,
        50.0e-12,
    );
    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 0.25e-9).abs() <= 1.0e-18 && token == "1s"),
        "ngspice clamps init_phase=-270 to -180, so d_osc first rises at 0.25ns; got {trace:?}"
    );
    assert!(
        !trace
            .iter()
            .any(|(time, token)| (*time - 0.5e-9).abs() <= 1.0e-18 && token == "1s"),
        "unclamped init_phase=-270 would rise at 0.5ns; got {trace:?}"
    );
}

#[test]
fn d_osc_accepts_and_clamps_duty_cycle_outside_official_limits() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("d_osc").expect("d_osc is registered");

    for duty in [-0.25, 1.2] {
        XspiceInstance::new(
            format!("a_d_osc_{duty}"),
            model.clone(),
            vec![PortConnection::Analog(1), PortConnection::Digital(2)],
            &[("duty_cycle".to_string(), duty)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| panic!("d_osc must clamp duty_cycle={duty} like ngspice: {err}"));
    }

    let result = run_temp_deck(
        "rspice-d-osc-duty-cycle-clamp",
        "",
        "\
* d_osc clamps duty_cycle below the official lower bound before timing
vctrl ctrl 0 0
aosc ctrl [out] osc
.model osc d_osc (cntl_array=[0 1] freq_array=[1g 1g] duty_cycle=-0.25 init_phase=0)
.end
",
        1.2e-9,
        50.0e-12,
    );
    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 0.999_999e-9).abs() <= 1.0e-18 && token == "1s"),
        "ngspice clamps duty_cycle=-0.25 to 1e-6, so d_osc rises at 0.999999ns; got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "0s"),
        "ngspice clamps duty_cycle=-0.25 to 1e-6, so d_osc falls again at 1ns; got {trace:?}"
    );
}

#[test]
fn d_pwm_accepts_and_clamps_frequency_below_official_limit() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("d_pwm").expect("d_pwm is registered");

    for frequency in [-1.0, 0.0] {
        XspiceInstance::new(
            format!("a_d_pwm_{frequency}"),
            model.clone(),
            vec![PortConnection::Analog(1), PortConnection::Digital(2)],
            &[("frequency".to_string(), frequency)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("d_pwm must clamp frequency={frequency} to 1e-6 like ngspice: {err}")
        });
    }

    let result = run_temp_deck(
        "rspice-d-pwm-frequency-clamp",
        "",
        "\
* d_pwm clamps frequency below the official lower bound before timing
vctrl ctrl 0 0
apwm ctrl [out] pwm
.model pwm d_pwm (cntl_array=[-1 1] dc_array=[0.25 0.75] frequency=-1 init_phase=0)
.end
",
        1.0e-9,
        50.0e-12,
    );
    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| *time == 0.0 && token.starts_with('0')),
        "ngspice keeps d_pwm low at t=0 after clamping frequency=-1 to 1e-6; got {trace:?}"
    );
    assert!(
        !trace.iter().any(|(_, token)| token.starts_with('1')),
        "ngspice produces no high d_pwm transition within 1ns after clamping frequency=-1 to 1e-6; got {trace:?}"
    );
}

#[test]
fn digital_oscillators_accept_and_ignore_unused_compat_delays_outside_official_limits() {
    let registry = CodeModelRegistry::with_builtins();

    for model_name in ["d_osc", "d_pwm"] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        for delay_name in ["rise_delay", "fall_delay"] {
            XspiceInstance::new(
                format!("a_{model_name}_{delay_name}"),
                model.clone(),
                vec![PortConnection::Analog(1), PortConnection::Digital(2)],
                &[(delay_name.to_string(), -1.0)],
                &[],
                &[],
                &[],
            )
            .unwrap_or_else(|err| {
                panic!("{model_name} must clamp unused {delay_name}=-1 like ngspice: {err}")
            });
        }
    }

    let osc_result = run_temp_deck(
        "rspice-d-osc-unused-compat-delay-clamp",
        "",
        "\
* d_osc accepts negative unused compatibility delays and ignores them for timing
vctrl ctrl 0 0
aosc ctrl [out] osc
.model osc d_osc (cntl_array=[0 1] freq_array=[1g 1g] duty_cycle=0.25 init_phase=0 rise_delay=-1 fall_delay=-2)
.end
",
        1.1e-9,
        50.0e-12,
    );
    let osc_trace = digital_tokens(&osc_result, "out");
    assert!(
        osc_trace
            .iter()
            .any(|(time, token)| (*time - 0.75e-9).abs() <= 1.0e-18 && token == "1s"),
        "ngspice ignores d_osc rise_delay/fall_delay for timing, so it rises at 0.75ns; got {osc_trace:?}"
    );
    assert!(
        osc_trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "0s"),
        "ngspice ignores d_osc rise_delay/fall_delay for timing, so it falls at 1ns; got {osc_trace:?}"
    );

    let pwm_result = run_temp_deck(
        "rspice-d-pwm-unused-compat-delay-clamp",
        "",
        "\
* d_pwm accepts negative unused compatibility delays and ignores them for timing
vctrl ctrl 0 0
apwm ctrl [out] pwm
.model pwm d_pwm (cntl_array=[-1 1] dc_array=[0.25 0.75] frequency=1g init_phase=0 rise_delay=-1 fall_delay=-2)
.end
",
        1.1e-9,
        50.0e-12,
    );
    let pwm_trace = digital_tokens(&pwm_result, "out");
    assert!(
        pwm_trace
            .iter()
            .any(|(time, token)| (*time - 0.5e-9).abs() <= 1.0e-18 && token == "1s"),
        "ngspice ignores d_pwm rise_delay/fall_delay for timing, so it rises at 0.5ns; got {pwm_trace:?}"
    );
    assert!(
        pwm_trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "0s"),
        "ngspice ignores d_pwm rise_delay/fall_delay for timing, so it falls at 1ns; got {pwm_trace:?}"
    );
}

#[test]
fn digital_oscillators_reject_table_vectors_below_official_minimum_length() {
    let registry = CodeModelRegistry::with_builtins();

    let d_osc = registry.get("d_osc").expect("d_osc is registered");
    let err = XspiceInstance::new(
        "a_d_osc_short_table",
        d_osc,
        vec![PortConnection::Analog(1), PortConnection::Digital(2)],
        &[],
        &[],
        &[
            ("cntl_array".to_string(), vec![0.0]),
            ("freq_array".to_string(), vec![1.0e6]),
        ],
        &[],
    )
    .expect_err("official d_osc table vector lower bound must be enforced");

    assert!(
        err.to_string().contains("at least 2"),
        "d_osc one-point tables should be rejected like ngspice, got {err}"
    );

    let d_pwm = registry.get("d_pwm").expect("d_pwm is registered");
    let err = XspiceInstance::new(
        "a_d_pwm_short_table",
        d_pwm,
        vec![PortConnection::Analog(1), PortConnection::Digital(2)],
        &[],
        &[],
        &[
            ("cntl_array".to_string(), vec![-1.0]),
            ("dc_array".to_string(), vec![0.5]),
        ],
        &[],
    )
    .expect_err("official d_pwm table vector lower bound must be enforced");

    assert!(
        err.to_string().contains("at least 2"),
        "d_pwm one-point tables should be rejected like ngspice, got {err}"
    );
}

#[test]
fn digital_lookup_delays_clamp_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let d_lut = registry.get("d_lut").expect("d_lut is registered");

    for (delay_name, value) in [("rise_delay", 0.0), ("fall_delay", -1.0e-9)] {
        XspiceInstance::new(
            format!("a_lut_{delay_name}_clamp"),
            d_lut.clone(),
            vec![
                PortConnection::DigitalVector(vec![1]),
                PortConnection::Digital(2),
            ],
            &[(delay_name.to_string(), value)],
            &[("table_values".to_string(), "01".to_string())],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("d_lut {delay_name}={value} should clamp like ngspice, got {err}")
        });
    }

    let d_genlut = registry.get("d_genlut").expect("d_genlut is registered");
    for (delay_name, value) in [("rise_delay", 0.0), ("fall_delay", -1.0e-9)] {
        XspiceInstance::new(
            format!("a_genlut_{delay_name}_clamp"),
            d_genlut.clone(),
            vec![
                PortConnection::DigitalVector(vec![1, 2]),
                PortConnection::DigitalVector(vec![3]),
            ],
            &[],
            &[("table_values".to_string(), "0000".to_string())],
            &[(delay_name.to_string(), vec![value])],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("d_genlut {delay_name}=[{value}] should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn digital_lookup_models_use_clamped_delay_minimum_in_transient() {
    let result = run_temp_deck(
        "rspice-digital-lookup-delay-clamp",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* d_lut and d_genlut delay lower-bound clamp oracle
a_src [din] src
a_lut [din] [lutout] lut
a_gen [din] [genout] gen
.model src d_source (input_file=\"stim.stim\")
.model lut d_lut (table_values=\"01\" rise_delay=0 fall_delay=-1n)
.model gen d_genlut (table_values=\"01\" rise_delay=[0] fall_delay=[-1n])
.end
",
        2.2e-9,
        50.0e-12,
    );

    for node in ["lutout", "genout"] {
        let trace = digital_tokens(&result, node);
        assert!(
            trace
                .iter()
                .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "1s"),
            "{node} rise delay should clamp to 1ps like ngspice, got {trace:?}"
        );
        assert!(
            trace
                .iter()
                .any(|(time, token)| (*time - 2.001e-9).abs() <= 1.0e-18 && token == "0s"),
            "{node} fall delay should clamp to 1ps like ngspice, got {trace:?}"
        );
    }
}

fn parse_file(path: &Path) -> Netlist {
    Netlist::parse_file(path).unwrap_or_else(|err| panic!("deck parses: {err}"))
}

fn run_temp_deck(
    prefix: &str,
    stimulus: &str,
    deck: &str,
    tstop: f64,
    max_step: f64,
) -> TransientResult {
    let dir = unique_temp_dir(prefix);
    fs::create_dir_all(&dir).expect("create temp d_source fixture dir");
    fs::write(dir.join("stim.stim"), stimulus).expect("write d_source stimulus");
    let deck_path = dir.join("deck.cir");
    fs::write(&deck_path, deck).expect("write d_source deck");
    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, tstop, max_step)
        .expect("transient solves");
    let _ = fs::remove_dir_all(dir);
    result
}

#[test]
fn d_source_reads_registered_virtual_data_file_and_invalidates_cache_on_replace() {
    let _guard = DataFileRegistryGuard::new();

    let deck = "\
* d_source virtual data file
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"virtual://d_source/stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
rload out 0 1k
.end
";

    register_data_file("virtual://d_source/stim", "0 0s\n1n 1s\n")
        .expect("register first virtual d_source data");
    let netlist = Netlist::parse(deck).expect("deck parses");
    let first = Engine::default()
        .run_tran(&netlist, 1.5e-9, 100.0e-12)
        .expect("first transient solves");
    let first_out = transient_node_series(&first, "out");
    assert!(
        max_between(&first.time, first_out, 1.1e-9, 1.5e-9) > 4.9,
        "first virtual d_source data should drive the DAC high"
    );

    register_data_file("virtual://d_source/stim", "0 0s\n1n 0s\n")
        .expect("replace virtual d_source data");
    let second = Engine::default()
        .run_tran(&netlist, 1.5e-9, 100.0e-12)
        .expect("second transient solves");
    let second_out = transient_node_series(&second, "out");
    assert!(
        max_between(&second.time, second_out, 1.1e-9, 1.5e-9) < 0.1,
        "replaced virtual d_source data should invalidate cached rows"
    );
}

#[test]
fn d_source_native_file_cache_invalidates_same_length_rewrites() {
    let dir = unique_temp_dir("rspice-d-source-native-cache");
    fs::create_dir_all(&dir).expect("create d_source native cache fixture dir");
    let stim_path = dir.join("stim.stim");
    let fixed_modified = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
    let fixed_times = fs::FileTimes::new().set_modified(fixed_modified);

    fs::write(&stim_path, "0 0s\n1n 1s\n").expect("write first native d_source data");
    fs::File::options()
        .write(true)
        .open(&stim_path)
        .expect("open first native d_source data")
        .set_times(fixed_times)
        .expect("pin first native d_source mtime");

    let deck_path = dir.join("deck.cir");
    fs::write(
        &deck_path,
        "\
* d_source native cache invalidation
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
rload out 0 1k
.end
",
    )
    .expect("write d_source native cache deck");
    let netlist = parse_file(&deck_path);

    let first = Engine::default()
        .run_tran(&netlist, 1.5e-9, 100.0e-12)
        .expect("first native-cache transient solves");
    let first_out = transient_node_series(&first, "out");
    assert!(
        max_between(&first.time, first_out, 1.1e-9, 1.5e-9) > 4.9,
        "first native d_source data should drive the DAC high"
    );

    fs::write(&stim_path, "0 0s\n1n 0s\n").expect("rewrite same-length native d_source data");
    fs::File::options()
        .write(true)
        .open(&stim_path)
        .expect("open rewritten native d_source data")
        .set_times(fixed_times)
        .expect("pin rewritten native d_source mtime");
    let second = Engine::default()
        .run_tran(&netlist, 1.5e-9, 100.0e-12)
        .expect("second native-cache transient solves");
    let second_out = transient_node_series(&second, "out");
    let _ = fs::remove_dir_all(dir);

    assert!(
        max_between(&second.time, second_out, 1.1e-9, 1.5e-9) < 0.1,
        "same-length native d_source rewrite with unchanged mtime must invalidate cached rows"
    );
}

#[test]
fn d_source_accepts_unbounded_negative_input_load_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("d_source").expect("d_source is registered");

    XspiceInstance::new(
        "a_d_source_negative_input_load",
        model,
        vec![PortConnection::DigitalVector(vec![1])],
        &[("input_load".to_string(), -1.0e-12)],
        &[],
        &[],
        &[],
    )
    .expect("ngspice accepts negative d_source input_load at construction");
}

#[test]
fn d_source_drives_dac_bridge_from_stimulus_file_like_ngspice() {
    let result = run_temp_deck(
        "rspice-d-source",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* d_source to dac_bridge oracle
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
rload out 0 1k
.end
",
        3.0e-9,
        100.0e-12,
    );
    let out = transient_node_series(&result, "out");

    assert!(
        out.first().copied().unwrap_or(f64::NAN).abs() < 0.1,
        "d_source t=0 vector should initialize dac low like ngspice, got {:?}",
        out.first()
    );
    let high_window_max = max_between(&result.time, out, 1.15e-9, 1.9e-9);
    assert!(
        high_window_max > 4.9,
        "d_source 1ns event should drive dac high like ngspice, max={high_window_max}"
    );
    let low_window_min = min_between(&result.time, out, 2.15e-9, 3.0e-9);
    assert!(
        low_window_min < 0.1,
        "d_source 2ns event should drive dac low like ngspice, min={low_window_min}"
    );
}

#[test]
fn dac_bridge_omitted_out_undef_uses_midpoint_when_levels_specified_like_ngspice() {
    let result = run_temp_deck(
        "rspice-dac-out-undef-midpoint",
        "0 Us\n",
        "\
* dac_bridge computes omitted out_undef from explicit output levels
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 t_rise=1p t_fall=1p)
rload out 0 1k
.end
",
        100.0e-12,
        10.0e-12,
    );

    let out = transient_node_series(&result, "out");
    let initial = out.first().copied().unwrap_or(f64::NAN);
    assert!(
        (initial - 2.5).abs() < 1.0e-6,
        "ngspice dac_bridge uses midpoint out_undef when out_low/out_high are provided and out_undef is omitted, got {initial}"
    );
}

#[test]
fn dac_bridge_reversed_output_levels_step_like_ngspice_source() {
    let result = run_temp_deck(
        "rspice-dac-reversed-levels",
        "0 0s\n1n 1s\n",
        "\
* dac_bridge reversed output levels follow ngspice signed-slope branches
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=5 out_high=0 out_undef=2.5 t_rise=1n t_fall=1n)
rload out 0 1k
.end
",
        1.4e-9,
        0.2e-9,
    );

    let out = transient_node_series(&result, "out");
    let before_event = max_between(&result.time, out, 0.0, 0.99e-9);
    let after_event = min_between(&result.time, out, 1.01e-9, 1.4e-9);
    assert!(
        (before_event - 5.0).abs() < 1.0e-9,
        "reversed-level dac_bridge should hold ZERO at out_low before the event, got {before_event}"
    );
    assert!(
        after_event.abs() < 1.0e-9,
        "ngspice source branches step reversed-level ONE output to out_high instead of ramping, got {after_event}"
    );
}

#[test]
fn d_source_equal_timepoints_return_only_initial_state_like_ngspice() {
    let dir = unique_temp_dir("rspice-d-source-equal-timepoints");
    fs::create_dir_all(&dir).expect("create d_source fixture dir");
    fs::write(dir.join("stim.stim"), "0 0s\n1n 1s\n1n 0s\n").expect("write d_source stimulus");
    let deck_path = dir.join("deck.cir");
    fs::write(
        &deck_path,
        "\
* d_source duplicate timepoint oracle
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=1 out_undef=0.5 t_rise=1p t_fall=1p)
rload out 0 1k
.end
",
    )
    .expect("write d_source deck");

    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, 2.0e-9, 100.0e-12)
        .expect("ngspice logs malformed d_source data and returns only the initial state");
    let _ = fs::remove_dir_all(dir);
    let out = transient_node_series(&result, "out");
    assert!(
        out.iter().all(|value| value.abs() < 1.0e-9),
        "malformed d_source should leave dac_bridge at the zero-initialized source state, got {out:?}"
    );
}

#[test]
fn d_source_negative_first_timepoint_stays_unknown_like_ngspice() {
    let result = run_temp_deck(
        "rspice-d-source-negative-first-timepoint",
        "-1n 1s\n1n 0s\n",
        "\
* d_source negative first timepoint oracle
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.end
",
        1.5e-9,
        100.0e-12,
    );

    let trace = digital_tokens(&result, "d");
    assert!(
        trace
            .iter()
            .any(|(time, token)| *time == 0.0 && token == "Uu"),
        "ngspice leaves d_source unknown when the first stimulus row is negative, got {trace:?}"
    );
    assert!(
        !trace
            .iter()
            .any(|(time, token)| *time > 0.0 && token != "Uu"),
        "ngspice never advances past a negative first d_source row, got {trace:?}"
    );
}

#[test]
fn d_source_indented_star_rows_return_only_initial_state_like_ngspice() {
    let dir = unique_temp_dir("rspice-d-source-indented-star");
    fs::create_dir_all(&dir).expect("create d_source fixture dir");
    fs::write(dir.join("stim.stim"), "0 0s\n * indented comment\n1n 1s\n")
        .expect("write d_source stimulus");
    let deck_path = dir.join("deck.cir");
    fs::write(
        &deck_path,
        "\
* d_source column-zero comment oracle
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.end
",
    )
    .expect("write d_source deck");

    let netlist = parse_file(&deck_path);
    Engine::default()
        .run_tran(&netlist, 1.5e-9, 100.0e-12)
        .expect("ngspice logs malformed d_source rows and returns only the initial state");
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn d_source_nonofficial_digital_token_aliases_return_only_initial_state_like_ngspice() {
    for token in ["us", "Xs", "1S", "Z"] {
        let dir = unique_temp_dir(&format!("rspice-d-source-token-{token}"));
        fs::create_dir_all(&dir).expect("create d_source fixture dir");
        fs::write(dir.join("stim.stim"), format!("0 {token}\n")).expect("write d_source stimulus");
        let deck_path = dir.join("deck.cir");
        fs::write(
            &deck_path,
            "\
* d_source exact token oracle
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.end
",
        )
        .expect("write d_source deck");

        let netlist = parse_file(&deck_path);
        Engine::default()
            .run_tran(&netlist, 0.2e-9, 100.0e-12)
            .unwrap_or_else(|err| {
                panic!(
                    "ngspice logs invalid d_source token alias {token} and returns only the initial state: {err}"
                )
            });
        let _ = fs::remove_dir_all(dir);
    }
}

#[test]
fn d_source_applies_suffix_after_exponent_in_times_like_ngspice() {
    let result = run_temp_deck(
        "rspice-d-source-time-exponent-suffix",
        "0 0s\n1e3p 1s\n",
        "\
* d_source time numeric conversion oracle
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.end
",
        1.4e-9,
        100.0e-12,
    );

    let trace = digital_tokens(&result, "d");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "1s"),
        "ngspice converts d_source time token 1e3p to 1 ns, got {trace:?}"
    );
}

#[test]
fn d_source_accepts_cnvgettok_separators_like_ngspice() {
    let result = run_temp_deck(
        "rspice-d-source-cnv-separators",
        "0=0s\n(1n,1s)\n",
        "\
* d_source CNVgettok separator oracle
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.end
",
        1.4e-9,
        100.0e-12,
    );

    let trace = digital_tokens(&result, "d");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "1s"),
        "ngspice d_source tokenizes '=(), ' separators in stimulus files, got {trace:?}"
    );
}

#[test]
fn adc_and_dac_bridges_support_vectors_and_official_defaults() {
    let deck = "\
* vector adc/dac bridge defaults
vlow ain0 0 dc 0.05
vhigh ain1 0 dc 0.95
aadc [ain0 ain1] [d0 d1] adc
adac [d0 d1] [out0 out1] dac
.model adc adc_bridge (rise_delay=1p fall_delay=1p)
.model dac dac_bridge (t_rise=1p t_fall=1p)
r0 out0 0 1k
r1 out1 0 1k
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.0e-9, 100.0e-12)
        .expect("transient solves");

    let out0 = transient_node_series(&result, "out0");
    let out1 = transient_node_series(&result, "out1");
    assert!(
        out0.last().copied().unwrap_or(f64::NAN).abs() < 1.0e-9,
        "ADC vector low bit should drive DAC default low level 0 V, got {:?}",
        out0.last()
    );
    assert!(
        (out1.last().copied().unwrap_or(f64::NAN) - 1.0).abs() < 1.0e-9,
        "ADC vector high bit should drive DAC official default high level 1 V, got {:?}",
        out1.last()
    );
}

#[test]
fn d_source_records_digital_event_trace() {
    let result = run_temp_deck(
        "rspice-d-source-trace",
        "0 0s\n1n 1s\n2n Uu\n",
        "\
* d_source digital trace oracle
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.end
",
        2.5e-9,
        100.0e-12,
    );

    let trace = digital_tokens(&result, "d");
    assert!(
        trace
            .iter()
            .any(|(time, token)| *time == 0.0 && token == "0s"),
        "missing t=0 0s digital event in {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "1s"),
        "missing t=1ns 1s digital event in {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.0e-9).abs() <= 1.0e-18 && token == "Uu"),
        "missing t=2ns Uu digital event in {trace:?}"
    );
}

#[test]
fn d_buffer_inertial_unknown_third_value_uses_stable_previous_delay_like_ngspice() {
    let result = run_temp_deck(
        "rspice-d-buffer-inertial-unknown-third",
        "0 0s\n1n 1s\n1.2n Us\n",
        "\
* d_buffer inertial pending-transition unknown delay oracle
a_src [din] src
a_buf [din] [out] buf
.model src d_source (input_file=\"stim.stim\")
.model buf d_buffer (rise_delay=1n fall_delay=4n inertial_delay=true)
.end
",
        3.0e-9,
        50.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.2e-9).abs() <= 1.0e-18 && token == "Us"),
        "unknown transition during pending low-to-high pulse should use the stable low value's rise delay like ngspice, got {trace:?}"
    );
}

#[test]
fn bidi_bridge_forced_adc_drives_digital_vector_from_analog_thresholds() {
    let netlist = Netlist::parse(
        "\
* bidi_bridge forced analog-to-digital direction
vin a 0 pwl(0 0 1n 0 1.01n 3.3)
ab a [d] null bd
.model bd bidi_bridge (direction=1 in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)
.end
",
    )
    .expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 2.0e-9, 50.0e-12)
        .expect("transient solves");

    let trace = digital_tokens(&result, "d");
    assert!(
        trace
            .iter()
            .any(|(time, token)| *time <= 2.0e-12 && token == "0s"),
        "forced ADC should emit a digital zero for the low analog input: {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| *time >= 1.0e-9 && token == "1s"),
        "forced ADC should emit a digital one after the analog input crosses in_high: {trace:?}"
    );
}

#[test]
fn bidi_bridge_strength_two_emits_high_z_strength_like_ngspice() {
    let netlist = Netlist::parse(
        "\
* bidi_bridge strength=2 high-z strength oracle
vin a 0 dc 3.3
ab [a] [d] null bd
.model bd bidi_bridge (direction=1 strength=2 in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)
.end
",
    )
    .expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.0e-9, 100.0e-12)
        .expect("transient solves");

    let trace = digital_tokens(&result, "d");
    assert!(
        trace
            .iter()
            .any(|(time, token)| *time <= 2.0e-12 && token == "1z"),
        "ngspice bidi_bridge passes strength=2 through as HI_IMPEDANCE, got {trace:?}"
    );
}

#[test]
fn bidi_bridge_forced_dac_drives_analog_vector_from_digital_input() {
    let result = run_temp_deck(
        "rspice-bidi-bridge-dac",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* bidi_bridge forced digital-to-analog direction
a_src [d] src
ab a [d] null bd
.model src d_source (input_file=\"stim.stim\")
.model bd bidi_bridge (direction=0 out_low=0 out_high=5 t_rise=1p t_fall=1p r_stl=1 r_sth=1 drive_low=10 drive_high=10)
rload a 0 1k
.end
",
        3.0e-9,
        50.0e-12,
    );
    let a = transient_node_series(&result, "a");

    let low_start = value_near_time(&result.time, a, 0.0);
    assert!(
        low_start.abs() < 0.05,
        "forced DAC should initially drive the analog node low, got {low_start}"
    );
    let high_window = max_between(&result.time, a, 1.1e-9, 1.9e-9);
    assert!(
        high_window > 4.9,
        "forced DAC should drive the analog node high after the digital one, max={high_window}"
    );
    let low_window = min_between(&result.time, a, 2.1e-9, 3.0e-9);
    assert!(
        low_window < 0.05,
        "forced DAC should return the analog node low after the digital zero, min={low_window}"
    );
}

#[test]
fn bidi_bridge_resistive_unknown_uses_asymmetric_open_circuit_target_like_ngspice() {
    let result = run_temp_deck(
        "rspice-bidi-bridge-ur-target",
        "0 Ur\n",
        "\
* bidi_bridge resistive unknown open-circuit target oracle
a_src [d] src
ab [a] [d] null bd
.model src d_source (input_file=\"stim.stim\")
.model bd bidi_bridge (direction=0 out_low=0 out_high=4 r_low=1000 r_high=3000 t_rise=1p t_fall=1p)
rload a 0 1000
.end
",
        1.0e-9,
        100.0e-12,
    );

    let a = transient_node_series(&result, "a");
    let settled = value_near_time(&result.time, a, 1.0e-9);
    assert!(
        (settled - 4.0 / 7.0).abs() <= 1.0e-6,
        "ngspice biases Ur toward r_low/(r_low+r_high), expected 4/7 V, got {settled:e}"
    );
}

#[test]
fn bidi_bridge_strong_unknown_biases_high_when_drive_high_exceeds_drive_low_like_ngspice() {
    let result = run_temp_deck(
        "rspice-bidi-bridge-us-target",
        "0 Us\n",
        "\
* bidi_bridge strong unknown open-circuit target oracle
a_src [d] src
ab [a] [d] null bd
.model src d_source (input_file=\"stim.stim\")
.model bd bidi_bridge (direction=0 out_low=0 out_high=4 drive_low=1 drive_high=10 r_stl=1000 r_sth=1000 t_rise=1p t_fall=1p)
rload a 0 1000
.end
",
        1.0e-9,
        100.0e-12,
    );

    let a = transient_node_series(&result, "a");
    let settled = value_near_time(&result.time, a, 1.0e-9);
    assert!(
        (settled - 8.0 / 3.0).abs() <= 1.0e-6,
        "ngspice biases strong unknown high when drive_high>drive_low, expected 8/3 V, got {settled:e}"
    );
}

#[test]
fn bidi_bridge_default_direction_passes_external_digital_drive_to_analog_side() {
    let result = run_temp_deck(
        "rspice-bidi-bridge-default-dac",
        "0 0s\n1n 1s\n",
        "\
* bidi_bridge default direction senses external digital drive
a_src [d] src
ab a [d] null bd
.model src d_source (input_file=\"stim.stim\")
.model bd bidi_bridge (out_low=0 out_high=3.3 t_rise=1p t_fall=1p r_stl=1 r_sth=1 drive_low=10 drive_high=10)
rload a 0 1k
.end
",
        2.0e-9,
        50.0e-12,
    );
    let a = transient_node_series(&result, "a");

    let high_window = max_between(&result.time, a, 1.1e-9, 2.0e-9);
    assert!(
        high_window > 3.2,
        "default bidirectional bridge should pass an external digital one to the analog side, max={high_window}"
    );
}

#[test]
fn digital_event_nodes_resolve_multiple_active_drivers_by_strength() {
    let netlist = Netlist::parse(
        "\
* XSPICE digital nodes must resolve simultaneous active drivers
aup [bus] pu
adn [bus] pd
.model pu d_pullup
.model pd d_pulldown
.end
",
    )
    .expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.0e-12, 1.0e-12)
        .expect("transient solves");

    let trace = digital_tokens(&result, "bus");
    assert!(
        trace
            .iter()
            .any(|(time, token)| *time == 0.0 && token == "Ur"),
        "equal resistive pullup and pulldown drivers must resolve to Ur, got {trace:?}"
    );
}

#[test]
fn open_output_models_clamp_delays_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();

    for (model_name, params) in [
        (
            "d_open_c",
            vec![
                ("open_delay".to_string(), 0.0),
                ("fall_delay".to_string(), -1.0e-9),
            ],
        ),
        (
            "d_open_e",
            vec![
                ("rise_delay".to_string(), 0.0),
                ("open_delay".to_string(), -1.0e-9),
            ],
        ),
    ] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));
        XspiceInstance::new(
            format!("a_{model_name}_delay_clamp"),
            model,
            vec![PortConnection::Digital(1), PortConnection::Digital(2)],
            &params,
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("{model_name} below-limit delays should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn open_output_models_use_clamped_delay_minimum_in_transient() {
    let result = run_temp_deck(
        "rspice-open-output-delay-clamp",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* d_open_c and d_open_e delay lower-bound clamp oracle
a_src [din] src
a_oc [din] [outc] oc
a_oe [din] [oute] oe
.model src d_source (input_file=\"stim.stim\")
.model oc d_open_c (open_delay=0 fall_delay=-1n)
.model oe d_open_e (rise_delay=0 open_delay=-1n)
.end
",
        2.2e-9,
        50.0e-12,
    );

    let outc = digital_tokens(&result, "outc");
    assert!(
        outc.iter()
            .any(|(time, token)| (*time - 1.0e-12).abs() <= 1.0e-18 && token == "0s"),
        "d_open_c fall_delay should clamp to 1ps like ngspice, got {outc:?}"
    );
    assert!(
        outc.iter()
            .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "1z"),
        "d_open_c open_delay should clamp to 1ps like ngspice, got {outc:?}"
    );

    let oute = digital_tokens(&result, "oute");
    assert!(
        oute.iter()
            .any(|(time, token)| (*time - 1.0e-12).abs() <= 1.0e-18 && token == "0z"),
        "d_open_e open_delay should clamp to 1ps like ngspice, got {oute:?}"
    );
    assert!(
        oute.iter()
            .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_open_e rise_delay should clamp to 1ps like ngspice, got {oute:?}"
    );
}

#[test]
fn d_open_c_outputs_high_z_for_logic_one_like_ngspice() {
    let result = run_temp_deck(
        "rspice-d-open-c",
        "0 0s\n1n 1s\n2n Uu\n",
        "\
* d_open_c official strength semantics
a_src [din] src
a_open [din] [out] oc
.model src d_source (input_file=\"stim.stim\")
.model oc d_open_c (fall_delay=1p open_delay=1p)
.end
",
        2.2e-9,
        0.1e-9,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-12).abs() <= 1.0e-18 && token == "0s"),
        "d_open_c input zero must drive a strong zero after fall_delay: {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "1z"),
        "d_open_c input one must emit logic-one high-Z after open_delay: {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.001e-9).abs() <= 1.0e-18 && token == "Uu"),
        "d_open_c unknown input must emit undetermined unknown: {trace:?}"
    );
}

#[test]
fn d_open_e_outputs_high_z_for_logic_zero_like_ngspice() {
    let result = run_temp_deck(
        "rspice-d-open-e",
        "0 0s\n1n 1s\n2n Uu\n",
        "\
* d_open_e official strength semantics
a_src [din] src
a_open [din] [out] oe
.model src d_source (input_file=\"stim.stim\")
.model oe d_open_e (open_delay=1p rise_delay=1p)
.end
",
        2.2e-9,
        0.1e-9,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-12).abs() <= 1.0e-18 && token == "0z"),
        "d_open_e input zero must emit logic-zero high-Z after open_delay: {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_open_e input one must drive a strong one after rise_delay: {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.001e-9).abs() <= 1.0e-18 && token == "Uu"),
        "d_open_e unknown input must emit undetermined unknown: {trace:?}"
    );
}

#[test]
fn open_output_inertial_unknown_third_value_uses_stable_previous_delay_like_ngspice() {
    let result = run_temp_deck(
        "rspice-open-output-inertial-unknown-third",
        "0 0s\n5n 1s\n5.2n Us\n",
        "\
* d_open_c/d_open_e inertial pending-transition unknown delay oracle
a_src [din] src
a_oc [din] [outc] oc
a_oe [din] [oute] oe
.model src d_source (input_file=\"stim.stim\")
.model oc d_open_c (open_delay=1n fall_delay=4n inertial_delay=true)
.model oe d_open_e (rise_delay=1n open_delay=4n inertial_delay=true)
.end
",
        7.0e-9,
        50.0e-12,
    );

    let outc = digital_tokens(&result, "outc");
    assert!(
        outc.iter()
            .any(|(time, token)| (*time - 6.2e-9).abs() <= 1.0e-18 && token == "Uu"),
        "d_open_c unknown transition during pending low-to-open pulse should use stable low open_delay like ngspice, got {outc:?}"
    );

    let oute = digital_tokens(&result, "oute");
    assert!(
        oute.iter()
            .any(|(time, token)| (*time - 6.2e-9).abs() <= 1.0e-18 && token == "Uu"),
        "d_open_e unknown transition during pending low-to-high pulse should use stable low rise_delay like ngspice, got {oute:?}"
    );
}

#[test]
fn d_buffer_transport_delay_preserves_short_pulse() {
    let result = run_temp_deck(
        "rspice-d-buffer-transport-delay",
        "0 0s\n1n 1s\n1.4n 0s\n3n 1s\n",
        "\
* transport delay keeps a pulse shorter than the propagation delay
a_src [din] src
a_buf [din] [out] buf
.model src d_source (input_file=\"stim.stim\")
.model buf d_buffer (rise_delay=1n fall_delay=1n inertial_delay=false)
.end
",
        4.2e-9,
        0.1e-9,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.0e-9).abs() <= 1.0e-18 && token == "1s"),
        "transport delay must preserve the delayed high pulse, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.4e-9).abs() <= 1.0e-18 && token == "0s"),
        "transport delay must preserve the delayed return low, got {trace:?}"
    );
}

#[test]
fn d_buffer_inertial_delay_swallows_short_pulse() {
    let result = run_temp_deck(
        "rspice-d-buffer-inertial-delay",
        "0 0s\n1n 1s\n1.4n 0s\n3n 1s\n",
        "\
* inertial delay swallows a pulse shorter than the propagation delay
a_src [din] src
a_buf [din] [out] buf
.model src d_source (input_file=\"stim.stim\")
.model buf d_buffer (rise_delay=1n fall_delay=1n inertial_delay=true)
.end
",
        4.2e-9,
        0.1e-9,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        !trace
            .iter()
            .any(|(time, token)| *time < 3.5e-9 && token == "1s"),
        "inertial delay must swallow the 1ns..1.4ns input pulse, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 4.0e-9).abs() <= 1.0e-18 && token == "1s"),
        "inertial delay must still propagate a stable later transition, got {trace:?}"
    );
}

#[test]
fn d_tristate_clamps_delay_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry
        .get("d_tristate")
        .expect("d_tristate is registered");

    for value in [0.0, -1.0e-9] {
        XspiceInstance::new(
            format!("a_tristate_delay_clamp_{value:e}"),
            model.clone(),
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
            ],
            &[("delay".to_string(), value)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("d_tristate delay={value:e} should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn d_tristate_uses_clamped_delay_minimum_in_transient() {
    let result = run_temp_deck(
        "rspice-d-tristate-delay-clamp",
        "0 1s 1s\n1n 0s 1s\n",
        "\
* d_tristate delay lower-bound clamp oracle
a_src [din en] src
a_tri [din] [en] [out] tri
.model src d_source (input_file=\"stim.stim\")
.model tri d_tristate (delay=0)
.end
",
        1.2e-9,
        50.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-12).abs() <= 1.0e-18 && token == "1s"),
        "d_tristate initial output should clamp zero delay to 1ps like ngspice, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_tristate transition should clamp zero delay to 1ps like ngspice, got {trace:?}"
    );
}

#[test]
fn d_tristate_uses_official_delay_parameter() {
    let result = run_temp_deck(
        "rspice-d-tristate-delay",
        "0 1s 1s\n",
        "\
* d_tristate has a single official delay parameter
a_src [din en] src
a_tri [din] [en] [out] tri
.model src d_source (input_file=\"stim.stim\")
.model tri d_tristate (delay=250p)
.end
",
        1.2e-9,
        50.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 250.0e-12).abs() <= 1.0e-18 && token == "1s"),
        "d_tristate must use delay=250p for the initial enabled output, got {trace:?}"
    );
    assert!(
        !trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_tristate must not fall back to rise_delay default when delay is set, got {trace:?}"
    );
}

#[test]
fn d_tristate_disabled_output_preserves_input_state_high_z() {
    let result = run_temp_deck(
        "rspice-d-tristate-highz-state",
        "0 1s 0s\n1n 0s 0s\n",
        "\
* disabled d_tristate preserves the input state and emits high-Z strength
a_src [din en] src
a_tri [din] [en] [out] tri
.model src d_source (input_file=\"stim.stim\")
.model tri d_tristate (delay=1p)
.end
",
        1.2e-9,
        0.1e-9,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-12).abs() <= 1.0e-18 && token == "1z"),
        "disabled d_tristate with high input must emit 1z after delay, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "0z"),
        "disabled d_tristate with low input must emit 0z after delay, got {trace:?}"
    );
}

#[test]
fn d_tristate_inertial_keeps_independent_state_and_strength_transitions() {
    let result = run_temp_deck(
        "rspice-d-tristate-inertial-two-channel",
        "0 0s 0s\n1n 1s 0s\n1.2n 1s 1s\n",
        "\
* official d_tristate inertial delay tracks state and strength channels separately
a_src [din en] src
a_tri [din] [en] [out] tri
.model src d_source (input_file=\"stim.stim\")
.model tri d_tristate (delay=1n inertial_delay=true)
.end
",
        2.5e-9,
        0.1e-9,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.0e-9).abs() <= 1.0e-18 && token == "1z"),
        "d_tristate must preserve the pending state transition at 2ns, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.2e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_tristate must apply the later strength transition at 2.2ns, got {trace:?}"
    );
}

#[test]
fn d_lut_uses_official_table_string_order_and_delays() {
    let result = run_temp_deck(
        "rspice-d-lut-table-delays",
        "0 0s 0s\n1n 1s 0s\n2n 1s 1s\n",
        "\
* d_lut uses low-order input bits and separate rise/fall delays
a_src [a b] src
a_lut [a b] [out] lut
.model src d_source (input_file=\"stim.stim\")
.model lut d_lut (table_values=\"0110\" rise_delay=100p fall_delay=200p)
.end
",
        2.5e-9,
        50.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.1e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_lut input vector 10 should index table[1] and rise after rise_delay, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.2e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_lut input vector 11 should index table[3] and fall after fall_delay, got {trace:?}"
    );
}

#[test]
fn d_genlut_uses_concatenated_tables_and_per_output_delays() {
    let result = run_temp_deck(
        "rspice-d-genlut-table-delays",
        "0 0s 0s\n1n 1s 0s\n2n 1s 1s\n",
        "\
* d_genlut concatenates one truth table per output bit
a_src [a b] src
a_lut [a b] [sum carry] lut
.model src d_source (input_file=\"stim.stim\")
.model lut d_genlut (table_values=\"01100001\" rise_delay=[100p 400p] fall_delay=[200p 300p])
.end
",
        2.7e-9,
        50.0e-12,
    );

    let sum = digital_tokens(&result, "sum");
    let carry = digital_tokens(&result, "carry");
    assert!(
        sum.iter()
            .any(|(time, token)| (*time - 1.1e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_genlut output[0] should use table[1] and rise_delay[0], got {sum:?}"
    );
    assert!(
        sum.iter()
            .any(|(time, token)| (*time - 2.2e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_genlut output[0] should fall with fall_delay[0], got {sum:?}"
    );
    assert!(
        carry
            .iter()
            .any(|(time, token)| (*time - 2.4e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_genlut output[1] should use the second concatenated table and rise_delay[1], got {carry:?}"
    );
    assert!(
        !carry
            .iter()
            .any(|(time, token)| (*time - 2.2e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_genlut output[1] must not reuse output[0]'s fall delay, got {carry:?}"
    );
}

#[test]
fn d_fdiv_clamps_delays_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("d_fdiv").expect("d_fdiv is registered");

    XspiceInstance::new(
        "a_fdiv_delay_clamp".to_string(),
        model,
        vec![PortConnection::Digital(1), PortConnection::Digital(2)],
        &[
            ("rise_delay".to_string(), 0.0),
            ("fall_delay".to_string(), -1.0e-9),
        ],
        &[],
        &[],
        &[],
    )
    .unwrap_or_else(|err| panic!("d_fdiv below-limit delays should clamp like ngspice, got {err}"));
}

#[test]
fn d_fdiv_accepts_and_clamps_counter_params_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("d_fdiv").expect("d_fdiv is registered");

    for (param_name, value) in [
        ("div_factor", 0.0),
        ("high_cycles", -1.0),
        ("i_count", -2.0),
    ] {
        XspiceInstance::new(
            format!("a_fdiv_{param_name}_clamp"),
            model.clone(),
            vec![PortConnection::Digital(1), PortConnection::Digital(2)],
            &[(param_name.to_string(), value)],
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("d_fdiv {param_name}={value:e} should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn d_fdiv_uses_clamped_delay_minimum_in_transient() {
    let result = run_temp_deck(
        "rspice-d-fdiv-delay-clamp",
        "0 0s\n1n 1s\n1.5n 0s\n2n 1s\n2.5n 0s\n3n 1s\n",
        "\
* d_fdiv delay lower-bound clamp oracle
a_src [clk] src
a_div [clk] [out] div
.model src d_source (input_file=\"stim.stim\")
.model div d_fdiv (div_factor=2 high_cycles=1 rise_delay=0 fall_delay=-1n)
.end
",
        3.4e-9,
        50.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.001e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_fdiv rise_delay should clamp to 1ps like ngspice, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.001e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_fdiv fall_delay should clamp to 1ps like ngspice, got {trace:?}"
    );
}

#[test]
fn d_fdiv_counts_rising_edges_and_uses_high_cycle_window() {
    let result = run_temp_deck(
        "rspice-d-fdiv-divide-window",
        "0 0s\n1n 1s\n1.2n 0s\n2n 1s\n2.2n 0s\n3n 1s\n3.2n 0s\n4n 1s\n4.2n 0s\n5n 1s\n",
        "\
* d_fdiv divides rising input edges and keeps output high for high_cycles counts
a_src [clk] src
a_div [clk] [out] div
.model src d_source (input_file=\"stim.stim\")
.model div d_fdiv (div_factor=4 high_cycles=2 rise_delay=100p fall_delay=200p)
.end
",
        5.4e-9,
        50.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.1e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_fdiv first rising edge should raise output after rise_delay, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 3.2e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_fdiv high_cycles=2 should drop output on the third rising edge after fall_delay, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 5.1e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_fdiv div_factor=4 should restart the high window on the fifth rising edge, got {trace:?}"
    );
}

#[test]
fn d_state_accepts_official_nullable_input_and_reset_ports() {
    let dir = unique_temp_dir("rspice-d-state-null-ports");
    fs::create_dir_all(&dir).expect("create d_state fixture dir");
    fs::write(dir.join("clk.stim"), "0 0s\n1n 1s\n").expect("write d_state clock");
    fs::write(dir.join("state.tbl"), "0 1s -> 0\n").expect("write d_state table");
    let deck_path = dir.join("deck.cir");

    fs::write(
        &deck_path,
        "\
* d_state official nullable input vector and reset
a_clk [clk] src
a_state null clk null [out] sm
.model src d_source (input_file=\"clk.stim\")
.model sm d_state (state_file=\"state.tbl\" reset_state=0 clk_delay=1p reset_delay=1p)
.end
",
    )
    .expect("write d_state deck");

    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, 1.5e-9, 50.0e-12)
        .expect("transient solves");
    let _ = fs::remove_dir_all(dir);

    let out = digital_tokens(&result, "out");
    assert!(
        out.iter()
            .any(|(time, token)| *time == 0.0 && token == "1s"),
        "d_state with null input and null reset should initialize from reset_state output, got {out:?}"
    );
}

#[test]
fn d_state_ignores_arrow_position_token_like_ngspice() {
    let dir = unique_temp_dir("rspice-d-state-arrow-token");
    fs::create_dir_all(&dir).expect("create d_state fixture dir");
    fs::write(dir.join("clk.stim"), "0 0s\n1n 1s\n").expect("write d_state clock");
    fs::write(dir.join("state.tbl"), "0 1s bogus 0\n").expect("write d_state table");
    let deck_path = dir.join("deck.cir");

    fs::write(
        &deck_path,
        "\
* d_state arrow-position token oracle
a_clk [clk] src
a_state null clk null [out] sm
.model src d_source (input_file=\"clk.stim\")
.model sm d_state (state_file=\"state.tbl\" reset_state=0 clk_delay=1p reset_delay=1p)
.end
",
    )
    .expect("write d_state deck");

    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, 1.5e-9, 50.0e-12)
        .expect("transient solves");
    let _ = fs::remove_dir_all(dir);

    let out = digital_tokens(&result, "out");
    assert!(
        out.iter()
            .any(|(time, token)| *time == 0.0 && token == "1s"),
        "ngspice ignores the d_state arrow-position token and initializes high, got {out:?}"
    );
}

#[test]
fn d_state_accepts_cnvgettok_separators_like_ngspice() {
    let dir = unique_temp_dir("rspice-d-state-cnv-separators");
    fs::create_dir_all(&dir).expect("create d_state fixture dir");
    fs::write(dir.join("clk.stim"), "0 0s\n1n 1s\n").expect("write d_state clock");
    fs::write(dir.join("state.tbl"), "(0),(1s),bogus,(0)\n").expect("write d_state table");
    let deck_path = dir.join("deck.cir");

    fs::write(
        &deck_path,
        "\
* d_state CNVgettok separator oracle
a_clk [clk] src
a_state null clk null [out] sm
.model src d_source (input_file=\"clk.stim\")
.model sm d_state (state_file=\"state.tbl\" reset_state=0 clk_delay=1p reset_delay=1p)
.end
",
    )
    .expect("write d_state deck");

    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, 1.5e-9, 50.0e-12)
        .expect("transient solves");
    let _ = fs::remove_dir_all(dir);

    let out = digital_tokens(&result, "out");
    assert!(
        out.iter()
            .any(|(time, token)| *time == 0.0 && token == "1s"),
        "ngspice d_state tokenizes '=(), ' separators in state files, got {out:?}"
    );
}

#[test]
fn d_state_indented_star_rows_return_without_fatal_error_like_ngspice() {
    let dir = unique_temp_dir("rspice-d-state-indented-star");
    fs::create_dir_all(&dir).expect("create d_state fixture dir");
    fs::write(dir.join("clk.stim"), "0 0s\n1n 1s\n").expect("write d_state clock");
    fs::write(dir.join("state.tbl"), " * indented comment\n0 1s -> 0\n")
        .expect("write d_state table");
    let deck_path = dir.join("deck.cir");

    fs::write(
        &deck_path,
        "\
* d_state column-zero comment oracle
a_clk [clk] src
a_state null clk null [out] sm
.model src d_source (input_file=\"clk.stim\")
.model sm d_state (state_file=\"state.tbl\" reset_state=0 clk_delay=1p reset_delay=1p)
.end
",
    )
    .expect("write d_state deck");

    let netlist = parse_file(&deck_path);
    Engine::default()
        .run_tran(&netlist, 1.5e-9, 50.0e-12)
        .expect("ngspice logs malformed d_state rows and returns without fatal error");
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn d_state_missing_reset_state_returns_without_fatal_error_like_ngspice() {
    let dir = unique_temp_dir("rspice-d-state-missing-reset");
    fs::create_dir_all(&dir).expect("create d_state fixture dir");
    fs::write(dir.join("clk.stim"), "0 0s\n1n 1s\n").expect("write d_state clock");
    fs::write(dir.join("state.tbl"), "0 1s -> 0\n").expect("write d_state table");
    let deck_path = dir.join("deck.cir");

    fs::write(
        &deck_path,
        "\
* d_state missing reset_state index oracle
a_clk [clk] src
a_state null clk null [out] sm
.model src d_source (input_file=\"clk.stim\")
.model sm d_state (state_file=\"state.tbl\" reset_state=1 clk_delay=1p reset_delay=1p)
.end
",
    )
    .expect("write d_state deck");

    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, 0.2e-9, 50.0e-12)
        .expect("ngspice logs d_state index errors and returns without fatal error");
    let _ = fs::remove_dir_all(dir);

    let out = digital_tokens(&result, "out");
    assert_eq!(
        out,
        vec![(0.0, "1s".to_string())],
        "ngspice leaves d_state indices at row zero when reset_state is missing, got {out:?}"
    );
}

#[test]
fn d_state_missing_transition_target_returns_without_fatal_error_like_ngspice() {
    let dir = unique_temp_dir("rspice-d-state-missing-transition");
    fs::create_dir_all(&dir).expect("create d_state fixture dir");
    fs::write(dir.join("in.stim"), "0 1s\n").expect("write d_state input");
    fs::write(dir.join("clk.stim"), "0 0s\n1n 1s\n").expect("write d_state clock");
    fs::write(dir.join("state.tbl"), "0 1s 1 -> 1\n").expect("write d_state table");
    let deck_path = dir.join("deck.cir");

    fs::write(
        &deck_path,
        "\
* d_state missing transition target oracle
a_in [din] insrc
a_clk [clk] clksrc
a_state [din] clk null [out] sm
.model insrc d_source (input_file=\"in.stim\")
.model clksrc d_source (input_file=\"clk.stim\")
.model sm d_state (state_file=\"state.tbl\" reset_state=0 clk_delay=1p reset_delay=1p)
.end
",
    )
    .expect("write d_state deck");

    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, 1.5e-9, 50.0e-12)
        .expect("ngspice logs d_state transition index errors and returns without fatal error");
    let _ = fs::remove_dir_all(dir);

    let out = digital_tokens(&result, "out");
    assert_eq!(
        out,
        vec![(0.0, "1s".to_string())],
        "missing d_state transition targets should leave the previous output event unchanged, got {out:?}"
    );
}

#[test]
fn d_state_applies_suffix_after_exponent_in_state_ids_like_ngspice() {
    let dir = unique_temp_dir("rspice-d-state-exponent-suffix");
    fs::create_dir_all(&dir).expect("create d_state fixture dir");
    fs::write(dir.join("clk.stim"), "0 0s\n1n 1s\n").expect("write d_state clock");
    fs::write(dir.join("state.tbl"), "1e3k 1s -> 1e3k\n").expect("write d_state table");
    let deck_path = dir.join("deck.cir");

    fs::write(
        &deck_path,
        "\
* d_state state id numeric conversion oracle
a_clk [clk] src
a_state null clk null [out] sm
.model src d_source (input_file=\"clk.stim\")
.model sm d_state (state_file=\"state.tbl\" reset_state=1000000 clk_delay=1p reset_delay=1p)
.end
",
    )
    .expect("write d_state deck");

    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, 0.2e-9, 50.0e-12)
        .expect("transient solves");
    let _ = fs::remove_dir_all(dir);

    let out = digital_tokens(&result, "out");
    assert!(
        out.iter()
            .any(|(time, token)| *time == 0.0 && token == "1s"),
        "ngspice converts d_state id 1e3k to reset_state 1000000, got {out:?}"
    );
}

#[test]
fn d_state_accepts_unbounded_negative_delay_and_load_parameters_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let d_state = registry.get("d_state").expect("d_state is registered");

    XspiceInstance::new(
        "a_d_state_negative_unbounded_params",
        d_state,
        vec![
            PortConnection::DigitalVector(vec![1]),
            PortConnection::Digital(2),
            PortConnection::Digital(3),
            PortConnection::DigitalVector(vec![4]),
        ],
        &[
            ("clk_delay".to_string(), -1.0e-9),
            ("reset_delay".to_string(), -1.0e-9),
            ("input_load".to_string(), -1.0e-12),
            ("clk_load".to_string(), -1.0e-12),
            ("reset_load".to_string(), -1.0e-12),
        ],
        &[],
        &[],
        &[],
    )
    .expect("ngspice accepts negative d_state delay/load parameters at construction");
}

#[test]
fn latches_clamp_delays_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();

    for (model_name, ports, params) in [
        (
            "d_dlatch",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
            ],
            vec![
                ("data_delay".to_string(), 0.0),
                ("enable_delay".to_string(), -1.0e-9),
                ("set_delay".to_string(), 0.0),
                ("reset_delay".to_string(), -1.0e-9),
                ("rise_delay".to_string(), 0.0),
                ("fall_delay".to_string(), -1.0e-9),
            ],
        ),
        (
            "d_srlatch",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
                PortConnection::Digital(7),
            ],
            vec![
                ("sr_delay".to_string(), 0.0),
                ("enable_delay".to_string(), -1.0e-9),
                ("set_delay".to_string(), 0.0),
                ("reset_delay".to_string(), -1.0e-9),
                ("rise_delay".to_string(), 0.0),
                ("fall_delay".to_string(), -1.0e-9),
            ],
        ),
    ] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));
        XspiceInstance::new(
            format!("a_{model_name}_delay_clamp"),
            model,
            ports,
            &params,
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("{model_name} below-limit delays should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn latches_use_clamped_delay_minimum_in_transient() {
    let result = run_temp_deck(
        "rspice-latch-delay-clamp",
        "0 1s 0s 0s 0s 1s 0s\n1n 1s 1s 0s 0s 1s 0s\n1.5n 0s 1s 0s 0s 0s 1s\n",
        "\
* latch delay lower-bound clamp oracle
a_src [data enable set reset s r] src
a_dl [data] [enable] [set] [reset] [outd] [noutd] dl
a_sr [s] [r] [enable] [set] [reset] [outs] [nouts] sr
.model src d_source (input_file=\"stim.stim\")
.model dl d_dlatch (ic=0 data_delay=0 enable_delay=-1n set_delay=0 reset_delay=-1n rise_delay=0 fall_delay=-1n)
.model sr d_srlatch (ic=0 sr_delay=0 enable_delay=-1n set_delay=0 reset_delay=-1n rise_delay=0 fall_delay=-1n)
.end
",
        2.0e-9,
        50.0e-12,
    );

    let outd = digital_tokens(&result, "outd");
    assert!(
        outd.iter()
            .any(|(time, token)| (*time - 1.002e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_dlatch enable rise should use clamped enable_delay + clamped rise_delay, got {outd:?}"
    );
    assert!(
        outd.iter()
            .any(|(time, token)| (*time - 1.502e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_dlatch data fall should use clamped data_delay + clamped fall_delay, got {outd:?}"
    );

    let outs = digital_tokens(&result, "outs");
    assert!(
        outs.iter()
            .any(|(time, token)| (*time - 1.002e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_srlatch enable set should use clamped enable_delay + clamped rise_delay, got {outs:?}"
    );
    assert!(
        outs.iter()
            .any(|(time, token)| (*time - 1.502e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_srlatch SR reset should use clamped sr_delay + clamped fall_delay, got {outs:?}"
    );
}

#[test]
fn d_dlatch_accepts_official_async_ports_and_combines_input_and_output_delays() {
    let result = run_temp_deck(
        "rspice-d-dlatch-official-delays",
        "0 0s 0s 0s 0s\n\
         1n 1s 1s 0s 0s\n\
         2n 0s 1s 0s 0s\n\
         3n 0s 1s 1s 0s\n\
         3.5n 1s 1s 0s 0s\n\
         4n 1s 1s 0s 1s\n",
        "\
* d_dlatch official six-port interface and delay composition
a_src [data enable set reset] src
a_latch data enable set reset out nout latch
.model src d_source (input_file=\"stim.stim\")
.model latch d_dlatch (ic=0 enable_delay=200p data_delay=300p set_delay=100p reset_delay=400p rise_delay=10p fall_delay=20p)
.end
",
        4.8e-9,
        50.0e-12,
    );

    let out = digital_tokens(&result, "out");
    let nout = digital_tokens(&result, "nout");
    assert!(
        out.iter()
            .any(|(time, token)| *time == 0.0 && token == "0s"),
        "d_dlatch ic=0 should initialize out low at t=0, got {out:?}"
    );
    assert!(
        nout.iter()
            .any(|(time, token)| *time == 0.0 && token == "1s"),
        "d_dlatch ic=0 should initialize nout high at t=0, got {nout:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 1.210e-9).abs() <= 1.0e-18 && token == "1s"),
        "enable transition should drive out high after enable_delay + rise_delay, got {out:?}"
    );
    assert!(
        nout.iter()
            .any(|(time, token)| (*time - 1.220e-9).abs() <= 1.0e-18 && token == "0s"),
        "enable transition should drive nout low after enable_delay + fall_delay, got {nout:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 2.320e-9).abs() <= 1.0e-18 && token == "0s"),
        "data transition should drive out low after data_delay + fall_delay, got {out:?}"
    );
    assert!(
        nout.iter()
            .any(|(time, token)| (*time - 2.310e-9).abs() <= 1.0e-18 && token == "1s"),
        "data transition should drive nout high after data_delay + rise_delay, got {nout:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 3.110e-9).abs() <= 1.0e-18 && token == "1s"),
        "async set should drive out high after set_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 4.420e-9).abs() <= 1.0e-18 && token == "0s"),
        "async reset should drive out low after reset_delay + fall_delay, got {out:?}"
    );
}

#[test]
fn sequential_models_accept_and_clamp_ic_outside_official_range_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();

    for (model_name, ports) in [
        (
            "d_dff",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
            ],
        ),
        (
            "d_tff",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
            ],
        ),
        (
            "d_jkff",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
                PortConnection::Digital(7),
            ],
        ),
        (
            "d_srff",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
                PortConnection::Digital(7),
            ],
        ),
        (
            "d_dlatch",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
            ],
        ),
        (
            "d_srlatch",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
                PortConnection::Digital(7),
            ],
        ),
    ] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        for ic in [-1.0, 3.0] {
            XspiceInstance::new(
                format!("a_{model_name}_ic_{ic:e}"),
                model.clone(),
                ports.clone(),
                &[("ic".to_string(), ic)],
                &[],
                &[],
                &[],
            )
            .unwrap_or_else(|err| {
                panic!("{model_name} ic={ic:e} should clamp like ngspice, got {err}")
            });
        }
    }
}

#[test]
fn edge_flipflops_clamp_delays_below_official_minimum_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let params = [
        ("clk_delay".to_string(), 0.0),
        ("set_delay".to_string(), -1.0e-9),
        ("reset_delay".to_string(), 0.0),
        ("rise_delay".to_string(), 0.0),
        ("fall_delay".to_string(), -1.0e-9),
    ];

    for (model_name, ports) in [
        (
            "d_dff",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
            ],
        ),
        (
            "d_tff",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
            ],
        ),
        (
            "d_jkff",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
                PortConnection::Digital(7),
            ],
        ),
        (
            "d_srff",
            vec![
                PortConnection::Digital(1),
                PortConnection::Digital(2),
                PortConnection::Digital(3),
                PortConnection::Digital(4),
                PortConnection::Digital(5),
                PortConnection::Digital(6),
                PortConnection::Digital(7),
            ],
        ),
    ] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));
        XspiceInstance::new(
            format!("a_{model_name}_delay_clamp"),
            model,
            ports,
            &params,
            &[],
            &[],
            &[],
        )
        .unwrap_or_else(|err| {
            panic!("{model_name} below-limit delays should clamp like ngspice, got {err}")
        });
    }
}

#[test]
fn d_dff_uses_clamped_delay_minimum_in_transient() {
    let result = run_temp_deck(
        "rspice-d-dff-delay-clamp",
        "0 1s 0s 0s 0s\n1n 1s 1s 0s 0s\n1.5n 0s 0s 0s 0s\n2n 0s 1s 0s 0s\n",
        "\
* d_dff delay lower-bound clamp oracle
a_src [data clk set reset] src
a_ff [data] [clk] [set] [reset] [out] [nout] ff
.model src d_source (input_file=\"stim.stim\")
.model ff d_dff (ic=0 clk_delay=0 set_delay=-1n reset_delay=0 rise_delay=0 fall_delay=-1n)
.end
",
        2.4e-9,
        50.0e-12,
    );

    let out = digital_tokens(&result, "out");
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 1.002e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_dff output rise should use clamped clk_delay + clamped rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 2.002e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_dff output fall should use clamped clk_delay + clamped fall_delay, got {out:?}"
    );
}

#[test]
fn d_dff_accepts_official_async_ports_and_combines_clock_and_output_delays() {
    let result = run_temp_deck(
        "rspice-d-dff-official-delays",
        "0 0s 0s 0s 0s\n\
         1n 1s 1s 0s 0s\n\
         2n 0s 0s 0s 0s\n\
         3n 0s 1s 0s 0s\n\
         4n 0s 1s 1s 0s\n\
         4.5n 0s 1s 0s 0s\n\
         5n 0s 1s 0s 1s\n",
        "\
* d_dff official six-port interface and delay composition
a_src [data clk set reset] src
a_ff data clk set reset out nout ff
.model src d_source (input_file=\"stim.stim\")
.model ff d_dff (ic=0 clk_delay=200p set_delay=100p reset_delay=400p rise_delay=10p fall_delay=20p)
.end
",
        5.8e-9,
        50.0e-12,
    );

    let out = digital_tokens(&result, "out");
    let nout = digital_tokens(&result, "nout");
    assert!(
        out.iter()
            .any(|(time, token)| *time == 0.0 && token == "0s"),
        "d_dff ic=0 should initialize out low at t=0, got {out:?}"
    );
    assert!(
        nout.iter()
            .any(|(time, token)| *time == 0.0 && token == "1s"),
        "d_dff ic=0 should initialize nout high at t=0, got {nout:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 1.210e-9).abs() <= 1.0e-18 && token == "1s"),
        "rising clock should sample data high after clk_delay + rise_delay, got {out:?}"
    );
    assert!(
        nout.iter()
            .any(|(time, token)| (*time - 1.220e-9).abs() <= 1.0e-18 && token == "0s"),
        "rising clock should drive nout low after clk_delay + fall_delay, got {nout:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 3.220e-9).abs() <= 1.0e-18 && token == "0s"),
        "rising clock should sample data low after clk_delay + fall_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 4.110e-9).abs() <= 1.0e-18 && token == "1s"),
        "async set should drive out high after set_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 5.420e-9).abs() <= 1.0e-18 && token == "0s"),
        "async reset should drive out low after reset_delay + fall_delay, got {out:?}"
    );
}

#[test]
fn d_tff_accepts_official_async_ports_and_combines_clock_and_output_delays() {
    let result = run_temp_deck(
        "rspice-d-tff-official-delays",
        "0 1s 0s 0s 0s\n\
         1n 1s 1s 0s 0s\n\
         2n 1s 0s 0s 0s\n\
         3n 1s 1s 0s 0s\n\
         4n 0s 1s 1s 0s\n\
         4.5n 0s 1s 0s 0s\n\
         5n 0s 1s 0s 1s\n",
        "\
* d_tff official six-port interface and delay composition
a_src [toggle clk set reset] src
a_ff toggle clk set reset out nout ff
.model src d_source (input_file=\"stim.stim\")
.model ff d_tff (ic=0 clk_delay=200p set_delay=100p reset_delay=400p rise_delay=10p fall_delay=20p)
.end
",
        5.8e-9,
        50.0e-12,
    );

    let out = digital_tokens(&result, "out");
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 1.210e-9).abs() <= 1.0e-18 && token == "1s"),
        "t=1 rising clock should toggle out high after clk_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 3.220e-9).abs() <= 1.0e-18 && token == "0s"),
        "second t=1 rising clock should toggle out low after clk_delay + fall_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 4.110e-9).abs() <= 1.0e-18 && token == "1s"),
        "async set should drive out high after set_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 5.420e-9).abs() <= 1.0e-18 && token == "0s"),
        "async reset should drive out low after reset_delay + fall_delay, got {out:?}"
    );
}

#[test]
fn d_jkff_accepts_official_ports_and_combines_delays() {
    let result = run_temp_deck(
        "rspice-d-jkff-official-delays",
        "0 0s 0s 0s 0s 0s\n\
         1n 1s 0s 1s 0s 0s\n\
         2n 1s 1s 0s 0s 0s\n\
         3n 1s 1s 1s 0s 0s\n\
         4n 0s 0s 1s 1s 0s\n\
         4.5n 0s 0s 1s 0s 0s\n\
         5n 0s 0s 1s 0s 1s\n",
        "\
* d_jkff official seven-port interface and delay composition
a_src [j k clk set reset] src
a_ff j k clk set reset out nout ff
.model src d_source (input_file=\"stim.stim\")
.model ff d_jkff (ic=0 clk_delay=200p set_delay=100p reset_delay=400p rise_delay=10p fall_delay=20p)
.end
",
        5.8e-9,
        50.0e-12,
    );

    let out = digital_tokens(&result, "out");
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 1.210e-9).abs() <= 1.0e-18 && token == "1s"),
        "J=1/K=0 rising clock should set out after clk_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 3.220e-9).abs() <= 1.0e-18 && token == "0s"),
        "J=1/K=1 rising clock should toggle out low after clk_delay + fall_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 4.110e-9).abs() <= 1.0e-18 && token == "1s"),
        "async set should drive out high after set_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 5.420e-9).abs() <= 1.0e-18 && token == "0s"),
        "async reset should drive out low after reset_delay + fall_delay, got {out:?}"
    );
}

#[test]
fn d_srff_accepts_official_ports_and_combines_delays() {
    let result = run_temp_deck(
        "rspice-d-srff-official-delays",
        "0 0s 0s 0s 0s 0s\n\
         1n 1s 0s 1s 0s 0s\n\
         2n 0s 1s 0s 0s 0s\n\
         3n 0s 1s 1s 0s 0s\n\
         4n 0s 0s 1s 1s 0s\n\
         4.5n 0s 0s 1s 0s 0s\n\
         5n 0s 0s 1s 0s 1s\n",
        "\
* d_srff official seven-port interface and delay composition
a_src [s r clk set reset] src
a_ff s r clk set reset out nout ff
.model src d_source (input_file=\"stim.stim\")
.model ff d_srff (ic=0 clk_delay=200p set_delay=100p reset_delay=400p rise_delay=10p fall_delay=20p)
.end
",
        5.8e-9,
        50.0e-12,
    );

    let out = digital_tokens(&result, "out");
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 1.210e-9).abs() <= 1.0e-18 && token == "1s"),
        "S=1/R=0 rising clock should set out after clk_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 3.220e-9).abs() <= 1.0e-18 && token == "0s"),
        "S=0/R=1 rising clock should reset out after clk_delay + fall_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 4.110e-9).abs() <= 1.0e-18 && token == "1s"),
        "async set should drive out high after set_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 5.420e-9).abs() <= 1.0e-18 && token == "0s"),
        "async reset should drive out low after reset_delay + fall_delay, got {out:?}"
    );
}

#[test]
fn d_srlatch_accepts_official_async_ports_and_combines_input_and_output_delays() {
    let result = run_temp_deck(
        "rspice-d-srlatch-official-delays",
        "0 0s 0s 0s 0s 0s\n\
         1n 1s 0s 1s 0s 0s\n\
         2n 0s 1s 1s 0s 0s\n\
         3n 0s 1s 1s 1s 0s\n\
         3.5n 1s 0s 1s 0s 0s\n\
         4n 1s 0s 1s 0s 1s\n",
        "\
* d_srlatch official seven-port interface and delay composition
a_src [s r enable set reset] src
a_latch s r enable set reset out nout latch
.model src d_source (input_file=\"stim.stim\")
.model latch d_srlatch (ic=0 enable_delay=200p sr_delay=300p set_delay=100p reset_delay=400p rise_delay=10p fall_delay=20p)
.end
",
        4.8e-9,
        50.0e-12,
    );

    let out = digital_tokens(&result, "out");
    let nout = digital_tokens(&result, "nout");
    assert!(
        out.iter()
            .any(|(time, token)| *time == 0.0 && token == "0s"),
        "d_srlatch ic=0 should initialize out low at t=0, got {out:?}"
    );
    assert!(
        nout.iter()
            .any(|(time, token)| *time == 0.0 && token == "1s"),
        "d_srlatch ic=0 should initialize nout high at t=0, got {nout:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 1.210e-9).abs() <= 1.0e-18 && token == "1s"),
        "enable transition should drive out high after enable_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 2.320e-9).abs() <= 1.0e-18 && token == "0s"),
        "S/R input transition should drive out low after sr_delay + fall_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 3.110e-9).abs() <= 1.0e-18 && token == "1s"),
        "async set should drive out high after set_delay + rise_delay, got {out:?}"
    );
    assert!(
        out.iter()
            .any(|(time, token)| (*time - 4.420e-9).abs() <= 1.0e-18 && token == "0s"),
        "async reset should drive out low after reset_delay + fall_delay, got {out:?}"
    );
}

#[test]
fn d_osc_uses_controlled_frequency_table_and_initial_phase() {
    let result = run_temp_deck(
        "rspice-d-osc-frequency",
        "",
        "\
* d_osc schedules strong digital transitions from controlled frequency
vctrl ctrl 0 0
aosc ctrl [out] osc
.model osc d_osc (cntl_array=[0 1] freq_array=[1g 1g] duty_cycle=0.25 init_phase=0)
.end
",
        1.25e-9,
        500.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 0.0).abs() <= 1.0e-18 && token == "0s"),
        "d_osc init_phase=0 should initialize low at t=0, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 0.75e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_osc duty_cycle=0.25 at 1GHz should rise at 0.75ns, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_osc duty_cycle=0.25 at 1GHz should fall at 1.0ns, got {trace:?}"
    );
}

#[test]
fn d_pwm_interpolates_duty_cycle_table_for_transition_times() {
    let result = run_temp_deck(
        "rspice-d-pwm-duty",
        "",
        "\
* d_pwm schedules digital transitions from interpolated duty cycle
vctrl ctrl 0 0
apwm ctrl [out] pwm
.model pwm d_pwm (cntl_array=[-1 1] dc_array=[0.25 0.75] frequency=1g init_phase=0)
.end
",
        1.75e-9,
        500.0e-12,
    );

    let trace = digital_tokens(&result, "out");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 0.0).abs() <= 1.0e-18 && token == "0s"),
        "d_pwm init_phase=0 should initialize low at t=0, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 0.5e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_pwm control=0 should interpolate duty cycle 0.5 and rise at 0.5ns, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_pwm control=0 should fall at 1.0ns for a 1GHz period, got {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.5e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_pwm should keep the interpolated 50 percent duty cycle across periods, got {trace:?}"
    );
}

#[test]
fn d_source_schedules_stimulus_times_as_breakpoints() {
    let result = run_temp_deck(
        "rspice-d-source-breakpoints",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* d_source should force transient stops at stimulus event times
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.end
",
        3.0e-9,
        5.0e-9,
    );

    let trace = digital_tokens(&result, "d");
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "1s"),
        "d_source must schedule the 1ns stimulus event as a transient breakpoint: {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.0e-9).abs() <= 1.0e-18 && token == "0s"),
        "d_source must schedule the 2ns stimulus event as a transient breakpoint: {trace:?}"
    );
}

#[test]
fn d_source_reaches_dac_bridge_independent_of_instance_order() {
    let result = run_temp_deck(
        "rspice-d-source-reversed",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* d_source should not depend on instance order
a_dac [d] [out] dac
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
rload out 0 1k
.end
",
        2.0e-9,
        100.0e-12,
    );
    let out = transient_node_series(&result, "out");
    let at_event = max_between(&result.time, out, 1.01e-9, 1.2e-9);
    assert!(
        at_event > 4.9,
        "reversed instance order should propagate the 1ns d_source event before stamping, got {at_event}"
    );
}

#[test]
fn dac_bridge_initial_high_uses_target_not_default_ramp_from_zero() {
    let result = run_temp_deck(
        "rspice-d-source-initial-high",
        "0 1s\n",
        "\
* dac_bridge initial high should resolve immediately
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5)
rload out 0 1k
.end
",
        200.0e-12,
        20.0e-12,
    );
    let out = transient_node_series(&result, "out");
    let early_min = min_between(&result.time, out, 0.0, 200.0e-12);
    assert!(
        early_min > 4.9,
        "initial high d_source row should not ramp from 0 V during the first transient window, min={early_min}"
    );
}

#[test]
fn dac_bridge_interrupted_ramp_starts_new_ramp_at_event_value() {
    let result = run_temp_deck(
        "rspice-d-source-interrupted-ramp",
        "0 0s\n250p 1s\n500p 0s\n",
        "\
* dac_bridge interrupted ramp continuity
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1n t_fall=1n)
rload out 0 1k
.end
",
        900.0e-12,
        50.0e-12,
    );
    let out = transient_node_series(&result, "out");
    let at_fall_event = value_near_time(&result.time, out, 500.0e-12);
    assert!(
        at_fall_event > 1.2 && at_fall_event < 1.3,
        "falling event should start from the rising ramp value at 500ps, got {at_fall_event}"
    );
    let completed_fall = value_near_time(&result.time, out, 750.0e-12);
    assert!(
        completed_fall.abs() < 1.0e-9,
        "ngspice uses the full-scale fall slope, so the interrupted 1.25 V fall should complete at 750ps; got {completed_fall}; times={:?}",
        result.time
    );
}

#[test]
fn dac_bridge_schedules_ramp_completion_breakpoint_like_ngspice() {
    let result = run_temp_deck(
        "rspice-dac-ramp-completion-breakpoint",
        "0 0s\n1n 1s\n",
        "\
* dac_bridge schedules a breakpoint when the analog ramp reaches its target
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=750p t_fall=750p)
rload out 0 1k
.end
",
        2.5e-9,
        2.0e-9,
    );
    let out = transient_node_series(&result, "out");
    let completion = value_near_time(&result.time, out, 1.75e-9);
    assert!(
        (completion - 5.0).abs() < 1.0e-9,
        "dac_bridge should land on the 1.75 ns ramp completion breakpoint with out=5 V, got {completion}; times={:?}",
        result.time
    );
}
