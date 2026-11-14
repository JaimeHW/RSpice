//! Every shipped Verilog-A model, brought into a project as its own
//! Verilog-A, through the browser JIT the worker compiles project Verilog-A
//! with.
//!
//! A user who imports one of these sources (or one its size) as project
//! Verilog-A gets exactly this route: the source closure compiles in the
//! worker's compile request, the JIT emits the model module, and the browser
//! compiles and instantiates it. Each source's outcome is pinned, so a change
//! that moves one says so.
//!
//! Node only: the sources are read from the checkout through Node's `fs`.
//! `cargo test -p rspice-worker --target wasm32-unknown-unknown --release
//! --test shipped_source_jit` with `wasm-bindgen-test-runner` as the runner.

#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rspice_design::project_sources::{
    ProjectSourceBundle, ProjectSourceDependency, ProjectSourceFile, ProjectSourceLanguage,
    ProjectSourceOwner,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file_sync(path: &str, encoding: &str) -> Result<String, JsValue>;
}

/// The revision of the worker's compile request (`compile_service/worker.rs`).
const COMPILE_PROTOCOL: u32 = 4;

const MODEL_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../models/veriloga");

/// What the browser does with one source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    /// The module is emitted, compiled and instantiated by the engine.
    Emit,
    /// The compile request or the JIT refuses the source, or the engine
    /// refuses the module. The detail says which.
    Refuse,
    /// The worker traps compiling it. Such a source is not walked, since a
    /// trap leaves the module's stack and heap unusable for the next one.
    Trap,
}

use Outcome::{Emit, Refuse, Trap};

