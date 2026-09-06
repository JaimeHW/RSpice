//! Release qualification of the shipped compact-model census through whichever
//! machine backend the host provides.
//!
//! Both backends compile from the same canonical plan, but they encode,
//! allocate, and verify independently, so a census that runs on only one of
//! them qualifies only that one.
#![cfg(all(
    feature = "native",
    any(target_arch = "aarch64", target_arch = "x86_64"),
    any(target_os = "macos", target_os = "linux", windows)
))]

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::native::SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

#[test]
#[ignore = "release qualification for the shipped device census; run with --release --features native -- --ignored --nocapture"]
fn shipped_models_compile_and_execute_through_the_public_native_jit() {
    let cases = [
        (
            "juncap200",
            shipped_model_path(&["cmc", "PSP104.1.0_vacode", "vacode", "juncap200.va"]),
            None,
        ),
        (
            "ekv26",
            shipped_model_path(&["ekv26_2.6", "ekv26_SDext_Verilog-A.va"]),
            Some("ekv_va"),
        ),
        (
            "r3_cmc",
            shipped_model_path(&["cmc", "r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"]),
            None,
        ),
        (
            "diode_cmc",
            shipped_model_path(&["cmc", "diode_cmc_3.0_20250714", "vacode", "diode_cmc.va"]),
            Some("DIODE_CMC"),
        ),
        (
            "vbic13_4t",
            shipped_model_path(&["vbic_1.3", "vacode", "vbic_1p3.va"]),
            Some("vbic13_4t"),
        ),
        (
            "bsimbulk",
            shipped_model_path(&["cmc", "BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"]),
            Some("bsimbulk"),
        ),
        (
            "bsimcmg",
            shipped_model_path(&["cmc", "BSIM-CMG_112.1.0_04282026", "code", "bsimcmg.va"]),
            Some("bsimcmg_va"),
        ),
        (
            "psp104",
            shipped_model_path(&["cmc", "PSP104.1.0_vacode", "vacode", "psp104.va"]),
            Some("PSP104VA"),
        ),
        (
            "bsimimg",
            shipped_model_path(&["cmc", "BSIM-IMG_103.0.0_20200102", "code", "bsimimg.va"]),
            Some("bsimimg"),
        ),
        (
            "psp104_nqs",
            shipped_model_path(&["cmc", "PSP104.1.0_vacode", "vacode", "psp104_nqs.va"]),
            Some("PSPNQS104VA"),
        ),
        (
            "hicuml0",
            shipped_model_path(&["cmc", "hicumL0_v2p1p0_files", "hicumL0_v2p1p0.va"]),
            Some("hicumL0va"),
        ),
        (
            "hicuml2",
            shipped_model_path(&["cmc", "hicumL2_v320_files", "hicumL2_v320.va"]),
            Some("hicumL2va"),
        ),
        (
            "bsimsoi47",
            shipped_model_path(&["cmc", "BSIM-SOI_4.7.0_05192025", "code", "bsimsoi.va"]),
            Some("bsimsoi"),
        ),
        (
            "bsimsoi461",
            shipped_model_path(&["bsimsoi_4.6.1", "vacode", "bsimsoi.va"]),
            Some("bsimsoi_va"),
        ),
        (
            "bsimsoi100",
            shipped_model_path(&["cmc", "BSIM_SOI_100.1.1_09152025", "code", "bsimsoi.va"]),
            Some("bsimsoi"),
        ),
        (
            "l_utsoi102",
            shipped_model_path(&[
                "cmc",
                "L_UTSOI_102.9.0_code_package",
                "vacode",
                "L_UTSOI_102.va",
            ]),
            Some("l_utsoi"),
        ),
        (
            "hisimhv",
            shipped_model_path(&[
                "cmc",
                "HiSIM_HV_2.5.1_Release_20230209",
                "HiSIM_HV_2.5.1_VA-Code",
                "hisimhv_va",
                "hisimhv.va",
            ]),
            Some("hisimhv_va"),
        ),
        (
            "hisimsoi",
            shipped_model_path(&[
                "cmc",
                "HiSIM_SOI_1.5.0_Release_20211008",
                "HiSIM_SOI_1.5.0_VA-Code",
                "hisimsoi_va",
                "hisimsoi.va",
            ]),
            Some("hisimsoi_va"),
        ),
        (
            "asmhemt",
            shipped_model_path(&["cmc", "ASM-HEMT101.6.0_05132026", "vacode", "asmhemt.va"]),
            Some("asmhemt"),
        ),
    ];

    for (name, path, module) in cases {
        if shipped_model_filter_allows(name) {
            qualify_shipped_model(name, &path, module);
        }
    }
}

