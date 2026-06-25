//! Native VDMOS / Xyce MOS LEVEL=18 integration tests.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, Netlist};

fn run_report(deck: &str) -> rspice_core::circuit::DeviceOpReport {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    engine
        .run_dc_op_with_report(&netlist)
        .map(|(_, report)| report)
        .expect("deck runs")
}

fn ac_voltage_named(
    result: &rspice_core::analysis::AcResult,
    name: &str,
) -> rspice_core::Complex64 {
    let index = result
        .node_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("node {name} exists in AC result"));
    result.voltages[index]
}

fn ac_branch_current_named(
    result: &rspice_core::analysis::AcResult,
    name: &str,
) -> rspice_core::Complex64 {
    let index = result
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("branch {name} exists in AC result"));
    result.currents[index]
}

fn transient_node_series<'a>(names: &[String], voltages: &'a [Vec<f64>], want: &str) -> &'a [f64] {
    let idx = names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("missing node {want} in {names:?}"));
    &voltages[idx]
}

fn interpolate(time: &[f64], values: &[f64], t: f64) -> f64 {
    let idx = time.partition_point(|candidate| *candidate < t);
    if idx == 0 {
        return values[0];
    }
    if idx >= time.len() {
        return *values.last().unwrap();
    }
    let (t0, t1) = (time[idx - 1], time[idx]);
    let (v0, v1) = (values[idx - 1], values[idx]);
    if t1 == t0 {
        v0
    } else {
        v0 + (v1 - v0) * (t - t0) / (t1 - t0)
    }
}

fn xyce_irf130_dc_deck() -> &'static str {
    "\
IRF130 Test Circuit
VD 3 0 0
VS 2 0 0
VG 4 0 DC 5
VID 3 5 DC 0
M1 5 4 2 0 IRF130 W=0.386 L=2.5u
.MODEL IRF130 NMOS LEVEL=18
+ CV=1
+ CVE=1
+ VTO=3.5
+ RD=0
+ RS=0.005
+ LAMBDA=0
+ M=3
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=50nm
*
.DC VD 0 50 5 VG 5 15 5
.PRINT DC V(3) V(4) V(2) I(VID)
.END
"
}

fn xyce_irhc110_dc_deck() -> &'static str {
    "\
IRHC110 Test Circuit
VD 1 0 DC 0
VS 4 0 0
VG 3 0 DC 0
VID 1 2 DC 0
M1 2 3 4 0 IRHC110 W=0.25 L=3u
.MODEL IRHC110 NMOS LEVEL=18
+ CV=1
+ CVE=1
+ VTO=4.0
+ RD=0.12
+ RS=0.055
+ LAMBDA=0.0
+ KAPPA=1e-3
+ NFS=3e11
+ M=3
+ SIGMA0=0
+ UO=700
+ TOX=100nm
+ NSUB=4.6e16
+ PHI=0.6
+ CBSO=2.65e-11
*
.DC VD 0 10 1 VG 4 8 1
.PRINT DC V(1) V(3) V(2) I(VID)
.END
"
}

fn xyce_irf130_tran_deck() -> &'static str {
    "\
IRF130 Test Circuit
VD 3 1 0.5
VS 2 0 0
VG 4 0 10 pulse(0 10 300ns 50ns 50ns 400ns 1000ns)
VID 0 1 DC 0
M1 3 4 2 0 IRF130 W=0.386 L=2.5u
.MODEL IRF130 NMOS LEVEL=18
+ CV=1
+ CVE=1
+ VTO=3.5
+ RD= 0
+ RS= 0.005
+ LAMBDA=0
+ M=3
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=50nm
*
.TRAN 0.5n 1u 0u 2n
.PRINT TRAN precision=10 width=19 V(3) V(4) {I(VID)+0.5}
.options timeint reltol=1.0e-2 abstol=1.0e-7
.END
"
}

fn xyce_vdmos_cgdo_displacement_deck() -> &'static str {
    "\
VDMOS Cgd displacement oracle
VD d 0 0.5
VG g 0 pulse(0 10 10n 10n 10n 40n 100n)
VS s 0 0
M1 d g s 0 OFF W=1 L=1u
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGDO=1e-11
+ CGSO=0
+ CGBO=0
+ CBD=0
+ CBS=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=50nm
.TRAN 0.1n 15.1n 0 0.1n
.PRINT TRAN precision=12 width=21 V(g) I(VD)
.options timeint reltol=1.0e-5 abstol=1.0e-10
.END
    "
}

fn xyce_vdmos_cgbo_displacement_deck() -> &'static str {
    "\
VDMOS Cgb displacement oracle
VD d 0 0
VG g 0 sin(0 1 1MEG 0 0 0)
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=1e-5
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.TRAN 1n 0.5u 0 1n
.PRINT TRAN precision=12 width=21 V(g) I(VG) I(VB)
.options timeint reltol=1.0e-5 abstol=1.0e-10
.END
"
}

fn xyce_vdmos_cbs_displacement_deck() -> &'static str {
    "\
VDMOS Cbs displacement oracle
VD d 0 0
VG g 0 0
VS s 0 sin(0 1m 1MEG 0 0 0)
VB b 0 0
M1 d g s b OFF W=1 L=1u
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=1e-7
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.TRAN 1n 0.5u 0 1n
.PRINT TRAN precision=12 width=21 V(s) I(VS) I(VB)
.options timeint reltol=1.0e-5 abstol=1.0e-10
.END
"
}

fn xyce_vdmos_ac_charge_branch_deck(
    title: &str,
    vd: &str,
    vg: &str,
    vs: &str,
    vb: &str,
    charge_params: &str,
    print: &str,
) -> String {
    format!(
        "\
VDMOS {title} AC oracle
{vd}
{vg}
{vs}
{vb}
M1 d g s b OFF W=1 L=1u
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
{charge_params}
+ IS=1e-30
+ JS=0
+ D1IS=1e-30
+ D1ISR=0
+ D1RS=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.AC LIN 1 1MEG 1MEG
.PRINT AC {print}
.END
"
    )
}

fn xyce_vdmos_cbs_ac_deck() -> String {
    xyce_vdmos_ac_charge_branch_deck(
        "Cbs",
        "VD d 0 0",
        "VG g 0 0",
        "VS s 0 0",
        "VB b 0 dc 0 ac 1",
        "+ CGBO=0\n+ CGDO=0\n+ CGSO=0\n+ CBD=0\n+ CBS=1e-7\n+ CJ=0\n+ CJSW=0\n+ D1CJO=0",
        "VR(b) VI(b) IR(VB) II(VB) IM(VB) IP(VB)",
    )
}

