#!/usr/bin/env python3
"""Generate the RSpice logo-mark geometry (SVG path data).

Like ui/palette.rs, the brand artwork is generated, not hand-drawn:
  - "Trace"      : an exact second-order underdamped step response,
                   rendered as a tapered ribbon (numerical offset curves).
  - "Monolith R" : the letter R lifted from the product's embedded
                   IBM Plex Sans SemiBold, with a sine band knockout.
  - "Convergence": concentric arcs with staggered sweeps, closing on a dot.
  - "Folded"     : the resistance element as a mitered, folded ribbon.

Prints SVG fragments to stdout; paste into design/logo-exploration.html.
"""

import math
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.pens.boundsPen import BoundsPen
from fontTools.misc.transform import Transform

FONT_DIR = r"C:\Users\James\Desktop\RSpice\crates\rspice-ui\assets\fonts"
SEMIBOLD = FONT_DIR + r"\IBMPlexSans-SemiBold.ttf"


def fmt(v: float) -> str:
    s = f"{v:.2f}".rstrip("0").rstrip(".")
    return "0" if s == "-0" else s


# ---------------------------------------------------------------- ribbons --
def _trim_cusps(chain, tangents):
    """Drop offset points that double back (w/2 exceeded curvature radius)."""
    out = [chain[0]]
    for i in range(1, len(chain)):
        px, py = out[-1]
        qx, qy = chain[i]
        tx, ty = tangents[i]
        if (qx - px) * tx + (qy - py) * ty > 1e-6:
            out.append(chain[i])
    if out[-1] != chain[-1]:
        out.append(chain[-1])
    return out


def ribbon_path(pts, widths):
    """Closed filled outline around a sampled centerline with round caps.

    pts:    [(x, y), ...] screen coords (y down)
    widths: full ribbon width at each sample
    """
    n = len(pts)
    top, bot, tans = [], [], []
    for i in range(n):
        x0, y0 = pts[max(i - 1, 0)]
        x1, y1 = pts[min(i + 1, n - 1)]
        tx, ty = x1 - x0, y1 - y0
        L = math.hypot(tx, ty) or 1.0
        tx, ty = tx / L, ty / L
        tans.append((tx, ty))
        nx, ny = ty, -tx  # screen-coords "up" normal for a left-to-right path
        h = widths[i] / 2.0
        px, py = pts[i]
        top.append((px + nx * h, py + ny * h))
        bot.append((px - nx * h, py - ny * h))
    top = _trim_cusps(top, tans)
    bot = _trim_cusps(bot, tans)

    r_end = widths[-1] / 2.0
    r_start = widths[0] / 2.0
    d = [f"M{fmt(top[0][0])} {fmt(top[0][1])}"]
    for p in top[1:]:
        d.append(f"L{fmt(p[0])} {fmt(p[1])}")
    d.append(f"A{fmt(r_end)} {fmt(r_end)} 0 0 1 {fmt(bot[-1][0])} {fmt(bot[-1][1])}")
    for p in reversed(bot[:-1]):
        d.append(f"L{fmt(p[0])} {fmt(p[1])}")
    d.append(f"A{fmt(r_start)} {fmt(r_start)} 0 0 1 {fmt(top[0][0])} {fmt(top[0][1])}")
    d.append("Z")
    return "".join(d)


# ------------------------------------------------------------------ trace --
def gen_trace():
    """Second-order underdamped step response: zeta = 0.35.

    Curvature must stay gentler than the ribbon half-width: with zeta=0.35
    the first overshoot's radius of curvature is ~4.7 units against a 3.5
    half-width, so the offset curves stay clean.
    """
    zeta = 0.35
    wd = math.sqrt(1 - zeta * zeta)
    phi = math.acos(zeta)

    def resp(tau):
        return 1 - math.exp(-zeta * tau) / wd * math.sin(wd * tau + phi)

    tau_s, tau_end = 0.55, 11.8   # start on the rising edge; end fully settled
    x0, x1 = 12.0, 82.0
    y_base, height = 78.0, 30.0   # settle line at y = 48

    n = 240
    pts, widths = [], []
    for i in range(n + 1):
        t = i / n
        tau = tau_s + (tau_end - tau_s) * t
        x = x0 + (x1 - x0) * t
        y = y_base - height * resp(tau)
        pts.append((x, y))
        widths.append(7.0 - 2.4 * t)  # taper 7 -> 4.6
    settle_y = y_base - height
    peak1 = 1 + math.exp(-zeta * math.pi / wd)
    print("<!-- TRACE: settle_y=%.2f  first_overshoot=%.3f -->" % (settle_y, peak1))
    print('<path class="ink-grad-x" d="%s"/>' % ribbon_path(pts, widths))
    print('<!-- settle line y=%.2f, dot at (86.5, %.2f) -->' % (settle_y, settle_y))


# -------------------------------------------------------------- letter R --
def glyph_path(font, ch, transform):
    gs = font.getGlyphSet()
    gname = font.getBestCmap()[ord(ch)]
    pen = SVGPathPen(gs, ntos=fmt)
    glyph = gs[gname]
    glyph.draw(TransformPen(pen, transform))
    return pen.getCommands(), glyph.width


