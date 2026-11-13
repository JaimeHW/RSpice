//! Official XSPICE file-backed table2d/table3d models.

use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;
use rspice_core::xspice::{
    CodeModelRegistry, PortConnection, XspiceInstance, clear_registered_data_files,
    register_data_file,
};
use rspice_core::{Complex64, Value};
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
    fs::create_dir_all(&dir).expect("create temp XSPICE table fixture dir");
    for (name, contents) in files {
        fs::write(dir.join(name), contents).expect("write table fixture");
    }
    let deck_path = dir.join("deck.cir");
    fs::write(&deck_path, deck).expect("write table deck");
    let netlist =
        Netlist::parse_file(&deck_path).unwrap_or_else(|err| panic!("deck parses: {err}"));
    TempNetlist { dir, netlist }
}

fn op_voltage(netlist: &Netlist, node: &str) -> Value {
    let op = Engine::default()
        .run_dc_op(netlist)
        .expect("operating point solves");
    let idx = op
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from OP result"));
    op.node_voltages[idx]
}

fn ac_voltage(netlist: &Netlist, node: &str) -> Complex64 {
    let point = Engine::default()
        .run_ac(netlist, &[1.0e3])
        .expect("AC solves")
        .into_iter()
        .next()
        .expect("one AC point");
    let idx = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", point.node_names));
    point.voltages[idx]
}

fn op_error_from_temp(prefix: &str, files: &[(&str, &str)], deck: &str) -> String {
    let dir = unique_temp_dir(prefix);
    fs::create_dir_all(&dir).expect("create temp XSPICE table fixture dir");
    for (name, contents) in files {
        fs::write(dir.join(name), contents).expect("write table fixture");
    }
    let deck_path = dir.join("deck.cir");
    fs::write(&deck_path, deck).expect("write table deck");
    let netlist = Netlist::parse_file(Path::new(&deck_path)).expect("deck parses");
    let message = Engine::default()
        .run_dc_op(&netlist)
        .expect_err("operating point must fail")
        .to_string();
    let _ = fs::remove_dir_all(dir);
    message
}

fn quadratic_table2d() -> String {
    let mut table = String::from(
        "\
* f(x,y)=x^2+3y^2+xy
4
4
0 1 2 3
0 1 2 3
",
    );

    for y in 0..4 {
        for x in 0..4 {
            if x > 0 {
                table.push(' ');
            }
            let value = (x * x) + 3 * (y * y) + (x * y);
            table.push_str(&value.to_string());
        }
        table.push('\n');
    }

    table
}

fn quadratic_table3d() -> String {
    let mut table = String::from(
        "\
* f(x,y,z)=x^2+3y^2+5z^2+xy+2xz+3yz
4
4
4
0 1 2 3
0 1 2 3
0 1 2 3
",
    );

    for z in 0..4 {
        for y in 0..4 {
            for x in 0..4 {
                if x > 0 {
                    table.push(' ');
                }
                let value =
                    (x * x) + 3 * (y * y) + 5 * (z * z) + (x * y) + 2 * (x * z) + 3 * (y * z);
                table.push_str(&value.to_string());
            }
            table.push('\n');
        }
    }

    table
}

fn quadratic_table2d_with_axes(xs: &[i32], ys: &[i32]) -> String {
    let mut table = format!(
        "\
* f(x,y)=x^2+3y^2+xy
{}
{}
{}
{}
",
        xs.len(),
        ys.len(),
        xs.iter().map(i32::to_string).collect::<Vec<_>>().join(" "),
        ys.iter().map(i32::to_string).collect::<Vec<_>>().join(" ")
    );

    for &y in ys {
        for (index, &x) in xs.iter().enumerate() {
            if index > 0 {
                table.push(' ');
            }
            let value = (x * x) + 3 * (y * y) + (x * y);
            table.push_str(&value.to_string());
        }
        table.push('\n');
    }

    table
}

