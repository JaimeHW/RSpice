//! The canvas's operating-point voltage badges, kept across frames while
//! what they were read from stands still.
//!
//! Placing a badge joins every solved node to the run's geometry and then to
//! what the active sheet draws. That join changes only when the run, the
//! drawing, the sheet or the occurrence does, so a steady frame reuses it and
//! only positions the text.

use std::sync::Arc;

use rspice_schematic_editor::view::scene::{BadgeLayoutMemo, OperatingPointCanvasAnnotation};

/// One set of voltage badges and a fingerprint of everything they were read
/// from. The fingerprint hashes the inputs by content, so an input replaced
/// without a counter moving still misses.
///
/// It also holds where the canvas last placed the badges. Placing them
/// against each other and against every label, name and body on the sheet
/// is quadratic; the layout is reused while its inputs, compared by content,
/// stand still.
#[derive(Clone, Default)]
pub struct OperatingPointMemo {
    entry: Option<(u64, Arc<Vec<OperatingPointCanvasAnnotation>>)>,
    layout: BadgeLayoutMemo,
}

impl std::fmt::Debug for OperatingPointMemo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OperatingPointMemo")
            .field(
                "badges",
                &self.entry.as_ref().map(|(_, badges)| badges.len()),
            )
            .finish()
    }
}

impl OperatingPointMemo {
    /// The badges read from inputs with this fingerprint, if they are held.
    pub(crate) fn current(
        &self,
        fingerprint: u64,
    ) -> Option<Arc<Vec<OperatingPointCanvasAnnotation>>> {
        self.entry
            .as_ref()
            .filter(|(held, _)| *held == fingerprint)
            .map(|(_, badges)| Arc::clone(badges))
    }

    pub(crate) fn store(
        &mut self,
        fingerprint: u64,
        badges: Arc<Vec<OperatingPointCanvasAnnotation>>,
    ) {
        self.entry = Some((fingerprint, badges));
    }

    /// Where the canvas holds the badges' last layout.
    pub(crate) const fn layout(&self) -> &BadgeLayoutMemo {
        &self.layout
    }
}
