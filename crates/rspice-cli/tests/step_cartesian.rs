//! Public-CLI contracts for `.STEP` as a Cartesian decorator around authored
//! analyses. Every coordinate owns a complete child analysis and a distinct
//! output artifact; the first authored STEP dimension varies fastest.

mod common;

use common::{AxisRunSet, test_dir};

use std::path::{Path, PathBuf};
use std::process::Command;

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

/// The artifact one coordinate of a completed axis run published.
///
/// The coordinate identity and the artifact name both come from the run-set
/// manifest the run committed after its last coordinate, so this never has to
/// reproduce the coordinate-tag sanitizer or the namespace composition rule.
fn step_output(base: &Path, one_based_index: usize) -> PathBuf {
    AxisRunSet::read(base)
        .coordinate(one_based_index)
        .only_artifact()
        .to_path_buf()
}

/// The artifact one coordinate published for one analysis identity.
fn step_analysis_output(base: &Path, one_based_index: usize, analysis_id: &str) -> PathBuf {
    AxisRunSet::read(base)
        .coordinate(one_based_index)
        .artifact(analysis_id)
        .to_path_buf()
}

/// How many coordinates the run published, or none when it committed no set.
fn published_coordinate_count(base: &Path) -> usize {
    AxisRunSet::try_read(base).map_or(0, |set| set.coordinates.len())
}

/// Every artifact in the output directory, sorted, so a test can assert that
/// the run published exactly the set its own manifest declares.
fn directory_contents(directory: &Path) -> Vec<PathBuf> {
    let mut files = std::fs::read_dir(directory)
        .expect("list output directory")
        .map(|entry| entry.expect("directory entry").path())
        .collect::<Vec<_>>();
    files.sort();
    files
}

/// Files beside a checkpoint destination, which is not an artifact the run-set
/// manifest names.
fn checkpoint_siblings(checkpoint: &Path) -> Vec<PathBuf> {
    let stem = checkpoint.file_stem().expect("checkpoint stem");
    directory_contents(checkpoint.parent().expect("checkpoint parent"))
        .into_iter()
        .filter(|path| {
            path != checkpoint
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(&format!("{}.", stem.to_string_lossy())))
        })
        .collect()
}

