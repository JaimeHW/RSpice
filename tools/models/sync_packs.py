"""Materialize third-party SPICE model packs declared in models/spice/sources.toml.

Every pack is fetched from a pinned upstream commit into a local cache, filtered
through the pack's include/exclude globs, and copied verbatim into

    models/spice/<category>/<id>/upstream/

alongside a generated ``pack.toml`` recording provenance, licensing and a
content digest. Nothing under ``upstream/`` is ever edited by hand: re-running
this script must reproduce the tree byte for byte.

Usage:
    python tools/models/sync_packs.py                 sync every pack
    python tools/models/sync_packs.py --pack sky130   sync one pack
    python tools/models/sync_packs.py --check         verify, write nothing
"""

from __future__ import annotations

import argparse
import hashlib
import re
import shutil
import subprocess
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SPICE_ROOT = REPO_ROOT / "models" / "spice"
SOURCES = SPICE_ROOT / "sources.toml"
CACHE_ROOT = REPO_ROOT / ".model-src-cache"

VALID_TIERS = {"permissive", "copyleft", "ambiguous", "own"}


class SyncError(RuntimeError):
    """A pack could not be materialized."""


#==============================================================================
# Glob matching
#==============================================================================


def _compile_glob(pattern: str) -> re.Pattern[str]:
    """Translate a forward-slash glob into a regex.

    Supports ``**`` (any number of path segments), ``*`` (within one segment)
    and ``?``. Implemented directly rather than via :mod:`fnmatch` so that
    ``*`` never crosses a path separator.
    """
    out = ["(?s:"]
    i = 0
    while i < len(pattern):
        char = pattern[i]
        if pattern.startswith("**/", i):
            out.append("(?:[^/]+/)*")
            i += 3
        elif pattern.startswith("**", i):
            out.append(".*")
            i += 2
        elif char == "*":
            out.append("[^/]*")
            i += 1
        elif char == "?":
            out.append("[^/]")
            i += 1
        else:
            out.append(re.escape(char))
            i += 1
    out.append(r")\Z")
    return re.compile("".join(out))


@dataclass(frozen=True)
class Filter:
    include: tuple[re.Pattern[str], ...]
    exclude: tuple[re.Pattern[str], ...]

    @classmethod
    def build(cls, include: list[str], exclude: list[str]) -> "Filter":
        return cls(
            include=tuple(_compile_glob(p) for p in include),
            exclude=tuple(_compile_glob(p) for p in exclude),
        )

    def accepts(self, relpath: str) -> bool:
        if not any(p.match(relpath) for p in self.include):
            return False
        return not any(p.match(relpath) for p in self.exclude)


#==============================================================================
# Pack model
#==============================================================================


@dataclass
class Pack:
    id: str
    name: str
    category: str
    originator: str
    description: str
    source: dict
    license: dict
    contents: dict

    @property
    def dest(self) -> Path:
        return SPICE_ROOT / self.category / self.id

    @property
    def upstream_dir(self) -> Path:
        return self.dest / "upstream"

    @property
    def cache_dir(self) -> Path:
        return CACHE_ROOT / self.id


def load_packs() -> list[Pack]:
    with SOURCES.open("rb") as handle:
        data = tomllib.load(handle)
    if data.get("schema") != 1:
        raise SyncError(f"{SOURCES}: unsupported schema {data.get('schema')!r}")

    packs: list[Pack] = []
    seen: set[str] = set()
    for entry in data.get("pack", []):
        pack = Pack(
            id=entry["id"],
            name=entry["name"],
            category=entry["category"],
            originator=entry.get("originator", ""),
            description=entry.get("description", "").strip(),
            source=entry.get("source", {}),
            license=entry["license"],
            contents=entry.get("contents", {}),
        )
        if pack.id in seen:
            raise SyncError(f"duplicate pack id {pack.id!r} in {SOURCES}")
        seen.add(pack.id)
        tier = pack.license.get("tier")
        if tier not in VALID_TIERS:
            raise SyncError(
                f"pack {pack.id!r}: license.tier {tier!r} not one of {sorted(VALID_TIERS)}"
            )
        packs.append(pack)
    return packs


#==============================================================================
# Fetching
#==============================================================================


