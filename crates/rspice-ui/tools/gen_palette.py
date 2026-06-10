#!/usr/bin/env python3
"""Generate src/ui/palette.rs from the VOLTA design-token tables.

The token values are transcribed verbatim from the professionally designed
"VOLTA IDE shell" specification (oklch color space, CSS Color 4). This script
performs the oklch -> sRGB conversion using the CSS Color 4 reference math and
emits a Rust module with one `Palette` const per (direction, mode).

Run from the crate root:  py tools/gen_palette.py
"""

import math
import os

# --------------------------------------------------------------------------
# oklch -> sRGB (CSS Color 4 reference conversion)
# --------------------------------------------------------------------------


def oklch_to_srgb(lightness: float, chroma: float, hue_deg: float):
    """Convert an oklch color (L in 0..1) to 8-bit sRGB with gamut clipping."""
    a = chroma * math.cos(math.radians(hue_deg))
    b = chroma * math.sin(math.radians(hue_deg))

    l_ = lightness + 0.3963377774 * a + 0.2158037573 * b
    m_ = lightness - 0.1055613458 * a - 0.0638541728 * b
    s_ = lightness - 0.0894841775 * a - 1.2914855480 * b

    l, m, s = l_**3, m_**3, s_**3

    r_lin = +4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s
    g_lin = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s
    b_lin = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s

    def encode(x: float) -> int:
        x = min(max(x, 0.0), 1.0)
        x = x * 12.92 if x <= 0.0031308 else 1.055 * x ** (1 / 2.4) - 0.055
        return round(x * 255)

    return encode(r_lin), encode(g_lin), encode(b_lin)


def rgba(spec):
    """spec = (L%, C, H) or (L%, C, H, alpha). Returns (r, g, b, a) unmultiplied."""
    lightness, chroma, hue = spec[0] / 100.0, spec[1], spec[2]
    alpha = spec[3] if len(spec) > 3 else 1.0
    r, g, b = oklch_to_srgb(lightness, chroma, hue)
    return r, g, b, round(alpha * 255)


# --------------------------------------------------------------------------
# Token tables — transcribed from the design CSS, one dict per palette.
# Order of fields here defines field order in the generated struct literal.
# --------------------------------------------------------------------------

FIELDS = [
    "bg_app", "bg_panel", "bg_inset", "bg_elevated", "bg_hover", "bg_active",
    "border", "border_strong",
    "text", "text_dim", "text_faint",
    "accent", "accent_ink", "accent_dim",
    "ok", "warn", "err",
    "canvas_bg", "canvas_grid",
    "wire", "symbol", "net_label",
]
TRACES = ["trace_1", "trace_2", "trace_3", "trace_4", "trace_5", "trace_6"]

