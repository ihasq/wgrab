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
    pub fn timestamp(&self) -> WgrabTimestamp;
    pub fn frames(&self) -> usize;
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
