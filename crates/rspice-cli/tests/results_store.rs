//! A transient writes its rows to a results store, leaves the store beside its output, and
//! writes the output from it.

mod common;

use common::{run_rspice, test_dir};

const RC: &str = "rc step
v1 in 0 pulse(0 1 10n 1n 1n 200n 400n)
r1 in out 1k
c1 out 0 100p
.tran 5n 2u
.end
";

/// The same circuit with a measurement of an expression that reads another statement's running
/// result, which no chunk of rows gives alone, so the run is read back from its store into
/// memory before its outputs are written.
const RC_MEASURED: &str = "rc step
v1 in 0 pulse(0 1 10n 1n 1n 200n 400n)
r1 in out 1k
c1 out 0 100p
.tran 5n 2u
.meas tran vavg avg v(out)
.meas tran vmax max par('2*v(out)+0*vavg')
.end
";

/// Measurements that read the run's own columns, an expression of them or a voltage between two
/// nodes, which are measured in passes over its store.
const RC_MEASURES: &str = ".meas tran vavg avg v(out)
.meas tran gap max par('v(in)-v(out)') from=50n
.meas tran across rms v(in,out)
.meas tran vmax max v(out) from=100n
.meas tran rise trig v(out) val=0.1 rise=1 targ v(out) val=0.8 rise=1
.meas tran vat find v(out) at=333n
.meas tran fall when v(out)=0.5 fall=2
.meas tran slope deriv v(out) when v(in)=0.5 rise=last
.meas tran charge integ i(v1) from=0 to=1u
.meas tran both param='vavg+vmax'
";

/// `.FOUR` cards over a sine's response: the store's own columns, a voltage between two nodes
/// and an expression of them, and a card whose window ends before the run does.
const RC_FOUR: &str = "rc sine
v1 in 0 sin(0 1 1k)
r1 in out 1k
c1 out 0 100n
.tran 10u 5m
.four 1k v(out) v(in,out) {2*v(out)-v(in)}
.four 1k v(out) periods=2 to=3.3m
";

