//! Oracle coverage for native Berkeley MOS3 (`LEVEL=3`) support.
//!
//! NGSpice 46 is the oracle. These tests exercise RSpice's parser-builder-
//! solver path and verify that MOS3 routes through the native Berkeley model
//! rather than the simplified short-channel fallback.

#![allow(clippy::excessive_precision)]

use rspice_core::circuit::DeviceOpEntry;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, Netlist};

#[derive(Clone, Copy, Debug)]
struct Mos3Oracle {
    id: f64,
    gm: f64,
    gds: f64,
    gmb: f64,
    von: f64,
    vdsat: f64,
}

// Source: ngspice 46 (`Spice64/bin/ngspice_con.exe -b`) with
// `set numdgt=15`, `op`, and `wrdata ... @m1[cd] @m1[gm] @m1[gds]
// @m1[gmbs] @m1[von] @m1[vdsat]` on the decks below.
const NMOS_ORACLE: Mos3Oracle = Mos3Oracle {
    id: 5.998_923_772_554_548e-4,
    gm: 3.338_795_280_444_773e-4,
    gds: 2.774_136_992_435_784e-5,
    gmb: 8.255_266_123_075_666e-5,
    von: 7.589_955_991_238_436e-1,
    vdsat: 1.021_205_861_719_642,
};

const PMOS_ORACLE: Mos3Oracle = Mos3Oracle {
    id: 2.425_515_462_863_035e-4,
    gm: 2.238_898_944_121_671e-4,
    gds: 5.790_721_317_550_307e-6,
    gmb: 5.479_340_588_755_883e-5,
    von: -8.859_415_882_320_186e-1,
    vdsat: -8.895_065_062_528_498e-1,
};

const INVERSE_ORACLE: Mos3Oracle = Mos3Oracle {
    id: -3.769_762_713_788_676e-4,
    gm: 2.713_347_600_497_562e-4,
    gds: 3.513_830_768_749_185e-5,
    gmb: 6.209_544_186_333_239e-5,
    von: 5.823_279_980_055_280e-1,
    vdsat: 7.111_305_398_000_489e-1,
};

const SHORT_VMAX_ORACLE: Mos3Oracle = Mos3Oracle {
    id: 7.725_876_408_583_034e-4,
    gm: 2.402_917_688_369_275e-4,
    gds: 1.597_228_290_845_190e-4,
    gmb: 6.283_097_653_109_673e-5,
    von: -6.068_900_683_665_898e-1,
    vdsat: 5.678_467_038_549_644e-1,
};

const SHORT_NO_VMAX_ORACLE: Mos3Oracle = Mos3Oracle {
    id: 3.856_903_433_455_451e-3,
    gm: 1.709_681_471_589_484e-3,
    gds: 1.787_720_666_681_520e-3,
    gmb: 3.443_365_047_746_089e-4,
    von: -6.068_900_683_665_898e-1,
    vdsat: 2.850_196_216_579_879,
};

const SHORT_NO_VMAX_ALPHA_ORACLE: Mos3Oracle = Mos3Oracle {
    id: 4.610_419_090_077_125e-3,
    gm: 1.241_393_616_561_123e-3,
    gds: 2.395_189_143_014_649e-3,
    gmb: 1.291_805_120_948_991e-4,
    von: -8.160_847_184_245_068e-1,
    vdsat: 3.163_758_574_237_703,
};

const WEAK_SAT_ALPHA_ORACLE: Mos3Oracle = Mos3Oracle {
    id: 4.328_493_660_338_096e-7,
    gm: 9.367_426_614_041_116e-6,
    gds: 6.387_371_464_232_800e-6,
    gmb: 1.267_349_992_252_157e-6,
    von: -8.160_847_184_245_068e-1,
    vdsat: 4.182_071_076_322_277e-2,
};

const WEAK_ZERO_VDS_ORACLE: Mos3Oracle = Mos3Oracle {
    id: 4.099_963_655_925_660e-13,
    gm: 0.0,
    gds: 1.038_718_005_194_965e-19,
    gmb: 0.0,
    von: 6.462_634_331_029_411e-1,
    vdsat: 4.182_071_076_322_277e-2,
};

