//! One dirty answer per working state.
//!
//! The toolbar, the window title, the Project pages and command enablement
//! ask whether the project holds unsaved work every frame. Each reads the
//! comparison made for the working project's stamp, and only a stamp that
//! moved compares again. A comparison digests again only the documents
//! whose own stamps moved, from the working project as its owners hold it,
//! and compares only those with the accepted baseline: an edit costs what
//! it changed, not what the project holds. A decision that can drop work —
//! save, close, quit, replacing the project — reads [`fresh`], which
//! captures the whole project and never trusts what is held.
//!
//! While a canvas gesture edits the drawing, the answer held before it
//! started stands, with the drawing's document read as unsaved from the
//! first move; the edit is compared once, at the drop.
//!
//! The generated netlist's input digest, which the Netlist page reads every
//! frame, is held for the working stamp the same way.
//!
//! In debug builds and tests every held answer is checked: the project is
//! captured again and a document whose identity moved under an unchanged
//! stamp panics, naming it. A subject the stamp misses fails the suite; it
//! cannot ship as lost work.

use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

use rspice_app_state::state::ViewType;
use rspice_app_state::workbench::app_state::model::InPlaceLayout;
use rspice_project::SnapshotContent;
use rspice_project::lifecycle::{DirtyBaseline, DirtyComparison};
use rspice_project::registry::{
    DocumentFingerprints, ProjectDocumentId, WorkingDocuments, WorkingSource,
};
use rspice_vocabulary::product::ContentDigest;

use super::{LifecycleSource, LifecycleView, ProjectLifecycleError};

/// Everything a capture of the working project reads: each subject by the
/// stamp its owner issues on every mutable access, the two with no stamp of
/// their own by content. Equal stamps, equal working content.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkingStamp {
    /// The workspace content: every buffered schematic, the project
    /// descriptor, configurations, plan payloads, sources, layouts, reports,
    /// result documents, hardcopy setups, open views and their dirty marks.
    workspace: u64,
    /// The schematic in the editor, which a capture takes in place of its
    /// buffered copy.
    active_schematic: u64,
    /// The design library catalog, then the libraries it references.
    libraries: (u64, u64),
    /// The persisted simulation plan.
    simulation_plan: u64,
    /// The model catalog, its resolution records and its validation receipt.
    model_catalog: [u64; 3],
    /// The retained results as the project persists them.
    results: u64,
    /// The Results presentation a save keeps, which its feature owns and
    /// does not stamp: by its canonical encoding, as the fingerprint reads
    /// it, so a marker moved from 0 to -0 is an edit.
    presentation: Result<ContentDigest, String>,
    /// The symbol edited in place, which a save writes behind its sheet.
    in_place_layout: Option<InPlaceLayout>,
}

impl WorkingStamp {
    pub(super) fn of(state: &LifecycleView<'_>) -> Self {
        let model = state.model;
        Self {
            workspace: model.workspace.content.stamp(),
            active_schematic: model.schematic.editor_ref().design.content_stamp(),
            libraries: (
                model.library_manager.content_stamp(),
                model.library_manager.references_stamp(),
            ),
            simulation_plan: model.sim_setup.setup.content_stamp(),
            model_catalog: model.model_library_manager.content_stamp(),
            results: state
                .platform
                .project_lifecycle
                .result_cache
                .stamp(&model.simulation),
            presentation: rspice_project::registry::digest(
                &state.presentation.project_presentation(&model.simulation),
            ),
            in_place_layout: model.symbol_in_place_layout.clone(),
        }
    }

    /// Whether `self` and `other` differ in nothing but the drawing in the
    /// editor.
    fn differs_only_in_the_drawing(&self, other: &Self) -> bool {
        self.workspace == other.workspace
            && self.libraries == other.libraries
            && self.simulation_plan == other.simulation_plan
            && self.model_catalog == other.model_catalog
            && self.results == other.results
            && self.presentation == other.presentation
            && self.in_place_layout == other.in_place_layout
    }
}

/// The last comparison, with the working stamp and baseline it was made
/// for, and the working documents' digests it was made from.
#[derive(Debug, Clone, Default)]
pub struct DirtyAnswer {
    held: RefCell<Option<Answer>>,
    documents: RefCell<WorkingDocuments>,
    /// Whether the working project, as last captured, was one a save could
    /// write, with the stamp it was captured under.
    validity: RefCell<Option<(WorkingStamp, Result<(), String>)>>,
}

