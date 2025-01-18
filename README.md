# Deus Ex Machina
[Artificial Life](https://en.wikipedia.org/wiki/Artificial_life) simulation written in Rust using [Bevy](https://github.com/bevyengine/bevy) and [Rapier](https://github.com/dimforge/rapier)

---

## Installation

### Linux dependencies
#### Ubuntu/ Debian
```bash
apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev
```

#### Fedora
```bash
dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
```

### Cargo
```bash
cargo build --release
```