const TEMP_85C_ORACLE: Mos3Oracle = Mos3Oracle {
    id: 5.541_628_539_284_347e-4,
    gm: 3.121_931_640_043_636e-4,
    gds: 2.593_949_410_802_763e-5,
    gmb: 7.818_157_419_068_965e-5,
    von: 6.866_171_421_819_509e-1,
    vdsat: 1.200_143_774_482_931,
};

const NMOS_DECK: &str = r#"
* mos3_nmos_op
M1 d g s b MOD W=12U L=1.2U
VDS d 0 2.5
VGS g 0 3.0
VBS b 0 -0.6
VS  s 0 0
.MODEL MOD NMOS LEVEL=3 VTO=0.72 KP=55U GAMMA=0.62 PHI=0.68
+ TOX=22N LD=0.08U ETA=0.18 THETA=0.05 KAPPA=0.35
+ NFS=8E11 VMAX=8E4 XJ=0.18U DELTA=0.22
.OP
.END
"#;

const TEMP_85C_DECK: &str = r#"
* mos3_temp_op
.options reltol=1e-7 temp=85
M1 d g s b MOD W=12U L=1.2U
VD d 0 2.5
VG g 0 3.0
VS s 0 0
VB b 0 -0.6
.MODEL MOD NMOS LEVEL=3 VTO=0.72 KP=55U GAMMA=0.62 PHI=0.68 TNOM=27
+ TOX=22N LD=0.08U ETA=0.18 THETA=0.05 KAPPA=0.35
+ NFS=8E11 VMAX=8E4 XJ=0.18U DELTA=0.22
.OP
.END
"#;

const SERIES_MULTIPLIER_DECK: &str = r#"
* mos3_series_m_op
.options reltol=1e-7
VDD supply 0 3.3
VMON supply d 0
VG g 0 2.4
VS s 0 0
VB b 0 -0.4
M1 d g s b MOD W=10U L=1.0U M=2.5
.MODEL MOD NMOS LEVEL=3 VTO=0.66 KP=80U GAMMA=0.55 PHI=0.67
+ TOX=20N LD=0.05U ETA=0.16 THETA=0.06 KAPPA=0.38
+ NFS=6E11 VMAX=7E4 XJ=0.16U DELTA=0.21 RD=18 RS=24
.OP
.END
"#;

const SERIES_MULTIPLIER_NO_RS_DECK: &str = r#"
* mos3_series_m_nors_op
.options reltol=1e-7
VDD supply 0 3.3
VMON supply d 0
VG g 0 2.4
VS s 0 0
VB b 0 -0.4
M1 d g s b MOD W=10U L=1.0U M=2.5
.MODEL MOD NMOS LEVEL=3 VTO=0.66 KP=80U GAMMA=0.55 PHI=0.67
+ TOX=20N LD=0.05U ETA=0.16 THETA=0.06 KAPPA=0.38
+ NFS=6E11 VMAX=7E4 XJ=0.16U DELTA=0.21 RD=0 RS=0
.OP
.END
"#;

const BODY_JUNCTION_DECK: &str = r#"
* mos3_body_junction_op
.options reltol=1e-9 abstol=1e-15
VD d 0 0
VG g 0 0
VS s 0 0
VB b 0 0.62
M1 d g s b MOD W=8U L=1U AD=12P AS=10P PD=18U PS=16U
.MODEL MOD NMOS LEVEL=3 VTO=0.72 KP=55U GAMMA=0.62 PHI=0.68
+ TOX=22N LD=0.08U ETA=0.18 THETA=0.05 KAPPA=0.35
+ NFS=0 VMAX=0 XJ=0.18U DELTA=0.22 IS=1E-14 JS=2E-8
+ PB=0.8 CJ=2E-4 CJSW=1E-10
.OP
.END
"#;

