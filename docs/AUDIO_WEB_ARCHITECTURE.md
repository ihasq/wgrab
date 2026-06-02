# wgrab web audio architecture

## Capture pipeline

```text
navigator.mediaDevices.getDisplayMedia({ audio: ... })
  -> MediaStream
  -> audio MediaStreamTrack
  -> AudioContext
  -> MediaStreamAudioSourceNode
  -> AudioWorkletNode
  -> AudioWorkletProcessor
  -> WgrabWebAudioFrame
```

## Alternative microphone pipeline

```text
navigator.mediaDevices.getUserMedia({ audio: true })
  -> MediaStream
  -> MediaStreamAudioSourceNode
  -> AudioWorkletNode
  -> WgrabWebAudioFrame
```

## Threading

Main thread:

- permission
- stream setup
- AudioContext ownership
- UI integration

Audio rendering thread:

- AudioWorkletProcessor
- real-time callback
- sample block processing

Wasm shared state:

- optional ring buffer
- timestamp queue
- frame handoff

## Frame model

```rust
pub struct WgrabWebAudioFrame {
    // future web-specific audio frame
}
```

## Why web-specific frame type

`AudioWorklet` and `GPUExternalTexture` are browser concepts.

They should not be forced into native `WgrabAudioFrame` if doing so hides
important browser lifetime and permission constraints.

## Backend boundary

Native CPAL and browser Web Audio should share policy-level capture concepts
but not implementation ownership.

Native CPAL owns native host/device/stream setup.

Browser Web Audio owns permission prompts, `MediaStream` creation,
AudioContext setup, worklet loading, and browser-specific frame lifetime.
