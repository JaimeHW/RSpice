//! Keyboard shortcut profiles as the user's preferences hold them.
//!
//! A profile binds commands to chords per platform. This module owns the
//! profile values and their validation, and the named-preset library a user
//! can switch between. A profile names each command by its key and reads what
//! it needs to know about one through a
//! [`CommandCatalog`](super::command_catalog::CommandCatalog).

mod library;
mod profile;
mod types;

pub use library::{ShortcutProfileLibrary, ShortcutProfileLibraryError};
pub use profile::shortcut_contexts_overlap;
pub use profile::{
    ShortcutPreferences, ShortcutProfileAudit, ShortcutProfileIssue, ShortcutProfileIssueCode,
    ShortcutProfileIssueSeverity, shortcut_context_precedence_rank,
};
pub use types::{
    ChordTimeoutPolicy, CommandShortcutOverride, ContextPrecedencePolicy,
    MAX_SHORTCUT_SEQUENCE_STROKES, ProfileShortcutBinding, ProtectedShortcutPolicy,
    ResolvedShortcutBinding, ShortcutBindingSlot, ShortcutBindingSource, ShortcutPolicies,
    ShortcutProfileError, ShortcutSequence, ShortcutStroke, SingleKeyCanvasPolicy,
};
