//! What a Verilog-A instance reads from the run's HDL options: its open
//! files, its plusargs and the run's random seed.
//!
//! The open files (LRM 2.4 section 9.5) are kept by the digital kernel's
//! file machinery: the same path rules and confinement,
//! descriptor numbering, run store and `$fscanf` scanner, under the analog
//! transaction [`AnalogFileTable`] describes.
//!
//! An evaluation works on a candidate copy of the accepted table, so a
//! `$fopen` or `$fscanf` of a rejected point leaves no descriptor, position
//! or read behind. Writes reach the table only when an accepted point's file
//! tasks are delivered, and the run's store receives them, with the files an
//! accepted `$fopen` created, at once.

use rspice_veriloga::canonical_ir::digital::DigitalValueClass;
use rspice_veriloga::canonical_ir::digital_eval::DigitalScalar;
use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga_runtime::{
    AnalogFileOpen, AnalogFileTable, AnalogFileWrite, AnalogScan, AnalogScanValue,
    AnalogSystemFunction,
};

use super::files::scan::{self, ScanContext};
use super::files::{
    FD_FLAG, HandleImage, HdlFileOptions, HdlFiles, OpenFailure, WriteTarget, text_bytes,
};
use super::plusargs;
use crate::config::HdlRunConfig;

/// Give `device` what its value-returning system functions read from the
/// run: the plusargs the run was given, the run's random seed, and, when the
/// model opens or reads files, a table over the run's files.
pub(crate) fn configure_device(
    device: &mut VerilogADevice,
    config: &HdlRunConfig,
    files: &HdlFileOptions,
) -> Result<(), String> {
    let sites = device.analog_system_sites();
    if sites.is_empty() {
        return Ok(());
    }
    let mut opens_files = false;
    let mut answers = Vec::with_capacity(sites.len());
    for site in sites {
        let text = site.text.first().map(String::as_str).unwrap_or_default();
        answers.push(match site.function {
            AnalogSystemFunction::TestPlusargs => {
                plusargs::test(&config.plusargs, text).then_some(1.0)
            }
            AnalogSystemFunction::ValuePlusargs => {
                let class = if site.targets.first().copied().unwrap_or(true) {
                    INTEGER
                } else {
                    DigitalValueClass::Real
                };
                plusargs::value(&config.plusargs, text, class)
                    .map_err(|error| format!("{}: {error}", site.label()))?
                    .map(|value| match value {
                        DigitalScalar::Real(value) => value,
                        DigitalScalar::Integer(value) => f64::from(value),
                        // An analog integer has no unknown bits.
                        DigitalScalar::FourState(bits) => {
                            f64::from(bits.to_u64().map_or(0, |bits| bits as u32 as i32))
                        }
                        DigitalScalar::Effect => 0.0,
                    })
            }
            AnalogSystemFunction::Fopen | AnalogSystemFunction::Fscanf => {
                opens_files = true;
                None
            }
            _ => None,
        });
    }
    device
        .set_analog_plusargs(answers)
        .map_err(|error| error.to_string())?;
    // Each instance's stream starts from the run's seed and the instance's
    // name; the design's stream, a "global" draw's, from the run's seed, as
    // the digital kernel's `$random` does.
    device.set_analog_random_seed(config.seed as i32);
    if opens_files {
        let table = AnalogHdlFiles::new(files.clone(), &device.name);
        device.set_analog_files(Box::new(table));
    }
    Ok(())
}

/// The `$fscanf` targets an analog integer reads as.
const INTEGER: DigitalValueClass = DigitalValueClass::FourState {
    width: 32,
    signed: true,
};

#[derive(Debug, Clone)]
pub(crate) struct AnalogHdlFiles {
    options: HdlFileOptions,
    /// What `%m` in a `$fscanf` format reads: the instance.
    scope: String,
    accepted: HdlFiles,
    candidate: HdlFiles,
    /// Warnings the candidate raised.
    raised: Vec<String>,
    /// Warnings accepted points raised, not yet reported.
    warnings: Vec<String>,
}

