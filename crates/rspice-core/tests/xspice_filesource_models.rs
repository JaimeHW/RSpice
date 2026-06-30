//! Official XSPICE `filesource` analog file-source model.

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_core::xspice::{
    CodeModelRegistry, PortConnection, XspiceInstance, clear_registered_data_files,
    register_data_file,
};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

static DATA_FILE_REGISTRY_LOCK: Mutex<()> = Mutex::new(());

struct DataFileRegistryGuard {
    _lock: MutexGuard<'static, ()>,
}

impl DataFileRegistryGuard {
    fn new() -> Self {
        let lock = DATA_FILE_REGISTRY_LOCK
            .lock()
            .expect("lock XSPICE data-file registry test guard");
        clear_registered_data_files().expect("clear XSPICE data-file registry");
        Self { _lock: lock }
    }
}

impl Drop for DataFileRegistryGuard {
    fn drop(&mut self) {
        let _ = clear_registered_data_files();
    }
}

struct TempNetlist {
    dir: PathBuf,
    netlist: Netlist,
}

impl Drop for TempNetlist {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

struct TempDir {
    dir: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Self {
        let dir = unique_temp_dir(prefix);
        fs::create_dir_all(&dir).expect("create temp XSPICE fixture dir");
        Self { dir }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

static NGSPICE_INPUT_DIR_LOCK: Mutex<()> = Mutex::new(());

struct NgspiceInputDirGuard {
    _lock: MutexGuard<'static, ()>,
    previous: Option<OsString>,
}

impl NgspiceInputDirGuard {
    fn set(path: &Path) -> Self {
        let lock = NGSPICE_INPUT_DIR_LOCK
            .lock()
            .expect("lock NGSPICE_INPUT_DIR test guard");
        let previous = std::env::var_os("NGSPICE_INPUT_DIR");
        unsafe {
            std::env::set_var("NGSPICE_INPUT_DIR", path);
        }
        Self {
            _lock: lock,
            previous,
        }
    }
}

impl Drop for NgspiceInputDirGuard {
    fn drop(&mut self) {
        unsafe {
            if let Some(previous) = &self.previous {
                std::env::set_var("NGSPICE_INPUT_DIR", previous);
            } else {
                std::env::remove_var("NGSPICE_INPUT_DIR");
            }
        }
    }
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

fn parse_temp_deck(prefix: &str, files: &[(&str, &str)], deck: &str) -> TempNetlist {
    let dir = unique_temp_dir(prefix);
    fs::create_dir_all(&dir).expect("create temp XSPICE filesource fixture dir");
    for (name, contents) in files {
        fs::write(dir.join(name), contents).expect("write filesource fixture");
    }
    let deck_path = dir.join("deck.cir");
    fs::write(&deck_path, deck).expect("write filesource deck");
    let netlist = Netlist::parse_file(Path::new(&deck_path))
        .unwrap_or_else(|err| panic!("deck parses: {err}"));
    TempNetlist { dir, netlist }
}

fn transient_node_series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[idx]
}

fn value_at_time(times: &[f64], values: &[f64], target: f64) -> f64 {
    assert_eq!(times.len(), values.len(), "waveform lengths must match");
    let first_time = *times.first().expect("waveform has samples");
    let first_value = *values.first().expect("waveform has samples");
    if target <= first_time {
        return first_value;
    }

    for (time_pair, value_pair) in times.windows(2).zip(values.windows(2)) {
        let (t0, t1) = (time_pair[0], time_pair[1]);
        if target <= t1 {
            let (v0, v1) = (value_pair[0], value_pair[1]);
            let span = t1 - t0;
            if span.abs() <= f64::EPSILON {
                return v1;
            }
            let alpha = (target - t0) / span;
            return v0 + alpha * (v1 - v0);
        }
    }

    *values.last().expect("waveform has samples")
}

fn sample_at_time<'a>(times: &'a [f64], values: &'a [f64], target: f64) -> f64 {
    assert_eq!(times.len(), values.len(), "waveform lengths must match");
    let index = times
        .iter()
        .position(|time| (*time - target).abs() <= 1.0e-18)
        .unwrap_or_else(|| panic!("missing transient sample at {target:e}; samples={times:?}"));
    values[index]
}

fn run_filesource_tran(temp: &TempNetlist, tstop: f64, max_step: f64) -> TransientResult {
    Engine::default()
        .run_tran(&temp.netlist, tstop, max_step)
        .expect("transient solves")
}

#[test]
fn filesource_falls_back_to_ngspice_input_dir_like_ngspice() {
    let data_dir = TempDir::new("rspice-xspice-filesource-env-data");
    let file_name = format!(
        "{}.txt",
        data_dir
            .dir
            .file_name()
            .expect("temp data dir has name")
            .to_string_lossy()
    );
    fs::write(
        data_dir.dir.join(&file_name),
        "\
0 0
1e-9 1
",
    )
    .expect("write filesource NGSPICE_INPUT_DIR fixture");
    let _env_guard = NgspiceInputDirGuard::set(&data_dir.dir);

    let netlist = Netlist::parse(&format!(
        "\
* XSPICE filesource NGSPICE_INPUT_DIR fallback
a_src out fs
.model fs filesource (file=\"{file_name}\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
"
    ))
    .expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.0e-9, 0.25e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 0.5).abs() < 1.0e-3,
        "filesource should find relative data files through NGSPICE_INPUT_DIR, got {mid}"
    );
}

#[test]
fn filesource_reads_registered_virtual_data_file() {
    let _guard = DataFileRegistryGuard::new();
    register_data_file(
        "virtual://filesource/stim",
        "\
0 1
1e-9 3
",
    )
    .expect("register virtual filesource data");

    let netlist = Netlist::parse(
        "\
* XSPICE filesource virtual data file
a_src out fs
.model fs filesource (file=\"virtual://filesource/stim\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    )
    .expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 1.0e-9, 0.25e-9)
        .expect("transient solves");
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 2.0).abs() < 1.0e-9,
        "filesource should interpolate registered virtual data, got {mid}"
    );
}

