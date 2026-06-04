# wgrab audio API sketch

## Primary types

```rust
pub struct WgrabAudioContext {
    // backend-specific context
}

pub struct WgrabAudioStream {
    // capture stream
}

pub struct WgrabAudioFrame {
    // timestamped samples
}
```

## Audio frame metadata

```rust
pub struct WgrabAudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: WgrabSampleFormat,
}

pub enum WgrabSampleFormat {
    F32,
    I16,
    U16,
}
```

## Audio frame

```rust
pub struct WgrabAudioFrame {
    // internal sample storage
}

impl WgrabAudioFrame {
    pub fn format(&self) -> WgrabAudioFormat;
    pub fn timestamp(&self) -> Option<WgrabTimestamp>;
    pub fn frames(&self) -> usize;
    pub fn samples_f32(&self) -> &[f32];
}
```

## Sample data policy

Unlike video, audio samples are normally processed by the CPU.

Therefore, the audio API cannot copy the exact GPU-only video rule.

Instead, the audio rule is:

- avoid platform raw output as the primary API
- provide normalized audio frames
- expose timing metadata
- keep backend-specific streams internal

## Synchronization with video

Future API should allow pairing:

- `WgpuCaptureFrame`
- `WgrabAudioFrame`

Possible shared field:

```rust
pub struct WgrabTimestamp {
    // monotonic capture timestamp
}
```

## Open design constraints

- Audio callback timing and video frame timing may use different clocks.
- System audio capture and microphone capture may have different permission and
  timestamp behavior.
- Web Audio and native CPAL capture may need different frame types until a common
  timestamp and sample format model is stable.

## Phase 28A prototype

`WgrabAudioFrame` stores interleaved `f32` samples.

This is a prototype format. Future phases may add:

- planar audio
- explicit channel layout
- resampling policy
- lock-free ring buffer
- A/V synchronization timestamps

## Phase 29S timestamp model

`WgrabAudioFrame` carries `Option<WgrabTimestamp>`.

The first prototype uses a monotonic `WgrabCaptureClock`.

Precise backend timestamps are deferred.

## Video pairing

`WgrabAudioFrame::timestamp()` is intended to be comparable with
`WgpuCaptureFrame::timestamp()`.

## Timestamp quality

`WgrabAudioFrame` exposes timestamp quality so users can distinguish backend
timestamps from dequeue-time prototype timestamps.

## Audio/video pairing

`WgrabAudioFrame` can be paired with `WgpuCaptureFrame` through timestamp
helpers when both frames provide `WgrabTimestamp`.

## Queue pairing

Audio frames can be inserted into a future sync queue by timestamp and quality.
