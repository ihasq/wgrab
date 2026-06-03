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

## Phase 26A owner decision

CPAL is adopted as the native audio backend for wgrab.

It plays the same architectural role for native audio that wgpu plays for
video/GPU interop: a cross-platform backend abstraction layer.

Limitations:

- CPAL does not by itself guarantee system audio loopback capture on every platform.
- System audio capture may still need platform-specific integration.
- CPAL is the first public backend abstraction, not the entire audio capture solution.

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

## Phase 27W decision

CPAL remains the native backend.

For web, CPAL AudioWorklet support is investigated but not assumed sufficient
for capture input.

wgrab's web audio backend should be designed around Web Audio / AudioWorklet
directly, with CPAL integration only if it fits the capture pipeline.

## CPAL input stream prototype

The first implementation target is CPAL default input stream capture.

System audio loopback remains future work.

## Timestamp strategy

CPAL input stream prototype uses a wgrab monotonic capture clock first.

Backend-native stream timestamps are future work.

## Phase 30L decision

CPAL remains the generic native audio backend.

System audio capture is investigated separately:

- Windows: WASAPI loopback candidate
- macOS: ScreenCaptureKit audio candidate

Both should feed `WgrabAudioFrame`.

## Phase 32A decision

Try CPAL system-audio candidate streams first.

If insufficient, proceed to direct platform backends.

## CPAL sufficiency criteria

CPAL remains sufficient for system audio only if real devices expose usable
loopback candidates.

If not, wgrab should use:

- Windows: direct WASAPI loopback
- macOS: ScreenCaptureKit audio
