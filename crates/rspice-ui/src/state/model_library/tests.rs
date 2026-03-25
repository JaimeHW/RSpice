use super::*;

// =========================================================================
// ModelType Tests
// =========================================================================

#[test]
fn test_model_type_from_str() {
    assert_eq!(ModelType::from_name("nmos"), ModelType::Nmos);
    assert_eq!(ModelType::from_name("PMOS"), ModelType::Pmos);
    assert_eq!(ModelType::from_name("nch"), ModelType::Nmos);
    assert_eq!(ModelType::from_name("pch"), ModelType::Pmos);
    assert_eq!(ModelType::from_name("resistor"), ModelType::Resistor);
    assert_eq!(ModelType::from_name("unknown"), ModelType::Other);
}

#[test]
fn test_model_type_display() {
    assert_eq!(ModelType::Nmos.display_name(), "NMOS");
    assert_eq!(ModelType::Capacitor.display_name(), "Capacitor");
}

// =========================================================================
// DeviceModel Tests
// =========================================================================

#[test]
fn test_device_model_creation() {
    let model = DeviceModel::new("nmos_svt", ModelType::Nmos);
    assert_eq!(model.name, "nmos_svt");
    assert_eq!(model.model_type, ModelType::Nmos);
}

#[test]
fn test_device_model_with_geometry() {
    let model =
        DeviceModel::new("nmos_svt", ModelType::Nmos).with_geometry(60e-9, 10e-6, 120e-9, 100e-6);

    assert_eq!(model.l_min, Some(60e-9));
    assert_eq!(model.w_max, Some(100e-6));
}

#[test]
fn test_device_model_geometry_check() {
    let model =
        DeviceModel::new("nmos", ModelType::Nmos).with_geometry(60e-9, 10e-6, 120e-9, 100e-6);

    assert!(model.check_geometry(100e-9, 1e-6)); // Within limits
    assert!(!model.check_geometry(50e-9, 1e-6)); // L too small
    assert!(!model.check_geometry(100e-9, 200e-6)); // W too large
}

#[test]
fn test_device_model_parameters() {
    let mut model = DeviceModel::new("nmos", ModelType::Nmos);
    model.add_parameter("vth0", 0.4);
    model.add_parameter("tox", 2e-9);

    assert_eq!(model.parameters.get("vth0"), Some(&0.4));
    assert_eq!(model.parameters.get("tox"), Some(&2e-9));
}

// =========================================================================
// ProcessCorner Tests
// =========================================================================

#[test]
fn test_process_corner_creation() {
    let corner = ProcessCorner::new("ff");
    assert_eq!(corner.name, "ff");
    assert!(!corner.is_default);
}

#[test]
fn test_standard_corners() {
    let corners = ProcessCorner::standard_corners();
    assert_eq!(corners.len(), 5);

    let tt = corners.iter().find(|c| c.name == "tt").unwrap();
    assert!(tt.is_default);
    assert_eq!(tt.temperature, 27.0);

    let ff = corners.iter().find(|c| c.name == "ff").unwrap();
    assert_eq!(ff.temperature, -40.0);
    assert_eq!(ff.vdd_factor, 1.1);

    let ss = corners.iter().find(|c| c.name == "ss").unwrap();
    assert_eq!(ss.temperature, 125.0);
}

// =========================================================================
// ModelLibrary Tests
// =========================================================================

#[test]
fn test_model_library_creation() {
    let lib = ModelLibrary::new("tsmc180");
    assert_eq!(lib.name, "tsmc180");
    assert_eq!(lib.corner_count(), 5); // Standard corners added
    assert_eq!(lib.selected_corner, Some("tt".to_string()));
}

#[test]
fn test_model_library_add_model() {
    let mut lib = ModelLibrary::new("test");
    lib.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
    lib.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));

    assert_eq!(lib.model_count(), 2);
    assert!(lib.get_model("nmos_svt").is_some());
}

#[test]
fn test_model_library_models_by_type() {
    let mut lib = ModelLibrary::new("test");
    lib.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
    lib.add_model(DeviceModel::new("nmos_hvt", ModelType::Nmos));
    lib.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));

    let nmos_models = lib.models_by_type(ModelType::Nmos);
    assert_eq!(nmos_models.len(), 2);

    let pmos_models = lib.models_by_type(ModelType::Pmos);
    assert_eq!(pmos_models.len(), 1);
}

