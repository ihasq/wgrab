# wgrab 第18Pフェーズログ

## 結論

- package strategy: documented
- versioning notes: documented
- Git dependency docs: documented
- lower-level GPU API policy: documented
- generated docs audit: completed
- target checks: success
- push CI: pending after commit
- ready for owner package decision: pending push CI

## Branch

- branch: `wgrab-gpu-only-api-design`
- commit: pending docs commit
- status: pending push CI

## Package options

- Option A: continue as `crabgrab`, likely `0.5.0` for a GPU-only breaking release if crates.io ownership permits.
- Option B: publish as `wgrab`, likely `0.1.0` as a new successor crate identity.
- Option C: stay Git-only until package naming and publishing strategy are settled.

## Current recommendation

Keep package metadata unchanged in Phase 18P, do not publish, document Git dependency usage, and choose the final package name before the first GPU-only release.

## Generated docs

- candidates: `docs/macos_docs` (5.3M), `docs/windows_docs` (4.8M)
- decision: no generated docs are deleted in Phase 18P; keep temporarily pending ownership and release-docs review

## Owner decisions still needed

- package name: undecided
- version: undecided
- publish channel: undecided
- generated docs: regenerate, remove, or keep temporarily
- removal timing: before or after first GPU-only registry release

## Target checks

- Windows `cargo check --features wgpu`: success
- macOS x86 `cargo check --features wgpu`: success
- macOS arm `cargo check --features wgpu`: success
- error index: empty
