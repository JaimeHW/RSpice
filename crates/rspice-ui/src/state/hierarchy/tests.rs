use super::*;

// =========================================================================
// Library Tests
// =========================================================================

#[test]
fn test_library_new() {
    let lib = Library::new("my_project");
    assert_eq!(lib.name, "my_project");
    assert!(lib.path.is_none());
    assert_eq!(lib.cell_count(), 0);
    assert!(!lib.read_only);
    assert_eq!(lib.library_type, LibraryType::Project);
}

#[test]
fn test_library_with_path() {
    let lib = Library::new("test_lib").with_path("/home/user/libs/test_lib");
    assert_eq!(lib.path, Some("/home/user/libs/test_lib".to_string()));
}

#[test]
fn test_library_with_type() {
    let lib = Library::new("tech_lib").with_type(LibraryType::Technology);
    assert_eq!(lib.library_type, LibraryType::Technology);

    let ref_lib = Library::new("ip_lib").with_type(LibraryType::Reference);
    assert_eq!(ref_lib.library_type, LibraryType::Reference);
}

#[test]
fn test_library_create_cell() {
    let mut lib = Library::new("my_project");
    lib.create_cell("opamp");
    lib.create_cell("comparator");
    lib.create_cell("bandgap");

    assert_eq!(lib.cell_count(), 3);
    assert!(lib.has_cell("opamp"));
    assert!(lib.has_cell("comparator"));
    assert!(lib.has_cell("bandgap"));
    assert!(!lib.has_cell("nonexistent"));
}

#[test]
fn test_library_get_cell() {
    let mut lib = Library::new("test");
    lib.create_cell("cell_a");

    let cell = lib.get_cell("cell_a");
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().name, "cell_a");

    assert!(lib.get_cell("nonexistent").is_none());
}

#[test]
fn test_library_get_cell_mut() {
    let mut lib = Library::new("test");
    lib.create_cell("cell_a");

    if let Some(cell) = lib.get_cell_mut("cell_a") {
        cell.set_property("key", "value");
    }

    let cell = lib.get_cell("cell_a").unwrap();
    assert_eq!(cell.properties.get("key"), Some(&"value".to_string()));
}

#[test]
fn test_library_cell_names() {
    let mut lib = Library::new("test");
    lib.create_cell("alpha");
    lib.create_cell("beta");
    lib.create_cell("gamma");

    let names = lib.cell_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
    assert!(names.contains(&"gamma"));
}

#[test]
fn test_library_delete_cell() {
    let mut lib = Library::new("test");
    lib.create_cell("to_delete");
    lib.create_cell("to_keep");

    assert!(lib.delete_cell("to_delete"));
    assert!(!lib.has_cell("to_delete"));
    assert!(lib.has_cell("to_keep"));

    // Deleting non-existent cell returns false
    assert!(!lib.delete_cell("nonexistent"));
}

#[test]
fn test_library_metadata_default() {
    let lib = Library::new("test");
    assert!(lib.metadata.description.is_empty());
    assert!(lib.metadata.author.is_empty());
    assert!(lib.metadata.technology.is_none());
}

// =========================================================================
// Cell Tests
// =========================================================================

#[test]
fn test_cell_new() {
    let cell = Cell::new("opamp", "analog_lib");
    assert_eq!(cell.name, "opamp");
    assert_eq!(cell.library, "analog_lib");
    assert_eq!(cell.category, CellCategory::Analog);
    assert!(cell.views.is_empty());
    assert!(cell.properties.is_empty());
}

#[test]
fn test_cell_with_category() {
    let cell = Cell::new("counter", "digital").with_category(CellCategory::Digital);
    assert_eq!(cell.category, CellCategory::Digital);

    let mixed = Cell::new("adc", "mixed").with_category(CellCategory::MixedSignal);
    assert_eq!(mixed.category, CellCategory::MixedSignal);
}

#[test]
fn test_cell_full_name() {
    let cell = Cell::new("opamp", "analog_lib");
    assert_eq!(cell.full_name(), "analog_lib:opamp");
}