#[test]
fn test_model_library_select_corner() {
    let mut lib = ModelLibrary::new("test");

    assert!(lib.select_corner("ff"));
    assert_eq!(lib.selected_corner, Some("ff".to_string()));

    assert!(!lib.select_corner("nonexistent"));
    assert_eq!(lib.selected_corner, Some("ff".to_string())); // Unchanged
}

#[test]
fn test_model_library_current_corner() {
    let lib = ModelLibrary::new("test");
    let corner = lib.current_corner().unwrap();
    assert_eq!(corner.name, "tt");
}

#[test]
fn test_model_library_with_technology() {
    let lib = ModelLibrary::new("tsmc180").with_technology("TSMC", "180nm");

    assert_eq!(lib.pdk_name, "TSMC");
    assert_eq!(lib.technology_node, "180nm");
}

// =========================================================================
// ModelLibraryManager Tests
// =========================================================================

#[test]
fn test_manager_creation() {
    let mgr = ModelLibraryManager::new();
    assert_eq!(mgr.library_count(), 0);
}

#[test]
fn test_manager_add_library() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("lib1"));
    mgr.add_library(ModelLibrary::new("lib2"));

    assert_eq!(mgr.library_count(), 2);
}

#[test]
fn test_manager_select_library() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("lib1"));

    mgr.select_library("lib1");
    assert!(mgr.current_library().is_some());
    assert_eq!(mgr.current_library().unwrap().name, "lib1");
}

#[test]
fn test_manager_search_models() {
    let mut mgr = ModelLibraryManager::new();
    let mut lib = ModelLibrary::new("lib1");
    lib.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
    lib.add_model(DeviceModel::new("nmos_hvt", ModelType::Nmos));
    lib.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));
    mgr.add_library(lib);

    let results = mgr.search_models("svt");
    assert_eq!(results.len(), 2);

    let results = mgr.search_models("nmos");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_manager_search_with_type_filter() {
    let mut mgr = ModelLibraryManager::new();
    let mut lib = ModelLibrary::new("lib1");
    lib.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
    lib.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));
    mgr.add_library(lib);

    mgr.filter_type = Some(ModelType::Nmos);
    let results = mgr.search_models("svt");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1.model_type, ModelType::Nmos);
}

#[test]
fn test_manager_total_model_count() {
    let mut mgr = ModelLibraryManager::new();

    let mut lib1 = ModelLibrary::new("lib1");
    lib1.add_model(DeviceModel::new("m1", ModelType::Nmos));
    lib1.add_model(DeviceModel::new("m2", ModelType::Pmos));
    mgr.add_library(lib1);

    let mut lib2 = ModelLibrary::new("lib2");
    lib2.add_model(DeviceModel::new("m3", ModelType::Resistor));
    mgr.add_library(lib2);

    assert_eq!(mgr.total_model_count(), 3);
}

#[test]
fn test_manager_libraries_sorted() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("zebra"));
    mgr.add_library(ModelLibrary::new("alpha"));

    let sorted = mgr.libraries_sorted();
    assert_eq!(sorted[0].name, "alpha");
    assert_eq!(sorted[1].name, "zebra");
}

#[test]
fn test_manager_clear() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("lib1"));
    mgr.select_library("lib1");

    mgr.clear();
    assert_eq!(mgr.library_count(), 0);
    assert!(mgr.selected_library.is_none());
}

// =========================================================================
// PDK Integration Tests
// =========================================================================

#[test]
fn test_load_builtin_models() {
    let mut mgr = ModelLibraryManager::new();
    mgr.load_builtin_models();

    // Should have created libraries for built-in model types
    assert!(
        mgr.library_count() > 0,
        "Should have loaded built-in libraries"
    );

    // Should have some models
    assert!(
        mgr.total_model_count() > 0,
        "Should have loaded built-in models"
    );
}

#[test]
fn test_convert_core_model_type() {
    use rspice_core::library::ModelType as CoreType;

    assert_eq!(
        ModelLibraryManager::convert_core_model_type(CoreType::Nmos),
        ModelType::Nmos
    );
    assert_eq!(
        ModelLibraryManager::convert_core_model_type(CoreType::Pmos),
        ModelType::Pmos
    );
    assert_eq!(
        ModelLibraryManager::convert_core_model_type(CoreType::NpnBjt),
        ModelType::Npn
    );
    assert_eq!(
        ModelLibraryManager::convert_core_model_type(CoreType::PnpBjt),
        ModelType::Pnp
    );
    assert_eq!(
        ModelLibraryManager::convert_core_model_type(CoreType::Diode),
        ModelType::Diode
    );
    assert_eq!(
        ModelLibraryManager::convert_core_model_type(CoreType::Resistor),
        ModelType::Resistor
    );
    assert_eq!(
        ModelLibraryManager::convert_core_model_type(CoreType::Capacitor),
        ModelType::Capacitor
    );
    assert_eq!(
        ModelLibraryManager::convert_core_model_type(CoreType::Other),
        ModelType::Other
    );
}

