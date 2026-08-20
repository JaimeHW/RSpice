//! Corner transactions: add, edit, duplicate, delete, bind, and unbind.
//!
//! These are the commit halves of the corner dialogs and rows. Each one
//! validates its input, mutates a clone of the model-library manager, and
//! publishes that candidate as a single guarded project revision, so a
//! rejected edit leaves the retained library exactly as it was and every
//! accepted one is undoable in a single step.

use super::*;

pub(super) fn add_corner(
    app: &mut ManagerRenderContext<'_>,
    library_name: &str,
    name: &str,
    temperature_c: &str,
    supply_factor: &str,
) {
    let name = name.trim();
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        receipt(
            app,
            Err("Corner name must use ASCII letters, digits, or underscore.".to_owned()),
        );
        return;
    }
    let temperature = match temperature_c.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && (-273.15..=1000.0).contains(&value) => value,
        _ => {
            receipt(
                app,
                Err("Corner temperature must be finite and at least absolute zero.".to_owned()),
            );
            return;
        }
    };
    let supply = match supply_factor.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && value > 0.0 && value <= 10.0 => value,
        _ => {
            receipt(
                app,
                Err(
                    "Supply factor must be a finite value greater than 0 and no more than 10."
                        .to_owned(),
                ),
            );
            return;
        }
    };
    let Some(source) = app
        .state
        .model_library_manager
        .get_library(library_name)
        .cloned()
    else {
        receipt(
            app,
            Err(format!("Library '{library_name}' no longer exists.")),
        );
        return;
    };
    let Some(root_path) = source.root_path.clone() else {
        receipt(
            app,
            Err(format!(
                "Library '{library_name}' has no authenticated source root to bind a corner to."
            )),
        );
        return;
    };
    if source
        .corners
        .keys()
        .any(|existing| existing.eq_ignore_ascii_case(name))
    {
        receipt(
            app,
            Err(format!(
                "Corner '{name}' already exists in library '{library_name}'."
            )),
        );
        return;
    }
    let mut candidate = app.state.model_library_manager.clone();
    let library = candidate
        .get_library_mut(library_name)
        .expect("the source library was resolved above");
    let mut corner = ProcessCorner::new(name);
    corner.description = format!("Project corner {name}");
    corner.nmos_corner = name.to_owned();
    corner.pmos_corner = name.to_owned();
    corner.temperature = temperature;
    corner.vdd_factor = supply;
    corner.file_path = Some(root_path);
    corner.required_domains = vec![CornerSectionDomain::Composite];
    corner.is_default = library.corners.is_empty();
    if let Err(errors) = corner.validate_draft_contract() {
        receipt(
            app,
            Err(format!("Corner draft is invalid: {}", errors.join("; "))),
        );
        return;
    }
    let select_after_insert = corner.is_default;
    library.corners.insert(name.to_owned(), corner);
    if select_after_insert {
        library.select_corner(name);
    }
    let result = publish_model_library_candidate(
        app.state,
        candidate,
        library_name,
        format!("add model corner {name}"),
    )
    .map(|revision| {
        format!(
            "Added unbound corner draft '{name}' to '{library_name}' at project revision {}. Bind an authenticated section before execution.",
            revision.get()
        )
    });
    receipt(app, result);
}

