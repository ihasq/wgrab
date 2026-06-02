# CrabGrab wgpu29 第5Cフェーズログ

## 結論

- Windows DX12 backend smoke: 未実行
- macOS Metal backend smoke arm64: 未実行
- macOS Metal backend smoke Intel: 未実行
- Windows capture probe: 未実行
- macOS capture probe arm64: 未実行
- macOS capture probe Intel: 未実行
- Ubuntu Vulkan smoke: 未実行

## GitHub Actions run

- workflow: `.github/workflows/wgpu-runtime.yml`, `.github/workflows/wgpu-linux-vulkan.yml`
- run URL: 未実行
- branch: wgpu29-phase5c-github-actions-runtime
- commit: このログを含む最終 commit。正確な hash は提出報告と git log を参照。

## runner capability summary

### Windows

```text
CI未実行。
ローカル cross check:
- cargo check --target x86_64-pc-windows-msvc --features wgpu: 成功
- cargo check --target x86_64-pc-windows-msvc --example wgpu_backend_smoke --features wgpu: 成功
- cargo check --target x86_64-pc-windows-msvc --example wgpu_capture_smoke --features wgpu,wgpu-debug-descriptor: 成功
```

### macOS arm64

```text
CI未実行。
ローカル cross check:
- cargo check --target aarch64-apple-darwin --features wgpu: 成功
- cargo check --target aarch64-apple-darwin --example wgpu_backend_smoke --features wgpu: 成功
- cargo check --target aarch64-apple-darwin --example wgpu_capture_smoke --features wgpu,wgpu-debug-descriptor: 成功
```

### macOS Intel

```text
CI未実行。
ローカル cross check:
- cargo check --target x86_64-apple-darwin --features wgpu: 成功
- cargo check --target x86_64-apple-darwin --example wgpu_backend_smoke --features wgpu: 成功
- cargo check --target x86_64-apple-darwin --example wgpu_capture_smoke --features wgpu,wgpu-debug-descriptor: 成功
```

### Ubuntu Vulkan

```text
CI未実行。
CrabGrab 本体は Linux platform module を持たないため、Ubuntu workflow は
examples/wgpu_backend_smoke.rs を一時 standalone crate にコピーして実行する。

VPS standalone check:
- cargo check --manifest-path /tmp/wgpu-backend-smoke-standalone/Cargo.toml: 成功

VPS runtime:
- CI_BACKEND_SMOKE_UNAVAILABLE
- reason: Vulkan runtime / adapter unavailable on this VPS
```

## artifact

* windows-dx12-wgpu-logs: workflow 実行後に生成
* macos-metal-macos-arm64-wgpu-logs: workflow 実行後に生成
* macos-metal-macos-intel-wgpu-logs: workflow 実行後に生成
* ubuntu-vulkan-wgpu-logs: workflow 実行後に生成

## 判断

* 必須ゲートにできるもの:
  - Windows DX12 backend smoke
  - Windows / macOS `cargo check --features wgpu`
  - Windows / macOS `wgpu_backend_smoke` example check
  - Windows / macOS `wgpu_capture_smoke` example check
  - Ubuntu standalone Vulkan backend smoke after Lavapipe install
* capability probe に残すもの:
  - Windows capture probe
  - macOS capture probe
  - macOS Metal backend unavailable case on hosted runner
* runtime bugfix が必要なもの:
  - `CI_BACKEND_SMOKE_FAILED`
  - `as_hal::<Dx12>()` / `as_hal::<Metal>()` failure
  - texture creation / texture view creation failure
  - `CI_CAPTURE_SMOKE_FAILED`
  - descriptor assertion failure

## 判断を求めたい点

1. macOS Metal backend smoke の `CI_BACKEND_SMOKE_UNAVAILABLE` を初回は許容する現在の workflow 判定でよいか。
2. Ubuntu Vulkan は CrabGrab 本体 Linux build ではなく、standalone backend smoke で代替する方針でよいか。
3. 初回 CI 実行後、capture probe の `CI_CAPTURE_UNAVAILABLE` を docs に分類して cleanup フェーズへ進むか。
