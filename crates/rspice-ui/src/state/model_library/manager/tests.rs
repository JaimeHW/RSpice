//! Tests for library and pack search.
//!
//! Search must be bounded, must ignore an empty query rather than returning
//! everything, and must carry each hit's redistribution status. A missing
//! pack tree is a normal absence, not an error.

use super::*;
use crate::state::model_library::{
    CorrelationDatasetClass, CorrelationDatasetRevision, CorrelationSimulationProvenance,
    CorrelationSuite,
};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn model_fixture() -> (std::path::PathBuf, std::path::PathBuf) {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    // `load_library_file` records the canonical path, and the authenticated
    // closure is keyed on it. A fixture rooted at an aliased temporary
    // directory would hand out sibling paths that never match those keys.
    let directory = crate::fixture_root::canonical_temp_dir().join(format!(
        "rspice-model-manager-{}-{unique}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).expect("create model fixture directory");
    let path = directory.join("foundry.lib");
    fs::write(
            &path,
            ".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n",
        )
        .expect("write model fixture");
    (directory, path)
}

#[test]
fn changing_corner_rebuilds_the_effective_model_catalog() {
    let mut manager = ModelLibraryManager::new();
    let library_name = manager
        .load_library_bytes(
            "section-projection.lib",
            b".lib TT\n.model shared NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n.lib SS\n.model shared NMOS (LEVEL=1 KP=2e-3)\n.model ss_only NMOS (LEVEL=1 KP=3e-3)\n.endl SS\n"
                .to_vec(),
            None,
        )
        .expect("sectioned library imports");
    let library = manager
        .get_library_mut(&library_name)
        .expect("imported library");

    let tt = library.models.get("shared").expect("TT shared model");
    assert_eq!(tt.section.as_deref(), Some("TT"));
    assert_eq!(tt.parameters.get("kp"), Some(&1.0e-3));
    assert!(!library.models.contains_key("ss_only"));
    assert!(library.select_corner("SS"));

    let ss = library.models.get("shared").expect("SS shared model");
    assert_eq!(ss.section.as_deref(), Some("SS"));
    assert_eq!(ss.parameters.get("kp"), Some(&2.0e-3));
    assert_eq!(
        library
            .models
            .get("ss_only")
            .and_then(|model| model.parameters.get("kp")),
        Some(&3.0e-3)
    );
}

#[test]
fn byte_backed_import_retains_exact_execution_authority() {
    let bytes = b".model nch NMOS (LEVEL=1 KP=1e-3)\n".to_vec();
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bytes("browser-models.lib", bytes.clone(), None)
        .expect("self-contained byte source imports");
    let library = manager.get_library(&name).expect("library retained");
    assert_eq!(library.source_closure.len(), 1);
    assert_eq!(library.source_contents.len(), 1);
    assert_eq!(library.source_contents[0].bytes, bytes);
    let binding = crate::state::ProjectTechnologyBinding::from_model_library(library)
        .expect("byte-backed library is attachable");
    manager
        .validate_attached_technology(Some(&binding))
        .expect("unchanged byte-backed catalog matches attachment");
}

#[test]
fn browser_bundle_retains_and_executes_the_complete_sibling_dependency_closure() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bundle(
            "browser-bundle.lib",
            vec![
                (
                    "root.lib".to_owned(),
                    b".include \"device.inc\"\n.lib TT\n.model root_n NMOS (LEVEL=1)\n.endl TT\n"
                        .to_vec(),
                ),
                (
                    "device.inc".to_owned(),
                    b".model nested_n NMOS (LEVEL=1 KP=7e-3)\n".to_vec(),
                ),
            ],
            Some("TT"),
        )
        .expect("complete sibling bundle imports");
    assert_eq!(name, "root");
    let library = manager.get_library(&name).expect("bundle retained");
    assert_eq!(library.source_closure.len(), 2);
    assert_eq!(library.source_contents.len(), 2);
    assert_eq!(library.source_edges.len(), 1);
    assert!(library.models.contains_key("nested_n"));
    assert_eq!(
        library.models["nested_n"]
            .file_path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str()),
        Some("device.inc")
    );
    manager
        .seal_execution_sources()
        .expect("the retained sibling closure seals without filesystem access");
}

