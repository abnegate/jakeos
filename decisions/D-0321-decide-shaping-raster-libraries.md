# D-0321 · Decide the shaping and rasterisation libraries for the native text stack
- Status: proposed
- Task: TXT-003
- Surfaces: none
- Layer: none
- Spikes: TXT-011
- Supersedes: none
- Superseded by: none
- Baseline: §41, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every V0.5 application renders text through one shaping and rasterisation stack (§41). TXT-011 measures the candidates for correctness on the script list, shaping and raster latency and unsafe surface. §67 Principle 15 says a mature library is not replaced without a recorded benefit, and R-019 is the risk that a minimal V0.5 path hardens into the permanent design. The accepted option names the libraries and the crate boundary the toolkit links.

## Options

### Option A · Retain HarfBuzz plus FreeType
Summary: HarfBuzz for shaping and FreeType for rasterisation, wrapped behind a Rust crate boundary that owns all unsafe calls.
Consequences: The reference implementations, correct on every script in the D-0316 matrix, with decades of edge cases handled and the same output as the personalities' own text. A C unsafe surface parses untrusted font bytes (T-030), so placement (D-0323) must isolate it, and the crate boundary is the only place the toolkit may touch it.
Evidence: `reports/spikes/TXT-011.md`

### Option B · Rust-native rustybuzz, swash or cosmic-text
Summary: A Rust-native stack: rustybuzz for shaping and swash (or the cosmic-text combination) for rasterisation and layout.
Consequences: No unsafe font parsing in the text path and a single-language build. Coverage of complex scripts and hinting quality lag the C libraries and must be verified per script in the spike; divergence from HarfBuzz output means Linux applications and native applications shape the same text differently.
Evidence: `reports/spikes/TXT-011.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
