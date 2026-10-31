//! Characters no typeface here can letter, named on the Problems page.
//!
//! Text is never refused for the faces it needs: it is kept as entered, and a
//! character no embedded face and no face installed on the machine has shows
//! on the canvas and prints as the replacement square. This names each such
//! character once, with every place in the project it stands, so the square
//! is found on the screen rather than on an issued sheet.
//!
//! Whether an installed face has a character is the system-font worker's
//! answer, which comes on a later frame. A character not settled yet is asked
//! about, and named once no face is found to have it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use rspice_design::drc::DrcLocation;
use rspice_design::schematic::document::SchematicDocument;
use rspice_schematic_editor::view::drawing_sheet::DrawingSheetGeometry;
use rspice_ui_kit::fonts::coverage::{self, Coverage};
use unicode_normalization::UnicodeNormalization;

use super::diagnostic_store::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, RelatedPlace, mint_canonical,
};
use crate::diagnostics::LogAnchor;
use crate::product::{ContentDigest, ProjectId};
use crate::state::{
    CellViewRef, DrawingSheetInheritance, DrawingSheetTitleFieldId,
    DrawingSheetTitleFieldValueAuthority, Point, SchematicSheetFormat, SheetId,
};
use crate::workbench::app_state::model::{Model, SlotFacts};

/// Producer identity every lettering record carries.
const LETTERING_PRODUCER: &str = "rspice.text.lettering";

/// The code a character no face has is published under.
const UNCOVERED_CHARACTER: &str = "LETTERING-UNCOVERED-CHARACTER";

impl Model {
    /// Name every character no face here has. The text is read again only
    /// when the project's text or what the faces are known to have moves: a
    /// frame where neither did reads one remembered digest.
    pub fn refresh_lettering(&mut self, facts: SlotFacts) {
        let inputs = DiagnosticInputs::Lettering;
        if self
            .diagnostics
            .batch(&inputs)
            .is_some_and(|batch| self.diagnostic_batch_standing(batch, facts).is_current())
        {
            return;
        }
        self.publish_lettering(facts, &coverage::coverage);
    }

    /// Publish each character `covers` finds no face has, and ask about the
    /// ones it cannot answer yet.
    fn publish_lettering(&mut self, facts: SlotFacts, covers: &dyn Fn(char) -> Coverage) {
        let found = scan(self, covers);
        coverage::ask(found.unsettled.iter().copied());
        let records = lettering_records(self, &found);
        // One record per character, so the batch never mints one identity
        // twice; the digest is the lettering's own. Neither refusal the
        // store has can occur.
        if let Err(error) = self.publish_diagnostics(DiagnosticInputs::Lettering, records, facts) {
            debug_assert!(false, "the lettering records were refused: {error}");
        }
    }
}

/// What a pass reads, by the stamps that move when it does: the project
/// file, its sheets and title values, every drawing's text, and what the
/// faces are known to have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) struct LetteringKey {
    project: ProjectId,
    file: String,
    design_management: u64,
    active: (String, u64),
    buffers: Vec<(String, u64)>,
    settled: u64,
}

/// The key of what a pass would read now.
pub(in crate::workbench) fn inputs_key(model: &Model) -> LetteringKey {
    let content = &model.workspace.content;
    let mut buffers = content
        .schematic_buffers
        .iter()
        .map(|(key, schematic)| (key.clone(), schematic.content_stamp()))
        .collect::<Vec<_>>();
    buffers.sort_unstable();
    LetteringKey {
        project: content.project.id(),
        file: super::rule_deck::project_file_label(model),
        design_management: content.design_management.stamp(),
        active: (
            content.active_schematic_reference().key(),
            model.schematic.content_stamp(),
        ),
        buffers,
        settled: coverage::changes(),
    }
}

/// The digest of what a pass would publish: each character no face has,
/// with its places, and the characters not settled yet.
///
/// Over the findings rather than the text: an edit that names no new
/// character must not stale the records, while a character settling, or a
/// place moving, must.
pub(super) fn live_input_digest(model: &Model) -> Result<ContentDigest, String> {
    let found = scan(model, &coverage::coverage);
    let missing = found
        .missing
        .iter()
        .map(|(character, places)| {
            let places = places
                .iter()
                .map(|place| {
                    (
                        &place.label,
                        place.cell_view.key(),
                        place.at.x,
                        place.at.y,
                        place.component,
                        place.sheet,
                    )
                })
                .collect::<Vec<_>>();
            (u32::from(*character), places)
        })
        .collect::<Vec<_>>();
    let unsettled = found
        .unsettled
        .iter()
        .map(|character| u32::from(*character))
        .collect::<Vec<_>>();
    super::diagnostic_store::digest_of(&(
        model.workspace.content.project.id(),
        super::rule_deck::project_file_label(model),
        missing,
        unsettled,
    ))
}

