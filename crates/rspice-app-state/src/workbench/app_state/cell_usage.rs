//! Where a library cell is placed: the schematic instances that bind it and
//! the layouts that place it.
//!
//! The library's Where used report, the inspector's consumer count, the
//! deletion reviews and the Verilog-A navigator's Used in section all ask
//! this one question, so it is answered once. Every schematic of the project
//! is in the workspace's buffers; the one open in the editor is read from
//! the editor instead, because its buffer is only brought up to date when the
//! view is left. Every layout is in the workspace's physical-layout catalog.
//!
//! The panels that show the answer ask it every frame, and the answer walks
//! every component of every drawing and every instance of every layout. So
//! each answer is remembered with the [stamps](rspice_design::stamped) of the
//! drawings and the layouts it was read from, and is read again only once one
//! of them has moved.

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
    /// Every layout that places it, by layout.
    layouts: Vec<LayoutPlacementUse>,
    /// The schematics and then the layouts it is placed in, each once, as
    /// paths.
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
/// with its reference, every buffer by its key, and the layouts.
#[derive(Default)]
struct Drawings {
    live: Option<(CellViewRef, u64)>,
    buffers: HashMap<String, u64>,
    layouts: u64,
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
            layouts: model.workspace.content.physical_layout_stamp(),
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
        live && self.layouts == content.physical_layout_stamp()
            && self.buffers.len() == content.schematic_buffers.len()
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

/// One layout that places a library cell, and how many times.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutPlacementUse {
    /// The layout view the cell is placed in.
    pub layout: CellViewRef,
    /// How many of that layout's instances place it.
    pub placements: usize,
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

    /// Every layout in the project that places `library`/`cell`, or with
    /// `view` that view of it alone, ordered by layout.
    pub fn cell_layout_placements(
        &self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) -> Vec<LayoutPlacementUse> {
        self.cell_uses(library, cell, view).layouts.clone()
    }

    /// Every placement of a cell of `library` made from outside it: the
    /// schematic instances with the cell each places, by schematic and then
    /// by instance name, and the layouts, by layout. A library's own cells
    /// placing each other is its own hierarchy and is not counted.
    pub fn library_external_placements(
        &self,
        library: &str,
    ) -> (Vec<(String, CellInstanceUse)>, Vec<LayoutPlacementUse>) {
        let Some(found) = self.library_manager.get_library(library) else {
            return (Vec::new(), Vec::new());
        };
        let outside = |reference: &CellViewRef| !reference.library.eq_ignore_ascii_case(library);
        let mut instances = Vec::new();
        let mut layouts = std::collections::BTreeMap::<String, LayoutPlacementUse>::new();
        for cell in found.cells.values() {
            let uses = self.cell_uses(library, &cell.name, None);
            instances.extend(
                uses.instances
                    .iter()
                    .filter(|usage| outside(&usage.schematic))
                    .map(|usage| (cell.name.clone(), usage.clone())),
            );
            for usage in uses.layouts.iter().filter(|usage| outside(&usage.layout)) {
                layouts
                    .entry(usage.layout.key().to_ascii_lowercase())
                    .and_modify(|merged| merged.placements += usage.placements)
                    .or_insert_with(|| usage.clone());
            }
        }
        instances.sort_by_key(|(_, usage)| {
            (
                usage.schematic.key().to_ascii_lowercase(),
                usage.instance.to_ascii_lowercase(),
            )
        });
        (instances, layouts.into_values().collect())
    }

    /// The schematics and then the layouts that place `reference`, each
    /// once, as the paths the panels print.
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
        let mut layouts = content
            .physical_layout_documents()
            .values()
            .filter_map(|document| {
                let placements = document
                    .instances()
                    .values()
                    .filter(|instance| masters(&instance.master, library, cell, view))
                    .count();
                (placements > 0).then(|| LayoutPlacementUse {
                    layout: document.owner().clone(),
                    placements,
                })
            })
            .collect::<Vec<_>>();
        layouts.sort_by_key(|usage| usage.layout.key().to_ascii_lowercase());
        let mut consumers = uses
            .iter()
            .map(|usage| usage.schematic.display_path())
            .collect::<Vec<_>>();
        consumers.dedup();
        consumers.extend(layouts.iter().map(|usage| usage.layout.display_path()));
        CellUses {
            instances: uses,
            layouts,
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
            hierarchy_pruned,
        }
    }
}

/// What taking one cell out of the workspace left undecided: the focus and
/// the occurrence trail both name cell views, and either can have named the
/// cell that just went. The project top cannot: it is never deleted.
pub struct CellRemovalScope {
    pub active_removed: bool,
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

/// Whether a layout instance of `master` places `library`/`cell`, or its
/// `view` alone, by the comparison [`places`] makes.
fn masters(master: &CellViewRef, library: &str, cell: &str, view: Option<&str>) -> bool {
    master.library.eq_ignore_ascii_case(library)
        && master.cell.eq_ignore_ascii_case(cell)
        && view.is_none_or(|view| master.view.eq_ignore_ascii_case(view))
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
