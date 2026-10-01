//! Persisted catalog compatibility and revision boundaries used by project saves.

use super::ProjectLibraries as LibraryManager;

const JSON: &str = concat!(
    r#"{"libraries":{"work":{"name":"work","path":null,"cells":{"amp":{"name":"amp","views":{"schematic":{"name":"schematic","view_type":"Schematic","file_path":"model.sp","modified_time":42,"modified":true,"is_open":true,"metadata":{"k":"v"}}},"description":"amplifier","category":"analog","expanded":true,"metadata":{}}},"technology":"pdk","read_only":false,"expanded":true,"metadata":{}}},"#,
    r#""selected_library":"work","selected_cell":"amp","selected_view":"schematic","filter_text":"amp","show_read_only":false,"revision":17}"#,
);

const RON: &str = concat!(
    r#"(libraries:{"work":(name:"work",path:None,cells:{"amp":(name:"amp",views:{"schematic":(name:"schematic",view_type:Schematic,file_path:Some("model.sp"),modified_time:Some(42),modified:true,is_open:true,metadata:{"k":"v"})},description:"amplifier",category:"analog",expanded:true,metadata:{})},technology:"pdk",read_only:false,expanded:true,metadata:{})},"#,
    r#"selected_library:Some("work"),selected_cell:Some("amp"),selected_view:Some("schematic"),filter_text:"amp",show_read_only:false,revision:17)"#,
);

#[test]
fn catalog_preserves_project_json_and_session_ron_contracts() {
    let json: LibraryManager = serde_json::from_str(JSON).unwrap();
    let ron: LibraryManager = ron::from_str(RON).unwrap();
    assert_eq!(serde_json::to_string(&json).unwrap(), JSON);
    assert_eq!(ron::to_string(&json).unwrap(), RON);
    assert_eq!(serde_json::to_string(&ron).unwrap(), JSON);
    let named: LibraryManager = ron::from_str(&format!("LibraryManager{RON}")).unwrap();
    assert_eq!(serde_json::to_string(&named).unwrap(), JSON);

    let legacy = JSON.replace(",\"revision\":17", "");
    let restored: LibraryManager = serde_json::from_str(&legacy).unwrap();
    assert_eq!(restored.revision(), 0);
    let legacy = RON.replace(",revision:17", "");
    assert_eq!(
        ron::from_str::<LibraryManager>(&legacy).unwrap().revision(),
        0
    );
    assert!(
        serde_json::from_str::<LibraryManager>(
            &JSON.replace("\"revision\":17", "\"revision\":17,\"revision\":18")
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<LibraryManager>(&JSON.replace("\"filter_text\":\"amp\",", ""))
            .is_err()
    );
    let mut extended: serde_json::Value = serde_json::from_str(JSON).unwrap();
    extended["future_field"] = serde_json::json!(true);
    let restored: LibraryManager = serde_json::from_value(extended).unwrap();
    assert_eq!(serde_json::to_string(&restored).unwrap(), JSON);
}

#[test]
fn catalog_revisions_distinguish_edits_runtime_projection_and_snapshot_replacement() {
    let snapshot: LibraryManager = serde_json::from_str(JSON).unwrap();
    let mut manager = snapshot.clone();
    assert!(manager.edit_library("missing").is_none());
    assert_eq!(manager.revision(), 18);
    assert!(manager.remove_library("missing").is_none());
    assert_eq!(manager.revision(), 19);
    assert!(manager.set_view_modified_runtime("work", "amp", "schematic", false));
    manager.sanitize_views_for_persistence();
    assert_eq!(manager.revision(), 19);
    let view = manager.current_view().unwrap();
    assert!(!view.modified && !view.is_open);
    assert!(view.file_path.is_none() && view.modified_time.is_none());
    assert_eq!(snapshot.revision(), 17);
    assert!(snapshot.current_view().unwrap().modified);

    manager.filter_text = "local filter".into();
    manager.show_read_only = true;
    assert_eq!(
        manager.replace_catalog_from_snapshot(&snapshot).unwrap(),
        20
    );
    assert_eq!(
        manager.selected_lcv_path().as_deref(),
        Some("work/amp/schematic")
    );
    assert_eq!(manager.filter_text, "local filter");
    assert!(manager.show_read_only);
    let exhausted = JSON.replace("\"revision\":17", &format!("\"revision\":{}", u64::MAX));
    let mut manager: LibraryManager = serde_json::from_str(&exhausted).unwrap();
    assert!(manager.replace_catalog_from_snapshot(&snapshot).is_err());
    assert_eq!(serde_json::to_string(&manager).unwrap(), exhausted);
    assert!(manager.edit_library("missing").is_none());
    assert_eq!(manager.revision(), 0);
}