const PMOS_DECK: &str = r#"
* mos3_pmos_op
M1 d g s b MOD W=18U L=1.5U
VSD s 0 3.0
VSG g 0 0.6
VSB b 0 3.3
VD  d 0 0.2
.MODEL MOD PMOS LEVEL=3 VTO=-0.82 KP=32U GAMMA=0.55 PHI=0.7
+ TOX=24N LD=0.06U ETA=0.12 THETA=0.04 KAPPA=0.28
+ NFS=5E11 VMAX=7E4 XJ=0.2U DELTA=0.18
.OP
.END
"#;

const PMOS_LOADED_DECK: &str = r#"
* mos3_pmos_loaded_dc
.option reltol=1e-6
VDD vdd 0 3.3
VG g 0 1.0
RD out 0 12k
M1 out g vdd vdd PMOD W=18U L=1.5U
.MODEL PMOD PMOS LEVEL=3 VTO=-0.82 KP=32U GAMMA=0.55 PHI=0.7
+ TOX=24N LD=0.06U ETA=0.12 THETA=0.04 KAPPA=0.28
+ NFS=5E11 VMAX=7E4 XJ=0.2U DELTA=0.18
.OP
.END
"#;

const INVERSE_DECK: &str = r#"
* mos3_inverse_mode
M1 d g s b MOD W=10U L=1.0U
VD d 0 0.15
VG g 0 2.4
VS s 0 1.8
VB b 0 -0.2
.MODEL MOD NMOS LEVEL=3 VTO=0.65 KP=70U GAMMA=0.5 PHI=0.65
+ TOX=20N LD=0.05U ETA=0.2 THETA=0.08 KAPPA=0.4
+ NFS=6E11 VMAX=6E4 XJ=0.15U DELTA=0.25
.OP
.END
"#;

const SHORT_VMAX_DECK: &str = r#"
* mos3_short_channel_vmax
M1 d g s b MOD W=8U L=0.6U
VD d 0 2.2
VG g 0 2.6
VS s 0 0
VB b 0 -0.4
.MODEL MOD NMOS LEVEL=3 VTO=0.68 KP=70U GAMMA=0.58 PHI=0.68
+ TOX=18N LD=0.04U ETA=0.22 THETA=0.07 KAPPA=0.42
+ NFS=7E11 VMAX=6E4 XJ=0.12U DELTA=0.24
.OP
.END
"#;

const SHORT_NO_VMAX_DECK: &str = r#"
* mos3_short_channel_no_vmax
M1 d g s b MOD W=8U L=0.6U
VD d 0 2.2
VG g 0 2.6
VS s 0 0
VB b 0 -0.4
.MODEL MOD NMOS LEVEL=3 VTO=0.68 KP=70U GAMMA=0.58 PHI=0.68
+ TOX=18N LD=0.04U ETA=0.22 THETA=0.07 KAPPA=0.42
+ NFS=7E11 VMAX=0 XJ=0.12U DELTA=0.24
.OP
.END
"#;

const SHORT_NO_VMAX_ALPHA_DECK: &str = r#"
* mos3_short_channel_no_vmax_alpha
M1 d g s b MOD W=8U L=0.6U
VD d 0 2.2
VG g 0 2.6
VS s 0 0
VB b 0 -0.4
.MODEL MOD NMOS LEVEL=3 VTO=0.68 KP=70U GAMMA=0.58 PHI=0.68
+ TOX=18N NSUB=1E16 LD=0.04U ETA=0.22 THETA=0.07 KAPPA=0.42
+ NFS=7E11 VMAX=0 XJ=0.12U DELTA=0.24
.OP
.END
"#;

const WEAK_SAT_ALPHA_DECK: &str = r#"
* mos3_weak_sat_alpha
M1 d g s b MOD W=8U L=0.6U
VD d 0 2.2
VG g 0 -0.9
VS s 0 0
VB b 0 -0.4
.MODEL MOD NMOS LEVEL=3 VTO=0.68 KP=70U GAMMA=0.58 PHI=0.68
+ TOX=18N NSUB=1E16 LD=0.04U ETA=0.22 THETA=0.07 KAPPA=0.42
+ NFS=7E11 VMAX=0 XJ=0.12U DELTA=0.24
.OP
.END
"#;

