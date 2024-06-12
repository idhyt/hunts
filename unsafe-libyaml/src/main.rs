// https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
// rustup toolchain install nightly
// rustup override set nightly
// rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
// RUSTFLAGS=-Zsanitizer=address cargo run -Zbuild-std --target x86_64-unknown-linux-gnu

mod doublefree;
mod overflow;

#[allow(unused_imports)]
use doublefree::poc as poc2;
#[allow(unused_imports)]
use overflow::poc as poc1;

fn main() {
    // println!("stack-buffer-overflow.");
    // let xs = [0, 1, 2, 3];
    // let _y = unsafe { *xs.as_ptr().offset(4) };
    // poc1();
    poc2();
}
