# D-0266 · Decide whether an AI assistant is a distinct principal
- Status: proposed
- Task: SEC-034
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §44, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V2 AI demo is capability-only (§44, §57): an assistant invokes semantic interfaces (D-0278) with grants a user gave. This decision, made after the semantic registry exists (SEM-001, SEM-029), answers Q-037: whether the assistant is a distinct principal for audit and revocation, acts as the user, or has a distinct audit identity with user-held grants, and how grants are scoped (project, file, time) and revocation attributed (T-017, I-051). It sits on identity (D-0277) and the grant taxonomy (D-0269).

## Options

### Option A · Assistant is a distinct principal
Summary: The assistant is a distinct principal with its own root Capabilities; users delegate scoped grants to it and revoke them independently of their own.
Consequences: Every action is attributable to the assistant, revocation is one operation, and a compromised assistant holds only what was delegated. A second grant set to manage and display, and users must understand that giving the assistant a file is different from having the file.
Evidence: none

### Option B · Assistant acts as the user
Summary: The assistant acts as the user with the user's own grants.
Consequences: Nothing new to grant and the demo is trivial. Audit cannot tell the assistant from the user, revoking the assistant means revoking the user, and prompt-injection (T-017) inherits the user's full authority; rejected.
Evidence: none

### Option C · Hybrid with distinct audit identity and user-held grants
Summary: The assistant runs under a distinct audit identity but holds only Capabilities the user explicitly hands it per session or per task; grants are the user's and expire with the session.
Consequences: Attribution without a standing second principal, and the default is no authority. Session-scoped grants mean re-granting for recurring tasks, and the audit identity must be threaded through every semantic call.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
