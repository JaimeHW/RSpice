//! Where a library cell is placed: the schematic instances that bind it.
//!
//! The library's Where used report, the inspector's consumer count and the
//! Verilog-A navigator's Used in section all ask this one question, so it is
//! answered once. Every schematic of the project is in the workspace's
//! buffers; the one open in the editor is read from the editor instead,
//! because its buffer is only brought up to date when the view is left.
//!
//! The panels that show the answer ask it every frame, and the answer walks
//! every component of every drawing. So each answer is remembered with the
//! [stamps](rspice_design::stamped) of the drawings it was read from, and is
//! read again only once one of them has moved.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use rspice_design::schematic::document::SchematicDocument;
use rspice_project::ProjectWorkspace;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::model::Model;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Times this thread walked the project's drawings for a placement.
    pub static CELL_USAGE_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Where one cell, or one view of it, is placed.
#[derive(Debug, Default)]
struct CellUses {
    /// Every placed instance, by schematic and then by instance name.
    instances: Vec<CellInstanceUse>,
    /// The schematics those instances are placed in, each once, as paths.
    consumers: Arc<[String]>,
}

/// One question: a library, a cell and, for one view of it, the view.
type Query = (String, String, Option<String>);

/// The answers given since the drawings last moved, and the stamps of the
/// drawings they were read from.
#[derive(Default)]
struct UsageMemo {
    drawings: Drawings,
    answers: Vec<(Query, Arc<CellUses>)>,
}

thread_local! {
    static MEMO: RefCell<UsageMemo> = RefCell::default();
}

/// Every drawing an answer reads, by its stamp: the one open in the editor
/// with its reference, and every buffer by its key.
#[derive(Default)]
struct Drawings {
    live: Option<(CellViewRef, u64)>,
    buffers: HashMap<String, u64>,
}

impl Drawings {
    fn of(model: &Model) -> Self {
        Self {
            live: live_drawing(&model.workspace.content)
                .map(|reference| (reference.clone(), model.schematic.content_stamp())),
            buffers: model
                .workspace
                .content
                .schematic_buffers
                .iter()
                .map(|(key, schematic)| (key.clone(), schematic.content_stamp()))
                .collect(),
        }
    }

    /// Whether every drawing still stands as it was read, without copying
    /// a key.
    fn stand(&self, model: &Model) -> bool {
        let content = &model.workspace.content;
        let live = match (&self.live, live_drawing(content)) {
            (None, None) => true,
            (Some((reference, stamp)), Some(active)) => {
                reference == active && *stamp == model.schematic.content_stamp()
            }
            _ => false,
        };
        live && self.buffers.len() == content.schematic_buffers.len()
            && content
                .schematic_buffers
                .iter()
                .all(|(key, schematic)| self.buffers.get(key) == Some(&schematic.content_stamp()))
    }
}

/// The drawing the editor holds, when it holds a schematic: its reference is
/// the active view's.
fn live_drawing(content: &ProjectWorkspace) -> Option<&CellViewRef> {
    matches!(
        content.active_view_type(),
        Some(ViewType::Schematic | ViewType::Testbench)
    )
    .then_some(&content.active_view)
}

/// One placed instance of a library cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellInstanceUse {
    /// The schematic the instance is placed in.
    pub schematic: CellViewRef,
    /// The instance's name in that schematic, `I3`.
    pub instance: String,
    /// The instance's component identity in that schematic.
    pub component_id: u64,
}

impl Model {
    /// Every instance in the project that binds `library`/`cell`, ordered by
    /// schematic and then by instance name.
    ///
    /// With `view` it counts only instances bound to that view. Without one it
    /// counts every view of the cell: an instance placed from a cell's symbol
    /// is still an instance of the cell its Verilog-A view describes.
    pub fn cell_instances(
        &self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) -> Vec<CellInstanceUse> {
        self.cell_uses(library, cell, view).instances.clone()
    }

    /// The schematics that place `reference`, each once, as the paths the
    /// panels print.
    pub fn cell_view_consumers(&self, reference: &CellViewRef) -> Arc<[String]> {
        Arc::clone(
            &self
                .cell_uses(&reference.library, &reference.cell, Some(&reference.view))
                .consumers,
        )
    }

    /// The answer for one question: remembered while every drawing it was
    /// read from stands, read afresh otherwise.
    fn cell_uses(&self, library: &str, cell: &str, view: Option<&str>) -> Arc<CellUses> {
        MEMO.with(|memo| {
            let mut memo = memo.borrow_mut();
            if !memo.drawings.stand(self) {
                *memo = UsageMemo {
                    drawings: Drawings::of(self),
                    answers: Vec::new(),
                };
            }
            let remembered =
                memo.answers
                    .iter()
                    .find(|((asked_library, asked_cell, asked_view), _)| {
                        asked_library == library
                            && asked_cell == cell
                            && asked_view.as_deref() == view
                    });
            if let Some((_, uses)) = remembered {
                return Arc::clone(uses);
            }
            let uses = Arc::new(self.scan_cell_uses(library, cell, view));
            memo.answers.push((
                (library.to_owned(), cell.to_owned(), view.map(str::to_owned)),
                Arc::clone(&uses),
            ));
            uses
        })
    }

