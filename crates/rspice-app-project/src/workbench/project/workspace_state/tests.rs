//! The wire spellings the project pages are saved under.

use super::*;

#[test]
fn project_pages_round_trip() {
    for page in ProjectPage::ALL {
        let encoded = serde_json::to_string(&page).expect("project page serializes");
        let restored: ProjectPage =
            serde_json::from_str(&encoded).expect("canonical project page restores");
        assert_eq!(restored, page);
    }
}
