//! What the Project > Configuration page keeps between frames: which
//! configuration is in front of the reader, and the refusal the last
//! transaction on it stated.
//!
//! The project's configuration catalog stays the authority. Nothing here is
//! saved with the project, and nothing here changes what a run executes.

use crate::state::{ConfigurationSetCatalog, ConfigurationSetId};

/// The configuration the page shows. `Default` is the built-in search order
/// a project runs with when no configuration is used for runs; it is not a
/// catalog entry and can be neither renamed nor deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ConfigurationSelection {
    Default,
    Configuration(ConfigurationSetId),
}

/// The Configuration page's own state.
#[derive(Debug, Clone, Default)]
pub(crate) struct ConfigurationPageState {
    /// What the reader picked. `None` follows whatever is used for runs, so
    /// a project opens on the configuration it netlists with.
    selection: Option<ConfigurationSelection>,
    /// Why the last apply, activation or deletion was refused. It stays on
    /// the page until the next transaction or a new selection.
    pub(crate) refusal: Option<String>,
}

impl ConfigurationPageState {
    /// The configuration in front of the reader. A picked configuration that
    /// no longer exists falls back to the one used for runs.
    pub(crate) fn selection(&self, catalog: &ConfigurationSetCatalog) -> ConfigurationSelection {
        match self.selection {
            Some(ConfigurationSelection::Default) => ConfigurationSelection::Default,
            Some(ConfigurationSelection::Configuration(id)) if catalog.find(id).is_some() => {
                ConfigurationSelection::Configuration(id)
            }
            _ => catalog
                .active_configuration_id()
                .map_or(ConfigurationSelection::Default, |id| {
                    ConfigurationSelection::Configuration(id)
                }),
        }
    }

    /// Put `selection` in front of the reader. A refusal belongs to what it
    /// refused, so a new selection clears it.
    pub(crate) fn select(&mut self, selection: ConfigurationSelection) {
        if self.selection != Some(selection) {
            self.refusal = None;
        }
        self.selection = Some(selection);
    }
}

/// Whether `selection` is what runs use today.
pub(crate) fn used_for_runs(
    catalog: &ConfigurationSetCatalog,
    selection: ConfigurationSelection,
) -> bool {
    match selection {
        ConfigurationSelection::Default => catalog.active_configuration_id().is_none(),
        ConfigurationSelection::Configuration(id) => catalog.active_configuration_id() == Some(id),
    }
}

/// The built-in default the design model resolves with when no configuration
/// is used for runs, and the return to it. These stand in for the design
/// model's own items until it publishes them; each is replaced by its
/// re-export, one line apiece.
pub(crate) mod built_in {
    use crate::state::{ConfigurationSetCatalog, ViewType};

    /// View types the built-in order stops descending at.
    pub(crate) const DEFAULT_STOP_VIEW_TYPES: [ViewType; 4] = [
        ViewType::Spice,
        ViewType::Verilog,
        ViewType::VerilogA,
        ViewType::Extracted,
    ];

    /// The built-in view search order for an instance that asks for
    /// `requested`.
    pub(crate) fn default_view_search_order(requested: &str, is_root: bool) -> Vec<String> {
        let requested = if requested.eq_ignore_ascii_case("symbol") {
            "schematic"
        } else {
            requested
        };
        let mut order = vec![requested.to_owned()];
        match ViewType::from_name(requested) {
            ViewType::Schematic | ViewType::Testbench if !is_root => {
                order.push("extracted".to_owned());
                order.push("spice".to_owned());
            }
            ViewType::Spice => {}
            _ => order.push("spice".to_owned()),
        }
        order.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
        order
    }

    /// Stop using any configuration for runs: netlisting returns to the
    /// built-in order. No configuration is revised.
    pub(crate) fn deactivate(catalog: &mut ConfigurationSetCatalog) -> Result<(), String> {
        if catalog.active_configuration_id().is_none() {
            return Ok(());
        }
        let mut wire = serde_json::to_value(&*catalog).map_err(|error| error.to_string())?;
        wire["active_configuration_id"] = serde_json::Value::Null;
        *catalog = serde_json::from_value(wire).map_err(|error| error.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
