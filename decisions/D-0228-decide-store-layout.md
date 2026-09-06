# D-0228 · Decide the Content-addressed store layout for Packages and SystemGenerations
- Status: proposed
- Task: PKG-014
- Surfaces: none
- Layer: none
- Spikes: PKG-040
- Supersedes: none
- Superseded by: none
- Baseline: §27, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Packages, SystemGenerations and development environments share one persistent content-addressed store (§27, §30). V0.5 exit requires the layout so PKG-014 can build it and STO-017 can map it onto the chosen filesystem. PKG-040 measures deduplication ratio and update size for the candidate layouts; the accepted layout must make a new SystemGeneration cost only its changed objects.

## Options

### Option A · Nix-style store paths
Summary: Nix-style store: each object is a directory tree under a path that embeds its content hash, and references between objects are literal store paths.
Consequences: Proven at scale by Nix and Guix, trivially inspectable with ordinary tools, and a generation is a tree of links into it. Deduplication is per whole object, so a one-file change duplicates the tree, and paths embed hashes that personality software occasionally trips over; hash-in-path means relocation is impossible by design.
Evidence: `reports/spikes/PKG-040.md`

### Option B · OSTree-style object repository
Summary: OSTree-style repository: every file is a content-addressed object, directory trees are metadata objects, and a generation is a commit checked out via hardlinks.
Consequences: File-level deduplication across Packages and generations, cheap commits, and a checkout that looks conventional to personality software. Checkout time grows with the number of files, hardlink farms require a filesystem that supports them and confuse tools that count inode links, and metadata objects are a second format to sign.
Evidence: `reports/spikes/PKG-040.md`

### Option C · casync-like chunk store
Summary: casync-like chunk store: objects are stored as content-defined chunks and reassembled into an image or tree on demand.
Consequences: Best deduplication and the smallest updates, and it pairs naturally with verified images (D-0217 option C). Reassembly costs time and space at install, chunk indexes are the unit of trust, and there is no directly inspectable tree without a materialisation step.
Evidence: `reports/spikes/PKG-040.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
