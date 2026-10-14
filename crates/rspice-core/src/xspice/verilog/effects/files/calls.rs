//! The file and string system calls as a site table describes them: each
//! one's arguments read off the site, performed on the host's
//! [`super::HdlFiles`], and its results handed back as the call's value and
//! outputs.

use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalArrayRef, DigitalRadix, DigitalSystemArgument, DigitalSystemCall,
    DigitalSystemCallKind as Kind, DigitalValueClass,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalClock, DigitalEnvironment, DigitalEvalError, DigitalFunctionOutcome, DigitalScalar,
    DigitalTaskOutcome,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::four_state::FourStateBit;

use super::super::{
    CallOrigin, FileMonitor, HdlEffects, HdlStream, MonitorTask, integer_of, string_argument,
};
use super::scan::{self, ScanContext, TextSource};
use super::{EBADF, OpenFailure, WriteTarget, text_bytes};
use crate::xspice::verilog::store::DigitalSignalStore;

/// C's `EOF`.
const EOF: i32 = -1;

fn failed(site: &DigitalSystemCall, detail: impl Into<String>) -> DigitalEvalError {
    DigitalEvalError::SystemCallFailed {
        site: site.id,
        detail: detail.into(),
    }
}

fn word(value: i32) -> DigitalScalar {
    DigitalScalar::FourState(FourStateValue::from_u64(32, value as u32 as u64))
}

/// The value at argument `index`, with its signedness.
fn value_at<'a>(
    site: &DigitalSystemCall,
    args: &'a [DigitalScalar],
    index: usize,
) -> Option<(&'a DigitalScalar, bool)> {
    match site.arguments.get(index)? {
        DigitalSystemArgument::Value { operand, class }
        | DigitalSystemArgument::Output {
            operand: Some(operand),
            class,
            ..
        } => args.get(*operand as usize).map(|value| {
            (
                value,
                matches!(class, DigitalValueClass::FourState { signed: true, .. }),
            )
        }),
        _ => None,
    }
}

/// A known integer argument; `None` when it is absent, `x` or `z`.
fn integer_at(site: &DigitalSystemCall, args: &[DigitalScalar], index: usize) -> Option<i64> {
    let (value, signed) = value_at(site, args, index)?;
    integer_of(Some(value), signed)
}

/// A file name argument: a string with no unknown bit, not empty, and of
/// printable characters only, as Icarus checks one; the warning otherwise.
fn file_name(
    site: &DigitalSystemCall,
    args: &[DigitalScalar],
    index: usize,
) -> Result<String, String> {
    let unknown = matches!(
        value_at(site, args, index),
        Some((DigitalScalar::FourState(bits), _)) if bits.has_unknown()
    );
    let name = string_argument(site, args, index)
        .filter(|name| !unknown && !name.is_empty())
        .ok_or_else(|| "the file name argument is not a valid string".to_owned())?;
    if name
        .chars()
        .all(|character| character == ' ' || character.is_ascii_graphic())
    {
        return Ok(name);
    }
    let escaped: String = name
        .chars()
        .map(|character| {
            if character == ' ' || character.is_ascii_graphic() {
                character.to_string()
            } else {
                format!("\\{:03o}", u32::from(character) & 0xff)
            }
        })
        .collect();
    Err(format!(
        "the file name argument contains non-printable characters: \"{escaped}\""
    ))
}

/// A descriptor argument, as the 32 bits it names.
fn descriptor_at(site: &DigitalSystemCall, args: &[DigitalScalar], index: usize) -> Option<u32> {
    integer_at(site, args, index).map(|value| value as u32)
}

/// The class of the output at argument `index`.
fn output_class(site: &DigitalSystemCall, index: usize) -> Option<DigitalValueClass> {
    match site.arguments.get(index)? {
        DigitalSystemArgument::Output { class, .. } => Some(*class),
        _ => None,
    }
}

