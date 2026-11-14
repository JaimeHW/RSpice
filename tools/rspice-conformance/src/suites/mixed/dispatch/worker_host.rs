//! The worker route: a case's prepared tasks run in the browser worker.
//!
//! The case prepares and authorizes its deck here, as the application's main
//! thread does in the browser, then hands each task to the worker image the
//! browser loads, running the application's own `simulation-worker.js` under
//! Node (`worker_host.mjs`). A request crosses as the browser posts it and the
//! answer comes back through the transport the browser reads, so the worker
//! route differs from the others only where the browser does.
//!
//! `RSPICE_WORKER_HOST` replaces the Node host with another command speaking
//! the same frames (`worker_frames.mjs`), given the worker directory as its
//! last argument: `tools/ci/browser_worker_host.py` runs the worker in a
//! headless browser.

use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use rspice_core::abort_signal::AbortSignal;
use rspice_core::backends::ExecutionBackends;
use rspice_simulation::error::SimulationError;
use rspice_simulation::execution::ResolvedTaskDispatch;
use rspice_simulation::results::SimulationResult;
use rspice_simulation::runner::worker_wire::{worker_wire_request, worker_wire_result};

/// The Node program that hosts the worker, beside this file.
const HOST_SCRIPT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/suites/mixed/dispatch/worker_host.mjs"
);

/// How long the worker may take to load its image and qualify its JIT.
const STARTUP: Duration = Duration::from_secs(120);

/// One frame the host wrote: its header and its blobs.
struct Frame {
    header: serde_json::Value,
    blobs: Vec<Vec<u8>>,
}

impl Frame {
    fn kind(&self) -> &str {
        self.header["type"].as_str().unwrap_or_default()
    }

    fn id(&self) -> u64 {
        self.header["id"].as_u64().unwrap_or_default()
    }
}

/// What one task's run in the worker gave: its result, which code ran it,
/// the transient states it streamed, and its engine log lines.
struct WorkerRun {
    outcome: Result<SimulationResult, SimulationError>,
    backends: Option<ExecutionBackends>,
    checkpoints: Vec<Arc<[u8]>>,
    log: Vec<String>,
}

/// One task running in its worker on a thread of its own, so that a case can
/// run several side by side, each in a worker of its own.
pub(super) struct WorkerTask {
    thread: Option<JoinHandle<(WorkerHost, WorkerRun)>>,
    cancelled: Arc<AtomicBool>,
    worker: Option<WorkerHost>,
    run: Option<WorkerRun>,
}

impl WorkerTask {
    /// Run `dispatch` in `worker` until it answers, `deadline` passes or the
    /// task is aborted.
    pub(super) fn start(
        mut worker: WorkerHost,
        dispatch: ResolvedTaskDispatch,
        deadline: impl AbortSignal + 'static,
    ) -> Self {
        let cancelled = Arc::new(AtomicBool::new(false));
        let abort = TaskAbort {
            deadline,
            cancelled: Arc::clone(&cancelled),
        };
        let thread = std::thread::spawn(move || {
            let run = worker.run(dispatch, &abort);
            (worker, run)
        });
        Self {
            thread: Some(thread),
            cancelled,
            worker: None,
            run: None,
        }
    }

