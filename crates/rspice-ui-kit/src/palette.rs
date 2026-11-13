//! Exact dark/light palettes for the reviewed Instrument visual direction.
//! Values mirror the semantic design tokens in the workbench mockup.

use egui::Color32;

/// How many hues a trace palette cycles through before it repeats them in
/// another dash.
pub const TRACE_HUES: usize = 8;

const fn color_is_rgb(color: Color32, r: u8, g: u8, b: u8) -> bool {
    color.r() == r && color.g() == g && color.b() == b && color.a() == 255
}

/// One complete semantic color palette for a surface mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    /// Application background (outermost chrome).
    pub bg_app: Color32,
    /// Panel surfaces (side panels, bars, cards).
    pub bg_panel: Color32,
    /// Secondary panel treatment (table heads and section labels).
    pub bg_panel_2: Color32,
    /// Inset wells (inputs, embedded lists).
    pub bg_inset: Color32,
    /// Elevated surfaces (menus, popovers, toasts).
    pub bg_elevated: Color32,
    /// Hover fill for interactive rows/buttons.
    pub bg_hover: Color32,
    /// Pressed/active fill.
    pub bg_active: Color32,
    /// Hairline borders between surfaces.
    pub border: Color32,
    /// Emphasized borders (focused inputs, popovers).
    pub border_strong: Color32,
    /// Primary text.
    pub text: Color32,
    /// Secondary text.
    pub text_dim: Color32,
    /// Tertiary text (metadata, disabled).
    pub text_faint: Color32,
    /// Accent (selection, primary actions).
    pub accent: Color32,
    /// Text/icon color on accent fills.
    pub accent_ink: Color32,
    /// Translucent accent wash (selected rows, toggles).
    pub accent_dim: Color32,
    /// Success state.
    pub ok: Color32,
    /// Warning state.
    pub warn: Color32,
    /// Error state.
    pub err: Color32,
    /// Informational state.
    pub info: Color32,
    /// Document-well background (schematic, plots, code).
    pub canvas_bg: Color32,
    /// Canvas grid dots / plot gridlines.
    pub canvas_grid: Color32,
    /// Schematic wire stroke.
    pub wire: Color32,
    /// Schematic device symbol stroke.
    pub symbol: Color32,
    /// Schematic net-name labels.
    pub net_label: Color32,
    /// Selection on the schematic canvas: a selected conductor, symbol,
    /// junction or mark, and the selection rect behind selected text. Its
    /// own hue, apart from the wire and the net highlight, rather than the
    /// interface `accent`.
    pub canvas_select: Color32,
    /// A highlighted net on the schematic canvas.
    pub canvas_net_highlight: Color32,
    /// Net-class colours: ground, supply and clock nets.
    pub net_class_ground: Color32,
    pub net_class_supply: Color32,
    pub net_class_clock: Color32,
    /// Primary text drawn on a canvas: tick values, cursor readouts and the
    /// labels a viewer paints on its well. The canvas has its own theme, so
    /// the interface text tones can sit on it dark on dark.
    pub canvas_text: Color32,
    /// Secondary text on a canvas (tick labels, lane names).
    pub canvas_text_dim: Color32,
    /// Tertiary text on a canvas (axis units, hints).
    pub canvas_text_faint: Color32,
    /// Cursor A on a canvas: the canvas theme's own accent, which stands off
    /// that canvas whatever the interface mode beside it.
    pub canvas_cursor_a: Color32,
    /// Cursor B on a canvas: a neutral no trace hue is near, so the second
    /// cursor never reads as a curve.
    pub canvas_cursor_b: Color32,
    /// Waveform trace cycle (assignment order for new traces): eight hues
    /// that stand apart from each other and off the canvas.
    pub traces: [Color32; TRACE_HUES],
    /// Popover/menu shadow color (translucent).
    pub shadow_color: Color32,
    /// Popover shadow geometry: (y-offset, blur) in points.
    pub shadow_geom: (i8, u8),
    /// Source-code inks and the editor's own fills.
    pub syntax: SyntaxPalette,
}