fn qualify_shipped_model(name: &str, path: &Path, module: Option<&str>) {
    let frontend_started = Instant::now();
    let runtime = VerilogACompiler::new(CompilerOptions::default())
        .compile_file_runtime_with_metadata(path, module)
        .unwrap_or_else(|error| panic!("{name}: compile {}: {error}", path.display()));
    let frontend_elapsed = frontend_started.elapsed();

    let model = Arc::new(runtime.model);
    let terminal_biases = (0..model.num_terminals)
        .map(|terminal| terminal_bias(name, terminal))
        .collect::<Vec<_>>();
    let terminal_nodes = (1..=model.num_terminals).collect::<Vec<_>>();
    let native_started = Instant::now();
    let mut device = VerilogADevice::try_new_with_canonical_ir(
        format!("{name}_native_qualification"),
        Arc::clone(&model),
        &runtime.canonical_ir,
        &terminal_nodes,
    )
    .unwrap_or_else(|error| panic!("{name}: construct native device: {error}"));
    let native_elapsed = native_started.elapsed();
    assert!(device.is_using_native(), "{name}: interpreter fallback");
    assert!(
        device.native_code_size_bytes() <= SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES,
        "{name}: native image is {} bytes, exceeding the shipped-model budget of {} bytes",
        device.native_code_size_bytes(),
        SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES,
    );
    let plan_stats = device.native_plan_stats();
    assert_eq!(plan_stats.evaluation_kernel_entry_points, 1);
    assert_eq!(plan_stats.stamp_kernel_entry_points, 1);
    if matches!(name, "hicuml0" | "hicuml2") {
        device
            .try_set_analysis_type(2)
            .unwrap_or_else(|error| panic!("{name}: set transient analysis: {error}"));
    }

    let terminal_count = model.num_terminals;
    let internal_count = device.num_internal_nodes();
    let branch_count = device.num_branch_unknowns();
    let internal_indices =
        ((terminal_count + 1)..=(terminal_count + internal_count)).collect::<Vec<_>>();
    device.set_internal_node_indices(&internal_indices);
    let branch_indices = ((terminal_count + internal_count + 1)
        ..=(terminal_count + internal_count + branch_count))
        .collect::<Vec<_>>();
    device
        .try_set_branch_current_indices(&branch_indices)
        .unwrap_or_else(|error| panic!("{name}: set branch-current indices: {error}"));

    let mut solution = vec![0.0_f64; (terminal_count + internal_count + branch_count).max(1)];
    solution[..terminal_count].copy_from_slice(&terminal_biases);
    let canonical_internal_nodes = runtime
        .canonical_ir
        .mir
        .nodes
        .iter()
        .filter(|node| !node.is_external)
        .collect::<Vec<_>>();
    assert_eq!(canonical_internal_nodes.len(), internal_count);
    for (ordinal, node) in canonical_internal_nodes.into_iter().enumerate() {
        if let Some(value) = internal_bias(name, node.name.as_str(), &terminal_biases) {
            solution[terminal_count + ordinal] = value;
        }
    }
    device
        .try_update_all_voltages(&solution)
        .unwrap_or_else(|error| panic!("{name}: update voltages: {error}"));
    device
        .try_set_analysis_step(true, false)
        .unwrap_or_else(|error| panic!("{name}: enter initial step: {error}"));
    device
        .try_evaluate()
        .unwrap_or_else(|error| panic!("{name}: initial evaluation: {error}"));
    // Accept the initial step before leaving it, as the engine does
    // (`rspice-core/src/device/veriloga_builtins.rs` initial-step protocol):
    // every evaluation begins by restoring the *accepted* event-controlled
    // variables, so an unaccepted `@(initial_step)` block leaves them at the
    // zeros they were allocated with. VBIC's `tiniK` is one, and `rT =
    // tdevK / tiniK` then poisons the whole body.
    device
        .try_advance_state()
        .unwrap_or_else(|error| panic!("{name}: accept initial step: {error}"));
    device
        .try_set_analysis_step(false, false)
        .unwrap_or_else(|error| panic!("{name}: leave initial step: {error}"));

    let currents = device
        .try_evaluate()
        .unwrap_or_else(|error| panic!("{name}: native evaluation: {error}"));
    assert_eq!(currents.len(), model.stamp_programs.len());
    let finite_currents = currents.iter().filter(|value| value.is_finite()).count();
    assert!(finite_currents > 0, "{name}: no finite native currents");

    let mut matrix_entries = 0_usize;
    let mut rhs_entries = 0_usize;
    device
        .try_stamp(
            &solution,
            |row, column, value| {
                assert!(
                    value.is_finite(),
                    "{name}: non-finite matrix entry ({row}, {column})"
                );
                matrix_entries += 1;
            },
            |row, value| {
                assert!(value.is_finite(), "{name}: non-finite RHS entry {row}");
                rhs_entries += 1;
            },
        )
        .unwrap_or_else(|error| panic!("{name}: native stamp: {error}"));
    assert!(matrix_entries > 0, "{name}: no matrix entries");
    assert!(rhs_entries > 0, "{name}: no RHS entries");

    let mut reactive_entries = 0_usize;
    device
        .try_stamp_reactive(&solution, |row, column, value| {
            assert!(
                value.is_finite(),
                "{name}: non-finite reactive entry ({row}, {column})"
            );
            reactive_entries += 1;
        })
        .unwrap_or_else(|error| panic!("{name}: native reactive stamp: {error}"));
    assert!(reactive_entries > 0, "{name}: no reactive entries");

    eprintln!(
        "native-shipped model={name} frontend_ms={:.3} native_ms={:.3} code_bytes={} code_chunks={} currents={} matrix_entries={matrix_entries} rhs_entries={rhs_entries} reactive_entries={reactive_entries}",
        frontend_elapsed.as_secs_f64() * 1_000.0,
        native_elapsed.as_secs_f64() * 1_000.0,
        device.native_code_size_bytes(),
        device.native_chunk_count(),
        finite_currents,
    );
}

