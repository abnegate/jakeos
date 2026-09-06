# D-0270 · Decide 1.0 multi-user and per-user encryption scope
- Status: proposed
- Task: SEC-042
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V3 ships multi-user sessions, so the 1.0 scope must be fixed first (§9.1, §63): how many local accounts, whether one graphical session switches between users or several run concurrently, and whether per-user home data is encrypted under keys unlocked at login (T-026, I-092). It sits on identity (D-0277) and the encryption layer (D-0268), and the choice drives APP's greeter, SVC's session supervision and STO's key handling.

## Options

### Option A · Single graphical session with multiple accounts
Summary: Multiple local accounts, one graphical session at a time; switching users ends the session.
Consequences: The simplest greeter and session model, and per-user keys are unlocked one at a time. Shared family machines lose running work on switch, which is below the comparison platforms.
Evidence: none

### Option B · Fast user switching of one graphical session
Summary: Multiple accounts and fast user switching: the current session is locked and kept alive while another user logs in; one active graphical session at a time.
Consequences: Matches user expectations and keeps work alive. The locked session's processes keep running with its keys in memory (T-026), so the ResourceDomain budgets and key policy of the inactive session must be stated.
Evidence: none

### Option C · Concurrent graphical sessions with per-user home keys
Summary: Concurrent graphical sessions on separate seats or virtual terminals with per-user home keys unlocked at each login.
Consequences: Full multi-user including multi-seat. Compositor and input broker must handle several seats, the threat model must cover one user observing another's session (T-026), and it is the largest APP and GFX effort of the three.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