/// Every shipped model: what the Library calls it, its module, its root file
/// relative to `models/veriloga`, and the outcome it has today.
const SOURCES: &[(&str, &str, &str, Outcome)] = &[
    (
        "BSIM-BULK",
        "bsimbulk",
        "cmc/BSIM-BULK107.2.1_02112025/code/bsimbulk.va",
        Refuse,
    ),
    (
        "PSP",
        "PSP104VA",
        "cmc/PSP104.1.0_vacode/vacode/psp104.va",
        Refuse,
    ),
    (
        "PSP NQS",
        "PSPNQS104VA",
        "cmc/PSP104.1.0_vacode/vacode/psp104_nqs.va",
        Refuse,
    ),
    (
        "PSP self-heating",
        "PSP104TVA",
        "cmc/PSP104.1.0_vacode/vacode/psp104t.va",
        Refuse,
    ),
    (
        "PSP 103",
        "PSP103VA",
        "cmc/PSP103p4_vacode/vacode/psp103.va",
        Emit,
    ),
    (
        "PSP 103 NQS",
        "PSPNQS103VA",
        "cmc/PSP103p4_vacode/vacode/psp103_nqs.va",
        Refuse,
    ),
    (
        "PSP 103 self-heating",
        "PSP103TVA",
        "cmc/PSP103p4_vacode/vacode/psp103t.va",
        Emit,
    ),
    ("PSP 102", "PSP102VA", "cmc/vacode_102.5.0/psp102.va", Emit),
    (
        "PSP 102 NQS",
        "PSPNQS102VA",
        "cmc/vacode_102.5.0/psp102_nqs.va",
        Refuse,
    ),
    (
        "PSP 102 binned",
        "PSP102BVA",
        "cmc/vacode_102.5.0/psp102b.va",
        Emit,
    ),
    (
        "PSP 102 binned NQS",
        "PSPNQS102BVA",
        "cmc/vacode_102.5.0/psp102b_nqs.va",
        Refuse,
    ),
    (
        "PSP 102 local",
        "PSP102EVA",
        "cmc/vacode_102.5.0/psp102e.va",
        Emit,
    ),
    (
        "PSP 102 local NQS",
        "PSPNQS102EVA",
        "cmc/vacode_102.5.0/psp102e_nqs.va",
        Refuse,
    ),
    (
        "EKV 2.6",
        "ekv_va",
        "ekv26_2.6/ekv26_SDext_Verilog-A.va",
        Emit,
    ),
    ("EKV3 RF", "ekv3_rf", "ekv3_302.00/vacode/ekv3.va", Emit),
    (
        "BSIM-CMG",
        "bsimcmg_va",
        "cmc/BSIM-CMG_112.1.0_04282026/code/bsimcmg.va",
        Refuse,
    ),
    (
        "BSIM-CMG 111",
        "bsimcmg_va",
        "cmc/BSIM-CMG_111.2.1_06062022/code/bsimcmg.va",
        Refuse,
    ),
    (
        "BSIM-CMG 110",
        "bsimcmg",
        "cmc/BSIMCMG110.0.0_20160101/code/bsimcmg.va",
        Emit,
    ),
    (
        "BSIM-CMG 108",
        "bsimcmg",
        "cmc/BSIMCMG108.0.0_20140822/code/bsimcmg.va",
        Refuse,
    ),
    (
        "BSIM-CMG 107",
        "bsimcmg",
        "cmc/BSIMCMG107.0.0_20130712/code/bsimcmg.va",
        Emit,
    ),
    (
        "BSIM-IMG",
        "bsimimg",
        "cmc/BSIM-IMG_103.0.0_20200102/code/bsimimg.va",
        Emit,
    ),
    (
        "BSIM-SOI 4.6",
        "bsimsoi_va",
        "bsimsoi_4.6.1/vacode/bsimsoi.va",
        Refuse,
    ),
    (
        "BSIM-SOI 4.7",
        "bsimsoi",
        "cmc/BSIM-SOI_4.7.0_05192025/code/bsimsoi.va",
        Refuse,
    ),
    (
        "BSIM-SOI 100",
        "bsimsoi",
        "cmc/BSIM_SOI_100.1.1_09152025/code/bsimsoi.va",
        Refuse,
    ),
    (
        "HiSIM-SOI",
        "hisimsoi_va",
        "cmc/HiSIM_SOI_1.5.0_Release_20211008/HiSIM_SOI_1.5.0_VA-Code/hisimsoi_va/hisimsoi.va",
        Refuse,
    ),
    (
        "HiSIM-SOI n4",
        "hisimsoi_va",
        "cmc/HiSIM_SOI_1.5.0_Release_20211008/HiSIM_SOI_1.5.0_VA-Code/hisimsoi_va/hisimsoi_n4.va",
        Refuse,
    ),
    (
        "HiSIM-SOI n5",
        "hisimsoi_va",
        "cmc/HiSIM_SOI_1.5.0_Release_20211008/HiSIM_SOI_1.5.0_VA-Code/hisimsoi_va/hisimsoi_n5.va",
        Refuse,
    ),
    (
        "HiSIM-SOTB",
        "hisimsotb_va",
        "cmc/HiSIM_SOTB_1.3.0_Release_20211116/HiSIM_SOTB_1.3.0_VA-Code/hisimsotb_va/hisimsotb.va",
        Refuse,
    ),
    (
        "L-UTSOI",
        "l_utsoi",
        "cmc/L_UTSOI_102.9.0_code_package/vacode/L_UTSOI_102.va",
        Refuse,
    ),
    (
        "L-UTSOI NQS",
        "l_utsoi",
        "cmc/L_UTSOI_102.9.0_code_package/vacode/L_UTSOI_102_nqs.va",
        Refuse,
    ),
    (
        "HiSIM-HV",
        "hisimhv_va",
        "cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va/hisimhv.va",
        Trap,
    ),
    (
        "HiSIM-HV n4",
        "hisimhv_n4_va",
        "cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va/hisimhv_n4.va",
        Trap,
    ),
    (
        "HiSIM-HV n5",
        "hisimhv_n5_va",
        "cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va/hisimhv_n5.va",
        Trap,
    ),
    (
        "Mextram 504",
        "bjt504va",
        "cmc/vacode_504.12.1/bjt504.va",
        Emit,
    ),
    (
        "Mextram 504 self-heating",
        "bjt504tva",
        "cmc/vacode_504.12.1/bjt504t.va",
        Emit,
    ),
    (
        "Mextram 504 discrete",
        "bjtd504va",
        "cmc/vacode_504.12.1/bjtd504.va",
        Emit,
    ),
    (
        "Mextram 504 discrete self-heating",
        "bjtd504tva",
        "cmc/vacode_504.12.1/bjtd504t.va",
        Emit,
    ),
    ("Mextram 505", "bjt505_va", "cmc/505p5p0_va/bjt505.va", Emit),
    (
        "Mextram 505 self-heating",
        "bjt505t_va",
        "cmc/505p5p0_va/bjt505t.va",
        Emit,
    ),
    (
        "Mextram 505 discrete",
        "bjtd505_va",
        "cmc/505p5p0_va/bjtd505.va",
        Emit,
    ),
    (
        "Mextram 505 discrete self-heating",
        "bjtd505t_va",
        "cmc/505p5p0_va/bjtd505t.va",
        Emit,
    ),
    (
        "HICUM L0",
        "hicumL0va",
        "cmc/hicumL0_v2p1p0_files/hicumL0_v2p1p0.va",
        Emit,
    ),
    (
        "HICUM L2",
        "hicumL2va",
        "cmc/hicumL2_v320_files/hicumL2_v320.va",
        Emit,
    ),
    (
        "VBIC 3-terminal",
        "vbic13",
        "vbic_1.3/vacode/vbic_3T.va",
        Emit,
    ),
    (
        "VBIC 3-terminal self-heating",
        "vbic13_3t_et",
        "vbic_1.3/vacode/vbic_3T_et.va",
        Emit,
    ),
    (
        "VBIC 4-terminal",
        "vbic13_4t",
        "vbic_1.3/vacode/vbic_1p3.va",
        Emit,
    ),
    (
        "VBIC 4-terminal self-heating",
        "vbic_4T_et_cf",
        "vbic_1.3/vacode/vbic_4T_et_cf.va",
        Emit,
    ),
    (
        "ASM-HEMT",
        "asmhemt",
        "cmc/ASM-HEMT101.6.0_05132026/vacode/asmhemt.va",
        Refuse,
    ),
    (
        "MVSG",
        "mvsg_cmc",
        "cmc/mvsg_cmc_v4.0.0_official/vacode/mvsg_cmc_4.0.0.va",
        Emit,
    ),
    (
        "EPFL-HEMT",
        "EPFL_HEMT_10a",
        "epfl_hemt_3.0.0/vacode/epfl_hemt.va",
        Emit,
    ),
    ("Angelov", "angelov", "angelov_2.0/vacode/angelov.va", Emit),
    (
        "Angelov GaN",
        "angelov_gan",
        "angelov_2.0/vacode/angelov_gan.va",
        Emit,
    ),
    (
        "CMC diode",
        "DIODE_CMC",
        "cmc/diode_cmc_3.0_20250714/vacode/diode_cmc.va",
        Emit,
    ),
    (
        "JUNCAP 200",
        "JUNCAP200",
        "cmc/PSP104.1.0_vacode/vacode/juncap200.va",
        Emit,
    ),
    (
        "ASM-ESD",
        "asmesd",
        "cmc/ASM-ESD101.1.0_04042025/asmesd.va",
        Emit,
    ),
    (
        "ASM-ESD diode",
        "asmesd_dio",
        "cmc/ASM-ESD101.1.0_04042025/asmesd_dio.va",
        Emit,
    ),
    ("R2", "r2_cmc", "cmc/r2_cmc_v1.0.2/r2_cmc.va", Emit),
    (
        "R2 self-heating",
        "r2_et_cmc",
        "cmc/r2_cmc_v1.0.2/r2_et_cmc.va",
        Emit,
    ),
    (
        "R3",
        "r3_cmc",
        "cmc/r3_cmc_release1.1.2_2023Jun16/r3_cmc.va",
        Emit,
    ),
    ("MOSVAR", "mosvar", "cmc/MOSVAR140/mosvar.va", Emit),
];