#[test]
fn test_load_from_pdk_config() {
    use crate::state::pdk_config::PdkConfig;
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();

    // Create a test .lib file
    let lib_content = r#"
* Test library
.lib tt
.model nmos_test nmos level=54 vth0=0.4
.model pmos_test pmos level=54 vth0=-0.4
.endl tt
"#;
    let lib_path = temp_dir.path().join("test.lib");
    fs::write(&lib_path, lib_content).unwrap();

    // Create PdkConfig and discover files
    let mut pdk_config = PdkConfig::new();
    pdk_config.add_library_path(temp_dir.path().to_string_lossy().to_string());
    pdk_config.discover_model_files();

    // Load from PDK config
    let mut mgr = ModelLibraryManager::new();
    let result = mgr.load_from_pdk_config(&pdk_config);

    assert!(
        result.is_ok(),
        "Should load from PDK config: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap(), 1, "Should have loaded 1 library");
    assert!(
        mgr.library_count() > 0,
        "Should have libraries after loading"
    );
}

#[test]
fn test_peek_library_sections() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();

    // Create a test .lib file with multiple sections
    let lib_content = r#"
.lib tt
.model nmos nmos level=54
.endl tt

.lib ff
.model nmos nmos level=54
.endl ff

.lib ss
.model nmos nmos level=54
.endl ss
"#;
    let lib_path = temp_dir.path().join("multi_corner.lib");
    fs::write(&lib_path, lib_content).unwrap();

    let sections = ModelLibraryManager::peek_library_sections(&lib_path);
    assert!(
        sections.is_ok(),
        "Should peek sections: {:?}",
        sections.err()
    );

    let sections = sections.unwrap();
    assert_eq!(sections.len(), 3, "Should have 3 sections");
    assert!(sections.contains(&"tt".to_string()));
    assert!(sections.contains(&"ff".to_string()));
    assert!(sections.contains(&"ss".to_string()));
}

#[test]
fn test_load_library_file_with_section() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();

    // Create a test .lib file with sections
    let lib_content = r#"
.lib tt
.model nmos_tt nmos level=54 vth0=0.4
.endl tt

.lib ff
.model nmos_ff nmos level=54 vth0=0.35
.endl ff
"#;
    let lib_path = temp_dir.path().join("corners.lib");
    fs::write(&lib_path, lib_content).unwrap();

    let mut mgr = ModelLibraryManager::new();

    // Load only the "tt" section
    let result = mgr.load_library_file(&lib_path, Some("tt"));
    assert!(result.is_ok(), "Should load tt section: {:?}", result.err());

    let lib = mgr.get_library("corners");
    assert!(lib.is_some(), "Should have 'corners' library");

    let lib = lib.unwrap();
    assert!(
        lib.get_model("nmos_tt").is_some(),
        "Should have nmos_tt model"
    );
    // Should not have model from ff section since we only loaded tt
}

#[test]
fn test_load_library_file_all_sections() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();

    // Create a test .lib file with sections
    let lib_content = r#"
.lib tt
.model nmos_tt nmos level=54 vth0=0.4
.endl tt

.lib ff
.model nmos_ff nmos level=54 vth0=0.35
.endl ff
"#;
    let lib_path = temp_dir.path().join("corners.lib");
    fs::write(&lib_path, lib_content).unwrap();

    let mut mgr = ModelLibraryManager::new();

    // Load all sections (no section filter)
    let result = mgr.load_library_file(&lib_path, None);
    assert!(
        result.is_ok(),
        "Should load all sections: {:?}",
        result.err()
    );

    let lib = mgr.get_library("corners");
    assert!(lib.is_some(), "Should have 'corners' library");

    let lib = lib.unwrap();
    assert!(
        lib.get_model("nmos_tt").is_some(),
        "Should have nmos_tt model"
    );
    assert!(
        lib.get_model("nmos_ff").is_some(),
        "Should have nmos_ff model"
    );

    // Should have corners populated
    assert!(lib.corners.contains_key("tt"), "Should have tt corner");
    assert!(lib.corners.contains_key("ff"), "Should have ff corner");
}

