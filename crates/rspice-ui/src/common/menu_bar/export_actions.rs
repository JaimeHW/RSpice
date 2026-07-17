use crate::common::app::AppState;
use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub(super) fn action_export_svg_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    use crate::schematic::export::{SvgExportConfig, export_to_svg_with_symbol_resolver};

    let config = SvgExportConfig::default();
    let resolver = crate::state::SymbolResolver::new(
        &state.library_manager,
        &state.workspace.schematic_buffers,
    );
    let svg_content = export_to_svg_with_symbol_resolver(&state.schematic, &config, &resolver);

    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| format!("{}.svg", s.to_string_lossy()))
        .unwrap_or_else(|| "schematic.svg".to_string());

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export SVG",
        default_name: &default_name,
        filter_name: "SVG Image",
        filter_extensions: &["svg"],
    }) {
        Ok(Some(mut path)) => {
            crate::common::file_actions::ensure_file_extension(&mut path, "svg");

            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &svg_content));
            match export {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        crate::common::export_workflow::export_completion_message(
                            "SVG", &path, None, io,
                        ),
                    ));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                        "SVG export failed: {}",
                        e
                    )));
                }
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "SVG export failed: {}",
                e
            )));
        }
    }
}

pub(crate) fn action_export_netlist_with_io(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let netlist_content =
        if state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
            if state.ui.netlist.generated_source.is_empty() {
                state.push_user_message(crate::common::app::ConsoleMessage::warning(
                    "No retained generated artifact is available to export.",
                ));
                return;
            }
            if state.ui.netlist.generation_error.is_some()
                || state.ui.netlist.generated_input_digest
                    != state.ui.netlist.current_generation_input_digest
            {
                state.push_user_message(crate::common::app::ConsoleMessage::warning(
                    "Generated export is blocked because the retained artifact is stale.",
                ));
                return;
            }
            match format {
                crate::io::NetlistFormat::Spectre => {
                    super::netlist_compat::spice_to_ahdl_compatible_netlist(
                        &state.ui.netlist.generated_source,
                    )
                }
                _ => state.ui.netlist.generated_source.clone(),
            }
        } else {
            if state.schematic.components.is_empty() {
                state.push_user_message(crate::common::app::ConsoleMessage::warning(
                    "No circuit to export. Add components first.",
                ));
                return;
            }
            let Some(content) = build_menu_netlist(state, format) else {
                return;
            };
            content
        };

    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| format!("{}.{}", s.to_string_lossy(), format.extension()))
        .unwrap_or_else(|| format!("circuit.{}", format.extension()));

    let filter_name = match format {
        crate::io::NetlistFormat::Spectre => "Spectre Netlist",
        crate::io::NetlistFormat::Spice => "SPICE Netlist",
        crate::io::NetlistFormat::Hspice => "HSPICE Netlist",
        crate::io::NetlistFormat::Xyce => "Xyce Netlist",
    };

    let extension = format.extension();

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Netlist",
        default_name: &default_name,
        filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(Some(mut path)) => {
            crate::common::file_actions::ensure_file_extension(&mut path, extension);

            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_text_file_observed(&destination, &netlist_content)
            });
            match export {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        crate::common::export_workflow::export_completion_message(
                            filter_name,
                            &path,
                            None,
                            io,
                        ),
                    ));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                        "Netlist export failed: {}",
                        e
                    )));
                }
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Netlist export failed: {}",
                e
            )));
        }
    }
}

pub(crate) fn action_export_generated_netlist_with_options(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
    bundle_dependencies: bool,
    include_source_map: bool,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> bool {
    let Some(document) = state.ui.netlist.generated_document.as_ref() else {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "No retained generated artifact is available to export.",
        ));
        return false;
    };
    let artifact = document.generated_artifact().clone();
    if state.ui.netlist.generation_error.is_some()
        || state.ui.netlist.generated_input_digest
            != state.ui.netlist.current_generation_input_digest
        || state.ui.netlist.generated_input_digest != Some(artifact.provenance().input().digest())
    {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "Generated export is blocked because the retained artifact is stale or inconsistent with its authenticated inputs.",
        ));
        return false;
    }
    if include_source_map && !bundle_dependencies {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "Source-map export requires a self-contained bundle.",
        ));
        return false;
    }
    if include_source_map && format != crate::io::NetlistFormat::Spice {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "Generated source maps identify exact SPICE lines and cannot be attached to a translated dialect.",
        ));
        return false;
    }
    if !bundle_dependencies && !artifact.dependencies().is_empty() {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "Generated decks with project or external source dependencies must be exported as a self-contained bundle.",
        ));
        return false;
    }
    if !bundle_dependencies {
        return publish_generated_source(state, &artifact, format, io);
    }

    let bundle = match build_generated_bundle(&artifact, format, include_source_map) {
        Ok(bundle) => bundle,
        Err(error) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Generated bundle export failed: {error}"
            )));
            return false;
        }
    };
    let default_name = format!(
        "generated-{}.zip",
        &artifact.content_digest().to_string()[..12]
    );
    let picked = io.show_save_dialog(SaveDialogConfig {
        title: "Export Generated Netlist Bundle",
        default_name: &default_name,
        filter_name: "RSpice Netlist Bundle",
        filter_extensions: &["zip"],
    });
    let Some(mut path) = (match picked {
        Ok(path) => path,
        Err(error) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Generated bundle export failed: {error}"
            )));
            return false;
        }
    }) else {
        return false;
    };
    crate::common::file_actions::ensure_file_extension(&mut path, "zip");
    let result = io.observe_destination(&path).and_then(|destination| {
        io.write_bytes_file_observed(&destination, &bundle, "application/zip")
    });
    match result {
        Ok(()) => {
            state.push_user_message(crate::common::app::ConsoleMessage::info(
                crate::common::export_workflow::export_completion_message(
                    "Generated netlist bundle",
                    &path,
                    None,
                    io,
                ),
            ));
            true
        }
        Err(error) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Generated bundle export failed: {error}"
            )));
            false
        }
    }
}

