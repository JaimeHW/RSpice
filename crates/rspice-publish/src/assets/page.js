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
