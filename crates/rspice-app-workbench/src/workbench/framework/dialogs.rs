//! The dialog host: one spec for each step of a frame's dialogs, and the walk
//! that draws them in the one order the shell lists them in.
//!
//! The order is behaviour. A dialog drawn later sits above one drawn earlier
//! and takes the keyboard from it, and a step that hands its outcome on is
//! answered by the step after it in the same frame. So the host draws every
//! step, open or not, in list order, and each step decides for itself whether
//! it has anything to draw.
//!
//! The same specs answer the modal gates. Each modal step says whether what
//! it draws is open, each modal state no step draws is listed beside the
//! steps, and a gate is open while anything it counts is.

/// The name a dialog step is listed under: the slice that owns the step, a
/// dot, and the step, as in `design.bus_tap`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogKey(pub &'static str);

/// How a dialog step holds the workbench while what it draws is open.
///
/// Two gates answer for the modal ones. The dialog gate is the narrower: the
/// workbench asks it before it opens a dialog of its own. The application
/// modal holds whenever either kind is open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modality {
    /// While it is open nothing beneath it answers the keyboard or the
    /// pointer, and the dialog gate holds as well as the application modal.
    Dialog,
    /// While it is open nothing beneath it answers the keyboard or the
    /// pointer: the application modal holds, and the dialog gate does not.
    Application,
    /// Drawn beside the workspace, which stays live beneath it.
    Modeless,
    /// A step that draws no dialog of its own: it keeps routes and the
    /// dialogs they own in step, or finishes what a dialog decided.
    None,
}

impl Modality {
    const fn is_modal(self) -> bool {
        matches!(self, Self::Dialog | Self::Application)
    }
}

/// One step of the frame's dialogs, drawn by a host `H` whose state is `S`.
pub struct DialogSpec<H, S> {
    pub key: DialogKey,
    pub modality: Modality,
    /// Whether what the step draws is open, as the modal gates ask it. A
    /// modal step answers it, and no other step does.
    pub is_open: Option<fn(&S) -> bool>,
    /// Draw the step for this frame. It returns at once when nothing it
    /// draws is open.
    pub render: fn(&mut H, &egui::Context),
}

/// Something open that holds a modal gate but that no step of the frame's
/// dialogs draws: a route, a dialog a surface or the chrome draws, or a
/// dialog a step draws whose gate differs from the step's own.
pub struct ModalSpec<S> {
    pub key: DialogKey,
    pub modality: Modality,
    pub is_open: fn(&S) -> bool,
}

/// The state the modal gates are asked of, and where they look: the steps
/// of its host's frame dialogs, and the modal states no step draws. The
/// shell lists both, so the state answers its gates without naming it.
pub trait ModalGate: Sized + 'static {
    type Host: 'static;
    const ORDER: &'static [DialogSpec<Self::Host, Self>];
    const MODAL_ONLY: &'static [ModalSpec<Self>];
}

/// Whether anything the dialog gate counts is open.
pub fn dialog_modal_open<S: ModalGate>(state: &S) -> bool {
    any_open(state, |modality| matches!(modality, Modality::Dialog))
}

/// Whether anything that holds the application modal is open: everything
/// the dialog gate counts, and everything else that is modal.
pub fn application_modal_open<S: ModalGate>(state: &S) -> bool {
    any_open(state, Modality::is_modal)
}

fn any_open<S: ModalGate>(state: &S, counts: impl Fn(Modality) -> bool) -> bool {
    S::ORDER.iter().any(|spec| {
        spec.is_open
            .is_some_and(|is_open| counts(spec.modality) && is_open(state))
    }) || S::MODAL_ONLY
        .iter()
        .any(|spec| counts(spec.modality) && (spec.is_open)(state))
}

/// Draw one frame's dialogs: every step of `order`, first to last.
pub fn render<H, S>(host: &mut H, ctx: &egui::Context, order: &[DialogSpec<H, S>]) {
    for spec in order {
        (spec.render)(host, ctx);
    }
}

/// Fails the build unless `order` and `modal_only` hold together: each key
/// is listed once, a step says whether it is open exactly when it is modal,
/// and everything `modal_only` lists is modal.
pub const fn check<H, S>(order: &[DialogSpec<H, S>], modal_only: &[ModalSpec<S>]) {
    let mut index = 0;
    while index < order.len() {
        let spec = &order[index];
        assert!(
            spec.is_open.is_some() == spec.modality.is_modal(),
            "a dialog step says whether it is open exactly when it is modal"
        );
        let mut earlier = 0;
        while earlier < index {
            assert!(
                !same_key(order[earlier].key, spec.key),
                "a dialog key is listed twice"
            );
            earlier += 1;
        }
        index += 1;
    }
    let mut index = 0;
    while index < modal_only.len() {
        let spec = &modal_only[index];
        assert!(
            spec.modality.is_modal(),
            "a modal state no step draws holds a modal gate"
        );
        let mut step = 0;
        while step < order.len() {
            assert!(
                !same_key(order[step].key, spec.key),
                "a dialog key is listed twice"
            );
            step += 1;
        }
        let mut earlier = 0;
        while earlier < index {
            assert!(
                !same_key(modal_only[earlier].key, spec.key),
                "a dialog key is listed twice"
            );
            earlier += 1;
        }
        index += 1;
    }
}

const fn same_key(left: DialogKey, right: DialogKey) -> bool {
    let (left, right) = (left.0.as_bytes(), right.0.as_bytes());
    if left.len() != right.len() {
        return false;
    }
    let mut at = 0;
    while at < left.len() {
        if left[at] != right[at] {
            return false;
        }
        at += 1;
    }
    true
}
