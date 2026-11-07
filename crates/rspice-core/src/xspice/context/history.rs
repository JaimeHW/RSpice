//! The samples a code-model instance keeps under one history key.

use super::TransientHistorySample;
use std::collections::VecDeque;
use std::sync::Arc;

/// Samples in each shared chunk.
const CHUNK: usize = 32;

/// Samples in time order, kept as shared full chunks and a short owned tail.
///
/// A copy of the context, as a trial's or an acceptance's rollback image
/// takes, shares every full chunk, so a write after the copy duplicates at
/// most a chunk's worth of samples rather than the whole history.
#[derive(Debug, Clone, Default)]
pub(super) struct TransientHistory {
    /// Full chunks of exactly [`CHUNK`] samples, oldest first.
    chunks: VecDeque<Arc<[TransientHistorySample]>>,
    /// Samples dropped from the front of the first chunk; zero without one.
    skip: usize,
    /// The newest samples, at most [`CHUNK`].
    tail: Vec<TransientHistorySample>,
}

impl TransientHistory {
    fn shared_len(&self) -> usize {
        self.chunks.len() * CHUNK - self.skip
    }

    pub(super) fn len(&self) -> usize {
        self.shared_len() + self.tail.len()
    }

    pub(super) fn get(&self, index: usize) -> Option<&TransientHistorySample> {
        let shared = self.shared_len();
        if index < shared {
            let at = index + self.skip;
            Some(&self.chunks[at / CHUNK][at % CHUNK])
        } else {
            self.tail.get(index - shared)
        }
    }

    pub(super) fn last(&self) -> Option<&TransientHistorySample> {
        self.len().checked_sub(1).and_then(|index| self.get(index))
    }

    pub(super) fn last_mut(&mut self) -> Option<&mut TransientHistorySample> {
        self.own_newest_chunk();
        self.tail.last_mut()
    }

    pub(super) fn pop(&mut self) -> Option<TransientHistorySample> {
        self.own_newest_chunk();
        self.tail.pop()
    }

    pub(super) fn push(&mut self, sample: TransientHistorySample) {
        self.tail.push(sample);
        if self.tail.len() > CHUNK {
            let newest = self.tail.split_off(CHUNK);
            let full = std::mem::replace(&mut self.tail, newest);
            self.chunks.push_back(Arc::from(full));
        }
    }

    /// How many samples lead the history for which `pred` holds, `pred`
    /// holding on a prefix of it.
    pub(super) fn partition_point(
        &self,
        mut pred: impl FnMut(&TransientHistorySample) -> bool,
    ) -> usize {
        let (mut low, mut high) = (0, self.len());
        while low < high {
            let middle = low + (high - low) / 2;
            if self.get(middle).is_some_and(&mut pred) {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        low
    }

    /// Drop the `count` oldest samples.
    pub(super) fn drop_oldest(&mut self, mut count: usize) {
        while count > 0 && !self.chunks.is_empty() {
            let available = CHUNK - self.skip;
            if count < available {
                self.skip += count;
                return;
            }
            self.chunks.pop_front();
            self.skip = 0;
            count -= available;
        }
        self.tail.drain(..count.min(self.tail.len()));
    }

    #[cfg(any(feature = "veriloga", test))]
    pub(super) fn iter(&self) -> impl Iterator<Item = &TransientHistorySample> {
        self.chunks
            .iter()
            .flat_map(|chunk| chunk.iter())
            .skip(self.skip)
            .chain(&self.tail)
    }

    /// With the tail empty, make the newest shared chunk the tail, so the
    /// newest sample can change.
    fn own_newest_chunk(&mut self) {
        if !self.tail.is_empty() {
            return;
        }
        let Some(chunk) = self.chunks.pop_back() else {
            return;
        };
        let skip = if self.chunks.is_empty() {
            std::mem::take(&mut self.skip)
        } else {
            0
        };
        self.tail = chunk[skip..].to_vec();
    }
}

#[cfg(any(feature = "veriloga", test))]
impl FromIterator<TransientHistorySample> for TransientHistory {
    fn from_iter<I: IntoIterator<Item = TransientHistorySample>>(samples: I) -> Self {
        let mut history = Self::default();
        for sample in samples {
            history.push(sample);
        }
        history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(time: f64) -> TransientHistorySample {
        TransientHistorySample {
            time,
            values: vec![time],
        }
    }

    fn times(history: &TransientHistory) -> Vec<f64> {
        history.iter().map(|sample| sample.time).collect()
    }

    #[test]
    fn a_history_reads_like_the_vector_it_replaces() {
        let mut history = TransientHistory::default();
        let mut reference: Vec<f64> = Vec::new();
        for step in 0..5 * CHUNK {
            let time = step as f64;
            history.push(sample(time));
            reference.push(time);
            if step % 7 == 3 {
                history.drop_oldest(2);
                reference.drain(..2);
            }
            if step % 11 == 5 {
                history.pop();
                reference.pop();
            }
            if step % 13 == 6 {
                history.last_mut().expect("a sample").time += 0.5;
                *reference.last_mut().expect("a sample") += 0.5;
            }
            assert_eq!(times(&history), reference);
            assert_eq!(history.len(), reference.len());
            for (index, time) in reference.iter().enumerate() {
                assert_eq!(history.get(index).map(|sample| sample.time), Some(*time));
            }
            assert_eq!(
                history.last().map(|sample| sample.time),
                reference.last().copied()
            );
            let cut = time - 10.0;
            assert_eq!(
                history.partition_point(|sample| sample.time < cut),
                reference.partition_point(|&time| time < cut)
            );
        }
        let rebuilt: TransientHistory = history.iter().cloned().collect();
        assert_eq!(times(&rebuilt), reference);
        history.drop_oldest(history.len());
        assert_eq!(history.len(), 0);
        assert!(history.last().is_none());
    }

    #[test]
    fn a_copy_shares_the_full_chunks() {
        let history: TransientHistory =
            (0..3 * CHUNK + 5).map(|step| sample(step as f64)).collect();
        let mut copy = history.clone();
        copy.push(sample(1e3));
        copy.last_mut().expect("a sample").time = 2e3;
        assert!(
            history
                .chunks
                .iter()
                .zip(&copy.chunks)
                .all(|(a, b)| Arc::ptr_eq(a, b))
        );
        assert_eq!(history.len(), 3 * CHUNK + 5);
        assert_eq!(copy.last().map(|sample| sample.time), Some(2e3));
    }
}
