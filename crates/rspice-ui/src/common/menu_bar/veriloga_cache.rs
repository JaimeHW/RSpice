use crate::common::app::{AppState, ConsoleMessage};

pub(super) fn format_cache_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= GB {
        format!("{:.2} GiB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MiB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KiB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}

pub(crate) fn action_veriloga_cache_status(state: &mut AppState) {
    match rspice_core::veriloga_cache_stats() {
        Ok(stats) => {
            state.push_user_message(ConsoleMessage::info(format!(
                "Verilog-A compile cache: {} entries, {} used (limits: {} entries, {}) at {}",
                stats.entry_count,
                format_cache_bytes(stats.total_bytes),
                stats.max_entries,
                format_cache_bytes(stats.max_bytes),
                stats.root.display()
            )));
        }
        Err(err) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Failed to inspect Verilog-A compile cache: {}",
                err
            )));
        }
    }
}

pub(crate) fn action_veriloga_cache_list_entries(state: &mut AppState) {
    match rspice_core::veriloga_cache_entries() {
        Ok(entries) => {
            if entries.is_empty() {
                state.push_user_message(ConsoleMessage::info("Verilog-A compile cache is empty"));
                return;
            }

            state.push_user_message(ConsoleMessage::info(format!(
                "Verilog-A compile cache entries: {}",
                entries.len()
            )));
            let max_lines = 64_usize;
            for entry in entries.iter().take(max_lines) {
                state.push_user_message(ConsoleMessage::info(format!(
                    "  {} -> {} (deps: {}, size: {})",
                    entry.source_path.display(),
                    entry.cache_path.display(),
                    entry.dependencies.len(),
                    format_cache_bytes(entry.size_bytes)
                )));
                for dep in &entry.dependencies {
                    state.push_user_message(ConsoleMessage::info(format!(
                        "    dep: {}",
                        dep.display()
                    )));
                }
            }
            if entries.len() > max_lines {
                state.push_user_message(ConsoleMessage::warning(format!(
                    "Listing truncated to first {} entries ({} total)",
                    max_lines,
                    entries.len()
                )));
            }
        }
        Err(err) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Failed to list Verilog-A compile cache entries: {}",
                err
            )));
        }
    }
}

pub(crate) fn action_veriloga_cache_prune(state: &mut AppState) {
    match rspice_core::prune_veriloga_cache() {
        Ok(report) => {
            state.push_user_message(ConsoleMessage::info(format!(
                "Pruned Verilog-A compile cache: removed {} entries, reclaimed {} (now {} entries, {})",
                report.removed_entries,
                format_cache_bytes(report.reclaimed_bytes),
                report.stats.entry_count,
                format_cache_bytes(report.stats.total_bytes)
            )));
        }
        Err(err) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Failed to prune Verilog-A compile cache: {}",
                err
            )));
        }
    }
}

pub(crate) fn action_veriloga_cache_clear(state: &mut AppState) {
    match rspice_core::clear_veriloga_cache() {
        Ok(report) => {
            state.push_user_message(ConsoleMessage::info(format!(
                "Cleared Verilog-A compile cache: removed {} entries, reclaimed {}",
                report.removed_entries,
                format_cache_bytes(report.reclaimed_bytes)
            )));
        }
        Err(err) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Failed to clear Verilog-A compile cache: {}",
                err
            )));
        }
    }
}

pub(crate) fn action_veriloga_recompile_library(state: &mut AppState) {
    #[derive(Clone)]
    struct Candidate {
        library: String,
        cell: String,
        view: String,
        source_path: std::path::PathBuf,
    }

    let Some(lib) = state.library_manager.get_library("veriloga") else {
        state.push_user_message(ConsoleMessage::warning(
            "No global 'veriloga' library found",
        ));
        return;
    };

    let (candidates, discovery_warnings) = {
        let mut candidates = Vec::<Candidate>::new();
        let mut discovery_warnings = Vec::<String>::new();
        for cell in lib.cells_sorted() {
            for view in cell.views_sorted() {
                if view.view_type != crate::state::library_browser::ViewType::VerilogA {
                    continue;
                }
                let source_path = view
                    .file_path
                    .clone()
                    .or_else(|| {
                        view.metadata
                            .get("netlist.source_path")
                            .or_else(|| view.metadata.get("veriloga.source_path"))
                            .map(std::path::PathBuf::from)
                    })
                    .or_else(|| {
                        cell.metadata
                            .get("netlist.source_path")
                            .or_else(|| cell.metadata.get("veriloga.source_path"))
                            .map(std::path::PathBuf::from)
                    });
                let Some(source_path) = source_path else {
                    discovery_warnings.push(format!(
                        "Skipping {}/{}/{}: missing source path metadata",
                        lib.name, cell.name, view.name
                    ));
                    continue;
                };
                candidates.push(Candidate {
                    library: lib.name.clone(),
                    cell: cell.name.clone(),
                    view: view.name.clone(),
                    source_path,
                });
            }
        }
        (candidates, discovery_warnings)
    };

    for warning in discovery_warnings {
        state.push_user_message(ConsoleMessage::warning(warning));
    }

    if candidates.is_empty() {
        state.push_user_message(ConsoleMessage::warning(
            "No Verilog-A views found in global library",
        ));
        return;
    }

    let compiler = rspice_veriloga::VerilogACompiler::default();
    let mut ok_count = 0_usize;
    let mut fail_count = 0_usize;
    state.push_user_message(ConsoleMessage::info(format!(
        "Recompiling {} Verilog-A model(s) from global library...",
        candidates.len()
    )));

    for candidate in candidates {
        match compiler.compile_file_with_metadata(&candidate.source_path) {
            Ok(compiled) => {
                let model_name = compiled.model.name.clone();
                match rspice_core::register_precompiled_veriloga_model_with_dependencies(
                    &candidate.source_path,
                    &compiled.dependencies,
                    compiled.model,
                ) {
                    Ok(()) => {
                        ok_count += 1;
                        state.push_user_message(ConsoleMessage::info(format!(
                            "Recompiled {}/{}/{} -> '{}' ({} deps)",
                            candidate.library,
                            candidate.cell,
                            candidate.view,
                            model_name,
                            compiled.dependencies.len()
                        )));
                    }
                    Err(err) => {
                        fail_count += 1;
                        state.push_user_message(ConsoleMessage::error(format!(
                            "Cache registration failed for {}/{}/{} ({}): {}",
                            candidate.library,
                            candidate.cell,
                            candidate.view,
                            candidate.source_path.display(),
                            err
                        )));
                    }
                }
            }
            Err(err) => {
                fail_count += 1;
                state.push_user_message(ConsoleMessage::error(format!(
                    "Compile failed for {}/{}/{} ({}): {}",
                    candidate.library,
                    candidate.cell,
                    candidate.view,
                    candidate.source_path.display(),
                    err
                )));
            }
        }
    }

    state.push_user_message(ConsoleMessage::info(format!(
        "Verilog-A recompile complete: {} succeeded, {} failed",
        ok_count, fail_count
    )));
}
