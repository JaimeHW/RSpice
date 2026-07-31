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
    let directory = std::env::temp_dir().join(format!(
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
    assert!(
        library.selected_corner.is_none(),
        "a sectionless upload must not retain ModelLibrary::new's default TT selection"
    );
    assert!(matches!(
        library.source_authority,
        ModelSourceAuthority::RetainedImport { .. }
    ));
    let binding = crate::state::ProjectTechnologyBinding::from_model_library(library)
        .expect("byte-backed library is attachable");
    manager
        .validate_attached_technology(Some(&binding))
        .expect("unchanged byte-backed catalog matches attachment");
}

#[test]
fn byte_backed_subcircuit_import_retains_exact_interface_without_device_models() {
    let bytes = b"* Precision amplifier\n\
        .subckt AMP inp inn out params: GAIN=100 MODE=\"low noise\" SCALE={GAIN * 2}\n\
        e1 out 0 inp inn {GAIN}\n\
        .ends AMP\n"
        .to_vec();
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bytes("browser-subcircuits.lib", bytes, None)
        .expect("a pure subcircuit source imports");
    let library = manager.get_library(&name).expect("library retained");
    assert!(library.models.is_empty());
    let interface = library.subcircuits.get("AMP").expect("interface retained");
    assert_eq!(interface.ports, ["inp", "inn", "out"]);
    assert_eq!(
        interface.parameter_defaults.get("GAIN").map(String::as_str),
        Some("100")
    );
    assert_eq!(
        interface.parameter_defaults.get("MODE").map(String::as_str),
        Some("\"low noise\"")
    );
    assert_eq!(
        interface
            .parameter_defaults
            .get("SCALE")
            .map(String::as_str),
        Some("{GAIN * 2}")
    );
    assert_eq!(interface.source_line, Some(2));
    assert!(interface.section.is_none());
    assert_eq!(
        interface.file_path.as_deref(),
        library.root_path.as_deref(),
        "browser imports use their authenticated virtual source identity"
    );
}

#[test]
fn active_scope_rejects_case_colliding_subcircuit_interfaces_transactionally() {
    let mut manager = ModelLibraryManager::new();
    let error = manager
        .load_library_bytes(
            "duplicate-subcircuits.lib",
            b".lib TT\n\
              .subckt AMP in out\n.ends AMP\n\
              .subckt amp plus minus\n.ends amp\n\
              .endl TT\n"
                .to_vec(),
            Some("TT"),
        )
        .expect_err("case-colliding active interfaces must fail closed");
    assert!(error.contains("same library section"), "{error}");
    assert_eq!(manager.library_count(), 0);
}

#[test]
fn same_named_subcircuits_in_distinct_sections_remain_independently_addressable() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bytes(
            "sectioned-subcircuits.lib",
            b".lib TT\n\
              .subckt AMP inp out\n.ends AMP\n\
              .endl TT\n\
              .lib FF\n\
              .subckt AMP inp inn out\n.ends AMP\n\
              .endl FF\n"
                .to_vec(),
            Some("TT"),
        )
        .expect("same name in different sections is valid");
    let library = manager.get_library(&name).expect("library retained");
    assert_eq!(library.subcircuits.len(), 2);
    assert_eq!(library.subcircuits["TT\u{1f}AMP"].ports.len(), 2);
    assert_eq!(library.subcircuits["FF\u{1f}AMP"].ports.len(), 3);
    assert_eq!(
        library.subcircuits["FF\u{1f}AMP"].section.as_deref(),
        Some("FF")
    );
}

