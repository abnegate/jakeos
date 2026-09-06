# D-0223 · Decide how Linux and Windows compatibility applications are packaged immutably
- Status: proposed
- Task: PKG-047
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §28, §36
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Compatibility applications arrive as OCI images, Flatpak bundles and Wine prefixes, yet every installed thing must be an immutable Package (§28, §3) that satisfies the no-mutation rule (D-0218). The V1 L2 corpus needs OCI and Flatpak import; Wine prefixes follow with the Windows personality (§36). This answers Q-020 and records that personality install never writes a mutable global prefix or image store; native software sees none of these formats.

## Options

### Option A · OCI layers as content-addressed objects
Summary: OCI image layers are imported as content-addressed store objects and an OCI application is a Package whose Components run in the personality over those layers.
Consequences: Deduplication across images that share base layers, and the OCI toolchain (registries, signatures) is reused for fetch. Layer semantics (whiteouts, overlay order) must be materialised by the personality at start, and image mutation inside a running container is ApplicationData, not the layer.
Evidence: none

### Option B · Flatpak bundles wrapped as Packages
Summary: Flatpak bundles and their runtimes are wrapped as Packages whose contents are the bundle's ostree objects; portals carry their grants (D-0179).
Consequences: Sandbox metadata maps onto Capability requests and the store deduplicates runtimes. The Flatpak runtime expects its own ostree repository layout and update path, which must be redirected into the store without breaking `flatpak` tooling that applications call.
Evidence: none

### Option C · Wine prefixes as immutable base layers plus ApplicationData overlays
Summary: A Wine prefix is an immutable base layer built at install (registry, DLLs, redistributables) plus a per-application ApplicationData overlay for everything the application writes at run time.
Consequences: Reinstall and rollback return the prefix to a known state and the base layer deduplicates across titles. The overlay must capture registry writes, installers that expect to modify the prefix need the install itself to be the layer-building step (Q-041), and Wine's own prefix update logic runs against a read-only base.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