/// What an output argument held before the call.
fn output_current(
    site: &DigitalSystemCall,
    args: &[DigitalScalar],
    index: usize,
) -> Option<DigitalScalar> {
    match site.arguments.get(index)? {
        DigitalSystemArgument::Output {
            operand: Some(operand),
            ..
        } => args.get(*operand as usize).cloned(),
        _ => None,
    }
}

/// A one-dimensional memory a call loads (`$readmem*`, `$fread`).
struct Memory<'p> {
    array: DigitalArrayRef,
    class: DigitalValueClass,
    name: &'p str,
    /// The declared bounds, left first.
    bounds: (i64, i64),
}

/// A packed string at a target class.
fn string_value(class: DigitalValueClass, bytes: &[u8]) -> DigitalScalar {
    match class {
        DigitalValueClass::FourState { width, .. } => {
            DigitalScalar::FourState(scan::packed_bytes(width, bytes))
        }
        DigitalValueClass::Real => DigitalScalar::Real(0.0),
    }
}

impl HdlEffects {
    /// A file, string or waveform task (sections 17.2 and 18).
    #[allow(clippy::too_many_arguments)]
    pub(in super::super) fn file_task(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
        origin: CallOrigin,
    ) -> Result<DigitalTaskOutcome, DigitalEvalError> {
        let mut outcome = DigitalTaskOutcome::CONTINUE;
        match site.kind {
            Kind::Fdisplay(radix) | Kind::Fwrite(radix) => {
                let Some(descriptor) = self.descriptor_or_warn(plan, site, args, clock) else {
                    return Ok(outcome);
                };
                let mut text = self.format(plan, site, args, 1, radix)?;
                if matches!(site.kind, Kind::Fdisplay(_)) {
                    text.push('\n');
                }
                self.write_descriptor(plan, site, clock, descriptor, &text);
            }
            Kind::Fstrobe(radix) | Kind::Fmonitor(radix) => match origin {
                CallOrigin::Observer => {
                    let descriptor = match self.evaluating_monitor() {
                        Some(monitor) => match monitor.descriptor {
                            Some(descriptor) => Some(descriptor),
                            None => {
                                let descriptor = descriptor_at(site, args, 0);
                                if let Some(monitor) = self.evaluating_monitor_mut() {
                                    monitor.descriptor = descriptor;
                                }
                                descriptor
                            }
                        },
                        None => descriptor_at(site, args, 0),
                    };
                    let Some(descriptor) = descriptor else {
                        self.warn(plan, site, clock, "the file descriptor is unknown (x or z)");
                        return Ok(outcome);
                    };
                    let mut text = self.format(plan, site, args, 1, radix)?;
                    text.push('\n');
                    self.write_descriptor(plan, site, clock, descriptor, &text);
                }
                CallOrigin::Process(_) => {
                    let observer = site
                        .observer
                        .ok_or_else(|| failed(site, "an observer call has no argument function"))?;
                    if matches!(site.kind, Kind::Fstrobe(_)) {
                        self.queue.push(MonitorTask::Strobe(observer));
                    } else {
                        // Section 17.2.2: any number of `$fmonitor`s may be
                        // active, each until its file closes. Each prints at
                        // the end of the step that sets it up.
                        let serial = self.next_monitor_serial;
                        self.next_monitor_serial = serial.wrapping_add(1);
                        self.file_monitors.push(FileMonitor {
                            serial,
                            observer,
                            descriptor: None,
                            queued: true,
                        });
                        self.queue.push(MonitorTask::FileMonitor(serial));
                    }
                }
            },
            Kind::Fclose => {
                let Some(descriptor) = self.descriptor_or_warn(plan, site, args, clock) else {
                    return Ok(outcome);
                };
                match self.files.close(descriptor) {
                    Ok(()) => {
                        // Section 17.2.1: closing a file ends the `$fmonitor`s
                        // and pending `$fstrobe`s that write to it.
                        self.file_monitors.retain(|monitor| {
                            monitor
                                .descriptor
                                .is_none_or(|written| !closes(descriptor, written))
                        });
                    }
                    Err(message) => self.warn(plan, site, clock, &message),
                }
            }
            Kind::Fflush => {
                if !site.arguments.is_empty() {
                    let Some(descriptor) = self.descriptor_or_warn(plan, site, args, clock) else {
                        return Ok(outcome);
                    };
                    if let Err(message) = self.files.write_targets(descriptor) {
                        self.warn(plan, site, clock, &message);
                    }
                }
                // Every write reaches its file when the step commits; there
                // is no buffer of the design's to flush before that.
            }
            Kind::Sformat | Kind::Swrite(_) => {
                let class = output_class(site, 0)
                    .ok_or_else(|| failed(site, "the first argument must name a variable"))?;
                let text = if site.kind == Kind::Sformat {
                    let format = match site.arguments.get(1) {
                        Some(DigitalSystemArgument::String(text)) => text.to_string(),
                        _ => string_argument(site, args, 1)
                            .ok_or_else(|| failed(site, "the format must be a string"))?,
                    };
                    self.format_with(plan, site, args, 2, DigitalRadix::Decimal, Some(&format))?
                } else {
                    let radix = match site.kind {
                        Kind::Swrite(radix) => radix,
                        _ => DigitalRadix::Decimal,
                    };
                    self.format(plan, site, args, 1, radix)?
                };
                outcome.outputs = vec![string_value(class, &text_bytes(&text))];
            }
            other => {
                return Err(DigitalEvalError::SystemCallUnavailable {
                    site: site.id,
                    detail: format!("`{}` has no task behaviour", other.name()),
                });
            }
        }
        Ok(outcome)
    }