#[test]
fn contested_model_definitions_require_explicit_stable_provider_precedence() {
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_bytes(
            "provider-a.lib",
            b".model SharedCard NMOS (LEVEL=1 KP=1e-3)\n".to_vec(),
            None,
        )
        .expect("first provider imports");
    manager
        .load_library_bytes(
            "provider-b.lib",
            b".model sharedcard NMOS (LEVEL=1 KP=2e-3)\n".to_vec(),
            None,
        )
        .expect("second provider imports");

    let conflicts = manager.definition_conflicts();
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].normalized_name, "sharedcard");
    assert_eq!(conflicts[0].providers.len(), 2);
    assert_eq!(conflicts[0].providers[0].library, "provider-a");
    assert_eq!(conflicts[0].providers[1].library, "provider-b");

    let error = manager
        .seal_execution_sources()
        .expect_err("implicit provider order must never authorize execution");
    assert!(error.contains("Contested model definition 'sharedcard'"));
    assert!(error.contains("RSpice will not choose by implicit include order"));

    manager
        .resolve_definition_conflict("sharedcard", "provider-b", "sharedcard")
        .expect("exact second provider is selected");
    manager
        .validate_definition_resolution()
        .expect("explicit precedence is complete and acyclic");
    let cards = manager
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
        .expect("resolved providers seal and materialize");
    assert_eq!(cards.len(), 2);
    assert!(cards[0].contains("provider-b.lib"));
    assert!(cards[0].contains("KP=2e-3"));
    assert!(cards[1].contains("provider-a.lib"));
    let parsed = rspice_core::Netlist::parse(&format!(
        "resolved provider precedence\n{}\n.end\n",
        cards.join("\n")
    ))
    .expect("materialized resolution deck parses");
    assert!(parsed.models[0].name.eq_ignore_ascii_case("sharedcard"));
    assert_eq!(
        parsed.models[0]
            .params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("KP"))
            .map(|(_, value)| *value),
        Some(2.0e-3)
    );
    assert_ne!(
        ModelLibraryManager::new().execution_catalog_digest(),
        manager.execution_catalog_digest(),
        "provider resolution participates in execution identity"
    );
}

#[test]
fn active_duplicate_subcircuits_fail_closed_before_source_sealing() {
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_bytes(
            "macro-a.lib",
            b".lib TT\n.subckt SharedAmp in out\nR1 in out 1k\n.ends SharedAmp\n.endl TT\n"
                .to_vec(),
            Some("TT"),
        )
        .expect("first subcircuit provider imports");
    manager
        .load_library_bytes(
            "macro-b.lib",
            b".lib TT\n.subckt sharedamp in out\nR1 in out 2k\n.ends sharedamp\n.endl TT\n"
                .to_vec(),
            Some("TT"),
        )
        .expect("second subcircuit provider imports");

    let error = manager
        .seal_execution_sources()
        .expect_err("implicit subcircuit include order must never authorize execution");
    assert!(error.contains("Active subcircuit definition 'sharedamp'"));
    assert!(error.contains("macro-a/TT"));
    assert!(error.contains("macro-b/TT"));
    assert!(error.contains("will not choose a subcircuit by implicit include order"));
}

#[test]
fn conflicting_provider_choices_that_require_a_cycle_fail_closed() {
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_bytes(
            "provider-a.lib",
            b".model shared_one NMOS (LEVEL=1 KP=1e-3)\n.model shared_two NMOS (LEVEL=1 KP=2e-3)\n"
                .to_vec(),
            None,
        )
        .expect("first provider imports");
    manager
        .load_library_bytes(
            "provider-b.lib",
            b".model shared_one NMOS (LEVEL=1 KP=3e-3)\n.model shared_two NMOS (LEVEL=1 KP=4e-3)\n"
                .to_vec(),
            None,
        )
        .expect("second provider imports");
    manager
        .resolve_definition_conflict("shared_one", "provider-a", "shared_one")
        .expect("first choice records");
    manager
        .resolve_definition_conflict("shared_two", "provider-b", "shared_two")
        .expect("second choice records");

    let error = manager
        .validate_definition_resolution()
        .expect_err("crossed provider choices cannot produce one exact source order");
    assert!(error.contains("cyclic precedence contract"));
}

#[test]
fn pdk_config_import_is_atomic_when_any_discovered_library_fails() {
    let (directory, valid) = model_fixture();
    let missing = directory.join("missing.lib");
    let mut config = crate::state::pdk_config::PdkConfig::new();
    config.discovered_files = vec![
        crate::state::pdk_config::DiscoveredFile::new(valid, directory.clone()),
        crate::state::pdk_config::DiscoveredFile::new(missing, directory.clone()),
    ];
    let mut manager = ModelLibraryManager::new();
    let before = manager.execution_catalog_digest();

    let errors = manager
        .load_from_pdk_config(&config)
        .expect_err("one failed source must reject the complete batch");

    assert_eq!(errors.len(), 1);
    assert_eq!(manager.execution_catalog_digest(), before);
    assert!(
        manager.libraries_sorted().is_empty(),
        "a failed batch must not retain libraries loaded before the failure"
    );
    fs::remove_dir_all(directory).expect("remove model fixture");
}

