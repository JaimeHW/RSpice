//! How wide the live lane sets actually are.
//!
//! The emitter has a choice to make about derivative layout, and it turns on one
//! measurement. Packing lanes into `[f64; L]` collapses `L` emitted lines into
//! one, which is the remaining lever on source size — but only if `L` is a
//! per-model constant, which means a value carries every lane whether or not it
//! can reach the unknown. The cost of that is arithmetic on lanes that are
//! structurally zero.
//!
//! So: over the values that carry any lane at all, how many do they carry? If
//! the mean is close to the model's lane count, dense packing is nearly free and
//! the decision is easy. If it is a small fraction, dense packing trades a large
//! runtime cost for the size win and the layout has to be sparser than that.

use rspice_veriloga::canonical_ir::AdSeed;
use rspice_veriloga::canonical_ir::ad::lane_liveness;
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const MODELS: &[(&str, &str, &str)] = &[
    ("r3_cmc", "cmc/r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"),
    (
        "DIODE_CMC",
        "cmc/diode_cmc_3.0_20250714/vacode",
        "diode_cmc.va",
    ),
    ("EPFL_HEMT_10a", "epfl_hemt_3.0.0/vacode", "epfl_hemt.va"),
    ("vbic_4T_et_cf", "vbic_1.3/vacode", "vbic_4T_et_cf.va"),
    ("bsimbulk", "cmc/BSIM-BULK107.2.1_02112025/code", "bsimbulk.va"),
    (
        "bsimcmg_va",
        "cmc/BSIM-CMG_112.1.0_04282026/code",
        "bsimcmg.va",
    ),
    (
        "hisimhv_va",
        "cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va",
        "hisimhv.va",
    ),
];

#[test]
#[ignore = "compiles large models; run with --ignored"]
fn live_lane_sets_are_measured() {
    let root = model_root();
    for (module, directory, file) in MODELS {
        let path = directory
            .split('/')
            .fold(root.clone(), |path, part| path.join(part))
            .join(file);
        if !path.exists() {
            eprintln!("{module:>16}  fixture missing at {}", path.display());
            continue;
        }
        census(&root, &path, module);
    }
}

fn census(root: &Path, path: &Path, module: &str) {
    let mut options = CompilerOptions::default();
    options.include_paths.push(root.to_path_buf());
    if let Some(directory) = path.parent()
        && let Ok(candidates) = discover_veriloga_sources(directory)
        && let Some(candidate) = candidates.into_iter().find(|entry| entry.path == path)
    {
        options.defines = candidate.compile_profile.defines;
        options.undefines = candidate.compile_profile.undefines;
    }
    let artifact = match VerilogACompiler::new(options)
        .compile_file_canonical_ir_with_metadata(path, Some(module))
    {
        Ok(compiled) => compiled.artifact,
        Err(error) => {
            eprintln!("{module:>16}  front end: {error}");
            return;
        }
    };
    let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
        eprintln!("{module:>16}  lowering failed");
        return;
    };

    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain(
            (0..artifact.mir.branch_unknowns.len()).map(|index| AdSeed::BranchUnknownFlow(index.into())),
        )
        .collect();
    let live = lane_liveness(&cfg.function, &lanes);

    let widths: Vec<usize> = live
        .iter()
        .map(std::collections::HashSet::len)
        .filter(|width| *width > 0)
        .collect();
    let carrying = widths.len();
    let total = live.len();
    let sum: usize = widths.iter().sum();
    let widest = widths.iter().copied().max().unwrap_or(0);
    let mean = if carrying == 0 {
        0.0
    } else {
        sum as f64 / carrying as f64
    };

    // How many distinct sets there are decides whether a small lattice of
    // canonical widths could keep the sparsity that a single dense `L` throws
    // away.
    let mut shapes: HashMap<Vec<usize>, usize> = HashMap::new();
    for set in &live {
        if set.is_empty() {
            continue;
        }
        let mut key: Vec<usize> = set.iter().copied().collect();
        key.sort_unstable();
        *shapes.entry(key).or_default() += 1;
    }
    let mut ranked: Vec<(Vec<usize>, usize)> = shapes.into_iter().collect();
    ranked.sort_by_key(|(key, count)| (std::cmp::Reverse(*count), key.clone()));
    let distinct = ranked.len();
    let top: usize = ranked.iter().take(4).map(|(_, count)| *count).sum();

    eprintln!(
        "{module:>16}  {:>2} lanes  {carrying:>7}/{total:<7} carry  mean {mean:>5.2}  \
         widest {widest:>2}  {distinct:>5} shapes  top4 {:>5.1}%  sum {sum:>8}  dense {:>8}",
        lanes.len(),
        100.0 * top as f64 / carrying as f64,
        carrying * lanes.len(),
    );
    for (key, count) in ranked.iter().take(4) {
        eprintln!("{:>18}x{count:<7} {key:?}", "");
    }
}

fn model_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    assert!(root.exists(), "model tree missing: {}", root.display());
    root
}
