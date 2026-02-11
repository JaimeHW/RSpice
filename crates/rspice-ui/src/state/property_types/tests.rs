use super::*;

// =========================================================================
// PropertyValue Tests
// =========================================================================

#[test]
fn test_property_value_number() {
    let v = PropertyValue::number(1000.0);
    assert_eq!(v.as_number(), Some(1000.0));
    assert_eq!(v.display_string(), "1k");
}

#[test]
fn test_property_value_number_with_unit() {
    let v = PropertyValue::number_with_unit(1000.0, "Ω");
    assert_eq!(v.display_string(), "1kΩ");
}

#[test]
fn test_property_value_string() {
    let v = PropertyValue::string("R1");
    assert_eq!(v.as_string(), Some("R1"));
    assert_eq!(v.display_string(), "R1");
}

#[test]
fn test_property_value_expression() {
    let v = PropertyValue::expression("2*vdd");
    assert!(v.is_expression());
    assert_eq!(v.display_string(), "{2*vdd}");
}

#[test]
fn test_property_value_enum() {
    let v = PropertyValue::enumeration("nmos", vec!["nmos".to_string(), "pmos".to_string()]);
    assert_eq!(v.display_string(), "nmos");
}

#[test]
fn test_property_value_boolean() {
    let v = PropertyValue::boolean(true);
    assert_eq!(v.display_string(), "yes");
    let v = PropertyValue::boolean(false);
    assert_eq!(v.display_string(), "no");
}

// =========================================================================
// PropertyDefinition Tests
// =========================================================================

#[test]
fn test_property_definition_builder() {
    let def = PropertyDefinition::new("resistance")
        .with_display_name("Resistance")
        .with_description("The resistance value")
        .with_type(PropertyType::Number)
        .with_unit("Ω")
        .with_range(0.0, 1e12)
        .with_category("Electrical")
        .required();

    assert_eq!(def.name, "resistance");
    assert_eq!(def.display_name, "Resistance");
    assert_eq!(def.unit, Some("Ω".to_string()));
    assert_eq!(def.min_value, Some(0.0));
    assert_eq!(def.max_value, Some(1e12));
    assert!(def.required);
}

#[test]
fn test_property_definition_validate_required() {
    let def = PropertyDefinition::new("name")
        .with_type(PropertyType::String)
        .required();

    // Empty string should fail
    let result = def.validate(&PropertyValue::string(""));
    assert!(result.is_err());

    // Non-empty should pass
    let result = def.validate(&PropertyValue::string("R1"));
    assert!(result.is_ok());
}

#[test]
fn test_property_definition_validate_range() {
    let def = PropertyDefinition::new("value")
        .with_type(PropertyType::Number)
        .with_range(0.0, 1000.0);

    // In range
    assert!(def.validate(&PropertyValue::number(500.0)).is_ok());

    // Below range
    assert!(def.validate(&PropertyValue::number(-1.0)).is_err());

    // Above range
    assert!(def.validate(&PropertyValue::number(1001.0)).is_err());
}

#[test]
fn test_property_definition_validate_enum() {
    let def = PropertyDefinition::new("type").with_type(PropertyType::Enum);

    // Valid selection
    let valid = PropertyValue::enumeration("nmos", vec!["nmos".to_string(), "pmos".to_string()]);
    assert!(def.validate(&valid).is_ok());

    // Invalid selection
    let invalid =
        PropertyValue::enumeration("invalid", vec!["nmos".to_string(), "pmos".to_string()]);
    assert!(def.validate(&invalid).is_err());
}

// =========================================================================
// PropertySheet Tests
// =========================================================================

#[test]
fn test_property_sheet_add_and_get() {
    let mut sheet = PropertySheet::new();
    sheet.add(PropertyDefinition::new("r").with_display_name("Resistance"));
    sheet.add(PropertyDefinition::new("tc1").with_display_name("Temp Coeff 1"));

    assert_eq!(sheet.len(), 2);
    assert!(sheet.get("r").is_some());
    assert!(sheet.get("tc1").is_some());
    assert!(sheet.get("nonexistent").is_none());
}

#[test]
fn test_property_sheet_order() {
    let mut sheet = PropertySheet::new();
    sheet.add(PropertyDefinition::new("c").with_order(2));
    sheet.add(PropertyDefinition::new("a").with_order(0));
    sheet.add(PropertyDefinition::new("b").with_order(1));

    let names: Vec<_> = sheet.iter().map(|d| d.name.as_str()).collect();
    assert_eq!(names, vec!["a", "b", "c"]);
}

#[test]
fn test_property_sheet_by_category() {
    let mut sheet = PropertySheet::new();
    sheet.add(PropertyDefinition::new("name").with_category("Instance"));
    sheet.add(PropertyDefinition::new("r").with_category("Electrical"));
    sheet.add(PropertyDefinition::new("tc1").with_category("Temperature"));
    sheet.add(PropertyDefinition::new("tc2").with_category("Temperature"));

    let by_cat = sheet.by_category();
    assert_eq!(by_cat.get("Instance").map(|v| v.len()), Some(1));
    assert_eq!(by_cat.get("Electrical").map(|v| v.len()), Some(1));
    assert_eq!(by_cat.get("Temperature").map(|v| v.len()), Some(2));
}

// =========================================================================
// PropertyRegistry Tests
// =========================================================================

#[test]
fn test_registry_has_all_common_types() {
    let registry = PropertyRegistry::new();

    // Passives
    assert!(registry.get(ComponentType::Resistor).is_some());
    assert!(registry.get(ComponentType::Capacitor).is_some());
    assert!(registry.get(ComponentType::Inductor).is_some());

    // Sources
    assert!(registry.get(ComponentType::VoltageSource).is_some());
    assert!(registry.get(ComponentType::CurrentSource).is_some());

    // Semiconductors
    assert!(registry.get(ComponentType::Diode).is_some());
    assert!(registry.get(ComponentType::Nmos).is_some());
    assert!(registry.get(ComponentType::Pmos).is_some());
    assert!(registry.get(ComponentType::NpnBjt).is_some());
    assert!(registry.get(ComponentType::PnpBjt).is_some());

    // Controlled sources
    assert!(registry.get(ComponentType::Vcvs).is_some());
    assert!(registry.get(ComponentType::Vccs).is_some());
    assert!(registry.get(ComponentType::Ccvs).is_some());
    assert!(registry.get(ComponentType::Cccs).is_some());
}

#[test]
fn test_registry_resistor_properties() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Resistor).unwrap();

    assert!(sheet.get("name").is_some());
    assert!(sheet.get("r").is_some());
    assert!(sheet.get("tc1").is_some());
    assert!(sheet.get("tc2").is_some());

    let r_def = sheet.get("r").unwrap();
    assert!(r_def.required);
    assert_eq!(r_def.unit, Some("Ω".to_string()));
}

