# D-0194 · Decide encrypted MemoryObject key ownership and hardware encryption
- Status: proposed
- Task: MEM-045
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §16, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Who owns the key for an encrypted MemoryObject, who may map plaintext and whether hardware memory encryption is a software-visible map or a provider property must be decided (§16, §51).

## Options

### Option A · Per-Component key in the secrets service
Summary: Each Component's key lives in the secrets service.
Consequences: Fine-grained authority; key management per Component.
Evidence: none

### Option B · ResourceDomain-held key
Summary: The ResourceDomain holds the key.
Consequences: Simpler sharing within a domain; coarser isolation.
Evidence: none

### Option C · Hardware memory encryption with no software plaintext map
Summary: SME/TDX-class encryption with no plaintext mapping in software.
Consequences: Strongest guarantee; depends on hardware.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