#[test]
fn explicit_browser_root_ignores_unreachable_documents_binaries_and_sources() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bundle_from_root(
            "selected-root.lib",
            "models/root.lib",
            vec![
                (
                    "models/root.lib".to_owned(),
                    b".include \"device.inc\"\n".to_vec(),
                ),
                (
                    "models/device.inc".to_owned(),
                    b".model reachable_d D (IS=2e-14)\n".to_vec(),
                ),
                ("README.txt".to_owned(), b"Installation notes".to_vec()),
                ("datasheet.pdf".to_owned(), vec![0, 0xff, 0, 0xfe]),
                (
                    "examples/unrelated.lib".to_owned(),
                    b".model unrelated_d D (IS=9e-14)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect("the selected executable closure imports");
    let library = manager.get_library(&name).expect("library retained");
    assert_eq!(library.source_contents.len(), 2);
    assert!(library.models.contains_key("reachable_d"));
    assert!(!library.models.contains_key("unrelated_d"));
    assert!(library.source_contents.iter().all(|source| {
        let path = source.path.to_string_lossy();
        !path.ends_with("README.txt") && !path.ends_with("datasheet.pdf")
    }));
}

#[test]
fn ambiguous_browser_bundle_requires_an_explicit_root() {
    let mut manager = ModelLibraryManager::new();
    let error = manager
        .load_library_bundle(
            "browser-selection",
            vec![
                (
                    "first.lib".to_owned(),
                    b".model first_d D (IS=1e-14)\n".to_vec(),
                ),
                (
                    "second.lib".to_owned(),
                    b".model second_d D (IS=2e-14)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect_err("independent roots cannot be silently joined");
    assert!(
        error.contains("select the entry file explicitly"),
        "{error}"
    );
    assert_eq!(manager.library_count(), 0);
}

#[test]
fn browser_bundle_preserves_nested_source_tree_and_owner_relative_dependencies() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bundle(
            "nested-browser-bundle.lib",
            vec![
                (
                    "corners/tt.lib".to_owned(),
                    b".include \"../models/device.inc\"\n.include \"../shared/device.inc\"\n"
                        .to_vec(),
                ),
                (
                    "models/device.inc".to_owned(),
                    b".model nested_n NMOS (LEVEL=1 KP=7e-3)\n".to_vec(),
                ),
                (
                    "shared/device.inc".to_owned(),
                    b".model nested_d D (IS=2e-14)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect("nested browser source trees import without flattening");
    assert_eq!(name, "tt");
    let library = manager.get_library(&name).expect("bundle retained");
    assert_eq!(library.source_closure.len(), 3);
    assert_eq!(library.source_edges.len(), 2);
    assert!(library.models.contains_key("nested_n"));
    assert!(library.models.contains_key("nested_d"));
    assert!(library.source_closure.iter().any(|pin| {
        pin.path
            .to_string_lossy()
            .replace('\\', "/")
            .ends_with("/models/device.inc")
    }));
    assert!(library.source_closure.iter().any(|pin| {
        pin.path
            .to_string_lossy()
            .replace('\\', "/")
            .ends_with("/shared/device.inc")
    }));
    manager
        .seal_execution_sources()
        .expect("the retained nested closure seals without filesystem access");
}

#[test]
fn browser_bundle_rejects_dependencies_that_escape_the_selected_tree() {
    let mut manager = ModelLibraryManager::new();
    let error = manager
        .load_library_bundle_from_root(
            "escaping-browser-bundle.lib",
            "corners/root.lib",
            vec![
                (
                    "corners/root.lib".to_owned(),
                    b".include \"../../outside.inc\"\n".to_vec(),
                ),
                (
                    "outside.inc".to_owned(),
                    b".model should_not_import D (IS=1e-14)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect_err("a nested dependency cannot traverse above the selected source root");
    assert!(
        error.contains("escapes the selected source tree"),
        "{error}"
    );
    assert_eq!(manager.library_count(), 0);
}

#[test]
fn browser_bundle_rejects_case_colliding_nested_member_identities() {
    let mut manager = ModelLibraryManager::new();
    let error = manager
        .load_library_bundle(
            "ambiguous-browser-bundle.lib",
            vec![
                (
                    "models/Device.inc".to_owned(),
                    b".model first D (IS=1e-14)\n".to_vec(),
                ),
                (
                    "MODELS/device.inc".to_owned(),
                    b".model second D (IS=2e-14)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect_err("portable bundle identities cannot collide by case");
    assert!(error.contains("ignoring case"), "{error}");
    assert_eq!(manager.library_count(), 0);
}

#[test]
fn browser_bundle_resolves_sibling_names_case_insensitively_without_losing_identity() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bundle(
            "case-bundle.lib",
            vec![
                ("ROOT.LIB".to_owned(), b".include \"device.inc\"\n".to_vec()),
                (
                    "Device.INC".to_owned(),
                    b".model nested_d D (IS=2e-14)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect("browser bundles use portable case-insensitive sibling lookup");
    let library = manager.get_library(&name).expect("imported library");
    assert!(
        library
            .source_closure
            .iter()
            .any(|pin| pin.path.ends_with("Device.INC"))
    );
    assert!(library.models.contains_key("nested_d"));
}

#[test]
fn browser_bundle_discovers_native_spectre_include_edges_after_adaptation() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bundle(
            "spectre-bundle.scs",
            vec![
                (
                    "root.scs".to_owned(),
                    b"simulator lang=spectre\ninclude \"device.scs\"\n".to_vec(),
                ),
                (
                    "device.scs".to_owned(),
                    b"simulator lang=spectre\nmodel native_d diode { is=2e-14 }\n".to_vec(),
                ),
            ],
            None,
        )
        .expect("adapted native Spectre includes retain authenticated edges");
    let library = manager.get_library(&name).expect("imported library");
    assert_eq!(library.source_edges.len(), 1);
    assert!(library.models.contains_key("native_d"));
}

#[test]
fn browser_bundle_retains_native_spectre_ahdl_dependency_without_parsing_it_as_spice() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bundle(
            "spectre-ahdl-bundle.scs",
            vec![
                (
                    "root.scs".to_owned(),
                    b"simulator lang=spectre\nahdl_include \"va/device.va\"\nmodel native_d diode is=2e-14\n"
                        .to_vec(),
                ),
                (
                    "va/device.va".to_owned(),
                    b"`include \"../shared/device_params.vh\"\nmodule device(p, n); inout p, n; electrical p, n; parameter real r = `DEVICE_R; analog I(p, n) <+ V(p, n) / r; endmodule\n".to_vec(),
                ),
                (
                    "shared/device_params.vh".to_owned(),
                    b"`define DEVICE_R 1k\n".to_vec(),
                ),
            ],
            None,
        )
        .expect("Spectre AHDL sources remain authenticated compiler inputs");
    let library = manager.get_library(&name).expect("imported library");
    assert_eq!(library.source_closure.len(), 3);
    assert_eq!(library.source_edges.len(), 2);
    assert!(library.models.contains_key("native_d"));
    assert!(
        library.source_edges[0]
            .target
            .to_string_lossy()
            .replace('\\', "/")
            .ends_with("/va/device.va")
    );
    let sealed = manager
        .seal_execution_sources()
        .expect("retained AHDL edge seals as part of the source closure");
    let authority = sealed
        .model_library_veriloga_authority()
        .expect("sealed AHDL authority is valid")
        .expect("AHDL authority is present");
    assert_eq!(authority.roots.len(), 1);
    let runtimes = crate::simulation::veriloga::compile_model_library_source_runtimes(&authority)
        .expect("retained Spectre AHDL compiles through the sealed runtime path");
    assert_eq!(runtimes.len(), 1);
    let runtime = runtimes.iter().next().expect("compiled runtime");
    assert_eq!(runtime.netlist_alias(), "device");
    assert!(
        runtime
            .source_key()
            .starts_with("__rspice_model_library__/")
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_spectre_import_captures_transitive_veriloga_preprocessor_dependencies() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "rspice-native-spectre-ahdl-{}-{unique}",
        std::process::id()
    ));
    let va_dir = directory.join("va");
    let shared_dir = directory.join("shared");
    fs::create_dir_all(&va_dir).expect("create Verilog-A fixture directory");
    fs::create_dir_all(&shared_dir).expect("create Verilog-A include directory");
    let root = directory.join("models.scs");
    fs::write(
        &root,
        "simulator lang=spectre\nahdl_include \"va/device.va\"\nmodel native_d diode is=2e-14\n",
    )
    .expect("write Spectre root");
    fs::write(
        va_dir.join("device.va"),
        "`include \"../shared/value.vh\"\nmodule native_retained(p, n); inout p, n; electrical p, n; analog I(p, n) <+ V(p, n) / `DEVICE_R; endmodule\n",
    )
    .expect("write Verilog-A root");
    fs::write(shared_dir.join("value.vh"), "`define DEVICE_R 1k\n")
        .expect("write nested Verilog-A include");

    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&root, None)
        .expect("native Spectre AHDL closure imports");
    let library = manager.get_library(&name).expect("native library retained");
    assert_eq!(library.source_closure.len(), 3);
    assert_eq!(library.source_edges.len(), 2);
    let sealed = manager
        .seal_execution_sources()
        .expect("native AHDL closure seals");
    let authority = sealed
        .model_library_veriloga_authority()
        .expect("native authority is valid")
        .expect("native authority is present");
    let runtimes = crate::simulation::veriloga::compile_model_library_source_runtimes(&authority)
        .expect("native retained AHDL compiles without reopening host paths");
    assert_eq!(runtimes.len(), 1);

    fs::remove_dir_all(&directory).expect("remove native Spectre AHDL fixture");
}

#[test]
fn browser_bundle_rejects_missing_dependencies_without_partial_publication() {
    let mut manager = ModelLibraryManager::new();
    let error = manager
        .load_library_bundle(
            "incomplete.lib",
            vec![(
                "root.lib".to_owned(),
                b".include missing.inc\n.model unreachable_n NMOS (LEVEL=1)\n".to_vec(),
            )],
            None,
        )
        .expect_err("an incomplete browser closure must fail closed");
    assert!(
        error.contains("missing from the selected browser bundle"),
        "{error}"
    );
    assert_eq!(manager.library_count(), 0);
}

#[test]
fn scs_import_requires_and_executes_only_the_explicit_spice_interoperability_profile() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bytes(
            "interop.scs",
            b"simulator lang=spice\n.lib TT\n.model nch NMOS (LEVEL=1 KP=4e-3)\n.endl TT\n"
                .to_vec(),
            Some("TT"),
        )
        .expect("explicit Spectre SPICE interoperability source imports");
    assert!(
        manager
            .get_library(&name)
            .unwrap()
            .models
            .contains_key("nch")
    );
    let cards = manager
        .seal_execution_sources()
        .expect("qualified interop source seals")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("qualified interop source materializes")
        .model_cards()
        .join("\n");
    assert!(cards.contains(".model nch"), "{cards}");
    assert!(
        cards.contains("* RSpice spectre-spice/1 presentation directive"),
        "{cards}"
    );
    assert!(!cards.lines().any(|line| line == "simulator lang=spice"));

    let mut missing_boundary = ModelLibraryManager::new();
    let error = missing_boundary
        .load_library_bytes(
            "unqualified.scs",
            b".model nch NMOS (LEVEL=1)\n".to_vec(),
            None,
        )
        .expect_err(".scs without an explicit SPICE boundary fails closed");
    assert!(
        error.contains("requires an explicit simulator lang=spice boundary"),
        "{error}"
    );

    let mut native_spectre = ModelLibraryManager::new();
    let native = native_spectre
        .load_library_bytes(
            "native.scs",
            b"simulator lang=spectre\nsection tt\nmodel native_n nmos { level=1 kp=8e-3 }\nmodel native_b4 bsim4 { type=p vth0=-0.4 }\nendsection tt\n"
                .to_vec(),
            Some("tt"),
        )
        .expect("supported native Spectre model sections adapt explicitly");
    assert!(
        native_spectre
            .get_library(&native)
            .unwrap()
            .models
            .contains_key("native_n")
    );
    let bsim4 = native_spectre
        .get_library(&native)
        .unwrap()
        .models
        .get("native_b4")
        .expect("canonicalized BSIM4 model");
    assert_eq!(bsim4.model_type, ModelType::Pmos);
    assert_eq!(bsim4.spice_level, Some(54));
    assert_eq!(bsim4.level, ModelLevel::Bsim4);
    assert!(!bsim4.string_parameters.contains_key("type"));

    let mut unsupported_native = ModelLibraryManager::new();
    let error = unsupported_native
        .load_library_bytes(
            "native-macro.scs",
            b"simulator lang=spectre\nU1 (a b) unsupported_primitive gain=1\n".to_vec(),
            None,
        )
        .expect_err("unimplemented native Spectre instances fail closed");
    assert!(error.contains("no statement was discarded"), "{error}");
}

#[test]
fn authenticated_root_expands_retained_model_include_closure_without_filesystem_lookup() {
    let (directory, path) = model_fixture();
    let child = directory.join("device.inc");
    fs::write(&child, ".model sealed_n NMOS (LEVEL=1 KP=7e-3)\n")
        .expect("write nested model source");
    fs::write(
        &path,
        ".include device.inc\n.lib TT\n.model root_n NMOS (LEVEL=1)\n.endl TT\n",
    )
    .expect("write model root");

    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("import authenticated model closure");
    let sealed = manager
        .seal_execution_sources()
        .expect("seal exact model bytes");
    let deck = directory.join("browser-root.cir");
    let source = "browser root\n.lib \"foundry.lib\" TT\nM1 d g 0 0 sealed_n\n.end\n";

    let (expanded, dependencies) = sealed
        .expand_root_dependencies(&deck, source, &rspice_core::abort_signal::NoAbort)
        .expect("expand through authenticated bundle");

    assert!(expanded.contains("sealed_n"), "{expanded}");
    assert!(expanded.lines().all(|line| {
        rspice_core::netlist::parse_include_directive(line).is_none()
            && rspice_core::netlist::parse_lib_directive(line)
                .is_none_or(|(_, section)| section.is_none())
    }));
    assert_eq!(dependencies.len(), 2);

    fs::remove_dir_all(directory).expect("remove authenticated expansion fixture");
}

#[test]
fn authenticated_root_rejects_missing_or_tampered_retained_sources() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, None)
        .expect("import authenticated model source");
    manager
        .get_library_mut(&name)
        .expect("library exists")
        .source_contents[0]
        .bytes
        .push(b' ');
    let tamper = manager
        .seal_execution_sources()
        .expect_err("retained byte tamper must fail closed");
    assert!(
        tamper.contains("do not match the accepted digest"),
        "{tamper}"
    );

    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, None)
        .expect("re-import clean source");
    let sealed = manager.seal_execution_sources().expect("seal clean source");
    let missing = sealed
        .bundle_for_root(
            &directory.join("browser-root.cir"),
            "browser root\n.include missing.lib\n.end\n",
        )
        .expect_err("unretained dependency must fail closed");
    assert!(
        missing.contains("not present in the authenticated"),
        "{missing}"
    );

    fs::remove_dir_all(directory).expect("remove authenticated failure fixture");
}

#[test]
fn loaded_sections_resolve_to_exact_reference_and_corner_bindings() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, None)
        .expect("load sectioned model library");

    let reference = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("selected TT binding exists");
    let bindings = manager
        .corner_model_bindings(&[CornerProcess::TT, CornerProcess::FF])
        .expect("TT and FF bindings exist");

    assert_eq!(reference.len(), 1);
    assert!(reference[0].contains("RSpice sealed model source"));
    assert!(reference[0].contains("KP=1e-3"));
    assert!(reference[0].lines().all(|line| {
        rspice_core::netlist::parse_lib_directive(line).is_none()
            && rspice_core::netlist::parse_include_directive(line).is_none()
    }));
    assert_eq!(bindings.len(), 2);
    assert_eq!(bindings[0].process, CornerProcess::TT);
    assert_eq!(bindings[0].section.as_deref(), Some("TT"));
    assert!(bindings[0].materialized_model_cards.contains("KP=1e-3"));
    assert_eq!(bindings[1].process, CornerProcess::FF);
    assert_eq!(bindings[1].section.as_deref(), Some("FF"));
    assert!(bindings[1].materialized_model_cards.contains("KP=2e-3"));

    let error = manager
        .corner_model_bindings(&[CornerProcess::SS])
        .expect_err("undefined SS section must fail closed");
    assert!(
        error.contains("does not define selected corner 'SS'")
            && error.contains("SS reference process"),
        "{error}"
    );
    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn nominal_execution_plan_honors_the_published_library_corner() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, None)
        .expect("load sectioned model library");

    let tt_plan = manager
        .seal_execution_sources()
        .expect("seal TT selection")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("build exact TT execution plan");
    assert_eq!(
        tt_plan.selected_library_corners(),
        &[(name.clone(), Some("TT".to_owned()))]
    );
    assert!(tt_plan.model_cards()[0].contains("KP=1e-3"));

    assert!(
        manager
            .get_library_mut(&name)
            .expect("library remains loaded")
            .select_corner("FF")
    );
    let ff_plan = manager
        .seal_execution_sources()
        .expect("seal FF selection")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("build exact FF execution plan for the nominal run");

    assert_eq!(
        ff_plan.selected_library_corners(),
        &[(name, Some("FF".to_owned()))]
    );
    assert!(ff_plan.model_cards()[0].contains("KP=2e-3"));
    assert_ne!(tt_plan.digest(), ff_plan.digest());

    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn simulation_plan_binding_owns_nominal_section_without_mutating_library() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, None)
        .expect("load sectioned model library");
    assert_eq!(
        manager
            .get_library(&name)
            .unwrap()
            .selected_corner
            .as_deref(),
        Some("TT")
    );
    let mut binding = manager
        .simulation_plan_binding(&name)
        .expect("library can be explicitly attached");
    binding.selected_corner = Some("FF".to_owned());

    let plan = manager
        .seal_execution_sources_for_plan(&[binding])
        .expect("seal the plan-owned selection")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("materialize the plan-owned FF section");

    assert_eq!(
        plan.selected_library_corners(),
        &[(name.clone(), Some("FF".to_owned()))]
    );
    assert!(plan.model_cards()[0].contains("KP=2e-3"));
    assert_eq!(
        manager
            .get_library(&name)
            .unwrap()
            .selected_corner
            .as_deref(),
        Some("TT"),
        "editing a simulation plan must not mutate the project model catalog"
    );
    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn simulation_plan_binding_order_is_preserved_and_stale_digest_is_refused() {
    let mut manager = ModelLibraryManager::new();
    let late = manager
        .load_library_bytes(
            "z-late.lib",
            b".model z_device D (IS=2e-12)\n".to_vec(),
            None,
        )
        .expect("load late library");
    let early = manager
        .load_library_bytes(
            "a-early.lib",
            b".model a_device D (IS=1e-12)\n".to_vec(),
            None,
        )
        .expect("load early library");
    let bindings = vec![
        manager.simulation_plan_binding(&late).unwrap(),
        manager.simulation_plan_binding(&early).unwrap(),
    ];
    let plan = manager
        .seal_execution_sources_for_plan(&bindings)
        .expect("seal explicit precedence")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("materialize explicit precedence");
    let names = plan
        .selected_library_corners()
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, vec![late.as_str(), early.as_str()]);
    assert!(plan.model_cards()[0].contains("z_device"));
    assert!(plan.model_cards()[1].contains("a_device"));

    let mut stale = bindings;
    stale[0].source_digest = crate::product::ContentDigest::from_bytes([0; 32]);
    let error = manager
        .seal_execution_sources_for_plan(&stale)
        .expect_err("a stale plan binding must not redirect to current content");
    assert!(error.contains("accepted source digest changed"), "{error}");
}

#[test]
fn contested_materialized_model_names_fail_closed() {
    let (directory, first) = model_fixture();
    let second = directory.join("alternate.lib");
    fs::write(&first, ".model contested NMOS (LEVEL=1 KP=1e-3)\n")
        .expect("write first duplicate provider");
    fs::write(&second, ".model contested NMOS (LEVEL=1 KP=2e-3)\n")
        .expect("write second duplicate provider");
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&first, None)
        .expect("load first duplicate provider");
    manager
        .load_library_file(&second, None)
        .expect("load second duplicate provider");

    let error = manager
        .seal_execution_sources()
        .expect("seal both authenticated sources")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect_err("a contested executable namespace must fail closed");

    assert!(
        error.contains("Executable model namespace is contested"),
        "{error}"
    );
    assert!(error.contains("'contested'"), "{error}");
    assert!(error.contains("foundry.lib"), "{error}");
    assert!(error.contains("alternate.lib"), "{error}");

    fs::remove_dir_all(directory).expect("remove duplicate-provider fixture");
}

#[test]
fn source_qualified_provider_decision_removes_loser_before_engine_parse() {
    let (directory, first) = model_fixture();
    let second = directory.join("alternate.lib");
    fs::write(&first, ".model contested NMOS (LEVEL=1 KP=1e-3)\n")
        .expect("write selected provider");
    fs::write(&second, ".model contested NMOS (LEVEL=1 KP=2e-3)\n").expect("write losing provider");
    let mut manager = ModelLibraryManager::new();
    let winner = manager
        .load_library_file(&first, None)
        .expect("load selected provider");
    manager
        .load_library_file(&second, None)
        .expect("load losing provider");

    let record = manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "CONTESTED",
            &winner,
            "Device-owner review selected the characterized foundry card.",
        )
        .expect("publish source-qualified provider decision in manager candidate");
    let plan = manager
        .seal_execution_sources()
        .expect("seal exact authenticated providers")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("the explicit provider decision resolves the namespace");

    assert_eq!(plan.applied_resolutions(), &[record]);
    let combined = format!(
        "resolved namespace\n{}\n.end\n",
        plan.model_cards().join("\n")
    );
    let parsed = rspice_core::netlist::parse_netlist(&combined).expect("resolved cards parse");
    let definitions = parsed
        .models
        .iter()
        .filter(|definition| definition.name.eq_ignore_ascii_case("contested"))
        .collect::<Vec<_>>();
    assert_eq!(
        definitions.len(),
        1,
        "the losing card must not reach the engine"
    );
    assert!(plan.model_cards().join("\n").contains("KP=1e-3"));
    assert!(!plan.model_cards().join("\n").contains("KP=2e-3"));

    fs::remove_dir_all(directory).expect("remove resolved-provider fixture");
}

#[test]
fn refreshed_provider_digest_invalidates_persisted_decision() {
    let (directory, first) = model_fixture();
    let second = directory.join("alternate.lib");
    fs::write(&first, ".model contested NMOS (LEVEL=1 KP=1e-3)\n")
        .expect("write selected provider");
    fs::write(&second, ".model contested NMOS (LEVEL=1 KP=2e-3)\n").expect("write losing provider");
    let mut manager = ModelLibraryManager::new();
    let winner = manager
        .load_library_file(&first, None)
        .expect("load selected provider");
    manager
        .load_library_file(&second, None)
        .expect("load losing provider");
    manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "contested",
            &winner,
            "Initial model-owner review.",
        )
        .expect("record provider decision");

    fs::write(&first, ".model contested NMOS (LEVEL=1 KP=3e-3)\n").expect("revise selected source");
    manager
        .load_library_file(&first, None)
        .expect("explicitly refresh changed source");
    let error = manager
        .seal_execution_sources()
        .expect_err("refresh must invalidate the old digest-bound decision");
    assert!(error.contains("provider decision"), "{error}");
    assert!(error.contains("changed digest"), "{error}");

    fs::remove_dir_all(directory).expect("remove stale-provider fixture");
}