#[test]
fn test_cell_add_view() {
    let mut cell = Cell::new("test", "lib");
    cell.add_view(CellView::netlist(".subckt test\n.ends"));
    cell.add_view(CellView::symbol_placeholder());

    assert_eq!(cell.views.len(), 2);
    assert!(cell.get_view("netlist").is_some());
    assert!(cell.get_view("symbol").is_some());
}

#[test]
fn test_cell_with_view_builder() {
    let cell = Cell::new("test", "lib")
        .with_view(CellView::netlist(".subckt"))
        .with_category(CellCategory::Test);

    assert!(cell.get_view("netlist").is_some());
    assert_eq!(cell.category, CellCategory::Test);
}

#[test]
fn test_cell_view_names() {
    let mut cell = Cell::new("test", "lib");
    cell.add_view(CellView::netlist(""));
    cell.add_view(CellView::symbol_placeholder());

    let names = cell.view_names();
    assert!(names.contains(&"netlist"));
    assert!(names.contains(&"symbol"));
}

#[test]
fn test_cell_convenience_getters() {
    let mut cell = Cell::new("test", "lib");
    assert!(cell.schematic().is_none());
    assert!(cell.symbol().is_none());

    cell.add_view(CellView::symbol_placeholder());
    assert!(cell.symbol().is_some());
}

#[test]
fn test_cell_set_property() {
    let mut cell = Cell::new("test", "lib");
    cell.set_property("component_type", "resistor");
    cell.set_property("description", "Test resistor");

    assert_eq!(
        cell.properties.get("component_type"),
        Some(&"resistor".to_string())
    );
    assert_eq!(
        cell.properties.get("description"),
        Some(&"Test resistor".to_string())
    );
}

// =========================================================================
// InterfacePin Tests
// =========================================================================

#[test]
fn test_interface_pin_input() {
    let pin = InterfacePin::input("A");
    assert_eq!(pin.name, "A");
    assert_eq!(pin.direction, PinDirection::Input);
    assert_eq!(pin.pin_type, PinType::Signal);
    assert_eq!(pin.width, 1);
}

#[test]
fn test_interface_pin_output() {
    let pin = InterfacePin::output("Y");
    assert_eq!(pin.direction, PinDirection::Output);
}

#[test]
fn test_interface_pin_inout() {
    let pin = InterfacePin::inout("IO");
    assert_eq!(pin.direction, PinDirection::InOut);
}

#[test]
fn test_interface_pin_power() {
    let pin = InterfacePin::input("VDD").power();
    assert_eq!(pin.pin_type, PinType::Power);
}

#[test]
fn test_interface_pin_ground() {
    let pin = InterfacePin::input("VSS").ground();
    assert_eq!(pin.pin_type, PinType::Ground);
}

#[test]
fn test_interface_pin_bus() {
    let pin = InterfacePin::input("DATA").bus(8);
    assert_eq!(pin.width, 8);
}

#[test]
fn test_cell_add_pins() {
    let mut cell = Cell::new("buffer", "std_lib");
    cell.add_pin(InterfacePin::input("A"));
    cell.add_pin(InterfacePin::output("Y"));
    cell.add_pin(InterfacePin::input("VDD").power());
    cell.add_pin(InterfacePin::input("VSS").ground());

    assert_eq!(cell.interface.pins.len(), 4);
    assert_eq!(cell.interface.pins[0].name, "A");
    assert_eq!(cell.interface.pins[1].name, "Y");
}

// =========================================================================
// CellView Tests
// =========================================================================

#[test]
fn test_cell_view_netlist() {
    let view = CellView::netlist(".subckt opamp in+ in- out\n.ends opamp");
    assert_eq!(view.name, "netlist");
    assert_eq!(view.view_type, ViewType::Netlist);
    match &view.content {
        ViewContent::Netlist(s) => assert!(s.contains("opamp")),
        _ => panic!("Expected Netlist content"),
    }
}

#[test]
fn test_cell_view_symbol_placeholder() {
    let view = CellView::symbol_placeholder();
    assert_eq!(view.name, "symbol");
    assert_eq!(view.view_type, ViewType::Symbol);
}

