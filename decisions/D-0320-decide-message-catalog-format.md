# D-0320 · Decide the message catalog format between Fluent and gettext
- Status: accepted
- Task: TXT-017
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §52, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
First-party applications accumulate string sites from V0.5, so the catalog format and SDK string API must be decided beside SDK v1 (§41, §52, §66).

## Options

### Option A · Fluent
Summary: Fluent catalogs.
Consequences: Expressive plurals and gender; tooling.
Evidence: none

### Option B · gettext
Summary: gettext catalogs.
Consequences: Ubiquitous; weak plurals.
Evidence: none

## Decision
Option A. Message catalogs use Fluent. Plurals, genders and grammatical variants are expressed by translators in the catalog, not in code; the Rust implementation is used by the SDK and the shell.

## Consequences
- The localisation pipeline (TXT) exports Fluent files per language and measures coverage with B-052.
- gettext catalogs are supported only inside the Linux personality for guests.
- Translation tooling is chosen from tools that understand Fluent.

## Rejected options and why
- Option B (gettext) rejected: plural and variant logic would live in application code, and its universal tooling is a smaller advantage than translator-side grammar handling.

## Follow-ups
none
