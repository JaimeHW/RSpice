//! One-shot requests between the parts of the workbench, and the points in
//! the frame where each is taken.
//!
//! A part that wants another part to act once (export the result on screen,
//! focus a search field, put text on the platform clipboard) pushes a typed
//! request into the [`Outbox`] rather than setting a flag in the other part's
//! state. Each request type names the one point in the frame where its
//! consumer takes it, so whether a request is answered in the frame that
//! raised it or in the next one follows from frame order alone.
//!
//! The outbox lives in the workbench session for the life of the process and
//! is never saved. A request waits there, across frames, until its consumer
//! takes it, as a set flag stayed set until it was read: nothing discards it
//! at a frame boundary or at its drain point. Raising a request again before
//! it is taken replaces the waiting one, as setting a flag twice left one flag
//! set and writing an `Option` twice kept the last value.

use std::any::Any;
use std::fmt;

/// The most times one drain answers its request in a single pass.
///
/// A drain answers until nothing of its type is left, so a consumer that
/// raises its own request again in answer is run again at once. Past this
/// many rounds that is a loop, not a cascade.
const DRAIN_ROUNDS: usize = 64;

/// A fixed point in the frame where a consumer takes its requests, listed in
/// the order the main window reaches them.
///
/// Each is the place a one-shot flag used to be read, and a request taken
/// there keeps that flag's timing: raised earlier in the frame, it is answered
/// in the same frame; raised later, it waits for the next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drain {
    /// The last step of the frame's preparation: after the shortcuts, the
    /// imports and the run controller, before the cloud account's polls.
    EndOfPrepare,
    /// The live-session engine's pump, after the cloud account's polls.
    LiveSessionPump,
    /// Straight after the live-session pump, before autosave and the chrome.
    AfterLiveSession,
    /// A navigator's filter field, wherever the navigator on screen draws
    /// one: in its dock, or in its drawer after the surfaces.
    NavigatorFilter,
    /// The component shelf's search field.
    ShelfSearch,
    /// The source editor a workspace surface draws, just before drawing it.
    SourceEditor,
    /// The project launcher's search field.
    LauncherSearch,
    /// After the notification centre and before the toasts.
    BeforeToasts,
    /// The last step of a window's chrome.
    AfterChrome,
    /// Straight after the feature call that raised it returns, wherever in
    /// the frame that call ran: a command another command raised runs once
    /// the one that raised it is done.
    AfterContribution,
    /// The drawing sheet's inspector section, as it draws: in the inspector's
    /// dock, or in its drawer after the surfaces.
    SheetSection,
}

/// A value one part of the workbench raises for another to act on once.
pub trait Request: Clone + fmt::Debug + 'static {
    /// Where in the frame the request is taken.
    const DRAIN: Drain;
}

/// Requests raised and not yet taken, in the order they were raised.
#[derive(Clone, Debug, Default)]
pub struct Outbox {
    waiting: Vec<Waiting>,
}

impl Outbox {
    /// Raise `request`. A request of the same type already waiting is
    /// replaced: the later raise is the one answered.
    pub fn push<R: Request>(&mut self, request: R) {
        self.waiting.retain(|waiting| !waiting.holds::<R>());
        self.waiting.push(Waiting {
            drain: R::DRAIN,
            request: Box::new(request),
        });
    }

    /// Take the waiting `R`, if one was raised. Only its consumer calls this,
    /// at `R::DRAIN`.
    pub fn take<R: Request>(&mut self) -> Option<R> {
        let index = self.waiting.iter().position(Waiting::holds::<R>)?;
        let request = self.waiting.remove(index).request.into_any();
        request.downcast::<R>().ok().map(|request| *request)
    }

    /// Whether a request waits to be taken at `drain`: the question a
    /// consumer asks when it does not name the type it takes there.
    pub fn waits_at(&self, drain: Drain) -> bool {
        self.waiting.iter().any(|waiting| waiting.drain == drain)
    }

