# CrabGrab wgpu29 第5Cフェーズログ

## 結論

- Windows DX12 backend smoke: 成功
- macOS Metal backend smoke arm64: 成功または unavailable として job 成功
- macOS Metal backend smoke Intel: 成功または unavailable として job 成功
- Windows capture probe: job 成功。marker 詳細は artifact 参照
- macOS capture probe arm64: job 成功。marker 詳細は artifact 参照
- macOS capture probe Intel: job 成功。marker 詳細は artifact 参照
- Ubuntu Vulkan smoke: 成功

## GitHub Actions run

- workflow: `.github/workflows/wgpu-runtime.yml`, `.github/workflows/wgpu-linux-vulkan.yml`
- run URL:
  - https://github.com/ihasq/wgrab/actions/runs/26802529065
  - https://github.com/ihasq/wgrab/actions/runs/26802528967
- branch: wgpu29-phase5c-github-actions-runtime
- commit: 11694a524deb8b1fccf8b99d6261ef9bda1ebdbb

## runner capability summary

### Windows

```text
GitHub Actions job:
- Windows DX12 wgpu runtime: success
- Check crate: success
- Check examples: success
- Run DX12 backend smoke: success
- Run Windows capture probe: success

ローカル cross check:
- cargo check --target x86_64-pc-windows-msvc --features wgpu: 成功
- cargo check --target x86_64-pc-windows-msvc --example wgpu_backend_smoke --features wgpu: 成功
- cargo check --target x86_64-pc-windows-msvc --example wgpu_capture_smoke --features wgpu,wgpu-debug-descriptor: 成功
```

### macOS arm64

```text
GitHub Actions job:
- macOS Metal wgpu runtime (macos-15, macos-arm64): success
- Check crate: success
- Check examples: success
- Run Metal backend smoke: success
- Run macOS capture probe: success

ローカル cross check:
- cargo check --target aarch64-apple-darwin --features wgpu: 成功
- cargo check --target aarch64-apple-darwin --example wgpu_backend_smoke --features wgpu: 成功
- cargo check --target aarch64-apple-darwin --example wgpu_capture_smoke --features wgpu,wgpu-debug-descriptor: 成功
```

### macOS Intel

```text
GitHub Actions job:
- macOS Metal wgpu runtime (macos-15-intel, macos-intel): success
- Check crate: success
- Check examples: success
- Run Metal backend smoke: success
- Run macOS capture probe: success

ローカル cross check:
- cargo check --target x86_64-apple-darwin --features wgpu: 成功
- cargo check --target x86_64-apple-darwin --example wgpu_backend_smoke --features wgpu: 成功
- cargo check --target x86_64-apple-darwin --example wgpu_capture_smoke --features wgpu,wgpu-debug-descriptor: 成功
```

### Ubuntu Vulkan

```text
CrabGrab 本体は Linux platform module を持たないため、Ubuntu workflow は
examples/wgpu_backend_smoke.rs を一時 standalone crate にコピーして実行する。

GitHub Actions job:
- Ubuntu Lavapipe Vulkan smoke: success
- Install Vulkan tools: success
- Show Vulkan info: success
- Check backend smoke example: success
- Run Vulkan backend smoke: success

VPS standalone check:
- cargo check --manifest-path /tmp/wgpu-backend-smoke-standalone/Cargo.toml: 成功

VPS runtime:
- CI_BACKEND_SMOKE_UNAVAILABLE
- reason: Vulkan runtime / adapter unavailable on this VPS
```

## artifact

* windows-dx12-wgpu-logs: 7350511469
* macos-metal-macos-arm64-wgpu-logs: 7350473656
* macos-metal-macos-intel-wgpu-logs: 7350519913
* ubuntu-vulkan-wgpu-logs: 7350471569

未認証 GitHub API では artifact zip download が 403 になったため、
`CI_CAPTURE_SMOKE_OK` / `CI_CAPTURE_UNAVAILABLE` の marker 本文は
GitHub Actions UI または認証済み artifact download で確認する。

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

## 第5C-R marker hardening

### 目的

artifact を手動取得しなくても、Actions の通常ログと job summary だけで
backend smoke / capture probe を判定できるようにした。

### 判定規則

- backend smoke は `CI_BACKEND_SMOKE_OK` 必須
- backend smoke の `CI_BACKEND_SMOKE_UNAVAILABLE` は失敗扱い
- capture probe は `CI_CAPTURE_SMOKE_OK` または `CI_CAPTURE_UNAVAILABLE` を許容
- capture probe の `CI_CAPTURE_SMOKE_FAILED` は失敗
- marker 欠落は失敗
- crash / panic / validation error は失敗

### 実施した hardening

- `continue-on-error: true` を capture probe から削除
- capture probe step 内で exit status と marker を分類
- backend smoke / capture probe の marker を `$GITHUB_STEP_SUMMARY` に出力
- CI mode の Rust panic を `CI_CAPTURE_SMOKE_FAILED` marker に変換
- segfault など marker を出せない crash は marker missing として workflow 失敗

### CI結果

- run URL:
  - https://github.com/ihasq/wgrab/actions/runs/26806516230
  - https://github.com/ihasq/wgrab/actions/runs/26806516788
- Windows backend marker: `CI_BACKEND_SMOKE_OK`
- Windows capture marker: `CI_CAPTURE_SMOKE_OK` または `CI_CAPTURE_UNAVAILABLE`
- macOS arm64 backend marker: `CI_BACKEND_SMOKE_OK`
- macOS arm64 capture marker: `CI_CAPTURE_SMOKE_OK` または `CI_CAPTURE_UNAVAILABLE`
- macOS Intel backend marker: `CI_BACKEND_SMOKE_OK`
- macOS Intel capture marker: `CI_CAPTURE_SMOKE_OK` または `CI_CAPTURE_UNAVAILABLE`
- Ubuntu Vulkan marker: `CI_BACKEND_SMOKE_OK`

上記 run はいずれも completed successfully。capture marker は workflow の
分類 script 上、`CI_CAPTURE_SMOKE_OK` または `CI_CAPTURE_UNAVAILABLE`
がログに存在した場合のみ job success になる。marker 欠落、panic marker、
`CI_CAPTURE_SMOKE_FAILED`、segfault などの marker 欠落 crash は失敗扱い。

未認証の GitHub HTML / API では job summary と artifact 本文を取得できず、
capture marker が OK / unavailable のどちらだったかまでは機械確認できなかった。
