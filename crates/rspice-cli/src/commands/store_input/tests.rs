//! A results store converts as the run that wrote it published its outputs.

use std::path::{Path, PathBuf};

use clap::Parser;

use crate::cli::{Cli, Commands, Config};

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice-store-input-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn command(args: &[&str]) -> Commands {
    Cli::try_parse_from(std::iter::once("rspice").chain(args.iter().copied()))
        .unwrap()
        .command
}

/// Run `deck` to `output` in `format`, leaving its store beside it.
fn run(dir: &Path, deck: &str, format: &str, output: &Path) {
    let input = dir.join("deck.cir");
    std::fs::write(&input, deck).unwrap();
    let Commands::Run(args) = command(&[
        "run",
        input.to_str().unwrap(),
        "-f",
        format,
        "-o",
        output.to_str().unwrap(),
    ]) else {
        panic!("expected the run command");
    };
    crate::commands::run(args, &Config::default(), false, true).unwrap();
}

fn convert(input: &Path, output: &Path, to: &str, extra: &[&str]) {
    let mut args = vec![
        "convert",
        input.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        to,
    ];
    args.extend_from_slice(extra);
    let Commands::Convert(args) = command(&args) else {
        panic!("expected the convert command");
    };
    crate::commands::convert::execute(args, &Config::default(), true).unwrap();
}

/// The bytes with every rawfile `Date:` line emptied: each write stamps the wall clock.
fn undated(path: &Path) -> Vec<u8> {
    std::fs::read(path)
        .unwrap()
        .split(|&byte| byte == b'\n')
        .filter(|line| !line.starts_with(b"Date: "))
        .collect::<Vec<_>>()
        .join(&b'\n')
}

const RC: &str = "* rc into a store\nv1 in 0 pulse(0 1 1n 1n 1n 20n 50n)\n\
                  r1 in out 1k\nc1 out 0 1p\nr2 out 0 10k\n.tran 0.1n 200n\n.end\n";

/// A store converts to the table its run wrote, read a chunk at a time, and a selection and
/// a range convert as they do from that table.
#[test]
fn a_store_converts_to_the_table_its_run_wrote() {
    let dir = scratch("table");
    for format in ["csv", "raw", "ascii"] {
        let written = dir.join(format!("run.{format}"));
        run(&dir, RC, format, &written);
        let store = dir.join("run.rsr");
        assert!(store.is_dir(), "the run leaves its store beside its output");
        let converted = dir.join(format!("converted.{format}"));
        convert(&store, &converted, format, &[]);
        assert!(
            undated(&converted) == undated(&written),
            "{format}: the store converts to the run's own table"
        );
        let selected = ["--variables", "out", "--start", "20n", "--stop", "120n"];
        let (from_store, from_table) = (dir.join("a.csv"), dir.join("b.csv"));
        convert(&store, &from_store, "csv", &selected);
        // A rawfile carries no analysis name, so a table converted from one is named
        // `converted`; the run's own table keeps `transient`, as the store does.
        convert(&dir.join("run.csv"), &from_table, "csv", &selected);
        assert_eq!(
            std::fs::read(&from_store).unwrap(),
            std::fs::read(&from_table).unwrap(),
            "{format}: a selection and a range read from the store"
        );
    }
    let whole =
        super::load_table(&dir.join("run.rsr"), Config::default().resources.limits()).unwrap();
    assert!(whole.scale.len() > 100);
    let _ = std::fs::remove_dir_all(&dir);
}

const EVENTS: &str = "* xspice events into a store\nv1 in 0 pulse(0 5 0 1n 1n 5n 10n)\n\
                      abridge1 [in] [d] adc\nadac [d] [out] dac\naobs out rnode obs\n\
                      rout out 0 1k\n.model adc adc_bridge(in_low=1 in_high=4)\n\
                      .model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)\n\
                      .model obs v_to_real(gain=2)\n.tran 1n 20n\n.end\n";

