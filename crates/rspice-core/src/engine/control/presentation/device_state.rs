//! `@device[quantity]`: ngspice's vec_get (frontend/vectors.c) finds a vector
//! of that name in the plot when a `save` put one there, and otherwise asks
//! the device what the last analysis left in it, one value long, whichever
//! plot is current.
use super::*;
use crate::circuit::DeviceOpReport;

/// What the devices hold after the last analysis.
pub(in crate::engine::control) enum DeviceState {
    /// The analysis reported them at the last point it solved.
    Solved(Box<DeviceOpReport>),
    /// The analysis solved only the bias it linearizes around, which the
    /// first quantity read solves again from this deck.
    Bias(Box<Netlist>),
}

/// The device and quantity of `@device[quantity]`.
fn split_quantity(name: &str) -> Option<(&str, &str)> {
    name.strip_prefix('@')?.strip_suffix(']')?.split_once('[')
}

/// Every `@device[quantity]` a command's text names.
fn named_quantities(arguments: &str) -> impl Iterator<Item = (&str, &str)> {
    arguments.match_indices('@').filter_map(|(start, _)| {
        let rest = &arguments[start..];
        split_quantity(&rest[..=rest.find(']')?])
    })
}

/// Whether `device`, as a script spells it, is the instance `name`: an
/// ngspice flat name carries the device letter in front of the path.
fn is_device(name: &str, device: &str) -> bool {
    name.eq_ignore_ascii_case(device)
        || crate::netlist::flat_instance_path(device)
            .is_some_and(|path| name.eq_ignore_ascii_case(path))
}

/// Where a report holds a device's quantity, as entry and parameter positions.
fn report_position(
    report: &DeviceOpReport,
    device: &str,
    quantity: &str,
) -> Option<(usize, usize)> {
    report
        .entries
        .iter()
        .enumerate()
        .find_map(|(index, entry)| {
            if !is_device(&entry.name, device) {
                return None;
            }
            let parameter = entry
                .params
                .iter()
                .position(|(label, _)| label.eq_ignore_ascii_case(quantity))?;
            Some((index, parameter))
        })
}

fn report_value(report: &DeviceOpReport, device: &str, quantity: &str) -> Option<Value> {
    let (entry, parameter) = report_position(report, device, quantity)?;
    Some(report.entries[entry].params[parameter].1)
}

impl ControlNamedDataset {
    /// The vectors a `save` of a device quantity put in this plot.
    pub(super) fn saved_quantities(&self) -> impl Iterator<Item = (String, &'static str)> + '_ {
        self.saved_device_quantities
            .iter()
            .filter(|(device, quantity)| match &self.result {
                ControlAnalysisResult::OperatingPoint(_) => self
                    .device_op_report
                    .as_deref()
                    .is_some_and(|report| report_position(report, device, quantity).is_some()),
                ControlAnalysisResult::DcSweep(result) => {
                    result.points.first().is_some_and(|point| {
                        report_position(&point.device_op_report, device, quantity).is_some()
                    })
                }
                ControlAnalysisResult::Transient(result) => {
                    result.device_op_traces.iter().any(|trace| {
                        is_device(&trace.device_name, device)
                            && trace.parameter.eq_ignore_ascii_case(quantity)
                    })
                }
                _ => false,
            })
            .map(|(device, quantity)| (format!("@{device}[{quantity}]"), "notype"))
    }
}

impl ControlCircuit {
    /// Have ready what the quantities a command names read: the bias of a
    /// small-signal analysis, and a device's instance parameters, which
    /// follow the deck as `alter` leaves it.
    pub(in crate::engine::control) fn prepare_device_state(
        &mut self,
        engine: &Engine,
        arguments: &str,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        if let Some(DeviceState::Bias(netlist)) = &self.device_state {
            let bounded = engine
                .try_resolved_with_config(self.analysis_config(engine))
                .map_err(|source| ControlExecutionError::Configuration { line, source })?;
            let (_, report) = bounded
                .run_dc_op_with_report_and_abort(netlist, abort)
                .map_err(|error| simulation_error(line, error))?;
            self.device_state = Some(DeviceState::Solved(Box::new(report)));
        }
        self.instance_quantities.clear();
        let unreported = named_quantities(arguments)
            .filter(|(device, quantity)| self.held_quantity(device, quantity).is_none())
            .map(|(device, quantity)| (device.to_owned(), quantity.to_owned()))
            .collect::<Vec<_>>();
        if unreported.is_empty() {
            return Ok(());
        }
        let circuit = engine
            .build_circuit(&self.netlist)
            .map_err(|error| simulation_error(line, error))?;
        for (device, quantity) in unreported {
            let path = crate::netlist::flat_instance_path(&device).unwrap_or(&device);
            if let Some((_, value)) = circuit.static_instance_parameter(path, &quantity) {
                self.instance_quantities.push((device, quantity, value));
            }
        }
        Ok(())
    }

