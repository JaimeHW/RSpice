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
        let abi = if cfg!(windows) {
            SmolStr::new("windows")
        } else if cfg!(unix) {
            SmolStr::new("system-v")
        } else {
            SmolStr::new("unknown")
        };

        Some(Self { arch, os, abi })
    }

    pub fn display_name(&self) -> String {
        format!("{:?}-{}-{}", self.arch, self.os, self.abi)
    }
}
