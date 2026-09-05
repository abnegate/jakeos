# D-0098 · Decide font shipping versus metric-compatible substitutes
- Status: accepted
- Task: GOV-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0.5 ships the first immutable image with fonts, so redistributable-font policy must exist before TXT selects families (§41, §49, I-070).

## Options

### Option A · OSI-approved font licenses only
Summary: Only fonts under OSI-approved licenses ship.
Consequences: Clean licensing; no metric-compatible substitutes unless open.
Evidence: none

### Option B · OSI fonts plus documented metric-compatible substitutes
Summary: Open fonts plus open metric-compatible substitutes for Windows core fonts.
Consequences: Migrated documents lay out correctly; substitutes must be inventoried.
Evidence: none

### Option C · Ship no fonts
Summary: Applications bundle their own.
Consequences: No policy needed; a broken first image.
Evidence: none

## Decision
Option B. The system ships fonts under OSI-approved licences (OFL and similar) and a documented substitution map from common proprietary font names to metric-compatible open fonts, so documents and applications from Linux and Windows render without layout shifts. No proprietary font is ever shipped.

## Consequences
- TXT owns the font store, the substitution map and its exposure to fontconfig and Wine (TXT-fontconfig bridge).
- GOV records each shipped font family and licence in the notices bundle.
- Users may add their own fonts; the map applies only when a requested family is absent.

## Rejected options and why
- Option A (OSI fonts only) rejected: documents created elsewhere would reflow, undermining the compatibility promise (§49).
- Option C (ship no fonts) rejected: the V0.5 applications could not render text out of the box.

## Follow-ups
none
