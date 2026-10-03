//! Local specification drafts and measurement authoring controls.

mod measurement_builder;
mod reference_import;
pub use reference_import::{ReferenceCompletion, ReferenceExchange};

use egui::Ui;
use rspice_app_types::{
    product::AnalysisInstanceId, quantity::engineering::parse_engineering_value,
};
use rspice_results::specification::{
    MeasurementReferenceSource, SpecEntry, SpecPointScope, SpecificationComparison,
    SpecificationDefinition, SpecificationRole,
};
use rspice_ui_kit::{
    panels::{WorkbenchIcon, icon_button},
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

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
    const ALL: [Self; 5] = [
        Self::Tracked,
        Self::Minimum,
        Self::Maximum,
        Self::Range,
        Self::EqualWithin,
    ];

    const fn label(self) -> &'static str {
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

/// Edit local drafts using the host's current producer/scope choices and file picker.
pub fn show(
    ui: &mut Ui,
    drafts: &mut Vec<SpecDraft>,
    untracked: &[String],
    analysis_options: &[(AnalysisInstanceId, String)],
    scope_options: &[(String, SpecPointScope)],
    mut start_reference: impl FnMut(
        &egui::Context,
        Option<&MeasurementReferenceSource>,
    ) -> Result<Box<dyn ReferenceExchange>, String>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut remove: Option<usize> = None;

    egui::ScrollArea::both()
        .id_salt("rspice.results.specs-edit")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(
                        "Limits and guard bands accept engineering notation (10meg, 1u, 3.3). \
                         Requirement identity, comparison semantics, and authored expressions are retained exactly; results never rewrite them.",
                    )
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_dim),
                );
            });
            ui.add_space(6.0);

            // Column captions.
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                for (label, width) in [
                    ("REQUIREMENT KEY", 130.0),
                    ("REQUIREMENT NAME", 180.0),
                    ("MEASUREMENT", 150.0),
                    ("EXPRESSION", 230.0),
                ] {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(width, 18.0), egui::Sense::hover());
                    ui.painter().text(
                        egui::pos2(rect.left() + 4.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        label,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    );
                }
            });

            for (idx, draft) in drafts.iter_mut().enumerate() {
                let field = |ui: &mut Ui, text: &mut String, width: f32, hint: &str| {
                    ui.add(
                        egui::TextEdit::singleline(text)
                            .desired_width(width)
                            .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                            .hint_text(hint),
                    );
                };
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    field(ui, &mut draft.requirement_key, 130.0, "SPEC-…");
                    field(ui, &mut draft.requirement_name, 180.0, "requirement name");
                    field(ui, &mut draft.measurement, 150.0, "measurement");
                    field(ui, &mut draft.expression, 230.0, ".MEAS expression");
                    if icon_button(
                        ui,
                        WorkbenchIcon::Close,
                        "Remove spec",
                        false,
                        egui::vec2(22.0, 22.0),
                    )
                    .clicked()
                    {
                        remove = Some(idx);
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(10.0);
                    ui.checkbox(&mut draft.define_measurement, "Define measurement when running the plan");
                    ui.label(if draft.define_measurement {
                        "Enter a complete .MEAS card with the measurement name above. Manual netlists use their own cards."
                    } else {
                        "Reference an existing result; expression text is descriptive."
                    });
                });
                if draft.define_measurement {
                    ui.push_id(("measurement-builder", idx), |ui| {
                        if draft.measurement_builder.is_none() && ui.button("Build measurement card…").clicked() {
                            let mut builder = measurement_builder::MeasurementBuilder::for_card(&draft.expression);
                            if let Some(reference) = &draft.measurement_reference { builder.set_reference_path(&reference.logical_path); }
                            draft.measurement_builder = Some(builder);
                        }
                        let mut close = false;
                        if let Some(builder) = &mut draft.measurement_builder {
                            ui.group(|ui| {
                                if let Some(card) = builder.show(ui, &draft.measurement) {
                                    draft.expression = card;
                                    close = true;
                                }
                                close |= ui.button("Close builder").clicked();
                            });
                        }
                        if close { draft.measurement_builder = None; }
                    });
                }
                ui.push_id(("measurement-reference", idx), |ui| {
                    if draft.reference_import.show(ui, &mut draft.measurement_reference, &mut start_reference)
                        && let (Some(builder), Some(reference)) = (&mut draft.measurement_builder, &draft.measurement_reference) {
                            builder.set_reference_path(&reference.logical_path);
                        }
                });
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    egui::ComboBox::from_id_salt(("spec-comparison", idx))
                        .selected_text(draft.comparison.label())
                        .width(170.0)
                        .show_ui(ui, |ui| {
                            for comparison in ComparisonDraftKind::ALL {
                                ui.selectable_value(
                                    &mut draft.comparison,
                                    comparison,
                                    comparison.label(),
                                );
                            }
                        });
                    let (primary_hint, secondary_hint) = match draft.comparison {
                        ComparisonDraftKind::Tracked => ("no limit", ""),
                        ComparisonDraftKind::Minimum => ("minimum", ""),
                        ComparisonDraftKind::Maximum => ("maximum", ""),
                        ComparisonDraftKind::Range => ("range minimum", "range maximum"),
                        ComparisonDraftKind::EqualWithin => ("target", "tolerance"),
                    };
                    field(ui, &mut draft.primary_limit, 110.0, primary_hint);
                    field(ui, &mut draft.secondary_limit, 110.0, secondary_hint);
                    field(ui, &mut draft.guard_band, 100.0, "guard band");
                    ui.add(egui::TextEdit::singleline(&mut draft.unit)
                        .desired_width(80.0)
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .hint_text("unit"))
                        .on_hover_text("Unit for the limit and guard band, such as mV, ns, dB, or V^2/Hz. Leave blank to use the measurement's native unit.");
                    egui::ComboBox::from_id_salt(("spec-role", idx))
                        .selected_text(match draft.role {
                            SpecificationRole::Blocking => "Blocking",
                            SpecificationRole::Review => "Review",
                            SpecificationRole::Informational => "Informational",
                        })
                        .width(120.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut draft.role,
                                SpecificationRole::Blocking,
                                "Blocking",
                            );
                            ui.selectable_value(
                                &mut draft.role,
                                SpecificationRole::Review,
                                "Review",
                            );
                            ui.selectable_value(
                                &mut draft.role,
                                SpecificationRole::Informational,
                                "Informational",
                            );
                        });
                    if let Err(error) = draft.parse() {
                        ui.label(
                            egui::RichText::new(error)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.err),
                        );
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("APPLIES TO")
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                    let current_scope = scope_options
                        .iter()
                        .find(|(_, scope)| scope == &draft.scope)
                        .map_or_else(
                            || match &draft.scope {
                                SpecPointScope::SelectedCorners { corners } => {
                                    format!("Corners {}", corners.join(" · "))
                                }
                                SpecPointScope::AllPoints => "All PVT points".to_owned(),
                                SpecPointScope::Nominal => "Nominal only".to_owned(),
                            },
                            |(label, _)| label.clone(),
                        );
                    egui::ComboBox::from_id_salt(("spec-scope", idx))
                        .selected_text(current_scope)
                        .width(180.0)
                        .show_ui(ui, |ui| {
                            for (label, scope) in scope_options {
                                ui.selectable_value(&mut draft.scope, scope.clone(), label);
                            }
                        });
                    ui.label(
                        egui::RichText::new("PRODUCING ANALYSIS")
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                    let current_producer = draft.producing_analysis.map_or_else(
                        || "Any exact producer".to_owned(),
                        |id| {
                            analysis_options
                                .iter()
                                .find(|(candidate, _)| *candidate == id)
                                .map_or_else(
                                    || format!("Missing analysis · {id}"),
                                    |(_, label)| label.clone(),
                                )
                        },
                    );
                    egui::ComboBox::from_id_salt(("spec-producer", idx))
                        .selected_text(current_producer)
                        .width(280.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut draft.producing_analysis,
                                None,
                                "Any exact producer",
                            );
                            for (id, label) in analysis_options {
                                ui.selectable_value(
                                    &mut draft.producing_analysis,
                                    Some(*id),
                                    label,
                                );
                            }
                        });
                });
                if let Some(original) = draft.original.as_ref() {
                    let source = original.source.as_ref().map_or_else(
                        || "authored in this plan".to_owned(),
                        |source| {
                            format!(
                                "{}:{} · revision {} · digest {}",
                                source.logical_path,
                                source.row,
                                source.imported_revision,
                                source.source_digest
                            )
                        },
                    );
                    let waiver = original.waiver.as_ref().map_or_else(
                        || "none".to_owned(),
                        |waiver| {
                            format!(
                                "{} · owner {} · {}",
                                waiver.reference, waiver.owner, waiver.rationale
                            )
                        },
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Stable ID {} · source {source} · waiver/disposition {waiver}",
                                original.id
                            ))
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                        );
                    });
                }
                ui.add_space(2.0);
            }

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                if ui.button("Add spec").clicked() {
                    drafts.push(SpecDraft::default());
                }
            });

            // One-click drafts for measurements seen in runs but unbound.
            let missing: Vec<&String> = untracked
                .iter()
                .filter(|name| {
                    !drafts
                        .iter()
                        .any(|d| d.measurement.eq_ignore_ascii_case(name))
                })
                .collect();
            if !missing.is_empty() {
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("discovered:")
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                    let mut add: Option<String> = None;
                    for name in missing {
                        if ui.small_button(name.as_str()).clicked() {
                            add = Some(name.clone());
                        }
                    }
                    if let Some(name) = add {
                        drafts.push(SpecDraft {
                            requirement_name: name.clone(),
                            measurement: name,
                            ..Default::default()
                        });
                    }
                });
            }
        });

    if let Some(idx) = remove {
        drafts.remove(idx);
    }
}