#[test]
fn test_cell_view_named() {
    let view = CellView::named("custom_view", ViewType::Documentation);
    assert_eq!(view.name, "custom_view");
    assert_eq!(view.view_type, ViewType::Documentation);
    matches!(view.content, ViewContent::Placeholder);
}

#[test]
fn test_cell_view_schematic() {
    let schematic = SchematicState::default();
    let view = CellView::schematic(schematic);
    assert_eq!(view.name, "schematic");
    assert_eq!(view.view_type, ViewType::Schematic);
}

// =========================================================================
// CellReference Tests
// =========================================================================

#[test]
fn test_cell_reference_schematic() {
    let cell_ref = CellReference::schematic("my_lib", "opamp");
    assert_eq!(cell_ref.library, "my_lib");
    assert_eq!(cell_ref.cell, "opamp");
    assert_eq!(cell_ref.view, "schematic");
}

#[test]
fn test_cell_reference_symbol() {
    let cell_ref = CellReference::symbol("my_lib", "opamp");
    assert_eq!(cell_ref.view, "symbol");
}

#[test]
fn test_cell_reference_full_path() {
    let cell_ref = CellReference::schematic("lib", "cell");
    assert_eq!(cell_ref.full_path(), "lib:cell:schematic");
}

// =========================================================================
// HierarchyInstance Tests
// =========================================================================

#[test]
fn test_hierarchy_instance_new() {
    let cell_ref = CellReference::symbol("analog", "opamp");
    let inst = HierarchyInstance::new("U1", cell_ref, 100, 200);

    assert_eq!(inst.instance_name, "U1");
    assert_eq!(inst.position, Point::new(100, 200));
    assert!(inst.connections.is_empty());
    assert!(inst.properties.is_empty());
}

#[test]
fn test_hierarchy_instance_connections() {
    let cell_ref = CellReference::symbol("analog", "opamp");
    let mut inst = HierarchyInstance::new("U1", cell_ref, 0, 0);

    inst.connect("INP", "net1");
    inst.connect("INN", "net2");
    inst.connect("OUT", "vout");

    assert_eq!(inst.get_connection("INP"), Some("net1"));
    assert_eq!(inst.get_connection("INN"), Some("net2"));
    assert_eq!(inst.get_connection("OUT"), Some("vout"));
    assert_eq!(inst.get_connection("NONEXISTENT"), None);
}

#[test]
fn test_hierarchy_instance_parameters() {
    let cell_ref = CellReference::symbol("analog", "opamp");
    let mut inst = HierarchyInstance::new("U1", cell_ref, 0, 0);

    inst.set_property("gain", "100");
    inst.set_property("bandwidth", "1e6");

    assert_eq!(inst.get_param("gain"), Some("100"));
    assert_eq!(inst.get_param("bandwidth"), Some("1e6"));
    assert_eq!(inst.get_param("nonexistent"), None);
}

#[test]
fn test_hierarchy_instance_param_pairs() {
    let cell_ref = CellReference::symbol("analog", "opamp");
    let mut inst = HierarchyInstance::new("U1", cell_ref, 0, 0);

    inst.set_property("a", "1.0");
    inst.set_property("b", "2.0");

    let pairs = inst.param_pairs();
    assert_eq!(pairs.len(), 2);
}

#[test]
fn test_hierarchy_instance_full_hierarchy_path() {
    let cell_ref = CellReference::symbol("analog", "opamp");
    let inst = HierarchyInstance::new("U1", cell_ref, 0, 0);

    let path = inst.full_hierarchy_path("TOP.BLOCK_A");
    assert_eq!(path, "TOP.BLOCK_A.U1");

    let path_empty = inst.full_hierarchy_path("");
    assert_eq!(path_empty, "U1");
}

// =========================================================================
// HierarchyManager Tests
// =========================================================================

#[test]
fn test_hierarchy_manager_new() {
    let mgr = HierarchyManager::new();
    assert!(mgr.libraries.is_empty());
    assert!(mgr.navigation_stack.is_empty());
}

