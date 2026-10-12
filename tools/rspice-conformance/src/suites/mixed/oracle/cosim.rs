//! O2: ngspice-46 with `d_cosim` running Icarus Verilog (`ivlng` + libvvp).
//!
//! `oracle/<variant>.o2/build.txt` holds the exact `iverilog -o <name> <files>`
//! command; `deck.cir` is the ngspice deck that loads the result through a
//! `d_cosim` model naming `simulation="ivlng"`. The checked-in deck stays
//! portable: at capture the model card is rewritten to the discovered shim
//! (`simulation="<RSPICE_NGSPICE_COSIM_LIB>" lib_args=["libvvp"
//! "<RSPICE_NGSPICE_COSIM_VPI>"]`), because `ivlng.dll`'s compiled-in VPI path
//! need not exist. `libvvp` itself is loaded through `PATH`, so the capture
//! runs under the oracle environment (`oracles/env.sh`).
//!
//! ngspice exits 0 when `d_cosim` fails to load and simulates on without the
//! digital block, so the run's log is scanned for the load failures and any is
//! an oracle failure.

use std::path::{Path, PathBuf};
use std::time::Duration;

use super::OracleAbsent;
use crate::suites::verilog::oracle::{OracleAvailability, VerilogEngine, detect};

pub const DEFAULT_COSIM_LIB: &str =
    "C:/Users/James/Desktop/ngspice-46-release/Spice64/lib/ngspice/ivlng.dll";
pub const DEFAULT_COSIM_VPI: &str =
    "C:/Users/James/Desktop/ngspice-46-release/Spice64/lib/ngspice/ivlng.vpi";

/// Log lines that mean the digital block never ran.
pub const LOAD_FAILURES: [&str; 3] = [
    "d_cosim failed to load",
    "mismatched XSPICE/co-simulator",
    "Unable to find module",
];

/// The tools O2 needs: ngspice, `iverilog`, the `ivlng` shim and its VPI
/// module (which in turn needs a `libvvp` that Icarus only builds with
/// `--enable-libvvp`).
pub struct CosimTools {
    pub ngspice: PathBuf,
    pub iverilog: PathBuf,
    pub cosim_library: PathBuf,
    pub cosim_vpi: PathBuf,
}

pub fn discover() -> Result<CosimTools, OracleAbsent> {
    let ngspice = super::ngspice::discover()?;
    let cosim_library = super::discover(
        "d_cosim ivlng (O2)",
        "RSPICE_NGSPICE_COSIM_LIB",
        DEFAULT_COSIM_LIB,
        "ngspice's ivlng shim (Spice64/lib/ngspice/ivlng.dll)",
    )?;
    let cosim_vpi = super::discover(
        "d_cosim ivlng VPI (O2)",
        "RSPICE_NGSPICE_COSIM_VPI",
        DEFAULT_COSIM_VPI,
        "ngspice's ivlng VPI module (Spice64/lib/ngspice/ivlng.vpi)",
    )?;
    let iverilog = match detect(VerilogEngine::Icarus) {
        OracleAvailability::Available(tools) => tools
            .program("compiler")
            .map(|program| PathBuf::from(&program.program))
            .ok_or_else(|| absent("Icarus resolved without a compiler"))?,
        other => return Err(absent(&other.diagnostic())),
    };
    if !libvvp_present(&iverilog) {
        return Err(absent(
            "iverilog found, but no libvvp beside it; O2 needs Icarus built with --enable-libvvp (P0.3)",
        ));
    }
    Ok(CosimTools {
        ngspice,
        iverilog,
        cosim_library,
        cosim_vpi,
    })
}

fn absent(probed: &str) -> OracleAbsent {
    OracleAbsent {
        oracle: "ngspice + d_cosim + Icarus (O2)",
        probed: vec![probed.to_owned()],
        hint: "install Icarus Verilog built with --enable-libvvp (P0.3), source oracles/env.sh, then run `rspice-mixed-corpus capture --oracle O2`",
    }
}

