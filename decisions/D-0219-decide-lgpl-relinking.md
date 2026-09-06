# D-0219 · Decide how immutable Packages preserve LGPL relinking rights
- Status: proposed
- Task: PKG-010
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
LGPL section 6 lets a user substitute a modified copy of an LGPL library (glibc, Wine) into an application that links it. Content-addressed immutable Packages (D-0218) must still make that possible without mutating the original Package identity (§28), and GOV-003's licence firewall must be able to state the compliance path. This decision records I-069 as the standing rule and depends on how the SDK links personality libraries (SDK-026).

## Options

### Option A · Dynamically linked separate store objects the user can replace
Summary: LGPL libraries are separate store objects linked dynamically; a user installs a modified library object and a local generation resolves the application against it.
Consequences: Substitution is a normal dependency override and the original Package is untouched, so identity and signatures hold. Applications must link those libraries dynamically, the override must be expressible in the dependency model (D-0214), and a modified library breaks any content-identity assumption the application made about its dependencies.
Evidence: none

### Option B · Shipping relinkable object files inside the Package
Summary: Packages that statically link LGPL code ship the relinkable object files so a user can relink locally.
Consequences: Compliance works even for static linking. Every such Package grows by its object files, relinking requires the SDK toolchain on the user's machine, and the relinked result is a new local Package identity that updates will overwrite unless pinned.
Evidence: none

### Option C · Documented local-generation substitution flow
Summary: A documented flow: `os` exports the application's link inputs, the user relinks or substitutes, and installs the result as a local Package into a generation.
Consequences: The obligation is met with no constraint on how Packages link. It is a manual, developer-grade procedure that few users can follow, and the exported inputs must be kept available for every shipped version.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
