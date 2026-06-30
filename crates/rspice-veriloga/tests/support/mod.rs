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
    #[cfg(feature = "native")]
    pub canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
}

impl DeviceFixture {
    pub fn compile(source: &str) -> Self {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compilation failed");
        #[cfg(feature = "native")]
        {
            let canonical_ir = compiler
                .compile_canonical_ir(source)
                .expect("canonical IR compilation failed");
            Self {
                model,
                canonical_ir,
            }
        }
        #[cfg(not(feature = "native"))]
        {
            Self { model }
        }
    }

    pub fn device(&self, name: &str, nodes: &[usize]) -> VerilogADevice {
        self.try_device(name, nodes)
            .expect("native device construction failed")
    }

    pub fn try_device(
        &self,
        name: &str,
        nodes: &[usize],
    ) -> Result<VerilogADevice, rspice_veriloga::vm::VmError> {
        #[cfg(feature = "native")]
        {
            VerilogADevice::try_new_with_canonical_ir(
                name,
                self.model.clone(),
                &self.canonical_ir,
                nodes,
            )
        }
        #[cfg(not(feature = "native"))]
        {
            VerilogADevice::try_new(name, self.model.clone(), nodes)
        }
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