#[test]
fn test_hierarchy_manager_add_library() {
    let mut mgr = HierarchyManager::new();
    mgr.add_library(Library::new("project"));
    mgr.add_library(Library::new("ip_lib"));

    assert!(mgr.get_library("project").is_some());
    assert!(mgr.get_library("ip_lib").is_some());
    assert!(mgr.get_library("nonexistent").is_none());
}

#[test]
fn test_hierarchy_manager_library_names() {
    let mut mgr = HierarchyManager::new();
    mgr.add_library(Library::new("alpha"));
    mgr.add_library(Library::new("beta"));

    let names = mgr.library_names();
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
}

#[test]
fn test_hierarchy_manager_navigation() {
    let mut mgr = HierarchyManager::new();

    let mut lib = Library::new("project");
    lib.create_cell("top");
    lib.create_cell("block_a");
    lib.create_cell("block_b");
    mgr.add_library(lib);

    // Start at root
    assert!(mgr.depth() == 0);
    assert_eq!(mgr.depth(), 0);

    // Navigate into top
    mgr.push_into(CellReference::schematic("project", "top"));
    assert!(mgr.depth() != 0);
    assert_eq!(mgr.depth(), 1);

    // Navigate deeper
    mgr.push_into(CellReference::schematic("project", "block_a"));
    assert_eq!(mgr.depth(), 2);

    // Navigate back
    mgr.pop_out();
    assert_eq!(mgr.depth(), 1);

    // Navigate to top
    while mgr.depth() > 0 {
        mgr.pop_out();
    }
    assert!(mgr.depth() == 0);
    assert_eq!(mgr.depth(), 0);
}

#[test]
fn test_hierarchy_manager_breadcrumb() {
    let mut mgr = HierarchyManager::new();

    let mut lib = Library::new("project");
    lib.create_cell("top");
    lib.create_cell("middle");
    lib.create_cell("leaf");
    mgr.add_library(lib);

    mgr.push_into(CellReference::schematic("project", "top"));
    mgr.push_into(CellReference::schematic("project", "middle"));
    mgr.push_into(CellReference::schematic("project", "leaf"));

    let breadcrumb = mgr.breadcrumb();
    assert_eq!(breadcrumb.len(), 3);
    assert_eq!(breadcrumb[0], "project:top");
    assert_eq!(breadcrumb[1], "project:middle");
    assert_eq!(breadcrumb[2], "project:leaf");
}

#[test]
fn test_hierarchy_manager_current_hierarchy_path() {
    let mut mgr = HierarchyManager::new();

    let mut lib = Library::new("project");
    lib.create_cell("top");
    lib.create_cell("sub");
    mgr.add_library(lib);

    assert_eq!(mgr.current_hierarchy_path(), "");

    mgr.push_into(CellReference::schematic("project", "top"));
    assert_eq!(mgr.current_hierarchy_path(), "top");

    mgr.push_into(CellReference::schematic("project", "sub"));
    assert_eq!(mgr.current_hierarchy_path(), "top.sub");
}

#[test]
fn test_hierarchy_manager_path_segments() {
    let mut mgr = HierarchyManager::new();

    let mut lib = Library::new("project");
    lib.create_cell("a");
    lib.create_cell("b");
    lib.create_cell("c");
    mgr.add_library(lib);

    mgr.push_into(CellReference::schematic("project", "a"));
    mgr.push_into(CellReference::schematic("project", "b"));
    mgr.push_into(CellReference::schematic("project", "c"));

    let segments = mgr.path_segments();
    assert_eq!(segments, vec!["a", "b", "c"]);
}

#[test]
fn test_hierarchy_manager_resolve_cell() {
    let mut mgr = HierarchyManager::new();

    let mut lib = Library::new("project");
    lib.create_cell("opamp");
    mgr.add_library(lib);

    let cell_ref = CellReference::schematic("project", "opamp");
    let cell = mgr.resolve_cell(&cell_ref);
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().name, "opamp");

    let bad_ref = CellReference::schematic("project", "nonexistent");
    assert!(mgr.resolve_cell(&bad_ref).is_none());
}

