//! Simulation Runner
//!
//! Shared simulation execution logic for both menu and toolbar triggers.
//! This module provides a centralized run_simulation() function that handles:
//! - Netlist generation from schematic
//! - Simulation command configuration
//! - Async simulation execution
//! - Result processing (waveforms, DC annotations)
//!
//! Commercial-grade design: single source of truth for simulation execution.

use crate::dialogs::SimulationOptions;
use crate::state::dc_annotation::Annotation;
use crate::state::dc_annotation_placement::{select_anchor_point, AnnotationPlacer};
use crate::state::simulation_command::SimulationConfig;
use crate::state::{
    generate_netlist, run_simulation_with_options, Component, ConsoleMessage, Point,
    SchematicState, SimulationResult, SimulationState, WaveformData, Wire,
};
use crate::theme::Theme;
use dioxus::prelude::*;

/// Strip existing analysis commands from a netlist.
/// This allows dialog-configured commands to replace schematic-embedded ones.
pub fn strip_analysis_commands(netlist: &str) -> String {
    let analysis_prefixes = [".tran", ".dc", ".ac", ".op", ".noise", ".tf", ".sens"];
    netlist
        .lines()
        .filter(|line| {
            let lower = line.to_lowercase();
            let trimmed = lower.trim();
            !analysis_prefixes
                .iter()
                .any(|prefix| trimmed.starts_with(prefix))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Run a simulation with the current schematic and configuration.
///
/// This is the central simulation execution function used by both:
/// - Toolbar Run button onclick
/// - Menu Simulate → Run action
///
/// # Arguments
/// * `schematic` - Signal containing the schematic state
/// * `sim_state` - Signal containing simulation state (is_running, waveforms, etc.)
/// * `sim_config` - Signal containing simulation configuration (analysis type, params)
/// * `sim_options` - Signal containing simulation options (tolerances, etc.)
pub fn run_simulation(
    mut schematic: Signal<SchematicState>,
    mut sim_state: Signal<SimulationState>,
    sim_config: Signal<SimulationConfig>,
    sim_options: Signal<SimulationOptions>,
) {
    // Generate netlist from schematic
    let schematic_state = schematic.read();
    let netlist_result = generate_netlist(&schematic_state);
    drop(schematic_state);

    // Store the point-to-net mapping for probe tool
    schematic.write().net_mapping = netlist_result.point_to_net.clone();

    // Check for errors in netlist generation
    if !netlist_result.errors.is_empty() {
        let mut state = sim_state.write();
        for err in &netlist_result.errors {
            state
                .console_messages
                .push(ConsoleMessage::error(err.clone()));
        }
        return;
    }

    // Log warnings
    for warn in &netlist_result.warnings {
        sim_state
            .write()
            .console_messages
            .push(ConsoleMessage::warning(warn.clone()));
    }

    // Get simulation configuration and generate SPICE commands
    let config = sim_config.read();
    let sim_commands = config.to_spice_string();
    let has_config = config.has_analysis();
    drop(config);

    // Build complete netlist with simulation commands
    let mut netlist_content = netlist_result.netlist.clone();

    // Behavior: if dialog has configuration, it REPLACES
    // any existing analysis commands in the netlist
    if has_config {
        // Strip existing analysis commands from netlist
        netlist_content = strip_analysis_commands(&netlist_content);

        // Insert configured commands before .END
        if let Some(end_pos) = netlist_content.to_uppercase().rfind(".END") {
            netlist_content.insert_str(end_pos, &format!("{}\n\n", sim_commands));
        } else {
            // No .END found, append commands at the end
            netlist_content.push_str(&format!("\n{}\n.END\n", sim_commands));
        }
    }
    // If no dialog config, use whatever's already in the netlist

    sim_state.write().netlist_content = netlist_content.clone();
    sim_state.write().is_running = true;

    // Log the generated netlist for debugging
    log::info!("Generated netlist:\n{}", netlist_content);
    for (net_name, _points) in &netlist_result.nets {
        log::info!("  Net: {}", net_name);
    }

    sim_state
        .write()
        .console_messages
        .push(ConsoleMessage::info(format!(
            "Generated netlist ({} nets)",
            netlist_result.nets.len()
        )));

    // Clone data needed for async block
    let netlist_for_sim = netlist_content.clone();
    let nets_for_ground = netlist_result.nets.clone();
    // Clone point_to_net for component-terminal annotation positions
    let point_to_net = netlist_result.point_to_net.clone();
    // Clone component data for annotation placement
    let components_for_exclusion: Vec<Component> = schematic.read().components.clone();
    let wires_for_exclusion: Vec<Wire> = schematic.read().wires.clone();
    let component_positions: Vec<Point> = components_for_exclusion.iter().map(|c| c.pos).collect();
    let grid_size_for_ann = schematic.read().grid_size;

    // Clone simulation options for async block
    let options_for_sim = sim_options.read().clone();

    // Run simulation in background thread to prevent UI freeze
    // spawn_blocking is essential for CPU-bound work in async context
    spawn(async move {
        // Use spawn_blocking to run CPU-intensive simulation on thread pool
        let result = tokio::task::spawn_blocking(move || {
            run_simulation_with_options(&netlist_for_sim, Some(&options_for_sim))
        })
        .await
        .unwrap_or_else(|_| SimulationResult {
            success: false,
            transient: None,
            dc_op: None,
            error: Some("Simulation task panicked".to_string()),
            stats: Default::default(),
        });

        // Update state with results
        let mut state = sim_state.write();
        state.is_running = false;

        if result.success {
            // Clear old waveforms and add new ones
            state.waveforms.clear();

            // Collect waveform names for ground detection
            let mut waveform_nets: std::collections::HashSet<String> =
                std::collections::HashSet::new();

            if let Some(tran) = result.transient {
                log::info!(
                    "Transient data: {} time points, {} voltage traces",
                    tran.time.len(),
                    tran.voltages.len()
                );
                for (idx, (name, values)) in tran.voltages.into_iter().enumerate() {
                    log::info!("  Adding waveform: {} with {} points", name, values.len());
                    // Extract net name from V(N001) -> N001
                    let net_name = name
                        .trim_start_matches("V(")
                        .trim_end_matches(')')
                        .to_string();
                    waveform_nets.insert(net_name);
                    state.waveforms.push(WaveformData {
                        name,
                        x: tran.time.clone(),
                        y: values,
                        color: Theme::trace_color_static(idx).to_string(),
                        visible: true,
                    });
                }
            } else {
                log::warn!("No transient data in simulation result");
            }

            // Derive ground node: the net that exists in netlist but not in waveforms
            let all_nets: std::collections::HashSet<String> =
                nets_for_ground.keys().cloned().collect();
            let ground_candidates: Vec<String> =
                all_nets.difference(&waveform_nets).cloned().collect();
            if let Some(ground) = ground_candidates.first() {
                state.ground_node = Some(ground.clone());
                log::info!("Ground reference node: {}", ground);
            } else {
                state.ground_node = None;
            }

            // ============================================================
            // DC Annotation Population
            // ============================================================
            if let Some(ref dc_op) = result.dc_op {
                log::info!(
                    "Populating DC annotations from {} operating points",
                    dc_op.len()
                );

                // Use AnnotationPlacer with radial search algorithm
                let mut placer = AnnotationPlacer::from_schematic(
                    &components_for_exclusion,
                    &wires_for_exclusion,
                    grid_size_for_ann,
                );

                // Build net name → position map using point_to_net
                let mut net_positions: std::collections::HashMap<String, Point> =
                    std::collections::HashMap::new();

                // Invert point_to_net: for each net, collect all its points
                let mut net_to_points: std::collections::HashMap<String, Vec<Point>> =
                    std::collections::HashMap::new();
                for (point, net_name) in &point_to_net {
                    net_to_points
                        .entry(net_name.clone())
                        .or_default()
                        .push(*point);
                }

                // Wire-centric anchor selection
                for (net_name, points) in &net_to_points {
                    if points.is_empty() {
                        continue;
                    }

                    let best_point =
                        select_anchor_point(points, &wires_for_exclusion, &component_positions)
                            .unwrap_or(points[0]);

                    net_positions.insert(net_name.clone(), best_point);
                    log::info!(
                        "  Net {} annotation anchor at ({}, {}) - wire-centric selection",
                        net_name,
                        best_point.x,
                        best_point.y
                    );
                }

                // Clear existing annotations and populate with new DC results
                state.dc_annotations.clear();

                // Process each DC operating point voltage using radial search placement
                for (net_name, voltage) in dc_op {
                    let position = net_positions.get(net_name.as_str());

                    if let Some(&pos) = position {
                        // Use AnnotationPlacer to find collision-free position
                        let label_text = crate::state::dc_annotation::format_voltage(*voltage);
                        let offset = placer.find_optimal_position(pos, &label_text);

                        let mut annotation = Annotation::voltage(pos, *voltage, net_name.clone());
                        annotation.offset = offset;

                        // Mark this annotation as placed for subsequent annotations
                        placer.mark_placed(pos, offset, &label_text);

                        state
                            .dc_annotations
                            .voltages
                            .insert(net_name.clone(), annotation);
                        log::info!("  DC annotation: {} = {:.4}V at ({}, {}) offset ({:.1}, {:.1}) - collision-free",
                            net_name, voltage, pos.x, pos.y, offset.0, offset.1);
                    }
                }

                log::info!(
                    "DC annotations populated: {} voltage annotations",
                    state.dc_annotations.voltages.len()
                );
            }

            log::info!(
                "Total waveforms after simulation: {}",
                state.waveforms.len()
            );

            state.console_messages.push(ConsoleMessage::success(format!(
                "Simulation complete! {} points, {:.1}ms",
                result.stats.num_points, result.stats.sim_time_ms
            )));
            log::info!(
                "Simulation completed: {} points in {:.1}ms",
                result.stats.num_points,
                result.stats.sim_time_ms
            );
        } else {
            state.console_messages.push(ConsoleMessage::error(
                result.error.unwrap_or_else(|| "Unknown error".to_string()),
            ));
            log::error!("Simulation failed");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_analysis_commands_removes_tran() {
        let netlist = "V1 in 0 5V\n.tran 0 1ms 0 1us\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.contains(".tran"));
        assert!(result.contains("V1 in 0 5V"));
        assert!(result.contains("R1 in out 1k"));
    }

    #[test]
    fn test_strip_analysis_commands_removes_dc() {
        let netlist = "V1 in 0 5V\n.dc V1 0 10 0.1\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.contains(".dc"));
        assert!(result.contains("V1 in 0 5V"));
    }

    #[test]
    fn test_strip_analysis_commands_removes_ac() {
        let netlist = "V1 in 0 AC 1\n.ac dec 10 1 1Meg\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.to_lowercase().contains(".ac dec"));
        assert!(result.contains("V1 in 0 AC 1"));
    }

    #[test]
    fn test_strip_analysis_commands_removes_op() {
        let netlist = "V1 in 0 5V\n.op\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.contains(".op"));
    }

    #[test]
    fn test_strip_analysis_commands_removes_multiple() {
        let netlist = "V1 in 0 5V\n.op\n.tran 0 1ms 0 1us\n.dc V1 0 10 0.1\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.contains(".op"));
        assert!(!result.contains(".tran"));
        assert!(!result.contains(".dc"));
        assert!(result.contains("V1 in 0 5V"));
        assert!(result.contains("R1 in out 1k"));
    }

    #[test]
    fn test_strip_analysis_commands_case_insensitive() {
        let netlist = "V1 in 0 5V\n.TRAN 0 1ms 0 1us\n.DC V1 0 10 0.1\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.to_lowercase().contains(".tran"));
        assert!(!result.to_lowercase().contains(".dc"));
    }

    #[test]
    fn test_strip_analysis_commands_preserves_end() {
        let netlist = "V1 in 0 5V\n.tran 0 1ms 0 1us\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(result.contains(".END"));
    }

    #[test]
    fn test_strip_analysis_commands_removes_noise() {
        let netlist = "V1 in 0 5V\n.noise V(out) V1 dec 10 1 1Meg\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.to_lowercase().contains(".noise"));
    }

    #[test]
    fn test_strip_analysis_commands_removes_tf() {
        let netlist = "V1 in 0 5V\n.tf V(out) V1\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.to_lowercase().contains(".tf"));
    }

    #[test]
    fn test_strip_analysis_commands_removes_sens() {
        let netlist = "V1 in 0 5V\n.sens V(out)\nR1 in out 1k\n.END";
        let result = strip_analysis_commands(netlist);
        assert!(!result.to_lowercase().contains(".sens"));
    }
}
