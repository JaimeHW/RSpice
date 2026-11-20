//! A store in a browser's origin-private file system.
//!
//! A worker reaches a file there through a synchronous access handle, which offers the positioned
//! calls [`SegmentIo`] makes (read and write at an offset, truncate, flush, size) and holds the
//! file exclusively while it is open: another handle on it, from this tab or another, is refused,
//! so a segment has one writer without a lock of its own. [`OpfsIo`] runs over any
//! [`SyncAccessHandle`]; on wasm32 the browser's own handle is one, and [`OpfsStoreDir`] lays a
//! store out in an origin-private directory as [`crate::StoreDir`] does on disk.
//!
//! The origin-private file system has no rename every browser offers, so a manifest is saved
//! twice: whole to a staged file first, then over the manifest. Whichever write a closed tab cuts
//! short, the other file is whole, and a load takes the manifest when it is whole and the staged
//! copy otherwise, so a save is kept whole or not at all.

use std::io;

use crate::{Manifest, SegmentIo, StoreError};

/// The calls of an origin-private file's synchronous access handle that a store makes. A read or
/// a write may move fewer bytes than asked, as the browser's may.
pub trait SyncAccessHandle {
    /// Read into `buf` from byte `at`: the bytes read, fewer at the end of the file.
    fn read(&self, buf: &mut [u8], at: u64) -> io::Result<usize>;
    /// Write `data` at byte `at`, growing the file: the bytes written.
    fn write(&self, data: &[u8], at: u64) -> io::Result<usize>;
    /// Cut the file to `size` bytes, or zero-extend it to them.
    fn truncate(&self, size: u64) -> io::Result<()>;
    /// Make every write so far durable.
    fn flush(&self) -> io::Result<()>;
    /// The file's size in bytes.
    fn size(&self) -> io::Result<u64>;
    /// Let go of the file, which another handle may then open.
    fn close(&self);
}

/// A segment (or a manifest) in an origin-private file, through its synchronous access handle,
/// which it closes when dropped.
pub struct OpfsIo<H: SyncAccessHandle> {
    handle: H,
}

impl<H: SyncAccessHandle> OpfsIo<H> {
    pub fn new(handle: H) -> Self {
        Self { handle }
    }
}

fn at(offset: u64, done: usize) -> io::Result<u64> {
    offset
        .checked_add(done as u64)
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))
}

impl<H: SyncAccessHandle> SegmentIo for OpfsIo<H> {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        let mut done = 0;
        while done < buf.len() {
            match self.handle.read(&mut buf[done..], at(offset, done)?)? {
                0 => return Err(io::ErrorKind::UnexpectedEof.into()),
                read => done += read.min(buf.len() - done),
            }
        }
        Ok(())
    }

    fn write_at(&mut self, offset: u64, data: &[u8]) -> io::Result<()> {
        let mut done = 0;
        while done < data.len() {
            match self.handle.write(&data[done..], at(offset, done)?)? {
                0 => return Err(io::ErrorKind::WriteZero.into()),
                written => done += written.min(data.len() - done),
            }
        }
        Ok(())
    }

    fn truncate(&mut self, len: u64) -> io::Result<()> {
        self.handle.truncate(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.handle.flush()
    }

    fn size(&self) -> io::Result<u64> {
        self.handle.size()
    }
}

impl<H: SyncAccessHandle> Drop for OpfsIo<H> {
    fn drop(&mut self) {
        self.handle.close();
    }
}

