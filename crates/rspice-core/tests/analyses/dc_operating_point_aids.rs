//! The operating-point aids run in ngspice's order: when the direct Newton
//! fails, dynamic GMIN stepping goes first and source stepping only after it.

use rspice_core::NoAbort;
use rspice_core::engine::{ContinuationAid, Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;
use rspice_core::notice::{EngineNotice, NoticeCollector};

/// A 1 mA source into twenty diode-connected level-1 NMOS in series, every
/// bulk on ground. Started from cut-off devices, the direct Newton reaches a
/// singular system; ngspice-46 also fails its direct Newton here and finds
/// the operating point by dynamic GMIN stepping. QSPICE and LTspice shunt
/// each channel with GMIN and solve it by the direct Newton, as the default
/// dialect does, so the deck runs in the ngspice dialect.
fn stack_deck() -> String {
    let mut deck = String::from(
        "twenty diode-connected NMOS fed by a current source\n\
         .model nch nmos level=1 vto=0.7 kp=110u lambda=0.04 gamma=0.4 phi=0.7\n\
         I1 0 n0 1m\n",
    );
    for device in 0..20 {
        let below = if device == 19 {
            "0".to_owned()
        } else {
            format!("n{}", device + 1)
        };
        deck.push_str(&format!(
            "M{device} n{device} n{device} {below} 0 nch w=10u l=1u\n"
        ));
    }
    deck.push_str(".op\n.end\n");
    deck
}

/// ngspice-46's operating point of [`stack_deck`], `n0` to `n19`.
const NGSPICE_STACK_VOLTAGES: [f64; 20] = [
    73.32823444021408,
    68.40521921706993,
    63.59757250827586,
    58.90671484280629,
    54.3341979501464,
    49.88172538647819,
    45.55117795521992,
    41.34464545103654,
    37.26446689705192,
    33.31328242877171,
    29.49410153635087,
    25.81039493451317,
    22.26622170798406,
    18.86641126099537,
    15.61683467303629,
    12.52483116115745,
    9.599926050650167,
    6.8551597385935,
    4.309918236972723,
    1.997556138454433,
];

#[test]
fn the_direct_newton_fails_and_gmin_stepping_finds_the_operating_point_before_source_stepping() {
    let netlist = Netlist::parse(&stack_deck()).expect("the deck parses");

    let ngspice = || SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    let mut direct_only = ngspice();
    let aids = &mut direct_only.convergence_config;
    aids.gmin_stepping = false;
    aids.source_stepping = false;
    aids.pseudo_transient = false;
    aids.arc_length = false;
    assert!(
        Engine::new(direct_only)
            .run_dc_op_with_abort(&netlist, &NoAbort)
            .is_err(),
        "the direct Newton alone must not find this operating point"
    );

    let collector = NoticeCollector::new();
    let _installed = collector.install();
    let result = Engine::new(ngspice())
        .run_dc_op_with_abort(&netlist, &NoAbort)
        .expect("the convergence aids find the operating point");
    for (node, &expected) in NGSPICE_STACK_VOLTAGES.iter().enumerate() {
        let name = format!("n{node}");
        let actual = result
            .try_voltage_named(&name)
            .unwrap_or_else(|| panic!("missing voltage for {name}"));
        let tolerance = 1e-3 * actual.abs().max(expected.abs()) + 1e-6;
        assert!(
            (actual - expected).abs() <= tolerance,
            "V({name}) = {actual}, ngspice-46 {expected}"
        );
    }

    let aids: Vec<ContinuationAid> = collector
        .take()
        .into_iter()
        .filter_map(|recorded| match recorded.notice {
            EngineNotice::ConvergenceAid(aid) => Some(aid),
            _ => None,
        })
        .collect();
    assert_eq!(
        aids,
        [ContinuationAid::GminStepping],
        "GMIN stepping answers before source stepping runs"
    );
}

/// A diode clamp the direct Newton solves on its own.
const DIODE_CLAMP: &str = "diode clamp\n\
    v1 in 0 5\n\
    r1 in out 1k\n\
    d1 out 0 dmod\n\
    .model dmod d is=1e-14\n";

/// Xyce's `.OPTIONS NONLIN CONTINUATION` names the homotopy that runs in
/// place of its direct Newton solve, and every mode ends on the circuit as
/// written. Each reaches the default solve's operating point here, through
/// the aid the mode names; `MOS` has no rung of its own and solves as the
/// default does.
#[test]
fn xyce_continuation_modes_reach_the_operating_point_through_the_aid_they_name() {
    let solve = |options: &str| {
        let deck = format!("{DIODE_CLAMP}{options}\n.op\n.end\n");
        let netlist = Netlist::parse(&deck).expect("the deck parses");
        let collector = NoticeCollector::new();
        let _installed = collector.install();
        let result = Engine::new(SimulationConfig::default())
            .run_dc_op_with_abort(&netlist, &NoAbort)
            .unwrap_or_else(|error| panic!("{options}: {error}"));
        let aids: Vec<ContinuationAid> = collector
            .take()
            .into_iter()
            .filter_map(|recorded| match recorded.notice {
                EngineNotice::ConvergenceAid(aid) => Some(aid),
                _ => None,
            })
            .collect();
        (result.try_voltage_named("out").expect("V(out)"), aids)
    };
    let (reference, aids) = solve("");
    assert!(aids.is_empty(), "the direct Newton answers: {aids:?}");
    for (options, aid) in [
        (
            ".options nonlin continuation=gmin",
            Some(ContinuationAid::GminStepping),
        ),
        (
            ".options nonlin continuation=3",
            Some(ContinuationAid::GminStepping),
        ),
        (
            ".options nonlin continuation=1\n.options loca conparam=GSTEPPING",
            Some(ContinuationAid::GminStepping),
        ),
        (
            ".options nonlin continuation=pseudo",
            Some(ContinuationAid::PseudoTransient),
        ),
        (
            ".options nonlin continuation=sourcestep2",
            Some(ContinuationAid::SourceStepping),
        ),
        (".options nonlin continuation=mos", None),
    ] {
        let (voltage, aids) = solve(options);
        assert_eq!(aids, aid.into_iter().collect::<Vec<_>>(), "{options}");
        assert!(
            (voltage - reference).abs() <= 1e-6 * reference.abs() + 1e-9,
            "{options}: V(out) = {voltage}, default solve {reference}"
        );
    }
}

/// 1 mOhm links from `y` to `dp` and `dn`, and 1 TOhm legs from `dp` to an
/// 11.5 mV supply and from `dn` to ground: the legs alone set the cluster,
/// at half the supply. Summed into its node's 1000 S diagonal, each leg's
/// 1e-12 S rounds to 1.023e-12 S, and the rounded system's own solution is
/// 2.3% low.
const MILLIOHM_DIVIDER: &str = "1 mOhm links between two 1 TOhm divider legs\n\
    vdd dd 0 dc 0.0115\n\
    r1 y dp 1m\n\
    r2 y dn 1m\n\
    r3 dp dd 1e12\n\
    r4 dn 0 1e12\n\
    .op\n\
    .end\n";

/// One BSIM3 inverter at an 11.5 mV supply, 1 mOhm in series with every
/// drain and source, its input held 19.2 mV above its output. The channels
/// carry about 1e-28 A, so the drain junctions' GMIN sets the output cluster
/// at half the supply. LTspice 26.1.1 reaches it by direct Newton
/// (5.750000000000068 mV), as do Xyce 7.10 (5.749985 mV) and QSPICE's 80-bit
/// build (5.749973 mV); QSPICE's double build and ngspice-46 settle 4% low.
const MILLIOHM_INVERTER: &str = "inverter with 1 mOhm drain and source resistors\n\
    .model nch nmos level=49\n\
    .model pch pmos level=49\n\
    vdd dd 0 dc 0.0115\n\
    vin in out dc 0.0192\n\
    rdp out dp 1m\n\
    rsp dd sp 1m\n\
    mp dp in sp dd pch w=10u l=0.15u\n\
    rdn out dn 1m\n\
    rsn 0 sn 1m\n\
    mn dn in sn 0 nch w=4u l=0.15u\n\
    .op\n\
    .end\n";

#[test]
fn a_milliohm_cluster_holds_the_picosiemens_that_set_it_by_direct_newton() {
    use rspice_core::engine::SpiceDialect;

    let half_supply = 0.5 * 0.0115;
    // The Xyce dialect returns its backend's own answer, as Xyce does, and
    // keeps a nodal GMIN floor at the operating point that Xyce does not:
    // 1e-15 S on each cluster node moves these clusters by 0.15%.
    for dialect in [SpiceDialect::BestAvailable, SpiceDialect::Ngspice] {
        let mut direct_only = SimulationConfig::default().with_spice_dialect(dialect);
        let aids = &mut direct_only.convergence_config;
        aids.gmin_stepping = false;
        aids.source_stepping = false;
        aids.pseudo_transient = false;
        aids.arc_length = false;
        for (deck, nodes) in [
            (MILLIOHM_DIVIDER, ["y", "dp", "dn"]),
            (MILLIOHM_INVERTER, ["out", "dp", "dn"]),
        ] {
            let netlist = Netlist::parse(deck).expect("the deck parses");
            let result = Engine::new(direct_only.clone())
                .run_dc_op_with_abort(&netlist, &NoAbort)
                .unwrap_or_else(|error| panic!("{dialect:?}, {deck}: {error}"));
            for node in nodes {
                let voltage = result
                    .try_voltage_named(node)
                    .unwrap_or_else(|| panic!("missing V({node})"));
                assert!(
                    (voltage - half_supply).abs() <= 1e-12 * half_supply,
                    "{dialect:?}, {deck}: V({node}) = {voltage:.12e}, expected {half_supply:.12e}"
                );
            }
        }
    }
}