#[test]
fn source_qualified_subcircuit_decision_removes_the_entire_losing_body() {
    let mut manager = ModelLibraryManager::new();
    let winner = manager
        .load_library_bytes(
            "approved-subcircuit.lib",
            b".subckt AMP in out\ne1 out 0 in 0 2\n.ends AMP\n".to_vec(),
            None,
        )
        .expect("approved subcircuit imports");
    manager
        .load_library_bytes(
            "alternate-subcircuit.lib",
            b".subckt amp in out\ne1 out 0 in 0 9\n.ends amp\n".to_vec(),
            None,
        )
        .expect("alternate subcircuit imports");
    let record = manager
        .resolve_definition_provider(
            ModelConsumerScope::Subcircuit,
            "amp",
            &winner,
            "Macro-model owner selected the released source.",
        )
        .expect("subcircuit provider decision records");

    let plan = manager
        .seal_execution_sources()
        .expect("subcircuit sources seal")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("subcircuit decision resolves the namespace");
    let cards = plan.model_cards().join("\n");

    assert_eq!(plan.applied_resolutions(), &[record]);
    assert!(cards.contains("in 0 2"));
    assert!(!cards.contains("in 0 9"));
    let source_map = rspice_core::netlist::source_map_for_editor(&format!("resolved\n{cards}\n"));
    assert_eq!(
        source_map
            .subckt_defs
            .iter()
            .filter(|definition| definition.name.eq_ignore_ascii_case("amp"))
            .count(),
        1,
        "the engine-facing namespace must contain one callable AMP body"
    );
}