/// The manifest of a store whose manifest file and staged copy are `main` and `staged` (`None`
/// where there is no such file): the manifest when it is whole, else the staged copy, which a save
/// wrote whole before it began to overwrite the manifest. `None` before the first save finished
/// writing its staged copy. An empty manifest file is one a save made and had not yet written.
pub(crate) fn load_manifest_from(
    main: Option<&impl SegmentIo>,
    staged: Option<&impl SegmentIo>,
) -> Result<Option<Manifest>, StoreError> {
    fn read(io: &impl SegmentIo) -> Result<Option<Manifest>, StoreError> {
        let size = usize::try_from(io.size()?).map_err(|_| StoreError::MemoryFull)?;
        if size == 0 {
            return Ok(None);
        }
        let mut bytes = Vec::new();
        crate::error::reserve(&mut bytes, size, "a results store's manifest")?;
        bytes.resize(size, 0);
        io.read_at(0, &mut bytes)?;
        Manifest::decode(&bytes).map(Some)
    }
    let main = main.map_or(Ok(None), read);
    if let Ok(Some(manifest)) = main {
        return Ok(Some(manifest));
    }
    match staged.map_or(Ok(None), read) {
        Ok(Some(manifest)) => Ok(Some(manifest)),
        Ok(None) | Err(_) => main,
    }
}

/// Save `manifest` to `staged`, whole and durable, then over `main`; see the module documentation.
pub(crate) fn save_manifest_to(
    staged: &mut impl SegmentIo,
    main: &mut impl SegmentIo,
    manifest: &Manifest,
) -> Result<(), StoreError> {
    fn replace(io: &mut impl SegmentIo, bytes: &[u8]) -> Result<(), StoreError> {
        io.truncate(0)?;
        io.write_at(0, bytes)?;
        io.flush()?;
        Ok(())
    }
    let bytes = manifest.encode();
    replace(staged, &bytes)?;
    replace(main, &bytes)
}