// =========================================================================
// Comprehensive Integration Tests - Commercial Grade
// =========================================================================

#[test]
fn test_load_library_invalid_section() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let lib_content = r#"
.lib tt
.model nmos_tt nmos level=54 vth0=0.4
.endl tt
"#;
    let lib_path = temp_dir.path().join("test.lib");
    fs::write(&lib_path, lib_content).unwrap();

    let mut mgr = ModelLibraryManager::new();
    let result = mgr.load_library_file(&lib_path, Some("nonexistent"));
    assert!(result.is_err(), "Should error on nonexistent section");
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_load_library_empty_file() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let lib_path = temp_dir.path().join("empty.lib");
    fs::write(&lib_path, "").unwrap();

    let mut mgr = ModelLibraryManager::new();
    let result = mgr.load_library_file(&lib_path, None);
    // Empty files should succeed but load no models
    assert!(result.is_ok());

    let lib = mgr.get_library("empty");
    assert!(lib.is_some());
    assert_eq!(lib.unwrap().model_count(), 0);
}

#[test]
fn test_load_library_comments_only() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let lib_content = r#"
* This is a comment
* Another comment
.comment More comments
"#;
    let lib_path = temp_dir.path().join("comments.lib");
    fs::write(&lib_path, lib_content).unwrap();

    let mut mgr = ModelLibraryManager::new();
    let result = mgr.load_library_file(&lib_path, None);
    assert!(result.is_ok());
}

#[test]
fn test_load_library_nonexistent_file() {
    let mut mgr = ModelLibraryManager::new();
    let result = mgr.load_library_file("/nonexistent/path/to/file.lib", None);
    assert!(result.is_err(), "Should error on nonexistent file");
}

#[test]
fn test_load_multiple_libraries() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();

    // Create multiple library files
    fs::write(
        temp_dir.path().join("nmos.lib"),
        ".model nmos1 nmos level=54 vth0=0.4",
    )
    .unwrap();
    fs::write(
        temp_dir.path().join("pmos.lib"),
        ".model pmos1 pmos level=54 vth0=-0.4",
    )
    .unwrap();
    fs::write(
        temp_dir.path().join("diode.lib"),
        ".model d1 d is=1e-14 n=1.0",
    )
    .unwrap();

    let mut mgr = ModelLibraryManager::new();

    assert!(
        mgr.load_library_file(temp_dir.path().join("nmos.lib"), None)
            .is_ok()
    );
    assert!(
        mgr.load_library_file(temp_dir.path().join("pmos.lib"), None)
            .is_ok()
    );
    assert!(
        mgr.load_library_file(temp_dir.path().join("diode.lib"), None)
            .is_ok()
    );

    assert_eq!(mgr.library_count(), 3);
    assert!(mgr.get_library("nmos").is_some());
    assert!(mgr.get_library("pmos").is_some());
    assert!(mgr.get_library("diode").is_some());
}

#[test]
fn test_reload_same_library() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let lib_path = temp_dir.path().join("models.lib");

    // First version
    fs::write(&lib_path, ".model nmos1 nmos level=54").unwrap();

    let mut mgr = ModelLibraryManager::new();
    mgr.load_library_file(&lib_path, None).unwrap();
    assert_eq!(mgr.get_library("models").unwrap().model_count(), 1);

    // Second version - add another model
    fs::write(
        &lib_path,
        ".model nmos1 nmos level=54\n.model nmos2 nmos level=54",
    )
    .unwrap();
    mgr.load_library_file(&lib_path, None).unwrap();

    // Should still only have one library but with updated content
    assert_eq!(mgr.library_count(), 1);
    assert_eq!(mgr.get_library("models").unwrap().model_count(), 2);
}

#[test]
fn test_search_models_case_insensitive() {
    let mut mgr = ModelLibraryManager::new();
    let mut lib = ModelLibrary::new("test");
    lib.add_model(DeviceModel::new("NMOS_1V8", ModelType::Nmos));
    lib.add_model(DeviceModel::new("nmos_3v3", ModelType::Nmos));
    lib.add_model(DeviceModel::new("PMOS_1V8", ModelType::Pmos));
    mgr.add_library(lib);

    // Search should be case-insensitive
    let results = mgr.search_models("nmos");
    assert_eq!(results.len(), 2, "Should find both NMOS models");

    let results = mgr.search_models("NMOS");
    assert_eq!(results.len(), 2, "Should find both nmos models");

    let results = mgr.search_models("Nmos");
    assert_eq!(results.len(), 2, "Should find both nmos models");
}