    /// Stop the task; its worker is told to stop and the run ends `Aborted`.
    pub(super) fn abort(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// The task's result, once its worker has answered.
    pub(super) fn poll_result(&mut self) -> Option<Result<SimulationResult, SimulationError>> {
        if !self.thread.as_ref()?.is_finished() {
            return None;
        }
        let (worker, mut run) = match self.thread.take()?.join() {
            Ok(finished) => finished,
            Err(panic) => std::panic::resume_unwind(panic),
        };
        let outcome = std::mem::replace(&mut run.outcome, Err(SimulationError::Aborted));
        self.worker = Some(worker);
        self.run = Some(run);
        Some(outcome)
    }

    /// The transient states the finished task streamed.
    pub(super) fn take_checkpoints(&mut self) -> Vec<Arc<[u8]>> {
        self.run
            .as_mut()
            .map(|run| std::mem::take(&mut run.checkpoints))
            .unwrap_or_default()
    }

    /// Which code the finished task ran, as its worker's answer says.
    pub(super) fn backends(&self) -> Option<ExecutionBackends> {
        self.run.as_ref().and_then(|run| run.backends.clone())
    }

    /// The finished task's engine log lines.
    pub(super) fn take_log(&mut self) -> Vec<String> {
        self.run
            .as_mut()
            .map(|run| std::mem::take(&mut run.log))
            .unwrap_or_default()
    }

    /// The finished task's worker, ready for another task.
    pub(super) fn take_worker(&mut self) -> Option<WorkerHost> {
        self.worker.take()
    }
}

/// A task's stop: the case's deadline, or the case abandoning the task.
struct TaskAbort<A> {
    deadline: A,
    cancelled: Arc<AtomicBool>,
}

impl<A: AbortSignal> AbortSignal for TaskAbort<A> {
    fn is_aborted(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed) || self.deadline.is_aborted()
    }
}

/// The worker, up and ready, for one case's tasks in turn.
pub(super) struct WorkerHost {
    child: Child,
    stdin: Option<ChildStdin>,
    frames: Receiver<Result<Frame, String>>,
    next_id: u64,
}

