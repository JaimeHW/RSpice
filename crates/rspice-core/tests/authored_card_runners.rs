//! Authored `.SENS`, `.PZ` and `.SP` cards run through core entry points that
//! take the card, and their results project into the shared result document.

use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::SensitivityCardResult;
use rspice_core::execution::{
    AnalysisResultDocument, DeckPlan, ResultPayload, result_document::ScalarValue,
};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::resource::ResourceLimits;
use rspice_core::{Engine, Netlist, SimulationConfig};

fn card(netlist: &Netlist, select: fn(&AnalysisCommand) -> bool) -> AnalysisCommand {
    netlist
        .analyses
        .iter()
        .find(|command| select(command))
        .expect("the deck authors the card under test")
        .clone()
}

fn instance(netlist: &Netlist, tag: &str) -> rspice_core::execution::AnalysisInstanceId {
    let plan = DeckPlan::from_netlist(netlist, &ResourceLimits::default()).expect("the deck plans");
    plan.analyses()
        .iter()
        .find(|analysis| analysis.id().tag() == tag)
        .unwrap_or_else(|| panic!("the plan has no {tag}"))
        .id()
}

const DIVIDER: &str = "Resistive divider\n\
     V1 in 0 DC 1 AC 1\n\
     R1 in out 1k\n\
     R2 out 0 1k\n\
     C1 out 0 1n\n\
     .sens V(out)\n\
     .end\n";

#[test]
fn an_authored_sens_card_resolves_its_own_output_node() {
    let netlist = Netlist::parse(DIVIDER).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_sensitivity_from_card_with_abort(
            &netlist,
            &card(&netlist, |command| {
                matches!(command, AnalysisCommand::Sensitivity { .. })
            }),
            &NoAbort,
        )
        .expect("the authored .SENS card runs from its own node names");
    let SensitivityCardResult::Dc(dc) = result else {
        panic!("a .SENS card with no AC clause is a DC study");
    };
    assert!(
        !dc.sensitivities.is_empty(),
        "a divider has sensitive parameters"
    );
    AnalysisResultDocument::from_sensitivity(instance(&netlist, "sens-001"), &dc)
        .expect("the shared document accepts the runner's result")
        .build()
        .expect("document builds");
}

#[test]
fn an_authored_sens_ac_card_selects_the_ac_driver_and_publishes_a_document() {
    let source = DIVIDER.replace(".sens V(out)", ".sens V(out) AC DEC 2 1k 10k");
    let netlist = Netlist::parse(&source).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_sensitivity_from_card_with_abort(
            &netlist,
            &card(&netlist, |command| {
                matches!(command, AnalysisCommand::Sensitivity { .. })
            }),
            &NoAbort,
        )
        .expect("the authored .SENS AC card runs");
    let SensitivityCardResult::Ac(ac) = result else {
        panic!("a .SENS card with an AC clause is a frequency-domain study");
    };
    assert!(!ac.frequencies.is_empty());
    let document = AnalysisResultDocument::from_ac_sensitivity(instance(&netlist, "sens-001"), &ac)
        .expect("AC sensitivity now has a document builder")
        .build()
        .expect("document builds");
    assert_eq!(document.point_count(), ac.frequencies.len());
    let ResultPayload::Sensitivity(payload) = document.payload() else {
        panic!("a .SENS card projects a sensitivity payload");
    };
    assert!(
        payload.entries.is_empty() && !payload.ac_entries.is_empty(),
        "an AC study populates the AC traces and nothing else"
    );
    for entry in &payload.ac_entries {
        assert_eq!(entry.absolute.len(), ac.frequencies.len());
        assert_eq!(entry.phase.len(), ac.frequencies.len());
    }
}

#[test]
fn an_authored_pz_card_resolves_all_four_of_its_ports() {
    let source = DIVIDER.replace(".sens V(out)", ".pz in 0 out 0 vol pz");
    let netlist = Netlist::parse(&source).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_pz_from_card_with_abort(
            &netlist,
            &card(&netlist, |command| {
                matches!(command, AnalysisCommand::PoleZero { .. })
            }),
            &NoAbort,
        )
        .expect("the authored .PZ card runs from its own node names");
    assert!(
        !result.poles.is_empty(),
        "an RC divider has at least one pole"
    );
    AnalysisResultDocument::from_pole_zero(instance(&netlist, "pz-001"), &result)
        .expect("the shared document accepts the runner's result")
        .build()
        .expect("document builds");
}