def glyph_bounds(font, ch):
    gs = font.getGlyphSet()
    gname = font.getBestCmap()[ord(ch)]
    bp = BoundsPen(gs)
    gs[gname].draw(bp)
    return bp.bounds  # font units, y up


def gen_monolith_r():
    font = TTFont(SEMIBOLD)
    upm = font["head"].unitsPerEm
    cap = font["OS/2"].sCapHeight or 0.7 * upm
    xmin, ymin, xmax, ymax = glyph_bounds(font, "R")

    target_cap = 62.0
    s = target_cap / cap
    gw = (xmax - xmin) * s
    tx = (96 - gw) / 2 - xmin * s
    baseline = (96 - target_cap) / 2 + target_cap  # 79
    t = Transform(s, 0, 0, -s, tx, baseline)
    d, adv = glyph_path(font, "R", t)
    print("<!-- MONOLITH R: cap=%d upm=%d scale=%.4f baseline=%.1f bboxW=%.1f -->"
          % (cap, upm, s, baseline, gw))
    print('<path class="ink-fg" d="%s"/>' % d)

    # sine band through the lower waist. Phase solved so the slit crosses the
    # stem at y~62 (clear of the bowl) and the leg at its visual midpoint.
    yc, amp, lam = 63.5, 4.0, 52.0
    bx0, bx1 = 4.0, 92.0
    phase = 0.384
    n = 220
    pts, widths = [], []
    for i in range(n + 1):
        t01 = i / n
        x = bx0 + (bx1 - bx0) * t01
        y = yc + amp * math.sin(2 * math.pi * (x - bx0) / lam + phase)
        pts.append((x, y))
        # gentle taper at both ends
        end_ease = min(1.0, 4.0 * t01 * (1 - t01) + 0.55)
        widths.append(6.6 * end_ease)
    print('<!-- band yc=%.2f -->' % yc)
    print('<path class="band" d="%s"/>' % ribbon_path(pts, widths))


# ------------------------------------------------------------ convergence --
def gen_convergence():
    cx, cy = 48.0, 48.0

    def pt(r, deg):
        a = math.radians(deg)
        return cx + r * math.cos(a), cy - r * math.sin(a)

    arcs = [
        # r, start_deg, sweep_deg (clockwise on screen), class
        (31.0, 100.0, 235.0, "arc-deep"),
        (20.5, 65.0, 195.0, "arc-mid"),
        (10.5, 30.0, 140.0, "arc-hi"),
    ]
    print("<!-- CONVERGENCE -->")
    for r, a0, sweep, cls in arcs:
        a1 = a0 - sweep  # decreasing angle = clockwise on screen
        x0, y0 = pt(r, a0)
        x1, y1 = pt(r, a1)
        large = 1 if sweep > 180 else 0
        print('<path class="%s" d="M%s %s A%s %s 0 %d 1 %s %s"/>'
              % (cls, fmt(x0), fmt(y0), fmt(r), fmt(r), large, fmt(x1), fmt(y1)))
    print('<circle class="dot-hi" cx="48" cy="48" r="5"/>')


# ----------------------------------------------------------------- folded --
def gen_folded():
    """Resistance zigzag as a folded strip: constant HORIZONTAL thickness, so
    fold creases are clean vertical joints (no miter explosion on the sharp
    fold angles)."""
    verts = [(13, 48), (23, 27), (39, 69), (55, 27), (71, 69), (81, 48)]
    half = 4.5

    nseg = len(verts) - 1
    print("<!-- FOLDED ELEMENT (alternating faces) -->")
    for i in range(nseg):
        cls = "face-a" if i % 2 == 0 else "face-b"
        (x0, y0), (x1, y1) = verts[i], verts[i + 1]
        print('<path class="%s" d="M%s %s L%s %s L%s %s L%s %s Z"/>'
              % (cls,
                 fmt(x0 - half), fmt(y0), fmt(x1 - half), fmt(y1),
                 fmt(x1 + half), fmt(y1), fmt(x0 + half), fmt(y0)))


# --------------------------------------------------------------- wordmark --
def gen_wordmark():
    font = TTFont(SEMIBOLD)
    upm = font["head"].unitsPerEm
    cap = font["OS/2"].sCapHeight
    text = "RSpice"
    target_cap = 100.0  # generous master size; scaled at use sites
    s = target_cap / cap
    baseline = target_cap  # viewBox top at cap height; descender below

    paths = []
    pen_x = 0.0
    for ch in text:
        t = Transform(s, 0, 0, -s, pen_x, baseline)
        d, adv = glyph_path(font, ch, t)
        if d:
            paths.append(d)
        pen_x += adv * s
    desc = -font["OS/2"].sTypoDescender * s
    print("<!-- WORDMARK 'RSpice': width=%.2f capline=0 baseline=%.1f descent=%.1f -->"
          % (pen_x, baseline, desc))
    print('<path class="ink-fg" d="%s"/>' % "".join(paths))


if __name__ == "__main__":
    import io, sys
    sys.stdout = io.TextIOWrapper(
        open(__file__.replace("gen_logo_marks.py", "marks_out.txt"), "wb"),
        encoding="utf-8")
    gen_trace()
    print()
    gen_monolith_r()
    print()
    gen_convergence()
    print()
    gen_folded()
    print()
    gen_wordmark()
    sys.stdout.flush()