#[test]
fn durable_validation_receipt_is_bound_to_revision_plan_catalog_and_sources() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, None)
        .expect("load authenticated model fixture");
    let plan = manager
        .seal_execution_sources()
        .expect("seal fixture")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("build validation plan");
    let expected_source_count = manager.model_validation_source_identity().0;
    let receipt = manager
        .issue_model_validation_receipt(
            ObjectRevision::INITIAL,
            plan.digest(),
            None,
            16,
            vec![ModelValidationFinding {
                code: "SPICE_NAMESPACE_COMPILED".to_owned(),
                severity: ModelValidationFindingSeverity::Information,
                message: "The frozen test namespace compiled.".to_owned(),
            }],
        )
        .expect("issue durable validation receipt");
    receipt.verify().expect("receipt self-authenticates");
    assert!(expected_source_count > 0);
    assert_eq!(receipt.source_count, expected_source_count);
    let encoded = serde_json::to_string(&receipt).expect("receipt serializes");
    assert!(
        !encoded.contains("shared.inc") && !encoded.contains("source_digests"),
        "a durable receipt must not duplicate source paths or per-file records"
    );
    manager
        .validate_model_validation_receipt(ObjectRevision::INITIAL, plan.digest(), None, 16)
        .expect("exact inputs retain validation authority");
    let stale = manager
        .validate_model_validation_receipt(
            ObjectRevision::new(2).expect("revision two"),
            plan.digest(),
            None,
            16,
        )
        .expect_err("project revision changes invalidate the receipt");
    assert!(stale.contains("project revision"), "{stale}");

    let mut tampered = serde_json::to_value(&receipt).expect("receipt serializes");
    tampered["validated_at_unix_ms"] = serde_json::json!(receipt.validated_at_unix_ms + 1);
    let tampered: ModelValidationReceipt =
        serde_json::from_value(tampered).expect("tampered shape remains parseable");
    let error = tampered
        .verify()
        .expect_err("payload tampering must invalidate the receipt digest");
    assert!(error.contains("digest"), "{error}");

    fs::remove_dir_all(directory).expect("remove validation receipt fixture");
}

#[test]
fn failed_section_refresh_is_transactional() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, Some("TT"))
        .expect("load TT section");
    let before = manager
        .get_library(&name)
        .expect("loaded library exists")
        .clone();

    let error = manager
        .load_library_file(&path, Some("MISSING"))
        .expect_err("missing section must fail");

    assert!(error.contains("Section 'MISSING' not found"));
    let after = manager.get_library(&name).expect("library remains loaded");
    assert_eq!(after.selected_corner, before.selected_corner);
    assert_eq!(after.models.len(), before.models.len());
    assert_eq!(after.source_closure, before.source_closure);
    assert_eq!(after.source_edges, before.source_edges);
    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn explicit_refresh_atomically_accepts_new_source_closure() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, Some("TT"))
        .expect("load original source");
    let original = manager
        .get_library(&name)
        .expect("original library exists")
        .source_closure
        .clone();
    assert_eq!(original.len(), 1);

    fs::write(
            &path,
            ".lib TT\n.model nch NMOS (LEVEL=1 KP=7e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=8e-3)\n.endl FF\n",
        )
        .expect("replace source content");
    let blocked = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("unaccepted source change must block");
    assert!(blocked.contains("dependency changed at"));

    manager
        .load_library_file(&path, Some("TT"))
        .expect("explicit refresh accepts replacement");
    let refreshed = manager
        .get_library(&name)
        .expect("refreshed library exists")
        .source_closure
        .clone();

    assert_ne!(refreshed, original);
    assert_eq!(
        refreshed[0].digest,
        ModelLibraryManager::calculate_source_digest(&path)
            .expect("current source digest computes")
    );
    manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("refreshed source binds");

    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn transitive_include_change_blocks_until_explicit_refresh() {
    let (directory, path) = model_fixture();
    let dependency = directory.join("device.inc");
    fs::write(&dependency, ".model included_nch NMOS (LEVEL=1 KP=1e-3)\n")
        .expect("write included source");
    fs::write(
        &path,
        ".include \"device.inc\"\n.lib TT\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl TT\n",
    )
    .expect("write root with include");

    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, Some("TT"))
        .expect("load transitive source closure");
    let accepted = manager
        .get_library(&name)
        .expect("library exists")
        .source_closure
        .clone();
    assert_eq!(accepted.len(), 2);
    assert_eq!(
        manager
            .get_library(&name)
            .expect("library exists")
            .source_edges
            .len(),
        1
    );

    fs::write(&dependency, ".model included_nch NMOS (LEVEL=1 KP=9e-3)\n")
        .expect("change only included source");
    let blocked = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("changed transitive dependency must block");
    assert!(blocked.contains("device.inc"));
    assert!(blocked.contains("dependency changed"));

    manager
        .load_library_file(&path, Some("TT"))
        .expect("explicit refresh accepts new dependency closure");
    let refreshed = &manager
        .get_library(&name)
        .expect("refreshed library exists")
        .source_closure;
    assert_ne!(refreshed, &accepted);
    manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("refreshed transitive source binds");

    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn external_lib_section_dependency_is_part_of_the_pinned_closure() {
    let (directory, path) = model_fixture();
    let dependency = directory.join("sectioned models.lib");
    fs::write(
        &dependency,
        ".lib TT\n.model child_nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n",
    )
    .expect("write external library dependency");
    fs::write(&path, ".lib \"sectioned models.lib\" TT\n").expect("write external library wrapper");

    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, Some("TT"))
        .expect("load external library dependency");
    let accepted = &manager
        .get_library(&name)
        .expect("library exists")
        .source_closure;
    assert_eq!(accepted.len(), 2);
    assert!(
        accepted
            .iter()
            .any(|source| source.path.ends_with("sectioned models.lib"))
    );

    fs::write(
        &dependency,
        ".lib TT\n.model child_nch NMOS (LEVEL=1 KP=8e-3)\n.endl TT\n",
    )
    .expect("change external library dependency");
    let blocked = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("changed external .lib dependency must block");
    assert!(blocked.contains("sectioned models.lib"));
    assert!(blocked.contains("dependency changed"));

    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn cyclic_include_is_rejected_with_owning_source_and_no_partial_library() {
    let (directory, path) = model_fixture();
    let dependency = directory.join("cycle.inc");
    fs::write(
        &path,
        ".include \"cycle.inc\"\n.lib TT\n.model nch NMOS (LEVEL=1)\n.endl TT\n",
    )
    .expect("write root cycle member");
    fs::write(&dependency, ".include \"foundry.lib\"\n").expect("write dependency cycle member");

    let mut manager = ModelLibraryManager::new();
    let error = manager
        .load_library_file(&path, Some("TT"))
        .expect_err("cycle must fail closed");

    assert!(error.contains("Cyclic include dependency"));
    assert!(error.contains("cycle.inc:1"));
    assert_eq!(manager.library_count(), 0);

    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn sealed_snapshot_survives_mutation_and_deletion_without_reopening_sources() {
    let (directory, path) = model_fixture();
    let dependency = directory.join("device.inc");
    fs::write(&dependency, ".model sealed_n NMOS (LEVEL=1 KP=1e-3)\n")
        .expect("write sealed dependency");
    fs::write(
        &path,
        ".include \"device.inc\"\n.lib TT\n.model root_n NMOS (LEVEL=1 KP=2e-3)\n.endl TT\n",
    )
    .expect("write sealed root");

    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("load sealed fixture");
    let snapshot = manager
        .seal_execution_sources()
        .expect("authenticate one immutable run snapshot");

    fs::write(&dependency, ".model sealed_n NMOS (LEVEL=1 KP=9e-3)\n")
        .expect("mutate dependency after sealing");
    fs::remove_file(&path).expect("delete root after sealing");

    let cards = snapshot
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("materialization uses only sealed bytes");
    let cards = cards.join("\n");
    assert!(cards.contains("KP=1e-3"), "{cards}");
    assert!(cards.contains("KP=2e-3"), "{cards}");
    assert!(!cards.contains("KP=9e-3"), "{cards}");
    rspice_core::Netlist::parse(&format!("sealed worker deck\n{cards}\n.end\n"))
        .expect("self-contained sealed cards parse without source files");

    let fresh_error = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("a new run snapshot must reject the changed/deleted closure");
    assert!(
        fresh_error.contains("changed") || fresh_error.contains("unavailable"),
        "{fresh_error}"
    );

    fs::remove_dir_all(directory).expect("remove sealed fixture directory");
}

#[test]
fn existing_dependency_without_authenticated_edge_is_rejected() {
    let (directory, path) = model_fixture();
    let dependency = directory.join("device.inc");
    fs::write(&dependency, ".model edge_n NMOS (LEVEL=1)\n").expect("write dependency");
    fs::write(&path, ".include device.inc\n.lib TT\n.endl TT\n").expect("write root");

    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, Some("TT"))
        .expect("load dependency graph");
    manager
        .get_library_mut(&name)
        .expect("library exists")
        .source_edges
        .clear();

    let error = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("filesystem presence must not substitute for a missing edge");
    assert!(
        error.contains("no authenticated resolution edge"),
        "{error}"
    );
    assert!(dependency.is_file(), "dependency remains tempting on disk");

    fs::remove_dir_all(directory).expect("remove edge fixture directory");
}