#[test]
fn test_registry_mosfet_properties() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Nmos).unwrap();

    assert!(sheet.get("name").is_some());
    assert!(sheet.get("model").is_some());
    assert!(sheet.get("w").is_some());
    assert!(sheet.get("l").is_some());
    assert!(sheet.get("m").is_some());
    assert!(sheet.get("nf").is_some());

    let w_def = sheet.get("w").unwrap();
    assert!(w_def.required);
    assert_eq!(w_def.prop_type, PropertyType::Expression);
}

// =========================================================================
// Passive Component Commercial-Grade Parameter Tests
// =========================================================================

#[test]
fn test_registry_resistor_commercial_parameters() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Resistor).unwrap();

    // Basic parameters
    assert!(sheet.get("name").is_some());
    assert!(sheet.get("r").is_some());

    // Multiplier and scale
    assert!(sheet.get("m").is_some()); // multiplier
    assert!(sheet.get("scale").is_some()); // scale factor

    // Temperature coefficients
    assert!(sheet.get("tc1").is_some());
    assert!(sheet.get("tc2").is_some());
    assert!(sheet.get("tce").is_some()); // exponential temp coefficient
    assert!(sheet.get("dtemp").is_some()); // temp rise

    // Noise
    assert!(sheet.get("noisy").is_some());

    // Verify resistance uses Expression type for design variables
    let r_def = sheet.get("r").unwrap();
    assert_eq!(r_def.prop_type, PropertyType::Expression);
}

#[test]
fn test_registry_capacitor_commercial_parameters() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Capacitor).unwrap();

    // Basic
    assert!(sheet.get("name").is_some());
    assert!(sheet.get("c").is_some());

    // Multiplier and scale
    assert!(sheet.get("m").is_some());
    assert!(sheet.get("scale").is_some());

    // Voltage coefficients
    assert!(sheet.get("vc1").is_some());
    assert!(sheet.get("vc2").is_some());

    // Temperature
    assert!(sheet.get("tc1").is_some());
    assert!(sheet.get("tc2").is_some());
    assert!(sheet.get("dtemp").is_some());

    // Initial conditions
    assert!(sheet.get("ic").is_some());

    // Verify capacitance uses Expression type
    let c_def = sheet.get("c").unwrap();
    assert_eq!(c_def.prop_type, PropertyType::Expression);
}

#[test]
fn test_registry_inductor_commercial_parameters() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Inductor).unwrap();

    // Basic
    assert!(sheet.get("name").is_some());
    assert!(sheet.get("l").is_some());

    // Multiplier and scale
    assert!(sheet.get("m").is_some());
    assert!(sheet.get("scale").is_some());

    // Series resistance (lossy inductor)
    assert!(sheet.get("r").is_some());

    // Temperature
    assert!(sheet.get("tc1").is_some());
    assert!(sheet.get("tc2").is_some());
    assert!(sheet.get("dtemp").is_some());

    // Initial conditions
    assert!(sheet.get("ic").is_some());

    // Mutual inductance coupling
    assert!(sheet.get("coupling_factor").is_some());
    assert!(sheet.get("coupled_to").is_some());

    // Verify inductance uses Expression type
    let l_def = sheet.get("l").unwrap();
    assert_eq!(l_def.prop_type, PropertyType::Expression);
}

// =========================================================================
// Controlled Source Commercial-Grade Parameter Tests
// =========================================================================

#[test]
fn test_registry_vcvs_commercial_parameters() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Vcvs).unwrap();

    // Basic
    assert!(sheet.get("name").is_some());
    assert!(sheet.get("gain").is_some());
    assert!(sheet.get("m").is_some()); // multiplier

    // Polynomial behavior
    assert!(sheet.get("poly").is_some());

    // AC parameters
    assert!(sheet.get("ac_gain").is_some());
    assert!(sheet.get("ac_phase").is_some());

    // Output limiting
    assert!(sheet.get("vmax").is_some());
    assert!(sheet.get("vmin").is_some());

    // Verify gain uses Expression type
    let gain_def = sheet.get("gain").unwrap();
    assert_eq!(gain_def.prop_type, PropertyType::Expression);
}

#[test]
fn test_registry_vccs_commercial_parameters() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Vccs).unwrap();

    assert!(sheet.get("name").is_some());
    assert!(sheet.get("gm").is_some());
    assert!(sheet.get("m").is_some());

    // Polynomial
    assert!(sheet.get("poly").is_some());

    // AC
    assert!(sheet.get("ac_gm").is_some());

    // Limiting
    assert!(sheet.get("imax").is_some());
    assert!(sheet.get("imin").is_some());
}

#[test]
fn test_registry_ccvs_commercial_parameters() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Ccvs).unwrap();

    assert!(sheet.get("name").is_some());
    assert!(sheet.get("rm").is_some()); // transresistance
    assert!(sheet.get("m").is_some());
    assert!(sheet.get("vref").is_some()); // sensing branch

    // Polynomial
    assert!(sheet.get("poly").is_some());

    // Limiting
    assert!(sheet.get("vmax").is_some());
    assert!(sheet.get("vmin").is_some());
}

#[test]
fn test_registry_cccs_commercial_parameters() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Cccs).unwrap();

    assert!(sheet.get("name").is_some());
    assert!(sheet.get("gain").is_some());
    assert!(sheet.get("m").is_some());
    assert!(sheet.get("vref").is_some());

    // Polynomial
    assert!(sheet.get("poly").is_some());

    // Limiting
    assert!(sheet.get("imax").is_some());
    assert!(sheet.get("imin").is_some());
}

// =========================================================================
// Engineering Formatting Tests
// =========================================================================

#[test]
fn test_format_engineering_basic() {
    assert_eq!(format_engineering(1000.0), "1k");
    assert_eq!(format_engineering(1e6), "1M");
    assert_eq!(format_engineering(1e-3), "1m");
    assert_eq!(format_engineering(1e-6), "1u");
    assert_eq!(format_engineering(1e-9), "1n");
    assert_eq!(format_engineering(1e-12), "1p");
}

#[test]
fn test_format_engineering_fractional() {
    assert_eq!(format_engineering(4700.0), "4.700k");
    assert_eq!(format_engineering(2.2e6), "2.200M");
}

#[test]
fn test_format_engineering_zero() {
    assert_eq!(format_engineering(0.0), "0");
}

// =========================================================================
// Source Registration Tests
// =========================================================================

