//! File I/O, string I/O, `$readmem*` and the value change dump end to end.
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`, RSpice-authored probes), with binary modes
//! so that its Windows build's text-mode newline translation does not move
//! `$ftell`.

use super::*;

/// A fresh run directory of this test's own.
fn run_directory(name: &str) -> std::path::PathBuf {
    let directory =
        std::env::temp_dir().join(format!("rspice-hdl-bench-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(directory.join("work")).unwrap();
    directory
}

fn bench_in(directory: &std::path::Path, source: &str, write_files: bool) -> HdlTestbenchReport {
    testbench(
        &[("tb.v", source)],
        &HdlTestbenchOptions {
            run_directory: Some(directory.to_path_buf()),
            write_files,
            ..HdlTestbenchOptions::default()
        },
    )
}

/// The text the design wrote to `name`, from its `File` records.
fn written(report: &HdlTestbenchReport, name: &str) -> String {
    report
        .records
        .iter()
        .filter(|record| matches!(&record.stream, HdlStream::File { path, .. } if path == name))
        .map(|record| record.text.as_str())
        .collect()
}

fn warnings(report: &HdlTestbenchReport) -> Vec<String> {
    report
        .records
        .iter()
        .filter(|record| {
            record.stream
                == HdlStream::Diagnostic {
                    severity: HdlSeverity::Warning,
                }
        })
        .map(|record| record.text.clone())
        .collect()
}

const FILES: &str = r#"module tb;
  integer fd, mcd, r, c, i;
  reg [8*16:1] line;
  reg [31:0] a, b;
  reg [8*4:1] word;
  initial begin
    fd = $fopen("work/p4.txt", "wb");
    mcd = $fopen("work/p4m.txt");
    $fdisplay(fd, "alpha %0d", 7);
    $fwrite(fd, "%h %b\n", 8'hc3, 3'b1x0);
    $fwrite(fd, "12 34 end");
    $fdisplay(mcd | 1, "both %0d", 3);
    $fclose(fd);
    $fclose(mcd);
    fd = $fopen("work/p4.txt", "rb");
    r = $fgets(line, fd);
    $display("fgets %0d [%0s]", r, line);
    c = $fgetc(fd);
    r = $ungetc(c, fd);
    $display("fgetc %0d ungetc %0d tell %0d", c, r, $ftell(fd));
    r = $fscanf(fd, "%h %b", a, b);
    $display("fscanf %0d %h %b", r, a, b[2:0]);
    r = $fscanf(fd, "%d %d %s", a, b, word);
    $display("fscanf %0d %0d %0d %0s eof %0d", r, a, b, word, $feof(fd));
    r = $fscanf(fd, "%d", a);
    $display("at end %0d eof %0d", r, $feof(fd));
    r = $rewind(fd);
    r = $fseek(fd, 6, 0);
    c = $fgetc(fd);
    $display("seek %0d char %0d tell %0d", r, c, $ftell(fd));
    $fclose(fd);
    fd = $fopen("work/p4m.txt", "rb");
    r = $fgets(line, fd);
    $display("mcd [%0s]", line);
    $fclose(fd);
    fd = $fopen("work/p4.txt", "ab");
    $fwrite(fd, "+tail");
    $fclose(fd);
    fd = $fopen("work/p4.txt", "rb");
    for (i = 0; i < 3; i = i + 1) r = $fgets(line, fd);
    r = $fgets(line, fd);
    $display("appended %0d [%0s]", r, line);
    $fclose(fd);
    $display("missing %h", $fopen("work/none/p4.txt", "w"));
    $display("closed %0d", $fgetc(fd));
  end
endmodule
"#;

/// Descriptors, multichannel writes, read-back of what the run wrote before
/// anything reached a disk, `$fgets`, `$fgetc`/`$ungetc`, `$fscanf` to the
/// end of the file, `$fseek`/`$rewind`/`$ftell`, append mode, and the
/// failures that return 0 or EOF.
#[test]
fn a_testbench_writes_and_reads_files_as_icarus_does() {
    let directory = run_directory("files");
    let report = bench_in(&directory, FILES, false);
    assert_eq!(
        stdout(&report),
        "both 3\n\
         fgets 8 [alpha 7\n]\n\
         fgetc 99 ungetc 0 tell 8\n\
         fscanf 2 000000c3 1x0\n\
         fscanf 3 12 34 end eof 1\n\
         at end -1 eof 1\n\
         seek 0 char 55 tell 7\n\
         mcd [both 3\n]\n\
         appended 0 [12 34 end+tail]\n\
         missing 00000000\n\
         closed -1\n"
    );
    assert_eq!(
        written(&report, "work/p4.txt"),
        "alpha 7\nc3 1x0\n12 34 end+tail"
    );
    assert_eq!(written(&report, "work/p4m.txt"), "both 3\n");
    assert_eq!(
        warnings(&report),
        ["$fgetc: invalid file descriptor (0x80000003)"]
    );
    // The files stayed in memory.
    assert!(!directory.join("work").join("p4.txt").exists());

    let report = bench_in(&directory, FILES, true);
    assert_eq!(
        std::fs::read_to_string(directory.join("work").join("p4.txt")).unwrap(),
        "alpha 7\nc3 1x0\n12 34 end+tail"
    );
    assert!(stdout(&report).contains("appended 0 [12 34 end+tail]\n"));
}

/// `$sformat` with a literal and a variable format, `$swriteh`, `$sscanf`
/// of a literal and of a variable, `$readmemh` past the memory's end and
/// `$readmemb` downward into a narrower memory with comments, `x`, `z` and
/// `_`.
#[test]
fn string_io_and_readmem_run_as_icarus_runs_them() {
    let directory = run_directory("strings");
    std::fs::write(
        directory.join("work").join("p3a.txt"),
        "1 2 3\n4 // c\n/* x */ 5\n",
    )
    .unwrap();
    std::fs::write(
        directory.join("work").join("p5b.txt"),
        "// nibbles\n1010 0_1x1\nzz11\n",
    )
    .unwrap();
    let report = bench_in(
        &directory,
        r#"module tb;
  reg [8*12:1] s, fmt;
  reg [7:0] mem [0:3];
  reg [3:0] nib [7:4];
  integer r, a, b;
  real x;
  initial begin
    $sformat(s, "v=%0d|%s", 42, "ok");
    $display("[%0s]", s);
    fmt = "%0h-%0h";
    $sformat(s, fmt, 10, 255);
    $display("[%0s]", s);
    $swriteh(s, 8'hab, "|", 4'd3);
    $display("[%0s]", s);
    r = $sscanf("7 2.5 zz", "%d %f %h", a, x, b);
    $display("%0d %0d %f %h", r, a, x, b);
    r = $sscanf(s, "%h", a);
    $display("%0d %h", r, a);
    $readmemh("work/p3a.txt", mem);
    $display("%h %h %h %h", mem[0], mem[1], mem[2], mem[3]);
    $readmemb("work/p5b.txt", nib, 6, 4);
    $display("%b %b %b %b", nib[4], nib[5], nib[6], nib[7]);
  end
endmodule
"#,
        false,
    );
    assert_eq!(
        stdout(&report),
        "[v=42|ok]\n\
         [a-ff]\n\
         [ab|3]\n\
         3 7 2.500000 zzzzzzzz\n\
         1 000000ab\n\
         01 02 03 04\n\
         zz11 01x1 1010 xxxx\n"
    );
    assert_eq!(
        warnings(&report),
        ["$readmemh: work/p3a.txt: too many words in the file for the requested range [0:3]"]
    );
}

/// Every condition ivtest's `readmem-error` diagnoses is reported, in call
/// order, and loads what Icarus loads: an unknown register or integer as the
/// file name, a non-printable character in it, a real start or finish
/// address (rounded, and the load goes ahead), a start or finish address
/// outside an ascending or offset memory (nothing loads), too many and too
/// few words for the range, and an invalid digit (the load stops there).
/// Icarus 13.0 printed the same contents and one `WARNING:` or `ERROR:` per
/// call (probe `p8.v`).
#[test]
fn readmem_reports_every_condition_icarus_reports() {
    let directory = run_directory("readmem-errors");
    for (name, text) in [
        ("p8a.txt", "1 2 3 4\n"),
        ("p8b.txt", "5 6\n"),
        ("p8c.txt", "7 8 u 9\n"),
    ] {
        std::fs::write(directory.join("work").join(name), text).unwrap();
    }
    let report = bench_in(
        &directory,
        r#"module tb;
  reg [8*12:1] str;
  integer istr, i;
  real rval;
  reg [7:0] mem [0:3];
  reg [7:0] hi [8:11];
  task clear;
    for (i = 0; i < 4; i = i + 1) begin
      mem[i] = 0;
      hi[i + 8] = 0;
    end
  endtask
  task show;
    $display("%h %h %h %h | %h %h %h %h", mem[0], mem[1], mem[2], mem[3],
             hi[8], hi[9], hi[10], hi[11]);
  endtask
  initial begin
    clear;
    $readmemh(str, mem);
    $readmemh(istr, mem);
    str = "work/p8a.txt";
    str[8:1] = 8'd2;
    $readmemh(str, mem);
    show;
    rval = 0.0;
    $readmemh("work/p8a.txt", mem, rval);
    show;
    clear;
    rval = 3.0;
    $readmemh("work/p8a.txt", mem, 0, rval);
    show;
    clear;
    $readmemh("work/p8a.txt", mem, -1, 3);
    $readmemh("work/p8a.txt", hi, 7, 11);
    $readmemh("work/p8a.txt", mem, 0, 4);
    $readmemh("work/p8a.txt", hi, 8, 12);
    show;
    $readmemh("work/p8a.txt", mem, 0, 1);
    show;
    clear;
    $readmemh("work/p8b.txt", mem);
    show;
    clear;
    $readmemh("work/p8c.txt", mem);
    show;
  end
endmodule
"#,
        false,
    );
    assert_eq!(
        stdout(&report),
        "00 00 00 00 | 00 00 00 00\n\
         01 02 03 04 | 00 00 00 00\n\
         01 02 03 04 | 00 00 00 00\n\
         00 00 00 00 | 00 00 00 00\n\
         01 02 00 00 | 00 00 00 00\n\
         05 06 00 00 | 00 00 00 00\n\
         07 08 00 00 | 00 00 00 00\n"
    );
    assert_eq!(
        warnings(&report),
        [
            "$readmemh: the file name argument is not a valid string",
            "$readmemh: the file name argument is not a valid string",
            "$readmemh: the file name argument contains non-printable characters: \
             \"work/p8a.tx\\002\"",
            "$readmemh: the start address is a real value; it is rounded to an integer",
            "$readmemh: the finish address is a real value; it is rounded to an integer",
            "$readmemh: work/p8a.txt: Start address -1 is out of bounds for the memory [0:3]",
            "$readmemh: work/p8a.txt: Start address 7 is out of bounds for the memory [8:11]",
            "$readmemh: work/p8a.txt: Finish address 4 is out of bounds for the memory [0:3]",
            "$readmemh: work/p8a.txt: Finish address 12 is out of bounds for the memory [8:11]",
            "$readmemh: work/p8a.txt: too many words in the file for the requested range [0:1]",
            "$readmemh: work/p8b.txt: not enough words in the file for the requested range [0:3]",
            "$readmemh: work/p8c.txt: invalid input character: u",
        ]
    );
}

/// Section 17.2.2: any number of `$fmonitor`s print to their files at the end
/// of each step an operand changes in, and closing any file one of them
/// writes ends it (Icarus probe `fmonitor2.v`'s behaviour).
#[test]
fn fmonitors_print_until_their_file_closes() {
    let directory = run_directory("fmonitor");
    let report = bench_in(
        &directory,
        r#"module tb;
  integer a, b, m1, m2, m3;
  initial begin
    a = 0; b = 0;
    m1 = $fopen("work/m1.log"); m2 = $fopen("work/m2.log"); m3 = $fopen("work/m3.log");
    $fmonitor(m1 | m2, "@%0t a = %0d", $time, a);
    $fmonitor(m3, "@%0t b = %0d", $time, b);
    $fstrobe(m3, "strobe %0d", b);
    repeat (3) begin #1; a = a + 1; b = b + 1; end
    $fclose(m2);
    repeat (2) begin #1; a = a + 1; b = b + 1; end
  end
endmodule
"#,
        false,
    );
    assert_eq!(
        written(&report, "work/m1.log"),
        "@0 a = 0\n@1 a = 1\n@2 a = 2\n"
    );
    assert_eq!(
        written(&report, "work/m2.log"),
        "@0 a = 0\n@1 a = 1\n@2 a = 2\n"
    );
    assert_eq!(
        written(&report, "work/m3.log"),
        "@0 b = 0\nstrobe 0\n@1 b = 1\n@2 b = 2\n@3 b = 3\n@4 b = 4\n@5 b = 5\n"
    );
}

/// Section 18.1: the dump reads back through RSpice's VCD reader with the
/// selected variables, their values at each step that changed them, `x`
/// while dumping is off, and the current values at `$dumpon`.
#[test]
fn a_dump_reads_back_as_vcd() {
    use crate::io::vcd::{VcdBit, VcdValue};
    let directory = run_directory("dump");
    let report = bench_in(
        &directory,
        r#"`timescale 1ns/1ns