#[test]
fn disconnected_pinned_member_is_rejected_before_any_filesystem_probe() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, Some("TT"))
        .expect("load source root");
    let orphan = directory.join("must-never-be-probed.inc");
    assert!(!orphan.exists());

    let library = manager.get_library_mut(&name).expect("library exists");
    library.source_closure.push(ModelSourcePin {
        path: orphan.clone(),
        digest: crate::product::ContentDigest::from_bytes([0xa5; 32]),
    });
    library.source_edges.push(ModelSourceEdge {
        owner: orphan.clone(),
        requested_path: "must-never-be-probed.inc".to_owned(),
        target: orphan.clone(),
    });

    let error = manager
        .seal_execution_sources()
        .expect_err("a disconnected authenticated subgraph must fail closed");
    assert!(error.contains("not reachable from root"), "{error}");
    assert!(
        !error.contains("unavailable"),
        "reachability must be checked before filesystem availability: {error}"
    );

    fs::remove_dir_all(directory).expect("remove disconnected fixture directory");
}

#[test]
fn captured_search_precedence_is_frozen_in_the_run_snapshot() {
    let (directory, path) = model_fixture();
    let subdirectory = directory.join("sub");
    fs::create_dir_all(&subdirectory).expect("create search-precedence directory");
    let first = subdirectory.join("first.inc");
    let local = subdirectory.join("shared.inc");
    let fallback = directory.join("shared.inc");
    fs::write(&first, ".incl shared.inc\n").expect("write nested include");
    fs::write(&local, ".model local_n NMOS (LEVEL=1 KP=1e-3)\n").expect("write local winner");
    fs::write(&fallback, ".model fallback_n NMOS (LEVEL=1 KP=9e-3)\n")
        .expect("write top-level fallback");
    fs::write(
        &path,
        ".include sub/first.inc\n.lib TT\n.model root_n NMOS (LEVEL=1)\n.endl TT\n",
    )
    .expect("write root");

    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, Some("TT"))
        .expect("capture local-first resolution");
    let canonical_local = fs::canonicalize(&local).expect("canonical local path");
    assert!(
        manager
            .get_library(&name)
            .expect("library exists")
            .source_edges
            .iter()
            .any(|edge| edge.requested_path == "shared.inc" && edge.target == canonical_local)
    );
    let snapshot = manager
        .seal_execution_sources()
        .expect("seal captured precedence");
    fs::remove_file(&local).expect("remove original local winner");

    let cards = snapshot
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("snapshot retains captured local winner")
        .join("\n");
    assert!(cards.contains("local_n"), "{cards}");
    assert!(!cards.contains("fallback_n"), "{cards}");

    fs::remove_dir_all(directory).expect("remove precedence fixture directory");
}

#[test]
fn raw_byte_digest_and_supported_encoding_decode_share_one_read() {
    let (directory, path) = model_fixture();
    let source = ".lib TT\n.model utf16_n NMOS (LEVEL=1 KP=3e-3)\n.endl TT\n";
    let mut bytes = vec![0xff, 0xfe];
    bytes.extend(
        source
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>(),
    );
    fs::write(&path, &bytes).expect("write UTF-16LE model source");

    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_file(&path, Some("TT"))
        .expect("supported source encoding imports");
    let pin = manager
        .get_library(&name)
        .expect("library exists")
        .source_closure[0]
        .digest;
    assert_eq!(
        pin,
        crate::product::ContentDigest::from_bytes(Sha256::digest(&bytes).into())
    );
    let cards = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("verified raw bytes decode from memory")
        .join("\n");
    assert!(cards.contains("utf16_n"), "{cards}");

    fs::remove_dir_all(directory).expect("remove encoding fixture directory");
}

fn project_definition(vth0: f64, tag: &str) -> ProjectModelDefinition {
    ProjectModelDefinition {
        name: "owned_nch".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Project-owned regression model".to_owned(),
        numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0), ("vth0".to_owned(), vth0)]),
        string_parameters: BTreeMap::from([("revision_tag".to_owned(), tag.to_owned())]),
    }
}

#[test]
fn editable_project_copy_preserves_catalog_metadata_and_rebinds_source_identity() {
    let mut manager = ModelLibraryManager::new();
    let mut source = ModelLibrary::new("foundry reference");
    source.pdk_name = "Example PDK".to_owned();
    source.technology_node = "28nm".to_owned();
    let mut model = DeviceModel::new("nch_reference", ModelType::Nmos);
    model.spice_type = Some("NMOS".to_owned());
    model.spice_level = Some(54);
    model.model_version = Some(4.8);
    model.description = "Authenticated foundry reference".to_owned();
    model.l_min = Some(0.028);
    model.l_max = Some(10.0);
    model.w_min = Some(0.08);
    model.w_max = Some(100.0);
    model.parameters.insert("kp".to_owned(), 0.0012);
    model
        .string_parameters
        .insert("version_tag".to_owned(), "pdk_r7".to_owned());
    source.add_model(model);

    let mut metadata = ModelDefinitionMetadata::default();
    metadata.parameters.push(ParameterDefinition {
        name: "kp".to_owned(),
        data_type: ParameterDataType::Numeric,
        value: ParameterValue::Numeric(FiniteF64::new(0.0012).expect("finite fixture")),
        unit: Some("A/V^2".to_owned()),
        bounds: None,
        source: ParameterSource::Declared {
            source: "foundry reference card".to_owned(),
        },
        description: "Preserved transconductance metadata".to_owned(),
    });
    source
        .model_definition_metadata
        .insert("nch_reference".to_owned(), metadata);
    manager.add_library(source);

    let commit = manager
        .create_editable_project_copy(
            "foundry reference",
            "nch_reference",
            "nch_reference project",
        )
        .expect("catalog model becomes an editable project revision");
    assert!(commit.before.is_none());
    assert!(commit.affects_execution);

    let project = manager
        .get_library("nch_reference project")
        .expect("project copy is retained");
    assert_eq!(project.pdk_name, "Example PDK");
    assert_eq!(project.technology_node, "28nm");
    assert!(project.source_authority.is_project_owned());
    assert!(project.model_qualification.is_empty());
    assert!(project.model_correlation.is_empty());

    let copied = &project.models["nch_reference"];
    assert_eq!(copied.description, "Authenticated foundry reference");
    assert_eq!(copied.spice_level, Some(54));
    assert_eq!(copied.model_version, Some(4.8));
    assert_eq!(copied.l_min, Some(0.028));
    assert_eq!(copied.l_max, Some(10.0));
    assert_eq!(copied.w_min, Some(0.08));
    assert_eq!(copied.w_max, Some(100.0));
    assert_eq!(
        copied
            .string_parameters
            .get("version_tag")
            .map(String::as_str),
        Some("pdk_r7")
    );

    let copied_metadata = &project.model_definition_metadata["nch_reference"];
    assert!(copied_metadata.source_identity.is_some());
    let kp = copied_metadata
        .parameter("kp")
        .expect("typed source parameter metadata is preserved");
    assert_eq!(kp.unit.as_deref(), Some("A/V^2"));
    assert_eq!(kp.description, "Preserved transconductance metadata");
}

#[test]
fn editable_project_copy_rejects_owned_sources_and_name_collisions_atomically() {
    let mut manager = ModelLibraryManager::new();
    manager
        .create_project_model("owned source", &project_definition(0.48, "r1"))
        .expect("owned source fixture");
    let before = manager
        .get_library("owned source")
        .expect("owned source retained")
        .clone();

    let owned_error = manager
        .create_editable_project_copy("owned source", "owned_nch", "owned copy")
        .expect_err("owned sources are already editable");
    assert!(owned_error.contains("already project-owned"));
    assert!(manager.get_library("owned copy").is_none());

    let mut built_in = ModelLibrary::new("built-in source");
    built_in.add_model(DeviceModel::new("copy_nch", ModelType::Nmos));
    manager.add_library(built_in);
    let collision = manager
        .create_editable_project_copy("built-in source", "copy_nch", "owned source")
        .expect_err("target collision fails before mutation");
    assert!(collision.contains("conflicts with existing library"));
    assert_eq!(
        manager.get_library("owned source"),
        Some(&before),
        "failed copy must not disturb the colliding library"
    );
}

fn current_model_source(
    library: &ModelLibrary,
) -> (
    ModelSourceId,
    ObjectRevision,
    ObjectRevision,
    ContentDigest,
    ModelSourceEvidenceBinding,
) {
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: library_revision,
        ..
    } = library.source_authority
    else {
        panic!("fixture model must be project-owned");
    };
    let model = &library.models["owned_nch"];
    let metadata = library.model_definition_metadata["owned_nch"].clone();
    let definition = ProjectModelRevisionDefinition::new(
        ProjectModelDefinition::from_device_model(model),
        metadata,
    );
    let canonical = definition.canonical_source().unwrap();
    let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
    let model_revision = definition
        .project_source_identity()
        .unwrap()
        .expect("project source identity")
        .revision;
    let binding = ModelSourceEvidenceBinding::try_new_project_bound(
        "owned_nch",
        source_id,
        model_digest,
        model_revision,
    )
    .unwrap();
    (
        source_id,
        library_revision,
        model_revision,
        model_digest,
        binding,
    )
}

