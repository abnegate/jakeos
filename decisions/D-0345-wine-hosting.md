# D-0345 · Decide whether Wine hosts on the Linux Personality or the Native ABI
- Status: proposed
- Task: WIN-013
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §4, §48, §69
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V1 non-gated Wine bring-up needs an architecture: Wine over the Linux personality as it runs on any distribution, Wine ported onto the Native ABI, or a hybrid where graphics and input are native and the rest stays on Linux (§3, §4, §48, §69). WIN-014's spike report is the input; the V2 object mapping (WIN-036) follows this choice. The accepted option states what native software still must not see (I-007) and which V2 tasks change with the host.

## Options

### Option A · Wine over the Linux personality
Summary: Wine and Proton run unmodified inside the Linux personality; Windows windows reach the compositor through the Wayland bridge like any Linux application.
Consequences: Lowest risk and fastest bring-up, with Proton's tested stack (DXVK, VKD3D) intact. Windows applications are two personalities deep, every frame and input event crosses both bridges, and native integration (Capabilities, typed objects) is unavailable until the V2 mapping.
Evidence: `reports/spikes/WIN-014.md`

### Option B · Wine ported onto the Native ABI
Summary: Wine is ported onto the Native ABI: wineserver and the loader become native Components, and Win32 maps directly to Objects, Channels and MemoryObjects.
Consequences: One personality deep, native integration from the start, and the WIN-036 mapping is the design rather than a retrofit. A very large port of a fast-moving upstream that must be rebased on every Wine release, and Proton's Linux-specific pieces (esync, fsync, DXVK's Vulkan loader) need native equivalents.
Evidence: `reports/spikes/WIN-014.md`

### Option C · Hybrid with graphics and input native
Summary: Wine runs on the Linux personality for process, file and NT semantics, but its graphics (Vulkan, DXVK) and input paths are bound to native Surfaces and input directly.
Consequences: The latency-critical paths skip the Wayland bridge while the bulk of Wine stays upstream. Two hosting paths inside one process, Wine's winex11 or winewayland driver is replaced by a native driver the project maintains, and the seam between Linux and native halves is where bugs will live.
Evidence: `reports/spikes/WIN-014.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
