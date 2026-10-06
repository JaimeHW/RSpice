mod common;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(dir: &Path, family: &str, filename: &str) -> (PathBuf, PathBuf) {
    let (data, body) = match family {
        "filesource" => (
            "0 1\n1e-9 1\n",
            format!("A1 out fs\n.model fs filesource (file=\"{filename}\")\n"),
        ),
        "instance" => (
            "0 1\n1e-9 1\n",
            format!("A1 out file_source file=\"{filename}\"\n"),
        ),
        "hierarchy" => (
            "0 1\n1e-9 1\n",
            format!(
                ".subckt child out filename=\"unused.dat\"\nA1 out fs\n.model fs filesource (file=filename)\n.ends\nX1 out child filename=\"{filename}\"\n"
            ),
        ),
        "table2d" => (
            "2\n2\n0 1\n0 1\n0 1\n2 3\n",
            format!(
                "V1 x 0 0.5\nV2 y 0 0.25\nA1 x y out tab\n.model tab table2d (file=\"{filename}\" order=2)\n"
            ),
        ),
        "xfer" => (
            "# Hz S RI R 50\n1 1 0\n2 1 0\n",
            format!("V1 in 0 1\nA1 in out xf\n.model xf xfer (file=\"{filename}\")\n"),
        ),
        _ => unreachable!(),
    };
    let source = dir.join(filename);
    let deck = dir.join("deck.cir");
    std::fs::write(&source, data).unwrap();
    std::fs::write(
        &deck,
        format!("XSPICE ownership\n{body}R1 out 0 1k\n.op\n.end\n"),
    )
    .unwrap();
    (deck, source)
}

#[test]
fn model_inputs_are_preserved_for_all_output_roles() {
    for family in ["filesource", "instance", "hierarchy", "table2d", "xfer"] {
        for flags in [
            vec!["-o"],
            vec!["--checkpoint"],
            vec!["--summary"],
            vec!["--meas-file"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            let dir = common::test_dir("xspice_source_ownership");
            let (deck, source) = fixture(&dir, family, "source.dat");
            let original = std::fs::read(&source).unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "csv"])
                .args(&flags)
                .arg(&source)
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(2),
                "{family}, {flags:?}: {result:?}"
            );
            assert!(
                String::from_utf8_lossy(&result.stderr).contains("source"),
                "{result:?}"
            );
            assert_eq!(std::fs::read(&source).unwrap(), original);
        }
    }
}

#[test]
fn normal_and_parallel_runs_use_the_same_model_input_inventory() {
    for family in ["filesource", "instance", "hierarchy", "table2d", "xfer"] {
        let dir = common::test_dir("xspice_parallel_sources");
        let (deck, source) = fixture(&dir, family, "result.ss.csv");
        let original = std::fs::read(&source).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["-f", "csv", "-o"])
            .arg(dir.join("ordinary.csv"))
            .output()
            .unwrap();
        assert!(result.status.success(), "{family}: {result:?}");
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--corners", "tt,ss", "-j", "2", "-f", "csv", "-o"])
            .arg(dir.join("result.csv"))
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2), "{family}: {result:?}");
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("source"),
            "{result:?}"
        );
        assert_eq!(std::fs::read(&source).unwrap(), original);
    }
}

#[test]
fn default_inputs_and_search_directory_shadowing_are_protected() {
    for in_search_dir in [false, true] {
        let dir = common::test_dir("xspice_default_source");
        let search = dir.join("search");
        std::fs::create_dir(&search).unwrap();
        let deck = dir.join("deck.cir");
        std::fs::write(
            &deck,
            "defaults\nA1 out filesource\nR1 out 0 1k\n.op\n.end\n",
        )
        .unwrap();
        let source = if in_search_dir {
            search.join("filesource.txt")
        } else {
            dir.join("filesource.txt")
        };
        std::fs::write(&source, "0 1\n1e-9 1\n").unwrap();
        // Both overwriting an existing file and creating a preferred candidate
        // change what later reads of the same model will consume.
        for output in [source.clone(), search.join("filesource.txt")] {
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "csv", "-o"])
                .arg(&output)
                .current_dir(&dir)
                .env("NGSPICE_INPUT_DIR", &search)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(2), "{result:?}");
            assert!(
                String::from_utf8_lossy(&result.stderr).contains("source"),
                "{result:?}"
            );
            assert_eq!(std::fs::read_to_string(&source).unwrap(), "0 1\n1e-9 1\n");
            if !in_search_dir {
                assert!(!search.join("filesource.txt").exists());
            }
        }
    }
}

#[test]
fn a_later_step_model_input_is_reserved_before_publication() {
    let dir = common::test_dir("xspice_step_sources");
    let (deck, source) = fixture(&dir, "filesource", "later.dat");
    std::fs::write(dir.join("first.dat"), "0 1\n1e-9 1\n").unwrap();
    std::fs::write(&deck, "steps\n.param choice=0\n.step param choice list 0 1\n.if (choice == 0)\n.model fs filesource (file=\"first.dat\")\n.else\n.model fs filesource (file=\"later.dat\")\n.endif\nA1 out fs\nR1 out 0 1k\n.op\n.end\n").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["-f", "csv", "-o"])
        .arg(&source)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("source"),
        "{result:?}"
    );
    assert_eq!(std::fs::read_to_string(&source).unwrap(), "0 1\n1e-9 1\n");
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 3);
}

#[test]
fn unused_model_inputs_do_not_reserve_output_paths() {
    let dir = common::test_dir("unused_xspice_source");
    let (deck, source) = fixture(&dir, "filesource", "source.dat");
    std::fs::write(&deck, "unused\n.model unused filesource (file=\"source.dat\")\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["-f", "csv", "-o"])
        .arg(&source)
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    assert!(
        std::fs::read_to_string(&source)
            .unwrap()
            .starts_with("signal,value")
    );
}
