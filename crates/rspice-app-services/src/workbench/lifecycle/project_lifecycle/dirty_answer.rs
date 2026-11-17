//! One dirty answer per working state.
//!
//! The toolbar, the window title, the Project pages and command enablement
//! ask whether the project holds unsaved work every frame. Each reads the
//! comparison made for the working project's stamp, and only a stamp that
//! moved captures and fingerprints the whole project again. A decision that
//! can drop work — save, close, quit, replacing the project — reads
//! [`fresh`], which never trusts what is held.
//!
//! The generated netlist's input digest, which the Netlist page reads every
//! frame, is held for the working stamp the same way.
//!
//! In debug builds and tests every held answer is checked: the project is
//! captured again and a document whose identity moved under an unchanged
//! stamp panics, naming it. A subject the stamp misses fails the suite; it
//! cannot ship as lost work.

use std::cell::RefCell;
use std::rc::Rc;

use rspice_app_state::workbench::app_state::model::InPlaceLayout;
use rspice_project::SnapshotContent;
use rspice_project::lifecycle::{DirtyBaseline, DirtyComparison};
use rspice_project::registry::DocumentFingerprints;
use rspice_vocabulary::product::ContentDigest;

use super::{LifecycleSource, LifecycleView, ProjectLifecycleError};

/// Everything a capture of the working project reads: each subject by the
/// stamp its owner issues on every mutable access, the two with no stamp of
/// their own by content. Equal stamps, equal working content.
#[derive(Debug, Clone, PartialEq)]
struct WorkingStamp {
    /// The workspace content: every buffered schematic, the project
    /// descriptor, configurations, plan payloads, sources, layouts, reports,
    /// result documents, hardcopy setups, open views and their dirty marks.
    workspace: u64,
    /// The schematic in the editor, which a capture takes in place of its
    /// buffered copy.
    active_schematic: u64,
    /// The design library catalog.
    libraries: u64,
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
    fn of(state: &LifecycleView<'_>) -> Self {
        let model = state.model;
        Self {
            workspace: model.workspace.content.stamp(),
            active_schematic: model.schematic.editor_ref().design.content_stamp(),
            libraries: model.library_manager.content_stamp(),
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
}

/// The last comparison, with the working stamp and baseline it was made for.
#[derive(Debug, Clone, Default)]
pub struct DirtyAnswer(RefCell<Option<Answer>>);

#[derive(Debug, Clone)]
struct Answer {
    stamp: WorkingStamp,
    baseline: DirtyBaseline,
    compared: Rc<Compared>,
}

/// One capture of the working project, fingerprinted and compared.
#[derive(Debug)]
struct Compared {
    working: Result<DocumentFingerprints, String>,
    comparison: DirtyComparison,
}

impl DirtyAnswer {
    fn held(&self, stamp: &WorkingStamp, baseline: &DirtyBaseline) -> Option<Rc<Compared>> {
        self.0
            .borrow()
            .as_ref()
            .filter(|held| held.stamp == *stamp && held.baseline == *baseline)
            .map(|held| Rc::clone(&held.compared))
    }

    fn hold(&self, answer: Answer) {
        *self.0.borrow_mut() = Some(answer);
    }
}

/// `answer`, read from the comparison held for the working state as it
/// stands, made once per stamp.
pub(super) fn read<R>(state: &LifecycleView<'_>, answer: impl FnOnce(&DirtyComparison) -> R) -> R {
    let authority = &state.platform.project_lifecycle.authority;
    if !authority.compares_working_content() {
        return answer(&authority.compare_working(None));
    }
    answer(&held_or_compared(state).comparison)
}

/// `read` the working project's fingerprints as held for its stamp, taken
/// once per stamp; the error when it could not be captured or fingerprinted.
pub(super) fn working<R>(
    state: &LifecycleView<'_>,
    read: impl FnOnce(Result<&DocumentFingerprints, &str>) -> R,
) -> R {
    read(
        held_or_compared(state)
            .working
            .as_ref()
            .map_err(String::as_str),
    )
}

fn held_or_compared(state: &LifecycleView<'_>) -> Rc<Compared> {
    let stamp = WorkingStamp::of(state);
    let baseline = state.platform.project_lifecycle.authority.dirty_baseline();
    if let Some(held) = state
        .platform
        .project_lifecycle
        .dirty_answer
        .held(&stamp, &baseline)
    {
        #[cfg(debug_assertions)]
        if !audit_suspended() {
            audit(
                &held,
                &rspice_project::registry::unobserved(|| fingerprint(state)),
            );
        }
        return held;
    }
    compare(state, stamp, baseline)
}

/// `answer`, from a capture made now whatever is held. For every decision
/// that drops or keeps the user's work.
pub(super) fn fresh<R>(state: &LifecycleView<'_>, answer: impl FnOnce(&DirtyComparison) -> R) -> R {
    let authority = &state.platform.project_lifecycle.authority;
    if !authority.compares_working_content() {
        return answer(&authority.compare_working(None));
    }
    let stamp = WorkingStamp::of(state);
    let baseline = authority.dirty_baseline();
    #[cfg(debug_assertions)]
    let held = state
        .platform
        .project_lifecycle
        .dirty_answer
        .held(&stamp, &baseline);
    let compared = compare(state, stamp, baseline);
    #[cfg(debug_assertions)]
    if let Some(held) = held {
        audit(&held, &compared.working);
    }
    answer(&compared.comparison)
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

/// Capture and compare the working project, and hold the comparison for
/// `stamp`, taken before the capture read anything.
fn compare(
    state: &LifecycleView<'_>,
    stamp: WorkingStamp,
    baseline: DirtyBaseline,
) -> Rc<Compared> {
    let working = fingerprint(state);
    let comparison = state
        .platform
        .project_lifecycle
        .authority
        .compare_working(working.as_ref().ok());
    let compared = Rc::new(Compared {
        working,
        comparison,
    });
    state.platform.project_lifecycle.dirty_answer.hold(Answer {
        stamp,
        baseline,
        compared: Rc::clone(&compared),
    });
    compared
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
    lifecycle.dirty_answer.hold(Answer {
        stamp: WorkingStamp::of(&state),
        baseline: lifecycle.authority.dirty_baseline(),
        compared: Rc::new(Compared {
            working,
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
fn audit(held: &Compared, current: &Result<DocumentFingerprints, String>) {
    if audit_suspended() {
        return;
    }
    let differing = match (&held.working, current) {
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
