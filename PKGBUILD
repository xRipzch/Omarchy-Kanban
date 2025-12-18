# Maintainer: xRipzch
pkgname=omarchy-kanban
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple terminal-based kanban board for the Omarchy Arch Community"
arch=('x86_64')
url="https://github.com/xRipzch/Omarchy-Kanban"
license=('MIT')  
depends=('gcc-libs')
makedepends=('cargo' 'git')
source=("git+https://github.com/xRipzch/Omarchy-Kanban.git#tag=v${pkgver}")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname"
    cargo build --release --locked
}

check() {
    cd "$srcdir/$pkgname"
    cargo test --release --locked
}

package() {
    cd "$srcdir/$pkgname"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