module tb;
  reg clk;
  reg [3:0] count;
  real level;
  reg [7:0] mem [0:1];
  initial begin
    $dumpfile("work/waves.vcd");
    $dumpvars(0, tb);
    clk = 0; count = 0; level = 0.5;
    #5 clk = 1; count = 1;
    #5 clk = 0; level = 1.5;
    #5 $dumpoff;
    #5 count = 3;
    #5 $dumpon;
    #5 $finish;
  end
endmodule
"#,
        false,
    );
    let text = written(&report, "work/waves.vcd");
    let document = crate::io::vcd::parse_vcd_bytes_with_limits(
        text.as_bytes(),
        crate::resource::ResourceLimits::default(),
    )
    .unwrap_or_else(|error| panic!("{error}:\n{text}"));
    assert_eq!(document.timescale.seconds(), 1e-9);
    let signal = |name: &str| {
        document
            .signals
            .iter()
            .find(|signal| signal.variables[0].name.starts_with(name))
            .unwrap_or_else(|| panic!("no `{name}` in\n{text}"))
    };
    assert_eq!(signal("clk").variables[0].scope, ["tb"]);
    assert!(
        document
            .signals
            .iter()
            .all(|signal| !signal.variables[0].name.starts_with("mem")),
        "memories are not dumped: {text}"
    );
    let ticks = |name: &str| -> Vec<u64> { signal(name).changes.iter().map(|c| c.tick).collect() };
    assert_eq!(ticks("clk"), [0, 5, 10, 15, 25]);
    assert_eq!(ticks("count"), [0, 5, 15, 25]);
    let count: Vec<_> = signal("count")
        .changes
        .iter()
        .map(|change| change.value.clone())
        .collect();
    let bits = |spelling: &str| {
        VcdValue::Logic(
            spelling
                .chars()
                .map(|bit| match bit {
                    '0' => VcdBit::Zero,
                    '1' => VcdBit::One,
                    _ => VcdBit::Unknown,
                })
                .collect(),
        )
    };
    assert_eq!(
        count,
        [bits("0000"), bits("0001"), bits("xxxx"), bits("0011")]
    );
    let level: Vec<_> = signal("level")
        .changes
        .iter()
        .map(|change| (change.tick, change.value.clone()))
        .collect();
    assert_eq!(
        level,
        [
            (0, VcdValue::Real(0.5)),
            (10, VcdValue::Real(1.5)),
            (25, VcdValue::Real(1.5))
        ]
    );
    assert!(
        text.contains("$dumpoff\n") && text.contains("$dumpon\n"),
        "{text}"
    );
}