fn publish_generated_source(
    state: &mut AppState,
    artifact: &crate::workbench::code_workspace::GeneratedArtifact,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> bool {
    let source = generated_export_source(artifact.source(), format);
    let extension = format.extension();
    let filter_name = match format {
        crate::io::NetlistFormat::Spectre => "Spectre Netlist",
        crate::io::NetlistFormat::Spice => "SPICE Netlist",
        crate::io::NetlistFormat::Hspice => "HSPICE Netlist",
        crate::io::NetlistFormat::Xyce => "Xyce Netlist",
    };
    let default_name = format!(
        "generated-{}.{}",
        &artifact.content_digest().to_string()[..12],
        extension
    );
    let Some(mut path) = (match io.show_save_dialog(SaveDialogConfig {
        title: "Export Generated Netlist",
        default_name: &default_name,
        filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(path) => path,
        Err(error) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Generated netlist export failed: {error}"
            )));
            return false;
        }
    }) else {
        return false;
    };
    crate::common::file_actions::ensure_file_extension(&mut path, extension);
    let result = io
        .observe_destination(&path)
        .and_then(|destination| io.write_text_file_observed(&destination, &source));
    match result {
        Ok(()) => {
            state.push_user_message(crate::common::app::ConsoleMessage::info(
                crate::common::export_workflow::export_completion_message(
                    filter_name,
                    &path,
                    None,
                    io,
                ),
            ));
            true
        }
        Err(error) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Generated netlist export failed: {error}"
            )));
            false
        }
    }
}

fn generated_export_source(source: &str, format: crate::io::NetlistFormat) -> String {
    match format {
        crate::io::NetlistFormat::Spectre => {
            super::netlist_compat::spice_to_ahdl_compatible_netlist(source)
        }
        crate::io::NetlistFormat::Spice
        | crate::io::NetlistFormat::Hspice
        | crate::io::NetlistFormat::Xyce => source.to_owned(),
    }
}

#[derive(Debug)]
struct BundleEntry {
    name: String,
    bytes: Vec<u8>,
}

