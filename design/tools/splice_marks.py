#!/usr/bin/env python3
"""Splice generated mark geometry (marks_out.txt) into logo-exploration.html."""

import io
import re
import sys

TOOLS = r"C:\Users\James\Desktop\RSpice\design\tools"
HTML = r"C:\Users\James\Desktop\RSpice\design\logo-exploration.html"

out = io.open(TOOLS + r"\marks_out.txt", encoding="utf-8").read()

def grab(pattern):
    m = re.search(pattern, out)
    if not m:
        sys.exit("pattern not found: " + pattern)
    return m.group(1)

trace = grab(r'class="ink-grad-x" d="([^"]+)"')
band = grab(r'class="band" d="([^"]+)"')
# first ink-fg after the MONOLITH comment is the R; the one after WORDMARK is the wordmark
rpath = grab(r'MONOLITH[^\n]*\n<path class="ink-fg" d="([^"]+)"')
wm = grab(r"WORDMARK[^\n]*\n" + r'<path class="ink-fg" d="([^"]+)"')

html = io.open(HTML, encoding="utf-8").read()
for token, value in (("__TRACE__", trace), ("__BAND__", band),
                     ("__RPATH__", rpath), ("__WM__", wm)):
    if token not in html:
        sys.exit("token missing from html: " + token)
    html = html.replace(token, value)

io.open(HTML, "w", encoding="utf-8").write(html)
print("spliced: trace=%d band=%d r=%d wm=%d chars"
      % (len(trace), len(band), len(rpath), len(wm)))
