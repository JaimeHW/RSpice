//! Cooperative qualification execution on the current runtime.
//!
//! Each step runs one vector. Cancellation discards partial work; solver and
//! measurement failures are retained as failed outcomes in a complete run.
//! Only the terminal step publishes a validated platform run.
//!
//! The desktop solves each vector in process. The browser posts it to the
//! simulation worker and collects the outcome on a later step, so the
//! interactive image links no solver for qualification.

use rspice_model_library::qualification::*;
use rspice_simulation::model_qualification::QualificationVectorError;

/// Observable suite progress. Emitting progress never publishes partial
/// qualification evidence; cancellation discards the in-progress run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationExecutionProgress {
    pub platform: QualificationPlatform,
    pub completed_vectors: usize,
    pub total_vectors: usize,
    pub current_vector_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum QualificationExecutionError {
    #[error(transparent)]
    InvalidContract(#[from] QualificationValidationError),
    #[error("qualification execution was cancelled; no partial platform run was published")]
    Cancelled,
    #[error("qualification execution session has already finished")]
    SessionFinished,
    #[error("the browser simulation worker could not run the vector: {0}")]
    Worker(String),
}

impl From<QualificationVectorError> for QualificationExecutionError {
    fn from(error: QualificationVectorError) -> Self {
        match error {
            QualificationVectorError::Cancelled => Self::Cancelled,
            QualificationVectorError::InvalidContract(error) => Self::InvalidContract(error),
        }
    }
}

/// Result of one cooperative session step. An in-progress step never exposes
/// partial evidence. The terminal variant contains the first publishable,
/// fully validated platform run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualificationExecutionStep {
    InProgress(QualificationExecutionProgress),
    Complete {
        progress: QualificationExecutionProgress,
        run: QualificationPlatformRun,
    },
}

/// Owned, frame-safe execution state. `step` executes at most one vector, so
/// callers can yield to egui or a browser event loop between vectors.
#[derive(Debug, Clone)]
pub struct QualificationExecutionSession {
    suite: QualificationSuite,
    source: ModelSourceEvidenceBinding,
    platform: QualificationPlatform,
    next_vector: usize,
    vector_outcomes: Vec<QualificationPlatformVectorOutcome>,
    cancelled: bool,
    finished: bool,
    /// The worker request solving the next vector.
    #[cfg(target_arch = "wasm32")]
    in_flight: Option<u32>,
}

impl QualificationExecutionSession {
    pub fn try_new(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
    ) -> Result<Self, QualificationExecutionError> {
        suite.validate_source_binding(source)?;
        Ok(Self {
            suite: suite.clone(),
            source: source.clone(),
            platform: current_qualification_platform(),
            next_vector: 0,
            vector_outcomes: Vec::with_capacity(suite.vectors.len()),
            cancelled: false,
            finished: false,
            #[cfg(target_arch = "wasm32")]
            in_flight: None,
        })
    }

    #[must_use]
    pub fn progress(&self) -> QualificationExecutionProgress {
        QualificationExecutionProgress {
            platform: self.platform,
            completed_vectors: self.next_vector,
            total_vectors: self.suite.vectors.len(),
            current_vector_id: self
                .suite
                .vectors
                .get(self.next_vector)
                .map(|value| value.id.clone()),
        }
    }

    /// Request cancellation before another vector begins. No partial run can
    /// be retrieved from a cancelled session.
    pub fn cancel(&mut self) {
        if !self.finished {
            self.cancelled = true;
            #[cfg(target_arch = "wasm32")]
            browser_worker::abandon(&mut self.in_flight);
        }
    }

    #[must_use]
    #[cfg(test)]
    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    #[must_use]
    #[cfg(test)]
    pub const fn is_finished(&self) -> bool {
        self.finished
    }

