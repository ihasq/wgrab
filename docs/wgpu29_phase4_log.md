# CrabGrab wgpu29 第4フェーズログ

## 結論

- macOS objc2-metal 移行: 成功
- raw_device().lock() 除去: 成功
- metal::Texture 除去: 成功
- MTLTextureType 型混在解消: 成功
- macOS x86 cargo check: 成功
- macOS arm cargo check: 成功
- Windows regression check: 成功

## 作業ブランチ

- branch: wgpu29-phase4-macos-objc2-metal
- base commit: 5e2c414b0f537f805def30287c76467ac5c1960b
- final commit: このログを含む最終 commit。正確な hash は提出報告と git log を参照。
- status: commit 後 clean

## 変更した依存

- objc2: 0.5 を維持
- objc2_06: package = "objc2", version = "0.6.3" を追加
- objc2-foundation: 0.3.2 を追加
- objc2-io-surface: 0.3.2 を追加
- objc2-metal: 0.3.2 を追加し、objc2-io-surface feature を有効化
- metal: 0.28 を維持
- block2: 変更なし

## 変更したコード

- MacosCaptureConfig: wgpu device backend の検証だけを行い、旧 raw_device().lock().clone() は削除
- Metal device type: wgpu interop path では Objc2MetalDevice を導入
- Metal texture type: IOSurface から Objc2MetalTexture を生成し、metal::Texture 依存を除去
- pixel format conversion: objc2_metal::MTLPixelFormat 入力へ変更。RG8Uint は wgpu::TextureFormat::Rg8Uint へ対応
- texture_from_raw: objc2-metal の Retained<ProtocolObject<dyn MTLTexture>> を渡す形へ変更
- create_texture_from_hal: objc2-metal texture の getter から TextureDescriptor を構築
- 削除した unsafe / raw pointer 操作: raw_device().lock() は削除。Retained::from_raw / transmute / into_raw は追加していない

## 採用した objc2-metal method 名

- texture type: textureType()
- pixel format: pixelFormat()
- width: width()
- height: height()
- mip count: mipmapLevelCount()
- sample count: sampleCount()
- array length: arrayLength()
- storage mode: storageMode()
- usage: usage()
- IOSurface texture creation: newTextureWithDescriptor_iosurface_plane()

## 残っている macOS エラー

```text
logs/phase4_remaining_macos_metal_errors.txt は空。
```

## Windows regression

```text
warning: `crabgrab` (lib) generated 11 warnings (run `cargo fix --lib -p crabgrab` to apply 8 suggestions)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
```

## 自分で判断したこと

- 判断: 既存の objc2 = 0.5 は維持し、objc2-metal 用に objc2_06 renamed dependency を追加した。
- 理由: objc2 を直接 0.6 系へ更新すると platform の Objective-C wrapper まで広範囲に signature 差分が出て、第4フェーズの停止条件に近づくため。
- 結果: wgpu macOS interop path は objc2-metal へ移行しつつ、既存 public metal feature と macOS capture core の大規模移行は避けた。

- 判断: metal crate は削除しなかった。
- 理由: crate の public metal feature と platform 側既存 API がまだ metal::Device / metal::Texture を公開しているため。
- 結果: src/feature/wgpu/mod.rs の wgpu path から metal crate 型参照は除去し、既存 API 互換を維持した。

- 判断: IoSurface::get_raw() から objc2_io_surface::IOSurfaceRef への借用 cast を使った。
- 理由: IoSurface は IOSurfaceUseCountIncrement/Decrement で raw IOSurface の利用期間を保持しており、objc2-metal の texture creation API には borrowed IOSurfaceRef が必要だったため。
- 結果: 所有権移譲は行わず、Retained::from_raw も使用していない。

## 判断を求めたい点

1. 次フェーズで objc2 = 0.5 legacy wrapper を 0.6 系へ統一するか。
2. metal feature の public API を維持するか、objc2-metal へ移行して breaking change とするか。
3. macOS 実機 runtime 検証で IOSurface plane と Metal texture descriptor の一致を追加確認するか。