#[test]
fn test_registry_all_source_types_registered() {
    let registry = PropertyRegistry::new();

    // DC Sources
    assert!(
        registry.get(ComponentType::VoltageSource).is_some(),
        "VoltageSource"
    );
    assert!(
        registry.get(ComponentType::CurrentSource).is_some(),
        "CurrentSource"
    );

    // AC Sources
    assert!(
        registry.get(ComponentType::VoltageSourceAc).is_some(),
        "VoltageSourceAc"
    );
    assert!(
        registry.get(ComponentType::CurrentSourceAc).is_some(),
        "CurrentSourceAc"
    );

    // Transient Voltage Sources
    assert!(
        registry.get(ComponentType::VoltageSourcePulse).is_some(),
        "VoltageSourcePulse"
    );
    assert!(
        registry.get(ComponentType::VoltageSourceSin).is_some(),
        "VoltageSourceSin"
    );
    assert!(
        registry.get(ComponentType::VoltageSourcePwl).is_some(),
        "VoltageSourcePwl"
    );
    assert!(
        registry.get(ComponentType::VoltageSourceExp).is_some(),
        "VoltageSourceExp"
    );
    assert!(
        registry.get(ComponentType::VoltageSourceSffm).is_some(),
        "VoltageSourceSffm"
    );

    // Transient Current Sources
    assert!(
        registry.get(ComponentType::CurrentSourcePulse).is_some(),
        "CurrentSourcePulse"
    );
    assert!(
        registry.get(ComponentType::CurrentSourceSin).is_some(),
        "CurrentSourceSin"
    );
    assert!(
        registry.get(ComponentType::CurrentSourcePwl).is_some(),
        "CurrentSourcePwl"
    );
    assert!(
        registry.get(ComponentType::CurrentSourceExp).is_some(),
        "CurrentSourceExp"
    );
    assert!(
        registry.get(ComponentType::CurrentSourceNoise).is_some(),
        "CurrentSourceNoise"
    );
}

#[test]
fn test_registry_vsource_pulse_properties() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSourcePulse).unwrap();

    // PULSE(V1 V2 TD TR TF PW PER)
    assert!(sheet.get("name").is_some());
    assert!(sheet.get("v1").is_some()); // Initial value
    assert!(sheet.get("v2").is_some()); // Pulsed value
    assert!(sheet.get("td").is_some()); // Delay time
    assert!(sheet.get("tr").is_some()); // Rise time
    assert!(sheet.get("tf").is_some()); // Fall time
    assert!(sheet.get("pw").is_some()); // Pulse width
    assert!(sheet.get("per").is_some()); // Period

    let v1 = sheet.get("v1").unwrap();
    assert!(v1.required);
    assert_eq!(v1.unit, Some("V".to_string()));
}

#[test]
fn test_registry_vsource_sin_properties() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSourceSin).unwrap();

    // SIN(VO VA FREQ TD THETA PHASE)
    assert!(sheet.get("vo").is_some()); // DC offset
    assert!(sheet.get("va").is_some()); // Amplitude
    assert!(sheet.get("freq").is_some()); // Frequency
    assert!(sheet.get("td").is_some()); // Delay
    assert!(sheet.get("theta").is_some()); // Damping
    assert!(sheet.get("phase").is_some()); // Phase

    let freq = sheet.get("freq").unwrap();
    assert!(freq.required);
    assert_eq!(freq.unit, Some("Hz".to_string()));
}

#[test]
fn test_registry_vsource_pwl_properties() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSourcePwl).unwrap();

    assert!(sheet.get("pwl_data").is_some());
    assert!(sheet.get("td").is_some());
    assert!(sheet.get("repeat").is_some());

    let pwl_data = sheet.get("pwl_data").unwrap();
    assert!(pwl_data.required);
    assert_eq!(pwl_data.prop_type, PropertyType::String);
}

#[test]
fn test_registry_vsource_exp_properties() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSourceExp).unwrap();

    // EXP(V1 V2 TD1 TAU1 TD2 TAU2)
    assert!(sheet.get("v1").is_some());
    assert!(sheet.get("v2").is_some());
    assert!(sheet.get("td1").is_some());
    assert!(sheet.get("tau1").is_some());
    assert!(sheet.get("td2").is_some());
    assert!(sheet.get("tau2").is_some());
}

#[test]
fn test_registry_vsource_sffm_properties() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSourceSffm).unwrap();

    // SFFM(VO VA FC MDI FS)
    assert!(sheet.get("vo").is_some()); // DC offset
    assert!(sheet.get("va").is_some()); // Amplitude
    assert!(sheet.get("fc").is_some()); // Carrier frequency
    assert!(sheet.get("mdi").is_some()); // Modulation index
    assert!(sheet.get("fs").is_some()); // Signal frequency

    let fc = sheet.get("fc").unwrap();
    assert!(fc.required);
    assert_eq!(fc.unit, Some("Hz".to_string()));
}

#[test]
fn test_registry_isource_noise_properties() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::CurrentSourceNoise).unwrap();

    assert!(sheet.get("dc").is_some());
    assert!(sheet.get("noise_type").is_some());
    assert!(sheet.get("noiseval").is_some());
    assert!(sheet.get("kf").is_some()); // Flicker coefficient
    assert!(sheet.get("af").is_some()); // Flicker exponent

    // Check noise_type is enum
    let noise_type = sheet.get("noise_type").unwrap();
    assert_eq!(noise_type.prop_type, PropertyType::Enum);
}

// =========================================================================
// Semiconductor Parameter Category Tests
// =========================================================================

#[test]
fn test_registry_diode_parameter_categories() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Diode).unwrap();

    // Instance category
    assert!(sheet.get("name").is_some());

    // Model category
    assert!(sheet.get("model").is_some());

    // Geometry category
    assert!(sheet.get("area").is_some());
    assert!(sheet.get("m").is_some()); // multiplier
    assert!(sheet.get("pj").is_some()); // perimeter

    // Initial conditions
    assert!(sheet.get("ic").is_some());
    assert!(sheet.get("off").is_some());

    // Temperature
    assert!(sheet.get("dtemp").is_some());
}

#[test]
fn test_registry_mosfet_parameter_categories() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Nmos).unwrap();

    // Geometry category
    assert!(sheet.get("w").is_some());
    assert!(sheet.get("l").is_some());
    assert!(sheet.get("nf").is_some()); // fingers
    assert!(sheet.get("m").is_some()); // multiplier

    // Parasitics
    assert!(sheet.get("ad").is_some()); // drain area
    assert!(sheet.get("as").is_some()); // source area
    assert!(sheet.get("pd").is_some()); // drain perimeter
    assert!(sheet.get("ps").is_some()); // source perimeter
    assert!(sheet.get("nrd").is_some()); // drain squares
    assert!(sheet.get("nrs").is_some()); // source squares

    // Stress effects (for advanced nodes)
    assert!(sheet.get("sa").is_some());
    assert!(sheet.get("sb").is_some());
    assert!(sheet.get("sd").is_some());

    // Initial conditions
    assert!(sheet.get("ic_vds").is_some());
    assert!(sheet.get("ic_vgs").is_some());
    assert!(sheet.get("ic_vbs").is_some());
}

#[test]
fn test_registry_bjt_parameter_categories() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::NpnBjt).unwrap();

    // Geometry
    assert!(sheet.get("area").is_some());
    assert!(sheet.get("m").is_some());
    assert!(sheet.get("areab").is_some());
    assert!(sheet.get("areac").is_some());

    // Initial conditions
    assert!(sheet.get("ic_vbe").is_some());
    assert!(sheet.get("ic_vce").is_some());

    // Operating region
    assert!(sheet.get("off").is_some());

    // Temperature
    assert!(sheet.get("dtemp").is_some());
}

