//! The fixed benchmark population: named suites of workloads, each a deck
//! and the command run on it.
//!
//! Generated suites (`neospice`, `flat`, `startup`) are written by this
//! module's own generators into `target/benchmarks/decks/<suite>/`; their
//! shapes follow published simulator-benchmark workloads, but every deck is
//! our own text. Corpus suites (`iscas`, `gf180`, `mixed`) name decks the
//! repository already carries under `tests/` and run them in place, because
//! they include their model files by relative path.

use crate::error::BenchError;
use clap::ValueEnum;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// The argument on which `rspice-bench` exits before doing anything else.
/// Spawning it so is the floor every process pays: a minimal Rust
/// executable, the same on every platform.
pub const SPAWN_FLOOR_ARG: &str = "--spawn-floor";

/// Program and arguments of the spawn floor: this executable, exiting at
/// once.
pub fn spawn_floor_command() -> Result<(PathBuf, Vec<OsString>), BenchError> {
    let exe = std::env::current_exe()
        .map_err(|error| BenchError::io("failed to locate the rspice-bench executable", error))?;
    Ok((exe, vec![SPAWN_FLOOR_ARG.into()]))
}

/// A named workload population.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Suite {
    /// Shapes of the neospice benchmark population: grids, meshes, ladders,
    /// noise, sweeps and op-amp op+ac cases.
    Neospice,
    /// The 40,002-element flat diode ladder, checked and solved.
    Flat,
    /// An ISCAS85 subset (BSIM4 gate-level transients).
    Iscas,
    /// One GF180MCU PDK device case.
    Gf180,
    /// Mixed-signal corpus designs.
    Mixed,
    /// Process cost: spawn floor, `--version`, and a five-line deck.
    Startup,
    /// Every suite above.
    All,
}

impl Suite {
    pub fn name(self) -> &'static str {
        match self {
            Self::Neospice => "neospice",
            Self::Flat => "flat",
            Self::Iscas => "iscas",
            Self::Gf180 => "gf180",
            Self::Mixed => "mixed",
            Self::Startup => "startup",
            Self::All => "all",
        }
    }
}

/// What a workload runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Spawn the floor ([`SPAWN_FLOOR_ARG`]): the cost every process pays.
    SpawnFloor,
    /// `rspice --version`.
    Version,
    /// `rspice check <deck>`.
    Check,
    /// `rspice run <deck> -q` (ngspice `-b -r <raw> <deck>`, which needs an
    /// output request to simulate at all).
    RunQuiet,
    /// `rspice run <deck> -q -o <raw>` (ngspice `-b -r <raw> <deck>`).
    RunRaw,
}

impl Action {
    /// The command as a report names it.
    pub fn label(self) -> &'static str {
        match self {
            Self::SpawnFloor => "spawn floor",
            Self::Version => "--version",
            Self::Check => "check",
            Self::RunQuiet => "run -q",
            Self::RunRaw => "run -q -o x.raw",
        }
    }

    /// Whether the action solves the deck and so has outputs to compare.
    pub fn simulates(self) -> bool {
        matches!(self, Self::RunQuiet | Self::RunRaw)
    }
}

/// One timed workload.
#[derive(Clone, Debug)]
pub struct Workload {
    /// Stable identifier, unique within a population.
    pub id: String,
    /// Suite the workload belongs to.
    pub suite: &'static str,
    /// What is run.
    pub action: Action,
    /// The deck, absolute; `None` for the spawn floor and `--version`.
    pub deck: Option<PathBuf>,
    /// Whether ngspice and ESpice can run the deck as context.
    pub context: bool,
    /// A workload of a minute or more: timed without warm-up, and for
    /// `--heavy-repeats` in `run` and `--heavy-pairs` in `paired`.
    pub heavy: bool,
}

impl Workload {
    /// A `run -q` workload over one deck, the shape `--circuits` produces.
    pub fn run_quiet(deck: PathBuf) -> Self {
        let id = deck
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        Self {
            id,
            suite: "circuits",
            action: Action::RunQuiet,
            deck: Some(deck),
            context: true,
            heavy: false,
        }
    }

    /// The directory a child runs in: the deck's own, so relative includes
    /// resolve and stray artifacts land beside the deck.
    pub fn cwd(&self) -> PathBuf {
        self.deck
            .as_deref()
            .and_then(Path::parent)
            .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    }

