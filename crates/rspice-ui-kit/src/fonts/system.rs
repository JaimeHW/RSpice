//! Letters what no embedded face has from the faces installed on the desktop.
//!
//! A plugin reads each frame's text as egui hands it to the renderer. A
//! character no face of its family has is asked for, once, from the worker in
//! [`rspice_fonts::system`], and the frame goes on painting the replacement
//! square. The faces the worker has found by the start of a later frame join
//! every family in one font rebuild, after the embedded chain: the script
//! faces, then the CJK faces in the Han order, then the rest. Nothing here
//! waits on the worker, so no frame waits on a disk.
//!
//! A rebuild re-parses every face, starts a new glyph atlas and lays out every
//! visible text again, so faces that arrive together join together.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use egui::{Context, FontData, FontDefinitions, FullOutput, Shape, text::LayoutJob};
pub use rspice_fonts::HanVariant;
use rspice_fonts::system::SystemFace;
pub use rspice_fonts::system::{FaceClass, KnownFace, Source, SystemFonts};
use rspice_fonts::{HanPreference, Role, han_order};

use super::role_of;

/// How often a frame looks for the worker's answer while a request is out.
const POLL: Duration = Duration::from_millis(50);

/// The worker [`start`] began before there was a context to attach it to.
static STARTED: Mutex<Option<(SystemFonts, Vec<String>)>> = Mutex::new(None);

/// Starts reading the operating system's font files now, on a worker thread,
/// so the read overlaps the window's creation. `system_languages` are the
/// operating system's display languages, most preferred first.
pub fn start(system_languages: Vec<String>) {
    let order = han_order(None, HanPreference::Auto, &system_languages);
    let fonts = SystemFonts::spawn(Source::system(), order);
    if let Ok(mut started) = STARTED.lock() {
        *started = Some((fonts, system_languages));
    }
}

/// Letters `ctx`'s text from the installed faces where no embedded face has a
/// character, using the worker [`start`] began, or starting one.
pub fn attach(ctx: &Context) {
    let started = STARTED.lock().ok().and_then(|mut started| started.take());
    let (fonts, system_languages) = started.unwrap_or_else(|| {
        (
            SystemFonts::spawn(Source::system(), HanVariant::Simplified.order()),
            Vec::new(),
        )
    });
    attach_with(ctx, fonts, system_languages);
}

/// Letters `ctx`'s text from the faces `fonts` finds.
pub fn attach_with(ctx: &Context, fonts: SystemFonts, system_languages: Vec<String>) {
    ctx.add_plugin(SystemFontsPlugin::new(fonts, system_languages));
}

/// Sets the language the open document is written in (its title block's
/// Language code), which comes first in choosing the Han form. Costs nothing
/// when it is unchanged.
pub fn set_document_language(ctx: &Context, language: Option<&str>) {
    ctx.with_plugin(|plugin: &mut SystemFontsPlugin| plugin.set_document_language(language));
}

/// The installed faces that have joined `ctx`'s families, in chain order.
#[must_use]
pub fn joined_families(ctx: &Context) -> Vec<String> {
    ctx.with_plugin(|plugin: &mut SystemFontsPlugin| {
        plugin
            .joined
            .iter()
            .map(|face| face.family().to_owned())
            .collect()
    })
    .unwrap_or_default()
}

/// The font definitions `ctx` letters with while the plugin is attached: the
/// embedded faces and every face joined so far. The plugin holds the joined
/// faces, so they outlive anything kept in the context's memory, and every
/// install builds from this one set.
pub(super) fn definitions_with_joined(ctx: &Context) -> Option<FontDefinitions> {
    ctx.with_plugin(|plugin: &mut SystemFontsPlugin| definitions(&plugin.joined))
}

/// The egui plugin that finds and installs the faces.
pub struct SystemFontsPlugin {
    fonts: SystemFonts,
    system_languages: Vec<String>,
    document_language: Option<String>,
    order: [HanVariant; 5],
    /// The faces that have joined, in chain order.
    joined: Vec<SystemFace>,
    /// Every (role, character) whose coverage has been settled.
    seen: HashSet<(Role, char)>,
    /// Every character asked for, in the order it was asked.
    asked: Vec<char>,
    /// Requests the worker has not yet answered.
    outstanding: usize,
    /// Whether the chain order changed since the faces were last installed.
    reordered: bool,
    rebuilds: usize,
}

impl SystemFontsPlugin {
    fn new(fonts: SystemFonts, system_languages: Vec<String>) -> Self {
        let order = han_order(None, HanPreference::Auto, &system_languages);
        Self {
            fonts,
            system_languages,
            document_language: None,
            order,
            joined: Vec::new(),
            seen: HashSet::new(),
            asked: Vec::new(),
            outstanding: 0,
            reordered: false,
            rebuilds: 0,
        }
    }