def _git(args: list[str], cwd: Path) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=cwd,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise SyncError(
            f"git {' '.join(args)} failed in {cwd}:\n{result.stderr.strip()}"
        )
    return result.stdout.strip()


def fetch_git(pack: Pack) -> Path:
    """Ensure the pack's cache holds the pinned upstream commit."""
    url = pack.source["url"]
    commit = pack.source["commit"]
    sparse = pack.source.get("sparse", [])
    cache = pack.cache_dir

    if not (cache / ".git").exists():
        if cache.exists():
            shutil.rmtree(cache)
        cache.parent.mkdir(parents=True, exist_ok=True)
        print(f"  cloning {url}")
        subprocess.run(
            [
                "git", "clone", "--filter=blob:none", "--no-checkout",
                "--sparse", url, str(cache),
            ],
            check=True,
            capture_output=True,
            text=True,
        )

    # Sparse-checkout state is applied as its own invocation: chaining it onto
    # the clone leaves the cone list empty on Windows git. An empty `sparse`
    # list means the whole tree is wanted, which requires actively disabling
    # the sparse mode the clone was created with.
    if sparse:
        _git(["sparse-checkout", "set", "--cone", *sparse], cwd=cache)
    else:
        _git(["sparse-checkout", "disable"], cwd=cache)

    if not _has_head(cache) or _git(["rev-parse", "HEAD"], cwd=cache) != commit:
        try:
            _git(["fetch", "--depth", "1", "origin", commit], cwd=cache)
        except SyncError:
            # Servers that refuse single-commit fetches still serve full history.
            _git(["fetch", "origin"], cwd=cache)

    # Always check out: a clone made with --no-checkout leaves an empty worktree
    # even when HEAD already points at the pinned commit.
    print(f"  checking out {commit[:12]}")
    _git(["checkout", "--detach", "--force", commit], cwd=cache)

    return cache


def _has_head(cache: Path) -> bool:
    result = subprocess.run(
        ["git", "rev-parse", "--verify", "HEAD"],
        cwd=cache,
        capture_output=True,
        text=True,
    )
    return result.returncode == 0


def _bsdtar() -> str:
    """Locate a libarchive tar able to read zip and 7z."""
    for candidate in (
        Path("C:/Windows/System32/tar.exe"),
        Path("C:/msys64/usr/bin/bsdtar.exe"),
    ):
        if candidate.exists():
            return str(candidate)
    found = shutil.which("bsdtar") or shutil.which("tar")
    if not found:
        raise SyncError(
            "no bsdtar/tar on PATH; needed to unpack archive-backed model packs"
        )
    return found