impl WorkerHost {
    /// Start the worker that `web_dir` serves (`simulation-worker.js` beside
    /// the worker image's `pkg/`) and wait until it is ready. `RSPICE_NODE`
    /// names the Node executable when `node` on PATH is not the one.
    pub(super) fn start(web_dir: &Path) -> Result<Self, String> {
        if !web_dir.join("simulation-worker.js").is_file() {
            return Err(format!(
                "{} holds no simulation-worker.js for the worker route",
                web_dir.display()
            ));
        }
        let host = std::env::var("RSPICE_WORKER_HOST").ok();
        let mut command = match host.as_deref().map(str::split_whitespace) {
            Some(mut words) => {
                let program = words.next().ok_or("RSPICE_WORKER_HOST is empty")?;
                let mut command = Command::new(program);
                command.args(words);
                command
            }
            None => {
                let mut command =
                    Command::new(std::env::var_os("RSPICE_NODE").unwrap_or_else(|| "node".into()));
                command.arg(HOST_SCRIPT);
                command
            }
        };
        let mut child = command
            .arg(web_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|error| format!("start the worker host: {error}"))?;
        let stdin = child.stdin.take().ok_or("the worker host has no stdin")?;
        let stdout = child.stdout.take().ok_or("the worker host has no stdout")?;
        let (sender, frames) = channel();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let frame = read_frame(&mut reader);
                let end = frame.is_err();
                if sender.send(frame).is_err() || end {
                    break;
                }
            }
        });
        let host = Self {
            child,
            stdin: Some(stdin),
            frames,
            next_id: 0,
        };
        let started = Instant::now();
        loop {
            let remaining = STARTUP.saturating_sub(started.elapsed());
            match host.frames.recv_timeout(remaining) {
                Ok(Ok(frame)) if frame.kind() == "ready" => return Ok(host),
                Ok(Ok(frame)) if frame.kind() == "error" => {
                    return Err(format!(
                        "the worker did not start: {}",
                        frame.header["error"].as_str().unwrap_or_default()
                    ));
                }
                Ok(Ok(_)) => {}
                Ok(Err(error)) => return Err(format!("the worker did not start: {error}")),
                Err(_) => {
                    return Err(format!(
                        "the worker was not ready within {} s",
                        STARTUP.as_secs()
                    ));
                }
            }
        }
    }

    /// Run one task in the worker until it answers or `abort` fires; on
    /// abort the host is told to stop and the run ends `Aborted`.
    fn run(&mut self, dispatch: ResolvedTaskDispatch, abort: &dyn AbortSignal) -> WorkerRun {
        self.next_id += 1;
        let id = self.next_id;
        let mut run = WorkerRun {
            outcome: Err(SimulationError::Aborted),
            backends: None,
            checkpoints: Vec::new(),
            log: Vec::new(),
        };
        // The worker reports locations on the expanded deck it solved; they
        // are read back to the files that wrote them, as the application's
        // runner reads them.
        let locator = dispatch.source_locator();
        let request = match worker_wire_request(id, dispatch) {
            Ok(request) => request,
            Err(error) => {
                run.outcome = Err(error);
                return run;
            }
        };
        let header = serde_json::json!({
            "id": id,
            "protocol": request.protocol,
            "floats": request.buffers.len(),
            "bytes": request.byte_buffers.len(),
        });
        let mut blobs: Vec<Vec<u8>> =
            Vec::with_capacity(1 + request.buffers.len() + request.byte_buffers.len());
        blobs.push(request.metadata.as_bytes().to_vec());
        blobs.extend(request.buffers.iter().map(|values| f64_bytes(values)));
        blobs.extend(request.byte_buffers.iter().cloned());
        let written = match self.stdin.as_mut() {
            Some(stdin) => write_frame(stdin, &header, &blobs),
            None => Err(std::io::Error::other("its input is closed")),
        };
        if let Err(error) = written {
            run.outcome = Err(SimulationError::EngineStopped(format!(
                "the worker host stopped reading: {error}"
            )));
            return run;
        }
        loop {
            if abort.is_aborted() {
                // The host stops the worker once its input closes, and is
                // killed if it does not ([`SHUTDOWN`]).
                self.stdin.take();
                run.outcome = Err(SimulationError::Aborted);
                return run;
            }
            let frame = match self.frames.recv_timeout(Duration::from_millis(20)) {
                Ok(Ok(frame)) => frame,
                Ok(Err(error)) => {
                    run.outcome = Err(SimulationError::EngineStopped(format!(
                        "the worker host stopped: {error}"
                    )));
                    return run;
                }
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => {
                    run.outcome = Err(SimulationError::EngineStopped(
                        "the worker host stopped".to_owned(),
                    ));
                    return run;
                }
            };
            if frame.id() != id {
                continue;
            }
            match frame.kind() {
                "checkpoint" => {
                    let bytes = frame.blobs.into_iter().next().unwrap_or_default();
                    match request.transient_checkpoint(bytes) {
                        Ok(checkpoint) => run.checkpoints.push(checkpoint),
                        Err(error) => {
                            // The browser host stops a worker whose
                            // checkpoint stream it cannot read.
                            run.outcome = Err(SimulationError::EngineStopped(format!(
                                "Invalid checkpoint stream: {error}"
                            )));
                            return run;
                        }
                    }
                }
                "log" => {
                    if let Some(message) = frame.header["message"].as_str() {
                        run.log.push(message.to_owned());
                    }
                }
                "error" => {
                    run.outcome = Err(SimulationError::EngineStopped(
                        frame.header["error"]
                            .as_str()
                            .unwrap_or_default()
                            .to_owned(),
                    ));
                    return run;
                }
                "result" => {
                    run.outcome = match answer(&frame) {
                        Ok((protocol, metadata, buffers)) => {
                            let (backends, result) =
                                worker_wire_result(id, protocol, metadata, buffers);
                            run.backends = backends;
                            result.map_err(|error| locator.locate(error))
                        }
                        Err(message) => Err(SimulationError::InvalidConfig(message)),
                    };
                    return run;
                }
                _ => {}
            }
        }
    }
}

/// How long a host whose input closed may take to stop its worker (and a
/// browser host its browser) before it is killed.
const SHUTDOWN: Duration = Duration::from_secs(20);

