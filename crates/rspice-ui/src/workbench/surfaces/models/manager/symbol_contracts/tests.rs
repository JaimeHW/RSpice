//! What the symbol-contract cache must notice.
//!
//! A stale row here is not a cosmetic lag: it states which pins a symbol has
//! and whether they match the provider's, so serving one for artwork that has
//! since been replaced is a wrong verdict rather than an old one.

use super::*;
use crate::state::{Cell, Library, LibraryManager, View};

/// A one-symbol design catalog whose symbol declares `family`.
///
/// Both catalogs this file builds take the same two mutations — one
/// `add_library`, one `get_library_mut` — so they carry the same content
/// revision, which is the point.
fn symbol_catalog(family: &str) -> LibraryManager {
    let mut libraries = LibraryManager::new();
    libraries.add_library(Library::new("sym"));
    let library = libraries
        .get_library_mut("sym")
        .expect("the library was just added");
    let mut cell = Cell::new("device");
    cell.metadata
        .insert("model.family".to_owned(), family.to_owned());
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    libraries
}

/// The families one frame's symbol registry reports, painted on `ctx` so that
/// successive calls see the same cache the real page would.
fn painted_families(ctx: &egui::Context, state: &mut AppState) -> Vec<String> {
    let mut families = Vec::new();
    let mut pending_actions = Vec::new();
    let app = ManagerRenderContext {
        state,
        pending_actions: &mut pending_actions,
    };
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            families = symbol_rows(ui, &app)
                .into_iter()
                .map(|row| row.family)
                .collect();
        });
    });
    families
}

#[test]
fn repainting_an_untouched_catalog_reads_no_symbol_twice() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state.library_manager = symbol_catalog("nch");

    crate::state::SYMBOL_VIEW_PARSES.with(|count| count.set(0));
    assert_eq!(painted_families(&ctx, &mut state), vec!["nch".to_owned()]);
    assert_eq!(
        crate::state::SYMBOL_VIEW_PARSES.with(std::cell::Cell::get),
        1,
        "the first frame is the one that reads the corpus"
    );

    crate::state::SYMBOL_VIEW_PARSES.with(|count| count.set(0));
    assert_eq!(painted_families(&ctx, &mut state), vec!["nch".to_owned()]);
    assert_eq!(
        crate::state::SYMBOL_VIEW_PARSES.with(std::cell::Cell::get),
        0,
        "nothing changed, so the second frame must read the cache"
    );
}

#[test]
fn a_catalog_replaced_at_the_same_revision_is_still_re_read() {
    // The hazard the key is content for. Opening a project, accepting a
    // recovery comparison, and restoring a design-history candidate all replace
    // the whole `LibraryManager` with one carrying whatever revision counter it
    // was serialized with — and two catalogs assembled by the same number of
    // mutations carry the same counter. A key made of the counter alone would
    // paint the first catalog's symbol contracts over the second's artwork.
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state.library_manager = symbol_catalog("nch");
    assert_eq!(painted_families(&ctx, &mut state), vec!["nch".to_owned()]);

    let replacement = symbol_catalog("pch");
    assert_eq!(
        replacement.revision(),
        state.library_manager.revision(),
        "the two catalogs must be indistinguishable by revision, or this test \
         proves nothing about the content half of the key"
    );
    state.library_manager = replacement;

    assert_eq!(painted_families(&ctx, &mut state), vec!["pch".to_owned()]);
}

// ---------------------------------------------------------------------------
// The detail cards' derivations, and the registry's grouping.
// ---------------------------------------------------------------------------

use crate::state::{
    GeneratedSymbolViews, ParameterInheritance, PortDirection, PropertyType, SymbolElectricalType,
    SymbolGraphicTemplate, SymbolIdentity, SymbolModelReference, SymbolNetlistBinding,
    SymbolParameterConstraints, SymbolParameterDefault, SymbolParameterField, SymbolParameterForm,
    SymbolParameterSection, SymbolParameterVisibility, SymbolPinSide, SymbolSourceContract,
};