const WEAK_ZERO_VDS_DECK: &str = r#"
* mos3_weak_zero_vds
M1 d g s b MOD W=8U L=0.6U
VD d 0 0
VG g 0 -0.9
VS s 0 0
VB b 0 -0.4
.MODEL MOD NMOS LEVEL=3 VTO=0.68 KP=70U GAMMA=0.58 PHI=0.68
+ TOX=18N NSUB=1E16 LD=0.04U ETA=0.22 THETA=0.07 KAPPA=0.42
+ NFS=7E11 VMAX=0 XJ=0.12U DELTA=0.24
.OP
.END
"#;

const AC_WIDTH_DECK: &str = r#"
* mos3_ac_width_sensitive_common_source
.option reltol=1e-6
VDD vdd 0 3.3
VIN g 0 dc 1.8 ac 1
RD vdd out 8k
M1 out g 0 0 MOD W=10U L=1.0U
.MODEL MOD NMOS LEVEL=3 VTO=0.68 KP=70U GAMMA=0.58 PHI=0.68
+ TOX=18N NSUB=1E16 LD=0.04U ETA=0.22 THETA=0.07 KAPPA=0.42
+ NFS=7E11 VMAX=0 XJ=0.12U DELTA=0.24 WD=2U XW=1U
+ CGSO=3E-10 CGDO=4E-10 CGBO=1E-10
.AC LIN 3 1E6 1E10
.END
"#;

const TRAN_WIDTH_DECK: &str = r#"
* mos3_transient_width_sensitive_common_source
.option reltol=1e-6
VDD vdd 0 3.3
VIN g 0 PULSE(0 1.8 0.2N 0.1N 0.1N 1.0N 2.0N)
RD vdd out 8k
M1 out g 0 0 MOD W=10U L=1.0U
.MODEL MOD NMOS LEVEL=3 VTO=0.68 KP=70U GAMMA=0.58 PHI=0.68
+ TOX=18N NSUB=1E16 LD=0.04U ETA=0.22 THETA=0.07 KAPPA=0.42
+ NFS=7E11 VMAX=0 XJ=0.12U DELTA=0.24 WD=2U XW=1U
+ CGSO=3E-10 CGDO=4E-10 CGBO=1E-10
.TRAN 0.02N 4N
.END
"#;

const XYCE_NMOS3_DC_DECK: &str = r#"
* Xyce 7.10 NMOS3_DC/mos3_dc.cir
VDS 4 0 0V
VGS 1 0 0V
VMON 4 3 0V
M1 3 1 0 0 NFET L=2.0U W=2.0U
.MODEL NFET NMOS
+ LEVEL=3 UO=966.5 L=2.0U W=2.0U VTO=1.043
+ NFS=1.009E+11 TOX=1E-07 NSUB=1.379E+16 VMAX=4.096E+05
+ RSH=0 RS=0 RD=0 IS=1E-14
+ XJ=5.378E-06 LD=0 DELTA=0 NSS=1E10
+ THETA=0.0582 ETA=0.095 KAPPA=2.93 CGDO=1PF
+ CGSO=1PF CGBO=1PF CBD=1PF CBS=1PF
.DC VDS 0 6 0.01 VGS 1 4 1
.PRINT DC V(4) V(1) I(VMON)
.END
"#;

const XYCE_PMOS3_POINT_DECK: &str = r#"
* Xyce 7.10 MOS3 PMOS polarity point
VSD s 0 3.0
VSG g 0 0.6
VSB b 0 3.3
VD0 d0 0 0.2
VMON d0 d 0
M1 d g s b MOD W=18U L=1.5U
.MODEL MOD PMOS LEVEL=3 VTO=-0.82 KP=32U GAMMA=0.55 PHI=0.7
+ TOX=24N LD=0.06U ETA=0.12 THETA=0.04 KAPPA=0.28
+ NFS=5E11 VMAX=7E4 XJ=0.2U DELTA=0.18
.DC VD0 0.2 0.2 1
.PRINT DC I(VMON) V(d) V(s) V(g) V(b)
.END
"#;

