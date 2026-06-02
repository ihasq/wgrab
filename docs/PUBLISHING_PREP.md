# wgrab publishing preparation

## Goal

Publish `wgrab` to crates.io.

## Selected package identity

- package: `wgrab`
- version candidate: `0.1.0`
- repository: `https://github.com/ihasq/wgrab`
- publish status: not published in Phase 21P

## Preflight

- cargo search: no `wgrab` result returned
- cargo package: success
- cargo publish --dry-run: success
- target checks: success
- CI: pending

## Package contents review

- crate file: `target/package/wgrab-0.1.0.crate`
- size: 84K on disk, 80.5KiB compressed in cargo output
- generated docs included: no
- logs included: no
- concerns: direct crates.io page fetch returned HTTP 403 in this environment; owner UI confirmation is still required

## Owner approval required before publish

Real publishing is irreversible as a registry action and requires explicit owner
approval.

## WebGPU roadmap

WebGPU / web_sys support is a future roadmap item.

It is not a blocker for publishing `wgrab 0.1.0`.