#[test]
fn table2d_reads_registered_virtual_data_file() {
    let _guard = DataFileRegistryGuard::new();
    register_data_file(
        "virtual://table/simple",
        "\
2
2
0 1
0 1
0 1
2 3
",
    )
    .expect("register virtual table2d data");

    let netlist = Netlist::parse(
        "\
* XSPICE table2d virtual data file
vx x 0 dc 0.5
vy y 0 dc 0.25
atab x y out t2
.model t2 table2d (file=\"virtual://table/simple\" order=2)
rload out 0 1
.op
.end
",
    )
    .expect("deck parses");
    let out = op_voltage(&netlist, "out");

    assert!(
        (out + 1.0).abs() < 1.0e-9,
        "table2d should evaluate registered virtual data, got {out}"
    );
}

#[test]
fn table2d_interpolates_offsets_scales_and_linearizes_current_output() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d",
        &[(
            "table2d.tbl",
            "\
* f(x,y)=x+2y
2
2
0 1
0 1
0 1
2 3
",
        )],
        "\
* XSPICE table2d interpolation
vx x 0 dc 0.5 ac 1
vy y 0 dc 0.25 ac 0.5
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2 offset=1 gain=2)
rload out 0 1
.op
.ac lin 1 1k 1k
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 3.0).abs() < 1.0e-9,
        "table2d default current output should drive -3 V through 1 ohm, got {out}"
    );

    let ac = ac_voltage(&temp.netlist, "out");
    assert!(
        (ac.re + 4.0).abs() < 1.0e-9 && ac.im.abs() < 1.0e-12,
        "table2d AC should include dI/dx=2 and dI/dy=4 into 1 ohm, got {ac}"
    );
}

#[test]
fn table2d_treats_unparseable_table_values_as_zero_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-junk-value",
        &[(
            "table2d.tbl",
            "\
2
2
0 1
0 1
0 junk
2 3
",
        )],
        "\
* XSPICE table2d junk value oracle
vx x 0 dc 1
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        out.abs() < 1.0e-12,
        "ngspice reads an unparseable table value as 0, got {out}"
    );
}

#[test]
fn table2d_applies_suffix_after_exponent_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-exponent-suffix",
        &[(
            "table2d.tbl",
            "\
2
2
0 1
0 1
1e-3k 0
0 0
",
        )],
        "\
* XSPICE table2d exponent suffix oracle
vx x 0 dc 0
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 1.0).abs() < 1.0e-9,
        "ngspice applies the k suffix after exponent notation in table values, got {out}"
    );
}

#[test]
fn table3d_applies_suffix_after_exponent_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table3d-exponent-suffix",
        &[(
            "table3d.tbl",
            "\
2
2
2
0 1
0 1
0 1
1e-3k 0
0 0
0 0
0 0
",
        )],
        "\
* XSPICE table3d exponent suffix oracle
vx x 0 dc 0
vy y 0 dc 0
vz z 0 dc 0
atab x y z out t3
.model t3 table3d (file=\"table3d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 1.0).abs() < 1.0e-9,
        "ngspice applies the k suffix after exponent notation in table3d values, got {out}"
    );
}

#[test]
fn table2d_treats_indented_star_as_data_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-indented-star",
        &[(
            "table2d.tbl",
            "\
2
2
0 1
0 1
 * 10
20 30
",
        )],
        "\
* XSPICE table2d indented star oracle
vx x 0 dc 0.5
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 5.0).abs() < 1.0e-9,
        "ngspice treats indented '*' as table data, got {out}"
    );
}

#[test]
fn table3d_treats_indented_star_as_data_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table3d-indented-star",
        &[(
            "table3d.tbl",
            "\
2
2
2
0 1
0 1
0 1
 * 10
20 30
40 50
60 70
",
        )],
        "\
* XSPICE table3d indented star oracle
vx x 0 dc 0.5
vy y 0 dc 0
vz z 0 dc 0
atab x y z out t3
.model t3 table3d (file=\"table3d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 5.0).abs() < 1.0e-9,
        "ngspice treats indented '*' as table3d data, got {out}"
    );
}

#[test]
fn table2d_treats_unparseable_axis_values_as_zero_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-junk-axis",
        &[(
            "table2d.tbl",
            "\
2
2
junk 1
0 1
0 10
20 30
",
        )],
        "\
* XSPICE table2d junk axis oracle
vx x 0 dc 0.5
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 5.0).abs() < 1.0e-9,
        "ngspice reads an unparseable axis value as 0 and interpolates, got {out}"
    );
}