fn libvvp_present(iverilog: &Path) -> bool {
    let Some(directory) = iverilog.parent() else {
        return false;
    };
    ["libvvp.dll", "libvvp.so", "libvvp.dylib"]
        .iter()
        .any(|name| directory.join(name).is_file() || directory.join("../lib").join(name).is_file())
}

/// Point every `simulation="ivlng"` d_cosim model at the discovered shim.
pub fn bind_cosim_models(source: &str, library: &Path, vpi: &Path) -> Result<String, String> {
    let library = library.display().to_string().replace('\\', "/");
    let vpi = vpi.display().to_string().replace('\\', "/");
    let mut bound = 0usize;
    let text = source
        .lines()
        .map(|line| {
            if line.to_ascii_lowercase().contains("d_cosim")
                && line.contains("simulation=\"ivlng\"")
            {
                bound += 1;
                line.replace(
                    "simulation=\"ivlng\"",
                    &format!("simulation=\"{library}\" lib_args=[\"libvvp\" \"{vpi}\"]"),
                )
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    if bound == 0 {
        return Err("O2 deck has no d_cosim model with simulation=\"ivlng\"".into());
    }
    Ok(text + "\n")
}

/// Build the Verilog with the deck's `build.txt`, then run the deck in ngspice.
pub(crate) fn run(
    tools: &CosimTools,
    deck: &Path,
    timeout: Duration,
) -> Result<super::ngspice::NgspiceRun, String> {
    let directory = deck.parent().ok_or("O2 deck has no directory")?;
    let build = std::fs::read_to_string(directory.join("build.txt"))
        .map_err(|error| format!("O2 build.txt: {error}"))?;
    let words: Vec<&str> = build.split_whitespace().collect();
    let (Some(&"iverilog"), Some(output)) = (
        words.first(),
        words
            .windows(2)
            .find(|pair| pair[0] == "-o")
            .map(|pair| pair[1]),
    ) else {
        return Err("build.txt must be one `iverilog -o <name> ...` command".into());
    };
    // The compiled image is written beside the deck, which loads it by its
    // relative name; it is a build product, so it goes when the run ends.
    let image = directory.join(output);
    let run = build_and_run(tools, directory, &words[1..], deck, timeout);
    let _ = std::fs::remove_file(&image);
    run
}

fn build_and_run(
    tools: &CosimTools,
    directory: &Path,
    arguments: &[&str],
    deck: &Path,
    timeout: Duration,
) -> Result<super::ngspice::NgspiceRun, String> {
    let status = std::process::Command::new(&tools.iverilog)
        .args(arguments)
        .current_dir(directory)
        .status()
        .map_err(|error| format!("iverilog: {error}"))?;
    if !status.success() {
        return Err(format!("iverilog exited with {status}"));
    }
    let source =
        std::fs::read_to_string(deck).map_err(|error| format!("{}: {error}", deck.display()))?;
    let source = bind_cosim_models(&source, &tools.cosim_library, &tools.cosim_vpi)?;
    let run = super::ngspice::run_source(&tools.ngspice, directory, &source, timeout)?;
    if let Some(failure) = LOAD_FAILURES
        .iter()
        .find(|failure| run.log.contains(**failure))
    {
        return Err(format!(
            "d_cosim did not run the Verilog ({failure}); ngspice simulated without the digital block"
        ));
    }
    Ok(run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ivlng_models_are_bound_to_the_discovered_shim() {
        let deck =
            "t\naq [a] [b] m\n.model m d_cosim simulation=\"ivlng\" sim_args=[\"top\"]\n.end\n";
        let bound = bind_cosim_models(
            deck,
            Path::new("C:/ng/ivlng.dll"),
            Path::new("C:/ng/ivlng.vpi"),
        )
        .unwrap();
        assert!(
            bound.contains("simulation=\"C:/ng/ivlng.dll\" lib_args=[\"libvvp\" \"C:/ng/ivlng.vpi\"] sim_args=[\"top\"]"),
            "{bound}"
        );
        assert!(bind_cosim_models("t\n.end\n", Path::new("a"), Path::new("b")).is_err());
    }
}
