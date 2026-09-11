#![allow(dead_code)]

use std::ffi::OsString;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompiledModel, CompilerOptions, VerilogACompiler};

pub const BSIM4_VA_ENV: &str = "RSPICE_BSIM4_VA";

#[derive(Debug, Clone)]
pub struct DeviceFixture {
    pub model: CompiledModel,
    /// Held in every configuration, not only the native one. A grouped-noise
    /// model refuses to construct without it — "no legacy fallback is
    /// permitted" — and every compiler writes `noise_process_schema = 1`, so
    /// the interpreter build needs the artifact for exactly the same reason
    /// the engine's own builder passes one.
    pub canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
}

impl DeviceFixture {
    pub fn compile(source: &str) -> Self {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        // Runtime compilation pairs both artifacts and renumbers state per
        // source site. Separately compiled legacy bytecode retains independent
        // derivative histories and cannot be paired with a canonical artifact.
        let report = compiler
            .compile_runtime(source, None)
            .expect("runtime compilation failed");
        Self {
            model: report.model,
            canonical_ir: report.canonical_ir,
        }
    }

    pub fn device(&self, name: &str, nodes: &[usize]) -> VerilogADevice {
        self.try_device(name, nodes)
            .expect("native device construction failed")
    }

    /// Publish a device's named variables so a test can read one back.
    ///
    /// An evaluation computes what the equations read; a name a test asserts on
    /// comes from the observation pass, and asks for it here.
    pub fn observe(&self, device: &mut VerilogADevice) {
        device
            .observe_variables(&self.canonical_ir)
            .expect("observation pass publishes the named variables");
    }

    pub fn try_device(
        &self,
        name: &str,
        nodes: &[usize],
    ) -> Result<VerilogADevice, rspice_veriloga::vm::VmError> {
        VerilogADevice::try_new_with_canonical_ir(
            name,
            self.model.clone(),
            &self.canonical_ir,
            nodes,
        )
    }

    /// The device the *bytecode* lowering produces from the same compilation.
    ///
    /// [`Self::try_device`] pairs the canonical artifact, so it is the CFG
    /// lowering; this one hands the backend only the `CompiledModel`, which is
    /// the lowering the interpreter executes. A route-parity test needs both
    /// out of one `compile_runtime`, because separately compiled artifacts
    /// renumber state and could not be compared step for step.
    ///
    /// Reachable only where the bytecode backend is: a non-`native` build, or
    /// `native-bytecode-contract-tests`. Elsewhere native construction refuses
    /// an absent artifact, and that refusal is what this returns.
    pub fn try_bytecode_device(
        &self,
        name: &str,
        nodes: &[usize],
    ) -> Result<VerilogADevice, rspice_veriloga::vm::VmError> {
        VerilogADevice::try_new(name, self.model.clone(), nodes)
    }
}

impl Deref for DeviceFixture {
    type Target = CompiledModel;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl DerefMut for DeviceFixture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

pub fn optional_bsim4_va_path(manifest_dir: &str) -> Option<PathBuf> {
    let fallback = Path::new(manifest_dir)
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .join("bsim4.va");
    optional_bsim4_va_path_from(std::env::var_os(BSIM4_VA_ENV), &fallback)
}

pub fn optional_bsim4_va_path_from(
    configured: Option<OsString>,
    fallback: &Path,
) -> Option<PathBuf> {
    if let Some(raw) = configured {
        let path = PathBuf::from(raw);
        assert!(
            path.is_file(),
            "{BSIM4_VA_ENV} must point at an externally supplied BSIM4 Verilog-A source file: {}",
            path.display()
        );
        return Some(path);
    }

    fallback.is_file().then(|| fallback.to_path_buf())
}