/// A five-terminal op-amp symbol bound to a model, exactly as the create
/// dialog publishes one: pins in netlist order, a source contract carrying the
/// provider's ports, and an executable template naming both.
pub(in crate::workbench::surfaces::models::manager) fn bound_definition(
    cell: &str,
) -> ModelBoundSymbolDefinition {
    // The source contract requires an absolute path, and absoluteness is
    // judged by the syntax of any desktop host — so this is built from the
    // test's own working directory rather than spelled for one platform.
    let model = SymbolModelReference::new("vendor", "OPA189_A").with_source_path(
        std::env::current_dir()
            .expect("a working directory")
            .join("models/opa189.lib")
            .display()
            .to_string(),
    );
    let pins = [
        ("INP", SymbolElectricalType::Analog, PortDirection::In),
        ("INN", SymbolElectricalType::Analog, PortDirection::In),
        ("VSS", SymbolElectricalType::Power, PortDirection::Supply),
        ("OUT", SymbolElectricalType::Analog, PortDirection::Out),
        ("VDD", SymbolElectricalType::Power, PortDirection::Supply),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (name, electrical, direction))| {
        SymbolPinDefinition::new(name, electrical, direction, SymbolPinSide::Left, index + 1)
    })
    .collect::<Vec<_>>();
    let ports = pins.iter().map(SymbolPinDefinition::port_spec).collect();
    let form = SymbolParameterForm {
        revision: 1,
        sections: vec![SymbolParameterSection {
            key: "electrical".to_owned(),
            label: "Electrical parameters".to_owned(),
            help: "Model-safe electrical overrides.".to_owned(),
            fields: vec![SymbolParameterField {
                key: "gain".to_owned(),
                label: "Gain".to_owned(),
                help: "Closed-loop gain override.".to_owned(),
                property_type: PropertyType::Number,
                default: SymbolParameterDefault::Number {
                    engineering: "100".to_owned(),
                    unit: None,
                },
                unit: None,
                constraints: SymbolParameterConstraints {
                    minimum: Some("1".to_owned()),
                    maximum: Some("1Meg".to_owned()),
                    enum_values: Vec::new(),
                    max_length: None,
                },
                inheritance: ParameterInheritance::InstanceOverride,
                visibility: SymbolParameterVisibility::Visible,
                required: true,
                aliases: Vec::new(),
            }],
        }],
    };
    ModelBoundSymbolDefinition::new(
        SymbolIdentity::new("project_symbols", cell, 3, format!("symbol-{cell}")),
        SymbolSourceContract::model(model.clone(), ports),
        pins,
        SymbolGraphicTemplate::OperationalAmplifier5Pin,
        form,
        SymbolNetlistBinding {
            device_prefix: "X".to_owned(),
            model: Some(model),
            template: "X{name} {nodes} {model} {params}".to_owned(),
            parameter_order: vec!["gain".to_owned()],
        },
        GeneratedSymbolViews::default(),
    )
}

fn row_for(definition: ModelBoundSymbolDefinition) -> SymbolRow {
    SymbolRow {
        reference: CellViewRef::new(
            &definition.identity.library,
            &definition.identity.cell,
            "symbol",
        ),
        authority: SymbolRowAuthority::DesignLibrary { read_only: false },
        family: "vendor/OPA189_A".to_owned(),
        pins: definition
            .pins
            .iter()
            .map(|pin| pin.name.clone())
            .collect::<Vec<_>>(),
        form: "Electrical parameters".to_owned(),
        template: definition.netlist.template.clone(),
        status: SymbolStatus::Bound,
        definition: Some(definition),
        diagnostics: Vec::new(),
    }
}

fn checked(row: &SymbolRow) -> Vec<PinContractRow> {
    match pin_contract(row) {
        PinContract::Checked(rows) => rows,
        PinContract::NoProviderPorts(reason) => {
            panic!("the fixture names a provider, but the card says: {reason}")
        }
        PinContract::Legacy(_) => panic!("the fixture carries a typed definition"),
    }
}

