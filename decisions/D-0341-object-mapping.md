# D-0341 · Decide how Wine and Proton map onto native Objects
- Status: proposed
- Task: WIN-036
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §4, §48, §69
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
D-0345 chose how Wine is hosted at V1; V2 exit requires the architecture for how Wine and Proton map onto native Objects (§4, §48, §69): staying on Linux-personality syscalls with native UX chrome, replacing Wine's unixlib with Native ABI bindings, or a hybrid with native graphics and input. The accepted option names the native Object terminus for files, GPU, input, audio and clipboard (I-027) and sits on the NT fidelity decision (D-0340).

## Options

### Option A · Wine stays on Linux-personality syscalls with native UX chrome
Summary: Wine keeps calling Linux syscalls through the personality; native integration is chrome: window decorations, launcher entries, notifications and the chooser reached through portals.
Consequences: Lowest risk and every Proton release drops in. Files, GPU, input and audio terminate in Linux objects behind the personality, so Windows applications never hold native Capabilities directly and the terminus for I-027 is the personality, not the Object.
Evidence: none

### Option B · unixlib replaced by Native ABI bindings
Summary: Wine's unixlib layer is replaced by Native ABI bindings: files are storage objects, GPU is the native Vulkan path, input and audio are native Interfaces, the clipboard is S-032.
Consequences: Windows applications are one personality deep with native Objects as the terminus everywhere. A port of unixlib that must track Wine releases, Proton's Linux-specific pieces need native equivalents, and every terminus is a new binding to test against the W corpora.
Evidence: none

### Option C · Hybrid with graphics and input native
Summary: Graphics (Vulkan and DXVK), input and audio bind natively; files, process and NT semantics stay on the personality.
Consequences: The latency-critical paths are native and the rest is upstream Wine. The seam splits one process between two authority models, and files (the largest surface) still terminate in the personality.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
