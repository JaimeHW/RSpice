//! Instrumentation for whole-dataset work done inside a Results frame.
//!
//! Immediate mode hides cost: a viewer that rebuilds a million-sample
//! projection every frame is indistinguishable, in the source, from one that
//! reads a memo. The only way to keep the difference honest is to count it,
//! so every place the Results workspace walks a complete retained dataset
//! reports the walk here before doing it.
//!
//! The counters exist only under `cfg(test)`; in a shipped build [`note`] is
//! an empty function and the call sites optimize away. What ships is the
//! discipline: a new whole-dataset walk on a Results surface is expected to
//! name itself in [`DatasetWalk`], and the idle-frame gate then holds it to
//! being memoized.

/// One class of work whose cost scales with the retained dataset rather than
/// with what the reader can see.
///
/// Ordinal values index the counter table, so variants are appended rather
/// than reordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum DatasetWalk {
    /// `AnalysisResult::validate_retained_evidence` — walks every retained
    /// sample of every waveform in one analysis.
    EvidenceValidation,
    /// `SimulationRun::dataset_content_digest` — hashes the complete ordered
    /// dataset of a run.
    DatasetDigest,
    /// The dataset-manifest view model: one projected row per retained task.
    ManifestViewModel,
    /// The operating-point row plan: filter, group and sort every retained
    /// node and device row.
    OpPlan,
    /// One SOA stress-history scan: locating and verifying the retained
    /// stress waveform behind a single rule.
    SoaStressScan,
    /// Ranking every retained sensitivity row.
    SensitivityRank,
    /// Revalidating the retained optimizer cost/variable history.
    OptimizationView,
    /// Monte-Carlo moment and consistency walks over a sample population.
    HistMoments,
    /// The shared X extent of a strip: every visible trace's coordinates.
    WaveXRange,
    /// Family-envelope construction: every sample of every family member.
    WaveEnvelope,
    /// Baking the eye density texture from every folded acquisition.
    EyeRaster,
    /// Mapping a cursor to a retained sample by scanning the sample grid.
    TableCursorScan,
}

impl DatasetWalk {
    /// Every variant, for reporting a complete count table.
    #[cfg(test)]
    pub(super) const ALL: [Self; 12] = [
        Self::EvidenceValidation,
        Self::DatasetDigest,
        Self::ManifestViewModel,
        Self::OpPlan,
        Self::SoaStressScan,
        Self::SensitivityRank,
        Self::OptimizationView,
        Self::HistMoments,
        Self::WaveXRange,
        Self::WaveEnvelope,
        Self::EyeRaster,
        Self::TableCursorScan,
    ];
}

/// Report that one whole-dataset walk is about to happen.
///
/// Call this at the point the work is actually performed — inside the memo
/// miss, never at the memo lookup — so the count answers "how much dataset
/// did this frame touch", not "how often was the answer wanted".
#[inline]
pub(super) fn note(_walk: DatasetWalk) {
    #[cfg(test)]
    COUNTS.with(|counts| counts.borrow_mut()[_walk as usize] += 1);
}

/// One class of per-frame painting work that reads retained samples.
///
/// [`DatasetWalk`] counts work that must not happen at all in a steady frame.
/// The paths named here are different: they legitimately touch the dataset on
/// every frame — the overview lane under a strip redraws its mini-trace
/// whatever the reader is doing — so the question is never whether they read
/// it, but how much of it. A walk counter cannot tell reading a hundred
/// samples from reading a million, because neither is a *complete* walk of a
/// multi-waveform dataset, and that is exactly where a per-frame cost
/// proportional to the whole run hid behind a clean idle-frame gate.
///
/// Ordinal values index the counter table, so variants are appended rather
/// than reordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum FrameSampleRead {
    /// The mini-trace drawn in a strip's shared-X overview lane.
    StripOverview,
    /// [`super::finite_extremes`]: one pass over a series for its bounds.
    TraceExtremes,
}

