#!/usr/bin/env python3
"""Merge slug-planning JSON into tools/coverage/slugs.tsv."""

from __future__ import annotations

import collections
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/jakebarnby/Local/jakeos")
SLUG_RE = re.compile(r"^[a-z0-9][a-z0-9-]{2,39}$")
PREFIXES = (
    "KRN BOOT ABI CAP CMP TSK IPC MEM SCH OBS SVC STO PKG GFX UIP TXT ACC "
    "SEM LNX WIN VIRT ENV HET WASM SEC NET AUD MED HW PWR SDK APP INS BLD "
    "LAB BEN REL DOC GOV"
).split()


def load_items() -> dict:
    items = {}
    for name in ("inventory", "gaps", "extra"):
        for line in (ROOT / "tools/coverage" / f"{name}.jsonl").read_text().splitlines():
            if line.strip():
                item = json.loads(line)
                items[item["id"]] = item
    return items


def normalize_slug(raw: str) -> str:
    slug = raw.lower().replace("_", "-")
    if "/" in slug:
        slug = slug.split("/", 1)[1]
    slug = re.sub(r"[^a-z0-9-]", "-", slug)[:40].strip("-")
    return slug


def task_type(task: dict) -> str:
    value = task.get("type") or task.get("kind") or "build"
    if value not in {"build", "adr", "spike", "benchmark", "docs"}:
        return "build"
    return value


def load_existing(path: Path) -> list[tuple]:
    rows = []
    if not path.exists():
        return rows
    lines = path.read_text().splitlines()
    if not lines:
        return rows
    header = lines[0].split("\t")
    for line in lines[1:]:
        if not line.strip():
            continue
        parts = line.split("\t")
        while len(parts) < 7:
            parts.append("")
        rows.append(tuple(parts[:7]))
    return rows


def merge_task(by_prefix: dict[str, dict[str, dict]], prefix: str, task: dict, problems: list) -> None:
    slug = normalize_slug(task.get("slug") or "")
    if not SLUG_RE.match(slug):
        problems.append(("badslug", prefix, task.get("slug")))
        return
    bucket = by_prefix.setdefault(prefix, {})
    if slug in bucket:
        existing = bucket[slug]
        covers = list(existing.get("covers") or [])
        for item_id in task.get("covers") or []:
            if item_id not in covers:
                covers.append(item_id)
        existing["covers"] = covers
        return
    bucket[slug] = {
        "slug": slug,
        "milestone": task.get("milestone") or "V1",
        "type": task_type(task),
        "size": task.get("size") or "M",
        "title": (task.get("title") or slug).replace("\t", " ").strip(),
        "covers": list(task.get("covers") or []),
        "justification": (task.get("justification") or "")
        .replace("\t", " ")
        .replace("\n", " ")
        .strip(),
    }


def rows_from_buckets(by_prefix: dict[str, dict[str, dict]], items: dict, problems: list) -> list[tuple]:
    rows = []
    for prefix in PREFIXES:
        for slug, task in sorted(by_prefix.get(prefix, {}).items()):
            covers = [item_id for item_id in task["covers"] if item_id in items]
            for item_id in task["covers"]:
                if item_id not in items:
                    problems.append(("unknownid", prefix, slug, item_id))
            rows.append(
                (
                    f"{prefix}-@{slug}",
                    task["milestone"],
                    task["type"],
                    task["size"],
                    task["title"],
                    ",".join(covers),
                    task["justification"],
                )
            )
    return rows


def main() -> None:
    if len(sys.argv) < 2:
        print("usage: merge_plans.py PLAN.json [PLAN.json ...]", file=sys.stderr)
        sys.exit(2)
    items = load_items()
    problems: list = []
    by_prefix: dict[str, dict[str, dict]] = collections.defaultdict(dict)
    existing = load_existing(ROOT / "tools/coverage/slugs.tsv")
    for draft, milestone, kind, size, title, covers, justification in existing:
        if "-@" not in draft:
            continue
        prefix, slug = draft.split("-@", 1)
        merge_task(
            by_prefix,
            prefix,
            {
                "slug": slug,
                "milestone": milestone,
                "type": kind,
                "size": size,
                "title": title,
                "covers": [part for part in covers.split(",") if part],
                "justification": justification,
            },
            problems,
        )
    for path in sys.argv[1:]:
        payload = json.loads(Path(path).read_text())
        result = payload.get("result", payload)
        plans = result.get("plans") or {}
        additions = result.get("additions") or []
        for plan in list(plans.values()) + additions:
            if not isinstance(plan, dict):
                continue
            prefix = plan.get("prefix")
            if prefix not in PREFIXES:
                problems.append(("badprefix", prefix))
                continue
            for task in plan.get("tasks") or []:
                merge_task(by_prefix, prefix, task, problems)
    rows = rows_from_buckets(by_prefix, items, problems)
    covered = collections.Counter()
    for row in rows:
        for item_id in row[5].split(","):
            if item_id:
                covered[item_id] += 1
    uncovered = [item_id for item_id in items if covered[item_id] == 0]
    out = ROOT / "tools/coverage/slugs.tsv"
    with out.open("w") as handle:
        handle.write("draft\tmilestone\ttype\tsize\ttitle\tcovers\tjustification\n")
        for row in sorted(rows):
            handle.write("\t".join(row) + "\n")
    (ROOT / "tools/coverage/uncovered.json").write_text(json.dumps(uncovered, indent=1) + "\n")
    by_milestone = collections.Counter(row[1] for row in rows)
    by_prefix_count = collections.Counter(row[0].split("-@")[0] for row in rows)
    print("tasks", len(rows))
    print("by milestone", dict(sorted(by_milestone.items())))
    print("by prefix", dict(sorted(by_prefix_count.items())))
    print("uncovered", len(uncovered))
    print("problems", len(problems))
    for problem in problems[:30]:
        print(" problem", problem)
    print(
        "xl",
        sum(1 for row in rows if row[3] == "XL"),
        "adr",
        sum(1 for row in rows if row[2] == "adr"),
        "spike",
        sum(1 for row in rows if row[2] == "spike"),
        "bench",
        sum(1 for row in rows if row[2] == "benchmark"),
    )


if __name__ == "__main__":
    main()
