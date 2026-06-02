# CrabGrab wgpu29 第5フェーズログ

## 結論

- macOS native check: 未実施
- macOS smoke build: 未実施。VPS cross `cargo check --example` は成功。VPS cross `cargo build --example` は linker 環境制約で失敗
- macOS smoke run 1 frame: 未実施
- macOS 30 frame loop: 未実施
- Windows regression check: 成功
- Linux CPU Vulkan smoke: 環境復旧待ち

## 作業ブランチ

- branch: wgpu29-phase5-runtime-validation
- base commit: 2951128c993df6f46faa264d079dee8030e44bfe
- final commit: このログを含む最終 commit。正確な hash は提出報告と git log を参照。
- status: commit 後 clean

## 実行環境

### macOS

- machine: 未実施
- arch: 未実施
- macOS version: 未実施
- rustc: 未実施
- Screen Recording permission: 未確認

### Linux VPS

- OS: Debian GNU/Linux 12 (bookworm), Linux 6.1.0-48-amd64
- rustc: rustc 1.95.0 (59807616e 2026-04-14)
- Vulkan ICD: `/usr/share/vulkan/icd.d` が存在しない
- vulkaninfo: `command not found`

## descriptor dump summary

### IOSurface

```text
実機 runtime 未実施のため実値なし。
追加した dump 項目:
- width
- height
- pixel format
- plane count
- selected plane
- plane width
- plane height
- plane bytes per row
```

### Metal texture

```text
実機 runtime 未実施のため実値なし。
追加した dump 項目:
- width
- height
- depth
- array length
- mipmap level count
- sample count
- texture type
- pixel format
- usage
- storage mode
```

### wgpu descriptor

```text
実機 runtime 未実施のため実値なし。
追加した dump 項目:
- width
- height
- depth_or_array_layers
- mip_level_count
- sample_count
- dimension
- format
- usage
- view_formats
```

## runtime 結果

```text
macOS runtime は Linux VPS では実施不可。
logs/phase5_macos_smoke_run.txt と logs/phase5_macos_30frames.txt に未実施理由と実機用コマンドを記録。
```

## validation / panic / error

```text
static check:
- logs/phase5_check_error_index.txt は空

VPS cross example check:
- logs/phase5_macos_smoke_check_x86_cross.txt: 成功
- logs/phase5_macos_smoke_check_arm_cross.txt: 成功

VPS cross example build:
- Linux の cc が macOS linker option `-framework`, `-arch`, `-mmacosx-version-min` を扱えず失敗
- logs/phase5_macos_smoke_build_cross_link_errors.txt に分離

Vulkan:
- vulkaninfo が未導入
- Lavapipe ICD 未検出
- logs/phase5_wgpu29_vulkan_smoke.txt に未実施理由を記録
```

## 自分で判断したこと

- descriptor dump は `wgpu-debug-descriptor` feature、または debug build で `CRABGRAB_WGPU_DEBUG_DESCRIPTOR=1` がある場合だけ有効化した。
- dump 有効時だけ descriptor assertion を実行するようにし、本番 path に常時 panic を残さなかった。
- macOS native build/run は VPS で代替せず、未実施として記録した。
- Linux VPS での `cargo build --target *-apple-darwin` は linker 制約で失敗したため、compile 確認は `cargo check --example` の結果を採用した。

## 判断を求めたい点

1. macOS 実機で `wgpu_capture_smoke` の 1 frame / 30 frame を実行してよいか。
2. descriptor dump の assertion を `wgpu-debug-descriptor` feature のみに限定し、env var 経路を削るか。
3. Vulkan runtime 復旧後に CPU Vulkan smoke をこの Phase5 ブランチで再実施するか。
