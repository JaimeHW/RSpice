//! AC/noise DATA rows must survive normal runs, control runs and conversion.
mod common;
use rspice_core::execution::AnalysisResultDocument;
use std::path::Path;
use std::process::{Command, Output};

const NETWORK: &str = "Table export\n.param resistance=1k\nV1 in 0 AC 1\nR1 in out {resistance}\nC1 out 0 1u\n.data points FREQ resistance\n100 1k\n10 2k\n100 3k\n.enddata\n";

fn run(input: &Path, output: &Path, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(input)
        .arg("-o")
        .arg(output)
        .args(["-f", format])
        .output()
        .unwrap()
}

fn success(output: Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn direct_and_control_tables_publish_the_same_coordinates_and_samples() {
    let directory = common::test_dir("frequency-table-documents");
    for (family, command) in [
        ("ac", "ac data=points"),
        ("noise", "noise V(out) V1 data=points"),
    ] {
        let mut documents = Vec::new();
        for (name, cards) in [
            ("direct", format!(".{command}")),
            ("explicit", format!(".control\n{command}\n.endc")),
            ("run", format!(".{command}\n.control\nrun\n.endc")),
        ] {
            let input = directory.join(format!("{family}-{name}.cir"));
            let output = directory.join(format!("{family}-{name}.json"));
            std::fs::write(&input, format!("{NETWORK}{cards}\n.end\n")).unwrap();
            success(
                Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .args(["--quiet", "check"])
                    .arg(&input)
                    .output()
                    .unwrap(),
            );
            success(run(&input, &output, "json"));
            let published = if name == "direct" {
                output
            } else {
                directory.join(format!("{family}-{name}.{family}-001.json"))
            };
            let document =
                AnalysisResultDocument::from_json(&std::fs::read_to_string(published).unwrap())
                    .unwrap();
            assert!(
                document.frequency_table().is_some(),
                "{family} {name} lost its table coordinates"
            );
            assert_eq!(document.axes().len(), 2);
            documents.push(document);
        }
        for document in &documents[1..] {
            assert_eq!(document.frequency_table(), documents[0].frequency_table());
            assert_eq!(document.axes(), documents[0].axes());
            assert_eq!(document.signals(), documents[0].signals());
            assert_eq!(document.payload(), documents[0].payload());
        }
    }
}

#[test]
fn flat_exports_and_typed_conversion_keep_duplicate_frequency_rows_aligned() {
    let directory = common::test_dir("frequency-table-flat");
    for (family, command) in [
        ("ac", "ac data=points"),
        ("noise", "noise V(out) V1 data=points"),
    ] {
        for control in [false, true] {
            let input = directory.join(format!("{family}-{control}.cir"));
            let cards = if control {
                format!(".control\n{command}\n.endc")
            } else {
                format!(".{command}")
            };
            std::fs::write(&input, format!("{NETWORK}{cards}\n.end\n")).unwrap();
            for (format, extension) in [
                ("csv", "csv"),
                ("tsv", "tsv"),
                ("ascii", "raw"),
                ("raw", "raw"),
                ("hdf5", "h5"),
                ("json", "json"),
            ] {
                let stem = format!("{family}-{control}-{format}");
                let output = directory.join(format!("{stem}.{extension}"));
                success(run(&input, &output, format));
                let published = if control {
                    directory.join(format!("{stem}.{family}-001.{extension}"))
                } else {
                    output
                };
                let converted = directory.join(format!("{stem}-converted.csv"));
                success(
                    Command::new(env!("CARGO_BIN_EXE_rspice"))
                        .args(["--quiet", "convert"])
                        .arg(&published)
                        .arg(&converted)
                        .args(["--to", "csv"])
                        .output()
                        .unwrap(),
                );
                let csv = std::fs::read_to_string(&converted).unwrap();
                let mut lines = csv.lines();
                let header = lines.next().unwrap().split(',').collect::<Vec<_>>();
                let parameter = header
                    .iter()
                    .position(|name| {
                        name.eq_ignore_ascii_case("data(resistance)")
                            || name.eq_ignore_ascii_case("Re(data(resistance))")
                    })
                    .unwrap_or_else(|| panic!("{csv}"));
                let rows = lines
                    .map(|line| {
                        line.split(',')
                            .map(|value| value.parse::<f64>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                assert_eq!(rows.len(), 3, "{stem}: {csv}");
                for (row, (frequency, resistance)) in
                    rows.iter()
                        .zip([(100.0, 1000.0), (10.0, 2000.0), (100.0, 3000.0)])
                {
                    assert_eq!(row[0], frequency, "{stem}");
                    assert_eq!(row[parameter], resistance, "{stem}");
                }
                if format == "json" {
                    success(
                        Command::new(env!("CARGO_BIN_EXE_rspice"))
                            .args(["--quiet", "convert"])
                            .arg(&published)
                            .arg(&converted)
                            .args(["--to", "csv", "--start", "50", "--stop", "200"])
                            .output()
                            .unwrap(),
                    );
                    let clipped = std::fs::read_to_string(&converted).unwrap();
                    let rows = clipped
                        .lines()
                        .skip(1)
                        .map(|line| {
                            line.split(',')
                                .map(|value| value.parse::<f64>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(rows.len(), 2);
                    assert_eq!(rows[0][parameter], 1000.0);
                    assert_eq!(rows[1][parameter], 3000.0);
                }
            }
        }
    }
}

#[test]
fn compare_detects_changed_row_bindings_even_when_the_response_is_identical() {
    let directory = common::test_dir("frequency-table-compare");
    let input = directory.join("input.cir");
    let reference = directory.join("reference.json");
    let changed = directory.join("changed.json");
    std::fs::write(&input, format!("{NETWORK}.ac data=points\n.end\n")).unwrap();
    success(run(&input, &reference, "json"));
    let mut wire: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&reference).unwrap()).unwrap();
    wire["axes"][1]["values"]["values"][1] = 2500.into();
    std::fs::write(&changed, serde_json::to_vec(&wire).unwrap()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(&changed)
        .arg(&reference)
        .arg("--json")
        .output()
        .unwrap();
    assert!(!result.status.success());
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert!(report.to_string().contains("data(resistance)"), "{report}");
}

#[test]
fn table_execution_uses_compact_coordinates_instead_of_retaining_every_row_netlist() {
    let directory = common::test_dir("frequency-table-compact");
    let input = directory.join("input.cir");
    let config = directory.join("config.toml");
    let output = directory.join("output.json");
    let rows = "100 1k\n".repeat(128);
    std::fs::write(&input, format!("Compact table\n.param resistance=1k\nV1 out 0 AC 1\nR1 out 0 {{resistance}}\n.data points FREQ resistance\n{rows}.enddata\n.ac data=points\n.end\n")).unwrap();
    std::fs::write(&config, "[resources]\nmax_expanded_source_bytes = 16384\n").unwrap();
    success(
        Command::new(env!("CARGO_BIN_EXE_rspice"))
            .arg("--config")
            .arg(&config)
            .args(["--quiet", "run"])
            .arg(&input)
            .arg("-o")
            .arg(&output)
            .args(["-f", "json"])
            .output()
            .unwrap(),
    );
    let document =
        AnalysisResultDocument::from_json(&std::fs::read_to_string(output).unwrap()).unwrap();
    assert_eq!(document.point_count(), 128);
}

#[test]
fn a_later_failed_table_rolls_back_every_control_artifact() {
    let directory = common::test_dir("frequency-table-rollback");
    let input = directory.join("input.cir");
    let output = directory.join("output.json");
    let previous = directory.join("output.op-001.json");
    std::fs::write(
        &input,
        format!("{NETWORK}.control\nop\nac data=missing\n.endc\n.end\n"),
    )
    .unwrap();
    std::fs::write(&previous, "prior artifact").unwrap();
    assert!(!run(&input, &output, "json").status.success());
    assert_eq!(std::fs::read_to_string(previous).unwrap(), "prior artifact");
    let files = std::fs::read_dir(&directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    assert_eq!(files.len(), 2, "{files:?}");
}

#[test]
fn changing_resistance_and_temperature_rows_match_circuit_equations() {
    use rspice_core::execution::result_document::SeriesValues;
    let directory = common::test_dir("frequency-table-physics");
    let network = "Thermal table\n.param resistance=1k\n.options temp=27\nV1 in 0 AC 1\nR1 in out {resistance} tc1=.01 tnom=27\nC1 out 0 1u\n.data points resistance FREQ TEMP\n1k 100 27\n2k 10 127\n3k 100 77\n.enddata\n";
    let close = |actual: f64, expected: f64| {
        assert!(
            (actual - expected).abs() <= expected.abs() * 1e-9 + 1e-30,
            "{actual:e} != {expected:e}"
        );
    };
    for (family, command) in [
        ("ac", "ac data=points"),
        ("noise", "noise V(out) V1 data=points"),
    ] {
        for control in [false, true] {
            let stem = format!("{family}-{control}");
            let input = directory.join(format!("{stem}.cir"));
            let output = directory.join(format!("{stem}.json"));
            let cards = if control {
                format!(".control\n{command}\n.endc")
            } else {
                format!(".{command}")
            };
            std::fs::write(&input, format!("{network}{cards}\n.end\n")).unwrap();
            success(run(&input, &output, "json"));
            let path = if control {
                directory.join(format!("{stem}.{family}-001.json"))
            } else {
                output
            };
            let document =
                AnalysisResultDocument::from_json(&std::fs::read_to_string(path).unwrap()).unwrap();
            let temperature = document
                .axes()
                .iter()
                .find(|axis| axis.name() == "data(temp)")
                .unwrap();
            assert_eq!(temperature.unit().symbol(), "degC");
            let voltage = document
                .signals()
                .iter()
                .find(|signal| signal.descriptor().canonical_name() == "v(out)")
                .unwrap();
            let SeriesValues::Complex { samples } = voltage.values() else {
                panic!("voltage phasors");
            };
            for (index, (frequency, resistance, temp)) in [
                (100.0, 1000.0, 27.0),
                (10.0, 2000.0, 127.0),
                (100.0, 3000.0, 77.0),
            ]
            .into_iter()
            .enumerate()
            {
                let r = resistance * (1.0 + 0.01 * (temp - 27.0));
                let wrc = std::f64::consts::TAU * frequency * r * 1e-6;
                let sample = samples[index].unwrap();
                close(sample.real, 1.0 / (1.0 + wrc * wrc));
                close(sample.imaginary, -wrc / (1.0 + wrc * wrc));
                if family == "noise" {
                    for (name, expected) in [
                        ("inoise_spectrum", 4.0 * 1.380649e-23 * (temp + 273.15) * r),
                        (
                            "onoise_spectrum",
                            4.0 * 1.380649e-23 * (temp + 273.15) * r / (1.0 + wrc * wrc),
                        ),
                    ] {
                        let signal = document
                            .signals()
                            .iter()
                            .find(|signal| signal.descriptor().canonical_name() == name)
                            .unwrap();
                        let SeriesValues::Real { samples } = signal.values() else {
                            panic!("noise power densities");
                        };
                        close(samples[index].unwrap(), expected);
                    }
                }
            }
        }
    }
}

#[test]
fn noise_integrates_only_a_frequency_band_with_constant_physical_coordinates() {
    let directory = common::test_dir("frequency-table-noise-band");
    for (index, (rows, integrates)) in [
        ("10 1k\n100 1k", true),
        ("10 1k\n100 2k", false),
        ("10 1k", false),
        ("10 1k\n10 1k", false),
        ("100 1k\n10 1k", false),
    ]
    .into_iter()
    .enumerate()
    {
        for control in [false, true] {
            let input = directory.join(format!("band-{index}-{control}.cir"));
            let command = "noise V(out) V1 data=points";
            let cards = if control {
                format!(".control\n{command}\n.endc")
            } else {
                format!(".{command}")
            };
            std::fs::write(&input, format!("Noise band\n.param resistance=1k\nV1 in 0 AC 1\nR1 in out {{resistance}}\nC1 out 0 1u\n.data points FREQ resistance\n{rows}\n.enddata\n{cards}\n.end\n")).unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .arg("run")
                .arg(&input)
                .output()
                .unwrap();
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            success(output);
            assert_eq!(
                stdout.contains("Total integrated output noise:"),
                integrates,
                "{stdout}"
            );
            assert_eq!(
                stdout.contains("Total-noise integration unavailable:"),
                !integrates,
                "{stdout}"
            );
        }
    }
}