#[test]
fn test_registry_jfet_parameter_categories() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::Njfet).unwrap();

    // Instance and Geometry
    assert!(sheet.get("name").is_some());
    assert!(sheet.get("area").is_some());
    assert!(sheet.get("m").is_some());

    // Initial conditions
    assert!(sheet.get("ic_vds").is_some());
    assert!(sheet.get("ic_vgs").is_some());
    assert!(sheet.get("off").is_some());

    // Temperature
    assert!(sheet.get("dtemp").is_some());
}

#[test]
fn test_source_properties_use_expression_type() {
    // Verify that value properties use Expression type for design variable support
    let registry = PropertyRegistry::new();

    // DC Source
    let vsource = registry.get(ComponentType::VoltageSource).unwrap();
    assert_eq!(
        vsource.get("dc").unwrap().prop_type,
        PropertyType::Expression
    );

    // Sin Source frequency
    let vsin = registry.get(ComponentType::VoltageSourceSin).unwrap();
    assert_eq!(
        vsin.get("freq").unwrap().prop_type,
        PropertyType::Expression
    );

    // Pulse timing
    let vpulse = registry.get(ComponentType::VoltageSourcePulse).unwrap();
    assert_eq!(
        vpulse.get("pw").unwrap().prop_type,
        PropertyType::Expression
    );
    assert_eq!(
        vpulse.get("per").unwrap().prop_type,
        PropertyType::Expression
    );
}

// =========================================================================
// Spectre-Parity Source Parameter Tests
// =========================================================================
// These tests ensure commercial-grade parity with Cadence Spectre source
// parameter sets including DC, AC, Advanced AC (XF/PAC), Parasitics, and Noise.

#[test]
fn test_spectre_parity_dc_voltage_source_categories() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSource).unwrap();

    // =====================================================================
    // Instance Category
    // =====================================================================
    let name = sheet
        .get("name")
        .expect("DC vsource must have 'name' property");
    assert!(name.required, "Instance name must be required");
    assert_eq!(name.category, "Instance");
    assert_eq!(name.prop_type, PropertyType::String);

    // =====================================================================
    // DC Category
    // =====================================================================
    let dc = sheet.get("dc").expect("DC vsource must have 'dc' property");
    assert!(dc.required, "DC voltage must be required");
    assert_eq!(dc.category, "DC");
    assert_eq!(dc.unit, Some("V".to_string()));
    assert_eq!(dc.prop_type, PropertyType::Expression);

    // =====================================================================
    // AC Category - Small-signal analysis parameters
    // =====================================================================
    let ac = sheet
        .get("ac")
        .expect("DC vsource must have 'ac' property for AC analysis");
    assert_eq!(ac.category, "AC");
    assert_eq!(ac.unit, Some("V".to_string()));

    let acphase = sheet
        .get("acphase")
        .expect("DC vsource must have 'acphase' property");
    assert_eq!(acphase.category, "AC");
    assert_eq!(acphase.unit, Some("°".to_string()));
    assert!(
        acphase.min_value.is_some() && acphase.max_value.is_some(),
        "AC phase should have range validation"
    );

    // =====================================================================
    // Advanced AC Category - XF/PAC analysis (Spectre-specific)
    // =====================================================================
    let xfmag = sheet
        .get("xfmag")
        .expect("DC vsource must have 'xfmag' for XF analysis");
    assert_eq!(xfmag.category, "Advanced AC");
    assert_eq!(
        xfmag.display_mode,
        DisplayMode::Advanced,
        "XF magnitude should be marked as advanced"
    );

    let pacmag = sheet
        .get("pacmag")
        .expect("DC vsource must have 'pacmag' for PAC analysis");
    assert_eq!(pacmag.category, "Advanced AC");
    assert_eq!(pacmag.display_mode, DisplayMode::Advanced);

    let pacdbm = sheet
        .get("pacdbm")
        .expect("DC vsource must have 'pacdbm' (dBm alternative)");
    assert_eq!(pacdbm.category, "Advanced AC");
    assert_eq!(pacdbm.unit, Some("dBm".to_string()));

    let pacphase = sheet
        .get("pacphase")
        .expect("DC vsource must have 'pacphase'");
    assert_eq!(pacphase.category, "Advanced AC");
    assert_eq!(pacphase.unit, Some("°".to_string()));

    // =====================================================================
    // Parasitics Category - Non-ideal source characteristics
    // =====================================================================
    let rs = sheet
        .get("rs")
        .expect("DC vsource must have 'rs' (series resistance)");
    assert_eq!(rs.category, "Parasitics");
    assert_eq!(rs.unit, Some("Ω".to_string()));

    let rp = sheet
        .get("rp")
        .expect("DC vsource must have 'rp' (parallel resistance)");
    assert_eq!(rp.category, "Parasitics");

    let cpar = sheet
        .get("cpar")
        .expect("DC vsource must have 'cpar' (parasitic capacitance)");
    assert_eq!(cpar.category, "Parasitics");
    assert_eq!(cpar.unit, Some("F".to_string()));

    // =====================================================================
    // Noise Category
    // =====================================================================
    let isnoisy = sheet
        .get("isnoisy")
        .expect("DC vsource must have 'isnoisy'");
    assert_eq!(isnoisy.category, "Noise");
    assert_eq!(isnoisy.prop_type, PropertyType::Boolean);
}

#[test]
fn test_spectre_parity_ac_voltage_source_defaults() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSourceAc).unwrap();

    // AC source should default to 1V AC magnitude (primary purpose)
    let ac = sheet.get("ac").unwrap();
    match &ac.default_value {
        PropertyValue::Number { value, .. } => {
            assert_eq!(*value, 1.0, "AC source should default to 1V")
        }
        other => panic!("AC magnitude should default to 1.0, got {:?}", other),
    }

    // DC offset should default to 0V
    let dc = sheet.get("dc").unwrap();
    match &dc.default_value {
        PropertyValue::Number { value, .. } => {
            assert_eq!(*value, 0.0, "DC offset should default to 0V")
        }
        other => panic!("DC offset should default to 0.0, got {:?}", other),
    }

    // AC source should have all the same categories as DC source
    assert!(
        sheet.get("xfmag").is_some(),
        "AC source must have Advanced AC params"
    );
    assert!(sheet.get("pacmag").is_some());
    assert!(
        sheet.get("rs").is_some(),
        "AC source must have Parasitics params"
    );
    assert!(
        sheet.get("isnoisy").is_some(),
        "AC source must have Noise params"
    );
}