/// The header from its first `$scope` through `$enddefinitions`.
fn dump_header(text: &str) -> &str {
    let start = text
        .find("$scope")
        .unwrap_or_else(|| panic!("no scope in\n{text}"));
    let end = text
        .find("$enddefinitions $end\n")
        .map_or(text.len(), |end| end + 21);
    &text[start..end]
}

/// Section 18.1.2: `$dumpvars` of a hierarchical name dumps that scope, an
/// instance, a generate block, a named block or a task, under the scopes that
/// enclose it, `levels` deep. A port joined to the net it connects is listed
/// in its own scope with that net's code. The header is Icarus Verilog's for
/// the same calls.
#[test]
fn a_dump_of_hierarchical_scopes_lays_them_out_as_icarus_does() {
    const SOURCE: &str = r#"`timescale 1ns/1ns
module leaf(input d, output q);
  reg r;
  assign q = ~d;
  always @(d) r = d;
endmodule
module mid(input d, output q);
  wire w;
  leaf u1(.d(d), .q(w));
  assign q = w;
  genvar i;
  generate for (i = 0; i < 2; i = i + 1) begin : g
    reg [1:0] gv;
    initial gv = 2'd1;
  end endgenerate
  generate if (1) begin : gi
    reg [2:0] giv;
    initial giv = 3'd5;
  end endgenerate
endmodule
module tb;
  reg d;
  wire q;
  integer k;
  mid dut(.d(d), .q(q));
  task t;
    reg [3:0] tv;
    tv = 4'd3;
  endtask
  initial begin : blk
    reg [2:0] bv;
    bv = 1;
    $dumpfile("work/scopes.vcd");
    $dumpvars(1, tb.dut);
    $dumpvars(0, tb.dut.u1);
    $dumpvars(0, tb.dut.gi);
    $dumpvars(0, tb.blk);
    $dumpvars(0, tb.t);
    d = 0; #1 d = 1; #1 t; #1 $finish;
  end
endmodule
"#;
    const ICARUS: &str = r#"$scope module tb $end
$scope module dut $end
$var wire 1 ! d $end
$var wire 1 " q $end
$var wire 1 # w $end
$upscope $end
$upscope $end
$scope module tb $end
$scope module dut $end
$scope module u1 $end
$var wire 1 ! d $end
$var wire 1 # q $end
$var reg 1 $ r $end
$upscope $end
$upscope $end
$upscope $end
$scope module tb $end
$scope module dut $end
$scope begin gi $end
$var reg 3 % giv [2:0] $end
$upscope $end
$upscope $end
$upscope $end
$scope module tb $end
$scope begin blk $end
$var reg 3 & bv [2:0] $end
$upscope $end
$upscope $end
$scope module tb $end
$scope task t $end
$var reg 4 ' tv [3:0] $end
$upscope $end
$upscope $end
$enddefinitions $end
"#;
    let directory = run_directory("dump-scopes");
    let report = bench_in(&directory, SOURCE, false);
    assert_eq!(dump_header(&written(&report, "work/scopes.vcd")), ICARUS);
}

