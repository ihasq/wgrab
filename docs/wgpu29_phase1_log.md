# CrabGrab wgpu29 第1フェーズログ

## 1. 作業ブランチ

- Branch: wgpu29-phase1
- Base commit: a535cfbbadb9ea5ba36aef9fa61a5da15021815b
- Final commit: 最終提出時の branch tip は最終報告に記載。依存更新診断コミットは 0122f242。

## 2. 開発環境

- OS: Debian GNU/Linux 12 (bookworm), Linux 6.1.0-48-amd64
- rustc: rustc 1.95.0 (59807616e 2026-04-14)
- cargo: cargo 1.95.0 (f2d3ce0bd 2026-03-21)
- Vulkan ICD: 未検出
- CPU Vulkan smoke test:
  - 未実施
  - adapter 名: 未取得

## 3. 変更内容

### Cargo.toml

- wgpu macOS: `^0.20` + `["metal", "hal"]` から `29.0.3`, `default-features = false`, `["std", "wgsl", "metal"]` に変更。
- wgpu Windows: `0.20` + `["dx12", "hal"]` から `29.0.3`, `default-features = false`, `["std", "wgsl", "dx12"]` に変更。
- wgpu dev-dependencies: `0.20` から `29.0.3`, `default-features = false`, `["std", "wgsl", "vulkan"]` に変更。
- rust-version: `1.87` を追加。

### 変更しなかった依存

- windows: 未変更 (`0.52`)
- d3d12: 未変更 (`0.20`)
- winapi: 未変更 (`0.3`)
- metal: 未変更 (`0.28`)
- objc2: 未変更 (`0.5`)

### Cargo.lock

- `cargo metadata` により `Cargo.lock` はローカル生成された。
- この crate は `.gitignore` で `Cargo.lock` を除外しており、元から未追跡運用のため commit しない。

## 4. 実行コマンド結果

| コマンド | 結果 | ログ |
|---|---:|---|
| env baseline | OK | logs/env_baseline.txt |
| rust target add | OK | logs/rust_target_add.txt |
| sudo noninteractive check | NG | logs/sudo_noninteractive_check.txt |
| Vulkan ICD discovery | NG | logs/vulkan_icd_files.txt |
| Lavapipe ICD discovery | NG | logs/vulkan_lavapipe_icd_files.txt |
| vulkaninfo --summary | NG | logs/vulkaninfo_summary.txt |
| wgpu29_vulkan_smoke | 未実施 | - |
| cargo fmt --all --check | NG | logs/fmt_after_cargo_patch.txt |
| cargo metadata after | OK | logs/metadata_after_wgpu29.err |
| cargo tree after | OK | logs/tree_after_wgpu29.txt |
| remaining hal references | OK | logs/remaining_hal_references.txt |
| windows check | NG | logs/check_windows_after_wgpu29.txt |
| macOS x86 check | NG | logs/check_macos_x86_after_wgpu29.txt |
| macOS arm check | NG | logs/check_macos_arm_after_wgpu29.txt |

## 5. 主要エラー分類

### A. 依存解決エラー

- なし
- 内容: `cargo metadata` と `cargo tree --features wgpu` は `wgpu 29.0.3` で成功。`hal` feature 残存なし。

### B. wgpu API signature 差分

- あり
- 内容:
  - Windows: `Device::as_hal::<Dx12, _, _>(closure)` が `E0107` / `E0061`。wgpu 29 の `as_hal` は generic 1 個、引数なしで `Option<guard>` を返す。
  - macOS: `Device::as_hal::<Metal, _, _>(closure)` が `E0107` / `E0061`。同じく callback 形式が使えない。
  - Windows: `get_wgpu_texture` 内で `as_hal(...)` の戻り型が期待する `Result<Texture, WgpuVideoFrameError>` と合わず `E0308`。
  - macOS: `metal_device` へ `Option<metal::Device>` ではなく `impl Deref<Target = hal::metal::Device>` が渡って `E0308`。

### C. Windows raw handle / COM 型差分

- あり
- 内容:
  - `wgpu::hal::dx12::Device::texture_from_raw` が `windows::Win32::Graphics::Direct3D12::ID3D12Resource` を期待。
  - 現行コードは `d3d12::ComPtr<winapi::um::d3d12::ID3D12Resource>` を渡しており `E0308`。
  - `raw_device().as_ptr()` / `raw_queue().as_ptr()` の扱いは `as_hal` 形式変更後に再評価が必要。

### D. macOS Metal / Objective-C 型差分

- あり
- 内容:
  - `wgpu::hal::metal::Device::texture_from_raw` が `objc2::rc::Retained<ProtocolObject<dyn objc2_metal::MTLTexture>>` を期待。
  - 現行コードは `metal::Texture` を渡しており `E0308`。
  - `metal::MTLTextureType` と `objc2_metal::MTLTextureType` が別型として衝突。
  - `raw_device().lock().clone()` 前提は wgpu 29 の `as_hal` guard 型に合わせた再設計が必要。

### E. その他

- あり
- 内容:
  - Vulkan runtime は未復旧。`/usr/share/vulkan/icd.d` と `vulkaninfo` が存在しない。
  - `sudo -n true` は `no new privileges` と `/etc/sudo.conf` owner 問題で失敗。
  - `cargo fmt --all --check` は既存 Rust ファイルの formatting diff を大量に出して失敗。今回の第1フェーズでは整形変更を行わない。
  - 初回の依存取得と macOS check はサンドボックス内 DNS 制約で失敗したため、ネットワーク許可付きで再実行した。

## 6. 自分で判断したこと

- 判断1: Vulkan runtime 復旧は 1A として保留し、1B の依存更新と compile 診断を先行した。
- 理由: 改訂指示により、Vulkan runtime は Windows/macOS API 差分診断には必須ではないため。
- 結果: `wgpu 29.0.3` 依存解決と target 別 check ログを取得できた。

- 判断2: `Cargo.lock` は commit しない。
- 理由: この crate は `.gitignore` で `Cargo.lock` を除外する library crate 運用であり、元から lock file は追跡されていなかったため。
- 結果: `Cargo.toml`、docs、logs のみを commit 対象にする。

## 7. 判断できず保留したこと

- 保留1: Vulkan / Lavapipe runtime 復旧。
- 理由: apt install には管理者作業が必要であり、現在の sudo は非対話・対話とも実行不能。
- 必要な判断: 管理者が `vulkan-tools` と `mesa-vulkan-drivers` を導入後、smoke test を再開する。

- 保留2: wgpu 29 API 差分の本格修正。
- 理由: 第1フェーズでは診断収集が目的であり、unsafe / COM / Objective-C retain-release 周りを推測で変更しないため。
- 必要な判断: 第2フェーズで `as_hal` guard、Windows COM 型、Metal objc2 型への移行方針を決める。

## 8. 第2フェーズで確認してほしいこと

1. `as_hal` の新戻り値 `Option<impl Deref<Target = A::Device>>` を使う設計に変える範囲。
2. Windows 側で `winapi`/`d3d12::ComPtr` を維持するか、`windows` crate COM 型へ寄せるか。
3. macOS 側で `metal` crate 型を維持するか、wgpu 29 hal が要求する `objc2-metal` 型へ寄せるか。
4. 管理者による Vulkan runtime 復旧後、`wgpu29_vulkan_smoke` を再実施する。