fn xyce_vdmos_cj_as_displacement_deck() -> &'static str {
    "\
VDMOS CJ/CJSW displacement oracle
VD d 0 0
VG g 0 0
VS s 0 sin(0 1m 1MEG 0 0 0)
VB b 0 0
M1 d g s b OFF W=1 L=1u AS=2e-6 AD=0 PS=1e-6 PD=0
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CJ=0.04
+ CJSW=0.02
+ MJ=0.5
+ MJSW=0.5
+ PB=0.8
+ IS=1e-30
+ JS=0
+ D1IS=1e-30
+ D1ISR=0
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.TRAN 1n 0.5u 0 1n
.PRINT TRAN precision=12 width=21 V(s) I(VS) I(VB)
.options timeint reltol=1.0e-5 abstol=1.0e-10
.END
"
}

fn xyce_vdmos_d1cjo_displacement_deck() -> &'static str {
    "\
VDMOS D1CJO displacement oracle
VD d 0 sin(0 1m 1MEG 0 0 0)
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-30
+ D1ISR=0
+ D1RS=0
+ D1CJO=1e-7
+ D1VJ=1
+ D1M=0.5
+ D1FC=0.5
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.TRAN 1n 0.5u 0 1n
.PRINT TRAN precision=12 width=21 V(d) I(VD) I(VS)
.options timeint reltol=1.0e-5 abstol=1.0e-10
.END
"
}

fn xyce_vdmos_d1cjo_temp_displacement_deck() -> &'static str {
    "\
VDMOS D1CJO temperature displacement oracle
VD d 0 sin(0 1m 1MEG 0 0 0)
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-30
+ D1ISR=0
+ D1RS=0
+ D1CJO=1e-7
+ D1VJ=0.6
+ D1M=0.5
+ D1FC=0.5
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.OPTIONS TEMP=125
.TRAN 1n 0.5u 0 1n
.PRINT TRAN precision=12 width=21 V(d) I(VD) I(VS)
.options timeint reltol=1.0e-5 abstol=1.0e-10
.END
"
}

fn xyce_vdmos_d1is_dc_deck() -> &'static str {
    "\
VDMOS D1IS DC oracle
VD d 0 DC -0.5
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-12
+ D1ISR=0
+ D1RS=0
+ D1N=1
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.OP
.PRINT DC precision=12 width=21 V(d) I(VD)
.END
"
}

fn xyce_vdmos_d1is_external_rd_dc_deck() -> &'static str {
    "\
VDMOS D1IS external-node DC oracle
VD d 0 DC -0.5
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=10
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-12
+ D1ISR=0
+ D1RS=0
+ D1N=1
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.OP
.PRINT DC precision=12 width=21 V(d) I(VD)
.END
"
}

fn xyce_vdmos_d1is_temp_dc_deck() -> &'static str {
    "\
VDMOS D1IS temperature DC oracle
VD d 0 DC -0.3
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-12
+ D1ISR=0
+ D1RS=0
+ D1N=1
+ D1CJO=0
+ D1TT=0
+ D1EG=1.11
+ D1XTI=3
+ D1TNOM=300.15
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.OPTIONS TEMP=75
.OP
.PRINT DC precision=12 width=21 V(d) I(VD)
.END
"
}

fn xyce_vdmos_d1is_temp_ignores_d1tnom_dc_deck() -> &'static str {
    "\
VDMOS D1IS D1TNOM ignored temperature DC oracle
VD d 0 DC -0.3
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-12
+ D1ISR=0
+ D1RS=0
+ D1N=1
+ D1CJO=0
+ D1TT=0
+ D1EG=1.11
+ D1XTI=3
+ D1TNOM=500
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.OPTIONS TEMP=75
.OP
.PRINT DC precision=12 width=21 V(d) I(VD)
.END
"
}

fn xyce_vdmos_d1rs_dc_deck() -> &'static str {
    "\
VDMOS D1RS DC oracle
VD d 0 DC -0.5
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-6
+ D1ISR=0
+ D1RS=100
+ D1N=1
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.OP
.PRINT DC precision=12 width=21 V(d) I(VD)
.END
"
}

fn xyce_vdmos_d1bv_dc_deck() -> &'static str {
    "\
VDMOS D1BV D1IBV DC oracle
VD d 0 DC 0
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-12
+ D1ISR=0
+ D1RS=0
+ D1N=1
+ D1CJO=0
+ D1TT=0
+ D1BV=5
+ D1IBV=1e-3
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.DC VD 4.8 5.2 0.1
.PRINT DC precision=12 width=21 V(d) I(VD)
.options reltol=1.0e-8 iabstol=1.0e-14 residual_reltol=1.0e-8
.END
"
}

fn xyce_vdmos_d1isr_dc_deck() -> &'static str {
    "\
VDMOS D1ISR D1NR DC oracle
VD d 0 DC -0.5
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=0
+ D1ISR=1e-9
+ D1NR=2
+ D1RS=0
+ D1N=1
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.DC VD -0.1 -0.5 -0.1
.PRINT DC precision=12 width=21 V(d) I(VD)
.options reltol=1.0e-8 iabstol=1.0e-14 residual_reltol=1.0e-8
.END
"
}

fn xyce_vdmos_d1isr_temp_vj_dc_deck() -> &'static str {
    "\
VDMOS D1ISR D1VJ temperature DC oracle
VD d 0 DC -0.5
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=0
+ D1ISR=1e-9
+ D1NR=2
+ D1RS=0
+ D1N=1
+ D1VJ=0.6
+ D1M=0.5
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.OPTIONS TEMP=125
.DC VD -0.2 -0.6 -0.1
.PRINT DC precision=12 width=21 V(d) I(VD)
.options reltol=1.0e-8 iabstol=1.0e-14 residual_reltol=1.0e-8
.END
"
}

fn xyce_vdmos_d1ikf_dc_deck() -> &'static str {
    "\
VDMOS D1IKF DC oracle
VD d 0 DC 0
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-12
+ D1ISR=0
+ D1RS=0
+ D1N=1
+ D1IKF=1e-5
+ D1CJO=0
+ D1TT=0
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.DC VD -0.3 -0.6 -0.1
.PRINT DC precision=12 width=21 V(d) I(VD)
.options reltol=1.0e-8 iabstol=1.0e-14 residual_reltol=1.0e-8
.END
"
}

fn xyce_vdmos_d1tt_displacement_deck() -> &'static str {
    "\
VDMOS D1TT diffusion-charge oracle
VD d 0 sin(0 1m 1MEG 0 0 0)
VG g 0 0
VS s 0 0
VB b 0 0
M1 d g s b OFF W=1 L=1u M=10
.MODEL OFF NMOS LEVEL=18
+ VTO=100
+ RD=0
+ RS=0
+ RG=0
+ CGBO=0
+ CGDO=0
+ CGSO=0
+ CBD=0
+ CBS=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
+ D1IS=1e-6
+ D1ISR=0
+ D1RS=0
+ D1N=1
+ D1CJO=0
+ D1TT=1e-6
+ CV=1
+ CVE=1
+ LAMBDA=0
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=1
.TRAN 1n 0.5u 0 1n
.PRINT TRAN precision=12 width=21 V(d) I(VD) I(VS)
.options timeint reltol=1.0e-5 abstol=1.0e-10
.END
"
}