fn correlation_suite(
    source: ModelSourceEvidenceBinding,
    revision: ObjectRevision,
) -> CorrelationSuite {
    let reference_bytes = b"id,quantity,value,unit\nr1,gain,1,V\n".to_vec();
    let simulation_bytes = b"id,quantity,value,unit\ns1,gain,1,V\n".to_vec();
    let reference = CorrelationDatasetRevision::try_from_csv(
        "reference",
        ObjectRevision::INITIAL,
        "Reference",
        CorrelationDatasetClass::BenchMeasurement,
        "test authority",
        "lot-1",
        "fixture-1",
        "calibration-1",
        "reference.csv",
        reference_bytes,
        None,
    )
    .unwrap();
    let simulation_digest = ContentDigest::from_bytes(Sha256::digest(&simulation_bytes).into());
    let simulation = CorrelationDatasetRevision::try_from_csv_with_provenance(
        "simulation",
        ObjectRevision::INITIAL,
        "Simulation",
        CorrelationDatasetClass::ModelSimulation,
        "RSpice",
        "owned_nch",
        "retained-plan",
        "numeric-contract",
        "simulation.csv",
        simulation_bytes,
        Some(source.clone()),
        Some(CorrelationSimulationProvenance {
            run_id: "run-1".to_owned(),
            run_dataset_id: "dataset-1".to_owned(),
            analysis_id: 1,
            analysis_result_digest: ContentDigest::from_bytes([0x40; 32]),
            plan_id: "plan-1".to_owned(),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: ContentDigest::from_bytes([0x41; 32]),
            source_content_digest: ContentDigest::from_bytes([0x42; 32]),
            task_config_digest: ContentDigest::from_bytes([0x43; 32]),
            execution_target: "Local desktop engine".to_owned(),
            export_digest: simulation_digest,
            model_source: source.clone(),
            executed_at_unix_ms: 1,
        }),
    )
    .unwrap();
    CorrelationSuite::try_new(
        "owned-nch-correlation",
        revision,
        "Owned NCH correlation",
        "model-owner",
        source,
        vec![reference, simulation],
        Vec::new(),
        Vec::new(),
    )
    .unwrap()
}

#[test]
fn project_model_correlation_commit_is_guarded_append_only_and_source_bound() {
    let mut manager = ModelLibraryManager::new();
    let created = manager
        .create_project_model("owned_models", &project_definition(0.48, "r1"))
        .expect("create project model");
    let (source_id, library_revision, model_revision, model_digest, current_source) =
        current_model_source(&created.after);
    let first_suite = correlation_suite(current_source.clone(), ObjectRevision::INITIAL);
    let first_state =
        ModelCorrelationState::try_new(vec![first_suite.clone()], Vec::new()).unwrap();
    manager
        .replace_project_model_correlation(
            "owned_models",
            source_id,
            library_revision,
            model_revision,
            model_digest,
            "owned_nch",
            &first_state,
        )
        .expect("first correlation revision commits without changing model bytes");

    let second_revision = ObjectRevision::INITIAL.next().unwrap();
    let second_suite = correlation_suite(current_source.clone(), second_revision);
    let second_state =
        ModelCorrelationState::try_new(vec![first_suite.clone(), second_suite.clone()], Vec::new())
            .unwrap();
    let appended = manager
        .replace_project_model_correlation(
            "owned_models",
            source_id,
            library_revision,
            model_revision,
            model_digest,
            "owned_nch",
            &second_state,
        )
        .expect("suite history appends atomically");
    assert!(!appended.affects_execution);

    let deleted_history =
        ModelCorrelationState::try_new(vec![second_suite.clone()], Vec::new()).unwrap();
    let error = manager
        .replace_project_model_correlation(
            "owned_models",
            source_id,
            library_revision,
            model_revision,
            model_digest,
            "owned_nch",
            &deleted_history,
        )
        .unwrap_err();
    assert!(error.contains("immutable") && error.contains("cannot be removed"));

    let stale_source = ModelSourceEvidenceBinding::try_new_project_bound(
        "owned_nch",
        source_id,
        ContentDigest::from_bytes([0xee; 32]),
        model_revision,
    )
    .unwrap();
    let third_revision = second_revision.next().unwrap();
    let stale_suite = correlation_suite(stale_source, third_revision);
    let stale_state =
        ModelCorrelationState::try_new(vec![first_suite, second_suite, stale_suite], Vec::new())
            .unwrap();
    let error = manager
        .replace_project_model_correlation(
            "owned_models",
            source_id,
            library_revision,
            model_revision,
            model_digest,
            "owned_nch",
            &stale_state,
        )
        .unwrap_err();
    assert!(error.contains("exact current model source revision"));

    let wrong_library_revision = library_revision.next().unwrap();
    let error = manager
        .replace_project_model_correlation(
            "owned_models",
            source_id,
            wrong_library_revision,
            model_revision,
            model_digest,
            "owned_nch",
            &second_state,
        )
        .unwrap_err();
    assert!(error.contains("changed after correlation review began"));
}

#[test]
fn project_model_create_and_replace_publish_exact_retained_execution_bytes() {
    let mut manager = ModelLibraryManager::new();
    let created = manager
        .create_project_model("owned_models", &project_definition(0.48, "r1"))
        .expect("create project model");
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        digest: first_digest,
    } = created.after.source_authority
    else {
        panic!("created model must be project-owned")
    };
    assert_eq!(revision, ObjectRevision::INITIAL);
    assert_eq!(
        created.after.models["owned_nch"].string_parameters["revision_tag"],
        "r1"
    );

    let sealed = manager
        .seal_execution_sources_with_reader(|path| {
            panic!(
                "project-owned desktop sealing must not read {}",
                path.display()
            )
        })
        .expect("retained project bytes seal");
    assert_eq!(sealed.sources.len(), 1);
    assert!(sealed.sources[0].1.contains("VTH0=0.48"));
    assert!(sealed.sources[0].1.contains("REVISION_TAG=\"r1\""));

    let replaced = manager
        .replace_project_model(
            "owned_models",
            source_id,
            revision,
            &project_definition(0.51, "r2"),
        )
        .expect("replace project model");
    let ModelSourceAuthority::ProjectOwned {
        revision: second_revision,
        digest: second_digest,
        ..
    } = replaced.after.source_authority
    else {
        panic!("replacement must remain project-owned")
    };
    assert_eq!(second_revision.get(), 2);
    assert_ne!(first_digest, second_digest);
    assert_eq!(replaced.after.models["owned_nch"].parameters["vth0"], 0.51);
    assert_eq!(
        replaced.after.models["owned_nch"].string_parameters["revision_tag"],
        "r2"
    );
}

#[test]
fn project_model_replacement_is_guarded_and_atomic() {
    let mut manager = ModelLibraryManager::new();
    let created = manager
        .create_project_model("owned_models", &project_definition(0.48, "r1"))
        .expect("create project model");
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        ..
    } = created.after.source_authority
    else {
        panic!("created model must be project-owned")
    };
    let original = created.after.source_contents[0].bytes.clone();

    let stale = manager
        .replace_project_model(
            "owned_models",
            ModelSourceId::new(),
            revision,
            &project_definition(0.52, "r2"),
        )
        .expect_err("stale identity must fail");
    assert!(stale.contains("changed after this candidate was opened"));
    assert_eq!(
        manager.get_library("owned_models").unwrap().source_contents[0].bytes,
        original
    );

    let no_op = manager
        .replace_project_model(
            "owned_models",
            source_id,
            revision,
            &project_definition(0.48, "r1"),
        )
        .expect_err("unchanged source must not create a revision");
    assert!(no_op.contains("no source changes"));

    let mut invalid = project_definition(f64::NAN, "r2");
    invalid
        .string_parameters
        .insert("VTH0".to_owned(), "duplicate".to_owned());
    let invalid_error = manager
        .replace_project_model("owned_models", source_id, revision, &invalid)
        .expect_err("invalid candidate must fail before publication");
    assert!(
        invalid_error.contains("more than once") || invalid_error.contains("finite"),
        "{invalid_error}"
    );
    assert_eq!(
        manager.get_library("owned_models").unwrap().source_contents[0].bytes,
        original
    );
}

fn sectioned_project_revision(vth0: f64) -> ProjectModelRevisionDefinition {
    let base = project_definition(vth0, "r1");
    let metadata =
        reconcile_project_model_metadata(&base, None).expect("synthesize typed project metadata");
    let mut definition = ProjectModelRevisionDefinition::new(base, metadata);
    definition
        .metadata
        .sections
        .push(crate::state::model_library::ModelSectionDefinition {
            name: "TT".to_owned(),
            parent: None,
            overrides: BTreeMap::from([(
                "vth0".to_owned(),
                ParameterValue::Numeric(FiniteF64::new(0.49).expect("finite fixture")),
            )]),
            model_files: Vec::new(),
            qualification: crate::state::model_library::ModelSectionQualification::Unqualified,
        });
    definition
}

