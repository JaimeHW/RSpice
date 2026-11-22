//! What an imported drawing held that did not come across.
//!
//! An import of another program's schematic brings most of it across and
//! says what it could not: a symbol with no part, a card nothing runs, a
//! file it could not read. Each such thing is a row of the Problems page,
//! filed under the drawing the import made, so a partial import does not
//! read as a complete one, as KiCad's and Altium's importers end on a report
//! of what they skipped.

use std::sync::Arc;

use super::diagnostic_store::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, mint_canonical,
};
use crate::diagnostics::LogAnchor;
use crate::product::ContentDigest;
use crate::state::{CellViewRef, Point};
use crate::workbench::app_state::model::{Model, SlotFacts};

/// Producer identity every import record carries.
const IMPORT_PRODUCER: &str = "rspice.design.foreign-import";

/// The rule code of a part of the source file that did not come across.
const LEFT_OUT: &str = "IMPORT-LEFT-OUT";

impl Model {
    /// Publish what the import of `source` into `drawing` could not bring
    /// across, one row each, replacing what an earlier import into that
    /// drawing published. Following a row opens the drawing at `centre`.
    pub fn publish_import_refusals(
        &mut self,
        drawing: &CellViewRef,
        source: &str,
        centre: Point,
        refusals: &[String],
        facts: SlotFacts,
    ) {
        let project = self.workspace.content.project.id();
        let revision = self.workspace.content.project.revision().get();
        let document: Arc<str> = Arc::from(drawing.key().as_str());
        let keys = refusals
            .iter()
            .map(|refusal| {
                FindingKey::new(
                    IMPORT_PRODUCER,
                    LEFT_OUT,
                    Arc::clone(&document),
                    refusal.as_str(),
                )
            })
            .collect::<Vec<_>>();
        let canonicals = mint_canonical(&keys, revision, "import");
        let route = DiagnosticRoute::Anchor(LogAnchor::Schematic {
            cell_view: drawing.clone(),
            x: centre.x,
            y: centre.y,
            component: None,
            wire: None,
            sheet: None,
        });
        let records = refusals
            .iter()
            .zip(keys)
            .zip(canonicals)
            .map(|((refusal, key), canonical)| {
                DiagnosticRecord::new(
                    canonical,
                    DiagnosticLevel::Warning,
                    DiagnosticSubject::new(project, format!("Import of {source}"))
                        .in_cell_view(drawing.clone()),
                    refusal.clone(),
                )
                .keyed(key)
                .with_route(Some(route.clone()))
            })
            .collect();
        let inputs = DiagnosticInputs::Import {
            subject: drawing.clone(),
        };
        // Repeated refusals are told apart by their occurrence; the drawing
        // can still fail to digest, which the log then says.
        if let Err(error) = self.publish_diagnostics(inputs, records, facts) {
            self.record(
                crate::diagnostics::LogSeverity::Warning,
                crate::diagnostics::LogSource::Studio,
                format!("What the import left out could not be listed in Problems: {error}"),
            );
        }
    }
}

/// The digest of the drawing an import made, as the design now holds it:
/// none once the drawing is gone.
pub(super) fn live_input_digest(
    model: &Model,
    subject: &CellViewRef,
) -> Result<ContentDigest, String> {
    let projection = model
        .design_projection_through_gesture()
        .map_err(|error| error.to_string())?;
    let key = subject.key();
    let drawing = projection
        .schematic_buffers()
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(&key))
        .map(|(_, schematic)| {
            schematic
                .content_digest()
                .ok_or_else(|| format!("could not digest schematic {key}"))
        })
        .transpose()?;
    super::diagnostic_store::digest_of(&(model.workspace.content.project.id(), subject, drawing))
}
