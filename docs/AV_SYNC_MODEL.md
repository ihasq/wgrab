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

`WgpuCaptureFrame` exposes an optional timestamp:

```rust
impl WgpuCaptureFrame {
    pub fn timestamp(&self) -> Option<WgrabTimestamp>;
}
```

The timestamp is normalized from backend video frame timing when available.

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

The Phase 35L prototype forwards the sample-buffer presentation timestamp when
the f32 sample conversion path succeeds.

Future wgrab A/V sync should prefer sample buffer timestamps over dequeue-time
timestamps when available.

## Phase 36S integration

Both video and audio frames can expose `Option<WgrabTimestamp>`.

- `WgpuCaptureFrame::timestamp()`
- `WgrabAudioFrame::timestamp()`

A frame with `None` timestamp can still be used, but cannot be precisely paired.

`WgrabTimestamp` is still available through the audio feature for compatibility,
and is also exposed from the shared `wgrab::time` module. Future phases may
move more capture-clock helpers into the shared module.

## Pairing policy

A future pairing helper may use nearest-neighbor matching:

```text
video timestamp t_v
audio frame timestamp t_a
pair if abs(t_v - t_a) <= tolerance
```

Initial tolerance candidates:

- 16 ms for 60 Hz video
- 33 ms for 30 Hz video
- configurable in future

## Phase 36S non-goals

Phase 36S does not implement:

- drift correction
- resampling
- muxing
- scheduling