const XYCE_NMOS3_INVERSE_POINT_DECK: &str = r#"
* Xyce 7.10 MOS3 inverse-mode point
VD0 d0 0 0.15
VMON d0 d 0
VG g 0 2.4
VS s 0 1.8
VB b 0 -0.2
M1 d g s b MOD W=10U L=1.0U
.MODEL MOD NMOS LEVEL=3 VTO=0.65 KP=70U GAMMA=0.5 PHI=0.65
+ TOX=20N LD=0.05U ETA=0.2 THETA=0.08 KAPPA=0.4
+ NFS=6E11 VMAX=6E4 XJ=0.15U DELTA=0.25
.DC VD0 0.15 0.15 1
.PRINT DC I(VMON) V(d) V(s) V(g) V(b)
.END
"#;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn assert_close(what: &str, actual: f64, expected: f64, rel: f64, abs: f64) {
    let diff = (actual - expected).abs();
    let tol = abs.max(rel * expected.abs().max(actual.abs()));
    assert!(
        diff <= tol,
        "{what}: actual={actual:.12e} expected={expected:.12e} diff={diff:.12e} tol={tol:.12e}"
    );
}

fn m1_op_entry(deck: &str) -> DeviceOpEntry {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("op converges");
    report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .unwrap_or_else(|| panic!("missing m1 OP entry in {:?}", report.entries))
        .clone()
}

fn op_param(entry: &DeviceOpEntry, key: &str) -> f64 {
    entry
        .params
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, value)| *value)
        .unwrap_or_else(|| panic!("{key} missing from MOS3 OP entry: {:?}", entry.params))
}

fn assert_op_matches_ngspice46(entry: &DeviceOpEntry, oracle: Mos3Oracle) {
    assert_close("id", op_param(entry, "id"), oracle.id, 2.0e-3, 1.0e-10);
    assert_close("gm", op_param(entry, "gm"), oracle.gm, 2.0e-3, 1.0e-10);
    assert_close("gds", op_param(entry, "gds"), oracle.gds, 2.0e-3, 1.0e-10);
    assert_close("gmb", op_param(entry, "gmb"), oracle.gmb, 2.0e-3, 1.0e-10);
    assert_close(
        "vth/von",
        op_param(entry, "vth"),
        oracle.von,
        2.0e-3,
        1.0e-8,
    );
    assert_close(
        "vdsat",
        op_param(entry, "vdsat"),
        oracle.vdsat,
        2.0e-3,
        1.0e-8,
    );
}

fn dc_node_voltage(deck: &str, node_name: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = engine().run_dc_op(&netlist).expect("op converges");
    let node = result
        .node_index_named(node_name)
        .unwrap_or_else(|| panic!("missing node {node_name}"));
    result.voltage(node)
}

fn dc_branch_current(deck: &str, branch_name: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = engine().run_dc_op(&netlist).expect("op converges");
    let branch = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(branch_name))
        .unwrap_or_else(|| panic!("missing {branch_name} branch in {:?}", result.branch_names));
    result.branch_currents[branch]
}

fn dc_sweep_branch_current(
    deck: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
    branch_name: &str,
) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let mut results = engine()
        .run_dc_sweep(&netlist, source_name, start, stop, step)
        .expect("dc sweep converges");
    assert_eq!(results.len(), 1, "single-point sweep expected");
    let (_, result) = results.pop().expect("one dc point");
    let branch = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(branch_name))
        .unwrap_or_else(|| panic!("missing {branch_name} branch in {:?}", result.branch_names));
    result.branch_currents[branch]
}

