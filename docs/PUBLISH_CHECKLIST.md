# wgrab 0.1.0 publish checklist

## Identity

- [x] package name: `wgrab`
- [x] version: `0.1.0`
- [x] repository: `https://github.com/ihasq/wgrab`
- [x] license present
- [x] README present
- [x] CHANGELOG present

## Name availability

- [ ] owner confirmed `wgrab` availability on crates.io UI
- [x] `cargo search wgrab` checked

## Local verification

- [x] `cargo package` success
- [x] `cargo publish --dry-run` success
- [x] package contents reviewed
- [x] no logs included
- [x] no `.github` included
- [x] no generated docs included
- [x] no target artifacts included

## CI

- [x] Windows DX12 backend smoke success
- [x] macOS Metal arm64 backend smoke success
- [x] macOS Metal Intel backend smoke success
- [x] Ubuntu Lavapipe Vulkan smoke success
- [x] GPU-only texture probe marker gate success

## WebGPU roadmap

- [x] WebGPU / web_sys support documented as future roadmap
- [x] WebGPU / web_sys support is not a `wgrab 0.1.0` publish blocker

## Audio roadmap

- [x] Audio capture documented as future roadmap
- [x] Audio capture is not a `wgrab 0.1.0` publish blocker

## Owner approval

- [x] owner explicitly approves real publish
- [x] publish method selected

## Publish method

Selected:

- [x] manual `cargo publish`
- [ ] trusted publishing later

## Post-publish

- [x] crates.io page visible
- [x] `cargo info wgrab` works
- [x] `cargo install` not applicable / not tested
- [x] README badge updated if needed
- [x] publish result documented
