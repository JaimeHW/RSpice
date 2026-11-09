#!/usr/bin/env python3
"""Read the browser workbench through Chromium's accessibility tree.

The canvas paints everything, so a screen reader gets only the page's
off-screen mirror of egui's accessibility tree. Drive the real workbench and
read that mirror back the way assistive technology does: a dialog's fields, the
editor's caret line, the live region a compile answers in, and F6 walking the
panes without the keyboard ever leaving the canvas.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from browser_workbench import WorkbenchBrowser, controls, wait_for
from check_browser_workbench import choose_command, wait_until_ready

F6 = ""
SHIFT = ""
ARROW_DOWN = ""
HOME = ""
CONTROL = ""

MIRROR = {
    "focused": "rspice_mirror_focused_control",
    "caret": "rspice_mirror_caret_line",
    "dialog": "rspice_mirror_dialog",
    "focus_said": "rspice_mirror_focus_announcement",
    "said": "rspice_mirror_announcement",
}
EDITOR = "Verilog-A/AMS source editor"


def ax_property(node, key):
    return (node.get(key) or {}).get("value", "")


def read_mirror(browser):
    """What the browser's accessibility tree holds for each mirror element."""
    backend = {}

    def find(node):
        names = node.get("attributes", [])
        element = dict(zip(names[0::2], names[1::2])).get("id")
        if element in MIRROR.values():
            backend[element] = node["backendNodeId"]
        for child in node.get("children", []):
            find(child)

    find(browser.cdp("DOM.getDocument", {"depth": -1})["root"])
    nodes = browser.cdp("Accessibility.getFullAXTree", {})["nodes"]
    by_id = {node["nodeId"]: node for node in nodes}
    by_backend = {node.get("backendDOMNodeId"): node for node in nodes}

    def words(node):
        said = [ax_property(node, "name")] if ax_property(node, "role") == "StaticText" else []
        for child in node.get("childIds", []):
            if child in by_id:
                said.extend(words(by_id[child]))
        return said

    def read(node):
        return {"role": ax_property(node, "role"), "name": ax_property(node, "name"),
                "value": ax_property(node, "value"),
                "description": ax_property(node, "description"),
                "text": "".join(words(node)),
                "properties": {entry["name"]: entry["value"].get("value")
                               for entry in node.get("properties", [])}}

    mirror = {}
    for key, element in MIRROR.items():
        node = by_backend.get(backend.get(element))
        if node is None:
            raise AssertionError(f"{element} is not in the accessibility tree")
        mirror[key] = read(node)
        if key == "dialog":
            mirror["fields"] = [read(by_id[child]) for child in node.get("childIds", [])
                                if child in by_id]
    return mirror


def keyboard_owner(browser):
    return browser.script("""
        const active = document.activeElement;
        return {tag: active.tagName, id: active.id};
    """)


def assert_keyboard_stays_with_the_page(browser, where):
    owner = keyboard_owner(browser)
    if owner["id"].startswith("rspice_mirror"):
        raise AssertionError(f"{where}: the mirror took the keyboard: {owner}")
    return owner


def wait_for_control(browser, label, role=None):
    return wait_for(lambda: next((control for control in controls(browser.snapshot())
                                  if control["label"] == label
                                  and (role is None or control["role"] == role)), None),
                    f"the {label!r} control", 60)


def device_dialog(browser):
    """A dialog opens as a named dialog listing its fields by name and value."""
    choose_command(browser, "New Verilog-A device")
    mirror = wait_for(lambda: (lambda seen: seen if seen["dialog"]["role"] == "dialog"
                               and len(seen["fields"]) >= 3 else None)(read_mirror(browser)),
                      "the dialog in the accessibility tree", 60)
    first = mirror["dialog"], mirror["fields"]
    browser.capture("device-dialog")
    if first[0]["name"] != "New Verilog-A device":
        raise AssertionError(f"The dialog is not named: {first[0]}")
    fields = {field["name"]: field for field in first[1]}
    if fields.get("Library", {}).get("text") != "user" or "Cell name" not in fields or "Ports" not in fields:
        raise AssertionError(f"Field names or values are missing: {first[1]}")
    if not fields.get("Create", {}).get("properties", {}).get("disabled"):
        raise AssertionError(f"Create should read as unavailable: {first[1]}")
    # A value typed on the canvas shows up in the tree.
    browser.click("Cell name", "textInput", keyboard=(("gain", ()),))
    typed = wait_for(lambda: next((field for field in read_mirror(browser)["fields"]
                                   if field["name"] == "Cell name" and field["text"] == "gain"), None),
                     "the typed name in the accessibility tree", 30)
    assert_keyboard_stays_with_the_page(browser, "typing in a dialog field")
    return {"dialog": first[0], "fields": first[1], "typed": typed}


