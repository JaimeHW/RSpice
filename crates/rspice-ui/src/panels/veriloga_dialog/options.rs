use std::path::PathBuf;

/// Compiler options configuration.
#[derive(Debug, Clone, Default)]
pub struct VerilogADialogOptions {
    /// Include paths for `include directives.
    pub include_paths: Vec<PathBuf>,
    /// Input field for new include path.
    pub new_include_path: String,
    /// Preprocessor defines (name, value).
    pub defines: Vec<(String, String)>,
    /// Input field for new define name.
    pub new_define_name: String,
    /// Input field for new define value.
    pub new_define_value: String,
    /// Enable strict LRM compliance.
    pub strict_mode: bool,
    /// Enable Verilog-AMS mixed-signal support.
    pub enable_ams: bool,
}

impl VerilogADialogOptions {
    /// Add an include path.
    pub fn add_include_path(&mut self, path: PathBuf) {
        if !path.as_os_str().is_empty() && !self.include_paths.contains(&path) {
            self.include_paths.push(path);
        }
    }

    /// Remove an include path by index.
    pub fn remove_include_path(&mut self, index: usize) {
        if index < self.include_paths.len() {
            self.include_paths.remove(index);
        }
    }

    /// Add a preprocessor define.
    pub fn add_define(&mut self, name: String, value: String) {
        if !name.is_empty() {
            self.defines.retain(|(n, _)| n != &name);
            self.defines.push((name, value));
        }
    }

    /// Remove a define by index.
    pub fn remove_define(&mut self, index: usize) {
        if index < self.defines.len() {
            self.defines.remove(index);
        }
    }

    /// Convert to rspice-veriloga CompilerOptions.
    #[cfg(feature = "veriloga")]
    pub fn to_compiler_options(&self) -> rspice_veriloga::CompilerOptions {
        rspice_veriloga::CompilerOptions {
            enable_ams: self.enable_ams,
            include_paths: self.include_paths.clone(),
            defines: self
                .defines
                .iter()
                .map(|(n, v)| (n.clone(), Some(v.clone())))
                .collect(),
            strict_mode: self.strict_mode,
            ..Default::default()
        }
    }
}