    /// A file or string function (section 17.2).
    pub(in super::super) fn file_function(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
        store: &mut DigitalSignalStore,
    ) -> Result<DigitalFunctionOutcome, DigitalEvalError> {
        let value = |value: i32| DigitalFunctionOutcome {
            value: word(value),
            outputs: Vec::new(),
        };
        match site.kind {
            Kind::Fopen => {
                let name = match file_name(site, args, 0) {
                    Ok(name) => name,
                    Err(message) => {
                        self.warn(plan, site, clock, &message);
                        return Ok(value(0));
                    }
                };
                let mode = match site.arguments.get(1) {
                    None => None,
                    Some(_) => Some(
                        string_argument(site, args, 1)
                            .ok_or_else(|| failed(site, "the mode must be a string"))?,
                    ),
                };
                let options = self.config.files.clone();
                match self.files.open(&options, &name, mode.as_deref()) {
                    Ok(descriptor) => Ok(value(descriptor as i32)),
                    Err(OpenFailure::Quiet) => Ok(value(0)),
                    Err(OpenFailure::Warn(message)) => {
                        self.warn(plan, site, clock, &message);
                        Ok(value(0))
                    }
                }
            }
            Kind::Fgetc => {
                let byte = self
                    .reader(plan, site, args, clock, 0)
                    .map(|file| file.next_byte());
                Ok(value(match byte {
                    Some(Some(byte)) => i32::from(byte),
                    _ => EOF,
                }))
            }
            Kind::Ungetc => {
                let byte = integer_at(site, args, 0);
                let file = self.reader(plan, site, args, clock, 1);
                Ok(value(match (file, byte) {
                    (Some(file), Some(byte)) if byte != i64::from(EOF) => {
                        file.unget(byte as u8);
                        0
                    }
                    _ => EOF,
                }))
            }
            Kind::Fgets => {
                let class = output_class(site, 0)
                    .ok_or_else(|| failed(site, "the first argument must name a variable"))?;
                let capacity = match class {
                    DigitalValueClass::FourState { width, .. } => (width / 8).max(1) as usize,
                    DigitalValueClass::Real => 8,
                };
                let mut line = Vec::new();
                if let Some(file) = self.reader(plan, site, args, clock, 1) {
                    while line.len() < capacity {
                        let Some(byte) = file.next_byte() else { break };
                        line.push(byte);
                        if byte == b'\n' {
                            break;
                        }
                    }
                }
                let output = if line.is_empty() {
                    output_current(site, args, 0).unwrap_or_else(|| string_value(class, b""))
                } else {
                    string_value(class, &line)
                };
                Ok(DigitalFunctionOutcome {
                    value: word(line.len() as i32),
                    outputs: vec![output],
                })
            }
            Kind::Fscanf | Kind::Sscanf => self.scanf(plan, site, args, clock),
            Kind::Fread => self.fread(plan, site, args, clock, store),
            Kind::Ftell => Ok(value(
                self.file(plan, site, args, clock, 0)
                    .map_or(EOF, |file| i32::try_from(file.tell()).unwrap_or(i32::MAX)),
            )),
            Kind::Fseek | Kind::Rewind => {
                let (offset, whence) = if site.kind == Kind::Rewind {
                    (Some(0), Some(0))
                } else {
                    (integer_at(site, args, 1), integer_at(site, args, 2))
                };
                let file = self.file(plan, site, args, clock, 0);
                let moved = match (file, offset, whence) {
                    (Some(file), Some(offset), Some(whence)) => file.seek(offset, whence),
                    _ => false,
                };
                Ok(value(if moved { 0 } else { EOF }))
            }
            Kind::Feof => Ok(value(
                self.file(plan, site, args, clock, 0)
                    .map_or(EOF, |file| i32::from(file.at_eof())),
            )),
            Kind::Ferror => {
                let class = output_class(site, 1)
                    .ok_or_else(|| failed(site, "the second argument must name a variable"))?;
                let code = match descriptor_at(site, args, 0) {
                    Some(descriptor) => match self.files.file_mut(descriptor) {
                        Ok(file) => file.error(),
                        Err(_) => EBADF,
                    },
                    None => EBADF,
                };
                let message = error_message(code);
                Ok(DigitalFunctionOutcome {
                    value: word(code),
                    outputs: vec![string_value(class, message.as_bytes())],
                })
            }
            other => Err(DigitalEvalError::SystemCallUnavailable {
                site: site.id,
                detail: format!("`{}` has no function behaviour", other.name()),
            }),
        }
    }

