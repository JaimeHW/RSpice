//! Notices the model raises for the reader: a failure, or a warning, of
//! something they asked for.
//!
//! The model writes the Console line itself and queues the notice here. The
//! toasts belong to the workbench, so the shell raises each queued notice as a
//! toast, in the order queued, at fixed points of the frame: at its start, and
//! before anything that shows the toasts or reads what they hold.
//!
//! The queue is runtime state. It is never persisted, and a clone of it is
//! empty, so a state cloned for a project transaction cannot raise a pending
//! notice twice.

/// How a notice reads: a failure, or something that completed, but not the way
/// the reader asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NoticeKind {
    Error,
    Warning,
}

/// One notice, and the Console line that keeps it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Notice {
    pub(crate) kind: NoticeKind,
    pub(crate) title: String,
    pub(crate) detail: String,
    /// The Console line written for this notice. The toast offers to show it,
    /// so the mirrored copy of that line is the same notice, not another.
    pub(crate) log_id: u64,
}

/// Notices raised and not yet shown, oldest first.
#[derive(Debug, Default)]
pub(crate) struct NoticeQueue {
    pending: Vec<Notice>,
}

/// A notice belongs to the state that raised it. A copy of that state, such
/// as a project transaction's candidate, starts with none.
impl Clone for NoticeQueue {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl NoticeQueue {
    pub(crate) fn push(&mut self, notice: Notice) {
        self.pending.push(notice);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Every pending notice, oldest first, leaving the queue empty.
    pub(crate) fn take(&mut self) -> Vec<Notice> {
        std::mem::take(&mut self.pending)
    }

    /// Put `earlier`, raised before anything queued here, ahead of it.
    pub(crate) fn prepend(&mut self, mut earlier: NoticeQueue) {
        earlier.pending.append(&mut self.pending);
        self.pending = earlier.pending;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn notice(title: &str, log_id: u64) -> Notice {
        Notice {
            kind: NoticeKind::Error,
            title: title.to_owned(),
            detail: "detail".to_owned(),
            log_id,
        }
    }

    #[test]
    fn a_clone_holds_no_pending_notice() {
        let mut queue = NoticeQueue::default();
        queue.push(notice("Save failed", 1));

        assert!(queue.clone().is_empty());
        assert!(!queue.is_empty(), "the original keeps its notice");
    }

    #[test]
    fn notices_leave_in_the_order_they_were_raised() {
        let mut queue = NoticeQueue::default();
        queue.push(notice("first", 1));
        queue.push(notice("second", 2));

        let titles: Vec<_> = queue.take().into_iter().map(|n| n.title).collect();
        assert_eq!(titles, ["first", "second"]);
        assert!(queue.is_empty());
    }

    #[test]
    fn earlier_notices_go_ahead_of_later_ones() {
        let mut earlier = NoticeQueue::default();
        earlier.push(notice("live", 1));
        let mut later = NoticeQueue::default();
        later.push(notice("candidate", 2));

        later.prepend(earlier);

        let ids: Vec<_> = later.take().into_iter().map(|n| n.log_id).collect();
        assert_eq!(ids, [1, 2]);
    }
}