#[test]
fn test_spectre_parity_dc_current_source_categories() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::CurrentSource).unwrap();

    // =====================================================================
    // Instance Category
    // =====================================================================
    let name = sheet.get("name").expect("DC isource must have 'name'");
    assert_eq!(name.category, "Instance");
    match &name.default_value {
        PropertyValue::String(s) => {
            assert_eq!(s, "I1", "Current source should default to I1")
        }
        other => panic!("Name should default to 'I1', got {:?}", other),
    }

    // =====================================================================
    // DC Category - Current unit (A instead of V)
    // =====================================================================
    let dc = sheet.get("dc").expect("DC isource must have 'dc'");
    assert_eq!(
        dc.unit,
        Some("A".to_string()),
        "Current source uses Amperes"
    );
    assert!(dc.required);

    // =====================================================================
    // AC Category - Current unit
    // =====================================================================
    let ac = sheet.get("ac").expect("DC isource must have 'ac'");
    assert_eq!(ac.unit, Some("A".to_string()));

    let acphase = sheet
        .get("acphase")
        .expect("DC isource must have 'acphase'");
    assert_eq!(acphase.unit, Some("°".to_string()));

    // =====================================================================
    // Advanced AC Category
    // =====================================================================
    let xfmag = sheet.get("xfmag").expect("DC isource must have 'xfmag'");
    assert_eq!(xfmag.unit, Some("A".to_string()));

    let pacmag = sheet.get("pacmag").expect("DC isource must have 'pacmag'");
    assert_eq!(pacmag.unit, Some("A".to_string()));

    // =====================================================================
    // Parasitics Category - Current source only has parallel elements
    // =====================================================================
    assert!(
        sheet.get("rp").is_some(),
        "Current source must have parallel resistance"
    );
    assert!(
        sheet.get("cpar").is_some(),
        "Current source must have parasitic capacitance"
    );
    // Note: Current sources don't have series resistance (rs) - that would change topology

    // =====================================================================
    // Noise Category
    // =====================================================================
    assert!(sheet.get("isnoisy").is_some());
}

#[test]
fn test_spectre_parity_ac_current_source_defaults() {
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::CurrentSourceAc).unwrap();

    // AC current source should default to 1A
    let ac = sheet.get("ac").unwrap();
    match &ac.default_value {
        PropertyValue::Number { value, .. } => {
            assert_eq!(*value, 1.0, "AC current source should default to 1A")
        }
        other => panic!("AC magnitude should default to 1.0, got {:?}", other),
    }

    // Should have all Spectre-parity categories
    assert!(sheet.get("xfmag").is_some());
    assert!(sheet.get("pacmag").is_some());
    assert!(sheet.get("pacdbm").is_some());
    assert!(sheet.get("pacphase").is_some());
    assert!(sheet.get("rp").is_some());
    assert!(sheet.get("cpar").is_some());
    assert!(sheet.get("isnoisy").is_some());
}

#[test]
fn test_spectre_parity_parameter_ordering() {
    // Verify ordering follows Spectre convention:
    // Instance (0-9) < DC (10-19) < AC (20-29) < Advanced AC (30-39) < Parasitics (40-49) < Noise (50+)
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSource).unwrap();

    let name_order = sheet.get("name").unwrap().display_order;
    let dc_order = sheet.get("dc").unwrap().display_order;
    let ac_order = sheet.get("ac").unwrap().display_order;
    let xfmag_order = sheet.get("xfmag").unwrap().display_order;
    let rs_order = sheet.get("rs").unwrap().display_order;
    let isnoisy_order = sheet.get("isnoisy").unwrap().display_order;

    assert!(
        name_order < dc_order,
        "Instance params should come before DC"
    );
    assert!(dc_order < ac_order, "DC params should come before AC");
    assert!(
        ac_order < xfmag_order,
        "AC params should come before Advanced AC"
    );
    assert!(
        xfmag_order < rs_order,
        "Advanced AC should come before Parasitics"
    );
    assert!(
        rs_order < isnoisy_order,
        "Parasitics should come before Noise"
    );
}

#[test]
fn test_spectre_parity_category_grouping() {
    // Verify by_category returns correct groupings for tab display
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSource).unwrap();

    let categories = sheet.by_category();

    // Should have 6 categories for DC voltage source
    assert!(categories.iter().any(|(cat, _)| cat == "Instance"));
    assert!(categories.iter().any(|(cat, _)| cat == "DC"));
    assert!(categories.iter().any(|(cat, _)| cat == "AC"));
    assert!(categories.iter().any(|(cat, _)| cat == "Advanced AC"));
    assert!(categories.iter().any(|(cat, _)| cat == "Parasitics"));
    assert!(categories.iter().any(|(cat, _)| cat == "Noise"));

    // Verify each category has expected properties
    for (cat, props) in &categories {
        match cat.as_str() {
            "Instance" => assert!(props.iter().any(|p| p.name == "name")),
            "DC" => assert!(props.iter().any(|p| p.name == "dc")),
            "AC" => {
                assert!(props.iter().any(|p| p.name == "ac"));
                assert!(props.iter().any(|p| p.name == "acphase"));
            }
            "Advanced AC" => {
                assert!(props.iter().any(|p| p.name == "xfmag"));
                assert!(props.iter().any(|p| p.name == "pacmag"));
                assert!(props.iter().any(|p| p.name == "pacdbm"));
                assert!(props.iter().any(|p| p.name == "pacphase"));
            }
            "Parasitics" => {
                assert!(props.iter().any(|p| p.name == "rs"));
                assert!(props.iter().any(|p| p.name == "rp"));
                assert!(props.iter().any(|p| p.name == "cpar"));
            }
            "Noise" => assert!(props.iter().any(|p| p.name == "isnoisy")),
            _ => {} // Other categories are OK
        }
    }
}

#[test]
fn test_spectre_parity_phase_range_validation() {
    // All phase parameters should have ±360° range
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSource).unwrap();

    let phase_params = ["acphase", "pacphase"];
    for param_name in phase_params {
        let param = sheet
            .get(param_name)
            .expect(&format!("Must have {}", param_name));
        let min = param
            .min_value
            .expect(&format!("{} must have min_value", param_name));
        let max = param
            .max_value
            .expect(&format!("{} must have max_value", param_name));
        assert_eq!(min, -360.0, "{} min should be -360°", param_name);
        assert_eq!(max, 360.0, "{} max should be 360°", param_name);
    }
}

#[test]
fn test_spectre_parity_non_negative_parasitics() {
    // Parasitic values should have non-negative ranges where appropriate
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSource).unwrap();

    // Series resistance should be >= 0
    let rs = sheet.get("rs").unwrap();
    if let Some(min) = rs.min_value {
        assert!(min >= 0.0, "Series resistance cannot be negative");
    }

    // Parasitic capacitance should be >= 0
    let cpar = sheet.get("cpar").unwrap();
    if let Some(min) = cpar.min_value {
        assert!(min >= 0.0, "Parasitic capacitance cannot be negative");
    }
}

