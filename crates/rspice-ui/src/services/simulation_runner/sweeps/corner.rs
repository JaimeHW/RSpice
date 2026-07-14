use super::execution::{expand_corner_points, run_corner_sweep};
use super::mapping::map_corner_results;
use super::netlist_mutation::infer_nominal_supply_voltage;
use super::sweep_points::extract_temp_points;
use super::types::{
    CornerData, CornerRunConfig, REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END,
};
use std::collections::HashMap;
use std::path::Path;

/// Run corner analysis from `.TEMP` commands in the netlist.
///
/// This compatibility entry point executes temperature-only TT/nominal sweeps.
pub fn run_corner_analysis(netlist_text: &str) -> Result<CornerData, String> {
    run_corner_analysis_with_source_path(netlist_text, None)
}

/// Run corner analysis from `.TEMP` commands in the netlist, resolving relative
/// includes from the source path when provided.
pub fn run_corner_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    let netlist = super::super::parse_runner_netlist(netlist_text, source_path)?;
    let temperatures = extract_temp_points(&netlist);

    if temperatures.is_empty() {
        return Err("Corner analysis requires at least one .TEMP command".to_string());
    }

    let config = CornerRunConfig {
        temperatures_c: temperatures,
        ..Default::default()
    };
    run_corner_analysis_with_netlist(&netlist, &config)
}

/// Run corner analysis with explicit process/voltage/temperature configuration.
pub fn run_corner_analysis_with_config(
    netlist_text: &str,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    run_corner_analysis_with_config_and_source_path(netlist_text, config, None)
}

/// Run corner analysis with explicit process/voltage/temperature configuration
/// and a source path used to resolve relative includes and model file
/// references.
pub fn run_corner_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &CornerRunConfig,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    config.validate()?;
    if config.model_bindings.is_empty() {
        let netlist = super::super::parse_runner_netlist(netlist_text, source_path)?;
        return run_corner_analysis_with_netlist(&netlist, config);
    }
    run_corner_analysis_with_bound_models(netlist_text, config, source_path)
}

fn run_corner_analysis_with_bound_models(
    netlist_text: &str,
    config: &CornerRunConfig,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    let points = expand_corner_points(config);
    if points.is_empty() {
        return Err("Corner analysis produced no corner points".to_owned());
    }
    let source_without_reference = strip_reference_model_binding(netlist_text)?;
    let mut process_netlists = HashMap::new();
    for process in &config.process_corners {
        process_netlists.entry(*process).or_insert_with(|| {
            let directives: Vec<String> = config
                .model_bindings
                .iter()
                .filter(|binding| binding.process == *process)
                .map(|binding| binding.spice_directive())
                .collect();
            inject_model_directives(&source_without_reference, &directives)
        });
    }

    let mut parsed_by_process = HashMap::new();
    for (process, source) in process_netlists {
        let parsed = super::super::parse_runner_netlist(&source, source_path).map_err(|error| {
            format!(
                "{} model-section binding failed: {error}",
                process.as_keyword()
            )
        })?;
        parsed_by_process.insert(process, parsed);
    }

    let mut results = Vec::with_capacity(points.len());
    for point in &points {
        let netlist = parsed_by_process.get(&point.process).ok_or_else(|| {
            format!(
                "No parsed model binding exists for {}",
                point.process.as_keyword()
            )
        })?;
        let nominal_voltage = config
            .nominal_voltage
            .or_else(|| infer_nominal_supply_voltage(netlist))
            .unwrap_or(1.0);
        results.extend(run_corner_sweep(
            netlist,
            std::slice::from_ref(point),
            config,
            nominal_voltage,
        )?);
    }
    finish_corner_data(&points, results, config)
}

fn run_corner_analysis_with_netlist(
    netlist: &rspice_core::Netlist,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    config.validate()?;
    let points = expand_corner_points(config);
    if points.is_empty() {
        return Err("Corner analysis produced no corner points".to_string());
    }

    let nominal_voltage = config
        .nominal_voltage
        .or_else(|| infer_nominal_supply_voltage(netlist))
        .unwrap_or(1.0);
    let results = run_corner_sweep(netlist, &points, config, nominal_voltage)?;
    if results.is_empty() {
        return Err("Corner analysis produced no converged corner points".to_string());
    }

    finish_corner_data(&points, results, config)
}

