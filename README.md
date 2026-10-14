# t3d — High-Performance 3D Terminal Graphics Engine

[![Crates.io](https://img.shields.io/badge/crates.io-v1.0.0-orange.svg?style=flat-square)](https://crates.io/)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-blue.svg?style=flat-square)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS-lightgrey.svg?style=flat-square)]()
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)]()

**t3d** is a lightweight, zero-dependency, standalone terminal 3D graphics and demoscene engine written in pure Rust. Built from the ground up without heavy external game engines or math libraries, `t3d` renders complex 3D procedural meshes directly inside standard terminal emulators using 24-bit TrueColor ANSI escape sequences, depth buffering (Z-buffer), Blinn-Phong specular reflection, Unicode Braille subpixel rendering, CRT post-processing, and full mouse viewport manipulation.

---

## Table of Contents

- [Features](#features)
- [Quick Installation (Binary Releases)](#quick-installation-binary-releases)
  - [Debian / Ubuntu / Pop!_OS (.deb)](#debian--ubuntu--pop_os-deb)
  - [Arch Linux / Manjaro / CachyOS (.pkg.tar.zst)](#arch-linux--manjaro--cachyos-pkgtarzst)
  - [Fedora / RHEL / AlmaLinux / Rocky Linux (.rpm)](#fedora--rhel--almalinux--rocky-linux-rpm)
  - [One-Line Automated Shell Installer](#one-line-automated-shell-installer)
  - [GitHub CLI (gh)](#github-cli-gh)
- [Building From Source](#building-from-source)
- [Interactive Controls](#interactive-controls)
  - [Keyboard Navigation](#keyboard-navigation)
  - [Mouse Navigation](#mouse-navigation)
- [Procedural Geometry](#procedural-geometry)
- [Render Pipelines & Modes](#render-pipelines--modes)
  - [1. Hybrid (Shaded Polygons + Wireframe Outlines)](#1-hybrid-shaded-polygons--wireframe-outlines)
  - [2. Solid (Blinn-Phong Depth-Buffered Triangles)](#2-solid-blinn-phong-depth-buffered-triangles)
  - [3. Braille (High-Resolution Subpixel Wireframe)](#3-braille-high-resolution-subpixel-wireframe)
  - [4. Wire (Depth-Attenuated Vector Wireframe)](#4-wire-depth-attenuated-vector-wireframe)
  - [5. Points (Vertex Cloud with Normal Vectors)](#5-points-vertex-cloud-with-normal-vectors)
- [Color Palettes](#color-palettes)
- [Engine Architecture & Math Internals](#engine-architecture--math-internals)
  - [Math Core (Vector Operations)](#math-core-vector-operations)
  - [Barycentric Triangle Rasterization](#barycentric-triangle-rasterization)
  - [Braille Subpixel Bitmask Mapping](#braille-subpixel-bitmask-mapping)
  - [Lighting Model (Lambert + Blinn-Phong Specular)](#lighting-model-lambert--blinn-phong-specular)
  - [Post-Processing FX Pipeline](#post-processing-fx-pipeline)
  - [Zero-Allocation Backbuffer & Flush Mechanics](#zero-allocation-backbuffer--flush-mechanics)
- [Package Maintainer & Release Guide](#package-maintainer--release-guide)
  - [Debian (.deb) via cargo-deb](#debian-deb-via-cargo-deb)
  - [Red Hat / Fedora (.rpm) via cargo-generate-rpm](#red-hat--fedora-rpm-via-cargo-generate-rpm)
  - [Arch Linux PKGBUILD](#arch-linux-pkgbuild)
  - [Automated GitHub Actions CI/CD Workflow](#automated-github-actions-cicd-workflow)
- [Terminal Compatibility & Troubleshooting](#terminal-compatibility--troubleshooting)
- [License](#license)

---

## Features

- **Blazing Fast Rasterizer:** Written in idiomatic, allocation-free Rust routines targeting continuous 60 FPS viewport refresh rates.
- **TrueColor 24-bit Output:** Smooth color gradients across highlight, mid-tone, shadow, and specular reflection channels.
- **Subpixel Unicode Braille Engine:** Employs the `U+2800..U+28FF` glyph block to achieve an effective $2 \times 4$ resolution multiplier per terminal cell ($8\times$ resolution boost).
- **9 Procedural 3D Geometric Meshes:** From Platonic solids to intricate diamond-cut jewels, toroidal manifolds, parametric knots, and DNA helices.
- **Dynamic Lighting & Shading:** Moving orbital light source with Blinn-Phong half-angle specular calculation and backface culling.
- **Full Viewport Interactivity:** Natural mouse drag-to-rotate and scroll-to-zoom capabilities alongside tactile WASD / Arrow keyboard controls.
- **Demoscene FX Stack:** CRT scanlines, horizontal row glitch displacement, sinusoidal vertex wave pulsing, and parallax 3D starfields with hyperwarp streak trails.
- **Zero Runtime Dependencies:** Native binary linking solely against system `libc`/`libgcc`.

---

## Quick Installation (Binary Releases)

Ready-to-install packages are compiled and distributed with every GitHub Release for all major Linux distributions[cite: 1, 2, 3].

### Debian / Ubuntu / Pop!_OS (.deb)

Download and install the pre-compiled `.deb` package[cite: 2] directly using `wget` or `curl`:

```bash
# Download the latest .deb release package
wget [https://github.com/AdrescorGiti/t3d/releases/latest/download/t3d_1.0.0-1_amd64.deb](https://github.com/AdrescorGiti/t3d/releases/latest/download/t3d_1.0.0-1_amd64.deb)

# Install package with dependencies handled automatically
sudo apt install -y ./t3d_1.0.0-1_amd64.deb

# Verify installation and launch
t3d
```

*Alternative installation using `dpkg`:*
```bash
sudo dpkg -i t3d_1.0.0-1_amd64.deb || sudo apt-get install -f -y
```

*To uninstall:*
```bash
sudo apt remove t3d
```

---

### Arch Linux / Manjaro / CachyOS (.pkg.tar.zst)

Download and install the native Arch package archive[cite: 1] via `pacman`:

```bash
# Download the latest Arch package archive
wget [https://github.com/AdrescorGiti/t3d/releases/latest/download/t3d-1.0.0-1-x86_64.pkg.tar.zst](https://github.com/AdrescorGiti/t3d/releases/latest/download/t3d-1.0.0-1-x86_64.pkg.tar.zst)

# Install via pacman local package flag
sudo pacman -U --noconfirm t3d-1.0.0-1-x86_64.pkg.tar.zst

# Launch
t3d
```

*To uninstall:*
```bash
sudo pacman -R t3d
```

---

### Fedora / RHEL / AlmaLinux / Rocky Linux (.rpm)

Download and install the standalone `.rpm` package[cite: 3] via `dnf` or `rpm`:

```bash
# Download the latest RPM binary release
wget [https://github.com/AdrescorGiti/t3d/releases/latest/download/t3d-1.0.0-1.x86_64.rpm](https://github.com/AdrescorGiti/t3d/releases/latest/download/t3d-1.0.0-1.x86_64.rpm)

# Install via dnf
sudo dnf install -y ./t3d-1.0.0-1.x86_64.rpm

# Launch
t3d
```

*Alternative installation using `rpm`:*
```bash
sudo rpm -Uvh t3d-1.0.0-1.x86_64.rpm
```

*To uninstall:*
```bash
sudo dnf remove t3d
```

---

### One-Line Automated Shell Installer

Fetch, detect your package manager, and install `t3d` in a single command:

```bash
curl -sSL [https://raw.githubusercontent.com/AdrescorGiti/t3d/main/install.sh](https://raw.githubusercontent.com/AdrescorGiti/t3d/main/install.sh) | bash
```

Or run directly without root using standard `curl` into `~/.local/bin`:

```bash
mkdir -p ~/.local/bin
curl -sL [https://github.com/AdrescorGiti/t3d/releases/latest/download/t3d-linux-x86_64](https://github.com/AdrescorGiti/t3d/releases/latest/download/t3d-linux-x86_64) -o ~/.local/bin/t3d
chmod +x ~/.local/bin/t3d
export PATH="$HOME/.local/bin:$PATH"
t3d
```

---

### GitHub CLI (gh)

If you have GitHub CLI installed:

```bash
# Download release asset for your distribution
gh release download --repo AdrescorGiti/t3d --pattern "*.deb"
sudo apt install ./t3d*.deb
```

---

## Building From Source

### Prerequisites

Ensure you have a modern Rust toolchain installed (edition 2021, Rust 1.75+ recommended):

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
source "$HOME/.cargo/env"
```

### Build & Run

```bash
# Clone the repository
git clone [https://github.com/AdrescorGiti/t3d.git](https://github.com/AdrescorGiti/t3d.git)
cd t3d

# Build optimized binary
cargo build --release

# Strip debugging symbols for minimum binary size (< 1.2MB)
strip -s target/release/t3d

# Run directly
./target/release/t3d

# Optional: Install globally into Cargo binary directory
cargo install --path .
```

---

## Interactive Controls

The engine listens for raw asynchronous terminal input without buffering or needing to press Enter.

### Keyboard Navigation

```
  CAMERA PITCH / YAW                 ROTATION RESET
       [ W ]                             [ R ]
   [ A ][ S ][ D ]                       
  (or Arrow Keys)                      AUTO-SPIN TOGGLE
                                         [ Space ]
```

| Keybind | Primary Function | Details |
| :--- | :--- | :--- |
| `1` | Select **Cube** | Procedural 8-vertex, 12-triangle hexahedron |
| `2` | Select **Pyramid** | 5-vertex square-base pyramid |
| `3` | Select **Globe** | Dual-tone checkered UV-sphere (288 triangles) |
| `4` | Select **Ruby** | Brilliant-cut facet jewel with table & culet |
| `5` | Select **Torus** | Dual-frequency checkered donut manifold |
| `6` | Select **Octahedron** | 8-faceted crystal bipyramid |
| `7` | Select **Icosahedron** | 20-sided regular polyhedron (D20 die) |
| `8` | Select **Trefoil Knot** | Continuous 3D topological torus knot tube |
| `9` | Select **DNA Helix** | Double-stranded antiparallel nucleic acid chain |
| `Tab` / `]` | Next Mesh | Cycles forward through available geometries |
| `[` | Previous Mesh | Cycles backward through available geometries |
| `M` / `m` | Switch Render Mode | `Hybrid` → `Solid` → `Braille` → `Wire` → `Points` |
| `C` / `c` | Cycle Color Palette | `Crimson Ruby` → `Cyber Neon` → `Emerald Matrix` → `Amber CRT` → `Argon Frost` |
| `B` / `b` | Toggle Starfield | `Off` → `Star Drift` → `Hyperwarp Streaks` |
| `X` / `x` | Backface Culling | Toggles hidden polygon/edge discard ($N \cdot V \le 0$) |
| `L` / `l` | Orbit Light Source | Toggles dynamic rotating light vs. static sun |
| `P` / `p` | Wave Pulse Effect | Sinusoidal real-time vertex displacement |
| `G` / `g` | CRT Glitch Mode | Enables scanline darkening & random row tearing |
| `H` / `h` | Toggle HUD Overlay | Toggles telemetry headers and status footers |
| `Space` | Pause Rotation | Freezes mesh rotation for inspect mode |
| `W` / `Up` | Pitch Up | Decreases Euler pitch angle $\theta_x$ |
| `S` / `Down` | Pitch Down | Increases Euler pitch angle $\theta_x$ |
| `A` / `Left` | Yaw Left | Decreases Euler yaw angle $\theta_y$ |
| `D` / `Right` | Yaw Right | Increases Euler yaw angle $\theta_y$ |
| `+` / `=` | Camera Zoom In | Decreases camera distance $Z_{\text{dist}}$ |
| `-` / `_` | Camera Zoom Out | Increases camera distance $Z_{\text{dist}}$ |
| `R` / `r` | Viewport Reset | Resets camera distance, angles, and pulse |
| `Q` / `Esc` | Clean Exit | Restores screen buffer, cursor, and disables mouse capture |

### Mouse Navigation

`t3d` uses standard X10 / SGR mouse reporting (`EnableMouseCapture`):

| Mouse Input | Action |
| :--- | :--- |
| `Left Click + Drag` | Free-form trackball rotation around pitch ($X$) and yaw ($Y$) axes |
| `Scroll Wheel Up` | Zoom camera closer to the object ($Z_{\text{dist}} - 0.25$) |
| `Scroll Wheel Down` | Zoom camera further away from the object ($Z_{\text{dist}} + 0.25$) |

---

## Procedural Geometry

All geometry in `t3d` is evaluated algebraically at runtime without reading external `.obj` or `.gltf` asset files:

```
    [1] Cube           [2] Pyramid          [3] Globe           [4] Ruby
      ┌───┐                ▲                  ╭───╮              /─────\
     /   /│               /█\                │▓░▓░│             / ╱ █ ╲ \
    ├───┼ │              /███\               │░▓░▓│            │─────────│
    │   │ /             /█████\               ╰───╯             \   │   /
    └───┘              ─────────                                 \  │  /
                                                                   ▼
    [5] Torus        [6] Octahedron       [7] Icosahedron     [8] Trefoil Knot
      ╭─╮                  ▲                  ╭───╮               ╭─╮ ╭─╮
    ╭╯   ╰╮               /█\                /╱╲_╱\               │ ╰─╯ │
    ╰╮   ╭╯              <███>              < ╲_╱ >               ╰─╮ ╭─╯
      ╰─╯                 \█/                \╱   ╲/                ╰─╯
                           ▼                  ╰───╯
```

1. **Cube (`1`):** A canonical 8-vertex hexahedron with 12 triangulated faces, optimized quad normals, and individual quadrant tone attenuation.
2. **Pyramid (`2`):** A 5-vertex square-based pyramid featuring a sharp apex and base polygon triangulation with normal alignment.
3. **Globe (`3`):** UV sphere parameterized across 12 latitude rings and 24 longitudinal sectors ($288$ faces). Features alternating checkered surface albedo ($tone = 1.0$ vs $tone = 0.58$) to guarantee rotation and curvature perception under any lighting angle.
4. **Ruby (`4`):** Authentic brilliant-cut gemstone composed of an octagonal crown, elevated flat *table*, offset upper girdle facets, mid girdle ring, and steep conical *pavilion* converging at the lower *culet*.
5. **Torus (`5`):** Dual-parameter donut manifold ($\theta, \phi \in [0, 2\pi)$) with $R_{\text{major}} = 1.05$, $r_{\text{minor}} = 0.42$, and procedural surface striping.
6. **Octahedron (`6`):** 6-vertex regular octahedron with dual opposing pyramidal caps producing crystalline light reflections.
7. **Icosahedron (`7`):** 12-vertex, 20-equilateral-triangle convex solid built from 3 orthogonal golden rectangles with the golden ratio $\phi = \frac{1 + \sqrt{5}}{2} \approx 1.6180339$.
8. **Trefoil Knot (`8`):** A continuous, self-interlocking $(2, 3)$-torus knot parameterized along 64 path increments enveloped by an 8-sided tubular cross-section.
9. **DNA Helix (`9`):** Antiparallel double-stranded ribbon with 36 step intervals, continuous phosphate-deoxyribose backbone edges, and periodic hydrogen-bonded base pair rungs.

---

## Render Pipelines & Modes

Press `M` to dynamically cycle through the 5 rendering engines:

### 1. Hybrid (Shaded Polygons + Wireframe Outlines)
Combines rasterized triangle illumination with high-contrast foreground wireframe edge tracing. Provides maximum depth perception, volumetric presence, and geometric edge clarity simultaneously.

### 2. Solid (Blinn-Phong Depth-Buffered Triangles)
Pure polygonal rasterization. Every triangle is projected to viewport space, clipped, and rasterized using half-space edge equations with barycentric coordinate interpolation. Z-buffer depth rejection prevents visual overlap.

### 3. Braille (High-Resolution Subpixel Wireframe)
Bypasses character-level limitations by treating each terminal cell as a $2 \times 4$ dot matrix mapped to Unicode Braille patterns (`U+2800..U+28FF`). Rasterizes vector edges at $8\times$ effective terminal resolution with Z-depth attenuation.

### 4. Wire (Depth-Attenuated Vector Wireframe)
Classic demoscene vector wireframe rendered using a 3D Bresenham line algorithm. Automatically dims distant occluded edges while illuminating foreground contours.

### 5. Points (Vertex Cloud with Normal Vectors)
Renders vertices as depth-shaded points and extrudes surface normal vectors ($V + N \cdot 0.22$) into empty 3D space, showing the geometric curvature vectors of the shape.

---

## Color Palettes

Press `C` to switch TrueColor 24-bit RGB palettes:

| Palette Name | Low Shadow (RGB) | Mid-Tone (RGB) | Highlight (RGB) | Specular Shine (RGB) | Theme Mood |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Crimson Ruby** | `(55, 8, 22)` | `(215, 25, 65)` | `(255, 95, 130)` | `(255, 235, 240)` | Deep luxury gemstone |
| **Cyber Neon** | `(25, 15, 75)` | `(0, 210, 255)` | `(255, 50, 210)` | `(240, 255, 255)` | Synthwave / Cyberpunk |
| **Emerald Matrix** | `(8, 45, 18)` | `(25, 185, 65)` | `(130, 255, 155)` | `(230, 255, 235)` | Retro terminal hacker |
| **Amber CRT** | `(60, 25, 5)` | `(220, 120, 15)` | `(255, 200, 65)` | `(255, 250, 220)` | Vintage VT220 phosphor |
| **Argon Frost** | `(15, 35, 75)` | `(65, 145, 255)` | `(165, 225, 255)` | `(255, 255, 255)` | Glacial cryogenic blue |

---

## Engine Architecture & Math Internals

### Math Core (Vector Operations)

`t3d` uses a custom `Vec3` struct implementing `Add`, `Sub`, and `Mul<f32>` operator overloading:

$$\mathbf{u} \cdot \mathbf{v} = u_x v_x + u_y v_y + u_z v_z$$

$$\mathbf{u} \times \mathbf{v} = \begin{pmatrix} u_y v_z - u_z v_y \\ u_z v_x - u_x v_z \\ u_x v_y - u_y v_x \end{pmatrix}$$

3D rotations use consecutive Euler angle transformations around the principal axes ($X \to Y \to Z$):

$$\mathbf{v}' = R_z(\theta_z) R_y(\theta_y) R_x(\theta_x) \mathbf{v}$$

Perspective projection to screen space coordinates $(x_s, y_s)$ incorporates font aspect ratio compensation:

$$x_s = \frac{W}{2} + v_x \left(\frac{f}{v_z + d}\right) \cdot 2.1, \quad y_s = \frac{H}{2} + v_y \left(\frac{f}{v_z + d}\right)$$

where $W, H$ are terminal dimensions, $f$ is focal length, and $d$ is camera distance.

### Barycentric Triangle Rasterization

For every screen-space triangle with vertices $A, B, C$ and candidate pixel $P = (x + 0.5, y + 0.5)$:

$$w_0 = \frac{\text{Edge}(B, C, P)}{\text{Area}(A, B, C)}, \quad w_1 = \frac{\text{Edge}(C, A, P)}{\text{Area}(A, B, C)}, \quad w_2 = \frac{\text{Edge}(A, B, P)}{\text{Area}(A, B, C)}$$

A fragment is drawn if and only if $w_0 \ge 0, w_1 \ge 0, w_2 \ge 0$. Interpolated depth $Z_p$ is calculated as:

$$Z_p = w_0 Z_A + w_1 Z_B + w_2 Z_C$$

If $Z_p < Z_{\text{buffer}}[x, y]$, the fragment updates both the depth buffer and character buffer.

### Braille Subpixel Bitmask Mapping

Each character cell covers a $2 \times 4$ subpixel grid. Subpixels $(dx, dy)$ map into bits of Unicode offset `0x2800`:

```
  ┌───────┬───────┐
  │ dy=0: │ dy=0: │  Bit 0 (0x01) | Bit 3 (0x08)
  │ dx=0  │ dx=1  │
  ├───────┼───────┤
  │ dy=1: │ dy=1: │  Bit 1 (0x02) | Bit 4 (0x10)
  │ dx=0  │ dx=1  │
  ├───────┼───────┤
  │ dy=2: │ dy=2: │  Bit 2 (0x04) | Bit 5 (0x20)
  │ dx=0  │ dx=1  │
  ├───────┼───────┤
  │ dy=3: │ dy=3: │  Bit 6 (0x40) | Bit 7 (0x80)
  │ dx=0  │ dx=1  │
  └───────┴───────┘
```

Glyph calculation:

$$\text{Codepoint} = \text{0x2800} + \sum \text{bit}_k$$

### Lighting Model (Lambert + Blinn-Phong Specular)

Illumination combines ambient reflection, Lambertian diffuse, and Blinn-Phong specular highlights using the half-angle vector $\mathbf{H}$:

$$\mathbf{H} = \frac{\mathbf{L} + \mathbf{V}}{\Vert{}\mathbf{L} + \mathbf{V}\Vert{}}$$

$$I_{\text{diffuse}} = \max(0, \mathbf{N} \cdot \mathbf{L}), \quad I_{\text{specular}} = \max(0, \mathbf{N} \cdot \mathbf{H})^{16}$$

$$Luminance = \left(0.16 + 0.84 \cdot I_{\text{diffuse}}\right) \cdot Tone_{\text{mesh}}$$

The resulting luminance indexes into a 70-character density ramp:
```
 .'^\",:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$
```

### Post-Processing FX Pipeline

- **CRT Scanlines & Glitch (`G`):** Every even terminal scanline undergoes a 25% brightness attenuation. Random row-shift operations rotate character lines by $1..5$ characters to emulate analog synchronization loss.
- **Wave Pulse Displacement (`P`):** Displaces vertices along their direction vectors using a sinusoidal function:
  $$mv = v \cdot \left(1.0 + 0.16 \cdot \sin(4.0 \cdot t_{\text{elapsed}} + 2.5 \cdot v_y)\right)$$
- **3D Starfield Warp (`B`):** 180 independent star particles simulated in camera space. Drift mode renders point specks; Hyperwarp mode connects previous and current frames with Bresenham streak lines.

### Zero-Allocation Backbuffer & Flush Mechanics

Instead of issuing escape sequences per character (which causes I/O bottlenecks and cursor tearing), `t3d` renders into flat contiguous vectors:
- `Vec<char>`: Character buffer ($W \times H$)
- `Vec<(u8, u8, u8)>`: 24-bit TrueColor foreground buffer
- `Vec<f32>`: Z-buffer floating-point depth array

At the end of each frame, the buffers serialize into a pre-allocated `String` and flush to standard output using a single atomic `out.write_all()` call, capped at 60 FPS.

---

## Package Maintainer & Release Guide

`t3d` includes configuration for building downstream native packages for Arch, Debian, and Fedora/RHEL[cite: 1, 2, 3].

### Debian (.deb) via cargo-deb

Add metadata to `Cargo.toml`:

```toml
[package.metadata.deb]
maintainer = "AdrescorGiti"
copyright = "2026, AdrescorGiti [https://github.com/AdrescorGiti](https://github.com/AdrescorGiti)"
license-file = ["LICENSE", "4"]
extended-description = "A lightweight terminal 3D engine featuring Phong shading, Braille subpixels, and procedural geometry."
section = "utils"
priority = "optional"
assets = [
    ["target/release/t3d", "usr/bin/t3d", "755"],
]
```

Build the `.deb` package[cite: 2]:

```bash
cargo install cargo-deb
cargo build --release
cargo deb
# Output located at: target/debian/t3d_1.0.0-1_amd64.deb[cite: 2]
```

### Red Hat / Fedora (.rpm) via cargo-generate-rpm

Configure RPM asset mapping in `Cargo.toml`:

```toml
[package.metadata.generate-rpm]
assets = [
    { source = "target/release/t3d", dest = "/usr/bin/t3d", mode = "755" }
]
```

Build the `.rpm` package[cite: 3]:

```bash
cargo install cargo-generate-rpm
cargo build --release
strip -s target/release/t3d
cargo generate-rpm
# Output located at: target/generate-rpm/t3d-1.0.0-1.x86_64.rpm[cite: 3]
```

### Arch Linux PKGBUILD

Save as `PKGBUILD`[cite: 1]:

```sh
pkgname=t3d
pkgver=1.0.0
pkgrel=1
pkgdesc="Terminal 3D engine with Phong shading, Braille subpixels, and procedural geometry"
arch=('x86_64' 'aarch64')
url="[https://github.com/AdrescorGiti/t3d](https://github.com/AdrescorGiti/t3d)"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')

build() {
    cd "$startdir"
    export CARGO_TARGET_DIR="$startdir/target-pkg"
    cargo build --release --all-features
}

package() {
    install -Dm0755 "$startdir/target-pkg/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
```

Build and install[cite: 1]:

```bash
makepkg -si
```

### Automated GitHub Actions CI/CD Workflow

Create `.github/workflows/release.yml` to automatically build and attach `.deb`, `.rpm`, and Arch `.pkg.tar.zst` packages on every tagged release[cite: 1, 2, 3]:

```yaml
name: Release Packages

on:
  push:
    tags:
      - 'v*'

jobs:
  build-release:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Install packaging utilities
        run: |
          sudo apt-get update
          sudo apt-get install -y binutils zstd
          cargo install cargo-deb
          cargo install cargo-generate-rpm

      - name: Compile release binary
        run: |
          cargo build --release
          strip -s target/release/t3d

      - name: Build Debian package
        run: |
          cargo deb
          cp target/debian/*.deb ./t3d_1.0.0-1_amd64.deb

      - name: Build RPM package
        run: |
          cargo generate-rpm
          cp target/generate-rpm/*.rpm ./t3d-1.0.0-1.x86_64.rpm

      - name: Build standalone tarball
        run: |
          tar -czvf t3d-linux-x86_64.tar.gz -C target/release t3d

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            t3d_1.0.0-1_amd64.deb
            t3d-1.0.0-1.x86_64.rpm
            t3d-linux-x86_64.tar.gz
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

## Terminal Compatibility & Troubleshooting

| Terminal Emulator | TrueColor (24-bit) | Mouse Capture | Braille Alignment | Recommended Settings |
| :--- | :---: | :---: | :---: | :--- |
| **Alacritty** | Yes | Full | Flawless | Default config, GPU-accelerated |
| **Kitty** | Yes | Full | Flawless | Default config |
| **Foot** (Wayland) | Yes | Full | Flawless | Lightweight Wayland terminal |
| **WezTerm** | Yes | Full | Flawless | Excellent font fallback |
| **Konsole** | Yes | Full | Great | Set font to *JetBrains Mono* or *Fira Code* |
| **GNOME Terminal** | Yes | Full | Good | Ensure `Use TrueColor` enabled |
| **Tmux** | Config Req. | Config Req. | Good | Add `set -g mouse on` and `set -ga terminal-overrides ',*256col*:Tc'` |

### Troubleshooting Steps

1. **Braille Characters Show As Question Marks or Boxes:**
   - Your terminal font lacks Unicode Braille patterns (`U+2800..U+28FF`).
   - *Fix:* Install a modern coding font such as `ttf-jetbrains-mono`, `ttf-fira-code`, or `ttf-nerd-fonts-symbols`.
2. **Colors Appear Flat or Distorted:**
   - Your terminal may be running in 256-color or 16-color mode.
   - *Fix:* Run `export COLORTERM=truecolor` and verify your `$TERM` is set to `xterm-256color` or `alacritty`/`kitty`.
3. **Mouse Dragging Selects Text Instead of Rotating:**
   - In some multiplexers (e.g., `tmux` or `screen`), mouse reporting must be enabled explicitly.
   - *Fix:* Add `set -g mouse on` to `~/.tmux.conf`. In macOS Terminal.app, hold `Option` or use a modern terminal like Alacritty, Kitty, or iTerm2.

---

## License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for complete details.
