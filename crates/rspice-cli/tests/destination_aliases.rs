mod common;
use std::process::Command;

const DECK: &str = "* source alias\nV1 in 0 1\nR1 in 0 1k\n.tran 1n 2n\n.end\n";

#[test]
fn replacing_a_distinct_hard_link_preserves_the_source_entry() {
    let dir = common::test_dir("hard_link_output");
    let source = dir.join("source.cir");
    let output = dir.join("output.csv");
    std::fs::write(&source, DECK).unwrap();
    std::fs::hard_link(&source, &output).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&source)
        .args(["-f", "csv", "-o"])
        .arg(&output)
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    assert_eq!(std::fs::read_to_string(source).unwrap(), DECK);
    assert!(
        std::fs::read_to_string(output)
            .unwrap()
            .starts_with("time,")
    );
}

#[cfg(windows)]
mod windows {
    use super::*;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use std::path::{Path, PathBuf};

    fn short_alias(path: &Path) -> Option<PathBuf> {
        let wide: Vec<_> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let mut buffer = vec![0_u16; 32768];
        // SAFETY: the input is NUL-terminated and the output buffer has the
        // advertised capacity. GetShortPathNameW retains neither pointer.
        let count = unsafe {
            windows_sys::Win32::Storage::FileSystem::GetShortPathNameW(
                wide.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len() as u32,
            )
        };
        assert!(count > 0, "{}", std::io::Error::last_os_error());
        assert!((count as usize) < buffer.len());
        buffer.truncate(count as usize);
        let alias = PathBuf::from(std::ffi::OsString::from_wide(&buffer));
        if alias.file_name() == path.file_name() {
            eprintln!(
                "8.3 names are disabled for {}; alias probe unavailable",
                path.display()
            );
            return None;
        }
        assert_eq!(std::fs::read(&alias).unwrap(), std::fs::read(path).unwrap());
        Some(alias)
    }

    #[test]
    fn short_names_cannot_bypass_source_protection_for_run_artifacts() {
        for flags in [
            vec!["-o"],
            vec!["--checkpoint"],
            vec!["--summary"],
            vec!["--meas-file"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            let dir = common::test_dir("short_source_output");
            let source = dir.join("long_circuit_source_filename.cir");
            std::fs::write(&source, DECK).unwrap();
            let Some(alias) = short_alias(&source) else {
                return;
            };
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&source)
                .args(["-f", "csv"])
                .args(&flags)
                .arg(&alias)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(2), "{flags:?}: {result:?}");
            assert!(String::from_utf8_lossy(&result.stderr).contains("source"));
            assert_eq!(std::fs::read_to_string(&source).unwrap(), DECK);
            assert_eq!(std::fs::read_to_string(alias).unwrap(), DECK);
            assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
        }
    }

    #[test]
    fn short_names_cannot_replace_compiler_sources_or_loaded_configuration() {
        let dir = common::test_dir("short_tool_source");
        let model = dir.join("long_resistor_model_filename.va");
        let config = dir.join("long_configuration_filename.toml");
        let waveform = dir.join("wave.csv");
        let model_text = "module resistor(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n)/1000; endmodule\n";
        let config_text = "[output]\nformat='csv'\n";
        std::fs::write(&model, model_text).unwrap();
        std::fs::write(&config, config_text).unwrap();
        std::fs::write(&waveform, "time,V(out)\n0,1\n1,1\n").unwrap();
        let (Some(model_alias), Some(config_alias)) = (short_alias(&model), short_alias(&config))
        else {
            return;
        };
        for (command, input, output) in [
            ("compile-va", &model, &model_alias),
            ("compile-va", &model, &config_alias),
            ("convert", &waveform, &config_alias),
            ("compare", &waveform, &config_alias),
        ] {
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_rspice"));
            cmd.args(["--quiet", "--config"])
                .arg(&config)
                .arg(command)
                .arg(input);
            if command == "compile-va" {
                cmd.arg("-o");
            }
            cmd.arg(output);
            if command == "convert" {
                cmd.args(["--to", "csv"]);
            }
            if command == "compare" {
                cmd.arg("--bless");
            }
            let result = cmd.output().unwrap();
            assert_eq!(result.status.code(), Some(2), "{command}: {result:?}");
            assert!(String::from_utf8_lossy(&result.stderr).contains("source"));
            assert_eq!(std::fs::read_to_string(&model).unwrap(), model_text);
            assert_eq!(std::fs::read_to_string(&config).unwrap(), config_text);
        }
    }

    #[test]
    fn long_and_short_output_spellings_are_one_destination() {
        let dir = common::test_dir("short_output_collision");
        let source = dir.join("deck.cir");
        let destination = dir.join("long_result_destination_filename.csv");
        std::fs::write(&source, DECK).unwrap();
        std::fs::write(&destination, "previous result").unwrap();
        let Some(alias) = short_alias(&destination) else {
            return;
        };
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&source)
            .args(["-f", "csv", "-o"])
            .arg(&destination)
            .arg("--summary")
            .arg(&alias)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2), "{result:?}");
        assert!(String::from_utf8_lossy(&result.stderr).contains("share output destination"));
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "previous result"
        );
        assert_eq!(std::fs::read_to_string(alias).unwrap(), "previous result");
    }
}