    /// Take every request waiting at `drain` and drop it unanswered. This is
    /// a consumer turning a request down at its own point, never a frame
    /// boundary clearing what nobody has taken yet.
    pub(crate) fn decline_at(&mut self, drain: Drain) {
        self.waiting.retain(|waiting| waiting.drain != drain);
    }

    /// The waiting `R`, left where it is.
    #[cfg(any(test, feature = "test-support"))]
    pub fn peek<R: Request>(&self) -> Option<&R> {
        self.waiting
            .iter()
            .find_map(|waiting| waiting.request.as_any().downcast_ref::<R>())
    }
}

/// Answer the `R` waiting in `host`'s outbox, and any its answer raises
/// again, until none is left. The shell calls this at `R::DRAIN`; a request
/// raised after that point in the frame waits for the next frame's pass.
pub fn drain<H, R: Request>(
    host: &mut H,
    outbox: fn(&mut H) -> &mut Outbox,
    mut answer: impl FnMut(&mut H, R),
) {
    let mut rounds = 0;
    while let Some(request) = outbox(host).take::<R>() {
        rounds += 1;
        debug_assert!(
            rounds <= DRAIN_ROUNDS,
            "{} raised itself again in answer {DRAIN_ROUNDS} times in one drain",
            std::any::type_name::<R>()
        );
        answer(host, request);
    }
}

/// Export the quantities of the Data Browser rows the reader picked. Raised
/// by the navigator's data browser; the frame writes the file, because the
/// export IO belongs to the application. The keys, rather than a copied
/// payload, cross that boundary, so the export re-resolves immutable evidence
/// when it publishes.
#[derive(Clone, Debug)]
pub struct ExportResultQuantities(pub Vec<rspice_results_ui::selection::ResultBrowserSelectionKey>);

impl Request for ExportResultQuantities {
    const DRAIN: Drain = Drain::BeforeToasts;
}

/// Ask for a format and export the displayed dataset in it. Raised by a
/// result document's export control.
#[derive(Clone, Debug)]
pub struct ExportCsv;

impl Request for ExportCsv {
    const DRAIN: Drain = Drain::BeforeToasts;
}

/// Open the plan that produced the displayed result. Raised by the result
/// manifest and taken after the panels are drawn, so the workspace changes
/// between frames rather than under the control that asked for it.
#[derive(Clone, Debug)]
pub struct OpenProducingPlan;

impl Request for OpenProducingPlan {
    const DRAIN: Drain = Drain::BeforeToasts;
}

/// Export the active view as a publication figure, through the hardcopy
/// export workflow that renders a sheet as PDF/A, PDF, SVG or PNG at a chosen
/// resolution rather than a capture of the window. Raised by a result
/// document while it is drawn, so the next frame's preparation opens it.
#[derive(Clone, Debug)]
pub struct ExportFigure;

impl Request for ExportFigure {
    const DRAIN: Drain = Drain::EndOfPrepare;
}

/// Open the print workflow on the active view. Raised by a feature that
/// offers printing as the next step of its own dialog, so the workflow opens
/// straight after that dialog is drawn, in the same frame.
#[derive(Clone, Debug)]
pub struct PrintActiveView;

