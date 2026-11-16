//! A transient's result as it stands once its rows went to a result sink, kept with the rows.
//!
//! A run into a sink returns everything but its rows and events: every node and branch it named
//! (saved or not), each Verilog-A instance unknown's role, each node's discipline unit, the event
//! nets, the order of its traces, the digital buses, and what it learns only once its last row is
//! written (the HDL time precision of each trace an HDL design observes, and each HDL variable's
//! source lines). The run hands this shell to the sink as an annotation under
//! [`TRANSIENT_SHELL_ANNOTATION`], so a reader with neither the engine nor the netlist rebuilds
//! the result the live run had, trace for trace, and fills its columns and events from the rows.
//!
//! The encoding is little-endian: strings are a `u32` byte length and UTF-8, sequences a `u32`
//! count, options a `u8` flag. A version byte leads, and a reader refuses any other.

use crate::CurrentImpulseOwner;
use crate::analysis::transient::{
    DigitalBusDeclaration, DigitalBusSource, DigitalTrace, EventOnlyNetKind, HdlDriverKind,
    HdlSourceLine, HdlTraceDriver, HdlTraceSource, RealTrace, TransientDeviceOpTrace,
    TransientResult, TransientStoreTrace,
};
use crate::solver::InstanceUnknown;
use crate::{CurrentImpulseTrace, VoltageImpulseTrace};

/// The annotation key a transient's shell is kept under.
pub const TRANSIENT_SHELL_ANNOTATION: &str = "rspice.transient.shell";

const SHELL_VERSION: u8 = 1;

/// The shell of `result`: everything but its time axis, columns, step sizes, event points and
/// spectra.
pub fn encode_transient_shell(result: &TransientResult) -> Vec<u8> {
    let mut out = Out(Vec::new());
    out.u8(SHELL_VERSION);
    out.u64(result.num_nodes as u64);
    out.strings(&result.node_names);
    out.strings(&result.branch_names);
    out.seq(&result.instance_unknowns, |out, unknown| {
        out.u8(match unknown {
            None => 0,
            Some(InstanceUnknown::BranchCurrent) => 1,
            Some(InstanceUnknown::Private) => 2,
        });
    });
    out.seq(&result.potential_units, |out, unit| {
        out.option(unit.as_ref(), |out, unit| out.str(unit));
    });
    out.seq(&result.event_only_nodes, |out, kind| {
        out.u8(match kind {
            None => 0,
            Some(EventOnlyNetKind::Digital) => 1,
            Some(EventOnlyNetKind::Real) => 2,
        });
    });
    out.option(result.current_impulses.as_ref(), |out, traces| {
        out.seq(traces, |out, trace| {
            match &trace.owner {
                CurrentImpulseOwner::Branch { branch_name } => {
                    out.u8(0);
                    out.str(branch_name);
                }
                CurrentImpulseOwner::DeviceLead {
                    device_name,
                    parameter,
                } => {
                    out.u8(1);
                    out.str(device_name);
                    out.str(parameter);
                }
            }
            out.u8(u8::from(trace.complete));
        });
    });
    out.option(result.voltage_impulses.as_ref(), |out, traces| {
        out.seq(traces, |out, trace| {
            out.str(&trace.node_name);
            out.u8(u8::from(trace.complete));
        });
    });
    out.seq(&result.digital_traces, |out, trace| {
        out.str(&trace.node_name);
        out.option(trace.hdl_time_precision.as_ref(), |out, precision| {
            out.f64(*precision);
        });
    });
    out.seq(&result.digital_buses, |out, bus| {
        out.str(&bus.name);
        out.i64(bus.msb);
        out.i64(bus.lsb);
        out.strings(&bus.members);
        out.u8(match bus.source {
            DigitalBusSource::Engine => 0,
            DigitalBusSource::Schematic => 1,
            DigitalBusSource::Import => 2,
        });
    });
    out.seq(&result.real_traces, |out, trace| out.str(&trace.node_name));
    out.seq(&result.hdl_sources, |out, source| {
        out.str(&source.trace);
        out.str(&source.instance);
        out.str(&source.identifier);
        out.option(source.declaration.as_ref(), Out::line);
        out.seq(&source.drivers, |out, driver| {
            out.str(&driver.scope);
            out.u8(match driver.kind {
                HdlDriverKind::Initial => 0,
                HdlDriverKind::Always => 1,
                HdlDriverKind::Assign => 2,
            });
            out.option(driver.source.as_ref(), Out::line);
        });
    });
    out.seq(&result.device_op_traces, |out, trace| {
        out.str(&trace.device_name);
        out.str(&trace.parameter);
    });
    out.seq(&result.store_traces, |out, trace| out.str(&trace.name));
    out.0
}