#[test]
fn table2d_accepts_equal_signs_as_token_separators_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-equal-separators",
        &[(
            "table2d.tbl",
            "\
2
2
0=1
0=1
0=10
20=30
",
        )],
        "\
* XSPICE table2d equal separator oracle
vx x 0 dc 0.5
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 5.0).abs() < 1.0e-9,
        "ngspice splits '=' while scanning table files and interpolates, got {out}"
    );
}

#[test]
fn table2d_truncates_fractional_dimensions_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-fractional-dimensions",
        &[(
            "table2d.tbl",
            "\
2.9
2.9
0 1
0 1
0 10
20 30
",
        )],
        "\
* XSPICE table2d fractional dimension oracle
vx x 0 dc 0.5
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 5.0).abs() < 1.0e-9,
        "ngspice casts fractional table dimensions to int before reading data, got {out}"
    );
}

#[test]
fn table2d_rejects_compact_dimension_header_like_ngspice() {
    let message = op_error_from_temp(
        "rspice-xspice-table2d-compact-header",
        &[(
            "bad.tbl",
            "\
2 2
0 1
0 1
0 10
20 30
",
        )],
        "\
* XSPICE table2d compact header oracle
vx x 0 dc 0.5
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"bad.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    assert!(
        message.contains("table2d") && message.contains("y dimension"),
        "ngspice reads dimensions from physical lines, got {message}"
    );
}

#[test]
fn table2d_rejects_extra_axis_row_tokens_like_ngspice() {
    let message = op_error_from_temp(
        "rspice-xspice-table2d-extra-axis-token",
        &[(
            "bad.tbl",
            "\
2
2
0 1 9
10 11
100 110
120
",
        )],
        "\
* XSPICE table2d extra axis token oracle
vx x 0 dc 0.5
vy y 0 dc 10
atab x y out t2
.model t2 table2d (file=\"bad.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    assert!(
        message.contains("table2d") && message.contains("x row"),
        "ngspice rejects extra tokens on an axis row, got {message}"
    );
}

#[test]
fn table2d_rejects_extra_data_row_tokens_like_ngspice() {
    let message = op_error_from_temp(
        "rspice-xspice-table2d-extra-data-token",
        &[(
            "bad.tbl",
            "\
2
2
0 1
0 1
0 10 99
20
",
        )],
        "\
* XSPICE table2d extra data token oracle
vx x 0 dc 0.5
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"bad.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    assert!(
        message.contains("table2d") && message.contains("y row"),
        "ngspice rejects extra tokens on a data row, got {message}"
    );
}

#[test]
fn table2d_rejects_whitespace_only_data_rows_like_ngspice() {
    let message = op_error_from_temp(
        "rspice-xspice-table2d-whitespace-data-row",
        &[("bad.tbl", "2\n2\n0 1\n0 1\n   \n0 10\n20 30\n")],
        "\
* XSPICE table2d whitespace data row oracle
vx x 0 dc 0.5
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"bad.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    assert!(
        message.contains("table2d") && message.contains("not enough numbers"),
        "ngspice rejects whitespace-only data rows, got {message}"
    );
}

#[test]
fn table2d_rejects_empty_data_rows_like_ngspice() {
    let message = op_error_from_temp(
        "rspice-xspice-table2d-empty-data-row",
        &[("bad.tbl", "2\n2\n0 1\n0 1\n\n0 10\n20 30\n")],
        "\
* XSPICE table2d empty data row oracle
vx x 0 dc 0.5
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"bad.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    assert!(
        message.contains("table2d") && message.contains("not enough numbers"),
        "ngspice rejects empty table2d data rows, got {message}"
    );
}

#[test]
fn table2d_zero_fills_missing_trailing_data_rows_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-missing-data-row",
        &[("table2d.tbl", "2\n2\n0 1\n0 1\n10 20\n")],
        "\
* XSPICE table2d missing trailing data row oracle
vx x 0 dc 0.5
vy y 0 dc 1
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        out.abs() < 1.0e-12,
        "ngspice zero-fills missing trailing table2d data rows, got {out}"
    );
}