    /// Argument 0 as a descriptor, warning when it is unknown.
    fn descriptor_or_warn(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
    ) -> Option<u32> {
        let descriptor = descriptor_at(site, args, 0);
        if descriptor.is_none() {
            self.warn(plan, site, clock, "the file descriptor is unknown (x or z)");
        }
        descriptor
    }

    /// The open file argument `index` names, warning when there is none.
    fn file(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
        index: usize,
    ) -> Option<&mut super::OpenFile> {
        let Some(descriptor) = descriptor_at(site, args, index) else {
            self.warn(plan, site, clock, "the file descriptor is unknown (x or z)");
            return None;
        };
        if let Err(message) = self.files.file_mut(descriptor) {
            self.warn(plan, site, clock, &message);
            return None;
        }
        self.files.file_mut(descriptor).ok()
    }

    /// The readable open file argument `index` names.
    fn reader(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
        index: usize,
    ) -> Option<&mut super::OpenFile> {
        let Some(descriptor) = descriptor_at(site, args, index) else {
            self.warn(plan, site, clock, "the file descriptor is unknown (x or z)");
            return None;
        };
        if let Err(message) = self.files.reader(descriptor) {
            self.warn(plan, site, clock, &message);
            return None;
        }
        self.files.reader(descriptor).ok()
    }

