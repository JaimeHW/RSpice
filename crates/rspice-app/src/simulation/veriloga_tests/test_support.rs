//! Shared source fixture for the import, snapshot and worker boundaries.

use super::{
    PreparedVerilogARuntime, PreparedVerilogARuntimeSet, append_project_veriloga_directive,
    build_profile, compile_model_library_source_runtimes, project_veriloga_directive,
};
use crate::state::{
    ProjectSourceBundle, ProjectSourceDependency, ProjectSourceFile, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceRole, ProjectSourceRoleBinding,
};

pub fn standalone_connection_authority()
-> rspice_simulation::model_sources::SealedModelLibraryVerilogAAuthority {
    standalone_connection_authority_with_directive(".va \"rules.vams\" UI_CONNECTIONS\n")
        .expect("one alias per source is a well-defined authority")
}

pub fn standalone_connection_authority_with_directive(
    rules_directive: &str,
) -> Result<rspice_simulation::model_sources::SealedModelLibraryVerilogAAuthority, String> {
    let mut rules = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
        .iter()
        .map(|(_, source)| *source)
        .collect::<String>();
    rules.push_str("\nconnectrules Low; connect d2a #(.vsup(1.0)); endconnectrules\nconnectrules High; connect d2a #(.vsup(5.0)); endconnectrules\n");
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    manager.load_library_bundle(
        "standalone-ui-connections.lib",
        vec![
            ("root.lib".to_owned(), format!(".va \"driver.vams\" UI_DRIVER\n{rules_directive}").into_bytes()),
            ("driver.vams".to_owned(), b"module ui_driver(p,q); inout p; electrical p; output q; reg q; initial q=1; analog I(p)<+0; endmodule\n".to_vec()),
            ("rules.vams".to_owned(), rules.into_bytes()),
        ],
        None,
    ).expect("sealed library import retains both source kinds");
    manager
        .seal_execution_sources()
        .unwrap()
        .reference_model_library_veriloga_authority(crate::product::ProcessCorner::TT)
        .map(|authority| authority.expect("the library carries Verilog-A"))
}

pub fn standalone_connection_fixture() -> (PreparedVerilogARuntimeSet, String) {
    let sources =
        compile_model_library_source_runtimes(&standalone_connection_authority()).unwrap();
    let mut deck = "standalone UI connections\nV1 p 0 1\nX1 p q UI_DRIVER\n.options connectrules=Low connectrules_source=ui_connections\n.tran 0.2n 2n\n.end\n".to_owned();
    for source in sources.sources() {
        assert!(!std::path::Path::new(source.source_key()).exists());
        append_project_veriloga_directive(&mut deck, source.source_key(), source.netlist_alias());
    }
    (sources, deck)
}

const PROJECT_SOURCE: &str = r#"`include "gain.vams"
module project_controlled(p, n);
    inout p, n; electrical p, n;
    reg selected;
    initial begin selected = 1; #1 selected = 0; end
    analog I(p,n) <+ `PROFILE_SCALE * `BASE_GAIN * (1 + selected) * V(p,n);
endmodule
"#;

pub fn project_bundle(profile: &build_profile::VerilogABuildProfile) -> ProjectSourceBundle {
    ProjectSourceBundle::try_new_with_roles(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "model.va",
        PROJECT_SOURCE,
        [
            ProjectSourceFile::try_new("models/gain.vams", "`define BASE_GAIN 0.001\n").unwrap(),
            ProjectSourceFile::try_new("config/build.data", profile.to_toml().unwrap()).unwrap(),
        ],
        [
            ProjectSourceDependency::try_new("model.va", "models/gain.vams").unwrap(),
            ProjectSourceDependency::try_new("model.va", "config/build.data").unwrap(),
        ],
        [ProjectSourceRoleBinding::try_new(
            "config/build.data",
            ProjectSourceRole::VerilogABuildProfile,
        )
        .unwrap()],
    )
    .unwrap()
}

pub fn assert_controlled_conductance(runtime: &PreparedVerilogARuntime, series_resistance: f64) {
    let _installed = super::PreparedVerilogARuntimeSet::try_new(vec![runtime.clone()])
        .unwrap()
        .install()
        .unwrap();
    let deck = rspice_core::Netlist::parse(&format!(
        "profiled mixed device\nV1 input 0 1\nR1 input out {series_resistance}\nX1 out 0 {}\n{}\n.end\n",
        runtime.netlist_alias(), project_veriloga_directive(runtime.source_key(), runtime.netlist_alias()),
    )).unwrap();
    let result = rspice_core::Engine::default()
        .run_tran(&deck, 2e-9, 0.2e-9)
        .unwrap();
    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    let mut checked = [0usize; 2];
    for (&time, &voltage) in result.time.iter().zip(&result.voltages[out]) {
        let (phase, expected) = if time < 0.8e-9 {
            (0, 1.0 / 3.0)
        } else if time > 1.2e-9 {
            (1, 0.5)
        } else {
            continue;
        };
        checked[phase] += 1;
        assert!(
            (voltage - expected).abs() < 1e-9,
            "{} at {time:e}: {voltage}, expected {expected}",
            runtime.source_key()
        );
    }
    assert!(checked.into_iter().all(|count| count > 0));
}