fn shipped_model_filter_allows(name: &str) -> bool {
    let Ok(filter) = std::env::var("RSPICE_NATIVE_SHIPPED_MODEL_FILTER") else {
        return true;
    };
    filter
        .split(',')
        .map(str::trim)
        .any(|candidate| candidate.eq_ignore_ascii_case(name))
}

fn terminal_bias(name: &str, terminal: usize) -> f64 {
    let values: &[f64] = match name {
        "juncap200" => &[0.2, 0.0],
        "ekv26" => &[0.8, 0.7, 0.0, 0.0],
        "r3_cmc" => &[0.1, 0.0, 0.0, 0.0],
        "diode_cmc" => &[0.7, 0.0],
        "vbic13_4t" => &[0.2, 0.75, 0.0, 0.0],
        "bsimbulk" | "bsimcmg" => &[0.05, 0.7, 0.0, 0.0, 0.0],
        "psp104" | "psp104_nqs" | "hisimsoi" => &[0.05, 0.7, 0.0, 0.0],
        "bsimimg" => &[0.15, 0.7, 0.05, -0.05, 0.01],
        "hicuml0" | "hicuml2" => &[0.2, 0.8, 0.0, 0.0, 0.0],
        "bsimsoi47" => &[0.05, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0],
        "bsimsoi100" => &[0.05, 0.7, 0.0, 0.0, 0.0, 0.0],
        "asmhemt" => &[0.1, 0.3, 0.0, 0.0, 0.0],
        "bsimsoi461" | "l_utsoi102" | "hisimhv" => &[],
        _ => &[],
    };
    values.get(terminal).copied().unwrap_or(0.0)
}

fn internal_bias(name: &str, node: &str, terminals: &[f64]) -> Option<f64> {
    match (name, node.to_ascii_lowercase().as_str()) {
        ("bsimcmg", "di" | "di1" | "di2") => Some(terminals[0]),
        ("bsimcmg", "si" | "si1") => Some(terminals[2]),
        ("bsimcmg", "ge" | "gi" | "gint" | "gints" | "gintd") => Some(terminals[1]),
        ("bsimimg", "di") => Some(terminals[0]),
        ("bsimimg", "si") => Some(terminals[2]),
        ("bsimimg", "ge" | "gi") => Some(terminals[1]),
        ("vbic13_4t", "cx" | "ci") => Some(terminals[0]),
        ("vbic13_4t", "bx" | "bi" | "bp") => Some(terminals[1]),
        ("vbic13_4t", "ei") => Some(terminals[2]),
        ("vbic13_4t", "si") => Some(terminals[3]),
        _ => None,
    }
}

fn shipped_model_path(parts: &[&str]) -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    for part in parts {
        path.push(part);
    }
    assert!(path.is_file(), "missing shipped model: {}", path.display());
    path
}