/// What a source editor paints code with: one ink per syntax class, the fill
/// behind selected text and the fill of the caret's line. Every ink stands
/// 4.5:1 off the code well (`bg_inset`), the caret's line and the selection,
/// so a selected or current line reads as plainly as any other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxPalette {
    /// Reserved words: `module`, `analog`, `begin`, `parameter`.
    pub keyword: Color32,
    /// Disciplines and natures, and a nature's access function: `electrical`,
    /// `thermal`, the `V` of `V(out)`.
    pub discipline: Color32,
    /// System functions and tasks: `$temperature`, `$strobe`.
    pub system: Color32,
    /// Analog operators and filters: `ddt`, `transition`, `laplace_nd`.
    pub analog_operator: Color32,
    /// The contribution operator `<+`.
    pub contribution: Color32,
    /// Numbers, with their scale factor or size: `10k`, `1e-3`, `4'hF`.
    pub number: Color32,
    pub string: Color32,
    pub comment: Color32,
    /// Compiler directives and macro uses: `` `include ``, `` `M_PI ``.
    pub directive: Color32,
    /// Attribute instances: `(* desc="gain" *)`.
    pub attribute: Color32,
    /// The name a module declares.
    pub module_name: Color32,
    /// The names of a module's ports where they are declared.
    pub port_name: Color32,
    /// The names of parameters where they are declared.
    pub parameter_name: Color32,
    /// The fill behind selected text.
    pub selection: Color32,
    /// The fill of the line the caret is on.
    pub current_line: Color32,
}

/// The colors a contrast theme sets, which everything on screen is drawn from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContrastInks {
    /// The ground of every surface.
    pub window: Color32,
    pub window_text: Color32,
    /// Selection and primary actions.
    pub highlight: Color32,
    /// Text on [`Self::highlight`].
    pub highlight_text: Color32,
    pub link: Color32,
}

impl ContrastInks {
    /// White on black, with cyan selection and yellow links.
    pub const BLACK: Self = Self {
        window: Color32::from_rgb(0, 0, 0),
        window_text: Color32::from_rgb(255, 255, 255),
        highlight: Color32::from_rgb(0x1a, 0xeb, 0xff),
        highlight_text: Color32::from_rgb(0, 0, 0),
        link: Color32::from_rgb(255, 255, 0),
    };

    /// Whether text on this theme's ground is lighter than the ground.
    pub const fn has_dark_ground(self) -> bool {
        let ground = self.window;
        (ground.r() as u32 * 2126 + ground.g() as u32 * 7152 + ground.b() as u32 * 722)
            < 128 * 10_000
    }
}

/// `to` laid over `from` at `percent`, rounded down channel by channel.
const fn blend(from: Color32, to: Color32, percent: u32) -> Color32 {
    const fn channel(from: u8, to: u8, percent: u32) -> u8 {
        ((from as u32 * (100 - percent) + to as u32 * percent) / 100) as u8
    }
    Color32::from_rgb(
        channel(from.r(), to.r(), percent),
        channel(from.g(), to.g(), percent),
        channel(from.b(), to.b(), percent),
    )
}

/// The states, the canvas classes, the traces and the code a contrast theme
/// has no system color for, one set per kind of ground.
struct ContrastGroundInks {
    ok: Color32,
    warn: Color32,
    err: Color32,
    info: Color32,
    magenta: Color32,
    traces: [Color32; TRACE_HUES],
    keyword: Color32,
    discipline: Color32,
    system: Color32,
    analog_operator: Color32,
    contribution: Color32,
    number: Color32,
    string: Color32,
    comment: Color32,
    directive: Color32,
    attribute: Color32,
    module_name: Color32,
    port_name: Color32,
    parameter_name: Color32,
}

