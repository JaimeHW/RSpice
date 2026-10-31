//! A binned family is one catalog row, named as an instance binds it.
//!
//! A foundry kit's `nch.1` … `nch.48` were 48 rows, and binding from one
//! pinned the instance to that bin, which then ran at every size. The row is
//! the family now: its bins are its detail, the instances its bins run for are
//! counted against it, and Bind gives an instance `nch` with its library.

use super::*;
use crate::workbench::models::tests::kit_project;

/// The catalog opens a family on its first bin, shows another bin from the
/// family's own detail, and in either case binds the selected instance to
/// `nch` with its library, unpinning the bin it named.
#[test]
fn binding_from_the_catalog_binds_the_family_with_its_library() {
    let mut app = RSpiceApp::test_instance();
    kit_project(&mut app.state.model, &mut app.state.models, &["nch.1"]);
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(1);
    let mut studio = ModelsStudio::open(app);
    assert!(
        studio.announces(|label| label == "nch · 2 bins in kit"),
        "the family is one row; the page announces {:#?}",
        studio.announcements()
    );
    assert!(
        !studio.announces(|label| label.starts_with("nch.1 in") || label.starts_with("nch.2 in")),
        "no bin is a row of its own; the page announces {:#?}",
        studio.announcements()
    );
    assert_eq!(
        studio.app.state.models.selected_model.as_deref(),
        Some("nch.1"),
        "a family's selection is a card, so every reader of it still reads one"
    );
    assert!(
        studio
            .painted
            .iter()
            .any(|(text, _)| text.ends_with("names bin nch.1")),
        "where-used names the bin M1 is pinned to"
    );

    studio.click(|label| label.starts_with("nch.2 · L "));
    assert_eq!(
        studio.app.state.models.selected_model.as_deref(),
        Some("nch.2")
    );
    assert!(
        studio.announces(|label| label == "nch · 2 bins in kit"),
        "showing another bin keeps the family's row selected"
    );

    studio.click(|label| label == "Bind to M1…");
    let params = &studio
        .app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == 1)
        .expect("M1")
        .params;
    assert!(
        params.ends_with("model=nch model_library=kit"),
        "Bind gives the instance the family, not the bin on show: {params}"
    );
}