impl AnalogHdlFiles {
    pub(crate) fn new(options: HdlFileOptions, scope: &str) -> Self {
        Self {
            options,
            scope: scope.to_owned(),
            accepted: HdlFiles::default(),
            candidate: HdlFiles::default(),
            raised: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Hand the accepted table's journal to the run's store.
    fn release(&mut self) -> Vec<String> {
        self.accepted.release(&self.options.store)
    }
}

impl AnalogFileTable for AnalogHdlFiles {
    fn clone_table(&self) -> Box<dyn AnalogFileTable> {
        Box::new(self.clone())
    }

    fn begin_evaluation(&mut self) {
        self.candidate.clone_from(&self.accepted);
        self.raised.clear();
    }

    fn accept(&mut self) {
        self.accepted.clone_from(&self.candidate);
        let failures = self.release();
        self.candidate.clone_from(&self.accepted);
        self.warnings.append(&mut self.raised);
        self.warnings.extend(failures);
    }

    fn reset_analysis(&mut self) {
        self.candidate.clone_from(&self.accepted);
        self.raised.clear();
    }

    fn open(&mut self, name: &str, mode: Option<&str>) -> AnalogFileOpen {
        match self.candidate.open(&self.options, name, mode) {
            Ok(descriptor) => AnalogFileOpen::Opened(descriptor),
            Err(OpenFailure::Quiet) => AnalogFileOpen::Failed(None),
            Err(OpenFailure::Warn(message)) => {
                let message = format!("$fopen(\"{name}\"): {message}");
                self.raised.push(message.clone());
                AnalogFileOpen::Failed(Some(message))
            }
        }
    }

    fn scan(&mut self, descriptor: u32, format: &str, targets: &[bool]) -> AnalogScan {
        let file = match self.candidate.reader(descriptor) {
            Ok(file) => file,
            Err(message) => {
                let message = format!("$fscanf: {message}");
                self.raised.push(message.clone());
                return AnalogScan {
                    count: -1,
                    values: Vec::new(),
                    warning: Some(message),
                };
            }
        };
        let classes: Vec<DigitalValueClass> = targets
            .iter()
            .map(|integer| {
                if *integer {
                    INTEGER
                } else {
                    DigitalValueClass::Real
                }
            })
            .collect();
        let context = ScanContext {
            scope: &self.scope,
            time_shift: 0,
            time_precision: 0,
        };
        let outcome = scan::scan(file, &text_bytes(format), &classes, &context);
        // Conversions assign their targets in order, so the assigned ones are
        // a prefix.
        let values = outcome
            .values
            .iter()
            .zip(&classes)
            .map_while(|(scanned, class)| {
                Some(match scan::fit(scanned.as_ref()?, *class) {
                    DigitalScalar::Real(value) => AnalogScanValue::Real(value),
                    // An analog integer has no unknown bits: an `x` or `z`
                    // field reads as zero.
                    DigitalScalar::FourState(bits) => {
                        AnalogScanValue::Integer(bits.to_u64().map_or(0, |bits| bits as u32 as i32))
                    }
                    DigitalScalar::Integer(value) => AnalogScanValue::Integer(value),
                    DigitalScalar::Effect => AnalogScanValue::Integer(0),
                })
            })
            .collect();
        AnalogScan {
            count: outcome.count,
            values,
            warning: None,
        }
    }

    fn deliver(&mut self, descriptor: u32, text: Option<&str>) -> Vec<AnalogFileWrite> {
        let mut writes = Vec::new();
        match text {
            None => {
                if let Err(message) = self.accepted.close(descriptor) {
                    writes.push(AnalogFileWrite::Warning(message));
                }
            }
            Some(text) => match self.accepted.write_targets(descriptor) {
                Err(message) => writes.push(AnalogFileWrite::Warning(message)),
                Ok(targets) => {
                    let bytes = text_bytes(text);
                    for target in targets {
                        match target {
                            WriteTarget::Stdout => {
                                writes.push(AnalogFileWrite::Stdout(text.to_owned()));
                            }
                            WriteTarget::Stderr => {
                                writes.push(AnalogFileWrite::Stderr(text.to_owned()));
                            }
                            WriteTarget::Descriptor(_) | WriteTarget::Channel(_) => {
                                let Some(file) = self.accepted.target_file(target) else {
                                    continue;
                                };
                                let name = file.name.to_string();
                                self.accepted.write(target, &bytes);
                                writes.push(AnalogFileWrite::File {
                                    handle: match target {
                                        WriteTarget::Descriptor(slot) => FD_FLAG | slot as u32,
                                        WriteTarget::Channel(bit) => 1 << bit,
                                        _ => 0,
                                    },
                                    name,
                                    text: text.to_owned(),
                                });
                            }
                        }
                    }
                }
            },
        }
        writes.extend(self.release().into_iter().map(AnalogFileWrite::Warning));
        self.candidate.clone_from(&self.accepted);
        writes
    }

    fn take_warnings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.warnings)
    }

