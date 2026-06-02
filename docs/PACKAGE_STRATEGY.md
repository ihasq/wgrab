# wgrab package strategy

## Current metadata

- package name: `crabgrab`
- version: `0.4.0`
- repository: `https://github.com/ihasq/wgrab`

## Goal

wgrab is moving toward a GPU-only API centered on `WgpuCaptureFrame`.

Before removing deprecated APIs, package and version strategy must be decided.

## Option A: continue as `crabgrab`

### Pros

- dependency name remains familiar
- downstream users can migrate with less Cargo.toml churn
- matches original API lineage

### Cons

- crates.io ownership may not be available
- name does not reflect successor project
- GPU-only breaking API may surprise existing `crabgrab` users

### Status

Do not choose until crates.io ownership is confirmed.

## Option B: publish as `wgrab`

### Pros

- matches successor repository
- clean branding for GPU-only API
- clear break from archived upstream

### Cons

- crate availability must be confirmed
- downstream users must update dependency name
- existing imports may require dependency aliasing

### Possible dependency alias

```toml
crabgrab = { package = "wgrab", version = "..." }
```

## Option C: Git dependency only

### Pros

- no crates.io decision required
- fastest iteration path
- useful before breaking API stabilizes

### Cons

- less discoverable
- downstream users pin commits manually
- no registry semver

## crates.io checks required before publishing

- Is `crabgrab` ownership available to this maintainer?
- Is `wgrab` available?
- If using `wgrab`, should downstream users alias it as `crabgrab`?
- Should the first GPU-only release be registry-published or Git-only?

## Recommended interim decision

Before removal implementation:

- keep package metadata unchanged
- do not publish
- document Git dependency usage
- choose final package name before first GPU-only release

## Phase 19X decision

The owner selected Option C: Git dependency only.

No package name, version, crates.io publishing, tag, GitHub Release, or generated
docs change is made in Phase 19X.

## Phase 20P owner decision

Current distribution mode:

- Git dependency only

Registry package candidate:

- first choice: `wgrab`
- fallback: `crabgrab` only if ownership and compatibility concerns are resolved

No package metadata is changed in Phase 20P.

## Why Git-only now

The GPU-only branch removed deprecated CPU-readable and raw platform output APIs.
This is a breaking API direction.

The branch should remain Git-only until the owner decides:

1. final package name
2. version number
3. registry publish policy
4. compatibility messaging
