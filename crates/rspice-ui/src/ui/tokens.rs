//! Design tokens — the single source of truth for the visual system.
//!
//! A [`Tokens`] value bundles the active color [`Palette`] with the metric
//! scales (spacing, type, control sizes, radii) for one combination of
//! [`Direction`] × [`Mode`] × [`Density`]. The active tokens are installed
//! into the egui [`Context`] by [`crate::ui::theme::Theme::apply`] and read
//! back by widgets via [`Tokens::get`], so every part of the UI renders from
//! the same token set with no per-callsite color or size literals.

use std::sync::Arc;

use egui::{Context, Id};
use serde::{Deserialize, Serialize};

use super::palette::{self, Palette};

// ============================================================================
// Spacing scale (4 px base — identical across directions and densities)
// ============================================================================

/// 4 px — tightest gap (icon-to-text, chip padding).
pub const SP_1: f32 = 4.0;
/// 8 px — default gap between controls.
pub const SP_2: f32 = 8.0;
/// 12 px — panel inner padding.
pub const SP_3: f32 = 12.0;
/// 16 px — section padding.
pub const SP_4: f32 = 16.0;
/// 24 px — large structural padding.
pub const SP_5: f32 = 24.0;

// ============================================================================
// Type scale (13 px UI base — dense professional tool)
// ============================================================================

/// 11 px — metadata, badges, mono annotations.
pub const FS_0: f32 = 11.0;
/// 12 px — secondary UI text, tree rows, inputs.
pub const FS_1: f32 = 12.0;
/// 13 px — base UI text.
pub const FS_2: f32 = 13.0;
/// 14 px — emphasized headings.
pub const FS_3: f32 = 14.0;
/// 16 px — large headings (rare).
pub const FS_4: f32 = 16.0;

// ============================================================================
// Direction / Mode / Density
// ============================================================================

/// Design direction: one coherent visual identity for the whole shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Direction {
    /// "Instrument" — bench-instrument heritage: cool neutral grays, a single
    /// phosphor-green accent, mono-forward numerics, 2 px radii, accent line
    /// on the *top* edge of the active tab.
    #[default]
    Instrument,
    /// "Meridian" — contemporary professional IDE: indigo accent, softer
    /// surfaces, 5 px radii, accent underline on the active tab.
    Meridian,
    /// "Graphite" — drafting-table austerity: true monochrome, one amber
    /// accent, 0 px radii, inverted active tab.
    Graphite,
}

impl Direction {
    /// All directions, in presentation order.
    pub const ALL: [Direction; 3] = [
        Direction::Instrument,
        Direction::Meridian,
        Direction::Graphite,
    ];

    /// Human-readable name.
    pub fn label(self) -> &'static str {
        match self {
            Direction::Instrument => "Instrument",
            Direction::Meridian => "Meridian",
            Direction::Graphite => "Graphite",
        }
    }

    /// Control corner radius.
    pub fn radius(self) -> f32 {
        match self {
            Direction::Instrument => 2.0,
            Direction::Meridian => 5.0,
            Direction::Graphite => 0.0,
        }
    }

    /// Large-surface corner radius (menus, cards, popovers).
    pub fn radius_lg(self) -> f32 {
        match self {
            Direction::Instrument => 3.0,
            Direction::Meridian => 7.0,
            Direction::Graphite => 0.0,
        }
    }
}

/// Light or dark rendering of the active direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Dark surfaces (default for long bench sessions).
    #[default]
    Dark,
    /// Light surfaces.
    Light,
}

impl Mode {
    /// All modes, in presentation order.
    pub const ALL: [Mode; 2] = [Mode::Dark, Mode::Light];

    /// Human-readable name.
    pub fn label(self) -> &'static str {
        match self {
            Mode::Dark => "Dark",
            Mode::Light => "Light",
        }
    }
}

/// UI density: compact is the professional default; relaxed enlarges
/// interactive targets without changing the type scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Density {
    /// 24 px controls / 26 px rows.
    #[default]
    Compact,
    /// 28 px controls / 32 px rows.
    Relaxed,
}

impl Density {
    /// All densities, in presentation order.
    pub const ALL: [Density; 2] = [Density::Compact, Density::Relaxed];

    /// Human-readable name.
    pub fn label(self) -> &'static str {
        match self {
            Density::Compact => "Compact",
            Density::Relaxed => "Relaxed",
        }
    }
}

// ============================================================================
// Metrics
// ============================================================================

/// Density-dependent control metrics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metrics {
    /// Height of inputs, selects and buttons.
    pub ctl_h: f32,
    /// Height of list/tree rows.
    pub row_h: f32,
    /// Vertical padding inside bars.
    pub bar_pad: f32,
}

impl Metrics {
    fn for_density(density: Density) -> Self {
        match density {
            Density::Compact => Metrics {
                ctl_h: 24.0,
                row_h: 26.0,
                bar_pad: 6.0,
            },
            Density::Relaxed => Metrics {
                ctl_h: 28.0,
                row_h: 32.0,
                bar_pad: 9.0,
            },
        }
    }
}

