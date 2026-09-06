# D-0323 · Decide whether shaping runs in-Component or in a shared text service Component
- Status: proposed
- Task: TXT-004
- Surfaces: none
- Layer: none
- Spikes: TXT-011
- Supersedes: none
- Superseded by: none
- Baseline: §41, §10, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Where shaping and rasterisation run changes isolation and latency for every UI Component (§10, §41). A shared text service parses untrusted font bytes on behalf of everyone (T-030) and a shared writable glyph atlas is a cross-Component write channel (I-083), while per-Component shaping duplicates caches. TXT-011 supplies the latency notes. The accepted option states where untrusted font bytes are parsed and that no shared writable atlas exists (§51).

## Options

### Option A · In-Component library
Summary: Every Component links the text stack and shapes and rasterises its own glyphs.
Consequences: A font-parsing exploit compromises only the Component that loaded the font, and no cross-Component state exists. Each Component pays cold shaping and caching, glyph atlases are duplicated per application, and startup (B-016) includes font loading.
Evidence: `reports/spikes/TXT-011.md`

### Option B · Shared system text service
Summary: A system text service shapes and rasterises for all Components and hands back glyph MemoryObjects.
Consequences: One warm cache, one copy of every atlas, and fonts load once per session. The service is a single font-parsing attack surface with everyone's text passing through it (T-030), it is on the latency path of every frame, and its restart must not lose glyphs mid-frame.
Evidence: `reports/spikes/TXT-011.md`

### Option C · Hybrid: library shaping, service-minted read-only caches
Summary: Components shape and rasterise in-process with the library; a text service mints read-only, content-addressed glyph atlas MemoryObjects for system fonts that Components map immutably.
Consequences: System fonts are parsed once by the service and shared read-only, user-supplied fonts are parsed only inside the Component that trusts them, and no shared writable atlas exists (I-083). Two code paths (service atlas hit, local fallback), and the atlas format is a Layer 2 contract between service and library.
Evidence: `reports/spikes/TXT-011.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