fn build_generated_bundle(
    artifact: &crate::workbench::code_workspace::GeneratedArtifact,
    format: crate::io::NetlistFormat,
    include_source_map: bool,
) -> Result<Vec<u8>, String> {
    use crate::workbench::code_workspace::DependencyResolution;

    if include_source_map && format != crate::io::NetlistFormat::Spice {
        return Err(
            "source-map coordinates are valid only for the exact SPICE generated artifact"
                .to_owned(),
        );
    }
    let dependencies = artifact.dependencies();
    let mut ordered_dependencies = dependencies.iter().enumerate().collect::<Vec<_>>();
    ordered_dependencies.sort_by(|(_, left), (_, right)| {
        left.parent()
            .map(crate::workbench::code_workspace::SourceLocator::logical_identity)
            .cmp(
                &right
                    .parent()
                    .map(crate::workbench::code_workspace::SourceLocator::logical_identity),
            )
            .then_with(|| {
                left.direct_include_index()
                    .or(left.parent_include_index())
                    .cmp(
                        &right
                            .direct_include_index()
                            .or(right.parent_include_index()),
                    )
            })
            .then_with(|| left.requested_locator().cmp(right.requested_locator()))
            .then_with(|| {
                left.locator()
                    .logical_identity()
                    .cmp(right.locator().logical_identity())
            })
    });
    let mut entry_names = vec![String::new(); dependencies.len()];
    for (bundle_index, (dependency_index, dependency)) in ordered_dependencies.iter().enumerate() {
        entry_names[*dependency_index] = format!(
            "sources/{bundle_index:03}-{}",
            safe_bundle_file_name(dependency.locator().display_name(), bundle_index)
        );
    }

    for (_, dependency) in &ordered_dependencies {
        match dependency.resolution() {
            DependencyResolution::Resolved { .. } => {}
            DependencyResolution::Unresolved => {
                return Err(format!(
                    "dependency '{}' is unresolved; validate and seal it before bundling",
                    dependency.requested_locator()
                ));
            }
            DependencyResolution::Missing { reason } => {
                return Err(format!(
                    "dependency '{}' is missing: {reason}",
                    dependency.requested_locator()
                ));
            }
        }
    }

    let direct_mappings = dependencies
        .iter()
        .enumerate()
        .filter_map(|(index, dependency)| {
            dependency.direct_include_index().map(|include_index| {
                (
                    include_index,
                    dependency.requested_locator().to_owned(),
                    entry_names[index].clone(),
                )
            })
        })
        .collect::<Vec<_>>();
    let rewritten_root = rewrite_include_locators(artifact.source(), &direct_mappings)?;
    let main_source = generated_export_source(&rewritten_root, format);
    let main_name = format!("generated.{}", format.extension());
    let retained_main_name = "retained/generated.spice";
    let mut entries = vec![
        BundleEntry {
            name: main_name.clone(),
            bytes: main_source.as_bytes().to_vec(),
        },
        BundleEntry {
            name: retained_main_name.to_owned(),
            bytes: artifact.source_bytes().to_vec(),
        },
    ];

    let mut manifest_dependencies = Vec::with_capacity(dependencies.len());
    for (index, dependency) in ordered_dependencies {
        let source = match dependency.resolution() {
            DependencyResolution::Resolved { source, .. } => source,
            DependencyResolution::Unresolved => {
                return Err(format!(
                    "dependency '{}' became unresolved while materializing the bundle",
                    dependency.requested_locator()
                ));
            }
            DependencyResolution::Missing { reason } => {
                return Err(format!(
                    "dependency '{}' became missing while materializing the bundle: {reason}",
                    dependency.requested_locator()
                ));
            }
        };
        let mut child_mappings = Vec::new();
        for (child_index, child) in dependencies.iter().enumerate() {
            if child.parent() != Some(dependency.locator()) {
                continue;
            }
            let target = entry_names[child_index]
                .strip_prefix("sources/")
                .ok_or_else(|| {
                    format!(
                        "dependency '{}' has an invalid materialized archive path",
                        child.requested_locator()
                    )
                })?
                .to_owned();
            let include_index = child.parent_include_index().ok_or_else(|| {
                format!(
                    "transitive dependency '{}' has no authenticated parent include index",
                    child.requested_locator()
                )
            })?;
            child_mappings.push((include_index, child.requested_locator().to_owned(), target));
        }
        let rewritten = rewrite_include_locators(source, &child_mappings)?;
        let materialized = generated_export_source(&rewritten, format);
        let retained_entry = format!("retained/{}", entry_names[index]);
        entries.push(BundleEntry {
            name: entry_names[index].clone(),
            bytes: materialized.as_bytes().to_vec(),
        });
        entries.push(BundleEntry {
            name: retained_entry.clone(),
            bytes: source.as_bytes().to_vec(),
        });
        manifest_dependencies.push(serde_json::json!({
            "requested_locator": dependency.requested_locator(),
            "logical_identity": dependency.locator().logical_identity(),
            "bundle_entry": entry_names[index],
            "content_digest": crate::workbench::code_workspace::content_digest(&materialized).to_string(),
            "retained_entry": retained_entry,
            "retained_content_digest": dependency.resolution().content_digest().map(|digest| digest.to_string()),
            "edge": if let Some(include_index) = dependency.direct_include_index() {
                serde_json::json!({ "owner": "generated", "include_index": include_index })
            } else {
                serde_json::json!({
                    "owner": dependency.parent().map(crate::workbench::code_workspace::SourceLocator::logical_identity),
                    "include_index": dependency.parent_include_index(),
                })
            },
        }));
    }

    if include_source_map {
        let source_map = serde_json::json!({
            "schema": "rspice-generated-source-map/v1",
            "source": retained_main_name,
            "generated_content_digest": artifact.content_digest().to_string(),
            "entries": artifact.source_map(),
        });
        entries.push(BundleEntry {
            name: "source-map.json".to_owned(),
            bytes: serde_json::to_vec_pretty(&source_map).map_err(|error| error.to_string())?,
        });
    }
    let manifest = serde_json::json!({
        "schema": "rspice-generated-netlist-bundle/v1",
        "main": main_name,
        "main_content_digest": crate::workbench::code_workspace::content_digest(&main_source).to_string(),
        "dialect": format.extension(),
        "retained_generated_source": retained_main_name,
        "generated_content_digest": artifact.content_digest().to_string(),
        "generator": artifact.provenance().generator(),
        "input_revision": artifact.provenance().input().revision().get(),
        "input_digest": artifact.provenance().input().digest().to_string(),
        "source_map": include_source_map.then_some("source-map.json"),
        "dependencies": manifest_dependencies,
    });
    entries.push(BundleEntry {
        name: "manifest.json".to_owned(),
        bytes: serde_json::to_vec_pretty(&manifest).map_err(|error| error.to_string())?,
    });
    build_store_zip(entries)
}

fn safe_bundle_file_name(display_name: &str, index: usize) -> String {
    // Treat both separator families as separators on every target, including
    // wasm, so an archive created in a browser cannot encode a Windows path.
    let candidate = display_name.rsplit(['/', '\\']).next().unwrap_or_default();
    let safe = candidate
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
                ch
            } else {
                '_'
            }
        })
        .take(120)
        .collect::<String>();
    if safe.is_empty() || matches!(safe.as_str(), "." | "..") {
        format!("dependency-{index}.inc")
    } else {
        safe
    }
}

