//! For tests of work that asks the reader service for stored series: a stored column held in
//! memory that counts the rows read from it, and a service that answers nothing until it is let
//! go, as the app's answers a read only once its worker has made it.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::{ColumnSource, Read, ReadService, ReadValue, StoredWindow, WaveformValues, WindowLead};

/// A stored column whose rows are in memory.
pub(crate) struct Column {
    values: Vec<f64>,
    /// The rows read from it so far.
    pub(crate) rows_read: AtomicU64,
}

impl ColumnSource for Column {
    fn name(&self) -> &str {
        "column"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.rows_read.fetch_add(out.len() as u64, Ordering::SeqCst);
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }
}

/// `values` as a stored column, and the column, to count what is read of it.
pub(crate) fn stored(values: &[f64]) -> (Arc<Column>, WaveformValues) {
    let column = Arc::new(Column {
        values: values.to_vec(),
        rows_read: AtomicU64::new(0),
    });
    let window = StoredWindow {
        first: 0,
        rows: values.len() as u64,
        lead: WindowLead::None,
    };
    let stored = WaveformValues::stored(column.clone(), window).expect("the window fits");
    (column, stored)
}

/// A reader service that queues every read until [`Gate::open`], and then makes each as it is
/// asked.
pub(crate) struct Gate {
    open: AtomicBool,
}

impl Gate {
    pub(crate) const fn closed() -> Self {
        Self {
            open: AtomicBool::new(false),
        }
    }

    /// Let every read through from now on.
    pub(crate) fn open(&self) {
        self.open.store(true, Ordering::SeqCst);
    }
}

impl ReadService for Gate {
    fn request(
        &self,
        _run: u64,
        values: &WaveformValues,
        x: &WaveformValues,
        read: Read,
    ) -> Option<ReadValue> {
        self.open
            .load(Ordering::SeqCst)
            .then(|| super::perform(values, x, read))
    }
}
