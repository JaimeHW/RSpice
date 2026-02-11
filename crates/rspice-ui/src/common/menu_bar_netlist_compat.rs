use std::collections::HashSet;

pub(super) fn spice_to_spectre_compatible_netlist(spice_netlist: &str) -> String {
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
            push_spectre_ahdl_prefix(&mut output, &ahdl_paths);
            inserted_prefix = true;
        }
        output.push(line.to_string());
    }

    if !inserted_prefix {
        push_spectre_ahdl_prefix(&mut output, &ahdl_paths);
    }

    output.join("\n")
}

fn push_spectre_ahdl_prefix(output: &mut Vec<String>, ahdl_paths: &[String]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_veriloga_include_path_handles_quoted_and_unquoted_paths() {
        let quoted = r#".VERILOGA "C:/models/opamp.va" opamp"#;
        let bare = ".veriloga C:/models/opamp.va";
        let unrelated = ".include \"models.lib\"";

        assert_eq!(
            parse_veriloga_include_path(quoted),
            Some("C:/models/opamp.va".to_string())
        );
        assert_eq!(
            parse_veriloga_include_path(bare),
            Some("C:/models/opamp.va".to_string())
        );
        assert_eq!(parse_veriloga_include_path(unrelated), None);
    }

    #[test]
    fn test_spice_to_spectre_compatible_netlist_converts_veriloga_directives() {
        let spice = r#"
* Header
.VERILOGA "C:/models/opamp.va" opamp
R1 in out 1k
.end
"#;

        let spectre = spice_to_spectre_compatible_netlist(spice);
        assert!(spectre.contains("simulator lang=spectre"));
        assert!(spectre.contains("ahdl_include \"C:/models/opamp.va\""));
        assert!(spectre.contains("simulator lang=spice"));
        assert!(spectre.contains("R1 in out 1k"));
        assert!(!spectre.contains(".VERILOGA"));
    }

    #[test]
    fn test_spice_to_spectre_compatible_netlist_preserves_spice_when_no_veriloga() {
        let spice = "R1 in out 1k\n.end";
        assert_eq!(spice_to_spectre_compatible_netlist(spice), spice);
    }

    #[test]
    fn test_spice_to_spectre_compatible_netlist_deduplicates_veriloga_includes() {
        let spice = r#"
.veriloga "C:/models/opamp.va"
.VERILOGA "C:/models/opamp.va"
R1 in out 1k
"#;

        let spectre = spice_to_spectre_compatible_netlist(spice);
        assert_eq!(spectre.matches("ahdl_include").count(), 1);
    }

    #[test]
    fn test_spice_to_spectre_compatible_netlist_inserts_prefix_after_header_comments() {
        let spice = r#"
* generated netlist
* keep this header
.VERILOGA "C:/models/opamp.va"
R1 in out 1k
"#;

        let spectre = spice_to_spectre_compatible_netlist(spice);
        let expected = "* keep this header\nsimulator lang=spectre";
        assert!(spectre.contains(expected));
    }
}
