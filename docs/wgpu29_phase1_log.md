# CrabGrab wgpu29 第1フェーズログ

## 1. 作業ブランチ

- Branch: wgpu29-phase1
- Base commit: a535cfbbadb9ea5ba36aef9fa61a5da15021815b
- Final commit: 未作成

## 2. 開発環境

- OS: Debian GNU/Linux 12 (bookworm), Linux 6.1.0-48-amd64
- rustc: rustc 1.95.0 (59807616e 2026-04-14)
- cargo: cargo 1.95.0 (f2d3ce0bd 2026-03-21)
- Vulkan ICD: 未検出
- CPU Vulkan smoke test:
  - 失敗
  - adapter 名: 未取得

## 3. 変更内容

### Cargo.toml

- wgpu macOS: 未変更
- wgpu Windows: 未変更
- wgpu dev-dependencies: 未変更
- rust-version: 未変更

### 変更しなかった依存

- windows: 未変更
- d3d12: 未変更
- winapi: 未変更
- metal: 未変更
- objc2: 未変更

## 4. 実行コマンド結果

| コマンド | 結果 | ログ |
|---|---:|---|
| env baseline | OK | logs/env_baseline.txt |
| rust target add | OK | logs/rust_target_add.txt |
| Vulkan ICD discovery | NG | logs/vulkan_icd_files.txt |
| Lavapipe ICD discovery | NG | logs/vulkan_lavapipe_icd_files.txt |
| vulkaninfo --summary | NG | logs/vulkaninfo_summary.txt |
| wgpu29_vulkan_smoke | 未実施 | - |
| cargo metadata after | 未実施 | - |
| windows check | 未実施 | - |
| macOS x86 check | 未実施 | - |
| macOS arm check | 未実施 | - |

## 5. 主要エラー分類

### A. 依存解決エラー

- なし
- 内容: wgpu 29 依存更新前に停止したため未評価。

### B. wgpu API signature 差分

- なし
- 内容: wgpu 29 依存更新前に停止したため未評価。

### C. Windows raw handle / COM 型差分

- なし
- 内容: wgpu 29 依存更新前に停止したため未評価。

### D. macOS Metal / Objective-C 型差分

- なし
- 内容: wgpu 29 依存更新前に停止したため未評価。

### E. その他

- あり
- 内容:
  - `sudo apt-get update` は password/TTY 要求で実行不可。
  - `/usr/share/vulkan/icd.d` が存在しない。
  - `vulkaninfo` が未インストール。
  - Lavapipe ICD が未検出。
  - `rustup --version` はサンドボックス内で SIGCHLD handler の panic を起こしたが、`rustc` と `cargo` は実行可能。

## 6. 自分で判断したこと

- 判断1: マニュアル第8章の停止条件に従い、CrabGrab の依存更新と cross target check には進まなかった。
- 理由: Lavapipe ICD が検出できず、`vulkaninfo --summary` も実行できない状態であり、「ここで停止して報告する」に該当するため。
- 結果: Cargo.toml/Cargo.lock/src は未変更。環境復旧用ログのみ保存。

## 7. 判断できず保留したこと

- 保留1: VPS に `vulkan-tools` と `mesa-vulkan-drivers` をインストールする方法。
- 理由: `sudo` が password/TTY を要求し、こちらから apt install を完了できないため。
- 必要な判断: ユーザー側で package install を行うか、パスワードなし sudo/管理者権限を用意してから再実行する。

## 8. 第2フェーズで確認してほしいこと

1. まず VPS に `vulkan-tools` と `mesa-vulkan-drivers` を導入し、Lavapipe ICD が出る状態にする。
2. `vulkaninfo --summary` が Lavapipe/Mesa device を表示することを確認する。
3. その後に wgpu 29 smoke test、依存更新、Windows/macOS check へ進む。