def fetch_http(pack: Pack) -> Path:
    """Download (and unpack) an HTTP-backed pack, returning its content root.

    The artifact is pinned by sha256: a hash mismatch means upstream changed
    under a pin and is always an error, never a silent re-vendor.
    """
    url = pack.source["url"]
    expected = pack.source["sha256"]
    cache = pack.cache_dir
    cache.mkdir(parents=True, exist_ok=True)

    artifact = cache / (pack.source.get("filename") or url.rsplit("/", 1)[-1])
    if not artifact.exists() or file_digest(artifact) != expected:
        print(f"  downloading {url}")
        result = subprocess.run(
            ["curl", "-sSL", "--fail", "--max-time", "300", url, "-o", str(artifact)],
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            raise SyncError(f"download failed for {url}: {result.stderr.strip()}")

    actual = file_digest(artifact)
    if actual != expected:
        raise SyncError(
            f"pack {pack.id!r}: sha256 mismatch for {url}\n"
            f"  expected {expected}\n  actual   {actual}\n"
            f"  upstream changed under a pin; review before updating sources.toml"
        )

    archive = pack.source.get("archive")
    if not archive:
        return artifact.parent

    extract = cache / "extract"
    if extract.exists():
        shutil.rmtree(extract)
    extract.mkdir(parents=True)
    result = subprocess.run(
        [_bsdtar(), "-xf", str(artifact), "-C", str(extract)],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise SyncError(f"unpacking {artifact.name} failed: {result.stderr.strip()}")

    root = extract
    for _ in range(int(pack.source.get("strip_components", 0))):
        children = [c for c in root.iterdir()]
        if len(children) != 1 or not children[0].is_dir():
            raise SyncError(
                f"pack {pack.id!r}: strip_components exceeds the archive's "
                f"single-root depth at {root}"
            )
        root = children[0]
    return root


#==============================================================================
# Materialization
#==============================================================================


def collect_files(root: Path, filt: Filter) -> list[Path]:
    """Return accepted paths relative to *root*, sorted for determinism."""
    selected: list[Path] = []
    for path in root.rglob("*"):
        if not path.is_file():
            continue
        rel = path.relative_to(root).as_posix()
        if rel.startswith(".git/"):
            continue
        if filt.accepts(rel):
            selected.append(path.relative_to(root))
    return sorted(selected, key=lambda p: p.as_posix())


def file_digest(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def pack_digest(root: Path, files: list[Path]) -> str:
    """Deterministic digest over the materialized file set."""
    outer = hashlib.sha256()
    for rel in files:
        outer.update(rel.as_posix().encode("utf-8"))
        outer.update(b"\0")
        outer.update(file_digest(root / rel).encode("ascii"))
        outer.update(b"\n")
    return outer.hexdigest()


def _toml_str(value: str) -> str:
    escaped = value.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def _toml_list(values: list[str]) -> str:
    if not values:
        return "[]"
    return "[" + ", ".join(_toml_str(v) for v in values) + "]"


def render_pack_toml(pack: Pack, files: list[Path], digest: str, total_bytes: int) -> str:
    src = pack.source
    lic = pack.license
    contents = pack.contents

    lines = [
        "# Generated by tools/models/sync_packs.py — do not edit by hand.",
        "# Declared in models/spice/sources.toml; re-run the sync tool to update.",
        "",
        "[pack]",
        f"id = {_toml_str(pack.id)}",
        f"name = {_toml_str(pack.name)}",
        f"category = {_toml_str(pack.category)}",
        f"originator = {_toml_str(pack.originator)}",
        f"description = {_toml_str(' '.join(pack.description.split()))}",
        "",
        "[provenance]",
        f"kind = {_toml_str(src.get('kind', 'git'))}",
        f"url = {_toml_str(src.get('url', ''))}",
    ]
    if src.get("commit"):
        lines.append(f"commit = {_toml_str(src['commit'])}")
    if src.get("sha256"):
        lines.append(f"sha256 = {_toml_str(src['sha256'])}")
    lines += [
        "",
        "[license]",
        f"spdx = {_toml_str(lic.get('spdx') or 'NOASSERTION')}",
        f"tier = {_toml_str(lic.get('tier', ''))}",
        f"redistributable = {str(bool(lic.get('redistributable', False))).lower()}",
        f"notice_required = {str(bool(lic.get('notice_required', False))).lower()}",
        f"file = {_toml_str(lic.get('file') or 'LICENSE-NOTE.md')}",
        "",
        "[contents]",
    ]
    if contents.get("process"):
        lines.append(f"process = {_toml_str(contents['process'])}")
    lines.extend(
        [
            f"devices = {_toml_list(contents.get('devices', []))}",
            f"corners = {_toml_list(contents.get('corners', []))}",
            f"entry = {_toml_str(contents.get('entry', ''))}",
            f"files = {len(files)}",
            f"bytes = {total_bytes}",
            f"digest = {_toml_str(digest)}",
            "",
        ]
    )
    return "\n".join(lines)


def render_license_note(pack: Pack) -> str:
    """Record licensing for packs whose upstream ships no license text."""
    lic = pack.license
    tier = lic.get("tier", "")
    guidance = {
        "permissive": "Redistribution is granted. A NOTICE attribution entry satisfies it.",
        "copyleft": (
            "Copyleft terms apply. This pack is vendored for development and "
            "validation and must not be embedded into a proprietary binary."
        ),
        "ambiguous": (
            "Redistribution is neither clearly granted nor clearly forbidden. "
            "This pack is vendored so the models are usable; confirm the terms "
            "before including it in a distributed build."
        ),
        "own": "Authored by the RSpice project. No third-party rights attach.",
    }.get(tier, "")

    return "\n".join(
        [
            f"# Licensing note — {pack.name}",
            "",
            "Generated by `tools/models/sync_packs.py`; edit the pack's entry in",
            "`models/spice/sources.toml` instead of this file.",
            "",
            "Upstream ships no license file. The terms below are recorded as found",
            "at the source, verbatim where quoted.",
            "",
            f"- **Originator:** {pack.originator}",
            f"- **Source:** {pack.source.get('url', '')}",
            f"- **Declared SPDX:** {lic.get('spdx') or 'none stated'}",
            f"- **Tier:** `{tier}`",
            f"- **Redistributable:** {'yes' if lic.get('redistributable') else 'not established'}",
            "",
            "## Terms as found",
            "",
            lic.get("note", "").strip(),
            "",
            "## What this means for RSpice",
            "",
            guidance,
            "",
        ]
    )


def sync_pack(pack: Pack, check: bool) -> tuple[int, int]:
    print(f"[{pack.id}] {pack.name}")
    kind = pack.source.get("kind", "git")
    if kind == "git":
        cache = fetch_git(pack)
    elif kind == "http":
        cache = fetch_http(pack)
    else:
        raise SyncError(f"pack {pack.id!r}: unsupported source kind {kind!r}")

    filt = Filter.build(
        pack.source.get("include", []),
        pack.source.get("exclude", []),
    )
    files = collect_files(cache, filt)
    if not files:
        raise SyncError(f"pack {pack.id!r}: include globs matched no files")

    total_bytes = sum((cache / rel).stat().st_size for rel in files)
    digest = pack_digest(cache, files)

    if check:
        existing = pack.dest / "pack.toml"
        if not existing.exists():
            raise SyncError(f"pack {pack.id!r}: {existing} is missing")
        with existing.open("rb") as handle:
            recorded = tomllib.load(handle)["contents"].get("digest")
        if recorded != digest:
            raise SyncError(
                f"pack {pack.id!r}: tree digest {digest[:12]} does not match "
                f"recorded {str(recorded)[:12]}"
            )
        print(f"  ok - {len(files)} files, {total_bytes / 1e6:.1f} MB")
        return len(files), total_bytes

    upstream = pack.upstream_dir
    if upstream.exists():
        shutil.rmtree(upstream)
    for rel in files:
        target = upstream / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(cache / rel, target)

    license_file = pack.license.get("file")
    stale_note = pack.dest / "LICENSE-NOTE.md"
    if stale_note.exists():
        stale_note.unlink()
    if license_file:
        source_license = cache / license_file
        if not source_license.exists():
            raise SyncError(
                f"pack {pack.id!r}: license file {license_file!r} not found upstream"
            )
        shutil.copy2(source_license, pack.dest / "LICENSE")
    elif pack.license.get("note"):
        # Upstream ships no license text. Record the terms as they were found
        # rather than inventing a LICENSE file the originator never wrote.
        stale_note.write_text(
            render_license_note(pack),
            encoding="utf-8",
        )
    else:
        raise SyncError(
            f"pack {pack.id!r}: needs either license.file or license.note"
        )

    (pack.dest / "pack.toml").write_text(
        render_pack_toml(pack, files, digest, total_bytes),
        encoding="utf-8",
    )
    print(f"  {len(files)} files, {total_bytes / 1e6:.1f} MB -> {pack.dest.relative_to(REPO_ROOT)}")
    return len(files), total_bytes


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pack", action="append", help="sync only this pack id")
    parser.add_argument(
        "--check",
        action="store_true",
        help="verify the vendored tree matches upstream without writing",
    )
    args = parser.parse_args()

    packs = load_packs()
    if args.pack:
        wanted = set(args.pack)
        unknown = wanted - {p.id for p in packs}
        if unknown:
            print(f"unknown pack id(s): {', '.join(sorted(unknown))}", file=sys.stderr)
            return 2
        packs = [p for p in packs if p.id in wanted]

    total_files = 0
    total_bytes = 0
    failures = 0
    for pack in packs:
        try:
            files, size = sync_pack(pack, args.check)
        except SyncError as exc:
            print(f"  FAILED: {exc}", file=sys.stderr)
            failures += 1
            continue
        total_files += files
        total_bytes += size

    print(f"\n{len(packs) - failures}/{len(packs)} packs, "
          f"{total_files} files, {total_bytes / 1e6:.1f} MB")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