fn next_bundle_card_token(line: &str, cursor: &mut usize) -> Option<std::ops::Range<usize>> {
    while *cursor < line.len() {
        let ch = line[*cursor..].chars().next()?;
        if !ch.is_whitespace() {
            break;
        }
        *cursor += ch.len_utf8();
    }
    if *cursor >= line.len() || matches!(line[*cursor..].chars().next()?, ';' | '$') {
        return None;
    }

    let first = line[*cursor..].chars().next()?;
    if matches!(first, '\'' | '"') {
        *cursor += first.len_utf8();
        let start = *cursor;
        while *cursor < line.len() {
            let ch = line[*cursor..].chars().next()?;
            if ch == first {
                let end = *cursor;
                *cursor += ch.len_utf8();
                return Some(start..end);
            }
            *cursor += ch.len_utf8();
        }
        return Some(start..*cursor);
    }

    let start = *cursor;
    while *cursor < line.len() {
        let ch = line[*cursor..].chars().next()?;
        if ch.is_whitespace() || matches!(ch, ';' | '$') {
            break;
        }
        *cursor += ch.len_utf8();
    }
    Some(start..*cursor)
}

fn dependency_locator_span(line: &str) -> Option<std::ops::Range<usize>> {
    let logical_line = line
        .strip_suffix('\n')
        .unwrap_or(line)
        .strip_suffix('\r')
        .unwrap_or_else(|| line.strip_suffix('\n').unwrap_or(line));
    let trimmed = logical_line.trim_start();
    if trimmed.is_empty()
        || trimmed.starts_with('*')
        || trimmed.starts_with(';')
        || trimmed.starts_with("//")
        || trimmed.starts_with('+')
    {
        return None;
    }
    let mut cursor = 0;
    let head = next_bundle_card_token(logical_line, &mut cursor)?;
    let locator = next_bundle_card_token(logical_line, &mut cursor)?;
    let head = logical_line[head].to_ascii_lowercase();
    match head.as_str() {
        ".include" | ".inc" | ".veriloga" => Some(locator),
        ".lib" if next_bundle_card_token(logical_line, &mut cursor).is_some() => Some(locator),
        _ => None,
    }
}

fn rewrite_include_locators(
    source: &str,
    mappings: &[(usize, String, String)],
) -> Result<String, String> {
    let mut indexed_mappings = std::collections::BTreeMap::new();
    for (include_index, requested, entry) in mappings {
        if indexed_mappings
            .insert(*include_index, (requested.as_str(), entry.as_str()))
            .is_some()
        {
            return Err(format!(
                "multiple bundle members resolve dependency directive {include_index}"
            ));
        }
    }
    let mut include_index = 0_usize;
    let mut output = String::with_capacity(source.len());
    for line in source.split_inclusive('\n') {
        let Some(locator_span) = dependency_locator_span(line) else {
            output.push_str(line);
            continue;
        };
        let Some((requested, entry)) = indexed_mappings.remove(&include_index) else {
            return Err(format!(
                "dependency directive {include_index} has no sealed bundle member"
            ));
        };
        let actual = &line[locator_span.clone()];
        if actual != requested {
            return Err(format!(
                "dependency directive {include_index} names '{actual}', not its authenticated locator '{requested}'"
            ));
        }
        output.push_str(&line[..locator_span.start]);
        output.push_str(entry);
        output.push_str(&line[locator_span.end..]);
        include_index += 1;
    }
    if let Some((include_index, _)) = indexed_mappings.first_key_value() {
        return Err(format!(
            "sealed dependency edge {include_index} has no source directive"
        ));
    }
    Ok(output)
}

fn validate_bundle_entry_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("bundle entry name is empty".to_owned());
    }
    if name.starts_with('/')
        || name.contains('\\')
        || name.contains(':')
        || name.chars().any(char::is_control)
        || name
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(format!("unsafe bundle entry name '{name}'"));
    }
    Ok(())
}

fn build_store_zip(mut entries: Vec<BundleEntry>) -> Result<Vec<u8>, String> {
    entries.sort_by(|left, right| left.name.cmp(&right.name));
    let mut names = std::collections::HashSet::with_capacity(entries.len());
    for entry in &entries {
        validate_bundle_entry_name(&entry.name)?;
        if !names.insert(entry.name.as_str()) {
            return Err(format!("duplicate bundle entry name '{}'", entry.name));
        }
    }
    let entry_count = u16::try_from(entries.len()).map_err(|_| "too many bundle entries")?;
    let mut output = Vec::new();
    let mut central = Vec::new();
    for entry in entries {
        let name = entry.name.as_bytes();
        let name_len = u16::try_from(name.len()).map_err(|_| "bundle entry name is too long")?;
        let size = u32::try_from(entry.bytes.len()).map_err(|_| "bundle entry exceeds 4 GiB")?;
        let offset =
            u32::try_from(output.len()).map_err(|_| "bundle exceeds classic ZIP limits")?;
        let crc = crc32(&entry.bytes);
        push_u32(&mut output, 0x0403_4b50);
        push_u16(&mut output, 20);
        push_u16(&mut output, 0x0800);
        push_u16(&mut output, 0);
        push_u16(&mut output, 0);
        push_u16(&mut output, 0x0021);
        push_u32(&mut output, crc);
        push_u32(&mut output, size);
        push_u32(&mut output, size);
        push_u16(&mut output, name_len);
        push_u16(&mut output, 0);
        output.extend_from_slice(name);
        output.extend_from_slice(&entry.bytes);

        push_u32(&mut central, 0x0201_4b50);
        push_u16(&mut central, 20);
        push_u16(&mut central, 20);
        push_u16(&mut central, 0x0800);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0x0021);
        push_u32(&mut central, crc);
        push_u32(&mut central, size);
        push_u32(&mut central, size);
        push_u16(&mut central, name_len);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u32(&mut central, 0);
        push_u32(&mut central, offset);
        central.extend_from_slice(name);
    }
    let central_offset = u32::try_from(output.len()).map_err(|_| "bundle exceeds ZIP limits")?;
    let central_size = u32::try_from(central.len()).map_err(|_| "bundle exceeds ZIP limits")?;
    central_offset
        .checked_add(central_size)
        .ok_or("bundle exceeds classic ZIP limits")?;
    output.extend_from_slice(&central);
    push_u32(&mut output, 0x0605_4b50);
    push_u16(&mut output, 0);
    push_u16(&mut output, 0);
    push_u16(&mut output, entry_count);
    push_u16(&mut output, entry_count);
    push_u32(&mut output, central_size);
    push_u32(&mut output, central_offset);
    push_u16(&mut output, 0);
    Ok(output)
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff_u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xedb8_8320 & mask);
        }
    }
    !crc
}