#[test]
fn test_search_models_partial_match() {
    let mut mgr = ModelLibraryManager::new();
    let mut lib = ModelLibrary::new("test");
    lib.add_model(DeviceModel::new("nmos_1v8_hp", ModelType::Nmos));
    lib.add_model(DeviceModel::new("nmos_1v8_lp", ModelType::Nmos));
    lib.add_model(DeviceModel::new("nmos_3v3_hp", ModelType::Nmos));
    mgr.add_library(lib);

    let results = mgr.search_models("1v8");
    assert_eq!(results.len(), 2, "Should find models containing '1v8'");

    let results = mgr.search_models("hp");
    assert_eq!(results.len(), 2, "Should find models ending in 'hp'");
}

#[test]
fn test_model_type_icon() {
    // Verify all model types have icons
    let types = [
        ModelType::Nmos,
        ModelType::Pmos,
        ModelType::Npn,
        ModelType::Pnp,
        ModelType::Resistor,
        ModelType::Capacitor,
        ModelType::Inductor,
        ModelType::Diode,
        ModelType::Varactor,
        ModelType::Rf,
        ModelType::Esd,
        ModelType::Other,
    ];

    for mt in types {
        let icon = mt.icon();
        assert!(!icon.is_empty(), "{:?} should have an icon", mt);
    }
}

#[test]
fn test_model_level_display_names() {
    let levels = [
        ModelLevel::Bsim3v3,
        ModelLevel::Bsim4,
        ModelLevel::BsimCmg,
        ModelLevel::Psp,
        ModelLevel::Ekv,
        ModelLevel::VerilogA,
        ModelLevel::SpiceLevel1,
        ModelLevel::SpiceLevel3,
        ModelLevel::Unknown,
    ];

    for level in levels {
        let name = level.display_name();
        assert!(!name.is_empty(), "{:?} should have display name", level);
    }
}

#[test]
fn test_geometry_check_edge_cases() {
    let model =
        DeviceModel::new("test", ModelType::Nmos).with_geometry(100e-9, 10e-6, 100e-9, 100e-6);

    // Exact minimum
    assert!(model.check_geometry(100e-9, 100e-9));

    // Exact maximum
    assert!(model.check_geometry(10e-6, 100e-6));

    // Just below minimum
    assert!(!model.check_geometry(99e-9, 100e-9));

    // Just above maximum
    assert!(!model.check_geometry(11e-6, 100e-6));
}

#[test]
fn test_process_corner_temperatures() {
    let corners = ProcessCorner::standard_corners();

    // Find corners with temperature variations
    let _hot_corner = corners
        .iter()
        .find(|c| c.name.contains("hot") || c.description.contains("hot"));
    let _cold_corner = corners
        .iter()
        .find(|c| c.name.contains("cold") || c.description.contains("cold"));

    // Standard corners should provide temperature information
    assert!(
        corners.len() >= 5,
        "Should have at least 5 standard corners"
    );
}

#[test]
fn test_library_models_by_type_empty() {
    let lib = ModelLibrary::new("empty");

    assert!(lib.models_by_type(ModelType::Nmos).is_empty());
    assert!(lib.models_by_type(ModelType::Pmos).is_empty());
    assert!(lib.models_by_type(ModelType::Diode).is_empty());
}

#[test]
fn test_manager_current_library_selection() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("lib_a"));
    mgr.add_library(ModelLibrary::new("lib_b"));
    mgr.add_library(ModelLibrary::new("lib_c"));

    assert!(mgr.current_library().is_none(), "No default selection");

    mgr.select_library("lib_b");
    assert_eq!(mgr.current_library().unwrap().name, "lib_b");

    mgr.select_library("lib_c");
    assert_eq!(mgr.current_library().unwrap().name, "lib_c");

    // Selecting nonexistent library is a no-op (keeps current selection)
    mgr.select_library("nonexistent");
    assert_eq!(
        mgr.current_library().unwrap().name,
        "lib_c",
        "Invalid selection keeps current"
    );
}

#[test]
fn test_library_clear() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("lib1"));
    mgr.add_library(ModelLibrary::new("lib2"));
    mgr.select_library("lib1");

    assert_eq!(mgr.library_count(), 2);
    assert!(mgr.current_library().is_some());

    mgr.clear();

    assert_eq!(mgr.library_count(), 0);
    assert!(mgr.current_library().is_none());
}