fn csv_column(csv: &str, name: &str) -> Vec<f64> {
    let mut lines = csv.lines();
    let header = lines.next().expect("CSV header");
    let index = header
        .split(',')
        .position(|column| column.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing {name} in {header}"));
    lines
        .map(|line| {
            line.split(',')
                .nth(index)
                .expect("CSV value")
                .parse()
                .expect("numeric CSV value")
        })
        .collect()
}

fn optional_scalar_csv_value(csv: &str, name: &str) -> Option<f64> {
    csv.lines().skip(1).find_map(|line| {
        let (signal, value) = line.split_once(',')?;
        signal
            .eq_ignore_ascii_case(name)
            .then(|| value.parse().expect("numeric scalar CSV value"))
    })
}

fn scalar_csv_value(csv: &str, name: &str) -> f64 {
    optional_scalar_csv_value(csv, name).unwrap_or_else(|| panic!("missing {name} in {csv}"))
}

fn complex_csv_value_at(csv: &str, name: &str, frequency: f64) -> (f64, f64) {
    let frequencies = csv_column(csv, "frequency");
    let real = csv_column(csv, &format!("Re({name})"));
    let imaginary = csv_column(csv, &format!("Im({name})"));
    let row = frequencies
        .iter()
        .position(|value| (*value - frequency).abs() <= frequency.abs().max(1.0) * 1.0e-12)
        .unwrap_or_else(|| panic!("missing {frequency} Hz row in {csv}"));
    (real[row], imaginary[row])
}

fn complex_csv_magnitude_at(csv: &str, name: &str, frequency: f64) -> f64 {
    let (real, imaginary) = complex_csv_value_at(csv, name, frequency);
    real.hypot(imaginary)
}

#[derive(Debug)]
struct ConditionalCoordinateResult {
    out: f64,
    extra: Option<f64>,
    coordinate_id: String,
    topology_fingerprint: String,
}

fn assert_conditional_implicit_step_topology_is_typed(
    tag: &str,
    values: &str,
) -> std::collections::BTreeMap<i32, ConditionalCoordinateResult> {
    let dir = test_dir(tag);
    let deck = dir.join("conditional_topology.sp");
    let output_path = dir.join("conditional_topology.csv");
    std::fs::write(
        &deck,
        format!(
            "* Conditional topology must not inherit the first coordinate schema\n\
             .param mode=0\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             .if (mode==1)\n\
             R2 out extra 1k\n\
             R3 extra 0 1k\n\
             .else\n\
             R4 out 0 1k\n\
             .endif\n\
             .step param mode list {values}\n\
             .end\n"
        ),
    )
    .expect("write conditional-topology deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "conditional topology must execute with coordinate-local schemas; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        !output_path.exists(),
        "changing topology must not reuse an unqualified wide artifact"
    );

    let modes = values
        .split_ascii_whitespace()
        .map(|value| value.parse::<i32>().expect("integer mode"))
        .collect::<Vec<_>>();
    let manifest_path = output_path.with_file_name("conditional_topology.step_schema.json");
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&manifest_path).expect("conditional schema manifest must exist"),
    )
    .expect("conditional schema manifest must be JSON");
    assert_eq!(manifest["aggregation"], "coordinate_local");
    assert_eq!(manifest["missingness"], "union_validity_bitmap");
    // The manifest groups its union by analysis instance, so a deck whose
    // coordinates publish several analyses keeps them apart.
    let analyses = manifest["analyses"].as_array().expect("manifest analyses");
    assert_eq!(analyses.len(), 1);
    let entry = &analyses[0];
    assert_eq!(entry["analysis_id"], "implicit-op-001");
    let union_schema = entry["union_schema"]
        .as_array()
        .expect("union schema array");
    let extra_index = union_schema
        .iter()
        .position(|descriptor| {
            descriptor["display_name"]
                .as_str()
                .is_some_and(|name| name.eq_ignore_ascii_case("V(extra)"))
        })
        .expect("union schema retains conditional V(extra)");
    let manifest_coordinates = entry["coordinates"]
        .as_array()
        .expect("coordinate metadata array");
    assert_eq!(manifest_coordinates.len(), modes.len());

    let mut semantic = std::collections::BTreeMap::new();
    for (index, mode) in modes.into_iter().enumerate() {
        let path = step_output(&output_path, index + 1);
        assert_eq!(
            metadata_artifact(&manifest_coordinates[index]),
            path.file_name()
                .and_then(std::ffi::OsStr::to_str)
                .expect("portable coordinate artifact filename"),
            "manifest must identify the coordinate-local artifact by portable filename"
        );
        let csv = std::fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("coordinate artifact {} is missing: {error}", path.display())
        });
        let out = scalar_csv_value(&csv, "V(out)");
        let extra = optional_scalar_csv_value(&csv, "V(extra)");
        let metadata = &manifest_coordinates[index];
        let validity = metadata["validity"]
            .as_array()
            .expect("coordinate validity bitmap");
        assert_eq!(
            validity[extra_index].as_bool(),
            Some(mode == 1),
            "V(extra) validity must follow its coordinate topology"
        );
        assert_eq!(
            extra.is_some(),
            mode == 1,
            "a missing conditional node must be absent, never fabricated as zero"
        );
        if mode == 0 {
            assert!((out - 0.5).abs() < 1.0e-12);
        } else {
            assert!((out - 2.0 / 3.0).abs() < 1.0e-12);
            assert!((extra.expect("mode 1 has V(extra)") - 1.0 / 3.0).abs() < 1.0e-12);
        }
        semantic.insert(
            mode,
            ConditionalCoordinateResult {
                out,
                extra,
                coordinate_id: metadata["coordinate_id"]
                    .as_str()
                    .expect("coordinate ID")
                    .to_string(),
                topology_fingerprint: metadata["topology_fingerprint"]
                    .as_str()
                    .expect("topology fingerprint")
                    .to_string(),
            },
        );
    }
    assert_ne!(
        semantic[&0].topology_fingerprint, semantic[&1].topology_fingerprint,
        "conditional component/node membership must change topology identity"
    );
    assert_eq!(
        published_coordinate_count(&output_path),
        modes_len(values),
        "only planned coordinate artifacts may be published"
    );

    let _ = std::fs::remove_dir_all(&dir);
    semantic
}

fn tagged_output(base: &Path, tag: &str) -> PathBuf {
    let stem = base.file_stem().expect("output stem").to_string_lossy();
    base.with_file_name(format!(
        "{stem}.{tag}.{}",
        base.extension()
            .expect("output extension")
            .to_string_lossy()
    ))
}

fn metadata_artifact(metadata: &serde_json::Value) -> &str {
    metadata["artifact"]
        .as_str()
        .expect("coordinate artifact path")
}

fn modes_len(values: &str) -> usize {
    values.split_ascii_whitespace().count()
}

#[test]
fn conditional_implicit_step_topology_is_order_independent_and_never_fabricates_data() {
    let forward = assert_conditional_implicit_step_topology_is_typed("topology_forward", "0 1");
    let reverse = assert_conditional_implicit_step_topology_is_typed("topology_reverse", "1 0");

    for mode in [0, 1] {
        assert!((forward[&mode].out - reverse[&mode].out).abs() < 1.0e-15);
        assert_eq!(forward[&mode].extra, reverse[&mode].extra);
        assert_eq!(
            forward[&mode].coordinate_id, reverse[&mode].coordinate_id,
            "semantic coordinate identity must not depend on enumeration order"
        );
        assert_eq!(
            forward[&mode].topology_fingerprint, reverse[&mode].topology_fingerprint,
            "the same materialized topology must have the same fingerprint"
        );
    }
}

