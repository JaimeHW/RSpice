use crate::properties::parse_engineering_value;
use std::fmt;

use super::data::{PwlData, PwlPoint, PwlValidationError, format_spice_number_lossless};

/// State for the PWL editor UI widget.
#[derive(Debug, Clone, Default)]
pub struct PwlEditorState {
    /// The PWL data being edited.
    pub data: PwlData,
    /// Selected row index (if any).
    pub selected_row: Option<usize>,
    /// Text buffers for editing (time, value).
    pub edit_buffers: Vec<(String, String)>,
    /// Unparsed imported source. This remains authoritative until the user
    /// repairs it, so opening the editor can never replace bad source with an
    /// empty, apparently valid waveform.
    pub(super) raw_source_draft: Option<String>,
    /// Validation error for the current data.
    pub validation_error: Option<String>,
    /// Whether the editor is in "add new point" mode.
    pub adding_point: bool,
    /// Buffer for new point time.
    pub new_time: String,
    /// Buffer for new point value.
    pub new_value: String,
    /// Unit for values (V or A).
    pub value_unit: String,
    /// Whether the data has been modified since opening.
    pub is_modified: bool,
}

impl PwlEditorState {
    /// Create new editor state.
    pub fn new() -> Self {
        Self {
            value_unit: "V".to_string(),
            ..Default::default()
        }
    }

    /// Initialize from PWL data string.
    pub fn from_string(s: &str, value_unit: &str) -> Self {
        match PwlData::parse(s) {
            Ok(data) => Self {
                data,
                edit_buffers: buffers_from_source(s),
                value_unit: value_unit.to_string(),
                ..Default::default()
            },
            Err(error) => Self {
                raw_source_draft: Some(s.to_owned()),
                validation_error: Some(error.to_string()),
                value_unit: value_unit.to_string(),
                ..Default::default()
            },
        }
    }

    /// Sync edit buffers from data.
    pub fn sync_buffers_from_data(&mut self) {
        self.edit_buffers = self
            .data
            .points()
            .iter()
            .map(|p| {
                (
                    format_spice_number_lossless(p.time),
                    format_spice_number_lossless(p.value),
                )
            })
            .collect();
    }

    /// Sync data from edit buffers.
    pub fn sync_data_from_buffers(&mut self) -> Result<(), PwlValidationError> {
        let mut new_points = Vec::with_capacity(self.edit_buffers.len());

        for (i, (time_str, value_str)) in self.edit_buffers.iter().enumerate() {
            let time = parse_engineering_value(time_str).map_err(|_| {
                PwlValidationError::TimeParseError {
                    index: i,
                    text: time_str.clone(),
                }
            })?;

            let value = parse_engineering_value(value_str).map_err(|_| {
                PwlValidationError::ValueParseError {
                    index: i,
                    text: value_str.clone(),
                }
            })?;

            new_points.push(PwlPoint::new(time, value));
        }

        let new_data = PwlData::with_ordered_points(new_points);
        new_data.validate()?;
        self.data = new_data;
        self.validation_error = None;
        Ok(())
    }

    /// Apply the current row buffers as a user-authored draft.
    ///
    /// Modification state is recorded even when parsing fails. The raw cell
    /// text is retained while the last valid waveform stays intact.
    pub fn apply_buffer_edits(&mut self) -> Result<(), PwlValidationError> {
        self.is_modified = true;
        match self.sync_data_from_buffers() {
            Ok(()) => Ok(()),
            Err(error) => {
                self.validation_error = Some(error.to_string());
                Err(error)
            }
        }
    }

    /// Return imported source that could not yet be represented as rows.
    pub fn raw_source_draft(&self) -> Option<&str> {
        self.raw_source_draft.as_deref()
    }

    /// Replace and revalidate an unparsed source draft.
    ///
    /// A successful repair atomically installs the parsed waveform and
    /// recreates row buffers from the repaired source. A failed repair keeps
    /// the complete text available for the next edit.
    pub fn replace_raw_source_draft(&mut self, source: String) -> Result<(), PwlValidationError> {
        self.is_modified = true;
        self.raw_source_draft = Some(source);
        self.repair_raw_source_draft()
    }

    fn repair_raw_source_draft(&mut self) -> Result<(), PwlValidationError> {
        let source = self.raw_source_draft.clone().unwrap_or_default();
        match PwlData::parse(&source) {
            Ok(data) => {
                self.data = data;
                self.edit_buffers = buffers_from_source(&source);
                self.raw_source_draft = None;
                self.validation_error = None;
                self.selected_row = None;
                Ok(())
            }
            Err(error) => {
                self.validation_error = Some(error.to_string());
                Err(error)
            }
        }
    }