/// The provider's ports, mutable, whichever contract shape carries them.
fn ports_mut(definition: &mut ModelBoundSymbolDefinition) -> &mut Vec<PortSpec> {
    match &mut definition.source {
        SymbolSourceContract::Model { ports, .. }
        | SymbolSourceContract::ExistingSchematicPins { ports, .. } => ports,
        SymbolSourceContract::BlankExplicitContract => {
            panic!("the fixture is a model-bound contract")
        }
    }
}

#[test]
fn an_aligned_contract_reports_every_position_aligned() {
    let row = row_for(bound_definition("precision_opamp"));
    let rows = checked(&row);
    assert_eq!(rows.len(), 5);
    for (index, pin) in rows.iter().enumerate() {
        assert_eq!(pin.order, index + 1, "the order column is the position");
        assert!(
            pin.check.is_aligned(),
            "position {} reported {}",
            index + 1,
            pin.check.label()
        );
    }
    assert_eq!(rows[0].pin.as_deref(), Some("INP"));
    assert_eq!(rows[0].provider_port.as_deref(), Some("INP in"));
    assert_eq!(rows[2].electrical, "power · supply");
}

#[test]
fn the_pin_card_and_the_save_path_agree_on_every_mutation() {
    // The card re-expresses `compare_source_ports` rather than calling it, so
    // this is what stops the two drifting: every way the save path refuses a
    // pin contract, the card must report at least one position that is not
    // aligned — and where the save path accepts, the card must report none.
    //
    // The mutations are the whole vocabulary of that comparison: a renamed
    // terminal, a re-directed one, a provider with a terminal the symbol does
    // not declare, and a symbol with one the provider does not.
    let mutations: [(&str, fn(&mut ModelBoundSymbolDefinition)); 5] = [
        ("untouched", |_| {}),
        ("provider terminal renamed", |definition| {
            ports_mut(definition)[2].name = "VEE".to_owned();
        }),
        ("provider terminal re-directed", |definition| {
            ports_mut(definition)[3].direction = PortDirection::In;
        }),
        ("provider declares one terminal more", |definition| {
            ports_mut(definition).push(PortSpec {
                name: "SHDN".to_owned(),
                direction: PortDirection::In,
            });
        }),
        ("provider declares one terminal fewer", |definition| {
            ports_mut(definition).pop();
        }),
    ];
    for (name, mutate) in mutations {
        let mut definition = bound_definition("precision_opamp");
        mutate(&mut definition);
        // Case matters to neither side, so a spelling change alone must not
        // move either verdict.
        let save_path_refuses = definition.validate().is_err();
        let row = row_for(definition);
        let card_refuses = checked(&row).iter().any(|pin| !pin.check.is_aligned());
        assert_eq!(
            card_refuses,
            save_path_refuses,
            "`{name}`: the card says {}, the save path says {}",
            if card_refuses { "refused" } else { "aligned" },
            if save_path_refuses {
                "refused"
            } else {
                "accepted"
            }
        );
    }
}

#[test]
fn a_terminal_spelled_in_another_case_is_still_the_same_terminal() {
    // `compare_source_ports` compares names case-insensitively, so a card that
    // compared them exactly would print a blocking mismatch about a symbol the
    // save path accepts.
    let mut definition = bound_definition("precision_opamp");
    ports_mut(&mut definition)[0].name = "inp".to_owned();
    assert!(definition.validate().is_ok());
    let row = row_for(definition);
    assert!(checked(&row).iter().all(|pin| pin.check.is_aligned()));
}

#[test]
fn a_position_only_one_side_declares_is_a_row_rather_than_a_dropped_terminal() {
    let mut definition = bound_definition("precision_opamp");
    ports_mut(&mut definition).push(PortSpec {
        name: "SHDN".to_owned(),
        direction: PortDirection::In,
    });
    let row = row_for(definition);
    let rows = checked(&row);
    assert_eq!(
        rows.len(),
        6,
        "the extra provider terminal gets its own line"
    );
    assert_eq!(rows[5].pin, None);
    assert_eq!(rows[5].provider_port.as_deref(), Some("SHDN in"));
    assert_eq!(rows[5].check.label(), "the symbol declares no pin here");
}

