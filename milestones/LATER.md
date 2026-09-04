# LATER — Deferred work
- Sequence: 99
- Title: Deferred work
- Baseline: §57
- Hardware scope: none
- Surfaces to freeze: none
- Risks to retire: none

## Purpose

Parking rung for work deferred beyond 1.0. Nothing outside LATER may depend on a LATER task. Promoting an item means changing its Milestone first. The 1.0 2.0-planning RFC cites this file without depending on it.

## Not in this milestone

Any work required by a 1.0 gate. Any freeze. Any hardware gate.

## Gates

LATER has no gates. Rank 99 is excluded from 1.0 progress and counted in totals.

## Demos

LATER has no demos.

## Notes

Deferred beyond 1.0, matching I-093 and the 1.0 non-promises:

- ARM64 and RISC-V as supported platforms (the fork keeps them compiling; I-011, I-012)
- Native filesystem or content-addressed object store replacing the mature Linux filesystem
- Native GPU driver stack replacing DRM/KMS and Mesa
- Native browser or native IDE as shipped products
- Distributed and remote interfaces as a kernel concern
- Hardware capability enforcement (CHERI-class); ABI room is preserved
- Casting (Miracast, Chromecast, AirPlay), NFC, WWAN/eSIM, MIDI and pro-audio
- Kernel-level anti-cheat and vendor-proprietary DRM
- Enterprise directory join, MDM, fleet management, multi-seat, kiosk
- Formal security certifications (Common Criteria, FIPS 140)
- OEM certification programme (Q-055 may move this earlier)
- Application-state restore if Q-056 evidence is negative
- Kernel live-patching (I-086)
- Universal PC compatibility

<!-- roadmap:generated:begin milestone -->
Status: planned.
Gates: 0/0. Count: 0% (0/13). Weighted: 0%.

| Gate | Kind | Satisfied |
| --- | --- | --- |
<!-- roadmap:generated:end -->
