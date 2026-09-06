# D-0318 · Decide hosting existing IME engines versus native engines and the 1.0 language list
- Status: proposed
- Task: TXT-026
- Surfaces: S-016
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
CJK, Indic and Vietnamese users cannot type without input-method engines (§41), and the V4 CJK gate needs engines from V2 onward. Principle 15 (§67) argues against rewriting mature engines. This decision fixes sandboxed hosting of existing engines (librime, libpinyin, anthy, libhangul, m17n) versus writing native engines, records the 1.0 input-language list with the engine for each, and shapes the IME protocol surface S-016 (Layer 2). It sits on the IME protocol (TXT-021) and the input-method service (TXT-022).

## Options

### Option A · Sandboxed hosting of librime, libpinyin, anthy, libhangul, m17n
Summary: Existing engines run as sandboxed helper Components (personality-hosted per SDK-097) behind the native IME protocol; the 1.0 list is Simplified and Traditional Chinese (librime), Japanese (anthy or libkkc via librime), Korean (libhangul), Vietnamese and Indic scripts (m17n), Latin dead keys natively.
Consequences: Coverage on the first day with engines their communities already trust, and a compromised engine sees only the text service's Channel. Each engine is C code hosted under the C-library strategy, dictionaries and user data must live in ApplicationData, and latency adds a Component hop per keystroke that the input-to-photon rig will show.
Evidence: none

### Option B · Native-written engines
Summary: Native Rust engines written for each 1.0 language.
Consequences: Exact fit with the protocol and no hosting. Input-method engines embody decades of linguistic data and behaviour; a rewrite before 1.0 is Principle 15's textbook violation and would ship worse conversion than users have today.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
