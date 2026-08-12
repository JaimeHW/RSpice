//! Per-device operating-point report: device bias and small-signal values
//! surfaced after a DC solve.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

#[test]
fn mosfet_report_carries_bias_and_small_signal_values() {
    // NMOS in saturation: VGS = 2V (VTO = 1), VDS = 5V > VGS - VTO.
    let deck = "\
* mosfet op report
vdd d 0 dc 5
vg g 0 dc 2
m1 d g 0 0 nmod w=10u l=1u
.model nmod NMOS (LEVEL=1 VTO=1 KP=100u LAMBDA=0)
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, report) = engine
        .run_dc_op_with_report(&netlist)
        .expect("operating point solves");

    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 present in OP report");
    assert_eq!(entry.device_kind, "MOSFET");
    assert_eq!(entry.region, Some("saturation"));

    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
            .unwrap_or_else(|| panic!("{key} missing from MOSFET OP entry"))
    };

    // Level-1 closed form: Id = KP/2 * W/L * (Vgs - Vth)^2 = 50u*10*(1)^2.
    let id = get("id");
    assert!(
        (id - 500e-6).abs() / 500e-6 < 1e-3,
        "saturation current: got {id}"
    );
    let gm = get("gm");
    // gm = KP * W/L * (Vgs - Vth) = 1m
    assert!(
        (gm - 1e-3).abs() / 1e-3 < 1e-3,
        "transconductance: got {gm}"
    );
    assert!((get("vgs") - 2.0).abs() < 1e-9);
    assert!((get("vds") - 5.0).abs() < 1e-9);
    assert!((get("vth") - 1.0).abs() < 1e-9);
}

#[test]
fn diode_report_is_consistent_with_shockley_equation() {
    let deck = "\
* diode op report
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (result, report) = engine
        .run_dc_op_with_report(&netlist)
        .expect("operating point solves");

    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("d1"))
        .expect("d1 present in OP report");
    assert_eq!(entry.device_kind, "DIODE");

    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
            .unwrap()
    };

    // The reported junction voltage must equal the solved anode voltage and
    // the reported current must satisfy KCL with the series resistor.
    let anode_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("anode node");
    let v_a = result.node_voltages[anode_idx];
    assert!((get("vd") - v_a).abs() < 1e-9, "vd vs solved anode voltage");

    let i_r = (5.0 - v_a) / 1000.0;
    assert!(
        (get("id") - i_r).abs() / i_r.abs() < 1e-6,
        "diode current consistent with resistor current: {} vs {}",
        get("id"),
        i_r
    );
    assert!(get("gd") > 0.0, "conductance positive in forward bias");
    assert!(get("cd").is_finite(), "capacitance is reported");
}

#[test]
fn bjt_report_carries_currents_and_beta() {
    let deck = "\
* bjt op report
vcc c 0 dc 10
vbb bdrv 0 dc 2
rb bdrv b 100k
rc c col 1k
q1 col b 0 qmod
.model qmod NPN (IS=1e-15 BF=100)
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, report) = engine
        .run_dc_op_with_report(&netlist)
        .expect("operating point solves");

    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("q1"))
        .expect("q1 present in OP report");
    assert_eq!(entry.device_kind, "BJT");

    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
            .unwrap()
    };

    // Forward-active: vbe near 0.7V, ic = beta*ib with beta near BF.
    let vbe = get("vbe");
    assert!((0.55..0.85).contains(&vbe), "forward-active vbe: got {vbe}");
    let beta = get("beta");
    assert!(
        (50.0..150.0).contains(&beta),
        "beta near BF=100: got {beta}"
    );
    assert!(get("ic") > 0.0 && get("ib") > 0.0);
}