#[test]
fn filesource_voltage_vector_outputs_interpolate_file_rows() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-vector",
        &[(
            "stim.txt",
            "\
* time out0 out1
0 1 10
1e-9 3 14
2e-9 5 18
",
        )],
        "\
* XSPICE filesource vector interpolation
a_src out0 out1 fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out0 0 1k
r1 out1 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 2.0e-9, 0.25e-9);
    let out0 = transient_node_series(&result, "out0");
    let out1 = transient_node_series(&result, "out1");

    let mid0 = value_at_time(&result.time, out0, 0.5e-9);
    let mid1 = value_at_time(&result.time, out1, 0.5e-9);
    assert!(
        (mid0 - 2.0).abs() < 1.0e-3,
        "filesource out0 should interpolate to 2 V at 0.5 ns, got {mid0}"
    );
    assert!(
        (mid1 - 12.0).abs() < 1.0e-3,
        "filesource out1 should interpolate to 12 V at 0.5 ns, got {mid1}"
    );
}

#[test]
fn filesource_before_first_row_uses_second_row_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-before-first",
        &[(
            "stim.txt",
            "\
1e-9 1
2e-9 3
",
        )],
        "\
* XSPICE filesource pre-first-row oracle
a_src out fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 2.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");
    let initial = sample_at_time(&result.time, out, 0.0);
    assert!(
        (initial - 3.0).abs() < 1.0e-9,
        "ngspice filesource outputs the second row before the first timestamp, got {initial}"
    );
}

#[test]
fn filesource_ignores_extra_data_columns_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-extra-columns",
        &[(
            "stim.txt",
            "\
0 0 99
1e-9 1 88
",
        )],
        "\
* XSPICE filesource ignores extra data columns
a_src out fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 1.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 0.5).abs() < 1.0e-3,
        "filesource should ignore trailing columns and interpolate first value, got {mid}"
    );
}