#[test]
fn test_hierarchy_manager_truncate_navigation() {
    let mut mgr = HierarchyManager::new();

    let mut lib = Library::new("project");
    lib.create_cell("a");
    lib.create_cell("b");
    lib.create_cell("c");
    mgr.add_library(lib);

    mgr.push_into(CellReference::schematic("project", "a"));
    mgr.push_into(CellReference::schematic("project", "b"));
    mgr.push_into(CellReference::schematic("project", "c"));

    // Pop back twice
    mgr.pop_out();
    mgr.pop_out();
    assert_eq!(mgr.depth(), 1);

    // Go all the way to top
    while mgr.depth() > 0 {
        mgr.pop_out();
    }
    assert_eq!(mgr.depth(), 0);
}

// =========================================================================
// Primitives Library Tests
// =========================================================================

#[test]
fn test_hierarchy_manager_ensure_primitives() {
    let mut mgr = HierarchyManager::new();
    mgr.create_primitives_library();

    let primitives = mgr.get_library(HierarchyManager::PRIMITIVES_LIBRARY);
    assert!(primitives.is_some());

    let lib = primitives.unwrap();
    assert!(lib.read_only);
    assert_eq!(lib.library_type, LibraryType::Technology);
}

#[test]
fn test_hierarchy_manager_primitives_contains_components() {
    let mut mgr = HierarchyManager::new();
    mgr.create_primitives_library();

    let lib = mgr
        .get_library(HierarchyManager::PRIMITIVES_LIBRARY)
        .unwrap();

    // Check for basic passive components
    assert!(lib.has_cell("Resistor"));
    assert!(lib.has_cell("Capacitor"));
    assert!(lib.has_cell("Inductor"));

    // Check for sources
    assert!(lib.has_cell("Voltage Source (DC)"));
    assert!(lib.has_cell("Current Source (DC)"));

    // Check for semiconductors
    assert!(lib.has_cell("NMOS Transistor"));
    assert!(lib.has_cell("PMOS Transistor"));
    assert!(lib.has_cell("NPN BJT"));
    assert!(lib.has_cell("PNP BJT"));
    assert!(lib.has_cell("Diode"));
}

#[test]
fn test_hierarchy_manager_is_primitive() {
    let mut mgr = HierarchyManager::new();
    mgr.create_primitives_library();

    assert!(mgr.is_primitive(HierarchyManager::PRIMITIVES_LIBRARY, "Resistor"));
    assert!(mgr.is_primitive(HierarchyManager::PRIMITIVES_LIBRARY, "Capacitor"));
    assert!(mgr.is_primitive(HierarchyManager::PRIMITIVES_LIBRARY, "NMOS Transistor"));
    assert!(!mgr.is_primitive("user_lib", "my_custom_cell"));
}

#[test]
fn test_hierarchy_manager_get_component_type() {
    let mut mgr = HierarchyManager::new();
    mgr.create_primitives_library();

    assert_eq!(
        mgr.get_component_type(HierarchyManager::PRIMITIVES_LIBRARY, "Resistor"),
        Some("Resistor".to_string())
    );
    assert_eq!(
        mgr.get_component_type(HierarchyManager::PRIMITIVES_LIBRARY, "Capacitor"),
        Some("Capacitor".to_string())
    );
    assert_eq!(
        mgr.get_component_type(HierarchyManager::PRIMITIVES_LIBRARY, "NMOS Transistor"),
        Some("Nmos".to_string())
    );
    assert_eq!(
        mgr.get_component_type(HierarchyManager::PRIMITIVES_LIBRARY, "nonexistent"),
        None
    );
}

#[test]
fn test_hierarchy_manager_primitive_cell_properties() {
    let mut mgr = HierarchyManager::new();
    mgr.create_primitives_library();

    let lib = mgr
        .get_library(HierarchyManager::PRIMITIVES_LIBRARY)
        .unwrap();
    let resistor = lib.get_cell("Resistor").unwrap();

    // Check that properties are set
    assert!(resistor.properties.contains_key("component_type"));
    assert!(resistor.properties.contains_key("category"));
}

