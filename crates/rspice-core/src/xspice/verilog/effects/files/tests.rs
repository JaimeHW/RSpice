//! Resolution, confinement, snapshot reads, commit-time writes and checkpoint
//! offset truncation.

use super::*;

/// A fresh directory of this test's own under the system temporary
/// directory.
fn scratch(name: &str) -> PathBuf {
    let directory =
        std::env::temp_dir().join(format!("rspice-hdl-files-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

fn options(directory: &Path, target: HdlFileTarget) -> HdlFileOptions {
    HdlFileOptions::new(Some(directory), false, Arc::new(HdlFileStore::new(target)))
}

fn read_all(file: &mut OpenFile) -> Vec<u8> {
    let mut bytes = Vec::new();
    while let Some(byte) = file.next_byte() {
        bytes.push(byte);
    }
    bytes
}

#[test]
fn relative_names_resolve_against_the_run_directory() {
    let run = scratch("resolve");
    let options = options(&run, HdlFileTarget::Memory);
    assert_eq!(
        options.resolve_read("data/in.txt").unwrap(),
        run.join("data").join("in.txt")
    );
    assert_eq!(
        options.resolve_read("./a/../b.txt").unwrap(),
        run.join("b.txt")
    );
    // Reads may name anything, as `.include` may.
    let outside = run.parent().unwrap().join("elsewhere.txt");
    assert_eq!(
        options.resolve_read(outside.to_str().unwrap()).unwrap(),
        outside
    );
    assert_eq!(
        options.resolve_read("../x.txt").unwrap(),
        run.parent().unwrap().join("x.txt")
    );
    assert!(options.resolve_read("").is_err());
    // A relative run directory is the working directory's.
    let relative = HdlFileOptions::new(
        Some(Path::new("out")),
        false,
        Arc::new(HdlFileStore::new(HdlFileTarget::Memory)),
    );
    assert_eq!(
        relative.run_directory,
        normalize(&std::env::current_dir().unwrap().join("out"))
    );
}

#[test]
fn writes_stay_inside_the_run_directory() {
    let run = scratch("confine");
    let confined = options(&run, HdlFileTarget::Memory);
    assert_eq!(
        confined.resolve_write("sub/out.txt").unwrap(),
        run.join("sub").join("out.txt")
    );
    assert_eq!(
        confined.resolve_write("sub/../out.txt").unwrap(),
        run.join("out.txt")
    );
    let inside = run.join("abs.txt");
    assert_eq!(
        confined.resolve_write(inside.to_str().unwrap()).unwrap(),
        inside
    );
    for escape in [
        "../out.txt".to_owned(),
        "sub/../../out.txt".to_owned(),
        run.parent()
            .unwrap()
            .join("abs.txt")
            .to_string_lossy()
            .into_owned(),
        ".".to_owned(),
    ] {
        let error = confined.resolve_write(&escape).unwrap_err();
        assert!(
            error.contains("outside the run directory"),
            "{escape}: {error}"
        );
        assert!(error.contains("--hdl-unrestricted-io"), "{error}");
    }
    let mut files = HdlFiles::default();
    assert_eq!(
        files.open(&confined, "../out.txt", Some("w")),
        Err(OpenFailure::Warn(
            confined.resolve_write("../out.txt").unwrap_err()
        ))
    );
    let unrestricted = HdlFileOptions {
        unrestricted: true,
        ..confined
    };
    assert_eq!(
        unrestricted.resolve_write("../out.txt").unwrap(),
        run.parent().unwrap().join("out.txt")
    );
}

/// Section 17.2.1's numbering, as Icarus numbers: file descriptors from
/// 0x8000_0003, lowest free first; multichannel bits from 1.
#[test]
fn descriptors_are_numbered_lowest_free_first() {
    let run = scratch("numbers");
    let options = options(&run, HdlFileTarget::Memory);
    let mut files = HdlFiles::default();
    let first = files.open(&options, "a.txt", Some("w")).unwrap();
    let second = files.open(&options, "b.txt", Some("w")).unwrap();
    assert_eq!((first, second), (0x8000_0003, 0x8000_0004));
    files.close(first).unwrap();
    assert_eq!(files.open(&options, "c.txt", Some("w")).unwrap(), first);
    assert_eq!(files.open(&options, "d.txt", None).unwrap(), 2);
    assert_eq!(files.open(&options, "e.txt", None).unwrap(), 4);
    assert_eq!(
        files.close(0x8000_000f),
        Err(invalid_descriptor(0x8000_000f))
    );
    assert_eq!(
        files.write_targets(&options, 0x4000_0000),
        Err(invalid_channel(0x4000_0000))
    );
    assert_eq!(
        files.write_targets(&options, 1 | 2 | 4).unwrap(),
        [
            WriteTarget::Stdout,
            WriteTarget::Channel(1),
            WriteTarget::Channel(2)
        ]
    );
    assert_eq!(
        files.open(&options, "missing.txt", Some("r")),
        Err(OpenFailure::Quiet)
    );
    assert_eq!(
        files.open(&options, "no-such-directory/x.txt", Some("w")),
        Err(OpenFailure::Quiet)
    );
    assert!(matches!(
        files.open(&options, "a.txt", Some("q")),
        Err(OpenFailure::Warn(_))
    ));
}

/// A read-mode `$fopen` reads the file once: later changes on disk do not
/// reach the handle, so a retried trial reads what the first attempt read.
#[test]
fn a_read_handle_reads_the_snapshot_taken_at_open() {
    let run = scratch("snapshot");
    std::fs::write(run.join("in.txt"), "abc").unwrap();
    let options = options(&run, HdlFileTarget::Disk);
    let mut files = HdlFiles::default();
    let descriptor = files.open(&options, "in.txt", Some("r")).unwrap();
    std::fs::write(run.join("in.txt"), "changed").unwrap();
    let file = files.reader(descriptor).unwrap();
    assert_eq!(file.next_byte(), Some(b'a'));
    file.unget(b'q');
    assert_eq!(file.tell(), 0);
    assert_eq!(read_all(file), b"qbc");
    assert!(file.at_eof());
    assert!(file.seek(-1, 2));
    assert_eq!(read_all(file), b"c");
}

/// No write reaches a file before its trial commits, and a rolled-back
/// trial — the host restoring the clone it took when the trial opened —
/// writes nothing.
#[test]
fn writes_land_at_commit_and_a_rolled_back_trial_writes_nothing() {
    let run = scratch("commit");
    let options = options(&run, HdlFileTarget::Disk);
    let mut files = HdlFiles::default();
    let descriptor = files.open(&options, "log.txt", Some("w")).unwrap();
    let target = files.write_targets(&options, descriptor).unwrap().remove(0);
    files.write_file(&target, b"one\n");
    assert!(!run.join("log.txt").exists(), "nothing before the commit");
    assert!(files.release(&options).is_empty());
    assert_eq!(std::fs::read(run.join("log.txt")).unwrap(), b"one\n");

    let accepted = files.clone();
    files.write_file(&target, b"rejected\n");
    files.close(descriptor).unwrap();
    files = accepted;
    assert!(files.release(&options).is_empty());
    assert_eq!(std::fs::read(run.join("log.txt")).unwrap(), b"one\n");

    files.write_file(&target, b"two\n");
    files.close(descriptor).unwrap();
    // Read back before the commit: the journal is visible to the design.
    let again = files.open(&options, "log.txt", Some("r")).unwrap();
    assert_eq!(read_all(files.reader(again).unwrap()), b"one\ntwo\n");
    assert!(files.release(&options).is_empty());
    assert_eq!(std::fs::read(run.join("log.txt")).unwrap(), b"one\ntwo\n");
}

#[test]
fn a_memory_store_keeps_writes_off_the_disk() {
    let run = scratch("memory");
    let options = options(&run, HdlFileTarget::Memory);
    let mut files = HdlFiles::default();
    let descriptor = files.open(&options, "log.txt", Some("w")).unwrap();
    let target = files.write_targets(&options, descriptor).unwrap().remove(0);
    files.write_file(&target, b"kept\n");
    files.close(descriptor).unwrap();
    files.release(&options);
    assert!(!run.join("log.txt").exists());
    assert_eq!(
        options.store.written(&run.join("log.txt")).as_deref(),
        Some(&b"kept\n"[..])
    );
    let again = files.open(&options, "log.txt", Some("a+")).unwrap();
    let target = files.write_targets(&options, again).unwrap().remove(0);
    files.write_file(&target, b"more\n");
    assert!(files.reader(again).unwrap().seek(0, 0));
    assert_eq!(read_all(files.reader(again).unwrap()), b"kept\nmore\n");
}

/// A resumed checkpoint truncates each written file to what it held when the
/// checkpoint was taken, so output written after it is discarded.
#[test]
fn a_restored_checkpoint_truncates_writes_after_it() {
    let run = scratch("checkpoint");
    let options = options(&run, HdlFileTarget::Disk);
    let mut files = HdlFiles::default();
    let descriptor = files.open(&options, "log.txt", Some("w")).unwrap();
    let channel = files.open(&options, "mcd.txt", None).unwrap();
    let target = files.write_targets(&options, descriptor).unwrap().remove(0);
    files.write_file(&target, b"one\n");
    files.write_file(
        &WriteTarget::Channel(channel.trailing_zeros() as usize),
        b"m1\n",
    );
    files.release(&options);
    let image = files.capture().unwrap();
    files.write_file(&target, b"after\n");
    files.release(&options);
    assert_eq!(std::fs::read(run.join("log.txt")).unwrap(), b"one\nafter\n");

    let mut resumed = HdlFiles::default();
    resumed.restore(&options, &image).unwrap();
    resumed.write_file(&target, b"two\n");
    resumed.release(&options);
    assert_eq!(std::fs::read(run.join("log.txt")).unwrap(), b"one\ntwo\n");
    assert_eq!(std::fs::read(run.join("mcd.txt")).unwrap(), b"m1\n");
    assert_eq!(resumed.write_targets(&options, channel).unwrap().len(), 1);
}

#[test]
fn a_restored_read_handle_must_find_the_file_it_was_reading() {
    let run = scratch("checkpoint-read");
    std::fs::write(run.join("in.txt"), "abcdef").unwrap();
    let options = options(&run, HdlFileTarget::Disk);
    let mut files = HdlFiles::default();
    let descriptor = files.open(&options, "in.txt", Some("r")).unwrap();
    files.reader(descriptor).unwrap().next_byte();
    let image = files.capture().unwrap();
    let mut resumed = HdlFiles::default();
    resumed.restore(&options, &image).unwrap();
    assert_eq!(read_all(resumed.reader(descriptor).unwrap()), b"bcdef");
    std::fs::write(run.join("in.txt"), "abcdeX").unwrap();
    let error = HdlFiles::default().restore(&options, &image).unwrap_err();
    assert!(error.contains("changed since it was saved"), "{error}");
    // A checkpoint is taken at a released boundary.
    let written = files.open(&options, "w.txt", Some("w")).unwrap();
    let target = files.write_targets(&options, written).unwrap().remove(0);
    files.write_file(&target, b"x");
    assert!(files.capture().is_err());
}

/// LRM 2.4 section 9.5.1.2: the tables of one analysis hand out distinct
/// numbers, and a write through a number another table opened for writing
/// lands in its file, after what that table wrote and before what it writes
/// next. A rejected trial's retried open gets the number the first attempt
/// got.
#[test]
fn tables_of_one_analysis_share_their_descriptors() {
    let run = scratch("shared");
    let options = options(&run, HdlFileTarget::Disk);
    let mut digital = HdlFiles::default();
    let mut analog = HdlFiles::for_instance("x1");
    let fd = digital.open(&options, "log.txt", Some("w")).unwrap();
    let mcd = digital.open(&options, "mcd.txt", None).unwrap();
    let saved = analog.clone();
    let rejected = analog.open(&options, "own.txt", Some("w")).unwrap();
    analog = saved;
    let own = analog.open(&options, "own.txt", Some("w")).unwrap();
    assert_eq!((fd, mcd, own), (0x8000_0003, 2, 0x8000_0004));
    assert_eq!(rejected, own);
    let target = digital.write_targets(&options, fd).unwrap().remove(0);
    digital.write_file(&target, b"digital 1\n");
    digital.release(&options);
    for descriptor in [fd, mcd | 1] {
        let targets = analog.write_targets(&options, descriptor).unwrap();
        for target in targets
            .iter()
            .filter(|target| **target != WriteTarget::Stdout)
        {
            assert!(matches!(target, WriteTarget::Shared { .. }), "{target:?}");
            analog.write_file(target, b"analog\n");
        }
    }
    analog.release(&options);
    digital.write_file(&target, b"digital 2\n");
    digital.release(&options);
    let read = |file: &str| std::fs::read_to_string(run.join(file)).unwrap();
    assert_eq!(read("log.txt"), "digital 1\nanalog\ndigital 2\n");
    assert_eq!(read("mcd.txt"), "analog\n");
    // A read-only descriptor is not shared.
    let input = digital.open(&options, "log.txt", Some("r")).unwrap();
    assert_eq!(
        analog.write_targets(&options, input),
        Err(invalid_descriptor(input))
    );
}
