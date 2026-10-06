use rspice_core::Netlist;
use rspice_core::netlist::{
    ElementKind, SourceSpec, flatten_netlist_with_models, independent_source_file_dependency,
};

#[test]
fn deferred_paths_resolve_without_evaluating_instance_values_or_opening_files() {
    let base = std::env::current_dir()
        .unwrap()
        .join("unopened PWL sources");
    let source = "paths\n.subckt driver p params: gain=2 delay=1n\nV1 p 0 AC 1 PWL(FILE='wave µ.dat' VSCALE={gain} TD={delay}) DISTOF1 1\n.ends\nX1 out driver gain=3\nR1 out 0 1k\n.end\n";
    let parsed = Netlist::parse_with_path(source, &base.join("deck.cir")).unwrap();
    let kind = &parsed.subcircuits[0].elements[0].kind;
    let ElementKind::VoltageSourceDeferred(raw) = kind else {
        panic!("scope must remain deferred: {kind:?}")
    };
    assert!(
        raw.contains("{gain}") && raw.contains("{delay}"),
        "{raw}"
    );
    let expected = base.join("wave µ.dat");
    assert_eq!(
        independent_source_file_dependency(kind).unwrap().as_deref(),
        expected.to_str()
    );
    let flat = flatten_netlist_with_models(&parsed).unwrap();
    let kind = &flat
        .elements
        .iter()
        .find(|element| element.name.ends_with("V1"))
        .unwrap()
        .kind;
    assert_eq!(
        independent_source_file_dependency(kind).unwrap().as_deref(),
        expected.to_str()
    );
    let ElementKind::VoltageSource(SourceSpec::Distortion { inner, .. }) = kind else {
        panic!("{kind:?}")
    };
    let SourceSpec::AcTransient { transient, .. } = inner.as_ref() else {
        panic!("{inner:?}")
    };
    assert!(
        matches!(transient.as_ref(), SourceSpec::PwlFile { value_scale, delay, .. } if *value_scale == 3.0 && (*delay - 1e-9).abs() < 1e-20)
    );
}

#[test]
fn resolving_paths_leaves_other_deferred_waveforms_and_absolute_files_unchanged() {
    let base = std::env::current_dir()
        .unwrap()
        .join("source path fixtures");
    let absolute = base.join("input data.dat");
    let source = format!(
        "paths\n.subckt driver p params: file=1 pwl=2\nV1 p 0 PULSE(0 {{file}} 0 1n 1n 2n 4n)\nI1 p 0 PWL FILE='{}' VSCALE={{pwl}}\n.ends\nX1 out driver\nR1 out 0 1k\n.end\n",
        absolute.display()
    );
    let plain = Netlist::parse(&source).unwrap();
    let with_path = Netlist::parse_with_path(&source, &base.join("deck.cir")).unwrap();
    for (before, after) in plain.subcircuits[0]
        .elements
        .iter()
        .zip(&with_path.subcircuits[0].elements)
    {
        match (&before.kind, &after.kind) {
            (
                ElementKind::VoltageSourceDeferred(before),
                ElementKind::VoltageSourceDeferred(after),
            )
            | (
                ElementKind::CurrentSourceDeferred(before),
                ElementKind::CurrentSourceDeferred(after),
            ) => assert_eq!(before, after),
            pair => panic!("expected deferred sources: {pair:?}"),
        }
    }
}
