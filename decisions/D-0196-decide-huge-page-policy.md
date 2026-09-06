# D-0196 · Decide the per-Component page-table and huge-page policy for MemoryObjects
- Status: proposed
- Task: MEM-021
- Surfaces: none
- Layer: none
- Spikes: MEM-031
- Supersedes: none
- Superseded by: none
- Baseline: §16, §34, §38
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Warm startup of Terminal and Editor (B-016 via CMP-019) depends on how cheaply verified immutable pages from the store are mapped and shared between Components (§34). MEM-031 measures per-Component page-table cost and huge-page TLB effects. This decision picks the mechanism without exposing page-table layout or Linux mechanism names on the ABI (§38, §65): no native API may say THP or hugetlbfs.

## Options

### Option A · Shared page-table fragments
Summary: Immutable MemoryObject mappings share page-table fragments across Components, so mapping a store object a second time costs a pointer, not a page walk.
Consequences: Startup cost and memory for shared code and read-only data drop for every Component that maps the same object, which is every application on the same runtime. Shared page tables are invasive kernel work with subtle invalidation rules, and a bug is a cross-Component information leak.
Evidence: `reports/spikes/MEM-031.md`

### Option B · Transparent huge pages
Summary: MemoryObjects rely on transparent huge pages from the retained mm with no native property.
Consequences: Nothing new in the ABI and TLB reach improves where khugepaged happens to collapse pages. Behaviour is heuristic and workload-dependent, so startup numbers vary run to run, and the SDK cannot ask for or verify the layout.
Evidence: `reports/spikes/MEM-031.md`

### Option C · Explicit huge-page property on the MemoryObject
Summary: A MemoryObject carries an explicit page-size property requested at creation and honoured or rejected with a typed error.
Consequences: Deterministic layout for code, atlases and large buffers, and `os inspect` can show it. A property on the object is Layer 1 ABI to version, and honouring it needs reserved contiguous memory, so rejection paths and fallbacks must be specified.
Evidence: `reports/spikes/MEM-031.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