fn xyce_mtb60p06v_dc_deck() -> &'static str {
    "\
MTB60P06V Test Circuit
VD 1 0 DC 0
VG 3 0 DC 0
VS 4 0 DC 0
VID 1 2 DC 0
*
M1 2 3 4 4 MAIN W=1.114 L=1.632u
.MODEL MAIN PMOS
+ LEVEL=18
+ CV=1
+ CVE=1
+ RD=0
+ RG=0
+ RS=0.0025
+ VTO=-3.20
+ M=3
+ SIGMA0=0
+ NSUB=4.6e15
+ PHI=0.6
+ UO=355
+ TOX=105nm
*
.DC VD 0 -5 -0.5v VG -3.35 -4.11 -0.25v
.PRINT DC V(1) V(3) V(2) I(VID)
.END
"
}

fn xyce_vdmos_rd_plus_drift_dc_deck() -> &'static str {
    "\
VDMOS RD plus drift topology oracle
VD 1 0 DC 10
VG 3 0 DC 8
VS 4 0 DC 0
VID 1 2 DC 0
M1 2 3 4 4 MAIN W=0.386 L=2.5u
.MODEL MAIN NMOS
+ LEVEL=18
+ CV=1
+ CVE=1
+ RD=0.5
+ RG=0
+ RS=0
+ VTO=3.5
+ M=3
+ SIGMA0=0
+ PHI=0.6
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=50nm
+ LAMBDA=0
+ D1IS=0
+ D1ISR=0
+ D1CJO=0
+ CJ=0
+ CJSW=0
+ IS=1e-30
+ JS=0
.DC VD 10 10 1 VG 8 8 1
.PRINT DC V(1) V(3) V(2) I(VID)
.END
"
}

#[test]
fn xyce_level18_nmos_routes_to_native_vdmos_without_simplified_opt_in() {
    let deck = "* xyce level 18 vdmos native\n\
                vd d 0 dc 12\n\
                vg g 0 dc 8\n\
                vs s 0 dc 0\n\
                m1 d g s 0 irf130 w=0.386 l=2.5u\n\
                .model irf130 nmos level=18\n\
                + cv=1 cve=1 vto=3.5 rd=0 rs=0.005 lambda=0\n\
                + m=3 sigma0=0 uo=230 vmax=4e4 delta=5 tox=50n\n\
                .op\n\
                .end\n";

    assert!(
        !deck.contains("allow_simplified_mos"),
        "LEVEL=18 must not need the simplified MOS escape hatch"
    );
    let report = run_report(deck);
    let m1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(m1.device_kind, "VDMOS");

    let id = m1
        .params
        .iter()
        .find_map(|(name, value)| (*name == "id").then_some(*value))
        .expect("id op value");
    assert!(
        id.is_finite() && id > 0.0,
        "native VDMOS should produce a finite positive drain current, got {id}"
    );
}

#[test]
fn xyce_level18_vdmos_rejects_unresolved_native_model_params() {
    let deck = "* xyce level 18 vdmos unresolved model param\n\
                vd d 0 dc 12\n\
                vg g 0 dc 8\n\
                vs s 0 dc 0\n\
                m1 d g s 0 irf130 w=0.386 l=2.5u\n\
                .model irf130 nmos level=18 vto={missing_vto} kp=2 rd=0 rs=0.005 lambda=0\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("VDMOS unresolved VTO deck parses");

    let message = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect_err("unresolved VDMOS model param must fail closed before native defaults")
        .to_string();

    assert!(
        message.contains("VDMOS")
            && message.contains("VTO")
            && message.contains("unresolved")
            && message.contains("finite numeric literal"),
        "unexpected unresolved VDMOS VTO error: {message}"
    );
}

#[test]
fn xyce_level18_irf130_dc_matches_xyce_gold_current() {
    let netlist = Netlist::parse(xyce_irf130_dc_deck()).expect("deck parses");
    let sweep2 = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc { sweep2, .. } => sweep2.clone(),
            _ => None,
        })
        .expect("second sweep captured");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep2_with_abort(
            &netlist,
            "vd",
            0.0,
            50.0,
            5.0,
            Some(&sweep2),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("xyce IRF130 sweep solves");

    // Xyce 7.10 regression `VDMOS_DC/irf130.cir.prn`.
    let reference = [
        9.939_647_41e-29,
        2.557_226_09,
        2.557_235_87,
        2.557_235_86,
        2.557_235_86,
        2.557_235_86,
        2.557_235_86,
        2.557_235_86,
        2.557_235_86,
        2.557_235_86,
        2.557_235_85,
        1.372_669_09e-15,
        24.077_541_5,
        32.898_226_7,
        33.473_232_2,
        33.612_093_2,
        33.636_241_0,
        33.636_579_3,
        33.635_910_0,
        33.635_345_4,
        33.634_979_8,
        33.634_918_6,
        1.895_666_77e-14,
        28.542_425_5,
        42.969_349_7,
        51.127_838_7,
        56.252_458_0,
        59.733_063_5,
        62.237_381_8,
        64.119_480_0,
        65.582_590_7,
        66.750_984_6,
        67.704_629_3,
    ];
    assert_eq!(results.len(), reference.len());

    for (idx, ((_, result), expected)) in results.iter().zip(reference).enumerate() {
        let got = result
            .branch_current_named("vid")
            .unwrap_or_else(|| panic!("missing VID branch in {:?}", result.branch_names));
        let abs = (got - expected).abs();
        // Xyce's Vgs=10 transition band is sensitive to the exact nonlinear
        // drift-branch Newton path; enforce a tight envelope there and strict
        // gold matching elsewhere.
        let knee_transition = (14..=20).contains(&idx);
        let tolerance = if knee_transition {
            5.0e-3 * expected.abs().max(1.0)
        } else {
            2.0e-6 * expected.abs().max(1.0)
        };
        assert!(
            abs < tolerance,
            "Xyce IRF130 row {idx}: rspice={got:.12e} xyce={expected:.12e} abs={abs:.3e}"
        );
    }
}