/// One place a character stands: what it is, and where it is drawn.
#[derive(Debug, Clone)]
struct Place {
    label: String,
    cell_view: CellViewRef,
    location: DrcLocation,
    at: Point,
    component: Option<u64>,
    sheet: Option<SheetId>,
}

impl Place {
    fn route(&self) -> DiagnosticRoute {
        DiagnosticRoute::Anchor(LogAnchor::Schematic {
            cell_view: self.cell_view.clone(),
            x: self.at.x,
            y: self.at.y,
            component: self.component,
            wire: None,
            sheet: self.sheet,
        })
    }
}

/// What one pass found.
#[derive(Debug, Default)]
struct Found {
    /// Each character no face has, with every place it stands, in order.
    missing: BTreeMap<char, Vec<Place>>,
    /// The characters an installed face may still turn out to have.
    unsettled: BTreeSet<char>,
}

impl Found {
    /// Read one lettered text, standing where `place` says.
    fn read(
        &mut self,
        text: &str,
        covers: &dyn Fn(char) -> Coverage,
        place: impl FnOnce() -> Place,
    ) {
        // Every face this build letters in has ASCII. A file written
        // elsewhere may hold text decomposed; it is read as it letters.
        if text.is_ascii() {
            return;
        }
        let mut missing = BTreeSet::new();
        for character in text.nfc() {
            match covers(character) {
                Coverage::Lettered => {}
                Coverage::Unsettled => {
                    self.unsettled.insert(character);
                }
                Coverage::Missing => {
                    missing.insert(character);
                }
            }
        }
        if missing.is_empty() {
            return;
        }
        let place = place();
        for character in missing {
            self.missing
                .entry(character)
                .or_default()
                .push(place.clone());
        }
    }
}

/// Read every text the project letters: the title values of its sheets, and
/// the parts, notes and net labels of its drawings.
fn scan(model: &Model, covers: &dyn Fn(char) -> Coverage) -> Found {
    let mut found = Found::default();
    title_values(model, covers, &mut found);
    let content = &model.workspace.content;
    let active = content.active_schematic_reference();
    let active_key = active.key();
    drawing(
        model,
        &active,
        model.schematic.document(),
        covers,
        &mut found,
    );
    let mut buffers = content
        .schematic_buffers
        .iter()
        .filter(|(key, _)| !key.eq_ignore_ascii_case(&active_key))
        .filter_map(|(key, schematic)| Some((CellViewRef::from_key(key)?, schematic)))
        .collect::<Vec<_>>();
    buffers.sort_by_key(|(cell_view, _)| cell_view.key());
    for (cell_view, schematic) in buffers {
        drawing(model, &cell_view, schematic.document(), covers, &mut found);
    }
    found
}

