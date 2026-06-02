# CrabGrab wgpu29 第3フェーズログ

## 結論

- Windows DX12 型統一: 成功
- `as_mut_ptr` / `as_ptr` 除去: 成功
- `ComPtr<ID3D12Resource>` 除去: 成功
- `transmute_copy` 除去: 成功
- Windows cargo check: 成功
- macOS cargo check: 未対応エラーあり

## 作業ブランチ

- branch: wgpu29-phase3-windows-dx12
- base commit: ae4508fe fix: update wgpu as_hal usage for phase2
- final commit: このログを含む第3フェーズコミット。具体的な hash は最終報告に記載。
- status: commit 前確認時点で変更あり

## 変更した依存

- windows: `0.52` から `0.62` に更新。
- d3d12: 未変更 (`0.20`)。Windows wgpu path からは未使用。
- winapi: dependency 自体は残したが、`wgpu` feature から `dep:winapi` を除去。Windows wgpu path からは未使用。

## 変更したコード

- raw_device / raw_queue: `as_mut_ptr` / `as_ptr` / `from_raw` / `from_raw_borrowed` を廃止し、wgpu-hal の `windows` COM 参照をそのまま使用。
- ID3D12Resource: `d3d12::ComPtr<winapi::um::d3d12::ID3D12Resource>` を廃止し、`windows::Win32::Graphics::Direct3D12::ID3D12Resource` を使用。
- texture_from_raw: `d3d12_texture.clone()` を渡す形に変更。
- create_texture_from_hal: DX12 resource wrapping の safety comment を追加。
- 削除した unsafe hack: `std::mem::transmute_copy` による COM refcount 調整を削除。
- windows 0.62 対応: `ComInterface` を `Interface` に置換し、Win32 API の `Option<HWND/HANDLE/HDC>` などの signature 差分を最小修正。

## 残っている Windows エラー

```text
logs/phase3_remaining_windows_dx12_errors.txt は空。
Windows cargo check は warning のみで成功。
```

## 残っている macOS エラー

```text
error[E0599]: no method named `lock` found for reference
`&Retained<ProtocolObject<dyn MTLDevice>>`

error[E0308]: arguments to this function are incorrect
expected `Retained<ProtocolObject<dyn MTLTexture>>`, found `Texture`
expected `MTLTextureType`, found a different `MTLTextureType`
```

## 自分で判断したこと

- `windows` 0.62 へ上げると wgpu path 以外の Windows modules も compile 対象で壊れるため、Windows 側に限定して最小修正した。
- `d3d12` dependency は削除せず、wgpu path から `d3d12::ComPtr` 使用だけを除去した。
- `winapi` dependency は削除せず、`wgpu` feature からの pull-in と wgpu path の参照を除去した。
- Windows 実機 runtime 検証は未実施。確認できたのは cross target の `cargo check` のみ。

## 判断を求めたい点

1. 第4フェーズで macOS を `objc2-metal` 型へ寄せるか。
2. 第4フェーズとは別に、Windows runtime 実機検証をどの環境で行うか。
3. 未使用になった `d3d12` / `winapi` dependency を後続フェーズで削除対象にするか。