/// One deck per device family, so the vocabulary check below runs against
/// labels a real solve produced rather than against hand-built entries.
fn family_decks() -> Vec<(&'static str, String)> {
    vec![
        (
            "MOSFET",
            "* classic level-1 MOSFET\n\
             vdd d 0 dc 5\n\
             vg g 0 dc 2\n\
             m1 d g 0 0 nmod w=10u l=1u\n\
             .model nmod NMOS (LEVEL=1 VTO=1 KP=100u)\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "MOS9",
            "* ngspice MOS9\n\
             vdd d 0 dc 3\n\
             vg g 0 dc 2\n\
             m1 d g 0 0 nmod w=10u l=1u\n\
             .model nmod NMOS LEVEL=9 VTO=0.7 KP=110U GAMMA=0.4 PHI=0.65\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "BSIM3",
            "* BSIM3v3\n\
             vdd d 0 dc 1.8\n\
             vg g 0 dc 1.2\n\
             m1 d g 0 0 n018 w=1u l=0.18u\n\
             .model n018 nmos level=49\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "BSIM4",
            "* BSIM4v4.8\n\
             vdd d 0 dc 1.0\n\
             vg g 0 dc 0.8\n\
             m1 d g 0 0 n45 w=1u l=45n\n\
             .model n45 nmos level=54 version=4.8\n\
             .op\n.end\n"
                .to_owned(),
        ),
        ("B3SOIPD", b3soi_deck(0)),
        ("B3SOIDD", b3soi_deck(1)),
        ("B3SOIFD", b3soi_deck(2)),
        (
            "EKV26",
            "* EKV 2.6\n\
             vdd d 0 dc 1.8\n\
             vg g 0 dc 1.2\n\
             m1 d g 0 0 ekvm w=10u l=1u\n\
             .model ekvm nmos level=260\n\
             .op\n.end\n"
                .to_owned(),
        ),
        ("EKV3", ekv3_deck()),
        (
            "VDMOS",
            "* power VDMOS\n\
             vd d 0 dc 10\n\
             vg g 0 dc 5\n\
             m1 d g 0 0 irfmod W=0.386 L=2.5u\n\
             .MODEL irfmod NMOS LEVEL=18 VTO=3.5 RS=0.005 M=3\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "BJT",
            "* bipolar\n\
             vcc c 0 dc 10\n\
             vbb bdrv 0 dc 2\n\
             rb bdrv b 100k\n\
             rc c col 1k\n\
             q1 col b 0 qmod\n\
             .model qmod NPN (IS=1e-15 BF=100)\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "DIODE",
            "* junction diode\n\
             v1 in 0 dc 5\n\
             r1 in a 1k\n\
             d1 a 0 dmod\n\
             .model dmod D IS=1e-14 N=1.5\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "RESISTOR",
            "* Xyce LEVEL=2 self-consistent thermal resistor\n\
             v1 in 0 dc 1\n\
             r1 in out rmod l=1u a=1u\n\
             rload out 0 1k\n\
             .model rmod R (LEVEL=2 RESISTIVITY=1 HEATCAPACITY=1)\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "CAPACITOR",
            "* solution-dependent capacitance\n\
             vctrl ctrl 0 dc 0.5\n\
             v1 a 0 dc 1\n\
             r1 a b 1k\n\
             rb b 0 1k\n\
             c1 b 0 C={1p*(1+V(ctrl))}\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "JFET",
            "* Shichman-Hodges JFET\n\
             vd d 0 dc 5\n\
             vg g 0 dc -0.5\n\
             j1 d g 0 jmod\n\
             .model jmod NJF (VTO=-2 BETA=1e-4)\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "JFET2",
            "* Parker-Skellern JFET2\n\
             vd d 0 dc 5\n\
             vg g 0 dc -0.25\n\
             j1 d g 0 psmod area=1\n\
             .model psmod NJF (level=2 beta=1e-3 vt0=-2 lambda=0.02 vbi=1 is=1e-14 n=1)\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "MESFET",
            "* Berkeley legacy MESFET\n\
             vd d 0 dc 3\n\
             vg g 0 dc -0.5\n\
             z1 d g 0 zmod\n\
             .model zmod NMF\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "HFET1",
            "* HFET1 heterostructure FET\n\
             vd d 0 dc 1.0\n\
             vg g 0 dc 0.3\n\
             z1 d g 0 hmod L=1u W=10u\n\
             .model hmod nhfet\n\
             .op\n.end\n"
                .to_owned(),
        ),
        (
            "HFET2",
            "* HFET2 (MESA level 6)\n\
             vd d 0 dc 1.0\n\
             vg g 0 dc 0.3\n\
             z1 d g 0 hmod L=1u W=10u\n\
             .model hmod nhfet level=6 rd=60 rs=60 m=2.57 lambda=0.17\n\
             + vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04\n\
             + vsigma=0.1 vsigmat=0.3 nmax=6e15 d1=0.03e-6 d2=0.2e-6\n\
             + di=0.04e-6 delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0\n\
             .op\n.end\n"
                .to_owned(),
        ),
    ]
}

