//! Exact retained-byte publication and tamper rejection across catalog and execution.

use super::*;
use rspice_app_types::product::ObjectRevision;
use rspice_model_library::qualification::ModelQualificationState;
use rspice_model_library::{ProjectModelDefinition, ProjectModelRevisionDefinition};

fn create_project_model(
    catalog: &mut rspice_model_library::ModelCatalog,
    name: &str,
    definition: &ProjectModelDefinition,
) -> Result<rspice_model_library::ProjectModelCommit, String> {
    let metadata = definition.reconcile_metadata(None)?;
    catalog.create_project_model_revision(
        name,
        &ProjectModelRevisionDefinition::new(definition.clone(), metadata),
        &ModelQualificationState::default(),
    )
}

fn seal(
    catalog: &rspice_model_library::ModelCatalog,
    read_external: impl FnMut(&Path) -> Result<Vec<u8>, String>,
) -> Result<SealedModelExecutionSources, String> {
    let libraries = catalog
        .libraries_sorted()
        .into_iter()
        .filter(|library| library.source_authority.has_execution_source())
        .map(|library| (library, library.selected_corner.clone()))
        .collect();
    seal_model_sources(libraries, &BTreeMap::new(), read_external)
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
fn project_model_create_and_replace_publish_exact_retained_execution_bytes() {
    let mut manager = rspice_model_library::ModelCatalog::default();
    let created = create_project_model(
        &mut manager,
        "owned_models",
        &project_definition(0.48, "r1"),
    )
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

    let sealed = seal(&manager, |path| {
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
        .replace_project_model_revision_in_library(
            rspice_model_library::ProjectModelTarget {
                library_name: "owned_models",
                source_id,
                library_revision: revision,
                model_revision: revision,
                model_name: "owned_nch",
                model_digest: first_digest,
            },
            &ProjectModelRevisionDefinition::new(
                project_definition(0.51, "r2"),
                project_definition(0.51, "r2")
                    .reconcile_metadata(None)
                    .expect("candidate metadata"),
            ),
            &ModelQualificationState::default(),
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
fn project_model_tamper_fails_before_any_external_read() {
    let mut manager = rspice_model_library::ModelCatalog::default();
    create_project_model(
        &mut manager,
        "owned_models",
        &project_definition(0.48, "r1"),
    )
    .expect("create project model");
    manager
        .get_library_mut("owned_models")
        .unwrap()
        .source_contents[0]
        .bytes
        .push(b' ');
    let error = seal(&manager, |path| {
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

#[test]
fn materialized_cards_leave_their_veriloga_directives_to_the_sealed_runtimes() {
    let cards = ".model fast_d D (IS=1e-14)\n\
                 .va \"rules.vams\" CONNECTIONS\n\
                 .veriloga \"devices.va\" cell\n\
                 + module=device\n\
                 .model slow_d D (IS=1e-15\n\
                 + N=1.2)\n";

    assert_eq!(
        without_veriloga_source_directives(cards.to_owned()).as_deref(),
        Some(".model fast_d D (IS=1e-14)\n.model slow_d D (IS=1e-15\n+ N=1.2)\n")
    );
    let plain = ".model d D\n".to_owned();
    assert_eq!(
        without_veriloga_source_directives(plain.clone()),
        Some(plain)
    );
    // A connection library: its rules reach the deck as their runtime alone.
    assert_eq!(
        without_veriloga_source_directives(
            "* rules\n.va \"rules.vams\" CONNECTIONS\n\n".to_owned()
        ),
        None
    );
}
