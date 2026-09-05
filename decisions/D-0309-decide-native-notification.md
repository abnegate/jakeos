# D-0309 · Decide native expression of termination, cancellation and async notification without signals
- Status: accepted
- Task: TSK-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §18, §19, §21
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Signals have no native equivalent (§1, §18); the notification model for termination, cancellation and async wake-ups must be named with rejected signal-like options.

## Options

### Option A · Operation completion plus Wait-able objects
Summary: Completions and Wait-able objects notify.
Consequences: Uniform and typed; no signals anywhere.
Evidence: none

### Option B · Typed Channel messages as the sole wake-up
Summary: Only Channel messages wake a Task.
Consequences: Simple; message overhead for every event.
Evidence: none

### Option C · Retained signal-like native event
Summary: A signal-like event exists.
Consequences: Familiar; rejected as a POSIX shape.
Evidence: none

## Decision
Option A. Termination, cancellation and asynchronous notification are all expressed as Operation completions and waitable Objects. Cancelling a TaskGroup completes every outstanding Operation with a typed Cancelled result; an Event Object is waited on like any other Operation; timers, device events and supervisor notices arrive as completions. Running code is never interrupted asynchronously.

## Consequences
- No signal-like handler exists in the native ABI; the Linux personality synthesises signals for its guests.
- Every blocking wait in the SDK is an Operation, so cancellation is uniform and deterministic (§21).
- The Operation kind set (ABI-014) includes Wait on Event and Timer from V0.

## Rejected options and why
- Option B (Channel messages as the sole wake-up) rejected: cancellation of an already blocked Operation would have to be indirect, and every concern needs its own channel.
- Option C (signal-like native event) rejected: it reintroduces the asynchronous interruption semantics §1 removes.

## Follow-ups
none
