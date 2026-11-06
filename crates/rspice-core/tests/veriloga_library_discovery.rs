use rspice_core::library::discover_veriloga_models;
use std::collections::HashSet;
use std::path::Path;

#[test]
fn shipped_cmc_tree_discovers_veriloga_models_without_omi() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");

    let entries = discover_veriloga_models(&root).expect("discover shipped Verilog-A models");
    let packages: HashSet<&str> = entries.iter().map(|entry| entry.package.as_str()).collect();

    assert!(
        packages.contains("cmc/BSIM-CMG_112.1.0_04282026"),
        "BSIM-CMG package should be discoverable"
    );
    assert!(
        packages.contains("cmc/PSP104.1.0_vacode"),
        "PSP package should be discoverable"
    );
    assert!(
        packages.contains("cmc/hicumL2_v320_files"),
        "HICUM/L2 package should be discoverable"
    );
    let mextram504 = entries
        .iter()
        .filter(|entry| entry.package == "cmc/vacode_504.12.1")
        .flat_map(|entry| entry.modules.iter().map(String::as_str))
        .collect::<HashSet<_>>();
    assert_eq!(
        mextram504,
        HashSet::from(["bjt504va", "bjt504tva", "bjtd504va", "bjtd504tva"]),
        "the Mextram 504.12.1 package should discover its four modules"
    );
    assert!(
        packages
            .iter()
            .all(|package| !package.to_ascii_lowercase().contains("omi")),
        "OMI must not be part of the shipped Verilog-A discovery tree"
    );

    let bsim_cmg = entries
        .iter()
        .find(|entry| entry.source_path.ends_with("code/bsimcmg.va"))
        .expect("BSIM-CMG Verilog-A entry");
    assert!(
        bsim_cmg
            .modules
            .iter()
            .any(|module| module.eq_ignore_ascii_case("bsimcmg_va")),
        "BSIM-CMG module name should be extracted from source"
    );
    assert!(
        bsim_cmg
            .include_dirs
            .iter()
            .any(|dir| dir.ends_with("cmc/BSIM-CMG_112.1.0_04282026/code")),
        "BSIM-CMG include directory should be available"
    );

    assert!(
        entries.len() >= 20,
        "expected a broad shipped CMC Verilog-A corpus, got {} entries",
        entries.len()
    );
    assert!(
        entries.iter().all(|entry| !entry.modules.is_empty()),
        "discovered entries should be module-bearing .va files"
    );
}