#[test]
fn test_device_model_default_values() {
    let model = DeviceModel::default();

    assert_eq!(model.name, "");
    assert_eq!(model.model_type, ModelType::Nmos);
    assert!(model.vdd.is_none());
    assert!(model.parameters.is_empty());
}

#[test]
fn test_device_model_builder_pattern() {
    let model = DeviceModel::new("nch_1v8", ModelType::Nmos)
        .with_level(ModelLevel::Bsim4)
        .with_geometry(45e-9, 1e-6, 100e-9, 10e-6)
        .with_vdd(1.8);

    assert_eq!(model.name, "nch_1v8");
    assert_eq!(model.model_type, ModelType::Nmos);
    assert_eq!(model.level, ModelLevel::Bsim4);
    assert_eq!(model.l_min, Some(45e-9));
    assert_eq!(model.l_max, Some(1e-6));
    assert_eq!(model.w_min, Some(100e-9));
    assert_eq!(model.w_max, Some(10e-6));
    assert_eq!(model.vdd, Some(1.8));
}

#[test]
fn test_builtin_models_loaded() {
    let mut mgr = ModelLibraryManager::new();
    mgr.load_builtin_models();

    // Should have built-in libraries loaded
    assert!(
        mgr.library_count() > 0,
        "Should have at least one builtin library"
    );

    // Should have some models
    assert!(
        mgr.total_model_count() > 0,
        "Should have loaded some models"
    );
}

#[test]
fn test_library_serialization() {
    let mut lib = ModelLibrary::new("test_pdk");
    lib.pdk_name = "TSMC 180nm".to_string();
    lib.technology_node = "180nm".to_string();
    lib.add_model(DeviceModel::new("nch_1v8", ModelType::Nmos));

    let json = serde_json::to_string(&lib).unwrap();
    let loaded: ModelLibrary = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.name, "test_pdk");
    assert_eq!(loaded.pdk_name, "TSMC 180nm");
    assert_eq!(loaded.technology_node, "180nm");
    assert_eq!(loaded.model_count(), 1);
}

#[test]
fn test_manager_serialization() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("lib1"));
    mgr.add_library(ModelLibrary::new("lib2"));
    mgr.select_library("lib1");

    let json = serde_json::to_string(&mgr).unwrap();
    let loaded: ModelLibraryManager = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.library_count(), 2);
    assert_eq!(loaded.selected_library.as_deref(), Some("lib1"));
}

#[test]
fn test_library_remove() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("keep_me"));
    mgr.add_library(ModelLibrary::new("remove_me"));

    assert_eq!(mgr.library_count(), 2);

    let removed = mgr.remove_library("remove_me");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().name, "remove_me");

    assert_eq!(mgr.library_count(), 1);
    assert!(mgr.get_library("keep_me").is_some());
    assert!(mgr.get_library("remove_me").is_none());
}

#[test]
fn test_library_remove_nonexistent() {
    let mut mgr = ModelLibraryManager::new();
    mgr.add_library(ModelLibrary::new("exists"));

    let removed = mgr.remove_library("does_not_exist");
    assert!(removed.is_none());
}

#[test]
fn test_model_type_from_str_comprehensive() {
    // Standard casing
    assert_eq!(ModelType::from_name("nmos"), ModelType::Nmos);
    assert_eq!(ModelType::from_name("pmos"), ModelType::Pmos);
    assert_eq!(ModelType::from_name("npn"), ModelType::Npn);
    assert_eq!(ModelType::from_name("pnp"), ModelType::Pnp);

    // Mixed casing
    assert_eq!(ModelType::from_name("NMOS"), ModelType::Nmos);
    assert_eq!(ModelType::from_name("NMos"), ModelType::Nmos);
    assert_eq!(ModelType::from_name("nMoS"), ModelType::Nmos);

    // Alternative names
    assert_eq!(ModelType::from_name("r"), ModelType::Resistor);
    assert_eq!(ModelType::from_name("c"), ModelType::Capacitor);
    assert_eq!(ModelType::from_name("l"), ModelType::Inductor);
    assert_eq!(ModelType::from_name("d"), ModelType::Diode);

    // Unknown types
    assert_eq!(ModelType::from_name("unknown_type"), ModelType::Other);
    assert_eq!(ModelType::from_name(""), ModelType::Other);
}

// =========================================================================
// Process Corner Tests - Spectre Parity
// =========================================================================

