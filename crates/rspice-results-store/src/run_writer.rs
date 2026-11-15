//! The writer a run uses: rows are staged on the solve thread and encoded on a writer thread.
//!
//! Two chunk buffers circulate. The solve thread fills one while the writer thread encodes the
//! other; when the solve thread fills its buffer before the writer is done, it waits for the free
//! one, so memory stays two chunks whatever the run's length. Event changes travel the same way in
//! two buffers of 4,096 changes. The writer thread applies everything in the order it was pushed,
//! so the bytes are those [`SegmentWriter`] writes for the same pushes.
//!
//! About once a second of wall clock the writer thread asks for a durability flush; the next push
//! copies the open chunk into a third buffer and the writer writes it provisionally. Flushes never
//! change the final bytes.
//!
//! wasm32 has no threads: there the same API encodes inline, and the caller flushes with
//! [`RunWriter::sync`].

#[cfg(target_arch = "wasm32")]
use crate::SegmentWriter;
use crate::{EventChange, RunMeta, RunStatus, Schema, SegmentIo, StoreError, StoreReceipt};

/// What a [`RunWriter`] can write to: on native targets the I/O moves to the writer thread.
#[cfg(not(target_arch = "wasm32"))]
pub trait RunIo: SegmentIo + Send + 'static {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: SegmentIo + Send + 'static> RunIo for T {}

/// What a [`RunWriter`] can write to.
#[cfg(target_arch = "wasm32")]
pub trait RunIo: SegmentIo {}
#[cfg(target_arch = "wasm32")]
impl<T: SegmentIo> RunIo for T {}

/// Writes one run's segment; see the module documentation.
pub struct RunWriter<I: RunIo> {
    #[cfg(not(target_arch = "wasm32"))]
    inner: threaded::Threaded<I>,
    #[cfg(target_arch = "wasm32")]
    inner: SegmentWriter<I>,
}

impl<I: RunIo> RunWriter<I> {
    /// Start a segment on `io` (emptied first) for rows of `schema`.
    pub fn create(io: I, schema: &Schema, meta: &RunMeta) -> Result<Self, StoreError> {
        #[cfg(not(target_arch = "wasm32"))]
        let inner = threaded::Threaded::create(
            io,
            schema,
            meta,
            crate::format::chunk_rows(schema.slots()),
            threaded::SYNC_INTERVAL,
        )?;
        #[cfg(target_arch = "wasm32")]
        let inner = SegmentWriter::create(io, schema, meta)?;
        Ok(Self { inner })
    }

