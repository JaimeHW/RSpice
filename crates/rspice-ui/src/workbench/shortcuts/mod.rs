mod library;
mod profile;
mod types;

pub use library::{
    NamedShortcutPreset, ShortcutPresetName, ShortcutProfileLibrary, ShortcutProfileLibraryError,
};
pub(crate) use profile::shortcut_contexts_overlap;
pub use profile::{
    ShortcutPreferences, ShortcutProfileAudit, ShortcutProfileIssue, ShortcutProfileIssueCode,
    ShortcutProfileIssueSeverity, shortcut_context_precedence_rank,
};
pub use types::{
    ChordTimeoutPolicy, CommandShortcutOverride, ContextPrecedencePolicy,
    MAX_SHORTCUT_SEQUENCE_STROKES, ProfileShortcutBinding, ProtectedShortcutPolicy,
    ResolvedShortcutBinding, ShortcutBindingSlot, ShortcutBindingSource, ShortcutKey,
    ShortcutPolicies, ShortcutProfileError, ShortcutSequence, ShortcutStroke,
    SingleKeyCanvasPolicy,
};