#[test]
fn pdk_config_reconciliation_removes_disabled_and_deleted_sources_only() {
    let (directory, _) = model_fixture();
    fs::write(
        directory.join("passives.mod"),
        ".model pdk_res R (RSH=11)\n",
    )
    .expect("write second PDK source");

    let mut config = crate::state::pdk_config::PdkConfig::new();
    config.add_library_path(directory.to_string_lossy().to_string());
    config.discover_model_files();

    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_bytes(
            "direct-import.lib",
            b".model direct_n NMOS (LEVEL=1 KP=9e-3)\n".to_vec(),
            None,
        )
        .expect("direct retained source imports");
    assert_eq!(
        manager
            .load_from_pdk_config(&config)
            .expect("initial PDK scan publishes"),
        2
    );
    assert_eq!(manager.pdk_config_libraries.len(), 2);
    assert_eq!(manager.library_count(), 3);

    config.library_paths_mut()[0].enabled = false;
    config.discover_model_files();
    assert_eq!(
        manager
            .load_from_pdk_config(&config)
            .expect("disabled scan reconciles"),
        0
    );
    assert!(manager.pdk_config_libraries.is_empty());
    assert_eq!(manager.library_count(), 1);
    assert!(
        manager.get_library("direct-import").is_some(),
        "direct retained imports are not owned by host PDK configuration"
    );

    config.library_paths_mut()[0].enabled = true;
    config.discover_model_files();
    assert_eq!(
        manager
            .load_from_pdk_config(&config)
            .expect("re-enabled scan republishes"),
        2
    );
    config.remove_library_path(0);
    config.discover_model_files();
    assert_eq!(
        manager
            .load_from_pdk_config(&config)
            .expect("deleted path reconciles"),
        0
    );
    assert_eq!(manager.library_count(), 1);
    assert!(manager.get_library("direct-import").is_some());

    fs::remove_dir_all(directory).expect("remove reconciliation fixture");
}

#[test]
fn pdk_config_loads_every_advertised_model_extension() {
    let (directory, _) = model_fixture();
    for (extension, name) in [
        ("scs", "from_scs"),
        ("mod", "from_mod"),
        ("sp", "from_sp"),
        ("cir", "from_cir"),
    ] {
        fs::write(
            directory.join(format!("{name}.{extension}")),
            format!(".model {name} D (IS=1e-14)\n"),
        )
        .expect("write advertised model source");
    }
    let mut config = crate::state::pdk_config::PdkConfig::new();
    config.add_library_path(directory.to_string_lossy().to_string());
    config.discover_model_files();
    assert_eq!(config.discovered_files().len(), 5);

    let mut manager = ModelLibraryManager::new();
    assert_eq!(
        manager
            .load_from_pdk_config(&config)
            .expect("all advertised source types load"),
        5
    );
    for model in ["nch", "from_scs", "from_mod", "from_sp", "from_cir"] {
        assert!(
            manager
                .libraries_sorted()
                .iter()
                .any(|library| library.models.contains_key(model)),
            "missing model from advertised source: {model}"
        );
    }

    fs::remove_dir_all(directory).expect("remove extension fixture");
}

