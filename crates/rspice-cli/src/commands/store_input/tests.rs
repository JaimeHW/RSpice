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
