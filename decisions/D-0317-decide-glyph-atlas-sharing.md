# D-0317 · Decide the cross-Component glyph atlas and shaped-text cache sharing model
- Status: proposed
- Task: TXT-015
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §51, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 daily driving runs many text-rendering Components at once. D-0323 placed shaping; this decision fixes how glyph rasters and shaped runs are shared between Components: per-Component caches, a read-only atlas minted by the text service, or a shared writable atlas (§41, §67). A shared writable atlas is a cross-domain channel (I-083, T-030, T-015) that SEC-029 governs. The accepted option states the MemoryObject rights of atlas pages and that clients cannot map them writable (§51).

## Options

### Option A · Per-Component caches
Summary: Each Component rasterises and caches its own glyphs; nothing is shared.
Consequences: No cross-Component state and no side channel. Every application holds its own copy of the system font glyphs, memory grows with the number of text-rendering Components (B-008 idle overhead), and cold caches show in startup.
Evidence: none

### Option B · Read-only atlas minted by the text service
Summary: The text service rasterises system fonts into content-addressed atlas MemoryObjects that Components map read-only; each Component adds a small private cache for its own fonts and sizes.
Consequences: One copy of the common glyphs, warm on first use, and read-only mapping means no writer can influence another Component. The service is on the path of cache misses, atlas pages must be immutable once published, and eviction of a shared page needs a generation scheme so a mapped page is never reused underneath a client.
Evidence: none

### Option C · Shared writable atlas
Summary: A shared writable atlas any Component may add glyphs to.
Consequences: The most memory-efficient and the simplest cache. A writable shared page is a covert channel and a corruption vector between Components (I-083, T-015), and a malicious writer can replace glyphs another application displays (T-030); rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