impl Drop for WorkerHost {
    fn drop(&mut self) {
        // Closing its input asks the host to stop what it started; killing it
        // straight away would leave a browser host's browser running.
        self.stdin.take();
        let started = Instant::now();
        while started.elapsed() < SHUTDOWN {
            match self.child.try_wait() {
                Ok(Some(_)) | Err(_) => return,
                Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            }
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// The worker's answer as the browser host reads it: its transport revision,
/// its metadata and its numeric buffers.
fn answer(frame: &Frame) -> Result<(u8, &str, Vec<Vec<f64>>), String> {
    let protocol = frame.header["protocol"]
        .as_u64()
        .and_then(|protocol| u8::try_from(protocol).ok())
        .ok_or("the worker's answer names no protocol")?;
    let mut blobs = frame.blobs.iter();
    let metadata = blobs
        .next()
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .ok_or("the worker's answer has no metadata")?;
    Ok((
        protocol,
        metadata,
        blobs.map(|bytes| f64_values(bytes)).collect(),
    ))
}

fn f64_bytes(values: &[f64]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn f64_values(bytes: &[u8]) -> Vec<f64> {
    bytes
        .chunks_exact(8)
        .map(|chunk| f64::from_le_bytes(chunk.try_into().expect("eight bytes")))
        .collect()
}

fn write_frame(
    out: &mut impl Write,
    header: &serde_json::Value,
    blobs: &[Vec<u8>],
) -> std::io::Result<()> {
    let header = serde_json::to_vec(header)?;
    let mut frame = Vec::with_capacity(
        8 + header.len() + blobs.iter().map(|blob| 4 + blob.len()).sum::<usize>(),
    );
    frame.extend(length(header.len())?);
    frame.extend(&header);
    frame.extend(length(blobs.len())?);
    for blob in blobs {
        frame.extend(length(blob.len())?);
        frame.extend(blob);
    }
    out.write_all(&frame)?;
    out.flush()
}

fn length(value: usize) -> std::io::Result<[u8; 4]> {
    u32::try_from(value)
        .map(u32::to_le_bytes)
        .map_err(|_| std::io::Error::other(format!("{value} bytes do not fit a frame")))
}

fn read_frame(input: &mut impl Read) -> Result<Frame, String> {
    let header_length = read_u32(input)?;
    let mut header = vec![0; header_length];
    input
        .read_exact(&mut header)
        .map_err(|error| error.to_string())?;
    let header: serde_json::Value =
        serde_json::from_slice(&header).map_err(|error| error.to_string())?;
    let count = read_u32(input)?;
    let mut blobs = Vec::with_capacity(count);
    for _ in 0..count {
        let mut blob = vec![0; read_u32(input)?];
        input
            .read_exact(&mut blob)
            .map_err(|error| error.to_string())?;
        blobs.push(blob);
    }
    Ok(Frame { header, blobs })
}

fn read_u32(input: &mut impl Read) -> Result<usize, String> {
    let mut bytes = [0; 4];
    input
        .read_exact(&mut bytes)
        .map_err(|error| error.to_string())?;
    Ok(u32::from_le_bytes(bytes) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_frame_reads_back_as_it_was_written() {
        let header = serde_json::json!({"type": "result", "id": 3, "protocol": 38, "floats": 1});
        let values = [1.5, -0.0, f64::MIN_POSITIVE, 1e300];
        let blobs = vec![b"{}".to_vec(), f64_bytes(&values)];
        let mut bytes = Vec::new();
        write_frame(&mut bytes, &header, &blobs).unwrap();
        let frame = read_frame(&mut bytes.as_slice()).unwrap();
        assert_eq!(frame.header, header);
        assert_eq!(frame.kind(), "result");
        assert_eq!(frame.id(), 3);
        let read = f64_values(&frame.blobs[1]);
        assert_eq!(
            read.iter().map(|value| value.to_bits()).collect::<Vec<_>>(),
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
    }
}
