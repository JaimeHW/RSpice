//! The project pages' catalog and the wire spellings older sessions saved
//! them under.

use std::collections::HashSet;

use super::*;

#[test]
fn project_page_catalog_is_the_complete_five_tab_contract() {
    assert_eq!(
        ProjectPage::ALL,
        [
            ProjectPage::Overview,
            ProjectPage::Library,
            ProjectPage::Configuration,
            ProjectPage::Dependencies,
            ProjectPage::Recovery,
        ]
    );
    assert_eq!(ProjectPage::default(), ProjectPage::Overview);
    assert_eq!(
        ProjectPage::ALL.map(ProjectPage::label),
        [
            "Overview",
            "Library",
            "Configuration",
            "Dependencies",
            "Recovery",
        ]
    );
    assert_eq!(
        ProjectPage::ALL.into_iter().collect::<HashSet<_>>().len(),
        ProjectPage::ALL.len(),
        "the project tab catalog must not contain duplicate routes"
    );
}

#[test]
fn legacy_project_pages_restore_to_their_supported_successors() {
    for (serialized, expected) in [
        (r#""Dashboard""#, ProjectPage::Overview),
        (r#""Activity""#, ProjectPage::Overview),
        (r#""Technology""#, ProjectPage::Dependencies),
    ] {
        assert_eq!(
            serde_json::from_str::<ProjectPage>(serialized)
                .expect("legacy project page remains decodable"),
            expected
        );
    }

    for page in ProjectPage::ALL {
        let encoded = serde_json::to_string(&page).expect("project page serializes");
        let restored: ProjectPage =
            serde_json::from_str(&encoded).expect("canonical project page restores");
        assert_eq!(restored, page);
    }
}