#[derive(Debug, Clone)]
struct Answer {
    stamp: WorkingStamp,
    baseline: DirtyBaseline,
    compared: Rc<Compared>,
}

/// One comparison of the working project.
#[derive(Debug)]
struct Compared {
    /// The working project's fingerprints, taken from the held digests when
    /// first read; why the working project could not be digested.
    working: Result<OnceCell<DocumentFingerprints>, String>,
    comparison: DirtyComparison,
}

impl DirtyAnswer {
    fn held(&self, stamp: &WorkingStamp, baseline: &DirtyBaseline) -> Option<Rc<Compared>> {
        self.held
            .borrow()
            .as_ref()
            .filter(|held| held.stamp == *stamp && held.baseline == *baseline)
            .map(|held| Rc::clone(&held.compared))
    }

    fn hold(&self, answer: Answer) {
        *self.held.borrow_mut() = Some(answer);
    }

    /// While a gesture edits the drawing, the answer held from before it
    /// moved, with `editing` read as unsaved; `None` when anything else
    /// moved since.
    fn held_through_gesture(
        &self,
        stamp: &WorkingStamp,
        baseline: &DirtyBaseline,
        editing: ProjectDocumentId,
    ) -> Option<Rc<Compared>> {
        let mut held = self.held.borrow_mut();
        let answer = held.as_mut().filter(|held| {
            held.baseline == *baseline && held.stamp.differs_only_in_the_drawing(stamp)
        })?;
        if answer.compared.comparison.edit_in_progress() != Some(&editing) {
            answer.compared = Rc::new(Compared {
                working: Err("a gesture is editing the drawing".to_owned()),
                comparison: answer
                    .compared
                    .comparison
                    .clone()
                    .with_edit_in_progress(editing),
            });
        }
        answer.stamp = stamp.clone();
        Some(Rc::clone(&answer.compared))
    }
}

/// `answer`, read from the comparison held for the working state as it
/// stands, made once per stamp and held through a canvas gesture.
pub(super) fn read<R>(state: &LifecycleView<'_>, answer: impl FnOnce(&DirtyComparison) -> R) -> R {
    let authority = &state.platform.project_lifecycle.authority;
    if !authority.compares_working_content() {
        return answer(&authority.compare_working(None));
    }
    answer(&held_or_compared(state, true).comparison)
}

/// `read` the working project's fingerprints as held for its stamp, taken
/// once per stamp; the error when it could not be digested.
pub(super) fn working<R>(
    state: &LifecycleView<'_>,
    read: impl FnOnce(Result<&DocumentFingerprints, &str>) -> R,
) -> R {
    let compared = held_or_compared(state, false);
    match &compared.working {
        Ok(fingerprints) => {
            let documents = state
                .platform
                .project_lifecycle
                .dirty_answer
                .documents
                .borrow();
            read(Ok(fingerprints.get_or_init(|| documents.fingerprints())))
        }
        Err(error) => read(Err(error)),
    }
}

/// The comparison held for the working state, or one made now. Through a
/// gesture, the one held before it, with the drawing read as edited.
fn held_or_compared(state: &LifecycleView<'_>, through_gesture: bool) -> Rc<Compared> {
    let stamp = WorkingStamp::of(state);
    let baseline = state.platform.project_lifecycle.authority.dirty_baseline();
    let answer = &state.platform.project_lifecycle.dirty_answer;
    if let Some(held) = answer
        .held(&stamp, &baseline)
        .filter(|held| through_gesture || held.comparison.edit_in_progress().is_none())
    {
        #[cfg(debug_assertions)]
        if !audit_suspended() && held.comparison.edit_in_progress().is_none() {
            audit(
                &held,
                answer,
                &rspice_project::registry::unobserved(|| fingerprint(state)),
            );
        }
        return held;
    }
    let workspace = &state.model.workspace.content;
    if through_gesture
        && state.model.canvas_gesture_in_progress()
        && matches!(
            workspace.active_view_type(),
            Some(ViewType::Schematic | ViewType::Testbench)
        )
        && let Some(held) = answer.held_through_gesture(
            &stamp,
            &baseline,
            ProjectDocumentId::CellView(workspace.active_view.clone()),
        )
    {
        return held;
    }
    compare(state, stamp, baseline)
}