const ON_DARK_GROUND: ContrastGroundInks = ContrastGroundInks {
    ok: Color32::from_rgb(0x5c, 0xf2, 0x7a),
    warn: Color32::from_rgb(0xff, 0xc1, 0x4d),
    err: Color32::from_rgb(0xff, 0x9a, 0x8c),
    info: Color32::from_rgb(0x8c, 0xd8, 0xff),
    magenta: Color32::from_rgb(0xff, 0x8c, 0xf0),
    // Eight chromatic hues: white is cursor B's, so no trace takes it.
    traces: [
        Color32::from_rgb(0xff, 0xe0, 0x33),
        Color32::from_rgb(0x4d, 0xe8, 0xff),
        Color32::from_rgb(0xff, 0x8c, 0xf0),
        Color32::from_rgb(0x5c, 0xf2, 0x7a),
        Color32::from_rgb(0xff, 0xa8, 0x5c),
        Color32::from_rgb(0x78, 0xa0, 0xff),
        Color32::from_rgb(0xff, 0x78, 0x78),
        Color32::from_rgb(0xbe, 0xff, 0x3c),
    ],
    keyword: Color32::from_rgb(0x8c, 0xcb, 0xff),
    discipline: Color32::from_rgb(0x4e, 0xe6, 0xd0),
    system: Color32::from_rgb(0xd6, 0xb4, 0xfd),
    analog_operator: Color32::from_rgb(0xff, 0xa6, 0xe6),
    contribution: Color32::from_rgb(0xff, 0xe0, 0x66),
    number: Color32::from_rgb(0xb5, 0xf0, 0xa0),
    string: Color32::from_rgb(0xff, 0xc0, 0x8a),
    comment: Color32::from_rgb(0xc4, 0xd2, 0xc4),
    directive: Color32::from_rgb(0xff, 0xb0, 0xcc),
    attribute: Color32::from_rgb(0xd0, 0xd8, 0xe6),
    module_name: Color32::from_rgb(0xff, 0xe0, 0x66),
    port_name: Color32::from_rgb(0x9c, 0xe8, 0xff),
    parameter_name: Color32::from_rgb(0xff, 0xd0, 0x80),
};

const ON_LIGHT_GROUND: ContrastGroundInks = ContrastGroundInks {
    ok: Color32::from_rgb(0x0a, 0x5c, 0x1e),
    warn: Color32::from_rgb(0x7a, 0x42, 0x00),
    err: Color32::from_rgb(0xa8, 0x00, 0x1c),
    info: Color32::from_rgb(0x00, 0x4a, 0x80),
    magenta: Color32::from_rgb(0x8a, 0x00, 0x72),
    // Eight chromatic hues: black is cursor B's, so no trace takes it.
    traces: [
        Color32::from_rgb(0x00, 0x33, 0xa0),
        Color32::from_rgb(0xa8, 0x00, 0x1c),
        Color32::from_rgb(0x0a, 0x5c, 0x1e),
        Color32::from_rgb(0x8a, 0x00, 0x72),
        Color32::from_rgb(0x7a, 0x42, 0x00),
        Color32::from_rgb(0x00, 0x5a, 0x64),
        Color32::from_rgb(0x55, 0x28, 0xa0),
        Color32::from_rgb(0xaa, 0x46, 0x00),
    ],
    keyword: Color32::from_rgb(0x00, 0x33, 0x9a),
    discipline: Color32::from_rgb(0x00, 0x57, 0x4f),
    system: Color32::from_rgb(0x5a, 0x1e, 0x96),
    analog_operator: Color32::from_rgb(0x8a, 0x0f, 0x6a),
    contribution: Color32::from_rgb(0x6b, 0x3e, 0x00),
    number: Color32::from_rgb(0x1b, 0x5e, 0x20),
    string: Color32::from_rgb(0x8a, 0x3a, 0x00),
    comment: Color32::from_rgb(0x35, 0x50, 0x35),
    directive: Color32::from_rgb(0x8a, 0x1e, 0x50),
    attribute: Color32::from_rgb(0x3d, 0x4a, 0x5c),
    module_name: Color32::from_rgb(0x5f, 0x43, 0x00),
    port_name: Color32::from_rgb(0x00, 0x4f, 0x6e),
    parameter_name: Color32::from_rgb(0x6b, 0x3e, 0x00),
};