    #[cfg(all(test, not(target_arch = "wasm32")))]
    pub(crate) fn with_options(
        io: I,
        schema: &Schema,
        meta: &RunMeta,
        chunk_rows: usize,
        sync_interval: std::time::Duration,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            inner: threaded::Threaded::create(io, schema, meta, chunk_rows, sync_interval)?,
        })
    }

    /// Rows accepted so far.
    pub fn rows(&self) -> u64 {
        self.inner.rows()
    }

    /// Stage one row: x and the schema's value slots in order. Blocks only when the writer thread
    /// is still encoding the previous chunk.
    #[inline]
    pub fn push_row(&mut self, x: f64, values: &[f64]) -> Result<(), StoreError> {
        self.inner.push_row(x, values)
    }

    /// Stage event changes in commit order; a change the schema does not admit refuses the whole
    /// call.
    pub fn push_events(&mut self, changes: &[EventChange]) -> Result<(), StoreError> {
        self.inner.push_events(changes)
    }

    /// Overwrite one value of a row still in the open chunk.
    pub fn amend_open_row(&mut self, row: u64, slot: usize, value: f64) -> Result<(), StoreError> {
        self.inner.amend_open_row(row, slot, value)
    }

    /// Make every row and change pushed so far durable. On native targets the writer thread does
    /// it after the chunks already handed to it.
    pub fn sync(&mut self) -> Result<(), StoreError> {
        self.inner.sync()
    }

    /// Write the last chunk and `END` with `status`; returns the receipt and the segment's I/O.
    pub fn finish(self, status: RunStatus) -> Result<(StoreReceipt, I), StoreError> {
        self.inner.finish(status)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod threaded {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, TryRecvError, sync_channel};
    use std::thread::JoinHandle;
    use std::time::{Duration, Instant};

    use crate::events::EVENTS_PER_FRAME;
    use crate::writer::{Batch, Encoder, Layout};
    use crate::{EventChange, RunMeta, RunStatus, Schema, SegmentIo, StoreError, StoreReceipt};

    /// Wall clock between durability flushes while a run goes.
    pub(super) const SYNC_INTERVAL: Duration = Duration::from_secs(1);

    enum Message {
        Events(Vec<EventChange>),
        Chunk(Batch),
        Sync(Batch),
        Finish(Batch, RunStatus),
    }

    type Outcome<I> = Result<(StoreReceipt, I), StoreError>;

    /// The writer thread's ends of the channels.
    struct Thread<I: SegmentIo> {
        encoder: Encoder<I>,
        work: Receiver<Message>,
        free_rows: SyncSender<Batch>,
        free_events: SyncSender<Vec<EventChange>>,
        free_snapshot: SyncSender<Batch>,
        sync_due: Arc<AtomicBool>,
        interval: Duration,
    }

    impl<I: SegmentIo> Thread<I> {
        fn run(mut self) -> Outcome<I> {
            let mut last_sync = Instant::now();
            loop {
                let message = if self.sync_due.load(Ordering::Relaxed) {
                    self.work.recv().map_err(|_| RecvTimeoutError::Disconnected)
                } else {
                    self.work
                        .recv_timeout(self.interval.saturating_sub(last_sync.elapsed()))
                };
                match message {
                    Ok(Message::Events(changes)) => {
                        self.encoder.add_events(&changes)?;
                        let _ = self.free_events.send(changes);
                    }
                    Ok(Message::Chunk(batch)) => {
                        self.encoder.write_batch(&batch)?;
                        let _ = self.free_rows.send(batch);
                    }
                    Ok(Message::Sync(batch)) => {
                        self.encoder.write_provisional(&batch)?;
                        let _ = self.free_snapshot.send(batch);
                        self.sync_due.store(false, Ordering::Relaxed);
                        last_sync = Instant::now();
                    }
                    Ok(Message::Finish(batch, status)) => {
                        self.encoder.write_batch(&batch)?;
                        return self.encoder.finish(&status);
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => return Err(StoreError::WriterStopped),
                }
                if !self.sync_due.load(Ordering::Relaxed) && last_sync.elapsed() >= self.interval {
                    self.sync_due.store(true, Ordering::Relaxed);
                }
            }
        }
    }

    /// The solve thread's side.
    pub(super) struct Threaded<I: SegmentIo> {
        open: Batch,
        events: Vec<EventChange>,
        layout: Layout,
        work: Option<SyncSender<Message>>,
        free_rows: Receiver<Batch>,
        free_events: Receiver<Vec<EventChange>>,
        free_snapshot: Receiver<Batch>,
        sync_due: Arc<AtomicBool>,
        thread: Option<JoinHandle<Outcome<I>>>,
        stopped: bool,
    }

    impl<I: SegmentIo + Send + 'static> Threaded<I> {
        pub(super) fn create(
            io: I,
            schema: &Schema,
            meta: &RunMeta,
            chunk_rows: usize,
            interval: Duration,
        ) -> Result<Self, StoreError> {
            let encoder = Encoder::create(io, schema, meta, chunk_rows, true)?;
            let layout = encoder.layout().clone();
            let slots = layout.slots;
            let (work_tx, work) = sync_channel(2);
            let (rows_tx, free_rows) = sync_channel(2);
            let (events_tx, free_events) = sync_channel(2);
            let (snapshot_tx, free_snapshot) = sync_channel(1);
            let _ = rows_tx.send(Batch::new(chunk_rows, slots));
            let _ = events_tx.send(Vec::with_capacity(EVENTS_PER_FRAME));
            let _ = snapshot_tx.send(Batch::new(chunk_rows, slots));
            let sync_due = Arc::new(AtomicBool::new(false));
            let thread = Thread {
                encoder,
                work,
                free_rows: rows_tx,
                free_events: events_tx,
                free_snapshot: snapshot_tx,
                sync_due: Arc::clone(&sync_due),
                interval,
            };
            let thread = std::thread::Builder::new()
                .name("results store writer".into())
                .spawn(move || thread.run())?;
            Ok(Self {
                open: Batch::new(chunk_rows, slots),
                events: Vec::with_capacity(EVENTS_PER_FRAME),
                layout,
                work: Some(work_tx),
                free_rows,
                free_events,
                free_snapshot,
                sync_due,
                thread: Some(thread),
                stopped: false,
            })
        }

        pub(super) fn rows(&self) -> u64 {
            self.open.first_row + self.open.rows as u64
        }

        /// Stop the writer thread and wait for its outcome.
        fn join(&mut self) -> Outcome<I> {
            self.work = None;
            match self.thread.take().map(JoinHandle::join) {
                Some(Ok(outcome)) => outcome,
                Some(Err(_)) => Err(StoreError::Io {
                    kind: std::io::ErrorKind::Other,
                    message: "the results store writer thread panicked".into(),
                }),
                None => Err(StoreError::WriterStopped),
            }
        }

        /// The writer thread stopped: collect its error and refuse everything after.
        fn writer_error(&mut self) -> StoreError {
            self.stopped = true;
            self.join().err().unwrap_or(StoreError::WriterStopped)
        }

        fn send(&mut self, message: Message) -> Result<(), StoreError> {
            match self.work.as_ref().map(|work| work.send(message)) {
                Some(Ok(())) => Ok(()),
                _ => Err(self.writer_error()),
            }
        }

        fn check(&self) -> Result<(), StoreError> {
            if self.stopped {
                Err(StoreError::WriterStopped)
            } else {
                Ok(())
            }
        }

        #[inline]
        pub(super) fn push_row(&mut self, x: f64, values: &[f64]) -> Result<(), StoreError> {
            if values.len() != self.open.slots || self.stopped {
                self.check()?;
                return Err(StoreError::RowWidth {
                    expected: self.open.slots,
                    found: values.len(),
                });
            }
            self.open.push(x, values);
            if self.open.rows == self.open.x.len() {
                self.hand_off_rows()
            } else if self.sync_due.load(Ordering::Relaxed) {
                self.timed_sync()
            } else {
                Ok(())
            }
        }

        /// Hand the full open chunk (after the changes staged before it) to the writer thread
        /// and take the free buffer back, waiting for it if the writer is behind.
        fn hand_off_rows(&mut self) -> Result<(), StoreError> {
            self.hand_off_events()?;
            let Ok(mut free) = self.free_rows.recv() else {
                return Err(self.writer_error());
            };
            free.reset(self.rows());
            let full = std::mem::replace(&mut self.open, free);
            self.send(Message::Chunk(full))
        }

        fn hand_off_events(&mut self) -> Result<(), StoreError> {
            if self.events.is_empty() {
                return Ok(());
            }
            let Ok(mut free) = self.free_events.recv() else {
                return Err(self.writer_error());
            };
            free.clear();
            let staged = std::mem::replace(&mut self.events, free);
            self.send(Message::Events(staged))
        }

        pub(super) fn push_events(&mut self, changes: &[EventChange]) -> Result<(), StoreError> {
            self.check()?;
            for change in changes {
                self.layout.event_position(change)?;
            }
            for change in changes {
                self.events.push(change.clone());
                if self.events.len() == EVENTS_PER_FRAME {
                    self.hand_off_events()?;
                }
            }
            if self.sync_due.load(Ordering::Relaxed) {
                self.timed_sync()?;
            }
            Ok(())
        }

        pub(super) fn amend_open_row(
            &mut self,
            row: u64,
            slot: usize,
            value: f64,
        ) -> Result<(), StoreError> {
            let open = &mut self.open;
            if row < open.first_row
                || row >= open.first_row + open.rows as u64
                || slot >= open.slots
            {
                return Err(StoreError::RowNotOpen { row, slot });
            }
            open.values[(row - open.first_row) as usize * open.slots + slot] = value;
            Ok(())
        }

        pub(super) fn sync(&mut self) -> Result<(), StoreError> {
            self.check()?;
            let Ok(snapshot) = self.free_snapshot.recv() else {
                return Err(self.writer_error());
            };
            self.send_sync(snapshot)
        }

        /// The flush the writer thread asked for, unless one is already on its way.
        fn timed_sync(&mut self) -> Result<(), StoreError> {
            match self.free_snapshot.try_recv() {
                Ok(snapshot) => self.send_sync(snapshot),
                Err(TryRecvError::Empty) => Ok(()),
                Err(TryRecvError::Disconnected) => Err(self.writer_error()),
            }
        }

        /// Hand the staged changes over, then a copy of the open chunk to write provisionally.
        fn send_sync(&mut self, mut snapshot: Batch) -> Result<(), StoreError> {
            self.hand_off_events()?;
            snapshot.copy_from(&self.open);
            self.send(Message::Sync(snapshot))
        }

        pub(super) fn finish(mut self, status: RunStatus) -> Result<(StoreReceipt, I), StoreError> {
            self.check()?;
            self.hand_off_events()?;
            let last = std::mem::replace(&mut self.open, Batch::new(0, 0));
            self.send(Message::Finish(last, status))?;
            self.join()
        }
    }

    impl<I: SegmentIo> Drop for Threaded<I> {
        /// An unfinished writer stops its thread, which leaves the segment without `END`.
        fn drop(&mut self) {
            self.work = None;
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }
}