#[test]
fn pdk_config_same_stem_sources_receive_stable_collision_safe_names() {
    let (directory, _) = model_fixture();
    let first = directory.join("first");
    let second = directory.join("second");
    fs::create_dir_all(&first).expect("create first source directory");
    fs::create_dir_all(&second).expect("create second source directory");
    fs::write(
        first.join("models.lib"),
        ".model first_n NMOS (LEVEL=1 KP=1e-3)\n",
    )
    .expect("write first same-stem source");
    fs::write(
        second.join("models.lib"),
        ".model second_n NMOS (LEVEL=1 KP=2e-3)\n",
    )
    .expect("write second same-stem source");

    let mut config = crate::state::pdk_config::PdkConfig::new();
    config.add_library_path(first.to_string_lossy().to_string());
    config.add_library_path(second.to_string_lossy().to_string());
    config.discover_model_files();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_from_pdk_config(&config)
        .expect("same-stem sources load independently");
    let identities = manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            (
                library.root_path.clone().expect("PDK source has root"),
                library.name.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(identities.len(), 2);
    assert_eq!(
        identities
            .values()
            .filter(|name| name.as_str() == "models")
            .count(),
        1
    );
    assert_eq!(
        identities
            .values()
            .filter(|name| name.starts_with("models@"))
            .count(),
        1
    );

    fs::write(
        second.join("models.lib"),
        ".model second_n NMOS (LEVEL=1 KP=3e-3)\n",
    )
    .expect("refresh second same-stem source");
    config.discover_model_files();
    manager
        .load_from_pdk_config(&config)
        .expect("same-stem refresh remains stable");
    let refreshed = manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            (
                library.root_path.clone().expect("PDK source has root"),
                library.name.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(refreshed, identities);

    fs::remove_dir_all(directory).expect("remove same-stem fixture");
}

#[test]
fn empty_or_unsupported_pdk_source_rejects_the_complete_reconciliation() {
    let (directory, _) = model_fixture();
    let empty = directory.join("spectre-only.scs");
    fs::write(&empty, "simulator lang=spectre\nparameters vdd=1.8\n")
        .expect("write unsupported empty source");
    let mut config = crate::state::pdk_config::PdkConfig::new();
    config.discovered_files = vec![crate::state::pdk_config::DiscoveredFile::new(
        empty,
        directory.clone(),
    )];
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_bytes(
            "direct-import.lib",
            b".model direct_n NMOS (LEVEL=1 KP=9e-3)\n".to_vec(),
            None,
        )
        .expect("direct source imports");
    let before = manager.execution_catalog_digest();

    let errors = manager
        .load_from_pdk_config(&config)
        .expect_err("unsupported source cannot publish an empty library");
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("no supported device models"));
    assert_eq!(manager.execution_catalog_digest(), before);
    assert!(manager.get_library("direct-import").is_some());

    fs::remove_dir_all(directory).expect("remove unsupported-source fixture");
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
            && !rspice_core::netlist::parse_lib_directive(line)
                .is_some_and(|(_, section)| section.is_some())
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::FF)
        .expect("FF binding exists");
    let bindings = manager
        .corner_model_bindings(&[CornerProcess::TT, CornerProcess::FF])
        .expect("TT and FF bindings exist");

    assert_eq!(reference.len(), 1);
    assert!(reference[0].contains("RSpice sealed model source"));
    assert!(reference[0].contains("KP=2e-3"));
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
    assert_eq!(
        manager
            .get_library("foundry")
            .and_then(|library| library.models.get("nch"))
            .and_then(|model| model.section.as_deref()),
        Some("TT")
    );

    manager
        .get_library_mut("foundry")
        .expect("library exists")
        .selected_corner = Some("FF".to_owned());
    manager
        .rebuild_active_model_projection("foundry")
        .expect("active FF catalog rebuilds from retained authenticated bytes");
    let ff_model = &manager
        .get_library("foundry")
        .expect("library exists")
        .models["nch"];
    assert_eq!(ff_model.section.as_deref(), Some("FF"));
    assert_eq!(
        ff_model.parameters.get("kp").map(|value| value.to_bits()),
        Some(2e-3_f64.to_bits())
    );

    let error = manager
        .corner_model_bindings(&[CornerProcess::SS])
        .expect_err("undefined SS section must fail closed");
    assert!(error.contains("does not define the SS process section"));
    fs::remove_dir_all(directory).expect("remove model fixture directory");
}

