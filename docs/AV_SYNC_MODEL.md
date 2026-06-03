# wgrab audio/video synchronization model

## Goal

wgrab should allow audio and video capture frames to be related on a shared
capture timeline.

## Timestamp type

`WgrabTimestamp` represents monotonic elapsed time on a wgrab capture timeline.

It is not wall-clock time.

## Current prototype

Audio frames receive a prototype timestamp from `WgrabCaptureClock`.

The timestamp is currently assigned when `WgrabAudioStream::try_next_frame()`
dequeues samples.

## Future precision improvements

Future phases should use backend-provided timing when available:

- CPAL callback timing
- platform audio stream timestamps
- ScreenCaptureKit frame timestamps
- Windows capture timestamps
- browser media timestamps

## Video frame timestamp

Future `WgpuCaptureFrame` should expose a timestamp.

Candidate API:

```rust
impl WgpuCaptureFrame {
    pub fn timestamp(&self) -> Option<WgrabTimestamp>;
}
```

This is not added in Phase 29S because it would touch the existing video API.

## Drift

Audio and video may use different clocks.

Future drift handling may require:

- shared capture clock
- clock offset estimation
- sample rate drift tracking
- frame pairing tolerance
- resampling outside wgrab or optional resampler support

## Non-goals

Phase 29S does not implement:

- drift correction
- resampling
- frame scheduling
- A/V muxing
- encoding

## System audio timestamps

System audio capture may provide better timestamps than generic input capture.

Future backend timestamp sources:

- WASAPI capture timestamps
- ScreenCaptureKit sample buffer timestamps
- browser media timestamps

The public API should normalize these to `WgrabTimestamp`.

## ScreenCaptureKit audio timestamps

ScreenCaptureKit audio sample buffers may provide timing metadata.

Future wgrab A/V sync should prefer sample buffer timestamps over dequeue-time
timestamps when available.