#[test]
fn test_hierarchy_manager_primitives_generate_symbol_views() {
    let mut mgr = HierarchyManager::new();
    mgr.create_primitives_library();

    let lib = mgr
        .get_library(HierarchyManager::PRIMITIVES_LIBRARY)
        .expect("primitives library should exist");
    let resistor = lib.get_cell("Resistor").expect("resistor should exist");
    let symbol_view = resistor
        .symbol()
        .expect("primitive should have a symbol view");

    match &symbol_view.content {
        ViewContent::Symbol(symbol) => {
            assert!(!symbol.graphics.primitives.is_empty());
            assert!(!symbol.pins.is_empty());
        }
        other => panic!("expected symbol content, got {:?}", other),
    }
}

// =========================================================================
// Verilog-A Library Tests
// =========================================================================

#[test]
fn test_hierarchy_manager_ensure_veriloga_library() {
    let mut mgr = HierarchyManager::new();
    mgr.ensure_veriloga_library();

    let veriloga = mgr.get_library(HierarchyManager::VERILOGA_LIBRARY);
    assert!(veriloga.is_some());

    let lib = veriloga.unwrap();
    assert_eq!(lib.library_type, LibraryType::Reference);
}

#[test]
fn test_hierarchy_manager_add_veriloga_model() {
    let mut mgr = HierarchyManager::new();

    let terminals = vec!["in".to_string(), "out".to_string()];
    let parameters = vec![
        ("gain".to_string(), "1.0".to_string()),
        ("offset".to_string(), "0.0".to_string()),
    ];

    mgr.add_veriloga_model(
        "my_model",
        &terminals,
        &parameters,
        Some("/path/to/model.va"),
    );

    let lib = mgr.get_library(HierarchyManager::VERILOGA_LIBRARY).unwrap();
    let cell = lib.get_cell("my_model");
    assert!(cell.is_some());

    let model = cell.unwrap();
    assert_eq!(model.interface.pins.len(), 2);
    assert!(model.properties.contains_key("source_path"));
    assert!(model.properties.contains_key("parameters"));
    let symbol_view = model
        .symbol()
        .expect("veriloga import should generate a symbol view");
    match &symbol_view.content {
        ViewContent::Symbol(symbol) => assert_eq!(symbol.pins.len(), 2),
        _ => panic!("expected symbol content"),
    }
}

#[test]
fn test_hierarchy_manager_is_veriloga_model() {
    let mgr = HierarchyManager::new();

    assert!(mgr.is_veriloga_model(HierarchyManager::VERILOGA_LIBRARY, "any_cell"));
    assert!(!mgr.is_veriloga_model("some_other_lib", "cell"));
}

#[test]
fn test_hierarchy_manager_veriloga_model_names() {
    let mut mgr = HierarchyManager::new();

    let names_empty = mgr.veriloga_model_names();
    assert!(names_empty.is_empty());

    mgr.add_veriloga_model("model_a", &["a".to_string()], &[], None);
    mgr.add_veriloga_model("model_b", &["b".to_string()], &[], None);

    let names = mgr.veriloga_model_names();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"model_a".to_string()));
    assert!(names.contains(&"model_b".to_string()));
}

// =========================================================================
// SymbolContent Tests
// =========================================================================

#[test]
fn test_symbol_content_rectangle() {
    let symbol = SymbolContent::rectangle(40, 60, vec![]);
    assert!(symbol.bounds.0 > 0); // Width > 0
    assert!(symbol.bounds.1 > 0); // Height > 0
    assert!(symbol.pins.is_empty());
}

#[test]
fn test_symbol_pin_left() {
    let pin = SymbolPin::left("input", 0);
    assert_eq!(pin.name, "input");
    assert_eq!(pin.position.x, 0);
    assert_eq!(pin.position.y, 0);
}

#[test]
fn test_symbol_pin_right() {
    let pin = SymbolPin::right("output", 20, 0);
    assert_eq!(pin.name, "output");
    assert_eq!(pin.position.x, 20);
}

