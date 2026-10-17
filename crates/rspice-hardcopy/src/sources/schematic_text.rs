//! Where a printed schematic letters its text, in world units.
//!
//! Every position comes from the canvas's own layout
//! (`rspice_design::schematic::label_layout`), so a name the canvas sets
//! clear of a part's body is set clear of it on paper too. The scene compiler
//! draws these runs and the content bounds hold them, so a label is never cut
//! off at the page's edge.

use rspice_design::schematic::{
    bus::{Bus, BusTap},
    component_display::SchematicParameterLabelVisibility,
    component_type::ComponentType,
    label_layout::{self, LayoutPoint, LayoutRect, PlacedText},
    value_label::{cell_instance_value, component_value_label},
};

use super::SemanticComponent;

/// The type a run is set in: D1 section 11's text classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchematicTextClass {
    /// An instance name: Plex Sans, 10 wu.
    Name,
    /// An instance value: Plex Sans, 9 wu.
    Value,
    /// A bus declaration or a tap's bit slice: Plex Mono Medium, 9 wu.
    Slice,
}

impl SchematicTextClass {
    /// The class's type size, in world units.
    pub const fn size_world(self) -> f32 {
        match self {
            Self::Name => label_layout::INSTANCE_NAME_WORLD,
            Self::Value => label_layout::INSTANCE_VALUE_WORLD,
            Self::Slice => label_layout::SMALL_TEXT_WORLD,
        }
    }
}

/// One run of schematic text as the page sets it. A run of several lines
/// stacks them, as the canvas does.
#[derive(Debug, Clone, PartialEq)]
pub struct SchematicText {
    pub class: SchematicTextClass,
    pub placed: PlacedText,
    pub text: String,
}

impl SchematicText {
    /// The box the run takes, in world units.
    pub fn bounds(&self) -> LayoutRect {
        label_layout::text_box(self.placed, &self.text, self.class.size_world())
    }
}

/// An instance's name and value where the canvas letters them, upright for
/// every turn and mirror: an authored cell's at its symbol's anchors, any
/// other part's clear of its turned body. Ground has none, and a name or
/// value the instance hides stays hidden.
pub fn instance_texts(semantic: &SemanticComponent) -> Vec<SchematicText> {
    let component = &semantic.component;
    if component.kind == ComponentType::Ground {
        return Vec::new();
    }
    let (name, value, value_text) = match &semantic.resolved_symbol {
        Some(document) => (
            label_layout::symbol_label_placement(component, document, document.name_anchor),
            label_layout::symbol_label_placement(component, document, document.value_anchor),
            cell_instance_value(component).to_owned(),
        ),
        None => {
            let layout = label_layout::instance_label_layout(
                LayoutPoint::new(component.pos.x as f32, component.pos.y as f32),
                1.0,
                component,
            );
            (
                layout.name,
                layout.value,
                component_value_label(component).into_owned(),
            )
        }
    };
    let visibility = SchematicParameterLabelVisibility::NamesAndValues;
    let mut texts = Vec::with_capacity(2);
    if component.display_mode.show_name(visibility) && !component.name.trim().is_empty() {
        texts.push(SchematicText {
            class: SchematicTextClass::Name,
            placed: name,
            text: component.name.clone(),
        });
    }
    if component.display_mode.show_value(visibility) && !value_text.trim().is_empty() {
        texts.push(SchematicText {
            class: SchematicTextClass::Value,
            placed: value,
            text: value_text,
        });
    }
    texts
}

/// A bus's declaration beside its last point, as the canvas letters it, so
/// the page says which signals the bus carries.
pub fn bus_declaration_text(bus: &Bus) -> Option<SchematicText> {
    Some(SchematicText {
        class: SchematicTextClass::Slice,
        placed: label_layout::bus_declaration_placement(bus)?,
        text: bus.declaration.as_ref()?.to_string(),
    })
}

/// A tap's bit slice beside its connection point, so the page says which
/// bit the tap carries.
pub fn tap_slice_text(tap: &BusTap) -> SchematicText {
    SchematicText {
        class: SchematicTextClass::Slice,
        placed: label_layout::tap_slice_placement(tap),
        text: tap.slice.to_string(),
    }
}