fn ac_node_voltage(deck: &str, node_name: &str, freq: f64) -> num_complex::Complex64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = engine()
        .run_ac(&netlist, &[freq])
        .expect("ac converges")
        .pop()
        .expect("one AC result");
    let node = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node_name))
        .unwrap_or_else(|| panic!("missing node {node_name} in {:?}", result.node_names));
    result.voltages[node]
}

fn transient_node_series<'a>(
    names: &[String],
    voltages: &'a [Vec<f64>],
    node_name: &str,
) -> &'a [f64] {
    let node = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node_name))
        .unwrap_or_else(|| panic!("missing node {node_name} in {:?}", names));
    &voltages[node]
}

fn interpolate(time: &[f64], values: &[f64], target: f64) -> f64 {
    assert_eq!(time.len(), values.len(), "time and value vectors align");
    if target <= time[0] {
        return values[0];
    }
    for index in 1..time.len() {
        if time[index] >= target {
            let t0 = time[index - 1];
            let t1 = time[index];
            let y0 = values[index - 1];
            let y1 = values[index];
            let frac = if t1 == t0 {
                0.0
            } else {
                (target - t0) / (t1 - t0)
            };
            return y0 + frac * (y1 - y0);
        }
    }
    *values.last().expect("non-empty value vector")
}

#[test]
fn mos3_nmos_op_matches_ngspice46() {
    let entry = m1_op_entry(NMOS_DECK);
    assert_op_matches_ngspice46(&entry, NMOS_ORACLE);
}

#[test]
fn mos3_pmos_op_matches_ngspice46() {
    let entry = m1_op_entry(PMOS_DECK);
    assert_op_matches_ngspice46(&entry, PMOS_ORACLE);
}

#[test]
fn mos3_pmos_loaded_dc_matches_ngspice46() {
    // ngspice-46 reference on the same deck, `.option reltol=1e-6`:
    // v(out)=2.58003651 V. This circuit-level check catches PMOS channel
    // sign mistakes that a standalone `@m1[cd]` OP comparison can hide.
    let vout = dc_node_voltage(PMOS_LOADED_DECK, "out");
    assert_close("loaded pmos v(out)", vout, 2.580_036_51, 2.0e-3, 1.0e-5);
}

#[test]
fn mos3_temperature_matches_ngspice46() {
    let entry = m1_op_entry(TEMP_85C_DECK);
    assert_op_matches_ngspice46(&entry, TEMP_85C_ORACLE);
}

#[test]
fn mos3_series_resistance_and_multiplier_match_ngspice46() {
    // ngspice-46 references on the same deck:
    // with RD=18 RS=24 and M=2.5, i(vmon)=1.412774112778570e-03.
    // With RD=RS=0, the same biased deck gives 1.431951424595751e-03,
    // so this rejects a MOS3 path that silently bypasses series resistance.
    let id = dc_branch_current(SERIES_MULTIPLIER_DECK, "vmon");
    assert_close(
        "series-resistance M-scaled i(vmon)",
        id,
        1.412_774_112_778_570e-3,
        3.0e-3,
        2.0e-7,
    );

    let no_series_id = dc_branch_current(SERIES_MULTIPLIER_NO_RS_DECK, "vmon");
    assert!(
        (no_series_id - id).abs() > 0.01 * no_series_id.abs(),
        "MOS3 RD/RS must materially affect current: RD/RS id={id:.12e}, \
         no-RD/RS id={no_series_id:.12e}"
    );
}

#[test]
fn mos3_body_junction_current_matches_ngspice46() {
    // ngspice-46 reference on the same deck:
    // i(vb)=-1.13198377223539e-08 at v(b)=0.62 V.
    let ib = dc_branch_current(BODY_JUNCTION_DECK, "vb");
    assert_close(
        "body-junction i(vb)",
        ib,
        -1.131_983_772_235_39e-8,
        5.0e-2,
        2.0e-10,
    );
}

#[test]
fn mos3_inverse_mode_matches_ngspice46() {
    let entry = m1_op_entry(INVERSE_DECK);
    assert_op_matches_ngspice46(&entry, INVERSE_ORACLE);
}