    /// Write formatted text to a descriptor's files and streams.
    pub(in super::super) fn write_descriptor(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        clock: Option<DigitalClock>,
        descriptor: u32,
        text: &str,
    ) {
        let targets = match self.files.write_targets(descriptor) {
            Ok(targets) => targets,
            Err(message) => {
                self.warn(plan, site, clock, &message);
                return;
            }
        };
        let bytes = text_bytes(text);
        for target in targets {
            match target {
                // Standard error has no stream of its own in the records; it
                // reaches the transcript beside standard output.
                WriteTarget::Stdout | WriteTarget::Stderr => {
                    self.record(plan, site, clock, HdlStream::Stdout, text.to_owned());
                }
                WriteTarget::Descriptor(_) | WriteTarget::Channel(_) => {
                    let Some(file) = self.files.target_file(target) else {
                        continue;
                    };
                    let stream = HdlStream::File {
                        handle: match target {
                            WriteTarget::Descriptor(slot) => super::FD_FLAG | slot as u32,
                            WriteTarget::Channel(bit) => 1 << bit,
                            _ => 0,
                        },
                        path: file.name.to_string(),
                    };
                    self.files.write(target, &bytes);
                    self.record(plan, site, clock, stream, text.to_owned());
                }
            }
        }
    }

    fn scanf(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
    ) -> Result<DigitalFunctionOutcome, DigitalEvalError> {
        let targets: Vec<(usize, DigitalValueClass)> = (2..site.arguments.len())
            .filter_map(|index| output_class(site, index).map(|class| (index, class)))
            .collect();
        let unchanged = |site: &DigitalSystemCall| -> Vec<DigitalScalar> {
            targets
                .iter()
                .map(|(index, class)| {
                    output_current(site, args, *index)
                        .unwrap_or_else(|| super::super::unknown_of(*class))
                })
                .collect()
        };
        // A format with an unknown or high-impedance bit reads nothing and
        // returns EOF, as Icarus's does.
        if let Some(DigitalSystemArgument::Value { operand, .. }) = site.arguments.get(1)
            && let Some(DigitalScalar::FourState(bits)) = args.get(*operand as usize)
            && bits.has_unknown()
        {
            return Ok(DigitalFunctionOutcome {
                value: word(EOF),
                outputs: unchanged(site),
            });
        }
        let format = string_argument(site, args, 1)
            .ok_or_else(|| failed(site, "the format must be a string"))?;
        let format = text_bytes(&format);
        let classes: Vec<DigitalValueClass> = targets.iter().map(|(_, class)| *class).collect();
        let scope = plan
            .scopes
            .get(site.scope.index())
            .map_or_else(String::new, |scope| scope.path.to_string());
        let context = ScanContext {
            scope: &scope,
            time_shift: i32::from(self.time_format.units)
                - i32::from(site.time_scale.unit_exponent()),
            time_precision: self.time_format.precision,
        };
        let outcome = if site.kind == Kind::Sscanf {
            let input = string_argument(site, args, 0)
                .ok_or_else(|| failed(site, "the input must be a string"))?;
            let input = text_bytes(&input);
            scan::scan(&mut TextSource::new(&input), &format, &classes, &context)
        } else {
            match self.reader(plan, site, args, clock, 0) {
                Some(file) => scan::scan(file, &format, &classes, &context),
                None => scan::ScanOutcome {
                    count: EOF,
                    values: vec![None; classes.len()],
                },
            }
        };
        let outputs = targets
            .iter()
            .zip(outcome.values)
            .map(|((index, class), scanned)| match scanned {
                Some(scanned) => scan::fit(&scanned, *class),
                None => output_current(site, args, *index)
                    .unwrap_or_else(|| super::super::unknown_of(*class)),
            })
            .collect();
        Ok(DigitalFunctionOutcome {
            value: word(outcome.count),
            outputs,
        })
    }