#[test]
fn xyce_level18_irhc110_dc_matches_xyce_gold_current() {
    let netlist = Netlist::parse(xyce_irhc110_dc_deck()).expect("deck parses");
    let sweep2 = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc { sweep2, .. } => sweep2.clone(),
            _ => None,
        })
        .expect("second sweep captured");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep2_with_abort(
            &netlist,
            "vd",
            0.0,
            10.0,
            1.0,
            Some(&sweep2),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("xyce IRHC110 sweep solves");

    // Xyce 7.10 regression `VDMOS_DC/irhc110.cir.prn`.
    let reference = [
        2.621_290_12e-25,
        2.168_432_58e-03,
        2.168_432_59e-03,
        2.168_432_59e-03,
        2.168_432_59e-03,
        2.168_432_59e-03,
        2.168_432_59e-03,
        2.168_432_60e-03,
        2.168_432_60e-03,
        2.168_432_60e-03,
        2.168_432_60e-03,
        2.397_127_41e-25,
        6.320_630_39e-01,
        6.709_923_47e-01,
        6.717_771_88e-01,
        6.717_924_38e-01,
        6.717_927_34e-01,
        6.717_927_40e-01,
        6.717_927_40e-01,
        6.717_927_40e-01,
        6.717_927_40e-01,
        6.717_927_40e-01,
        1.888_217_46e-25,
        1.294_283_00,
        1.876_518_54,
        1.990_414_42,
        2.005_573_19,
        2.007_491_84,
        2.007_733_86,
        2.007_764_39,
        2.007_768_24,
        2.007_768_72,
        2.007_768_78,
        -8.875_389_48e-23,
        1.585_952_06,
        2.781_292_82,
        3.380_809_55,
        3.573_992_33,
        3.623_699_72,
        3.635_750_38,
        3.638_640_08,
        3.639_332_02,
        3.639_497_66,
        3.639_537_32,
        -2.383_789_77e-20,
        1.744_808_31,
        3.264_041_38,
        4.372_980_21,
        4.989_901_60,
        5.251_591_06,
        5.346_118_18,
        5.378_221_94,
        5.388_927_94,
        5.392_481_53,
        5.393_659_87,
    ];
    assert_eq!(results.len(), reference.len());

    for (idx, ((_, result), expected)) in results.iter().zip(reference).enumerate() {
        let got = result
            .branch_current_named("vid")
            .unwrap_or_else(|| panic!("missing VID branch in {:?}", result.branch_names));
        let abs = (got - expected).abs();
        let tolerance = 2.0e-6 * expected.abs().max(1.0);
        assert!(
            abs < tolerance,
            "Xyce IRHC110 row {idx}: rspice={got:.12e} xyce={expected:.12e} abs={abs:.3e}"
        );
    }
}

#[test]
fn xyce_level18_irf130_transient_switching_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_irf130_tran_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 1.0e-6, 2.0e-9)
        .expect("Xyce IRF130 transient solves");

    assert!(
        result.time.len() > 100,
        "expected dense accepted transient points, got {}",
        result.time.len()
    );
    let v4 = transient_node_series(&result.node_names, &result.voltages, "4");
    let i_vid = result
        .try_branch_current_waveform_named("vid")
        .unwrap_or_else(|| panic!("missing VID branch in {:?}", result.branch_names));

    // Xyce 7.10 regression `VDMOS_TRAN/irf130-tran.cir.prn`.
    // The printed expression is `{I(VID)+0.5}`.
    let oracle = [
        (3.102_480_468_8e-7, 2.049_609_375_0, 0.500_000_000_0),
        (3.179_943_453_8e-7, 3.598_869_076_1, 0.519_737_371_92),
        (3.250_279_340_2e-7, 5.005_586_803_9, 1.725_921_473_6),
        (3.500_000_000_0e-7, 10.000_000_000, 3.660_882_045_6),
        (4.005_627_523_5e-7, 10.000_000_000, 3.693_171_411_7),
        (7.749_694_147_5e-7, 5.006_117_050_3, 1.812_170_223_2),
        (8.000_000_000_0e-7, 0.000_000_000_0, 0.500_000_000_0),
    ];

    let mut worst_expr = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
    for (t, expected_vg, expected_expr) in oracle {
        let got_vg = interpolate(&result.time, v4, t);
        let got_expr = interpolate(&result.time, i_vid, t) + 0.5;
        let gate_abs = (got_vg - expected_vg).abs();
        assert!(
            gate_abs < 8.0e-2,
            "gate pulse mismatch at t={t:.12e}: rspice={got_vg:.12e} xyce={expected_vg:.12e} abs={gate_abs:.3e}"
        );

        let expr_abs = (got_expr - expected_expr).abs();
        let expr_tol = 8.0e-2 * expected_expr.abs().max(1.0);
        if expr_abs / expr_tol > worst_expr.3 {
            worst_expr = (t, got_expr, expected_expr, expr_abs / expr_tol);
        }
    }

    assert!(
        worst_expr.3 <= 1.0,
        "Xyce IRF130 transient current mismatch at t={:.12e}: rspice={{I(VID)+0.5}}={:.12e} xyce={:.12e} tol_ratio={:.3e}",
        worst_expr.0,
        worst_expr.1,
        worst_expr.2,
        worst_expr.3
    );
}