impl Request for PrintActiveView {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Move design's snap onto the open schematic's grid. Raised by a feature
/// that changes the grid the schematic is drawn on, so the snap follows the
/// grid in the same frame, as it does when design changes the grid itself.
#[derive(Clone, Debug)]
pub struct FollowSchematicGrid;

impl Request for FollowSchematicGrid {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Drop whatever chord is half-typed: the shortcut library in use changed,
/// so a chord begun under the previous one must not complete under this
/// one. Raised by the settings feature when it puts a library in use, and
/// taken straight after the call that raised it, before any shortcut is
/// resolved again.
#[derive(Clone, Debug)]
pub struct ResetShortcutResolver;

impl Request for ResetShortcutResolver {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Start the open waveform view from Results ▸ Waveform defaults, as a new
/// view starts: wrapped or unwrapped phase, and the minor grid. Raised by
/// the settings feature when the reader changes or restores those
/// defaults, and taken straight after the call that raised it, before the
/// view is drawn again.
#[derive(Clone, Debug)]
pub struct ApplyWaveformDefaults;

impl Request for ApplyWaveformDefaults {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Export the displayed dataset as the File menu does: the edit typed into
/// the inspector lands, then the format is asked. Raised by the results
/// feature's Export dataset… command, and taken straight after it returns.
#[derive(Clone, Debug)]
pub struct ExportDataset;

impl Request for ExportDataset {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Select an analysis instance on the simulation plan's Analyses page, or
/// none. Raised by the results feature's hop back to the plan that produced
/// a dataset, and taken straight after it returns, before the page draws.
#[derive(Clone, Copy, Debug)]
pub struct SelectAnalysisInstance(pub Option<crate::product::AnalysisInstanceId>);

impl Request for SelectAnalysisInstance {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Open the source the engine executed for the run of this sequence, or
/// say why it cannot be. Raised by the results feature's Open task deck
/// command, and taken straight after it returns.
#[derive(Clone, Copy, Debug)]
pub struct RevealExecutedDeck(pub u64);

impl Request for RevealExecutedDeck {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Show these waveforms in the waveform viewer: each revealed where its
/// dataset hides it, `selected` made the selected trace, and the cursors
/// cleared. Raised by a hand-off that opens the viewer on them, just before
/// it runs the viewer's command, and taken straight after that command
/// returns.
#[derive(Clone, Debug)]
pub struct ShowWaveforms {
    /// Each waveform, and whether its dataset shows it.
    pub visibility: Vec<(
        rspice_results::result_presentation::SourceWaveformPresentationKey,
        bool,
    )>,
    /// The trace to select, where the hand-off names a single one.
    pub selected: Option<rspice_results_ui::selection::SelectedResultTrace>,
}

impl Request for ShowWaveforms {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Narrow the Family slicing form to this filter. Raised by a hand-off that
/// opens the form on one member of a family, just before it runs the form's
/// command, and taken straight after that command returns: opening the form
/// seeds its filter from the policy in force, and the member has to survive
/// that seeding.
#[derive(Clone, Debug)]
pub struct SetFamilyFilter(pub String);

impl Request for SetFamilyFilter {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Put this answer on the status bar once the commands the same call raised
/// have run. Raised by a gesture that runs a command and then says what it
/// did, so its answer is the one the bar shows, over anything the command's
/// prologue said on the way, as when the command ran in place.
#[derive(Clone, Debug)]
pub struct PromptAfterCommands(pub String);

impl Request for PromptAfterCommands {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Show these annotations on the schematic. Raised by a result viewer that
/// locates what it reports on the schematic, as it opens Design, and taken
/// straight after the call that raised it, before the schematic draws.
#[derive(Clone, Copy, Debug)]
pub struct ShowSchematicAnnotations(
    pub rspice_schematic_editor::session::visibility::SchematicAnnotationVisibility,
);

impl Request for ShowSchematicAnnotations {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Mark on the schematic the objects the failed analysis on display was
/// attributed to, and, when the marking lands, record it as the marking of
/// this run. Raised by the Results failure banner's control, and taken
/// straight after the call that raised it.
#[derive(Clone, Copy, Debug)]
pub struct MarkFailureSites(pub Option<u64>);

impl Request for MarkFailureSites {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Bring the drawing sheet's inspector section into view. Raised by a
/// feature that shows the open sheet's properties from its own surface,
/// once it has cleared the selection and opened the inspector; the section
/// takes it as it next draws.
#[derive(Clone, Debug)]
pub struct RevealSheetSection;

impl Request for RevealSheetSection {
    const DRAIN: Drain = Drain::SheetSection;
}

/// Put this text on the platform clipboard.
#[derive(Clone, Debug)]
pub struct ClipboardText(pub String);

impl Request for ClipboardText {
    const DRAIN: Drain = Drain::AfterLiveSession;
}

/// Enter (`true`) or leave platform full screen. Taken once, so the web
/// backend is not sent the same viewport command every frame, and the
/// session's own full-screen state is left to say what the host is doing.
#[derive(Clone, Debug)]
pub struct FullScreen(pub bool);

impl Request for FullScreen {
    const DRAIN: Drain = Drain::AfterChrome;
}

/// Adopt the live session host's project as this install's mirror. Raised
/// once the close that makes room for it has landed, or at once when no
/// project is open.
#[derive(Clone, Debug)]
pub struct LiveMirrorEntry;

impl Request for LiveMirrorEntry {
    const DRAIN: Drain = Drain::LiveSessionPump;
}

/// Adopt the project's own netlist source a live session replaced, with the
/// root document rewritten over it when the project keeps one. Raised as the
/// session applies a participant's source, and taken once the session's
/// inbound half is done, before the host's project can replace the one it
/// applied to.
#[derive(Clone, Debug)]
pub struct AdoptLiveOwnedSource(pub Option<rspice_app_state::state::NetlistDocument>);

impl Request for AdoptLiveOwnedSource {
    const DRAIN: Drain = Drain::LiveSessionPump;
}

/// Close the open project to make room for the live session host's as this
/// install's mirror: through the close review when it holds unsaved work,
/// at once otherwise. Raised by the live session's dialog, and taken straight
/// after the dialog step that raised it.
#[derive(Clone, Debug)]
pub struct CloseForLiveMirror;

impl Request for CloseForLiveMirror {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// A project file command, carried out by the shell's file workflow as the
/// File menu carries it out: the edit typed into the inspector lands, the
/// project's unsaved work is reviewed where the command would lose it, and
/// then the dialog, the save or the close runs. Raised by the project
/// feature's file commands, and taken straight after the command returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectFileAction {
    NewProject,
    OpenProject,
    OpenSchematic,
    Save,
    SaveAs,
    SaveAll,
    CloseProject,
}

impl Request for ProjectFileAction {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// A project dialog the shell opens over the whole application, for the
/// project feature's command that opens it: each reads the drawing, its
/// checks or the library catalog beyond what the feature's context holds.
/// Taken straight after the command returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenProjectDialog {
    NewView,
    DesignManagement,
    RevisionHistory,
    CheckAndSave,
}

impl Request for OpenProjectDialog {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Focus the navigator's filter field the next time a navigator draws it.
#[derive(Clone, Debug)]
pub struct FocusNavigatorSearch;

impl Request for FocusNavigatorSearch {
    const DRAIN: Drain = Drain::NavigatorFilter;
}

/// Focus the component shelf's search field the next time it is drawn.
#[derive(Clone, Debug)]
pub struct FocusPlacementSearch;

impl Request for FocusPlacementSearch {
    const DRAIN: Drain = Drain::ShelfSearch;
}

/// Focus the project launcher's search field, raised as the launcher opens.
#[derive(Clone, Debug)]
pub struct FocusProjectLauncherSearch;

impl Request for FocusProjectLauncherSearch {
    const DRAIN: Drain = Drain::LauncherSearch;
}

#[derive(Debug)]
struct Waiting {
    drain: Drain,
    request: Box<dyn Queued>,
}

impl Waiting {
    fn holds<R: Request>(&self) -> bool {
        self.request.as_any().is::<R>()
    }
}

impl Clone for Waiting {
    fn clone(&self) -> Self {
        Self {
            drain: self.drain,
            request: self.request.clone_queued(),
        }
    }
}

/// A request with its type erased, so requests of every type wait in one
/// list in the order they were raised.
trait Queued: fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn clone_queued(&self) -> Box<dyn Queued>;
}

impl<R: Request> Queued for R {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_queued(&self) -> Box<dyn Queued> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct Early(u32);
    impl Request for Early {
        const DRAIN: Drain = Drain::EndOfPrepare;
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Late;
    impl Request for Late {
        const DRAIN: Drain = Drain::BeforeToasts;
    }