    fn deck_file(&self) -> OsString {
        self.deck
            .as_deref()
            .and_then(Path::file_name)
            .map(OsString::from)
            .unwrap_or_default()
    }

    /// Program and arguments for an RSpice executable. `raw` is where a
    /// simulating action writes its rawfile, when one is wanted.
    pub fn rspice_command(
        &self,
        rspice: &Path,
        raw: Option<&Path>,
    ) -> Result<(PathBuf, Vec<OsString>), BenchError> {
        let deck = self.deck_file();
        let args: Vec<OsString> = match self.action {
            Action::SpawnFloor => return spawn_floor_command(),
            Action::Version => vec!["--version".into()],
            Action::Check => vec!["check".into(), deck, "-q".into()],
            Action::RunQuiet | Action::RunRaw => {
                let mut args = vec!["run".into(), deck, "-q".into()];
                if let Some(raw) = raw {
                    args.push("-o".into());
                    args.push(raw.into());
                }
                args
            }
        };
        Ok((rspice.to_path_buf(), args))
    }

    /// Arguments for ngspice, when the workload has an ngspice equivalent.
    /// Both simulating actions write a rawfile to `raw`: ngspice's batch
    /// mode runs no analysis at all for a deck without an output request,
    /// so even its `run -q` counterpart writes one.
    pub fn ngspice_command(&self, raw: &Path) -> Option<Vec<OsString>> {
        (self.context && self.action.simulates())
            .then(|| vec!["-b".into(), "-r".into(), raw.into(), self.deck_file()])
    }

    /// Arguments for ESpice: its executable takes the deck. A simulating
    /// action only; ESpice's own flags are not assumed.
    pub fn espice_command(&self) -> Option<Vec<OsString>> {
        (self.context && self.action.simulates()).then(|| vec![self.deck_file()])
    }
}

/// The population of `suite`, generating its decks under `decks_dir` and
/// resolving corpus decks under the workspace `root`.
pub fn population(
    suite: Suite,
    root: &Path,
    decks_dir: &Path,
) -> Result<Vec<Workload>, BenchError> {
    let suites: &[Suite] = match suite {
        Suite::All => &[
            Suite::Startup,
            Suite::Flat,
            Suite::Neospice,
            Suite::Gf180,
            Suite::Iscas,
            Suite::Mixed,
        ],
        _ => std::slice::from_ref(&suite),
    };
    let mut workloads = Vec::new();
    for &suite in suites {
        workloads.extend(match suite {
            Suite::Startup => startup(decks_dir)?,
            Suite::Flat => flat(decks_dir)?,
            Suite::Neospice => neospice(root, decks_dir)?,
            Suite::Iscas => corpus(root, "iscas", ISCAS_DECKS, true, true)?,
            Suite::Gf180 => corpus(root, "gf180", GF180_DECKS, true, false)?,
            Suite::Mixed => corpus(root, "mixed", MIXED_DECKS, false, false)?,
            Suite::All => Vec::new(),
        });
    }
    Ok(workloads)
}

fn write_deck(dir: &Path, file_name: &str, text: &str) -> Result<PathBuf, BenchError> {
    fs::create_dir_all(dir).map_err(|error| {
        BenchError::io(
            format!("failed to create deck directory `{}`", dir.display()),
            error,
        )
    })?;
    let path = dir.join(file_name);
    fs::write(&path, text).map_err(|error| {
        BenchError::io(
            format!("failed to write generated deck `{}`", path.display()),
            error,
        )
    })?;
    absolute(&path)
}

fn absolute(path: &Path) -> Result<PathBuf, BenchError> {
    std::path::absolute(path).map_err(|error| {
        BenchError::io(
            format!("failed to resolve deck path `{}`", path.display()),
            error,
        )
    })
}

fn workload(
    id: impl Into<String>,
    suite: &'static str,
    action: Action,
    deck: Option<PathBuf>,
    context: bool,
) -> Workload {
    Workload {
        id: id.into(),
        suite,
        action,
        deck,
        context,
        heavy: false,
    }
}

// ---------------------------------------------------------------------------
// startup
// ---------------------------------------------------------------------------