    /// Execute at most one vector and yield a progress or terminal record.
    pub fn step(
        &mut self,
        abort: &dyn rspice_core::AbortSignal,
    ) -> Result<QualificationExecutionStep, QualificationExecutionError> {
        if self.finished {
            return Err(QualificationExecutionError::SessionFinished);
        }
        if self.cancelled || abort.is_aborted() {
            self.cancelled = true;
            #[cfg(target_arch = "wasm32")]
            browser_worker::abandon(&mut self.in_flight);
            return Err(QualificationExecutionError::Cancelled);
        }
        let vector = self
            .suite
            .vectors
            .get(self.next_vector)
            .expect("a validated qualification suite is non-empty and unfinished");
        #[cfg(not(target_arch = "wasm32"))]
        let executed = rspice_simulation::model_qualification::execute_qualification_vector(
            vector,
            self.platform,
            abort,
        )
        .map_err(QualificationExecutionError::from);
        #[cfg(target_arch = "wasm32")]
        let Some(executed) = browser_worker::execute(&mut self.in_flight, vector, self.platform)?
        else {
            return Ok(QualificationExecutionStep::InProgress(self.progress()));
        };
        let outcome = match executed {
            Ok(value) => value,
            Err(QualificationExecutionError::Cancelled) => {
                self.cancelled = true;
                return Err(QualificationExecutionError::Cancelled);
            }
            Err(error) => return Err(error),
        };
        if abort.is_aborted() {
            self.cancelled = true;
            return Err(QualificationExecutionError::Cancelled);
        }
        self.vector_outcomes
            .push(QualificationPlatformVectorOutcome {
                vector_id: vector.id.clone(),
                input_digest: vector.input_digest,
                outcome,
            });
        self.next_vector += 1;
        let progress = self.progress();
        abort.observe_progress(
            progress.completed_vectors as f64 / progress.total_vectors.max(1) as f64,
        );
        if self.next_vector < self.suite.vectors.len() {
            return Ok(QualificationExecutionStep::InProgress(progress));
        }

        let run = QualificationPlatformRun::try_new(
            self.platform,
            self.source.clone(),
            &self.suite,
            self.vector_outcomes.clone(),
        )?;
        self.finished = true;
        #[cfg(target_arch = "wasm32")]
        rspice_simulation::model_qualification::browser::release_qualification_worker();
        Ok(QualificationExecutionStep::Complete { progress, run })
    }
}

/// The browser half of a step: the simulation worker solves the vector.
#[cfg(target_arch = "wasm32")]
mod browser_worker {
    use super::*;
    use rspice_simulation::model_qualification::browser;

    type VectorOutcome = Result<PlatformQualificationOutcome, QualificationExecutionError>;

    /// Post the vector, or collect its outcome once the worker answers;
    /// `None` while the worker is still solving it.
    pub(super) fn execute(
        in_flight: &mut Option<u32>,
        vector: &QualificationVector,
        platform: QualificationPlatform,
    ) -> Result<Option<VectorOutcome>, QualificationExecutionError> {
        let Some(request) = *in_flight else {
            *in_flight =
                Some(browser::post_qualification_vector(vector, platform).map_err(failed)?);
            return Ok(None);
        };
        let Some(answer) = browser::take_qualification_outcome(request) else {
            return Ok(None);
        };
        *in_flight = None;
        answer
            .map(|result| Some(result.map_err(Into::into)))
            .map_err(failed)
    }

    /// Stop the worker solving an abandoned vector.
    pub(super) fn abandon(in_flight: &mut Option<u32>) {
        if in_flight.take().is_some() {
            browser::release_qualification_worker();
        }
    }

    fn failed(error: String) -> QualificationExecutionError {
        browser::release_qualification_worker();
        QualificationExecutionError::Worker(error)
    }
}

#[cfg(target_arch = "wasm32")]
const fn current_qualification_platform() -> QualificationPlatform {
    QualificationPlatform::WebAssembly
}

#[cfg(not(target_arch = "wasm32"))]
const fn current_qualification_platform() -> QualificationPlatform {
    QualificationPlatform::Desktop
}

#[cfg(test)]
mod tests;
