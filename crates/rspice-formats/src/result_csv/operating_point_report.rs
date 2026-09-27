//! CSV rows for the operating-point evidence selected by the caller.
use crate::table::escape_csv_field as csv_field;
use rspice_core::circuit::DeviceOpEntry;
use rspice_results::operating_point::{
    DcOpResult,
    report::{OperatingPointReportFacts, annotation_label, device_param_unit, process_label},
};

/// Accumulates solve facts, retained DC values and selected device parameters.
#[derive(Debug)]
pub struct OperatingPointReportCsv {
    contents: String,
    rows: usize,
}
impl Default for OperatingPointReportCsv {
    fn default() -> Self {
        Self::new()
    }
}
impl OperatingPointReportCsv {
    pub fn new() -> Self {
        Self {
            contents: String::from("section,owner,kind,region,quantity,value,unit,detail\n"),
            rows: 0,
        }
    }
    pub fn append_solve_facts(&mut self, facts: &OperatingPointReportFacts) {
        for (quantity, value, unit) in [
            (
                "temperature",
                format!("{:.17e}", facts.temperature_celsius),
                "degC",
            ),
            ("process", process_label(facts.process).to_owned(), ""),
            ("point_index", facts.point_index.to_string(), ""),
            ("point_count", facts.point_count.to_string(), ""),
            ("mna_nodes", facts.mna_nodes.to_string(), "count"),
            ("mna_branches", facts.mna_branches.to_string(), "count"),
            (
                "annotation",
                annotation_label(facts.annotation).to_owned(),
                "",
            ),
        ] {
            self.push_value(["solve_fact", "", "", "", quantity, &value, unit, ""]);
        }
    }
    /// Preserve nonfinite retained values with their existing explicit report label.
    pub fn append_dc_values(&mut self, dc: &DcOpResult) {
        for (section, values) in [
            ("node_voltage", dc.node_voltages.as_slice()),
            ("branch_current", dc.branch_currents.as_slice()),
            ("device_power", dc.power_dissipation.as_slice()),
        ] {
            for value in values {
                let row_detail = if value.value.is_finite() {
                    ""
                } else {
                    "non-finite retained value"
                };
                self.push_value([
                    section,
                    &value.name,
                    "",
                    "",
                    "value",
                    &format!("{:.17e}", value.value),
                    &value.unit,
                    row_detail,
                ]);
            }
        }
    }
    pub fn append_device(&mut self, entry: &DeviceOpEntry) {
        for (name, value) in &entry.params {
            let row_detail = if value.is_finite() {
                ""
            } else {
                "non-finite retained value"
            };
            self.push_value([
                "device_parameter",
                &entry.name,
                entry.device_kind,
                entry.region.unwrap_or_default(),
                name,
                &format!("{value:.17e}"),
                device_param_unit(entry.device_kind, name),
                row_detail,
            ]);
        }
    }
    fn push_value(&mut self, fields: [&str; 8]) {
        self.contents.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            csv_field(fields[0]),
            csv_field(fields[1]),
            csv_field(fields[2]),
            csv_field(fields[3]),
            csv_field(fields[4]),
            csv_field(fields[5]),
            csv_field(fields[6]),
            csv_field(fields[7]),
        ));
        self.rows += 1;
    }
    pub fn row_count(&self) -> usize {
        self.rows
    }
    pub fn into_string(self) -> String {
        self.contents
    }
}
