//! `@device[quantity]`: ngspice's vec_get (frontend/vectors.c) finds a vector
//! of that name in the plot when a `save` put one there, and otherwise asks
//! the device what the last analysis left in it, whichever plot is current:
//! one value, or a source's waveform coefficients.
use super::*;
use crate::circuit::DeviceOpReport;
use crate::netlist::{ElementKind, SourceSpec};

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

/// Whether a `save` put the quantity in the dataset.
fn is_saved(dataset: &ControlNamedDataset, device: &str, quantity: &str) -> bool {
    dataset
        .saved_device_quantities
        .iter()
        .any(|(saved, parameter)| {
            is_device(saved, device) && parameter.eq_ignore_ascii_case(quantity)
        })
}

/// The axis of a DC sweep that steps the quantity: the value of a swept
/// source or resistor, which ngspice's sweep sets at each point
/// (spicelib/analysis/dctrcurv.c), so a saved `@v1[dc]` holds the sweep.
fn swept_axis(
    result: &crate::engine::DcSweepResult,
    device: &str,
    quantity: &str,
) -> Option<usize> {
    let quantity = quantity.to_ascii_uppercase();
    if !matches!(quantity.as_str(), "DC" | "DCV0" | "R" | "RESISTANCE") {
        return None;
    }
    // The device's letter says what it is, in a flat name too.
    let answers = match device
        .chars()
        .next()
        .map(|letter| letter.to_ascii_uppercase())
    {
        Some('V' | 'I') => matches!(quantity.as_str(), "DC" | "DCV0"),
        Some('R') => matches!(quantity.as_str(), "R" | "RESISTANCE"),
        _ => false,
    };
    answers
        .then(|| {
            result
                .axes
                .iter()
                .position(|axis| is_device(&axis.name, device))
        })
        .flatten()
}

/// The parameters ngspice's independent sources answer with their waveform
/// coefficients, whichever waveform the source has (vsrc.c, isrc.c).
const WAVEFORM_PARAMETERS: &[&str] = &[
    "pulse", "sine", "sin", "exp", "pwl", "sffm", "am", "trnoise", "trrandom", "coeffs",
];