    /// The array an argument names, with its declared bounds.
    fn memory<'p>(
        plan: &'p CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        index: usize,
    ) -> Result<Memory<'p>, DigitalEvalError> {
        let Some(DigitalSystemArgument::Array { array, class }) = site.arguments.get(index) else {
            return Err(failed(
                site,
                format!("argument {} must name a memory", index + 1),
            ));
        };
        let declared = plan
            .arrays
            .iter()
            .find(|declared| declared.storage == *array)
            .ok_or_else(|| failed(site, "the memory is not declared in this design"))?;
        if !declared.dimensions.is_empty() {
            return Err(failed(
                site,
                format!(
                    "`{}` is multidimensional; IEEE 1364-2005 section 17.2.8 loads a \
                     one-dimensional memory",
                    declared.name
                ),
            ));
        }
        Ok(Memory {
            array: *array,
            class: *class,
            name: declared.name.as_str(),
            bounds: declared.bounds,
        })
    }

    /// `$fread(memory, fd, start, count)` (section 17.2.4.4): big-endian
    /// words of whole bytes; the count of bytes read.
    fn fread(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
        store: &mut DigitalSignalStore,
    ) -> Result<DigitalFunctionOutcome, DigitalEvalError> {
        let Memory {
            array,
            class,
            name: memory,
            bounds,
        } = Self::memory(plan, site, 0)?;
        let DigitalValueClass::FourState { width, .. } = class else {
            return Err(failed(site, "`$fread` loads an integral memory"));
        };
        let low = bounds.0.min(bounds.1);
        let high = bounds.0.max(bounds.1);
        let start = integer_at(site, args, 2).unwrap_or(low);
        if !(low..=high).contains(&start) {
            self.warn(
                plan,
                site,
                clock,
                &format!(
                    "the start address {start} is outside the memory `{memory}` [{}:{}]; \
                     nothing is read",
                    bounds.0, bounds.1
                ),
            );
            return Ok(DigitalFunctionOutcome {
                value: word(0),
                outputs: Vec::new(),
            });
        }
        let room = high - start + 1;
        let mut count = integer_at(site, args, 3).map_or(room, |count| count.max(0));
        if count > room {
            self.warn(
                plan,
                site,
                clock,
                &format!(
                    "the count {count} runs past the memory `{memory}` [{}:{}] from {start}; \
                     {room} words are read",
                    bounds.0, bounds.1
                ),
            );
            count = room;
        }
        let mut read = 0usize;
        if let Some(file) = self.reader(plan, site, args, clock, 1) {
            for address in start..start + count {
                let Some(signal) = array.element(address) else {
                    break;
                };
                let mut value = store.value(signal).map_or_else(
                    || FourStateValue::splat(width, FourStateBit::Unknown),
                    Clone::clone,
                );
                let bytes = read_word(file, &mut value);
                read += bytes;
                if bytes == 0 {
                    break;
                }
                store.write_signal(signal, value);
            }
        }
        Ok(DigitalFunctionOutcome {
            value: word(read as i32),
            outputs: Vec::new(),
        })
    }
}

/// Read one `$fread` word into `value`: whole bytes, most significant first
/// (section 17.2.4.4), each replacing the eight bits it lands on. At the
/// end of the file the bytes not read keep what `value` held, as Icarus
/// leaves them. The count of bytes read.
fn read_word(file: &mut super::OpenFile, value: &mut FourStateValue) -> usize {
    let width = value.width();
    let bytes = width.div_ceil(8);
    let mut read = 0;
    for position in (0..bytes).rev() {
        let Some(byte) = file.next_byte() else {
            break;
        };
        read += 1;
        for bit in 0..8 {
            let index = position * 8 + bit;
            if index < width {
                let state = if byte >> bit & 1 == 1 {
                    FourStateBit::One
                } else {
                    FourStateBit::Zero
                };
                value.set_bit(index, state);
            }
        }
    }
    read
}

/// Whether closing `closed` ends output to `written`: the same file
/// descriptor, or a multichannel descriptor sharing a file bit.
fn closes(closed: u32, written: u32) -> bool {
    if closed & super::FD_FLAG != 0 || written & super::FD_FLAG != 0 {
        closed == written
    } else {
        closed & written & !1 != 0
    }
}

/// The text `$ferror` gives a C `errno` (section 17.2.7).
fn error_message(code: i32) -> &'static str {
    match code {
        0 => "",
        super::ENOENT => "No such file or directory",
        super::EIO => "Input/output error",
        EBADF => "Bad file descriptor",
        super::EACCES => "Permission denied",
        super::EINVAL => "Invalid argument",
        _ => "Unknown error",
    }
}
