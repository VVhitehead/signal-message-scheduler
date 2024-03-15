# Maintainer: VVhitehead <perm4z38@tutanota.com>
pkgname="signal-message-scheduler"
pkgver=1.1.0
pkgrel=1
pkgdesc="Interactive CLI message scheduler for Signal Messenger"
arch=("x86_64")
url="https://github.com/VVhitehead/signal-message-scheduler"
license=('MIT')
provides=("smsch")
depends=("signal-cli")
makedepends=('rust' 'cargo' 'git')
optdepends=()
_tag="b9c65a71ccf4b13981bf877fe94e9bfac3a9096a"
source=("git+${url}.git#tag={_tag}?signed")
md5sums=('SKIP')
validpgpkeys=(A4694B0277FDB74B5B1208C1E510461A0D172902)

pkgver() {
    cd "${pkgname}"
    git describe --tags | sed 's/^v//'
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --locked --release
}

package() {
    cd "${pkgname-$pkgver}"
    install -Dm 644 LICENSE -t "${pkgdir}" /usr/share/licenses/"${pkgname}"/
    install -Dm 644 README.md -t "${pkgdir}" /usr/share/doc/"${pkgname}"
    install -Dm 755 target/release/"${pkgname}" /usr/bin/"${pkgname}"
}
