# D-0233 · Decide the suspend state for V1 reference machines
- Status: proposed
- Task: PWR-002
- Surfaces: none
- Layer: none
- Spikes: PWR-004
- Supersedes: none
- Superseded by: none
- Baseline: §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1-G07 requires automated suspend and resume cycling on H-002 and H-004 (§61). The platforms offer s2idle (suspend-to-idle) and S3; without one default the cycle harness (LAB-009) and the wake-source policy cannot be written. PWR-004 measures resume latency and residency for both. This decision names the default per reference machine, its inspectable identifier, the fallback rule, and the V1 wake sources (lid, power button, RTC in scope; USB wake off by default). Hibernation is a later decision.

## Options

### Option A · s2idle only
Summary: Every reference machine suspends to idle; S3 is never requested.
Consequences: One code path, one harness, and the state modern laptop firmware is validated for; resume is fastest. Idle residency and battery drain depend on every device driver's runtime PM behaviour, so a single misbehaving driver drains the battery overnight, and desktops that support S3 well pay more power in s2idle.
Evidence: `reports/spikes/PWR-004.md`

### Option B · S3 only
Summary: Every reference machine uses S3.
Consequences: Deepest sleep with predictable drain and few driver dependencies. Many current laptops (including the likely H-004 SKU) no longer support S3 or support it badly, so the gate machine may be unable to take the default at all.
Evidence: `reports/spikes/PWR-004.md`

### Option C · s2idle default with per-machine S3 fallback
Summary: s2idle is the default; a per-machine record in `registers/hardware.md` selects S3 when the spike shows s2idle residency or resume on that machine fails the PWR-004 criteria.
Consequences: Both reference machines pass V1-G07 with the state that works on them, and the rule for choosing is explicit and inspectable. Two paths to test in the harness, and the per-machine record is one more thing the HCL (HW-043) must carry for Tier 2 machines.
Evidence: `reports/spikes/PWR-004.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
