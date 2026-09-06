#!/usr/bin/env bash
# Fails when a task that becomes done in this pull request names a Verified by
# handle that is neither the pull request author nor an approving reviewer.
set -euo pipefail

base="${1:?base ref}"
: "${GH_TOKEN:?}" "${PR:?}" "${AUTHOR:?}" "${GITHUB_REPOSITORY:?}"

approvers="$(gh api "repos/${GITHUB_REPOSITORY}/pulls/${PR}/reviews" --paginate \
  --jq '.[] | select(.state == "APPROVED") | .user.login' | sort -u)"

python3 - "$base" "$AUTHOR" "$approvers" <<'PY'
import re, subprocess, sys, glob

base, author, approvers = sys.argv[1], sys.argv[2], set(sys.argv[3].split())
allowed = approvers | {author}

def blocks(text):
    out = {}
    for block in re.split(r"\n(?=### [A-Z]{2,4}-\d{3,} · )", text)[1:]:
        task_id = block.split()[1]
        status = re.search(r"^- Status: (\S+)", block, re.M)
        verified = re.search(r"^- Verified by: @(\S+)", block, re.M)
        out[task_id] = (status.group(1) if status else "", verified.group(1) if verified else "")
    return out

failures = []
for path in sorted(glob.glob("workstreams/*.md")):
    try:
        before = subprocess.run(["git", "show", f"{base}:{path}"], check=True, capture_output=True, text=True).stdout
    except subprocess.CalledProcessError:
        before = ""
    old, new = blocks(before), blocks(open(path, encoding="utf-8").read())
    for task_id, (status, verified) in new.items():
        if status != "done" or old.get(task_id, ("", ""))[0] == "done":
            continue
        if not verified:
            failures.append(f"{task_id}: done without Verified by")
        elif verified not in allowed:
            failures.append(f"{task_id}: Verified by @{verified} is neither the author (@{author}) nor an approver ({', '.join(sorted(approvers)) or 'none'})")

if failures:
    print("\n".join(failures))
    sys.exit(1)
print("every done transition is verified by the author or an approver")
PY
