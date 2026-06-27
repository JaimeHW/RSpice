//! Native XSPICE analog code models pinned against ngspice code-model semantics.

use rspice_core::Complex64;
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::Netlist;

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
.model div_alias divide (out_gain=1 out_offset=0 den_lower_limit=1e-12)
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
        (out - 2.0).abs() < 1e-9,
        "d_dt alias should match differentiator offset behavior: got {out}"
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
fn xspice_pwl_interpolates_smooths_extrapolates_and_limits_like_ngspice() {
    let deck = "\
* XSPICE pwl lookup oracle
vbelow below 0 dc -1
vfirst first 0 dc 0
vmid mid 0 dc 0.5
vbreak break 0 dc 1
vabove above 0 dc 3
vlimbelow limbelow 0 dc -1
vlimabove limabove 0 dc 3
abelow below out_below lut
afirst first out_first lut
amid mid out_mid lut
abreak break out_break lut
aabove above out_above lut
alimbelow limbelow out_limbelow lutlim
alimabove limabove out_limabove lutlim
.model lut pwl (x_array=[0 1 2] y_array=[0 10 30] input_domain=0.01 fraction=false limit=false)
.model lutlim pwl (x_array=[0 1 2] y_array=[0 10 30] input_domain=0.01 fraction=false limit=true)
r1 out_below 0 1meg
r2 out_first 0 1meg
r3 out_mid 0 1meg
r4 out_break 0 1meg
r5 out_above 0 1meg
r6 out_limbelow 0 1meg
r7 out_limabove 0 1meg
.op
.end
";

    let below = op_voltage(deck, "out_below");
    let first = op_voltage(deck, "out_first");
    let mid = op_voltage(deck, "out_mid");
    let break_point = op_voltage(deck, "out_break");
    let above = op_voltage(deck, "out_above");
    let lim_below = op_voltage(deck, "out_limbelow");
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
.model lut pwl (x_array=[0 1] y_array=[0] input_domain=0.01 fraction=false)
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
.model lut pwlts (x_array=[0 1n] y_array=[0])
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