    fn scan_cell_uses(&self, library: &str, cell: &str, view: Option<&str>) -> CellUses {
        #[cfg(any(test, feature = "test-support"))]
        CELL_USAGE_SCANS.with(|scans| scans.set(scans.get() + 1));
        let content = &self.workspace.content;
        let live = live_drawing(content);
        let mut uses = Vec::new();
        if let Some(reference) = live {
            collect(
                self.schematic.as_ref(),
                reference,
                (library, cell, view),
                &mut uses,
            );
        }
        for (key, schematic) in &content.schematic_buffers {
            let Some(reference) = CellViewRef::from_key(key) else {
                continue;
            };
            if live.is_some_and(|live| live.key().eq_ignore_ascii_case(key)) {
                continue;
            }
            collect(
                schematic.as_ref(),
                &reference,
                (library, cell, view),
                &mut uses,
            );
        }
        uses.sort_by(|left, right| {
            (
                left.schematic.key().to_ascii_lowercase(),
                left.instance.to_ascii_lowercase(),
            )
                .cmp(&(
                    right.schematic.key().to_ascii_lowercase(),
                    right.instance.to_ascii_lowercase(),
                ))
        });
        let mut consumers = uses
            .iter()
            .map(|usage| usage.schematic.display_path())
            .collect::<Vec<_>>();
        consumers.dedup();
        CellUses {
            instances: uses,
            consumers: consumers.into(),
        }
    }

    /// Take a removed cell out of every workspace collection that names it,
    /// and say what that left undecided.
    pub fn drop_workspace_references_to_cell(
        &mut self,
        library: &str,
        cell: &str,
    ) -> CellRemovalScope {
        let active_removed = self.workspace.content.active_view.library == library
            && self.workspace.content.active_view.cell == cell;
        let project_root_removed = self.workspace.content.project.root_library == library
            && self.workspace.content.project.top_cell == cell;
        let prefix = format!("{library}/{cell}/");
        self.workspace
            .content
            .schematic_buffers
            .retain(|key, _| !key.starts_with(&prefix));
        self.workspace
            .session
            .schematic_sessions
            .retain(|key, _| !key.starts_with(&prefix));
        let orphaned_layouts = self
            .workspace
            .content
            .physical_layout_documents()
            .values()
            .filter(|document| document.owner().library == library && document.owner().cell == cell)
            .map(|document| document.owner().clone())
            .collect::<Vec<_>>();
        for owner in orphaned_layouts {
            self.workspace
                .content
                .remove_physical_layout_document(&owner);
        }
        self.workspace
            .content
            .open_views
            .retain(|open| open.reference.library != library || open.reference.cell != cell);
        let libraries = &self.library_manager;
        for schematic in self.workspace.content.schematic_buffers.values_mut() {
            schematic.revalidate_instance_bindings(libraries.catalog());
        }
        let hierarchy_pruned = self
            .workspace
            .content
            .retain_valid_occurrences(|reference| {
                reference.library != library || reference.cell != cell
            });
        CellRemovalScope {
            active_removed,
            project_root_removed,
            hierarchy_pruned,
        }
    }
}

/// What taking one cell out of the workspace left undecided: the focus, the
/// project root, and the occurrence trail all name cell views, and any of the
/// three can have named the cell that just went.
pub struct CellRemovalScope {
    pub active_removed: bool,
    pub project_root_removed: bool,
    pub hierarchy_pruned: bool,
}

/// Whether `component` is a placement of `library`/`cell`, or of its `view`
/// alone: the one comparison every count and removal of placements makes.
/// Names are compared ignoring ASCII case, as the catalog keeps them unique
/// that way.
pub fn places(
    component: &crate::state::Component,
    library: &str,
    cell: &str,
    view: Option<&str>,
) -> bool {
    component.library_cell.as_ref().is_some_and(|binding| {
        binding.library.eq_ignore_ascii_case(library)
            && binding.cell.eq_ignore_ascii_case(cell)
            && view.is_none_or(|view| binding.view.eq_ignore_ascii_case(view))
    })
}

fn collect(
    document: &SchematicDocument,
    schematic: &CellViewRef,
    (library, cell, view): (&str, &str, Option<&str>),
    uses: &mut Vec<CellInstanceUse>,
) {
    for component in &document.components {
        if places(component, library, cell, view) {
            uses.push(CellInstanceUse {
                schematic: schematic.clone(),
                instance: component.name.clone(),
                component_id: component.id,
            });
        }
    }
}
