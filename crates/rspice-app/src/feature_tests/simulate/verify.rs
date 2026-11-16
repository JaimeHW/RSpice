//! The Verify surface over the whole application: each view named by its
//! label, a stored session reopening on its view, and the seeded pages' table
//! grammar.

use crate::workbench::RSpiceApp;
use rspice_app_simulate::workbench::surfaces::verify::toolbar::TOOLBAR_H;
use rspice_app_state::state::VerificationPage;

/// One name per view: the inspector's title and the view's toolbar title are
/// both the view's label, the name its menu row, navigator row and palette
/// entry carry too.
#[test]
fn the_inspector_and_the_toolbar_name_each_view_by_its_label() {
    use rspice_app_simulate::workbench::surfaces::verify::raster::{
        VerifyStage, page_input, page_pass,
    };
    use rspice_app_workbench::workbench::docks::inspector::header;

    for page in VerificationPage::NAVIGATION {
        let mut app = RSpiceApp::test_instance();
        app.open(page);
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut headings = Vec::new();
        for _ in 0..2 {
            let (output, _) = page_pass(&ctx, page_input(Vec::new()), |ui| {
                header(ui, &mut app);
                app.draw(ui);
            });
            headings = output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Heading)
                // The toolbar's heading carries its title as its value.
                .map(|(_, node)| {
                    (
                        node.level(),
                        node.label().or(node.value()).map(str::to_owned),
                    )
                })
                .collect::<Vec<_>>();
        }
        let label = Some(page.label().to_owned());
        assert!(
            headings.contains(&(Some(2), label.clone())),
            "{page:?}: the inspector's title among {headings:?}"
        );
        assert!(
            headings.contains(&(None, label)),
            "{page:?}: the toolbar's title among {headings:?}"
        );
    }
}

/// The workspace has one name. The rail, the document bar, the title, the
/// phone's tab bar and the surface catalog all read the workspace's label at
/// every width, and the navigator's header names what it lists, as every
/// other workspace's does.
#[test]
fn every_surface_that_names_the_workspace_reads_verify() {
    use crate::feature_tests::replay::Replay;
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::host::{DocumentHost, Host};
    use rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId;

    assert_eq!(Workspace::Verify.label(), "Verify");
    assert_eq!(SurfaceId::Verify.label(), "Verify");
    for size in [
        egui::vec2(1440.0, 900.0),
        egui::vec2(820.0, 1180.0),
        egui::vec2(390.0, 844.0),
    ] {
        let mut replay = Replay::new().sized(size);
        replay.app.state.workbench.activate(Workspace::Verify);
        let controls = replay.frames(Vec::new());
        let old_name = controls
            .iter()
            .filter(|control| {
                [&control.label, &control.value, &control.description]
                    .iter()
                    .any(|text| text.contains("Verification"))
            })
            .map(|control| (control.role, control.label.as_str()))
            .collect::<Vec<_>>();
        assert!(old_name.is_empty(), "{size:?}: {old_name:?}");
        assert!(
            controls.iter().any(|control| control.label == "Verify"),
            "{size:?}: nothing names the open workspace"
        );
        let app = &replay.app;
        assert_eq!(app.title_cell(Workspace::Verify), "Verify");
        assert_eq!(app.navigator_section(Workspace::Verify).title, "Views");
        assert_eq!(
            app.state
                .workspace_documents(Workspace::Verify)
                .iter()
                .map(|document| document.label.as_str())
                .collect::<Vec<_>>(),
            ["Verify"]
        );
    }
}