/// `answer`, from a capture made now whatever is held. For every decision
/// that drops or keeps the user's work. A held answer the capture
/// contradicts is replaced by the capture's.
pub(super) fn fresh<R>(state: &LifecycleView<'_>, answer: impl FnOnce(&DirtyComparison) -> R) -> R {
    let authority = &state.platform.project_lifecycle.authority;
    if !authority.compares_working_content() {
        return answer(&authority.compare_working(None));
    }
    let stamp = WorkingStamp::of(state);
    let baseline = authority.dirty_baseline();
    let working = fingerprint(state);
    let comparison = authority.compare_working(working.as_ref().ok());
    let dirty_answer = &state.platform.project_lifecycle.dirty_answer;
    let Some(held) = dirty_answer
        .held(&stamp, &baseline)
        .filter(|held| held.comparison.edit_in_progress().is_none())
    else {
        return answer(&comparison);
    };
    #[cfg(debug_assertions)]
    audit(&held, dirty_answer, &working);
    if agrees(&held, dirty_answer, &working) {
        return answer(&comparison);
    }
    // The digests held are not the ones the capture took: the next
    // comparison digests and compares every document.
    dirty_answer.documents.borrow_mut().clear();
    let compared = Rc::new(Compared {
        working: working.map(OnceCell::from),
        comparison,
    });
    dirty_answer.hold(Answer {
        stamp,
        baseline,
        compared: Rc::clone(&compared),
    });
    answer(&compared.comparison)
}

/// Whether the working project `held` was compared from is the one
/// captured as `current`.
fn agrees(
    held: &Compared,
    answer: &DirtyAnswer,
    current: &Result<DocumentFingerprints, String>,
) -> bool {
    let documents = answer.documents.borrow();
    let working = held
        .working
        .as_ref()
        .map(|working| working.get_or_init(|| documents.fingerprints()));
    match (working, current) {
        (Ok(held), Ok(current)) => held.content_digest() == current.content_digest(),
        (Err(_), Err(_)) => true,
        _ => false,
    }
}

fn fingerprint(state: &LifecycleView<'_>) -> Result<DocumentFingerprints, String> {
    super::capture_snapshot(state, SnapshotContent::Current)
        .and_then(|snapshot| {
            state
                .platform
                .project_lifecycle
                .authority
                .working_fingerprints(&snapshot.file)
        })
        .map_err(|error| error.to_string())
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Comparisons of the working project made for a stamp that moved.
    pub static DIRTY_RECOMPUTES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Compare the working project, digesting again only the documents whose
/// stamps moved and comparing only those with the baseline, and hold the
/// comparison for `stamp`, taken before anything was read.
fn compare(
    state: &LifecycleView<'_>,
    stamp: WorkingStamp,
    baseline: DirtyBaseline,
) -> Rc<Compared> {
    #[cfg(any(test, feature = "test-support"))]
    DIRTY_RECOMPUTES.with(|count| count.set(count.get() + 1));
    let lifecycle = &state.platform.project_lifecycle;
    let dirty_answer = &lifecycle.dirty_answer;
    let previous = dirty_answer
        .held
        .borrow_mut()
        .take()
        .filter(|held| held.baseline == baseline);
    let mut documents = dirty_answer.documents.borrow_mut();
    // Over no held digests, every document is compared.
    let previous = previous.filter(|_| !documents.documents().is_empty());
    let changed = match validity(state, &stamp).and(stamp.presentation.clone()) {
        Ok(presentation) => documents.update(
            &working_source(state, &stamp, presentation),
            || {
                crate::io::capture_execution_context(
                    &state.model.sim_setup,
                    &state.model.model_library_manager,
                )
            },
            || {
                let workspace = &state.model.workspace.content;
                lifecycle.authority.result_fingerprints().digest(
                    &lifecycle.result_cache.capture(&state.model.simulation),
                    &workspace.report_documents,
                    &workspace.visualization_documents,
                    &state
                        .presentation
                        .project_presentation(&state.model.simulation),
                )
            },
        ),
        Err(error) => Err(error),
    };
    let comparison = match &changed {
        Ok(changed) => lifecycle.authority.compare_working_documents(
            Some(&documents),
            previous.map(|held| (into_comparison(held.compared), changed.as_slice())),
        ),
        Err(_) => lifecycle.authority.compare_working_documents(None, None),
    };
    drop(documents);
    let compared = Rc::new(Compared {
        working: changed.map(|_| OnceCell::new()),
        comparison,
    });
    dirty_answer.hold(Answer {
        stamp,
        baseline,
        compared: Rc::clone(&compared),
    });
    compared
}

/// Whether the working project is one a save could write, as a capture
/// validates it. A capture answers when anything but the editor's drawing
/// moved since the last one; when only the drawing moved, under a valid
/// project, only the drawing's own record is checked again.
fn validity(state: &LifecycleView<'_>, stamp: &WorkingStamp) -> Result<(), String> {
    let held = &state.platform.project_lifecycle.dirty_answer.validity;
    if held
        .borrow()
        .as_ref()
        .is_some_and(|(then, valid)| valid.is_ok() && then.differs_only_in_the_drawing(stamp))
    {
        let workspace = &state.model.workspace.content;
        if !matches!(
            workspace.active_view_type(),
            Some(ViewType::Schematic | ViewType::Testbench)
        ) {
            return Ok(());
        }
        return state
            .model
            .schematic
            .editor_ref()
            .design
            .document()
            .validated_revisions
            .validate()
            .map_err(|error| {
                format!(
                    "workspace schematic buffer '{}' has invalid validated revision history:                      {error}",
                    workspace.active_key()
                )
            });
    }
    let valid = super::capture_snapshot(state, SnapshotContent::Current)
        .map(drop)
        .map_err(|error| error.to_string());
    *held.borrow_mut() = Some((stamp.clone(), valid.clone()));
    valid
}

/// The working project as its owners hold it, under `stamp`.
fn working_source<'a>(
    state: &LifecycleView<'a>,
    stamp: &WorkingStamp,
    presentation: ContentDigest,
) -> WorkingSource<'a> {
    let model = state.model;
    WorkingSource {
        workspace: &model.workspace.content,
        workspace_stamp: stamp.workspace,
        active_schematic: model.schematic.editor_ref().design,
        libraries: &model.library_manager,
        execution_stamp: [
            stamp.simulation_plan,
            stamp.model_catalog[0],
            stamp.model_catalog[1],
            stamp.model_catalog[2],
        ],
        results_stamp: (stamp.results, presentation),
    }
}

