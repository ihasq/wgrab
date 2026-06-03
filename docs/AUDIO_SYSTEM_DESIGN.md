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