#[test]
fn selected_section_shadows_top_level_model_names_case_insensitively() {
    let mut manager = ModelLibraryManager::new();
    let name = manager
        .load_library_bytes(
            "case-shadow.lib",
            b".model NCH NMOS (LEVEL=1 KP=5e-4)\n\
              .lib TT\n\
              .model nch NMOS (LEVEL=1 KP=1e-3)\n\
              .endl TT\n"
                .to_vec(),
            Some("tt"),
        )
        .expect("case-insensitive selected section imports");

    let library = manager.get_library(&name).expect("library exists");
    assert_eq!(library.models.len(), 1);
    assert!(!library.models.contains_key("NCH"));
    let selected = &library.models["nch"];
    assert_eq!(selected.section.as_deref(), Some("TT"));
    assert_eq!(
        selected.parameters.get("kp").map(|value| value.to_bits()),
        Some(1e-3_f64.to_bits())
    );

    manager
        .rebuild_active_model_projection(&name)
        .expect("retained projection preserves case-insensitive shadowing");
    let rebuilt = manager.get_library(&name).expect("library exists");
    assert_eq!(rebuilt.models.len(), 1);
    assert_eq!(rebuilt.models["nch"].section.as_deref(), Some("TT"));
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
        .expect("materialization uses only sealed bytes");
    let cards = cards.join("\n");
    assert!(cards.contains("KP=1e-3"), "{cards}");
    assert!(cards.contains("KP=2e-3"), "{cards}");
    assert!(!cards.contains("KP=9e-3"), "{cards}");
    rspice_core::Netlist::parse(&format!("sealed worker deck\n{cards}\n.end\n"))
        .expect("self-contained sealed cards parse without source files");

    let fresh_error = manager
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
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
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
        .expect("authenticated edge no longer needs symlink")
        .join("\n");
    assert!(cards.contains("symlink_n"), "{cards}");

    fs::remove_dir_all(directory).expect("remove symlink fixture directory");
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

#[test]
fn pack_search_finds_definitions_the_libraries_do_not_hold() {
    let manager = repo_pack_manager();
    // Only top-level `.model` cards belong in this workspace. Subcircuits and
    // nested helper definitions use separate interface workflows.
    assert!(
        manager.pack_definition_count() > 15_000,
        "expected the shipped packs, counted {}",
        manager.pack_definition_count()
    );

    // Nothing is loaded, so a plain library search finds nothing...
    assert!(manager.search_models("2N3819").is_empty());
    // ...but the shipped packs carry it.
    let hits = manager.search_pack_models("2N3819", 50);
    assert!(!hits.is_empty(), "expected 2N3819 in the shipped packs");
    let hit = &hits[0];
    assert_eq!(hit.kind, "model");
    assert!(hit.source.as_ref().is_some_and(|p| p.is_file()));
    assert!(hit.line > 0);
}

#[test]
fn redistributable_pack_model_activation_is_exact_and_executable() {
    let mut manager = repo_pack_manager();
    let hit = manager
        .search_pack_models("2N3819", 50)
        .into_iter()
        .find(|hit| hit.pack == "builtin")
        .expect("redistributable built-in pack hit");
    assert!(hit.redistributable);

    let library_name = manager
        .activate_pack_model(&hit)
        .expect("pack model activates");
    assert_eq!(library_name, "pack-builtin-jfet");
    assert_eq!(
        manager.selected_library.as_deref(),
        Some(library_name.as_str())
    );
    let library = manager
        .get_library(&library_name)
        .expect("activated library retained");
    assert!(
        library
            .models
            .values()
            .any(|model| model.name.eq_ignore_ascii_case("2N3819"))
    );
    assert!(library.source_authority.has_execution_source());
    assert!(matches!(
        library.source_authority,
        ModelSourceAuthority::RetainedImport { .. }
    ));
    assert!(library.source_authority.uses_retained_bytes());
    assert!(!library.source_closure.is_empty());
    manager
        .seal_execution_sources()
        .expect("activated pack source seals for execution");
}

#[test]
fn redistributable_pack_subcircuit_activation_retains_exact_ordered_interface() {
    let mut manager = repo_pack_manager();
    let hit = manager
        .query_pack_parts("LM358", Some("builtin"), &[], 0, 50)
        .expect("pack query succeeds")
        .hits
        .into_iter()
        .find(|hit| hit.name == "LM358" && hit.kind == "subckt")
        .expect("redistributable LM358 subcircuit hit");
    assert!(hit.redistributable);

    let activated = manager
        .activate_pack_subcircuit(&hit)
        .expect("subcircuit source and interface activate");
    assert_eq!(activated.name, "LM358");
    assert!(!activated.ports.is_empty());
    assert!(activated.source_path.is_file());
    let library = manager
        .get_library(&activated.library)
        .expect("activated source library retained");
    assert!(matches!(
        library.source_authority,
        ModelSourceAuthority::RetainedImport { .. }
    ));
    let interface = library
        .subcircuits
        .get(&activated.name)
        .expect("activated interface is retained in the project catalog");
    assert_eq!(interface.ports, activated.ports);
    assert_eq!(
        interface.file_path.as_deref(),
        Some(activated.source_path.as_path())
    );
    manager
        .seal_execution_sources()
        .expect("activated subcircuit source seals");
}

#[test]
fn browse_only_pack_model_activation_fails_without_mutating_libraries() {
    let mut manager = repo_pack_manager();
    let hit = manager
        .search_pack_models("2N3819", 50)
        .into_iter()
        .find(|hit| hit.pack == "interfet-jfet")
        .expect("browse-only vendor hit");
    assert!(!hit.redistributable);
    let before = manager.library_count();

    let error = manager
        .activate_pack_model(&hit)
        .expect_err("unestablished redistribution must block activation");
    assert!(error.contains("browse-only"), "{error}");
    assert_eq!(manager.library_count(), before);
    assert!(manager.selected_library.is_none());
}

#[test]
fn stale_or_forged_pack_hit_fails_before_loading_source() {
    let mut manager = repo_pack_manager();
    let mut hit = manager
        .search_pack_models("2N3819", 50)
        .into_iter()
        .find(|hit| hit.pack == "builtin")
        .expect("redistributable built-in pack hit");
    hit.line = hit.line.saturating_add(1);

    let error = manager
        .activate_pack_model(&hit)
        .expect_err("forged line identity must fail");
    assert!(error.contains("changed or disappeared"), "{error}");
    assert_eq!(manager.library_count(), 0);
}

#[test]
fn pack_search_is_bounded_and_ignores_an_empty_query() {
    let manager = repo_pack_manager();
    // An empty query must not stream the whole 16 MB index.
    assert!(manager.search_pack_models("", 50).is_empty());
    assert!(manager.search_pack_models("   ", 50).is_empty());

    // A broad query is capped at the caller's limit.
    let hits = manager.search_pack_models("1N", 25);
    assert_eq!(hits.len(), 25);
}

#[test]
fn pack_hits_carry_their_redistribution_status() {
    let manager = repo_pack_manager();
    let hits = manager.search_pack_models("mcl1d", 20);
    assert!(!hits.is_empty(), "expected sky130 model cards in the packs");
    // sky130 is Apache-2.0, so its rows must not be flagged unlicensed.
    assert!(
        hits.iter().any(|hit| hit.redistributable),
        "expected at least one redistributable hit"
    );
}

#[test]
fn shipped_part_query_pages_the_complete_addressable_corpus() {
    let manager = repo_pack_manager();
    let page = manager
        .query_pack_parts("", None, &[], 0, 40)
        .expect("part catalog query");
    assert_eq!(
        page.total_matches,
        manager.spice_packs().expect("index exists").part_count()
    );
    assert_eq!(page.hits.len(), 40);
    assert!(page.hits.iter().all(|hit| hit.source.is_some()));

    let diode_page = manager
        .query_pack_parts("", None, &["diode"], 0, 25)
        .expect("device facet query");
    assert!(diode_page.total_matches > diode_page.hits.len());
    assert!(
        diode_page
            .hits
            .iter()
            .all(|hit| hit.device.eq_ignore_ascii_case("diode"))
    );
}

#[test]
fn shipped_part_preview_revalidates_exact_identity() {
    let manager = repo_pack_manager();
    let hit = manager
        .query_pack_parts("1N4148", None, &[], 0, 1)
        .expect("part query")
        .hits
        .into_iter()
        .next()
        .expect("part exists");
    let preview = manager.preview_pack_part(&hit).expect("preview reads");
    assert!(preview.source.to_ascii_lowercase().contains("1n4148"));
    assert_eq!(preview.start_line, hit.line);

    let mut forged = hit;
    forged.line = forged.line.saturating_add(1);
    assert!(
        manager
            .preview_pack_part(&forged)
            .expect_err("forged identity fails")
            .contains("changed or disappeared")
    );
}

#[test]
fn pack_attachment_state_and_detach_are_exact() {
    let mut manager = repo_pack_manager();
    let hit = manager
        .search_pack_models("2N3819", 50)
        .into_iter()
        .find(|hit| hit.pack == "builtin")
        .expect("redistributable built-in hit");
    assert!(!manager.is_pack_attached("builtin"));
    manager
        .activate_pack_model(&hit)
        .expect("model source attaches");
    assert!(manager.is_pack_attached("builtin"));

    assert_eq!(manager.detach_pack("builtin").expect("pack detaches"), 1);
    assert!(!manager.is_pack_attached("builtin"));
    assert_eq!(manager.library_count(), 0);
}

#[test]
fn redistributable_pack_entry_attaches_and_restricted_or_entryless_packs_fail_closed() {
    let mut manager = repo_pack_manager();
    let loaded = manager
        .attach_pack("builtin")
        .expect("built-in pack attaches");
    assert!(loaded.starts_with("pack-builtin-"));
    assert!(manager.is_pack_attached("builtin"));
    assert!(manager.get_library(&loaded).is_some_and(|library| {
        !library.models.is_empty()
            && matches!(
                library.source_authority,
                ModelSourceAuthority::RetainedImport { .. }
            )
    }));

    assert!(
        manager
            .attach_pack("microcap-library")
            .expect_err("restricted pack cannot attach")
            .contains("browse-only")
    );
    assert!(
        manager
            .attach_pack("mosis-bsim")
            .expect_err("entryless pack cannot attach")
            .contains("no declared entry")
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

#[test]
fn browser_bundle_import_authenticates_the_complete_include_closure() {
    let mut manager = ModelLibraryManager::new();

    let library_name = manager
        .load_library_bundle_bytes(
            vec![
                ("root.lib".to_owned(), b".include \"device.inc\"\n".to_vec()),
                (
                    "device.inc".to_owned(),
                    b"* exact dependency\n.model nested_n NMOS (LEVEL=1 KP=1e-3)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect("authenticated sibling bundle imports");

    assert_eq!(library_name, "root");
    let library = manager.get_library("root").expect("library published");
    assert_eq!(library.source_closure.len(), 2);
    assert_eq!(library.source_contents.len(), 2);
    assert_eq!(library.source_edges.len(), 1);
    assert_eq!(library.source_edges[0].requested_path, "device.inc");
    assert!(library.models.values().any(|model| {
        model.name == "nested_n"
            && model
                .file_path
                .as_ref()
                .is_some_and(|path| path.file_name().is_some_and(|name| name == "device.inc"))
    }));
    assert!(matches!(
        library.source_authority,
        ModelSourceAuthority::RetainedImport { .. }
    ));
}

#[test]
fn browser_bundle_import_rejects_missing_dependencies_without_mutation() {
    let mut manager = ModelLibraryManager::new();

    let error = manager
        .load_library_bundle_bytes(
            vec![(
                "root.lib".to_owned(),
                b".include \"missing.inc\"\n".to_vec(),
            )],
            None,
        )
        .expect_err("missing dependency must fail closed");

    assert!(error.contains("missing sibling 'missing.inc'"), "{error}");
    assert_eq!(manager.library_count(), 0);
}

#[test]
fn browser_bundle_import_rejects_ambiguous_roots_and_case_collisions() {
    let mut manager = ModelLibraryManager::new();
    let error = manager
        .load_library_bundle_bytes(
            vec![
                (
                    "first.lib".to_owned(),
                    b".model first_n NMOS (LEVEL=1)\n".to_vec(),
                ),
                (
                    "second.lib".to_owned(),
                    b".model second_n NMOS (LEVEL=1)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect_err("independent sources have no unique root");
    assert!(error.contains("2 independent roots"), "{error}");
    assert_eq!(manager.library_count(), 0);

    let error = manager
        .load_library_bundle_bytes(
            vec![
                (
                    "device.inc".to_owned(),
                    b".model first_n NMOS (LEVEL=1)\n".to_vec(),
                ),
                (
                    "DEVICE.INC".to_owned(),
                    b".model second_n NMOS (LEVEL=1)\n".to_vec(),
                ),
            ],
            None,
        )
        .expect_err("portable browser bundle names cannot case-collide");
    assert!(error.contains("case-insensitive file name"), "{error}");
    assert_eq!(manager.library_count(), 0);
}
