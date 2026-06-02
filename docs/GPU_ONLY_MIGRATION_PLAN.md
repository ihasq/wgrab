# GPU-only migration plan

## Phase 1: Design and audit

- Document GPU-only policy.
- Audit public API.
- Classify CPU/raw platform outputs.
- Do not remove APIs yet.

## Phase 2: Introduce GPU-only API

- Add `WgpuCaptureStream`.
- Add `WgpuCaptureFrame`.
- Add `texture()`, `size()`, `format()`.
- Add backend mismatch errors.
- Keep old APIs temporarily if needed.

## Phase 3: Deprecate CPU/raw output APIs

- Mark CPU-readable APIs as deprecated.
- Mark raw platform output APIs as deprecated unless explicitly retained as unsafe/internal.
- Add migration docs.

## Phase 4: Remove deprecated APIs

- Remove CPU-readable output APIs.
- Remove raw platform output APIs from the primary public surface.
- Keep internal platform interop private.

## Phase 5: Release as wgrab GPU-only baseline

- Decide package name.
- Decide version.
- Decide crates.io strategy.

## Current implementation phase

Phase 2 has started.

The new GPU-only API is being introduced without removing legacy APIs yet.

## Phase 3 status

Legacy CPU-readable and raw platform output APIs are now being marked as
deprecated.

No API is removed in this phase.
