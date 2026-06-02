# CrabGrab wgpu29 第6フェーズログ

## 結論

- PR created: 未実施
- PR Actions: 未実施
- backend smoke: latest branch Actions success
- capture marker: marker-gated
- warning cleanup: 一部実施、既存 warning は deferred
- dependency policy: 文書化
- required checks candidate: 文書化
- ready for review: PR 作成後の Actions 確認待ち

## PR

- URL: https://github.com/ihasq/wgrab/pull/new/wgpu29-phase5c-marker-hardening
- branch: wgpu29-phase5c-marker-hardening
- commit: 80b6bcbfbbbed270bcb57700ac302cc702557876 以降

PR は GitHub UI または認証済み GitHub CLI/API が必要。ローカル環境には
`gh` / `hub` がなく、未認証 API では PR 作成できないため未作成。

## CI result

### wgpu runtime

- run URL: https://github.com/ihasq/wgrab/actions/runs/26808014483
- Windows DX12: success
- macOS arm64: success
- macOS Intel: success

### wgpu linux vulkan smoke

- run URL: https://github.com/ihasq/wgrab/actions/runs/26808014492
- Ubuntu Vulkan: success

## Capture marker summary

- Windows: `CI_CAPTURE_SMOKE_OK` または `CI_CAPTURE_UNAVAILABLE`
- macOS arm64: `CI_CAPTURE_SMOKE_OK` または `CI_CAPTURE_UNAVAILABLE`
- macOS Intel: `CI_CAPTURE_SMOKE_OK` または `CI_CAPTURE_UNAVAILABLE`

未認証の GitHub HTML / API では job summary 本文を取得できなかったため、
OK / unavailable の内訳は PR 作成後に GitHub UI で確認する。

## Dependency policy

### wgpu

- `wgpu = 29.0.3`
- Old `hal` feature removed.
- Backend features are platform-specific.

### windows

- Windows wgpu path uses `windows = 0.62`.
- `winapi` is not used in the wgpu DX12 interop path.

### objc2 / objc2-metal

- Existing `objc2 = 0.5` public/platform wrapper is preserved.
- `objc2_06 = { package = "objc2", version = "0.6.3" }` is used for wgpu Metal interop.
- `objc2-metal` is used for new Metal texture/device interop.
- Full migration to objc2 0.6 is deferred.

### metal

- Existing public `metal` feature/API is preserved.
- wgpu interop path no longer depends on `metal::Texture`.
- Removing or deprecating `metal` public API is a future breaking-change decision.

## GitHub Actions Node.js runtime warning

- Warning observed: Node.js 20 deprecation warning is possible with older action majors.
- Action affected: `actions/checkout@v4`, `actions/upload-artifact@v4`.
- Mitigation:
  - `actions/checkout@v6`
  - `actions/upload-artifact@v7`
- Remaining risk: new action majors require current GitHub-hosted runner support; branch Actions must be checked after this update.

## Formatting policy

- `cargo fmt --all --check` is not a required gate in this phase.
- Reason: existing `src/util.rs` differs from rustfmt output.
- Checked:
  - `rustfmt --edition 2021 --check examples/wgpu_backend_smoke.rs`
  - `rustfmt --edition 2021 --check examples/wgpu_capture_smoke.rs`
- Full formatting cleanup is deferred.

## Warnings

- fixed:
  - removed unused Windows imports from `src/feature/wgpu/mod.rs`
- deferred:
  - `src/platform/macos/objc_wrap.rs` missing ABI warnings
  - `src/platform/macos/objc_wrap.rs` unnecessary transmute warnings
  - Windows platform unused `HWND`
  - public/platform dead-code warnings

## Required checks candidate

- wgpu runtime / Windows DX12 wgpu runtime
- wgpu runtime / macOS Metal wgpu runtime (macos-15, macos-arm64)
- wgpu runtime / macOS Metal wgpu runtime (macos-15-intel, macos-intel)
- wgpu linux vulkan smoke / Ubuntu Lavapipe Vulkan smoke

Manual strict capture is not required. It depends on runner capture capability and is intended for explicit runtime investigation.

## Deferred work

- full objc2 0.6 migration
- public metal API deprecation
- full cargo fmt cleanup
- strict capture on dedicated runner if needed

## 判断を求めたい点

1. PR を GitHub UI で作成し、PR Actions の job summary で capture marker の OK / unavailable 内訳を確認するか。
2. `actions/upload-artifact@v7` をそのまま採用するか、runner issue が出た場合に `@v6` へ下げるか。
3. 第7フェーズを review対応として進めるか。
