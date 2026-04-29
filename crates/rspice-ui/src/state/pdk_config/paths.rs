use std::path::PathBuf;

use super::*;

impl PdkConfig {
    // =========================================================================
    // Library Path Management
    // =========================================================================

    /// Add a library search path
    pub fn add_library_path(&mut self, path: impl Into<String>) {
        let entry = LibraryPathEntry::new(path);
        if !self.library_paths.iter().any(|e| e.path == entry.path) {
            self.library_paths.push(entry);
        }
    }

    /// Add a library path entry with full configuration
    pub fn add_library_path_entry(&mut self, entry: LibraryPathEntry) {
        if !self.library_paths.iter().any(|e| e.path == entry.path) {
            self.library_paths.push(entry);
        }
    }

    /// Remove a library path by index
    pub fn remove_library_path(&mut self, index: usize) -> Option<LibraryPathEntry> {
        if index < self.library_paths.len() {
            Some(self.library_paths.remove(index))
        } else {
            None
        }
    }

    /// Get enabled library paths
    pub fn enabled_paths(&self) -> impl Iterator<Item = &LibraryPathEntry> {
        self.library_paths.iter().filter(|e| e.enabled)
    }

    /// Move a library path up in priority
    pub fn move_path_up(&mut self, index: usize) {
        if index > 0 && index < self.library_paths.len() {
            self.library_paths.swap(index, index - 1);
        }
    }

    /// Move a library path down in priority
    pub fn move_path_down(&mut self, index: usize) {
        if index + 1 < self.library_paths.len() {
            self.library_paths.swap(index, index + 1);
        }
    }

    // =========================================================================
    // Environment Variables
    // =========================================================================

    /// Set an environment variable override
    pub fn set_env_var(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.environment_variables.insert(name.into(), value.into());
    }

    /// Remove an environment variable
    pub fn remove_env_var(&mut self, name: &str) -> Option<String> {
        self.environment_variables.remove(name)
    }

    /// Get an environment variable (checks overrides first, then system)
    pub fn get_env_var(&self, name: &str) -> Option<String> {
        self.environment_variables
            .get(name)
            .cloned()
            .or_else(|| std::env::var(name).ok())
    }

    /// Expand environment variables in a path string
    ///
    /// Supports:
    /// - `$VAR` and `${VAR}` syntax
    /// - Nested resolution
    /// - Both config overrides and system environment
    pub fn expand_path(&self, path: &str) -> String {
        let mut result = path.to_string();
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10; // Prevent infinite recursion

        // Keep expanding until no more variables or max iterations
        while iterations < MAX_ITERATIONS {
            let before = result.clone();
            result = self.expand_path_once(&result);
            if result == before {
                break;
            }
            iterations += 1;
        }

        result
    }

    /// Single pass of environment variable expansion
    fn expand_path_once(&self, path: &str) -> String {
        let mut result = String::with_capacity(path.len());
        let mut chars = path.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '$' {
                // Check for ${VAR} or $VAR syntax
                let var_name = if chars.peek() == Some(&'{') {
                    chars.next(); // consume '{'
                    let mut name = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch == '}' {
                            chars.next(); // consume '}'
                            break;
                        }
                        name.push(chars.next().unwrap());
                    }
                    name
                } else {
                    // $VAR - read until non-alphanumeric/underscore
                    let mut name = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    name
                };

                // Expand the variable
                if !var_name.is_empty() {
                    if let Some(value) = self.get_env_var(&var_name) {
                        result.push_str(&value);
                    } else {
                        // Keep original if not found
                        result.push('$');
                        result.push_str(&var_name);
                    }
                } else {
                    result.push('$');
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    /// Expand a path and convert to PathBuf
    pub fn expand_path_buf(&self, path: &str) -> PathBuf {
        PathBuf::from(self.expand_path(path))
    }
}