#[test]
fn a_review_only_contract_says_it_has_no_provider_rather_than_showing_a_blank_table() {
    let definition = ModelBoundSymbolDefinition::review_only("project_symbols", "sketch");
    let mut row = row_for(bound_definition("sketch"));
    row.definition = Some(definition);
    assert!(
        matches!(pin_contract(&row), PinContract::NoProviderPorts(_)),
        "an explicitly unbound contract has no ports, and the card has to say \
         so rather than draw five empty positions"
    );
}

#[test]
fn a_legacy_symbol_reports_its_drawn_pins_and_nothing_it_cannot_know() {
    let mut row = row_for(bound_definition("legacy"));
    row.definition = None;
    row.pins = vec!["A".to_owned(), "K".to_owned()];
    match pin_contract(&row) {
        PinContract::Legacy(pins) => assert_eq!(pins, vec!["A".to_owned(), "K".to_owned()]),
        _ => panic!("a symbol with no typed contract has nothing to check"),
    }
}

#[test]
fn every_token_of_the_emitted_line_is_accounted_for() {
    let definition = bound_definition("precision_opamp");
    let NetlistTemplate::Tokens(tokens) = netlist_template(&definition) else {
        panic!("the fixture's template is the validated grammar");
    };
    let spelled = tokens
        .iter()
        .map(|token| token.token.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        spelled,
        vec!["X{name}", "{nodes}", "{model}", "{params}"],
        "the table must account for the whole line, in emission order"
    );
    assert!(tokens.iter().all(|token| token.resolves));
    assert_eq!(tokens[1].value, "INP INN VSS OUT VDD");
    assert_eq!(tokens[1].owner, "schematic");
    assert_eq!(tokens[2].value, "OPA189_A");
    // The model is pinned to the source contract by `validate_netlist`, so no
    // placed instance can override it — which is what the owner column says.
    assert_eq!(tokens[2].owner, "symbol definition");
    assert_eq!(tokens[3].value, "gain");
    assert_eq!(tokens[3].owner, "parameter form");
}

#[test]
fn an_unbound_model_token_is_reported_as_resolving_from_nothing() {
    let mut definition = bound_definition("precision_opamp");
    definition.netlist.model = None;
    let NetlistTemplate::Tokens(tokens) = netlist_template(&definition) else {
        panic!("dropping the model does not change the template's grammar");
    };
    let model = tokens
        .iter()
        .find(|token| token.token == "{model}")
        .expect("the template names a model");
    assert!(
        !model.resolves,
        "a `{{model}}` with nothing behind it blocks netlisting, and the card \
         is where a reader finds out which token it was"
    );
}

#[test]
fn a_template_the_netlist_writer_refuses_produces_no_token_table() {
    // This page shows definitions that failed to validate on purpose, so the
    // template it holds may be one no instance line is ever emitted from. A
    // token table derived from it would describe a line that does not exist.
    let mut definition = bound_definition("precision_opamp");
    definition.netlist.template = "X{name} {model} {nodes}".to_owned();
    let NetlistTemplate::Invalid(reason) = netlist_template(&definition) else {
        panic!("the node and model tokens are positional; a swap is not a line");
    };
    assert!(
        reason.contains("{nodes} {model}"),
        "the card repeats the writer's own refusal, which was: {reason}"
    );
}

