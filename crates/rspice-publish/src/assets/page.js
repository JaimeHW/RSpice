// Progressive page chrome for sealed RSpice publications. The document
// remains complete when this module is unavailable; this only adds tabs,
// local theme preference, and share conveniences.

const root = document.documentElement;
const panels = Array.from(document.querySelectorAll("[data-panel]"));
const tabs = Array.from(document.querySelectorAll("[data-tab]"));
const toast = document.querySelector("[data-toast]");

function panelForHash(hash) {
  const id = decodeURIComponent((hash || "").replace(/^#/, ""));
  if (!id) {
    const preferred = document.querySelector("[data-default-panel]")?.dataset.defaultPanel;
    return (preferred && document.getElementById(preferred)) || panels[0] || null;
  }
  const target = document.getElementById(id);
  return target ? target.closest("[data-panel]") : null;
}

function activate(panel, updateHistory = false) {
  if (!panel) return;
  for (const candidate of panels) {
    if (candidate === panel) candidate.setAttribute("data-active", "");
    else candidate.removeAttribute("data-active");
  }
  for (const tab of tabs) {
    if (tab.getAttribute("href") === `#${panel.id}`) tab.setAttribute("aria-current", "page");
    else tab.removeAttribute("aria-current");
  }
  if (updateHistory && location.hash !== `#${panel.id}`) {
    history.pushState(null, "", `#${panel.id}`);
  }
}

function announce(message) {
  if (!toast) return;
  toast.textContent = message;
  toast.hidden = false;
  clearTimeout(announce.timer);
  announce.timer = setTimeout(() => { toast.hidden = true; }, 2600);
}

function storedTheme() {
  try { return localStorage.getItem("rspice-publication-theme") || "system"; }
  catch { return "system"; }
}

function applyTheme(theme) {
  if (theme === "light" || theme === "dark") root.dataset.theme = theme;
  else delete root.dataset.theme;
  const button = document.querySelector("[data-theme-toggle]");
  if (button) {
    const label = theme[0].toUpperCase() + theme.slice(1);
    button.setAttribute("aria-label", `Theme: ${label}. Activate to change.`);
    const text = button.querySelector("[data-theme-label]");
    if (text) text.textContent = label;
  }
}

root.classList.add("js-ready");
for (const element of document.querySelectorAll("[data-js-only]")) element.hidden = false;
const embeddedFigurePanel = root.hasAttribute("data-rspice-embed") && !location.hash
  ? panels.find((panel) => panel.querySelector("figure"))
  : null;
activate(embeddedFigurePanel || panelForHash(location.hash));
applyTheme(storedTheme());

for (const tab of tabs) {
  tab.addEventListener("click", (event) => {
    const panel = panelForHash(tab.hash);
    if (!panel) return;
    event.preventDefault();
    activate(panel, true);
    panel.focus({ preventScroll: true });
    panel.scrollIntoView({ behavior: "smooth", block: "start" });
  });
}

addEventListener("hashchange", () => activate(panelForHash(location.hash)));
addEventListener("popstate", () => activate(panelForHash(location.hash)));

const themeButton = document.querySelector("[data-theme-toggle]");
if (themeButton) {
  themeButton.addEventListener("click", () => {
    const themes = ["system", "dark", "light"];
    const current = storedTheme();
    const next = themes[(themes.indexOf(current) + 1) % themes.length];
    try { localStorage.setItem("rspice-publication-theme", next); } catch {}
    applyTheme(next);
    announce(`Theme set to ${next}`);
  });
}

const shareButton = document.querySelector("[data-share]");
if (shareButton) {
  shareButton.addEventListener("click", async () => {
    const canonical = document.querySelector('link[rel="canonical"]')?.href || location.href;
    const data = { title: document.title, url: canonical };
    try {
      if (navigator.share) {
        await navigator.share(data);
        return;
      }
      await navigator.clipboard.writeText(canonical);
      announce("Publication link copied");
    } catch (error) {
      if (error && error.name === "AbortError") return;
      announce("Could not copy the link");
    }
  });
}

function escapeAttribute(value) {
  return value.replaceAll("&", "&amp;").replaceAll('"', "&quot;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
}

const contextLink = document.querySelector('link[rel="rspice-publication-context"]');
const embedButton = document.querySelector("[data-embed-copy]");
if (embedButton && contextLink) {
  embedButton.addEventListener("click", async () => {
    const base = new URL(".", contextLink.href);
    const embedUrl = new URL("embed", base).href;
    const title = (document.querySelector("h1")?.textContent || "RSpice circuit").trim();
    const markup = `<iframe src="${escapeAttribute(embedUrl)}" title="${escapeAttribute(title)}" loading="lazy" width="100%" height="720" style="border:0" allowfullscreen></iframe>`;
    try {
      await navigator.clipboard.writeText(markup);
      announce("Embed code copied");
    } catch {
      announce("Could not copy the embed code");
    }
  });
}

function safeSameOriginUrl(value, prefix) {
  if (typeof value !== "string") return null;
  try {
    const url = new URL(value, location.origin);
    return url.origin === location.origin && url.pathname.startsWith(prefix) ? url : null;
  } catch {
    return null;
  }
}

function versionLink(label, value) {
  const url = safeSameOriginUrl(value, "/c/");
  if (!url) return null;
  const link = document.createElement("a");
  link.className = "button";
  link.href = url.href;
  link.textContent = label;
  return link;
}

async function loadCloudContext() {
  if (!contextLink) return;
  try {
    const response = await fetch(contextLink.href, {
      cache: "no-cache",
      credentials: "omit",
      headers: { Accept: "application/json" },
    });
    if (!response.ok) return;
    const context = await response.json();
    if (!context || typeof context !== "object" || !context.version) return;
    const number = Number(context.version.number);
    const total = Number(context.version.total);
    if (!Number.isSafeInteger(number) || !Number.isSafeInteger(total) || number < 1 || total < number) return;

    const section = document.querySelector("[data-cloud-context]");
    const version = document.querySelector("[data-cloud-version]");
    const actions = document.querySelector("[data-version-actions]");
    if (!section || !version || !actions) return;
    version.textContent = `Version ${number} of ${total}`;
    for (const [label, value] of [["Previous", context.version.previous_url], ["Next", context.version.next_url]]) {
      const link = versionLink(label, value);
      if (link) actions.append(link);
    }

    const list = document.querySelector("[data-cloud-artifacts]");
    const wrap = document.querySelector("[data-cloud-artifacts-wrap]");
    if (list && wrap && Array.isArray(context.artifacts)) {
      for (const artifact of context.artifacts) {
        const download = safeSameOriginUrl(artifact?.download_url, "/api/v1/publications/");
        if (!download || typeof artifact?.label !== "string" || typeof artifact?.detail !== "string") continue;
        const item = document.createElement("li");
        const copy = document.createElement("div");
        const label = document.createElement("strong");
        const detail = document.createElement("span");
        const link = document.createElement("a");
        label.textContent = artifact.label;
        detail.textContent = artifact.detail;
        copy.append(label, detail);
        link.className = "button";
        link.href = download.href;
        link.textContent = "Download";
        item.append(copy, link);
        list.append(item);
      }
      wrap.hidden = list.childElementCount === 0;
    }
    section.hidden = false;
  } catch {}
}

void loadCloudContext();

const schematicTags = Array.from(document.querySelectorAll("svg g[data-instance], svg g[data-net]"));

function selectSchematicTag(element) {
  for (const tag of schematicTags) tag.classList.toggle("is-selected", tag === element);
}

for (const tag of schematicTags) {
  tag.addEventListener("click", () => selectSchematicTag(tag));
  tag.addEventListener("keydown", (event) => {
    if (event.key !== "Enter" && event.key !== " ") return;
    event.preventDefault();
    selectSchematicTag(tag);
  });
}

for (const button of document.querySelectorAll("[data-figure-fullscreen]")) {
  const figure = button.closest("figure");
  if (!figure || typeof figure.requestFullscreen !== "function") {
    button.hidden = true;
    continue;
  }
  button.addEventListener("click", async () => {
    try {
      if (document.fullscreenElement === figure) await document.exitFullscreen();
      else await figure.requestFullscreen();
    } catch {
      announce("Fullscreen is unavailable");
    }
  });
  document.addEventListener("fullscreenchange", () => {
    button.textContent = document.fullscreenElement === figure ? "Exit fullscreen" : "Fullscreen";
  });
}

function safeFilename(value) {
  const normalized = value.toLocaleLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
  return normalized || "rspice-figure";
}

const PAINT_ROLES = ["foreground", "secondary", "grid", "accent", "warning", "success"];
for (let index = 0; index < 8; index += 1) PAINT_ROLES.push(`trace-${index}`);
const PAINT_CLASS = /^([sft])-(.+)$/;

function pageRules() {
  const sheet = document.querySelector('link[rel="stylesheet"][href$="page.css"]')?.sheet;
  try { return { href: sheet.href, rules: Array.from(sheet.cssRules) }; }
  catch { return { href: location.href, rules: [] }; }
}

// A downloaded figure is a document, so it takes the light palette the page
// opens in whatever theme the reader chose: light ink on a transparent ground
// would vanish on paper.
function exportPalette(rules) {
  const light = rules.find((rule) => rule.selectorText === ":root")?.style;
  const current = getComputedStyle(root);
  return new Map(PAINT_ROLES.map((role) => {
    const name = `--${role}`;
    return [role, light?.getPropertyValue(name).trim() || current.getPropertyValue(name).trim()];
  }));
}

function dataUrl(blob) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result);
    reader.onerror = () => reject(reader.error);
    reader.readAsDataURL(blob);
  });
}

