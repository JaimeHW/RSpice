//! Complete DC sweep coordinates shared by direct and control execution.

use super::{DcSweepPointResult, Engine, SimulationError, bounded_dc_sweep_points};
use crate::netlist::{DcSecondSweep, DcSweepSpec};
use crate::{AbortSignal, Netlist, Value};

/// Compact authored coordinates, ordered outermost to innermost.
#[derive(Debug, Clone, PartialEq)]
pub struct DcSweepAxis {
    pub name: String,
    pub unit: crate::signal_unit::SignalUnit,
    pub values: Vec<Value>,
}

/// A DC sweep with its physical coordinates and per-point operating reports.
#[derive(Debug, Clone)]
pub struct DcSweepResult {
    pub axes: Vec<DcSweepAxis>,
    pub points: Vec<DcSweepPointResult>,
}

impl DcSweepResult {
    /// Coordinate for a flattened point; the innermost axis varies fastest.
    pub fn axis_value(&self, axis: usize, row: usize) -> Option<Value> {
        if row >= self.points.len() {
            return None;
        }
        let selected = self.axes.get(axis)?;
        let stride = self
            .axes
            .get(axis + 1..)?
            .iter()
            .try_fold(1usize, |n, a| n.checked_mul(a.values.len()))?;
        let coordinate = row
            .checked_div(stride)?
            .checked_rem(selected.values.len())?;
        selected.values.get(coordinate).copied()
    }

    pub(in crate::engine) fn value_count(&self) -> usize {
        self.points.iter().fold(
            self.axes
                .iter()
                .fold(0usize, |n, a| n.saturating_add(a.values.len())),
            |n, p| n.saturating_add(super::dc_sweep_point_value_count(p)),
        )
    }
}

impl Engine {
    /// Execute the full DC specification and retain both nested coordinates.
    /// Uses the ordinary DC solver, lifecycle, options and cancellation policy.
    pub fn run_dc_analysis_with_abort(
        &self,
        netlist: &Netlist,
        source: &str,
        primary: &DcSweepSpec,
        outer: Option<&DcSecondSweep>,
        abort: &dyn AbortSignal,
    ) -> Result<DcSweepResult, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        let mut axes = Vec::with_capacity(2);
        if let Some(outer) = outer {
            axes.push(DcSweepAxis {
                name: outer.source.trim().to_ascii_lowercase(),
                unit: crate::execution::sweep_axis_unit(&outer.source),
                values: bounded_dc_sweep_points(&engine, &outer.spec(), abort)?,
            });
        }
        axes.push(DcSweepAxis {
            name: source.trim().to_ascii_lowercase(),
            unit: crate::execution::sweep_axis_unit(source),
            values: bounded_dc_sweep_points(&engine, primary, abort)?,
        });
        let axis_values = axes
            .iter()
            .fold(0usize, |n, a| n.saturating_add(a.values.len()));
        engine.ensure_result_values(axis_values)?;
        let mut config = engine.config().clone();
        config.resource_limits.max_result_values -= axis_values;
        let bounded = engine.try_resolved_with_config(config)?;
        let points = bounded
            .run_dc_sweep2_spec_with_report_and_abort(netlist, source, primary, outer, abort)?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let result = DcSweepResult { axes, points };
        engine.ensure_result_values(result.value_count())?;
        Ok(result)
    }
}