pub(super) fn edit_corner(
    app: &mut ManagerRenderContext<'_>,
    library_name: &str,
    original_name: &str,
    duplicate: bool,
    name: &str,
    description: &str,
    nmos_corner: &str,
    pmos_corner: &str,
    temperature_c: &str,
    supply_factor: &str,
    minimum_temperature_c: &str,
    maximum_temperature_c: &str,
    required_domains: &[CornerSectionDomain],
    make_default: bool,
) {
    let name = match validated_corner_name(name) {
        Ok(name) => name,
        Err(error) => {
            receipt(app, Err(error));
            return;
        }
    };
    let temperature = match parse_corner_temperature("Nominal temperature", temperature_c) {
        Ok(value) => value,
        Err(error) => {
            receipt(app, Err(error));
            return;
        }
    };
    let supply = match supply_factor.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && value > 0.0 && value <= 10.0 => value,
        _ => {
            receipt(
                app,
                Err(
                    "Supply factor must be a finite value greater than 0 and no more than 10."
                        .to_owned(),
                ),
            );
            return;
        }
    };
    let (minimum, maximum) =
        match parse_corner_temperature_range(minimum_temperature_c, maximum_temperature_c) {
            Ok(range) => range,
            Err(error) => {
                receipt(app, Err(error));
                return;
            }
        };
    if nmos_corner.trim().is_empty() || pmos_corner.trim().is_empty() {
        receipt(
            app,
            Err("NMOS and PMOS corner-axis names cannot be empty.".to_owned()),
        );
        return;
    }

    let mut candidate = app.state.model_library_manager.clone();
    let Some(library) = candidate.get_library_mut(library_name) else {
        receipt(
            app,
            Err(format!("Library '{library_name}' no longer exists.")),
        );
        return;
    };
    let Some(original_key) = library
        .corners
        .keys()
        .find(|key| key.eq_ignore_ascii_case(original_name))
        .cloned()
    else {
        receipt(
            app,
            Err(format!(
                "Corner '{original_name}' no longer exists in '{library_name}'."
            )),
        );
        return;
    };
    if library.corners.keys().any(|existing| {
        existing.eq_ignore_ascii_case(name) && (duplicate || existing != &original_key)
    }) {
        receipt(
            app,
            Err(format!(
                "Corner '{name}' already exists in library '{library_name}'."
            )),
        );
        return;
    }

    let mut corner = library
        .corners
        .get(&original_key)
        .expect("the corner key was resolved above")
        .clone();
    let selected_original = library
        .selected_corner
        .as_deref()
        .is_some_and(|selected| selected.eq_ignore_ascii_case(&original_key));
    if !duplicate {
        library.corners.remove(&original_key);
    }
    // Persist the legacy implicit binding before declaring explicit required
    // domains, otherwise editing an old project would silently unbind it.
    corner.section_bindings = corner.effective_section_bindings();
    corner.name = name.to_owned();
    corner.description = description.trim().to_owned();
    corner.nmos_corner = nmos_corner.trim().to_owned();
    corner.pmos_corner = pmos_corner.trim().to_owned();
    corner.temperature = temperature;
    corner.vdd_factor = supply;
    corner.minimum_temperature_c = minimum;
    corner.maximum_temperature_c = maximum;
    corner.required_domains = required_domains.to_vec();
    corner.required_domains.sort();
    corner.required_domains.dedup();
    corner.is_default = false;

    let select_after_insert = make_default || (selected_original && !duplicate);
    if make_default {
        for existing in library.corners.values_mut() {
            existing.is_default = false;
        }
        corner.is_default = true;
    } else if !duplicate
        && library
            .corners
            .values()
            .all(|existing| !existing.is_default)
    {
        let replacement = library.corners.keys().min().cloned();
        if let Some(replacement) = replacement {
            if let Some(existing) = library.corners.get_mut(&replacement) {
                existing.is_default = true;
            }
        } else {
            corner.is_default = true;
        }
    }
    if let Err(errors) = corner.validate_draft_contract() {
        receipt(
            app,
            Err(format!("Corner draft is invalid: {}", errors.join("; "))),
        );
        return;
    }
    let executable = corner.validate_contract().is_ok();
    library.corners.insert(name.to_owned(), corner);
    if select_after_insert {
        library.select_corner(name);
    }
    let action = if duplicate { "duplicate" } else { "edit" };
    let result = publish_model_library_candidate(
        app.state,
        candidate,
        library_name,
        format!("{action} model corner {name}"),
    )
    .map(|revision| {
        format!(
            "{} corner '{name}' in '{library_name}' at project revision {} ({}).",
            if duplicate { "Duplicated" } else { "Updated" },
            revision.get(),
            if executable {
                "executable contract"
            } else {
                "retained non-executable draft"
            }
        )
    });
    receipt(app, result);
}

