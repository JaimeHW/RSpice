//! The operating-point aids run in ngspice's order: when the direct Newton
//! fails, dynamic GMIN stepping goes first and source stepping only after it.

use rspice_core::NoAbort;
use rspice_core::engine::{ContinuationAid, Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use rspice_core::notice::{EngineNotice, NoticeCollector};

/// A 1 mA source into twenty diode-connected level-1 NMOS in series, every
/// bulk on ground. Started from cut-off devices, the direct Newton reaches a
/// singular system; ngspice-46 also fails its direct Newton here and finds
/// the operating point by dynamic GMIN stepping.
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

    let mut direct_only = SimulationConfig::default();
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
    let result = Engine::new(SimulationConfig::default())
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
