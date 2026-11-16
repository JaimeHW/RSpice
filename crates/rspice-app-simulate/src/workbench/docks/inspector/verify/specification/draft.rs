//! A specification's definition as the inspector holds it while it is being
//! edited, and the aids that author its measurement: the measurement-card
//! builder and the reference-table picker.

use egui::Ui;
use rspice_results::specification::{
    MeasurementReferenceSource, SpecEntry, SpecPointScope, SpecificationComparison,
    SpecificationDefinition, SpecificationRole,
};
use rspice_vocabulary::{
    product::AnalysisInstanceId, quantity::engineering::parse_engineering_value,
};

use super::measurement_builder;
use super::reference_import::{self, ReferenceExchange};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComparisonDraftKind {
    #[default]
    Tracked,
    Minimum,
    Maximum,
    Range,
    EqualWithin,
}

impl ComparisonDraftKind {
    pub const ALL: [Self; 5] = [
        Self::Tracked,
        Self::Minimum,
        Self::Maximum,
        Self::Range,
        Self::EqualWithin,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Tracked => "Track only",
            Self::Minimum => "At least",
            Self::Maximum => "At most",
            Self::Range => "Inside range",
            Self::EqualWithin => "Equals within tolerance",
        }
    }
}

/// One governed requirement being edited. Numeric values remain text so
/// partial engineering notation survives frames. `original` carries the
/// immutable identity plus imported source and waiver metadata that ordinary
/// scalar edits are not authorized to rewrite.
#[derive(Debug, Clone, Default)]
pub struct SpecDraft {
    original: Option<SpecificationDefinition>,
    pub requirement_key: String,
    pub requirement_name: String,
    pub measurement: String,
    pub expression: String,
    pub define_measurement: bool,
    pub measurement_reference: Option<MeasurementReferenceSource>,
    reference_import: reference_import::ReferenceImport,
    measurement_builder: Option<measurement_builder::MeasurementBuilder>,
    pub comparison: ComparisonDraftKind,
    /// Minimum/maximum limit, range minimum, or equality target.
    pub primary_limit: String,
    /// Range maximum or equality tolerance.
    pub secondary_limit: String,
    pub guard_band: String,
    pub unit: String,
    pub role: SpecificationRole,
    pub scope: SpecPointScope,
    pub producing_analysis: Option<AnalysisInstanceId>,
}

impl SpecDraft {
    pub fn from_definition(definition: &SpecificationDefinition) -> Self {
        let (comparison, primary_limit, secondary_limit) = match definition.comparison {
            SpecificationComparison::Tracked => {
                (ComparisonDraftKind::Tracked, String::new(), String::new())
            }
            SpecificationComparison::Minimum { limit } => (
                ComparisonDraftKind::Minimum,
                limit.to_string(),
                String::new(),
            ),
            SpecificationComparison::Maximum { limit } => (
                ComparisonDraftKind::Maximum,
                limit.to_string(),
                String::new(),
            ),
            SpecificationComparison::Range { minimum, maximum } => (
                ComparisonDraftKind::Range,
                minimum.to_string(),
                maximum.to_string(),
            ),
            SpecificationComparison::EqualWithin { target, tolerance } => (
                ComparisonDraftKind::EqualWithin,
                target.to_string(),
                tolerance.to_string(),
            ),
        };
        Self {
            original: Some(definition.clone()),
            requirement_key: definition.requirement_key.clone(),
            requirement_name: definition.requirement_name.clone(),
            measurement: definition.measurement.clone(),
            expression: definition.expression.clone(),
            define_measurement: definition.define_measurement,
            measurement_reference: definition.measurement_reference.clone(),
            reference_import: Default::default(),
            measurement_builder: None,
            comparison,
            primary_limit,
            secondary_limit,
            guard_band: definition
                .guard_band
                .map(|value| value.to_string())
                .unwrap_or_default(),
            unit: definition.unit.clone(),
            role: definition.role,
            scope: definition.scope.clone(),
            producing_analysis: definition.producing_analysis,
        }
    }

    /// Take over the reference picker `from` is waiting on, when `from` is the
    /// draft this one copies.
    ///
    /// A copied draft starts without its original's picker, because a picker
    /// answers one draft. A host that replaces its live drafts with copies
    /// therefore moves each picker across before the originals drop, or the
    /// file the operator is choosing is discarded with them.
    pub fn adopt_reference_exchange(&mut self, from: &mut Self) {
        if self.original == from.original {
            self.reference_import.adopt(&mut from.reference_import);
        }
    }

    /// Whether a reference picker opened from this draft is still waiting.
    #[cfg(test)]
    pub fn reference_exchange_pending(&self) -> bool {
        self.reference_import.is_pending()
    }

    /// Wait on `exchange` as the picker the operator opened from this draft.
    #[cfg(test)]
    pub fn await_reference_exchange(&mut self, exchange: Box<dyn ReferenceExchange>) {
        self.reference_import.await_exchange(exchange);
    }

