# D-0338 · Decide a clean-room policy for the Windows Personality
- Status: proposed
- Task: WIN-005
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Wine bring-up starts at V1, so the clean-room rules must exist first (§48): no disassembly of Microsoft binaries, exclusion of contributors exposed to leaked Windows source (I-070), and a rule for what Microsoft bits may exist inside a prefix. The last point is where a total ban collides with the per-prefix redistributables (Visual C++ runtimes, .NET, DirectX) that WIN-042 must ship for titles to run. GOV-003's licence firewall applies.

## Options

### Option A · Wine's rules as-is
Summary: Adopt Wine's clean-room rules unchanged: no disassembly, no leaked source, documentation-and-behaviour-based reimplementation.
Consequences: Proven over decades and understood by every Wine contributor the project might attract. Wine's rules are informal about contributor exposure, so the project relies on self-declaration, and they say nothing about what a prefix may contain.
Evidence: none

### Option B · Wine's rules plus contributor affidavits
Summary: Wine's rules plus a signed contributor affidavit for anyone touching the Windows personality.
Consequences: A documented paper trail for the exposure rule that GOV can point to. Friction for drive-by contributors, and the affidavit does nothing for the prefix question.
Evidence: none

### Option C · Ban on all Microsoft binaries in tree and prefixes
Summary: No Microsoft-authored binary anywhere: not in tree, not in any prefix.
Consequences: The purest legal position. Titles that require the Visual C++ runtime or .NET, which is most of the W corpora, cannot run, so WIN-042 is impossible; rejected.
Evidence: none

### Option D · Wine's rules plus Decision-listed per-prefix redistributables
Summary: Wine's rules plus contributor exclusion for leaked-source exposure, plus a list in this decision of redistributable Microsoft components (those whose licences permit redistribution) that WIN-042 may place in prefixes; anything not listed is fetched by the user.
Consequences: Bring-up can start, titles run, and the boundary between the project's tree and a user's prefix is explicit. The list is a legal artifact GOV maintains, and each entry's redistribution terms must be verified and recorded.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