    fn order(outbox: &Outbox) -> Vec<Drain> {
        outbox.waiting.iter().map(|waiting| waiting.drain).collect()
    }

    #[derive(Default)]
    struct Host {
        outbox: Outbox,
        answered: Vec<u32>,
    }

    fn outbox(host: &mut Host) -> &mut Outbox {
        &mut host.outbox
    }

    #[test]
    fn requests_wait_in_the_order_raised_and_a_second_raise_replaces_the_first() {
        let mut outbox = Outbox::default();
        outbox.push(Early(1));
        outbox.push(Late);
        assert_eq!(order(&outbox), [Drain::EndOfPrepare, Drain::BeforeToasts]);

        outbox.push(Early(2));
        assert_eq!(order(&outbox), [Drain::BeforeToasts, Drain::EndOfPrepare]);
        assert_eq!(outbox.peek::<Early>(), Some(&Early(2)));
        assert_eq!(outbox.take::<Early>(), Some(Early(2)));
        assert_eq!(outbox.take::<Late>(), Some(Late));
    }

    #[test]
    fn a_take_empties_its_type_and_leaves_every_other_waiting() {
        let mut outbox = Outbox::default();
        outbox.push(Early(7));
        outbox.push(Late);

        assert_eq!(outbox.take::<Early>(), Some(Early(7)));
        assert_eq!(outbox.take::<Early>(), None);
        assert_eq!(outbox.peek::<Early>(), None);
        assert_eq!(outbox.peek::<Late>(), Some(&Late));
        assert!(outbox.waits_at(Drain::BeforeToasts));
        assert!(!outbox.waits_at(Drain::EndOfPrepare));
    }

