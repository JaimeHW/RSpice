use crate::properties::parse_engineering_value;
use std::fmt;

use super::data::{PwlData, PwlPoint, PwlValidationError, format_engineering_for_spice};

/// State for the PWL editor UI widget.
#[derive(Debug, Clone, Default)]
pub struct PwlEditorState {
    /// The PWL data being edited.
    pub data: PwlData,
    /// Selected row index (if any).
    pub selected_row: Option<usize>,
    /// Text buffers for editing (time, value).
    pub edit_buffers: Vec<(String, String)>,
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
        let data = PwlData::parse(s).unwrap_or_default();
        let edit_buffers = data
            .points()
            .iter()
            .map(|p| {
                (
                    format_engineering_for_spice(p.time),
                    format_engineering_for_spice(p.value),
                )
            })
            .collect();

        Self {
            data,
            edit_buffers,
            value_unit: value_unit.to_string(),
            ..Default::default()
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
                    format_engineering_for_spice(p.time),
                    format_engineering_for_spice(p.value),
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

        self.data = PwlData::with_points(new_points);
        self.data.validate()?;
        self.validation_error = None;
        Ok(())
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
        self.new_time.clear();
        self.new_value.clear();
        self.adding_point = false;
    }

    /// Delete the selected point.
    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected_row {
            self.data.remove_point(idx);
            self.sync_buffers_from_data();
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
        write!(f, "{}", self.data.serialize())
    }
}
