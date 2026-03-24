use super::{AnalysisSpec, RunQueue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalysisPlan {
    /// Ordered analyses.
    pub analyses: Vec<AnalysisSpec>,
    /// Stop queue execution on first failed run.
    pub stop_on_error: bool,
}

impl AnalysisPlan {
    /// Create empty plan.
    pub fn new() -> Self {
        Self {
            analyses: Vec::new(),
            stop_on_error: true,
        }
    }

    /// Append an analysis.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, analysis: AnalysisSpec) -> Self {
        self.analyses.push(analysis);
        self
    }

    /// Append an analysis with an explicit builder-style name.
    pub fn with_analysis(mut self, analysis: AnalysisSpec) -> Self {
        self.analyses.push(analysis);
        self
    }

    /// Validate all analyses in order.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.analyses.is_empty() {
            errors.push("Analysis plan is empty".to_string());
        }
        for (idx, analysis) in self.analyses.iter().enumerate() {
            if let Err(e) = analysis.validate() {
                errors.push(format!("Analysis #{}: {}", idx + 1, e));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Build a run queue from this plan.
    pub fn into_queue(self) -> Result<RunQueue, Vec<String>> {
        self.validate()?;
        let mut queue = RunQueue::new();
        queue.stop_on_error = self.stop_on_error;
        for analysis in self.analyses {
            queue.add_analysis(analysis);
        }
        Ok(queue)
    }
}
