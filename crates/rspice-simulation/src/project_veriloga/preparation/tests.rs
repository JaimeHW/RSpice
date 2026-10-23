//! A project runs a Verilog-AMS design on top of the SPICE cells of its deck:
//! the design is compiled against the cells of the deck the run builds, and
//! runs exactly as the same design read from a file by the same deck does.

use super::*;
use rspice_design::project_sources::ProjectSourceOwner;
use std::sync::atomic::{AtomicU64, Ordering};

/// A digital process, a deck `.subckt`, a device of a deck model card and an
/// analog contribution the process switches: the shape of the corpus
/// testbench whose design is on top.
const TOP: &str = "`include \"disciplines.vams\"\n\
                   `timescale 1ns/1ps\n\
                   module top(vin, vout);\n\
                   \x20 inout vin, vout;\n\
                   \x20 electrical vin, vout, m;\n\
                   \x20 electrical gnd;\n\
                   \x20 ground gnd;\n\
                   \x20 reg sel;\n\
                   \x20 initial begin sel = 1'b0; #20 sel = 1'b1; end\n\
                   \x20 half xu(vin, m);\n\
                   \x20 dclamp dl(m, gnd);\n\
                   \x20 analog I(m, vout) <+ V(m, vout) / (sel ? 1.0e3 : 2.0e3);\n\
                   endmodule\n";

/// The deck around the design, which `instance` places.
fn deck(cells: &str, header: &str, instance: &str) -> String {
    format!(
        "* top over SPICE cells\n\
         {cells}\
         .model dclamp d(is=1e-14)\n\
         {header}\n\
         vin in 0 pulse(0 1 0 10n 10n 40n 100n)\n\
         rload out 0 10k\n\
         xtop in out {instance}\n\
         .tran 1n 60n\n\
         .end\n"
    )
}

const HALF: &str = ".subckt half a b\nr1 a b 1k\n.ends\n";

fn project() -> (ProjectId, ProjectSourceRegistry) {
    project_of(TOP)
}

/// A project whose one Verilog-A bundle is the design `top` of `source`.
fn project_of(source: &str) -> (ProjectId, ProjectSourceRegistry) {
    let mut sources = ProjectSourceRegistry::default();
    sources
        .insert_bundle(
            ProjectSourceBundle::try_new(
                ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
                ProjectSourceLanguage::VerilogA,
                "top.va",
                source,
                [],
                [],
            )
            .expect("a valid bundle"),
        )
        .expect("install the design");
    sources
        .mark_validated(ProjectSourceLanguage::VerilogA)
        .expect("the editor validated the design");
    (ProjectId::new(), sources)
}

/// The directive a project deck names the design by, and the name its X-card
/// gives: the module's own, as the code workspace writes it.
fn directive(project_id: ProjectId, sources: &ProjectSourceRegistry) -> (String, String) {
    let bundle = sources.iter_bundles().next().expect("the design's bundle");
    let key = rspice_design::project_sources::project_veriloga_bundle_source_key(
        project_id, bundle, "top",
    )
    .expect("the design's key");
    (
        crate::netlist_preparation::project_veriloga_directive(&key, "top"),
        "top".to_owned(),
    )
}

/// The runtimes a project run of `deck` prepares, carried to its worker.
fn prepared(
    project_id: ProjectId,
    sources: &ProjectSourceRegistry,
    deck: &str,
) -> crate::veriloga::PreparedVerilogARuntimeSet {
    let runtimes = project_veriloga_runtimes_referenced_by(project_id, sources, None, deck)
        .unwrap_or_else(|error| panic!("the project route prepares the design: {error:?}"));
    serde_json::from_str(&serde_json::to_string(&runtimes).expect("encode for the worker"))
        .expect("decode in the worker")
}

fn run(deck: &str) -> Result<rspice_core::engine::TransientResult, rspice_core::SimulationError> {
    rspice_core::Engine::default().run_tran(
        &rspice_core::Netlist::parse(deck).expect("the deck parses"),
        60e-9,
        1e-9,
    )
}

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// The manual deck route: the same design, read from a file.
fn manual_run() -> rspice_core::engine::TransientResult {
    manual_run_of(TOP, "")
}