    /// Serialize the current draft, including invalid source or cell text.
    pub fn draft_source(&self) -> String {
        if let Some(source) = &self.raw_source_draft {
            return source.clone();
        }
        if self.edit_buffers.is_empty() {
            return self.data.serialize();
        }
        self.edit_buffers
            .iter()
            .map(|(time, value)| format!("{time} {value}"))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Add a new point.
    pub fn add_point(&mut self) {
        if self.new_time.is_empty() {
            self.new_time = "0".to_string();
        }
        if self.new_value.is_empty() {
            self.new_value = "0".to_string();
        }

        let time = parse_engineering_value(&self.new_time).unwrap_or(0.0);
        let value = parse_engineering_value(&self.new_value).unwrap_or(0.0);

        self.data.add_point(PwlPoint::new(time, value));
        self.sync_buffers_from_data();
        self.is_modified = true;
        self.validation_error = self.data.validate().err().map(|error| error.to_string());
        self.new_time.clear();
        self.new_value.clear();
        self.adding_point = false;
    }

    /// Delete the selected point.
    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected_row {
            self.data.remove_point(idx);
            self.sync_buffers_from_data();
            self.is_modified = true;
            self.validation_error = self.data.validate().err().map(|error| error.to_string());
            self.selected_row = None;
        }
    }

    /// Check if data has been modified.
    pub fn is_valid(&self) -> bool {
        self.validation_error.is_none() && self.data.validate().is_ok()
    }
}

impl fmt::Display for PwlEditorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.draft_source())
    }
}

fn buffers_from_source(source: &str) -> Vec<(String, String)> {
    source
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|pair| (pair[0].to_owned(), pair[1].to_owned()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_import_retains_raw_source_until_a_valid_repair_resyncs_rows() {
        let invalid = "0 0 1n";
        let mut state = PwlEditorState::from_string(invalid, "V");

        assert_eq!(state.raw_source_draft(), Some(invalid));
        assert_eq!(state.to_string(), invalid);
        assert!(state.validation_error.is_some());
        assert!(!state.is_valid());

        state
            .replace_raw_source_draft("0 0 1n 1".to_owned())
            .unwrap();
        assert_eq!(state.raw_source_draft(), None);
        assert_eq!(state.data.len(), 2);
        assert_eq!(state.edit_buffers[1], ("1n".to_owned(), "1".to_owned()));
        assert!(state.is_valid());
    }

    #[test]
    fn invalid_row_edit_is_modified_and_retains_the_raw_cell_text() {
        let mut state = PwlEditorState::from_string("0 0 1n 1", "V");
        state.edit_buffers[1].1 = "1e".to_owned();

        assert!(state.apply_buffer_edits().is_err());
        assert!(state.is_modified);
        assert!(state.validation_error.is_some());
        assert_eq!(state.to_string(), "0 0 1n 1e");
        assert_eq!(state.data.points()[1].value.to_bits(), 1.0f64.to_bits());
    }

    #[test]
    fn editing_one_row_keeps_untouched_high_precision_points_bit_exact() {
        let precise_time = f64::from_bits(0x3ff0_0000_0000_0001);
        let precise_value = f64::from_bits(0x3fd5_5555_5555_5555);
        let source = format!("0 0 {precise_time} {precise_value} 2 2");
        let mut state = PwlEditorState::from_string(&source, "V");

        state.edit_buffers[2].1 = "3.25".to_owned();
        state.apply_buffer_edits().unwrap();

        assert_eq!(
            state.data.points()[1].time.to_bits(),
            precise_time.to_bits()
        );
        assert_eq!(
            state.data.points()[1].value.to_bits(),
            precise_value.to_bits()
        );

        let reparsed = PwlData::parse(&state.to_string()).unwrap();
        assert_eq!(reparsed.points()[1].time.to_bits(), precise_time.to_bits());
        assert_eq!(
            reparsed.points()[1].value.to_bits(),
            precise_value.to_bits()
        );
    }

    #[test]
    fn deleting_every_point_cannot_publish_an_empty_waveform() {
        let mut state = PwlEditorState::from_string("0 0", "V");
        state.selected_row = Some(0);
        state.delete_selected();

        assert!(state.data.is_empty());
        assert!(state.is_modified);
        assert_eq!(
            state.validation_error.as_deref(),
            Some("PWL data cannot be empty")
        );
        assert!(!state.is_valid());
        assert_eq!(state.data.validate(), Err(PwlValidationError::EmptyData));
    }
}