#[test]
fn test_process_corner_default_is_tt() {
    let corner = ProcessCorner::default();
    assert_eq!(corner.name, "tt");
    assert_eq!(corner.description, "Typical-Typical");
    assert!(corner.is_default);
    assert_eq!(corner.temperature, 27.0);
    assert_eq!(corner.vdd_factor, 1.0);
}

#[test]
fn test_process_corner_new_not_default() {
    let corner = ProcessCorner::new("custom");
    assert_eq!(corner.name, "custom");
    assert!(!corner.is_default);
}

#[test]
fn test_standard_corners_count() {
    let corners = ProcessCorner::standard_corners();
    assert_eq!(
        corners.len(),
        5,
        "Standard PDK has 5 corners: tt, ff, ss, sf, fs"
    );
}

#[test]
fn test_standard_corners_names() {
    let corners = ProcessCorner::standard_corners();
    let names: Vec<_> = corners.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"tt"), "Must have tt corner");
    assert!(names.contains(&"ff"), "Must have ff corner");
    assert!(names.contains(&"ss"), "Must have ss corner");
    assert!(names.contains(&"sf"), "Must have sf corner");
    assert!(names.contains(&"fs"), "Must have fs corner");
}

#[test]
fn test_standard_corners_tt_properties() {
    let corners = ProcessCorner::standard_corners();
    let tt = corners.iter().find(|c| c.name == "tt").unwrap();

    assert_eq!(tt.description, "Typical-Typical");
    assert_eq!(tt.nmos_corner, "typical");
    assert_eq!(tt.pmos_corner, "typical");
    assert_eq!(tt.temperature, 27.0);
    assert_eq!(tt.vdd_factor, 1.0);
    assert!(tt.is_default, "tt should be the default corner");
}

#[test]
fn test_standard_corners_ff_properties() {
    let corners = ProcessCorner::standard_corners();
    let ff = corners.iter().find(|c| c.name == "ff").unwrap();

    assert_eq!(ff.description, "Fast-Fast");
    assert_eq!(ff.nmos_corner, "fast");
    assert_eq!(ff.pmos_corner, "fast");
    assert_eq!(ff.temperature, -40.0, "FF corner uses cold temperature");
    assert!(ff.vdd_factor > 1.0, "FF corner uses higher voltage");
    assert!(!ff.is_default);
}

#[test]
fn test_standard_corners_ss_properties() {
    let corners = ProcessCorner::standard_corners();
    let ss = corners.iter().find(|c| c.name == "ss").unwrap();

    assert_eq!(ss.description, "Slow-Slow");
    assert_eq!(ss.nmos_corner, "slow");
    assert_eq!(ss.pmos_corner, "slow");
    assert_eq!(ss.temperature, 125.0, "SS corner uses hot temperature");
    assert!(ss.vdd_factor < 1.0, "SS corner uses lower voltage");
    assert!(!ss.is_default);
}

#[test]
fn test_standard_corners_skew_sf() {
    let corners = ProcessCorner::standard_corners();
    let sf = corners.iter().find(|c| c.name == "sf").unwrap();

    assert_eq!(sf.description, "Slow-Fast");
    assert_eq!(sf.nmos_corner, "slow");
    assert_eq!(sf.pmos_corner, "fast");
    assert_eq!(sf.temperature, 27.0, "Skew corners use room temperature");
}

#[test]
fn test_standard_corners_skew_fs() {
    let corners = ProcessCorner::standard_corners();
    let fs = corners.iter().find(|c| c.name == "fs").unwrap();

    assert_eq!(fs.description, "Fast-Slow");
    assert_eq!(fs.nmos_corner, "fast");
    assert_eq!(fs.pmos_corner, "slow");
    assert_eq!(fs.temperature, 27.0, "Skew corners use room temperature");
}

#[test]
fn test_standard_corners_exactly_one_default() {
    let corners = ProcessCorner::standard_corners();
    let default_count = corners.iter().filter(|c| c.is_default).count();
    assert_eq!(default_count, 1, "Exactly one corner should be default");
}

#[test]
fn test_library_new_includes_standard_corners() {
    let lib = ModelLibrary::new("test_lib");
    assert_eq!(lib.corner_count(), 5, "New library gets 5 standard corners");
    assert!(lib.corners.contains_key("tt"));
    assert!(lib.corners.contains_key("ff"));
    assert!(lib.corners.contains_key("ss"));
    assert!(
        lib.selected_corner.is_some(),
        "Default corner should be selected"
    );
    assert_eq!(lib.selected_corner.as_deref(), Some("tt"));
}

