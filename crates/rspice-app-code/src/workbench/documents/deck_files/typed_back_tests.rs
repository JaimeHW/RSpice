//! A deck file edited back to the text last read from or written to disk
//! reads as saved once the digest of that text is taken, and only then.

use super::*;

const READ: &str = "* typed back\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";

/// As long as [`READ`], and not it.
const OTHER: &str = "* typed back\nV1 in 0 1\nR1 in 0 2k\n.op\n.end\n";

/// The file of [`READ`], as read from disk.
fn read() -> OpenFile {
    let mut file = OpenFile::in_memory("typed.cir", READ);
    file.disk = Some(disk_stamp(&file.path, READ));
    file
}

/// Take the digest of the file's text as it stands, as the worker does.
fn take_digest(file: &mut OpenFile) {
    let digest = editor_text_digest(&file.text);
    file.take_digest(file.text.revision(), digest);
}

#[test]
fn a_file_typed_back_to_what_was_read_reads_as_saved_once_its_digest_is_taken() {
    let mut file = read();
    assert!(!file.wants_digest(), "a file as read holds no edits");

    let mut typed = file.text.clone();
    typed.replace_range(0..0, "x");
    assert!(file.take_typed(&typed));
    assert!(file.dirty);
    assert!(!file.wants_digest(), "longer than what was read");

    typed.replace_range(0..1, "");
    assert!(file.take_typed(&typed));
    assert!(
        file.dirty,
        "typed back, it holds edits until its digest is taken"
    );
    assert!(file.wants_digest());
    take_digest(&mut file);
    assert!(!file.dirty, "typed back, it reads as saved");
    assert!(!file.wants_digest());
}

#[test]
fn a_digest_of_another_text_or_of_another_revision_leaves_the_edits() {
    let mut file = read();
    assert!(file.edit(OTHER.to_owned()));
    assert!(file.wants_digest(), "as long as what was read");
    let other = file.text.revision();
    take_digest(&mut file);
    assert!(file.dirty, "another text as long holds edits");
    assert!(!file.wants_digest(), "a revision is read once");

    assert!(file.edit(READ.to_owned()));
    let digest = editor_text_digest(&file.text);
    file.take_digest(other, digest);
    assert!(file.dirty, "a digest taken for a revision before is let go");
    assert!(file.wants_digest());

    file.take_digest(file.text.revision(), digest);
    assert!(!file.dirty, "edited back, it reads as saved");

    assert!(file.edit(OTHER.to_owned()));
    assert!(file.dirty, "edited again, it holds edits again");
}