    fn checkpoint(&self) -> Result<String, String> {
        let images = self.accepted.capture()?;
        serde_json::to_string(&images).map_err(|error| error.to_string())
    }

    fn restore(&mut self, image: &str) -> Result<(), String> {
        let images: Vec<HandleImage> =
            serde_json::from_str(image).map_err(|error| error.to_string())?;
        self.accepted.restore(&self.options, &images)?;
        let failures = self.release();
        if let Some(failure) = failures.into_iter().next() {
            return Err(failure);
        }
        self.candidate.clone_from(&self.accepted);
        self.raised.clear();
        self.warnings.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::xspice::verilog::effects::files::{HdlFileStore, HdlFileTarget};

    /// A run directory of the test's own on disk; the files themselves stay
    /// in the memory store.
    fn scratch(name: &str) -> std::path::PathBuf {
        let directory =
            std::env::temp_dir().join(format!("rspice-analog-files-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        directory
    }

    fn table(directory: &std::path::Path) -> (AnalogHdlFiles, Arc<HdlFileStore>) {
        let store = Arc::new(HdlFileStore::new(HdlFileTarget::Memory));
        let options = HdlFileOptions::new(Some(directory), false, Arc::clone(&store));
        (AnalogHdlFiles::new(options, "x1"), store)
    }

    #[test]
    fn a_rejected_open_and_read_leave_no_descriptor_or_position_behind() {
        let directory = &scratch("rejected");
        let (mut files, store) = table(directory);
        store.apply(vec![
            crate::xspice::verilog::effects::files::FileOp::Write {
                path: directory.join("in.txt"),
                offset: 0,
                bytes: b"1 2.5\n3 4.5\n".to_vec(),
            },
        ]);
        files.begin_evaluation();
        let AnalogFileOpen::Opened(input) = files.open("in.txt", Some("r")) else {
            panic!("in.txt opens");
        };
        files.accept();
        // A rejected point reads a line; the retry reads the same one.
        for _ in 0..2 {
            files.begin_evaluation();
            let scan = files.scan(input, "%d %g", &[true, false]);
            assert_eq!(scan.count, 2);
            assert_eq!(
                scan.values,
                vec![AnalogScanValue::Integer(1), AnalogScanValue::Real(2.5)]
            );
            let AnalogFileOpen::Opened(rejected) = files.open("rejected.txt", Some("w")) else {
                panic!("rejected.txt opens");
            };
            assert_eq!(rejected, input + 1);
        }
        files.begin_evaluation();
        assert_eq!(files.scan(input, "%d %g", &[true, false]).count, 2);
        files.accept();
        files.begin_evaluation();
        let scan = files.scan(input, "%d %g", &[true, false]);
        assert_eq!(
            scan.values,
            vec![AnalogScanValue::Integer(3), AnalogScanValue::Real(4.5)]
        );
        assert_eq!(store.written(&directory.join("rejected.txt")), None);
    }

    #[test]
    fn delivered_writes_land_in_the_store_and_a_checkpoint_reopens_them() {
        let directory = &scratch("delivered");
        let (mut files, store) = table(directory);
        files.begin_evaluation();
        let AnalogFileOpen::Opened(out) = files.open("out.txt", Some("w")) else {
            panic!("out.txt opens");
        };
        files.accept();
        let writes = files.deliver(out, Some("a 1\n"));
        assert_eq!(
            writes,
            vec![AnalogFileWrite::File {
                handle: out,
                name: "out.txt".into(),
                text: "a 1\n".into(),
            }]
        );
        let image = files.checkpoint().unwrap();
        files.deliver(out, Some("after the checkpoint\n"));
        let mut resumed = files.clone();
        resumed.restore(&image).unwrap();
        assert_eq!(
            store.written(&directory.join("out.txt")).as_deref(),
            Some(&b"a 1\n"[..])
        );
        resumed.deliver(out, Some("b 2\n"));
        assert_eq!(
            store.written(&directory.join("out.txt")).as_deref(),
            Some(&b"a 1\nb 2\n"[..])
        );
        assert_eq!(
            files.deliver(0x8000_0009, Some("x")),
            vec![AnalogFileWrite::Warning(
                "invalid file descriptor (0x80000009)".into()
            )]
        );
    }
}
