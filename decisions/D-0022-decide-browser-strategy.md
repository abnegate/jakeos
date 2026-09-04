# D-0022 · Decide browser strategy for 1.0 and the native WebView Component
- Status: proposed
- Task: APP-019
- Surfaces: none
- Layer: none
- Spikes: APP-021
- Supersedes: none
- Superseded by: none
- Baseline: §56.5, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§56.5 makes a browser existential for V1 daily driving and 1.0, so the Decision records which personality browsers ship, whether a native port is promised, and whether applications get a sandboxed native WebView Component (§56.5, §61).

## Options

### Option A · Chromium through the Linux personality
Summary: Chromium is the default browser, hosted by the personality.
Consequences: Widest site compatibility; the browser is not native and pulls in personality dependencies.
Evidence: none

### Option B · Firefox through the Linux personality
Summary: Firefox is the default browser, hosted by the personality.
Consequences: Independent engine and permissive licensing; some sites and DRM paths are weaker.
Evidence: none

### Option C · Both personality browsers
Summary: Both ship and one is default.
Consequences: User choice; double the packaging and update burden.
Evidence: none

### Option D · Native port
Summary: A browser engine is ported to the native ABI.
Consequences: Capability-scoped browsing; a multi-year effort that 1.0 does not promise.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
