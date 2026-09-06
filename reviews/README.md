# Reviews

Adversarial reviews of the design plan, the architecture baseline and the roadmap itself. Each review is a numbered file: what was examined, every finding with its severity, what was changed in the same series of commits, and what was deliberately left alone with the reason. Findings that became work are tasks, register entries or decisions and are cited by ID; a review never carries a requirement that has no ID.

Reviews are hand-written and are not read by the `roadmap` tool.

| Review | Scope |
|---|---|
| [01 · Architecture and roadmap nitpick](01-architecture-and-roadmap.md) | Line-by-line pass over BASELINE.md, THREAT-MODEL.md, CONVENTIONS.md, every milestone file, the registers and all 217 V0 tasks |
| [02 · Compounding and swarm readiness](02-compounding-and-swarm-readiness.md) | Dependency-order walk of V0 to V2, cross-task consistency sweeps, unresolved gate quantities, hosting of inherited C stacks, mis-scheduled hardware |
| [03 · Walking back from 1.0](03-reverse-chain-from-1.0.md) | Closure of every 1.0 and V4 gate traced to V0 roots; missing lab resources, cross-task criteria outside their closure, inventory formats, re-weighing of earlier defaults |
