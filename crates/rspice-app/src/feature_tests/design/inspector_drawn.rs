//! The inspector drawn through a whole application: the sheet an inspected object names and
//! the models a bound instance is offered.

use crate::state::{Point, SchematicProbe};
use crate::workbench::RSpiceApp;
use crate::workbench::design::inspector::component::{bound_model_choices, catalog_model_location};
use crate::workbench::design::inspector::tests::{app_with_model_bound_instance, painted_panel};
use crate::workbench::design::inspector::*;

/// One retained probe on a cell view whose sheet catalog holds `sheets`
/// sheets. With two, the probe is assigned to the second one.
#[cfg(not(target_arch = "wasm32"))]
fn app_with_probe_on_last_sheet(sheets: usize) -> (RSpiceApp, u64) {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    const PROBE: u64 = 77;
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .probes
        .push(
            SchematicProbe::new(
                PROBE,
                Point::new(10, 20),
                "V(OUT)",
                Some("V(OUT)".to_owned()),
            )
            .expect("the fixture probe is well formed"),
        );
    app.state.sync_active_schematic_to_workspace();

    let key = app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = app
        .state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input stage", [PROBE])
        .expect("the fixture cell view takes a sheet catalog");
    if sheets > 1 {
        let catalog = app
            .state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("the catalog was just bootstrapped");
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Output stage".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .expect("a second sheet");
        catalog
            .assign_objects(catalog.revision(), second, [PROBE])
            .expect("the probe takes its assignment");
        catalog
            .set_active(first)
            .expect("the first sheet is active");
    }
    (app, PROBE)
}

/// An inspected object states which sheet holds it, by the catalog's own page
/// number and name — and says nothing at all where there is one sheet, since
/// naming it would only repeat the document.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_inspected_object_names_its_sheet_only_above_one_sheet() {
    let (mut multi, probe) = app_with_probe_on_last_sheet(2);
    let painted = painted_panel(|ui| probe_panel(ui, &mut multi.design_cx(), probe));
    assert!(
        painted.contains("Sheet 2 \u{00b7} Output stage"),
        "the row names the ordinal and the name the catalog carries: {painted}"
    );

    let (mut single, probe) = app_with_probe_on_last_sheet(1);
    let painted = painted_panel(|ui| probe_panel(ui, &mut single.design_cx(), probe));
    assert!(
        !painted.contains("Input stage"),
        "a single-sheet cell view is its sheet, so the row is absent: {painted}"
    );
}

#[test]
fn model_choices_stay_inside_the_bound_library_and_device_family() {
    let (app, id) = app_with_model_bound_instance();
    let component = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("bound component");
    let choices = bound_model_choices(&app.state.model, component, "OPA189_A");
    let values = choices
        .iter()
        .map(|(value, _)| value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(values, ["OPA189_A", "OPA189_B"]);
    assert_eq!(
        catalog_model_location(&app.state.model, component),
        Some(("vendor_analog".to_owned(), "OPA189_A".to_owned()))
    );
}