/// A design catalog with the three registry states a reader has to tell apart:
/// a symbol whose artwork agrees with its provider, one whose artwork does
/// not, and a legacy symbol with no typed contract at all.
pub(in crate::workbench::surfaces::models::manager) fn seed_symbol_registry(state: &mut AppState) {
    use crate::state::{PortSpec, SymbolDocument};

    state.library_manager = LibraryManager::new();
    state
        .library_manager
        .add_library(Library::new("project_symbols"));
    let library = state
        .library_manager
        .get_library_mut("project_symbols")
        .expect("the library was just added");

    for cell_name in ["precision_opamp", "buffer_opamp"] {
        let definition = bound_definition(cell_name);
        let mut view = View::new("symbol", ViewType::Symbol);
        definition
            .store_in_view(&mut view)
            .expect("a validated definition stores with its generated artwork");
        let mut cell = Cell::new(cell_name);
        cell.add_view(view);
        library.add_cell(cell);
    }

    // Same contract, artwork drawn with the terminals in another order — the
    // blocking state, and the reason the page checks rather than trusts.
    let definition = bound_definition("mismatched_opamp");
    let mut view = View::new("symbol", ViewType::Symbol);
    definition
        .store_in_view(&mut view)
        .expect("a validated definition stores with its generated artwork");
    let scrambled = ["OUT", "INP", "INN", "VDD", "VSS"]
        .into_iter()
        .map(|name| PortSpec {
            name: name.to_owned(),
            direction: PortDirection::InOut,
        })
        .collect::<Vec<_>>();
    SymbolDocument::generated_from_ports(&scrambled)
        .store_in_view(&mut view)
        .expect("generated artwork is storable");
    let mut cell = Cell::new("mismatched_opamp");
    cell.add_view(view);
    library.add_cell(cell);

    // A legacy symbol: artwork, a declared family in the cell's metadata, and
    // no typed contract anywhere.
    let mut view = View::new("symbol", ViewType::Symbol);
    SymbolDocument::generated_from_ports(&[
        PortSpec {
            name: "A".to_owned(),
            direction: PortDirection::InOut,
        },
        PortSpec {
            name: "K".to_owned(),
            direction: PortDirection::InOut,
        },
    ])
    .store_in_view(&mut view)
    .expect("generated artwork is storable");
    let mut cell = Cell::new("legacy_diode");
    cell.metadata
        .insert("model.family".to_owned(), "1N4148".to_owned());
    cell.add_view(view);
    library.add_cell(cell);
}

/// The registry rows one frame reports, painted on `ctx`.
fn painted_rows(ctx: &egui::Context, state: &mut AppState) -> Vec<SymbolRow> {
    let mut rows = Vec::new();
    let mut pending_actions = Vec::new();
    let app = ManagerRenderContext {
        state,
        pending_actions: &mut pending_actions,
    };
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            rows = symbol_rows(ui, &app);
        });
    });
    rows
}

#[test]
fn the_registry_states_a_verdict_per_symbol_rather_than_one_for_the_library() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    seed_symbol_registry(&mut state);
    let rows = painted_rows(&ctx, &mut state);

    let verdict = |cell: &str| {
        rows.iter()
            .find(|row| row.reference.cell == cell)
            .unwrap_or_else(|| panic!("{cell} is in the registry"))
            .status
    };
    assert_eq!(verdict("precision_opamp"), SymbolStatus::Bound);
    assert_eq!(verdict("mismatched_opamp"), SymbolStatus::PinMismatch);
    assert_eq!(
        verdict("legacy_diode"),
        SymbolStatus::Review,
        "a symbol with no typed contract is work, not a settled state"
    );
    assert!(
        SymbolStatus::Review.needs_attention() && !SymbolStatus::Bound.needs_attention(),
        "the footer's count is of the rows a reader still has to act on"
    );
}

#[test]
fn a_bound_signed_package_is_grouped_under_its_own_band() {
    // The only shape that draws a band, built the way the product builds it:
    // a publisher-signed technology archive installed into the registry and
    // pinned by the project, not a hand-assembled row.
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    seed_symbol_registry(&mut state);
    state.provision_test_project_symbol_technology_contract();
    let rows = painted_rows(&ctx, &mut state);

    let project = project_row_count(&rows);
    assert_eq!(project, 4, "the seeded project library contributes four");
    assert!(
        rows.len() > project,
        "the signed package contributes symbols of its own"
    );
    assert!(
        rows[..project]
            .iter()
            .all(|row| row.corpus() == SymbolCorpus::Project)
            && rows[project..]
                .iter()
                .all(|row| row.corpus() == SymbolCorpus::SignedTechnology),
        "grouping has to be one contiguous split, or the band is a header over \
         a mixture"
    );
    let band = technology_group_band(&rows).expect("two corpora, so one band");
    assert!(
        band.contains(&format!("{} TECHNOLOGY SYMBOL", rows.len() - project))
            && band.ends_with("READ-ONLY"),
        "the band counts the symbols under it and states that none of them may \
         be edited here; it said: {band}"
    );
}

