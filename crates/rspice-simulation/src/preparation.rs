//! Typed preparation failures shared by source, model and dispatch checks.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreparationStage {
    DesignChecks,
    SourceChecks,
    AnalysisPlan,
    ModelBindings,
    Netlist,
    Authorization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparationError {
    stage: PreparationStage,
    message: String,
    /// The 1-based deck line this failure named, where it named one.
    line: Option<usize>,
}

impl PreparationError {
    pub fn new(stage: PreparationStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
            line: None,
        }
    }

    /// The same failure, at the line the parser reported it on.
    pub fn at_line(mut self, line: Option<usize>) -> Self {
        self.line = line;
        self
    }

    pub const fn stage(&self) -> PreparationStage {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn line(&self) -> Option<usize> {
        self.line
    }
}

impl std::fmt::Display for PreparationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}
