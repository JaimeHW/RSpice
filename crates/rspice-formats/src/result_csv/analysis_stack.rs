//! Long-form CSV preserves each selected analysis and its independent coordinate grid.

use crate::table::{escape_csv_field as csv_text, sanitize_column_label};
use rspice_app_types::product::DatasetId;
use rspice_results::analysis_result::AnalysisResult;

#[derive(Debug)]
pub enum AnalysisStackCsvError {
    SampleCountMismatch {
        trace: String,
        component: String,
        coordinates: usize,
        samples: usize,
    },
    NonFiniteSample {
        trace: String,
        component: String,
        sample_index: usize,
    },
}

impl std::fmt::Display for AnalysisStackCsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SampleCountMismatch {
                trace,
                component,
                coordinates,
                samples,
            } => write!(
                f,
                "Displayed trace '{}' has {} coordinates and {} {component} samples; export refused instead of truncating evidence.",
                sanitize_column_label(trace),
                coordinates,
                samples,
            ),
            Self::NonFiniteSample {
                trace,
                component,
                sample_index,
            } => write!(
                f,
                "Displayed trace {} contains a non-finite {component} sample at index {sample_index}.",
                sanitize_column_label(trace),
            ),
        }
    }
}

impl std::error::Error for AnalysisStackCsvError {}

/// Accumulates components in caller order without aligning their coordinate grids.
#[derive(Debug)]
pub struct AnalysisStackCsv {
    contents: String,
    rows: usize,
}

impl Default for AnalysisStackCsv {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisStackCsv {
    pub fn new() -> Self {
        Self {
            contents: String::from(
                "dataset_id,analysis_sequence,analysis_label,analysis_type,trace,component,sample_index,x,y\n",
            ),
            rows: 0,
        }
    }

    /// Append exact samples. On refusal, earlier rows remain in the buffer;
    /// the caller must discard the document instead of publishing a partial export.
    pub fn append_component<W>(
        &mut self,
        dataset_id: DatasetId,
        analysis: &AnalysisResult<W>,
        trace: &str,
        component: &str,
        x: &[f64],
        y: &[f64],
    ) -> Result<(), AnalysisStackCsvError> {
        if x.len() != y.len() {
            return Err(AnalysisStackCsvError::SampleCountMismatch {
                trace: trace.to_owned(),
                component: component.to_owned(),
                coordinates: x.len(),
                samples: y.len(),
            });
        }
        let dataset = dataset_id.to_string();
        let label = csv_text(&analysis.label);
        let analysis_type = csv_text(analysis.analysis_type.short_label());
        let trace = csv_text(trace);
        for (sample_index, (&x, &y)) in x.iter().zip(y).enumerate() {
            if !x.is_finite() || !y.is_finite() {
                return Err(AnalysisStackCsvError::NonFiniteSample {
                    trace,
                    component: component.to_owned(),
                    sample_index,
                });
            }
            self.contents.push_str(&format!(
                "{dataset},{},{label},{analysis_type},{trace},{component},{sample_index},{x:.17e},{y:.17e}\n",
                analysis.id,
            ));
            self.rows += 1;
        }
        Ok(())
    }

    pub const fn row_count(&self) -> usize {
        self.rows
    }

    pub fn into_string(self) -> String {
        self.contents
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::analysis_type::AnalysisType;

    #[test]
    fn independent_grids_and_component_errors_preserve_exact_rows_and_diagnostics() {
        let dataset_id: DatasetId = "d24e84c0-0000-4000-8000-000000000001".parse().unwrap();
        let first: AnalysisResult =
            AnalysisResult::new(1, AnalysisType::Transient, "A,\"one\"", 0.0);
        let second: AnalysisResult = AnalysisResult::new(2, AnalysisType::Transient, "B", 0.0);
        let mut csv = AnalysisStackCsv::new();
        csv.append_component(dataset_id, &first, "V(a)", "display", &[0.0], &[-0.0])
            .unwrap();
        csv.append_component(
            dataset_id,
            &second,
            "V(b)",
            "imaginary",
            &[10.0, 20.0],
            &[3.0, 4.0],
        )
        .unwrap();
        assert_eq!(csv.row_count(), 3);
        let text = csv.into_string();
        let rows = csv::Reader::from_reader(text.as_bytes())
            .records()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(&rows[0][2], "A,\"one\"");
        assert_eq!(&rows[0][8], "-0.00000000000000000e0");
        assert_eq!(&rows[1][1], "2");
        assert_eq!(&rows[1][5], "imaginary");
        assert_eq!(&rows[1][7], "1.00000000000000000e1");
        assert_eq!(&rows[2][7], "2.00000000000000000e1");

        let mut csv = AnalysisStackCsv::new();
        let error = csv
            .append_component(dataset_id, &first, " V(a,\nb) ", "real", &[f64::NAN], &[])
            .unwrap_err();
        assert!(matches!(
            error,
            AnalysisStackCsvError::SampleCountMismatch { .. }
        ));
        assert_eq!(
            error.to_string(),
            "Displayed trace 'V(a b)' has 1 coordinates and 0 real samples; export refused instead of truncating evidence."
        );
        assert_eq!(csv.row_count(), 0);
        let error = csv
            .append_component(
                dataset_id,
                &first,
                "V(a,b)",
                "real",
                &[0.0, 1.0],
                &[2.0, f64::INFINITY],
            )
            .unwrap_err();
        assert!(matches!(
            error,
            AnalysisStackCsvError::NonFiniteSample {
                sample_index: 1,
                ..
            }
        ));
        assert_eq!(
            error.to_string(),
            "Displayed trace \"V(a b)\" contains a non-finite real sample at index 1."
        );
        assert_eq!(csv.row_count(), 1);
    }
}