PALETTES = {
    # ---------------- A: Instrument · dark
    "INSTRUMENT_DARK": {
        "bg_app": (20, 0.012, 235), "bg_panel": (24, 0.012, 235),
        "bg_inset": (17.5, 0.012, 235), "bg_elevated": (27.5, 0.014, 235),
        "bg_hover": (29.5, 0.014, 235), "bg_active": (34, 0.02, 235),
        "border": (31, 0.012, 235), "border_strong": (41, 0.012, 235),
        "text": (89, 0.006, 235), "text_dim": (68, 0.01, 235), "text_faint": (52, 0.01, 235),
        "accent": (79, 0.15, 152), "accent_ink": (20, 0.04, 152),
        "accent_dim": (79, 0.15, 152, 0.14),
        "ok": (75, 0.14, 152), "warn": (80, 0.13, 85), "err": (68, 0.17, 25),
        "canvas_bg": (15, 0.012, 235), "canvas_grid": (25, 0.012, 235),
        "wire": (79, 0.08, 152), "symbol": (86, 0.01, 235), "net_label": (78, 0.12, 152),
        "trace_1": (78, 0.16, 152), "trace_2": (78, 0.12, 220), "trace_3": (80, 0.14, 85),
        "trace_4": (72, 0.17, 345), "trace_5": (72, 0.12, 260), "trace_6": (70, 0.16, 25),
        "shadow": (0, 0, 0, 0.45), "shadow_geom": (6, 24),
    },
    # ---------------- A: Instrument · light
    "INSTRUMENT_LIGHT": {
        "bg_app": (94, 0.005, 235), "bg_panel": (97.5, 0.003, 235),
        "bg_inset": (92, 0.005, 235), "bg_elevated": (99.3, 0.002, 235),
        "bg_hover": (93.5, 0.006, 235), "bg_active": (89, 0.012, 235),
        "border": (87, 0.006, 235), "border_strong": (74, 0.008, 235),
        "text": (26, 0.01, 235), "text_dim": (46, 0.01, 235), "text_faint": (61, 0.008, 235),
        "accent": (51, 0.14, 152), "accent_ink": (98.5, 0.005, 152),
        "accent_dim": (51, 0.14, 152, 0.12),
        "ok": (54, 0.13, 152), "warn": (60, 0.13, 80), "err": (54, 0.17, 25),
        "canvas_bg": (99.3, 0.002, 235), "canvas_grid": (91.5, 0.004, 235),
        "wire": (45, 0.11, 152), "symbol": (31, 0.01, 235), "net_label": (47, 0.13, 152),
        "trace_1": (55, 0.15, 152), "trace_2": (55, 0.12, 220), "trace_3": (60, 0.13, 80),
        "trace_4": (54, 0.17, 345), "trace_5": (52, 0.15, 260), "trace_6": (54, 0.17, 25),
        "shadow": (20, 0.01, 235, 0.18), "shadow_geom": (6, 24),
    },
    # ---------------- B: Meridian · dark
    "MERIDIAN_DARK": {
        "bg_app": (21.5, 0.015, 262), "bg_panel": (25, 0.015, 262),
        "bg_inset": (18.5, 0.015, 262), "bg_elevated": (28.5, 0.017, 262),
        "bg_hover": (30.5, 0.018, 262), "bg_active": (35, 0.025, 262),
        "border": (32, 0.016, 262), "border_strong": (42, 0.018, 262),
        "text": (91, 0.008, 262), "text_dim": (70, 0.012, 262), "text_faint": (54, 0.014, 262),
        "accent": (64, 0.18, 262), "accent_ink": (97.5, 0.008, 262),
        "accent_dim": (64, 0.18, 262, 0.16),
        "ok": (74, 0.14, 155), "warn": (80, 0.13, 85), "err": (68, 0.17, 25),
        "canvas_bg": (17, 0.014, 262), "canvas_grid": (26.5, 0.015, 262),
        "wire": (80, 0.07, 230), "symbol": (88, 0.01, 262), "net_label": (75, 0.12, 262),
        "trace_1": (75, 0.13, 230), "trace_2": (76, 0.14, 152), "trace_3": (80, 0.14, 85),
        "trace_4": (72, 0.17, 345), "trace_5": (70, 0.15, 262), "trace_6": (70, 0.16, 25),
        "shadow": (0, 0, 0, 0.45), "shadow_geom": (8, 28),
    },
    # ---------------- B: Meridian · light
    "MERIDIAN_LIGHT": {
        "bg_app": (95, 0.004, 250), "bg_panel": (98.2, 0.002, 250),
        "bg_inset": (92.8, 0.004, 250), "bg_elevated": (99.5, 0.001, 250),
        "bg_hover": (94, 0.005, 250), "bg_active": (89.5, 0.012, 255),
        "border": (88, 0.005, 250), "border_strong": (75, 0.008, 250),
        "text": (25, 0.012, 262), "text_dim": (46, 0.014, 262), "text_faint": (61, 0.012, 262),
        "accent": (49, 0.19, 262), "accent_ink": (98.5, 0.005, 262),
        "accent_dim": (49, 0.19, 262, 0.11),
        "ok": (54, 0.13, 155), "warn": (60, 0.13, 80), "err": (54, 0.17, 25),
        "canvas_bg": (99.5, 0.001, 250), "canvas_grid": (92, 0.004, 250),
        "wire": (46, 0.1, 240), "symbol": (30, 0.012, 262), "net_label": (48, 0.16, 262),
        "trace_1": (50, 0.13, 240), "trace_2": (55, 0.13, 152), "trace_3": (60, 0.13, 80),
        "trace_4": (54, 0.17, 345), "trace_5": (48, 0.19, 262), "trace_6": (54, 0.17, 25),
        "shadow": (25, 0.02, 262, 0.16), "shadow_geom": (8, 28),
    },
    # ---------------- C: Graphite · dark
    "GRAPHITE_DARK": {
        "bg_app": (18.5, 0, 0), "bg_panel": (22, 0, 0),
        "bg_inset": (15.5, 0, 0), "bg_elevated": (25.5, 0, 0),
        "bg_hover": (27.5, 0, 0), "bg_active": (32, 0, 0),
        "border": (30, 0, 0), "border_strong": (41, 0, 0),
        "text": (90, 0, 0), "text_dim": (66, 0, 0), "text_faint": (50, 0, 0),
        "accent": (78, 0.14, 80), "accent_ink": (20, 0.05, 80),
        "accent_dim": (78, 0.14, 80, 0.15),
        "ok": (75, 0.13, 150), "warn": (78, 0.14, 80), "err": (66, 0.17, 25),
        "canvas_bg": (13.5, 0, 0), "canvas_grid": (23.5, 0, 0),
        "wire": (78, 0.03, 80), "symbol": (88, 0, 0), "net_label": (76, 0.12, 80),
        "trace_1": (80, 0.14, 80), "trace_2": (75, 0.11, 230), "trace_3": (76, 0.13, 150),
        "trace_4": (72, 0.16, 350), "trace_5": (68, 0.05, 80), "trace_6": (66, 0.16, 25),
        "shadow": (0, 0, 0, 0.5), "shadow_geom": (6, 24),
    },
    # ---------------- C: Graphite · light
    "GRAPHITE_LIGHT": {
        "bg_app": (95.5, 0.006, 85), "bg_panel": (98.2, 0.004, 85),
        "bg_inset": (92.8, 0.007, 85), "bg_elevated": (99.4, 0.002, 85),
        "bg_hover": (94, 0.007, 85), "bg_active": (89.5, 0.012, 85),
        "border": (87, 0.008, 85), "border_strong": (73, 0.012, 85),
        "text": (24, 0.005, 85), "text_dim": (45, 0.008, 85), "text_faint": (60, 0.008, 85),
        "accent": (55, 0.13, 70), "accent_ink": (98.8, 0.004, 85),
        "accent_dim": (55, 0.13, 70, 0.13),
        "ok": (53, 0.12, 150), "warn": (58, 0.13, 75), "err": (54, 0.17, 25),
        "canvas_bg": (99.2, 0.003, 85), "canvas_grid": (91.5, 0.007, 85),
        "wire": (42, 0.06, 75), "symbol": (30, 0.005, 85), "net_label": (52, 0.13, 70),
        "trace_1": (58, 0.13, 70), "trace_2": (50, 0.12, 240), "trace_3": (54, 0.12, 150),
        "trace_4": (54, 0.16, 350), "trace_5": (40, 0.03, 80), "trace_6": (54, 0.17, 25),
        "shadow": (30, 0.01, 85, 0.16), "shadow_geom": (6, 24),
    },
}