fn run(args: &[&str]) {
    let output = run_rspice(args);
    assert!(
        output.status.success(),
        "rspice {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn a_streamed_output_is_byte_identical_to_the_same_run_read_back_into_memory() {
    let dir = test_dir("results-store-streamed");
    let streamed_deck = dir.join("rc.cir");
    let measured_deck = dir.join("rc_measured.cir");
    std::fs::write(&streamed_deck, RC).unwrap();
    std::fs::write(&measured_deck, RC_MEASURED).unwrap();
    for format in ["raw", "ascii", "csv", "tsv", "hdf5"] {
        let streamed = dir.join(format!("streamed-{format}.out"));
        let read_back = dir.join(format!("read-back-{format}.out"));
        for (deck, output) in [(&streamed_deck, &streamed), (&measured_deck, &read_back)] {
            run(&[
                "--quiet",
                "run",
                deck.to_str().unwrap(),
                "-f",
                format,
                "-o",
                output.to_str().unwrap(),
            ]);
        }
        let streamed_bytes = std::fs::read(&streamed).unwrap();
        assert!(streamed_bytes.len() > 1000, "{format}: a whole table");
        assert_eq!(
            streamed_bytes,
            std::fs::read(&read_back).unwrap(),
            "{format}: the streamed table differs from the table read back"
        );
    }
}

/// The `format` output of `deck` (a deck without `.end`) written from its store, and of the
/// same deck with a measurement that reads it back whole, each run asserted to take its route.
fn streamed_and_read_back(
    dir: &std::path::Path,
    tag: &str,
    deck: &str,
    format: &str,
) -> [Vec<u8>; 2] {
    let read_back = ".meas tran vavg avg v(out)\n.meas tran vmax max par('2*v(out)+0*vavg')\n";
    [("streamed", ""), ("read-back", read_back)].map(|(twin, extra)| {
        let path = dir.join(format!("{tag}-{twin}.cir"));
        std::fs::write(&path, format!("{deck}{extra}.end\n")).unwrap();
        let out = dir.join(format!("{tag}-{twin}.{format}"));
        let output = run_rspice(&[
            "run",
            path.to_str().unwrap(),
            "-f",
            format,
            "-o",
            out.to_str().unwrap(),
            "--log-level",
            "debug",
        ]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{tag} {twin}: {stderr}");
        assert_eq!(
            twin == "streamed",
            !stderr.contains("back whole"),
            "{tag} {twin}: {stderr}"
        );
        std::fs::read(&out).unwrap()
    })
}

/// A `.PRINT` card chooses a delimited table's columns and names them as it spells them. A
/// column that is one of the store's is read from the store; an expression or a voltage between
/// two nodes is evaluated from the store's columns a chunk at a time. Either way the table is
/// written from the store, byte for byte the table of the run read back whole.
#[test]
fn a_printed_table_is_written_from_the_store() {
    let dir = test_dir("results-store-printed");
    let body = RC.trim_end_matches(".end\n");
    for (tag, print, header) in [
        ("columns", ".print tran v(out) i(v1)\n", "time,v(out),i(v1)"),
        (
            "derived",
            ".print tran v(in,out) {2*v(out)-v(in)} v(out)\n",
            "time,\"v(in,out)\",{2*v(out)-v(in)},v(out)",
        ),
    ] {
        let [streamed, read_back] =
            streamed_and_read_back(&dir, tag, &format!("{body}{print}"), "csv");
        let table = String::from_utf8(streamed.clone()).unwrap();
        assert!(table.starts_with(header), "{tag}: {table}");
        assert!(table.lines().count() > 300, "{tag}");
        assert_eq!(streamed, read_back, "{tag}");
    }
}

/// An `.OPTIONS OUTPUT` schedule's rows are walked from the store's as they are read: a
/// lattice interpolated between accepted steps, or exact time points the run stopped at, byte
/// for byte the output of the run read back whole, and fewer rows than every accepted step.
#[test]
fn an_output_schedule_is_projected_from_the_store() {
    let dir = test_dir("results-store-scheduled");
    let body = RC.trim_end_matches(".end\n");
    let [every_step, _] = streamed_and_read_back(&dir, "every-step", body, "csv");
    let every_step = String::from_utf8(every_step).unwrap().lines().count();
    for (tag, schedule) in [
        ("interval", ".options output initial_interval=25n\n"),
        ("points", ".options output outputtimepoints=100n,333n,1u\n"),
    ] {
        for format in ["csv", "raw"] {
            let tag = format!("{tag}-{format}");
            let [streamed, read_back] =
                streamed_and_read_back(&dir, &tag, &format!("{body}{schedule}"), format);
            if format == "csv" {
                let rows = String::from_utf8(streamed.clone()).unwrap().lines().count();
                assert!(
                    rows > 2 && rows < every_step,
                    "{tag}: {rows} of {every_step}"
                );
            }
            assert_eq!(streamed, read_back, "{tag}");
        }
    }
}

#[test]
fn measurements_read_from_the_store_equal_the_run_read_back_whole() {
    let dir = test_dir("results-store-measured");
    let streamed = dir.join("streamed.cir");
    let read_back = dir.join("read_back.cir");
    let body = RC.trim_end_matches(".end\n");
    std::fs::write(&streamed, format!("{body}{RC_MEASURES}.end\n")).unwrap();
    // A measurement of another one's running result reads the run in memory, and the others
    // with it.
    std::fs::write(
        &read_back,
        format!("{body}{RC_MEASURES}.meas tran twice max par('2*v(out)+0*vavg')\n.end\n"),
    )
    .unwrap();
    let measured = |deck: &std::path::Path, file: &str| {
        let path = dir.join(file);
        run(&[
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "--meas-file",
            path.to_str().unwrap(),
        ]);
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        json["measurements"].as_array().unwrap().clone()
    };
    let streamed = measured(&streamed, "streamed.json");
    let read_back = measured(&read_back, "read_back.json");
    assert_eq!(streamed.len(), 10);
    for measurement in &streamed {
        let twin = read_back
            .iter()
            .find(|twin| twin["name"] == measurement["name"])
            .unwrap();
        assert!(measurement["value"].is_number(), "{measurement}");
        for field in ["value", "raw_value", "event_axis", "error", "passed"] {
            assert_eq!(
                measurement[field], twin[field],
                "{field}: {measurement} {twin}"
            );
        }
    }
}

/// `.MEASURE TRAN_CONT` statements over the store's columns are measured in one pass over its
/// rows: every record, failure and endpoint the same as the run read back whole.
#[test]
fn continuous_measurements_read_from_the_store_equal_the_run_read_back_whole() {
    let dir = test_dir("results-store-continuous");
    let body = RC.trim_end_matches(".end\n");
    let statements = ".meas tran_cont up when v(out)=0.5 rise=2
.meas tran_cont lastfall when v(out)=0.5 fall=-2
.meas tran_cont crossed find v(in) when v(out)=0.3 cross=2
.meas tran_cont slope deriv v(out) when v(out)=0.4
.meas tran_cont slopeat deriv v(out) at=333n
.meas tran_cont findat find v(out) at=333n
.meas tran_cont windowed when v(out)=0.5 cross=1 from=500n to=1.5u td=100n
.meas tran_cont delay trig v(in) val=0.5 rise=1 targ v(out) val=0.5 rise=1
.meas tran_cont delayat trig at=100n targ v(out) val=0.5 fall=1
.meas tran_cont never when v(out)=5
.meas tran_cont unpaired trig v(in) val=0.5 rise=1 targ v(out) val=5 rise=1
";
    let read_back = ".meas tran vavg avg v(out)\n.meas tran vmax max par('2*v(out)+0*vavg')\n";
    let measured = |twin: &str, extra: &str| {
        let deck = dir.join(format!("{twin}.cir"));
        std::fs::write(&deck, format!("{body}{statements}{extra}.end\n")).unwrap();
        let path = dir.join(format!("{twin}.json"));
        let output = run_rspice(&[
            "run",
            deck.to_str().unwrap(),
            "--meas-file",
            path.to_str().unwrap(),
            "--log-level",
            "debug",
        ]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        // NEVER and UNPAIRED fail, which fails the run's verification.
        assert_eq!(output.status.code(), Some(3), "{twin}: {stderr}");
        assert_eq!(
            twin == "streamed",
            !stderr.contains("back whole"),
            "{twin}: {stderr}"
        );
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        json["measurements"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| !["VAVG", "VMAX"].contains(&row["name"].as_str().unwrap_or("")))
            .map(|row| {
                // Each row names its deck.
                let mut row = row.clone();
                if let Some(row) = row.as_object_mut() {
                    row.remove("netlist");
                    row.remove("run");
                }
                row
            })
            .collect::<Vec<_>>()
    };
    let streamed = measured("streamed", "");
    let read_back = measured("read-back", read_back);
    assert!(streamed.len() > 11, "{streamed:?}");
    assert!(
        streamed.iter().any(|row| row["value"].is_number()),
        "{streamed:?}"
    );
    assert_eq!(streamed, read_back);
}

/// A FRAC_MAX delay, whose levels follow each clause's running maximum, is measured in a pass
/// over the store's rows, as the run read back whole measures it.
#[test]
fn frac_max_delays_are_measured_from_the_store() {
    let dir = test_dir("results-store-frac-max");
    let body = RC.trim_end_matches(".end\n");
    let statements = ".meas tran rise trig v(out) frac_max=0.1 targ v(out) frac_max=0.9
.meas tran fall trig v(out) frac_max=0.9 fall=1 targ v(out) frac_max=0.1 fall=1
.meas tran late trig v(out) frac_max=0.2 rise=2 targ v(out) frac_max=0.8 rise=2 td=300n
.meas tran attarg trig at=100n targ v(out) frac_max=0.5
.meas tran never trig v(out) frac_max=0.1 rise=9 targ v(out) frac_max=0.9 rise=9
";
    let read_back = ".meas tran vavg avg v(out)\n.meas tran vmax max par('2*v(out)+0*vavg')\n";
    let measured = |twin: &str, extra: &str| {
        let deck = dir.join(format!("{twin}.cir"));
        std::fs::write(&deck, format!("{body}{statements}{extra}.end\n")).unwrap();
        let path = dir.join(format!("{twin}.json"));
        let output = run_rspice(&[
            "run",
            deck.to_str().unwrap(),
            "--meas-file",
            path.to_str().unwrap(),
            "--log-level",
            "debug",
        ]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        // NEVER fails, which fails the run's verification.
        assert_eq!(output.status.code(), Some(3), "{twin}: {stderr}");
        assert_eq!(
            twin == "streamed",
            !stderr.contains("back whole"),
            "{twin}: {stderr}"
        );
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        json["measurements"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| !["VAVG", "VMAX"].contains(&row["name"].as_str().unwrap_or("")))
            .map(|row| {
                // Each row names its deck.
                let mut row = row.clone();
                if let Some(row) = row.as_object_mut() {
                    row.remove("netlist");
                    row.remove("run");
                }
                row
            })
            .collect::<Vec<_>>()
    };
    let streamed = measured("streamed", "");
    let read_back = measured("read-back", read_back);
    assert_eq!(streamed.len(), 5, "{streamed:?}");
    assert!(streamed[0]["value"].is_number(), "{streamed:?}");
    assert_eq!(streamed, read_back);
}

/// `.FFT` cards over a sine's response: a store column, a voltage between two nodes, an
/// expression of them and a lead power, with lead power printed and measured.
const RC_FFT: &str = "rc sine fft
v1 in 0 sin(0 1 1k)
r1 in out 1k
c1 out 0 100n
.tran 10u 4m
.fft v(out) np=64 window=hann
.fft v(in,out) np=32 start=1m stop=3m window=rect
.fft {2*v(out)-v(in)} np=16 format=unorm
.fft p(r1) np=64 window=hann
.print tran v(out) p(r1) w(r1)
.meas tran pmax max p(r1)
.meas tran wavg avg w(r1)
.meas fft thd thd v(out)
";

/// The `.FFT` spectra are taken in one pass over the store, and lead power is derived from each
/// chunk's rows: the table, the spectra and the measurements equal the run read back whole.
#[test]
fn fft_spectra_and_lead_power_read_from_the_store_equal_the_run_read_back_whole() {
    let dir = test_dir("results-store-fft-power");
    let read_back = ".meas tran vavg avg v(out)\n.meas tran vmax max par('2*v(out)+0*vavg')\n";
    let twins = [("streamed", ""), ("read-back", read_back)].map(|(twin, extra)| {
        let deck = dir.join(format!("{twin}.cir"));
        std::fs::write(&deck, format!("{RC_FFT}{extra}.end\n")).unwrap();
        let out = dir.join(twin);
        let meas = dir.join(format!("{twin}.json"));
        let output = run_rspice(&[
            "run",
            deck.to_str().unwrap(),
            "-f",
            "csv",
            "-o",
            out.join("out.csv").to_str().unwrap(),
            "--meas-file",
            meas.to_str().unwrap(),
            "--log-level",
            "debug",
        ]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{twin}: {stderr}");
        assert_eq!(
            twin == "streamed",
            !stderr.contains("back whole"),
            "{twin}: {stderr}"
        );
        let mut files = std::fs::read_dir(&out)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .map(|path| {
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                (name, std::fs::read(&path).unwrap())
            })
            .collect::<Vec<_>>();
        files.sort();
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&meas).unwrap()).unwrap();
        let rows = json["measurements"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| !["VAVG", "VMAX"].contains(&row["name"].as_str().unwrap_or("")))
            .map(|row| {
                // Each row names its deck.
                let mut row = row.clone();
                if let Some(row) = row.as_object_mut() {
                    row.remove("netlist");
                    row.remove("run");
                }
                row
            })
            .collect::<Vec<_>>();
        (files, rows)
    });
    let (files, rows) = &twins[0];
    assert!(
        files.iter().any(|(name, _)| name.contains("fft")),
        "{:?}",
        files.iter().map(|(name, _)| name).collect::<Vec<_>>()
    );
    assert_eq!(rows.len(), 3, "{rows:?}");
    assert!(rows.iter().all(|row| row["value"].is_number()), "{rows:?}");
    let names = files.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let (_, table) = files
        .iter()
        .find(|(name, _)| name.contains("tran"))
        .unwrap_or_else(|| panic!("{names:?}"));
    let header = String::from_utf8_lossy(table)
        .lines()
        .next()
        .unwrap_or("")
        .to_owned();
    assert_eq!(header, "time,v(out),p(r1),w(r1)");
    assert_eq!(twins[0], twins[1]);
}

#[test]
fn fourier_spectra_read_around_their_windows_equal_the_run_read_back_whole() {
    let dir = test_dir("results-store-fourier");
    // A measurement of another one's running result reads the run back whole.
    let read_back = ".meas tran vavg avg v(out)\n.meas tran vmax max par('2*v(out)+0*vavg')\n";
    // The second deck ends with a card whose window is longer than the run.
    for (tag, card) in [("spectra", ""), ("refused", ".four 1k v(out) periods=50\n")] {
        let mut twins = Vec::new();
        for (twin, extra) in [("windowed", ""), ("read-back", read_back)] {
            let deck = dir.join(format!("{tag}-{twin}.cir"));
            std::fs::write(&deck, format!("{RC_FOUR}{card}{extra}.end\n")).unwrap();
            let out = dir.join(format!("{tag}-{twin}"));
            let output = run_rspice(&[
                "run",
                deck.to_str().unwrap(),
                "-f",
                "csv",
                "-o",
                out.join("out.csv").to_str().unwrap(),
            ]);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let frames = stdout
                .lines()
                .filter(|line| line.starts_with(['│', '┌', '├', '└']) || line.contains("Fourier"))
                .map(str::to_owned)
                .collect::<Vec<_>>();
            let refusal = String::from_utf8_lossy(&output.stderr)
                .lines()
                .filter(|line| line.contains("could not be analyzed"))
                .map(str::to_owned)
                .collect::<Vec<_>>();
            let mut spectra = std::fs::read_dir(&out)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.to_string_lossy().contains(".four-"))
                .map(|path| std::fs::read(path).unwrap())
                .collect::<Vec<_>>();
            spectra.sort();
            twins.push((output.status.code(), frames, refusal, spectra));
        }
        let (code, frames, refusal, spectra) = &twins[0];
        assert_eq!(spectra.len(), 4, "{tag}: one spectrum per operand");
        assert!(frames.len() > 60, "{tag}: {frames:?}");
        assert_eq!(*code == Some(0), refusal.is_empty(), "{tag}: {refusal:?}");
        assert_eq!(twins[0], twins[1], "{tag}");
    }
}

#[test]
fn a_run_leaves_its_store_beside_its_output_and_the_summary_names_it() {
    let dir = test_dir("results-store-kept");
    let deck = dir.join("rc.cir");
    std::fs::write(&deck, RC).unwrap();
    let output = dir.join("out.raw");
    let summary = dir.join("summary.json");
    for _ in 0..2 {
        // The second run replaces the first one's store.
        run(&[
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "--summary",
            summary.to_str().unwrap(),
        ]);
    }
    let store = dir.join("out.rsr");
    assert!(store.join("manifest.rsm").is_file());
    assert!(store.join("runs/000000.rsr").is_file());
    let leftovers = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(".rspice-store")
        })
        .count();
    assert_eq!(leftovers, 0, "no store is left under its private name");

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).unwrap()).unwrap();
    // The summary has listed its stores since schema 5.
    assert!(json["schema_version"].as_u64().unwrap() >= 5, "{json}");
    let stores = json["stores"].as_array().unwrap();
    assert_eq!(stores.len(), 1, "{json}");
    assert_eq!(stores[0]["path"], store.display().to_string());
    assert!(stores[0]["rows"].as_u64().unwrap() > 400);
    assert_eq!(stores[0]["bytes"].as_u64().unwrap(), {
        std::fs::metadata(store.join("runs/000000.rsr"))
            .unwrap()
            .len()
    });
    assert_eq!(stores[0]["digest"].as_str().unwrap().len(), 64);
}

#[test]
fn no_store_deletes_the_store_once_the_output_is_written() {
    let dir = test_dir("results-store-deleted");
    let deck = dir.join("rc.cir");
    std::fs::write(&deck, RC).unwrap();
    let output = dir.join("out.csv");
    let summary = dir.join("summary.json");
    run(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-f",
        "csv",
        "-o",
        output.to_str().unwrap(),
        "--no-store",
        "--summary",
        summary.to_str().unwrap(),
    ]);
    assert!(output.is_file());
    let entries = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.contains("rsr"))
        .collect::<Vec<_>>();
    assert!(entries.is_empty(), "{entries:?}");
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).unwrap()).unwrap();
    assert_eq!(json["stores"], serde_json::json!([]));
}

#[test]
fn a_directory_that_is_not_a_store_is_never_replaced() {
    let dir = test_dir("results-store-foreign");
    let deck = dir.join("rc.cir");
    std::fs::write(&deck, RC).unwrap();
    let foreign = dir.join("out.rsr");
    std::fs::create_dir_all(&foreign).unwrap();
    std::fs::write(foreign.join("notes.txt"), "mine").unwrap();
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        dir.join("out.raw").to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("not a results store"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(foreign.join("notes.txt")).unwrap(),
        "mine"
    );
}
