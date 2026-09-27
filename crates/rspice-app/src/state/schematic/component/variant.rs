//! Checked, owned variant replacements for an immutable execution projection.

use super::{Component, ComponentType, LibraryCellInstance, Point, Rotation};
use crate::state::{PortSpec, ResolvedCellSymbol, ViewType};

/// A source pin contract validated before resolving its replacement master.
pub(crate) struct VariantSource<'a> {
    source: &'a Component,
    symbol: Option<&'a ResolvedCellSymbol>,
}

impl Component {
    pub(crate) fn variant_source<'a>(
        &'a self,
        source_symbol: Option<&'a ResolvedCellSymbol>,
    ) -> Result<VariantSource<'a>, String> {
        let source = self;
        if source.kind == ComponentType::CellInstance
            && source_symbol.is_none()
            && source
                .library_cell
                .as_ref()
                .is_none_or(|binding| binding.terminal_order.is_empty())
        {
            return Err("source instance has no resolved pin contract".to_owned());
        }
        if let Some(symbol) = &source_symbol
            && !symbol.issues().is_empty()
        {
            return Err(format!(
                "source symbol has invalid pin metadata: {:?}",
                symbol.issues()
            ));
        }
        Ok(VariantSource {
            source,
            symbol: source_symbol,
        })
    }

    /// Read the local terminal geometry retained by this execution projection.
    pub(crate) fn execution_terminal_layout(&self) -> Option<&[(String, Point)]> {
        self.execution_terminal_layout.as_deref()
    }
}

impl VariantSource<'_> {
    /// Build a replacement without changing its projected occurrence. Callers
    /// collect every successful candidate before publishing the batch.
    pub(crate) fn prepare_replacement(
        self,
        projected: &Component,
        mut target_binding: LibraryCellInstance,
        target_symbol: &ResolvedCellSymbol,
    ) -> Result<Component, String> {
        let source = self.source;
        let source_symbol = self.symbol;
        if !target_symbol.issues().is_empty() {
            return Err(format!(
                "replacement symbol has invalid pin metadata: {:?}",
                target_symbol.issues()
            ));
        }
        // Resolve local offsets, so the projected occurrence applies its
        // own rotation, mirrors and sheet translation exactly once.
        let mut local_source = source.clone();
        local_source.pos = Point::origin();
        local_source.rotation = Rotation::R0;
        local_source.mirror_h = false;
        local_source.mirror_v = false;
        local_source.execution_terminal_layout = None;
        let mut source_pins = local_source.terminal_positions_resolved(source_symbol);
        if source_symbol.is_none()
            && let Some(binding) = &source.library_cell
            && binding.terminal_order.len() == source_pins.len()
        {
            for ((name, _), bound) in source_pins.iter_mut().zip(&binding.terminal_order) {
                name.clone_from(bound);
            }
        }
        let target_pins = target_symbol.connectable_pins().collect::<Vec<_>>();
        if source_pins.len() != target_pins.len() {
            return Err(format!(
                "source has {} terminals but replacement has {}",
                source_pins.len(),
                target_pins.len()
            ));
        }
        let named = source.kind == ComponentType::CellInstance
            && (source_symbol.is_some()
                || source
                    .library_cell
                    .as_ref()
                    .is_some_and(|binding| !binding.terminal_order.is_empty()));
        let mut used = std::collections::HashSet::new();
        let mut target_names = std::collections::HashSet::new();
        let mut layout = Vec::with_capacity(target_pins.len());
        let mut ports = Vec::with_capacity(target_pins.len());
        for (index, pin) in target_pins.into_iter().enumerate() {
            let source_index = if named {
                source_pins
                    .iter()
                    .position(|(name, _)| name.eq_ignore_ascii_case(&pin.name))
                    .ok_or_else(|| {
                        format!(
                            "replacement terminal '{}' has no matching source terminal",
                            pin.name
                        )
                    })?
            } else {
                index
            };
            let (source_name, offset) = &source_pins[source_index];
            if !used.insert(source_index) || !target_names.insert(pin.name.to_ascii_lowercase()) {
                return Err("terminal mapping is not one-to-one".to_owned());
            }
            if crate::state::declared_width(source_name) != crate::state::declared_width(&pin.name)
            {
                return Err(format!("terminal '{}' changes conductor width", pin.name));
            }
            layout.push((pin.name.clone(), *offset));
            ports.push(PortSpec {
                name: pin.name.clone(),
                direction: pin.direction,
            });
        }
        target_binding.bind_interface(&ports);
        let mut replacement = projected.clone();
        replacement.library_cell = Some(target_binding);
        replacement.execution_terminal_layout = Some(layout);
        Ok(replacement)
    }
}

impl LibraryCellInstance {
    /// Bind a variant section after checking the resolved view's source kind.
    /// An absent view retains the resolver's existing fallback behavior.
    pub(crate) fn with_variant_model_section(
        mut self,
        section: Option<&str>,
        view_type: Option<ViewType>,
    ) -> Result<Self, String> {
        if section.is_some()
            && view_type.is_some_and(|view_type| {
                !matches!(view_type, ViewType::Spice | ViewType::Extracted)
            })
        {
            return Err(
                "a model section requires a source-backed SPICE or extracted view".to_owned(),
            );
        }
        self.variant_model_section = section.map(str::to_owned);
        if section.is_some() {
            self.model_section = section.map(str::to_owned);
        }
        Ok(self)
    }

    /// Carry the checked variant section through source materialization.
    pub(crate) fn inherit_variant_model_section(&mut self, placed: &Self) {
        self.variant_model_section
            .clone_from(&placed.variant_model_section);
    }

    pub(crate) fn variant_model_section(&self) -> Option<&str> {
        self.variant_model_section.as_deref()
    }
}