/// A source's waveform coefficients as ngspice holds them (VSRCcoeffs): the
/// waveform's values in order, without the trailing ones the deck left to
/// their defaults, which ngspice never stores. `None` for a source with no
/// waveform ngspice reads.
fn waveform_coefficients(spec: &SourceSpec) -> Option<Vec<Value>> {
    const NAN: Value = Value::NAN;
    let (values, defaults): (Vec<Value>, &[Value]) = match spec {
        SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
            return waveform_coefficients(inner);
        }
        SourceSpec::DcTransient { transient, .. }
        | SourceSpec::AcTransient { transient, .. }
        | SourceSpec::DcAcTransient { transient, .. } => return waveform_coefficients(transient),
        SourceSpec::Pulse {
            v1,
            v2,
            delay,
            rise,
            fall,
            width,
            period,
            pulse_count,
            ..
        } => (
            vec![
                *v1,
                *v2,
                *delay,
                *rise,
                *fall,
                *width,
                *period,
                *pulse_count,
            ],
            &[NAN, NAN, 0.0, NAN, NAN, NAN, NAN, 0.0],
        ),
        SourceSpec::Sin {
            offset,
            amplitude,
            frequency,
            delay,
            damping,
            phase,
            ..
        } => (
            vec![
                *offset,
                *amplitude,
                *frequency,
                *delay,
                *damping,
                phase.to_degrees(),
            ],
            &[NAN, NAN, NAN, 0.0, 0.0, 0.0],
        ),
        SourceSpec::Exp {
            v1,
            v2,
            td1,
            tau1,
            td2,
            tau2,
        } => (vec![*v1, *v2, *td1, *tau1, *td2, *tau2], &[NAN; 6]),
        SourceSpec::Pwl { points, .. } => (
            points
                .iter()
                .flat_map(|(time, value)| [*time, *value])
                .collect(),
            &[],
        ),
        SourceSpec::Sffm {
            offset,
            amplitude,
            carrier_freq,
            modulation_index,
            signal_freq,
            delay,
            phase_modulation,
            phase_carrier,
        } => (
            vec![
                *offset,
                *amplitude,
                *carrier_freq,
                *modulation_index,
                *signal_freq,
                *delay,
                *phase_modulation,
                *phase_carrier,
            ],
            &[NAN, NAN, NAN, NAN, NAN, 0.0, 0.0, 0.0],
        ),
        SourceSpec::Am {
            offset,
            modulation_offset,
            modulation_amplitude,
            modulating_freq,
            carrier_freq,
            delay,
            phase_modulation,
            phase_carrier,
        } => (
            vec![
                *offset,
                *modulation_offset,
                *modulation_amplitude,
                *modulating_freq,
                *carrier_freq,
                *delay,
                *phase_modulation,
                *phase_carrier,
            ],
            &[NAN, NAN, 1.0, NAN, NAN, 0.0, 0.0, 0.0],
        ),
        SourceSpec::TrNoise {
            na,
            nt,
            nalpha,
            namp,
            rts_amplitude,
            rts_capture,
            rts_emit,
        } => (
            vec![
                *na,
                *nt,
                *nalpha,
                *namp,
                *rts_amplitude,
                *rts_capture,
                *rts_emit,
            ],
            &[0.0; 7],
        ),
        SourceSpec::TrRandom {
            distribution,
            sample_interval,
            delay,
            parameter1,
            parameter2,
        } => (
            vec![
                Value::from(*distribution),
                *sample_interval,
                *delay,
                *parameter1,
                *parameter2,
            ],
            &[NAN, NAN, 0.0, 1.0, 0.0],
        ),
        _ => return None,
    };
    let mut length = values.len();
    while length > 0 {
        let (value, default) = (values[length - 1], defaults.get(length - 1).copied());
        if value.is_nan() || default.is_some_and(|default| value == default) {
            length -= 1;
        } else {
            break;
        }
    }
    (length > 0).then(|| values[..length].to_vec())
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
                    }) || swept_axis(result, device, quantity).is_some()
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
            .filter(|(device, quantity)| {
                self.held_quantity(device, quantity).is_none()
                    && self.held_values(device, quantity).is_none()
            })
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
            let values = match circuit.static_instance_parameter(path, &quantity) {
                Some((_, value)) => Some(vec![value]),
                None => self.deck_quantity(engine, path, &quantity, line, abort)?,
            };
            if let Some(values) = values {
                self.instance_quantities.push((device, quantity, values));
            }
        }
        Ok(())
    }

    /// A quantity the built circuit does not answer, read from the deck as
    /// ngspice's queries read it: a source's waveform coefficients
    /// (spicelib/devices/vsrc/vsrcask.c:43, the same in isrcask.c), and a
    /// parameter of the model card of that name, which ngspice asks once no
    /// instance has the name (frontend/spiceif.c finddev).
    fn deck_quantity(
        &self,
        engine: &Engine,
        device: &str,
        quantity: &str,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, ControlExecutionError> {
        if let Some(element) = self
            .netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(device))
        {
            return Ok(match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec)
                    if WAVEFORM_PARAMETERS
                        .iter()
                        .any(|name| quantity.eq_ignore_ascii_case(name)) =>
                {
                    waveform_coefficients(spec)
                }
                _ => None,
            });
        }
        let Some(model) = self
            .netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(device))
        else {
            return Ok(None);
        };
        let temperature = self.analysis_config(engine).temperature;
        crate::engine::builder::model_card_parameter(
            &self.netlist,
            model,
            temperature,
            quantity,
            abort,
        )
        .map(|value| value.map(|value| vec![value]))
        .map_err(|error| simulation_error(line, error))
    }

    /// The quantity as the device holds it now, when it is one value.
    fn held_quantity(&self, device: &str, quantity: &str) -> Option<Value> {
        let reported = match &self.device_state {
            Some(DeviceState::Solved(report)) => report_value(report, device, quantity),
            _ => None,
        };
        reported.or_else(|| match self.instance_values(device, quantity)? {
            [value] => Some(*value),
            _ => None,
        })
    }

    /// The quantity as the device holds it now, when it is more than one
    /// value: a source's waveform coefficients.
    fn held_values(&self, device: &str, quantity: &str) -> Option<&[Value]> {
        self.instance_values(device, quantity)
            .filter(|values| values.len() != 1)
    }

    fn instance_values(&self, device: &str, quantity: &str) -> Option<&[Value]> {
        self.instance_quantities
            .iter()
            .find(|(known, parameter, _)| {
                known.eq_ignore_ascii_case(device) && parameter.eq_ignore_ascii_case(quantity)
            })
            .map(|(_, _, values)| values.as_slice())
    }

    /// The vector of a quantity a device holds as several values, read when
    /// no `save` put the quantity in `dataset`.
    pub(super) fn held_vector(
        &self,
        dataset: &ControlNamedDataset,
        name: &str,
    ) -> Option<VectorValue> {
        let (device, quantity) = split_quantity(name)?;
        if is_saved(dataset, device, quantity) {
            return None;
        }
        Some(VectorValue {
            samples: self
                .held_values(device, quantity)?
                .iter()
                .map(|value| (*value).into())
                .collect(),
            complex: false,
            scale: None,
            unit: SignalUnit::Unspecified,
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
                match report_position(first, device, quantity) {
                    Some((entry, parameter)) => Some(Column::DeviceSweep { entry, parameter }),
                    None => swept_axis(result, device, quantity).map(Column::DcAxis),
                }
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
        let column = is_saved(dataset, device, quantity)
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
