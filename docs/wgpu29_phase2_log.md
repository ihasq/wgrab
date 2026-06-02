# CrabGrab wgpu29 第2フェーズログ

## 1. 作業ブランチ

- Branch: wgpu29-phase2-as-hal
- Base commit: a50d6603 docs: record phase1 final git state
- Final commit: このログを含む第2フェーズコミット。具体的な hash は最終報告に記載。

## 2. 変更内容

- `WgpuVideoFrameError::WrongWgpuBackend` を追加。
- Windows 用 `get_dx12_hal_device` helper を追加。
- macOS 用 `get_metal_hal_device` helper を追加。
- 旧 callback 形式の `as_hal::<Api, _, _>(|device| { ... })` を削除。
- `as_hal::<Api>() -> Option<guard>` 形式へ置換。

## 3. 実行コマンド結果

| コマンド | 結果 | ログ |
|---|---:|---|
| as_hal call site scan before | OK | logs/phase2_as_hal_call_sites.txt |
| as_hal call site scan after | OK | logs/phase2_as_hal_call_sites_after_patch.txt |
| windows check | NG | logs/phase2_check_windows.txt |
| macOS x86 check | NG | logs/phase2_check_macos_x86.txt |
| macOS arm check | NG | logs/phase2_check_macos_arm.txt |
| remaining as_hal errors | OK | logs/phase2_remaining_as_hal_errors.txt |

## 4. as_hal 残エラー

- なし。
- `logs/phase2_remaining_as_hal_errors.txt` は空。
- `method takes 1 generic argument` / `this method takes 0 arguments` は残っていない。

## 5. 残った主要エラー

### Windows raw 型

- `raw_device()` / `raw_queue()` が `windows` crate の `ID3D12Device` / `ID3D12CommandQueue` 参照を返すため、旧 `as_ptr` / `as_mut_ptr` 呼び出しが存在しない。
- `texture_from_raw` は `windows::Win32::Graphics::Direct3D12::ID3D12Resource` を期待するが、現行コードは `d3d12::ComPtr<winapi::um::d3d12::ID3D12Resource>` を渡している。

### macOS raw 型

- `raw_device()` が `Retained<ProtocolObject<dyn MTLDevice>>` 参照を返すため、旧 `lock()` 呼び出しが存在しない。
- `texture_from_raw` は `objc2-metal` 系 `MTLTexture` / `MTLTextureType` を期待するが、現行コードは `metal::Texture` / `metal::MTLTextureType` を渡している。

## 6. 自分で判断したこと

- backend mismatch は `unwrap()` / `expect()` ではなく `WrongWgpuBackend` として返すようにした。
- `with_wgpu_device` は既存 API が `Result<Self, String>` なので、helper の `WgpuVideoFrameError` を `to_string()` して返す形にした。
- Windows / macOS raw 型エラーは第3フェーズ対象として残した。
- `windows`, `d3d12`, `winapi`, `metal`, `objc2` は更新しなかった。

## 7. 判断できず保留したこと

1. Windows 側で `windows` crate COM 型に寄せるか、既存 `winapi` / `d3d12::ComPtr` との変換を用意するか。
2. macOS 側で `objc2-metal` 型に寄せるか、既存 `metal` crate 型から変換するか。
3. `with_wgpu_device` の public API を第3フェーズ以降で error enum 化するか。