fn validated_corner_name(name: &str) -> Result<&str, String> {
    let name = name.trim();
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        Err("Corner name must use ASCII letters, digits, or underscore.".to_owned())
    } else {
        Ok(name)
    }
}

fn parse_corner_temperature(label: &str, value: &str) -> Result<f64, String> {
    match value.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && (-273.15..=1000.0).contains(&value) => Ok(value),
        _ => Err(format!(
            "{label} must be finite and between -273.15 °C and 1000 °C."
        )),
    }
}

fn parse_corner_temperature_range(
    minimum: &str,
    maximum: &str,
) -> Result<(Option<f64>, Option<f64>), String> {
    match (minimum.trim(), maximum.trim()) {
        ("", "") => Ok((None, None)),
        ("", _) | (_, "") => {
            Err("Qualified temperature range requires both minimum and maximum.".to_owned())
        }
        (minimum, maximum) => {
            let minimum = parse_corner_temperature("Minimum qualified temperature", minimum)?;
            let maximum = parse_corner_temperature("Maximum qualified temperature", maximum)?;
            if minimum > maximum {
                Err("Minimum qualified temperature exceeds the maximum.".to_owned())
            } else {
                Ok((Some(minimum), Some(maximum)))
            }
        }
    }
}

pub(super) fn set_default_corner(
    app: &mut ManagerRenderContext<'_>,
    library_name: &str,
    corner_name: &str,
) {
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate
        .get_library_mut(library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))
        .and_then(|library| {
            let key = library
                .corners
                .keys()
                .find(|key| key.eq_ignore_ascii_case(corner_name))
                .cloned()
                .ok_or_else(|| {
                    format!("Corner '{corner_name}' no longer exists in '{library_name}'.")
                })?;
            for corner in library.corners.values_mut() {
                corner.is_default = false;
            }
            library
                .corners
                .get_mut(&key)
                .expect("the corner key was resolved above")
                .is_default = true;
            Ok(())
        })
        .and_then(|()| {
            publish_model_library_candidate(
                app.state,
                candidate,
                library_name,
                format!("set default model corner {corner_name}"),
            )
        })
        .map(|revision| {
            format!(
                "Set '{library_name}/{corner_name}' as the default at project revision {}.",
                revision.get()
            )
        });
    receipt(app, result);
}

pub(super) fn delete_corner(
    app: &mut ManagerRenderContext<'_>,
    library_name: &str,
    corner_name: &str,
) {
    let mut candidate = app.state.model_library_manager.clone();
    let Some(library) = candidate.get_library_mut(library_name) else {
        receipt(
            app,
            Err(format!("Library '{library_name}' no longer exists.")),
        );
        return;
    };
    let Some(key) = library
        .corners
        .keys()
        .find(|key| key.eq_ignore_ascii_case(corner_name))
        .cloned()
    else {
        receipt(
            app,
            Err(format!(
                "Corner '{corner_name}' no longer exists in '{library_name}'."
            )),
        );
        return;
    };
    library
        .corners
        .remove(&key)
        .expect("the corner key was resolved above");
    let replacement = library.corners.keys().min().cloned();
    if !library.corners.values().any(|corner| corner.is_default)
        && let Some(replacement) = replacement.as_ref()
    {
        library
            .corners
            .get_mut(replacement)
            .expect("the replacement key came from this map")
            .is_default = true;
    }
    if library
        .selected_corner
        .as_deref()
        .is_some_and(|selected| selected.eq_ignore_ascii_case(&key))
    {
        let selected_corner = library
            .corners
            .iter()
            .find_map(|(name, corner)| corner.is_default.then(|| name.clone()))
            .or(replacement);
        if let Some(selected_corner) = selected_corner {
            library.select_corner(&selected_corner);
        } else {
            library.selected_corner = None;
            library.refresh_effective_model_projection();
        }
    }
    let result = publish_model_library_candidate(
        app.state,
        candidate,
        library_name,
        format!("delete model corner {corner_name}"),
    )
    .map(|revision| {
        format!(
            "Deleted corner '{corner_name}' from '{library_name}' at project revision {}.",
            revision.get()
        )
    });
    receipt(app, result);
}