def color_expr(spec) -> str:
    r, g, b, a = rgba(spec)
    if a == 255:
        return f"Color32::from_rgb({r}, {g}, {b})"
    # Premultiply so the const constructor can be used.
    pr = round(r * a / 255)
    pg = round(g * a / 255)
    pb = round(b * a / 255)
    return f"Color32::from_rgba_premultiplied({pr}, {pg}, {pb}, {a})"


def emit() -> str:
    out = []
    out.append("//! Color palettes for the three design directions × dark/light modes.")
    out.append("//!")
    out.append("//! GENERATED by `tools/gen_palette.py` from the VOLTA design-token tables")
    out.append("//! (oklch, CSS Color 4) — do not edit values by hand; rerun the script.")
    out.append("")
    out.append("use egui::Color32;")
    out.append("")
    out.append("/// One complete color palette (a design direction in one mode).")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    out.append("pub struct Palette {")
    docs = {
        "bg_app": "Application background (outermost chrome).",
        "bg_panel": "Panel surfaces (side panels, bars, cards).",
        "bg_inset": "Inset wells (inputs, embedded lists).",
        "bg_elevated": "Elevated surfaces (menus, popovers, toasts).",
        "bg_hover": "Hover fill for interactive rows/buttons.",
        "bg_active": "Pressed/active fill.",
        "border": "Hairline borders between surfaces.",
        "border_strong": "Emphasized borders (focused inputs, popovers).",
        "text": "Primary text.",
        "text_dim": "Secondary text.",
        "text_faint": "Tertiary text (metadata, disabled).",
        "accent": "Accent (selection, primary actions).",
        "accent_ink": "Text/icon color on accent fills.",
        "accent_dim": "Translucent accent wash (selected rows, toggles).",
        "ok": "Success state.",
        "warn": "Warning state.",
        "err": "Error state.",
        "canvas_bg": "Document-well background (schematic, plots, code).",
        "canvas_grid": "Canvas grid dots / plot gridlines.",
        "wire": "Schematic wire stroke.",
        "symbol": "Schematic device symbol stroke.",
        "net_label": "Schematic net-name labels.",
    }
    for f in FIELDS:
        out.append(f"    /// {docs[f]}")
        out.append(f"    pub {f}: Color32,")
    out.append("    /// Waveform trace cycle (assignment order for new traces).")
    out.append("    pub traces: [Color32; 6],")
    out.append("    /// Popover/menu shadow color (translucent).")
    out.append("    pub shadow_color: Color32,")
    out.append("    /// Popover shadow geometry: (y-offset, blur) in points.")
    out.append("    pub shadow_geom: (i8, u8),")
    out.append("}")
    out.append("")

    for name, p in PALETTES.items():
        direction, mode = name.split("_", 1)
        out.append(f"/// {direction.title()} · {mode.lower()}")
        out.append(f"pub const {name}: Palette = Palette {{")
        for f in FIELDS:
            out.append(f"    {f}: {color_expr(p[f])},")
        traces = ", ".join(color_expr(p[t]) for t in TRACES)
        out.append("    traces: [")
        for t in TRACES:
            out.append(f"        {color_expr(p[t])},")
        out.append("    ],")
        out.append(f"    shadow_color: {color_expr(p['shadow'])},")
        oy, blur = p["shadow_geom"]
        out.append(f"    shadow_geom: ({oy}, {blur}),")
        out.append("};")
        out.append("")
    return "\n".join(out)


if __name__ == "__main__":
    here = os.path.dirname(os.path.abspath(__file__))
    target = os.path.join(here, "..", "src", "ui", "palette.rs")
    os.makedirs(os.path.dirname(target), exist_ok=True)
    src = emit()
    with open(target, "w", encoding="utf-8", newline="\n") as fh:
        fh.write(src + "\n")
    print(f"wrote {os.path.normpath(target)} ({len(src.splitlines())} lines)")
