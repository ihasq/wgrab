# wgrab audio roadmap

## Goal

wgrab should eventually support audio capture in addition to GPU-only video capture.

The goal is not to expose raw platform audio APIs as the primary output.
The goal is to provide a unified audio capture context that can be paired with
the existing GPU-only video capture API.

## Video precedent

Video capture output is centered on:

- `WgpuCaptureFrame`
- `wgpu::Texture`

This avoids CPU-readable bitmap output as the public capture result.

## Audio equivalent

There is no direct `wgpu::Texture` equivalent for audio.

The closest native abstraction is a cross-platform audio I/O backend such as
CPAL.

The closest web abstraction is Web Audio:

- `AudioContext`
- `MediaStreamAudioSourceNode`
- `AudioWorkletNode`

## Native target

Candidate backend:

- CPAL

Potential public API:

```rust
pub struct WgrabAudioContext {
    // internal backend context
}

pub struct WgrabAudioStream {
    // internal input stream
}

pub struct WgrabAudioFrame {
    // timestamped audio buffer
}
```

## Native backend decision

Native audio starts with CPAL.

Future work:

- input device capture
- system audio capture investigation
- timestamp model
- synchronization with `WgpuCaptureFrame`

## Web target

Candidate pipeline:

```text
getDisplayMedia / getUserMedia
  -> MediaStream
  -> MediaStreamAudioSourceNode
  -> AudioContext
  -> AudioWorkletNode
  -> WgrabWebAudioFrame
```

## Design principles

- Do not expose platform-specific audio devices as the primary output.
- Do not expose raw WASAPI / CoreAudio / WebAudio nodes as the primary public API.
- Provide timestamps suitable for audio/video synchronization.
- Keep audio capture optional.
- Keep audio support out of `wgrab 0.1.0`.
- Do not block GPU-only video capture on audio support.

## Open questions

- Should audio live in the main crate or a companion crate?
- Should native audio use CPAL directly?
- How should system audio capture differ from microphone capture?
- How should audio/video synchronization be represented?
- Should audio frames be interleaved or planar?
- Should resampling be part of wgrab or left to users?
- Should Web Audio use AudioWorklet from the beginning?

## Reference material

- CPAL: <https://github.com/RustAudio/cpal>
- MDN Web Audio API: <https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API>
- MDN AudioContext: <https://developer.mozilla.org/en-US/docs/Web/API/AudioContext>
- MDN AudioWorklet: <https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet>

## Web CPAL boundary

CPAL may be used on wasm for Web Audio / AudioWorklet processing, but wgrab's
browser capture setup must remain web-specific because permissions and
MediaStream setup are asynchronous browser APIs.

## Phase 28A

The first native audio prototype uses CPAL default input streams.

This is not system audio loopback. It is a default input-device capture
prototype.

## A/V synchronization

A/V sync starts with a shared timestamp model.

System audio and Web Audio integration should attach timestamps compatible with
`WgrabTimestamp`.
