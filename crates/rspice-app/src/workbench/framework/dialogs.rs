//! The dialog host: one spec for each step of a frame's dialogs, and the walk
//! that draws them in the one order the shell lists them in.
//!
//! The order is behaviour. A dialog drawn later sits above one drawn earlier
//! and takes the keyboard from it, and a step that hands its outcome on is
//! answered by the step after it in the same frame. So the host draws every
//! step, open or not, in list order, and each step decides for itself whether
//! it has anything to draw.

/// The name a dialog step is listed under: the slice that owns the step, a
/// dot, and the step, as in `design.bus_tap`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DialogKey(pub(crate) &'static str);

/// How a dialog step holds the workbench while what it draws is open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Modality {
    /// While it is open nothing beneath it answers the keyboard or the
    /// pointer: the application modal holds.
    Application,
    /// Drawn beside the workspace, which stays live beneath it.
    Modeless,
    /// A step that draws no dialog of its own: it keeps routes and the
    /// dialogs they own in step, or finishes what a dialog decided.
    None,
}

/// One step of the frame's dialogs, drawn by a host `H` whose state is `S`.
pub(crate) struct DialogSpec<H, S> {
    pub(crate) key: DialogKey,
    pub(crate) modality: Modality,
    /// Whether what the step draws is open, as the application modal asks
    /// it. Only an [`Modality::Application`] step answers it.
    pub(crate) is_open: Option<fn(&S) -> bool>,
    /// Draw the step for this frame. It returns at once when nothing it
    /// draws is open.
    pub(crate) render: fn(&mut H, &egui::Context),
}

/// Draw one frame's dialogs: every step of `order`, first to last.
pub(crate) fn render<H, S>(host: &mut H, ctx: &egui::Context, order: &[DialogSpec<H, S>]) {
    for spec in order {
        (spec.render)(host, ctx);
    }
}

/// Fails the build unless `order` holds together: each key is listed once,
/// and only an application-modal step says whether it is open.
pub(crate) const fn check<H, S>(order: &[DialogSpec<H, S>]) {
    let mut index = 0;
    while index < order.len() {
        let spec = &order[index];
        assert!(
            spec.is_open.is_none() || matches!(spec.modality, Modality::Application),
            "only an application-modal dialog step says whether it is open"
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