#[test]
fn a_project_only_registry_has_no_group_band_to_draw() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    seed_symbol_registry(&mut state);
    let rows = painted_rows(&ctx, &mut state);
    assert_eq!(project_row_count(&rows), rows.len());
    assert_eq!(
        technology_group_band(&rows),
        None,
        "a band naming the technology corpus when there is no technology corpus \
         is a header over nothing"
    );
}

#[test]
fn the_two_corpora_are_grouped_and_the_band_names_the_technology() {
    // Assembled directly: the signed half of the registry comes from a
    // publisher-signed package, and what is being checked here is the ordering
    // and the band, neither of which the package's signature participates in.
    let project = |cell: &str| SymbolRow {
        reference: CellViewRef::new("project_symbols", cell, "symbol"),
        authority: SymbolRowAuthority::DesignLibrary { read_only: false },
        family: "vendor/OPA189_A".to_owned(),
        pins: Vec::new(),
        form: "not defined".to_owned(),
        template: "not defined".to_owned(),
        status: SymbolStatus::Bound,
        definition: None,
        diagnostics: Vec::new(),
    };
    let technology = |cell: &str| SymbolRow {
        reference: CellViewRef::new("demo180", cell, "symbol"),
        authority: SymbolRowAuthority::SignedTechnology {
            technology_name: "DEMO180".to_owned(),
            revision: "2.3.1".to_owned(),
            manifest_digest: crate::product::ContentDigest::from_bytes([7; 32]),
            archive_digest: crate::product::ContentDigest::from_bytes([9; 32]),
        },
        ..project(cell)
    };
    // Deliberately interleaved, and with the technology library sorting first
    // by name: grouping has to beat the library ordering, or the band cannot
    // be one contiguous split.
    let rows = {
        let mut rows = vec![
            technology("nch_core"),
            project("opamp_5pin"),
            technology("pch_core"),
            project("comp_3pin"),
        ];
        rows.sort_by(|left, right| {
            left.corpus()
                .cmp(&right.corpus())
                .then_with(|| left.reference.cell.cmp(&right.reference.cell))
        });
        rows
    };

    assert_eq!(project_row_count(&rows), 2);
    assert_eq!(
        rows.iter()
            .map(|row| row.reference.cell.as_str())
            .collect::<Vec<_>>(),
        vec!["comp_3pin", "opamp_5pin", "nch_core", "pch_core"],
        "the project's own forms come first, whatever the libraries are called"
    );
    assert_eq!(
        technology_group_band(&rows).as_deref(),
        Some("DEMO180 2.3.1 · 2 TECHNOLOGY SYMBOLS · READ-ONLY"),
        "the band names the technology, the revision the project pinned, how \
         many symbols it contributes, and that none may be edited here"
    );
}

/// A schematic holding `components`, saved as the active sheet.
fn place(state: &mut AppState, components: Vec<crate::state::Component>) {
    use crate::state::{ComponentType, Point};

    state.workspace.ensure_active_buffer();
    let mut schematic = state
        .workspace
        .active_schematic()
        .cloned()
        .expect("an active schematic exists");
    schematic.components = components;
    // Something unrelated on the sheet, so a card that counted every instance
    // rather than this symbol's would be caught.
    let mut resistor =
        crate::state::Component::new(9_000, ComponentType::Resistor, Point::origin());
    resistor.name = "R1".to_owned();
    resistor.value = "1k".to_owned();
    schematic.components.push(resistor);
    state.workspace.save_active_schematic(&schematic);
}