/// A stored session reopens on the Verify view it was left on, stored and
/// read back the way the application does it.
#[test]
fn a_stored_session_reopens_on_its_verify_view() {
    #[derive(Default)]
    struct MemoryStorage(std::collections::HashMap<String, String>);

    impl eframe::Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.to_owned(), value);
        }

        fn remove_string(&mut self, key: &str) {
            self.0.remove(key);
        }

        fn flush(&mut self) {}
    }

    for page in VerificationPage::NAVIGATION {
        let mut app = RSpiceApp::test_instance();
        app.state.simulate.verification_page = page;
        let mut storage = MemoryStorage::default();
        app.state.store_session(&mut storage);
        let restored = crate::workbench::app_state::AppState::restore_eframe_session(&storage)
            .expect("RSpice restores a session it just stored");
        assert_eq!(restored.simulate.verification_page, page);
    }
}

/// The shell's filter field narrows the open view's rows, so it names them:
/// specifications on the views listed by specification, checks on the rest.
#[test]
fn the_filter_field_names_the_rows_of_the_open_view() {
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::host::Host;

    let mut app = RSpiceApp::test_instance();
    for (page, placeholder) in [
        (
            VerificationPage::Specifications,
            "Filter specifications\u{2026}",
        ),
        (VerificationPage::Corners, "Filter specifications\u{2026}"),
        (
            VerificationPage::MonteCarlo,
            "Filter specifications\u{2026}",
        ),
        (VerificationPage::Checks, "Filter checks\u{2026}"),
        (VerificationPage::Reference, "Filter checks\u{2026}"),
    ] {
        app.state.simulate.verification_page = page;
        assert_eq!(
            app.navigator_section(Workspace::Verify).placeholder,
            placeholder,
            "{page:?}"
        );
    }
    assert_eq!(
        app.navigator_section(Workspace::Simulate).placeholder,
        "Filter setup\u{2026}"
    );
}

/// Every seeded page opens with a 36 px toolbar whose title starts on the
/// panel inset, then a table of 24 px head and rows directly under it, with
/// every right-aligned column ending on one edge and nothing past the insets.
#[test]
fn seeded_verify_pages_follow_the_table_grammar() {
    use super::verify_fixtures::{corner_app, monte_carlo_app, soa_app};
    use rspice_app_simulate::workbench::surfaces::verify::raster::{PAGE_WIDTH, page_geometry};

    for (page, app) in [
        (VerificationPage::Specifications, corner_app()),
        (VerificationPage::MonteCarlo, monte_carlo_app()),
        (VerificationPage::Corners, corner_app()),
        (VerificationPage::Checks, soa_app()),
    ] {
        let geometry = page_geometry(app, page);
        let toolbar = geometry.one("toolbar").expect("toolbar");
        assert_eq!(toolbar.height(), TOOLBAR_H, "{page:?}");
        assert_eq!(toolbar.left(), 0.0, "{page:?}");
        assert_eq!(
            geometry.one("toolbar.title").expect("title").left(),
            10.0,
            "{page:?}"
        );
        let head = geometry.one("table.head").expect("table head");
        assert_eq!(head.height(), 24.0, "{page:?}");
        // Band rows naming runs of columns are the head's own top rows.
        let bands = geometry.all("table.band");
        assert_eq!(
            bands.first().map_or(head.top(), |band| band.top()),
            toolbar.bottom(),
            "{page:?}: nothing between toolbar and table"
        );
        if let Some(band) = bands.last() {
            assert_eq!(head.top(), band.bottom(), "{page:?}");
        }
        let rows = geometry.all("table.row");
        assert!(!rows.is_empty(), "{page:?}");
        assert!(rows.iter().all(|row| row.height() == 24.0), "{page:?}");
        assert_eq!(rows[0].top(), head.bottom(), "{page:?}");
        for (column, edges) in geometry.right_edges() {
            assert_eq!(
                edges.len(),
                1,
                "{page:?} {column}: numbers end on one edge, {edges:?}"
            );
        }
        assert_eq!(
            geometry.outside_insets(PAGE_WIDTH),
            Vec::<String>::new(),
            "{page:?}"
        );
        assert!(
            geometry.one("toolbar.summary").is_some(),
            "{page:?} states a summary"
        );
    }
}