/// The title values: the ones every sheet shares and the revision, which
/// stand on the sheet in view, and each sheet's own.
fn title_values(model: &Model, covers: &dyn Fn(char) -> Coverage, found: &mut Found) {
    let content = &model.workspace.content;
    let settings = content.design_management.drawing_sheet_settings();
    let active = content.active_schematic_reference();
    let active_sheet = content
        .design_management
        .sheet_catalog(&active.key())
        .and_then(|catalog| catalog.active())
        .map(|sheet| sheet.id());
    let in_view = title_block_centre(&model.active_drawing_sheet().format);
    let shared = |field: DrawingSheetTitleFieldId| Place {
        label: format!("Title block \u{00b7} {}", field.display_label()),
        cell_view: active.clone(),
        location: DrcLocation::Point {
            x: f64::from(in_view.x),
            y: f64::from(in_view.y),
        },
        at: in_view,
        component: None,
        sheet: active_sheet,
    };
    for (field, value) in &settings.title_block_field_values {
        found.read(value, covers, || shared(*field));
    }
    found.read(&settings.document_control.revision, covers, || {
        shared(DrawingSheetTitleFieldId::Revision)
    });
    for (key, catalog) in content.design_management.sheet_catalogs() {
        let Some(cell_view) = CellViewRef::from_key(key) else {
            continue;
        };
        for sheet in catalog.sheets() {
            let format = match sheet.page_format().inheritance {
                DrawingSheetInheritance::ProjectDefault => settings
                    .default_format
                    .with_target_sheet_title_fields(sheet.page_format()),
                DrawingSheetInheritance::Explicit | DrawingSheetInheritance::UserDefault => {
                    sheet.page_format().clone()
                }
            };
            let centre = title_block_centre(&format);
            for (field, entry) in &format.title_block.fields {
                // The shared values and the release record are the
                // project's, read above.
                if field.is_project_owned()
                    || matches!(
                        field,
                        DrawingSheetTitleFieldId::Revision | DrawingSheetTitleFieldId::Date
                    )
                    || field.policy().value_authority
                        != DrawingSheetTitleFieldValueAuthority::Authored
                    || !entry.visible
                {
                    continue;
                }
                found.read(&entry.value, covers, || Place {
                    label: format!(
                        "Title block \u{00b7} {} \u{00b7} {}",
                        field.display_label(),
                        sheet.name()
                    ),
                    cell_view: cell_view.clone(),
                    location: DrcLocation::Point {
                        x: f64::from(centre.x),
                        y: f64::from(centre.y),
                    },
                    at: centre,
                    component: None,
                    sheet: Some(sheet.id()),
                });
            }
        }
    }
}

/// The middle of the title block a sheet of `format` draws, in drawing
/// units, or the origin where it draws none.
fn title_block_centre(format: &SchematicSheetFormat) -> Point {
    DrawingSheetGeometry::from_format(format)
        .title_block
        .map_or(Point::new(0, 0), |block| {
            Point::new(
                ((block.min_x + block.max_x) * 0.5).round() as i32,
                ((block.min_y + block.max_y) * 0.5).round() as i32,
            )
        })
}

/// The text one drawing letters: each part's name and value, each note and
/// each net label.
fn drawing(
    model: &Model,
    cell_view: &CellViewRef,
    document: &SchematicDocument,
    covers: &dyn Fn(char) -> Coverage,
    found: &mut Found,
) {
    let key = cell_view.key();
    let sheet_of = |id: u64| {
        model
            .workspace
            .content
            .design_management
            .sheet_for_object_or_active(&key, id)
    };
    for component in &document.components {
        let place = |what: &str| Place {
            label: format!("{} \u{00b7} {what}", component.name),
            cell_view: cell_view.clone(),
            location: DrcLocation::Component {
                id: component.id,
                name: component.name.clone(),
            },
            at: component.pos,
            component: Some(component.id),
            sheet: sheet_of(component.id),
        };
        found.read(&component.name, covers, || place("name"));
        found.read(&component.value, covers, || place("value"));
    }
    for note in &document.design_notes {
        found.read(&note.text, covers, || Place {
            label: format!("Note \u{00b7} {}", first_line(&note.text)),
            cell_view: cell_view.clone(),
            location: DrcLocation::Point {
                x: f64::from(note.pos.x),
                y: f64::from(note.pos.y),
            },
            at: note.pos,
            component: None,
            sheet: sheet_of(note.id),
        });
    }
    for label in &document.net_labels {
        found.read(&label.name, covers, || Place {
            label: format!("Net label \u{00b7} {}", label.name),
            cell_view: cell_view.clone(),
            location: DrcLocation::NetLabel {
                name: label.name.clone(),
            },
            at: label.pos,
            component: None,
            sheet: sheet_of(label.id),
        });
    }
}

/// A note's first line, cut to a length a place line can show.
fn first_line(text: &str) -> String {
    const SHOWN: usize = 32;
    let line = text.lines().next().unwrap_or_default();
    if line.chars().count() > SHOWN {
        let mut shown = line.chars().take(SHOWN - 1).collect::<String>();
        shown.push('\u{2026}');
        shown
    } else {
        line.to_owned()
    }
}

/// A character as Unicode names it: `U+0378`.
fn code_point(character: char) -> String {
    format!("U+{:04X}", u32::from(character))
}