#[test]
fn filesource_skips_rows_with_invalid_time_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-invalid-time",
        &[(
            "stim.txt",
            "\
junk
0 0
1e-9 1
",
        )],
        "\
* XSPICE filesource skips invalid time rows
a_src out fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 1.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 0.5).abs() < 1.0e-3,
        "filesource should skip nonnumeric time rows and interpolate later valid rows, got {mid}"
    );
}

#[test]
fn filesource_skips_leading_comma_time_rows_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-leading-comma-time",
        &[(
            "stim.txt",
            "\
,0 99
0 0
1e-9 1
",
        )],
        "\
* XSPICE filesource skips rows whose time field starts with a comma
a_src out fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 1.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 0.5).abs() < 1.0e-3,
        "filesource should skip leading-comma time rows like ngspice strtod, got {mid}"
    );
}

#[test]
fn filesource_uses_strtod_numeric_prefixes_without_spice_suffix_scaling() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-value-prefix",
        &[(
            "stim.txt",
            "\
0 0v
1e-9 1k
",
        )],
        "\
* XSPICE filesource uses strtod-style numeric prefixes for values
a_src out fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 1.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 0.5).abs() < 1.0e-3,
        "ngspice filesource uses strtod, so 1k must parse as numeric prefix 1 rather than spice-scaled 1000; got {mid}"
    );
}

#[test]
fn filesource_keeps_stream_alignment_after_invalid_data_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-invalid-data-stream",
        &[(
            "stim.txt",
            "\
0 0
1e-9 garbage
2e-9 2
",
        )],
        "\
* XSPICE filesource keeps flat data stream after invalid value
a_src out fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 2.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 1.0e-9).abs() < 1.0e-12,
        "filesource should treat the next valid numeric field as the invalid row's data like ngspice, got {mid}"
    );
}

#[test]
fn filesource_preserves_nonmonotonic_time_stream_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-nonmonotonic-time",
        &[(
            "stim.txt",
            "\
0 0
2e-9 2
1e-9 1
3e-9 3
",
        )],
        "\
* XSPICE filesource nonmonotonic time stream
a_src out fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 3.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");

    let before_backtrack = value_at_time(&result.time, out, 1.5e-9);
    let after_backtrack = value_at_time(&result.time, out, 2.5e-9);
    assert!(
        (before_backtrack - 1.5).abs() < 1.0e-3,
        "filesource should interpolate through the first 0ns->2ns stream interval, got {before_backtrack}"
    );
    assert!(
        (after_backtrack - 2.5).abs() < 1.0e-3,
        "filesource should continue through the later 1ns->3ns stream interval after the backtrack row, got {after_backtrack}"
    );
}

#[test]
fn filesource_schedules_nonmonotonic_row_times_as_breakpoints() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-nonmonotonic-breakpoints",
        &[(
            "stim.txt",
            "\
0 0
2e-9 2
1e-9 1
3e-9 3
",
        )],
        "\
* XSPICE filesource nonmonotonic row breakpoints
a_src out fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 3.0e-9, 5.0e-9);
    assert!(
        result
            .time
            .iter()
            .any(|time| (*time - 1.0e-9).abs() <= 1.0e-18),
        "filesource should request the earliest future row time even when rows are not monotonic; samples={:?}",
        result.time
    );
}

#[test]
fn file_source_alias_matches_filesource_behavior() {
    let temp = parse_temp_deck(
        "rspice-xspice-file-source-alias",
        &[(
            "stim.txt",
            "\
0 1
1e-9 3
",
        )],
        "\
* XSPICE file_source alias
a_src out fs
.model fs file_source (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );
    let result = run_filesource_tran(&temp, 1.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 2.0).abs() < 1.0e-9,
        "file_source alias should delegate to filesource interpolation, got {mid}"
    );
}