const DUMPED_DESIGN: &str = r#"`timescale 1ns/1ns
module leaf(input d, output q);
  reg r;
  assign q = ~d;
  always @(d) r = d;
endmodule
module mid(input d, output q);
  wire w;
  leaf u1(.d(d), .q(w));
  assign q = w;
  generate if (1) begin : gi
    reg [2:0] giv;
    wire gq;
    leaf u2(.d(w), .q(gq));
    initial giv = 3'd5;
  end endgenerate
endmodule
module tb;
  reg d;
  wire q;
  integer k;
  real x;
  mid dut(.d(d), .q(q));
  task t;
    reg [3:0] tv;
    tv = 4'd3;
  endtask
  function [1:0] f;
    input [1:0] a;
    f = a + 1;
  endfunction
  initial begin : blk
    reg [2:0] bv;
    bv = 1;
    k = f(2'd1);
    x = 0.5;
    $dumpfile("work/DUMP.vcd");
    CALLS
    d = 0; #1 d = 1; bv = 2; #1 t; #1 $finish;
  end
endmodule
"#;

/// A whole-design dump lists each scope's nets, variables and integers and
/// reals, then its functions, generate blocks, instances, named blocks and
/// tasks, as Icarus Verilog does: every port, an instance inside a generate
/// block, a named block's variable, and a function without its result.
/// Icarus differs twice: it gives a real width 1, and lists `u2`'s ports in
/// the order its runtime links them rather than by name.
#[test]
fn a_whole_design_dump_lists_every_scope_as_icarus_does() {
    const EXPECTED: &str = r#"$scope module tb $end
$var wire 1 ! q $end
$var reg 1 " d $end
$var integer 32 # k [31:0] $end
$var real 64 $ x $end
$scope function f $end
$var reg 2 % a [1:0] $end
$upscope $end
$scope module dut $end
$var wire 1 " d $end
$var wire 1 ! q $end
$var wire 1 & w $end
$scope begin gi $end
$var wire 1 ' gq $end
$var reg 3 ( giv [2:0] $end
$scope module u2 $end
$var wire 1 & d $end
$var wire 1 ' q $end
$var reg 1 ) r $end
$upscope $end
$upscope $end
$scope module u1 $end
$var wire 1 " d $end
$var wire 1 & q $end
$var reg 1 * r $end
$upscope $end
$upscope $end
$scope begin blk $end
$var reg 3 + bv [2:0] $end
$upscope $end
$scope task t $end
$var reg 4 , tv [3:0] $end
$upscope $end
$upscope $end
$enddefinitions $end
"#;
    let directory = run_directory("dump-whole");
    let source = DUMPED_DESIGN
        .replace("DUMP", "whole")
        .replace("CALLS", "$dumpvars(0, tb);");
    let report = bench_in(&directory, &source, false);
    assert_eq!(dump_header(&written(&report, "work/whole.vcd")), EXPECTED);
}

/// A variable an earlier argument named is not listed again when its scope is
/// dumped, and one whose scope or self an earlier argument dumped is skipped
/// with a warning, as Icarus Verilog does it.
#[test]
fn a_dump_lists_a_named_variable_once() {
    const ICARUS: &str = r#"$scope module tb $end
$scope module dut $end
$var wire 1 ! d $end
$upscope $end
$upscope $end
$scope module tb $end
$scope module dut $end
$scope module u1 $end
$var reg 1 " r $end
$upscope $end
$upscope $end
$upscope $end
$scope module tb $end
$scope module dut $end
$scope module u1 $end
$var wire 1 ! d $end
$var wire 1 # q $end
$upscope $end
$upscope $end
$upscope $end
$enddefinitions $end
"#;
    let directory = run_directory("dump-once");
    let source = DUMPED_DESIGN.replace("DUMP", "once").replace(
        "CALLS",
        "$dumpvars(1, tb.dut.d, tb.dut.u1.r); $dumpvars(0, tb.dut.u1); $dumpvars(0, tb.dut.d);",
    );
    let report = bench_in(&directory, &source, false);
    assert_eq!(dump_header(&written(&report, "work/once.vcd")), ICARUS);
    let skipped = warnings(&report);
    assert_eq!(skipped.len(), 1, "{skipped:?}");
    assert!(skipped[0].contains("`tb.dut.d`"), "{skipped:?}");
}

/// A write that would leave the run directory fails as
/// section 17.2.1 prescribes, with a located warning; the run may lift the
/// confinement.
#[test]
fn writes_outside_the_run_directory_fail_unless_unrestricted() {
    let directory = run_directory("sandbox");
    let source = r#"module tb;
  integer fd;
  initial begin
    fd = $fopen("../escaped.txt", "w");
    $display("fd=%0d", fd != 0);
    $dumpfile("../escaped.vcd");
    $dumpvars;
  end
endmodule
"#;
    let report = bench_in(&directory, source, false);
    assert_eq!(stdout(&report), "fd=0\n");
    let escapes = warnings(&report);
    assert_eq!(escapes.len(), 2, "{escapes:?}");
    assert!(
        escapes
            .iter()
            .all(|warning| warning.contains("outside the run directory")),
        "{escapes:?}"
    );
    let warning = report
        .records
        .iter()
        .find(|record| matches!(record.stream, HdlStream::Diagnostic { .. }))
        .unwrap();
    assert_eq!(
        (warning.origin.file.as_str(), warning.origin.line),
        ("tb.v", 4)
    );

    let report = testbench(
        &[("tb.v", source)],
        &HdlTestbenchOptions {
            run_directory: Some(directory.clone()),
            unrestricted_io: true,
            ..HdlTestbenchOptions::default()
        },
    );
    assert_eq!(stdout(&report), "fd=1\n");
    assert!(warnings(&report).is_empty());
}

/// `$fread` into a memory (IEEE 1364-2005 section 17.2.4.4): whole bytes,
/// most significant first, from a start address for a count of words; a
/// count past the memory's end is cut to it, and a start outside it reads
/// nothing. Expected text from Icarus 13.0 running the same source.
#[test]
fn fread_loads_a_memory_as_icarus_does() {
    let directory = run_directory("fread-memory");
    let source = r#"module tb;
  integer fd, r;
  reg [7:0] mem [0:3];
  initial begin
    fd = $fopen("work/p7.bin", "wb");
    $fwrite(fd, "%c%c%c%c%c", 8'h12, 8'h34, 8'hab, 8'hcd, 8'hef);
    $fclose(fd);
    fd = $fopen("work/p7.bin", "rb");
    mem[0] = 8'h00; mem[1] = 8'h00; mem[2] = 8'h00; mem[3] = 8'h77;
    r = $fread(mem, fd, 1, 2);
    $display("mem %0d %h %h %h %h", r, mem[0], mem[1], mem[2], mem[3]);
    r = $fread(mem, fd, 3, 5);
    $display("count %0d %h", r, mem[3]);
    r = $fread(mem, fd, 4);
    $display("outside %0d", r);
    $fclose(fd);
  end
endmodule
"#;
    let report = bench_in(&directory, source, false);
    assert_eq!(
        stdout(&report),
        "mem 2 00 12 34 77\n\
         count 1 ab\n\
         outside 0\n"
    );
    let warned = warnings(&report);
    assert_eq!(warned.len(), 2, "{warned:?}");
    assert!(warned[0].contains("the count 5 runs past"), "{warned:?}");
    assert!(
        warned[1].contains("the start address 4 is outside"),
        "{warned:?}"
    );
}

/// A write to the standard-error descriptor is a `Stderr` record, never
/// standard output; the standard-output descriptor's is `Stdout`.
#[test]
fn writes_to_the_stderr_descriptor_are_stderr_records() {
    let report = testbench(
        &[(
            "tb.v",
            "module tb;\n  initial begin\n    $fdisplay(32'h8000_0001, \"out\");\n    \
             $fwrite(32'h8000_0002, \"err %0d\\n\", 2);\n    $fstrobe(32'h8000_0002, \"late\");\n  \
             end\nendmodule\n",
        )],
        &HdlTestbenchOptions::default(),
    );
    assert_eq!(stdout(&report), "out\n");
    let errors: Vec<&str> = report
        .records
        .iter()
        .filter(|record| record.stream == HdlStream::Stderr)
        .map(|record| record.text.as_str())
        .collect();
    assert_eq!(errors, ["err 2\n", "late\n"]);
}

/// `$fread` into a reg (IEEE 1364-2005 section 17.2.4.4): whole bytes, most
/// significant first, a narrow reg keeping the low bits, and a short read
/// keeping the bytes it did not reach. Expected text from Icarus 13.0
/// running the same source.
#[test]
fn fread_loads_a_reg_as_icarus_does() {
    let directory = run_directory("fread-reg");
    let source = r#"module tb;
  integer fd, r;
  reg [15:0] h;
  reg [11:0] t;
  initial begin
    fd = $fopen("work/p7.bin", "wb");
    $fwrite(fd, "%c%c%c%c%c", 8'h12, 8'h34, 8'hab, 8'hcd, 8'hef);
    $fclose(fd);
    fd = $fopen("work/p7.bin", "rb");
    h = 16'hxxxx;
    r = $fread(h, fd);
    $display("reg16 %0d %h", r, h);
    t = 12'h000;
    r = $fread(t, fd);
    $display("reg12 %0d %h", r, t);
    h = 16'h5a5a;
    r = $fread(h, fd);
    $display("partial %0d %h", r, h);
    r = $fread(h, fd);
    $display("at end %0d %h", r, h);
    $fclose(fd);
  end
endmodule
"#;
    let report = bench_in(&directory, source, false);
    assert_eq!(
        stdout(&report),
        "reg16 2 1234\n\
         reg12 2 bcd\n\
         partial 1 ef5a\n\
         at end 0 ef5a\n"
    );
    assert!(warnings(&report).is_empty());
}