/// The EKV3 slice only accepts a complete NMOS150 card, so the deck carries
/// the same one the native EKV3 oracle uses.
fn ekv3_deck() -> String {
    let model = "\
.MODEL NMOS150 NMOS
+ LEVEL=301
+ SIGN = 1 TG = -1
+ SCALE = 1.0 XL = 0.0 XW = 0.0
+ COX = 8.58E-3 GAMMAG = 18.4 AQMA = 0.0 AQMI = 0.0 ETAQM = 0.75
+ VTO = 400.0E-3 PHIF = 450.0E-3 GAMMA = 300.0E-3 XJ = 30.0E-9 N0 = 1.025
+ KP = 390.0E-6 E0 = 438.0E+6 E1 = 159.0E+6 ETA = 0.57 ZC = 1.0E-6 THC = 0.0
+ PDITS = 2.58E-6 PDITSD = 0.91 PDITSL = 0.0 FPROUT = 1.85E+6 DDITS = 0.1
+ AVTO = 0.0 AKP = 0.0 AGAMMA = 0.0
+ UCRIT = 5.0E+6 DELTA = 1.5 LAMBDA = 0.5 ACLM = 0.85
+ DL = -16.7E-9 DLC = -23.0E-9 WDL = 0.0 LL = 0.0 LLN = 1.0 DW = -45.3E-9 DWC = 0.0 LDW = 0.0
+ LETA0 = 1.0E+6 LETA = 1.3 LETA2 = 0.0 WETA = 1.0 NCS = 0.5
+ ETAD = 0.75 SIGMAD = 1.0
+ LR = 100E-9 QLR = 580E-6 NLR = 100.0E-3 FLR = 2
+ WR = 80.0E-9 QWR = 500.0E-6 NWR = 12.0E-3
+ RLX = 170.0E-6
+ LOV = 25.0E-9 GAMMAOV = 5.0 VFBOV = 0.0 KJF = 150.0E-12 CJF = 300.0E-3
+ KG = 50.0E-6 XB = 5.5 EB = 21.0E+9 LOVIG = 40.0E-12
+ TNOM = 30.0 TCV = 600.0E-6 BEX = -1.6 TE0EX = -4.15 TE1EX = 0.0 TETA = 2.0E-3
+ UCEX = 1.2 TLAMBDA = 0.15 TCVL = 0.0 TCVW = 0.0 TCVWL = 0.0";
    format!(
        "* EKV3 NMOS150 slice\n\
         {model}\n\
         Vd d 0 DC 1.0\n\
         Vg g 0 DC 1.0\n\
         Vs s 0 DC 0\n\
         Vb b 0 DC 0\n\
         M1 d g s b NMOS150 W=150e-9 L=150e-9 NF=1\n\
         .op\n.end\n"
    )
}

fn b3soi_deck(soimod: u32) -> String {
    format!(
        "* BSIM3SOI soimod={soimod}\n\
         m1 3 2 0 0 nsoi w=4u l=1u\n\
         rsource 1 2 100k\n\
         rload 3 vdd 25k\n\
         vdd1 vdd 0 5\n\
         vin 1 0 1.44\n\
         .model nsoi nmos level=10 soimod={soimod} capmod=2\n\
         .op\n.end\n"
    )
}

/// Every label an operating-point report emits has to be one a frontend can
/// read back after persisting it, and the label vocabulary is the only
/// authority on which those are. The families are driven through real decks so
/// that a family reporting a fresh quantity fails here rather than in a project
/// the user can no longer save.
///
/// Three emitters stay out of this circuit's reach: the Xyce nonlinear magnetic
/// core, which needs a K card naming a core output; the Xyce-dialect JFET2,
/// which needs a dialect the default configuration does not select; and the
/// generated Verilog-A catalog, which is absent unless a model feature is
/// enabled and whose labels resolve out of that catalog rather than out of the
/// fixed vocabulary.
#[test]
fn every_reported_device_family_uses_readable_labels() {
    let mut families_seen = Vec::new();
    for (family, deck) in family_decks() {
        let netlist =
            Netlist::parse(&deck).unwrap_or_else(|error| panic!("{family} deck parses: {error}"));
        let (_, report) = Engine::new(SimulationConfig::default())
            .run_dc_op_with_report(&netlist)
            .unwrap_or_else(|error| panic!("{family} operating point solves: {error}"));

        for entry in &report.entries {
            let resolve = |label: &'static str, role: &str| {
                assert_eq!(
                    rspice_core::circuit::resolve_op_label(label),
                    Some(label),
                    "{family}: {} reports {role} '{label}', which no reader can restore",
                    entry.name
                );
            };
            resolve(entry.device_kind, "family");
            if let Some(region) = entry.region {
                resolve(region, "region");
            }
            for (name, _) in &entry.params {
                resolve(name, "quantity");
            }
            families_seen.push(entry.device_kind);
        }

        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.device_kind == family),
            "the {family} deck did not route to that family; it reported {:?}",
            report
                .entries
                .iter()
                .map(|entry| entry.device_kind)
                .collect::<Vec<_>>()
        );
    }

    families_seen.sort_unstable();
    families_seen.dedup();
    assert!(
        families_seen.len() >= 19,
        "the family sweep lost coverage; it exercised only {families_seen:?}"
    );
}
