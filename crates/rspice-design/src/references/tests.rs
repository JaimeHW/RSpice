use super::*;
use crate::hierarchy::{HierarchyBindingStatus, ResolvedHierarchyBinding};
use rspice_design_model::Point;

#[test]
fn shared_master_renames_prepare_probes_without_mutating_sources() {
    let root = CellViewRef::new("work", "top", "schematic");
    let child = CellViewRef::new("work", "amp", "schematic");
    let before_top = [(1, "Xleft"), (2, "Xright")].map(|(id, name)| {
        Component::new(id, ComponentType::CellInstance, Point::origin())
            .with_name_value(name, "amp")
    });
    let mut after_top = before_top.clone();
    after_top[0].name = "Xright".to_owned();
    after_top[1].name = "Xleft".to_owned();
    let before_child = [
        Component::new(1, ComponentType::VoltageSource, Point::origin())
            .with_name_value("1", "DC 1"),
    ];
    let mut after_child = before_child.clone();
    after_child[0].name = "2".to_owned();
    let components = |reference: &CellViewRef| {
        if *reference == root {
            Some(ReferenceComponents {
                before: &before_top,
                after: &after_top,
            })
        } else if *reference == child {
            Some(ReferenceComponents {
                before: &before_child,
                after: &after_child,
            })
        } else {
            None
        }
    };
    let resolution = HierarchyResolution {
        bindings: vec![ResolvedHierarchyBinding {
            reference: child.clone(),
            purpose: String::new(),
            view_search_order: Vec::new(),
            stop_view: None,
            model_section: String::new(),
            status: HierarchyBindingStatus::Resolved,
            instance_count: 2,
            instance_paths: vec!["/Xleft".to_owned(), "/Xright".to_owned()],
            used_review_fallback: false,
            diagnostic: None,
            warnings: Vec::new(),
        }],
        total_instances: 3,
        resolved_instances: 3,
        configuration_id: None,
        configuration_revision: None,
        configuration_digest: None,
    };
    for (emitted, leaf) in [(false, "1"), (true, "V1")] {
        let paths = hierarchy_reference_paths(&root, &resolution, &components, emitted).unwrap();
        assert_eq!(paths.len(), 4);
        assert!(
            paths
                .iter()
                .any(|(from, to)| from.to_string() == format!("/Xleft/{leaf}")
                    && to.to_string() == if emitted { "/Xright/V2" } else { "/Xright/2" })
        );
    }
    let paths = hierarchy_reference_paths(&root, &resolution, &components, true).unwrap();
    let probes = [
        SchematicProbe::new(
            1,
            Point::origin(),
            "I(/Xleft/V1)",
            Some("I(/Xleft/V1)".to_owned()),
        )
        .unwrap(),
        SchematicProbe::new(
            2,
            Point::origin(),
            "supply",
            Some("@/Xright/V1[i]".to_owned()),
        )
        .unwrap(),
    ];
    let rewritten = remap_schematic_probes(&probes, &paths).unwrap().unwrap();
    assert_eq!(rewritten[0].reference, "I(/Xright/V2)");
    assert_eq!(
        rewritten[0].source_expression.as_deref(),
        Some("I(/Xright/V2)")
    );
    assert_eq!(rewritten[1].reference, "supply");
    assert_eq!(
        rewritten[1].source_expression.as_deref(),
        Some("@/Xleft/V2[i]")
    );
    let mut invalid = probes.clone();
    invalid[1].id = 0;
    assert_eq!(
        remap_schematic_probes(&invalid, &paths).unwrap_err(),
        "probe identity must not be zero"
    );
    assert_eq!(invalid[0], probes[0]);
    assert_eq!(invalid[1].source_expression, probes[1].source_expression);
    assert_eq!(before_top[0].name, "Xleft");
    assert_eq!(before_child[0].name, "1");
}
