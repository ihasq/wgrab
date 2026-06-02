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