fn read_shipped(relative: &str) -> Option<String> {
    read_file_sync(&format!("{MODEL_ROOT}/{relative}"), "utf8").ok()
}

/// A shipped file as the tree ships it: with its release patch applied.
fn shipped_file(relative: &str) -> Option<String> {
    let text = read_shipped(relative)?;
    let patched =
        rspice_veriloga::patch_shipped_source(relative, &text, |patch| Ok(read_shipped(patch)))
            .unwrap_or_else(|error| panic!("{relative}: {error}"));
    Some(patched.into_owned())
}

/// `requested` beside `parent`, `.` and `..` folded, or `None` above the root.
fn joined(parent: Option<&str>, requested: &str) -> Option<String> {
    let mut kept: Vec<&str> = parent
        .map(|parent| parent.split('/').collect())
        .unwrap_or_default();
    for segment in requested.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                kept.pop()?;
            }
            other => kept.push(other),
        }
    }
    Some(kept.join("/"))
}

/// The files a root's includes reach, read from the root's folder, as an
/// import of the root into a project reads them: an include resolves beside
/// the file that writes it, then from the root's folder.
struct ImportClosure {
    folder: String,
    files: RefCell<BTreeMap<String, String>>,
    edges: RefCell<Vec<(String, String)>>,
}

impl ImportClosure {
    fn file(&self, logical: &str) -> Option<String> {
        if let Some(text) = self.files.borrow().get(logical) {
            return Some(text.clone());
        }
        let text = shipped_file(&format!("{}/{logical}", self.folder))?;
        self.files
            .borrow_mut()
            .insert(logical.to_owned(), text.clone());
        Some(text)
    }
}