#[test]
fn test_spectre_parity_advanced_params_marked_advanced() {
    // Advanced AC parameters should be marked as advanced (hidden by default in simple UIs)
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSource).unwrap();

    let advanced_params = ["xfmag", "pacmag", "pacdbm", "pacphase"];
    for param_name in advanced_params {
        let param = sheet
            .get(param_name)
            .expect(&format!("Must have {}", param_name));
        assert_eq!(
            param.display_mode,
            DisplayMode::Advanced,
            "{} should be marked as advanced for cleaner UI",
            param_name
        );
    }
}

#[test]
fn test_spectre_parity_property_count_per_source() {
    // Verify enhanced sources have expected number of properties
    let registry = PropertyRegistry::new();

    // DC Voltage Source: 12 properties
    // name, dc, ac, acphase, xfmag, pacmag, pacdbm, pacphase, rs, rp, cpar, isnoisy
    let vsource = registry.get(ComponentType::VoltageSource).unwrap();
    assert!(
        vsource.len() >= 12,
        "DC voltage source should have at least 12 Spectre-parity properties, got {}",
        vsource.len()
    );

    // DC Current Source: 11 properties (no rs - current sources don't have series resistance)
    // name, dc, ac, acphase, xfmag, pacmag, pacdbm, pacphase, rp, cpar, isnoisy
    let isource = registry.get(ComponentType::CurrentSource).unwrap();
    assert!(
        isource.len() >= 11,
        "DC current source should have at least 11 Spectre-parity properties, got {}",
        isource.len()
    );
}

#[test]
fn test_spectre_parity_dbm_defaults() {
    // PAC dBm should default to -inf (disabled) as per Spectre convention
    let registry = PropertyRegistry::new();
    let sheet = registry.get(ComponentType::VoltageSource).unwrap();

    let pacdbm = sheet.get("pacdbm").unwrap();
    match &pacdbm.default_value {
        PropertyValue::Number { value, .. } => {
            assert!(
                value.is_infinite() && value.is_sign_negative(),
                "pacdbm should default to -inf (disabled), got {}",
                value
            );
        }
        other => panic!("pacdbm should default to -inf, got {:?}", other),
    }
}

#[test]
fn test_spectre_parity_parallel_resistance_defaults() {
    // Parallel resistance should default to infinity (ideal source)
    let registry = PropertyRegistry::new();

    let vsource = registry.get(ComponentType::VoltageSource).unwrap();
    let rp = vsource.get("rp").unwrap();
    match &rp.default_value {
        PropertyValue::Number { value, .. } => {
            assert!(
                value.is_infinite() && value.is_sign_positive(),
                "Parallel resistance should default to +inf (ideal), got {}",
                value
            );
        }
        other => panic!("rp should default to +inf, got {:?}", other),
    }

    let isource = registry.get(ComponentType::CurrentSource).unwrap();
    let rp = isource.get("rp").unwrap();
    match &rp.default_value {
        PropertyValue::Number { value, .. } => {
            assert!(value.is_infinite() && value.is_sign_positive());
        }
        other => panic!("Current source rp should default to +inf, got {:?}", other),
    }
}

#[test]
fn test_spectre_parity_noisy_defaults_to_true() {
    // isnoisy should default to true (sources contribute noise by default)
    let registry = PropertyRegistry::new();

    for component_type in [
        ComponentType::VoltageSource,
        ComponentType::VoltageSourceAc,
        ComponentType::CurrentSource,
        ComponentType::CurrentSourceAc,
    ] {
        let sheet = registry.get(component_type).unwrap();
        let isnoisy = sheet
            .get("isnoisy")
            .expect(&format!("{:?} must have isnoisy", component_type));
        match &isnoisy.default_value {
            PropertyValue::Boolean(b) => {
                assert!(*b, "{:?} isnoisy should default to true", component_type);
            }
            other => panic!(
                "{:?} isnoisy should be boolean true, got {:?}",
                component_type, other
            ),
        }
    }
}

// =========================================================================
// Semiconductor Spectre-Parity Tests
// =========================================================================

#[test]
fn test_spectre_parity_mosfet_categories() {
    let registry = PropertyRegistry::new();
    let nmos = registry.get(ComponentType::Nmos).unwrap();
    let pmos = registry.get(ComponentType::Pmos).unwrap();

    // Both NMOS and PMOS should have same category structure
    for (name, sheet) in [("NMOS", nmos), ("PMOS", pmos)] {
        // Instance category
        assert!(
            sheet.get("name").is_some(),
            "{} must have instance name",
            name
        );

        // Model category
        let model = sheet
            .get("model")
            .expect(&format!("{} must have model", name));
        assert_eq!(model.category, "Model");

        // Geometry category - essential for MOSFET sizing
        let w = sheet.get("w").expect(&format!("{} must have width", name));
        assert_eq!(w.category, "Geometry");
        assert!(w.required, "Width should be required for MOSFETs");

        let l = sheet.get("l").expect(&format!("{} must have length", name));
        assert_eq!(l.category, "Geometry");
        assert!(l.required, "Length should be required for MOSFETs");

        let nf = sheet
            .get("nf")
            .expect(&format!("{} must have nf (fingers)", name));
        assert_eq!(nf.category, "Geometry");

        let m = sheet
            .get("m")
            .expect(&format!("{} must have multiplier", name));
        assert_eq!(m.category, "Geometry");

        // Parasitics category - S/D areas and perimeters
        let as_param = sheet
            .get("as")
            .expect(&format!("{} must have source area", name));
        assert_eq!(as_param.category, "Parasitics");

        let ad = sheet
            .get("ad")
            .expect(&format!("{} must have drain area", name));
        assert_eq!(ad.category, "Parasitics");

        let ps = sheet
            .get("ps")
            .expect(&format!("{} must have source perimeter", name));
        assert_eq!(ps.category, "Parasitics");

        let pd = sheet
            .get("pd")
            .expect(&format!("{} must have drain perimeter", name));
        assert_eq!(pd.category, "Parasitics");

        // Stress category - STI effects for advanced nodes
        assert!(
            sheet.get("sa").is_some(),
            "{} must have SA for STI stress",
            name
        );
        assert!(sheet.get("sb").is_some(), "{} must have SB", name);
        assert!(sheet.get("sd").is_some(), "{} must have SD", name);

        // Temperature category
        let dtemp = sheet
            .get("dtemp")
            .expect(&format!("{} must have dtemp", name));
        assert_eq!(dtemp.category, "Temperature");

        // Initial Conditions category
        assert!(sheet.get("off").is_some(), "{} must have off flag", name);
        assert!(sheet.get("ic_vgs").is_some(), "{} must have IC VGS", name);
        assert!(sheet.get("ic_vds").is_some(), "{} must have IC VDS", name);
        assert!(sheet.get("ic_vbs").is_some(), "{} must have IC VBS", name);
    }
}

