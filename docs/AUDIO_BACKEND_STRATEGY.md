# wgrab audio backend strategy

## Native backend candidates

### CPAL

Role:

- cross-platform audio I/O
- input and output streams
- host/device abstraction

Pros:

- Rust ecosystem standard candidate
- works across native platforms
- fits callback-based audio capture

Cons:

- may not expose system audio capture uniformly
- platform-specific loopback/system capture may still require native APIs
- synchronization with screen capture needs additional design

### Platform-specific APIs

Examples:

- WASAPI on Windows
- CoreAudio / ScreenCaptureKit audio on macOS
- Web Audio in browsers

Pros:

- may expose system audio capture more directly
- may provide better timestamps

Cons:

- more platform-specific code
- less unified public surface

## Web backend candidates

### Web Audio

Primary concepts:

- `AudioContext`
- `MediaStreamAudioSourceNode`
- `AudioWorkletNode`

Pros:

- browser-native audio graph
- pairs with `getDisplayMedia` / `getUserMedia`
- can process audio in AudioWorklet

Cons:

- browser support and permission behavior vary
- system audio availability depends on browser/platform
- wasm integration requires careful threading/worklet design

## Proposed direction

Start with docs and API design.

Native:

- investigate CPAL as a first backend abstraction
- keep platform system-audio capture requirements separate

Web:

- investigate Web Audio + AudioWorklet
- keep web frame type separate from native audio frame if necessary

## Relationship to video

Video has a clear cross-backend GPU output shape through `WgpuCaptureFrame` and
`wgpu::Texture`.

Audio does not have a single GPU resource equivalent. The audio abstraction
should therefore focus on:

- normalized frame metadata
- sample format
- timestamps
- backend/device compatibility
- avoiding raw platform API exposure as the primary public output
