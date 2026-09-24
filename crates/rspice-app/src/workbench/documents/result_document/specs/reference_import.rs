//! Import and export retained comparison tables through the shared file picker.

use crate::io::file_exchange::{self, FileKind};
use crate::state::workspace::MeasurementReferenceSource;
use egui::Ui;

const TABLE: FileKind = FileKind {
    label: "Comparison tables",
    extensions: &["csv", "prn", "csd"],
    subject: "Reference table",
    fallback_name: "reference.csv",
};

#[derive(Debug)]
struct PendingExchange {
    context: egui::Context,
    id: egui::Id,
    saving: bool,
}

impl Drop for PendingExchange {
    fn drop(&mut self) {
        file_exchange::discard_exchange(&self.context, self.id);
    }
}

#[derive(Debug, Default)]
pub(super) struct ReferenceImport {
    pending: Option<PendingExchange>,
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
    ) -> bool {
        let mut changed = false;
        if let Some(pending) = &self.pending {
            let completed = if pending.saving {
                file_exchange::take_saved(ui.ctx(), pending.id).map(|outcome| {
                    outcome.map(|saved| {
                        if saved.is_some() {
                            self.message = Some("Reference export prepared.".into());
                        }
                    })
                })
            } else {
                file_exchange::take_opened(ui.ctx(), pending.id).map(|outcome| {
                    outcome.and_then(|opened| {
                        if let Some(opened) = opened {
                            *reference = Some(retain(opened.name, opened.text)?);
                            changed = true;
                            self.message = None;
                        }
                        Ok(())
                    })
                })
            };
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
                self.start(ui, None);
            }
            if let Some(source) = reference.as_ref()
                && ui
                    .add_enabled(
                        self.pending.is_none(),
                        egui::Button::new("Export reference…"),
                    )
                    .clicked()
            {
                self.start(ui, Some(source));
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

    fn start(&mut self, ui: &Ui, source: Option<&MeasurementReferenceSource>) {
        let pending = PendingExchange {
            context: ui.ctx().clone(),
            id: egui::Id::new(uuid::Uuid::new_v4()),
            saving: source.is_some(),
        };
        let result = if let Some(source) = source {
            let name = std::path::Path::new(&source.logical_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(TABLE.fallback_name)
                .to_owned();
            file_exchange::save_file(
                ui.ctx(),
                pending.id,
                TABLE,
                name,
                source.contents.as_bytes().to_vec(),
            )
        } else {
            file_exchange::open_file(
                ui.ctx(),
                pending.id,
                TABLE,
                rspice_core::ResourceLimits::default().max_netlist_bytes,
            )
        };
        match result {
            Ok(()) => {
                self.pending = Some(pending);
                self.message = None;
            }
            Err(error) => self.message = Some(error),
        }
    }
}