#[test]
fn an_authored_pz_card_naming_an_absent_node_fails_before_any_solve() {
    let source = DIVIDER.replace(".sens V(out)", ".pz in 0 nowhere 0 vol pz");
    let netlist = Netlist::parse(&source).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let error = engine
        .run_pz_from_card_with_abort(
            &netlist,
            &card(&netlist, |command| {
                matches!(command, AnalysisCommand::PoleZero { .. })
            }),
            &NoAbort,
        )
        .expect_err("an absent port node must fail closed");
    let message = error.to_string();
    assert!(
        message.to_ascii_lowercase().contains("nowhere") && message.contains(".PZ output"),
        "the failure names the port and the node it could not resolve: {message}"
    );
}

const TWO_PORT: &str = "Two-port pad\n\
     V1 p1 0 AC 1 portnum=1 z0=50\n\
     V2 p2 0 AC 0 portnum=2 z0=50\n\
     R1 p1 mid 25\n\
     R2 mid 0 50\n\
     R3 mid p2 25\n\
     .sp lin 3 1meg 3meg 1\n\
     .end\n";

#[test]
fn an_authored_sp_card_publishes_the_shared_sp_and_port_noise_documents() {
    let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let sp = card(&netlist, |command| {
        matches!(command, AnalysisCommand::Sp { .. })
    });
    let run = engine
        .run_sp_with_abort(&netlist, &sp, &NoAbort)
        .expect(".SP runs");
    let id = instance(&netlist, "sp-001");

    let scattering = AnalysisResultDocument::from_s_parameters(id, &run.scattering)
        .expect("the shared S-parameter document accepts exactly what the runner returns")
        .build()
        .expect("document builds");
    assert_eq!(scattering.point_count(), run.scattering.data.len());
    let ResultPayload::Sp(payload) = scattering.payload() else {
        panic!("a .SP card projects an S-parameter payload");
    };
    assert_eq!(payload.ports.len(), 2);

    let noise = run.port_noise.expect("the card requested port noise");
    let port_noise = AnalysisResultDocument::from_port_noise(id, &noise)
        .expect("the shared port-noise document accepts exactly what the runner returns")
        .build()
        .expect("document builds");
    assert_eq!(port_noise.point_count(), noise.points.len());
}

#[test]
fn a_stability_document_from_an_uncrossed_loop_records_the_absent_crossover() {
    // A single-pole loop below unity gain never crosses 0 dB, so its Tian
    // margins are unbounded. The projection must publish that determination
    // rather than fail the whole run closed.
    let netlist = Netlist::parse(
        "Uncrossed loop\n\
         V1 in 0 AC 1\n\
         Vprobe mid fb DC 0\n\
         R1 in mid 1k\n\
         R2 mid 0 1k\n\
         E1 fb 0 mid 0 0.1\n\
         .stb dec 4 1 1k probe=Vprobe\n\
         .end\n",
    )
    .expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let AnalysisCommand::Stb {
        variation,
        points,
        start_freq,
        stop_freq,
        probe,
    } = card(&netlist, |command| {
        matches!(command, AnalysisCommand::Stb { .. })
    })
    else {
        panic!("the deck authors a .STB card");
    };
    let config = rspice_core::analysis::StbConfig::new()
        .with_sweep(start_freq, stop_freq, points)
        .with_sweep_type(match variation {
            rspice_core::netlist::FreqVariation::Lin => rspice_core::analysis::StbSweepType::Linear,
            rspice_core::netlist::FreqVariation::Dec => rspice_core::analysis::StbSweepType::Decade,
            rspice_core::netlist::FreqVariation::Oct => rspice_core::analysis::StbSweepType::Octave,
        })
        .with_probe(&probe);
    let result = engine
        .run_stb_with_abort(&netlist, config, &NoAbort)
        .expect(".STB runs");
    let document =
        AnalysisResultDocument::from_stability(instance(&netlist, "stb-001"), &result.result)
            .expect("an unconditionally stable loop must publish")
            .build()
            .expect("document builds");
    let margin = document
        .scalars()
        .iter()
        .find(|scalar| scalar.name() == "gain_margin_db")
        .expect("the document reports a gain margin");
    assert!(
        matches!(
            margin.value(),
            ScalarValue::Real { .. } | ScalarValue::Unavailable { .. }
        ),
        "a margin is either a number or a typed determination"
    );
}