#[test]
fn test_symbol_pin_creation() {
    let pin = SymbolPin::new("vdd", 10, -20, PinOrientation::Top);
    assert_eq!(pin.name, "vdd");
    assert_eq!(pin.position.x, 10);
    assert_eq!(pin.position.y, -20);
}

#[test]
fn test_symbol_content_generated_pin_placement() {
    let pins = vec![
        InterfacePin::input("IN"),
        InterfacePin::output("OUT"),
        InterfacePin::input("VDD").power(),
        InterfacePin::input("VSS").ground(),
    ];
    let symbol = SymbolContent::generated("amp", &pins);

    assert_eq!(symbol.pins.len(), 4);
    assert!(
        symbol
            .pins
            .iter()
            .any(|pin| pin.orientation == PinOrientation::Left)
    );
    assert!(
        symbol
            .pins
            .iter()
            .any(|pin| pin.orientation == PinOrientation::Right)
    );
    assert!(
        symbol
            .pins
            .iter()
            .any(|pin| pin.orientation == PinOrientation::Top)
    );
    assert!(
        symbol
            .pins
            .iter()
            .any(|pin| pin.orientation == PinOrientation::Bottom)
    );
}

// =========================================================================
// Edge Case Tests
// =========================================================================

#[test]
fn test_pop_out_at_root_is_safe() {
    let mut mgr = HierarchyManager::new();
    mgr.pop_out(); // Should not panic
    assert!(mgr.depth() == 0);
}

#[test]
fn test_navigate_to_top_from_deep() {
    let mut mgr = HierarchyManager::new();

    let mut lib = Library::new("project");
    lib.create_cell("a");
    lib.create_cell("b");
    mgr.add_library(lib);

    mgr.push_into(CellReference::schematic("project", "a"));
    mgr.push_into(CellReference::schematic("project", "b"));
    assert_eq!(mgr.depth(), 2);

    while mgr.depth() > 0 {
        mgr.pop_out();
    }
    assert!(mgr.depth() == 0);
}

#[test]
fn test_library_dual_creation() {
    let mut mgr = HierarchyManager::new();
    mgr.create_primitives_library();
    mgr.create_primitives_library(); // Second call should be idempotent

    assert_eq!(mgr.libraries.len(), 1);
}

#[test]
fn test_veriloga_library_dual_creation() {
    let mut mgr = HierarchyManager::new();
    mgr.ensure_veriloga_library();
    mgr.ensure_veriloga_library(); // Should be idempotent

    // Count veriloga libraries (should be exactly 1)
    let count = mgr
        .libraries
        .values()
        .filter(|l| l.name == HierarchyManager::VERILOGA_LIBRARY)
        .count();
    assert_eq!(count, 1);
}

#[test]
fn test_complex_navigation_scenario() {
    let mut mgr = HierarchyManager::new();

    // Build a realistic hierarchy
    let mut lib = Library::new("chip");
    lib.create_cell("top");
    lib.create_cell("analog_core");
    lib.create_cell("opamp");
    lib.create_cell("bias");
    lib.create_cell("resistor_array");
    mgr.add_library(lib);

    // Navigate down the hierarchy
    mgr.push_into(CellReference::schematic("chip", "top"));
    mgr.push_into(CellReference::schematic("chip", "analog_core"));
    mgr.push_into(CellReference::schematic("chip", "opamp"));
    mgr.push_into(CellReference::schematic("chip", "bias"));

    assert_eq!(mgr.depth(), 4);
    assert_eq!(mgr.current_hierarchy_path(), "top.analog_core.opamp.bias");

    // Go back to analog_core level (depth 3)
    mgr.pop_out(); // Remove "bias"
    assert_eq!(mgr.depth(), 3);
    assert_eq!(mgr.current_hierarchy_path(), "top.analog_core.opamp");

    // Navigate to completely different path
    while mgr.depth() > 0 {
        mgr.pop_out();
    }
    mgr.push_into(CellReference::schematic("chip", "top"));
    mgr.push_into(CellReference::schematic("chip", "resistor_array"));

    assert_eq!(mgr.current_hierarchy_path(), "top.resistor_array");
}
