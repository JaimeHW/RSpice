//! How a deck opens on machines of every size: each outcome is reached by
//! the memory a test says its machine has, never by the memory it has.

use std::path::{Path, PathBuf};

use super::*;
use crate::workbench::code::tests::{SourceHarness, harness};
use crate::workbench::documents::deck_files;
use crate::workbench::documents::netlist_document::{language, visible_source_index_in};

const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * MB;

const DECK: &str = "* memory\nV1 in 0 1\nR1 in out {r}\n.param r=1k\nX1 out cell\n\
.subckt cell a\nR2 a 0 1k\n.ends cell\n.op\n.end\n";

/// A folder of its own for one test's deck files, gone when it ends.
struct Folder(PathBuf);

impl Folder {
    fn new(label: &str) -> Self {
        let path = crate::fixture_root::canonical_temp_dir()
            .join(format!("rspice-deck-memory-{label}-{}", std::process::id()));
        std::fs::create_dir_all(&path).expect("the folder");
        Self(path)
    }

    fn write(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, bytes).expect("the deck file");
        path
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn open(harness: &mut SourceHarness, path: &Path) -> Result<(), String> {
    let messages = harness.wb.messages();
    deck_files::open_deck(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        path,
        messages,
    )
}

/// Whether the outline and the project index of the deck in front are there.
fn indexed(harness: &mut SourceHarness) -> (bool, bool) {
    let ctx = egui::Context::default();
    let outline = visible_source_index_in(&ctx, &harness.model, &mut harness.state.netlist);
    let project = language::project_index_in(&ctx, &harness.model, &mut harness.state.netlist);
    (outline.is_some(), project.is_some())
}

/// Go to the definition of `r` in `{r}`.
fn go_to_r(harness: &mut SourceHarness) -> Result<String, String> {
    let caret = DECK.find("{r}").expect("the read") + 1;
    harness.state.netlist.cursor_char_index = caret;
    language::go_to_definition_at_cursor_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        &mut harness.wb,
    )
}

/// A deck file whose text alone needs more memory than the machine has is
/// refused before it is read, in the words a run's refusal says memory in.
/// The file is not text: read, it would be refused for that instead.
#[test]
fn a_deck_file_the_machine_cannot_hold_is_refused_before_it_is_read() {
    let folder = Folder::new("refused");
    let mut garbled = vec![0xef, 0xbb, 0xbf, 0xc3, 0x28];
    garbled.resize(10 * MB as usize, b'a');
    let path = folder.write("garbled.cir", &garbled);
    let mut harness = harness();

    assume_machine_memory_here(Some(MachineMemory {
        installed_bytes: 16 * MB,
        free_bytes: 16 * MB,
    }));
    assert_eq!(
        open(&mut harness, &path),
        Err(
            "garbled.cir is 10 MB. Opening it needs about 22 MB of memory, more than the 16 MB \
             this computer has. Open it on a computer with more memory."
                .to_owned()
        )
    );
    assert!(harness.state.netlist.open_deck.is_none());
    assert!(harness.model.workspace.content.decks.is_empty());

    assume_machine_memory_here(Some(MachineMemory {
        installed_bytes: 64 * GB,
        free_bytes: 32 * GB,
    }));
    let unreadable = open(&mut harness, &path).expect_err("the file is not text");
    assert!(
        !unreadable.contains("memory"),
        "on a machine that holds it the file is read, and refused as no text: {unreadable}"
    );
}

/// A deck whose indexes need more memory than the machine has free opens
/// with its outline, hover, go to definition, completion and check off, and
/// says so with the numbers, until the reader indexes it anyway.
#[test]
fn a_deck_whose_indexes_do_not_fit_opens_with_its_language_features_off() {
    let folder = Folder::new("off");
    let path = folder.write("memory.cir", DECK.as_bytes());
    let mut harness = harness();
    let messages = harness.wb.messages();
    assume_machine_memory_here(Some(MachineMemory {
        installed_bytes: 64 * GB,
        free_bytes: 100,
    }));

    open(&mut harness, &path).expect("the deck opens");
    let id = harness.state.netlist.open_deck.as_ref().expect("open").id;
    let deck_bytes = DECK.len() as u64;
    assert_eq!(
        harness.state.netlist.language_off(),
        Some(LanguageOff {
            deck: id,
            deck_bytes,
            needed: (deck_bytes as f64 * INDEX_BYTES_PER_DECK_BYTE).ceil() as u64,
            free: 100,
        })
    );
    assert_eq!(indexed(&mut harness), (false, false));
    assert_eq!(
        go_to_r(&mut harness),
        Err(messages.text(MessageId::NetlistDeckLanguageOffShort))
    );

    harness.state.netlist.index_anyway();
    assert_eq!(harness.state.netlist.language_off(), None);
    assert_eq!(indexed(&mut harness), (true, true));
    assert!(go_to_r(&mut harness).is_ok(), "the definition is found");
}

/// A deck whose indexes fit in the memory free opens with them.
#[test]
fn a_deck_the_machine_holds_opens_with_its_language_features_on() {
    let folder = Folder::new("on");
    let path = folder.write("memory.cir", DECK.as_bytes());
    let mut harness = harness();
    assume_machine_memory_here(Some(MachineMemory {
        installed_bytes: 64 * GB,
        free_bytes: 32 * GB,
    }));

    open(&mut harness, &path).expect("the deck opens");
    assert_eq!(harness.state.netlist.language_off(), None);
    assert_eq!(indexed(&mut harness), (true, true));
    assert!(go_to_r(&mut harness).is_ok(), "the definition is found");
}

/// The band says what indexing the deck needs and what the machine had
/// free, in the words memory is said in.
#[test]
fn the_band_says_what_indexing_needs_and_what_was_free() {
    let harness = harness();
    let messages = harness.wb.messages();
    assume_machine_memory_here(Some(MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: 200 * MB,
    }));
    let off = LanguageOff::for_deck(DeckId::new(), 100 * MB).expect("off");
    assert_eq!(
        off.words("big.cir", messages),
        format!(
            "Language features are off: indexing big.cir needs about {}, and 200 MB was free.",
            memory_words(off.needed)
        )
    );
    assume_machine_memory_here(Some(MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: 8 * GB,
    }));
    assert_eq!(LanguageOff::for_deck(DeckId::new(), 100 * MB), None);
}
