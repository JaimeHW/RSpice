use std::collections::HashSet;

pub(super) fn spice_to_ahdl_compatible_netlist(spice_netlist: &str) -> String {
    let mut ahdl_paths: Vec<String> = Vec::new();
    let mut seen_paths: HashSet<String> = HashSet::new();
    let mut retained_lines: Vec<&str> = Vec::new();

    for line in spice_netlist.lines() {
        if let Some(path) = parse_veriloga_include_path(line) {
            if seen_paths.insert(path.clone()) {
                ahdl_paths.push(path);
            }
            continue;
        }
        retained_lines.push(line);
    }

    if ahdl_paths.is_empty() {
        return retained_lines.join("\n");
    }

    let mut output: Vec<String> = Vec::with_capacity(retained_lines.len() + ahdl_paths.len() + 3);
    let mut inserted_prefix = false;
    for line in retained_lines {
        let trimmed = line.trim();
        if !inserted_prefix && !trimmed.is_empty() && !trimmed.starts_with('*') {
            push_ahdl_prefix(&mut output, &ahdl_paths);
            inserted_prefix = true;
        }
        output.push(line.to_string());
    }

    if !inserted_prefix {
        push_ahdl_prefix(&mut output, &ahdl_paths);
    }

    output.join("\n")
}

fn push_ahdl_prefix(output: &mut Vec<String>, ahdl_paths: &[String]) {
    output.push("simulator lang=spectre".to_string());
    for path in ahdl_paths {
        output.push(format!("ahdl_include {}", quote_netlist_path(path)));
    }
    output.push("simulator lang=spice".to_string());
}

pub(super) fn parse_veriloga_include_path(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let prefix = trimmed.get(..9)?;
    if !prefix.eq_ignore_ascii_case(".veriloga") {
        return None;
    }

    let rest = trimmed.get(9..)?.trim_start();
    if rest.is_empty() {
        return None;
    }

    if let Some(quoted) = rest.strip_prefix('"') {
        let end = quoted.find('"')?;
        return Some(quoted[..end].to_string());
    }

    rest.split_whitespace().next().map(ToString::to_string)
}

pub(super) fn quote_netlist_path(path: &str) -> String {
    let escaped = path.replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