/// The comparison `compared` holds, taken without a copy when nothing else
/// holds it.
fn into_comparison(compared: Rc<Compared>) -> DirtyComparison {
    Rc::try_unwrap(compared).map_or_else(
        |shared| shared.comparison.clone(),
        |compared| compared.comparison,
    )
}

/// The generated netlist's input digest held for the working stamp it was
/// taken for. Only a digest is held: a capture that failed is taken again.
#[derive(Debug, Clone, Default)]
pub struct NetlistInput(RefCell<Option<(WorkingStamp, ContentDigest)>>);

/// The generated netlist's input digest for the working state as it stands,
/// taken once per stamp.
pub(super) fn netlist_input(
    state: &LifecycleView<'_>,
) -> Result<ContentDigest, ProjectLifecycleError> {
    let stamp = WorkingStamp::of(state);
    let held = &state.platform.project_lifecycle.netlist_input.0;
    let digest = held
        .borrow()
        .as_ref()
        .filter(|(held, _)| *held == stamp)
        .map(|(_, digest)| *digest);
    if let Some(digest) = digest {
        #[cfg(debug_assertions)]
        if !audit_suspended() {
            audit_netlist_input(
                digest,
                &rspice_project::registry::unobserved(|| digest_netlist_input(state)),
            );
        }
        return Ok(digest);
    }
    let digest = digest_netlist_input(state)?;
    *held.borrow_mut() = Some((stamp, digest));
    Ok(digest)
}

fn digest_netlist_input(state: &LifecycleView<'_>) -> Result<ContentDigest, ProjectLifecycleError> {
    super::capture_snapshot(state, SnapshotContent::Current)?
        .file
        .generated_netlist_input_digest()
}

