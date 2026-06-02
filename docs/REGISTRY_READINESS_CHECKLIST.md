# Registry readiness checklist

## Package identity

- [ ] Confirm whether `wgrab` is available on crates.io.
- [ ] Confirm whether publishing as `crabgrab` is possible.
- [ ] Decide package name.
- [ ] Decide version.
- [ ] Decide whether dependency aliasing should be documented.

## API readiness

- [ ] GPU-only primary API documented.
- [ ] Legacy CPU/raw output APIs removed.
- [ ] Migration guide updated.
- [ ] Lower-level `get_wgpu_texture` documented as advanced GPU API.
- [ ] Diagnostic APIs documented as public.

## CI readiness

- [ ] Windows DX12 backend smoke passes.
- [ ] macOS Metal arm64 backend smoke passes.
- [ ] macOS Metal Intel backend smoke passes.
- [ ] Ubuntu Lavapipe Vulkan smoke passes.
- [ ] GPU-only texture probe marker gate passes.

## Docs readiness

- [ ] README reflects GPU-only API.
- [ ] generated docs policy decided.
- [ ] stale generated docs removed or regenerated.
- [ ] CHANGELOG updated.
- [ ] release notes prepared.

## Publish readiness

- [ ] crates.io token available, if publishing.
- [ ] dry-run package check performed.
- [ ] package contents reviewed.
- [ ] owner approves irreversible registry publish.

## Phase 21P status

- [ ] Confirm `wgrab` availability on crates.io UI.
- [x] Rename package to `wgrab`.
- [x] Set initial version to `0.1.0`.
- [x] Run `cargo package`.
- [x] Run `cargo publish --dry-run`.
- [x] Review package contents.
- [ ] Get owner approval before real publish.