fn logical(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

impl rspice_veriloga::SourceProvider for ImportClosure {
    fn load_root(
        &self,
        requested: &Path,
    ) -> Result<rspice_veriloga::SourceDocument, rspice_veriloga::PreprocessorError> {
        let name = logical(requested);
        self.file(&name)
            .map(|text| rspice_veriloga::SourceDocument::provided(name.clone(), text))
            .ok_or_else(|| {
                rspice_veriloga::PreprocessorError::new(
                    format!("{name} is not in the model tree"),
                    Some(requested.to_path_buf()),
                    0,
                )
            })
    }

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        _include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<rspice_veriloga::SourceDocument>, rspice_veriloga::PreprocessorError> {
        let including = including_file.map(logical).unwrap_or_default();
        let parent = including.rsplit_once('/').map(|(parent, _)| parent);
        for candidate in [joined(parent, requested), joined(None, requested)]
            .into_iter()
            .flatten()
        {
            if let Some(text) = self.file(&candidate) {
                let edge = (including.clone(), candidate.clone());
                if !self.edges.borrow().contains(&edge) {
                    self.edges.borrow_mut().push(edge);
                }
                return Ok(Some(rspice_veriloga::SourceDocument::provided(
                    candidate, text,
                )));
            }
        }
        if rspice_veriloga::stdlib::builtin_include_document(requested).is_some() {
            return Ok(None);
        }
        Err(rspice_veriloga::PreprocessorError::new(
            format!("{requested} (included by {including}) is not in the model tree"),
            including_file.map(Path::to_path_buf),
            0,
        ))
    }
}

/// `root` and every file its includes reach, as the project bundle an
/// import of it makes.
fn import_bundle(root: &str) -> ProjectSourceBundle {
    let (folder, file) = root.rsplit_once('/').unwrap_or(("", root));
    let closure = ImportClosure {
        folder: folder.to_owned(),
        files: RefCell::new(BTreeMap::new()),
        edges: RefCell::new(Vec::new()),
    };
    rspice_veriloga::Preprocessor::new()
        .preprocess_provider_root(&closure, Path::new(file))
        .unwrap_or_else(|error| panic!("{root}: {error}"));
    let mut files = closure.files.into_inner();
    let root_text = files.remove(file).expect("the root was read");
    let edges = closure
        .edges
        .into_inner()
        .into_iter()
        .filter(|(_, imported)| imported != file)
        .map(|(importer, imported)| ProjectSourceDependency::try_new(importer, imported).unwrap());
    ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        file,
        root_text,
        files
            .into_iter()
            .map(|(path, text)| ProjectSourceFile::try_new(path, text).unwrap()),
        edges,
    )
    .unwrap_or_else(|error| panic!("{root}: {error}"))
}

