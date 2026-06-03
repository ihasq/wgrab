# wgrab system audio design

## Public output

System audio capture output:

- `WgrabAudioFrame`
- `WgrabAudioFormat`
- `WgrabTimestamp`

## Backends

### Generic input

- CPAL default input stream
- microphone / input device capture

### Windows system audio

- candidate: WASAPI loopback
- first prototype candidate: CPAL WASAPI output-device-as-input loopback
- direct fallback: WASAPI `AUDCLNT_STREAMFLAGS_LOOPBACK`
- public output: `WgrabAudioFrame`

### macOS system audio

- candidate: ScreenCaptureKit audio
- secondary candidate: CPAL CoreAudio loopback for generic output-device capture
- public output: `WgrabAudioFrame`

### Web system/tab audio

- future: `getDisplayMedia` audio track
- public output: web audio frame type

## Unified API sketch

```rust
pub enum WgrabAudioSource {
    DefaultInput,
    SystemAudio,
}

pub struct WgrabAudioConfig {
    pub source: WgrabAudioSource,
}

impl WgrabAudioContext {
    pub fn build_stream(
        &self,
        config: WgrabAudioConfig,
    ) -> Result<WgrabAudioStream, WgrabAudioError>;
}
```

## Important distinction

`DefaultInput` and `SystemAudio` may use different backends.

The user should not need to know whether the backend is CPAL, WASAPI, or
ScreenCaptureKit.

Phase 30L does not implement this API.

## Conversion model

Each backend should normalize data into the same internal audio frame model:

```text
backend packet/buffer
  -> sample conversion
  -> interleaved f32
  -> WgrabAudioFormat
  -> WgrabTimestamp
  -> WgrabAudioFrame
```

## Timestamp model

System audio backends should prefer native packet/sample timestamps over
dequeue-time timestamps:

- CPAL: `InputCallbackInfo::timestamp().capture`
- WASAPI: capture packet timing where available
- ScreenCaptureKit: `CMSampleBuffer` timing

All timestamp sources must be converted into the wgrab capture timeline.

## A/V sync boundary

All audio backends should normalize backend timestamps to `WgrabTimestamp`.

Video backends should do the same when frame timestamps are available.

## CPAL loopback probe

The CPAL loopback probe is not a final system audio backend.

It determines whether CPAL can expose useful system-audio candidates on each
platform.

The probe reports input/output devices, marks default input/output devices, and
uses a name/config heuristic to classify possible loopback candidates.

If useful candidates appear, the next CPAL phase can try building
`WgrabAudioFrame` streams from those candidates. If not, direct WASAPI or
ScreenCaptureKit backends should be prioritized.

## SystemAudioCandidate

`SystemAudioCandidate` is a CPAL-based attempt to capture system-audio-like
input.

It is not a final platform guarantee.

The stream reports `WgrabAudioSource::SystemAudioCandidate` so examples and
future APIs can distinguish it from default microphone/input-device capture.

## System audio candidate quality

A CPAL `SystemAudioCandidate` is considered useful only if:

- a candidate device can be selected
- an input stream can be built
- frames can be produced
- non-silent samples are observed in a real environment

CI may only prove the first three.