    /// The quantity as the device holds it now.
    fn held_quantity(&self, device: &str, quantity: &str) -> Option<Value> {
        let reported = match &self.device_state {
            Some(DeviceState::Solved(report)) => report_value(report, device, quantity),
            _ => None,
        };
        reported.or_else(|| {
            self.instance_quantities
                .iter()
                .find(|(known, parameter, _)| {
                    known.eq_ignore_ascii_case(device) && parameter.eq_ignore_ascii_case(quantity)
                })
                .map(|(_, _, value)| *value)
        })
    }

    /// The column a `save` put in this plot for the quantity.
    fn saved_column(
        &self,
        dataset: &ControlNamedDataset,
        device: &str,
        quantity: &str,
    ) -> Option<Column> {
        match &dataset.result {
            ControlAnalysisResult::OperatingPoint(_) => {
                let value = report_value(dataset.device_op_report.as_deref()?, device, quantity)?;
                Some(Column::DeviceState {
                    value,
                    complex: false,
                })
            }
            ControlAnalysisResult::DcSweep(result) => {
                let first = &result.points.first()?.device_op_report;
                let (entry, parameter) = report_position(first, device, quantity)?;
                Some(Column::DeviceSweep { entry, parameter })
            }
            ControlAnalysisResult::Transient(result) => result
                .device_op_traces
                .iter()
                .position(|trace| {
                    is_device(&trace.device_name, device)
                        && trace.parameter.eq_ignore_ascii_case(quantity)
                })
                .map(Column::DeviceTrace),
            // The output of a small-signal run holds the one bias value, as
            // that analysis's complex data.
            _ => Some(Column::DeviceState {
                value: self.held_quantity(device, quantity)?,
                complex: dataset.is_complex(),
            }),
        }
    }

    pub(super) fn device_quantity<'a>(
        &'a self,
        dataset: &'a ControlNamedDataset,
        name: &str,
        line: usize,
    ) -> Result<Selected<'a>, ControlError> {
        let (device, quantity) =
            split_quantity(name).ok_or_else(|| unavailable(line, dataset, name))?;
        let saved = dataset
            .saved_device_quantities
            .iter()
            .any(|(saved, parameter)| {
                is_device(saved, device) && parameter.eq_ignore_ascii_case(quantity)
            });
        let column = saved
            .then(|| self.saved_column(dataset, device, quantity))
            .flatten()
            .or_else(|| {
                let value = self.held_quantity(device, quantity)?;
                Some(Column::DeviceState {
                    value,
                    complex: false,
                })
            })
            .ok_or_else(|| unavailable(line, dataset, name))?;
        Ok(Selected {
            dataset,
            id: ControlVectorId {
                dataset: dataset.name.clone(),
                signal: name.to_ascii_lowercase(),
            },
            column,
            unit: SignalUnit::Unspecified,
            noise_probe: None,
        })
    }
}
