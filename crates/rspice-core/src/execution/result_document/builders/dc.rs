//! Projection of complete DC grids, including the outer physical coordinate.
use super::*;

impl AnalysisResultDocument {
    /// Project a DC sweep without discarding its nested axis values.
    pub fn from_dc_analysis(
        analysis: AnalysisInstanceId,
        result: &crate::engine::DcSweepResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        let axes = result
            .axes
            .iter()
            .map(|axis| DcSweepAxisDocument {
                name: axis.name.clone(),
                unit: axis.unit.clone(),
                value_count: axis.values.len(),
            })
            .collect::<Vec<_>>();
        let mut builder = Self::from_nested_dc_sweep(analysis, &axes, &result.points)?;
        // The existing constructor projects the inner point coordinates. Every
        // outer axis must also travel with the result, repeated in point order.
        for (index, axis) in result
            .axes
            .iter()
            .enumerate()
            .take(axes.len().saturating_sub(1))
        {
            let values = (0..result.points.len())
                .map(|row| {
                    result
                        .axis_value(index, row)
                        .ok_or_else(|| source_error("DC sweep result", "invalid axis shape"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            builder = builder.axis(ResultAxis::new(
                format!("sweep:{}", axis.name),
                &axis.name,
                ResultAxisKind::SweepValue,
                axis.unit.clone(),
                AxisValues::Real {
                    values: finite_axis("DC sweep result", &axis.name, &values)?,
                },
            )?);
        }
        if let Some(index) = result.axes.len().checked_sub(1) {
            for (row, point) in result.points.iter().enumerate() {
                if result.axis_value(index, row) != Some(point.sweep_value) {
                    return Err(source_error(
                        "DC sweep result",
                        "point disagrees with its inner coordinate",
                    ));
                }
            }
        }
        Ok(builder)
    }
}