#[test]
fn the_instances_card_counts_this_symbol_by_both_routes_that_reach_it() {
    use crate::state::{Component, ComponentType, LibraryCellInstance, Point};

    let ctx = egui::Context::default();
    let mut state = AppState::default();
    seed_symbol_registry(&mut state);

    let mut instance = Component::new(1, ComponentType::Nmos, Point::new(4, 6));
    instance.name = "X1".to_owned();
    instance.library_cell = Some(LibraryCellInstance::new(
        "project_symbols",
        "precision_opamp",
        "symbol",
    ));
    let mut by_model = Component::new(2, ComponentType::Nmos, Point::new(8, 2));
    by_model.name = "M2".to_owned();
    by_model.params = "model=OPA189_A".to_owned();
    let mut elsewhere = Component::new(3, ComponentType::Nmos, Point::new(1, 1));
    elsewhere.name = "M3".to_owned();
    elsewhere.params = "model=nch".to_owned();
    place(&mut state, vec![instance, by_model, elsewhere]);

    let rows = painted_rows(&ctx, &mut state);
    let row = rows
        .iter()
        .find(|row| row.reference.cell == "precision_opamp")
        .expect("the seeded symbol");

    let mut pending_actions = Vec::new();
    let app = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending_actions,
    };
    let mut instances = None;
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            instances = Some(placed_instances(ui, &app, row));
        });
    });
    let instances = instances.expect("one frame");

    assert_eq!(instances.total, 2, "two instances reach this symbol");
    assert_eq!(
        instances
            .shown
            .iter()
            .map(|instance| (instance.designator.as_str(), instance.binding))
            .collect::<Vec<_>>(),
        vec![
            ("X1", InstanceBinding::CellView),
            ("M2", InstanceBinding::Model),
        ],
        "an instance that names the library and cell was placed from the \
         symbol; one that names only the model reaches the same provider \
         without following the symbol's pin contract, and the card says which"
    );
    assert!(
        instances.shown[0].location.contains("(4, 6)"),
        "an instance a reader has to go and find is located, not just counted"
    );
}

#[test]
fn a_repainted_instances_card_walks_the_sheet_once() {
    // The card is derived for the selected symbol only, but "one symbol" is
    // still a pass over every instance on the sheet, and it repaints sixty
    // times a second. The key is made of what the pass reads, so an untouched
    // sheet is answered from the cache — and a sheet that changed is not.
    use crate::state::{Component, ComponentType, LibraryCellInstance, Point};

    let ctx = egui::Context::default();
    let mut state = AppState::default();
    seed_symbol_registry(&mut state);
    let mut instance = Component::new(1, ComponentType::Nmos, Point::new(4, 6));
    instance.name = "X1".to_owned();
    instance.library_cell = Some(LibraryCellInstance::new(
        "project_symbols",
        "precision_opamp",
        "symbol",
    ));
    place(&mut state, vec![instance]);

    let rows = painted_rows(&ctx, &mut state);
    let row = rows
        .iter()
        .find(|row| row.reference.cell == "precision_opamp")
        .expect("the seeded symbol")
        .clone();

    let count = |state: &mut AppState| {
        let mut pending_actions = Vec::new();
        let app = ManagerRenderContext {
            state,
            pending_actions: &mut pending_actions,
        };
        let mut total = 0;
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                total = placed_instances(ui, &app, &row).total;
            });
        });
        total
    };

    assert_eq!(count(&mut state), 1);
    assert_eq!(count(&mut state), 1, "the cached answer is the same answer");

    let mut second = crate::state::Component::new(2, ComponentType::Nmos, Point::new(9, 9));
    second.name = "X2".to_owned();
    second.library_cell = Some(LibraryCellInstance::new(
        "project_symbols",
        "precision_opamp",
        "symbol",
    ));
    let existing = state
        .workspace
        .active_schematic()
        .cloned()
        .expect("an active schematic")
        .components;
    place(&mut state, existing.into_iter().chain([second]).collect());
    assert_eq!(
        count(&mut state),
        2,
        "an instance placed since the last frame must appear; a cache that \
         missed it would state a count the sheet contradicts"
    );
}
