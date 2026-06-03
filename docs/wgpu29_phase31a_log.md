# wgrab 第31Aフェーズログ

## 結論

- CPAL device report: added host device reports with input/output/default flags
- loopback heuristic: added provisional name/config based candidate detection
- loopback probe example: `examples/audio_cpal_loopback_probe.rs`
- audio CI: loopback probe check and marker gate added
- target checks: Windows and macOS target checks completed with warnings only;
  Linux host checks stopped in `alsa-sys` because `pkg-config` is unavailable
  on this machine
- ready for backend decision: pending CI

## Branch

- branch: wgrab-cpal-loopback-capability
- commit: phase branch commit
- status: implementation phase complete

## Probe results

- Windows: pending CI
- macOS: pending CI
- Ubuntu: pending CI

## Decision

- CPAL sufficient: pending probe data
- direct WASAPI needed: pending Windows probe data
- ScreenCaptureKit audio needed: still likely for screen-coupled macOS capture

## Deferred

- direct WASAPI loopback implementation
- ScreenCaptureKit audio implementation
- stable device identifiers
- precise timestamps