    /// Parse into a complete governed definition. `Err` identifies the field
    /// or domain invariant that refused the draft.
    pub fn parse(&self) -> Result<Option<SpecificationDefinition>, String> {
        let name = self.measurement.trim();
        if name.is_empty() {
            if self.define_measurement || self.measurement_reference.is_some() {
                return Err("Measurement name is required".into());
            }
            return Ok(None); // blank rows are simply dropped
        }
        let required_value = |text: &str, field: &str| -> Result<f64, String> {
            let text = text.trim();
            if text.is_empty() {
                return Err(format!("{field} is required"));
            }
            parse_engineering_value(text).map_err(|_| format!("{field} is invalid"))
        };
        let comparison = match self.comparison {
            ComparisonDraftKind::Tracked => SpecificationComparison::Tracked,
            ComparisonDraftKind::Minimum => SpecificationComparison::Minimum {
                limit: required_value(&self.primary_limit, "minimum limit")?,
            },
            ComparisonDraftKind::Maximum => SpecificationComparison::Maximum {
                limit: required_value(&self.primary_limit, "maximum limit")?,
            },
            ComparisonDraftKind::Range => SpecificationComparison::Range {
                minimum: required_value(&self.primary_limit, "range minimum")?,
                maximum: required_value(&self.secondary_limit, "range maximum")?,
            },
            ComparisonDraftKind::EqualWithin => SpecificationComparison::EqualWithin {
                target: required_value(&self.primary_limit, "equality target")?,
                tolerance: required_value(&self.secondary_limit, "equality tolerance")?,
            },
        };
        let projected = SpecEntry {
            measurement: name.to_owned(),
            expression: self.expression.trim().to_owned(),
            min: None,
            max: None,
            unit: self.unit.trim().to_owned(),
            scope: self.scope.clone(),
        };
        let mut definition = self
            .original
            .clone()
            .unwrap_or_else(|| SpecificationDefinition::new_from_projection(&projected));
        let requirement_key = self.requirement_key.trim();
        let requirement_name = self.requirement_name.trim();
        if !requirement_key.is_empty() {
            definition.requirement_key = requirement_key.to_owned();
        }
        definition.requirement_name = if requirement_name.is_empty() {
            name.to_owned()
        } else {
            requirement_name.to_owned()
        };
        definition.measurement = name.to_owned();
        definition.expression = self.expression.trim().to_owned();
        definition.define_measurement = self.define_measurement;
        definition.measurement_reference = self.measurement_reference.clone();
        definition.producing_analysis = self.producing_analysis;
        definition.comparison = comparison;
        definition.guard_band = if self.guard_band.trim().is_empty() {
            None
        } else {
            Some(required_value(&self.guard_band, "guard band")?)
        };
        definition.role = self.role;
        definition.scope = self.scope.clone();
        definition.unit = self.unit.trim().to_owned();
        definition.validate()?;
        Ok(Some(definition))
    }
}

/// The authoring aids of one draft: the measurement-card builder, while the
/// draft defines its own measurement, and the reference-result picker.
/// Returns whether either changed the draft — a card built into its
/// expression, or a reference chosen or cleared — so a host that commits each
/// change knows to.
pub fn measurement_authoring(
    ui: &mut Ui,
    draft: &mut SpecDraft,
    start_reference: &mut impl FnMut(
        &egui::Context,
        Option<&MeasurementReferenceSource>,
    ) -> Result<Box<dyn ReferenceExchange>, String>,
) -> bool {
    let mut changed = false;
    if draft.define_measurement {
        ui.push_id("measurement-builder", |ui| {
            if draft.measurement_builder.is_none() && ui.button("Build measurement card…").clicked()
            {
                let mut builder =
                    measurement_builder::MeasurementBuilder::for_card(&draft.expression);
                if let Some(reference) = &draft.measurement_reference {
                    builder.set_reference_path(&reference.logical_path);
                }
                draft.measurement_builder = Some(builder);
            }
            let mut close = false;
            if let Some(builder) = &mut draft.measurement_builder {
                ui.group(|ui| {
                    if let Some(card) = builder.show(ui, &draft.measurement) {
                        draft.expression = card;
                        changed = true;
                        close = true;
                    }
                    close |= ui.button("Close builder").clicked();
                });
            }
            if close {
                draft.measurement_builder = None;
            }
        });
    }
    ui.push_id("measurement-reference", |ui| {
        if draft
            .reference_import
            .show(ui, &mut draft.measurement_reference, start_reference)
        {
            changed = true;
            if let (Some(builder), Some(reference)) =
                (&mut draft.measurement_builder, &draft.measurement_reference)
            {
                builder.set_reference_path(&reference.logical_path);
            }
        }
    });
    changed
}
