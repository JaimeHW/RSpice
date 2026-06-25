#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustDeviceNames {
    pub public_model_name: String,
    pub rust_module: String,
    pub folder: String,
}

impl RustDeviceNames {
    pub fn new(source_file_name: &str, module_name: &str, digest: &str) -> Self {
        let source_stem = source_file_name
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(source_file_name)
            .split('.')
            .next()
            .unwrap_or(source_file_name);
        let digest = digest.chars().take(8).collect::<String>();
        let source = sanitize_identifier(source_stem);
        let module = sanitize_identifier(module_name);

        Self {
            public_model_name: module_name.to_string(),
            rust_module: format!("{module}__{digest}"),
            folder: format!("{source}__{module}__{digest}"),
        }
    }
}

pub fn sanitize_identifier(input: &str) -> String {
    let mut out = String::with_capacity(input.len().max(1));
    for ch in input.chars() {
        if ch == '_' || ch.is_ascii_alphanumeric() {
            if out.is_empty() && ch.is_ascii_digit() {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }

    let out = out.trim_matches('_').to_string();
    if out.is_empty() {
        "_model".to_string()
    } else {
        out
    }
}