/// A flat palette built from a contrast theme's colors: one ground, no
/// shadow, and every state told apart by an ink that stands well off it.
pub const fn high_contrast(inks: ContrastInks) -> Palette {
    let ground = if inks.has_dark_ground() {
        ON_DARK_GROUND
    } else {
        ON_LIGHT_GROUND
    };
    Palette {
        bg_app: inks.window,
        bg_panel: inks.window,
        bg_panel_2: inks.window,
        bg_inset: inks.window,
        bg_elevated: inks.window,
        bg_hover: blend(inks.window, inks.window_text, 18),
        bg_active: blend(inks.window, inks.window_text, 30),
        border: blend(inks.window, inks.window_text, 75),
        border_strong: inks.window_text,
        text: inks.window_text,
        text_dim: blend(inks.window, inks.window_text, 88),
        text_faint: blend(inks.window, inks.window_text, 75),
        accent: inks.highlight,
        accent_ink: inks.highlight_text,
        accent_dim: blend(inks.window, inks.highlight, 30),
        ok: ground.ok,
        warn: ground.warn,
        err: ground.err,
        info: ground.info,
        canvas_bg: inks.window,
        canvas_grid: blend(inks.window, inks.window_text, 40),
        wire: inks.link,
        symbol: inks.window_text,
        net_label: inks.window_text,
        canvas_select: inks.highlight,
        canvas_net_highlight: ground.magenta,
        net_class_ground: ground.info,
        net_class_supply: ground.warn,
        net_class_clock: ground.magenta,
        canvas_text: inks.window_text,
        canvas_text_dim: blend(inks.window, inks.window_text, 88),
        canvas_text_faint: blend(inks.window, inks.window_text, 75),
        canvas_cursor_a: inks.highlight,
        canvas_cursor_b: inks.window_text,
        traces: ground.traces,
        shadow_color: Color32::TRANSPARENT,
        shadow_geom: (0, 0),
        syntax: SyntaxPalette {
            keyword: ground.keyword,
            discipline: ground.discipline,
            system: ground.system,
            analog_operator: ground.analog_operator,
            contribution: ground.contribution,
            number: ground.number,
            string: ground.string,
            comment: ground.comment,
            directive: ground.directive,
            attribute: ground.attribute,
            module_name: ground.module_name,
            port_name: ground.port_name,
            parameter_name: ground.parameter_name,
            selection: blend(inks.window, inks.highlight, 20),
            current_line: blend(inks.window, inks.window_text, 12),
        },
    }
}

/// Instrument · high contrast
pub const INSTRUMENT_HIGH_CONTRAST: Palette = high_contrast(ContrastInks::BLACK);

/// Instrument · dark
pub const INSTRUMENT_DARK: Palette = Palette {
    bg_app: Color32::from_rgb(17, 23, 27),
    bg_panel: Color32::from_rgb(26, 32, 36),
    bg_panel_2: Color32::from_rgb(0x20, 0x28, 0x2d),
    bg_inset: Color32::from_rgb(11, 17, 21),
    bg_elevated: Color32::from_rgb(33, 41, 46),
    bg_hover: Color32::from_rgb(38, 46, 51),
    bg_active: Color32::from_rgb(46, 58, 65),
    border: Color32::from_rgb(43, 49, 54),
    border_strong: Color32::from_rgb(68, 76, 80),
    text: Color32::from_rgb(215, 219, 222),
    text_dim: Color32::from_rgb(147, 153, 158),
    // One step below `text_dim` against a dark ground, which the mockup's
    // own token (0x96, 0x9d, 0xa2) is not: at (150, 157, 162) the tertiary
    // tone carried *more* contrast than the secondary one, so a unit caption
    // outshone the value it annotated and a placeholder read as a declared
    // value. Same hue, 21% less contrast than `text_dim` on every ground the
    // palette paints on. `text_tones_descend_against_every_ground` holds the
    // ladder.
    text_faint: Color32::from_rgb(126, 133, 138),
    accent: Color32::from_rgb(242, 184, 36),
    accent_ink: Color32::from_rgb(23, 24, 26),
    accent_dim: Color32::from_rgba_premultiplied(27, 20, 4, 28),
    ok: Color32::from_rgb(101, 214, 138),
    warn: Color32::from_rgb(234, 147, 68),
    err: Color32::from_rgb(0xef, 0x80, 0x6f),
    info: Color32::from_rgb(0x7c, 0xb8, 0xe8),
    canvas_bg: Color32::from_rgb(7, 12, 16),
    canvas_grid: Color32::from_rgb(28, 35, 39),
    // Dark enough that the yellow selection stands 3:1 above it.
    wire: Color32::from_rgb(68, 140, 96),
    symbol: Color32::from_rgb(203, 210, 215),
    net_label: Color32::from_rgb(220, 177, 85),
    canvas_select: Color32::from_rgb(255, 226, 92),
    // Magenta: orange beside the yellow selection collapses for deuteranopes.
    canvas_net_highlight: Color32::from_rgb(232, 110, 210),
    net_class_ground: Color32::from_rgb(0x7c, 0xb8, 0xe8),
    net_class_supply: Color32::from_rgb(234, 147, 68),
    net_class_clock: Color32::from_rgb(217, 132, 176),
    canvas_text: Color32::from_rgb(215, 219, 222),
    canvas_text_dim: Color32::from_rgb(147, 153, 158),
    canvas_text_faint: Color32::from_rgb(126, 133, 138),
    canvas_cursor_a: Color32::from_rgb(242, 184, 36),
    canvas_cursor_b: Color32::from_rgb(205, 212, 217),
    // Blue, coral, green, violet, cyan, rose, lime, peach: at least 0.1
    // apart in OKLab, and none of them the amber of cursor A or the grey of
    // cursor B, so a cursor never reads as a curve.
    traces: [
        Color32::from_rgb(100, 170, 245),
        Color32::from_rgb(250, 105, 90),
        Color32::from_rgb(110, 215, 110),
        Color32::from_rgb(175, 145, 255),
        Color32::from_rgb(60, 215, 210),
        Color32::from_rgb(235, 120, 190),
        Color32::from_rgb(200, 230, 80),
        Color32::from_rgb(255, 175, 140),
    ],
    shadow_color: Color32::from_rgba_premultiplied(0, 0, 0, 107),
    shadow_geom: (14, 42),
    syntax: SyntaxPalette {
        keyword: Color32::from_rgb(122, 178, 240),
        discipline: Color32::from_rgb(86, 196, 182),
        system: Color32::from_rgb(196, 160, 240),
        analog_operator: Color32::from_rgb(236, 142, 200),
        contribution: Color32::from_rgb(242, 184, 36),
        number: Color32::from_rgb(140, 206, 120),
        string: Color32::from_rgb(232, 160, 100),
        comment: Color32::from_rgb(150, 162, 156),
        directive: Color32::from_rgb(222, 138, 168),
        attribute: Color32::from_rgb(160, 168, 184),
        module_name: Color32::from_rgb(236, 204, 104),
        port_name: Color32::from_rgb(128, 210, 230),
        parameter_name: Color32::from_rgb(220, 177, 85),
        selection: Color32::from_rgb(56, 48, 24),
        current_line: Color32::from_rgb(22, 29, 34),
    },
};