#[test]
fn two_step_dimensions_wrap_the_complete_dc_sweep_in_xyce_order() {
    let dir = test_dir("dc");
    let deck = dir.join("cartesian.sp");
    let output_path = dir.join("sweep.csv");
    std::fs::write(
        &deck,
        "* Cartesian STEP around DC\n\
         V1 in 0 0\n\
         R1 in out {rleft}\n\
         R2 out 0 {rright}\n\
         .param rleft=1k rright=1k\n\
         .step param rleft list 1k 2k\n\
         .step param rright list 1k 2k 3k\n\
         .dc V1 0 1 1\n\
         .end\n",
    )
    .expect("write Cartesian deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        !output_path.exists(),
        "the unqualified path must not be reused"
    );

    let expected_vout = [0.5, 1.0 / 3.0, 2.0 / 3.0, 0.5, 0.75, 0.6];
    for (index, expected) in expected_vout.into_iter().enumerate() {
        let path = step_output(&output_path, index + 1);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let source = csv_column(&csv, "V1");
        let vout = csv_column(&csv, "V(OUT)");
        assert_eq!(source, vec![0.0, 1.0], "complete DC grid in {csv}");
        assert_eq!(vout.len(), 2, "one full child sweep in {csv}");
        assert!(
            (vout[1] - expected).abs() < 1.0e-9,
            "coordinate {} violates first-dimension-fastest order: {csv}",
            index + 1
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn canonical_data_step_temp_order_drives_cli_coordinate_namespaces() {
    let dir = test_dir("canonical_axis_order");
    let deck = dir.join("canonical_axis_order.sp");
    let output_path = dir.join("operating_point.csv");
    std::fs::write(
        &deck,
        "* DeckPlan orders DATA, numeric STEP, then TEMP\n\
         .param rleft=1k rright=1k\n\
         V1 in 0 1\n\
         R1 in out {rleft}\n\
         R2 out 0 {rright}\n\
         .step param rright list 1k 2k\n\
         .temp 25 75\n\
         .data left_values rleft\n\
         1k\n\
         3k\n\
         .enddata\n\
         .step data=left_values\n\
         .op\n\
         .end\n",
    )
    .expect("write mixed canonical-axis deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists(), "base path must remain unused");

    // DATA is canonical axis 0 and therefore varies fastest, even though its
    // card appeared after the numeric STEP and TEMP cards in source order.
    let expected = [0.5, 0.25, 2.0 / 3.0, 0.4, 0.5, 0.25, 2.0 / 3.0, 0.4];
    for (index, expected_vout) in expected.into_iter().enumerate() {
        let path = step_output(&output_path, index + 1);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let vout = scalar_csv_value(&csv, "V(OUT)");
        assert!(
            (vout - expected_vout).abs() < 1.0e-9,
            "coordinate {} is not in canonical DATA→STEP→TEMP order: {csv}",
            index + 1
        );
    }
    assert_eq!(published_coordinate_count(&output_path), 8);

    let _ = std::fs::remove_dir_all(&dir);
}

fn verbose_coordinate_ids(stdout: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(stdout)
        .lines()
        .filter(|line| line.starts_with("=== run-"))
        .map(|line| {
            line.split_ascii_whitespace()
                .nth(1)
                .unwrap_or_else(|| panic!("missing canonical run ID in '{line}'"))
                .to_string()
        })
        .collect()
}

#[test]
fn cli_reports_deterministic_canonical_coordinate_ids() {
    let dir = test_dir("canonical_ids");
    let deck = dir.join("canonical_ids.sp");
    std::fs::write(
        &deck,
        "* canonical coordinate identity\n\
         .param a=1 b=10\n\
         V1 out 0 1\n\
         R1 out 0 1k\n\
         .step param a list 1 2\n\
         .step param b list 10 20\n\
         .op\n\
         .end\n",
    )
    .expect("write canonical-ID deck");

    let first = run_rspice(&["--verbose", "run", deck.to_str().unwrap()]);
    let second = run_rspice(&["--verbose", "run", deck.to_str().unwrap()]);
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(
        second.status.success(),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    let first_ids = verbose_coordinate_ids(&first.stdout);
    let second_ids = verbose_coordinate_ids(&second.stdout);
    assert_eq!(
        first_ids.len(),
        4,
        "{}",
        String::from_utf8_lossy(&first.stdout)
    );
    assert_eq!(first_ids, second_ids, "coordinate IDs changed across runs");
    let unique = first_ids.iter().collect::<std::collections::HashSet<_>>();
    assert_eq!(unique.len(), 4, "every Cartesian coordinate needs one ID");
    assert!(
        first_ids.iter().all(|id| id.ends_with("-001")),
        "distinct semantic coordinates must use occurrence 1: {first_ids:?}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_materializer_bit_matches_canonical_lin_dec_oct_coordinates() {
    let dir = test_dir("canonical_numeric_generators");
    let deck = dir.join("canonical_numeric_generators.sp");
    std::fs::write(
        &deck,
        "* exercise intermediate LIN/DEC/OCT values through both planners\n\
         .param linear=0.1 decade=1 octave=1\n\
         V1 out 0 1\n\
         R1 out 0 1k\n\
         .step lin param linear 0.1 0.3 0.1\n\
         .step dec param decade 1 10 2\n\
         .step oct param octave 1 2 2\n\
         .op\n\
         .end\n",
    )
    .expect("write canonical numeric-generator deck");

    // The CLI validates every materialized value against the canonical
    // coordinate with exact f64 equality. Intermediate logarithmic values
    // make this a regression against independently rounded generators.
    let run = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn femto_step_runs_exactly_six_fresh_transient_analyses() {
    let dir = test_dir("tran");
    let deck = dir.join("transient.sp");
    let output_path = dir.join("wave.csv");
    std::fs::write(
        &deck,
        "* Femto STEP around transient\n\
         V1 in 0 PULSE(0 1 0 10p 10p 1n 2n)\n\
         R1 in out 1k\n\
         C1 out 0 {cap}\n\
         .param cap=100f\n\
         .step param cap 100f 150f 10f\n\
         .tran 50p 500p\n\
         .end\n",
    )
    .expect("write transient deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    for index in 1..=6 {
        let path = step_output(&output_path, index);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let time = csv_column(&csv, "time");
        assert_eq!(time.first().copied(), Some(0.0), "fresh transient: {csv}");
        assert!(time.last().is_some_and(|value| *value >= 500.0e-12));
    }
    assert_eq!(
        published_coordinate_count(&output_path),
        6,
        "the femto grid must contain exactly six coordinates"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn authored_hb_runs_once_per_step_coordinate_with_distinct_spectra() {
    let dir = test_dir("hb");
    let deck = dir.join("harmonic_balance.sp");
    let output_path = dir.join("spectrum.csv");
    std::fs::write(
        &deck,
        "* STEP around authored harmonic balance\n\
         .param amplitude=100m\n\
         V1 out 0 SIN(0 {amplitude} 1meg)\n\
         R1 out 0 1k\n\
         .step param amplitude list 100m 300m\n\
         .hb 1meg\n\
         .print hb V(out) I(V1)\n\
         .end\n",
    )
    .expect("write stepped HB deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        !output_path.exists(),
        "the unqualified path must not be reused"
    );

    for (index, expected) in [0.1, 0.3].into_iter().enumerate() {
        let path = step_output(&output_path, index + 1);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let fundamental = complex_csv_magnitude_at(&csv, "V(OUT)", 1.0e6);
        assert!(
            (fundamental - expected).abs() <= expected * 1.0e-8,
            "coordinate {} must bind amplitude={expected}: {csv}",
            index + 1
        );
        let (voltage_real, voltage_imaginary) = complex_csv_value_at(&csv, "V(OUT)", 1.0e6);
        let (current_real, current_imaginary) = complex_csv_value_at(&csv, "I(V1)", 1.0e6);
        let current_magnitude = current_real.hypot(current_imaginary);
        assert!(
            (current_magnitude - expected / 1_000.0).abs() <= expected * 1.0e-11,
            "coordinate {} must retain the V1 branch-current spectrum: {csv}",
            index + 1
        );
        assert!(
            (voltage_real + 1_000.0 * current_real).abs() <= expected * 1.0e-8
                && (voltage_imaginary + 1_000.0 * current_imaginary).abs() <= expected * 1.0e-8,
            "coordinate {} must satisfy V(out)/R1 + I(V1) = 0: {csv}",
            index + 1
        );
    }
    assert_eq!(
        published_coordinate_count(&output_path),
        2,
        "the bounded plan must contain exactly two HB coordinates"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn repeated_ac_cards_use_analysis_ordinals_inside_each_step_coordinate() {
    let dir = test_dir("repeated_ac");
    let deck = dir.join("repeated_ac.sp");
    let output_path = dir.join("response.csv");
    std::fs::write(
        &deck,
        "* Repeated analyses need coordinate and analysis identities\n\
         .param resistance=1k\n\
         V1 in 0 AC 1\n\
         R1 in out {resistance}\n\
         C1 out 0 1n\n\
         .step param resistance list 1k 2k\n\
         .ac lin 1 1k 1k\n\
         .ac lin 1 10k 10k\n\
         .print ac V(out)\n\
         .end\n",
    )
    .expect("write repeated-AC deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    for coordinate in 1..=2 {
        let first = step_analysis_output(&output_path, coordinate, "ac-001");
        let second = step_analysis_output(&output_path, coordinate, "ac-002");
        let first_csv = std::fs::read_to_string(&first)
            .unwrap_or_else(|error| panic!("read {}: {error}", first.display()));
        let second_csv = std::fs::read_to_string(&second)
            .unwrap_or_else(|error| panic!("read {}: {error}", second.display()));
        assert_eq!(csv_column(&first_csv, "frequency"), vec![1.0e3]);
        assert_eq!(csv_column(&second_csv, "frequency"), vec![1.0e4]);
    }
    // Every artifact the run published is analysis-qualified: an artifact
    // named by the coordinate alone would let two `.AC` cards of one
    // coordinate overwrite each other.
    let run_set = AxisRunSet::read(&output_path);
    assert_eq!(
        directory_contents(&dir),
        {
            let mut expected = run_set.artifacts();
            expected.push(deck.clone());
            expected.push(common::axis_run_set_path(&output_path));
            expected.push(common::axis_schema_manifest_path(&output_path));
            expected.sort();
            expected
        },
        "an artifact outside the published coordinate set was written"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn authored_noise_runs_once_per_step_coordinate() {
    let dir = test_dir("noise");
    let deck = dir.join("noise.sp");
    let output_path = dir.join("noise.csv");
    std::fs::write(
        &deck,
        "* STEP around authored noise analysis\n\
         .param resistance=1k\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out {resistance}\n\
         C1 out 0 1n\n\
         .step param resistance list 1k 2k\n\
         .noise V(out) V1 lin 1 1k 1k\n\
         .end\n",
    )
    .expect("write stepped-noise deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    for coordinate in 1..=2 {
        let path = step_output(&output_path, coordinate);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        assert_eq!(csv_column(&csv, "frequency"), vec![1.0e3]);
        let onoise = csv_column(&csv, "onoise_spectrum");
        assert_eq!(onoise.len(), 1);
        assert!(onoise[0].is_finite() && onoise[0] > 0.0, "{csv}");
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn conditional_hb_signature_change_fails_before_any_step_output() {
    let dir = test_dir("conditional_hb");
    let deck = dir.join("conditional_hb.sp");
    let output_path = dir.join("conditional_hb.csv");
    std::fs::write(
        &deck,
        "* STEP must not conditionally add harmonic balance\n\
         .param mode=0\n\
         V1 out 0 SIN(0 100m 1meg)\n\
         R1 out 0 1k\n\
         .step param mode list 0 1\n\
         .if (mode==1)\n\
         .hb 1meg\n\
         .endif\n\
         .end\n",
    )
    .expect("write conditional HB deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert_eq!(
        run.status.code(),
        Some(85),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        String::from_utf8_lossy(&run.stderr)
            .contains("must be unconditional across every coordinate"),
        "stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists());
    assert_eq!(published_coordinate_count(&output_path), 0);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn default_ngspice_cli_does_not_apply_xyce_print_voltage_wildcard() {
    let dir = test_dir("ngspice_tran_print_vstar");
    let deck = dir.join("transient_vstar.sp");
    let output_path = dir.join("wave.csv");
    std::fs::write(
        &deck,
        "* STEP transient PRINT V(*)\n\
         VDrive Supply 0 PULSE(0 1 0 1p 1p 20p 50p)\n\
         RLoad Supply Load {resistance}\n\
         CLoad Load 0 1p\n\
         .param resistance=1k\n\
         .step param resistance list 1k 2k\n\
         .tran 5p 20p\n\
         .print tran V(Load) V(*)\n\
         .end\n",
    )
    .expect("write stepped wildcard deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        !run.status.success(),
        "default CLI mode must not reinterpret ngspice V(*) as Xyce: stdout={}; stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        String::from_utf8_lossy(&run.stderr).contains("V(*)"),
        "failure must retain the unsupported authored probe: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists());
    assert_eq!(published_coordinate_count(&output_path), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn step_data_table_reaches_the_typed_planner_and_binds_each_row() {
    let dir = test_dir("data");
    let deck = dir.join("data_step.sp");
    let output_path = dir.join("operating_point.csv");
    std::fs::write(
        &deck,
        "* Typed STEP DATA ownership\n\
         V1 in 0 1\n\
         R1 in out {rleft}\n\
         R2 out 0 1k\n\
         .param rleft=1k\n\
         .data coordinates rleft\n\
         1k\n\
         3k\n\
         .enddata\n\
         .step data=coordinates\n\
         .op\n\
         .end\n",
    )
    .expect("write STEP DATA deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    for (index, expected) in [0.5, 0.25].into_iter().enumerate() {
        let path = step_output(&output_path, index + 1);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let vout = scalar_csv_value(&csv, "V(OUT)");
        assert!((vout - expected).abs() < 1.0e-9, "{csv}");
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn conditional_child_analysis_change_fails_before_any_step_output() {
    let dir = test_dir("conditional_analysis");
    let deck = dir.join("conditional.sp");
    let output_path = dir.join("conditional.csv");
    std::fs::write(
        &deck,
        "* STEP must not change its child analysis set\n\
         .param mode=0\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .step param mode list 0 1\n\
         .if (mode==1)\n\
         .tran 1n 2n\n\
         .endif\n\
         .end\n",
    )
    .expect("write conditional deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert_eq!(
        run.status.code(),
        Some(85),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        String::from_utf8_lossy(&run.stderr)
            .contains("must be unconditional across every coordinate"),
        "stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists());
    assert_eq!(published_coordinate_count(&output_path), 0);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn outer_alter_and_inner_step_compose_unique_output_namespaces() {
    let dir = test_dir("alter_preflight");
    let deck = dir.join("alter_step.sp");
    let output_path = dir.join("alter_step.csv");
    std::fs::write(
        &deck,
        "* outer ALTER must be globally preflighted\n\
         .param r=1k\n\
         V1 in 0 1\n\
         R1 in 0 {r}\n\
         .op\n\
         .alter stepped\n\
         .step param r list 1k 2k\n\
         .end\n",
    )
    .expect("write ALTER STEP deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "outer/inner run axes must compose; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists());
    let base_output = tagged_output(&output_path, "base");
    assert!(base_output.exists(), "missing base output");
    let mut step_outputs = std::fs::read_dir(&dir)
        .expect("read ALTER output directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.starts_with("alter_step.stepped_run_") && name.ends_with(".op-001.csv")
                })
        })
        .collect::<Vec<_>>();
    step_outputs.sort();
    assert_eq!(step_outputs.len(), 2, "stable inner namespaces");
    let base_current = scalar_csv_value(
        &std::fs::read_to_string(base_output).expect("base output"),
        "I(V1)",
    );
    let mut step_currents = step_outputs
        .iter()
        .map(|path| {
            scalar_csv_value(
                &std::fs::read_to_string(path).expect("STEP output"),
                "I(V1)",
            )
        })
        .collect::<Vec<_>>();
    step_currents.sort_by(f64::total_cmp);
    assert!((base_current + 1.0e-3).abs() < 1.0e-12);
    assert!((step_currents[0] + 1.0e-3).abs() < 1.0e-12);
    assert!((step_currents[1] + 0.5e-3).abs() < 1.0e-12);
    assert_eq!(
        std::fs::read_dir(&dir)
            .expect("read test directory")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "csv"))
            .count(),
        3,
        "one outer base artifact and two inner coordinate artifacts are required"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn outer_alter_step_cross_product_obeys_one_global_batch_budget() {
    let dir = test_dir("alter_step_batch_limit");
    let deck = dir.join("bounded_alter_step.sp");
    let config = dir.join("rspice.toml");
    let output_path = dir.join("bounded_alter_step.csv");
    std::fs::write(
        &deck,
        "* aggregate outer/inner batch budget\n\
         .param r=1k\n\
         V1 in 0 1\n\
         R1 in 0 {r}\n\
         .step param r list 1k 2k\n\
         .op\n\
         .alter second\n\
         V1 in 0 2\n\
         .end\n",
    )
    .expect("write bounded ALTER STEP deck");
    std::fs::write(&config, "[resources]\nmax_batch_runs = 3\n").expect("write resource config");

    let run = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--error-format",
        "json",
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert_eq!(run.status.code(), Some(75));
    let diagnostic: serde_json::Value =
        serde_json::from_slice(&run.stderr).expect("structured diagnostic");
    assert_eq!(diagnostic["error"]["code"], "resource_limit");
    assert_eq!(diagnostic["error"]["resource"], "batch_runs");
    assert_eq!(diagnostic["error"]["requested"], 4);
    assert_eq!(diagnostic["error"]["limit"], 3);
    assert_eq!(
        std::fs::read_dir(&dir)
            .expect("read test directory")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "csv"))
            .count(),
        0,
        "aggregate preflight must reject before any outer artifact"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_step_checkpoints_are_coordinate_local_and_resumable() {
    let dir = test_dir("coordinate_checkpoints");
    let deck = dir.join("checkpoint_step.sp");
    let checkpoint = dir.join("state.chk");
    std::fs::write(
        &deck,
        "* coordinate-local transient checkpoints\n\
         .param r=1k\n\
         V1 in 0 PULSE(0 1 0 10p 10p 1n 2n)\n\
         R1 in 0 {r}\n\
         .step param r list 1k 2k\n\
         .tran 100p 2n\n\
         .end\n",
    )
    .expect("write checkpoint STEP deck");

    let initial = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--checkpoint",
        checkpoint.to_str().unwrap(),
    ]);
    assert!(
        initial.status.success(),
        "coordinate checkpoints must save; stderr: {}",
        String::from_utf8_lossy(&initial.stderr)
    );
    // A checkpoint is not a published result, so the run-set manifest does not
    // name it; what matters is that each coordinate wrote its own and none
    // shared the unqualified destination.
    let checkpoints = checkpoint_siblings(&checkpoint);
    assert_eq!(
        checkpoints.len(),
        2,
        "each coordinate must checkpoint separately: {checkpoints:?}"
    );
    assert!(
        !checkpoint.exists(),
        "the unqualified path must not be shared"
    );

    let resumed = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--resume",
        checkpoint.to_str().unwrap(),
        "--tran-stop",
        "3n",
    ]);
    assert!(
        resumed.status.success(),
        "every coordinate must resume from its matching state; stderr: {}",
        String::from_utf8_lossy(&resumed.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn repeated_transient_checkpoints_compose_analysis_and_coordinate_ids() {
    let dir = test_dir("repeated_transient_checkpoints");
    let deck = dir.join("repeated_transient_step.sp");
    let checkpoint = dir.join("state.chk");
    std::fs::write(
        &deck,
        "* coordinate and analysis-local transient checkpoints\n\
         .param r=1k\n\
         V1 in 0 PULSE(0 1 0 10p 10p 500p 1n)\n\
         R1 in 0 {r}\n\
         .step param r list 1k 2k\n\
         .tran 100p 1n\n\
         .tran 50p 1n\n\
         .end\n",
    )
    .expect("write repeated transient STEP deck");

    let initial = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--checkpoint",
        checkpoint.to_str().unwrap(),
    ]);
    assert!(
        initial.status.success(),
        "repeated transient checkpoints must save; stderr: {}",
        String::from_utf8_lossy(&initial.stderr)
    );
    let checkpoints = checkpoint_siblings(&checkpoint);
    assert_eq!(
        checkpoints.len(),
        4,
        "two coordinates times two transients: {checkpoints:?}"
    );
    for analysis in 1..=2 {
        let tag = format!(".tran-{analysis:03}.");
        assert_eq!(
            checkpoints
                .iter()
                .filter(|path| path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.contains(&tag)))
                .count(),
            2,
            "each authored transient checkpoints at each coordinate: {checkpoints:?}"
        );
    }
    assert!(!checkpoint.exists());

    let resumed = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--resume",
        checkpoint.to_str().unwrap(),
        "--tran-stop",
        "2n",
    ]);
    assert!(
        resumed.status.success(),
        "every transient must resume from its matching analysis/coordinate state; stderr: {}",
        String::from_utf8_lossy(&resumed.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn step_batch_limit_retains_typed_resource_metadata() {
    let dir = test_dir("batch_limit");
    let deck = dir.join("bounded.sp");
    let config = dir.join("rspice.toml");
    std::fs::write(
        &deck,
        "* bounded STEP\n\
         .param r=1k\n\
         V1 in 0 1\n\
         R1 in 0 {r}\n\
         .step param r list 1k 2k\n\
         .op\n\
         .end\n",
    )
    .expect("write bounded STEP deck");
    std::fs::write(&config, "[resources]\nmax_batch_runs = 1\n").expect("write resource config");

    let run = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--error-format",
        "json",
        "--quiet",
        "run",
        deck.to_str().unwrap(),
    ]);
    assert_eq!(run.status.code(), Some(75));
    let diagnostic: serde_json::Value =
        serde_json::from_slice(&run.stderr).expect("structured diagnostic");
    assert_eq!(diagnostic["error"]["code"], "resource_limit");
    assert_eq!(diagnostic["error"]["resource"], "batch_runs");
    assert_eq!(diagnostic["error"]["requested"], 2);
    assert_eq!(diagnostic["error"]["limit"], 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn implicit_step_charges_its_single_aggregate_report_once() {
    let dir = test_dir("aggregate_report_limit");
    let deck = dir.join("aggregate.sp");
    let config = dir.join("rspice.toml");
    std::fs::write(
        &deck,
        "* one aggregate report for implicit STEP compatibility\n\
         .param p=0\n\
         .step param p list 1 2 3 4\n\
         .meas op constant param='1'\n\
         .end\n",
    )
    .expect("write aggregate STEP deck");
    std::fs::write(&config, "[resources]\nmax_result_values = 8\n").expect("write resource config");

    let run = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--allow-failed-meas",
    ]);
    assert!(
        run.status.success(),
        "the four OP payloads and single aggregate report fit exactly; stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn stepped_measurement_exports_identify_every_coordinate_run() {
    let dir = test_dir("measurement_identity");
    let deck = dir.join("measured.sp");
    let json_path = dir.join("measurements.json");
    let csv_path = dir.join("measurements.csv");
    std::fs::write(
        &deck,
        "* STEP measurement identity\n\
         .param rleft=1k\n\
         V1 in 0 0\n\
         R1 in out {rleft}\n\
         R2 out 0 1k\n\
         .step param rleft list 1k 2k\n\
         .dc V1 0 1 1\n\
         .meas dc vfinal find V(out) at=1\n\
         .end\n",
    )
    .expect("write measured STEP deck");

    // The run also publishes results, so the coordinate identities the
    // measurement labels must carry can be read out of the run-set manifest
    // rather than re-derived from the plan.
    let output_path = dir.join("measured.csv");
    let json_run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
        "--meas-file",
        json_path.to_str().unwrap(),
    ]);
    assert!(
        json_run.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&json_run.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&json_path).expect("read measurement JSON"))
            .expect("parse measurement JSON");
    let measurements = json["measurements"].as_array().expect("measurement array");
    assert_eq!(measurements.len(), 2);
    let run_set = AxisRunSet::read(&output_path);
    assert_eq!(run_set.coordinates.len(), 2);
    for (measurement, coordinate) in measurements.iter().zip(&run_set.coordinates) {
        let label = measurement["run"].as_str().expect("measurement run label");
        assert!(
            label.ends_with(&format!("[{}]", coordinate.tag)),
            "measurement label {label} does not name coordinate {}",
            coordinate.tag
        );
    }

    let csv_run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--meas-file",
        csv_path.to_str().unwrap(),
        "--meas-format",
        "csv",
    ]);
    assert!(
        csv_run.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&csv_run.stderr)
    );
    let csv = std::fs::read_to_string(&csv_path).expect("read measurement CSV");
    let lines = csv.lines().collect::<Vec<_>>();
    assert_eq!(
        lines[0],
        "netlist,name,value,expected,tolerance,passed,error,run,raw_value,failure_limit,failure_limit_exceeded,record_index,event_axis,trigger_axis,target_axis,aggregate_policy"
    );
    let run_column = lines[0]
        .split(',')
        .position(|column| column == "run")
        .expect("measurement CSV has a run column");
    for (line, coordinate) in lines[1..=2].iter().zip(&run_set.coordinates) {
        let label = line
            .split(',')
            .nth(run_column)
            .expect("measurement CSV run column");
        assert!(
            label.ends_with(&format!("[{}]", coordinate.tag)),
            "measurement label {label} does not name coordinate {}",
            coordinate.tag
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

/// Everything one coordinate of a stepped physical deck published, keyed by
/// the semantic value it was produced at rather than by enumeration position.
#[derive(Debug, PartialEq)]
struct SteppedCoordinateResult {
    coordinate_id: String,
    topology_fingerprint: String,
    /// Union columns this coordinate's artifact actually carried.
    validity: Vec<bool>,
    /// The coordinate's own artifact, verbatim.
    artifact: String,
    /// Whether the conditional column is present in the artifact at all.
    conditional_present: bool,
}

/// Run a stepped physical deck whose topology is conditional on the swept
/// parameter, and return each coordinate's published contract.
///
/// `mode==1` adds a second divider leg and the node `extra`; `mode==0` does
/// not. The signal set therefore differs across coordinates, which is exactly
/// what the union schema and the validity bitmap exist to describe.
fn run_conditional_stepped_analysis(
    tag: &str,
    card: &str,
    values: &str,
    conditional_column: &str,
) -> std::collections::BTreeMap<i32, SteppedCoordinateResult> {
    let dir = test_dir(tag);
    let deck = dir.join(format!("{tag}.sp"));
    std::fs::write(
        &deck,
        format!(
            "* conditional stepped analysis\n\
             .param mode=0\n\
             V1 in 0 SIN(0 1 1k) AC 1\n\
             R1 in out 1k\n\
             C1 out 0 1u\n\
             .if (mode==1)\n\
             R2 out extra 1k\n\
             C2 extra 0 1u\n\
             .endif\n\
             .step param mode list {values}\n\
             {card}\n\
             .end\n"
        ),
    )
    .expect("write conditional stepped deck");

    let requested = dir.join("stepped.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        requested.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "conditional stepped run failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(dir.join("stepped.step_schema.json"))
            .expect("stepped schema manifest must exist"),
    )
    .expect("stepped schema manifest must be JSON");
    assert_eq!(manifest["aggregation"], "coordinate_local");
    assert_eq!(manifest["missingness"], "union_validity_bitmap");
    let analyses = manifest["analyses"].as_array().expect("manifest analyses");
    assert_eq!(analyses.len(), 1, "one authored analysis, one union");
    let entry = &analyses[0];
    let union_schema = entry["union_schema"]
        .as_array()
        .expect("union schema array");
    let conditional_index = union_schema
        .iter()
        .position(|descriptor| {
            descriptor["display_name"]
                .as_str()
                .is_some_and(|name| name.eq_ignore_ascii_case("V(extra)"))
        })
        .expect("union schema dropped the conditional node");

    let modes = values
        .split_ascii_whitespace()
        .map(|value| value.parse::<i32>().expect("integer mode"))
        .collect::<Vec<_>>();
    let coordinates = entry["coordinates"]
        .as_array()
        .expect("manifest coordinates");
    assert_eq!(coordinates.len(), modes.len());

    let mut semantic = std::collections::BTreeMap::new();
    for (index, mode) in modes.into_iter().enumerate() {
        let metadata = &coordinates[index];
        let filename = metadata["artifact"].as_str().expect("artifact filename");
        let artifact =
            std::fs::read_to_string(dir.join(filename)).expect("coordinate artifact must exist");
        let validity = metadata["validity"]
            .as_array()
            .expect("coordinate validity bitmap")
            .iter()
            .map(|value| value.as_bool().expect("boolean validity bit"))
            .collect::<Vec<_>>();
        assert_eq!(validity.len(), union_schema.len());

        let header = artifact.lines().next().expect("artifact header");
        let conditional_present = header
            .split(',')
            .any(|column| column.eq_ignore_ascii_case(conditional_column));
        assert_eq!(
            validity[conditional_index], conditional_present,
            "the validity bitmap must say exactly what the artifact carries"
        );
        assert_eq!(
            conditional_present,
            mode == 1,
            "a signal a conditional removed must be absent, never fabricated as zero"
        );

        semantic.insert(
            mode,
            SteppedCoordinateResult {
                coordinate_id: metadata["coordinate_id"]
                    .as_str()
                    .expect("coordinate id")
                    .to_string(),
                topology_fingerprint: metadata["topology_fingerprint"]
                    .as_str()
                    .expect("topology fingerprint")
                    .to_string(),
                validity,
                artifact,
                conditional_present,
            },
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
    semantic
}

/// A stepped `.TRAN` and a stepped `.AC` must publish the same values, the
/// same coordinate identities, and the same topology fingerprints regardless
/// of the order their axis values were authored in.
#[test]
fn stepped_physical_analyses_are_order_independent_and_never_fabricate_data() {
    for (family, card, conditional) in [
        ("tran", ".tran 100u 400u", "V(extra)"),
        ("ac", ".ac dec 2 100 1k", "Re(V(extra))"),
    ] {
        let forward = run_conditional_stepped_analysis(
            &format!("{family}_forward"),
            card,
            "0 1",
            conditional,
        );
        let reverse = run_conditional_stepped_analysis(
            &format!("{family}_reverse"),
            card,
            "1 0",
            conditional,
        );

        for mode in [0, 1] {
            let forward = &forward[&mode];
            let reverse = &reverse[&mode];
            assert_eq!(
                forward.coordinate_id, reverse.coordinate_id,
                "{family}: semantic coordinate identity must not depend on enumeration order"
            );
            assert_eq!(
                forward.topology_fingerprint, reverse.topology_fingerprint,
                "{family}: the same materialized topology must have the same fingerprint"
            );
            assert_eq!(
                forward.validity, reverse.validity,
                "{family}: the union validity bitmap must not depend on enumeration order"
            );
            assert_eq!(
                forward.conditional_present,
                mode == 1,
                "{family}: the conditional signal must follow its coordinate topology"
            );
            assert_eq!(
                forward.artifact, reverse.artifact,
                "{family}: coordinate {mode} published different values in the two orders"
            );
        }

        // The two coordinates really are different circuits, so the test is
        // not vacuously comparing identical runs.
        assert_ne!(
            forward[&0].topology_fingerprint,
            forward[&1].topology_fingerprint
        );
    }
}