// The page's faces that the figure letters in, inlined so the file keeps its
// lettering and weights on a machine without them. A face that cannot be read
// is left out and its text falls back along the family list.
async function embeddedFaces(stylesheet, clone) {
  const texts = Array.from(clone.querySelectorAll("text"));
  const faces = [];
  for (const rule of stylesheet.rules) {
    if (!(rule instanceof CSSFontFaceRule)) continue;
    const family = rule.style.getPropertyValue("font-family").replace(/^["']|["']$/g, "");
    const weight = rule.style.getPropertyValue("font-weight") || "400";
    const used = texts.some((text) => (text.getAttribute("font-family") || "").startsWith(`'${family}'`)
      && (text.getAttribute("font-weight") || "400") === weight);
    const src = rule.style.getPropertyValue("src");
    const path = /url\(\s*(["']?)(.*?)\1\s*\)/.exec(src)?.[2];
    if (!used || !path) continue;
    try {
      const response = await fetch(new URL(path, stylesheet.href));
      if (!response.ok) continue;
      const format = /format\([^)]*\)/.exec(src)?.[0] || "";
      faces.push(`@font-face{font-family:"${family}";font-weight:${weight};src:url("${await dataUrl(await response.blob())}") ${format}}`);
    } catch {}
  }
  return faces.join("");
}

// The file is the static figure as published: paint resolved to plain
// presentation attributes, which every SVG reader honours where many ignore
// CSS variables, and none of the page's interaction state.
async function exportSvg(figure) {
  const source = figure.querySelector("svg");
  if (!source) return false;
  const clone = source.cloneNode(true);
  // The loader hides the static figure while the interactive view is open.
  clone.removeAttribute("style");
  const stylesheet = pageRules();
  const palette = exportPalette(stylesheet.rules);
  for (const element of clone.querySelectorAll("[class]")) {
    const kept = [];
    for (const name of element.classList) {
      const paint = PAINT_CLASS.exec(name);
      const colour = paint && palette.get(paint[2]);
      if (colour) element.setAttribute(paint[1] === "s" ? "stroke" : "fill", colour);
      else if (name !== "is-selected") kept.push(name);
    }
    if (kept.length) element.setAttribute("class", kept.join(" "));
    else element.removeAttribute("class");
  }
  for (const tag of clone.querySelectorAll("[tabindex]")) {
    for (const name of ["tabindex", "role", "aria-label"]) tag.removeAttribute(name);
  }
  const faces = await embeddedFaces(stylesheet, clone);
  if (faces) {
    const style = document.createElementNS("http://www.w3.org/2000/svg", "style");
    style.textContent = faces;
    clone.prepend(style);
  }
  const bytes = new Blob([new XMLSerializer().serializeToString(clone)], { type: "image/svg+xml" });
  const url = URL.createObjectURL(bytes);
  const link = document.createElement("a");
  link.href = url;
  link.download = `${safeFilename(figure.querySelector("figcaption")?.textContent || "rspice-figure")}.svg`;
  link.click();
  // Safari and Firefox read the blob after click() returns, so revoking it
  // at once can cancel the download.
  setTimeout(() => URL.revokeObjectURL(url), 60_000);
  return true;
}

for (const button of document.querySelectorAll("[data-figure-svg]")) {
  const figure = button.closest("figure");
  button.addEventListener("click", async () => {
    button.disabled = true;
    try {
      if (!figure || !(await exportSvg(figure))) announce("The static SVG is unavailable");
    } catch {
      announce("The static SVG is unavailable");
    } finally {
      button.disabled = false;
    }
  });
}