#[test]
fn test_spectre_parity_mosfet_geometry_constraints() {
    let registry = PropertyRegistry::new();
    let nmos = registry.get(ComponentType::Nmos).unwrap();

    // Width should have reasonable range (1nm to 1mm)
    let w = nmos.get("w").unwrap();
    assert!(w.min_value.is_some(), "Width should have minimum");
    assert!(w.max_value.is_some(), "Width should have maximum");
    assert!(
        w.min_value.unwrap() >= 1e-9,
        "Min width should be at least 1nm"
    );
    assert!(
        w.max_value.unwrap() <= 1e-3,
        "Max width should be at most 1mm"
    );

    // Length should have reasonable range
    let l = nmos.get("l").unwrap();
    assert!(l.min_value.is_some() && l.max_value.is_some());

    // Multiplier should be >= 1
    let m = nmos.get("m").unwrap();
    assert!(m.min_value.unwrap() >= 1.0, "Multiplier must be at least 1");

    // Fingers should be >= 1
    let nf = nmos.get("nf").unwrap();
    assert!(nf.min_value.unwrap() >= 1.0, "Fingers must be at least 1");
}

#[test]
fn test_spectre_parity_diode_categories() {
    let registry = PropertyRegistry::new();
    let diode = registry.get(ComponentType::Diode).unwrap();

    // Instance
    assert!(diode.get("name").is_some());

    // Model
    let model = diode.get("model").expect("Diode must have model");
    assert_eq!(model.category, "Model");

    // Geometry
    let area = diode.get("area").expect("Diode must have area");
    assert_eq!(area.category, "Geometry");
    let pj = diode.get("pj").expect("Diode must have perimeter");
    assert_eq!(pj.category, "Geometry");
    let m = diode.get("m").expect("Diode must have multiplier");
    assert_eq!(m.category, "Geometry");

    // Temperature
    let dtemp = diode.get("dtemp").expect("Diode must have dtemp");
    assert_eq!(dtemp.category, "Temperature");

    // Initial Conditions
    let off = diode.get("off").expect("Diode must have off flag");
    assert_eq!(off.category, "Initial Conditions");
    let ic = diode.get("ic").expect("Diode must have initial voltage");
    assert_eq!(ic.category, "Initial Conditions");
}

#[test]
fn test_spectre_parity_bjt_categories() {
    let registry = PropertyRegistry::new();
    let npn = registry.get(ComponentType::NpnBjt).unwrap();
    let pnp = registry.get(ComponentType::PnpBjt).unwrap();

    for (name, sheet) in [("NPN", npn), ("PNP", pnp)] {
        // Instance
        assert!(
            sheet.get("name").is_some(),
            "{} must have instance name",
            name
        );

        // Model
        let model = sheet
            .get("model")
            .expect(&format!("{} must have model", name));
        assert_eq!(model.category, "Model");

        // Geometry - BJT has multiple area factors
        let area = sheet
            .get("area")
            .expect(&format!("{} must have area", name));
        assert_eq!(area.category, "Geometry");

        // Spectre has separate area factors for E, B, C
        let areab = sheet
            .get("areab")
            .expect(&format!("{} must have base area", name));
        assert_eq!(areab.category, "Geometry");

        let areac = sheet
            .get("areac")
            .expect(&format!("{} must have collector area", name));
        assert_eq!(areac.category, "Geometry");

        let m = sheet
            .get("m")
            .expect(&format!("{} must have multiplier", name));
        assert_eq!(m.category, "Geometry");

        // Temperature
        let dtemp = sheet
            .get("dtemp")
            .expect(&format!("{} must have dtemp", name));
        assert_eq!(dtemp.category, "Temperature");

        // Initial Conditions
        let off = sheet
            .get("off")
            .expect(&format!("{} must have off flag", name));
        assert_eq!(off.category, "Initial Conditions");

        // Region hint (Spectre feature for convergence)
        let region = sheet
            .get("region")
            .expect(&format!("{} must have region hint", name));
        assert_eq!(region.category, "Initial Conditions");
    }
}

#[test]
fn test_spectre_parity_jfet_categories() {
    let registry = PropertyRegistry::new();
    let njfet = registry.get(ComponentType::Njfet).unwrap();
    let pjfet = registry.get(ComponentType::Pjfet).unwrap();

    for (name, sheet) in [("NJFET", njfet), ("PJFET", pjfet)] {
        // Instance
        assert!(
            sheet.get("name").is_some(),
            "{} must have instance name",
            name
        );

        // Model
        let model = sheet
            .get("model")
            .expect(&format!("{} must have model", name));
        assert_eq!(model.category, "Model");

        // Geometry
        let area = sheet
            .get("area")
            .expect(&format!("{} must have area", name));
        assert_eq!(area.category, "Geometry");

        let m = sheet
            .get("m")
            .expect(&format!("{} must have multiplier", name));
        assert_eq!(m.category, "Geometry");

        // Temperature
        let dtemp = sheet
            .get("dtemp")
            .expect(&format!("{} must have dtemp", name));
        assert_eq!(dtemp.category, "Temperature");

        // Initial Conditions
        let off = sheet
            .get("off")
            .expect(&format!("{} must have off flag", name));
        assert_eq!(off.category, "Initial Conditions");

        // JFET should have IC for VGS and VDS
        assert!(sheet.get("ic_vgs").is_some(), "{} must have IC VGS", name);
        assert!(sheet.get("ic_vds").is_some(), "{} must have IC VDS", name);
    }
}

// =========================================================================
// Controlled Source Spectre-Parity Tests
// =========================================================================

#[test]
fn test_spectre_parity_vcvs_categories() {
    let registry = PropertyRegistry::new();
    let vcvs = registry.get(ComponentType::Vcvs).unwrap();

    // Instance
    let name = vcvs.get("name").expect("VCVS must have name");
    assert_eq!(name.category, "Instance");

    // Electrical - main gain parameter
    let gain = vcvs.get("gain").expect("VCVS must have gain");
    assert_eq!(gain.category, "Electrical");
    assert!(gain.required, "Gain is essential for VCVS");

    let m = vcvs.get("m").expect("VCVS must have multiplier");
    assert_eq!(m.category, "Electrical");

    // Polynomial (for nonlinear behavior - Spectre feature)
    let poly = vcvs
        .get("poly")
        .expect("VCVS must have polynomial coefficients");
    assert_eq!(poly.category, "Polynomial");

    // AC parameters
    let ac_gain = vcvs.get("ac_gain").expect("VCVS must have AC gain");
    assert_eq!(ac_gain.category, "AC");

    // Limits (saturation - Spectre feature)
    let vmax = vcvs.get("vmax").expect("VCVS must have max output limit");
    assert_eq!(vmax.category, "Limits");
    let vmin = vcvs.get("vmin").expect("VCVS must have min output limit");
    assert_eq!(vmin.category, "Limits");
}

#[test]
fn test_spectre_parity_vccs_categories() {
    let registry = PropertyRegistry::new();
    let vccs = registry.get(ComponentType::Vccs).unwrap();

    // Instance
    assert!(vccs.get("name").is_some());

    // Electrical - transconductance (output current / input voltage)
    let gm = vccs.get("gm").expect("VCCS must have transconductance");
    assert_eq!(gm.category, "Electrical");
    assert!(gm.required, "Transconductance is essential for VCCS");

    // Output limits (current limits)
    let imax = vccs.get("imax").expect("VCCS must have max current limit");
    assert_eq!(imax.category, "Limits");
}

