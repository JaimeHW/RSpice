//! Regression coverage for the numerical structure used by public power-MOS
//! behavioral macromodels: nested functions, branch-current probes, TABLE
//! sources, and capacitor-current sensing networks in a DC continuation sweep.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn node_index(result: &rspice_core::solver::SimulationResult, name: &str) -> usize {
    result
        .node_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing {name} node in {:?}", result.node_names))
}

fn branch_index(result: &rspice_core::solver::SimulationResult, name: &str) -> usize {
    result
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing {name} branch in {:?}", result.branch_names))
}

#[test]
fn power_macro_load_line_sweep_reaches_the_analytic_on_state() {
    let deck = r#"
* clean-room structural regression for a power-MOS behavioral macro
VDD SUP 0 10
RLOAD SUP D 100
VGS G 0 0
XPOWER D G 0 POWER_MACRO

.SUBCKT POWER_MACRO D G S
E_TEMP TJ TREF VALUE={TEMP}
R_TEMP TREF 0 1m
R_TJ TJ 0 10G

.FUNC CHAN(VG, VD) {IF(VG > 3, IF(VD > 0, VD, 0), 0)}
V_SENSE D DIN 0
G_CH DIN S VALUE={CHAN(V(G,S), V(DIN,S))}

* Retain a branch-current dependency without changing the external load line.
G_MIRROR MIRROR 0 VALUE={I(V_SENSE)}
R_MIRROR MIRROR 0 1

* Voltage-dependent capacitance probes use the stiff sensing topology found in
* practical power-device macros. Their DC current is zero by construction.
E_DG DG1 0 VALUE={V(D,G)}
C_DG DG1 DG2 1p
R_DG DG2 DG3 1u
V_DG DG3 0 0
G_DG D G VALUE={V(CDG)*I(V_DG)*1e12}
R_CDG CDG 0 1k
E_CDG CDG 0 TABLE={V(D,G)} (0.1,1p) (2,0.7p) (10,0.2p)

E_DS DS1 0 VALUE={V(D,S)}
C_DS DS1 DS2 1p
R_DS_PROBE DS2 DS3 1
V_DS_PROBE DS3 0 0
G_DS D S VALUE={V(CDS)*I(V_DS_PROBE)*1e12}
R_CDS CDS 0 1k
E_CDS CDS 0 TABLE={V(D,S)} (0.1,5p) (2,2p) (10,0.5p)

R_D_LEAK D 0 10T
.ENDS POWER_MACRO
.END
"#;

    let netlist = Netlist::parse(deck).expect("behavioral power macro parses unchanged");
    let results = Engine::new(SimulationConfig::default())
        .run_dc_sweep(&netlist, "VGS", 0.0, 5.0, 0.1)
        .expect("behavioral power macro load-line sweep converges");
    assert_eq!(results.len(), 51, "the complete gate sweep must execute");

    let (_, result) = results.last().expect("VGS=5 result exists");
    let drain = result.node_voltages[node_index(result, "D")];
    let supply_current = result.branch_currents[branch_index(result, "VDD")];

    // At VGS=5 the authored channel is exactly one siemens. With a 10 V
    // source and 100 ohm load, Vd = 10 / (1 + 100) and I(VDD) = -Vd.
    let expected = 10.0 / 101.0;
    assert!(
        (drain - expected).abs() < 1e-10,
        "unexpected final drain voltage: got {drain:.16e}, expected {expected:.16e}"
    );
    assert!(
        (supply_current + expected).abs() < 1e-10,
        "unexpected final supply current: got {supply_current:.16e}, expected -{expected:.16e}"
    );
}
