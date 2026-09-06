# D-0280 · Decide the Automation rule format
- Status: proposed
- Task: SEM-018
- Surfaces: none
- Layer: none
- Spikes: SEM-034
- Supersedes: none
- Superseded by: none
- Baseline: §13, §45, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Automation rules act on the user's behalf over semantic interfaces (§45). The format decides how much a rule can express, how it is inspected and how it is sandboxed: declarative rules, a scripting language, or Wasm modules (§13, §62). Every rule holds only delegated Capabilities and a background rule requires `Capability<BackgroundExecution>`; a Wasm option uses WASM-015's host and does not make Wasm the Native ABI. SEM-034's spike is the evidence.

## Options

### Option A · Declarative when-event if-condition action rules
Summary: Rules are declarative documents: when an event, if conditions on typed objects hold, invoke listed semantic actions with bound arguments.
Consequences: Rules are data: inspectable, diffable, safe to restore, and the editor can show exactly what a rule may touch. Anything needing computation (string transforms, arithmetic, loops) is impossible until a function library exists, so power users hit the ceiling early.
Evidence: `reports/spikes/SEM-034.md`

### Option B · Sandboxed scripting language
Summary: Rules are scripts in a sandboxed language (a small embedded language or a restricted subset of an existing one) with the semantic actions as its API.
Consequences: Expressive enough for real automation. The sandbox is a security boundary the project must own, the language is a Layer 2 surface to version, and inspection of what a script may do requires running or analysing it.
Evidence: `reports/spikes/SEM-034.md`

### Option C · Wasm automation modules
Summary: Rules are Wasm Components using the WASM-015 host, with semantic actions as WIT imports.
Consequences: Any language compiles to it and the sandbox is the one the platform already has (D-0332, D-0333). Heavier than a declarative rule for the common case, tooling for authoring rules becomes a compiler toolchain, and the rule editor cannot show a module's behaviour.
Evidence: `reports/spikes/SEM-034.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