#[test]
fn table3d_rejects_extra_data_row_tokens_like_ngspice() {
    let message = op_error_from_temp(
        "rspice-xspice-table3d-extra-data-token",
        &[(
            "bad.tbl",
            "\
2
2
2
0 1
0 1
0 1
0 1 99
2
4 5
6 7
",
        )],
        "\
* XSPICE table3d extra data token oracle
vx x 0 dc 0.5
vy y 0 dc 0
vz z 0 dc 0
atab x y z out t3
.model t3 table3d (file=\"bad.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    assert!(
        message.contains("table3d") && message.contains("y row"),
        "ngspice rejects extra tokens on a table3d data row, got {message}"
    );
}

#[test]
fn table3d_ignores_trailing_rows_after_expected_core_like_ngspice() {
    let temp = parse_temp_deck(
        "rspice-xspice-table3d-trailing-rows",
        &[(
            "table3d.tbl",
            "\
2
2
2
0 1
0 1
0 1
1 0
0 0
0 0
0 0
999 999
",
        )],
        "\
* XSPICE table3d trailing data row oracle
vx x 0 dc 0
vy y 0 dc 0
vz z 0 dc 0
atab x y z out t3
.model t3 table3d (file=\"table3d.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 1.0).abs() < 1.0e-12,
        "ngspice ignores rows after the table3d core and uses the first value, got {out}"
    );
}

#[test]
fn table2d_clamps_values_and_tapers_out_of_range_partials() {
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-clamp",
        &[(
            "table2d.tbl",
            "\
2
2
0 1
0 1
0 1
2 3
",
        )],
        "\
* XSPICE table2d boundary clamp
vx x 0 dc 2 ac 1
vy y 0 dc 0.25 ac 0
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=2)
rload out 0 1
.op
.ac lin 1 1k 1k
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 1.5).abs() < 1.0e-9,
        "table2d should clamp x above the upper edge before interpolation, got {out}"
    );

    let ac = ac_voltage(&temp.netlist, "out");
    assert!(
        ac.norm() < 1.0e-12,
        "table2d dI/dx should taper to zero beyond the boundary ramp, got {ac}"
    );
}

#[test]
fn table3d_interpolates_and_linearizes_all_three_inputs() {
    let temp = parse_temp_deck(
        "rspice-xspice-table3d",
        &[(
            "table3d.tbl",
            "\
* f(x,y,z)=x+2y+4z
2
2
2
0 1
0 1
0 1
0 1
2 3
4 5
6 7
",
        )],
        "\
* XSPICE table3d interpolation
vx x 0 dc 0.5 ac 1
vy y 0 dc 0.25 ac 0.5
vz z 0 dc 0.125 ac 0.25
atab x y z out t3
.model t3 table3d (file=\"table3d.tbl\" order=2)
rload out 0 1
.op
.ac lin 1 1k 1k
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 1.5).abs() < 1.0e-9,
        "table3d default current output should drive -1.5 V through 1 ohm, got {out}"
    );

    let ac = ac_voltage(&temp.netlist, "out");
    assert!(
        (ac.re + 3.0).abs() < 1.0e-9 && ac.im.abs() < 1.0e-12,
        "table3d AC should include dI/dx=1, dI/dy=2, dI/dz=4, got {ac}"
    );
}

