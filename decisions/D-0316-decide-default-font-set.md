# D-0316 · Decide the default system font set and publish its script coverage matrix
- Status: proposed
- Task: TXT-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The first immutable V0.5 image redistributes fonts, so the default system set is fixed now under the GOV-009 redistribution rule (§41). It must cover Latin, Cyrillic, Greek, Arabic, Hebrew, Devanagari, Thai, CJK, a monospace family and COLRv1 colour emoji, include metric-compatible substitutes for the Windows core fonts that Linux and Windows documents assume, and ship no Microsoft font (I-070). The coverage matrix (family, scripts, substitute-for) lives in this file so TXT and the personalities read one table.

## Options

### Option A · Named open-licensed families covering the script list with metric-compatible substitutes
Summary: A named open-licensed set: Noto Sans and Serif for the script list, Noto Sans CJK, Noto Color Emoji (COLRv1), a monospace family such as JetBrains Mono or Noto Sans Mono, and Liberation and Carlito as metric-compatible substitutes for Arial, Times New Roman, Courier New and Calibri.
Consequences: Every listed script renders out of the box, Windows and Linux documents lay out with the same metrics, and there is one coverage matrix to test against. CJK and the full Noto set add hundreds of megabytes to the image, and every family is one more upstream to track for security and shaping fixes.
Evidence: none

### Option B · Latin-first core set with optional per-script Packages
Summary: A Latin, Greek and Cyrillic core with emoji and the metric substitutes; other scripts ship as optional per-script font Packages installed on locale selection or on demand.
Consequences: The base image stays small and the store deduplicates scripts the user never selects. A document in an uninstalled script shows fallback boxes until a Package installs, first-boot locale selection becomes a dependency of readable text, and the accessibility and localisation gates must test the installed-on-demand path.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