#[test]
fn filesource_explicit_current_vector_outputs_drive_each_element() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-current-vector",
        &[(
            "stim.txt",
            "\
0 1e-3 2e-3
1e-9 2e-3 4e-3
",
        )],
        "\
* XSPICE filesource explicit current vector outputs
a_src %id[out0 0] %id[out1 0] fs
.model fs filesource (file=\"stim.txt\" amploffset=[0] amplscale=[1])
r0 out0 0 1k
r1 out1 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 1.0e-9, 0.25e-9);
    let out0 = transient_node_series(&result, "out0");
    let out1 = transient_node_series(&result, "out1");

    let mid0 = value_at_time(&result.time, out0, 0.5e-9);
    let mid1 = value_at_time(&result.time, out1, 0.5e-9);
    assert!(
        (mid0 + 1.5).abs() < 1.0e-3,
        "filesource positive current output should drive -1.5 V through 1 kohm, got {mid0}"
    );
    assert!(
        (mid1 + 3.0).abs() < 1.0e-3,
        "filesource positive current output should drive -3 V through 1 kohm, got {mid1}"
    );
}

#[test]
fn filesource_step_time_and_amplitude_transforms_match_official_contract() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-step",
        &[(
            "stim.txt",
            "\
0 1 2
1 3 4
2 5 6
",
        )],
        "\
* XSPICE filesource transforms
a_src out0 out1 fs
.model fs filesource (file=\"stim.txt\" timescale=1e-9 timeoffset=1e-9 amplstep=true amplscale=[2] amploffset=[1 10])
r0 out0 0 1k
r1 out1 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 3.0e-9, 0.25e-9);
    let out0 = transient_node_series(&result, "out0");
    let out1 = transient_node_series(&result, "out1");

    let before_second_row0 = value_at_time(&result.time, out0, 1.5e-9);
    let before_second_row1 = value_at_time(&result.time, out1, 1.5e-9);
    assert!(
        (before_second_row0 - 3.0).abs() < 1.0e-3,
        "amplstep should hold transformed first row on out0, got {before_second_row0}"
    );
    assert!(
        (before_second_row1 - 12.0).abs() < 1.0e-3,
        "amplstep should hold transformed first row on out1, got {before_second_row1}"
    );

    let after_second_row0 = value_at_time(&result.time, out0, 2.25e-9);
    let after_second_row1 = value_at_time(&result.time, out1, 2.25e-9);
    assert!(
        (after_second_row0 - 7.0).abs() < 1.0e-3,
        "amplstep should advance to transformed second row on out0, got {after_second_row0}"
    );
    assert!(
        (after_second_row1 - 14.0).abs() < 1.0e-3,
        "amplstep should advance to transformed second row on out1, got {after_second_row1}"
    );
}

#[test]
fn filesource_omitted_amplitude_vectors_default_to_identity_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-identity-defaults",
        &[(
            "stim.txt",
            "\
0 1
1e-9 3
",
        )],
        "\
* XSPICE filesource omitted amplitude vectors
a_src out fs
.model fs filesource (file=\"stim.txt\")
.end
",
    );

    let result = run_filesource_tran(&temp, 1.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");

    let mid = value_at_time(&result.time, out, 0.5e-9);
    assert!(
        (mid - 2.0).abs() < 1.0e-9,
        "omitted amplscale/amploffset should behave as scale=1 and offset=0 like ngspice, got {mid}"
    );
}

#[test]
fn filesource_step_mode_holds_previous_row_at_exact_row_time_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-step-exact",
        &[(
            "stim.txt",
            "\
0 1
1e-9 3
2e-9 5
",
        )],
        "\
* XSPICE filesource exact step breakpoint value
a_src out fs
.model fs filesource (file=\"stim.txt\" amplstep=true amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 2.0e-9, 0.25e-9);
    let out = transient_node_series(&result, "out");
    let exact_second_row = sample_at_time(&result.time, out, 1.0e-9);
    assert!(
        (exact_second_row - 1.0).abs() < 1.0e-3,
        "amplstep should hold the previous row at the exact second-row time, got {exact_second_row}"
    );

    let exact_last_row = sample_at_time(&result.time, out, 2.0e-9);
    assert!(
        (exact_last_row - 3.0).abs() < 1.0e-3,
        "amplstep should hold the previous row at the exact final-row time, got {exact_last_row}"
    );
}

