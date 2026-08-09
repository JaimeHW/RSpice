//! Which architecture the JIT is compiling for.
//!
//! [`TargetSpec::host`] resolves both the running architecture and its actual
//! platform ABI. Model dispatch remains fail-closed while a backend is under
//! qualification, but diagnostics must still distinguish Darwin AAPCS64,
//! generic ELF AAPCS64, Windows ARM64, System V x64, and Windows x64.

use smol_str::SmolStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X64,
    AArch64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetSpec {
    pub arch: Architecture,
    pub os: SmolStr,
    pub abi: SmolStr,
}

impl TargetSpec {
    pub fn host() -> Option<Self> {
        let arch = if cfg!(target_arch = "x86_64") {
            Architecture::X64
        } else if cfg!(target_arch = "aarch64") {
            Architecture::AArch64
        } else {
            return None;
        };

        let os = SmolStr::new(std::env::consts::OS);
        let abi = match arch {
            Architecture::X64 if cfg!(windows) => SmolStr::new("windows"),
            Architecture::X64 if cfg!(unix) => SmolStr::new("system-v"),
            Architecture::AArch64 if cfg!(windows) => SmolStr::new("windows-arm64"),
            Architecture::AArch64 if cfg!(target_os = "macos") => SmolStr::new("aapcs64-darwin"),
            Architecture::AArch64 if cfg!(unix) => SmolStr::new("aapcs64"),
            Architecture::X64 | Architecture::AArch64 => SmolStr::new("unknown"),
        };

        Some(Self { arch, os, abi })
    }

    pub fn display_name(&self) -> String {
        format!("{:?}-{}-{}", self.arch, self.os, self.abi)
    }
}

#[cfg(test)]
mod tests {
    use super::{Architecture, TargetSpec};

    #[test]
    fn host_names_the_real_architecture_abi() {
        let target = TargetSpec::host().expect("supported test host");
        #[cfg(target_arch = "aarch64")]
        assert_eq!(target.arch, Architecture::AArch64);
        #[cfg(target_arch = "x86_64")]
        assert_eq!(target.arch, Architecture::X64);

        #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
        assert_eq!(target.abi.as_str(), "aapcs64-darwin");
        #[cfg(all(target_arch = "aarch64", unix, not(target_os = "macos")))]
        assert_eq!(target.abi.as_str(), "aapcs64");
        #[cfg(all(target_arch = "aarch64", windows))]
        assert_eq!(target.abi.as_str(), "windows-arm64");
        #[cfg(all(target_arch = "x86_64", unix))]
        assert_eq!(target.abi.as_str(), "system-v");
        #[cfg(all(target_arch = "x86_64", windows))]
        assert_eq!(target.abi.as_str(), "windows");
    }
}
