// Progressive page chrome for sealed RSpice publications. The document
// remains complete when this module is unavailable; this only adds tabs,
// local theme preference, and share conveniences.

const root = document.documentElement;
const panels = Array.from(document.querySelectorAll("[data-panel]"));
const tabs = Array.from(document.querySelectorAll("[data-tab]"));
const toast = document.querySelector("[data-toast]");

function panelForHash(hash) {
  const id = decodeURIComponent((hash || "").replace(/^#/, ""));
  if (!id) return panels[0] || null;
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
activate(panelForHash(location.hash));
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
    const data = { title: document.title, url: location.href };
    try {
      if (navigator.share) {
        await navigator.share(data);
        return;
      }
      await navigator.clipboard.writeText(location.href);
      announce("Publication link copied");
    } catch (error) {
      if (error && error.name === "AbortError") return;
      announce("Could not copy the link");
    }
  });
}

function tagLabel(element) {
  if (element.dataset.instance) return `Component ${element.dataset.instance}`;
  if (element.dataset.net) return `Net ${element.dataset.net}`;
  return "Schematic object";
}

const schematicTags = Array.from(document.querySelectorAll("svg g[data-instance], svg g[data-net]"));
const schematicSearch = document.querySelector("[data-schematic-search]");
const schematicStatus = document.querySelector("[data-schematic-status]");

function selectSchematicTag(element) {
  for (const tag of schematicTags) tag.classList.toggle("is-selected", tag === element);
  if (schematicStatus) schematicStatus.textContent = `${tagLabel(element)} selected`;
}

for (const tag of schematicTags) {
  tag.addEventListener("click", () => selectSchematicTag(tag));
  tag.addEventListener("keydown", (event) => {
    if (event.key !== "Enter" && event.key !== " ") return;
    event.preventDefault();
    selectSchematicTag(tag);
  });
}

function searchSchematic() {
  const query = (schematicSearch?.value || "").trim().toLocaleLowerCase();
  let matches = 0;
  for (const tag of schematicTags) {
    const match = !query || tagLabel(tag).toLocaleLowerCase().includes(query);
    tag.classList.toggle("search-match", Boolean(query) && match);
    tag.classList.toggle("search-dimmed", Boolean(query) && !match);
    if (query && match) matches += 1;
  }
  if (schematicStatus) {
    schematicStatus.textContent = query
      ? `${matches} matching tagged object${matches === 1 ? "" : "s"}`
      : "Select a tagged component or net to inspect it.";
  }
  return schematicTags.find((tag) => tag.classList.contains("search-match"));
}

if (schematicSearch) {
  schematicSearch.addEventListener("input", searchSchematic);
  schematicSearch.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      schematicSearch.value = "";
      searchSchematic();
      return;
    }
    if (event.key !== "Enter") return;
    const first = searchSchematic();
    if (first) {
      selectSchematicTag(first);
      first.focus({ preventScroll: true });
      first.closest("figure")?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
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