fn lettering_records(model: &Model, found: &Found) -> Vec<DiagnosticRecord> {
    let project = model.workspace.content.project.id();
    let revision = model.workspace.content.project.revision().get();
    let file = super::rule_deck::project_file_label(model);
    let document: Arc<str> = Arc::from(file.as_str());
    let keys = found
        .missing
        .keys()
        .map(|character| {
            FindingKey::new(
                LETTERING_PRODUCER,
                UNCOVERED_CHARACTER,
                Arc::clone(&document),
                code_point(*character),
            )
        })
        .collect::<Vec<_>>();
    let canonicals = mint_canonical(&keys, revision, "lettering");
    found
        .missing
        .iter()
        .zip(keys)
        .zip(canonicals)
        .map(|(((character, places), key), canonical)| {
            let code = code_point(*character);
            DiagnosticRecord::new(
                canonical,
                DiagnosticLevel::Warning,
                DiagnosticSubject::new(project, code.clone()),
                format!("No typeface here has {code}, so it shows and prints as \u{25fb}."),
            )
            .keyed(key)
            .with_details("Install a typeface that has it, or change the text it stands in.")
            .with_route(places.first().map(Place::route))
            .with_related_places(
                places
                    .iter()
                    .map(|place| RelatedPlace {
                        label: place.label.clone(),
                        cell_view: place.cell_view.clone(),
                        location: place.location.clone(),
                        route: Some(place.route()),
                    })
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ComponentType;
    use crate::state::pdk_config::PdkConfig;

    fn model() -> Model {
        Model::session_start(
            PdkConfig::default(),
            None,
            crate::workbench::workspace::restore_model_library_manager,
        )
    }

    const FACTS: SlotFacts = SlotFacts {
        preflight_report_retained: false,
        review_markers:
            rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility::OpenAndAssigned,
    };

    /// One record per character no face has, listing each place it stands;
    /// none for a character a face letters, and none yet for one an
    /// installed face may still have, which is asked about instead.
    #[test]
    fn a_character_no_face_has_is_named_once_with_every_place_it_stands() {
        let mut model = model();
        let value = "\u{3a9} \u{2300} \u{76f4} \u{378}";
        for x in [0, 400] {
            let part = model
                .schematic
                .add_component(ComponentType::Resistor, Point::new(x, 0));
            if let Some(part) = model
                .schematic
                .document_mut_for_test()
                .components
                .iter_mut()
                .find(|component| component.id == part)
            {
                part.value = value.to_owned();
            }
        }
        let covers = |character: char| match character {
            '\u{378}' => Coverage::Missing,
            '\u{76f4}' => Coverage::Unsettled,
            _ => coverage::coverage(character),
        };

        let found = scan(&model, &covers);
        assert_eq!(
            found.missing.keys().copied().collect::<Vec<_>>(),
            ['\u{378}']
        );
        assert_eq!(
            found.unsettled.iter().copied().collect::<Vec<_>>(),
            ['\u{76f4}']
        );

        model.publish_lettering(FACTS, &covers);
        let batch = model
            .diagnostics
            .batch(&DiagnosticInputs::Lettering)
            .expect("the lettering is published");
        let [record] = batch.records.as_slice() else {
            panic!("one record per character: {:?}", batch.records);
        };
        assert_eq!(record.level, DiagnosticLevel::Warning);
        assert_eq!(record.subject.object, "U+0378");
        assert_eq!(
            record.message,
            "No typeface here has U+0378, so it shows and prints as \u{25fb}."
        );
        assert_eq!(
            record
                .related_places
                .iter()
                .map(|place| place.label.as_str())
                .collect::<Vec<_>>(),
            ["R1 \u{00b7} value", "R2 \u{00b7} value"]
        );
        assert!(record.route.is_some());
    }

    /// Decomposed text from a file is read as it letters.
    #[test]
    fn decomposed_text_is_read_composed() {
        let mut found = Found::default();
        let covers = |character: char| {
            if character == '\u{e9}' {
                Coverage::Missing
            } else {
                Coverage::Lettered
            }
        };
        found.read("Cafe\u{301}", &covers, || Place {
            label: String::new(),
            cell_view: CellViewRef::default_top(),
            location: DrcLocation::Global,
            at: Point::new(0, 0),
            component: None,
            sheet: None,
        });
        assert_eq!(
            found.missing.keys().copied().collect::<Vec<_>>(),
            ['\u{e9}']
        );
    }
}
