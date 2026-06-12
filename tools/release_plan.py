#!/usr/bin/env python
"""Release candidate manifest helpers for split modules.

This script is intentionally lightweight. It turns ``tools/package-index.toml``
into an actionable candidate list for outbound publication work (repo checks,
release checks, and candidate artifact notes).
"""

from __future__ import annotations

import argparse
import json
import pathlib
import tomllib

from datetime import datetime, timezone
from typing import Any, Dict, List


REPO_ROOT = pathlib.Path(__file__).resolve().parents[1]
INDEX_PATH = REPO_ROOT / "tools" / "package-index.toml"
DEFAULT_MARKDOWN_PATH = REPO_ROOT / "releases" / "release-candidates.md"


def load_packages() -> List[Dict[str, Any]]:
    data = tomllib.loads(INDEX_PATH.read_text(encoding="utf-8"))
    return list(data.get("package", []))


def evaluate_package(pkg: Dict[str, Any]) -> Dict[str, Any]:
    source = REPO_ROOT / pkg["source"]
    status: List[str] = []
    if source.exists():
        status.append("source:ok")
    else:
        status.append("source:missing")

    lang = pkg.get("language", "").lower()
    if lang == "rust":
        if (source / "Cargo.toml").exists() or any(source.rglob("Cargo.toml")):
            status.append("manifest:ok")
        else:
            status.append("manifest:missing")
    elif lang == "quanta":
        if (source / "lib.quanta").exists():
            status.append("manifest:ok")
        else:
            status.append("manifest:missing")

    if (source / "README.md").exists():
        status.append("readme:ok")
    else:
        status.append("readme:missing")

    if not source.exists():
        publish = False
    else:
        publish = bool(pkg.get("publish", False))
    if publish and "missing" not in "".join(status):
        status.append("publish:ready")
    else:
        status.append("publish:not-ready")

    return {
        "name": pkg["name"],
        "slug": pkg["slug"],
        "source": pkg["source"],
        "language": pkg["language"],
        "repository": pkg["repository"],
        "category": pkg.get("category", "unassigned"),
        "status": status,
        "publish": publish,
    }


def format_markdown(packages: List[Dict[str, Any]], title: str) -> str:
    header = [
        f"# {title}",
        "",
        f"Generated: {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M:%SZ')}",
        "",
        "| Package | Slug | Language | Category | Source | Repository | Ready |",
        "|---|---|---|---|---|---|---|",
    ]
    rows = []
    for pkg in packages:
        ready = "yes" if "publish:ready" in pkg["status"] else "no"
        rows.append(
            f"| {pkg['name']} | `{pkg['slug']}` | {pkg['language']} | "
            f"{pkg['category']} | `{pkg['source']}` | {pkg['repository']} | {ready} |"
        )
    return "\n".join(header + rows) + "\n"


def cmd_plan(args: argparse.Namespace) -> int:
    packages = load_packages()
    records = [evaluate_package(pkg) for pkg in packages]

    if args.module:
        records = [p for p in records if p["slug"] == args.module]

    if args.only_publish:
        records = [p for p in records if p["publish"]]

    if args.json:
        print(json.dumps(records, indent=2, sort_keys=True))
    else:
        for p in records:
            print(f"{p['name']} ({p['slug']})")
            print(f"  - source: {p['source']}")
            print(f"  - repository: {p['repository']}")
            print(f"  - category: {p['category']}")
            print(f"  - publish-ready: {'yes' if 'publish:ready' in p['status'] else 'no'}")
            print(f"  - checks: {', '.join(p['status'])}")

    markdown_output = args.markdown
    if args.write_markdown and markdown_output is None:
        markdown_output = DEFAULT_MARKDOWN_PATH

    if markdown_output is not None:
        markdown_output.parent.mkdir(parents=True, exist_ok=True)
        title = "Module Release Candidates"
        if args.module:
            title = f"Module Release Candidate: {args.module}"
        markdown_output.write_text(format_markdown(records, title), encoding="utf-8")
        print(f"wrote {markdown_output}")

    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Generate module release candidate summary from package-index."
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="emit JSON summary",
    )
    parser.add_argument(
        "--module",
        help="limit to one slug",
    )
    parser.add_argument(
        "--only-publish",
        action="store_true",
        help="limit to publish=true packages",
    )
    parser.add_argument(
        "--markdown",
        type=pathlib.Path,
        default=None,
        help="optional markdown output file path",
    )
    parser.add_argument(
        "--write-markdown",
        action="store_true",
        help=(
            "emit releases/release-candidates.md after planning."
        ),
    )
    return parser


def main() -> int:
    args = build_parser().parse_args()
    return cmd_plan(args)


if __name__ == "__main__":
    raise SystemExit(main())
