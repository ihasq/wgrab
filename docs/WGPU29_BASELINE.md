# wgpu29 modernization baseline

## Baseline commit

- branch: wgpu29-phase5c-marker-hardening
- commit: 0779f6dfa3f404d5631df3c9f4d9d382acc3c830

## Dependency baseline

- wgpu: 29.0.3
- windows: 0.62
- objc2: 0.5
- objc2_06: 0.6.3
- objc2-metal: 0.3.2
- objc2-io-surface: 0.3.2
- metal: 0.28

## CI baseline

### wgpu runtime

- Windows DX12 backend smoke: success
- macOS Metal arm64 backend smoke: success
- macOS Metal Intel backend smoke: success
- Windows capture probe marker: `CI_CAPTURE_SMOKE_OK` or `CI_CAPTURE_UNAVAILABLE`
- macOS arm64 capture probe marker: `CI_CAPTURE_SMOKE_OK` or `CI_CAPTURE_UNAVAILABLE`
- macOS Intel capture probe marker: `CI_CAPTURE_SMOKE_OK` or `CI_CAPTURE_UNAVAILABLE`
- run: https://github.com/ihasq/wgrab/actions/runs/26808348392

### Ubuntu Vulkan

- Ubuntu Lavapipe Vulkan backend smoke: success
- run: https://github.com/ihasq/wgrab/actions/runs/26808348396

## Explicitly preserved

- CrabGrab API surface
- public `metal` API
- existing `objc2 = 0.5` platform wrapper
- public feature names where possible

## Deferred

- full `objc2` modernization
- public `metal` API deprecation
- public error type redesign
- full formatting cleanup
- strict capture on dedicated runner

## Future tag candidates

- `wgpu29-modernization-rc1`
- `wgrab-successor-baseline-rc1`