/// Instrument · light
pub const INSTRUMENT_LIGHT: Palette = Palette {
    bg_app: Color32::from_rgb(232, 236, 238),
    bg_panel: Color32::from_rgb(245, 247, 248),
    bg_panel_2: Color32::from_rgb(0xed, 0xf1, 0xf3),
    bg_inset: Color32::from_rgb(225, 229, 231),
    bg_elevated: Color32::from_rgb(251, 253, 254),
    bg_hover: Color32::from_rgb(230, 234, 237),
    bg_active: Color32::from_rgb(212, 220, 226),
    border: Color32::from_rgb(209, 213, 216),
    border_strong: Color32::from_rgb(166, 172, 175),
    text: Color32::from_rgb(31, 37, 40),
    text_dim: Color32::from_rgb(83, 89, 93),
    text_faint: Color32::from_rgb(0x5c, 0x63, 0x67),
    accent: Color32::from_rgb(143, 100, 0),
    accent_ink: Color32::from_rgb(252, 250, 246),
    accent_dim: Color32::from_rgba_premultiplied(13, 9, 0, 23),
    ok: Color32::from_rgb(0, 124, 58),
    warn: Color32::from_rgb(194, 110, 18),
    err: Color32::from_rgb(192, 58, 40),
    info: Color32::from_rgb(0x3d, 0x72, 0xa8),
    canvas_bg: Color32::from_rgb(251, 253, 254),
    canvas_grid: Color32::from_rgb(224, 227, 229),
    // Dark enough that the azure selection, which has to sit in the 0.21 to
    // 0.29 luminance band on paper, still stands 3:1 off it.
    wire: Color32::from_rgb(78, 52, 0),
    symbol: Color32::from_rgb(44, 49, 53),
    net_label: Color32::from_rgb(125, 82, 0),
    canvas_select: Color32::from_rgb(30, 136, 255),
    // Orange, the complement of the azure selection.
    canvas_net_highlight: Color32::from_rgb(194, 110, 18),
    net_class_ground: Color32::from_rgb(0, 124, 58),
    net_class_supply: Color32::from_rgb(194, 110, 18),
    net_class_clock: Color32::from_rgb(168, 83, 126),
    canvas_text: Color32::from_rgb(31, 37, 40),
    canvas_text_dim: Color32::from_rgb(83, 89, 93),
    canvas_text_faint: Color32::from_rgb(0x5c, 0x63, 0x67),
    canvas_cursor_a: Color32::from_rgb(143, 100, 0),
    canvas_cursor_b: Color32::from_rgb(52, 60, 66),
    // Blue, crimson, green, violet, teal, magenta, orange, navy: each deep
    // enough to stand 4.5:1 off paper, at least 0.1 apart in OKLab, and none
    // of them the gold of cursor A or the slate of cursor B.
    traces: [
        Color32::from_rgb(30, 105, 190),
        Color32::from_rgb(190, 30, 80),
        Color32::from_rgb(0, 128, 60),
        Color32::from_rgb(110, 70, 190),
        Color32::from_rgb(0, 122, 128),
        Color32::from_rgb(160, 50, 150),
        Color32::from_rgb(195, 80, 0),
        Color32::from_rgb(10, 60, 150),
    ],
    shadow_color: Color32::from_rgba_premultiplied(7, 8, 9, 46),
    shadow_geom: (14, 42),
    syntax: SyntaxPalette {
        keyword: Color32::from_rgb(28, 78, 158),
        discipline: Color32::from_rgb(0, 96, 87),
        system: Color32::from_rgb(106, 58, 168),
        analog_operator: Color32::from_rgb(153, 37, 105),
        contribution: Color32::from_rgb(123, 73, 0),
        number: Color32::from_rgb(27, 98, 22),
        string: Color32::from_rgb(144, 59, 0),
        comment: Color32::from_rgb(77, 87, 83),
        directive: Color32::from_rgb(146, 46, 78),
        attribute: Color32::from_rgb(78, 84, 100),
        module_name: Color32::from_rgb(115, 77, 0),
        port_name: Color32::from_rgb(0, 90, 130),
        parameter_name: Color32::from_rgb(96, 82, 20),
        selection: Color32::from_rgb(222, 206, 156),
        current_line: Color32::from_rgb(214, 220, 224),
    },
};