/// The result a shell describes, with every column, step size and event list empty.
pub fn decode_transient_shell(bytes: &[u8]) -> Result<TransientResult, String> {
    let mut input = In { bytes, at: 0 };
    let version = input.u8()?;
    if version != SHELL_VERSION {
        return Err(format!(
            "a transient shell of version {version}; this build reads version {SHELL_VERSION}"
        ));
    }
    let num_nodes = usize::try_from(input.u64()?).map_err(|_| input.error("node count"))?;
    let node_names = input.strings()?;
    let branch_names = input.strings()?;
    let instance_unknowns = input.seq(|input| {
        Ok(match input.u8()? {
            0 => None,
            1 => Some(InstanceUnknown::BranchCurrent),
            2 => Some(InstanceUnknown::Private),
            _ => return Err(input.error("instance unknown")),
        })
    })?;
    let potential_units = input.seq(|input| input.option(In::string))?;
    let event_only_nodes = input.seq(|input| {
        Ok(match input.u8()? {
            0 => None,
            1 => Some(EventOnlyNetKind::Digital),
            2 => Some(EventOnlyNetKind::Real),
            _ => return Err(input.error("event-only net kind")),
        })
    })?;
    let current_impulses = input.option(|input| {
        input.seq(|input| {
            let owner = match input.u8()? {
                0 => CurrentImpulseOwner::Branch {
                    branch_name: input.string()?,
                },
                1 => CurrentImpulseOwner::DeviceLead {
                    device_name: input.string()?,
                    parameter: input.string()?,
                },
                _ => return Err(input.error("impulse owner")),
            };
            Ok(CurrentImpulseTrace {
                owner,
                complete: input.flag()?,
                points: Vec::new(),
                derivatives: Vec::new(),
            })
        })
    })?;
    let voltage_impulses = input.option(|input| {
        input.seq(|input| {
            Ok(VoltageImpulseTrace {
                node_name: input.string()?,
                complete: input.flag()?,
                points: Vec::new(),
                derivatives: Vec::new(),
            })
        })
    })?;
    let digital_traces = input.seq(|input| {
        Ok(DigitalTrace {
            node_name: input.string()?,
            points: Vec::new(),
            hdl_time_precision: input.option(In::f64)?,
        })
    })?;
    let digital_buses = input.seq(|input| {
        Ok(DigitalBusDeclaration {
            name: input.string()?,
            msb: input.i64()?,
            lsb: input.i64()?,
            members: input.strings()?,
            source: match input.u8()? {
                0 => DigitalBusSource::Engine,
                1 => DigitalBusSource::Schematic,
                2 => DigitalBusSource::Import,
                _ => return Err(input.error("bus source")),
            },
        })
    })?;
    let real_traces = input.seq(|input| {
        Ok(RealTrace {
            node_name: input.string()?,
            points: Vec::new(),
        })
    })?;
    let hdl_sources = input.seq(|input| {
        Ok(HdlTraceSource {
            trace: input.string()?,
            instance: input.string()?,
            identifier: input.string()?,
            declaration: input.option(In::line)?,
            drivers: input.seq(|input| {
                Ok(HdlTraceDriver {
                    scope: input.string()?,
                    kind: match input.u8()? {
                        0 => HdlDriverKind::Initial,
                        1 => HdlDriverKind::Always,
                        2 => HdlDriverKind::Assign,
                        _ => return Err(input.error("driver kind")),
                    },
                    source: input.option(In::line)?,
                })
            })?,
        })
    })?;
    let device_op_traces = input.seq(|input| {
        Ok(TransientDeviceOpTrace {
            device_name: input.string()?,
            parameter: input.string()?,
            values: Vec::new(),
        })
    })?;
    let store_traces = input.seq(|input| {
        Ok(TransientStoreTrace {
            name: input.string()?,
            values: Vec::new(),
        })
    })?;
    if input.at != bytes.len() {
        return Err(input.error("trailing bytes"));
    }
    if node_names.len() != num_nodes
        || instance_unknowns.len() > num_nodes
        || potential_units.len() > num_nodes
        || event_only_nodes.len() > num_nodes
    {
        return Err("a transient shell whose node tables disagree".to_owned());
    }
    Ok(TransientResult {
        time: Vec::new(),
        step_sizes: Vec::new(),
        voltages: vec![Vec::new(); num_nodes],
        branch_currents: vec![Vec::new(); branch_names.len()],
        num_nodes,
        node_names,
        branch_names,
        instance_unknowns,
        potential_units,
        event_only_nodes,
        current_impulses,
        voltage_impulses,
        digital_traces,
        digital_buses,
        real_traces,
        hdl_sources,
        device_op_traces,
        store_traces,
        fft_results: Vec::new(),
    })
}