// ============================================================================
// Tokens
// ============================================================================

/// The resolved token set for the active theme. Cheap to read each frame via
/// [`Tokens::get`] (one `Arc` clone out of egui's context data).
#[derive(Debug, Clone, PartialEq)]
pub struct Tokens {
    /// Active design direction.
    pub direction: Direction,
    /// Active light/dark mode.
    pub mode: Mode,
    /// Active density.
    pub density: Density,
    /// Resolved color palette.
    pub color: Palette,
    /// Density-dependent control metrics.
    pub metrics: Metrics,
    /// Control corner radius.
    pub radius: f32,
    /// Large-surface corner radius.
    pub radius_lg: f32,
}

impl Default for Tokens {
    fn default() -> Self {
        Self::new(Direction::default(), Mode::default(), Density::default())
    }
}

impl Tokens {
    /// Resolve the token set for a theme selection.
    pub fn new(direction: Direction, mode: Mode, density: Density) -> Self {
        let color = match (direction, mode) {
            (Direction::Instrument, Mode::Dark) => palette::INSTRUMENT_DARK,
            (Direction::Instrument, Mode::Light) => palette::INSTRUMENT_LIGHT,
            (Direction::Meridian, Mode::Dark) => palette::MERIDIAN_DARK,
            (Direction::Meridian, Mode::Light) => palette::MERIDIAN_LIGHT,
            (Direction::Graphite, Mode::Dark) => palette::GRAPHITE_DARK,
            (Direction::Graphite, Mode::Light) => palette::GRAPHITE_LIGHT,
        };
        Self {
            direction,
            mode,
            density,
            color,
            metrics: Metrics::for_density(density),
            radius: direction.radius(),
            radius_lg: direction.radius_lg(),
        }
    }

    fn ctx_key() -> Id {
        Id::new("volta.design.tokens")
    }

    /// Install this token set into the egui context (done by `Theme::apply`).
    /// Also publishes the palette process-globally for paint-level helpers
    /// that have no [`Context`] in reach (see [`active_palette`]).
    pub fn install(self, ctx: &Context) {
        if let Ok(mut palette) = ACTIVE_PALETTE.write() {
            *palette = self.color;
        }
        let tokens = Arc::new(self);
        ctx.data_mut(|d| d.insert_temp(Self::ctx_key(), tokens));
    }

    /// Read the active token set. Falls back to the default theme's tokens if
    /// the theme has not been applied yet (never panics).
    ///
    /// This is the most-called function in the design system (every widget,
    /// every frame), so it takes only the context's read lock; the write
    /// path runs at most once, before the theme is first applied.
    pub fn get(ctx: &Context) -> Arc<Tokens> {
        if let Some(tokens) = ctx.data(|d| d.get_temp::<Arc<Tokens>>(Self::ctx_key())) {
            return tokens;
        }
        ctx.data_mut(|d| {
            d.get_temp_mut_or_insert_with(Self::ctx_key(), || Arc::new(Tokens::default()))
                .clone()
        })
    }

    /// Popover shadow derived from the palette's shadow tokens.
    pub fn shadow(&self) -> egui::epaint::Shadow {
        let (offset_y, blur) = self.color.shadow_geom;
        egui::epaint::Shadow {
            offset: [0, offset_y / 2],
            blur: blur / 2,
            spread: 0,
            color: self.color.shadow_color,
        }
    }
}

/// Process-global copy of the active palette, refreshed by
/// [`Tokens::install`]. Lets paint-level helpers (which receive only a
/// `Painter`) stay theme-correct without threading a `Context` through
/// every rendering signature.
static ACTIVE_PALETTE: std::sync::RwLock<Palette> =
    std::sync::RwLock::new(palette::INSTRUMENT_DARK);

/// The palette of the most recently applied theme.
pub fn active_palette() -> Palette {
    ACTIVE_PALETTE
        .read()
        .map(|palette| *palette)
        .unwrap_or(palette::INSTRUMENT_DARK)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_resolve_every_direction_and_mode() {
        for direction in Direction::ALL {
            for mode in Mode::ALL {
                for density in Density::ALL {
                    let t = Tokens::new(direction, mode, density);
                    assert_eq!(t.direction, direction);
                    assert_eq!(t.radius, direction.radius());
                    assert!(t.metrics.ctl_h > 0.0);
                }
            }
        }
    }

    #[test]
    fn compact_is_denser_than_relaxed() {
        let compact = Metrics::for_density(Density::Compact);
        let relaxed = Metrics::for_density(Density::Relaxed);
        assert!(compact.ctl_h < relaxed.ctl_h);
        assert!(compact.row_h < relaxed.row_h);
        assert!(compact.bar_pad < relaxed.bar_pad);
    }
}