#[test]
fn complete_project_revision_publishes_sections_and_executes_selected_corner() {
    let mut manager = ModelLibraryManager::new();
    let created = manager
        .create_project_model_revision(
            "owned_sections",
            &sectioned_project_revision(0.48),
            &ModelQualificationState::default(),
        )
        .expect("create complete model revision");
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        digest,
    } = created.after.source_authority
    else {
        panic!("complete revision must be project-owned")
    };
    assert_eq!(revision, ObjectRevision::INITIAL);
    assert_eq!(created.after.selected_corner.as_deref(), Some("TT"));
    assert_eq!(created.after.corners.len(), 1);
    let metadata = &created.after.model_definition_metadata["owned_nch"];
    assert_eq!(metadata.sections[0].model_files.len(), 1);
    assert_eq!(metadata.sections[0].model_files[0].revision, 1);
    assert_eq!(
        metadata.sections[0].model_files[0].content_digest,
        digest.to_string()
    );

    let cards = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("materialize retained TT section")
        .join("\n");
    assert!(cards.contains("VTH0=0.49"), "{cards}");

    let mut metadata_only =
        ProjectModelRevisionDefinition::new(project_definition(0.48, "r1"), metadata.clone());
    metadata_only.metadata.parameters[0].unit = Some("dimensionless".to_owned());
    let replaced = manager
        .replace_project_model_revision(
            "owned_sections",
            source_id,
            revision,
            &metadata_only,
            &ModelQualificationState::default(),
        )
        .expect("metadata-only change creates a complete revision");
    assert_eq!(
        replaced.after.project_source_revision(),
        Some(ObjectRevision::new(2).expect("second revision"))
    );
    assert_eq!(
        replaced.after.source_contents[0].bytes,
        created.after.source_contents[0].bytes
    );
    assert_eq!(
        replaced.after.model_definition_metadata["owned_nch"].parameters[0]
            .unit
            .as_deref(),
        Some("dimensionless")
    );
}

#[test]
fn project_model_tamper_fails_before_any_external_read() {
    let mut manager = ModelLibraryManager::new();
    manager
        .create_project_model("owned_models", &project_definition(0.48, "r1"))
        .expect("create project model");
    manager
        .get_library_mut("owned_models")
        .unwrap()
        .source_contents[0]
        .bytes
        .push(b' ');
    let error = manager
        .seal_execution_sources_with_reader(|path| {
            panic!(
                "tampered project source must fail before reading {}",
                path.display()
            )
        })
        .expect_err("tampered retained bytes must fail");
    assert!(
        error.contains("do not match the accepted digest"),
        "{error}"
    );
}

#[cfg(unix)]
#[test]
fn persisted_symlink_edge_survives_alias_removal_after_sealing() {
    use std::os::unix::fs::symlink;

    let (directory, path) = model_fixture();
    let target = directory.join("real.inc");
    let alias = directory.join("alias.inc");
    fs::write(&target, ".model symlink_n NMOS (LEVEL=1)\n").expect("write symlink target");
    symlink(&target, &alias).expect("create symlink alias");
    fs::write(&path, ".include alias.inc\n.lib TT\n.endl TT\n").expect("write symlink root");

    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("capture symlink resolution");
    let snapshot = manager
        .seal_execution_sources()
        .expect("seal symlink target bytes");
    fs::remove_file(&alias).expect("remove alias after sealing");
    let cards = snapshot
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("authenticated edge no longer needs symlink")
        .join("\n");
    assert!(cards.contains("symlink_n"), "{cards}");

    fs::remove_dir_all(directory).expect("remove symlink fixture directory");
}