#[test]
fn mos3_short_channel_vmax_changes_current() {
    let with_vmax = m1_op_entry(SHORT_VMAX_DECK);
    let without_vmax = m1_op_entry(SHORT_NO_VMAX_DECK);

    assert_op_matches_ngspice46(&with_vmax, SHORT_VMAX_ORACLE);
    assert_op_matches_ngspice46(&without_vmax, SHORT_NO_VMAX_ORACLE);

    let with_id = op_param(&with_vmax, "id");
    let without_id = op_param(&without_vmax, "id");
    assert!(
        (with_id - without_id).abs() > 0.25 * without_id.abs(),
        "MOS3 VMAX must materially change short-channel current: \
         VMAX id={with_id:.12e}, VMAX=0 id={without_id:.12e}"
    );
}

#[test]
fn mos3_short_channel_no_vmax_alpha_derivatives_match_ngspice46() {
    let entry = m1_op_entry(SHORT_NO_VMAX_ALPHA_DECK);
    assert_op_matches_ngspice46(&entry, SHORT_NO_VMAX_ALPHA_ORACLE);
}

#[test]
fn mos3_weak_inversion_alpha_clm_matches_ngspice46() {
    let entry = m1_op_entry(WEAK_SAT_ALPHA_DECK);
    assert_op_matches_ngspice46(&entry, WEAK_SAT_ALPHA_ORACLE);
}

#[test]
fn mos3_weak_zero_vds_conductance_matches_ngspice46() {
    let entry = m1_op_entry(WEAK_ZERO_VDS_DECK);
    assert_op_matches_ngspice46(&entry, WEAK_ZERO_VDS_ORACLE);
}

#[test]
fn mos3_ac_width_and_overlap_caps_match_ngspice46() {
    // ngspice-46 references on the same deck, `.ac lin 3 1e6 1e10`:
    // f=1e6   v(out)=(-7.86376304e-1, 1.75245693e-4)
    // f=5.0005e9 v(out)=(-4.39877851e-1, 7.06339489e-1)
    // f=1e10  v(out)=( 8.96911684e-2, 8.93024061e-1)
    let reference = [
        (1.0e6, -7.863_763_04e-1, 1.752_456_93e-4),
        (5.0005e9, -4.398_778_51e-1, 7.063_394_89e-1),
        (1.0e10, 8.969_116_84e-2, 8.930_240_61e-1),
    ];

    for (freq, re_ref, im_ref) in reference {
        let vout = ac_node_voltage(AC_WIDTH_DECK, "out", freq);
        assert_close(
            &format!("AC re v(out) at {freq:.4e}"),
            vout.re,
            re_ref,
            2.0e-3,
            2.0e-4,
        );
        assert_close(
            &format!("AC im v(out) at {freq:.4e}"),
            vout.im,
            im_ref,
            2.0e-3,
            2.0e-4,
        );
    }
}

#[test]
fn mos3_transient_width_caps_match_ngspice46() {
    let netlist = Netlist::parse(TRAN_WIDTH_DECK).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 4.0e-9, 0.02e-9)
        .expect("transient runs");
    let out = transient_node_series(&result.node_names, &result.voltages, "out");

    // ngspice-46 reference on the same deck, `tran 0.02n 4n 0 0.02n`.
    let reference = [
        (0.0, 3.299_956_920, 1.0e-2),
        (0.2e-9, 3.299_956_920, 1.0e-2),
        (0.3e-9, 1.148_526_200, 5.0e-2),
        (0.5e-9, 7.463_281_710e-1, 2.0e-2),
        (1.3e-9, 7.463_277_450e-1, 1.0e-2),
        (2.3e-9, 1.150_394_570, 5.0e-2),
        (2.6e-9, 7.463_277_450e-1, 2.0e-2),
        (3.3e-9, 7.463_277_450e-1, 1.0e-2),
        (4.0e-9, 3.299_956_920, 1.0e-2),
    ];

    for (time, v_ref, abs_tol) in reference {
        let vout = interpolate(&result.time, out, time);
        assert_close(
            &format!("transient v(out) at {time:.3e}s"),
            vout,
            v_ref,
            2.0e-2,
            abs_tol,
        );
    }
}

