# GPU-only versioning notes

## Current package metadata

- package name: `crabgrab`
- current version: `0.4.0`
- repository: `https://github.com/ihasq/wgrab`

## Planned API direction

The future wgrab API is GPU-only and centered on `WgpuCaptureFrame`.

## Versioning options

### Option A: stay on `crabgrab`, bump to `0.5.0`

Pros:

- minimal Cargo dependency change
- preserves existing crate identity if ownership permits

Cons:

- may conflict with crates.io ownership
- name no longer reflects successor branding

### Option B: publish as `wgrab`

Pros:

- matches successor repo
- clean break for GPU-only API

Cons:

- users must update dependency name
- requires compatibility documentation

### Option C: Git dependency only for now

Pros:

- no crates.io decision yet
- fastest path for GPU-only iteration

Cons:

- less discoverable
- less stable for downstream users

## Owner decision

No package or version change in Phase 17X.

## Owner decision needed before removal

Removal of deprecated APIs is a breaking change.

Versioning candidates:

### Candidate 1: `0.5.0`

Use if staying on `crabgrab` package lineage.

Meaning:

- first GPU-only breaking release
- deprecated CPU/raw APIs removed
- wgpu 29 baseline retained

### Candidate 2: `0.1.0` under `wgrab`

Use if publishing as a new successor crate.

Meaning:

- new crate identity
- GPU-only API from the start
- CrabGrab compatibility only through migration docs

### Candidate 3: Git-only pre-release

Use if package name remains undecided.

Meaning:

- no registry publish
- removal can proceed on branch
- downstream users pin Git revision

## Phase 18P decision

No package or version change is made in Phase 18P.

## Phase 19X status

Deprecated APIs have been removed on the Git dependency branch.

Package name and version remain unchanged until the owner decides the release
channel.

Distribution strategy remains Option C: Git dependency only.