    fn set_document_language(&mut self, language: Option<&str>) {
        if self.document_language.as_deref() == language {
            return;
        }
        self.document_language = language.map(str::to_owned);
        let order = han_order(language, HanPreference::Auto, &self.system_languages);
        if order == self.order {
            return;
        }
        self.order = order;
        self.reordered = true;
        self.fonts.reorder(order, self.asked.clone());
        self.outstanding += 1;
    }

    /// Whether `character` letters from a face of `role`'s family.
    fn covers(&self, role: Role, character: char) -> bool {
        role.covers(character) || self.joined.iter().any(|face| face.covers(character))
    }

    fn scan(&mut self, shape: &Shape, wanted: &mut Vec<char>) {
        match shape {
            Shape::Text(text) => self.scan_job(&text.galley.job, wanted),
            Shape::Vec(shapes) => {
                for shape in shapes {
                    self.scan(shape, wanted);
                }
            }
            _ => {}
        }
    }

    fn scan_job(&mut self, job: &LayoutJob, wanted: &mut Vec<char>) {
        if job.text.is_ascii() {
            return;
        }
        for section in &job.sections {
            let range = &section.byte_range;
            let Some(text) = job.text.get(range.start.0..range.end.0) else {
                continue;
            };
            if text.is_ascii() {
                continue;
            }
            let role = role_of(&section.format.font_id.family);
            for character in text.chars() {
                if character.is_ascii()
                    || !needs_a_glyph(character)
                    || !self.seen.insert((role, character))
                    || self.covers(role, character)
                    || self.asked.contains(&character)
                {
                    continue;
                }
                self.asked.push(character);
                wanted.push(character);
            }
        }
    }

    /// Installs the embedded faces and every joined face, in chain order.
    fn install(&mut self, ctx: &Context) {
        let order = self.order;
        self.joined.sort_by_key(|face| face.precedence(&order));
        ctx.set_fonts(definitions(&self.joined));
        ctx.request_repaint();
        self.reordered = false;
        self.rebuilds += 1;
    }
}

impl egui::Plugin for SystemFontsPlugin {
    fn debug_name(&self) -> &'static str {
        "rspice_system_fonts"
    }

    fn on_begin_pass(&mut self, ui: &mut egui::Ui) {
        let mut arrived = false;
        while let Some(resolution) = self.fonts.try_resolution() {
            self.outstanding = self.outstanding.saturating_sub(1);
            arrived |= !resolution.joined.is_empty();
            self.joined.extend(resolution.joined);
        }
        if arrived || (self.reordered && self.outstanding == 0) {
            self.install(ui.ctx());
        }
        if self.outstanding > 0 {
            ui.ctx().request_repaint_after(POLL);
        }
    }

    fn output_hook(&mut self, ctx: &Context, output: &mut FullOutput) {
        let mut wanted = Vec::new();
        for clipped in &output.shapes {
            self.scan(&clipped.shape, &mut wanted);
        }
        if !wanted.is_empty() {
            self.fonts.request(wanted);
            self.outstanding += 1;
            ctx.request_repaint_after(POLL);
        }
    }
}

/// The embedded faces with `joined` after each family's chain.
fn definitions(joined: &[SystemFace]) -> FontDefinitions {
    let mut fonts = rspice_fonts::egui_font_definitions();
    for face in joined {
        let mut data = FontData::from_static(face.bytes());
        data.index = face.index();
        if matches!(face.class(), FaceClass::Han(_)) {
            // A CJK glyph is drawn at whole pixels: four subpixel offsets of
            // each would fill the atlas four times as fast.
            data.tweak.subpixel_binning = Some(false);
        }
        let key = face.key();
        fonts.font_data.insert(key.clone(), Arc::new(data));
        for family in fonts.families.values_mut() {
            family.push(key.clone());
        }
    }
    fonts
}

/// Whether a character is drawn: controls, spaces, format characters and
/// variation selectors steer the text around them, private-use characters
/// belong to faces that name them, and none is worth reading every installed
/// face for.
fn needs_a_glyph(character: char) -> bool {
    !(character.is_control()
        || character.is_whitespace()
        || matches!(
            character,
            '\u{00AD}'
                | '\u{034F}'
                | '\u{061C}'
                | '\u{115F}'..='\u{1160}'
                | '\u{17B4}'..='\u{17B5}'
                | '\u{180B}'..='\u{180F}'
                | '\u{200B}'..='\u{200F}'
                | '\u{202A}'..='\u{202E}'
                | '\u{2060}'..='\u{206F}'
                | '\u{3164}'
                | '\u{E000}'..='\u{F8FF}'
                | '\u{FE00}'..='\u{FE0F}'
                | '\u{FEFF}'
                | '\u{FFA0}'
                | '\u{FFF0}'..='\u{FFFB}'
                | '\u{FFFE}'..='\u{FFFF}'
                | '\u{1BCA0}'..='\u{1BCA3}'
                | '\u{1D173}'..='\u{1D17A}'
                | '\u{E0000}'..='\u{E0FFF}'
                | '\u{F0000}'..='\u{10FFFF}'
        ))
}

#[cfg(test)]
mod tests;