#[test]
fn test_spectre_parity_ccvs_categories() {
    let registry = PropertyRegistry::new();
    let ccvs = registry.get(ComponentType::Ccvs).unwrap();

    // Instance
    assert!(ccvs.get("name").is_some());

    // Electrical - transresistance (output voltage / input current)
    let rm = ccvs.get("rm").expect("CCVS must have transresistance");
    assert_eq!(rm.category, "Electrical");
    assert!(rm.required, "Transresistance is essential for CCVS");

    // Sensing reference (voltage source for current sensing)
    let vref = ccvs
        .get("vref")
        .expect("CCVS must have sensing branch reference");
    assert_eq!(vref.category, "Electrical");

    // Output limits
    let vmax = ccvs.get("vmax").expect("CCVS must have max voltage limit");
    assert_eq!(vmax.category, "Limits");
}

#[test]
fn test_spectre_parity_cccs_categories() {
    let registry = PropertyRegistry::new();
    let cccs = registry.get(ComponentType::Cccs).unwrap();

    // Instance
    assert!(cccs.get("name").is_some());

    // Electrical - current gain
    let gain = cccs.get("gain").expect("CCCS must have current gain");
    assert_eq!(gain.category, "Electrical");
    assert!(gain.required, "Current gain is essential for CCCS");

    // Sensing reference
    let vref = cccs
        .get("vref")
        .expect("CCCS must have sensing branch reference");
    assert_eq!(vref.category, "Electrical");

    // Polynomial (for nonlinear behavior)
    let poly = cccs.get("poly").expect("CCCS must have polynomial");
    assert_eq!(poly.category, "Polynomial");

    // Output limits
    let imax = cccs.get("imax").expect("CCCS must have max current limit");
    assert_eq!(imax.category, "Limits");
}

#[test]
fn test_spectre_parity_controlled_source_gain_defaults() {
    let registry = PropertyRegistry::new();

    // All controlled sources should default to unity gain
    let vcvs = registry.get(ComponentType::Vcvs).unwrap();
    let gain = vcvs.get("gain").unwrap();
    match &gain.default_value {
        PropertyValue::Number { value, .. } => {
            assert_eq!(*value, 1.0, "VCVS gain should default to 1.0");
        }
        other => panic!("VCVS gain should be number 1.0, got {:?}", other),
    }

    let cccs = registry.get(ComponentType::Cccs).unwrap();
    let gain = cccs.get("gain").unwrap();
    match &gain.default_value {
        PropertyValue::Number { value, .. } => {
            assert_eq!(*value, 1.0, "CCCS gain should default to 1.0");
        }
        other => panic!("CCCS gain should be number 1.0, got {:?}", other),
    }
}

#[test]
fn test_spectre_parity_transient_sources_have_all_categories() {
    let registry = PropertyRegistry::new();

    // All transient sources should have AC, Parasitics, and Noise categories
    let transient_sources = [
        ComponentType::VoltageSourcePulse,
        ComponentType::VoltageSourceSin,
        ComponentType::VoltageSourcePwl,
        ComponentType::VoltageSourceExp,
        ComponentType::VoltageSourceSffm,
        ComponentType::CurrentSourcePulse,
        ComponentType::CurrentSourceSin,
        ComponentType::CurrentSourcePwl,
        ComponentType::CurrentSourceExp,
        ComponentType::CurrentSourceNoise,
    ];

    for comp_type in transient_sources {
        let sheet = registry
            .get(comp_type)
            .expect(&format!("{:?} must be registered", comp_type));

        // AC parameters
        assert!(
            sheet.get("ac").is_some(),
            "{:?} must have AC magnitude",
            comp_type
        );
        assert!(
            sheet.get("acphase").is_some(),
            "{:?} must have AC phase",
            comp_type
        );

        // Advanced AC (Spectre XF/PAC)
        assert!(
            sheet.get("xfmag").is_some(),
            "{:?} must have XF magnitude",
            comp_type
        );
        assert!(
            sheet.get("pacmag").is_some(),
            "{:?} must have PAC magnitude",
            comp_type
        );

        // Parasitics - voltage sources have rs, current sources don't
        if comp_type.spice_prefix() == "V" {
            assert!(
                sheet.get("rs").is_some(),
                "{:?} (voltage) must have series resistance",
                comp_type
            );
        }
        assert!(
            sheet.get("rp").is_some(),
            "{:?} must have parallel resistance",
            comp_type
        );
        assert!(
            sheet.get("cpar").is_some(),
            "{:?} must have parasitic capacitance",
            comp_type
        );

        // Noise
        assert!(
            sheet.get("isnoisy").is_some(),
            "{:?} must have noise flag",
            comp_type
        );
    }
}

#[test]
fn test_spectre_parity_all_semiconductors_have_dtemp() {
    // All semiconductor devices must have instance temperature (dtemp)
    let registry = PropertyRegistry::new();

    let semiconductors = [
        ComponentType::Diode,
        ComponentType::Nmos,
        ComponentType::Pmos,
        ComponentType::NpnBjt,
        ComponentType::PnpBjt,
        ComponentType::Njfet,
        ComponentType::Pjfet,
    ];

    for comp_type in semiconductors {
        let sheet = registry
            .get(comp_type)
            .expect(&format!("{:?} must be registered", comp_type));
        let dtemp = sheet.get("dtemp").expect(&format!(
            "{:?} must have dtemp for temperature analysis",
            comp_type
        ));
        assert_eq!(
            dtemp.unit,
            Some("°C".to_string()),
            "{:?} dtemp should have °C unit",
            comp_type
        );
    }
}

#[test]
fn test_spectre_parity_all_semiconductors_have_off_flag() {
    // All semiconductor devices must have 'off' flag for DC analysis convergence
    let registry = PropertyRegistry::new();

    let semiconductors = [
        ComponentType::Diode,
        ComponentType::Nmos,
        ComponentType::Pmos,
        ComponentType::NpnBjt,
        ComponentType::PnpBjt,
        ComponentType::Njfet,
        ComponentType::Pjfet,
    ];

    for comp_type in semiconductors {
        let sheet = registry
            .get(comp_type)
            .expect(&format!("{:?} must be registered", comp_type));
        let off = sheet.get("off").expect(&format!(
            "{:?} must have 'off' flag for DC analysis",
            comp_type
        ));
        assert_eq!(
            off.prop_type,
            PropertyType::Boolean,
            "{:?} 'off' should be boolean",
            comp_type
        );
        // Should default to false (device on)
        match &off.default_value {
            PropertyValue::Boolean(b) => {
                assert!(!*b, "{:?} 'off' should default to false", comp_type);
            }
            other => panic!("{:?} 'off' should be boolean, got {:?}", comp_type, other),
        }
    }
}
