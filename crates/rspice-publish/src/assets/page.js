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

function exportSvg(figure) {
  const source = figure.querySelector("svg");
  if (!source) return false;
  const clone = source.cloneNode(true);
  const roles = ["foreground", "secondary", "grid", "accent", "warning", "success"];
  for (let index = 0; index < 8; index += 1) roles.push(`trace-${index}`);
  const computed = getComputedStyle(root);
  const variables = roles.map((role) => `--${role}:${computed.getPropertyValue(`--${role}`).trim()}`).join(";");
  const rules = roles.map((role) => `.s-${role}{stroke:var(--${role})}.f-${role},.t-${role}{fill:var(--${role})}`).join("");
  const style = document.createElementNS("http://www.w3.org/2000/svg", "style");
  style.textContent = `:root{${variables}}${rules}`;
  clone.prepend(style);
  const bytes = new Blob([new XMLSerializer().serializeToString(clone)], { type: "image/svg+xml" });
  const url = URL.createObjectURL(bytes);
  const link = document.createElement("a");
  link.href = url;
  link.download = `${safeFilename(figure.querySelector("figcaption")?.textContent || "rspice-figure")}.svg`;
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), 0);
  return true;
}

for (const button of document.querySelectorAll("[data-figure-svg]")) {
  const figure = button.closest("figure");
  button.addEventListener("click", () => {
    if (!figure || !exportSvg(figure)) announce("The static SVG is unavailable");
  });
}
