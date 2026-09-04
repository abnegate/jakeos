# D-0026 · Decide the shared PDF renderer for viewer, thumbnails and print preview
- Status: proposed
- Task: APP-053
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §11, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
One shared sandboxed PDF renderer must serve viewer, thumbnails and print preview so the same attack surface is not shipped four times (§11, §51), as an isolated Component (T-038) within the GOV licence allowlist.

## Options

### Option A · pdfium
Summary: pdfium is the shared renderer.
Consequences: Mature and permissively licensed; large C++ codebase to sandbox.
Evidence: none

### Option B · poppler
Summary: poppler is the shared renderer.
Consequences: Widely used on Linux; GPL licensing constrains where it can be linked.
Evidence: none

### Option C · Rust renderer
Summary: A Rust PDF renderer is the shared renderer.
Consequences: Memory safety and native fit; less complete format coverage.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