/// [`manual_run`] of the design `top` of `source`, in a deck whose header
/// also carries `options`.
fn manual_run_of(source: &str, options: &str) -> rspice_core::engine::TransientResult {
    let path = std::env::temp_dir().join(format!(
        "rspice_project_top_{}_{}.vams",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&path, source).expect("write the design");
    let header = format!(
        "{options}.hdl \"{}\" module=top",
        path.display().to_string().replace('\\', "/")
    );
    let result = run(&deck(HALF, &header, "top"));
    let _ = std::fs::remove_file(&path);
    result.expect("the manual deck runs")
}

/// Whether two runs accepted the same time points and solved the same
/// voltages, bit for bit.
fn assert_bit_identical(
    project: &rspice_core::engine::TransientResult,
    manual: &rspice_core::engine::TransientResult,
    route: &str,
) {
    let bits = |values: &[f64]| {
        values
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    };
    assert_eq!(project.node_names, manual.node_names, "{route}");
    assert_eq!(
        bits(&project.time),
        bits(&manual.time),
        "{route}: the same accepted time points"
    );
    for (name, (project, manual)) in project
        .node_names
        .iter()
        .zip(project.voltages.iter().zip(&manual.voltages))
    {
        assert_eq!(
            bits(project),
            bits(manual),
            "{route}: v({name}) differs between the routes"
        );
    }
}

#[test]
fn a_project_runs_its_design_over_the_deck_cells_as_the_manual_deck_does() {
    let (project_id, sources) = project();
    // Compiled against no cells, the design names a module nothing defines.
    let refused = crate::project_veriloga::compile_project_source_bundle_runtime(
        project_id,
        sources.iter_bundles().next().expect("the design's bundle"),
        "top",
        None,
        DigitalDelaySelect::Typ,
    )
    .expect_err("without the deck's cells `half` is undefined");
    assert!(refused.to_string().contains("half"), "{refused}");

    let (header, alias) = directive(project_id, &sources);
    let project_deck = deck(HALF, &header, &alias);
    let runtimes = prepared(project_id, &sources, &project_deck);
    assert_eq!(runtimes.len(), 1);
    let _installed = runtimes.install().expect("install the sealed design");
    let project = run(&project_deck).expect("the project deck runs");
    let manual = manual_run();

    assert_eq!(project.node_names, manual.node_names);
    assert_eq!(
        project
            .time
            .iter()
            .map(|time| time.to_bits())
            .collect::<Vec<_>>(),
        manual
            .time
            .iter()
            .map(|time| time.to_bits())
            .collect::<Vec<_>>(),
        "the same accepted time points"
    );
    for (name, (project, manual)) in project
        .node_names
        .iter()
        .zip(project.voltages.iter().zip(&manual.voltages))
    {
        assert!(
            project
                .iter()
                .zip(manual)
                .all(|(project, manual)| project.to_bits() == manual.to_bits()),
            "v({name}) differs between the routes"
        );
    }
    // The design reached its leaves: the clamp holds m below the drive.
    let out = project
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("node out");
    assert!(project.voltages[out].iter().any(|volts| *volts > 0.01));
}

/// The design is compiled against the cells of the deck that runs it, and
/// the artifact names them: another table is another artifact, and a deck
/// offering other cells never builds one compiled against these.
#[test]
fn a_changed_cell_table_is_another_artifact_that_only_its_deck_builds() {
    let (project_id, sources) = project();
    let (header, alias) = directive(project_id, &sources);
    let first_deck = deck(HALF, &header, &alias);
    let second_deck = deck(
        &format!("{HALF}.subckt spare a b\nr1 a b 2k\n.ends\n"),
        &header,
        &alias,
    );
    let first = prepared(project_id, &sources, &first_deck);
    let second = prepared(project_id, &sources, &second_deck);
    let identity = |set: &crate::veriloga::PreparedVerilogARuntimeSet| {
        set.device_runtimes()
            .map(|runtime| runtime.artifact_digest())
            .collect::<Vec<_>>()
    };
    assert_ne!(
        identity(&first),
        identity(&second),
        "the cell table is part of the artifact's identity"
    );

    // Both may be held at once: they are different keys.
    let _second = second.install().expect("install the second table's design");
    let refused = run(&first_deck).expect_err("the first deck's table is not installed");
    assert!(
        refused
            .to_string()
            .contains("not installed for this execution"),
        "{refused}"
    );
    let _first = first.install().expect("install the first table's design");
    run(&first_deck).expect("the first deck runs its own artifact");
    run(&second_deck).expect("the second deck runs its own artifact");
}

/// A design with no leaf compiles the same against any table and keeps the
/// identity it had before a run offered it one.
#[test]
fn a_design_without_leaves_keeps_its_identity_under_any_table() {
    let mut sources = ProjectSourceRegistry::default();
    sources
        .insert_bundle(
            ProjectSourceBundle::try_new(
                ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
                ProjectSourceLanguage::VerilogA,
                "load.va",
                "`include \"disciplines.vams\"\n\
                 module load(p, n); inout p, n; electrical p, n;\n\
                 analog I(p, n) <+ V(p, n) / 1.0e3;\nendmodule\n",
                [],
                [],
            )
            .expect("a valid bundle"),
        )
        .expect("install the design");
    let project_id = ProjectId::new();
    let bundle = sources.iter_bundles().next().expect("the bundle");
    let bare = crate::project_veriloga::compile_project_source_bundle_runtime(
        project_id,
        bundle,
        "load",
        None,
        DigitalDelaySelect::Typ,
    )
    .expect("compile without cells");
    let offered = crate::project_veriloga::compile_project_source_bundle_runtime(
        project_id,
        bundle,
        "load",
        Some(&deck_external_cells(HALF)),
        DigitalDelaySelect::Typ,
    )
    .expect("compile against the deck's cells");
    assert_eq!(bare, offered);
}

/// [`TOP`] with its switch delayed by a min:typ:max delay (IEEE 1364-2005
/// section 5.3): the conductance changes at 10, 20 or 30 ns.
const DELAYED_TOP: &str = "`include \"disciplines.vams\"\n\
                           `timescale 1ns/1ps\n\
                           module top(vin, vout);\n\
                           \x20 inout vin, vout;\n\
                           \x20 electrical vin, vout, m;\n\
                           \x20 electrical gnd;\n\
                           \x20 ground gnd;\n\
                           \x20 reg sel;\n\
                           \x20 initial begin sel = 1'b0; #(10:20:30) sel = 1'b1; end\n\
                           \x20 half xu(vin, m);\n\
                           \x20 dclamp dl(m, gnd);\n\
                           \x20 analog I(m, vout) <+ V(m, vout) / (sel ? 1.0e3 : 2.0e3);\n\
                           endmodule\n";

/// The project deck of `top` selecting `options`.
fn delayed_project_deck(
    project_id: ProjectId,
    sources: &ProjectSourceRegistry,
    options: &str,
) -> String {
    let (header, alias) = directive(project_id, sources);
    deck(HALF, &format!("{options}{header}"), &alias)
}

/// A deck's `.options hdl_delay_select` reaches a project's design: under
/// min and under max the project route compiles, seals and runs it exactly
/// as the manual deck runs the same design read from a file, bit for bit,
/// and the two selections run differently.
#[test]
fn a_project_runs_its_design_under_the_deck_selection_as_the_manual_deck_does() {
    let (project_id, sources) = project_of(DELAYED_TOP);
    let mut outputs = Vec::new();
    for (options, selection) in [
        (".options hdl_delay_select=min\n", DigitalDelaySelect::Min),
        (".options hdl_delay_select=max\n", DigitalDelaySelect::Max),
        ("", DigitalDelaySelect::Typ),
    ] {
        let project_deck = delayed_project_deck(project_id, &sources, options);
        let runtimes = prepared(project_id, &sources, &project_deck);
        let runtime = runtimes.device_runtimes().next().expect("the design");
        assert_eq!(runtime.delay_select(), selection);
        let _installed = runtimes.install().expect("install the sealed design");
        let project = run(&project_deck)
            .unwrap_or_else(|error| panic!("{selection:?}: the project deck runs: {error}"));
        let manual = manual_run_of(DELAYED_TOP, options);
        assert_bit_identical(&project, &manual, &format!("{selection:?}"));
        outputs.push(project.voltages);
    }
    assert_ne!(outputs[0], outputs[1], "min and max switch at other times");
    assert_ne!(outputs[0], outputs[2], "min and typ switch at other times");
}

/// An artifact sealed under one selection is never run under another: a
/// deck selecting another refuses it by naming both selections, and its own
/// preparation compiles another artifact, which runs beside the first.
#[test]
fn an_artifact_sealed_at_min_is_never_reused_under_another_selection() {
    let (project_id, sources) = project_of(DELAYED_TOP);
    let min_deck = delayed_project_deck(project_id, &sources, ".options hdl_delay_select=min\n");
    let minimum = prepared(project_id, &sources, &min_deck);
    let _minimum = minimum.install().expect("install the minimum design");
    let identity = |set: &crate::veriloga::PreparedVerilogARuntimeSet| {
        set.device_runtimes()
            .map(|runtime| runtime.artifact_digest())
            .collect::<Vec<_>>()
    };
    for (options, keyword) in [("", "typ"), (".options hdl_delay_select=max\n", "max")] {
        let other_deck = delayed_project_deck(project_id, &sources, options);
        let refused = run(&other_deck)
            .expect_err("the minimum artifact is not the other deck's")
            .to_string();
        for expected in [
            "`hdl_delay_select=min`".to_owned(),
            format!("`hdl_delay_select={keyword}`"),
        ] {
            assert!(refused.contains(&expected), "{expected}: {refused}");
        }
        assert!(!refused.contains("not installed"), "{refused}");

        let other = prepared(project_id, &sources, &other_deck);
        assert_ne!(
            identity(&other),
            identity(&minimum),
            "{keyword}: recompiled"
        );
        let _other = other
            .install()
            .expect("install the other selection's design");
        assert_bit_identical(
            &run(&other_deck).expect("the other deck runs its own artifact"),
            &manual_run_of(DELAYED_TOP, options),
            keyword,
        );
        run(&min_deck).expect("the minimum deck still runs its own artifact");
    }
}

/// The editor's compile receipt names the selection it compiled under: a
/// typical receipt serves a typical deck, and goes stale once the deck
/// selects min, whose run compiles the design again under min.
#[test]
fn a_typical_receipt_goes_stale_when_the_deck_selects_min() {
    let (project_id, sources) = project_of(DELAYED_TOP);
    let bundle = sources.iter_bundles().next().expect("the design's bundle");
    let typ_deck = delayed_project_deck(project_id, &sources, "");
    let min_deck = delayed_project_deck(project_id, &sources, ".options hdl_delay_select=min\n");
    let cells = deck_external_cells(&typ_deck);
    assert_eq!(
        cells,
        deck_external_cells(&min_deck),
        "only the selection changed"
    );
    assert_eq!(deck_delay_select(&typ_deck), DigitalDelaySelect::Typ);
    assert_eq!(deck_delay_select(&min_deck), DigitalDelaySelect::Min);

    let (receipt, _) = crate::project_veriloga::receipt::compile_project_bundle_receipt_against(
        project_id,
        bundle,
        None,
        Some(&cells),
        DigitalDelaySelect::Typ,
    )
    .unwrap_or_else(|diagnostics| panic!("the editor compiles the design: {diagnostics:?}"));
    assert_eq!(receipt.delay_select(), DigitalDelaySelect::Typ);
    assert!(receipt.builds_for(&cells, DigitalDelaySelect::Typ));
    assert!(!receipt.builds_for(&cells, DigitalDelaySelect::Min));
    let typical = receipt
        .prepare_runtime(project_id, bundle)
        .expect("the receipt seals");

    let from = |deck: &str| {
        project_veriloga_runtimes_referenced_by(project_id, &sources, Some(&receipt), deck)
            .unwrap_or_else(|error| panic!("the project route prepares the design: {error:?}"))
            .device_runtimes()
            .next()
            .cloned()
            .expect("the design")
    };
    assert_eq!(from(&typ_deck), typical, "the current receipt serves");
    let minimum = from(&min_deck);
    assert_eq!(minimum.delay_select(), DigitalDelaySelect::Min);
    assert_ne!(minimum.artifact_digest(), typical.artifact_digest());
}