#[cfg(target_arch = "wasm32")]
pub use browser::OpfsStoreDir;

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::io;

    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::{FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemSyncAccessHandle};

    use super::{OpfsIo, SyncAccessHandle};
    use crate::manifest::{MANIFEST, RUNS, segment_file_name};
    use crate::{Manifest, StoreError};

    /// Where a manifest save writes its staged copy, beside the manifest.
    const STAGED: &str = "manifest.rsm.staged";

    impl SyncAccessHandle for FileSystemSyncAccessHandle {
        fn read(&self, buf: &mut [u8], at: u64) -> io::Result<usize> {
            let options = web_sys::FileSystemReadWriteOptions::new();
            options.set_at(at as f64);
            let read = self
                .read_with_u8_array_and_options(buf, &options)
                .map_err(js_error)?;
            Ok(read as usize)
        }

        fn write(&self, data: &[u8], at: u64) -> io::Result<usize> {
            let options = web_sys::FileSystemReadWriteOptions::new();
            options.set_at(at as f64);
            let written = self
                .write_with_u8_array_and_options(data, &options)
                .map_err(js_error)?;
            Ok(written as usize)
        }

        fn truncate(&self, size: u64) -> io::Result<()> {
            self.truncate_with_f64(size as f64).map_err(js_error)
        }

        fn flush(&self) -> io::Result<()> {
            FileSystemSyncAccessHandle::flush(self).map_err(js_error)
        }

        fn size(&self) -> io::Result<u64> {
            Ok(self.get_size().map_err(js_error)? as u64)
        }

        fn close(&self) {
            FileSystemSyncAccessHandle::close(self);
        }
    }

    /// A browser error as an I/O error of the kind the store acts on: a full quota is a full
    /// volume, and a file another handle holds is busy.
    fn js_error(error: JsValue) -> io::Error {
        let Some(exception) = error.dyn_ref::<web_sys::DomException>() else {
            return io::Error::other(format!("{error:?}"));
        };
        let name = exception.name();
        let kind = match name.as_str() {
            "NotFoundError" => io::ErrorKind::NotFound,
            "QuotaExceededError" => io::ErrorKind::StorageFull,
            "NoModificationAllowedError" => io::ErrorKind::ResourceBusy,
            "NotAllowedError" | "SecurityError" => io::ErrorKind::PermissionDenied,
            _ => io::ErrorKind::Other,
        };
        io::Error::new(kind, format!("{name}: {}", exception.message()))
    }

    async fn resolved<T: JsCast>(promise: js_sys::Promise) -> io::Result<T> {
        wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(js_error)?
            .dyn_into::<T>()
            .map_err(|_| io::Error::other("the origin-private file system answered another type"))
    }

    async fn directory(
        parent: &FileSystemDirectoryHandle,
        name: &str,
        create: bool,
    ) -> io::Result<FileSystemDirectoryHandle> {
        let options = web_sys::FileSystemGetDirectoryOptions::new();
        options.set_create(create);
        resolved(parent.get_directory_handle_with_options(name, &options)).await
    }

    async fn file(
        parent: &FileSystemDirectoryHandle,
        name: &str,
        create: bool,
    ) -> io::Result<FileSystemFileHandle> {
        let options = web_sys::FileSystemGetFileOptions::new();
        options.set_create(create);
        resolved(parent.get_file_handle_with_options(name, &options)).await
    }

    async fn opened(
        parent: &FileSystemDirectoryHandle,
        name: &str,
        create: bool,
    ) -> io::Result<OpfsIo<FileSystemSyncAccessHandle>> {
        let file = file(parent, name, create).await?;
        resolved(file.create_sync_access_handle())
            .await
            .map(OpfsIo::new)
    }

    /// `name` in `parent` opened, or `None` when there is no such file.
    async fn existing(
        parent: &FileSystemDirectoryHandle,
        name: &str,
    ) -> io::Result<Option<OpfsIo<FileSystemSyncAccessHandle>>> {
        match opened(parent, name, false).await {
            Ok(io) => Ok(Some(io)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }

    /// A store directory in a browser's origin-private file system: a manifest and a `runs`
    /// folder of segments, named as a store on disk names them. Its files are opened through
    /// synchronous access handles, which browsers offer to dedicated workers only.
    #[derive(Debug, Clone)]
    pub struct OpfsStoreDir {
        root: FileSystemDirectoryHandle,
    }

    impl OpfsStoreDir {
        /// The store directory `name` in `parent`, made with its `runs` folder when `create`.
        pub async fn open(
            parent: &FileSystemDirectoryHandle,
            name: &str,
            create: bool,
        ) -> io::Result<Self> {
            let root = directory(parent, name, create).await?;
            if create {
                directory(&root, RUNS, true).await?;
            }
            Ok(Self { root })
        }

        /// The store directory whose handle is `root`.
        pub fn new(root: FileSystemDirectoryHandle) -> Self {
            Self { root }
        }

        pub fn root(&self) -> &FileSystemDirectoryHandle {
            &self.root
        }

        /// Segment `index`, made empty when `create` and there is none. It is held exclusively
        /// until the I/O is dropped: opening it again meanwhile fails as busy.
        pub async fn segment(
            &self,
            index: u32,
            create: bool,
        ) -> io::Result<OpfsIo<FileSystemSyncAccessHandle>> {
            let runs = directory(&self.root, RUNS, create).await?;
            opened(&runs, &segment_file_name(index), create).await
        }

        /// The manifest, or `None` before the first member finished.
        pub async fn load_manifest(&self) -> Result<Option<Manifest>, StoreError> {
            let main = existing(&self.root, MANIFEST).await?;
            let staged = existing(&self.root, STAGED).await?;
            super::load_manifest_from(main.as_ref(), staged.as_ref())
        }

        /// Replace the manifest, whole or not at all.
        pub async fn save_manifest(&self, manifest: &Manifest) -> Result<(), StoreError> {
            let mut staged = opened(&self.root, STAGED, true).await?;
            let mut main = opened(&self.root, MANIFEST, true).await?;
            super::save_manifest_to(&mut staged, &mut main, manifest)?;
            drop((staged, main));
            // The staged copy is the manifest now; a load ignores it while the manifest is whole.
            let _ = resolved::<JsValue>(self.root.remove_entry(STAGED)).await;
            Ok(())
        }
    }
}