#[test]
fn xyce_level18_vdmos_cgdo_displacement_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_cgdo_displacement_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 15.1e-9, 0.1e-9)
        .expect("Xyce VDMOS displacement-current transient solves");

    let vg = result
        .try_voltage_waveform_named("g")
        .expect("gate waveform present");
    let i_vd = result
        .try_branch_current_waveform_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // Custom Xyce 7.10 LEVEL=18 oracle with VTO=100 forcing the channel off:
    // CGDO=10 pF and dVg/dt=1 V/ns gives about 10 mA through the drain source.
    let oracle = [
        (9.85e-9, 0.0, -1.203_312_770_709e-12),
        (
            10.086_974_773_54e-9,
            8.697_477_354_029e-2,
            1.000_000_000_759e-2,
        ),
        (
            12.523_383_456_59e-9,
            2.523_383_456_590,
            1.000_000_000_838e-2,
        ),
        (
            15.023_383_456_59e-9,
            5.023_383_456_590,
            9.999_999_988_950e-3,
        ),
    ];

    for (t, expected_vg, expected_current) in oracle {
        let got_vg = interpolate(&result.time, vg, t);
        let got_current = interpolate(&result.time, i_vd, t);
        assert!(
            (got_vg - expected_vg).abs() < 5.0e-2,
            "gate waveform mismatch at t={t:.12e}: rspice={got_vg:.12e} xyce={expected_vg:.12e}"
        );
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-3_f64.max(5.0e-2 * expected_current.abs());
        assert!(
            abs < tol,
            "VDMOS CGDO displacement current mismatch at t={t:.12e}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_cgbo_displacement_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_cgbo_displacement_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5.0e-7, 1.0e-9)
        .expect("Xyce VDMOS gate-bulk displacement transient solves");

    let i_vb = result
        .try_branch_current_waveform_named("vb")
        .unwrap_or_else(|| panic!("missing VB branch in {:?}", result.branch_names));

    // Custom Xyce 7.10 LEVEL=18 oracle with VTO=100 forcing the channel off
    // and TOX=1 making intrinsic Meyer Cgb negligible. Xyce source scales
    // CGBO by effective channel length (`GateBulkOverlapCap = CGBO * L`), so
    // CGBO=1e-5 and L=1um produce a 10 pF gate-bulk overlap capacitor.
    for &(t, expected_current) in &[
        (1.0e-9, 6.283_165_662_159e-5),
        (5.0e-7, -6.283_197_707_149e-5),
    ] {
        let got_current = interpolate(&result.time, i_vb, t);
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-6_f64.max(expected_current.abs() * 0.08);
        assert!(
            abs <= tol,
            "VDMOS CGBO displacement current mismatch at t={t:.12e}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_cbs_displacement_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_cbs_displacement_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5.0e-7, 1.0e-9)
        .expect("Xyce VDMOS bulk-source displacement transient solves");

    let i_vb = result
        .try_branch_current_waveform_named("vb")
        .unwrap_or_else(|| panic!("missing VB branch in {:?}", result.branch_names));

    // Custom Xyce 7.10 LEVEL=18 oracle. The explicit CBS branch is isolated by
    // forcing the channel and diode currents off. Xyce prints the transient
    // lead current after trapezoidal charge integration, so the first 1 ns
    // point is approximately 2*C*DeltaV/dt rather than the continuous sine
    // derivative.
    for &(t, expected_current) in &[
        (1.0e-9, 1.256_626_325_770e-3),
        (5.0e-7, -1.256_632_568_559e-3),
    ] {
        let got_current = interpolate(&result.time, i_vb, t);
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-5_f64.max(expected_current.abs() * 0.08);
        assert!(
            abs <= tol,
            "VDMOS CBS displacement current mismatch at t={t:.12e}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_cj_as_displacement_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_cj_as_displacement_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5.0e-7, 1.0e-9)
        .expect("Xyce VDMOS CJ/AS displacement transient solves");

    let i_vb = result
        .try_branch_current_waveform_named("vb")
        .unwrap_or_else(|| panic!("missing VB branch in {:?}", result.branch_names));

    // Custom Xyce 7.10 LEVEL=18 oracle. With CBS omitted, CJ=0.04/AS=2e-6
    // and CJSW=0.02/PS=1e-6 derive the same 100 nF zero-bias source-body
    // capacitance as the explicit-CBS oracle above.
    for &(t, expected_current) in &[
        (1.0e-9, 1.256_626_325_770e-3),
        (5.0e-7, -1.256_632_568_559e-3),
    ] {
        let got_current = interpolate(&result.time, i_vb, t);
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-5_f64.max(expected_current.abs() * 0.08);
        assert!(
            abs <= tol,
            "VDMOS CJ/AS displacement current mismatch at t={t:.12e}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_d1cjo_displacement_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1cjo_displacement_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5.0e-7, 1.0e-9)
        .expect("Xyce VDMOS D1CJO displacement transient solves");

    let i_vd = result
        .try_branch_current_waveform_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // Custom Xyce 7.10 LEVEL=18 oracle. D1RS=0 aliases D1Prime to source,
    // so D1CJO is isolated as a two-terminal drain-source charge branch.
    for &(t, expected_current) in &[
        (1.0e-9, -6.283_134_096_119e-4),
        (5.0e-7, 6.283_167_144_615e-4),
    ] {
        let got_current = interpolate(&result.time, i_vd, t);
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-6_f64.max(expected_current.abs() * 0.08);
        assert!(
            abs <= tol,
            "VDMOS D1CJO displacement current mismatch at t={t:.12e}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_d1cjo_temperature_displacement_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1cjo_temp_displacement_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5.0e-7, 1.0e-9)
        .expect("Xyce VDMOS D1CJO temperature transient solves");

    let i_vd = result
        .try_branch_current_waveform_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // XyceNF 7.10.0 LEVEL=18 oracle at `.OPTIONS TEMP=125`. The channel,
    // body junctions, D1 current, and D1 transit time are disabled; this
    // isolates the temperature-adjusted D1 junction capacitance.
    for &(t, expected_current) in &[
        (1.0e-9, -7.590_820_736_477e-4),
        (5.0e-7, 7.590_869_215_081e-4),
    ] {
        let got_current = interpolate(&result.time, i_vd, t);
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-6_f64.max(expected_current.abs() * 0.08);
        assert!(
            abs <= tol,
            "VDMOS D1CJO temperature displacement mismatch at t={t:.12e}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_d1is_dc_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1is_dc_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("Xyce VDMOS D1IS DC operating point solves");

    let i_vd = result
        .branch_current_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // Custom Xyce 7.10 LEVEL=18 oracle. The channel and bulk diodes are off;
    // D1IS/D1N supply the drain-source diode current at V(D)=-0.5 V.
    let expected_current = 2.486_981_754_870e-4;
    let abs = (i_vd - expected_current).abs();
    let tol = 1.0e-7_f64.max(expected_current.abs() * 0.02);
    assert!(
        abs <= tol,
        "VDMOS D1IS DC current mismatch: rspice={i_vd:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

#[test]
fn xyce_level18_vdmos_d1is_current_uses_external_drain_source_nodes_with_rd() {
    let netlist = Netlist::parse(xyce_vdmos_d1is_external_rd_dc_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("Xyce VDMOS D1IS external-node DC operating point solves");

    let i_vd = result
        .branch_current_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // Custom Xyce 7.10 LEVEL=18 oracle. With D1RS=0, D1Prime aliases the
    // source node and the D1 current is controlled by the external drain-source
    // voltage even when RD creates a separate channel drain node.
    let expected_current = 2.486_981_754_870e-4;
    let abs = (i_vd - expected_current).abs();
    let tol = 1.0e-7_f64.max(expected_current.abs() * 0.02);
    assert!(
        abs <= tol,
        "VDMOS D1IS external-node DC current mismatch: rspice={i_vd:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

#[test]
fn xyce_level18_vdmos_d1is_temperature_scaling_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1is_temp_dc_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("Xyce VDMOS D1IS temperature DC operating point solves");

    let i_vd = result
        .branch_current_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // XyceNF 7.10.0 LEVEL=18 oracle at `.OPTIONS TEMP=75`. The channel,
    // body junctions, D1 capacitance, and D1 recombination are disabled; the
    // current increase comes from D1EG/D1XTI temperature scaling of D1IS.
    let expected_current = 1.276_022_408_751e-5;
    let abs = (i_vd - expected_current).abs();
    let tol = 1.0e-8_f64.max(expected_current.abs() * 0.02);
    assert!(
        abs <= tol,
        "VDMOS D1IS temperature DC current mismatch: rspice={i_vd:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

#[test]
fn xyce_level18_vdmos_d1tnom_is_ignored_like_xyce() {
    let netlist =
        Netlist::parse(xyce_vdmos_d1is_temp_ignores_d1tnom_dc_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("Xyce VDMOS D1TNOM ignored temperature DC operating point solves");

    let i_vd = result
        .branch_current_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // XyceNF 7.10.0 parses D1TNOM but overwrites the D1 nominal temperature
    // with model/general TNOM during VDMOS temperature setup, so D1TNOM=500
    // gives the same result as the D1TNOM=300.15 oracle above.
    let expected_current = 1.276_022_408_751e-5;
    let abs = (i_vd - expected_current).abs();
    let tol = 1.0e-8_f64.max(expected_current.abs() * 0.02);
    assert!(
        abs <= tol,
        "VDMOS D1TNOM compatibility mismatch: rspice={i_vd:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

#[test]
fn xyce_level18_vdmos_d1tt_diffusion_charge_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1tt_displacement_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5.0e-7, 1.0e-9)
        .expect("Xyce VDMOS D1TT transient solves");

    let i_vd = result
        .try_branch_current_waveform_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // Custom Xyce 7.10 LEVEL=18 oracle with D1CJO=0. The current comes from
    // D1TT * d(D1 diode current)/dt plus the D1 diode's own F-vector current.
    for &(t, expected_current) in &[
        (1.0e-9, -2.431_417_443_665e-7),
        (5.0e-7, 2.429_284_656_863e-7),
    ] {
        let got_current = interpolate(&result.time, i_vd, t);
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-9_f64.max(expected_current.abs() * 0.08);
        assert!(
            abs <= tol,
            "VDMOS D1TT diffusion-charge current mismatch at t={t:.12e}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_d1rs_series_resistance_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1rs_dc_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("Xyce VDMOS D1RS DC operating point solves");

    let i_vd = result
        .branch_current_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));

    // XyceNF 7.10.0, same deck with `.print dc I(VD)`: D1RS creates the
    // internal D1Prime node and the source-to-D1Prime series resistor.
    let expected_current = 2.934_812_120_323e-3;
    let abs = (i_vd - expected_current).abs();
    let tol = 1.0e-7_f64.max(expected_current.abs() * 0.01);
    assert!(
        abs <= tol,
        "VDMOS D1RS DC current mismatch: rspice={i_vd:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

#[test]
fn xyce_level18_vdmos_d1bv_breakdown_sweep_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1bv_dc_deck()).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep_with_abort(
            &netlist,
            "vd",
            4.8,
            5.2,
            0.1,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("Xyce VDMOS D1BV DC sweep solves");

    // XyceNF 7.10.0 LEVEL=18 oracle with the channel, body junctions, and D1
    // capacitance disabled. Positive V(D) reverse-biases the NMOS D1 diode and
    // exercises the D1BV/D1IBV breakdown branch.
    let expected = [
        (4.8, -4.382_590_423_942e-7),
        (4.9, -2.093_463_558_774e-5),
        (5.0, -9.999_998_284_057e-4),
        (5.1, -4.776_771_263_204e-2),
        (5.2, -2.281_754_761_633e0),
    ];
    assert_eq!(results.len(), expected.len());

    for ((got_vd, result), (want_vd, expected_current)) in results.iter().zip(expected) {
        assert!(
            (got_vd - want_vd).abs() < 1.0e-12,
            "VDMOS D1BV sweep point mismatch: rspice={got_vd:.12e} xyce={want_vd:.12e}"
        );
        let got_current = result
            .branch_current_named("vd")
            .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-8_f64.max(expected_current.abs() * 0.01);
        assert!(
            abs <= tol,
            "VDMOS D1BV DC current mismatch at V(D)={want_vd:.3}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_d1isr_recombination_sweep_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1isr_dc_deck()).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep_with_abort(
            &netlist,
            "vd",
            -0.1,
            -0.5,
            -0.1,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("Xyce VDMOS D1ISR DC sweep solves");

    // XyceNF 7.10.0 LEVEL=18 oracle. D1IS is effectively zero, so these
    // currents are produced by the D1ISR/D1NR recombination branch.
    let expected = [
        (-0.1, 5.609_795_121_782e-9),
        (-0.2, 4.184_665_157_401e-8),
        (-0.3, 2.755_209_847_088e-7),
        (-0.4, 1.767_890_920_031e-6),
        (-0.5, 1.116_161_891_449e-5),
    ];
    assert_eq!(results.len(), expected.len());

    for ((got_vd, result), (want_vd, expected_current)) in results.iter().zip(expected) {
        assert!(
            (got_vd - want_vd).abs() < 1.0e-12,
            "VDMOS D1ISR sweep point mismatch: rspice={got_vd:.12e} xyce={want_vd:.12e}"
        );
        let got_current = result
            .branch_current_named("vd")
            .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-10_f64.max(expected_current.abs() * 0.01);
        assert!(
            abs <= tol,
            "VDMOS D1ISR DC current mismatch at V(D)={want_vd:.3}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_d1isr_uses_temperature_adjusted_junction_potential() {
    let netlist = Netlist::parse(xyce_vdmos_d1isr_temp_vj_dc_deck()).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep_with_abort(
            &netlist,
            "vd",
            -0.2,
            -0.6,
            -0.1,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("Xyce VDMOS D1ISR temperature sweep solves");

    // XyceNF 7.10.0 LEVEL=18 oracle at `.OPTIONS TEMP=125`. D1ISR itself is
    // not temperature-scaled; the change comes from Xyce's temperature-adjusted
    // D1 junction potential inside the recombination branch.
    let expected = [
        (-0.2, 1.190_738_301_889e-8),
        (-0.3, 3.496_394_285_535e-8),
        (-0.4, 9.409_914_786_095e-8),
        (-0.5, 8.502_072_926_059e-7),
        (-0.6, 4.883_260_151_444e-6),
    ];
    assert_eq!(results.len(), expected.len());

    for ((got_vd, result), (want_vd, expected_current)) in results.iter().zip(expected) {
        assert!(
            (got_vd - want_vd).abs() < 1.0e-12,
            "VDMOS D1ISR temperature sweep point mismatch: rspice={got_vd:.12e} xyce={want_vd:.12e}"
        );
        let got_current = result
            .branch_current_named("vd")
            .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-10_f64.max(expected_current.abs() * 0.01);
        assert!(
            abs <= tol,
            "VDMOS D1ISR temperature current mismatch at V(D)={want_vd:.3}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_d1ikf_high_injection_sweep_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_vdmos_d1ikf_dc_deck()).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep_with_abort(
            &netlist,
            "vd",
            -0.3,
            -0.6,
            -0.1,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("Xyce VDMOS D1IKF DC sweep solves");

    // XyceNF 7.10.0 LEVEL=18 oracle. The channel, body junctions, and D1
    // recombination branch are disabled; D1IKF attenuates the forward D1 diode
    // current once it approaches the high-injection knee.
    let expected = [
        (-0.3, 1.084_040_785_745e-7),
        (-0.4, 4.222_063_023_553e-6),
        (-0.5, 4.889_629_084_493e-5),
        (-0.6, 3.445_250_854_548e-4),
    ];
    assert_eq!(results.len(), expected.len());

    for ((got_vd, result), (want_vd, expected_current)) in results.iter().zip(expected) {
        assert!(
            (got_vd - want_vd).abs() < 1.0e-12,
            "VDMOS D1IKF sweep point mismatch: rspice={got_vd:.12e} xyce={want_vd:.12e}"
        );
        let got_current = result
            .branch_current_named("vd")
            .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));
        let abs = (got_current - expected_current).abs();
        let tol = 1.0e-10_f64.max(expected_current.abs() * 0.01);
        assert!(
            abs <= tol,
            "VDMOS D1IKF DC current mismatch at V(D)={want_vd:.3}: rspice={got_current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
        );
    }
}

#[test]
fn xyce_level18_vdmos_rd_plus_drift_matches_xyce_gold_current() {
    let netlist = Netlist::parse(xyce_vdmos_rd_plus_drift_dc_deck()).expect("deck parses");
    let sweep2 = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc { sweep2, .. } => sweep2.clone(),
            _ => None,
        })
        .expect("second sweep captured");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep2_with_abort(
            &netlist,
            "vd",
            10.0,
            10.0,
            1.0,
            Some(&sweep2),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("xyce VDMOS RD plus drift deck solves");
    assert_eq!(results.len(), 1);

    let current = results[0]
        .1
        .branch_current_named("vid")
        .unwrap_or_else(|| panic!("missing VID branch in {:?}", results[0].1.branch_names));
    let expected_current = 8.224_834_09;
    let abs = (current - expected_current).abs();
    let tol = 2.0e-7;
    assert!(
        abs <= tol,
        "VDMOS RD+drift DC current mismatch: rspice={current:.12e} xyce={expected_current:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

#[test]
fn xyce_level18_mtb60p06v_pmos_dc_matches_xyce_gold_current() {
    let netlist = Netlist::parse(xyce_mtb60p06v_dc_deck()).expect("deck parses");
    let sweep2 = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc { sweep2, .. } => sweep2.clone(),
            _ => None,
        })
        .expect("second sweep captured");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep2_with_abort(
            &netlist,
            "vd",
            0.0,
            -5.0,
            -0.5,
            Some(&sweep2),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("xyce MTB60P06V sweep solves");

    // Xyce 7.10 regression `VDMOS_DC/mtb60p06v.cir.prn`.
    let reference = [
        -1.538_846_75e-25,
        -1.224_691_37e-1,
        -1.253_887_55e-1,
        -1.282_433_03e-1,
        -1.310_970_84e-1,
        -1.339_501_10e-1,
        -1.368_023_84e-1,
        -1.396_539_04e-1,
        -1.425_046_73e-1,
        -1.453_546_89e-1,
        -1.482_039_54e-1,
        -1.373_249_06e-25,
        -6.343_194_77e-1,
        -6.698_572_90e-1,
        -6.852_069_24e-1,
        -7.003_324_13e-1,
        -7.154_463_50e-1,
        -7.305_509_36e-1,
        -7.456_462_05e-1,
        -7.607_321_67e-1,
        -7.758_088_34e-1,
        -7.908_762_16e-1,
        -1.229_865_71e-25,
        -1.344_187_67,
        -1.632_119_26,
        -1.684_026_70,
        -1.721_646_28,
        -1.758_540_00,
        -1.795_365_67,
        -1.832_154_78,
        -1.868_908_89,
        -1.905_628_14,
        -1.942_312_60,
        -1.114_755_12e-25,
        -1.941_928_41,
        -2.840_431_51,
        -3.054_842_83,
        -3.138_194_45,
        -3.206_651_94,
        -3.273_499_83,
        -3.340_109_63,
        -3.406_621_33,
        -3.473_049_32,
        -3.539_395_23,
    ];
    assert_eq!(results.len(), reference.len());

    let mut worst = (0usize, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
    for (idx, ((_, result), expected)) in results.iter().zip(reference).enumerate() {
        let got = result
            .branch_current_named("vid")
            .unwrap_or_else(|| panic!("missing VID branch in {:?}", result.branch_names));
        let abs = (got - expected).abs();
        // This deck omits GAMMAS0 but supplies NSUB; Xyce derives GAMMAS0
        // from NSUB/TOX before evaluating the LEVEL=18 UCCM channel.
        let tolerance = 1.0e-4 * expected.abs().max(1.0);
        let rel = abs / expected.abs().max(1.0e-30);
        if abs / tolerance > worst.3 {
            worst = (idx, got, expected, abs / tolerance, rel);
        }
    }
    assert!(
        worst.3 <= 1.0,
        "Xyce MTB60P06V worst row {}: rspice={:.12e} xyce={:.12e} tol_ratio={:.3e} rel={:.3e}",
        worst.0,
        worst.1,
        worst.2,
        worst.3,
        worst.4
    );
}

#[test]
fn ngspice_vdmos_model_routes_to_same_native_device() {
    let deck = "* ngspice vdmos native\n\
                vd d 0 dc 12\n\
                vg g 0 dc 8\n\
                vs s 0 dc 0\n\
                m1 d g s irf130\n\
                .model irf130 vdmos nchan vto=3.5 kp=2 rd=0 rs=0.005 lambda=0\n\
                .op\n\
                .end\n";

    let report = run_report(deck);
    let m1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(m1.device_kind, "VDMOS");
}

#[test]
fn ngspice_vdmosn_and_vdmosp_aliases_route_to_native_device() {
    for (model_type, drain_bias, gate_bias, expected_id_sign) in
        [("vdmosn", 12.0, 8.0, 1.0), ("vdmosp", -12.0, -8.0, -1.0)]
    {
        let deck = format!(
            "* ngspice {model_type} native alias\n\
             vd d 0 dc {drain_bias}\n\
             vg g 0 dc {gate_bias}\n\
             vs s 0 dc 0\n\
             m1 d g s dmod\n\
             .model dmod {model_type} vto=3.5 kp=2 rd=0 rs=0.005 lambda=0\n\
             .op\n\
             .end\n"
        );

        let report = run_report(&deck);
        let m1 = report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
            .unwrap_or_else(|| panic!("{model_type} m1 op entry"));
        assert_eq!(m1.device_kind, "VDMOS");

        let id = m1
            .params
            .iter()
            .find_map(|(name, value)| (*name == "id").then_some(*value))
            .unwrap_or_else(|| panic!("{model_type} id op value"));
        assert!(
            id.is_finite() && id * expected_id_sign > 0.0,
            "{model_type} should preserve native channel polarity, got id={id}"
        );
    }
}

#[test]
fn ngspice_vdmos_rejects_non_numeric_native_model_params() {
    let deck = "* ngspice vdmos non-numeric model param\n\
                vd d 0 dc 12\n\
                vg g 0 dc 8\n\
                vs s 0 dc 0\n\
                m1 d g s irf130\n\
                .model irf130 vdmos nchan vto=3.5 kp=\"2\" rd=0 rs=0.005 lambda=0\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("VDMOS non-numeric KP deck parses");

    let message = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect_err("non-numeric VDMOS model param must fail closed before native defaults")
        .to_string();

    assert!(
        message.contains("VDMOS")
            && message.contains("KP")
            && message.contains("non-numeric")
            && message.contains("finite numeric literal"),
        "unexpected non-numeric VDMOS KP error: {message}"
    );
}

#[test]
fn vdmos_participates_in_ac_small_signal_linearization() {
    let deck = "* vdmos ac small signal\n\
                vdd vdd 0 dc 12\n\
                vin g 0 dc 8 ac 1\n\
                rd vdd out 10\n\
                rs s 0 1m\n\
                m1 out g s 0 irf130 w=0.386 l=2.5u\n\
                .model irf130 nmos level=18 vto=3.5 rd=0 rs=0.005 lambda=0 kp=2\n\
                .op\n\
                .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let results = engine.run_ac(&netlist, &[1.0e3]).expect("ac runs");
    let out = ac_voltage_named(&results[0], "out");

    assert!(
        out.norm().is_finite() && out.norm() > 1.0e-3,
        "VDMOS common-source AC output should be driven by gm/caps, got {out}"
    );
}

#[test]
fn xyce_level18_vdmos_cbs_ac_current_matches_xyce_gold() {
    // Xyce 7.10 LEVEL=18 reference from the same deck with
    // `.PRINT AC FORMAT=NOINDEX IR(VB) II(VB)`. Unlike the transient path,
    // Xyce's AC linearization does not load the VDMOS CBS charge as a
    // small-signal capacitance; only the tiny diode conductance remains.
    let deck = xyce_vdmos_cbs_ac_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_ac(&netlist, &[1.0e6])
        .expect("Xyce VDMOS CBS AC deck runs")
        .pop()
        .expect("one AC point");
    let current = ac_branch_current_named(&result, "vb");

    let expected = rspice_core::Complex64::new(-2.0e-12, 0.0);
    assert!(
        (current.re - expected.re).abs() < 1.0e-11,
        "VDMOS CBS AC real branch current mismatch: rspice={:.12e} xyce={:.12e}",
        current.re,
        expected.re
    );
    assert!(
        (current.im - expected.im).abs() < 1.0e-13,
        "VDMOS CBS AC imaginary branch current mismatch: rspice={:.12e} xyce={:.12e}",
        current.im,
        expected.im
    );
}

#[test]
fn xyce_level18_vdmos_ac_charge_branches_are_omitted_like_xyce() {
    // Xyce 7.10 LEVEL=18 references from isolated AC decks at 1 MHz with
    // `.PRINT AC FORMAT=NOINDEX IR(V*) II(V*)`. These charge parameters are
    // active in transient, but Xyce does not load them into its AC matrix.
    let cases = [
        (
            "CGSO",
            xyce_vdmos_ac_charge_branch_deck(
                "Cgso",
                "VD d 0 0",
                "VG g 0 0",
                "VS s 0 dc 0 ac 1",
                "VB b 0 0",
                "+ CGSO=1e-7\n+ CGDO=0\n+ CGBO=0\n+ CBD=0\n+ CBS=0\n+ CJ=0\n+ CJSW=0\n+ D1CJO=0",
                "VR(s) VI(s) IR(VS) II(VS) IM(VS) IP(VS)",
            ),
            "vs",
            -1.0e-12,
        ),
        (
            "CGDO",
            xyce_vdmos_ac_charge_branch_deck(
                "Cgdo",
                "VD d 0 dc 0 ac 1",
                "VG g 0 0",
                "VS s 0 0",
                "VB b 0 0",
                "+ CGSO=0\n+ CGDO=1e-7\n+ CGBO=0\n+ CBD=0\n+ CBS=0\n+ CJ=0\n+ CJSW=0\n+ D1CJO=0",
                "VR(d) VI(d) IR(VD) II(VD) IM(VD) IP(VD)",
            ),
            "vd",
            -2.000_177_80e-12,
        ),
        (
            "CGBO",
            xyce_vdmos_ac_charge_branch_deck(
                "Cgbo",
                "VD d 0 0",
                "VG g 0 dc 0 ac 1",
                "VS s 0 0",
                "VB b 0 0",
                "+ CGSO=0\n+ CGDO=0\n+ CGBO=1e-5\n+ CBD=0\n+ CBS=0\n+ CJ=0\n+ CJSW=0\n+ D1CJO=0",
                "VR(g) VI(g) IR(VG) II(VG) IM(VG) IP(VG)",
            ),
            "vg",
            0.0,
        ),
        (
            "CBD",
            xyce_vdmos_ac_charge_branch_deck(
                "Cbd",
                "VD d 0 0",
                "VG g 0 0",
                "VS s 0 0",
                "VB b 0 dc 0 ac 1",
                "+ CGSO=0\n+ CGDO=0\n+ CGBO=0\n+ CBD=1e-7\n+ CBS=0\n+ CJ=0\n+ CJSW=0\n+ D1CJO=0",
                "VR(b) VI(b) IR(VB) II(VB) IM(VB) IP(VB)",
            ),
            "vb",
            -2.0e-12,
        ),
        (
            "D1CJO",
            xyce_vdmos_ac_charge_branch_deck(
                "D1cjo",
                "VD d 0 dc 0 ac 1",
                "VG g 0 0",
                "VS s 0 0",
                "VB b 0 0",
                "+ CGSO=0\n+ CGDO=0\n+ CGBO=0\n+ CBD=0\n+ CBS=0\n+ CJ=0\n+ CJSW=0\n+ D1CJO=1e-7\n+ D1VJ=1\n+ D1M=0.5\n+ D1FC=0.5",
                "VR(d) VI(d) IR(VD) II(VD) IM(VD) IP(VD)",
            ),
            "vd",
            -2.000_177_80e-12,
        ),
    ];

    for (label, deck, branch, expected_real) in cases {
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_ac(&netlist, &[1.0e6])
            .unwrap_or_else(|err| panic!("{label} AC deck runs: {err}"))
            .pop()
            .expect("one AC point");
        let current = ac_branch_current_named(&result, branch);
        assert!(
            (current.re - expected_real).abs() < 1.0e-10,
            "{label} AC real branch current mismatch: rspice={:.12e} xyce={:.12e}",
            current.re,
            expected_real
        );
        assert!(
            current.im.abs() < 1.0e-13,
            "{label} AC imaginary branch current mismatch: rspice={:.12e} xyce=0",
            current.im
        );
    }
}
