# wgrab 第15Iフェーズログ

## 結論

- targeted internalization: 実装
- public shims: deprecated public shim として維持
- dxgi targeted deprecation: `WindowsDxgiVideoFrame::get_dxgi_surface`
- diagnostic policy: public 維持
- GPU-only independence: deprecated public shim 直接呼び出しなし
- negative grep: 既知 false positive のみ
- target checks: 成功
- push CI: 成功
- ready for removal planning: yes

## Branch

- branch: wgrab-gpu-only-api-design
- implementation commit: aad1b30f1e1502b3564ac7f7194736208b766487
- status: clean after commit

## Changed APIs

### Newly internalized

- `macos_frame_iosurface`
- `macos_metal_texture_for_video_frame`
- `windows_dx11_surface_for_video_frame`
- `windows_dx11_texture_for_video_frame`
- `windows_dxgi_surface_for_video_frame`

### Deprecated public shims retained

- bitmap APIs
- iosurface raw output APIs
- metal raw output APIs
- dx11 raw output APIs
- dxgi `WindowsDxgiVideoFrame::get_dxgi_surface`

### DXGI

- `WindowsDxgiVideoFrame::get_dxgi_surface`: targeted deprecated
- `WindowsDxgiCaptureStream::get_dxgi_adapter`: retained, not deprecated
- `WindowsDxgiCaptureStream::get_dxgi_device`: retained, not deprecated

### Diagnostic

- diagnostic feature remains public
- no diagnostic API is deprecated in Phase 15I

## Compatibility risk

- `metal`, `iosurface`, `dx11`, and `dxgi` feature public exports remain visible
  as compatibility shims.
- Actual removal requires a breaking-change phase and migration window.

## GPU-only independence

`logs/phase15i_wgpu_legacy_dependency_grep.txt` is empty.

The wgpu path now calls internal helpers instead of deprecated public shims:

- `macos_frame_iosurface`
- `windows_dx11_texture_for_video_frame`

## Negative grep

`logs/phase15i_gpu_only_negative_grep.txt` contains the known
`GetIoSurfaceError::NoImageBuffer` false positive only.

## Checks

- Windows target check: 成功
- macOS x86 target check: 成功
- macOS arm target check: 成功
- texture-only Windows target check: 成功
- texture-only macOS x86 target check: 成功
- texture-only macOS arm target check: 成功
- dxgi Windows target check: 成功
- dx11 Windows target check: 成功
- metal/iosurface macOS target check: 成功
- `logs/phase15i_check_error_index.txt`: 空
- `logs/phase15i_raw_feature_check_error_index.txt`: 空

`logs/phase15i_texture_only_warning_error_index.txt` contains existing macOS
`extern` ABI deprecation warnings from platform wrappers. The filtered
GPU/raw-output deprecation check,
`logs/phase15i_texture_only_gpu_deprecation_warning_index.txt`, is empty.

## Push CI

- wgpu runtime: 成功
  - https://github.com/ihasq/wgrab/actions/runs/26817702570
- wgpu linux vulkan smoke: 成功
  - https://github.com/ihasq/wgrab/actions/runs/26817703071

## Deferred

- actual removal of shims
- package/version strategy
- crates.io strategy
- Linux native wgpu feature split