/// Five lines: the smallest deck that elaborates, solves and publishes.
const FIVE_LINE_DECK: &str = "* five-line process-cost deck\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";

fn startup(decks_dir: &Path) -> Result<Vec<Workload>, BenchError> {
    let deck = write_deck(&decks_dir.join("startup"), "five_line.cir", FIVE_LINE_DECK)?;
    Ok(vec![
        workload("spawn_floor", "startup", Action::SpawnFloor, None, false),
        workload("version", "startup", Action::Version, None, false),
        workload(
            "five_line_check",
            "startup",
            Action::Check,
            Some(deck.clone()),
            false,
        ),
        workload(
            "five_line_run_q",
            "startup",
            Action::RunQuiet,
            Some(deck.clone()),
            true,
        ),
        workload(
            "five_line_run_raw",
            "startup",
            Action::RunRaw,
            Some(deck),
            true,
        ),
    ])
}

// ---------------------------------------------------------------------------
// flat
// ---------------------------------------------------------------------------

fn flat(decks_dir: &Path) -> Result<Vec<Workload>, BenchError> {
    let deck = write_deck(
        &decks_dir.join("flat"),
        "diode_ladder_20000.cir",
        &diode_ladder(20_000),
    )?;
    Ok(vec![
        workload(
            "flat40k_check",
            "flat",
            Action::Check,
            Some(deck.clone()),
            false,
        ),
        workload("flat40k_run_q", "flat", Action::RunQuiet, Some(deck), true),
    ])
}

/// `stages` series diodes, each stage loaded to ground: `stages` diodes,
/// `stages + 1` resistors and one source, flat, solved with `.op`.
fn diode_ladder(stages: usize) -> String {
    let mut deck = String::new();
    let _ = writeln!(
        deck,
        "* series diode ladder, {stages} stages, {} elements",
        2 * stages + 2
    );
    deck.push_str("* GENERATED by `rspice-bench`; do not edit by hand.\n");
    deck.push_str(".model DLAD D(IS=2e-14 N=1.05)\n");
    deck.push_str("VS top 0 DC 5\n");
    deck.push_str("RS top s0 1k\n");
    for stage in 0..stages {
        let _ = writeln!(deck, "D{stage} s{stage} s{} DLAD", stage + 1);
        let _ = writeln!(deck, "RL{stage} s{} 0 10k", stage + 1);
    }
    deck.push_str(".op\n.end\n");
    deck
}

// ---------------------------------------------------------------------------
// neospice shapes
// ---------------------------------------------------------------------------

/// The grid drive: a DC bias, an AC stimulus and a repeating pulse.
const GRID_SOURCE: &str = "VG in 0 DC 0.7 AC 1 PULSE(0.25 0.75 0 8u 8u 45u 100u)";

/// An `side` x `side` RC grid: 1k between orthogonal neighbours, 1n from
/// every node to ground, fed at one corner through 1k and loaded at the far
/// corner, `out`, by 1k. With `diodes`, a junction diode from every node to
/// ground as well.
fn grid(side: usize, diodes: bool, analysis: &str) -> String {
    let node = |row: usize, column: usize| {
        if row + 1 == side && column + 1 == side {
            "out".to_string()
        } else {
            format!("g{row}_{column}")
        }
    };
    let mut deck = String::new();
    let _ = writeln!(
        deck,
        "* {side}x{side} {}RC grid",
        if diodes { "diode-" } else { "" }
    );
    deck.push_str("* GENERATED by `rspice-bench`; do not edit by hand.\n");
    deck.push_str(GRID_SOURCE);
    deck.push('\n');
    if diodes {
        deck.push_str(".model DGR D(IS=2e-12 N=1.3 RS=3 CJO=2p M=0.45 TT=2n)\n");
    }
    let _ = writeln!(deck, "RFEED in {} 1k", node(0, 0));
    for row in 0..side {
        for column in 0..side {
            let here = node(row, column);
            if column + 1 < side {
                let _ = writeln!(deck, "RH{row}_{column} {here} {} 1k", node(row, column + 1));
            }
            if row + 1 < side {
                let _ = writeln!(deck, "RV{row}_{column} {here} {} 1k", node(row + 1, column));
            }
            let _ = writeln!(deck, "C{row}_{column} {here} 0 1n");
            if diodes {
                let _ = writeln!(deck, "D{row}_{column} {here} 0 DGR");
            }
        }
    }
    deck.push_str("RLOAD out 0 1k\n");
    deck.push_str(analysis);
    deck.push_str("\n.end\n");
    deck
}

