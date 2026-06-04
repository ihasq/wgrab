# Linux desktop testing policy

## Test tiers

### Tier 1: backend smoke

Runs on GitHub Actions.

- wgpu Vulkan / Lavapipe
- audio/wgpu compile and marker gates

### Tier 2: synthetic desktop smoke

Runs on GitHub Actions and Hetzner CX43.

- Xvfb
- Weston headless
- Wayland client probe
- PipeWire / portal availability probe

### Tier 3: persistent synthetic E2E

Runs on Hetzner CX43.

- stable package set
- repeated Wayland / PipeWire / Vulkan probes
- future Linux capture backend prototype

### Tier 4: real desktop E2E

Requires self-hosted or paid desktop runner.

- GNOME/KDE/Sway
- real PipeWire portal permission
- real GPU if needed
- real monitor / session
- strict screen/audio capture

## Current decision

wgrab should implement Tier 1 and Tier 2 now.

Hetzner CX43 is used for Tier 3.

Tier 4 is future work.