#[test]
fn table3d_ac_uses_each_input_derivative() {
    let table = "\
2
2
2
0 1
0 1
0 1
0 1
2 3
4 5
6 7
";

    for (axis, y_ac, z_ac, expected) in [("y", 1.0, 0.0, -2.0), ("z", 0.0, 1.0, -4.0)] {
        let deck = format!(
            "\
* XSPICE table3d AC {axis}-input derivative
vx x 0 dc 0.5 ac 0
vy y 0 dc 0.25 ac {y_ac}
vz z 0 dc 0.125 ac {z_ac}
atab x y z out t3
.model t3 table3d (file=\"table3d.tbl\" order=2)
rload out 0 1
.ac lin 1 1k 1k
.end
"
        );
        let temp = parse_temp_deck(
            &format!("rspice-xspice-table3d-ac-{axis}-derivative"),
            &[("table3d.tbl", table)],
            &deck,
        );

        let ac = ac_voltage(&temp.netlist, "out");
        assert!(
            (ac.re - expected).abs() < 1.0e-9 && ac.im.abs() < 1.0e-12,
            "table3d AC should use the {axis} derivative and drive v(out)={expected}, got {ac}"
        );
    }
}

#[test]
fn table_models_clamp_order_below_two_like_ngspice() {
    let table2 = parse_temp_deck(
        "rspice-xspice-table2d-order-clamp",
        &[(
            "table2d.tbl",
            "\
2
2
0 1
0 1
0 1
2 3
",
        )],
        "\
* XSPICE table2d order lower clamp
vx x 0 dc 0.5
vy y 0 dc 0.25
atab x y out t2
.model t2 table2d (file=\"table2d.tbl\" order=1)
rload out 0 1
.op
.end
",
    );

    let table2_out = op_voltage(&table2.netlist, "out");
    assert!(
        (table2_out + 1.0).abs() < 1.0e-9,
        "table2d should clamp order=1 to order=2 like ngspice, got {table2_out}"
    );

    let table3 = parse_temp_deck(
        "rspice-xspice-table3d-order-clamp",
        &[(
            "table3d.tbl",
            "\
2
2
2
0 1
0 1
0 1
0 1
2 3
4 5
6 7
",
        )],
        "\
* XSPICE table3d order lower clamp
vx x 0 dc 0.5
vy y 0 dc 0.25
vz z 0 dc 0.125
atab x y z out t3
.model t3 table3d (file=\"table3d.tbl\" order=1)
rload out 0 1
.op
.end
",
    );

    let table3_out = op_voltage(&table3.netlist, "out");
    assert!(
        (table3_out + 1.5).abs() < 1.0e-9,
        "table3d should clamp order=1 to order=2 like ngspice, got {table3_out}"
    );
}

#[test]
fn table_models_accept_and_clamp_verbose_outside_official_range_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();

    for (model_name, ports) in [
        (
            "table2d",
            vec![
                PortConnection::Analog(1),
                PortConnection::Analog(2),
                PortConnection::CurrentOutput { pos: 3, neg: 0 },
            ],
        ),
        (
            "table3d",
            vec![
                PortConnection::Analog(1),
                PortConnection::Analog(2),
                PortConnection::Analog(3),
                PortConnection::CurrentOutput { pos: 4, neg: 0 },
            ],
        ),
    ] {
        let model = registry
            .get(model_name)
            .unwrap_or_else(|| panic!("{model_name} is registered"));

        for verbose in [-1.0, 3.0] {
            XspiceInstance::new(
                format!("a_{model_name}_verbose_{verbose:e}"),
                model.clone(),
                ports.clone(),
                &[("verbose".to_string(), verbose)],
                &[],
                &[],
                &[],
            )
            .unwrap_or_else(|err| {
                panic!("{model_name} verbose={verbose:e} should clamp like ngspice, got {err}")
            });
        }
    }
}

#[test]
fn table_models_reject_malformed_files() {
    let message = op_error_from_temp(
        "rspice-xspice-table2d-malformed",
        &[(
            "bad.tbl",
            "\
2
2
0 0
0 1
0 1
2 3
",
        )],
        "\
* XSPICE table2d malformed axis
vx x 0 dc 0
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"bad.tbl\" order=2)
rload out 0 1
.op
.end
",
    );

    assert!(
        message.contains("table2d")
            && message.contains("bad.tbl")
            && message.contains("strictly increasing"),
        "malformed table error should identify model, file, and axis issue, got {message}"
    );
}

#[test]
fn table2d_uses_official_eno_derivatives_for_partials() {
    let table = quadratic_table2d();
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-eno",
        &[("quad2.tbl", table.as_str())],
        "\
* XSPICE table2d ENO derivative parity
vx x 0 dc 1.3 ac 1
vy y 0 dc 1.7 ac 1
atab x y out t2
.model t2 table2d (file=\"quad2.tbl\" order=3)
rload out 0 1
.op
.ac lin 1 1k 1k
.end
",
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 13.41).abs() < 1.0e-9,
        "table2d OP value should remain bilinear while derivatives use ENO, got {out}"
    );

    let ac = ac_voltage(&temp.netlist, "out");
    assert!(
        (ac.re + 15.8).abs() < 1.0e-9 && ac.im.abs() < 1.0e-12,
        "table2d AC should use ngspice-compatible ENO dI/dx+dI/dy=15.8, got {ac}"
    );
}

#[test]
fn table3d_uses_official_eno_derivatives_for_partials() {
    let table = quadratic_table3d();
    let op_deck = "\
* XSPICE table3d ENO derivative parity
vx x 0 dc 1.3 ac 0
vy y 0 dc 1.7 ac 0
vz z 0 dc 1.2 ac 0
atab x y z out t3
.model t3 table3d (file=\"quad3.tbl\" order=3)
rload out 0 1
.op
.ac lin 1 1k 1k
.end
";
    let temp = parse_temp_deck(
        "rspice-xspice-table3d-eno-op",
        &[("quad3.tbl", table.as_str())],
        op_deck,
    );

    let out = op_voltage(&temp.netlist, "out");
    assert!(
        (out + 30.65).abs() < 1.0e-9,
        "table3d OP value should remain trilinear while derivatives use ENO, got {out}"
    );

    for (axis, x_ac, y_ac, z_ac, expected) in [
        ("x", 1.0, 0.0, 0.0, 6.7),
        ("y", 0.0, 1.0, 0.0, 15.1),
        ("z", 0.0, 0.0, 1.0, 19.7),
    ] {
        let deck = format!(
            "\
* XSPICE table3d ENO {axis}-partial
vx x 0 dc 1.3 ac {x_ac}
vy y 0 dc 1.7 ac {y_ac}
vz z 0 dc 1.2 ac {z_ac}
atab x y z out t3
.model t3 table3d (file=\"quad3.tbl\" order=3)
rload out 0 1
.op
.ac lin 1 1k 1k
.end
"
        );
        let temp = parse_temp_deck(
            &format!("rspice-xspice-table3d-eno-{axis}"),
            &[("quad3.tbl", table.as_str())],
            &deck,
        );
        let ac = ac_voltage(&temp.netlist, "out");
        assert!(
            (ac.re + expected).abs() < 1.0e-9 && ac.im.abs() < 1.0e-12,
            "table3d AC should use ENO response {expected} for {axis}-only excitation, got {ac}"
        );
    }
}

#[test]
fn table2d_non_unit_axes_match_ngspice_eno_derivative_scaling() {
    let table = quadratic_table2d_with_axes(&[0, 2, 4, 6], &[0, 2, 4, 6]);
    let temp = parse_temp_deck(
        "rspice-xspice-table2d-nonunit-eno",
        &[("quad2.tbl", table.as_str())],
        "\
* XSPICE table2d non-unit-axis ENO derivative parity
vx x 0 dc 2.6 ac 1
vy y 0 dc 3.4 ac 1
atab x y out t2
.model t2 table2d (file=\"quad2.tbl\" order=3)
rload out 0 1
.op
.ac lin 1 1k 1k
.end
",
    );

    let ac = ac_voltage(&temp.netlist, "out");
    assert!(
        (ac.re + 43.2).abs() < 1.0e-9 && ac.im.abs() < 1.0e-12,
        "table2d non-unit axes should match ngspice ENO scaling, got {ac}"
    );
}

#[test]
fn table2d_rejects_eno_order_that_exceeds_grid() {
    let message = op_error_from_temp(
        "rspice-xspice-table2d-order",
        &[(
            "small.tbl",
            "\
2
2
0 1
0 1
0 1
2 3
",
        )],
        "\
* XSPICE table2d invalid ENO order
vx x 0 dc 0
vy y 0 dc 0
atab x y out t2
.model t2 table2d (file=\"small.tbl\" order=3)
rload out 0 1
.op
.end
",
    );

    assert!(
        message.contains("table2d") && message.contains("order") && message.contains("grid"),
        "too-large ENO order error should identify model, order, and grid issue, got {message}"
    );
}
