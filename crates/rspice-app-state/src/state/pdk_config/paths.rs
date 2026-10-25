//! Expand PDK paths against configuration overrides and the host environment.

use super::PdkConfig;

fn get_env_var(config: &PdkConfig, name: &str) -> Option<String> {
    config
        .environment_variables
        .get(name)
        .cloned()
        .or_else(|| std::env::var(name).ok())
}

pub fn expand_path(config: &PdkConfig, path: &str) -> String {
    let mut result = path.to_string();
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 10; // Prevent infinite recursion

    // Keep expanding until no more variables or max iterations
    while iterations < MAX_ITERATIONS {
        let before = result.clone();
        result = expand_path_once(config, &result);
        if result == before {
            break;
        }
        iterations += 1;
    }

    result
}

fn expand_path_once(config: &PdkConfig, path: &str) -> String {
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
                if let Some(value) = get_env_var(config, &var_name) {
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