/// Run `mark`, which writes only the runtime unsaved marks — a tab's dot, a
/// view's modified flag, the per-subject dirty flags — and keep the
/// comparison held for the working state before it as the one for the
/// state after: a capture clears every mark, so none moves what it holds.
/// Debug builds check the claim on the next read.
pub(super) fn across_marks<R>(
    state: &mut super::LifecycleParts<'_>,
    mark: impl FnOnce(&mut super::LifecycleParts<'_>) -> R,
) -> R {
    let before = {
        let view = state.lifecycle_view();
        view.platform.project_lifecycle.dirty_answer.held(
            &WorkingStamp::of(&view),
            &view.platform.project_lifecycle.authority.dirty_baseline(),
        )
    };
    let marked = mark(state);
    if let Some(compared) = before {
        let view = state.lifecycle_view();
        let lifecycle = &view.platform.project_lifecycle;
        lifecycle.dirty_answer.hold(Answer {
            stamp: WorkingStamp::of(&view),
            baseline: lifecycle.authority.dirty_baseline(),
            compared,
        });
    }
    marked
}

#[cfg(debug_assertions)]
thread_local! {
    /// Set while a test holds an answer it made stale on purpose.
    static AUDIT_SUSPENDED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Whether held answers go unchecked: a suspended audit captures nothing.
#[cfg(debug_assertions)]
fn audit_suspended() -> bool {
    AUDIT_SUSPENDED.with(std::cell::Cell::get)
}

/// Run `work` with held answers unchecked and uncaptured: for a test that
/// holds a stale one on purpose to show what reads it and what does not, and
/// for one that drives thousands of frames to prove something else.
#[cfg(any(test, feature = "test-support"))]
pub fn without_audit<R>(work: impl FnOnce() -> R) -> R {
    #[cfg(debug_assertions)]
    {
        struct Restore(bool);
        impl Drop for Restore {
            fn drop(&mut self) {
                AUDIT_SUSPENDED.with(|flag| flag.set(self.0));
            }
        }
        let _restore = Restore(AUDIT_SUSPENDED.with(|flag| flag.replace(true)));
        work()
    }
    #[cfg(not(debug_assertions))]
    work()
}

/// Hold, for the working state as it stands, the answer the accepted
/// content itself would get: what a stamp that missed an edit would leave
/// held. For a test that shows a decision reading past it.
#[cfg(any(test, feature = "test-support"))]
pub fn hold_accepted_answer(state: &dyn LifecycleSource) {
    let state = state.lifecycle_view();
    let lifecycle = &state.platform.project_lifecycle;
    let accepted = lifecycle
        .authority
        .accepted()
        .expect("a held answer is compared with an accepted project");
    let working = rspice_project::registry::document_fingerprints(accepted.baseline());
    let comparison = lifecycle.authority.compare_working(working.as_ref().ok());
    // The digests held are not the ones this answer was made from: the next
    // comparison digests and compares every document.
    lifecycle.dirty_answer.documents.borrow_mut().clear();
    lifecycle.dirty_answer.hold(Answer {
        stamp: WorkingStamp::of(&state),
        baseline: lifecycle.authority.dirty_baseline(),
        compared: Rc::new(Compared {
            working: working.map(OnceCell::from),
            comparison,
        }),
    });
}

/// Panic when the netlist input digest held under a stamp is not the one
/// taken now under that same stamp.
#[cfg(debug_assertions)]
fn audit_netlist_input(
    held: ContentDigest,
    current: &Result<ContentDigest, ProjectLifecycleError>,
) {
    if audit_suspended() {
        return;
    }
    assert!(
        current.as_ref().is_ok_and(|current| *current == held),
        "the generated netlist input held for this working stamp is stale: it now reads \
         {current:?} while no stamped subject changed. Whatever was edited needs a stamp \
         contributor."
    );
}

/// Panic when the working content held under a stamp is not the content
/// captured now under that same stamp: a subject changed that the stamp
/// does not cover.
#[cfg(debug_assertions)]
fn audit(held: &Compared, answer: &DirtyAnswer, current: &Result<DocumentFingerprints, String>) {
    if audit_suspended() {
        return;
    }
    let documents = answer.documents.borrow();
    let working = held
        .working
        .as_ref()
        .map(|working| working.get_or_init(|| documents.fingerprints()));
    let differing = match (working, current) {
        (Ok(held), Ok(current)) => held
            .differing_documents(current)
            .iter()
            .map(rspice_project::registry::ProjectDocumentId::stable_key)
            .collect::<Vec<_>>(),
        (Err(_), Err(_)) => Vec::new(),
        (Ok(_), Err(error)) => vec![format!("the capture, which now fails: {error}")],
        (Err(error), Ok(_)) => vec![format!("the capture, which failed then: {error}")],
    };
    assert!(
        differing.is_empty(),
        "the dirty answer held for this working stamp is stale: {} changed while no \
         stamped subject did. Whatever was edited needs a stamp contributor.",
        differing.join(", ")
    );
}
