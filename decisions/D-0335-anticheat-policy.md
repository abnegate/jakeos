# D-0335 · Decide the kernel-level anti-cheat policy
- Status: proposed
- Task: WIN-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §48, §56.2, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Kernel-level anti-cheat drivers demand exactly what the Capability model removes: unrestricted kernel presence and inspection of every process (§48, §56.2, §57). Bypassing anti-cheat is an explicit non-goal (I-071). This decision answers Q-043 and fixes the 1.0 policy: refuse outright, refuse in the personality and route those titles to the VIRT fallback, or engage vendors to load drivers. It names how excluded titles are disclosed in the compatibility database; vendor contracts (WIN-058) come later and may not relax it.

## Options

### Option A · Refuse kernel-level anti-cheat in 1.0
Summary: No anti-cheat kernel driver ever loads; titles that require one are rated Broken in the compatibility database with the reason stated.
Consequences: The kernel's integrity and the capability model are untouched, and the position is simple to state publicly. A visible share of popular multiplayer titles is unsupported with no path at all, which the W corpora and the HCL must present honestly.
Evidence: none

### Option B · Refuse in the personality and offer the VIRT fallback
Summary: No anti-cheat kernel driver loads in the JakeOS kernel; titles that require one are routed to the VIRT fallback VM (D-0159) where a guest Windows kernel may load whatever the vendor requires, and the database rates them as VM-only.
Consequences: Integrity of the host is preserved and users still have a path for those titles, with the rating telling them what to expect. Anti-cheat vendors increasingly detect and refuse VMs, so the path may not work for a given title, and VM graphics and input add latency the compatibility scale must reflect; this is the maintainer's recorded direction for the adr to confirm.
Evidence: none

### Option C · Vendor engagement loading anti-cheat kernel drivers
Summary: Pursue vendor-signed anti-cheat support inside the personality or kernel.
Consequences: The largest title coverage if vendors agree. A kernel driver with anti-cheat's requirements breaks the capability model and I-071's neighbours by construction, so this is acceptable only for a user-space-only vendor mode, and vendor engagement is WIN-058's question, not this one.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