/// A store's event dump, read from the store a frame at a time, is the dump its run wrote; one
/// with a range is built whole from the histories read back, as from any result file.
#[test]
fn a_store_converts_to_the_event_dump_its_run_wrote() {
    let dir = scratch("vcd");
    let written = dir.join("run.vcd");
    run(&dir, EVENTS, "vcd", &written);
    let store = dir.join("run.rsr");
    let converted = dir.join("converted.vcd");
    convert(&store, &converted, "vcd", &[]);
    assert_eq!(
        std::fs::read(&converted).unwrap(),
        std::fs::read(&written).unwrap()
    );
    let clipped = dir.join("clipped.vcd");
    convert(&store, &clipped, "vcd", &["--stop", "10n"]);
    let document = rspice_core::io::parse_vcd_bytes_with_limits(
        &std::fs::read(&clipped).unwrap(),
        Default::default(),
    )
    .unwrap();
    assert!(!document.signals.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

const LONG_RC: &str = "* rc into a store over several chunks\nv1 in 0 sin(0 1 10meg)\n\
                       r1 in out 1k\nc1 out 0 1p\n.tran 0.1n 2000n 0 0.1n\n.end\n";

fn try_convert(input: &Path, output: &Path) -> Result<(), crate::cli::CliError> {
    let Commands::Convert(args) = command(&[
        "convert",
        input.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        "csv",
    ]) else {
        panic!("expected the convert command");
    };
    crate::commands::convert::execute(args, &Config::default(), true).map(drop)
}

/// A copy of the store at `from` as `to`, its files as they are.
fn copy_store(from: &Path, to: &Path) -> PathBuf {
    let _ = std::fs::remove_dir_all(to);
    std::fs::create_dir_all(to.join("runs")).unwrap();
    for name in [
        rspice_results_store::StoreDir::MANIFEST.to_owned(),
        rspice_results_store::StoreDir::segment_name(0),
    ] {
        std::fs::copy(from.join(&name), to.join(&name)).unwrap();
    }
    to.join(rspice_results_store::StoreDir::segment_name(0))
}

/// A store whose segment is not the run its manifest names (a sync that brought back another
/// run's segment) is refused, never converted as the run the manifest records.
#[test]
fn a_segment_its_manifest_does_not_name_is_refused() {
    let dir = scratch("synced");
    let (first, second) = (dir.join("first"), dir.join("second"));
    std::fs::create_dir_all(&first).unwrap();
    std::fs::create_dir_all(&second).unwrap();
    run(&first, RC, "csv", &first.join("run.csv"));
    run(&second, LONG_RC, "csv", &second.join("run.csv"));
    let store = first.join("run.rsr");
    assert_eq!(super::store_notes(&store).unwrap(), Vec::<String>::new());
    let segment = rspice_results_store::StoreDir::segment_name(0);
    std::fs::copy(second.join("run.rsr").join(&segment), store.join(&segment)).unwrap();
    let refused = try_convert(&store, &dir.join("out.csv")).unwrap_err();
    assert!(
        refused
            .to_string()
            .contains("not the run its manifest names"),
        "{refused}"
    );
    assert!(!dir.join("out.csv").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

/// A store that cannot show its run is whole still converts what it holds, and says why: a run
/// that did not finish, a damaged `END`, a missing or damaged manifest.
#[test]
fn a_store_that_cannot_show_its_run_is_whole_converts_with_a_note() {
    let dir = scratch("notes");
    run(&dir, LONG_RC, "csv", &dir.join("run.csv"));
    let store = dir.join("run.rsr");
    let bytes = std::fs::read(store.join(rspice_results_store::StoreDir::segment_name(0))).unwrap();
    let end = u64::from_le_bytes(bytes[32..40].try_into().unwrap()) as usize;
    let mut damaged_end = bytes.clone();
    damaged_end[end + 16 + 40] ^= 0x01;
    // A run that died: its whole chunks on disk, no END, and the superblock naming none.
    let mut crashed = bytes[..bytes.len() * 3 / 5].to_vec();
    crashed[32..40].fill(0);
    let mut manifest = std::fs::read(store.join(rspice_results_store::StoreDir::MANIFEST)).unwrap();
    manifest[30] ^= 0x01;
    for (case, expected) in [
        ("crash", "did not finish"),
        ("cut", "no sound END"),
        ("end", "no sound END"),
        ("no manifest", "has no manifest"),
        ("bad manifest", "cannot be read"),
    ] {
        let copy = dir.join(format!("{}.rsr", case.replace(' ', "-")));
        let segment = copy_store(&store, &copy);
        match case {
            "crash" => std::fs::write(&segment, &crashed).unwrap(),
            "cut" => std::fs::write(&segment, &bytes[..bytes.len() * 3 / 5]).unwrap(),
            "end" => std::fs::write(&segment, &damaged_end).unwrap(),
            "no manifest" => {
                std::fs::remove_file(copy.join(rspice_results_store::StoreDir::MANIFEST)).unwrap()
            }
            _ => std::fs::write(
                copy.join(rspice_results_store::StoreDir::MANIFEST),
                &manifest,
            )
            .unwrap(),
        }
        let notes = super::store_notes(&copy).unwrap();
        assert!(
            notes.iter().any(|note| note.contains(expected)),
            "{case}: {notes:?}"
        );
        assert!(
            !notes.iter().any(|note| note.contains(" 0 rows")),
            "{case}: {notes:?}"
        );
        try_convert(&copy, &dir.join(format!("{case}.csv"))).unwrap();
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A store converts to JSON a chunk at a time, writing the document the whole table writes, and
/// within resource limits that the whole table, read at once, exceeds.
#[test]
fn a_store_streams_the_json_document_the_whole_table_writes() {
    use crate::cli::{NumericBound, OutputFormat};
    let dir = scratch("json");
    run(&dir, LONG_RC, "csv", &dir.join("run.csv"));
    let store = dir.join("run.rsr");
    let mut tight = Config::default().resources.limits();
    tight.max_external_data_values = 30_000;
    assert!(
        crate::commands::waveform_io::load_result_selected(
            &store,
            crate::cli::InputFormat::Rsr,
            tight,
            None
        )
        .is_err(),
        "the whole table, read at once, exceeds the limit"
    );
    let selected = ["--variables", "out", "--start", "300n", "--stop", "1300n"];
    for extra in [&[][..], &selected[..]] {
        let output = dir.join("streamed.json");
        let mut args = vec![
            "convert",
            store.to_str().unwrap(),
            output.to_str().unwrap(),
            "--to",
            "json",
        ];
        args.extend_from_slice(extra);
        let Commands::Convert(args) = command(&args) else {
            panic!("expected the convert command");
        };
        assert!(
            super::convert_streamed(&args, tight).unwrap(),
            "{extra:?}: the store streams its JSON"
        );
        let mut whole = super::load_table(&store, Config::default().resources.limits()).unwrap();
        whole.select_variables(&args.variables).unwrap();
        whole.clip_scale_range(
            args.start.as_ref().map(NumericBound::value),
            args.stop.as_ref().map(NumericBound::value),
        );
        let mut written = Vec::new();
        whole
            .write_to(&mut written, &output, OutputFormat::Json)
            .unwrap();
        assert!(
            std::fs::read(&output).unwrap() == written,
            "{extra:?}: the streamed document is the whole table's"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A segment's bytes in memory, every read of them counted.
struct CountedIo {
    bytes: Vec<u8>,
    reads: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl rspice_results_store::SegmentIo for CountedIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        self.reads
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let start = usize::try_from(offset).unwrap_or(usize::MAX);
        let held = start
            .checked_add(buf.len())
            .and_then(|end| self.bytes.get(start..end))
            .ok_or(std::io::ErrorKind::UnexpectedEof)?;
        buf.copy_from_slice(held);
        Ok(())
    }

    fn write_at(&mut self, _offset: u64, _data: &[u8]) -> std::io::Result<()> {
        Err(std::io::ErrorKind::Unsupported.into())
    }

    fn truncate(&mut self, _len: u64) -> std::io::Result<()> {
        Err(std::io::ErrorKind::Unsupported.into())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn size(&self) -> std::io::Result<u64> {
        Ok(self.bytes.len() as u64)
    }
}

/// A range reads only the chunks it reaches, and of them only the columns it keeps: a range
/// within one chunk of a long run reads that chunk's times and its one kept column and nothing
/// else, and ranges within a chunk and across chunks convert as the run's own table clips them.
#[test]
fn a_range_reads_only_its_own_chunks_and_kept_columns() {
    use std::sync::atomic::Ordering::Relaxed;
    let dir = scratch("window");
    run(&dir, LONG_RC, "csv", &dir.join("run.csv"));
    let store = dir.join("run.rsr");
    let bytes = std::fs::read(store.join(rspice_results_store::StoreDir::segment_name(0))).unwrap();
    let reads = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let reader = rspice_results_store::SegmentReader::open(CountedIo {
        bytes,
        reads: reads.clone(),
    })
    .unwrap();
    assert!(
        reader.chunk_spans().len() >= 3,
        "the run spans several chunks"
    );
    let (lo, hi) = reader.chunk_x_ranges().nth(1).unwrap();
    let (start, rows) = reader.chunk_spans().nth(1).unwrap();
    let slot = reader.slots() - 1;
    reads.store(0, Relaxed);
    assert_eq!(
        super::kept_rows(&store, &reader, &[slot], lo, hi).unwrap(),
        Some((start, rows as u64))
    );
    let window = reads.swap(0, Relaxed);
    let (mut x, mut values) = (Vec::new(), Vec::new());
    reader.read_chunk_x(1, &mut x).unwrap();
    reader.read_chunk_slot(1, slot, &mut values).unwrap();
    assert_eq!(
        window,
        reads.load(Relaxed),
        "the range reads its one chunk's times and kept column"
    );
    for (from, to) in [("700n", "800n"), ("300n", "1300n")] {
        let selected = ["--variables", "out", "--start", from, "--stop", to];
        let (from_store, from_table) = (dir.join("a.csv"), dir.join("b.csv"));
        convert(&store, &from_store, "csv", &selected);
        convert(&dir.join("run.csv"), &from_table, "csv", &selected);
        assert_eq!(
            std::fs::read(&from_store).unwrap(),
            std::fs::read(&from_table).unwrap(),
            "{from}..{to}: the range read from the store"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}