/// An `side` x `side` resistor mesh driven at one corner and returned to
/// ground at the other.
fn mesh(side: usize) -> String {
    let node = |row: usize, column: usize| format!("m{row}_{column}");
    let mut deck = String::new();
    let _ = writeln!(deck, "* {side}x{side} resistor mesh");
    deck.push_str("* GENERATED by `rspice-bench`; do not edit by hand.\n");
    let _ = writeln!(deck, "VM {} 0 DC 1", node(0, 0));
    let mut count = 0usize;
    for row in 0..side {
        for column in 0..side {
            if column + 1 < side {
                let _ = writeln!(
                    deck,
                    "R{count} {} {} 1k",
                    node(row, column),
                    node(row, column + 1)
                );
                count += 1;
            }
            if row + 1 < side {
                let _ = writeln!(
                    deck,
                    "R{count} {} {} 1k",
                    node(row, column),
                    node(row + 1, column)
                );
                count += 1;
            }
        }
    }
    let _ = writeln!(deck, "R{count} {} 0 1k", node(side - 1, side - 1));
    deck.push_str(".op\n.end\n");
    deck
}

/// A two-resistor divider under `analysis`.
fn divider(analysis: &str) -> String {
    format!(
        "* resistor divider\n* GENERATED by `rspice-bench`; do not edit by hand.\n\
         VD in 0 DC 5 AC 1\nRTOP in out 10k\nRBOT out 0 10k\n{analysis}\n.end\n"
    )
}

fn rc_lowpass(analysis: &str) -> String {
    format!(
        "* RC low-pass\n* GENERATED by `rspice-bench`; do not edit by hand.\n\
         VR in 0 DC 0 AC 1 PULSE(0 1 0 1u 1u 100u 200u)\nR1 in out 1k\nC1 out 0 100n\n\
         {analysis}\n.end\n"
    )
}

fn rlc_series(analysis: &str) -> String {
    format!(
        "* series RLC\n* GENERATED by `rspice-bench`; do not edit by hand.\n\
         VL in 0 DC 0 PULSE(0 1 0 100n 100n 20u 40u)\nR1 in a 20\nL1 a b 100u\nC1 b 0 10n\n\
         {analysis}\n.end\n"
    )
}

/// A PULSE written with its trailing parameters omitted, so each simulator
/// fills them with its defaults.
fn pulse_defaults(analysis: &str) -> String {
    format!(
        "* PULSE with defaulted parameters\n* GENERATED by `rspice-bench`; do not edit by hand.\n\
         VP in 0 PULSE(0 2 5u)\nR1 in out 1k\nC1 out 0 10n\n{analysis}\n.end\n"
    )
}

/// The bipolar op-amp the repository's Xyce corpus carries (CircuitSim90
/// `opampal`, transistor level), standing in for the vendor op-amp
/// macromodels neospice benchmarks (TI THS4131/OPA1632), which cannot be
/// shipped. Its `.MODEL` cards and `.SUBCKT` are read from the corpus file
/// at generation time.
const OPAMPAL_SOURCE: &str = "tests/xyce/Netlists/MCNC_BJT_OPAMPAL/opampal.cir";

fn opampal_library(root: &Path) -> Result<String, BenchError> {
    let path = root.join(OPAMPAL_SOURCE);
    let text = fs::read_to_string(&path).map_err(|error| {
        BenchError::io(
            format!("failed to read op-amp source `{}`", path.display()),
            error,
        )
    })?;
    let start = text
        .lines()
        .position(|line| line.to_ascii_uppercase().starts_with(".MODEL"))
        .ok_or(BenchError::Internal("op-amp source has no .MODEL card"))?;
    let end = text
        .lines()
        .position(|line| line.to_ascii_uppercase().starts_with(".ENDS"))
        .ok_or(BenchError::Internal("op-amp source has no .ENDS card"))?;
    let mut library = String::new();
    for line in text.lines().skip(start).take(end + 1 - start) {
        library.push_str(line);
        library.push('\n');
    }
    Ok(library)
}