#[test]
fn filesource_rows_are_static_transient_breakpoints_even_with_large_steps() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-static-breakpoints",
        &[(
            "stim.txt",
            "\
0 0
1e-12 1
2e-12 2
",
        )],
        "\
* XSPICE filesource static row breakpoints
a_src out fs
.model fs filesource (file=\"stim.txt\" amplstep=true amploffset=[0] amplscale=[1])
r0 out 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 1.0e-9, 1.0e-9);
    let has_first_row = result
        .time
        .iter()
        .any(|time| (*time - 1.0e-12).abs() <= 1.0e-18);
    let has_second_row = result
        .time
        .iter()
        .any(|time| (*time - 2.0e-12).abs() <= 1.0e-18);

    assert!(
        has_first_row && has_second_row,
        "filesource row times should be scheduled before the first transient step; samples={:?}",
        result.time
    );
}

#[test]
fn filesource_rejects_vector_ports_and_params_below_official_minimum_at_construction() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry
        .get("filesource")
        .expect("filesource is registered");

    let err = XspiceInstance::new(
        "afile_empty_out",
        model.clone(),
        vec![PortConnection::AnalogVector(Vec::new())],
        &[],
        &[],
        &[],
        &[],
    )
    .expect_err("official filesource output vector lower bound must be enforced");
    assert!(
        err.to_string().contains("out") && err.to_string().contains("at least 1"),
        "filesource empty output vector should be rejected like ngspice, got {err}"
    );

    for param_name in ["amploffset", "amplscale"] {
        let real_vectors = if param_name == "amploffset" {
            vec![
                ("amploffset".to_string(), Vec::new()),
                ("amplscale".to_string(), vec![1.0]),
            ]
        } else {
            vec![
                ("amploffset".to_string(), vec![0.0]),
                ("amplscale".to_string(), Vec::new()),
            ]
        };
        let err = XspiceInstance::new(
            format!("afile_empty_{param_name}"),
            model.clone(),
            vec![PortConnection::AnalogVector(vec![1])],
            &[],
            &[],
            &real_vectors,
            &[],
        )
        .expect_err("official filesource amplitude vector lower bound must be enforced");
        assert!(
            err.to_string().contains(param_name) && err.to_string().contains("at least 1"),
            "filesource explicit empty {param_name} vector should be rejected like ngspice, got {err}"
        );
    }

    XspiceInstance::new(
        "afile_missing_amplitude_vectors",
        model,
        vec![PortConnection::AnalogVector(vec![1])],
        &[],
        &[],
        &[],
        &[],
    )
    .expect("ngspice accepts omitted filesource amplitude vectors");
}

#[test]
fn filesource_keeps_short_rows_in_flat_stream_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-filesource-width",
        &[(
            "bad.txt",
            "\
0 1
1e-9 2 3
",
        )],
        "\
* XSPICE filesource malformed row width
a_src out0 out1 fs
.model fs filesource (file=\"bad.txt\" amploffset=[0] amplscale=[1])
r0 out0 0 1k
r1 out1 0 1k
.end
",
    );

    let result = run_filesource_tran(&temp, 1.0e-9, 0.25e-9);
    let out0 = transient_node_series(&result, "out0");
    let out1 = transient_node_series(&result, "out1");
    let mid0 = value_at_time(&result.time, out0, 0.5e-9);
    let mid1 = value_at_time(&result.time, out1, 0.5e-9);

    assert!(
        (mid0 - 1.0).abs() < 1.0e-9,
        "short filesource rows should preserve the first valid stream value on out0, got {mid0}"
    );
    assert!(
        (mid1 - 1.0e-9).abs() < 1.0e-12,
        "short filesource rows should shift the next time value into out1 like ngspice, got {mid1}"
    );
}
