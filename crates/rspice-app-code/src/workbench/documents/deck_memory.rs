//! What a deck costs the editor in memory, worked out from its size before
//! it is read, against what this machine has.
//!
//! A deck file whose text alone needs more memory than the machine has
//! installed is refused before it is read. A deck whose indexes need more
//! than the machine has free once its text is read opens with the language
//! features off: the outline, hover, go to definition, completion and the
//! check. Its band says so, with the numbers, and offers to index it anyway.
//! The costs are what the editor was measured to hold for each byte of a
//! deck; the bounds are the machine's own memory, so a larger machine opens
//! and indexes a larger deck.

use std::path::Path;

use crate::simulation::controller::machine_memory as installed_machine_memory;
use crate::state::DeckId;
use crate::workbench::{MessageCatalog, MessageId};
use rspice_simulation::execution::preparation::{MachineMemory, memory_words};

/// What the editor holds of a deck's text for each byte of the deck: the
/// text in one piece as it was read and the rope the editor edits. Measured
/// on a 100 MB deck: 2.14.
const TEXT_BYTES_PER_DECK_BYTE: f64 = 2.2;

/// What the indexes behind the language features hold for each byte of the
/// deck: the outline and the engine's index of the text it reads, the
/// project index with its definitions and node tables, and the map of the
/// deck's values. Measured on a 100 MB deck of devices with long node
/// names, the most an index holds for a byte: 1.6 for the outline and its
/// index, 1.3 for the project index and its tables, 2.9 in all; the rest is
/// what building them holds for a moment.
const INDEX_BYTES_PER_DECK_BYTE: f64 = 4.0;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// The memory a test says this thread's machine has.
    static ASSUMED: std::cell::Cell<Option<MachineMemory>> = const { std::cell::Cell::new(None) };
}

/// Answer `memory` for this thread's machine from now on, and for the
/// threads it starts to read a deck: what a test of a machine smaller than
/// the one it runs on does.
#[cfg(any(test, feature = "test-support"))]
pub fn assume_machine_memory_here(memory: Option<MachineMemory>) {
    ASSUMED.set(memory);
}

/// The memory a test said this thread's machine has, which a thread it
/// starts to read a deck takes on.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn assumed_machine_memory() -> Option<MachineMemory> {
    ASSUMED.get()
}

/// The memory installed and free on this machine, or `None` where the host
/// cannot say: a deck then opens as it would on a machine that holds it.
fn machine_memory() -> Option<MachineMemory> {
    #[cfg(any(test, feature = "test-support"))]
    if let Some(memory) = ASSUMED.get() {
        return Some(memory);
    }
    installed_machine_memory()
}

/// `deck_bytes` bytes of deck at `per_byte` bytes of memory each.
fn cost(deck_bytes: u64, per_byte: f64) -> u64 {
    (deck_bytes as f64 * per_byte).ceil() as u64
}

/// Why the deck file at `path` cannot open on this machine, said before it
/// is read: its text alone needs more memory than the machine has. `None`
/// when it fits, or when its size or the machine's memory cannot be read.
pub(crate) fn refusal_before_reading(path: &Path, messages: MessageCatalog) -> Option<String> {
    let deck_bytes = std::fs::metadata(path).ok()?.len();
    let memory = machine_memory()?;
    let needed = cost(deck_bytes, TEXT_BYTES_PER_DECK_BYTE);
    (needed > memory.installed_bytes).then(|| {
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        messages.format(
            MessageId::NetlistDeckTooLarge,
            &[
                ("name", &name),
                ("size", &memory_words(deck_bytes)),
                ("needed", &memory_words(needed)),
                ("installed", &memory_words(memory.installed_bytes)),
            ],
        )
    })
}

/// An open deck whose language features are off: its indexes need more
/// memory than the machine had free when it opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageOff {
    /// The deck.
    pub deck: DeckId,
    /// The bytes of its files as it opened.
    pub deck_bytes: u64,
    /// What its indexes need.
    pub needed: u64,
    /// What the machine had free.
    pub free: u64,
}

impl LanguageOff {
    /// The deck `deck`, whose files hold `deck_bytes`, with its language
    /// features off when its indexes need more memory than the machine has
    /// free: `None` when they fit, or the machine cannot say.
    pub(crate) fn for_deck(deck: DeckId, deck_bytes: u64) -> Option<Self> {
        let memory = machine_memory()?;
        let needed = cost(deck_bytes, INDEX_BYTES_PER_DECK_BYTE);
        (needed > memory.free_bytes).then_some(Self {
            deck,
            deck_bytes,
            needed,
            free: memory.free_bytes,
        })
    }

    /// `deck` as it opens, with its language features off when its indexes
    /// need more memory than the machine has free.
    pub(crate) fn as_opened(deck: &super::deck_files::OpenDeck) -> Option<Self> {
        let deck_bytes = std::iter::once(&deck.root)
            .chain(&deck.includes)
            .map(|file| file.text.len() as u64)
            .sum();
        Self::for_deck(deck.id, deck_bytes)
    }

    /// What the deck's band says, of the deck named `name`.
    #[must_use]
    pub fn words(&self, name: &str, messages: MessageCatalog) -> String {
        messages.format(
            MessageId::NetlistDeckLanguageOff,
            &[
                ("name", name),
                ("needed", &memory_words(self.needed)),
                ("free", &memory_words(self.free)),
            ],
        )
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