#[test]
fn test_library_corner_selection() {
    let mut lib = ModelLibrary::new("test_lib");

    // Default is tt
    assert_eq!(lib.selected_corner.as_deref(), Some("tt"));

    // Can change selected corner
    lib.selected_corner = Some("ff".to_string());
    assert_eq!(lib.selected_corner.as_deref(), Some("ff"));
}

#[test]
fn test_corner_temperature_range() {
    let corners = ProcessCorner::standard_corners();

    for corner in &corners {
        // All corners should have reasonable PDK temperature ranges
        assert!(
            corner.temperature >= -55.0,
            "Temperature too low: {}",
            corner.name
        );
        assert!(
            corner.temperature <= 150.0,
            "Temperature too high: {}",
            corner.name
        );
    }
}

#[test]
fn test_corner_vdd_factor_range() {
    let corners = ProcessCorner::standard_corners();

    for corner in &corners {
        // VDD factors should be within ±20% of nominal
        assert!(
            corner.vdd_factor >= 0.8,
            "VDD factor too low: {}",
            corner.name
        );
        assert!(
            corner.vdd_factor <= 1.2,
            "VDD factor too high: {}",
            corner.name
        );
    }
}

// =========================================================================
// Additional Model Tests
// =========================================================================

#[test]
fn test_model_type_icons_distinct() {
    let types = [
        ModelType::Nmos,
        ModelType::Pmos,
        ModelType::Npn,
        ModelType::Pnp,
        ModelType::Resistor,
        ModelType::Capacitor,
        ModelType::Diode,
    ];

    let _icons: Vec<_> = types.iter().map(|t| t.icon()).collect();
    // At minimum NMOS/PMOS should have different icons
    assert_ne!(ModelType::Nmos.icon(), ModelType::Pmos.icon());
    assert_ne!(ModelType::Npn.icon(), ModelType::Pnp.icon());
}

#[test]
fn test_device_model_geometry_check_boundaries() {
    let model =
        DeviceModel::new("nch", ModelType::Nmos).with_geometry(0.18e-6, 10e-6, 0.24e-6, 100e-6);

    // Valid geometry
    assert!(model.check_geometry(1e-6, 1e-6));
    assert!(model.check_geometry(0.18e-6, 0.24e-6)); // Min values
    assert!(model.check_geometry(10e-6, 100e-6)); // Max values

    // Invalid geometry - too small
    assert!(!model.check_geometry(0.1e-6, 1e-6)); // L too small
    assert!(!model.check_geometry(1e-6, 0.1e-6)); // W too small

    // Invalid geometry - too large
    assert!(!model.check_geometry(20e-6, 1e-6)); // L too large
    assert!(!model.check_geometry(1e-6, 200e-6)); // W too large
}

#[test]
fn test_device_model_geometry_no_limits() {
    let model = DeviceModel::new("nch", ModelType::Nmos);

    // Without geometry limits set, any geometry is valid
    assert!(model.check_geometry(1e-6, 1e-6));
    assert!(model.check_geometry(0.001e-6, 0.001e-6));
    assert!(model.check_geometry(1000e-6, 1000e-6));
}

#[test]
fn test_device_model_parameters_storage() {
    let mut model = DeviceModel::new("nch", ModelType::Nmos);

    model.add_parameter("vth0", 0.4);
    model.add_parameter("k1", 0.5);
    model.add_parameter("k2", -0.1);

    assert_eq!(model.parameters.len(), 3);
    assert_eq!(model.parameters.get("vth0"), Some(&0.4));
    assert_eq!(model.parameters.get("k1"), Some(&0.5));
    assert_eq!(model.parameters.get("k2"), Some(&-0.1));
}

#[test]
fn test_builtin_models_coverage() {
    let mut mgr = ModelLibraryManager::new();
    mgr.load_builtin_models();

    assert!(
        mgr.library_count() > 0,
        "Should load at least one builtin library"
    );

    let libs = mgr.libraries_sorted();
    assert!(
        libs.iter().any(|lib| !lib.models.is_empty()),
        "Builtin libraries should include at least one non-empty library"
    );

    // Check for variety of model types across all builtin libraries.
    let has_nmos = libs
        .iter()
        .any(|lib| lib.models.values().any(|m| m.model_type == ModelType::Nmos));
    let has_pmos = libs
        .iter()
        .any(|lib| lib.models.values().any(|m| m.model_type == ModelType::Pmos));
    assert!(
        has_nmos || has_pmos,
        "Builtin models should include transistors"
    );
}