struct Out(Vec<u8>);

impl Out {
    fn u8(&mut self, value: u8) {
        self.0.push(value);
    }

    fn u32(&mut self, value: usize) {
        // A shell's strings and tables are names and declarations, far below 4 GiB each.
        self.0
            .extend_from_slice(&u32::try_from(value).unwrap_or(u32::MAX).to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.0.extend_from_slice(&value.to_le_bytes());
    }

    fn i64(&mut self, value: i64) {
        self.0.extend_from_slice(&value.to_le_bytes());
    }

    fn f64(&mut self, value: f64) {
        self.0.extend_from_slice(&value.to_bits().to_le_bytes());
    }

    fn str(&mut self, value: &str) {
        self.u32(value.len());
        self.0.extend_from_slice(value.as_bytes());
    }

    fn strings(&mut self, values: &[String]) {
        self.seq(values, |out, value| out.str(value));
    }

    fn seq<T>(&mut self, values: &[T], mut each: impl FnMut(&mut Self, &T)) {
        self.u32(values.len());
        for value in values {
            each(self, value);
        }
    }

    fn option<T>(&mut self, value: Option<&T>, each: impl FnOnce(&mut Self, &T)) {
        match value {
            None => self.u8(0),
            Some(value) => {
                self.u8(1);
                each(self, value);
            }
        }
    }

    fn line(&mut self, line: &HdlSourceLine) {
        self.str(&line.file);
        self.0.extend_from_slice(&line.line.to_le_bytes());
    }
}

struct In<'b> {
    bytes: &'b [u8],
    at: usize,
}

impl In<'_> {
    fn error(&self, what: &str) -> String {
        format!(
            "a transient shell's {what} at byte {} is malformed",
            self.at
        )
    }

    fn take(&mut self, count: usize) -> Result<&[u8], String> {
        let end = self
            .at
            .checked_add(count)
            .filter(|&end| end <= self.bytes.len())
            .ok_or_else(|| self.error("length"))?;
        let taken = &self.bytes[self.at..end];
        self.at = end;
        Ok(taken)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], String> {
        let mut array = [0; N];
        array.copy_from_slice(self.take(N)?);
        Ok(array)
    }

    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.array::<1>()?[0])
    }

    fn flag(&mut self) -> Result<bool, String> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(self.error("flag")),
        }
    }

    fn u32(&mut self) -> Result<u32, String> {
        self.array().map(u32::from_le_bytes)
    }

    fn u64(&mut self) -> Result<u64, String> {
        self.array().map(u64::from_le_bytes)
    }

    fn i64(&mut self) -> Result<i64, String> {
        self.array().map(i64::from_le_bytes)
    }

    fn f64(&mut self) -> Result<f64, String> {
        self.array()
            .map(|bytes| f64::from_bits(u64::from_le_bytes(bytes)))
    }

    fn string(&mut self) -> Result<String, String> {
        let length = self.u32()? as usize;
        let bytes = self.take(length)?.to_vec();
        String::from_utf8(bytes).map_err(|_| self.error("text"))
    }

    fn strings(&mut self) -> Result<Vec<String>, String> {
        self.seq(In::string)
    }

    fn seq<T>(
        &mut self,
        mut each: impl FnMut(&mut Self) -> Result<T, String>,
    ) -> Result<Vec<T>, String> {
        let count = self.u32()? as usize;
        // Each element takes at least one byte: a count past the bytes left is malformed, and
        // never reserves more than the shell could hold.
        if count > self.bytes.len() - self.at {
            return Err(self.error("count"));
        }
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(each(self)?);
        }
        Ok(values)
    }

    fn option<T>(
        &mut self,
        each: impl FnOnce(&mut Self) -> Result<T, String>,
    ) -> Result<Option<T>, String> {
        if self.flag()? {
            each(self).map(Some)
        } else {
            Ok(None)
        }
    }

    fn line(&mut self) -> Result<HdlSourceLine, String> {
        Ok(HdlSourceLine {
            file: self.string()?,
            line: self.u32()?,
        })
    }
}