def source_editor(browser):
    """The caret's line is readable, and arriving on another line says it."""
    create = wait_for_control(browser, "Create", "button")
    browser.click(create["label"], create["role"])
    wait_for_control(browser, EDITOR, "multilineTextInput")
    browser.click(EDITOR, "multilineTextInput")
    # The click lands past the module's last line; its first line has words.
    browser.keys(HOME, modifiers=(CONTROL,))
    start = wait_for(lambda: (lambda seen: seen if seen["caret"]["text"] else None)(read_mirror(browser)),
                     "the caret line in the accessibility tree", 30)
    before_line, before_said = start["caret"]["text"], start["said"]["text"]
    if start["caret"]["name"] != EDITOR or not start["caret"]["properties"].get("readonly"):
        raise AssertionError(f"The caret line does not read as the editor's: {start['caret']}")
    browser.keys(ARROW_DOWN)
    after = wait_for(lambda: (lambda seen: seen if seen["caret"]["text"] != before_line
                              and seen["said"]["text"] != before_said else None)(read_mirror(browser)),
                     "the next line in the accessibility tree", 30)
    browser.capture("caret-line")
    assert_keyboard_stays_with_the_page(browser, "moving the caret")
    if after["said"]["text"] != after["caret"]["text"]:
        raise AssertionError(f"The caret's arrival said something else: {after}")
    return {"caret_before": start["caret"], "caret_after": after["caret"],
            "arrival_said": after["said"], "focused": after["focused"]}


def compile_answer(browser):
    """The live region changes when the compile finishes."""
    before = read_mirror(browser)["said"]["text"]
    choose_command(browser, "Compile Verilog-A/AMS")

    def answered():
        said = read_mirror(browser)["said"]["text"]
        return said if said != before and said.startswith("Compile") else None

    answer = wait_for(answered, "the compile answer in the live region", 120)
    browser.capture("compile-answer")
    assert_keyboard_stays_with_the_page(browser, "compiling")
    if not answer.startswith("Compiled gain"):
        raise AssertionError(f"The compile answered something unexpected: {answer}")
    return {"before": before, "answer": answer}


def cycle_panes(browser):
    """F6 and Shift+F6 go round the panes, and the page keeps the key."""
    browser.script("""
        window.__rspiceF6 = [];
        window.addEventListener('keydown', event => {
            if (event.key === 'F6') {
                const press = {prevented: false, target: event.target.tagName};
                window.__rspiceF6.push(press);
                // Read once the page and eframe have both had the event.
                setTimeout(() => { press.prevented = event.defaultPrevented; }, 0);
            }
        }, true);
    """)

    def step(modifiers=()):
        before = read_mirror(browser)["focused"]
        browser.keys(F6, modifiers=modifiers)
        seen = wait_for(lambda: (lambda now: now if now != before else None)(
            read_mirror(browser)["focused"]), "focus to move to the next pane", 30)
        assert_keyboard_stays_with_the_page(browser, "pressing F6")
        return {"role": seen["role"], "name": seen["name"], "text": seen["text"]}

    forward = [step() for _ in range(6)]
    backward = [step((SHIFT,)) for _ in range(6)]
    browser.capture("panes")
    # This document has four panes that can hold the keyboard.
    cycle = forward[:4]
    if len({(pane["role"], pane["name"]) for pane in cycle}) != 4 or forward[4:] != cycle[:2]:
        raise AssertionError(f"F6 did not go round the panes: {forward}")
    expected = [cycle[(1 - step_number) % 4] for step_number in range(1, 7)]
    if backward != expected:
        raise AssertionError(f"Shift+F6 did not walk back: {backward}, expected {expected}")
    events = wait_for(lambda: (lambda seen: seen if len(seen) == 12
                               and all(event["prevented"] for event in seen) else None)(
                          browser.script("return window.__rspiceF6")),
                      "the page to keep every F6 from the browser", 10)
    return {"forward": forward, "backward": backward, "events": events}


def run(browser):
    wait_until_ready(browser)
    browser.cdp("Accessibility.enable", {})
    browser.cdp("DOM.enable", {})
    browser.click("Schematic canvas", "canvas")
    startup = read_mirror(browser)
    owner = assert_keyboard_stays_with_the_page(browser, "startup")
    return {"startup_focused": startup["focused"], "keyboard_owner": owner,
            "dialog": device_dialog(browser), "editor": source_editor(browser),
            "compile": compile_answer(browser), "panes": cycle_panes(browser)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--web-root", type=Path, default=Path("crates/rspice-app/web"))
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--driver")
    parser.add_argument("--browser")
    parser.add_argument("--software-webgpu", action="store_true",
                        help="Use SwiftShader for functional CI, without qualifying physical GPU support")
    args = parser.parse_args()
    inputs = {name: hashlib.sha256((args.web_root / name).read_bytes()).hexdigest()
              for name in ("index.html", "pkg/rspice-ui.js", "pkg/rspice-ui_bg.wasm")}
    with WorkbenchBrowser(args.web_root, args.output, args.driver, args.browser,
                          args.software_webgpu) as browser:
        try:
            result = run(browser)
            browser.assert_no_errors()
            (browser.output / "result.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
        except BaseException as error:
            (browser.output / "failure.json").write_text(json.dumps({"error": str(error)}), encoding="utf-8")
            (browser.output / "failure-mirror.json").write_text(
                json.dumps(read_mirror(browser), indent=2), encoding="utf-8")
            browser.capture("failure-ui")
            raise
        finally:
            (browser.output / "inputs.json").write_text(json.dumps(inputs, indent=2), encoding="utf-8")
    print(json.dumps(result, ensure_ascii=False))


if __name__ == "__main__":
    main()