fn property(object: &JsValue, key: &str) -> JsValue {
    js_sys::Reflect::get(object, &key.into()).unwrap()
}

// The raw capability exports a JIT-compiled model module imports. The worker
// binary exports them (`src/main.rs`); this walk is a module of its own, so it
// exports the same four over the same library functions.

#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub extern "C" fn rspice_ui_wasm_jit_eval_op_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand0: f64,
    operand1: f64,
    operand2: f64,
    operand3: f64,
    operand4: f64,
) -> f64 {
    rspice_worker::rspice_ui_wasm_jit_eval_op_v1(
        frame_offset,
        opcode,
        aux0,
        aux1,
        aux2,
        operand0,
        operand1,
        operand2,
        operand3,
        operand4,
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn rspice_ui_wasm_jit_eval_op_slice_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand_count: i32,
) -> f64 {
    rspice_worker::rspice_ui_wasm_jit_eval_op_slice_v1(
        frame_offset,
        opcode,
        aux0,
        aux1,
        aux2,
        operand_count,
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn rspice_ui_wasm_jit_math1_v1(opcode: i32, value: f64) -> f64 {
    rspice_worker::rspice_ui_wasm_jit_math1_v1(opcode, value)
}

#[unsafe(no_mangle)]
pub extern "C" fn rspice_ui_wasm_jit_math2_v1(opcode: i32, left: f64, right: f64) -> f64 {
    rspice_worker::rspice_ui_wasm_jit_math2_v1(opcode, left, right)
}

/// The raw capability exports a model module imports, as
/// `simulation-worker.js` binds them.
fn capability_imports() -> js_sys::Object {
    let exports = wasm_bindgen::exports();
    let capabilities = js_sys::Object::new();
    for (import, export) in [
        ("memory", "memory"),
        ("eval_op_v1", "rspice_ui_wasm_jit_eval_op_v1"),
        ("eval_op_slice_v1", "rspice_ui_wasm_jit_eval_op_slice_v1"),
        ("math1_v1", "rspice_ui_wasm_jit_math1_v1"),
        ("math2_v1", "rspice_ui_wasm_jit_math2_v1"),
    ] {
        let value = property(&exports, export);
        assert!(!value.is_undefined(), "the test module exports {export}");
        js_sys::Reflect::set(&capabilities, &import.into(), &value).unwrap();
    }
    let imports = js_sys::Object::new();
    js_sys::Reflect::set(&imports, &"rspice_jit".into(), &capabilities).unwrap();
    imports
}

struct Walked {
    outcome: Outcome,
    detail: String,
    module_bytes: usize,
    compile_ms: f64,
    instantiate_ms: f64,
}

/// One source through the worker's compile request and, when the JIT emits
/// a module, through the engine's compile and instantiation.
fn walk(module: &str, root: &str, imports: &js_sys::Object) -> Walked {
    let bundle = import_bundle(root);
    let request = serde_json::json!({
        "protocol": COMPILE_PROTOCOL,
        "id": 1,
        "bundle": bundle,
        "selected_module": module,
    });
    let request = js_sys::JSON::parse(&request.to_string()).unwrap();
    let started = js_sys::Date::now();
    let response = rspice_worker::run_rspice_ui_veriloga_compile_request(request);
    let compile_ms = js_sys::Date::now() - started;
    let mut walked = Walked {
        outcome: Refuse,
        detail: String::new(),
        module_bytes: 0,
        compile_ms,
        instantiate_ms: 0.0,
    };
    let response = match response {
        Ok(response) => response,
        Err(error) => {
            walked.detail = format!("compile request: {}", error.as_string().unwrap_or_default());
            return walked;
        }
    };
    let artifact = property(&response, "wasmJitArtifact");
    if artifact.is_undefined() {
        let error = property(&response, "wasmJitError");
        walked.detail = match error.as_string() {
            Some(error) => format!("JIT: {error}"),
            None => format!(
                "front end: {}",
                js_sys::JSON::stringify(&property(&response, "outcome"))
                    .map(String::from)
                    .unwrap_or_default()
                    .chars()
                    .take(400)
                    .collect::<String>()
            ),
        };
        return walked;
    }
    let bytes = js_sys::Uint8Array::new(&property(&artifact, "moduleBytes"));
    walked.module_bytes = bytes.length() as usize;
    let started = js_sys::Date::now();
    let instantiated = js_sys::WebAssembly::Module::new(&bytes)
        .and_then(|compiled| js_sys::WebAssembly::Instance::new(&compiled, imports));
    walked.instantiate_ms = js_sys::Date::now() - started;
    match instantiated {
        Ok(_) => walked.outcome = Emit,
        Err(error) => {
            walked.detail = format!(
                "engine: {}",
                js_sys::JSON::stringify(&error)
                    .map(String::from)
                    .ok()
                    .filter(|text| text != "{}")
                    .or_else(|| error.as_string())
                    .unwrap_or_else(|| format!("{error:?}"))
            );
        }
    }
    walked
}

/// `JIT_WALK_ONLY`, a comma-separated list of names to walk alone, trapping
/// ones included: how a trap is looked at, one source per process.
fn walk_only() -> Option<Vec<String>> {
    let env = property(&property(&js_sys::global(), "process"), "env");
    property(&env, "JIT_WALK_ONLY")
        .as_string()
        .map(|names| names.split(',').map(str::to_owned).collect())
}

/// The shipped census is exactly the walked list.
fn shipped_device_count() -> usize {
    let manifest = read_file_sync(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../rspice-veriloga-models/manifest.txt"
        ),
        "utf8",
    )
    .expect("the generated catalog manifest is readable");
    let json = &manifest[manifest.find('{').expect("a JSON manifest")..];
    let manifest: serde_json::Value = serde_json::from_str(json).unwrap();
    manifest["device_count"].as_u64().unwrap() as usize
}

#[wasm_bindgen_test]
fn every_shipped_source_reaches_its_pinned_browser_jit_outcome() {
    assert_eq!(
        SOURCES.len(),
        shipped_device_count(),
        "the walk lists every shipped model"
    );
    let imports = capability_imports();
    let only = walk_only();
    let mut moved = Vec::new();
    for &(name, module, root, expected) in SOURCES {
        if let Some(only) = &only {
            if !only.iter().any(|walked| walked == name) {
                continue;
            }
            console_log!("JIT_WALK|{name}|walking");
        } else if expected == Trap {
            console_log!("JIT_WALK|{name}|trap|not walked");
            continue;
        }
        let walked = walk(module, root, &imports);
        let outcome = match walked.outcome {
            Emit => "emit",
            Refuse => "refuse",
            Trap => "trap",
        };
        console_log!(
            "JIT_WALK|{name}|{outcome}|{} bytes|compile {:.0} ms|instantiate {:.0} ms|{}",
            walked.module_bytes,
            walked.compile_ms,
            walked.instantiate_ms,
            walked.detail
        );
        if walked.outcome != expected {
            moved.push(format!(
                "{name}: {expected:?} -> {outcome} {}",
                walked.detail
            ));
        }
    }
    assert!(moved.is_empty(), "outcomes moved:\n{}", moved.join("\n"));
}