/// Two inverting stages of gain ten in a differential pair, on +/-15 V.
fn opampal_differential(library: &str, analyses: &str) -> String {
    format!(
        "* differential gain stage on two bipolar op-amps\n\
         * GENERATED by `rspice-bench` from {OPAMPAL_SOURCE}; do not edit by hand.\n\
         {library}\
         VPOS vp 0 15\nVNEG vn 0 -15\n\
         VIP ip 0 DC 0 AC 0.5\nVIN in 0 DC 0 AC 0.5 180\n\
         RI1 ip a 1k\nRF1 a on 10k\nX1 0 a on vp vn OPAMPAL\n\
         RI2 in b 1k\nRF2 b op 10k\nX2 0 b op vp vn OPAMPAL\n\
         RL op on 2k\n{analyses}\n.end\n"
    )
}

/// Four unity-gain followers in series, the corpus deck's own topology.
fn opampal_chain(library: &str, analyses: &str) -> String {
    format!(
        "* four bipolar op-amp followers in series\n\
         * GENERATED by `rspice-bench` from {OPAMPAL_SOURCE}; do not edit by hand.\n\
         {library}\
         VPOS vp 0 15\nVNEG vn 0 -15\nVI n0 0 DC 0.5 AC 1\n\
         XA n0 n1 n1 vp vn OPAMPAL\nXB n1 n2 n2 vp vn OPAMPAL\n\
         XC n2 n3 n3 vp vn OPAMPAL\nXD n3 out out vp vn OPAMPAL\n\
         RL out 0 10k\n{analyses}\n.end\n"
    )
}

const GRID_AC: &str = ".ac dec 10 1 1meg";
const GRID_TRAN: &str = ".tran 1u 500u";
const GRID_NOISE: &str = ".noise v(out) vg dec 10 1 1meg";