    #[test]
    fn a_request_is_held_across_frames_while_its_consumer_does_not_run() {
        let mut host = Host::default();
        host.outbox.push(Early(3));
        // Frames in which other points drain and other requests come and go,
        // but the consumer of `Early` is not drawn.
        for _ in 0..5 {
            host.outbox.push(Late);
            drain(&mut host, outbox, |_, Late| {});
        }
        assert_eq!(host.outbox.peek::<Early>(), Some(&Early(3)));

        drain(&mut host, outbox, |host, Early(value)| {
            host.answered.push(value)
        });
        assert_eq!(host.answered, [3]);
        assert_eq!(host.outbox.peek::<Early>(), None);
    }

    #[test]
    fn a_request_raised_after_its_drain_waits_for_the_next_frame() {
        fn frame(host: &mut Host, raise: Option<u32>) {
            drain(host, outbox, |host, Early(value)| host.answered.push(value));
            if let Some(value) = raise {
                host.outbox.push(Early(value));
            }
        }
        let mut host = Host::default();
        frame(&mut host, Some(5));
        assert!(host.answered.is_empty());
        assert_eq!(host.outbox.peek::<Early>(), Some(&Early(5)));

        frame(&mut host, None);
        assert_eq!(host.answered, [5]);
        assert_eq!(host.outbox.peek::<Early>(), None);
    }

    #[test]
    fn a_drain_answers_what_its_answer_raises_again_in_the_same_pass() {
        let mut host = Host::default();
        host.outbox.push(Early(0));
        drain(&mut host, outbox, |host, Early(value)| {
            host.answered.push(value);
            if value < 3 {
                host.outbox.push(Early(value + 1));
            }
        });
        assert_eq!(host.answered, [0, 1, 2, 3]);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "raised itself again in answer 64 times in one drain")]
    fn a_drain_whose_answer_always_raises_again_trips_the_round_cap() {
        let mut host = Host::default();
        host.outbox.push(Early(0));
        drain(&mut host, outbox, |host, Early(value)| {
            host.outbox.push(Early(value + 1));
        });
    }

    #[test]
    fn a_declined_point_drops_only_its_own_requests() {
        let mut outbox = Outbox::default();
        outbox.push(Early(1));
        outbox.push(Late);
        outbox.decline_at(Drain::BeforeToasts);
        assert_eq!(outbox.peek::<Late>(), None);
        assert_eq!(outbox.peek::<Early>(), Some(&Early(1)));
    }

    #[test]
    fn a_cloned_outbox_carries_what_is_waiting() {
        let mut outbox = Outbox::default();
        outbox.push(Early(9));
        let mut copy = outbox.clone();
        assert_eq!(copy.take::<Early>(), Some(Early(9)));
        assert_eq!(outbox.peek::<Early>(), Some(&Early(9)));
    }
}
