# D-0322 · Decide hinting, subpixel positioning and gamma policy across scale factors
- Status: proposed
- Task: TXT-027
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V2 introduces fractional and per-display scaling. If native text and compatibility-window text are hinted, positioned or gamma-corrected differently, the difference is the visible signal that an application is not native, which §49 forbids (§41). This decision fixes one rendering policy (hinting mode, subpixel positioning, gamma) at 1x, 1.25x, 1.5x and 2x, and whether it is applied natively and projected into the personality's FreeType and fontconfig or set independently. It sits on the rasteriser configuration (TXT-013) and scaling (TXT-019).

## Options

### Option A · One policy applied natively and projected into personality FreeType/fontconfig
Summary: One policy (for example: no hinting or slight autohint, subpixel positioning on, linear-light blending with a fixed gamma) is defined natively and projected into the personality through generated fontconfig and FreeType properties, per scale factor.
Consequences: Text looks identical in native and compatibility windows at every scale, which is the §49 requirement. The projection must track fontconfig and FreeType option semantics, some Linux toolkits ignore parts of it, and any policy change is a two-place change.
Evidence: none

### Option B · Independent native and personality policies
Summary: Native text follows the native policy; personality applications keep whatever FreeType and fontconfig defaults their toolkit chooses.
Consequences: No projection to build. Visible mismatch between windows on the same screen, exactly the signal §49 forbids; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