const _: [(); 1] = [(); color_is_rgb(INSTRUMENT_DARK.wire, 0x44, 0x8c, 0x60) as usize];
const _: [(); 1] = [(); color_is_rgb(INSTRUMENT_LIGHT.wire, 0x4e, 0x34, 0x00) as usize];

/// sRGB relative luminance (WCAG 2.x).
fn relative_luminance(color: Color32) -> f64 {
    let [red, green, blue] = linear_rgb(color);
    0.2126 * red + 0.7152 * green + 0.0722 * blue
}

fn linear_rgb(color: Color32) -> [f64; 3] {
    let linear = |channel: u8| {
        let channel = f64::from(channel) / 255.0;
        if channel <= 0.040_45 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    };
    [linear(color.r()), linear(color.g()), linear(color.b())]
}

/// How far an ink stands off the ground it is painted on, as the WCAG
/// contrast ratio. Channel values cannot answer this: on a dark ground a
/// brighter grey stands further off, on a light ground it stands closer.
pub fn contrast_ratio(ink: Color32, ground: Color32) -> f64 {
    let (ink, ground) = (relative_luminance(ink), relative_luminance(ground));
    let (brighter, darker) = if ink > ground {
        (ink, ground)
    } else {
        (ground, ink)
    };
    (brighter + 0.05) / (darker + 0.05)
}

