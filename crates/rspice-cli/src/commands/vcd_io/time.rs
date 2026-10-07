//! Admit timeline coordinates before projecting them onto VCD event ticks.
use super::{CliError, ExportTable, Path, conversion_error};
use crate::commands::export_table::type_unit;

/// Resolve a declared time unit and validate every sample before coalescing
/// unchanged levels. An ordinary numeric sweep does not imply elapsed time.
pub(super) fn seconds_factor(path: &Path, table: &ExportTable) -> Result<f64, CliError> {
    let quantity = type_unit(&table.scale_type);
    if quantity.is_some_and(|unit| unit != "s") {
        return Err(conversion_error(
            path,
            format!(
                "VCD requires a time coordinate; '{}' declares type '{}'",
                table.scale_name, table.scale_type
            ),
        ));
    }
    let factor = match table.scale_unit.as_deref().map(str::trim) {
        Some("s" | "sec" | "second" | "seconds") => 1.0,
        Some("ms") => 1e-3,
        Some("us" | "µs" | "μs") => 1e-6,
        Some("ns") => 1e-9,
        Some("ps") => 1e-12,
        Some("fs") => 1e-15,
        None if quantity == Some("s") => 1.0,
        unit => {
            let stated_unit = unit.map_or_else(
                || "no stated unit".to_owned(),
                |unit| format!("unit '{unit}'"),
            );
            return Err(conversion_error(
                path,
                format!(
                    "coordinate '{}' has no supported time unit for VCD (type '{}', {stated_unit}); use a time coordinate in s, ms, us, ns, ps or fs",
                    table.scale_name, table.scale_type
                ),
            ));
        }
    };
    let mut previous = None;
    for (index, &value) in table.scale.iter().enumerate() {
        let seconds = value * factor;
        if !seconds.is_finite() || value < 0.0 || (value != 0.0 && seconds == 0.0) {
            return Err(conversion_error(
                path,
                format!(
                    "coordinate '{}' sample {index} cannot represent a finite, non-negative VCD time in seconds",
                    table.scale_name
                ),
            ));
        }
        if let Some((previous_value, previous_seconds)) = previous {
            if value < previous_value {
                return Err(conversion_error(
                    path,
                    format!(
                        "coordinate '{}' decreases at sample {index}; VCD time must not run backwards",
                        table.scale_name
                    ),
                ));
            }
            if value > previous_value && seconds <= previous_seconds {
                return Err(conversion_error(
                    path,
                    format!(
                        "coordinate '{}' loses distinct times when converted to seconds at sample {index}",
                        table.scale_name
                    ),
                ));
            }
        }
        previous = Some((value, seconds));
    }
    Ok(factor)
}
