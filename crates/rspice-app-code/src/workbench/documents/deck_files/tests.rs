//! A deck's files read again from disk on machines of every size: a file
//! changed on disk is weighed as an open weighs it, by the memory a test
//! says its machine has, never by the memory it has.

use std::path::{Path, PathBuf};

use rspice_simulation::execution::preparation::MachineMemory;

use super::*;
use crate::workbench::code::tests::{SourceHarness, harness};
use crate::workbench::documents::deck_memory::assume_machine_memory_here;
use crate::workbench::documents::netlist_document::edit_deck_file;

const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * MB;

const DECK: &str = "* weighed\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";

/// A folder of its own for one test's deck files, gone when it ends.
struct Folder(PathBuf);

impl Folder {
    fn new(label: &str) -> Self {
        let path = crate::fixture_root::canonical_temp_dir()
            .join(format!("rspice-deck-files-{label}-{}", std::process::id()));
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

/// From now on, this test's machine has `installed` bytes, `free` of them
/// free.
fn machine(installed: u64, free: u64) {
    assume_machine_memory_here(Some(MachineMemory {
        installed_bytes: installed,
        free_bytes: free,
    }));
}

/// The deck at `path`, open.
fn opened(path: &Path) -> SourceHarness {
    let mut harness = harness();
    let messages = harness.wb.messages();
    open_deck(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        path,
        messages,
    )
    .expect("the deck opens");
    harness
}

/// Look at the disk, as the page does when it is shown.
fn look(harness: &mut SourceHarness) -> Vec<String> {
    let messages = harness.wb.messages();
    check_disk(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.disk_looks,
        messages,
    )
}

fn root(harness: &SourceHarness) -> &OpenFile {
    &harness
        .state
        .netlist
        .open_deck
        .as_ref()
        .expect("a deck is open")
        .root
}

/// A file grown on disk past what the machine can hold is not read again,
/// as an open would not read it: the look says why, in the open's words,
/// the tab keeps its text, and Save does not write over the file it could
/// not read.
#[test]
fn a_file_grown_on_disk_past_the_machines_memory_is_not_read_again() {
    let folder = Folder::new("grown");
    let path = folder.write("grown.cir", DECK.as_bytes());
    machine(64 * GB, 32 * GB);
    let mut harness = opened(&path);
    let messages = harness.wb.messages();

    let mut grown = DECK.as_bytes().to_vec();
    grown.resize(10 * MB as usize, b'*');
    std::fs::write(&path, &grown).expect("another writer");
    machine(16 * MB, 16 * MB);
    let refusal = "grown.cir is 10 MB. Opening it needs about 22 MB of memory, more than \
                   the 16 MB this computer has. Open it on a computer with more memory.";

    assert_eq!(look(&mut harness), vec![refusal.to_owned()]);
    assert_eq!(root(&harness).text, DECK);
    assert!(!root(&harness).dirty);
    assert!(root(&harness).disk_change.is_none());

    let file = &mut harness
        .state
        .netlist
        .open_deck
        .as_mut()
        .expect("a deck is open")
        .root;
    assert_eq!(save_file(file, messages), Err(refusal.to_owned()));
    assert_eq!(
        std::fs::metadata(&path).expect("the file").len(),
        10 * MB,
        "the file on disk is as the other writer left it"
    );
}

/// A file read again from disk weighs the deck as an open weighs it: its
/// language features go off when its indexes need more memory than is
/// free, and come back once they fit, whether the tab held no edits or the
/// reader chose Reload over theirs.
#[test]
fn a_file_read_again_from_disk_weighs_the_deck_as_an_open_does() {
    let folder = Folder::new("weighed");
    let path = folder.write("weighed.cir", DECK.as_bytes());
    machine(64 * GB, 32 * GB);
    let mut harness = opened(&path);
    let messages = harness.wb.messages();
    let id = harness.state.netlist.open_deck.as_ref().expect("open").id;
    assert_eq!(harness.state.netlist.language_off(), None);

    let longer = format!("{DECK}* written again\n");
    std::fs::write(&path, &longer).expect("another writer");
    machine(64 * GB, 100);
    assert_eq!(
        look(&mut harness),
        vec!["Reloaded weighed.cir: changed on disk.".to_owned()]
    );
    assert_eq!(root(&harness).text, longer.as_str());
    assert_eq!(
        harness
            .state
            .netlist
            .language_off()
            .map(|off| (off.deck, off.deck_bytes, off.free)),
        Some((id, longer.len() as u64, 100))
    );

    assert!(edit_deck_file(
        &mut harness.model,
        &mut harness.state.netlist,
        format!("{longer}* mine\n")
    ));
    let again = format!("{DECK}* written again, and again\n");
    std::fs::write(&path, &again).expect("another writer");
    machine(64 * GB, 32 * GB);
    assert_eq!(look(&mut harness), Vec::<String>::new());
    assert!(root(&harness).disk_change.is_some(), "the tab holds edits");
    assert!(harness.state.netlist.language_off().is_some());

    reload_from_disk(
        &mut harness.model,
        &mut harness.state.netlist,
        None,
        messages,
    )
    .expect("the file reloads");
    assert_eq!(root(&harness).text, again.as_str());
    assert_eq!(harness.state.netlist.language_off(), None);
}
