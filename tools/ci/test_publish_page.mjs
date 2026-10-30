import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import vm from "node:vm";

const assets = new URL("../../crates/rspice-publish/src/assets/", import.meta.url);
const script = await readFile(new URL("page.js", assets), "utf8");
const styles = await readFile(new URL("page.css", assets), "utf8");

// The stacks the publisher writes: a figure's root (sans) stack and its
// monospace text's stack.
const SANS = "'RSpice Symbols', 'Segoe UI', system-ui, sans-serif";
const MONO = "'RSpice Plex Mono', 'RSpice Symbols', Consolas, 'Cascadia Code', 'DejaVu Sans Mono', monospace";

// Just enough of an SVG element for the figure download to walk, copy and
// serialize.
class Element {
  constructor(name, attributes = {}, children = []) {
    this.name = name;
    this.attributes = new Map(Object.entries(attributes));
    this.children = [];
    this.parent = null;
    for (const child of children) this.append(child);
  }
  append(child) {
    if (typeof child !== "string") child.parent = this;
    this.children.push(child);
  }
  prepend(child) {
    child.parent = this;
    this.children.unshift(child);
  }
  getAttribute(name) { return this.attributes.get(name) ?? null; }
  hasAttribute(name) { return this.attributes.has(name); }
  setAttribute(name, value) { this.attributes.set(name, String(value)); }
  removeAttribute(name) { this.attributes.delete(name); }
  get classList() { return (this.getAttribute("class") || "").split(/\s+/).filter(Boolean); }
  get textContent() {
    return this.children.map((child) => (typeof child === "string" ? child : child.textContent)).join("");
  }
  set textContent(value) { this.children = [String(value)]; }
  matches(selector) {
    const attribute = /^\[([a-z-]+)\]$/.exec(selector);
    return attribute ? this.hasAttribute(attribute[1]) : this.name === selector;
  }
  closest(selector) {
    for (let node = this; node; node = node.parent) if (node.matches(selector)) return node;
    return null;
  }
  querySelectorAll(selector) {
    return this.children.filter((child) => typeof child !== "string")
      .flatMap((child) => [...(child.matches(selector) ? [child] : []), ...child.querySelectorAll(selector)]);
  }
  querySelector(selector) { return this.querySelectorAll(selector)[0] ?? null; }
  cloneNode() {
    const children = this.children.map((child) => (typeof child === "string" ? child : child.cloneNode()));
    return new Element(this.name, Object.fromEntries(this.attributes), children);
  }
  serialize() {
    const attributes = Array.from(this.attributes, ([name, value]) => ` ${name}="${value}"`).join("");
    const content = this.children.map((child) => (typeof child === "string" ? child : child.serialize())).join("");
    return `<${this.name}${attributes}>${content}</${this.name}>`;
  }
}

// page.css's @font-face rules, as the browser's CSSOM hands them to the page.
class CSSFontFaceRule {
  constructor(block) {
    const descriptors = new Map(block.split(";").map((line) => line.split(":"))
      .filter((pair) => pair.length > 1)
      .map(([name, ...value]) => [name.trim(), value.join(":").replace(/\s+/g, " ").trim()]));
    this.style = { getPropertyValue: (name) => descriptors.get(name) ?? "" };
  }
}
const fontFaces = Array.from(styles.matchAll(/@font-face\s*\{([^}]*)\}/g), (match) => new CSSFontFaceRule(match[1]));

class FileReader {
  async readAsDataURL(blob) {
    const bytes = Buffer.from(await blob.arrayBuffer());
    this.result = `data:${blob.type};base64,${bytes.toString("base64")}`;
    this.onload();
  }
}

// The file the figure's download button saves, with page.js run on a page
// that holds only that figure.
async function download(...texts) {
  const svg = new Element("svg", { "font-family": SANS }, texts);
  const button = new Element("button", { "data-figure-svg": "" });
  const figure = new Element("figure", {}, [new Element("figcaption", {}, ["Sheet 1"]), svg, button]);
  let click;
  button.addEventListener = (_type, listener) => { click = listener; };
  const saved = [];
  class PageURL extends URL {
    static createObjectURL(blob) {
      saved.push(blob);
      return "blob:figure";
    }
    static revokeObjectURL() {}
  }
  const stylesheet = { href: "https://example.test/p/assets/page.css", cssRules: fontFaces };
  const document = {
    documentElement: { classList: { add() {} }, dataset: {}, hasAttribute: () => false },
    querySelectorAll: (selector) => (selector === "[data-figure-svg]" ? [button] : []),
    querySelector: (selector) => (selector.includes('href$="page.css"') ? { sheet: stylesheet } : null),
    getElementById: () => null,
    addEventListener() {},
    createElementNS: (_namespace, name) => new Element(name),
    createElement: (name) => Object.assign(new Element(name), { click() {} }),
  };
  vm.runInContext(script, vm.createContext({
    document,
    location: { hash: "", href: "https://example.test/p/" },
    localStorage: { getItem: () => null, setItem() {} },
    addEventListener() {},
    getComputedStyle: () => ({ getPropertyValue: () => "" }),
    setTimeout: () => 0,
    clearTimeout() {},
    fetch: async () => new Response(new Uint8Array([0, 1, 0, 0])),
    URL: PageURL,
    Blob,
    FileReader,
    CSSFontFaceRule,
    XMLSerializer: class { serializeToString(node) { return node.serialize(); } },
  }));
  await click();
  assert.equal(saved.length, 1, "the figure was not saved");
  return saved[0].text();
}

const face = (family, weight) => `@font-face{font-family:"${family}";font-weight:${weight};`;

test("a figure lettered with a symbol Plex lacks downloads carrying RSpice Symbols", async () => {
  const lettered = [
    new Element("text", {}, ["⌀ 6 mm"]),
    new Element("text", { "font-family": MONO }, ["⌀6"]),
    new Element("text", { "font-family": MONO, "font-weight": "500" }, ["GND ⏚"]),
  ];
  for (const text of lettered) {
    const file = await download(text);
    assert.ok(file.includes(`${face("RSpice Symbols", "100 900")}unicode-range:U+207B, `), text.serialize());
  }
});

test("a figure without such a symbol downloads without RSpice Symbols", async () => {
  const file = await download(
    new Element("text", { "font-family": MONO }, ["R1 10 k"]),
    new Element("text", {}, ["Vout"]),
  );
  assert.ok(!file.includes(face("RSpice Symbols", "100 900")), file);
  assert.ok(file.includes(face("RSpice Plex Mono", "400")), file);
  assert.ok(!file.includes(face("RSpice Plex Mono", "500")), file);
});
