//! Design tokens — the single source of truth for the visual system.
//!
//! A [`Tokens`] value bundles the active color [`Palette`] with the metric
//! scales (spacing, type, control sizes, radii) for the reviewed visual
//! identity in one [`Mode`] × [`Density`] combination. The active tokens are installed
//! into the egui [`Context`] by [`crate::ui::theme::Theme::apply`] and read
//! back by widgets via [`Tokens::get`], so every part of the UI renders from
//! the same token set with no per-callsite color or size literals.

use std::sync::Arc;

use egui::{Color32, Context, Id};
use serde::{Deserialize, Deserializer, Serialize};

use super::palette::{self, Palette};

// ============================================================================
// Exact mockup spacing scale — identical across modes and densities
// ============================================================================

/// 2 px — optical/microcopy separation.
pub const SP_1: f32 = 2.0;
/// 4 px — tight icon-to-text and chip separation.
pub const SP_2: f32 = 4.0;
/// 6 px — compact control groups.
pub const SP_3: f32 = 6.0;
/// 8 px — default control gap.
pub const SP_4: f32 = 8.0;
/// 12 px — panel inner padding.
pub const SP_5: f32 = 12.0;
/// 16 px — section padding.
pub const SP_6: f32 = 16.0;
/// 24 px — large structural padding.
pub const SP_7: f32 = 24.0;
/// 32 px — major surface separation.
pub const SP_8: f32 = 32.0;

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

/// Design direction: one coherent visual identity for the whole application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum Direction {
    /// "Instrument" — bench-instrument heritage: cool neutral grays, a single
    /// amber accent, mono-forward numerics, 3 px radii, accent line
    /// on the *top* edge of the active tab.
    #[default]
    Instrument,
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        enum PersistedDirection {
            Instrument,
            Meridian,
            Graphite,
        }

        // Meridian and Graphite were emitted by the retired placeholder
        // shell. Reading them is a migration operation, not a supported
        // visual mode: every restored session uses the mockup-owned palette.
        let _legacy_value = PersistedDirection::deserialize(deserializer)?;
        Ok(Self::Instrument)
    }
}

impl Direction {
    /// All directions, in presentation order.
    pub const ALL: [Direction; 1] = [Direction::Instrument];

    /// Human-readable name.
    pub fn label(self) -> &'static str {
        "Instrument"
    }

    /// Control corner radius.
    pub fn radius(self) -> f32 {
        3.0
    }

    /// Large-surface corner radius (menus, cards, popovers).
    pub fn radius_lg(self) -> f32 {
        6.0
    }
}

/// User-selected color-mode policy for the active direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Dark surfaces (default for long bench sessions).
    #[default]
    Dark,
    /// Light surfaces.
    Light,
    /// Follow the host operating-system or browser color preference.
    System,
}

impl Mode {
    /// All modes, in presentation order.
    pub const ALL: [Mode; 3] = [Mode::Dark, Mode::Light, Mode::System];

    /// Human-readable name.
    pub fn label(self) -> &'static str {
        match self {
            Mode::Dark => "Dark",
            Mode::Light => "Light",
            Mode::System => "System",
        }
    }

    /// Resolve a system-following preference against the current host.
    pub fn effective(self, ctx: &Context) -> Self {
        match self {
            Self::System => match ctx.system_theme() {
                Some(egui::Theme::Light) => Self::Light,
                Some(egui::Theme::Dark) | None => Self::Dark,
            },
            explicit => explicit,
        }
    }
}

/// UI density: compact is the professional default; relaxed enlarges
/// interactive targets without changing the type scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Density {
    /// Compact professional spacing (Instrument: 28 px controls / rows).
    #[default]
    Compact,
    /// Relaxed spacing (Instrument: 32 px controls / 33 px rows).
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

/// Density-dependent control metrics for the reviewed Instrument direction.
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
    fn for_selection(_direction: Direction, density: Density) -> Self {
        match density {
            Density::Compact => Metrics {
                ctl_h: 28.0,
                row_h: 28.0,
                bar_pad: 6.0,
            },
            Density::Relaxed => Metrics {
                ctl_h: 32.0,
                row_h: 33.0,
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
    /// Whether engineering traces use color, dash, and marker redundancy.
    pub color_safe_traces: bool,
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
            (Direction::Instrument, Mode::Dark | Mode::System) => palette::INSTRUMENT_DARK,
            (Direction::Instrument, Mode::Light) => palette::INSTRUMENT_LIGHT,
        };
        Self {
            direction,
            mode,
            density,
            color_safe_traces: false,
            color,
            metrics: Metrics::for_selection(direction, density),
            radius: direction.radius(),
            radius_lg: direction.radius_lg(),
        }
    }

    fn ctx_key() -> Id {
        Id::new("rspice.design.tokens")
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
            offset: [0, offset_y],
            blur,
            spread: 0,
            color: self.color.shadow_color,
        }
    }

    /// Modal-workflow elevation from the mockup's `--shadow-dialog` token.
    /// egui supports one shadow layer, so this preserves the exact dominant
    /// layer rather than substituting the much smaller floating-menu shadow.
    pub fn dialog_shadow(&self) -> egui::epaint::Shadow {
        let color = if self.mode == Mode::Light {
            Color32::from_rgba_premultiplied(11, 12, 13, 66)
        } else {
            Color32::from_rgba_premultiplied(0, 0, 0, 117)
        };
        egui::epaint::Shadow {
            offset: [0, 18],
            blur: 52,
            spread: 0,
            color,
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
    fn spacing_scale_matches_the_workbench_mockup_exactly() {
        assert_eq!(
            [SP_1, SP_2, SP_3, SP_4, SP_5, SP_6, SP_7, SP_8],
            [2.0, 4.0, 6.0, 8.0, 12.0, 16.0, 24.0, 32.0]
        );
    }

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
        for direction in Direction::ALL {
            let compact = Metrics::for_selection(direction, Density::Compact);
            let relaxed = Metrics::for_selection(direction, Density::Relaxed);
            assert!(compact.ctl_h < relaxed.ctl_h);
            assert!(compact.row_h < relaxed.row_h);
            assert!(compact.bar_pad < relaxed.bar_pad);
        }
    }

    #[test]
    fn instrument_metrics_and_radii_match_the_workbench_mockup() {
        let compact = Tokens::new(Direction::Instrument, Mode::Dark, Density::Compact);
        assert_eq!(compact.metrics.ctl_h, 28.0);
        assert_eq!(compact.metrics.row_h, 28.0);
        assert_eq!(compact.radius, 3.0);
        assert_eq!(compact.radius_lg, 6.0);
        assert_eq!(compact.shadow().offset, [0, 14]);
        assert_eq!(compact.shadow().blur, 42);
        assert_eq!(compact.dialog_shadow().offset, [0, 18]);
        assert_eq!(compact.dialog_shadow().blur, 52);
    }

    #[test]
    fn retired_placeholder_directions_deserialize_into_instrument() {
        for persisted in ["Instrument", "Meridian", "Graphite"] {
            let encoded = format!("\"{persisted}\"");
            let restored: Direction =
                serde_json::from_str(&encoded).expect("known persisted direction migrates");
            assert_eq!(restored, Direction::Instrument);
        }
        assert!(serde_json::from_str::<Direction>("\"invented\"").is_err());
    }
}
