# Linux CX43 synthetic desktop E2E

## Goal

Hetzner CX43 is used as the persistent Linux synthetic desktop capture testbed.

This environment validates:

- software Vulkan
- Wayland compositor startup
- Weston headless
- PipeWire
- xdg-desktop-portal
- future Linux capture backend probes

It does not claim to validate real GPU / DRM/KMS / physical desktop capture.

## Base environment

- Provider: Hetzner
- Instance: CX43
- GPU: none
- Vulkan: software Vulkan / Lavapipe
- Desktop: synthetic Wayland / Weston headless

## Packages

```bash
sudo apt-get update
sudo apt-get install -y \
  mesa-vulkan-drivers \
  vulkan-tools \
  weston \
  wayland-utils \
  pipewire \
  wireplumber \
  xdg-desktop-portal \
  xdg-desktop-portal-wlr \
  dbus-user-session \
  xvfb \
  x11-utils \
  x11-apps
```

## Runtime directory

```bash
export XDG_RUNTIME_DIR="$HOME/.run"
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"
```

## Vulkan smoke

```bash
vulkaninfo --summary
```

## Weston headless smoke

```bash
weston --backend=headless-backend.so --socket=wayland-1 --idle-time=0 > weston-headless.log 2>&1 &
WAYLAND_DISPLAY=wayland-1 wayland-info
```

## Xvfb smoke

```bash
Xvfb :99 -screen 0 1280x720x24 > xvfb.log 2>&1 &
DISPLAY=:99 xdpyinfo
```

## PipeWire / portal probe

```bash
dbus-run-session -- bash -lc '
  pipewire > pipewire.log 2>&1 &
  wireplumber > wireplumber.log 2>&1 &
  sleep 2
  ps aux | grep -E "pipewire|wireplumber" | grep -v grep
'
```

## Future wgrab probes

Future Linux probes should be added in this order:

1. Wayland environment detection
2. PipeWire connection detection
3. xdg-desktop-portal ScreenCast availability
4. synthetic capture session probe
5. frame metadata probe
6. optional Vulkan import path
