//! Physical units of measured values, independent of display preferences.

mod infer;
mod native;
pub(super) use native::annotate_native;

pub use infer::{expression_unit, measurement_units};
pub use rspice_units::{MeasurementUnit, MeasurementUnits};

pub(super) fn annotate(
    statements: &[&crate::netlist::measure::MeasureStatement],
    results: &mut [super::MeasureResult],
    axis: Option<&MeasurementUnit>,
    signals: &std::collections::HashMap<String, MeasurementUnit>,
) {
    for (result, units) in results
        .iter_mut()
        .zip(measurement_units(statements, axis, signals))
    {
        result.units = Some(units);
    }
}

#[cfg(test)]
mod tests;