#[test]
fn configured_source_replacement_rescans_and_rolls_back_every_file_on_error() {
    let old_root = std::env::temp_dir().join(format!("rspice-pdk-old-{}", uuid::Uuid::new_v4()));
    let next_root = std::env::temp_dir().join(format!("rspice-pdk-next-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&old_root).expect("create old PDK root");
    fs::create_dir_all(&next_root).expect("create next PDK root");
    let old_file = old_root.join("accepted.lib");
    fs::write(&old_file, ".model accepted_n NMOS (LEVEL=1 KP=1e-3)\n")
        .expect("write accepted source");

    let mut manager = ModelLibraryManager::new();
    let mut accepted_config = crate::state::pdk_config::PdkConfig::new();
    accepted_config.add_library_path(old_root.to_string_lossy());
    assert_eq!(
        manager
            .replace_from_pdk_config(None, &mut accepted_config)
            .expect("initial configured source applies"),
        1
    );
    let accepted = manager
        .get_library("accepted")
        .expect("accepted library")
        .clone();

    let good_file = next_root.join("candidate.lib");
    fs::write(&good_file, ".model candidate_n NMOS (LEVEL=1 KP=2e-3)\n")
        .expect("write candidate source");
    fs::write(
        next_root.join("broken.lib"),
        ".include definitely-missing.inc\n.model broken_n NMOS (LEVEL=1)\n",
    )
    .expect("write broken source");
    let mut next_config = crate::state::pdk_config::PdkConfig::new();
    next_config.add_library_path(next_root.to_string_lossy());
    next_config
        .discovered_files
        .push(crate::state::pdk_config::DiscoveredFile::new(
            old_file.clone(),
            old_root.clone(),
        ));

    let errors = manager
        .replace_from_pdk_config(Some(&accepted_config), &mut next_config)
        .expect_err("one invalid enabled file rejects the whole candidate");

    assert!(errors.iter().any(|error| error.contains("broken.lib")));
    assert_eq!(manager.get_library("accepted"), Some(&accepted));
    assert!(manager.get_library("candidate").is_none());
    assert!(
        next_config
            .discovered_files
            .iter()
            .any(|file| file.path == good_file)
    );
    assert!(
        next_config
            .discovered_files
            .iter()
            .all(|file| file.path != old_file)
    );

    fs::remove_dir_all(old_root).expect("remove old PDK root");
    fs::remove_dir_all(next_root).expect("remove next PDK root");
}

#[test]
fn disabling_configured_sources_unloads_only_retained_pdk_ownership() {
    let configured_root =
        std::env::temp_dir().join(format!("rspice-pdk-configured-{}", uuid::Uuid::new_v4()));
    let manual_root =
        std::env::temp_dir().join(format!("rspice-pdk-manual-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&configured_root).expect("create configured root");
    fs::create_dir_all(&manual_root).expect("create manual root");
    fs::write(
        configured_root.join("configured.lib"),
        ".model configured_n NMOS (LEVEL=1)\n",
    )
    .expect("write configured source");
    let manual_file = manual_root.join("manual.lib");
    fs::write(&manual_file, ".model manual_n NMOS (LEVEL=1)\n").expect("write manual source");

    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&manual_file, None)
        .expect("load manual external library");
    let mut enabled = crate::state::pdk_config::PdkConfig::new();
    enabled.add_library_path(configured_root.to_string_lossy());
    manager
        .replace_from_pdk_config(None, &mut enabled)
        .expect("configured source applies");
    assert_eq!(enabled.managed_model_sources.len(), 1);

    let mut disabled = enabled.clone();
    disabled.toggle_path_enabled(0);
    assert_eq!(
        manager
            .replace_from_pdk_config(Some(&enabled), &mut disabled)
            .expect("disabled configured source unloads atomically"),
        0
    );
    assert!(manager.get_library("configured").is_none());
    assert!(manager.get_library("manual").is_some());
    assert!(disabled.managed_model_sources.is_empty());
    assert!(disabled.discovered_files.is_empty());

    fs::remove_dir_all(configured_root).expect("remove configured root");
    fs::remove_dir_all(manual_root).expect("remove manual root");
}

//=========================================================================
// Shipped model pack discovery
//=========================================================================

/// Open the repository's own model tree, so these tests do not depend on
/// where the test binary happens to sit.
fn repo_pack_manager() -> ModelLibraryManager {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/spice");
    let index =
        rspice_core::library::SpiceLibraryIndex::open(root).expect("repository model tree opens");
    let mut manager = ModelLibraryManager::new();
    manager.spice_packs = Some(std::sync::Arc::new(index));
    manager
}

/// Addressable `mosfet-n` cards in `fixture-open`: more than two 17-row pages
/// and more than one bounded 25-hit search.
const FIXTURE_MOSFET_CARDS: usize = 40;
/// Addressable definitions across both fixture packs.
const FIXTURE_PARTS: usize = 44;

/// A synthetic pack corpus on disk and the manager that indexes it.
///
/// The repository ships the foundation pack alone, so paging, filtering and
/// redistribution status are exercised against a corpus built here. Index rows
/// address real files at exact lines, because a hit carries a source path a
/// caller opens.
struct FixturePacks {
    root: std::path::PathBuf,
    manager: ModelLibraryManager,
}

impl FixturePacks {
    fn discard(self) {
        fs::remove_dir_all(&self.root).expect("remove fixture pack corpus");
    }
}

fn fixture_pack_manager(label: &str) -> FixturePacks {
    let root = std::env::temp_dir().join(format!(
        "rspice-ui-packs-{label}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    ));

    // Cards are (name, kind, canonical device class, SPICE card body).
    let mut open_cards = (0..FIXTURE_MOSFET_CARDS)
        .map(|index| {
            (
                format!("FIXNMOS_{index:02}"),
                "model",
                "mosfet-n",
                "NMOS (LEVEL=1 VTO=0.7)",
            )
        })
        .collect::<Vec<_>>();
    open_cards.push(("FIXJFET".to_owned(), "model", "jfet-n", "NJF (VTO=-2.0)"));
    open_cards.push(("FIXDIODE".to_owned(), "model", "diode", "D (IS=1e-14 N=1)"));
    let packs = [
        (
            "fixture-open",
            "permissive",
            "Apache-2.0",
            true,
            "lib/open.lib",
            "Fixture Open Pack",
            open_cards,
        ),
        (
            "fixture-limited",
            "ambiguous",
            "NOASSERTION",
            false,
            "lib/limited.lib",
            "Fixture Unestablished Pack",
            vec![
                (
                    "FIXDIODE".to_owned(),
                    "model",
                    "diode",
                    "D (IS=5e-14 N=1.1)",
                ),
                ("FIXNPN".to_owned(), "model", "bjt-npn", "NPN (BF=180)"),
            ],
        ),
    ];

    let mut packs_index = String::from(
        "# id\tcategory\tpath\ttier\tspdx\tredistributable\tentry\tmodels\tsubcircuits\tmodels_top\tsubcircuits_top\tfiles\tbytes\tdevices\tname\n",
    );
    let mut catalog_index =
        String::from("# name\tkind\tdevice\tpack\tpath\tline\trestricted\tscope\n");
    for (id, tier, spdx, redistributable, entry, name, cards) in &packs {
        let mut source = format!("* {id}\n");
        let mut devices = std::collections::BTreeSet::new();
        for (line, (part, kind, device, card)) in cards.iter().enumerate() {
            // The header occupies line 1, so the nth card declares line n + 2.
            source.push_str(&format!(".model {part} {card}\n"));
            devices.insert(*device);
            catalog_index.push_str(&format!(
                "{part}\t{kind}\t{device}\t{id}\t{entry}\t{}\t0\ttop\n",
                line + 2
            ));
        }
        let path = root.join(id).join("lib");
        fs::create_dir_all(&path).expect("create fixture pack directory");
        fs::write(
            path.join(entry.rsplit('/').next().expect("entry file name")),
            &source,
        )
        .expect("write fixture pack source");
        packs_index.push_str(&format!(
            "{id}\tfixture\t{id}\t{tier}\t{spdx}\t{}\t{entry}\t{}\t0\t{}\t0\t1\t{}\t{}\t{name}\n",
            u8::from(*redistributable),
            cards.len(),
            cards.len(),
            source.len(),
            devices.into_iter().collect::<Vec<_>>().join(",")
        ));
    }
    fs::write(root.join("PACKS.tsv"), packs_index).expect("write fixture pack index");
    fs::write(root.join("CATALOG.tsv"), catalog_index).expect("write fixture catalog");

    let index =
        rspice_core::library::SpiceLibraryIndex::open(&root).expect("fixture model tree opens");
    let mut manager = ModelLibraryManager::new();
    manager.spice_packs = Some(std::sync::Arc::new(index));
    FixturePacks { root, manager }
}

#[test]
fn pack_search_finds_definitions_the_libraries_do_not_hold() {
    let fixture = fixture_pack_manager("search-finds");
    // Addressable parts, not raw definitions: `pack_definition_count` reports
    // what a netlist can reference by name.
    assert_eq!(fixture.manager.pack_definition_count(), FIXTURE_PARTS);

    // Nothing is loaded, so a plain library search finds nothing...
    assert!(fixture.manager.search_models("FIXJFET").is_empty());
    // ...but the packs carry it.
    let hits = fixture.manager.search_pack_models("FIXJFET", 50);
    assert_eq!(hits.len(), 1);
    let hit = &hits[0];
    assert!(hit.source.as_ref().is_some_and(|p| p.is_file()));
    assert!(hit.line > 0);

    fixture.discard();
}

#[test]
fn pack_search_is_bounded_and_ignores_an_empty_query() {
    let fixture = fixture_pack_manager("search-bounded");
    // An empty query must not stream the whole index.
    assert!(fixture.manager.search_pack_models("", 50).is_empty());
    assert!(fixture.manager.search_pack_models("   ", 50).is_empty());

    // A broad query is capped at the caller's limit.
    let hits = fixture.manager.search_pack_models("FIXNMOS", 25);
    assert_eq!(hits.len(), 25);

    fixture.discard();
}

#[test]
fn pack_browse_applies_pack_and_device_filters_before_exact_paging() {
    let fixture = fixture_pack_manager("browse-paging");
    let (total, first) = fixture
        .manager
        .browse_pack_models("", Some("fixture-open"), &["mosfet-n"], 0, 17)
        .expect("first exact page");
    let (same_total, second) = fixture
        .manager
        .browse_pack_models("", Some("fixture-open"), &["mosfet-n"], 17, 17)
        .expect("second exact page");

    assert_eq!(total, FIXTURE_MOSFET_CARDS);
    assert_eq!(same_total, total);
    assert_eq!(first.len(), 17);
    assert_eq!(second.len(), 17);
    assert!(
        first.iter().chain(&second).all(|hit| {
            hit.pack == "fixture-open" && hit.device.eq_ignore_ascii_case("mosfet-n")
        })
    );
    let first_keys = first
        .iter()
        .map(|hit| (&hit.name, &hit.source, hit.line))
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        second
            .iter()
            .all(|hit| !first_keys.contains(&(&hit.name, &hit.source, hit.line)))
    );

    fixture.discard();
}

#[test]
fn pack_hits_carry_their_redistribution_status() {
    let fixture = fixture_pack_manager("redistribution-status");
    // The same part name sits in both packs, so the flag has to come from the
    // owning pack rather than from the definition.
    let hits = fixture.manager.search_pack_models("FIXDIODE", 20);
    assert_eq!(hits.len(), 2);
    assert!(
        hits.iter()
            .filter(|hit| hit.pack == "fixture-open")
            .all(|hit| hit.redistributable)
    );
    assert!(
        hits.iter()
            .filter(|hit| hit.pack == "fixture-limited")
            .all(|hit| !hit.redistributable)
    );

    fixture.discard();
}

#[test]
fn attached_pack_becomes_a_portable_retained_project_snapshot() {
    let mut manager = repo_pack_manager();
    let library_name = manager
        .attach_spice_pack("rspice-foundation")
        .expect("foundation pack attaches");
    let library = manager
        .get_library(&library_name)
        .expect("pack library exists");
    assert_eq!(library.pack_id.as_deref(), Some("rspice-foundation"));
    assert!(matches!(
        library.source_authority,
        ModelSourceAuthority::RetainedImport { .. }
    ));
    assert_eq!(library.source_contents.len(), library.source_closure.len());
    manager
        .seal_execution_sources()
        .expect("attached pack seals entirely from its retained snapshot");
}

#[test]
fn browser_selected_sources_remain_generic_project_imports() {
    let mut manager = ModelLibraryManager::new();
    let entry = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../models/spice/foundation/lib/foundation.lib");
    let bytes = std::fs::read(entry).expect("foundation pack entry is readable");
    let library = manager
        .load_library_bundle_from_root(
            "foundation.lib",
            "lib/foundation.lib",
            vec![("lib/foundation.lib".to_owned(), bytes.clone())],
            None,
        )
        .expect("selected entry imports from retained bytes");
    let imported = manager
        .get_library(&library)
        .expect("imported library remains present");
    assert_eq!(imported.pack_id, None);
    assert!(matches!(
        imported.source_authority,
        ModelSourceAuthority::RetainedImport { .. }
    ));

    let collision = manager
        .load_library_bundle_from_root(
            "foundation.lib",
            "lib/foundation.lib",
            vec![("lib/foundation.lib".to_owned(), bytes.clone())],
            None,
        )
        .expect_err("an existing import cannot be overwritten implicitly");
    assert!(collision.contains("already exists"));
    assert_eq!(
        manager
            .get_library(&library)
            .and_then(|library| library.pack_id.as_deref()),
        None,
        "a rejected import must leave the prior generic snapshot intact"
    );

    let mut extra_manager = ModelLibraryManager::new();
    let imported_with_unrelated_source = extra_manager
        .load_library_bundle_from_root(
            "foundation.lib",
            "lib/foundation.lib",
            vec![
                ("lib/foundation.lib".to_owned(), bytes),
                (
                    "unrelated.lib".to_owned(),
                    b".model unrelated D (IS=2e-14)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect("the selected entry ignores an unrelated selected source");
    let imported = extra_manager
        .get_library(&imported_with_unrelated_source)
        .expect("pack library retained");
    assert!(!imported.models.contains_key("unrelated"));
    assert_eq!(imported.source_contents.len(), 1);
}

#[test]
fn execution_catalog_digest_is_invariant_under_model_map_iteration_order() {
    let mut deck = String::new();
    for index in 0..16 {
        deck.push_str(&format!(
            ".model nch_{index:02} NMOS (LEVEL=1 KP={}e-3)\n",
            index + 1
        ));
    }
    let mut manager = ModelLibraryManager::new();
    let library_name = manager
        .load_library_bytes("catalog-order.lib", deck.into_bytes(), None)
        .expect("catalogue imports");
    let baseline = manager.execution_catalog_digest();

    // Re-collecting rebuilds the map behind an independent hasher, which is
    // what happens whenever the catalogue is reconstructed instead of cloned.
    // A prepared run rebuilds its snapshot before dispatch and rejects it when
    // this digest moves, so identical content has to hash identically.
    let reordered = manager
        .get_library(&library_name)
        .expect("library retained")
        .models
        .clone()
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    manager
        .get_library_mut(&library_name)
        .expect("library retained")
        .models = reordered;

    assert_eq!(
        manager.execution_catalog_digest(),
        baseline,
        "catalogue identity must depend on content, never on map iteration order"
    );
}

#[test]
fn missing_pack_tree_is_not_an_error() {
    // The browser build has no packs, and a source checkout may not have
    // synced them. Both must degrade to an empty search, not a failure.
    let manager = ModelLibraryManager::new();
    assert_eq!(manager.pack_definition_count(), 0);
    assert!(manager.search_pack_models("2N3904", 10).is_empty());
    assert!(manager.spice_packs().is_none());
}
