# wgrab 第33Aフェーズログ

## 結論

- device report hardening: added config counts and default input/output formats
- candidate scoring: added scored CPAL loopback candidates with selection reasons
- system audio stats: system audio smoke reports max_abs, mean_abs, and rms
- silence detection: added non-failing silence detection with optional strict mode
- audio CI: existing marker gate remains normal mode with unavailable allowed
- package/dry-run: completed with existing warnings only
- ready for platform backend decision: pending pushed CI

## Branch

- branch: wgrab-cpal-system-audio-hardening
- commit: pending
- status: local checks complete

## Results

- Windows: target check completed with warnings only
- macOS: x86_64 and aarch64 target checks completed with warnings only
- Ubuntu: local host audio checks may require ALSA `pkg-config`; CI is authoritative

## CPAL sufficiency

- stream available: CI can validate candidate stream construction or graceful unavailable markers
- non-silent verified: requires real environment with active system audio
- remaining risk: CPAL candidates may be silent or not tied to the screen capture source

## Decision request

- CPAL continue: pending real-device non-silent observations
- direct WASAPI: still candidate if Windows CPAL loopback is unavailable or insufficient
- ScreenCaptureKit audio: still candidate for macOS screen-coupled audio