pub(super) fn unbind_corner_section(
    app: &mut ManagerRenderContext<'_>,
    library_name: &str,
    corner_name: &str,
    domain: CornerSectionDomain,
) {
    let mut candidate = app.state.model_library_manager.clone();
    let Some(library) = candidate.get_library_mut(library_name) else {
        receipt(
            app,
            Err(format!("Library '{library_name}' no longer exists.")),
        );
        return;
    };
    let Some(corner) = library
        .corners
        .values_mut()
        .find(|candidate| candidate.name.eq_ignore_ascii_case(corner_name))
    else {
        receipt(
            app,
            Err(format!(
                "Corner '{corner_name}' no longer exists in '{library_name}'."
            )),
        );
        return;
    };
    let mut bindings = corner.effective_section_bindings();
    if !bindings.iter().any(|binding| binding.domain == domain) {
        receipt(
            app,
            Err(format!(
                "{} is not bound for '{library_name}/{corner_name}'.",
                domain.label()
            )),
        );
        return;
    }
    let mut required = corner.effective_required_domains();
    if !required.contains(&domain) {
        required.push(domain);
    }
    bindings.retain(|binding| binding.domain != domain);
    corner.section_bindings = bindings;
    corner.required_domains = required;
    corner.required_domains.sort();
    corner.required_domains.dedup();
    if let Err(errors) = corner.validate_draft_contract() {
        receipt(
            app,
            Err(format!("Corner draft is invalid: {}", errors.join("; "))),
        );
        return;
    }
    let result = publish_model_library_candidate(
        app.state,
        candidate,
        library_name,
        format!("unbind {domain:?} section for corner {corner_name}"),
    )
    .map(|revision| {
        format!(
            "Unbound {} for '{library_name}/{corner_name}' at project revision {}; the retained draft now fails closed until rebound.",
            domain.label(),
            revision.get()
        )
    });
    receipt(app, result);
}

pub(super) fn bind_corner_section(
    app: &mut ManagerRenderContext<'_>,
    library_name: &str,
    corner_name: &str,
    domain: CornerSectionDomain,
    section: &str,
) {
    let section = section.trim();
    let mut candidate = app.state.model_library_manager.clone();
    let Some(library) = candidate.get_library_mut(library_name) else {
        receipt(
            app,
            Err(format!("Library '{library_name}' no longer exists.")),
        );
        return;
    };
    if !library.defines_section(section) {
        receipt(
            app,
            Err(format!(
                "Authenticated library '{library_name}' defines no non-empty section named '{section}'."
            )),
        );
        return;
    }
    let source_path = library.root_path.clone();
    let Some(corner) = library
        .corners
        .values_mut()
        .find(|candidate| candidate.name.eq_ignore_ascii_case(corner_name))
    else {
        receipt(
            app,
            Err(format!(
                "Corner '{corner_name}' no longer exists in '{library_name}'."
            )),
        );
        return;
    };
    corner.file_path = source_path;
    if let Some(binding) = corner
        .section_bindings
        .iter_mut()
        .find(|binding| binding.domain == domain)
    {
        binding.section = section.to_owned();
    } else {
        corner
            .section_bindings
            .push(CornerSectionBinding::new(domain, section));
    }
    if !corner.required_domains.contains(&domain) {
        corner.required_domains.push(domain);
    }
    if let Err(errors) = corner.validate_draft_contract() {
        receipt(
            app,
            Err(format!(
                "Corner section binding is invalid: {}",
                errors.join("; ")
            )),
        );
        return;
    }
    let result = publish_model_library_candidate(
        app.state,
        candidate,
        library_name,
        format!("bind {domain:?} section {section} for corner {corner_name}"),
    )
    .map(|revision| {
        format!(
            "Bound {} to section '{section}' for '{library_name}/{corner_name}' at project revision {}.",
            domain.label(),
            revision.get()
        )
    });
    receipt(app, result);
}
