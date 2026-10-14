pkgname=t3d
pkgver=1.0.0
pkgrel=1
pkgdesc="Terminal 3D engine with Phong shading, Braille subpixels, and procedural geometry"
arch=('x86_64' 'aarch64')
url="https://github.com/AdrescorGiti/term3d"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')
source=()
sha256sums=()

build() {
    cd "$startdir"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR="$startdir/target-pkg"
    cargo build --frozen --release --all-features || cargo build --release --all-features
}

package() {
    install -Dm0755 "$startdir/target-pkg/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
