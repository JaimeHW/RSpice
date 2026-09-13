//! Release qualification of the shipped compact-model census through whichever
//! machine backend the host provides.
//!
//! Both backends compile from the same canonical plan, but they encode,
//! allocate, and verify independently, so a census that runs on only one of
//! them qualifies only that one.
//!
//! `NATIVE_SHIPPED_CENSUS=` JSON records declare the corpus and selection,
//! bracket every model attempt, and summarize all results. A model failure
//! keeps the gate failing while allowing later rows to execute; a missing
//! summary means an incomplete census. The optional comma-separated
//! `RSPICE_NATIVE_SHIPPED_MODEL_FILTER` must resolve every requested name.
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

/// Release tests have no application logger. Preserve the compiler's route
/// refusals alongside each census row so an oversized image can be attributed
/// to the plan that actually produced it.
struct CompilerDiagnostics;

impl log::Log for CompilerDiagnostics {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Warn
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            eprintln!(
                "native-compiler target={} {}",
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

#[test]
#[ignore = "release qualification for the shipped device census; run with --release --features native -- --ignored --nocapture"]
fn shipped_models_compile_and_execute_through_the_public_native_jit() {
    log::set_logger(&CompilerDiagnostics).expect("one compiler logger per census process");
    log::set_max_level(log::LevelFilter::Warn);
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

    let filter = match std::env::var("RSPICE_NATIVE_SHIPPED_MODEL_FILTER") {
        Ok(filter) => Some(filter),
        Err(std::env::VarError::NotPresent) => None,
        Err(error) => panic!("invalid RSPICE_NATIVE_SHIPPED_MODEL_FILTER: {error}"),
    };
    let names = cases.iter().map(|(name, _, _)| *name).collect::<Vec<_>>();
    let selected = selected_shipped_indices(&names, filter.as_deref())
        .unwrap_or_else(|error| panic!("invalid RSPICE_NATIVE_SHIPPED_MODEL_FILTER: {error}"));
    census_record(serde_json::json!({
        "event": "inventory",
        "schema": "rspice-native-shipped-census",
        "schema_version": 1,
        "test": "shipped_models_compile_and_execute_through_the_public_native_jit",
        "architecture": std::env::consts::ARCH,
        "os": std::env::consts::OS,
        "required_feature": "native",
        "debug_assertions": cfg!(debug_assertions),
        "declared": names,
        "selected": selected.iter().map(|&index| names[index]).collect::<Vec<_>>(),
    }));
    let attempted = selected.len();
    let mut failures = Vec::new();
    for index in selected {
        let (name, path, module) = &cases[index];
        census_record(serde_json::json!({
            "event": "started", "model": name, "source": path, "module": module,
        }));
        // A failed row must not prevent the next model's qualification. Each
        // closure owns its compiler/device, so unwinding drops that row's state.
        // A process abort remains a failed, incomplete census (no summary).
        match std::panic::catch_unwind(|| qualify_shipped_model(name, path, *module)) {
            Ok(report) => census_record(serde_json::json!({
                "event": "finished", "model": name, "status": "passed", "report": report,
            })),
            Err(payload) => {
                let error = payload
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| payload.downcast_ref::<&str>().copied())
                    .unwrap_or("non-string qualification panic");
                census_record(serde_json::json!({
                    "event": "finished", "model": name, "status": "failed", "error": error,
                }));
                failures.push(*name);
            }
        }
    }
    census_record(serde_json::json!({
        "event": "summary", "attempted": attempted,
        "passed": attempted - failures.len(), "failed": failures.len(),
        "failed_models": failures,
    }));
    assert!(
        failures.is_empty(),
        "shipped models failed qualification: {}",
        failures.join(", ")
    );
}

fn census_record(record: serde_json::Value) {
    eprintln!("NATIVE_SHIPPED_CENSUS={record}");
}

fn qualify_shipped_model(name: &str, path: &Path, module: Option<&str>) -> serde_json::Value {
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
    assert_eq!(
        finite_currents,
        currents.len(),
        "{name}: non-finite native currents"
    );

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
    serde_json::json!({
        "frontend_ms": frontend_elapsed.as_secs_f64() * 1_000.0,
        "native_ms": native_elapsed.as_secs_f64() * 1_000.0,
        "code_bytes": device.native_code_size_bytes(),
        "code_chunks": device.native_chunk_count(),
        "currents": finite_currents,
        "matrix_entries": matrix_entries,
        "rhs_entries": rhs_entries,
        "reactive_entries": reactive_entries,
    })
}

/// Resolve the complete requested census before running a model. A misspelled
/// member of an otherwise valid list must not silently reduce qualification.
fn selected_shipped_indices(names: &[&str], filter: Option<&str>) -> Result<Vec<usize>, String> {
    let Some(filter) = filter else {
        return Ok((0..names.len()).collect());
    };
    let mut selected = Vec::new();
    for candidate in filter.split(',').map(str::trim) {
        if candidate.is_empty() {
            return Err("empty model name".into());
        }
        let index = names
            .iter()
            .position(|name| candidate.eq_ignore_ascii_case(name))
            .ok_or_else(|| {
                format!(
                    "unknown model '{candidate}'; expected one of {}",
                    names.join(", ")
                )
            })?;
        if selected.contains(&index) {
            return Err(format!("duplicate model '{}'", names[index]));
        }
        selected.push(index);
    }
    // A filter narrows the declared corpus without changing its execution order.
    selected.sort_unstable();
    Ok(selected)
}

#[test]
fn shipped_filter_preserves_the_declared_census_order() {
    let names = ["juncap200", "r3_cmc", "vbic13_4t"];
    assert_eq!(selected_shipped_indices(&names, None).unwrap(), [0, 1, 2]);
    assert_eq!(
        selected_shipped_indices(&names, Some(" VBIC13_4T , R3_CMC ")).unwrap(),
        [1, 2]
    );
}

#[test]
fn shipped_filter_rejects_empty_unknown_and_duplicate_requests() {
    let names = ["juncap200", "r3_cmc", "vbic13_4t"];
    for filter in ["", " ", ",", "r3_cmc,", ",r3_cmc"] {
        assert_eq!(
            selected_shipped_indices(&names, Some(filter)).unwrap_err(),
            "empty model name"
        );
    }
    for filter in ["missing", "r3_cmc,missing"] {
        let error = selected_shipped_indices(&names, Some(filter)).unwrap_err();
        assert!(error.contains("unknown model 'missing'"), "{error}");
    }
    assert_eq!(
        selected_shipped_indices(&names, Some("r3_cmc,R3_CMC")).unwrap_err(),
        "duplicate model 'r3_cmc'"
    );
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
