use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::{Engine, Netlist, ResourceKind, SimulationConfig, SimulationError};
use std::path::PathBuf;

fn candidates(netlist: &Netlist) -> Vec<PathBuf> {
    Engine::default()
        .data_file_candidates_with_abort(netlist, &NoAbort)
        .unwrap()
}

#[test]
fn behavioral_dependencies_expand_functions_and_use_the_readers_path_rules() {
    let root = std::env::temp_dir().join("rspice-behavioral-dependency-paths");
    let netlist = Netlist::parse_with_path(
        "behavioral inputs\n\
         .func wave(x) {tablefile(\"wave.dat\")+x}\n\
         B1 out 0 V={wave(0)}\n\
         R1 out 0 R={1000*fasttablefile(\"resistance.dat\")}\n\
         C1 out 0 {1u*akimafile(\"capacitance.dat\")}\n\
         B2 other 0 V=table(time,0,1,1,1)\n.end\n",
        &root.join("deck.cir"),
    )
    .unwrap();
    assert_eq!(
        candidates(&netlist),
        vec![
            root.join("capacitance.dat"),
            root.join("resistance.dat"),
            root.join("wave.dat")
        ]
    );
}

#[test]
fn active_model_and_instance_paths_use_builder_precedence_without_reading_files() {
    let root = std::env::temp_dir().join("rspice-xspice-dependency-paths");
    let netlist = Netlist::parse_with_path(
        "data dependencies\n\
         .model ignored filesource (file=\"unused.dat\")\n\
         .model chosen filesource (file=\"overridden.dat\")\n\
         A1 out chosen file=\"instance.dat\"\n\
         A2 out file_source file=\"instance.dat\"\n\
         .subckt child out filename=\"default.dat\"\n\
         A3 out local\n\
         .model local filesource (file=filename)\n\
         .ends\n\
         X1 out child filename=\"scoped.dat\"\n.end\n",
        &root.join("deck.cir"),
    )
    .unwrap();
    assert_eq!(
        candidates(&netlist),
        vec![root.join("instance.dat"), root.join("scoped.dat")]
    );
}

#[test]
fn reader_defaults_and_optional_files_are_reported() {
    let netlist = Netlist::parse(
        "defaults\n\
         A1 out filesource\n\
         A2 [d] d_source input_file=\"\"\n\
         A3 [d] clk null [q] d_state state_file=\" \"\n\
         A4 x y out table2d file=\"\"\n\
         A5 x y z out table3d\n\
         A6 in out xfer\n\
         A7 out filesource file=\"\"\n\
         .end\n",
    )
    .unwrap();
    let paths = candidates(&netlist);
    for name in [
        "filesource.txt",
        "source.txt",
        "state.txt",
        "2D-table-model.txt",
        "3D-table-model.txt",
    ] {
        assert!(paths.contains(&PathBuf::from(name)), "{name}: {paths:?}");
    }
    assert!(!paths.contains(&PathBuf::new()));
}

#[test]
fn transfer_trims_paths_but_filesource_preserves_filename_whitespace() {
    let netlist = Netlist::parse(
        "whitespace\n\
         A1 in out xfer file=\" transfer.s1p \"\n\
         A2 out filesource file=\" stimulus.dat \"\n.end\n",
    )
    .unwrap();
    let paths = candidates(&netlist);
    assert!(paths.contains(&PathBuf::from("transfer.s1p")), "{paths:?}");
    assert!(
        paths.contains(&PathBuf::from(" stimulus.dat ")),
        "{paths:?}"
    );
    assert!(!paths.contains(&PathBuf::from(" transfer.s1p ")));
}

#[test]
fn virtual_inputs_have_no_native_candidates() {
    let path = std::env::temp_dir().join(format!(
        "rspice-registered-input-{}.dat",
        std::process::id()
    ));
    let spelling = path.to_string_lossy().replace('\\', "/");
    rspice_core::xspice::register_data_file(&spelling, "0 1\n").unwrap();
    let netlist = Netlist::parse(&format!(
        "virtual\nA1 out filesource file=\"{spelling}\"\nA2 out filesource file=\"virtual://dependency-test/wave\"\n.end\n"
    )).unwrap();
    let result = candidates(&netlist);
    rspice_core::xspice::unregister_data_file(&spelling).unwrap();
    assert!(result.is_empty(), "{result:?}");
}

#[test]
fn discovery_preserves_statistical_stream_and_cancellation_and_hierarchy_bounds() {
    let mut netlist = Netlist::parse(
        "bounded\n\
         .subckt child out\n\
         R1 out 0 {unif(1000,0.1)}\n\
         A1 out filesource file=\"stimulus.dat\"\n\
         .ends\nX1 out child\nX2 out child\n.end\n",
    )
    .unwrap();
    netlist.params.set_random_seed(42);
    // Start in the middle of the stream: discovery must neither consume nor rewind it.
    let random = rspice_core::netlist::expr::RandomState::new(42);
    assert_eq!(
        netlist.params.random().next_uniform(),
        random.next_uniform()
    );
    candidates(&netlist);
    assert_eq!(
        netlist.params.random().next_uniform(),
        random.next_uniform()
    );

    struct Cancel;
    impl AbortSignal for Cancel {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    assert!(matches!(
        Engine::default().data_file_candidates_with_abort(&netlist, &Cancel),
        Err(SimulationError::Aborted)
    ));
    let mut config = SimulationConfig::default();
    config.resource_limits.max_flattened_elements = 1;
    assert!(
        matches!(Engine::new(config).data_file_candidates_with_abort(&netlist, &NoAbort), Err(SimulationError::ResourceLimit(error)) if error.resource == ResourceKind::FlattenedElements)
    );
}
