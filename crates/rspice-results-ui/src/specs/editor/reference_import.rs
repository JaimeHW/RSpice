//! Reference-table controls over an application-owned file exchange.

use egui::Ui;
use rspice_results::specification::MeasurementReferenceSource;

/// A completed host file operation; cancellation is an absent completion.
pub enum ReferenceCompletion {
    Opened { name: String, text: String },
    Exported,
}

/// An in-flight picker. Dropping the host handle discards its pending exchange.
pub trait ReferenceExchange: std::fmt::Debug + Send + Sync {
    fn poll(&self, ctx: &egui::Context) -> Option<Result<Option<ReferenceCompletion>, String>>;
}

#[derive(Debug, Default)]
pub(super) struct ReferenceImport {
    pending: Option<Box<dyn ReferenceExchange>>,
    message: Option<String>,
}

// A copied draft does not own the original draft's in-flight picker.
impl Clone for ReferenceImport {
    fn clone(&self) -> Self {
        Self {
            pending: None,
            message: self.message.clone(),
        }
    }
}

pub(super) fn retain(name: String, text: String) -> Result<MeasurementReferenceSource, String> {
    let reference = MeasurementReferenceSource {
        logical_path: name,
        contents: text,
    };
    reference.validate()?;
    Ok(reference)
}

impl ReferenceImport {
    pub(super) fn show(
        &mut self,
        ui: &mut Ui,
        reference: &mut Option<MeasurementReferenceSource>,
        start: &mut impl FnMut(
            &egui::Context,
            Option<&MeasurementReferenceSource>,
        ) -> Result<Box<dyn ReferenceExchange>, String>,
    ) -> bool {
        let mut changed = false;
        if let Some(pending) = &self.pending {
            let completed = pending.poll(ui.ctx()).map(|outcome| {
                outcome.and_then(|completed| {
                    match completed {
                        Some(ReferenceCompletion::Exported) => {
                            self.message = Some("Reference export prepared.".into())
                        }
                        Some(ReferenceCompletion::Opened { name, text }) => {
                            *reference = Some(retain(name, text)?);
                            changed = true;
                            self.message = None;
                        }
                        None => {}
                    }
                    Ok(())
                })
            });
            if let Some(outcome) = completed {
                self.pending = None;
                if let Err(error) = outcome {
                    self.message = Some(error);
                }
            }
        }
        ui.horizontal_wrapped(|ui| {
            if ui
                .add_enabled(
                    self.pending.is_none(),
                    egui::Button::new("Attach reference table…"),
                )
                .clicked()
            {
                self.start(ui.ctx(), None, start);
            }
            if let Some(source) = reference.as_ref()
                && ui
                    .add_enabled(
                        self.pending.is_none(),
                        egui::Button::new("Export reference…"),
                    )
                    .clicked()
            {
                self.start(ui.ctx(), Some(source), start);
            }
            if reference.is_some() && ui.button("Remove reference").clicked() {
                self.pending = None;
                *reference = None;
                changed = true;
            }
            if self.pending.is_some() {
                ui.spinner();
            }
        });
        if let Some(source) = reference {
            ui.horizontal_wrapped(|ui| {
                ui.label("Reference name");
                changed |= ui.text_edit_singleline(&mut source.logical_path).changed();
                ui.label(format!(
                    "{} bytes retained with the project",
                    source.contents.len()
                ));
            });
            ui.small("Reference name must match FILE in the measurement card. Column numbers start at zero.");
            ui.small("For use outside Studio, export the reference table beside the netlist at the path named by FILE.");
            egui::CollapsingHeader::new("Reference preview").show(ui, |ui| {
                for line in source.contents.lines().take(5) {
                    ui.monospace(line.chars().take(240).collect::<String>());
                }
            });
        }
        if let Some(message) = &self.message {
            ui.label(message);
        }
        changed
    }

    fn start(
        &mut self,
        ctx: &egui::Context,
        source: Option<&MeasurementReferenceSource>,
        start: &mut impl FnMut(
            &egui::Context,
            Option<&MeasurementReferenceSource>,
        ) -> Result<Box<dyn ReferenceExchange>, String>,
    ) {
        match start(ctx, source) {
            Ok(pending) => {
                self.pending = Some(pending);
                self.message = None;
            }
            Err(error) => self.message = Some(error),
        }
    }
}