fn neospice(root: &Path, decks_dir: &Path) -> Result<Vec<Workload>, BenchError> {
    let dir = decks_dir.join("neospice");
    let library = opampal_library(root)?;
    let mut decks: Vec<(String, String)> = vec![
        ("dc_divider".into(), divider(".op")),
        ("sweep_divider".into(), divider(".dc vd -5 5 0.01")),
        (
            "noise_divider".into(),
            divider(".noise v(out) vd dec 10 1 1g"),
        ),
        ("ac_rc".into(), rc_lowpass(".ac dec 10 1 1g")),
        ("tran_rc".into(), rc_lowpass(".tran 1u 500u")),
        ("tran_rlc".into(), rlc_series(".tran 100n 100u")),
        ("tran_pulse".into(), pulse_defaults(".tran 100n 100u")),
        (
            "dc_opampal_diff".into(),
            opampal_differential(&library, ".op"),
        ),
        (
            "ac_opampal_diff_10".into(),
            opampal_differential(&library, ".ac dec 10 1 100meg"),
        ),
        (
            "ac_opampal_diff_1000".into(),
            opampal_differential(&library, ".ac dec 1000 1 100meg"),
        ),
        (
            "op_ac_opampal_diff".into(),
            opampal_differential(&library, ".op\n.ac dec 10 1 100meg"),
        ),
        (
            "op_ac_opampal_chain".into(),
            opampal_chain(&library, ".op\n.ac dec 10 1 100meg"),
        ),
    ];
    for side in [10, 32, 71, 141] {
        decks.push((format!("dc_mesh_{side}"), mesh(side)));
    }
    for stages in [100, 1000, 5000, 20_000] {
        decks.push((format!("dc_diode_ladder_{stages}"), diode_ladder(stages)));
    }
    for diodes in [false, true] {
        let family = if diodes { "diode_rc_grid" } else { "rc_grid" };
        for side in [8, 24, 48] {
            decks.push((format!("ac_{family}_{side}"), grid(side, diodes, GRID_AC)));
            decks.push((
                format!("tran_{family}_{side}"),
                grid(side, diodes, GRID_TRAN),
            ));
        }
        decks.push((format!("noise_{family}_24"), grid(24, diodes, GRID_NOISE)));
    }
    decks
        .into_iter()
        .map(|(id, text)| {
            let deck = write_deck(&dir, &format!("{id}.cir"), &text)?;
            Ok(workload(id, "neospice", Action::RunQuiet, Some(deck), true))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// corpus suites
// ---------------------------------------------------------------------------

/// Two small ISCAS85 gate-level circuits over a 45 nm BSIM4 card, each a
/// minute-scale transient. Not c432, the smallest: its `.control` block
/// plots, which writes files beside the deck, into the source tree.
const ISCAS_DECKS: &[(&str, &str)] = &[
    ("iscas_c499", "tests/iscas85/85/c499/c499.net"),
    ("iscas_c880", "tests/iscas85/85/c880/c880.net"),
];

/// One GF180MCU BSIM4 I-V case: the PDK library parse dominates it.
const GF180_DECKS: &[(&str, &str)] = &[(
    "gf180_nmos_3p3_w10_l5",
    "tests/gf180mcu/cases/mos_iv_vds/nmos_3p3_W10_L5_T25_Vg3.3.spice",
)];

/// Mixed-signal designs the default build runs.
const MIXED_DECKS: &[(&str, &str)] = &[
    (
        "mixed_c04_sar8_ports_op",
        "tests/mixed-corpus/c04-sar8/deck/ports.op.cir",
    ),
    (
        "mixed_c12_ldo_top_op",
        "tests/mixed-corpus/c12-ldo-tb/deck/top.op.cir",
    ),
    (
        "mixed_c08_ladder_va_ac",
        "tests/mixed-corpus/c08-ladder/deck/va.ac.cir",
    ),
];

fn corpus(
    root: &Path,
    suite: &'static str,
    decks: &[(&str, &str)],
    context: bool,
    heavy: bool,
) -> Result<Vec<Workload>, BenchError> {
    decks
        .iter()
        .map(|(id, relative)| {
            let deck = root.join(relative);
            if !deck.is_file() {
                return Err(BenchError::io(
                    format!("corpus deck `{}` is missing", deck.display()),
                    std::io::Error::from(std::io::ErrorKind::NotFound),
                ));
            }
            Ok(Workload {
                heavy,
                ..workload(*id, suite, Action::RunQuiet, Some(deck), context)
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_prefixed(deck: &str, prefix: char) -> usize {
        deck.lines().filter(|line| line.starts_with(prefix)).count()
    }

    #[test]
    fn the_flat_ladder_has_forty_thousand_and_two_elements() {
        let deck = diode_ladder(20_000);
        let elements =
            count_prefixed(&deck, 'D') + count_prefixed(&deck, 'R') + count_prefixed(&deck, 'V');
        assert_eq!(elements, 40_002);
        assert_eq!(count_prefixed(&deck, 'D'), 20_000);
    }

    #[test]
    fn grids_load_their_far_corner_and_place_one_capacitor_per_node() {
        let deck = grid(8, true, GRID_TRAN);
        assert_eq!(count_prefixed(&deck, 'C'), 64);
        assert_eq!(count_prefixed(&deck, 'D'), 64);
        // 2 * 8 * 7 neighbour resistors, the feed and the load.
        assert_eq!(count_prefixed(&deck, 'R'), 2 * 8 * 7 + 2);
        assert!(deck.contains("RLOAD out 0 1k"));
        assert!(deck.ends_with(".tran 1u 500u\n.end\n"));
    }

    #[test]
    fn the_five_line_deck_is_five_lines() {
        assert_eq!(FIVE_LINE_DECK.lines().count(), 5);
    }

    #[test]
    fn the_population_is_complete_and_uniquely_named() {
        let root = crate::workspace_root();
        let decks = tempfile::tempdir().expect("temporary deck directory");
        let workloads = population(Suite::All, &root, decks.path()).expect("population");
        let mut ids: Vec<&str> = workloads.iter().map(|w| w.id.as_str()).collect();
        let neospice = workloads.iter().filter(|w| w.suite == "neospice").count();
        assert_eq!(neospice, 34);
        ids.sort_unstable();
        let before = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), before, "workload ids must be unique");
        for workload in &workloads {
            if let Some(deck) = &workload.deck {
                assert!(
                    deck.is_file(),
                    "{} has no deck at {}",
                    workload.id,
                    deck.display()
                );
            }
        }
    }
}