fn finish_corner_data(
    points: &[super::types::CornerPoint],
    results: Vec<(super::types::CornerPoint, super::types::SweepPointResult)>,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    if results.is_empty() {
        return Err("Corner analysis produced no converged corner points".to_owned());
    }
    let num_failures = points.len().saturating_sub(results.len());
    let metric = config.base_mode.metric_label();
    let (x_values, x_label, x_unit, temperatures_c, corner_labels, voltages) =
        map_corner_results(&results, metric);

    Ok(CornerData {
        x_values,
        x_label,
        x_unit,
        num_points: temperatures_c.len(),
        temperatures_c,
        corner_labels,
        voltages,
        num_failures,
    })
}

fn strip_reference_model_binding(source: &str) -> Result<String, String> {
    let mut result = Vec::new();
    let mut inside_binding = false;
    let mut saw_binding = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == REFERENCE_MODEL_BINDING_BEGIN {
            if inside_binding || saw_binding {
                return Err("Malformed reference model-binding block".to_owned());
            }
            inside_binding = true;
            saw_binding = true;
            continue;
        }
        if trimmed == REFERENCE_MODEL_BINDING_END {
            if !inside_binding {
                return Err("Reference model-binding block ends without a start marker".to_owned());
            }
            inside_binding = false;
            continue;
        }
        if !inside_binding {
            result.push(line);
        }
    }
    if inside_binding {
        return Err("Reference model-binding block is not closed".to_owned());
    }
    let mut stripped = result.join("\n");
    if source.ends_with('\n') {
        stripped.push('\n');
    }
    Ok(stripped)
}

fn inject_model_directives(source: &str, directives: &[String]) -> String {
    if directives.is_empty() {
        return source.to_owned();
    }
    let mut lines: Vec<String> = source.lines().map(str::to_owned).collect();
    let insertion_idx = lines
        .iter()
        .position(|line| line.trim_start().to_ascii_lowercase().starts_with(".end"))
        .unwrap_or(lines.len());
    lines.splice(insertion_idx..insertion_idx, directives.iter().cloned());
    let mut merged = lines.join("\n");
    if source.ends_with('\n') {
        merged.push('\n');
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::super::types::{CornerBaseMode, CornerModelBinding, CornerProcess};
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn process_binding_replaces_reference_binding_block() {
        let source = format!(
            "title\nR1 in 0 1k\n{REFERENCE_MODEL_BINDING_BEGIN}\n.lib \"old.lib\" TT\n{REFERENCE_MODEL_BINDING_END}\n.op\n.end\n"
        );

        let stripped = strip_reference_model_binding(&source).expect("marker block is valid");
        let rebound = inject_model_directives(&stripped, &[".lib \"new.lib\" FF".to_owned()]);

        assert!(!rebound.contains("old.lib"));
        assert!(rebound.contains(".lib \"new.lib\" FF"));
        assert!(rebound.find("new.lib").unwrap() < rebound.find(".end").unwrap());
    }

    #[test]
    fn explicit_library_section_drives_non_typical_corner() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rspice-corner-binding-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).expect("create corner fixture directory");
        let library_path = directory.join("models.lib");
        fs::write(
            &library_path,
            ".lib FF\n.model DFAST D (IS=1e-12)\n.endl FF\n",
        )
        .expect("write corner model fixture");
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::FF],
            voltages: vec![1.0],
            temperatures_c: vec![27.0],
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::Op,
            model_bindings: vec![CornerModelBinding {
                process: CornerProcess::FF,
                library_path: library_path.to_string_lossy().into_owned(),
                section: Some("FF".to_owned()),
            }],
            ..CornerRunConfig::default()
        };
        let deck = "binding test\nV1 in 0 1\nR1 in out 1k\nD1 out 0 DFAST\n.op\n.end\n";

        let result = run_corner_analysis_with_config(deck, &config)
            .expect("the selected FF section supplies DFAST");

        assert_eq!(result.num_points, 1);
        assert_eq!(result.corner_labels, vec!["FF_1.000000V_27.000000C"]);
        fs::remove_dir_all(directory).expect("remove corner fixture directory");
    }
}
