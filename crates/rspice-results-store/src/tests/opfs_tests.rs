//! A segment and a manifest through an origin-private file's synchronous access handle: here a
//! double over bytes in memory, which moves a few bytes a call, as the browser's may move fewer
//! than asked, and can be cut off mid-write, as a closed tab cuts a write short.

use std::cell::{Cell, RefCell};
use std::io;
use std::rc::Rc;

use crate::manifest::segment_file_name;
use crate::opfs::{OpfsIo, SyncAccessHandle, load_manifest_from, save_manifest_to};
use crate::tests::support::{meta, mixed_schema, rows};
use crate::{Manifest, Member, MemoryIo, RunStatus, SegmentIo, SegmentReader, SegmentWriter};

/// A file, which outlives the handles opened on it.
#[derive(Default)]
struct File {
    bytes: RefCell<Vec<u8>>,
    flushes: Cell<usize>,
    open: Cell<usize>,
}

/// The bytes a tab may still write, to any file, before it is closed: `None` while it stays open.
type Tab = Rc<Cell<Option<usize>>>;

/// A synchronous access handle on `file`, moving at most `step` bytes a call.
struct Handle {
    file: Rc<File>,
    tab: Tab,
    step: usize,
    closed: Cell<bool>,
}

impl Handle {
    fn open(file: &Rc<File>, tab: &Tab, step: usize) -> OpfsIo<Self> {
        file.open.set(file.open.get() + 1);
        OpfsIo::new(Self {
            file: file.clone(),
            tab: tab.clone(),
            step,
            closed: Cell::new(false),
        })
    }
}

impl SyncAccessHandle for Handle {
    fn read(&self, buf: &mut [u8], at: u64) -> io::Result<usize> {
        let bytes = self.file.bytes.borrow();
        let held = bytes.get(at as usize..).unwrap_or_default();
        let read = buf.len().min(held.len()).min(self.step);
        buf[..read].copy_from_slice(&held[..read]);
        Ok(read)
    }

    fn write(&self, data: &[u8], at: u64) -> io::Result<usize> {
        let mut written = data.len().min(self.step);
        if let Some(left) = self.tab.get() {
            if left == 0 {
                return Err(io::Error::other("the tab was closed"));
            }
            written = written.min(left);
            self.tab.set(Some(left - written));
        }
        let at = at as usize;
        let mut bytes = self.file.bytes.borrow_mut();
        if bytes.len() < at + written {
            bytes.resize(at + written, 0);
        }
        bytes[at..at + written].copy_from_slice(&data[..written]);
        Ok(written)
    }

    fn truncate(&self, size: u64) -> io::Result<()> {
        self.file.bytes.borrow_mut().resize(size as usize, 0);
        Ok(())
    }

    fn flush(&self) -> io::Result<()> {
        self.file.flushes.set(self.file.flushes.get() + 1);
        Ok(())
    }

    fn size(&self) -> io::Result<u64> {
        Ok(self.file.bytes.borrow().len() as u64)
    }

    fn close(&self) {
        assert!(!self.closed.replace(true), "a handle is closed once");
        self.file.open.set(self.file.open.get() - 1);
    }
}

/// A segment of 300 rows, synced once partway, written to `io`.
fn written<I: SegmentIo>(io: I) -> (crate::StoreReceipt, I) {
    let schema = mixed_schema();
    let slots = schema.slots();
    let (x, values) = rows(300, slots, 5);
    let mut writer = SegmentWriter::with_chunk_rows(io, &schema, &meta(), 64).unwrap();
    writer.push_rows(&x[..150], &values[..150 * slots]).unwrap();
    writer.sync().unwrap();
    writer.push_rows(&x[150..], &values[150 * slots..]).unwrap();
    writer.finish(RunStatus::Completed).unwrap()
}

/// A segment written through a synchronous access handle that moves a few bytes a call is the
/// segment memory holds, reads back through it, and lets go of its file with its I/O.
#[test]
fn a_segment_through_a_sync_access_handle_is_the_one_memory_holds() {
    let (file, tab) = (Rc::new(File::default()), Tab::default());
    let (receipt, io) = written(Handle::open(&file, &tab, 7));
    let (expected, memory) = written(MemoryIo::new());
    assert_eq!(receipt.digest, expected.digest);
    assert!(*file.bytes.borrow() == memory.bytes(), "the same bytes");
    assert!(
        file.flushes.get() > 0,
        "the writer's syncs reach the handle"
    );
    let reader = SegmentReader::open(io).unwrap();
    assert_eq!(reader.rows(), 300);
    assert_eq!(reader.digest().unwrap(), expected.digest);
    drop(reader);
    assert_eq!(file.open.get(), 0, "the handle is closed with its I/O");
    let io = Handle::open(&file, &tab, 7);
    let size = io.size().unwrap();
    let mut past = [0u8; 16];
    assert_eq!(
        io.read_at(size - 8, &mut past).unwrap_err().kind(),
        io::ErrorKind::UnexpectedEof,
        "a read past the end fails as the segment being shorter"
    );
}

fn manifest(members: u32) -> Manifest {
    let mut manifest = Manifest::default();
    for index in 0..members {
        manifest.upsert(Member {
            index,
            segment: format!("runs/{}", segment_file_name(index)),
            analysis: ".tran 1n 10u".into(),
            ..Member::default()
        });
    }
    manifest
}

/// A manifest save cut short after any number of bytes, as a closed tab cuts it, leaves the
/// manifest before it or the one it saves, never neither; a first save leaves none or its own.
#[test]
fn a_manifest_save_cut_short_anywhere_keeps_the_old_manifest_or_the_new() {
    let (old, new) = (manifest(1), manifest(2));
    let whole = 2 * new.encode().len();
    for first in [true, false] {
        for budget in 0..=whole {
            let (main, staged, tab) = (
                Rc::new(File::default()),
                Rc::new(File::default()),
                Tab::default(),
            );
            let open = |file: &Rc<File>| Handle::open(file, &tab, 5);
            if !first {
                save_manifest_to(&mut open(&staged), &mut open(&main), &old).unwrap();
            }
            tab.set(Some(budget));
            let saved = save_manifest_to(&mut open(&staged), &mut open(&main), &new);
            tab.set(None);
            let loaded = load_manifest_from(Some(&open(&main)), Some(&open(&staged))).unwrap();
            let before = (!first).then(|| old.clone());
            assert!(
                loaded == before || loaded.as_ref() == Some(&new),
                "first save {first}, cut after {budget} bytes: {loaded:?}"
            );
            if saved.is_ok() {
                assert_eq!(loaded.as_ref(), Some(&new), "a save that finished is kept");
            }
            assert_eq!(main.open.get() + staged.open.get(), 0);
        }
    }
    let tab = Tab::default();
    let damaged = Rc::new(File::default());
    damaged
        .bytes
        .borrow_mut()
        .extend_from_slice(b"not a manifest");
    assert!(
        load_manifest_from(Some(&Handle::open(&damaged, &tab, 5)), None::<&MemoryIo>).is_err(),
        "a damaged manifest with no staged copy is refused"
    );
}