/// How different two colours look: their distance in OKLab, where 0.02 is
/// about the smallest step an eye sees and two hues a reader names apart
/// stand 0.09 or more apart.
pub fn perceptual_distance(a: Color32, b: Color32) -> f64 {
    let oklab = |color: Color32| {
        let [r, g, b] = linear_rgb(color);
        let l = (0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b).cbrt();
        let m = (0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b).cbrt();
        let s = (0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b).cbrt();
        [
            0.210_454_255_3 * l + 0.793_617_785 * m - 0.004_072_046_8 * s,
            1.977_998_495_1 * l - 2.428_592_205 * m + 0.450_593_709_9 * s,
            0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766 * s,
        ]
    };
    let (a, b) = (oklab(a), oklab(b));
    a.iter()
        .zip(b)
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Selection, the wire and the canvas stand apart in both palettes: a
    /// selected wire 3:1 off an unselected one, both 3:1 off the canvas, and
    /// a highlighted net 3:1 off the canvas. Selection, highlight and the
    /// wire are different hues, so the state also reads for a deuteranope.
    #[test]
    fn canvas_state_colours_stand_apart() {
        for (mode, palette) in [("dark", INSTRUMENT_DARK), ("light", INSTRUMENT_LIGHT)] {
            for (pair, ink, ground) in [
                ("selection on the wire", palette.canvas_select, palette.wire),
                ("wire on the canvas", palette.wire, palette.canvas_bg),
                (
                    "selection on the canvas",
                    palette.canvas_select,
                    palette.canvas_bg,
                ),
                (
                    "net highlight on the canvas",
                    palette.canvas_net_highlight,
                    palette.canvas_bg,
                ),
            ] {
                let ratio = contrast_ratio(ink, ground);
                assert!(ratio >= 3.0, "{mode} {pair} is only {ratio:.2}:1");
            }
            assert_ne!(palette.canvas_select, palette.canvas_net_highlight);
            assert_ne!(palette.canvas_select, palette.accent, "{mode}");
        }
    }

    /// Primary, secondary, tertiary: each tone must stand off its ground less
    /// than the one above it, in both palettes and on every surface the
    /// palette paints text on.
    ///
    /// A tertiary tone is a third step or it is noise, so it must also give
    /// up a real share of the secondary tone's contrast — and still be
    /// readable, since a ladder nobody can read at the bottom is a worse
    /// defect than the inversion it replaced.
    /// The contrast themes Windows ships, light ground and dark, and the one
    /// the Color mode row selects. What each sets is its own; what is drawn
    /// from it has to stand off its ground on all of them.
    pub(crate) const CONTRAST_THEMES: [(&str, ContrastInks); 3] = [
        ("black", ContrastInks::BLACK),
        (
            "desert",
            ContrastInks {
                window: Color32::from_rgb(0xff, 0xfa, 0xef),
                window_text: Color32::from_rgb(0x3d, 0x3d, 0x3d),
                highlight: Color32::from_rgb(0x90, 0x39, 0x09),
                highlight_text: Color32::from_rgb(0xff, 0xf5, 0xe3),
                link: Color32::from_rgb(0x1c, 0x5e, 0x75),
            },
        ),
        (
            "dusk",
            ContrastInks {
                window: Color32::from_rgb(0x2d, 0x32, 0x36),
                window_text: Color32::from_rgb(0xff, 0xff, 0xff),
                highlight: Color32::from_rgb(0xab, 0xcf, 0xf2),
                highlight_text: Color32::from_rgb(0x21, 0x2d, 0x3b),
                link: Color32::from_rgb(0x70, 0xeb, 0xde),
            },
        ),
    ];

    #[test]
    fn a_contrast_palette_stands_off_its_ground_everywhere() {
        for (theme, inks) in CONTRAST_THEMES {
            let palette = high_contrast(inks);
            let ground = palette.bg_panel;
            for (name, ink, needed) in [
                ("text", palette.text, 7.0),
                ("secondary text", palette.text_dim, 4.5),
                ("tertiary text", palette.text_faint, 4.5),
                ("border", palette.border, 3.0),
                ("strong border", palette.border_strong, 3.0),
                ("accent", palette.accent, 3.0),
                ("ok", palette.ok, 4.5),
                ("warn", palette.warn, 4.5),
                ("err", palette.err, 4.5),
                ("info", palette.info, 4.5),
                ("wire", palette.wire, 4.5),
                ("symbol", palette.symbol, 7.0),
            ] {
                let measured = contrast_ratio(ink, ground);
                assert!(measured >= needed, "{theme} {name}: {measured:.2}:1");
            }
            assert!(
                contrast_ratio(palette.text, palette.bg_hover) >= 4.5
                    && contrast_ratio(palette.text, palette.bg_active) >= 4.5
                    && contrast_ratio(palette.text, palette.accent_dim) >= 4.5,
                "{theme}: text on the row fills"
            );
            assert!(
                contrast_ratio(palette.accent_ink, palette.accent) >= 4.5,
                "{theme}"
            );
            for trace in palette.traces {
                assert!(
                    contrast_ratio(trace, palette.canvas_bg) >= 4.5,
                    "{theme} {trace:?}"
                );
            }
            let syntax = palette.syntax;
            for ink in [
                syntax.keyword,
                syntax.discipline,
                syntax.system,
                syntax.analog_operator,
                syntax.contribution,
                syntax.number,
                syntax.string,
                syntax.comment,
                syntax.directive,
                syntax.attribute,
                syntax.module_name,
                syntax.port_name,
                syntax.parameter_name,
            ] {
                for fill in [palette.bg_inset, syntax.current_line, syntax.selection] {
                    let measured = contrast_ratio(ink, fill);
                    assert!(
                        measured >= 4.5,
                        "{theme} {ink:?} on {fill:?}: {measured:.2}:1"
                    );
                }
            }
            assert_eq!(palette.shadow_color, Color32::TRANSPARENT, "{theme}");
        }
    }

    #[test]
    fn the_ground_decides_which_inks_a_contrast_palette_takes() {
        assert!(ContrastInks::BLACK.has_dark_ground());
        assert!(CONTRAST_THEMES[2].1.has_dark_ground());
        assert!(!CONTRAST_THEMES[1].1.has_dark_ground());
    }

    #[test]
    fn text_tones_descend_against_every_ground() {
        for (mode, palette) in [("dark", INSTRUMENT_DARK), ("light", INSTRUMENT_LIGHT)] {
            for (surface, ground) in [
                ("bg_app", palette.bg_app),
                ("bg_panel", palette.bg_panel),
                ("bg_panel_2", palette.bg_panel_2),
                ("bg_inset", palette.bg_inset),
                ("bg_elevated", palette.bg_elevated),
            ] {
                let text = contrast_ratio(palette.text, ground);
                let dim = contrast_ratio(palette.text_dim, ground);
                let faint = contrast_ratio(palette.text_faint, ground);
                assert!(
                    dim < text,
                    "{mode} text_dim ({dim:.2}:1) must be fainter than text ({text:.2}:1) on {surface}"
                );
                assert!(
                    faint < dim,
                    "{mode} text_faint ({faint:.2}:1) must be fainter than text_dim ({dim:.2}:1) on {surface}"
                );
                assert!(
                    faint <= 0.90 * dim,
                    "{mode} text_faint ({faint:.2}:1) is not a step below text_dim ({dim:.2}:1) on {surface}, only {:.0}% of it",
                    100.0 * faint / dim
                );
                assert!(
                    faint >= 3.5,
                    "{mode} text_faint is unreadable on {surface} at {faint:.2}:1"
                );
            }
        }
    }

    /// Every syntax ink reads 4.5:1 off the code well, the caret's line and
    /// the selection fill, and plain text does on the selection too, in both
    /// palettes: a selected or current line never loses its code. The
    /// selection and the caret's line still stand off the well.
    #[test]
    fn syntax_inks_read_on_every_editor_fill() {
        for (mode, palette) in [("dark", INSTRUMENT_DARK), ("light", INSTRUMENT_LIGHT)] {
            let syntax = palette.syntax;
            let inks = [
                ("keyword", syntax.keyword),
                ("discipline", syntax.discipline),
                ("system", syntax.system),
                ("analog operator", syntax.analog_operator),
                ("contribution", syntax.contribution),
                ("number", syntax.number),
                ("string", syntax.string),
                ("comment", syntax.comment),
                ("directive", syntax.directive),
                ("attribute", syntax.attribute),
                ("module name", syntax.module_name),
                ("port name", syntax.port_name),
                ("parameter name", syntax.parameter_name),
                ("text", palette.text),
            ];
            for (fill, ground) in [
                ("code well", palette.bg_inset),
                ("caret line", syntax.current_line),
                ("selection", syntax.selection),
            ] {
                for (ink, color) in inks {
                    let ratio = contrast_ratio(color, ground);
                    assert!(
                        ratio >= 4.5,
                        "{mode} {ink} is only {ratio:.2}:1 on the {fill}"
                    );
                }
            }
            let selection = contrast_ratio(syntax.selection, palette.bg_inset);
            assert!(
                selection >= 1.2,
                "{mode} selection stands only {selection:.2}:1 off the code well"
            );
            assert_ne!(syntax.current_line, palette.bg_inset, "{mode}");
        }
    }
}