fn push_u16(output: &mut Vec<u8>, value: u16) {
    output.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn action_view_netlist(state: &mut AppState) {
    if state.schematic.components.is_empty() {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "No circuit to generate netlist. Add components first.",
        ));
        return;
    }

    let Some(netlist_content) = build_menu_netlist(state, crate::io::NetlistFormat::Spice) else {
        return;
    };

    state.simulation.netlist_content = netlist_content.clone();

    let preview_lines: Vec<&str> = netlist_content.lines().take(10).collect();
    let preview = preview_lines.join("\n");
    let total_lines = netlist_content.lines().count();

    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
        "Generated netlist ({} lines):\n{}{}",
        total_lines,
        preview,
        if total_lines > 10 { "\n..." } else { "" }
    )));
}

pub(crate) fn build_menu_netlist(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
) -> Option<String> {
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
        &state.library_manager,
        &state.workspace.schematic_buffers,
    );
    let analysis_instances = state
        .sim_setup
        .analysis_plan
        .as_ref()
        .into_iter()
        .flat_map(|plan| plan.instances())
        .filter(|instance| instance.enabled())
        .map(crate::simulation::plan::AnalysisInstance::id)
        .collect::<Vec<_>>();
    let Some(plan_id) = state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
    else {
        state.push_user_message(crate::common::app::ConsoleMessage::error(
            "Netlist export requires a stable active simulation plan.",
        ));
        return None;
    };
    let Some(plan_payload) = state.workspace.active_plan_data(plan_id) else {
        state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
            "Simulation plan {plan_id} has no plan-owned configuration payload."
        )));
        return None;
    };
    let generation = crate::simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
        &state.schematic,
        &[],
        &hierarchy,
        &plan_payload.design_variables,
        crate::simulation::netlist_gen::DesignVariableNetlistContext {
            active_cell: &state.workspace.active_view,
            analysis_instances: &analysis_instances,
        },
    );

    if !generation.errors.is_empty() {
        for err in generation.errors {
            state.push_user_message(crate::common::app::ConsoleMessage::error(err));
        }
        return None;
    }

    for warning in generation.warnings {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(warning));
    }

    let spice_netlist = generation.netlist;
    Some(match format {
        crate::io::NetlistFormat::Spectre => {
            super::netlist_compat::spice_to_ahdl_compatible_netlist(&spice_netlist)
        }
        _ => spice_netlist,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::path::{Path, PathBuf};

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct SaveDialogConfigSnapshot {
        title: String,
        default_name: String,
        filter_name: String,
        filter_extensions: Vec<String>,
    }

    #[derive(Debug)]
    struct MockExportWorkflowIo {
        dialog_result: Result<Option<PathBuf>, String>,
        write_result: Result<(), String>,
        saved_paths_are_reopenable: bool,
        configs: RefCell<Vec<SaveDialogConfigSnapshot>>,
        writes: RefCell<Vec<(PathBuf, String)>>,
    }

    impl MockExportWorkflowIo {
        fn returning_path(path: impl Into<PathBuf>) -> Self {
            Self {
                dialog_result: Ok(Some(path.into())),
                write_result: Ok(()),
                saved_paths_are_reopenable: true,
                configs: RefCell::default(),
                writes: RefCell::default(),
            }
        }

        fn failing_dialog(message: impl Into<String>) -> Self {
            Self {
                dialog_result: Err(message.into()),
                write_result: Ok(()),
                saved_paths_are_reopenable: true,
                configs: RefCell::default(),
                writes: RefCell::default(),
            }
        }

        fn with_write_error(mut self, message: impl Into<String>) -> Self {
            self.write_result = Err(message.into());
            self
        }

        fn download_only(mut self) -> Self {
            self.saved_paths_are_reopenable = false;
            self
        }
    }

    impl ExportWorkflowIo for MockExportWorkflowIo {
        fn show_save_dialog(
            &self,
            config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            self.configs.borrow_mut().push(SaveDialogConfigSnapshot {
                title: config.title.to_owned(),
                default_name: config.default_name.to_owned(),
                filter_name: config.filter_name.to_owned(),
                filter_extensions: config
                    .filter_extensions
                    .iter()
                    .map(|extension| (*extension).to_owned())
                    .collect(),
            });
            self.dialog_result.clone()
        }

        fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.writes
                .borrow_mut()
                .push((path.to_path_buf(), contents.to_owned()));
            self.write_result.clone()
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Ok(())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            self.saved_paths_are_reopenable
        }
    }

    fn last_log_message(state: &AppState) -> String {
        state
            .log_buffer
            .entries()
            .last()
            .expect("a user-facing log line is emitted")
            .message
            .clone()
    }

    fn stored_zip_entries(zip: &[u8]) -> std::collections::BTreeMap<String, Vec<u8>> {
        fn u16_at(bytes: &[u8], offset: usize) -> u16 {
            u16::from_le_bytes(bytes[offset..offset + 2].try_into().expect("u16 field"))
        }
        fn u32_at(bytes: &[u8], offset: usize) -> u32 {
            u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
        }

        let mut entries = std::collections::BTreeMap::new();
        let mut offset = 0;
        while zip.get(offset..offset + 4) == Some(&0x0403_4b50_u32.to_le_bytes()) {
            assert_eq!(u16_at(zip, offset + 8), 0, "stored compression method");
            assert_eq!(u16_at(zip, offset + 12), 0x0021, "valid 1980-01-01 date");
            let crc = u32_at(zip, offset + 14);
            let compressed_size = u32_at(zip, offset + 18) as usize;
            assert_eq!(compressed_size, u32_at(zip, offset + 22) as usize);
            let name_length = u16_at(zip, offset + 26) as usize;
            let extra_length = u16_at(zip, offset + 28) as usize;
            let name_start = offset + 30;
            let content_start = name_start + name_length + extra_length;
            let content_end = content_start + compressed_size;
            let name = std::str::from_utf8(&zip[name_start..name_start + name_length])
                .expect("UTF-8 entry name")
                .to_owned();
            let contents = zip[content_start..content_end].to_vec();
            assert_eq!(crc, crc32(&contents));
            assert!(entries.insert(name, contents).is_none());
            offset = content_end;
        }
        assert_eq!(
            zip.get(offset..offset + 4),
            Some(0x0201_4b50_u32.to_le_bytes().as_slice())
        );
        entries
    }

    #[test]
    fn svg_export_uses_dialog_defaults_and_writes_svg_file() {
        let mut state = AppState::default();
        state.schematic.current_file = Some(PathBuf::from("designs").join("rc_filter.sch"));
        let io = MockExportWorkflowIo::returning_path(PathBuf::from("exports").join("rc_filter"));

        action_export_svg_with_io(&mut state, &io);

        assert_eq!(
            io.configs.borrow().as_slice(),
            &[SaveDialogConfigSnapshot {
                title: "Export SVG".to_owned(),
                default_name: "rc_filter.svg".to_owned(),
                filter_name: "SVG Image".to_owned(),
                filter_extensions: vec!["svg".to_owned()],
            }]
        );
        let writes = io.writes.borrow();
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0].0, PathBuf::from("exports").join("rc_filter.svg"));
        assert!(writes[0].1.starts_with("<?xml version=\"1.0\""));
        assert!(
            writes[0]
                .1
                .contains("<svg xmlns=\"http://www.w3.org/2000/svg\"")
        );
        assert_eq!(
            last_log_message(&state),
            format!("Exported SVG: {}", writes[0].0.display())
        );
    }

    #[test]
    fn svg_export_reports_save_dialog_errors_without_writing() {
        let mut state = AppState::default();
        let io = MockExportWorkflowIo::failing_dialog("native dialog unavailable");

        action_export_svg_with_io(&mut state, &io);

        assert!(io.writes.borrow().is_empty());
        assert_eq!(
            last_log_message(&state),
            "SVG export failed: native dialog unavailable"
        );
    }

    #[test]
    fn svg_export_propagates_injected_publication_failure_without_claiming_success() {
        let mut state = AppState::default();
        let io = MockExportWorkflowIo::returning_path(PathBuf::from("schematic.svg"))
            .with_write_error("disk full");

        action_export_svg_with_io(&mut state, &io);

        assert_eq!(io.writes.borrow().len(), 1);
        assert_eq!(last_log_message(&state), "SVG export failed: disk full");
    }

    #[test]
    fn svg_export_reports_browser_download_start_without_claiming_file_written() {
        let mut state = AppState::default();
        let io =
            MockExportWorkflowIo::returning_path(PathBuf::from("schematic.svg")).download_only();

        action_export_svg_with_io(&mut state, &io);

        assert_eq!(
            last_log_message(&state),
            "SVG download started: schematic.svg (confirm the browser accepted the download)"
        );
    }

    #[test]
    fn store_zip_uses_standard_crc_and_exact_entry_count() {
        assert_eq!(crc32(b"123456789"), 0xcbf4_3926);
        let zip = build_store_zip(vec![
            BundleEntry {
                name: "a.txt".to_owned(),
                bytes: b"alpha".to_vec(),
            },
            BundleEntry {
                name: "b.txt".to_owned(),
                bytes: b"beta".to_vec(),
            },
        ])
        .expect("valid store zip");
        assert!(zip.starts_with(&0x0403_4b50_u32.to_le_bytes()));
        assert_eq!(
            &zip[zip.len() - 22..zip.len() - 18],
            &0x0605_4b50_u32.to_le_bytes()
        );
        assert_eq!(
            u16::from_le_bytes([zip[zip.len() - 12], zip[zip.len() - 11]]),
            2
        );
        let entries = stored_zip_entries(&zip);
        assert_eq!(
            entries.get("a.txt").map(Vec::as_slice),
            Some(b"alpha".as_slice())
        );
        assert_eq!(
            entries.get("b.txt").map(Vec::as_slice),
            Some(b"beta".as_slice())
        );
    }

    #[test]
    fn store_zip_is_order_deterministic_and_rejects_unsafe_or_duplicate_paths() {
        let first = build_store_zip(vec![
            BundleEntry {
                name: "z.txt".to_owned(),
                bytes: b"z".to_vec(),
            },
            BundleEntry {
                name: "dir/a.txt".to_owned(),
                bytes: b"a".to_vec(),
            },
        ])
        .expect("archive");
        let reversed = build_store_zip(vec![
            BundleEntry {
                name: "dir/a.txt".to_owned(),
                bytes: b"a".to_vec(),
            },
            BundleEntry {
                name: "z.txt".to_owned(),
                bytes: b"z".to_vec(),
            },
        ])
        .expect("archive");
        assert_eq!(first, reversed);

        for unsafe_name in [
            "../escape",
            "/absolute",
            "C:/absolute",
            "a\\b",
            "a//b",
            "a/./b",
        ] {
            let error = build_store_zip(vec![BundleEntry {
                name: unsafe_name.to_owned(),
                bytes: Vec::new(),
            }])
            .expect_err("unsafe archive path");
            assert!(error.contains("unsafe bundle entry"), "{error}");
        }
        let error = build_store_zip(vec![
            BundleEntry {
                name: "same".to_owned(),
                bytes: vec![1],
            },
            BundleEntry {
                name: "same".to_owned(),
                bytes: vec![2],
            },
        ])
        .expect_err("duplicate path");
        assert!(error.contains("duplicate bundle entry"));
    }

    #[test]
    fn include_rewrite_uses_exact_locator_token_and_ignores_local_lib_sections() {
        let source = "deck\r\n.lib LOCAL\r\n.include \"a.lib\" $ a.lib remains in comment\r\n.veriloga 'device.va'\r\n.lib \"corner.lib\" TT\r\n.end\r\n";
        let rewritten = rewrite_include_locators(
            source,
            &[
                (0, "a.lib".to_owned(), "sources/000-a.lib".to_owned()),
                (
                    1,
                    "device.va".to_owned(),
                    "sources/001-device.va".to_owned(),
                ),
                (
                    2,
                    "corner.lib".to_owned(),
                    "sources/002-corner.lib".to_owned(),
                ),
            ],
        )
        .expect("exact rewrites");
        assert_eq!(
            rewritten,
            "deck\r\n.lib LOCAL\r\n.include \"sources/000-a.lib\" $ a.lib remains in comment\r\n.veriloga 'sources/001-device.va'\r\n.lib \"sources/002-corner.lib\" TT\r\n.end\r\n"
        );

        let error = rewrite_include_locators(source, &[])
            .expect_err("an external dependency may not leak from a bundle");
        assert!(error.contains("no sealed bundle member"));
    }

    #[test]
    fn non_spectre_generated_exports_preserve_exact_source_bytes() {
        let source = "title Î¼\r\nR1 out 0 1k\r\n.end\r\n";
        for format in [
            crate::io::NetlistFormat::Spice,
            crate::io::NetlistFormat::Hspice,
            crate::io::NetlistFormat::Xyce,
        ] {
            assert_eq!(
                generated_export_source(source, format).as_bytes(),
                source.as_bytes()
            );
        }
    }

    #[test]
    fn generated_bundle_rewrites_and_embeds_sealed_dependency_and_source_map() {
        use crate::product::ObjectRevision;
        use crate::workbench::code_workspace::{
            DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry,
            GenerationInput, SourceLocator, content_digest,
        };

        let dependency = DependencyMetadata::unresolved_direct(
            0,
            SourceLocator::try_new("models/device.lib", "device.lib").expect("locator"),
        )
        .resolve_utf8(b".model ideal r r=1k\n".to_vec())
        .expect("resolved dependency");
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "test-generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest("inputs")),
            )
            .expect("provenance"),
            b"deck\n.include \"models/device.lib\"\nR1 out 0 ideal\n.op\n.end\n".to_vec(),
            vec![dependency],
            (1..=5)
                .map(|line| {
                    GeneratedSourceMapEntry::try_new(
                        line,
                        "user/top",
                        "user/top/schematic",
                        None,
                        None,
                    )
                    .expect("source map")
                })
                .collect(),
        )
        .expect("artifact");

        let zip = build_generated_bundle(&artifact, crate::io::NetlistFormat::Spice, true)
            .expect("bundle");
        let entries = stored_zip_entries(&zip);
        assert_eq!(
            entries.get("generated.spice").map(Vec::as_slice),
            Some(
                b"deck\n.include \"sources/000-device.lib\"\nR1 out 0 ideal\n.op\n.end\n"
                    .as_slice()
            )
        );
        assert_eq!(
            entries.get("retained/generated.spice").map(Vec::as_slice),
            Some(artifact.source_bytes())
        );
        assert_eq!(
            entries.get("sources/000-device.lib").map(Vec::as_slice),
            Some(b".model ideal r r=1k\n".as_slice())
        );
        assert!(entries.contains_key("retained/sources/000-device.lib"));
        let source_map: serde_json::Value =
            serde_json::from_slice(entries.get("source-map.json").expect("source map entry"))
                .expect("source map JSON");
        assert_eq!(source_map["source"], "retained/generated.spice");
        assert_eq!(source_map["entries"].as_array().map(Vec::len), Some(5));
        assert!(entries.contains_key("manifest.json"));
    }

    #[test]
    fn generated_bundle_rewrites_transitive_edges_relative_to_the_parent_directory() {
        use crate::product::ObjectRevision;
        use crate::workbench::code_workspace::{
            DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput,
            SourceLocator, content_digest,
        };

        let parent_locator = SourceLocator::try_new("resolved/a.lib", "a.lib").expect("parent");
        let child_locator = SourceLocator::try_new("resolved/b.lib", "b.lib").expect("child");
        let parent_source = ".include \"../shared/b.lib\"\r\n.model a r r=1k\r\n";
        let child_source = ".model b r r=2k\r\n";
        let direct =
            DependencyMetadata::unresolved_direct_to(0, "../vendor/a.lib", parent_locator.clone())
                .expect("direct edge")
                .resolve_utf8(parent_source.as_bytes().to_vec())
                .expect("parent source");
        let child = DependencyMetadata::unresolved_transitive_to(
            parent_locator,
            0,
            "../shared/b.lib",
            child_locator,
        )
        .expect("child edge")
        .resolve_utf8(child_source.as_bytes().to_vec())
        .expect("child source");
        let root = "deck\r\n.lib LOCAL\r\n.include \"../vendor/a.lib\"\r\n.end\r\n";
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "test-generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest("inputs")),
            )
            .expect("provenance"),
            root.as_bytes().to_vec(),
            // Deliberately reverse graph order; archive paths are canonical.
            vec![child, direct],
            Vec::new(),
        )
        .expect("artifact");

        let zip = build_generated_bundle(&artifact, crate::io::NetlistFormat::Spice, false)
            .expect("bundle");
        let entries = stored_zip_entries(&zip);
        assert_eq!(
            entries.get("generated.spice").map(Vec::as_slice),
            Some(b"deck\r\n.lib LOCAL\r\n.include \"sources/000-a.lib\"\r\n.end\r\n".as_slice())
        );
        assert_eq!(
            entries.get("sources/000-a.lib").map(Vec::as_slice),
            Some(b".include \"001-b.lib\"\r\n.model a r r=1k\r\n".as_slice())
        );
        assert_eq!(
            entries.get("sources/001-b.lib").map(Vec::as_slice),
            Some(child_source.as_bytes())
        );
        assert_eq!(
            entries.get("retained/generated.spice").map(Vec::as_slice),
            Some(root.as_bytes())
        );
        assert_eq!(
            entries.get("retained/sources/000-a.lib").map(Vec::as_slice),
            Some(parent_source.as_bytes())
        );

        let manifest: serde_json::Value =
            serde_json::from_slice(entries.get("manifest.json").expect("manifest"))
                .expect("manifest JSON");
        assert_eq!(manifest["dependencies"][0]["edge"]["owner"], "generated");
        assert_eq!(
            manifest["dependencies"][1]["edge"]["owner"],
            "resolved/a.lib"
        );
        assert_ne!(
            manifest["dependencies"][0]["content_digest"],
            manifest["dependencies"][0]["retained_content_digest"]
        );
    }

    #[test]
    fn translated_bundle_rejects_generated_source_map() {
        use crate::product::ObjectRevision;
        use crate::workbench::code_workspace::{
            GeneratedArtifact, GeneratedProvenance, GenerationInput, content_digest,
        };
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "test-generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest("inputs")),
            )
            .expect("provenance"),
            b"deck\n.end\n".to_vec(),
            Vec::new(),
            Vec::new(),
        )
        .expect("artifact");
        let error = build_generated_bundle(&artifact, crate::io::NetlistFormat::Spectre, true)
            .expect_err("translated coordinates are not exact");
        assert!(error.contains("exact SPICE"));
    }

    #[test]
    fn generated_bundle_fails_closed_for_unresolved_dependencies() {
        use crate::product::ObjectRevision;
        use crate::workbench::code_workspace::{
            DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput,
            SourceLocator, content_digest,
        };
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "test-generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest("inputs")),
            )
            .expect("provenance"),
            b"deck\n.veriloga model.va\n.end\n".to_vec(),
            vec![DependencyMetadata::unresolved_direct(
                0,
                SourceLocator::try_new("model.va", "model.va").expect("locator"),
            )],
            Vec::new(),
        )
        .expect("artifact");

        let error = build_generated_bundle(&artifact, crate::io::NetlistFormat::Spice, false)
            .expect_err("unresolved dependency must block bundle");
        assert!(error.contains("unresolved"));
    }
}
