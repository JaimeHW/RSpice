//! Portable symbol validation at the retained-source boundary.

use super::*;

#[test]
fn retained_graphics_and_electrical_contracts_survive_json_without_losing_validation() {
    let mut definition = ModelBoundSymbolDefinition::review_only("symbols", "buffer");
    definition.imported_graphic = Some(
        ImportedGraphicSource::from_bytes(
            ImportedGraphicFormat::Svg,
            "buffer.svg",
            br#"<svg><rect x="0" y="0" width="20" height="10"/></svg>"#,
        )
        .unwrap(),
    );
    definition.pins = vec![SymbolPinDefinition::new(
        "IN",
        SymbolElectricalType::Analog,
        PortDirection::In,
        SymbolPinSide::Left,
        1,
    )];
    definition.source = SymbolSourceContract::existing_schematic_pins(
        "schematic",
        definition
            .pins
            .iter()
            .map(SymbolPinDefinition::port_spec)
            .collect(),
    );
    definition.netlist = SymbolNetlistBinding {
        device_prefix: "X".to_owned(),
        model: None,
        template: "X{name} {nodes} {model}".to_owned(),
        parameter_order: Vec::new(),
    };
    definition.validate().unwrap();
    let restored = ModelBoundSymbolDefinition::from_json_bytes(
        &serde_json::to_vec(&definition).unwrap(),
        "buffer.rspicesym",
    )
    .unwrap();
    assert_eq!(restored, definition);
    assert_eq!(
        restored.validation_digest().unwrap(),
        definition.validation_digest().unwrap()
    );

    let mut changed = restored.clone();
    changed.imported_graphic.as_mut().unwrap().shapes.clear();
    assert_eq!(
        changed.validate(),
        Err(SymbolDefinitionError::Import(
            "retained typed graphic does not match its source".to_owned(),
        ))
    );
    let mut changed = restored;
    changed.pins[0].direction = PortDirection::Out;
    assert!(matches!(
        changed.validate(),
        Err(SymbolDefinitionError::SourcePinMismatch(_))
    ));
}

#[test]
fn graphic_decoder_retains_the_byte_limit_and_active_content_refusal() {
    let oversized = vec![b' '; MAX_IMPORTED_GRAPHIC_BYTES + 1];
    assert_eq!(
        ImportedGraphicSource::from_bytes(ImportedGraphicFormat::Svg, "large.svg", &oversized),
        Err(SymbolDefinitionError::Import(
            "large.svg: graphic source exceeds the 1048576-byte limit".to_owned(),
        )),
    );
    assert_eq!(
        ImportedGraphicSource::from_bytes(
            ImportedGraphicFormat::Svg,
            "active.svg",
            b"<svg><script/></svg>",
        ),
        Err(SymbolDefinitionError::Import(
            "active.svg: SVG contains forbidden active or external content `<script`".to_owned(),
        )),
    );
}
