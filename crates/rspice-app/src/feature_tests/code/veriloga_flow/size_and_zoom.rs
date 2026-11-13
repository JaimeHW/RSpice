//! The size of the source text and of the interface, through whole frames.

use egui::Key;

use super::language_tools::{CTRL, EDITOR, op_amp, source};
use super::*;
use crate::workbench::ChoicePreference;

const SOURCE_LINE: &str = "module opamp_bhv";

fn text_sizes_of(shape: &egui::Shape, sizes: &mut Vec<(String, f32)>) {
    match shape {
        egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| text_sizes_of(shape, sizes)),
        egui::Shape::Text(text) => {
            if let Some(section) = text.galley.job.sections.first() {
                sizes.push((text.galley.job.text.clone(), section.format.font_id.size));
            }
        }
        _ => {}
    }
}

/// The size `line` of the source was painted at in the last frame.
fn painted_size(user: &User, line: &str) -> f32 {
    let mut sizes = Vec::new();
    for clipped in &user.shapes {
        text_sizes_of(&clipped.shape, &mut sizes);
    }
    sizes
        .into_iter()
        .find_map(|(text, size)| text.contains(line).then_some(size))
        .unwrap_or_else(|| panic!("{line:?} was painted"))
}

fn preferred_size(user: &User) -> f32 {
    user.app.state.model.preferences.editor_font_size()
}

#[test]
fn the_zoom_keys_resize_the_text_while_the_editor_has_focus() {
    let mut user = op_amp();
    user.click(EDITOR);
    let before = source(&user);
    assert_eq!(painted_size(&user, SOURCE_LINE), 11.0);

    user.key(Key::Equals, CTRL);
    user.settle();
    assert_eq!(preferred_size(&user), 12.0);
    assert_eq!(painted_size(&user, SOURCE_LINE), 12.0);

    user.key(Key::Plus, CTRL);
    user.settle();
    assert_eq!(painted_size(&user, SOURCE_LINE), 13.0);

    user.key(Key::Minus, CTRL);
    user.key(Key::Minus, CTRL);
    user.settle();
    assert_eq!(painted_size(&user, SOURCE_LINE), 11.0);
    assert_eq!(source(&user), before, "the keys typed nothing");
}

#[test]
fn the_zoom_keys_are_the_editors_only_while_it_has_focus() {
    let mut user = op_amp();
    user.ctx.memory_mut(|memory| {
        if let Some(focused) = memory.focused() {
            memory.surrender_focus(focused);
        }
    });
    user.settle();

    user.key(Key::Equals, CTRL);
    user.settle();

    assert_eq!(preferred_size(&user), 11.0);
}

#[test]
fn the_preferred_size_is_the_size_the_editor_draws_at() {
    let mut user = op_amp();
    user.app
        .state
        .model
        .preferences
        .set_choice(ChoicePreference::EditorFontSize, 8)
        .expect("the largest size");
    user.settle();
    assert_eq!(painted_size(&user, SOURCE_LINE), 20.0);
}

#[test]
fn the_interface_scale_reaches_double_size() {
    let mut user = op_amp();
    user.app
        .state
        .model
        .preferences
        .set_choice(ChoicePreference::InterfaceScale, 5)
        .expect("the largest scale");
    user.settle();
    assert_eq!(user.ctx.zoom_factor(), 2.0);
}
