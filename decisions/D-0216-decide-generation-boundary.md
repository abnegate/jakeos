# D-0216 · Decide what is excluded from a SystemGeneration and how mutable state is separated
- Status: proposed
- Task: PKG-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A SystemGeneration is the immutable image the machine boots (§30). Rolling back must never rewrite user data, so the boundary between the generation and mutable state (user files, ApplicationData, logs, caches, settings) must be exact and each excluded class must have an owner (§31). This answers Q-023 and is a prerequisite for generation compose (PKG-014) and the 1.0 rollback guarantee.

## Options

### Option A · Exclude user data, ApplicationData, logs and caches
Summary: User data, ApplicationData, logs and caches live outside the generation tree in owner-managed mutable volumes; the generation contains only Packages, configuration defaults and the composed system.
Consequences: Rollback is a boot-entry switch that touches nothing the user made. Application state is unmanaged by the generation, so an application whose data schema changed between generations must handle its own downgrade, and restore of application state (Q-056) is a separate later mechanism.
Evidence: none

### Option B · Snapshot selected mutable trees into the generation
Summary: Selected mutable trees (settings, ApplicationData) are snapshotted into each generation at compose time.
Consequences: Rolling back also restores the application state that matched that generation, so schema mismatches cannot happen. Generations grow with user state, snapshots of live data need quiescing, and a rollback silently discards data written since, which users do not expect from an OS rollback.
Evidence: none

### Option C · Hybrid with explicit restorable classes
Summary: Mutable state is classified into named restorable classes (OS configuration, application settings, application data, caches); the generation records which classes are snapshotted with it and the rest stay outside.
Consequences: Precise: OS configuration rolls back with the generation, documents never do, and each class has one owning prefix and one documented behaviour. Classification must be enforced at the API (STO-018 typed objects), and every application must place its state in the right class for the promise to hold.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
