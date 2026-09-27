//! Importing a symbol definition.
//!
//! The formats a definition can arrive in, and the parse into the internal
//! model.

use super::*;

#[derive(Debug, Clone)]
pub struct SymbolDefinitionImport {
    pub definition: ModelBoundSymbolDefinition,
    pub report: SymbolImportReport,
}

impl SymbolDefinitionImport {
    /// Import a canonical definition or bounded graphic source. SVG/EDIF
    /// callers must supply an explicit definition; geometry never supplies
    /// or changes electrical semantics.
    pub fn from_bytes(
        bytes: &[u8],
        source_name: &str,
        explicit_contract: Option<ModelBoundSymbolDefinition>,
    ) -> Result<Self, SymbolDefinitionError> {
        let format = SymbolImportFormat::detect(bytes, source_name)?;
        match format {
            SymbolImportFormat::RSpiceJson => {
                if explicit_contract.is_some() {
                    return Err(SymbolDefinitionError::Import(
                        "an explicit contract cannot override canonical symbol JSON".to_owned(),
                    ));
                }
                let definition = ModelBoundSymbolDefinition::from_json_bytes(bytes, source_name)?;
                let report = report_for_definition(format, &definition, Vec::new());
                Ok(Self { definition, report })
            }
            SymbolImportFormat::Svg | SymbolImportFormat::Edif | SymbolImportFormat::LtspiceAsy => {
                let graphic_format = match format {
                    SymbolImportFormat::Svg => ImportedGraphicFormat::Svg,
                    SymbolImportFormat::Edif => ImportedGraphicFormat::Edif,
                    SymbolImportFormat::LtspiceAsy => ImportedGraphicFormat::LtspiceAsy,
                    SymbolImportFormat::RSpiceJson => unreachable!(),
                };
                let imported =
                    ImportedGraphicSource::from_bytes(graphic_format, source_name, bytes)?;
                let mut definition = explicit_contract.ok_or_else(|| {
                    SymbolDefinitionError::Import(format!(
                        "{source_name}: {} geometry has no electrical semantics; choose a model/pin contract or Blank explicit contract",
                        format.label()
                    ))
                })?;
                definition.imported_graphic = Some(imported);
                validate_import_pin_anchors(&definition)?;
                definition.validate()?;
                let warnings = definition
                    .source
                    .is_explicitly_unbound_for_review()
                    .then(|| "graphic is explicitly unbound and remains review-only".to_owned())
                    .into_iter()
                    .collect();

                let report = report_for_definition(format, &definition, warnings);
                Ok(Self { definition, report })
            }
        }
    }
}

fn report_for_definition(
    format: SymbolImportFormat,

    definition: &ModelBoundSymbolDefinition,
    warnings: Vec<String>,
) -> SymbolImportReport {
    SymbolImportReport::for_definition(
        format,
        definition,
        warnings,
        definition.imported_graphic.as_ref().map_or_else(
            || {
                crate::state::materialize_symbol_document(definition)
                    .body
                    .len()
            },
            |source| source.primitive_count,
        ),
        definition.imported_graphic.as_ref().map_or_else(
            || {
                crate::state::materialize_symbol_document(definition)
                    .pins
                    .len()
            },
            |source| source.pin_anchors.len(),
        ),
    )
}