impl FrameSampleRead {
    /// Every variant, for reporting a complete count table.
    #[cfg(test)]
    pub(super) const ALL: [Self; 2] = [Self::StripOverview, Self::TraceExtremes];
}

/// Report how many retained samples a per-frame painting path is about to
/// read, at the point it reads them.
#[inline]
pub(super) fn note_samples(_read: FrameSampleRead, _samples: usize) {
    #[cfg(test)]
    SAMPLES.with(|samples| samples.borrow_mut()[_read as usize] += _samples as u64);
}

#[cfg(test)]
thread_local! {
    static COUNTS: std::cell::RefCell<[u64; DatasetWalk::ALL.len()]> =
        const { std::cell::RefCell::new([0; DatasetWalk::ALL.len()]) };
    static SAMPLES: std::cell::RefCell<[u64; FrameSampleRead::ALL.len()]> =
        const { std::cell::RefCell::new([0; FrameSampleRead::ALL.len()]) };
}

/// Counted whole-dataset work, as a snapshot that arithmetic can be done on.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct WorkCounts {
    walks: [u64; DatasetWalk::ALL.len()],
    samples: [u64; FrameSampleRead::ALL.len()],
}

#[cfg(test)]
impl WorkCounts {
    const ZERO: Self = Self {
        walks: [0; DatasetWalk::ALL.len()],
        samples: [0; FrameSampleRead::ALL.len()],
    };

    /// The counts accumulated so far on this thread.
    pub(super) fn read() -> Self {
        Self {
            walks: COUNTS.with(|counts| *counts.borrow()),
            samples: SAMPLES.with(|samples| *samples.borrow()),
        }
    }

    /// Reset the counters and return a zero snapshot to measure against.
    pub(super) fn reset() -> Self {
        COUNTS.with(|counts| *counts.borrow_mut() = Self::ZERO.walks);
        SAMPLES.with(|samples| *samples.borrow_mut() = Self::ZERO.samples);
        Self::ZERO
    }

    /// Work counted since `self` was taken.
    pub(super) fn since(self) -> Self {
        let now = Self::read();
        let mut delta = Self::ZERO;
        for (index, slot) in delta.walks.iter_mut().enumerate() {
            *slot = now.walks[index] - self.walks[index];
        }
        for (index, slot) in delta.samples.iter_mut().enumerate() {
            *slot = now.samples[index] - self.samples[index];
        }
        delta
    }

    /// Count for one class of work.
    pub(super) const fn get(self, walk: DatasetWalk) -> u64 {
        self.walks[walk as usize]
    }

    /// Retained samples read by one class of per-frame painting work.
    pub(super) const fn samples(self, read: FrameSampleRead) -> u64 {
        self.samples[read as usize]
    }

    /// Total whole-dataset walks across every class.
    pub(super) const fn total(self) -> u64 {
        let mut total = 0;
        let mut index = 0;
        while index < self.walks.len() {
            total += self.walks[index];
            index += 1;
        }
        total
    }

    /// Total retained samples read across every painting class.
    pub(super) const fn total_samples(self) -> u64 {
        let mut total = 0;
        let mut index = 0;
        while index < self.samples.len() {
            total += self.samples[index];
            index += 1;
        }
        total
    }

    /// The classes that counted anything, for a failure message that names
    /// the offending surface instead of only its total.
    pub(super) fn nonzero(self) -> Vec<(DatasetWalk, u64)> {
        DatasetWalk::ALL
            .into_iter()
            .filter(|walk| self.get(*walk) > 0)
            .map(|walk| (walk, self.get(walk)))
            .collect()
    }

    /// The painting classes that read anything, on the same terms.
    pub(super) fn nonzero_samples(self) -> Vec<(FrameSampleRead, u64)> {
        FrameSampleRead::ALL
            .into_iter()
            .filter(|read| self.samples(*read) > 0)
            .map(|read| (read, self.samples(read)))
            .collect()
    }
}

#[cfg(test)]
mod tests;
