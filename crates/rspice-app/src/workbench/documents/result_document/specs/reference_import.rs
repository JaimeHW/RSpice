//! Import and export retained comparison tables through the shared file picker.

use crate::io::file_exchange::{self, FileKind};
use crate::state::workspace::MeasurementReferenceSource;
use rspice_results_ui::specs::editor::{ReferenceCompletion, ReferenceExchange};

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

impl ReferenceExchange for PendingExchange {
    fn poll(&self, ctx: &egui::Context) -> Option<Result<Option<ReferenceCompletion>, String>> {
        if self.saving {
            file_exchange::take_saved(ctx, self.id)
                .map(|outcome| outcome.map(|saved| saved.map(|_| ReferenceCompletion::Exported)))
        } else {
            file_exchange::take_opened(ctx, self.id).map(|outcome| {
                outcome.map(|opened| {
                    opened.map(|opened| ReferenceCompletion::Opened {
                        name: opened.name,
                        text: opened.text,
                    })
                })
            })
        }
    }
}

pub(super) fn start(
    ctx: &egui::Context,
    source: Option<&MeasurementReferenceSource>,
) -> Result<Box<dyn ReferenceExchange>, String> {
    let pending = PendingExchange {
        context: ctx.clone(),
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
            ctx,
            pending.id,
            TABLE,
            name,
            source.contents.as_bytes().to_vec(),
        )
    } else {
        file_exchange::open_file(
            ctx,
            pending.id,
            TABLE,
            rspice_core::ResourceLimits::default().max_netlist_bytes,
        )
    };
    result.map(|()| Box::new(pending) as Box<dyn ReferenceExchange>)
}