#[test]
fn mos3_model_space_onset_and_saturation_are_not_simplified_fallbacks() {
    let entry = m1_op_entry(NMOS_DECK);

    // `vth` is the only generic MOSFET OP-report value currently exposing the
    // model-space onset used by limiting and capacitance paths.
    assert_close(
        "vth/von",
        op_param(&entry, "vth"),
        NMOS_ORACLE.von,
        2.0e-3,
        1.0e-8,
    );

    // Native MOS3 also needs to surface `vdsat` for AC/transient-facing oracle
    // coverage. The current generic MOSFET report omits it, so this is the
    // strongest RED assertion available through the public OP report today.
    let vdsat = op_param(&entry, "vdsat");
    assert_close("vdsat", vdsat, NMOS_ORACLE.vdsat, 2.0e-3, 1.0e-8);

    let old_fallback_vdsat = 3.0 - NMOS_ORACLE.von;
    assert!(
        (NMOS_ORACLE.vdsat - old_fallback_vdsat).abs() > 0.5,
        "oracle sanity check: MOS3 vdsat should differ from old Vgs-von fallback"
    );
}

#[test]
fn mos3_matches_xyce_710_nmos3_dc_subset() {
    let netlist = Netlist::parse(XYCE_NMOS3_DC_DECK).expect("deck parses");
    let sweep2 = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc { sweep2, .. } => sweep2.clone(),
            _ => None,
        })
        .expect("second sweep captured");
    let results = engine()
        .run_dc_sweep2_with_abort(
            &netlist,
            "vds",
            0.0,
            6.0,
            0.01,
            Some(&sweep2),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("Xyce NMOS3 sweep solves");
    let vmon_branch = results[0]
        .1
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("vmon"))
        .unwrap_or_else(|| panic!("missing vmon branch in {:?}", results[0].1.branch_names));

    // Xyce 7.10 regression `NMOS3_DC/mos3_dc.cir.prn`, representative rows.
    let reference = [
        (0, -1.960_784_31e-38),
        (100, 2.110_557_86e-6),
        (600, 3.710_058_04e-6),
        (651, 1.559_137_79e-5),
        (801, 2.112_993_32e-5),
        (1302, 4.570_722_08e-5),
        (1802, 6.137_162_32e-5),
        (1823, 1.786_673_43e-5),
        (2103, 9.814_379_35e-5),
        (2403, 1.076_422_85e-4),
    ];

    assert_eq!(results.len(), 2404, "Xyce NMOS3 sweep grid size");
    for &(idx, expected) in &reference {
        let got = results[idx].1.branch_currents[vmon_branch];
        assert_close(
            &format!("Xyce NMOS3 row {idx} I(VMON)"),
            got,
            expected,
            2.0e-2,
            5.0e-8,
        );
    }
}

#[test]
fn mos3_matches_xyce_710_pmos_and_inverse_dc_points() {
    // Live XyceNF 7.10.0 references from the exact decks above:
    // PMOS point:    I(VMON)=-2.42551547e-04.
    // inverse point: I(VMON)=-3.76976271e-04.
    let pmos_current = dc_sweep_branch_current(XYCE_PMOS3_POINT_DECK, "vd0", 0.2, 0.2, 1.0, "vmon");
    assert_close(
        "Xyce PMOS3 point I(VMON)",
        pmos_current,
        -2.425_515_47e-4,
        2.0e-2,
        5.0e-8,
    );

    let inverse_current = dc_sweep_branch_current(
        XYCE_NMOS3_INVERSE_POINT_DECK,
        "vd0",
        0.15,
        0.15,
        1.0,
        "vmon",
    );
    assert_close(
        "Xyce inverse NMOS3 point I(VMON)",
        inverse_current,
        -3.769_762_71e-4,
        2.0e-2,
        5.0e-8,
    );
}
